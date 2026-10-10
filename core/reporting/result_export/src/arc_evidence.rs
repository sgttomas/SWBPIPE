//! T4-U2: realized-arc members of a `pressure-1` (`3.0.0/exact_pressure_v3`)
//! pressure region. A region with a realized arc carries the tangency rule,
//! the bend-adjacent junctions and the withheld-on-arcs statement beyond a
//! straight region's keys; each arc member carries `member_kind` and its end
//! tangents in the region geometry and its own applied-load record (the bend
//! term K_b*u_free(eps_p) - c_b). On an arc N_w = N_el + pAi and S = N_el, the
//! end rows are along the end tangents, and the straight Lame rows and the
//! straight-statics maximum are withheld.
//!
//! These are code constants shared in meaning with the producer
//! (`product_physics` `pressure_runtime.rs`, `bend_pressure.rs`) and with the
//! Python and TypeScript readers. The reader checks bindings and declared
//! semantics, never values: no table and no static is added (T4-RV14 F1).
use serde_json::Value;
use std::collections::HashSet;

type Check = Result<(), String>;

pub(crate) const MEMBER_KIND: &str = "realized_arc";
pub(crate) const APPROXIMATION: &str =
    "straight_lame_annulus_and_realized_arc_member_term_h2_small_strain_v3";
pub(crate) const TANGENCY_TOLERANCE_RAD: f64 = 1.0e-3;
pub(crate) const TANGENCY_RULE: &str = "a region changes direction only at a realized bend; each straight run keeps the 64*epsilon collinearity guard (a straight-straight kink is PRESSURE_REGION_NONCOLLINEAR); at each bend-adjacent junction theta = atan2(|t_in x t_out|, t_in . t_out) <= alpha_tan is admitted and carried exactly by the remainder pAi(t_in - t_out), and a larger theta is refused as a mitre (PRESSURE_REGION_MITRE_UNSUPPORTED)";
pub(crate) const WITHHELD_REASON: &str = "withheld on realized arcs: the straight Lame hoop and radial values do not hold on a torus (a toroidal membrane hoop is a later unit), and the straight-statics maximum does not bound an arc";
pub(crate) const WITHHELD_KINDS: [&str; 3] = [
    "pipe_lame_radial_stress_v2",
    "pipe_lame_hoop_stress_v2",
    "pipe_elastic_normal_stress_maximum_v2",
];
pub(crate) const BEND_TERM: &str = "K_b*u_free(eps_p)-c_b";
pub(crate) const STRAIN_DEFINITION: &str = "(1-2nu)pAi/(E As)";
/// The region keys a realized arc adds.
pub(crate) const REGION_KEYS: [&str; 4] = [
    "tangency_tolerance_rad",
    "tangency_rule",
    "bend_adjacent_junctions",
    "withheld_on_arcs",
];
/// The geometry keys an arc member adds.
pub(crate) const GEOMETRY_KEYS: [&str; 2] = ["member_kind", "arc_end_tangents_global"];
const APPLIED_KEYS: [&str; 8] = [
    "pipe_id",
    "bend_term",
    "arc_pressure_strain",
    "arc_pressure_strain_definition",
    "mathematical_cap_pair_local_n",
    "arc_end_tangents_global",
    "bend_cap_pair_removed_global_n",
    "thermal_included",
];
const JUNCTION_KEYS: [&str; 6] = ["node_ref", "pipe_in", "pipe_out", "t_in_global", "t_out_global", "theta_rad"];

/// The sign convention of a pressure-family row on an arc member.
pub(crate) fn sign(kind: &str) -> Option<&'static str> {
    Some(match kind {
        "pipe_wall_endpoint_action_v2" => "node-on-element wall action along the arc end tangent (local x toward end j at that end); N_w = N_el + pAi; cap transfer is not subtracted from wall recovery",
        "pipe_wall_axial_force_v2" => "tension-positive material wall section resultant Nw = N_el + pAi along the arc tangent; shear and moments are the elastic section actions",
        "pipe_effective_axial_force_v2" => "effective wall-fluid resultant S=Nw-pAi (the arc's elastic axial force); not material stress or a support reaction",
        "pipe_axial_membrane_stress_v2" => "tension-positive axial wall membrane stress Nw/As on the arc section; no added longitudinal pressure scalar",
        _ => return None,
    })
}

