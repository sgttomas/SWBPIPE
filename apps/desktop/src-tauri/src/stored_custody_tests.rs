//! C1 slice 6: saved and recorded runs reopened into native custody
//! (`stored_custody`, `analysis_run_v03_validate`).
//!
//! - **The TypeScript contexts' findings.** Every case of
//!   `apps/desktop/src/test-support/native-reopen/historical_reopen_cases.json`
//!   (written by `historicalReopenCases.ts` from the unchanged
//!   `buildHistoricalRunContext` and `buildRecordedRunContext` over full
//!   rows; `historicalReopenParity.test.ts` regenerates and compares it) must
//!   give the same findings, in order, the same run name, and the same
//!   envelope payload hash or model hash, natively from the stored values.
//! - **Recorded table bases, verbatim.** main's recorded pressure-2 runs, one
//!   per row of `RECORDED_RUN_TABLE_BASES` (`RECORDED_BASIS_FIXTURES`), are
//!   cases of that file read from the fixture files themselves; the native
//!   table must equal its TypeScript and Python twins, and each run of a
//!   current model document saved into a store must reopen natively with the
//!   webview's findings.
use super::*;
use crate::analysis_run_v03_validate::{projection_checked_sha256, recorded_run_findings, recorded_run_validation, saved_run_findings, saved_run_validation, RECORDED_RUN_TABLE_BASES};
use serde_json::{json, Value};
pub(crate) use crate::native_verdict::tests::gate as command_gate;

const REOPEN_CASES: &str = "apps/desktop/src/test-support/native-reopen/historical_reopen_cases.json";
const BUILDER_CASES: &str = "apps/desktop/src/test-support/analysis-run-v03/analysis_run_v03_cases.json";
/// `RECORDED_BASIS_FIXTURES` of `historicalReopenCases.ts`: main's verbatim recorded pressure-2
/// runs (a record, its source and its request model), one per row of
/// `RECORDED_RUN_TABLE_BASES` in its order. Read, never rewritten.
const RECORDED_BASIS_FIXTURES: [&str; 6] = [
    "fixtures/persistence/recorded_model_0_7_0",
    "fixtures/persistence/recorded_model_0_8_0",
    "fixtures/persistence/recorded_model_0_9_0_pre_2a",
    "fixtures/persistence/recorded_model_0_9_0_pre_2b",
    "fixtures/persistence/recorded_model_0_9_0_pre_ce",
    "fixtures/persistence/recorded_model_0_9_0_pre_rt_a4",
];
const FIXTURE_RECORD: &str = "pressure_plain_connected_sparse.analysis_run.json";
const FIXTURE_SOURCE: &str = "pressure_connected_mechanics_sparse.json";

/// Hold more actual command futures than the async runtime has workers at
/// the first proportional operation. Another async task must still run.
#[test]
fn reopen_admission_and_store_reads_run_off_the_async_executor() {
    use crate::command_ingress::test_support::headers;
    use crate::command_ingress::INGRESS_ARGS_JSON_V1;
    use crate::memory_pool::test_host::ScriptedHost;
    use std::time::Duration;
    let host = ScriptedHost::normal();
    let (registry, _) = crate::native_verdict::tests::gated(&host);
    let commands = crate::native_verdict::tests::runtime_workers() * 2 + 2;
    for command in [OPEN_COMMAND, RUN_COMMAND] {
        let held = command_gate::hold(&registry, command, false);
        let tasks: Vec<_> = (0..commands).map(|_| {
            let registry = registry.clone();
            tauri::async_runtime::spawn(async move {
                let args = if command == OPEN_COMMAND { json!({"projectId":"invented:project"}) }
                    else { json!({"projectId":"invented:project","recordingId":"invented:record"}) };
                let body = InvokeBody::Raw(serde_json::to_vec(&args).unwrap());
                let header = headers(Some(INGRESS_ARGS_JSON_V1));
                let store = || Err("TEST-REOPEN-STORE-UNAVAILABLE".to_string());
                if command == OPEN_COMMAND { open_project(&registry, store, &body, &header).await }
                    else { read_run(&registry, store, &body, &header).await }
            })
        }).collect();
        let all_entered = held.0.wait_entered(commands, Duration::from_secs(30));
        let (sender, receiver) = std::sync::mpsc::channel();
        tauri::async_runtime::spawn(async move { let _ = sender.send(()); });
        let probe = receiver.recv_timeout(Duration::from_secs(5));
        held.0.open();
        for task in tasks {
            let outcome = tauri::async_runtime::block_on(task).unwrap();
            assert!(matches!(outcome, Err(CommandError::Command(ref text)) if text == "TEST-REOPEN-STORE-UNAVAILABLE"));
        }
        assert!(all_entered, "{command}: admission occupied the async executor");
        assert!(probe.is_ok(), "{command}: no other async task could run");
        drop(held);
    }
    assert_eq!(registry.pool().reserved_total(), 0);
}

pub(crate) fn root() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..")
}
pub(crate) fn read_text(path: &str) -> String {
    std::fs::read_to_string(root().join(path)).unwrap_or_else(|e| panic!("{path}: {e}"))
}

/// A builder case's held text: a file, a file's top-level member, or a text.
pub(crate) fn source_text(source: &Value) -> String {
    if let Some(text) = source["text"].as_str() {
        return text.to_string();
    }
    let text = read_text(source["path"].as_str().unwrap());
    match source["member"].as_str() {
        None => text,
        Some(member) => crate::result_detail::last_members(&text, [member]).unwrap()[0].unwrap().get().to_string(),
    }
}

/// The builder corpus's cases by id.
pub(crate) fn builder_cases() -> std::collections::HashMap<String, Value> {
    let file: Value = serde_json::from_str(&read_text(BUILDER_CASES)).unwrap();
    file["cases"].as_array().unwrap().iter().map(|case| (case["id"].as_str().unwrap().to_string(), case.clone())).collect()
}

pub(crate) fn reopen_cases() -> Vec<Value> {
    serde_json::from_str::<Value>(&read_text(REOPEN_CASES)).unwrap()["cases"].as_array().unwrap().clone()
}

/// `applyPatches` of the generator.
fn apply_patches(result: &mut Value, record: &mut Value, patches: &[Value]) {
    for patch in patches {
        let target = match patch["on"].as_str().unwrap() {
            "result" => &mut *result,
            "record" => &mut *record,
            other => panic!("patch on {other}"),
        };
        let path = patch["path"].as_array().unwrap();
        let value = patch["raw_json"].as_str().map(|text| serde_json::from_str(text).unwrap())
            .unwrap_or_else(|| patch.get("value").cloned().unwrap_or(Value::Null));
        if path.is_empty() {
            *target = value;
            continue;
        }
        let mut at = target;
        for key in &path[..path.len() - 1] {
            at = match key {
                Value::String(key) => at.get_mut(key.as_str()).unwrap(),
                Value::Number(index) => at.get_mut(index.as_u64().unwrap() as usize).unwrap(),
                _ => unreachable!(),
            };
        }
        let last = &path[path.len() - 1];
        match (patch["op"].as_str().unwrap(), last) {
            ("set", Value::String(key)) => {
                at[key.as_str()] = value;
            }
            ("set", Value::Number(index)) => at[index.as_u64().unwrap() as usize] = value,
            ("delete", Value::String(key)) => {
                at.as_object_mut().unwrap().remove(key.as_str());
            }
            ("append", Value::String(key)) => at[key.as_str()].as_array_mut().unwrap().push(value),
            other => panic!("patch {other:?}"),
        }
    }
}

/// A record resealed as the generator reseals it.
fn reseal(record: &mut Value) {
    let value = projection_checked_sha256(record).unwrap();
    for hash in record["analysis_run"]["hashes"].as_array_mut().unwrap() {
        if hash["payload_scope"] == "analysis_run_record" {
            hash["value"] = json!(value);
        }
    }
}

/// A recorded-basis fixture's file, named by a case's `fixture`.
/// A case's `FileRef`: a committed fixture file's value at a JSON pointer (read, never rewritten).
fn file_value(reference: &Value) -> Value {
    let path = reference["path"].as_str().unwrap();
    assert!(path.starts_with("fixtures/persistence/recorded_pre_rt_a4_pf/"), "{path}: no pre-RT-A4 fixture");
    let value: Value = serde_json::from_str(&read_text(path)).unwrap();
    value.pointer(reference["pointer"].as_str().unwrap()).unwrap_or_else(|| panic!("{reference}")).clone()
}

fn fixture_text(fixture: &Value, name: &str) -> String {
    let dir = RECORDED_BASIS_FIXTURES.iter().find(|dir| fixture.as_str() == Some(**dir)).unwrap_or_else(|| panic!("{fixture} is no recorded-basis fixture"));
    read_text(&format!("{dir}/{name}"))
}

/// A saved case's stored result and record (the values the store parses from what it stored).
pub(crate) fn saved_run(case: &Value, builder: &std::collections::HashMap<String, Value>) -> (Value, Value) {
    let (mut result, mut record) = match (case["base"].as_str(), case.get("fixture")) {
        (Some(base), _) => {
            let base = &builder[base];
            (serde_json::from_str::<Value>(&source_text(&base["source"])).unwrap(), base["expected"]["record"].clone())
        }
        (None, Some(fixture)) => (
            serde_json::from_str::<Value>(&fixture_text(fixture, FIXTURE_SOURCE)).unwrap(),
            serde_json::from_str::<Value>(&fixture_text(fixture, FIXTURE_RECORD)).unwrap(),
        ),
        (None, None) => match case.get("files") {
            Some(files) => (file_value(&files["result"]), file_value(&files["record"])),
            None => (case["mechanics_result"].clone(), case["analysis_run"].clone()),
        },
    };
    apply_patches(&mut result, &mut record, case.get("patches").and_then(Value::as_array).map(Vec::as_slice).unwrap_or(&[]));
    if case["reseal"] == true {
        reseal(&mut record);
    }
    (result, record)
}

/// A recorded case's stored result text and record: embedded, or a recorded-basis fixture's
/// source text and its record with `record_patches` applied.
fn recorded_run(read: &Value) -> (String, Value) {
    if let Some(files) = read.get("files") {
        // A whole result is the file's exact text, as the store returns its stored text.
        assert_eq!(files["result"]["pointer"], "", "a recorded case's result is a whole file");
        return (read_text(files["result"]["path"].as_str().unwrap()), file_value(&files["record"]));
    }
    match read.get("fixture") {
        None => (read["result_json"].as_str().unwrap().to_string(), read["record"].clone()),
        Some(fixture) => {
            let mut record: Value = serde_json::from_str(&fixture_text(fixture, FIXTURE_RECORD)).unwrap();
            apply_patches(&mut Value::Null, &mut record, read["record_patches"].as_array().unwrap());
            (fixture_text(fixture, FIXTURE_SOURCE), record)
        }
    }
}

