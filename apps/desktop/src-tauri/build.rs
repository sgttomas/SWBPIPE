fn main() {
    // Code review r6 at #1682 (DEL-08-01 Design `report-user-release.md`
    // §2.8, "Threat model"): a product build sets no `--cfg test`. Cargo
    // passes the rustflags it compiles this package's crates with, from
    // `RUSTFLAGS`, `CARGO_ENCODED_RUSTFLAGS` and `build.rustflags` or
    // `target.*.rustflags` configuration alike, as `CARGO_ENCODED_RUSTFLAGS`;
    // one that sets `cfg(test)` would compile the dependencies' test-only
    // items into the app, so the build stops here.
    println!("cargo:rerun-if-env-changed=CARGO_ENCODED_RUSTFLAGS");
    let flags = std::env::var("CARGO_ENCODED_RUSTFLAGS").unwrap_or_default();
    let flags: Vec<&str> = flags.split('\x1f').collect();
    for (index, flag) in flags.iter().enumerate() {
        let value = match flag.strip_prefix("--cfg") {
            Some("") => flags.get(index + 1).copied().unwrap_or_default(),
            Some(rest) => rest.strip_prefix('=').unwrap_or(rest),
            None => continue,
        };
        let name = value.trim().split(|c: char| c == '=' || c.is_whitespace()).next().unwrap_or_default();
        if name == "test" {
            panic!("the rustflags set --cfg test, which no build of this app may set (DEL-08-01 Design section 2.8)");
        }
    }
    tauri_build::build()
}
