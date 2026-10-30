//! Deny-by-default Seatbelt recipe. Native probes, not this text, qualify it.
use super::loopback::Loopback;
use std::path::{Component, Path};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct InvalidPath;

fn quote(path: &Path) -> Result<String, InvalidPath> {
    let value = path.to_str().ok_or(InvalidPath)?;
    if !path.is_absolute()
        || value.len() > 4096
        || value.chars().any(char::is_control)
        || path
            .components()
            .any(|c| matches!(c, Component::ParentDir | Component::CurDir))
    {
        return Err(InvalidPath);
    }
    Ok(format!(
        "\"{}\"",
        value.replace('\\', "\\\\").replace('"', "\\\"")
    ))
}

/// All paths must be canonical, privately owned, disjoint from real user data.
/// The manager may use this builder with an invented native probe executable;
/// the production launcher always supplies its verified private supplier copy.
/// Fork remains explicitly denied. Runtime port/alias coverage still needs native
/// qualification; an existing fixed-port witness is not that evidence.
pub(crate) fn profile(
    root: &Path,
    home: &Path,
    cwd: &Path,
    executable: &Path,
    relay: &Loopback,
) -> Result<String, InvalidPath> {
    profile_at_port(root, home, cwd, executable, relay.port())
}

fn profile_at_port(
    root: &Path,
    home: &Path,
    cwd: &Path,
    executable: &Path,
    port: u16,
) -> Result<String, InvalidPath> {
    if port == 0 {
        return Err(InvalidPath);
    }
    for path in [root, home, cwd, executable] {
        quote(path)?;
    }
    if home.parent() != Some(root)
        || cwd.parent() != Some(root)
        || executable.parent() != Some(root)
        || home == cwd
        || executable == home
        || executable == cwd
    {
        return Err(InvalidPath);
    }
    let mut result = String::from(
        "(version 1)\n(deny default)\n(deny process-fork)\n(allow process-info* (target self))\n(allow sysctl-read)\n(allow mach-lookup (global-name \"com.apple.system.opendirectoryd.libinfo\"))\n",
    );
    // These OS locations are intentionally narrower than /System or /private.
    for path in ["/System/Library", "/usr/lib", "/usr/share/locale"] {
        result.push_str(&format!(
            "(allow file-read* (subpath {}))\n",
            quote(Path::new(path))?
        ));
    }
    for path in ["/System/Library", "/usr/lib"] {
        result.push_str(&format!(
            "(allow file-map-executable (subpath {}))\n",
            quote(Path::new(path))?
        ));
    }
    for path in [
        "/dev/null",
        "/dev/urandom",
        "/dev/random",
        "/usr/share/zoneinfo",
        "/private/etc/localtime",
    ] {
        result.push_str(&format!(
            "(allow file-read* (literal {}))\n",
            quote(Path::new(path))?
        ));
    }
    result.push_str("(allow file-write-data (literal \"/dev/null\"))\n");
    // Metadata for fixed OS aliases/ancestors, not user directories or files.
    for path in ["/etc", "/private/etc"] {
        result.push_str(&format!(
            "(allow file-read-metadata (literal {}))\n",
            quote(Path::new(path))?
        ));
    }
    // The selected Darwin supplier opens the root directory during startup.
    // This literal allows that directory only, never descendant file contents.
    result.push_str("(allow file-read-data (literal \"/\"))\n");
    // Ancestor metadata only: no enumeration or file contents in shared temp.
    for ancestor in root.ancestors() {
        result.push_str(&format!(
            "(allow file-read-metadata (literal {}))\n",
            quote(ancestor)?
        ));
    }
    result.push_str(&format!("(allow file-read* (subpath {}))\n", quote(root)?));
    for path in [home, cwd] {
        result.push_str(&format!("(allow file-write* (subpath {}))\n", quote(path)?));
    }
    // Protect the Codex-home link as well as the read-only target outside home.
    result.push_str(&format!(
        "(deny file-write* (literal {}))\n",
        quote(&home.join("codex/config.toml"))?
    ));
    result.push_str(&format!(
        "(allow process-exec file-map-executable (literal {}))\n",
        quote(executable)?
    ));
    // Seatbelt's IP filter accepts only * or localhost, not a numeric host.
    // Both named TCP loopback endpoints remain owned by the Rust relay scope.
    // Restrict to TCP: the IP filter alone would also grant unowned UDP.
    result.push_str(&format!(
        "(allow network-outbound (remote tcp \"localhost:{port}\"))\n"
    ));
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reviewed_libinfo_profile_hash_and_single_allow_delta() {
        use sha2::{Digest, Sha256};
        let root = Path::new("/private/tmp/swbpipe-agent-invented");
        let profile = profile_at_port(
            root,
            &root.join("home"),
            &root.join("work"),
            &root.join("codex"),
            8000,
        )
        .unwrap();
        let actual = profile.replace(root.to_str().unwrap(), "<private-runtime>");
        assert_eq!(actual.matches("(remote tcp ").count(), 1);
        assert!(!actual.contains("(remote ip "));
        // Invert only the reviewed narrowing from IP to TCP. All remaining
        // fixed-port bytes must reproduce the exact prior qualification hash.
        let normalized = actual.replacen("(remote tcp ", "(remote ip ", 1);
        assert_eq!(
            format!("{:x}", Sha256::digest(normalized.as_bytes())),
            "be2fe1d54fcc5c60a00ed7d304bf2e4c4885d662d84bfe5a9c7fe6ff9c6f4d7c"
        );
        let allow =
            "(allow mach-lookup (global-name \"com.apple.system.opendirectoryd.libinfo\"))\n";
        assert_eq!(normalized.matches(allow).count(), 1);
        let reviewed_old = normalized.replacen(allow, "", 1);
        assert_eq!(
            format!("{:x}", Sha256::digest(reviewed_old.as_bytes())),
            "80936c838f26fef039bbb9733f714fb68414baaec97d03709a17ba397b794f66"
        );
    }

    #[test]
    fn runtime_port_and_tcp_narrowing_change_only_the_single_network_grant() {
        use crate::{
            memory_pool::{test_host::ScriptedHost, MemoryPool},
            solve_jobs::SolveJobRegistry,
        };
        let registry = SolveJobRegistry::new(MemoryPool::new(ScriptedHost::normal()));
        let relay = Loopback::bind(&registry).unwrap();
        for invented in [
            "/private/tmp/neutral-person-runtime",
            "/tmp/swbpipe-invented-home",
        ] {
            let root = Path::new(invented);
            let actual = profile(
                root,
                &root.join("home"),
                &root.join("work"),
                &root.join("codex"),
                &relay,
            )
            .unwrap();
            let qualified = profile_at_port(
                root,
                &root.join("home"),
                &root.join("work"),
                &root.join("codex"),
                8000,
            )
            .unwrap();
            assert_eq!(
                actual.replace(&format!("localhost:{}", relay.port()), "localhost:8000"),
                qualified
            );
            assert_eq!(actual.matches("(allow network-outbound ").count(), 1);
            assert!(actual.contains("(deny process-fork)"));
            assert_eq!(actual.matches("(allow process-exec ").count(), 1);
            assert!(profile_at_port(
                root,
                &root.join("home"),
                &root.join("work"),
                &root.join("codex"),
                0
            )
            .is_err());
        }
    }

    /// Compile tools/agent_host_sandbox_probe.rs through t3_slot.sh, then pass
    /// its path as SWBPIPE_SANDBOX_PROBE. No real documents or keys are read.
    #[test]
    #[ignore = "manager-owned native filesystem/network/process probes"]
    fn native_permission_probes() {
        use crate::{
            memory_pool::{test_host::ScriptedHost, MemoryPool},
            solve_jobs::SolveJobRegistry,
        };
        use std::fs;
        use std::net::TcpListener;
        use std::os::unix::fs::PermissionsExt;
        use std::process::Command;

        let probe = std::env::var_os("SWBPIPE_SANDBOX_PROBE").expect("compiled invented probe");
        let mut nonce = [0; 16];
        getrandom::fill(&mut nonce).unwrap();
        let suffix: String = nonce.iter().map(|byte| format!("{byte:02x}")).collect();
        let parent = Path::new("/private/tmp").join(format!("swbpipe-profile-test-{suffix}"));
        struct Fixture(std::path::PathBuf);
        impl Drop for Fixture {
            fn drop(&mut self) {
                let _ = fs::remove_dir_all(&self.0);
            }
        }
        fs::create_dir(&parent).unwrap();
        let _fixture = Fixture(parent.clone());
        let root = parent.join("runtime");
        let outside = parent.join("outside");
        for dir in [&root, &outside, &root.join("home"), &root.join("work")] {
            fs::create_dir(dir).unwrap();
            fs::set_permissions(dir, fs::Permissions::from_mode(0o700)).unwrap();
        }
        for name in ["project", "documents", "keychain"] {
            fs::write(outside.join(name), b"invented outside canary").unwrap();
        }
        let executable = root.join("probe");
        fs::copy(probe, &executable).unwrap();
        fs::set_permissions(&executable, fs::Permissions::from_mode(0o500)).unwrap();
        let registry = SolveJobRegistry::new(MemoryPool::new(ScriptedHost::normal()));
        let relay = Loopback::bind(&registry).unwrap();
        let policy = profile(
            &root,
            &root.join("home"),
            &root.join("work"),
            &executable,
            &relay,
        )
        .unwrap();
        // Invented endpoints, no provider/model/key. Refuse a busy port rather
        // than inspect or interfere with another server.
        let forbidden = TcpListener::bind("127.0.0.1:0").unwrap();
        let result = Command::new("/usr/bin/sandbox-exec")
            .args(["-p", &policy])
            .arg(&executable)
            .current_dir(root.join("work"))
            .env_clear()
            .env("HOME", root.join("home"))
            .env("TMPDIR", root.join("home"))
            .env("SWBPIPE_PROBE_OUTSIDE", &outside)
            .env("SWBPIPE_PROBE_ALLOWED_PORT", relay.port().to_string())
            .env(
                "SWBPIPE_PROBE_FORBIDDEN_ENDPOINT",
                forbidden.local_addr().unwrap().to_string(),
            )
            .output()
            .unwrap();
        assert!(
            result.status.success(),
            "native permission observations: {}",
            String::from_utf8_lossy(&result.stdout)
        );
    }

    #[test]
    fn denies_widening_and_quotes_scheme_paths() {
        assert!(quote(Path::new("/private/tmp/a\n")).is_err());
        assert!(quote(Path::new("relative")).is_err());
        assert!(quote(Path::new("/private/../Users")).is_err());
        assert_eq!(
            quote(Path::new("/private/tmp/a\"b")).unwrap(),
            "\"/private/tmp/a\\\"b\""
        );
        let root = Path::new("/private/tmp/invented");
        let p = profile_at_port(
            root,
            &root.join("home"),
            &root.join("work"),
            &root.join("codex"),
            8000,
        )
        .unwrap();
        assert!(p.starts_with("(version 1)\n(deny default)"));
        assert!(!p.contains("(subpath \"/System\")"));
        assert!(!p.contains("/System/Volumes/Data"));
        assert!(!p.contains("network-inbound"));
        assert!(!p.contains("allow process-fork"));
        assert_eq!(p.matches("(allow file-write* ").count(), 2);
        assert!(profile_at_port(
            root,
            Path::new("/tmp/swbpipe-invented-home"),
            &root.join("work"),
            &root.join("codex"),
            8000
        )
        .is_err());
    }
}