/// A raw project preserves stored integer digits that JavaScript's oracle rounds.
pub(crate) fn saved_project(case: &Value) -> Value {
    case["project_text"].as_str().map(|text| serde_json::from_str(text).unwrap())
        .unwrap_or_else(|| case["project"].clone())
}

#[test]
fn the_native_findings_are_the_typescript_contexts() {
    let builder = builder_cases();
    let cases = reopen_cases();
    assert!(cases.len() >= 150, "{} cases", cases.len());
    let mut failures = Vec::new();
    for case in &cases {
        let id = case["id"].as_str().unwrap();
        let expected = &case["expected"];
        let actual = native_findings(case, &builder);
        if &actual != expected {
            failures.push(format!("{id}:\n  expected {expected}\n  actual   {actual}"));
        }
    }
    assert!(failures.is_empty(), "{} of {} cases differ:\n{}", failures.len(), cases.len(), failures.join("\n"));
}

/// RT-A4 §10.4: main's runs recorded before RT-A4 (`recorded_pre_rt_a4_pf`) reopen natively as the
/// webview reopens them: read under their recorded table, their standing findings the historical
/// reader's (kept apart, labelled by that table), and none of a current-only reader's refusals among
/// the findings. Without the native recorded read they read under the current tables and are refused
/// (`SOURCE_PREVIEW_PHYSICS_FORMULATION_BASIS`, `ANALYSIS_SOURCE_CONTRACT_VERSION_MISMATCH`, …), so
/// this fails. An unknown digest names no variant and reads as before, under the current tables.
#[test]
fn runs_recorded_before_rt_a4_reopen_natively_under_their_recorded_table() {
    const CURRENT_ONLY: [&str; 5] = ["SOURCE_SEMANTIC_CONTRACT_UNSUPPORTED", "SOURCE_PREVIEW_PHYSICS_FORMULATION_BASIS", "ANALYSIS_SOURCE_CONTRACT_VERSION_MISMATCH", "ANALYSIS_SEMANTIC_CONTRACT_MISMATCH", "SOURCE_RECORDED_BINDING"];
    let builder = builder_cases();
    let cases: Vec<Value> = reopen_cases().into_iter().filter(|case| case["id"].as_str().unwrap().contains("pre-rt-a4:")).collect();
    assert_eq!(cases.len(), 7, "five saved variants, one unknown digest, one recorded revision");
    for case in &cases {
        let id = case["id"].as_str().unwrap();
        let actual = native_findings(case, &builder);
        assert_eq!(&actual, &case["expected"], "{id}: the webview's findings, in their scopes");
        if id.ends_with(":unknown-digest") {
            assert_eq!(actual["recorded_table_read"], Value::Null, "{id}");
            assert!(actual["historical_reader_findings"].as_array().unwrap().is_empty(), "{id}");
            continue;
        }
        let table = &actual["recorded_table_read"];
        assert!(open_pipe_stress_result_export::recorded_tables::variant_of(table).is_some(), "{id}: read under a registry variant: {table}");
        let shown: Vec<&str> = actual["findings"].as_array().unwrap().iter().chain(actual["historical_reader_findings"].as_array().unwrap()).map(|f| f.as_str().unwrap()).collect();
        for marker in CURRENT_ONLY {
            assert!(!shown.iter().any(|finding| finding.contains(marker)), "{id}: {marker} in {shown:?}");
        }
    }
}

/// A case's findings, natively from its stored values, in the form of its `expected`.
fn native_findings(case: &Value, builder: &std::collections::HashMap<String, Value>) -> Value {
    match case["kind"].as_str().unwrap() {
        "saved" => {
            let (mut result, record) = saved_run(case, builder);
            let project = saved_project(case);
            let saved = SavedProject {
                model: &project["model"],
                editor_intents: &project["editor_intents"],
                proposal: &project["proposal"],
                selected_review_target: &project["selected_review_target"],
                model_hash: &project["model_hash"],
                project_envelope_hash: &project["project_envelope_hash"],
            };
            match saved_run_findings(&saved, &mut result, &record) {
                None => json!({"absent": true}),
                Some(mut found) => {
                    saved_run_validation(&mut found, &mut result, &record);
                    json!({"findings": found.findings, "check_findings": found.check_findings, "recorded_table_read": found.recorded_table_read, "historical_reader_findings": found.historical_reader_findings,
                        "run_id": found.run_id, "envelope_payload_hash": found.envelope_payload_hash, "model_hash": null})
                }
            }
        }
        _ => {
            let read = &case["read"];
            let context = RecordedRead {
                state_id: read["state"]["state_id"].as_str().unwrap(),
                state_project_id: read["state"]["project_id"].as_str().unwrap(),
                state_model_hash: read["state"]["model_hash"].as_str().unwrap(),
                run_state_id: read["run"]["state_id"].as_str().unwrap(),
                producer_run_id: read["run"]["producer_run_id"].as_str().unwrap(),
                run_record_hash: read["run"]["run_record_hash"].as_str().unwrap(),
            };
            let (result_json, record) = recorded_run(read);
            let mut result = serde_json::from_str::<Value>(&result_json).ok();
            let manifest = serde_json::from_str::<Value>(read["input_manifest_canonical"].as_str().unwrap()).ok();
            let mut found = recorded_run_findings(&context, result.as_mut(), manifest.as_ref(), &record);
            if let Some(result) = result.as_mut() {
                recorded_run_validation(&mut found, &context, result, manifest.as_ref(), &record);
            }
            json!({"findings": found.findings, "check_findings": found.check_findings, "recorded_table_read": found.recorded_table_read, "historical_reader_findings": found.historical_reader_findings,
                "run_id": found.run_id, "envelope_payload_hash": null, "model_hash": found.model_hash})
        }
    }
}

/// A token of a table literal: one of `()[]{},:`, a quoted string (either quote; no escapes or
/// line breaks inside), or a name.
#[derive(Debug, Clone, PartialEq)]
enum Token {
    Punct(char),
    Str(String),
    Name(String),
}

/// The tokens of the bracketed literal that opens at `text`'s first non-space character, through
/// its matching close, and the text after it. Whitespace and line breaks separate tokens; any
/// other character (a comment, an escape, an operator) fails, so nothing in the literal is
/// skipped unread.
fn bracketed_tokens<'a>(text: &'a str, what: &str) -> (Vec<Token>, &'a str) {
    let mut tokens = Vec::new();
    let mut depth = 0usize;
    let mut chars = text.char_indices().peekable();
    while let Some((at, c)) = chars.next() {
        match c {
            c if c.is_whitespace() => continue,
            '(' | '[' | '{' => {
                depth += 1;
                tokens.push(Token::Punct(c));
            }
            ')' | ']' | '}' => {
                assert!(depth > 0, "{what}: unbalanced {c}");
                depth -= 1;
                tokens.push(Token::Punct(c));
                if depth == 0 {
                    return (tokens, &text[at + 1..]);
                }
            }
            ',' | ':' | '/' => tokens.push(Token::Punct(c)),
            '"' | '\'' => {
                let mut value = String::new();
                loop {
                    match chars.next() {
                        Some((_, end)) if end == c => break,
                        Some((_, '\\' | '\n')) | None => panic!("{what}: unsupported or unterminated string"),
                        Some((_, other)) => value.push(other),
                    }
                }
                tokens.push(Token::Str(value));
            }
            c if c.is_ascii_alphabetic() || c == '_' => {
                let mut name = String::from(c);
                while let Some(&(_, next)) = chars.peek() {
                    if !(next.is_ascii_alphanumeric() || next == '_') {
                        break;
                    }
                    name.push(next);
                    chars.next();
                }
                tokens.push(Token::Name(name));
            }
            other => panic!("{what}: unrecognized text {other:?} in the table"),
        }
        assert!(depth > 0, "{what}: the table does not open with a bracket");
    }
    panic!("{what}: the table does not close")
}

/// The comma-separated entries of a bracketed list's tokens (`open … close`, an optional
/// trailing comma), each entry's own tokens with its brackets.
fn entries(tokens: &[Token], open: char, close: char, what: &str) -> Vec<Vec<Token>> {
    assert!(tokens.first() == Some(&Token::Punct(open)) && tokens.last() == Some(&Token::Punct(close)), "{what}: not a {open}…{close} list: {tokens:?}");
    let (mut out, mut depth, mut current) = (Vec::new(), 0usize, Vec::new());
    for token in &tokens[1..tokens.len() - 1] {
        match token {
            Token::Punct(',') if depth == 0 => {
                assert!(!current.is_empty(), "{what}: an empty entry");
                out.push(std::mem::take(&mut current));
                continue;
            }
            Token::Punct('(' | '[' | '{') => depth += 1,
            Token::Punct(')' | ']' | '}') => depth -= 1,
            _ => {}
        }
        current.push(token.clone());
    }
    if !current.is_empty() {
        out.push(current);
    }
    out
}

/// The native `RECORDED_RUN_TABLE_BASES` (`analysis_run_v03_validate.rs`) is the record
/// validator's table in both its twins, row for row and in order: `analysisRunCompatibility.ts`
/// and `compatibility.py`. Each twin's whole table literal is read and every entry must be a
/// recognized row: an entry or text that is not one fails the test rather than being skipped.
/// Whitespace, line breaks, either quote style and a trailing comma are tolerated.
#[test]
fn the_recorded_table_bases_are_the_typescript_and_python_tables() {
    let native: Vec<[String; 3]> = RECORDED_RUN_TABLE_BASES.iter().map(|(id, recorded, current)| [id.to_string(), recorded.to_string(), current.to_string()]).collect();
    // TypeScript: `Object.freeze([ { id: …, recorded: …, current: … }, … ]);`.
    let ts = read_text("apps/desktop/src/services/analysisRunCompatibility.ts");
    assert_eq!(ts.matches("const RECORDED_RUN_TABLE_BASES").count(), 1);
    let declared = &ts[ts.find("const RECORDED_RUN_TABLE_BASES").unwrap()..];
    let literal = &declared[declared.find("Object.freeze(").expect("the frozen TypeScript table") + "Object.freeze(".len()..];
    let (tokens, rest) = bracketed_tokens(literal, "TypeScript");
    assert!(rest.trim_start().starts_with(");"), "TypeScript: text after the table: {:?}", &rest[..rest.len().min(40)]);
    let typescript: Vec<[String; 3]> = entries(&tokens, '[', ']', "TypeScript").iter().map(|row| {
        let fields = entries(row, '{', '}', "TypeScript row");
        let mut values = std::collections::BTreeMap::new();
        for field in &fields {
            match field.as_slice() {
                [Token::Name(key), Token::Punct(':'), Token::Str(value)] if ["id", "recorded", "current"].contains(&key.as_str()) => {
                    assert!(values.insert(key.clone(), value.clone()).is_none(), "TypeScript: {key} twice in {fields:?}");
                }
                other => panic!("TypeScript: unrecognized field {other:?}"),
            }
        }
        assert_eq!(values.len(), 3, "TypeScript: {fields:?}");
        [values["id"].clone(), values["recorded"].clone(), values["current"].clone()]
    }).collect();
    // Python: `RECORDED_RUN_TABLE_BASES = ( (item, item, item), … )`; a name is a module-level string constant.
    let py = read_text("core/analysis_runs/compatibility.py");
    let head = "\nRECORDED_RUN_TABLE_BASES = ";
    assert_eq!(py.matches(head).count(), 1);
    let (tokens, _) = bracketed_tokens(&py[py.find(head).unwrap() + head.len()..], "Python");
    let constant = |name: &str| -> String {
        let assigned: Vec<&str> = py.lines().filter_map(|line| line.strip_prefix(&format!("{name} = "))).collect();
        assert_eq!(assigned.len(), 1, "Python: {name} is not assigned once at module level");
        let wrapped = format!("({})", assigned[0]);
        let (value, rest) = bracketed_tokens(&wrapped, name);
        assert!(rest.trim().is_empty(), "Python: {name}: {rest:?}");
        match value.as_slice() {
            [Token::Punct('('), Token::Str(text), Token::Punct(')')] => text.clone(),
            other => panic!("Python: {name} is not one string constant: {other:?}"),
        }
    };
    let python: Vec<[String; 3]> = entries(&tokens, '(', ')', "Python").iter().map(|row| {
        let items: Vec<String> = entries(row, '(', ')', "Python row").iter().map(|item| match item.as_slice() {
            [Token::Str(text)] => text.clone(),
            [Token::Name(name)] => constant(name),
            other => panic!("Python: unrecognized item {other:?}"),
        }).collect();
        <[String; 3]>::try_from(items).unwrap_or_else(|items| panic!("Python: a row of {} items: {items:?}", items.len()))
    }).collect();
    assert_eq!(native.len(), 6, "{native:?}");
    assert_eq!(typescript, native, "the TypeScript table");
    assert_eq!(python, native, "the Python table");
}

