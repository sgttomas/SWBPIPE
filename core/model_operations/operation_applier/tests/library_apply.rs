//! W-17 library apply (DEL-03-01 Design `apply-library-records.md` §2.2, §4):
//! `apply_library_record` on materials, sections and components, the
//! `provenance_record` carriage of the create kinds, and real solves of a
//! model holding an inserted E/nu/G material on the exact and pressure-free
//! routes. Every value here is invented.
use open_pipe_stress_operation_applier::{
    apply_operation, apply_operation_batch, canonical_json, sha256_hex, validate_model_document,
    validate_operation, document_validator::has_blocking,
};
use open_pipe_stress_product_physics::{
    run_linear_static_preview_with_mode, LinearStaticPreviewRequest, PreviewSolverMode,
};
use serde_json::{json, Value};

const E_PA: f64 = 2.0e11;
const NU: f64 = 0.3;
/// A stored G deliberately unequal to E/(2(1+nu)), so the route that uses it
/// is visible in the solve.
const STORED_G_PA: f64 = 7.0e10;
const TORQUE_N_M: f64 = 400.0;

fn exact_fixture() -> Value {
    serde_json::from_str(include_str!(
        "../../../../fixtures/model_operations/exact_pressure_authoring_model.json"
    ))
    .unwrap()
}

/// The same document on the pressure-free (non-exact) route.
fn pressure_free_fixture() -> Value {
    let mut model = exact_fixture();
    model.as_object_mut().unwrap().remove("pressure_contract");
    for case in model["load_cases"].as_array_mut().unwrap() {
        case.as_object_mut().unwrap().remove("pressure_regions");
    }
    // A corrosion allowance needs a pressure contract; the original material
    // needs its own G off the exact route.
    for pipe in model["pipe_segments"].as_array_mut().unwrap() {
        pipe["section"].as_object_mut().unwrap().remove("corrosion_allowance");
    }
    model["materials"][0]["shear_modulus"] = json!({"value":7.7e10,"unit":"Pa"});
    model
}

fn claim(model: &Value) -> Value {
    json!({"algorithm":"sha256","canonicalization":"rfc8785_jcs","payload_scope":"model_payload","value":format!("sha256:{}",sha256_hex(&canonical_json(model)))})
}

#[allow(clippy::too_many_arguments)]
fn intent(operation_kind: &str, change_kind: &str, object: &str, id: &str, path: &str, before: &str, after: &str, unit: &str, dimension: &str) -> Value {
    json!({"operation_id":format!("op:library:{change_kind}:{id}"),"operation_kind":operation_kind,"operation_status":"proposed","author_type":"user",
      "target":{"object_type":object,"ref":id},
      "change":{"change_id":format!("change:library:{change_kind}:{id}"),"change_kind":change_kind,"field_label":"Library record","field_path":path,"before":before,"after":after,"unit":unit,"dimension":dimension,"source_note":"library:material:invented-lib#invented-record"},
      "validation":{"schema_validation":"not_run","constraint_validation":"not_run","unit_validation":"not_run","diff_preview_status":"not_generated","application_status":"not_applied"},
      "rationale":"Invented library apply fixture",
      "audit_boundary":{"mutation_route":"structured_operations_only","direct_model_mutation_allowed":false,"requires_user_acceptance":true,"mutates_accepted_model_state":false},
      "professional_boundary":{"human_review_required":true,"software_makes_compliance_claim":false,"software_makes_certification_claim":false,"software_makes_sealing_claim":false,"software_makes_approval_claim":false,"software_makes_authentication_claim":false}})
}

/// `apply_library_record`: before is the current value of every named member
/// that exists.
fn apply_intent(model: &Value, object: &str, collection: &str, id: &str, after: &Value) -> Value {
    let entity = model[collection].as_array().unwrap().iter().find(|e| e["id"] == id).unwrap();
    let before: serde_json::Map<String, Value> = after
        .as_object()
        .unwrap()
        .keys()
        .filter_map(|path| {
            path.split('.').try_fold(entity, |v, s| v.get(s)).map(|v| (path.clone(), v.clone()))
        })
        .collect();
    intent("modify", "apply_library_record", object, id, "library_record", &canonical_json(&Value::Object(before)), &canonical_json(after), "none", "dimensionless")
}

fn applied(model: &Value, op: &Value) -> (Value, Vec<String>) {
    let hash = claim(model);
    let preview = validate_operation(model, op, Some(&hash));
    assert!(preview.applied_model.is_none());
    let outcome = apply_operation(model, op, Some(&hash));
    assert_eq!(
        serde_json::to_value(&preview.diff_preview).unwrap(),
        serde_json::to_value(&outcome.diff_preview).unwrap()
    );
    let next = outcome.applied_model.unwrap_or_else(|| panic!("{:?}", outcome.diagnostics));
    assert!(!has_blocking(&validate_model_document(&next)), "{:?}", validate_model_document(&next));
    (next, outcome.diff_preview.iter().map(|row| format!("{}:{}", row.entity_ref, row.field_path)).collect())
}

