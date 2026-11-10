//! The designated-source check (DEL-17-04 Design `desktop-mbf-export-import.md` §5.4) and review format 3's
//! designation: an exact source establishes the identity set; a changed source, a missing or older identity file, a
//! derivation whose settings do not reproduce the pair, and a non-model document do not; substitution, a duplicate and
//! a missing id in any class are detected class by class; and a truncated review never states an established outcome.
//! Invented data only.

mod common;

use open_pipe_stress_caepipe_mbf::{
    self as mbf, designation, evaluate_designated_source, identity_inventory, import_review, import_review_designated, ImportOptions, ImportResult,
    ImportReviewSource, ReviewDesignation, ReviewDocumentCheck, ReviewModel, ReviewedIdentityFile, SidecarInput, SourceEvaluation, Value, INVENTORY_CLASSES,
    MBF_REVIEW_MAX_BYTES,
};

const EXPORT_MODEL: &str = include_str!("../../../../fixtures/caepipe_mbf/invented/desktop_export/export_model.json");
const SETTINGS: &[u8] = br#"{"title":"Invented MBF export","referenceTemperature":{"value":20,"unit":"degC"},"maxLayoutRows":null}"#;

fn parse(bytes: &[u8]) -> Value {
    mbf::parse_sidecar_json(bytes).expect("invented JSON")
}

/// The writer's pair for `model` with the dialog's default settings.
fn pair(model: &[u8]) -> (Vec<u8>, Vec<u8>) {
    let settings = mbf::export_settings_from_json(SETTINGS).unwrap();
    let result = mbf::render_si(&parse(model), &settings).unwrap();
    assert_eq!(result.status, "written", "{:?}", result.diagnostics);
    let text = result.text.clone().unwrap().into_bytes();
    let sidecar = result.sidecar_bytes(&common::sha(&text)).unwrap();
    (text, sidecar)
}

fn import(mbf_bytes: &[u8], sidecar: Option<&[u8]>) -> ImportResult {
    let mut options = ImportOptions::new("invented.mbf");
    options.sidecar = sidecar.map(|bytes| SidecarInput::Json(bytes.to_vec()));
    mbf::import(mbf_bytes, &options).expect("an import result")
}

/// The same document, formatted differently: the same canonical form.
fn reformatted(model: &str) -> Vec<u8> {
    format!("\n\t {}\n\n", model.trim()).into_bytes()
}

fn member<'v>(value: &'v mut Value, path: &[&str]) -> &'v mut Value {
    let mut here = value;
    for step in path {
        here = here.as_dict_mut().expect("an object").get_mut(&step.to_string()).expect("a member");
    }
    here
}

fn set(value: &mut Value, path: &[&str], key: &str, item: Value) {
    member(value, path).as_dict_mut().expect("an object").insert(key.to_string(), item);
}

fn entry<'v>(value: &'v mut Value, list: &str, index: usize) -> &'v mut Value {
    let Value::List(items) = member(value, &[list]) else { panic!("a list at {list}") };
    &mut items[index]
}

/// The identity file with `edit` applied, its self-digest recomputed (a deliberate editor's file).
fn with_sidecar(sidecar: &[u8], edit: impl FnOnce(&mut Value)) -> Vec<u8> {
    let mut parsed = parse(sidecar);
    edit(&mut parsed);
    if parsed.get("sidecar_sha256").is_some() {
        let digest = mbf::sidecar_self_digest(&parsed).unwrap();
        set(&mut parsed, &[], "sidecar_sha256", Value::Str(digest));
    }
    mbf::ports::dumps_sorted(&parsed).unwrap().into_bytes()
}

fn outcome<'a>(source: &'a [u8], mbf_bytes: &[u8], sidecar: Option<&[u8]>, sha: &'a str) -> (ReviewDesignation<'a>, ImportResult) {
    let evaluation = evaluate_designated_source(source, mbf_bytes, sidecar);
    let result = import(mbf_bytes, sidecar);
    (designation("source.json", sha, source.len() as u64, &evaluation, &result), result)
}