/// RT-A4 §10.4: the registry the native reopen reads under (`recorded_tables::VARIANTS`, the
/// same Rust the wasm engine and runner read) is the webview's (`recordedPreRtA4.ts`
/// `RECORDED_PRE_RT_A4_TABLES`) and Python's (`compatibility.py` `RECORDED_PRE_RT_A4_TABLES`),
/// variant for variant and in order, each twin's table file the same verbatim bytes, hashing to
/// the variant's digest. Each twin's whole literal is read; an entry that is not a recognized
/// variant fails the test rather than being skipped.
#[test]
fn the_recorded_pre_rt_a4_registry_is_the_typescript_and_python_registry() {
    use sha2::{Digest, Sha256};
    let native: Vec<[String; 2]> = open_pipe_stress_result_export::recorded_tables::VARIANTS.iter().map(|v| [v.id.to_string(), v.sha256.to_string()]).collect();
    // TypeScript: `Object.freeze([ { id: NAME, sha256: "…", text: importName }, … ].map(…));`.
    let ts = read_text("apps/desktop/src/services/recordedPreRtA4.ts");
    let ts_constant = |name: &str| -> String {
        let assigned: Vec<&str> = ts.lines().filter_map(|line| line.strip_prefix(&format!("const {name} = "))).collect();
        assert_eq!(assigned.len(), 1, "TypeScript: {name} is not one module constant");
        assigned[0].trim_end_matches(';').trim_matches('"').to_string()
    };
    let ts_import = |name: &str| -> String {
        let imported: Vec<&str> = ts.lines().filter_map(|line| line.strip_prefix(&format!("import {name} from \""))).collect();
        assert_eq!(imported.len(), 1, "TypeScript: {name} is not imported once");
        let path = imported[0].strip_suffix("?raw\";").unwrap_or_else(|| panic!("TypeScript: {name} is not a raw import"));
        path.rsplit('/').next().unwrap().to_string()
    };
    assert_eq!(ts.matches("export const RECORDED_PRE_RT_A4_TABLES").count(), 1);
    let declared = &ts[ts.find("export const RECORDED_PRE_RT_A4_TABLES").unwrap()..];
    let literal = &declared[declared.find("Object.freeze(").unwrap() + "Object.freeze(".len()..];
    let (tokens, rest) = bracketed_tokens(literal, "TypeScript");
    assert!(rest.starts_with(".map(variant => Object.freeze(variant)));"), "TypeScript: text after the registry: {:?}", &rest[..rest.len().min(60)]);
    let typescript: Vec<[String; 3]> = entries(&tokens, '[', ']', "TypeScript").iter().map(|row| {
        let mut values = std::collections::BTreeMap::new();
        for field in entries(row, '{', '}', "TypeScript variant") {
            let (key, value) = match field.as_slice() {
                [Token::Name(key), Token::Punct(':'), Token::Name(name)] if key == "id" => (key.clone(), ts_constant(name)),
                [Token::Name(key), Token::Punct(':'), Token::Str(value)] if key == "sha256" => (key.clone(), value.clone()),
                [Token::Name(key), Token::Punct(':'), Token::Name(name)] if key == "text" => (key.clone(), ts_import(name)),
                other => panic!("TypeScript: unrecognized field {other:?}"),
            };
            assert!(values.insert(key.clone(), value).is_none(), "TypeScript: {key} twice");
        }
        assert_eq!(values.len(), 3, "TypeScript: {values:?}");
        [values["id"].clone(), values["sha256"].clone(), values["text"].clone()]
    }).collect();
    // Python: `{ (NAME, "…"): _RECORDED_DIR / "file", … }`.
    let py = read_text("core/analysis_runs/compatibility.py");
    let head = "\nRECORDED_PRE_RT_A4_TABLES = ";
    assert_eq!(py.matches(head).count(), 1);
    let (tokens, _) = bracketed_tokens(&py[py.find(head).unwrap() + head.len()..], "Python");
    let py_constant = |name: &str| -> String {
        let assigned: Vec<&str> = py.lines().filter_map(|line| line.strip_prefix(&format!("{name} = "))).collect();
        assert_eq!(assigned.len(), 1, "Python: {name} is not assigned once at module level");
        assigned[0].trim_matches('"').to_string()
    };
    let python: Vec<[String; 3]> = entries(&tokens, '{', '}', "Python").iter().map(|entry| match entry.as_slice() {
        [Token::Punct('('), Token::Name(id), Token::Punct(','), Token::Str(sha), Token::Punct(')'), Token::Punct(':'), Token::Name(dir), Token::Punct('/'), Token::Str(file)] if dir == "_RECORDED_DIR" => {
            [py_constant(id), sha.clone(), file.clone()]
        }
        other => panic!("Python: unrecognized entry {other:?}"),
    }).collect();
    assert_eq!(native.len(), 5, "{native:?}");
    assert_eq!(typescript.iter().map(|[id, sha, _]| [id.clone(), sha.clone()]).collect::<Vec<_>>(), native, "the TypeScript registry");
    assert_eq!(python.iter().map(|[id, sha, _]| [id.clone(), sha.clone()]).collect::<Vec<_>>(), native, "the Python registry");
    for ([_, sha, ts_file], [_, _, py_file]) in typescript.iter().zip(&python) {
        assert_eq!(ts_file, py_file, "both twins read one file per variant");
        let bytes = std::fs::read(root().join("fixtures/results/recorded_pre_rt_a4").join(ts_file)).unwrap();
        assert_eq!(&format!("{:x}", Sha256::digest(&bytes)), sha, "{ts_file}: the variant's verbatim bytes");
    }
}

/// RT-A3 #99: main's verbatim pressure-2 runs recorded under each recorded table basis
/// (`RECORDED_BASIS_FIXTURES`: the 0.7.0- and 0.8.0-era tables, the 0.9.0 registration's
/// `d46e1963…` and reducer 2a's `032e861f…`) reopen natively with the webview's findings and
/// none of them is refused as `ANALYSIS_SEMANTIC_CONTRACT_MISMATCH`: each as a recorded revision
/// read by the native reader's findings, and, when its model document is current, as a saved
/// project's run, which the native open restores from a real store with those findings. A
/// project of a retired model document (0.7.0, 0.8.0) is refused for editing by the native open
/// exactly as by the legacy open (recorded runs keep their own format, saved projects do not).
#[test]
fn recorded_basis_fixtures_reopen_natively_with_the_webviews_findings() {
    let host = ScriptedHost::normal();
    let (registry, _gate) = gated(&host);
    let dir = scratch("reopen-recorded-basis");
    let cases = reopen_cases();
    let builder = std::collections::HashMap::new();
    for (index, fixture) in RECORDED_BASIS_FIXTURES.iter().enumerate() {
        let name = fixture.rsplit('/').next().unwrap();
        let verbatim: Value = serde_json::from_str(&read_text(&format!("{fixture}/{FIXTURE_RECORD}"))).unwrap();
        let case = |id: String| cases.iter().find(|case| case["id"] == json!(id));
        let recorded = case(format!("recorded-basis:{name}")).unwrap_or_else(|| panic!("no recorded case for {fixture}"));
        assert_eq!(recorded["read"]["fixture"], json!(fixture));
        let saved = case(format!("saved-recorded-basis:{name}"));
        // The run, verbatim, stored with its request's model and reopened natively.
        let request: Value = serde_json::from_str(&read_text(&format!("{fixture}/exact_pressure_connected_request.json"))).unwrap();
        let (project, (result, record)) = match saved {
            Some(saved) => {
                assert_eq!(saved["fixture"], json!(fixture));
                (saved_project(saved), saved_run(saved, &builder))
            }
            None => {
                let source: Value = serde_json::from_str(&read_text(&format!("{fixture}/{FIXTURE_SOURCE}"))).unwrap();
                (json!({"model": request["model"], "editor_intents": [], "proposal": null, "selected_review_target": null, "model_hash": null, "project_envelope_hash": null}), (source, verbatim.clone()))
            }
        };
        // The case's model went through JavaScript (`1.0` written `1`): the same data as the request's.
        assert!(crate::native_persistence::same_js_data(&project["model"], &request["model"]), "{fixture}: the request's model");
        assert_eq!(record, verbatim);
        let path = dir.join(format!("{name}.sqlite"));
        let project_id = crate::project_id_from_model(&project["model"]);
        {
            let (mut connection, _) = crate::open_project_store(&path).unwrap();
            crate::upsert_project(
                &mut connection,
                &project_id,
                &crate::project_name_from_model(&project["model"]),
                &project["model"],
                &project["editor_intents"],
                &project["proposal"],
                &project["selected_review_target"],
                &result,
                &record,
                &project["model_hash"],
                &project["project_envelope_hash"],
            )
            .unwrap();
        }
        match (native_open(&registry, &path, Some(&project_id)), saved) {
            (Ok(opened), Some(saved)) => {
                let run = &opened["stored_run"];
                assert_eq!(run["state"], "restored", "{fixture}: {run}");
                assert_eq!(run["findings"]["findings"], saved["expected"]["findings"], "{fixture}: the native open's findings are the webview's");
                assert!(registry.dispose_result(run["handle"].as_str().unwrap()).unwrap().released);
            }
            // A project of a retired model document: refused for editing, as the legacy open refuses it.
            (Err(native), None) => {
                assert!(native.contains("MODEL_SCHEMA_VERSION_RETIRED"), "{fixture}: {native}");
                let legacy = crate::open_project_in_store(path.clone(), Some(&project_id)).err();
                assert_eq!(Some(native), legacy, "{fixture}: refused as the legacy open refuses");
            }
            (Ok(opened), None) => panic!("{fixture}: a project with no saved case opened: {}", opened["stored_run"]),
            (Err(native), Some(_)) => panic!("{fixture}: {native}"),
        }
        // Both forms' findings, natively from the stored values, are the webview's, and none refuses the basis.
        for case in saved.into_iter().chain([recorded]) {
            assert_eq!(native_findings(case, &builder), case["expected"], "{}", case["id"]);
            let findings = case["expected"]["findings"].as_array().unwrap();
            assert!(!findings.iter().any(|finding| finding.as_str().unwrap().contains("MISMATCH")), "{}: {findings:?}", case["id"]);
        }
        // Each fixture binds its own row's recorded basis, in the table's order.
        let basis = RECORDED_RUN_TABLE_BASES.get(index).map(|(id, recorded, _)| json!({"id": id, "sha256": recorded}));
        assert_eq!(Some(&verbatim["analysis_run"]["reproducibility"]["semantic_contract"]), basis.as_ref(), "{fixture} binds row {index}'s recorded basis");
    }
    std::fs::remove_dir_all(&dir).unwrap();
    assert_eq!(registry.pool().reserved_total(), 0);
}

