//! T4-U3 (J-A): the FK objective connector against T4-I12's frozen round-02
//! references (`validation/references/t4_i12/u3_reference_cases.json`,
//! confirmed by T4-RV9), through the public API only.
//!
//! Comparison rule R02-Z (`round_02.zero_floor_rule`): an entry passes when
//! |obs − exp| ≤ 1e-12·max(|exp|, F), F the floor of its family in that
//! evaluation (`round_02.zero_floors`); F = 0 families are compared exactly.
//! Ranks, the PD flag and the W4 decision are exact. Exact checks of B (the
//! W4 successor, slot table S-5 (a) and (b)) use this file's own dyadic
//! arithmetic on the connector's binary64 inputs, independent of FK's.
use open_pipe_stress_frame_kernel::connector::{
    exact_definiteness, ConnectorAttachment, ConnectorDefiniteness, ConnectorError,
    ObjectiveConnector, ScaledWorkMatrix,
};
use open_pipe_stress_frame_kernel::exact_sum::ExactAccumulator;
use open_pipe_stress_frame_kernel::{
    assemble_global_stiffness_with_connectors, reduce_system, solve_dense, ForceScale, FrameNode,
    Matrix3,
};

const REFERENCES: &str =
    include_str!("../../../../validation/references/t4_i12/u3_reference_cases.json");

// ------------------------------------------------------------------ JSON

#[derive(Debug, Clone)]
enum Json {
    Null,
    Bool(bool),
    Number(String),
    Text(String),
    Array(Vec<Json>),
    Object(Vec<(String, Json)>),
}

impl Json {
    fn get(&self, key: &str) -> Option<&Json> {
        match self {
            Json::Object(entries) => entries.iter().find(|(k, _)| k == key).map(|(_, v)| v),
            _ => None,
        }
    }
    fn at(&self, key: &str) -> &Json {
        self.get(key).unwrap_or_else(|| panic!("missing key {key}"))
    }
    fn items(&self) -> &[Json] {
        match self {
            Json::Array(items) => items,
            other => panic!("not an array: {other:?}"),
        }
    }
    fn text(&self) -> &str {
        match self {
            Json::Text(s) | Json::Number(s) => s,
            other => panic!("not a string: {other:?}"),
        }
    }
    fn is_null(&self) -> bool {
        matches!(self, Json::Null)
    }
    fn boolean(&self) -> bool {
        match self {
            Json::Bool(b) => *b,
            other => panic!("not a bool: {other:?}"),
        }
    }
    fn usize(&self) -> usize {
        self.text().parse().unwrap()
    }
    /// A number given as a rational string, a decimal string, an
    /// `{exact, decimal}` pair or a `{binary64, hex, exact}` record.
    fn value(&self) -> f64 {
        match self {
            Json::Object(_) => match self.get("binary64") {
                Some(b) => b.text().parse().unwrap(),
                None => self.at("exact").value(),
            },
            _ => rational(self.text()),
        }
    }
    fn values(&self) -> Vec<f64> {
        match self {
            Json::Object(_) if self.get("exact").is_some() => match self.at("exact") {
                Json::Array(_) => self.at("exact").values(),
                scalar => vec![scalar.value()],
            },
            Json::Text(_) | Json::Number(_) => vec![self.value()],
            _ => self.items().iter().map(Json::value).collect(),
        }
    }
    fn vec3(&self) -> [f64; 3] {
        self.values().try_into().unwrap()
    }
    fn matrix3(&self) -> Matrix3 {
        let rows: Vec<[f64; 3]> = self.items().iter().map(Json::vec3).collect();
        rows.try_into().unwrap()
    }
}

/// "p/q", an integer or a decimal string, to the nearest binary64 (a
/// quotient of two correctly rounded parts: within 1.5 ulp).
fn rational(text: &str) -> f64 {
    match text.split_once('/') {
        Some((p, q)) => p.trim().parse::<f64>().unwrap() / q.trim().parse::<f64>().unwrap(),
        None => text.trim().parse().unwrap(),
    }
}

struct Parser<'a> {
    bytes: &'a [u8],
    at: usize,
}

impl Parser<'_> {
    fn space(&mut self) {
        while self.at < self.bytes.len() && self.bytes[self.at].is_ascii_whitespace() {
            self.at = self.at.wrapping_add(1);
        }
    }
    fn parse(&mut self) -> Json {
        self.space();
        let c = self.bytes[self.at];
        match c {
            b'{' => {
                self.at = self.at.wrapping_add(1);
                let mut entries = Vec::new();
                loop {
                    self.space();
                    if self.bytes[self.at] == b'}' {
                        self.at = self.at.wrapping_add(1);
                        break;
                    }
                    let Json::Text(key) = self.parse() else {
                        panic!("object key")
                    };
                    self.space();
                    assert_eq!(self.bytes[self.at], b':');
                    self.at = self.at.wrapping_add(1);
                    let value = self.parse();
                    entries.push((key, value));
                    self.space();
                    if self.bytes[self.at] == b',' {
                        self.at = self.at.wrapping_add(1);
                    }
                }
                Json::Object(entries)
            }
            b'[' => {
                self.at = self.at.wrapping_add(1);
                let mut items = Vec::new();
                loop {
                    self.space();
                    if self.bytes[self.at] == b']' {
                        self.at = self.at.wrapping_add(1);
                        break;
                    }
                    items.push(self.parse());
                    self.space();
                    if self.bytes[self.at] == b',' {
                        self.at = self.at.wrapping_add(1);
                    }
                }
                Json::Array(items)
            }
            b'"' => {
                self.at = self.at.wrapping_add(1);
                let mut out = Vec::new();
                while self.bytes[self.at] != b'"' {
                    if self.bytes[self.at] == b'\\' {
                        self.at = self.at.wrapping_add(1);
                        match self.bytes[self.at] {
                            b'u' => {
                                let hex =
                                    std::str::from_utf8(&self.bytes[self.at + 1..self.at + 5])
                                        .unwrap();
                                let code = u32::from_str_radix(hex, 16).unwrap();
                                let ch = char::from_u32(code).unwrap_or('?');
                                let mut buffer = [0u8; 4];
                                out.extend_from_slice(ch.encode_utf8(&mut buffer).as_bytes());
                                self.at = self.at.wrapping_add(4);
                            }
                            b'n' => out.push(b'\n'),
                            b't' => out.push(b'\t'),
                            other => out.push(other),
                        }
                    } else {
                        out.push(self.bytes[self.at]);
                    }
                    self.at = self.at.wrapping_add(1);
                }
                self.at = self.at.wrapping_add(1);
                Json::Text(String::from_utf8(out).unwrap())
            }
            b't' => {
                self.at = self.at.wrapping_add(4);
                Json::Bool(true)
            }
            b'f' => {
                self.at = self.at.wrapping_add(5);
                Json::Bool(false)
            }
            b'n' => {
                self.at = self.at.wrapping_add(4);
                Json::Null
            }
            _ => {
                let start = self.at;
                while self.at < self.bytes.len()
                    && (self.bytes[self.at].is_ascii_digit()
                        || b"+-.eE".contains(&self.bytes[self.at]))
                {
                    self.at = self.at.wrapping_add(1);
                }
                Json::Number(
                    std::str::from_utf8(&self.bytes[start..self.at])
                        .unwrap()
                        .to_string(),
                )
            }
        }
    }
}

fn references() -> Json {
    let mut parser = Parser {
        bytes: REFERENCES.as_bytes(),
        at: 0,
    };
    parser.parse()
}

fn case<'a>(root: &'a Json, id: &str) -> &'a Json {
    root.at("cases").at(id)
}

// ------------------------------------------------------------------ R02-Z

/// The floors of one evaluation (a JSON pointer of `round_02.zero_floors`).
struct Floors {
    pointer: String,
    floors: Vec<(String, f64)>,
}

