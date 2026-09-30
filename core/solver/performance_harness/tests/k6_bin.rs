//! K6 tests F3, F4 and G, and RV18's tests of the time stops, the summary's
//! peak and the content-keyed lane refusal: the observation binary, run as a
//! subprocess (`CARGO_BIN_EXE_k6_observe`, the debug build). Every run is
//! small: at most 100 members, or a 10,000-member model refused before any
//! n² allocation.

use std::process::{Command, Output};

const BIN: &str = env!("CARGO_BIN_EXE_k6_observe");
const CAP_512_MIB: &str = "536870912";

fn run(args: &[&str]) -> Output {
    Command::new(BIN)
        .args(args)
        .output()
        .expect("k6_observe runs")
}

fn stdout(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).into_owned()
}

/// The integer value of `"key":<digits>` in the first line of `kind`.
fn field(text: &str, kind: &str, key: &str) -> Option<u128> {
    let line = text
        .lines()
        .find(|l| l.contains(&format!("\"kind\":\"{kind}\"")))?;
    let pattern = format!("\"{key}\":");
    let start = line.find(&pattern)? + pattern.len();
    line[start..]
        .chars()
        .take_while(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse()
        .ok()
}

fn refusal_reason(text: &str) -> Option<String> {
    let line = text.lines().find(|l| l.contains("\"kind\":\"refusal\""))?;
    let start = line.find("\"reason\":\"")? + "\"reason\":\"".len();
    Some(line[start..].split('"').next()?.to_string())
}

/// Every line of `kind`, in order.
fn lines_of<'a>(text: &'a str, kind: &str) -> Vec<&'a str> {
    let tag = format!("\"kind\":\"{kind}\"");
    text.lines().filter(|l| l.contains(&tag)).collect()
}