fn refused(model: &Value, op: &Value) -> Vec<String> {
    let outcome = apply_operation(model, op, Some(&claim(model)));
    assert!(outcome.applied_model.is_none(), "applied unexpectedly");
    outcome.diagnostics.iter().filter(|d| d.severity == "blocking").map(|d| d.code.clone()).collect()
}

fn record(library_kind: &str) -> Value {
    json!({"source_name":"Invented private library","source_location":"local store","redistribution_status":"private_only","review_status":"pending",
        "verification_note":"shear_modulus: derived from E and ν (isotropic), G = E/(2(1+ν))",
        "library_ref":{"library_kind":library_kind,"library_id":"invented-lib","library_version":format!("sha256:{}", "a".repeat(64)),"record_id":"invented-record"}})
}

fn derived_g(e: f64, nu: f64) -> f64 {
    open_pipe_stress_operation_applier::derived_shear_modulus_pa(e, "Pa", nu).unwrap()
}

/// The inserted material: E, nu and G with the exact basis (the HELP_HUMAN
/// E/nu rule: ν is never discarded; G is stored or derived).
fn inserted_material(id: &str, g: f64) -> Value {
    json!({"id":id,"label":"Invented library steel","constitutive_basis":"homogeneous_isotropic_E_nu_v1",
        "elastic_modulus":{"value":E_PA,"unit":"Pa"},"poisson_ratio":{"value":NU,"unit":"1"},"shear_modulus":{"value":g,"unit":"Pa"},
        "provenance":"library:material:invented-lib#invented-record","provenance_record":record("material")})
}

/// Insert the material and point the span at it, as one batch.
fn insert_and_use(model: &Value, material: &Value) -> Value {
    let id = material["id"].as_str().unwrap();
    let create = intent("create", "create_material", "Material", id, "materials", "not_present", &canonical_json(material), "Pa", "stress");
    let mut use_it = intent("modify", "set_field", "Element", "pipe:fixture-span", "material", "material:fixture-isotropic", id, "none", "dimensionless");
    use_it["operation_id"] = json!("op:library:use");
    use_it["change"]["change_id"] = json!("change:library:use");
    let batch = json!({"batch_id":"batch:library-insert","operations":[create, use_it]});
    let outcome = apply_operation_batch(model, &batch, Some(&claim(model)));
    let next = outcome["applied_model"].clone();
    assert!(next.is_object(), "{}", outcome["diagnostics"]);
    assert!(!has_blocking(&validate_model_document(&next)));
    next
}

fn tip_rotation_x(model: &Value) -> (f64, Vec<String>) {
    let request: LinearStaticPreviewRequest = serde_json::from_value(json!({"model":model,"materials":[]})).unwrap();
    let result = run_linear_static_preview_with_mode(request, PreviewSolverMode::DenseScrutiny);
    assert_eq!(result.status.mechanics, "MECHANICS_SOLVED", "{:?}", result.diagnostics);
    let row = result
        .results
        .iter()
        .find(|r| {
            r.basis_ref.as_ref().is_some_and(|b| b.ref_id == "case:six-component-load")
                && r.entity_ref == "node:fixture-tip"
                && r.kind == "global_nodal_rotation_x"
        })
        .expect("tip rotation row");
    let radians = match row.unit.as_str() {
        "rad" => row.value,
        "deg" => row.value.to_radians(),
        other => panic!("unexpected rotation unit {other}"),
    };
    (radians, result.diagnostics.iter().map(|d| d.code.clone()).collect())
}

/// theta = T L / (G J) for the fixture's 1 m cantilever (OD 0.12 m, wall 0.01 m).
fn torsion_rotation(g: f64) -> f64 {
    let j = std::f64::consts::PI / 2.0 * (0.06_f64.powi(4) - 0.05_f64.powi(4));
    TORQUE_N_M * 1.0 / (g * j)
}

fn close(actual: f64, expected: f64) {
    assert!((actual - expected).abs() <= expected.abs() * 1e-9, "{actual} vs {expected}");
}

#[test]
fn create_material_and_section_carry_provenance_record() {
    let model = pressure_free_fixture();
    let material = inserted_material("material:library-steel", derived_g(E_PA, NU));
    let op = intent("create", "create_material", "Material", "material:library-steel", "materials", "not_present", &canonical_json(&material), "Pa", "stress");
    let (next, _) = applied(&model, &op);
    let created = next["materials"].as_array().unwrap().iter().find(|m| m["id"] == "material:library-steel").unwrap();
    assert_eq!(created, &material, "the inserted material carries E, nu, G, the basis and the record unchanged");
    // A malformed record is the validator's refusal.
    let mut bad = material.clone();
    bad["provenance_record"]["library_ref"]["payload"] = json!({"record":"snapshot"});
    let op = intent("create", "create_material", "Material", "material:library-steel", "materials", "not_present", &canonical_json(&bad), "Pa", "stress");
    assert!(refused(&model, &op).contains(&"MODEL-PROVENANCE-RECORD-INVALID".to_string()));

    let section = json!({"id":"section:library","name":"Invented library pipe","section_type":"pipe",
        "properties":{"outside_diameter":{"value":0.12,"unit":"m"},"wall_thickness":{"value":0.01,"unit":"m"}},
        "provenance":"library:section:invented-lib#invented-record","provenance_record":record("section")});
    let op = intent("create", "create_section", "Section", "section:library", "sections", "not_present", &canonical_json(&section), "m", "length");
    let (next, _) = applied(&model, &op);
    assert_eq!(next["sections"][0], section);
}