fn review(result: &ImportResult, mbf_bytes: &[u8], designated: Option<&ReviewDesignation<'_>>, allowance: usize) -> (mbf::ImportReview, Value) {
    let model = result.model_bytes().unwrap();
    let model_sha = model.as_ref().map(|bytes| common::sha(bytes));
    let mbf_sha = common::sha(mbf_bytes);
    let source = ImportReviewSource { file_name: "invented.mbf", mbf_sha256: &mbf_sha, mbf_bytes: mbf_bytes.len() as u64, identity_file: ReviewedIdentityFile::Absent };
    let candidate = model.as_ref().zip(model_sha.as_ref()).map(|(bytes, sha256)| ReviewModel { sha256, bytes: bytes.len() as u64 });
    let check = if candidate.is_some() { ReviewDocumentCheck::Passed { schema_status: "current" } } else { ReviewDocumentCheck::NotRun };
    let review = import_review_designated(result, source, None, candidate, check, designated, allowance, |_| true).unwrap();
    let parsed = parse(review.text.as_bytes());
    (review, parsed)
}

fn strings(value: &Value) -> Vec<String> {
    value.as_list().unwrap().iter().map(|item| item.as_str().unwrap().to_string()).collect()
}

#[test]
fn the_exact_source_establishes_the_identity_set() {
    let (text, sidecar) = pair(EXPORT_MODEL.as_bytes());
    let source = reformatted(EXPORT_MODEL);
    let sha = common::sha(&source);
    let (designated, result) = outcome(&source, &text, Some(&sidecar), &sha);
    assert_eq!(designated.reasons, Vec::<&str>::new(), "{:?}", result.identity_recovery);
    assert_eq!(designated.differing, [false; 9]);
    let (review, parsed) = review(&result, &text, Some(&designated), MBF_REVIEW_MAX_BYTES);
    assert!(review.summary.complete);
    let identity = parsed.get_or_null("identity");
    assert_eq!(identity.get_or_null("source_completeness").as_str(), Some("established_for_identity_set"));
    let stated = identity.get_or_null("designation");
    assert_eq!(stated.get_or_null("outcome").as_str(), Some("established_for_identity_set"));
    assert_eq!(stated.get_or_null("file_name").as_str(), Some("source.json"));
    assert_eq!(stated.get_or_null("sha256").as_str(), Some(sha.as_str()));
    assert_eq!(mbf::value::small_int(stated.get_or_null("bytes")), Some(source.len() as i64));
    assert!(strings(stated.get_or_null("reasons")).is_empty());
    assert!(strings(stated.get_or_null("differing_classes")).is_empty());
}

#[test]
fn without_a_designation_the_review_is_format_3_with_none_stated() {
    let (text, sidecar) = pair(EXPORT_MODEL.as_bytes());
    let result = import(&text, Some(&sidecar));
    let (designated_text, parsed) = review(&result, &text, None, MBF_REVIEW_MAX_BYTES);
    assert_eq!(mbf::value::small_int(parsed.get_or_null("format")), Some(3));
    assert!(parsed.get_or_null("identity").get("designation").is_some_and(Value::is_null));
    assert_eq!(parsed.get_or_null("identity").get_or_null("source_completeness").as_str(), Some("not_established"));
    // `import_review` is `import_review_designated` without a designation, byte for byte.
    let mbf_sha = common::sha(&text);
    let model = result.model_bytes().unwrap().unwrap();
    let model_sha = common::sha(&model);
    let source = ImportReviewSource { file_name: "invented.mbf", mbf_sha256: &mbf_sha, mbf_bytes: text.len() as u64, identity_file: ReviewedIdentityFile::Absent };
    let plain = import_review(
        &result,
        source,
        None,
        Some(ReviewModel { sha256: &model_sha, bytes: model.len() as u64 }),
        ReviewDocumentCheck::Passed { schema_status: "current" },
        MBF_REVIEW_MAX_BYTES,
        |_| true,
    )
    .unwrap();
    assert_eq!(plain.text, designated_text.text);
}

