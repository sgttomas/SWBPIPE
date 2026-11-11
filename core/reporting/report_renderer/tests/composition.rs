//! The report composer's contract (DEL-08-01 Design
//! `report-section-composer.md` §2, slices C1-C2): the family table against
//! the semantic contract tables, the shared selection and refusal vectors,
//! the row-field refusals, exact magnitudes and the composition's canonical
//! bytes. Invented data only. Standard claim fence applies (claims taxonomy
//! per DEC-081).

use open_pipe_stress_report_renderer::composer::{
    compose, composition_jcs, composition_sha256, exact_magnitude, family_table,
    parse_family_table, ReportComposition, CATALOGUE, COMPOSITION_ROW_FIELDS_INVALID,
    RESULT_OTHER_ROWS,
};
use open_pipe_stress_report_renderer::ResultRow;
use serde_json::Value;
use std::collections::BTreeMap;

const VECTORS: &str =
    include_str!("../../../../fixtures/reports/invented/report_composition_vectors.json");
const SEMANTIC_TABLES: [&str; 6] = [
    include_str!("../../../../fixtures/results/semantic_contract_v0_3_physics_retained_1.json"),
    include_str!("../../../../fixtures/results/semantic_contract_v0_3_pressure_2.json"),
    include_str!("../../../../fixtures/results/semantic_contract_v0_3_pressure_source_1.json"),
    include_str!("../../../../fixtures/results/semantic_contract_v0_3_preview_physics_2.json"),
    include_str!(
        "../../../../fixtures/results/semantic_contract_v0_3_preview_physics_retained_1.json"
    ),
    include_str!(
        "../../../../fixtures/results/semantic_contract_v0_3_preview_physics_timoshenko_1.json"
    ),
];

fn vectors() -> Value {
    serde_json::from_str(VECTORS).unwrap()
}

fn rows(vectors: &Value) -> Vec<ResultRow> {
    serde_json::from_value(vectors["rows"].clone()).unwrap()
}

fn composition(value: &Value) -> ReportComposition {
    serde_json::from_value(value.clone()).unwrap()
}

fn codes(refusals: &[String]) -> Vec<&str> {
    refusals
        .iter()
        .map(|reason| reason.split(':').next().unwrap())
        .collect()
}

/// Every kind the current producers emit with a mechanics family is mapped
/// to that family's section, and nothing else is listed: the table reflects
/// the actual kinds.
#[test]
fn the_family_table_is_the_semantic_tables_family_column() {
    let section_of_family = BTreeMap::from([
        ("displacement", "result_displacements"),
        ("rotation", "result_displacements"),
        ("force", "result_forces_moments"),
        ("moment", "result_forces_moments"),
        ("stress", "result_stresses"),
        ("reaction", "result_support_loads"),
    ]);
    let mut expected: BTreeMap<String, &str> = BTreeMap::new();
    let mut unmapped = Vec::new();
    for text in SEMANTIC_TABLES {
        let table: Value = serde_json::from_str(text).unwrap();
        for row in table["rows"].as_array().unwrap() {
            let kind = row["kind"].as_str().unwrap().to_string();
            match row["family"].as_str() {
                Some(family) => {
                    let section = section_of_family[family];
                    assert_eq!(
                        *expected.entry(kind.clone()).or_insert(section),
                        section,
                        "{kind} has two families"
                    );
                }
                None => unmapped.push(kind),
            }
        }
    }
    let table = family_table().unwrap();
    let listed: BTreeMap<String, &str> = table
        .entries()
        .map(|(kind, section)| (kind.to_string(), section))
        .collect();
    assert_eq!(listed, expected);
    assert!(!unmapped.is_empty());
    for kind in unmapped.iter().filter(|kind| !expected.contains_key(*kind)) {
        assert_eq!(table.section_of(kind), RESULT_OTHER_ROWS, "{kind}");
    }
    assert_eq!(
        table.section_of("spring_hanger_user_input_review"),
        RESULT_OTHER_ROWS
    );
}

