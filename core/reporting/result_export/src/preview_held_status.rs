//! RT-A5 `minuend_final` (DEL-05-02 Work `operating-case.md` §7.6): a preview-physics-2
//! declared operating comparison whose subtrahend is the subtrahend case's loads solved
//! with each nonlinear support held at the minuend's final status. Its held gates, its
//! held-operand rows (basis `held_status_operand`, under the combination), its rows' sign
//! convention and its evidence (`contract_evidence.held_status_derivation`). The Python
//! mirror is `tests/verifiers/preview_held_status.py`.
//!
//! - **Without the model** (option A, as slice 1b). Every record: a closed shape in gate
//!   order (`…_HELD_STATUS_EVIDENCE`), bound to a declared subtraction's
//!   `difference_basis_derivation` record (source = minuend, target = subtrahend), the
//!   document's solver mode, and a gate that is exactly the record's outcome
//!   (`…_HELD_STATUS_BINDING`); the source's qualification by the outcome, its standing by the
//!   minuend's confirmed evidence line, the target's modulus key by the subtrahend's modulus
//!   record and each derived normal's source by the minuend's derived-normal row
//!   (`…_HELD_STATUS_BINDING`). Admitted records only: `held` covers exactly the
//!   minuend's published state-code rows, each held state equal to its code and its DOF
//!   the row's location, and the held-operand rows exist under the combination
//!   (`…_HELD_STATUS_BINDING`). Each ordinary decision (`classified_state`, `released`
//!   and the derived-normal branch) is re-derived from its published inputs in binary64,
//!   in the classifier's order; each strict-gap decision by the decision rule over its
//!   published exact signs and prior (never from binary64 bits, and the signs are the
//!   proof's, not verified here) (`…_HELD_STATUS_DERIVATION`). The standing, the
//!   conditions, `HELD_STATUS_RELEASED` and the held solve's convergence and exit diagnostic
//!   follow from the decisions; the sign text is
//!   only on the difference rows and the typed basis only on held-operand rows
//!   (`…_HELD_STATUS_ROW`).
//! - **With the model.** The model-free checks first; then each support's node, DOF,
//!   behaviour, sense or closing direction, gap, friction coefficient, explicit normal and
//!   derived-normal source (support, node and DOF) against the model, the target's modulus key
//!   against the subtrahend case's, and each trial's ground and the sliding
//!   ground rule from both cases' authored imposed motion (`…_HELD_STATUS_MODEL`). A case
//!   with `condition_use` is refused (`SOURCE_PREVIEW_PHYSICS_COMBINATION_GATE_UNVERIFIED`).
//! - **What is claimed.** Both readings detect a forgery of what they check. Solved
//!   values (u, r, the normal, the friction force, the exact signs, the certified rows)
//!   are never recomputed, so a forgery that alters solved values coherently with their
//!   decisions passes both readings.
use serde_json::Value;
use std::collections::{HashMap, HashSet};

type Check = Result<(), String>;

pub(crate) const MEMBER: &str = "held_status_derivation";
pub(crate) const OPERAND_BASIS: &str = "held_status_operand";
pub(crate) const HELD_SUPPORT_STATE: &str = "held_at_minuend_final";
pub(crate) const RELEASED: &str = "HELD_STATUS_RELEASED";
/// §7.5's held gates, in gate order (exported as [`crate::HELD_STATUS_GATE_CODES`]).
pub const GATES: [&str; 5] = [
    "HELD_STATUS_SOURCE_UNAVAILABLE",
    "HELD_STATUS_SOURCE_UNQUALIFIED",
    "HELD_STATUS_BINDING",
    "HELD_STATUS_SLIDING_GROUND_DIFFERS",
    "HELD_STATUS_SOLVE_FAILED",
];
const SLIDING_GROUND_DIFFERS: &str = "HELD_STATUS_SLIDING_GROUND_DIFFERS";
/// The producer's `held_status::SIGN`.
pub(crate) const SIGN: &str = "positive value follows explicit user result-state subtraction (minuend minus subtrahend) of matching source result sign conventions; operating minus sustained difference at the operating support status: the minuend case's own solved state minus the subtrahend case's loads solved with each nonlinear support held at the minuend's final status, a held-constraint comparison that can need a reaction a unilateral support cannot supply and is not an admissible sustained state, under the static, history-free friction model; derived from case results; not an equilibrium state";
const ESTABLISHED: &str = "Established within its stated numerical bound for the explicitly held system at this case's loads. The verdict does not establish this case's own support state, an admissible sustained state, a load history or uniqueness.";
const RELEASED_STANDING: &str = "Not established: the held status is not consistent with this case's loads; rows are inspection evidence.";
const GAP_STANDING: &str = "Gap supports keep their recovery-basis branch (R1); not established.";
const NOT_ESTABLISHED_PREFIX: &str = "Not established: ";
const NOT_ESTABLISHED_SUFFIX: &str = ". The held status is kept by this case's classifier; rows are inspection evidence.";
const SUBTRACTION_BASIS: &str = "explicit_user_result_state_subtraction";
const STATE_CODE: &str = "nonlinear_support_active_set_state_code";
const ITERATION_COUNT: &str = "nonlinear_support_active_set_iteration_count";
const MODULUS_RECORD: &str = "combination_modulus_basis_record";
const EVIDENCE: &str = "SOURCE_PREVIEW_PHYSICS_HELD_STATUS_EVIDENCE";
const BINDING: &str = "SOURCE_PREVIEW_PHYSICS_HELD_STATUS_BINDING";
const DERIVATION: &str = "SOURCE_PREVIEW_PHYSICS_HELD_STATUS_DERIVATION";
const ROW: &str = "SOURCE_PREVIEW_PHYSICS_HELD_STATUS_ROW";
const MODEL: &str = "SOURCE_PREVIEW_PHYSICS_HELD_STATUS_MODEL";
const UNVERIFIED: &str = "SOURCE_PREVIEW_PHYSICS_COMBINATION_GATE_UNVERIFIED";

