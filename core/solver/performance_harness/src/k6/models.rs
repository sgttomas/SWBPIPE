//! K6's sealed kernel models (T3 D1 §4.8; ROOT's K6 rulings Q10 and N4).
//!
//! - **RF-LARGE** (24 cases): R1's families (`T3/REFERENCES/references.py`,
//!   sha256 `80d473a7…`; `lg_chain_defn`, `lg_tree_defn`, `continuous_defn`,
//!   `rot_defn`), with every node and member in R1's insertion order. Every
//!   coordinate and load is built in exact integer arithmetic (the rotated
//!   cases through Q3 with an exact division by 3) and converted to binary64
//!   once; each is exactly representable.
//! - **The DEC-053 nine**: the harness's own invented fixtures, taken from the
//!   crate's DEC-053 specification list, unchanged.
//! - **Outside the sealed set**: the Q4 ceiling chain (RF-LARGE-CHAIN geometry,
//!   1,364 members) and the Q5 invented grids.
//!
//! No function of unspecified precision is used: the section is formed from
//! explicit products and `std::f64::consts::PI` only.

use crate::{invented_grid_frame_fixture, BenchmarkFixture};
use open_pipe_stress_frame_kernel::{
    FrameElement, FrameKernelError, FrameNode, FrameSection, DOF_PER_NODE,
};
use std::f64::consts::PI;

/// The source label of R1's models (references.json sha256 `7b176dbb…`).
pub const R1_SOURCE: &str = "r1:references.json@7b176dbb";
/// RF-LARGE's member counts.
pub const RF_LARGE_SIZES: [usize; 4] = [10, 100, 1000, 10000];
/// The Q4 ceiling chain's member count (8,190 DOFs; ROOT's K6 ruling Q4).
pub const CEILING_CHAIN_MEMBERS: usize = 1364;
/// The Q5 grid sides (ROOT's K6 ruling N14; 128 is conditional).
pub const Q5_GRID_SIDES: [usize; 5] = [16, 32, 64, 96, 128];
/// The CONT size whose identity-order lane is never run (Q12).
pub const CONT_LANE_REFUSAL_MEMBERS: usize = 10_000;

/// The model family, for the runner's admission ratios.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Family {
    Chain,
    Tree,
    Cont,
    Dec053,
    Grid,
}

impl Family {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Chain => "CHAIN",
            Self::Tree => "TREE",
            Self::Cont => "CONT",
            Self::Dec053 => "DEC053",
            Self::Grid => "GRID",
        }
    }
}

/// A kernel model: one section, nodes and members in their authored order,
/// restraints and nodal loads (global DOF, value).
#[derive(Debug, Clone, PartialEq)]
pub struct K6Model {
    pub id: String,
    pub source: String,
    pub family: Family,
    pub section: FrameSection,
    /// (label, coordinates), in the model's node order (node index = position).
    pub nodes: Vec<(String, [f64; 3])>,
    /// (label, node i, node j, y_reference), in the model's member order.
    pub members: Vec<(String, usize, usize, [f64; 3])>,
    /// (node, restrained UX UY UZ RX RY RZ), ascending node.
    pub restraints: Vec<(usize, [bool; 6])>,
    /// (global DOF, value), ascending DOF, nonzero only.
    pub loads: Vec<(usize, f64)>,
}

impl K6Model {
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }
    pub fn member_count(&self) -> usize {
        self.members.len()
    }
    pub fn dofs(&self) -> usize {
        self.nodes.len() * DOF_PER_NODE
    }
    /// The frames, formed by `FrameElement::new` in member order.
    pub fn frames(&self) -> Result<Vec<FrameElement>, FrameKernelError> {
        self.members
            .iter()
            .map(|(_, i, j, y)| {
                FrameElement::new(
                    FrameNode::new(*i, self.nodes[*i].1)?,
                    FrameNode::new(*j, self.nodes[*j].1)?,
                    self.section,
                    *y,
                )
            })
            .collect()
    }
    /// The restrained global DOFs, ascending.
    pub fn restrained_dofs(&self) -> Vec<usize> {
        self.restraints
            .iter()
            .flat_map(|(node, mask)| {
                (0..DOF_PER_NODE)
                    .filter(move |&d| mask[d])
                    .map(move |d| node * DOF_PER_NODE + d)
            })
            .collect()
    }
    /// The global nodal force vector (one term per loaded DOF).
    pub fn force_vector(&self) -> Vec<f64> {
        let mut force = vec![0.0; self.dofs()];
        for &(dof, value) in &self.loads {
            force[dof] = value;
        }
        force
    }
    /// R1's CONT case at 10,000 members or more, whose identity-order lane is
    /// never run (ROOT's K6 ruling Q12, restated with RV16-N4).
    ///
    /// Decided from the model's content, not its id (RV18-N6), since a model
    /// file carries its own id: the family is CONT (as generated, or as R1's
    /// id names it), or the model carries CONT's restraint signature. That
    /// signature is m + 1 nodes, the first fully fixed and the next m/2 pinned
    /// in UX, UY and UZ, and nothing else restrained. The size is the member
    /// count.
    pub fn is_cont_n10000(&self) -> bool {
        self.member_count() >= CONT_LANE_REFUSAL_MEMBERS
            && (self.family == Family::Cont || self.has_cont_restraints())
    }

    fn has_cont_restraints(&self) -> bool {
        let supports = self.member_count() / 2;
        self.node_count() == self.member_count() + 1
            && self.restraints.len() == supports + 1
            && self.restraints.iter().enumerate().all(|(k, (node, mask))| {
                *node == k && *mask == if k == 0 { ALL_SIX } else { PIN_THREE }
            })
    }
}

