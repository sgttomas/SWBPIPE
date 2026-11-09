"""Build b1.diff: B1 of the curved-bend transverse-shear option (DEL-04-01 curved-bend-transverse-shear.md §5).

Read-only on the base: files come from `git show 3c9db2295:<path>`; copies and the diff live beside this
script. Each `old` must occur exactly once in its file. New files are emitted as new-file hunks.
"""
import hashlib
import json
import subprocess
import sys
from pathlib import Path

REPO = Path("/Users/ryan/dev/chirality-t3/bend-shear-b1-base")
BASE = "3c9db2295"
HERE = Path(__file__).resolve().parent
A, B = HERE / "a", HERE / "b"

V1 = "timoshenko_annulus_energy_v1"
V2 = "timoshenko_annulus_energy_curved_v2"
PF_ID = "openpipestress.result_semantics/0.3.0/preview-physics-curved-shear-1"
PF_PROFILE = "product_preview_mechanics_curved_shear_v1"
TIM_ID = "openpipestress.result_semantics/0.3.0/preview-physics-timoshenko-1"
TIM_PROFILE = "product_preview_mechanics_timoshenko_v1"
TIM_SHA = "8a0abaeac93729b5d6ee6c922f2a60f0caf9332948622bb8b5896747ce41442d"
PF_LIM0 = (
    "Small-displacement, linear-elastic frame preview: straight members, including geometry-only bends, and realized "
    "arcs are Timoshenko with the energy-matched annular shear factor (timoshenko_annulus_energy_curved_v2) and "
    "Poisson's ratio E/(2G) - 1; on a realized arc the straight-annulus factor is applied along the centreline with "
    "no curvature correction, the arc's thermal equivalent load is formed with that shear-dependent arc stiffness, "
    "as the solve is, and the arc's stress recovery stays the straight-annulus recovery; a straight member's "
    "published nodal rotations are section rotations, not slopes; numerical integrity does not establish physical "
    "correctness.")
PF_TABLE = "fixtures/results/semantic_contract_v0_3_preview_physics_curved_shear_1.json"


def show(path):
    return subprocess.run(["git", "-C", str(REPO), "show", f"{BASE}:{path}"], check=True,
                          capture_output=True).stdout.decode("utf-8")


def dumps(value):
    """semantic_contract_tables.dumps: two-space indent, ASCII escapes, final newline."""
    return json.dumps(value, indent=2, ensure_ascii=True) + "\n"


# ---------------------------------------------------------------- the derived PF table (semantic_contract_tables.py derive)
_preview = json.loads(show("fixtures/results/semantic_contract_v0_3_preview_physics_2.json"))
_tim = json.loads(show("fixtures/results/semantic_contract_v0_3_preview_physics_timoshenko_1.json"))
# The derivation reproduces the committed Timoshenko table byte for byte before it is trusted for the curved one.
_check = dict(_preview, semantic_contract_id=TIM_ID, formulation_profile_id=TIM_PROFILE)
_check["supported_profile_limitations"] = [_tim["supported_profile_limitations"][0]] + _preview["supported_profile_limitations"][1:]
assert dumps(_check) == show("fixtures/results/semantic_contract_v0_3_preview_physics_timoshenko_1.json"), "derivation"
assert hashlib.sha256(dumps(_check).encode()).hexdigest() == TIM_SHA
_curved = dict(_preview, semantic_contract_id=PF_ID, formulation_profile_id=PF_PROFILE)
_curved["supported_profile_limitations"] = [PF_LIM0] + _preview["supported_profile_limitations"][1:]
PF_TABLE_TEXT = dumps(_curved)
PF_SHA = hashlib.sha256(PF_TABLE_TEXT.encode("ascii")).hexdigest()
NEW_FILES = {PF_TABLE: PF_TABLE_TEXT}

E = []  # (path, old, new)


def edit(path, old, new):
    E.append((path, old, new))


# ================================================================ product physics
P = "core/product_physics/src/model_settings.rs"
edit(P, """//! pressure-free route (DEL-05-03 Design `pf-timoshenko.md`; `crate::pf_timoshenko`),
//! which removed 0.8.0's interim refusal at solve and at authoring.
""", """//! pressure-free route (DEL-05-03 Design `pf-timoshenko.md`; `crate::pf_timoshenko`),
//! which removed 0.8.0's interim refusal at solve and at authoring.
//! `timoshenko_annulus_energy_curved_v2` (DEL-04-01 Design `curved-bend-transverse-shear.md`
//! §1, §5) forms the straight members exactly as v1 does and adds transverse shear to every
//! realized arc. It is never inferred: v1 keeps its meaning, and only an explicit set selects v2.
""")
edit(P, """/// `timoshenko_annulus_energy_v1`: Timoshenko straight members on a pressure-free document.
pub const SHEAR_MODEL_TIMOSHENKO: &str = "timoshenko_annulus_energy_v1";
/// The value is not one of the two strings (a non-string and `null` included).
""", """/// `timoshenko_annulus_energy_v1`: Timoshenko straight members on a pressure-free document.
pub const SHEAR_MODEL_TIMOSHENKO: &str = "timoshenko_annulus_energy_v1";
/// `timoshenko_annulus_energy_curved_v2`: v1's straight members, and realized arcs with transverse shear.
pub const SHEAR_MODEL_TIMOSHENKO_CURVED: &str = "timoshenko_annulus_energy_curved_v2";
/// The value is not one of the three strings (a non-string and `null` included).
""")
edit(P, """/// The authored `model_settings.shear_model`, as decoded: one of the two names, or any other
/// JSON value (a non-string and `null` included), which is refused by name rather than at decode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShearModelSetting {
    EulerBernoulli,
    TimoshenkoAnnulusEnergyV1,
    Unknown,
}
""", """/// The authored `model_settings.shear_model`, as decoded: one of the three names, or any other
/// JSON value (a non-string and `null` included), which is refused by name rather than at decode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShearModelSetting {
    EulerBernoulli,
    TimoshenkoAnnulusEnergyV1,
    TimoshenkoAnnulusEnergyCurvedV2,
    Unknown,
}
""")
edit(P, """            Some(SHEAR_MODEL_TIMOSHENKO) => ShearModelSetting::TimoshenkoAnnulusEnergyV1,
            _ => ShearModelSetting::Unknown,
        }
    }
}
""", """            Some(SHEAR_MODEL_TIMOSHENKO) => ShearModelSetting::TimoshenkoAnnulusEnergyV1,
            Some(SHEAR_MODEL_TIMOSHENKO_CURVED) => ShearModelSetting::TimoshenkoAnnulusEnergyCurvedV2,
            _ => ShearModelSetting::Unknown,
        }
    }
}
""")
edit(P, """                SHEAR_MODEL_TIMOSHENKO => ShearModelSetting::TimoshenkoAnnulusEnergyV1,
                _ => ShearModelSetting::Unknown,
""", """                SHEAR_MODEL_TIMOSHENKO => ShearModelSetting::TimoshenkoAnnulusEnergyV1,
                SHEAR_MODEL_TIMOSHENKO_CURVED => ShearModelSetting::TimoshenkoAnnulusEnergyCurvedV2,
                _ => ShearModelSetting::Unknown,
""")
edit(P, """/// The refusal message for a value that is not one of the two names.
pub fn shear_model_unknown_message() -> String {
    format!("{SHEAR_MODEL_PATH} must be `{SHEAR_MODEL_EULER_BERNOULLI}` or `{SHEAR_MODEL_TIMOSHENKO}`.")
}
""", """/// The refusal message for a value that is not one of the three names.
pub fn shear_model_unknown_message() -> String {
    format!("{SHEAR_MODEL_PATH} must be `{SHEAR_MODEL_EULER_BERNOULLI}`, `{SHEAR_MODEL_TIMOSHENKO}` or `{SHEAR_MODEL_TIMOSHENKO_CURVED}`.")
}
""")
edit(P, """/// validator alike: a value other than the two names, then the member on a document
/// that declares `pressure_contract`. Both names are admitted on a pressure-free document.
""", """/// validator alike: a value other than the three names, then the member on a document
/// that declares `pressure_contract`. Every name is admitted on a pressure-free document.
""")
edit(P, """    /// Euler–Bernoulli. An unknown value reads as Euler–Bernoulli here and is refused by
    /// [`shear_model_refusal`]; `crate::pf_timoshenko::selects` consumes the choice.
    pub fn shear_model(&self) -> ShearModelSetting {
        match self.model_settings.as_ref().and_then(|settings| settings.shear_model) {
            Some(ShearModelSetting::TimoshenkoAnnulusEnergyV1) => ShearModelSetting::TimoshenkoAnnulusEnergyV1,
            _ => ShearModelSetting::EulerBernoulli,
        }
    }
""", """    /// Euler–Bernoulli. An unknown value reads as Euler–Bernoulli here and is refused by
    /// [`shear_model_refusal`]; `crate::pf_timoshenko::selects` and `selects_curved` consume the choice.
    pub fn shear_model(&self) -> ShearModelSetting {
        match self.model_settings.as_ref().and_then(|settings| settings.shear_model) {
            Some(ShearModelSetting::TimoshenkoAnnulusEnergyV1) => ShearModelSetting::TimoshenkoAnnulusEnergyV1,
            Some(ShearModelSetting::TimoshenkoAnnulusEnergyCurvedV2) => ShearModelSetting::TimoshenkoAnnulusEnergyCurvedV2,
            _ => ShearModelSetting::EulerBernoulli,
        }
    }
""")

P = "core/product_physics/src/pf_timoshenko.rs"
edit(P, """//! Realized arcs stay
//! Euler–Bernoulli and are disclosed by name (§4).
""", """//! Realized arcs stay
//! Euler–Bernoulli and are disclosed by name (§4).
//!
//! `timoshenko_annulus_energy_curved_v2` (DEL-04-01 Design `curved-bend-transverse-shear.md`
//! §1, §5) forms the same straight members bit for bit, and each realized arc on a consumed
//! basis also takes A_s from this module's [`shear_area`] (the arc's own PF annulus, area and
//! unscaled moduli); its result has its own identity, profile and limitation 0, and no arc
//! disclosure.
""")
edit(P, """/// §4: the named disclosure of the realized arcs that stay Euler–Bernoulli.
pub(crate) const REALIZED_ARCS_EULER_BERNOULLI: &str = "PF_TIMOSHENKO_REALIZED_ARCS_EULER_BERNOULLI";
""", f"""/// §4: the named disclosure of the realized arcs that stay Euler–Bernoulli.
pub(crate) const REALIZED_ARCS_EULER_BERNOULLI: &str = "PF_TIMOSHENKO_REALIZED_ARCS_EULER_BERNOULLI";

/// DEL-04-01 `curved-bend-transverse-shear.md` §5: the curved-shear PF result's semantic contract identity.
pub(crate) const CURVED_ID: &str = "{PF_ID}";
/// The curved-shear PF result's profile.
pub(crate) const CURVED_PROFILE_ID: &str = "{PF_PROFILE}";
/// Limitation 0 of the curved-shear PF identity; the other limitations are preview-physics-2's.
pub(crate) const CURVED_LIMITATION_0: &str = "{PF_LIM0}";
""")
edit(P, """/// Whether `model` is a pressure-free document selecting Timoshenko straight members.
pub(crate) fn selects(model: &PreviewModel) -> bool {
    model.pressure_contract.is_none() && model.shear_model() == ShearModelSetting::TimoshenkoAnnulusEnergyV1
}

/// The producer's semantic contract identity on the preview route.
pub(crate) fn semantic_contract_id(model: &PreviewModel) -> &'static str {
    if selects(model) { ID } else { preview_physics::ID }
}
""", """/// Whether `model` is a pressure-free document selecting Timoshenko straight members (v1 or v2;
/// both form them identically).
pub(crate) fn selects(model: &PreviewModel) -> bool {
    model.pressure_contract.is_none()
        && matches!(
            model.shear_model(),
            ShearModelSetting::TimoshenkoAnnulusEnergyV1 | ShearModelSetting::TimoshenkoAnnulusEnergyCurvedV2
        )
}

/// Whether `model` is a pressure-free document selecting `timoshenko_annulus_energy_curved_v2`:
/// realized arcs with transverse shear as well.
pub(crate) fn selects_curved(model: &PreviewModel) -> bool {
    model.pressure_contract.is_none() && model.shear_model() == ShearModelSetting::TimoshenkoAnnulusEnergyCurvedV2
}

/// The selected Timoshenko model's document name, as the diagnostics state it.
fn shear_model_name(model: &PreviewModel) -> &'static str {
    if selects_curved(model) {
        crate::model_settings::SHEAR_MODEL_TIMOSHENKO_CURVED
    } else {
        crate::model_settings::SHEAR_MODEL_TIMOSHENKO
    }
}

/// The producer's semantic contract identity on the preview route.
pub(crate) fn semantic_contract_id(model: &PreviewModel) -> &'static str {
    if selects_curved(model) {
        CURVED_ID
    } else if selects(model) {
        ID
    } else {
        preview_physics::ID
    }
}
""")
edit(P, """    let mut basis = preview_physics::formulation_basis();
    if selects(model) {
        basis.profile_id = PROFILE_ID.to_string();
        basis.limitations[0] = LIMITATION_0.to_string();
    }
    basis
""", """    let mut basis = preview_physics::formulation_basis();
    if selects_curved(model) {
        basis.profile_id = CURVED_PROFILE_ID.to_string();
        basis.limitations[0] = CURVED_LIMITATION_0.to_string();
    } else if selects(model) {
        basis.profile_id = PROFILE_ID.to_string();
        basis.limitations[0] = LIMITATION_0.to_string();
    }
    basis
""")
edit(P, """pub(crate) struct ConsumedBasis {
    pub(crate) description: String,
    pub(crate) case_ids: Vec<String>,
}
""", """pub(crate) struct ConsumedBasis {
    pub(crate) description: String,
    pub(crate) case_ids: Vec<String>,
    /// The selected Timoshenko model's name (v1 or v2), as the diagnostics state it.
    pub(crate) shear_model: &'static str,
}
""")
edit(P, """                        bases.selected.push((key, ConsumedBasis { description, case_ids: Vec::new() }));
""", """                        bases.selected.push((key, ConsumedBasis { description, case_ids: Vec::new(), shear_model: shear_model_name(model) }));
""")
edit(P, """                    description: "the base material basis (no temperature selection)".to_string(),
                    case_ids: Vec::new(),
                }),
""", """                    description: "the base material basis (no temperature selection)".to_string(),
                    case_ids: Vec::new(),
                    shear_model: shear_model_name(model),
                }),
""")
edit(P, """/// §2: one straight member's shear area on a consumed basis, from PF's stiffness annulus
""", """/// §2: one straight member's shear area on a consumed basis (under v2, also one realized
/// arc's, by the same rule; `curved-bend-transverse-shear.md` §5), from PF's stiffness annulus
""")
edit(P, """                format!(
                    "timoshenko_annulus_energy_v1 member {} has no energy-matched annular shear factor on {} (load cases {cases}): nu = E/(2G) - 1 = {nu:?} from material {} E = {elastic_modulus:?} Pa and G = {shear_modulus:?} Pa, r_i = {inner_radius:?} m, r_o = {outer_radius:?} m: {error}; no default factor or Euler-Bernoulli member is used",
                    pipe.id, basis.description, material.id,
                ),
""", """                format!(
                    "{} member {} has no energy-matched annular shear factor on {} (load cases {cases}): nu = E/(2G) - 1 = {nu:?} from material {} E = {elastic_modulus:?} Pa and G = {shear_modulus:?} Pa, r_i = {inner_radius:?} m, r_o = {outer_radius:?} m: {error}; no default factor or Euler-Bernoulli member is used",
                    basis.shear_model, pipe.id, basis.description, material.id,
                ),
""")
edit(P, """                format!(
                    "material {} authors poisson_ratio {authored:?}, which is not used: its timoshenko_annulus_energy_v1 straight members on {} (load cases {cases}) take nu = E/(2G) - 1 = {nu:?} from E = {elastic_modulus:?} Pa and G = {shear_modulus:?} Pa",
                    material.id, basis.description,
                ),
""", """                format!(
                    "material {} authors poisson_ratio {authored:?}, which is not used: its {} on {} (load cases {cases}) take nu = E/(2G) - 1 = {nu:?} from E = {elastic_modulus:?} Pa and G = {shear_modulus:?} Pa",
                    material.id,
                    if basis.shear_model == crate::model_settings::SHEAR_MODEL_TIMOSHENKO_CURVED {
                        "timoshenko_annulus_energy_curved_v2 straight members and realized arcs"
                    } else {
                        "timoshenko_annulus_energy_v1 straight members"
                    },
                    basis.description,
                ),
""")
edit(P, """                format!(
                    "load case {} equivalent_static.wind.exposed_spans[{index}] is a partial-span member load on timoshenko_annulus_energy_v1 straight member {pipe}; partial-span loads on Timoshenko members are not yet supported, so mark the whole span (exposed_pipe_refs) or select euler_bernoulli",
                    case.id,
                ),
""", """                format!(
                    "load case {} equivalent_static.wind.exposed_spans[{index}] is a partial-span member load on {} straight member {pipe}; partial-span loads on Timoshenko members are not yet supported, so mark the whole span (exposed_pipe_refs) or select euler_bernoulli",
                    case.id,
                    shear_model_name(model),
                ),
""")
edit(P, """/// §4: the named arc disclosure, once per Timoshenko result that contains a realized arc.
pub(crate) fn append_arc_disclosure(model: &PreviewModel, built: &BuiltModel, diagnostics: &mut Vec<Diagnostic>) {
    if !selects(model) {
        return;
    }
""", """/// §4: the named arc disclosure, once per v1 Timoshenko result that contains a realized arc.
/// Under v2 the arcs carry shear, so nothing is disclosed (`curved-bend-transverse-shear.md` §5).
pub(crate) fn append_arc_disclosure(model: &PreviewModel, built: &BuiltModel, diagnostics: &mut Vec<Diagnostic>) {
    if !selects(model) || selects_curved(model) {
        return;
    }
""")

