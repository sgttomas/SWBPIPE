//! The received result's text as the webview writes it (`result_js_text`),
//! against the unchanged TypeScript paths' texts
//! (`apps/desktop/src/test-support/analysis-run-v03/persistence_forms.json`,
//! written by `persistenceFormsCases.ts` and byte-checked by
//! `persistenceForms.test.ts`): every edge text whole, and every preview
//! case's held text by its SHA-256 and length, for both negative-zero forms;
//! and Save Project's stored result form, `serde_json::to_string` of the value
//! serde_json reads from `JSON.stringify`'s text, is the builder's stored
//! writer over the held value.
use super::*;
use crate::analysis_run_v03_build::{Style, Writer};
use sha2::{Digest, Sha256};

const FORMS: &str = "apps/desktop/src/test-support/analysis-run-v03/persistence_forms.json";
const CASES: &str = "apps/desktop/src/test-support/analysis-run-v03/analysis_run_v03_cases.json";

fn root() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..")
}
fn read_text(path: &str) -> String {
    std::fs::read_to_string(root().join(path)).unwrap_or_else(|e| panic!("{path}: {e}"))
}
pub(crate) fn forms() -> Value {
    serde_json::from_str(&read_text(FORMS)).unwrap()
}
/// A case's held text (as `analysis_run_v03_build_tests`' `source_text`).
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
/// The preview cases of the builder's corpus by id.
pub(crate) fn preview_cases() -> std::collections::BTreeMap<String, Value> {
    let cases: Value = serde_json::from_str(&read_text(CASES)).unwrap();
    cases["cases"].as_array().unwrap().iter().filter(|case| case["kind"] == "preview").map(|case| (case["id"].as_str().unwrap().to_string(), case.clone())).collect()
}

fn written(text: &str, zero: NegativeZero) -> String {
    let mut out = Vec::new();
    write(text, zero, &mut out).unwrap();
    String::from_utf8(out).unwrap()
}

fn digest(text: &str) -> Value {
    let sha: String = Sha256::digest(text.as_bytes()).iter().map(|byte| format!("{byte:02x}")).collect();
    serde_json::json!({"sha256": sha, "length": text.len()})
}

#[test]
fn the_edge_texts_are_written_as_javascript_writes_them() {
    let forms = forms();
    let edges = forms["edges"].as_array().unwrap();
    assert!(edges.len() >= 10);
    for edge in edges {
        let text = edge["text"].as_str().unwrap();
        for (zero, key) in [(NegativeZero::Keep, "js_text"), (NegativeZero::AsZero, "stringify_text")] {
            let expected = edge[key].as_str().unwrap();
            assert_eq!(written(text, zero), expected, "{key} of {text}");
            assert_eq!(length(text, zero).unwrap(), expected.len() as u64, "{key} length of {text}");
        }
        // Save Project's stored form: serde_json's text of its reading of `JSON.stringify`'s,
        // which the builder's stored writer writes from the held value, where serde_json reads both.
        if let (Ok(held), Ok(sent)) = (serde_json::from_str::<Value>(text), serde_json::from_str::<Value>(edge["stringify_text"].as_str().unwrap())) {
            let mut stored = Vec::new();
            Writer::new(&mut stored, Style::Stored).value(&held);
            assert_eq!(String::from_utf8(stored).unwrap(), serde_json::to_string(&sent).unwrap(), "stored form of {text}");
        }
    }
}

#[test]
fn every_preview_cases_held_text_is_written_as_the_webview_writes_it() {
    let forms = forms();
    let cases = preview_cases();
    let listed = forms["cases"].as_array().unwrap();
    assert!(listed.len() >= 5);
    for case in listed {
        let id = case["id"].as_str().unwrap();
        let text = source_text(&cases[id]["source"]);
        let js = written(&text, NegativeZero::Keep);
        assert_eq!(digest(&js), case["js_text"], "{id}: Record Run's result text");
        assert_eq!(length(&text, NegativeZero::Keep).unwrap(), js.len() as u64, "{id}");
        let stringified = written(&text, NegativeZero::AsZero);
        assert_eq!(digest(&stringified), case["stringify_text"], "{id}: Save Project's sent result text");
        let mut stored = Vec::new();
        Writer::new(&mut stored, Style::Stored).value(&serde_json::from_str::<Value>(&text).unwrap());
        assert_eq!(String::from_utf8(stored).unwrap(), serde_json::to_string(&serde_json::from_str::<Value>(&stringified).unwrap()).unwrap(), "{id}: Save Project's stored result");
    }
}

#[test]
fn a_text_that_is_not_json_is_refused_by_name() {
    for text in ["{", "[1,", "{\"a\" 1}", "tru", "1 2", "\"\\ud800\""] {
        assert!(length(text, NegativeZero::Keep).unwrap_err().starts_with("RESULT-JS-TEXT-INVALID"), "{text}");
    }
}

#[test]
fn the_scratch_covers_one_objects_members_per_level() {
    let wide: Value = serde_json::from_str(&format!("{{{}}}", (0..64).map(|i| format!("\"k{i}\":{{\"x\":1}}")).collect::<Vec<_>>().join(","))).unwrap();
    let narrow: Value = serde_json::from_str("{\"a\":{\"b\":{\"c\":1}}}").unwrap();
    assert!(scratch_bytes(&wide) > 64 * std::mem::size_of::<Member>() as u64);
    assert!(scratch_bytes(&narrow) < 4096);
    let text = serde_json::to_string(&wide).unwrap();
    let ((), measured) = crate::ingress::counting::measure(|| {
        length(&text, NegativeZero::Keep).unwrap();
    });
    assert!(measured.peak_priced <= scratch_bytes(&wide) + 4096, "{measured:?} over {}", scratch_bytes(&wide));
}
