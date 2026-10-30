//! importer.py assembly (§15, §16): nodes, materials, pipes, supports and load cases into the model document.

use super::{Element, Importer, Num, Stop, MODEL_KIND, R};
use crate::dict;
use crate::pyfmt::py_repr;
use crate::pystr::{py_repr_str, py_strip, skip_chars};
use crate::rational::Q;
use crate::runtime::{bend_tangents, dot, frame_axes, local_y_reference, section_valid, AXIS_TOLERANCE, V3};
use crate::sidecar::{direction_axis, primitive_key, runtime_support_findings, Axis, PrimKey, DOFS, MATERIAL_ORIGINS, SPRING_FAMILY};
use crate::value::{py_eq, str_in, Dict, OMap, Value};
use crate::MODEL_SCHEMA_VERSION;
use std::collections::{BTreeSet, HashMap, HashSet};

/// si_writer `BEND_SIF_FIELDS`: a bend's directional SIF pair, from the comment BSIF(in-plane,out-plane).
const BEND_SIF_FIELDS: [&str; 2] = ["sif_in_plane_user_value", "sif_out_of_plane_user_value"];

pub(crate) const SUSTAINED_CASE: &str = "case:mbf-sustained";
pub(crate) const EXPANSION_CASE: &str = "case:mbf-expansion-t1";
pub(crate) const LABEL_SUSTAINED: &str = "Sustained (W+P)";
pub(crate) const LABEL_EXPANSION: &str = "Expansion (T1)";
pub(crate) const INVENTED_CERTIFICATION: &str = "invented non-engineering example";
pub(crate) const USER_SUPPLIED_CERTIFICATION: &str = "user-supplied own source";
pub(crate) const STANDARD_GRAVITY: f64 = 9.80665;
const PSI_PA: f64 = 4.448_221_615_260_5 / 0.000_645_16;
const LBF_PER_IN_N_PER_M: f64 = 4.448_221_615_260_5 / 0.0254;

/// A support as assembled: (id, DOFs, family, line, label).
type Imported = (String, Vec<String>, Option<String>, usize, String);

/// One primitive load before ids are chosen.
pub(crate) struct Primitive {
    pub category: &'static str,
    pub target: Value,
    pub target_id: String,
    pub target_kind: &'static str,
    pub direction: String,
    pub magnitude: f64,
    pub unit: String,
    pub dimension: &'static str,
    pub line: usize,
    pub axis: Axis,
}

/// What a restored primitive key maps to: (sidecar id, count, direction, [(map key, direction)]).
pub(crate) type Restorable = (String, usize, Option<String>, Vec<(String, Option<String>)>);

pub(crate) struct Claims {
    used: HashMap<String, String>,
}

impl<'a> Importer<'a> {
    pub(crate) fn provenance(&self, line: usize) -> String {
        format!("imported from MBF sha256:{}, line {}", self.sha, line)
    }

    pub(crate) fn claim(&mut self, claims: &mut Claims, swb_id: &str, what: &str, line: usize) -> String {
        if let Some(previous) = claims.used.get(swb_id) {
            if previous != what {
                let message = format!("{what} would take id {swb_id}, which {previous} already has");
                self.refuse("MBF-IMPORT-SIDECAR-CONFLICT", line, what, &message);
            }
        }
        claims.used.insert(swb_id.to_string(), what.to_string());
        swb_id.to_string()
    }

    fn node_name_id(&mut self, name: &str) -> String {
        if (self.bound || self.rebound) && self.node_ids.contains_key(&name.to_string()) {
            let restored = self.node_ids.get(&name.to_string()).expect("node").clone();
            if self.rebound {
                self.unconfirmed.push(restored.clone());
            }
            return restored;
        }
        format!("node:{name}")
    }

    fn map(&self, name: &str) -> Dict {
        self.maps.get(name).and_then(Value::as_dict).cloned().unwrap_or_default()
    }