impl Floors {
    fn of(root: &Json, pointer: &str) -> Self {
        let table = root.at("round_02").at("zero_floors").at(pointer);
        let Json::Object(entries) = table else {
            panic!("floors {pointer}")
        };
        Self {
            pointer: pointer.to_string(),
            floors: entries
                .iter()
                .map(|(family, v)| (family.clone(), v.value()))
                .collect(),
        }
    }
    fn floor(&self, family: &str) -> f64 {
        self.floors
            .iter()
            .find(|(f, _)| f == family)
            .map(|(_, v)| *v)
            .unwrap_or_else(|| panic!("{}: no floor for {family}", self.pointer))
    }
    /// R02-Z for one array.
    fn check(&self, what: &str, family: &str, observed: &[f64], expected: &[f64]) {
        assert_eq!(observed.len(), expected.len(), "{} {what}", self.pointer);
        let floor = self.floor(family);
        for (k, (&o, &e)) in observed.iter().zip(expected).enumerate() {
            let allowed = 1e-12 * e.abs().max(floor);
            assert!(
                (o - e).abs() <= allowed,
                "{} {what}[{k}] ({family}): observed {o:e}, expected {e:e}, allowed {allowed:e}",
                self.pointer
            );
        }
    }
}

// ------------------------------------------------------------------ inputs

fn work_matrix(inputs: &Json) -> ScaledWorkMatrix {
    ScaledWorkMatrix {
        upper_triangle: inputs
            .at("H_upper_triangle_21_N_m")
            .values()
            .try_into()
            .unwrap(),
        translation_scale: inputs.at("translation_scale_Ls_m").value(),
    }
}

fn attachments(inputs: &Json) -> (ConnectorAttachment, ConnectorAttachment) {
    match inputs.get("attachment_local") {
        Some(local) if !local.is_null() => (
            ConnectorAttachment {
                node_axes: local.at("initial_node_axes_global_i").matrix3(),
                offset_local: local.at("offset_local_i").vec3(),
            },
            ConnectorAttachment {
                node_axes: local.at("initial_node_axes_global_j").matrix3(),
                offset_local: local.at("offset_local_j").vec3(),
            },
        ),
        _ => (
            ConnectorAttachment::global(inputs.at("a_i_global").vec3()),
            ConnectorAttachment::global(inputs.at("a_j_global").vec3()),
        ),
    }
}

fn connector_from(inputs: &Json, q_ref: [f64; 6]) -> Result<ObjectiveConnector, ConnectorError> {
    let (end_i, end_j) = attachments(inputs);
    ObjectiveConnector::new(
        FrameNode::new(0, inputs.at("x_i").vec3()).unwrap(),
        FrameNode::new(1, inputs.at("x_j").vec3()).unwrap(),
        end_i,
        end_j,
        inputs.at("Q_row_major_columns_are_axes").matrix3(),
        work_matrix(inputs),
        q_ref,
    )
}

fn q_ref_of(inputs: &Json) -> [f64; 6] {
    inputs.at("q_ref").values().try_into().unwrap()
}

fn exact_matvec(m: &[[f64; 6]; 6], v: &[f64; 6]) -> [f64; 6] {
    let mut out = [0.0; 6];
    for (k, slot) in out.iter_mut().enumerate() {
        let mut sum = ExactAccumulator::new();
        for l in 0..6 {
            sum.add_product(m[k][l], v[l]).unwrap();
        }
        *slot = sum.round().unwrap();
    }
    out
}

/// The standard evaluation of a unit case: every key of `expected` that the
/// connector publishes, by R02-Z.
fn evaluate(connector: &ObjectiveConnector, expected: &Json, floors: &Floors) {
    evaluate_with(connector, expected, floors, None)
}

/// `evaluate`, with the displacements given by the case's inputs when its
/// expected object does not carry them.
fn evaluate_with(
    connector: &ObjectiveConnector,
    expected: &Json,
    floors: &Floors,
    d_input: Option<&Json>,
) {
    let check = |what: &str, family: &str, observed: &[f64]| {
        if let Some(e) = expected.get(what) {
            floors.check(what, family, observed, &e.values());
        }
    };
    check("r", "geometry", &connector.chord());
    if let Some(b) = expected.get("B") {
        let observed = connector.b();
        for (row, e) in b.items().iter().enumerate() {
            floors.check("B", "B", &observed[row], &e.values());
        }
    }
    let ke = connector.global_stiffness().unwrap();
    if let Some(e) = expected.get("Ke") {
        for (row, e) in e.items().iter().enumerate() {
            floors.check("Ke", "Ke", &ke[row], &e.values());
        }
    }
    if let Some(pd) = expected.get("K_pd_exact") {
        assert_eq!(
            connector.definiteness() == ConnectorDefiniteness::PositiveDefinite,
            pd.boolean(),
            "{} PD",
            floors.pointer
        );
    }
    if let Some(stress_free) = expected.get("stress_free") {
        assert_eq!(
            connector.stress_free(),
            stress_free.boolean(),
            "{}",
            floors.pointer
        );
    }
    let kq = exact_matvec(&connector.stiffness(), &connector.q_ref());
    check_split(expected, floors, "K_qref", &kq, "force", "moment");
    let rhs = connector
        .reference_load()
        .unwrap()
        .map_or([0.0; 12], |load| load.values);
    if let Some(e) = expected.get("installed_rhs_BT_K_qref") {
        check_blocks(floors, "installed_rhs_BT_K_qref", &rhs, &e.values());
    }
    if let Some(rank_ke) = expected.get("rank_Ke") {
        // rank(BᵀKB) = rank(KB) for PSD K, on the exact B.
        let b = exact_b(connector);
        let k = connector.stiffness();
        let kb: Vec<Vec<Dy>> = (0..6)
            .map(|row| {
                (0..12)
                    .map(|col| {
                        let mut sum = Dy::zero();
                        for l in 0..6 {
                            sum = sum.add(&Dy::from(k[row][l]).mul(&b[l][col]));
                        }
                        sum
                    })
                    .collect()
            })
            .collect();
        assert_eq!(rank(&kb), rank_ke.usize(), "{} rank Ke", floors.pointer);
    }
    if let Some(rank_b) = expected.get("rank_B") {
        assert_eq!(
            rank(&exact_b(connector)),
            rank_b.usize(),
            "{} rank B",
            floors.pointer
        );
    }
    let Some(d) = expected.get("d").or(d_input) else {
        return;
    };
    let d: [f64; 12] = d.values().try_into().unwrap();
    let recovered = connector.recover(&d).unwrap();
    check_split(
        expected,
        floors,
        "q",
        &recovered.q,
        "translation",
        "rotation",
    );
    check_split(expected, floors, "g", &recovered.g, "force", "moment");
    check("energy", "energy", &[recovered.energy]);
    check("F_global", "force", &recovered.force_global);
    check("M_global", "moment", &recovered.moment_global);
    if let Some(actions) = expected.get("end_actions_node_on_element") {
        for (key, family, range) in [
            ("Fi", "force", 0..3),
            ("Mi", "moment", 3..6),
            ("Fj", "force", 6..9),
            ("Mj", "moment", 9..12),
        ] {
            floors.check(
                key,
                family,
                &recovered.end_actions[range],
                &actions.at(key).values(),
            );
        }
    }
}

/// A 6-vector whose first half is `first` and second half `second`.
fn check_split(
    expected: &Json,
    floors: &Floors,
    what: &str,
    observed: &[f64; 6],
    first: &str,
    second: &str,
) {
    if let Some(e) = expected.get(what) {
        let e = e.values();
        floors.check(what, first, &observed[..3], &e[..3]);
        floors.check(what, second, &observed[3..], &e[3..]);
    }
}

/// A 12-vector of [force, moment, force, moment] blocks.
fn check_blocks(floors: &Floors, what: &str, observed: &[f64; 12], expected: &[f64]) {
    for (family, range) in [
        ("force", 0..3),
        ("moment", 3..6),
        ("force", 6..9),
        ("moment", 9..12),
    ] {
        floors.check(what, family, &observed[range.clone()], &expected[range]);
    }
}

fn standard(root: &Json, id: &str) {
    let c = case(root, id);
    let inputs = c.at("inputs");
    let connector = connector_from(inputs, q_ref_of(inputs)).unwrap();
    evaluate_with(
        &connector,
        c.at("expected"),
        &Floors::of(root, &format!("/cases/{id}/expected")),
        inputs.get("d"),
    );
}

