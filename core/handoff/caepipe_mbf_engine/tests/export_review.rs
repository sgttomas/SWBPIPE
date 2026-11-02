//! The bounded export review (DEL-17-04 Design `desktop-mbf-export-import.md` §3.3 and §3.9.5; slice 4b.1): its
//! classification of the writer's result, its bound R on every shape, status first, the truncation counts, the
//! pairing of a diagnostic with its loss entry, long strings, the output-ceiling flag and determinism. The review is
//! read back by the engine's own JSON reader. The desktop's counting-allocator check of the long-string
//! representation is in `apps/desktop/src-tauri/src/mbf_export.rs`.

mod common;

use open_pipe_stress_caepipe_mbf::{
    self as mbf, export_review, Dict, Review, ReviewFiles, ReviewList, Value, WriterResult, MBF_REVIEW_MAX_BYTES,
    REVIEW_FIELD_MAX_BYTES,
};

const EXPORT_MODEL: &str = include_str!("../../../../fixtures/caepipe_mbf/invented/desktop_export/export_model.json");

fn settings() -> mbf::WriterSettings {
    mbf::export_settings_from_json(br#"{"title":"Invented MBF export","referenceTemperature":{"value":20,"unit":"degC"},"maxLayoutRows":null}"#).unwrap()
}

fn render(text: &str) -> WriterResult {
    render_with(text, &settings())
}

fn render_with(text: &str, settings: &mbf::WriterSettings) -> WriterResult {
    let model = mbf::parse_sidecar_json(text.as_bytes()).unwrap();
    mbf::render_si(&model, settings).unwrap()
}

/// The review of `result` without files and with the output within the ceiling.
fn again_text(result: &WriterResult) -> String {
    export_review(result, None, MBF_REVIEW_MAX_BYTES, |_| true).unwrap().text
}

/// The review of a writer result with its files' real digests, within `allowance`, the output within the ceiling.
fn reviewed(result: &WriterResult, allowance: usize) -> Review {
    let (mbf_sha, sidecar, sidecar_sha) = match (&result.status[..], &result.text) {
        ("written", Some(text)) => {
            let mbf_sha = common::sha(text.as_bytes());
            let sidecar = result.sidecar_bytes(&mbf_sha).unwrap();
            let sidecar_sha = common::sha(&sidecar);
            (mbf_sha, sidecar, sidecar_sha)
        }
        _ => (String::new(), Vec::new(), String::new()),
    };
    let files = (!mbf_sha.is_empty()).then(|| ReviewFiles {
        mbf_sha256: &mbf_sha,
        sidecar_sha256: &sidecar_sha,
        mbf_bytes: result.text.as_ref().unwrap().len() as u64,
        sidecar_bytes: sidecar.len() as u64,
    });
    let review = export_review(result, files, allowance, |_| true).unwrap();
    check(&review, allowance);
    review
}

fn parsed(review: &Review) -> Value {
    mbf::parse_sidecar_json(review.text.as_bytes()).expect("the review is JSON the engine reads")
}

fn int(value: &Value) -> usize {
    match value {
        Value::Int(number) => number.to_string().parse().unwrap(),
        other => panic!("not an int: {other:?}"),
    }
}

/// Every invariant of a formed review: within its allowance, ASCII, JSON, its counts agreeing with its entries and
/// its summary, status first.
fn check(review: &Review, allowance: usize) {
    assert!(review.text.len() <= allowance, "{} > {allowance}", review.text.len());
    assert!(review.text.bytes().all(|byte| (b' '..=b'~').contains(&byte)), "printable ASCII");
    let value = parsed(review);
    let summary = &review.summary;
    assert_eq!(value.get_or_null("document_kind").as_str(), Some("swbpipe.mbf_export_review"));
    assert_eq!(value.get_or_null("blocking").as_bool(), Some(summary.blocking));
    assert_eq!(value.get_or_null("refused_unexplained").as_bool(), Some(summary.refused_unexplained));
    assert_eq!(value.get_or_null("completeness").as_str(), Some(if summary.complete { "complete" } else { "truncated" }));
    assert_eq!(value.get_or_null("output_ceiling").as_str(), Some(if summary.output_within_ceiling { "within" } else { "beyond" }));
    assert_eq!(int(value.get_or_null("field_max_bytes")), REVIEW_FIELD_MAX_BYTES);
    let entries = value.get_or_null("entries").as_list().unwrap();
    let materials = value.get_or_null("undeclared_materials").as_list().unwrap();
    for (index, list) in ReviewList::ALL.iter().enumerate() {
        let counts = value.get_or_null("lists").get_or_null(list.name());
        let shown = if *list == ReviewList::UndeclaredMaterials {
            materials.len()
        } else {
            entries.iter().filter(|entry| entry.get_or_null("list").as_str() == Some(list.name())).count()
        };
        assert_eq!(int(counts.get_or_null("listed")), shown, "{}", list.name());
        assert_eq!(int(counts.get_or_null("listed")), summary.listed[index]);
        assert_eq!(int(counts.get_or_null("total")), summary.totals[index]);
        assert_eq!(int(counts.get_or_null("not_shown")), summary.totals[index] - summary.listed[index]);
        assert_eq!(int(counts.get_or_null("not_shown_blocking")), if list.blocks() { summary.totals[index] - summary.listed[index] } else { 0 });
    }
    assert_eq!(int(value.get_or_null("not_shown")), summary.not_shown());
    assert_eq!(int(value.get_or_null("not_shown_blocking")), summary.not_shown_blocking());
    assert_eq!(summary.complete, summary.not_shown() == 0);
    // The lists are listed in order, each entry once: the listed entries are a prefix of the listing order.
    let order: Vec<&str> = entries.iter().map(|entry| entry.get_or_null("list").as_str().unwrap()).collect();
    let mut sorted = order.clone();
    sorted.sort_by_key(|name| ReviewList::ALL.iter().position(|list| list.name() == *name).unwrap());
    assert_eq!(order, sorted, "lists in order");
    // Once an entry is counted, no later one is listed.
    let first_unlisted = ReviewList::ALL.iter().position(|list| summary.listed[*list as usize] < summary.totals[*list as usize]);
    if let Some(first) = first_unlisted {
        for later in ReviewList::ALL.iter().skip(first + 1) {
            assert_eq!(summary.listed[*later as usize], 0, "{} listed after an earlier list was cut", later.name());
        }
    }
}

fn entries_of<'v>(value: &'v Value, list: &str) -> Vec<(&'v str, &'v Value, &'v str)> {
    value
        .get_or_null("entries")
        .as_list()
        .unwrap()
        .iter()
        .filter(|entry| entry.get_or_null("list").as_str() == Some(list))
        .map(|entry| (entry.get_or_null("code").as_str().unwrap_or(""), entry.get_or_null("entity"), entry.get_or_null("category").as_str().unwrap()))
        .collect()
}

