//! Shared helpers of the S4 combination tests: building a combination request from a frozen
//! reference's published signed contributions, and comparing at the verification tolerance.
//!
//! **Reference keys.** The references name their rows by composed keys (`N3.uy`, `P1.i.Fx`,
//! `P1.i.Fx_m`, `A1.Fx`). [`quantity`] maps a reference key to the product's structured row and
//! [`reference_key`] maps it back, for these tests' lookups in the reference's own tables only: the
//! product identifies rows by their fields and parses no key. A reference's reaction key
//! `<support>.<Fx..Mz>` is the constraint reaction at the node that support restrains, held by it,
//! or the compliant support force of a spring or finite snubber. The rows of a support acting along a skew
//! direction (C4's and C10's skew springs, `K*.Fx`, `K*.Fy`, `K*.Fz`, `K*.Fd`) are not REQ-006 rows
//! and are left out of every request ([`skew_supports`]); every other row is compared as before.
//!
//! **The tolerance** is REQ-008's verification tolerance, 1e-9 relative to the expected value,
//! with no floor: a value the reference publishes as exactly 0 must be produced as exactly 0.
//! The references flush a value below 1e-50 of its unit's largest magnitude to 0, and the
//! product's exact arithmetic reproduces each of those zeros exactly (the cluster sums of such
//! rows are exactly zero), so no zero needs an allowance.
#![allow(dead_code)]
use open_pipe_stress_response_spectrum::{
    ClusterIndex, Combination, CombinationRequest, Direction, DirectionalRule, DisplacementComponent, End,
    ForceComponent, Frame, MemberFamily, MissingMass, ModalContribution, ModalMode, ModalRule, ModeIndex, Quantity,
    ReactionComponent, RigidContribution, RigidRule,
};
use std::collections::BTreeSet;
use serde_json::Value;
use std::collections::BTreeMap;

pub const TOLERANCE: f64 = 1e-9;

/// A reference pair `[decimal, binary64]`: the binary64, used for inputs.
pub fn input(pair: &Value) -> f64 {
    pair[1].as_f64().unwrap_or_else(|| panic!("binary64 of {pair}"))
}

/// A reference pair's 40-digit decimal parsed to its nearest binary64, used for expected values.
pub fn expected(pair: &Value) -> f64 {
    pair[0].as_str().unwrap_or_else(|| panic!("decimal of {pair}")).parse().expect("a decimal")
}

const REACTIONS: [(&str, ReactionComponent); 6] = [
    ("Fx", ReactionComponent::Ux),
    ("Fy", ReactionComponent::Uy),
    ("Fz", ReactionComponent::Uz),
    ("Mx", ReactionComponent::Rx),
    ("My", ReactionComponent::Ry),
    ("Mz", ReactionComponent::Rz),
];

/// The product row of a reference key in `case`: `N3.uy`, `P1.i.Fx` (global axes), `P1.i.Fx_m`
/// (member axes), and a support's reaction `A1.Fx`. A reaction of a support that restrains that
/// component rigidly is the constraint reaction at the restrained node, held by that support; a
/// reaction of a spring or a finite snubber is that support's compliant support force.
pub fn quantity(case: &Value, key: &str) -> Quantity {
    let parts: Vec<&str> = key.split('.').collect();
    match parts.as_slice() {
        [object, component] => {
            if let Some(c) = DisplacementComponent::parse(component) {
                return Quantity::NodeDisplacement { node: object.to_string(), component: c };
            }
            let c = REACTIONS.iter().find(|(n, _)| n == component).unwrap_or_else(|| panic!("component of {key}")).1;
            let dof = c.as_str().to_ascii_lowercase();
            let restrained = case["derived_model"]["restrained_dofs"].as_array().unwrap().iter()
                .find(|r| r[2] == *object && r[1] == dof.as_str());
            if let Some(r) = restrained {
                return Quantity::ConstraintReaction { node: r[0].as_str().unwrap().to_string(), component: c, holders: vec![object.to_string()] };
            }
            let model = &case["model"];
            let compliant = model["springs"].as_array().into_iter().flatten()
                .chain(model["snubbers"].as_array().into_iter().flatten().filter(|s| !s["stiffness"].is_null()))
                .any(|s| s["id"] == *object);
            assert!(compliant, "{key}: neither a rigid restraint of that component nor a compliant support");
            Quantity::CompliantSupportForce { support: object.to_string(), component: c }
        }
        [member, end, component] => {
            let (component, frame) = match component.strip_suffix("_m") {
                Some(c) => (c, Frame::MemberAxes),
                None => (*component, Frame::GlobalAxes),
            };
            Quantity::ElementEndAction {
                member: member.to_string(),
                family: MemberFamily::Straight,
                end: if *end == "i" { End::I } else { End::J },
                component: ForceComponent::parse(component).unwrap_or_else(|| panic!("component of {key}")),
                frame,
            }
        }
        _ => panic!("unrecognized key {key}"),
    }
}

