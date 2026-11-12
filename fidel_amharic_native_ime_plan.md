# Fidel Amharic --- Detailed Plan for a Native, System-Wide Ethiopic Input Method

**Document type:** Product, architecture, implementation, and release
plan\
**Primary language:** Amharic (`am`)\
**Script:** Ethiopic / Ge'ez (`Ethi`)\
**Target operating systems:** Windows, macOS, and Linux\
**Working name:** Fidel Amharic

## 1. Executive summary

Build a **native system-wide Amharic input method** that lets users type
Unicode Ethiopic text in third-party applications---not just inside a
dedicated editor.

Use a shared Rust keyboard-composition engine and native
operating-system adapters:

-   **Windows:** Text Services Framework (TSF).
-   **macOS:** InputMethodKit.
-   **Linux:** IBus and Fcitx5 integrations.

Use **Tauri 2 only for the companion settings application**, onboarding,
keyboard visualizer, tutorials, diagnostics, and preferences. Tauri is
not the system-wide input method itself. The native input-method
component must continue working when the settings window is closed.

Do not use clipboard automation, simulated keystrokes, global keyboard
hooks, or a hidden typing window as substitutes for native IME APIs.

## 2. Product goals

1.  Type Amharic in supported applications across the operating system.
2.  Output real Unicode Ethiopic characters, not proprietary
    font-encoded text.
3.  Support a verified Power Geez phonetic-compatible Unicode layout.
4.  Provide a modern, easy-to-learn Amharic phonetic layout.
5.  Support GFF Amharic and a Microsoft-style layout only when the
    actual mappings are verified.
6.  Keep language, keyboard layout, and font as separate concepts.
7.  Work offline after installation.
8.  Provide a modern settings app and clear OS activation instructions.
9.  Share one tested Rust engine across all platforms.
10. Provide reliable installation, upgrade, and uninstall processes.

### Out of scope for the first Amharic MVP

-   A full word processor.
-   Tigrinya and every Ethiopic language (design for future extension).
-   Cloud prediction, autocorrect, speech recognition, or AI writing
    features.
-   Legacy Power Geez document conversion.
-   Claiming compatibility with every game, terminal, secure field,
    remote desktop, or application that bypasses native text-input
    frameworks.

## 3. User experience

### First-run journey

1.  User installs the package for their operating system.
2.  Installer installs/registers the native input-method component and
    settings app.
3.  Welcome screen explains how to enable Fidel Amharic in OS input
    settings.
4.  User enables **Fidel Amharic** in the OS input-source/language
    settings.
5.  User selects Fidel Amharic from the OS input menu or shortcut.
6.  User types in Word, LibreOffice, browser forms, email clients, and
    other supported applications.
7.  User can close the settings app; system-wide typing continues.

### Settings application

Build a modern, accessible companion app with:

-   **Welcome:** activation steps and a test typing field.
-   **Dashboard:** installed status, default layout, engine version,
    quick actions.
-   **Keyboard layouts:** Power Geez-compatible, Fidel Modern Phonetic,
    GFF Amharic, and verified Microsoft-style profile.
-   **Keyboard visualizer:** QWERTY map, Shift/AltGr layers, searchable
    Fidel character lookup.
-   **Typing tutorial:** guided syllable exercises, ambiguous sequences,
    terminators.
-   **Preferences:** default layout, shortcut guidance, optional
    feedback (off by default), startup behavior.
-   **Diagnostics:** OS version, adapter/engine/profile versions,
    integration status. Never include typed text.
-   **About:** licenses, attributions, privacy statement, version
    details.

Do not make a font selector part of the system-wide input path. The
target application controls text rendering. The companion app may
provide a font specimen page to help users choose an Ethiopic font in
their other applications.

## 4. Research and compatibility baseline

### Unicode

All new text must be Unicode Ethiopic. Relevant Unicode blocks include:

-   Ethiopic: `U+1200..U+137F`
-   Ethiopic Supplement: `U+1380..U+139F`
-   Ethiopic Extended: `U+2D80..U+2DDF`
-   Ethiopic Extended-A: `U+AB00..U+AB2F`
-   Ethiopic Extended-B: `U+1E7E0..U+1E7FF`

Define an Amharic-specific character inventory and reviewed test corpus.
Do not assume every code point in these blocks is needed for Amharic.

