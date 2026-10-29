//! importer.py §9.4: exact pressure regions from LOADS Pressure 1.

use super::assemble::Claims;
use super::{Element, Importer, Num, Stop, R};
use crate::dict;
use crate::pyfmt::{py_float_repr, py_format_g};
use crate::pymath::{atan2, correct_norm3};
use crate::pystr::PyError;
use crate::rational::Q;
use crate::runtime::{bore_radius_m, bores_equal, collinear, kink, sub, unit, AXIS_TOLERANCE, TANGENCY_TOLERANCE_RAD, V3};
use crate::sidecar::CLOSURE_TRANSFERS;
use crate::value::{str_in, Dict, OMap, Value};
use std::collections::{BTreeSet, HashMap, HashSet};

const PSI_PA: f64 = 4.448_221_615_260_5 / 0.000_645_16;
const PRESSURE_BASIS: &str = "internal_differential_zero_external_v1";

pub(crate) type Region = (String, Vec<String>, Vec<Value>);
type Junction = (String, Option<f64>);
type Failures = (Vec<Vec<String>>, bool, Vec<Junction>);

/// The context the route checks read (`route_checks`).
struct Route<'r> {
    ends: &'r OMap<String, (String, String)>,
    arcs: &'r HashSet<String>,
    point_of: &'r HashMap<String, V3>,
    name_of: &'r HashMap<String, String>,
    pressures: &'r OMap<String, (Option<Num>, Element)>,
    factor: f64,
}

impl<'r> Route<'r> {
    fn far(&self, pipe_id: &str, node: &str) -> String {
        let (a, b) = self.ends.get(&pipe_id.to_string()).expect("pipe");
        if node == a {
            b.clone()
        } else {
            a.clone()
        }
    }

    fn runs(&self, members: &[String]) -> Vec<(usize, usize)> {
        let mut found = Vec::new();
        let mut index = 0;
        while index < members.len() {
            if self.arcs.contains(&members[index]) {
                index += 1;
                continue;
            }
            let mut end = index;
            while end < members.len() && !self.arcs.contains(&members[end]) {
                end += 1;
            }
            found.push((index, end));
            index = end;
        }
        found
    }

    fn metres(&self, node: &str) -> V3 {
        let point = self.point_of[&self.name_of[node]];
        [point[0] * self.factor + 0.0, point[1] * self.factor + 0.0, point[2] * self.factor + 0.0]
    }

    fn collinear(&self, nodes: &[String]) -> bool {
        let points: Vec<V3> = nodes.iter().map(|node| self.metres(node)).collect();
        collinear(&points)
    }
}

impl<'a> Importer<'a> {
    fn bore_m(&self, route: &Route, pipe_id: &str) -> f64 {
        let section = self.pipes.get(&route.pressures.get(&pipe_id.to_string()).expect("pipe").1.section).expect("section");
        bore_radius_m(section.od.as_ref().expect("od").value, section.thk.as_ref().expect("thk").value, route.factor)
    }

    fn directions(&self, route: &Route, pipe_id: &str, node: &str) -> Option<[V3; 2]> {
        let pair = if route.arcs.contains(pipe_id) {
            self.build.arc_tangents.get(pipe_id).cloned().flatten()
        } else {
            self.build.frame_x.get(pipe_id).cloned().flatten().map(|axis| (axis, axis))
        }?;
        if node == route.ends.get(&pipe_id.to_string()).expect("pipe").0 {
            Some([pair.0, pair.1])
        } else {
            Some([[-pair.1[0], -pair.1[1], -pair.1[2]], [-pair.0[0], -pair.0[1], -pair.0[2]]])
        }
    }