const RECORD_KEYS: [&str; 8] = ["combination_id", "subtrahend_support_state", "source", "target", "outcome", "held", "held_solve", "standing"];
const STATES: [&str; 4] = ["inactive", "active", "sticking", "sliding"];
/// The producer's DOF tokens on a held entry (`FrameDof::as_str`, upper case).
const DOFS: [&str; 6] = ["UX", "UY", "UZ", "RX", "RY", "RZ"];
const NORMAL_DERIVED: &str = "nonlinear_support_friction_normal_reaction_derived";
const NONCONVERGENCE: &str = "NONLINEAR_SUPPORT_NONCONVERGENCE";
const UNAVAILABLE: &str = "HELD_STATUS_SOURCE_UNAVAILABLE";
const UNQUALIFIED: &str = "HELD_STATUS_SOURCE_UNQUALIFIED";
/// The base-material label of an operand's modulus record (no selected modulus basis).
const BASE_MATERIAL: &str = "material_base_values";

fn require(ok: bool, code: &str) -> Check {
    if ok {
        Ok(())
    } else {
        Err(code.to_string())
    }
}
fn items(v: &Value) -> &[Value] {
    v.as_array().map(Vec::as_slice).unwrap_or(&[])
}
fn exact_keys(value: &Value, keys: &[&str]) -> bool {
    value.as_object().is_some_and(|o| o.len() == keys.len() && keys.iter().all(|k| o.contains_key(*k)))
}
fn text<'a>(v: &'a Value, code: &str) -> Result<&'a str, String> {
    v.as_str().filter(|s| !s.is_empty()).ok_or_else(|| code.to_string())
}
fn combination_of(row: &Value) -> Option<&str> {
    (row["basis_ref"]["ref_type"] == "combination").then(|| row["basis_ref"]["ref_id"].as_str()).flatten()
}
fn case_of(row: &Value) -> Option<&str> {
    (row["basis_ref"]["ref_type"] == "load_case").then(|| row["basis_ref"]["ref_id"].as_str()).flatten()
}
/// A held-operand row: its id is the combination's `:held:` id.
pub(crate) fn is_held_row(row: &Value) -> bool {
    combination_of(row).is_some() && row["id"].as_str().is_some_and(|id| id.contains(":held:"))
}
/// Sixteen lowercase hexadecimal digits, read as finite binary64 bits.
fn bits(v: &Value) -> Result<f64, String> {
    let s = v.as_str().ok_or_else(|| EVIDENCE.to_string())?;
    require(s.len() == 16 && s.bytes().all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b)), EVIDENCE)?;
    let x = f64::from_bits(u64::from_str_radix(s, 16).map_err(|_| EVIDENCE.to_string())?);
    require(x.is_finite(), EVIDENCE)?;
    Ok(x)
}
fn state(v: &Value) -> Result<&str, String> {
    v.as_str().filter(|s| STATES.contains(s)).ok_or_else(|| EVIDENCE.to_string())
}
fn code_state(code: f64) -> Option<&'static str> {
    STATES.get(code as usize).copied().filter(|_| code.fract() == 0.0 && (0.0..=3.0).contains(&code))
}

/// One support entry, read: its shape and the decision its inputs imply.
struct Entry<'a> {
    support_id: &'a str,
    behavior: &'a str,
    held: &'a str,
    released: &'a str,
    gap: bool,
    derived: bool,
    zero_limit: bool,
    normal_trial: f64,
}

