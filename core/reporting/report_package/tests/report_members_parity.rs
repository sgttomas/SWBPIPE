//! The native producers of a report input's row-scale members
//! (`report_package::report_members`, DEL-08-01) against report inputs the
//! desktop's builder wrote: the five exact-route witness requests
//! (`fixtures/reports/invented/exact_report_package_requests/`, written by
//! `apps/desktop/src/features/report/exactReportBelowGate.test.tsx`), the two
//! exact user-release fixtures (`fixtures/reports/invented/user_release_exact/`)
//! and the pressure-free user-release vectors
//! (`fixtures/reports/invented/user_release_derivation_vectors.json`).
//!
//! Equality is on canonical bytes (RFC 8785): a produced member's canonical
//! text is the committed member's. Invented data only. Standard claim fence
//! applies (claims taxonomy per DEC-081).

use open_pipe_stress_canonical_json::canonical_json;
use open_pipe_stress_report_package::bindings::{check_report_diagnostics, check_report_result_refs, check_unit_display_summary};
use open_pipe_stress_report_package::report_members::{
    derivative_sha256, place_members, result_coordinate_systems, result_diagnostics, result_document, result_export_refs, result_rows,
    seal_report_sections, unit_display_summary, MEMBERS_NOT_PLACED,
};
use open_pipe_stress_report_package::user_release::{first_difference, session_provenance};
use open_pipe_stress_result_export::derivative::digest;
use serde_json::{json, Value};

const WITNESS_REQUESTS: [(&str, &str); 5] = [
    ("pressure_2_straight", include_str!("../../../../fixtures/reports/invented/exact_report_package_requests/pressure_2_straight.json")),
    (
        "pressure_2_load_reference_states",
        include_str!("../../../../fixtures/reports/invented/exact_report_package_requests/pressure_2_load_reference_states.json"),
    ),
    ("pressure_2_connector", include_str!("../../../../fixtures/reports/invented/exact_report_package_requests/pressure_2_connector.json")),
    ("pressure_2_realized_bend", include_str!("../../../../fixtures/reports/invented/exact_report_package_requests/pressure_2_realized_bend.json")),
    ("pressure_source_1", include_str!("../../../../fixtures/reports/invented/exact_report_package_requests/pressure_source_1.json")),
];
const USER_RELEASE_EXACT: [(&str, &str); 2] = [
    ("user_release_exact/pressure_2_straight", include_str!("../../../../fixtures/reports/invented/user_release_exact/pressure_2_straight.json")),
    ("user_release_exact/pressure_source_1", include_str!("../../../../fixtures/reports/invented/user_release_exact/pressure_source_1.json")),
];
const USER_RELEASE_VECTORS: &str = include_str!("../../../../fixtures/reports/invented/user_release_derivation_vectors.json");
const VECTOR_MODEL: &str = include_str!("../../../../fixtures/model_operations/precision_connected_ui_model.json");
const VECTOR_SOURCES: [(&str, &str); 2] = [
    ("fixtures/results/preview_physics_connected_sparse.json", include_str!("../../../../fixtures/results/preview_physics_connected_sparse.json")),
    ("fixtures/results/preview_physics_connected_dense.json", include_str!("../../../../fixtures/results/preview_physics_connected_dense.json")),
];
/// The standing witnesses' render inputs (`reportStandingWitness.test.tsx`), each with the
/// raw result it was built from.
const STANDING_WITNESSES: [(&str, &str, &str); 2] = [
    (
        "standing witness pressure_3_coupled_friction_sensitive",
        include_str!("../../../../fixtures/reports/invented/report_standing_witnesses/pressure_3_coupled_friction_sensitive.render.json"),
        include_str!("../../../../fixtures/reports/invented/report_standing_witnesses/pressure_3_coupled_friction_sensitive-sparse_interactive.raw.json"),
    ),
    (
        "standing witness pressure_3_gap_unresolved",
        include_str!("../../../../fixtures/reports/invented/report_standing_witnesses/pressure_3_gap_unresolved.render.json"),
        include_str!("../../../../fixtures/reports/invented/report_standing_witnesses/pressure_3_gap_unresolved-sparse_interactive.raw.json"),
    ),
];
const DIAGNOSTICS_VECTORS: &str = include_str!("../../../../fixtures/reports/invented/report_diagnostics_parity.json");

