//! `apply_library_record`: writes the members a library record maps to onto
//! an existing Material, Section or Component (W-17, DEL-03-01 Design
//! `apply-library-records.md` §2.2).
//!
//! The desktop builds the change from a stored, natively validated library
//! document; this module never sees the library. It checks what the applier
//! can judge:
//!
//! - the members written are the fixed copy table's destinations for the
//!   target's family (§2.3, §2.3b), plus the label, the short `provenance`
//!   string, the group source-reference strings and `provenance_record`;
//! - every quantity has the destination's dimension and sign (§2.4); nothing
//!   is converted;
//! - `before` is the current value of every named member that exists (absent
//!   members are created), so a stale change is refused;
//! - a member the solver requires for the family is never removed
//!   (`OP-LIBRARY-APPLY-REQUIRED-MEMBER`), and placement or topology
//!   references are not writable at all;
//! - the result is a valid material (`pressure_authoring::validate_material`)
//!   or pipe section; a Section change carries its bound pipes through
//!   `section_bindings::propagate`, and any supplement failure refuses the
//!   whole change.
//!
//! Wire: `target` names the entity; `field_path` is `library_record`; unit
//! `none`, dimension `dimensionless`. `after` is a JSON object of member path
//! (dot-separated) to the new value, or `null` to remove the member; `before`
//! is the JSON object of the same paths to their current values, omitting
//! members that are absent. Library payload never travels in the change.
use crate::geometry_operations::{GeometryEdit, GeometryError};
use crate::{canonical_json, DiffPreviewRow};
use open_pipe_stress_units::Dimension;
use serde_json::{Map, Value};

pub(crate) const CHANGE_KIND: &str = "apply_library_record";
pub(crate) const FIELD_PATH: &str = "library_record";

const BEND: &[&str] = &["bend", "elbow"];
const BRANCH: &[&str] = &["branch", "tee", "branch_connection"];
const RIGID: &[&str] = &["reducer", "valve", "flange", "rigid", "specialty"];
const EXPANSION_JOINT: &[&str] = &["expansion_joint"];
const BEND_OR_BRANCH: &[&str] = &["bend", "elbow", "branch", "tee", "branch_connection"];
const WITH_MODIFIERS: &[&str] = &[
    "bend",
    "elbow",
    "branch",
    "tee",
    "branch_connection",
    "reducer",
    "valve",
    "flange",
    "rigid",
    "specialty",
];

#[derive(Clone, Copy, PartialEq)]
enum Kind {
    /// `{value, unit}` of a dimension; `positive` refuses zero and negatives.
    Quantity { dimension: Dimension, positive: bool },
    /// A dimensionless user value (unit `1` or `none`), positive.
    Ratio,
    /// Poisson's ratio: unit `1`, -1 < nu < 0.5 (`pressure_authoring::validate_nu`).
    Poisson,
    /// Non-blank text.
    Text,
    /// The exact constitutive basis token.
    Basis,
    /// The structured provenance object (judged by the document validator).
    Record,
    /// A member the apply may only remove ("replace all values").
    RemoveOnly,
}

struct Member {
    path: &'static str,
    kind: Kind,
    /// Component families the member belongs to; empty for every family.
    families: &'static [&'static str],
    /// Never removed (the entity or the solver requires it).
    required: bool,
}