    pub(crate) fn assemble(&mut self) -> R<(Value, Value)> {
        let length_unit = if self.si { "mm" } else { "in" };
        let angle_unit = if self.rad { "rad" } else { "deg" };
        let binding = if self.bound { "bound" } else if self.rebound { "rebound_unconfirmed" } else { "unbound" };
        let mut id_map = Dict::new();
        id_map.insert("mbf_sha256".into(), Value::str(self.sha.clone()));
        id_map.insert("binding".into(), Value::str(binding));
        for key in ["node_map", "element_map", "support_map", "bend_map", "material_map", "load_case_map", "load_map"] {
            id_map.insert(key.into(), Value::Dict(Dict::new()));
        }
        let mut claims = Claims { used: HashMap::new() };

        // nodes
        let mut nodes = Vec::new();
        let mut node_names: OMap<u32, Vec<String>> = OMap::new();
        let mut point_of: HashMap<String, V3> = HashMap::new();
        let mut node_map = Dict::new();
        for number in self.build.node_order.clone() {
            let node_line = self.build.nodes.get(&number).expect("node").line;
            let (names, points): (Vec<String>, Vec<V3>) = match self.build.bend_geometry.get(&number) {
                Some(bend) => (vec![format!("{number}A"), format!("{number}B")], vec![bend.near, bend.far]),
                None => (vec![number.to_string()], vec![self.position(number)]),
            };
            node_names.insert(number, names.clone());
            for (name, point) in names.iter().zip(points) {
                point_of.insert(name.clone(), point);
                let formed = self.node_name_id(name);
                let swb_id = self.claim(&mut claims, &formed, &format!("CAEPIPE node {name}"), node_line);
                node_map.insert(name.clone(), Value::str(swb_id.clone()));
                nodes.push(dict![
                    ("id", Value::str(swb_id)),
                    ("label", Value::str(name.clone())),
                    ("position", dict![("x", Value::Float(point[0])), ("y", Value::Float(point[1])), ("z", Value::Float(point[2]))]),
                    ("provenance", Value::str(self.provenance(node_line))),
                ]);
            }
        }
        let mut node_id: OMap<String, String> = OMap::new();
        for names in node_names.values() {
            for name in names {
                node_id.insert(name.clone(), node_map.get(name).and_then(Value::as_str).expect("node id").to_string());
            }
        }
        id_map.insert("node_map".into(), Value::Dict(node_map.clone()));
        if self.bound {
            let in_file: Vec<String> = node_id.keys().cloned().collect();
            let in_sidecar: Vec<String> = self.node_ids.keys().cloned().collect();
            self.incomplete("node_map", &in_file, &in_sidecar, "nodes");
        }

        // materials (§7)
        let (materials, material_ids) = self.assemble_materials(&mut id_map, &mut claims)?;

        // pipes, weights and temperatures
        let factor = if self.si { 0.001 } else { 0.0254 };
        let mut element_lookup: OMap<(String, String), (String, Value)> = OMap::new();
        for (swb_id, entry) in self.map("element_map").iter() {
            let key = (entry.get_or_null("from").as_str().unwrap_or_default().to_string(), entry.get_or_null("to").as_str().unwrap_or_default().to_string());
            element_lookup.insert(key, (swb_id.clone(), entry.clone()));
        }
        let mut pipes: Vec<Value> = Vec::new();
        let mut components: Vec<Value> = Vec::new();
        let mut unresolved: Vec<Value> = Vec::new();
        let mut pressures: OMap<String, (Option<Num>, Element)> = OMap::new();
        let mut weights: Vec<(String, f64, usize)> = Vec::new();
        let mut thermals: Vec<(String, f64, usize)> = Vec::new();
        let mut element_pairs: Vec<(String, String)> = Vec::new();
        self.build.carriers = OMap::new();
        let mut derived_y: Vec<Value> = Vec::new();
        let mut element_map = Dict::new();
        let elements = self.build.elements.clone();
        for element in &elements {
            let start = node_names.get(&element.start).expect("names").last().expect("name").clone();
            let end_names = node_names.get(&element.end).expect("names").clone();
            let end = end_names[0].clone();
            let mut members: Vec<(String, String, bool)> = vec![(start, end, false)];
            if element.ti {
                members.push((end_names[0].clone(), end_names[1].clone(), true));
            }
            for (a, b, is_bend) in members {
                let caepipe_name = format!("{a}-{b}");
                element_pairs.push((a.clone(), b.clone()));
                let mut swb_id = format!("pipe:{caepipe_name}");
                let mut reversed = false;
                let mut entry = Value::Dict(Dict::new());
                if self.bound || self.rebound {
                    if let Some((found_id, found)) = element_lookup.get(&(a.clone(), b.clone())).cloned() {
                        swb_id = found_id;
                        entry = found;
                        reversed = entry.get("reversed").and_then(Value::as_bool).unwrap_or(false);
                        if self.rebound {
                            self.unconfirmed.push(swb_id.clone());
                        }
                        if entry.has("bend") != is_bend {
                            let message = format!(
                                "the sidecar records {} bend for {swb_id}, but the file's element {caepipe_name} is {}",
                                if entry.has("bend") { "a" } else { "no" },
                                if is_bend { "a bend" } else { "straight" }
                            );
                            self.refuse("MBF-IMPORT-SIDECAR-CONFLICT", element.line, &format!("element_map[{swb_id}].bend"), &message);
                        }
                        if self.bound && !self.sidecar_current {
                            let order = element_pairs.len() - 1;
                            self.legacy_element(&swb_id, &entry, element, &caepipe_name, order, is_bend)?;
                        }
                    }
                }
                self.claim(&mut claims, &swb_id, &format!("CAEPIPE element {caepipe_name}"), element.line);
                element_map.insert(caepipe_name.clone(), Value::str(swb_id.clone()));
                let section = self.pipes.get(&element.section).expect("section").clone();
                let id_a = node_id.get(&a).expect("a").clone();
                let id_b = node_id.get(&b).expect("b").clone();
                let (from, to) = if reversed { (id_b.clone(), id_a.clone()) } else { (id_a.clone(), id_b.clone()) };
                let od = section.od.as_ref().expect("od").value;
                let thk = section.thk.as_ref().expect("thk").value;
                let mut pipe = Dict::new();
                pipe.insert("id".into(), Value::str(swb_id.clone()));
                pipe.insert("label".into(), Value::str(caepipe_name.clone()));
                pipe.insert("from".into(), Value::str(from));
                pipe.insert("to".into(), Value::str(to));
                pipe.insert("material".into(), Value::str(material_ids.get(&element.material).expect("material").clone()));
                pipe.insert(
                    "section".into(),
                    dict![
                        ("outside_diameter", dict![("value", Value::Float(od)), ("unit", Value::str(length_unit))]),
                        ("wall_thickness", dict![("value", Value::Float(thk)), ("unit", Value::str(length_unit))]),
                    ],
                );
                pipe.insert("provenance".into(), Value::str(self.provenance(element.line)));
                let restored = if self.bound { entry.get("y_reference").filter(|value| !value.is_null()).cloned() } else { None };
                let names = if reversed { [b.clone(), a.clone()] } else { [a.clone(), b.clone()] };
                let ends_m: Vec<V3> = names.iter().map(|name| {
                    let point = point_of[name];
                    [point[0] * factor + 0.0, point[1] * factor + 0.0, point[2] * factor + 0.0]
                }).collect();
                if is_bend {
                    let bend = self.build.bend_geometry.get(&element.end).expect("bend");
                    pipe.insert("y_reference".into(), xyz(&bend.y_reference));
                } else if let Some(restored) = restored {
                    let vector = [component(&restored, "x"), component(&restored, "y"), component(&restored, "z")];
                    let usable = frame_axes(&ends_m[0], &ends_m[1], &vector).is_some();
                    pipe.insert("y_reference".into(), if usable { xyz(&vector) } else { Value::Null });
                    if !usable {
                        let message = format!(
                            "the sidecar's y_reference for {swb_id} is not one the frame kernel forms an orientation \
                             from on the MBF's positions in m (finite, a part perpendicular to the element above \
                             1e-12, and orthonormal right-handed axes within 1e-12); nothing is substituted"
                        );
                        self.refuse("MBF-IMPORT-ORIENTATION-UNUSABLE", element.line, &swb_id, &message);
                    } else if entry.get("y_reference_authored").and_then(Value::as_bool) == Some(false) {
                        derived_y.push(Value::str(swb_id.clone()));
                        let derived = local_y_reference(&point_of[&a], &point_of[&b], self.z_vertical);
                        if !py_eq(pipe.get(&"y_reference".to_string()).expect("y"), &xyz(&derived)) {
                            let message = format!(
                                "the sidecar records {swb_id}'s y_reference as derived (y_reference_authored false), but \
                                 it is not CAEPIPE's documented local y on the MBF's positions"
                            );
                            self.refuse("MBF-IMPORT-SIDECAR-CONFLICT", element.line, &format!("element_map[{swb_id}].y_reference"), &message);
                        } else {
                            let message = format!(
                                "{swb_id} has no authored y_reference; the sidecar records the derived one, CAEPIPE's \
                                 documented pipe local y (CAEPIPE-LOCAL-AXES), equal to the derivation"
                            );
                            self.info("MBF-IMPORT-ORIENTATION-DEFAULTED", element.line, &swb_id, &message);
                        }
                    }
                } else {
                    derived_y.push(Value::str(swb_id.clone()));
                    let documented = local_y_reference(&point_of[&a], &point_of[&b], self.z_vertical);
                    pipe.insert("y_reference".into(), xyz(&documented));
                    let vertical = if self.z_vertical { "global Z vertical, in the rotated axes" } else { "global Y vertical" };
                    let message = format!(
                        "{swb_id} has no y_reference in the MBF; it takes CAEPIPE's documented pipe local y axis \
                         (CAEPIPE-LOCAL-AXES, {vertical}). A circular section's stiffness does not depend on it; only \
                         local-axis results do"
                    );
                    self.info("MBF-IMPORT-ORIENTATION-DEFAULTED", element.line, &swb_id, &message);
                    let axis = [ends_m[1][0] - ends_m[0][0], ends_m[1][1] - ends_m[0][1], ends_m[1][2] - ends_m[0][2]];
                    if frame_axes(&ends_m[0], &ends_m[1], &documented).is_none() && dot(&axis, &axis).sqrt() > AXIS_TOLERANCE {
                        let message = format!(
                            "the frame kernel forms no orientation for {swb_id} from CAEPIPE's documented local y on \
                             the MBF's positions in m (they differ in direction from the file's own values), and the \
                             MBF states no other"
                        );
                        self.refuse("MBF-IMPORT-ORIENTATION-UNUSABLE", element.line, &swb_id, &message);
                    }
                }
                if is_bend {
                    let near_id = id_a.clone();
                    let component = self.bend_component(element, &swb_id, &near_id, length_unit, &mut unresolved, &mut id_map, &mut claims);
                    components.push(component);
                }
                pipes.push(Value::Dict(pipe));
                self.build.carriers.entry_or_insert_with(("material".into(), element.material.clone()), Vec::new).push(swb_id.clone());
                self.build.carriers.entry_or_insert_with(("section".into(), element.section.clone()), Vec::new).push(swb_id.clone());
                let p1 = element.load.as_ref().map(|load| self.loads.get(load).expect("load").p1.clone().expect("p1"));
                pressures.insert(swb_id.clone(), (p1, element.clone()));
                let weight = self.weight_per_length(element);
                let density_nonzero = !self.materials.get(&element.material).expect("material").density.as_ref().expect("density").exact.is_zero();
                if weight != 0.0 || density_nonzero {
                    let weight = self.binary64_f(weight, element.line, &format!("LAYOUT line {}", element.line), "weight per length", true)?;
                    weights.push((swb_id.clone(), weight, element.line));
                }
                if let Some(load) = &element.load {
                    let delta = match self.t_ref.clone() {
                        Some(t_ref) => {
                            let t1 = self.loads.get(load).expect("load").t1.clone().expect("t1");
                            Some(self.binary64_q(&t1.exact.sub(&t_ref.exact), element.line, &format!("LOADS {load}"), "temperature change", false)?)
                        }
                        None => None,
                    };
                    let Some(delta) = delta else {
                        self.refuse(
                            "MBF-IMPORT-REFERENCE-TEMPERATURE-MISSING",
                            element.line,
                            &format!("LAYOUT line {}", element.line),
                            "a LOADS temperature is read, but OPTEMP gives no reference temperature",
                        );
                        return Err(Stop::Refused);
                    };
                    if delta != 0.0 {
                        thermals.push((swb_id.clone(), delta, element.line));
                    }
                }
            }
        }
        id_map.insert("element_map".into(), Value::Dict(element_map.clone()));
        if self.bound {
            let in_file: Vec<String> = element_pairs.iter().map(|(a, b)| format!("{a}-{b}")).collect();
            let in_sidecar: Vec<String> = element_lookup.keys().map(|(a, b)| format!("{a}-{b}")).collect();
            self.incomplete("element_map", &in_file, &in_sidecar, "elements");
        }
        let mut name_of: HashMap<String, String> = HashMap::new();
        for (name, swb_id) in node_map.iter() {
            name_of.insert(swb_id.as_str().expect("id").to_string(), name.clone());
        }
        let mut radius_of: HashMap<String, f64> = HashMap::new();
        for component in &components {
            let geometry = component.get_or_null("geometry");
            radius_of.insert(
                geometry.get_or_null("bend_pipe_ref").as_str().expect("ref").to_string(),
                geometry.get_or_null("bend_radius").get_or_null("value").as_f64().expect("radius"),
            );
        }
        self.build.frame_x = HashMap::new();
        self.build.length_m = HashMap::new();
        self.build.arc_tangents = HashMap::new();
        for pipe in &pipes {
            let id = pipe.get_or_null("id").as_str().expect("id").to_string();
            let point = |end: &str| -> V3 {
                let p = point_of[&name_of[pipe.get_or_null(end).as_str().expect("end")]];
                [p[0] * factor + 0.0, p[1] * factor + 0.0, p[2] * factor + 0.0]
            };
            let (a, b) = (point("from"), point("to"));
            if a == b {
                let line = pressures.get(&id).expect("pipe").1.line;
                self.refuse("MBF-IMPORT-GEOMETRY-ZERO-LENGTH", line, &id, &format!("{id} has zero length in metres, the unit the solve reads"));
                continue;
            }
            let y = pipe.get_or_null("y_reference");
            if y.is_null() {
                continue;
            }
            let y_reference = [component(y, "x"), component(y, "y"), component(y, "z")];
            let axes = frame_axes(&a, &b, &y_reference);
            self.build.frame_x.insert(id.clone(), axes.map(|axes| axes[0]));
            let axis = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
            self.build.length_m.insert(id.clone(), dot(&axis, &axis).sqrt());
            if let Some(radius) = radius_of.get(&id) {
                self.build.arc_tangents.insert(id.clone(), bend_tangents(&a, &b, radius * factor + 0.0, &y_reference));
            }
        }
        let mut sections_used: Vec<String> = Vec::new();
        for element in &elements {
            if !sections_used.contains(&element.section) {
                sections_used.push(element.section.clone());
            }
        }
        if self.bound && !self.sidecar_current {
            let in_sidecar: Vec<String> = self.map("section_map").keys().cloned().collect();
            self.incomplete("section_map", &sections_used, &in_sidecar, "sections");
            self.legacy_tables()?;
        }
        for name in &sections_used {
            let section = self.pipes.get(name).expect("section").clone();
            if !section_valid(section.od.as_ref().expect("od").value, section.thk.as_ref().expect("thk").value, factor) {
                let message = format!(
                    "section {name} needs 0 < thickness < OD/2 with a bore radius OD/2 - thickness that is positive and \
                     below OD/2 on the values in m the solve reads; the file's decimals pass, but their binary64 values \
                     in m do not"
                );
                self.refuse("MBF-IMPORT-VALUE-OUT-OF-RANGE", section.line, &format!("PIPE {name}"), &message);
            }
        }
        if self.has_errors() {
            return Err(Stop::Refused);
        }
        if !elements.is_empty() && elements.iter().all(|element| element.load.is_none()) {
            let header = self.sections.get(&"LAYOUT".to_string()).map(|(line, _)| *line).unwrap_or(0);
            self.listed(
                "tbd",
                "MBF-IMPORT-LOADS-ABSENT",
                header,
                "LAYOUT",
                "the file has no LOADS section; elements are imported with no temperature, pressure or contents load, \
                 and CAEPIPE's treatment of an element without a load record is not documented",
            );
        }
        self.list_unused(&material_ids);
        let (supports, support_ids) = self.assemble_supports(&node_id, &mut id_map, &mut claims)?;
        let findings_input: Vec<(String, String, Option<String>, Vec<String>)> = supports
            .iter()
            .map(|support| {
                (
                    support.get_or_null("id").as_str().expect("id").to_string(),
                    support.get_or_null("node").as_str().expect("node").to_string(),
                    support.get("family").and_then(Value::as_str).map(String::from),
                    support.get_or_null("restraints").as_list().expect("list").iter().map(|dof| dof.as_str().expect("dof").to_string()).collect(),
                )
            })
            .collect();
        for (support_id, reason) in runtime_support_findings(&findings_input) {
            let line = *self.build.support_lines.get(&support_id).expect("line");
            let message = format!("support {support_id}: {reason}; the solve refuses it (SUPPORT_INPUT_INVALID)");
            self.refuse("MBF-IMPORT-SUPPORT-UNSUPPORTED", line, &support_id, &message);
        }
        if self.has_errors() {
            return Err(Stop::Refused);
        }
        self.build.labels = pipes
            .iter()
            .map(|pipe| (pipe.get_or_null("id").as_str().expect("id").to_string(), pipe.get_or_null("label").as_str().expect("label").to_string()))
            .collect();
        let arcs: HashSet<String> = components
            .iter()
            .map(|component| component.get_or_null("geometry").get_or_null("bend_pipe_ref").as_str().expect("ref").to_string())
            .collect();
        let pressure = self.assemble_pressure(&pipes, &pressures, &arcs, &point_of, &mut id_map, &mut claims)?;
        let regions = pressure.as_ref().map(|(_, regions)| regions.clone());
        let load_cases = self.assemble_loads(&node_id, &support_ids, &weights, &thermals, length_unit, angle_unit, &mut id_map, &mut claims, regions.as_ref())?;
        if self.has_errors() {
            return Err(Stop::Refused);
        }
        if pressure.is_some() {
            let moved: Vec<String> = load_cases
                .iter()
                .flat_map(|case| case.get_or_null("primitive_loads").as_list().cloned().unwrap_or_default())
                .filter(|load| load.get_or_null("category").as_str() == Some("imposed_displacement"))
                .map(|load| load.get_or_null("id").as_str().expect("id").to_string())
                .collect();
            if !moved.is_empty() {
                self.refuse(
                    "MBF-IMPORT-PRESSURE-DISPLACEMENT-UNSUPPORTED",
                    0,
                    &moved.join(", "),
                    "the file has pressure and anchor displacements; the exact pressure route takes boundary motion only \
                     in load-state form (EXACT_IMPOSED_DISPLACEMENT_PRIMITIVE_UNSUPPORTED), which this importer does not \
                     write",
                );
                return Err(Stop::Refused);
            }
            let missing: BTreeSet<String> = elements
                .iter()
                .filter(|element| !self.pipes.get(&element.section).expect("section").allowance_given)
                .map(|element| element.section.clone())
                .collect();
            for name in &missing {
                let line = self.pipes.get(name).expect("section").line;
                let message = format!(
                    "section {name} gives no corrosion allowance; the exact pressure contract needs an explicit \
                     allowance on every section, and none is supplied"
                );
                self.refuse("MBF-IMPORT-CORROSION-ALLOWANCE-MISSING", line, &format!("PIPE {name} field 4"), &message);
            }
            if !missing.is_empty() {
                return Err(Stop::Refused);
            }
            for pipe in pipes.iter_mut() {
                let section = pipe.as_dict_mut().expect("pipe").get_mut(&"section".to_string()).expect("section");
                section.as_dict_mut().expect("section").insert(
                    "corrosion_allowance".into(),
                    dict![("value", Value::Float(0.0)), ("unit", Value::str(length_unit))],
                );
            }
            let ids_of = |items: &[Value]| -> Vec<String> { items.iter().map(|item| item.get_or_null("id").as_str().expect("id").to_string()).collect() };
            for (kind, ids) in [("pipe", ids_of(&pipes)), ("node", ids_of(&nodes)), ("support", ids_of(&supports)), ("load_case", ids_of(&load_cases))] {
                let mut seen: HashMap<String, String> = HashMap::new();
                for identity in ids {
                    let previous = seen.entry(identity.replace(':', "-")).or_insert_with(|| identity.clone()).clone();
                    if previous != identity {
                        let message = format!(
                            "{kind} ids {previous} and {identity} are equal once ':' is read as '-', which the exact route \
                             refuses (EXACT_PRESSURE_RESULT_ID_COLLISION)"
                        );
                        self.refuse("MBF-IMPORT-PRESSURE-ID-COLLISION", 0, &format!("{kind} {previous}, {identity}"), &message);
                    }
                }
            }
            if self.has_errors() {
                return Err(Stop::Refused);
            }
        }
        self.report_unrecoverable();

        let units = if self.si {
            dict![
                ("length", Value::str("mm")), ("force", Value::str("N")), ("moment", Value::str("N*m")),
                ("pressure", Value::str("MPa")), ("stress", Value::str("Pa")), ("temperature", Value::str("degC")),
                ("angle", Value::str(angle_unit)),
            ]
        } else {
            dict![
                ("length", Value::str("in")), ("force", Value::str("lbf")), ("moment", Value::str("lbf*in")),
                ("pressure", Value::str("psi")), ("stress", Value::str("psi")), ("temperature", Value::str("degF")),
                ("angle", Value::str(angle_unit)),
            ]
        };
        let title = match self.heading.first() {
            Some((_, text)) if !py_strip(text).is_empty() => py_strip(text).to_string(),
            _ => self.options.source_label.clone(),
        };
        let mut notes: Vec<String> = self
            .heading
            .iter()
            .skip(1)
            .filter(|(_, text)| !py_strip(text).is_empty())
            .map(|(line, text)| format!("line {line}: {}", py_strip(text)))
            .collect();
        notes.extend(self.notes.iter().cloned());
        let mut project = Dict::new();
        project.insert("id".into(), Value::Str(format!("project:mbf-{}", &self.sha[..12])));
        project.insert("name".into(), Value::Str(title));
        project.insert("description".into(), Value::Str(format!("Imported from MBF sha256:{}", self.sha)));
        project.insert("units".into(), units);
        if !notes.is_empty() {
            project.insert("notes".into(), Value::Str(notes.join("\n")));
        }
        let binding_text = if self.bound {
            "bound to the SWBPIPE writer sidecar"
        } else if self.rebound {
            "unbound; sidecar ids rebound by CAEPIPE name, unconfirmed"
        } else {
            "unbound"
        };
        let mut model = Dict::new();
        model.insert("schema_version".into(), Value::str(MODEL_SCHEMA_VERSION));
        model.insert("document_kind".into(), Value::str(MODEL_KIND));
        if let Some((contract, _)) = &pressure {
            model.insert("pressure_contract".into(), contract.clone());
        }
        model.insert(
            "data_boundary".into(),
            dict![
                ("public_examples_policy", Value::str("blank_user_created_local_document_no_bundled_engineering_values")),
                ("protected_source_policy", Value::str("no_protected_standards_content_inserted")),
                ("professional_boundary", Value::str("human_review_required_no_software_approval_claim")),
                (
                    "private_data_policy",
                    Value::Str(format!(
                        "imported from MBF sha256:{} ({binding_text}); local user document not committed to repository",
                        self.sha
                    ))
                ),
            ],
        );
        model.insert("project".into(), Value::Dict(project));
        model.insert(
            "analysis_status".into(),
            dict![
                ("mechanics", Value::str("ready_for_preview_diagnostics")),
                ("rule_check", Value::str("not_performed_user_rule_inputs_missing")),
                ("professional_acceptance", Value::str("not_provided")),
            ],
        );
        model.insert("materials".into(), Value::List(materials));
        model.insert("nodes".into(), Value::List(nodes));
        model.insert("pipe_segments".into(), Value::List(pipes));
        model.insert("supports".into(), Value::List(supports));
        model.insert("components".into(), Value::List(components));
        model.insert("load_cases".into(), Value::List(load_cases));
        model.insert("diagnostics".into(), Value::List(Vec::new()));
        if !unresolved.is_empty() {
            model.insert("unresolved_assumptions".into(), Value::List(unresolved));
        }
        let mut unconfirmed: Vec<String> = self.unconfirmed.clone();
        unconfirmed.sort();
        unconfirmed.dedup();
        id_map.insert("rebound_unconfirmed".into(), Value::List(unconfirmed.into_iter().map(Value::Str).collect()));
        id_map.insert("y_reference_derived".into(), Value::List(derived_y));
        Ok((Value::Dict(model), Value::Dict(id_map)))
    }

