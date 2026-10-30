//! The committed case files (`cases/*.jsonl`, written by `gen_vk_cases.py`
//! from R1's frozen references), their kernel models and R1's key map.
//!
//! - A model is V-K's adapted kernel input (plan §4.1): binary64 bits in R1's
//!   node and member order, springs as global-axis or directional, rigid DOFs,
//!   loads (RF-CANCEL's contributions one each) and one station per member at
//!   0.5. `Model::source_parts` builds K4's `SourceParts` from it.
//! - `Model::resolve` maps an R1 key to the kernel quantity it is compared with
//!   (plan §4.2).
//! - Every expected value, scale and control value stays R1's decimal string.
use open_pipe_stress_frame_kernel::structural::retained_api::{
    Component, Constraint, DirectionalSpring, Dof, NodalLoad, SourceParts, Spring, SpringKind,
    Station, StraightMember,
};
use serde_json::Value;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

/// The ten family files, in R1's family order.
pub const FAMILY_FILES: [(&str, &str); 10] = [
    ("RF-CHAIN", "rf_chain.jsonl"),
    ("RF-SKEW", "rf_skew.jsonl"),
    ("RF-WEAK", "rf_weak.jsonl"),
    ("RF-LARGE", "rf_large.jsonl"),
    ("RF-INVARIANCE", "rf_invariance.jsonl"),
    ("RF-RANGE", "rf_range.jsonl"),
    ("RF-ZERO", "rf_zero.jsonl"),
    ("RF-FINITE", "rf_finite.jsonl"),
    ("RF-MECH", "rf_mech.jsonl"),
    ("RF-CANCEL", "rf_cancel.jsonl"),
];

pub fn crate_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

pub fn cases_dir() -> PathBuf {
    crate_dir().join("cases")
}

#[derive(Clone, Debug)]
pub struct Row {
    pub key: String,
    pub expected: String,
    pub class: String,
    /// RF-CANCEL's recommended (binding, net-governed) scale; None elsewhere.
    pub scale: Option<String>,
}

impl Row {
    /// The class without RF-WEAK's `@region` suffix.
    pub fn kind(&self) -> &str {
        self.class.split('@').next().unwrap()
    }
}

#[derive(Clone, Debug)]
pub enum ControlKind {
    /// R1's violating values of a value control, by key.
    Value(Vec<(String, String)>),
    /// An outcome control: the defect it describes.
    Outcome(String),
}

#[derive(Clone, Debug)]
pub struct Control {
    pub id: String,
    pub discriminates: bool,
    pub kind: ControlKind,
}

#[derive(Clone, Debug)]
pub struct Member {
    pub name: String,
    pub id: u32,
    pub node_i: u32,
    pub node_j: u32,
    pub elastic_modulus: f64,
    pub shear_modulus: f64,
    pub area: f64,
    pub second_moment_y: f64,
    pub second_moment_z: f64,
    pub torsion_constant: f64,
    pub y_reference: [f64; 3],
}

#[derive(Clone, Debug)]
pub struct SpringSpec {
    /// R1's `S.<node>.<index>`.
    pub key: String,
    pub id: u32,
    pub node: u32,
    pub translation: bool,
    /// The global component (0–5) of a global-axis spring; None for a
    /// directional one.
    pub axis: Option<usize>,
    pub direction: [f64; 3],
    pub stiffness: f64,
}

#[derive(Clone, Debug)]
pub struct Model {
    pub node_names: Vec<String>,
    pub nodes: Vec<[f64; 3]>,
    pub members: Vec<Member>,
    pub springs: Vec<SpringSpec>,
    /// R1 springs with k = 0, omitted by the adapter (RF-MECH-K0; plan §4.1).
    pub omitted_springs: Vec<String>,
    pub constraints: Vec<(u32, usize)>,
    pub loads: Vec<(u32, usize, f64, String)>,
    pub stations: Vec<(u32, u32, f64)>,
}

