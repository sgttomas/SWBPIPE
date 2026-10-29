//! Deny-by-default Seatbelt recipe. Native probes, not this text, qualify it.
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
/// Fork remains denied by default. Its compatibility is unqualified until probed.
pub(crate) fn profile(
    root: &Path,
    home: &Path,
    cwd: &Path,
    executable: &Path,
) -> Result<String, InvalidPath> {
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
        "(version 1)\n(deny default)\n(allow process-info* (target self))\n(allow sysctl-read)\n",
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
    // The sole configured URL is IPv4 loopback; this grants loopback port 8000.
    result.push_str("(allow network-outbound (remote ip \"localhost:8000\"))\n");
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Compile tools/agent_host_sandbox_probe.rs through t3_slot.sh, then pass
    /// its path as SWBPIPE_SANDBOX_PROBE. No real documents or keys are read.
    #[test]
    #[ignore = "manager-owned native filesystem/network/process probes"]
    fn native_permission_probes() {
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
        let policy = profile(&root, &root.join("home"), &root.join("work"), &executable).unwrap();
        // Invented endpoints, no provider/model/key. Refuse a busy port rather
        // than inspect or interfere with another server.
        let _allowed = TcpListener::bind("127.0.0.1:8000").expect("provider port must be idle");
        let forbidden = TcpListener::bind("127.0.0.1:0").unwrap();
        let result = Command::new("/usr/bin/sandbox-exec")
            .args(["-p", &policy])
            .arg(&executable)
            .current_dir(root.join("work"))
            .env_clear()
            .env("HOME", root.join("home"))
            .env("TMPDIR", root.join("home"))
            .env("SWBPIPE_PROBE_OUTSIDE", &outside)
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
        let p = profile(
            root,
            &root.join("home"),
            &root.join("work"),
            &root.join("codex"),
        )
        .unwrap();
        assert!(p.starts_with("(version 1)\n(deny default)"));
        assert!(!p.contains("(subpath \"/System\")"));
        assert!(!p.contains("/System/Volumes/Data"));
        assert!(!p.contains("network-inbound"));
        assert!(!p.contains("allow process-fork"));
        assert_eq!(p.matches("(allow file-write* ").count(), 2);
        assert!(profile(
            root,
            Path::new("/Users/invented"),
            &root.join("work"),
            &root.join("codex")
        )
        .is_err());
    }
}
