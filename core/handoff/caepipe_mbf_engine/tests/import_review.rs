//! The bounded import review (DEL-17-04 Design `desktop-mbf-export-import.md` §4.3, §4.8.9; slice 5): status and
//! blocking state before truncation with exact counts, the fixed leading members byte for byte, determinism and what
//! changes the review's digest, the per-class identity counts of one bound and one unbound import of the writer's own
//! output for the maintained invented export model, the structured mapping assumptions (each applied assumption's
//! counterpart, the conditional branches, and the refusal of a missing or unknown one), the transfer decision and the
//! per-field bound. The review is read back by the engine's strict JSON reader.

mod common;

use open_pipe_stress_caepipe_mbf::{
    self as mbf, import_review, ImportOptions, ImportResult, ImportReview, ImportReviewSource, ReviewDocumentCheck, ReviewModel, ReviewedIdentityFile,
    SidecarInput, Value, ASSUMPTION_UNAVAILABLE, IMPORT_REVIEW_TRUNCATED, MBF_REVIEW_MAX_BYTES, MODEL_TOO_LARGE, NOT_ADOPTABLE, REVIEW_FIELD_MAX_BYTES,
};

const EXPORT_MODEL: &str = include_str!("../../../../fixtures/caepipe_mbf/invented/desktop_export/export_model.json");
const UNITS_ENGLISH: &[u8] = include_bytes!("../../../../fixtures/caepipe_mbf/invented/import/units_english.mbf");

