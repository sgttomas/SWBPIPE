//! S11-G, the formation-noise guard (`S11G_GUARD.md` revision 2.1, selected
//! by ROOT; I5 rulings recorded in `ROOT_RULINGS_V1.md`).
//!
//! Two guards demote a load case to `NUMERICAL_INTEGRITY_SENSITIVE` when a
//! published value may carry more formation noise than the 1e-9 criterion
//! allows. Nothing is refused, nothing is `Err`, and no envelope field, code
//! or enum value is added: a fired case keeps every value and gains one
//! reason sentence in its existing integrity message.
//!
//! - **The load-row guard** (section 3) reads the case ledger's formation
//!   records (`AssembledForce::formation_rows`): per loaded row, the exact
//!   12-scaled defects of the formed terms in two accumulators (net, and
//!   self-equilibrated), the bound sum B and the self-equilibrated magnitude
//!   P. A row fires when any of these holds, each decided exactly:
//!   - `B > 0` and `B >= T0` (the `B > 0` guard is ROOT's erratum ruling to
//!     revision 2.1's first clause: at `B = T0 = 0` a zero bound claims
//!     nothing, which keeps design test T6 silent);
//!   - `|A_net| + 12 B - 12 T0 > 0` (V1 D21-1: ±12B and ∓12T0 are added
//!     exactly into a copy of the accumulator, no rounded comparison);
//!   - `|A_se| - 12 Tf > 0`;
//!   - a `CannotBound` term (curved consistent vectors, SF-2) or a range
//!     failure at the row.
//!
//!   `T0 = RD(1e-9) * max(|n|, S*)` and `Tf = RD(1e-9) * max(|n|, S*, 2^-10 P)`,
//!   both rounded downward, with |n| a lower bound of the row's intended net
//!   and S* the section 4.1.6 coupled body scale of the intended nets (free
//!   rows over the body's free rows, restrained rows over all its loaded
//!   rows). The floor applies to the self-equilibrated defect only (DB-1).
//! - **The recovery guard R-b'** (section 4) reads each straight member end
//!   whose published bending rows come from the formed K_e * u: q =
//!   hypot(My, Mz), B = `bending_formation_bound`. It fires when
//!   `B > RD(1e-9) q` (exact), `q > 2^10 B` and `q >= 2^-34 S*_moment`, with
//!   S*_moment the body's coupled moment scale over the case's published
//!   rows (DESIGN section 4.1.6.1).
use super::{Diagnostic, ResultItem, DOF_PER_NODE};
use open_pipe_stress_frame_kernel::exact_sum::ExactAccumulator;
use open_pipe_stress_frame_kernel::load_ledger::{
    product_downward, quotient_downward, AssembledForce, FormationRow,
};
use std::collections::{HashMap, HashSet};

/// RD(10^-9): the largest binary64 value not above 10^-9. The binary64
/// literal `1e-9` lies 6.2e-17 (relative) above 10^-9 (V1 DN-3), so the
/// threshold uses the value one step below it.
pub(crate) const CRITERION: f64 = f64::from_bits(1e-9_f64.to_bits() - 1);
/// 2^-10: the self-equilibrated floor factor (SF-4).
const FLOOR_FACTOR: f64 = 1.0 / 1024.0;
/// 2^10: R-b's resolution factor.
const RESOLUTION_FACTOR: f64 = 1024.0;
/// 2^-34: the DESIGN V1-S8/S8-R floor R.
const DESIGN_FLOOR: f64 = f64::from_bits(0x3DD0_0000_0000_0000);
/// S* below 2^-988 makes the floor product inexact (DESIGN N-2): every row of
/// that body is below the floor.
const DESIGN_FLOOR_MIN_SCALE: f64 = f64::from_bits((1023 - 988) << 52);
const PASSED: &str = "NUMERICAL_INTEGRITY_CHECKS_PASSED";
const SENSITIVE: &str = "NUMERICAL_INTEGRITY_SENSITIVE";
/// At most this many fired rows or ends are named in a sentence.
pub(crate) const NAMED: usize = 6;

/// Which guard fired.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Guard {
    LoadRow,
    Recovery,
}

