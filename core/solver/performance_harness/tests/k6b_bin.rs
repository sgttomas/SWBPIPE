//! K6b: the observation binary's `w1a` mode, run as a subprocess (the debug
//! build). Every run is small (10 members). No time or memory bound is
//! asserted.

use open_pipe_stress_solver_performance_harness::k6::models::model;
use open_pipe_stress_solver_performance_harness::k6::w1::adapter::source;
use open_pipe_stress_solver_performance_harness::k6::Mode;
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

fn lines_of<'a>(text: &'a str, kind: &str) -> Vec<&'a str> {
    let tag = format!("\"kind\":\"{kind}\"");
    text.lines().filter(|l| l.contains(&tag)).collect()
}

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

fn temp_dir(tag: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("k6b_bin_{tag}_{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

/// The w1a mode: counts with W1's keys, one outcome and two attempt lines per
/// repeat, every parity item true, the prefixes, and the rows dump.
#[test]
fn w1a_prints_its_lines_and_every_parity_holds() {
    let dir = temp_dir("w1a");
    let rows = dir.join("rows.txt");
    let rows_text = rows.to_string_lossy().into_owned();
    let output = run(&[
        "--model",
        "RF-LARGE-CONT-n00010-ROT",
        "--mode",
        "w1a",
        "--heap-cap-bytes",
        CAP_512_MIB,
        "--repeats",
        "2",
        "--w1-prefixes",
        "--dump-published",
        &rows_text,
    ]);
    let text = stdout(&output);
    assert!(output.status.success(), "{text}");
    let start = lines_of(&text, "start")[0];
    assert!(start.contains("\"case_limit\":18446744073709551615"));
    assert!(start.contains("\"invocation_limit\":18446744073709551615"));
    let counts = lines_of(&text, "counts")[0];
    for key in [
        "estimate_adm_bytes_w1a",
        "estimate_w1_sel128_bytes",
        "w1_profile_entries",
        "w1_rows",
        "w1_limbs_per_entry_1024",
        "w1_source_encoding_fnv64",
    ] {
        assert!(counts.contains(&format!("\"{key}\":")), "{key}");
    }
    let outcomes = lines_of(&text, "outcome");
    assert_eq!(outcomes.len(), 2);
    assert!(outcomes
        .iter()
        .all(|l| l.contains("\"class\":\"Selected\"") && l.contains("\"selected_precision\":128")));
    let attempts = lines_of(&text, "attempt");
    assert_eq!(attempts.len(), 4);
    // Completed builds: nothing charged is unstaged (ROOT's ruling on the
    // K6B-S3 stop).
    assert!(attempts
        .iter()
        .all(|l| l.contains("\"stages_complete\":true")
            && l.contains("\"own_unstaged\":0,")
            && l.contains("\"shared_unstaged\":0,")));
    let parity = lines_of(&text, "parity");
    for item in [
        "w1_profile_equals_storage",
        "w1_pattern_equals_storage",
        "w1_limbs_equal_table",
        "w1_stages_equal_totals",
        "w1_work_closes",
        "w1_source_encoding_equals_evidence",
        "w1_budget_not_reached",
        "repeat_determinism",
        "w1_prefix_segments",
    ] {
        assert!(
            parity
                .iter()
                .any(|l| l.contains(&format!("\"item\":\"{item}\""))),
            "{item}"
        );
    }
    assert!(
        parity.iter().all(|l| l.contains("\"equal\":true")),
        "{text}"
    );
    assert_eq!(lines_of(&text, "prefix").len(), 3);
    let stages: Vec<&str> = lines_of(&text, "stage");
    assert!(stages
        .iter()
        .any(|l| l.contains("\"stage\":\"w1_prefix_3\"")));
    let summary = lines_of(&text, "summary")[0];
    assert!(number(summary, "prefix_phase_heap_peak").is_some());
    let dump = std::fs::read_to_string(&rows).unwrap();
    assert!(dump.starts_with("k6b-rows v1\nmodel RF-LARGE-CONT-n00010-ROT\n"));
    let published = number(outcomes[0], "rows").unwrap() as usize;
    assert_eq!(dump.lines().count(), published + 2);
    let _ = std::fs::remove_dir_all(&dir);
}

/// `--emit-source` prints the adapter's K4SRC bytes.
#[test]
fn emit_source_prints_the_adapters_encoding() {
    let id = "RF-LARGE-TREE-n00010-AX";
    let output = run(&["--emit-source", "--model", id]);
    assert!(output.status.success());
    let hex: String = source(&model(id).unwrap())
        .unwrap()
        .encoding()
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect();
    assert_eq!(stdout(&output).trim(), hex);
}

/// w1a is refused only by the half-cap estimate rule (it is never an n² mode),
/// and a counts file must carry W1's counts.
#[test]
fn w1a_admission_and_counts_file() {
    let counts = run(&[
        "--counts-only",
        "--model",
        "RF-LARGE-CHAIN-n00010-AX",
        "--heap-cap-bytes",
        CAP_512_MIB,
    ]);
    let text = stdout(&counts);
    let line = lines_of(&text, "counts")[0];
    let estimate = number(line, "estimate_adm_bytes_w1a").expect("W1's estimate");
    let dir = temp_dir("counts");
    let path = dir.join("counts.jsonl");
    std::fs::write(&path, &text).unwrap();
    let path_text = path.to_string_lossy().into_owned();
    // A cap under twice the estimate is refused by name, before any solve.
    let cap = (estimate + estimate / 2).to_string();
    let refused = run(&[
        "--model",
        "RF-LARGE-CHAIN-n00010-AX",
        "--mode",
        "w1a",
        "--heap-cap-bytes",
        &cap,
        "--counts-file",
        &path_text,
    ]);
    let refused_text = stdout(&refused);
    assert_eq!(refused.status.code(), Some(3), "{refused_text}");
    assert!(refused_text.contains("\"reason\":\"estimate_exceeds_half_cap\""));
    assert!(!refused_text.contains("\"stage\":\"w1_solve\""));
    // With the counts file and a sufficient cap, the run takes its counts from
    // the file.
    let ok = run(&[
        "--model",
        "RF-LARGE-CHAIN-n00010-AX",
        "--mode",
        "w1a",
        "--heap-cap-bytes",
        CAP_512_MIB,
        "--repeats",
        "1",
        "--counts-file",
        &path_text,
    ]);
    let ok_text = stdout(&ok);
    assert!(ok.status.success(), "{ok_text}");
    assert!(ok_text.contains("\"counts_source\":\"file\""));
    // K6's own counts lines carry no W1 counts: the w1a mode refuses them.
    let k6_counts = run(&[
        "--model",
        "RF-LARGE-CHAIN-n00010-AX",
        "--mode",
        "sparse",
        "--heap-cap-bytes",
        CAP_512_MIB,
        "--repeats",
        "1",
    ]);
    let k6_text = stdout(&k6_counts);
    assert!(!lines_of(&k6_text, "counts")[0].contains("w1_"));
    let k6_path = dir.join("k6_counts.jsonl");
    std::fs::write(&k6_path, &k6_text).unwrap();
    let no_w1 = run(&[
        "--model",
        "RF-LARGE-CHAIN-n00010-AX",
        "--mode",
        "w1a",
        "--heap-cap-bytes",
        CAP_512_MIB,
        "--counts-file",
        &k6_path.to_string_lossy(),
    ]);
    assert_eq!(no_w1.status.code(), Some(2));
    // w1a is not an n² mode, so no by-name refusal applies to it at any size
    // (the half-cap rule decides; no debug run above 100 members).
    assert!(!Mode::W1a.materializes_n2());
    assert_eq!(Mode::parse("w1a"), Some(Mode::W1a));
    let _ = std::fs::remove_dir_all(&dir);
}