P = "core/product_physics/src/lib.rs"
edit(P, """fn mechanics_producer_for_model(model: &PreviewModel) -> MechanicsProducer {
    mechanics_producer(if pressure_runtime::is_exact(model) {
        PRESSURE_SEMANTIC_CONTRACT_ID
    } else {
""", """fn mechanics_producer_for_model(model: &PreviewModel) -> MechanicsProducer {
    mechanics_producer(if pressure_runtime::selects_arc_shear(model) {
        // DEL-04-01 `curved-bend-transverse-shear.md` §5: its own exact identity.
        exact_admission::PRESSURE_CURVED_SHEAR_SEMANTIC_CONTRACT_ID
    } else if pressure_runtime::is_exact(model) {
        PRESSURE_SEMANTIC_CONTRACT_ID
    } else {
""")
edit(P, """        let mut basis = exact_admission::pressure_v3_formulation_basis_with_source(
            load_state,
            connectors,
            source_selected,
        );
""", """        let mut basis = if pressure_runtime::selects_arc_shear(model) {
            exact_admission::pressure_v3_curved_shear_formulation_basis(load_state, connectors)
        } else {
            exact_admission::pressure_v3_formulation_basis_with_source(load_state, connectors, source_selected)
        };
""")
edit(P, """    let mut member_moduli: HashMap<&str, (f64, f64)> = HashMap::new();
    let mut exact_sections = HashMap::new();
""", """    let mut member_moduli: HashMap<&str, (f64, f64)> = HashMap::new();
    // DEL-04-01 `curved-bend-transverse-shear.md` §5: each realized-bend pipe's transverse
    // shear area A_s under timoshenko_annulus_energy_curved_v2, by its route's straight rule.
    // Every other document inserts nothing.
    let mut member_shear_areas: HashMap<&str, f64> = HashMap::new();
    let mut exact_sections = HashMap::new();
""")

edit(P, """        // realized arc stays Euler-Bernoulli. A missing basis or a factor out of
        // range blocks by name; no default factor exists.
        let shear_area = if pressure_runtime::selects_timoshenko(model) && !curved_bend_pipe_ids.contains(pipe.id.as_str()) {
""", """        // realized arc stays Euler-Bernoulli (timoshenko_annulus_energy_curved_v2 is
        // refused at validation until its exact identity is admitted; DEL-04-01
        // `curved-bend-transverse-shear.md` §5). A missing basis or a factor out of
        // range blocks by name; no default factor exists.
        let arc = curved_bend_pipe_ids.contains(pipe.id.as_str());
        let shear_area = if pressure_runtime::selects_timoshenko(model) && !arc {
""")
edit(P, """        } else if let Some(basis) = pf_shear.filter(|_| !curved_bend_pipe_ids.contains(pipe.id.as_str())) {
            // PF Timoshenko (DEL-05-03 Design `pf-timoshenko.md` §2): every straight
            // member on a consumed basis, from PF's stiffness annulus and the nu of
            // the unscaled (E, G) above; a refused factor blocks the member by name.
""", """        } else if let Some(basis) = pf_shear.filter(|_| !arc || pf_timoshenko::selects_curved(model)) {
            // PF Timoshenko (DEL-05-03 Design `pf-timoshenko.md` §2): every straight
            // member on a consumed basis, and under v2 every realized arc too, from PF's
            // stiffness annulus and the nu of the unscaled (E, G) above; a refused factor
            // blocks the member by name.
""")
edit(P, """        let section = match shear_area.map_or_else(|| StraightPipeSectionProperties::new(
""", """        // A realized arc's chord section stays Euler-Bernoulli: its A_s reaches only the
        // macro-element, through `member_shear_areas`.
        let section = match shear_area.filter(|_| !arc).map_or_else(|| StraightPipeSectionProperties::new(
""")
edit(P, """        if curved_bend_pipe_ids.contains(pipe.id.as_str()) {
            member_moduli.insert(pipe.id.as_str(), (elastic_modulus, shear_modulus));
        } else if !replaced_span_ids.contains(pipe.id.as_str()) {
""", """        if arc {
            member_moduli.insert(pipe.id.as_str(), (elastic_modulus, shear_modulus));
            if let Some(shear_area) = shear_area {
                member_shear_areas.insert(pipe.id.as_str(), shear_area);
            }
        } else if !replaced_span_ids.contains(pipe.id.as_str()) {
""")
edit(P, """    let curved_bend_elements = build_curved_bend_macro_elements(
        model,
        &nodes,
        &node_map,
        &sections,
        &member_moduli,
        diagnostics,
    );
""", """    let curved_bend_elements = build_curved_bend_macro_elements(
        model,
        &nodes,
        &node_map,
        &sections,
        &member_moduli,
        &member_shear_areas,
        diagnostics,
    );
""")
edit(P, """    sections: &HashMap<String, DerivedSection>,
    member_moduli: &HashMap<&str, (f64, f64)>,
    diagnostics: &mut Vec<Diagnostic>,
) -> Vec<CurvedBendMacroBuild> {
""", """    sections: &HashMap<String, DerivedSection>,
    member_moduli: &HashMap<&str, (f64, f64)>,
    member_shear_areas: &HashMap<&str, f64>,
    diagnostics: &mut Vec<Diagnostic>,
) -> Vec<CurvedBendMacroBuild> {
""")
edit(P, """            flexibility,
            flexibility,
        ) {
            Ok(element) => element,
            Err(error) => {
                diagnostics.push(curved_bend_macro_element_diag(
                    component,
                    pipe_ref,
                    &error.to_string(),
                ));
                continue;
            }
        };
        let (global_stiffness, arc_length, included_angle) = match (
""", """            flexibility,
            flexibility,
        ) {
            Ok(element) => element,
            Err(error) => {
                diagnostics.push(curved_bend_macro_element_diag(
                    component,
                    pipe_ref,
                    &error.to_string(),
                ));
                continue;
            }
        };
        // DEL-04-01 `curved-bend-transverse-shear.md` §5: a v2 arc carries its route's A_s;
        // every other arc is formed exactly as before (no shear arithmetic for `None`).
        let element = match member_shear_areas.get(pipe_ref) {
            None => element,
            Some(&shear_area) => match element.with_transverse_shear_area(shear_area) {
                Ok(element) => element,
                Err(error) => {
                    diagnostics.push(curved_bend_macro_element_diag(component, pipe_ref, &error.to_string()));
                    continue;
                }
            },
        };
        let (global_stiffness, arc_length, included_angle) = match (
""")

P = "core/product_physics/src/modal/prepare.rs"
edit(P, """    if pf_timoshenko::selects(&model) || pressure_runtime::selects_timoshenko(&model) {
        diagnostics.push(diag(
            "diagnostic:modal:timoshenko-mass",
            MODAL_TIMOSHENKO_MASS_UNSUPPORTED,
            "blocking",
            "the model selects Timoshenko straight members (timoshenko_annulus_energy_v1); modal analysis forms an Euler-Bernoulli consistent mass, and pairing it with shear-flexible stiffness would be inconsistent, so nothing is formed (a Timoshenko consistent mass is a later slice, M4)",
            vec!["shear_model".to_string()],
        ));
""", """    if pf_timoshenko::selects(&model)
        || pressure_runtime::selects_timoshenko(&model)
        || pressure_runtime::selects_arc_shear(&model)
    {
        // DEL-04-01 `curved-bend-transverse-shear.md` §5: v2 is refused under the same code,
        // naming its value, on either route (the exact route also refuses it at validation
        // until its identity is admitted); its arcs would otherwise pair an Euler-Bernoulli
        // mass with shear-flexible arcs too.
        let message = if pf_timoshenko::selects_curved(&model) || pressure_runtime::selects_arc_shear(&model) {
            "the model selects Timoshenko straight members and realized arcs (timoshenko_annulus_energy_curved_v2); modal analysis forms an Euler-Bernoulli consistent mass, and pairing it with shear-flexible stiffness would be inconsistent, so nothing is formed (a Timoshenko consistent mass is a later slice, M4)"
        } else {
            "the model selects Timoshenko straight members (timoshenko_annulus_energy_v1); modal analysis forms an Euler-Bernoulli consistent mass, and pairing it with shear-flexible stiffness would be inconsistent, so nothing is formed (a Timoshenko consistent mass is a later slice, M4)"
        };
        diagnostics.push(diag(
            "diagnostic:modal:timoshenko-mass",
            MODAL_TIMOSHENKO_MASS_UNSUPPORTED,
            "blocking",
            message,
            vec!["shear_model".to_string()],
        ));
""")

P = "core/reporting/modal_result/src/model_mass.rs"
edit(P, """const MM_TIMOSHENKO: &str = "timoshenko_annulus_energy_v1";
""", """const MM_TIMOSHENKO: &str = "timoshenko_annulus_energy_v1";
/// DEL-04-01 `curved-bend-transverse-shear.md` §5: refused by the producer under the same code.
const MM_TIMOSHENKO_CURVED: &str = "timoshenko_annulus_energy_curved_v2";
""")
edit(P, """        Some(MM_TIMOSHENKO) => return mm_fail("the bound model selects Timoshenko members, which modal analysis refuses (MODAL_TIMOSHENKO_MASS_UNSUPPORTED)"),
""", """        Some(MM_TIMOSHENKO | MM_TIMOSHENKO_CURVED) => return mm_fail("the bound model selects Timoshenko members, which modal analysis refuses (MODAL_TIMOSHENKO_MASS_UNSUPPORTED)"),
""")

P = "schemas/model.schema.yaml"
edit(P, """        "mode": {
          "type": "string",
          "const": "exact_pressure_v3"
        },
        "shear_model": {
          "type": "string",
          "enum": [
            "euler_bernoulli",
            "timoshenko_annulus_energy_v1"
          ]
        }
""", """        "mode": {
          "type": "string",
          "const": "exact_pressure_v3"
        },
        "shear_model": {
          "description": "euler_bernoulli; timoshenko_annulus_energy_v1 (Timoshenko straight members, realized arcs Euler–Bernoulli); or timoshenko_annulus_energy_curved_v2 (v1's straight members, and realized arcs with transverse shear by the same energy-matched annular factor; DEL-04-01 Design curved-bend-transverse-shear.md). Any other value is refused by name.",
          "type": "string",
          "enum": [
            "euler_bernoulli",
            "timoshenko_annulus_energy_v1",
            "timoshenko_annulus_energy_curved_v2"
          ]
        }
""")
edit(P, """An explicit user set of either value writes the member (as for the tolerance); the product never writes the default into a document the user did not author, and clear restores the absent form. timoshenko_annulus_energy_v1 makes every straight member of the pressure-free document Timoshenko, with the exact route's energy-matched annular shear factor and Poisson's ratio E/(2G) - 1; realized arcs stay Euler–Bernoulli (DEL-05-03 Design pf-timoshenko.md). Refused""", """An explicit user set of any value writes the member (as for the tolerance); the product never writes the default into a document the user did not author, and clear restores the absent form. timoshenko_annulus_energy_v1 makes every straight member of the pressure-free document Timoshenko, with the exact route's energy-matched annular shear factor and Poisson's ratio E/(2G) - 1; realized arcs stay Euler–Bernoulli (DEL-05-03 Design pf-timoshenko.md). timoshenko_annulus_energy_curved_v2 forms the same straight members and adds transverse shear to every realized arc by the same factor (DEL-04-01 Design curved-bend-transverse-shear.md). Refused""")
edit(P, """          "type": "string",
          "enum": [
            "euler_bernoulli",
            "timoshenko_annulus_energy_v1"
          ]
        },
        "node_proximity_tolerance": {""", """          "type": "string",
          "enum": [
            "euler_bernoulli",
            "timoshenko_annulus_energy_v1",
            "timoshenko_annulus_energy_curved_v2"
          ]
        },
        "node_proximity_tolerance": {""")

P = "schemas/physical_transform_model.schema.yaml"
edit(P, """shear_model selects the straight members' shear deformation: euler_bernoulli, or timoshenko_annulus_energy_v1 (the energy-matched annular shear factor on the nominal section); realized arcs are Euler–Bernoulli under either.""",
     """shear_model selects the shear deformation: euler_bernoulli; timoshenko_annulus_energy_v1 (the energy-matched annular shear factor on the nominal section for straight members; realized arcs Euler–Bernoulli); or timoshenko_annulus_energy_curved_v2 (the same straight members, and realized arcs with transverse shear by the same factor).""")
edit(P, """          "enum": [
            "euler_bernoulli",
            "timoshenko_annulus_energy_v1"
          ]""", """          "enum": [
            "euler_bernoulli",
            "timoshenko_annulus_energy_v1",
            "timoshenko_annulus_energy_curved_v2"
          ]""")