fn diag(code: &str, severity: &str, entity: &str, message: &str) -> Value {
    let mut dict = Dict::new();
    for (key, value) in [("code", code), ("severity", severity), ("entity", entity), ("message", message)] {
        dict.insert(key.to_string(), Value::str(value));
    }
    Value::Dict(dict)
}

fn loss(code: &str, category: &str, entity: &str, reason: &str) -> Value {
    let mut dict = Dict::new();
    for (key, value) in [("entity", entity), ("category", category), ("code", code), ("reason", reason)] {
        dict.insert(key.to_string(), Value::str(value));
    }
    Value::Dict(dict)
}

/// A writer result of the given status with no maps, for synthetic shapes.
fn synthetic(status: &str) -> WriterResult {
    let mut result = render(EXPORT_MODEL);
    result.status = status.to_string();
    result.text = None;
    result.diagnostics.clear();
    result.loss_report.clear();
    result.material_map = Dict::new();
    result.unit_normalizations.clear();
    result
}

#[test]
fn the_fixtures_review_is_complete_written_and_lists_its_omissions_without_messages() {
    let result = render(EXPORT_MODEL);
    assert_eq!(result.status, "written");
    let review = reviewed(&result, MBF_REVIEW_MAX_BYTES);
    let summary = &review.summary;
    assert!(summary.written && !summary.blocking && summary.complete && summary.output_within_ceiling && summary.save_eligible());
    let value = parsed(&review);
    assert_eq!(value.get_or_null("status").as_str(), Some("written"));
    assert_eq!(value.get_or_null("mbf_sha256").as_str(), Some(common::sha(result.text.as_ref().unwrap().as_bytes()).as_str()));
    assert!(entries_of(&value, "not_written").is_empty());
    let omitted = entries_of(&value, "omitted");
    let expected: Vec<(&str, &str)> = result
        .loss_report
        .iter()
        .filter(|item| item.get_or_null("category").as_str() == Some("omitted"))
        .map(|item| (item.get_or_null("code").as_str().unwrap(), item.get_or_null("entity").as_str().unwrap()))
        .collect();
    assert!(!expected.is_empty());
    assert_eq!(omitted.iter().map(|(code, entity, _)| (*code, entity.as_str().unwrap())).collect::<Vec<_>>(), expected);
    // No engine message, reason or assumption text is carried.
    for item in result.diagnostics.iter().chain(&result.loss_report) {
        for key in ["message", "reason"] {
            if let Some(text) = item.get_or_null(key).as_str().filter(|text| text.len() > 12) {
                assert!(!review.text.contains(text), "{text}");
            }
        }
    }
    for sentence in &result.assumptions {
        assert!(!review.text.contains(sentence.as_str()), "{sentence}");
    }
    assert!(!review.text.to_lowercase().contains("caepipe"));
}

