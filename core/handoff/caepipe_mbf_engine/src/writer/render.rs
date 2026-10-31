//! The writer's geometry and text: bends at their tangent intersections, the layout, the MBF records, the maps, the
//! preflight re-import and the assumptions (si_writer `_Writer.resolve_bends` through `add_assumptions`).

use super::*;
use crate::pymath::{acos, degrees, hypot, py_max, py_min, tan};
use crate::value::py_eq;
use crate::{ImportOptions, SidecarInput};
use sha2::{Digest, Sha256};
use std::collections::HashSet;

fn sub(a: &V3, b: &V3) -> V3 {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn dot(a: &V3, b: &V3) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn cross(a: &V3, b: &V3) -> V3 {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}

fn norm(a: &V3) -> f64 {
    hypot(a)
}

fn unit(a: &V3) -> V3 {
    let length = norm(a);
    [a[0] / length, a[1] / length, a[2] / length]
}

/// A layout row's node: a model node, or ("TI", bend pipe id) for a bend row at its tangent intersection.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) enum Key {
    Node(String),
    Ti(String),
}

pub(crate) struct Row {
    pub kind: char,
    pub node: Key,
    pub end: String,
    pub element: Option<String>,
    pub new: bool,
    /// (bend pipe id, near tangent point, far tangent point) of a bend row.
    pub bend: Option<(String, String, String)>,
}

struct Attributes {
    materials: OMap<(String, String), String>,
    sections: OMap<(String, String), String>,
    loads: OMap<(String, String), String>,
    material_map: Dict,
    section_map: Dict,
    load_entries: OMap<String, (f64, f64)>,
    temperatures: Vec<f64>,
}

fn push_element(map: &mut Dict, name: &str, pipe_id: &str) {
    if let Some(Value::List(items)) = map.get_mut(&name.to_string()).and_then(|entry| entry.as_dict_mut()).and_then(|entry| entry.get_mut(&"elements".to_string())) {
        items.push(Value::str(pipe_id));
    }
}

fn float_field(entry: &Value, key: &str) -> f64 {
    entry.get_or_null(key).as_f64().expect("a float field")
}

impl<'m> Writer<'m> {
    /// Place each bend at the tangent intersection (TI) of its adjacent straights, or refuse it by name.
    pub fn resolve_bends(&mut self) -> R<()> {
        let mut incident: OMap<String, Vec<String>> = OMap::new();
        for (pipe_id, pipe) in self.pipes.iter() {
            incident.entry_or_insert_with(pipe.from.clone(), Vec::new).push(pipe_id.clone());
            incident.entry_or_insert_with(pipe.to.clone(), Vec::new).push(pipe_id.clone());
        }
        let loaded: Vec<Value> = self.load_records.values().map(|record| record.get(&"target".to_string()).expect("target").get_or_null("node").clone()).collect();
        let mut ids: Vec<String> = self.bends.keys().cloned().collect();
        ids.sort();
        for pipe_id in ids {
            let entity = self.bends.get(&pipe_id).expect("bend").entity.clone();
            let (from, to) = {
                let pipe = self.pipes.get(&pipe_id).expect("pipe");
                (pipe.from.clone(), pipe.to.clone())
            };
            let mut straights: OMap<String, String> = OMap::new();
            for tangent in [from, to] {
                let others: Vec<String> =
                    incident.get(&tangent).cloned().unwrap_or_default().into_iter().filter(|other| *other != pipe_id).collect();
                let quoted = crate::pystr::py_repr_str(&tangent);
                if self.node_restraints.contains_key(&tangent) || loaded.iter().any(|node| node.as_str() == Some(tangent.as_str())) {
                    let message = format!(
                        "{entity} tangent point {quoted} carries a support or load; the target program reports bend \
                         tangent points as generated nodes, so the writer does not place one there"
                    );
                    self.refuse("MBF-SI-BEND-UNSUPPORTED", &entity, &message);
                } else if others.is_empty() {
                    let message =
                        format!("{entity} ends a run at tangent point {quoted}; a bend needs a straight pipe on both sides");
                    self.refuse("MBF-SI-BEND-UNSUPPORTED", &entity, &message);
                } else if others.len() > 1 {
                    let message = format!(
                        "{entity} tangent point {quoted} joins {} pipes; a branch at a bend tangent point is not exported",
                        others.len() + 1
                    );
                    self.refuse("MBF-SI-BEND-UNSUPPORTED", &entity, &message);
                } else if self.bends.contains_key(&others[0]) {
                    let message = format!(
                        "{entity} meets another bend at {quoted}; consecutive bends without a straight pipe between \
                         them are not exported"
                    );
                    self.refuse("MBF-SI-BEND-UNSUPPORTED", &entity, &message);
                } else {
                    straights.insert(tangent, others[0].clone());
                }
            }
            if straights.len() != 2 {
                continue;
            }
            let Some(ti) = self.tangent_intersection(&pipe_id, &straights)? else { continue };
            self.bends.get_mut(&pipe_id).expect("bend").ti = Some(ti);
            for tangent in straights.keys() {
                self.tangent_points.insert(tangent.clone(), pipe_id.clone());
            }
        }
        Ok(())
    }