/// One report input the desktop's builder wrote, with the model and the
/// source result it was built from.
struct Built {
    name: String,
    model: Value,
    source: Value,
    report: Value,
}

fn parse(text: &str) -> Value {
    serde_json::from_str(text).unwrap()
}

/// Thirteen committed report inputs the desktop's builder wrote: five full
/// local-private exact inputs, two minimal exact inputs, four minimal
/// pressure-free inputs and the two standing witnesses' render inputs.
fn built_reports() -> Vec<Built> {
    let mut built = Vec::new();
    for (name, text) in WITNESS_REQUESTS {
        let fixture = parse(text);
        built.push(Built {
            name: name.to_string(),
            model: fixture["model"].clone(),
            source: fixture["request"]["source_results"][0].clone(),
            report: fixture["request"]["report"].clone(),
        });
    }
    for (name, text) in USER_RELEASE_EXACT {
        let fixture = parse(text);
        built.push(Built { name: name.to_string(), model: fixture["model"].clone(), source: fixture["source"].clone(), report: fixture["request"]["report"].clone() });
    }
    let model = parse(VECTOR_MODEL);
    for vector in parse(USER_RELEASE_VECTORS)["vectors"].as_array().unwrap() {
        let path = vector["source_fixture"].as_str().unwrap();
        let source = VECTOR_SOURCES.iter().find(|(name, _)| *name == path).unwrap_or_else(|| panic!("no committed fixture {path}")).1;
        built.push(Built {
            name: format!("vector {}", vector["name"].as_str().unwrap()),
            model: model.clone(),
            source: parse(source),
            report: vector["expected"]["report"].clone(),
        });
    }
    for (name, render, raw) in STANDING_WITNESSES {
        let fixture = parse(render);
        built.push(Built { name: name.to_string(), model: parse(fixture["exact_intake_text"].as_str().unwrap()), source: parse(raw), report: fixture["input"].clone() });
    }
    assert_eq!(built.len(), 13);
    built
}

#[track_caller]
fn assert_canonical_eq(produced: &Value, committed: &Value, what: &str) {
    if canonical_json(produced) != canonical_json(committed) {
        panic!("{what}: differs from the desktop builder's at {:?}", first_difference(committed, produced));
    }
}

/// The report input without the members the host derives: no result rows,
/// references, unit display summary or coordinate systems, and only the
/// Diagnostics entries that follow the result's. Where the source has
/// diagnostics, the sections checksum the input still states is then stale.
fn without_row_scale_members(built: &Built) -> Value {
    let mut report = built.report.clone();
    let source_diagnostics = built.source["diagnostics"].as_array().unwrap().len();
    assert!(report.as_object_mut().unwrap().remove("result_rows").is_some());
    assert!(report["calculation_report"].as_object_mut().unwrap().remove("result_export_refs").is_some());
    assert!(report["calculation_report"]["model_input_summary"].as_object_mut().unwrap().remove("unit_display_summary").is_some());
    assert!(report["report_context"].as_object_mut().unwrap().remove("result_coordinate_systems").is_some());
    report["report_sections"]["diagnostics"].as_array_mut().unwrap().drain(..source_diagnostics);
    report
}