#[test]
fn a_changed_source_is_not_established_and_is_not_rendered() {
    let (text, sidecar) = pair(EXPORT_MODEL.as_bytes());
    let mut value = parse(EXPORT_MODEL.as_bytes());
    set(&mut value, &["project"], "id", Value::str("project:another-invented-project"));
    let source = mbf::ports::dumps_sorted(&value).unwrap().into_bytes();
    let sha = common::sha(&source);
    let (designated, _) = outcome(&source, &text, Some(&sidecar), &sha);
    assert_eq!(designated.reasons, ["source_digest_differs"]);
}

#[test]
fn a_file_that_is_not_a_model_document_is_named() {
    let (text, sidecar) = pair(EXPORT_MODEL.as_bytes());
    for source in [&b"not json"[..], b"{\"a\":1,\"a\":2}", b"[1e400]"] {
        let sha = common::sha(source);
        let (designated, _) = outcome(source, &text, Some(&sidecar), &sha);
        assert!(designated.reasons.contains(&"source_not_a_model_document"), "{:?}", designated.reasons);
        assert!(!designated.reasons.is_empty());
    }
}

#[test]
fn an_absent_or_older_identity_file_has_no_derivation() {
    let (text, sidecar) = pair(EXPORT_MODEL.as_bytes());
    let source = EXPORT_MODEL.as_bytes();
    let sha = common::sha(source);
    let (designated, _) = outcome(source, &text, None, &sha);
    assert_eq!(designated.reasons[0], "identity_file_absent");
    let format_4 = with_sidecar(&sidecar, |value| {
        let map = value.as_dict_mut().unwrap();
        map.remove(&"derivation".to_string());
        map.insert("sidecar_format".to_string(), Value::int(4));
        for entry in member(value, &["element_map"]).as_dict_mut().unwrap().values_mut() {
            if let Some(bend) = entry.as_dict_mut().and_then(|entry| entry.get_mut(&"bend".to_string())).and_then(Value::as_dict_mut) {
                bend.remove(&"node".to_string());
            }
        }
    });
    let evaluation = evaluate_designated_source(source, &text, Some(&format_4));
    assert_eq!(evaluation.reasons, ["identity_file_not_format_6"]);
    assert_eq!(evaluation.derivation_source_sha256, None);
}

#[test]
fn a_derivation_that_does_not_reproduce_the_pair_is_named() {
    let (text, sidecar) = pair(EXPORT_MODEL.as_bytes());
    let source = EXPORT_MODEL.as_bytes();
    // Another title: the writer prints another title line, so the MBF differs.
    // The pair's own derivation reproduces it.
    assert!(evaluate_designated_source(source, &text, Some(&sidecar)).reasons.is_empty());
    // Another title: the writer prints another title line, so the MBF differs (and the identity file records it).
    let retitled = with_sidecar(&sidecar, |value| {
        set(value, &["derivation", "export_settings"], "title", Value::str("Another invented title"));
    });
    let reasons = evaluate_designated_source(source, &text, Some(&retitled)).reasons;
    assert_eq!(reasons, ["rerender_mbf_differs", "rerender_identity_file_differs"]);
    // The identity file's bytes must be the writer's exactly, whatever it parses to.
    let respaced = [&b" "[..], &sidecar].concat();
    assert_eq!(evaluate_designated_source(source, &text, Some(&respaced)).reasons, ["rerender_identity_file_differs"]);
    // A malformed derivation is no derivation.
    let malformed = with_sidecar(&sidecar, |value| {
        set(value, &["derivation", "export_settings", "reference_temperature"], "unit", Value::str("K"));
    });
    assert_eq!(evaluate_designated_source(source, &text, Some(&malformed)).reasons, ["identity_file_not_format_6"]);
}

/// The source's inventory with `edit` applied to its document, against the exact pair's import.
fn against_inventory_of(edit: impl FnOnce(&mut Value)) -> ReviewDesignation<'static> {
    let (text, sidecar) = pair(EXPORT_MODEL.as_bytes());
    let mut evaluation: SourceEvaluation = evaluate_designated_source(EXPORT_MODEL.as_bytes(), &text, Some(&sidecar));
    assert!(evaluation.reasons.is_empty());
    let mut value = parse(EXPORT_MODEL.as_bytes());
    edit(&mut value);
    evaluation.inventory = Some(identity_inventory(&value));
    let result = import(&text, Some(&sidecar));
    designation("source.json", "0", 0, &evaluation, &result)
}