#[test]
fn a_family_table_that_does_not_partition_is_refused() {
    for text in [
        r#"{"schema_version":"1.0.0","sections":{"result_stresses":["k"],"result_forces_moments":["k"]}}"#,
        r#"{"schema_version":"1.0.0","sections":{"result_other_rows":["k"]}}"#,
        r#"{"schema_version":"1.0.0","sections":{"results":["k"]}}"#,
        r#"{"schema_version":"2.0.0","sections":{}}"#,
        r#"{"schema_version":"1.0.0","sections":{"result_stresses":[""]}}"#,
    ] {
        assert!(
            parse_family_table(text)
                .unwrap_err()
                .starts_with("REPORT-COMPOSITION-FAMILIES-INVALID"),
            "{text}"
        );
    }
}

#[test]
fn the_shared_selection_vectors_hold() {
    let vectors = vectors();
    let rows = rows(&vectors);
    let selection = vectors["selection_vectors"].as_array().unwrap();
    assert!(selection.len() >= 7);
    for vector in selection {
        let name = vector["name"].as_str().unwrap();
        let composed = compose(&rows, &composition(&vector["composition"]))
            .unwrap_or_else(|refusals| panic!("{name}: {refusals:?}"));
        let found: Vec<Value> = composed
            .sections
            .iter()
            .map(|section| {
                serde_json::json!({
                    "section_id": section.entry.id,
                    "row_ids": section.rows.iter().map(|index| rows[*index].row_id.clone()).collect::<Vec<_>>(),
                })
            })
            .collect();
        assert_eq!(Value::Array(found), vector["expected_sections"], "{name}");
        assert_eq!(
            composed.shown_rows() as u64,
            vector["shown"].as_u64().unwrap(),
            "{name}"
        );
        assert_eq!(
            composed.bound_rows as u64,
            vectors["bound_rows"].as_u64().unwrap(),
            "{name}"
        );
        // The disclosure line is never selected, ranked or counted.
        let disclosures: Vec<&str> = composed
            .disclosure_rows
            .iter()
            .map(|index| rows[*index].row_id.as_str())
            .collect();
        assert_eq!(disclosures, ["numerical-health:A"], "{name}");
        // Each row at most once.
        let mut all: Vec<usize> = composed
            .sections
            .iter()
            .flat_map(|section| section.rows.clone())
            .collect();
        let shown = all.len();
        all.sort();
        all.dedup();
        assert_eq!(all.len(), shown, "{name}");
    }
}

#[test]
fn the_shared_refusal_vectors_refuse_with_exactly_their_codes() {
    let vectors = vectors();
    let rows = rows(&vectors);
    for vector in vectors["refusal_vectors"].as_array().unwrap() {
        let name = vector["name"].as_str().unwrap();
        let refusals = compose(&rows, &composition(&vector["composition"])).expect_err(name);
        let expected: Vec<&str> = vector["codes"]
            .as_array()
            .unwrap()
            .iter()
            .map(|code| code.as_str().unwrap())
            .collect();
        assert_eq!(codes(&refusals), expected, "{name}: {refusals:?}");
    }
}

/// Each refusal code fires on its vector and on no selection vector (K5).
#[test]
fn every_refusal_code_has_a_vector() {
    let vectors = vectors();
    let fired: std::collections::BTreeSet<&str> = vectors["refusal_vectors"]
        .as_array()
        .unwrap()
        .iter()
        .flat_map(|vector| {
            vector["codes"]
                .as_array()
                .unwrap()
                .iter()
                .map(|code| code.as_str().unwrap())
        })
        .collect();
    for code in [
        "REPORT-COMPOSITION-VERSION-UNSUPPORTED",
        "REPORT-COMPOSITION-SECTION-UNKNOWN",
        "REPORT-COMPOSITION-SECTION-DUPLICATE",
        "REPORT-COMPOSITION-OPTION-INVALID",
        "REPORT-COMPOSITION-CASE-UNKNOWN",
        "REPORT-COMPOSITION-ROWS-OVERLAP",
    ] {
        assert!(fired.contains(code), "{code}");
    }
}

