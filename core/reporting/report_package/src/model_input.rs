//! The model-input projection `P(M, C)` and its binding (RT-R4 slice C5;
//! DEL-08-01 Design `report-section-composer.md` §2.7).
//!
//! [`model_input_tables`] projects the bound model `M` for the model-input
//! ids the composition `C` lists: one member per listed id, named by the id;
//! each table lists rows in the model's array order (no sorting) with exactly
//! the §2.7 columns. A cell is a stored string, copied; a stored number as its
//! canonical number text (`canonical_number`, the row contract's `value_text`
//! rule); a stored quantity (`value` and `unit`) as `"<number> <unit>"`,
//! unconverted; or null for an absent member. The desktop mirror is
//! `modelInputTables.ts`, held equal by
//! `fixtures/reports/invented/report_model_input_vectors.json`.
//!
//! [`check_model_input_tables`] is the binding every boundary that holds `M`
//! runs (native package save on routes 009 and 010, the exact native render):
//! it re-derives `P(M, C)` from `M` and the report's own composition and
//! requires canonical equality with the report's `model_input_tables`,
//! refusing tables without a listing id and a listing id without tables. The
//! user-release derivation writes `P(M, C)` into the derived request, so its
//! whole-request equality binds it. Every refusal names a fixed text or a
//! JSON pointer, never a value.
//!
//! Standard claim fence applies (claims taxonomy per DEC-081).

use crate::user_release::first_difference_shown;
use open_pipe_stress_canonical_json::canonical_number;
use open_pipe_stress_report_renderer::model_input::{
    is_model_input, INPUT_GEOMETRY, INPUT_LOADS, INPUT_MATERIALS_SECTIONS, INPUT_SUPPORTS, LOAD_COMBINATIONS,
    MODEL_INPUT_SECTIONS, MODEL_INPUT_UNBOUND,
};
use serde_json::{Map, Value};

fn unprojectable(pointer: &str, what: &str) -> String {
    format!("{MODEL_INPUT_UNBOUND}: the model's {pointer} is not {what}, so its model-input table cannot be projected")
}

/// `value` at `pointer` (for refusals), as a cell.
fn cell(value: Option<&Value>, pointer: &str) -> Result<Value, String> {
    match value {
        None | Some(Value::Null) => Ok(Value::Null),
        Some(Value::String(text)) => Ok(Value::String(text.clone())),
        Some(Value::Number(number)) => Ok(Value::String(canonical_number(number))),
        Some(Value::Object(object)) => match (object.get("value"), object.get("unit")) {
            (Some(Value::Number(number)), Some(Value::String(unit))) => {
                Ok(Value::String(format!("{} {unit}", canonical_number(number))))
            }
            _ => Err(unprojectable(pointer, "a stored string, number, quantity or null")),
        },
        Some(_) => Err(unprojectable(pointer, "a stored string, number, quantity or null")),
    }
}

/// The member `key` of `object` at `pointer`, as a cell.
fn member(object: &Value, pointer: &str, key: &str) -> Result<Value, String> {
    cell(object.get(key), &format!("{pointer}/{key}"))
}

/// The object at `pointer`, or none when absent or null.
fn optional_object<'a>(value: Option<&'a Value>, pointer: &str) -> Result<Option<&'a Value>, String> {
    match value {
        None | Some(Value::Null) => Ok(None),
        Some(object @ Value::Object(_)) => Ok(Some(object)),
        Some(_) => Err(unprojectable(pointer, "an object")),
    }
}

/// The model's list `key`; absent or null is empty.
fn list<'a>(model: &'a Value, key: &str) -> Result<&'a [Value], String> {
    match model.get(key) {
        None | Some(Value::Null) => Ok(&[]),
        Some(Value::Array(items)) => Ok(items),
        Some(_) => Err(unprojectable(&format!("/{key}"), "a list")),
    }
}