fn require(ok: bool, code: &str) -> Check {
    if ok {
        Ok(())
    } else {
        Err(format!("SOURCE_PHYSICS_ARC_{code}"))
    }
}
fn keys(v: &Value, required: &[&str]) -> bool {
    v.as_object()
        .is_some_and(|o| o.len() == required.len() && required.iter().all(|k| o.contains_key(*k)))
}
fn vector3(v: &Value) -> Option<[f64; 3]> {
    let a = v.as_array().filter(|a| a.len() == 3)?;
    let mut out = [0.0; 3];
    for (slot, x) in out.iter_mut().zip(a) {
        *slot = x.as_f64().filter(|x| x.is_finite())?;
    }
    Some(out)
}
fn pair3(v: &Value) -> Option<[[f64; 3]; 2]> {
    let a = v.as_array().filter(|a| a.len() == 2)?;
    Some([vector3(&a[0])?, vector3(&a[1])?])
}
/// Exact binary64 values; a signed zero equals zero (as in Python and
/// TypeScript, whose JSON loaders may not keep the sign of a zero).
fn bits(a: [f64; 3]) -> [f64; 3] {
    a
}

/// The arc members of one region, in geometry order. A `member_kind` other
/// than `realized_arc`, or any arc under the v2 contract, is refused.
pub(crate) fn members<'a>(region: &'a Value, admitted: bool) -> Result<HashSet<&'a str>, String> {
    let mut arcs = HashSet::new();
    for g in region["geometry"].as_array().map(Vec::as_slice).unwrap_or(&[]) {
        if g.get("member_kind").is_none() && g.get("arc_end_tangents_global").is_none() {
            continue;
        }
        require(admitted, "UNSUPPORTED")?;
        require(
            g["member_kind"] == MEMBER_KIND && pair3(&g["arc_end_tangents_global"]).is_some(),
            "GEOMETRY",
        )?;
        arcs.insert(g["pipe_id"].as_str().unwrap_or_default());
    }
    Ok(arcs)
}

/// Exact region keys: a straight region's, plus the arc keys when (and only
/// when) the region has a realized arc.
pub(crate) fn region_shape(region: &Value, base: &[&str], has_arc: bool) -> bool {
    if !has_arc {
        return keys(region, base);
    }
    let all: Vec<&str> = base.iter().copied().chain(REGION_KEYS).collect();
    keys(region, &all)
}

/// The region-level arc statements and the arc members' applied loads.
/// `geometry` is the region's geometry list (arc entries carry the tangents);
/// `p` is the region pressure.
pub(crate) fn region(region: &Value, arcs: &HashSet<&str>, p: f64) -> Check {
    require(
        region["approximation"] == APPROXIMATION
            && region["tangency_tolerance_rad"].as_f64() == Some(TANGENCY_TOLERANCE_RAD)
            && region["tangency_rule"] == TANGENCY_RULE
            && keys(&region["withheld_on_arcs"], &["result_kinds", "reason"])
            && region["withheld_on_arcs"]["reason"] == WITHHELD_REASON
            && region["withheld_on_arcs"]["result_kinds"] == serde_json::json!(WITHHELD_KINDS),
        "REGION_PROFILE",
    )?;
    let geometry = region["geometry"].as_array().map(Vec::as_slice).unwrap_or(&[]);
    let tangents = |pipe: &str| {
        geometry
            .iter()
            .find(|g| g["pipe_id"] == pipe)
            .and_then(|g| pair3(&g["arc_end_tangents_global"]))
    };
    let loads = region["applied_loads"].as_array().map(Vec::as_slice).unwrap_or(&[]);
    // Each member's (start, end) direction in traversal order: an arc's end
    // tangents, a straight member's local x; reversed members negated.
    let mut directions: Vec<(&str, [f64; 3], [f64; 3])> = Vec::new();
    for member in region["member_pipe_ids"].as_array().map(Vec::as_slice).unwrap_or(&[]) {
        let pipe = member.as_str().unwrap_or_default();
        let load = loads.iter().find(|l| l["pipe_id"] == pipe).ok_or("SOURCE_PHYSICS_ARC_APPLIED_LOAD")?;
        let forward = geometry
            .iter()
            .find(|g| g["pipe_id"] == pipe)
            .and_then(|g| g["traversal_forward"].as_bool())
            .ok_or("SOURCE_PHYSICS_ARC_GEOMETRY")?;
        let [t_i, t_j] = if arcs.contains(pipe) {
            applied(load, tangents(pipe).ok_or("SOURCE_PHYSICS_ARC_GEOMETRY")?, p)?
        } else {
            let axis = vector3(&load["local_x_global"]).ok_or("SOURCE_PHYSICS_ARC_APPLIED_LOAD")?;
            [axis, axis]
        };
        directions.push(if forward {
            (pipe, t_i, t_j)
        } else {
            (pipe, t_j.map(|v| -v), t_i.map(|v| -v))
        });
    }
    // Every adjacent pair with an arc is one junction, in traversal order,
    // within the tangency tolerance, with the members' own end directions.
    let expected: Vec<usize> = (1..directions.len())
        .filter(|&k| arcs.contains(directions[k - 1].0) || arcs.contains(directions[k].0))
        .collect();
    let junctions = region["bend_adjacent_junctions"]
        .as_array()
        .ok_or("SOURCE_PHYSICS_ARC_JUNCTIONS")?;
    require(junctions.len() == expected.len(), "JUNCTIONS")?;
    for (junction, &k) in junctions.iter().zip(&expected) {
        let theta = junction["theta_rad"].as_f64().unwrap_or(f64::NAN);
        require(
            keys(junction, &JUNCTION_KEYS)
                && junction["node_ref"].as_str().is_some_and(|s| !s.is_empty())
                && junction["pipe_in"] == directions[k - 1].0
                && junction["pipe_out"] == directions[k].0
                && (0.0..=TANGENCY_TOLERANCE_RAD).contains(&theta),
            "JUNCTIONS",
        )?;
        require(
            vector3(&junction["t_in_global"]).map(bits) == Some(bits(directions[k - 1].2))
                && vector3(&junction["t_out_global"]).map(bits) == Some(bits(directions[k].1)),
            "JUNCTION_TANGENT",
        )?;
    }
    Ok(())
}

