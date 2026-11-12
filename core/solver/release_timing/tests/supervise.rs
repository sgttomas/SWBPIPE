//! The supervise mode's tests. `Classification` and `ProgressLines` port the cases of
//! `core/solver/performance_harness/runner/test_release_runner.py`'s classes of those names:
//! the same inputs, the same outputs. The live tests (macOS) run this crate's binary in supervise
//! mode on the invented demo model. The cross-check against the Python supervisor is
//! `tests/test_release_timing_supervise.py`. Nothing here measures or asserts a time or memory
//! bound.

use open_pipe_stress_release_timing::classify::{as_expected, classify, last_progress, sanitize, CLASSES, PROGRESS_PREFIX, SOLVED};
use open_pipe_stress_release_timing::host::matches_sweep;
use serde_json::{json, Value};

const GIB: i128 = 1 << 30;
const MIB: i128 = 1 << 20;
const BUDGET: i128 = 12_884_901_888;
const EMERGENCY: i128 = 16 * GIB;

/// test_release_runner.py `launched`.
fn launched(classification: &str, exit_code: i64, extra: Value) -> Value {
    let mut record = json!({"classification": classification, "exit_code": exit_code, "timeout_s": 60, "wall_s": 1.5,
        "rss": {"time_max_rss_bytes": 3 * GIB, "time_peak_footprint_bytes": 2 * GIB, "ps_max_kib": 3 * 1024 * 1024}});
    for (k, v) in extra.as_object().unwrap() {
        record[k] = v.clone();
    }
    record
}

fn ok() -> Value {
    launched("ok", 0, json!({}))
}

/// test_release_runner.py `driver`.
fn driver(status: &str, codes: &[&str], footprint: i128, extra: Value) -> Value {
    let mut line = json!({"schema": "release-timing-v1", "status": status, "blocking_codes": codes, "route": "EX",
        "nodes": 2000, "members": 2100, "supports": 300, "cases": 3, "combinations": 2,
        "rows": 1000, "output_bytes": 123456, "output_sha256": "f".repeat(64),
        "rows_per_basis": {"load_case:L1": 600, "combination:C1": 400},
        "row_classes": {"station": {"rows": 800, "bytes": 100000}},
        "seconds": {"read": 0.1, "parse": 0.2, "request": 0.0, "entry": 5.0, "serialize": 1.0,
                    "release_envelope": 0.1, "parse_back": 1.1, "total": 7.5},
        "peak_memory": {"entry": {"ru_maxrss_bytes": 3 * GIB, "phys_footprint_bytes": GIB,
                                  "lifetime_max_phys_footprint_bytes": footprint},
                        "parse_back": {"ru_maxrss_bytes": 3 * GIB + 5, "phys_footprint_bytes": GIB,
                                       "lifetime_max_phys_footprint_bytes": footprint}},
        "worker_peak_footprint_bytes_equivalent": footprint, "limits": ["a limit"]});
    for (k, v) in extra.as_object().unwrap() {
        line[k] = v.clone();
    }
    line
}

fn solved() -> Value {
    driver(SOLVED, &[], 2 * GIB, json!({}))
}

fn c(launch: &Value, line: Option<&Value>, cap: i128, budget: i128) -> (String, Option<String>) {
    classify(launch, line, cap, budget, None)
}

// --------------------------------------------------------------- Classification

#[test]
fn classification_ok() {
    assert_eq!(c(&ok(), Some(&solved()), BUDGET, BUDGET), ("ok".into(), None));
}

#[test]
fn classification_timed_out() {
    assert_eq!(c(&launched("timed_out", -9, json!({})), None, BUDGET, BUDGET).0, "timed_out");
}

