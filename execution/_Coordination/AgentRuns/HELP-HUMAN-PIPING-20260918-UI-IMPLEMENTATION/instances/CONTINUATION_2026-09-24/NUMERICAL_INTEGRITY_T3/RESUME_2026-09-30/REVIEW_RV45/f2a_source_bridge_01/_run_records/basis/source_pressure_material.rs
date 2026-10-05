//! Explicit common E/nu selection for the straight-annulus profile.
use super::*;
use pressure_exact::IsotropicENu;

fn failure(diagnostics: &mut Vec<Diagnostic>, material: &str, code: &str, message: &str) {
    diagnostics.push(diag(
        &format!(
            "diagnostic:exact-material:{}:{}",
            stable_suffix(material),
            code
        ),
        code,
        "blocking",
        message,
        vec![material.to_string()],
    ));
}

fn checked_pair(
    material: &MaterialInput,
    e: &Quantity,
    nu: Option<&Quantity>,
    diagnostics: &mut Vec<Diagnostic>,
) -> Option<IsotropicENu> {
    if material.constitutive_basis.as_deref() != Some("homogeneous_isotropic_E_nu_v1") {
        failure(
            diagnostics,
            &material.id,
            "EXACT_PRESSURE_MATERIAL_BASIS_REQUIRED",
            "exact pressure requires explicit homogeneous_isotropic_E_nu_v1 constitutive basis",
        );
        return None;
    }
    let Some(nu) = nu else {
        failure(
            diagnostics,
            &material.id,
            "EXACT_PRESSURE_POISSON_RATIO_REQUIRED",
            "the selected material basis requires explicit Poisson ratio; no value is inferred",
        );
        return None;
    };
    if nu.unit != "1" {
        failure(
            diagnostics,
            &material.id,
            "EXACT_PRESSURE_MATERIAL_INVALID",
            "Poisson ratio must carry dimensionless unit 1",
        );
        return None;
    }
    match IsotropicENu::new(e.value, nu.value) {
        Ok(pair) => Some(pair),
        Err(_) => {
            failure(diagnostics, &material.id, "EXACT_PRESSURE_MATERIAL_INVALID", "selected E and nu require finite E > 0, -1 < nu < 0.5 and representable positive derived G");
            None
        }
    }
}

pub(super) fn resolve_base(
    model: &PreviewModel,
    materials: &mut [MaterialInput],
    diagnostics: &mut Vec<Diagnostic>,
) {
    if !pressure_runtime::is_exact(model) {
        return;
    }
    let used = model
        .pipe_segments
        .iter()
        .map(|p| p.material.as_str())
        .collect::<HashSet<_>>();
    for material in materials
        .iter_mut()
        .filter(|m| used.contains(m.id.as_str()))
    {
        if let Some(pair) = checked_pair(
            material,
            &material.elastic_modulus,
            material.poisson_ratio.as_ref(),
            diagnostics,
        ) {
            if material.shear_modulus.is_some() {
                diagnostics.push(diag(&format!("diagnostic:exact-material:{}:g-derived", stable_suffix(&material.id)),
                    "EXACT_PRESSURE_REDUNDANT_G_IGNORED", "warning", "retained shear modulus is nonauthoritative; stiffness uses G derived from the selected common E/nu pair", vec![material.id.clone()]));
            }
            material.shear_modulus = Some(Quantity {
                value: pair.shear_modulus_pa(),
                unit: "Pa".to_string(),
            });
        }
    }
}

fn thermal_required(model: &PreviewModel, case: &PreviewLoadCase, material: &str) -> bool {
    case.primitive_loads.iter().any(|load| load.category == "thermal" && is_temperature_change_dimension(&load.dimension)
        && matches!(&load.target, LoadTargetInput::Element { pipe } if model.pipe_segments.iter().any(|p| &p.id == pipe && p.material == material)))
}