# ================================================================ operation applier
P = "core/model_operations/operation_applier/src/lib.rs"
edit(P, """            && matches!(contract.get("shear_model").and_then(Value::as_str),
                Some("euler_bernoulli" | "timoshenko_annulus_energy_v1")))
""", """            // The 3.0.0 vocabulary (DEL-04-01 `curved-bend-transverse-shear.md` §5): v2 is a
            // closed-contract value; the exact route refuses it at validation until B2, so the
            // authoring texts still name euler_bernoulli and timoshenko_annulus_energy_v1 only.
            && matches!(contract.get("shear_model").and_then(Value::as_str),
                Some("euler_bernoulli" | "timoshenko_annulus_energy_v1" | "timoshenko_annulus_energy_curved_v2")))
""")

# ================================================================ desktop
P = "apps/desktop/src/types.ts"

edit(P, """ * `timoshenko_annulus_energy_v1` (DEL-05-03 Design `pf-timoshenko.md`); it is refused beside a
 * pressure contract. `pressure_elongation`""", """ * `timoshenko_annulus_energy_v1` (DEL-05-03 Design `pf-timoshenko.md`), or
 * `timoshenko_annulus_energy_curved_v2` (realized arcs too; DEL-04-01 Design
 * `curved-bend-transverse-shear.md`); it is refused beside a pressure contract. `pressure_elongation`""")

P = "apps/desktop/src/features/model-settings/ModelSettingsSection.tsx"
edit(P, """/** The straight-member shear deformation choices (pf-timoshenko.md §1), default first. */
export const SHEAR_MODEL_CHOICES = Object.freeze([
  { value: "euler_bernoulli", label: "Euler–Bernoulli (the default)" },
  { value: "timoshenko_annulus_energy_v1", label: "Timoshenko (energy-matched annular factor)" }
] as const);
export const SHEAR_MODEL_NOTE =
  "Applies to every straight member, including tees' straight legs, rigid-family components and geometry-only bends. Realized arcs stay Euler–Bernoulli. Timoshenko members use Poisson's ratio E/(2G) − 1 from each case's moduli; an authored Poisson's ratio is not used.";
""", """/** The shear deformation choices (pf-timoshenko.md §1; curved-bend-transverse-shear.md §5), default first. */
export const SHEAR_MODEL_CHOICES = Object.freeze([
  { value: "euler_bernoulli", label: "Euler–Bernoulli (the default)" },
  { value: "timoshenko_annulus_energy_v1", label: "Timoshenko (energy-matched annular factor)" },
  { value: "timoshenko_annulus_energy_curved_v2", label: "Timoshenko, straight members and bends (energy-matched annular factor)" }
] as const);
export const SHEAR_MODEL_NOTE =
  "Applies to every straight member, including tees' straight legs, rigid-family components and geometry-only bends. Realized arcs stay Euler–Bernoulli unless you choose straight members and bends, which applies the same factor along each arc (no curvature correction; arc stresses stay the straight-annulus recovery). Timoshenko members use Poisson's ratio E/(2G) − 1 from each case's moduli; an authored Poisson's ratio is not used. Modal analysis refuses either Timoshenko choice.";
""")
P = "apps/desktop/src/features/model-settings/modelSettingsIntents.ts"
edit(P, """/** Set the shear model to `name`, `euler_bernoulli` or `timoshenko_annulus_energy_v1` (the
""", """/** Set the shear model to `name`, `euler_bernoulli`, `timoshenko_annulus_energy_v1` or
 * `timoshenko_annulus_energy_curved_v2` (the
""")

# ================================================================ governing text (DEL-04-01)
P = "execution/PKG-04/DEL-04-01/Work/curved-bend-transverse-shear.md"
edit(P, """This boundary was audited at `5f2946ef12`. Step A, the kernel and
certification, is implemented on branch `codex/bend-transverse-shear`; no
product path sets the shear area yet. Step B, product admission, is reserved
until Agent 0 reviews its actual diff.
""", """This boundary was audited at `5f2946ef12`. Step A, the kernel and
certification, is implemented. Step B is delivered in two changes:

- **B1** adds `timoshenko_annulus_energy_curved_v2` to the
  `model_settings.shear_model` values and, amended in place, to the 3.0.0
  `pressure_contract.shear_model` vocabulary. It admits v2 on the
  pressure-free route under `preview-physics-curved-shear-1`
  (`product_preview_mechanics_curved_shear_v1`). A document without a realized
  arc is recorded under that identity with rows bit-equal to v1's. Modal
  analysis refuses v2 by name, MBF export and import keep their refusals, and
  the user-release package refuses the identity by name. The exact route
  declines v2 by name (`EXACT_PRESSURE_SHEAR_MODEL_NOT_ADMITTED`), never
  solving it as v1.
- **B2** admits v2 on the exact route under `pressure-curved-shear-1`
  (`exact_pressure_v3_curved_shear_v1`), derived from pressure-3, and removes
  that refusal. The retained-source method declines v2 by name.
""")
edit(P, """### Step B: product admission (reserved; DEL-03-03, DEL-05-03, DEL-02-01, DEL-04-07)
""", """### Step B: product admission (DEL-03-03, DEL-05-03, DEL-02-01, DEL-04-07)

Mapped deliverables (`tools.deliverables`' matching rule): Step B also reaches DEL-05-02 (result
semantics, readers and the derived table), DEL-04-08 and DEL-04-09 (verifier
twins, modal mass), DEL-08-01, DEL-08-02, DEL-08-04 and DEL-08-06 (report
package admission and projection), DEL-07-03, DEL-07-05, DEL-07-07, DEL-07-11
and DEL-07-12 (desktop result, report and export consumers), DEL-04-06 and
DEL-16-06 (model settings control and authoring), DEL-14-02 (analysis-run
records), DEL-17-06 (stress-neutral export) and DEL-13-04 (the
physical-transform schema).
""")



# ================================================================ EXTRA
P = "execution/PKG-04/DEL-04-01/Work/curved-bend-transverse-shear.md"
edit(P, """results and the minimal implementation boundary. Nothing here is implemented
or accepted. The independent reference is
""", """results and the minimal implementation boundary. Step A and B1 (§5) are
implemented; nothing here is accepted. The independent reference is
""")
edit(P, """## 9. Open choices (Agent 0)

1. **Exact-contract vocabulary.** `pressure_contract` 3.0.0 is a closed
  vocabulary. Adding v2 there is a contract change, while the
  `model_settings` route takes it additively.
  - The recommendation is to admit v2 on both routes in Step B, so that
    pressure documents can take arc shear.
  - The alternative is PF first and the exact route later.
2. **UI wording and the offer.** These need an explicit design. The proposed
  label is "Timoshenko, straight members and bends (energy-matched annular
  factor)".
3. **Result identity names** for v2 (semantic ID and profile).""", """## 9. Choices (decided by Agent 0)

1. **Exact-contract vocabulary.** v2 joins the closed 3.0.0
  `pressure_contract` vocabulary in place, with no new contract version. Both
  routes take it, in two changes (§5): B1 admits the pressure-free route and
  declines the exact route by name, and B2 admits the exact route.
2. **UI wording and the offer.** The label is "Timoshenko, straight members
  and bends (energy-matched annular factor)". The control's legend is "Shear
  deformation". Refusal texts that list the admitted values name v2, except the
  exact route's authoring texts, which name euler_bernoulli and
  timoshenko_annulus_energy_v1 until B2 admits v2 there.
3. **Result identity names.** On the pressure-free route the identity is
  `preview-physics-curved-shear-1` with profile
  `product_preview_mechanics_curved_shear_v1`. On the exact route (B2) it is
  `pressure-curved-shear-1` on pressure-3, with profile
  `exact_pressure_v3_curved_shear_v1`. A v2 document without a realized arc is
  recorded under its v2 identity, with rows bit-equal to v1's.
4. **Other consumers.** The user-release package refuses both curved
  identities by name. The retained-source method declines v2 by name (B2).""")
def sub(text):
    return (text.replace("@PF_ID@", PF_ID).replace("@PF_PROFILE@", PF_PROFILE).replace("@PF_SHA@", PF_SHA)
            .replace("@PF_LIM0@", PF_LIM0).replace("@PF_LIM0_JSON@", json.dumps(PF_LIM0)))


def edit_s(path, old, new):
    edit(path, old, sub(new))


# ---------------------------------------------------------------- pressure_runtime (B1: vocabulary admitted, solve declined by name)
P = "core/product_physics/src/pressure_runtime.rs"
edit(P, """/// The exact contract's two shear models (DEL-05-03 Design §3, §5).
pub(crate) const EULER_BERNOULLI: &str = "euler_bernoulli";
pub(crate) const TIMOSHENKO_ANNULUS_ENERGY: &str = "timoshenko_annulus_energy_v1";
""", """/// The exact contract's shear models (DEL-05-03 Design §3, §5; DEL-04-01 Design
/// `curved-bend-transverse-shear.md` §5 adds `timoshenko_annulus_energy_curved_v2` to the
/// 3.0.0 vocabulary, which this route declines by name until its own identity is admitted).
pub(crate) const EULER_BERNOULLI: &str = "euler_bernoulli";
pub(crate) const TIMOSHENKO_ANNULUS_ENERGY: &str = "timoshenko_annulus_energy_v1";
pub(crate) const TIMOSHENKO_ANNULUS_ENERGY_CURVED: &str = "timoshenko_annulus_energy_curved_v2";
/// `curved-bend-transverse-shear.md` §5: the named refusal of an exact document selecting
/// `timoshenko_annulus_energy_curved_v2` before the exact curved-shear identity exists.
pub(crate) const EXACT_PRESSURE_SHEAR_MODEL_NOT_ADMITTED: &str = "EXACT_PRESSURE_SHEAR_MODEL_NOT_ADMITTED";
""")
edit(P, """            == Some(TIMOSHENKO_ANNULUS_ENERGY)
}

""", """            == Some(TIMOSHENKO_ANNULUS_ENERGY)
}

/// `curved-bend-transverse-shear.md` §5: whether an exact-contract document selects
/// `timoshenko_annulus_energy_curved_v2`. Such a document is refused at validation
/// ([`EXACT_PRESSURE_SHEAR_MODEL_NOT_ADMITTED`]), so no exact solve forms it; modal
/// preparation names it in its own refusal.
pub(crate) fn selects_arc_shear(model: &PreviewModel) -> bool {
    is_exact(model)
        && model.pressure_contract.as_ref().and_then(|contract| contract.shear_model.as_deref())
            == Some(TIMOSHENKO_ANNULUS_ENERGY_CURVED)
}

""")
edit(P, """                        // T4-U8 (DEL-05-03 Design §5): both shear models are admitted.
                        Some(EULER_BERNOULLI | TIMOSHENKO_ANNULUS_ENERGY) => {}
""", """                        // T4-U8 (DEL-05-03 Design §5): both shear models are admitted.
                        Some(EULER_BERNOULLI | TIMOSHENKO_ANNULUS_ENERGY) => {}
                        // `curved-bend-transverse-shear.md` §5: in the vocabulary, not yet solved
                        // on this route; refused by name, never solved as v1.
                        Some(TIMOSHENKO_ANNULUS_ENERGY_CURVED) => problem(diagnostics, EXACT_PRESSURE_SHEAR_MODEL_NOT_ADMITTED,
                            &["pressure_contract", "shear_model"],
                            "The exact pressure route does not admit shear model timoshenko_annulus_energy_curved_v2; use euler_bernoulli or timoshenko_annulus_energy_v1 on this route."),
""")

# ---------------------------------------------------------------- semantic_contract.rs (PF only)
P = "core/reporting/result_export/src/semantic_contract.rs"
edit_s(P, """pub const PREVIEW_PHYSICS_TIMOSHENKO_SHA256: &str =
    "8a0abaeac93729b5d6ee6c922f2a60f0caf9332948622bb8b5896747ce41442d";
/// The preview-physics route's admitted identities, `(id, profile)`, in one place: every
/// identity-aware site keys by the envelope's `semantic_contract_id` through this list.
pub const PREVIEW_PHYSICS_IDENTITIES: [(&str, &str); 2] = [
    (PREVIEW_PHYSICS_ID, PREVIEW_PHYSICS_PROFILE),
    (PREVIEW_PHYSICS_TIMOSHENKO_ID, PREVIEW_PHYSICS_TIMOSHENKO_PROFILE),
];
/// Either preview-physics identity (Euler–Bernoulli or Timoshenko).
""", """pub const PREVIEW_PHYSICS_TIMOSHENKO_SHA256: &str =
    "8a0abaeac93729b5d6ee6c922f2a60f0caf9332948622bb8b5896747ce41442d";
/// DEL-04-01 Design `curved-bend-transverse-shear.md` §5: the pressure-free curved-shear
/// identity (`model_settings.shear_model = timoshenko_annulus_energy_curved_v2`), a third
/// complete identity on the route, derived from preview-physics-2 as the Timoshenko one is
/// (`semantic_contract_tables.py derive`).
pub const PREVIEW_PHYSICS_CURVED_SHEAR_ID: &str =
    "@PF_ID@";
pub const PREVIEW_PHYSICS_CURVED_SHEAR_PROFILE: &str = "@PF_PROFILE@";
pub const PREVIEW_PHYSICS_CURVED_SHEAR_SHA256: &str =
    "@PF_SHA@";
/// The preview-physics route's admitted identities, `(id, profile)`, in one place: every
/// identity-aware site keys by the envelope's `semantic_contract_id` through this list.
pub const PREVIEW_PHYSICS_IDENTITIES: [(&str, &str); 3] = [
    (PREVIEW_PHYSICS_ID, PREVIEW_PHYSICS_PROFILE),
    (PREVIEW_PHYSICS_TIMOSHENKO_ID, PREVIEW_PHYSICS_TIMOSHENKO_PROFILE),
    (PREVIEW_PHYSICS_CURVED_SHEAR_ID, PREVIEW_PHYSICS_CURVED_SHEAR_PROFILE),
];
/// Any preview-physics identity (Euler–Bernoulli, Timoshenko or curved shear).
""")
edit(P, """        PREVIEW_PHYSICS_TIMOSHENKO_ID => {
            Some((preview_physics_timoshenko_contract(), PREVIEW_PHYSICS_TIMOSHENKO_PROFILE))
        }
        _ => None,
""", """        PREVIEW_PHYSICS_TIMOSHENKO_ID => {
            Some((preview_physics_timoshenko_contract(), PREVIEW_PHYSICS_TIMOSHENKO_PROFILE))
        }
        // No recorded variant exists: the identity is newer than every recorded run.
        PREVIEW_PHYSICS_CURVED_SHEAR_ID => {
            Some((preview_physics_curved_shear_contract(), PREVIEW_PHYSICS_CURVED_SHEAR_PROFILE))
        }
        _ => None,
""")
edit(P, """        .expect("pinned preview-physics-timoshenko semantic contract")
    })
}
""", """        .expect("pinned preview-physics-timoshenko semantic contract")
    })
}
/// Pinned curved-shear table bytes: identity, profile and sha256 are checked.
pub fn verify_preview_physics_curved_shear_table(bytes: &[u8]) -> Result<Value, String> {
    use sha2::{Digest, Sha256};
    if format!("{:x}", Sha256::digest(bytes)) != PREVIEW_PHYSICS_CURVED_SHEAR_SHA256 {
        return Err("SOURCE_PREVIEW_PHYSICS_CURVED_SHEAR_TABLE_HASH".into());
    }
    let table: Value = serde_json::from_slice(bytes)
        .map_err(|_| "SOURCE_PREVIEW_PHYSICS_CURVED_SHEAR_TABLE_HASH".to_string())?;
    if table["semantic_contract_id"] != PREVIEW_PHYSICS_CURVED_SHEAR_ID
        || table["formulation_profile_id"] != PREVIEW_PHYSICS_CURVED_SHEAR_PROFILE
    {
        return Err("SOURCE_PREVIEW_PHYSICS_CURVED_SHEAR_TABLE_IDENTITY".into());
    }
    Ok(table)
}
pub fn preview_physics_curved_shear_contract() -> &'static Value {
    static CONTRACT: OnceLock<Value> = OnceLock::new();
    CONTRACT.get_or_init(|| {
        verify_preview_physics_curved_shear_table(include_bytes!(
            "../../../../fixtures/results/semantic_contract_v0_3_preview_physics_curved_shear_1.json"
        ))
        .expect("pinned preview-physics-curved-shear semantic contract")
    })
}
""")
edit(P, """                Some(
                    PRESSURE_ID | PRESSURE_SOURCE_ID | PREVIEW_PHYSICS_ID | PREVIEW_PHYSICS_TIMOSHENKO_ID
                )
""", """                Some(
                    PRESSURE_ID
                        | PRESSURE_SOURCE_ID
                        | PREVIEW_PHYSICS_ID
                        | PREVIEW_PHYSICS_TIMOSHENKO_ID
                        | PREVIEW_PHYSICS_CURVED_SHEAR_ID
                )
""")
edit(P, """                Some(PREVIEW_PHYSICS_TIMOSHENKO_ID) => PREVIEW_PHYSICS_TIMOSHENKO_PROFILE,
                _ => PREVIEW_PHYSICS_PROFILE,
""", """                Some(PREVIEW_PHYSICS_TIMOSHENKO_ID) => PREVIEW_PHYSICS_TIMOSHENKO_PROFILE,
                Some(PREVIEW_PHYSICS_CURVED_SHEAR_ID) => PREVIEW_PHYSICS_CURVED_SHEAR_PROFILE,
                _ => PREVIEW_PHYSICS_PROFILE,
""")
edit(P, """                Some(id @ (PREVIEW_PHYSICS_ID | PREVIEW_PHYSICS_TIMOSHENKO_ID)) => {
                    preview_physics_identity(id).expect("a preview-physics identity").0
""", """                Some(id @ (PREVIEW_PHYSICS_ID | PREVIEW_PHYSICS_TIMOSHENKO_ID | PREVIEW_PHYSICS_CURVED_SHEAR_ID)) => {
                    preview_physics_identity(id).expect("a preview-physics identity").0
""")
edit(P, """        Some(PREVIEW_PHYSICS_ID | PREVIEW_PHYSICS_TIMOSHENKO_ID) => validate_preview_physics_evidence(source)?,
""", """        Some(PREVIEW_PHYSICS_ID | PREVIEW_PHYSICS_TIMOSHENKO_ID | PREVIEW_PHYSICS_CURVED_SHEAR_ID) => {
            validate_preview_physics_evidence(source)?
        }
""")
edit(P, """    // selects `timoshenko_annulus_energy_v1`.
    PREVIEW_PHYSICS_TIMOSHENKO_ID,
    PRESSURE_SOURCE_ID,
""", """    // selects `timoshenko_annulus_energy_v1`.
    PREVIEW_PHYSICS_TIMOSHENKO_ID,
    // curved-bend-transverse-shear.md §5: one selecting `timoshenko_annulus_energy_curved_v2`.
    PREVIEW_PHYSICS_CURVED_SHEAR_ID,
    PRESSURE_SOURCE_ID,
""")

