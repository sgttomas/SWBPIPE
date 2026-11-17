//! DEL-04-09 / DEL-10-05 REQ-015, AC-011: the actual `openpipestress-runner modal`
//! process and `read modal-result`. The modal request goes in on stdin or
//! `--input`. The `ControlledExport`-wrapped document comes out on stdout, and
//! `--output` is written as the other verbs write it. The exit status is 0 when
//! the document is solved, 1 when it is refused or blocked, and 2 when the input
//! is unusable and no document is formed.
//!
//! The requests are the committed, invented modal request fixtures
//! (`fixtures/results/modal_*.request.json`) and variants built from them here.
//! The connected path is checked end to end: CLI request, product entry,
//! modal result, and the independent product reader with the bound request.
//! The CLI's payload equals, by canonical value, the document the owning entry
//! forms in-process from the same request. Both come from this build on this
//! platform, so no committed bytes are compared.

use open_pipe_stress_modal_result::{read_modal_result_text, ModalSupplied};
use open_pipe_stress_product_physics::modal_experimental_result_document;
use serde_json::{json, Value};
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};

fn project() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..")
}

fn request(stem: &str) -> Value {
    let path = project().join("fixtures/results").join(format!("{stem}.request.json"));
    serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap()
}

struct Run {
    code: i32,
    stdout: String,
    stderr: String,
}

fn run(args: &[&str], stdin: &[u8]) -> Run {
    let mut child = Command::new(env!("CARGO_BIN_EXE_openpipestress-runner"))
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let mut pipe = child.stdin.take().unwrap();
    let input = stdin.to_vec();
    let writer = std::thread::spawn(move || {
        let _ = pipe.write_all(&input);
    });
    let output = child.wait_with_output().unwrap();
    writer.join().unwrap();
    Run {
        code: output.status.code().unwrap(),
        stdout: String::from_utf8(output.stdout).unwrap(),
        stderr: String::from_utf8(output.stderr).unwrap(),
    }
}

fn modal(request: &Value, intent: bool) -> Run {
    let args: &[&str] = if intent { &["modal", "--explicit-local-private-intent"] } else { &["modal"] };
    run(args, request.to_string().as_bytes())
}

fn controlled(run: &Run) -> Value {
    serde_json::from_str(&run.stdout).unwrap_or_else(|e| panic!("{e}: {}", run.stdout))
}

/// The modal reader's verdict through `read modal-result`, and the same reader called
/// directly on the same bytes, which must agree.
fn read_cli(document: &Value, bound: Option<&Value>) -> (i32, Value) {
    let input = match bound {
        Some(request) => json!({"source": document, "request": request}),
        None => json!({"source": document}),
    };
    let out = run(&["read", "modal-result"], input.to_string().as_bytes());
    let output: Value = serde_json::from_str(&out.stdout).unwrap_or_else(|e| panic!("{e}: {} {}", out.stdout, out.stderr));
    let direct = read_modal_result_text(&serde_json::to_string(document).unwrap(), ModalSupplied { request: bound });
    assert_eq!(output["outcome"]["verdict"], direct.modal_verdict_json(), "the CLI verdict is not the reader's");
    (out.code, output["outcome"].clone())
}

/// The experimental stamp every document carries (M1's selected parameter standing,
/// separate from the result standing).
fn assert_experimental(document: &Value, ctx: &str) {
    let admission = &document["admission"];
    assert_eq!(admission["status"], "experimental", "{ctx}");
    assert_eq!(admission["public_engineering_result"], false, "{ctx}");
    assert_eq!(admission["parameter_standing"], "selected", "{ctx}");
    assert_eq!(admission["parameter_label"], "selected_a1_a2_a3_criteria", "{ctx}");
}

/// The runner adds no option: the document binds exactly the members the request
/// carries besides its model and materials.
fn assert_no_option_added(document: &Value, request: &Value, ctx: &str) {
    let mut bound: Vec<&String> = document["binding"]["options"].as_object().unwrap().keys().collect();
    let mut given: Vec<&String> =
        request.as_object().unwrap().keys().filter(|k| *k != "model" && *k != "materials").collect();
    bound.sort();
    given.sort();
    assert_eq!(bound, given, "{ctx}: the bound options are not the request's");
}