    fn junctions(&self, route: &Route, members: &[String], nodes: &[String]) -> Vec<Junction> {
        let mut found: Vec<Junction> = members
            .iter()
            .filter(|member| {
                (route.arcs.contains(*member) && self.build.arc_tangents.get(*member).cloned().flatten().is_none())
                    || (self.build.frame_x.get(*member).cloned().flatten().is_none()
                        && self.build.length_m.get(*member).copied().unwrap_or(0.0) > AXIS_TOLERANCE)
            })
            .map(|member| (format!("element {}", self.build.labels[member]), None))
            .collect();
        for index in 0..members.len().saturating_sub(1) {
            let (before, after) = (&members[index], &members[index + 1]);
            if !route.arcs.contains(before) && !route.arcs.contains(after) {
                continue;
            }
            let incoming = self.directions(route, before, &nodes[index]);
            let outgoing = self.directions(route, after, &nodes[index + 1]);
            let theta = match (incoming, outgoing) {
                (Some(incoming), Some(outgoing)) => kink(&incoming[1], &outgoing[0]),
                _ => None,
            };
            if theta.map(|theta| !(theta <= TANGENCY_TOLERANCE_RAD)).unwrap_or(true) {
                found.push((format!("node {}", route.name_of[&nodes[index + 1]]), theta));
            }
        }
        found
    }

    fn failures(&self, route: &Route, members: &[String], start: &str) -> Failures {
        let mut nodes = vec![start.to_string()];
        for member in members {
            let next = route.far(member, nodes.last().expect("node"));
            nodes.push(next);
        }
        let bad: Vec<Vec<String>> = route
            .runs(members)
            .into_iter()
            .filter(|(low, high)| !route.collinear(&nodes[*low..*high + 1]))
            .map(|(low, high)| members[low..high].to_vec())
            .collect();
        let bores: Vec<f64> = members.iter().map(|member| self.bore_m(route, member)).collect();
        (bad, !bores_equal(&bores), self.junctions(route, members, &nodes))
    }