const fn m(path: &'static str, kind: Kind, families: &'static [&'static str], required: bool) -> Member {
    Member { path, kind, families, required }
}

const fn q(dimension: Dimension, positive: bool) -> Kind {
    Kind::Quantity { dimension, positive }
}

const MATERIAL: &[Member] = &[
    m("label", Kind::Text, &[], false),
    m("elastic_modulus", q(Dimension::Stress, true), &[], true),
    m("shear_modulus", q(Dimension::Stress, true), &[], false),
    m("poisson_ratio", Kind::Poisson, &[], false),
    m("constitutive_basis", Kind::Basis, &[], false),
    m("thermal_expansion_coefficient", q(Dimension::ThermalExpansionCoefficient, false), &[], false),
    m("temperature_points", Kind::RemoveOnly, &[], false),
    m("expansion_laws", Kind::RemoveOnly, &[], false),
    m("provenance", Kind::Text, &[], true),
    m("provenance_record", Kind::Record, &[], false),
];

const SECTION: &[Member] = &[
    m("name", Kind::Text, &[], true),
    m("properties.outside_diameter", q(Dimension::Length, true), &[], true),
    m("properties.wall_thickness", q(Dimension::Length, true), &[], true),
    m("provenance", Kind::Text, &[], true),
    m("provenance_record", Kind::Record, &[], false),
];

const COMPONENT: &[Member] = &[
    m("label", Kind::Text, &[], false),
    m("provenance", Kind::Text, &[], true),
    m("provenance_record", Kind::Record, &[], false),
    // Bend geometry group.
    m("geometry.bend_radius", q(Dimension::Length, true), BEND, true),
    // Required unless the bend is realized (checked below).
    m("geometry.bend_angle", q(Dimension::Angle, true), BEND, false),
    m("geometry.bend_geometry_source_reference", Kind::Text, BEND, true),
    // Branch geometry group.
    m("geometry.branch_run_size", q(Dimension::Length, true), BRANCH, true),
    m("geometry.branch_header_size", q(Dimension::Length, true), BRANCH, true),
    m("geometry.branch_connection_angle", q(Dimension::Angle, true), BRANCH, true),
    m("geometry.branch_reinforcement_area", q(Dimension::Area, true), BRANCH, false),
    m("geometry.branch_geometry_source_reference", Kind::Text, BRANCH, true),
    // Rigid geometry group.
    m("geometry.rigid_body_length", q(Dimension::Length, true), RIGID, true),
    m("geometry.end_a_size", q(Dimension::Length, true), RIGID, true),
    m("geometry.end_b_size", q(Dimension::Length, true), RIGID, true),
    m("geometry.weight", q(Dimension::Force, true), RIGID, true),
    m("geometry.rigid_component_source_reference", Kind::Text, RIGID, true),
    // Expansion-joint geometry group.
    m("geometry.effective_area", q(Dimension::Area, true), EXPANSION_JOINT, false),
    m("geometry.movement_limit", q(Dimension::Length, true), EXPANSION_JOINT, false),
    m("geometry.expansion_joint_source_reference", Kind::Text, EXPANSION_JOINT, true),
    // Modifiers group.
    m("modifiers.sif_user_value", Kind::Ratio, BEND, true),
    m("modifiers.flexibility_factor_user_value", Kind::Ratio, BEND_OR_BRANCH, true),
    m("modifiers.sif_in_plane_user_value", Kind::Ratio, BEND, false),
    m("modifiers.sif_out_of_plane_user_value", Kind::Ratio, BEND, false),
    m("modifiers.linear_stiffness_user_value", q(Dimension::LinearStiffness, true), RIGID, true),
    m("modifiers.rotational_stiffness_user_value", q(Dimension::RotationalStiffness, true), RIGID, true),
    m("modifiers.source_reference", Kind::Text, WITH_MODIFIERS, true),
];

fn members(object_type: &str) -> Option<(&'static str, &'static [Member])> {
    match object_type {
        "Material" => Some(("materials", MATERIAL)),
        "Section" => Some(("sections", SECTION)),
        "Component" => Some(("components", COMPONENT)),
        _ => None,
    }
}

/// How to proceed after each refusal (HELP_HUMAN, 2026-10-23: "Each refusal
/// must tell the user how to proceed"); appended to the cause.
pub(crate) fn next_step(code: &str) -> &'static str {
    match code {
        "OP-LIBRARY-APPLY-SHAPE-INVALID" | "OP-LIBRARY-APPLY-PAYLOAD-INVALID" | "OP-LIBRARY-APPLY-MEMBER-INVALID" => {
            "Prepare the apply again from the Library Manager, which builds this change."
        }
        "OP-TARGET-NOT-FOUND" => "Select an entity that is in the model and prepare the apply again.",
        "OP-LIBRARY-APPLY-KIND-MISMATCH" => "Choose a record for this component's kind, or select a component of the record's kind.",
        "OP-LIBRARY-APPLY-REQUIRED-MEMBER" => "Keep the member (choose \"keep them\"), or apply a record that supplies it.",
        "OP-LIBRARY-APPLY-VALUE-INVALID" => "Correct the value or its unit in the library, re-import it, and prepare the apply again.",
        "OP-LIBRARY-APPLY-SIF-PAIR" => {
            "Supply both directional SIF values, and apply them only to a bend realized as the curved-bend macro element."
        }
        "OP-STALE-BEFORE-VALUE" => "The model changed after the apply was prepared; prepare it again.",
        "OP-LIBRARY-APPLY-UNCHANGED" => "There is nothing to apply; choose a different record or leave the entity as it is.",
        "OP-LIBRARY-APPLY-MATERIAL-INVALID" => {
            "Choose the other material mode, or a record with a complete E/ν or E/G set, and prepare the apply again."
        }
        "OP-SECTION-GEOMETRY-INVALID" => "Choose a record whose wall thickness is less than half its outside diameter.",
        "OP-SECTION-BINDING-INVALID" => {
            "Adjust the bound pipes' mill tolerance or corrosion allowance, or detach them from the section, then apply again."
        }
        _ => "Prepare the apply again from the Library Manager.",
    }
}

