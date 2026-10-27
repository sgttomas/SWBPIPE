//! Source-faithful product producers for K-D5's intended loads.
use super::*;
use open_pipe_stress_frame_kernel::structural::{
    CoefficientPrimitive, EigenstrainPrimitive, ExpansionLawPrimitive, FitPrimitive,
    IntensityPrimitive, LoadFormula, LoadPrimitive, LoadSummandIdentity, MassPrimitive,
    ThermalPointPrimitive, ThermalPrimitive,
};

pub(super) fn constant_alpha(alpha: f64, dt: f64) -> EigenstrainPrimitive {
    EigenstrainPrimitive {
        thermal: ThermalPrimitive::ConstantAlphaInterval {
            coefficient_per_kelvin: alpha,
            temperature_change_kelvin: dt,
        },
        fit: FitPrimitive::None,
    }
}
pub(super) fn eigenstrain(
    thermal: &case_state::thermal::ThermalInput,
    fit: case_state::thermal::FitInput,
) -> EigenstrainPrimitive {
    use case_state::thermal::{
        CoefficientData as C, FitInput as F, NormalizedExpansionLaw as L, ThermalInput as T,
    };
    let points = |p: &[case_state::thermal::ThermalPoint]| {
        p.iter()
            .map(|p| ThermalPointPrimitive {
                temperature_kelvin: p.temperature_kelvin,
                value: p.value,
            })
            .collect()
    };
    let law = |l: &L| match l {
        L::EngineeringSecant { datum_kelvin, data } => ExpansionLawPrimitive::EngineeringSecant {
            datum_kelvin: *datum_kelvin,
            data: match data {
                C::Constant(c) => CoefficientPrimitive::Constant(*c),
                C::Table(p) => CoefficientPrimitive::Table(points(p)),
            },
        },
        L::EngineeringDilation {
            datum_kelvin,
            points: p,
        } => ExpansionLawPrimitive::EngineeringDilation {
            datum_kelvin: *datum_kelvin,
            points: points(p),
        },
        L::DifferentialPerDatumLength {
            datum_kelvin,
            points: p,
        } => ExpansionLawPrimitive::DifferentialPerDatumLength {
            datum_kelvin: *datum_kelvin,
            points: points(p),
        },
        L::LogarithmicPerCurrentLength {
            datum_kelvin,
            points: p,
        } => ExpansionLawPrimitive::LogarithmicPerCurrentLength {
            datum_kelvin: *datum_kelvin,
            points: points(p),
        },
    };
    EigenstrainPrimitive {
        thermal: match thermal {
            T::UnchangedReference => ThermalPrimitive::UnchangedReference,
            T::ExplicitIntervalStrain { strain } => {
                ThermalPrimitive::ExplicitIntervalStrain { strain: *strain }
            }
            T::ConstantAlphaInterval {
                coefficient_per_kelvin,
                temperature_change_kelvin,
            } => ThermalPrimitive::ConstantAlphaInterval {
                coefficient_per_kelvin: *coefficient_per_kelvin,
                temperature_change_kelvin: *temperature_change_kelvin,
            },
            T::FreeLengthState {
                installation_kelvin,
                operating_kelvin,
                law: l,
            } => ThermalPrimitive::FreeLengthState {
                installation_kelvin: *installation_kelvin,
                operating_kelvin: *operating_kelvin,
                law: law(l),
            },
        },
        fit: match fit {
            F::None => FitPrimitive::None,
            F::EngineeringStrain { strain } => FitPrimitive::EngineeringStrain { strain },
            F::NaturalLengthChange {
                change_m,
                reference_length_m,
            } => FitPrimitive::NaturalLengthChange {
                change_m,
                reference_length_m,
            },
        },
    }
}

