//! Admission of the current closed pressure evidence namespace. These checks establish
//! internal source consistency, not solver accuracy, origin, or model freshness.
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};

type Check = Result<(), String>;

/// The sole current exact pressure contract; the caller selects its namespace.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PressureContract {
    PressureV3,
}
impl PressureContract {
    pub const fn version(self) -> &'static str { "3.0.0" }
    pub const fn mode(self) -> &'static str { "exact_pressure_v3" }
    fn table(self) -> &'static Value { crate::semantic_contract::pressure_rows_contract() }
}
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
const RESOLVED_MATERIAL_KEYS: &[&str] = &[
    "pipe_id", "material_id", "E_pa", "nu", "G_pa", "constitutive_basis",
    "thermal_consumed", "alpha_per_kelvin", "provenance",
    "material_selection_kind", "resolved_eigenstrain",
];
const EVIDENCE_KEYS: &[&str] = &["pressure", "connector", "exact_cases"];
const RESOLVED_EVIDENCE_KEYS: &[&str] = &[
    "pressure", "connector", "exact_cases", "load_reference_states",
];
const STATIONS: &[&str] = &["end_i", "end_j", "quarter_1", "midspan", "quarter_3"];
/// A straight pressure region's keys (an arc region adds `arc_evidence::REGION_KEYS`).
pub(crate) const REGION_KEYS: &[&str] = &[
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
];

/// W-16 slice c (§2): the exact mass method (PP `self_weight::EXACT_MASS_METHOD`).
pub(crate) const EXACT_MASS_METHOD: &str = "source_od_nominal_wall_pressure_bore_areas/v1";