#[test]
fn a_row_with_part_of_its_bound_members_or_an_unreadable_value_is_refused() {
    let vectors = vectors();
    let all = composition(&vectors["selection_vectors"][0]["composition"]);
    let base = rows(&vectors);
    let mut cases: Vec<(&str, Vec<ResultRow>)> = Vec::new();
    let mut partial = base.clone();
    partial[0].unit = None;
    cases.push(("unit absent", partial));
    let mut entity_only = base.clone();
    entity_only[16].entity_ref = Some("node:N-1".to_string());
    cases.push(("a disclosure line with an entity", entity_only));
    for text in [
        "1.",
        ".5",
        "01",
        "1e",
        "--1",
        "1,5",
        "NaN",
        "Infinity",
        "",
        "1e99999999999999999999",
    ] {
        let mut unreadable = base.clone();
        unreadable[5].value_text = Some(text.to_string());
        cases.push(("unreadable value_text", unreadable));
    }
    for (name, rows) in cases {
        let refusals = compose(&rows, &all).expect_err(name);
        assert!(
            codes(&refusals)
                .iter()
                .all(|code| *code == COMPOSITION_ROW_FIELDS_INVALID),
            "{name}: {refusals:?}"
        );
    }
}

#[test]
fn magnitudes_are_exact_decimals_in_the_display_unit() {
    let magnitude =
        |text: &str, shift: i64| exact_magnitude(text, shift).unwrap_or_else(|| panic!("{text}"));
    // One value, several texts.
    assert_eq!(magnitude("1e-7", 0), magnitude("0.0000001", 0));
    assert_eq!(magnitude("-0", 0), magnitude("0", 0));
    assert_eq!(magnitude("0.000", 0), magnitude("0", 0));
    assert_eq!(magnitude("1.5E+2", 0), magnitude("150", 0));
    // Pa compared in MPa, exactly.
    assert_eq!(magnitude("1234567.5", 6), magnitude("1.2345675", 0));
    assert!(magnitude("1.234574", 0) > magnitude("1234567.5", 6));
    // Beyond binary64 resolution: decimals, not floats.
    assert!(magnitude("0.30000000000000004", 0) > magnitude("0.3", 0));
    assert!(magnitude("9007199254740993", 0) > magnitude("9007199254740992", 0));
    assert!(magnitude("5e-324", 0) > magnitude("0", 0));
    assert!(magnitude("10", 0) > magnitude("9.99999999999", 0));
    assert!(magnitude("-12", 0) > magnitude("11.9", 0));
}

#[test]
fn the_composition_digest_is_over_its_rfc_8785_bytes() {
    let composition: ReportComposition = serde_json::from_value(serde_json::json!({
        "sections": [
            {"view": {"limit": 25, "kind": "largest"}, "section_id": "result_stresses", "cases": "all"},
            {"section_id": "result_support_loads", "cases": ["load_case:é\"\n", "load_case:A"], "view": {"kind": "all"}}
        ],
        "composition_version": "1.0.0"
    }))
    .unwrap();
    assert_eq!(
        composition_jcs(&composition),
        "{\"composition_version\":\"1.0.0\",\"sections\":[{\"cases\":\"all\",\"section_id\":\"result_stresses\",\"view\":{\"kind\":\"largest\",\"limit\":25}},{\"cases\":[\"load_case:é\\\"\\n\",\"load_case:A\"],\"section_id\":\"result_support_loads\",\"view\":{\"kind\":\"all\"}}]}"
    );
    assert_eq!(
        composition_sha256(&composition),
        "b6b2436b1ae53ce33d36d80f21e8a904c1c9c1c3ae27e25845858c61c37b8498"
    );
}