// ------------------------------------------------------------------ JR cases

#[test]
fn jr_j1_and_j2_and_the_end_moment_control() {
    let root = references();
    for id in [
        "U3-J1-LATERAL",
        "U3-J1-COMMON-ROTATION",
        "U3-J2-ROTATION",
        "U3-J2-ROTATION-HELD",
        "U3-REF-ENDMOMENT",
    ] {
        standard(&root, id);
    }
}

#[test]
fn generic_skew_offset_prestress_covariance_offsets_and_reversal() {
    let root = references();
    for id in [
        "U3-GENERIC-SKEW-OFFSET-PRESTRESS",
        "U3-FRAME-COVARIANCE",
        "U3-OFFSETS",
        "U3-REVERSAL",
    ] {
        standard(&root, id);
    }
}

#[test]
fn six_components_each_coordinate_isolated() {
    let root = references();
    let c = case(&root, "U3-SIX-COMPONENTS");
    let inputs = c.at("inputs");
    let connector = connector_from(inputs, q_ref_of(inputs)).unwrap();
    for sub in ["tx", "ty", "tz", "rx", "ry", "rz"] {
        evaluate(
            &connector,
            c.at("expected").at(sub),
            &Floors::of(&root, &format!("/cases/U3-SIX-COMPONENTS/expected/{sub}")),
        );
    }
}

/// U3-REVERSAL against U3-GENERIC: the reversed connector's end-action
/// blocks are the generic ones exchanged, q' = Tq, and the energy is equal.
#[test]
fn reversal_exchanges_the_end_blocks() {
    let root = references();
    let generic = case(&root, "U3-GENERIC-SKEW-OFFSET-PRESTRESS");
    let reversed = case(&root, "U3-REVERSAL");
    let a = connector_from(generic.at("inputs"), q_ref_of(generic.at("inputs"))).unwrap();
    let b = connector_from(reversed.at("inputs"), q_ref_of(reversed.at("inputs"))).unwrap();
    let d: [f64; 12] = generic.at("expected").at("d").values().try_into().unwrap();
    let mut swapped = [0.0; 12];
    swapped[..6].copy_from_slice(&d[6..]);
    swapped[6..].copy_from_slice(&d[..6]);
    let ra = a.recover(&d).unwrap();
    let rb = b.recover(&swapped).unwrap();
    let floors = Floors::of(&root, "/cases/U3-REVERSAL/expected");
    floors.check("Fi'", "force", &rb.end_actions[0..3], &ra.end_actions[6..9]);
    floors.check(
        "Mi'",
        "moment",
        &rb.end_actions[3..6],
        &ra.end_actions[9..12],
    );
    floors.check("Fj'", "force", &rb.end_actions[6..9], &ra.end_actions[0..3]);
    floors.check(
        "Mj'",
        "moment",
        &rb.end_actions[9..12],
        &ra.end_actions[3..6],
    );
    floors.check("U'", "energy", &[rb.energy], &[ra.energy]);
    // T = blockdiag(-J, -J), J = diag(-1, 1, -1): q' = (q0, -q1, q2, q3, -q4, q5).
    let t = [1.0, -1.0, 1.0, 1.0, -1.0, 1.0];
    let tq: Vec<f64> = ra.q.iter().zip(t).map(|(q, s)| q * s).collect();
    floors.check("q'", "translation", &rb.q[..3], &tq[..3]);
    floors.check("q'", "rotation", &rb.q[3..], &tq[3..]);
}

#[test]
fn coupled_h_congruent_rescaling_and_installed_preload() {
    let root = references();
    let c = case(&root, "U3-COUPLED-H-SCALE-PRELOAD");
    let inputs = c.at("inputs").at("coupled");
    let coupled = connector_from(inputs, q_ref_of(inputs)).unwrap();
    let pointer = "/cases/U3-COUPLED-H-SCALE-PRELOAD/expected";
    evaluate(
        &coupled,
        c.at("expected").at("coupled"),
        &Floors::of(&root, &format!("{pointer}/coupled")),
    );
    assert_eq!(
        coupled.definiteness(),
        ConnectorDefiniteness::PositiveSemidefinite
    );
    // q-hat = D^-1 q.
    let d: [f64; 12] = c
        .at("expected")
        .at("coupled")
        .at("d")
        .values()
        .try_into()
        .unwrap();
    let q = coupled.recover(&d).unwrap().q;
    let ls = coupled.work_matrix().translation_scale;
    let qhat = [q[0] / ls, q[1] / ls, q[2] / ls, q[3], q[4], q[5]];
    Floors::of(&root, &format!("{pointer}/qhat")).check(
        "qhat",
        "scaled_coordinate",
        &qhat,
        &c.at("expected").at("qhat").values(),
    );
    // The decode is congruent: H' at Ls = 1 m gives the same K, bit for bit.
    let h1: [f64; 21] = c
        .at("inputs")
        .at("rescaled_H_upper_triangle_Ls_1m")
        .values()
        .try_into()
        .unwrap();
    let (end_i, end_j) = attachments(inputs);
    let at_one = ObjectiveConnector::new(
        coupled.node_i(),
        coupled.node_j(),
        end_i,
        end_j,
        coupled.axes(),
        ScaledWorkMatrix {
            upper_triangle: h1,
            translation_scale: 1.0,
        },
        coupled.q_ref(),
    )
    .unwrap();
    assert_eq!(at_one.stiffness(), coupled.stiffness());
    // Installed prestress at d = 0.
    let preload: [f64; 6] = c
        .at("inputs")
        .at("preload_q_ref")
        .values()
        .try_into()
        .unwrap();
    let prestressed = connector_from(inputs, preload).unwrap();
    evaluate(
        &prestressed,
        c.at("expected").at("preload_installed_d0"),
        &Floors::of(&root, &format!("{pointer}/preload_installed_d0")),
    );
}

#[test]
fn b_oracle_from_b() {
    // N-4: B from `b()`, then f = Bᵀg, q = B d and g·q = f·d.
    let root = references();
    let c = case(&root, "U3-B-ORACLE");
    let inputs = c.at("inputs");
    let diagonal = {
        let mut h = [0.0; 21];
        for (k, index) in [0, 6, 11, 15, 18, 20].into_iter().enumerate() {
            h[index] = 1.0 + k as f64;
        }
        h
    };
    let connector = ObjectiveConnector::new(
        FrameNode::new(0, inputs.at("x_i").vec3()).unwrap(),
        FrameNode::new(1, inputs.at("x_j").vec3()).unwrap(),
        ConnectorAttachment::global(inputs.at("a_i_global").vec3()),
        ConnectorAttachment::global(inputs.at("a_j_global").vec3()),
        inputs.at("Q_row_major_columns_are_axes").matrix3(),
        ScaledWorkMatrix {
            upper_triangle: diagonal,
            translation_scale: 1.0,
        },
        [0.0; 6],
    )
    .unwrap();
    let floors = Floors::of(&root, "/cases/U3-B-ORACLE/expected");
    let expected = c.at("expected");
    floors.check("r", "B", &connector.chord(), &inputs.at("r").values());
    let b = connector.b();
    for (row, e) in expected.at("B").items().iter().enumerate() {
        floors.check("B", "B", &b[row], &e.values());
    }
    let g: Vec<f64> = inputs.at("g_given").values();
    let d: Vec<f64> = inputs.at("d_given").values();
    let mut f = [0.0; 12];
    for (i, slot) in f.iter_mut().enumerate() {
        let mut sum = ExactAccumulator::new();
        for k in 0..6 {
            sum.add_product(b[k][i], g[k]).unwrap();
        }
        *slot = sum.round().unwrap();
    }
    let actions = expected.at("end_actions_node_on_element");
    for (key, family, range) in [
        ("Fi", "force", 0..3),
        ("Mi", "moment", 3..6),
        ("Fj", "force", 6..9),
        ("Mj", "moment", 9..12),
    ] {
        floors.check(key, family, &f[range], &actions.at(key).values());
    }
    let mut q = [0.0; 6];
    for (k, slot) in q.iter_mut().enumerate() {
        let mut sum = ExactAccumulator::new();
        for l in 0..12 {
            sum.add_product(b[k][l], d[l]).unwrap();
        }
        *slot = sum.round().unwrap();
    }
    let qe = expected.at("q_for_d_given").values();
    floors.check("q", "translation", &q[..3], &qe[..3]);
    floors.check("q", "rotation", &q[3..], &qe[3..]);
    let work = |a: &[f64], b: &[f64]| {
        let mut sum = ExactAccumulator::new();
        for (x, y) in a.iter().zip(b) {
            sum.add_product(*x, *y).unwrap();
        }
        sum.round().unwrap()
    };
    let virtual_work = expected.at("virtual_work").value();
    floors.check("g.q", "energy", &[work(&g, &q)], &[virtual_work]);
    floors.check("f.d", "energy", &[work(&f, &d)], &[virtual_work]);
}

