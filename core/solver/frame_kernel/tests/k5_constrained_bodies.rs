//! K5 (T3 D1 revision 5a.2 §4.9, W4): `rigid_body::assess_constrained_bodies`,
//! through the public API only.
//!
//! Expectations come from `k5_constrained/gen_k5_vectors.py` (exact rationals on
//! the binary64 inputs; the oracle is the UNREDUCED stacked map, independent of
//! the function's tie reduction). A published witness is checked here again,
//! exactly, with `exact_sum::ExactAccumulator` (not the function's `Expansion`):
//! rigid on every sub-body, equal at every tie, zero on every ground row.
//! No expected value passes through the platform libm.
use open_pipe_stress_frame_kernel::exact_sum::ExactAccumulator;
use open_pipe_stress_frame_kernel::rigid_body::{
    assess_constrained_bodies, assess_rigid_body, objective_sub_bodies, user_element_tie,
    ConstrainedAssessment, ConstrainedGround, GroundKind, ObjectiveFamily, RigidBodyStatus,
    TieRefusal,
};
use open_pipe_stress_frame_kernel::structural::StructuralError;
use open_pipe_stress_frame_kernel::{FrameNode, UserStiffnessElement};

const SAMPLE: &str = include_str!("k5_constrained/b1_sample.txt");
const CASES: &str = include_str!("k5_constrained/cases.txt");
const SOURCE: &str = include_str!("../src/rigid_body.rs");

// ------------------------------------------------------------------ records

#[derive(Debug, Clone)]
struct Record {
    name: String,
    expect: char,
    coordinates: Vec<[f64; 3]>,
    sub_bodies: Vec<Vec<usize>>,
    ties: Vec<[usize; 2]>,
    grounds: Vec<ConstrainedGround>,
    motion: Option<Vec<[f64; 6]>>,
    rigid: Option<char>,
}

fn hex(text: &str) -> f64 {
    f64::from_bits(u64::from_str_radix(text, 16).expect("hex bits"))
}

fn parse(line: &str) -> Record {
    let mut fields = line.split(' ');
    let name = fields.next().unwrap().to_string();
    let expect = fields.next().unwrap().chars().next().unwrap();
    let mut record = Record {
        name,
        expect,
        coordinates: Vec::new(),
        sub_bodies: Vec::new(),
        ties: Vec::new(),
        grounds: Vec::new(),
        motion: None,
        rigid: None,
    };
    let mut n = 0;
    for field in fields {
        let (key, value) = field.split_once('=').expect("key=value");
        match key {
            "n" => n = value.parse().unwrap(),
            "subs" => {
                record.sub_bodies = value
                    .split(';')
                    .map(|s| s.split(',').map(|x| x.parse().unwrap()).collect())
                    .collect()
            }
            "ties" if value != "-" => {
                record.ties = value
                    .split(',')
                    .map(|t| {
                        let (a, b) = t.split_once('-').unwrap();
                        [a.parse().unwrap(), b.parse().unwrap()]
                    })
                    .collect()
            }
            "grounds" if value != "-" => {
                record.grounds = value
                    .split(',')
                    .map(|g| {
                        let (kind, rest) = g.split_at(1);
                        if kind == "d" {
                            return ConstrainedGround::Dof(rest.parse().unwrap());
                        }
                        let parts = rest.split(':').collect::<Vec<_>>();
                        ConstrainedGround::Directional {
                            node: parts[0].parse().unwrap(),
                            kind: if kind == "t" {
                                GroundKind::Translation
                            } else {
                                GroundKind::Rotation
                            },
                            direction: [hex(parts[1]), hex(parts[2]), hex(parts[3])],
                        }
                    })
                    .collect()
            }
            "coords" => {
                let values = value.split(',').map(hex).collect::<Vec<_>>();
                record.coordinates = values.chunks(3).map(|c| [c[0], c[1], c[2]]).collect();
            }
            "motion" if value != "-" => {
                let values = value.split(',').map(hex).collect::<Vec<_>>();
                record.motion = Some(
                    values
                        .chunks(6)
                        .map(|c| std::array::from_fn(|i| c[i]))
                        .collect(),
                );
            }
            "rigid" => record.rigid = value.chars().next(),
            _ => {}
        }
    }
    assert_eq!(record.coordinates.len(), n, "{}", record.name);
    record
}

fn records(text: &str) -> Vec<Record> {
    text.lines().filter(|l| !l.is_empty()).map(parse).collect()
}

