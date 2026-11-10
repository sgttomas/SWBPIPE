//! The proposed projection-size bound B_doc of Design §5.3.8 ("Projection
//! size: a proposed bound from format and input lengths",
//! `execution/PKG-04/DEL-04-09/Work/modal-analysis.md`), computed reader-side
//! from each committed document and asserted against its canonical length.
//!
//! This is a transcription of the Design's numbers, not of `document.rs`: the
//! row skeletons S_t(8), the wrapper 121 + d, the 204 B of table keys, the
//! header's H₀ = 8,450 B and its variable fields. The identities each bound
//! adds are read from the document's own references. Where the Design gives
//! no skeleton (supports and diagnostics rows) the guard takes the exact
//! length, and so does this test; those rows check nothing. A refused
//! document is not bounded by the Design (open item 4), so it is measured and
//! reported, not asserted.
//!
//! B_doc stays a proposed bound: this test is evidence toward its open
//! items 1 and 2, not its adoption.

use open_pipe_stress_canonical_json::{canonical_json, canonical_json_checked_v1_text};
use serde_json::Value;

const FIXTURES: [(&str, &str); 12] = [
    ("modal_ex_n05", include_str!("../../../../fixtures/results/modal_ex_n05.json")),
    ("modal_arc", include_str!("../../../../fixtures/results/modal_arc.json")),
    ("modal_component_body", include_str!("../../../../fixtures/results/modal_component_body.json")),
    ("modal_state_one_way_pressed", include_str!("../../../../fixtures/results/modal_state_one_way_pressed.json")),
    ("modal_state_friction_sliding", include_str!("../../../../fixtures/results/modal_state_friction_sliding.json")),
    ("modal_cantilever_whole_spectrum", include_str!("../../../../fixtures/results/modal_cantilever_whole_spectrum.json")),
    ("modal_cantilever_cluster_extension", include_str!("../../../../fixtures/results/modal_cantilever_cluster_extension.json")),
    ("modal_cantilever_below_cutoff", include_str!("../../../../fixtures/results/modal_cantilever_below_cutoff.json")),
    ("modal_cantilever_subnormal_cutoff", include_str!("../../../../fixtures/results/modal_cantilever_subnormal_cutoff.json")),
    ("modal_cantilever_sensitive", include_str!("../../../../fixtures/results/modal_cantilever_sensitive.json")),
    ("modal_refused_unseparated", include_str!("../../../../fixtures/results/modal_refused_unseparated.json")),
    ("modal_refused_no_bound", include_str!("../../../../fixtures/results/modal_refused_no_bound.json")),
];

/// D = 8: the Design's skeletons hold for documents whose integers and
/// reference indices have at most 8 digits.
const D_LIMIT: u64 = 100_000_000;

/// S_t(8) per row, from the Design's table.
const S_SHAPES: usize = 379;
const S_MASS_MATRIX: usize = 407;
const S_DOFS: usize = 175;
const S_NODES: usize = 596;
const S_MASS_TOTALS: usize = 525;
const S_MODES: usize = 1_360;
const S_PARTICIPATION: usize = 958;
const S_CLUSTERS: usize = 423;
const S_COMPUTED: usize = 1_145;
const S_STURM: usize = 2_394;
const S_IDENTITY: usize = 383;
const S_DISCRETIZATION: usize = 666;
const S_DISCLOSURE: usize = 163;

/// Each table's wrapper is 121 + d; the `tables` object's 15 keys 204 B.
const TABLE_WRAPPER: usize = 121;
const TABLE_KEYS: usize = 204;

/// The header's fixed part and its template-bounded members.
const H0: usize = 8_450;
const MODE_REASON: usize = 110;
const ORTHOGONALITY_REASON: usize = 128;
const PARAMETER_ELEMENT: usize = 26;

/// E(x): serde_json's escape of x, without its quotes.
fn e(text: &str) -> usize {
    serde_json::to_string(text).unwrap().len() - 2
}

fn s<'a>(row: &'a Value, key: &str) -> &'a str {
    row[key].as_str().unwrap_or_else(|| panic!("{key} is not a string in {row}"))
}

/// The node identity inside a DOF id `dof:<node>:<component>`.
fn dof_node(dof: &str) -> &str {
    let rest = dof.strip_prefix("dof:").unwrap_or_else(|| panic!("{dof} is not a DOF id"));
    &rest[..rest.rfind(':').unwrap()]
}

