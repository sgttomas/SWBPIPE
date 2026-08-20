fn main() {
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