    #[allow(clippy::too_many_arguments)]
    fn bend_component(
        &mut self,
        element: &Element,
        pipe_id: &str,
        near_id: &str,
        length_unit: &str,
        unresolved: &mut Vec<Value>,
        id_map: &mut Dict,
        claims: &mut Claims,
    ) -> Value {
        let n = element.end;
        let mut component_id = format!("component:bend-{n}");
        if self.bound || self.rebound {
            let bend = self.map("element_map").get(&pipe_id.to_string()).and_then(|entry| entry.get("bend").cloned());
            if let Some(bend) = bend.filter(|bend| !bend.is_null()) {
                component_id = bend.get_or_null("component").as_str().unwrap_or_default().to_string();
                if self.rebound {
                    self.unconfirmed.push(component_id.clone());
                }
            }
            if self.bound {
                let message = format!(
                    "the sidecar does not record which tangent node bend {n}'s component names; the importer uses the \
                     near tangent node {n}A"
                );
                self.listed("approximated", "MBF-IMPORT-ID-UNRECOVERABLE", element.line, &format!("bend {n}"), &message);
            }
        }
        self.claim(claims, &component_id, &format!("CAEPIPE bend {n}"), element.line);
        id_map.get_mut(&"bend_map".to_string()).and_then(Value::as_dict_mut).expect("bend_map").insert(n.to_string(), Value::str(component_id.clone()));
        let radius = element.radius.as_ref().expect("radius").value;
        let mut component = Dict::new();
        component.insert("id".into(), Value::str(component_id.clone()));
        component.insert("label".into(), Value::Str(format!("bend {n}")));
        component.insert("kind".into(), Value::str("bend"));
        component.insert("node".into(), Value::str(near_id));
        component.insert(
            "geometry".into(),
            dict![
                ("bend_pipe_ref", Value::str(pipe_id)),
                ("bend_radius", dict![("value", Value::Float(radius)), ("unit", Value::str(length_unit))]),
                ("bend_geometry_source_reference", Value::Str(format!("MBF sha256:{}, line {}", self.sha, element.line))),
            ],
        );
        component.insert("mechanics_interface".into(), dict![("solver_consumption", Value::str("curved_bend_macro_element"))]);
        component.insert("provenance".into(), Value::str(self.provenance(element.line)));
        let comments = if element.sifs.is_some() { "comments FF and BSIF" } else { "comment FF" };
        if let Some(factor) = &element.factor {
            component.insert(
                "modifiers".into(),
                dict![
                    ("flexibility_factor_user_value", dict![("value", Value::Float(factor.value)), ("unit", Value::str("none"))]),
                    ("source_reference", Value::Str(format!("MBF sha256:{}, line {}, {comments}", self.sha, element.line))),
                ],
            );
        } else {
            component.insert(
                "modifiers".into(),
                dict![(
                    "source_reference",
                    Value::Str(format!(
                        "MBF sha256:{}, line {}: the MBF supplies no flexibility factor{}",
                        self.sha,
                        element.line,
                        if element.sifs.is_some() { ", comment BSIF" } else { "" }
                    ))
                )],
            );
            unresolved.push(dict![
                ("assumption_id", Value::Str(format!("assumption:mbf-bend-{n}-flexibility"))),
                ("kind", Value::str("open_issue")),
                ("statement", Value::Str(format!("bend {n} has no flexibility factor in the MBF; enter a user factor"))),
                ("status", Value::str("unresolved")),
                ("affected_refs", Value::List(vec![Value::str(component_id.clone())])),
            ]);
            let message = format!("bend {n} has no FF comment; SWBPIPE never supplies a factor, so the bend is incomplete");
            self.listed("omitted", "MBF-IMPORT-BEND-FLEXIBILITY-MISSING", element.line, &format!("bend {n}"), &message);
        }
        if let Some((in_plane, out_of_plane)) = &element.sifs {
            if let Some(Value::Dict(modifiers)) = component.get_mut(&"modifiers".to_string()) {
                for (name, number) in BEND_SIF_FIELDS.iter().zip([in_plane, out_of_plane]) {
                    modifiers.insert(
                        (*name).into(),
                        dict![("value", Value::Float(number.value)), ("unit", Value::str("none"))],
                    );
                }
            }
        }
        Value::Dict(component)
    }

