//! `openpipestress-runner read`: the retained reader's verdict through the CLI
//! over the shared corpus's bases and the producer-solved successors. The
//! outcome is the reader's own `outcome_json` of `retained_precision::validate`
//! and `validate_transport_metadata`, run in process here for comparison, and
//! the corpus's Rust expectation.
use open_pipe_stress_result_export::retained_precision as rp;
use serde_json::{json, Value};
use std::io::Write;
use std::process::{Command, Stdio};

const CORPUS_FILES: [&str; 2] = [
    include_str!("../../../../fixtures/results/retained_precision_cases.json"),
    include_str!("../../../../fixtures/results/retained_precision_cases_07o.json"),
];
const SUCCESSORS: [&str; 10] = [
    include_str!("../../../../fixtures/results/retained_precision_milestone_successor_sparse_interactive.json"),
    include_str!("../../../../fixtures/results/retained_precision_milestone_successor_dense_scrutiny.json"),
    include_str!("../../../../fixtures/results/retained_precision_l0_successor_sparse_interactive.json"),
    include_str!("../../../../fixtures/results/retained_precision_l0_successor_dense_scrutiny.json"),
    include_str!("../../../../fixtures/results/retained_precision_combination_successor_sparse_interactive.json"),
    include_str!("../../../../fixtures/results/retained_precision_combination_successor_dense_scrutiny.json"),
    include_str!("../../../../fixtures/results/retained_precision_w_c2_successor_sparse_interactive.json"),
    include_str!("../../../../fixtures/results/retained_precision_w_c2_successor_dense_scrutiny.json"),
    include_str!("../../../../fixtures/results/retained_precision_exact_successor_sparse_interactive.json"),
    include_str!("../../../../fixtures/results/retained_precision_exact_successor_dense_scrutiny.json"),
];

fn cases() -> Vec<Value> {
    CORPUS_FILES
        .iter()
        .flat_map(|text| {
            let file: Value = serde_json::from_str(text).unwrap();
            file["cases"].as_array().unwrap().clone()
        })
        .collect()
}

/// Run `read <reader>` with `input` on stdin: (exit code, stdout JSON or Null, stderr).
fn run(args: &[&str], input: &str) -> (i32, Value, String) {
    let mut child = Command::new(env!("CARGO_BIN_EXE_openpipestress-runner"))
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    // An unusable invocation exits before reading stdin; its pipe may close first.
    if let Err(error) = child.stdin.take().unwrap().write_all(input.as_bytes()) {
        assert_eq!(error.kind(), std::io::ErrorKind::BrokenPipe);
    }
    let output = child.wait_with_output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let verdict = if stdout.is_empty() {
        Value::Null
    } else {
        assert!(
            stdout.ends_with('\n') && stdout.matches('\n').count() == 1,
            "one JSON line"
        );
        serde_json::from_str(&stdout).unwrap()
    };
    (
        output.status.code().unwrap(),
        verdict,
        String::from_utf8(output.stderr).unwrap(),
    )
}

/// The CLI verdict a reader result must produce: exit 0 or 1 and the reader's
/// own `outcome_json` in the command's envelope.
fn expected(reader: &str, result: Result<rp::Validation, rp::ValidationError>) -> (i32, Value) {
    let code = if result.is_ok() { 0 } else { 1 };
    let verdict = json!({
        "artifact": "openpipestress.result_reader_verdict",
        "schema_version": "1.0.0",
        "reader": reader,
        "outcome": rp::outcome_json(&result),
    });
    (code, verdict)
}

/// Bound, unbound and transport reads of one statement through the CLI.
fn read_three_ways(source: &Value, invocation: &Value) -> [(i32, Value); 3] {
    let bound = json!({"source": source, "invocation": invocation}).to_string();
    let unbound = json!({"source": source}).to_string();
    let reads = [
        ("retained-precision", bound),
        ("retained-precision", unbound.clone()),
        ("retained-precision-transport", unbound),
    ];
    let want = [
        expected("retained-precision", rp::validate(source, Some(invocation))),
        expected("retained-precision", rp::validate(source, None)),
        expected(
            "retained-precision-transport",
            rp::validate_transport_metadata(source),
        ),
    ];
    let mut got = Vec::new();
    for ((reader, input), want) in reads.iter().zip(&want) {
        let (code, verdict, stderr) = run(&["read", reader], input);
        assert!(stderr.is_empty(), "{stderr}");
        assert_eq!((code, &verdict), (want.0, &want.1));
        got.push((code, verdict));
    }
    got.try_into().unwrap()
}

