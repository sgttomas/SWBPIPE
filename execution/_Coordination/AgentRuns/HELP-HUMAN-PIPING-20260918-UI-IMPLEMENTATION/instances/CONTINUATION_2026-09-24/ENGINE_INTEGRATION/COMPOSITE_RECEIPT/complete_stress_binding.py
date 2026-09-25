from pathlib import Path
b=Path('projects/chirality-piping/core/product_physics/src')
p=b/'source_receipt.rs';s=p.read_text().replace('    derived_checks: Vec<Value>,','    derived_checks: Vec<Value>,\n    section_stress_checks: Vec<Value>,');s=s.replace('            derived_checks: vec![],','            derived_checks: vec![],\n            section_stress_checks: vec![],');s=s.replace('                row["derived_checks"] = json!(case.derived_checks);','                row["derived_checks"] = json!(case.derived_checks);\n                row["section_stress_checks"] = json!(case.section_stress_checks);');p.write_text(s)
p=b/'source_receipt/rows.rs';s=p.read_text();s=s.replace('recipe: "straight_open_stress_v1",\n                    inputs: vec![primary[&function].id.clone()],','recipe: if composite {"retained_source_straight_stress_v1"} else {"straight_open_stress_v1"},\n                    inputs: if composite {vec![]} else {vec![primary[&function].id.clone()]},');p.write_text(s)
p=b/'source_receipt/source.rs';s=p.read_text().replace('"straight_open_stress_v1"=>','"straight_open_stress_v1"|"retained_source_straight_stress_v1"=>');p.write_text(s)
p=b/'source_receipt/composite.rs';s=p.read_text();s=s.replace('            let mut derived_checks = Vec::new();','            let mut derived_checks = Vec::new();\n            let mut section_stress_checks = Vec::new();');s=s.replace('            let ledger = rows::bind_for(','''            for member in selected.members() {
                let section=input.built.sections.get(&member.member_id).ok_or_else(||bad("section stress geometry"))?;
                for row in actual.iter().filter(|r|r.entity_ref==member.member_id) {
                    let component=match row.kind.as_str() {"element_local_axial_normal_stress"=>0usize,"element_local_bending_normal_stress_y"=>4,"element_local_bending_normal_stress_z"=>5,"element_local_torsional_shear_stress"=>3,_=>continue};
                    let metadata=row.metadata.as_ref().ok_or_else(||bad("section stress metadata"))?;
                    let station=match metadata.location.as_str(){"end_i"=>0,"quarter_1"=>1,"midspan"=>2,"quarter_3"=>3,"end_j"=>4,_=>return Err(bad("section stress location"))};
                    let index=member.section_functional_indices[station][component];
                    let descriptor=selected.retained().descriptors().get(index).ok_or_else(||bad("section stress descriptor"))?;
                    if descriptor.key.quantity != (FunctionalQuantity::MemberSection {member:member.member_id.clone(),station_bits:source_recovery::STATIONS[station].to_bits(),component:component as u8}) || descriptor.key.convention!=FunctionalConvention::SectionLocal || descriptor.key.case_id!=input.load_case.id {return Err(bad("section stress functional binding"));}
                    let projection=selected.retained().projections().iter().find(|p|p.index()==index).ok_or_else(||bad("section stress projection"))?;
                    if !same(projection.value(),member.sections[station][component]) {return Err(bad("section stress action binding"));}
                    section_stress_checks.push(json!({"result_id":row.id,"recipe_id":"retained_source_straight_stress_v1","pipe_id":member.member_id,"location":metadata.location,"component":metadata.component,"functional_index":index,"functional_id":source::functional_id(&input.load_case.id,index),"action":{"value":projection.value(),"interval":projection.interval()},"parameters":{"area_m2":section.area,"section_modulus_m3":section.section_modulus,"torsion_radius_m":section.torsion_radius,"torsion_constant_m4":section.torsion_constant,"pa_per_mpa":1e6}}));
                }
            }
            let ledger = rows::bind_for(''',1)
s=s.replace('Ok((ledger, source, derived_checks))','Ok((ledger, source, derived_checks, section_stress_checks))');s=s.replace('let (ledger, source, derived_checks) =','let (ledger, source, derived_checks, section_stress_checks) =');s=s.replace('            derived_checks,\n            _selected:', '            derived_checks,\n            section_stress_checks,\n            _selected:');s=s.replace('            derived_checks: vec![],','            derived_checks: vec![],\n            section_stress_checks: vec![],');p.write_text(s)
# Tests remain within owned scope; production changes require parent lane release.
e=Path('projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/ENGINE_INTEGRATION/COMPOSITE_RECEIPT')
p=b/'source_receipt/tests.rs';p.write_text(p.read_text()+'\n'+(e/'tests.draft.rs').read_text())
p=b/'source_receipt/composite.rs';p.write_text(p.read_text()+'\n'+(e/'norm_tests.draft.rs').read_text())
