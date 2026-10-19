use super::*;
use exact::functionals::{
    AttemptStage, FunctionalDescriptor, FunctionalQuantity, FunctionalUnit, MemberEnd,
};
fn matrix(m: &[Vec<f64>]) -> Value {
    json!(m
        .iter()
        .map(|r| r.iter().copied().map(bits).collect::<Vec<_>>())
        .collect::<Vec<_>>())
}
fn matrix12(m: &Matrix12) -> Value {
    json!(m
        .iter()
        .map(|r| r.iter().copied().map(bits).collect::<Vec<_>>())
        .collect::<Vec<_>>())
}
fn products(p: &[Vec<f64>]) -> Value {
    json!(p
        .iter()
        .map(|r| r.iter().copied().map(bits).collect::<Vec<_>>())
        .collect::<Vec<_>>())
}
fn owner(kind: &str, id: &str) -> Value {
    json!({"kind":kind,"id":id})
}
pub(super) fn functional_id(case: &str, index: usize) -> String {
    format!("source-functional:{}:{case}:{index}", case.len())
}
fn unit(u: FunctionalUnit) -> Result<&'static str, ReceiptError> {
    Ok(match u {
        FunctionalUnit::Newton => "N",
        FunctionalUnit::NewtonMetre => "N*m",
        FunctionalUnit::Metre => "m",
        FunctionalUnit::Millimetre => "mm",
        FunctionalUnit::Radian => "rad",
        _ => return Err(bad("unallocated functional unit")),
    })
}
fn source_operand(payload: &Value, path: String, expected: f64) -> Result<Value, ReceiptError> {
    let expected = bits(expected);
    if payload.pointer(&path).and_then(Value::as_str) != Some(expected.as_str()) {
        return Err(bad("plan operand does not resolve to same captured bits"));
    }
    Ok(json!({"kind":"source","source_path":path,"bits":expected}))
}
/// O4 (DEL-04-08 Design `o4-source-extension.md` §3.2): a member's reduced
/// stress section where it differs from its nominal (built) one in any of A,
/// Z, J or r_o, the only operands a source stress row or the endpoint maximum
/// reads; `None` where the two are bit-equal or the document has one basis.
fn distinct_reduced_section(
    input: &source_recovery::Input<'_>,
    member_id: &str,
) -> Result<Option<DerivedSection>, ReceiptError> {
    let Some(bases) = input.built.exact_sections.get(member_id).copied() else {
        return Ok(None);
    };
    let section = input
        .built
        .sections
        .get(member_id)
        .ok_or_else(|| bad("section source"))?;
    let reduced = exact_basis_section(bases, pressure_exact::SectionBasis::Reduced);
    Ok((!same_stress_section(&reduced, section)).then_some(reduced))
}
/// The payload operands a derived stress or maximum row reads: the nominal
/// (built) section's, or, for a reduced row whose member's sections differ,
/// the private `reduced_recovery_section`'s (O4 §3.2).
fn stress_operands(
    input: &source_recovery::Input<'_>,
    row: &ResultItem,
    i: usize,
) -> Result<(DerivedSection, [String; 4]), ReceiptError> {
    let reduced_row = row
        .metadata
        .as_ref()
        .is_some_and(|m| m.basis.ends_with(pressure_exact::SectionBasis::Reduced.basis_suffix()));
    if reduced_row {
        if let Some(reduced) = distinct_reduced_section(input, &row.entity_ref)? {
            let path = |key: &str| format!("/frames/{i}/reduced_recovery_section/{key}");
            return Ok((
                reduced,
                [path("area_bits"), path("section_modulus_bits"), path("torsion_radius_bits"), path("torsion_constant_bits")],
            ));
        }
    }
    let section = input
        .built
        .sections
        .get(&row.entity_ref)
        .ok_or_else(|| bad("recipe section"))?
        .clone();
    Ok((
        section,
        [
            format!("/frames/{i}/formation_inputs/area_bits"),
            format!("/frames/{i}/recovery_section/section_modulus_bits"),
            format!("/frames/{i}/recovery_section/torsion_radius_bits"),
            format!("/frames/{i}/formation_inputs/torsion_constant_bits"),
        ],
    ))
}
fn lowered_products(payload: &Value, path: &str, p: &[Vec<f64>]) -> Result<Value, ReceiptError> {
    let mut out = Vec::new();
    for (i, product) in p.iter().enumerate() {
        let mut factors = Vec::new();
        for (j, v) in product.iter().enumerate() {
            factors.push(source_operand(payload, format!("{path}/{i}/{j}"), *v)?);
        }
        out.push(json!({"factors":factors}));
    }
    Ok(json!(out))
}
fn function(
    payload: &Value,
    input: &source_recovery::Input<'_>,
    d: &FunctionalDescriptor,
    index: usize,
    bindings: &[FunctionalRowBinding],
) -> Result<Value, ReceiptError> {
    const ACTIONS: [&str; 6] = [
        "axial_force",
        "shear_force_y",
        "shear_force_z",
        "torsional_moment",
        "bending_moment_y",
        "bending_moment_z",
    ];
    let (owned, quantity, component, frame, location) = match &d.key.quantity {
        FunctionalQuantity::NodeDisplacement { node, dof } => (
            owner("node", node),
            if dof % 6 < 3 {
                "nodal_translation"
            } else {
                "nodal_rotation"
            },
            ["UX", "UY", "UZ", "RX", "RY", "RZ"][dof % 6].to_string(),
            "global",
            "node".to_string(),
        ),
        FunctionalQuantity::MemberEnd { member, end, row } => (
            owner("frame", member),
            "member_end_action",
            ACTIONS[*row as usize].to_string(),
            "element_local",
            if *end == MemberEnd::I {
                "end_i"
            } else {
                "end_j"
            }
            .to_string(),
        ),
        FunctionalQuantity::MemberSection {
            member,
            station_bits,
            component,
        } => {
            let station = f64::from_bits(*station_bits);
            let location = match station {
                0. => "end_i",
                0.25 => "quarter_1",
                0.5 => "midspan",
                0.75 => "quarter_3",
                1. => "end_j",
                _ => return Err(bad("plan station")),
            };
            (
                owner("frame", member),
                "member_station_action",
                ACTIONS[*component as usize].to_string(),
                "element_local",
                location.to_string(),
            )
        }
        FunctionalQuantity::GroundSpring { support, dof } => (
            owner("spring", support),
            "support_action_component",
            ["Fx", "Fy", "Fz", "Mx", "My", "Mz"][dof % 6].to_string(),
            "global",
            "node".into(),
        ),
        FunctionalQuantity::SupportAction {
            support, component, ..
        } => (
            owner("support", support),
            "support_action_component",
            ["Fx", "Fy", "Fz", "Mx", "My", "Mz"][*component as usize].to_string(),
            "global",
            "node".into(),
        ),
        FunctionalQuantity::SourceReaction { dof } => (
            owner("node", &input.model.nodes[dof / 6].id),
            "constraint_reaction",
            ["Fx", "Fy", "Fz", "Mx", "My", "Mz"][dof % 6].to_string(),
            "global",
            "node".into(),
        ),
        FunctionalQuantity::DeclaredAffine { .. } => {
            return Err(bad("untyped product plan function"))
        }
    };
    let mut terms = Vec::new();
    for (i, t) in d.terms.iter().enumerate() {
        terms.push(json!({"dof":t.dof,"coefficient_terms":lowered_products(payload,&format!("/lowered_recovery_operands/{index}/terms/{i}/product_bits"),&t.products)?}));
    }
    Ok(
        json!({"functional_id":functional_id(&d.key.case_id,index),"result_ids":bindings.iter().filter(|b|b.functional_index==index).map(|b|&b.result_id).collect::<Vec<_>>(),"owner":owned,"quantity":quantity,"component":component,"coordinate_system":frame,"location":location,"unit":unit(d.key.unit)?,"sign_convention":match d.key.convention{exact::functionals::FunctionalConvention::NodeOnElement=>"node_on_element_local_end",exact::functionals::FunctionalConvention::SectionLocal=>"j_side_section_local",exact::functionals::FunctionalConvention::SpringOnStructure=>"spring_on_structure",exact::functionals::FunctionalConvention::SupportOnStructure=>"support_on_pipe_positive_global_force_right_hand_couple_at_attachment_node",exact::functionals::FunctionalConvention::SourceEquilibrium=>"source_equilibrium",exact::functionals::FunctionalConvention::DeclaredAffine=>"declared_global_nodal_component"},"displacement_terms":terms,"offset_terms":lowered_products(payload,&format!("/lowered_recovery_operands/{index}/offset_bits"),&d.offset)?,"final_unit_factor":{"kind":"method_constant","bits":bits(1.)},"projection_relative_limit_bits":bits(CRITERION)}),
    )
}
fn recipes(
    payload: &Value,
    input: &source_recovery::Input<'_>,
    selected: &SelectedSourceRecovery,
    ledger: &[RowTreatment],
    actual: &[ResultItem],
) -> Result<Value, ReceiptError> {
    let mut recipes = Vec::new();
    for treatment in ledger.iter().filter(|r| r.treatment == "checked_derived") {
        let row = actual
            .iter()
            .find(|r| r.id == treatment.result_id)
            .ok_or_else(|| bad("recipe row missing"))?;
        let recipe = treatment
            .recipe_id
            .ok_or_else(|| bad("recipe registration missing"))?;
        let mut parameters = Vec::new();
        match recipe{
            "translation_norm_scaled_v1"|"support_force_norm_scaled_checked_v1"|"support_moment_norm_scaled_checked_v1"=>{},
            "retained_source_endpoint_normal_max_v1"=>{
                let i=input.built.pipes.iter().position(|p|p.element_id==row.entity_ref).ok_or_else(||bad("endpoint recipe member"))?;
                let (section,[area,modulus,_,_])=stress_operands(input,row,i)?;
                parameters.push(json!({"name":"area","operand":source_operand(payload,area,section.area)?}));
                parameters.push(json!({"name":"section_modulus","operand":source_operand(payload,modulus,section.section_modulus)?}));
            },
            "retained_source_straight_stress_v1"=>{
                let i=input.built.pipes.iter().position(|p|p.element_id==row.entity_ref).ok_or_else(||bad("recipe member"))?;
                let (section,[area,modulus,radius,torsion])=stress_operands(input,row,i)?;
                let operands:Vec<(&str,String,f64)>=match row.kind.as_str(){"element_local_axial_normal_stress"=>vec![("area",area,section.area)],"element_local_bending_normal_stress_y"|"element_local_bending_normal_stress_z"=>vec![("section_modulus",modulus,section.section_modulus)],"element_local_torsional_shear_stress"=>vec![("torsion_radius",radius,section.torsion_radius),("torsion_constant",torsion,section.torsion_constant)],_=>return Err(bad("recipe signature"))};
                for (name,path,v) in operands{parameters.push(json!({"name":name,"operand":source_operand(payload,path,v)?}));}
                parameters.push(json!({"name":"pa_per_mpa","operand":{"kind":"method_constant","bits":bits(1_000_000.)}}));
            },
            _=>return Err(bad("unimplemented recipe")),
        }
        let mut record = json!({"result_id":row.id,"recipe_id":recipe,"input_result_ids":treatment.input_result_ids,"parameters":parameters});
        let mut indices = Vec::new();
        if recipe == "retained_source_straight_stress_v1"
            || recipe == "retained_source_endpoint_normal_max_v1"
        {
            let member = selected
                .members()
                .iter()
                .find(|m| m.member_id == row.entity_ref)
                .ok_or_else(|| bad("source recipe member"))?;
            if recipe == "retained_source_endpoint_normal_max_v1" {
                for station in [0usize, 4] {
                    indices.extend(
                        [0usize, 4, 5].map(|c| member.section_functional_indices[station][c]),
                    );
                }
            } else {
                let metadata = row
                    .metadata
                    .as_ref()
                    .ok_or_else(|| bad("source stress metadata"))?;
                let station = match metadata.location.as_str() {
                    "end_i" => 0,
                    "quarter_1" => 1,
                    "midspan" => 2,
                    "quarter_3" => 3,
                    "end_j" => 4,
                    _ => return Err(bad("source stress station")),
                };
                let component = match row.kind.as_str() {
                    "element_local_axial_normal_stress" => 0,
                    "element_local_bending_normal_stress_y" => 4,
                    "element_local_bending_normal_stress_z" => 5,
                    "element_local_torsional_shear_stress" => 3,
                    _ => return Err(bad("source stress component")),
                };
                indices.push(member.section_functional_indices[station][component]);
            }
        } else if matches!(
            recipe,
            "support_force_norm_scaled_checked_v1" | "support_moment_norm_scaled_checked_v1"
        ) {
            let support = selected
                .support_actions()
                .iter()
                .find(|s| s.support_id == row.entity_ref)
                .ok_or_else(|| bad("source norm support"))?;
            let first = if recipe == "support_force_norm_scaled_checked_v1" {
                0
            } else {
                3
            };
            indices.extend((first..first + 3).map(|i| support.functional_indices[i]));
        }
        if !indices.is_empty() {
            record["input_functional_ids"] = json!(indices
                .into_iter()
                .map(|i| functional_id(&input.load_case.id, i))
                .collect::<Vec<_>>());
        }
        recipes.push(record);
    }
    Ok(json!(recipes))
}
pub(super) fn commitment(
    input: &source_recovery::Input<'_>,
    selected: &SelectedSourceRecovery,
    bindings: &[FunctionalRowBinding],
    invocation: &str,
    material_basis_record: Option<&str>,
    ledger: &[RowTreatment],
    actual: &[ResultItem],
) -> Result<Value, ReceiptError> {
    let retained = selected.retained();
    let response = retained.response();
    let system = response.source_system();
    let terms = system
        .contributions
        .ok_or_else(|| bad("missing complete stiffness source"))?;
    let exact::ForceBasis::IdentifiedContributions(forces) = response.force_basis() else {
        return Err(bad("identified forces required"));
    };
    if terms.len() != input.built.frame_elements.len() * 144 + input.spring_entries.len() {
        return Err(bad("stiffness ownership inventory"));
    }
    let mut stiffness = Vec::new();
    let mut cursor = 0;
    let mut frames = Vec::new();
    // RF design §3.3 and §7: the retained source's congruence members, in member
    // order; each is named by its frame's element id.
    let congruence = response.congruence_members();
    let mut congruence_ids = Vec::new();
    for (frame, pipe) in input.built.frame_elements.iter().zip(&input.built.pipes) {
        let map = element_dof_map(frame.node_i.index, frame.node_j.index);
        let global = frame.global_stiffness().map_err(|e| bad(e.to_string()))?;
        let local = frame.local_stiffness().map_err(|e| bad(e.to_string()))?;
        let transform = frame
            .orientation()
            .map_err(|e| bad(e.to_string()))?
            .transformation_matrix();
        if let Some(member) = congruence
            .get(congruence_ids.len())
            .filter(|m| m.contributions.start == cursor)
        {
            let operands = |a: &Matrix12, b: &Matrix12| {
                a.iter().flatten().zip(b.iter().flatten()).all(|(x, y)| same(*x, *y))
            };
            if member.dofs != map
                || !operands(&member.local_stiffness, &local)
                || !operands(&member.transform, &transform)
            {
                return Err(bad("congruence member owner mismatch"));
            }
            congruence_ids.push(&pipe.element_id);
        }
        for i in 0..12 {
            for j in 0..12 {
                let term = &terms[cursor];
                if term.row != map[i] || term.col != map[j] || !same(term.value, global[i][j]) {
                    return Err(bad("member stiffness owner mismatch"));
                }
                stiffness.push(json!({"owner":owner("frame",&pipe.element_id),"row":term.row,"col":term.col,"value_bits":bits(term.value)}));
                cursor += 1;
            }
        }
        let section = input
            .built
            .sections
            .get(&pipe.element_id)
            .ok_or_else(|| bad("section source"))?;
        let mut frame_payload = json!({"element_id":pipe.element_id,"node_i_id":input.model.nodes[frame.node_i.index].id,"node_j_id":input.model.nodes[frame.node_j.index].id,"scatter":map,"formation_inputs":{"coordinates_bits":[frame.node_i.coordinates.map(bits),frame.node_j.coordinates.map(bits)],"young_modulus_bits":bits(frame.section.elastic_modulus),"shear_modulus_bits":bits(frame.section.shear_modulus),"area_bits":bits(frame.section.area),"iy_bits":bits(frame.section.second_moment_y),"iz_bits":bits(frame.section.second_moment_z),"torsion_constant_bits":bits(frame.section.torsion_constant),"length_bits":bits(frame.length().map_err(|e|bad(e.to_string()))?)},"local_matrix_bits":matrix12(&local),"transform_bits":matrix12(&transform),"global_matrix_bits":matrix12(&global),"recovery_section":{"section_modulus_bits":bits(section.section_modulus),"torsion_radius_bits":bits(section.torsion_radius)}});
        // O4 (§3.1, §3.2): the private payload names a Timoshenko frame's
        // shear-area leaf, and a member's reduced stress section where it
        // differs from the nominal one. Both are bound only through
        // `normalized_source_sha256`; an Euler-Bernoulli frame with bit-equal
        // sections keeps its bytes.
        if let Some(shear_area) = frame.section.shear_area() {
            frame_payload["formation_inputs"]["shear_area_bits"] = json!(bits(shear_area));
        }
        if let Some(reduced) = distinct_reduced_section(input, &pipe.element_id)? {
            frame_payload["reduced_recovery_section"] = json!({"area_bits":bits(reduced.area),"section_modulus_bits":bits(reduced.section_modulus),"torsion_constant_bits":bits(reduced.torsion_constant),"torsion_radius_bits":bits(reduced.torsion_radius)});
        }
        frames.push(frame_payload);
    }
    if congruence_ids.len() != congruence.len() {
        return Err(bad("congruence member owner"));
    }
    let mut springs = Vec::new();
    for s in input.spring_entries {
        let t = &terms[cursor];
        let dof = s.node_dof.global_index();
        if t.row != dof || t.col != dof || !same(t.value, s.stiffness.value) {
            return Err(bad("spring source owner mismatch"));
        }
        stiffness.push(json!({"owner":owner("spring",&s.support_id),"row":dof,"col":dof,"value_bits":bits(t.value)}));
        springs.push(json!({"spring_id":s.support_id,"support_id":s.support_id,"node_id":input.model.nodes[s.node_dof.node_index].id,"dof":dof,"stiffness_bits":bits(s.stiffness.value),"ground_movement_bits":bits(0.0)}));
        cursor += 1;
    }
    let mut prescribed = Vec::new();
    for (dof, value) in system.prescribed {
        let owners: Vec<_> = input
            .built
            .supports
            .iter()
            .filter(|s| {
                s.family != SupportFamily::Spring
                    && s.node_index == dof / 6
                    && s.restrained_dofs.iter().any(|d| dof_index(*d) == dof % 6)
            })
            .collect();
        if owners.len() != 1 {
            return Err(bad("prescribed support attribution"));
        }
        prescribed
            .push(json!({"dof":dof,"value_bits":bits(*value),"support_id":owners[0].support_id}));
    }
    // A resolved case owns one eigen axial load per member with nonzero
    // eigenstrain; prepare_sources refused any primitive ID collision.
    let eigen_sources: BTreeMap<String, &str> = if input.load_state.is_some() {
        input
            .thermal_loads
            .iter()
            .map(|l| {
                let pipe = &input.built.pipes[l.element_index].element_id;
                (source_recovery::eigen_source_id(pipe), pipe.as_str())
            })
            .collect()
    } else {
        BTreeMap::new()
    };
    let mut supported_loads = Vec::new();
    for f in forces {
        if eigen_sources.contains_key(&f.source) {
            continue;
        }
        let primitive = input
            .load_case
            .primitive_loads
            .iter()
            .position(|p| p.id == f.source)
            .ok_or_else(|| bad("force primitive owner"))?;
        supported_loads.push(json!({"load_id":f.source,"node_id":input.model.nodes[f.dof/6].id,"dof":f.dof,"value_bits":bits(f.value),"source_primitive_index":primitive}));
    }
    let lowered:Vec<_>=retained.descriptors().iter().enumerate().map(|(i,d)|json!({"functional_index":i,"offset_bits":products(&d.offset),"terms":d.terms.iter().map(|t|json!({"dof":t.dof,"product_bits":products(&t.products)})).collect::<Vec<_>>()})).collect();
    let load_reference_state = match input.load_state {
        Some(state) => json!({
            "resolved_evidence_sha256": hash("load_reference_state_resolved_evidence_v1", &state.evidence)?,
            "members": state.members.iter().map(|m| json!({"pipe_id":m.pipe_id,"material_id":m.material.material_id,
                "selection_kind":m.material.selection_kind,"E_bits":bits(m.material.pair.elastic_modulus_pa()),
                "nu_bits":bits(m.material.pair.poisson_ratio()),"G_bits":bits(m.material.pair.shear_modulus_pa()),
                "thermal_strain_bits":bits(m.strain.thermal_strain),"fit_strain_bits":bits(m.strain.fit_strain),
                "total_eigenstrain_bits":bits(m.strain.total_eigenstrain)})).collect::<Vec<_>>(),
            "eigen_loads": input.thermal_loads.iter().map(|l| {
                let pipe = &input.built.pipes[l.element_index].element_id;
                json!({"pipe_id":pipe,"force_source":source_recovery::eigen_source_id(pipe),
                    "axial_load_bits":bits(l.axial_load),"eigenstrain_bits":bits(l.thermal_strain),
                    "recovery_offset":"end_i_axial_plus_end_j_axial_minus"})
            }).collect::<Vec<_>>(),
            "prescribed_motions": state.prescribed.iter().map(|(dof, v)| json!({"dof":dof,"value_bits":bits(*v)})).collect::<Vec<_>>(),
        }),
        None => Value::Null,
    };
    let mut payload = json!({"payload_version":"1.0.0","invocation_sha256":invocation,"case_id":input.load_case.id,"source_identity":response.source_identity(),"material_basis_record":material_basis_record,"dof_map":input.model.nodes.iter().flat_map(|n|(0..6).map(move |c|json!({"node_id":n.id,"component":(["UX","UY","UZ","RX","RY","RZ"][c]),"displacement_unit":if c<3{"m"}else{"rad"},"action_unit":if c<3{"N"}else{"N*m"}}))).collect::<Vec<_>>(),"stiffness_aggregate_bits":matrix(system.stiffness),"force_aggregate_bits":system.force.iter().copied().map(bits).collect::<Vec<_>>(),"stiffness_terms":stiffness,"force_terms":forces.iter().map(|f|json!({"owner":match eigen_sources.get(&f.source){Some(pipe)=>owner("member_eigenstrain",pipe),None=>owner("load",&f.source)},"dof":f.dof,"value_bits":bits(f.value)})).collect::<Vec<_>>(),"free_dofs":system.free_dofs,"prescribed":prescribed,"frames":frames,"springs":springs,"supported_loads":supported_loads,"observed_family_ids":{"frames":input.built.pipes.iter().map(|p|&p.element_id).collect::<Vec<_>>(),"supports":input.built.supports.iter().map(|s|&s.support_id).collect::<Vec<_>>(),"loads":input.load_case.primitive_loads.iter().map(|p|&p.id).collect::<Vec<_>>(),"modifiers":input.model.components.iter().map(|c|&c.id).collect::<Vec<_>>(),"nonlinear_supports":input.model.supports.iter().filter(|s|s.nonlinear.is_some()).map(|s|&s.id).collect::<Vec<_>>(),"user_elements":[],"curved_elements":[],"combinations":input.model.combinations.iter().map(|c|&c.id).collect::<Vec<_>>()},"lowered_recovery_operands":lowered});
    // Pre-0.4 payload bytes are unchanged; only a resolved case adds its block.
    if !load_reference_state.is_null() {
        payload["load_reference_state"] = load_reference_state;
    }
    // RF design §7: only a source with a congruence member names them; without
    // one, the payload and its hash keep their structure and bytes.
    if !congruence_ids.is_empty() {
        payload["congruence_members"] = json!(congruence_ids);
    }
    #[cfg(test)]
    OBSERVED_CONGRUENCE_MEMBERS.with(|o| o.borrow_mut().push(payload.get("congruence_members").cloned()));
    let normalized = hash("source_blocks_normalized_source_v1", &payload)?;
    let functions: Vec<_> = retained
        .descriptors()
        .iter()
        .enumerate()
        .map(|(i, d)| function(&payload, input, d, i, bindings))
        .collect::<Result<_, _>>()?;
    let plan = json!({"payload_version":"1.0.0","normalized_source_sha256":normalized,"functions":functions,"derived_recipes":recipes(&payload,input,selected,ledger,actual)?,"observation_result_ids":actual.iter().filter(|r|rows::observation(r)).map(|r|&r.id).collect::<Vec<_>>()});
    let plan_hash = hash("source_blocks_functional_plan_v1", &plan)?;
    #[cfg(test)]
    record_source_dump(&input.load_case.id, &payload, &normalized, &plan, &plan_hash)?;
    let blocks: Vec<_> = response
        .block_witnesses()
        .iter()
        .map(|b| b.dofs().to_vec())
        .collect();
    let all: BTreeSet<_> = system.free_dofs.iter().copied().collect();
    let flat: Vec<_> = blocks.iter().flatten().copied().collect();
    if system.force.len() > 256
        || terms.len() + forces.len() > 16_384
        || blocks.iter().any(|b| b.is_empty() || b.len() > 4)
        || flat.len() != all.len()
        || flat.iter().copied().collect::<BTreeSet<_>>() != all
    {
        return Err(bad("source block coverage"));
    }
    // RF design §7: the level states the source basis.
    let level = if congruence.is_empty() {
        "complete_identified_represented_contributions"
    } else {
        "complete_identified_declared_member_congruence"
    };
    Ok(
        json!({"level":level,"normalized_source_sha256":normalized,"functional_plan_sha256":plan_hash,"dof_count":system.force.len(),"stiffness_term_count":terms.len(),"force_term_count":forces.len(),"functional_count":retained.descriptors().len(),"free_dofs":system.free_dofs,"prescribed_dofs":system.prescribed.iter().map(|x|x.0).collect::<Vec<_>>(),"free_blocks":blocks,"member_ids":selected.members().iter().map(|m|&m.member_id).collect::<Vec<_>>(),"support_ids":selected.support_actions().iter().map(|s|&s.support_id).collect::<Vec<_>>() }),
    )
}
pub(super) fn failure_fields(
    f: &source_recovery::RecoveryFailure,
) -> (&'static str, &'static str, Option<usize>) {
    use source_recovery::RecoveryError as R;
    let stage = match f.helper_stage {
        AttemptStage::SourceClosure => "source_validation",
        AttemptStage::Preparation => "source_validation",
        AttemptStage::Solve => "exact_solve",
        AttemptStage::Plan | AttemptStage::Evaluation => "functional_evaluation",
        AttemptStage::Projection => "projection",
        AttemptStage::Retention | AttemptStage::Replay => "finalization",
    };
    let (code, block) = match &f.error {
        R::Unsupported(s) if s.contains("coincident") => ("support_attribution_ambiguous", None),
        R::Unsupported(s) if s.contains("transform") => ("unsupported_source_closure", None),
        R::Unsupported(_) => ("unsupported_family", None),
        R::SourceMismatch(_) => ("source_mismatch", None),
        R::Exact(exact::Error::UnsupportedBlock { order }) => ("unsupported_block", Some(*order)),
        R::Exact(exact::Error::NotPositiveDefinite) => ("not_positive_definite", None),
        R::Exact(exact::Error::Budget) => ("budget", None),
        R::Exact(exact::Error::ProjectionUnresolved(_)) => ("projection_unresolved", None),
        R::Exact(exact::Error::Arithmetic(_)) => ("arithmetic_range", None),
        _ => ("invalid_source", None),
    };
    (stage, code, block)
}