/// §7.2's classifier over one ordinary entry's binary64 inputs, in the classifier's order
/// (NS `classify_support_state`; NI `resolve_sliding_friction_states`): the classified
/// state and the derived-normal branch verdict.
fn classify_ordinary(entry: &Value, behavior: &str, held: &str) -> Result<(&'static str, bool, bool, f64), String> {
    let inputs = &entry["inputs"];
    let delta = bits(&inputs["displacement_bits"])?;
    let r = bits(&inputs["reaction_bits"])?;
    let _ = bits(&inputs["ground_bits"])?;
    let engaged = held == "active";
    let active = |yes: bool| if yes { "active" } else { "inactive" };
    Ok(match behavior {
        "one_way" | "lift_off" => {
            let positive = match entry["sense"].as_str() {
                Some("positive_reaction") => true,
                Some("negative_reaction") => false,
                _ => return Err(EVIDENCE.into()),
            };
            let state = if engaged { if positive { r > 0.0 } else { r < 0.0 } } else if positive { delta < 0.0 } else { delta > 0.0 };
            (active(state), true, false, 0.0)
        }
        "gap" => {
            let positive = match entry["closes_when"].as_str() {
                Some("positive_displacement") => true,
                Some("negative_displacement") => false,
                _ => return Err(EVIDENCE.into()),
            };
            let gap = bits(&entry["gap_bits"])?;
            let state = if engaged { if positive { r <= 0.0 } else { r >= 0.0 } } else if positive { delta >= gap } else { delta <= -gap };
            (active(state), true, false, 0.0)
        }
        "friction" => {
            let mu = bits(&inputs["coefficient_bits"])?;
            let normal = bits(&inputs["normal_trial_bits"])?;
            let signed = bits(&inputs["normal_signed_bits"])?;
            let applied = bits(&inputs["applied_force_bits"])?;
            let derived = match inputs["normal_source"].as_str() {
                Some("explicit") => false,
                Some("derived") => true,
                _ => return Err(EVIDENCE.into()),
            };
            require(if derived { normal == signed.abs() } else { normal == signed }, DERIVATION)?;
            let zero_limit = mu * normal.abs() == 0.0;
            if normal <= 0.0 {
                return Ok(("inactive", true, zero_limit, normal));
            }
            if held != "sliding" {
                let state = if r.abs() <= mu * normal.abs() { "sticking" } else { "sliding" };
                return Ok((state, true, zero_limit, normal));
            }
            let sense = inputs["slip_sense"].as_i64().filter(|s| *s == 1 || *s == -1).ok_or_else(|| EVIDENCE.to_string())? as f64;
            let mut branch = true;
            if derived {
                let beta = inputs["beta"].as_i64().filter(|b| (-1..=1).contains(b)).ok_or_else(|| EVIDENCE.to_string())? as f64;
                branch = if mu == 0.0 || sense == 0.0 { true } else if beta == 0.0 { signed == 0.0 } else { beta * signed >= 0.0 };
                require(inputs["branch_admissible"] == branch, DERIVATION)?;
                if !branch {
                    return Ok(("sliding", false, zero_limit, normal));
                }
            }
            let limit = mu * normal;
            let sliding = if limit == 0.0 {
                applied == 0.0 && delta != 0.0
            } else {
                applied != 0.0 && delta != 0.0 && applied * delta < 0.0 && r * delta < 0.0
            };
            (if sliding { "sliding" } else { "sticking" }, branch, zero_limit, normal)
        }
        _ => return Err(EVIDENCE.into()),
    })
}

/// The strict-gap decision rule over the published exact signs and prior
/// (`structural_adapter.rs`): a closed gap stays closed unless its reaction is positive; an
/// open gap closes unless its gap sign is negative.
fn classify_strict(entry: &Value, held: &str) -> Result<&'static str, String> {
    let s = &entry["strict_gap"];
    require(exact_keys(s, &["prior", "gap_sign", "reaction_sign", "state"]) && s["prior"] == held, EVIDENCE)?;
    let sign = |k: &str| s[k].as_str().filter(|t| matches!(*t, "negative" | "zero" | "positive")).ok_or_else(|| EVIDENCE.to_string());
    let (gap_sign, reaction_sign) = (sign("gap_sign")?, sign("reaction_sign")?);
    let active = if held == "active" { reaction_sign != "positive" } else { gap_sign != "negative" };
    let decided = if active { "active" } else { "inactive" };
    require(s["state"] == decided, DERIVATION)?;
    Ok(decided)
}

/// kd5 §2.2's condition and class for a support held in `held` (§7.3, outcome E).
fn condition(behavior: &str, held: &str, derived: bool, zero_limit: bool) -> (&'static str, &'static str) {
    match (behavior, held) {
        ("one_way" | "lift_off", "active") => ("unilateral_contact_sign", "strict_ball"),
        ("one_way" | "lift_off", _) => ("unilateral_separation", "strict_ball"),
        ("friction", "inactive") => ("friction_normal_nonpositive", "input_decision"),
        ("friction", "sticking") if derived => ("stick_capacity_derived", "strict_ball"),
        ("friction", "sticking") => ("stick_capacity_explicit", "strict_ball"),
        ("friction", _) if zero_limit => ("slip_zero_limit", "strict_ball"),
        ("friction", _) if derived => ("slip_derived", "strict_ball"),
        ("friction", _) => ("slip_explicit", "strict_ball"),
        _ => ("gap_r1_not_run", "not_run"),
    }
}