Reference:
https://unicode.org/versions/Unicode18.0.0/core-spec/chapter-19/

### Power Geez phonetic compatibility

Use **SIL Ethiopic Power-G** as the first candidate/reference for a
Power Geez phonetic-compatible Unicode profile. Its stated purpose is to
support Ethiopic Saba Fidel scripts using key sequences found in the
Power Geez (Phonetic) keyboard. Its Keyman listing describes the package
as MIT-licensed and cross-platform.

-   https://keyman.com/keyboards/sil_ethiopic_power_g
-   https://keyman.com/en/keyboards/download?id=sil_ethiopic_power_g&mode=standalone&platform=windows

Before importing mappings, inspect the actual source, version,
documentation, and license. Preserve attribution and license notices.
Build golden tests against verified upstream behavior. Label it **Power
Geez Phonetic-compatible (Unicode)**, not "official Power Geez". Do not
confuse Unicode typing with legacy font-encoded output.

### Other layouts

Review GFF Amharic as a separate Unicode mnemonic layout:

-   https://keyman.com/keyboards/gff_amharic
-   https://help.keyman.com/keyboard/gff_amharic/3.2/gff_amharic

Research actual Microsoft/Windows Ethiopic keyboard behavior. Do not
invent a mapping and call it Microsoft-compatible. If exact behavior
cannot be verified, label the profile "Windows-style community layout"
and publish its mapping.

### Native OS documentation

Use official documentation as the primary authority and re-check it
during implementation:

**Windows** - TSF overview:
https://learn.microsoft.com/en-us/windows/win32/tsf/text-services-framework -
TSF architecture:
https://learn.microsoft.com/en-us/windows/win32/tsf/text-services-framework-architecture -
Text services:
https://learn.microsoft.com/en-us/windows/win32/tsf/text-services

**macOS** - InputMethodKit:
https://developer.apple.com/documentation/inputmethodkit - Apple
development and signing: https://developer.apple.com/documentation/xcode

**Linux** - IBus: https://github.com/ibus/ibus - IBus documentation:
https://ibus.github.io/docs/ibus-1.0/ - Fcitx5:
https://github.com/fcitx/fcitx5 - Fcitx developer documentation:
https://fcitx-im.org/wiki/Develop

## 5. System architecture

``` text
                  Any supported target application
                               |
                     Native text-input framework
                               |
          +--------------------+--------------------+
          |                    |                    |
       Windows               macOS                Linux
         TSF              InputMethodKit        IBus / Fcitx5
          |                    |                    |
          +--------------------+--------------------+
                               |
                    Shared Rust input engine
                               |
             +-----------------+-----------------+
             |                 |                 |
         Profiles         Composition         Unicode
         and rules         state machine        output
                               |
                      Native adapter commits
                         text to target app

                Tauri companion settings app
                (not required while typing)
```

### Suggested repository structure

``` text
fidel-amharic/
  README.md
  LICENSE
  THIRD_PARTY_NOTICES.md
  docs/
    product-requirements.md
    architecture.md
    os-installation.md
    keyboard-layouts.md
    privacy-security.md
    testing-matrix.md
    decisions/
  profiles/
    power_geez_phonetic_unicode.json
    gff_amharic.json
    fidel_modern_phonetic.json
  crates/
    fidel-profile/
    fidel-engine/
    fidel-protocol/
    fidel-core/
    fidel-diagnostics/
  apps/
    settings-tauri/
  platform/
    windows-tsf/
    macos-inputmethod/
    linux-ibus/
    linux-fcitx5/
  tests/
    golden/
    integration/
    fixtures/
  packaging/
    windows/
    macos/
    linux/
```

Some native glue may need C++, Swift, Objective-C, or Objective-C++. Do
not force every OS API into pure Rust if that makes the integration
fragile. Keep the shared typing logic in Rust and native adapters small.

### Shared Rust modules

-   `fidel-profile`: versioned layout schema, parser, validator,
    provenance and license metadata.
-   `fidel-engine`: deterministic composition state machine; no Tauri or
    GUI dependency.
-   `fidel-protocol`: narrow versioned interface between native adapters
    and engine.
-   `fidel-core`: profile registry, settings, version compatibility.
-   `fidel-diagnostics`: integration status and safe diagnostic
    reporting; no typed text or keystroke history.