    fn tangent_intersection(&mut self, pipe_id: &str, straights: &OMap<String, String>) -> R<Option<V3>> {
        let (entity, radius, angle) = {
            let bend = self.bends.get(&pipe_id.to_string()).expect("bend");
            (bend.entity.clone(), bend.radius, bend.angle)
        };
        let (near, far) = {
            let pipe = self.pipes.get(&pipe_id.to_string()).expect("pipe");
            (pipe.from.clone(), pipe.to.clone())
        };
        let far_end = |tangent: &str| -> String {
            let straight = self.pipes.get(straights.get(&tangent.to_string()).expect("straight")).expect("pipe");
            if straight.from == tangent {
                straight.to.clone()
            } else {
                straight.from.clone()
            }
        };
        let position = |node: &str| *self.nodes.get(&node.to_string()).expect("node");
        let (p, q) = (position(&near), position(&far));
        let incoming = sub(&p, &position(&far_end(&near)));
        let outgoing = sub(&position(&far_end(&far)), &q);
        let chord = sub(&q, &p);
        let smallest = py_min(py_min(norm(&incoming), norm(&outgoing)), norm(&chord));
        if smallest == 0.0 {
            self.refuse("MBF-SI-GEOMETRY-UNSUPPORTED", &entity, &format!("{entity}: a bend or adjacent straight has zero length"));
            return Ok(None);
        }
        let (d1, d2) = (unit(&incoming), unit(&outgoing));
        let cosine = py_max(-1.0, py_min(1.0, dot(&d1, &d2)));
        let det = 1.0 - cosine * cosine;
        let (near_q, far_q) = (crate::pystr::py_repr_str(&near), crate::pystr::py_repr_str(&far));
        if det <= BEND_RELATIVE_TOLERANCE {
            let message =
                format!("{entity}: the straights at {near_q} and {far_q} are parallel; there is no tangent intersection");
            self.refuse("MBF-SI-BEND-GEOMETRY-INCONSISTENT", &entity, &message);
            return Ok(None);
        }
        let (b1, b2) = (dot(&d1, &chord), dot(&d2, &chord));
        let t1 = (b1 - cosine * b2) / det;
        let t2 = (b2 - cosine * b1) / det;
        let ti = [p[0] + t1 * d1[0], p[1] + t1 * d1[1], p[2] + t1 * d1[2]];
        let residual = norm(&sub(&sub(&q, &[t2 * d2[0], t2 * d2[1], t2 * d2[2]]), &ti));
        let chord_length = norm(&chord);
        let theta = acos(cosine).map_err(py)?;
        let tangent_length = radius * tan(theta / 2.0).map_err(py)?;
        if !self.finite(&[ti[0], ti[1], ti[2], t1, t2, residual, tangent_length], &entity, "tangent intersection") {
            return Ok(None);
        }
        let tol = BEND_RELATIVE_TOLERANCE;
        if residual > tol * chord_length {
            let message = format!(
                "{entity}: the straights at {near_q} and {far_q} do not intersect (gap {} mm)",
                num(residual)
            );
            self.refuse("MBF-SI-BEND-GEOMETRY-INCONSISTENT", &entity, &message);
            return Ok(None);
        }
        if !((t1 - tangent_length).abs() <= tol * tangent_length && (t2 - tangent_length).abs() <= tol * tangent_length) {
            let message = format!(
                "{entity}: tangent lengths {} mm and {} mm disagree with R tan(theta/2) = {} mm (R = {} mm, theta = {} \
                 deg)",
                num(t1),
                num(t2),
                num(tangent_length),
                num(radius),
                num(degrees(theta))
            );
            self.refuse("MBF-SI-BEND-GEOMETRY-INCONSISTENT", &entity, &message);
            return Ok(None);
        }
        if let Some(angle) = angle {
            if (angle - theta).abs() > tol * py_max(theta, 1.0) {
                let message = format!(
                    "{entity}: bend_angle {} rad disagrees with the {} rad between the adjacent straights",
                    num(angle),
                    num(theta)
                );
                self.refuse("MBF-SI-BEND-GEOMETRY-INCONSISTENT", &entity, &message);
                return Ok(None);
            }
        }
        let y_ref = self.pipes.get(&pipe_id.to_string()).expect("pipe").y_reference.clone();
        let y_raw = [float_of(y_ref.get_or_null("x"))?, float_of(y_ref.get_or_null("y"))?, float_of(y_ref.get_or_null("z"))?];
        let chord_unit = unit(&chord);
        let along = dot(&y_raw, &chord_unit);
        let side = sub(&y_raw, &[along * chord_unit[0], along * chord_unit[1], along * chord_unit[2]]);
        let middle = [(p[0] + q[0]) / 2.0, (p[1] + q[1]) / 2.0, (p[2] + q[2]) / 2.0];
        let toward_ti = sub(&ti, &middle);
        let aligned = if norm(&side) == 0.0 || norm(&toward_ti) == 0.0 {
            false
        } else {
            let (side_unit, ti_unit) = (unit(&side), unit(&toward_ti));
            dot(&side_unit, &ti_unit) > 0.0 && norm(&cross(&side_unit, &ti_unit)) <= tol
        };
        if !aligned {
            let message = format!(
                "{entity}: pipe:{pipe_id} y_reference does not point from the chord toward the tangent intersection in \
                 the plane of the adjacent straights, so SWBPIPE's arc is not the corner bend the target program would \
                 form"
            );
            self.refuse("MBF-SI-BEND-GEOMETRY-INCONSISTENT", &entity, &message);
            return Ok(None);
        }
        self.bends.get_mut(&pipe_id.to_string()).expect("bend").theta = Some(theta);
        Ok(Some(ti))
    }