fn case(name: &str) -> Record {
    records(CASES)
        .into_iter()
        .find(|r| r.name == name)
        .unwrap_or_else(|| panic!("case {name}"))
}

fn assess(record: &Record) -> ConstrainedAssessment {
    assess_constrained_bodies(
        &record.coordinates,
        &record.sub_bodies,
        &record.ties,
        &record.grounds,
    )
    .unwrap_or_else(|e| panic!("{}: {e:?}", record.name))
}

fn bits6(motion: &[[f64; 6]]) -> Vec<[u64; 6]> {
    motion.iter().map(|m| m.map(f64::to_bits)).collect()
}

// ------------------------------------------------------------------ the independent exact check

/// A published witness, checked exactly with `ExactAccumulator`.
fn verify_witness(record: &Record, result: &ConstrainedAssessment) {
    let motion = result.node_motion.as_ref().expect("node motion");
    let n = record.coordinates.len();
    assert_eq!(motion.len(), n, "{}", record.name);
    assert!(
        motion.iter().flatten().any(|&v| v != 0.0),
        "{}: zero witness",
        record.name
    );
    // One rotation for the whole connected body.
    let theta = [motion[0][3], motion[0][4], motion[0][5]];
    for m in motion {
        assert_eq!(
            [m[3], m[4], m[5]].map(f64::to_bits),
            theta.map(f64::to_bits)
        );
    }
    // The published parameters: [u(node 0)/L, θ].
    let parameters = result.rigid_parameters.expect("parameters");
    let length = result.characteristic_length;
    assert!(
        length > 0.0 && length.to_bits() & ((1 << 52) - 1) == 0,
        "L = {length}"
    );
    for k in 0..3 {
        assert_eq!(parameters[k] * length, motion[0][k], "{}", record.name);
        assert_eq!(parameters[3 + k], theta[k]);
    }
    assert_eq!(result.origin, record.coordinates[0]);
    // Rigid on every sub-body: u_y − u_f − θ × (x_y − x_f) = 0 exactly.
    for body in &record.sub_bodies {
        let f = body[0];
        for &y in body {
            for k in 0..3 {
                let (k1, k2) = ((k + 1) % 3, (k + 2) % 3);
                let (xy, xf) = (record.coordinates[y], record.coordinates[f]);
                let mut acc = ExactAccumulator::new();
                acc.add(motion[y][k]).unwrap();
                acc.add(-motion[f][k]).unwrap();
                // (θ × d)_k = θ_k1 d_k2 − θ_k2 d_k1, d = x_y − x_f.
                acc.add_product(-theta[k1], xy[k2]).unwrap();
                acc.add_product(theta[k1], xf[k2]).unwrap();
                acc.add_product(theta[k2], xy[k1]).unwrap();
                acc.add_product(-theta[k2], xf[k1]).unwrap();
                assert!(acc.is_zero(), "{}: not rigid at node {y}", record.name);
            }
        }
    }
    // Equal motion at every tie.
    for &[a, b] in &record.ties {
        assert_eq!(
            motion[a].map(f64::to_bits),
            motion[b].map(f64::to_bits),
            "{}: tie {a}-{b}",
            record.name
        );
    }
    // Zero on every ground row.
    for ground in &record.grounds {
        match *ground {
            ConstrainedGround::Dof(d) => {
                assert_eq!(motion[d / 6][d % 6], 0.0, "{}: DOF {d}", record.name)
            }
            ConstrainedGround::Directional {
                node,
                kind,
                direction,
            } => {
                let offset = if kind == GroundKind::Translation {
                    0
                } else {
                    3
                };
                let mut acc = ExactAccumulator::new();
                for k in 0..3 {
                    acc.add_product(direction[k], motion[node][offset + k])
                        .unwrap();
                }
                assert!(acc.is_zero(), "{}: directional row at {node}", record.name);
            }
        }
    }
}

