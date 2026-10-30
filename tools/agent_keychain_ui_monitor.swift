// Explicit native qualification helper, never product startup code.
// Observe GUI launch notifications and sample daemon identities. No argv,
// environment, keychain data, file contents or arbitrary process text is logged.
import AppKit
import Foundation

let names: Set<String> = ["SecurityAgent", "SecurityAgentHelper", "authorizationhost", "AuthorizationHost", "CoreServicesUIAgent", "UserNotificationCenter"]
func identities() throws -> Set<String> {
    let probe = Process()
    probe.executableURL = URL(fileURLWithPath: "/bin/ps")
    probe.arguments = ["-A", "-o", "pid=,comm="]
    let pipe = Pipe()
    probe.standardOutput = pipe
    probe.standardError = FileHandle.nullDevice
    try probe.run()
    let data = pipe.fileHandleForReading.readDataToEndOfFile()
    probe.waitUntilExit()
    guard probe.terminationStatus == 0, let text = String(data: data, encoding: .utf8) else {
        throw NSError(domain: "monitor", code: 1)
    }
    var result = Set<String>()
    for line in text.split(separator: "\n") {
        let fields = line.split(maxSplits: 1, whereSeparator: { $0 == " " || $0 == "\t" })
        guard fields.count == 2 else { continue }
        let name = URL(fileURLWithPath: String(fields[1])).lastPathComponent
        if names.contains(name) { result.insert(String(fields[0]) + ":" + name) }
    }
    return result
}

guard CommandLine.arguments.count > 1 else { exit(2) }
do {
    let baseline = try identities()
    var observedLaunch = false
    var complete = true
    var samples = 0
    let center = NSWorkspace.shared.notificationCenter
    let token = center.addObserver(forName: NSWorkspace.didLaunchApplicationNotification, object: nil, queue: .main) { note in
        if let app = note.userInfo?[NSWorkspace.applicationUserInfoKey] as? NSRunningApplication {
            let name = app.executableURL?.lastPathComponent ?? ""
            if names.contains(name) || app.bundleIdentifier == "com.apple.SecurityAgent" {
                observedLaunch = true
            }
        }
    }
    defer { center.removeObserver(token) }
    let timer = Timer.scheduledTimer(withTimeInterval: 0.02, repeats: true) { _ in
        do {
            if !(try identities()).subtracting(baseline).isEmpty { observedLaunch = true }
            samples += 1
        } catch { complete = false }
    }
    defer { timer.invalidate() }
    let child = Process()
    child.executableURL = URL(fileURLWithPath: CommandLine.arguments[1])
    child.arguments = Array(CommandLine.arguments.dropFirst(2))
    child.standardInput = FileHandle.nullDevice
    child.standardOutput = FileHandle.standardOutput
    child.standardError = FileHandle.standardError
    try child.run()
    let deadline = Date().addingTimeInterval(30)
    while child.isRunning && Date() < deadline {
        RunLoop.current.run(until: Date().addingTimeInterval(0.02))
    }
    guard !child.isRunning else {
        child.terminate() // original owned Process, never a discovered PID
        child.waitUntilExit()
        print("SWBPIPE_UI_MONITOR: deadline_exceeded")
        exit(1)
    }
    child.waitUntilExit()
    // Allow already-posted workspace launch notifications to reach the observer.
    let drain = Date().addingTimeInterval(1)
    while Date() < drain { RunLoop.current.run(until: Date().addingTimeInterval(0.02)) }
    if !(try identities()).subtracting(baseline).isEmpty { observedLaunch = true }
    print("SWBPIPE_UI_MONITOR: complete=\(complete),samples=\(samples),observed_launch=\(observedLaunch)")
    guard complete && samples > 0 && !observedLaunch && child.terminationStatus == 0 else { exit(1) }
} catch {
    print("SWBPIPE_UI_MONITOR: observation_failed")
    exit(1)
}