    // -- layout ---------------------------------------------------------
    pub fn layout(&mut self) -> Vec<Row> {
        let mut incident: OMap<String, Vec<(String, String)>> = OMap::new();
        let mut ids: Vec<String> = self.pipes.keys().cloned().collect();
        ids.sort();
        for pipe_id in &ids {
            let pipe = self.pipes.get(pipe_id).expect("pipe");
            incident.entry_or_insert_with(pipe.from.clone(), Vec::new).push((pipe_id.clone(), pipe.to.clone()));
            incident.entry_or_insert_with(pipe.to.clone(), Vec::new).push((pipe_id.clone(), pipe.from.clone()));
        }
        let mut unconnected: Vec<String> = self.nodes.keys().filter(|node| !incident.contains_key(*node)).cloned().collect();
        unconnected.sort();
        for node in unconnected {
            let entity = format!("node:{node}");
            let referenced = self.node_restraints.contains_key(&node)
                || self.load_records.values().any(|record| {
                    record.get(&"target".to_string()).map(|target| target.get_or_null("node").as_str() == Some(node.as_str())).unwrap_or(false)
                });
            if referenced {
                self.refuse("MBF-SI-TOPOLOGY-UNSUPPORTED", &entity, &format!("{entity} carries a support or load but no pipe segment"));
            } else {
                self.omit("MBF-SI-NODE-NOT-EXPORTED", &entity, &format!("{entity} is not connected to any pipe segment"));
            }
        }
        if incident.len() > MAX_NODES {
            let message = format!(
                "{} connected nodes exceed the {MAX_NODES} node numbers 10..{}",
                incident.len(),
                MAX_NODES * 10
            );
            self.refuse("MBF-SI-NODE-NUMBER-LIMIT", "model.nodes", &message);
        }
        if self.pipes.is_empty() {
            self.refuse("MBF-SI-TOPOLOGY-UNSUPPORTED", "model.pipe_segments", "the model has no exportable pipe segment");
        }
        let mut rows: Vec<Row> = Vec::new();
        let mut written: HashSet<String> = HashSet::new();
        let mut used: HashSet<String> = HashSet::new();
        loop {
            let mut remaining: Vec<String> = incident.keys().filter(|node| !written.contains(*node)).cloned().collect();
            remaining.sort();
            if remaining.is_empty() {
                break;
            }
            let anchors: Vec<&String> = remaining.iter().filter(|node| self.is_anchor(node)).collect();
            let plain: Vec<&String> = remaining.iter().filter(|node| !self.tangent_points.contains_key(*node)).collect();
            let root = anchors.first().or(plain.first()).copied().unwrap_or(&remaining[0]).clone();
            rows.push(Row { kind: 'F', node: Key::Node(root.clone()), end: root.clone(), element: None, new: true, bend: None });
            written.insert(root.clone());
            let mut stack = vec![root];
            while let Some(current) = stack.last().cloned() {
                let step = incident.get(&current).expect("incident").iter().find(|(pipe_id, _)| !used.contains(pipe_id)).cloned();
                let Some((pipe_id, other)) = step else {
                    stack.pop();
                    continue;
                };
                used.insert(pipe_id.clone());
                if written.contains(&other) {
                    let message = format!(
                        "pipe:{pipe_id} closes a loop at node {}; closed loops are not exported",
                        crate::pystr::py_repr_str(&other)
                    );
                    self.refuse("MBF-SI-TOPOLOGY-UNSUPPORTED", &format!("pipe:{pipe_id}"), &message);
                    continue;
                }
                if let Some(bend) = self.bends.get(&pipe_id) {
                    let entity = bend.entity.clone();
                    let message = format!(
                        "{entity} is reached from its tangent point {} without its incoming straight; the layout cannot \
                         place it",
                        crate::pystr::py_repr_str(&current)
                    );
                    self.refuse("MBF-SI-BEND-UNSUPPORTED", &entity, &message);
                    continue;
                }
                if rows.last().expect("row").end != current {
                    if self.tangent_points.contains_key(&current) {
                        let message =
                            format!("the layout would restart at bend tangent point {}", crate::pystr::py_repr_str(&current));
                        self.refuse("MBF-SI-BEND-UNSUPPORTED", &format!("node:{current}"), &message);
                    }
                    rows.push(Row { kind: 'F', node: Key::Node(current.clone()), end: current.clone(), element: None, new: false, bend: None });
                }
                let Some(bend_id) = self.tangent_points.get(&other).cloned() else {
                    rows.push(Row { kind: 'T', node: Key::Node(other.clone()), end: other.clone(), element: Some(pipe_id), new: true, bend: None });
                    written.insert(other.clone());
                    stack.push(other);
                    continue;
                };
                let far = {
                    let bend_pipe = self.pipes.get(&bend_id).expect("pipe");
                    if bend_pipe.from == other {
                        bend_pipe.to.clone()
                    } else {
                        bend_pipe.from.clone()
                    }
                };
                if written.contains(&far) {
                    let message = format!(
                        "pipe:{bend_id} closes a loop at node {}; closed loops are not exported",
                        crate::pystr::py_repr_str(&far)
                    );
                    self.refuse("MBF-SI-TOPOLOGY-UNSUPPORTED", &format!("pipe:{bend_id}"), &message);
                    continue;
                }
                used.insert(bend_id.clone());
                rows.push(Row {
                    kind: 'T',
                    node: Key::Ti(bend_id.clone()),
                    end: far.clone(),
                    element: Some(pipe_id),
                    new: true,
                    bend: Some((bend_id, other.clone(), far.clone())),
                });
                written.insert(other);
                written.insert(far.clone());
                stack.push(far);
            }
        }
        rows
    }

    fn position(&self, key: &Key) -> V3 {
        match key {
            Key::Ti(bend) => self.bends.get(bend).expect("bend").ti.expect("ti"),
            Key::Node(node) => *self.nodes.get(node).expect("node"),
        }
    }

    fn pipe_attributes(&mut self, pipe_id: &str, state: &mut Attributes) -> (String, String, String) {
        let (material_id, od, thk) = {
            let pipe = self.pipes.get(&pipe_id.to_string()).expect("pipe");
            (pipe.material.clone(), pipe.od, pipe.thk)
        };
        let mut density = 0.0;
        if let Some(&weight) = self.weight.get(&pipe_id.to_string()) {
            let (od_m, thk_m) = (od / 1000.0, thk / 1000.0);
            let area = std::f64::consts::PI / 4.0 * (od_m * od_m - (od_m - 2.0 * thk_m) * (od_m - 2.0 * thk_m));
            density = if area > 0.0 { -weight / (area * STANDARD_GRAVITY) } else { f64::INFINITY };
            if !density.is_finite() || (weight != 0.0 && density == 0.0) {
                let message = format!("pipe:{pipe_id} weight density is not representable in binary64");
                self.diag("MBF-SI-VALUE-OUT-OF-RANGE", "error", &format!("pipe:{pipe_id}"), &message);
            }
        }
        let material_key = (material_id.clone(), num(density));
        if !state.materials.contains_key(&material_key) {
            let name = format!("M{}", state.materials.len() + 1);
            state.materials.insert(material_key.clone(), name.clone());
            let material = self.materials.get(&material_id).expect("material");
            let mut entry = dict![
                ("swbpipe_material", Value::str(material_id.clone())),
                ("weight_density_kgf_per_m3", Value::Float(density)),
                ("poisson_ratio", Value::Float(material.nu)),
                ("elastic_modulus_n_per_mm2", Value::Float(material.e)),
                ("thermal_expansion_mm_per_mm_per_c", Value::Float(material.alpha)),
                ("temperatures_c", Value::List(state.temperatures.iter().map(|value| Value::Float(*value)).collect())),
                ("elements", Value::List(Vec::new())),
            ];
            entry.as_dict_mut().expect("dict").insert("origin".into(), Value::str(material.origin.unwrap_or("undeclared")));
            state.material_map.insert(name, entry);
        }
        let section_key = (num(od), num(thk));
        if !state.sections.contains_key(&section_key) {
            let name = format!("P{}", state.sections.len() + 1);
            state.sections.insert(section_key.clone(), name.clone());
            state.section_map.insert(
                name,
                dict![
                    ("outside_diameter_mm", Value::Float(od)),
                    ("wall_thickness_mm", Value::Float(thk)),
                    ("elements", Value::List(Vec::new())),
                ],
            );
        }
        let delta = self.delta_t.get(&pipe_id.to_string()).copied().unwrap_or(0.0);
        let pressure = self.pipe_pressure.get(&pipe_id.to_string()).copied().unwrap_or(0.0);
        let load_key = (num(delta), num(pressure));
        if !state.loads.contains_key(&load_key) {
            let name = format!("L{}", state.loads.len() + 1);
            state.loads.insert(load_key.clone(), name.clone());
            let temperature = self.t_ref + delta;
            state.load_entries.insert(name, (temperature, pressure));
            self.finite(&[temperature], &format!("pipe:{pipe_id}"), "operating temperature (reference + thermal change)");
        }
        let names = (
            state.materials.get(&material_key).expect("material").clone(),
            state.sections.get(&section_key).expect("section").clone(),
            state.loads.get(&load_key).expect("load").clone(),
        );
        push_element(&mut state.material_map, &names.0, pipe_id);
        push_element(&mut state.section_map, &names.1, pipe_id);
        names
    }