    fn weight_per_length(&self, element: &Element) -> f64 {
        let density = self.materials.get(&element.material).expect("material").density.as_ref().expect("density").value;
        if density == 0.0 {
            return 0.0;
        }
        let section = self.pipes.get(&element.section).expect("section");
        let (mut od, mut thk) = (section.od.as_ref().expect("od").value, section.thk.as_ref().expect("thk").value);
        if self.si {
            od /= 1000.0;
            thk /= 1000.0;
        }
        // float ** 2 raises OverflowError on overflow, which the Python catches as -inf
        let Some(outer) = py_square(od) else { return f64::NEG_INFINITY };
        let Some(inner) = py_square(od - 2.0 * thk) else { return f64::NEG_INFINITY };
        let area = std::f64::consts::PI / 4.0 * (outer - inner);
        if self.si {
            -density * area * STANDARD_GRAVITY
        } else {
            -density * area
        }
    }

    fn material_qualifies(&mut self, name: &str) -> Option<(String, String, Option<String>, bool)> {
        let declared = match &self.options.material_provenance {
            Some(Value::Dict(dict)) => match dict.get(&name.to_string()) {
                Some(value) => value.clone(),
                None => dict.get(&"*".to_string()).cloned().unwrap_or(Value::Null),
            },
            _ => Value::Null,
        };
        let entry = if self.bound { self.map("material_map").get(&name.to_string()).cloned() } else { None };
        let recorded = entry.as_ref().map(|entry| entry.get_or_null("origin").clone()).unwrap_or(Value::Null);
        if str_in(&recorded, &MATERIAL_ORIGINS) {
            let recorded = recorded.as_str().expect("origin").to_string();
            if declared.as_dict().is_some() && !py_eq(declared.get_or_null("origin"), &Value::str(recorded.clone())) {
                let line = self.materials.get(&name.to_string()).expect("material").line;
                let shown = py_repr(declared.get_or_null("origin")).unwrap_or_default();
                let message = format!(
                    "the bound sidecar records material {name}'s origin as {recorded}, and the caller declares {shown}; \
                     neither is preferred"
                );
                self.refuse("MBF-IMPORT-SIDECAR-CONFLICT", line, &format!("material_map[{name}].origin"), &message);
                return None;
            }
            let status = if recorded == "user_supplied" { "private_only" } else { "public_permissive" };
            return Some((recorded, "material origin recorded by the SWBPIPE MBF writer sidecar".into(), Some(status.into()), true));
        }
        if declared.as_dict().is_some() {
            let origin = declared.get_or_null("origin");
            let source = declared.get_or_null("source_name");
            let status = declared.get_or_null("redistribution_status");
            if let (true, Value::Str(source)) = (str_in(origin, &MATERIAL_ORIGINS), source) {
                if !py_strip(source).is_empty() {
                    let origin = origin.as_str().expect("origin").to_string();
                    if origin == "user_supplied" {
                        return Some((origin, source.clone(), Some("private_only".into()), false));
                    }
                    if status.is_null() || str_in(status, &["public_permissive", "private_only"]) {
                        return Some((origin, source.clone(), status.as_str().map(String::from), false));
                    }
                }
            }
        }
        None
    }