    pub(crate) fn assemble_pressure(
        &mut self,
        pipes: &[Value],
        pressures: &OMap<String, (Option<Num>, Element)>,
        arcs: &HashSet<String>,
        point_of: &HashMap<String, V3>,
        id_map: &mut Dict,
        claims: &mut Claims,
    ) -> R<Option<(Value, Vec<Value>)>> {
        let mut active: OMap<String, Num> = OMap::new();
        for (pipe_id, (number, _)) in pressures.iter() {
            if let Some(number) = number {
                if !number.exact.is_zero() {
                    active.insert(pipe_id.clone(), number.clone());
                }
            }
        }
        let stored = if self.bound { self.maps.get("pressure_map").filter(|value| value.truthy()).cloned() } else { None };
        if self.bound && !active.is_empty() && stored.is_none() {
            self.refuse(
                "MBF-IMPORT-SIDECAR-CONFLICT",
                self.loads_line(),
                "pressure_map",
                "the file has Pressure 1, but the bound sidecar records no pressure_map (contract, shear model and \
                 regions); a bound import restores them and never forms regions or defaults the shear model",
            );
            return Err(Stop::Refused);
        }
        if active.is_empty() && stored.is_none() {
            return Ok(None);
        }
        let to_pa = if self.si { 100_000.0 } else { PSI_PA };
        let mut names: Vec<String> = Vec::new();
        for pipe_id in active.keys() {
            let load = pressures.get(pipe_id).expect("pipe").1.load.clone().expect("load");
            if !names.contains(&load) {
                names.push(load);
            }
        }
        for name in &names {
            let record = self.loads.get(name).expect("load").clone();
            if !(record.p1.as_ref().expect("p1").value * to_pa + 0.0).is_finite() {
                let message = format!(
                    "Pressure 1 of LOADS {name} is finite in {} but beyond binary64 in Pa, the unit the solve reads",
                    if self.si { "bar" } else { "psi" }
                );
                self.refuse("MBF-IMPORT-VALUE-UNREPRESENTABLE", record.line, &format!("LOADS {name} Pressure 1"), &message);
            }
        }
        if self.has_errors() {
            return Err(Stop::Refused);
        }
        let caller_shear = self.options.shear_model.as_ref().and_then(Value::as_str).map(String::from);
        let mut shear_model = "timoshenko_annulus_energy_v1".to_string();
        if let Some(stored) = &stored {
            shear_model = stored.get_or_null("contract").get_or_null("shear_model").as_str().unwrap_or_default().to_string();
            if let Some(caller) = &caller_shear {
                if *caller != shear_model {
                    let message = format!(
                        "the shear_model argument {caller} differs from the bound sidecar's {shear_model}, which a bound \
                         import keeps"
                    );
                    self.refuse("MBF-IMPORT-SIDECAR-CONFLICT", 0, "shear_model", &message);
                    return Err(Stop::Refused);
                }
            }
        } else if let Some(caller) = &caller_shear {
            shear_model = caller.clone();
        }
        let contract = dict![("version", Value::str("3.0.0")), ("mode", Value::str("exact_pressure_v3")), ("shear_model", Value::str(shear_model.clone()))];
        if !active.is_empty() {
            self.pressure_effects()?;
        }
        let mut ends: OMap<String, (String, String)> = OMap::new();
        for pipe in pipes {
            ends.insert(
                pipe.get_or_null("id").as_str().expect("id").to_string(),
                (pipe.get_or_null("from").as_str().expect("from").to_string(), pipe.get_or_null("to").as_str().expect("to").to_string()),
            );
        }
        let mut name_of: HashMap<String, String> = HashMap::new();
        if let Some(nodes) = id_map.get(&"node_map".to_string()).and_then(Value::as_dict) {
            for (name, swb_id) in nodes.iter() {
                name_of.insert(swb_id.as_str().expect("id").to_string(), name.clone());
            }
        }
        let labels: HashMap<String, String> = pipes
            .iter()
            .map(|pipe| (pipe.get_or_null("id").as_str().expect("id").to_string(), pipe.get_or_null("label").as_str().expect("label").to_string()))
            .collect();
        let unit_name = if self.si { "bar" } else { "psi" };
        let route = Route { ends: &ends, arcs, point_of, name_of: &name_of, pressures, factor: if self.si { 0.001 } else { 0.0254 } };
        let (members, how) = if let Some(stored) = &stored {
            let regions = stored.get_or_null("regions").as_list().cloned().unwrap_or_default();
            let mut members = self.restore_regions(&regions, &active, &route);
            if let Some(found) = members {
                members = self.check_restored(found, &route, &labels);
            }
            (
                members,
                "restored from the sidecar's pressure_map (bound import), each checked whole as the exact route checks \
                 it in its declared order",
            )
        } else {
            let members = self.form_regions(&active, &route, &labels)?;
            let shear_note = if caller_shear.is_some() {
                format!("the caller's {shear_model}")
            } else {
                "timoshenko_annulus_energy_v1, the exact contract's default (owner D-6: Timoshenko with the energy-matched \
                 annular shear factor), as the MBF states none"
                    .to_string()
            };
            self.pressure_notes.push(format!(
                "Shear model: {shear_note}; realized curved bends stay Euler-Bernoulli under either model, as the route \
                 discloses (Agent 0's decision under the owner's delegation, 2026-10-27)."
            ));
            (
                members,
                "formed as maximal chains of elements with equal Pressure 1 and bore, ended at tees, where pressure or \
                 bore changes, and at straight-to-straight corners of a run the exact route's collinearity check refuses \
                 whole; closure transfer at each real end (free end, tee, pressure or bore change) is the caller's \
                 declaration (pressure_closure), never inferred from the file; a corner terminal takes \
                 transfers_to_wall, deduced from geometry: the pipe wall is continuous there, so it carries the \
                 corner's pressure force pA(t_in - t_out), unless pressure_closure.nodes names the corner (Agent 0's \
                 decision under the owner's delegation, 2026-10-27)",
            )
        };
        let Some(members) = members else { return Err(Stop::Refused) };
        let mut regions = Vec::new();
        let mut region_map = Dict::new();
        let mut element_name: HashMap<String, String> = HashMap::new();
        if let Some(elements) = id_map.get(&"element_map".to_string()).and_then(Value::as_dict) {
            for (name, swb_id) in elements.iter() {
                element_name.insert(swb_id.as_str().expect("id").to_string(), name.clone());
            }
        }
        for (region_id, member_ids, terminals) in members {
            let first = pressures.get(&member_ids[0]).expect("pipe").1.clone();
            let number = active.get(&member_ids[0]).expect("active").clone();
            self.claim(claims, &region_id, &format!("pressure region of {}", element_name[&member_ids[0]]), first.line);
            let load = first.load.clone().expect("load");
            let load_line = self.loads.get(&load).expect("load").line;
            regions.push(dict![
                ("id", Value::str(region_id.clone())),
                ("member_pipe_ids", Value::List(member_ids.iter().map(|item| Value::str(item.clone())).collect())),
                ("pressure_basis", Value::str(PRESSURE_BASIS)),
                ("pressure", dict![("value", Value::Float(number.value)), ("unit", Value::str(unit_name))]),
                ("terminals", Value::List(terminals)),
                ("provenance", Value::Str(format!("imported from MBF sha256:{}, LOADS {load} line {load_line} (Pressure 1)", self.sha))),
            ]);
            region_map.insert(region_id, Value::List(member_ids.iter().map(|item| Value::str(element_name[item].clone())).collect()));
        }
        if !active.is_empty() {
            id_map.insert("region_map".into(), Value::Dict(region_map));
        }
        self.pressure_regions = Some(regions.len());
        self.pressure_notes.push(format!(
            "Pressure 1 is kept in {unit_name} as exact pressure regions ({}) under the contract 3.0.0/exact_pressure_v3 \
             with shear_model {shear_model}; regions are {how}. Pressure acts in the sustained case; the expansion case \
             states an empty region list. Each section's corrosion allowance is the 0 its PIPE record states in field \
             4; a section that omits it is refused.",
            regions.len()
        ));
        Ok(Some((contract, regions)))
    }

