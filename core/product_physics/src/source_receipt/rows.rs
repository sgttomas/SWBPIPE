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
    if !(pressure_runtime::is_exact(input.model) && row.kind.ends_with("_v2"))
        && input
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
/// E15: the registered translation/support norm recipes are one correctly
/// rounded `norm3` of the published components; the range refusals are unchanged.
pub(super) fn finite_norm(values: [f64; 3]) -> Result<f64, ReceiptError> {
    let value = norm3(values[0], values[1], values[2]);
    if values.iter().any(|x| !x.is_finite())
        || !value.is_finite()
        || (values.iter().any(|x| *x != 0.0) && !value.is_normal())
    {
        return Err(bad("derived norm range"));
    }
    Ok(value)
}
pub(super) fn expected_primary_for(
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
            let row = ResultItem { component_body: None,
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
    // B4/D1 (b): every source-block receipt is a composite's, whose supports
    // publish the signed support rows.
    for support in selected.support_actions() {
        let authored = input
            .model
            .supports
            .iter()
            .find(|s| s.id == support.support_id)
            .ok_or_else(|| bad("composite support owner"))?;
        let mut generated = Vec::new();
        append_signed_support_results(
            &mut generated,
            input.load_case,
            authored,
            support.values,
        );
        for (component, row) in generated.into_iter().take(6).enumerate() {
            expected.insert(support.functional_indices[component], qualify(input, row));
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
    ResultItem { component_body: None,
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
/// One station's straight stress components on one section, with the
/// publication-range checks each basis's rows must pass (MPa range, torsion
/// intermediate and the open-formula summary range).
fn checked_station_stress(
    actions: &[f64; 6],
    section: &DerivedSection,
) -> Result<open_pipe_stress_stress_recovery::StressComponents, ReceiptError> {
    let stress = recover_section_stress(actions, section);
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
            || (action != 0.0 && (!mpa.is_normal() || !value.is_normal()))
        {
            return Err(bad("straight stress MPa publication range"));
        }
    }
    if actions[3] != 0.0 && !(actions[3] * section.torsion_radius).is_normal() {
        return Err(bad("straight stress torsion intermediate range"));
    }
    let local = open_formula_summary_mpa(&stress).ok_or_else(|| bad("summary unavailable"))?;
    let nonzero_normal = [
        c.axial_normal.unwrap(),
        c.bending_normal_y.unwrap(),
        c.bending_normal_z.unwrap(),
    ]
    .iter()
    .any(|v| *v != 0.0);
    if !local.is_finite()
        || (nonzero_normal && !local.is_normal())
        || (!nonzero_normal && local != 0.0)
    {
        return Err(bad("stress summary MPa publication range"));
    }
    Ok(stress.components)
}
pub(super) struct Derived {
    pub(super) row: ResultItem,
    pub(super) recipe: &'static str,
    pub(super) inputs: Vec<String>,
}
pub(super) fn derived_for(
    input: &source_recovery::Input<'_>,
    selected: &SelectedSourceRecovery,
    primary: &BTreeMap<usize, ResultItem>,
) -> Result<Vec<Derived>, ReceiptError> {
    // Euclidean norm is 1-Lipschitz relative to a componentwise relative
    // perturbation. Reserve an additional conservative 64 machine epsilons
    // for the fixed scaled recipe's rounding; affine bounds alone do not
    // establish the derivative criterion.
    let arithmetic_bound = 64.0 * f64::EPSILON;
    if selected.summary().max_relative_projection_error
        > (CRITERION - arithmetic_bound) / (1.0 + arithmetic_bound)
    {
        return Err(bad("derived norm error budget"));
    }
    // Stress and its summary are two arithmetic layers after projection. This
    // carrier conservatively admits only their normal nonzero range; the
    // additional allowance covers both layers rather than reusing the affine
    // criterion without room for their rounding.
    let stress_bound = 128.0 * f64::EPSILON;
    if !selected.members().is_empty()
        && selected.summary().max_relative_projection_error
            > (CRITERION - stress_bound) / (1.0 + stress_bound)
    {
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
    // B4/D1 (b): source-blocks-1's support resultant, straight-stress and
    // stress-summary recipes went with its preview case rows; a composite's
    // supports and members take its own recipes.
    for m in selected.members() {
        let section = input
            .built
            .sections
            .get(&m.member_id)
            .ok_or_else(|| bad("derived section source"))?;
        // D-5 and O4 (DEL-04-08 Design `o4-source-extension.md` §3.2): the
        // source-derived stress rows are published per basis, each from the
        // same section actions on its own section: the nominal (built)
        // section and the reduced section. Where the two are bit-equal the
        // reduced row is the nominal row bit for bit.
        let bases = input
            .built
            .exact_sections
            .get(&m.member_id)
            .copied()
            .ok_or_else(|| bad("derived section bases"))?;
        let reduced = exact_basis_section(bases, pressure_exact::SectionBasis::Reduced);
        for (s, location) in [
            (0, "end_i"),
            (1, "quarter_1"),
            (2, "midspan"),
            (3, "quarter_3"),
            (4, "end_j"),
        ] {
            let actions: [f64; 6] = m.sections[s];
            let mut components = Vec::with_capacity(2);
            for basis_section in [section, &reduced] {
                components.push(checked_station_stress(&actions, basis_section)?);
            }
            let mut generated = Vec::new();
            push_basis_twins(&mut generated, true, bad("derived stress twin rows"), |rows, basis| {
                let c = match basis {
                    pressure_exact::SectionBasis::Nominal => components[0],
                    pressure_exact::SectionBasis::Reduced => components[1],
                };
                if s == 0 || s == 4 {
                    append_endpoint_stress_results(
                        rows,
                        &m.member_id,
                        location,
                        &c,
                        STRAIGHT_ENDPOINT_SECTION_SIGN_CONVENTION,
                    );
                } else {
                    append_station_stress_results(
                        rows,
                        &m.member_id,
                        location,
                        &c,
                        "recovered_from_open_mechanics_stress_components",
                        None,
                    );
                }
                Ok(())
            })?;
            for row in generated {
                if !matches!(
                    row.kind.as_str(),
                    "element_local_axial_normal_stress"
                        | "element_local_bending_normal_stress_y"
                        | "element_local_bending_normal_stress_z"
                        | "element_local_torsional_shear_stress"
                ) {
                    return Err(bad("stress recipe component"));
                }
                out.push(Derived {
                    row: qualify(input, row),
                    recipe: "retained_source_straight_stress_v1",
                    inputs: vec![],
                });
            }
        }
    }
    Ok(out)
}
pub(super) fn bind_for(
    input: &source_recovery::Input<'_>,
    selected: &SelectedSourceRecovery,
    rows: &[ResultItem],
    bindings: &[FunctionalRowBinding],
    additional: Vec<Derived>,
) -> Result<Ledger, ReceiptError> {
    validate_case_rows(&input.load_case.id, rows)?;
    let primary = expected_primary_for(input, selected)?;
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
    for d in derived_for(input, selected, &primary)?
        .into_iter()
        .chain(additional)
    {
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
    // T4-U4b (DEL-04-07 Design u4b-u5b-implementation.md §4.3): the section
    // normal-stress extremes and transverse-shear maxima, formed from this
    // publication's own rows (the owner's Q2), carry `derived_not_qualified`:
    // no projection, no REQ-002 guarantee, no recipe work; their formula and
    // station inputs are named for the reader, which recomputes them.
    for d in not_qualified_for(rows)? {
        if ledger.contains_key(&d.result_id) {
            return Err(bad("derived_not_qualified row already treated"));
        }
        ledger.insert(d.result_id.clone(), d);
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
/// §4.3's two formula recipes, and the treatment that names them.
pub(super) const EXTREMES_RECIPE: &str = "section_extremes_from_published_stress_rows_v1";
pub(super) const SHEAR_RECIPE: &str = "transverse_shear_from_published_shear_rows_v1";
pub(super) const NOT_QUALIFIED: &str = "derived_not_qualified";
/// The station rows a §4.3 derived row reads, in formula order: the axial
/// and bending stress rows of its station and basis (σ⁺/σ⁻), or the two
/// shear rows of its station (τ: the section rows inside, the end-force rows
/// at the ends; no basis).
fn not_qualified_inputs<'a>(rows: &'a [ResultItem], row: &ResultItem) -> Result<(&'static str, Vec<&'a ResultItem>), ReceiptError> {
    let m = row.metadata.as_ref().ok_or_else(|| bad("derived_not_qualified metadata"))?;
    let suffix = pressure_exact::SectionBasis::BOTH
        .into_iter()
        .map(|b| b.basis_suffix())
        .find(|s| m.basis.ends_with(s))
        .ok_or_else(|| bad("derived_not_qualified section basis"))?;
    let find = |kind: &str, basis: Option<&str>| -> Result<&'a ResultItem, ReceiptError> {
        let mut found = rows.iter().filter(|r| {
            r.entity_ref == row.entity_ref
                && r.kind == kind
                && r.basis_ref == row.basis_ref
                && r.metadata.as_ref().is_some_and(|x| {
                    x.location == m.location && basis.is_none_or(|b| x.basis.ends_with(b))
                })
        });
        match (found.next(), found.next()) {
            (Some(r), None) => Ok(r),
            _ => Err(bad(format!("derived_not_qualified input: {kind}"))),
        }
    };
    if row.kind == u4b_rows::EXTREME_KIND {
        Ok((
            EXTREMES_RECIPE,
            vec![
                find("element_local_axial_normal_stress", Some(suffix))?,
                find("element_local_bending_normal_stress_y", Some(suffix))?,
                find("element_local_bending_normal_stress_z", Some(suffix))?,
            ],
        ))
    } else if row.kind == u4b_rows::SHEAR_MAX_KIND {
        Ok((
            SHEAR_RECIPE,
            vec![find("element_local_shear_force_y", None)?, find("element_local_shear_force_z", None)?],
        ))
    } else {
        Err(bad("derived_not_qualified kind"))
    }
}
/// §4.3: the `derived_not_qualified` treatments of a selected case's rows,
/// each bound to its recipe and station inputs. σ⁺/σ⁻ are re-formed from
/// their inputs bit for bit; τ is checked finite and nonnegative here (its
/// c_b is the reader's, from the evidence's ν and sections).
fn not_qualified_for(rows: &[ResultItem]) -> Result<Vec<RowTreatment>, ReceiptError> {
    let mut out = Vec::new();
    for row in rows.iter().filter(|r| r.kind == u4b_rows::EXTREME_KIND || r.kind == u4b_rows::SHEAR_MAX_KIND) {
        let (recipe, inputs) = not_qualified_inputs(rows, row)?;
        let m = row.metadata.as_ref().ok_or_else(|| bad("derived_not_qualified metadata"))?;
        let consistent = if recipe == EXTREMES_RECIPE {
            let e = open_pipe_stress_stress_recovery::section_stress_quantities::normal_stress_extremes(
                inputs[0].value,
                inputs[1].value,
                inputs[2].value,
            );
            row.unit == "MPa"
                && match m.component.as_str() {
                    "sigma_plus" => same(row.value, e.sigma_plus),
                    "sigma_minus" => same(row.value, e.sigma_minus),
                    _ => false,
                }
        } else {
            row.unit == "MPa" && m.component == "tau_v_max" && row.value.is_finite() && row.value >= 0.0
        };
        if !consistent {
            return Err(bad(format!("derived_not_qualified value: {}", row.id)));
        }
        out.push(RowTreatment {
            result_id: row.id.clone(),
            treatment: NOT_QUALIFIED,
            projection_id: None,
            recipe_id: Some(recipe),
            input_result_ids: inputs.iter().map(|r| r.id.clone()).collect(),
        });
    }
    Ok(out)
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
        "modulus_basis_record" | "combination_modulus_basis_record" => {
            row.unit == "record"
                && row.value == 1.0
                && m.component == "material_modulus_basis"
                && m.coordinate_system == "not_applicable"
        }
        _ => false,
    }
}