    fn assemble_materials(&mut self, id_map: &mut Dict, claims: &mut Claims) -> R<(Vec<Value>, HashMap<String, String>)> {
        let mut used: Vec<String> = Vec::new();
        for element in &self.build.elements {
            if !used.contains(&element.material) {
                used.push(element.material.clone());
            }
        }
        let mut materials = Vec::new();
        let mut material_ids: HashMap<String, String> = HashMap::new();
        let mut merged: HashMap<String, (f64, f64, f64)> = HashMap::new();
        if self.bound {
            let in_sidecar: Vec<String> = self.map("material_map").keys().cloned().collect();
            self.incomplete("material_map", &used, &in_sidecar, "materials");
        }
        let (e_unit, alpha_unit) = if self.si { ("MPa", "1/degC") } else { ("psi", "1/degF") };
        for name in &used {
            let record = self.materials.get(name).expect("material").clone();
            if record.refused {
                return Err(Stop::Refused);
            }
            let before = self.diagnostics.len();
            let qualified = self.material_qualifies(name);
            if qualified.is_none() && self.diagnostics.len() > before {
                continue;
            }
            let Some((origin, source, status, recorded)) = qualified else {
                let message = format!(
                    "material {name} needs an affirmative project_invented or user_supplied provenance declaration; \
                     vendor-library, unknown or undeclared origins are refused"
                );
                self.refuse("MBF-IMPORT-MATERIAL-PROVENANCE-REQUIRED", record.line, &format!("MATERIAL {name}"), &message);
                continue;
            };
            let (modulus, alpha, _) = record.rows[0].clone();
            let nu = record.nu.clone().expect("nu");
            let mut swb_id = format!("material:{name}");
            if self.bound || self.rebound {
                if let Some(entry) = self.map("material_map").get(name) {
                    swb_id = entry.get_or_null("swbpipe_material").as_str().unwrap_or_default().to_string();
                    if self.rebound {
                        self.unconfirmed.push(swb_id.clone());
                    }
                }
            }
            material_ids.insert(name.clone(), swb_id.clone());
            id_map.get_mut(&"material_map".to_string()).and_then(Value::as_dict_mut).expect("material_map").insert(name.clone(), Value::str(swb_id.clone()));
            let key = (modulus.value, nu.value, alpha.value);
            if let Some(existing) = merged.get(&swb_id) {
                if *existing != key {
                    let message = format!(
                        "material {name} maps to {swb_id} like another MBF material, but E, nu or the expansion \
                         coefficient differ"
                    );
                    self.refuse("MBF-IMPORT-SIDECAR-CONFLICT", record.line, &format!("MATERIAL {name}"), &message);
                }
                continue;
            }
            self.claim(claims, &swb_id, &format!("MBF material {name}"), record.line);
            merged.insert(swb_id.clone(), key);
            let factor = if e_unit == "MPa" { 1_000_000.0 } else { PSI_PA };
            let e_pa = self.binary64_f(modulus.value * factor, record.line, &format!("MATERIAL {name}"), "E in Pa", false)?;
            let shear_exact = Q::from_f64(e_pa).expect("finite").div(&Q::from_int(1).add(&Q::from_f64(nu.value).expect("finite")).mul_int(2));
            let shear = self.binary64_q(&shear_exact, record.line, &format!("MATERIAL {name}"), "shear modulus", false)?;
            let note = format!(
                "G derived from E and nu (isotropic): G = E/(2(1+nu)) with E in Pa, from MBF MATERIAL {name} E = {} \
                 {e_unit} and nu = {}",
                modulus.text, nu.text
            );
            let certification = if recorded {
                if origin == "project_invented" { INVENTED_CERTIFICATION.to_string() } else { USER_SUPPLIED_CERTIFICATION.to_string() }
            } else {
                origin.clone()
            };
            let mut provenance_record = Dict::new();
            provenance_record.insert("source_name".into(), Value::Str(source));
            provenance_record.insert("source_location".into(), Value::Str(format!("sha256:{}", self.sha)));
            provenance_record.insert("contributor_certification".into(), Value::Str(certification));
            provenance_record.insert("verification_note".into(), Value::Str(note));
            if let Some(status) = status {
                provenance_record.insert("redistribution_status".into(), Value::Str(status));
            }
            let label = py_strip(&record.description).to_string();
            materials.push(dict![
                ("id", Value::str(swb_id.clone())),
                ("label", Value::Str(if label.is_empty() { name.clone() } else { label })),
                ("constitutive_basis", Value::str("homogeneous_isotropic_E_nu_v1")),
                ("elastic_modulus", dict![("value", Value::Float(modulus.value)), ("unit", Value::str(e_unit))]),
                ("poisson_ratio", dict![("value", Value::Float(nu.value)), ("unit", Value::str("1"))]),
                ("shear_modulus", dict![("value", Value::Float(shear)), ("unit", Value::str("Pa"))]),
                ("thermal_expansion_coefficient", dict![("value", Value::Float(alpha.value)), ("unit", Value::str(alpha_unit))]),
                ("provenance", Value::str(self.provenance(record.line))),
                ("provenance_record", Value::Dict(provenance_record)),
            ]);
        }
        if self.has_errors() {
            return Err(Stop::Refused);
        }
        Ok((materials, material_ids))
    }

    fn list_unused(&mut self, material_ids: &HashMap<String, String>) {
        let used_sections: HashSet<String> = self.build.elements.iter().map(|element| element.section.clone()).collect();
        let used_loads: HashSet<Option<String>> = self.build.elements.iter().map(|element| element.load.clone()).collect();
        let materials: Vec<(String, usize)> = self.materials.iter().map(|(name, record)| (name.clone(), record.line)).collect();
        for (name, line) in materials {
            if !material_ids.contains_key(&name) {
                let message = format!("MATERIAL {name} is not referenced by any element and is not imported");
                self.listed("omitted", "MBF-IMPORT-RECORD-UNUSED", line, &format!("MATERIAL {name}"), &message);
            }
        }
        let pipes: Vec<(String, usize)> = self.pipes.iter().map(|(name, record)| (name.clone(), record.line)).collect();
        for (name, line) in pipes {
            if !used_sections.contains(&name) {
                let message = format!("PIPE {name} is not referenced by any element and is not imported");
                self.listed("omitted", "MBF-IMPORT-RECORD-UNUSED", line, &format!("PIPE {name}"), &message);
            }
        }
        let loads: Vec<(String, usize)> = self.loads.iter().map(|(name, record)| (name.clone(), record.line)).collect();
        for (name, line) in loads {
            if !used_loads.contains(&Some(name.clone())) {
                let message = format!("LOADS {name} is not referenced by any element and is not imported");
                self.listed("omitted", "MBF-IMPORT-RECORD-UNUSED", line, &format!("LOADS {name}"), &message);
            }
        }
    }