const SOLVED: [&str; 4] = [
    "modal_cantilever_below_cutoff",
    "modal_state_one_way_pressed",
    "modal_component_body",
    "modal_ex_n05",
];

#[test]
fn a_solved_request_is_the_entrys_document_and_the_reader_reads_it_with_the_bound_request() {
    for stem in SOLVED {
        let request = request(stem);
        let out = modal(&request, true);
        assert_eq!(out.code, 0, "{stem}: {}", out.stderr);
        let c = controlled(&out);
        assert_eq!(c["blocked"], false, "{stem}");
        let document = &c["payload"];
        assert_eq!(document, &modal_experimental_result_document(&request), "{stem}: the payload is not the entry's document");
        assert_eq!(document["status"], "solved", "{stem}");
        assert_experimental(document, stem);
        assert_no_option_added(document, &request, stem);
        // The product reader, with the bound request: not rejected, the binding passed,
        // and public admission disabled whatever the decision.
        let (code, outcome) = read_cli(document, Some(&request));
        let verdict = &outcome["verdict"];
        assert_eq!((code, &outcome["ok"]), (0, &json!(true)), "{stem}: {verdict}");
        assert_ne!(verdict["decision"], "rejected", "{stem}");
        assert_eq!(verdict["binding"]["status"], "passed", "{stem}: {verdict}");
        assert_eq!(verdict["binding"]["not_checked"], json!([]), "{stem}: {verdict}");
        assert_eq!(verdict["public_admission"]["status"], "disabled", "{stem}");
        // Without the request the reader still reads it, scoped: the document's own
        // binding relations pass, the request is reported not checked, and the
        // model-derived mass is not derived, so it is not fully validated.
        let (code, outcome) = read_cli(document, None);
        assert_eq!(code, 0, "{stem}");
        assert_eq!(outcome["verdict"]["validation"]["status"], "not_fully_validated", "{stem}");
        assert_eq!(outcome["verdict"]["binding"]["not_checked"], json!(["request"]), "{stem}");
    }
}

/// Every named refusal passes through: the runner supplies no extraction bound,
/// state case or acceleration, and the refusal is the entry's.
#[test]
fn named_refusals_pass_through_with_no_default_supplied() {
    let mut no_state = request("modal_state_one_way_pressed");
    no_state.as_object_mut().unwrap().remove("state_case");
    let mut no_acceleration = request("modal_component_body");
    no_acceleration.as_object_mut().unwrap().remove("weight_to_mass_acceleration");
    let mut conflict = request("modal_state_one_way_pressed");
    conflict["modulus_basis_ref"] = json!("load:PRESSED");
    let mut zero_count = request("modal_cantilever_below_cutoff");
    zero_count.as_object_mut().unwrap().remove("frequency_cutoff");
    zero_count["mode_count"] = json!(0);
    for (ctx, request, code) in [
        ("no bound", request("modal_refused_no_bound"), "MODAL_EXTRACTION_BOUND_REQUIRED"),
        ("no state case", no_state, "MODAL_STATE_CASE_REQUIRED"),
        ("no acceleration", no_acceleration, "MODAL_WEIGHT_ACCELERATION_REQUIRED"),
        ("modulus basis with a state case", conflict, "MODAL_MODULUS_BASIS_CONFLICT"),
        ("zero mode count", zero_count, "MODAL_EXTRACTION_BOUND_INVALID"),
        ("not a request", json!({"model": 1}), "MODAL_REQUEST_INVALID"),
    ] {
        let out = modal(&request, true);
        assert_eq!(out.code, 1, "{ctx}: {}", out.stderr);
        let document = &controlled(&out)["payload"];
        assert_eq!(document, &modal_experimental_result_document(&request), "{ctx}");
        assert_eq!(document["status"], "refused", "{ctx}");
        assert_experimental(document, ctx);
        let codes: Vec<&str> =
            document["refusals"]["rows"].as_array().unwrap().iter().map(|r| r["code"].as_str().unwrap()).collect();
        assert!(codes.contains(&code), "{ctx}: {codes:?}");
        let (read_code, outcome) = read_cli(document, Some(&request));
        assert_eq!(read_code, 0, "{ctx}: {}", outcome["verdict"]);
        assert_eq!(outcome["verdict"]["decision"], "refusal_consistent", "{ctx}");
    }
}

