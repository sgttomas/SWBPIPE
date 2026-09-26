//! Composite-only row admission. Every derived action uses the owned response;
//! private numeric transports never establish custody or replay by themselves.
use super::endpoint_maximum as maximum;
use super::*;
use exact::functionals::{FunctionalConvention, FunctionalQuantity, FunctionalUnit};

const MAX_BASIS: &str = "retained_source_endpoint_normal_max_v1";
const RECIPE_WORK: usize = 4096;

fn charge(selected: &mut SelectedSourceRecovery, work: usize) -> Result<(), ReceiptError> {
    selected.charge_finalization(work).map_err(|e| {
        ReceiptError(
            format!("composite recipe reservation: {e:?}"),
            Some(selected.summary().work),
        )
    })
}
fn endpoint_name(endpoint: maximum::Endpoint) -> &'static str {
    match endpoint {
        maximum::Endpoint::I => "i",
        maximum::Endpoint::J => "j",
    }
}
pub(in super::super) struct CompositeMaximum {
    value: f64,
    evidence: Value,
}
impl CompositeMaximum {
    pub(in super::super) fn value_pa(&self) -> f64 {
        self.value
    }
    pub(in super::super) fn evidence(&self, result_id: &str) -> Value {
        let mut evidence = self.evidence.clone();
        evidence["result_id"] = json!(result_id);
        evidence
    }
}
fn maximum_inner(
    input: &source_recovery::Input<'_>,
    selected: &SelectedSourceRecovery,
    member_id: &str,
    identity_digest: &str,
) -> Result<CompositeMaximum, ReceiptError> {
    if !pressure_runtime::is_exact(input.model)
        || !input
            .load_case
            .pressure_regions
            .as_ref()
            .is_some_and(Vec::is_empty)
    {
        return Err(bad(
            "endpoint maximum requires exact empty pressure inventory",
        ));
    }
    let member = selected
        .members()
        .iter()
        .find(|m| m.member_id == member_id)
        .ok_or_else(|| bad("maximum member"))?;
    let geometry = input
        .built
        .exact_sections
        .get(member_id)
        .ok_or_else(|| bad("maximum source geometry"))?;
    let section = input
        .built
        .sections
        .get(member_id)
        .ok_or_else(|| bad("maximum recovery section"))?;
    if !same(section.area, geometry.wall_area_m2())
        || !same(section.section_modulus, geometry.section_modulus_m3())
    {
        return Err(bad("maximum source section mismatch"));
    }
    let mut endpoints = Vec::new();
    let mut transport = Vec::new();
    for station in [0usize, 4] {
        let indices = [0usize, 4, 5].map(|c| member.section_functional_indices[station][c]);
        let mut arguments = Vec::new();
        let mut actions = Vec::new();
        for (component, index) in [0usize, 4, 5].into_iter().zip(indices) {
            let descriptor = selected
                .retained()
                .descriptors()
                .get(index)
                .ok_or_else(|| bad("maximum descriptor missing"))?;
            if descriptor.key.case_id != input.load_case.id
                || descriptor.key.convention != FunctionalConvention::SectionLocal
                || descriptor.key.unit
                    != if component == 0 {
                        FunctionalUnit::Newton
                    } else {
                        FunctionalUnit::NewtonMetre
                    }
                || descriptor.key.quantity
                    != (FunctionalQuantity::MemberSection {
                        member: member_id.into(),
                        station_bits: source_recovery::STATIONS[station].to_bits(),
                        component: component as u8,
                    })
            {
                return Err(bad("maximum endpoint functional binding"));
            }
            let projection = selected
                .retained()
                .projections()
                .iter()
                .find(|p| p.index() == index)
                .ok_or_else(|| bad("maximum projection missing"))?;
            if !same(projection.value(), member.sections[station][component]) {
                return Err(bad("maximum action binding"));
            }
            arguments.push(
                maximum::ProjectionBounds::from_retained_parts(
                    index,
                    projection.value(),
                    projection.interval(),
                )
                .map_err(|e| bad(format!("maximum action bounds: {e:?}")))?,
            );
            actions.push(json!({"value":projection.value(),"interval":projection.interval()}));
        }
        transport.push(
            <[maximum::ProjectionBounds; 3]>::try_from(arguments)
                .map_err(|_| bad("maximum endpoint count"))?,
        );
        endpoints.push(json!({"station_fraction":source_recovery::STATIONS[station],"functional_indices":indices,"functional_ids":indices.map(|i|source::functional_id(&input.load_case.id,i)),"actions":actions}));
    }
    let result = maximum::endpoint_maximum(
        section.area,
        section.section_modulus,
        [transport[0], transport[1]],
    )
    .map_err(|e| bad(format!("endpoint maximum: {e:?}")))?;
    for (e, computed) in endpoints.iter_mut().zip(result.endpoints) {
        e["value_pa"] = json!(computed.value_pa);
        e["interval_pa"] = json!(computed.interval_pa);
    }
    let locations = match result.locations {
        maximum::Locations::StrictEndpoint(e) => {
            json!({"kind":"strict_endpoint","endpoint":endpoint_name(e)})
        }
        maximum::Locations::WholeSpanConstant => json!({"kind":"whole_span_constant"}),
        maximum::Locations::EndpointCandidates {
            exact_tie_proven,
            interior_equal_possible,
        } => {
            json!({"kind":"endpoint_candidates","exact_tie_proven":exact_tie_proven,"interior_equal_possible":interior_equal_possible})
        }
    };
    Ok(CompositeMaximum {
        value: result.value_pa,
        evidence: json!({
            "load_case_id":input.load_case.id,"pipe_id":member_id,"basis":MAX_BASIS,
            "coefficient_basis":"retained_section_functionals_binary64",
            "source_identity_sha256":identity_digest,
            "area_m2":section.area,"section_modulus_m3":section.section_modulus,"value_pa":result.value_pa,
            "value_lower_pa":result.interval_pa[0],"value_upper_pa":result.interval_pa[1],
            "absolute_error_bound_pa":result.absolute_error_bound_pa,"relative_error_bound":result.relative_error_bound,"relative_limit":CRITERION,
            "station_fraction":if result.witness == maximum::Endpoint::I {0.0}else{1.0},"locations":locations,"endpoints":endpoints,
            "enclosure_scope":"retained_action_projection_and_ordered_binary64_recipe;represented_source_section_operands"
        }),
    })
}
pub(in super::super) fn composite_member_maximum(
    input: &source_recovery::Input<'_>,
    selected: &mut SelectedSourceRecovery,
    member_id: &str,
) -> Result<CompositeMaximum, ReceiptError> {
    #[cfg(test)]
    trace("derived_recipe_entry", selected);
    charge(selected, RECIPE_WORK)?;
    let digest = selected.retained_identity_digest().map_err(|e| {
        ReceiptError(
            format!("source identity digest: {e:?}"),
            Some(selected.summary().work),
        )
    })?;
    maximum_inner(input, selected, member_id, &digest).map_err(|mut e| {
        e.1 = Some(selected.summary().work);
        e
    })
}