fn read_entry(entry: &Value) -> Result<Entry<'_>, String> {
    let support_id = text(&entry["support_id"], EVIDENCE)?;
    text(&entry["node"], EVIDENCE)?;
    text(&entry["dof"], EVIDENCE)?;
    let behavior = text(&entry["behavior"], EVIDENCE)?;
    let held = state(&entry["held_state"])?;
    let classified = state(&entry["classified_state"])?;
    let released = entry["released"].as_str().filter(|r| matches!(*r, "none" | "changed_state" | "branch")).ok_or_else(|| EVIDENCE.to_string())?;
    let mut keys: Vec<&str> = vec!["support_id", "node", "dof", "behavior", "held_state", "classified_state", "decision", "released"];
    match behavior {
        "one_way" | "lift_off" => keys.push("sense"),
        "gap" => keys.extend(["closes_when", "gap_bits"]),
        "friction" => {}
        _ => return Err(EVIDENCE.into()),
    }
    let (decided, branch, zero_limit, normal_trial) = match entry["decision"].as_str() {
        Some("ordinary") => {
            keys.push("inputs");
            let mut input_keys = vec!["ground_bits", "displacement_bits", "reaction_bits"];
            if behavior == "friction" {
                input_keys.extend(["coefficient_bits", "normal_source", "normal_trial_bits", "normal_signed_bits", "applied_force_bits"]);
                let derived = entry["inputs"]["normal_source"] == "derived";
                if derived {
                    input_keys.push("normal_source_ref");
                }
                if held == "sliding" {
                    input_keys.push("slip_sense");
                    if derived {
                        input_keys.extend(["beta", "branch_admissible"]);
                    }
                }
            }
            require(exact_keys(&entry["inputs"], &input_keys), EVIDENCE)?;
            if let Some(source) = entry["inputs"].get("normal_source_ref") {
                // The derived normal's source, closed and typed: a support, its node and a DOF.
                require(exact_keys(source, &["support", "node", "dof"]), EVIDENCE)?;
                text(&source["support"], EVIDENCE)?;
                text(&source["node"], EVIDENCE)?;
                require(source["dof"].as_str().is_some_and(|d| DOFS.contains(&d)), EVIDENCE)?;
            }
            classify_ordinary(entry, behavior, held)?
        }
        Some("strict_gap") if behavior == "gap" => {
            keys.push("strict_gap");
            (classify_strict(entry, held)?, true, false, 0.0)
        }
        _ => return Err(EVIDENCE.into()),
    };
    require(exact_keys(entry, &keys), EVIDENCE)?;
    require(classified == decided, DERIVATION)?;
    let expected_release = if !branch { "branch" } else if classified != held { "changed_state" } else { "none" };
    require(released == expected_release, DERIVATION)?;
    Ok(Entry {
        support_id,
        behavior,
        held,
        released,
        gap: behavior == "gap",
        derived: entry["inputs"]["normal_source"] == "derived",
        zero_limit,
        normal_trial,
    })
}

/// The document's solver mode, from the minuend case's loop row (`<mode>_active_set_loop; …`).
fn loop_mode<'a>(rows: &'a [Value], case: &str) -> Option<&'a str> {
    rows.iter()
        .find(|r| r["kind"] == ITERATION_COUNT && case_of(r) == Some(case))
        .and_then(|r| r["metadata"]["basis"].as_str())
        .and_then(|b| b.split_once("_active_set_loop").map(|(mode, _)| mode))
}

/// A read record.
pub(crate) struct Record<'a> {
    pub id: &'a str,
    pub minuend: &'a str,
    pub subtrahend: &'a str,
    pub outcome: &'a str,
    held: Vec<Entry<'a>>,
}

