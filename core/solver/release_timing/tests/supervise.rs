//! The supervise mode's tests. `Classification` and `ProgressLines` port the cases of
//! `core/solver/performance_harness/runner/test_release_runner.py`'s classes of those names:
//! the same inputs, the same outputs; `classification_the_reading_at_exit_*` is this supervisor's
//! one boundary case beyond them. The live tests (macOS) run this crate's binary in supervise
//! mode on the invented demo model, and the launch tests drive `supervise::launch` directly: a
//! record destination that is not a regular file is refused before any spawn, and a failure after
//! the spawn leaves no driver process. The cross-check against the Python supervisor is
//! `tests/test_release_timing_supervise.py`. Nothing here measures or asserts a time or memory
//! bound.

use open_pipe_stress_release_timing::classify::{
    as_expected, classification_readings, classify, last_progress, readings_record, sanitize, CHECKS_REPORTED, CLASSES,
    PROGRESS_PREFIX, SOLVED,
};
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
    let mut line = json!({"schema": "release-timing-v2", "status": status, "blocking_codes": codes, "route": "EX",
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
        "driver_peak_footprint_bytes": footprint, "limits": ["a limit"]});
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
fn classification_the_reading_at_exit_can_make_an_rss_cap_kill_exceed_the_budget() {
    // The independent review's boundary case: the emergency RSS cap kills the driver while every
    // sampled footprint is within the budget, and the driver crosses the budget before SIGKILL
    // takes effect. This supervisor reads the exited driver's lifetime maximum before reaping it
    // and counts it (the Agent 0's ruling: the stricter observation is kept); the Python runner's
    // /usr/bin/time is killed with the group, so its record has no such reading and classes the
    // same run killed_by_rss_cap. The record names the readings each used.
    let sampled = json!({"peak_phys_footprint_bytes": BUDGET - 1, "peak_lifetime_max_phys_footprint_bytes": BUDGET - 1});
    let rust = launched(
        "killed_by_rss_watchdog",
        -9,
        json!({"rss": {"exit_lifetime_max_phys_footprint_bytes": BUDGET + 1}, "sampler": sampled.clone()}),
    );
    let (cls, detail) = c(&rust, None, EMERGENCY, BUDGET);
    assert_eq!(cls, "exceeds_base_machine_budget");
    assert_eq!(detail.as_deref(), Some(format!("killed_above_rss_cap_{EMERGENCY}_footprint_{}", BUDGET + 1).as_str()));
    let readings = readings_record(&rust, None, None);
    assert_eq!(
        readings["used"],
        json!(["sampler:peak_phys_footprint_bytes", "sampler:peak_lifetime_max_phys_footprint_bytes",
               "rss:exit_lifetime_max_phys_footprint_bytes"])
    );
    assert_eq!(readings["at_exit"], "rss:exit_lifetime_max_phys_footprint_bytes");
    assert!(readings["at_exit_absent"].is_null());
    // The same run as the Python runner records it, without the reading at exit.
    let python = launched("killed_by_rss_watchdog", -9, json!({"rss": {"ps_max_kib": 1}, "sampler": sampled}));
    assert_eq!(c(&python, None, EMERGENCY, BUDGET).0, "killed_by_rss_cap");
    let readings = readings_record(&python, None, None);
    assert!(readings["at_exit"].is_null() && readings["at_exit_absent"].is_string(), "{readings}");
}

#[test]
fn classification_readings_follow_the_branch_that_decided() {
    let killed = launched("killed_by_sampler", -9, json!({"sampler": {"kill_footprint_bytes": BUDGET + 7}}));
    assert_eq!(classification_readings(&killed, None, None), ["sampler:kill_footprint_bytes"]);
    assert!(classification_readings(&launched("timed_out", -9, json!({})), None, None).is_empty());
    assert!(classification_readings(&launched("error", 101, json!({})), None, None).is_empty());
    let progress = json!({"phys_footprint_bytes": 5, "lifetime_max_phys_footprint_bytes": 6});
    assert_eq!(
        classification_readings(&ok(), Some(&solved()), Some(&progress)),
        ["rss:time_peak_footprint_bytes", "driver:driver_peak_footprint_bytes", "progress:phys_footprint_bytes",
         "progress:lifetime_max_phys_footprint_bytes"]
    );
}