/// One case's finding: the guard, the reason sentence appended to the
/// integrity message, and the fired items (for tests and the sentence).
#[derive(Debug, Clone)]
pub(crate) struct FormationFinding {
    /// Read by tests; the verdict itself needs only the sentence.
    #[allow(dead_code)]
    pub(crate) guard: Guard,
    pub(crate) sentence: String,
    /// Every fired row or end (the sentence names the first few).
    #[allow(dead_code)]
    pub(crate) fired: Vec<String>,
}

// ------------------------------------------------------------------ bodies

/// Connected bodies of the element graph (DESIGN section 4.1.6.1 item 1):
/// straight members, curved spans and user stiffness elements connect their
/// nodes; springs, restraints and imposed motions do not.
pub(crate) struct Bodies {
    body_of_node: Vec<usize>,
    extent: Vec<f64>,
}

impl Bodies {
    pub(crate) fn new(coordinates: &[[f64; 3]], edges: &[(usize, usize)]) -> Self {
        let n = coordinates.len();
        let mut parent: Vec<usize> = (0..n).collect();
        fn root(parent: &mut [usize], mut x: usize) -> usize {
            while parent[x] != x {
                parent[x] = parent[parent[x]];
                x = parent[x];
            }
            x
        }
        for &(a, b) in edges {
            if a < n && b < n {
                let (ra, rb) = (root(&mut parent, a), root(&mut parent, b));
                if ra != rb {
                    parent[ra.max(rb)] = ra.min(rb);
                }
            }
        }
        let mut ids = HashMap::new();
        let mut body_of_node = Vec::with_capacity(n);
        for node in 0..n {
            let r = root(&mut parent, node);
            let next = ids.len();
            body_of_node.push(*ids.entry(r).or_insert(next));
        }
        let bodies = ids.len();
        let mut low = vec![[f64::INFINITY; 3]; bodies];
        let mut high = vec![[f64::NEG_INFINITY; 3]; bodies];
        for (node, point) in coordinates.iter().enumerate() {
            let body = body_of_node[node];
            for axis in 0..3 {
                low[body][axis] = low[body][axis].min(point[axis]);
                high[body][axis] = high[body][axis].max(point[axis]);
            }
        }
        // DESIGN item 5: d_a = fl(max_a - min_a); L_b = fl(sqrt(fl(fl(fl(dx dx)
        // + fl(dy dy)) + fl(dz dz)))).
        let extent = (0..bodies)
            .map(|body| {
                let d = [0, 1, 2].map(|axis| high[body][axis] - low[body][axis]);
                ((d[0] * d[0] + d[1] * d[1]) + d[2] * d[2]).sqrt()
            })
            .collect();
        Self {
            body_of_node,
            extent,
        }
    }

    pub(crate) fn body_of_node(&self, node: usize) -> Option<usize> {
        self.body_of_node.get(node).copied()
    }

    fn extent(&self, body: usize) -> f64 {
        self.extent[body]
    }
}

/// DESIGN section 4.1.6 coupling, rounded downward on the coupled terms:
/// (fo, mo) = (max(F, M/L_b), max(M, L_b F)); L_b = 0 omits the coupling.
fn coupled(force: f64, moment: f64, extent: f64) -> (f64, f64) {
    if extent > 0.0 && extent.is_finite() {
        (
            force.max(quotient_downward(moment, extent)),
            moment.max(product_downward(extent, force)),
        )
    } else {
        (force, moment)
    }
}

// ------------------------------------------------------------ load-row guard

/// One row's exact decision and its approximate statistics (the numeric
/// fields are read by tests and kept for diagnostics).
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub(crate) struct RowDecision {
    pub(crate) fires: bool,
    /// T0 (unfloored) and Tf (floored), rounded downward.
    pub(crate) t0: f64,
    pub(crate) tf: f64,
    /// (|A_net|/12 + B) / T0 and (|A_se|/12) / Tf, approximately (text and
    /// tests only; the decision above is exact).
    pub(crate) net_ratio: f64,
    pub(crate) self_equilibrated_ratio: f64,
    pub(crate) reason: Option<String>,
}

