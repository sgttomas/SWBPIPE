//! RT-A3 increment 4 (DEL-04-07 Design `route-unification-r4.md` §2.4 and §3.5
//! S5; DEL-04-01 `kd5-nonlinear-active-set.md` §0, §2.2, §2.4 and §3;
//! DEL-04-07 `pressure-contract.md`, "Nonlinear supports on EX (RT-A3
//! increment 4)"): the pressure-2 reader's binding of the nonlinear support
//! results the exact route publishes through PF's shared active-set loop. The
//! Python mirror is `tests/verifiers/nonlinear_support_evidence.py`.
//!
//! The rows, their kinds and their producer are PF's (`append_nonlinear_support_loop_results`).
//! PF's readers bind only their attribution (`preview_physics_evidence`) and
//! the operand records (`preview_combination_operands`); this module reuses
//! PF's support-magnitude rule (`preview_physics_evidence::guarded` over the
//! correctly rounded `norm3`) and the operand reader's state-code tokens, and
//! adds the state, law and standing checks below.
//!
//! [`check`] reads the envelope alone, per load case with nonlinear rows:
//! - the loop rows: one iteration count (an integer ≥ 1) naming the support
//!   classes, a zero final residual count and a converged flag of 1;
//! - per support: one state row (code 0–3), one final displacement and one
//!   final reaction row, each on the state row's DOF, in that DOF's units,
//!   stating the same `final_state`; and its `NONLINEAR_SUPPORT_STATE_REVIEW`
//!   diagnostic naming that state;
//! - no relative-displacement row: S5 admits nonlinear supports on plain v3
//!   documents only, and EX refuses a support-targeted primitive by name, so
//!   no case moves a nonlinear support's ground (W-16 R-b's row is PF's);
//! - the final displacement equals, bit for bit, some node's published
//!   displacement or rotation row at the support DOF in the row's unit, mm or
//!   rad (the producer's `restate_nonlinear_support_displacements` restates
//!   the support node's row). Without the model this is association with some
//!   published node row, not proof that the row is the support node's: a value
//!   equal to another node's row is admitted. [`model_checks`] names the node;
//! - the final reaction is the support's published action component at its
//!   DOF, bit for bit, and the support's magnitudes follow its components;
//! - a derived friction normal is the absolute value of its named source's
//!   published component, bit for bit; friction supports are exactly the
//!   supports with a normal row, and the classes name the support kinds;
//! - DEC-067 without inputs: a sliding support slips (δ ≠ 0) against its force
//!   (R·δ ≤ 0); an inactive friction support has N ≤ 0 (derived: N = 0);
//! - the standing: one integrity diagnostic with a code the producer assigns
//!   (`assessed_numerical_quality`), and the case's `numerical_quality` record
//!   is exactly that code's four tokens and names the diagnostic; an unknown
//!   code is refused; a gap-bearing case publishes R1's
//!   `NUMERICAL_INTEGRITY_RECOVERY_BASIS_UNQUALIFIED` (unresolved); a Passed
//!   case carries kd5 §3.3's evidence line, has no friction support in state
//!   active and no derived-normal friction support inactive (kd5 §2.2: it fails
//!   closed), and publishes every inactive support's reaction as exact +0
//!   (kd5 §3.1, R4). A non-Passed standing keeps whichever earlier reason the
//!   producer stated (kd5 §2.4: the confirmation did not run or failed);
//! - an inactive support in a non-Passed case keeps the loop's reported
//!   reaction, the binary64 residual of its free row (kd5 §0, §1.2): inspection
//!   evidence, never a claim of zero, and finite (a backstop: the outer number
//!   rules, `SLOT` and `MAGNITUDE` refuse a non-finite value first). The
//!   case must also publish the
//!   producer's own original-equation evaluation of the final linearized solve's
//!   free rows (`append_nonlinear_residual_observation_results`) once in the
//!   support's family (N or N·m), finite, ≥ 0 and with `observed_compliance=true`
//!   (every free row met 64·γ(n) of its own denominator). That record is
//!   required evidence, not a rounding allowance for the reaction: the reaction
//!   is recovered by a different evaluation (a row dot product, then the load
//!   subtracted; `nonlinear_integration::solve_linearized_system_evidence`)
//!   from the residual's (the negative load, then normalized products
//!   accumulated; `frame_kernel` `structural::evaluate_original_residual_bound`),
//!   and the envelope publishes neither row's
//!   denominator, so no published bound relates the two values. Consequently
//!   neither reader can refuse a forged non-Passed inactive reaction magnitude
//!   that keeps its slot and magnitude rows consistent; only the references or
//!   an equilibrium binding the readers do not have refuse it.
//!
//! [`model_checks`] binds the same rows to the model's nonlinear supports
//! (coverage, kinds, DOFs, nodes, normals): the final displacement is the
//! support node's published row at the support DOF, bit for bit. It checks the
//! DEC-067 laws that need their inputs (μ, the clearance and the sense), with a
//! separation s and a contact c formed from that displacement δ and the
//! reaction R:
//! - gap closing in σ with clearance g: s = g − σδ, c = −σR;
//! - one-way or lift-off with sense σ: s = σδ, c = σR;
//! - s ≥ 0; active ⇒ s = 0 and c ≥ 0. An inactive support's force is the
//!   model-free rule above, inspection evidence whose sign and magnitude state
//!   nothing, so s·c is not checked for it;
//! - friction, not inactive: |R| ≤ μN; sticking ⇒ δ = 0; sliding ⇒ |R| = μN,
//!   opposite to a nonzero slip when μN > 0, and in a Passed case with an
//!   explicit normal R = −sign(δ)·fl(μN) bit for bit (kd5 §3.1).
//!
//! Each "= 0" and "≥ 0" of the model checks holds within 1e-9 of the case's
//! scale in the support's family (kd5's criterion; the largest published
//! support component and μN for forces, the largest nodal translation or
//! rotation and the clearance for displacements), except the exact rules named
//! above.
use open_pipe_stress_correct_norm::norm3;
use open_pipe_stress_units::Dimension;
use serde_json::Value;

