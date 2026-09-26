//! Admission of the closed preview-physics-1 evidence namespace (T0R S1 section 9).
//! These checks establish internal consistency of a received envelope, not
//! solver accuracy, origin, model freshness or engineering acceptance.
use serde_json::Value;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

type Check = Result<(), String>;

const CASE_KEYS: &[&str] = &[
    "load_case_id",
    "pipe_stress_extrema",
    "stress_maximum_coverage",
    "support_attribution",
    "intensified_measures",
];
const EXTREMA_KEYS: &[&str] = &[
    "pipe_id",
    "result_id",
    "station_fraction",
    "span_index",
    "local_fraction",
    "value_lower_pa",
    "value_upper_pa",
    "global_upper_bound_pa",
    "certified_gap_pa",
    "subdivisions",
    "approximation",
    "coefficient_basis",
    "enclosure_scope",
];
const INTENSIFIED_KEYS: &[&str] = &[
    "result_id",
    "component_id",
    "pipe_id",
    "location",
    "factor_role",
    "sif",
    "sif_source_reference",
    "section_modulus_m3",
    "bending_moment_y_n_m",
    "bending_moment_z_n_m",
];
const METADATA_KEYS: &[&str] = &[
    "component",
    "coordinate_system",
    "location",
    "basis",
    "sign_convention",
];
const RETIRED_CODES: &[&str] = &[
    "COMPONENT_STRESS_MULTIPLIER_APPLIED",
    "COMBINATION_STRESS_SUMMARY_SKIPPED",
];
const WITHHELD_REASONS: &[&str] = &[
    "SUPPORT_ACTION_ATTRIBUTION_WITHHELD",
    "CONSTANT_EFFORT_NOT_CONSUMED",
];
const GATE_CODES: &[&str] = &[
    "NONLINEAR_COMBINATION_REQUIRES_SOLVE",
    "CONSTANT_EFFORT_COMBINATION_REQUIRES_SOLVE",
    "COMBINATION_MODULUS_BASIS_MIXED",
];
const SUPPORT_COMPONENTS: [&str; 8] = [
    "Fx",
    "Fy",
    "Fz",
    "Mx",
    "My",
    "Mz",
    "force_magnitude",
    "moment_magnitude",
];
/// `MAX_SUBDIVISIONS` of core/loads/stress_recovery/src/elastic_extrema.rs (A2 item 1).
const MAX_SUBDIVISIONS: u64 = 131072;
const MAXIMUM: &str = "pipe_elastic_normal_stress_maximum_v2";
const INTENSIFIED: &str = "component_equal_factor_intensified_bending_stress_v1";
const SUPPORT_COMPONENT: &str = "support_reaction_component_v2";
const SUPPORT_FORCE: &str = "support_reaction_force_magnitude_v2";
const SUPPORT_MOMENT: &str = "support_reaction_moment_magnitude_v2";
const SUPPORT_SIGN: &str = "support-on-pipe; positive global force and right-hand couple about attached node; force and moment norms remain separate";
const MAXIMUM_SIGN: &str = "nonnegative circumferential maximum |Nw/As|+hypot(My,Mz)/Z; bounded over all straight statics intervals; torsional shear remains separate; no code stress or equivalent stress claim";
const INTENSIFIED_SIGN_PREFIX: &str = "nonnegative i*hypot(My,Mz)/Z at the member end; i=";
const RANGE_BASIS: &str = "explicit_user_range_envelope";