/// Checks `result` against the record's expectation; returns the outcome letter.
fn check(record: &Record, result: &ConstrainedAssessment) -> char {
    use RigidBodyStatus::*;
    let witnessed = result.status == MechanismWitnessed;
    match record.expect {
        'R' => assert_eq!(result.status, Restrained, "{}", record.name),
        'W' => assert_eq!(result.status, MechanismWitnessed, "{}", record.name),
        'M' => assert!(
            matches!(result.status, MechanismWitnessed | NumericallyUnresolved),
            "{}: {:?}",
            record.name,
            result.status
        ),
        'U' => assert_eq!(result.status, NumericallyUnresolved, "{}", record.name),
        'D' => assert!(
            matches!(result.status, MechanismWitnessed | NumericallyUnresolved),
            "{}: {:?}",
            record.name,
            result.status
        ),
        'N' => assert!(
            matches!(result.status, Restrained | NumericallyUnresolved),
            "{}: {:?}",
            record.name,
            result.status
        ),
        other => panic!("expectation {other}"),
    }
    if witnessed {
        verify_witness(record, result);
        if let Some(expected) = &record.motion {
            assert_eq!(
                bits6(result.node_motion.as_ref().unwrap()),
                bits6(expected),
                "{}: not the canonical representative",
                record.name
            );
        }
    } else {
        assert!(result.node_motion.is_none() && result.rigid_parameters.is_none());
    }
    match result.status {
        Restrained => 'R',
        MechanismWitnessed => 'W',
        NumericallyUnresolved => 'U',
        UnqualifiedFamily => panic!("{}: W4 never returns UnqualifiedFamily", record.name),
    }
}

fn dof_grounds(record: &Record) -> Vec<usize> {
    record
        .grounds
        .iter()
        .map(|g| match g {
            ConstrainedGround::Dof(d) => *d,
            _ => panic!("{}: DOF grounds only", record.name),
        })
        .collect()
}

// ------------------------------------------------------------------ B1

fn run_b1(text: &str) -> std::collections::BTreeMap<String, usize> {
    let mut tally = std::collections::BTreeMap::new();
    for record in records(text) {
        let outcome = check(&record, &assess(&record));
        *tally
            .entry(format!("{}->{}", record.expect, outcome))
            .or_insert(0) += 1;
    }
    tally
}

/// B1: on the seeded set (1–8 sub-bodies, 0–10 ties including cycles and ties
/// inside one sub-body, DOF and directional grounds; outside the τ_B band),
/// the status agrees with the exact rank of the unreduced stacked map; every
/// case built from a small-integer null motion (W) is witnessed with the
/// generator's canonical bits; every witness is verified exactly. The full
/// set (4,000 records, `b1_summary.txt`) runs with `K5_B1_VECTORS=<path>`.
#[test]
fn k5_b1_reduction_agrees_with_the_unreduced_stacked_map() {
    let tally = run_b1(SAMPLE);
    eprintln!("k5 b1 sample: {tally:?}");
    assert_eq!(tally.values().sum::<usize>(), 1000);
    assert!(tally.get("W->W").copied().unwrap_or(0) > 300);
    assert!(tally.get("R->R").copied().unwrap_or(0) > 300);
    if let Ok(path) = std::env::var("K5_B1_VECTORS") {
        let full = std::fs::read_to_string(path).expect("K5_B1_VECTORS");
        let tally = run_b1(&full);
        eprintln!("k5 b1 full: {tally:?}");
        assert_eq!(tally.values().sum::<usize>(), 4000);
    }
}

// ------------------------------------------------------------------ B2, B3

fn check_named(name: &str) -> (Record, ConstrainedAssessment) {
    let record = case(name);
    let result = assess(&record);
    check(&record, &result);
    if let Some(rigid) = record.rigid {
        // The rigid-link model: the welded union of every node.
        let welded = assess_rigid_body(
            &record.coordinates,
            &dof_grounds(&record),
            ObjectiveFamily::WeldedUnreleasedElasticFrames,
        )
        .unwrap();
        let expected = if rigid == 'R' {
            RigidBodyStatus::Restrained
        } else {
            RigidBodyStatus::MechanismWitnessed
        };
        assert_eq!(welded.status, expected, "{name}: the rigid-link model");
    }
    (record, result)
}

/// B2: an NP-C-like internal mechanism. The welded union is restrained, the
/// tie motion is free; the stabilized companion restrains it.
#[test]
fn k5_b2_np_c_like_internal_mechanism_and_its_companion() {
    let (_, internal) = check_named("npc_internal");
    assert_eq!(internal.status, RigidBodyStatus::MechanismWitnessed);
    let (_, companion) = check_named("npc_companion");
    assert_eq!(companion.status, RigidBodyStatus::Restrained);
}