type Check = Result<(), String>;

const STATE: &str = "nonlinear_support_active_set_state_code";
const DISPLACEMENT: &str = "nonlinear_support_final_displacement";
const RELATIVE: &str = "nonlinear_support_final_relative_displacement";
const REACTION: &str = "nonlinear_support_final_reaction";
const NORMAL_INPUT: &str = "nonlinear_support_friction_normal_reaction_input";
const NORMAL_DERIVED: &str = "nonlinear_support_friction_normal_reaction_derived";
const ITERATIONS: &str = "nonlinear_support_active_set_iteration_count";
const RESIDUAL: &str = "nonlinear_support_active_set_final_residual_count";
const CONVERGED: &str = "nonlinear_support_active_set_converged_flag";
const FORCE_RESIDUAL: &str = "nonlinear_support_observed_free_dof_force_residual";
const MOMENT_RESIDUAL: &str = "nonlinear_support_observed_free_dof_moment_residual";
const LOOP_ENTITY: &str = "nonlinear_supports";
const PER_SUPPORT: [&str; 6] = [STATE, DISPLACEMENT, RELATIVE, REACTION, NORMAL_INPUT, NORMAL_DERIVED];
const COMPONENT: &str = "support_reaction_component_v2";
const FORCE_MAGNITUDE: &str = "support_reaction_force_magnitude_v2";
const MOMENT_MAGNITUDE: &str = "support_reaction_moment_magnitude_v2";
/// `preview_combination_operands`' state-code tokens (0–3).
const STATES: [&str; 4] = ["inactive", "active", "sticking", "sliding"];
const CLASSES: [&str; 4] = ["friction", "gap", "lift_off", "one_way"];
/// The producer's DOF tokens on the rows (`FrameDof::as_str`).
const DOFS: [&str; 6] = ["ux", "uy", "uz", "rx", "ry", "rz"];
const COMPONENTS: [&str; 6] = ["Fx", "Fy", "Fz", "Mx", "My", "Mz"];
const NODAL: [&str; 6] = [
    "global_nodal_displacement_x",
    "global_nodal_displacement_y",
    "global_nodal_displacement_z",
    "global_nodal_rotation_x",
    "global_nodal_rotation_y",
    "global_nodal_rotation_z",
];
const STATE_REVIEW: &str = "NONLINEAR_SUPPORT_STATE_REVIEW";
const INTEGRITY: &str = "NUMERICAL_INTEGRITY_";
const PASSED: &str = "NUMERICAL_INTEGRITY_CHECKS_PASSED";
const UNQUALIFIED: &str = "NUMERICAL_INTEGRITY_RECOVERY_BASIS_UNQUALIFIED";
/// kd5 §3.3's evidence line on a confirmed (Passed) nonlinear case.
const CONFIRMED: &str = "Converged support state: established at 128-bit precision by one formation-checked solve;";
/// kd5's 1e-9 criterion, relative to the case's scale in the family.
const RELATIVE_TOLERANCE: f64 = 1e-9;