#[derive(Clone, Debug)]
pub struct Case {
    pub id: String,
    pub family: String,
    pub basis: String,
    pub units: String,
    pub needs_directional_spring: bool,
    pub refuse: bool,
    pub k4src_sha256: String,
    /// None for RF-LARGE at 1,000 and 10,000 members (generated on demand).
    pub model: Option<Model>,
    pub model_sha256: Option<String>,
    pub scales: BTreeMap<String, String>,
    pub rows: Vec<Row>,
    pub controls: Vec<Control>,
    /// The committed not-covered keys of this case (DESIGN.md §4.10's list).
    pub not_covered: Vec<String>,
    /// S(kind) of the complete reference solution (n ≥ 1,000 only).
    pub s_full: Option<BTreeMap<String, String>>,
}

impl Case {
    /// The comparison scale string of a row: its own (RF-CANCEL) or its class's.
    pub fn scale_of<'a>(&'a self, row: &'a Row) -> &'a str {
        match &row.scale {
            Some(s) => s,
            None => &self.scales[&row.class],
        }
    }

    /// RF-LARGE at 1,000 or 10,000 members: an example, never a CI test.
    pub fn is_large(&self) -> bool {
        self.model.is_none()
    }
}

fn hex(v: &Value) -> f64 {
    let s = v.as_str().expect("hex string");
    f64::from_bits(u64::from_str_radix(s, 16).expect("16 hex digits"))
}

fn text(v: &Value) -> String {
    v.as_str().expect("string").to_string()
}

fn uint(v: &Value) -> u32 {
    u32::try_from(v.as_u64().expect("integer")).expect("u32")
}

pub fn parse_model(m: &Value) -> Model {
    let arr = |k: &str| m[k].as_array().unwrap_or_else(|| panic!("model.{k}"));
    let mut node_names = Vec::new();
    let mut nodes = Vec::new();
    for n in arr("nodes") {
        node_names.push(text(&n[0]));
        nodes.push([hex(&n[1]), hex(&n[2]), hex(&n[3])]);
    }
    let members = arr("members")
        .iter()
        .map(|x| Member {
            name: text(&x[0]),
            id: uint(&x[1]),
            node_i: uint(&x[2]),
            node_j: uint(&x[3]),
            elastic_modulus: hex(&x[4]),
            shear_modulus: hex(&x[5]),
            area: hex(&x[6]),
            second_moment_y: hex(&x[7]),
            second_moment_z: hex(&x[8]),
            torsion_constant: hex(&x[9]),
            y_reference: [hex(&x[10]), hex(&x[11]), hex(&x[12])],
        })
        .collect();
    let springs = arr("springs")
        .iter()
        .map(|s| {
            let d = s[5].as_array().expect("direction");
            SpringSpec {
                key: text(&s[0]),
                id: uint(&s[1]),
                node: uint(&s[2]),
                translation: s[3].as_str() == Some("t"),
                axis: s[4].as_u64().map(|a| a as usize),
                direction: [hex(&d[0]), hex(&d[1]), hex(&d[2])],
                stiffness: hex(&s[6]),
            }
        })
        .collect();
    Model {
        node_names,
        nodes,
        members,
        springs,
        omitted_springs: arr("omitted_springs").iter().map(text).collect(),
        constraints: arr("constraints")
            .iter()
            .map(|c| (uint(&c[0]), uint(&c[1]) as usize))
            .collect(),
        loads: arr("loads")
            .iter()
            .map(|l| (uint(&l[0]), uint(&l[1]) as usize, hex(&l[2]), text(&l[3])))
            .collect(),
        stations: arr("stations")
            .iter()
            .map(|s| (uint(&s[0]), uint(&s[1]), hex(&s[2])))
            .collect(),
    }
}

pub fn parse_case(line: &str) -> Case {
    parse_case_capture(line, None)
}