#[test]
fn apply_to_material_authors_absent_members_and_refuses_stale_mismatched_or_required_removals() {
    let mut model = pressure_free_fixture();
    // A legacy E/G material receiving an E/nu record: the basis and nu are
    // absent members the apply creates; G is carried as before.
    model["materials"][0] = json!({"id":"material:fixture-isotropic","label":"old","elastic_modulus":{"value":1.9e11,"unit":"Pa"},
        "shear_modulus":{"value":7.3e10,"unit":"Pa"},"thermal_expansion_coefficient":{"value":1.1e-5,"unit":"1/degC"},"provenance":"invented old source"});
    let after = json!({"label":"Invented library steel","constitutive_basis":"homogeneous_isotropic_E_nu_v1",
        "elastic_modulus":{"value":E_PA,"unit":"Pa"},"poisson_ratio":{"value":NU,"unit":"1"},"shear_modulus":{"value":STORED_G_PA,"unit":"Pa"},
        "provenance":"library:material:invented-lib#invented-record","provenance_record":record("material")});
    let op = apply_intent(&model, "Material", "materials", "material:fixture-isotropic", &after);
    let before: Value = serde_json::from_str(op["change"]["before"].as_str().unwrap()).unwrap();
    assert!(before.get("poisson_ratio").is_none() && before["shear_modulus"]["value"] == 7.3e10);
    let (next, rows) = applied(&model, &op);
    let material = &next["materials"][0];
    assert_eq!(material["shear_modulus"]["value"].as_f64().unwrap().to_bits(), STORED_G_PA.to_bits());
    assert_eq!(material["poisson_ratio"], json!({"value":NU,"unit":"1"}));
    assert_eq!(material["thermal_expansion_coefficient"]["value"], 1.1e-5, "an unwritten member is kept");
    assert_eq!(rows.len(), 7, "{rows:?}");

    // Stale before.
    let mut stale = op.clone();
    stale["change"]["before"] = json!(canonical_json(&json!({"elastic_modulus":{"value":1.0e11,"unit":"Pa"},"label":"old","provenance":"invented old source","shear_modulus":{"value":7.3e10,"unit":"Pa"}})));
    assert_eq!(refused(&model, &stale), vec!["OP-STALE-BEFORE-VALUE"]);
    // Dimension mismatch: a length unit on a modulus.
    let op = apply_intent(&model, "Material", "materials", "material:fixture-isotropic", &json!({"elastic_modulus":{"value":2.0,"unit":"m"}}));
    assert_eq!(refused(&model, &op), vec!["OP-LIBRARY-APPLY-VALUE-INVALID"]);
    // E is never removed.
    let op = apply_intent(&model, "Material", "materials", "material:fixture-isotropic", &json!({"elastic_modulus":null}));
    assert_eq!(refused(&model, &op), vec!["OP-LIBRARY-APPLY-REQUIRED-MEMBER"]);
    // nu without the exact basis is not a valid material.
    let op = apply_intent(&model, "Material", "materials", "material:fixture-isotropic", &json!({"poisson_ratio":{"value":NU,"unit":"1"}}));
    assert_eq!(refused(&model, &op), vec!["OP-LIBRARY-APPLY-MATERIAL-INVALID"]);
    // No hash, no apply.
    let op = apply_intent(&model, "Material", "materials", "material:fixture-isotropic", &after);
    assert!(apply_operation(&model, &op, None).applied_model.is_none());
    // Only listed members are writable.
    let op = apply_intent(&model, "Material", "materials", "material:fixture-isotropic", &json!({"density":{"value":7800,"unit":"kg/m^3"}}));
    assert_eq!(refused(&model, &op), vec!["OP-LIBRARY-APPLY-MEMBER-INVALID"]);
}

/// Two pipes bound to one shared section, the second with a mill tolerance.
fn bound_section_model() -> Value {
    let mut model = pressure_free_fixture();
    model["nodes"].as_array_mut().unwrap().push(json!({"id":"node:fixture-far","position":{"x":2,"y":0,"z":0},"provenance":"invented","label":"far"}));
    let mut second = model["pipe_segments"][0].clone();
    second["id"] = json!("pipe:fixture-second");
    second["from"] = json!("node:fixture-tip");
    second["to"] = json!("node:fixture-far");
    second["section"]["mill_tolerance"] = json!({"value":0.005,"unit":"m"});
    model["pipe_segments"].as_array_mut().unwrap().push(second);
    for pipe in model["pipe_segments"].as_array_mut().unwrap() {
        pipe["section_ref"] = json!("section:shared");
    }
    model["sections"] = json!([{"id":"section:shared","name":"Invented shared","section_type":"pipe",
        "properties":{"outside_diameter":{"value":0.12,"unit":"m"},"wall_thickness":{"value":0.01,"unit":"m"}},"provenance":"invented old source"}]);
    // The load at the far node.
    for load in model["load_cases"][1]["primitive_loads"].as_array_mut().unwrap() {
        load["target"]["node"] = json!("node:fixture-far");
    }
    model
}