#[test]
fn classification_a_watchdog_kill_exceeds_the_budget_only_on_a_footprint_reading() {
    // RSS above the cap does not prove the phys_footprint budget exceeded.
    // (The Python passes the budget as rss_cap_bytes and leaves budget_bytes at its default.)
    let (cls, detail) = c(&launched("killed_by_rss_watchdog", -9, json!({})), None, BUDGET, BUDGET);
    assert_eq!(cls, "killed_by_rss_cap");
    assert!(detail.unwrap().contains(&format!("footprint_{}_within_budget", 2 * GIB)));
    let (cls, detail) = c(&launched("killed_by_rss_watchdog", -9, json!({"rss": {}})), None, BUDGET, BUDGET);
    assert_eq!((cls.as_str(), detail.unwrap().ends_with("footprint_unread")), ("killed_by_rss_cap", true));
    let over = launched("killed_by_rss_watchdog", -9, json!({"rss": {"time_peak_footprint_bytes": BUDGET + 1}}));
    assert_eq!(c(&over, None, BUDGET, BUDGET).0, "exceeds_base_machine_budget");
    let under = launched("killed_by_rss_watchdog", -9, json!({"rss": {"time_peak_footprint_bytes": BUDGET - 1}}));
    assert_eq!(c(&under, None, BUDGET, BUDGET).0, "killed_by_rss_cap");
    assert_eq!(c(&launched("killed_by_rss_watchdog", -9, json!({})), None, 128 * MIB, BUDGET).0, "killed_by_rss_cap");
}

#[test]
fn classification_a_footprint_watchdog_kill_exceeds_the_budget() {
    let record = launched(
        "killed_by_sampler",
        -9,
        json!({"sampler": {"kill_footprint_bytes": BUDGET + 7, "peak_phys_footprint_bytes": BUDGET + 7}}),
    );
    let (cls, detail) = c(&record, None, EMERGENCY, BUDGET);
    assert_eq!(cls, "exceeds_base_machine_budget");
    assert!(detail.unwrap().contains(&(BUDGET + 7).to_string()));
}

#[test]
fn classification_an_emergency_rss_kill_with_and_without_a_footprint_over_budget() {
    let within = launched(
        "killed_by_rss_watchdog",
        -9,
        json!({"rss": {}, "sampler": {"peak_phys_footprint_bytes": BUDGET - 1, "peak_lifetime_max_phys_footprint_bytes": BUDGET - 1}}),
    );
    let (cls, detail) = c(&within, None, EMERGENCY, BUDGET);
    assert_eq!(cls, "killed_by_rss_cap");
    assert!(detail.unwrap().contains(&format!("{}_within_budget", BUDGET - 1)));
    let over = launched(
        "killed_by_rss_watchdog",
        -9,
        json!({"rss": {}, "sampler": {"peak_phys_footprint_bytes": BUDGET - 1, "peak_lifetime_max_phys_footprint_bytes": BUDGET + 1}}),
    );
    assert_eq!(c(&over, None, EMERGENCY, BUDGET).0, "exceeds_base_machine_budget");
    // The driver's last progress line also counts as a footprint reading.
    let progress = json!({"phys_footprint_bytes": BUDGET + 2, "lifetime_max_phys_footprint_bytes": BUDGET + 2});
    let bare = launched("killed_by_rss_watchdog", -9, json!({"rss": {}}));
    assert_eq!(classify(&bare, None, EMERGENCY, BUDGET, Some(&progress)).0, "exceeds_base_machine_budget");
    assert_eq!(c(&bare, None, EMERGENCY, BUDGET).0, "killed_by_rss_cap");
}

#[test]
fn classification_a_completed_run_above_the_budget_exceeds_it() {
    let (cls, detail) = c(&ok(), Some(&driver(SOLVED, &[], BUDGET + 1, json!({}))), BUDGET, BUDGET);
    assert_eq!(cls, "exceeds_base_machine_budget");
    assert!(detail.unwrap().contains("at_result"));
    let over_time = launched("ok", 0, json!({"rss": {"time_peak_footprint_bytes": BUDGET + 1}}));
    assert_eq!(c(&over_time, Some(&solved()), BUDGET, BUDGET).0, "exceeds_base_machine_budget");
    let sampled = launched("ok", 0, json!({"sampler": {"peak_lifetime_max_phys_footprint_bytes": BUDGET + 1}}));
    assert_eq!(c(&sampled, Some(&solved()), EMERGENCY, BUDGET).0, "exceeds_base_machine_budget");
}

#[test]
fn classification_the_exit_reading_stands_where_time_peak_footprint_stands() {
    // Not a Python case: this supervisor's one added footprint reading (rss.exit_lifetime_max_
    // phys_footprint_bytes) counts exactly as /usr/bin/time's peak memory footprint counts.
    for key in ["time_peak_footprint_bytes", "exit_lifetime_max_phys_footprint_bytes"] {
        let over = launched("ok", 0, json!({"rss": {key: BUDGET + 1}}));
        assert_eq!(c(&over, Some(&solved()), EMERGENCY, BUDGET).0, "exceeds_base_machine_budget", "{key}");
        let killed = launched("killed_by_rss_watchdog", -9, json!({"rss": {key: BUDGET + 1}}));
        assert_eq!(c(&killed, None, EMERGENCY, BUDGET).0, "exceeds_base_machine_budget", "{key}");
    }
}