    #[allow(clippy::type_complexity)]
    fn assemble_supports(
        &mut self,
        node_id: &OMap<String, String>,
        id_map: &mut Dict,
        claims: &mut Claims,
    ) -> R<(Vec<Value>, HashMap<u32, Vec<(String, Vec<String>)>>)> {
        let mut supports = Vec::new();
        let mut support_ids: HashMap<u32, Vec<(String, Vec<String>)>> = HashMap::new();
        self.build.support_lines = HashMap::new();
        let mut sidecar_by_node: OMap<String, Vec<(String, Value)>> = OMap::new();
        let support_map = self.map("support_map");
        for (swb_id, entry) in support_map.iter() {
            let node = entry.get_or_null("node").as_str().unwrap_or_default().to_string();
            sidecar_by_node.entry_or_insert_with(node, Vec::new).push((swb_id.clone(), entry.clone()));
        }
        let mut matched_sidecar: HashSet<String> = HashSet::new();
        let stiffness_unit = if self.si { "N/mm" } else { "lbf/in" };
        let n_per_m = if self.si { 1000.0 } else { LBF_PER_IN_N_PER_M };
        let mut map_out = Dict::new();
        for number in self.build.node_order.clone() {
            let Some(record) = self.build.node_data.get(&number).cloned() else { continue };
            let name = number.to_string();
            let mut imported: Vec<Imported> = Vec::new();
            let mut springs: OMap<String, (Num, usize)> = OMap::new();
            let flexible = !record.anchor.is_empty() && !record.trak.is_empty();
            if !record.anchor.is_empty() && !flexible {
                imported.push((format!("support:{name}"), DOFS.iter().map(|dof| dof.to_string()).collect(), Some("anchor".into()), record.anchor[0], "anchor".into()));
            }
            if flexible {
                let mut stiffnesses: Vec<(usize, Num)> = record.trak[0].0.iter().map(|(axis, value)| (*axis, value.clone())).collect();
                stiffnesses.sort_by_key(|(axis, _)| *axis);
                for (axis, stiffness) in stiffnesses {
                    if !stiffness.exact.is_zero() {
                        self.spring(&mut springs, &name, DOFS[axis], stiffness, record.anchor[0], "TRAK");
                    }
                }
            }
            let mut fixd: Vec<usize> = record.fixd.keys().copied().collect();
            fixd.sort();
            let mut fixr: Vec<usize> = record.fixr.keys().copied().collect();
            fixr.sort();
            let dofs: Vec<String> = fixd.iter().map(|axis| DOFS[*axis].to_string()).chain(fixr.iter().map(|axis| DOFS[3 + axis].to_string())).collect();
            if !dofs.is_empty() {
                let line = record.fixd.values().chain(record.fixr.values()).copied().min().expect("line");
                imported.push((format!("support:{name}-restraint"), dofs, None, line, "restraint".into()));
            }
            for item in &record.supports {
                let (axis, line) = (item.axis, item.line);
                match item.kind {
                    "guide" => {
                        let lateral: Vec<String> = (0..3).filter(|index| Some(*index) != axis).map(|index| DOFS[index].to_string()).collect();
                        match &item.stiffness {
                            None => self.add_support(&mut imported, &name, (format!("support:{name}-guide"), lateral, Some("guide".into()), line, "guide".into()), "guide"),
                            Some(stiffness) => {
                                for dof in &lateral {
                                    self.spring(&mut springs, &name, dof, stiffness.clone(), line, "a flexible guide (G with STIFF)");
                                }
                            }
                        }
                    }
                    "stop" => {
                        let axis = axis.expect("axis");
                        let letter = ["x", "y", "z"][axis];
                        match &item.stiffness {
                            None => self.add_support(
                                &mut imported,
                                &name,
                                (format!("support:{name}-stop-{letter}"), vec![DOFS[axis].to_string()], Some("line_stop".into()), line, "limit stop".into()),
                                &format!("limit stop along {letter}"),
                            ),
                            Some(stiffness) => self.spring(&mut springs, &name, DOFS[axis], stiffness.clone(), line, "a flexible limit stop (LS with STIFF)"),
                        }
                    }
                    _ => {
                        let axis = axis.expect("axis");
                        self.spring(&mut springs, &name, DOFS[axis], item.stiffness.clone().expect("stiffness"), line, "a skewed restraint (K)");
                    }
                }
            }
            let rigid: HashSet<String> = imported.iter().flat_map(|item| item.1.iter().cloned()).collect();
            for (dof, (_, line)) in springs.iter() {
                if rigid.contains(dof) {
                    let message = format!("a spring on {dof} at node {name}, which a rigid support there already restrains, is not read");
                    self.refuse("MBF-IMPORT-SUPPORT-UNSUPPORTED", *line, &format!("node {name}"), &message);
                }
            }
            for dof in DOFS {
                if let Some((_, line)) = springs.get(&dof.to_string()) {
                    imported.push((format!("support:{name}-spring-{}", dof.to_lowercase()), vec![dof.to_string()], Some(SPRING_FAMILY.into()), *line, "spring".into()));
                }
            }
            let tags = record.tag.clone();
            if imported.is_empty() {
                for (_, line) in &tags {
                    let message = format!("TAG at node {name}, which has no support, is not imported");
                    self.listed("omitted", "MBF-IMPORT-TAG-WITHOUT-SUPPORT", *line, &format!("node {name} TAG"), &message);
                }
                continue;
            }
            let line = imported[0].3;
            let restored = if self.bound || self.rebound { sidecar_by_node.get(&name).cloned().unwrap_or_default() } else { Vec::new() };
            let mut ids = Vec::new();
            if self.bound && restored.is_empty() {
                let message = format!("the bound sidecar records no support at node {name}, which the file supports");
                self.refuse("MBF-IMPORT-SIDECAR-CONFLICT", line, &format!("node {name}"), &message);
                continue;
            }
            let entries: Vec<(String, Vec<String>, Option<String>, String, usize)> = if !restored.is_empty() {
                let wanted: HashSet<String> =
                    imported.iter().filter(|item| item.2.as_deref() != Some(SPRING_FAMILY)).flat_map(|item| item.1.iter().cloned()).collect();
                let restrained_of = |entry: &Value| -> Vec<String> {
                    entry.get_or_null("restrained").as_list().map(|items| items.iter().filter_map(|item| item.as_str().map(String::from)).collect()).unwrap_or_default()
                };
                let given: HashSet<String> = restored
                    .iter()
                    .filter(|(_, entry)| entry.get("family").and_then(Value::as_str) != Some(SPRING_FAMILY))
                    .flat_map(|(_, entry)| restrained_of(entry))
                    .collect();
                let mut given_springs: Vec<String> = restored
                    .iter()
                    .filter(|(_, entry)| entry.get("family").and_then(Value::as_str) == Some(SPRING_FAMILY))
                    .flat_map(|(_, entry)| restrained_of(entry))
                    .collect();
                given_springs.sort();
                let mut spring_dofs: Vec<String> = springs.keys().cloned().collect();
                spring_dofs.sort();
                if wanted != given || given_springs != spring_dofs {
                    let message = format!(
                        "the sidecar's supports at node {name} restrain other DOFs, or carry springs on other DOFs, than the MBF"
                    );
                    self.refuse("MBF-IMPORT-SIDECAR-CONFLICT", line, &format!("node {name}"), &message);
                    continue;
                }
                let mut entries = Vec::new();
                for (swb_id, entry) in &restored {
                    let authored = entry.get("family_authored").and_then(Value::as_bool).unwrap_or(true);
                    let family = if authored { entry.get("family").and_then(Value::as_str).map(String::from) } else { None };
                    if !entry.has("family") {
                        let message = format!("the sidecar entry for {swb_id} carries no family, which is not recoverable");
                        self.listed("approximated", "MBF-IMPORT-ID-UNRECOVERABLE", line, &format!("support_map[{swb_id}]"), &message);
                    }
                    let label = match family.as_deref() {
                        Some(SPRING_FAMILY) => "spring",
                        Some("anchor") => "anchor",
                        _ => "restraint",
                    };
                    let restrained = restrained_of(entry);
                    let dofs: Vec<String> = DOFS.iter().filter(|dof| restrained.iter().any(|item| item == *dof)).map(|dof| dof.to_string()).collect();
                    entries.push((swb_id.clone(), dofs, family, label.to_string(), line));
                    let kind = if record.anchor.is_empty() { "restraint" } else { "anchor" };
                    if self.bound && !self.sidecar_current && entry.get_or_null("caepipe_kind").as_str() != Some(kind) {
                        let message = format!(
                            "the sidecar records {swb_id} as an {}, but the file writes node {name} as {}",
                            crate::sidecar::py_str(entry.get_or_null("caepipe_kind"))?,
                            if kind == "anchor" { "an anchor (KA)" } else { "a restraint" }
                        );
                        self.refuse("MBF-IMPORT-SIDECAR-CONFLICT", line, &format!("support_map[{swb_id}].caepipe_kind"), &message);
                    }
                    matched_sidecar.insert(swb_id.clone());
                    if self.rebound {
                        self.unconfirmed.push(swb_id.clone());
                    }
                }
                entries
            } else {
                imported.iter().map(|(swb_id, dofs, family, own, label)| (swb_id.clone(), dofs.clone(), family.clone(), label.clone(), *own)).collect()
            };
            for (swb_id, dofs, family, label, own) in entries {
                self.claim(claims, &swb_id, &format!("support at node {name}"), line);
                self.build.support_lines.insert(swb_id.clone(), own);
                let mut support = Dict::new();
                support.insert("id".into(), Value::str(swb_id.clone()));
                support.insert("label".into(), Value::Str(match tags.first() {
                    Some((tag, _)) => tag.clone(),
                    None => format!("{label} at {name}"),
                }));
                support.insert("node".into(), Value::str(node_id.get(&name).expect("node").clone()));
                support.insert("restraints".into(), Value::List(dofs.iter().map(|dof| Value::str(dof.clone())).collect()));
                support.insert("provenance".into(), Value::str(self.provenance(line)));
                if let Some(family) = &family {
                    support.insert("family".into(), Value::str(family.clone()));
                }
                if family.as_deref() == Some(SPRING_FAMILY) {
                    let (stiffness, spring_line) = springs.get(&dofs[0]).expect("spring").clone();
                    let in_n_per_m = stiffness.value * n_per_m;
                    if !(in_n_per_m.is_finite() && in_n_per_m > 0.0) {
                        let message = format!("the stiffness of {swb_id} is outside the binary64 range in N/m, as the solve reads it");
                        self.refuse("MBF-IMPORT-VALUE-UNREPRESENTABLE", spring_line, &swb_id, &message);
                    }
                    support.insert(
                        "stiffness".into(),
                        dict![("dof", Value::str(dofs[0].clone())), ("value", dict![("value", Value::Float(stiffness.value)), ("unit", Value::str(stiffness_unit))])],
                    );
                }
                supports.push(Value::Dict(support));
                ids.push((swb_id, dofs));
            }
            map_out.insert(name.clone(), Value::List(ids.iter().map(|(swb_id, _)| Value::str(swb_id.clone())).collect()));
            support_ids.insert(number, ids);
        }
        id_map.insert("support_map".into(), Value::Dict(map_out));
        if self.bound {
            let in_file: Vec<String> = support_map.keys().filter(|key| matched_sidecar.contains(*key)).cloned().collect();
            let in_sidecar: Vec<String> = support_map.keys().cloned().collect();
            self.incomplete("support_map", &in_file, &in_sidecar, "supports");
        }
        if self.rebound {
            for swb_id in support_map.keys() {
                if !matched_sidecar.contains(swb_id) {
                    let message = format!("sidecar support {swb_id} has no counterpart in this MBF");
                    self.listed("omitted", "MBF-IMPORT-SIDECAR-ENTRY-ABSENT", 0, &format!("support_map[{swb_id}]"), &message);
                }
            }
        }
        Ok((supports, support_ids))
    }