// ---------------------------------------------------------------------------
// The connecting path: native Save / Record Run -> SQLite -> native reopen
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// RT-A4 §10.4: a run recorded before RT-A4, through the native commands
// ---------------------------------------------------------------------------

const PRE_RT_A4_REPLIES: &str = "apps/desktop/src/test-support/native-reopen/native_reopen_pre_rt_a4_replies.json";
const PRE_RT_A4_REQUESTS: &str = "apps/desktop/src/test-support/native-reopen/native_reopen_pre_rt_a4_requests.json";
const PRE_RT_A4_CASE: &str = "saved-pre-rt-a4:preview-physics-2";

/// The connecting path the desktop uses for a run recorded before RT-A4: main's preview-physics-2
/// project (`recorded_pre_rt_a4_pf`, verbatim result and record, its model) stored in a real
/// store, opened by the native open, read through the native result commands on the restored
/// handle (each request the webview's historical panel sends, `native_reopen_pre_rt_a4_requests.json`,
/// written by `nativeReopenPreRtA4.session.test.tsx` with NATIVE_REOPEN_RECORD=1), and re-saved
/// from the handle. The handle holds the record's recorded table (host-established), reads S and
/// row semantics under it, and stays historical: the bare verdict is still the current reader's
/// refusal, a solve holding the same bytes reads under the current tables only, and re-save keeps
/// the stored texts. NATIVE_REOPEN_REPLIES_WRITE=1 rewrites `native_reopen_pre_rt_a4_replies.json`.
#[test]
fn a_run_recorded_before_rt_a4_reopens_through_the_native_commands() {
    use crate::native_persistence::tests::{repo_text, stringify_text};
    let host = ScriptedHost::normal();
    let (registry, gate) = gated(&host);
    let dir = scratch("reopen-pre-rt-a4");
    let path = dir.join("store.sqlite");
    let builder = builder_cases();
    let case = reopen_cases().into_iter().find(|case| case["id"] == PRE_RT_A4_CASE).expect("the corpus case");
    let (result, record) = saved_run(&case, &builder);
    // As the app's store holds a saved project: the values as JavaScript writes them.
    let js = |value: &Value| -> Value { serde_json::from_str(&stringify_text(&serde_json::to_string(value).unwrap())).unwrap() };
    let (result, record) = (js(&result), js(&record));
    let project = saved_project(&case);
    let project_id = crate::project_id_from_model(&project["model"]);
    {
        let (mut connection, _) = crate::open_project_store(&path).unwrap();
        crate::upsert_project(&mut connection, &project_id, &crate::project_name_from_model(&project["model"]), &project["model"], &project["editor_intents"],
            &project["proposal"], &project["selected_review_target"], &result, &record, &project["model_hash"], &project["project_envelope_hash"]).unwrap();
    }
    let stored = (column_text(&path, "SELECT mechanics_result_json FROM local_projects"), column_text(&path, "SELECT analysis_run_json FROM local_projects"));
    let variant = json!({"id": "openpipestress.result_semantics/0.3.0/preview-physics-2", "sha256": "1ccc9561cb4ce59d047f3dab7927a927c44a707efb3acb90e7b07a21b3bad0a2"});

    // The native open: restored, read under the record's table, with the webview's findings.
    let opened = native_open(&registry, &path, Some(&project_id)).unwrap();
    let run = &opened["stored_run"];
    assert_eq!(run["state"], "restored", "{run}");
    let handle = run["handle"].as_str().unwrap().to_string();
    assert_eq!(run["findings"]["recorded_table_read"], variant);
    for key in ["findings", "check_findings", "recorded_table_read", "historical_reader_findings", "run_id"] {
        assert_eq!(run["findings"][key], case["expected"][key], "{key}: the webview's reopen");
    }
    assert_eq!(run["recorded_dispatch"]["ok"], true, "{}", run["recorded_dispatch"]);
    assert_eq!(run["recorded_dispatch"]["route"], "preview_physics");
    assert_eq!(run["recorded_dispatch"]["binding"], variant);
    assert_eq!(registry.held_result(&handle).unwrap().recorded_table().map(|table| table.binding()), Some(variant.clone()), "the handle holds the record's table");

    // Current admission is unaffected: the bare verdict is the current reader's, under the current
    // table (not the record's), and it withholds the statement as before.
    let verdict = &crate::native_replay_tests::reply(&registry, crate::native_verdict::COMMAND, &json!({"jobId": handle}))["value"];
    assert_ne!(verdict["binding"], variant, "{verdict}");
    assert_eq!(verdict["standing"]["eligible"], false, "{verdict}");
    assert!(verdict["standing"]["findings"].as_array().unwrap().iter().any(|f| f == "SOURCE_PREVIEW_PHYSICS_FORMULATION_BASIS"), "{verdict}");

    // S and row semantics on the handle read under its table; a solve holding the same bytes holds no table.
    let summary = |job: &str| {
        let (raw, lease) = crate::result_summary::reserve(&registry, &body(&json!({"jobId": job, "modelJson": "null", "standing": null})), &headers(Some(INGRESS_ARGS_JSON_V1))).unwrap();
        let bytes = crate::result_summary::run(registry.clone(), raw, lease).unwrap();
        crate::result_planes::decode(&bytes).unwrap().header["tables"]["results_panel"]["family_counts"].clone()
    };
    let semantics = |job: &str| {
        let command = crate::result_detail::SEMANTICS_COMMAND;
        let (raw, lease) = crate::result_detail::reserve(&registry, &body(&json!({"jobId": job, "ordinals": [0, 1, 2, 3]})), &headers(Some(INGRESS_ARGS_JSON_V1)), command).unwrap();
        serde_json::from_str::<Value>(&crate::result_detail::run_semantics(registry.clone(), raw, lease).unwrap()).unwrap()
    };
    let restored_counts = summary(&handle);
    assert!(restored_counts["displacement"].as_u64().unwrap() > 0 && restored_counts["other"] != restored_counts["total"], "{restored_counts}");
    let restored_rows = semantics(&handle);
    assert!(restored_rows.as_array().unwrap().iter().all(|outcome| outcome["ok"] == true && !outcome["signature"].is_null()), "{restored_rows}");
    let solve = held(&registry, &gate, &stored.0, &stringify_text(&repo_text(REPLAY_MODEL)), "sparse_interactive", ResultCustody::Native);
    assert_eq!(registry.held_result(&solve).unwrap().recorded_table(), None, "a solve holds no recorded table");
    // The rows are the same under either table (only limitations, policy text and vocabulary moved):
    // what the current reader withholds is the statement (its dispatch), not its rows.
    assert_eq!(semantics(&solve), restored_rows, "the rows read alike under the current and the recorded table");
    assert!(registry.dispose_result(&solve).unwrap().released);

    // The webview historical panel's requests on the handle, answered by their actual commands.
    let requests: Value = serde_json::from_str(&std::fs::read_to_string(root().join(PRE_RT_A4_REQUESTS)).unwrap_or_else(|_| "{\"requests\":[]}".into())).unwrap();
    let answered: Vec<Value> = requests["requests"].as_array().unwrap().iter().map(|request| {
        let command = request["command"].as_str().unwrap();
        json!({"command": command, "args": request["args"], "reply": crate::native_replay_tests::reply(&registry, command, &with_handle(&request["args"], &handle))})
    }).collect();

    // Re-save from the handle keeps the stored texts byte for byte and the read under the table.
    // The unchanged re-save, against the submitted path's bytes (`resave_matches_legacy`).
    let base = opened["project_envelope_hash"]["value"].clone();
    let args = crate::stored_save::tests::resave_args(&opened, &crate::stored_save::tests::opened_fields(&opened), "save", base, Value::Null);
    let saved = crate::stored_save::tests::resave_matches_legacy(&registry, &path, &opened, &args, "pre-RT-A4 unchanged");
    assert_eq!(saved["restored"]["findings"]["recorded_table_read"], variant, "{saved}");
    assert_eq!(saved["restored"]["findings"]["findings"], case["expected"]["findings"], "the re-saved project's findings are unchanged");
    assert_eq!(column_text(&path, "SELECT mechanics_result_json FROM local_projects"), stored.0, "the stored result is kept");
    assert_eq!(column_text(&path, "SELECT analysis_run_json FROM local_projects"), stored.1, "the stored record is kept: no migration, no reseal");
    assert!(registry.dispose_result(&handle).unwrap().released);
    assert!(registry.held_result(&handle).is_err(), "disposal ends the held table with the handle");

    let document = json!({
        "document_kind": "swbpipe.desktop.native_reopen_pre_rt_a4_replies",
        "note": "Written by stored_custody_tests.rs from the actual commands on a real store holding main's preview-physics-2 run recorded before RT-A4 (`fixtures/persistence/recorded_pre_rt_a4_pf`, the reopen corpus's `saved-pre-rt-a4:preview-physics-2`): the native open and the historical panel's reads on the restored handle (written $HANDLE; the store's path $STORE); NATIVE_REOPEN_REPLIES_WRITE=1 rewrites it.",
        "case": PRE_RT_A4_CASE,
        "open": {"command": OPEN_COMMAND, "args": {"projectId": project_id}, "reply": {"ok": true, "body": "json", "value": webview_value(opened, &handle)}},
        "requests": answered,
    });
    let written = format!("{}\n", serde_json::to_string_pretty(&document).unwrap());
    let target = root().join(PRE_RT_A4_REPLIES);
    if std::env::var_os("NATIVE_REOPEN_REPLIES_WRITE").is_some() {
        std::fs::write(&target, &written).unwrap();
    }
    assert!(std::fs::read_to_string(&target).unwrap_or_default() == written, "the committed pre-RT-A4 reopen replies are the commands' replies (NATIVE_REOPEN_REPLIES_WRITE=1 rewrites them)");
    std::fs::remove_dir_all(&dir).unwrap();
    assert_eq!(registry.pool().reserved_total(), 0);
}