/// The 33 sealed models: RF-LARGE's 24 (family, size, orientation), then the
/// DEC-053 nine in the harness's order.
pub fn sealed_model_ids() -> Vec<String> {
    let mut ids = Vec::new();
    for family in ["CHAIN", "TREE", "CONT"] {
        for n in RF_LARGE_SIZES {
            for orientation in ["AX", "ROT"] {
                ids.push(format!("RF-LARGE-{family}-n{n:05}-{orientation}"));
            }
        }
    }
    for spec in crate::sparse_default_promotion_fixture_specs() {
        if let Ok(fixture) = spec.fixture() {
            ids.push(format!("DEC053:{}", fixture.fixture_id));
        }
    }
    ids
}

/// The models outside the sealed set: the Q4 ceiling chain and the Q5 grids.
pub fn extra_model_ids() -> Vec<String> {
    let mut ids = vec![format!("K6-CEIL-CHAIN-n{CEILING_CHAIN_MEMBERS:05}-AX")];
    for side in Q5_GRID_SIDES {
        ids.push(format!("K6-GRID-{side}x{side}"));
    }
    ids
}

/// Builds a model by id.
pub fn model(id: &str) -> Result<K6Model, String> {
    model_described(id).map(|(model, _)| model)
}

/// Construction provenance from the branch that actually created the model.
/// Kept separately so model/element representation and allocation remain unchanged.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ModelOrigin(pub(crate) ModelRecipe);
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ModelRecipe {
    Builder,
    Fixture,
    Canonical,
}

pub fn model_described(id: &str) -> Result<(K6Model, ModelOrigin), String> {
    if let Some(rest) = id.strip_prefix("RF-LARGE-") {
        return rf_large(id, rest).map(|m| (m, ModelOrigin(ModelRecipe::Builder)));
    }
    if let Some(fixture_id) = id.strip_prefix("DEC053:") {
        for spec in crate::sparse_default_promotion_fixture_specs() {
            let fixture = spec.fixture().map_err(|e| format!("{e:?}"))?;
            if fixture.fixture_id == fixture_id {
                return from_fixture(id, &format!("dec053:{fixture_id}"), Family::Dec053, fixture)
                    .map(|m| (m, ModelOrigin(ModelRecipe::Fixture)));
            }
        }
        return Err(format!("unknown DEC-053 fixture: {fixture_id}"));
    }
    if id == format!("K6-CEIL-CHAIN-n{CEILING_CHAIN_MEMBERS:05}-AX") {
        let (s, s2) = chain_exponents(CEILING_CHAIN_MEMBERS).ok_or("no chain exponents")?;
        return Ok((
            chain(
                id,
                "k6-invented:rf-large-chain-rule",
                CEILING_CHAIN_MEMBERS,
                false,
                s,
                s2,
            ),
            ModelOrigin(ModelRecipe::Builder),
        ));
    }
    if let Some(rest) = id.strip_prefix("K6-GRID-") {
        let side: usize = rest
            .split_once('x')
            .filter(|(a, b)| a == b)
            .and_then(|(a, _)| a.parse().ok())
            .filter(|s| Q5_GRID_SIDES.contains(s))
            .ok_or_else(|| format!("unknown grid: {id}"))?;
        let fixture = invented_grid_frame_fixture(side, side).map_err(|e| format!("{e:?}"))?;
        return from_fixture(
            id,
            "k6-invented:invented_grid_frame_fixture",
            Family::Grid,
            fixture,
        )
        .map(|m| (m, ModelOrigin(ModelRecipe::Fixture)));
    }
    Err(format!("unknown model id: {id}"))
}

