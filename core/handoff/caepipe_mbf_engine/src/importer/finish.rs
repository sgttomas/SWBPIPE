//! importer.py: legacy sidecar agreement checks, restored directions and the assumptions list.

use super::assemble::{LABEL_SUSTAINED, STANDARD_GRAVITY};
use super::{tenth_digit, Element, Importer, Num, IMPORT_PROFILE_ID, R};
use crate::pyfmt::{py_float_repr, py_repr};
use crate::pymath::{asin, degrees, hypot, py_max, py_min, radians, sin, sum};
use crate::pystr::{py_repr_str, PyError};
use crate::rational::Q;
use crate::runtime::{norm, sub};
use crate::sidecar::direction_axis;
use crate::value::{py_eq, Value};
use std::collections::{BTreeSet, HashMap};

impl<'a> Importer<'a> {
    pub(crate) fn displacement_target(&self, name: &str, dof: &str, candidates: &[String]) -> Option<String> {
        let mut found: BTreeSet<String> = BTreeSet::new();
        if let Some(loads) = self.maps.get("load_map").and_then(Value::as_dict) {
            for entry in loads.values() {
                let target = entry.get_or_null("target");
                if entry.get_or_null("category").as_str() == Some("imposed_displacement")
                    && entry.get("caepipe_node").and_then(Value::as_str) == Some(name)
                    && target.get("dof").and_then(Value::as_str) == Some(dof)
                {
                    if let Some(support) = target.get("support").and_then(Value::as_str) {
                        found.insert(support.to_string());
                    }
                }
            }
        }
        let found: Vec<String> = found.into_iter().filter(|item| candidates.contains(item)).collect();
        if found.len() == 1 {
            Some(found[0].clone())
        } else {
            None
        }
    }

    fn printed(&mut self, recorded: f64, number: Option<&Num>, path: &str, line: usize) -> R<()> {
        if let Some(number) = number {
            let difference = Q::from_f64(recorded).expect("finite").sub(&number.exact).abs();
            if difference <= tenth_digit(&number.exact)? {
                return Ok(());
            }
        }
        let shown = number.map(|number| number.text.clone()).unwrap_or_else(|| "nothing".into());
        let message = format!(
            "the sidecar records {}, but the MBF prints {shown}; a writer's record lies within half a unit in the tenth \
             significant digit of the printed value",
            py_float_repr(recorded)?
        );
        self.refuse("MBF-IMPORT-SIDECAR-CONFLICT", line, path, &message);
        Ok(())
    }

    fn writer_bound(&self, number: u32) -> R<[Q; 3]> {
        let parent: HashMap<u32, u32> = self.build.elements.iter().map(|element| (element.end, element.start)).collect();
        let mut bound = [Q::zero(), Q::zero(), Q::zero()];
        let mut number = number;
        loop {
            let exact = self.build.nodes.get(&number).expect("node").exact.clone();
            let above = parent.get(&number).copied();
            let base = match above {
                None => [Q::zero(), Q::zero(), Q::zero()],
                Some(above) => self.build.nodes.get(&above).expect("node").exact.clone(),
            };
            for axis in 0..3 {
                bound[axis] = bound[axis].add(&tenth_digit(&exact[axis].sub(&base[axis]))?);
            }
            match above {
                None => return Ok(bound),
                Some(above) => number = above,
            }
        }
    }