    // -- rendering ------------------------------------------------------
    pub fn render(mut self, rows: Vec<Row>) -> R<WriterResult> {
        let mut numbers: OMap<Key, i64> = OMap::new();
        let mut node_map: Dict = Dict::new();
        for row in &rows {
            if !numbers.contains_key(&row.node) {
                let number = 10 * (numbers.len() as i64 + 1);
                numbers.insert(row.node.clone(), number);
            }
            let number = *numbers.get(&row.node).expect("number");
            if let Some((_, near, far)) = &row.bend {
                node_map.insert(near.clone(), Value::str(format!("{number}A")));
                node_map.insert(far.clone(), Value::str(format!("{number}B")));
            } else if let Key::Node(node) = &row.node {
                if !node_map.contains_key(node) {
                    node_map.insert(node.clone(), Value::str(number.to_string()));
                }
            }
        }
        let node_name = |node_map: &Dict, node: &str| -> String {
            node_map.get(&node.to_string()).and_then(Value::as_str).expect("node name").to_string()
        };
        let deltas: Vec<f64> = self.pipes.keys().map(|pipe_id| self.delta_t.get(pipe_id).copied().unwrap_or(0.0)).collect();
        let abs: Vec<f64> = deltas.iter().map(|value| value.abs()).collect();
        let t_hi = self.t_ref + py_max(crate::pymath::py_max_all(&abs), 100.0);
        let t_lo = self.t_ref + py_min(0.0, py_min_all(&deltas));
        if !self.finite(&[t_lo, t_hi], "MATERIAL", "temperature table (reference temperature and thermal changes)") {
            return Ok(self.refused());
        }
        let mut temperatures: Vec<f64> = Vec::new();
        for value in [t_lo, self.t_ref, t_hi] {
            if !temperatures.contains(&value) {
                temperatures.push(value);
            }
        }
        temperatures.sort_by(|a, b| a.partial_cmp(b).expect("finite"));

        let mut state = Attributes {
            materials: OMap::new(),
            sections: OMap::new(),
            loads: OMap::new(),
            material_map: Dict::new(),
            section_map: Dict::new(),
            load_entries: OMap::new(),
            temperatures: temperatures.clone(),
        };
        let mut element_map: Dict = Dict::new();
        let mut attributes: OMap<String, (String, String, String)> = OMap::new();
        let mut order: i64 = 0;
        let mut start = rows[0].end.clone();
        for (layout_row, row) in rows.iter().enumerate() {
            if row.kind == 'F' {
                start = row.end.clone();
                continue;
            }
            let pipe_id = row.element.clone().expect("element");
            let names = self.pipe_attributes(&pipe_id, &mut state);
            attributes.insert(pipe_id.clone(), names.clone());
            let first_end = match (&row.bend, &row.node) {
                (Some((_, near, _)), _) => near.clone(),
                (None, Key::Node(node)) => node.clone(),
                (None, Key::Ti(_)) => unreachable!("a TI row is a bend row"),
            };
            let mut members = vec![(pipe_id.clone(), start.clone(), first_end)];
            if let Some((bend_id, near, far)) = &row.bend {
                let bend_names = self.pipe_attributes(bend_id, &mut state);
                if bend_names != names {
                    let entity = self.bends.get(bend_id).expect("bend").entity.clone();
                    let message = format!(
                        "{entity}: pipe:{bend_id} and its incoming straight pipe:{pipe_id} differ in material, weight, \
                         section, temperature or pressure; the target program writes them as one layout row with one \
                         set of properties"
                    );
                    self.refuse("MBF-SI-BEND-UNSUPPORTED", &entity, &message);
                }
                attributes.insert(bend_id.clone(), bend_names);
                members.push((bend_id.clone(), near.clone(), far.clone()));
            }
            for (member_id, member_start, member_end) in members {
                let (member_from, y_reference) = {
                    let member = self.pipes.get(&member_id).expect("pipe");
                    (member.from.clone(), member.y_reference.clone())
                };
                let (material, section, load) = attributes.get(&member_id).expect("attributes").clone();
                let mut entry = dict![
                    ("from", Value::str(node_name(&node_map, &member_start))),
                    ("to", Value::str(node_name(&node_map, &member_end))),
                    ("layout_order", Value::int(order)),
                    ("layout_row", Value::int(layout_row as i64)),
                    ("reversed", Value::Bool(member_start != member_from)),
                    ("material", Value::str(material)),
                    ("section", Value::str(section)),
                    ("load", Value::str(load)),
                ];
                if !self.bends.contains_key(&member_id) {
                    let derived = self.regenerating.as_ref().map(|set| set.contains(&member_id)).unwrap_or(false);
                    let given = is_vector(&y_reference)?;
                    let vector = if given {
                        let mut vector = Dict::new();
                        for axis in ["x", "y", "z"] {
                            vector.insert(axis.into(), Value::Float(float_of(y_reference.get_or_null(axis))?));
                        }
                        Value::Dict(vector)
                    } else {
                        Value::Null
                    };
                    let dict = entry.as_dict_mut().expect("dict");
                    dict.insert("y_reference".into(), vector);
                    dict.insert("y_reference_authored".into(), Value::Bool(given && !derived));
                }
                element_map.insert(member_id, entry);
                order += 1;
            }
            if let Some((bend_id, _, _)) = &row.bend {
                let bend = self.bends.get(bend_id).expect("bend");
                let ti = bend.ti.expect("ti");
                let mut record = dict![
                    ("component", bend.component.clone()),
                    ("radius_mm", Value::Float(bend.radius)),
                    ("angle_deg", Value::Float(degrees(bend.theta.expect("theta")))),
                    ("flexibility_factor", Value::Float(bend.k)),
                    ("tangent_intersection_mm", Value::List(ti.iter().map(|value| Value::Float(*value)).collect())),
                    ("caepipe_row_node", Value::str(numbers.get(&row.node).expect("number").to_string())),
                ];
                if let Some([in_plane, out_plane]) = bend.sifs {
                    let dict = record.as_dict_mut().expect("dict");
                    dict.insert("sif_in_plane".into(), Value::Float(in_plane));
                    dict.insert("sif_out_of_plane".into(), Value::Float(out_plane));
                }
                element_map.get_mut(bend_id).and_then(Value::as_dict_mut).expect("bend entry").insert("bend".into(), record);
            }
            start = row.end.clone();
        }
        if self.has_errors() {
            return Ok(self.refused());
        }

        let piping_code = self.piping_code.as_str().map(String::from);
        let mut lines = vec![
            title(&text(&self.title)?),
            "VERSION".into(),
            MBF_VERSION.into(),
            "OPTIONS".into(),
            match &piping_code {
                Some(code) => format!("HGRA,{code},SI"),
                None => "HGRA,SI".into(),
            },
        ];
        if piping_code.as_deref().map(|code| code.starts_with("B31")).unwrap_or(false) {
            lines.push("OPCODE".into());
            lines.push(OPCODE_LINE.into());
        }
        lines.push("OPTEMP".into());
        lines.push(num(self.t_ref));
        if !self.pipe_pressure.is_empty() {
            lines.push("OPPRES".into());
            lines.push(OPPRES_LINE.into());
        }
        lines.push("MATERIAL".into());
        for (name, record) in state.material_map.iter() {
            let described = sanitize(record.get_or_null("swbpipe_material").as_str().expect("str"), DESCRIPTION_MAX);
            let description = if described.is_empty() { name.clone() } else { described };
            lines.push(format!(
                "{name},{},{},1,1,CS,{description}",
                num(float_field(record, "weight_density_kgf_per_m3")),
                num(float_field(record, "poisson_ratio"))
            ));
            for temperature in &temperatures {
                lines.push(format!(
                    "{name},{},{},{}",
                    num(*temperature),
                    num(float_field(record, "elastic_modulus_n_per_mm2")),
                    num(float_field(record, "thermal_expansion_mm_per_mm_per_c"))
                ));
            }
        }
        lines.push("PIPE".into());
        for (name, record) in state.section_map.iter() {
            lines.push(format!(
                "{name},{},{},0,0,0,0,0,0",
                num(float_field(record, "outside_diameter_mm")),
                num(float_field(record, "wall_thickness_mm"))
            ));
        }
        lines.push("LOADS".into());
        for (name, (temperature, pressure)) in state.load_entries.iter() {
            lines.push(format!("{name},{},{},0", num(*temperature), num(*pressure)));
        }
        lines.push("LAYOUT".into());

        let mut carried: (Option<String>, Option<String>, Option<String>) = (None, None, None);
        let first_element = rows.iter().find(|row| row.kind == 'T').and_then(|row| row.element.clone()).expect("an element row");
        for (index, row) in rows.iter().enumerate() {
            let node = &row.node;
            let bend = row.bend.as_ref().map(|(bend_id, _, _)| self.bends.get(bend_id).expect("bend"));
            let mut fields = vec![format!("{}{}", row.kind, numbers.get(node).expect("number"))];
            let mut comments: Vec<String> = Vec::new();
            let (bend_k, bend_sifs, bend_radius) = match bend {
                Some(bend) => (Some(bend.k), bend.sifs, Some(bend.radius)),
                None => (None, None, None),
            };
            let bend = bend.is_some();
            if let Some(k) = bend_k {
                fields.push("KI".into());
                comments.push(format!("FF={}", num(k)));
                if let Some([a, b]) = bend_sifs {
                    comments.push(format!("BSIF({},{})", num(a), num(b)));
                }
            }
            let node_id = match node {
                Key::Node(node) => Some(node.as_str()),
                Key::Ti(_) => None,
            };
            if row.new && node_id.map(|node| self.is_anchor(node) || self.node_springs.contains_key(&node.to_string())).unwrap_or(false) {
                fields.push("KA".into());
            }
            let pipe_id = if row.kind == 'T' { row.element.clone() } else if index == 0 { Some(first_element.clone()) } else { None };
            if let Some(pipe_id) = &pipe_id {
                let wanted = attributes.get(pipe_id).expect("attributes").clone();
                if Some(&wanted.0) != carried.0.as_ref() {
                    fields.push(format!("M{}", wanted.0));
                }
                if Some(&wanted.1) != carried.1.as_ref() {
                    fields.push(format!("P{}", wanted.1));
                }
                if Some(&wanted.2) != carried.2.as_ref() {
                    comments.push(format!("L={}", wanted.2));
                }
                carried = (Some(wanted.0), Some(wanted.1), Some(wanted.2));
            }
            if let Some(radius) = bend_radius {
                fields.push(format!("B{}", num(radius)));
            }
            let [x, y, z] = self.position(node);
            if row.kind == 'F' {
                if index > 0 || [x, y, z].iter().any(|value| *value != 0.0) {
                    fields.extend([format!("X{}", num(x)), format!("Y{}", num(y)), format!("Z{}", num(z))]);
                }
            } else {
                let [px, py, pz] = self.position(&rows[index - 1].node);
                let offsets: Vec<(char, f64)> =
                    ['X', 'Y', 'Z'].into_iter().zip([x - px, y - py, z - pz]).filter(|(_, value)| *value != 0.0).collect();
                let pipe_id = pipe_id.clone().expect("element");
                let values: Vec<f64> = offsets.iter().map(|(_, value)| *value).collect();
                self.finite(&values, &format!("pipe:{pipe_id}"), "layout offset");
                if offsets.is_empty() {
                    self.refuse("MBF-SI-GEOMETRY-UNSUPPORTED", &format!("pipe:{pipe_id}"), &format!("pipe:{pipe_id} has zero length"));
                }
                fields.extend(offsets.iter().map(|(axis, value)| format!("{axis}{}", num(*value))));
            }
            if row.new && !bend {
                comments.extend(self.node_comments(node_id.expect("a model node"))?);
            }
            if let Some(first) = comments.first_mut() {
                *first = format!("C{first}");
            }
            fields.extend(comments);
            lines.push(fields.join(","));
        }
        if self.has_errors() {
            return Ok(self.refused());
        }
        let text: String = lines.iter().map(|line| format!("{line}\r\n")).collect();
        let derived = crate::documented_orientations(text.as_bytes())?;
        let keys: Vec<String> = element_map.keys().cloned().collect();
        for member_id in keys {
            let entry = element_map.get(&member_id).expect("entry");
            if !(entry.get("y_reference_authored").and_then(Value::as_bool) == Some(false) && entry.get_or_null("y_reference").is_null()) {
                continue;
            }
            let (from, to) = (
                entry.get_or_null("from").as_str().expect("str").to_string(),
                entry.get_or_null("to").as_str().expect("str").to_string(),
            );
            let vector = derived.as_ref().and_then(|derived| {
                derived.iter().rev().find(|((a, b), _)| *a == from && *b == to).map(|(_, vector)| *vector)
            });
            let Some(vector) = vector else {
                let message = format!(
                    "pipe:{member_id} has no authored y_reference, and the import does not read this MBF far enough to \
                     derive its documented local y"
                );
                self.refuse("MBF-SI-PREFLIGHT-REFUSED", &format!("pipe:{member_id}"), &message);
                continue;
            };
            let entry = element_map.get_mut(&member_id).and_then(Value::as_dict_mut).expect("entry");
            entry.insert(
                "y_reference".into(),
                dict![("x", Value::Float(vector[0])), ("y", Value::Float(vector[1])), ("z", Value::Float(vector[2]))],
            );
        }
        if self.has_errors() {
            return Ok(self.refused());
        }

        let layout_row_count = 1 + rows.len() as i64;
        if let Some(limit) = self.max_layout_rows {
            if layout_row_count > limit {
                let message = format!(
                    "{layout_row_count} layout rows (title row plus {} F/T rows) exceed {limit}; a row-limited \
                     (evaluation) build of the target program may refuse the file",
                    rows.len()
                );
                self.diag("MBF-SI-LAYOUT-ROW-LIMIT", "warning", "LAYOUT", &message);
            }
        }

        let mut support_map = Dict::new();
        for (support_id, support) in self.supports.iter() {
            let anchor = self.is_anchor(&support.node);
            let family = match &support.family {
                Some(family) => family.clone(),
                None => (if support.dofs.len() == DOFS.len() { "anchor" } else { "guide" }).to_string(),
            };
            let Some(node) = node_map.get(&support.node).cloned() else {
                return Err(py(key_error(&support.node)));
            };
            support_map.insert(
                support_id.clone(),
                dict![
                    ("node", node),
                    ("caepipe_kind", Value::str(if anchor || support.family.as_deref() == Some("spring") { "anchor" } else { "restraint" })),
                    ("restrained", Value::List(DOFS.iter().filter(|dof| support.dofs.contains(*dof)).map(|dof| Value::str(*dof)).collect())),
                    ("family", Value::str(family)),
                    ("family_authored", Value::Bool(support.family.is_some())),
                ],
            );
        }
        if !self.normalized.is_empty() {
            let listed: Vec<&str> = self.normalized.values().map(String::as_str).collect();
            let message = format!(
                "US-customary units are normalized to the SI profile by their exact definitions: {}",
                listed.join("; ")
            );
            self.diag("MBF-SI-UNITS-NORMALIZED", "info", "project.units", &message);
        }
        let mut load_map = Dict::new();
        for (key, record) in self.load_records.iter() {
            let mut entry = record.clone();
            let target = entry.get(&"target".to_string()).expect("target").clone();
            if let Some(node) = target.get("node") {
                let name = node_map.get(&node.as_str().expect("str").to_string()).cloned().expect("node name");
                entry.insert("caepipe_node".into(), name);
            } else if let Some(pipe) = target.get("pipe") {
                let element = element_map.get(&pipe.as_str().expect("str").to_string()).expect("element").clone();
                entry.insert("caepipe_element".into(), element.clone());
                let mbf = if entry.get(&"category".to_string()).and_then(Value::as_str) == Some("thermal") {
                    format!("LOADS {} Temperature 1", element.get_or_null("load").as_str().expect("str"))
                } else {
                    format!("MATERIAL {} weight density", element.get_or_null("material").as_str().expect("str"))
                };
                entry.insert("mbf".into(), Value::Str(mbf));
            } else {
                let support = target.get_or_null("support").as_str().expect("str");
                let node = &self.supports.get(&support.to_string()).expect("support").node;
                entry.insert("caepipe_node".into(), node_map.get(node).cloned().expect("node name"));
            }
            load_map.insert(key.clone(), Value::Dict(entry));
        }
        let mut pressure_map = Dict::new();
        match &self.contract {
            None => {
                pressure_map.insert("contract".into(), Value::str("none"));
                pressure_map.insert("shear_model".into(), Value::str("euler_bernoulli"));
                pressure_map.insert("regions".into(), Value::List(Vec::new()));
            }
            Some(contract) => {
                pressure_map.insert(
                    "contract".into(),
                    dict![("version", contract.get_or_null("version").clone()), ("mode", contract.get_or_null("mode").clone())],
                );
                pressure_map.insert("shear_model".into(), contract.get_or_null("shear_model").clone());
                let mut regions = Vec::new();
                for region in &self.regions {
                    let members = region.get_or_null("member_pipe_ids").clone();
                    let first = members.as_list().expect("members")[0].as_str().expect("str").to_string();
                    let terminals: Vec<Value> = region
                        .get_or_null("terminals")
                        .as_list()
                        .expect("terminals")
                        .iter()
                        .map(|terminal| {
                            dict![
                                ("node_ref", terminal.get_or_null("node_ref").clone()),
                                ("closure_transfer", terminal.get_or_null("closure_transfer").clone()),
                                ("provenance", terminal.get_or_null("provenance").clone()),
                            ]
                        })
                        .collect();
                    regions.push(dict![
                        ("id", region.get_or_null("id").clone()),
                        ("members", members),
                        ("terminals", Value::List(terminals)),
                        ("provenance", region.get_or_null("provenance").clone()),
                        ("pressure_bar", Value::Float(*self.pipe_pressure.get(&first).expect("pressure"))),
                    ]);
                }
                pressure_map.insert("regions".into(), Value::List(regions));
            }
        }
        self.add_assumptions(&temperatures);
        let mut load_case_map = Dict::new();
        for (key, value) in self.case_labels.iter() {
            load_case_map.insert(key.clone(), Value::str(value.clone()));
        }
        let mut result = WriterResult {
            status: "written".into(),
            text: Some(text),
            diagnostics: Vec::new(),
            node_map,
            element_map,
            support_map,
            load_case_map,
            material_map: state.material_map,
            section_map: state.section_map,
            load_map,
            pressure_map,
            layout_row_count,
            loss_report: Vec::new(),
            assumptions: Vec::new(),
        };
        match result.sidecar_checked(&"0".repeat(64)) {
            Ok(_) => {}
            Err(Ok(error)) => {
                let message = format!("the sidecar's {} {}; the sidecar has no self-digest", error.path, error.reason);
                self.invalid(&error.path, &message);
                return Ok(self.refused());
            }
            Err(Err(error)) => return Err(py(error)),
        }
        if self.regenerating.is_none() && !self.preflight_disabled && !self.survives_reimport(&result)? {
            return Ok(self.refused());
        }
        result.diagnostics = self.diagnostics;
        result.loss_report = self.loss;
        result.assumptions = self.assumptions;
        Ok(result)
    }

