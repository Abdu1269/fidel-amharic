import Cocoa
import InputMethodKit

@objc(FidelServerDelegate)
open class FidelServerDelegate: NSObject, NSApplicationDelegate {
    public static var server: IMKServer?

    public func applicationDidFinishLaunching(_ notification: Notification) {
        guard let connectionName = Bundle.main.infoDictionary?["InputMethodConnectionName"] as? String else {
            fatalError("Missing InputMethodConnectionName in Info.plist")
        }

        guard let identifier = Bundle.main.bundleIdentifier else {
            fatalError("Missing CFBundleIdentifier in Info.plist")
        }

        // Initialize InputMethodKit server
        FidelServerDelegate.server = IMKServer(name: connectionName, bundleIdentifier: identifier)
        NSLog("Fidel Amharic Input Method Server initialized: %@", connectionName)
    }

    public func applicationWillTerminate(_ notification: Notification) {
        NSLog("Fidel Amharic Input Method Server terminating.")
    }
}