#[derive(Debug)]
struct Norm {
    value: f64,
    interval: [f64; 2],
    absolute: f64,
    relative: f64,
}
fn outward(v: f64, upper: bool) -> Result<f64, ReceiptError> {
    if !v.is_finite() || v < 0.0 {
        return Err(bad("support norm arithmetic range"));
    }
    let result = if upper {
        f64::from_bits(v.to_bits() + 1)
    } else if v == 0.0 {
        0.0
    } else {
        f64::from_bits(v.to_bits() - 1)
    };
    if !result.is_finite() {
        return Err(bad("support norm outward range"));
    }
    Ok(result)
}
fn norm_corner(values: [f64; 3], upper: bool) -> Result<f64, ReceiptError> {
    let m = values.into_iter().fold(0.0_f64, f64::max);
    if m == 0.0 {
        return Ok(0.0);
    }
    let mut squares = [0.0; 3];
    for (i, v) in values.into_iter().enumerate() {
        if v == 0.0 {
            continue;
        }
        if v == m {
            squares[i] = 1.0;
            continue;
        }
        let ratio = outward(v / m, upper)?;
        squares[i] = outward(ratio * ratio, upper)?;
    }
    let add = |a: f64, b: f64| {
        if a == 0.0 {
            Ok(b)
        } else if b == 0.0 {
            Ok(a)
        } else {
            outward(a + b, upper)
        }
    };
    let sum = add(add(squares[0], squares[1])?, squares[2])?;
    let root = if sum == 1.0 {
        1.0
    } else {
        outward(sum.sqrt(), upper)?
    };
    if root == 1.0 {
        Ok(m)
    } else {
        outward(m * root, upper)
    }
}
fn norm(values: [f64; 3], intervals: [[f64; 2]; 3]) -> Result<Norm, ReceiptError> {
    if values.iter().any(|v| !v.is_finite())
        || intervals
            .iter()
            .zip(values)
            .any(|(p, v)| !p[0].is_finite() || !p[1].is_finite() || p[0] > v || v > p[1])
    {
        return Err(bad("support norm action interval"));
    }
    let abs = intervals.map(|[lo, hi]| {
        if lo >= 0.0 {
            [lo, hi]
        } else if hi <= 0.0 {
            [-hi, -lo]
        } else {
            [0.0, (-lo).max(hi)]
        }
    });
    let lo = norm_corner(abs.map(|p| p[0]), false)?;
    let hi = norm_corner(abs.map(|p| p[1]), true)?;
    let value = scaled_norm(values);
    if !value.is_finite() || value < lo || value > hi {
        return Err(bad("support norm representative range"));
    }
    let absolute = if lo == value && value == hi {
        0.0
    } else {
        outward((value - lo).max(hi - value), true)?
    };
    let relative = if hi == 0.0 {
        0.0
    } else {
        if !lo.is_normal() || !hi.is_normal() || !value.is_normal() {
            return Err(bad("support norm publication range"));
        }
        if absolute == 0.0 {
            0.0
        } else {
            outward(absolute / lo, true)?
        }
    };
    if relative > CRITERION {
        return Err(bad("support norm protected criterion"));
    }
    Ok(Norm {
        value,
        interval: [lo, hi],
        absolute,
        relative,
    })
}
fn support_norm_checks(
    input: &source_recovery::Input<'_>,
    selected: &SelectedSourceRecovery,
    id: &str,
) -> Result<([f64; 2], Vec<Value>), ReceiptError> {
    let support = selected
        .support_actions()
        .iter()
        .find(|s| s.support_id == id)
        .ok_or_else(|| bad("support norm owner"))?;
    let mut checks = Vec::new();
    let mut values = [0.0; 2];
    for group in 0..2 {
        let indices = std::array::from_fn::<_, 3, _>(|c| support.functional_indices[group * 3 + c]);
        let v = std::array::from_fn::<_, 3, _>(|c| support.values[group * 3 + c]);
        let mut intervals = [[0.0; 2]; 3];
        for c in 0..3 {
            let descriptor = selected
                .retained()
                .descriptors()
                .get(indices[c])
                .ok_or_else(|| bad("support norm descriptor"))?;
            if descriptor.key.case_id != input.load_case.id
                || descriptor.key.quantity
                    != (FunctionalQuantity::SupportAction {
                        support: id.into(),
                        node: support.node_index,
                        component: (group * 3 + c) as u8,
                    })
                || descriptor.key.convention != FunctionalConvention::SupportOnStructure
                || descriptor.key.unit
                    != if group == 0 {
                        FunctionalUnit::Newton
                    } else {
                        FunctionalUnit::NewtonMetre
                    }
            {
                return Err(bad("support norm functional binding"));
            }
            let p = selected
                .retained()
                .projections()
                .iter()
                .find(|p| p.index() == indices[c])
                .ok_or_else(|| bad("support norm projection"))?;
            if !same(p.value(), v[c]) {
                return Err(bad("support norm projection value"));
            }
            intervals[c] = p.interval();
        }
        let result = norm(v, intervals)?;
        values[group] = result.value;
        checks.push(json!({"result_id":format!("result:support-action:{}:{}:{}:{}:{}",input.load_case.id.len(),input.load_case.id,id.len(),id,if group==0 {"force_magnitude"}else{"moment_magnitude"}),"recipe_id":if group==0 {"support_force_norm_scaled_checked_v1"}else{"support_moment_norm_scaled_checked_v1"},"support_id":id,"functional_indices":indices,"functional_ids":indices.map(|i|source::functional_id(&input.load_case.id,i)),"values":v,"intervals":intervals,"value":result.value,"interval":result.interval,"absolute_error_bound":result.absolute,"relative_error_bound":result.relative,"relative_limit":CRITERION}));
    }
    Ok((values, checks))
}
pub(in super::super) fn composite_support_norms(
    input: &source_recovery::Input<'_>,
    selected: &mut SelectedSourceRecovery,
    id: &str,
) -> Result<[f64; 2], ReceiptError> {
    #[cfg(test)]
    trace("derived_recipe_entry", selected);
    charge(selected, RECIPE_WORK)?;
    support_norm_checks(input, selected, id)
        .map(|r| r.0)
        .map_err(|mut e| {
            e.1 = Some(selected.summary().work);
            e
        })
}