    fn pressure_effects(&mut self) -> R<()> {
        let Some((line, fields, defaults)) = self.oppres.clone() else {
            self.listed(
                "omitted",
                "MBF-IMPORT-PRESSURE-EFFECTS-OMITTED",
                self.loads_line(),
                "OPPRES",
                "the file has pressure and no OPPRES section, so it does not state CAEPIPE's Bourdon effect or pressure \
                 correction for bends (a blank OPPRES line turns both on); SWBPIPE's exact pressure route models neither \
                 (DEL-04-07 D-3)",
            );
            return Ok(());
        };
        let mut on = Vec::new();
        for (index, effect) in [(1usize, "Bourdon effect"), (2, "pressure correction for bends")] {
            let text = fields.get(index).cloned().unwrap_or_default();
            if text.is_empty() {
                on.push(format!("{effect} (unstated)"));
                continue;
            }
            let number = self.real(&text, line, &format!("OPPRES field {} ({effect})", index + 1))?;
            if let Some(number) = number {
                if number.exact != Q::zero() && number.exact != Q::from_int(1) {
                    self.refuse("MBF-IMPORT-FIELD-UNDOCUMENTED", line, &format!("OPPRES field {}", index + 1), &format!("OPPRES {effect} is 0 (off) or 1 (on)"));
                } else if number.exact == Q::from_int(1) {
                    on.push(format!("{effect} (on{})", if defaults { " by the blank-line default" } else { "" }));
                }
            }
        }
        if !on.is_empty() {
            let message = format!(
                "OPPRES gives {}; SWBPIPE's exact pressure route models neither the Bourdon effect nor pressure \
                 stiffening of bends (DEL-04-07 D-3)",
                on.join(", ")
            );
            self.listed("omitted", "MBF-IMPORT-PRESSURE-EFFECTS-OMITTED", line, "OPPRES", &message);
        }
        Ok(())
    }