/// The node identity inside a node reference `node:<node>`.
fn node_ref_node(node_ref: &str) -> &str {
    node_ref.strip_prefix("node:").unwrap_or_else(|| panic!("{node_ref} is not a node reference"))
}

/// The bound of one row, or None where the Design gives no skeleton and the
/// guard takes the row's exact length.
fn row_bound(table: &str, row: &Value) -> Option<usize> {
    Some(match table {
        "shapes" => S_SHAPES + 2 * e(dof_node(s(row, "dof_ref"))),
        "mass_matrix" => S_MASS_MATRIX + 2 * e(dof_node(s(row, "row_dof_ref"))) + 2 * e(dof_node(s(row, "col_dof_ref"))),
        "dofs" => S_DOFS + 2 * e(node_ref_node(s(row, "node_ref"))),
        "nodes" => S_NODES + 2 * e(s(row, "node_id")),
        "mass_totals" => S_MASS_TOTALS,
        "modes" => {
            // The skeleton carries one Sensitive reason.
            let reasons = row["standing"]["reasons"].as_array().unwrap();
            assert!(reasons.len() <= 1, "a mode row carries {} reasons", reasons.len());
            S_MODES + e(dof_node(s(row, "reference_dof_ref")))
        }
        "participation" => S_PARTICIPATION,
        "clusters" => S_CLUSTERS,
        "computed_modes" => S_COMPUTED,
        "sturm_candidates" => S_STURM,
        "identity" => S_IDENTITY + e(s(row, "subject")),
        "discretization" => S_DISCRETIZATION + 2 * e(s(row, "member_ref")),
        "disclosures" => S_DISCLOSURE + e(s(row, "text")),
        "supports" | "diagnostics" => return None,
        other => panic!("table {other} is not one of the Design's"),
    })
}

fn digits(n: usize) -> usize {
    n.to_string().len()
}

fn assert_d8(value: &Value, where_: &str) {
    match value {
        Value::Number(n) => {
            if let Some(u) = n.as_u64() {
                assert!(u < D_LIMIT, "{where_}: integer {u} exceeds D = 8");
            }
        }
        Value::Array(items) => items.iter().for_each(|v| assert_d8(v, where_)),
        Value::Object(map) => map.values().for_each(|v| assert_d8(v, where_)),
        _ => {}
    }
}

/// H: H₀ plus the Design's variable header fields, read from the document.
fn header_bound(doc: &Value, reported: usize) -> usize {
    let mut h = H0;
    h += e(s(&doc["admission"], "parameter_label"));
    h += canonical_json(&doc["binding"]["options"]).len() - 2;
    if let Some(case) = doc["binding"]["state_case"]["case_id"].as_str() {
        h += 2 * e(case);
    }
    if let Some(refs) = doc["basis"]["state_case"]["restrained_dof_refs"].as_array() {
        h += refs.iter().map(|r| e(dof_node(r.as_str().unwrap())) + 10).sum::<usize>();
    }
    for key in ["stiffness_row_ref", "mass_row_ref"] {
        if let Some(dof) = doc["formation"][key].as_str() {
            h += e(dof_node(dof));
        }
    }
    let parameters = &doc["binding"]["integrity_parameters"];
    for key in ["sturm_gap_log_positions", "sturm_whole_spectrum_offsets"] {
        h += PARAMETER_ELEMENT * parameters[key].as_array().unwrap().len();
    }
    let reasons = doc["standing"]["reasons"].as_array().unwrap();
    assert!(reasons.len() <= reported + 1, "{} header reasons for {reported} modes", reasons.len());
    for reason in reasons {
        assert!(e(reason.as_str().unwrap()) + 2 <= ORTHOGONALITY_REASON, "header reason {reason} exceeds its template");
    }
    h + reported * (MODE_REASON + 1) + ORTHOGONALITY_REASON + 1
}

struct Totals {
    rows: usize,
    tables: usize,
    headers: usize,
    min_row_margin: Option<(isize, String)>,
}