fn normalized_case(
    capture: &CapturedInvocation,
    case_id: &str,
) -> Result<(PreviewModel, Vec<MaterialInput>, BuiltModel, Option<String>), ReceiptError> {
    let request = capture.requested()?;
    let mut model = request.model;
    let mut materials = if request.materials.is_empty() {
        model.materials.clone()
    } else {
        request.materials
    };
    let mut diagnostics = Vec::new();
    pressure_runtime::validate_profile(&model, &mut diagnostics);
    resolve_shared_sections(&mut model, &mut diagnostics);
    normalize_model_units(&mut model, &mut materials, &mut diagnostics);
    pressure_material::resolve_base(&model, &mut materials, &mut diagnostics);
    let case = model
        .load_cases
        .iter()
        .find(|c| c.id == case_id)
        .ok_or_else(|| bad("physical case capture"))?;
    let record = if modulus_basis_key(case, &mut diagnostics).is_some() {
        let (selected, record) =
            pressure_material::resolve_case(&model, &materials, case, &mut diagnostics)
                .ok_or_else(|| bad("physical common material selection"))?;
        materials = selected;
        Some(record)
    } else {
        None
    };
    let built = build_model(&model, &materials, &mut diagnostics)
        .ok_or_else(|| bad("physical source build"))?;
    if has_blocking(&diagnostics) || !pressure_runtime::is_exact(&model) {
        return Err(bad("physical exact profile unavailable"));
    }
    Ok((model, materials, built, record))
}
fn physical_source(
    capture: &CapturedInvocation,
    case_id: &str,
    evidence: &Value,
    pressure: &[Value],
    actual: &[ResultItem],
    source_selected: bool,
    load_record: Option<&Value>,
) -> Result<(), ReceiptError> {
    if case_state::is_load_state(&capture.requested()?.model) {
        // The resolver pipeline, never the case-wide material methods.
        let replay = capture.captured_load_state_case(case_id)?;
        return physical_source_built(
            capture,
            &replay.model,
            &replay.materials,
            &replay.built,
            None,
            case_id,
            evidence,
            pressure,
            actual,
            source_selected,
            Some((&replay.resolved, load_record)),
        );
    }
    if load_record.is_some() {
        return Err(bad("load/reference-state record on a pre-0.4 case"));
    }
    let (model, materials, built, record) = normalized_case(capture, case_id)?;
    physical_source_built(
        capture,
        &model,
        &materials,
        &built,
        record.as_deref(),
        case_id,
        evidence,
        pressure,
        actual,
        source_selected,
        None,
    )
}
#[allow(clippy::too_many_arguments)]
pub(super) fn physical_source_built(
    capture: &CapturedInvocation,
    model: &PreviewModel,
    materials: &[MaterialInput],
    built: &BuiltModel,
    record: Option<&str>,
    case_id: &str,
    evidence: &Value,
    pressure: &[Value],
    actual: &[ResultItem],
    source_selected: bool,
    load_state: Option<(&case_state::resolve::ResolvedCase, Option<&Value>)>,
) -> Result<(), ReceiptError> {
    if !pressure_runtime::is_exact(model) || built.exact_sections.len() != model.pipe_segments.len()
    {
        return Err(bad("physical source exact profile and geometry inventory"));
    }
    if case_state::is_load_state(model) != load_state.is_some() {
        return Err(bad("physical load/reference-state ownership"));
    }
    // A resolved case publishes its declared source ledger: the effective case.
    let case = match load_state {
        Some((state, _)) => Some(&state.effective_case).filter(|c| c.id == case_id),
        None => model.load_cases.iter().find(|c| c.id == case_id),
    }
    .ok_or_else(|| bad("physical case"))?;
    if let Some((state, record)) = load_state {
        let expected = load_state_case_record(state, capture.mode, source_selected);
        if record != Some(&expected) {
            return Err(bad("physical load/reference-state case record binding"));
        }
    }
    let expected_method = if source_selected {
        "retained_source_blocks_exact_v1"
    } else if capture.mode == PreviewSolverMode::DenseScrutiny {
        "ordinary_dense_structural_v1"
    } else {
        "ordinary_sparse_structural_v1"
    };
    let keys: BTreeSet<_> = evidence
        .as_object()
        .ok_or_else(|| bad("physical object"))?
        .keys()
        .map(String::as_str)
        .collect();
    if keys
        != BTreeSet::from([
            "load_case_id",
            "profile_mode",
            "material_basis",
            "pressure_rhs_assembly",
            "pipe_sections",
            "pipe_stress_extrema",
            "stress_maximum_coverage",
            "pipe_materials",
            "recovery_method",
        ])
        || evidence["load_case_id"] != case_id
        || evidence["profile_mode"] != "exact_straight_pressure_v2"
        || evidence["recovery_method"] != expected_method
        || evidence["material_basis"]
            != if load_state.is_some() {
                json!("resolved_per_member_load_reference_state_v1")
            } else {
                json!(record.unwrap_or("base_material_common_E_nu"))
            }
        || evidence["stress_maximum_coverage"] != json!({"complete":true,"unavailable_pipe_ids":[]})
    {
        return Err(bad("physical case/method/material/coverage binding"));
    }
    let sections: Vec<_> = built
        .pipes
        .iter()
        .map(|p| exact_section_evidence(&p.element_id, built.exact_sections[&p.element_id]))
        .collect();
    if evidence["pipe_sections"] != json!(sections) {
        return Err(bad("physical source geometry binding"));
    }
    let material_evidence: Value = if let Some((state, _)) = load_state {
        load_state_pipe_materials(state)
    } else {
        json!(model.pipe_segments.iter().map(|pipe| {
        let material=materials.iter().find(|m|m.id==pipe.material).expect("validated build material");
        let thermal=case.primitive_loads.iter().any(|l|l.category=="thermal" && is_temperature_change_dimension(&l.dimension) && matches!(&l.target,LoadTargetInput::Element{pipe:target} if target==&pipe.id));
        json!({"pipe_id":pipe.id,"material_id":material.id,"E_pa":material.elastic_modulus.value,"nu":material.poisson_ratio.as_ref().map(|q|q.value),"G_pa":material.shear_modulus.as_ref().map(|q|q.value),"constitutive_basis":material.constitutive_basis,"thermal_consumed":thermal,"alpha_per_kelvin":if thermal {material.thermal_expansion_coefficient.as_ref().map(|q|q.value)}else{None},"provenance":material.provenance})
    }).collect::<Vec<_>>())
    };
    if evidence["pipe_materials"] != material_evidence {
        return Err(bad("physical common E/nu material binding"));
    }
    let mut diagnostics = Vec::new();
    let mut pressure_case = match load_state {
        Some((state, _)) => pressure_runtime::build_pressure_case_with_members(
            model,
            built,
            materials,
            case,
            Some(&state.pairs),
            &mut diagnostics,
        ),
        None => {
            pressure_runtime::build_pressure_case(model, built, materials, case, &mut diagnostics)
        }
    }
    .ok_or_else(|| bad("physical pressure source"))?;
    if has_blocking(&diagnostics)
        || evidence["pressure_rhs_assembly"] != pressure_case.assembly_evidence
    {
        return Err(bad("physical pressure assembly binding"));
    }
    for region in &mut pressure_case.evidence {
        let region_id = region["region_id"].as_str().unwrap_or("");
        let members: BTreeSet<_> = pressure_case
            .pipe_states
            .iter()
            .filter(|(_, p)| p.region_id == region_id)
            .map(|(i, _)| built.pipes[*i].element_id.as_str())
            .collect();
        region["result_ids"] = json!(actual
            .iter()
            .filter(|r| members.contains(r.entity_ref.as_str())
                && r.kind.starts_with("pipe_")
                && r.kind.ends_with("_v2"))
            .map(|r| &r.id)
            .collect::<Vec<_>>());
    }
    if pressure != pressure_case.evidence {
        return Err(bad("physical pressure evidence binding"));
    }
    if source_selected
        && (!pressure.is_empty() || !case.pressure_regions.as_ref().is_some_and(Vec::is_empty))
    {
        return Err(bad("source physical pressure inventory"));
    }
    validate_physics_rows(&model, case, actual, evidence, source_selected)
}
fn validate_physics_rows(
    model: &PreviewModel,
    case: &PreviewLoadCase,
    actual: &[ResultItem],
    evidence: &Value,
    source_selected: bool,
) -> Result<(), ReceiptError> {
    validate_case_rows(&case.id, actual)?;
    // Ordinary physical admission deliberately uses the exact-physics table,
    // never the older precision/source ordinary allowlist.
    let table: Value = serde_json::from_str(include_str!(
        "../../../../fixtures/results/semantic_contract_v0_3_physics_1.json"
    ))
    .map_err(|e| bad(e.to_string()))?;
    let signatures = table["rows"]
        .as_array()
        .ok_or_else(|| bad("physics signatures"))?;
    for row in actual {
        let component = row.metadata.as_ref().map(|m| m.component.as_str());
        if !signatures.iter().any(|s| {
            s["kind"] == row.kind && s["unit"] == row.unit && s["component"].as_str() == component
        }) {
            return Err(bad(format!("unregistered physical row: {}", row.kind)));
        }
    }
    let extrema = evidence["pipe_stress_extrema"]
        .as_array()
        .ok_or_else(|| bad("physical maxima"))?;
    if extrema.len() != model.pipe_segments.len() {
        return Err(bad("physical member maximum coverage"));
    }
    let mut seen = BTreeSet::new();
    for pipe in &model.pipe_segments {
        let e = extrema
            .iter()
            .find(|e| e["pipe_id"] == pipe.id)
            .ok_or_else(|| bad("physical member maximum missing"))?;
        let id = e["result_id"]
            .as_str()
            .ok_or_else(|| bad("physical maximum result"))?;
        if !seen.insert(id) {
            return Err(bad("physical maximum duplicated"));
        }
        let row = actual
            .iter()
            .find(|r| r.id == id)
            .ok_or_else(|| bad("physical maximum row missing"))?;
        let metadata = row
            .metadata
            .as_ref()
            .ok_or_else(|| bad("physical maximum metadata"))?;
        if row.kind != "pipe_elastic_normal_stress_maximum_v2"
            || row.unit != "Pa"
            || row.entity_ref != pipe.id
            || metadata.component != "maximum_absolute_normal_stress"
            || metadata.coordinate_system != "pipe_section"
            || metadata.location != "governing_station"
            || metadata.basis
                != if source_selected {
                    MAX_BASIS
                } else {
                    "recovered_from_open_mechanics_stress_components"
                }
        {
            return Err(bad("physical maximum signature"));
        }
        let lo = e["value_lower_pa"]
            .as_f64()
            .ok_or_else(|| bad("maximum lower bound"))?;
        let hi = e["value_upper_pa"]
            .as_f64()
            .ok_or_else(|| bad("maximum upper bound"))?;
        if !lo.is_finite() || !hi.is_finite() || lo < 0.0 || lo > row.value || row.value > hi {
            return Err(bad("physical maximum range"));
        }
        if !source_selected
            && (e["coefficient_basis"] != "j_side_section_equilibrium_binary64"
                || e["approximation"] != "piecewise_quadratic_straight_section_statics")
        {
            return Err(bad("ordinary physical maximum method"));
        }
    }
    for node in &model.nodes {
        if actual
            .iter()
            .filter(|r| r.entity_ref == node.id && r.kind == "displacement_magnitude")
            .count()
            != 1
            || actual
                .iter()
                .filter(|r| {
                    r.entity_ref == node.id
                        && (r.kind.starts_with("global_nodal_displacement_")
                            || r.kind.starts_with("global_nodal_rotation_"))
                })
                .count()
                != 6
        {
            return Err(bad("physical nodal coverage"));
        }
    }
    for support in &model.supports {
        let mut expected = Vec::new();
        let values = std::array::from_fn(|i| {
            actual
                .iter()
                .find(|r| {
                    r.entity_ref == support.id
                        && r.kind == "support_reaction_component_v2"
                        && r.metadata
                            .as_ref()
                            .is_some_and(|m| m.component == ["Fx", "Fy", "Fz", "Mx", "My", "Mz"][i])
                })
                .map(|r| r.value)
                .unwrap_or(f64::NAN)
        });
        append_signed_support_results(&mut expected, case, support, values);
        for mut row in expected {
            let current = actual
                .iter()
                .find(|r| r.id == row.id)
                .ok_or_else(|| bad("physical support six-component/norm coverage"))?;
            if source_selected && row.kind != "support_reaction_component_v2" {
                row.value = current.value;
            }
            if !same(current.value, row.value) || serialized(current)? != serialized(&row)? {
                return Err(bad("physical support signature or value"));
            }
        }
    }
    Ok(())
}