#[test]
fn section_apply_propagates_to_every_bound_pipe_or_refuses_the_whole_change() {
    let model = bound_section_model();
    let after = json!({"name":"Invented library pipe","properties.outside_diameter":{"value":0.14,"unit":"m"},
        "properties.wall_thickness":{"value":0.012,"unit":"m"},"provenance":"library:section:invented-lib#invented-record","provenance_record":record("section")});
    let op = apply_intent(&model, "Section", "sections", "section:shared", &after);
    let (next, rows) = applied(&model, &op);
    for pipe in next["pipe_segments"].as_array().unwrap() {
        assert_eq!(pipe["section"]["outside_diameter"], json!({"value":0.14,"unit":"m"}));
        assert_eq!(pipe["section"]["wall_thickness"], json!({"value":0.012,"unit":"m"}));
    }
    assert!(rows.contains(&"pipe:fixture-span:section".to_string()) && rows.contains(&"pipe:fixture-second:section".to_string()), "{rows:?}");
    // The solver finds no stale cache.
    let request: LinearStaticPreviewRequest = serde_json::from_value(json!({"model":next,"materials":[]})).unwrap();
    let result = run_linear_static_preview_with_mode(request, PreviewSolverMode::DenseScrutiny);
    assert!(result.diagnostics.iter().all(|d| d.code != "SECTION_REFERENCE_CACHE_STALE"));
    assert_eq!(result.status.mechanics, "MECHANICS_SOLVED", "{:?}", result.diagnostics);

    // A thinner wall that the second pipe's mill tolerance no longer fits: the
    // whole change is refused, and nothing is applied.
    let thin = json!({"properties.outside_diameter":{"value":0.14,"unit":"m"},"properties.wall_thickness":{"value":0.004,"unit":"m"}});
    let op = apply_intent(&model, "Section", "sections", "section:shared", &thin);
    assert_eq!(refused(&model, &op), vec!["OP-SECTION-BINDING-INVALID"]);
    // A wall over half the diameter is not a pipe section.
    let op = apply_intent(&model, "Section", "sections", "section:shared", &json!({"properties.wall_thickness":{"value":0.07,"unit":"m"}}));
    assert_eq!(refused(&model, &op), vec!["OP-SECTION-GEOMETRY-INVALID"]);
}

fn demo_components() -> Value {
    serde_json::from_str(include_str!("../../../../fixtures/product_preview/invented_demo_model.json")).unwrap()
}

#[test]
fn component_apply_checks_family_pair_and_required_members() {
    let model = demo_components();
    // Absent member authored on a rigid component; geometry string rewritten.
    let after = json!({"geometry.weight":{"value":1500,"unit":"N"},"geometry.rigid_component_source_reference":"prev; library:component:invented-lib#invented-record: weight",
        "provenance":"library:component:invented-lib#invented-record","provenance_record":record("component")});
    let op = apply_intent(&model, "Component", "components", "component:C-130", &after);
    let (next, _) = applied(&model, &op);
    let valve = next["components"].as_array().unwrap().iter().find(|c| c["id"] == "component:C-130").unwrap();
    assert_eq!(valve["geometry"]["weight"], json!({"value":1500,"unit":"N"}));
    assert!(valve["geometry"]["center_of_gravity"].is_object(), "unwritten members are kept");

    // Kind mismatch: bend geometry on a valve.
    let op = apply_intent(&model, "Component", "components", "component:C-130", &json!({"geometry.bend_radius":{"value":0.5,"unit":"m"}}));
    assert_eq!(refused(&model, &op), vec!["OP-LIBRARY-APPLY-KIND-MISMATCH"]);
    // Required members are never removed.
    let op = apply_intent(&model, "Component", "components", "component:C-110", &json!({"modifiers.flexibility_factor_user_value":null}));
    assert_eq!(refused(&model, &op), vec!["OP-LIBRARY-APPLY-REQUIRED-MEMBER"]);
    let op = apply_intent(&model, "Component", "components", "component:C-120", &json!({"modifiers.flexibility_factor_user_value":null}));
    assert_eq!(refused(&model, &op), vec!["OP-LIBRARY-APPLY-REQUIRED-MEMBER"]);
    // Placement references are not writable.
    let op = apply_intent(&model, "Component", "components", "component:C-130", &json!({"geometry.rigid_pipe_ref":null}));
    assert_eq!(refused(&model, &op), vec!["OP-LIBRARY-APPLY-MEMBER-INVALID"]);
    // The directional SIF pair needs a realized bend, and travels together.
    let pair = json!({"modifiers.sif_in_plane_user_value":{"value":1.2,"unit":"1"},"modifiers.sif_out_of_plane_user_value":{"value":1.1,"unit":"1"}});
    let op = apply_intent(&model, "Component", "components", "component:C-110", &pair);
    assert_eq!(refused(&model, &op), vec!["OP-LIBRARY-APPLY-SIF-PAIR"]);
    let op = apply_intent(&model, "Component", "components", "component:C-110", &json!({"modifiers.sif_in_plane_user_value":{"value":1.2,"unit":"1"}}));
    assert_eq!(refused(&model, &op), vec!["OP-LIBRARY-APPLY-SIF-PAIR"]);
    // An agent never writes the bend flexibility factor.
    let mut op = apply_intent(&model, "Component", "components", "component:C-110", &json!({"modifiers.flexibility_factor_user_value":{"value":1.2,"unit":"none"}}));
    op["author_type"] = json!("agent");
    assert!(refused(&model, &op).contains(&"OP-BEND-FLEXIBILITY-USER-ENTRY-REQUIRED".to_string()));
    // Applying the same values again changes nothing and is refused as such.
    let op = apply_intent(&next, "Component", "components", "component:C-130", &after);
    assert_eq!(refused(&next, &op), vec!["OP-LIBRARY-APPLY-UNCHANGED"]);
}