/// The reference key of a row [`quantity`] made, for lookups in the reference's tables.
pub fn reference_key(q: &Quantity) -> String {
    match q {
        Quantity::NodeDisplacement { node, component } => format!("{node}.{}", component.as_str()),
        Quantity::ElementEndAction { member, end, component, frame, .. } => {
            let suffix = if *frame == Frame::MemberAxes { "_m" } else { "" };
            format!("{member}.{}.{}{suffix}", end.as_str(), component.as_str())
        }
        Quantity::ConstraintReaction { component, holders, .. } => {
            format!("{}.{}", holders[0], REACTIONS.iter().find(|(_, c)| c == component).unwrap().0)
        }
        Quantity::CompliantSupportForce { support, component } => {
            format!("{support}.{}", REACTIONS.iter().find(|(_, c)| c == component).unwrap().0)
        }
        other => panic!("no reference key for {other:?}"),
    }
}

/// The supports of a discrete case that act along a skew direction: its skew springs and its
/// snubbers with a vector direction. Their rows are not stated by REQ-006.
pub fn skew_supports(case: &Value) -> BTreeSet<String> {
    let model = &case["model"];
    let mut out = BTreeSet::new();
    for s in model["skew_springs"].as_array().into_iter().flatten() {
        out.insert(s["id"].as_str().unwrap().to_string());
    }
    for s in model["snubbers"].as_array().into_iter().flatten() {
        if s["direction"].is_array() {
            out.insert(s["id"].as_str().unwrap().to_string());
        }
    }
    out
}

pub fn modal_rule(name: &str) -> ModalRule {
    match name {
        "ABSSUM" => ModalRule::AbsoluteSum,
        other => ModalRule::parse(other).unwrap_or_else(|| panic!("modal rule {other}")),
    }
}

pub fn directional_rule(name: &str) -> DirectionalRule {
    DirectionalRule::parse(name).unwrap_or_else(|| panic!("directional rule {name}"))
}

/// One analysis of a discrete reference, read into the inputs of a combination.
pub struct DiscreteAnalysis {
    pub id: String,
    pub damping: f64,
    pub directions: Vec<Direction>,
    pub modes: Vec<ModalMode>,
    pub keys: Vec<String>,
    /// The product row of each key, in key order.
    pub rows: Vec<Quantity>,
    /// (direction, mode) → values in key order, for every mode with published values.
    pub contributions: BTreeMap<(Direction, u32), Vec<f64>>,
    /// The modes the reference classifies as periodic.
    pub periodic_modes: Vec<u32>,
    pub rigid_frequency: Option<f64>,
    pub rigid: BTreeMap<Direction, Vec<f64>>,
}