#[test]
fn classification_refused_by_name() {
    let codes = ["NONLINEAR_DENSE_MEMORY_LIMIT_EXCEEDED", "OTHER"];
    let (cls, detail) = c(&ok(), Some(&driver("MODEL_INCOMPLETE", &codes, 2 * GIB, json!({}))), BUDGET, BUDGET);
    assert_eq!((cls.as_str(), detail.as_deref()), ("refused", Some("NONLINEAR_DENSE_MEMORY_LIMIT_EXCEEDED,OTHER")));
    let d = detail.as_deref();
    assert_eq!(as_expected("refused:NONLINEAR_DENSE_MEMORY_LIMIT_EXCEEDED", &cls, d), Some(true));
    assert_eq!(as_expected("refused:SOMETHING_ELSE", &cls, d), Some(false));
    assert_eq!(as_expected("refused", &cls, d), Some(true));
    assert_eq!(as_expected("solved", &cls, d), Some(false));
}

#[test]
fn classification_plain_classification_expectations_match_exactly() {
    assert_eq!(as_expected("exceeds_base_machine_budget", "exceeds_base_machine_budget", Some("x")), Some(true));
    assert_eq!(as_expected("exceeds_base_machine_budget", "ok", None), Some(false));
    assert_eq!(as_expected("exceeds_base_machine_budget", "timed_out", None), Some(false));
    for name in CLASSES {
        assert_eq!(as_expected(name, name, None), Some(true), "{name}");
        assert_eq!(as_expected(name, if name != "error" { "error" } else { "ok" }, None), Some(false), "{name}");
    }
    assert_eq!(as_expected("solved", "ok", None), Some(true));
    assert_eq!(as_expected("no-such-expectation", "ok", None), None);
}

#[test]
fn classification_entry_error() {
    let line = driver("entry_error", &[], 2 * GIB, json!({"entry_error": {"stage": "entry", "message": "PREVIEW-X: bad"}}));
    let (cls, detail) = c(&launched("error", 3, json!({})), Some(&line), BUDGET, BUDGET);
    assert_eq!(cls, "entry_error");
    assert_eq!(detail.as_deref(), Some("entry: PREVIEW-X: bad"));
}

#[test]
fn classification_errors() {
    assert_eq!(
        c(&launched("error", 101, json!({})), None, BUDGET, BUDGET),
        ("error".into(), Some("no_driver_line_exit_101".into()))
    );
    assert_eq!(
        c(&ok(), Some(&driver("SOMETHING", &[], 2 * GIB, json!({}))), BUDGET, BUDGET),
        ("error".into(), Some("status_SOMETHING".into()))
    );
    assert_eq!(c(&launched("error", 2, json!({})), Some(&driver("driver_error", &[], 2 * GIB, json!({}))), BUDGET, BUDGET).0, "error");
}

// --------------------------------------------------------------- ProgressLines

#[test]
fn progress_lines_last_stage_from_a_killed_runs_stderr() {
    let line = |stage: &str, f: i64| {
        format!(
            "{PROGRESS_PREFIX}{}",
            json!({"repeat": 0, "stage": stage, "seconds": 0.5, "ru_maxrss_bytes": f + 9, "phys_footprint_bytes": f,
                   "lifetime_max_phys_footprint_bytes": f + 1})
        )
    };
    // A kill can cut the last line short; other stderr text is ignored.
    let cut: String = line("entry", 40).chars().take(25).collect();
    let text = [line("read", 10), "warning: something".into(), line("parse", 20), line("request", 30), cut].join("\n");
    let (stage, memory, finished) = last_progress(Some(&text));
    assert_eq!(last_progress(None), (Value::Null, Value::Null, 0));
    assert_eq!(stage, json!({"repeat": 0, "stage": "request", "seconds": 0.5}));
    assert_eq!(memory, json!({"ru_maxrss_bytes": 39, "phys_footprint_bytes": 30, "lifetime_max_phys_footprint_bytes": 31}));
    assert_eq!(finished, 3);
}

// --------------------------------------------------------------- host text