fn read_record<'a>(record: &'a Value, source: &'a Value) -> Result<Record<'a>, String> {
    let admitted = record["outcome"] == "admitted";
    let mut keys = RECORD_KEYS.to_vec();
    if admitted && record.get("conditions").is_some() {
        keys.push("conditions");
    }
    require(exact_keys(record, &keys) && record["subtrahend_support_state"] == "minuend_final", EVIDENCE)?;
    let id = text(&record["combination_id"], EVIDENCE)?;
    let src = &record["source"];
    require(exact_keys(src, &["load_case_id", "solver_mode", "qualification", "standing"]), EVIDENCE)?;
    let minuend = text(&src["load_case_id"], EVIDENCE)?;
    text(&src["solver_mode"], EVIDENCE)?;
    require(src["qualification"].is_null() || matches!(src["qualification"].as_str(), Some("qualified" | "inspectable_unqualified_gap")), EVIDENCE)?;
    require(src["standing"].is_null() || matches!(src["standing"].as_str(), Some("confirmed" | "not_established")), EVIDENCE)?;
    let target = &record["target"];
    require(exact_keys(target, &["load_case_id", "modulus_basis_key"]) && (target["modulus_basis_key"].is_null() || target["modulus_basis_key"].is_string()), EVIDENCE)?;
    let subtrahend = text(&target["load_case_id"], EVIDENCE)?;
    let outcome = text(&record["outcome"], EVIDENCE)?;
    require(admitted || GATES.contains(&outcome), EVIDENCE)?;
    let standing = &record["standing"];
    require(exact_keys(standing, &["kind", "reason"]), EVIDENCE)?;
    let kind = text(&standing["kind"], EVIDENCE)?;
    let reason = text(&standing["reason"], EVIDENCE)?;
    let list = record["held"].as_array().ok_or_else(|| EVIDENCE.to_string())?;
    if !admitted {
        // A withheld record: an empty `held`, no solve and no conditions.
        require(list.is_empty() && record["held_solve"].is_null() && kind == "withheld", EVIDENCE)?;
        return Ok(Record { id, minuend, subtrahend, outcome, held: Vec::new() });
    }
    let solve = &record["held_solve"];
    require(exact_keys(solve, &["converged", "exit_diagnostic"]) && solve["converged"].is_boolean(), EVIDENCE)?;
    let exit = &solve["exit_diagnostic"];
    require(exit.is_null() || (exact_keys(exit, &["code", "message"]) && exit["code"].is_string() && exit["message"].is_string()), EVIDENCE)?;
    let mut held = Vec::new();
    let mut last: Option<&str> = None;
    for entry in list {
        let read = read_entry(entry)?;
        // Canonical order: by support id, each once.
        require(last.is_none_or(|l| l < read.support_id), EVIDENCE)?;
        last = Some(read.support_id);
        held.push(read);
    }
    require(!held.is_empty(), EVIDENCE)?;
    // The held iterate converges exactly when no support is released (§7.3: a release is
    // never promoted); a non-converged one carries the loop's exit diagnostic, a converged
    // one none.
    let any_released = held.iter().any(|e| e.released != "none");
    let converged = solve["converged"] == true;
    require(converged == !any_released, DERIVATION)?;
    if converged {
        require(exit.is_null(), DERIVATION)?;
    } else {
        require(exit["code"] == NONCONVERGENCE && exit["message"].as_str().is_some_and(|m| !m.trim().is_empty()), DERIVATION)?;
    }
    // Standing (§7.3): G with a gap; else R with a release; else E or N.
    let gap = held.iter().any(|e| e.gap);
    let expected_kind = if gap { "gap_inspection" } else if any_released { "not_established" } else { kind };
    require(kind == expected_kind && matches!(kind, "established" | "not_established" | "gap_inspection"), DERIVATION)?;
    let reason_ok = match kind {
        "established" => reason == ESTABLISHED,
        "gap_inspection" => reason == GAP_STANDING,
        _ if any_released => reason == RELEASED_STANDING,
        _ => reason.starts_with(NOT_ESTABLISHED_PREFIX) && reason.ends_with(NOT_ESTABLISHED_SUFFIX),
    };
    require(reason_ok, DERIVATION)?;
    // Conditions exactly at E, each by kd5 §2.2's table; an input decision from its inputs.
    require(record.get("conditions").is_some() == (kind == "established"), DERIVATION)?;
    if kind == "established" {
        let conditions = items(&record["conditions"]);
        require(conditions.len() == held.len(), EVIDENCE)?;
        for (c, e) in conditions.iter().zip(&held) {
            let (name, class) = condition(e.behavior, e.held, e.derived, e.zero_limit);
            let strict = class == "strict_ball";
            let keys: &[&str] = if strict && c.get("published_bits").is_some() {
                &["support_id", "condition", "class", "published_bits", "radius_bits"]
            } else {
                &["support_id", "condition", "class"]
            };
            require(exact_keys(c, keys) && c["support_id"] == e.support_id, EVIDENCE)?;
            require(c["condition"] == name && c["class"] == class, DERIVATION)?;
            if keys.len() == 5 {
                bits(&c["published_bits"])?;
                require(bits(&c["radius_bits"])? >= 0.0, EVIDENCE)?;
            }
            if class == "input_decision" {
                require(e.normal_trial <= 0.0, DERIVATION)?;
            }
        }
    }
    let _ = source;
    Ok(Record { id, minuend, subtrahend, outcome, held })
}

/// The member, read and bound to the document (both readings).
pub(crate) fn read(source: &Value) -> Result<Vec<Record<'_>>, String> {
    let evidence = &source["contract_evidence"];
    let Some(member) = evidence.get(MEMBER) else { return Ok(Vec::new()) };
    let list = member.as_array().filter(|l| !l.is_empty()).ok_or_else(|| EVIDENCE.to_string())?;
    let gates = items(&evidence["combination_gates"]);
    let position: HashMap<&str, usize> = gates.iter().enumerate().filter_map(|(i, g)| g["combination_id"].as_str().map(|id| (id, i))).collect();
    let declared: HashMap<&str, (&str, &str)> = items(evidence.get(crate::preview_difference_basis::MEMBER).unwrap_or(&Value::Null))
        .iter()
        .filter_map(|r| Some((r["combination_id"].as_str()?, (r["minuend"]["load_case_id"].as_str()?, r["subtrahend"]["load_case_id"].as_str()?))))
        .collect();
    let rows = items(&source["results"]);
    let diagnostics = items(&source["diagnostics"]);
    let mut records = Vec::new();
    let mut last: Option<usize> = None;
    for record in list {
        let read = read_record(record, source)?;
        let at = *position.get(read.id).ok_or_else(|| BINDING.to_string())?;
        require(last.is_none_or(|l| at > l), EVIDENCE)?;
        last = Some(at);
        // A declared subtraction's record: source = minuend, target = subtrahend.
        require(declared.get(read.id) == Some(&(read.minuend, read.subtrahend)), BINDING)?;
        let gate = &gates[at];
        let admitted = read.outcome == "admitted";
        require(gate["withheld"] == !admitted && (admitted == gate["reason"].is_null()) && (admitted || gate["reason"] == read.outcome), BINDING)?;
        provenance_check(record, &read, rows, diagnostics)?;
        if admitted {
            // The document's mode is the minuend's loop's.
            require(loop_mode(rows, read.minuend) == record["source"]["solver_mode"].as_str(), BINDING)?;
            // Coverage: exactly the minuend's published state-code rows, by support.
            let codes: Vec<&Value> = rows.iter().filter(|r| r["kind"] == STATE_CODE && case_of(r) == Some(read.minuend)).collect();
            require(codes.len() == read.held.len(), BINDING)?;
            for (entry, raw) in read.held.iter().zip(items(&record["held"])) {
                let code = codes.iter().find(|r| r["entity_ref"] == entry.support_id).ok_or_else(|| BINDING.to_string())?;
                require(code["value"].as_f64().and_then(code_state) == Some(entry.held), BINDING)?;
                require(code["metadata"]["location"].as_str().map(str::to_ascii_uppercase).as_deref() == raw["dof"].as_str(), BINDING)?;
            }
            // The held-operand rows exist, under the combination and no case.
            require(rows.iter().any(|r| is_held_row(r) && combination_of(r) == Some(read.id)), BINDING)?;
            // HELD_STATUS_RELEASED names exactly the released supports.
            let released: Vec<&str> = read.held.iter().filter(|e| e.released != "none").map(|e| e.support_id).collect();
            let notices: Vec<&Value> = diagnostics.iter().filter(|d| d["code"] == RELEASED && items(&d["affected_refs"]).first().is_some_and(|r| r == read.id)).collect();
            if released.is_empty() {
                require(notices.is_empty(), DERIVATION)?;
            } else {
                require(notices.len() == 1 && notices[0]["severity"] == "warning", DERIVATION)?;
                let named: Vec<&str> = items(&notices[0]["affected_refs"])[1..].iter().filter_map(Value::as_str).collect();
                require(named == released, DERIVATION)?;
            }
        } else {
            require(!diagnostics.iter().any(|d| d["code"] == RELEASED && items(&d["affected_refs"]).first().is_some_and(|r| r == read.id)), DERIVATION)?;
        }
        records.push(read);
    }
    Ok(records)
}