fn err(code: &'static str, message: impl Into<String>, target: &str) -> GeometryError {
    let cause = message.into();
    let separator = if cause.ends_with('.') { " " } else { ". " };
    GeometryError { code, message: format!("{cause}{separator}{}", next_step(code)), affected_refs: vec![target.to_string()] }
}

fn get_path<'a>(entity: &'a Value, path: &str) -> Option<&'a Value> {
    path.split('.').try_fold(entity, |value, segment| value.get(segment))
}

fn write_path(entity: &mut Value, path: &str, value: Option<&Value>) -> Result<(), String> {
    let segments: Vec<&str> = path.split('.').collect();
    let (last, parents) = segments.split_last().expect("member paths are non-empty");
    let mut current = entity;
    for segment in parents {
        let Some(map) = current.as_object_mut() else {
            return Err(format!("`{path}` cannot be written: a parent is not an object"));
        };
        if !map.contains_key(*segment) {
            if value.is_none() {
                return Ok(());
            }
            map.insert((*segment).to_string(), Value::Object(Map::new()));
        }
        current = map.get_mut(*segment).expect("inserted above");
    }
    let Some(map) = current.as_object_mut() else {
        return Err(format!("`{path}` cannot be written: its parent is not an object"));
    };
    match value {
        Some(value) => {
            map.insert((*last).to_string(), value.clone());
        }
        None => {
            map.remove(*last);
        }
    }
    Ok(())
}

/// Drops `geometry` or `modifiers` left empty by removals, so a removal
/// leaves no empty shell behind.
fn drop_empty_groups(entity: &mut Value) {
    if let Some(map) = entity.as_object_mut() {
        for group in ["geometry", "modifiers"] {
            if map.get(group).and_then(Value::as_object).is_some_and(Map::is_empty) {
                map.remove(group);
            }
        }
    }
}