fn parse_case_capture(line: &str, facts: Option<&mut crate::envelope::FamilyInputFacts>) -> Case {
    let v: Value = serde_json::from_str(line).expect("case JSON");
    let rows = v["rows"]
        .as_array()
        .expect("rows")
        .iter()
        .map(|r| Row {
            key: text(&r[0]),
            expected: text(&r[1]),
            class: text(&r[2]),
            scale: r[3].as_str().map(str::to_string),
        })
        .collect();
    let controls = v["controls"]
        .as_array()
        .expect("controls")
        .iter()
        .map(|c| Control {
            id: text(&c[0]),
            discriminates: c[1].as_bool().expect("bool"),
            kind: if c[2].as_str() == Some("value") {
                ControlKind::Value(
                    c[3].as_object()
                        .expect("values")
                        .iter()
                        .map(|(k, v)| (k.clone(), text(v)))
                        .collect(),
                )
            } else {
                ControlKind::Outcome(text(&c[3]["defect"]))
            },
        })
        .collect();
    let strings = |x: &Value| -> BTreeMap<String, String> {
        x.as_object()
            .map(|o| o.iter().map(|(k, v)| (k.clone(), text(v))).collect())
            .unwrap_or_default()
    };
    let case = Case {
        id: text(&v["id"]),
        family: text(&v["family"]),
        basis: text(&v["basis"]),
        units: text(&v["units"]),
        needs_directional_spring: v["needs_directional_spring"].as_bool().unwrap(),
        refuse: v["refuse"].as_bool().unwrap(),
        k4src_sha256: text(&v["k4src_sha256"]),
        model: (!v["model"].is_null()).then(|| parse_model(&v["model"])),
        model_sha256: v["model_sha256"].as_str().map(str::to_string),
        scales: strings(&v["scales"]),
        rows,
        controls,
        not_covered: v["not_covered"]
            .as_array()
            .unwrap()
            .iter()
            .map(text)
            .collect(),
        s_full: v.get("s_full").map(strings),
    };
    if let Some(facts) = facts {
        facts.observe(&v, &case);
    }
    case
}

pub fn load_file(path: &Path) -> Vec<Case> {
    std::fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("{}: {e}", path.display()))
        .lines()
        .filter(|l| !l.is_empty())
        .map(parse_case)
        .collect()
}

pub fn load_family(family: &str) -> Vec<Case> {
    let (_, file) = FAMILY_FILES
        .iter()
        .find(|(f, _)| *f == family)
        .unwrap_or_else(|| panic!("{family}"));
    load_file(&cases_dir().join(file))
}

pub fn load_all() -> Vec<Case> {
    FAMILY_FILES
        .iter()
        .flat_map(|(_, f)| load_file(&cases_dir().join(f)))
        .collect()
}

/// Described loader: original Case iterator/collect, separate inline history.
pub fn load_file_described(path: &Path) -> (Vec<Case>, crate::envelope::FamilyInputFacts) {
    let text = std::fs::read_to_string(path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    let mut facts =
        crate::envelope::FamilyInputFacts::begin(&text, path.file_name().unwrap().len());
    let cases = text
        .lines()
        .filter(|l| !l.is_empty())
        .map(|line| parse_case_capture(line, Some(&mut facts)))
        .collect();
    (cases, facts)
}

pub fn load_family_described(family: &str) -> (Vec<Case>, crate::envelope::FamilyInputFacts) {
    let (_, file) = FAMILY_FILES
        .iter()
        .find(|(f, _)| *f == family)
        .unwrap_or_else(|| panic!("{family}"));
    load_file_described(&cases_dir().join(file))
}

pub fn load_all_described() -> (Vec<Case>, [crate::envelope::FamilyInputFacts; 10]) {
    let mut facts = [crate::envelope::FamilyInputFacts::default(); 10];
    let cases = FAMILY_FILES
        .iter()
        .enumerate()
        .flat_map(|(i, (_, file))| {
            let (cases, description) = load_file_described(&cases_dir().join(file));
            facts[i] = description;
            cases
        })
        .collect();
    (cases, facts)
}

/// The committed expected-unresolved list (`cases/expected_unresolved.json`;
/// ROOT's ruling on I17's A1 stop): the cases W1a is expected to leave
/// honestly unresolved, with no rows.
pub fn expected_unresolved() -> &'static [String] {
    static LIST: std::sync::OnceLock<Vec<String>> = std::sync::OnceLock::new();
    LIST.get_or_init(|| {
        let text = std::fs::read_to_string(cases_dir().join("expected_unresolved.json"))
            .expect("expected_unresolved.json");
        let v: Value = serde_json::from_str(&text).expect("JSON");
        v["entries"]
            .as_array()
            .expect("entries")
            .iter()
            .map(|e| text_of(&e["case"]))
            .collect()
    })
}