/// The integer value of `"key":<digits>` in one line.
fn number(line: &str, key: &str) -> Option<u128> {
    let pattern = format!("\"{key}\":");
    let start = line.find(&pattern)? + pattern.len();
    line[start..]
        .chars()
        .take_while(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse()
        .ok()
}

/// The value of `"key":` in one line: a string's contents, or `null`.
fn text_value(line: &str, key: &str) -> Option<String> {
    let pattern = format!("\"{key}\":");
    let rest = &line[line.find(&pattern)? + pattern.len()..];
    match rest.strip_prefix('"') {
        Some(inner) => Some(inner.split('"').next()?.to_string()),
        None if rest.starts_with("null") => Some("null".to_string()),
        None => None,
    }
}

/// F3: two runs of one model and mode give equal peak requested bytes (both
/// peak models), in sparse and dense mode.
#[test]
fn peak_requested_bytes_are_deterministic() {
    for mode in ["sparse", "dense"] {
        let args = [
            "--model",
            "RF-LARGE-CHAIN-n00010-ROT",
            "--mode",
            mode,
            "--heap-cap-bytes",
            CAP_512_MIB,
            "--repeats",
            "2",
        ];
        let first = run(&args);
        let second = run(&args);
        assert!(first.status.success() && second.status.success(), "{mode}");
        for key in ["repeats_heap_peak", "repeats_heap_peak_move"] {
            let a = field(&stdout(&first), "summary", key).expect(key);
            let b = field(&stdout(&second), "summary", key).expect(key);
            assert_eq!(a, b, "{mode} {key}");
            assert!(a > 0);
        }
    }
}

/// F4: at a small heap cap, a 100-member dense model aborts at the cap
/// (SIGABRT), with the allocator's marker and Rust's allocation-failure line.
#[test]
fn heap_cap_abort_is_marked() {
    let output = run(&[
        "--model",
        "RF-LARGE-CHAIN-n00100-AX",
        "--mode",
        "dense",
        "--heap-cap-bytes",
        "8388608",
        "--allow-over-estimate",
        "--repeats",
        "1",
    ]);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(!output.status.success());
    #[cfg(unix)]
    {
        use std::os::unix::process::ExitStatusExt;
        assert_eq!(
            output.status.signal(),
            Some(6),
            "SIGABRT expected: {stderr}"
        );
    }
    assert!(stderr.contains("k6_observe: heap cap refused "), "{stderr}");
    assert!(stderr.contains("memory allocation of "), "{stderr}");
    assert!(!stdout(&output).contains("\"kind\":\"summary\""));
}

/// `--allow-over-estimate` is refused above a 512 MiB heap cap (usage, exit 2).
#[test]
fn allow_over_estimate_needs_a_small_cap() {
    let output = run(&[
        "--model",
        "RF-LARGE-CHAIN-n00010-AX",
        "--mode",
        "dense",
        "--heap-cap-bytes",
        "536870913",
        "--allow-over-estimate",
    ]);
    assert_eq!(output.status.code(), Some(2));
}

/// G1: every n² mode at 10,000 members is refused by name before any count or
/// n² allocation (ROOT's ruling on RV16-N4).
#[test]
fn n2_modes_at_10000_refused_before_n2() {
    for mode in ["dense", "lane-lu"] {
        let output = run(&[
            "--model",
            "RF-LARGE-CHAIN-n10000-AX",
            "--mode",
            mode,
            "--heap-cap-bytes",
            CAP_512_MIB,
        ]);
        let text = stdout(&output);
        assert_eq!(output.status.code(), Some(3), "{mode}: {text}");
        assert_eq!(
            refusal_reason(&text).as_deref(),
            Some("n2_mode_at_or_above_10000_members"),
            "{mode}"
        );
        let peak = field(&text, "refusal", "heap_peak_so_far").expect("peak");
        assert!(
            peak < 64 * 1024 * 1024,
            "{mode}: {peak} bytes before the refusal"
        );
        assert!(
            !text.contains("\"kind\":\"counts\""),
            "{mode}: counted before refusing"
        );
    }
}

/// G2: a run whose admission estimate exceeds half the passed cap is refused
/// by name, in dense and in lane-id mode. The caps are taken from each
/// model's own counts line, so the counts phase fits under the cap while the
/// estimate exceeds half of it.
#[test]
fn estimate_over_half_cap_refused() {
    for (model, mode, key) in [
        (
            "RF-LARGE-CHAIN-n00100-AX",
            "dense",
            "estimate_adm_bytes_dense",
        ),
        (
            "RF-LARGE-CONT-n00100-ROT",
            "lane-id",
            "estimate_adm_bytes_lane_id",
        ),
    ] {
        let counts = run(&[
            "--counts-only",
            "--model",
            model,
            "--heap-cap-bytes",
            CAP_512_MIB,
        ]);
        let text = stdout(&counts);
        let estimate = field(&text, "counts", key).expect("estimate");
        let phase = field(&text, "counts", "phase_peak_heap").expect("phase peak");
        let cap = estimate + estimate / 2;
        assert!(
            cap > 2 * phase,
            "{model} {mode}: the counts phase ({phase}) would not fit well under the cap ({cap})"
        );
        let output = run(&[
            "--model",
            model,
            "--mode",
            mode,
            "--heap-cap-bytes",
            &cap.to_string(),
        ]);
        let text = stdout(&output);
        assert_eq!(output.status.code(), Some(3), "{model} {mode}: {text}");
        assert_eq!(
            refusal_reason(&text).as_deref(),
            Some("estimate_exceeds_half_cap"),
            "{model} {mode}"
        );
    }
}

/// G3: the identity-order lane on CONT n10000 is refused by name (ROOT's
/// rulings Q12 and on RV16-N4), before any lane entry is built.
#[test]
fn cont_n10000_identity_lane_refused() {
    for orientation in ["AX", "ROT"] {
        let model = format!("RF-LARGE-CONT-n10000-{orientation}");
        let output = run(&[
            "--model",
            &model,
            "--mode",
            "lane-id",
            "--heap-cap-bytes",
            CAP_512_MIB,
        ]);
        let text = stdout(&output);
        assert_eq!(output.status.code(), Some(3), "{model}: {text}");
        assert_eq!(
            refusal_reason(&text).as_deref(),
            Some("cont_n10000_identity_lane"),
            "{model}"
        );
        assert!(!text.contains("\"kind\":\"counts\""));
    }
}

/// The counts file stands in for the counts phase: the run carries no counts
/// phase and records the file as its source; a digest mismatch is refused.
#[test]
fn counts_file_replaces_the_counts_phase() {
    let dir = std::env::temp_dir().join(format!("k6_bin_counts_{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("counts.jsonl");
    let counts = run(&[
        "--counts-only",
        "--model",
        "RF-LARGE-CONT-n00010-AX",
        "--heap-cap-bytes",
        CAP_512_MIB,
    ]);
    std::fs::write(&path, stdout(&counts)).unwrap();
    let path_text = path.to_string_lossy().into_owned();
    let output = run(&[
        "--model",
        "RF-LARGE-CONT-n00010-AX",
        "--mode",
        "lane-id",
        "--heap-cap-bytes",
        CAP_512_MIB,
        "--repeats",
        "1",
        "--counts-file",
        &path_text,
    ]);
    let text = stdout(&output);
    assert!(output.status.success(), "{text}");
    assert!(text.contains("\"counts_source\":\"file\""));
    assert!(text.contains("\"counts_phase_heap_peak\":null"));
    // Another model's counts are not accepted for this model.
    let other = run(&[
        "--model",
        "RF-LARGE-CONT-n00010-ROT",
        "--mode",
        "lane-id",
        "--heap-cap-bytes",
        CAP_512_MIB,
        "--counts-file",
        &path_text,
    ]);
    assert_eq!(other.status.code(), Some(2));
    let _ = std::fs::remove_dir_all(&dir);
}

/// `--noop`, the process baseline (ROOT's A1-stop ruling Q2): a start line and
/// a summary, no model, no stage, exit 0.
#[test]
fn noop_prints_start_and_summary_only() {
    let output = run(&["--noop", "--heap-cap-bytes", CAP_512_MIB]);
    let text = stdout(&output);
    assert!(output.status.success(), "{text}");
    assert!(text.contains("\"kind\":\"start\"") && text.contains("\"noop\":true"));
    assert!(text.contains("\"kind\":\"summary\""));
    assert!(!text.contains("\"kind\":\"counts\"") && !text.contains("\"kind\":\"stage\""));
}

/// RV18-1: the binary's two time stops (Q7) on a 10-member sparse model.
/// - With three repeats, a first-repeat limit of 0 s and a time budget of 0 s each stop
///   after repeat 0 and give their reason, and a budget of 3,600 s lets all three run.
/// - A budget in whole seconds is exceeded only by a run of at least 1 s, so a fourth run
///   makes one: 1,000 repeats under a 1 s budget stop by time, long before the last (about
///   50 repeats in debug on the owner's Mac). A budget that never binds (RV18-M2) runs all
///   1,000.
#[test]
fn first_repeat_and_time_budget_stops() {
    let summary = |extra: &[&str]| -> (u128, String) {
        let mut args = vec![
            "--model",
            "RF-LARGE-CHAIN-n00010-AX",
            "--mode",
            "sparse",
            "--heap-cap-bytes",
            CAP_512_MIB,
        ];
        args.extend_from_slice(extra);
        let output = run(&args);
        let text = stdout(&output);
        assert!(output.status.success(), "{extra:?}: {text}");
        let line = *lines_of(&text, "summary").last().expect("summary");
        (
            number(line, "repeats_completed").expect("repeats_completed"),
            text_value(line, "stop_reason").expect("stop_reason"),
        )
    };
    assert_eq!(
        summary(&["--repeats", "3", "--first-repeat-limit-s", "0"]),
        (1, "first_repeat_over_limit".to_string())
    );
    assert_eq!(
        summary(&["--repeats", "3", "--time-budget-s", "0"]),
        (1, "time_budget".to_string())
    );
    assert_eq!(
        summary(&["--repeats", "3", "--time-budget-s", "3600"]),
        (3, "null".to_string())
    );
    let (completed, reason) = summary(&[
        "--repeats",
        "1000",
        "--entry-repeats",
        "1",
        "--time-budget-s",
        "1",
    ]);
    assert_eq!(reason, "time_budget");
    assert!(completed < 1000, "{completed} repeats under a 1 s budget");
}

/// RV18-2: the summary's repeats peak is the largest stage peak, in both peak models; the
/// process peak is at least that; and each stage's peak restarts at the stage. This is dense
/// CHAIN-n00010-AX with `--repeats 2 --entry-repeats 1`, the configuration of the dense
/// 1,000-member and ceiling runs: repeat 1's `assembly` peaks below repeat 0's `prepare`.
#[test]
fn summary_peak_is_the_largest_stage_peak_and_stage_peaks_restart() {
    let output = run(&[
        "--model",
        "RF-LARGE-CHAIN-n00010-AX",
        "--mode",
        "dense",
        "--heap-cap-bytes",
        CAP_512_MIB,
        "--repeats",
        "2",
        "--entry-repeats",
        "1",
    ]);
    let text = stdout(&output);
    assert!(output.status.success(), "{text}");
    let stages = lines_of(&text, "stage");
    let summary = *lines_of(&text, "summary").last().expect("summary");
    for (key, stage_key) in [
        ("repeats_heap_peak", "heap_peak"),
        ("repeats_heap_peak_move", "heap_peak_move"),
    ] {
        let largest = stages
            .iter()
            .map(|l| number(l, stage_key).expect(stage_key))
            .max()
            .expect("stage lines");
        assert_eq!(number(summary, key), Some(largest), "{key}");
    }
    assert!(number(summary, "heap_peak") >= number(summary, "repeats_heap_peak"));
    let peak = |repeat: u128, stage: &str| {
        stages
            .iter()
            .find(|l| {
                number(l, "repeat") == Some(repeat)
                    && text_value(l, "stage").as_deref() == Some(stage)
            })
            .and_then(|l| number(l, "heap_peak"))
            .expect(stage)
    };
    assert!(
        peak(1, "assembly") < peak(0, "prepare"),
        "repeat 1 assembly {} against repeat 0 prepare {}",
        peak(1, "assembly"),
        peak(0, "prepare")
    );
}

/// RV18-N6: CONT n10000's identity-order lane is refused from the model's content, not its
/// id. R1's CONT-n10000-AX, emitted and renamed, is still refused by name through
/// `--model-file`, before any count.
#[test]
fn renamed_cont_n10000_lane_id_refused() {
    let emitted = run(&["--emit-model", "--model", "RF-LARGE-CONT-n10000-AX"]);
    assert!(emitted.status.success());
    let text = String::from_utf8(emitted.stdout).expect("canonical bytes are UTF-8");
    let renamed = text.replacen(
        "id RF-LARGE-CONT-n10000-AX\n",
        "id renamed-continuous-beam\n",
        1,
    );
    assert_ne!(renamed, text, "the id line is renamed");
    let path = std::env::temp_dir().join(format!(
        "k6_bin_renamed_cont_{}.k6model",
        std::process::id()
    ));
    std::fs::write(&path, renamed).expect("write the renamed model");
    let output = run(&[
        "--model-file",
        path.to_str().expect("path"),
        "--mode",
        "lane-id",
        "--heap-cap-bytes",
        CAP_512_MIB,
    ]);
    let _ = std::fs::remove_file(&path);
    let out = stdout(&output);
    assert_eq!(output.status.code(), Some(3), "{out}");
    assert_eq!(
        refusal_reason(&out).as_deref(),
        Some("cont_n10000_identity_lane")
    );
    assert!(
        !out.contains("\"kind\":\"counts\""),
        "counted before refusing"
    );
}
