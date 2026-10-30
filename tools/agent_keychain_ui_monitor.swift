// Explicit native qualification helper, never product startup code.
// Observe GUI launch notifications and sample daemon identities. Never log
// arbitrary argv, environment, keychain data, paths or process text.
import AppKit
import Darwin
import Foundation

struct ObservationFailure: Error {}
// Only this object waits for its spawned PID. A zero WNOHANG result preserves
// the original unreaped identity; observed exit permanently revokes signaling.
// Each child starts its own group, so timeout cleanup covers its descendants.
final class OwnedTask {
    private let pid: pid_t
    private var status: Int32? = nil
    init(_ path: String, _ arguments: [String], stdout: Int32? = nil) throws {
        var actions: posix_spawn_file_actions_t? = nil
        var attributes: posix_spawnattr_t? = nil
        guard posix_spawn_file_actions_init(&actions) == 0 else { throw ObservationFailure() }
        defer { posix_spawn_file_actions_destroy(&actions) }
        guard posix_spawnattr_init(&attributes) == 0 else { throw ObservationFailure() }
        defer { posix_spawnattr_destroy(&attributes) }
        guard posix_spawnattr_setflags(&attributes, Int16(POSIX_SPAWN_SETPGROUP)) == 0,
              posix_spawnattr_setpgroup(&attributes, 0) == 0 else { throw ObservationFailure() }
        if let output = stdout {
            guard posix_spawn_file_actions_adddup2(&actions, output, STDOUT_FILENO) == 0 else { throw ObservationFailure() }
        }
        let strings = ([path] + arguments).map { strdup($0) }
        defer { strings.forEach { free($0) } }
        guard strings.allSatisfy({ $0 != nil }) else { throw ObservationFailure() }
        var argv = strings + [nil]
        var child: pid_t = 0
        let result = argv.withUnsafeMutableBufferPointer { buffer in
            posix_spawn(&child, path, &actions, &attributes, buffer.baseAddress!, environ)
        }
        guard result == 0 else { throw ObservationFailure() }
        pid = child
    }
    func poll() throws -> Int32? {
        if let value = status { return value }
        var value: Int32 = 0
        let result = waitpid(pid, &value, WNOHANG)
        if result == pid { status = value; return value }
        if result < 0 && errno != EINTR { status = -1; throw ObservationFailure() }
        return nil
    }
    func abort() -> Bool {
        guard status == nil else { return true }
        // No Foundation auto-reaper or discovered/saved PID supplies authority.
        // Do not poll/reap between the original-identity check and group kill.
        guard kill(-pid, SIGKILL) == 0 || errno == ESRCH else { return false }
        let deadline = ProcessInfo.processInfo.systemUptime + 1
        while ProcessInfo.processInfo.systemUptime < deadline {
            do { if try poll() != nil { return true } } catch { return false }
            usleep(1000)
        }
        return false
    }
    deinit { if status == nil { _ = abort() } }
}
let names: Set<String> = ["SecurityAgent", "SecurityAgentHelper", "authorizationhost", "AuthorizationHost", "CoreServicesUIAgent", "UserNotificationCenter"]
func identities() throws -> Set<String> {
    var fds = [Int32](repeating: -1, count: 2)
    guard pipe(&fds) == 0 else { throw ObservationFailure() }
    defer { close(fds[0]); if fds[1] >= 0 { close(fds[1]) } }
    guard fcntl(fds[0], F_SETFD, FD_CLOEXEC) == 0,
          fcntl(fds[1], F_SETFD, FD_CLOEXEC) == 0,
          fcntl(fds[0], F_SETFL, O_NONBLOCK) == 0 else { throw ObservationFailure() }
    let probe = try OwnedTask("/bin/ps", ["-A", "-o", "pid=,comm="], stdout: fds[1])
    close(fds[1]); fds[1] = -1
    var bytes = Data()
    var buffer = [UInt8](repeating: 0, count: 8192)
    let deadline = ProcessInfo.processInfo.systemUptime + 1
    var eof = false
    while ProcessInfo.processInfo.systemUptime < deadline {
        let count = read(fds[0], &buffer, buffer.count)
        if count > 0 {
            guard bytes.count + count <= 2 * 1024 * 1024 else { throw ObservationFailure() }
            bytes.append(contentsOf: buffer.prefix(count))
        } else if count == 0 { eof = true }
        else if errno != EAGAIN && errno != EINTR { throw ObservationFailure() }
        if let status = try probe.poll(), eof {
            guard status == 0 else { throw ObservationFailure() }
            break
        }
        usleep(1000)
    }
    guard eof, try probe.poll() == 0, let text = String(data: bytes, encoding: .utf8) else {
        _ = probe.abort()
        throw ObservationFailure()
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
            if names.contains(name) || app.bundleIdentifier == "com.apple.SecurityAgent" { observedLaunch = true }
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
    let child = try OwnedTask(CommandLine.arguments[1], Array(CommandLine.arguments.dropFirst(2)))
    let deadline = ProcessInfo.processInfo.systemUptime + 30
    var outcome: Int32? = nil
    while ProcessInfo.processInfo.systemUptime < deadline {
        if let status = try child.poll() { outcome = status; break }
        if !complete { break }
        RunLoop.current.run(until: Date().addingTimeInterval(0.02))
    }
    guard let status = outcome else {
        let stopped = child.abort()
        print("SWBPIPE_UI_MONITOR: deadline_or_observation_failure,owned_cleanup=\(stopped)")
        exit(1)
    }
    let drain = Date().addingTimeInterval(1)
    while Date() < drain { RunLoop.current.run(until: Date().addingTimeInterval(0.02)) }
    if !(try identities()).subtracting(baseline).isEmpty { observedLaunch = true }
    print("SWBPIPE_UI_MONITOR: complete=\(complete),samples=\(samples),observed_launch=\(observedLaunch)")
    guard complete && samples > 0 && !observedLaunch && status == 0 else { exit(1) }
} catch {
    print("SWBPIPE_UI_MONITOR: observation_failed")
    exit(1)
}