fn rf_large(id: &str, rest: &str) -> Result<K6Model, String> {
    let parts: Vec<&str> = rest.split('-').collect();
    let [family, size, orientation] = parts.as_slice() else {
        return Err(format!("malformed RF-LARGE id: {id}"));
    };
    let n: usize = size
        .strip_prefix('n')
        .filter(|digits| digits.len() == 5)
        .and_then(|digits| digits.parse().ok())
        .filter(|n| RF_LARGE_SIZES.contains(n))
        .ok_or_else(|| format!("unknown RF-LARGE size: {id}"))?;
    let rotated = match *orientation {
        "AX" => false,
        "ROT" => true,
        _ => return Err(format!("unknown orientation: {id}")),
    };
    match *family {
        "CHAIN" => {
            let (s, s2) = chain_exponents(n).ok_or("no chain exponents")?;
            Ok(chain(id, R1_SOURCE, n, rotated, s, s2))
        }
        "TREE" => Ok(tree(
            id,
            n,
            rotated,
            tree_exponent(n).ok_or("no tree exponent")?,
        )),
        "CONT" => Ok(cont(id, n, rotated)),
        _ => Err(format!("unknown RF-LARGE family: {id}")),
    }
}

/// CHAIN's load exponents (s, s2): R1's `round(log2(1.264/n²))` and
/// `round(log2(24.5/n))` (`lg_chain_defn`), tabulated and re-derived by exact
/// rational comparison (2^(2k-1) <= x² < 2^(2k+1)). They reproduce the tip
/// loads of `references.json` at n = 10, 100, 1,000 and 10,000. n = 1,364 is
/// the Q4 chain, by the same rule.
pub fn chain_exponents(n: usize) -> Option<(i32, i32)> {
    match n {
        10 => Some((-6, 1)),
        100 => Some((-13, -2)),
        1000 => Some((-20, -5)),
        10000 => Some((-26, -9)),
        CEILING_CHAIN_MEMBERS => Some((-20, -6)),
        _ => None,
    }
}

/// TREE's load exponent sc, which R1 derives from its exact solution
/// (`round(log2(1e-4/max_rotation))`, `references.py:1889`). It is data, read
/// from `references.json`'s purpose strings and checked by the recorded
/// cross-check against `references.py --model`.
pub fn tree_exponent(n: usize) -> Option<i32> {
    match n {
        10 => Some(0),
        100 => Some(-6),
        1000 => Some(-14),
        10000 => Some(-20),
        _ => None,
    }
}

/// The RF-LARGE section: E = 200 GPa, G = 80 GPa, OD = 0.2 m and ID = 0.18 m,
/// each rounded once; A, I and J from explicit products and `PI` only:
/// `A = PI*(od²-id²)/4`, `I = PI*(od²·od² - id²·id²)/64`, `J = 2I`, with
/// od² = od*od and id² = id*id, evaluated left to right.
pub fn rf_large_section() -> FrameSection {
    let elastic_modulus = 200_000_000_000.0_f64;
    let shear_modulus = 80_000_000_000.0_f64;
    let od = 0.2_f64;
    let id = 0.18_f64;
    let od2 = od * od;
    let id2 = id * id;
    let area = PI * (od2 - id2) / 4.0;
    let second_moment = PI * (od2 * od2 - id2 * id2) / 64.0;
    let torsion_constant = 2.0 * second_moment;
    FrameSection::new(
        elastic_modulus,
        shear_modulus,
        area,
        second_moment,
        second_moment,
        torsion_constant,
    )
    .expect("the RF-LARGE section is positive and finite")
}

/// Q3 = (1/3)[[1,2,2],[2,1,-2],[-2,2,-1]] (`references.py:1298`) applied to an
/// integer vector, with the division by 3 checked exact.
fn q3(v: [i64; 3]) -> [i64; 3] {
    let r = [
        v[0] + 2 * v[1] + 2 * v[2],
        2 * v[0] + v[1] - 2 * v[2],
        -2 * v[0] + 2 * v[1] - v[2],
    ];
    assert!(
        r.iter().all(|c| c % 3 == 0),
        "Q3 image of {v:?} is not an integer vector"
    );
    [r[0] / 3, r[1] / 3, r[2] / 3]
}

fn oriented(v: [i64; 3], rotated: bool) -> [i64; 3] {
    if rotated {
        q3(v)
    } else {
        v
    }
}