fn check_value(member: &Member, value: &Value) -> Result<(), String> {
    let path = member.path;
    match member.kind {
        Kind::Text => match value.as_str() {
            Some(text) if !text.trim().is_empty() => Ok(()),
            _ => Err(format!("`{path}` must be non-blank text.")),
        },
        Kind::Basis => (value.as_str() == Some("homogeneous_isotropic_E_nu_v1"))
            .then_some(())
            .ok_or_else(|| format!("`{path}` must be `homogeneous_isotropic_E_nu_v1`.")),
        Kind::Record => value
            .is_object()
            .then_some(())
            .ok_or_else(|| format!("`{path}` must be an object.")),
        Kind::RemoveOnly => Err(format!("`{path}` can only be removed by a library apply.")),
        Kind::Poisson => crate::pressure_authoring::validate_nu(value).map_err(|error| format!("`{path}`: {}", error.message)),
        Kind::Ratio | Kind::Quantity { .. } => {
            let record = value
                .as_object()
                .filter(|record| record.len() == 2 && record.contains_key("value") && record.contains_key("unit"))
                .ok_or_else(|| format!("`{path}` must be a quantity with exactly value and unit."))?;
            let number = record
                .get("value")
                .and_then(Value::as_f64)
                .filter(|number| number.is_finite())
                .ok_or_else(|| format!("`{path}` value must be a finite number."))?;
            let unit = record
                .get("unit")
                .and_then(Value::as_str)
                .ok_or_else(|| format!("`{path}` unit must be text."))?;
            let (dimension, positive) = match member.kind {
                Kind::Quantity { dimension, positive } => (dimension, positive),
                _ => (Dimension::Dimensionless, true),
            };
            let unit_ok = if member.kind == Kind::Ratio {
                matches!(unit, "1" | "none") || crate::unit_symbol_matches_dimension(unit, dimension)
            } else {
                crate::unit_symbol_matches_dimension(unit, dimension)
            };
            if !unit_ok {
                return Err(format!(
                    "`{path}` unit `{unit}` is not an accepted DEC-018 {} unit; nothing is converted.",
                    dimension.as_str()
                ));
            }
            if positive && number <= 0.0 {
                return Err(format!("`{path}` must be positive."));
            }
            Ok(())
        }
    }
}

fn is_realized_bend(component: &Value) -> bool {
    component.get("kind").and_then(Value::as_str).is_some_and(|kind| BEND.contains(&kind))
        && component.pointer("/mechanics_interface/solver_consumption").and_then(Value::as_str)
            == Some("curved_bend_macro_element")
}