/// W-16 slice c (§2): a case's `equivalent_static` record is closed, names the
/// exact route's D-5 mass method, and lists at least one generated primitive,
/// each once, each in the case's generated namespace (`<case>:generated:`).
pub(crate) fn equivalent_static_record(case_id: &str, record: &Value) -> Check {
    require(keys(record, &["mass_method", "generated_load_ids"]), "EQUIVALENT_STATIC_SHAPE")?;
    require(record["mass_method"] == EXACT_MASS_METHOD, "EQUIVALENT_STATIC_MASS_METHOD")?;
    let ids = array(&record["generated_load_ids"])?;
    require(!ids.is_empty(), "EQUIVALENT_STATIC_SHAPE")?;
    let prefix = format!("{case_id}:generated:");
    let mut seen = HashSet::new();
    for id in ids {
        let id = text(id)?;
        require(id.starts_with(&prefix) && seen.insert(id), "EQUIVALENT_STATIC_LOAD_IDS")?;
    }
    Ok(())
}

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
fn material(v: &Value) -> Check { material_in(v, false) }
fn material_in(v: &Value, resolved: bool) -> Check {
    let fields = if resolved { RESOLVED_MATERIAL_KEYS } else { MATERIAL_KEYS };
    require(keys(v, fields), "MATERIAL_SHAPE")?;
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
/// T4-U6 (D-5): one `pipe_sections` entry, its three geometries rederived from
/// its captured authored values (`section_basis::section`). The exact-retained
/// transport reads it with exactly §2.5's straight keys (the retained route has
/// no realized arc); the evidence reader takes the arc marker
/// (`section_basis::marked_section`).
fn geometry(v: &Value) -> Result<crate::section_basis::Sections, String> {
    crate::section_basis::section(v)
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
    without_all(v, &[key])
}
fn without_all(v: &Value, removed: &[&str]) -> Value {
    let mut copy = v.clone();
    if let Some(o) = copy.as_object_mut() {
        for key in removed {
            o.remove(*key);
        }
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
/// A row's load case, which must be one of the evidence's exact cases.
fn row_case<'a>(row: &'a Value, cases: &HashMap<&str, &Value>) -> Result<&'a str, String> {
    let case = case_basis(row)?;
    require(cases.contains_key(case), "ROW_CASE_UNRESOLVED")?;
    Ok(case)
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

/// One published row's declared semantics; `signature` is the row's signature
/// under the reader's table (taken once per row by the caller).
fn physical_row(
    row: &Value,
    kind: &str,
    signature: Option<&'static Value>,
    source_selected: bool,
    arc: bool,
) -> Check {
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
        (
            "pipe_section",
            if source_selected {
                crate::physics_source::MAX_BASIS
            } else {
                "recovered_from_open_mechanics_stress_components"
            },
            md["location"] == "governing_station",
            if source_selected {
                crate::physics_source::MAX_SIGN
            } else {
                "nonnegative circumferential maximum |Nw/As|+hypot(My,Mz)/Z; bounded over all straight statics intervals; torsional shear remains separate; no code stress or equivalent stress claim"
            },
        )
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
    // T4-U2: a pressure row on a realized arc (pressure-2 only) states the
    // arc's own convention; the straight Lame rows and maximum are withheld.
    // T4-U4b (§2.3): an arc's maximum states the along-arc bound.
    let sign = if arc {
        require(!crate::arc_evidence::REFUSED_KINDS.contains(&kind), "ARC_WITHHELD")?;
        if kind == "pipe_elastic_normal_stress_maximum_v2" && !source_selected {
            crate::u4b_evidence::ARC_MAXIMUM_SIGN
        } else {
            crate::arc_evidence::sign(kind).unwrap_or(sign)
        }
    } else {
        sign
    };
    // T4-U6 (D-5, §2.1): a stress row's token is its recovery token followed by
    // its section basis; the recovery token is the one its kind states.
    let declared = md["basis"].as_str().unwrap_or_default();
    let declared = if crate::section_basis::is_stress_kind(kind) {
        crate::section_basis::split_token(declared).map_or("", |(token, _)| token)
    } else {
        declared
    };
    require(
        md["coordinate_system"] == frame
            && declared == basis
            && location_ok
            && md["sign_convention"] == sign,
        "ROW_SEMANTICS",
    )
}

/// The current ordinary pressure reader. An authored load/reference-state
/// group receives its closed pre-pass before the pressure evidence checks.
/// W-16 slice b (DEL-05-02 Design `exact-combinations.md` §6): a document with
/// a load combination takes these direct-case checks on its view without the
/// combination rows and record, then the combination pass on the whole
/// document (`exact_combination_evidence`).
pub fn validate_pressure_evidence(source: &Value) -> Check {
    if crate::exact_combination_evidence::present(source) {
        let view = crate::exact_combination_evidence::direct_view(source)?;
        validate_direct_pressure_evidence(&view)?;
        return crate::exact_combination_evidence::validate(source);
    }
    validate_direct_pressure_evidence(source)
}
fn validate_direct_pressure_evidence(source: &Value) -> Check {
    if source["contract_evidence"]
        .get("load_reference_states")
        .is_some()
    {
        crate::load_reference::validate_pressure_load_reference_evidence(source)
    } else {
        validate_physics_evidence_in(source, false, PressureContract::PressureV3)
    }
}
/// The physics checks under an explicitly selected exact pressure contract.
/// A v3 source composite reads its received 0.4 resolved state directly.
pub(crate) fn validate_physics_evidence_in(
    source: &Value,
    composite: bool,
    contract: PressureContract,
) -> Check {
    finite_tree(source)?;
    // W-15 slice d (DEL-03-05 Design §5): the component bodies and their
    // reference-section records, checked once over rows, extrema and coverage.
    let bodies = crate::component_body::source_bodies(source, crate::semantic_contract::pressure_rows_contract())?;
    require(
        (composite || source.get("source_block_recovery").is_none())
            && source.get("carrier_evidence").is_none(),
        "FOREIGN_METHOD_EVIDENCE",
    )?;
    let evidence = &source["contract_evidence"];
    let resolved = composite && contract == PressureContract::PressureV3
        && evidence.get("load_reference_states").is_some();
    if resolved {
        // The direct v3 context binds the received resolved fields. It never
        // projects them onto the v2 namespace or recomputes a commitment.
        crate::load_reference::prepass(source, true, crate::load_reference::Method::PressureJoined)?;
    }
    let fields = if resolved { RESOLVED_EVIDENCE_KEYS } else { EVIDENCE_KEYS };
    require(keys(evidence, fields), "EVIDENCE_SHAPE")?;
    // T4-U3: connector records are admitted under pressure-2 only (checked,
    // with their rows, by `connector_evidence` once the cases are read).
    require(
        (!composite && contract == PressureContract::PressureV3) || array(&evidence["connector"])?.is_empty(),
        "CONNECTOR_UNSUPPORTED",
    )?;
    let cases = indexed(&evidence["exact_cases"], "load_case_id")?;
    // T4-U2: the realized-arc members of each case's pressure regions. The
    // pressure-source-1 composite never carries a realized arc (D7 declines
    // every document with one), so an arc region there is refused by name
    // (`SOURCE_PHYSICS_ARC_UNSUPPORTED`).
    let arcs_admitted = !composite && contract == PressureContract::PressureV3;
    let mut arc_members: HashSet<(&str, &str)> = HashSet::new();
    // §2.5: each member a region's geometry records, by case; its record and its
    // case section mark the same arcs.
    let mut region_members: HashSet<(&str, &str)> = HashSet::new();
    for region in array(&evidence["pressure"])? {
        let case = region["load_case_id"].as_str().unwrap_or_default();
        for pipe in crate::arc_evidence::members(region, arcs_admitted)? {
            arc_members.insert((case, pipe));
        }
        for g in region["geometry"].as_array().map(Vec::as_slice).unwrap_or(&[]) {
            if let Some(pipe) = g["pipe_id"].as_str() {
                region_members.insert((case, pipe));
            }
        }
    }
    // RV23 NOTE-4: a case's structural members are its published pipes and
    // its objective connectors (pressure-2); a model whose only pipes are
    // replaced spans has no published pipe and is still covered.
    let has_connector = !array(&evidence["connector"])?.is_empty();
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
    // T4-U4b: the arc maxima's records, checked with their station cover.
    let mut arc_extrema: Vec<(&str, &str, &str, &Value, &Value)> = Vec::new();
    // T4-U6 (D-5, §2.2): one coverage and one headline per section basis.
    let mut coverage_complete = [true; 2];
    let mut all_members: Option<HashSet<&str>> = None;
    // T4-U6/U8: each case's checked sections, and the document's one straight
    // shear model (§3.6: the selection is per document; arcs stay Euler-Bernoulli).
    let mut case_sections: HashMap<(&str, &str), crate::section_basis::Sections> = HashMap::new();
    let mut straight_models: HashSet<&str> = HashSet::new();
    // §2.5: the document's realized arcs, as every case's `pipe_sections`
    // marks them (`member_kind = "realized_arc"`), including arcs outside every
    // pressure region; a model fact, so every case marks the same members.
    let mut marked_arcs: Option<HashSet<&str>> = None;
    // N2b (RV120; WORKING_ITEMS' ruling): every loop of this validator runs in array order,
    // as TS's and PY's do, so its first failure (the code) never depends on a hash map's or
    // set's per-process order. The maps and sets only detect duplicates and answer lookups.
    for case in array(&evidence["exact_cases"])? {
        let case_id = text(&case["load_case_id"])?;
        // T4-U4b (§2.6): `intensified_measures` only when non-empty, and
        // never on the pressure-source-1 composite.
        let directional = case.get("intensified_measures").is_some();
        require(!directional || (!composite && case["intensified_measures"].as_array().is_some_and(|a| !a.is_empty())), "CASE_SHAPE")?;
        // W-16 slice c (DEL-05-01 Design `exact-equivalent-static.md` §2): a
        // generating case's closed record; never on a selected retained case.
        let generation = case.get("equivalent_static").is_some();
        if generation {
            equivalent_static_record(case_id, &case["equivalent_static"])?;
            require(!composite || case["recovery_method"] != "retained_source_blocks_exact_v1", "CASE_SHAPE")?;
        }
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
                ]
                .iter()
                .copied()
                .chain(composite.then_some("recovery_method"))
                .chain(directional.then_some("intensified_measures"))
                .chain(generation.then_some("equivalent_static"))
                .collect::<Vec<_>>()
                .as_slice(),
            ),
            "CASE_SHAPE",
        )?;
        require(case["profile_mode"] == contract.mode(), "CASE_PROFILE")?;
        text(&case["material_basis"])?;
        let materials = indexed(&case["pipe_materials"], "pipe_id")?;
        let sections = indexed(&case["pipe_sections"], "pipe_id")?;
        let members: HashSet<_> = materials.keys().copied().collect();
        require(
            (!members.is_empty() || has_connector) && members == sections.keys().copied().collect(),
            "MEMBER_COVERAGE",
        )?;
        if let Some(expected) = &all_members {
            require(*expected == members, "CASE_MEMBER_COVERAGE")?;
        } else {
            all_members = Some(members.clone());
        }
        for m in array(&case["pipe_materials"])? {
            material_in(m, resolved)?;
        }
        let mut case_arcs: HashSet<&str> = HashSet::new();
        for s in array(&case["pipe_sections"])? {
            let (sections, arc) = crate::section_basis::marked_section(s)?;
            let pipe = text(&s["pipe_id"])?;
            // §2.5: an arc is known by its marker, and a region records a member
            // as an arc exactly when its case section is marked. The
            // pressure-source-1 composite carries none.
            require(!(composite && arc), "ARC_UNSUPPORTED")?;
            require(
                !region_members.contains(&(case_id, pipe)) || arc == arc_members.contains(&(case_id, pipe)),
                "ARC_MARKER",
            )?;
            // §3.1: kappa_E from this case's nu (per resolved case on load-state
            // documents); a marked arc is Euler-Bernoulli.
            crate::section_basis::beam_theory(&sections, number(&materials[pipe]["nu"])?, arc)?;
            if arc {
                case_arcs.insert(pipe);
            } else {
                straight_models.insert(crate::section_basis::model(&sections));
            }
            case_sections.insert((case_id, pipe), sections);
        }
        require(straight_models.len() <= 1, "SHEAR_MODEL_CONSISTENCY")?;
        match &marked_arcs {
            Some(arcs) => require(*arcs == case_arcs, "ARC_MARKER")?,
            None => marked_arcs = Some(case_arcs.clone()),
        }
        let coverage = crate::section_basis::coverage(&case["stress_maximum_coverage"])?;
        let mut unavailable_by_basis: [HashSet<&str>; 2] = [HashSet::new(), HashSet::new()];
        for (slot, ((unavailable, complete), done)) in coverage
            .iter()
            .zip(coverage_complete.iter_mut())
            .enumerate()
        {
            let unavailable: HashSet<&str> = unavailable.iter().copied().collect();
            require(unavailable.is_subset(&members), "STRESS_COVERAGE")?;
            *done &= *complete;
            unavailable_by_basis[slot] = unavailable;
        }
        // §2.5: one extremum per (pipe, basis) available on that basis.
        let mut extrema_by_basis: [HashMap<&str, &Value>; 2] = [HashMap::new(), HashMap::new()];
        for ex in array(&case["pipe_stress_extrema"])? {
            let basis = crate::section_basis::extremum_basis(ex)?;
            let slot = usize::from(basis == crate::section_basis::REDUCED);
            require(
                extrema_by_basis[slot].insert(text(&ex["pipe_id"])?, ex).is_none(),
                "DUPLICATE_ID",
            )?;
        }
        for slot in 0..2 {
            require(
                extrema_by_basis[slot].keys().copied().collect::<HashSet<_>>()
                    == members.difference(&unavailable_by_basis[slot]).copied().collect(),
                "EXTREMA_MEMBER_COVERAGE",
            )?;
        }
        for ex in array(&case["pipe_stress_extrema"])? {
            let pipe = text(&ex["pipe_id"])?;
            let basis = crate::section_basis::extremum_basis(ex)?;
            if composite && case["recovery_method"] == crate::physics_source::EXACT {
                crate::physics_source::validate_maximum(source, case, ex, &rows)?;
                require(
                    extrema_ids.insert(text(&ex["result_id"])?),
                    "EXTREMA_RESULT_DUPLICATE",
                )?;
                continue;
            }
            require(
                keys(
                    &crate::component_body::extremum_without_record(ex),
                    &[
                        "pipe_id",
                        "section_basis",
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
                    && case_basis(row)? == case_id
                    && crate::section_basis::split_token(row["metadata"]["basis"].as_str().unwrap_or_default())
                        .map(|(_, b)| b)
                        == Some(basis),
                "EXTREMA_RESULT_BINDING",
            )?;
            // T4-U4b (§2.3): an arc's record carries the arc tokens, checked
            // with its station cover once the rows are read; the arc tokens on a
            // straight member are refused.
            if case_arcs.contains(pipe) {
                arc_extrema.push((case_id, pipe, basis, ex, *row));
                continue;
            }
            require(ex["approximation"] != crate::u4b_evidence::ARC_APPROXIMATION, "EXTREMA_BASIS")?;
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
                    && upper == global
                    && gap >= 0.0
                    && gap <= 1e-12 + 1e-12 * lower
                    && gap >= global - lower
                    && number(&row["value"])? == lower + 0.5 * (upper - lower),
                "EXTREMA_BOUNDS",
            )?;
            require(ex["approximation"] == "piecewise_quadratic_straight_section_statics" && ex["coefficient_basis"] == "j_side_section_equilibrium_binary64_enclosures" && ex["enclosure_scope"] == "supplied_binary64_polynomial_coefficient_enclosures; solution error is separate", "EXTREMA_BASIS")?;
        }
        // Each pipe's two maxima against its two sections, in array order.
        for ex in array(&case["pipe_stress_extrema"])? {
            let pipe = text(&ex["pipe_id"])?;
            if crate::section_basis::extremum_basis(ex)? == crate::section_basis::NOMINAL {
                if let Some(reduced) = extrema_by_basis[1].get(pipe) {
                    crate::section_basis::extrema_twins(ex, reduced, &case_sections[&(case_id, pipe)])?;
                }
            }
        }
        case_members.insert(case_id, members);
    }
    let marked_arcs = marked_arcs.unwrap_or_default();
    let mut slots = HashSet::new();
    let mut support_components: HashMap<(&str, &str), HashSet<&str>> = HashMap::new();
    for row in array(&source["results"])? {
        let id = text(&row["id"])?;
        number(&row["value"])?;
        text(&row["unit"])?;
        text(&row["entity_ref"])?;
        require(
            row.get("source_result_refs")
                .is_none_or(|v| v.as_array().is_some_and(|a| a.is_empty())),
            "DERIVED_ROWS_UNSUPPORTED",
        )?;
        // T4-U2 (pressure-2): an input review row (a realized bend's
        // flexibility, a hanger's or a component's entered values) is model
        // scoped and states no load case. Only a pressure-2 row without
        // `basis_ref` whose signature is `review_evidence` qualifies; every
        // other row stays case scoped, and physics-1 keeps its case-scoped
        // rule. The contract is checked before the row's one signature lookup.
        let scoped = contract == PressureContract::PressureV3 && row.get("basis_ref").is_none();
        let case = if scoped { None } else { Some(row_case(row, &cases)?) };
        let kind = text(&row["kind"])?;
        let signature = crate::semantic_contract::signature_in(
            if composite {
                crate::semantic_contract::pressure_source_contract()
            } else {
                contract.table()
            },
            row,
        )?;
        let case = match case {
            Some(case) => case,
            None if signature.is_some_and(|s| s["category"] == "review_evidence") => continue,
            None => row_case(row, &cases)?,
        };
        // T4-U6 (§2.4): an inherited thin-wall row names no section basis and is
        // refused on an exact envelope; every stress row names its basis (token
        // and id) and belongs to a member whose sections the case states.
        require(!crate::section_basis::is_thin_wall_kind(kind), "ROW_SECTION_BASIS")?;
        let section_basis = if crate::section_basis::is_stress_kind(kind) {
            let (_, basis) = crate::section_basis::row_basis(row)?;
            require(
                case_members[case].contains(text(&row["entity_ref"])?),
                "ROW_MEMBER_UNRESOLVED",
            )?;
            basis
        } else {
            ""
        };
        physical_row(
            row,
            kind,
            signature,
            composite && cases[case]["recovery_method"] == crate::physics_source::EXACT,
            marked_arcs.contains(text(&row["entity_ref"])?),
        )?;
        // T4-U4b (§5.1): the new kinds' signatures and declared semantics; the
        // pressure-source-1 composite carries the straight-member derived kinds
        // only (§4.3: no arc reaches its source family).
        if crate::u4b_evidence::KINDS.contains(&kind) {
            require(
                signature.is_some() && (!composite || [crate::u4b_evidence::EXTREME, crate::u4b_evidence::SHEAR_MAX].contains(&kind)),
                "ROW_SIGNATURE",
            )?;
            crate::u4b_evidence::row_semantics(row, crate::u4b_evidence::Route::Exact, marked_arcs.contains(text(&row["entity_ref"])?))?;
        }
        // W-7 (DEL-04-01 Design `global-end-actions.md`): the global end
        // actions on pressure-2 (the wall basis admitted); a pressure-source-1
        // composite never carries them (`global_end_rows_published`).
        if crate::global_end_evidence::is_kind(kind) {
            require(signature.is_some() && !composite, "ROW_SIGNATURE")?;
            crate::global_end_evidence::row_semantics(row, true)?;
        }
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
                    section_basis,
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
    // T4-U6 (D-5, §2.1, §2.5): both families of every stress signature. Each
    // nominal row's twin is the row of the same id with the reduced segment, the
    // same recovery token and the same declared semantics; each pair is checked
    // against its member's two sections (array order, as every loop here).
    let mut lame: std::collections::BTreeMap<(&str, &str, &str, &str), HashMap<&str, f64>> =
        std::collections::BTreeMap::new();
    let (mut stress_rows, mut pairs) = (0usize, 0usize);
    let published = array(&source["results"])?;
    for (position, row) in published.iter().enumerate() {
        let kind = text(&row["kind"])?;
        if !crate::section_basis::is_stress_kind(kind) {
            continue;
        }
        let (token, basis) = crate::section_basis::row_basis(row)?;
        let case = case_basis(row)?;
        let entity = text(&row["entity_ref"])?;
        let sections = case_sections
            .get(&(case, entity))
            .ok_or("SOURCE_PHYSICS_ROW_MEMBER_UNRESOLVED")?;
        stress_rows += 1;
        let md = &row["metadata"];
        if matches!(kind, "pipe_lame_radial_stress_v2" | "pipe_lame_hoop_stress_v2") {
            require(
                lame.entry((case, entity, text(&md["location"])?, basis))
                    .or_default()
                    .insert(text(&md["component"])?, number(&row["value"])?)
                    .is_none(),
                "ROW_SLOT_DUPLICATE",
            )?;
        }
        if basis != crate::section_basis::NOMINAL {
            continue;
        }
        let twin = crate::section_basis::twin_id(text(&row["id"])?)
            .and_then(|id| rows.get(id.as_str()).copied())
            .ok_or("SOURCE_PHYSICS_ROW_BASIS_TWIN")?;
        let tmd = &twin["metadata"];
        require(
            crate::section_basis::row_basis(twin)? == (token, crate::section_basis::REDUCED)
                && twin["kind"] == row["kind"]
                && twin["unit"] == row["unit"]
                && twin["entity_ref"] == row["entity_ref"]
                && twin["basis_ref"] == row["basis_ref"]
                && ["component", "coordinate_system", "location", "sign_convention"]
                    .iter()
                    .all(|k| tmd[*k] == md[*k]),
            "ROW_BASIS_TWIN",
        )?;
        // T4-U4b: D-5's row order (section-bases-and-timoshenko.md §2.5) for the
        // U4b families and the arc maxima: the reduced twin immediately follows
        // its nominal row.
        if crate::u4b_evidence::KINDS.contains(&kind) || (kind == "pipe_elastic_normal_stress_maximum_v2" && marked_arcs.contains(entity)) {
            require(published.get(position + 1).is_some_and(|next| std::ptr::eq(next, twin)), "ROW_BASIS_TWIN_ORDER")?;
        }
        crate::section_basis::twin_values(kind, number(&row["value"])?, number(&twin["value"])?, sections)?;
        pairs += 1;
    }
    require(2 * pairs == stress_rows, "ROW_BASIS_TWIN")?;
    crate::connector_evidence::validate(
        &evidence["connector"],
        &evidence["pressure"],
        &rows,
        all_members.as_ref().unwrap_or(&HashSet::new()),
        &cases.keys().copied().collect(),
        source["status"]["mechanics"] == "MECHANICS_SOLVED",
        contract == PressureContract::PressureV3,
    )?;
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
    let mut member_pressure: HashMap<(&str, &str), f64> = HashMap::new();
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
        let arcs = crate::arc_evidence::members(region, arcs_admitted)?;
        require(
            crate::arc_evidence::region_shape(region, REGION_KEYS, !arcs.is_empty()),
            "REGION_SHAPE",
        )?;
        require(
            region["profile_version"] == contract.version()
                && region["profile_mode"] == contract.mode()
                && region["pressure_basis"] == "internal_differential_zero_external_v1"
                && region["external_pressure_increment_pa"] == 0.0
                && (!arcs.is_empty()
                    || region["approximation"] == "long_straight_annulus_small_strain_v2"
                    || region["approximation"] == crate::connector_evidence::JOINT_APPROXIMATION),
            "REGION_PROFILE",
        )?;
        let p = number(&region["p_pa"])?;
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
        for member in array(&region["member_pipe_ids"])? {
            let member = text(member)?;
            require(
                owned_members.insert((case, member)),
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
            for duplicate in array(&region[field])? {
                let member = text(&duplicate["pipe_id"])?;
                // §2.5: an arc's record adds only its end tangents to its case
                // section, which carries the marker.
                let dropped: Vec<&str> = if field == "geometry" && arcs.contains(member) {
                    [extra].into_iter().chain(crate::arc_evidence::REGION_GEOMETRY_KEYS).collect()
                } else {
                    vec![extra]
                };
                require(
                    if resolved && field == "materials" {
                        MATERIAL_KEYS.iter().all(|key| duplicate[*key] == basis[member][*key])
                    } else { without_all(duplicate, &dropped) == *basis[member] },
                    "REGION_DUPLICATE_CONTRADICTION",
                )?;
                if field == "geometry" {
                    require(duplicate[extra].is_boolean(), "TRAVERSAL_FLAG")?;
                } else if resolved {
                    require(duplicate[extra] == crate::load_reference::REGION_TEMPERATURE_BASIS, "TEMPERATURE_CASE_BINDING")?;
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
        for item in array(&region["applied_loads"])? {
            // An arc member's record is the bend term's (checked below).
            if arcs.contains(text(&item["pipe_id"])?) {
                continue;
            }
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
        if !arcs.is_empty() {
            crate::arc_evidence::region(region, &arcs, p)?;
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
        for id in array(&region["result_ids"])? {
            require(bound_results.insert(text(id)?), "REGION_RESULT_DUPLICATE")?;
        }
        for member in array(&region["member_pipe_ids"])? {
            let member = text(member)?;
            // T4-U6 (§2.1): a stress slot is published on both bases; a force
            // slot once.
            let covered = |kind: &str, component: &str, station: &str| {
                if crate::section_basis::is_stress_kind(kind) {
                    crate::section_basis::BASES
                        .iter()
                        .all(|basis| slots.contains(&(case, member, kind, component, station, *basis)))
                } else {
                    slots.contains(&(case, member, kind, component, station, ""))
                }
            };
            member_pressure.insert((case, member), p);
            if arcs.contains(member) {
                for (kind, component, stations) in crate::arc_evidence::ROWS {
                    for station in stations {
                        require(covered(kind, component, station), "PRESSURE_ROW_COVERAGE")?;
                    }
                }
                continue;
            }
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
                        require(covered(kind, component, station), "PRESSURE_ROW_COVERAGE")?;
                    }
                }
            }
        }
    }
    // T4-U6 (§2.1): each straight member's Lame rows on each basis against that
    // basis's own annulus and its region's pressure.
    for ((case, member, _, basis), values) in &lame {
        let p = *member_pressure
            .get(&(*case, *member))
            .ok_or("SOURCE_PHYSICS_PRESSURE_ROW_UNBOUND")?;
        crate::section_basis::lame_station(values, case_sections[&(*case, *member)].basis(basis), p)?;
    }
    for row in array(&source["results"])? {
        let id = text(&row["id"])?;
        if pressure_kind(row["kind"].as_str().unwrap_or("")) {
            require(bound_results.contains(id), "PRESSURE_ROW_UNBOUND")?;
        }
    }
    // T4-U4b (§5.1; S4 round 2): the station checks run on the pressure-source-1
    // composite too (straight members only: σ±, τ from the case's ν and basis
    // sections, and station × basis coverage), for exact-recovered and
    // ordinary-method cases alike.
    u4b_cases(source, evidence, &rows, &case_sections, &marked_arcs, &arc_extrema, composite)?;
    for case in array(&evidence["exact_cases"])? {
        assembly(
            &case["pressure_rhs_assembly"],
            text(&case["load_case_id"])?,
            &regions,
            contract,
            &evidence["connector"],
        )?;
    }
    // W-7: one global end row per member, end and component; none on a composite.
    // Checked after the published rows' own coverage, so a missing local or wall
    // end row is refused by that coverage, not by the global row derived from it.
    if !composite {
        crate::global_end_evidence::coverage(published)?;
    }
    // T4-U6 (D-5, §2.2): no governing basis; one headline per basis.
    // W-15 slice d: the headlines range over the non-body members.
    let maxima: Vec<&Value> = array(&source["results"])?
        .iter()
        .filter(|r| r["kind"] == "pipe_elastic_normal_stress_maximum_v2"
            && !bodies.contains_key(r["entity_ref"].as_str().unwrap_or_default()))
        .collect();
    for basis in crate::section_basis::BASES {
        crate::component_body::headline_not_body(&source["summary"][format!("max_open_formula_stress_{basis}")], &bodies)?;
    }
    crate::section_basis::headlines(&source["summary"], &rows, &maxima, coverage_complete)
}

/// T4-U4b (§5.1) on pressure-2: each member's σ±, τ (bit for bit with the
/// case's ν and basis sections), hoop and radial rows (bit-equal from the
/// region's p and R); each arc maximum's record with its station cover; the
/// directional records and rows.
fn u4b_cases(
    source: &Value,
    evidence: &Value,
    rows: &HashMap<&str, &Value>,
    case_sections: &HashMap<(&str, &str), crate::section_basis::Sections>,
    marked_arcs: &HashSet<&str>,
    arc_extrema: &[(&str, &str, &str, &Value, &Value)],
    composite: bool,
) -> Check {
    use crate::u4b_evidence as u;
    let route = u::Route::Exact;
    let results = array(&source["results"])?;
    let index = u::Index::new(results, route)?;
    let diagnostics = array(&source["diagnostics"])?;
    // S4 round 3: a pressure-source-1 composite's materials require -1 < ν < 1/2
    // (`material_in`), so `TRANSVERSE_SHEAR_POISSON_OUT_OF_DOMAIN` can never
    // name one of its members: on the composite the warning is refused by name
    // and exempts nothing, so every member's τ coverage is checked.
    if composite {
        require(!diagnostics.iter().any(|d| d["code"] == u::POISSON_OUT_OF_DOMAIN), "POISSON_DOMAIN_WARNING")?;
    }
    let mut covers: HashMap<(&str, &str, String), Vec<f64>> = HashMap::new();
    // Review item 3: the exact route admits only -1 < nu < 1/2 (the validated
    // homogeneous_isotropic_E_nu_v1 pair), so no member is exempt from τ
    // coverage and the warning is refused on ordinary pressure-2 too (the
    // composite has refused it by its own name above); every basis forms its c_b.
    require(!diagnostics.iter().any(|d| d["code"] == u::POISSON_OUT_OF_DOMAIN), "SHEAR_POISSON_EXEMPTION")?;
    for case in array(&evidence["exact_cases"])? {
        let case_id = text(&case["load_case_id"])?;
        let materials = indexed(&case["pipe_materials"], "pipe_id")?;
        // The case's pressurized arcs: (p, R) from their region.
        let mut pressurized: HashMap<&str, (f64, f64)> = HashMap::new();
        for region in array(&evidence["pressure"])?.iter().filter(|r| r["load_case_id"] == case_id) {
            let p = number(&region["p_pa"])?;
            for g in array(&region["geometry"])? {
                if g.get("bend_radius_m").is_some() {
                    pressurized.insert(text(&g["pipe_id"])?, (p, number(&g["bend_radius_m"])?));
                }
            }
        }
        for s in array(&case["pipe_sections"])? {
            let pipe = text(&s["pipe_id"])?;
            let sections = &case_sections[&(case_id, pipe)];
            let nu = number(&materials[pipe]["nu"])?;
            let arc = marked_arcs.contains(pipe);
            let bases = crate::section_basis::BASES.to_vec();
            let facts = u::MemberFacts {
                case: case_id,
                pipe,
                shear_factors: bases.iter().map(|b| {
                    let a = sections.basis(b);
                    (*b, u::shear_factor(nu, sections.ro, a.ri, a.second_moment))
                }).collect(),
                poisson_out_of_domain: false,
                toroidal: match pressurized.get(pipe) {
                    Some(&(p, r)) if arc => bases.iter().map(|b| {
                        let a = sections.basis(b);
                        (*b, (p, a.ri, a.wall, sections.ro, r))
                    }).collect(),
                    _ => Vec::new(),
                },
                bases,
            };
            // §2.5: a pressurized arc with R not above r_o carries the named
            // warning (its hoop and radial rows are absent, checked below).
            if let (Some(&(_, r)), true) = (pressurized.get(pipe), arc) {
                if !(r > sections.ro) {
                    require(diagnostics.iter().any(|d| d["code"] == u::TOROIDAL_DOMAIN && d["affected_refs"][0] == case_id && d["affected_refs"][1] == pipe), "TOROIDAL_DOMAIN_WARNING")?;
                }
            }
            require(facts.shear_factors.iter().all(|(_, factor)| factor.is_some()), "SHEAR_POISSON_DOMAIN")?;
            for (basis, cover) in u::member_stations(&index, route, &facts)? {
                covers.insert((case_id, pipe, basis), cover);
            }
        }
        // Directional records: every directional row bound, by key set.
        let records = case.get("intensified_measures").and_then(Value::as_array).map(Vec::as_slice).unwrap_or(&[]);
        let bound = u::directional_records(route, case_id, records, rows, &index, marked_arcs, true)?;
        require(bound.len() == records.len(), "INTENSIFIED_SHAPE")?;
        for row in results.iter().filter(|r| r["basis_ref"]["ref_id"] == case_id) {
            if matches!(row["kind"].as_str(), Some(u::DIRECTIONAL | u::DIRECTIONAL_MAX)) {
                require(bound.contains(text(&row["id"])?), "INTENSIFIED_ROW_UNBOUND")?;
            }
        }
    }
    for &(case, pipe, basis, record, row) in arc_extrema {
        require(row["metadata"]["sign_convention"] == u::ARC_MAXIMUM_SIGN, "ROW_SEMANTICS")?;
        let cover = covers.get(&(case, pipe, basis.to_string())).map(Vec::as_slice).unwrap_or(&[]);
        u::maximum_record(route, record, row, u::ARC_APPROXIMATION, cover)?;
    }
    Ok(())
}

fn assembly(
    v: &Value,
    case: &str,
    regions: &HashMap<(&str, &str), &Value>,
    contract: PressureContract,
    connectors: &Value,
) -> Check {
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
    let mut joint_face_sources = HashSet::new();
    for group in array(&v["groups"])? {
        let joint = group.get("effective_area_bits").is_some();
        if joint {
            require(contract == PressureContract::PressureV3 && keys(group, &["node_ref","component","pressure_bits","effective_area_bits","direction_component_magnitude","coefficient_sum","assembled_force_n","terms"]), "RHS_GROUP_SHAPE")?;
            let node = text(&group["node_ref"])?;
            let axis = match group["component"].as_str() { Some("Fx")=>0, Some("Fy")=>1, Some("Fz")=>2, _=>return Err("SOURCE_PHYSICS_RHS_GROUP_NODE".into()) };
            require(nodes.contains(node), "RHS_GROUP_NODE")?;
            let magnitude = number(&group["direction_component_magnitude"])?;
            // An authored connector axis may carry a component just above 1
            // (FK's orthonormality guard admits it), so the magnitude is bound
            // exactly to the axis below (`RHS_JOINT_DIRECTION`), not to 1.
            require(magnitude > 0.0, "RHS_DIRECTION")?;
            let area_bits = text(&group["effective_area_bits"])?;
            let pressure_bits = text(&group["pressure_bits"])?;
            require([area_bits,pressure_bits].iter().all(|bits| bits.len()==16 && u64::from_str_radix(bits,16).is_ok_and(|bits| f64::from_bits(bits).is_finite())), "RHS_SOURCE_BITS")?;
            require(groups.insert((node,text(&group["component"])?,pressure_bits,area_bits,"joint",magnitude.to_bits())), "RHS_GROUP_DUPLICATE")?;
            let terms = array(&group["terms"])?;
            require(!terms.is_empty(), "RHS_GROUP_TERMS")?;
            let mut coefficient_sum = 0.0;
            let mut joint_sources = HashSet::new();
            for term in terms {
                require(keys(term,&["coefficient","region_id","connector_id","kind"]) && term["kind"] == "joint_effective_area", "RHS_TERM_SHAPE")?;
                let id = text(&term["connector_id"])?;
                require(joint_sources.insert((text(&term["region_id"])?,id,node,axis)), "RHS_JOINT_SOURCE_DUPLICATE")?;
                let region = regions.get(&(case,text(&term["region_id"])?)).ok_or("SOURCE_PHYSICS_RHS_REGION_UNRESOLVED")?;
                let record = array(connectors)?.iter().find(|record| record["component_id"]==id).ok_or("SOURCE_PHYSICS_RHS_JOINT_UNRESOLVED")?;
                require(region["member_connector_ids"].as_array().is_some_and(|ids| ids.iter().any(|entry|entry==id)) && record["pressure_model"]=="constant_effective_area_reference_v1", "RHS_JOINT_BINDING")?;
                let p = number(&region["p_pa"])?;
                let ae = number(&record["effective_area_m2"])?;
                require(u64::from_str_radix(area_bits,16).ok()==Some(ae.to_bits()) && u64::from_str_radix(pressure_bits,16).ok()==Some(p.to_bits()), "RHS_JOINT_OPERANDS")?;
                let direction = number(&record["connector_axes_global"][axis][0])?;
                let end_sign = if record["node_i"]==node {-1.0} else if record["node_j"]==node {1.0} else {return Err("SOURCE_PHYSICS_RHS_JOINT_NODE".into())};
                let coefficient = number(&term["coefficient"])?;
                require(direction.abs()==magnitude && coefficient==end_sign*if direction.is_sign_negative(){-1.0}else{1.0}, "RHS_JOINT_DIRECTION")?;
                coefficient_sum += coefficient;
            }
            require(number(&group["coefficient_sum"])?==coefficient_sum, "RHS_JOINT_COEFFICIENT")?;
            // A joint face cannot meet another connector (§1); therefore a
            // joint-area group at a node has exactly one original source.
            require(terms.len()==1, "RHS_JOINT_SOURCE_CARDINALITY")?;
            let p = f64::from_bits(u64::from_str_radix(pressure_bits,16).unwrap());
            let ae = f64::from_bits(u64::from_str_radix(area_bits,16).unwrap());
            let expected = p*ae*magnitude*coefficient_sum;
            require(expected.is_finite() && (number(&group["assembled_force_n"])?-expected).abs() <= 64.0*f64::EPSILON*expected.abs()+f64::from_bits(1), "RHS_JOINT_VALUE")?;
            continue;
        }
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
        // DEL-04-07 Work `pressure-contract.md`, "Direction magnitudes of arc
        // tangents". A straight member's direction is its frame axis, the chord
        // scaled by a rounded reciprocal of a norm that dominates every
        // component, so it is at most 1. A realized arc's end tangent is formed
        // as c*d + s*n without renormalizing, and a component can round above 1;
        // a term on a realized arc of its region is therefore bound exactly to
        // the component, on the group's axis, of one of that arc's published
        // end tangents (`arc_end_tangents_global`), and a group with no arc term
        // keeps the bound 1.
        let axis = match group["component"].as_str() { Some("Fx") => 0, Some("Fy") => 1, _ => 2 };
        let mut on_arc = false;
        for term in array(&group["terms"])? {
            let (Some(region_id), Some(pipe)) = (term["region_id"].as_str(), term["pipe_id"].as_str()) else { continue };
            let Some(region) = regions.get(&(case, region_id)) else { continue };
            if !crate::arc_evidence::members(region, true).is_ok_and(|arcs| arcs.contains(pipe)) {
                continue;
            }
            on_arc = true;
            let tangents = array(&region["geometry"])?.iter().find(|g| g["pipe_id"] == pipe)
                .and_then(|g| g["arc_end_tangents_global"].as_array()).ok_or("SOURCE_PHYSICS_RHS_DIRECTION")?;
            require(tangents.iter().any(|t| t[axis].as_f64().is_some_and(|c| c.abs().to_bits() == magnitude.to_bits())), "RHS_DIRECTION")?;
        }
        require(magnitude > 0.0 && (on_arc || magnitude <= 1.0), "RHS_DIRECTION")?;
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
            // T4-U6 (D-5): the pressure load path is on the pressure bore, the
            // reduced wall's inner radius (`source_bore_key`).
            let a = number(&geometry["outside_diameter_m"])? * 0.5;
            let b = number(&geometry["reduced_section"]["wall_m"])?;
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
            // T4-U2: the bend and junction terms of a realized arc (v3 only).
            if contract == PressureContract::PressureV3 {
                let arcs = crate::arc_evidence::members(region, true)?;
                if let Some(check) = crate::arc_evidence::rhs_term(
                    term["kind"].as_str().unwrap_or_default(),
                    pipe,
                    coefficient,
                    region,
                    &arcs,
                ) {
                    check?;
                    continue;
                }
            }
            match term["kind"].as_str() {
                Some("terminal_cap" | "joint_face_cap") => {
                    require(coefficient.abs() == 1.0, "RHS_CAP_COEFFICIENT")?;
                    if term["kind"] == "joint_face_cap" {
                        let node=text(&group["node_ref"])?;
                        require(joint_face_sources.insert((text(&term["region_id"])?,pipe,node,text(&group["component"])?)), "RHS_JOINT_FACE_DUPLICATE")?;
                        let edge=array(&region["joint_edges"])?.iter().find(|edge| (edge["face_i_pipe_id"]==pipe && edge["node_i_ref"]==node) || (edge["face_j_pipe_id"]==pipe && edge["node_j_ref"]==node)).ok_or("SOURCE_PHYSICS_RHS_JOINT_FACE_BINDING")?;
                        let record=array(connectors)?.iter().find(|record|record["component_id"]==edge["connector_id"]).ok_or("SOURCE_PHYSICS_RHS_JOINT_FACE_BINDING")?;
                        let axis=match group["component"].as_str(){Some("Fx")=>0,Some("Fy")=>1,_=>2};
                        let expected=number(&record["connector_axes_global"][axis][0])?*if edge["node_i_ref"]==node{1.0}else{-1.0};
                        require(contract==PressureContract::PressureV3 && (coefficient*magnitude-expected).abs() <= 64.0*f64::EPSILON, "RHS_JOINT_FACE_DIRECTION")?;
                    }
                },
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

/// Current exact-retained G7 transport checks on the receipt's pressure-2
/// projection. Preserve the member/material/geometry/maximum/region/assembly
/// checks and their order; this cannot qualify a publication or invocation.
pub fn validate_transport_metadata(source: &Value) -> Check {
    let schema: Value =
        serde_json::from_str(include_str!("../../../../schemas/results.v0.3.schema.yaml"))
            .map_err(|e| e.to_string())?;
    let evidence = &source["contract_evidence"];
    require(
        crate::source_blocks::shape_in(
            evidence,
            &schema["$defs"]["ExactRetainedContractEvidence"],
            &schema,
        ),
        "TRANSPORT_SHAPE",
    )?;
    finite_tree(evidence)?;
    let cases = indexed(&evidence["exact_cases"], "load_case_id")?;
    let mut ids = HashSet::new();
    // N2 (RV120): this check's loops run in array order, as TS's and PY's do, so its first
    // failure (the code) does not depend on a hash map's per-process order. Its maps and
    // sets only detect duplicates and answer lookups.
    for case in array(&evidence["exact_cases"])? {
        let cid = text(&case["load_case_id"])?;
        let materials = indexed(&case["pipe_materials"], "pipe_id")?;
        let sections = indexed(&case["pipe_sections"], "pipe_id")?;
        require(
            !materials.is_empty()
                && materials.keys().copied().collect::<HashSet<_>>()
                    == sections.keys().copied().collect(),
            "TRANSPORT_MEMBERS",
        )?;
        for value in array(&case["pipe_materials"])? {
            material(value)?;
        }
        for value in array(&case["pipe_sections"])? {
            geometry(value)?;
        }
        // T4-U6 (§2.5): one coverage record and one maximum per pipe per basis.
        let members = sections.keys().copied().collect::<HashSet<_>>();
        let coverage = crate::section_basis::coverage(&case["stress_maximum_coverage"])
            .map_err(|_| "SOURCE_PHYSICS_TRANSPORT_MAXIMUM_COVERAGE".to_string())?;
        for (slot, (missing, _)) in coverage.iter().enumerate() {
            let missing: HashSet<&str> = missing.iter().copied().collect();
            require(missing.is_subset(&members), "TRANSPORT_MAXIMUM_COVERAGE")?;
            let basis = crate::section_basis::BASES[slot];
            let mut maxima = HashSet::new();
            for ex in array(&case["pipe_stress_extrema"])? {
                if crate::section_basis::extremum_basis(ex).map_err(|_| "SOURCE_PHYSICS_TRANSPORT_MAXIMUM_MEMBERS".to_string())? == basis {
                    require(maxima.insert(text(&ex["pipe_id"])?), "TRANSPORT_MAXIMUM_MEMBERS")?;
                }
            }
            require(
                maxima == members.difference(&missing).copied().collect(),
                "TRANSPORT_MAXIMUM_MEMBERS",
            )?;
        }
        for ex in array(&case["pipe_stress_extrema"])? {
            require(
                ids.insert(text(&ex["result_id"])?),
                "TRANSPORT_MAXIMUM_RESULT",
            )?;
            let lo = number(&ex["value_lower_pa"])?;
            let hi = number(&ex["value_upper_pa"])?;
            let upper = number(&ex["global_upper_bound_pa"])?;
            let gap = number(&ex["certified_gap_pa"])?;
            require(
                lo >= 0.0
                    && lo <= hi
                    && hi <= upper
                    && upper - lo <= gap
                    && gap >= 0.0
                    && gap <= 1e-12 + 1e-12 * lo,
                "TRANSPORT_MAXIMUM_BOUNDS",
            )?;
        }
        let regions = array(&evidence["pressure"])?
            .iter()
            .filter(|p| p["load_case_id"] == cid)
            .map(|p| Ok(((text(&p["load_case_id"])?, text(&p["region_id"])?), p)))
            .collect::<Result<HashMap<_, _>, String>>()?;
        assembly(&case["pressure_rhs_assembly"], cid, &regions, PressureContract::PressureV3, &json!([]))?;
    }
    let mut regions = HashSet::new();
    for region in array(&evidence["pressure"])? {
        let cid = text(&region["load_case_id"])?;
        let rid = text(&region["region_id"])?;
        require(
            regions.insert((cid, rid))
                && cases.contains_key(cid)
                && number(&region["p_pa"])? >= 0.0,
            "TRANSPORT_PRESSURE_REGION",
        )?;
        let case = cases[cid];
        let sections = indexed(&case["pipe_sections"], "pipe_id")?;
        let materials = indexed(&case["pipe_materials"], "pipe_id")?;
        let gm = indexed(&region["geometry"], "pipe_id")?;
        let mm = indexed(&region["materials"], "pipe_id")?;
        let members = strings(&region["member_pipe_ids"])?;
        require(
            !members.is_empty()
                && members == gm.keys().copied().collect()
                && members == mm.keys().copied().collect(),
            "TRANSPORT_REGION_MEMBERS",
        )?;
        for pid in array(&region["member_pipe_ids"])? {
            let pid = text(pid)?;
            require(
                sections
                    .get(pid)
                    .is_some_and(|s| without(gm[pid], "traversal_forward") == **s)
                    && materials
                        .get(pid)
                        .is_some_and(|m| without(mm[pid], "temperature_basis") == **m),
                "TRANSPORT_REGION_FACTS",
            )?;
            temperature(
                &mm[pid]["temperature_basis"],
                text(&case["material_basis"])?,
                text(&mm[pid]["material_id"])?,
            )?;
        }
    }
    Ok(())
}


#[cfg(test)]
#[path = "arc_marker_tests.rs"]
mod arc_marker_tests;

#[cfg(test)]
mod b7b_exact_transport_tests {
    use super::*;

    fn projected(mode: &str) -> Value {
        // Frozen invented predecessor values; only the declared exact tuple is
        // re-authored for this isolated transport check. No G0/receipt permit.
        let fixture: Value = serde_json::from_str(match mode {
            "sparse_interactive" => include_str!("../../../../fixtures/results/retained_precision_exact_successor_sparse_interactive.json"),
            _ => include_str!("../../../../fixtures/results/retained_precision_exact_successor_dense_scrutiny.json"),
        }).unwrap();
        let mut source = fixture["source"].clone();
        for case in source["contract_evidence"]["exact_cases"].as_array_mut().unwrap() {
            case["profile_mode"] = json!("exact_pressure_v3");
        }
        source
    }

    #[test]
    fn b7b_exact_transport_preserves_closed_shape_and_physical_checks() {
        for mode in ["sparse_interactive", "dense_scrutiny"] {
            let source = projected(mode);
            assert_eq!(validate_transport_metadata(&source), Ok(()), "{mode}");
            for (pointer, value, code) in [
                ("/contract_evidence/exact_cases/0/profile_mode", json!("exact_straight_pressure_v2"), "SOURCE_PHYSICS_TRANSPORT_SHAPE"),
                ("/contract_evidence/exact_cases/0/pipe_materials/0/G_pa", json!(1.0), "SOURCE_PHYSICS_MATERIAL_G_BINDING"),
                // T4-U6 (§2.5): the D-5 section and the per-basis coverage records.
                ("/contract_evidence/exact_cases/0/pipe_sections/0/nominal_section/ri_m", json!(0.08), "SOURCE_PHYSICS_GEOMETRY_SOURCE_BINDING"),
                ("/contract_evidence/exact_cases/0/pipe_sections/0/reduced_section/ri_m", json!(0.08), "SOURCE_PHYSICS_GEOMETRY_SOURCE_BINDING"),
                ("/contract_evidence/exact_cases/0/stress_maximum_coverage/0/complete", json!(false), "SOURCE_PHYSICS_TRANSPORT_MAXIMUM_COVERAGE"),
                ("/contract_evidence/exact_cases/0/stress_maximum_coverage/1/complete", json!(false), "SOURCE_PHYSICS_TRANSPORT_MAXIMUM_COVERAGE"),
                ("/contract_evidence/exact_cases/0/pipe_stress_extrema", json!([]), "SOURCE_PHYSICS_TRANSPORT_MAXIMUM_MEMBERS"),
                ("/contract_evidence/exact_cases/0/pressure_rhs_assembly/assembled_pressure_rhs_global/3", json!(1.0), "SOURCE_PHYSICS_RHS_PRESSURE_MOMENT"),
            ] {
                let mut changed = source.clone();
                *changed.pointer_mut(pointer).unwrap() = value;
                assert_eq!(validate_transport_metadata(&changed).unwrap_err(), code, "{mode} {pointer}");
            }
            let mut changed = source.clone();
            changed["contract_evidence"]["unexpected"] = Value::Null;
            assert_eq!(validate_transport_metadata(&changed).unwrap_err(), "SOURCE_PHYSICS_TRANSPORT_SHAPE");
            let mut changed = source.clone();
            let mut second = changed["contract_evidence"]["exact_cases"][0].clone();
            second["load_case_id"] = json!("second");
            second["pressure_rhs_assembly"]["load_case_id"] = json!("second");
            changed["contract_evidence"]["exact_cases"].as_array_mut().unwrap().push(second);
            assert_eq!(validate_transport_metadata(&changed).unwrap_err(), "SOURCE_PHYSICS_TRANSPORT_MAXIMUM_RESULT");
        }
    }
}

#[cfg(test)]
mod static_field_slice_tests {
    use super::*;
    fn material(resolved: bool) -> Value {
        let source: Value=serde_json::from_str(if resolved {
            include_str!("../../../../fixtures/product_preview/pressure_source/load_reference_source/n06-sparse_interactive.raw.json")
        } else {
            include_str!("../../../../fixtures/product_preview/pressure_source/physics_source/n06-sparse_interactive.raw.json")
        }).unwrap();
        source["contract_evidence"]["exact_cases"][0]["pipe_materials"][0].clone()
    }
    #[test]
    fn static_material_fields_preserve_exact_shapes_and_first_refusal() {
        for resolved in [false,true] {
            let base=material(resolved);
            assert_eq!(material_in(&base,resolved),Ok(()));
            let fields=if resolved {RESOLVED_MATERIAL_KEYS} else {MATERIAL_KEYS};
            assert_eq!(fields.len(),if resolved {11} else {9});
            for field in fields {
                let mut missing=base.clone();missing.as_object_mut().unwrap().remove(*field);
                assert_eq!(material_in(&missing,resolved),Err("SOURCE_PHYSICS_MATERIAL_SHAPE".into()));
            }
            let mut extra=base.clone();extra["unexpected"]=Value::Null;extra["E_pa"]=json!(-1.0);
            assert_eq!(material_in(&extra,resolved),Err("SOURCE_PHYSICS_MATERIAL_SHAPE".into()));
            assert_eq!(material_in(&base,!resolved),Err("SOURCE_PHYSICS_MATERIAL_SHAPE".into()));
        }
        assert_eq!(&RESOLVED_MATERIAL_KEYS[..MATERIAL_KEYS.len()],MATERIAL_KEYS);
    }
    #[test]
    fn static_evidence_fields_preserve_exact_member_sets() {
        for resolved in [false,true] {
            let fields=if resolved {RESOLVED_EVIDENCE_KEYS} else {EVIDENCE_KEYS};
            let mut value=Value::Object(fields.iter().map(|field|((*field).into(),Value::Null)).collect());
            assert!(keys(&value,fields));
            for field in fields {
                let mut missing=value.clone();missing.as_object_mut().unwrap().remove(*field);assert!(!keys(&missing,fields));
            }
            value["unexpected"]=Value::Null;assert!(!keys(&value,fields));
            assert_eq!(fields.len(),if resolved {4} else {3});
        }
        assert_eq!(&RESOLVED_EVIDENCE_KEYS[..EVIDENCE_KEYS.len()],EVIDENCE_KEYS);
    }
    #[test]
    fn static_field_slices_have_program_lifetime_and_positive_backing() {
        use std::mem::{size_of,size_of_val};
        fn program_lifetime(_: &'static [&'static str]) {}
        for fields in [MATERIAL_KEYS,RESOLVED_MATERIAL_KEYS,EVIDENCE_KEYS,RESOLVED_EVIDENCE_KEYS] {
            program_lifetime(fields);
            assert_eq!(size_of_val(fields),fields.len()*size_of::<&str>());
            assert!(size_of_val(fields)>0);
        }
        println!("static_field_backing={}",json!({"material":size_of_val(MATERIAL_KEYS),"resolved_material":size_of_val(RESOLVED_MATERIAL_KEYS),"evidence":size_of_val(EVIDENCE_KEYS),"resolved_evidence":size_of_val(RESOLVED_EVIDENCE_KEYS),"str_reference":size_of::<&str>(),"slice_reference":size_of::<&[&str]>()}));
    }
}