#[test]
fn a_refused_model_is_blocking_with_its_refusals_and_its_unpaired_errors() {
    let result = render(&EXPORT_MODEL.replacen("\"kind\": \"bend\"", "\"kind\": \"tee\"", 1));
    assert_eq!(result.status, "refused");
    let review = reviewed(&result, MBF_REVIEW_MAX_BYTES);
    assert!(review.summary.blocking && !review.summary.written && !review.summary.refused_unexplained && !review.summary.save_eligible());
    let value = parsed(&review);
    assert!(value.get_or_null("mbf_sha256").is_null());
    let blocked = entries_of(&value, "not_written");
    assert!(blocked.iter().any(|(code, entity, category)| *code == "MBF-SI-COMPONENT-UNSUPPORTED"
        && entity.as_str() == Some("component:component:bend-1")
        && *category == "unsupported"));
    // An error with no loss entry of its own: the last node beyond any finite value in mm.
    let far = EXPORT_MODEL.replacen("\"z\": 6.0", "\"z\": 1e306", 1);
    let result = render(&far);
    let review = reviewed(&result, MBF_REVIEW_MAX_BYTES);
    let value = parsed(&review);
    let blocked = entries_of(&value, "not_written");
    assert!(blocked.iter().any(|(code, _, category)| *code == "MBF-SI-VALUE-OUT-OF-RANGE" && *category == "error"), "{blocked:?}");
    assert!(review.summary.blocking);
}

#[test]
fn a_refusal_that_names_nothing_is_a_global_refusal() {
    let mut result = synthetic("refused");
    result.diagnostics.push(diag("MBF-SI-PIPING-CODE-NOT-WRITTEN", "info", "model", "a note"));
    let review = reviewed(&result, MBF_REVIEW_MAX_BYTES);
    assert!(review.summary.blocking && review.summary.refused_unexplained && review.summary.complete);
    assert_eq!(parsed(&review).get_or_null("refused_unexplained").as_bool(), Some(true));
}