/// Complete producer-source enumeration, before collecting the supplied primitives.
/// Keeping this source list separate from the supplied vector makes an omitted
/// or duplicated supplied summand disagree with its original manifest.
pub(super) struct Summand {
    pub identity: LoadSummandIdentity,
    pub local_row: usize,
    pub formula: LoadFormula,
}
pub(super) fn summand(
    family: &str,
    source: &str,
    ordinal: usize,
    local_row: usize,
    formula: LoadFormula,
) -> Summand {
    Summand {
        identity: LoadSummandIdentity {
            family: family.into(),
            source: source.into(),
            ordinal,
        },
        local_row,
        formula,
    }
}
pub(super) fn attach(ledger: &mut LoadLedger, term_index: usize, sources: Vec<Summand>) {
    // The expected manifest comes from the complete source enumeration. It is
    // installed before the distinct supplied-primitive collection starts.
    let expected = sources
        .iter()
        .map(|source| {
            LoadPrimitive {
                term_index,
                identity: source.identity.clone(),
                local_row: source.local_row,
                formula: source.formula.clone(),
                force_scale_exponent: 0,
            }
            .closure_entry()
        })
        .collect();
    assert!(ledger.set_closure_manifest(term_index, expected));
    #[cfg_attr(not(test), allow(unused_mut))]
    let mut supplied: Vec<LoadPrimitive> = sources
        .into_iter()
        .map(|source| LoadPrimitive {
            term_index,
            identity: source.identity,
            local_row: source.local_row,
            formula: source.formula,
            force_scale_exponent: 0,
        })
        .collect();
    #[cfg(test)]
    supplied_mutant::apply(term_index, &mut supplied);
    for primitive in supplied {
        ledger.attach_load_primitive(primitive);
    }
}

/// Test-only mutants of the supplied primitive collection (never of the
/// expected manifest, which is installed first): K-D5 Design section 7,
/// "The closure" mutants on the integrated producer.
#[cfg(test)]
pub(super) mod supplied_mutant {
    use super::LoadPrimitive;
    use std::cell::RefCell;

    /// Returns true when it mutated this term's supplied primitives; it is
    /// then disarmed, so exactly one term is mutated per ledger.
    pub(crate) type Mutant = Box<dyn Fn(usize, &mut Vec<LoadPrimitive>) -> bool>;
    thread_local! {
        static MUTANT: RefCell<Option<Mutant>> = const { RefCell::new(None) };
        static APPLIED: RefCell<Option<usize>> = const { RefCell::new(None) };
    }
    pub(crate) fn arm(mutant: Mutant) {
        MUTANT.with(|m| *m.borrow_mut() = Some(mutant));
        APPLIED.with(|a| *a.borrow_mut() = None);
    }
    /// Disarms and returns the mutated term, if any.
    pub(crate) fn disarm() -> Option<usize> {
        MUTANT.with(|m| *m.borrow_mut() = None);
        APPLIED.with(|a| a.borrow_mut().take())
    }
    pub(super) fn apply(term_index: usize, supplied: &mut Vec<LoadPrimitive>) {
        if APPLIED.with(|a| a.borrow().is_some()) {
            return;
        }
        let hit = MUTANT.with(|m| m.borrow().as_ref().is_some_and(|mutant| mutant(term_index, supplied)));
        if hit {
            APPLIED.with(|a| *a.borrow_mut() = Some(term_index));
        }
    }
}
pub(super) fn attach_last(ledger: &mut LoadLedger, source: Summand) {
    attach(ledger, ledger.term_count() - 1, vec![source]);
}

