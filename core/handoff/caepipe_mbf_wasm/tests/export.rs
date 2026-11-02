//! The webview's MBF writer and importer over the invented desktop export fixture (DEL-17-04 Design
//! `desktop-mbf-export-import.md` §3, slice 4a), and the recorded digests the native save checks its own against
//! (`fixtures/caepipe_mbf/invented/desktop_export/export_digests.json`; regenerate with
//! `MBF_EXPORT_DIGESTS_REGENERATE=1`).

use open_pipe_stress_caepipe_mbf_wasm::{
    export_settings, import_mbf_json, render_export, render_mbf_json, sha256_hex, ENGINE_FAILED,
    IMPORT_OPTIONS_INVALID, MODEL_JSON_INVALID, SETTINGS_INVALID,
};
use serde_json::{json, Value};
use std::path::{Path, PathBuf};

fn project() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..")
}

fn fixture(name: &str) -> PathBuf {
    project().join("fixtures/caepipe_mbf/invented/desktop_export").join(name)
}

/// The fixture as the desktop sends it: its RFC 8785 canonical text.
fn canonical(path: &Path) -> String {
    let value: Value = serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap();
    open_pipe_stress_canonical_json::canonical_json(&value)
}

fn export_model() -> String {
    canonical(&fixture("export_model.json"))
}

fn settings(title: &str, value: Value, unit: &str) -> String {
    settings_text(title, &value.to_string(), unit)
}

/// The settings text as the dialog writes it (`JSON.stringify` of `{title, referenceTemperature: {value, unit},
/// maxLayoutRows: null}`, in that member order), with the number's text as given.
fn settings_text(title: &str, value: &str, unit: &str) -> String {
    format!(
        r#"{{"title":{},"referenceTemperature":{{"value":{value},"unit":"{unit}"}},"maxLayoutRows":null}}"#,
        Value::String(title.to_string())
    )
}

fn render(model: &str, settings: &str) -> Value {
    serde_json::from_str(&render_mbf_json(model, settings)).unwrap()
}

/// The digest cases the native save repeats: (name, settings, model edit).
fn cases() -> Vec<(&'static str, String, String)> {
    let model = export_model();
    let mut degf: Value = serde_json::from_str(&model).unwrap();
    degf["project"]["units"]["temperature"] = json!("degF");
    vec![
        ("degc-default", settings("Invented MBF export", json!(20), "degC"), model.clone()),
        ("degc-fractional", settings("Invented MBF export", json!(21.5), "degC"), model.clone()),
        ("degf-project", settings("Invented MBF export", json!(68), "degF"), open_pipe_stress_canonical_json::canonical_json(&degf)),
        // Integers the dialog's JSON.stringify writes in full: one between 2^63 and 2^64, one between 2^64 and 10^21.
        // The native save is given the same text and reads it by the same engine function.
        ("degc-integer-above-2p63", settings_text("Invented MBF export", "9223372036854776000", "degC"), model.clone()),
        ("degf-integer-above-2p64", settings_text("Invented MBF export", "100000000000000000000", "degF"), model.clone()),
    ]
}

#[test]
fn the_fixture_is_written_with_its_identity_file_and_digests() {
    let model = export_model();
    let outcome = render(&model, &settings("Invented MBF export", json!(20), "degC"));
    assert_eq!(outcome["document_kind"], "swbpipe.desktop.mbf_render_outcome");
    assert_eq!(outcome["status"], "written", "{outcome}");
    assert_eq!(outcome["profile_id"], "caepipe-mbf-si-pilot-1");
    assert!(outcome["failure"].is_null());
    let text = outcome["mbf_text"].as_str().unwrap();
    let sidecar = outcome["sidecar_text"].as_str().unwrap();
    assert_eq!(outcome["mbf_sha256"], sha256_hex(text.as_bytes()));
    assert_eq!(outcome["sidecar_sha256"], sha256_hex(sidecar.as_bytes()));
    // The identity file is the writer's format-4 sidecar for exactly this text.
    let parsed: Value = serde_json::from_str(sidecar).unwrap();
    assert_eq!(parsed["sidecar_format"], 4);
    assert_eq!(parsed["mbf_sha256"], outcome["mbf_sha256"]);
    assert_eq!(parsed["section_map"]["P1"]["swbpipe_section"], "section:invented-a");
    assert_eq!(parsed["material_map"]["M1"]["origin"], "project_invented");
    // The review is the writer's to_dict: written, omissions only, the text it reports.
    let report: Value = serde_json::from_str(outcome["report_json"].as_str().unwrap()).unwrap();
    assert_eq!(report["status"], "written");
    assert_eq!(report["text"], text);
    let categories: Vec<&str> = report["loss_report"].as_array().unwrap().iter().map(|item| item["category"].as_str().unwrap()).collect();
    assert!(!categories.is_empty() && categories.iter().all(|category| *category == "omitted"), "{categories:?}");
    // The rlib path the outcome is built from gives the same bytes.
    let direct = render_export(model.as_bytes(), &export_settings(settings("Invented MBF export", json!(20), "degC").as_bytes()).unwrap()).unwrap();
    let written = direct.written.unwrap();
    assert_eq!(written.sidecar, sidecar.as_bytes());
    assert_eq!(direct.result.text.as_deref(), Some(text));
}