/// The record's provenance against the document (review P2-2, P2-1): the source's
/// qualification by the gate order (none when the minuend lent no evidence; past
/// qualification only when qualified), its standing confirmed exactly when the minuend's
/// integrity diagnostic carries the confirmed evidence line, the target's modulus key
/// as the subtrahend's modulus record in this combination states it, and each derived
/// normal's source as the minuend's derived-normal row names it.
fn provenance_check(record: &Value, read: &Record<'_>, rows: &[Value], diagnostics: &[Value]) -> Check {
    let src = &record["source"];
    let (qualification, standing) = (src["qualification"].as_str(), src["standing"].as_str());
    if read.outcome == UNAVAILABLE {
        require(qualification.is_none() && standing.is_none(), BINDING)?;
    } else {
        let wanted = if read.outcome == UNQUALIFIED { "inspectable_unqualified_gap" } else { "qualified" };
        require(qualification == Some(wanted), BINDING)?;
        let confirmed = diagnostics.iter().any(|d| {
            d["code"].as_str().is_some_and(|c| c.starts_with("NUMERICAL_INTEGRITY_"))
                && items(&d["affected_refs"]) == [Value::from(read.minuend)]
                && d["message"].as_str().is_some_and(|m| m.contains(crate::nonlinear_support_evidence::CONFIRMED))
        });
        require(standing == Some(if confirmed { "confirmed" } else { "not_established" }), BINDING)?;
    }
    let label = rows
        .iter()
        .find(|r| r["kind"] == MODULUS_RECORD && combination_of(r) == Some(read.id) && r["entity_ref"] == read.subtrahend)
        .and_then(|r| r["metadata"]["basis"].as_str());
    require(label.is_some_and(|l| modulus_label_states(record["target"]["modulus_basis_key"].as_str(), l)), BINDING)?;
    for raw in items(&record["held"]) {
        let derived = rows.iter().find(|r| r["kind"] == NORMAL_DERIVED && case_of(r) == Some(read.minuend) && r["entity_ref"] == raw["support_id"]);
        let Some(source) = raw["inputs"].get("normal_source_ref") else {
            require(derived.is_none(), BINDING)?;
            continue;
        };
        let basis = derived.and_then(|r| r["metadata"]["basis"].as_str()).ok_or_else(|| BINDING.to_string())?;
        let segment = |key: &str| basis.split("; ").find_map(|s| s.strip_prefix(key).and_then(|rest| rest.strip_prefix('=')));
        require(
            segment("source_ref") == source["support"].as_str() && segment("source_dof").map(str::to_ascii_uppercase).as_deref() == source["dof"].as_str(),
            BINDING,
        )?;
    }
    Ok(())
}

/// Whether an operand's modulus record label states this modulus key (product
/// `modulus_basis_key` and the record's basis label): none, the base material values;
/// `exact:<ref>`, the exact user-entered point `<ref>`; `temperature_kelvin_bits:<bits>`,
/// the declared solve temperature of exactly those bits.
fn modulus_label_states(key: Option<&str>, label: &str) -> bool {
    let Some(key) = key else { return label == BASE_MATERIAL };
    if let Some(reference) = key.strip_prefix("exact:") {
        return !reference.is_empty() && label.starts_with(&format!("temperature_point:{reference}; selection=exact_user_entered_point;"));
    }
    let Some(hex) = key.strip_prefix("temperature_kelvin_bits:") else { return false };
    let Some(rest) = label.strip_prefix("selection=declared_solve_temperature; solve_temperature_kelvin=") else { return false };
    let value = rest.split(';').next().and_then(|v| v.trim().parse::<f64>().ok());
    hex.len() == 16 && value.is_some_and(|v| format!("{:016x}", v.to_bits()) == hex)
}