#[test]
fn each_producer_gives_the_desktop_builders_member() {
    for built in built_reports() {
        let Built { name, model, source, report } = &built;
        let provenance = session_provenance(model).unwrap();
        let rows = source["results"].as_array().unwrap().len();
        assert!(rows > 0, "{name}");

        assert_canonical_eq(&Value::Array(result_rows(source).unwrap()), &report["result_rows"], &format!("{name}: result_rows"));

        let refs = result_export_refs(source, &provenance).unwrap();
        assert_eq!(refs.len(), rows, "{name}");
        assert_canonical_eq(&Value::Array(refs), &report["calculation_report"]["result_export_refs"], &format!("{name}: result_export_refs"));

        let diagnostics = result_diagnostics(source, &provenance).unwrap();
        let committed = report["report_sections"]["diagnostics"].as_array().unwrap();
        assert_eq!(diagnostics.len(), source["diagnostics"].as_array().unwrap().len(), "{name}");
        assert_canonical_eq(&Value::Array(diagnostics.clone()), &Value::Array(committed[..diagnostics.len()].to_vec()), &format!("{name}: diagnostics"));

        assert_canonical_eq(
            &unit_display_summary(model, source).unwrap(),
            &report["calculation_report"]["model_input_summary"]["unit_display_summary"],
            &format!("{name}: unit_display_summary"),
        );
        assert_canonical_eq(
            &json!(result_coordinate_systems(source).unwrap()),
            &report["report_context"]["result_coordinate_systems"],
            &format!("{name}: result_coordinate_systems"),
        );
    }
}

/// The whole report input: the builder's input without its row-scale members,
/// completed natively and sealed, is the builder's input, byte for byte in
/// canonical form, and passes the request checks that bind those members.
#[test]
fn a_report_input_completed_natively_is_the_desktop_builders() {
    for built in built_reports() {
        let mut report = without_row_scale_members(&built);
        assert_ne!(canonical_json(&report), canonical_json(&built.report), "{}", built.name);
        place_members(&mut report, &built.model, &built.source).unwrap_or_else(|error| panic!("{}: {error}", built.name));
        seal_report_sections(&mut report).unwrap_or_else(|error| panic!("{}: {error}", built.name));
        assert_canonical_eq(&report, &built.report, &built.name);
        check_report_result_refs(&report, &built.source).unwrap();
        check_unit_display_summary(&report, &built.source).unwrap();
        check_report_diagnostics(&report, &built.source).unwrap();
    }
}

/// The session's Diagnostics entries follow the result's, in their own order.
/// No committed builder input carries a session entry, so this is stated on an
/// invented one.
#[test]
fn the_sessions_diagnostics_follow_the_results() {
    let built = built_reports().into_iter().next().unwrap();
    let from_result = built.source["diagnostics"].as_array().unwrap().len();
    assert!(from_result > 0);
    let session = json!([{"code": "INVENTED_SESSION_FIRST"}, {"code": "INVENTED_SESSION_SECOND"}]);
    let mut report = without_row_scale_members(&built);
    assert_eq!(report["report_sections"]["diagnostics"], json!([]));
    report["report_sections"]["diagnostics"] = session.clone();
    place_members(&mut report, &built.model, &built.source).unwrap();
    let placed = report["report_sections"]["diagnostics"].as_array().unwrap();
    assert_eq!(placed.len(), from_result + 2);
    assert_canonical_eq(&Value::Array(placed[..from_result].to_vec()), &built.report["report_sections"]["diagnostics"], "the result's entries");
    assert_eq!(Value::Array(placed[from_result..].to_vec()), session);
    check_report_diagnostics(&report, &built.source).unwrap();
}

/// The seal is the builder's sections checksum: recomputed over the committed
/// sections it gives the committed value, and it follows a later change.
#[test]
fn the_sections_seal_is_the_desktop_builders_checksum() {
    for built in built_reports() {
        let committed = built.report["calculation_report"]["report_section_refs"][0]["checksum"]["value"].as_str().unwrap().to_string();
        let mut report = built.report.clone();
        report["calculation_report"]["report_section_refs"][0]["checksum"]["value"] = json!("TBD");
        seal_report_sections(&mut report).unwrap();
        assert_eq!(report["calculation_report"]["report_section_refs"][0]["checksum"]["value"], committed.as_str(), "{}", built.name);
        report["report_sections"]["assumptions"] = json!([{"changed": true}]);
        seal_report_sections(&mut report).unwrap();
        assert_ne!(report["calculation_report"]["report_section_refs"][0]["checksum"]["value"], committed.as_str(), "{}", built.name);
    }
}