/// Each element of a list must be an object.
fn objects<'a>(items: &'a [Value], pointer: &str) -> Result<Vec<(String, &'a Value)>, String> {
    items
        .iter()
        .enumerate()
        .map(|(index, item)| {
            let at = format!("{pointer}/{index}");
            if item.is_object() {
                Ok((at, item))
            } else {
                Err(unprojectable(&at, "an object"))
            }
        })
        .collect()
}

/// The strings of a list member joined by `separator`; null when absent.
fn joined(object: &Value, pointer: &str, key: &str, separator: &str) -> Result<Value, String> {
    let at = format!("{pointer}/{key}");
    match object.get(key) {
        None | Some(Value::Null) => Ok(Value::Null),
        Some(Value::Array(items)) => {
            let texts = items
                .iter()
                .enumerate()
                .map(|(index, item)| item.as_str().ok_or_else(|| unprojectable(&format!("{at}/{index}"), "a string")))
                .collect::<Result<Vec<&str>, String>>()?;
            Ok(Value::String(texts.join(separator)))
        }
        Some(_) => Err(unprojectable(&at, "a list")),
    }
}

fn row(cells: Vec<(&str, Value)>) -> Value {
    Value::Object(cells.into_iter().map(|(key, value)| (key.to_string(), value)).collect())
}

/// A combination's operands (§2.7): each term as `"<factor> × <load_case or
/// combination>"` joined by `"; "`; else `"<minuend_id> − <subtrahend_id>"`;
/// else `operand_ids` joined by `", "`; else null.
fn operands(combination: &Value, pointer: &str) -> Result<Value, String> {
    match combination.get("terms") {
        None | Some(Value::Null) => {}
        Some(Value::Array(terms)) => {
            let mut parts = Vec::with_capacity(terms.len());
            for (index, term) in terms.iter().enumerate() {
                let at = format!("{pointer}/terms/{index}");
                let factor = match term.get("factor") {
                    Some(Value::Number(number)) => canonical_number(number),
                    _ => return Err(unprojectable(&format!("{at}/factor"), "a number")),
                };
                let operand = match (term.get("load_case"), term.get("combination")) {
                    (Some(Value::String(id)), _) => id,
                    (None | Some(Value::Null), Some(Value::String(id))) => id,
                    _ => return Err(unprojectable(&at, "a term naming a load case or a combination")),
                };
                parts.push(format!("{factor} \u{00d7} {operand}"));
            }
            return Ok(Value::String(parts.join("; ")));
        }
        Some(_) => return Err(unprojectable(&format!("{pointer}/terms"), "a list")),
    }
    let minuend = member(combination, pointer, "minuend_id")?;
    let subtrahend = member(combination, pointer, "subtrahend_id")?;
    if let (Value::String(minuend), Value::String(subtrahend)) = (&minuend, &subtrahend) {
        return Ok(Value::String(format!("{minuend} \u{2212} {subtrahend}")));
    }
    joined(combination, pointer, "operand_ids", ", ")
}

fn load_combinations(model: &Value) -> Result<Value, String> {
    let rows = objects(list(model, "combinations")?, "/combinations")?
        .into_iter()
        .map(|(at, combination)| {
            Ok(row(vec![
                ("id", member(combination, &at, "id")?),
                ("label", member(combination, &at, "label")?),
                ("basis", member(combination, &at, "basis")?),
                ("operands", operands(combination, &at)?),
                ("mode", member(combination, &at, "mode")?),
            ]))
        })
        .collect::<Result<Vec<Value>, String>>()?;
    Ok(Value::Array(rows))
}