#[test]
fn sanitize_replaces_machine_paths_as_k6_does() {
    assert_eq!(sanitize("open /Users/x/y.json: no"), "open <path>: no");
    assert_eq!(sanitize("a /tmp/b c '/var/d' /etc/e"), "a <path> c '<path>' /etc/e");
    assert_eq!(sanitize("/Users"), "/Users");
}

#[test]
fn the_sweep_pattern_matches_as_pgrep_f_does() {
    // pgrep -f 'python[0-9.]* .*run_evidence_sweep'
    assert!(matches_sweep("/usr/bin/python3.12 tools/release/run_evidence_sweep.py --execute"));
    assert!(matches_sweep("python x run_evidence_sweep"));
    assert!(!matches_sweep("python3run_evidence_sweep"));
    assert!(!matches_sweep("node run_evidence_sweep"));
    assert!(!matches_sweep("python3 -m pytest"));
}

// --------------------------------------------------------------- live (macOS)

#[cfg(target_os = "macos")]
mod live {
    use open_pipe_stress_release_timing::supervise::{packet, read_records, PACKET_SCHEMA, RECORD_SCHEMA};
    use serde_json::{json, Value};
    use std::path::{Path, PathBuf};
    use std::process::Command;

    const BIN: &str = env!("CARGO_BIN_EXE_release_timing");
    const BUDGET: u64 = 12_884_901_888;

    fn demo() -> Vec<u8> {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../fixtures/product_preview/invented_demo_model.json");
        std::fs::read(path).unwrap()
    }

