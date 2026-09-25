#[allow(clippy::too_many_arguments)]
fn resolve_connect_pipe_run(
    model: &Value,
    target_ref: &str,
    field_path: &str,
    before: &str,
    after: &str,
    unit: &str,
    dimension: &str,
    object_type: &str,
    checker: &mut Checker,
) -> Option<Value> {
    if object_type != "Element" || field_path != "pipe_segments" {
        checker.schema_blocked = true;
        checker.push(
            "OP-CONNECT-PIPE-SHAPE-INVALID",
            "blocking",
            "Connect-pipe intents must target object_type `Element` with field_path `pipe_segments`.".to_string(),
            "Refresh the viewport connect-pipe intent from the explicit pipe connectivity form.",
            vec![target_ref.to_string()],
        );
        return None;
    }
    check_before("not_present", before, target_ref, field_path, checker);

    let stored_unit = value_at(model, &["project", "units", "length"]).and_then(Value::as_str);
    checker.unit_state = "passed";
    let Some(stored_unit) = stored_unit else {
        checker.unit_state = "blocked";
        checker.push(
            "OP-UNIT-METADATA-MISSING",
            "blocking",
            "Project length unit metadata is missing; explicit pipe geometry cannot be accepted.".to_string(),
            "Repair the model document's project.units.length metadata before creating pipe segments.",
            vec![target_ref.to_string()],
        );
        return None;
    };
    if !unit_symbol_matches_dimension(stored_unit, Dimension::Length) {
        checker.unit_state = "blocked";
        checker.push(
            "OP-UNIT-MISMATCH-CONVERSION-UNAVAILABLE",
            "blocking",
            format!("Project length unit metadata `{stored_unit}` is not an accepted DEC-018 length unit."),
            "Repair the model document's project.units.length metadata before creating pipe segments.",
            vec![target_ref.to_string()],
        );
        return None;
    }
    if !unit_symbol_matches_dimension(unit, Dimension::Length) {
        checker.unit_state = "blocked";
        checker.push(
            "OP-UNIT-MISMATCH-CONVERSION-UNAVAILABLE",
            "blocking",
            format!("Intent unit `{unit}` is not an accepted DEC-018 length unit; project length metadata is `{stored_unit}`."),
            "Select an accepted length unit from the DEC-018 catalog; no hidden fallback unit is supplied.",
            vec![target_ref.to_string()],
        );
        return None;
    }
    if dimension != "length" {
        checker.unit_state = "blocked";
        checker.push(
            "OP-UNIT-DIMENSION-UNKNOWN",
            "blocking",
            format!("Connect-pipe dimension `{dimension}` must be `length`."),
            "Emit pipe section geometry with explicit length dimension metadata.",
            vec![target_ref.to_string()],
        );
        return None;
    }
    if find_entity(model, "pipe_segments", target_ref).is_some() {
        checker.reference_state = "blocked";
        checker.push(
            "OP-TARGET-ALREADY-EXISTS",
            "blocking",
            format!("Pipe segment `{target_ref}` already exists in the current model."),
            "Choose a new stable pipe id; connect operations never overwrite existing entities.",
            vec![target_ref.to_string()],
        );
        return None;
    }

    let Ok(payload) = serde_json::from_str::<Value>(after) else {
        checker.push(
            "OP-CONNECT-PIPE-PAYLOAD-INVALID",
            "blocking",
            "Connect-pipe payload is not valid JSON.".to_string(),
            "Emit the explicit pipe connectivity payload as JSON in change.after.",
            vec![target_ref.to_string()],
        );
        return None;
    };
    let Some(record) = payload.as_object() else {
        checker.push(
            "OP-CONNECT-PIPE-PAYLOAD-INVALID",
            "blocking",
            "Connect-pipe payload must be a JSON object.".to_string(),
            "Emit id, label, from, to, section, material, y_reference, and provenance fields for the new pipe segment.",
            vec![target_ref.to_string()],
        );
        return None;
    };

    if record.contains_key("section_ref") {
        checker.push(
            "OP-CONNECT-PIPE-PAYLOAD-INVALID",
            "blocking",
            "Connect-pipe creates explicit local section inputs; use assign_section after creation to bind a shared section.".into(),
            "Create the pipe with local section inputs, then apply the explicit shared-section assignment.",
            vec![target_ref.to_string()],
        );
        return None;
    }

    let id = record
        .get("id")
        .and_then(Value::as_str)
        .unwrap_or("")
        .trim();
    let label = record
        .get("label")
        .and_then(Value::as_str)
        .unwrap_or("")
        .trim();
    let from = record
        .get("from")
        .and_then(Value::as_str)
        .unwrap_or("")
        .trim();
    let to = record
        .get("to")
        .and_then(Value::as_str)
        .unwrap_or("")
        .trim();
    let material = record
        .get("material")
        .and_then(Value::as_str)
        .unwrap_or("")
        .trim();
    let provenance = record
        .get("provenance")
        .and_then(Value::as_str)
        .unwrap_or("")
        .trim();
    let outside_diameter = dimensioned_quantity(
        record,
        &["section", "outside_diameter"],
        Dimension::Length,
        true,
    );
    let wall_thickness = dimensioned_quantity(
        record,
        &["section", "wall_thickness"],
        Dimension::Length,
        true,
    );
    let y_reference = vector_value(record, "y_reference");

    if id != target_ref
        || label.is_empty()
        || from.is_empty()
        || to.is_empty()
        || from == to
        || material.is_empty()
        || provenance.is_empty()
        || outside_diameter.is_none()
        || wall_thickness.is_none()
        || y_reference.is_none()
    {
        checker.push(
            "OP-CONNECT-PIPE-PAYLOAD-INVALID",
            "blocking",
            "Connect-pipe payload must include matching id, non-empty label/from/to/material/provenance, distinct endpoint nodes, positive OD/wall quantities in accepted DEC-018 length units, and a non-zero y_reference vector.".to_string(),
            "Refresh the viewport connect-pipe intent from explicit user-entered pipe fields.",
            vec![target_ref.to_string()],
        );
        return None;
    }
    let outside_diameter = outside_diameter.unwrap();
    let wall_thickness = wall_thickness.unwrap();
    let Some(outside_diameter_for_check) =
        quantity_value_in_unit(&outside_diameter, stored_unit, Dimension::Length)
    else {
        checker.unit_state = "blocked";
        checker.push(
            "OP-UNIT-MISMATCH-CONVERSION-UNAVAILABLE",
            "blocking",
            "Pipe outside-diameter unit could not be converted through the accepted DEC-018 length catalog.".to_string(),
            "Select an accepted length unit from the DEC-018 catalog.",
            vec![target_ref.to_string()],
        );
        return None;
    };
    let Some(wall_thickness_for_check) =
        quantity_value_in_unit(&wall_thickness, stored_unit, Dimension::Length)
    else {
        checker.unit_state = "blocked";
        checker.push(
            "OP-UNIT-MISMATCH-CONVERSION-UNAVAILABLE",
            "blocking",
            "Pipe wall-thickness unit could not be converted through the accepted DEC-018 length catalog.".to_string(),
            "Select an accepted length unit from the DEC-018 catalog.",
            vec![target_ref.to_string()],
        );
        return None;
    };
    if wall_thickness_for_check >= outside_diameter_for_check / 2.0 {
        checker.push(
            "OP-CONNECT-PIPE-PAYLOAD-INVALID",
            "blocking",
            "Connect-pipe wall thickness must be less than the outside-diameter radius."
                .to_string(),
            "Enter physically possible pipe section dimensions before connecting a pipe segment.",
            vec![target_ref.to_string()],
        );
        return None;
    }
    let incomplete_insulation = match rich_authoring::validate_pipe_section_inputs(&record["section"]) {
        Ok(incomplete) => incomplete,
        Err(error) => {
            checker.push(
                "OP-CONNECT-PIPE-PAYLOAD-INVALID",
                "blocking",
                error.message,
                "Supply supported explicit section quantities with accepted units and valid values.",
                vec![target_ref.to_string()],
            );
            return None;
        }
    };
    if find_entity(model, "nodes", from).is_none() || find_entity(model, "nodes", to).is_none() {
        checker.reference_state = "blocked";
        checker.push(
            "OP-PIPE-ENDPOINT-NOT-FOUND",
            "blocking",
            format!("Pipe `{target_ref}` references endpoint nodes `{from}` and `{to}`, but at least one is absent from the current model."),
            "Create or select existing endpoint nodes before connecting a pipe segment.",
            vec![target_ref.to_string(), from.to_string(), to.to_string()],
        );
        return None;
    }
    if find_entity(model, "materials", material).is_none() {
        checker.reference_state = "blocked";
        checker.push(
            "OP-PIPE-MATERIAL-NOT-FOUND",
            "blocking",
            format!("Pipe `{target_ref}` references material `{material}`, which is absent from the current model."),
            "Select an existing material before connecting a pipe segment.",
            vec![target_ref.to_string(), material.to_string()],
        );
        return None;
    }
    checker.reference_state = "passed";

    if incomplete_insulation {
        checker.push(
            "OP-MASS-NOT-SOLVE-READY",
            "warning",
            "Insulation mass requires both explicit thickness and density; no missing quantity is defaulted.".into(),
            "Supply the paired insulation quantity before solving.",
            vec![target_ref.to_string()],
        );
    }

    let (yrx, yry, yrz) = y_reference.unwrap();
    Some(serde_json::json!({
        "id": id,
        "label": label,
        "from": from,
        "to": to,
        "section": record["section"].clone(),
        "material": material,
        "y_reference": { "x": yrx, "y": yry, "z": yrz },
        "provenance": provenance,
    }))
}