    fn check_restored(&mut self, restored: Vec<Region>, route: &Route, labels: &HashMap<String, String>) -> Option<Vec<Region>> {
        let line = self.loads_line();
        for (region_id, members, terminals) in &restored {
            let start = terminals[0].get_or_null("node_ref").as_str().expect("node").to_string();
            let finish = terminals[1].get_or_null("node_ref").as_str().expect("node").to_string();
            let mut ordered: Vec<String> = Vec::new();
            let mut remaining: BTreeSet<String> = members.iter().cloned().collect();
            let mut node = start.clone();
            while !remaining.is_empty() {
                let following: Vec<String> = remaining
                    .iter()
                    .filter(|pipe_id| {
                        let (a, b) = route.ends.get(*pipe_id).expect("pipe");
                        *a == node || *b == node
                    })
                    .cloned()
                    .collect();
                if following.len() != 1 {
                    break;
                }
                ordered.push(following[0].clone());
                remaining.remove(&following[0]);
                node = route.far(&following[0], &node);
            }
            let location = format!("pressure_map region {region_id}");
            if !remaining.is_empty() || node != finish {
                let message = format!("the sidecar's region {region_id} is not one chain from its first terminal to its second");
                self.refuse("MBF-IMPORT-SIDECAR-CONFLICT", line, &location, &message);
                continue;
            }
            let mut inner: HashSet<String> = HashSet::new();
            let mut node = start.clone();
            for pipe_id in &ordered[..ordered.len().saturating_sub(1)] {
                node = route.far(pipe_id, &node);
                inner.insert(node.clone());
            }
            let mut branching: Vec<String> = inner
                .iter()
                .filter(|node| route.ends.iter().any(|(pipe_id, (a, b))| (a == *node || b == *node) && !members.contains(pipe_id)))
                .map(|node| route.name_of[node].clone())
                .collect();
            branching.sort();
            if !branching.is_empty() {
                let message = format!(
                    "region {region_id} as the sidecar declares it passes through {}, where another pipe joins; the exact \
                     route admits no internal branch (PRESSURE_REGION_INTERNAL_BRANCH), and the region is not cut",
                    branching.join(", ")
                );
                self.refuse("MBF-IMPORT-PRESSURE-INTERNAL-BRANCH", line, &location, &message);
                continue;
            }
            let (runs, bores, bends) = self.failures(route, &ordered, &start);
            self.refuse_junctions(&bends, line, &location, &format!("region {region_id} as the sidecar declares it"), "the region is not cut or reordered");
            for run in &runs {
                let names = run.iter().map(|pipe_id| labels[pipe_id].clone()).collect::<Vec<_>>().join(", ");
                let message = format!(
                    "region {region_id} as the sidecar declares it (from {}): its straight run {names} fails the exact \
                     route's check (64 epsilon of the run's span) on the MBF's values; the region is not cut or reordered",
                    route.name_of[&start]
                );
                self.refuse("MBF-IMPORT-PRESSURE-RUN-NONCOLLINEAR", line, &location, &message);
            }
            if bores {
                let message = format!(
                    "region {region_id} as the sidecar declares it (from {}): its bores in metres differ by more than \
                     the exact route's guard (64 epsilon of the radius) on the MBF's values; the region is not cut or \
                     reordered",
                    route.name_of[&start]
                );
                self.refuse("MBF-IMPORT-PRESSURE-BORE-MISMATCH", line, &location, &message);
            }
        }
        if self.has_errors() {
            None
        } else {
            Some(restored)
        }
    }