/// Whether the document claims a held comparison anywhere.
fn claimed(source: &Value) -> bool {
    let evidence = &source["contract_evidence"];
    evidence.get(MEMBER).is_some()
        || items(&evidence["combination_gates"]).iter().any(|g| g["reason"].as_str().is_some_and(|r| GATES.contains(&r)))
        || items(&source["results"]).iter().any(|r| r["metadata"]["basis"] == OPERAND_BASIS || r["metadata"]["sign_convention"] == SIGN || is_held_row(r))
        || items(&source["diagnostics"]).iter().any(|d| d["code"] == RELEASED)
}

/// The rows (§7.6): the held sign only on an admitted record's difference rows; the typed
/// basis, and the operand clause, only on its held-operand rows; no case row is held.
fn rows_check(source: &Value, records: &[Record<'_>]) -> Check {
    let admitted: HashSet<&str> = records.iter().filter(|r| r.outcome == "admitted").map(|r| r.id).collect();
    for row in items(&source["results"]) {
        let held_row = is_held_row(row);
        let basis = &row["metadata"]["basis"];
        let sign = row["metadata"]["sign_convention"].as_str();
        match combination_of(row) {
            Some(id) if admitted.contains(id) && row["kind"] != MODULUS_RECORD => {
                if held_row {
                    let clause = format!("held-status operand of {id}: the subtrahend case's loads solved with the minuend's final support status held; not the subtrahend case's result");
                    require(*basis == OPERAND_BASIS && sign.is_some_and(|s| s == clause || s.ends_with(&format!("; {clause}"))), ROW)?;
                } else {
                    require(*basis != OPERAND_BASIS && (row["metadata"].is_null() || (sign == Some(SIGN) && *basis == SUBTRACTION_BASIS)), ROW)?;
                }
            }
            _ => require(!held_row && *basis != OPERAND_BASIS && sign != Some(SIGN), ROW)?,
        }
        if held_row {
            require(case_of(row).is_none(), ROW)?;
        }
    }
    Ok(())
}

/// Entry from `preview_combination_gates::check` on a solved document.
pub(crate) fn check(source: &Value, model: Option<&Value>) -> Check {
    if !claimed(source) {
        return Ok(());
    }
    let records = read(source)?;
    let recorded: HashSet<&str> = records.iter().map(|r| r.id).collect();
    // Every held gate has its record.
    for gate in items(&source["contract_evidence"]["combination_gates"]) {
        if gate["reason"].as_str().is_some_and(|r| GATES.contains(&r)) {
            require(gate["combination_id"].as_str().is_some_and(|id| recorded.contains(id)), EVIDENCE)?;
        }
    }
    rows_check(source, &records)?;
    match model {
        Some(model) => model_check(source, model, &records),
        None => Ok(()),
    }
}

/// The held gate a record states for `id` (`None` when admitted or unrecorded).
pub(crate) fn stated_gate<'a>(source: &'a Value, id: &str) -> Option<&'a str> {
    items(source["contract_evidence"].get(MEMBER).unwrap_or(&Value::Null))
        .iter()
        .find(|r| r["combination_id"] == id)
        .and_then(|r| r["outcome"].as_str())
        .filter(|o| GATES.contains(o))
}

/// Whether a model combination is a declared `minuend_final` subtraction.
pub(crate) fn minuend_final(combination: &Value) -> bool {
    crate::preview_difference_basis::declared(combination) && combination["subtrahend_support_state"] == "minuend_final"
}

fn quantity_bits(q: &Value, dimension: open_pipe_stress_units::Dimension) -> Result<f64, String> {
    crate::physics_source::normalized_quantity(q, dimension).map_err(|_| MODEL.to_string())
}

/// A case's ground offset on one support DOF: its imposed displacements there, normalized
/// (the solve's `ground_motion`), or none.
fn ground(case: &Value, support: &str, dof: &str) -> Result<Option<f64>, String> {
    let mut out: Option<f64> = None;
    for load in items(&case["primitive_loads"]) {
        if load["category"] == "imposed_displacement" && load["target"]["type"] == "support" && load["target"]["support"] == support
            && load["target"]["dof"].as_str().is_some_and(|d| d.eq_ignore_ascii_case(dof))
        {
            let dimension = if dof.starts_with('R') { open_pipe_stress_units::Dimension::Rotation } else { open_pipe_stress_units::Dimension::Displacement };
            let value = quantity_bits(&load["magnitude"], dimension)?;
            out = Some(out.unwrap_or(0.0) + value);
        }
    }
    Ok(out)
}