/// §3.3's classification: unsupported entries, then unpaired errors, then omitted entries, then warnings without an
/// omitted entry of their own; info diagnostics are not carried.
#[test]
fn entries_follow_the_classification_and_a_diagnostic_travels_with_its_loss_entry() {
    let mut result = synthetic("refused");
    result.diagnostics = vec![
        diag("MBF-SI-LOAD-UNSUPPORTED", "error", "load:a/1", "m"),
        diag("MBF-SI-FIELD-NOT-EXPORTED", "warning", "node:n", "m"),
        diag("MBF-SI-INPUT-INVALID", "error", "model.nodes", "m"),
        diag("MBF-SI-SHEAR-MODULUS-NOT-EXPORTED", "warning", "material:m", "m"),
        diag("MBF-SI-UNITS-NORMALIZED", "info", "project.units", "m"),
    ];
    result.loss_report = vec![
        loss("MBF-SI-LOAD-UNSUPPORTED", "unsupported", "load:a/1", "r"),
        loss("MBF-SI-FIELD-NOT-EXPORTED", "omitted", "node:n", "r"),
    ];
    let value = parsed(&reviewed(&result, MBF_REVIEW_MAX_BYTES));
    let listed: Vec<(String, String, String, String)> = value
        .get_or_null("entries")
        .as_list()
        .unwrap()
        .iter()
        .map(|entry| ["list", "code", "entity", "category"].map(|key| entry.get_or_null(key).as_str().unwrap().to_string()).into())
        .map(|[a, b, c, d]: [String; 4]| (a, b, c, d))
        .collect();
    let expected = [
        ("not_written", "MBF-SI-LOAD-UNSUPPORTED", "load:a/1", "unsupported"),
        ("not_written", "MBF-SI-INPUT-INVALID", "model.nodes", "error"),
        ("omitted", "MBF-SI-FIELD-NOT-EXPORTED", "node:n", "omitted"),
        ("warnings", "MBF-SI-SHEAR-MODULUS-NOT-EXPORTED", "material:m", "warning"),
    ];
    assert_eq!(listed, expected.map(|(a, b, c, d)| (a.to_string(), b.to_string(), c.to_string(), d.to_string())));
    // Refused pairs cut part-way: each pair is one entry, listed or counted, never split.
    let mut result = synthetic("refused");
    for index in 0..400 {
        let entity = format!("load:case/{index}");
        result.diagnostics.push(diag("MBF-SI-LOAD-UNSUPPORTED", "error", &entity, "message"));
        result.loss_report.push(loss("MBF-SI-LOAD-UNSUPPORTED", "unsupported", &entity, "reason"));
    }
    let review = reviewed(&result, 8 * 1024);
    assert!(!review.summary.complete);
    assert_eq!(review.summary.totals[0], 400, "one entry per pair");
    let value = parsed(&review);
    let blocked = entries_of(&value, "not_written");
    assert!(blocked.iter().all(|(_, _, category)| *category == "unsupported"), "no paired error listed alone");
    assert_eq!(blocked.len() + review.summary.not_shown(), 400);
    assert_eq!(review.summary.not_shown_blocking(), 400 - blocked.len());
}

/// A truncated review of a written model: not blocking, but not save-eligible, and its counts are exact.
#[test]
fn a_truncated_review_of_a_written_model_is_not_blocking_and_not_eligible() {
    let result = render(EXPORT_MODEL);
    let full = reviewed(&result, MBF_REVIEW_MAX_BYTES);
    let omitted = full.summary.totals[ReviewList::Omitted as usize];
    assert!(omitted >= 2);
    // An allowance one byte short of the full review lists every entry but the last.
    let short = reviewed(&result, full.text.len() - 1);
    assert!(!short.summary.complete && !short.summary.blocking && short.summary.written && !short.summary.save_eligible());
    assert!(short.summary.not_shown() >= 1);
    assert_eq!(short.summary.not_shown_blocking(), 0);
    assert_eq!(short.summary.totals, full.summary.totals);
    // Down to the fixed fields alone: everything counted.
    let mut allowance = full.text.len();
    let mut last_listed = usize::MAX;
    while let Ok(cut) = export_review(&result, None, allowance, |_| true) {
        check(&cut, allowance);
        let listed: usize = cut.summary.listed.iter().sum();
        assert!(listed <= last_listed);
        last_listed = listed;
        allowance -= 97;
    }
    assert_eq!(last_listed, 0);
}