fn classes(designated: &ReviewDesignation<'_>) -> Vec<&'static str> {
    INVENTORY_CLASSES.iter().zip(designated.differing).filter(|(_, differs)| *differs).map(|(name, _)| *name).collect()
}

#[test]
fn substitution_duplicates_and_missing_ids_are_detected_class_by_class() {
    // Substituted: the same count, another id.
    let substituted = against_inventory_of(|value| {
        set(entry(value, "nodes", 0), &[], "id", Value::str("node:A-substituted"));
    });
    assert_eq!(substituted.reasons, ["identity_set_differs"]);
    assert_eq!(classes(&substituted), ["node"]);
    // Duplicated: one support's id given to another.
    let duplicated = against_inventory_of(|value| {
        set(entry(value, "supports", 1), &[], "id", Value::str("support:A"));
    });
    assert_eq!(classes(&duplicated), ["support"]);
    // Missing: a material the import carries that the source does not.
    let missing = against_inventory_of(|value| {
        let Value::List(materials) = member(value, &["materials"]) else { panic!("materials") };
        materials.push(parse(br#"{"id":"material:invented-extra"}"#));
    });
    assert_eq!(classes(&missing), ["material"]);
    // Malformed: an entry without a string id.
    let malformed = against_inventory_of(|value| {
        set(entry(value, "nodes", 0), &[], "id", Value::int(7));
    });
    assert!(malformed.reasons.contains(&"identity_inventory_malformed"));
}

#[test]
fn an_unbound_import_is_not_established() {
    let (text, sidecar) = pair(EXPORT_MODEL.as_bytes());
    let evaluation = evaluate_designated_source(EXPORT_MODEL.as_bytes(), &text, Some(&sidecar));
    // The same pair imported without its identity file (as if the file had gone).
    let mut options = ImportOptions::new("invented.mbf");
    options.material_provenance =
        Some(parse(br#"{"*":{"origin":"project_invented","source_name":"invented designation test"}}"#));
    let unbound = mbf::import(&text, &options).unwrap();
    let designated = designation("source.json", "0", 0, &evaluation, &unbound);
    assert!(designated.reasons.contains(&"import_not_bound_format_6"), "{:?}", designated.reasons);
}

#[test]
fn a_truncated_review_never_states_an_established_outcome() {
    let (text, sidecar) = pair(EXPORT_MODEL.as_bytes());
    let source = EXPORT_MODEL.as_bytes();
    let sha = common::sha(source);
    let (designated, result) = outcome(source, &text, Some(&sidecar), &sha);
    assert!(designated.reasons.is_empty());
    let (full, _) = review(&result, &text, Some(&designated), MBF_REVIEW_MAX_BYTES);
    assert!(full.summary.totals.iter().sum::<usize>() > 0);
    // The smallest allowance that forms a review leaves no room for any list entry.
    let model = result.model_bytes().unwrap().unwrap();
    let model_sha = common::sha(&model);
    let mbf_sha = common::sha(&text);
    let source = ImportReviewSource { file_name: "invented.mbf", mbf_sha256: &mbf_sha, mbf_bytes: text.len() as u64, identity_file: ReviewedIdentityFile::Absent };
    let candidate = Some(ReviewModel { sha256: &model_sha, bytes: model.len() as u64 });
    let check = ReviewDocumentCheck::Passed { schema_status: "current" };
    let form = |allowance| import_review_designated(&result, source, None, candidate, check, Some(&designated), allowance, |_| true);
    let allowance = form(0).unwrap_err().needed;
    let truncated = form(allowance).unwrap();
    assert!(!truncated.summary.complete);
    assert!(truncated.text.len() <= allowance);
    let parsed = parse(truncated.text.as_bytes());
    let identity = parsed.get_or_null("identity");
    assert_eq!(identity.get_or_null("source_completeness").as_str(), Some("not_established"));
    let stated = identity.get_or_null("designation");
    assert_eq!(stated.get_or_null("outcome").as_str(), Some("not_established"));
    assert_eq!(strings(stated.get_or_null("reasons")), ["review_truncated"]);
}