# ---------------------------------------------------------------- reader_verdict.rs (PF only)
P = "core/reporting/result_export/src/reader_verdict.rs"
edit(P, """const BINDINGS: [(&str, &str, &str); 6] = [
    ("pressure_source", sc::PRESSURE_SOURCE_ID, sc::PRESSURE_SOURCE_TABLE_SHA256),
    ("preview_physics", sc::PREVIEW_PHYSICS_ID, sc::PREVIEW_PHYSICS_SHA256),
    ("preview_physics", sc::PREVIEW_PHYSICS_TIMOSHENKO_ID, sc::PREVIEW_PHYSICS_TIMOSHENKO_SHA256),
""", """const BINDINGS: [(&str, &str, &str); 7] = [
    ("pressure_source", sc::PRESSURE_SOURCE_ID, sc::PRESSURE_SOURCE_TABLE_SHA256),
    ("preview_physics", sc::PREVIEW_PHYSICS_ID, sc::PREVIEW_PHYSICS_SHA256),
    ("preview_physics", sc::PREVIEW_PHYSICS_TIMOSHENKO_ID, sc::PREVIEW_PHYSICS_TIMOSHENKO_SHA256),
    // curved-bend-transverse-shear.md §5: the route's third identity.
    ("preview_physics", sc::PREVIEW_PHYSICS_CURVED_SHEAR_ID, sc::PREVIEW_PHYSICS_CURVED_SHEAR_SHA256),
""")
edit(P, """        Some(sc::PREVIEW_PHYSICS_ID | sc::PREVIEW_PHYSICS_TIMOSHENKO_ID) => "preview_physics",
""", """        Some(sc::PREVIEW_PHYSICS_ID | sc::PREVIEW_PHYSICS_TIMOSHENKO_ID | sc::PREVIEW_PHYSICS_CURVED_SHEAR_ID) => "preview_physics",
""")
edit(P, """/// the Timoshenko identity; an absent member (or an absent `model_settings`) or
""", """/// the Timoshenko identity, and `timoshenko_annulus_energy_curved_v2` the curved-shear identity
/// (`curved-bend-transverse-shear.md` §5); an absent member (or an absent `model_settings`) or
""")
edit(P, """/// not an object, `null` included (`MODEL-SLOT-SHAPE`), and a `shear_model` that is not one
/// of the two names, `null` and non-strings included (`MODEL-SETTING-SHEAR-MODEL-UNKNOWN`).
""", """/// not an object, `null` included (`MODEL-SLOT-SHAPE`), and a `shear_model` that is not one
/// of the three names, `null` and non-strings included (`MODEL-SETTING-SHEAR-MODEL-UNKNOWN`).
""")
edit(P, """        Some(Some("timoshenko_annulus_energy_v1")) => sc::PREVIEW_PHYSICS_TIMOSHENKO_ID,
        Some(_) => {
            return Err(format!("{SHEAR_MODEL_IDENTITY}: MODEL-SETTING-SHEAR-MODEL-UNKNOWN: model_settings.shear_model must be `euler_bernoulli` or `timoshenko_annulus_energy_v1`"));
""", """        Some(Some("timoshenko_annulus_energy_v1")) => sc::PREVIEW_PHYSICS_TIMOSHENKO_ID,
        Some(Some("timoshenko_annulus_energy_curved_v2")) => sc::PREVIEW_PHYSICS_CURVED_SHEAR_ID,
        Some(_) => {
            return Err(format!("{SHEAR_MODEL_IDENTITY}: MODEL-SETTING-SHEAR-MODEL-UNKNOWN: model_settings.shear_model must be `euler_bernoulli`, `timoshenko_annulus_energy_v1` or `timoshenko_annulus_energy_curved_v2`"));
""")

# ---------------------------------------------------------------- derivative.rs
P = "core/reporting/result_export/src/derivative.rs"
edit(P, """                    | crate::semantic_contract::PREVIEW_PHYSICS_TIMOSHENKO_ID
                    | PREVIEW_PHYSICS_RETAINED_ID
""", """                    | crate::semantic_contract::PREVIEW_PHYSICS_TIMOSHENKO_ID
                    | crate::semantic_contract::PREVIEW_PHYSICS_CURVED_SHEAR_ID
                    | PREVIEW_PHYSICS_RETAINED_ID
""")

# ---------------------------------------------------------------- report package
P = "core/reporting/report_package/src/document_form.rs"
edit(P, """    for_export_metadata, PRESSURE_ID, PRESSURE_SOURCE_ID, PREVIEW_PHYSICS_ID, PREVIEW_PHYSICS_TIMOSHENKO_ID, RESULT_EXPORT_VERSION,
};
""", """    for_export_metadata, PRESSURE_ID, PRESSURE_SOURCE_ID, PREVIEW_PHYSICS_CURVED_SHEAR_ID, PREVIEW_PHYSICS_ID,
    PREVIEW_PHYSICS_TIMOSHENKO_ID, RESULT_EXPORT_VERSION,
};
""")
edit(P, """/// pressure-2 and pressure-source-1 routes. The pressure-free route admits both of
/// its identities (DEL-05-03 Design `pf-timoshenko.md` §4); a Timoshenko report
/// carries its projection (`pf_timoshenko`).
pub const ADMITTED_SEMANTIC_CONTRACTS: [&str; 4] =
    [PREVIEW_PHYSICS_ID, PREVIEW_PHYSICS_TIMOSHENKO_ID, PRESSURE_ID, PRESSURE_SOURCE_ID];
""", """/// pressure-2 and pressure-source-1 routes. The pressure-free route admits each of
/// its identities (DEL-05-03 Design `pf-timoshenko.md` §4; DEL-04-01 Design
/// `curved-bend-transverse-shear.md` §5); a Timoshenko or curved-shear report carries its
/// projection (`pf_timoshenko`).
pub const ADMITTED_SEMANTIC_CONTRACTS: [&str; 5] = [
    PREVIEW_PHYSICS_ID,
    PREVIEW_PHYSICS_TIMOSHENKO_ID,
    PREVIEW_PHYSICS_CURVED_SHEAR_ID,
    PRESSURE_ID,
    PRESSURE_SOURCE_ID,
];
""")
P = "core/reporting/report_package/src/pf_timoshenko.rs"
edit(P, """//! text and references, with the delivered result. This identity only: a
//! preview-physics-2 report carries no such entry and takes neither check, so
//! its bytes and checks are unchanged. Equality checks only. Standard claim
//! fence applies (claims taxonomy per DEC-081).

use open_pipe_stress_result_export::semantic_contract::{
    preview_physics_timoshenko_contract, PREVIEW_PHYSICS_TIMOSHENKO_ID,
};
""", """//! text and references, with the delivered result. A
//! `preview-physics-curved-shear-1` result (DEL-04-01 Design
//! `curved-bend-transverse-shear.md` §5) takes the same projection with its own
//! limitation 0 and no arc disclosure. These identities only: a
//! preview-physics-2 report carries no such entry and takes neither check, so
//! its bytes and checks are unchanged. Equality checks only. Standard claim
//! fence applies (claims taxonomy per DEC-081).

use open_pipe_stress_result_export::semantic_contract::{
    preview_physics_curved_shear_contract, preview_physics_timoshenko_contract, PREVIEW_PHYSICS_CURVED_SHEAR_ID,
    PREVIEW_PHYSICS_TIMOSHENKO_ID,
};
""")
edit(P, """/// Whether `id` is the pressure-free Timoshenko identity.
pub fn is_pf_timoshenko_identity(id: &str) -> bool {
    id == PREVIEW_PHYSICS_TIMOSHENKO_ID
}
""", """/// Whether `id` is a pressure-free Timoshenko identity (v1, or the curved-shear v2).
pub fn is_pf_timoshenko_identity(id: &str) -> bool {
    id == PREVIEW_PHYSICS_TIMOSHENKO_ID || id == PREVIEW_PHYSICS_CURVED_SHEAR_ID
}
""")
edit(P, """    let identity = source.pointer("/producer/semantic_contract_id").and_then(Value::as_str);
    if identity != Some(PREVIEW_PHYSICS_TIMOSHENKO_ID) {
        return Err(mismatch("the source is not a pressure-free Timoshenko result"));
    }
    let published = source.pointer("/formulation_basis/limitations/0").and_then(Value::as_str);
    let stated = preview_physics_timoshenko_contract()["supported_profile_limitations"][0].as_str();
""", """    let (identity, table) = match source.pointer("/producer/semantic_contract_id").and_then(Value::as_str) {
        Some(id @ PREVIEW_PHYSICS_TIMOSHENKO_ID) => (id, preview_physics_timoshenko_contract()),
        Some(id @ PREVIEW_PHYSICS_CURVED_SHEAR_ID) => (id, preview_physics_curved_shear_contract()),
        _ => return Err(mismatch("the source is not a pressure-free Timoshenko result")),
    };
    let published = source.pointer("/formulation_basis/limitations/0").and_then(Value::as_str);
    let stated = table["supported_profile_limitations"][0].as_str();
""")
edit(P, """        "source": {"ref_type": "semantic_contract", "ref_id": PREVIEW_PHYSICS_TIMOSHENKO_ID},
""", """        "source": {"ref_type": "semantic_contract", "ref_id": identity},
""")


