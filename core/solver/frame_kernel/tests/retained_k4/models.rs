//! Test-only: the generator's model blocks (`models.txt`, `formation.txt`),
//! parsed into K4 sources, and the comparison of published rows with exact
//! expectations under the unchanged predicate |obs − exp| ≤ 1e-9·max(|exp|,
//! scale), the scale being the coupled body scale of the expected values.
use super::super::super::adaptive::{body_extent, coupled_scales, PublishedRow, RowClass};
use super::super::super::recover::{End, Kind, QuantityId};
use super::super::super::source::{
    Component, Constraint, DirectionalSpring, Dof, NodalLoad, PrimitiveSource, SourceParts, Spring,
    SpringKind, Station, StraightMember, SupportGroup,
};
use std::collections::BTreeMap;

pub(crate) const MODELS: &str = include_str!("models.txt");
pub(crate) const FORMATION: &str = include_str!("formation.txt");

fn hf(hex: &str) -> f64 {
    f64::from_bits(u64::from_str_radix(hex, 16).unwrap())
}

#[derive(Debug, Clone)]
pub(crate) struct Model {
    pub(crate) name: String,
    pub(crate) parts: SourceParts,
    pub(crate) expect: BTreeMap<String, f64>,
    /// R7 §7's SEED hook: (global DOF, value) added to the final state.
    pub(crate) seeds: Vec<(usize, f64)>,
}