    fn add_support(&mut self, imported: &mut Vec<Imported>, name: &str, item: Imported, what: &str) {
        if imported.iter().any(|existing| existing.0 == item.0) {
            let message = format!("a second {what} at node {name} is not read");
            self.refuse("MBF-IMPORT-SUPPORT-UNSUPPORTED", item.3, &format!("node {name}"), &message);
        } else {
            imported.push(item);
        }
    }

    fn spring(&mut self, springs: &mut OMap<String, (Num, usize)>, name: &str, dof: &str, stiffness: Num, line: usize, what: &str) {
        if springs.contains_key(&dof.to_string()) {
            let message = format!("{what} adds a second spring on {dof} at node {name}, which is not read");
            self.refuse("MBF-IMPORT-SUPPORT-UNSUPPORTED", line, &format!("node {name}"), &message);
        } else {
            springs.insert(dof.to_string(), (stiffness, line));
        }
    }

    fn case_ids(&self) -> (String, String) {
        let mut sustained = SUSTAINED_CASE.to_string();
        let mut expansion = EXPANSION_CASE.to_string();
        if self.bound {
            for (swb_id, label) in self.map("load_case_map").iter() {
                if label.as_str() == Some(LABEL_SUSTAINED) {
                    sustained = swb_id.clone();
                }
                if label.as_str() == Some(LABEL_EXPANSION) {
                    expansion = swb_id.clone();
                }
            }
        }
        (sustained, expansion)
    }