# ---------------------------------------------------------------- generator (PF only)
P = "tools/serialization/semantic_contract_tables.py"
edit_s(P, """SCHEMA = "schemas/results_pressure_3.v0.3.schema.json"
""", """# DEL-04-01 Design curved-bend-transverse-shear.md §5: the pressure-free curved-shear identity, derived as the
# Timoshenko one (preview-physics-2 with its identity, profile and limitation 0 replaced). Never hand-edited.
CURVED = "fixtures/results/semantic_contract_v0_3_preview_physics_curved_shear_1.json"
CURVED_ID = "@PF_ID@"
CURVED_PROFILE = "@PF_PROFILE@"
CURVED_LIMITATION_0 = (
    "Small-displacement, linear-elastic frame preview: straight members, including geometry-only bends, and realized "
    "arcs are Timoshenko with the energy-matched annular shear factor (timoshenko_annulus_energy_curved_v2) and "
    "Poisson's ratio E/(2G) - 1; on a realized arc the straight-annulus factor is applied along the centreline with "
    "no curvature correction, the arc's thermal equivalent load is formed with that shear-dependent arc stiffness, "
    "as the solve is, and the arc's stress recovery stays the straight-annulus recovery; a straight member's "
    "published nodal rotations are section rotations, not slopes; numerical integrity does not establish physical "
    "correctness.")
SCHEMA = "schemas/results_pressure_3.v0.3.schema.json"
""")
edit(P, """        elif identity == TIMOSHENKO_ID:
            # pf-timoshenko.md §4: preview-physics-2's texts with limitation 0 replaced, as the table.
            branch["properties"]["formulation_basis"]["properties"]["limitations"] = {
                "const": [TIMOSHENKO_LIMITATION_0] + list(preview_limitations[1:])}
""", """        elif identity == TIMOSHENKO_ID:
            # pf-timoshenko.md §4: preview-physics-2's texts with limitation 0 replaced, as the table.
            branch["properties"]["formulation_basis"]["properties"]["limitations"] = {
                "const": [TIMOSHENKO_LIMITATION_0] + list(preview_limitations[1:])}
        elif identity == CURVED_ID:
            # curved-bend-transverse-shear.md §5: likewise for the curved-shear identity.
            branch["properties"]["formulation_basis"]["properties"]["limitations"] = {
                "const": [CURVED_LIMITATION_0] + list(preview_limitations[1:])}
""")
edit(P, """    return {PRESSURE: pressure_bytes, RETAINED: dumps(retained), TIMOSHENKO: timoshenko_table()}
""", """    return {PRESSURE: pressure_bytes, RETAINED: dumps(retained), TIMOSHENKO: timoshenko_table(),
            CURVED: preview_variant(CURVED_ID, CURVED_PROFILE, CURVED_LIMITATION_0)}
""")
edit(P, """    sentence is reviewed here rather than carried silently.\"\"\"
    preview = json.loads((PROJECT / PREVIEW).read_bytes())
    if preview["semantic_contract_id"] != PREVIEW_IDENTITIES[0] or preview["formulation_profile_id"] != "product_preview_mechanics_v1":
        raise SystemExit(f"{PREVIEW}: unexpected identity or profile")
    if preview["supported_profile_limitations"][0] != PREVIEW_LIMITATION_0:
        raise SystemExit(f"{PREVIEW}: limitation 0 is not the Euler-Bernoulli sentence this derivation replaces")
    preview["semantic_contract_id"] = TIMOSHENKO_ID
    preview["formulation_profile_id"] = TIMOSHENKO_PROFILE
    preview["supported_profile_limitations"][0] = TIMOSHENKO_LIMITATION_0
    return dumps(preview)
""", """    sentence is reviewed here rather than carried silently.\"\"\"
    return preview_variant(TIMOSHENKO_ID, TIMOSHENKO_PROFILE, TIMOSHENKO_LIMITATION_0)


def preview_variant(identity: str, profile: str, limitation_0: str) -> bytes:
    \"\"\"A preview-physics-2 variant (Timoshenko; curved shear, curved-bend-transverse-shear.md §5): the
    identity, the profile and limitation 0 replaced, every other member and its order preview-physics-2's.\"\"\"
    preview = json.loads((PROJECT / PREVIEW).read_bytes())
    if preview["semantic_contract_id"] != PREVIEW_IDENTITIES[0] or preview["formulation_profile_id"] != "product_preview_mechanics_v1":
        raise SystemExit(f"{PREVIEW}: unexpected identity or profile")
    if preview["supported_profile_limitations"][0] != PREVIEW_LIMITATION_0:
        raise SystemExit(f"{PREVIEW}: limitation 0 is not the Euler-Bernoulli sentence this derivation replaces")
    preview["semantic_contract_id"] = identity
    preview["formulation_profile_id"] = profile
    preview["supported_profile_limitations"][0] = limitation_0
    return dumps(preview)
""")
edit(P, """PINNED = (PRESSURE, RETAINED, TABLE % "preview_physics_2", TIMOSHENKO, TABLE % "preview_physics_retained_1", SCHEMA,
""", """PINNED = (PRESSURE, RETAINED, TABLE % "preview_physics_2", TIMOSHENKO, CURVED, TABLE % "preview_physics_retained_1", SCHEMA,
""")

# ---------------------------------------------------------------- desktop (PF identity consumers)
P = "apps/desktop/src/features/results/numericalResultQuality.ts"
edit_s(P, """export const PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID = "openpipestress.result_semantics/0.3.0/preview-physics-timoshenko-1";
/** The `preview_physics` route's identities, Euler–Bernoulli first. */
export const PREVIEW_PHYSICS_CONTRACT_IDS: readonly string[] = Object.freeze([PREVIEW_PHYSICS_CONTRACT_ID, PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID]);
""", """export const PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID = "openpipestress.result_semantics/0.3.0/preview-physics-timoshenko-1";
/** DEL-04-01 Design `curved-bend-transverse-shear.md` §5: the pressure-free curved-shear identity
 * (`model_settings.shear_model` `timoshenko_annulus_energy_curved_v2`), derived as the Timoshenko one. */
export const PREVIEW_PHYSICS_CURVED_SHEAR_CONTRACT_ID = "@PF_ID@";
/** The `preview_physics` route's identities, Euler–Bernoulli first. */
export const PREVIEW_PHYSICS_CONTRACT_IDS: readonly string[] = Object.freeze([
  PREVIEW_PHYSICS_CONTRACT_ID,
  PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID,
  PREVIEW_PHYSICS_CURVED_SHEAR_CONTRACT_ID,
]);
""")
P = "apps/desktop/src/features/results/knownSemanticLimitations.ts"
edit(P, """import { PREVIEW_PHYSICS_CONTRACT_ID, PREVIEW_PHYSICS_RETAINED_CONTRACT_ID, PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID, PRESSURE_CONTRACT_ID""",
     """import { PREVIEW_PHYSICS_CONTRACT_ID, PREVIEW_PHYSICS_CURVED_SHEAR_CONTRACT_ID, PREVIEW_PHYSICS_RETAINED_CONTRACT_ID, PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID, PRESSURE_CONTRACT_ID""")
edit(P, """  PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID,
  PRESSURE_SOURCE_CONTRACT_ID,
  PRESSURE_CONTRACT_ID,
  PREVIEW_PHYSICS_RETAINED_CONTRACT_ID,
]);
""", """  PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID,
  // DEL-04-01 curved-bend-transverse-shear.md §5: the pressure-free curved-shear identity (Rust's set too).
  PREVIEW_PHYSICS_CURVED_SHEAR_CONTRACT_ID,
  PRESSURE_SOURCE_CONTRACT_ID,
  PRESSURE_CONTRACT_ID,
  PREVIEW_PHYSICS_RETAINED_CONTRACT_ID,
]);
""")
P = "apps/desktop/src/features/report/pfTimoshenkoReport.ts"
edit(P, """ * top-level `pf_timoshenko` section: the identity and the Export Result JSON admission
 * binding, which the native render binds to the delivered result before it renders. The
""", """ * top-level `pf_timoshenko` section: the identity and the Export Result JSON admission
 * binding, which the native render binds to the delivered result before it renders. A
 * `preview-physics-curved-shear-1` result (DEL-04-01 Design `curved-bend-transverse-shear.md`
 * §5) takes the same projection with its own limitation 0 and no arc disclosure. The
""")
edit(P, """import { PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID } from "../results/numericalResultQuality";
import timoshenkoTable from "../../../../../fixtures/results/semantic_contract_v0_3_preview_physics_timoshenko_1.json";
""", """import { PREVIEW_PHYSICS_CURVED_SHEAR_CONTRACT_ID, PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID } from "../results/numericalResultQuality";
import timoshenkoTable from "../../../../../fixtures/results/semantic_contract_v0_3_preview_physics_timoshenko_1.json";
import curvedShearTable from "../../../../../fixtures/results/semantic_contract_v0_3_preview_physics_curved_shear_1.json";
""")
edit(P, """/** Whether `result` names the pressure-free Timoshenko identity. */
export function isPfTimoshenkoReportSource(result: MechanicsResult | null | undefined): boolean {
  return result?.producer?.semantic_contract_id === PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID;
}
""", """/** Each pressure-free Timoshenko identity's limitation 0, by identity. */
const LIMITATION_0: Readonly<Record<string, string>> = Object.freeze({
  [PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID]: timoshenkoTable.supported_profile_limitations[0],
  [PREVIEW_PHYSICS_CURVED_SHEAR_CONTRACT_ID]: curvedShearTable.supported_profile_limitations[0],
});

/** Whether `result` names a pressure-free Timoshenko identity (v1, or the curved-shear v2). */
export function isPfTimoshenkoReportSource(result: MechanicsResult | null | undefined): boolean {
  const identity = result?.producer?.semantic_contract_id;
  return identity === PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID || identity === PREVIEW_PHYSICS_CURVED_SHEAR_CONTRACT_ID;
}
""")
edit(P, """  const published = result.formulation_basis?.limitations?.[0];
  const stated = timoshenkoTable.supported_profile_limitations[0];
  if (!isPfTimoshenkoReportSource(result) || published !== stated) {
    throw new Error(`${REPORT_PF_TIMOSHENKO_SECTIONS_MISMATCH}: the source's limitation 0 is not the identity's`);
  }
  return {
    limitation_id: `limitation:${run}:pf-timoshenko:1`,
    source: { ref_type: "semantic_contract", ref_id: PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID },
""", """  const identity = result.producer?.semantic_contract_id ?? "";
  const published = result.formulation_basis?.limitations?.[0];
  const stated = isPfTimoshenkoReportSource(result) ? LIMITATION_0[identity] : undefined;
  if (stated === undefined || published !== stated) {
    throw new Error(`${REPORT_PF_TIMOSHENKO_SECTIONS_MISMATCH}: the source's limitation 0 is not the identity's`);
  }
  return {
    limitation_id: `limitation:${run}:pf-timoshenko:1`,
    source: { ref_type: "semantic_contract", ref_id: identity },
""")
edit(P, """/** The rendered report's section: the identity and its admission binding. */
export function pfTimoshenkoSection(binding: Readonly<ExactAdmissionBinding>) {
  return { semantic_contract_id: PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID, admission: { ...binding } };
}
""", """/** The rendered report's section: the identity (the admitted result's, which the binding
 * states) and its admission binding. */
export function pfTimoshenkoSection(binding: Readonly<ExactAdmissionBinding>) {
  return { semantic_contract_id: binding.semantic_contract_id, admission: { ...binding } };
}
""")
P = "apps/desktop/src/features/report/reportPackageRequest.ts"
edit(P, """import { PRESSURE_CONTRACT_ID, PRESSURE_SOURCE_CONTRACT_ID, PREVIEW_PHYSICS_CONTRACT_ID, PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID } from "../r""",
     """import { PRESSURE_CONTRACT_ID, PRESSURE_SOURCE_CONTRACT_ID, PREVIEW_PHYSICS_CONTRACT_ID, PREVIEW_PHYSICS_CURVED_SHEAR_CONTRACT_ID, PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID } from "../r""")
edit(P, """  // DEL-05-03 pf-timoshenko.md §4: the pressure-free Timoshenko identity, with its projection.
  PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID,
  PRESSURE_CONTRACT_ID,
""", """  // DEL-05-03 pf-timoshenko.md §4: the pressure-free Timoshenko identity, with its projection.
  PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID,
  // DEL-04-01 curved-bend-transverse-shear.md §5: the curved-shear identity, with the same projection.
  PREVIEW_PHYSICS_CURVED_SHEAR_CONTRACT_ID,
  PRESSURE_CONTRACT_ID,
""")
P = "apps/desktop/src/features/stress-neutral/StressNeutralExportPanel.tsx"
edit(P, """import { PREVIEW_PHYSICS_CONTRACT_ID, PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID, RETAINED_PRECISION_DOWNGRADE_FORBIDDEN,""",
     """import { PREVIEW_PHYSICS_CONTRACT_ID, PREVIEW_PHYSICS_CURVED_SHEAR_CONTRACT_ID, PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID, RETAINED_PRECISION_DOWNGRADE_FORBIDDEN,""")
edit(P, """  [PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID]: "semantic_contract_v0_3_preview_physics_timoshenko_1.json",
});
""", """  [PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID]: "semantic_contract_v0_3_preview_physics_timoshenko_1.json",
  // DEL-04-01 curved-bend-transverse-shear.md §5.
  [PREVIEW_PHYSICS_CURVED_SHEAR_CONTRACT_ID]: "semantic_contract_v0_3_preview_physics_curved_shear_1.json",
});
""")
P = "apps/desktop/src/services/analysisRunCompatibility.ts"
edit(P, """import previewPhysicsTimoshenkoTable from "../../../../fixtures/results/semantic_contract_v0_3_preview_physics_timoshenko_1.json";
""", """import previewPhysicsTimoshenkoTable from "../../../../fixtures/results/semantic_contract_v0_3_preview_physics_timoshenko_1.json";
import previewPhysicsCurvedShearTable from "../../../../fixtures/results/semantic_contract_v0_3_preview_physics_curved_shear_1.json";
""")
edit(P, """  [previewPhysicsTimoshenkoTable.semantic_contract_id]: previewPhysicsTimoshenkoTable,
""", """  [previewPhysicsTimoshenkoTable.semantic_contract_id]: previewPhysicsTimoshenkoTable,
  // DEL-04-01 curved-bend-transverse-shear.md §5: the curved-shear identity's own table.
  [previewPhysicsCurvedShearTable.semantic_contract_id]: previewPhysicsCurvedShearTable,
""")
P = "apps/desktop/src-tauri/src/analysis_run_v03_build.rs"
edit(P, """        sc::preview_physics_timoshenko_contract(),
        sc::preview_physics_retained_contract(),
""", """        sc::preview_physics_timoshenko_contract(),
        sc::preview_physics_curved_shear_contract(),
        sc::preview_physics_retained_contract(),
""")
P = "apps/desktop/src/test-support/contractBindings.ts"
edit(P, """export const PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_SHA256 = sha256("semantic_contract_v0_3_preview_physics_timoshenko_1.json");
""", """export const PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_SHA256 = sha256("semantic_contract_v0_3_preview_physics_timoshenko_1.json");
/** DEL-04-01 curved-bend-transverse-shear.md §5: the pressure-free curved-shear identity's table. */
export const PREVIEW_PHYSICS_CURVED_SHEAR_CONTRACT_SHA256 = sha256("semantic_contract_v0_3_preview_physics_curved_shear_1.json");
""")

# ---------------------------------------------------------------- Python record builder, package and verifiers
P = "core/analysis_runs/compatibility.py"
edit_s(P, """_PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_PATH = _RESULTS_DIR / "semantic_contract_v0_3_preview_physics_timoshenko_1.json"
""", """_PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_PATH = _RESULTS_DIR / "semantic_contract_v0_3_preview_physics_timoshenko_1.json"
# DEL-04-01 Design curved-bend-transverse-shear.md §5: the pressure-free curved-shear identity, a third on the route.
PREVIEW_PHYSICS_CURVED_SHEAR_CONTRACT_ID = "@PF_ID@"
PREVIEW_PHYSICS_CURVED_SHEAR_CONTRACT_SHA256 = "@PF_SHA@"
_PREVIEW_PHYSICS_CURVED_SHEAR_CONTRACT_PATH = _RESULTS_DIR / "semantic_contract_v0_3_preview_physics_curved_shear_1.json"
""")
edit(P, """FRESH_CONTRACT_IDS = frozenset({PREVIEW_PHYSICS_CONTRACT_ID, PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID, PRESSURE_SOURCE_CONTRACT_ID,
                                # T4-U2: pressure-2 (`3.0.0/exact_pressure_v3`).
                                PRESSURE_CONTRACT_ID,
                                # U6b (D-U6-6): membership is not standing.
                                PREVIEW_PHYSICS_RETAINED_CONTRACT_ID})
# Every current-record identity""", """FRESH_CONTRACT_IDS = frozenset({PREVIEW_PHYSICS_CONTRACT_ID, PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID,
                                PREVIEW_PHYSICS_CURVED_SHEAR_CONTRACT_ID, PRESSURE_SOURCE_CONTRACT_ID,
                                # T4-U2: pressure-2 (`3.0.0/exact_pressure_v3`).
                                PRESSURE_CONTRACT_ID,
                                # U6b (D-U6-6): membership is not standing.
                                PREVIEW_PHYSICS_RETAINED_CONTRACT_ID})
# Every current-record identity""")
edit(P, """                                         PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID,
                                         PREVIEW_PHYSICS_RETAINED_CONTRACT_ID, PHYSICS_RETAINED_CONTRACT_ID})
""", """                                         PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID, PREVIEW_PHYSICS_CURVED_SHEAR_CONTRACT_ID,
                                         PREVIEW_PHYSICS_RETAINED_CONTRACT_ID, PHYSICS_RETAINED_CONTRACT_ID})
""")
edit(P, """           PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID: (PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_SHA256, _PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_PATH),
           PRESSURE_CONTRACT_ID""", """           PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID: (PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_SHA256, _PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_PATH),
           PREVIEW_PHYSICS_CURVED_SHEAR_CONTRACT_ID: (PREVIEW_PHYSICS_CURVED_SHEAR_CONTRACT_SHA256, _PREVIEW_PHYSICS_CURVED_SHEAR_CONTRACT_PATH),
           PRESSURE_CONTRACT_ID""")