/// A host that derives a member does not also take it from its caller: each of
/// the four members, present, refuses by name, and names no value.
#[test]
fn a_report_input_that_carries_a_derived_member_refuses() {
    let built = built_reports().into_iter().next().unwrap();
    let stripped = without_row_scale_members(&built);
    for (pointer, key, path) in [
        ("", "result_rows", "result_rows"),
        ("/calculation_report", "result_export_refs", "calculation_report.result_export_refs"),
        ("/calculation_report/model_input_summary", "unit_display_summary", "calculation_report.model_input_summary.unit_display_summary"),
        ("/report_context", "result_coordinate_systems", "report_context.result_coordinate_systems"),
    ] {
        for carried in [json!([]), json!(["invented-carried-value"]), json!({"invented": "carried-value"})] {
            let mut report = stripped.clone();
            report.pointer_mut(pointer).unwrap().as_object_mut().unwrap().insert(key.to_string(), carried);
            let before = report.clone();
            let error = place_members(&mut report, &built.model, &built.source).unwrap_err();
            assert_eq!(error, format!("{MEMBERS_NOT_PLACED}: the report input already carries {path}"));
            // Decided before anything is written: no derived member sits beside the caller's.
            assert_eq!(report, before, "{path}");
        }
        // A null member is an absent one.
        let mut report = stripped.clone();
        report.pointer_mut(pointer).unwrap().as_object_mut().unwrap().insert(key.to_string(), Value::Null);
        place_members(&mut report, &built.model, &built.source).unwrap();
    }
}

#[test]
fn a_report_input_with_no_place_for_a_member_refuses() {
    let built = built_reports().into_iter().next().unwrap();
    let stripped = without_row_scale_members(&built);
    for pointer in [
        "/calculation_report",
        "/calculation_report/model_input_summary",
        "/report_context",
        "/report_sections/diagnostics",
        "/report_sections",
        "/calculation_report/report_section_refs",
        "/calculation_report/report_section_refs/0/checksum",
    ] {
        let mut report = stripped.clone();
        *report.pointer_mut(pointer).unwrap() = Value::Null;
        let before = report.clone();
        let error = place_members(&mut report, &built.model, &built.source).unwrap_err();
        assert!(error.starts_with(&format!("{MEMBERS_NOT_PLACED}: ")), "{pointer}: {error}");
        assert_eq!(report, before, "{pointer}");
    }
    let error = place_members(&mut json!([]), &built.model, &built.source).unwrap_err();
    assert!(error.starts_with(&format!("{MEMBERS_NOT_PLACED}: ")), "{error}");
}

/// The seal writes only into the builder's own reference to these sections:
/// one entry, naming the sections by their id, stating SHA-256 over RFC 8785
/// for that same reference. Anything else refuses and writes nothing.
#[test]
fn the_seal_refuses_any_other_sections_reference() {
    let built = built_reports().into_iter().next().unwrap();
    let entry = built.report["calculation_report"]["report_section_refs"][0].clone();
    let with = |pointer: &str, value: Value| {
        let mut entry = entry.clone();
        *entry.pointer_mut(pointer).unwrap() = value;
        json!([entry])
    };
    for (what, refs) in [
        ("no entry", json!([])),
        ("two entries", json!([entry.clone(), entry.clone()])),
        ("no checksum", json!([{"ref": entry["ref"]}])),
        ("not a list", Value::Null),
        ("another algorithm", with("/checksum/algorithm", json!("sha512"))),
        ("another canonical form", with("/checksum/canonicalization", json!("JCS"))),
        ("a checksum of another payload", with("/checksum/payload_ref/ref_id", json!("sections:another"))),
        ("a reference to other sections", with("/ref/ref_id", json!("sections:another"))),
        ("a reference of another type", with("/ref/ref_type", json!("report"))),
    ] {
        let mut report = built.report.clone();
        report["calculation_report"]["report_section_refs"] = refs;
        let before = report.clone();
        let error = seal_report_sections(&mut report).unwrap_err();
        assert!(error.starts_with(&format!("{MEMBERS_NOT_PLACED}: ")), "{what}: {error}");
        assert_eq!(report, before, "{what}");
    }
    for (what, sections) in [("no sections", Value::Null), ("sections without an id", json!({"diagnostics": []}))] {
        let mut report = built.report.clone();
        report["report_sections"] = sections;
        let error = seal_report_sections(&mut report).unwrap_err();
        assert!(error.starts_with(&format!("{MEMBERS_NOT_PLACED}: ")), "{what}: {error}");
    }
    // A number the canonical form does not admit, anywhere in the sections.
    let mut report = built.report.clone();
    report["report_sections"]["assumptions"] = json!([1e21]);
    let before = report.clone();
    assert!(seal_report_sections(&mut report).unwrap_err().starts_with("REPORT-PACKAGE-SOURCE-INVALID: "));
    assert_eq!(report, before);
}