#[test]
fn inserted_e_nu_g_material_solves_with_the_route_specific_g() {
    let g_derived = derived_g(E_PA, NU);
    // Independent check of the derivation's arithmetic: 2e11 / 2.6.
    assert!((g_derived - 76_923_076_923.076_92).abs() < 1e-3);

    // Exact route: the solver derives G from the selected E/nu pair and
    // ignores a retained G, with a warning (pressure_material.rs).
    let exact = exact_fixture();
    let (stored_exact, codes) = tip_rotation_x(&insert_and_use(&exact, &inserted_material("material:library-steel", STORED_G_PA)));
    close(stored_exact, torsion_rotation(g_derived));
    assert!(codes.contains(&"EXACT_PRESSURE_REDUNDANT_G_IGNORED".to_string()), "{codes:?}");
    let (derived_exact, codes) = tip_rotation_x(&insert_and_use(&exact, &inserted_material("material:library-steel", g_derived)));
    close(derived_exact, torsion_rotation(g_derived));
    assert!(codes.contains(&"EXACT_PRESSURE_REDUNDANT_G_IGNORED".to_string()), "a derived G is retained too, and also ignored");

    // Pressure-free route: the stored G is the stiffness G; nu and the basis
    // are carried and unused for torsion.
    let free = pressure_free_fixture();
    let (stored_free, codes) = tip_rotation_x(&insert_and_use(&free, &inserted_material("material:library-steel", STORED_G_PA)));
    close(stored_free, torsion_rotation(STORED_G_PA));
    assert!(!codes.contains(&"EXACT_PRESSURE_REDUNDANT_G_IGNORED".to_string()));
    let (derived_free, _) = tip_rotation_x(&insert_and_use(&free, &inserted_material("material:library-steel", g_derived)));
    close(derived_free, torsion_rotation(g_derived));
    // With a derived G the two routes agree.
    assert!((derived_free - derived_exact).abs() <= derived_exact.abs() * 1e-12);
}