#[test]
fn finite_rotation_is_reported_never_suppressed() {
    let root = references();
    let c = case(&root, "U3-FINITE-ROTATION-NEGATIVE");
    for row in c.at("expected").at("rows").items() {
        let l = row.at("L_m").value();
        let phi = row.at("phi_rad").value();
        let mut h = [0.0; 21];
        for index in [0, 6, 11, 15, 18, 20] {
            h[index] = 1.0;
        }
        let connector = ObjectiveConnector::new(
            FrameNode::new(0, [0.0; 3]).unwrap(),
            FrameNode::new(1, [l, 0.0, 0.0]).unwrap(),
            ConnectorAttachment::global([0.0; 3]),
            ConnectorAttachment::global([0.0; 3]),
            [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
            ScaledWorkMatrix {
                upper_triangle: h,
                translation_scale: 1.0,
            },
            [0.0; 6],
        )
        .unwrap();
        let block = |key: &str| row.at("d").at(key).values();
        let mut d = [0.0; 12];
        d[0..3].copy_from_slice(&block("u_i"));
        d[3..6].copy_from_slice(&block("theta_i"));
        d[6..9].copy_from_slice(&block("u_j"));
        d[9..12].copy_from_slice(&block("theta_j"));
        let q = connector.recover(&d).unwrap().q;
        let qt = row.at("qt").values();
        for k in 0..3 {
            assert!((q[k] - qt[k]).abs() <= 1e-12 * l, "L {l} phi {phi} qt[{k}]");
        }
        assert_eq!(&q[3..], &[0.0; 3]);
        let leading = l * phi * phi / 2.0 * (1.0 - phi * phi / 12.0);
        assert!(q[0].abs() >= 0.99 * leading, "finite rotation suppressed");
        assert!(q.iter().any(|v| *v != 0.0));
    }
}

#[test]
fn preload_relief_solved_and_recovered() {
    let root = references();
    let c = case(&root, "U3-PRELOAD-RELIEF");
    let inputs = c.at("inputs");
    let connector = connector_from(inputs, q_ref_of(inputs)).unwrap();
    assert_eq!(
        connector.definiteness(),
        ConnectorDefiniteness::PositiveDefinite
    );
    let expected = c.at("expected");
    let load = connector.reference_load().unwrap().unwrap();
    let floors = Floors::of(&root, "/cases/U3-PRELOAD-RELIEF/expected/both_nodes_held");
    check_blocks(
        &floors,
        "installed_rhs_BT_K_qref",
        &load.values,
        &expected.at("installed_rhs_BT_K_qref").values(),
    );
    let k = assemble_global_stiffness_with_connectors(2, &[], &[connector]).unwrap();
    for (state, restrained) in [
        ("node_i_anchored_node_j_free", vec![0, 1, 2, 3, 4, 5]),
        ("node_i_anchored_node_j_ux_held", vec![0, 1, 2, 3, 4, 5, 6]),
        ("both_nodes_held", (0..12).collect::<Vec<_>>()),
    ] {
        let floors = Floors::of(&root, &format!("/cases/U3-PRELOAD-RELIEF/expected/{state}"));
        let e = expected.at(state);
        let mut u = [0.0; 12];
        if restrained.len() < 12 {
            let reduced = reduce_system(&k, &load.values, &restrained).unwrap();
            let solved = solve_dense(&reduced.stiffness, &reduced.force).unwrap();
            for (dof, value) in reduced.free_dofs.iter().zip(solved) {
                u[*dof] = value;
            }
        }
        if let Some(d) = e.get("d") {
            let d = d.values();
            floors.check("d", "translation", &u[0..3], &d[0..3]);
            floors.check("d", "rotation", &u[3..6], &d[3..6]);
            floors.check("d", "translation", &u[6..9], &d[6..9]);
            floors.check("d", "rotation", &u[9..12], &d[9..12]);
        }
        let recovered = connector.recover(&u).unwrap();
        if let Some(q) = e.get("q") {
            let q = q.values();
            floors.check("q", "translation", &recovered.q[..3], &q[..3]);
            floors.check("q", "rotation", &recovered.q[3..], &q[3..]);
        }
        let g = e.at("g_recovered").values();
        floors.check("g", "force", &recovered.g[..3], &g[..3]);
        floors.check("g", "moment", &recovered.g[3..], &g[3..]);
        if let Some(energy) = e.get("energy") {
            floors.check("energy", "energy", &[recovered.energy], &[energy.value()]);
        }
        // Support-on-element reactions are the end actions Bᵀg (free rows ≈ 0).
        check_blocks(
            &floors,
            "support_on_element_reactions",
            &recovered.end_actions,
            &e.at("support_on_element_reactions").values(),
        );
    }
}

#[test]
fn zero_length_explicit_q_and_its_discriminator() {
    let root = references();
    standard(&root, "U3-ZERO-LENGTH-EXPLICIT-Q");
    let c = case(&root, "U3-ZERO-LENGTH-EXPLICIT-Q");
    let inputs = c.at("inputs");
    let connector = connector_from(inputs, q_ref_of(inputs)).unwrap();
    assert_eq!(connector.chord(), [0.0; 3]);
    let d: [f64; 12] = inputs.at("d").values().try_into().unwrap();
    let floors = Floors::of(&root, "/cases/U3-ZERO-LENGTH-EXPLICIT-Q/expected");
    floors.check(
        "q",
        "translation",
        &connector.recover(&d).unwrap().q[..3],
        &c.at("expected").at("q").values()[..3],
    );
    // The identity-Q discriminator is not reproduced.
    let wrong = c.at("wrong_result_discriminators").items()[0]
        .at("q")
        .values();
    let q = connector.recover(&d).unwrap().q;
    assert!(q.iter().zip(&wrong).any(|(o, w)| (o - w).abs() > 1e-6));
}

#[test]
fn misaligned_axes_are_refused_and_the_control_is_admitted() {
    let root = references();
    let c = case(&root, "U3-QX-MISALIGNED-REFUSAL");
    let inputs = c.at("inputs");
    for variant in ["perpendicular", "antiparallel"] {
        let q = inputs
            .at("variants")
            .at(variant)
            .at("Q_row_major_columns_are_axes")
            .matrix3();
        let result = ObjectiveConnector::new(
            FrameNode::new(0, inputs.at("x_i").vec3()).unwrap(),
            FrameNode::new(1, inputs.at("x_j").vec3()).unwrap(),
            ConnectorAttachment::global(inputs.at("a_i_global").vec3()),
            ConnectorAttachment::global(inputs.at("a_j_global").vec3()),
            q,
            work_matrix(inputs),
            q_ref_of(inputs),
        );
        assert_eq!(result, Err(ConnectorError::AxisMisaligned), "{variant}");
    }
    let control = case(&root, "U3-J1-LATERAL").at("inputs");
    assert!(connector_from(control, q_ref_of(control)).is_ok());
}

#[test]
fn w4_link_rule_decisions_are_exact() {
    let root = references();
    let c = case(&root, "U3-W4-LINK-RULE");
    let geometry = case(&root, "U3-GENERIC-SKEW-OFFSET-PRESTRESS").at("inputs");
    let with_h = |h: &Json, ls: f64, q_ref: [f64; 6]| {
        let (end_i, end_j) = attachments(geometry);
        ObjectiveConnector::new(
            FrameNode::new(0, geometry.at("x_i").vec3()).unwrap(),
            FrameNode::new(1, geometry.at("x_j").vec3()).unwrap(),
            end_i,
            end_j,
            geometry.at("Q_row_major_columns_are_axes").matrix3(),
            ScaledWorkMatrix {
                upper_triangle: h.values().try_into().unwrap(),
                translation_scale: ls,
            },
            q_ref,
        )
    };
    // PD: the generic connector links.
    let pd = connector_from(geometry, q_ref_of(geometry)).unwrap();
    assert_eq!(pd.definiteness(), ConnectorDefiniteness::PositiveDefinite);
    // PSD (rank 2): admitted, unqualified for W4.
    let inputs = c.at("inputs");
    let psd = with_h(inputs.at("psd_H_upper_triangle_21_Ls_2m"), 2.0, [0.0; 6]).unwrap();
    assert_eq!(
        psd.definiteness(),
        ConnectorDefiniteness::PositiveSemidefinite
    );
    let expected = c.at("expected").at("psd_case");
    let d: [f64; 12] = expected
        .at("non_rigid_null_vector_d")
        .values()
        .try_into()
        .unwrap();
    let q = psd.recover(&d).unwrap();
    let its_q = expected.at("its_q").values();
    for k in 0..6 {
        assert!((q.q[k] - its_q[k]).abs() <= 1e-12, "psd q[{k}]");
    }
    // K·its_q = 0 exactly: a free generalized coordinate (no force, no energy).
    let kq = exact_matvec(&psd.stiffness(), &its_q.clone().try_into().unwrap());
    assert_eq!(kq, [0.0; 6]);
    // The null-coordinate q_ref is stress free.
    let nq: [f64; 6] = expected
        .at("null_coordinate_q_ref")
        .at("q_ref")
        .values()
        .try_into()
        .unwrap();
    let null_ref = with_h(inputs.at("psd_H_upper_triangle_21_Ls_2m"), 2.0, nq).unwrap();
    assert!(null_ref.stress_free());
    assert_eq!(null_ref.reference_load().unwrap(), None);
    // Indefinite: refused, never projected.
    assert_eq!(
        with_h(
            inputs.at("indefinite_H_upper_triangle_21_Ls_2m"),
            2.0,
            [0.0; 6]
        ),
        Err(ConnectorError::NotPositiveSemidefinite)
    );
    // The zero-pivot example (round 02 N-2): indefinite, not skipped.
    let example = c.at("decision_operand_round_02").at("zero_pivot_example");
    assert_eq!(
        with_h(
            example.at("H_upper_triangle_21_N_m"),
            example.at("translation_scale_Ls_m").value(),
            [0.0; 6]
        ),
        Err(ConnectorError::NotPositiveSemidefinite)
    );
    // The pivots' signs as published (H for the indefinite case, K else).
    let sign = |v: f64| v.partial_cmp(&0.0).unwrap();
    let pivots = |m: &[[f64; 6]; 6]| exact_pivot_signs(m);
    let published = c
        .at("expected")
        .at("indefinite_case")
        .at("ldl_pivots_H")
        .values();
    let h = sym21(&inputs.at("indefinite_H_upper_triangle_21_Ls_2m").values());
    assert_eq!(
        pivots(&h),
        published.iter().map(|v| sign(*v)).collect::<Vec<_>>()
    );
    assert_eq!(exact_definiteness(&h), ConnectorDefiniteness::Indefinite);
    let published = expected.at("ldl_pivots_K").values();
    assert_eq!(
        pivots(&psd.stiffness()),
        published.iter().map(|v| sign(*v)).collect::<Vec<_>>()
    );
}

/// SF-3 (T4-RV19): the decision reads both the authored H and its binary64
/// decode K = D⁻¹HD⁻¹. With a non-dyadic Ls (0.3 m) the divisions round, so
/// a singular (rank-deficient) H can decode to a K of either inertia. The
/// three H's are rank-1 in their tx–rx block ([[s², st], [st, t²]], other
/// diagonals 1); which K inertia each gives was found independently of FK by
/// exact rational elimination of the IEEE quotients (T4-I27's
/// `decode_inertia.py`, standard library only), and is re-checked here by
/// `exact_definiteness` on `stiffness()`.
#[test]
fn definiteness_reads_h_and_its_binary64_decode_at_a_non_dyadic_ls() {
    let root = references();
    let base = connector_from(case(&root, "U3-J1-LATERAL").at("inputs"), [0.0; 6]).unwrap();
    let with = |tt: f64, tr: f64, rr: f64| {
        let mut upper = [0.0; 21];
        for diagonal in [0, 6, 11, 15, 18, 20] {
            upper[diagonal] = 1.0;
        }
        (upper[0], upper[3], upper[15]) = (tt, tr, rr);
        let h = ScaledWorkMatrix {
            upper_triangle: upper,
            translation_scale: 0.3,
        };
        assert_eq!(
            exact_definiteness(&sym21(&upper)),
            ConnectorDefiniteness::PositiveSemidefinite,
            "H ({tt}, {tr}, {rr}) is singular PSD"
        );
        ObjectiveConnector::new(
            base.node_i(),
            base.node_j(),
            ConnectorAttachment::global([0.0; 3]),
            ConnectorAttachment::global([0.0; 3]),
            base.axes(),
            h,
            [0.0; 6],
        )
    };
    // H PSD, K indefinite: refused under its own reason (never assembled).
    assert_eq!(
        with(1.0, 5.0, 25.0),
        Err(ConnectorError::DecodedStiffnessIndefinite)
    );
    assert!(ConnectorError::DecodedStiffnessIndefinite
        .to_string()
        .contains("non-dyadic Ls"));
    // H PSD, K PD: admitted as PSD (the H operand keeps it unqualified for
    // W4; a K-only decision would link a mechanism).
    let rounded_pd = with(1.0, 1.0, 1.0).unwrap();
    assert_eq!(
        exact_definiteness(&rounded_pd.stiffness()),
        ConnectorDefiniteness::PositiveDefinite
    );
    assert_eq!(
        rounded_pd.definiteness(),
        ConnectorDefiniteness::PositiveSemidefinite
    );
    // Control: H PSD, K PSD.
    let both = with(9.0, 3.0, 1.0).unwrap();
    assert_eq!(
        exact_definiteness(&both.stiffness()),
        ConnectorDefiniteness::PositiveSemidefinite
    );
    assert_eq!(both.definiteness(), ConnectorDefiniteness::PositiveSemidefinite);
}

fn sym21(upper: &[f64]) -> [[f64; 6]; 6] {
    let mut m = [[0.0; 6]; 6];
    let mut index = 0;
    for i in 0..6 {
        for j in i..6 {
            m[i][j] = upper[index];
            m[j][i] = upper[index];
            index = index.wrapping_add(1);
        }
    }
    m
}

/// The signs of an LDLᵀ's pivots (no pivoting; a zero pivot with a zero
/// column skipped), by this file's dyadic arithmetic.
fn exact_pivot_signs(m: &[[f64; 6]; 6]) -> Vec<std::cmp::Ordering> {
    let mut a: Vec<Vec<Dy>> = m
        .iter()
        .map(|r| r.iter().map(|&v| Dy::from(v)).collect())
        .collect();
    let mut signs = Vec::new();
    // The remaining block is the Schur complement times the product of the
    // pivots used so far: its sign is carried.
    let mut flipped = false;
    for k in 0..6 {
        let p = a[k][k].clone();
        signs.push(if flipped {
            p.sign().reverse()
        } else {
            p.sign()
        });
        if p.is_zero() {
            continue;
        }
        flipped ^= p.sign() == std::cmp::Ordering::Less;
        for i in k + 1..6 {
            for j in k + 1..6 {
                a[i][j] = p.mul(&a[i][j]).sub(&a[i][k].mul(&a[k][j]));
            }
        }
    }
    signs
}

// ------------------------------------------------------------------ K-D5 case

#[test]
fn kd5_utm_case_at_the_unit_level() {
    let root = references();
    let c = case(&root, "U3-KD5-UTM-SKEW-OFFSET-COUPLED");
    let shared = c.at("inputs").at("shared");
    let floors = Floors::of(
        &root,
        "/cases/U3-KD5-UTM-SKEW-OFFSET-COUPLED/expected/identical_at_every_location",
    );
    let Json::Object(locations) = c.at("inputs").at("locations") else {
        panic!("locations")
    };
    assert_eq!(locations.len(), 3);
    let mut formed: Vec<[[f64; 12]; 12]> = Vec::new();
    for (name, location) in locations {
        let connector = ObjectiveConnector::new(
            FrameNode::new(0, location.at("x_i").vec3()).unwrap(),
            FrameNode::new(1, location.at("x_j").vec3()).unwrap(),
            ConnectorAttachment::global(shared.at("a_i_global").vec3()),
            ConnectorAttachment::global(shared.at("a_j_global").vec3()),
            shared.at("Q_row_major_columns_are_axes").matrix3(),
            work_matrix(shared),
            q_ref_of(shared),
        )
        .unwrap_or_else(|e| panic!("{name}: {e}"));
        evaluate(
            &connector,
            c.at("expected").at("identical_at_every_location"),
            &floors,
        );
        let d: [f64; 12] = shared.at("d").values().try_into().unwrap();
        let _ = connector.recover(&d).unwrap();
        // The difference form makes B, hence Ke, the same bits everywhere.
        formed.push(connector.global_stiffness().unwrap());
        exact_b_checks(&connector, name);
    }
    assert!(formed.windows(2).all(|w| w[0] == w[1]));
}

// ------------------------------------------------------------------ K2b

#[test]
fn force_scaling_is_exact() {
    let root = references();
    let inputs = case(&root, "U3-GENERIC-SKEW-OFFSET-PRESTRESS").at("inputs");
    let connector = connector_from(inputs, q_ref_of(inputs)).unwrap();
    for b in [20, -20, 2, 100] {
        let scale = ForceScale::new(b).unwrap();
        let formed = connector.force_scaled_global_stiffness(scale).unwrap();
        let reformed = connector
            .force_scaled(scale)
            .unwrap()
            .global_stiffness()
            .unwrap();
        assert_eq!(formed, reformed, "b = {b}");
        let unscaled = connector.global_stiffness().unwrap();
        let factor = 2f64.powi(b);
        for i in 0..12 {
            for j in 0..12 {
                assert_eq!(formed[i][j], unscaled[i][j] * factor);
            }
        }
        // B and q_ref are unscaled; the reference load scales exactly.
        let scaled = connector.force_scaled(scale).unwrap();
        assert_eq!(scaled.b(), connector.b());
        let a = connector.reference_load().unwrap().unwrap().values;
        let s = scaled.reference_load().unwrap().unwrap().values;
        for k in 0..12 {
            assert_eq!(s[k], a[k] * factor);
        }
    }
}

// ------------------------------------------------------------------ S13 bound

/// The reference-load bound holds against the exact BᵀKq_ref of the exact B
/// (this file's dyadic arithmetic), at ordinary and UTM coordinates, and it
/// stays far below the value (not vacuous).
#[test]
fn reference_load_bound_holds_against_the_exact_value() {
    let root = references();
    let generic = case(&root, "U3-GENERIC-SKEW-OFFSET-PRESTRESS").at("inputs");
    let mut connectors = vec![connector_from(generic, q_ref_of(generic)).unwrap()];
    let kd5 = case(&root, "U3-KD5-UTM-SKEW-OFFSET-COUPLED").at("inputs");
    let shared = kd5.at("shared");
    let Json::Object(locations) = kd5.at("locations") else {
        panic!()
    };
    for (_, location) in locations {
        connectors.push(
            ObjectiveConnector::new(
                FrameNode::new(0, location.at("x_i").vec3()).unwrap(),
                FrameNode::new(1, location.at("x_j").vec3()).unwrap(),
                ConnectorAttachment::global(shared.at("a_i_global").vec3()),
                ConnectorAttachment::global(shared.at("a_j_global").vec3()),
                shared.at("Q_row_major_columns_are_axes").matrix3(),
                work_matrix(shared),
                q_ref_of(shared),
            )
            .unwrap(),
        );
    }
    for connector in &connectors {
        let load = connector.reference_load().unwrap().unwrap();
        let b = exact_b(connector);
        let k = connector.stiffness();
        for i in 0..12 {
            let mut t = Dy::zero();
            for row in 0..6 {
                for l in 0..6 {
                    t = t.add(
                        &b[row][i]
                            .mul(&Dy::from(k[row][l]))
                            .mul(&Dy::from(connector.q_ref()[l])),
                    );
                }
            }
            let error = Dy::from(load.values[i]).sub(&t).abs();
            assert!(
                error.le(&Dy::from(load.bounds[i])),
                "entry {i}: bound {:e}",
                load.bounds[i]
            );
            assert!(load.bounds[i] < 1e-9 * load.values[i].abs().max(1.0));
        }
    }
}

// ------------------------------------------------------------------ W4 successor (S-5)

/// S-5 (a) and (b): on the exact B of the connector's own binary64 inputs,
/// rank B = 6 and B·rigid = 0 exactly for all six rigid modes about an
/// arbitrary origin; and `b()` agrees with that exact B entrywise within its
/// rounding (a tripwire: a changed formula fails it).
fn exact_b_checks(connector: &ObjectiveConnector, what: &str) {
    let b = exact_b(connector);
    assert_eq!(rank(&b), 6, "{what}: rank B");
    let xs = [
        connector.node_i().coordinates,
        connector.node_j().coordinates,
    ];
    for origin in [[7.0, -3.0, 5.0], xs[0], [-1.25e6, 3.5, 0.125]] {
        for mode in 0..6 {
            let d = rigid_mode(mode, origin, xs);
            for (row, b_row) in b.iter().enumerate() {
                let mut sum = Dy::zero();
                for (bk, dk) in b_row.iter().zip(&d) {
                    sum = sum.add(&bk.mul(dk));
                }
                assert!(sum.is_zero(), "{what}: B·rigid mode {mode} row {row}");
            }
        }
    }
    // Tripwire: |b̂ − B| ≤ 2^-49·max(1, |B|) entrywise (well above b()'s
    // rounding of a few u, far below any formula change).
    let formed = connector.b();
    for (row, b_row) in b.iter().enumerate() {
        for (col, exact) in b_row.iter().enumerate() {
            let error = Dy::from(formed[row][col]).sub(exact).abs();
            let scale = formed[row][col].abs().max(1.0) * 2f64.powi(-49);
            assert!(error.le(&Dy::from(scale)), "{what}: b()[{row}][{col}]");
        }
    }
}

/// u_k = t + ω × (x_k − o), θ_k = ω, for the unit t (modes 0-2) or ω (3-5).
fn rigid_mode(mode: usize, origin: [f64; 3], xs: [[f64; 3]; 2]) -> Vec<Dy> {
    let mut d = vec![Dy::zero(); 12];
    for (node, x) in xs.iter().enumerate() {
        let base = 6 * node;
        if mode < 3 {
            d[base + mode] = Dy::from(1.0);
        } else {
            let w = mode - 3;
            d[base + 3 + w] = Dy::from(1.0);
            let rel: Vec<Dy> = (0..3)
                .map(|c| Dy::from(x[c]).sub(&Dy::from(origin[c])))
                .collect();
            // ω = e_w: ω × rel.
            let (a, b) = ((w + 1) % 3, (w + 2) % 3);
            d[base + b] = rel[a].clone();
            d[base + a] = Dy::zero().sub(&rel[b]);
        }
    }
    d
}

/// The exact B of the connector's binary64 inputs (r exact, no rounding).
fn exact_b(connector: &ObjectiveConnector) -> Vec<Vec<Dy>> {
    let (xi, xj) = (
        connector.node_i().coordinates,
        connector.node_j().coordinates,
    );
    let (ai, aj) = connector.offsets();
    let q = connector.axes();
    let r: Vec<Dy> = (0..3)
        .map(|c| {
            Dy::from(xj[c])
                .sub(&Dy::from(xi[c]))
                .add(&Dy::from(aj[c]).sub(&Dy::from(ai[c])))
        })
        .collect();
    let half: Vec<Dy> = r.iter().map(|v| v.half()).collect();
    let skew = |v: &[Dy]| -> Vec<Vec<Dy>> {
        let z = Dy::zero();
        vec![
            vec![z.clone(), z.sub(&v[2]), v[1].clone()],
            vec![v[2].clone(), z.clone(), z.sub(&v[0])],
            vec![z.sub(&v[1]), v[0].clone(), z.clone()],
        ]
    };
    let s_r = skew(&half);
    let s_ai = skew(&ai.map(Dy::from));
    let s_aj = skew(&aj.map(Dy::from));
    let mut b = vec![vec![Dy::zero(); 12]; 6];
    for axis in 0..3 {
        for c in 0..3 {
            let qc = Dy::from(q[c][axis]);
            b[axis][c] = Dy::zero().sub(&qc);
            b[axis][6 + c] = qc.clone();
            b[3 + axis][3 + c] = Dy::zero().sub(&qc);
            b[3 + axis][9 + c] = qc;
            let mut ti = Dy::zero();
            let mut tj = Dy::zero();
            for k in 0..3 {
                let qk = Dy::from(q[k][axis]);
                ti = ti.add(&qk.mul(&s_ai[k][c].add(&s_r[k][c])));
                tj = tj.add(&qk.mul(&s_r[k][c].sub(&s_aj[k][c])));
            }
            b[axis][3 + c] = ti;
            b[axis][9 + c] = tj;
        }
    }
    b
}

/// The exact rank of a dyadic matrix (fraction-free elimination).
fn rank(m: &[Vec<Dy>]) -> usize {
    let mut a: Vec<Vec<Dy>> = m.to_vec();
    let (rows, cols) = (a.len(), a[0].len());
    let mut rank = 0;
    for col in 0..cols {
        let Some(p) = (rank..rows).find(|&r| !a[r][col].is_zero()) else {
            continue;
        };
        a.swap(rank, p);
        let pivot = a[rank][col].clone();
        for r in 0..rows {
            if r == rank || a[r][col].is_zero() {
                continue;
            }
            let factor = a[r][col].clone();
            for c in 0..cols {
                a[r][c] = pivot.mul(&a[r][c]).sub(&factor.mul(&a[rank][c]));
            }
        }
        rank = rank.wrapping_add(1);
    }
    rank
}

#[test]
fn w4_successor_exact_b_at_utm_with_offsets_skew_q_zero_and_coincident() {
    let root = references();
    for id in [
        "U3-GENERIC-SKEW-OFFSET-PRESTRESS",
        "U3-FRAME-COVARIANCE",
        "U3-OFFSETS",
        "U3-ZERO-LENGTH-EXPLICIT-Q",
        "U3-J1-LATERAL",
    ] {
        let inputs = case(&root, id).at("inputs");
        exact_b_checks(&connector_from(inputs, q_ref_of(inputs)).unwrap(), id);
    }
    // UTM, r = 0, skew Q: the zero-length case moved to x ≈ 7.3e6 m.
    let zl = case(&root, "U3-ZERO-LENGTH-EXPLICIT-Q").at("inputs");
    let shift = [7.3e6, 5.0e6, 12.5];
    let at = |x: [f64; 3]| [x[0] + shift[0], x[1] + shift[1], x[2] + shift[2]];
    let (end_i, end_j) = attachments(zl);
    let moved = ObjectiveConnector::new(
        FrameNode::new(3, at(zl.at("x_i").vec3())).unwrap(),
        FrameNode::new(7, at(zl.at("x_j").vec3())).unwrap(),
        end_i,
        end_j,
        zl.at("Q_row_major_columns_are_axes").matrix3(),
        work_matrix(zl),
        q_ref_of(zl),
    )
    .unwrap();
    exact_b_checks(&moved, "zero length at UTM");
    // Coincident node positions (distinct nodes), the chord from the offsets
    // alone, at UTM, with Q.x along r = a_j − a_i.
    let generic = case(&root, "U3-GENERIC-SKEW-OFFSET-PRESTRESS").at("inputs");
    let p = [7.3e6 + 0.1, 5.0e6 - 0.3, 41.0];
    let coincident = ObjectiveConnector::new(
        FrameNode::new(0, p).unwrap(),
        FrameNode::new(1, p).unwrap(),
        ConnectorAttachment::global([-0.25, -0.5, -0.5]),
        ConnectorAttachment::global([0.0625, 0.125, 0.125]),
        generic.at("Q_row_major_columns_are_axes").matrix3(),
        work_matrix(generic),
        q_ref_of(generic),
    )
    .unwrap();
    exact_b_checks(&coincident, "coincident nodes at UTM");
}

#[test]
fn constructor_refusals() {
    let root = references();
    let inputs = case(&root, "U3-J1-LATERAL").at("inputs");
    let base = connector_from(inputs, [0.0; 6]).unwrap();
    let rebuild = |node_j: FrameNode, axes: Matrix3, h: ScaledWorkMatrix, q_ref: [f64; 6]| {
        ObjectiveConnector::new(
            base.node_i(),
            node_j,
            ConnectorAttachment::global([0.0; 3]),
            ConnectorAttachment::global([0.0; 3]),
            axes,
            h,
            q_ref,
        )
    };
    let same = FrameNode::new(0, [1.0, 0.0, 0.0]).unwrap();
    assert_eq!(
        rebuild(same, base.axes(), base.work_matrix(), [0.0; 6]),
        Err(ConnectorError::RepeatedNode { node_index: 0 })
    );
    // Rows read as axes (Q transposed) is no longer along r for a skew Q.
    let generic = case(&root, "U3-GENERIC-SKEW-OFFSET-PRESTRESS").at("inputs");
    let q = generic.at("Q_row_major_columns_are_axes").matrix3();
    let transposed = [
        [q[0][0], q[1][0], q[2][0]],
        [q[0][1], q[1][1], q[2][1]],
        [q[0][2], q[1][2], q[2][2]],
    ];
    let (end_i, end_j) = attachments(generic);
    assert_eq!(
        ObjectiveConnector::new(
            FrameNode::new(0, generic.at("x_i").vec3()).unwrap(),
            FrameNode::new(1, generic.at("x_j").vec3()).unwrap(),
            end_i,
            end_j,
            transposed,
            work_matrix(generic),
            [0.0; 6],
        ),
        Err(ConnectorError::AxisMisaligned)
    );
    // An improper (reflected) or non-orthonormal triad.
    let reflected = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, -1.0]];
    assert_eq!(
        rebuild(base.node_j(), reflected, base.work_matrix(), [0.0; 6]),
        Err(ConnectorError::ImproperTriad {
            name: "connector axes"
        })
    );
    let skewed = [[1.0, 1e-9, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
    assert!(matches!(
        rebuild(base.node_j(), skewed, base.work_matrix(), [0.0; 6]),
        Err(ConnectorError::ImproperTriad { .. })
    ));
    let mut h = base.work_matrix();
    h.translation_scale = 0.0;
    assert_eq!(
        rebuild(base.node_j(), base.axes(), h, [0.0; 6]),
        Err(ConnectorError::TranslationScale)
    );
    let mut h = base.work_matrix();
    h.upper_triangle[3] = f64::NAN;
    assert!(matches!(
        rebuild(base.node_j(), base.axes(), h, [0.0; 6]),
        Err(ConnectorError::NonFinite { .. })
    ));
    assert!(matches!(
        rebuild(
            base.node_j(),
            base.axes(),
            base.work_matrix(),
            [f64::INFINITY; 6]
        ),
        Err(ConnectorError::NonFinite { .. })
    ));
    // A zero-length chord requires nothing of Q.x, but the triad must be
    // proper (an explicit Q, never an inserted identity).
    let zero = FrameNode::new(1, base.node_i().coordinates).unwrap();
    assert!(rebuild(zero, q, base.work_matrix(), [0.0; 6]).is_ok());
}

/// The dense assembly with no connector is `assemble_global_stiffness`, and
/// with one it scatters Ke on the connector's DOFs, after the frames.
#[test]
fn dense_assembly_places_ke() {
    let root = references();
    let inputs = case(&root, "U3-GENERIC-SKEW-OFFSET-PRESTRESS").at("inputs");
    let c = connector_from(inputs, q_ref_of(inputs)).unwrap();
    let moved = ObjectiveConnector::new(
        FrameNode::new(2, c.node_i().coordinates).unwrap(),
        FrameNode::new(0, c.node_j().coordinates).unwrap(),
        attachments(inputs).0,
        attachments(inputs).1,
        c.axes(),
        c.work_matrix(),
        c.q_ref(),
    )
    .unwrap();
    let k = assemble_global_stiffness_with_connectors(3, &[], &[moved]).unwrap();
    let ke = moved.global_stiffness().unwrap();
    let map: Vec<usize> = (12..18).chain(0..6).collect();
    for i in 0..12 {
        for j in 0..12 {
            assert_eq!(k[map[i]][map[j]], ke[i][j]);
        }
    }
    assert!(k[6..12].iter().all(|row| row.iter().all(|v| *v == 0.0)));
    assert_eq!(
        assemble_global_stiffness_with_connectors(2, &[], &[moved]),
        Err(
            open_pipe_stress_frame_kernel::FrameKernelError::InvalidNodeIndex {
                node_index: 2,
                node_count: 2
            }
        )
    );
}

// ------------------------------------------------------------------ dyadics

/// An exact dyadic rational m·2^e (m a signed big integer, test-only).
#[derive(Debug, Clone)]
struct Dy {
    negative: bool,
    magnitude: Vec<u64>,
    exponent: i64,
}

impl Dy {
    fn zero() -> Self {
        Self {
            negative: false,
            magnitude: Vec::new(),
            exponent: 0,
        }
    }
    fn from(value: f64) -> Self {
        assert!(value.is_finite());
        if value == 0.0 {
            return Self::zero();
        }
        let bits = value.to_bits();
        let e = ((bits >> 52) & 0x7ff) as i64;
        let f = bits & ((1u64 << 52) - 1);
        let (m, x) = if e == 0 {
            (f, -1074)
        } else {
            (f | (1u64 << 52), e - 1075)
        };
        Self {
            negative: value < 0.0,
            magnitude: vec![m],
            exponent: x,
        }
        .trimmed()
    }
    fn trimmed(mut self) -> Self {
        while self.magnitude.last() == Some(&0) {
            self.magnitude.pop();
        }
        if self.magnitude.is_empty() {
            return Self::zero();
        }
        self
    }
    fn is_zero(&self) -> bool {
        self.magnitude.is_empty()
    }
    fn sign(&self) -> std::cmp::Ordering {
        if self.is_zero() {
            std::cmp::Ordering::Equal
        } else if self.negative {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    }
    fn half(&self) -> Self {
        let mut h = self.clone();
        h.exponent = h.exponent.wrapping_sub(1);
        h
    }
    fn abs(&self) -> Self {
        let mut a = self.clone();
        a.negative = false;
        a
    }
    /// The magnitude shifted left by `bits`.
    fn shifted(magnitude: &[u64], bits: u64) -> Vec<u64> {
        let words = (bits / 64) as usize;
        let rest = bits % 64;
        let mut out = vec![0u64; words];
        let mut carry = 0u64;
        for &w in magnitude {
            if rest == 0 {
                out.push(w);
            } else {
                out.push((w << rest) | carry);
                carry = w >> (64 - rest);
            }
        }
        out.push(carry);
        out
    }
    fn mul(&self, other: &Self) -> Self {
        if self.is_zero() || other.is_zero() {
            return Self::zero();
        }
        let mut out = vec![0u64; self.magnitude.len() + other.magnitude.len() + 1];
        for (i, &a) in self.magnitude.iter().enumerate() {
            let mut carry = 0u128;
            for (j, &b) in other.magnitude.iter().enumerate() {
                let t = u128::from(a) * u128::from(b) + u128::from(out[i + j]) + carry;
                out[i + j] = t as u64;
                carry = t >> 64;
            }
            let mut k = i + other.magnitude.len();
            while carry != 0 {
                let t = u128::from(out[k]) + carry;
                out[k] = t as u64;
                carry = t >> 64;
                k = k.wrapping_add(1);
            }
        }
        Self {
            negative: self.negative != other.negative,
            magnitude: out,
            exponent: self.exponent + other.exponent,
        }
        .trimmed()
    }
    fn add(&self, other: &Self) -> Self {
        if self.is_zero() {
            return other.clone();
        }
        if other.is_zero() {
            return self.clone();
        }
        let e = self.exponent.min(other.exponent);
        let a = Self::shifted(&self.magnitude, (self.exponent - e) as u64);
        let b = Self::shifted(&other.magnitude, (other.exponent - e) as u64);
        if self.negative == other.negative {
            let n = a.len().max(b.len()) + 1;
            let mut out = Vec::with_capacity(n);
            let mut carry = 0u128;
            for i in 0..n {
                let t = u128::from(*a.get(i).unwrap_or(&0))
                    + u128::from(*b.get(i).unwrap_or(&0))
                    + carry;
                out.push(t as u64);
                carry = t >> 64;
            }
            return Self {
                negative: self.negative,
                magnitude: out,
                exponent: e,
            }
            .trimmed();
        }
        let (big, small, negative) = match cmp_mag(&a, &b) {
            std::cmp::Ordering::Equal => return Self::zero(),
            std::cmp::Ordering::Greater => (a, b, self.negative),
            std::cmp::Ordering::Less => (b, a, other.negative),
        };
        let mut out = Vec::with_capacity(big.len());
        let mut borrow = 0u64;
        for (i, &w) in big.iter().enumerate() {
            let (d1, o1) = w.overflowing_sub(*small.get(i).unwrap_or(&0));
            let (d2, o2) = d1.overflowing_sub(borrow);
            out.push(d2);
            borrow = u64::from(o1 || o2);
        }
        Self {
            negative,
            magnitude: out,
            exponent: e,
        }
        .trimmed()
    }
    fn sub(&self, other: &Self) -> Self {
        let mut negated = other.clone();
        negated.negative = !negated.negative;
        self.add(&negated.trimmed())
    }
    fn le(&self, other: &Self) -> bool {
        other.sub(self).sign() != std::cmp::Ordering::Less
    }
}

fn cmp_mag(a: &[u64], b: &[u64]) -> std::cmp::Ordering {
    let strip = |v: &[u64]| {
        let mut n = v.len();
        while n > 0 && v[n - 1] == 0 {
            n -= 1;
        }
        n
    };
    let (na, nb) = (strip(a), strip(b));
    na.cmp(&nb)
        .then_with(|| a[..na].iter().rev().cmp(b[..nb].iter().rev()))
}

#[test]
fn dyadic_arithmetic_self_check() {
    let a = Dy::from(0.1);
    let b = Dy::from(0.2);
    let s = a.add(&b);
    // 0.1 + 0.2 exactly is not 0.3 in binary64, and not the rounded sum.
    assert!(!s.sub(&Dy::from(0.1 + 0.2)).is_zero());
    assert!(s.sub(&b).sub(&a).is_zero());
    assert!(Dy::from(3.0)
        .mul(&Dy::from(-0.5))
        .sub(&Dy::from(-1.5))
        .is_zero());
    assert!(
        Dy::from(1e300)
            .mul(&Dy::from(1e-300))
            .sub(&Dy::from(1.0))
            .sign()
            != std::cmp::Ordering::Equal
    );
    assert!(Dy::from(-2.0).le(&Dy::from(1.0)));
    assert!(!Dy::from(2.0).le(&Dy::from(1.0)));
    assert_eq!(
        rank(&[
            vec![Dy::from(1.0), Dy::from(2.0)],
            vec![Dy::from(2.0), Dy::from(4.0)]
        ]),
        1
    );
}
