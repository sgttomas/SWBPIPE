//! `openpipestress-runner read`: the retained reader's verdict through the CLI
//! over the shared corpus's bases and the producer-solved successors. The
//! outcome is the reader's own `outcome_json` of `retained_precision::validate`
//! and `validate_transport_metadata`, run in process here for comparison, and
//! the corpus's Rust expectation.
use open_pipe_stress_result_export::retained_precision as rp;
use serde_json::{json, Value};
#[path = "../../../reporting/result_export/tests/support/retained_precision_corpus.rs"]
mod retained_precision_corpus;
use std::io::Write;
use std::sync::OnceLock;
use std::process::{Command, Stdio};

const CORPUS_FILES: [&str; 1] = [
    include_str!("../../../../fixtures/results/retained_precision_cases.json"),
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

fn cases() -> &'static [Value] {
    static CASES: OnceLock<Vec<Value>> = OnceLock::new();
    CASES.get_or_init(|| CORPUS_FILES
        .iter()
        .flat_map(|text| {
            let file = retained_precision_corpus::expand(serde_json::from_str(text).unwrap());
            file["cases"].as_array().unwrap().clone()
        })
        .collect())
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
    // W1 adds 16 bases; exercise every original and added statement through all CLI reads.
    // O4 (W1-E) adds four producer bases: the two dual-basis Timoshenko successors, the mixed
    // (one distinct-basis member) successor and the sourced-unavailable-case successor; each
    // passes bound with its stated classes and is never eligible unbound.
    // The W1 coverage repair adds the 73/24 Pa halfway control's two producer bases and the
    // D = 0.2 m represented-hull control's two.
    assert_eq!(cases.len(), 69);
    for case in cases {
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

/// R2: the base readers (`reader_verdict`) through the CLI: the dispatch entry,
/// each reader's validation and transport, and row semantics, over genuine
/// statements, as the in-process verdict.
#[test]
fn base_readers_read_through_the_cli_as_reader_verdict() {
    use open_pipe_stress_result_export::reader_verdict as rv;
    // B4/D1 (b): the composite statement is the source route here; B7b: the
    // current pressure-source-1 (0.9.0/v3) statement.
    let blocks: Value = serde_json::from_str(include_str!(
        "../../../../fixtures/product_preview/pressure_source/physics_source/n05-sparse_interactive.raw.json"
    ))
    .unwrap();
    let request: Value = serde_json::from_str(include_str!(
        "../../../../fixtures/product_preview/pressure_source/physics_source/n05.request.json"
    ))
    .unwrap();
    let invocation = json!({"request": request, "solver_mode": "sparse_interactive"});
    // pressure-2 with the load/reference-state group, and without it.
    let lr: Value = serde_json::from_str(include_str!(
        "../../../../fixtures/product_preview/pressure/load_reference/connected-sparse_interactive.raw.json"
    ))
    .unwrap();
    let lr_model = serde_json::from_str::<Value>(include_str!(
        "../../../../fixtures/product_preview/pressure/load_reference/connected.request.json"
    ))
    .unwrap()["model"]
        .clone();
    let physics: Value = serde_json::from_str(include_str!(
        "../../../../fixtures/results/pressure_connected_mechanics_sparse.json"
    ))
    .unwrap();
    let mut fewer = lr_model.clone();
    fewer["load_cases"].as_array_mut().unwrap().remove(0);
    let mut retired = physics.clone();
    retired["producer"]["semantic_contract_id"] = json!("openpipestress.result_semantics/0.3.0/physics-1");
    let reads = [
        ("dispatch", json!({"source": blocks, "model": request["model"], "invocation": invocation}),
            rv::dispatch(&blocks, Some(&request["model"]), Some(&invocation))),
        ("dispatch", json!({"source": lr, "model": lr_model}), rv::dispatch(&lr, Some(&lr_model), None)),
        ("dispatch", json!({"source": {"carrier_evidence": {}}}), rv::dispatch(&json!({"carrier_evidence": {}}), None, None)),
        ("pressure-source", json!({"source": blocks, "invocation": invocation}), rv::validate("pressure-source", &blocks, None, Some(&invocation))),
        ("pressure", json!({"source": lr, "model": fewer}), rv::validate("pressure", &lr, Some(&fewer), None)),
        ("pressure-source-transport", json!({"source": blocks}), rv::transport("pressure-source", &blocks)),
        ("pressure", json!({"source": physics}), rv::validate("pressure", &physics, None, None)),
        ("pressure-transport", json!({"source": physics}), rv::transport("pressure", &physics)),
        ("row-semantics", json!({"row": lr["results"][0], "source": lr}), rv::row_semantics(&lr["results"][0], Some(&lr))),
        ("dispatch", json!({"source": retired}), rv::dispatch(&retired, None, None)),
    ];
    let mut codes = Vec::new();
    for (reader, input, outcome) in reads {
        let (code, verdict, stderr) = run(&["read", reader], &input.to_string());
        assert!(stderr.is_empty(), "{stderr}");
        let want = json!({"artifact": "openpipestress.result_reader_verdict", "schema_version": "1.0.0",
            "reader": reader, "outcome": outcome});
        assert_eq!(verdict, want, "{reader}");
        assert_eq!(code, if verdict["outcome"]["ok"] == true { 0 } else { 1 }, "{reader}");
        codes.push(code);
    }
    // Admitted, admitted, refused; then the readers: pass, refuse (G2), pass, pass,
    // refuse (no pressure-2 transport reader), pass; then a retired v2 identity,
    // refused by name at the header.
    assert_eq!(codes, [0, 0, 1, 0, 1, 0, 0, 1, 0, 1]);
    // B7b: the retired v2 readers are no longer read commands (unusable: exit 2).
    for reader in ["physics", "physics-source", "load-reference", "load-reference-source", "physics-transport"] {
        let (code, verdict, stderr) = run(&["read", reader], &json!({"source": physics}).to_string());
        assert_eq!((code, verdict), (2, Value::Null), "{reader}");
        assert!(stderr.contains("unknown reader"), "{reader}: {stderr}");
    }
}