#[test]
fn classification_a_model_checks_report_is_ok() {
    assert_eq!(c(&ok(), Some(&driver(CHECKS_REPORTED, &[], GIB, json!({}))), BUDGET, BUDGET), ("ok".into(), None));
    assert_eq!(c(&ok(), Some(&driver(CHECKS_REPORTED, &[], BUDGET + 1, json!({}))), BUDGET, BUDGET).0, "exceeds_base_machine_budget");
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
    use open_pipe_stress_release_timing::supervise::{
        check_run_id, launch, packet, read_records, record_error_class, stale_driver_message, Hooks, OutDir, WriteWatch,
        PACKET_SCHEMA, PLAN_SCHEMA, RECORDS_INCOMPLETE_EXIT, RECORD_SCHEMA, TERMINATING_SIGNALS,
    };
    use serde_json::{json, Value};
    use std::path::{Path, PathBuf};
    use std::process::Command;
    use std::time::{Duration, Instant};

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
        json!({"order": order, "run_id": run_id, "task": "solve", "profile": "linear-ex", "nodes": 100, "mode": "sparse",
               "timeout_s": timeout_s, "footprint_budget_bytes": budget, "rss_cap_bytes": 16u64 << 30,
               "estimate_s": estimate_s, "expect": expect, "attribute": false, "model": model,
               "expected_model_sha256": null, "model_stats": null})
    }

    fn supervise(dir: &Path, runs: Vec<Value>, window_budget_s: i64) -> (i32, String, String) {
        let plan = json!({"schema": PLAN_SCHEMA, "window_budget_s": window_budget_s, "quiet_wait_s": 0, "runs": runs});
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
            // The desktop's on-demand model checks: a run, and a driver process, of their own.
            {
                let mut checks = run(6, "model-checks-demo", "demo.json", 60, 1, "ok", BUDGET);
                checks["task"] = json!("model_checks");
                checks
            },
        ];
        let (code, stdout, stderr) = supervise(&dir, runs, 900);
        assert_eq!(code, 1, "model_missing makes the exit 1, as the Python --run: {stdout}{stderr}");
        let out = dir.join("out");
        let records = read_records(&out);
        assert_eq!(records.len(), 7);
        assert!(records.iter().all(|r| r["schema"] == RECORD_SCHEMA));
        let ok = by_id(&records, "ok");
        assert_eq!(ok["classification"], "ok", "{ok}");
        assert_eq!(ok["as_expected"], true);
        assert_eq!(ok["driver"]["status"], "MECHANICS_SOLVED");
        // The solve's last stage is parse_back: nothing runs after it in the solve's process.
        assert_eq!(ok["last_stage"]["stage"], "parse_back");
        assert_eq!(ok["stages_finished"], 7);
        assert!(ok["driver"].get("model_checks").is_none() && ok["driver"].get("side_stages").is_none());
        assert_eq!(ok["launch"]["argv"], json!(["release_timing", "demo.json", "--mode", "sparse"]));
        assert_eq!(ok["classification_readings"]["at_exit"], "rss:exit_lifetime_max_phys_footprint_bytes");
        assert!(ok["classification_readings"]["used"].as_array().unwrap().contains(&json!("rss:exit_lifetime_max_phys_footprint_bytes")));
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

        let checks = by_id(&records, "model-checks-demo");
        assert_eq!(checks["classification"], "ok", "{checks}");
        assert_eq!(checks["as_expected"], true);
        assert_eq!(checks["task"], "model_checks");
        assert_eq!(checks["launch"]["argv"], json!(["release_timing", "demo.json", "--model-checks"]));
        assert_eq!(checks["driver"]["invocation"], "model_checks");
        assert_eq!(checks["driver"]["status"], "MODEL_CHECKS_REPORTED");
        assert_eq!((checks["last_stage"]["stage"].as_str(), checks["stages_finished"].as_u64()), (Some("serialize"), Some(4)));

        assert_eq!(by_id(&records, "missing")["classification"], "model_missing");
        let skipped = by_id(&records, "skipped");
        assert_eq!(skipped["classification"], "skipped_window_budget");
        assert!(skipped.get("launch").is_none());

        let written: Value = serde_json::from_str(&std::fs::read_to_string(out.join("packet.json")).unwrap()).unwrap();
        assert_eq!(written, packet(&out).unwrap());
        assert_eq!(written["schema"], PACKET_SCHEMA);
        assert_eq!(written["observation_only"], true);
        assert_eq!(
            written["classification_counts"],
            json!({"ok": 2, "exceeds_base_machine_budget": 1, "timed_out": 1, "refused": 1, "model_missing": 1, "skipped_window_budget": 1})
        );
        assert_eq!(written["runs"].as_array().unwrap().len(), 5);
        assert!(written["runs"].as_array().unwrap().iter().any(|r| r["task"] == "model_checks"));
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
        let mut untasked = run(0, "t", "demo.json", 60, 1, "solved", BUDGET);
        untasked["task"] = json!("side_stage");
        let (code, _, stderr) = supervise(&dir, vec![untasked], 900);
        assert_eq!(code, 2);
        assert!(stderr.contains("task must be solve or model_checks"), "{stderr}");
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
        let plan = json!({"schema": PLAN_SCHEMA, "window_budget_s": 900, "quiet_wait_s": 0,
                          "runs": [run(0, "big", "big.json", 600, 1, "solved", BUDGET)]});
        std::fs::write(dir.join("plan.json"), plan.to_string()).unwrap();
        let mut supervisor =
            Command::new(BIN).args(["--supervise", "plan.json", "--out", "out"]).current_dir(&dir).spawn().unwrap();
        // Wait until the driver is running (its process is listed; its record files are opened
        // before it is spawned, so they show nothing), then terminate.
        let big = dir.join("big.json").to_str().unwrap().to_string();
        let started = Instant::now();
        while driver_pids(&big).is_empty() && started.elapsed().as_secs() < 60 {
            std::thread::sleep(Duration::from_millis(20));
        }
        let drivers = driver_pids(&big);
        assert_eq!(drivers.len(), 1, "the driver was seen running");
        unsafe { libc::kill(supervisor.id() as i32, libc::SIGTERM) };
        let status = supervisor.wait().unwrap();
        assert_eq!(status.code(), Some(128 + libc::SIGTERM));
        // No process is left running the big model, and the driver's pid is gone (killed and reaped).
        assert!(driver_pids(&big).is_empty(), "the driver outlived its supervisor");
        assert!(!alive(drivers[0]), "the driver was not reaped");
        std::fs::remove_dir_all(dir).unwrap();
    }

    /// The pids of processes whose command line contains `needle`.
    fn driver_pids(needle: &str) -> Vec<i32> {
        let ps = Command::new("/bin/ps").args(["-axo", "pid=,command="]).output().unwrap();
        String::from_utf8_lossy(&ps.stdout)
            .lines()
            .filter(|l| l.contains(needle) && !l.contains("--supervise"))
            .filter_map(|l| l.split_whitespace().next()?.parse().ok())
            .collect()
    }

    /// Whether `pid` names a process at all, a zombie included (`kill(pid, 0)`: ESRCH once reaped).
    fn alive(pid: i32) -> bool {
        unsafe { libc::kill(pid, 0) == 0 || std::io::Error::last_os_error().raw_os_error() != Some(libc::ESRCH) }
    }

    fn fifo(path: &Path) {
        let name = std::ffi::CString::new(path.to_str().unwrap()).unwrap();
        assert_eq!(unsafe { libc::mkfifo(name.as_ptr(), 0o600) }, 0, "mkfifo {}", path.display());
    }

    fn symlink(target: &Path, link: &Path) {
        let (target, link) = (std::ffi::CString::new(target.to_str().unwrap()).unwrap(), std::ffi::CString::new(link.to_str().unwrap()).unwrap());
        assert_eq!(unsafe { libc::symlink(target.as_ptr(), link.as_ptr()) }, 0);
    }

    /// `supervise::launch` of `/usr/bin/touch <marker>`, so that a spawn leaves a mark.
    fn launch_touch(record_dir: &Path, marker: &Path, after_spawn: &mut dyn FnMut(i32) -> std::io::Result<()>) -> std::io::Result<()> {
        let out = OutDir::open(record_dir)?;
        launch(Path::new("/usr/bin/touch"), &[marker.to_str().unwrap().to_string()], 16 << 30, BUDGET, 60, &out, "r", Hooks::none(after_spawn))
            .map(|_| ())
    }

    #[test]
    fn a_record_destination_that_is_not_a_regular_file_is_refused_before_any_spawn() {
        // The independent review's case: the stdout record is a FIFO. Opening it must neither block
        // nor start the driver.
        let dir = folder("fifo");
        for suffix in [".jsonl", ".stderr.txt", ".record.json"] {
            let records = dir.join(suffix.trim_start_matches('.').replace('.', "_"));
            std::fs::create_dir_all(&records).unwrap();
            fifo(&records.join(format!("r{suffix}")));
            let marker = dir.join(format!("spawned{suffix}"));
            let mut spawned = Vec::new();
            let started = Instant::now();
            let refused = launch_touch(&records, &marker, &mut |pid| {
                spawned.push(pid);
                Ok(())
            });
            assert!(started.elapsed() < Duration::from_secs(5), "opening the FIFO blocked");
            let error = refused.expect_err("a FIFO record is refused");
            assert!(error.to_string().contains(&format!("r{suffix}")), "{error}");
            assert!(spawned.is_empty() && !marker.exists(), "{suffix}: no driver was started");
        }
        // A symbolic link is not followed, and a symbolic link as the folder is not a real directory.
        let linked = dir.join("linked");
        std::fs::create_dir_all(&linked).unwrap();
        symlink(&dir.join("elsewhere.jsonl"), &linked.join("r.jsonl"));
        let marker = dir.join("spawned-link");
        assert!(launch_touch(&linked, &marker, &mut |_| Ok(())).is_err());
        assert!(!marker.exists() && !dir.join("elsewhere.jsonl").exists());
        symlink(&linked, &dir.join("folder-link"));
        assert!(launch_touch(&dir.join("folder-link"), &marker, &mut |_| Ok(())).is_err());
        assert!(!marker.exists());
        // A regular file, or a new one, is a destination.
        let plain = dir.join("plain");
        std::fs::create_dir_all(&plain).unwrap();
        std::fs::write(plain.join("r.stderr.txt"), "old").unwrap();
        launch_touch(&plain, &marker, &mut |_| Ok(())).expect("a regular destination");
        assert!(marker.exists());
        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn a_supervisor_whose_record_is_a_fifo_refuses_the_run_without_blocking() {
        // The whole binary, with the reviewer's zero-second timeout: it must end by itself, with no
        // driver started and no run record written.
        let dir = folder("fifo-cli");
        std::fs::write(dir.join("demo.json"), demo()).unwrap();
        std::fs::create_dir_all(dir.join("out")).unwrap();
        fifo(&dir.join("out").join("zero.jsonl"));
        let plan = json!({"schema": PLAN_SCHEMA, "window_budget_s": 900, "quiet_wait_s": 0,
                          "runs": [run(0, "zero", "demo.json", 0, 1, "timed_out", BUDGET)]});
        std::fs::write(dir.join("plan.json"), plan.to_string()).unwrap();
        let mut supervisor = Command::new(BIN)
            .args(["--supervise", "plan.json", "--out", "out"])
            .current_dir(&dir)
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .unwrap();
        let started = Instant::now();
        let status = loop {
            if let Some(status) = supervisor.try_wait().unwrap() {
                break status;
            }
            if started.elapsed() > Duration::from_secs(30) {
                supervisor.kill().unwrap();
                panic!("the supervisor blocked on the FIFO record");
            }
            std::thread::sleep(Duration::from_millis(20));
        };
        let mut stderr = String::new();
        std::io::Read::read_to_string(&mut supervisor.stderr.take().unwrap(), &mut stderr).unwrap();
        assert_eq!(status.code(), Some(2), "{stderr}");
        assert!(stderr.contains("cannot launch zero") && stderr.contains("zero.jsonl"), "{stderr}");
        assert!(driver_pids(dir.join("demo.json").to_str().unwrap()).is_empty());
        assert!(read_records(&dir.join("out")).is_empty(), "no run record");
        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn a_failure_after_the_spawn_leaves_no_driver_process() {
        // An error injected right after the spawn, and a panic there: either way the one owner of
        // the driver kills its group and reaps it before the error leaves `launch`.
        let dir = folder("injected");
        let sleeper = [String::from("30")];
        let mut seen = None;
        let out = OutDir::open(&dir).unwrap();
        let error = launch(Path::new("/bin/sleep"), &sleeper, 16 << 30, BUDGET, 60, &out, "r", Hooks::none(&mut |pid| {
            seen = Some(pid);
            Err(std::io::Error::other("injected after the spawn"))
        }))
        .err()
        .expect("the injected error is returned");
        assert!(error.to_string().contains("injected"));
        let pid = seen.expect("the driver was spawned");
        assert!(!alive(pid), "pid {pid} is still running or unreaped after the error");
        let panicked = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _ = launch(Path::new("/bin/sleep"), &sleeper, 16 << 30, BUDGET, 60, &out, "r", Hooks::none(&mut |pid| {
                seen = Some(pid);
                panic!("injected panic after the spawn")
            }));
        }));
        assert!(panicked.is_err());
        let pid = seen.unwrap();
        assert!(!alive(pid), "pid {pid} is still running or unreaped after the panic");
        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn a_run_id_that_is_not_a_bare_name_is_refused_at_plan_load() {
        // The review's case: run_id "../escaped" would name record files outside --out.
        for bad in ["../escaped", "a/b", ".hidden", "", "a b"] {
            assert!(check_run_id(bad).is_err(), "{bad:?}");
        }
        assert!(check_run_id(&"x".repeat(129)).is_err());
        for good in ["linear-ex-n2000-sparse", "model-checks-linear-ex-n1000", "a.b_c-1"] {
            assert!(check_run_id(good).is_ok(), "{good}");
        }
        let dir = folder("escaped");
        std::fs::write(dir.join("demo.json"), demo()).unwrap();
        let (code, _, stderr) = supervise(&dir, vec![run(0, "../escaped", "demo.json", 60, 1, "solved", BUDGET)], 900);
        assert_eq!(code, 2);
        assert!(stderr.contains("run ids must be bare names"), "{stderr}");
        assert!(!dir.join("out").exists(), "nothing written inside --out");
        let written: Vec<_> = std::fs::read_dir(&dir).unwrap().map(|e| e.unwrap().file_name()).collect();
        assert_eq!(written.len(), 2, "nothing written outside it either: {written:?}");
        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn a_swap_of_the_out_path_after_it_is_opened_does_not_redirect_the_records() {
        let dir = folder("swap");
        let (out_path, moved, elsewhere) = (dir.join("out"), dir.join("out-moved"), dir.join("elsewhere"));
        std::fs::create_dir_all(&out_path).unwrap();
        std::fs::create_dir_all(&elsewhere).unwrap();
        let out = OutDir::open(&out_path).unwrap();
        // The path now names another folder, through a symbolic link.
        std::fs::rename(&out_path, &moved).unwrap();
        symlink(&elsewhere, &out_path);
        let marker = dir.join("spawned");
        launch(Path::new("/usr/bin/touch"), &[marker.to_str().unwrap().to_string()], 16 << 30, BUDGET, 60, &out, "r", Hooks::none(&mut |_| Ok(())))
            .expect("the launch");
        for name in ["r.jsonl", "r.stderr.txt", "r.record.json"] {
            assert!(moved.join(name).is_file(), "{name} is in the folder that was opened");
        }
        assert_eq!(std::fs::read_dir(&elsewhere).unwrap().count(), 0, "nothing was redirected");
        std::fs::remove_dir_all(dir).unwrap();
    }

    /// A stdout record that cannot be written (a full disk, injected).
    struct Failing;
    impl std::io::Write for Failing {
        fn write(&mut self, _: &[u8]) -> std::io::Result<usize> {
            Err(std::io::Error::other("injected: no space left on device"))
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    #[test]
    fn a_stdout_record_that_cannot_be_written_is_the_runs_error_with_no_hash() {
        let dir = folder("full");
        std::fs::write(dir.join("demo.json"), demo()).unwrap();
        let out = OutDir::open(&dir).unwrap();
        let tail = [dir.join("demo.json").to_str().unwrap().to_string()];
        let hooks = Hooks { after_spawn: &mut |_| Ok(()), stdout_record: Some(Box::new(|_file| Box::new(Failing))) };
        let launched = launch(Path::new(BIN), &tail, 16 << 30, BUDGET, 60, &out, "r", hooks).expect("the launch");
        assert!(launched.stdout.contains("release-timing-v2"), "the driver's line was received from the pipe");
        assert!(launched.record["stdout_sha256"].is_null(), "no hash of bytes that were never written");
        assert!(launched.record["stdout_record_error"].as_str().unwrap().contains("injected"));
        let written: Value = serde_json::from_str(&std::fs::read_to_string(dir.join("r.record.json")).unwrap()).unwrap();
        assert!(written["stdout_sha256"].is_null() && written["stdout_record_error"].is_string());
        let launched = Value::Object(launched.record);
        let (class, detail) = record_error_class(&launched, "ok".into(), None);
        assert_eq!(class, "error");
        assert!(detail.unwrap().starts_with("stdout_record_write_failed: injected"));
        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn record_writes_are_bounded_by_a_watchdog_with_its_own_exit_status() {
        use std::sync::atomic::{AtomicBool, Ordering};
        use std::sync::Arc;
        let fired = Arc::new(AtomicBool::new(false));
        let flag = fired.clone();
        let stalled = WriteWatch::arm_with(Duration::from_millis(50), move || flag.store(true, Ordering::SeqCst));
        std::thread::sleep(Duration::from_millis(400));
        assert!(fired.load(Ordering::SeqCst), "a write still running after the limit ends it");
        drop(stalled);
        let quiet = Arc::new(AtomicBool::new(false));
        let flag = quiet.clone();
        drop(WriteWatch::arm_with(Duration::from_millis(50), move || flag.store(true, Ordering::SeqCst)));
        std::thread::sleep(Duration::from_millis(400));
        assert!(!quiet.load(Ordering::SeqCst), "a write that finished disarms it");
        assert!(![0, 1, 2].contains(&RECORDS_INCOMPLETE_EXIT) && RECORDS_INCOMPLETE_EXIT < 128);
    }

    /// Starts a supervisor on a model large enough to be caught running, and waits for its driver.
    fn supervised_big_run(name: &str) -> (PathBuf, std::process::Child, String, i32) {
        let dir = folder(name);
        let big = format!("{{\"nodes\": [{}]}}", vec!["{\"id\": \"N\", \"x\": 0.0}"; 1_000_000].join(","));
        std::fs::write(dir.join("big.json"), big).unwrap();
        let plan = json!({"schema": PLAN_SCHEMA, "window_budget_s": 900, "quiet_wait_s": 0,
                          "runs": [run(0, "big", "big.json", 600, 1, "solved", BUDGET)]});
        std::fs::write(dir.join("plan.json"), plan.to_string()).unwrap();
        let supervisor = Command::new(BIN)
            .args(["--supervise", "plan.json", "--out", "out"])
            .current_dir(&dir)
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn()
            .unwrap();
        let needle = dir.join("big.json").to_str().unwrap().to_string();
        let started = Instant::now();
        while driver_pids(&needle).is_empty() && started.elapsed().as_secs() < 60 {
            std::thread::sleep(Duration::from_millis(20));
        }
        let drivers = driver_pids(&needle);
        assert_eq!(drivers.len(), 1, "the driver was seen running");
        (dir, supervisor, needle, drivers[0])
    }

    fn gone_within(pid: i32, limit: Duration) -> bool {
        let started = Instant::now();
        while alive(pid) && started.elapsed() < limit {
            std::thread::sleep(Duration::from_millis(20));
        }
        !alive(pid)
    }

    #[test]
    fn a_driver_whose_supervisor_is_killed_with_sigkill_stops() {
        // SIGKILL runs no handler and no drop: the driver, in its own session, watches its parent.
        let (dir, mut supervisor, needle, driver) = supervised_big_run("sigkill");
        unsafe { libc::kill(supervisor.id() as i32, libc::SIGKILL) };
        supervisor.wait().unwrap();
        assert!(gone_within(driver, Duration::from_secs(15)), "driver {driver} outlived a killed supervisor");
        assert!(driver_pids(&needle).is_empty());
        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn the_other_terminating_signals_are_handled_as_sigterm_is() {
        assert!(TERMINATING_SIGNALS.contains(&libc::SIGQUIT) && TERMINATING_SIGNALS.contains(&libc::SIGUSR1));
        let (dir, mut supervisor, _, driver) = supervised_big_run("sigquit");
        unsafe { libc::kill(supervisor.id() as i32, libc::SIGQUIT) };
        assert_eq!(supervisor.wait().unwrap().code(), Some(128 + libc::SIGQUIT));
        assert!(!alive(driver), "the supervisor killed and reaped its driver before exiting");
        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn a_record_of_an_earlier_driver_is_refused_by_name() {
        let dir = folder("stale");
        let old = json!({"schema": RECORD_SCHEMA, "run_id": "old-run", "classification": "ok", "launch": {},
                         "driver": {"schema": "release-timing-v1", "side_stages": ["model_checks"]}});
        std::fs::write(dir.join("records.jsonl"), format!("{old}\n")).unwrap();
        let expected = "record old-run: its driver line has schema release-timing-v1, not release-timing-v2; \
                        an earlier driver's record is not repackaged";
        assert_eq!(stale_driver_message(&old).as_deref(), Some(expected));
        assert_eq!(packet(&dir), Err(expected.to_string()));
        std::fs::remove_dir_all(dir).unwrap();
    }
}