/// A restored handle holding `text`, read under `table`, as `stored_custody` restores one.
fn restored_under(registry: &SolveJobRegistry, text: &str, table: Option<crate::analysis_run_v03_validate::RecordedTable>) -> String {
    let lease = registry.acquire_result_parse(text.len() as u64).unwrap();
    let source = RestoredSource { kind: "saved_project_run", project_id: "invented:project".into(), recording_id: None, result_sha256: "invented".into(), record_sha256: None };
    registry.restore(registry.reset_epoch(), source, HeldResult::restored(text.to_string(), lease).read_under(table), None).unwrap()
}

/// A native reader's reply on its job: the response bytes, or the refusal's JSON.
fn reader_reply(registry: &SolveJobRegistry, command: &'static str, args: &Value) -> Result<Vec<u8>, String> {
    use crate::{result_detail, result_summary};
    let (body, headers) = (body(args), headers(Some(INGRESS_ARGS_JSON_V1)));
    let r = registry.clone();
    let outcome = match command {
        result_summary::COMMAND => result_summary::reserve(registry, &body, &headers).and_then(|(raw, lease)| result_summary::run(r, raw, lease)),
        result_detail::SEMANTICS_COMMAND => result_detail::reserve(registry, &body, &headers, command).and_then(|(raw, lease)| result_detail::run_semantics(r, raw, lease)).map(String::into_bytes),
        result_detail::ROWS_COMMAND => result_detail::reserve(registry, &body, &headers, command).and_then(|(raw, lease)| result_detail::run_rows(r, raw, lease)),
        result_detail::NAVIGATOR_COMMAND => result_detail::reserve(registry, &body, &headers, command).and_then(|(raw, lease)| result_detail::run_navigator(r, raw, lease)),
        result_detail::LOOKUP_COMMAND => result_detail::reserve(registry, &body, &headers, command).and_then(|(raw, lease)| result_detail::run_lookup(r, raw, lease)),
        other => panic!("not a reader under test: {other}"),
    };
    outcome.map_err(|error| serde_json::to_string(&error).unwrap())
}

/// RT-A4 §10.4 review F1: a reader takes the held bytes and the recorded table they are read
/// under as one snapshot. A disposal or a session reset between a reader's acquisition of its
/// share and its read (the test gate in `held_result`) must not change the table that read uses:
/// every native reader of the handle (S, row semantics, the rows page and its family filter, the
/// navigator, the lookup) answers exactly as an undisturbed handle under the same table does. Two
/// stored statements: main's preview-physics-2 run recorded before RT-A4 under its own variant
/// (read under it; its rows read alike under the current tables), and a current
/// preview-physics-2 statement whose record binds the Timoshenko variant, another identity (the
/// binding is refused, so every row is refused; under the current tables the rows would read).
/// Before the table was part of the share, the readers looked it up again in the registry after
/// acquiring the share; after a disposal that lookup found nothing and they read the current
/// tables, so S, semantics, rows and navigator differed here.
#[test]
fn a_disposal_or_reset_during_a_read_never_changes_the_recorded_table_it_reads() {
    use crate::analysis_run_v03_validate::RecordedTable;
    use crate::{result_detail, result_summary};
    use open_pipe_stress_result_export::recorded_tables::VARIANTS;
    use std::time::Duration;
    let pre_rt_a4 = std::fs::read_to_string(root().join("fixtures/persistence/recorded_pre_rt_a4_pf/preview_physics_2/preview_physics_connected_sparse.json")).unwrap();
    let current = std::fs::read_to_string(root().join("fixtures/results/preview_physics_connected_sparse.json")).unwrap();
    let (preview, timoshenko) = (RecordedTable::of(&VARIANTS[0]), RecordedTable::of(&VARIANTS[1]));
    assert_eq!((preview.id, timoshenko.id), (open_pipe_stress_result_export::semantic_contract::PREVIEW_PHYSICS_ID, open_pipe_stress_result_export::semantic_contract::PREVIEW_PHYSICS_TIMOSHENKO_ID));
    let family = json!({"text": "", "family": "displacement", "frame": "all", "case": "all"});
    let readers: [(&str, fn(&str, &Value) -> Value, bool); 5] = [
        (result_summary::COMMAND, |job, _| json!({"jobId": job, "modelJson": "null", "standing": null}), true),
        (result_detail::SEMANTICS_COMMAND, |job, _| json!({"jobId": job, "ordinals": (0..12).collect::<Vec<u64>>()}), true),
        (result_detail::ROWS_COMMAND, |job, family| json!({"jobId": job, "filter": family, "offset": 0, "limit": 50}), true),
        (result_detail::NAVIGATOR_COMMAND, |job, family| json!({"jobId": job, "filter": family, "probe": null}), true),
        (result_detail::LOOKUP_COMMAND, |job, _| json!({"jobId": job, "kinds": ["preview_intensified_bending_stress"], "match": "first"}), false),
    ];
    let mut failures = Vec::new();
    for (case, text, table, refused) in [("pre-RT-A4 under its variant", &pre_rt_a4, preview, false), ("refused: another identity's variant", &current, timoshenko, true)] {
        for (command, args, discriminates) in readers {
            for interleaving in ["disposal", "reset"] {
                let label = format!("{case}, {command}, {interleaving}");
                let host = ScriptedHost::normal();
                let (registry, _gate) = gated(&host);
                // An undisturbed handle under the table, and one under the current tables.
                let undisturbed = restored_under(&registry, text, Some(table));
                let expected = reader_reply(&registry, command, &args(&undisturbed, &family));
                let current_tables = restored_under(&registry, text, None);
                let unbound = reader_reply(&registry, command, &args(&current_tables, &family));
                let handle = restored_under(&registry, text, Some(table));
                assert_eq!((undisturbed.len(), current_tables.len()), (handle.len(), handle.len()), "{label}: the handles' replies compare byte for byte");
                let named = |reply: Result<Vec<u8>, String>, job: &str| -> Result<Vec<u8>, String> {
                    let swap = |bytes: &[u8]| bytes.windows(job.len()).enumerate().filter(|(_, w)| *w == job.as_bytes()).map(|(at, _)| at).collect::<Vec<_>>();
                    reply.map(|mut bytes| { for at in swap(&bytes) { bytes[at..at + job.len()].copy_from_slice("$".repeat(job.len()).as_bytes()); } bytes })
                        .map_err(|text| text.replace(job, &"$".repeat(job.len())))
                };
                let (expected, unbound) = (named(expected, &undisturbed), named(unbound, &current_tables));
                // The refused binding is what a current-table read would replace (review F1); the
                // run recorded before RT-A4 reads its rows alike under either table.
                if discriminates && refused {
                    assert_ne!(expected, unbound, "{label}: the reader's reply depends on the table it reads under");
                }
                if refused && command == result_detail::SEMANTICS_COMMAND {
                    let outcomes: Value = serde_json::from_slice(expected.as_ref().unwrap()).unwrap();
                    assert!(outcomes.as_array().unwrap().iter().all(|outcome| outcome["ok"] == false && outcome.to_string().contains("SOURCE_RECORDED_BINDING_IDENTITY")), "{label}: every row refused: {outcomes}");
                }
                let held = command_gate::hold(&registry, "held", false);
                let reader = std::thread::spawn({
                    let (registry, args) = (registry.clone(), args(&handle, &family));
                    move || reader_reply(&registry, command, &args)
                });
                assert!(held.0.wait_entered(1, Duration::from_secs(30)), "{label}: the reader acquired its share");
                if interleaving == "disposal" {
                    assert!(registry.dispose_result(&handle).unwrap().released, "{label}");
                } else {
                    assert!(registry.dispose_all_results().released_results >= 1, "{label}");
                }
                assert!(registry.held_result(&handle).is_err(), "{label}: the handle is gone while its reader reads");
                held.0.open();
                let got = named(reader.join().unwrap(), &handle);
                drop(held);
                if got != expected {
                    failures.push(format!("{label}: the read used another table than its share's"));
                }
                if interleaving == "disposal" {
                    registry.dispose_result(&undisturbed).unwrap();
                    registry.dispose_result(&current_tables).unwrap();
                }
                assert_eq!(registry.pool().reserved_total(), 0, "{label}: every share released");
            }
        }
    }
    assert!(failures.is_empty(), "{} of 20 interleavings differ:\n{}", failures.len(), failures.join("\n"));
}

#[test]
fn matching_diagnostics_are_capped_before_the_header_is_allocated() {
    let host = ScriptedHost::normal();
    let (registry, _gate) = gated(&host);
    let diagnostic = json!({"source_annotation":{"source":RULE_PACK_REF_DIAGNOSTIC_SOURCE}, "message":"x".repeat(1024)});
    let record = json!({"analysis_run":{"diagnostics":vec![diagnostic; 5000]}});
    let ((header, omitted, held), measured) = crate::ingress::counting::measure(|| record_header(&registry, &record).unwrap());
    assert!(header.is_null());
    assert!(omitted.unwrap() > RECORD_HEADER_LIMIT_BYTES);
    assert!(held.is_none());
    assert!(measured.peak_priced < 64 * 1024, "omission cloned diagnostics: {measured:?}");
    assert_eq!(registry.pool().reserved_total(), 0);
}

#[test]
fn the_header_cap_includes_wrapper_and_escaped_key_bytes() {
    let host = ScriptedHost::normal();
    let (registry, _gate) = gated(&host);
    let mut record = json!({"\"\\\n": ""});
    let (header, omitted, held) = record_header(&registry, &record).unwrap();
    assert!(omitted.is_none());
    let overhead = serde_json::to_vec(&header).unwrap().len();
    drop((header, held));
    record["\"\\\n"] = json!("x".repeat(RECORD_HEADER_LIMIT_BYTES as usize - overhead));
    let (header, omitted, held) = record_header(&registry, &record).unwrap();
    assert!(omitted.is_none());
    assert_eq!(serde_json::to_vec(&header).unwrap().len() as u64, RECORD_HEADER_LIMIT_BYTES);
    drop((header, held));
    let text = format!("{}x", record["\"\\\n"].as_str().unwrap());
    record["\"\\\n"] = json!(text);
    let (header, omitted, held) = record_header(&registry, &record).unwrap();
    assert!(header.is_null() && held.is_none());
    assert_eq!(omitted, Some(RECORD_HEADER_LIMIT_BYTES + 1));
    assert_eq!(registry.pool().reserved_total(), 0);
}

