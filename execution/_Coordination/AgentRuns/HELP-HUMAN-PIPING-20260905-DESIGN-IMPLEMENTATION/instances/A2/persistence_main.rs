
fn main() {
 let args:Vec<String>=std::env::args().collect();
 let packet=std::path::Path::new(&args[1]);
 let db=std::path::Path::new(&args[2]);
 let mut model:Value=serde_json::from_str(&std::fs::read_to_string(packet.join("fixtures/invented_linear_m.json")).unwrap()).unwrap();
 let result:Value=serde_json::from_str(&std::fs::read_to_string(packet.join("results/native_m.json")).unwrap()).unwrap();
 let project_id=model["project"]["id"].as_str().unwrap().to_owned();
 let mut conn=Connection::open(db).unwrap();
 let migration=apply_store_migrations(&conn).unwrap();
 let first=model.clone();
 let run1=json!({"run_id":result["run_id"],"model_state_ref":{"object_type":"ModelState","id":format!("state:{}:preview",project_id)},"witness":"invented_input_pass_through_not_schema_acceptance","settings":{"profile":"linear"}});
 let hash1=json!({"witness_hash_label":"first"});let hash2=json!({"witness_hash_label":"second"});
 upsert_project(&mut conn,&project_id,"A2 invented",&model,&json!([]),&Value::Null,&Value::Null,&result,&run1,&hash1,&Value::Null,&json!([])).unwrap();
 drop(conn);
 let mut conn=Connection::open(db).unwrap();apply_store_migrations(&conn).unwrap();
 let restored1=load_project(&conn,Some(&project_id)).unwrap().unwrap();
 let first_roundtrip=restored1.model==first && restored1.mechanics_result==result && restored1.analysis_run==run1 && restored1.model_hash==hash1;
 model["nodes"][1]["position"]["x"]=json!(3);
 let mut run2=run1.clone();run2["witness_revision"]=json!(2);
 upsert_project(&mut conn,&project_id,"A2 invented",&model,&json!([]),&Value::Null,&Value::Null,&result,&run2,&hash2,&Value::Null,&json!([])).unwrap();
 drop(conn);
 let conn=Connection::open(db).unwrap();let restored2=load_project(&conn,Some(&project_id)).unwrap().unwrap();
 let second_roundtrip=restored2.model==model && restored2.analysis_run==run2 && restored2.model_hash==hash2;
 let count:i64=conn.query_row("SELECT COUNT(*) FROM local_projects WHERE project_id=?1",params![project_id],|r|r.get(0)).unwrap();
 let tables:Vec<String>=conn.prepare("SELECT name FROM sqlite_master WHERE type='table' ORDER BY name").unwrap().query_map([],|r|r.get(0)).unwrap().collect::<Result<_,_>>().unwrap();
 println!("{}",serde_json::to_string_pretty(&json!({"migration":migration,"first_close_reopen_exact":first_roundtrip,"second_close_reopen_exact":second_roundtrip,"model_changed":first!=model,"project_rows_after_two_saves":count,"first_revision_currently_retrievable":restored2.model==first,"stored_analysis_run":restored2.analysis_run,"tables":tables,"scope":"unchanged_native_private_storage_helpers_extracted_verbatim; bypasses_full_command_validation_and_UI; no_durable_history_claim"})).unwrap());
}