/// Status first: a blocked model whose entries far exceed R is still blocked, and its counts are exact.
#[test]
fn a_truncated_review_of_a_blocked_model_keeps_its_status_and_exact_counts() {
    let mut result = synthetic("refused");
    let id = "x".repeat(300);
    for index in 0..20_000 {
        let entity = format!("load:{id}/{index}");
        result.diagnostics.push(diag("MBF-SI-LOAD-UNSUPPORTED", "error", &entity, "message"));
        result.loss_report.push(loss("MBF-SI-LOAD-UNSUPPORTED", "unsupported", &entity, "reason"));
        result.diagnostics.push(diag("MBF-SI-FIELD-NOT-EXPORTED", "warning", &entity, "message"));
        result.loss_report.push(loss("MBF-SI-FIELD-NOT-EXPORTED", "omitted", &entity, "reason"));
    }
    let review = reviewed(&result, MBF_REVIEW_MAX_BYTES);
    let summary = &review.summary;
    assert!(summary.blocking && !summary.complete && !summary.refused_unexplained);
    assert_eq!(summary.totals, [20_000, 20_000, 0, 0]);
    assert_eq!(summary.listed[1], 0, "no omitted entry listed after the not-written list was cut");
    assert_eq!(summary.not_shown(), 40_000 - summary.listed[0]);
    assert_eq!(summary.not_shown_blocking(), 20_000 - summary.listed[0]);
    assert!(review.text.len() > MBF_REVIEW_MAX_BYTES - 1024, "the allowance is used");
}

/// At most R on extreme shapes: long ids, control characters escaped six to one, astral characters, many materials,
/// many conversions.
#[test]
fn every_shape_stays_within_r() {
    let shapes: Vec<(&str, Box<dyn Fn(usize) -> String>)> = vec![
        ("short", Box::new(|index| format!("node:{index}"))),
        ("at the bound", Box::new(|index| format!("{:0>width$}", index, width = REVIEW_FIELD_MAX_BYTES))),
        ("long", Box::new(|index| format!("load:{}/{index}", "c".repeat(16 * 1024)))),
        ("controls", Box::new(|index| format!("{}{index}", "\u{1}".repeat(REVIEW_FIELD_MAX_BYTES - 8)))),
        ("astral", Box::new(|index| format!("{}{index}", "😀".repeat(REVIEW_FIELD_MAX_BYTES / 4 - 2)))),
    ];
    for (name, entity) in shapes {
        let mut result = synthetic("written");
        for index in 0..3000 {
            result.loss_report.push(loss(&entity(index), "omitted", &entity(index), "reason"));
            result.diagnostics.push(diag(&entity(index), "warning", &entity(index + 1), "message"));
            let mut material = Dict::new();
            material.insert("swbpipe_material".into(), Value::str(entity(index)));
            material.insert("origin".into(), Value::str("undeclared"));
            result.material_map.insert(format!("M{index}"), Value::Dict(material));
        }
        result.unit_normalizations = (0..9).map(|index| (entity(index), entity(index + 9))).collect();
        let files = ReviewFiles { mbf_sha256: &"a".repeat(64), sidecar_sha256: &"b".repeat(64), mbf_bytes: u64::MAX, sidecar_bytes: u64::MAX };
        for allowance in [MBF_REVIEW_MAX_BYTES, 64 * 1024] {
            let review = export_review(&result, Some(files), allowance, |_| true).unwrap();
            check(&review, allowance);
            assert_eq!(review.summary.totals, [0, 3000, 3000, 3000], "{name}");
        }
    }
}

#[test]
fn a_long_string_is_carried_as_its_prefix_length_and_digest() {
    let mut result = synthetic("written");
    // A character straddles the bound, so the cut moves back to its start.
    let entity = format!("load:{}é{}", "c".repeat(REVIEW_FIELD_MAX_BYTES - 6), "d".repeat(20_000));
    result.loss_report.push(loss("MBF-SI-FIELD-NOT-EXPORTED", "omitted", &entity, "reason"));
    let review = reviewed(&result, MBF_REVIEW_MAX_BYTES);
    let value = parsed(&review);
    let carried = entries_of(&value, "omitted")[0].1;
    let prefix = carried.get_or_null("prefix").as_str().unwrap();
    assert_eq!(prefix, &entity[..REVIEW_FIELD_MAX_BYTES - 1]);
    assert!(entity.starts_with(prefix) && prefix.len() <= REVIEW_FIELD_MAX_BYTES);
    assert_eq!(int(carried.get_or_null("bytes")), entity.len());
    assert_eq!(carried.get_or_null("sha256").as_str(), Some(common::sha(entity.as_bytes()).as_str()));
    assert!(review.text.len() < 2 * REVIEW_FIELD_MAX_BYTES + 1024);
    // At the bound a string is carried as itself.
    let mut result = synthetic("written");
    let exact = "e".repeat(REVIEW_FIELD_MAX_BYTES);
    result.loss_report.push(loss("MBF-SI-FIELD-NOT-EXPORTED", "omitted", &exact, "reason"));
    let value = parsed(&reviewed(&result, MBF_REVIEW_MAX_BYTES));
    assert_eq!(entries_of(&value, "omitted")[0].1.as_str(), Some(exact.as_str()));
}