fn input_geometry(model: &Value) -> Result<Value, String> {
    let nodes = objects(list(model, "nodes")?, "/nodes")?
        .into_iter()
        .map(|(at, node)| {
            let position_at = format!("{at}/position");
            let position = optional_object(node.get("position"), &position_at)?;
            let coordinate = |key: &str| match position {
                Some(position) => member(position, &position_at, key),
                None => Ok(Value::Null),
            };
            Ok(row(vec![
                ("id", member(node, &at, "id")?),
                ("label", member(node, &at, "label")?),
                ("x", coordinate("x")?),
                ("y", coordinate("y")?),
                ("z", coordinate("z")?),
            ]))
        })
        .collect::<Result<Vec<Value>, String>>()?;
    let pipes = objects(list(model, "pipe_segments")?, "/pipe_segments")?
        .into_iter()
        .map(|(at, pipe)| {
            let section_at = format!("{at}/section");
            let section = optional_object(pipe.get("section"), &section_at)?;
            let quantity = |key: &str| match section {
                Some(section) => member(section, &section_at, key),
                None => Ok(Value::Null),
            };
            Ok(row(vec![
                ("id", member(pipe, &at, "id")?),
                ("label", member(pipe, &at, "label")?),
                ("from", member(pipe, &at, "from")?),
                ("to", member(pipe, &at, "to")?),
                ("material", member(pipe, &at, "material")?),
                ("outside_diameter", quantity("outside_diameter")?),
                ("wall_thickness", quantity("wall_thickness")?),
                ("section_ref", member(pipe, &at, "section_ref")?),
            ]))
        })
        .collect::<Result<Vec<Value>, String>>()?;
    let components = objects(list(model, "components")?, "/components")?
        .into_iter()
        .map(|(at, component)| {
            Ok(row(vec![
                ("id", member(component, &at, "id")?),
                ("label", member(component, &at, "label")?),
                ("kind", member(component, &at, "kind")?),
            ]))
        })
        .collect::<Result<Vec<Value>, String>>()?;
    Ok(row(vec![("nodes", Value::Array(nodes)), ("pipes", Value::Array(pipes)), ("components", Value::Array(components))]))
}

fn input_materials_sections(model: &Value) -> Result<Value, String> {
    let materials = objects(list(model, "materials")?, "/materials")?
        .into_iter()
        .map(|(at, material)| {
            Ok(row(vec![
                ("id", member(material, &at, "id")?),
                ("label", member(material, &at, "label")?),
                ("elastic_modulus", member(material, &at, "elastic_modulus")?),
                ("shear_modulus", member(material, &at, "shear_modulus")?),
                ("poisson_ratio", member(material, &at, "poisson_ratio")?),
                ("thermal_expansion_coefficient", member(material, &at, "thermal_expansion_coefficient")?),
            ]))
        })
        .collect::<Result<Vec<Value>, String>>()?;
    let sections = objects(list(model, "sections")?, "/sections")?
        .into_iter()
        .map(|(at, section)| {
            let properties_at = format!("{at}/properties");
            let properties = optional_object(section.get("properties"), &properties_at)?;
            let quantity = |key: &str| match properties {
                Some(properties) => member(properties, &properties_at, key),
                None => Ok(Value::Null),
            };
            Ok(row(vec![
                ("id", member(section, &at, "id")?),
                ("name", member(section, &at, "name")?),
                ("section_type", member(section, &at, "section_type")?),
                ("outside_diameter", quantity("outside_diameter")?),
                ("wall_thickness", quantity("wall_thickness")?),
            ]))
        })
        .collect::<Result<Vec<Value>, String>>()?;
    Ok(row(vec![("materials", Value::Array(materials)), ("sections", Value::Array(sections))]))
}