    fn form_regions(&mut self, active: &OMap<String, Num>, route: &Route, labels: &HashMap<String, String>) -> R<Option<Vec<Region>>> {
        let mut incident: HashMap<String, Vec<String>> = HashMap::new();
        for (pipe_id, (a, b)) in route.ends.iter() {
            incident.entry(a.clone()).or_default().push(pipe_id.clone());
            incident.entry(b.clone()).or_default().push(pipe_id.clone());
        }
        let order: HashMap<String, usize> = route.ends.keys().enumerate().map(|(index, pipe_id)| (pipe_id.clone(), index)).collect();
        let bore = |pipe_id: &str| -> Q {
            let section = self.pipes.get(&route.pressures.get(&pipe_id.to_string()).expect("pipe").1.section).expect("section");
            section.od.as_ref().expect("od").exact.sub(&section.thk.as_ref().expect("thk").exact.mul_int(2))
        };
        let continues = |pipe_id: &str, node: &str| -> Option<String> {
            let around = &incident[node];
            if around.len() != 2 {
                return None;
            }
            let other = around.iter().find(|item| item.as_str() != pipe_id)?.clone();
            let here = active.get(&pipe_id.to_string()).expect("active");
            match active.get(&other) {
                Some(there) if there.exact == here.exact && bore(&other) == bore(pipe_id) => Some(other),
                _ => None,
            }
        };
        let corner = |before: &str, node: &str, after: &str| -> Result<bool, PyError> {
            let a = unit(&sub(&route.metres(node), &route.metres(before)));
            let t = unit(&sub(&route.metres(after), &route.metres(node)));
            let difference = sub(&a, &t);
            let theta = 2.0
                * atan2(
                    correct_norm3(difference[0], difference[1], difference[2]),
                    correct_norm3(a[0] + t[0], a[1] + t[1], a[2] + t[2]),
                )?;
            Ok(theta > TANGENCY_TOLERANCE_RAD)
        };
        let mut chains: Vec<(Vec<String>, String, String)> = Vec::new();
        let mut seen: HashSet<String> = HashSet::new();
        let mut corners: HashSet<String> = HashSet::new();
        let mut sorted_active: Vec<String> = active.keys().cloned().collect();
        sorted_active.sort_by_key(|pipe_id| order[pipe_id]);
        for pipe_id in sorted_active {
            if seen.contains(&pipe_id) {
                continue;
            }
            seen.insert(pipe_id.clone());
            let mut sides: Vec<(Vec<String>, String)> = Vec::new();
            let (from, to) = route.ends.get(&pipe_id).expect("pipe").clone();
            for start_node in [from, to] {
                let mut walked = Vec::new();
                let mut current = pipe_id.clone();
                let mut node = start_node;
                while let Some(following) = continues(&current, &node) {
                    if seen.contains(&following) {
                        break;
                    }
                    walked.push(following.clone());
                    seen.insert(following.clone());
                    node = route.far(&following, &node);
                    current = following;
                }
                sides.push((walked, node));
            }
            let (back, mut start) = sides[0].clone();
            let (ahead, mut end) = sides[1].clone();
            let mut members: Vec<String> = back.iter().rev().cloned().collect();
            members.push(pipe_id.clone());
            members.extend(ahead);
            if order[members.last().expect("member")] < order[&members[0]] {
                members.reverse();
                std::mem::swap(&mut start, &mut end);
            }
            // split(members, start)
            let mut nodes = vec![start.clone()];
            for member in &members {
                let next = route.far(member, nodes.last().expect("node"));
                nodes.push(next);
            }
            let mut cuts: HashSet<usize> = HashSet::new();
            for (low, high) in route.runs(&members) {
                let stretch: Vec<String> = nodes[low..high + 1].to_vec();
                let reversed: Vec<String> = stretch.iter().rev().cloned().collect();
                if !route.collinear(&stretch) && !route.collinear(&reversed) {
                    for k in low + 1..high {
                        if corner(&nodes[k - 1], &nodes[k], &nodes[k + 1])? {
                            cuts.insert(k);
                        }
                    }
                }
            }
            for index in &cuts {
                corners.insert(nodes[*index].clone());
            }
            let mut current: Vec<String> = Vec::new();
            let mut piece_start = start.clone();
            for (index, member) in members.iter().enumerate() {
                if cuts.contains(&index) {
                    chains.push((current.clone(), piece_start.clone(), nodes[index].clone()));
                    current = Vec::new();
                    piece_start = nodes[index].clone();
                }
                current.push(member.clone());
            }
            chains.push((current, piece_start, nodes.last().expect("node").clone()));
        }
        let mut admitted = Vec::new();
        let mut refused = false;
        let names = |items: &[String]| items.iter().map(|pipe_id| labels[pipe_id].clone()).collect::<Vec<_>>().join(", ");
        for (members, start, end) in chains {
            let forward = self.failures(route, &members, &start);
            if forward.0.is_empty() && !forward.1 && forward.2.is_empty() {
                admitted.push((members, start, end));
                continue;
            }
            let reversed: Vec<String> = members.iter().rev().cloned().collect();
            let backward = self.failures(route, &reversed, &end);
            if backward.0.is_empty() && !backward.1 && backward.2.is_empty() {
                admitted.push((reversed, end, start));
                continue;
            }
            refused = true;
            let line = route.pressures.get(&members[0]).expect("pipe").1.line;
            self.refuse_junctions(&forward.2, line, &format!("elements {}", names(&members)), &format!("the region {}", names(&members)), "a region is never cut");
            let backward_runs: Vec<BTreeSet<String>> = backward.0.iter().map(|run| run.iter().cloned().collect()).collect();
            let both: Vec<Vec<String>> =
                forward.0.iter().filter(|run| backward_runs.contains(&run.iter().cloned().collect::<BTreeSet<String>>())).cloned().collect();
            for run in &both {
                let message = format!(
                    "the straight run {} carries one pressure but the exact route's check (64 epsilon of the run's span) \
                     refuses it in both traversal orders, and it has no corner (a direction change above {} rad) to end a \
                     region at; a region is never cut inside a straight run",
                    names(run),
                    py_float_repr(TANGENCY_TOLERANCE_RAD)?
                );
                self.refuse("MBF-IMPORT-PRESSURE-RUN-NONCOLLINEAR", line, &format!("elements {}", names(run)), &message);
            }
            if forward.1 && backward.1 {
                let message = format!(
                    "the region {} has equal bores in the file's decimals, but in metres they differ by more than the \
                     exact route's guard (64 epsilon of the radius) in both traversal orders; a region is never cut to \
                     separate them",
                    names(&members)
                );
                self.refuse("MBF-IMPORT-PRESSURE-BORE-MISMATCH", line, &format!("elements {}", names(&members)), &message);
            }
            if both.is_empty() && !(forward.1 && backward.1) && forward.2.is_empty() {
                let checks = |result: &Failures| -> String {
                    let mut parts: Vec<String> = result.0.iter().map(|run| format!("the straight-run check on {}", names(run))).collect();
                    if result.1 {
                        parts.push("the bore guard".into());
                    }
                    parts.join(", ")
                };
                let message = format!(
                    "the region {} is admitted in neither traversal order: in file order {} refuses it, reversed {}",
                    names(&members),
                    checks(&forward),
                    checks(&backward)
                );
                self.refuse("MBF-IMPORT-PRESSURE-RUN-NONCOLLINEAR", line, &format!("elements {}", names(&members)), &message);
            }
        }
        if refused {
            return Ok(None);
        }
        let Some((default, provenance, by_node)) = self.closure_declaration() else { return Ok(None) };
        let terminal_names: HashSet<String> =
            admitted.iter().flat_map(|(_, start, end)| [route.name_of[start].clone(), route.name_of[end].clone()]).collect();
        let mut unknown: Vec<String> = by_node.keys().filter(|name| !terminal_names.contains(*name)).cloned().collect();
        unknown.sort();
        if !unknown.is_empty() {
            let message = format!("the closure declaration names {}, which is not a pressure-region terminal", unknown.join(", "));
            self.refuse("MBF-IMPORT-PRESSURE-CLOSURE-INVALID", 0, "pressure_closure.nodes", &message);
            return Ok(None);
        }
        let wall = "closure transfer deduced from geometry at import, not inferred from the file: the pipe wall is \
                    continuous through this straight-to-straight corner, so it carries the corner's pressure force \
                    pA(t_in - t_out)";
        let mut formed = Vec::new();
        for (members, start, end) in admitted {
            let terminals: Vec<Value> = [&start, &end]
                .iter()
                .map(|node| {
                    let name = &route.name_of[*node];
                    if by_node.contains_key(name) || !corners.contains(*node) {
                        dict![
                            ("node_ref", Value::str((*node).clone())),
                            ("closure_transfer", Value::str(by_node.get(name).cloned().unwrap_or_else(|| default.clone()))),
                            ("provenance", Value::Str(format!("closure transfer declared at import: {provenance}"))),
                        ]
                    } else {
                        dict![
                            ("node_ref", Value::str((*node).clone())),
                            ("closure_transfer", Value::str("transfers_to_wall")),
                            ("provenance", Value::str(wall)),
                        ]
                    }
                })
                .collect();
            formed.push((format!("region:mbf-{}-{}", route.name_of[&start], route.name_of[&end]), members, terminals));
        }
        Ok(Some(formed))
    }