/// product_physics `assessed_numerical_quality`: the (structural_status,
/// solve_quality, model_matrix_fidelity, accuracy_evidence) the producer
/// assigns to each integrity code it publishes (`NUMERICAL_INTEGRITY_UNRESOLVED`
/// takes its default arm). `None`: a code the producer does not publish.
fn standing_quality(code: &str) -> Option<[&'static str; 4]> {
    Some(match code {
        PASSED => ["passive_model_basis", "checks_passed", "represented_equations_retained", "not_claimed"],
        "NUMERICAL_INTEGRITY_SENSITIVE" => ["passive_model_basis", "sensitive", "represented_equations_retained", "not_claimed"],
        "NUMERICAL_INTEGRITY_PHYSICAL_MECHANISM" => ["physical_mechanism_witnessed", "failed", "not_assessed", "not_claimed"],
        "NUMERICAL_INTEGRITY_NEGATIVE_ENERGY" => ["negative_energy_witnessed", "failed", "not_assessed", "not_claimed"],
        UNQUALIFIED => ["numerically_unresolved", "unresolved", "not_assessed", "unresolved"],
        "NUMERICAL_INTEGRITY_ASSEMBLY_UNRESOLVED" => ["numerically_unresolved", "unresolved", "assembly_loss_detected", "unresolved"],
        "NUMERICAL_INTEGRITY_FAILED" => ["numerically_unresolved", "failed", "assembly_uncertainty", "unresolved"],
        "NUMERICAL_INTEGRITY_UNRESOLVED" => ["numerically_unresolved", "unresolved", "assembly_uncertainty", "unresolved"],
        _ => return None,
    })
}

fn require(ok: bool, code: &str) -> Check {
    if ok {
        Ok(())
    } else {
        Err(format!("SOURCE_PHYSICS_NONLINEAR_{code}"))
    }
}
fn items(v: &Value) -> &[Value] {
    v.as_array().map(Vec::as_slice).unwrap_or(&[])
}
fn text(v: &Value) -> Option<&str> {
    v.as_str().filter(|s| !s.is_empty())
}
fn value(row: &Value) -> f64 {
    row["value"].as_f64().unwrap_or(f64::NAN)
}
fn kind(row: &Value) -> &str {
    row["kind"].as_str().unwrap_or("")
}
fn case_of(row: &Value) -> Option<&str> {
    (row["basis_ref"]["ref_type"] == "load_case").then(|| row["basis_ref"]["ref_id"].as_str()).flatten()
}
fn basis(row: &Value) -> &str {
    row["metadata"]["basis"].as_str().unwrap_or("")
}
/// The value of a `key=value` segment of a row basis (segments are `; `-separated).
fn segment<'a>(basis: &'a str, key: &str) -> Option<&'a str> {
    basis.split("; ").find_map(|s| s.strip_prefix(key).and_then(|rest| rest.strip_prefix('=')))
}
fn dof_index(dof: &str) -> Option<usize> {
    DOFS.iter().position(|d| *d == dof)
}
fn same_bits(a: f64, b: f64) -> bool {
    a.to_bits() == b.to_bits()
}

/// One support's published rows in one case.
struct Support<'a> {
    id: &'a str,
    dof: usize,
    state: &'static str,
    /// The deciding displacement δ in the row's unit (mm or rad).
    displacement: f64,
    reaction: f64,
    /// `(derived, value)` for a friction support.
    normal: Option<(bool, f64)>,
    source: Option<(&'a str, usize)>,
}

/// One case's nonlinear evidence, as [`check`] admitted it.
struct Case<'a> {
    id: &'a str,
    classes: Vec<&'a str>,
    supports: Vec<Support<'a>>,
    code: &'a str,
}

fn case_ids(rows: &[Value]) -> Result<Vec<&str>, String> {
    let mut cases: Vec<&str> = Vec::new();
    // A combination's rows are the combination pass's (`exact_combination_evidence`).
    for row in rows.iter().filter(|r| kind(r).starts_with("nonlinear_support_") && r["basis_ref"]["ref_type"] != "combination") {
        let case = case_of(row).ok_or_else(|| "SOURCE_PHYSICS_NONLINEAR_SCOPE".to_string())?;
        if !cases.contains(&case) {
            cases.push(case);
        }
    }
    Ok(cases)
}