/// Resolve one `apply_library_record` change against `model`.
pub(crate) fn resolve(model: &Value, intent: &Value) -> Result<GeometryEdit, GeometryError> {
    let text = |pointer: &str| intent.pointer(pointer).and_then(Value::as_str).unwrap_or("");
    let object_type = text("/target/object_type");
    let target = text("/target/ref");
    let Some((collection, table)) = members(object_type) else {
        return Err(err(
            "OP-LIBRARY-APPLY-SHAPE-INVALID",
            format!("`{CHANGE_KIND}` targets a Material, Section or Component, not `{object_type}`."),
            target,
        ));
    };
    if text("/change/field_path") != FIELD_PATH
        || text("/change/unit") != "none"
        || text("/change/dimension") != "dimensionless"
    {
        return Err(err(
            "OP-LIBRARY-APPLY-SHAPE-INVALID",
            format!("`{CHANGE_KIND}` carries field_path `{FIELD_PATH}`, unit none and dimension dimensionless."),
            target,
        ));
    }
    let entity = crate::find_entity(model, collection, target).ok_or_else(|| {
        err("OP-TARGET-NOT-FOUND", format!("{object_type} `{target}` was not found in the current model."), target)
    })?;
    let after: Value = serde_json::from_str(text("/change/after")).map_err(|_| {
        err("OP-LIBRARY-APPLY-PAYLOAD-INVALID", "The change's after must be a JSON object of member paths.", target)
    })?;
    let after = after
        .as_object()
        .filter(|after| !after.is_empty())
        .ok_or_else(|| err("OP-LIBRARY-APPLY-PAYLOAD-INVALID", "The change's after must be a non-empty JSON object of member paths.", target))?;

    let kind = entity.get("kind").and_then(Value::as_str).unwrap_or("");
    let realized = is_realized_bend(entity);
    let mut resolved: Vec<(&Member, Option<&Value>)> = Vec::new();
    for (path, value) in after {
        let Some(member) = table.iter().find(|member| member.path == path) else {
            return Err(err(
                "OP-LIBRARY-APPLY-MEMBER-INVALID",
                format!("`{path}` is not a member a library record can write on a {object_type}."),
                target,
            ));
        };
        if !member.families.is_empty() && !member.families.contains(&kind) {
            return Err(err(
                "OP-LIBRARY-APPLY-KIND-MISMATCH",
                format!("`{path}` belongs to the {} families; component `{target}` is a `{kind}`.", member.families.join("/")),
                target,
            ));
        }
        let required = member.required || (member.path == "geometry.bend_angle" && !realized);
        if value.is_null() {
            if required {
                return Err(err(
                    "OP-LIBRARY-APPLY-REQUIRED-MEMBER",
                    format!("`{path}` is required for this {object_type} and is never removed by a library apply."),
                    target,
                ));
            }
            resolved.push((member, None));
        } else {
            check_value(member, value).map_err(|message| err("OP-LIBRARY-APPLY-VALUE-INVALID", message, target))?;
            resolved.push((member, Some(value)));
        }
    }
    // The directional SIF pair: both or neither, on a realized curved bend only.
    let in_plane = after.get("modifiers.sif_in_plane_user_value");
    let out_of_plane = after.get("modifiers.sif_out_of_plane_user_value");
    if in_plane.is_some() != out_of_plane.is_some()
        || in_plane.is_some_and(Value::is_null) != out_of_plane.is_some_and(Value::is_null)
    {
        return Err(err(
            "OP-LIBRARY-APPLY-SIF-PAIR",
            "The in-plane and out-of-plane SIF values are written together or not at all.",
            target,
        ));
    }
    if in_plane.is_some_and(|value| !value.is_null()) && !realized {
        return Err(err(
            "OP-LIBRARY-APPLY-SIF-PAIR",
            format!("The directional SIF pair applies only to a bend realized as the curved-bend macro element; `{target}` is not."),
            target,
        ));
    }

    // Stale check: before is exactly the current value of every named member
    // that exists.
    let current: Map<String, Value> = after
        .keys()
        .filter_map(|path| get_path(entity, path).map(|value| (path.clone(), value.clone())))
        .collect();
    // Compared in canonical (RFC 8785) form: `2e11` and `200000000000` are
    // the same number.
    let same = |left: Option<&Value>, right: Option<&Value>| left.map(canonical_json) == right.map(canonical_json);
    let before: Option<Value> = serde_json::from_str(text("/change/before")).ok();
    if !before.as_ref().is_some_and(Value::is_object) || !same(before.as_ref(), Some(&Value::Object(current.clone()))) {
        return Err(err(
            "OP-STALE-BEFORE-VALUE",
            format!(
                "The change's before no longer matches the current members of `{target}` ({}).",
                canonical_json(&Value::Object(current))
            ),
            target,
        ));
    }
    if resolved.iter().all(|(member, value)| same(get_path(entity, member.path), *value)) {
        return Err(err(
            "OP-LIBRARY-APPLY-UNCHANGED",
            format!("Every member already holds the library value; `{target}` is unchanged."),
            target,
        ));
    }

    let mut candidate = model.clone();
    let next = crate::find_entity_mut(&mut candidate, collection, target).expect("found above");
    for (member, value) in &resolved {
        write_path(next, member.path, *value).map_err(|message| err("OP-LIBRARY-APPLY-PAYLOAD-INVALID", message, target))?;
    }
    drop_empty_groups(next);
    let next = next.clone();

    let mut diff_preview: Vec<DiffPreviewRow> = resolved
        .iter()
        .filter(|(member, value)| !same(get_path(entity, member.path), *value))
        .map(|(member, value)| {
            let (unit, dimension) = match (value, member.kind) {
                (Some(value), Kind::Quantity { dimension, .. }) => {
                    (value["unit"].as_str().unwrap_or("none").to_string(), dimension.as_str().to_string())
                }
                (Some(value), Kind::Ratio | Kind::Poisson) => {
                    (value["unit"].as_str().unwrap_or("none").to_string(), "dimensionless".to_string())
                }
                _ => ("none".to_string(), "dimensionless".to_string()),
            };
            DiffPreviewRow {
                entity_ref: target.to_string(),
                object_type: object_type.to_string(),
                field_path: member.path.to_string(),
                before: get_path(entity, member.path).map(canonical_json).unwrap_or_else(|| "not_present".into()),
                after: value.map(canonical_json).unwrap_or_else(|| "not_present".into()),
                unit,
                dimension,
                change_kind: CHANGE_KIND.to_string(),
            }
        })
        .collect();

    match object_type {
        "Material" => {
            crate::pressure_authoring::validate_material(&next).map_err(|error| {
                err(
                    "OP-LIBRARY-APPLY-MATERIAL-INVALID",
                    format!("The applied material would be invalid: {}", error.message),
                    target,
                )
            })?;
        }
        "Section" => {
            let mut checker = crate::Checker::new();
            crate::check_section_geometry(&next, target, &mut checker);
            if let Some(finding) = checker.diagnostics.first() {
                return Err(err("OP-SECTION-GEOMETRY-INVALID", finding.message.clone(), target));
            }
            // Every bound pipe's cached geometry follows the section; one
            // supplement failure refuses the whole change.
            let dependents = crate::section_bindings::propagate(model, &mut candidate, target)
                .map_err(|message| err("OP-SECTION-BINDING-INVALID", message, target))?;
            diff_preview.extend(dependents);
        }
        _ => {}
    }
    Ok(GeometryEdit { model: candidate, diff_preview })
}