fn require(ok: bool, code: &str) -> Check {
    if ok {
        Ok(())
    } else {
        Err(format!("SOURCE_PREVIEW_PHYSICS_{code}"))
    }
}
fn keys(v: &Value, required: &[&str]) -> bool {
    v.as_object()
        .is_some_and(|o| o.len() == required.len() && required.iter().all(|k| o.contains_key(*k)))
}
fn text(v: &Value) -> Result<&str, String> {
    v.as_str()
        .filter(|s| !s.is_empty())
        .ok_or_else(|| "SOURCE_PREVIEW_PHYSICS_STRING_INVALID".into())
}
fn number(v: &Value) -> Result<f64, String> {
    v.as_f64()
        .filter(|n| n.is_finite())
        .ok_or_else(|| "SOURCE_PREVIEW_PHYSICS_NUMBER_INVALID".into())
}
fn array(v: &Value) -> Result<&Vec<Value>, String> {
    v.as_array()
        .ok_or_else(|| "SOURCE_PREVIEW_PHYSICS_ARRAY_INVALID".into())
}
fn strings(v: &Value) -> Result<Vec<&str>, String> {
    let mut seen = HashSet::new();
    let mut out = Vec::new();
    for item in array(v)? {
        let s = text(item)?;
        require(seen.insert(s), "DUPLICATE_ID")?;
        out.push(s);
    }
    Ok(out)
}
fn finite_tree(v: &Value) -> Check {
    match v {
        Value::Number(_) => {
            number(v)?;
        }
        Value::Array(a) => {
            for x in a {
                finite_tree(x)?;
            }
        }
        Value::Object(o) => {
            for x in o.values() {
                finite_tree(x)?;
            }
        }
        _ => {}
    }
    Ok(())
}
/// `{L(part)}:{part}` with L the UTF-8 byte length (S1 section 1).
fn identity(parts: &[&str]) -> String {
    parts
        .iter()
        .map(|p| format!("{}:{p}", p.len()))
        .collect::<Vec<_>>()
        .join(":")
}
/// Representation guard only (64 epsilon relative), not an engineering tolerance.
fn guarded(published: f64, recomputed: f64) -> bool {
    (published - recomputed).abs() <= 64.0 * f64::EPSILON * published.abs().max(f64::MIN_POSITIVE)
}
fn case_of(row: &Value) -> Option<&str> {
    let b = &row["basis_ref"];
    (b["ref_type"] == "load_case")
        .then(|| b["ref_id"].as_str())
        .flatten()
}
fn combination_of(row: &Value) -> Option<&str> {
    let b = &row["basis_ref"];
    (b["ref_type"] == "combination")
        .then(|| b["ref_id"].as_str())
        .flatten()
}
fn component(row: &Value) -> &str {
    row["metadata"]["component"].as_str().unwrap_or("")
}
/// Headline order (S1 section 6): larger value, then smaller load case id,
/// then smaller location id, both in byte order.
fn governs(a: (f64, &str, &str), b: (f64, &str, &str)) -> bool {
    match a.0.partial_cmp(&b.0) {
        Some(Ordering::Greater) => true,
        Some(Ordering::Less) => false,
        _ => (a.1, a.2) < (b.1, b.2),
    }
}

fn row_semantics(row: &Value, kind: &str) -> Check {
    let md = &row["metadata"];
    let direct = case_of(row).is_some();
    match kind {
        SUPPORT_COMPONENT | SUPPORT_FORCE | SUPPORT_MOMENT => {
            // A1 f exact on case rows; A2 item 4 (frame, location, component) on combination rows.
            if direct {
                require(keys(md, METADATA_KEYS), "ROW_METADATA_SHAPE")?;
            }
            let c = component(row);
            require(
                match kind {
                    SUPPORT_COMPONENT => SUPPORT_COMPONENTS[..6].contains(&c),
                    SUPPORT_FORCE => c == "force_magnitude",
                    _ => c == "moment_magnitude",
                } && md["coordinate_system"] == "global"
                    && md["location"] == "node",
                "ROW_SEMANTICS",
            )?;
            if direct {
                require(
                    md["basis"] == "recovered_from_assembled_support_law"
                        && md["sign_convention"] == SUPPORT_SIGN,
                    "ROW_SEMANTICS",
                )?;
            }
        }
        MAXIMUM => {
            require(keys(md, METADATA_KEYS), "ROW_METADATA_SHAPE")?;
            require(
                direct
                    && component(row) == "maximum_absolute_normal_stress"
                    && md["coordinate_system"] == "pipe_section"
                    && md["location"] == "governing_station"
                    && md["basis"] == "recovered_from_open_mechanics_stress_components"
                    && md["sign_convention"] == MAXIMUM_SIGN,
                "ROW_SEMANTICS",
            )?;
        }
        INTENSIFIED => {
            require(keys(md, METADATA_KEYS), "ROW_METADATA_SHAPE")?;
            require(
                direct
                    && component(row) == "equal_factor_intensified_bending_stress"
                    && md["coordinate_system"] == "pipe_section"
                    && matches!(md["location"].as_str(), Some("end_i" | "end_j"))
                    && md["basis"] == "user_sif_times_member_section_bending_stress_v1"
                    && md["sign_convention"]
                        .as_str()
                        .is_some_and(|s| s.starts_with(INTENSIFIED_SIGN_PREFIX)),
                "ROW_SEMANTICS",
            )?;
        }
        _ => {}
    }
    Ok(())
}

