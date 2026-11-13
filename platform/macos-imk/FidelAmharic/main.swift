import Cocoa
import InputMethodKit

let delegate = FidelServerDelegate()
NSApplication.shared.delegate = delegate
_ = NSApplicationMain(CommandLine.argc, CommandLine.unsafeArgv)