use crate::command_ingress::test_support::headers;
use crate::command_ingress::INGRESS_ARGS_JSON_V1;
use crate::memory_pool::test_host::ScriptedHost;
use crate::native_persistence::tests::{cases as persistence_cases, held, native_record, native_request, native_save, scratch, stored_project, unlimited, Case};
use crate::native_verdict::tests::gated;
use crate::solve_jobs::{ResultCustody, ResultOrigin};
use rusqlite::Connection;
use std::path::Path;

fn body(args: &Value) -> InvokeBody {
    InvokeBody::Raw(serde_json::to_vec(args).unwrap())
}

fn text_of(error: CommandError<String>) -> String {
    match error {
        CommandError::Command(text) | CommandError::Response(text) => text,
        CommandError::Ingress(error) => format!("INGRESS: {error:?}"),
    }
}

/// The project open's two steps in this thread; its reply as the webview reads it (a JSON body).
pub(crate) fn native_open(registry: &SolveJobRegistry, path: &Path, project_id: Option<&str>) -> Result<Value, String> {
    let epoch = registry.reset_epoch();
    let (raw, lease) = reserve(registry, &body(&json!({"projectId": project_id})), &headers(Some(INGRESS_ARGS_JSON_V1)), OPEN_COMMAND).map_err(text_of)?;
    let path = path.to_path_buf();
    run_open(registry, move || Ok(path), raw, lease, epoch).map(reply_json).map_err(text_of)
}

/// The recorded run's reopen, two steps in this thread.
pub(crate) fn native_read(registry: &SolveJobRegistry, path: &Path, project_id: &str, recording_id: &str) -> Result<Value, String> {
    let epoch = registry.reset_epoch();
    let (raw, lease) = reserve(registry, &body(&json!({"projectId": project_id, "recordingId": recording_id})), &headers(Some(INGRESS_ARGS_JSON_V1)), RUN_COMMAND).map_err(text_of)?;
    let path = path.to_path_buf();
    run_read(registry, move || Ok(path), raw, lease, epoch).map(reply_json).map_err(text_of)
}

/// A reply's JSON body (`tauri::ipc::Response` holds it as text).
pub(crate) fn reply_json(reply: tauri::ipc::Response) -> Value {
    use tauri::ipc::IpcResponse;
    let tauri::ipc::InvokeResponseBody::Json(text) = reply.body().unwrap() else { panic!("the reply is a JSON body") };
    serde_json::from_str(&text).unwrap()
}

fn column_text(path: &Path, sql: &str) -> String {
    Connection::open(path).unwrap().query_row(sql, [], |row| row.get(0)).unwrap()
}

fn file_digest(path: &Path) -> String {
    hex(&sha256_in_place(&std::fs::read(path).unwrap()))
}

/// The detail rows command on a handle: the unfiltered first page's total.
fn rows_total(registry: &SolveJobRegistry, handle: &str) -> u64 {
    let args = json!({"jobId": handle, "filter": {}, "offset": 0, "limit": 1});
    let (raw, lease) = crate::result_detail::reserve(registry, &body(&args), &headers(Some(INGRESS_ARGS_JSON_V1)), crate::result_detail::ROWS_COMMAND).unwrap();
    let bytes = crate::result_detail::run_rows(registry.clone(), raw, lease).unwrap();
    assert_eq!(&bytes[..8], crate::result_detail::MAGIC);
    let length = u64::from_le_bytes(bytes[8..16].try_into().unwrap()) as usize;
    let header: Value = serde_json::from_slice(&bytes[16..16 + length]).unwrap();
    header["total"].as_u64().unwrap()
}

/// S on a handle with no model and no standing, as the historical panel reads it.
fn summary_rows(registry: &SolveJobRegistry, handle: &str) -> u64 {
    let args = json!({"jobId": handle, "modelJson": "null", "standing": null});
    let (raw, lease) = crate::result_summary::reserve(registry, &body(&args), &headers(Some(INGRESS_ARGS_JSON_V1))).unwrap();
    let bytes = crate::result_summary::run(registry.clone(), raw, lease).unwrap();
    let length = u64::from_le_bytes(bytes[8..16].try_into().unwrap()) as usize;
    let header: Value = serde_json::from_slice(&bytes[16..16 + length]).unwrap();
    header["row_count"].as_u64().unwrap()
}

/// What every restored handle must hold: the stored texts exactly, its
/// stored origin, no solve digests, readable rows; and every fresh
/// persistence route refuses it.
fn check_restored(registry: &SolveJobRegistry, case: &Case, path: &Path, handle: &str, result_sql: &str, record_sql: &str) {
    let stored_result = column_text(path, result_sql);
    let stored_record = column_text(path, record_sql);
    assert!(handle.starts_with(crate::solve_jobs::RESTORED_HANDLE_PREFIX), "{handle}");
    assert_eq!(registry.held_result(handle).unwrap().text(), stored_result, "{}: the held bytes are the stored result", case.id);
    assert_eq!(registry.restored_record(handle).unwrap().text(), stored_record, "{}: the held record is the stored record", case.id);
    assert!(registry.custody_binding(handle).is_none(), "a restored result has no solve custody");
    match registry.result_origin(handle) {
        Some(ResultOrigin::Restored(source)) => {
            assert_eq!(source.result_sha256, hex(&sha256_in_place(stored_result.as_bytes())));
            assert_eq!(source.record_sha256.as_deref(), Some(hex(&sha256_in_place(stored_record.as_bytes()))).as_deref());
        }
        other => panic!("{other:?}"),
    }
    let rows = serde_json::from_str::<Value>(&stored_result).unwrap()["results"].as_array().map_or(0, Vec::len) as u64;
    assert_eq!(rows_total(registry, handle), rows, "{}: paged rows read the restored bytes", case.id);
    assert_eq!(summary_rows(registry, handle), rows, "{}: S reads the restored bytes", case.id);
    // Fresh persistence refuses it, whatever the request names (the solve's own intake included).
    let before = file_digest(path);
    let refused = native_record(registry, path, &case.record_args(handle, native_request(None)), None).unwrap_err();
    assert!(refused.starts_with(crate::native_persistence::CUSTODY) && refused.contains("restored"), "{refused}");
    let refused = native_save(registry, path, &case.save_args(handle, "save", Value::Null, Value::Null), &unlimited()).unwrap_err();
    assert!(refused.starts_with(crate::native_persistence::CUSTODY) && refused.contains("restored"), "{refused}");
    assert_eq!(file_digest(path), before, "nothing was written");
    // No fresh transient record is previewed over it.
    let args = json!({"jobId": handle, "inputManifest": case.case["input_manifest"], "ruleCheckAggregate": case.case["rule_check_aggregate"], "rulePackExclusions": []});
    let (raw, lease) = crate::analysis_run_v03_build::reserve(registry, &body(&args), &headers(Some(INGRESS_ARGS_JSON_V1))).unwrap();
    let refused = text_of(crate::analysis_run_v03_build::run(registry.clone(), raw, lease).unwrap_err());
    assert!(refused.starts_with(crate::analysis_run_v03_build::PREVIEW_READ_FAILED), "{refused}");
    // The take refuses it; the bounded C2 copy reads it at or under 64 MiB.
    assert!(registry.take_result(handle).is_err());
    assert_eq!(registry.copy_result_within(handle, crate::solve_jobs::MAX_DISPLAYABLE_RESULT_BYTES).unwrap(), stored_result.as_bytes());
    assert!(registry.release_full_copy(handle).unwrap());
    let (copy, _lease) = registry.copy_restored_record_within(handle, crate::solve_jobs::MAX_DISPLAYABLE_RESULT_BYTES).unwrap();
    assert_eq!(copy, stored_record.as_bytes());
}

/// Every persistence case: the native Save stores the run; after the job is
/// gone and the session reset, the native open restores the stored bytes
/// into native custody with the findings the legacy open's values give.
#[test]
fn a_native_save_reopens_into_native_custody_after_a_reset() {
    let host = ScriptedHost::normal();
    let (registry, gate) = gated(&host);
    let dir = scratch("reopen-save");
    for case in &persistence_cases() {
        let name = case.id.replace(|c: char| !c.is_ascii_alphanumeric(), "-");
        let path = dir.join(format!("{name}.sqlite"));
        let job = held(&registry, &gate, &case.text, &case.intake, &case.mode, ResultCustody::Native);
        let saved = native_save(&registry, &path, &case.save_args(&job, "create", Value::Null, Value::Null), &unlimited()).unwrap_or_else(|e| panic!("{}: {e}", case.id));
        assert_eq!(saved["result"]["outcome"], "saved");
        registry.dispose_all_results();
        assert_eq!(registry.pool().reserved_total(), 0);
        let before = file_digest(&path);
        let project_id = case.model()["project"]["id"].as_str().unwrap();
        let opened = native_open(&registry, &path, Some(project_id)).unwrap_or_else(|e| panic!("{}: {e}", case.id));
        assert_eq!(file_digest(&path), before, "{}: opening wrote nothing", case.id);
        let run = &opened["stored_run"];
        assert_eq!(run["state"], "restored", "{}: {run}", case.id);
        assert_eq!(run["origin"], "restored_saved_run");
        for absent in ["mechanics_result", "analysis_run"] {
            assert!(opened.get(absent).is_none(), "the reply carries no {absent}");
        }
        let text = serde_json::to_string(&opened).unwrap();
        assert!(!text.contains("\"result_refs\":["), "{}: no ResultRef crosses", case.id);
        assert!(run["record_header"]["result_ref_count"].as_u64().is_some());
        let handle = run["handle"].as_str().unwrap();
        check_restored(&registry, case, &path, handle, "SELECT mechanics_result_json FROM local_projects", "SELECT analysis_run_json FROM local_projects");
        // The findings are those of the legacy open's values (whose TypeScript reading is the oracle above).
        let legacy = crate::open_project_in_store(path.clone(), Some(project_id)).unwrap().unwrap();
        let legacy_json = serde_json::to_value(&legacy).unwrap();
        let project = SavedProject {
            model: &legacy_json["model"],
            editor_intents: &legacy_json["editor_intents"],
            proposal: &legacy_json["proposal"],
            selected_review_target: &legacy_json["selected_review_target"],
            model_hash: &legacy_json["model_hash"],
            project_envelope_hash: &legacy_json["project_envelope_hash"],
        };
        let (mut result, record) = (legacy_json["mechanics_result"].clone(), legacy_json["analysis_run"].clone());
        let mut expected = saved_run_findings(&project, &mut result, &record).unwrap();
        saved_run_validation(&mut expected, &mut result, &record);
        assert_eq!(run["findings"], serde_json::to_value(&expected).unwrap(), "{}", case.id);
        assert!(!expected.findings.iter().any(|finding| finding.contains("MISMATCH")), "{}: a fresh save reopens clean: {:?}", case.id, expected.findings);
        for key in ["summary", "model", "editor_intents", "proposal", "selected_review_target", "model_hash", "project_envelope_hash", "model_document_version"] {
            assert_eq!(opened[key], legacy_json[key], "{}: {key} as open_local_project returns it", case.id);
        }
        assert!(registry.dispose_result(handle).unwrap().released);
        assert!(registry.dispose_result(handle).map(|disposal| !disposal.released).unwrap_or(true), "released once");
        assert!(registry.held_result(handle).is_err(), "no disposed handle resurrects");
    }
    std::fs::remove_dir_all(&dir).unwrap();
    assert_eq!(registry.pool().reserved_total(), 0, "every lease ended");
}