P = "core/handoff/stress_neutral/package.py"
edit(P, """    PRESSURE_SOURCE_CONTRACT_ID, PREVIEW_PHYSICS_CONTRACT_ID, PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID,
)
""", """    PRESSURE_SOURCE_CONTRACT_ID, PREVIEW_PHYSICS_CONTRACT_ID, PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID,
    PREVIEW_PHYSICS_CURVED_SHEAR_CONTRACT_ID,
)
""")
edit(P, """SUPPORTED_METHODS = {PRESSURE_CONTRACT_ID, PRESSURE_SOURCE_CONTRACT_ID, PREVIEW_PHYSICS_CONTRACT_ID,
                     PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID}
""", """SUPPORTED_METHODS = {PRESSURE_CONTRACT_ID, PRESSURE_SOURCE_CONTRACT_ID, PREVIEW_PHYSICS_CONTRACT_ID,
                     PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID, PREVIEW_PHYSICS_CURVED_SHEAR_CONTRACT_ID}
""")
edit(P, """# identity (DEL-05-03 pf-timoshenko.md §4) carries the same statement under its own table.
PREVIEW_METHODS = {PREVIEW_PHYSICS_CONTRACT_ID, PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID}
""", """# identity (DEL-05-03 pf-timoshenko.md §4) and the curved-shear identity (DEL-04-01
# curved-bend-transverse-shear.md §5) carry the same statement under their own tables.
PREVIEW_METHODS = {PREVIEW_PHYSICS_CONTRACT_ID, PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID, PREVIEW_PHYSICS_CURVED_SHEAR_CONTRACT_ID}
""")
P = "tests/verifiers/compatibility.py"
edit_s(P, """PREVIEW_PHYSICS_TIMOSHENKO_PROFILE = "product_preview_mechanics_timoshenko_v1"
# The preview-physics route's identities: id -> (sha256, table path, profile).
PREVIEW_PHYSICS_CONTRACTS = {
    PREVIEW_PHYSICS_CONTRACT_ID: (PREVIEW_PHYSICS_CONTRACT_SHA256, _PREVIEW_PHYSICS_CONTRACT_PATH, PREVIEW_PHYSICS_PROFILE),
    PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID: (PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_SHA256, _PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_PATH,
                                             PREVIEW_PHYSICS_TIMOSHENKO_PROFILE),
}
""", """PREVIEW_PHYSICS_TIMOSHENKO_PROFILE = "product_preview_mechanics_timoshenko_v1"
# DEL-04-01 Design curved-bend-transverse-shear.md §5: the pressure-free curved-shear identity, derived the same way.
PREVIEW_PHYSICS_CURVED_SHEAR_CONTRACT_ID = "@PF_ID@"
PREVIEW_PHYSICS_CURVED_SHEAR_CONTRACT_SHA256 = "@PF_SHA@"
_PREVIEW_PHYSICS_CURVED_SHEAR_CONTRACT_PATH = _RESULTS_DIR / "semantic_contract_v0_3_preview_physics_curved_shear_1.json"
PREVIEW_PHYSICS_CURVED_SHEAR_PROFILE = "@PF_PROFILE@"
# The preview-physics route's identities: id -> (sha256, table path, profile).
PREVIEW_PHYSICS_CONTRACTS = {
    PREVIEW_PHYSICS_CONTRACT_ID: (PREVIEW_PHYSICS_CONTRACT_SHA256, _PREVIEW_PHYSICS_CONTRACT_PATH, PREVIEW_PHYSICS_PROFILE),
    PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID: (PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_SHA256, _PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_PATH,
                                             PREVIEW_PHYSICS_TIMOSHENKO_PROFILE),
    PREVIEW_PHYSICS_CURVED_SHEAR_CONTRACT_ID: (PREVIEW_PHYSICS_CURVED_SHEAR_CONTRACT_SHA256, _PREVIEW_PHYSICS_CURVED_SHEAR_CONTRACT_PATH,
                                               PREVIEW_PHYSICS_CURVED_SHEAR_PROFILE),
}
""")
edit(P, """FRESH_CONTRACT_IDS = frozenset({PREVIEW_PHYSICS_CONTRACT_ID, PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID, PRESSURE_SOURCE_CONTRACT_ID,
""", """FRESH_CONTRACT_IDS = frozenset({PREVIEW_PHYSICS_CONTRACT_ID, PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID,
                                PREVIEW_PHYSICS_CURVED_SHEAR_CONTRACT_ID, PRESSURE_SOURCE_CONTRACT_ID,
""")
P = "tests/verifiers/preview_physics_evidence.py"
edit_s(P, """TIMOSHENKO_LIMITATIONS = list(json.loads(TIMOSHENKO_TABLE_PATH.read_text(encoding="utf-8"))["supported_profile_limitations"])
IDENTITIES = {CONTRACT_ID: (PROFILE, LIMITATIONS), TIMOSHENKO_CONTRACT_ID: (TIMOSHENKO_PROFILE, TIMOSHENKO_LIMITATIONS)}
""", """TIMOSHENKO_LIMITATIONS = list(json.loads(TIMOSHENKO_TABLE_PATH.read_text(encoding="utf-8"))["supported_profile_limitations"])
# DEL-04-01 Design curved-bend-transverse-shear.md §5: the curved-shear identity, derived the same way.
CURVED_SHEAR_CONTRACT_ID = "@PF_ID@"
CURVED_SHEAR_PROFILE = "@PF_PROFILE@"
CURVED_SHEAR_TABLE_PATH = ROOT / "fixtures/results/semantic_contract_v0_3_preview_physics_curved_shear_1.json"
CURVED_SHEAR_LIMITATIONS = list(json.loads(CURVED_SHEAR_TABLE_PATH.read_text(encoding="utf-8"))["supported_profile_limitations"])
IDENTITIES = {CONTRACT_ID: (PROFILE, LIMITATIONS), TIMOSHENKO_CONTRACT_ID: (TIMOSHENKO_PROFILE, TIMOSHENKO_LIMITATIONS),
              CURVED_SHEAR_CONTRACT_ID: (CURVED_SHEAR_PROFILE, CURVED_SHEAR_LIMITATIONS)}
""")
P = "tests/verifiers/reader_verdict.py"
edit(P, """                    c.PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID: "preview_physics",
""", """                    c.PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID: "preview_physics",
                    c.PREVIEW_PHYSICS_CURVED_SHEAR_CONTRACT_ID: "preview_physics",
""")
edit(P, """    selects preview-physics-2, `timoshenko_annulus_energy_v1` the Timoshenko identity; a crossed pair is
""", """    selects preview-physics-2, `timoshenko_annulus_energy_v1` the Timoshenko identity,
    `timoshenko_annulus_energy_curved_v2` the curved-shear identity; a crossed pair is
""")
edit(P, """    that is not an object, `null` included (MODEL-SLOT-SHAPE), and a `shear_model` that is not one of the
    two names, `null` and non-strings included (MODEL-SETTING-SHEAR-MODEL-UNKNOWN).\"\"\"
""", """    that is not an object, `null` included (MODEL-SLOT-SHAPE), and a `shear_model` that is not one of the
    three names, `null` and non-strings included (MODEL-SETTING-SHEAR-MODEL-UNKNOWN).\"\"\"
""")
edit(P, """        expected = c.PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID
    else:
        raise Refused(f"{SHEAR_MODEL_IDENTITY}: MODEL-SETTING-SHEAR-MODEL-UNKNOWN: model_settings.shear_model must be "
                      "`euler_bernoulli` or `timoshenko_annulus_energy_v1`")
""", """        expected = c.PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID
    elif selected == "timoshenko_annulus_energy_curved_v2":
        expected = c.PREVIEW_PHYSICS_CURVED_SHEAR_CONTRACT_ID
    else:
        raise Refused(f"{SHEAR_MODEL_IDENTITY}: MODEL-SETTING-SHEAR-MODEL-UNKNOWN: model_settings.shear_model must be "
                      "`euler_bernoulli`, `timoshenko_annulus_energy_v1` or `timoshenko_annulus_energy_curved_v2`")
""")


# ---------------------------------------------------------------- applier doc, desktop control
P = "core/model_operations/operation_applier/src/model_settings_authoring.rs"
edit(P, """//!   product_physics' `model_settings::shear_model_refusal`, so a value other
//!   than the two names (`MODEL-SETTING-SHEAR-MODEL-UNKNOWN`) and a document with
//!   `pressure_contract` (`-WITH-PRESSURE-CONTRACT`) are refused by name; both
//!   names are admitted on a pressure-free document (DEL-05-03 Design
//!   `pf-timoshenko.md` §1). An explicit""", """//!   product_physics' `model_settings::shear_model_refusal`, so a value other
//!   than the three names (`MODEL-SETTING-SHEAR-MODEL-UNKNOWN`) and a document with
//!   `pressure_contract` (`-WITH-PRESSURE-CONTRACT`) are refused by name; every
//!   name is admitted on a pressure-free document (DEL-05-03 Design
//!   `pf-timoshenko.md` §1; DEL-04-01 Design `curved-bend-transverse-shear.md` §5). An explicit""")
P = "apps/desktop/src/features/model-settings/ModelSettingsSection.tsx"
edit(P, """        <legend>Straight-member shear deformation</legend>
""", """        <legend>Shear deformation</legend>
""")
edit(P, """        <div role="radiogroup" aria-label="Straight-member shear deformation">
""", """        <div role="radiogroup" aria-label="Shear deformation">
""")
P = "apps/desktop/src/features/model-settings/ModelSettingsShearControl.test.tsx"
edit(P, """// The Model settings "Straight-member shear deformation" control (DEL-05-03 Design
// pf-timoshenko.md §1, §5 "Desktop"): Euler–Bernoulli (the default) or Timoshenko
// (energy-matched annular factor). Choosing either writes""", """// The Model settings "Shear deformation" control (DEL-05-03 Design
// pf-timoshenko.md §1, §5 "Desktop"; DEL-04-01 Design curved-bend-transverse-shear.md §5):
// Euler–Bernoulli (the default), Timoshenko (energy-matched annular factor), or Timoshenko for
// straight members and bends. Choosing any writes""")
edit(P, """describe("model settings: straight-member shear deformation", () => {
  it("shows the default, states the rules, and offers both choices with nothing written", () => {
    const model = demo();
    render(<ModelSettingsSection model={model} onQueueIntent={() => undefined} />);
    expect(byId("model-settings-shear-model")).toHaveTextContent("Straight-member shear deformation");
""", """describe("model settings: shear deformation", () => {
  it("shows the default, states the rules, and offers the three choices with nothing written", () => {
    const model = demo();
    render(<ModelSettingsSection model={model} onQueueIntent={() => undefined} />);
    expect(byId("model-settings-shear-model")).toHaveTextContent("Shear deformation");
    expect(screen.getByRole("radiogroup", { name: "Shear deformation" })).toBeInTheDocument();
""")
edit(P, """    expect(screen.getByLabelText("Timoshenko (energy-matched annular factor)")).toBeInTheDocument();
    expect(screen.getByLabelText("Euler–Bernoulli (the default)")).toBeInTheDocument();
""", """    expect(screen.getByLabelText("Timoshenko (energy-matched annular factor)")).toBeInTheDocument();
    expect(screen.getByLabelText("Euler–Bernoulli (the default)")).toBeInTheDocument();
    // DEL-04-01 curved-bend-transverse-shear.md §5: the explicit v2 choice, never a default.
    expect(byId("model-settings-shear-model-timoshenko_annulus_energy_curved_v2")).not.toBeChecked();
    expect(byId("model-settings-shear-model-timoshenko_annulus_energy_curved_v2")).not.toBeDisabled();
    expect(screen.getByLabelText("Timoshenko, straight members and bends (energy-matched annular factor)")).toBeInTheDocument();
    expect(note).toContain("unless you choose straight members and bends");
    expect(note).toContain("Modal analysis refuses either Timoshenko choice.");
""")
P = "apps/desktop/src/features/model-settings/modelSettingsShearModel.test.ts"
edit(P, """  expect(await outcome(model, "timoshenko_annulus_energy_v1")).toEqual([]);
""", """  expect(await outcome(model, "timoshenko_annulus_energy_v1")).toEqual([]);
  expect(await outcome(model, "timoshenko_annulus_energy_curved_v2")).toEqual([]);
""")
edit(P, """it("the document validator admits both names and refuses null and other values", () => {
  for (const name of ["euler_bernoulli", "timoshenko_annulus_energy_v1"]) {
""", """it("the document validator admits the three names and refuses null and other values", () => {
  for (const name of ["euler_bernoulli", "timoshenko_annulus_energy_v1", "timoshenko_annulus_energy_curved_v2"]) {
""")

# ---------------------------------------------------------------- modal verifier twin
P = "tests/verifiers/modal_model_mass.py"
edit(P, """TIMOSHENKO = "timoshenko_annulus_energy_v1"
""", """TIMOSHENKO = "timoshenko_annulus_energy_v1"
# DEL-04-01 curved-bend-transverse-shear.md §5: refused by the producer under the same code.
TIMOSHENKO_CURVED = "timoshenko_annulus_energy_curved_v2"
""")
edit(P, """        if contract.get("shear_model") == TIMOSHENKO:
            raise RefusedDomain("Timoshenko straights are refused by name (MODAL_TIMOSHENKO_MASS_UNSUPPORTED)")
""", """        if contract.get("shear_model") in (TIMOSHENKO, TIMOSHENKO_CURVED):
            raise RefusedDomain("Timoshenko straights are refused by name (MODAL_TIMOSHENKO_MASS_UNSUPPORTED)")
""")
edit(P, """        if isinstance(settings, dict) and settings.get("shear_model") == TIMOSHENKO:
""", """        if isinstance(settings, dict) and settings.get("shear_model") in (TIMOSHENKO, TIMOSHENKO_CURVED):
""")