/// A verbatim port of the dyadic `Scaled` arithmetic of
/// `core/product_physics/src/pressure_exact.rs` (private to that crate), used
/// to state the exact route's G derivation independently of the applier.
mod solver_scaled {
    #[derive(Clone, Copy)]
    pub struct Scaled { mantissa: f64, exponent: i32 }
    impl Scaled {
        pub fn from_f64(value: f64) -> Self {
            if value == 0.0 { return Self { mantissa: value, exponent: 0 }; }
            let bits = value.to_bits();
            let sign = bits & (1u64 << 63);
            let biased = ((bits >> 52) & 0x7ff) as i32;
            let fraction = bits & ((1u64 << 52) - 1);
            if biased != 0 {
                Self { mantissa: f64::from_bits(sign | (1022u64 << 52) | fraction), exponent: biased - 1022 }
            } else {
                let highest_bit = 63 - fraction.leading_zeros() as i32;
                let magnitude = (fraction as f64) / power_of_two(highest_bit + 1);
                Self { mantissa: magnitude.copysign(value), exponent: highest_bit + 1 - 1074 }
            }
        }
        fn from_parts(mantissa: f64, exponent: i32) -> Self {
            let normalized = Self::from_f64(mantissa);
            if normalized.mantissa == 0.0 { normalized } else { Self { mantissa: normalized.mantissa, exponent: normalized.exponent + exponent } }
        }
        pub fn add(self, other: Self) -> Self {
            if self.mantissa == 0.0 { return other; }
            if other.mantissa == 0.0 { return self; }
            let exponent = self.exponent.max(other.exponent);
            let left = align_mantissa(self.mantissa, self.exponent - exponent);
            let right = align_mantissa(other.mantissa, other.exponent - exponent);
            Self::from_parts(left + right, exponent)
        }
        pub fn mul(self, other: Self) -> Self {
            if self.mantissa == 0.0 || other.mantissa == 0.0 { return Self::from_f64(self.mantissa * other.mantissa); }
            Self::from_parts(self.mantissa * other.mantissa, self.exponent + other.exponent)
        }
        pub fn div(self, other: Self) -> Option<Self> {
            if other.mantissa == 0.0 { return None; }
            if self.mantissa == 0.0 { return Some(Self::from_f64(self.mantissa / other.mantissa)); }
            Some(Self::from_parts(self.mantissa / other.mantissa, self.exponent - other.exponent))
        }
        pub fn to_f64(self) -> f64 {
            if self.mantissa == 0.0 { return self.mantissa; }
            if self.exponent > 1024 { return f64::INFINITY.copysign(self.mantissa); }
            if self.exponent == 1024 { return (self.mantissa * power_of_two(1023)) * 2.0; }
            if self.exponent < -1074 { return 0.0f64.copysign(self.mantissa); }
            self.mantissa * power_of_two(self.exponent)
        }
    }
    fn align_mantissa(mantissa: f64, exponent_shift: i32) -> f64 {
        if exponent_shift < -1074 { 0.0f64.copysign(mantissa) } else { mantissa * power_of_two(exponent_shift) }
    }
    fn power_of_two(exponent: i32) -> f64 {
        if exponent >= -1022 { f64::from_bits(((exponent + 1023) as u64) << 52) } else { f64::from_bits(1u64 << (exponent + 1074)) }
    }
    /// `IsotropicENu::new(e_pa, nu)`'s G: (2 × (1 + ν)), then E / that.
    pub fn exact_route_g(e_pa: f64, nu: f64) -> Option<f64> {
        if !e_pa.is_finite() || !nu.is_finite() || e_pa <= 0.0 || nu <= -1.0 || nu >= 0.5 { return None; }
        let denominator = Scaled::from_f64(2.0).mul(Scaled::from_f64(1.0).add(Scaled::from_f64(nu)));
        let g = Scaled::from_f64(e_pa).div(denominator)?.to_f64();
        (g.is_finite() && g > 0.0).then_some(g)
    }
}

#[test]
fn derived_g_is_bit_identical_to_the_exact_route_derivation_across_a_sweep() {
    use open_pipe_stress_operation_applier::derived_shear_modulus_pa;
    use open_pipe_stress_units::{canonical_unit, convert_for_dimension, unit_by_symbol, Dimension};
    let moduli = [2.0e11, 2.07e11, 1.9999999999999998e11, 193e9, 7.5e10, 1.0, 3.0e-300, 1.0e300, 1.7976931348623157e308, 123456789.123456789, 0.1 + 0.2];
    let ratios = [0.3, 0.1 + 0.2, 0.29, 1.0 / 3.0, 0.49999999999999994, 0.0, -0.0, 1e-17, -1e-17, 5e-324, -0.5, -0.9999999999999999, 0.2928, 0.27];
    let units = ["Pa", "kPa", "MPa", "GPa", "psi", "ksi"];
    let mut compared = 0;
    for unit in units {
        for &e in &moduli {
            for &nu in &ratios {
                // The solver's normalize_quantity path for E.
                let from = unit_by_symbol(unit, Dimension::Stress).unwrap();
                let Ok(e_pa) = convert_for_dimension(e, Dimension::Stress, from, canonical_unit(Dimension::Stress).unwrap()) else { continue };
                let reference = solver_scaled::exact_route_g(e_pa, nu);
                match derived_shear_modulus_pa(e, unit, nu) {
                    Ok(g) => {
                        assert_eq!(Some(g.to_bits()), reference.map(f64::to_bits), "E={e} {unit}, nu={nu}");
                        compared += 1;
                    }
                    // Refused only where the exact route has no normal positive G either.
                    Err(_) => assert!(reference.is_none_or(|g| !g.is_normal()), "E={e} {unit}, nu={nu}: {reference:?}"),
                }
            }
        }
    }
    assert!(compared > 300, "{compared}");
}

/// Every `G_pa` the solve result publishes for `pipe_id` (the exact route's
/// per-member material evidence: the G that built stiffness).
fn published_g(value: &Value, pipe_id: &str, found: &mut Vec<f64>) {
    match value {
        Value::Object(map) => {
            if map.get("pipe_id").and_then(Value::as_str) == Some(pipe_id) {
                if let Some(g) = map.get("G_pa").and_then(Value::as_f64) {
                    found.push(g);
                }
            }
            map.values().for_each(|child| published_g(child, pipe_id, found));
        }
        Value::Array(items) => items.iter().for_each(|child| published_g(child, pipe_id, found)),
        _ => {}
    }
}

