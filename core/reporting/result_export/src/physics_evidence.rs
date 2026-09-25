//! Admission of the closed physics-1 evidence namespace. These checks establish
//! internal source consistency, not solver accuracy, origin, or model freshness.
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};

type Check = Result<(), String>;
const PROFILE: &str = "exact_straight_pressure_v2";
const MATERIAL_KEYS: &[&str] = &[
    "pipe_id",
    "material_id",
    "E_pa",
    "nu",
    "G_pa",
    "constitutive_basis",
    "thermal_consumed",
    "alpha_per_kelvin",
    "provenance",
];
const GEOMETRY_KEYS: &[&str] = &[
    "pipe_id",
    "geometry_basis",
    "outside_diameter_m",
    "effective_wall_thickness_m",
    "ri_m",
    "ro_m",
    "Ai_m2",
    "As_m2",
    "I_m4",
    "J_m4",
    "Z_m3",
];
const STATIONS: &[&str] = &["end_i", "end_j", "quarter_1", "midspan", "quarter_3"];

fn require(ok: bool, code: &str) -> Check {
    if ok {
        Ok(())
    } else {
        Err(format!("SOURCE_PHYSICS_{code}"))
    }
}
fn keys(v: &Value, required: &[&str]) -> bool {
    v.as_object()
        .is_some_and(|o| o.len() == required.len() && required.iter().all(|k| o.contains_key(*k)))
}
fn text(v: &Value) -> Result<&str, String> {
    v.as_str()
        .filter(|s| !s.is_empty())
        .ok_or_else(|| "SOURCE_PHYSICS_STRING_INVALID".into())
}
fn number(v: &Value) -> Result<f64, String> {
    v.as_f64()
        .filter(|n| n.is_finite())
        .ok_or_else(|| "SOURCE_PHYSICS_NUMBER_INVALID".into())
}
fn array(v: &Value) -> Result<&Vec<Value>, String> {
    v.as_array()
        .ok_or_else(|| "SOURCE_PHYSICS_ARRAY_INVALID".into())
}
fn strings(v: &Value) -> Result<HashSet<&str>, String> {
    let a = array(v)?;
    let mut set = HashSet::new();
    for item in a {
        require(set.insert(text(item)?), "DUPLICATE_ID")?;
    }
    Ok(set)
}
fn indexed<'a>(v: &'a Value, key: &str) -> Result<HashMap<&'a str, &'a Value>, String> {
    let mut map = HashMap::new();
    for item in array(v)? {
        require(
            map.insert(text(&item[key])?, item).is_none(),
            "DUPLICATE_ID",
        )?;
    }
    Ok(map)
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
fn vector(v: &Value, len: usize) -> Check {
    require(array(v)?.len() == len, "VECTOR_SHAPE")?;
    for x in array(v)? {
        number(x)?;
    }
    Ok(())
}
fn material(v: &Value) -> Check {
    require(keys(v, MATERIAL_KEYS), "MATERIAL_SHAPE")?;
    for k in ["pipe_id", "material_id", "provenance"] {
        text(&v[k])?;
    }
    let e = number(&v["E_pa"])?;
    let nu = number(&v["nu"])?;
    let g = number(&v["G_pa"])?;
    require(
        e > 0.0 && nu > -1.0 && nu < 0.5 && g > 0.0,
        "MATERIAL_RANGE",
    )?;
    // Same normalized E/nu operands as the admitted producer, without inventing
    // an independent G authority or tolerating a contradicting duplicate.
    let direct_g = e / (2.0 * (1.0 + nu));
    // The source uses scaled arithmetic; allow its final binary64 rounding at
    // the representation boundary, not a separate constitutive tolerance.
    require(
        direct_g.is_finite() && g.to_bits().abs_diff(direct_g.to_bits()) <= 2,
        "MATERIAL_G_BINDING",
    )?;
    require(
        v["constitutive_basis"] == "homogeneous_isotropic_E_nu_v1",
        "MATERIAL_BASIS",
    )?;
    match v["thermal_consumed"].as_bool() {
        Some(true) => {
            number(&v["alpha_per_kelvin"])?;
        }
        Some(false) => require(v["alpha_per_kelvin"].is_null(), "UNUSED_ALPHA")?,
        None => return Err("SOURCE_PHYSICS_THERMAL_FLAG".into()),
    }
    Ok(())
}
fn geometry(v: &Value) -> Check {
    require(keys(v, GEOMETRY_KEYS), "GEOMETRY_SHAPE")?;
    text(&v["pipe_id"])?;
    require(
        v["geometry_basis"] == "authored_normalized_od_wall_v1",
        "GEOMETRY_BASIS",
    )?;
    for k in &GEOMETRY_KEYS[2..] {
        require(number(&v[*k])? > 0.0, "GEOMETRY_RANGE")?;
    }
    let od = number(&v["outside_diameter_m"])?;
    let wall = number(&v["effective_wall_thickness_m"])?;
    require(
        wall < od * 0.5
            && number(&v["ri_m"])? < number(&v["ro_m"])?
            && number(&v["ro_m"])? == od * 0.5
            && number(&v["ri_m"])? == od * 0.5 - wall,
        "GEOMETRY_SOURCE_BINDING",
    )
}
fn temperature(v: &Value, case_basis: &str, material_id: &str) -> Check {
    match v["selection"].as_str() {
        Some("base_material") => {
            require(keys(v, &["selection"]), "TEMPERATURE_SHAPE")?;
            require(
                case_basis == "base_material_common_E_nu",
                "TEMPERATURE_CASE_BINDING",
            )
        }
        Some("exact_point") => {
            require(keys(v, &["selection", "point_id"]), "TEMPERATURE_SHAPE")?;
            let point = text(&v["point_id"])?;
            require(case_basis.contains(&format!("material={material_id};common_E_nu_basis=point:{point};G=E/[2(1+nu)];alpha_same_basis=")), "TEMPERATURE_CASE_BINDING")
        }
        Some("interpolation") => {
            require(
                keys(v, &["selection", "temperature_value", "temperature_unit"]),
                "TEMPERATURE_SHAPE",
            )?;
            number(&v["temperature_value"])?;
            require(
                matches!(v["temperature_unit"].as_str(), Some("degC" | "degF" | "K"))
                    && case_basis.contains(&format!(
                        "material={material_id};common_E_nu_basis=interpolated:"
                    ))
                    && case_basis.contains(";temperature_kelvin="),
                "TEMPERATURE_CASE_BINDING",
            )
        }
        _ => Err("SOURCE_PHYSICS_TEMPERATURE_BASIS".into()),
    }
}
fn without(v: &Value, key: &str) -> Value {
    let mut copy = v.clone();
    if let Some(o) = copy.as_object_mut() {
        o.remove(key);
    }
    copy
}
fn case_basis(row: &Value) -> Result<&str, String> {
    let b = &row["basis_ref"];
    require(
        keys(b, &["ref_type", "ref_id"]) && b["ref_type"] == "load_case",
        "CASE_BASIS",
    )?;
    text(&b["ref_id"])
}
fn pressure_kind(kind: &str) -> bool {
    matches!(
        kind,
        "pipe_wall_endpoint_action_v2"
            | "pipe_wall_axial_force_v2"
            | "pipe_effective_axial_force_v2"
            | "pipe_axial_membrane_stress_v2"
            | "pipe_lame_radial_stress_v2"
            | "pipe_lame_hoop_stress_v2"
    )
}
fn pipe_kind(kind: &str) -> bool {
    pressure_kind(kind) || kind == "pipe_elastic_normal_stress_maximum_v2"
}

fn physical_row(row: &Value) -> Check {
    let kind = text(&row["kind"])?;
    let signature =
        crate::semantic_contract::signature_in(crate::semantic_contract::physics_contract(), row)?;
    if !kind.ends_with("_v2") {
        return Ok(());
    }
    require(
        signature.is_some() && crate::semantic_contract::complete_metadata(row),
        "ROW_SIGNATURE",
    )?;
    let md = &row["metadata"];
    require(
        keys(
            md,
            &[
                "component",
                "coordinate_system",
                "location",
                "basis",
                "sign_convention",
            ],
        ),
        "ROW_METADATA_SHAPE",
    )?;
    let component = text(&md["component"])?;
    require(
        signature.unwrap()["component"] == component,
        "ROW_COMPONENT",
    )?;
    let (frame, basis, location_ok, sign) = if kind.starts_with("support_reaction_") {
        ("global", "recovered_from_assembled_support_law", md["location"] == "node", "support-on-pipe; positive global force and right-hand couple about attached node; force and moment norms remain separate")
    } else if kind == "pipe_wall_endpoint_action_v2" {
        ("element_local", "recovered_from_local_element_stiffness", matches!(md["location"].as_str(), Some("end_i" | "end_j")), "node-on-element wall action, positive along authored local x toward end j; cap transfer is not subtracted from wall recovery")
    } else if kind == "pipe_wall_axial_force_v2" {
        (
            "element_local",
            "recovered_from_local_element_stiffness",
            md["location"]
                .as_str()
                .is_some_and(|x| STATIONS.contains(&x)),
            "tension-positive material wall section resultant Nw",
        )
    } else if kind == "pipe_effective_axial_force_v2" {
        (
            "element_local",
            "recovered_from_local_element_stiffness",
            md["location"]
                .as_str()
                .is_some_and(|x| STATIONS.contains(&x)),
            "effective wall-fluid resultant S=Nw-pAi; not material stress or a support reaction",
        )
    } else if kind == "pipe_elastic_normal_stress_maximum_v2" {
        ("pipe_section", "recovered_from_open_mechanics_stress_components", md["location"] == "governing_station", "nonnegative circumferential maximum |Nw/As|+hypot(My,Mz)/Z; bounded over all straight statics intervals; torsional shear remains separate; no code stress or equivalent stress claim")
    } else {
        let sign = match kind {
            "pipe_axial_membrane_stress_v2" => "tension-positive axial wall membrane stress Nw/As; no added longitudinal pressure scalar",
            "pipe_lame_radial_stress_v2" => "tension-positive radial stress at named surface, inner traction -p and zero external pressure increment",
            "pipe_lame_hoop_stress_v2" => "tension-positive circumferential stress at named surface for long straight annulus, zero external pressure increment",
            _ => return Err("SOURCE_PHYSICS_ROW_KIND_UNSUPPORTED".into()),
        };
        (
            "pipe_section",
            "recovered_from_open_mechanics_stress_components",
            md["location"]
                .as_str()
                .is_some_and(|x| STATIONS.contains(&x)),
            sign,
        )
    };
    require(
        md["coordinate_system"] == frame
            && md["basis"] == basis
            && location_ok
            && md["sign_convention"] == sign,
        "ROW_SEMANTICS",
    )
}

/// Strict direct-case admission. Combination operands and source-block recovery
/// require their own semantic contract; no name or accuracy label enables them.
pub fn validate_physics_evidence(source: &Value) -> Check {
    finite_tree(source)?;
    require(
        source.get("source_block_recovery").is_none() && source.get("carrier_evidence").is_none(),
        "FOREIGN_METHOD_EVIDENCE",
    )?;
    let evidence = &source["contract_evidence"];
    require(
        keys(evidence, &["pressure", "connector", "exact_cases"]),
        "EVIDENCE_SHAPE",
    )?;
    require(
        array(&evidence["connector"])?.is_empty(),
        "CONNECTOR_UNSUPPORTED",
    )?;
    let cases = indexed(&evidence["exact_cases"], "load_case_id")?;
    let rows = indexed(&source["results"], "id")?;
    let mut all_ids: HashSet<&str> = rows.keys().copied().collect();
    for diagnostic in array(&source["diagnostics"])? {
        require(
            all_ids.insert(text(&diagnostic["id"])?),
            "DUPLICATE_SOURCE_ID",
        )?;
    }
    let mut quality_cases = HashSet::new();
    for case in array(&source["numerical_quality"]["cases"])? {
        require(
            quality_cases.insert(case_basis(case)?),
            "DUPLICATE_NUMERICAL_CASE",
        )?;
    }
    if source["status"]["mechanics"] == "MECHANICS_SOLVED" {
        require(
            !cases.is_empty() && cases.keys().copied().collect::<HashSet<_>>() == quality_cases,
            "NUMERICAL_CASE_COVERAGE",
        )?;
    }
    // Numerical references are case scoped. A resolving ID from another case
    // does not support this case, even when the aggregate status says passed.
    for case in array(&source["numerical_quality"]["cases"])? {
        let case_id = case_basis(case)?;
        let mut refs = HashSet::new();
        for reference in array(&case["evidence_refs"])? {
            let id = text(reference)?;
            require(refs.insert(id), "NUMERICAL_EVIDENCE_DUPLICATE")?;
            let scoped = if let Some(row) = rows.get(id) {
                case_basis(row)? == case_id
            } else {
                array(&source["diagnostics"])?.iter().any(|d| {
                    d["id"] == id
                        && d["affected_refs"]
                            .as_array()
                            .is_some_and(|r| r.contains(&json!(case_id)))
                })
            };
            require(scoped, "NUMERICAL_EVIDENCE_CASE_BINDING")?;
        }
    }
    let mut case_members = HashMap::new();
    let mut extrema_ids = HashSet::new();
    let mut coverage_complete = true;
    let mut all_members: Option<HashSet<&str>> = None;
    for (case_id, case) in &cases {
        require(
            keys(
                case,
                &[
                    "load_case_id",
                    "profile_mode",
                    "material_basis",
                    "pipe_materials",
                    "pipe_sections",
                    "pipe_stress_extrema",
                    "stress_maximum_coverage",
                    "pressure_rhs_assembly",
                ],
            ),
            "CASE_SHAPE",
        )?;
        require(case["profile_mode"] == PROFILE, "CASE_PROFILE")?;
        text(&case["material_basis"])?;
        let materials = indexed(&case["pipe_materials"], "pipe_id")?;
        let sections = indexed(&case["pipe_sections"], "pipe_id")?;
        let members: HashSet<_> = materials.keys().copied().collect();
        require(
            !members.is_empty() && members == sections.keys().copied().collect(),
            "MEMBER_COVERAGE",
        )?;
        if let Some(expected) = &all_members {
            require(*expected == members, "CASE_MEMBER_COVERAGE")?;
        } else {
            all_members = Some(members.clone());
        }
        for m in materials.values() {
            material(m)?;
        }
        for s in sections.values() {
            geometry(s)?;
        }
        let coverage = &case["stress_maximum_coverage"];
        require(
            keys(coverage, &["complete", "unavailable_pipe_ids"]),
            "STRESS_COVERAGE_SHAPE",
        )?;
        let unavailable = strings(&coverage["unavailable_pipe_ids"])?;
        require(
            unavailable.is_subset(&members)
                && coverage["complete"].as_bool() == Some(unavailable.is_empty()),
            "STRESS_COVERAGE",
        )?;
        coverage_complete &= unavailable.is_empty();
        let extrema = indexed(&case["pipe_stress_extrema"], "pipe_id")?;
        require(
            extrema.keys().copied().collect::<HashSet<_>>()
                == members.difference(&unavailable).copied().collect(),
            "EXTREMA_MEMBER_COVERAGE",
        )?;
        for (pipe, ex) in extrema {
            require(
                keys(
                    ex,
                    &[
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
                    ],
                ),
                "EXTREMA_SHAPE",
            )?;
            let id = text(&ex["result_id"])?;
            require(extrema_ids.insert(id), "EXTREMA_RESULT_DUPLICATE")?;
            let row = rows
                .get(id)
                .ok_or("SOURCE_PHYSICS_EXTREMA_RESULT_MISSING")?;
            require(
                row["kind"] == "pipe_elastic_normal_stress_maximum_v2"
                    && row["entity_ref"] == pipe
                    && case_basis(row)? == *case_id,
                "EXTREMA_RESULT_BINDING",
            )?;
            for k in ["station_fraction", "local_fraction"] {
                require((0.0..=1.0).contains(&number(&ex[k])?), "EXTREMA_STATION")?;
            }
            require(
                ex["span_index"].as_u64().is_some()
                    && ex["subdivisions"].as_u64().is_some_and(|n| n <= 131072),
                "EXTREMA_SUBDIVISIONS",
            )?;
            let lower = number(&ex["value_lower_pa"])?;
            let upper = number(&ex["value_upper_pa"])?;
            let global = number(&ex["global_upper_bound_pa"])?;
            let gap = number(&ex["certified_gap_pa"])?;
            require(
                lower >= 0.0
                    && upper >= lower
                    && global >= upper
                    && gap >= 0.0
                    && gap <= 1e-12 + 1e-12 * lower
                    && gap >= global - lower
                    && number(&row["value"])? == lower + 0.5 * (upper - lower),
                "EXTREMA_BOUNDS",
            )?;
            require(ex["approximation"] == "piecewise_quadratic_straight_section_statics" && ex["coefficient_basis"] == "j_side_section_equilibrium_binary64" && ex["enclosure_scope"] == "supplied_binary64_polynomial_coefficients; solution and coefficient formation error are separate", "EXTREMA_BASIS")?;
        }
        case_members.insert(*case_id, members);
    }
    let mut slots = HashSet::new();
    let mut support_components: HashMap<(&str, &str), HashSet<&str>> = HashMap::new();
    for (id, row) in &rows {
        number(&row["value"])?;
        text(&row["unit"])?;
        text(&row["entity_ref"])?;
        let case = case_basis(row)?;
        require(cases.contains_key(case), "ROW_CASE_UNRESOLVED")?;
        require(
            row.get("source_result_refs")
                .is_none_or(|v| v.as_array().is_some_and(|a| a.is_empty())),
            "DERIVED_ROWS_UNSUPPORTED",
        )?;
        let kind = text(&row["kind"])?;
        physical_row(row)?;
        if pipe_kind(kind) {
            require(
                case_members[case].contains(text(&row["entity_ref"])?),
                "ROW_MEMBER_UNRESOLVED",
            )?;
            require(
                slots.insert((
                    case,
                    text(&row["entity_ref"])?,
                    kind,
                    text(&row["metadata"]["component"])?,
                    text(&row["metadata"]["location"])?,
                )),
                "ROW_SLOT_DUPLICATE",
            )?;
        }
        if kind == "pipe_elastic_normal_stress_maximum_v2" {
            require(extrema_ids.contains(id), "EXTREMA_ROW_UNBOUND")?;
        }
        if kind.starts_with("support_reaction_") && kind.ends_with("_v2") {
            let component = text(&row["metadata"]["component"])?;
            require(
                support_components
                    .entry((case, text(&row["entity_ref"])?))
                    .or_default()
                    .insert(component),
                "SUPPORT_COMPONENT_DUPLICATE",
            )?;
            if component.ends_with("magnitude") {
                require(number(&row["value"])? >= 0.0, "SUPPORT_MAGNITUDE_RANGE")?;
            }
        }
    }
    for components in support_components.values() {
        require(
            *components
                == [
                    "Fx",
                    "Fy",
                    "Fz",
                    "Mx",
                    "My",
                    "Mz",
                    "force_magnitude",
                    "moment_magnitude",
                ]
                .into_iter()
                .collect(),
            "SUPPORT_COMPONENT_COVERAGE",
        )?;
    }
    let mut regions = HashMap::new();
    let mut owned_members = HashSet::new();
    let mut bound_results = HashSet::new();
    for region in array(&evidence["pressure"])? {
        let case = text(&region["load_case_id"])?;
        let region_id = text(&region["region_id"])?;
        require(
            regions.insert((case, region_id), region).is_none(),
            "REGION_DUPLICATE",
        )?;
        let exact = cases
            .get(case)
            .ok_or("SOURCE_PHYSICS_REGION_CASE_UNRESOLVED")?;
        require(
            keys(
                region,
                &[
                    "profile_version",
                    "profile_mode",
                    "load_case_id",
                    "region_id",
                    "member_pipe_ids",
                    "pressure_basis",
                    "p_pa",
                    "external_pressure_increment_pa",
                    "approximation",
                    "geometry_representation_guard",
                    "geometry",
                    "materials",
                    "applied_loads",
                    "terminals",
                    "provenance",
                    "result_ids",
                ],
            ),
            "REGION_SHAPE",
        )?;
        require(
            region["profile_version"] == "2.0.0"
                && region["profile_mode"] == PROFILE
                && region["pressure_basis"] == "internal_differential_zero_external_v1"
                && region["external_pressure_increment_pa"] == 0.0
                && region["approximation"] == "long_straight_annulus_small_strain_v2",
            "REGION_PROFILE",
        )?;
        require(number(&region["p_pa"])? >= 0.0, "PRESSURE_RANGE")?;
        text(&region["provenance"])?;
        require(
            region["geometry_representation_guard"]
                == json!({"epsilon_multiplier":64,"meaning":"arithmetic_representation_only"}),
            "REPRESENTATION_GUARD",
        )?;
        let members = strings(&region["member_pipe_ids"])?;
        require(
            !members.is_empty() && members.is_subset(&case_members[case]),
            "REGION_MEMBER_COVERAGE",
        )?;
        for member in &members {
            require(
                owned_members.insert((case, *member)),
                "REGION_MEMBER_OVERLAP",
            )?;
        }
        for (field, case_field, extra) in [
            ("geometry", "pipe_sections", "traversal_forward"),
            ("materials", "pipe_materials", "temperature_basis"),
        ] {
            let duplicates = indexed(&region[field], "pipe_id")?;
            let basis = indexed(&exact[case_field], "pipe_id")?;
            require(
                duplicates.keys().copied().collect::<HashSet<_>>() == members,
                "REGION_DUPLICATE_COVERAGE",
            )?;
            for (member, duplicate) in duplicates {
                require(
                    without(duplicate, extra) == *basis[member],
                    "REGION_DUPLICATE_CONTRADICTION",
                )?;
                if field == "geometry" {
                    require(duplicate[extra].is_boolean(), "TRAVERSAL_FLAG")?;
                } else {
                    temperature(
                        &duplicate[extra],
                        text(&exact["material_basis"])?,
                        text(&duplicate["material_id"])?,
                    )?;
                }
            }
        }
        let applied = indexed(&region["applied_loads"], "pipe_id")?;
        require(
            applied.keys().copied().collect::<HashSet<_>>() == members,
            "APPLIED_MEMBER_COVERAGE",
        )?;
        for item in applied.values() {
            require(
                keys(
                    item,
                    &[
                        "pipe_id",
                        "eigenload_pair_local_n",
                        "mathematical_cap_pair_local_n",
                        "local_x_global",
                        "thermal_included",
                    ],
                ),
                "APPLIED_SHAPE",
            )?;
            vector(&item["eigenload_pair_local_n"], 2)?;
            vector(&item["mathematical_cap_pair_local_n"], 2)?;
            vector(&item["local_x_global"], 3)?;
            require(item["thermal_included"] == false, "APPLIED_THERMAL_FLAG")?;
        }
        let terminals = array(&region["terminals"])?;
        require(
            terminals.len() == 2 && terminals[0]["node_ref"] != terminals[1]["node_ref"],
            "TERMINAL_COVERAGE",
        )?;
        for terminal in terminals {
            require(
                keys(
                    terminal,
                    &[
                        "node_ref",
                        "closure_transfer",
                        "provenance",
                        "closure_pressure_load_global_n",
                        "pipe_cap_transfer_global_n",
                        "remote_closure_support_reaction_global_n",
                        "remote_closure_excluded_from_pipe_solve",
                    ],
                ),
                "TERMINAL_SHAPE",
            )?;
            text(&terminal["node_ref"])?;
            text(&terminal["provenance"])?;
            vector(&terminal["closure_pressure_load_global_n"], 3)?;
            vector(&terminal["pipe_cap_transfer_global_n"], 3)?;
            match terminal["closure_transfer"].as_str() {
                Some("transfers_to_wall") => require(
                    terminal["remote_closure_excluded_from_pipe_solve"] == false
                        && terminal["remote_closure_support_reaction_global_n"].is_null()
                        && terminal["pipe_cap_transfer_global_n"]
                            == terminal["closure_pressure_load_global_n"],
                    "TERMINAL_TRANSFER",
                )?,
                Some("separately_supported_or_compensated") => {
                    require(
                        terminal["remote_closure_excluded_from_pipe_solve"] == true
                            && array(&terminal["pipe_cap_transfer_global_n"])?
                                .iter()
                                .all(|v| v.as_f64() == Some(0.0)),
                        "TERMINAL_TRANSFER",
                    )?;
                    vector(&terminal["remote_closure_support_reaction_global_n"], 3)?;
                    for (reaction, closure) in
                        array(&terminal["remote_closure_support_reaction_global_n"])?
                            .iter()
                            .zip(array(&terminal["closure_pressure_load_global_n"])?)
                    {
                        require(
                            number(reaction)? == -number(closure)?,
                            "TERMINAL_REMOTE_REACTION",
                        )?;
                    }
                }
                _ => return Err("SOURCE_PHYSICS_TERMINAL_CLOSURE".into()),
            }
        }
        let actual = strings(&region["result_ids"])?;
        let expected: HashSet<_> = rows
            .iter()
            .filter(|(_, row)| {
                case_basis(row).ok() == Some(case)
                    && members.contains(row["entity_ref"].as_str().unwrap_or(""))
                    && pipe_kind(row["kind"].as_str().unwrap_or(""))
            })
            .map(|(id, _)| *id)
            .collect();
        require(actual == expected, "REGION_RESULT_BINDING")?;
        for id in actual {
            require(bound_results.insert(id), "REGION_RESULT_DUPLICATE")?;
        }
        for member in &members {
            for (kind, components) in [
                (
                    "pipe_wall_endpoint_action_v2",
                    &["wall_axial_end_action"][..],
                ),
                ("pipe_wall_axial_force_v2", &["wall_axial_force"][..]),
                (
                    "pipe_effective_axial_force_v2",
                    &["effective_axial_force"][..],
                ),
                (
                    "pipe_axial_membrane_stress_v2",
                    &["axial_membrane_stress"][..],
                ),
                (
                    "pipe_lame_radial_stress_v2",
                    &["lame_inner_radial_stress", "lame_outer_radial_stress"][..],
                ),
                (
                    "pipe_lame_hoop_stress_v2",
                    &["lame_inner_hoop_stress", "lame_outer_hoop_stress"][..],
                ),
            ] {
                let stations = if kind == "pipe_wall_endpoint_action_v2" {
                    &STATIONS[..2]
                } else {
                    STATIONS
                };
                for component in components {
                    for station in stations {
                        require(
                            slots.contains(&(case, *member, kind, *component, *station)),
                            "PRESSURE_ROW_COVERAGE",
                        )?;
                    }
                }
            }
        }
    }
    for (id, row) in &rows {
        if pressure_kind(row["kind"].as_str().unwrap_or("")) {
            require(bound_results.contains(id), "PRESSURE_ROW_UNBOUND")?;
        }
    }
    for (case_id, case) in &cases {
        assembly(&case["pressure_rhs_assembly"], case_id, &regions)?;
    }
    let headline = &source["summary"]["max_open_formula_stress"];
    if !headline.is_null() {
        require(
            coverage_complete && keys(headline, &["value", "unit", "location_ref", "result_ref"]),
            "INCOMPLETE_STRESS_HEADLINE",
        )?;
        let row = rows
            .get(text(&headline["result_ref"])?)
            .ok_or("SOURCE_PHYSICS_HEADLINE_RESULT_MISSING")?;
        require(
            row["kind"] == "pipe_elastic_normal_stress_maximum_v2"
                && headline["value"] == row["value"]
                && headline["unit"] == row["unit"]
                && headline["location_ref"] == row["entity_ref"],
            "HEADLINE_RESULT_BINDING",
        )?;
        require(
            rows.values()
                .filter(|r| r["kind"] == "pipe_elastic_normal_stress_maximum_v2")
                .all(|r| {
                    r["value"].as_f64().unwrap_or(f64::INFINITY)
                        <= row["value"].as_f64().unwrap_or(f64::NEG_INFINITY)
                }),
            "HEADLINE_MAXIMUM",
        )?;
    }
    Ok(())
}

fn assembly(v: &Value, case: &str, regions: &HashMap<(&str, &str), &Value>) -> Check {
    require(
        keys(
            v,
            &[
                "method",
                "load_case_id",
                "node_order",
                "dof_order",
                "dof_units",
                "assembled_pressure_rhs_global",
                "groups",
                "rounded_cap_rhs_global",
                "rounded_poisson_rhs_global",
                "rounded_cap_and_eigen_ledgers_are_observational",
                "cancellation_screen",
                "screen_limit",
                "screen_roundoff_multiplier",
                "screen_is_not_numerical_qualification",
            ],
        ),
        "RHS_SHAPE",
    )?;
    require(
        v["method"] == "source_factor_grouped_pressure_rhs_v1"
            && v["load_case_id"] == case
            && v["dof_order"] == json!(["Fx", "Fy", "Fz", "Mx", "My", "Mz"])
            && v["dof_units"] == json!(["N", "N", "N", "N*m", "N*m", "N*m"])
            && v["rounded_cap_and_eigen_ledgers_are_observational"] == true
            && v["screen_is_not_numerical_qualification"] == true
            && v["screen_limit"] == 1e-9
            && v["screen_roundoff_multiplier"] == 32,
        "RHS_METHOD",
    )?;
    let nodes = strings(&v["node_order"])?;
    require(!nodes.is_empty(), "RHS_NODES")?;
    for key in [
        "assembled_pressure_rhs_global",
        "rounded_cap_rhs_global",
        "rounded_poisson_rhs_global",
    ] {
        vector(&v[key], nodes.len() * 6)?;
    }
    let screen = number(&v["cancellation_screen"])?;
    require((0.0..=1e-9).contains(&screen), "RHS_SCREEN")?;
    for values in [
        "assembled_pressure_rhs_global",
        "rounded_cap_rhs_global",
        "rounded_poisson_rhs_global",
    ] {
        for (i, n) in array(&v[values])?.iter().enumerate() {
            if i % 6 >= 3 {
                require(number(n)? == 0.0, "RHS_PRESSURE_MOMENT")?;
            }
        }
    }
    let mut groups = HashSet::new();
    for group in array(&v["groups"])? {
        require(
            keys(
                group,
                &[
                    "node_ref",
                    "component",
                    "pressure_bits",
                    "source_inner_radius_hi_bits",
                    "source_inner_radius_lo_bits",
                    "direction_component_magnitude",
                    "coefficient_sum",
                    "assembled_force_n",
                    "terms",
                ],
            ),
            "RHS_GROUP_SHAPE",
        )?;
        require(
            nodes.contains(text(&group["node_ref"])?)
                && matches!(group["component"].as_str(), Some("Fx" | "Fy" | "Fz")),
            "RHS_GROUP_NODE",
        )?;
        for k in [
            "pressure_bits",
            "source_inner_radius_hi_bits",
            "source_inner_radius_lo_bits",
        ] {
            let s = text(&group[k])?;
            require(
                s.len() == 16
                    && u64::from_str_radix(s, 16)
                        .is_ok_and(|bits| f64::from_bits(bits).is_finite()),
                "RHS_SOURCE_BITS",
            )?;
        }
        let magnitude = number(&group["direction_component_magnitude"])?;
        require(magnitude > 0.0 && magnitude <= 1.0, "RHS_DIRECTION")?;
        number(&group["coefficient_sum"])?;
        number(&group["assembled_force_n"])?;
        require(
            groups.insert((
                text(&group["node_ref"])?,
                text(&group["component"])?,
                text(&group["pressure_bits"])?,
                text(&group["source_inner_radius_hi_bits"])?,
                text(&group["source_inner_radius_lo_bits"])?,
                magnitude.to_bits(),
            )),
            "RHS_GROUP_DUPLICATE",
        )?;
        require(!array(&group["terms"])?.is_empty(), "RHS_GROUP_TERMS")?;
        for term in array(&group["terms"])? {
            require(
                keys(term, &["coefficient", "region_id", "pipe_id", "kind"]),
                "RHS_TERM_SHAPE",
            )?;
            let region = regions
                .get(&(case, text(&term["region_id"])?))
                .ok_or("SOURCE_PHYSICS_RHS_REGION_UNRESOLVED")?;
            let pipe = text(&term["pipe_id"])?;
            require(
                strings(&region["member_pipe_ids"])?.contains(pipe),
                "RHS_MEMBER_UNRESOLVED",
            )?;
            let coefficient = number(&term["coefficient"])?;
            let geometries = indexed(&region["geometry"], "pipe_id")?;
            let geometry = geometries[pipe];
            let a = number(&geometry["outside_diameter_m"])? * 0.5;
            let b = number(&geometry["effective_wall_thickness_m"])?;
            let hi = a - b;
            let b_virtual = a - hi;
            let a_virtual = hi + b_virtual;
            let lo = (a - a_virtual) + (b_virtual - b);
            require(
                u64::from_str_radix(text(&group["source_inner_radius_hi_bits"])?, 16).ok()
                    == Some(hi.to_bits())
                    && u64::from_str_radix(text(&group["source_inner_radius_lo_bits"])?, 16).ok()
                        == Some(if lo == 0.0 { 0 } else { lo.to_bits() }),
                "RHS_GEOMETRY_BINDING",
            )?;
            let p = number(&region["p_pa"])?;
            require(
                u64::from_str_radix(text(&group["pressure_bits"])?, 16).ok() == Some(p.to_bits()),
                "RHS_PRESSURE_BINDING",
            )?;
            match term["kind"].as_str() {
                Some("terminal_cap") => require(coefficient.abs() == 1.0, "RHS_CAP_COEFFICIENT")?,
                Some("poisson_eigen") => {
                    let materials = indexed(&region["materials"], "pipe_id")?;
                    require(
                        coefficient.abs() == (2.0 * number(&materials[pipe]["nu"])?).abs(),
                        "RHS_EIGEN_COEFFICIENT",
                    )?;
                }
                _ => return Err("SOURCE_PHYSICS_RHS_TERM_KIND".into()),
            }
        }
    }
    Ok(())
}