fn read_case<'a>(source: &'a Value, case: &'a str) -> Result<Case<'a>, String> {
    let rows = items(&source["results"]);
    let in_case: Vec<&Value> = rows.iter().filter(|r| case_of(r) == Some(case)).collect();
    let nonlinear: Vec<&Value> = in_case.iter().copied().filter(|r| kind(r).starts_with("nonlinear_support_")).collect();
    // The loop rows.
    let one = |k: &str| {
        let found: Vec<&&Value> = nonlinear.iter().filter(|r| kind(r) == k).collect();
        (found.len() == 1 && found[0]["entity_ref"] == LOOP_ENTITY).then(|| *found[0])
    };
    let (iterations, residual, converged) = (one(ITERATIONS), one(RESIDUAL), one(CONVERGED));
    let (Some(iterations), Some(residual), Some(converged)) = (iterations, residual, converged) else {
        return Err("SOURCE_PHYSICS_NONLINEAR_LOOP".into());
    };
    let count = value(iterations);
    require(count >= 1.0 && count.fract() == 0.0 && value(residual) == 0.0 && value(converged) == 1.0, "LOOP")?;
    let classes: Vec<&str> = segment(basis(iterations), "support_classes").map(|c| c.split(',').collect()).unwrap_or_default();
    require(
        !classes.is_empty() && classes.iter().all(|c| CLASSES.contains(c)) && classes.windows(2).all(|w| w[0] < w[1]),
        "CLASSES",
    )?;
    let mode = basis(iterations).split_once("_active_set_loop").map(|(m, _)| m).filter(|m| !m.is_empty() && !m.contains(' '));
    let mode = mode.ok_or_else(|| "SOURCE_PHYSICS_NONLINEAR_LOOP".to_string())?;
    // Every other nonlinear row is per support or the loop's own; no case
    // moves a nonlinear support's ground on S5 (module docs).
    for row in &nonlinear {
        let k = kind(row);
        require(k != RELATIVE, "RELATIVE_DISPLACEMENT")?;
        require(PER_SUPPORT.contains(&k) || row["entity_ref"] == LOOP_ENTITY, "ROWS")?;
    }
    // Per support, in state-row order.
    let mut supports: Vec<Support> = Vec::new();
    for row in nonlinear.iter().filter(|r| kind(r) == STATE) {
        let id = text(&row["entity_ref"]).ok_or_else(|| "SOURCE_PHYSICS_NONLINEAR_ROWS".to_string())?;
        require(!supports.iter().any(|s| s.id == id), "STATE_DUPLICATE")?;
        let code = value(row);
        require([0.0, 1.0, 2.0, 3.0].contains(&code), "STATE_CODE")?;
        let state = STATES[code as usize];
        let dof = row["metadata"]["location"].as_str().and_then(dof_index).ok_or_else(|| "SOURCE_PHYSICS_NONLINEAR_DOF".to_string())?;
        let mine = |k: &str| nonlinear.iter().copied().filter(|r| kind(r) == k && r["entity_ref"] == id).collect::<Vec<_>>();
        let (displacement, reaction) = (mine(DISPLACEMENT), mine(REACTION));
        let normals: Vec<&Value> = mine(NORMAL_INPUT).into_iter().chain(mine(NORMAL_DERIVED)).collect();
        require(displacement.len() == 1 && reaction.len() == 1 && normals.len() <= 1, "ROWS")?;
        let translational = dof < 3;
        let (length, force) = if translational { ("mm", "N") } else { ("rad", "N*m") };
        let stated = |r: &Value| segment(basis(r), "final_state") == Some(state) && r["metadata"]["location"] == DOFS[dof];
        require(stated(row) && stated(displacement[0]) && stated(reaction[0]), "STATE_BASIS")?;
        require(row["unit"] == "state_code" && displacement[0]["unit"] == length && reaction[0]["unit"] == force, "UNIT")?;
        // Some node's published row (association, not ownership; module docs).
        let restated = in_case.iter().any(|r| kind(r) == NODAL[dof] && r["unit"] == length && same_bits(value(r), value(displacement[0])));
        require(restated, "DISPLACEMENT")?;
        let (mut normal, mut source) = (None, None);
        if let Some(n) = normals.first() {
            let derived = kind(n) == NORMAL_DERIVED;
            require(n["unit"] == "N", "UNIT")?;
            if derived {
                let named = segment(basis(n), "source_ref");
                let at = segment(basis(n), "source_dof").and_then(dof_index).filter(|d| *d < 3);
                let (Some(named), Some(at)) = (named, at) else {
                    return Err("SOURCE_PHYSICS_NONLINEAR_NORMAL".into());
                };
                source = Some((named, at));
            }
            normal = Some((derived, value(n)));
        }
        supports.push(Support {
            id,
            dof,
            state,
            displacement: value(displacement[0]),
            reaction: value(reaction[0]),
            normal,
            source,
        });
    }
    // No per-support row names a support without a state row.
    for row in nonlinear.iter().filter(|r| PER_SUPPORT.contains(&kind(r))) {
        require(supports.iter().any(|s| row["entity_ref"] == s.id), "ROWS")?;
    }
    require(!supports.is_empty(), "ROWS")?;
    // The published support actions.
    let action = |entity: &str, kind_name: &str, component: &str| {
        let found: Vec<&&Value> = in_case
            .iter()
            .filter(|r| kind(r) == kind_name && r["entity_ref"] == entity && r["metadata"]["component"] == component)
            .collect();
        (found.len() == 1).then(|| value(found[0]))
    };
    for s in &supports {
        let slot = action(s.id, COMPONENT, COMPONENTS[s.dof]);
        require(slot.is_some_and(|v| same_bits(v, s.reaction)), "SLOT")?;
        // PF's magnitude rule (`preview_physics_evidence::magnitudes`).
        let c = COMPONENTS.map(|component| action(s.id, COMPONENT, component));
        let (fm, mm) = (action(s.id, FORCE_MAGNITUDE, "force_magnitude"), action(s.id, MOMENT_MAGNITUDE, "moment_magnitude"));
        let consistent = |m: Option<f64>, parts: &[Option<f64>]| match (m, parts) {
            (Some(m), [Some(a), Some(b), Some(c)]) => m >= 0.0 && crate::preview_physics_evidence::guarded(m, norm3(*a, *b, *c)),
            _ => false,
        };
        require(consistent(fm, &c[..3]) && consistent(mm, &c[3..]), "MAGNITUDE")?;
        if let Some((named, at)) = s.source {
            let published = action(named, COMPONENT, COMPONENTS[at]);
            require(published.is_some_and(|v| same_bits(v.abs(), s.normal.unwrap().1)), "NORMAL")?;
        }
    }
    // The state diagnostics: one per support, naming its state.
    let diagnostics = items(&source["diagnostics"]);
    let reviews: Vec<&Value> = diagnostics
        .iter()
        .filter(|d| d["code"] == STATE_REVIEW && items(&d["affected_refs"]).get(1).is_some_and(|c| c.as_str() == Some(case)))
        .collect();
    require(reviews.len() == supports.len(), "STATE_REVIEW")?;
    for s in &supports {
        let message = format!("nonlinear support {} ended in {} state for load case {case}; {mode} preview loop evidence only", s.id, s.state);
        require(
            reviews.iter().filter(|d| d["affected_refs"][0] == s.id && d["affected_refs"].as_array().is_some_and(|a| a.len() == 2) && d["message"] == message.as_str()).count() == 1,
            "STATE_REVIEW",
        )?;
    }
    // Kinds and classes.
    let friction = supports.iter().filter(|s| s.normal.is_some()).count();
    require((friction > 0) == classes.contains(&"friction") && (friction < supports.len()) == classes.iter().any(|c| *c != "friction"), "CLASSES")?;
    for s in &supports {
        match s.normal {
            None => require(matches!(s.state, "inactive" | "active"), "STATE_KIND")?,
            Some((derived, n)) => {
                // DEC-067 without inputs.
                if s.state == "sliding" {
                    require(s.displacement != 0.0 && s.reaction * s.displacement <= 0.0, "SLIP")?;
                }
                if s.state == "inactive" {
                    require(if derived { n == 0.0 } else { n <= 0.0 }, "FRICTION_NORMAL")?;
                }
            }
        }
    }
    // The standing.
    let integrity: Vec<&Value> = diagnostics
        .iter()
        .filter(|d| d["code"].as_str().is_some_and(|c| c.starts_with(INTEGRITY)) && items(&d["affected_refs"]) == [Value::from(case)])
        .collect();
    require(integrity.len() == 1, "STANDING")?;
    let code = integrity[0]["code"].as_str().unwrap_or("");
    let message = integrity[0]["message"].as_str().unwrap_or("");
    let quality: Vec<&Value> = items(&source["numerical_quality"]["cases"])
        .iter()
        .filter(|q| q["basis_ref"]["ref_type"] == "load_case" && q["basis_ref"]["ref_id"] == case)
        .collect();
    require(quality.len() == 1, "STANDING")?;
    let solve_quality = quality[0]["solve_quality"].as_str().unwrap_or("");
    // The producer's whole code-to-quality assignment, not only Passed's.
    let tokens = standing_quality(code).ok_or_else(|| "SOURCE_PHYSICS_NONLINEAR_STANDING_CODE".to_string())?;
    let q = quality[0];
    let fields = [&q["structural_status"], &q["solve_quality"], &q["model_matrix_fidelity"], &q["accuracy_evidence"]];
    require(
        fields.iter().zip(tokens).all(|(field, token)| **field == token)
            && integrity[0]["id"].is_string()
            && items(&q["evidence_refs"]).contains(&integrity[0]["id"]),
        "STANDING",
    )?;
    if classes.contains(&"gap") {
        require(code == UNQUALIFIED && solve_quality == "unresolved", "GAP_STANDING")?;
    }
    if code == PASSED {
        require(message.contains(CONFIRMED), "PASSED_EVIDENCE")?;
        for s in &supports {
            let derived_inactive = s.state == "inactive" && s.normal.is_some_and(|(derived, _)| derived);
            require(!derived_inactive, "FRICTION_NORMAL_STANDING")?;
            require(!(s.normal.is_some() && s.state == "active"), "PASSED_STATE")?;
            if s.state == "inactive" {
                require(same_bits(s.reaction, 0.0), "PASSED_ZERO")?;
            }
        }
    } else {
        // A non-Passed inactive support: the loop's free-row residual as
        // inspection evidence, with the producer's own compliant evaluation
        // record in its family; no magnitude relation between the two
        // (different evaluations; module docs).
        for s in supports.iter().filter(|s| s.state == "inactive") {
            require(s.reaction.is_finite(), "INACTIVE_REACTION")?;
            free_row_residual(&nonlinear, s.dof < 3)?;
        }
    }
    Ok(Case { id: case, classes, supports, code })
}