    fn folder(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("release_timing_supervise_{}_{name}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    /// A plan run as `release_runner.plan_document` writes one.
    fn run(order: u64, run_id: &str, model: &str, timeout_s: i64, estimate_s: i64, expect: &str, budget: u64) -> Value {
        json!({"order": order, "run_id": run_id, "profile": "linear-ex", "nodes": 100, "mode": "sparse",
               "timeout_s": timeout_s, "footprint_budget_bytes": budget, "rss_cap_bytes": 16u64 << 30,
               "estimate_s": estimate_s, "expect": expect, "attribute": false, "model": model,
               "expected_model_sha256": null, "model_stats": null})
    }

    fn supervise(dir: &Path, runs: Vec<Value>, window_budget_s: i64) -> (i32, String, String) {
        let plan = json!({"schema": "release-timing-plan-v1", "window_budget_s": window_budget_s, "quiet_wait_s": 0, "runs": runs});
        std::fs::write(dir.join("plan.json"), plan.to_string()).unwrap();
        let output = Command::new(BIN)
            .args(["--supervise", "plan.json", "--out", "out"])
            .current_dir(dir)
            .output()
            .expect("run the supervisor");
        (
            output.status.code().unwrap_or(-1),
            String::from_utf8_lossy(&output.stdout).to_string(),
            String::from_utf8_lossy(&output.stderr).to_string(),
        )
    }

    fn by_id(records: &[Value], run_id: &str) -> Value {
        records.iter().find(|r| r["run_id"] == run_id).cloned().unwrap_or_else(|| panic!("no record {run_id}"))
    }

    #[test]
    fn a_plan_of_runs_is_supervised_classified_and_recorded() {
        let dir = folder("plan");
        std::fs::write(dir.join("demo.json"), demo()).unwrap();
        let mut nosupport: Value = serde_json::from_slice(&demo()).unwrap();
        nosupport["supports"] = json!([]);
        std::fs::write(dir.join("nosupport.json"), nosupport.to_string()).unwrap();
        let runs = vec![
            run(0, "ok", "demo.json", 60, 1, "solved", BUDGET),
            // Killed by the footprint watchdog: the demo passes 4 MiB during entry.
            run(1, "kill", "demo.json", 60, 1, "exceeds_base_machine_budget", 4 << 20),
            // Killed at the first poll after the timeout (0 s).
            run(2, "timeout", "demo.json", 0, 1, "timed_out", BUDGET),
            run(3, "refused", "nosupport.json", 60, 1, "refused:SUPPORT_INPUT_MISSING", BUDGET),
            run(4, "missing", "absent.json", 60, 1, "solved", BUDGET),
            // Its estimate exceeds the window: skipped, never launched.
            run(5, "skipped", "demo.json", 60, 100_000, "solved", BUDGET),
        ];
        let (code, stdout, stderr) = supervise(&dir, runs, 900);
        assert_eq!(code, 1, "model_missing makes the exit 1, as the Python --run: {stdout}{stderr}");
        let out = dir.join("out");
        let records = read_records(&out);
        assert_eq!(records.len(), 6);
        assert!(records.iter().all(|r| r["schema"] == RECORD_SCHEMA));
        let ok = by_id(&records, "ok");
        assert_eq!(ok["classification"], "ok", "{ok}");
        assert_eq!(ok["as_expected"], true);
        assert_eq!(ok["driver"]["status"], "MECHANICS_SOLVED");
        assert_eq!(ok["last_stage"]["stage"], "model_checks");
        assert_eq!(ok["launch"]["argv"], json!(["release_timing", "demo.json", "--mode", "sparse"]));
        assert!(ok["launch"]["rss"]["exit_lifetime_max_phys_footprint_bytes"].as_u64().unwrap() > 0);
        assert!(ok["launch"]["rss"]["time_peak_footprint_bytes"].is_null());
        assert!(ok["footprint_watchdog"]["peak_phys_footprint_bytes"].as_u64().is_some());
        assert!(ok["footprint_watchdog"].get("samples").is_none(), "the run record leaves the samples to the launch record");

        let kill = by_id(&records, "kill");
        assert_eq!(kill["classification"], "exceeds_base_machine_budget", "{kill}");
        assert!(kill["classification_detail"].as_str().unwrap().ends_with("_above_budget_4194304_killed"), "{kill}");
        assert_eq!(kill["launch"]["killed_by_sampler"], true);
        assert_eq!(kill["launch"]["exit_code"], -9);
        assert_eq!(kill["launch"]["survivors"], json!([]));
        assert!(kill["last_stage"]["stage"].is_string(), "a stopped run records the stage it reached: {kill}");
        assert!(kill["footprint_watchdog"]["kill_footprint_bytes"].as_u64().unwrap() > 4 << 20);

        let timeout = by_id(&records, "timeout");
        assert_eq!(timeout["classification"], "timed_out");
        assert_eq!(timeout["classification_detail"], "after_0_s");
        assert_eq!(timeout["launch"]["timed_out"], true);
        assert_eq!(timeout["launch"]["survivors"], json!([]));

        let refused = by_id(&records, "refused");
        assert_eq!(refused["classification"], "refused");
        assert_eq!(refused["classification_detail"], "SUPPORT_INPUT_MISSING");
        assert_eq!(refused["as_expected"], true);

        assert_eq!(by_id(&records, "missing")["classification"], "model_missing");
        let skipped = by_id(&records, "skipped");
        assert_eq!(skipped["classification"], "skipped_window_budget");
        assert!(skipped.get("launch").is_none());

        let written: Value = serde_json::from_str(&std::fs::read_to_string(out.join("packet.json")).unwrap()).unwrap();
        assert_eq!(written, packet(&out));
        assert_eq!(written["schema"], PACKET_SCHEMA);
        assert_eq!(written["observation_only"], true);
        assert_eq!(
            written["classification_counts"],
            json!({"ok": 1, "exceeds_base_machine_budget": 1, "timed_out": 1, "refused": 1, "model_missing": 1, "skipped_window_budget": 1})
        );
        assert_eq!(written["runs"].as_array().unwrap().len(), 4);
        assert_eq!(written["not_run"].as_array().unwrap().len(), 2);
        assert!(written["driver_limits"].as_array().is_some_and(|l| !l.is_empty()));
        let md = &written["metadata"];
        assert_eq!(md["runner"], "release_timing --supervise");
        assert_eq!(md["os"], "Darwin");
        assert_eq!(md["binary_name"], "release_timing");
        assert_eq!(md["binary_sha256"].as_str().unwrap().len(), 64);
        assert!(md["memory_bytes"].as_str().unwrap().parse::<u64>().unwrap() > 0);
        // No machine path anywhere in the records.
        for name in ["records.jsonl", "packet.json", "metadata.json", "kill.stderr.txt", "kill.record.json"] {
            let text = std::fs::read_to_string(out.join(name)).unwrap();
            assert!(!text.contains(dir.to_str().unwrap()) && !text.contains("/Users/"), "{name}");
        }
        for row in written["runs"].as_array().unwrap() {
            for key in ["pass", "passed", "threshold", "within_bound"] {
                assert!(row.get(key).is_none(), "observations only");
            }
        }
        // A folder with records is refused.
        let (code, _, stderr) = supervise(&dir, vec![run(0, "ok", "demo.json", 60, 1, "solved", BUDGET)], 900);
        assert_eq!(code, 2);
        assert!(stderr.contains("records already exist"), "{stderr}");
        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn a_model_that_differs_from_the_plan_is_not_run() {
        let dir = folder("mismatch");
        std::fs::write(dir.join("demo.json"), demo()).unwrap();
        let mut entry = run(0, "mismatch", "demo.json", 60, 1, "solved", BUDGET);
        entry["expected_model_sha256"] = json!("0".repeat(64));
        let mut listed = run(1, "listed", "demo.json", 60, 1, "solved", BUDGET);
        listed["expected_model_sha256"] = json!(open_pipe_stress_release_timing::supervise::sha256_hex(&demo()));
        listed["model_stats"] = json!({"nodes": 7});
        let (code, _, _) = supervise(&dir, vec![entry, listed], 900);
        assert_eq!(code, 1);
        let records = read_records(&dir.join("out"));
        let mismatch = by_id(&records, "mismatch");
        assert_eq!(mismatch["classification"], "model_mismatch");
        assert!(mismatch.get("launch").is_none());
        let listed = by_id(&records, "listed");
        assert_eq!(listed["classification"], "ok");
        assert_eq!(listed["model_stats"], json!({"nodes": 7}));
        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn plans_it_cannot_honour_are_refused() {
        let dir = folder("refuse");
        let mut attributed = run(0, "a-attr", "demo.json", 60, 1, "solved", BUDGET);
        attributed["attribute"] = json!(true);
        let (code, _, stderr) = supervise(&dir, vec![attributed], 900);
        assert_eq!(code, 2);
        assert!(stderr.contains("attribution run"), "{stderr}");
        let (code, _, stderr) = supervise(&dir, vec![run(0, "up", "../demo.json", 60, 1, "solved", BUDGET)], 900);
        assert_eq!(code, 2);
        assert!(stderr.contains("file name"), "{stderr}");
        assert!(!dir.join("out").join("records.jsonl").exists());
        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn the_driver_mode_is_unchanged_by_the_supervise_mode() {
        // Without --supervise first, the arguments are the driver's, as before.
        let output = Command::new(BIN).args(["--out", "x"]).output().unwrap();
        assert_eq!(output.status.code(), Some(2));
        assert!(String::from_utf8_lossy(&output.stderr).contains("unknown option --out"));
    }

    #[test]
    fn a_terminated_supervisor_kills_its_driver_first() {
        // RV18-3: SIGTERM to the supervisor kills the running driver's group before it exits.
        let dir = folder("term");
        // A large invented array makes the driver's `read` and `parse` long enough to be caught running.
        let big = format!("{{\"nodes\": [{}]}}", vec!["{\"id\": \"N\", \"x\": 0.0}"; 1_000_000].join(","));
        std::fs::write(dir.join("big.json"), big).unwrap();
        let plan = json!({"schema": "release-timing-plan-v1", "window_budget_s": 900, "quiet_wait_s": 0,
                          "runs": [run(0, "big", "big.json", 600, 1, "solved", BUDGET)]});
        std::fs::write(dir.join("plan.json"), plan.to_string()).unwrap();
        let mut supervisor =
            Command::new(BIN).args(["--supervise", "plan.json", "--out", "out"]).current_dir(&dir).spawn().unwrap();
        // Wait until the driver has started (its stdout record exists), then terminate.
        let started = std::time::Instant::now();
        while !dir.join("out").join("big.jsonl").exists() && started.elapsed().as_secs() < 60 {
            std::thread::sleep(std::time::Duration::from_millis(20));
        }
        unsafe { libc::kill(supervisor.id() as i32, libc::SIGTERM) };
        let status = supervisor.wait().unwrap();
        assert_eq!(status.code(), Some(128 + libc::SIGTERM));
        // No process is left running the big model.
        let ps = Command::new("/bin/ps").args(["-axo", "command="]).output().unwrap();
        let listed = String::from_utf8_lossy(&ps.stdout).to_string();
        assert!(!listed.lines().any(|l| l.contains(dir.join("big.json").to_str().unwrap())), "the driver outlived its supervisor");
        std::fs::remove_dir_all(dir).unwrap();
    }
}
