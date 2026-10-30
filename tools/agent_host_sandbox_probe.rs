//! Standalone invented-data probe for the macOS embedded-agent profile.
//! Build only through the guarded host route; never a production executable.
use std::env;
use std::fs;
use std::io::ErrorKind;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::path::PathBuf;
use std::os::unix::net::UnixDatagram;
use std::process::Command;
use std::time::Duration;

fn denied<T>(result: std::io::Result<T>) -> bool {
    matches!(result, Err(error) if error.kind() == ErrorKind::PermissionDenied)
}

fn main() {
    if env::args().nth(1).as_deref() == Some("--child") {
        return;
    }
    let home = PathBuf::from(env::var_os("HOME").expect("private home"));
    let cwd = env::current_dir().expect("private cwd");
    let outside = PathBuf::from(env::var_os("SWBPIPE_PROBE_OUTSIDE").expect("invented fixture"));
    let port: u16 = env::var("SWBPIPE_PROBE_ALLOWED_PORT")
        .expect("owned fixture port")
        .parse()
        .unwrap();
    assert_ne!(port, 0);
    let allowed: SocketAddr = format!("127.0.0.1:{port}").parse().unwrap();
    let allowed_ipv6: SocketAddr = format!("[::1]:{port}").parse().unwrap();
    let alias: SocketAddr = format!("127.0.0.2:{port}").parse().unwrap();
    let forbidden: SocketAddr = env::var("SWBPIPE_PROBE_FORBIDDEN_ENDPOINT")
        .unwrap()
        .parse()
        .unwrap();
    let mut checks = Vec::new();
    for (name, path) in [
        ("private_home", home.join("write")),
        ("private_cwd", cwd.join("write")),
    ] {
        checks.push((
            name,
            fs::write(&path, b"invented").is_ok()
                && fs::read(&path).ok().as_deref() == Some(b"invented"),
        ));
    }
    for name in ["project", "documents", "keychain"] {
        checks.push((name, denied(fs::read(outside.join(name)))));
    }
    checks.push((
        "outside_write",
        denied(fs::write(outside.join("new"), b"invented")),
    ));
    let link = home.join("outside-link");
    checks.push((
        "symlink_read",
        std::os::unix::fs::symlink(outside.join("project"), &link).is_ok()
            && denied(fs::read(link)),
    ));
    checks.push((
        "provider_endpoint",
        TcpStream::connect_timeout(&allowed, Duration::from_secs(1)).is_ok(),
    ));
    checks.push((
        "provider_ipv6_endpoint",
        TcpStream::connect_timeout(&allowed_ipv6, Duration::from_secs(1)).is_ok(),
    ));
    // A permitted alias with no listener is not a denial. This deliberately
    // requires EPERM, not connection refusal, before certifying the boundary.
    checks.push((
        "unowned_loopback_alias",
        denied(TcpStream::connect_timeout(&alias, Duration::from_secs(1))),
    ));
    checks.push((
        "other_loopback",
        denied(TcpStream::connect_timeout(
            &forbidden,
            Duration::from_secs(1),
        )),
    ));
    checks.push((
        "off_host",
        denied(TcpStream::connect_timeout(
            &"198.51.100.1:9".parse().unwrap(),
            Duration::from_secs(1),
        )),
    ));
    checks.push(("syslog", denied(UnixDatagram::unbound().and_then(|socket| socket.connect("/private/var/run/syslog")))));
    checks.push(("users_read", denied(fs::read_dir("/Users"))));
    checks.push(("listener", denied(TcpListener::bind("127.0.0.1:0"))));
    checks.push((
        "other_exec",
        denied(Command::new("/bin/echo").arg("invented").output()),
    ));
    checks.push((
        "same_exec",
        denied(
            Command::new(env::current_exe().unwrap())
                .arg("--child")
                .output(),
        ),
    ));
    for (name, passed) in &checks {
        println!("{name}: {passed}");
    }
    if checks.iter().any(|(_, passed)| !passed) {
        std::process::exit(1);
    }
}