/// The generator's mass operands of a section. W-16 slice c (DEL-05-01
/// Design `exact-equivalent-static.md` §2): on the exact route the section's
/// authored `Some(c_a)` (zero included) selects the D-5 basis the generator
/// used (`self_weight::EXACT_MASS_METHOD`); the preview route keeps `None`.
fn mass(section: &PipeSectionInput, exact: bool) -> MassPrimitive {
    MassPrimitive {
        outer_diameter: section.outside_diameter.value,
        wall_thickness: section.wall_thickness.value,
        mill_tolerance: section.mill_tolerance.as_ref().map_or(0.0, |q| q.value),
        pipe_density: section.material_density.as_ref().map_or(0.0, |q| q.value),
        contents_density: section.contents_density.as_ref().map_or(0.0, |q| q.value),
        insulation_thickness: section
            .insulation_thickness
            .as_ref()
            .map_or(0.0, |q| q.value),
        insulation_density: section.insulation_density.as_ref().map_or(0.0, |q| q.value),
        corrosion_allowance: if exact {
            section.corrosion_allowance.as_ref().map(|q| q.value)
        } else {
            None
        },
    }
}
pub(super) fn intensity(
    model: &PreviewModel,
    load: &open_pipe_stress_primitive_loads::ElementUniformLoadContribution,
    case_id: &str,
) -> IntensityPrimitive {
    let axis = load.direction.dof_index();
    let mut direction = [0.0; 3];
    if axis < 3 {
        direction[axis] = 1.0;
    }
    let mut authored = direction.map(|d| d * load.magnitude.value);
    // Rotational distributed loads are already outside the admitted producer.
    if axis >= 3 {
        authored = [f64::NAN; 3];
    }
    let pipe = &model.pipe_segments[load.element_index];
    let factored = source_factor(model, case_id, &load.load_id);
    if let Some(case) = model.load_cases.iter().find(|case| case.id == case_id) {
        // W-16 slice c (§2): dispatch to the generator's operands only for a
        // load the generation produced; an authored id resembling the
        // generated prefix stays authored.
        if let Some(generation) = &case.equivalent_static {
            if is_generated_equivalent_static_load(case, &load.load_id, Some("seismic")) {
                if let Some(seismic) = &generation.seismic {
                    let coefficient = [
                        &seismic.g_factor_x,
                        &seismic.g_factor_y,
                        &seismic.g_factor_z,
                    ][axis]
                        .as_ref()
                        .map_or(f64::NAN, |q| q.value);
                    return IntensityPrimitive::Seismic {
                        mass: mass(&pipe.section, pressure_runtime::is_exact(model)),
                        coefficient,
                        gravity: seismic
                            .gravity_acceleration
                            .as_ref()
                            .map_or(f64::NAN, |q| q.value),
                        direction,
                    };
                }
            }
            if is_generated_equivalent_static_load(case, &load.load_id, Some("wind")) {
                if let Some(wind) = &generation.wind {
                    return IntensityPrimitive::Wind {
                        pressure: wind.pressure.as_ref().map_or(f64::NAN, |q| q.value),
                        shape_factor: wind.shape_factor.as_ref().map_or(f64::NAN, |q| q.value),
                        outer_diameter: pipe.section.outside_diameter.value,
                        insulation_thickness: pipe
                            .section
                            .insulation_thickness
                            .as_ref()
                            .map_or(0.0, |q| q.value),
                        direction,
                    };
                }
            }
        }
        if let Some(primitive) = case
            .primitive_loads
            .iter()
            .find(|primitive| primitive.id == load.load_id)
        {
            if let Some(evidence) = primitive
                .provenance
                .as_deref()
                .and_then(|text| serde_json::from_str::<serde_json::Value>(text).ok())
            {
                if evidence["method"] == self_weight::GENERATED_METHOD {
                    // This source provenance has already passed the applied-weight
                    // validation. A manual override is an authored intensity.
                    let q = |name: &str| {
                        evidence["mass_inputs"][name]["value"]
                            .as_f64()
                            .unwrap_or(0.0)
                    };
                    let mass = MassPrimitive {
                        outer_diameter: q("outside_diameter"),
                        wall_thickness: q("wall_thickness"),
                        mill_tolerance: q("mill_tolerance"),
                        pipe_density: q("material_density"),
                        contents_density: q("contents_density"),
                        insulation_thickness: q("insulation_thickness"),
                        insulation_density: q("insulation_density"),
                        // D-5: the exact mass method names its allowance.
                        corrosion_allowance: (evidence["mass_method"]
                            == self_weight::EXACT_MASS_METHOD)
                            .then(|| q("corrosion_allowance")),
                    };
                    let gravity = evidence["normalized_acceleration_m_per_s2"]
                        .as_f64()
                        .unwrap_or(f64::NAN);
                    return if let Some((_, factor)) = factored {
                        IntensityPrimitive::Seismic {
                            mass,
                            coefficient: factor,
                            gravity,
                            direction,
                        }
                    } else {
                        IntensityPrimitive::SelfWeight {
                            mass,
                            gravity: direction.map(|d| d * gravity),
                        }
                    };
                }
            }
        }
    }
    if let Some((magnitude, factor)) = factored {
        IntensityPrimitive::Product {
            factors: vec![magnitude, factor],
            direction,
        }
    } else {
        IntensityPrimitive::Authored(authored)
    }
}