/// An arc member's applied-load record; returns its end tangents.
fn applied(load: &Value, tangents: [[f64; 3]; 2], p: f64) -> Result<[[f64; 3]; 2], String> {
    let caps = load["mathematical_cap_pair_local_n"]
        .as_array()
        .filter(|a| a.len() == 2)
        .and_then(|a| Some([a[0].as_f64()?, a[1].as_f64()?]));
    let strain = load["arc_pressure_strain"].as_f64().filter(|x| x.is_finite());
    require(
        keys(load, &APPLIED_KEYS)
            && load["bend_term"] == BEND_TERM
            && load["arc_pressure_strain_definition"] == STRAIN_DEFINITION
            && load["thermal_included"] == false
            && caps.is_some_and(|c| c.iter().all(|x| x.is_finite()))
            // (1-2nu) > 0 for an admitted nu, so eps_p has the sign of p.
            && strain.is_some_and(|e| (e > 0.0) == (p > 0.0) && (e < 0.0) == (p < 0.0)),
        "APPLIED_LOAD",
    )?;
    require(
        pair3(&load["arc_end_tangents_global"]).map(|t| t.map(bits)) == Some(tangents.map(bits)),
        "APPLIED_LOAD",
    )?;
    let caps = caps.unwrap();
    // -c_b: the arc's own end caps along its end tangents, removed.
    let removed = [tangents[0].map(|v| -v * caps[0]), tangents[1].map(|v| -v * caps[1])];
    require(
        pair3(&load["bend_cap_pair_removed_global_n"]).map(|r| r.map(bits)) == Some(removed.map(bits)),
        "APPLIED_LOAD",
    )?;
    Ok(tangents)
}

/// The pressure rows an arc member publishes: (kind, components, locations).
pub(crate) const ROWS: [(&str, &str, &[&str]); 4] = [
    ("pipe_wall_endpoint_action_v2", "wall_axial_end_action", &["end_i", "end_j"]),
    ("pipe_wall_axial_force_v2", "wall_axial_force", &STATIONS),
    ("pipe_effective_axial_force_v2", "effective_axial_force", &STATIONS),
    ("pipe_axial_membrane_stress_v2", "axial_membrane_stress", &STATIONS),
];
const STATIONS: [&str; 5] = ["end_i", "end_j", "quarter_1", "midspan", "quarter_3"];

/// One pressure-RHS term under v3: the bend's removed caps (`bend_cap_removed`,
/// an arc), the junction remainder (`kink_remainder`, a member of a
/// bend-adjacent junction) and the straight Poisson term (never on an arc).
/// Returns `None` for a kind this module does not own.
pub(crate) fn rhs_term(kind: &str, pipe: &str, coefficient: f64, region: &Value, arcs: &HashSet<&str>) -> Option<Check> {
    Some(match kind {
        "bend_cap_removed" => require(arcs.contains(pipe) && coefficient.abs() == 1.0, "RHS_TERM"),
        "kink_remainder" => {
            let at_junction = region["bend_adjacent_junctions"]
                .as_array()
                .is_some_and(|j| j.iter().any(|j| j["pipe_in"] == pipe || j["pipe_out"] == pipe));
            require(at_junction && coefficient.abs() == 1.0, "RHS_TERM")
        }
        "poisson_eigen" if arcs.contains(pipe) => require(false, "RHS_TERM"),
        _ => return None,
    })
}