    fn refuse_junctions(&mut self, found: &[Junction], line: usize, location: &str, subject: &str, never: &str) {
        for (place, theta) in found {
            match theta {
                None => {
                    let message = format!(
                        "{subject}: at {place} the runtime forms no bend tangent, direction change or frame orientation \
                         on the MBF's values in m (PRESSURE_REGION_GEOMETRY_INVALID); {never}"
                    );
                    self.refuse("MBF-IMPORT-PRESSURE-GEOMETRY-INVALID", line, location, &message);
                }
                Some(theta) => {
                    let message = format!(
                        "{subject}: at {place} the bend's end tangent, as the runtime forms the arc on the MBF's \
                         positions and radius in m, turns {} rad from its neighbour, above the tangency tolerance 0.001 \
                         rad: the exact route refuses it as a mitre (PRESSURE_REGION_MITRE_UNSUPPORTED); {never}",
                        py_format_g(*theta, 6)
                    );
                    self.refuse("MBF-IMPORT-PRESSURE-MITRE", line, location, &message);
                }
            }
        }
    }

    fn closure_declaration(&mut self) -> Option<(String, String, OMap<String, String>)> {
        let line = self.loads_line();
        let Some(declared) = self.options.pressure_closure.clone() else {
            self.refuse(
                "MBF-IMPORT-PRESSURE-CLOSURE-REQUIRED",
                line,
                "LOADS Pressure 1",
                "the file has pressure; each pressure-region terminal needs a closure transfer (transfers_to_wall or \
                 separately_supported_or_compensated), which the MBF does not state and SWBPIPE never infers: pass \
                 pressure_closure",
            );
            return None;
        };
        let invalid = |importer: &mut Importer| {
            importer.refuse(
                "MBF-IMPORT-PRESSURE-CLOSURE-INVALID",
                line,
                "pressure_closure",
                "the closure declaration needs closure_transfer (transfers_to_wall or separately_supported_or_compensated), \
                 a non-empty provenance and optionally nodes, a map from CAEPIPE node names to a closure transfer",
            );
        };
        let Some(dict) = declared.as_dict() else {
            invalid(self);
            return None;
        };
        let nodes = match dict.get(&"nodes".to_string()) {
            None => Value::Dict(Dict::new()),
            Some(value) => value.clone(),
        };
        let keys_ok = dict.keys().all(|key| ["closure_transfer", "provenance", "nodes"].contains(&key.as_str()));
        let transfer_ok = str_in(declared.get_or_null("closure_transfer"), &CLOSURE_TRANSFERS);
        let provenance = declared.get_or_null("provenance").as_str().map(|text| crate::pystr::py_strip(text).to_string());
        let provenance_ok = provenance.as_ref().map(|text| !text.is_empty()).unwrap_or(false);
        let nodes_ok = nodes.as_dict().map(|map| map.values().all(|value| str_in(value, &CLOSURE_TRANSFERS))).unwrap_or(false);
        if !(keys_ok && transfer_ok && provenance_ok && nodes_ok) {
            invalid(self);
            return None;
        }
        let mut by_node = OMap::new();
        for (name, value) in nodes.as_dict().expect("nodes").iter() {
            by_node.insert(name.clone(), value.as_str().expect("transfer").to_string());
        }
        Some((declared.get_or_null("closure_transfer").as_str().expect("transfer").to_string(), provenance.expect("provenance"), by_node))
    }

