use super::*;
use exact::functionals::{FunctionalQuantity, FunctionalUnit, MemberEnd};
const SECTION_SIGN:&str="positive value follows the j-side section action in the element-local frame (x toward end j); section equilibrium from stiffness-recovered end actions with consistent distributed-load fixed-end correction";
const SUPPORT_SIGN: &str =
    "support_on_pipe_positive_global_force_right_hand_couple_at_attachment_node";
const COMPONENTS: [&str; 6] = ["Fx", "Fy", "Fz", "Mx", "My", "Mz"];
pub(super) struct Ledger {
    pub projections: Vec<Projection>,
    pub rows: Vec<RowTreatment>,
    pub supports: Value,
}
fn qualify(input: &source_recovery::Input<'_>, mut row: ResultItem) -> ResultItem {
    if input
        .model
        .load_cases
        .first()
        .is_some_and(|c| c.id != input.load_case.id)
    {
        row.id = qualified_load_case_result_id(&input.load_case.id, &row.id);
    }
    row.basis_ref = Some(ResultBasisRef {
        ref_type: "load_case".into(),
        ref_id: input.load_case.id.clone(),
    });
    row
}
fn support_id(case: &str, support: &str, component: &str) -> String {
    format!(
        "result:support-component-v2:{}:{case}{}:{support}:{component}",
        case.len(),
        support.len()
    )
}
fn unit(u: FunctionalUnit) -> Result<&'static str, ReceiptError> {
    Ok(match u {
        FunctionalUnit::Millimetre => "mm",
        FunctionalUnit::Metre => "m",
        FunctionalUnit::Radian => "rad",
        FunctionalUnit::Newton => "N",
        FunctionalUnit::NewtonMetre => "N*m",
        _ => return Err(bad("unsupported projection unit")),
    })
}
pub(super) fn finite_norm(values: [f64; 3]) -> Result<f64, ReceiptError> {
    let value = scaled_norm(values);
    if values.iter().any(|x| !x.is_finite()) || !value.is_finite()
        || (values.iter().any(|x| *x != 0.0) && !value.is_normal()) {
        return Err(bad("derived norm range"));
    }
    Ok(value)
}
pub(super) fn expected_primary(
    input: &source_recovery::Input<'_>,
    selected: &SelectedSourceRecovery,
) -> Result<BTreeMap<usize, ResultItem>, ReceiptError> {
    let mut expected = BTreeMap::new();
    for (node_index, node) in input.model.nodes.iter().enumerate() {
        let mut rows = Vec::new();
        append_node_displacement_component_results(
            &mut rows,
            &node.id,
            selected.displacements(),
            node_index,
        );
        for (component, mut row) in rows.into_iter().enumerate() {
            let index = node_index * 6 + component;
            row.value = selected.published_nodal_components()[index];
            expected.insert(
                selected.published_nodal_functional_indices()[index],
                qualify(input, row),
            );
        }
    }
    for member in selected.members() {
        let mut end = Vec::new();
        append_element_force_results(&mut end, &member.member_id, &member.end_forces);
        // append emits i,j pairs for each of six components.
        for (i, row) in end.into_iter().enumerate() {
            let component = i / 2;
            let slot = component + (i % 2) * 6;
            expected.insert(member.end_functional_indices[slot], qualify(input, row));
        }
        for (station, location) in [(1, "quarter_1"), (2, "midspan"), (3, "quarter_3")] {
            let mut rows = Vec::new();
            append_station_force_results(
                &mut rows,
                &member.member_id,
                location,
                &member.sections[station],
                SECTION_RESULTANT_BASIS,
                SECTION_SIGN,
                "element_local",
            );
            for (c, row) in rows.into_iter().enumerate() {
                expected.insert(
                    member.section_functional_indices[station][c],
                    qualify(input, row),
                );
            }
        }
    }
    for support in selected.support_actions() {
        for c in 0..6 {
            let row = ResultItem {
                id: support_id(&input.load_case.id, &support.support_id, COMPONENTS[c]),
                kind: "support_reaction_component_v2".into(),
                value: support.values[c],
                unit: if c < 3 { "N" } else { "N*m" }.into(),
                entity_ref: support.support_id.clone(),
                basis_ref: None,
                source_result_refs: vec![],
                metadata: Some(ResultMetadata {
                    component: COMPONENTS[c].into(),
                    coordinate_system: "global".into(),
                    location: "node".into(),
                    basis: "recovered_from_assembled_support_law".into(),
                    sign_convention: SUPPORT_SIGN.into(),
                }),
            };
            expected.insert(support.functional_indices[c], qualify(input, row));
        }
    }
    Ok(expected)
}
fn descriptor_matches(
    index: usize,
    row: &ResultItem,
    selected: &SelectedSourceRecovery,
) -> Result<&'static str, ReceiptError> {
    let d = selected
        .retained()
        .descriptors()
        .get(index)
        .ok_or_else(|| bad("functional index"))?;
    if unit(d.key.unit)? != row.unit {
        return Err(bad("functional final unit"));
    }
    let metadata = row
        .metadata
        .as_ref()
        .ok_or_else(|| bad("projection metadata"))?;
    let quantity = match &d.key.quantity {
        FunctionalQuantity::NodeDisplacement { node, dof } => {
            if node != &row.entity_ref
                || metadata.coordinate_system != "global"
                || metadata.location != "node"
            {
                return Err(bad("nodal functional identity"));
            }
            if dof % 6 < 3 {
                "nodal_translation"
            } else {
                "nodal_rotation"
            }
        }
        FunctionalQuantity::MemberEnd {
            member,
            end,
            row: r,
        } => {
            if member != &row.entity_ref
                || metadata.location
                    != match end {
                        MemberEnd::I => "end_i",
                        MemberEnd::J => "end_j",
                    }
                || *r >= 6
            {
                return Err(bad("member end identity"));
            }
            "member_end_action"
        }
        FunctionalQuantity::MemberSection {
            member,
            station_bits,
            component,
        } => {
            let station = f64::from_bits(*station_bits);
            let location = if station == 0.25 {
                "quarter_1"
            } else if station == 0.5 {
                "midspan"
            } else if station == 0.75 {
                "quarter_3"
            } else {
                return Err(bad("unpublished endpoint section mapping"));
            };
            if member != &row.entity_ref || metadata.location != location || *component >= 6 {
                return Err(bad("section identity"));
            }
            "member_station_action"
        }
        FunctionalQuantity::SupportAction {
            support,
            node,
            component,
        } => {
            let c = *component as usize;
            if c >= 6
                || support != &row.entity_ref
                || metadata.component != COMPONENTS[c]
                || row.kind != "support_reaction_component_v2"
                || !selected.support_actions().iter().any(|s| {
                    s.support_id == *support
                        && s.node_index == *node
                        && s.functional_indices[c] == index
                })
                || d.key.convention != exact::functionals::FunctionalConvention::SupportOnStructure
            {
                return Err(bad("typed support functional identity"));
            }
            "support_action_component"
        }
        _ => return Err(bad("unallocated public functional")),
    };
    Ok(quantity)
}
fn projected(
    index: usize,
    row: &ResultItem,
    selected: &SelectedSourceRecovery,
) -> Result<Projection, ReceiptError> {
    let quantity = descriptor_matches(index, row, selected)?;
    let p = selected
        .retained()
        .projections()
        .iter()
        .find(|p| p.index() == index)
        .ok_or_else(|| bad("missing retained projection"))?;
    let interval = p.interval();
    let absolute = p.absolute_error_bound();
    let relative = p.relative_error_bound();
    if !same(p.value(), row.value)
        || !same(p.relative_limit(), CRITERION)
        || [p.value(), interval[0], interval[1], absolute, relative]
            .iter()
            .any(|v| !v.is_finite())
        || interval[0] > p.value()
        || p.value() > interval[1]
        || absolute < 0.0
        || relative < 0.0
        || relative > CRITERION
        || p.value() != 0.0 && interval[0] <= 0.0 && interval[1] >= 0.0
    {
        return Err(bad("projection binding/bounds"));
    }
    let basis = match p.basis() {
        exact::ProjectionBasis::ExactZero => "exact_zero",
        exact::ProjectionBasis::ExactIdentity => "exact_identity",
        exact::ProjectionBasis::OutwardInterval => "outward_interval",
    };
    if basis != "outward_interval"
        && (absolute != 0.0
            || relative != 0.0
            || !same(interval[0], p.value())
            || !same(interval[1], p.value()))
    {
        return Err(bad("exact projection claim"));
    }
    let id = source::functional_id(&selected.retained().descriptors()[index].key.case_id, index);
    Ok(Projection {
        projection_id: format!("projection:{id}"),
        functional_id: id,
        result_id: row.id.clone(),
        quantity,
        value: p.value(),
        value_bits: bits(p.value()),
        unit: unit(selected.retained().descriptors()[index].key.unit)?,
        interval,
        absolute_error_bound: absolute,
        relative_error_bound: relative,
        relative_limit: CRITERION,
        basis,
    })
}
fn compare(actual: &ResultItem, expected: &ResultItem) -> Result<(), ReceiptError> {
    if !same(actual.value, expected.value) || serialized(actual)? != serialized(expected)? {
        return Err(bad(format!("row semantic/value mismatch: {}", actual.id)));
    }
    Ok(())
}
fn bare(id: String, kind: &str, value: f64, unit: &str, entity: &str) -> ResultItem {
    ResultItem {
        id,
        kind: kind.into(),
        value,
        unit: unit.into(),
        entity_ref: entity.into(),
        basis_ref: None,
        source_result_refs: vec![],
        metadata: None,
    }
}
pub(super) struct Derived {
    pub(super) row: ResultItem,
    recipe: &'static str,
    inputs: Vec<String>,
}
pub(super) fn derived(
    input: &source_recovery::Input<'_>,
    selected: &SelectedSourceRecovery,
    primary: &BTreeMap<usize, ResultItem>,
) -> Result<Vec<Derived>, ReceiptError> {
    // Euclidean norm is 1-Lipschitz relative to a componentwise relative
    // perturbation. Reserve an additional conservative 64 machine epsilons
    // for the fixed scaled recipe's rounding; affine bounds alone do not
    // establish the derivative criterion.
    let arithmetic_bound = 64.0 * f64::EPSILON;
    if selected.summary().max_relative_projection_error >
        (CRITERION - arithmetic_bound) / (1.0 + arithmetic_bound) {
        return Err(bad("derived norm error budget"));
    }
    // Stress and its summary are two arithmetic layers after projection. This
    // carrier conservatively admits only their normal nonzero range; the
    // additional allowance covers both layers rather than reusing the affine
    // criterion without room for their rounding.
    let stress_bound = 128.0 * f64::EPSILON;
    if !selected.members().is_empty() && selected.summary().max_relative_projection_error >
        (CRITERION - stress_bound) / (1.0 + stress_bound) {
        return Err(bad("derived stress error budget"));
    }
    let mut out = Vec::new();
    for (i, node) in input.model.nodes.iter().enumerate() {
        let values = std::array::from_fn(|c| selected.published_nodal_components()[i * 6 + c]);
        let ids = (0..3)
            .map(|c| {
                primary[&selected.published_nodal_functional_indices()[i * 6 + c]]
                    .id
                    .clone()
            })
            .collect();
        out.push(Derived {
            row: qualify(
                input,
                bare(
                    format!("result:disp:{}", stable_suffix(&node.id)),
                    "displacement_magnitude",
                    finite_norm(values)?,
                    "mm",
                    &node.id,
                ),
            ),
            recipe: "translation_norm_scaled_v1",
            inputs: ids,
        });
    }
    for s in selected.support_actions() {
        let ids = (0..3)
            .map(|c| primary[&s.functional_indices[c]].id.clone())
            .collect();
        out.push(Derived {
            row: qualify(
                input,
                bare(
                    format!("result:reaction:{}", stable_suffix(&s.support_id)),
                    "reaction_resultant",
                    finite_norm([s.values[0], s.values[1], s.values[2]])?,
                    "N",
                    &s.support_id,
                ),
            ),
            recipe: "support_force_norm_scaled_v1",
            inputs: ids,
        });
    }
    for m in selected.members() {
        let section = input
            .built
            .sections
            .get(&m.member_id)
            .ok_or_else(|| bad("derived section source"))?;
        let mut summary = 0.0_f64;
        let mut stress_ids = Vec::new();
        for (s, location) in [
            (0, "end_i"),
            (1, "quarter_1"),
            (2, "midspan"),
            (3, "quarter_3"),
            (4, "end_j"),
        ] {
            let actions: [f64; 6] = std::array::from_fn(|component| {
                if s == 0 {
                    -primary[&m.end_functional_indices[component]].value
                } else if s == 4 {
                    primary[&m.end_functional_indices[6 + component]].value
                } else {
                    primary[&m.section_functional_indices[s][component]].value
                }
            });
            let stress = recover_section_stress(&actions, section, None);
            if !stress.findings.is_empty() {
                return Err(bad("straight stress source unavailable"));
            }
            let c = &stress.components;
            if [
                c.axial_normal,
                c.bending_normal_y,
                c.bending_normal_z,
                c.torsional_shear,
            ]
            .iter()
            .any(|v| v.is_none_or(|v| !v.is_finite()))
            {
                return Err(bad("straight stress finite check"));
            }
            for (value, action) in [
                (c.axial_normal.unwrap(), actions[0]),
                (c.bending_normal_y.unwrap(), actions[4]),
                (c.bending_normal_z.unwrap(), actions[5]),
                (c.torsional_shear.unwrap(), actions[3]),
            ] {
                // MPa is the actual component publication unit. Pa can still
                // meet the criterion when this division loses too much range.
                let mpa = value / 1e6;
                if (action == 0.0 && (value != 0.0 || mpa != 0.0))
                    || (action != 0.0 && (!mpa.is_normal() || !value.is_normal())) {
                    return Err(bad("straight stress MPa publication range"));
                }
            }
            if actions[3] != 0.0 && !(actions[3] * section.torsion_radius).is_normal() {
                return Err(bad("straight stress torsion intermediate range"));
            }
            let local = open_formula_summary_mpa(&stress, false)
                .ok_or_else(|| bad("summary unavailable"))?;
            let nonzero_normal = [c.axial_normal.unwrap(), c.bending_normal_y.unwrap(), c.bending_normal_z.unwrap()]
                .iter().any(|v| *v != 0.0);
            if !local.is_finite() || (nonzero_normal && !local.is_normal())
                || (!nonzero_normal && local != 0.0) {
                return Err(bad("stress summary MPa publication range"));
            }
            summary = summary.max(local);
            let mut generated = Vec::new();
            if s == 0 || s == 4 {
                append_endpoint_stress_results(
                    &mut generated,
                    &m.member_id,
                    location,
                    c,
                    false,
                    false,
                    STRAIGHT_ENDPOINT_SECTION_SIGN_CONVENTION,
                );
            } else {
                append_station_stress_results(
                    &mut generated,
                    &m.member_id,
                    location,
                    c,
                    false,
                    false,
                    "recovered_from_open_mechanics_stress_components",
                    None,
                );
            }
            for row in generated {
                let component = match row.kind.as_str() {
                    "element_local_axial_normal_stress" => 0,
                    "element_local_bending_normal_stress_y" => 4,
                    "element_local_bending_normal_stress_z" => 5,
                    "element_local_torsional_shear_stress" => 3,
                    _ => return Err(bad("stress recipe component")),
                };
                let function = if s == 0 {
                    m.end_functional_indices[component]
                } else if s == 4 {
                    m.end_functional_indices[6 + component]
                } else {
                    m.section_functional_indices[s][component]
                };
                let row = qualify(input, row);
                stress_ids.push(row.id.clone());
                out.push(Derived {
                    row,
                    recipe: "straight_open_stress_v1",
                    inputs: vec![primary[&function].id.clone()],
                });
            }
        }
        // No distributed/thermal/pressure terms enter this selected source. Its
        // normal components are affine along a straight span: sum of absolute
        // affine components is convex, hence its maximum is at an endpoint.
        // Evaluated five points remain checked binary64 recipes, no exact-max claim.
        out.push(Derived {
            row: qualify(
                input,
                bare(
                    format!("result:stress:{}", stable_suffix(&m.member_id)),
                    "open_formula_stress_summary",
                    summary,
                    "MPa",
                    &m.member_id,
                ),
            ),
            recipe: "reviewed_stress_summary_v1",
            inputs: stress_ids,
        });
    }
    Ok(out)
}
pub(super) fn bind(
    input: &source_recovery::Input<'_>,
    selected: &SelectedSourceRecovery,
    rows: &[ResultItem],
    bindings: &[FunctionalRowBinding],
) -> Result<Ledger, ReceiptError> {
    validate_case_rows(&input.load_case.id, rows)?;
    let primary = expected_primary(input, selected)?;
    if primary.len() != bindings.len() {
        return Err(bad("complete public functional binding count"));
    }
    let actual: BTreeMap<_, _> = rows.iter().map(|r| (r.id.as_str(), r)).collect();
    let mut by_index = BTreeMap::new();
    let mut used = BTreeSet::new();
    let mut projections = Vec::new();
    let mut ledger = BTreeMap::new();
    for binding in bindings {
        if by_index
            .insert(binding.functional_index, binding.result_id.clone())
            .is_some()
            || !used.insert(binding.result_id.clone())
        {
            return Err(bad("ambiguous functional mapping"));
        }
        let expected = primary
            .get(&binding.functional_index)
            .ok_or_else(|| bad("unexpected public functional"))?;
        if binding.result_id != expected.id {
            return Err(bad("functional result ID mapping"));
        }
        let row = *actual
            .get(binding.result_id.as_str())
            .ok_or_else(|| bad("mapped row missing"))?;
        compare(row, expected)?;
        let projection = projected(binding.functional_index, row, selected)?;
        ledger.insert(
            row.id.clone(),
            RowTreatment {
                result_id: row.id.clone(),
                treatment: "qualified_projection",
                projection_id: Some(projection.projection_id.clone()),
                recipe_id: None,
                input_result_ids: vec![],
            },
        );
        projections.push(projection);
    }
    for d in derived(input, selected, &primary)? {
        let row = *actual
            .get(d.row.id.as_str())
            .ok_or_else(|| bad(format!("required derived row missing: {}", d.row.id)))?;
        compare(row, &d.row)?;
        if d.inputs.iter().any(|id| !actual.contains_key(id.as_str())) {
            return Err(bad("derived input reference missing"));
        }
        ledger.insert(
            row.id.clone(),
            RowTreatment {
                result_id: row.id.clone(),
                treatment: "checked_derived",
                projection_id: None,
                recipe_id: Some(d.recipe),
                input_result_ids: d.inputs,
            },
        );
    }
    for r in rows {
        ledger.entry(r.id.clone()).or_insert_with(|| RowTreatment {
            result_id: r.id.clone(),
            treatment: if observation(r) {
                "checked_derived"
            } else {
                "inspection_only"
            },
            projection_id: None,
            recipe_id: None,
            input_result_ids: r.source_result_refs.clone(),
        });
    }
    // Observation rows are explicitly inspection-only; they do not acquire a
    // fake derived recipe. Final qualification excludes them from physical use.
    for r in rows {
        if observation(r) {
            let l = ledger.get_mut(&r.id).unwrap();
            l.treatment = "inspection_only";
        }
    }
    let mut supports = Vec::new();
    for s in selected.support_actions() {
        let owned = input
            .built
            .supports
            .iter()
            .find(|x| x.support_id == s.support_id)
            .ok_or_else(|| bad("support ownership"))?;
        let mut components = Vec::new();
        for c in 0..6 {
            let dof = s.node_index * 6 + c;
            let mut terms = Vec::new();
            if owned.family != SupportFamily::Spring
                && owned.restrained_dofs.iter().any(|d| dof_index(*d) == c)
            {
                terms.push(
                    json!({"kind":"ideal_constraint","source_id":s.support_id,"global_dof":dof}),
                );
            }
            for spring in input
                .spring_entries
                .iter()
                .filter(|x| x.support_id == s.support_id && x.node_dof.global_index() == dof)
            {
                terms.push(
                    json!({"kind":"ground_spring","source_id":spring.support_id,"global_dof":dof}),
                );
            }
            if terms.is_empty() {
                terms.push(
                    json!({"kind":"structural_zero","source_id":s.support_id,"global_dof":dof}),
                );
            }
            components.push(json!({"component":COMPONENTS[c],"result_id":by_index[&s.functional_indices[c]],"functional_id":source::functional_id(&input.load_case.id,s.functional_indices[c]),"action_terms":terms}));
        }
        supports.push(json!({"support_id":s.support_id,"node_id":input.model.nodes[s.node_index].id,"attribution":"unique_source_owned","components":components}));
    }
    Ok(Ledger {
        projections,
        rows: rows.iter().map(|r| ledger.remove(&r.id).unwrap()).collect(),
        supports: json!(supports),
    })
}
pub(super) fn observation(row: &ResultItem) -> bool {
    let Some(m) = &row.metadata else {
        return false;
    };
    if !row.value.is_finite() {
        return false;
    }
    match row.kind.as_str() {
        "linear_solver_mode_basis" => {
            row.unit == "mode_code"
                && row.entity_ref == "solver:linear_static_preview"
                && m.component == "linear_solver_mode"
                && m.coordinate_system == "reduced_system"
                && matches!(row.value, 1.0 | 2.0 | 3.0)
        }
        "sparse_live_path_dense_parity_relative_delta" => {
            row.unit == "unitless"
                && row.entity_ref == "solver:sparse_direct"
                && m.component == "sparse_live_path"
                && m.coordinate_system == "reduced_system"
                && row.value >= 0.0
        }
        "modulus_basis_record" | "combination_modulus_basis_record" => {
            row.unit == "record"
                && row.value == 1.0
                && m.component == "material_modulus_basis"
                && m.coordinate_system == "not_applicable"
        }
        _ => false,
    }
}

pub(super) fn validate_summary(envelope: &MechanicsEnvelope) -> Result<(), ReceiptError> {
    for (headline, kind) in [
        (&envelope.summary.max_displacement, "displacement_magnitude"),
        (
            &envelope.summary.max_open_formula_stress,
            "open_formula_stress_summary",
        ),
    ] {
        if let Some(h) = headline {
            let row = envelope
                .results
                .iter()
                .find(|r| r.id == h.result_ref)
                .ok_or_else(|| bad("summary missing row"))?;
            if row.kind != kind
                || row.entity_ref != h.location_ref
                || row.unit != h.unit
                || !same(row.value, h.value)
            {
                return Err(bad("summary binding"));
            }
            // Existing headline is first requested case's maximum.
            if envelope
                .results
                .iter()
                .any(|r| r.kind == kind && r.basis_ref == row.basis_ref && r.value > h.value)
            {
                return Err(bad("summary is not actual maximum"));
            }
        } else if envelope.results.iter().any(|r| r.kind == kind) {
            return Err(bad("required summary missing"));
        }
    }
    Ok(())
}