/// Every persistence case: the native Record Run stores the run; the native
/// read restores it after every integrity check of `read_analysis_run`.
#[test]
fn a_native_record_run_reopens_into_native_custody_after_a_reset() {
    let host = ScriptedHost::normal();
    let (registry, gate) = gated(&host);
    let dir = scratch("reopen-record");
    for case in &persistence_cases() {
        let name = case.id.replace(|c: char| !c.is_ascii_alphanumeric(), "-");
        let path = dir.join(format!("{name}.sqlite"));
        stored_project(&path, case.project_id(), case.model());
        let job = held(&registry, &gate, &case.text, &case.intake, &case.mode, ResultCustody::Native);
        let outcome = native_record(&registry, &path, &case.record_args(&job, native_request(None)), None).unwrap_or_else(|e| panic!("{}: {e}", case.id));
        let recording_id = outcome["run"]["recording_id"].as_str().unwrap().to_string();
        registry.dispose_all_results();
        let before = file_digest(&path);
        let read = native_read(&registry, &path, case.project_id(), &recording_id).unwrap_or_else(|e| panic!("{}: {e}", case.id));
        assert_eq!(file_digest(&path), before, "{}: reading wrote nothing", case.id);
        let restored = &read["restored"];
        assert_eq!(restored["origin"], "restored_recorded_run");
        let legacy = crate::analysis_runs::read_run_in_store(&path, case.project_id(), &recording_id).unwrap();
        assert_eq!(read["run"], serde_json::to_value(&legacy.run).unwrap());
        assert_eq!(read["state"], serde_json::to_value(&legacy.state).unwrap());
        assert_eq!(read["input_manifest_canonical"], json!(legacy.input_manifest_canonical));
        assert!(read.get("result_json").is_none() && read.get("record").is_none());
        let handle = restored["handle"].as_str().unwrap();
        let sql = |column: &str| format!("SELECT {column} FROM analysis_runs WHERE recording_id = '{recording_id}'");
        check_restored(&registry, case, &path, handle, &sql("result_json"), &sql("run_record_json"));
        let context = RecordedRead {
            state_id: &legacy.state.state_id,
            state_project_id: &legacy.state.project_id,
            state_model_hash: &legacy.state.model_hash,
            run_state_id: &legacy.run.state_id,
            producer_run_id: &legacy.run.producer_run_id,
            run_record_hash: &legacy.run.run_record_hash,
        };
        let mut result: Value = serde_json::from_str(&legacy.result_json).unwrap();
        let manifest: Value = serde_json::from_str(&legacy.input_manifest_canonical).unwrap();
        let mut expected = recorded_run_findings(&context, Some(&mut result), Some(&manifest), &legacy.record);
        recorded_run_validation(&mut expected, &context, &mut result, Some(&manifest), &legacy.record);
        assert_eq!(restored["findings"], serde_json::to_value(&expected).unwrap(), "{}", case.id);
        assert!(!expected.findings.iter().any(|finding| finding.contains("MISMATCH")), "{}: {:?}", case.id, expected.findings);
        assert!(registry.dispose_result(handle).unwrap().released);
    }
    std::fs::remove_dir_all(&dir).unwrap();
    assert_eq!(registry.pool().reserved_total(), 0);
}

/// The first persistence case with at least one row, saved and recorded natively on one store.
fn stored_case(registry: &SolveJobRegistry, gate: &crate::solve_worker::test_support::Gate, path: &Path) -> (Case, String) {
    let case = persistence_cases().into_iter().next().unwrap();
    let job = held(registry, gate, &case.text, &case.intake, &case.mode, ResultCustody::Native);
    let saved = native_save(registry, path, &case.save_args(&job, "create", Value::Null, Value::Null), &unlimited()).unwrap();
    let base = saved["project_envelope_hash"]["value"].clone();
    let mut request = native_request(None);
    request["expected_base"] = base;
    let outcome = native_record(registry, path, &case.record_args(&job, request), None).unwrap();
    registry.dispose_all_results();
    (case, outcome["run"]["recording_id"].as_str().unwrap().to_string())
}

#[test]
fn both_reopen_headers_hold_their_leases_until_reply_serialization_ends() {
    let host = ScriptedHost::normal();
    let (registry, gate) = gated(&host);
    let dir = scratch("reopen-header-lease");
    let path = dir.join("store.sqlite");
    let (case, recording_id) = stored_case(&registry, &gate, &path);
    let project_id = case.project_id();
    let epoch = registry.reset_epoch();
    let opened = open_project_in_store(&registry, path.clone(), Some(project_id), epoch).unwrap().unwrap();
    registry.dispose_all_results();
    assert!(registry.pool().reserved_total() > 0, "project header outlived its lease");
    let (text, response_lease) = reply(&registry, &opened).unwrap();
    drop((text, response_lease));
    assert!(registry.pool().reserved_total() > 0);
    drop(opened);
    assert_eq!(registry.pool().reserved_total(), 0);
    let read = read_run_in_store(&registry, path.clone(), project_id, &recording_id, registry.reset_epoch()).unwrap();
    registry.dispose_all_results();
    assert!(registry.pool().reserved_total() > 0, "recorded header outlived its lease");
    let (text, response_lease) = reply(&registry, &read).unwrap();
    drop((text, response_lease));
    assert!(registry.pool().reserved_total() > 0);
    drop(read);
    assert_eq!(registry.pool().reserved_total(), 0);
    std::fs::remove_dir_all(dir).unwrap();
}

/// #51's withholding stays on the legacy whole-value reads (lowered seam);
/// a run above the old limit is held natively, by its lengths alone never withheld.
#[test]
fn the_old_display_limit_withholds_only_the_legacy_reads() {
    let host = ScriptedHost::normal();
    let (registry, gate) = gated(&host);
    let dir = scratch("reopen-limit");
    let path = dir.join("store.sqlite");
    let (case, recording_id) = stored_case(&registry, &gate, &path);
    let project_id = case.model()["project"]["id"].as_str().unwrap();
    let connection = Connection::open(&path).unwrap();
    let lowered = 16;
    let legacy = crate::load_project_within(&connection, Some(project_id), lowered).unwrap().unwrap();
    assert!(legacy.withheld.is_some() && legacy.mechanics_result.is_null(), "the legacy open withholds above the lowered limit");
    let refused = crate::analysis_runs::read_run_in_store_within(&path, project_id, &recording_id, &crate::stored_read::UnmeteredReadMemory, lowered).unwrap_err();
    assert!(refused.starts_with(crate::solve_jobs::STORED_RESULT_TOO_LARGE_CODE), "{refused}");
    let opened = native_open(&registry, &path, Some(project_id)).unwrap();
    assert_eq!(opened["stored_run"]["state"], "restored");
    assert!(opened["stored_run"]["result_bytes"].as_u64().unwrap() > lowered);
    let read = native_read(&registry, &path, project_id, &recording_id).unwrap();
    assert!(read["restored"]["result_bytes"].as_u64().unwrap() > lowered);
    registry.dispose_all_results();
    std::fs::remove_dir_all(&dir).unwrap();
    assert_eq!(registry.pool().reserved_total(), 0);
}

/// The pool refuses the held bytes' leases: the project opens with its model
/// editable and its run refused by name, nothing loaded, held or written; the
/// recorded run's read is the pool's named refusal. Every lease ends.
#[test]
fn a_pool_refusal_keeps_the_model_editable_and_the_store_untouched() {
    use crate::memory_pool::PressureLevel;
    const GIB: u64 = 1 << 30;
    let host = ScriptedHost::with(Some(256 * GIB), Some(1024 * GIB), PressureLevel::Normal);
    let (registry, gate) = gated(&host);
    registry.configure_pool(192 * GIB, 64 * GIB, false).unwrap();
    let dir = scratch("reopen-pool");
    let path = dir.join("store.sqlite");
    let (case, recording_id) = stored_case(&registry, &gate, &path);
    let project_id = case.model()["project"]["id"].as_str().unwrap();
    let result_bytes = column_text(&path, "SELECT mechanics_result_json FROM local_projects").len() as u64;
    // A budget the arguments fit and the held result's envelope does not.
    let budget = (crate::ingress::allocation_envelope(result_bytes).unwrap() - 1).max(64 * 1024);
    registry.configure_pool(192 * GIB, budget, false).unwrap();
    let before = file_digest(&path);
    let opened = native_open(&registry, &path, Some(project_id)).unwrap();
    let run = &opened["stored_run"];
    assert_eq!(run["state"], "refused", "{run}");
    assert_eq!(run["code"], RESTORE_REFUSED);
    assert!(run["message"].as_str().unwrap().contains(crate::stored_read::STORE_READ_MEMORY_UNAVAILABLE), "{run}");
    assert_eq!(run["result_bytes"].as_u64().unwrap(), result_bytes);
    assert!(opened["model"].is_object() && opened["model_document_version"].is_object(), "the model opens for editing");
    assert_eq!(registry.pool_status().retained_results, 0, "nothing is held");
    let refused = native_read(&registry, &path, project_id, &recording_id).unwrap_err();
    assert!(refused.contains(crate::stored_read::STORE_READ_MEMORY_UNAVAILABLE), "{refused}");
    assert_eq!(file_digest(&path), before, "the stored evidence is untouched");
    registry.configure_pool(192 * GIB, 64 * GIB, false).unwrap();
    std::fs::remove_dir_all(&dir).unwrap();
    assert_eq!(registry.pool().reserved_total(), 0, "every lease ended");
}