#[test]
fn desktop_derived_g_equals_the_g_the_exact_route_builds_stiffness_with_bit_for_bit() {
    use open_pipe_stress_operation_applier::derived_shear_modulus_pa;
    // Public product_physics API: the exact route publishes, per member, the
    // G that built its stiffness (`pipe_materials[].G_pa`).
    for (e, unit, nu) in [(207.0, "GPa", 0.29), (2.0e11, "Pa", 0.3), (193000.0000001, "MPa", 0.1 + 0.2), (29000.0, "ksi", 1.0 / 3.0), (69.0, "GPa", -0.9999999999999999)] {
        let g = derived_shear_modulus_pa(e, unit, nu).unwrap();
        let material = json!({"id":"material:library-steel","label":"Invented library steel","constitutive_basis":"homogeneous_isotropic_E_nu_v1",
            "elastic_modulus":{"value":e,"unit":unit},"poisson_ratio":{"value":nu,"unit":"1"},"shear_modulus":{"value":g,"unit":"Pa"},
            "provenance":"library:material:invented-lib#invented-record","provenance_record":record("material")});
        let model = insert_and_use(&exact_fixture(), &material);
        let request: LinearStaticPreviewRequest = serde_json::from_value(json!({"model":model,"materials":[]})).unwrap();
        let result = serde_json::to_value(run_linear_static_preview_with_mode(request, PreviewSolverMode::DenseScrutiny)).unwrap();
        let mut found = Vec::new();
        published_g(&result, "pipe:fixture-span", &mut found);
        assert!(!found.is_empty(), "the exact route publishes the member's G");
        for published in found {
            assert_eq!(published.to_bits(), g.to_bits(), "E={e} {unit}, nu={nu}");
        }
    }
}

#[test]
fn emission_golden_library_version_is_the_records_rfc8785_sha256() {
    // The record the desktop emission golden inserts (desktopEmission.golden.test.ts),
    // restated here: the digest the webview computed must equal the native one.
    let provenance = json!({"source_name":"Invented emission handbook","source_location":"invented://emission/handbook","source_license":"invented-permissive",
        "contributor":"Invented contributor","contributor_certification":"invented-attestation","redistribution_status":"public_permissive","review_status":"pending"});
    let property = |kind: &str, magnitude: f64, unit_id: &str| json!({"property_id":format!("mat.invented.emission.{kind}"),"property_kind":kind,
        "value_status":"provided","required_for":"mechanics_solve","provenance":provenance,"review_status":"pending",
        "value":{"magnitude":magnitude,"unit_ref":{"ref_type":"UnitDefinition","ref_id":unit_id},"dimension_id":"stress","quantity_kind":"unit_bearing",
            "unit_required":true,"missing_unit_behavior":"diagnostic_blocking","provenance":provenance}});
    let record = json!({"material_id":"mat.invented.emission","name":"Invented emission steel","material_family":"invented_example","privacy_class":"public_permissive",
        "redistribution_status":"public_permissive","properties":[property("elastic_modulus",2.0e11,"unit:pascal"),property("poisson_ratio",0.3,"unit:one")],
        "allowables":[],"completeness":[],"provenance":provenance,"review_status":"pending"});
    let golden: Value = serde_json::from_str(include_str!("../../../../fixtures/model_document/desktop_emitted.json")).unwrap();
    let entry = golden["entries"].as_array().unwrap().iter().find(|e| e["path"] == "library_material_insert").unwrap();
    let material = entry["document"]["materials"].as_array().unwrap().iter().find(|m| m["id"] == "material:library-emission").unwrap();
    assert_eq!(
        material["provenance_record"]["library_ref"]["library_version"],
        json!(format!("sha256:{}", sha256_hex(&canonical_json(&record))))
    );
    // The golden's derived G is the native derivation's, bit for bit.
    assert_eq!(material["shear_modulus"]["value"].as_f64().unwrap().to_bits(),
        open_pipe_stress_operation_applier::derived_shear_modulus_pa(2.0e11, "Pa", 0.3).unwrap().to_bits());
}