/// Every S1 section 9 check. A failure makes the source unsupported.
pub fn validate_preview_physics_evidence(source: &Value) -> Check {
    let table = crate::semantic_contract::preview_physics_contract();
    finite_tree(source)?;
    // 1. Header and closed evidence shape.
    require(
        source.get("source_block_recovery").is_none() && source.get("carrier_evidence").is_none(),
        "FOREIGN_METHOD_EVIDENCE",
    )?;
    require(
        source["formulation_basis"]["profile_id"] == "product_preview_mechanics_v1"
            && source["formulation_basis"]["limitations"] == table["supported_profile_limitations"],
        "FORMULATION_BASIS",
    )?;
    let evidence = &source["contract_evidence"];
    require(
        keys(evidence, &["preview_cases", "combination_gates"]),
        "EVIDENCE_SHAPE",
    )?;
    let preview_cases = array(&evidence["preview_cases"])?;
    let gates = array(&evidence["combination_gates"])?;
    let rows_list = array(&source["results"])?;
    let diagnostics = array(&source["diagnostics"])?;
    let summary = &source["summary"];
    let solved = source["status"]["mechanics"] == "MECHANICS_SOLVED";

    let mut rows: HashMap<&str, &Value> = HashMap::new();
    for row in rows_list {
        let id = text(&row["id"])?;
        require(rows.insert(id, row).is_none(), "DUPLICATE_SOURCE_ID")?;
    }
    // Unique diagnostic ids are T6 hardening (A3 note); not checked here.
    // 2. Kinds from the table only; no retired kind or code.
    let retired: Vec<&Value> = array(&table["retired_source_kinds"])?.iter().collect();
    for row in rows_list {
        let kind = text(&row["kind"])?;
        require(!retired.contains(&&row["kind"]), "RETIRED_KIND")?;
        number(&row["value"])?;
        text(&row["unit"])?;
        text(&row["entity_ref"])?; // A2 item 8
        require(
            row["metadata"].is_null() || row["metadata"].is_object(),
            "ROW_METADATA_SHAPE",
        )?;
        let signature = crate::semantic_contract::signature_in(table, row)
            .map_err(|e| format!("SOURCE_PREVIEW_PHYSICS_ROW_SIGNATURE: {e}"))?;
        require(signature.is_some(), "ROW_SIGNATURE")?;
        row_semantics(row, kind)?;
        // The basis_ref kind is T6 hardening (A3 note); not checked here.
    }
    // 3. Completeness of result-namespace references (F-1).
    for diagnostic in diagnostics {
        let code = diagnostic["code"].as_str().unwrap_or("");
        require(!RETIRED_CODES.contains(&code), "RETIRED_CODE")?;
        // A1 a: the key may be absent; A2 item 5: when present, an array of
        // non-empty strings (null is refused).
        let affected = match diagnostic.get("affected_refs") {
            Some(refs) => array(refs)?.as_slice(),
            None => &[],
        };
        for reference in affected {
            let reference = text(reference)?;
            if reference.starts_with("result:") {
                require(rows.contains_key(reference), "DANGLING_RESULT_REF")?;
            }
        }
    }
    if let Some(fields) = summary.as_object() {
        for value in fields.values() {
            if let Some(reference) = value.get("result_ref").filter(|r| !r.is_null()) {
                require(rows.contains_key(text(reference)?), "DANGLING_RESULT_REF")?;
            }
        }
    }

    // A1 g: the modifier count is the number of intensified rows.
    if let Some(count) = summary.get("component_stress_modifier_count") {
        let intensified = rows_list
            .iter()
            .filter(|r| r["kind"] == INTENSIFIED)
            .count();
        require(count.as_u64() == Some(intensified as u64), "MODIFIER_COUNT")?;
    }
    // A1 e: a blocked envelope has empty evidence, no rows and null headlines;
    // items 2-3 above already refused retired codes and any `result:` reference.
    if !solved {
        return require(
            preview_cases.is_empty()
                && gates.is_empty()
                && rows_list.is_empty()
                && summary["max_open_formula_stress"].is_null()
                && summary["max_displacement"].is_null(),
            "BLOCKED_ENVELOPE",
        );
    }

    // 4. Cases bind the numerical quality cases.
    // A2 item 2: same load cases in the same order.
    let quality_cases: Vec<&Value> = array(&source["numerical_quality"]["cases"])?
        .iter()
        .map(|case| &case["basis_ref"]["ref_id"])
        .collect();
    let mut cases: Vec<&str> = Vec::new();
    for case in preview_cases {
        require(keys(case, CASE_KEYS), "CASE_SHAPE")?;
        let id = text(&case["load_case_id"])?;
        require(!cases.contains(&id), "DUPLICATE_CASE")?;
        cases.push(id);
    }
    require(
        cases.len() == quality_cases.len()
            && cases
                .iter()
                .zip(&quality_cases)
                .all(|(c, q)| q.as_str() == Some(*c)),
        "NUMERICAL_CASE_COVERAGE",
    )?;
    // Load-case rows lying in a preview case is T6 hardening (A3 note).

    // A withheld support is announced once per model support (S1 section 8).
    let mut withheld_notices: HashSet<(&str, &str)> = HashSet::new();
    for diagnostic in diagnostics {
        let code = diagnostic["code"].as_str().unwrap_or("");
        if WITHHELD_REASONS.contains(&code) {
            let support = text(&diagnostic["affected_refs"][0])?;
            let kind = if code == WITHHELD_REASONS[0] {
                "attribution"
            } else {
                "constant-effort-not-consumed"
            };
            require(
                diagnostic["id"]
                    == format!("diagnostic:preview-physics:{kind}:{}", identity(&[support]))
                        .as_str(),
                "WITHHELD_NOTICE_ID",
            )?;
            require(
                withheld_notices.insert((support, code)),
                "WITHHELD_NOTICE_DUPLICATE",
            )?;
        }
    }
    let mut first_dispositions: Option<(HashSet<&str>, HashSet<(&str, &str)>)> = None;
    let mut coverage_complete = true;
    let mut bound_maxima = HashSet::new();
    let mut bound_intensified = HashSet::new();
    for case in preview_cases {
        let case_id = text(&case["load_case_id"])?;
        let in_case = |row: &&Value| case_of(row) == Some(case_id);
        // 5. Maxima bind their enclosures; coverage partitions the members.
        let members: HashSet<&str> = rows_list
            .iter()
            .filter(in_case)
            .filter(|r| r["kind"] == "element_local_axial_force")
            .map(|r| text(&r["entity_ref"]))
            .collect::<Result<_, _>>()?;
        let coverage = &case["stress_maximum_coverage"];
        require(
            keys(
                coverage,
                &[
                    "complete",
                    "unavailable_pipe_ids",
                    "outside_domain_pipe_ids",
                ],
            ),
            "STRESS_COVERAGE_SHAPE",
        )?;
        let unavailable = strings(&coverage["unavailable_pipe_ids"])?;
        let outside = strings(&coverage["outside_domain_pipe_ids"])?;
        let complete = unavailable.is_empty() && outside.is_empty();
        require(
            coverage["complete"].as_bool() == Some(complete),
            "STRESS_COVERAGE",
        )?;
        coverage_complete &= complete;
        let mut partition = HashSet::new();
        for pipe in unavailable.iter().chain(outside.iter()) {
            require(partition.insert(*pipe), "STRESS_COVERAGE_PARTITION")?;
        }
        for extrema in array(&case["pipe_stress_extrema"])? {
            require(keys(extrema, EXTREMA_KEYS), "EXTREMA_SHAPE")?;
            let pipe = text(&extrema["pipe_id"])?;
            require(partition.insert(pipe), "STRESS_COVERAGE_PARTITION")?;
            let id = text(&extrema["result_id"])?;
            require(bound_maxima.insert(id), "EXTREMA_RESULT_DUPLICATE")?;
            let row = rows
                .get(id)
                .ok_or("SOURCE_PREVIEW_PHYSICS_EXTREMA_RESULT_MISSING")?;
            require(
                row["kind"] == MAXIMUM
                    && row["entity_ref"] == pipe
                    && case_of(row) == Some(case_id)
                    && id == format!("result:elastic-maximum:{}", identity(&[case_id, pipe])),
                "EXTREMA_RESULT_BINDING",
            )?;
            for k in ["station_fraction", "local_fraction"] {
                require(
                    (0.0..=1.0).contains(&number(&extrema[k])?),
                    "EXTREMA_STATION",
                )?;
            }
            require(
                extrema["span_index"].as_u64().is_some()
                    && extrema["subdivisions"]
                        .as_u64()
                        .is_some_and(|n| n <= MAX_SUBDIVISIONS),
                "EXTREMA_SUBDIVISIONS",
            )?;
            let lower = number(&extrema["value_lower_pa"])?;
            let upper = number(&extrema["value_upper_pa"])?;
            number(&extrema["global_upper_bound_pa"])?;
            number(&extrema["certified_gap_pa"])?;
            let value = number(&row["value"])?;
            require(
                lower >= 0.0
                    && lower <= value
                    && value <= upper
                    && value == lower + 0.5 * (upper - lower),
                "EXTREMA_BOUNDS",
            )?;
            require(
                extrema["approximation"] == "piecewise_quadratic_straight_section_statics"
                    && extrema["coefficient_basis"] == "j_side_section_equilibrium_binary64"
                    && extrema["enclosure_scope"]
                        == "supplied_binary64_polynomial_coefficients; solution and coefficient formation error are separate",
                "EXTREMA_BASIS",
            )?;
        }
        require(partition == members, "STRESS_COVERAGE_PARTITION")?;

        // 7. Supports: attributed => exactly the 8 rows; withheld => none.
        let attribution = &case["support_attribution"];
        require(
            keys(attribution, &["attributed_support_ids", "withheld"]),
            "SUPPORT_ATTRIBUTION_SHAPE",
        )?;
        let attributed = strings(&attribution["attributed_support_ids"])?;
        let mut listed: HashSet<&str> = attributed.iter().copied().collect();
        let mut withheld = Vec::new();
        let mut attribution_withheld = Vec::new();
        let mut records = HashSet::new();
        for record in array(&attribution["withheld"])? {
            require(
                keys(record, &["support_id", "reason"]),
                "SUPPORT_WITHHELD_SHAPE",
            )?;
            let support = text(&record["support_id"])?;
            let reason = text(&record["reason"])?;
            require(
                WITHHELD_REASONS.contains(&reason),
                "SUPPORT_WITHHELD_REASON",
            )?;
            require(listed.insert(support), "SUPPORT_LISTED_TWICE")?;
            withheld.push(support);
            if reason == WITHHELD_REASONS[0] {
                attribution_withheld.push(support);
            }
            records.insert((support, reason));
        }
        require(records == withheld_notices, "SUPPORT_WITHHELD_RECORD")?;
        // A2 item 3: dispositions are structural, identical in every case.
        let dispositions = (
            attributed.iter().copied().collect::<HashSet<_>>(),
            records.clone(),
        );
        match &first_dispositions {
            Some(first) => require(*first == dispositions, "SUPPORT_ATTRIBUTION_CASE_MISMATCH")?,
            None => first_dispositions = Some(dispositions),
        }
        let mut actions: HashMap<&str, HashMap<&str, &Value>> = HashMap::new();
        for row in rows_list.iter().filter(in_case) {
            let kind = row["kind"].as_str().unwrap_or("");
            if matches!(kind, SUPPORT_COMPONENT | SUPPORT_FORCE | SUPPORT_MOMENT) {
                let support = text(&row["entity_ref"])?;
                let c = component(row);
                require(
                    text(&row["id"])?
                        == format!(
                            "result:support-action:{}:{c}",
                            identity(&[case_id, support])
                        ),
                    "SUPPORT_ROW_ID",
                )?;
                require(
                    actions.entry(support).or_default().insert(c, row).is_none(),
                    "SUPPORT_COMPONENT_DUPLICATE",
                )?;
            }
            if matches!(
                kind,
                "nonlinear_support_final_reaction"
                    | "nonlinear_support_final_displacement"
                    | "nonlinear_support_active_set_state_code"
            ) {
                require(
                    listed.contains(text(&row["entity_ref"])?),
                    "SUPPORT_ATTRIBUTION_MISSING",
                )?;
            }
            if kind == "nonlinear_support_final_reaction" {
                require(
                    !attribution_withheld.contains(&row["entity_ref"].as_str().unwrap_or("")),
                    "WITHHELD_SUPPORT_ROW",
                )?;
            }
        }
        for support in &withheld {
            require(!actions.contains_key(support), "WITHHELD_SUPPORT_ROW")?;
        }
        require(
            actions.keys().copied().collect::<HashSet<_>>() == attributed.iter().copied().collect(),
            "SUPPORT_ATTRIBUTION_COVERAGE",
        )?;
        for components in actions.values() {
            require(
                components.len() == 8
                    && SUPPORT_COMPONENTS
                        .iter()
                        .all(|c| components.contains_key(c)),
                "SUPPORT_COMPONENT_COVERAGE",
            )?;
            magnitudes(components)?;
        }

        // 8. Intensified rows <=> intensified_measures.
        let mut measures: HashMap<&str, &Value> = HashMap::new();
        for measure in array(&case["intensified_measures"])? {
            require(keys(measure, INTENSIFIED_KEYS), "INTENSIFIED_SHAPE")?;
            let id = text(&measure["result_id"])?;
            require(
                measures.insert(id, measure).is_none(),
                "INTENSIFIED_DUPLICATE",
            )?;
            let row = rows
                .get(id)
                .ok_or("SOURCE_PREVIEW_PHYSICS_INTENSIFIED_RESULT_MISSING")?;
            require(bound_intensified.insert(id), "INTENSIFIED_DUPLICATE")?;
            let location = text(&measure["location"])?;
            text(&measure["pipe_id"])?;
            require(
                row["kind"] == INTENSIFIED
                    && case_of(row) == Some(case_id)
                    && row["entity_ref"] == measure["component_id"]
                    && row["metadata"]["location"] == location
                    && matches!(
                        text(&measure["factor_role"])?,
                        "bend" | "branch_header" | "branch_branch"
                    ),
                "INTENSIFIED_BINDING",
            )?;
            text(&measure["component_id"])?;
            text(&measure["sif_source_reference"])?;
            let sif = number(&measure["sif"])?;
            let z = number(&measure["section_modulus_m3"])?;
            let my = number(&measure["bending_moment_y_n_m"])?;
            let mz = number(&measure["bending_moment_z_n_m"])?;
            require(sif > 0.0 && z > 0.0, "INTENSIFIED_RANGE")?;
            let value = number(&row["value"])?;
            require(value >= 0.0, "INTENSIFIED_NEGATIVE")?; // A3 item 1
            require(
                guarded(value, sif * (my.hypot(mz) / z)),
                "INTENSIFIED_VALUE",
            )?;
            // S1 section 9 item 8: the refs resolve in the same case. Exactly the
            // two bending rows is T6 hardening (A3 note).
            for reference in array(&row["source_result_refs"])? {
                let source_row = rows
                    .get(text(reference)?)
                    .ok_or("SOURCE_PREVIEW_PHYSICS_DANGLING_RESULT_REF")?;
                require(
                    source_row["basis_ref"] == row["basis_ref"],
                    "INTENSIFIED_SOURCE_BINDING",
                )?;
            }
        }
    }
    for row in rows_list {
        let id = text(&row["id"])?;
        if row["kind"] == MAXIMUM {
            require(bound_maxima.contains(id), "EXTREMA_ROW_UNBOUND")?;
        }
        if row["kind"] == INTENSIFIED {
            require(bound_intensified.contains(id), "INTENSIFIED_ROW_UNBOUND")?;
        }
    }

    // 6. Headlines: all cases, identity ties, stress only with complete coverage.
    headline(
        &summary["max_open_formula_stress"],
        rows_list,
        MAXIMUM,
        coverage_complete,
    )?;
    headline(
        &summary["max_displacement"],
        rows_list,
        "displacement_magnitude",
        true,
    )?;

    // 9. Combination rows only for admitted combinations.
    let mut admitted = HashSet::new();
    let mut gate_ids = HashSet::new();
    for gate in gates {
        require(
            keys(gate, &["combination_id", "withheld", "reason"]),
            "GATE_SHAPE",
        )?;
        let id = text(&gate["combination_id"])?;
        require(gate_ids.insert(id), "GATE_DUPLICATE")?;
        match gate["withheld"].as_bool() {
            Some(false) => {
                require(gate["reason"].is_null(), "GATE_REASON")?;
                admitted.insert(id);
            }
            Some(true) => require(
                gate["reason"]
                    .as_str()
                    .is_some_and(|r| GATE_CODES.contains(&r)),
                "GATE_REASON",
            )?,
            None => return Err("SOURCE_PREVIEW_PHYSICS_GATE_SHAPE".into()),
        }
    }
    let mut combined: HashMap<&str, Vec<&Value>> = HashMap::new();
    for row in rows_list {
        let Some(combination) = combination_of(row) else {
            continue;
        };
        require(admitted.contains(combination), "GATED_COMBINATION_ROW")?;
        require(
            row["kind"] != MAXIMUM && row["kind"] != INTENSIFIED,
            "COMBINATION_ROW_KIND",
        )?;
        if let Some(refs) = row.get("source_result_refs").filter(|v| !v.is_null()) {
            for reference in array(refs)? {
                require(rows.contains_key(text(reference)?), "DANGLING_RESULT_REF")?;
            }
        }
        combined.entry(combination).or_default().push(row);
    }
    for members in combined.values() {
        if members
            .iter()
            .any(|r| r["metadata"]["basis"] == RANGE_BASIS)
        {
            continue; // Range envelopes mode-select; they are not a combined state.
        }
        combination_magnitudes(members)?;
    }
    Ok(())
}