fn text_of(v: &Value) -> String {
    text(v)
}

/// A large model written by `gen_vk_cases.py --large <dir>` (examples only).
pub fn load_large_model(path: &Path) -> (Model, String) {
    let text = std::fs::read_to_string(path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    let v: Value = serde_json::from_str(&text).expect("model JSON");
    (parse_model(&v), crate::sha256::sha256_hex(text.as_bytes()))
}

/// Same read/parse/model/hash evaluation and drops, with inline source facts.
pub fn load_large_model_described(
    path: &Path,
) -> (Model, String, crate::envelope::ExternalModelFacts) {
    let text = std::fs::read_to_string(path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    let v: Value = serde_json::from_str(&text).expect("model JSON");
    let model = parse_model(&v);
    let facts = crate::envelope::ExternalModelFacts::capture(&text, &v, &model);
    (model, crate::sha256::sha256_hex(text.as_bytes()), facts)
}

/// The kernel quantity an R1 key is compared with (plan §4.2).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Target {
    Displacement(Dof),
    Reaction(Dof),
    Spring {
        id: u32,
        component: Component,
    },
    DirectionalSpring {
        id: u32,
        component: Component,
    },
    /// A component a global-axis spring (or a spring of the other kind) cannot
    /// have: no published row; R1's value must be exactly 0.
    StructuralZero,
    Axial(u32),
    Torque(u32),
    BendingEnd {
        member: u32,
        j_end: bool,
    },
    BendingStation(u32),
    Twist(u32),
    Extension(u32),
}

fn axis_offset(letter: &str) -> Option<usize> {
    match letter {
        "X" => Some(0),
        "Y" => Some(1),
        "Z" => Some(2),
        _ => None,
    }
}

impl Model {
    pub fn node_index(&self, name: &str) -> Option<u32> {
        self.node_names
            .iter()
            .position(|n| n == name)
            .map(|k| k as u32)
    }

    pub fn member(&self, name: &str) -> Option<&Member> {
        self.members.iter().find(|m| m.name == name)
    }

    pub fn resolve(&self, key: &str) -> Result<Target, String> {
        let bad = || format!("unresolved R1 key {key}");
        let parts: Vec<&str> = key.split('.').collect();
        let node = |name: &str| self.node_index(name).ok_or_else(bad);
        let member = |name: &str| self.member(name).map(|m| m.id).ok_or_else(bad);
        let dof = |n: u32, c: usize| Dof {
            node: n,
            component: Component::from_index(c),
        };
        match parts.as_slice() {
            ["u", n, c] => {
                let a = c.strip_prefix('U').and_then(axis_offset).ok_or_else(bad)?;
                Ok(Target::Displacement(dof(node(n)?, a)))
            }
            ["th", n, c] => {
                let a = c.strip_prefix('R').and_then(axis_offset).ok_or_else(bad)?;
                Ok(Target::Displacement(dof(node(n)?, 3 + a)))
            }
            ["R", n, d] => {
                let (kind, axis) = d.split_at(1);
                let off = match kind {
                    "U" => 0,
                    "R" => 3,
                    _ => return Err(bad()),
                };
                Ok(Target::Reaction(dof(
                    node(n)?,
                    off + axis_offset(axis).ok_or_else(bad)?,
                )))
            }
            ["S", n, i, c] => {
                let spec_key = format!("S.{n}.{i}");
                let s = self
                    .springs
                    .iter()
                    .find(|s| s.key == spec_key)
                    .ok_or_else(bad)?;
                let (kind, axis) = c.split_at(1);
                let force = match kind {
                    "F" => true,
                    "M" => false,
                    _ => return Err(bad()),
                };
                let component = if force { 0 } else { 3 } + axis_offset(axis).ok_or_else(bad)?;
                if force != s.translation {
                    return Ok(Target::StructuralZero);
                }
                match s.axis {
                    Some(a) if a == component => Ok(Target::Spring {
                        id: s.id,
                        component: Component::from_index(component),
                    }),
                    Some(_) => Ok(Target::StructuralZero),
                    None => Ok(Target::DirectionalSpring {
                        id: s.id,
                        component: Component::from_index(component),
                    }),
                }
            }
            ["N", m] => Ok(Target::Axial(member(m)?)),
            ["T", m] => Ok(Target::Torque(member(m)?)),
            ["Mb", m, "i"] => Ok(Target::BendingEnd {
                member: member(m)?,
                j_end: false,
            }),
            ["Mb", m, "j"] => Ok(Target::BendingEnd {
                member: member(m)?,
                j_end: true,
            }),
            ["Mb", m, "mid"] => {
                let id = member(m)?;
                let station = self
                    .stations
                    .iter()
                    .find(|s| s.1 == id && s.2 == 0.5)
                    .ok_or_else(bad)?;
                Ok(Target::BendingStation(station.0))
            }
            ["tw", m] => Ok(Target::Twist(member(m)?)),
            ["ext", m] => Ok(Target::Extension(member(m)?)),
            _ => Err(bad()),
        }
    }

    /// K4's `SourceParts` for this model (the adapter's Rust half).
    pub fn source_parts(&self) -> SourceParts {
        let dof = |node: u32, c: usize| Dof {
            node,
            component: Component::from_index(c),
        };
        let mut parts = SourceParts {
            nodes: self.nodes.clone(),
            ..Default::default()
        };
        for m in &self.members {
            parts.members.push(StraightMember {
                id: m.id,
                node_i: m.node_i,
                node_j: m.node_j,
                elastic_modulus: m.elastic_modulus,
                shear_modulus: m.shear_modulus,
                area: m.area,
                second_moment_y: m.second_moment_y,
                second_moment_z: m.second_moment_z,
                torsion_constant: m.torsion_constant,
                y_reference: m.y_reference,
            });
        }
        for s in &self.springs {
            match s.axis {
                Some(a) => parts.springs.push(Spring {
                    id: s.id,
                    dof: dof(s.node, a),
                    stiffness: s.stiffness,
                }),
                None => parts.directional_springs.push(DirectionalSpring {
                    id: s.id,
                    node: s.node,
                    kind: if s.translation {
                        SpringKind::Translation
                    } else {
                        SpringKind::Rotation
                    },
                    direction: s.direction,
                    stiffness: s.stiffness,
                }),
            }
        }
        for &(n, c) in &self.constraints {
            parts.constraints.push(Constraint {
                dof: dof(n, c),
                value: 0.0,
            });
        }
        for (n, c, v, src) in &self.loads {
            parts.loads.push(NodalLoad {
                dof: dof(*n, *c),
                value: *v,
                source_id: src.clone(),
            });
        }
        for &(id, member, fraction) in &self.stations {
            parts.stations.push(Station {
                id,
                member,
                fraction,
            });
        }
        parts
    }

    /// The connected components of the member graph, as a body per node.
    pub fn bodies(&self) -> Vec<u32> {
        let n = self.nodes.len();
        let mut parent: Vec<usize> = (0..n).collect();
        fn root(p: &mut [usize], mut x: usize) -> usize {
            while p[x] != x {
                p[x] = p[p[x]];
                x = p[x];
            }
            x
        }
        for m in &self.members {
            let (a, b) = (
                root(&mut parent, m.node_i as usize),
                root(&mut parent, m.node_j as usize),
            );
            if a != b {
                parent[a.max(b)] = a.min(b);
            }
        }
        let mut label = vec![u32::MAX; n];
        let mut next = 0;
        let mut out = vec![0; n];
        for k in 0..n {
            let r = root(&mut parent, k);
            if label[r] == u32::MAX {
                label[r] = next;
                next += 1;
            }
            out[k] = label[r];
        }
        out
    }
}