/// HELP_HUMAN, 2026-10-23: "Each refusal must tell the user how to proceed".
/// Each applier-side library-apply refusal names its cause and the way
/// forward; the texts are pinned here.
fn refusal_messages() -> Vec<(String, String)> {
    let message = |model: &Value, op: &Value| -> (String, String) {
        let outcome = apply_operation(model, op, Some(&claim(model)));
        assert!(outcome.applied_model.is_none());
        let d = outcome.diagnostics.iter().find(|d| d.severity == "blocking").unwrap();
        (d.code.clone(), d.message.clone())
    };
    let mut out = Vec::new();
    let mut material_model = pressure_free_fixture();
    material_model["materials"][0] = json!({"id":"material:fixture-isotropic","label":"old","elastic_modulus":{"value":1.9e11,"unit":"Pa"},
        "shear_modulus":{"value":7.3e10,"unit":"Pa"},"provenance":"invented old source"});
    let id = "material:fixture-isotropic";
    out.push(message(&material_model, &apply_intent(&material_model, "Material", "materials", id, &json!({"density":{"value":7800,"unit":"kg/m^3"}}))));
    out.push(message(&material_model, &apply_intent(&material_model, "Material", "materials", id, &json!({"elastic_modulus":{"value":2.0,"unit":"m"}}))));
    out.push(message(&material_model, &apply_intent(&material_model, "Material", "materials", id, &json!({"elastic_modulus":null}))));
    out.push(message(&material_model, &apply_intent(&material_model, "Material", "materials", id, &json!({"poisson_ratio":{"value":0.3,"unit":"1"}}))));
    let mut stale = apply_intent(&material_model, "Material", "materials", id, &json!({"label":"new"}));
    stale["change"]["before"] = json!("{\"label\":\"older\"}");
    out.push(message(&material_model, &stale));
    out.push(message(&material_model, &apply_intent(&material_model, "Material", "materials", id, &json!({"label":"old"}))));
    let sections = bound_section_model();
    out.push(message(&sections, &apply_intent(&sections, "Section", "sections", "section:shared", &json!({"properties.outside_diameter":{"value":0.14,"unit":"m"},"properties.wall_thickness":{"value":0.004,"unit":"m"}}))));
    out.push(message(&sections, &apply_intent(&sections, "Section", "sections", "section:shared", &json!({"properties.wall_thickness":{"value":0.07,"unit":"m"}}))));
    let demo = demo_components();
    out.push(message(&demo, &apply_intent(&demo, "Component", "components", "component:C-130", &json!({"geometry.bend_radius":{"value":0.5,"unit":"m"}}))));
    out.push(message(&demo, &apply_intent(&demo, "Component", "components", "component:C-110", &json!({"modifiers.sif_in_plane_user_value":{"value":1.2,"unit":"1"}}))));
    let mut absent = apply_intent(&material_model, "Material", "materials", id, &json!({"label":"new"}));
    absent["target"]["ref"] = json!("material:absent");
    out.push(message(&material_model, &absent));
    let mut shape = apply_intent(&material_model, "Material", "materials", id, &json!({"label":"new"}));
    shape["change"]["field_path"] = json!("label");
    out.push(message(&material_model, &shape));
    let mut payload = apply_intent(&material_model, "Material", "materials", id, &json!({"label":"new"}));
    payload["change"]["after"] = json!("[]");
    out.push(message(&material_model, &payload));
    out
}

#[test]
fn every_applier_library_refusal_names_its_cause_and_the_way_forward() {
    let expected = [
        ("OP-LIBRARY-APPLY-MEMBER-INVALID", "`density` is not a member a library record can write on a Material. Prepare the apply again from the Library Manager, which builds this change."),
        ("OP-LIBRARY-APPLY-VALUE-INVALID", "`elastic_modulus` unit `m` is not an accepted DEC-018 stress unit; nothing is converted. Correct the value or its unit in the library, re-import it, and prepare the apply again."),
        ("OP-LIBRARY-APPLY-REQUIRED-MEMBER", "`elastic_modulus` is required for this Material and is never removed by a library apply. Keep the member (choose \"keep them\"), or apply a record that supplies it."),
        ("OP-LIBRARY-APPLY-MATERIAL-INVALID", "The applied material would be invalid: E/nu authoring requires homogeneous_isotropic_E_nu_v1. Choose the other material mode, or a record with a complete E/ν or E/G set, and prepare the apply again."),
        ("OP-STALE-BEFORE-VALUE", "The change's before no longer matches the current members of `material:fixture-isotropic` ({\"label\":\"old\"}). The model changed after the apply was prepared; prepare it again."),
        ("OP-LIBRARY-APPLY-UNCHANGED", "Every member already holds the library value; `material:fixture-isotropic` is unchanged. There is nothing to apply; choose a different record or leave the entity as it is."),
        ("OP-SECTION-BINDING-INVALID", "Mill tolerance must be nonnegative and leave a positive effective wall. Adjust the bound pipes' mill tolerance or corrosion allowance, or detach them from the section, then apply again."),
        ("OP-SECTION-GEOMETRY-INVALID", "Section `section:shared` requires type pipe and accepted length quantities with 0 < wall < outside diameter / 2. Choose a record whose wall thickness is less than half its outside diameter."),
        ("OP-LIBRARY-APPLY-KIND-MISMATCH", "`geometry.bend_radius` belongs to the bend/elbow families; component `component:C-130` is a `valve`. Choose a record for this component's kind, or select a component of the record's kind."),
        ("OP-LIBRARY-APPLY-SIF-PAIR", "The in-plane and out-of-plane SIF values are written together or not at all. Supply both directional SIF values, and apply them only to a bend realized as the curved-bend macro element."),
        ("OP-TARGET-NOT-FOUND", "Material `material:absent` was not found in the current model. Select an entity that is in the model and prepare the apply again."),
        ("OP-LIBRARY-APPLY-SHAPE-INVALID", "`apply_library_record` carries field_path `library_record`, unit none and dimension dimensionless. Prepare the apply again from the Library Manager, which builds this change."),
        ("OP-LIBRARY-APPLY-PAYLOAD-INVALID", "The change's after must be a non-empty JSON object of member paths. Prepare the apply again from the Library Manager, which builds this change."),
    ];
    let actual = refusal_messages();
    assert_eq!(actual.len(), expected.len());
    for ((code, message), (expected_code, expected_message)) in actual.iter().zip(expected) {
        assert_eq!(code, expected_code);
        assert_eq!(message, expected_message);
    }
}