impl Model {
    pub(crate) fn source(&self) -> PrimitiveSource {
        PrimitiveSource::new(self.parts.clone()).unwrap()
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Combo {
    pub(crate) name: String,
    pub(crate) operands: Vec<(f64, String)>,
    pub(crate) expect: BTreeMap<String, f64>,
}

fn dof(node: &str, c: &str) -> Dof {
    Dof {
        node: node.parse().unwrap(),
        component: Component::from_index(c.parse().unwrap()),
    }
}

/// Parses every `model … end` block of a text.
pub(crate) fn parse_models(text: &str) -> Vec<Model> {
    let mut out = Vec::new();
    let mut current: Option<Model> = None;
    for line in text.lines() {
        let f: Vec<&str> = line.split_whitespace().collect();
        if f.is_empty() {
            continue;
        }
        match f[0] {
            "model" => {
                current = Some(Model {
                    name: f[1].to_string(),
                    parts: SourceParts::default(),
                    expect: BTreeMap::new(),
                    seeds: Vec::new(),
                })
            }
            "end" => {
                if let Some(m) = current.take() {
                    out.push(m);
                }
            }
            _ => {
                let Some(m) = current.as_mut() else { continue };
                let p = &mut m.parts;
                match f[0] {
                    "node" => p.nodes.push([hf(f[1]), hf(f[2]), hf(f[3])]),
                    "member" => p.members.push(StraightMember {
                        id: f[1].parse().unwrap(),
                        node_i: f[2].parse().unwrap(),
                        node_j: f[3].parse().unwrap(),
                        elastic_modulus: hf(f[4]),
                        shear_modulus: hf(f[5]),
                        area: hf(f[6]),
                        second_moment_y: hf(f[7]),
                        second_moment_z: hf(f[8]),
                        torsion_constant: hf(f[9]),
                        y_reference: [hf(f[10]), hf(f[11]), hf(f[12])],
                    }),
                    "spring" => p.springs.push(Spring {
                        id: f[1].parse().unwrap(),
                        dof: dof(f[2], f[3]),
                        stiffness: hf(f[4]),
                    }),
                    "dspring" => p.directional_springs.push(DirectionalSpring {
                        id: f[1].parse().unwrap(),
                        node: f[2].parse().unwrap(),
                        kind: if f[3] == "t" {
                            SpringKind::Translation
                        } else {
                            SpringKind::Rotation
                        },
                        direction: [hf(f[4]), hf(f[5]), hf(f[6])],
                        stiffness: hf(f[7]),
                    }),
                    "constraint" => p.constraints.push(Constraint {
                        dof: dof(f[1], f[2]),
                        value: hf(f[3]),
                    }),
                    "load" => p.loads.push(NodalLoad {
                        dof: dof(f[1], f[2]),
                        value: hf(f[3]),
                        source_id: f[4].to_string(),
                    }),
                    "station" => p.stations.push(Station {
                        id: f[1].parse().unwrap(),
                        member: f[2].parse().unwrap(),
                        fraction: hf(f[3]),
                    }),
                    "support" => {
                        let ids = |s: &str| -> Vec<u32> {
                            if s == "-" {
                                Vec::new()
                            } else {
                                s.split(',').map(|x| x.parse().unwrap()).collect()
                            }
                        };
                        let mut restrained = [false; 6];
                        for (c, ch) in f[3].chars().enumerate() {
                            restrained[c] = ch == '1';
                        }
                        p.supports.push(SupportGroup {
                            id: f[1].parse().unwrap(),
                            node: f[2].parse().unwrap(),
                            restrained,
                            springs: ids(f[4]),
                            directional_springs: ids(f[5]),
                        });
                    }
                    "expect" => {
                        m.expect.insert(f[1].to_string(), hf(f[2]));
                    }
                    "seed" => m.seeds.push((f[1].parse().unwrap(), hf(f[2]))),
                    _ => {}
                }
            }
        }
    }
    out
}

pub(crate) fn models() -> Vec<Model> {
    parse_models(MODELS)
}

pub(crate) fn model(name: &str) -> Model {
    models()
        .into_iter()
        .find(|m| m.name == name)
        .unwrap_or_else(|| panic!("{name}"))
}

pub(crate) fn combos() -> Vec<Combo> {
    parse_combos(MODELS)
}

/// Parses every `combo … end` block of a text.
pub(crate) fn parse_combos(text: &str) -> Vec<Combo> {
    let mut out = Vec::new();
    let mut current: Option<Combo> = None;
    for line in text.lines() {
        let f: Vec<&str> = line.split_whitespace().collect();
        match f.first().copied() {
            Some("combo") => {
                current = Some(Combo {
                    name: f[1].to_string(),
                    operands: f[2..]
                        .iter()
                        .map(|o| {
                            let (c, n) = o.split_once(':').unwrap();
                            (hf(c), n.to_string())
                        })
                        .collect(),
                    expect: BTreeMap::new(),
                })
            }
            Some("expect") => {
                if let Some(c) = current.as_mut() {
                    c.expect.insert(f[1].to_string(), self::hf(f[2]));
                }
            }
            Some("end") => {
                if let Some(c) = current.take() {
                    out.push(c);
                }
            }
            Some("model") => current = None,
            _ => {}
        }
    }
    out
}

/// NP-A's represented (stored binary64) root and tip rotations of N05.
pub(crate) fn np_a_n05() -> (f64, f64) {
    let line = MODELS
        .lines()
        .find(|l| l.starts_with("npa N05 root"))
        .unwrap();
    let f: Vec<&str> = line.split_whitespace().collect();
    (self::hf(f[3]), self::hf(f[5]))
}

/// The expectation key of a published row.
pub(crate) fn key(id: &QuantityId) -> String {
    match *id {
        QuantityId::Displacement(d) => format!("u.{}.{}", d.node, d.component.index()),
        QuantityId::DisplacementMagnitude(n) => format!("mag.{n}"),
        QuantityId::EndAction {
            member,
            end,
            component,
        } => format!(
            "end.{member}.{}.{}",
            if end == End::I { "i" } else { "j" },
            component.index()
        ),
        QuantityId::StationAction { station, component } => {
            format!("st.{station}.{}", component.index())
        }
        QuantityId::SpringAction { spring, component } => {
            format!("spr.{spring}.{}", component.index())
        }
        QuantityId::DirectionalSpringAction { spring, component } => {
            format!("dspr.{spring}.{}", component.index())
        }
        QuantityId::Reaction(d) => format!("R.{}.{}", d.node, d.component.index()),
        QuantityId::SupportForceMagnitude(g) => format!("sf.{g}"),
        QuantityId::SupportMomentMagnitude(g) => format!("sm.{g}"),
    }
}

/// sqrt(a² + b²) written out (not hypot), with an exact power-of-two scaling
/// when the squares would overflow or underflow.
fn magnitude(a: f64, b: f64) -> f64 {
    let m = a.abs().max(b.abs());
    let (down, up) = if m > f64::from_bits(0x5F30_0000_0000_0000) {
        (
            f64::from_bits(0x1A70_0000_0000_0000),
            f64::from_bits(0x6570_0000_0000_0000),
        ) // 2^-600, 2^600
    } else if m < f64::from_bits(0x20B0_0000_0000_0000) && m > 0.0 {
        (
            f64::from_bits(0x6570_0000_0000_0000),
            f64::from_bits(0x1A70_0000_0000_0000),
        )
    } else {
        (1.0, 1.0)
    };
    let (x, y) = (a * down, b * down);
    (x * x + y * y).sqrt() * up
}

/// The published values by key, plus the convention-free member quantities
/// (N, T and the bending magnitudes sqrt(My² + Mz²)).
pub(crate) fn published(rows: &[PublishedRow]) -> BTreeMap<String, (f64, Kind, u32)> {
    let mut out = BTreeMap::new();
    for r in rows {
        if let Some(v) = r.value.value() {
            out.insert(key(&r.id), (v, r.kind, r.body));
        }
    }
    // Derived quantities only where every operand was published (an
    // underflowing or overflowing operand leaves them out).
    let get = |k: &str| out.get(k).map(|x: &(f64, Kind, u32)| (x.0, x.2));
    let mut extra = Vec::new();
    for k in out.keys() {
        if let Some(rest) = k.strip_prefix("end.") {
            let parts: Vec<&str> = rest.split('.').collect();
            let (m, end, c) = (parts[0], parts[1], parts[2]);
            if end == "j" && c == "0" {
                let (v, b) = get(k).unwrap();
                extra.push((format!("N.{m}"), (v, Kind::Force, b)));
                if let Some((t, _)) = get(&format!("end.{m}.j.3")) {
                    extra.push((format!("T.{m}"), (t, Kind::Moment, b)));
                }
            }
            if c == "4" {
                let (my, b) = get(k).unwrap();
                if let Some((mz, _)) = get(&format!("end.{m}.{end}.5")) {
                    extra.push((
                        format!("Mb.{m}.{end}"),
                        (magnitude(my, mz), Kind::Moment, b),
                    ));
                }
            }
        }
        if let Some(rest) = k.strip_prefix("st.") {
            let parts: Vec<&str> = rest.split('.').collect();
            if parts[1] == "4" {
                let (my, b) = get(k).unwrap();
                if let Some((mz, _)) = get(&format!("st.{}.5", parts[0])) {
                    extra.push((
                        format!("Mbs.{}", parts[0]),
                        (magnitude(my, mz), Kind::Moment, b),
                    ));
                }
            }
        }
    }
    out.extend(extra);
    out
}

/// Worst |obs − exp| / max(|exp|, scale) over the expected keys, where the
/// scale is the coupled body scale of the expected values (D1 §4.1.6.1 items
/// 4–6 applied to them); constrained displacements carry no scale.
pub(crate) fn compare(
    source: &PrimitiveSource,
    rows: &[PublishedRow],
    expect: &BTreeMap<String, f64>,
) -> (f64, String, usize) {
    compare_with(source, rows, expect, &BTreeMap::new())
}

/// `compare`, where a row that the publication withholds as
/// `absolute_verified` with bound b may instead meet its own claim,
/// |obs − exp| ≤ b·(1 + 2^-21) (the published value's rounding is below
/// 2^-23·b; D1 revision 5a.3 selects states whose tiny rows are intervals).
/// Derived quantities take their operands' bounds: N and T their end row's,
/// a bending magnitude the sum of its two components' (both absolute).
pub(crate) fn compare_honest(
    source: &PrimitiveSource,
    rows: &[PublishedRow],
    expect: &BTreeMap<String, f64>,
) -> (f64, String, usize) {
    let mut bounds: BTreeMap<String, f64> = BTreeMap::new();
    for r in rows {
        if let RowClass::AbsoluteVerified { bound_bits } = r.class {
            bounds.insert(key(&r.id), f64::from_bits(bound_bits));
        }
    }
    let mut derived = Vec::new();
    for k in bounds.keys() {
        let parts: Vec<&str> = k.split('.').collect();
        match (parts[0], parts.len()) {
            ("end", 4) if parts[2] == "j" && parts[3] == "0" => {
                derived.push((format!("N.{}", parts[1]), bounds[k]));
            }
            ("end", 4) if parts[2] == "j" && parts[3] == "3" => {
                derived.push((format!("T.{}", parts[1]), bounds[k]));
            }
            ("end", 4) if parts[3] == "4" => {
                if let Some(z) = bounds.get(&format!("end.{}.{}.5", parts[1], parts[2])) {
                    derived.push((format!("Mb.{}.{}", parts[1], parts[2]), bounds[k] + z));
                }
            }
            ("st", 3) if parts[2] == "4" => {
                if let Some(z) = bounds.get(&format!("st.{}.5", parts[1])) {
                    derived.push((format!("Mbs.{}", parts[1]), bounds[k] + z));
                }
            }
            _ => {}
        }
    }
    bounds.extend(derived);
    compare_with(source, rows, expect, &bounds)
}

fn compare_with(
    source: &PrimitiveSource,
    rows: &[PublishedRow],
    expect: &BTreeMap<String, f64>,
    bounds: &BTreeMap<String, f64>,
) -> (f64, String, usize) {
    let obs = published(rows);
    let mut s = vec![[0.0f64; 4]; source.body_count() as usize];
    for (k, &e) in expect {
        if let Some(&(_, kind, body)) = obs.get(k) {
            if k.starts_with("u.") && is_constrained(source, k) {
                continue;
            }
            let slot = &mut s[body as usize][kind.index()];
            *slot = slot.max(e.abs());
        }
    }
    let scales: Vec<[f64; 4]> = (0..source.body_count())
        .map(|b| {
            let coords: Vec<[f64; 3]> = source
                .body_nodes(b)
                .iter()
                .map(|&n| source.nodes()[n as usize])
                .collect();
            coupled_scales(s[b as usize], body_extent(&coords))
        })
        .collect();
    let mut worst = (0.0f64, String::new());
    let mut compared = 0;
    for (k, &e) in expect {
        let Some(&(o, kind, body)) = obs.get(k) else {
            continue;
        };
        compared += 1;
        let scale = e.abs().max(scales[body as usize][kind.index()]);
        let mut ratio = if scale == 0.0 {
            if o == e {
                0.0
            } else {
                f64::INFINITY
            }
        } else {
            (o - e).abs() / scale
        };
        if let Some(&b) = bounds.get(k) {
            // In compare's units (1e-9 of the scale is 1).
            let claim = b * (1.0 + 2f64.powi(-21));
            let r = if (o - e).abs() <= claim {
                0.0
            } else {
                f64::INFINITY
            };
            ratio = ratio.min(r);
        }
        if ratio > worst.0 {
            worst = (ratio, k.clone());
        }
    }
    (worst.0 / 1e-9, worst.1, compared)
}

fn is_constrained(source: &PrimitiveSource, key: &str) -> bool {
    let parts: Vec<&str> = key.split('.').collect();
    let g = parts[1].parse::<usize>().unwrap() * 6 + parts[2].parse::<usize>().unwrap();
    source.constraint(g).is_some()
}