/// The producer's original-equation evaluation of the final linearized solve's
/// free rows in one family (`append_nonlinear_residual_observation_results`,
/// over `product_equilibrium::evaluate`): its largest |residual| in N
/// (translational rows) or N·m (rotational rows), published once on the loop
/// with that evaluation's verdict, which must be compliant. Required evidence;
/// its value bounds no other evaluation.
fn free_row_residual(nonlinear: &[&Value], translational: bool) -> Check {
    let (wanted, unit) = if translational { (FORCE_RESIDUAL, "N") } else { (MOMENT_RESIDUAL, "N*m") };
    let found: Vec<&&Value> = nonlinear.iter().filter(|r| kind(r) == wanted).collect();
    let [row] = found[..] else {
        return Err("SOURCE_PHYSICS_NONLINEAR_RESIDUAL".into());
    };
    let (evaluated, b) = (value(row), basis(row));
    require(
        row["entity_ref"] == LOOP_ENTITY
            && row["unit"] == unit
            && evaluated.is_finite()
            && evaluated >= 0.0
            && segment(b, "residual_basis") == Some("free_dof_force_moment_equilibrium")
            && segment(b, "observed_compliance") == Some("true"),
        "RESIDUAL",
    )
}

/// The model-free checks (module docs), per load case with nonlinear rows in
/// first-row order. An envelope without nonlinear rows passes unchanged.
pub(crate) fn check(source: &Value) -> Check {
    read(source).map(|_| ())
}