    pub(crate) fn legacy_element(&mut self, swb_id: &str, entry: &Value, element: &Element, name: &str, order: usize, bend: bool) -> R<()> {
        let where_ = format!("element_map[{swb_id}]");
        let load = element.load.clone().map(Value::Str).unwrap_or(Value::Null);
        let given = [
            ("material", Value::str(element.material.clone())),
            ("section", Value::str(element.section.clone())),
            ("load", load),
            ("layout_order", Value::int(order as i64)),
            ("layout_row", Value::int(element.row)),
        ];
        for (field, value) in given {
            let recorded = entry.get_or_null(field).clone();
            if !py_eq(&recorded, &value) {
                self.conflict(&where_, field, &recorded, &value, swb_id, name, element.line)?;
            }
        }
        let record = entry.get("bend").cloned().filter(|value| !value.is_null());
        let (Some(record), true) = (record, bend) else { return Ok(()) };
        let n = element.end;
        let row_node = record.get_or_null("caepipe_row_node").clone();
        if !py_eq(&row_node, &Value::Str(n.to_string())) {
            self.conflict(&where_, "bend.caepipe_row_node", &row_node, &Value::Str(n.to_string()), swb_id, name, element.line)?;
        }
        let radius = record.get_or_null("radius_mm").as_f64().expect("radius");
        self.printed(radius, element.radius.as_ref(), &format!("{where_}.bend.radius_mm"), element.line)?;
        let factor = record.get_or_null("flexibility_factor").as_f64().expect("factor");
        self.printed(factor, element.factor.as_ref(), &format!("{where_}.bend.flexibility_factor"), element.line)?;
        let node = self.build.nodes.get(&n).expect("node").exact.clone();
        let reach = self.writer_bound(n)?;
        let slack = Q::from_int(1).add(&Q::pow2(-20));
        let ti = record.get_or_null("tangent_intersection_mm").as_list().cloned().unwrap_or_default();
        for axis in 0..3 {
            let recorded = ti[axis].as_f64().expect("ti");
            if Q::from_f64(recorded).expect("finite").sub(&node[axis]).abs() > reach[axis].mul(&slack) {
                let message = format!(
                    "the sidecar records the TI of bend {n} at {} on this axis, beyond the printing bounds of the offsets \
                     that place it",
                    py_float_repr(recorded)?
                );
                self.refuse("MBF-IMPORT-SIDECAR-CONFLICT", element.line, &format!("{where_}.bend.tangent_intersection_mm[{axis}]"), &message);
            }
        }
        let geometry = self.build.bend_geometry.get(&n).expect("bend").clone();
        let subtended = |importer: &Importer, a: u32, b: u32| -> R<f64> {
            let mut hypots = Vec::new();
            for k in [a, b] {
                let bound = importer.writer_bound(k)?;
                let floats = [bound[0].to_f64()?, bound[1].to_f64()?, bound[2].to_f64()?];
                hypots.push(hypot(&floats));
            }
            let mut spread = sum(&hypots);
            spread += 4.0 * 2.0f64.powi(-52) * (norm(&importer.position(a)) + norm(&importer.position(b)));
            let length = norm(&sub(&importer.position(b), &importer.position(a)));
            Ok(asin(py_min(1.0, spread / length))?)
        };
        let theta = geometry.theta;
        let allowed = subtended(self, geometry.start, n)? + subtended(self, n, geometry.next)?
            + 64.0 * 2.0f64.powi(-52) / py_max(sin(theta)?, 2.0f64.powi(-26));
        let angle = record.get_or_null("angle_deg").as_f64().expect("angle");
        if !((radians(angle) - theta).abs() <= allowed) {
            let message = format!(
                "the sidecar records bend {n}'s angle as {} deg, but its legs in the MBF meet at {} deg, beyond the {} \
                 deg their printed positions allow",
                py_float_repr(angle)?,
                py_float_repr(degrees(theta))?,
                py_float_repr(degrees(allowed))?
            );
            self.refuse("MBF-IMPORT-SIDECAR-CONFLICT", element.line, &format!("{where_}.bend.angle_deg"), &message);
        }
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    fn conflict(&mut self, where_: &str, field: &str, recorded: &Value, given: &Value, swb_id: &str, name: &str, line: usize) -> R<()> {
        let message = format!(
            "the sidecar records {field} {} for {swb_id}, but the file's element {name} has {}",
            py_repr(recorded)?,
            py_repr(given)?
        );
        self.refuse("MBF-IMPORT-SIDECAR-CONFLICT", line, &format!("{where_}.{field}"), &message);
        Ok(())
    }

    pub(crate) fn legacy_tables(&mut self) -> R<()> {
        let materials = self.maps.get("material_map").and_then(Value::as_dict).cloned().unwrap_or_default();
        for (name, entry) in materials.iter() {
            let where_ = format!("material_map[{name}]");
            let Some(record) = self.materials.get(name).cloned() else { continue };
            if record.refused {
                continue;
            }
            let line = record.line;
            let number = |key: &str| entry.get_or_null(key).as_f64().expect("number");
            self.printed(number("weight_density_kgf_per_m3"), record.density.as_ref(), &format!("{where_}.weight_density_kgf_per_m3"), line)?;
            self.printed(number("poisson_ratio"), record.nu.as_ref(), &format!("{where_}.poisson_ratio"), line)?;
            for (modulus, alpha, _) in &record.rows {
                self.printed(number("elastic_modulus_n_per_mm2"), Some(modulus), &format!("{where_}.elastic_modulus_n_per_mm2"), line)?;
                self.printed(number("thermal_expansion_mm_per_mm_per_c"), Some(alpha), &format!("{where_}.thermal_expansion_mm_per_mm_per_c"), line)?;
            }
            let recorded = entry.get_or_null("temperatures_c").as_list().cloned().unwrap_or_default();
            if recorded.len() != record.rows.len() {
                let message = format!(
                    "the sidecar records {} temperatures for {name}; the MBF prints {}",
                    recorded.len(),
                    record.rows.len()
                );
                self.refuse("MBF-IMPORT-SIDECAR-CONFLICT", line, &format!("{where_}.temperatures_c"), &message);
            } else {
                for (index, (value, row)) in recorded.iter().zip(&record.rows).enumerate() {
                    self.printed(value.as_f64().expect("number"), Some(&row.2), &format!("{where_}.temperatures_c[{index}]"), line)?;
                }
            }
            let carried = self.build.carriers.get(&("material".to_string(), name.clone())).cloned().unwrap_or_default();
            if !py_eq(entry.get_or_null("elements"), &Value::List(carried.into_iter().map(Value::Str).collect())) {
                let message = format!("the sidecar's elements of {name} are not the file's, in layout order");
                self.refuse("MBF-IMPORT-SIDECAR-CONFLICT", line, &format!("{where_}.elements"), &message);
            }
        }
        let sections = self.maps.get("section_map").and_then(Value::as_dict).cloned().unwrap_or_default();
        for (name, entry) in sections.iter() {
            let where_ = format!("section_map[{name}]");
            let Some(record) = self.pipes.get(name).cloned() else { continue };
            let number = |key: &str| entry.get_or_null(key).as_f64().expect("number");
            self.printed(number("outside_diameter_mm"), record.od.as_ref(), &format!("{where_}.outside_diameter_mm"), record.line)?;
            self.printed(number("wall_thickness_mm"), record.thk.as_ref(), &format!("{where_}.wall_thickness_mm"), record.line)?;
            let carried = self.build.carriers.get(&("section".to_string(), name.clone())).cloned().unwrap_or_default();
            if !py_eq(entry.get_or_null("elements"), &Value::List(carried.into_iter().map(Value::Str).collect())) {
                let message = format!("the sidecar's elements of {name} are not the file's, in layout order");
                self.refuse("MBF-IMPORT-SIDECAR-CONFLICT", record.line, &format!("{where_}.elements"), &message);
            }
        }
        Ok(())
    }

    pub(crate) fn thermal_direction(&mut self, entries: &[(String, Option<String>)], formed: &str, line: usize) -> String {
        let recorded: Vec<(String, String)> =
            entries.iter().filter_map(|(key, direction)| direction.clone().map(|direction| (key.clone(), direction))).collect();
        let mut axes: Vec<(char, usize)> = Vec::new();
        for (_, direction) in &recorded {
            let axis = direction_axis(direction).expect("direction");
            if !axes.contains(&axis) {
                axes.push(axis);
            }
        }
        if axes.len() > 1 {
            let listed = recorded.iter().map(|(key, direction)| format!("{key}: {direction}")).collect::<Vec<_>>().join(", ");
            let message = format!(
                "the sidecar entries the MBF sums into {formed} record different directions ({listed}); a sum across \
                 directions is not meaningful"
            );
            let last = recorded.last().expect("recorded").0.clone();
            self.refuse("MBF-IMPORT-SIDECAR-CONFLICT", line, &format!("load_map[{last}].direction"), &message);
        }
        let effective = recorded.first().map(|(_, direction)| direction.clone()).unwrap_or_else(|| "global_x".into());
        let why = if recorded.is_empty() {
            "the import's token, as no entry records one"
        } else {
            "the direction the other entries it sums record"
        };
        for (key, direction) in entries {
            if direction.is_none() {
                let message = format!("load_map[{key}] records no direction; {formed} takes {effective}, {why}");
                self.listed("approximated", "MBF-IMPORT-DIRECTION-UNRECOVERABLE", line, &format!("load_map[{key}].direction"), &message);
            }
        }
        effective
    }

    pub(crate) fn report_unrecoverable(&mut self) {
        if !self.bound {
            return;
        }
        self.listed(
            "approximated",
            "MBF-IMPORT-ID-UNRECOVERABLE",
            0,
            "unexported members",
            "members the writer does not export (labels, provenance, combinations, lines, unresolved assumptions, \
             constraints, equipment interfaces, rule-pack references, model settings) are not recoverable from the MBF",
        );
    }

    pub(crate) fn add_assumptions(&mut self) -> Result<(), PyError> {
        let mode = if self.si {
            "SI: lengths in mm, forces in N, moments in N*m, E in MPa (N/mm2), temperatures in degC"
        } else {
            "English: lengths in in (feet-inch notation evaluated exactly, 1 ft = 12 in), forces in lbf, moments in \
             lbf*ft, E in psi, temperatures in degF"
        };
        let vertical = if self.z_vertical {
            "Z vertical, turned into SWBPIPE's Y-vertical axes by the exact rotation (X, Y, Z) -> (X, Z, -Y) of every \
             coordinate, offset, force, moment, displacement, rotation and restraint axis; the SI writer re-exports the \
             rotated model Y-vertical"
        } else {
            "Y vertical"
        };
        let version = self.version.clone().unwrap_or_else(|| "None".into());
        self.assumptions.push(format!(
            "Profile {IMPORT_PROFILE_ID}: MBF version {version}, unit mode {mode}; angles in {}; {vertical}; quantities \
             keep their MBF unit.",
            if self.rad { "rad" } else { "deg" }
        ));
        self.assumptions.push(
            "Node positions are summed from the printed offsets as exact rationals and rounded to binary64 once per node."
                .into(),
        );
        if !self.si {
            self.assumptions.push(
                "English imports keep their US-customary units; the SI writer re-exports them by its exact \
                 normalization to the SI profile (O-2, MBF-SI-UNITS-NORMALIZED), with the reference temperature passed \
                 in degF."
                    .into(),
            );
        }
        let dense = self.materials.values().any(|record| record.density.as_ref().map(|value| !value.exact.is_zero()).unwrap_or(false));
        let gravity = py_float_repr(STANDARD_GRAVITY)?;
        if self.si && dense {
            self.assumptions.push(format!(
                "Weight density in kgf/m3 becomes weight per length w = -rho * {gravity} * A in N/m along global -Y (1 \
                 kgf = {gravity} N), A the bare-metal area pi/4 (OD^2 - ID^2); weight acts on bare metal only."
            ));
        } else if !self.si {
            self.assumptions.push(
                "Weight density in lbf/in3 becomes weight per length w = -rho * A in lbf/in along global -Y, A the \
                 bare-metal area; weight acts on bare metal only."
                    .into(),
            );
        }
        self.assumptions.push(
            "G is derived as E/(2(1+nu)) with E in Pa; materials are temperature-independent (identical E and expansion \
             coefficient on every temperature line)."
                .into(),
        );
        self.assumptions.push(format!(
            "Thermal change per element is T1 - T_ref (OPTEMP); zero changes give no primitive. Section corrosion, mill \
             tolerance, insulation and lining fields given as zero are written as absent{}",
            if self.pressure_notes.is_empty() { "." } else { ", except the corrosion allowance under pressure." }
        ));
        let notes = self.pressure_notes.clone();
        self.assumptions.extend(notes);
        let kinds: BTreeSet<&str> = self.build.node_data.values().flat_map(|record| record.supports.iter().map(|item| item.kind)).collect();
        let flexible = self.build.node_data.values().any(|record| !record.anchor.is_empty() && !record.trak.is_empty());
        let unit = if self.si { "N/mm" } else { "lbf/in" };
        let notes = [
            (
                "guide",
                "A guide (G) restrains the two global translations lateral to the element whose local axes it takes, \
                 the preceding element or else the following one (technical reference, Guide), which lies along a \
                 global axis: rigid, or with STIFF a spring on each.",
            ),
            (
                "stop",
                "A limit stop LS(0,0) along an axis-aligned DV is stopped both ways at its undeflected position \
                 (technical reference, Limit Stop): a rigid line stop on that DOF, or with STIFF a spring.",
            ),
            (
                "skew",
                "A skewed restraint K along an axis-aligned DV is a two-way spring on that DOF (technical reference, \
                 Skewed Restraint).",
            ),
        ];
        for (kind, note) in notes {
            if kinds.contains(kind) {
                self.assumptions.push(note.into());
            }
        }
        if flexible {
            self.assumptions.push(
                "A flexible anchor (KA with TRAK and ROTK(0,0,0)) is a spring on each translation with a non-zero \
                 stiffness; a zero stiffness leaves its DOF free (technical reference, Anchor)."
                    .into(),
            );
        }
        let stiff = self.build.node_data.values().any(|record| record.supports.iter().any(|item| item.stiffness.is_some()));
        if kinds.iter().any(|kind| *kind != "guide") || flexible || stiff {
            self.assumptions.push(format!("Spring stiffnesses keep their MBF unit, {unit}."));
        }
        let gravity_key = ("gravity".to_string(), "the earth gravity setting".to_string());
        if self.used_conventions.contains(&gravity_key) {
            self.used_conventions.retain(|item| *item != gravity_key);
            self.assumptions.push(
                "OPMISC gives earth gravity (the export's default setting 386.4 with direction (0, -1, 0), or standard \
                 gravity in the documented layout's unit); weights are imported as earth weights along -Y."
                    .into(),
            );
        }
        let mut conventions = self.used_conventions.clone();
        conventions.sort();
        for (basis, what) in conventions {
            let source = if basis == "bound" {
                "the writer's convention (bound import)".to_string()
            } else {
                format!("profile decision PD-1 for MBF version {version} (TBD-17-04-007 stays open)")
            };
            let meaning = if what.contains("L=") {
                "is carried until changed, as M and P are".to_string()
            } else {
                format!("belongs to the sustained case {}", py_repr_str(LABEL_SUSTAINED))
            };
            let mut chars = what.chars();
            let first = chars.next().map(|c| c.to_uppercase().collect::<String>()).unwrap_or_default();
            self.assumptions.push(format!("{first}{} {meaning}; basis: {source}.", chars.as_str()));
        }
        self.assumptions.push("No combinations are created; CAEPIPE forms its own load combinations.".into());
        Ok(())
    }
}