/// B2: near-collinear virtual pins (the actual pins are not collinear): exact,
/// one ulp off (in the τ_B band, no exact null vector), separated; and NP-C's
/// `near_collinear_rows` (exact rank 2) as two rows of a one-node body.
#[test]
fn k5_b2_near_collinear_virtual_pins() {
    for (name, status) in [
        ("nc_exact", RigidBodyStatus::MechanismWitnessed),
        ("nc_ulp", RigidBodyStatus::NumericallyUnresolved),
        ("nc_separated", RigidBodyStatus::Restrained),
        ("npc_rows", RigidBodyStatus::NumericallyUnresolved),
    ] {
        let (_, result) = check_named(name);
        assert_eq!(result.status, status, "{name}");
    }
    // In the band: σ_min is at or below τ_B although the exact rank is full.
    let (_, ulp) = check_named("nc_ulp");
    let min = ulp
        .singular_values
        .iter()
        .copied()
        .fold(f64::INFINITY, f64::min);
    assert!(min <= ulp.rank_screen, "{min} > {}", ulp.rank_screen);
}

/// B3 (§7.3 item 12): a tie is not a rigid link, both ways. The rigid-link
/// model (the welded union) calls npc_internal restrained (a missed mechanism)
/// and b3_false_mechanism witnessed (a false physical mechanism); the ties
/// give the exact answers.
#[test]
fn k5_b3_tie_is_not_a_rigid_link_both_ways() {
    let (record, result) = check_named("npc_internal");
    assert_eq!(record.rigid, Some('R'));
    assert_eq!(result.status, RigidBodyStatus::MechanismWitnessed);
    let (record, result) = check_named("b3_false_mechanism");
    assert_eq!(record.rigid, Some('W'));
    assert_eq!(result.status, RigidBodyStatus::Restrained);
}

// ------------------------------------------------------------------ B4

/// B4: a cycle removes the only free mode; a cycle with offset exactly zero
/// adds no row; a tie inside one sub-body; a cycle offset along an axis (its
/// zero row dropped).
#[test]
fn k5_b4_cycles() {
    let (_, restrains) = check_named("b4_cycle_restrains");
    assert_eq!(restrains.status, RigidBodyStatus::Restrained);
    // Five ground rows and two nonzero cycle rows ((0,2,0) × e_y = 0).
    assert_eq!((restrains.rows, restrains.cycles), (7, 1));
    let (_, zero) = check_named("b4_cycle_zero");
    assert_eq!((zero.rows, zero.cycles), (5, 1));
    let (_, free) = check_named("b4_inside_free");
    assert_eq!(free.status, RigidBodyStatus::MechanismWitnessed);
    let (_, blocks) = check_named("b4_inside_blocks");
    assert_eq!(blocks.status, RigidBodyStatus::Restrained);
    assert_eq!((blocks.rows, blocks.cycles), (7, 1));
}

// ------------------------------------------------------------------ B5

fn scaled(record: &Record, factor: f64) -> Record {
    let mut out = record.clone();
    for p in &mut out.coordinates {
        for x in p.iter_mut() {
            *x *= factor;
        }
    }
    out
}

/// B5 (KREV-01 analogues for ties): at 1e16 and 1e200 scale a rounded
/// candidate looks null and the exact check refutes it (no witness, no node
/// motion), in radix units 0.5, 1 and 2; valid witnesses survive origin
/// shifts and radix units with the generator's bits; an exact mechanism whose
/// canonical direction is not representable ends unresolved.
#[test]
fn k5_b5_exactness() {
    for name in ["b5_big16", "b5_big200"] {
        let record = case(name);
        for units in [0.5, 1.0, 2.0] {
            let record = scaled(&record, units);
            let result = assess(&record);
            check(&record, &result);
            assert_ne!(
                result.status,
                RigidBodyStatus::MechanismWitnessed,
                "{name} {units}"
            );
            assert!(result.node_motion.is_none());
        }
    }
    for units in ["0.5", "1.0", "2.0"] {
        for shift in ["0.0", "8.0"] {
            let (_, result) = check_named(&format!("b5_radix_u{units}_s{shift}"));
            for motion in result.node_motion.unwrap() {
                assert_eq!(&motion[..3], &[0.0, 0.0, 0.0]);
            }
        }
    }
    let (_, result) = check_named("b5_unrepresentable");
    assert_eq!(result.status, RigidBodyStatus::NumericallyUnresolved);
}

// ------------------------------------------------------------------ B6, B7

/// A small deterministic generator (no dependency).
struct Lcg(u64);
impl Lcg {
    fn next(&mut self) -> u64 {
        self.0 = self
            .0
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        self.0 >> 33
    }
    fn shuffle<T>(&mut self, items: &mut [T]) {
        for i in (1..items.len()).rev() {
            let j = (self.next() % (i as u64 + 1)) as usize;
            items.swap(i, j);
        }
    }
}