/// The whole document is local-private, solved or refused. Without the intent flag
/// it is blocked by one decision at `$`: exit 1, no payload, and nothing of the
/// document is output, neither its digests nor its kind. No `--output` file is
/// written.
#[test]
fn without_intent_the_document_is_blocked_and_nothing_of_it_is_output() {
    let dir = std::env::temp_dir().join(format!("modal-cli-gate-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    for stem in ["modal_cantilever_below_cutoff", "modal_refused_no_bound"] {
        let request = request(stem);
        let document = modal_experimental_result_document(&request);
        let output = dir.join(format!("{stem}.json"));
        let out = run(&["modal", "--output", output.to_str().unwrap()], request.to_string().as_bytes());
        assert_eq!(out.code, 1, "{stem}: {}", out.stderr);
        let c = controlled(&out);
        assert_eq!((c["blocked"].clone(), c["payload"].clone()), (json!(true), Value::Null), "{stem}");
        assert_eq!(c["decisions"][0]["path"], "$", "{stem}");
        assert_eq!(c["decisions"][0]["field_class"], "modal_result", "{stem}");
        assert_eq!(c["decisions"][0]["action"], "block_export", "{stem}");
        for leaked in [
            document["binding"]["request_sha256"].as_str().unwrap(),
            document["binding"]["materials_sha256"].as_str().unwrap(),
            document["run"]["run_id"].as_str().unwrap(),
            "openpipestress.modal_result",
        ] {
            assert!(!out.stdout.contains(leaked), "{stem}: blocked output carries {leaked}");
        }
        assert!(!output.exists(), "{stem}: a blocked document was written");
    }
    std::fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn with_intent_the_output_file_is_the_stdout_document() {
    let dir = std::env::temp_dir().join(format!("modal-cli-out-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let input = dir.join("request.json");
    let output = dir.join("result.json");
    std::fs::write(&input, request("modal_cantilever_below_cutoff").to_string()).unwrap();
    let out = run(
        &["modal", "--input", input.to_str().unwrap(), "--output", output.to_str().unwrap(), "--explicit-local-private-intent"],
        b"",
    );
    assert_eq!(out.code, 0, "{}", out.stderr);
    assert_eq!(std::fs::read_to_string(&output).unwrap(), format!("{}\n", out.stdout.trim_end()));
    let c = controlled(&out);
    assert_eq!(c["decisions"][0]["action"], "warning_only");
    assert_experimental(&c["payload"], "file");
    std::fs::remove_dir_all(&dir).unwrap();
}

/// No document is formed for an unusable invocation or input: exit 2, empty stdout.
#[test]
fn an_unusable_invocation_or_input_exits_two_without_a_document() {
    let text = request("modal_cantilever_below_cutoff").to_string();
    for (ctx, args, input) in [
        ("solver mode", vec!["modal", "--solver-mode", "dense_scrutiny"], text.clone().into_bytes()),
        ("not JSON", vec!["modal", "--explicit-local-private-intent"], b"{\"model\": ".to_vec()),
        ("not UTF-8", vec!["modal", "--explicit-local-private-intent"], vec![0xff, 0xfe]),
        (
            "duplicate member",
            vec!["modal", "--explicit-local-private-intent"],
            text.replacen("{", "{\"mode_count\": 1, \"mode_count\": 2, ", 1).into_bytes(),
        ),
    ] {
        let out = run(&args, &input);
        assert_eq!((out.code, out.stdout.as_str()), (2, ""), "{ctx}: {}", out.stderr);
    }
    // The reader's input is held to the checked profile too.
    let out = run(&["read", "modal-result"], br#"{"source": {"status": "solved", "status": "refused"}}"#);
    assert_eq!((out.code, out.stdout.as_str()), (2, ""), "{}", out.stderr);
}

/// A request that is not the bound one fails the reader's binding: rejected, exit 1.
#[test]
fn the_reader_rejects_a_document_read_against_another_request() {
    let request = request("modal_cantilever_below_cutoff");
    let document = modal_experimental_result_document(&request);
    let (code, outcome) = read_cli(&document, Some(&self::request("modal_ex_n05")));
    assert_eq!((code, &outcome["ok"], &outcome["verdict"]["decision"]), (1, &json!(false), &json!("rejected")));
}
