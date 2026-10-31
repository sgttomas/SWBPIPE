//! The writer's reading of the model document: the document and project checks, nodes, materials, pipe segments,
//! bends, supports, load cases and the field sweep (si_writer `_Writer.check_*` and `read_*`).

use super::*;
use crate::importer::PIPING_CODE_OPTIONS;
use crate::pystr::py_strip;
use crate::value::{py_eq, str_in};

const MECHANICAL: [&str; 3] = ["concentrated_force", "concentrated_moment", "weight"];
const THERMAL: [&str; 2] = ["thermal", "imposed_displacement"];
const RIGID_FAMILIES: [&str; 4] = ["anchor", "guide", "line_stop", "vertical_support"];

/// `_FORCE_DIRECTIONS[token]`.
fn force_direction(token: &str) -> Option<usize> {
    match token {
        "global_x" | "GLOBAL_X" => Some(0),
        "global_y" | "GLOBAL_Y" => Some(1),
        "global_z" | "GLOBAL_Z" => Some(2),
        _ => None,
    }
}

/// `_MOMENT_DIRECTIONS[token]`.
fn moment_direction(token: &str) -> Option<usize> {
    match token {
        "rotation_x" => Some(0),
        "rotation_y" => Some(1),
        "rotation_z" => Some(2),
        _ => dof_token(token).filter(|dof| dof.starts_with('R')).map(|dof| DOFS.iter().position(|d| *d == dof).expect("dof") - 3),
    }
}

fn zeros() -> [Q; 3] {
    [Q::zero(), Q::zero(), Q::zero()]
}

impl<'m> Writer<'m> {
    /// `_list(value, writer, entity)`.
    pub fn list(&mut self, value: &Value, entity: &str) -> Vec<Value> {
        match value {
            Value::Null => Vec::new(),
            Value::List(items) => items.clone(),
            _ => {
                self.invalid(entity, &format!("{entity} must be a list"));
                Vec::new()
            }
        }
    }

    pub fn check_piping_code(&mut self) -> R<()> {
        let code = self.piping_code.clone();
        if code.is_null() {
            self.diag(
                "MBF-SI-PIPING-CODE-NOT-WRITTEN",
                "info",
                "OPTIONS",
                "no piping code was chosen, so OPTIONS carries none and the target program's default piping code \
                 applies; the code decides the load cases offered and which code-dependent settings are honoured \
                 (Analysis)",
            );
        } else if !str_in(&code, &PIPING_CODE_OPTIONS) {
            let message = format!(
                "piping code {} is not one of the documented OPTIONS piping-code tokens (Format of .MBF)",
                repr(&code)?
            );
            self.refuse("MBF-SI-PIPING-CODE-UNSUPPORTED", "OPTIONS", &message);
        } else if str_in(&code, &B31J_LOCKED_CODES) {
            let message = format!(
                "piping code {} is the current edition, for which the target program computes SIFs and flexibility \
                 factors by B31J whatever OPCODE says (Analysis); OPCODE B31J off is written but not honoured, and a \
                 tee's flexibility then differs from SWBPIPE's, which assembles none",
                text(&code)?
            );
            self.diag("MBF-SI-PIPING-CODE-B31J-LOCKED", "warning", "OPTIONS", &message);
        }
        Ok(())
    }

    pub fn check_document(&mut self) -> R<()> {
        let model = self.model;
        let version = model.get_or_null("schema_version");
        let current = crate::MODEL_SCHEMA_VERSION;
        if str_in(version, &RETIRED_MODEL_VERSIONS) {
            let message = format!(
                "model document {} is retired; re-author it as model document {current} (a pressure-free model omits \
                 pressure_contract). Nothing is migrated.",
                text(version)?
            );
            self.diag("MODEL_SCHEMA_VERSION_RETIRED", "error", "model.schema_version", &message);
        } else if version.as_str() != Some(current) {
            let message = format!("model schema_version {} is not {current}, the version this writer reads", repr(version)?);
            self.diag("MBF-SI-SCHEMA-VERSION", "error", "model.schema_version", &message);
        }
        let contract = model.get_or_null("pressure_contract");
        if !contract.is_null() {
            let admitted = match contract {
                Value::Dict(dict) => {
                    contract.get_or_null("version").as_str() == Some(PRESSURE_CONTRACT_VERSION.0)
                        && contract.get_or_null("mode").as_str() == Some(PRESSURE_CONTRACT_VERSION.1)
                        && str_in(contract.get_or_null("shear_model"), &SHEAR_MODELS)
                        && dict.len() == 3
                        && ["version", "mode", "shear_model"].iter().all(|key| dict.contains_key(&key.to_string()))
                }
                _ => false,
            };
            if !admitted {
                self.refuse(
                    "MBF-SI-PRESSURE-UNSUPPORTED",
                    "model.pressure_contract",
                    "only the exact pressure contract 3.0.0/exact_pressure_v3 with an admitted shear_model is exported",
                );
            } else {
                self.contract = Some(contract.clone());
                let message = format!(
                    "shear_model {} is not in the MBF; the target program applies its own beam theory, and the \
                     sidecar's pressure_map keeps the contract for a bound import",
                    text(contract.get_or_null("shear_model"))?
                );
                self.omit("MBF-SI-SHEAR-MODEL-NOT-EXPORTED", "model.pressure_contract.shear_model", &message);
            }
        }
        if !model.get_or_null("shear_model").is_null() {
            self.refuse("MBF-SI-SHEAR-MODEL-UNSUPPORTED", "model.shear_model", "shear_model is not exported");
        }
        if model.get_or_null("reference_configurations").truthy() {
            self.refuse(
                "MBF-SI-REFERENCE-CONFIGURATION-UNSUPPORTED",
                "model.reference_configurations",
                "reference configurations are not exported by the pilot writer",
            );
        }
        Ok(())
    }

    pub fn check_project_units(&mut self) -> R<()> {
        let project = self.model.get_or_null("project");
        let units = match project.get("units") {
            Some(units @ Value::Dict(_)) => units.clone(),
            _ => Value::Dict(Dict::new()),
        };
        for (key, accepted) in PROJECT_UNITS {
            let unit = units.get_or_null(key);
            if !str_in(unit, accepted) {
                let message = format!(
                    "project.units.{key} is {}; the SI MBF writer accepts only {} and converts no other unit system",
                    repr(unit)?,
                    accepted.join(" or ")
                );
                self.refuse("MBF-SI-PROJECT-UNIT-UNSUPPORTED", &format!("project.units.{key}"), &message);
            }
        }
        if let Some(scale) = units.get_or_null("length").as_str().and_then(|unit| lookup(LENGTH_TO_MM, unit)) {
            self.length_exp = scale;
        }
        Ok(())
    }

    // -- model members ----------------------------------------------------
    pub fn read_nodes(&mut self) -> R<()> {
        for node in self.list(self.model.get_or_null("nodes"), "model.nodes") {
            let Some(id) = node.get("id").and_then(Value::as_str).filter(|_| matches!(node, Value::Dict(_))) else {
                self.invalid("model.nodes", "each node needs a string id");
                continue;
            };
            let id = id.to_string();
            let entity = format!("node:{id}");
            if self.nodes.contains_key(&id) {
                self.invalid(&entity, &format!("node id {} is repeated", crate::pystr::py_repr_str(&id)));
                continue;
            }
            let position = node.get_or_null("position");
            if !matches!(position, Value::Dict(_)) {
                self.invalid(&entity, &format!("{entity} position must be an object with x, y, z"));
                continue;
            }
            let mut coords = Vec::new();
            for axis in ["x", "y", "z"] {
                coords.push(self.number(position.get_or_null(axis), &entity, &format!("position.{axis}"))?);
            }
            if coords.iter().any(Option::is_none) {
                continue;
            }
            let converted: Vec<Option<f64>> = coords.into_iter().map(|value| self.convert(value.expect("number"))).collect();
            if converted.iter().any(Option::is_none) {
                continue;
            }
            self.nodes.insert(id, [converted[0].unwrap(), converted[1].unwrap(), converted[2].unwrap()]);
        }
        Ok(())
    }