fn permuted(record: &Record, rng: &mut Lcg) -> Record {
    let mut out = record.clone();
    rng.shuffle(&mut out.sub_bodies);
    for body in &mut out.sub_bodies {
        rng.shuffle(body);
    }
    rng.shuffle(&mut out.ties);
    for tie in &mut out.ties {
        if rng.next() % 2 == 0 {
            tie.swap(0, 1);
        }
    }
    rng.shuffle(&mut out.grounds);
    out
}

fn corpus(b1: usize) -> Vec<Record> {
    let mut all = records(CASES);
    all.extend(records(SAMPLE).into_iter().take(b1));
    all
}

/// B6: permuting the sub-bodies, their nodes, the ties, the tie ends and the
/// grounds gives the same assessment, bit for bit.
#[test]
fn k5_b6_order_independence() {
    let mut rng = Lcg(0x6b35_2026_0928);
    for record in corpus(300) {
        let base = format!("{:?}", assess(&record));
        for _ in 0..4 {
            let other = permuted(&record, &mut rng);
            assert_eq!(format!("{:?}", assess(&other)), base, "{}", record.name);
        }
    }
}

/// B7: coordinates times 2^k give the same status.
#[test]
fn k5_b7_power_of_two_invariance() {
    for record in corpus(300) {
        let base = assess(&record).status;
        for k in [-60, -3, -2, -1, 1, 2, 3, 60] {
            let factor = f64::from_bits(((1023 + k) as u64) << 52);
            let other = scaled(&record, factor);
            assert_eq!(assess(&other).status, base, "{} 2^{k}", record.name);
        }
    }
}

// ------------------------------------------------------------------ B8

/// B8: invalid input, with `assess_rigid_body`'s error kinds.
#[test]
fn k5_b8_validation() {
    let c = [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]];
    let one = vec![vec![0, 1]];
    let invalid = |r: Result<ConstrainedAssessment, StructuralError>, what: &str| {
        assert!(
            matches!(
                r,
                Err(StructuralError::InvalidInput("constrained geometry"))
            ),
            "{what}: {r:?}"
        )
    };
    invalid(assess_constrained_bodies(&[], &[], &[], &[]), "no nodes");
    invalid(
        assess_constrained_bodies(&[[0.0, f64::NAN, 0.0], [1.0, 0.0, 0.0]], &one, &[], &[]),
        "NaN coordinate",
    );
    invalid(
        assess_constrained_bodies(
            &[[0.0, 0.0, f64::INFINITY], [1.0, 0.0, 0.0]],
            &one,
            &[],
            &[],
        ),
        "infinite coordinate",
    );
    invalid(
        assess_constrained_bodies(&c, &[vec![0, 1], vec![]], &[], &[]),
        "empty sub-body",
    );
    invalid(
        assess_constrained_bodies(&c, &[vec![0, 2]], &[], &[]),
        "node out of range",
    );
    invalid(
        assess_constrained_bodies(&c, &[vec![0, 1], vec![1]], &[], &[]),
        "node in two",
    );
    invalid(
        assess_constrained_bodies(&c, &[vec![0, 1, 1]], &[], &[]),
        "node twice",
    );
    invalid(
        assess_constrained_bodies(&c, &[vec![0]], &[], &[]),
        "node in none",
    );
    let two = vec![vec![0], vec![1]];
    invalid(
        assess_constrained_bodies(&c, &two, &[], &[]),
        "not connected",
    );
    invalid(
        assess_constrained_bodies(&c, &two, &[[0, 2]], &[]),
        "tie out of range",
    );
    invalid(
        assess_constrained_bodies(&c, &two, &[[0, 1], [1, 1]], &[]),
        "tie to itself",
    );
    invalid(
        assess_constrained_bodies(&c, &one, &[], &[ConstrainedGround::Dof(12)]),
        "DOF out of range",
    );
    for (node, direction) in [
        (2, [1.0, 0.0, 0.0]),
        (0, [0.0, 0.0, 0.0]),
        (0, [f64::NAN, 1.0, 0.0]),
        (0, [f64::INFINITY, 1.0, 0.0]),
    ] {
        for kind in [GroundKind::Translation, GroundKind::Rotation] {
            invalid(
                assess_constrained_bodies(
                    &c,
                    &one,
                    &[],
                    &[ConstrainedGround::Directional {
                        node,
                        kind,
                        direction,
                    }],
                ),
                "directional row",
            );
        }
    }
    // An exact coordinate difference outside the binary64 range.
    let far = [[-1.5e308, 0.0, 0.0], [1.5e308, 0.0, 0.0]];
    assert!(matches!(
        assess_constrained_bodies(&far, &one, &[], &[]),
        Err(StructuralError::Range("constrained relative coordinates"))
    ));
    assert!(matches!(
        objective_sub_bodies(2, &[[0, 2]]),
        Err(StructuralError::InvalidInput("constrained geometry"))
    ));
}

