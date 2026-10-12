fn main() {
    // A solve worker is started by the app itself (`solve_worker`); it must
    // not initialize Tauri or AppKit.
    if std::env::args_os().nth(1).as_deref() == Some(std::ffi::OsStr::new(openpipestress_desktop_lib::SOLVE_WORKER_FLAG)) {
        std::process::exit(openpipestress_desktop_lib::run_solve_worker());
    }
    if std::env::args().any(|argument| argument == "--self-test-saved-edited-load") {
        match openpipestress_desktop_lib::run_packaged_saved_edited_load_self_test() {
            Ok(evidence) => {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&evidence)
                        .expect("packaged saved edited-load evidence serializes")
                );
                return;
            }
            Err(error) => {
                eprintln!("PACKAGED-SAVED-EDITED-LOAD-SELF-TEST-FAILED: {error}");
                std::process::exit(1);
            }
        }
    }
    openpipestress_desktop_lib::run();
}
