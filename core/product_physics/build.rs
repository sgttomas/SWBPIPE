// U4 G5 (D-6 = (a); BUILD.md §2.1 as amended by G2_AMENDMENTS.md §1): record the
// compiler and target identity the retained memory profile is qualified for.
//
// The script reads cargo-provided environment variables, `$RUSTC -vV`, and the
// reviewed inputs (the PP lock and the precommit reader's 13 `include_str!`
// statics, which D-6's reviewed-lock record binds by SHA-256). It never panics
// and never fails the build: ordinary product builds are unaffected. Any read or
// parse failure of the identity emits the whole value `v1;unavailable`, and an
// unreadable input its own `unavailable`; the crate treats either as a mismatch
// (the profile is Stale and the ordinary path runs). An empty variable is a
// value, not a failure. It writes no files, needs no network and adds no
// build-dependency.
use std::io::Write;

include!("src/build_identity.rs");

fn main() {
    let identity = read_identity()
        .map(|values| {
            let borrowed: [&[u8]; 16] = std::array::from_fn(|i| values[i].as_slice());
            encode_identity(&borrowed)
        })
        .unwrap_or_else(|| IDENTITY_UNAVAILABLE.to_owned());
    let mut out = std::io::stdout().lock();
    let _ = writeln!(out, "cargo:rerun-if-changed=build.rs");
    let _ = writeln!(out, "cargo:rerun-if-changed=src/build_identity.rs");
    for variable in ["RUSTC", "RUSTC_WRAPPER", "RUSTFLAGS", "CARGO_ENCODED_RUSTFLAGS", "RUSTUP_TOOLCHAIN"] {
        let _ = writeln!(out, "cargo:rerun-if-env-changed={variable}");
    }
    let _ = writeln!(out, "cargo:rustc-env={IDENTITY_VARIABLE}={identity}");
    // The reviewed inputs: the PP lock and the reader's statics, by SHA-256.
    let root = std::env::var_os("CARGO_MANIFEST_DIR").map(std::path::PathBuf::from);
    let digests: [Option<[u8; 32]>; 17] = std::array::from_fn(|i| {
        let path = root.as_ref()?.join(REVIEWED_INPUTS[i]);
        std::fs::read(path).ok().map(|bytes| sha256(&bytes))
    });
    for path in REVIEWED_INPUTS {
        let _ = writeln!(out, "cargo:rerun-if-changed={path}");
    }
    let _ = writeln!(out, "cargo:rustc-env={REVIEWED_INPUTS_VARIABLE}={}", encode_reviewed_inputs(&digests));
}

/// Every key's value, in `IDENTITY_KEYS` order, or `None` on any read failure.
fn read_identity() -> Option<[Vec<u8>; 16]> {
    let var = |name: &str| std::env::var(name).ok();
    let rustc = std::env::var_os("RUSTC")?;
    let output = std::process::Command::new(rustc).arg("-vV").output().ok()?;
    if !output.status.success() {
        return None;
    }
    let text = String::from_utf8(output.stdout).ok()?;
    let field = |prefix: &str| -> Option<String> {
        let mut found = text.lines().filter_map(|line| line.strip_prefix(prefix));
        let value = found.next()?.trim().to_owned();
        // Exactly one line per field: an ambiguous report is a read failure.
        if found.next().is_some() || value.is_empty() {
            return None;
        }
        Some(value)
    };
    let debug_assertions = if std::env::var_os("CARGO_CFG_DEBUG_ASSERTIONS").is_some() { "true" } else { "false" };
    let values: [String; 16] = [
        field("release:")?,
        field("commit-hash:")?,
        field("host:")?,
        field("LLVM version:")?,
        var("TARGET")?,
        var("CARGO_CFG_TARGET_ARCH")?,
        var("CARGO_CFG_TARGET_POINTER_WIDTH")?,
        var("CARGO_CFG_TARGET_ENDIAN")?,
        var("CARGO_CFG_TARGET_OS")?,
        var("CARGO_CFG_TARGET_ENV")?,
        var("CARGO_CFG_PANIC")?,
        var("PROFILE")?,
        var("OPT_LEVEL")?,
        debug_assertions.to_owned(),
        var("CARGO_ENCODED_RUSTFLAGS")?,
        format!("{}@{}", var("CARGO_PKG_NAME")?, var("CARGO_PKG_VERSION")?),
    ];
    Some(values.map(String::into_bytes))
}