/// |defect| + 12 bound - 12 threshold > 0, decided exactly. A non-finite
/// operand decides for firing.
fn exceeds(defect: &ExactAccumulator, bound: f64, threshold: f64) -> bool {
    let positive = defect.signum() >= 0;
    let sign = if positive { 1.0 } else { -1.0 };
    let mut copy = defect.clone();
    if copy.add_product(sign * 12.0, bound).is_err()
        || copy.add_product(-sign * 12.0, threshold).is_err()
    {
        return true;
    }
    if positive {
        copy.signum() > 0
    } else {
        copy.signum() < 0
    }
}

fn ratio(numerator: f64, denominator: f64) -> f64 {
    if numerator == 0.0 {
        0.0
    } else if denominator == 0.0 {
        f64::INFINITY
    } else {
        numerator / denominator
    }
}

fn twelfth(accumulator: &ExactAccumulator) -> f64 {
    accumulator
        .round()
        .map_or(f64::INFINITY, |v| (v / 12.0).abs())
}

/// The exact decision for one loaded row at scale `s_star` (section 3.4, with
/// ROOT's `B > 0` erratum on the first clause).
pub(crate) fn decide_row(row: &FormationRow, s_star: f64) -> RowDecision {
    let base = row.intended_net_lower.max(s_star);
    let floor = product_downward(row.self_equilibrated_magnitude, FLOOR_FACTOR);
    let t0 = product_downward(CRITERION, base);
    let tf = product_downward(CRITERION, base.max(floor));
    let net = twelfth(&row.net_defect);
    let self_equilibrated = twelfth(&row.self_equilibrated_defect);
    let mut decision = RowDecision {
        fires: false,
        t0,
        tf,
        net_ratio: ratio(net + row.bound, t0),
        self_equilibrated_ratio: ratio(self_equilibrated, tf),
        reason: None,
    };
    let reason = if let Some(failure) = row.range_failure {
        Some(format!("range failure ({failure})"))
    } else if !row.cannot_bound_sources.is_empty() {
        Some(format!(
            "no conservative formation bound (CannotBound: curved-span consistent load vector from {:?})",
            row.cannot_bound_sources
        ))
    } else if !row.formed {
        None
    } else if (row.bound > 0.0 && row.bound >= t0)
        || exceeds(&row.net_defect, row.bound, t0)
        || exceeds(&row.self_equilibrated_defect, 0.0, tf)
    {
        Some(format!(
            "net formation defect {net:e}, self-equilibrated defect {self_equilibrated:e}, bound {:e}, intended net at least {:e}, body scale {s_star:e}, threshold {t0:e} (self-equilibrated threshold {tf:e}); formed sources {:?}",
            row.bound, row.intended_net_lower, row.formed_sources
        ))
    } else {
        None
    };
    decision.fires = reason.is_some();
    decision.reason = reason;
    decision
}

/// Per loaded row, its body scale S* (section 3.4): free rows couple the
/// body's free-row intended nets; restrained rows couple all of its loaded
/// rows. Lower bounds of |n| are used, and the coupled terms round downward.
pub(crate) fn row_scales(
    rows: &[FormationRow],
    bodies: &Bodies,
    restrained: &HashSet<usize>,
) -> HashMap<usize, f64> {
    // body -> [free force, free moment, all force, all moment]
    let mut maxima: HashMap<usize, [f64; 4]> = HashMap::new();
    for row in rows {
        let Some(body) = bodies.body_of_node(row.dof / DOF_PER_NODE) else {
            continue;
        };
        let n = row.intended_net_lower;
        let moment = row.dof % DOF_PER_NODE >= 3;
        let entry = maxima.entry(body).or_insert([0.0; 4]);
        let all = if moment { 3 } else { 2 };
        entry[all] = entry[all].max(n);
        if !restrained.contains(&row.dof) {
            let free = if moment { 1 } else { 0 };
            entry[free] = entry[free].max(n);
        }
    }
    rows.iter()
        .filter_map(|row| {
            let body = bodies.body_of_node(row.dof / DOF_PER_NODE)?;
            let m = maxima[&body];
            let (force, moment) = if restrained.contains(&row.dof) {
                (m[2], m[3])
            } else {
                (m[0], m[1])
            };
            let (fo, mo) = coupled(force, moment, bodies.extent(body));
            let scale = if row.dof % DOF_PER_NODE >= 3 { mo } else { fo };
            Some((row.dof, scale))
        })
        .collect()
}

