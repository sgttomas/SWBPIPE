/// Check explicit inline source inputs without normalizing their stored JSON.
/// The return value marks an incomplete insulation pair, which remains
/// authorable but cannot supply insulation mass until both inputs are present.
pub(crate) fn validate_pipe_section_inputs(section: &Value) -> Result<bool> {
    let record = object(
        section,
        &[
            "outside_diameter",
            "wall_thickness",
            "material_density",
            "mill_tolerance",
            "contents_density",
            "insulation_thickness",
            "insulation_density",
        ],
    )?;
    quantity(&section["outside_diameter"], Dimension::Length, true)?;
    let wall = quantity(&section["wall_thickness"], Dimension::Length, true)?;
    for (key, dimension, positive) in [
        ("material_density", Dimension::Density, true),
        ("mill_tolerance", Dimension::Length, false),
        ("contents_density", Dimension::Density, false),
        ("insulation_thickness", Dimension::Length, false),
        ("insulation_density", Dimension::Density, false),
    ] {
        if let Some(value) = record.get(key).filter(|v| !v.is_null()) {
            let normalized = if positive {
                quantity(value, dimension, true)
            } else {
                nonnegative(value, dimension)
            }
            .map_err(|error| err(format!("section.{key}: {}", error.message)))?;
            if key == "mill_tolerance" && wall - normalized <= 0.0 {
                return Err(err("Mill tolerance must leave a positive effective wall"));
            }
        }
    }
    Ok(record
        .get("insulation_thickness")
        .is_some_and(|v| !v.is_null())
        != record
            .get("insulation_density")
            .is_some_and(|v| !v.is_null()))
}