/// The objective sub-bodies are the components of the links, ordered.
#[test]
fn k5_objective_sub_bodies_are_the_link_components() {
    assert_eq!(
        objective_sub_bodies(6, &[[4, 1], [5, 5], [3, 0], [1, 2]]).unwrap(),
        vec![vec![0, 3], vec![1, 2, 4], vec![5]]
    );
    assert_eq!(
        objective_sub_bodies(0, &[]).unwrap(),
        Vec::<Vec<usize>>::new()
    );
}

// ------------------------------------------------------------------ B9

/// B9 (Q6): directional ground rows. A spanning triad per node is equivalent
/// to its three DOF rows (the same canonical witness); a translation-pinned
/// line with an axial rotational row is restrained, with a perpendicular one
/// it is not; a non-spanning set with a real mechanism is witnessed exactly.
#[test]
fn k5_b9_directional_grounds() {
    let (dof, reference) = check_named("npc_internal");
    let expected = bits6(dof.motion.as_ref().unwrap());
    for name in ["b9_triad_axes", "b9_triad_oblique"] {
        let (_, result) = check_named(name);
        assert_eq!(
            bits6(result.node_motion.as_ref().unwrap()),
            expected,
            "{name}"
        );
        assert_eq!(result.status, reference.status);
    }
    let (_, axial) = check_named("b9_line_axial_rotation");
    assert_eq!(axial.status, RigidBodyStatus::Restrained);
    for name in [
        "b9_line_perpendicular_rotation",
        "b9_nonspanning",
        "b9_tied_line",
    ] {
        let (_, result) = check_named(name);
        assert_eq!(result.status, RigidBodyStatus::MechanismWitnessed, "{name}");
    }
}

// ------------------------------------------------------------------ B10

/// Removes comments and blanks string and char literal contents (lifetimes
/// are kept), preserving the text's shape.
fn lex(src: &str) -> String {
    let b: Vec<char> = src.chars().collect();
    let n = b.len();
    let mut out = String::with_capacity(n);
    let mut i = 0;
    while i < n {
        let c = b[i];
        if c == '/' && i + 1 < n && b[i + 1] == '/' {
            while i < n && b[i] != '\n' {
                i += 1;
            }
            continue;
        }
        if c == '/' && i + 1 < n && b[i + 1] == '*' {
            let mut depth = 1;
            i += 2;
            while i < n && depth > 0 {
                if b[i] == '/' && i + 1 < n && b[i + 1] == '*' {
                    depth += 1;
                    i += 2;
                } else if b[i] == '*' && i + 1 < n && b[i + 1] == '/' {
                    depth -= 1;
                    i += 2;
                } else {
                    i += 1;
                }
            }
            continue;
        }
        if c == '"' {
            out.push('"');
            i += 1;
            while i < n && b[i] != '"' {
                if b[i] == '\\' {
                    i += 1;
                }
                i += 1;
            }
            out.push('"');
            i += 1;
            continue;
        }
        if c == '\'' {
            if i + 1 < n && b[i + 1] == '\\' {
                i += 2;
                while i < n && b[i] != '\'' {
                    i += 1;
                }
                out.push_str("' '");
                i += 1;
                continue;
            }
            if i + 2 < n && b[i + 2] == '\'' {
                out.push_str("' '");
                i += 3;
                continue;
            }
        }
        out.push(c);
        i += 1;
    }
    out
}

/// `code` with the body of `fn <name>(` removed (from its first `{` to the
/// matching `}`).
fn without_function(code: &str, name: &str) -> String {
    let at = code
        .find(&format!("fn {name}("))
        .unwrap_or_else(|| panic!("fn {name}"));
    let open = at + code[at..].find('{').unwrap();
    let mut depth = 0;
    let mut end = open;
    for (i, c) in code[open..].char_indices() {
        match c {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    end = open + i + 1;
                    break;
                }
            }
            _ => {}
        }
    }
    format!("{}{}", &code[..at], &code[end..])
}

