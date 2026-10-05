#[test]
fn rv60_g5a_join_mutation_controls() {
 use super::retained_product::{G5aFailure,OperationalError,ScalarWork};
 let (mut e,mut o)=observed(specimen(false));
 // Explicit synthetic clean-zero final snapshot to isolate all gates from the known -0 failure.
 for r in &mut e.results {if r.value==0.0 {r.value=0.0;}}
 let (inv,case)=o.native.take().unwrap();let k::ExecutionOutcome::Selected(owner)=&case.outcome else{panic!()};
 let rows=o.bind_rows(&e,owner).unwrap();let spent=inv.certify_product_case(case.run,owner,&o.facts,&rows);
 assert!(spent.passed());o.verdicts=spent.verdicts().to_vec();o.g5a_work=ScalarWork::default();
 assert!(o.rv60_check_g5a(owner,&rows).is_ok());assert!(o.observables(&e).is_ok());
 // Coherent final evidence mutation; all numeric fields remain exactly identical.
 let mut changed=e.clone();changed.results[0].metadata.as_mut().unwrap().sign_convention="mode_code 1=dense_scrutiny".into();
 let badrows=o.bind_rows(&changed,owner).unwrap();assert!(inv.certify_product_case(case.run,owner,&o.facts,&badrows).passed());
 assert!(o.rv60_check_g5a(owner,&badrows).is_ok());assert!(o.observables(&changed).is_ok());
 println!("RV60_SYNTHETIC_FULL_GATES_ACCEPT_WRONG_MODE_SIGN true");
 let saved=o.operational[0].inputs[0];o.operational[0].inputs[0]=1;
 assert!(matches!(o.rv60_check_g5a(owner,&rows),Err(G5aFailure::Operational{cause:OperationalError::MissingOrForeign,..})));o.operational[0].inputs[0]=saved;
 o.operational[0].work.lost=true;
 assert!(matches!(o.rv60_check_g5a(owner,&rows),Err(G5aFailure::Operational{cause:OperationalError::Accounting,..})));
 println!("RV60_G5A_FOREIGN_AND_LOST_OPERANDS_REJECTED true");
}
#[test]
fn rv60_closed_evidence_missing_gate_control() {
 let (e,o)=observed(specimen(true));
 let mut changed=e.clone();changed.contract_evidence.as_mut().unwrap().as_object_mut().unwrap().remove("combination_gates");
 println!("RV60_MISSING_COMBINATION_GATES_ACCEPTED {}",o.observables(&changed).is_ok());
 let mut changed=e.clone();changed.contract_evidence.as_mut().unwrap()["combination_gates"]=serde_json::json!([{"foreign":"entry"}]);
 println!("RV60_FOREIGN_COMBINATION_GATES_ACCEPTED {}",o.observables(&changed).is_ok());
 let mut changed=e.clone();changed.contract_evidence.as_mut().unwrap()["preview_cases"][0]["foreign"]=serde_json::json!(true);
 println!("RV60_EXTRA_CASE_KEY_ACCEPTED {}",o.observables(&changed).is_ok());
}