### Native adapter contract

Each native adapter translates platform events and editing APIs into a
shared engine interface. Conceptual input:

``` rust
struct KeyInput {
    key: KeyCode,
    modifiers: Modifiers,
    phase: KeyPhase,
    context: InputContext,
}
```

Conceptual output:

``` rust
enum EngineAction {
    InsertText(String),
    ReplaceComposition { text: String },
    CommitComposition,
    ClearComposition,
    PassThrough,
    Consume,
    ShowCandidates(Vec<Candidate>),
}
```

These are examples, not a final ABI. The native adapter owns native
selection ranges, marked text, focus, and commit behavior. The engine
must not assume every OS has the same editing model.

## 6. Keyboard engine requirements

Implement a finite-state machine or well-defined trie-based composer
supporting:

-   Single-key and multi-key consonant/vowel mappings.
-   Pending prefixes and ambiguous sequences.
-   Sequence terminators where a profile requires them.
-   Backspace while composition is pending.
-   Escape/cancel.
-   Punctuation and whitespace commit behavior.
-   Shift and supported modifier layers.
-   Focus loss and input-source switching.
-   Cursor movement and selection replacement.
-   Mixed Latin and Amharic text.

Distinguish physical key codes from produced text and modifier state. Do
not assume all users have a US keyboard. Respect dead keys and OS
composition where possible. Never double-process events already handled
by another input method. Pass through ordinary shortcuts
(Ctrl/Cmd+A/C/V/X/Z/Y/S/F/P).

Do not implement the engine as scattered JavaScript keydown handlers.
Keep it deterministic and testable without a GUI.

### Amharic profiles

**A. Power Geez Phonetic-compatible (Unicode)** - Use verified SIL
Ethiopic Power-G mapping data. - Preserve attribution and license
notices. - Build golden tests against upstream behavior.

**B. Fidel Modern Phonetic** - Design consistent consonant-vowel
rules. - Minimize awkward combinations and conflicts with
shortcuts/punctuation. - Publish a visual keyboard map and tutorial. -
Validate with Amharic native speakers.

**C. GFF Amharic** - Reproduce verified mapping behavior and retain
required attribution/license.

**D. Microsoft-style** - Implement only after real behavior is
documented and tested. - Never market an approximation as exact
compatibility.

### Profile schema

Use versioned JSON or TOML metadata, for example:

``` json
{
  "schema_version": 1,
  "id": "power_geez_phonetic_unicode",
  "display_name": "Power Geez Phonetic-compatible (Unicode)",
  "language": "am",
  "script": "Ethi",
  "source": "SIL Ethiopic Power-G",
  "license": "MIT",
  "version": "1.0.0",
  "rules": []
}
```

The real schema must express sequences, modifiers, priority,
composition, terminators, and conflict resolution. Validate before
activation. Profile files must not contain executable code.

## 7. Windows native implementation --- TSF

Implement a native Windows Text Services Framework text service. A
pragmatic architecture is a small C++ COM text-service component calling
the shared Rust engine through a narrow C ABI. Tauri itself cannot act
as a TSF text service.

Investigate the minimum required interfaces for the chosen design,
including: - `ITfTextInputProcessor` / `ITfTextInputProcessorEx` -
`ITfKeyEventSink` - `ITfComposition` - TSF text-store edit sessions -
Relevant compartments and language-bar APIs

The adapter must register the text service and language profile, receive
key events through TSF, read/write text through supported edit sessions,
maintain composition ranges, commit/cancel correctly, handle
focus/context changes, and pass through unrelated shortcuts.

Installation: - Build x64 first; decide ARM64 support explicitly. - Sign
binaries and installer for production. - Register COM server and TSF
profile through supported mechanisms. - Decide per-user versus
per-machine installation. - Uninstall must unregister only
application-owned components. - Test after sign-out/restart and after
application upgrades.

Test Notepad, Word, LibreOffice, Edge, Chrome, Firefox, Electron apps,
and Windows native text fields. Test selection, undo, composition,
backspace, shortcuts, focus changes, and uninstall.

Official docs: -
https://learn.microsoft.com/en-us/windows/win32/tsf/text-services-framework -
https://learn.microsoft.com/en-us/windows/win32/tsf/text-services-framework-architecture