#[test]
fn b_doc_bounds_every_committed_solved_fixture() {
    let mut totals = Totals { rows: 0, tables: 0, headers: 0, min_row_margin: None };
    let mut header_margins = Vec::new();
    let mut document_ratios = Vec::new();
    for (name, text) in FIXTURES {
        let canonical = canonical_json_checked_v1_text(text).unwrap();
        let mut doc: Value = serde_json::from_str(text).unwrap();
        if doc["status"] != "solved" {
            let refusals = doc["refusals"]["rows"].as_array().map_or(0, Vec::len);
            println!("B_doc: {name} is refused ({} canonical bytes, {refusals} refusal rows): not bounded by the Design (open item 4)", canonical.len());
            continue;
        }
        let tables = doc["tables"].as_object().unwrap().clone();
        assert_eq!(tables.len(), 15, "{name}: the Design prices 15 table keys");
        let reported = tables["modes"]["rows"].as_array().unwrap().len();
        let mut bound = TABLE_KEYS;
        for (table, wrapper) in &tables {
            let rows = wrapper["rows"].as_array().unwrap();
            assert!((rows.len() as u64) < D_LIMIT);
            assert_d8(&Value::Array(rows.clone()), &format!("{name}/{table}"));
            let actual_table = canonical_json(wrapper).len();
            let mut table_bound = TABLE_WRAPPER + digits(rows.len());
            for (i, row) in rows.iter().enumerate() {
                let actual = canonical_json(row).len();
                let row_bound = match row_bound(table, row) {
                    Some(b) => {
                        totals.rows += usize::from(!matches!(table.as_str(), "disclosures"));
                        let margin = b as isize - actual as isize;
                        assert!(margin >= 0, "{name}/{table}/{i}: {actual} canonical bytes over its bound {b}");
                        if totals.min_row_margin.as_ref().map_or(true, |(m, _)| margin < *m) {
                            totals.min_row_margin = Some((margin, format!("{name}/{table}/{i}")));
                        }
                        b
                    }
                    None => actual,
                };
                table_bound += row_bound + 1;
            }
            assert!(actual_table <= table_bound, "{name}/{table}: {actual_table} over {table_bound}");
            totals.tables += 1;
            bound += table_bound;
        }
        doc["tables"] = Value::Object(Default::default());
        let header_actual = canonical_json(&doc).len();
        let header = header_bound(&doc, reported);
        assert!(header_actual <= header, "{name}: header {header_actual} over {header}");
        totals.headers += 1;
        header_margins.push(header - header_actual);
        bound += header;
        assert!(canonical.len() <= bound, "{name}: {} canonical bytes over B_doc {bound}", canonical.len());
        document_ratios.push((canonical.len() as f64 / bound as f64, name));
    }
    let (min_margin, at) = totals.min_row_margin.unwrap();
    let (lo, hi) = (header_margins.iter().min().unwrap(), header_margins.iter().max().unwrap());
    let (r_lo, r_hi) = document_ratios.iter().fold((f64::MAX, 0f64), |(a, b), (r, _)| (a.min(*r), b.max(*r)));
    println!(
        "B_doc: {} rows with a stated skeleton over 12 tables, plus disclosures; {} tables; {} headers; \
         smallest row margin {min_margin} B at {at}; header margins {lo}–{hi} B; canonical/B_doc {r_lo:.3}–{r_hi:.3}",
        totals.rows, totals.tables, totals.headers
    );
    assert_eq!(totals.headers, 10);
}

/// Open item 2, the part the reader's toolchain can show: `{:e}` of a
/// binary64 is at most 24 B (sign, 17 significant digits, point, `e-308`).
#[test]
fn exponent_format_of_a_binary64_is_at_most_24_bytes() {
    let mut longest = (0, 0f64);
    let mut probe = |x: f64| {
        let n = format!("{x:e}").len();
        if n > longest.0 {
            longest = (n, x);
        }
    };
    for x in [f64::MAX, f64::MIN, f64::MIN_POSITIVE, -f64::MIN_POSITIVE, 5e-324, -5e-324, -2.2250738585072014e-308, 0.0, -0.0] {
        probe(x);
    }
    let mut state: u64 = 0x9e37_79b9_7f4a_7c15;
    for _ in 0..2_000_000 {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        let x = f64::from_bits(state);
        if x.is_finite() {
            probe(x);
        }
    }
    println!("{{:e}}: longest {} B at {:e}", longest.0, longest.1);
    assert!(longest.0 <= 24);
}
