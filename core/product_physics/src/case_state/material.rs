//! Pure per-member constitutive selection for the load/reference-state profile.
//!
//! Inputs retain authored units; only consumed values are normalized through the
//! existing units crate. This module neither clones material identities nor
//! mutates a material map. Recorded G and alpha are never consumed as E/nu.
use crate::{pressure_exact::IsotropicENu, MaterialInput, MaterialTemperaturePointInput, Quantity};
use open_pipe_stress_units::{canonical_unit, convert_for_dimension, unit_by_symbol, Dimension};
use std::collections::HashSet;

/// Closed private choices. The public DTO adapter owns unknown-discriminant
/// rejection and maps only its supported interpolation/extrapolation policy here.
#[derive(Debug, Clone)]
pub(crate) enum MaterialSelection {
    ExplicitBaseProperties {
        material_ref: String,
        applicability_reference: String,
    },
    ExactPoint {
        material_ref: String,
        point_ref: String,
    },
    TemperatureInterpolation {
        material_ref: String,
        temperature: Quantity,
    },
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct AnalysisBasisOverride {
    pub reason: String,
    pub provenance: String,
}
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ConsumedMaterialPoint {
    pub point_id: String,
    pub temperature_k: Option<f64>,
    pub pair: IsotropicENu,
    pub provenance: Option<String>,
    pub retained_g_ignored: bool,
}
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ResolvedMemberMaterial {
    pub material_id: String,
    pub material_provenance: Option<String>,
    pub selection_kind: &'static str,
    pub pair: IsotropicENu,
    /// Physical input, never inferred from a property point or requested basis.
    pub operating_temperature_k: Option<f64>,
    /// Property-data basis; None for fixed base data or an undated exact point.
    pub selection_temperature_k: Option<f64>,
    /// Empty for explicit base, one for named/exact endpoint, two for interior.
    pub consumed_points: Vec<ConsumedMaterialPoint>,
    pub interpolation_fraction: Option<f64>,
    pub applicability_reference: Option<String>,
    pub analysis_basis_override: Option<AnalysisBasisOverride>,
    /// Records ignored G on the selected base/point/bracket, not unrelated points.
    pub retained_g_ignored: bool,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct MaterialSelectionError {
    pub code: &'static str,
    pub material_id: String,
    pub point_refs: Vec<String>,
    pub message: String,
}
type Result<T> = std::result::Result<T, MaterialSelectionError>;
fn error(
    material: &MaterialInput,
    code: &'static str,
    message: impl Into<String>,
) -> MaterialSelectionError {
    MaterialSelectionError {
        code,
        material_id: material.id.clone(),
        point_refs: Vec::new(),
        message: message.into(),
    }
}
fn normalized(material: &MaterialInput, quantity: &Quantity, dimension: Dimension) -> Result<f64> {
    let fail = |message: String| error(material, "LOAD_STATE_MATERIAL_QUANTITY_INVALID", message);
    let from = unit_by_symbol(&quantity.unit, dimension).map_err(|e| fail(e.to_string()))?;
    let to = canonical_unit(dimension)
        .ok_or_else(|| fail("No canonical unit for consumed dimension".into()))?;
    let value = convert_for_dimension(quantity.value, dimension, from, to)
        .map_err(|e| fail(e.to_string()))?;
    if !value.is_finite() || (dimension == Dimension::Temperature && value < 0.0) {
        return Err(fail(
            "Consumed quantity must be finite; absolute temperature cannot be below zero kelvin"
                .into(),
        ));
    }
    Ok(value)
}
fn pair(
    material: &MaterialInput,
    e: Option<&Quantity>,
    nu: Option<&Quantity>,
) -> Result<IsotropicENu> {
    let (Some(e), Some(nu)) = (e, nu) else {
        return Err(error(
            material,
            "LOAD_STATE_MATERIAL_PAIR_MISSING",
            "Every consumed material basis requires explicit E and nu",
        ));
    };
    if nu.unit != "1" {
        return Err(error(
            material,
            "LOAD_STATE_MATERIAL_QUANTITY_INVALID",
            "Poisson ratio requires dimensionless unit 1",
        ));
    }
    let e = normalized(material, e, Dimension::Stress)?;
    IsotropicENu::new(e, nu.value).map_err(|e| {
        error(
            material,
            "LOAD_STATE_MATERIAL_PAIR_INVALID",
            format!("Consumed E/nu pair is outside the producer material domain: {e:?}"),
        )
    })
}
fn point(
    material: &MaterialInput,
    point: &MaterialTemperaturePointInput,
) -> Result<ConsumedMaterialPoint> {
    let consume = || {
        Ok(ConsumedMaterialPoint {
            point_id: point.id.clone(),
            temperature_k: point
                .temperature
                .as_ref()
                .map(|t| normalized(material, t, Dimension::Temperature))
                .transpose()?,
            pair: pair(
                material,
                point.elastic_modulus.as_ref(),
                point.poisson_ratio.as_ref(),
            )?,
            provenance: point.provenance.clone(),
            retained_g_ignored: point.shear_modulus.is_some(),
        })
    };
    consume().map_err(|mut e: MaterialSelectionError| {
        e.point_refs.push(point.id.clone());
        e
    })
}
fn check_temperature_basis(
    material: &MaterialInput,
    actual: Option<f64>,
    selected: Option<f64>,
    basis_override: Option<&AnalysisBasisOverride>,
) -> Result<()> {
    // An undated point is a deliberate fixed basis, not proof of applicability
    // at actual T. Preserve None; only two known differing temperatures conflict.
    if matches!((actual, selected), (Some(a), Some(s)) if a != s) && basis_override.is_none() {
        return Err(error(material, "LOAD_STATE_MATERIAL_TEMPERATURE_OVERRIDE_REQUIRED", "Selected property temperature differs from known operating temperature; enter an explicit analysis-basis reason and provenance"));
    }
    Ok(())
}

pub(crate) fn select_for_member(
    material: &MaterialInput,
    selection: &MaterialSelection,
    operating_temperature: Option<&Quantity>,
    analysis_basis_override: Option<&AnalysisBasisOverride>,
) -> Result<ResolvedMemberMaterial> {
    let material_ref = match selection {
        MaterialSelection::ExplicitBaseProperties { material_ref, .. }
        | MaterialSelection::ExactPoint { material_ref, .. }
        | MaterialSelection::TemperatureInterpolation { material_ref, .. } => material_ref,
    };
    if material.id.trim().is_empty() || material_ref != &material.id {
        return Err(error(
            material,
            "LOAD_STATE_MATERIAL_REFERENCE_MISMATCH",
            "Selection must name this member's actual material record",
        ));
    }
    if material.constitutive_basis.as_deref() != Some("homogeneous_isotropic_E_nu_v1") {
        return Err(error(
            material,
            "LOAD_STATE_MATERIAL_BASIS_REQUIRED",
            "An explicit homogeneous_isotropic_E_nu_v1 basis is required",
        ));
    }
    if analysis_basis_override
        .is_some_and(|v| v.reason.trim().is_empty() || v.provenance.trim().is_empty())
    {
        return Err(error(
            material,
            "LOAD_STATE_MATERIAL_OVERRIDE_INVALID",
            "Analysis-basis override requires nonempty reason and provenance",
        ));
    }
    let actual = operating_temperature
        .map(|t| normalized(material, t, Dimension::Temperature))
        .transpose()?;
    let mut resolved = match selection {
        MaterialSelection::ExplicitBaseProperties {
            applicability_reference,
            ..
        } => {
            if applicability_reference.trim().is_empty() {
                return Err(error(
                    material,
                    "LOAD_STATE_MATERIAL_APPLICABILITY_REQUIRED",
                    "Fixed base properties require an explicit applicability reference",
                ));
            }
            ResolvedMemberMaterial {
                material_id: material.id.clone(),
                material_provenance: material.provenance.clone(),
                selection_kind: "explicit_base_properties",
                pair: pair(
                    material,
                    Some(&material.elastic_modulus),
                    material.poisson_ratio.as_ref(),
                )?,
                operating_temperature_k: actual,
                selection_temperature_k: None,
                consumed_points: vec![],
                interpolation_fraction: None,
                applicability_reference: Some(applicability_reference.clone()),
                analysis_basis_override: None,
                retained_g_ignored: material.shear_modulus.is_some(),
            }
        }
        MaterialSelection::ExactPoint { point_ref, .. } => {
            if point_ref.trim().is_empty() {
                return Err(error(
                    material,
                    "LOAD_STATE_MATERIAL_POINT_UNRESOLVED",
                    "Exact point requires a nonempty ID on the actual material",
                ));
            }
            let mut matches = material
                .temperature_points
                .iter()
                .filter(|p| &p.id == point_ref);
            let selected = matches.next().ok_or_else(|| {
                error(
                    material,
                    "LOAD_STATE_MATERIAL_POINT_UNRESOLVED",
                    format!("Point {point_ref} does not exist on the actual material"),
                )
            })?;
            if matches.next().is_some() {
                return Err(error(
                    material,
                    "LOAD_STATE_MATERIAL_POINT_AMBIGUOUS",
                    "Exact point ID is duplicated on this material",
                ));
            }
            let selected = point(material, selected)?;
            check_temperature_basis(
                material,
                actual,
                selected.temperature_k,
                analysis_basis_override,
            )?;
            ResolvedMemberMaterial {
                material_id: material.id.clone(),
                material_provenance: material.provenance.clone(),
                selection_kind: "exact_point",
                pair: selected.pair,
                operating_temperature_k: actual,
                selection_temperature_k: selected.temperature_k,
                retained_g_ignored: selected.retained_g_ignored,
                consumed_points: vec![selected],
                interpolation_fraction: None,
                applicability_reference: None,
                analysis_basis_override: None,
            }
        }
        MaterialSelection::TemperatureInterpolation { temperature, .. } => {
            let requested = normalized(material, temperature, Dimension::Temperature)?;
            check_temperature_basis(material, actual, Some(requested), analysis_basis_override)?;
            let mut ids = HashSet::new();
            let mut table = Vec::new();
            for p in &material.temperature_points {
                if p.id.trim().is_empty() || !ids.insert(p.id.as_str()) {
                    return Err(error(
                        material,
                        "LOAD_STATE_MATERIAL_POINT_AMBIGUOUS",
                        "Material interpolation requires unique nonempty point IDs",
                    ));
                }
                if let Some(t) = &p.temperature {
                    table.push((normalized(material, t, Dimension::Temperature)?, p));
                }
            }
            table.sort_by(|a, b| a.0.total_cmp(&b.0));
            if table.windows(2).any(|w| w[0].0 == w[1].0) {
                return Err(error(
                    material,
                    "LOAD_STATE_MATERIAL_TEMPERATURE_AMBIGUOUS",
                    "Duplicate normalized material temperatures prevent a unique table selection",
                ));
            }
            let (points, fraction, selected_pair) = if let Some((_, p)) =
                table.iter().find(|(t, _)| *t == requested)
            {
                let p = point(material, p)?;
                let selected_pair = p.pair;
                (vec![p], None, selected_pair)
            } else {
                let bracket = table.windows(2).find(|w| w[0].0 < requested && requested < w[1].0).ok_or_else(|| error(material, "LOAD_STATE_MATERIAL_TEMPERATURE_UNRESOLVED", "No strict adjacent bracket or exact endpoint; extrapolation and fixed-data fallback are forbidden"))?;
                // Validate both actually consumed pairs before interpolation; an
                // inadmissible endpoint must not be hidden by a benign midpoint.
                let low = point(material, bracket[0].1)?;
                let high = point(material, bracket[1].1)?;
                let fraction = (requested - bracket[0].0) / (bracket[1].0 - bracket[0].0);
                if !(fraction > 0.0 && fraction < 1.0 && fraction.is_finite()) {
                    return Err(error(
                        material,
                        "LOAD_STATE_MATERIAL_INTERPOLATION_RANGE",
                        "Interior interpolation weight is not representable",
                    ));
                }
                let linear = |a: f64, b: f64| fraction.mul_add(b - a, a);
                let selected_pair = IsotropicENu::new(
                    linear(
                        low.pair.elastic_modulus_pa(),
                        high.pair.elastic_modulus_pa(),
                    ),
                    linear(low.pair.poisson_ratio(), high.pair.poisson_ratio()),
                )
                .map_err(|e| {
                    error(
                        material,
                        "LOAD_STATE_MATERIAL_PAIR_INVALID",
                        format!(
                            "Interpolated E/nu pair is outside the producer material domain: {e:?}"
                        ),
                    )
                })?;
                (vec![low, high], Some(fraction), selected_pair)
            };
            ResolvedMemberMaterial {
                material_id: material.id.clone(),
                material_provenance: material.provenance.clone(),
                selection_kind: "temperature_interpolation",
                pair: selected_pair,
                operating_temperature_k: actual,
                selection_temperature_k: Some(requested),
                retained_g_ignored: points.iter().any(|p| p.retained_g_ignored),
                consumed_points: points,
                interpolation_fraction: fraction,
                applicability_reference: None,
                analysis_basis_override: None,
            }
        }
    };
    resolved.analysis_basis_override = analysis_basis_override.cloned();
    Ok(resolved)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn q(value: f64, unit: &str) -> Quantity {
        Quantity {
            value,
            unit: unit.into(),
        }
    }
    fn sample() -> MaterialInput {
        serde_json::from_value(json!({
            "id":"material:user", "constitutive_basis":"homogeneous_isotropic_E_nu_v1",
            "elastic_modulus":{"value":200,"unit":"GPa"}, "poisson_ratio":{"value":0.3,"unit":"1"},
            "shear_modulus":{"value":1,"unit":"Pa"}, "provenance":"invented explicit test data",
            "temperature_points":[
                {"id":"point:cold","temperature":{"value":20,"unit":"degC"},"elastic_modulus":{"value":200,"unit":"GPa"},"poisson_ratio":{"value":0.3,"unit":"1"},"provenance":"invented cold input"},
                {"id":"point:hot","temperature":{"value":120,"unit":"degC"},"elastic_modulus":{"value":100,"unit":"GPa"},"poisson_ratio":{"value":0.2,"unit":"1"},"provenance":"invented hot input"}
            ]
        })).unwrap()
    }
    fn base() -> MaterialSelection {
        MaterialSelection::ExplicitBaseProperties {
            material_ref: "material:user".into(),
            applicability_reference: "user:fixed-data-applicability".into(),
        }
    }
    fn exact(id: &str) -> MaterialSelection {
        MaterialSelection::ExactPoint {
            material_ref: "material:user".into(),
            point_ref: id.into(),
        }
    }
    fn temperature(value: f64, unit: &str) -> MaterialSelection {
        MaterialSelection::TemperatureInterpolation {
            material_ref: "material:user".into(),
            temperature: q(value, unit),
        }
    }
    fn basis_override() -> AnalysisBasisOverride {
        AnalysisBasisOverride {
            reason: "Explicit alternate analysis basis".into(),
            provenance: "user:analysis-instruction".into(),
        }
    }
    fn close(actual: f64, expected: f64) {
        assert!(actual.is_finite());
        assert!(
            (actual - expected).abs() <= expected.abs() * 1e-9,
            "{actual:.17e} versus {expected:.17e}"
        );
    }
    #[test]
    fn same_actual_material_resolves_independently_for_two_members_without_mutation() {
        let material = sample();
        let original = format!("{material:?}");
        let cold = select_for_member(
            &material,
            &temperature(20.0, "degC"),
            Some(&q(20.0, "degC")),
            None,
        )
        .unwrap();
        let hot = select_for_member(
            &material,
            &temperature(120.0, "degC"),
            Some(&q(120.0, "degC")),
            None,
        )
        .unwrap();
        assert_eq!(cold.material_id, "material:user");
        assert_eq!(hot.material_id, cold.material_id);
        close(cold.pair.elastic_modulus_pa(), 200e9);
        close(hot.pair.elastic_modulus_pa(), 100e9);
        close(cold.pair.shear_modulus_pa(), 200e9 / 2.6);
        close(hot.pair.shear_modulus_pa(), 100e9 / 2.4);
        assert_eq!(cold.consumed_points[0].point_id, "point:cold");
        assert_eq!(hot.consumed_points[0].point_id, "point:hot");
        assert_eq!(format!("{material:?}"), original);
    }
    #[test]
    fn exact_endpoints_and_strict_adjacent_interpolation_use_common_e_nu_not_interpolated_g() {
        let mut material = sample();
        let mut middle = material.temperature_points[0].clone();
        middle.id = "point:middle".into();
        middle.temperature = Some(q(70.0, "degC"));
        middle.elastic_modulus = Some(q(180.0, "GPa"));
        middle.poisson_ratio = Some(q(-0.2, "1"));
        material.temperature_points.push(middle);
        // Source order does not choose the bracket.
        material.temperature_points.reverse();
        for (t, id, e) in [
            (20.0, "point:cold", 200e9),
            (70.0, "point:middle", 180e9),
            (120.0, "point:hot", 100e9),
        ] {
            let selected =
                select_for_member(&material, &temperature(t, "degC"), None, None).unwrap();
            assert_eq!(selected.consumed_points.len(), 1);
            assert_eq!(selected.consumed_points[0].point_id, id);
            assert_eq!(selected.interpolation_fraction, None);
            close(selected.pair.elastic_modulus_pa(), e);
        }
        let selected =
            select_for_member(&material, &temperature(95.0, "degC"), None, None).unwrap();
        assert_eq!(
            selected
                .consumed_points
                .iter()
                .map(|p| p.point_id.as_str())
                .collect::<Vec<_>>(),
            vec!["point:middle", "point:hot"]
        );
        assert_eq!(selected.interpolation_fraction, Some(0.5));
        close(selected.pair.elastic_modulus_pa(), 140e9);
        assert_eq!(selected.pair.poisson_ratio(), 0.0);
        close(selected.pair.shear_modulus_pa(), 70e9);
        let interpolated_g = (180e9 / 1.6 + 100e9 / 2.4) / 2.0;
        assert!((selected.pair.shear_modulus_pa() - interpolated_g).abs() > 1e9);
    }
    #[test]
    fn equivalent_authored_units_produce_same_selected_values_and_temperatures() {
        let original = sample();
        let mut converted = original.clone();
        converted.temperature_points[0].temperature = Some(q(293.15, "K"));
        converted.temperature_points[1].temperature = Some(q(393.15, "K"));
        converted.temperature_points[0].elastic_modulus = Some(q(200000.0, "MPa"));
        converted.temperature_points[1].elastic_modulus = Some(q(100000000000.0, "Pa"));
        let a = select_for_member(
            &original,
            &temperature(70.0, "degC"),
            Some(&q(70.0, "degC")),
            None,
        )
        .unwrap();
        let b = select_for_member(
            &converted,
            &temperature(343.15, "K"),
            Some(&q(343.15, "K")),
            None,
        )
        .unwrap();
        assert_eq!(a, b);
    }
    #[test]
    fn duplicate_normalized_temperatures_missing_pairs_and_extrapolation_are_rejected() {
        let mut material = sample();
        let mut duplicate = material.temperature_points[0].clone();
        duplicate.id = "point:duplicate".into();
        duplicate.temperature = Some(q(293.15, "K"));
        material.temperature_points.push(duplicate);
        assert_eq!(
            select_for_member(&material, &temperature(20.0, "degC"), None, None)
                .unwrap_err()
                .code,
            "LOAD_STATE_MATERIAL_TEMPERATURE_AMBIGUOUS"
        );
        let mut material = sample();
        material.temperature_points[0].poisson_ratio = None;
        for selected in [
            temperature(20.0, "degC"),
            temperature(70.0, "degC"),
            exact("point:cold"),
        ] {
            let error = select_for_member(&material, &selected, None, None).unwrap_err();
            assert_eq!(error.code, "LOAD_STATE_MATERIAL_PAIR_MISSING");
            assert_eq!(error.point_refs, vec!["point:cold"]);
        }
        for t in [19.0, 121.0] {
            assert_eq!(
                select_for_member(&sample(), &temperature(t, "degC"), None, None)
                    .unwrap_err()
                    .code,
                "LOAD_STATE_MATERIAL_TEMPERATURE_UNRESOLVED"
            );
        }
    }
    #[test]
    fn named_points_bind_to_actual_material_and_cannot_resolve_unknown_or_duplicate_ids() {
        let material = sample();
        let wrong = MaterialSelection::ExactPoint {
            material_ref: "material:other".into(),
            point_ref: "point:cold".into(),
        };
        assert_eq!(
            select_for_member(&material, &wrong, None, None)
                .unwrap_err()
                .code,
            "LOAD_STATE_MATERIAL_REFERENCE_MISMATCH"
        );
        assert_eq!(
            select_for_member(&material, &exact("point:other-material-only"), None, None)
                .unwrap_err()
                .code,
            "LOAD_STATE_MATERIAL_POINT_UNRESOLVED"
        );
        let mut duplicate = material.clone();
        duplicate
            .temperature_points
            .push(duplicate.temperature_points[0].clone());
        assert_eq!(
            select_for_member(&duplicate, &exact("point:cold"), None, None)
                .unwrap_err()
                .code,
            "LOAD_STATE_MATERIAL_POINT_AMBIGUOUS"
        );
        assert_eq!(
            select_for_member(&duplicate, &temperature(70.0, "degC"), None, None)
                .unwrap_err()
                .code,
            "LOAD_STATE_MATERIAL_POINT_AMBIGUOUS"
        );
    }
    #[test]
    fn actual_and_selected_temperature_remain_distinct_and_known_mismatch_needs_override() {
        let material = sample();
        let actual = q(120.0, "degC");
        let basis = basis_override();
        for selected in [exact("point:cold"), temperature(20.0, "degC")] {
            assert_eq!(
                select_for_member(&material, &selected, Some(&actual), None)
                    .unwrap_err()
                    .code,
                "LOAD_STATE_MATERIAL_TEMPERATURE_OVERRIDE_REQUIRED"
            );
            let resolved =
                select_for_member(&material, &selected, Some(&actual), Some(&basis)).unwrap();
            assert_eq!(resolved.operating_temperature_k, Some(393.15));
            assert_eq!(resolved.selection_temperature_k, Some(293.15));
            assert_eq!(resolved.analysis_basis_override, Some(basis.clone()));
        }
        let absent = select_for_member(&material, &exact("point:cold"), None, None).unwrap();
        assert_eq!(absent.operating_temperature_k, None);
        assert_eq!(absent.selection_temperature_k, Some(293.15));
        let mut undated = material.clone();
        undated.temperature_points[0].temperature = None;
        let selected =
            select_for_member(&undated, &exact("point:cold"), Some(&actual), None).unwrap();
        assert_eq!(selected.operating_temperature_k, Some(393.15));
        assert_eq!(selected.selection_temperature_k, None);
        assert_eq!(selected.consumed_points[0].temperature_k, None);
    }
    #[test]
    fn fixed_base_requires_applicability_and_ignores_retained_g_without_inventing_temperature() {
        let mut material = sample();
        material.shear_modulus = Some(q(f64::NAN, "unconsumed invalid G"));
        let selected = select_for_member(&material, &base(), None, None).unwrap();
        assert!(selected.retained_g_ignored);
        close(selected.pair.shear_modulus_pa(), 200e9 / 2.6);
        assert_eq!(selected.operating_temperature_k, None);
        assert_eq!(selected.selection_temperature_k, None);
        assert_eq!(
            selected.applicability_reference.as_deref(),
            Some("user:fixed-data-applicability")
        );
        let missing = MaterialSelection::ExplicitBaseProperties {
            material_ref: material.id.clone(),
            applicability_reference: " ".into(),
        };
        assert_eq!(
            select_for_member(&material, &missing, None, None)
                .unwrap_err()
                .code,
            "LOAD_STATE_MATERIAL_APPLICABILITY_REQUIRED"
        );
        material.temperature_points[0].shear_modulus = Some(q(-999.0, "Pa"));
        assert!(
            select_for_member(&material, &exact("point:cold"), None, None)
                .unwrap()
                .retained_g_ignored
        );
    }
    #[test]
    fn invalid_units_basis_absolute_temperature_and_override_fail_explicitly() {
        let material = sample();
        assert_eq!(
            select_for_member(&material, &temperature(20.0, "m"), None, None)
                .unwrap_err()
                .code,
            "LOAD_STATE_MATERIAL_QUANTITY_INVALID"
        );
        assert!(select_for_member(&material, &base(), Some(&q(-1.0, "K")), None).is_err());
        assert!(select_for_member(&material, &base(), Some(&q(f64::INFINITY, "K")), None).is_err());
        let mut wrong_basis = material.clone();
        wrong_basis.constitutive_basis = None;
        assert_eq!(
            select_for_member(&wrong_basis, &base(), None, None)
                .unwrap_err()
                .code,
            "LOAD_STATE_MATERIAL_BASIS_REQUIRED"
        );
        let mut wrong_nu = material.clone();
        wrong_nu.poisson_ratio = Some(q(0.3, "none"));
        assert_eq!(
            select_for_member(&wrong_nu, &base(), None, None)
                .unwrap_err()
                .code,
            "LOAD_STATE_MATERIAL_QUANTITY_INVALID"
        );
        let invalid = AnalysisBasisOverride {
            reason: " ".into(),
            provenance: "user".into(),
        };
        assert_eq!(
            select_for_member(&material, &base(), None, Some(&invalid))
                .unwrap_err()
                .code,
            "LOAD_STATE_MATERIAL_OVERRIDE_INVALID"
        );
    }
    #[test]
    fn every_consumed_pair_obeys_adjacent_producer_representability_boundaries() {
        let mut material = sample();
        material.poisson_ratio = Some(q(-0.75, "1"));
        material.elastic_modulus = q(f64::MAX / 2.0, "Pa");
        let valid = select_for_member(&material, &base(), None, None).unwrap();
        assert_eq!(valid.pair.shear_modulus_pa(), f64::MAX);
        material.elastic_modulus.value = material.elastic_modulus.value.next_up();
        assert_eq!(
            select_for_member(&material, &base(), None, None)
                .unwrap_err()
                .code,
            "LOAD_STATE_MATERIAL_PAIR_INVALID"
        );
        material.poisson_ratio = Some(q(0.0, "1"));
        material.elastic_modulus = q(f64::from_bits(2), "Pa");
        assert_eq!(
            select_for_member(&material, &base(), None, None)
                .unwrap()
                .pair
                .shear_modulus_pa(),
            f64::from_bits(1)
        );
        material.elastic_modulus.value = f64::from_bits(1);
        assert!(select_for_member(&material, &base(), None, None).is_err());
        let mut material = sample();
        material.temperature_points[0].elastic_modulus = Some(q(f64::MAX, "Pa"));
        material.temperature_points[0].poisson_ratio = Some(q(-0.75, "1"));
        // Midpoint alone is admissible, but its actually consumed endpoint is not.
        assert!(IsotropicENu::new(0.5 * f64::MAX + 0.5 * 100e9, (-0.75 + 0.2) / 2.0).is_ok());
        for selected in [
            exact("point:cold"),
            temperature(20.0, "degC"),
            temperature(70.0, "degC"),
        ] {
            assert_eq!(
                select_for_member(&material, &selected, None, None)
                    .unwrap_err()
                    .code,
                "LOAD_STATE_MATERIAL_PAIR_INVALID"
            );
        }
    }
    #[test]
    fn bracket_selection_sorts_by_temperature_not_authored_order() {
        // N3 control: authored order [120, 20, 70] degC has no adjacent pair
        // bracketing 95 degC; only the temperature-sorted table finds 70..120.
        let mut material = sample();
        let mut middle = material.temperature_points[0].clone();
        middle.id = "point:middle".into();
        middle.temperature = Some(q(70.0, "degC"));
        middle.elastic_modulus = Some(q(180.0, "GPa"));
        middle.poisson_ratio = Some(q(-0.2, "1"));
        let cold = material.temperature_points[0].clone();
        let hot = material.temperature_points[1].clone();
        material.temperature_points = vec![hot, cold, middle];
        let selected =
            select_for_member(&material, &temperature(95.0, "degC"), None, None).unwrap();
        assert_eq!(
            selected
                .consumed_points
                .iter()
                .map(|p| p.point_id.as_str())
                .collect::<Vec<_>>(),
            vec!["point:middle", "point:hot"]
        );
        close(selected.pair.elastic_modulus_pa(), 140e9);
    }
    #[test]
    fn exact_selection_consumes_only_its_point_and_does_not_require_unrelated_property_pairs() {
        let mut material = sample();
        material.elastic_modulus = q(-1.0, "Pa");
        material.poisson_ratio = None;
        material.temperature_points[1].elastic_modulus = None;
        material.temperature_points[1].poisson_ratio = None;
        let exact = select_for_member(&material, &exact("point:cold"), None, None).unwrap();
        close(exact.pair.elastic_modulus_pa(), 200e9);
        let endpoint =
            select_for_member(&material, &temperature(20.0, "degC"), None, None).unwrap();
        assert_eq!(endpoint.pair, exact.pair);
        assert!(select_for_member(&material, &temperature(70.0, "degC"), None, None).is_err());
    }
}