/// A caller's Diagnostics entry with the code and affected object of one of the
/// result's entries is a copy and refuses, whatever else it says; the refusal
/// names its index and the input is unchanged. The same code on another
/// object is not a copy. On an exact-route and on a pressure-free source.
#[test]
fn a_callers_copy_of_a_result_diagnostic_refuses() {
    let all = built_reports();
    for name in ["pressure_2_straight", "vector connected_sparse_interactive"] {
        let built = all.iter().find(|built| built.name == name).unwrap();
        let stripped = without_row_scale_members(built);
        let of_the_result = built.report["report_sections"]["diagnostics"].as_array().unwrap();
        let last = of_the_result.len() - 1;
        let mut edited = of_the_result[last].clone();
        edited["message"] = json!("invented edit");
        edited["severity"] = json!("info");
        for (what, copy) in [("a copy", of_the_result[0].clone()), ("an edited copy", edited)] {
            let mut report = stripped.clone();
            report["report_sections"]["diagnostics"] = json!([{"code": "INVENTED_SESSION"}, copy]);
            let before = report.clone();
            let error = place_members(&mut report, &built.model, &built.source).unwrap_err();
            assert_eq!(error, format!("{MEMBERS_NOT_PLACED}: report_sections.diagnostics entry 1 copies a result diagnostic's code and affected object"), "{name}: {what}");
            assert_eq!(report, before, "{name}: {what}");
        }
        let mut elsewhere = of_the_result[0].clone();
        elsewhere["affected_object"]["ref_id"] = json!("invented:other-object");
        let mut report = stripped.clone();
        report["report_sections"]["diagnostics"] = json!([elsewhere]);
        place_members(&mut report, &built.model, &built.source).unwrap_or_else(|error| panic!("{name}: {error}"));
        assert_eq!(report["report_sections"]["diagnostics"].as_array().unwrap().len(), of_the_result.len() + 1);
    }
}

/// A report input placed and not sealed states no sections checksum, and so
/// does not decode as a renderer input; sealed, it does.
#[test]
fn a_placed_input_is_not_a_renderer_input_until_it_is_sealed() {
    use open_pipe_stress_report_renderer::decode_renderable_report_input;
    let built = built_reports().into_iter().next().unwrap();
    let committed = built.report["calculation_report"]["report_section_refs"][0]["checksum"]["value"].clone();
    // Whatever the input states is removed: the builder's own digest, stale once the
    // result's Diagnostics are placed, and a placeholder alike.
    for stated in [committed.clone(), json!("TBD"), json!("0".repeat(64))] {
        let mut report = without_row_scale_members(&built);
        report["calculation_report"]["report_section_refs"][0]["checksum"]["value"] = stated;
        place_members(&mut report, &built.model, &built.source).unwrap();
        assert!(report["calculation_report"]["report_section_refs"][0]["checksum"].get("value").is_none());
        assert!(decode_renderable_report_input(report.clone()).is_err());
        seal_report_sections(&mut report).unwrap();
        assert_eq!(report["calculation_report"]["report_section_refs"][0]["checksum"]["value"], committed);
        assert!(decode_renderable_report_input(report).is_ok());
    }
    assert_eq!(committed.as_str().unwrap().len(), 64);
}

