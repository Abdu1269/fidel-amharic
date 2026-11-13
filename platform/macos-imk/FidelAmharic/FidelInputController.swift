import Cocoa
import InputMethodKit

@objc(FidelInputController)
open class FidelInputController: IMKInputController {
    private var engineHandle: OpaquePointer?

    public override init!(server: IMKServer!, delegate: Any!, client: Any!) {
        super.init(server: server, delegate: delegate, client: client)
        // Initialize pure Rust Fidel composition engine with embedded Power Geez profile
        self.engineHandle = fidel_engine_create(nil)
    }

    deinit {
        if let handle = engineHandle {
            fidel_engine_destroy(handle)
            engineHandle = nil
        }
    }

    // MARK: - Event Handling

    open override func handle(_ event: NSEvent!, client sender: Any!) -> Bool {
        guard let event = event, let engine = engineHandle else {
            return false
        }

        // Only handle key down events
        if event.type != .keyDown {
            return false
        }

        // Pass through Command and Control keyboard shortcuts (e.g., Cmd+C, Cmd+V, Cmd+Z, Cmd+S, Cmd+Q)
        let modifiers = event.modifierFlags.intersection([.command, .control])
        if !modifiers.isEmpty {
            return false
        }

        // Check for special virtual keys: Backspace (0x33), Escape (0x35)
        var action = CFidelAction()
        let keyCode = event.keyCode

        if keyCode == 0x33 { // Backspace
            let handled = fidel_engine_process_special(engine, 1, &action)
            if handled == 1 {
                return applyAction(action, client: sender)
            }
            return false
        }

        if keyCode == 0x35 { // Escape
            let handled = fidel_engine_process_special(engine, 2, &action)
            if handled == 1 {
                return applyAction(action, client: sender)
            }
            return false
        }

        // Process printable characters
        guard let characters = event.characters, !characters.isEmpty else {
            return false
        }

        var anyHandled = false
        for scalar in characters.unicodeScalars {
            let res = fidel_engine_process_char(engine, scalar.value, &action)
            if res == 1 {
                let consumed = applyAction(action, client: sender)
                if consumed {
                    anyHandled = true
                }
            } else {
                return false
            }
        }

        return anyHandled
    }

    // MARK: - Action Dispatcher

    private func applyAction(_ action: CFidelAction, client sender: Any?) -> Bool {
        guard let client = sender as? IMKTextInput else {
            return false
        }

        switch action.action_type {
        case FIDEL_ACTION_PASS_THROUGH:
            return false

        case FIDEL_ACTION_CONSUMED:
            return true

        case FIDEL_ACTION_SET_COMPOSITION:
            let displayText = action.display != nil ? String(cString: action.display) : ""
            let range = NSRange(location: displayText.utf16.count, length: 0)
            let replacement = NSRange(location: NSNotFound, length: NSNotFound)
            client.setMarkedText(displayText, selectionRange: range, replacementRange: replacement)
            return true

        case FIDEL_ACTION_COMMIT:
            let commitText = action.text != nil ? String(cString: action.text) : ""
            let replacement = NSRange(location: NSNotFound, length: NSNotFound)
            client.insertText(commitText, replacementRange: replacement)

            if let displayPtr = action.display, displayPtr.pointee != 0 {
                let displayText = String(cString: displayPtr)
                let range = NSRange(location: displayText.utf16.count, length: 0)
                client.setMarkedText(displayText, selectionRange: range, replacementRange: replacement)
            }
            return true

        case FIDEL_ACTION_COMMIT_AND_COMPOSE:
            let commitText = action.text != nil ? String(cString: action.text) : ""
            let displayText = action.display != nil ? String(cString: action.display) : ""
            let replacement = NSRange(location: NSNotFound, length: NSNotFound)

            client.insertText(commitText, replacementRange: replacement)
            let range = NSRange(location: displayText.utf16.count, length: 0)
            client.setMarkedText(displayText, selectionRange: range, replacementRange: replacement)
            return true

        case FIDEL_ACTION_CLEAR_COMPOSITION:
            let replacement = NSRange(location: NSNotFound, length: NSNotFound)
            client.setMarkedText("", selectionRange: NSRange(location: 0, length: 0), replacementRange: replacement)
            return true

        default:
            return false
        }
    }

    // MARK: - Composition Lifecycle

    open override func commitComposition(_ sender: Any!) {
        guard let engine = engineHandle, let client = sender as? IMKTextInput else {
            return
        }

        // Commit composition clears the buffer and resets engine state
        fidel_engine_reset(engine)
        let replacement = NSRange(location: NSNotFound, length: NSNotFound)
        client.setMarkedText("", selectionRange: NSRange(location: 0, length: 0), replacementRange: replacement)
    }

    open override func cancelComposition() {
        if let engine = engineHandle {
            fidel_engine_reset(engine)
        }
        if let client = client() as? IMKTextInput {
            let replacement = NSRange(location: NSNotFound, length: NSNotFound)
            client.setMarkedText("", selectionRange: NSRange(location: 0, length: 0), replacementRange: replacement)
        }
    }

    open override func activateServer(_ sender: Any!) {
        super.activateServer(sender)
    }

    open override func deactivateServer(_ sender: Any!) {
        cancelComposition()
        super.deactivateServer(sender)
    }

    // MARK: - Input Method Menu

    open override func menu() -> NSMenu! {
        let menu = NSMenu(title: "Fidel Amharic")

        let versionString: String
        if let verPtr = fidel_engine_version() {
            versionString = String(cString: verPtr)
        } else {
            versionString = "0.1.0"
        }

        let profileItem = NSMenuItem(title: "Profile: Power Geez Phonetic (Unicode)", action: nil, keyEquivalent: "")
        profileItem.isEnabled = false
        menu.addItem(profileItem)

        menu.addItem(NSMenuItem.separator())

        let settingsItem = NSMenuItem(title: "Fidel Settings & Visualizer...", action: #selector(openSettingsApp), keyEquivalent: "")
        settingsItem.target = self
        menu.addItem(settingsItem)

        let aboutItem = NSMenuItem(title: "About Fidel Amharic (v\(versionString))", action: #selector(showAbout), keyEquivalent: "")
        aboutItem.target = self
        menu.addItem(aboutItem)

        return menu
    }

    @objc private func openSettingsApp() {
        let appUrls = [
            URL(fileURLWithPath: "/Applications/Fidel Amharic.app"),
            URL(fileURLWithPath: NSHomeDirectory() + "/Applications/Fidel Amharic.app"),
        ]

        let workspace = NSWorkspace.shared
        for url in appUrls {
            if FileManager.default.fileExists(atPath: url.path) {
                workspace.open(url)
                return
            }
        }

        let alert = NSAlert()
        alert.messageText = "Fidel Settings Companion"
        alert.informativeText = "The Fidel companion settings application was not found in /Applications. You can launch it directly from your installation folder."
        alert.alertStyle = .informational
        alert.runModal()
    }

    @objc private func showAbout() {
        let alert = NSAlert()
        alert.messageText = "Fidel Amharic Input Method"
        alert.informativeText = "A native, system-wide Unicode Ethiopic input method editor.\n\nVersion: \(String(cString: fidel_engine_version()))\nLicense: MIT\nCompatible with Power Geez phonetic mappings."
        alert.alertStyle = .informational
        alert.runModal()
    }
}
