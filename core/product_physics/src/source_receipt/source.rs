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
            "translation_norm_scaled_v1"|"support_force_norm_scaled_v1"=>{},
            "straight_open_stress_v1"=>{
                let i=input.built.pipes.iter().position(|p|p.element_id==row.entity_ref).ok_or_else(||bad("recipe member"))?;
                let section=input.built.sections.get(&row.entity_ref).ok_or_else(||bad("recipe section"))?;
                let operands:Vec<(&str,String,f64)>=match row.kind.as_str(){"element_local_axial_normal_stress"=>vec![("area",format!("/frames/{i}/formation_inputs/area_bits"),section.area)],"element_local_bending_normal_stress_y"|"element_local_bending_normal_stress_z"=>vec![("section_modulus",format!("/frames/{i}/recovery_section/section_modulus_bits"),section.section_modulus)],"element_local_torsional_shear_stress"=>vec![("torsion_radius",format!("/frames/{i}/recovery_section/torsion_radius_bits"),section.torsion_radius),("torsion_constant",format!("/frames/{i}/formation_inputs/torsion_constant_bits"),section.torsion_constant)],_=>return Err(bad("recipe signature"))};
                for (name,path,v) in operands{parameters.push(json!({"name":name,"operand":source_operand(payload,path,v)?}));}
                parameters.push(json!({"name":"pa_per_mpa","operand":{"kind":"method_constant","bits":bits(1_000_000.)}}));
            },
            "reviewed_stress_summary_v1"=>parameters.push(json!({"name":"pa_per_mpa","operand":{"kind":"method_constant","bits":bits(1_000_000.)}})),
            _=>return Err(bad("unimplemented recipe")),
        }
        recipes.push(json!({"result_id":row.id,"recipe_id":recipe,"input_result_ids":treatment.input_result_ids,"parameters":parameters}));
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
    for (frame, pipe) in input.built.frame_elements.iter().zip(&input.built.pipes) {
        let map = element_dof_map(frame.node_i.index, frame.node_j.index);
        let global = frame.global_stiffness().map_err(|e| bad(e.to_string()))?;
        let local = frame.local_stiffness().map_err(|e| bad(e.to_string()))?;
        let transform = frame
            .orientation()
            .map_err(|e| bad(e.to_string()))?
            .transformation_matrix();
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
        frames.push(json!({"element_id":pipe.element_id,"node_i_id":input.model.nodes[frame.node_i.index].id,"node_j_id":input.model.nodes[frame.node_j.index].id,"scatter":map,"formation_inputs":{"coordinates_bits":[frame.node_i.coordinates.map(bits),frame.node_j.coordinates.map(bits)],"young_modulus_bits":bits(frame.section.elastic_modulus),"shear_modulus_bits":bits(frame.section.shear_modulus),"area_bits":bits(frame.section.area),"iy_bits":bits(frame.section.second_moment_y),"iz_bits":bits(frame.section.second_moment_z),"torsion_constant_bits":bits(frame.section.torsion_constant),"length_bits":bits(frame.length().map_err(|e|bad(e.to_string()))?)},"local_matrix_bits":matrix12(&local),"transform_bits":matrix12(&transform),"global_matrix_bits":matrix12(&global),"recovery_section":{"section_modulus_bits":bits(section.section_modulus),"torsion_radius_bits":bits(section.torsion_radius)}}));
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
    let mut supported_loads = Vec::new();
    for f in forces {
        let primitive = input
            .load_case
            .primitive_loads
            .iter()
            .position(|p| p.id == f.source)
            .ok_or_else(|| bad("force primitive owner"))?;
        supported_loads.push(json!({"load_id":f.source,"node_id":input.model.nodes[f.dof/6].id,"dof":f.dof,"value_bits":bits(f.value),"source_primitive_index":primitive}));
    }
    let lowered:Vec<_>=retained.descriptors().iter().enumerate().map(|(i,d)|json!({"functional_index":i,"offset_bits":products(&d.offset),"terms":d.terms.iter().map(|t|json!({"dof":t.dof,"product_bits":products(&t.products)})).collect::<Vec<_>>()})).collect();
    let payload = json!({"payload_version":"1.0.0","invocation_sha256":invocation,"case_id":input.load_case.id,"source_identity":response.source_identity(),"material_basis_record":material_basis_record,"dof_map":input.model.nodes.iter().flat_map(|n|(0..6).map(move |c|json!({"node_id":n.id,"component":(["UX","UY","UZ","RX","RY","RZ"][c]),"displacement_unit":if c<3{"m"}else{"rad"},"action_unit":if c<3{"N"}else{"N*m"}}))).collect::<Vec<_>>(),"stiffness_aggregate_bits":matrix(system.stiffness),"force_aggregate_bits":system.force.iter().copied().map(bits).collect::<Vec<_>>(),"stiffness_terms":stiffness,"force_terms":forces.iter().map(|f|json!({"owner":owner("load",&f.source),"dof":f.dof,"value_bits":bits(f.value)})).collect::<Vec<_>>(),"free_dofs":system.free_dofs,"prescribed":prescribed,"frames":frames,"springs":springs,"supported_loads":supported_loads,"observed_family_ids":{"frames":input.built.pipes.iter().map(|p|&p.element_id).collect::<Vec<_>>(),"supports":input.built.supports.iter().map(|s|&s.support_id).collect::<Vec<_>>(),"loads":input.load_case.primitive_loads.iter().map(|p|&p.id).collect::<Vec<_>>(),"modifiers":input.model.components.iter().map(|c|&c.id).collect::<Vec<_>>(),"nonlinear_supports":input.model.supports.iter().filter(|s|s.nonlinear.is_some()).map(|s|&s.id).collect::<Vec<_>>(),"user_elements":[],"curved_elements":[],"combinations":input.model.combinations.iter().map(|c|&c.id).collect::<Vec<_>>()},"lowered_recovery_operands":lowered});
    let normalized = hash("source_blocks_normalized_source_v1", &payload)?;
    let functions: Vec<_> = retained
        .descriptors()
        .iter()
        .enumerate()
        .map(|(i, d)| function(&payload, input, d, i, bindings))
        .collect::<Result<_, _>>()?;
    let plan = json!({"payload_version":"1.0.0","normalized_source_sha256":normalized,"functions":functions,"derived_recipes":recipes(&payload,input,ledger,actual)?,"observation_result_ids":actual.iter().filter(|r|rows::observation(r)).map(|r|&r.id).collect::<Vec<_>>()});
    let plan_hash = hash("source_blocks_functional_plan_v1", &plan)?;
    let blocks: Vec<_> = response
        .block_witnesses()
        .iter()
        .map(|b| b.dofs().to_vec())
        .collect();
    let all: BTreeSet<_> = system.free_dofs.iter().copied().collect();
    let flat: Vec<_> = blocks.iter().flatten().copied().collect();
    if system.force.len() > 256
        || terms.len() + forces.len() > 16_384
        || blocks.iter().any(|b| b.is_empty() || b.len() > 2)
        || flat.len() != all.len()
        || flat.iter().copied().collect::<BTreeSet<_>>() != all
    {
        return Err(bad("source block coverage"));
    }
    Ok(
        json!({"level":"complete_identified_represented_contributions","normalized_source_sha256":normalized,"functional_plan_sha256":plan_hash,"dof_count":system.force.len(),"stiffness_term_count":terms.len(),"force_term_count":forces.len(),"functional_count":retained.descriptors().len(),"free_dofs":system.free_dofs,"prescribed_dofs":system.prescribed.iter().map(|x|x.0).collect::<Vec<_>>(),"free_blocks":blocks,"member_ids":selected.members().iter().map(|m|&m.member_id).collect::<Vec<_>>(),"support_ids":selected.support_actions().iter().map(|s|&s.support_id).collect::<Vec<_>>() }),
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