    pub fn read_materials(&mut self) -> R<()> {
        for material in self.list(self.model.get_or_null("materials"), "model.materials") {
            let Some(id) = material.get("id").and_then(Value::as_str) else {
                self.invalid("model.materials", "each material needs a string id");
                continue;
            };
            let id = id.to_string();
            let entity = format!("material:{id}");
            if self.materials.contains_key(&id) {
                self.invalid(&entity, &format!("material id {} is repeated", crate::pystr::py_repr_str(&id)));
                continue;
            }
            let mut ok = true;
            if material.get_or_null("temperature_points").truthy() {
                self.refuse(
                    "MBF-SI-TEMPERATURE-DEPENDENT-MATERIAL-UNSUPPORTED",
                    &entity,
                    &format!("{entity} has temperature_points; temperature-dependent properties are not exported"),
                );
                ok = false;
            }
            if material.has("expansion_laws") {
                self.refuse(
                    "MBF-SI-TEMPERATURE-DEPENDENT-MATERIAL-UNSUPPORTED",
                    &entity,
                    &format!("{entity} has expansion_laws; material expansion laws are not exported"),
                );
                ok = false;
            }
            let mut e_mpa =
                self.quantity(material.get_or_null("elastic_modulus"), STRESS_TO_MPA, "N/mm2", &entity, "elastic_modulus")?;
            if matches!(e_mpa, Some(value) if value <= 0.0) {
                self.diag("MBF-SI-VALUE-OUT-OF-RANGE", "error", &entity, &format!("{entity} elastic_modulus must be positive"));
                e_mpa = None;
            }
            let mut g_mpa = None;
            if !material.get_or_null("shear_modulus").is_null() {
                g_mpa = self.quantity(material.get_or_null("shear_modulus"), STRESS_TO_MPA, "N/mm2", &entity, "shear_modulus")?;
            }
            let mut nu = None;
            if !material.get_or_null("poisson_ratio").is_null() {
                nu = self.quantity(material.get_or_null("poisson_ratio"), DIMENSIONLESS, "1", &entity, "poisson_ratio")?;
                if let (Some(nu), Some(e), Some(g)) = (nu, e_mpa, g_mpa) {
                    if g > 0.0 {
                        let denominator = 2.0 * (1.0 + nu);
                        if denominator == 0.0 {
                            return Err(py(PyError { kind: "ZeroDivisionError", message: "float division by zero".into() }));
                        }
                        let implied = e / denominator;
                        if (implied - g).abs() > 1e-9 * g {
                            let message = format!(
                                "{entity} gives both poisson_ratio and shear_modulus; the MBF carries the Poisson ratio \
                                 {}, whose implied G = {} MPa differs from the authored G = {} MPa",
                                num(nu),
                                num(implied),
                                num(g)
                            );
                            self.omit("MBF-SI-SHEAR-MODULUS-NOT-EXPORTED", &entity, &message);
                        }
                    }
                }
            } else if let (Some(e), Some(g)) = (e_mpa, g_mpa) {
                if g <= 0.0 {
                    self.diag("MBF-SI-VALUE-OUT-OF-RANGE", "error", &entity, &format!("{entity} shear_modulus must be positive"));
                } else {
                    let value = e / (2.0 * g) - 1.0;
                    nu = Some(value);
                    self.assumptions.push(format!(
                        "{entity}: Poisson ratio derived as E/(2G) - 1 = {} (E = {} MPa, G = {} MPa)",
                        num(value),
                        num(e),
                        num(g)
                    ));
                }
            } else if e_mpa.is_some() {
                self.refuse(
                    "MBF-SI-MATERIAL-POISSON-MISSING",
                    &entity,
                    &format!("{entity} has neither poisson_ratio nor shear_modulus; the MBF needs a Poisson ratio"),
                );
            }
            if let Some(value) = nu {
                if !(0.0 < value && value < 0.5) {
                    let message = format!("{entity} Poisson ratio {} is outside (0, 0.5)", num(value));
                    self.diag("MBF-SI-VALUE-OUT-OF-RANGE", "error", &entity, &message);
                    nu = None;
                }
            }
            let mut alpha = Some(0.0);
            let has_alpha = !material.get_or_null("thermal_expansion_coefficient").is_null();
            if has_alpha {
                alpha = self.quantity(
                    material.get_or_null("thermal_expansion_coefficient"),
                    EXPANSION_TO_PER_C,
                    "mm/mm/C",
                    &entity,
                    "thermal_expansion_coefficient",
                )?;
            }
            let (true, Some(e), Some(nu), Some(alpha)) = (ok, e_mpa, nu, alpha) else { continue };
            self.materials.insert(id, Material { e, nu, alpha, has_alpha, origin: material_origin(&material) });
        }
        Ok(())
    }