## 8. macOS native implementation --- InputMethodKit

Implement a native macOS input method using Apple's InputMethodKit.
Package it as a correctly structured and signed input-method
component/bundle. Use Swift or Objective-C/Objective-C++ for the
InputMethodKit host and call the Rust engine through a stable C ABI. Do
not make the Tauri webview act as the input method.

The adapter should: - Implement the required `IMKInputController`
behavior. - Receive key events from the input method server. - Manage
marked/preedit text and commit/cancel operations using supported client
APIs. - Handle active client, focus, and selection changes. - Work when
the settings window is closed. - Follow macOS input-source selection
conventions.

Start with minimal composition UI; add candidate UI only if required by
the chosen layout.

Packaging: - Define minimum macOS version. - Decide universal
Intel/Apple Silicon support. - Sign app and input method correctly. -
Investigate notarization and Gatekeeper requirements. - Document
enabling the input source in Keyboard settings. - Choose and test
user-level or system-level installation. - Do not request Accessibility
permission unless a separate feature truly requires it.

Test TextEdit, Pages, Word for Mac, LibreOffice, Safari, Chrome,
Firefox, Mail, native AppKit fields, and Electron apps. Test sleep/wake,
logout/login, switching sources, upgrade, and uninstall.

Official docs: -
https://developer.apple.com/documentation/inputmethodkit -
https://developer.apple.com/documentation/xcode

## 9. Linux native implementation --- IBus and Fcitx5

Linux desktops differ. Advertise only tested combinations.

Implement separate integrations sharing the Rust engine: 1. **IBus
engine** for GNOME and IBus-based desktops. 2. **Fcitx5 addon/engine**
for KDE and Fcitx5-based desktops.

Do not assume one integration automatically works with both.

### IBus

-   Implement using supported IBus APIs.
-   Register an engine component descriptor.
-   Handle key events and handled/unhandled status correctly.
-   Manage preedit and commit through IBus APIs.
-   Package engine, descriptor, icons, and locale metadata.
-   Test daemon/session restart.

### Fcitx5

-   Implement a native addon/engine following current Fcitx5 developer
    guidance.
-   Register input method and configuration.
-   Handle key events, preedit, commit, reset, and focus changes.
-   Package addon and metadata for target distributions.
-   Test Wayland and X11 separately.

### Linux release policy

-   Select named initial distributions and versions, such as Ubuntu LTS
    and Fedora Workstation.
-   Test GNOME/IBus and KDE/Fcitx5 combinations.
-   Provide deb/rpm packages where practical.
-   Document dependencies and activation.
-   Do not depend on global X11 hooks; they are not an appropriate
    Wayland solution.
-   Do not claim Wayland/X11 support until tested in real sessions.

Test GNOME Text Editor, KDE editor, LibreOffice, Firefox, Chromium,
GTK/Qt fields, Electron apps, IBus/Fcitx5, Wayland/X11, daemon restart,
session restart, upgrade, and removal.

References: - https://github.com/ibus/ibus -
https://ibus.github.io/docs/ibus-1.0/ -
https://github.com/fcitx/fcitx5 - https://fcitx-im.org/wiki/Develop

## 10. Shared engine integration and process model

Prefer linking the Rust engine into each native adapter as a library.
This provides low latency, avoids a dependency on the settings window,
and removes an unnecessary daemon from the core typing path.

Expose a narrow, versioned C ABI or platform binding layer. Define
memory ownership, UTF-8 handling, error reporting, and ABI versioning
carefully.

If a background service is needed for profile updates or settings
synchronization: - Keep typing functional using the last valid local
profile if the service is unavailable. - Use OS-native IPC with
appropriate permissions. - Never transmit typed text or key histories. -
Do not expose an unauthenticated TCP server. - Make settings updates
atomic and safe during active composition.

## 11. Tauri companion application

Use Tauri 2 for: - Preferences and profile selection. - Layout
installation and validation. - Keyboard visualizer and tutorials. - OS
activation instructions and settings links where supported. - Adapter
status and safe diagnostics. - License/attribution information and
updates.

Use TypeScript + React (or a justified alternative), accessible
components, and light/dark themes. The native IME must not require the
Tauri process to be running. Restrict Tauri filesystem and shell
permissions.