# ---------------------------------------------------------------- product tests
P = "core/product_physics/src/b7b_admission_tests.rs"
edit(P, """    raw["model"]["pressure_contract"]["shear_model"] = json!("timoshenko_annulus_energy_v1");
    assert!(!public_codes(raw).iter().any(|code| code.starts_with("EXACT_PRESSURE_SHEAR_MODEL")));
""", """    raw["model"]["pressure_contract"]["shear_model"] = json!("timoshenko_annulus_energy_v1");
    assert!(!public_codes(raw).iter().any(|code| code.starts_with("EXACT_PRESSURE_SHEAR_MODEL")));
    // DEL-04-01 curved-bend-transverse-shear.md §5 (B1): timoshenko_annulus_energy_curved_v2 is in the
    // 3.0.0 vocabulary and refused by name on this route until its exact identity is admitted; it is
    // neither an unknown value nor solved as v1.
    let mut raw = current_joint_request();
    raw["model"]["pressure_contract"]["shear_model"] = json!("timoshenko_annulus_energy_curved_v2");
    let codes = public_codes(raw);
    assert!(codes.iter().any(|code| code == "EXACT_PRESSURE_SHEAR_MODEL_NOT_ADMITTED"), "{codes:?}");
    assert!(!codes.iter().any(|code| code == "PRESSURE_CONTRACT_UNSUPPORTED"), "{codes:?}");
""")
P = "core/product_physics/src/modal/tests.rs"
edit(P, """    let found = prepare_modal(&modal(with(timoshenko, json!({"mode_count": 1})))).err().unwrap();
    assert_eq!(found.last().unwrap().code, MODAL_TIMOSHENKO_MASS_UNSUPPORTED, "{found:#?}");
}
""", """    let found = prepare_modal(&modal(with(timoshenko, json!({"mode_count": 1})))).err().unwrap();
    assert_eq!(found.last().unwrap().code, MODAL_TIMOSHENKO_MASS_UNSUPPORTED, "{found:#?}");
    assert!(found.last().unwrap().message.contains("(timoshenko_annulus_energy_v1)"), "{found:#?}");
    // DEL-04-01 curved-bend-transverse-shear.md §5: v2 under the same code, naming its value.
    let mut curved = pf_request(false);
    curved["model"]["model_settings"] = json!({"shear_model": "timoshenko_annulus_energy_curved_v2"});
    curved["model"]["materials"][0]["poisson_ratio"] = q(0.25, "1");
    let found = prepare_modal(&modal(with(curved, json!({"mode_count": 1})))).err().unwrap();
    assert_eq!(found.last().unwrap().code, MODAL_TIMOSHENKO_MASS_UNSUPPORTED, "{found:#?}");
    assert!(found.last().unwrap().message.contains("realized arcs (timoshenko_annulus_energy_curved_v2)"), "{found:#?}");
}
""")
P = "core/product_physics/tests/model_settings_shear_model.rs"
edit(P, """#[test]
fn unknown_and_null_values_are_refused_by_name() {
""", """/// DEL-04-01 Design `curved-bend-transverse-shear.md` §5: the explicit v2 value is admitted on a
/// pressure-free document under its own identity and profile.
#[test]
fn the_curved_value_is_admitted_on_a_pressure_free_document() {
    let demo = json!({"model": serde_json::from_str::<Value>(DEMO).unwrap(), "materials": []});
    let request = with_shear_model(&demo, json!("timoshenko_annulus_energy_curved_v2"));
    for mode in MODES {
        let envelope = solve(&request, mode);
        assert_eq!(envelope["status"]["mechanics"], json!("MECHANICS_SOLVED"), "{mode:?}: {}", envelope["diagnostics"]);
        assert_eq!(envelope["producer"]["semantic_contract_id"], json!("openpipestress.result_semantics/0.3.0/preview-physics-curved-shear-1"));
        assert_eq!(envelope["formulation_basis"]["profile_id"], json!("product_preview_mechanics_curved_shear_v1"));
        assert!(!envelope["diagnostics"].as_array().unwrap().iter().any(|d| d["code"].as_str().unwrap().starts_with("MODEL-SETTING-SHEAR-MODEL")));
        assert_eq!(serde_json::to_string(&typed(&request, mode)).unwrap(), serde_json::to_string(&envelope).unwrap(), "{mode:?}: typed entry");
    }
}

#[test]
fn unknown_and_null_values_are_refused_by_name() {
""")

# ---------------------------------------------------------------- MBF tests (no MBF code change)
P = "tests/test_caepipe_mbf_si_writer.py"
edit(P, """def test_pressure_refusals_and_omissions():
""", """def test_the_curved_shear_model_is_refused_on_the_contract_and_omitted_in_model_settings():
    \"\"\"DEL-04-01 curved-bend-transverse-shear.md §5: timoshenko_annulus_energy_curved_v2 is in the 3.0.0 vocabulary but
    not one of the writer's shear models, so an exact document selecting it is refused by name and never written under
    another model; on a pressure-free document model_settings stays the omitted member it is for v1.\"\"\"
    assert_refused(render_caepipe_mbf_si(pressurized_l(shear="timoshenko_annulus_energy_curved_v2"), title="P"),
                   "MBF-SI-PRESSURE-UNSUPPORTED", "admitted shear_model")
    for shear in ("timoshenko_annulus_energy_v1", "timoshenko_annulus_energy_curved_v2"):
        model = cantilever()
        model["model_settings"] = {"shear_model": shear}
        result = render_caepipe_mbf_si(model, title="P")
        assert result.status == "written", result.diagnostics
        assert [item["code"] for item in result.loss_report if item["entity"] == "model.model_settings"] == [
            "MBF-SI-MODEL-SETTINGS-NOT-EXPORTED"], shear
        assert result.text == render_caepipe_mbf_si(cantilever(), title="P").text, shear


def test_pressure_refusals_and_omissions():
""")
P = "tests/test_caepipe_mbf_round_trip.py"
edit(P, """    for bad in ("timoshenko", "", 1, True, ["euler_bernoulli"]):
""", """    # DEL-04-01 curved-bend-transverse-shear.md §5: the import never produces timoshenko_annulus_energy_curved_v2.
    for bad in ("timoshenko", "", 1, True, ["euler_bernoulli"], "timoshenko_annulus_energy_curved_v2"):
""")

# ---------------------------------------------------------------- generator test
P = "tests/test_semantic_contract_tables.py"
edit(P, """    assert branch["properties"]["formulation_basis"]["properties"]["profile_id"]["const"] == derived["formulation_profile_id"]
    assert branch["properties"]["formulation_basis"]["properties"]["limitations"]["const"] == derived["supported_profile_limitations"]
""", """    assert branch["properties"]["formulation_basis"]["properties"]["profile_id"]["const"] == derived["formulation_profile_id"]
    assert branch["properties"]["formulation_basis"]["properties"]["limitations"]["const"] == derived["supported_profile_limitations"]


def test_curved_shear_table_is_preview_physics_2_with_its_identity_profile_and_limitation_0():
    \"\"\"DEL-04-01 Design curved-bend-transverse-shear.md §5: the curved-shear table is derived as the Timoshenko one,
    differing from preview-physics-2 in exactly identity, profile and limitation 0; `derive --check` holds it current
    and the results schema's branch states the same array.\"\"\"
    import json
    tool = _tool()
    preview = json.loads((PROJECT / tool.PREVIEW).read_bytes())
    derived = json.loads((PROJECT / tool.CURVED).read_bytes())
    assert list(derived) == list(preview)
    assert {k for k in preview if preview[k] != derived[k]} == {
        "semantic_contract_id", "formulation_profile_id", "supported_profile_limitations"}
    assert derived["semantic_contract_id"] == "openpipestress.result_semantics/0.3.0/preview-physics-curved-shear-1"
    assert derived["formulation_profile_id"] == "product_preview_mechanics_curved_shear_v1"
    assert derived["supported_profile_limitations"] == [tool.CURVED_LIMITATION_0] + preview["supported_profile_limitations"][1:]
    assert (PROJECT / tool.CURVED).read_bytes() == tool.preview_variant(tool.CURVED_ID, tool.CURVED_PROFILE, tool.CURVED_LIMITATION_0)
    assert tool.derive(check=True) == 0
    assert tool.apply_u4b(check=True) == 0
    schema = json.loads((PROJECT / "schemas/results.v0.3.schema.yaml").read_text(encoding="utf-8"))
    branch = next(b for b in schema["$defs"]["ResultEnvelope"]["oneOf"]
                  if b.get("properties", {}).get("producer", {}).get("properties", {}).get("semantic_contract_id", {}).get("const")
                  == derived["semantic_contract_id"])
    assert branch["properties"]["formulation_basis"]["properties"]["profile_id"]["const"] == derived["formulation_profile_id"]
    assert branch["properties"]["formulation_basis"]["properties"]["limitations"]["const"] == derived["supported_profile_limitations"]
""")

# ---------------------------------------------------------------- inventory guard
P = "tests/test_preview_identity_inventory.py"
edit(P, """identity (its literal or symbol) in the same file, and names the round-trip test that fails if the gate refuses it.
\"\"\"
""", """identity (its literal or symbol) in the same file, and names the round-trip test that fails if the gate refuses it.

The pressure-free curved-shear identity (`preview-physics-curved-shear-1`; DEL-04-01 Design
curved-bend-transverse-shear.md §5) is a third identity at the same sites: each "admits both identities" entry also
names it (CURVED_MARKERS), or states why it has none (CURVED_NONE).
\"\"\"
""")
edit(P, """def _is_scanned(path: str) -> bool:
""", """# Each "admits both" entry's curved-shear marker (its literal or symbol in the same file). Identity-agnostic sites name
# the same selector their Timoshenko marker names; it keys by the source's identity.
CURVED_MARKERS = {
    "core/reporting/result_export/src/semantic_contract.rs": "PREVIEW_PHYSICS_CURVED_SHEAR_ID",
    "core/analysis_runs/compatibility.py": "PREVIEW_PHYSICS_CURVED_SHEAR_CONTRACT_ID",
    "tests/verifiers/compatibility.py": "PREVIEW_PHYSICS_CURVED_SHEAR_CONTRACT_ID",
    "tests/verifiers/preview_physics_evidence.py": "CURVED_SHEAR_CONTRACT_ID",
    "schemas/results.v0.3.schema.yaml": "preview-physics-curved-shear-1",
    "schemas/analysis_run.v0.3.schema.json": "preview-physics-curved-shear-1",
    "schemas/stress_neutral_export.v0.3.schema.json": "preview-physics-curved-shear-1",
    "tools/serialization/semantic_contract_tables.py": "CURVED_ID",
    "apps/desktop/src/features/results/numericalResultQuality.ts": "PREVIEW_PHYSICS_CURVED_SHEAR_CONTRACT_ID",
    "apps/desktop/src/features/stress-neutral/StressNeutralExportPanel.tsx": "PREVIEW_PHYSICS_CURVED_SHEAR_CONTRACT_ID",
    "apps/desktop/src/services/analysisRunCompatibility.ts": "previewPhysicsCurvedShearTable",
    "core/product_physics/src/pf_timoshenko.rs": "preview-physics-curved-shear-1",
    "core/product_physics/src/lib.rs": "pf_timoshenko::selects_curved",
    "core/reporting/result_export/src/reader_verdict.rs": "PREVIEW_PHYSICS_CURVED_SHEAR_ID",
    "core/reporting/result_export/src/preview_physics_evidence.rs": "preview_physics_identity_of",
    "core/reporting/result_export/src/u4b_evidence.rs": "preview_physics_identity_of",
    "core/reporting/result_export/src/derivative.rs": "PREVIEW_PHYSICS_CURVED_SHEAR_ID",
    "core/handoff/stress_neutral/package.py": "PREVIEW_PHYSICS_CURVED_SHEAR_CONTRACT_ID",
    "tests/verifiers/reader_verdict.py": "PREVIEW_PHYSICS_CURVED_SHEAR_CONTRACT_ID",
    "core/reporting/report_package/src/document_form.rs": "PREVIEW_PHYSICS_CURVED_SHEAR_ID",
    "core/reporting/report_package/src/bindings.rs": "pf_timoshenko::check_sections",
    "apps/desktop/src-tauri/src/report_render_exact.rs": "PF_TIMOSHENKO_SECTION",
    "apps/desktop/src/features/results/knownSemanticLimitations.ts": "PREVIEW_PHYSICS_CURVED_SHEAR_CONTRACT_ID",
    "apps/desktop/src/features/report/reportPackageRequest.ts": "PREVIEW_PHYSICS_CURVED_SHEAR_CONTRACT_ID",
    "apps/desktop/src/features/report/renderableReportInput.ts": "pfTimoshenkoLimitationEntry",
    "apps/desktop/src/test-support/contractBindings.ts": "PREVIEW_PHYSICS_CURVED_SHEAR_CONTRACT_SHA256",
}
# "Admits both" entries with no curved-shear marker, and why.
CURVED_NONE = {
    "core/reporting/result_export/src/recorded_tables.rs": "no run recorded before RT-A4 can name an identity introduced after it",
    "apps/desktop/src/services/recordedPreRtA4.ts": "the desktop twin of that registry; no recorded variant exists",
}
# The literal sites the curved-shear table and constants add; the pre-existing scan does not see them (its needles are
# preview-physics-2's), so they are listed here and checked to name the identity.
CURVED_SITES = {
    "core/reporting/report_package/src/pf_timoshenko.rs": "PREVIEW_PHYSICS_CURVED_SHEAR_ID",
    "apps/desktop/src/features/report/pfTimoshenkoReport.ts": "PREVIEW_PHYSICS_CURVED_SHEAR_CONTRACT_ID",
    "apps/desktop/src-tauri/src/analysis_run_v03_build.rs": "preview_physics_curved_shear_contract",
}


def _is_scanned(path: str) -> bool:
""")
edit(P, """                assert exercised and (PROJECT / exercised).is_file(), f"{path}: no round trip exercises it"
            else:
                assert marker is None and exercised is None, path
""", """                assert exercised and (PROJECT / exercised).is_file(), f"{path}: no round trip exercises it"
            else:
                assert marker is None and exercised is None, path


def test_each_admits_both_entry_names_the_curved_shear_identity_or_states_why_not():
    both = {path for table in (LITERAL_SITES, SYMBOL_GATES) for path, entry in table.items() if entry[1] == BOTH}
    assert set(CURVED_MARKERS) | set(CURVED_NONE) == both, sorted(both ^ (set(CURVED_MARKERS) | set(CURVED_NONE)))
    assert not set(CURVED_MARKERS) & set(CURVED_NONE)
    for path, marker in {**CURVED_MARKERS, **CURVED_SITES}.items():
        assert marker in (PROJECT / path).read_text(encoding="utf-8"), f"{path}: must name the curved-shear identity ({marker})"
    assert all(CURVED_NONE.values())
""")


P = "apps/desktop/src/features/results/knownSemanticLimitations.test.ts"
edit(P, """    // DEL-05-03 `pf-timoshenko.md` §4 adds the pressure-free Timoshenko identity.
    expect([...FRESH_SEMANTIC_CONTRACT_IDS].sort()).toEqual([
      "openpipestress.result_semantics/0.3.0/pressure-3", "openpipestress.result_semantics/0.3.0/pressure-source-1",
      "openpipestress.result_semantics/0.3.0/preview-physics-2", "openpipestress.result_semantics/0.3.0/preview-physics-retained-1",
      "openpipestress.result_semantics/0.3.0/preview-physics-timoshenko-1"]);
""", """    // DEL-05-03 `pf-timoshenko.md` §4 adds the pressure-free Timoshenko identity, and DEL-04-01
    // `curved-bend-transverse-shear.md` §5 the pressure-free curved-shear identity.
    expect([...FRESH_SEMANTIC_CONTRACT_IDS].sort()).toEqual([
      "openpipestress.result_semantics/0.3.0/pressure-3", "openpipestress.result_semantics/0.3.0/pressure-source-1",
      "openpipestress.result_semantics/0.3.0/preview-physics-2", "openpipestress.result_semantics/0.3.0/preview-physics-curved-shear-1",
      "openpipestress.result_semantics/0.3.0/preview-physics-retained-1", "openpipestress.result_semantics/0.3.0/preview-physics-timoshenko-1"]);
""")
P = "core/reporting/result_export/tests/preview_physics_contract.rs"
edit(P, """    // and the pressure-free Timoshenko identity (DEL-05-03 pf-timoshenko.md §4);
    // nothing else is fresh.
    let mut fresh: Vec<&str> = s::FRESH_IDENTITIES.to_vec();
    fresh.sort_unstable();
    let mut expected = vec![
        s::PREVIEW_PHYSICS_ID,
        s::PREVIEW_PHYSICS_TIMOSHENKO_ID,
""", """    // and the pressure-free Timoshenko identity (DEL-05-03 pf-timoshenko.md §4)
    // and curved-shear identity (DEL-04-01 curved-bend-transverse-shear.md §5);
    // nothing else is fresh.
    let mut fresh: Vec<&str> = s::FRESH_IDENTITIES.to_vec();
    fresh.sort_unstable();
    let mut expected = vec![
        s::PREVIEW_PHYSICS_ID,
        s::PREVIEW_PHYSICS_TIMOSHENKO_ID,
        s::PREVIEW_PHYSICS_CURVED_SHEAR_ID,
""")