#[test]
fn the_composition_is_closed_at_every_depth() {
    for value in [
        serde_json::json!({"composition_version": "1.0.0", "sections": [], "title": "x"}),
        serde_json::json!({"composition_version": "1.0.0", "sections": [{"section_id": "result_stresses", "cases": "all", "view": {"kind": "all"}, "heading": "x"}]}),
        serde_json::json!({"composition_version": "1.0.0", "sections": [{"section_id": "result_stresses", "cases": "all", "view": {"kind": "all", "unit": "psi"}}]}),
        serde_json::json!({"composition_version": "1.0.0", "sections": [{"section_id": "result_stresses", "cases": "load_case:A", "view": {"kind": "all"}}]}),
        serde_json::json!({"composition_version": "1.0.0", "sections": [{"section_id": "result_stresses", "cases": [1], "view": {"kind": "all"}}]}),
        serde_json::json!({"composition_version": "1.0.0", "sections": [{"section_id": "result_stresses", "cases": "all", "view": {"kind": "largest", "limit": -1}}]}),
        serde_json::json!({"composition_version": "1.0.0", "sections": [{"section_id": "result_stresses", "cases": "all", "view": {"kind": "largest", "limit": 2.5}}]}),
        serde_json::json!({"composition_version": "1.0.0", "sections": [{"section_id": "result_stresses", "cases": "all"}]}),
    ] {
        assert!(
            serde_json::from_value::<ReportComposition>(value.clone()).is_err(),
            "{value}"
        );
    }
}

#[test]
fn the_catalogue_is_the_six_result_sections_then_the_five_model_input_sections() {
    let entries: Vec<(&str, &str)> = CATALOGUE.iter().map(|entry| (entry.id, entry.parent)).collect();
    assert_eq!(
        entries,
        [
            ("result_rows_all", "results"),
            ("result_displacements", "results"),
            ("result_forces_moments", "results"),
            ("result_stresses", "results"),
            ("result_support_loads", "results"),
            ("result_other_rows", "results"),
            ("load_combinations", "load_cases"),
            ("input_geometry", "model_input_summary"),
            ("input_materials_sections", "model_input_summary"),
            ("input_supports", "model_input_summary"),
            ("input_loads", "model_input_summary"),
        ]
    );
}

/// Slice C5: a model-input section takes all cases and the `all` view
/// only, shows no result row and is never an overlap with `result_rows_all`.
#[test]
fn model_input_sections_take_only_all_cases_and_the_all_view() {
    use open_pipe_stress_report_renderer::composer::form_refusals;
    use open_pipe_stress_report_renderer::model_input::MODEL_INPUT_SECTIONS;
    let vectors = vectors();
    let rows = rows(&vectors);
    for id in MODEL_INPUT_SECTIONS {
        let admitted = composition(&serde_json::json!({"composition_version": "1.0.0", "sections": [
            {"section_id": "result_rows_all", "cases": "all", "view": {"kind": "all"}},
            {"section_id": id, "cases": "all", "view": {"kind": "all"}}
        ]}));
        assert!(form_refusals(&admitted).is_empty(), "{id}");
        let composed = compose(&rows, &admitted).unwrap();
        assert!(composed.sections[1].rows.is_empty(), "{id}");
        assert_eq!(composed.shown_rows(), composed.bound_rows, "{id}");
        for (cases, view) in [
            (serde_json::json!(["load_case:A"]), serde_json::json!({"kind": "all"})),
            (serde_json::json!([]), serde_json::json!({"kind": "all"})),
            (serde_json::json!("all"), serde_json::json!({"kind": "extremes"})),
            (serde_json::json!("all"), serde_json::json!({"kind": "largest", "limit": 5})),
            (serde_json::json!("all"), serde_json::json!({"kind": "all", "limit": 5})),
        ] {
            let refused = composition(&serde_json::json!({"composition_version": "1.0.0", "sections": [
                {"section_id": id, "cases": cases, "view": view}
            ]}));
            let refusals = form_refusals(&refused);
            assert_eq!(codes(&refusals), ["REPORT-COMPOSITION-OPTION-INVALID"], "{id} {cases} {view}: {refusals:?}");
            assert_eq!(codes(&compose(&rows, &refused).unwrap_err()), ["REPORT-COMPOSITION-OPTION-INVALID"], "{id}");
        }
    }
}