/// Reads an analysis; `block` selects the global blocks (`None`) or a named sub-block such as
/// `element_end_forces_member_axes`.
pub fn discrete_analysis(case: &Value, analysis: &Value, block: Option<&str>) -> DiscreteAnalysis {
    let request = &analysis["request"];
    let spectra = case["spectra"].as_array().unwrap();
    let directions: Vec<Direction> = request["directions"]
        .as_array()
        .unwrap()
        .iter()
        .map(|d| Direction::parse(d["direction"].as_str().unwrap()).unwrap())
        .collect();
    let dampings: Vec<f64> = request["directions"]
        .as_array()
        .unwrap()
        .iter()
        .map(|d| {
            let id = d["spectrum"].as_str().unwrap();
            spectra.iter().find(|s| s["id"] == id).unwrap()["damping_ratio"].as_f64().unwrap()
        })
        .collect();
    assert!(dampings.windows(2).all(|w| w[0] == w[1]));
    let scope_clusters = case["modal_basis"]["scope_clusters"].as_array().unwrap();
    let cluster_of = |mode: u64| -> u32 {
        let i = scope_clusters
            .iter()
            .position(|c| c["modes"].as_array().unwrap().iter().any(|m| m.as_u64() == Some(mode)))
            .unwrap_or_else(|| panic!("mode {mode} in no scope cluster"));
        i as u32 + 1
    };
    let modes: Vec<ModalMode> = analysis["mode_status"]
        .as_array()
        .unwrap()
        .iter()
        .map(|m| {
            let mode = m["mode"].as_u64().unwrap();
            ModalMode { mode: ModeIndex(mode as u32), cluster: ClusterIndex(cluster_of(mode)), frequency_hz: input(&m["frequency_Hz"]) }
        })
        .collect();
    let periodic_modes = analysis["mode_status"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|m| m["status"] == "periodic")
        .map(|m| m["mode"].as_u64().unwrap() as u32)
        .collect();
    let source = match block {
        None => analysis,
        Some(name) => &analysis[name],
    };
    let signed = source["contributions_signed"].as_object().unwrap();
    let first = signed.values().next().unwrap().as_array().unwrap();
    let skew = skew_supports(case);
    let keys: Vec<String> = first
        .iter()
        .find_map(|e| e["values"].as_object())
        .expect("a mode with values")
        .keys()
        .filter(|k| !skew.contains(k.split('.').next().unwrap()))
        .cloned()
        .collect();
    let mut contributions = BTreeMap::new();
    for (d, entries) in signed {
        let d = Direction::parse(d).unwrap();
        for e in entries.as_array().unwrap() {
            let Some(values) = e["values"].as_object() else { continue };
            let mode = e["mode"].as_u64().expect("a single mode") as u32;
            contributions.insert((d, mode), keys.iter().map(|k| input(&values[k])).collect());
        }
    }
    let rigid_frequency = request["missing_mass"]["rigid_frequency_Hz"].as_f64();
    let mut rigid = BTreeMap::new();
    if let Some(r) = source.get("rigid_response_signed").and_then(Value::as_object) {
        for (d, values) in r {
            rigid.insert(Direction::parse(d).unwrap(), keys.iter().map(|k| input(&values[k])).collect());
        }
    }
    DiscreteAnalysis {
        id: request["id"].as_str().unwrap().to_string(),
        damping: dampings[0],
        directions,
        modes,
        rows: keys.iter().map(|k| quantity(case, k)).collect(),
        keys,
        contributions,
        periodic_modes,
        rigid_frequency,
        rigid,
    }
}

impl DiscreteAnalysis {
    /// The request under the given rules; `rigid` is the rigid rule, `None` for the correction
    /// off (every extracted mode periodic).
    pub fn request(&self, modal: ModalRule, rigid: Option<RigidRule>, directional: DirectionalRule) -> CombinationRequest {
        let missing_mass = match rigid {
            None => MissingMass::NotApplied,
            Some(rule) => MissingMass::Applied { rigid_frequency_hz: self.rigid_frequency.expect("f_r"), rule },
        };
        let periodic_set: Vec<u32> = match rigid {
            None => self.modes.iter().map(|m| m.mode.0).collect(),
            Some(_) => self.periodic_modes.clone(),
        };
        let mut periodic = Vec::new();
        for &d in &self.directions {
            for &m in &periodic_set {
                let values = self.contributions.get(&(d, m)).unwrap_or_else(|| panic!("{}: mode {m} {d} has no values", self.id)).clone();
                periodic.push(ModalContribution { mode: ModeIndex(m), direction: d, values });
            }
        }
        let left_out: Vec<ClusterIndex> = {
            let mut c: Vec<ClusterIndex> =
                self.modes.iter().filter(|m| periodic_set.contains(&m.mode.0)).map(|m| m.cluster).collect();
            c.dedup();
            c
        };
        let rigid_responses = match rigid {
            None => Vec::new(),
            Some(_) => self
                .directions
                .iter()
                .map(|&d| RigidContribution { direction: d, periodic_clusters_left_out: left_out.clone(), values: self.rigid[&d].clone() })
                .collect(),
        };
        CombinationRequest {
            damping_ratio: self.damping,
            modal_rule: modal,
            directional_rule: directional,
            missing_mass,
            directions: self.directions.clone(),
            modes: self.modes.clone(),
            rows: self.rows.clone(),
            periodic,
            rigid: rigid_responses,
        }
    }