    /// Design §9.4, §12 and §14: every export as the printed MBF gives it back (the writer's own bound re-import).
    fn survives_reimport(&mut self, result: &WriterResult) -> R<bool> {
        let data = result.text.as_ref().expect("text").as_bytes();
        let sha = crate::sidecar::hex(&Sha256::digest(data));
        let sidecar = result.sidecar(&sha).map_err(py)?;
        let mut undeclared = Dict::new();
        if let Some(Value::Dict(materials)) = sidecar.get("material_map") {
            for (name, entry) in materials.iter() {
                if entry.get("origin").and_then(Value::as_str) == Some("undeclared") {
                    undeclared.insert(
                        name.clone(),
                        dict![("origin", Value::str("project_invented")), ("source_name", Value::str("writer preflight; never output"))],
                    );
                }
            }
        }
        let mut options = ImportOptions::new("writer preflight (geometry only)");
        options.sidecar = Some(SidecarInput::Parsed(sidecar));
        options.material_provenance = Some(Value::Dict(undeclared));
        let back = crate::import(data, &options)?;
        let prefix = "pressure_map region ";
        for item in &back.diagnostics {
            if item.get("severity").and_then(Value::as_str) != Some("error") {
                continue;
            }
            let code = item.get_or_null("code").as_str().expect("code").to_string();
            let location = item.get_or_null("location").as_str().expect("location").to_string();
            let message = item.get_or_null("message").as_str().expect("message").to_string();
            if REGION_CHECK_CODES.contains(&code.as_str()) && location.starts_with(prefix) {
                let region = &location[prefix.len()..];
                let text = format!(
                    "region {region} passes the exact route's checks on the model's values but not on the MBF's printed \
                     values (10 significant digits): {code}: {message}"
                );
                self.refuse("MBF-SI-PRESSURE-REGION-FORMAT", &format!("region:{region}"), &text);
            } else {
                let text = format!(
                    "the bound re-import of this MBF refuses it ({code} at {location}, line {}): {message}",
                    super::text(item.get_or_null("line"))?
                );
                let entity = if location.is_empty() { "MBF" } else { location.as_str() };
                self.refuse("MBF-SI-PREFLIGHT-REFUSED", entity, &text);
            }
        }
        let restored = back.summary.get_or_null("pressure_regions").clone();
        if back.status == "imported" && self.contract.is_some() && !py_eq(&restored, &Value::int(self.regions.len() as i64)) {
            let message = format!(
                "the bound re-import restored {} pressure regions, not the {} written; the region checks did not cover \
                 every region",
                super::text(&restored)?,
                self.regions.len()
            );
            self.refuse("MBF-SI-PREFLIGHT-REFUSED", "pressure_map", &message);
        }
        if back.status != "imported" && !self.has_errors() {
            self.refuse("MBF-SI-PREFLIGHT-REFUSED", "MBF", "the bound re-import of this MBF did not complete");
        }
        Ok(!self.has_errors())
    }