/// 2^e, exactly (normal range only).
fn pow2(e: i32) -> f64 {
    assert!((-1022..=1023).contains(&e));
    f64::from_bits(((1023 + e) as u64) << 52)
}

fn coordinates(v: [i64; 3]) -> [f64; 3] {
    [v[0] as f64, v[1] as f64, v[2] as f64]
}

/// P1's `y_reference` rule (`T3/DETECTION/scripts/gen.py.txt`): the global
/// axis least aligned with the member, preferring Y, then Z, then X on ties,
/// from the exact integer coordinate difference.
pub fn p1_y_reference(pi: [i64; 3], pj: [i64; 3]) -> [f64; 3] {
    let magnitudes = [
        (pj[0] - pi[0]).abs(),
        (pj[1] - pi[1]).abs(),
        (pj[2] - pi[2]).abs(),
    ];
    let mut best = 1;
    for axis in [2, 0] {
        if magnitudes[axis] < magnitudes[best] {
            best = axis;
        }
    }
    let mut y = [0.0; 3];
    y[best] = 1.0;
    y
}

/// Accumulates an integer model and converts it once.
struct Builder {
    points: Vec<(String, [i64; 3])>,
    members: Vec<(String, usize, usize)>,
    restraints: Vec<(usize, [bool; 6])>,
    loads: Vec<(usize, f64)>,
}

impl Builder {
    fn new() -> Self {
        Self {
            points: Vec::new(),
            members: Vec::new(),
            restraints: Vec::new(),
            loads: Vec::new(),
        }
    }
    fn node(&mut self, label: String, p: [i64; 3]) -> usize {
        self.points.push((label, p));
        self.points.len() - 1
    }
    /// A force (or, with `offset` 3, a moment) at `node`: each nonzero
    /// component `c * 2^exponent`, exact.
    fn load(&mut self, node: usize, offset: usize, v: [i64; 3], exponent: i32) {
        for (axis, &c) in v.iter().enumerate() {
            if c != 0 {
                assert!(c.unsigned_abs() < (1_u64 << 53));
                self.loads.push((
                    node * DOF_PER_NODE + offset + axis,
                    c as f64 * pow2(exponent),
                ));
            }
        }
    }
    fn finish(mut self, id: &str, source: &str, family: Family) -> K6Model {
        self.loads.sort_by_key(|&(dof, _)| dof);
        self.restraints.sort_by_key(|&(node, _)| node);
        let members = self
            .members
            .iter()
            .map(|(label, i, j)| {
                (
                    label.clone(),
                    *i,
                    *j,
                    p1_y_reference(self.points[*i].1, self.points[*j].1),
                )
            })
            .collect();
        K6Model {
            id: id.to_string(),
            source: source.to_string(),
            family,
            section: rf_large_section(),
            nodes: self
                .points
                .into_iter()
                .map(|(label, p)| (label, coordinates(p)))
                .collect(),
            members,
            restraints: self.restraints,
            loads: self.loads,
        }
    }
}

const ALL_SIX: [bool; 6] = [true; 6];
const PIN_THREE: [bool; 6] = [true, true, true, false, false, false];

/// R1's CHAIN (`lg_chain_defn`): nodes N0..Nn at (3i, 0, 0), members Mi =
/// (N(i-1), Ni), N0 fixed, and at Nn the force 2^s(6,-3,9) and the moment
/// 2^s2(3,6,-3); rotated by Q3 when `rotated`.
fn chain(id: &str, source: &str, n: usize, rotated: bool, s: i32, s2: i32) -> K6Model {
    let mut b = Builder::new();
    for i in 0..=n {
        b.node(format!("N{i}"), oriented([3 * i as i64, 0, 0], rotated));
    }
    for i in 1..=n {
        b.members.push((format!("M{i}"), i - 1, i));
    }
    b.restraints.push((0, ALL_SIX));
    b.load(n, 0, oriented([6, -3, 9], rotated), s);
    b.load(n, 3, oriented([3, 6, -3], rotated), s2);
    b.finish(id, source, Family::Chain)
}

