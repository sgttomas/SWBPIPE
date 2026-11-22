//! A pressure-free document's shear model selection through the project store (DEL-05-03 REQ-011; a new
//! document states Timoshenko). The document is saved with Save Local Project's store function
//! (`save_local_project`), reopened as the session reopens it (`open_local_project_in_native_custody`) and as
//! `open_local_project` does, and solved with the desktop's solve. A document stating a choice comes back stating
//! it, as stored, and solves on that choice's identity. A document that states nothing comes back stating nothing,
//! with nothing written on open, and solves Euler-Bernoulli: the new-document default never reaches an existing
//! document through the store. Invented data only (the bundled demo model).
use super::*;
use crate::memory_pool::test_host::ScriptedHost;
use crate::memory_pool::MemoryPool;
use crate::solve_worker::test_support::TestLauncher;
use std::sync::Arc;
use std::time::Duration;

const DEMO: &str = include_str!("../../../../fixtures/product_preview/invented_demo_model.json");
const TIMOSHENKO: &str = "timoshenko_annulus_energy_v1";

fn scratch_store(label: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!(
        "swbpipe-shear-store-{label}-{}-{}",
        std::process::id(),
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()
    ));
    fs::create_dir_all(&root).unwrap();
    root.join("store.sqlite")
}

fn registry() -> SolveJobRegistry {
    SolveJobRegistry::with_hooks(
        MemoryPool::with_wait_tick(ScriptedHost::normal(), Duration::from_millis(5)),
        TestLauncher::solve(),
        Arc::new(|job: Box<dyn FnOnce() + Send + 'static>| std::thread::Builder::new().spawn(job).map(drop)),
    )
}

/// The stored model text, as the store holds it.
fn stored_model_text(store: &Path, project_id: &str) -> String {
    let (connection, _) = open_project_store(store).unwrap();
    connection
        .query_row("SELECT model_json FROM local_projects WHERE project_id = ?1", rusqlite::params![project_id], |row| row.get(0))
        .unwrap()
}

#[test]
fn a_pressure_free_shear_model_selection_is_saved_reopened_and_solved_as_stated() {
    let registry = registry();
    for (stated, identity) in [
        (Some(TIMOSHENKO), "preview-physics-timoshenko-1"),
        (Some("euler_bernoulli"), "preview-physics-2"),
        (None, "preview-physics-2"),
    ] {
        let mut model: Value = serde_json::from_str(DEMO).unwrap();
        assert!(model.get("pressure_contract").is_none() && model.get("model_settings").is_none(), "the demo is pressure-free and states nothing");
        if let Some(stated) = stated {
            model["model_settings"] = json!({"shear_model": stated});
        }
        let project_id = model["project"]["id"].as_str().unwrap().to_string();
        let store = scratch_store(stated.unwrap_or("unstated"));
        save_project_in_store(
            store.clone(),
            SaveLocalProjectRequest::from_value(json!({
                "project_id": project_id, "project_name": "Invented shear model", "model": model,
                "model_hash": computed_model_hash(&model, &project_id), "confirmed_revision": null
            }))
            .unwrap(),
        )
        .unwrap_or_else(|error| panic!("{stated:?}: the document saves: {error}"));
        let text = stored_model_text(&store, &project_id);
        assert_eq!(serde_json::from_str::<Value>(&text).unwrap(), model, "{stated:?}: stored as saved");
        assert_eq!(text.contains("shear_model"), stated.is_some(), "{stated:?}: {text}");

        let epoch = registry.reset_epoch();
        let native = stored_custody::open_project_in_store(&stored_read::NativeLeaseMemory::unseeded(&registry), store.clone(), Some(&project_id), epoch)
            .unwrap()
            .expect("the session's open finds the project");
        let legacy = open_project_in_store(store.clone(), Some(&project_id)).unwrap().expect("open_local_project finds the project");
        for (route, opened) in [("open_local_project_in_native_custody", &native.model), ("open_local_project", &legacy.model)] {
            assert_eq!(opened, &model, "{stated:?} {route}: reopened as saved");
            assert_eq!(opened.pointer("/model_settings/shear_model").and_then(Value::as_str), stated, "{stated:?} {route}");
            let envelope = crate::solve_preview_mechanics(opened.clone()).unwrap();
            assert_eq!(envelope["status"]["mechanics"], "MECHANICS_SOLVED", "{stated:?} {route}: {}", envelope["diagnostics"]);
            let producer = envelope["producer"]["semantic_contract_id"].as_str().unwrap();
            assert!(producer.ends_with(&format!("/{identity}")), "{stated:?} {route}: {producer}");
        }
        assert_eq!(stored_model_text(&store, &project_id), text, "{stated:?}: opening rewrites nothing");
        let _ = registry.dispose_all_results();
        let _ = fs::remove_dir_all(store.parent().unwrap());
    }
}