    fn node_comments(&self, node: &str) -> R<Vec<String>> {
        let mut comments = Vec::new();
        if let Some(springs) = self.node_springs.get(&node.to_string()) {
            if !springs.is_empty() {
                let values: Vec<String> = DOFS[..3].iter().map(|dof| num(springs.get(dof).copied().unwrap_or(0.0))).collect();
                comments.push(format!("TRAK({})", values.join(",")));
                comments.push("ROTK(0,0,0)".into());
            }
        }
        if let Some(restrained) = self.node_restraints.get(node) {
            if !restrained.is_empty() && !self.is_anchor(node) {
                let translations: Vec<i32> = DOFS[..3].iter().map(|dof| restrained.contains(dof) as i32).collect();
                let rotations: Vec<i32> = DOFS[3..].iter().map(|dof| restrained.contains(dof) as i32).collect();
                if translations.iter().any(|value| *value != 0) {
                    comments.push(format!("FIXD({},{},{})", translations[0], translations[1], translations[2]));
                }
                if rotations.iter().any(|value| *value != 0) {
                    return Err(py(PyError { kind: "AssertionError", message: "partial rotational restraint reached rendering".into() }));
                }
            }
        }
        for (code, store) in [("FOR", &self.forces), ("MOM", &self.moments), ("D", &self.anchor_disp), ("ROT", &self.anchor_rot)] {
            if let Some(values) = store.get(&node.to_string()) {
                if values.iter().any(|value| *value != 0.0) {
                    let shown: Vec<String> = values.iter().map(|value| num(*value)).collect();
                    comments.push(format!("{code}({})", shown.join(",")));
                }
            }
        }
        Ok(comments)
    }