# ---------------------------------------------------------------- schema and set tests that enumerate the identities
P = "tests/test_load_reference_schema.py"
edit(P, """    # PF Timoshenko (DEL-05-03 pf-timoshenko.md §4): its branch follows preview-physics-2's.
    assert ids == [PRESSURE_SOURCE_ID, PREVIEW_ID, "openpipestress.result_semantics/0.3.0/preview-physics-timoshenko-1", PRESSURE_ID,
""", """    # PF Timoshenko (DEL-05-03 pf-timoshenko.md §4): its branch follows preview-physics-2's, and the curved-shear
    # branch (DEL-04-01 curved-bend-transverse-shear.md §5) follows it.
    assert ids == [PRESSURE_SOURCE_ID, PREVIEW_ID, "openpipestress.result_semantics/0.3.0/preview-physics-timoshenko-1",
                   "openpipestress.result_semantics/0.3.0/preview-physics-curved-shear-1", PRESSURE_ID,
""")
P = "tests/test_load_reference_source_schema.py"
edit(P, """# PF Timoshenko (DEL-05-03 pf-timoshenko.md §4): its branch follows preview-physics-2's in each carrier.
CARRIER_ORDER = [PRS_ID, PREVIEW_ID, "openpipestress.result_semantics/0.3.0/preview-physics-timoshenko-1", PRESSURE_ID, SUCCESSOR_ID, EXACT_ID]
""", """# PF Timoshenko (DEL-05-03 pf-timoshenko.md §4): its branch follows preview-physics-2's in each carrier, and the
# curved-shear branch (DEL-04-01 curved-bend-transverse-shear.md §5) follows it.
CARRIER_ORDER = [PRS_ID, PREVIEW_ID, "openpipestress.result_semantics/0.3.0/preview-physics-timoshenko-1",
                 "openpipestress.result_semantics/0.3.0/preview-physics-curved-shear-1", PRESSURE_ID, SUCCESSOR_ID, EXACT_ID]
""")
P = "tests/test_retained_precision_schema.py"
edit(P, """    # PF Timoshenko added preview-physics-timoshenko-1 (pf-timoshenko.md §4), which refuses a receipt above.
    assert ids.count(SUCC) == 1 and ids.count(EXACT) == 1 and ids.count(PRESSURE_SOURCE) == 1 and len(ids) == 6
""", """    # PF Timoshenko added preview-physics-timoshenko-1 (pf-timoshenko.md §4) and the curved-shear option
    # preview-physics-curved-shear-1 (curved-bend-transverse-shear.md §5), which refuse a receipt above.
    assert ids.count(SUCC) == 1 and ids.count(EXACT) == 1 and ids.count(PRESSURE_SOURCE) == 1 and len(ids) == 7
""")
edit(P, """    # (pf-timoshenko.md §4), which refuses a receipt above.
    assert ids.count(SUCC) == 1 and ids.count(EXACT) == 1 and ids.count(PRESSURE_SOURCE) == 1 and len(ids) == 6
""", """    # (pf-timoshenko.md §4) and the curved-shear option preview-physics-curved-shear-1's
    # (curved-bend-transverse-shear.md §5), which refuse a receipt above.
    assert ids.count(SUCC) == 1 and ids.count(EXACT) == 1 and ids.count(PRESSURE_SOURCE) == 1 and len(ids) == 7
""")
P = "tests/test_source_block_schema_contract.py"
edit(P, """    # PF Timoshenko added preview-physics-timoshenko-1's (pf-timoshenko.md §4).
    assert len(branches) == 6
""", """    # PF Timoshenko added preview-physics-timoshenko-1's (pf-timoshenko.md §4), and the curved-shear option
    # preview-physics-curved-shear-1's (curved-bend-transverse-shear.md §5).
    assert len(branches) == 7
""")
edit(P, """                  "openpipestress.result_semantics/0.3.0/preview-physics-timoshenko-1": "PreviewPhysicsResultSet",
""", """                  "openpipestress.result_semantics/0.3.0/preview-physics-timoshenko-1": "PreviewPhysicsResultSet",
                  "openpipestress.result_semantics/0.3.0/preview-physics-curved-shear-1": "PreviewPhysicsResultSet",
""")
P = "tests/test_preview_physics_consumer_contract.py"
edit(P, """    # DEL-05-03 pf-timoshenko.md §4: plus the pressure-free Timoshenko identity.
    assert FRESH_CONTRACT_IDS == {PREVIEW_PHYSICS_CONTRACT_ID, PRESSURE_SOURCE_CONTRACT_ID, PRESSURE_CONTRACT_ID, PREVIEW_PHYSICS_RETAINED_CONTRACT_ID,
                                  "openpipestress.result_semantics/0.3.0/preview-physics-timoshenko-1"}
""", """    # DEL-05-03 pf-timoshenko.md §4: plus the pressure-free Timoshenko identity; DEL-04-01
    # curved-bend-transverse-shear.md §5: plus the pressure-free curved-shear identity.
    assert FRESH_CONTRACT_IDS == {PREVIEW_PHYSICS_CONTRACT_ID, PRESSURE_SOURCE_CONTRACT_ID, PRESSURE_CONTRACT_ID, PREVIEW_PHYSICS_RETAINED_CONTRACT_ID,
                                  "openpipestress.result_semantics/0.3.0/preview-physics-timoshenko-1",
                                  "openpipestress.result_semantics/0.3.0/preview-physics-curved-shear-1"}
""")

# ---------------------------------------------------------------- admitted-set tests and the parity vectors' list
P = "apps/desktop/src/features/report/exactReportBelowGate.test.tsx"
edit(P, """    // DEL-05-03 `pf-timoshenko.md` §4: the pressure-free Timoshenko identity follows preview-physics-2.
    expect([...REPORT_PACKAGE_ADMITTED_CONTRACT_IDS]).toEqual([ALL_IDS[0], "openpipestress.result_semantics/0.3.0/preview-physics-timoshenko-1", ...ALL_IDS.slice(1)]);
""", """    // DEL-05-03 `pf-timoshenko.md` §4: the pressure-free Timoshenko identity follows preview-physics-2, and
    // the curved-shear identity (DEL-04-01 `curved-bend-transverse-shear.md` §5) follows it.
    expect([...REPORT_PACKAGE_ADMITTED_CONTRACT_IDS]).toEqual([ALL_IDS[0], "openpipestress.result_semantics/0.3.0/preview-physics-timoshenko-1",
      "openpipestress.result_semantics/0.3.0/preview-physics-curved-shear-1", ...ALL_IDS.slice(1)]);
""")
P = "apps/desktop/src/features/report/reportPackageRequest.test.ts"
edit(P, """      // DEL-05-03 `pf-timoshenko.md` §4: the pressure-free Timoshenko identity.
      "openpipestress.result_semantics/0.3.0/preview-physics-timoshenko-1",
      "openpipestress.result_semantics/0.3.0/pressure-3",
""", """      // DEL-05-03 `pf-timoshenko.md` §4: the pressure-free Timoshenko identity.
      "openpipestress.result_semantics/0.3.0/preview-physics-timoshenko-1",
      // DEL-04-01 `curved-bend-transverse-shear.md` §5: the pressure-free curved-shear identity.
      "openpipestress.result_semantics/0.3.0/preview-physics-curved-shear-1",
      "openpipestress.result_semantics/0.3.0/pressure-3",
""")
P = "apps/desktop/src/features/results/pfTimoshenkoIdentity.test.tsx"
edit(P, """    expect(PREVIEW_PHYSICS_CONTRACT_IDS).toEqual([PREVIEW_PHYSICS_CONTRACT_ID, PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID]);
""", """    // DEL-04-01 curved-bend-transverse-shear.md §5: the curved-shear identity follows.
    expect(PREVIEW_PHYSICS_CONTRACT_IDS).toEqual([PREVIEW_PHYSICS_CONTRACT_ID, PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID,
      "openpipestress.result_semantics/0.3.0/preview-physics-curved-shear-1"]);
""")
P = "fixtures/reports/invented/exact_report_sections_parity.json"
edit(P, """    "openpipestress.result_semantics/0.3.0/preview-physics-timoshenko-1",
    "openpipestress.result_semantics/0.3.0/pressure-2",
""", """    "openpipestress.result_semantics/0.3.0/preview-physics-timoshenko-1",
    "openpipestress.result_semantics/0.3.0/preview-physics-curved-shear-1",
    "openpipestress.result_semantics/0.3.0/pressure-2",
""")

# ---------------------------------------------------------------- new test file
NEW_FILES["core/product_physics/tests/pf_curved_shear_runtime.rs"] = (HERE / "pf_curved_shear_runtime.rs").read_text(encoding="utf-8")
NEW_FILES["tests/test_preview_curved_shear_identity.py"] = sub((HERE / "test_preview_curved_shear_identity.py").read_text(encoding="utf-8"))


# ================================================================ schema transforms (JSON, round-trip exact)
def curved_counterpart(value):
    return {TIM_ID: PF_ID, TIM_SHA: PF_SHA, TIM_PROFILE: PF_PROFILE}.get(value)


def deep_curved(node, tim_lim0):
    if isinstance(node, dict):
        return {k: deep_curved(v, tim_lim0) for k, v in node.items()}
    if isinstance(node, list):
        return [deep_curved(v, tim_lim0) for v in node]
    if node == tim_lim0:
        return PF_LIM0
    return curved_counterpart(node) or node


def names_tim(node):
    if isinstance(node, dict):
        return any(names_tim(v) for v in node.values())
    if isinstance(node, list):
        return any(names_tim(v) for v in node)
    return node == TIM_ID


def add_curved(node, tim_lim0, report, path=""):
    """Every enum naming the Timoshenko id, hash or profile gains the curved one after it; every oneOf
    branch naming the Timoshenko id is followed by its curved copy (id, hash, profile, limitation 0)."""
    if isinstance(node, dict):
        for key, value in node.items():
            if key == "oneOf" and isinstance(value, list):
                out = []
                for item in value:
                    out.append(add_curved(item, tim_lim0, report, f"{path}/oneOf"))
                    if names_tim(item):
                        out.append(deep_curved(item, tim_lim0))
                        report.append(f"{path}/oneOf: curved branch")
                node[key] = out
            else:
                node[key] = add_curved(value, tim_lim0, report, f"{path}/{key}")
        return node
    if isinstance(node, list):
        out = []
        for item in node:
            out.append(add_curved(item, tim_lim0, report, path) if isinstance(item, (dict, list)) else item)
            if isinstance(item, str) and curved_counterpart(item):
                out.append(curved_counterpart(item))
                report.append(f"{path}: + {curved_counterpart(item)[:40]}")
        return out
    return node


SCHEMA_TRANSFORMS = {
    "schemas/results.v0.3.schema.yaml": False,
    "schemas/analysis_run.v0.3.schema.json": True,
    "schemas/stress_neutral_export.v0.3.schema.json": True,
}
# Pinned files whose sha256 other files carry (semantic_contract_tables.py PINNED; the schema manifest).
REPIN = ("schemas/results.v0.3.schema.yaml", "schemas/analysis_run.v0.3.schema.json",
         "schemas/stress_neutral_export.v0.3.schema.json", "schemas/model.schema.yaml",
         "schemas/physical_transform_model.schema.yaml")
REPIN_EXCLUDED = ("validation/references/", "execution/", "tools/retained_pricing/", "fixtures/persistence/recorded_",
                  "fixtures/results/recorded_", "fixtures/product_preview/")


def main():
    files = sorted({e[0] for e in E})
    texts = {path: show(path) for path in files}
    out = dict(texts)
    failed = []
    for path, old, new in E:
        n = out[path].count(old)
        if n != 1:
            failed.append(f"{path}: anchor occurs {n} times:\n{old[:300]}\n")
            continue
        out[path] = out[path].replace(old, new)
    if failed:
        print("\n".join(failed))
        raise SystemExit(f"{len(failed)} anchors failed")
    tim_lim0 = _tim["supported_profile_limitations"][0]
    for path, ascii_ in SCHEMA_TRANSFORMS.items():
        text = out.get(path) or show(path)
        texts.setdefault(path, show(path))
        report = []
        value = add_curved(json.loads(text), tim_lim0, report)
        out[path] = json.dumps(value, indent=2, ensure_ascii=ascii_) + "\n"
        print(path, report)
    # repin: each changed pinned file's old sha256 in its text consumers.
    for path in REPIN:
        if path not in out or out[path] == texts[path]:
            continue
        old, new = (hashlib.sha256(t.encode("utf-8")).hexdigest() for t in (texts[path], out[path]))
        listed = subprocess.run(["git", "-C", str(REPO), "grep", "-l", old, BASE, "--", "."], capture_output=True, text=True).stdout.split()
        for consumer in (line.split(":", 1)[1] for line in listed):
            if consumer.startswith(REPIN_EXCLUDED):
                print(f"repin: {consumer} keeps {path} {old[:12]} (excluded)")
                continue
            if consumer not in out:
                texts[consumer] = out[consumer] = show(consumer)
            out[consumer] = out[consumer].replace(old, new)
            print(f"repin: {consumer}: {path} {old[:12]} -> {new[:12]}")
    diffs = []
    for path in sorted(set(out) | set(NEW_FILES)):
        if path in NEW_FILES:
            (B / path).parent.mkdir(parents=True, exist_ok=True)
            (B / path).write_text(NEW_FILES[path], encoding="utf-8")
            d = subprocess.run(["diff", "-u", "--label", "/dev/null", "--label", f"b/{path}", "/dev/null", str(B / path)],
                               capture_output=True, text=True).stdout
            diffs.append(f"diff --git a/{path} b/{path}\nnew file mode 100644\n" + d)
            continue
        if out[path] == texts[path]:
            continue
        for root, text in ((A, texts[path]), (B, out[path])):
            (root / path).parent.mkdir(parents=True, exist_ok=True)
            (root / path).write_text(text, encoding="utf-8")
        d = subprocess.run(["diff", "-u", "--label", f"a/{path}", "--label", f"b/{path}", str(A / path), str(B / path)],
                           capture_output=True, text=True).stdout
        diffs.append(f"diff --git a/{path} b/{path}\n" + d)
    (HERE / "b1.diff").write_text("".join(diffs), encoding="utf-8")
    hunks = sum(d.count("\n@@ ") for d in diffs)
    print(f"{len(diffs)} files, {hunks} hunks; PF table sha256 {PF_SHA}")


main()