fn model_check(source: &Value, model: &Value, records: &[Record<'_>]) -> Check {
    let cases = items(&model["load_cases"]);
    let supports = items(&model["supports"]);
    let combinations = items(&model["combinations"]);
    let rows = items(&source["results"]);
    for record in records {
        let combination = combinations.iter().find(|c| c["id"] == record.id).ok_or_else(|| MODEL.to_string())?;
        require(minuend_final(combination) && combination["minuend_id"] == record.minuend && combination["subtrahend_id"] == record.subtrahend, MODEL)?;
        let case = |id: &str| cases.iter().find(|c| c["id"] == id).ok_or_else(|| MODEL.to_string());
        let (minuend, subtrahend) = (case(record.minuend)?, case(record.subtrahend)?);
        if [minuend, subtrahend].iter().any(|c| c.get("condition_use").is_some_and(|u| !u.is_null())) {
            return Err(UNVERIFIED.into());
        }
        let raw = items(&source["contract_evidence"][MEMBER]).iter().find(|r| r["combination_id"] == record.id).ok_or_else(|| MODEL.to_string())?;
        // The target's modulus key is the subtrahend case's own.
        let key = crate::preview_combination_gates::modulus_key(subtrahend).map_err(|_| MODEL.to_string())?;
        require(raw["target"]["modulus_basis_key"].as_str() == key.as_deref(), MODEL)?;
        if record.outcome == SLIDING_GROUND_DIFFERS {
            // Some support the minuend publishes sliding has differing grounds in the two cases.
            let differs = rows.iter().filter(|r| r["kind"] == STATE_CODE && case_of(r) == Some(record.minuend) && r["value"] == 3.0).any(|r| {
                let id = r["entity_ref"].as_str().unwrap_or_default();
                let dof = r["metadata"]["location"].as_str().unwrap_or_default().to_ascii_uppercase();
                matches!((ground(minuend, id, &dof), ground(subtrahend, id, &dof)), (Ok(a), Ok(b)) if a.map(f64::to_bits) != b.map(f64::to_bits))
            });
            require(differs, MODEL)?;
        }
        for (entry, raw_entry) in record.held.iter().zip(items(&raw["held"])) {
            let support = supports.iter().find(|s| s["id"] == entry.support_id).ok_or_else(|| MODEL.to_string())?;
            let nonlinear = &support["nonlinear"];
            require(raw_entry["node"] == support["node"] && raw_entry["behavior"] == nonlinear["behavior"], MODEL)?;
            let dof = nonlinear["dof"].as_str().ok_or_else(|| MODEL.to_string())?.to_ascii_uppercase();
            require(raw_entry["dof"] == dof.as_str(), MODEL)?;
            match entry.behavior {
                "one_way" => require(raw_entry["sense"] == nonlinear["active_when"], MODEL)?,
                "lift_off" => require(raw_entry["sense"] == nonlinear["contact_when"], MODEL)?,
                "gap" => {
                    require(raw_entry["closes_when"] == nonlinear["closes_when"], MODEL)?;
                    let gap = quantity_bits(&nonlinear["gap"], open_pipe_stress_units::Dimension::Displacement)?;
                    require(bits(&raw_entry["gap_bits"])?.to_bits() == gap.to_bits(), MODEL)?;
                }
                _ => {
                    let inputs = &raw_entry["inputs"];
                    let mu = nonlinear["friction_coefficient"]["value"].as_f64().ok_or_else(|| MODEL.to_string())?;
                    require(bits(&inputs["coefficient_bits"])?.to_bits() == mu.to_bits(), MODEL)?;
                    match nonlinear.get("normal_reaction").filter(|n| !n.is_null()) {
                        Some(n) => {
                            let n = quantity_bits(n, open_pipe_stress_units::Dimension::Force)?;
                            require(inputs["normal_source"] == "explicit" && bits(&inputs["normal_signed_bits"])?.to_bits() == n.to_bits(), MODEL)?;
                        }
                        None => {
                            // The derived normal's source is the model's own assignment.
                            let assigned = &nonlinear["normal_reaction_source"];
                            let stated = &inputs["normal_source_ref"];
                            let source_support = supports.iter().find(|s| s["id"] == assigned["support_ref"]).ok_or_else(|| MODEL.to_string())?;
                            require(
                                inputs["normal_source"] == "derived"
                                    && stated["support"] == assigned["support_ref"]
                                    && assigned["dof"].as_str().map(str::to_ascii_uppercase).as_deref() == stated["dof"].as_str()
                                    && stated["node"] == source_support["node"],
                                MODEL,
                            )?;
                        }
                    }
                }
            }
            if raw_entry["decision"] == "ordinary" {
                // The trial's ground term: the target case's own.
                let g = ground(subtrahend, entry.support_id, &dof)?.unwrap_or(0.0);
                require(bits(&raw_entry["inputs"]["ground_bits"])?.to_bits() == g.to_bits(), MODEL)?;
            }
            if entry.held == "sliding" {
                let (gs, gt) = (ground(minuend, entry.support_id, &dof)?, ground(subtrahend, entry.support_id, &dof)?);
                require(gs.map(f64::to_bits) == gt.map(f64::to_bits), MODEL)?;
            }
        }
    }
    // Every declared `minuend_final` combination the model admits past G-1 and G-2 has its record.
    let (kept, order) = crate::preview_difference_basis::kept_combinations(model);
    let recorded: HashSet<&str> = records.iter().map(|r| r.id).collect();
    for &index in &order {
        let combination = &kept[index];
        if minuend_final(combination) && crate::preview_difference_basis::model_gate(model, combination)?.0.is_none() {
            require(combination["id"].as_str().is_some_and(|id| recorded.contains(id)), MODEL)?;
        }
    }
    Ok(())
}

#[cfg(test)]
#[path = "preview_held_status_tests.rs"]
mod tests;