fn input_supports(model: &Value) -> Result<Value, String> {
    let rows = objects(list(model, "supports")?, "/supports")?
        .into_iter()
        .map(|(at, support)| {
            let stiffness_at = format!("{at}/stiffness");
            let stiffness = match optional_object(support.get("stiffness"), &stiffness_at)? {
                None => Value::Null,
                Some(stiffness) => match (stiffness.get("dof"), member(stiffness, &stiffness_at, "value")?) {
                    (Some(Value::String(dof)), Value::String(quantity)) => Value::String(format!("{dof}: {quantity}")),
                    _ => return Err(unprojectable(&stiffness_at, "a stiffness with a degree of freedom and a value")),
                },
            };
            let nonlinear_at = format!("{at}/nonlinear");
            let behavior = match optional_object(support.get("nonlinear"), &nonlinear_at)? {
                None => Value::Null,
                Some(nonlinear) => member(nonlinear, &nonlinear_at, "behavior")?,
            };
            Ok(row(vec![
                ("id", member(support, &at, "id")?),
                ("label", member(support, &at, "label")?),
                ("node", member(support, &at, "node")?),
                ("family", member(support, &at, "family")?),
                ("restraints", joined(support, &at, "restraints", ", ")?),
                ("stiffness", stiffness),
                ("nonlinear_behavior", behavior),
            ]))
        })
        .collect::<Result<Vec<Value>, String>>()?;
    Ok(Value::Array(rows))
}

/// A load's target (§2.7): `"<type>:<node, pipe or support>"`, followed by
/// `" <dof>"` when the target names one; null when the target names none of
/// the three.
fn target(load: &Value, pointer: &str) -> Result<Value, String> {
    let at = format!("{pointer}/target");
    let Some(target) = optional_object(load.get("target"), &at)? else { return Ok(Value::Null) };
    let kind = member(target, &at, "type")?;
    let mut named = Value::Null;
    for key in ["node", "pipe", "support"] {
        named = member(target, &at, key)?;
        if !named.is_null() {
            break;
        }
    }
    let dof = member(target, &at, "dof")?;
    Ok(match (kind, named) {
        (Value::String(kind), Value::String(named)) => Value::String(match dof {
            Value::String(dof) => format!("{kind}:{named} {dof}"),
            _ => format!("{kind}:{named}"),
        }),
        _ => Value::Null,
    })
}

fn input_loads(model: &Value) -> Result<Value, String> {
    let mut rows = Vec::new();
    for (case_at, case) in objects(list(model, "load_cases")?, "/load_cases")? {
        let case_id = member(case, &case_at, "id")?;
        let loads_at = format!("{case_at}/primitive_loads");
        let loads: &[Value] = match case.get("primitive_loads") {
            None | Some(Value::Null) => &[],
            Some(Value::Array(loads)) => loads,
            Some(_) => return Err(unprojectable(&loads_at, "a list")),
        };
        for (at, load) in objects(loads, &loads_at)? {
            rows.push(row(vec![
                ("case_id", case_id.clone()),
                ("load_id", member(load, &at, "id")?),
                ("category", member(load, &at, "category")?),
                ("target", target(load, &at)?),
                ("direction", member(load, &at, "direction")?),
                ("magnitude", member(load, &at, "magnitude")?),
            ]));
        }
    }
    Ok(Value::Array(rows))
}

/// The model-input ids `composition` lists (a composition read by key; its
/// form is checked by the renderer and the derivation), in catalogue order.
pub fn listed_model_input_ids(composition: Option<&Value>) -> Vec<&'static str> {
    let listed: Vec<&str> = composition
        .and_then(|composition| composition.get("sections"))
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|section| section.get("section_id").and_then(Value::as_str))
        .filter(|id| is_model_input(id))
        .collect();
    MODEL_INPUT_SECTIONS.into_iter().filter(|id| listed.contains(id)).collect()
}

/// `P(M, C)`: none when `composition` lists no model-input id; otherwise one
/// member per listed id. Refuses, value-free, a model member that is not of
/// its stored form.
pub fn model_input_tables(model: &Value, composition: Option<&Value>) -> Result<Option<Value>, String> {
    let ids = listed_model_input_ids(composition);
    if ids.is_empty() {
        return Ok(None);
    }
    if !model.is_object() {
        return Err(unprojectable("root", "an object"));
    }
    let mut tables = Map::new();
    for id in ids {
        let table = match id {
            LOAD_COMBINATIONS => load_combinations(model)?,
            INPUT_GEOMETRY => input_geometry(model)?,
            INPUT_MATERIALS_SECTIONS => input_materials_sections(model)?,
            INPUT_SUPPORTS => input_supports(model)?,
            INPUT_LOADS => input_loads(model)?,
            _ => unreachable!("a model-input id"),
        };
        tables.insert(id.to_string(), table);
    }
    Ok(Some(Value::Object(tables)))
}