/// Every test module (`#[cfg(test)] mod … { … }`) removed.
fn without_test_modules(code: &str) -> String {
    let mut code = code.to_string();
    while let Some(at) = code.find("#[cfg(test)]") {
        let open = at + code[at..].find('{').unwrap();
        let mut depth = 0;
        let mut end = open;
        for (i, c) in code[open..].char_indices() {
            match c {
                '{' => depth += 1,
                '}' => {
                    depth -= 1;
                    if depth == 0 {
                        end = open + i + 1;
                        break;
                    }
                }
                _ => {}
            }
        }
        code = format!("{}{}", &code[..at], &code[end..]);
    }
    code
}

const FORBIDDEN: &[&str] = &[
    "hypot", "powi", "powf", "exp", "exp2", "exp_m1", "ln", "ln_1p", "log", "log2", "log10", "sin",
    "cos", "tan", "asin", "acos", "atan", "atan2", "sin_cos", "sinh", "cosh", "tanh", "asinh",
    "acosh", "atanh", "cbrt",
];

/// The forbidden calls in `code`: an identifier from FORBIDDEN followed by `(`.
fn forbidden_calls(code: &str) -> Vec<String> {
    let chars: Vec<char> = code.chars().collect();
    let mut found = Vec::new();
    let mut i = 0;
    while i < chars.len() {
        if chars[i].is_alphabetic() || chars[i] == '_' {
            let start = i;
            while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_') {
                i += 1;
            }
            let word: String = chars[start..i].iter().collect();
            let mut j = i;
            while j < chars.len() && chars[j].is_whitespace() {
                j += 1;
            }
            let preceded =
                start > 0 && (chars[start - 1].is_alphanumeric() || chars[start - 1] == '_');
            if !preceded && j < chars.len() && chars[j] == '(' && FORBIDDEN.contains(&word.as_str())
            {
                found.push(word);
            }
        } else {
            i += 1;
        }
    }
    found
}

/// B10 (ROOT's K5 ruling Q4(b)): K5's new code in `rigid_body.rs` calls no
/// function of unspecified precision. The scan excludes `assess_rigid_body`
/// and `original_rigid_witness` (unchanged; their `hypot` is on the T3-close
/// list) and the test modules. A control shows the scanner finds the `hypot`
/// in the excluded function. It is a source scan: it does not see a call made
/// through another function (K5 calls FK's existing `Expansion`, whose
/// `exact_radix` uses `2.0_f64.powi(step)` with a round-trip self-check).
#[test]
fn k5_b10_libm_free_source_scan() {
    let code = lex(SOURCE);
    // Control: the unchanged screen's `hypot` is found.
    assert!(forbidden_calls(&code).contains(&"hypot".to_string()));
    let probe = lex("// x.hypot(1.0)\nlet s = \"sin(\"; let c = 'a'; fn f<'a>() { y.cos() }");
    assert_eq!(forbidden_calls(&probe), vec!["cos".to_string()]);
    let scanned = without_test_modules(&without_function(
        &without_function(&code, "assess_rigid_body"),
        "original_rigid_witness",
    ));
    for name in [
        "assess_constrained_bodies",
        "reduce_constrained_body",
        "root_one_plus_square",
        "null_translations",
        "user_element_tie",
    ] {
        assert!(scanned.contains(&format!("fn {name}(")), "{name} scanned");
    }
    assert_eq!(forbidden_calls(&scanned), Vec::<String>::new());
}

// ------------------------------------------------------------------ C: user elements

fn element(lateral: f64) -> UserStiffnessElement {
    UserStiffnessElement {
        node_i: FrameNode {
            index: 3,
            coordinates: [0.0, 0.0, 0.0],
        },
        node_j: FrameNode {
            index: 7,
            coordinates: [3.0, 1.5, 0.0],
        },
        y_reference: [0.0, 0.0, 1.0],
        axial_stiffness: 2.5e6,
        lateral_stiffness: lateral,
        angular_stiffness: 4.0e4,
        torsional_stiffness: 6.0e4,
    }
}