fn read(source: &Value) -> Result<Vec<Case<'_>>, String> {
    let rows = items(&source["results"]);
    case_ids(rows)?.into_iter().map(|case| read_case(source, case)).collect()
}

/// One model nonlinear support's inputs, parsed as the producer parses them.
struct Input<'a> {
    id: &'a Value,
    /// The support's node (`support.node`).
    node: &'a str,
    behavior: &'static str,
    dof: usize,
    /// σ: the gap's closing sense or the unilateral support's contact sense.
    sense: f64,
    clearance_mm: f64,
    mu: f64,
    explicit_normal: Option<f64>,
    source: Option<(&'a Value, usize)>,
}

fn model_input(support: &Value) -> Result<Input<'_>, String> {
    let bad = || "SOURCE_PHYSICS_NONLINEAR_MODEL_SUPPORT".to_string();
    let n = &support["nonlinear"];
    let dof = match n["dof"].as_str() {
        Some(d) if ["UX", "Ux", "ux", "UY", "Uy", "uy", "UZ", "Uz", "uz", "RX", "Rx", "rx", "RY", "Ry", "ry", "RZ", "Rz", "rz"].contains(&d) => {
            dof_index(&d.to_ascii_lowercase()).ok_or_else(bad)?
        }
        _ => return Err(bad()),
    };
    let positive = |token: &str, plus: [&str; 4], minus: [&str; 4]| {
        if plus.contains(&token) {
            Some(1.0)
        } else if minus.contains(&token) {
            Some(-1.0)
        } else {
            None
        }
    };
    let reaction_sense = |field: &str| {
        n[field].as_str().and_then(|t| {
            positive(t, ["positive_reaction", "positive", "POSITIVE_REACTION", "POSITIVE"], ["negative_reaction", "negative", "NEGATIVE_REACTION", "NEGATIVE"])
        })
    };
    let quantity = |q: &Value, dimension: Dimension| crate::physics_source::normalized_quantity(q, dimension).map_err(|_| bad());
    let node = text(&support["node"]).ok_or_else(bad)?;
    let mut input = Input { id: &support["id"], node, behavior: "", dof, sense: 0.0, clearance_mm: 0.0, mu: 0.0, explicit_normal: None, source: None };
    match n["behavior"].as_str().unwrap_or("") {
        "one_way" | "one-way" | "oneway" => {
            input.behavior = "one_way";
            input.sense = reaction_sense("active_when").ok_or_else(bad)?;
        }
        "lift_off" | "lift-off" | "liftoff" => {
            input.behavior = "lift_off";
            input.sense = reaction_sense("contact_when").ok_or_else(bad)?;
        }
        "gap" => {
            input.behavior = "gap";
            input.sense = n["closes_when"]
                .as_str()
                .and_then(|t| {
                    positive(t, ["positive_displacement", "positive", "POSITIVE_DISPLACEMENT", "POSITIVE"], ["negative_displacement", "negative", "NEGATIVE_DISPLACEMENT", "NEGATIVE"])
                })
                .ok_or_else(bad)?;
            require(dof < 3, "MODEL_SUPPORT")?;
            input.clearance_mm = quantity(&n["gap"], Dimension::Length)? * 1000.0;
        }
        "friction" => {
            input.behavior = "friction";
            let coefficient = &n["friction_coefficient"];
            input.mu = if coefficient["unit"] == "none" {
                coefficient["value"].as_f64().filter(|v| v.is_finite()).ok_or_else(bad)?
            } else {
                quantity(coefficient, Dimension::Dimensionless)?
            };
            match (n.get("normal_reaction").filter(|v| !v.is_null()), n.get("normal_reaction_source").filter(|v| !v.is_null())) {
                (Some(normal), None) => input.explicit_normal = Some(quantity(normal, Dimension::Force)?),
                (None, Some(source)) => {
                    let at = source["dof"].as_str().filter(|d| ["UX", "Ux", "ux", "UY", "Uy", "uy", "UZ", "Uz", "uz"].contains(d)).and_then(|d| dof_index(&d.to_ascii_lowercase())).ok_or_else(bad)?;
                    input.source = Some((&source["support_ref"], at));
                }
                _ => return Err(bad()),
            }
        }
        _ => return Err(bad()),
    }
    Ok(input)
}