/// R1's TREE (`lg_tree_defn`): P0, then Pk and Bk interleaved; members Sk =
/// (P(k-1), Pk) and Qk = (Pk, Bk) interleaved; Bk at +3 in y (k odd) or z (k
/// even); the load at Bk is 3((k mod 7) - 3)·2^sc along z (k odd) or y (k
/// even), omitted when zero; P0 fixed.
fn tree(id: &str, n: usize, rotated: bool, sc: i32) -> K6Model {
    let mut b = Builder::new();
    let h = n / 2;
    let mut spine = vec![b.node("P0".to_string(), [0, 0, 0])];
    for k in 1..=h {
        let x = 3 * k as i64;
        spine.push(b.node(format!("P{k}"), oriented([x, 0, 0], rotated)));
        let tip = if k % 2 == 1 { [x, 3, 0] } else { [x, 0, 3] };
        let branch = b.node(format!("B{k}"), oriented(tip, rotated));
        b.members.push((format!("S{k}"), spine[k - 1], spine[k]));
        b.members.push((format!("Q{k}"), spine[k], branch));
        let c = (k % 7) as i64 - 3;
        if c != 0 {
            let force = if k % 2 == 1 {
                [0, 0, 3 * c]
            } else {
                [0, 3 * c, 0]
            };
            b.load(branch, 0, oriented(force, rotated), sc);
        }
    }
    b.restraints.push((0, ALL_SIX));
    b.finish(id, R1_SOURCE, Family::Tree)
}

/// R1's CONT (`continuous_defn` via `lg_cont_defn`): s = n/2 spans of 6 m;
/// nodes S0..Ss, then C1..Cs; members Aj = (S(j-1), Cj) and Bj = (Cj, Sj)
/// interleaved; S0 fixed, S1..Ss pinned in UX, UY, UZ; the load at Cj is
/// Q(0, 96((j mod 5) - 2), 96((j mod 3) - 1)), omitted when zero.
fn cont(id: &str, n: usize, rotated: bool) -> K6Model {
    let mut b = Builder::new();
    let s = n / 2;
    for i in 0..=s {
        b.node(format!("S{i}"), oriented([6 * i as i64, 0, 0], rotated));
    }
    for j in 1..=s {
        b.node(
            format!("C{j}"),
            oriented([3 * (2 * j as i64 - 1), 0, 0], rotated),
        );
    }
    for j in 1..=s {
        b.members.push((format!("A{j}"), j - 1, s + j));
        b.members.push((format!("B{j}"), s + j, j));
    }
    b.restraints.push((0, ALL_SIX));
    for i in 1..=s {
        b.restraints.push((i, PIN_THREE));
    }
    for j in 1..=s {
        let py = 96 * ((j % 5) as i64 - 2);
        let pz = 96 * ((j % 3) as i64 - 1);
        if py != 0 || pz != 0 {
            b.load(s + j, 0, oriented([0, py, pz], rotated), 0);
        }
    }
    b.finish(id, R1_SOURCE, Family::Cont)
}

/// A harness fixture as a K6 model: its own section, `y_reference`, force and
/// restraints, unchanged. Node labels are `n<i>`, member labels `e<k>`.
fn from_fixture(
    id: &str,
    source: &str,
    family: Family,
    fixture: BenchmarkFixture,
) -> Result<K6Model, String> {
    let mut coordinates: Vec<Option<[f64; 3]>> = vec![None; fixture.node_count];
    let mut members = Vec::with_capacity(fixture.elements.len());
    let section = fixture
        .elements
        .first()
        .map(|e| e.section)
        .ok_or("fixture without elements")?;
    for (k, element) in fixture.elements.iter().enumerate() {
        if element.section != section {
            return Err("fixture with more than one section".to_string());
        }
        for node in [element.node_i, element.node_j] {
            coordinates[node.index] = Some(node.coordinates);
        }
        members.push((
            format!("e{k}"),
            element.node_i.index,
            element.node_j.index,
            element.y_reference,
        ));
    }
    let nodes = coordinates
        .into_iter()
        .enumerate()
        .map(|(i, c)| c.map(|c| (format!("n{i}"), c)))
        .collect::<Option<Vec<_>>>()
        .ok_or("fixture node without an element")?;
    let mut restraints: Vec<(usize, [bool; 6])> = Vec::new();
    for &dof in &fixture.restrained_dofs {
        let node = dof / DOF_PER_NODE;
        match restraints.iter_mut().find(|(n, _)| *n == node) {
            Some((_, mask)) => mask[dof % DOF_PER_NODE] = true,
            None => {
                let mut mask = [false; 6];
                mask[dof % DOF_PER_NODE] = true;
                restraints.push((node, mask));
            }
        }
    }
    restraints.sort_by_key(|&(node, _)| node);
    let loads = fixture
        .force
        .iter()
        .enumerate()
        .filter(|(_, v)| **v != 0.0)
        .map(|(dof, v)| (dof, *v))
        .collect();
    Ok(K6Model {
        id: id.to_string(),
        source: source.to_string(),
        family,
        section,
        nodes,
        members,
        restraints,
        loads,
    })
}