#[cfg(test)]
thread_local! {
    static OBSERVED_CONGRUENCE_MEMBERS: std::cell::RefCell<Vec<Option<Value>>> = const { std::cell::RefCell::new(Vec::new()) };
}
/// RF-17: each normalized-source payload's `congruence_members` (absent: `None`)
/// committed on this thread since the last call, in order.
#[cfg(test)]
pub(crate) fn take_observed_congruence_members() -> Vec<Option<Value>> {
    OBSERVED_CONGRUENCE_MEMBERS.with(|o| std::mem::take(&mut *o.borrow_mut()))
}

#[cfg(test)]
thread_local! {
    static SOURCE_DUMP: std::cell::RefCell<Option<Vec<Value>>> = const { std::cell::RefCell::new(None) };
}
/// Domain of the normalized private source payload's commitment.
#[cfg(test)]
pub(crate) const NORMALIZED_SOURCE_DOMAIN: &str = "source_blocks_normalized_source_v1";
/// Domain of the private functional plan's commitment.
#[cfg(test)]
pub(crate) const FUNCTIONAL_PLAN_DOMAIN: &str = "source_blocks_functional_plan_v1";
/// O4 (DEL-04-08 Design `o4-source-extension.md` §5; the RF probe
/// precedent): the test-only declared-problem dump. While armed by
/// [`with_source_dump`], every normalized-source commitment made on this
/// thread (one per selected source case, in finalization order) appends one
/// record of the actual private payload and functional plan:
///
/// ```text
/// {"case_id": <load case id>,
///  "normalized_source_sha256": <hex>, "functional_plan_sha256": <hex>,
///  "normalized_source_text": <text>, "functional_plan_text": <text>}
/// ```
///
/// Each `*_text` is the checked canonical JSON text (canonical-json
/// `checked_v1`) of `{"domain": <domain>, "payload": <payload>}`, the exact
/// bytes the receipt's commitment hashes: `sha256(utf8(text))` is the
/// record's hash, which is the receipt's `source.normalized_source_sha256` or
/// `source.functional_plan_sha256` for that case. The domains are
/// [`NORMALIZED_SOURCE_DOMAIN`] and [`FUNCTIONAL_PLAN_DOMAIN`]; the payload is
/// `text["payload"]`. Floats in the payload are hex binary64 bit patterns
/// (`"%016x"`), so the declared operands are exact. The normalized payload
/// carries per frame `formation_inputs` (with `shear_area_bits` on a
/// Timoshenko frame only), `local_matrix_bits`, `transform_bits`,
/// `global_matrix_bits` and `recovery_section`, plus
/// `reduced_recovery_section` where the member's reduced stress section
/// differs from its nominal one; the plan carries the lowered functionals and
/// the derived recipes with their payload operand paths.
///
/// `o4_source_tests::o4_source_payload_dump` writes these records for a
/// given request to a file when `SWBPIPE_O4_SOURCE_DUMP_REQUEST` (a
/// `LinearStaticPreviewRequest` JSON file) and `SWBPIPE_O4_SOURCE_DUMP_OUT`
/// are set (`SWBPIPE_O4_SOURCE_DUMP_MODE`: `sparse_interactive`, the
/// default, or `dense_scrutiny`), as
/// `{"artifact": "o4-source-declared-problem-dump", "schema_version": "1.0.0",
/// "solver_mode": <mode>, "publication_sha256": <the envelope's
/// publication commitment, or null>, "records": [<record>...]}`.
#[cfg(test)]
pub(crate) fn with_source_dump<T>(run: impl FnOnce() -> T) -> (T, Vec<Value>) {
    SOURCE_DUMP.with(|dump| *dump.borrow_mut() = Some(Vec::new()));
    let result = run();
    let records = SOURCE_DUMP.with(|dump| dump.borrow_mut().take().unwrap_or_default());
    (result, records)
}
#[cfg(test)]
fn record_source_dump(
    case_id: &str,
    payload: &Value,
    normalized: &str,
    plan: &Value,
    plan_hash: &str,
) -> Result<(), ReceiptError> {
    if !SOURCE_DUMP.with(|dump| dump.borrow().is_some()) {
        return Ok(());
    }
    let record = json!({
        "case_id": case_id,
        "normalized_source_sha256": normalized,
        "functional_plan_sha256": plan_hash,
        "normalized_source_text": checked(&json!({"domain": NORMALIZED_SOURCE_DOMAIN, "payload": payload}))?,
        "functional_plan_text": checked(&json!({"domain": FUNCTIONAL_PLAN_DOMAIN, "payload": plan}))?,
    });
    SOURCE_DUMP.with(|dump| {
        if let Some(records) = dump.borrow_mut().as_mut() {
            records.push(record);
        }
    });
    Ok(())
}