/// Nothing is derived from an unreadable source, and the report input offered
/// with it is unchanged.
#[test]
fn an_unreadable_source_derives_nothing() {
    let built = built_reports().into_iter().next().unwrap();
    let provenance = session_provenance(&built.model).unwrap();
    for source in [json!({}), json!({"results": "rows"})] {
        assert!(result_export_refs(&source, &provenance).unwrap_err().starts_with("REPORT-PACKAGE-BINDING-MISMATCH: "));
        assert!(unit_display_summary(&built.model, &source).unwrap_err().starts_with("REPORT-PACKAGE-BINDING-MISMATCH: "));
        assert!(result_coordinate_systems(&source).unwrap_err().starts_with("REPORT-PACKAGE-BINDING-MISMATCH: "));
        let mut report = without_row_scale_members(&built);
        let before = report.clone();
        assert!(place_members(&mut report, &built.model, &source).is_err());
        assert_eq!(report, before);
    }
    // A row with no id, and a row whose value the canonical form does not admit.
    assert!(result_export_refs(&json!({"results": [{"kind": "k"}]}), &provenance).unwrap_err().starts_with("REPORT-PACKAGE-BINDING-MISMATCH: "));
    let unsafe_row = json!({"results": [{"id": "a", "kind": "k", "unit": "N", "value": 1e21}]});
    assert!(result_export_refs(&unsafe_row, &provenance).unwrap_err().starts_with("REPORT-PACKAGE-SOURCE-INVALID: "));
    assert!(unit_display_summary(&json!({"project": {"units": []}}), &built.source).unwrap_err().starts_with("REPORT-PACKAGE-BINDING-MISMATCH: "));
}

/// The stored units and the coordinate systems are each listed once, in UTF-16
/// code unit order, as the desktop's default sort gives them; a blank or `TBD`
/// coordinate system is not one.
#[test]
fn units_and_coordinate_systems_follow_the_desktops_order() {
    let row = |id: &str, unit: &str, system: Value| json!({"id": id, "kind": "k", "value": 1, "unit": unit, "metadata": {"component": "c", "coordinate_system": system}});
    let source = json!({"producer": {}, "summary": {}, "results": [
        row("a", "\u{1F600}", json!("global")),
        row("b", "\u{FF2E}", json!("\u{a0} ")),
        row("c", "N", json!("TBD")),
        row("d", "", json!("\u{FF2E}")),
        row("e", "N", json!("\u{1F600}")),
        row("f", "mm", json!(7)),
        {"id": "g", "kind": "k", "value": 1, "unit": "mm"}
    ]});
    let model = json!({"project": {"units": {"length": "m"}}});
    let summary = unit_display_summary(&model, &source).unwrap();
    // U+1F600 is the surrogate pair D83D DE00, which sorts before U+FF2E.
    assert_eq!(summary["result_units"], json!(["N", "mm", "\u{1F600}", "\u{FF2E}"]));
    assert_eq!(summary["model_units"], json!({"length": "m"}));
    assert_eq!(summary["conversion_performed"], false);
    assert_eq!(result_coordinate_systems(&source).unwrap(), ["global", "\u{1F600}", "\u{FF2E}"]);
    // The check that binds the summary agrees with the producer on that order.
    let report = json!({"calculation_report": {"model_input_summary": {"unit_display_summary": summary}}});
    check_unit_display_summary(&report, &source).unwrap();

    // A name is kept as written: only a name that is blank after trimming, or exactly `TBD`, is dropped.
    let source = json!({"results": [row("a", "N", json!(" global ")), row("b", "N", json!(" TBD ")), row("c", "N", json!("\u{feff}"))]});
    assert_eq!(result_coordinate_systems(&source).unwrap(), [" TBD ", " global "]);

    // A row with no unit, or a null one, adds none; a unit that is not text refuses.
    let source = json!({"results": [{"id": "a", "unit": null}, {"id": "b"}, {"id": "c", "unit": "N"}]});
    assert_eq!(unit_display_summary(&model, &source).unwrap()["result_units"], json!(["N"]));
    for unit in [json!(7), json!(true), json!(["N"]), json!({"unit": "N"})] {
        let error = unit_display_summary(&model, &json!({"results": [{"id": "a", "unit": unit}]})).unwrap_err();
        assert_eq!(error, "REPORT-PACKAGE-BINDING-MISMATCH: a source row's unit is not text");
    }

    // No rows: empty members, and nothing displayed scaled.
    let empty = json!({"results": [], "diagnostics": []});
    assert_eq!(result_rows(&empty).unwrap(), Vec::<Value>::new());
    assert_eq!(result_export_refs(&empty, &json!({})).unwrap(), Vec::<Value>::new());
    assert_eq!(result_coordinate_systems(&empty).unwrap(), Vec::<String>::new());
    let summary = unit_display_summary(&model, &empty).unwrap();
    assert_eq!((&summary["result_units"], &summary["conversion_performed"]), (&json!([]), &json!(false)));
}