/// The load-row guard's finding for one case, or `None` when no row fires.
pub(crate) fn load_row_finding(
    force: &AssembledForce,
    bodies: &Bodies,
    restrained_dofs: &[usize],
    dof_label: impl Fn(usize) -> String,
    case_id: &str,
) -> Option<FormationFinding> {
    if !force.has_formation_records() {
        return None;
    }
    let rows = force.formation_rows();
    let restrained: HashSet<usize> = restrained_dofs.iter().copied().collect();
    let scales = row_scales(&rows, bodies, &restrained);
    let mut fired = Vec::new();
    for row in &rows {
        let s_star = scales.get(&row.dof).copied().unwrap_or(0.0);
        let decision = decide_row(row, s_star);
        if let Some(reason) = decision.reason {
            fired.push(format!("{}: {reason}", dof_label(row.dof)));
        }
    }
    (!fired.is_empty()).then(|| FormationFinding {
        guard: Guard::LoadRow,
        sentence: format!(
            "S11-G formation-noise guard: load case {case_id} is Sensitive because {} load row(s) carry formed-term formation noise above the 1e-9 criterion; every value is kept for inspection. Rows: [{}]{}",
            fired.len(),
            fired.iter().take(NAMED).cloned().collect::<Vec<_>>().join("; "),
            if fired.len() > NAMED { format!("; and {} more", fired.len() - NAMED) } else { String::new() }
        ),
        fired,
    })
}

// ------------------------------------------------------------ recovery guard

/// One straight member's ends as published from the formed K_e * u.
#[derive(Debug, Clone)]
pub(crate) struct RecoveryRecord {
    pub(crate) member: String,
    pub(crate) body: Option<usize>,
    /// Per end (i, j): q = hypot(My, Mz) of the published rows, and B (or the
    /// reason B is unavailable, which fires).
    pub(crate) ends: [(f64, Result<f64, String>); 2],
}

/// DESIGN section 4.1.6.1 closed table, force and moment classes only, with
/// the pinned unit factors. `None` for every other (kind, unit).
fn action_class(kind: &str, unit: &str) -> Option<(bool, f64)> {
    let force_factor = match unit {
        "N" => Some(1.0),
        "kN" => Some(1e3),
        _ => None,
    };
    let moment_factor = match unit {
        "N*m" => Some(1.0),
        "kN*m" => Some(1e3),
        _ => None,
    };
    match kind {
        "element_local_axial_force"
        | "element_local_shear_force_y"
        | "element_local_shear_force_z"
        | "pipe_wall_axial_force_v2"
        | "pipe_effective_axial_force_v2" => force_factor.map(|f| (false, f)),
        "support_reaction_force_magnitude_v2" | "reaction_resultant" => {
            (unit == "N").then_some((false, 1.0))
        }
        "element_local_torsional_moment"
        | "element_local_bending_moment_y"
        | "element_local_bending_moment_z" => moment_factor.map(|f| (true, f)),
        "support_reaction_moment_magnitude_v2" => (unit == "N*m").then_some((true, 1.0)),
        "support_reaction_component_v2" | "pipe_wall_endpoint_action_v2" => force_factor
            .map(|f| (false, f))
            .or_else(|| moment_factor.map(|f| (true, f))),
        _ => None,
    }
}