    fn restore_regions(&mut self, stored: &[Value], active: &OMap<String, Num>, route: &Route) -> Option<Vec<Region>> {
        let mut restored: Vec<Region> = Vec::new();
        let mut owned: HashSet<String> = HashSet::new();
        let mut complete = true;
        for entry in stored {
            let id = entry.get_or_null("id").as_str().unwrap_or_default().to_string();
            let members: Vec<String> =
                entry.get_or_null("members").as_list().map(|items| items.iter().filter_map(|item| item.as_str().map(String::from)).collect()).unwrap_or_default();
            let terminals = entry.get_or_null("terminals").as_list().cloned().unwrap_or_default();
            let unique: HashSet<&String> = members.iter().collect();
            let valid = !restored.iter().any(|(region_id, _, _)| *region_id == id)
                && members.iter().all(|item| active.contains_key(item) && !owned.contains(item))
                && unique.len() == members.len()
                && {
                    let mut values: Vec<&Q> = Vec::new();
                    for item in &members {
                        let value = &active.get(item).expect("active").exact;
                        if !values.contains(&value) {
                            values.push(value);
                        }
                    }
                    values.len() == 1
                }
                && terminals.iter().all(|item| item.get("node_ref").and_then(Value::as_str).map(|node| route.name_of.contains_key(node)).unwrap_or(false));
            if !valid {
                complete = false;
                break;
            }
            owned.extend(members.iter().cloned());
            let copied: Vec<Value> = terminals
                .iter()
                .map(|item| {
                    dict![
                        ("node_ref", item.get_or_null("node_ref").clone()),
                        ("closure_transfer", item.get_or_null("closure_transfer").clone()),
                        ("provenance", item.get_or_null("provenance").clone()),
                    ]
                })
                .collect();
            restored.push((id, members, copied));
        }
        if complete {
            let all: HashSet<String> = active.keys().cloned().collect();
            if owned == all {
                return Some(restored);
            }
        }
        self.refuse(
            "MBF-IMPORT-SIDECAR-CONFLICT",
            self.loads_line(),
            "pressure_map",
            "the sidecar's pressure regions do not match the MBF's Pressure 1 values",
        );
        None
    }
}