pub(super) fn resolve_case(
    model: &PreviewModel,
    materials: &[MaterialInput],
    case: &PreviewLoadCase,
    diagnostics: &mut Vec<Diagnostic>,
) -> Option<(Vec<MaterialInput>, String)> {
    let used = model
        .pipe_segments
        .iter()
        .map(|p| p.material.as_str())
        .collect::<HashSet<_>>();
    let mut resolved = Vec::with_capacity(materials.len());
    let mut records = Vec::new();
    for material in materials {
        if !used.contains(material.id.as_str()) {
            resolved.push(material.clone());
            continue;
        }
        let alpha_required = thermal_required(model, case, &material.id);
        let selected = if let Some(id) = &case.modulus_basis_ref {
            let Some(point) = material.temperature_points.iter().find(|p| &p.id == id) else {
                failure(
                    diagnostics,
                    &material.id,
                    "MODULUS_BASIS_UNRESOLVED",
                    "selected exact material point does not exist",
                );
                continue;
            };
            let Some(e) = &point.elastic_modulus else {
                failure(
                    diagnostics,
                    &material.id,
                    "MODULUS_BASIS_INPUT_MISSING",
                    "selected exact point requires E and nu on the same point",
                );
                continue;
            };
            let Some(pair) = checked_pair(material, e, point.poisson_ratio.as_ref(), diagnostics)
            else {
                continue;
            };
            (
                pair,
                point.thermal_expansion_coefficient.clone(),
                format!("point:{id}"),
            )
        } else {
            let t = case.modulus_basis_temperature.as_ref()?.value;
            let mut points = material
                .temperature_points
                .iter()
                .filter_map(|p| p.temperature.as_ref().map(|t| (t.value, p)))
                .collect::<Vec<_>>();
            points.sort_by(|a, b| a.0.total_cmp(&b.0));
            if points.windows(2).any(|p| p[0].0 == p[1].0) {
                failure(
                    diagnostics,
                    &material.id,
                    "MODULUS_BASIS_INPUT_INVALID",
                    "duplicate temperatures make the common E/nu interpolation bracket ambiguous",
                );
                continue;
            }
            let Some(bracket) = points.windows(2).find(|p| p[0].0 < t && t < p[1].0) else {
                failure(diagnostics, &material.id, "MODULUS_BASIS_UNRESOLVED", "solve temperature requires strict interior bracketing; no endpoint fallback or extrapolation");
                continue;
            };
            let (t0, p0) = bracket[0];
            let (t1, p1) = bracket[1];
            let (Some(e0), Some(e1)) = (&p0.elastic_modulus, &p1.elastic_modulus) else {
                failure(
                    diagnostics,
                    &material.id,
                    "MODULUS_BASIS_INPUT_MISSING",
                    "both common bracket points require explicit E and nu",
                );
                continue;
            };
            let (Some(m0), Some(m1)) = (
                checked_pair(material, e0, p0.poisson_ratio.as_ref(), diagnostics),
                checked_pair(material, e1, p1.poisson_ratio.as_ref(), diagnostics),
            ) else {
                continue;
            };
            let fraction = (t - t0) / (t1 - t0);
            let interpolate = |a: f64, b: f64| (1.0 - fraction) * a + fraction * b;
            let e = Quantity {
                value: interpolate(m0.elastic_modulus_pa(), m1.elastic_modulus_pa()),
                unit: "Pa".to_string(),
            };
            let nu = Quantity {
                value: interpolate(m0.poisson_ratio(), m1.poisson_ratio()),
                unit: "1".to_string(),
            };
            let Some(pair) = checked_pair(material, &e, Some(&nu), diagnostics) else {
                continue;
            };
            let alpha = match (
                &p0.thermal_expansion_coefficient,
                &p1.thermal_expansion_coefficient,
            ) {
                (Some(a), Some(b)) => Some(Quantity {
                    value: interpolate(a.value, b.value),
                    unit: a.unit.clone(),
                }),
                _ => None,
            };
            (
                pair,
                alpha,
                format!("interpolated:{}..{};temperature_kelvin={t}", p0.id, p1.id),
            )
        };
        let (pair, alpha, basis) = selected;
        if alpha_required && alpha.is_none() {
            failure(
                diagnostics,
                &material.id,
                "THERMAL_EXPANSION_INPUT_MISSING",
                "thermal load requires alpha from the same selected point or bracket",
            );
            continue;
        }
        if alpha.as_ref().is_some_and(|a| !a.value.is_finite()) {
            failure(
                diagnostics,
                &material.id,
                "MODULUS_BASIS_INPUT_INVALID",
                "selected thermal coefficient must be finite",
            );
            continue;
        }
        let mut selected_material = material.clone();
        selected_material.elastic_modulus = Quantity {
            value: pair.elastic_modulus_pa(),
            unit: "Pa".to_string(),
        };
        selected_material.poisson_ratio = Some(Quantity {
            value: pair.poisson_ratio(),
            unit: "1".to_string(),
        });
        selected_material.shear_modulus = Some(Quantity {
            value: pair.shear_modulus_pa(),
            unit: "Pa".to_string(),
        });
        selected_material.thermal_expansion_coefficient = alpha;
        records.push(format!(
            "material={};common_E_nu_basis={basis};G=E/[2(1+nu)];alpha_same_basis={alpha_required}",
            material.id
        ));
        resolved.push(selected_material);
    }
    if has_blocking(diagnostics) {
        None
    } else {
        Some((resolved, records.join(" | ")))
    }
}