## 12. Security and privacy

An IME receives keyboard events in many applications. Privacy is a
critical requirement:

-   Never record or store typed text.
-   Never transmit keystrokes, document content, passwords, or field
    contents.
-   No analytics by default.
-   Respect secure/password fields and OS restrictions.
-   Do not use global keyboard hooks to capture arbitrary input.
-   Validate layout profiles; reject executable mapping logic.
-   Sign production binaries.
-   Apply least privilege.
-   Provide a clear privacy statement.
-   Diagnostics may include OS, engine, adapter, and profile versions
    only.
-   Keep third-party source, licenses, and checksums documented.

## 13. Font and rendering policy

The IME inserts Unicode; the target application controls appearance.
Recommend Noto Sans Ethiopic, Noto Serif Ethiopic, and Abyssinica SIL
for users' documents and for the settings app's specimen page. Verify
current redistribution terms before bundling any font. Do not bundle
Nyala or proprietary Power Geez fonts without permission.

Explain to users that: - Unicode text does not require a special Power
Geez font. - The target application's font determines glyph
appearance. - They may need to select an Ethiopic-capable font in Word
or another application. - Changing a font does not change the underlying
text.

References: -
https://fonts.google.com/noto/specimen/Noto+Sans+Ethiopic -
https://fonts.google.com/noto/specimen/Noto+Serif+Ethiopic -
https://software.sil.org/abyssinica/ -
https://learn.microsoft.com/en-us/typography/font-list/nyala

## 14. Testing and acceptance

### Engine tests

-   Golden vectors for every profile.
-   All Amharic consonant families and vowel orders.
-   Language-relevant forms, punctuation, and numerals.
-   Ambiguous sequences and terminators.
-   Backspace, escape, and pending composition.
-   Shift/modifiers and shortcut pass-through.
-   Focus loss and input-source switch.
-   Mixed Latin/Amharic.
-   Selection replacement and cursor movement.
-   Non-US keyboard input.
-   Invalid profile rejection and rapid/long input sequences.

### Native adapter tests

For each OS: - Install and register. - Enable from OS input settings. -
Type into native editor and browser. - Type into LibreOffice or
Microsoft Office where available. - Test selection, cursor, backspace,
undo/redo, shortcuts, and focus switching. - Test secure fields. - Close
settings app and confirm typing continues. - Test upgrade, restart, and
uninstall.

### Native-speaker acceptance

Recruit Amharic-first-language testers to assess Power Geez familiarity,
modern layout learnability, coverage, speed, error rate, punctuation,
and keyboard chart clarity. Automated tests do not replace
native-speaker validation.

### Performance

Measure input latency, memory use, startup/activation time, and behavior
on real devices. Avoid unsupported performance claims; publish test
hardware and OS versions.

## 15. Roadmap

### Phase 0 --- Feasibility and source audit (1--2 weeks)

-   Audit SIL Power-G and GFF Amharic sources, mappings, and licenses.
-   Define Amharic character inventory and reviewed test corpus.
-   Define profile schema.
-   Prototype Rust engine and golden tests.
-   Build the smallest native proof of concept for TSF, InputMethodKit,
    and IBus.
-   Confirm packaging, registration, and activation requirements.

**Exit:** verified mapping provenance; Rust engine emits Unicode; native
prototype inserts Ethiopic in a native text field on each target OS or
documents a justified blocker.

### Phase 1 --- Engine and Windows alpha (2--4 weeks)

-   Finalize profile and engine crates.
-   Implement Power Geez-compatible Unicode profile.
-   Implement TSF adapter and installer.
-   Build minimal Tauri settings app and keyboard visualizer.

**Exit:** system-wide typing works in Notepad, Word/LibreOffice, and
browsers on named Windows versions; settings app can close.

### Phase 2 --- macOS alpha (2--4 weeks)

-   Implement InputMethodKit adapter.
-   Link Rust engine.
-   Sign/package input method.
-   Test supported native and browser applications.

**Exit:** system-wide typing works in the macOS test matrix without the
settings window.

### Phase 3 --- Linux IBus alpha (2--4 weeks)

-   Implement IBus adapter and package.
-   Test named GNOME distributions and sessions.
-   Document activation.

**Exit:** IBus users can type in supported Linux applications.