/// True when the change writes any user-entered modifier member (k, a SIF,
/// a stiffness scaling or stiffness, or the modifier source; RT-E1 M2-D1).
pub(crate) fn writes_modifiers(intent: &Value) -> bool {
    intent
        .pointer("/change/after")
        .and_then(Value::as_str)
        .and_then(|after| serde_json::from_str::<Value>(after).ok())
        .and_then(|after| after.as_object().map(|o| o.keys().any(|k| k.starts_with("modifiers."))))
        .unwrap_or(false)
}

/// True when the change writes the user-entered bend flexibility factor k
/// (an agent never writes it, T4-I11 D-G).
pub(crate) fn writes_flexibility(intent: &Value) -> bool {
    intent
        .pointer("/change/after")
        .and_then(Value::as_str)
        .and_then(|after| serde_json::from_str::<Value>(after).ok())
        .is_some_and(|after| after.get("modifiers.flexibility_factor_user_value").is_some())
}

/// The isotropic shear modulus a library insert derives from E and ν (W-17
/// §2.3a), in Pa, bit for bit the value the exact route derives for the same
/// inputs:
///
/// - E is normalized to Pa by `open_pipe_stress_units::convert_for_dimension`
///   with the stress dimension and canonical unit, as the solver's
///   `normalize_quantity` does (identity for Pa, so no rounding then);
/// - G = E_Pa / (2 × (1 + ν)), evaluated in that order in binary64:
///   one rounding for 1 + ν, an exact doubling, one rounding for the quotient.
///   `IsotropicENu::new` (`product_physics/src/pressure_exact.rs`, called from
///   `pressure_material.rs`) evaluates the same order through its dyadic
///   `Scaled` type, whose add and divide round exactly as binary64 does for
///   normal values.
///
/// The value is stored in Pa (not E's entered unit): converting it back to
/// the entered unit and again to Pa on the pressure-free route could change
/// its last bit.
pub fn derived_shear_modulus_pa(elastic_modulus: f64, unit: &str, poisson_ratio: f64) -> Result<f64, String> {
    use open_pipe_stress_units::{canonical_unit, convert_for_dimension, unit_by_symbol};
    let from = unit_by_symbol(unit, Dimension::Stress).map_err(|error| error.to_string())?;
    let to = canonical_unit(Dimension::Stress).ok_or("no canonical stress unit")?;
    let e_pa = convert_for_dimension(elastic_modulus, Dimension::Stress, from, to).map_err(|error| error.to_string())?;
    if !e_pa.is_finite() || e_pa <= 0.0 || !poisson_ratio.is_finite() || poisson_ratio <= -1.0 || poisson_ratio >= 0.5 {
        return Err("E/ν requires finite E > 0 and -1 < ν < 0.5".into());
    }
    let g = e_pa / (2.0 * (1.0 + poisson_ratio));
    if !g.is_finite() || g <= 0.0 || !g.is_normal() {
        return Err("E/ν gives no representable positive G".into());
    }
    Ok(g)
}

#[cfg(test)]
mod tests {
    #[test]
    fn an_unlisted_code_still_tells_the_user_how_to_proceed() {
        assert_eq!(super::next_step("OP-SOMETHING-ELSE"), "Prepare the apply again from the Library Manager.");
    }
}