    /// The operand scale of each row: Σ|value| over every contribution and rigid response.
    pub fn scales(&self) -> Vec<f64> {
        let mut s = vec![0.0; self.keys.len()];
        for values in self.contributions.values().chain(self.rigid.values()) {
            for (a, v) in s.iter_mut().zip(values) {
                *a += v.abs();
            }
        }
        s
    }
}

/// The comparison tally: how many values were compared and the largest deviation, relative to
/// the expected value and as a fraction of the tolerance.
#[derive(Default, Debug)]
pub struct Tally {
    pub compared: usize,
    pub largest_relative: f64,
    pub largest_of_tolerance: f64,
    /// Where the largest deviation relative to the expected value occurred.
    pub largest_relative_at: String,
}

impl Tally {
    pub fn check(&mut self, what: &str, actual: f64, expected: f64) {
        let allowed = TOLERANCE * expected.abs();
        let deviation = (actual - expected).abs();
        assert!(deviation <= allowed, "{what}: {actual:e} against {expected:e} (deviation {deviation:e}, allowed {allowed:e})");
        self.compared += 1;
        if expected != 0.0 {
            let relative = deviation / expected.abs();
            if relative > self.largest_relative {
                self.largest_relative = relative;
                self.largest_relative_at = what.to_string();
            }
            self.largest_of_tolerance = self.largest_of_tolerance.max(deviation / allowed);
        }
    }
}

/// The combined magnitude of the row with `key` in a combination, over directions or in one.
pub fn combined_of(c: &Combination, keys: &[String]) -> BTreeMap<String, (f64, BTreeMap<Direction, (f64, Option<f64>, f64)>)> {
    let mut out = BTreeMap::new();
    for (row, key) in c.rows.iter().zip(keys) {
        assert_eq!(&reference_key(&row.quantity), key);
        let per = row
            .per_direction
            .iter()
            .map(|p| (p.direction, (p.periodic.value(), p.rigid.map(|m| m.value()), p.combined.value())))
            .collect();
        out.insert(key.clone(), (row.combined.value(), per));
    }
    out
}

/// Exact alternating C(n-1,k) cluster sums, encoded as two binary64 contributions per cluster.
/// Integer recurrence and 53-bit chunks avoid the rounded high-order binomial recurrence.
/// Scaling by a power of two retains the exact represented sum; n <= 100 keeps u128 products safe.
pub fn exact_binomial_request(n: usize, scale_exponent: i32) -> CombinationRequest {
    assert!((1..=100).contains(&n));
    let mut modes = Vec::new();
    let mut periodic = Vec::new();
    let mut coefficient = 1u128;
    for k in 0..n {
        let high = coefficient >> 53;
        let low = coefficient & ((1u128 << 53) - 1);
        assert!(high < (1u128 << 53));
        let sign = if k % 2 == 0 { 1.0 } else { -1.0 };
        for part in [high as f64 * 2f64.powi(53), low as f64] {
            let mode = ModeIndex(modes.len() as u32 + 1);
            modes.push(ModalMode { mode, cluster: ClusterIndex(k as u32 + 1),
                frequency_hz: 1.0 + k as f64 * 2f64.powi(-19) });
            let value = sign * part * 2f64.powi(scale_exponent);
            assert!(value.is_finite());
            periodic.push(ModalContribution { mode, direction: Direction::X, values: vec![value] });
        }
        coefficient = coefficient.checked_mul((n - 1 - k) as u128).unwrap() / (k + 1) as u128;
    }
    CombinationRequest { damping_ratio: 0.2, modal_rule: ModalRule::Cqc,
        directional_rule: DirectionalRule::Srss, missing_mass: MissingMass::NotApplied,
        directions: vec![Direction::X], modes,
        rows: vec![Quantity::NodeDisplacement { node: "N1".into(), component: DisplacementComponent::Ux }],
        periodic, rigid: vec![] }
}