    #[allow(clippy::too_many_arguments)]
    fn assemble_loads(
        &mut self,
        node_id: &OMap<String, String>,
        support_ids: &HashMap<u32, Vec<(String, Vec<String>)>>,
        weights: &[(String, f64, usize)],
        thermals: &[(String, f64, usize)],
        length_unit: &str,
        angle_unit: &str,
        id_map: &mut Dict,
        claims: &mut Claims,
        regions: Option<&Vec<Value>>,
    ) -> R<Vec<Value>> {
        let (sustained_id, expansion_id) = self.case_ids();
        let (force_unit, moment_unit) = if self.si { ("N", "N*m") } else { ("lbf", "lbf*ft") };
        let mut sustained: Vec<Primitive> = Vec::new();
        let mut expansion: Vec<Primitive> = Vec::new();
        for (pipe_id, weight, line) in weights {
            sustained.push(Primitive {
                category: "weight",
                target: dict![("type", Value::str("element")), ("pipe", Value::str(pipe_id.clone()))],
                target_id: pipe_id.clone(),
                target_kind: "pipe",
                direction: "global_y".into(),
                magnitude: *weight,
                unit: if self.si { "N/m" } else { "lbf/in" }.into(),
                dimension: "force_per_length",
                line: *line,
                axis: Axis::None,
            });
        }
        for number in self.build.node_order.clone() {
            let Some(record) = self.build.node_data.get(&number).cloned() else { continue };
            let name = number.to_string();
            for (kind, category, unit, dimension) in
                [("forces", "concentrated_force", force_unit, "force"), ("moments", "concentrated_moment", moment_unit, "moment")]
            {
                let store = if kind == "forces" { &record.forces } else { &record.moments };
                let mut axes: Vec<usize> = store.keys().copied().collect();
                axes.sort();
                for axis in axes {
                    let items = store.get(&axis).expect("axis");
                    let total = items.iter().fold(Q::zero(), |sum, (number, _)| sum.add(&number.exact));
                    let total = if total.is_zero() { 0.0 } else { self.binary64_q(&total, items[0].1, &format!("node {number}"), "summed load", false)? };
                    if total == 0.0 {
                        continue;
                    }
                    let direction = if kind == "forces" { ["global_x", "global_y", "global_z"][axis] } else { DOFS[3 + axis] };
                    let node = node_id.get(&name).expect("node").clone();
                    sustained.push(Primitive {
                        category,
                        target: dict![("type", Value::str("node")), ("node", Value::str(node.clone()))],
                        target_id: node,
                        target_kind: "node",
                        direction: direction.into(),
                        magnitude: total,
                        unit: unit.into(),
                        dimension,
                        line: items[0].1,
                        axis: Axis::Pair(if kind == "forces" { 't' } else { 'r' }, axis),
                    });
                }
            }
        }
        for (pipe_id, delta, line) in thermals {
            expansion.push(Primitive {
                category: "thermal",
                target: dict![("type", Value::str("element")), ("pipe", Value::str(pipe_id.clone()))],
                target_id: pipe_id.clone(),
                target_kind: "pipe",
                direction: "global_x".into(),
                magnitude: *delta,
                unit: if self.si { "degC" } else { "degF" }.into(),
                dimension: "temperature_interval",
                line: *line,
                axis: Axis::None,
            });
        }
        for number in self.build.node_order.clone() {
            let Some(record) = self.build.node_data.get(&number).cloned() else { continue };
            if record.anchor.is_empty() {
                continue;
            }
            for (kind, offset, unit, dimension) in [("disp", 0usize, length_unit, "displacement"), ("rot", 3, angle_unit, "rotation")] {
                let store = if kind == "disp" { &record.disp } else { &record.rot };
                let mut axes: Vec<usize> = store.keys().copied().collect();
                axes.sort();
                for axis in axes {
                    let items = store.get(&axis).expect("axis");
                    let total = items.iter().fold(Q::zero(), |sum, (number, _)| sum.add(&number.exact));
                    let total = if total.is_zero() { 0.0 } else { self.binary64_q(&total, items[0].1, &format!("node {number}"), "summed load", false)? };
                    if total == 0.0 {
                        continue;
                    }
                    let dof = DOFS[offset + axis];
                    let candidates: Vec<String> = support_ids
                        .get(&number)
                        .map(|ids| ids.iter().filter(|(_, dofs)| dofs.iter().any(|item| item == dof)).map(|(id, _)| id.clone()).collect())
                        .unwrap_or_default();
                    let mut target = candidates.first().cloned().ok_or_else(super::index_error)?;
                    if candidates.len() > 1 {
                        if let Some(found) = self.displacement_target(&number.to_string(), dof, &candidates) {
                            target = found;
                        }
                    }
                    let (kind_char, index) = direction_axis(dof).expect("dof");
                    expansion.push(Primitive {
                        category: "imposed_displacement",
                        target: dict![("type", Value::str("support")), ("support", Value::str(target.clone())), ("dof", Value::str(dof))],
                        target_id: target,
                        target_kind: "support",
                        direction: dof.into(),
                        magnitude: total,
                        unit: unit.into(),
                        dimension,
                        line: items[0].1,
                        axis: Axis::Pair(kind_char, index),
                    });
                }
            }
        }
        let restorable = self.restorable_primitives();
        let mut matched: Vec<PrimKey> = Vec::new();
        let mut load_cases = Vec::new();
        let mut ids_seen: HashMap<String, String> = HashMap::new();
        let regions_truthy = regions.map(|items| !items.is_empty()).unwrap_or(false);
        let case_map = self.map("load_case_map");
        for (kind, label, case_kind, primitives, case_target) in [
            ("sustained", LABEL_SUSTAINED, "sustained", &sustained, sustained_id.clone()),
            ("expansion", LABEL_EXPANSION, "thermal", &expansion, expansion_id.clone()),
        ] {
            if primitives.is_empty() && !(regions_truthy && kind == "sustained") {
                continue;
            }
            if self.bound && !case_map.values().any(|value| value.as_str() == Some(label)) {
                let message = format!("the bound sidecar records no case {}, which the file has", py_repr_str(label));
                self.refuse("MBF-IMPORT-SIDECAR-CONFLICT", 0, "load_case_map", &message);
            }
            let case_id = self.claim(claims, &case_target, &format!("CAEPIPE load case {label}"), 0);
            id_map.get_mut(&"load_case_map".to_string()).and_then(Value::as_dict_mut).expect("cases").insert(label.to_string(), Value::str(case_id.clone()));
            let mut loads = Vec::new();
            for item in primitives.iter() {
                let with_direction = matches!(item.category, "concentrated_force" | "concentrated_moment" | "imposed_displacement");
                let formed = format!("load:{}:{}{}", item.category, item.target_id, if with_direction { format!(":{}", item.direction) } else { String::new() });
                let mut load_id = formed.clone();
                let mut direction = item.direction.clone();
                let key = PrimKey {
                    case: case_id.clone(),
                    category: item.category.to_string(),
                    kind: item.target_kind.to_string(),
                    target: item.target_id.clone(),
                    axis: item.axis.clone(),
                };
                let found: Vec<PrimKey> = [key.clone(), key.with_star()].into_iter().filter(|option| restorable.contains_key(option)).collect();
                let matching: Option<Restorable> = found.first().map(|option| restorable.get(option).expect("restorable").clone());
                if let Some(first) = found.first() {
                    if !matched.contains(first) {
                        matched.push(first.clone());
                    }
                }
                if self.bound && matching.is_none() {
                    let message = format!("the bound sidecar's load_map records no primitive for {formed}, which the file has");
                    self.refuse("MBF-IMPORT-SIDECAR-CONFLICT", item.line, &formed, &message);
                }
                if let (true, Some(found)) = (self.bound, &matching) {
                    for (map_key, recorded) in &found.3 {
                        if let Some(recorded) = recorded {
                            if item.category != "thermal" && direction_axis(recorded) != direction_axis(&item.direction) {
                                let message = format!(
                                    "the sidecar records the direction {recorded} for {formed}, but the MBF gives {}",
                                    item.direction
                                );
                                self.refuse("MBF-IMPORT-SIDECAR-CONFLICT", item.line, &format!("load_map[{map_key}].direction"), &message);
                            }
                        }
                    }
                    if item.category == "thermal" {
                        direction = self.thermal_direction(&found.3, &formed, item.line);
                    }
                    if found.1 == 1 {
                        load_id = found.0.clone();
                    } else {
                        let keys: Vec<String> = found.3.iter().map(|(map_key, _)| map_key.clone()).collect();
                        let named = keys.iter().map(|key| format!("load_map[{key}]")).collect::<Vec<_>>().join(", ");
                        let message = format!(
                            "the MBF holds {named} summed as one primitive; no single sidecar id is its own, so it keeps \
                             its formed id {formed}"
                        );
                        self.listed("approximated", "MBF-IMPORT-ID-UNRECOVERABLE", item.line, &format!("load_map[{case_id}/{formed}]"), &message);
                        for map_key in &keys {
                            let message = format!(
                                "load_map[{map_key}] is one of the {} primitives the MBF sums into {formed}; its id is not recoverable",
                                keys.len()
                            );
                            self.listed("approximated", "MBF-IMPORT-ID-UNRECOVERABLE", item.line, &format!("load_map[{map_key}]"), &message);
                        }
                    }
                }
                let what = format!("the primitive {formed} of case {case_id}");
                if let Some(previous) = ids_seen.get(&load_id) {
                    let message = format!(
                        "{what} would take id {load_id}, which {previous} already has; primitive ids must be unique (DUPLICATE_ID)"
                    );
                    self.refuse("MBF-IMPORT-SIDECAR-CONFLICT", item.line, &format!("load_map[{case_id}/{load_id}]"), &message);
                    continue;
                }
                ids_seen.insert(load_id.clone(), what);
                let mut entry = Dict::new();
                entry.insert("case".into(), Value::str(case_id.clone()));
                entry.insert("category".into(), Value::str(item.category));
                entry.insert("target".into(), Value::str(item.target_id.clone()));
                if let (true, Some(found)) = (self.bound, &matching) {
                    if found.1 > 1 {
                        entry.insert("summed_entries".into(), Value::List(found.3.iter().map(|(map_key, _)| Value::str(map_key.clone())).collect()));
                    }
                }
                id_map.get_mut(&"load_map".to_string()).and_then(Value::as_dict_mut).expect("loads").insert(load_id.clone(), Value::Dict(entry));
                loads.push(dict![
                    ("id", Value::str(load_id)),
                    ("category", Value::str(item.category)),
                    ("target", item.target.clone()),
                    ("direction", Value::str(direction)),
                    ("magnitude", dict![("value", Value::Float(item.magnitude)), ("unit", Value::str(item.unit.clone()))]),
                    ("dimension", Value::str(item.dimension)),
                    ("provenance", Value::str(self.provenance(item.line))),
                ]);
            }
            let mut case = Dict::new();
            case.insert("id".into(), Value::str(case_id.clone()));
            case.insert("label".into(), Value::str(label));
            case.insert("kind".into(), Value::str(case_kind));
            case.insert("status".into(), Value::str("draft"));
            case.insert("primitive_loads".into(), Value::List(loads));
            case.insert("provenance".into(), Value::Str(format!("imported from MBF sha256:{}; CAEPIPE load case {label}", self.sha)));
            if let Some(regions) = regions {
                case.insert("pressure_regions".into(), Value::List(if kind == "sustained" { regions.clone() } else { Vec::new() }));
            }
            load_cases.push(Value::Dict(case));
        }
        if self.bound {
            let mut in_file: Vec<String> = matched.iter().map(PrimKey::repr).collect();
            in_file.sort();
            let mut in_sidecar: Vec<String> = restorable.keys().map(PrimKey::repr).collect();
            in_sidecar.sort();
            self.incomplete("load_map", &in_file, &in_sidecar, "primitives");
            let labels: Vec<String> = load_cases.iter().map(|case| case.get_or_null("label").as_str().expect("label").to_string()).collect();
            let recorded: Vec<String> = case_map.values().map(|value| value.as_str().unwrap_or_default().to_string()).collect();
            self.incomplete("load_case_map", &labels, &recorded, "load cases");
        }
        Ok(load_cases)
    }

    pub(crate) fn restorable_primitives(&self) -> OMap<PrimKey, Restorable> {
        let mut counts: OMap<PrimKey, Vec<(String, Option<String>, String)>> = OMap::new();
        if !self.bound {
            return OMap::new();
        }
        for (key, entry) in self.map("load_map").iter() {
            let case_length = entry.get_or_null("load_case").as_str().unwrap_or_default().chars().count();
            let load_id = skip_chars(key, case_length + 1).to_string();
            let direction = entry.get("direction").and_then(Value::as_str).map(String::from);
            counts.entry_or_insert_with(primitive_key(entry), Vec::new).push((load_id, direction, key.clone()));
        }
        let mut out = OMap::new();
        for (key, items) in counts.iter() {
            out.insert(
                key.clone(),
                (items[0].0.clone(), items.len(), items[0].1.clone(), items.iter().map(|item| (item.2.clone(), item.1.clone())).collect()),
            );
        }
        out
    }
}

/// `x ** 2` for a float: None where Python raises OverflowError.
pub(crate) fn py_square(value: f64) -> Option<f64> {
    let square = value * value;
    if square.is_infinite() && value.is_finite() {
        None
    } else {
        Some(square)
    }
}

fn xyz(vector: &V3) -> Value {
    dict![("x", Value::Float(vector[0])), ("y", Value::Float(vector[1])), ("z", Value::Float(vector[2]))]
}

fn component(vector: &Value, axis: &str) -> f64 {
    vector.get_or_null(axis).as_f64().unwrap_or(f64::NAN)
}