#[test]
fn corpus_bases_read_through_the_cli_as_the_reader_and_the_corpus_state() {
    let cases = cases();
    assert_eq!(cases.len(), 45);
    for case in &cases {
        let [bound, unbound, transport] = read_three_ways(&case["source"], &case["invocation"]);
        let id = &case["id"];
        if case["expected"].get("gate").is_some() {
            // The corpus's refused bases: refused bound, passed unbound and on transport.
            let refusal = &bound.1["outcome"]["error"];
            assert_eq!(
                json!({"gate": refusal["gate"], "code": refusal["code"]}),
                case["expected"],
                "{id}"
            );
            assert_eq!((unbound.0, transport.0), (0, 0), "{id}");
            continue;
        }
        assert_eq!(bound.0, 0, "{id}: {}", bound.1);
        let validation = &bound.1["outcome"]["validation"];
        assert_eq!(validation["invocation_bound"], true, "{id}");
        assert_eq!(
            validation["numerical_eligible"], case["expected"]["numerical_eligible"],
            "{id}"
        );
        assert_eq!(
            validation["classifications"], case["expected_classifications"],
            "{id}"
        );
        assert_eq!(
            validation["publication_sha256"],
            case["source"]["retained_precision"]["body"]["publication_sha256"],
            "{id}"
        );
        assert_eq!(
            unbound.1["outcome"]["validation"]["numerical_eligible"], false,
            "{id}"
        );
        assert_eq!(
            transport.1["outcome"]["validation"]["classifications"],
            json!([]),
            "{id}"
        );
    }
}

#[test]
fn producer_solved_successors_read_through_the_cli_and_bind_eligible() {
    for text in SUCCESSORS {
        let fixture: Value = serde_json::from_str(text).unwrap();
        let [bound, unbound, transport] =
            read_three_ways(&fixture["source"], &fixture["invocation"]);
        let id = &fixture["id"];
        assert_eq!(bound.0, 0, "{id}: {}", bound.1);
        assert_eq!(
            bound.1["outcome"]["validation"]["invocation_bound"], true,
            "{id}"
        );
        assert_eq!((unbound.0, transport.0), (0, 0), "{id}");
    }
}

#[test]
fn the_cli_reads_the_input_bytes_as_given_and_refuses_unusable_input_with_exit_2() {
    let case = &cases()[0];
    let input = json!({"source": case["source"], "invocation": case["invocation"]}).to_string();
    let (code, from_stdin, _) = run(&["read", "retained-precision"], &input);
    let path = std::env::temp_dir().join(format!("reader-cli-{}.json", std::process::id()));
    std::fs::write(&path, &input).unwrap();
    let (file_code, from_file, _) = run(
        &[
            "read",
            "retained-precision",
            "--input",
            path.to_str().unwrap(),
        ],
        "",
    );
    std::fs::remove_file(&path).unwrap();
    assert_eq!((code, &from_stdin), (file_code, &from_file));
    assert_eq!(code, 0);
    // A null invocation reads unbound, as an absent one.
    let null = json!({"source": case["source"], "invocation": null}).to_string();
    let absent = json!({"source": case["source"]}).to_string();
    assert_eq!(
        run(&["read", "retained-precision"], &null).1,
        run(&["read", "retained-precision"], &absent).1
    );
    for (args, input) in [
        (vec!["read"], absent.as_str()),
        (vec!["read", "base"], absent.as_str()),
        (
            vec!["read", "retained-precision", "--output", "x.json"],
            absent.as_str(),
        ),
        (vec!["read", "retained-precision"], "{"),
        (
            vec!["read", "retained-precision"],
            r#"{"source":{},"mode":"x"}"#,
        ),
        (vec!["read", "retained-precision-transport"], input.as_str()),
    ] {
        let (code, verdict, stderr) = run(&args, input);
        assert_eq!((code, verdict), (2, Value::Null), "{args:?}");
        assert!(!stderr.is_empty(), "{args:?}");
    }
}