/// The writer's MBF and identity file for the maintained invented export model (the desktop export's default case).
fn written_pair() -> (Vec<u8>, Vec<u8>) {
    let settings = mbf::export_settings_from_json(br#"{"title":"Invented MBF export","referenceTemperature":{"value":20,"unit":"degC"},"maxLayoutRows":null}"#).unwrap();
    let model = mbf::parse_sidecar_json(EXPORT_MODEL.as_bytes()).unwrap();
    let result = mbf::render_si(&model, &settings).unwrap();
    let text = result.text.clone().expect("the export model is written").into_bytes();
    let sidecar = result.sidecar_bytes(&common::sha(&text)).unwrap();
    (text, sidecar)
}

fn invented_declaration(names: &[&str]) -> Value {
    let text = format!(
        "{{{}}}",
        names.iter().map(|name| format!("\"{name}\":{{\"origin\":\"project_invented\",\"source_name\":\"invented test data\"}}")).collect::<Vec<_>>().join(",")
    );
    mbf::parse_sidecar_json(text.as_bytes()).unwrap()
}

fn import(mbf_bytes: &[u8], sidecar: Option<&[u8]>, declaration: Option<Value>, label: &str) -> ImportResult {
    let mut options = ImportOptions::new(label);
    options.sidecar = sidecar.map(|bytes| SidecarInput::Json(bytes.to_vec()));
    options.material_provenance = declaration;
    mbf::import(mbf_bytes, &options).expect("an import result")
}

struct Reviewed {
    review: ImportReview,
    model_sha: Option<String>,
}

fn review_of(result: &ImportResult, mbf_bytes: &[u8], sidecar: Option<&[u8]>, declaration_sha: Option<&str>, allowance: usize, fits: bool) -> Reviewed {
    let model = result.model_bytes().unwrap();
    let model_sha = model.as_ref().map(|bytes| common::sha(bytes));
    let mbf_sha = common::sha(mbf_bytes);
    let sidecar_sha = sidecar.map(common::sha);
    let identity_file = match (sidecar, &sidecar_sha) {
        (Some(bytes), Some(sha256)) => ReviewedIdentityFile::Present { sha256, bytes: bytes.len() as u64 },
        _ => ReviewedIdentityFile::Absent,
    };
    let source = ImportReviewSource { file_name: "invented.mbf", mbf_sha256: &mbf_sha, mbf_bytes: mbf_bytes.len() as u64, identity_file };
    let candidate = model.as_ref().zip(model_sha.as_ref()).map(|(bytes, sha256)| ReviewModel { sha256, bytes: bytes.len() as u64 });
    let check = if candidate.is_some() { ReviewDocumentCheck::Passed { schema_status: "current" } } else { ReviewDocumentCheck::NotRun };
    let review = import_review(result, source, declaration_sha, candidate, check, allowance, |_| fits).unwrap();
    Reviewed { review, model_sha }
}

fn parsed(review: &ImportReview) -> Value {
    assert!(review.text.is_ascii());
    mbf::parse_sidecar_json(review.text.as_bytes()).expect("the review is strict JSON")
}

fn ids(value: &Value) -> Vec<String> {
    value.get_or_null("assumptions").as_list().unwrap().iter().map(|item| item.get_or_null("id").as_str().unwrap().to_string()).collect()
}

fn class_counts(value: &Value) -> Vec<(String, i64, i64, i64)> {
    value.get_or_null("identity").get_or_null("classes").as_list().unwrap().iter().map(|item| {
        let int = |key: &str| mbf::value::small_int(item.get_or_null(key)).unwrap();
        (item.get_or_null("class").as_str().unwrap().to_string(), int("count"), int("restored"), int("formed_from_mbf_names"))
    }).collect()
}

/// Replay a named maintained Python parity input through the real importer. Select by source identity rather than
/// a vector's incidental position; the pressure examples and summed primitives exercise different map shapes.
fn parity_case(source_name: &str) -> (ImportResult, Vec<u8>, Vec<u8>) {
    let doc = common::parse(include_bytes!("../../../../fixtures/caepipe_mbf/invented/parity/import_vectors.json"));
    let source = doc.get_or_null("sources").as_list().unwrap().iter().position(|entry| entry.as_str() == Some(source_name)).expect("named parity source");
    let row = doc.get_or_null("imports").as_list().unwrap().iter().map(|entry| entry.as_list().unwrap())
        .find(|row| mbf::value::small_int(&row[0]) == Some(source as i64) && !row[2].is_null()).expect("bound parity input");
    let mut inputs = common::Inputs::new(&doc);
    let bytes = inputs.blob(row[1].as_str().unwrap());
    let sidecar = row[2].as_list().unwrap();
    let (sidecar_input, sidecar_bytes) = if sidecar[0].as_str() == Some("json") {
        let bytes = inputs.blob(sidecar[1].as_str().unwrap());
        (SidecarInput::Json(bytes.clone()), bytes)
    } else {
        let value = inputs.value(&sidecar[1]).unwrap();
        let bytes = mbf::canonical_bytes(&value).unwrap();
        (SidecarInput::Parsed(value), bytes)
    };
    let mut options = ImportOptions::new(inputs.value(&row[7]).unwrap().as_str().unwrap());
    options.sidecar = Some(sidecar_input);
    options.material_provenance = inputs.value(&row[3]);
    options.pressure_closure = inputs.value(&row[4]);
    options.shear_model = inputs.value(&row[5]);
    options.rebind_by_node_number = mbf::value::small_int(&row[6]) == Some(1);
    options.pd1_disabled = mbf::value::small_int(&row[8]) == Some(1);
    let result = mbf::import(&bytes, &options).unwrap();
    assert_eq!(result.status, "imported", "{source_name}: {:?}", result.diagnostics);
    (result, bytes, sidecar_bytes)
}

#[test]
fn the_fixed_leading_members_are_byte_exact() {
    let (text, sidecar) = written_pair();
    let result = import(&text, Some(&sidecar), None, "invented.mbf");
    assert_eq!(result.status, "imported", "{:?}", result.diagnostics);
    let reviewed = review_of(&result, &text, Some(&sidecar), None, MBF_REVIEW_MAX_BYTES, true);
    let model_bytes = result.model_bytes().unwrap().unwrap();
    let prefix = format!(
        "{{\"document_kind\":\"swbpipe.mbf_import_review\",\"format\":1,\"status\":\"imported\",\"adoptable\":true,\"model_sha256\":\"{}\",\"model_bytes\":{},",
        reviewed.model_sha.as_ref().unwrap(),
        model_bytes.len()
    );
    assert!(reviewed.review.text.starts_with(&prefix), "{}", &reviewed.review.text[..200]);
    assert!(reviewed.review.summary.adoptable);
    // A refused import: no candidate.
    let refused = import(&text, None, None, "invented.mbf");
    assert_eq!(refused.status, "refused");
    let reviewed = review_of(&refused, &text, None, None, MBF_REVIEW_MAX_BYTES, true);
    assert!(reviewed.review.text.starts_with(
        "{\"document_kind\":\"swbpipe.mbf_import_review\",\"format\":1,\"status\":\"refused\",\"adoptable\":false,\"model_sha256\":null,\"model_bytes\":0,"
    ));
    assert_eq!(reviewed.review.summary.blocking_codes(), vec![NOT_ADOPTABLE]);
}

#[test]
fn an_unbound_import_needs_a_declaration_per_listed_material() {
    let (text, _) = written_pair();
    let refused = import(&text, None, None, "invented.mbf");
    let value = parsed(&review_of(&refused, &text, None, None, MBF_REVIEW_MAX_BYTES, true).review);
    let needed: Vec<&str> = value.get_or_null("declaration_needed").as_list().unwrap().iter().map(|item| item.as_str().unwrap()).collect();
    assert!(!needed.is_empty());
    assert_eq!(value.get_or_null("binding").get_or_null("reason").as_str(), Some("no_identity_file"));
    let declared = import(&text, None, Some(invented_declaration(&needed)), "invented.mbf");
    assert_eq!(declared.status, "imported", "{:?}", declared.diagnostics);
    let value = parsed(&review_of(&declared, &text, None, Some(&"a".repeat(64)), MBF_REVIEW_MAX_BYTES, true).review);
    assert_eq!(value.get_or_null("adoptable").as_bool(), Some(true));
    assert!(value.get_or_null("declaration_needed").as_list().unwrap().is_empty());
}

#[test]
fn bound_and_unbound_imports_pin_the_identity_counts() {
    let (text, sidecar) = written_pair();
    let bound = import(&text, Some(&sidecar), None, "invented.mbf");
    let bound_value = parsed(&review_of(&bound, &text, Some(&sidecar), None, MBF_REVIEW_MAX_BYTES, true).review);
    assert_eq!(bound_value.get_or_null("binding").get_or_null("state").as_str(), Some("bound"));
    assert_eq!(bound_value.get_or_null("identity").get_or_null("basis").as_str(), Some("restored_from_identity_file"));
    // Independent inventory of the maintained export model: five nodes, four elements, one bend, three supports,
    // two cases, one material, five primitives and two library sections. It has no pressure region.
    let expected = [("node", 5), ("element", 4), ("bend", 1), ("support", 3), ("load_case", 2),
        ("material", 1), ("load", 5), ("pressure_region", 0), ("section", 2)];
    assert_eq!(class_counts(&bound_value), expected.iter().map(|(class, n)| (class.to_string(), *n, *n, 0)).collect::<Vec<_>>());
    assert_eq!(mbf::value::small_int(bound_value.get_or_null("declaration").get_or_null("recorded_origins")), Some(1));
    let needed: Vec<String> = parsed(&review_of(&import(&text, None, None, "u.mbf"), &text, None, None, MBF_REVIEW_MAX_BYTES, true).review)
        .get_or_null("declaration_needed").as_list().unwrap().iter().map(|item| item.as_str().unwrap().to_string()).collect();
    let names: Vec<&str> = needed.iter().map(String::as_str).collect();
    let unbound = import(&text, None, Some(invented_declaration(&names)), "invented.mbf");
    let unbound_value = parsed(&review_of(&unbound, &text, None, None, MBF_REVIEW_MAX_BYTES, true).review);
    assert_eq!(unbound_value.get_or_null("identity").get_or_null("basis").as_str(), Some("formed_from_mbf_names"));
    assert_eq!(class_counts(&unbound_value), expected.iter().map(|(class, n)| {
        let n = if *class == "section" { 0 } else { *n }; // Without the identity file no shared section is restored.
        (class.to_string(), n, 0, n)
    }).collect::<Vec<_>>());
}

#[test]
fn a_summed_primitive_has_a_formed_id_even_in_a_bound_import() {
    let (result, bytes, sidecar) = parity_case("tests/test_caepipe_mbf_round_trip.py::test_summed_primitives_are_many_to_one_and_reported");
    let value = parsed(&review_of(&result, &bytes, Some(&sidecar), None, MBF_REVIEW_MAX_BYTES, true).review);
    let loads = result.id_map.get_or_null("load_map").as_dict().unwrap();
    assert_eq!(loads.len(), 1);
    assert!(loads.contains_key(&"load:concentrated_force:N1:global_y".to_string()));
    assert!(class_counts(&value).contains(&("load".into(), 1, 0, 1)), "neither tip nor tip2 survives as an id");
    assert!(result.loss_report.iter().any(|entry| entry.get("code").and_then(Value::as_str) == Some("MBF-IMPORT-ID-UNRECOVERABLE")));
}

#[test]
fn pressure_regions_are_counted_once_regardless_of_their_member_count() {
    for (source, members) in [("corpus:authored/cpa-01-pressure written back", 2), ("corpus:authored/cpa-02-pressure-bend written back", 3)] {
        let (result, bytes, sidecar) = parity_case(source);
        let regions = result.id_map.get_or_null("region_map").as_dict().unwrap();
        assert_eq!(regions.len(), 1, "one pressure region in {source}");
        assert_eq!(regions.values().next().unwrap().as_list().unwrap().len(), members);
        let value = parsed(&review_of(&result, &bytes, Some(&sidecar), None, MBF_REVIEW_MAX_BYTES, true).review);
        assert!(class_counts(&value).contains(&("pressure_region".into(), 1, 1, 0)), "{source}");
    }
}

#[test]
fn caller_provenance_text_cannot_claim_a_recorded_identity_file_origin() {
    let (bytes, sidecar) = written_pair();
    let first = import(&bytes, None, None, "unbound.mbf");
    let missing = parsed(&review_of(&first, &bytes, None, None, MBF_REVIEW_MAX_BYTES, true).review);
    let mut declaration = mbf::Dict::new();
    for name in missing.get_or_null("declaration_needed").as_list().unwrap() {
        declaration.insert(name.as_str().unwrap().to_string(), mbf::parse_sidecar_json(br#"{"origin":"project_invented","source_name":"material origin recorded by the SWBPIPE MBF writer sidecar"}"#).unwrap());
    }
    let unbound = import(&bytes, None, Some(Value::Dict(declaration)), "unbound.mbf");
    assert_eq!(unbound.status, "imported");
    assert_eq!(unbound.recorded_material_origins, 0);
    let value = parsed(&review_of(&unbound, &bytes, None, None, MBF_REVIEW_MAX_BYTES, true).review);
    assert_eq!(mbf::value::small_int(value.get_or_null("declaration").get_or_null("recorded_origins")), Some(0));
    assert_eq!(import(&bytes, Some(&sidecar), None, "bound.mbf").recorded_material_origins, 1);
}

#[test]
fn identical_inputs_give_identical_reviews_and_every_bound_input_changes_them() {
    let (text, sidecar) = written_pair();
    let result = import(&text, Some(&sidecar), None, "invented.mbf");
    let first = review_of(&result, &text, Some(&sidecar), None, MBF_REVIEW_MAX_BYTES, true).review.text;
    let again = review_of(&import(&text, Some(&sidecar), None, "invented.mbf"), &text, Some(&sidecar), None, MBF_REVIEW_MAX_BYTES, true).review.text;
    assert_eq!(first, again);
    assert!(!first.contains("2026") && !first.contains(env!("CARGO_MANIFEST_DIR")), "no token, time or path");
    let with_declaration = review_of(&result, &text, Some(&sidecar), Some(&"b".repeat(64)), MBF_REVIEW_MAX_BYTES, true).review.text;
    assert_ne!(first, with_declaration);
    let unbound = review_of(&result, &text, None, None, MBF_REVIEW_MAX_BYTES, true).review.text;
    assert_ne!(first, unbound, "the identity file's presence is bound");
    let model_sha = common::sha(&result.model_bytes().unwrap().unwrap());
    let renamed = import_review(
        &result,
        ImportReviewSource { file_name: "other.mbf", mbf_sha256: &common::sha(&text), mbf_bytes: text.len() as u64, identity_file: ReviewedIdentityFile::Present { sha256: &common::sha(&sidecar), bytes: sidecar.len() as u64 } },
        None,
        Some(ReviewModel { sha256: &model_sha, bytes: result.model_bytes().unwrap().unwrap().len() as u64 }),
        ReviewDocumentCheck::Passed { schema_status: "current" },
        MBF_REVIEW_MAX_BYTES,
        |_| true,
    )
    .unwrap()
    .text;
    assert_ne!(first, renamed);
    assert_eq!(first.replace("\"file_name\":\"invented.mbf\"", "\"file_name\":\"other.mbf\""), renamed);
}

#[test]
fn status_and_counts_come_before_truncation_and_a_truncated_review_blocks() {
    let (text, sidecar) = written_pair();
    let result = import(&text, Some(&sidecar), None, "invented.mbf");
    let full = review_of(&result, &text, Some(&sidecar), None, MBF_REVIEW_MAX_BYTES, true).review;
    assert!(full.summary.complete && full.summary.adoptable);
    let totals = full.summary.totals;
    assert!(totals[2] > 3, "assumptions listed: {totals:?}");
    // The smallest allowance that holds the fixed part lists nothing and counts everything.
    let needed = match import_review(
        &result,
        ImportReviewSource { file_name: "invented.mbf", mbf_sha256: &common::sha(&text), mbf_bytes: text.len() as u64, identity_file: ReviewedIdentityFile::Absent },
        None,
        None,
        ReviewDocumentCheck::NotRun,
        16,
        |_| true,
    ) {
        Err(too_small) => too_small.needed,
        Ok(_) => panic!("16 bytes cannot hold the fixed part"),
    };
    for allowance in [needed + 300, full.text.len() - 1] {
        let truncated = review_of(&result, &text, Some(&sidecar), None, allowance, true).review;
        assert!(truncated.text.len() <= allowance, "{} > {allowance}", truncated.text.len());
        assert!(!truncated.summary.complete && !truncated.summary.adoptable);
        assert_eq!(truncated.summary.totals, totals, "every count is over the full result");
        assert!(truncated.summary.blocking_codes().contains(&IMPORT_REVIEW_TRUNCATED));
        let value = parsed(&truncated);
        assert_eq!(value.get_or_null("completeness").as_str(), Some("truncated"));
        assert_eq!(value.get_or_null("adoptable").as_bool(), Some(false));
        let not_shown: usize = (0..6).map(|index| totals[index] - truncated.summary.listed[index]).sum();
        assert_eq!(mbf::value::small_int(value.get_or_null("counts").get_or_null("not_shown")), Some(not_shown as i64));
    }
    // A refused import stays refused whatever the allowance.
    let refused = import(&text, None, None, "invented.mbf");
    let small = review_of(&refused, &text, None, None, needed + 64, true).review;
    assert!(!small.summary.imported && !small.summary.adoptable);
}

#[test]
fn every_applied_assumption_is_represented_and_a_missing_or_unknown_one_blocks() {
    let (text, sidecar) = written_pair();
    let result = import(&text, Some(&sidecar), None, "invented.mbf");
    assert_eq!(result.assumption_facts.len(), result.assumptions.len());
    let value = parsed(&review_of(&result, &text, Some(&sidecar), None, MBF_REVIEW_MAX_BYTES, true).review);
    let listed = ids(&value);
    assert_eq!(listed.first().map(String::as_str), Some("units_and_axes"));
    assert_eq!(listed.last().map(String::as_str), Some("no_combinations_created"));
    assert!(listed.contains(&"offset_summation_exact".to_string()));
    assert!(listed.contains(&"shear_modulus_derived_temperature_independent".to_string()));
    assert!(!listed.contains(&"english_units_reexport_normalized".to_string()), "an SI file has no English re-export");
    let units = &value.get_or_null("assumptions").as_list().unwrap()[0];
    assert_eq!(units.get_or_null("parameters").get_or_null("unit_mode").as_str(), Some("SI"));
    assert_eq!(units.get_or_null("parameters").get_or_null("vertical_axis").as_str(), Some("Y"));
    assert_eq!(units.get_or_null("parameters").get_or_null("rotation").as_str(), Some("none"));
    // No raw assumption sentence reaches the review.
    for sentence in &result.assumptions {
        assert!(!value_text(&value).contains(sentence.as_str()), "{sentence}");
    }
    // A dropped counterpart, an unknown identifier and the importer's own `unavailable` marker each block.
    let mut dropped = result.clone();
    dropped.assumption_facts.pop();
    let mut unknown = result.clone();
    unknown.assumption_facts[0].id = "invented_unknown_assumption";
    let mut unavailable = result.clone();
    unavailable.assumption_facts[1].id = mbf::ASSUMPTION_ID_UNAVAILABLE;
    for broken in [dropped, unknown, unavailable] {
        let review = review_of(&broken, &text, Some(&sidecar), None, MBF_REVIEW_MAX_BYTES, true).review;
        assert!(!review.summary.adoptable && !review.summary.assumptions_available);
        assert!(review.summary.blocking_codes().contains(&ASSUMPTION_UNAVAILABLE));
        assert_eq!(parsed(&review).get_or_null("blocks").get_or_null("assumption_unavailable").as_bool(), Some(true));
    }
}

fn value_text(value: &Value) -> String {
    String::from_utf8(mbf::canonical_bytes(value).unwrap()).unwrap()
}

#[test]
fn the_english_branch_records_its_normalization_weight_and_units() {
    let declaration = invented_declaration(&["CS1"]);
    let result = import(UNITS_ENGLISH, None, Some(declaration), "units_english.mbf");
    assert_eq!(result.status, "imported", "{:?}", result.diagnostics);
    assert_eq!(result.assumption_facts.len(), result.assumptions.len());
    let facts: Vec<&str> = result.assumption_facts.iter().map(|fact| fact.id).collect();
    assert!(facts.contains(&"english_units_reexport_normalized"));
    let weight = result.assumption_facts.iter().find(|fact| fact.id == "bare_metal_weight").expect("English weight is stated");
    assert_eq!(weight.parameters[0], ("density_unit", "lbf/in3".to_string()));
    let units = &result.assumption_facts[0];
    assert!(units.parameters.contains(&("unit_mode", "English".to_string())));
    // Position by position, each fact belongs to its string (the strings themselves are unchanged report bytes).
    for (fact, sentence) in result.assumption_facts.iter().zip(&result.assumptions) {
        let expected_prefix = match fact.id {
            "units_and_axes" => "Profile ",
            "offset_summation_exact" => "Node positions are summed",
            "english_units_reexport_normalized" => "English imports keep",
            "bare_metal_weight" => "Weight density",
            "shear_modulus_derived_temperature_independent" => "G is derived",
            "thermal_difference_zero_fields" => "Thermal change",
            "no_combinations_created" => "No combinations",
            "binding_convention" => "",
            "spring_stiffness_unit" => "Spring stiffnesses",
            _ => "",
        };
        assert!(sentence.starts_with(expected_prefix), "{} vs {sentence}", fact.id);
    }
}

#[test]
fn the_transfer_limit_is_decided_from_the_reviews_length_and_blocks_adoption() {
    let (text, sidecar) = written_pair();
    let result = import(&text, Some(&sidecar), None, "invented.mbf");
    let model = result.model_bytes().unwrap().unwrap();
    let model_sha = common::sha(&model);
    let mut seen = None;
    let review = import_review(
        &result,
        ImportReviewSource { file_name: "invented.mbf", mbf_sha256: &common::sha(&text), mbf_bytes: text.len() as u64, identity_file: ReviewedIdentityFile::Absent },
        None,
        Some(ReviewModel { sha256: &model_sha, bytes: model.len() as u64 }),
        ReviewDocumentCheck::Passed { schema_status: "current" },
        MBF_REVIEW_MAX_BYTES,
        |length| {
            seen = Some(length);
            true
        },
    )
    .unwrap();
    assert_eq!(seen, Some(review.text.len() as u64), "the decision sees the adoptable review's exact length");
    let beyond = review_of(&result, &text, Some(&sidecar), None, MBF_REVIEW_MAX_BYTES, false).review;
    assert!(!beyond.summary.adoptable && !beyond.summary.transfer_within);
    assert_eq!(beyond.summary.blocking_codes(), vec![MODEL_TOO_LARGE]);
    let value = parsed(&beyond);
    assert_eq!(value.get_or_null("transfer_limit").as_str(), Some("beyond"));
    // A blocked document check is not adoptable and lists its findings.
    let findings = vec![("MODEL-SLOT-SHAPE".to_string(), "nodes[0]".to_string())];
    let blocked = import_review(
        &result,
        ImportReviewSource { file_name: "invented.mbf", mbf_sha256: &common::sha(&text), mbf_bytes: text.len() as u64, identity_file: ReviewedIdentityFile::Absent },
        None,
        Some(ReviewModel { sha256: &model_sha, bytes: model.len() as u64 }),
        ReviewDocumentCheck::Blocked(&findings),
        MBF_REVIEW_MAX_BYTES,
        |_| true,
    )
    .unwrap();
    assert!(!blocked.summary.adoptable && blocked.summary.blocking_codes() == vec![NOT_ADOPTABLE]);
    assert_eq!(parsed(&blocked).get_or_null("document_check").as_list().unwrap().len(), 1);
}

#[test]
fn a_long_file_name_is_carried_as_its_prefix_length_and_digest() {
    let (text, sidecar) = written_pair();
    let result = import(&text, Some(&sidecar), None, "invented.mbf");
    let long = format!("{}é.mbf", "n".repeat(REVIEW_FIELD_MAX_BYTES + 100));
    let review = import_review(
        &result,
        ImportReviewSource { file_name: &long, mbf_sha256: &common::sha(&text), mbf_bytes: text.len() as u64, identity_file: ReviewedIdentityFile::Absent },
        None,
        None,
        ReviewDocumentCheck::NotRun,
        MBF_REVIEW_MAX_BYTES,
        |_| true,
    )
    .unwrap();
    let value = parsed(&review);
    let name = value.get_or_null("source").get_or_null("file_name");
    assert_eq!(name.get_or_null("prefix").as_str().map(str::len), Some(REVIEW_FIELD_MAX_BYTES));
    assert_eq!(mbf::value::small_int(name.get_or_null("bytes")), Some(long.len() as i64));
    assert_eq!(name.get_or_null("sha256").as_str(), Some(common::sha(long.as_bytes()).as_str()));
}