/// `conversion_performed` without a row stored in Pa: on an exact-route source
/// a stress headline stored in Pa is displayed in MPa; on any other source the
/// summary is not read.
#[test]
fn a_stress_headline_stored_in_pa_is_a_conversion_on_an_exact_source_only() {
    let built = built_reports().into_iter().next().unwrap();
    let exact_identity = built.source["producer"]["semantic_contract_id"].clone();
    let model = json!({"project": {"units": {}}});
    let source = |identity: Value, headline_unit: &str| {
        json!({
            "producer": {"semantic_contract_id": identity},
            "summary": {"max_open_formula_stress_reduced_section": {"value": 1.0, "unit": headline_unit}},
            "results": [{"id": "a", "kind": "k", "value": 1, "unit": "N"}]
        })
    };
    let performed = |source: &Value| unit_display_summary(&model, source).unwrap()["conversion_performed"].clone();
    assert_eq!(performed(&source(exact_identity.clone(), "Pa")), true);
    assert_eq!(performed(&source(exact_identity, "MPa")), false);
    assert_eq!(performed(&source(json!("openpipestress.result_semantics/invented"), "Pa")), false);
}

/// The section-basis token is trimmed as JavaScript trims: U+FEFF around a
/// token is removed and U+0085 is kept, so the first names a section basis and
/// the second names none.
#[test]
fn a_section_basis_token_is_trimmed_as_the_desktop_trims() {
    let row = |basis: &str| json!({"results": [{"id": "a", "kind": "k", "value": 1, "unit": "N", "metadata": {"component": "c", "basis": basis}}]});
    let label = |basis: &str| result_rows(&row(basis)).unwrap()[0]["label"].as_str().unwrap().to_string();
    assert_eq!(label("invented; section_basis=nominal_section"), "k (c) [nominal section]");
    assert_eq!(label("invented;\u{feff}section_basis=nominal_section\u{feff}"), "k (c) [nominal section]");
    assert_eq!(label("invented;\u{85}section_basis=nominal_section"), "k (c)");
    assert!(result_rows(&row("section_basis=nominal_section\u{85}")).unwrap_err().contains("unreadable section basis"));
}

/// The Diagnostics producer is the shared projection with the provenance
/// member, on every shared vector.
#[test]
fn result_diagnostics_are_the_shared_projection_with_provenance() {
    let provenance = json!({"source_name": "invented"});
    let vectors = parse(DIAGNOSTICS_VECTORS);
    let vectors = vectors["vectors"].as_array().unwrap();
    assert!(!vectors.is_empty());
    for vector in vectors {
        // A vector may state the result's combination evidence, rows and numerical cases.
        let mut source = json!({"diagnostics": vector["diagnostics"]});
        for member in ["contract_evidence", "results", "numerical_quality"] {
            if let Some(value) = vector.get(member) {
                source[member] = value.clone();
            }
        }
        let produced = result_diagnostics(&source, &provenance).unwrap();
        let expected: Vec<Value> = vector["expected"]
            .as_array()
            .unwrap()
            .iter()
            .map(|entry| {
                let mut entry = entry.clone();
                entry["provenance"] = provenance.clone();
                entry
            })
            .collect();
        assert_canonical_eq(&Value::Array(produced), &Value::Array(expected), vector["reason"].as_str().unwrap());
    }
}