pub(super) fn member_source(member: &str, source: &str) -> String {
    format!("member:{}:{member}:{source}", member.len())
}

/// Omit only source states that are exactly unchanged. A rounded zero total
/// cannot authorize dropping nonzero thermal/fit source operands.
pub(super) fn has_eigen_source(
    thermal: &case_state::thermal::ThermalInput,
    fit: case_state::thermal::FitInput,
) -> bool {
    use case_state::thermal::{FitInput as F, ThermalInput as T};
    let thermal_zero = match thermal {
        T::UnchangedReference => true,
        T::ExplicitIntervalStrain { strain } => *strain == 0.0,
        T::ConstantAlphaInterval {
            coefficient_per_kelvin,
            temperature_change_kelvin,
        } => *coefficient_per_kelvin == 0.0 || *temperature_change_kelvin == 0.0,
        T::FreeLengthState {
            installation_kelvin,
            operating_kelvin,
            ..
        } => installation_kelvin == operating_kelvin,
    };
    let fit_zero = match fit {
        F::None => true,
        F::EngineeringStrain { strain } => strain == 0.0,
        F::NaturalLengthChange { change_m, .. } => change_m == 0.0,
    };
    !thermal_zero || !fit_zero
}

/// Natural-length fit uses the owning member's raw geometry at p; the
/// rounded length in the ordinary resolved load is never the intended operand.
pub(super) fn member_eigenstrain(
    strain: &EigenstrainPrimitive,
    pipe: &StraightPipeElement,
    bend: Option<&CurvedBendMacroBuild>,
) -> EigenstrainPrimitive {
    let mut source = strain.clone();
    if let FitPrimitive::NaturalLengthChange { change_m, .. } = source.fit {
        source.fit = if let Some(bend) = bend {
            FitPrimitive::NaturalLengthChangeCurved {
                change_m,
                arc: curved_formation_of(bend),
            }
        } else {
            FitPrimitive::NaturalLengthChangeStraight {
                change_m,
                coordinates_i: pipe.node_i.coordinates,
                coordinates_j: pipe.node_j.coordinates,
            }
        };
    }
    source
}

/// The complete authored source/factor pair, before the ordinary case fold.
pub(super) fn source_factor(
    model: &PreviewModel,
    case_id: &str,
    load_id: &str,
) -> Option<(f64, f64)> {
    if !case_state::is_load_state(model) {
        return None;
    }
    let case = model.load_cases.iter().find(|case| case.id == case_id)?;
    let state = case.analysis_state.value()?;
    let source = state
        .load_sources
        .iter()
        .find(|source| source.source_ref == load_id)?;
    let load = case
        .primitive_loads
        .iter()
        .find(|load| load.id == load_id)?;
    Some((load.magnitude.value, source.factor))
}
/// Factored nodal values are formed products, while unscaled authored nodal
/// loads remain exact input terms under the normalized-input contract.
pub(super) fn nodal(
    ledger: &mut LoadLedger,
    model: &PreviewModel,
    application: &LoadApplication,
    case_id: &str,
    node_count: usize,
) {
    for (ordinal, load) in application.nodal_loads.iter().enumerate() {
        if load.global_dof >= 6 * node_count {
            continue;
        }
        if let Some((magnitude, factor)) = source_factor(model, case_id, &load.load_id) {
            ledger.push_formed(
                &load.load_id,
                load.global_dof,
                load.value,
                Formation::RoundedProduct {
                    k: 1.0,
                    a: magnitude,
                    b: factor,
                },
                0.0,
                false,
            );
            attach_last(
                ledger,
                summand(
                    "nodal_product",
                    &load.load_id,
                    ordinal,
                    load.global_dof % 6,
                    LoadFormula::NodalProduct {
                        global_dof: load.global_dof,
                        magnitude,
                        factor,
                    },
                ),
            );
        } else {
            ledger.push(&load.load_id, load.global_dof, load.value);
        }
    }
}