#[test]
fn the_written_pair_imports_bound_with_the_same_identities() {
    let model = export_model();
    let outcome = render(&model, &settings("Invented MBF export", json!(20), "degC"));
    let text = outcome["mbf_text"].as_str().unwrap();
    let sidecar = outcome["sidecar_text"].as_str().unwrap().as_bytes().to_vec();
    let imported: Value = serde_json::from_str(&import_mbf_json(text.as_bytes(), Some(sidecar), r#"{"sourceLabel":"swbpipe-model.mbf"}"#)).unwrap();
    assert_eq!(imported["status"], "imported", "{imported}");
    let report: Value = serde_json::from_str(imported["report_json"].as_str().unwrap()).unwrap();
    assert!(report["diagnostics"].as_array().unwrap().iter().all(|item| item["severity"] != "error"), "{report}");
    let source: Value = serde_json::from_str(&model).unwrap();
    let back: Value = serde_json::from_str(imported["model_json"].as_str().unwrap()).unwrap();
    for member in ["nodes", "pipe_segments", "supports", "load_cases", "materials", "sections"] {
        let ids = |document: &Value| {
            let mut ids: Vec<String> = document[member].as_array().unwrap().iter().map(|item| item["id"].as_str().unwrap().to_string()).collect();
            ids.sort();
            ids
        };
        assert_eq!(ids(&back), ids(&source), "{member}");
    }
    // Without the identity file the import is unbound and forms ids from node numbers.
    let unbound: Value = serde_json::from_str(&import_mbf_json(text.as_bytes(), None, r#"{"sourceLabel":"swbpipe-model.mbf","materialProvenance":null}"#)).unwrap();
    assert_ne!(unbound["status"], "failed", "{unbound}");
    if let Some(model_json) = unbound["model_json"].as_str() {
        let back: Value = serde_json::from_str(model_json).unwrap();
        assert_ne!(back["nodes"], source["nodes"]);
    }
}

#[test]
fn a_degf_project_writes_its_reference_temperature_exactly() {
    let model = export_model();
    let celsius = render(&model, &settings("T", json!(20), "degC"));
    let fahrenheit = render(&model, &settings("T", json!(68), "degF"));
    assert_eq!(fahrenheit["status"], "written", "{fahrenheit}");
    // 68 degF is exactly 20 degC: the same OPTEMP, so the same text.
    assert_eq!(fahrenheit["mbf_text"], celsius["mbf_text"]);
    let other = render(&model, &settings("T", json!(70), "degF"));
    assert_ne!(other["mbf_text"], celsius["mbf_text"]);
}

#[test]
fn a_model_the_writer_refuses_has_no_text_and_lists_its_refusals() {
    let every_slot = canonical(&project().join("fixtures/model_document/invented_every_slot_model.json"));
    let outcome = render(&every_slot, &settings("T", json!(20), "degC"));
    assert_eq!(outcome["status"], "refused", "{outcome}");
    for key in ["mbf_text", "mbf_sha256", "sidecar_text", "sidecar_sha256"] {
        assert!(outcome[key].is_null(), "{key}");
    }
    let report: Value = serde_json::from_str(outcome["report_json"].as_str().unwrap()).unwrap();
    assert!(report["loss_report"].as_array().unwrap().iter().any(|item| item["category"] == "unsupported"));
}

#[test]
fn settings_other_than_the_dialogs_are_refused_by_name() {
    let model = export_model();
    for bad in [
        json!({"title": "T", "referenceTemperature": {"value": 20, "unit": "degC"}, "maxLayoutRows": 20}),
        json!({"title": "T", "referenceTemperature": {"value": 20, "unit": "degC"}}),
        json!({"title": "T", "referenceTemperature": {"value": 20, "unit": "degC"}, "maxLayoutRows": null, "extra": 1}),
        json!({"title": 7, "referenceTemperature": {"value": 20, "unit": "degC"}, "maxLayoutRows": null}),
        json!({"title": "T", "referenceTemperature": {"value": 20, "unit": "K"}, "maxLayoutRows": null}),
        json!({"title": "T", "referenceTemperature": {"value": true, "unit": "degC"}, "maxLayoutRows": null}),
        json!({"title": "T", "referenceTemperature": {"value": "20", "unit": "degC"}, "maxLayoutRows": null}),
        json!({"title": "T", "referenceTemperature": {"value": 20, "unit": "degC", "x": 1}, "maxLayoutRows": null}),
        json!([]),
    ] {
        let outcome = render(&model, &bad.to_string());
        assert_eq!(outcome["status"], "failed", "{bad}");
        assert_eq!(outcome["failure"]["code"], SETTINGS_INVALID, "{bad}");
    }
    let outcome = render(&model, "{not json");
    assert_eq!(outcome["failure"]["code"], SETTINGS_INVALID);
    // The title bound: 4 KiB of UTF-8 is written, one byte more is refused before the model is read.
    assert_eq!(render(&model, &settings_text(&"é".repeat(2048), "20", "degC"))["status"], "written");
    for title in ["a".repeat(4097), "é".repeat(2049)] {
        let outcome = render("{not json", &settings_text(&title, "20", "degC"));
        assert_eq!(outcome["failure"]["code"], SETTINGS_INVALID);
        assert!(outcome["failure"]["message"].as_str().unwrap().contains("more than 4096"), "{outcome}");
    }
}

/// The unit conversions are the writer's normalization data with exact factors, not its assumption sentences.
#[test]
fn the_review_lists_each_unit_conversion_with_its_exact_factor() {
    let model = export_model();
    let outcome = render(&model, &settings("T", json!(20), "degC"));
    assert_eq!(outcome["unit_conversions"], json!([]));
    let mut us: Value = serde_json::from_str(&model).unwrap();
    us["project"]["units"]["temperature"] = json!("degF");
    let outcome = render(&open_pipe_stress_canonical_json::canonical_json(&us), &settings("T", json!(68), "degF"));
    let conversions = outcome["unit_conversions"].as_array().unwrap();
    assert!(conversions.iter().any(|item| item["unit"] == "degF absolute" && item["factor"].is_null()), "{conversions:?}");
    let report: Value = serde_json::from_str(outcome["report_json"].as_str().unwrap()).unwrap();
    for item in conversions {
        assert!(item["definition"].as_str().is_some_and(|text| !text.is_empty()));
        if item["unit"] == "degF" {
            assert_eq!(item["factor"], "5/9");
        }
    }
    assert!(report["assumptions"].as_array().unwrap().iter().all(|sentence| conversions.iter().all(|item| item["definition"] != *sentence)));
}

#[test]
fn model_text_the_engine_cannot_read_is_refused_by_name() {
    let good = settings("T", json!(20), "degC");
    for bad in ["{not json", "{\"a\":1,\"a\":2}", "\u{feff}{}"] {
        let outcome = render(bad, &good);
        assert_eq!(outcome["failure"]["code"], MODEL_JSON_INVALID, "{bad}");
    }
    let deep = format!("{}{}", "[".repeat(80), "]".repeat(80));
    assert_eq!(render(&deep, &good)["failure"]["code"], MODEL_JSON_INVALID);
    // A JSON value that is not a model is the writer's to judge (refused, or the engine's exception).
    let outcome = render("[]", &good);
    assert!(outcome["status"] == "refused" || outcome["failure"]["code"] == ENGINE_FAILED, "{outcome}");
}

#[test]
fn import_options_other_than_the_slices_are_refused_by_name() {
    for bad in ["[]", "{}", r#"{"sourceLabel":1}"#, r#"{"sourceLabel":"x","other":1}"#, "{"] {
        let outcome: Value = serde_json::from_str(&import_mbf_json(b"", None, bad)).unwrap();
        assert_eq!(outcome["failure"]["code"], IMPORT_OPTIONS_INVALID, "{bad}");
    }
}

/// The digests the native save (`mbf_export.rs`) must reproduce for the same text and settings: a check that both
/// map the dialog's settings to the same writer arguments.
#[test]
fn the_recorded_export_digests_are_the_wrappers() {
    let mut recorded = serde_json::Map::new();
    for (name, settings_text, model) in cases() {
        let outcome = render(&model, &settings_text);
        let mut case = json!({
            "model_sha256": sha256_hex(model.as_bytes()),
            "settings_json": settings_text,
            "status": outcome["status"],
        });
        if outcome["status"] == "written" {
            case["mbf_sha256"] = outcome["mbf_sha256"].clone();
            case["sidecar_sha256"] = outcome["sidecar_sha256"].clone();
        } else {
            // A refusal: the codes of the writer's error diagnostics, which the native save returns.
            assert_eq!(outcome["status"], "refused", "{name}: {outcome}");
            let report: Value = serde_json::from_str(outcome["report_json"].as_str().unwrap()).unwrap();
            let codes: Vec<Value> = report["diagnostics"].as_array().unwrap().iter().filter(|item| item["severity"] == "error").map(|item| item["code"].clone()).collect();
            assert!(!codes.is_empty(), "{name}");
            case["refusal_codes"] = Value::Array(codes);
        }
        recorded.insert(name.to_string(), case);
    }
    let document = json!({
        "document_kind": "swbpipe.test.mbf_export_digests",
        "generated_by": "core/handoff/caepipe_mbf_wasm/tests/export.rs (MBF_EXPORT_DIGESTS_REGENERATE=1)",
        "model": "fixtures/caepipe_mbf/invented/desktop_export/export_model.json, as its RFC 8785 text; degf-project with project.units.temperature degF",
        "cases": recorded,
    });
    let text = format!("{}\n", serde_json::to_string_pretty(&document).unwrap());
    let path = fixture("export_digests.json");
    if std::env::var_os("MBF_EXPORT_DIGESTS_REGENERATE").is_some() {
        std::fs::write(&path, &text).unwrap();
    }
    assert_eq!(std::fs::read_to_string(&path).unwrap_or_default(), text, "regenerate with MBF_EXPORT_DIGESTS_REGENERATE=1");
}