    fn add_assumptions(&mut self, temperatures: &[f64]) {
        let sustained = crate::pystr::py_repr_str(SUSTAINED);
        let expansion = crate::pystr::py_repr_str(EXPANSION);
        let listed: Vec<String> = temperatures.iter().map(|value| format!("{} C", num(*value))).collect();
        let pressure = if !self.pipe_pressure.is_empty() {
            "Every LOADS entry has Pressure 1 = the element's pressure-region pressure in the sustained case, in bar (1 \
             bar = 1e5 Pa exactly; psi by its exact definition), or 0 outside every region"
        } else {
            "Every LOADS entry has Pressure 1 = 0"
        };
        let mut notes = vec![
            format!(
                "Profile {MBF_SI_PROFILE_ID}: MBF version {MBF_VERSION}, OPTIONS SI with the default vertical axis Y; \
                 SWBPIPE global X, Y, Z are written unchanged as the target program's X, Y, Z, with gravity along -Y. \
                 The writer always writes Y vertical: SWBPIPE has one vertical convention (weight along -Y) and the \
                 model document records no vertical axis, so a Z-vertical import, which arrives rotated, is written \
                 Y-vertical."
            ),
            "Lengths are written in mm, forces in N, moments in N*m, modulus in N/mm2 (= MPa), temperatures in C; SI \
             source units scale to these by an exact power of ten, and US-customary units by the exact definitions \
             listed in MBF-SI-UNITS-NORMALIZED (Design caepipe-mbf-import.md section 5.2)."
                .to_string(),
            format!(
                "Load case mapping: mechanical cases (concentrated force, concentrated moment, weight) -> {sustained}; \
                 thermal and imposed-displacement cases -> {expansion}."
            ),
            format!(
                "Materials are written temperature-independent: identical E and expansion coefficient at {}; the \
                 optional allowable, yield and rupture columns are omitted.",
                listed.join(", ")
            ),
            format!(
                "{pressure} and specific gravity 0 (empty pipe); Temperature 1 = reference temperature + the element's \
                 SWBPIPE thermal change (summed when several primitives act on one element)."
            ),
        ];
        if let Some(code) = self.piping_code.as_str() {
            let opcode = if code.starts_with("B31") {
                format!(
                    "; OPCODE is written {OPCODE_LINE}: axial force in the code stress off and liberal allowable off \
                     (code-stress settings only), and B31J off, so SIFs and flexibility factors come from the code's \
                     own appendix and a tee assembles no branch flexibility (Analysis; Format of .MBF, OPCODE)"
                )
            } else {
                "; OPCODE is not written (its B31J field applies to ASME B31 codes only)".to_string()
            };
            notes.push(format!(
                "Piping code {code}, chosen by the caller, is written in OPTIONS{opcode}. Every bend carries the user \
                 flexibility factor, which replaces the code's value (Bend)."
            ));
        }
        if !self.node_springs.is_empty() {
            notes.push(
                "Spring supports are written as flexible anchors KA,TRAK(kx,ky,kz),ROTK(0,0,0), stiffness in N/mm; a \
                 zero component leaves its DOF free and ROTK(0,0,0) leaves rotations free (technical reference, \
                 Anchor)."
                    .into(),
            );
        }
        if !self.pipe_pressure.is_empty() {
            notes.push(format!(
                "OPPRES is written {OPPRES_LINE}: PD/4t, Bourdon effect off, pressure correction for bends off, peak \
                 pressure factor 1. SWBPIPE's exact pressure route models neither effect (DEL-04-07 D-3)."
            ));
        }
        if self.contract.is_some() {
            notes.push(
                "The pressure contract, region ids, members, terminals with their closure transfer and the provenance \
                 are not in the MBF; the sidecar's pressure_map carries them for a bound import."
                    .into(),
            );
        }
        if !self.weight.is_empty() {
            notes.push(format!(
                "Weight per length w (global -Y) is written as material weight density rho = |w| / (A * {} m/s^2) with \
                 A the bare metal area pi/4 (OD^2 - ID^2); kgf/m3 equals kg/m3 numerically. Elements without a weight \
                 primitive have density 0.",
                crate::pyfmt::py_float_repr(STANDARD_GRAVITY).expect("finite")
            ));
        }
        if self.materials.values().any(|material| !material.has_alpha) {
            notes.push("A material without thermal_expansion_coefficient is written with expansion coefficient 0.".into());
        }
        if self.rotation_converted {
            notes.push("Rotations authored in rad are written in degrees (x 180/pi).".into());
        }
        if !self.bends.is_empty() {
            notes.push(
                "Each curved bend (curved_bend_macro_element) is written as one layout row at the tangent intersection \
                 (TI) of its adjacent straights: 'T<n>,KI,B<radius mm>,<offsets to the TI>' with the user flexibility \
                 factor as the comment 'FF=<k>'; the incoming straight pipe and the bend share that row and its \
                 properties. The target program reports the near and far tangent points as nodes '<n>A' and '<n>B', \
                 which node_map assigns to the SWBPIPE tangent-point nodes; the next row's offsets start at the TI. \
                 The single user factor stands for SWBPIPE's in-plane and out-of-plane bending factor alike."
                    .into(),
            );
        }
        if self.bends.values().any(|bend| bend.sifs.is_some()) {
            notes.push(
                "A bend's directional SIF pair is written as the comment 'BSIF(<in-plane>,<out-of-plane>)' (Format of \
                 .MBF, BSIF), from SWBPIPE's in-plane (M_z of the arc's tangent frame) and out-of-plane (M_y) user \
                 values."
                    .into(),
            );
        }
        notes.push(
            "Model members the SWBPIPE solve does not read (analysis_status, data_boundary, diagnostics, lines, \
             unresolved_assumptions, constraints, equipment_interfaces, rule_pack_refs) are not exported."
                .into(),
        );
        self.assumptions.extend(notes);
    }
}

/// `min(values)` over a non-empty sequence, as Python evaluates it.
fn py_min_all(values: &[f64]) -> f64 {
    let mut best = values[0];
    for &value in &values[1..] {
        if value < best {
            best = value;
        }
    }
    best
}