/// The output-ceiling flag is the caller's decision on the review's exact length, which does not depend on it.
#[test]
fn the_output_ceiling_flag_is_decided_on_the_exact_length() {
    let result = render(EXPORT_MODEL);
    let within = reviewed(&result, MBF_REVIEW_MAX_BYTES);
    let mut seen = 0;
    let exceeds = export_review(&result, None, MBF_REVIEW_MAX_BYTES, |length| {
        seen = length;
        false
    })
    .unwrap();
    check(&exceeds, MBF_REVIEW_MAX_BYTES);
    assert_eq!(seen as usize, exceeds.text.len());
    assert!(!exceeds.summary.output_within_ceiling && !exceeds.summary.save_eligible());
    assert!(exceeds.text.contains("\"output_ceiling\":\"beyond\""));
    assert_eq!(exceeds.text.replace("\"output_ceiling\":\"beyond\"", "\"output_ceiling\":\"within\""), again_text(&result));
    // Without files the review is not save-eligible either; with them, the length differs only by the digests.
    let again = export_review(&result, None, MBF_REVIEW_MAX_BYTES, |_| true).unwrap();
    assert_eq!(again.text.len(), exceeds.text.len());
    assert!(!again.summary.written);
    assert!(within.summary.save_eligible());
}

#[test]
fn the_review_is_deterministic_and_states_its_unit_conversions() {
    let degf = EXPORT_MODEL.replacen("\"temperature\": \"degC\"", "\"temperature\": \"degF\"", 1);
    let settings = mbf::export_settings_from_json(br#"{"title":"T","referenceTemperature":{"value":68,"unit":"degF"},"maxLayoutRows":null}"#).unwrap();
    let result = render_with(&degf, &settings);
    let first = reviewed(&result, MBF_REVIEW_MAX_BYTES);
    let second = reviewed(&render_with(&degf, &settings), MBF_REVIEW_MAX_BYTES);
    assert_eq!(first.text, second.text);
    let value = parsed(&first);
    let conversions = value.get_or_null("unit_conversions").as_list().unwrap();
    assert_eq!(conversions.len(), result.unit_normalizations.len());
    assert!(!conversions.is_empty());
    for (conversion, (unit, definition)) in conversions.iter().zip(&result.unit_normalizations) {
        assert_eq!(conversion.get_or_null("unit").as_str(), Some(unit.as_str()));
        assert_eq!(conversion.get_or_null("definition").as_str(), Some(definition.as_str()));
    }
    assert!(conversions.iter().any(|item| item.get_or_null("definition").as_str() == Some("T(degC) = (T(degF) - 32) x 5/9")
        || item.get_or_null("unit").as_str() == Some("degF absolute")));
}

#[test]
fn undeclared_materials_are_named() {
    let mut result = synthetic("written");
    for (key, origin) in [("M1", "undeclared"), ("M2", "project_invented"), ("M3", "undeclared")] {
        let mut material = Dict::new();
        material.insert("swbpipe_material".into(), Value::str(format!("material:{key}")));
        material.insert("origin".into(), Value::str(origin));
        result.material_map.insert(key.into(), Value::Dict(material));
    }
    let value = parsed(&reviewed(&result, MBF_REVIEW_MAX_BYTES));
    let named: Vec<&str> = value.get_or_null("undeclared_materials").as_list().unwrap().iter().map(|item| item.as_str().unwrap()).collect();
    assert_eq!(named, vec!["material:M1", "material:M3"]);
}

#[test]
fn an_allowance_below_the_fixed_fields_is_refused() {
    let result = render(EXPORT_MODEL);
    let error = export_review(&result, None, 200, |_| true).unwrap_err();
    assert!(error.needed > 200 && error.allowance == 200);
}