/// The result document and its derivative hash, from the committed model,
/// source, run record and input manifest: the desktop assembly's document,
/// and the hash the exact report's admission binds.
#[test]
fn the_result_document_is_the_desktop_assemblys() {
    for (name, text) in USER_RELEASE_EXACT {
        let fixture = parse(text);
        let request = &fixture["request"];
        let manifest_ref = request["audit_manifest"]["manifest_id"].as_str().unwrap();
        let document =
            result_document(&fixture["model"], &fixture["source"], &fixture["run_record"], manifest_ref, &fixture["input_manifest"]["solver_basis"])
                .unwrap_or_else(|error| panic!("{name}: {error}"));
        assert_canonical_eq(&document, &request["result_documents"][0], name);
        assert_eq!(derivative_sha256(&document).unwrap(), request["report"]["exact_route"]["admission"]["derivative_sha256"], "{name}");
        // The run record's row references are not read.
        let mut header_only = fixture["run_record"].clone();
        header_only["analysis_run"]["result_refs"] = json!([]);
        let again = result_document(&fixture["model"], &fixture["source"], &header_only, manifest_ref, &fixture["input_manifest"]["solver_basis"]).unwrap();
        assert_eq!(canonical_json(&again), canonical_json(&document), "{name}");
    }
    let model = parse(VECTOR_MODEL);
    for vector in parse(USER_RELEASE_VECTORS)["vectors"].as_array().unwrap() {
        let path = vector["source_fixture"].as_str().unwrap();
        let source = parse(VECTOR_SOURCES.iter().find(|(name, _)| *name == path).unwrap().1);
        let manifest_ref = vector["expected"]["input_manifest_ref"].as_str().unwrap();
        let document = result_document(&model, &source, &vector["run_record"], manifest_ref, &vector["input_manifest"]["solver_basis"]).unwrap();
        assert_eq!(derivative_sha256(&document).unwrap(), vector["expected"]["result_document_derivative_sha256"], "{}", vector["name"]);
        // The whole document, by the digest the desktop's builder recorded for it.
        assert_eq!(digest(&document).unwrap(), vector["expected"]["result_document_sha256"].as_str().unwrap(), "{}", vector["name"]);
    }
    assert!(derivative_sha256(&json!({})).unwrap_err().starts_with("REPORT-PACKAGE-BINDING-MISMATCH: "));
}

/// A stated limit: a run record that carries a diagnostic the source does not
/// (a rule-pack reference exclusion) has no native document. The desktop's
/// assembly accepts such a run and adds a document diagnostic for it.
#[test]
fn a_run_record_with_a_rule_pack_exclusion_has_no_native_document() {
    let fixture = parse(USER_RELEASE_EXACT[0].1);
    let manifest_ref = fixture["request"]["audit_manifest"]["manifest_id"].as_str().unwrap();
    // The run record's diagnostic for an excluded reference, as the desktop's
    // `rulePackRefDiagnostic` and the host's `rule_pack_ref_diagnostic` write it.
    let identity = "invented-pack@1.0.0";
    let exclusion = json!({"source_annotation": {
        "id": format!("diagnostic:rule-pack-ref:{identity}"),
        "code": "MODEL-RULE-PACK-REF-UNRESOLVED",
        "severity": "warning",
        "message": format!("Rule pack {identity} is referenced by the model, but the local rule-pack store held no such pack for this project at solve time; it is not among this run's rule-pack references."),
        "source": "desktop/solve-rule-pack-resolution",
        "affected_refs": [identity]
    }});
    let mut run_record = fixture["run_record"].clone();
    run_record["analysis_run"]["diagnostics"].as_array_mut().unwrap().push(exclusion);
    let error = result_document(&fixture["model"], &fixture["source"], &run_record, manifest_ref, &fixture["input_manifest"]["solver_basis"]).unwrap_err();
    assert_eq!(
        error,
        "REPORT-USER-RELEASE-BASIS-INVALID: the run record carries diagnostics the source does not (a rule-pack reference exclusion is outside scope R)"
    );
    // The same run record without it has a document.
    result_document(&fixture["model"], &fixture["source"], &fixture["run_record"], manifest_ref, &fixture["input_manifest"]["solver_basis"]).unwrap();
}