/// S*_moment per body over the case's published rows (DESIGN section
/// 4.1.6.1): S(force) and S(moment) are the largest |row| of each class in
/// the body; mo = max(S(moment), L_b S(force)). Rows of `excluded` members
/// (curved spans, entity rule 2b) and rows whose entity has no body are not
/// covered.
pub(crate) fn moment_scales(
    results: &[ResultItem],
    body_of_entity: &HashMap<String, usize>,
    excluded: &HashSet<String>,
    bodies: &Bodies,
) -> HashMap<usize, f64> {
    let mut maxima: HashMap<usize, (f64, f64)> = HashMap::new();
    for row in results {
        let Some((moment, factor)) = action_class(&row.kind, &row.unit) else {
            continue;
        };
        if excluded.contains(&row.entity_ref) {
            continue;
        }
        let Some(&body) = body_of_entity.get(&row.entity_ref) else {
            continue;
        };
        let value = (row.value * factor).abs();
        let entry = maxima.entry(body).or_insert((0.0, 0.0));
        if moment {
            entry.1 = entry.1.max(value);
        } else {
            entry.0 = entry.0.max(value);
        }
    }
    maxima
        .into_iter()
        .map(|(body, (force, moment))| {
            // DESIGN item 6: mo = max(S_mo, fl(L_b S_fo)).
            let extent = bodies.extent(body);
            let mo = if extent > 0.0 {
                moment.max(extent * force)
            } else {
                moment
            };
            (body, mo)
        })
        .collect()
}

/// R-b' for one end: true when it fires (a non-finite B fires, fail-closed).
pub(crate) fn rb_prime_fires(q: f64, bound: f64, s_star_moment: f64) -> bool {
    if !bound.is_finite() {
        return true;
    }
    // B > RD(1e-9) q, exactly.
    let mut first = ExactAccumulator::new();
    let first_clause =
        first.add(bound).is_ok() && first.add_product(-CRITERION, q).is_ok() && first.signum() > 0;
    let resolved = q > RESOLUTION_FACTOR * bound;
    let above_floor = s_star_moment >= DESIGN_FLOOR_MIN_SCALE && q >= DESIGN_FLOOR * s_star_moment;
    first_clause && resolved && above_floor
}

/// The recovery guard's finding for one case, or `None` when no end fires.
pub(crate) fn recovery_finding(
    records: &[RecoveryRecord],
    scales: &HashMap<usize, f64>,
    case_id: &str,
) -> Option<FormationFinding> {
    let mut fired = Vec::new();
    for record in records {
        let s_star = record
            .body
            .and_then(|body| scales.get(&body).copied())
            .unwrap_or(0.0);
        for (end, (q, bound)) in ["i", "j"].iter().zip(&record.ends) {
            match bound {
                Ok(bound) => {
                    if rb_prime_fires(*q, *bound, s_star) {
                        fired.push(format!(
                            "{}.{end}: q={q:e} N*m, B={bound:e} N*m, S*_moment={s_star:e} N*m",
                            record.member
                        ));
                    }
                }
                Err(reason) => fired.push(format!(
                    "{}.{end}: the formation bound is unavailable ({reason})",
                    record.member
                )),
            }
        }
    }
    (!fired.is_empty()).then(|| FormationFinding {
        guard: Guard::Recovery,
        sentence: format!(
            "S11-G recovery guard (R-b'): load case {case_id} is Sensitive because {} straight member end bending row(s) published from the formed K_e*u carry formation noise above the 1e-9 criterion; every value is kept for inspection. Ends: [{}]{}",
            fired.len(),
            fired.iter().take(NAMED).cloned().collect::<Vec<_>>().join("; "),
            if fired.len() > NAMED { format!("; and {} more", fired.len() - NAMED) } else { String::new() }
        ),
        fired,
    })
}

// ------------------------------------------------------------------ verdict

/// Demotes an integrity diagnostic that is still `CHECKS_PASSED` to
/// `SENSITIVE` (warning) and appends the reason sentence. The no-op rule: any
/// other code (already Sensitive or weaker) is left byte for byte as it is.
/// Returns whether it demoted.
pub(crate) fn demote(diagnostic: &mut Diagnostic, finding: &FormationFinding) -> bool {
    if diagnostic.code != PASSED {
        return false;
    }
    diagnostic.code = SENSITIVE.to_string();
    diagnostic.severity = "warning".to_string();
    diagnostic.message = format!("{} {}", diagnostic.message, finding.sentence);
    true
}

/// R-b' after the recovery loop: demotes the case's integrity diagnostic
/// (found by id) under the no-op rule.
pub(crate) fn amend_integrity_report(
    diagnostics: &mut [Diagnostic],
    integrity_id: &str,
    finding: &FormationFinding,
) -> bool {
    diagnostics
        .iter_mut()
        .find(|d| d.id == integrity_id)
        .is_some_and(|d| demote(d, finding))
}