### Phase 4 --- Fcitx5 and modern layout (2--4 weeks)

-   Implement Fcitx5 adapter.
-   Design Fidel Modern Phonetic.
-   Conduct Amharic native-speaker review.
-   Add profile switching and management.

**Exit:** Fcitx5 works on named test distributions; modern layout has
reviewed golden vectors and usability evidence.

### Phase 5 --- Compatibility and release hardening (2--4 weeks)

-   Verify Microsoft-style mapping or explicitly defer it.
-   Audit security, privacy, licenses, signing, installers, upgrade, and
    uninstall.
-   Complete documentation and release support matrix.

**Exit:** release claims match actual tests; no known critical
composition or data-loss defects in supported configurations.

These are planning estimates, not guarantees. Native IME integration,
code signing, and application compatibility may take longer.

## 16. Definition of done

-   [ ] Registered native input method, not merely an in-app keyboard.
-   [ ] Selectable from the OS input-source controls.
-   [ ] Unicode Amharic insertion in supported third-party apps.
-   [ ] Typing continues when settings window is closed.
-   [ ] Windows uses TSF.
-   [ ] macOS uses InputMethodKit.
-   [ ] Linux supports the explicitly advertised IBus and/or Fcitx5
    integrations.
-   [ ] Power Geez-compatible mappings verified and attributed.
-   [ ] Modern layout reviewed by Amharic native speakers.
-   [ ] Shortcuts, cursor movement, selection, backspace, and undo
    behave correctly.
-   [ ] Secure-field and privacy restrictions respected.
-   [ ] Install, activation, upgrade, and uninstall tested.
-   [ ] No unauthorized proprietary assets bundled.
-   [ ] Core typing works offline.
-   [ ] Published OS/application support matrix reflects actual testing.

## 17. Instructions for the AI coding assistant

Act as a senior native input-method engineer and Rust architect. Treat
this document as the master specification.

1.  Inspect the repository before changing files.
2.  Verify current official TSF, InputMethodKit, IBus, and Fcitx5
    documentation.
3.  Audit Power Geez-compatible mappings and all third-party licenses.
4.  Propose the Rust workspace and narrow engine API.
5.  Build the Rust composition engine and golden tests before polishing
    the UI.
6.  Prove native input on each OS with a minimal prototype.
7.  Implement OS adapters using native input-method frameworks.
8.  Keep Tauri as a companion settings app, not the system-wide typing
    mechanism.
9.  Never fake compatibility with clipboard paste, simulated key events,
    or global hooks.
10. Never invent Microsoft or Power Geez mappings.
11. Keep the project runnable and tested after each milestone.
12. Report changed files, commands, test results, limitations, and next
    steps.
13. Do not claim support for an OS/application configuration until
    tested.
14. Use C++, Swift, Objective-C, or other native glue when appropriate;
    keep shared composition logic in Rust.
15. Start with Phase 0 and deliver a feasibility report plus minimal
    native proof of concept.

**First implementation task:** create the Rust workspace, implement a
deterministic Amharic composition engine with verified sample mappings,
and demonstrate actual Unicode insertion through one minimal native
input-method prototype. Then expand to the remaining operating systems.

## 18. Primary references

-   Unicode Ethiopic script chapter:
    https://unicode.org/versions/Unicode18.0.0/core-spec/chapter-19/
-   SIL Ethiopic Power-G:
    https://keyman.com/keyboards/sil_ethiopic_power_g
-   GFF Amharic: https://keyman.com/keyboards/gff_amharic
-   Windows TSF overview:
    https://learn.microsoft.com/en-us/windows/win32/tsf/text-services-framework
-   Windows TSF architecture:
    https://learn.microsoft.com/en-us/windows/win32/tsf/text-services-framework-architecture
-   Apple InputMethodKit:
    https://developer.apple.com/documentation/inputmethodkit
-   IBus: https://github.com/ibus/ibus
-   IBus docs: https://ibus.github.io/docs/ibus-1.0/
-   Fcitx5: https://github.com/fcitx/fcitx5
-   Fcitx developer docs: https://fcitx-im.org/wiki/Develop
-   Tauri architecture: https://tauri.app/concept/architecture/

Re-check current APIs, supported OS versions, SDK requirements,
licensing, and distribution rules at implementation time.