/// The case's scale in one family (forces or moments; translations or
/// rotations): the largest published support component and nodal row.
fn scales(source: &Value, case: &str, translational: bool) -> (f64, f64) {
    let (components, nodal) = if translational { (&COMPONENTS[..3], &NODAL[..3]) } else { (&COMPONENTS[3..], &NODAL[3..]) };
    let mut force = 0.0_f64;
    let mut motion = 0.0_f64;
    for row in items(&source["results"]).iter().filter(|r| case_of(r) == Some(case)) {
        let v = value(row).abs();
        if kind(row) == COMPONENT && row["metadata"]["component"].as_str().is_some_and(|c| components.contains(&c)) {
            force = force.max(v);
        }
        if nodal.contains(&kind(row)) {
            motion = motion.max(v);
        }
    }
    (force, motion)
}

/// The model-bound checks (module docs), after the model-free reader admitted
/// the envelope; a solved envelope only.
pub(crate) fn model_checks(source: &Value, model: &Value) -> Check {
    if source["status"]["mechanics"] != "MECHANICS_SOLVED" || model.get("supports").is_none_or(Value::is_null) {
        return Ok(());
    }
    let records: Vec<&Value> = items(&model["supports"]).iter().filter(|s| s.get("nonlinear").is_some_and(|n| !n.is_null())).collect();
    let inputs = records.iter().map(|s| model_input(s)).collect::<Result<Vec<_>, _>>()?;
    let cases = read(source)?;
    let solved: Vec<&Value> = items(&source["contract_evidence"]["exact_cases"]).iter().map(|c| &c["load_case_id"]).collect();
    // Every solved case publishes exactly the model's nonlinear supports.
    for id in &solved {
        let found = cases.iter().find(|c| id.as_str() == Some(c.id));
        match found {
            None => require(inputs.is_empty(), "MODEL_COVERAGE")?,
            Some(case) => require(
                case.supports.len() == inputs.len() && inputs.iter().all(|i| case.supports.iter().any(|s| i.id.as_str() == Some(s.id))),
                "MODEL_COVERAGE",
            )?,
        }
    }
    require(cases.iter().all(|c| solved.iter().any(|id| id.as_str() == Some(c.id))), "MODEL_COVERAGE")?;
    for case in &cases {
        let mut behaviors: Vec<&str> = inputs.iter().map(|i| i.behavior).collect();
        behaviors.sort_unstable();
        behaviors.dedup();
        require(case.classes == behaviors, "MODEL_CLASSES")?;
        for s in &case.supports {
            let input = inputs.iter().find(|i| i.id.as_str() == Some(s.id)).ok_or_else(|| "SOURCE_PHYSICS_NONLINEAR_MODEL_COVERAGE".to_string())?;
            require(input.dof == s.dof, "MODEL_DOF")?;
            // The final displacement is the support node's published row.
            let length = if s.dof < 3 { "mm" } else { "rad" };
            let nodal: Vec<f64> = items(&source["results"])
                .iter()
                .filter(|r| case_of(r) == Some(case.id) && kind(r) == NODAL[s.dof] && r["entity_ref"] == input.node && r["unit"] == length)
                .map(value)
                .collect();
            require(nodal.len() == 1 && same_bits(nodal[0], s.displacement), "MODEL_DISPLACEMENT")?;
            let normal_bound = match (input.behavior, s.normal, s.source) {
                ("friction", Some((false, n)), None) => input.explicit_normal.is_some_and(|m| same_bits(m, n)),
                ("friction", Some((true, _)), Some((named, at))) => input.source.is_some_and(|(r, d)| r.as_str() == Some(named) && d == at),
                ("friction", _, _) => false,
                (_, normal, _) => normal.is_none(),
            };
            require(normal_bound, "MODEL_NORMAL")?;
            let (force_scale, motion_scale) = scales(source, case.id, s.dof < 3);
            let (r, delta) = (s.reaction, s.displacement);
            if input.behavior == "friction" {
                let n = s.normal.map(|(_, n)| n).unwrap_or(0.0);
                let mu_n = input.mu * n;
                let tolerance = RELATIVE_TOLERANCE * force_scale.max(mu_n.abs());
                if s.state != "inactive" {
                    require(r.abs() <= mu_n + tolerance, "MODEL_FRICTION_CAPACITY")?;
                }
                match s.state {
                    "sticking" => require(delta.abs() <= RELATIVE_TOLERANCE * motion_scale, "MODEL_STICK")?,
                    "sliding" => {
                        require((r.abs() - mu_n).abs() <= tolerance && (mu_n <= tolerance || r * delta < 0.0), "MODEL_SLIDE")?;
                        if case.code == PASSED && input.explicit_normal.is_some() {
                            // kd5 §3.1: fl(φ), φ = −dir·μ·|N| (dir the slip sign); +0 at μN = 0.
                            let exact = if mu_n == 0.0 { 0.0 } else { -(mu_n.copysign(delta)) };
                            require(same_bits(r, exact), "MODEL_SLIDE_EXACT")?;
                        }
                    }
                    _ => {}
                }
            } else {
                let (separation, contact) = match input.behavior {
                    "gap" => (input.clearance_mm - input.sense * delta, -input.sense * r),
                    _ => (input.sense * delta, input.sense * r),
                };
                let length_tolerance = RELATIVE_TOLERANCE * motion_scale.max(input.clearance_mm.abs());
                let force_tolerance = RELATIVE_TOLERANCE * force_scale;
                require(separation >= -length_tolerance, "MODEL_COMPLEMENTARITY")?;
                // An active support is closed and pushes; an inactive one's
                // force is the model-free rule (a residual whose sign and
                // magnitude state nothing), so s·c is not checked for it.
                if s.state == "active" {
                    require(separation.abs() <= length_tolerance && contact >= -force_tolerance, "MODEL_COMPLEMENTARITY")?;
                }
            }
        }
    }
    Ok(())
}