fn magnitudes(components: &HashMap<&str, &Value>) -> Check {
    let v = |c: &str| number(&components[c]["value"]);
    require(
        guarded(
            v("force_magnitude")?,
            v("Fx")?.hypot(v("Fy")?).hypot(v("Fz")?),
        ) && guarded(
            v("moment_magnitude")?,
            v("Mx")?.hypot(v("My")?).hypot(v("Mz")?),
        ),
        "SUPPORT_MAGNITUDE",
    )
}

fn combination_magnitudes(rows: &[&Value]) -> Check {
    let find = |kind: &str, entity: &Value, c: Option<&str>| -> Result<f64, String> {
        let hits: Vec<_> = rows
            .iter()
            .filter(|r| {
                r["kind"] == kind
                    && r["entity_ref"] == *entity
                    && c.is_none_or(|c| component(r) == c)
            })
            .collect();
        // A combined support slot must be unique; a nodal component binds the
        // last matching row (same rule as the Python reader).
        require(
            !hits.is_empty() && (kind != SUPPORT_COMPONENT || hits.len() == 1),
            "COMBINATION_MAGNITUDE_COMPONENTS",
        )?;
        number(&hits[hits.len() - 1]["value"])
    };
    for row in rows {
        let entity = &row["entity_ref"];
        let recomputed = match row["kind"].as_str() {
            Some("displacement_magnitude") => find("global_nodal_displacement_x", entity, None)?
                .hypot(find("global_nodal_displacement_y", entity, None)?)
                .hypot(find("global_nodal_displacement_z", entity, None)?),
            Some(SUPPORT_FORCE) => find(SUPPORT_COMPONENT, entity, Some("Fx"))?
                .hypot(find(SUPPORT_COMPONENT, entity, Some("Fy"))?)
                .hypot(find(SUPPORT_COMPONENT, entity, Some("Fz"))?),
            Some(SUPPORT_MOMENT) => find(SUPPORT_COMPONENT, entity, Some("Mx"))?
                .hypot(find(SUPPORT_COMPONENT, entity, Some("My"))?)
                .hypot(find(SUPPORT_COMPONENT, entity, Some("Mz"))?),
            _ => continue,
        };
        require(
            guarded(number(&row["value"])?, recomputed),
            "COMBINATION_MAGNITUDE",
        )?;
    }
    Ok(())
}

fn headline(value: &Value, rows: &[Value], kind: &str, allowed: bool) -> Check {
    let mut winner: Option<(&Value, (f64, &str, &str))> = None;
    for row in rows.iter().filter(|r| r["kind"] == kind) {
        let Some(case) = case_of(row) else {
            continue; // Combinations are outside headline scope.
        };
        let key = (number(&row["value"])?, case, text(&row["entity_ref"])?);
        if winner.as_ref().is_none_or(|(_, best)| governs(key, *best)) {
            winner = Some((row, key));
        }
    }
    match winner.filter(|_| allowed) {
        None => require(value.is_null(), "HEADLINE_PRESENCE"),
        Some((row, _)) => {
            require(
                keys(value, &["value", "unit", "location_ref", "result_ref"]),
                "HEADLINE_PRESENCE",
            )?;
            require(
                value["result_ref"] == row["id"]
                    && value["unit"] == row["unit"]
                    && value["location_ref"] == row["entity_ref"]
                    && number(&value["value"])?.to_bits() == number(&row["value"])?.to_bits(),
                "HEADLINE_BINDING",
            )
        }
    }
}