impl FinalizedSourceBlockCase {
    pub(in super::super) fn composite_exact(
        capture: &CapturedInvocation,
        input: source_recovery::Input<'_>,
        mut selected: SelectedSourceRecovery,
        ordinary: OrdinaryAttempt,
        actual: &[ResultItem],
        bindings: &[FunctionalRowBinding],
        evidence: &Value,
        load_record: Option<&Value>,
    ) -> Result<Self, ReceiptError> {
        #[cfg(test)]
        trace("source_case_finalization_entry", &selected);
        let checked = (|| {
            if ordinary.mode != capture.mode
                || !matches!(ordinary.outcome, "sensitive" | "rejected")
            {
                return Err(bad("composite source selection ordinary attempt"));
            }
            let reservation = 1024usize
                .saturating_add(actual.len().saturating_mul(64))
                .saturating_add(finalization_size(capture, &selected, actual))
                .saturating_add(selected.support_actions().len().saturating_mul(RECIPE_WORK));
            charge(&mut selected, reservation)?;
            #[cfg(test)]
            trace("source_case_finalization_reserved", &selected);
            let material_record = capture.check_input_with_physical(
                &input,
                &mut selected,
                Some((evidence, actual, load_record)),
            )?;
            #[cfg(test)]
            trace("captured_source_replay_complete", &selected);
            selected
                .check_binding_against(source_recovery::Input {
                    model: input.model,
                    built: input.built,
                    stiffness: input.stiffness,
                    force: input.force,
                    free: input.free,
                    prescribed: input.prescribed,
                    spring_entries: input.spring_entries,
                    load_case: input.load_case,
                    load_application: input.load_application,
                    thermal_loads: input.thermal_loads,
                    pressure_thrust_loads: input.pressure_thrust_loads,
                    load_state: input.load_state,
                })
                .map_err(|e| bad(format!("composite current source: {e:?}")))?;
            #[cfg(test)]
            trace("current_source_binding_complete", &selected);
            let primary = rows::expected_primary_for(&input, &selected, true)?;
            let mut additional = Vec::new();
            let mut derived_checks = Vec::new();
            let mut section_stress_checks = Vec::new();
            let member_ids: Vec<_> = selected
                .members()
                .iter()
                .map(|m| m.member_id.clone())
                .collect();
            for id in member_ids {
                let maximum = composite_member_maximum(&input, &mut selected, &id)?;
                let result_id = format!(
                    "result:elastic-maximum:{}:{}:{}:{}",
                    input.load_case.id.len(),
                    input.load_case.id,
                    id.len(),
                    id
                );
                let expected = maximum.evidence(&result_id);
                let found = evidence["pipe_stress_extrema"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .find(|e| e["pipe_id"] == id)
                    .ok_or_else(|| bad("source maximum evidence"))?;
                if *found != expected {
                    return Err(bad("source endpoint maximum evidence binding"));
                }
                additional.push(rows::Derived { row:ResultItem{id:result_id,kind:"pipe_elastic_normal_stress_maximum_v2".into(),value:maximum.value_pa(),unit:"Pa".into(),entity_ref:id.clone(),basis_ref:Some(ResultBasisRef{ref_type:"load_case".into(),ref_id:input.load_case.id.clone()}),source_result_refs:vec![],metadata:Some(ResultMetadata{component:"maximum_absolute_normal_stress".into(),coordinate_system:"pipe_section".into(),location:"governing_station".into(),basis:MAX_BASIS.into(),sign_convention:"nonnegative maximum absolute axial-plus-bending normal stress over an unloaded circular straight span; retained endpoint actions with projected-action and arithmetic bounds; endpoint witness does not imply uniqueness; torsional shear separate".into()})},recipe:MAX_BASIS,inputs:vec![] });
            }
            for support in selected.support_actions() {
                let (values, checks) = support_norm_checks(&input, &selected, &support.support_id)?;
                let authored = input
                    .model
                    .supports
                    .iter()
                    .find(|s| s.id == support.support_id)
                    .ok_or_else(|| bad("support owner"))?;
                let mut generated = Vec::new();
                append_signed_support_results(
                    &mut generated,
                    input.load_case,
                    authored,
                    support.values,
                );
                for (group, mut row) in generated.into_iter().skip(6).enumerate() {
                    row.value = values[group];
                    additional.push(rows::Derived {
                        row,
                        recipe: if group == 0 {
                            "support_force_norm_scaled_checked_v1"
                        } else {
                            "support_moment_norm_scaled_checked_v1"
                        },
                        inputs: (0..3)
                            .map(|c| {
                                primary[&support.functional_indices[group * 3 + c]]
                                    .id
                                    .clone()
                            })
                            .collect(),
                    });
                }
                derived_checks.extend(checks);
            }
            for member in selected.members() {
                let section = input
                    .built
                    .sections
                    .get(&member.member_id)
                    .ok_or_else(|| bad("section stress geometry"))?;
                for row in actual.iter().filter(|r| r.entity_ref == member.member_id) {
                    let component = match row.kind.as_str() {
                        "element_local_axial_normal_stress" => 0usize,
                        "element_local_bending_normal_stress_y" => 4,
                        "element_local_bending_normal_stress_z" => 5,
                        "element_local_torsional_shear_stress" => 3,
                        _ => continue,
                    };
                    let metadata = row
                        .metadata
                        .as_ref()
                        .ok_or_else(|| bad("section stress metadata"))?;
                    let station = match metadata.location.as_str() {
                        "end_i" => 0,
                        "quarter_1" => 1,
                        "midspan" => 2,
                        "quarter_3" => 3,
                        "end_j" => 4,
                        _ => return Err(bad("section stress location")),
                    };
                    let index = member.section_functional_indices[station][component];
                    let descriptor = selected
                        .retained()
                        .descriptors()
                        .get(index)
                        .ok_or_else(|| bad("section stress descriptor"))?;
                    if descriptor.key.quantity
                        != (FunctionalQuantity::MemberSection {
                            member: member.member_id.clone(),
                            station_bits: source_recovery::STATIONS[station].to_bits(),
                            component: component as u8,
                        })
                        || descriptor.key.convention != FunctionalConvention::SectionLocal
                        || descriptor.key.case_id != input.load_case.id
                    {
                        return Err(bad("section stress functional binding"));
                    }
                    let projection = selected
                        .retained()
                        .projections()
                        .iter()
                        .find(|p| p.index() == index)
                        .ok_or_else(|| bad("section stress projection"))?;
                    if !same(projection.value(), member.sections[station][component]) {
                        return Err(bad("section stress action binding"));
                    }
                    section_stress_checks.push(json!({"result_id":row.id,"recipe_id":"retained_source_straight_stress_v1","pipe_id":member.member_id,"location":metadata.location,"component":metadata.component,"functional_index":index,"functional_id":source::functional_id(&input.load_case.id,index),"action":{"value":projection.value(),"interval":projection.interval()},"parameters":{"area_m2":section.area,"section_modulus_m3":section.section_modulus,"torsion_radius_m":section.torsion_radius,"torsion_constant_m4":section.torsion_constant,"pa_per_mpa":1e6}}));
                }
            }
            let ledger = rows::bind_for(&input, &selected, actual, bindings, true, additional)?;
            if ledger.rows.iter().any(|t| {
                t.treatment == "inspection_only"
                    && !actual
                        .iter()
                        .find(|r| r.id == t.result_id)
                        .is_some_and(rows::observation)
            }) {
                return Err(bad("unqualified composite physical row"));
            }
            let mut source = source::commitment(
                &input,
                &selected,
                bindings,
                &capture.digest,
                material_record.as_deref(),
                &ledger.rows,
                actual,
            )?;
            source["retained_identity_sha256"] = json!(selected
                .retained_identity_digest()
                .map_err(|e| bad(format!("source digest finalization: {e:?}")))?);
            source["endpoint_sections"]=json!(evidence["pipe_stress_extrema"].as_array().unwrap().iter().map(|e|json!({"pipe_id":e["pipe_id"],"endpoints":e["endpoints"].as_array().unwrap().iter().map(|p|json!({"station_fraction":p["station_fraction"],"functional_indices":p["functional_indices"],"functional_ids":p["functional_ids"],"actions":p["actions"]})).collect::<Vec<_>>()})).collect::<Vec<_>>());
            source["section_functionals"]=json!(selected.members().iter().map(|m|json!({"pipe_id":m.member_id,"stations":source_recovery::STATIONS.iter().enumerate().map(|(s,f)|json!({"station_fraction":f,"functional_indices":m.section_functional_indices[s],"functional_ids":m.section_functional_indices[s].map(|i|source::functional_id(&input.load_case.id,i)),"actions":m.section_functional_indices[s].iter().map(|index|{let p=selected.retained().projections().iter().find(|p|p.index()==*index).expect("owned retained projection");json!({"value":p.value(),"interval":p.interval()})}).collect::<Vec<_>>()})).collect::<Vec<_>>()})).collect::<Vec<_>>());
            #[cfg(test)]
            trace("source_case_finalization_complete", &selected);
            Ok((ledger, source, derived_checks, section_stress_checks))
        })();
        let (ledger, source, derived_checks, section_stress_checks) =
            checked.map_err(|mut e| {
                e.1 = Some(selected.summary().work);
                e
            })?;
        Ok(Self {
            invocation: capture.digest.clone(),
            case_id: input.load_case.id.clone(),
            ordinary,
            source: Some(source),
            projections: ledger.projections,
            rows: ledger.rows,
            supports: ledger.supports,
            failure: None,
            work: work_wire(selected.summary().work)?,
            actual_rows: serialized(&actual)?,
            qualified: true,
            exact: true,
            physical_evidence: Some(case_physical_evidence(evidence, &[], load_record)),
            derived_checks,
            section_stress_checks,
            _selected: Some(selected),
        })
    }
    pub(in super::super) fn ordinary_physics(
        capture: &CapturedInvocation,
        case_id: &str,
        ordinary: OrdinaryAttempt,
        actual: &[ResultItem],
        evidence: &Value,
        pressure: &[Value],
        load_record: Option<&Value>,
    ) -> Result<Self, ReceiptError> {
        if ordinary.mode != capture.mode || ordinary.outcome != "checks_passed" {
            return Err(bad(
                "ordinary physical selection requires actual ChecksPassed",
            ));
        }
        validate_case_rows(case_id, actual)?;
        // The complete captured physical source comparison is made under the
        // invocation publication reservation, before this case can be receipted.
        Ok(Self {
            invocation: capture.digest.clone(),
            case_id: case_id.into(),
            ordinary,
            source: None,
            projections: vec![],
            rows: actual
                .iter()
                .map(|r| RowTreatment {
                    result_id: r.id.clone(),
                    treatment: "ordinary_physics_checked",
                    projection_id: None,
                    recipe_id: None,
                    input_result_ids: vec![],
                })
                .collect(),
            supports: json!([]),
            failure: None,
            work: work_wire(exact::WorkReport {
                charged: 0,
                rejected: 0,
                limit: 0,
            })?,
            actual_rows: serialized(&actual)?,
            qualified: true,
            exact: false,
            physical_evidence: Some(case_physical_evidence(evidence, pressure, load_record)),
            derived_checks: vec![],
            section_stress_checks: vec![],
            _selected: None,
        })
    }
}

pub(super) fn validate_publication(
    capture: &CapturedInvocation,
    envelope: &MechanicsEnvelope,
    cases: &[FinalizedSourceBlockCase],
) -> Result<(), ReceiptError> {
    if !cases.iter().any(|c| c.exact) || cases.iter().any(|c| !c.qualified) {
        return Err(bad(
            "composite requires source selection and complete case qualification",
        ));
    }
    let evidence = envelope
        .contract_evidence
        .as_ref()
        .ok_or_else(|| bad("composite physical evidence missing"))?;
    let load_state = case_state::is_load_state(&capture.requested()?.model);
    let mut namespace = BTreeSet::from(["pressure", "connector", "exact_cases"]);
    if load_state {
        namespace.insert("load_reference_states");
    }
    if evidence
        .as_object()
        .map(|o| o.keys().map(String::as_str).collect::<BTreeSet<_>>())
        != Some(namespace)
        || evidence["connector"] != json!([])
    {
        return Err(bad("composite physical namespace"));
    }
    let load_records = if load_state {
        let records = evidence["load_reference_states"]
            .as_array()
            .ok_or_else(|| bad("composite load/reference-state cases"))?;
        if records.len() != cases.len() {
            return Err(bad("composite load/reference-state case coverage"));
        }
        Some(records)
    } else {
        None
    };
    let physical = evidence["exact_cases"]
        .as_array()
        .ok_or_else(|| bad("composite exact cases"))?;
    let pressure = evidence["pressure"]
        .as_array()
        .ok_or_else(|| bad("composite pressure cases"))?;
    if physical.len() != cases.len() {
        return Err(bad("composite exact case coverage"));
    }
    let mut observed_pressure = Vec::new();
    for (index, (case, physical)) in cases.iter().zip(physical).enumerate() {
        let expected = case
            .physical_evidence
            .as_ref()
            .ok_or_else(|| bad("case physical proof absent"))?;
        if expected["exact_case"] != *physical {
            return Err(bad("case physical evidence changed"));
        }
        let load_record = expected.get("load_reference_state");
        if load_record != load_records.map(|records| &records[index]) {
            return Err(bad("case load/reference-state record changed"));
        }
        let pressure = expected["pressure"]
            .as_array()
            .ok_or_else(|| bad("case pressure proof"))?;
        observed_pressure.extend(pressure.iter().cloned());
        let actual: Vec<_> = envelope
            .results
            .iter()
            .filter(|r| {
                r.basis_ref
                    .as_ref()
                    .is_some_and(|b| b.ref_type == "load_case" && b.ref_id == case.case_id)
            })
            .cloned()
            .collect();
        if !case.exact {
            physical_source(
                capture,
                &case.case_id,
                physical,
                pressure,
                &actual,
                false,
                load_record,
            )?;
        }
    }
    if observed_pressure != *pressure {
        return Err(bad("whole pressure case order/coverage"));
    }
    validate_composite_summary(envelope)
}
/// Per-case physical proof hashed into the receipt; a 0.4.0 case also binds
/// its published load/reference-state record.
fn case_physical_evidence(
    evidence: &Value,
    pressure: &[Value],
    load_record: Option<&Value>,
) -> Value {
    let mut proof = json!({"exact_case":evidence,"pressure":pressure});
    if let Some(record) = load_record {
        proof["load_reference_state"] = record.clone();
    }
    proof
}
fn validate_composite_summary(envelope: &MechanicsEnvelope) -> Result<(), ReceiptError> {
    for (headline, kind) in [
        (&envelope.summary.max_displacement, "displacement_magnitude"),
        (
            &envelope.summary.max_open_formula_stress,
            "pipe_elastic_normal_stress_maximum_v2",
        ),
    ] {
        let h = headline
            .as_ref()
            .ok_or_else(|| bad("composite complete headline required"))?;
        let row = envelope
            .results
            .iter()
            .find(|r| r.id == h.result_ref)
            .ok_or_else(|| bad("composite headline reference"))?;
        if row.kind != kind
            || row.entity_ref != h.location_ref
            || row.unit != h.unit
            || !same(row.value, h.value)
        {
            return Err(bad("composite headline binding"));
        }
        if envelope
            .results
            .iter()
            .any(|r| r.kind == kind && r.value > h.value)
        {
            return Err(bad("composite headline maximum"));
        }
    }
    Ok(())
}

// Append within composite.rs module at handoff boundary.
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn checked_support_norm_composes_interval_and_arithmetic_error() {
        for values in [
            [0.0, 0.0, 0.0],
            [3.0, 4.0, 0.0],
            [-3.0, 0.0, -4.0],
            [1e300, 1e-300, 0.0],
            [1e-200, 0.0, 0.0],
        ] {
            let result = norm(values, values.map(|v| [v, v])).unwrap();
            assert!(result.interval[0] <= result.value && result.value <= result.interval[1]);
            assert!(result.relative <= 1e-9);
            if values == [3.0, 4.0, 0.0] {
                assert_eq!(result.value, 5.0);
                assert!(result.interval[0] <= 5.0 && result.interval[1] >= 5.0);
            }
        }
        assert!(norm([3.0, 4.0, 0.0], [[2.9, 3.1], [4.0, 4.0], [0.0, 0.0]]).is_err());
        assert!(norm(
            [f64::from_bits(1), 0.0, 0.0],
            [[f64::from_bits(1); 2], [0.0; 2], [0.0; 2]]
        )
        .is_err());
        assert!(norm(
            [1.7e308, 1.7e308, 0.0],
            [[1.7e308; 2], [1.7e308; 2], [0.0; 2]]
        )
        .is_err());
    }
}

#[cfg(test)]
thread_local! {
    static WORK_TRACE: std::cell::RefCell<Option<Vec<Value>>> = const {std::cell::RefCell::new(None)};
}
#[cfg(test)]
pub(super) fn start_trace() {
    WORK_TRACE.with(|trace| *trace.borrow_mut() = Some(vec![]));
}
#[cfg(test)]
pub(super) fn take_trace() -> Vec<Value> {
    WORK_TRACE.with(|trace| trace.borrow_mut().take().unwrap_or_default())
}
#[cfg(test)]
fn trace(stage: &str, selected: &SelectedSourceRecovery) {
    WORK_TRACE.with(|trace|if let Some(values)=&mut *trace.borrow_mut(){values.push(json!({"stage":stage,"charged":selected.summary().work.charged,"rejected":selected.summary().work.rejected}));});
}