/// C (Q5(a)): a tie only when all four stiffnesses are finite and positive
/// and the orientation is valid. Lateral = 0 (a struct literal, as
/// `kd5_tests.rs` builds it) is no tie: the relative lateral translation is
/// then free, so a tie would over-constrain the element.
#[test]
fn k5_c_user_tie_rule() {
    assert_eq!(user_element_tie(&element(1.0e5)), Ok([3, 7]));
    assert_eq!(
        user_element_tie(&element(0.0)),
        Err(TieRefusal::Stiffness("lateral"))
    );
    assert_eq!(
        user_element_tie(&element(-1.0)),
        Err(TieRefusal::Stiffness("lateral"))
    );
    for (field, value) in [
        ("axial", f64::NAN),
        ("angular", -0.0),
        ("torsional", f64::INFINITY),
        ("axial", -2.0),
    ] {
        let mut e = element(1.0e5);
        match field {
            "axial" => e.axial_stiffness = value,
            "angular" => e.angular_stiffness = value,
            _ => e.torsional_stiffness = value,
        }
        assert_eq!(
            user_element_tie(&e),
            Err(TieRefusal::Stiffness(field)),
            "{field} {value}"
        );
    }
    let mut repeated = element(1.0e5);
    repeated.node_j.index = 3;
    assert_eq!(user_element_tie(&repeated), Err(TieRefusal::Orientation));
    let mut parallel = element(1.0e5);
    parallel.y_reference = [2.0, 1.0, 0.0];
    assert_eq!(user_element_tie(&parallel), Err(TieRefusal::Orientation));
}

/// **T4 tripwire (M07).** For today's user element with positive stiffnesses,
/// the tie space {u_a = u_b, θ_a = θ_b} is exactly the null space of the
/// represented element:
/// - `local_stiffness()` is, bit for bit, Σ_d k_d (e_d − e_{d+6})(e_d − e_{d+6})ᵀ
///   with k = (axial, lateral, lateral, torsional, angular, angular), whose null
///   space is exactly {Δ = 0} when every k_d > 0;
/// - `global_stiffness()` has the block form [[A, −A], [−A, A]] in value, so it
///   annihilates (b, b) exactly, and the orientation is valid (T invertible).
///
/// T4's M07 repair (the joint's lateral springs with the rigid-body moment
/// coupling) changes the local form and fails this test. W4's tie rule
/// (`user_element_tie`) must then be revised with it.
#[test]
fn k5_t4_tripwire_user_tie_space_is_the_represented_null_space() {
    let cases = [
        (
            [0.0, 0.0, 0.0],
            [3.0, 1.5, 0.0],
            [0.0, 0.0, 1.0],
            [2.5e6, 1.0e5, 4.0e4, 6.0e4],
        ),
        (
            [1.0, 2.0, 3.0],
            [1.0, 2.0, 4.0],
            [1.0, 0.0, 0.0],
            [1.0, 2.0, 3.0, 4.0],
        ),
        (
            [5e6, 5e6, 0.0],
            [5e6 + 0.7, 5e6 - 0.2, 0.3],
            [0.3, 0.1, 0.9],
            [9e5, 9e5, 1e3, 7e7],
        ),
    ];
    for (xi, xj, yref, [axial, lateral, angular, torsional]) in cases {
        let e = UserStiffnessElement {
            node_i: FrameNode {
                index: 0,
                coordinates: xi,
            },
            node_j: FrameNode {
                index: 1,
                coordinates: xj,
            },
            y_reference: yref,
            axial_stiffness: axial,
            lateral_stiffness: lateral,
            angular_stiffness: angular,
            torsional_stiffness: torsional,
        };
        assert_eq!(user_element_tie(&e), Ok([0, 1]));
        let k = [axial, lateral, lateral, torsional, angular, angular];
        let mut expected = [[0.0_f64; 12]; 12];
        for d in 0..6 {
            expected[d][d] = k[d];
            expected[d + 6][d + 6] = k[d];
            expected[d][d + 6] = -k[d];
            expected[d + 6][d] = -k[d];
        }
        let local = e.local_stiffness();
        for i in 0..12 {
            for j in 0..12 {
                assert_eq!(
                    local[i][j].to_bits(),
                    expected[i][j].to_bits(),
                    "local[{i}][{j}]"
                );
            }
        }
        assert!(e.orientation().is_ok());
        let g = e.global_stiffness().unwrap();
        for r in 0..6 {
            for c in 0..6 {
                assert!(g[r + 6][c + 6] == g[r][c], "G_jj != G_ii at {r},{c}");
                assert!(g[r][c + 6] == -g[r][c], "G_ij != -G_ii at {r},{c}");
                assert!(g[r + 6][c] == -g[r][c], "G_ji != -G_ii at {r},{c}");
            }
        }
    }
}