/// A reset during the read refuses the registration and drops what was
/// loaded; full retention refuses by name; a stored result that is not JSON
/// fails the open with the legacy open's own message.
#[test]
fn resets_retention_and_corruption_are_refused_with_nothing_held() {
    let host = ScriptedHost::normal();
    let (registry, gate) = gated(&host);
    let dir = scratch("reopen-refusals");
    let path = dir.join("store.sqlite");
    let (case, _) = stored_case(&registry, &gate, &path);
    let project_id = case.model()["project"]["id"].as_str().unwrap().to_string();
    // A reset after the command began.
    let epoch = registry.reset_epoch();
    registry.dispose_all_results();
    let opened = open_project_in_store(&registry, path.clone(), Some(&project_id), epoch).unwrap().unwrap();
    let opened = serde_json::to_value(&opened).unwrap();
    assert_eq!(opened["stored_run"]["state"], "refused");
    assert!(opened["stored_run"]["message"].as_str().unwrap().contains("RESTORED-RESULT-SESSION-RESET"), "{}", opened["stored_run"]);
    assert_eq!(registry.pool().reserved_total(), 0, "nothing loaded stays held");
    // Retention full: four retained solves.
    let jobs: Vec<String> = (0..crate::memory_pool::MAX_RETAINED_RESULTS).map(|_| crate::native_verdict::tests::retained(&registry, &gate, "{}")).collect();
    let opened = native_open(&registry, &path, Some(&project_id)).unwrap();
    assert_eq!(opened["stored_run"]["state"], "refused");
    assert!(opened["stored_run"]["message"].as_str().unwrap().contains("retention_full"), "{}", opened["stored_run"]);
    for job in &jobs {
        registry.dispose_result(job).unwrap();
    }
    assert_eq!(registry.pool().reserved_total(), 0);
    // Corruption: the legacy open's message, the store untouched.
    Connection::open(&path).unwrap().execute("UPDATE local_projects SET mechanics_result_json = '{\"run_id\": tru'", []).unwrap();
    let before = file_digest(&path);
    let legacy = crate::open_project_in_store(path.clone(), Some(&project_id)).unwrap_err();
    let native = native_open(&registry, &path, Some(&project_id)).unwrap_err();
    assert_eq!(native, legacy);
    assert_eq!(file_digest(&path), before);
    assert_eq!(registry.pool().reserved_total(), 0);
    std::fs::remove_dir_all(&dir).unwrap();
}

/// A disposal while a reader holds a share releases the bytes and both leases
/// only when the share drops; a second disposal releases nothing.
#[test]
fn a_restored_handle_is_released_once_by_its_last_holder() {
    let host = ScriptedHost::normal();
    let (registry, gate) = gated(&host);
    let dir = scratch("reopen-lifetime");
    let path = dir.join("store.sqlite");
    let (case, _) = stored_case(&registry, &gate, &path);
    let opened = native_open(&registry, &path, case.model()["project"]["id"].as_str()).unwrap();
    let handle = opened["stored_run"]["handle"].as_str().unwrap().to_string();
    let held_bytes = registry.pool().reserved_total();
    assert!(held_bytes > 0);
    let share = registry.held_result(&handle).unwrap();
    let record = registry.restored_record(&handle).unwrap();
    let disposal = registry.dispose_result(&handle).unwrap();
    assert!(disposal.released);
    assert_eq!(disposal.released_bytes, share.lease_bytes() + record.lease_bytes());
    assert_eq!(registry.pool().reserved_total(), held_bytes, "the readers still hold the bytes");
    drop(share);
    drop(record);
    assert_eq!(registry.pool().reserved_total(), 0);
    assert!(!registry.dispose_result(&handle).unwrap().released);
    std::fs::remove_dir_all(&dir).unwrap();
}

// ---------------------------------------------------------------------------
// The webview's reopen replies, from the actual commands
// ---------------------------------------------------------------------------

const REOPEN_REPLIES: &str = "apps/desktop/src/test-support/native-reopen/native_reopen_replies.json";
const REOPEN_REQUESTS: &str = "apps/desktop/src/test-support/native-reopen/native_reopen_requests.json";
const PERSISTENCE_REPLIES: &str = "apps/desktop/src/test-support/native-replay/native_persistence_replies.json";
const REPLAY_SOURCE: &str = "fixtures/results/pressure_connected_ui_mechanics_sparse.json";
const REPLAY_MODEL: &str = "fixtures/model_operations/exact_pressure_authoring_model.json";
const HANDLE: &str = "$HANDLE";

fn with_handle(value: &Value, handle: &str) -> Value {
    match value {
        Value::String(text) if text == HANDLE => json!(handle),
        Value::Array(items) => Value::Array(items.iter().map(|item| with_handle(item, handle)).collect()),
        Value::Object(map) => Value::Object(map.iter().map(|(key, item)| (key.clone(), with_handle(item, handle))).collect()),
        other => other.clone(),
    }
}

/// A reply with its restored handle written `$HANDLE` and the store's path `$STORE`.
fn webview_value(mut value: Value, handle: &str) -> Value {
    if let Some(summary) = value.get_mut("summary").and_then(Value::as_object_mut) {
        summary.insert("database_path".into(), json!("$STORE"));
    }
    fn walk(value: &mut Value, handle: &str) {
        match value {
            Value::String(text) if text == handle => *text = HANDLE.to_string(),
            Value::Array(items) => items.iter_mut().for_each(|item| walk(item, handle)),
            Value::Object(map) => map.values_mut().for_each(|item| walk(item, handle)),
            _ => {}
        }
    }
    walk(&mut value, handle);
    value
}

/// The session tests' solve (`nativeMechanicsReplayPair("sparse_interactive", {profile:
/// "pressure"})`) saved and recorded natively with the session's own requests
/// (`native_persistence_replies.json`), the session reset, then reopened: the project open's reply,
/// the recorded run's reply, and each request the webview's reopen sends on the restored handle
/// (`native_reopen_requests.json`, written by `nativeReopen.session.test.tsx` with
/// NATIVE_REOPEN_RECORD=1) answered by its actual command. NATIVE_REOPEN_REPLIES_WRITE=1 rewrites
/// `native_reopen_replies.json`.
#[test]
fn the_webviews_reopen_replies_are_the_commands_replies() {
    use crate::native_persistence::tests::{repo_text, stringify_text, with_job};
    let host = ScriptedHost::normal();
    let (registry, gate) = gated(&host);
    let dir = scratch("reopen-replies");
    let path = dir.join("store.sqlite");
    let persistence: Value = serde_json::from_str(&repo_text(PERSISTENCE_REPLIES)).unwrap();
    let scenarios: Vec<&Value> = persistence["scenarios"].as_array().unwrap().iter().filter(|scenario| scenario["store"] == "store").collect();
    let save_args = &scenarios[0]["args"];
    let manifest: Value = serde_json::from_str(save_args["inputManifest"]["canonical_bytes"].as_str().unwrap()).unwrap();
    let intake = stringify_text(&repo_text(REPLAY_MODEL));
    let job = held(&registry, &gate, &repo_text(REPLAY_SOURCE), &intake, manifest["solver_basis"]["solver_mode"].as_str().unwrap(), ResultCustody::Native);
    let saved = native_save(&registry, &path, &with_job(save_args, &job), &unlimited()).unwrap();
    assert_eq!(saved["result"]["outcome"], "saved");
    let recorded = native_record(&registry, &path, &with_job(&scenarios[1]["args"], &job), None).unwrap();
    let recording_id = recorded["run"]["recording_id"].as_str().unwrap().to_string();
    // Close: the session's reset releases the solve's custody; nothing of it remains.
    registry.dispose_all_results();
    assert_eq!(registry.pool().reserved_total(), 0);
    let project_id = save_args["projectId"].as_str().unwrap();
    let opened = native_open(&registry, &path, Some(project_id)).unwrap();
    let handle = opened["stored_run"]["handle"].as_str().unwrap().to_string();
    let requests: Value = serde_json::from_str(&std::fs::read_to_string(root().join(REOPEN_REQUESTS)).unwrap_or_else(|_| "{\"requests\":[]}".into())).unwrap();
    let mut answered = Vec::new();
    for request in requests["requests"].as_array().unwrap() {
        let command = request["command"].as_str().unwrap();
        let args = with_handle(&request["args"], &handle);
        // C1 slice 6's save continuity: the session's Save Project from the handle, on the same store.
        let reply = if command == crate::stored_save::COMMAND {
            match crate::stored_save::tests::native_resave(&registry, &path, &args, &unlimited()) {
                Ok(reply) => json!({"ok": true, "body": "json", "value": webview_value(reply, &handle)}),
                Err(error) => json!({"ok": false, "error": serde_json::to_value(CommandError::<String>::Command(error)).unwrap()}),
            }
        } else {
            crate::native_replay_tests::reply(&registry, command, &args)
        };
        answered.push(json!({"command": command, "args": request["args"], "reply": reply}));
    }
    assert!(registry.dispose_result(&handle).unwrap().released);
    let read = native_read(&registry, &path, project_id, &recording_id).unwrap();
    let read_handle = read["restored"]["handle"].as_str().unwrap().to_string();
    assert!(registry.dispose_result(&read_handle).unwrap().released);
    // The same store opened where the pool does not admit the held run: the model opens, the run is refused by name.
    let refused = {
        use crate::memory_pool::PressureLevel;
        const GIB: u64 = 1 << 30;
        let small_host = ScriptedHost::with(Some(256 * GIB), Some(1024 * GIB), PressureLevel::Normal);
        let (small, _gate) = gated(&small_host);
        let result_bytes = column_text(&path, "SELECT mechanics_result_json FROM local_projects").len() as u64;
        small.configure_pool(192 * GIB, crate::ingress::allocation_envelope(result_bytes).unwrap() - 1, false).unwrap();
        let opened = native_open(&small, &path, Some(project_id)).unwrap();
        assert_eq!(opened["stored_run"]["state"], "refused", "{}", opened["stored_run"]);
        assert_eq!(small.pool().reserved_total(), 0);
        webview_value(opened, "\u{0}")
    };
    let document = json!({
        "document_kind": "swbpipe.desktop.native_reopen_replies",
        "note": "Written by stored_custody_tests.rs from the actual commands on a real store (the session's native Save and Record Run, a session reset, then the reopen, the historical panel's reads and the session's Save Project from the restored handle; the store's path written $STORE, each restored handle $HANDLE); NATIVE_REOPEN_REPLIES_WRITE=1 rewrites it.",
        "source_path": REPLAY_SOURCE,
        "open": {"command": OPEN_COMMAND, "args": {"projectId": project_id}, "reply": {"ok": true, "body": "json", "value": webview_value(opened, &handle)}},
        "read": {"command": RUN_COMMAND, "args": {"projectId": project_id, "recordingId": recording_id}, "reply": {"ok": true, "body": "json", "value": webview_value(read, &read_handle)}},
        "open_refused": {"command": OPEN_COMMAND, "args": {"projectId": project_id}, "reply": {"ok": true, "body": "json", "value": refused}},
        "requests": answered,
    });
    let written = format!("{}\n", serde_json::to_string_pretty(&document).unwrap());
    let target = root().join(REOPEN_REPLIES);
    if std::env::var_os("NATIVE_REOPEN_REPLIES_WRITE").is_some() {
        std::fs::write(&target, &written).unwrap();
    }
    assert!(std::fs::read_to_string(&target).unwrap_or_default() == written, "the committed reopen replies are the commands' replies (NATIVE_REOPEN_REPLIES_WRITE=1 rewrites them)");
    std::fs::remove_dir_all(&dir).unwrap();
    assert_eq!(registry.pool().reserved_total(), 0);
}