/// The binding (§2.7) of a renderer input `report` (the request's `report`
/// member or the render input) to the bound model: its `model_input_tables`
/// must be `P(M, C)` for its own `report_composition`. Tables without a
/// listing id, or a listing id without tables, are refused; a difference is
/// named by its first JSON pointer under `model_input_tables`, never by a
/// value.
pub fn check_model_input_tables(model: &Value, report: &Value) -> Result<(), String> {
    let composition = report.get("report_composition").filter(|value| !value.is_null());
    let found = report.get("model_input_tables").filter(|value| !value.is_null());
    let listed = !listed_model_input_ids(composition).is_empty();
    match (listed, found) {
        (false, None) => Ok(()),
        (false, Some(_)) => Err(format!(
            "{MODEL_INPUT_UNBOUND}: model_input_tables is carried, but the composition lists no model-input section"
        )),
        (true, None) => Err(format!(
            "{MODEL_INPUT_UNBOUND}: the composition lists a model-input section, but model_input_tables is absent"
        )),
        (true, Some(found)) => {
            let expected = model_input_tables(model, composition)?.expect("a listed id");
            match first_difference_shown(&expected, found) {
                None => Ok(()),
                Some(pointer) => Err(format!(
                    "{MODEL_INPUT_UNBOUND}: model_input_tables differs from its projection of the bound model at {pointer}"
                )),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn listing(ids: &[&str]) -> Value {
        json!({"composition_version": "1.0.0", "sections": ids.iter().map(|id| json!({"section_id": id, "cases": "all", "view": {"kind": "all"}})).collect::<Vec<_>>()})
    }

    #[test]
    fn cells_follow_the_stored_form() {
        let model = json!({
            "nodes": [{"id": "node:1", "position": {"x": 0, "y": -0.0, "z": 1e21}}, {"id": "node:2", "label": null}],
            "supports": [{"id": "support:1", "node": "node:1", "restraints": ["UX", "RZ"], "stiffness": {"dof": "UY", "value": {"value": 1.5e6, "unit": "N/m"}}, "nonlinear": {"behavior": "gap", "dof": "UY"}}],
            "combinations": [
                {"id": "c1", "basis": "mechanics", "terms": [{"load_case": "a", "factor": 1}, {"combination": "c0", "factor": -0.5}]},
                {"id": "c2", "basis": "result_state_subtraction", "minuend_id": "a", "subtrahend_id": "b"},
                {"id": "c3", "basis": "range_envelope", "operand_ids": ["a", "b"], "mode": "max"},
                {"id": "c4", "basis": "srss"}
            ],
            "load_cases": [{"id": "a", "primitive_loads": [
                {"id": "l1", "category": "concentrated_force", "target": {"type": "node", "node": "node:2"}, "direction": "global_x", "magnitude": {"value": 1000, "unit": "N"}},
                {"id": "l2", "category": "uniform_force", "target": {"type": "element", "pipe": "pipe:1"}, "direction": "global_z", "magnitude": {"value": -2.5, "unit": "N/m"}},
                {"id": "l3", "category": "concentrated_force", "target": {"type": "support", "support": "support:1", "dof": "UZ"}, "direction": "global_z", "magnitude": {"value": 1, "unit": "N"}}
            ]}, {"id": "b"}]
        });
        let tables = model_input_tables(&model, Some(&listing(&["input_loads", "input_geometry", "input_supports", "load_combinations"]))).unwrap().unwrap();
        assert_eq!(tables["input_geometry"]["nodes"][0], json!({"id": "node:1", "label": null, "x": "0", "y": "0", "z": "1e+21"}));
        assert_eq!(tables["input_geometry"]["nodes"][1], json!({"id": "node:2", "label": null, "x": null, "y": null, "z": null}));
        assert_eq!(tables["input_geometry"]["pipes"], json!([]));
        assert_eq!(tables["input_supports"][0], json!({"id": "support:1", "label": null, "node": "node:1", "family": null, "restraints": "UX, RZ", "stiffness": "UY: 1500000 N/m", "nonlinear_behavior": "gap"}));
        let operands: Vec<&Value> = tables["load_combinations"].as_array().unwrap().iter().map(|row| &row["operands"]).collect();
        assert_eq!(operands, [&json!("1 \u{00d7} a; -0.5 \u{00d7} c0"), &json!("a \u{2212} b"), &json!("a, b"), &Value::Null]);
        let targets: Vec<&Value> = tables["input_loads"].as_array().unwrap().iter().map(|row| &row["target"]).collect();
        assert_eq!(targets, [&json!("node:node:2"), &json!("element:pipe:1"), &json!("support:support:1 UZ")]);
        assert_eq!(tables["input_loads"][1]["magnitude"], json!("-2.5 N/m"));
        assert!(tables.get("input_materials_sections").is_none());
        assert_eq!(model_input_tables(&model, Some(&listing(&["result_rows_all"]))).unwrap(), None);
        assert_eq!(model_input_tables(&model, None).unwrap(), None);
    }

    #[test]
    fn a_member_of_another_form_is_refused_by_its_pointer_only() {
        let model = json!({"nodes": [{"id": "node:1", "label": true}]});
        let refusal = model_input_tables(&model, Some(&listing(&["input_geometry"]))).unwrap_err();
        assert_eq!(refusal, "REPORT-COMPOSITION-MODEL-INPUT-UNBOUND: the model's /nodes/0/label is not a stored string, number, quantity or null, so its model-input table cannot be projected");
        let quantity = json!({"materials": [{"id": "m", "elastic_modulus": {"value": "secret-text", "unit": "Pa"}}]});
        let refusal = model_input_tables(&quantity, Some(&listing(&["input_materials_sections"]))).unwrap_err();
        assert!(refusal.contains("/materials/0/elastic_modulus") && !refusal.contains("secret-text"));
    }

    #[test]
    fn the_binding_requires_the_projection_and_its_listing() {
        let model = json!({"nodes": [{"id": "node:1", "label": "n", "position": {"x": 1, "y": 2, "z": 3}}]});
        let composition = listing(&["input_geometry"]);
        let tables = model_input_tables(&model, Some(&composition)).unwrap().unwrap();
        let report = json!({"report_composition": composition, "model_input_tables": tables});
        assert_eq!(check_model_input_tables(&model, &report), Ok(()));
        assert_eq!(check_model_input_tables(&model, &json!({})), Ok(()));
        let mut edited = report.clone();
        edited["model_input_tables"]["input_geometry"]["nodes"][0]["x"] = json!("1.0001");
        assert_eq!(
            check_model_input_tables(&model, &edited).unwrap_err(),
            "REPORT-COMPOSITION-MODEL-INPUT-UNBOUND: model_input_tables differs from its projection of the bound model at /input_geometry/nodes/0/x"
        );
        let mut without_listing = report.clone();
        without_listing["report_composition"] = listing(&["result_rows_all"]);
        assert!(check_model_input_tables(&model, &without_listing).unwrap_err().contains("lists no model-input section"));
        let mut without_tables = report.clone();
        without_tables.as_object_mut().unwrap().remove("model_input_tables");
        assert!(check_model_input_tables(&model, &without_tables).unwrap_err().contains("model_input_tables is absent"));
        let mut tables_only = report;
        tables_only.as_object_mut().unwrap().remove("report_composition");
        assert!(check_model_input_tables(&model, &tables_only).unwrap_err().contains("lists no model-input section"));
    }
}