    /// The section library (Design §15, Format 4): each record a pipe segment's `section_ref` may bind.
    pub fn read_sections(&mut self) {
        let model: &'m Value = self.model;
        let records: &'m [Value] = match model.get_or_null("sections") {
            Value::Null => &[],
            Value::List(items) => items,
            _ => {
                self.invalid("model.sections", "model.sections must be a list");
                &[]
            }
        };
        for section in records {
            if !matches!(section, Value::Dict(_)) || section.get("id").and_then(Value::as_str).is_none() {
                self.invalid("model.sections", "each section needs a string id");
                continue;
            }
            self.library.push(section);
        }
    }

    /// The library section id the pipe binds, or None without `section_ref`. A binding the solve refuses
    /// (product_physics resolve_shared_sections) is refused by name, since the import could not restore it.
    fn bind_section(&mut self, pipe: &Value, section: &Value, entity: &str) -> R<Option<String>> {
        let reference = pipe.get_or_null("section_ref");
        if reference.is_null() {
            return Ok(None);
        }
        let count = match reference.as_str() {
            Some(id) if !id.trim().is_empty() => {
                self.library.iter().filter(|item| item.get_or_null("id").as_str() == Some(id)).count()
            }
            _ => 0,
        };
        if count != 1 {
            let message = format!("{entity} section_ref {} must name exactly one section in model.sections", repr(reference)?);
            self.refuse("MBF-SI-REFERENCE-UNRESOLVED", entity, &message);
            return Ok(None);
        }
        let id = reference.as_str().expect("str").to_string();
        let shared = *self.library.iter().find(|item| item.get_or_null("id").as_str() == Some(id.as_str())).expect("one");
        let quantity = |value: &Value| match value {
            Value::Dict(record) => {
                record.len() == 2
                    && matches!(record.get(&"value".to_string()), Some(Value::Int(_) | Value::Float(_)))
                    && matches!(record.get(&"unit".to_string()), Some(Value::Str(_)))
            }
            _ => false,
        };
        let properties = shared.get_or_null("properties");
        let supported = shared.get_or_null("section_type").as_str() == Some("pipe")
            && match properties {
                Value::Dict(record) => {
                    record.len() == 2
                        && ["outside_diameter", "wall_thickness"].iter().all(|name| record.contains_key(&name.to_string()))
                        && record.values().all(quantity)
                }
                _ => false,
            };
        if !supported {
            let message = format!(
                "{entity} section_ref {} binds a section that is not of type pipe with exactly outside_diameter and \
                 wall_thickness quantities, which the solve refuses",
                repr(reference)?
            );
            self.refuse("MBF-SI-SECTION-BINDING-UNSUPPORTED", entity, &message);
            return Ok(None);
        }
        let stale: Vec<&str> = ["outside_diameter", "wall_thickness"]
            .into_iter()
            .filter(|name| {
                let (inline, shared) = (section.get_or_null(name), properties.get_or_null(name));
                !py_eq(inline.get_or_null("value"), shared.get_or_null("value"))
                    || !py_eq(inline.get_or_null("unit"), shared.get_or_null("unit"))
            })
            .collect();
        if !stale.is_empty() {
            let message = format!(
                "{entity} section {} differ from section {} in value or unit (SECTION_REFERENCE_CACHE_STALE); the \
                 import would restore the section from the pipe's values",
                stale.join(" and "),
                repr(reference)?
            );
            self.refuse("MBF-SI-SECTION-REFERENCE-STALE", entity, &message);
            return Ok(None);
        }
        Ok(Some(id))
    }

    pub fn read_pipes(&mut self) -> R<()> {
        for pipe in self.list(self.model.get_or_null("pipe_segments"), "model.pipe_segments") {
            let Some(id) = pipe.get("id").and_then(Value::as_str) else {
                self.invalid("model.pipe_segments", "each pipe segment needs a string id");
                continue;
            };
            let id = id.to_string();
            let entity = format!("pipe:{id}");
            if self.pipes.contains_key(&id) {
                self.invalid(&entity, &format!("pipe segment id {} is repeated", crate::pystr::py_repr_str(&id)));
                continue;
            }
            let (start, end) = (pipe.get_or_null("from"), pipe.get_or_null("to"));
            let known = |value: &Value, nodes: &OMap<String, V3>| value.as_str().map(|id| nodes.contains_key(&id.to_string())).unwrap_or(false);
            if !known(start, &self.nodes) || !known(end, &self.nodes) {
                let message = format!("{entity} connects {} to {}; both must be model nodes", repr(start)?, repr(end)?);
                self.diag("MBF-SI-REFERENCE-UNRESOLVED", "error", &entity, &message);
                continue;
            }
            let (start, end) = (start.as_str().expect("str").to_string(), end.as_str().expect("str").to_string());
            if start == end {
                self.refuse("MBF-SI-GEOMETRY-UNSUPPORTED", &entity, &format!("{entity} starts and ends at the same node"));
                continue;
            }
            let material_id = pipe.get_or_null("material");
            let material = material_id.as_str().filter(|id| self.materials.contains_key(&id.to_string()));
            let Some(material) = material.map(String::from) else {
                let named = format!("material:{}", text(material_id)?);
                if !self.diagnostics.iter().any(|item| item.get("entity").and_then(Value::as_str) == Some(named.as_str())) {
                    let message = format!("{entity} names material {}, which is not in model.materials", repr(material_id)?);
                    self.diag("MBF-SI-REFERENCE-UNRESOLVED", "error", &entity, &message);
                }
                continue;
            };
            let y_reference = pipe.get_or_null("y_reference").clone();
            let Some((od, thk)) = self.read_section(pipe.get_or_null("section"), &entity)? else { continue };
            let section_ref = self.bind_section(&pipe, pipe.get_or_null("section"), &entity)?; // the pipe stays read
            self.pipes.insert(id, Pipe { from: start, to: end, material, y_reference, section_ref, od, thk });
        }
        Ok(())
    }

    pub fn omit_orientation(&mut self) {
        let orientation: Vec<String> = self
            .pipes
            .iter()
            .filter(|(id, pipe)| !pipe.y_reference.is_null() && !self.bends.contains_key(*id))
            .map(|(id, _)| id.clone())
            .collect();
        if !orientation.is_empty() {
            let message = format!(
                "y_reference of {} is not exported; element local axes follow the target program's own convention",
                orientation.join(", ")
            );
            self.omit("MBF-SI-ORIENTATION-NOT-EXPORTED", "model.pipe_segments.y_reference", &message);
        }
    }

    pub fn read_section(&mut self, section: &Value, entity: &str) -> R<Option<(f64, f64)>> {
        if !matches!(section, Value::Dict(_)) {
            self.invalid(entity, &format!("{entity} section must be an object"));
            return Ok(None);
        }
        let od = self.quantity(section.get_or_null("outside_diameter"), LENGTH_TO_MM, "mm", entity, "section.outside_diameter")?;
        let thk = self.quantity(section.get_or_null("wall_thickness"), LENGTH_TO_MM, "mm", entity, "section.wall_thickness")?;
        let mut ok = od.is_some() && thk.is_some();
        if let (true, Some(od), Some(thk)) = (ok, od, thk) {
            if !(od > 0.0 && thk > 0.0 && 2.0 * thk < od) {
                let message = format!(
                    "{entity} needs 0 < wall_thickness < outside_diameter / 2 (got {} mm, {} mm)",
                    num(od),
                    num(thk)
                );
                self.diag("MBF-SI-VALUE-OUT-OF-RANGE", "error", entity, &message);
                ok = false;
            }
        }
        if self.contract.is_some() && section.get_or_null("corrosion_allowance").is_null() {
            let message = format!(
                "{entity} has no corrosion_allowance under the exact pressure contract; the MBF would write 0, an \
                 allowance the model does not give"
            );
            self.refuse("MBF-SI-CORROSION-ALLOWANCE-MISSING", entity, &message);
        }
        for name in ["corrosion_allowance", "mill_tolerance", "insulation_thickness"] {
            let record = section.get_or_null(name);
            if record.is_null() {
                continue;
            }
            match self.quantity(record, LENGTH_TO_MM, "mm", entity, &format!("section.{name}"))? {
                None => ok = false,
                Some(value) if value != 0.0 => {
                    let message = format!("{entity} section.{name} is nonzero; the pilot writer exports bare pipe only");
                    self.refuse("MBF-SI-SECTION-UNSUPPORTED", entity, &message);
                    ok = false;
                }
                Some(_) => {}
            }
        }
        for name in ["material_density", "contents_density", "insulation_density"] {
            if !section.get_or_null(name).is_null() {
                let message =
                    format!("{entity} section.{name} is not exported; weight is exported only from weight primitives");
                self.omit("MBF-SI-SECTION-FIELD-NOT-EXPORTED", entity, &message);
            }
        }
        if !ok {
            return Ok(None);
        }
        Ok(Some((od.expect("od"), thk.expect("thk"))))
    }

    pub fn read_components(&mut self) -> R<()> {
        for component in self.list(self.model.get_or_null("components"), "model.components") {
            let identity = component.get_or_null("id");
            let kind = component.get_or_null("kind");
            let consumption = component.get_or_null("mechanics_interface").get_or_null("solver_consumption");
            if kind.as_str() == Some("bend") && consumption.as_str() == Some(BEND_SOLVER_CONSUMPTION) {
                self.read_bend(&component)?;
                continue;
            }
            let message = format!(
                "component {} of kind {} (solver_consumption {}) is not exported; the pilot writer exports straight \
                 pipe segments and bends realized as {} only",
                repr(identity)?,
                repr(kind)?,
                repr(consumption)?,
                crate::pystr::py_repr_str(BEND_SOLVER_CONSUMPTION)
            );
            self.refuse("MBF-SI-COMPONENT-UNSUPPORTED", &format!("component:{}", text(identity)?), &message);
        }
        self.omit_orientation();
        Ok(())
    }

    pub fn read_bend(&mut self, component: &Value) -> R<()> {
        let entity = format!("component:{}", text(component.get_or_null("id"))?);
        let geometry = component.get_or_null("geometry");
        let modifiers = component.get_or_null("modifiers");
        if !matches!(geometry, Value::Dict(_)) {
            self.invalid(&entity, &format!("{entity} bend needs a geometry block with bend_pipe_ref and bend_radius"));
            return Ok(());
        }
        let pipe_ref = geometry.get_or_null("bend_pipe_ref");
        let Some(pipe_id) = pipe_ref.as_str().filter(|id| self.pipes.contains_key(&id.to_string())).map(String::from) else {
            let message = format!("{entity} bend_pipe_ref {} is not an exported pipe segment", repr(pipe_ref)?);
            self.diag("MBF-SI-REFERENCE-UNRESOLVED", "error", &entity, &message);
            return Ok(());
        };
        if self.bends.contains_key(&pipe_id) {
            let message = format!("{entity} realizes pipe:{pipe_id}, which another bend already realizes");
            self.refuse("MBF-SI-BEND-UNSUPPORTED", &entity, &message);
            return Ok(());
        }
        let (from, to) = {
            let pipe = self.pipes.get(&pipe_id).expect("pipe");
            (pipe.from.clone(), pipe.to.clone())
        };
        let node = component.get_or_null("node");
        if node.as_str() != Some(from.as_str()) && node.as_str() != Some(to.as_str()) {
            let message = format!("{entity} node {} is not an endpoint of pipe:{pipe_id}", repr(node)?);
            self.diag("MBF-SI-REFERENCE-UNRESOLVED", "error", &entity, &message);
            return Ok(());
        }
        let factor_record = match modifiers {
            Value::Dict(_) => modifiers.get_or_null("flexibility_factor_user_value"),
            _ => &Value::Null,
        };
        if factor_record.is_null() {
            let message = format!(
                "{entity} has no modifiers.flexibility_factor_user_value; the writer never supplies an implicit or \
                 code-based bend flexibility factor"
            );
            self.refuse("MBF-SI-BEND-FLEXIBILITY-MISSING", &entity, &message);
            return Ok(());
        }
        let factor = self.quantity(factor_record, DIMENSIONLESS, "1", &entity, "flexibility_factor_user_value")?;
        let radius = self.quantity(geometry.get_or_null("bend_radius"), LENGTH_TO_MM, "mm", &entity, "bend_radius")?;
        let mut angle = None;
        let angle_record = geometry.get_or_null("bend_angle");
        if !angle_record.is_null() {
            let unit = match angle_record {
                Value::Dict(_) => angle_record.get_or_null("unit"),
                _ => &Value::Null,
            };
            if !str_in(unit, &["rad", "deg"]) {
                let message = format!("{entity} bend_angle unit {} is refused (rad or deg)", repr(unit)?);
                self.refuse("MBF-SI-UNIT-UNSUPPORTED", &entity, &message);
                return Ok(());
            }
            angle = self.number(angle_record.get_or_null("value"), &entity, "bend_angle")?;
            if unit.as_str() == Some("deg") {
                angle = angle.map(crate::pymath::radians);
            }
        }
        let sif_records = [modifiers.get_or_null(BEND_SIF_FIELDS[0]), modifiers.get_or_null(BEND_SIF_FIELDS[1])];
        let mut sifs: Option<[Option<f64>; 2]> = None;
        if sif_records.iter().any(|record| !record.is_null()) {
            if sif_records.iter().any(|record| record.is_null()) {
                let message = format!(
                    "{entity} gives only one of modifiers.{} and modifiers.{}; the pair is written together as BSIF \
                     or not at all",
                    BEND_SIF_FIELDS[0], BEND_SIF_FIELDS[1]
                );
                self.refuse("MBF-SI-BEND-SIF-INCOMPLETE", &entity, &message);
                return Ok(());
            }
            let first = self.quantity(sif_records[0], DIMENSIONLESS, "1", &entity, BEND_SIF_FIELDS[0])?;
            let second = self.quantity(sif_records[1], DIMENSIONLESS, "1", &entity, BEND_SIF_FIELDS[1])?;
            sifs = Some([first, second]);
        }
        let (Some(factor), Some(radius)) = (factor, radius) else { return Ok(()) };
        let sifs = match sifs {
            None => None,
            Some([Some(a), Some(b)]) => Some([a, b]),
            Some(_) => return Ok(()),
        };
        if !(factor > 0.0) || !(radius > 0.0) {
            self.diag("MBF-SI-VALUE-OUT-OF-RANGE", "error", &entity, &format!("{entity} flexibility factor and bend radius must be positive"));
            return Ok(());
        }
        if let Some([a, b]) = sifs {
            if !(a > 0.0 && b > 0.0) {
                let message = format!("{entity} directional SIFs must be positive (got {}, {})", num(a), num(b));
                self.diag("MBF-SI-VALUE-OUT-OF-RANGE", "error", &entity, &message);
                return Ok(());
            }
        }
        if !is_vector(&self.pipes.get(&pipe_id).expect("pipe").y_reference)? {
            self.invalid(&entity, &format!("pipe:{pipe_id} needs a numeric y_reference to fix the bend plane"));
            return Ok(());
        }
        let component_id = component.get_or_null("id").clone();
        self.bends.insert(
            pipe_id,
            Bend { component: component_id, entity, radius, k: factor, angle, sifs, ti: None, theta: None },
        );
        Ok(())
    }

    pub fn read_supports(&mut self) -> R<()> {
        for support in self.list(self.model.get_or_null("supports"), "model.supports") {
            let Some(id) = support.get("id").and_then(Value::as_str) else {
                self.invalid("model.supports", "each support needs a string id");
                continue;
            };
            let id = id.to_string();
            let entity = format!("support:{id}");
            if self.supports.contains_key(&id) {
                self.invalid(&entity, &format!("support id {} is repeated", crate::pystr::py_repr_str(&id)));
                continue;
            }
            let node_value = support.get_or_null("node");
            let Some(node) = node_value.as_str().filter(|node| self.nodes.contains_key(&node.to_string())).map(String::from) else {
                let message = format!("{entity} is at unknown node {}", repr(node_value)?);
                self.diag("MBF-SI-REFERENCE-UNRESOLVED", "error", &entity, &message);
                continue;
            };
            let family = support.get_or_null("family");
            if family.as_str() == Some("spring") && support.get_or_null("hanger").is_null() && support.get_or_null("nonlinear").is_null() {
                self.read_spring(&support, &id, &entity, &node)?;
                continue;
            }
            let special: Vec<&str> =
                ["stiffness", "hanger", "nonlinear"].into_iter().filter(|name| !support.get_or_null(name).is_null()).collect();
            hashable(family)?;
            let rigid = family.is_null() || str_in(family, &RIGID_FAMILIES);
            if !rigid || !special.is_empty() {
                let what = if !rigid { format!("family {}", repr(family)?) } else { special.join(", ") };
                let message = format!(
                    "{entity} ({what}) is not a rigid restraint or a spring; hangers and nonlinear or gapped supports \
                     are not exported by the pilot writer"
                );
                self.refuse("MBF-SI-SUPPORT-UNSUPPORTED", &entity, &message);
                continue;
            }
            let Value::List(restraints) = support.get_or_null("restraints") else {
                self.invalid(&entity, &format!("{entity} restraints must be a list"));
                continue;
            };
            let mut dofs = BTreeSet::new();
            let mut broke = false;
            for token in restraints {
                hashable(token)?;
                let Some(dof) = known_dof(token) else {
                    let message = format!("{entity} restraint {} is not a frame DOF", repr(token)?);
                    self.invalid(&entity, &message);
                    broke = true;
                    break;
                };
                dofs.insert(dof);
            }
            if broke {
                continue;
            }
            if dofs.is_empty() {
                self.omit("MBF-SI-SUPPORT-EMPTY", &entity, &format!("{entity} restrains no DOF and is not exported"));
                continue;
            }
            let family = family.as_str().map(String::from);
            if family.as_deref() == Some("anchor") && dofs.len() != 6 {
                let listed: Vec<&str> = dofs.iter().copied().collect();
                let message = format!("{entity} has family anchor but restrains only {}", listed.join(", "));
                self.refuse("MBF-SI-SUPPORT-UNSUPPORTED", &entity, &message);
                continue;
            }
            self.node_restraints.entry(node.clone()).or_default().extend(dofs.iter().copied());
            self.node_supports.entry(node.clone()).or_default().push(id.clone());
            self.supports.insert(id, Support { node, dofs, family });
        }
        let restrained: Vec<(String, BTreeSet<&'static str>)> =
            self.node_restraints.iter().map(|(node, dofs)| (node.clone(), dofs.clone())).collect();
        for (node, dofs) in restrained {
            let rotations: Vec<&str> = dofs.iter().copied().filter(|dof| dof.starts_with('R')).collect();
            if rotations.is_empty() || dofs.len() == 6 {
                continue;
            }
            for support_id in self.node_supports.get(&node).cloned().unwrap_or_default() {
                let rotational = self.supports.get(&support_id).expect("support").dofs.iter().any(|dof| dof.starts_with('R'));
                if rotational {
                    let message = format!(
                        "support:{support_id} at node {} restrains rotation ({}) without being a full anchor; the \
                         target tool does not accept rotational restraints (FIXR) in an imported MBF, so partial \
                         rotational restraints are not exported",
                        crate::pystr::py_repr_str(&node),
                        rotations.join(", ")
                    );
                    self.refuse("MBF-SI-SUPPORT-UNSUPPORTED", &format!("support:{support_id}"), &message);
                }
            }
        }
        let mut both: Vec<String> =
            self.node_springs.keys().filter(|node| self.node_restraints.contains_key(*node)).cloned().collect();
        both.sort();
        for node in both {
            let message = format!(
                "node {} carries springs and rigid restraints; a spring is written as a flexible anchor (KA with \
                 TRAK), which the importer reads alone at its node",
                crate::pystr::py_repr_str(&node)
            );
            self.refuse("MBF-SI-SUPPORT-UNSUPPORTED", &format!("node:{node}"), &message);
        }
        Ok(())
    }

    pub fn read_spring(&mut self, support: &Value, id: &str, entity: &str, node: &str) -> R<()> {
        let stiffness = support.get_or_null("stiffness");
        let restraints = support.get_or_null("restraints");
        let (Value::Dict(_), Value::List(restraints)) = (stiffness, restraints) else {
            self.invalid(entity, &format!("{entity} is a spring and needs a stiffness and a restraints list"));
            return Ok(());
        };
        let dof = known_dof(stiffness.get_or_null("dof"));
        let tokens: Vec<Option<&str>> = restraints.iter().map(known_dof).collect();
        let Some(dof) = dof.filter(|dof| tokens == vec![Some(*dof)]) else {
            let message = format!("{entity} is a spring whose restraints and stiffness must name the same single DOF");
            self.refuse("MBF-SI-SUPPORT-UNSUPPORTED", entity, &message);
            return Ok(());
        };
        if !DOFS[..3].contains(&dof) {
            let message = format!(
                "{entity} is a rotational spring ({dof}); ROTK's unit is open (TBD-17-04-010), so only translational \
                 springs are written"
            );
            self.refuse("MBF-SI-SUPPORT-UNSUPPORTED", entity, &message);
            return Ok(());
        }
        let Some(value) = self.quantity(stiffness.get_or_null("value"), STIFFNESS_TO_N_PER_MM, "N/mm", entity, "stiffness")? else {
            return Ok(());
        };
        if !(value > 0.0) {
            let message = format!("{entity} stiffness must be positive (the solve refuses a spring without one)");
            self.diag("MBF-SI-VALUE-OUT-OF-RANGE", "error", entity, &message);
            return Ok(());
        }
        let springs = self.node_springs.entry_or_insert_with(node.to_string(), OMap::new);
        if springs.contains_key(&dof) {
            let message = format!(
                "{entity} is a second spring on {dof} at node {}; TRAK carries one stiffness per DOF",
                crate::pystr::py_repr_str(node)
            );
            self.refuse("MBF-SI-SUPPORT-UNSUPPORTED", entity, &message);
            return Ok(());
        }
        springs.insert(dof, value);
        self.supports.insert(
            id.to_string(),
            Support { node: node.to_string(), dofs: [dof].into_iter().collect(), family: Some("spring".into()) },
        );
        Ok(())
    }

    pub fn read_load_cases(&mut self) -> R<()> {
        let mut kinds: OMap<String, String> = OMap::new();
        let mut seen: BTreeSet<String> = BTreeSet::new();
        for case in self.list(self.model.get_or_null("load_cases"), "model.load_cases") {
            let Some(case_id) = case.get("id").and_then(Value::as_str).map(String::from) else {
                self.invalid("model.load_cases", "each load case needs a string id");
                continue;
            };
            let entity = format!("load_case:{case_id}");
            if seen.contains(&case_id) {
                self.invalid(&entity, &format!("load case id {} is repeated", crate::pystr::py_repr_str(&case_id)));
                continue;
            }
            seen.insert(case_id.clone());
            let regions = self.read_regions(case.get_or_null("pressure_regions"), &entity)?;
            for name in ["equivalent_static", "modulus_basis_ref", "modulus_basis_temperature", "analysis_state"] {
                if !case.get_or_null(name).is_null() {
                    self.refuse("MBF-SI-LOAD-CASE-FEATURE-UNSUPPORTED", &entity, &format!("{entity} {name} is not exported"));
                }
            }
            let loads_value = case.get_or_null("primitive_loads");
            let loads = if loads_value.truthy() { loads_value.clone() } else { Value::List(Vec::new()) };
            let Value::List(loads) = loads else {
                self.invalid(&entity, &format!("{entity} primitive_loads must be a list"));
                continue;
            };
            // `categories`, a set: hashing an unhashable category raises.
            let mut categories: Vec<Value> = Vec::new();
            for load in &loads {
                if let Value::Dict(_) = load {
                    let category = load.get_or_null("category");
                    hashable(category)?;
                    if matches!(category, Value::Float(number) if number.is_nan()) {
                        // Python's set holds NaN categories by object identity, which a Value does not carry.
                        return Err(ImportError::Unsupported(format!(
                            "{entity} has a NaN load category, whose set membership the Python decides by object identity"
                        )));
                    }
                    if !categories.iter().any(|item| py_eq(item, category)) {
                        categories.push(category.clone());
                    }
                }
            }
            let unsupported: Vec<Value> = categories
                .iter()
                .filter(|item| !str_in(item, &MECHANICAL) && !str_in(item, &THERMAL))
                .cloned()
                .collect();
            let is_unsupported = |value: &Value| unsupported.iter().any(|item| py_eq(item, value));
            for load in &loads {
                if matches!(load, Value::Dict(_)) && is_unsupported(load.get_or_null("category")) {
                    // The Python subscripts `load["category"]` here: an absent key (not an explicit null) raises.
                    let category = load.get("category").ok_or_else(|| py(key_error("category")))?;
                    let code = if str_in(category, &["pressure", "hydrotest"]) {
                        "MBF-SI-PRESSURE-UNSUPPORTED"
                    } else {
                        "MBF-SI-LOAD-UNSUPPORTED"
                    };
                    let message = format!(
                        "load {} in {entity} has category {}, which the pilot writer does not export",
                        repr(load.get_or_null("id"))?,
                        repr(category)?
                    );
                    self.refuse(code, &format!("load:{case_id}/{}", text(load.get_or_null("id"))?), &message);
                }
            }
            if loads.is_empty() && regions.is_empty() {
                self.omit("MBF-SI-LOAD-CASE-EMPTY", &entity, &format!("{entity} has no primitive loads and is not exported"));
                continue;
            }
            let mechanical = categories.iter().any(|item| str_in(item, &MECHANICAL)) || !regions.is_empty();
            let thermal = categories.iter().any(|item| str_in(item, &THERMAL));
            if !regions.is_empty() && thermal {
                let message = format!(
                    "{entity} has pressure regions and thermal or displacement primitives; the target program applies \
                     Pressure 1 with {} and its operating cases, not with {}",
                    crate::pystr::py_repr_str(SUSTAINED),
                    crate::pystr::py_repr_str(EXPANSION)
                );
                self.refuse("MBF-SI-PRESSURE-CASE-UNSUPPORTED", &entity, &message);
                continue;
            }
            if mechanical && thermal {
                let message = format!(
                    "{entity} mixes mechanical (force, moment, weight) and thermal/displacement primitives; each case \
                     must map to exactly one of {} or {}",
                    crate::pystr::py_repr_str(SUSTAINED),
                    crate::pystr::py_repr_str(EXPANSION)
                );
                self.refuse("MBF-SI-LOAD-CASE-MAPPING-UNSUPPORTED", &entity, &message);
                continue;
            }
            if !unsupported.is_empty() && !(mechanical || thermal) {
                continue;
            }
            let label = if mechanical { SUSTAINED } else { EXPANSION };
            let other = kinds.iter().find(|(_, value)| value.as_str() == label).map(|(key, _)| key.clone());
            if let Some(other) = other {
                let message = format!(
                    "{entity} and load_case:{other} both map to {}; at most one case of each kind is exported",
                    crate::pystr::py_repr_str(label)
                );
                self.refuse("MBF-SI-LOAD-CASE-MAPPING-UNSUPPORTED", &entity, &message);
                continue;
            }
            kinds.insert(case_id.clone(), label.to_string());
            if !regions.is_empty() {
                for (region, pressure) in regions {
                    for pipe_id in region.get_or_null("member_pipe_ids").as_list().expect("members") {
                        self.pipe_pressure.insert(pipe_id.as_str().expect("str").to_string(), pressure);
                    }
                    self.regions.push(region);
                }
            }
            for load in &loads {
                if matches!(load, Value::Dict(_)) && !is_unsupported(load.get_or_null("category")) {
                    self.read_load(&case_id, label, load)?;
                }
            }
        }
        self.case_labels = kinds;
        self.round_sums();
        for combination in self.list(self.model.get_or_null("combinations"), "model.combinations") {
            let identity = match &combination {
                Value::Dict(_) => combination.get_or_null("id").clone(),
                _ => Value::Null,
            };
            let message = format!(
                "combination {} is not exported; the target program forms its own combinations",
                repr(&identity)?
            );
            self.omit("MBF-SI-COMBINATION-NOT-EXPORTED", &format!("combination:{}", text(&identity)?), &message);
        }
        Ok(())
    }

    /// `read_regions(regions, entity)`: the case's non-zero pressure regions with their pressure in bar.
    pub fn read_regions(&mut self, regions: &Value, entity: &str) -> R<Vec<(Value, f64)>> {
        let regions = match regions {
            Value::Null => return Ok(Vec::new()),
            Value::List(items) => items,
            _ => {
                self.invalid(entity, &format!("{entity} pressure_regions must be a list"));
                return Ok(Vec::new());
            }
        };
        if !regions.is_empty() && self.contract.is_none() {
            let message = format!(
                "{entity} has pressure_regions without the exact pressure contract, which are not solved and not \
                 exported"
            );
            self.refuse("MBF-SI-PRESSURE-UNSUPPORTED", entity, &message);
            return Ok(Vec::new());
        }
        let mut out = Vec::new();
        let mut owned: BTreeSet<String> = BTreeSet::new();
        for region in regions {
            let is_dict = matches!(region, Value::Dict(_));
            let identity = if is_dict { region.get_or_null("id") } else { &Value::Null };
            let place = format!("{entity}/region:{}", text(identity)?);
            let members = if is_dict { region.get_or_null("member_pipe_ids") } else { &Value::Null };
            let terminals = if is_dict { region.get_or_null("terminals") } else { &Value::Null };
            let valid = matches!(identity, Value::Str(_))
                && matches!(members, Value::List(items) if !items.is_empty()
                    && items.iter().all(|item| matches!(item, Value::Str(id) if self.pipes.contains_key(id) && !owned.contains(id)))
                    && items.iter().map(|item| item.as_str().expect("str")).collect::<BTreeSet<_>>().len() == items.len())
                && region.get_or_null("pressure_basis").as_str() == Some(PRESSURE_BASIS)
                && matches!(region.get_or_null("provenance"), Value::Str(_))
                && matches!(terminals, Value::List(items) if items.len() == 2 && items.iter().all(|item| {
                    matches!(item, Value::Dict(_))
                        && matches!(item.get_or_null("node_ref"), Value::Str(node) if self.nodes.contains_key(node))
                        && str_in(item.get_or_null("closure_transfer"), &CLOSURE_TRANSFERS)
                        && matches!(item.get_or_null("provenance"), Value::Str(text) if !py_strip(text).is_empty())
                }));
            if !valid {
                let message = format!(
                    "{place} needs an id, unique exported member pipes, the basis {PRESSURE_BASIS}, a provenance and \
                     two terminals at model nodes with a closure transfer and provenance"
                );
                self.invalid(&place, &message);
                continue;
            }
            if region.get_or_null("member_connector_ids").truthy() {
                self.refuse("MBF-SI-PRESSURE-UNSUPPORTED", &place, &format!("{place} has member connectors, which are not exported"));
                continue;
            }
            for item in members.as_list().expect("members") {
                owned.insert(item.as_str().expect("str").to_string());
            }
            let Some(value) = self.quantity_exact(region.get_or_null("pressure"), PRESSURE_TO_BAR, "bar", &place, "pressure")? else {
                continue;
            };
            if value.is_negative() {
                let message = format!("{place} has a negative (external) pressure differential, which is not exported");
                self.refuse("MBF-SI-PRESSURE-UNSUPPORTED", &place, &message);
                continue;
            }
            if value.is_zero() {
                let message = format!("{place} has zero pressure; it writes Pressure 1 = 0, which imports as no region");
                self.omit("MBF-SI-PRESSURE-REGION-ZERO", &place, &message);
                continue;
            }
            if let Some(pressure) = self.rounded(&value, &place, "pressure") {
                out.push((region.clone(), pressure));
            }
        }
        Ok(out)
    }

    /// `round_sums()`: each summed load rounded once.
    pub fn round_sums(&mut self) {
        let exact = [
            ("forces", std::mem::take(&mut self.forces_exact)),
            ("moments", std::mem::take(&mut self.moments_exact)),
            ("displacements", std::mem::take(&mut self.disp_exact)),
            ("rotations", std::mem::take(&mut self.rot_exact)),
        ];
        let mut rounded = Vec::new();
        for (name, store) in &exact {
            let mut out = OMap::new();
            for (node, values) in store.iter() {
                let mut sums = [0.0; 3];
                for (index, value) in values.iter().enumerate() {
                    sums[index] = self.rounded(value, &format!("node:{node}"), &format!("summed {name}")).unwrap_or(0.0);
                }
                out.insert(node.clone(), sums);
            }
            rounded.push(out);
        }
        let mut rounded = rounded.into_iter();
        self.forces = rounded.next().expect("forces");
        self.moments = rounded.next().expect("moments");
        self.anchor_disp = rounded.next().expect("displacements");
        self.anchor_rot = rounded.next().expect("rotations");
        for (name, weight) in [("weight per length", true), ("thermal change", false)] {
            let store = std::mem::take(if weight { &mut self.weight_exact } else { &mut self.delta_t_exact });
            let mut out = OMap::new();
            for (pipe, value) in store.iter() {
                out.insert(pipe.clone(), self.rounded(value, &format!("pipe:{pipe}"), &format!("summed {name}")).unwrap_or(0.0));
            }
            if weight {
                self.weight = out;
            } else {
                self.delta_t = out;
            }
        }
    }

    pub fn read_load(&mut self, case_id: &str, label: &str, load: &Value) -> R<()> {
        let load_id = load.get_or_null("id");
        let entity = format!("load:{case_id}/{}", text(load_id)?);
        let category = load.get_or_null("category").as_str().expect("a mechanical or thermal category").to_string();
        let target = load.get_or_null("target");
        if !matches!(target, Value::Dict(_)) {
            self.invalid(&entity, &format!("{entity} target must be an object"));
            return Ok(());
        }
        let direction = load.get_or_null("direction");
        let dimension = load.get_or_null("dimension");
        let effective = if !direction.is_null() {
            direction.clone()
        } else {
            match category.as_str() {
                "thermal" => Value::str("global_x"),
                "weight" => Value::str("global_y"),
                _ => Value::Null,
            }
        };
        let mut record = Dict::new();
        record.insert("load_case".into(), Value::str(case_id));
        record.insert("caepipe_load_case".into(), Value::str(label));
        record.insert("category".into(), Value::str(category.clone()));
        record.insert("direction".into(), effective);
        let known_node = |value: &Value, nodes: &OMap<String, V3>| matches!(value, Value::Str(id) if nodes.contains_key(id));
        let known_pipe = |value: &Value, pipes: &OMap<String, Pipe>| matches!(value, Value::Str(id) if pipes.contains_key(id));
        match category.as_str() {
            "concentrated_force" | "concentrated_moment" => {
                let force = category == "concentrated_force";
                let node = target.get_or_null("node");
                if target.get_or_null("type").as_str() != Some("node") || !known_node(node, &self.nodes) {
                    self.diag("MBF-SI-REFERENCE-UNRESOLVED", "error", &entity, &format!("{entity} must target a model node"));
                    return Ok(());
                }
                let node = node.as_str().expect("str").to_string();
                hashable(direction)?;
                let index = direction.as_str().and_then(|token| if force { force_direction(token) } else { moment_direction(token) });
                let wanted = if force { "force" } else { "moment" };
                let Some(index) = index.filter(|_| dimension.as_str() == Some(wanted)) else {
                    let message = format!(
                        "{entity} direction {} / dimension {} is not exported; {} only",
                        repr(direction)?,
                        repr(dimension)?,
                        if force { "global_x/y/z force" } else { "global RX/RY/RZ moment" }
                    );
                    self.refuse("MBF-SI-LOAD-UNSUPPORTED", &entity, &message);
                    return Ok(());
                };
                let (table, unit) = if force { (FORCE_TO_N, "N") } else { (MOMENT_TO_NM, "N*m") };
                let Some(value) = self.quantity_exact(load.get_or_null("magnitude"), table, unit, &entity, "magnitude")? else {
                    return Ok(());
                };
                let store = if force { &mut self.forces_exact } else { &mut self.moments_exact };
                let slot = &mut store.entry_or_insert_with(node.clone(), zeros)[index];
                *slot = slot.add(&value);
                record.insert("target".into(), dict![("node", Value::str(node))]);
                record.insert("mbf".into(), Value::str(if force { "FOR" } else { "MOM" }));
            }
            "weight" => {
                let pipe = target.get_or_null("pipe");
                if target.get_or_null("type").as_str() != Some("element") || !known_pipe(pipe, &self.pipes) {
                    self.diag("MBF-SI-REFERENCE-UNRESOLVED", "error", &entity, &format!("{entity} must target a model pipe segment"));
                    return Ok(());
                }
                let pipe = pipe.as_str().expect("str").to_string();
                if dimension.as_str() != Some("force_per_length") {
                    self.refuse("MBF-SI-LOAD-UNSUPPORTED", &entity, &format!("{entity} weight needs dimension force_per_length"));
                    return Ok(());
                }
                let Some(value) =
                    self.quantity_exact(load.get_or_null("magnitude"), FORCE_PER_LENGTH_TO_N_PER_M, "N/m", &entity, "magnitude")?
                else {
                    return Ok(());
                };
                if !in_str_set(direction, &["global_y", "GLOBAL_Y"])? || value > Q::zero() {
                    let message = format!(
                        "{entity} weight acts along {} with magnitude {} N/m; only global -Y weight (the target \
                         program's default gravity) is exported",
                        repr(direction)?,
                        num_q(&value)
                    );
                    self.refuse("MBF-SI-LOAD-UNSUPPORTED", &entity, &message);
                    return Ok(());
                }
                let total = self.weight_exact.get(&pipe).cloned().unwrap_or_else(Q::zero).add(&value);
                self.weight_exact.insert(pipe.clone(), total);
                record.insert("target".into(), dict![("pipe", Value::str(pipe))]);
                record.insert("mbf".into(), Value::str("MATERIAL weight density"));
            }
            "thermal" => {
                let pipe = target.get_or_null("pipe");
                if target.get_or_null("type").as_str() != Some("element") || !known_pipe(pipe, &self.pipes) {
                    self.diag("MBF-SI-REFERENCE-UNRESOLVED", "error", &entity, &format!("{entity} must target a model pipe segment"));
                    return Ok(());
                }
                let pipe = pipe.as_str().expect("str").to_string();
                if !in_str_set(dimension, &["temperature_change", "temperature_interval"])? {
                    self.refuse("MBF-SI-LOAD-UNSUPPORTED", &entity, &format!("{entity} thermal needs a temperature-interval dimension"));
                    return Ok(());
                }
                let Some(value) =
                    self.quantity_exact(load.get_or_null("magnitude"), TEMPERATURE_INTERVAL_TO_C, "C", &entity, "magnitude")?
                else {
                    return Ok(());
                };
                let material_id = self.pipes.get(&pipe).expect("pipe").material.clone();
                if !value.is_zero() && !self.materials.get(&material_id).expect("material").has_alpha {
                    let message = format!(
                        "{entity} changes the temperature of pipe:{pipe}, whose material {} has no \
                         thermal_expansion_coefficient",
                        crate::pystr::py_repr_str(&material_id)
                    );
                    self.refuse("MBF-SI-MATERIAL-EXPANSION-MISSING", &entity, &message);
                    return Ok(());
                }
                let total = self.delta_t_exact.get(&pipe).cloned().unwrap_or_else(Q::zero).add(&value);
                self.delta_t_exact.insert(pipe.clone(), total);
                record.insert("target".into(), dict![("pipe", Value::str(pipe))]);
                record.insert("mbf".into(), Value::str("LOADS Temperature 1"));
            }
            _ => {
                let support_id = target.get_or_null("support");
                let dof = known_dof(target.get_or_null("dof"));
                let support = match support_id {
                    Value::Str(id) => self.supports.get(id).map(|support| (support.node.clone(), support.dofs.clone())),
                    _ => None,
                };
                let support = support.filter(|_| target.get_or_null("type").as_str() == Some("support"));
                let Some((support_node, support_dofs)) = support else {
                    let message = format!("{entity} must target an exported rigid support (got {})", repr(target)?);
                    self.refuse("MBF-SI-IMPOSED-DISPLACEMENT-UNSUPPORTED", &entity, &message);
                    return Ok(());
                };
                let Some(dof) = dof.filter(|dof| known_dof(direction) == Some(*dof)) else {
                    self.invalid(&entity, &format!("{entity} target dof and direction must name the same frame DOF"));
                    return Ok(());
                };
                if !self.is_anchor(&support_node) || !support_dofs.contains(dof) {
                    let message = format!(
                        "{entity} moves support {}, which does not map to an anchor restraining {dof}; only anchor \
                         displacements are exported",
                        repr(support_id)?
                    );
                    self.refuse("MBF-SI-IMPOSED-DISPLACEMENT-UNSUPPORTED", &entity, &message);
                    return Ok(());
                }
                let translation = dof.starts_with('U');
                let wanted = if translation { "displacement" } else { "rotation" };
                if dimension.as_str() != Some(wanted) {
                    self.refuse("MBF-SI-LOAD-UNSUPPORTED", &entity, &format!("{entity} {dof} needs dimension {wanted}"));
                    return Ok(());
                }
                let (table, unit) = if translation { (LENGTH_TO_MM, "mm") } else { (ROTATION_TO_DEG, "deg") };
                let Some(value) = self.quantity_exact(load.get_or_null("magnitude"), table, unit, &entity, "magnitude")? else {
                    return Ok(());
                };
                let index = DOFS.iter().position(|item| *item == dof).expect("dof") % 3;
                let store = if translation { &mut self.disp_exact } else { &mut self.rot_exact };
                let slot = &mut store.entry_or_insert_with(support_node, zeros)[index];
                *slot = slot.add(&value);
                record.insert("target".into(), dict![("support", support_id.clone()), ("dof", Value::str(dof))]);
                record.insert("mbf".into(), Value::str(if translation { "D" } else { "ROT" }));
            }
        }
        self.load_records.insert(format!("{case_id}/{}", text(load_id)?), record);
        Ok(())
    }

    pub fn read_other_members(&mut self) -> R<()> {
        if !self.model.get_or_null("model_settings").is_null() {
            self.omit(
                "MBF-SI-MODEL-SETTINGS-NOT-EXPORTED",
                "model.model_settings",
                "model_settings (input validation settings) are not exported",
            );
        }
        self.sweep_fields()
    }

    /// Design §17.1: every non-empty model member and field no reader writes, checks or refuses is listed.
    pub fn sweep_fields(&mut self) -> R<()> {
        let model = self.model;
        for (name, value) in model.as_dict().expect("dict").iter() {
            if !HANDLED_MEMBERS.contains(&name.as_str()) && present(value) {
                let message = format!("model.{name} is not exported; {}", omitted_reason(name));
                self.omit("MBF-SI-MEMBER-NOT-EXPORTED", &format!("model.{name}"), &message);
            }
        }
        let used: BTreeSet<&String> = self.pipes.values().map(|pipe| &pipe.material).collect();
        let unused: Vec<String> = self.materials.keys().filter(|id| !used.contains(id)).cloned().collect();
        if !unused.is_empty() {
            let message = format!(
                "materials {} are not exported; the MBF writes only the materials its pipe segments use",
                unused.join(", ")
            );
            self.omit("MBF-SI-FIELD-NOT-EXPORTED", "model.materials", &message);
        }
        let bound: BTreeSet<&str> = self.pipes.values().filter_map(|pipe| pipe.section_ref.as_deref()).collect();
        let mut unbound: Vec<&str> = Vec::new();
        for section in &self.library {
            let id = section.get_or_null("id").as_str().expect("str");
            if !bound.contains(id) && !unbound.contains(&id) {
                unbound.push(id);
            }
        }
        if !unbound.is_empty() {
            let message = format!(
                "sections {} are not exported; the MBF writes only the sections its pipe segments reference",
                unbound.join(", ")
            );
            self.omit("MBF-SI-FIELD-NOT-EXPORTED", "model.sections", &message);
        }
        let mut found: OMap<String, Vec<String>> = OMap::new();
        fn sweep(found: &mut OMap<String, Vec<String>>, record: &Value, path: &str, entity: &str) {
            if let Value::Dict(dict) = record {
                let handled = handled_fields(path);
                for (key, value) in dict.iter() {
                    if !handled.contains(&key.as_str()) && present(value) {
                        found.entry_or_insert_with(format!("{path}.{key}"), Vec::new).push(entity.to_string());
                    }
                }
            }
        }
        let project = model.get_or_null("project");
        sweep(&mut found, project, "project", "project");
        if let Some(Value::Dict(units)) = project.get("units") {
            let mut kept = Vec::new();
            for (key, value) in units.iter() {
                let imported = IMPORTED_PROJECT_UNITS.iter().find(|(name, _)| name == key).map(|(_, unit)| *unit);
                let same = match imported {
                    Some(unit) => value.as_str() == Some(unit),
                    None => value.is_null(),
                };
                if !same {
                    kept.push(format!("{key} {}", text(value)?));
                }
            }
            if !kept.is_empty() {
                found.insert("project.units".into(), kept);
            }
        }
        let prefixes = [
            ("materials", "material"), ("sections", "section"), ("nodes", "node"), ("pipe_segments", "pipe"), ("supports", "support"),
            ("components", "component"), ("load_cases", "load_case"),
        ];
        for (member, prefix) in prefixes {
            let Some(Value::List(records)) = model.get(member) else { continue };
            for record in records {
                if !matches!(record, Value::Dict(_)) {
                    continue;
                }
                let entity = format!("{prefix}:{}", text(record.get_or_null("id"))?);
                sweep(&mut found, record, member, &entity);
                if member == "pipe_segments" {
                    sweep(&mut found, record.get_or_null("section"), "pipe_segments.section", &entity);
                } else if member == "components" && record.get_or_null("kind").as_str() == Some("bend") {
                    for part in ["geometry", "modifiers", "mechanics_interface"] {
                        sweep(&mut found, record.get_or_null(part), &format!("components.{part}"), &entity);
                    }
                } else if member == "load_cases" {
                    for (part, kind) in [("primitive_loads", "load"), ("pressure_regions", "region")] {
                        let Some(Value::List(items)) = record.get(part) else { continue };
                        for item in items {
                            let label = if kind == "load" {
                                format!("load:{}/", text(record.get_or_null("id"))?)
                            } else {
                                format!("{entity}/region:")
                            };
                            let id = match item {
                                Value::Dict(_) => text(item.get_or_null("id"))?,
                                _ => "None".into(),
                            };
                            sweep(&mut found, item, &format!("load_cases.{part}"), &format!("{label}{id}"));
                        }
                    }
                }
            }
        }
        for (path, entities) in found.iter() {
            if path == "project.units" {
                let imported: Vec<String> = IMPORTED_PROJECT_UNITS.iter().map(|(key, value)| format!("{key} {value}")).collect();
                let message = format!(
                    "project.units ({}) are not kept; values are written in the MBF's SI units, and the import sets the \
                     project units {}",
                    entities.join(", "),
                    imported.join(", ")
                );
                self.omit("MBF-SI-FIELD-NOT-EXPORTED", "model.project.units", &message);
                continue;
            }
            let field = path.split_once('.').map(|(_, rest)| rest).unwrap_or("");
            let message = format!("{field} of {} is not exported; {}", entities.join(", "), omitted_reason(path));
            self.omit("MBF-SI-FIELD-NOT-EXPORTED", &format!("model.{path}"), &message);
        }
        Ok(())
    }
}
