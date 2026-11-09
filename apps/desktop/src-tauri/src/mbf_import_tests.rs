//! File > Import MBF…'s native path (DEL-17-04 Design `desktop-mbf-export-import.md` §4.8.9, slice 5): the commands'
//! ingress and ledger, task 5 in a real worker process (this test binary run as the worker), the decode and the
//! replies, over the actual `save_mbf_export` command's pair for the maintained invented export model
//! (`fixtures/caepipe_mbf/invented/desktop_export/`), bound and unbound; the App reply fixture follows the same
//! native-export-to-import path. Other file/refusal cases use engine-written inputs. Then acquisition races and change refusals,
//! the tickets and keyed retirements, Adopt's staleness, the decode's tamper refusals, the ledger and cancellation,
//! and the source scan that keeps task 5's arm the only production caller of `worker_import`.

use super::*;
use crate::command_ingress::test_support::headers;
use crate::command_ingress::INGRESS_ARGS_JSON_V1;
use crate::memory_pool::test_host::ScriptedHost;
use crate::memory_pool::MemoryPool;
use crate::solve_worker::test_support::{Gate, TestLauncher};
use crate::solve_worker::WorkerLauncher;
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

const EXPORT_MODEL: &str = include_str!("../../../../fixtures/caepipe_mbf/invented/desktop_export/export_model.json");
const EXPORT_DIGESTS: &str = include_str!("../../../../fixtures/caepipe_mbf/invented/desktop_export/export_digests.json");
const EXPORT_SETTINGS: &str = r#"{"title":"Invented MBF export","referenceTemperature":{"value":20,"unit":"degC"},"maxLayoutRows":null}"#;

// -- Fixtures and helpers -------------------------------------------------------------------------------------------

fn hex(bytes: &[u8]) -> String {
    ascii(&sha256_hex(bytes)).to_string()
}

/// The writer's MBF and identity file for the invented export model under the desktop's default settings: the bytes
/// `save_mbf_export` writes (their digests are the recorded `degc-default` case's).
fn written_pair() -> (Vec<u8>, Vec<u8>) {
    let settings = mbf::export_settings_from_json(EXPORT_SETTINGS.as_bytes()).unwrap();
    let model = mbf::parse_sidecar_json(EXPORT_MODEL.as_bytes()).unwrap();
    let result = mbf::render_si(&model, &settings).unwrap();
    let text = result.text.clone().unwrap().into_bytes();
    let sidecar = result.sidecar_bytes(&hex(&text)).unwrap();
    let recorded: Value = serde_json::from_str(EXPORT_DIGESTS).unwrap();
    let case = &recorded["cases"]["degc-default"];
    assert_eq!(case["mbf_sha256"], hex(&text), "the pair is the export's saved MBF");
    assert_eq!(case["sidecar_sha256"], hex(&sidecar), "and its saved identity file");
    (text, sidecar)
}

struct Sandbox(PathBuf);

impl Sandbox {
    fn new(label: &str) -> Self {
        let path = std::env::temp_dir().join(format!(
            "swbpipe-mbf-import-{label}-{}-{}",
            std::process::id(),
            std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()
        ));
        fs::create_dir_all(&path).unwrap();
        Sandbox(path)
    }

    /// Writes the pair as `swbpipe-model.mbf` and `swbpipe-model.swbpipe-ids.json`; returns the MBF's path.
    fn pair(&self, with_identity: bool) -> PathBuf {
        let (text, sidecar) = written_pair();
        let mbf_path = self.0.join("swbpipe-model.mbf");
        fs::write(&mbf_path, text).unwrap();
        if with_identity {
            fs::write(self.ids(), sidecar).unwrap();
        }
        mbf_path
    }

    fn ids(&self) -> PathBuf {
        self.0.join("swbpipe-model.swbpipe-ids.json")
    }

    /// Exercise the actual native review/save command path (task 3 and its file transaction), then let task 5
    /// open those files. Using this in the reply writer connects the maintained App fixture to native export too.
    fn native_pair(&self, registry: &SolveJobRegistry) -> PathBuf {
        use crate::{mbf_export, mbf_worker};
        let _save_lock = crate::atomic_report_package_save::test_support::atomic_save_test_lock();
        let runs = mbf_worker::MbfRuns::default();
        let text = open_pipe_stress_operation_applier::canonical_json(&serde_json::from_str::<Value>(EXPORT_MODEL).unwrap());
        let model_hash = format!("sha256:{}", hex(text.as_bytes()));
        let settings = EXPORT_SETTINGS;
        let mut args = json!({"expectedEpoch": runs.epoch(), "canonicalText": text, "claimedModelHash": model_hash, "settingsJson": settings});
        let (bytes, copy) = mbf_worker::reserve(registry, &raw(&args), &headers(Some(INGRESS_ARGS_JSON_V1)), mbf_export::REVIEW_COMMAND).unwrap();
        let reviewed: Value = mbf_worker::review_command(registry, &runs, bytes, copy, |_, text| serde_json::from_str(&text).unwrap()).unwrap();
        assert_eq!(reviewed["outcome"], "reviewed", "{reviewed}");
        args["suggestedName"] = json!("swbpipe-model.mbf");
        args["reviewed"] = json!({"mbfSha256": reviewed["review"]["mbf_sha256"], "sidecarSha256": reviewed["review"]["sidecar_sha256"]});
        args["screening"] = json!({
            "route_id": mbf_export::MBF_EXPORT_ROUTE_ID, "export_context": "local_private", "explicit_local_private_intent": true,
            "blocked": false, "materialization_withheld": false, "lossless_required": true, "exact_payload_match": true,
            "blocking_count": 0, "payload_sha256": model_hash
        });
        args["localFirstEvidence"] = json!({
            "route_id": mbf_export::MBF_EXPORT_ROUTE_ID, "export_context": "local_private", "storage_context": "local_private",
            "action": "include_metadata_only", "reason_code": "PRIVATE_LOCAL_METADATA_ALLOWED", "blocked": false,
            "metadata_only": true, "explicit_local_private_intent": true
        });
        let path = self.0.join("swbpipe-model.mbf");
        let (bytes, copy) = mbf_worker::reserve(registry, &raw(&args), &headers(Some(INGRESS_ARGS_JSON_V1)), mbf_export::COMMAND).unwrap();
        let saved: Value = mbf_worker::save_command(
            registry, &runs, bytes, copy, || Err("the invented fixture has no stored library references".into()),
            |_| Ok(Some(path.clone())), |_, text| serde_json::from_str(&text).unwrap(),
        ).unwrap();
        assert_eq!(saved["outcome"], "saved", "{saved}");
        assert_eq!(hex(&fs::read(&path).unwrap()), reviewed["review"]["mbf_sha256"]);
        assert_eq!(hex(&fs::read(self.ids()).unwrap()), reviewed["review"]["sidecar_sha256"]);
        assert_released(registry);
        path
    }
}

impl Drop for Sandbox {
    fn drop(&mut self) {
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Ok(entries) = fs::read_dir(&self.0) {
                for entry in entries.flatten() {
                    let _ = fs::set_permissions(entry.path(), fs::Permissions::from_mode(0o644));
                }
            }
        }
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn registry_with(launcher: Arc<dyn WorkerLauncher>) -> SolveJobRegistry {
    SolveJobRegistry::with_hooks(
        MemoryPool::with_wait_tick(ScriptedHost::normal(), Duration::from_millis(5)),
        launcher,
        Arc::new(|job: Box<dyn FnOnce() + Send + 'static>| std::thread::Builder::new().spawn(job).map(drop)),
    )
}

fn real_worker() -> SolveJobRegistry {
    registry_with(TestLauncher::solve())
}

fn assert_released(registry: &SolveJobRegistry) {
    assert_eq!(registry.pool_status().reserved_bytes, 0, "every lease is released");
    assert_eq!(registry.pool().running_workers(), 0, "every worker is reaped");
}

/// `body` with the current ticket unless it names one (or names `"absent"`, which removes it).
fn ticketed(imports: &MbfImports, body: &Value) -> Value {
    static SEQUENCE: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);
    let mut body = body.clone();
    match body.get(EXPECTED_EPOCH) {
        None => body[EXPECTED_EPOCH] = json!(imports.epoch()),
        Some(value) if value.as_str() == Some("absent") => drop(body.as_object_mut().unwrap().remove(EXPECTED_EPOCH)),
        Some(_) => {}
    }
    match body.get(REQUEST_SEQUENCE).and_then(Value::as_str) {
        Some("absent") => { body.as_object_mut().unwrap().remove(REQUEST_SEQUENCE); }
        _ if body.get(REQUEST_SEQUENCE).is_none() => body[REQUEST_SEQUENCE] = json!(SEQUENCE.fetch_add(1, Ordering::SeqCst)),
        _ => {}
    }
    body
}

fn raw(body: &Value) -> InvokeBody {
    InvokeBody::Raw(serde_json::to_vec(body).unwrap())
}

/// A reply and whether the pool still held the result lease and the reply's lease while the response was built.
struct Replied {
    value: Value,
    text: String,
    held_during_response: u64,
}

fn parse_reply(text: String, registry: &SolveJobRegistry) -> Replied {
    Replied {
        value: serde_json::from_str(&text).unwrap_or_else(|error| panic!("the reply is JSON ({error}): {text}")),
        text,
        held_during_response: registry.pool_status().reserved_bytes,
    }
}

fn open_with(registry: &SolveJobRegistry, imports: &MbfImports, body: &Value, choose: impl FnOnce() -> Result<Option<PathBuf>, String>) -> Replied {
    let body = ticketed(imports, body);
    let (raw_args, copy) = reserve(registry, &raw(&body), &headers(Some(INGRESS_ARGS_JSON_V1)), OPEN_COMMAND).unwrap();
    open_command(registry, imports, raw_args, copy, START_WAIT, choose, |text| parse_reply(text, registry)).unwrap()
}

fn open(registry: &SolveJobRegistry, imports: &MbfImports, path: &Path) -> Replied {
    let path = path.to_path_buf();
    open_with(registry, imports, &json!({}), move || Ok(Some(path)))
}

fn review(registry: &SolveJobRegistry, imports: &MbfImports, body: &Value) -> Replied {
    let body = ticketed(imports, body);
    let (raw_args, copy) = reserve(registry, &raw(&body), &headers(Some(INGRESS_ARGS_JSON_V1)), REVIEW_COMMAND).unwrap();
    review_command(registry, imports, raw_args, copy, START_WAIT, |text| parse_reply(text, registry)).unwrap()
}

fn adopt(registry: &SolveJobRegistry, imports: &MbfImports, body: &Value) -> Replied {
    let body = ticketed(imports, body);
    let (raw_args, copy) = reserve(registry, &raw(&body), &headers(Some(INGRESS_ARGS_JSON_V1)), ADOPT_COMMAND).unwrap();
    adopt_command(registry, imports, raw_args, copy, START_WAIT, |text| parse_reply(text, registry)).unwrap()
}

fn codes(reply: &Value) -> Vec<String> {
    reply["diagnostics"].as_array().map(|items| items.iter().map(|item| item["code"].as_str().unwrap().to_string()).collect()).unwrap_or_default()
}

fn page(n: usize) -> String {
    format!("{:032x}", 0xcd00_0000_0000_0000_0000_0000_0000_0000u128 + n as u128)
}

fn retire(registry: &SolveJobRegistry, imports: &MbfImports, n: usize, sequence: u64) -> MbfImportRetirement {
    let args = json!({ RETIREMENT_ID: { "page": page(n), "sequence": sequence } });
    cancel_command(registry, imports, &InvokeBody::Json(args), &headers(None)).unwrap()
}

/// The ids of a member's entries (`id`), as a set (an import orders entries by the file's layout).
fn ids_of(document: &Value, member: &str) -> std::collections::BTreeSet<String> {
    document[member].as_array().map(|items| items.iter().filter_map(|item| item["id"].as_str().map(String::from)).collect()).unwrap_or_default()
}

/// The engine's own import of the same bytes (the parity reference for an adopted document).
fn engine_model(mbf_bytes: &[u8], sidecar: Option<&[u8]>, declaration: Option<&[u8]>) -> Vec<u8> {
    let mut options = mbf::ImportOptions::new("swbpipe-model.mbf");
    options.sidecar = sidecar.map(|bytes| mbf::SidecarInput::Json(bytes.to_vec()));
    options.material_provenance = declaration.map(|bytes| mbf::parse_sidecar_json(bytes).unwrap());
    mbf::import(mbf_bytes, &options).unwrap().model_bytes().unwrap().expect("imported")
}

fn declaration_for(names: &[Value]) -> Value {
    let mut map = serde_json::Map::new();
    for name in names {
        map.insert(name.as_str().unwrap().to_string(), json!({"origin": "project_invented", "source_name": "project-invented example data"}));
    }
    Value::Object(map)
}

/// A complete review within R: no list holds anything back and the review fits the cap. Whether it blocks is the
/// caller's to assert.
fn assert_complete(review: &Value) {
    assert_eq!(review["completeness"], "complete", "{review}");
    assert_eq!(review["field_max_bytes"], mbf::REVIEW_FIELD_MAX_BYTES);
    assert_eq!(review["counts"]["not_shown"], 0);
    assert_eq!(review["counts"]["not_shown_blocking"], 0);
    let lists = review["counts"]["lists"].as_object().unwrap();
    assert_eq!(lists.len(), 6, "{review}");
    for (list, count) in lists {
        assert_eq!(count["listed"], count["total"], "{list}");
        assert_eq!((count["not_shown"].as_u64(), count["not_shown_blocking"].as_u64()), (Some(0), Some(0)), "{list}");
        assert_eq!(review[list.as_str()].as_array().map(Vec::len), count["total"].as_u64().map(|total| total as usize), "{list} is listed in full");
    }
    // Re-serialized (the fixture's review is ASCII with integer counts, so the length is the review's).
    assert!(serde_json::to_string(review).unwrap().len() <= REVIEW_MAX_BYTES);
}

/// The review's per-class identity inventory: each of the nine classes once, as (count, restored, formed).
fn identity_classes(review: &Value) -> BTreeMap<String, (u64, u64, u64)> {
    let classes = review["identity"]["classes"].as_array().unwrap();
    let inventory: BTreeMap<_, _> = classes
        .iter()
        .map(|class| {
            let number = |field: &str| class[field].as_u64().unwrap_or_else(|| panic!("{class}"));
            (class["class"].as_str().unwrap().to_string(), (number("count"), number("restored"), number("formed_from_mbf_names")))
        })
        .collect();
    assert_eq!(inventory.len(), classes.len(), "each class once");
    inventory
}

/// The source model's identity inventory in the review's classes, counted independently from the export model.
fn source_identity_counts(source: &Value) -> BTreeMap<String, u64> {
    let count = |member: &str| source[member].as_array().map_or(0, Vec::len) as u64;
    let cases = source["load_cases"].as_array().unwrap();
    let in_cases = |member: &str| cases.iter().map(|case| case[member].as_array().map_or(0, Vec::len) as u64).sum::<u64>();
    let bends = source["components"].as_array().unwrap().iter().filter(|component| component["kind"] == "bend").count() as u64;
    let referenced: BTreeSet<&str> = source["pipe_segments"].as_array().unwrap().iter().filter_map(|pipe| pipe["section_ref"].as_str()).collect();
    [
        ("node", count("nodes")),
        ("element", count("pipe_segments")),
        ("bend", bends),
        ("support", count("supports")),
        ("load_case", count("load_cases")),
        ("material", count("materials")),
        ("load", in_cases("primitive_loads")),
        ("pressure_region", in_cases("pressure_regions")),
        ("section", referenced.len() as u64),
    ]
    .into_iter()
    .map(|(class, n)| (class.to_string(), n))
    .collect()
}

/// `(case id, entry id)` for every entry of `member` in every load case: the primitive-load and pressure-region
/// identity inventories.
fn case_inventory(document: &Value, member: &str) -> BTreeSet<(String, String)> {
    let mut out = BTreeSet::new();
    for case in document["load_cases"].as_array().unwrap() {
        for entry in case[member].as_array().map(Vec::as_slice).unwrap_or_default() {
            out.insert((case["id"].as_str().unwrap().to_string(), entry["id"].as_str().unwrap().to_string()));
        }
    }
    out
}

// -- Source-to-adopted canonical parity (import Design `caepipe-mbf-import.md` §18.2) --------------------------------
//
// The maintained comparator is `tests/caepipe_mbf_canonical.py`, which has no route from this native test. This is an
// independent reduction of the same canonical form, limited to the families of the invented export model (an
// anchored bent run, two library sections, a linear spring, one force and per-pipe thermal changes, no pressure).
// This is a fixed-input comparison, not a coverage classifier: it compares only the fields enumerated below,
// rejects unhandled units/load categories, and relies on the saved-pair fixture bindings to constrain the input.
// Its bounds are §18.2's, taken from the source
// values: the export model's values have at most ten significant digits and the unit factors are powers of ten, so
// each printed value p is the source value and δ(p) scales with it. Where §18.2 adds model-specific terms (node
// paths, bend legs), only the term every bound includes is used, so no bound here is looser than §18.2's.

/// The binary64 unit roundoff, 2^-53.
const EPS: f64 = f64::EPSILON / 2.0;

fn to_si(value: f64, unit: &str) -> f64 {
    match unit {
        "m" | "N" | "N/m" | "Pa" | "1/degC" | "degC" | "rad" | "1" | "none" => value,
        "mm" => value / 1000.0,
        "N/mm" => value * 1000.0,
        "MPa" => value * 1e6,
        other => panic!("unit {other} is outside this fixture's comparison"),
    }
}

fn si(quantity: &Value) -> f64 {
    to_si(quantity["value"].as_f64().unwrap_or_else(|| panic!("{quantity}")), quantity["unit"].as_str().unwrap())
}

/// δ(p) = 0.5 × 10^(⌊log10|p|⌋ − 9), the half unit of the writer's tenth significant digit; 0 for zero.
fn delta(value: f64) -> f64 {
    if value == 0.0 {
        return 0.0;
    }
    let magnitude = value.abs();
    let mut exponent = magnitude.log10().floor() as i32;
    if 10f64.powi(exponent) > magnitude {
        exponent -= 1;
    } else if 10f64.powi(exponent + 1) <= magnitude {
        exponent += 1;
    }
    0.5 * 10f64.powi(exponent - 9)
}

#[derive(Clone, Copy, Debug)]
enum Bound {
    /// Printed directly: δ(p) + 4ε|p|.
    Printed,
    /// A node coordinate: 4εM, the import's term, which every node's §18.2 bound includes.
    Coordinate,
    /// G derived from the printed E and ν, when the source authored ν or G alone (not both): (δ(E)/E + δ(ν)/(1 + ν) + 8ε)|G|.
    Shear { e: f64, nu: f64 },
    /// ΔT = T1 − T_ref: δ(T1) + δ(T_ref) + 4ε|T1|.
    Thermal,
}

#[derive(Debug, Default)]
struct Canonical {
    values: BTreeMap<String, (f64, Bound)>,
    sets: BTreeMap<String, String>,
    planes: BTreeMap<String, [f64; 3]>,
}

fn norm(v: [f64; 3]) -> f64 {
    v[0].hypot(v[1]).hypot(v[2])
}

/// The bend plane as §18.2 compares it: `y` projected perpendicular to the unit chord from `a` to `b`, normalized.
fn project(y: [f64; 3], a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    let chord = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
    let length = norm(chord);
    let unit = chord.map(|value| value / length);
    let dot: f64 = (0..3).map(|axis| y[axis] * unit[axis]).sum();
    let side = [0, 1, 2].map(|axis| y[axis] - dot * unit[axis]);
    let length = norm(side);
    side.map(|value| value / length)
}

fn entries<'a>(value: &'a Value, member: &str) -> &'a [Value] {
    value[member].as_array().map(Vec::as_slice).unwrap_or_default()
}

fn by_id<'a>(document: &'a Value, member: &str, id: &Value) -> &'a Value {
    entries(document, member).iter().find(|item| item["id"] == *id).unwrap_or_else(|| panic!("{member} {id}"))
}

/// `document` reduced to the compared subset in SI base units, keyed by CAEPIPE node names (`nodes`, node id to
/// name) and case labels (`cases`, case id to label); pipes by their end names, ordered only when `oriented` (bound).
fn canonical(document: &Value, nodes: &BTreeMap<String, String>, cases: &BTreeMap<String, String>, oriented: bool) -> Canonical {
    let mut out = Canonical::default();
    let name = |id: &Value| nodes.get(id.as_str().unwrap()).unwrap_or_else(|| panic!("no name for {id}")).clone();
    let length = document["project"]["units"]["length"].as_str().unwrap();
    let mut positions = BTreeMap::new();
    for node in entries(document, "nodes") {
        let position = ["x", "y", "z"].map(|axis| to_si(node["position"][axis].as_f64().unwrap(), length));
        for (axis, value) in position.iter().enumerate() {
            out.values.insert(format!("node {} {axis}", name(&node["id"])), (*value, Bound::Coordinate));
        }
        positions.insert(node["id"].as_str().unwrap().to_string(), position);
    }
    let mut pipes = BTreeMap::new();
    for pipe in entries(document, "pipe_segments") {
        let mut ends = [name(&pipe["from"]), name(&pipe["to"])];
        if !oriented {
            ends.sort();
        }
        let key = format!("{}>{}", ends[0], ends[1]);
        let material = by_id(document, "materials", &pipe["material"]);
        let e = si(&material["elastic_modulus"]);
        // The writer's ν: the authored ν, or E/(2G) − 1 when only G was authored.
        let nu = if material["poisson_ratio"].is_object() { si(&material["poisson_ratio"]) } else { e / (2.0 * si(&material["shear_modulus"])) - 1.0 };
        let g = if material["shear_modulus"].is_object() { si(&material["shear_modulus"]) } else { e / (2.0 * (1.0 + nu)) };
        let alpha = si(&material["thermal_expansion_coefficient"]);
        for (field, value, bound) in [
            ("OD", si(&pipe["section"]["outside_diameter"]), Bound::Printed),
            ("t", si(&pipe["section"]["wall_thickness"]), Bound::Printed),
            ("E", e, Bound::Printed),
            ("nu", nu, Bound::Printed),
            ("G", g, Bound::Shear { e, nu }),
            ("alpha", alpha, Bound::Printed),
        ] {
            out.values.insert(format!("pipe {key} {field}"), (value, bound));
        }
        if oriented {
            out.sets.insert(format!("section_ref {key}"), pipe["section_ref"].to_string());
            if pipe["section_ref"].is_string() {
                let properties = &by_id(document, "sections", &pipe["section_ref"])["properties"];
                for (field, member) in [("OD", "outside_diameter"), ("t", "wall_thickness")] {
                    out.values.insert(format!("section {} {field}", pipe["section_ref"].as_str().unwrap()), (si(&properties[member]), Bound::Printed));
                }
            }
        }
        pipes.insert(pipe["id"].as_str().unwrap().to_string(), key);
    }
    let mut restrained: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for support in entries(document, "supports") {
        let node = name(&support["node"]);
        let dofs: BTreeSet<String> = entries(support, "restraints").iter().map(|dof| dof.as_str().unwrap().to_string()).collect();
        if oriented {
            out.sets.insert(format!("support {}", support["id"].as_str().unwrap()), format!("{node} {dofs:?}"));
        }
        if support["family"] == "spring" {
            let key = format!("spring {node} {}", support["stiffness"]["dof"].as_str().unwrap());
            let previous = out.values.insert(key.clone(), (si(&support["stiffness"]["value"]), Bound::Printed));
            assert!(previous.is_none(), "{key}: summed springs are outside this fixture's comparison");
            continue;
        }
        restrained.entry(node).or_default().extend(dofs);
    }
    for (node, dofs) in restrained {
        out.sets.insert(format!("restraints {node}"), format!("{dofs:?}"));
    }
    for component in entries(document, "components") {
        assert_eq!(component["kind"], "bend", "the fixture's only component family");
        let geometry = &component["geometry"];
        let pipe = by_id(document, "pipe_segments", &geometry["bend_pipe_ref"]);
        let key = &pipes[pipe["id"].as_str().unwrap()];
        let y = ["x", "y", "z"].map(|axis| pipe["y_reference"][axis].as_f64().unwrap());
        let (a, b) = (positions[pipe["from"].as_str().unwrap()], positions[pipe["to"].as_str().unwrap()]);
        out.planes.insert(format!("bend {key} plane"), project(y, a, b));
        out.values.insert(format!("bend {key} R"), (si(&geometry["bend_radius"]), Bound::Printed));
        let modifiers = &component["modifiers"];
        out.values.insert(format!("bend {key} k"), (si(&modifiers["flexibility_factor_user_value"]), Bound::Printed));
        for sif in ["sif_in_plane_user_value", "sif_out_of_plane_user_value"] {
            assert!(modifiers.get(sif).is_none(), "{sif} is outside this fixture's comparison");
        }
        out.sets.insert(format!("bend {key} node"), name(&component["node"]));
    }
    assert!(document.get("pressure_contract").is_none_or(Value::is_null), "pressure is outside this fixture's comparison");
    for case in entries(document, "load_cases") {
        let kind = &cases[case["id"].as_str().unwrap()];
        assert!(entries(case, "pressure_regions").is_empty(), "pressure is outside this fixture's comparison");
        for load in entries(case, "primitive_loads") {
            let target = &load["target"];
            let (key, bound) = match load["category"].as_str().unwrap() {
                "concentrated_force" => (format!("load {kind} force {} {}", name(&target["node"]), load["direction"].as_str().unwrap()), Bound::Printed),
                "thermal" => (format!("load {kind} thermal {}", pipes[target["pipe"].as_str().unwrap()]), Bound::Thermal),
                other => panic!("primitive category {other} is outside this fixture's comparison"),
            };
            let value = si(&load["magnitude"]);
            assert!(value != 0.0, "{key}: a zero primitive is outside this fixture's comparison");
            let previous = out.values.insert(key.clone(), (value, bound));
            assert!(previous.is_none(), "{key}: summed primitives are outside this fixture's comparison");
        }
    }
    out
}

/// The writer's own CAEPIPE names from the saved identity file: node ids to node names and case ids to case labels.
fn writer_names(sidecar: &[u8]) -> (BTreeMap<String, String>, BTreeMap<String, String>) {
    let sidecar: Value = serde_json::from_slice(sidecar).unwrap();
    let map = |member: &str| -> BTreeMap<String, String> {
        sidecar[member].as_object().unwrap_or_else(|| panic!("{member}")).iter().map(|(id, name)| (id.clone(), name.as_str().unwrap().to_string())).collect()
    };
    (map("node_map"), map("load_case_map"))
}

/// The adopted document's names: each entry's label (the import labels nodes by CAEPIPE name and cases by case).
fn labels(document: &Value, member: &str) -> BTreeMap<String, String> {
    entries(document, member).iter().map(|item| (item["id"].as_str().unwrap().to_string(), item["label"].as_str().unwrap().to_string())).collect()
}

/// The export model and the adopted document agree under §18.2's canonical form within its bounds. Returns the
/// number of compared scalars.
fn assert_canonical_parity(adopted: &Value, sidecar: &[u8], oriented: bool) -> usize {
    let source: Value = serde_json::from_str(EXPORT_MODEL).unwrap();
    let settings: Value = serde_json::from_str(EXPORT_SETTINGS).unwrap();
    assert_eq!(settings["referenceTemperature"]["unit"], "degC");
    let t_ref = settings["referenceTemperature"]["value"].as_f64().unwrap();
    let (nodes, cases) = writer_names(sidecar);
    let reference = canonical(&source, &nodes, &cases, oriented);
    let imported = canonical(adopted, &labels(adopted, "nodes"), &labels(adopted, "load_cases"), oriented);
    let largest = entries(&source, "nodes").iter().flat_map(|node| ["x", "y", "z"].map(|axis| node["position"][axis].as_f64().unwrap().abs())).fold(0.0, f64::max);
    assert_eq!(source["project"]["units"]["length"], "m");
    assert_eq!(reference.values.keys().collect::<Vec<_>>(), imported.values.keys().collect::<Vec<_>>(), "the same scalars are compared");
    for (key, (a, bound)) in &reference.values {
        let b = imported.values[key].0;
        let limit = match *bound {
            Bound::Printed => delta(*a) + 4.0 * EPS * a.abs(),
            Bound::Coordinate => 4.0 * EPS * largest,
            Bound::Shear { e, nu } => (delta(e) / e + delta(nu) / (1.0 + nu) + 8.0 * EPS) * a.abs(),
            Bound::Thermal => {
                let t1 = a + t_ref;
                delta(t1) + delta(t_ref) + 4.0 * EPS * t1.abs()
            }
        };
        assert!((a - b).abs() <= limit, "{key}: |{a:e} - {b:e}| > {limit:e}");
    }
    assert_eq!(reference.sets, imported.sets);
    // The bend plane: §18.2's bound is 2(b_TI + 2 b_tangent)/min(sagitta, chord) + 8ε plus residuals, and every b
    // includes 4εM; this takes only those terms.
    let plane_limit = entries(&source, "components")
        .iter()
        .map(|component| {
            let (r, theta) = (si(&component["geometry"]["bend_radius"]), si(&component["geometry"]["bend_angle"]));
            let chord = 2.0 * r * (theta / 2.0).sin();
            let sagitta = r * (1.0 / (theta / 2.0).cos() - (theta / 2.0).cos());
            2.0 * 3.0 * 4.0 * EPS * largest / chord.min(sagitta) + 8.0 * EPS
        })
        .fold(f64::INFINITY, f64::min);
    assert_eq!(reference.planes.keys().collect::<Vec<_>>(), imported.planes.keys().collect::<Vec<_>>());
    assert!(!reference.planes.is_empty());
    for (key, a) in &reference.planes {
        let b = imported.planes[key];
        let cross = [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]];
        let angle = norm(cross).min(1.0).asin();
        assert!((0..3).map(|axis| a[axis] * b[axis]).sum::<f64>() > 0.0 && angle <= plane_limit, "{key}: {angle:e} rad > {plane_limit:e}");
    }
    let families = |keys: Vec<&String>| keys.into_iter().map(|key| key.split(' ').next().unwrap().to_string()).collect::<BTreeSet<_>>();
    let expected: &[&str] = if oriented { &["bend", "load", "node", "pipe", "section", "spring"] } else { &["bend", "load", "node", "pipe", "spring"] };
    assert_eq!(families(reference.values.keys().collect()), expected.iter().map(|family| family.to_string()).collect::<BTreeSet<_>>(), "every family is compared");
    reference.values.len()
}

// -- The connected path ---------------------------------------------------------------------------------------------

/// Bound: the saved pair (exactly the writer's bytes for the export model, and nothing else in the directory) opens,
/// its review is complete, within its caps, adoptable and bound, with the source's identity inventory restored class
/// by class (primitive loads in full; no pressure region on either side), and Adopt returns the engine's import of
/// those bytes as the `NativeModelDocumentOutcome`, whose model digest is the review's, whose ids are the exported
/// model's for every class the export writes, and which equals the export model under §18.2's canonical comparison.
/// The result and reply leases are held while each response is built; nothing is held between commands; the
/// retirement drops the source, and Review and Adopt are then refused retired.
#[test]
fn a_bound_saved_pair_imports_through_task_5_with_the_exported_ids() {
    let registry = real_worker();
    let imports = MbfImports::default();
    let sandbox = Sandbox::new("bound");
    let mbf_path = sandbox.native_pair(&registry);
    let (text, sidecar) = (fs::read(&mbf_path).unwrap(), fs::read(sandbox.ids()).unwrap());
    let (written_text, written_sidecar) = written_pair();
    assert!(text == written_text && sidecar == written_sidecar, "the saved pair is the writer's bytes for the export model");
    let mut saved: Vec<String> = fs::read_dir(&sandbox.0).unwrap().map(|entry| entry.unwrap().file_name().into_string().unwrap()).collect();
    saved.sort();
    assert_eq!(saved, ["swbpipe-model.mbf", "swbpipe-model.swbpipe-ids.json"], "the save leaves exactly the pair");
    let opened = open(&registry, &imports, &mbf_path);
    assert!(opened.held_during_response > 0, "the result lease is held while the response is built");
    assert_released(&registry);
    let reply = &opened.value;
    assert_eq!(reply["outcome"], "reviewed", "{reply}");
    assert_eq!(reply["fileName"], "swbpipe-model.mbf");
    let source_id = reply["sourceId"].as_u64().unwrap();
    assert_eq!(imports.source_id(), Some(source_id));
    let review_text = serde_json::to_string(&reply["review"]).unwrap();
    assert_eq!(reply["reviewSha256"].as_str().unwrap().len(), 64);
    assert_eq!(reply["review"]["adoptable"], true, "{review_text}");
    assert_eq!(reply["review"]["binding"]["state"], "bound");
    assert_eq!(reply["review"]["source"]["mbf_sha256"], hex(&text));
    assert_eq!(reply["review"]["source"]["mbf_bytes"], text.len());
    assert_eq!(reply["review"]["source"]["identity_file"]["state"], "present");
    assert_eq!(reply["review"]["source"]["identity_file"]["sha256"], hex(&sidecar));
    assert_eq!(reply["review"]["source"]["identity_file"]["bytes"], sidecar.len());
    assert!(!review_text.contains(sandbox.0.to_str().unwrap()), "no path reaches the webview");
    assert_complete(&reply["review"]);
    assert!(reply["review"]["blocks"].as_object().unwrap().values().all(|blocked| *blocked == false), "{review_text}");
    assert_eq!(reply["review"]["status"], "imported");
    assert_eq!(reply["review"]["refusals"], json!([]));
    assert_eq!(reply["review"]["declaration_needed"], json!([]));
    assert_eq!(reply["review"]["transfer_limit"], "within");
    assert_eq!(reply["review"]["identity"]["basis"], "restored_from_identity_file");
    let exported: Value = serde_json::from_str(EXPORT_MODEL).unwrap();
    let expected_classes: BTreeMap<_, _> = source_identity_counts(&exported).into_iter().map(|(class, n)| (class, (n, n, 0))).collect();
    assert_eq!(identity_classes(&reply["review"]), expected_classes, "every class of the source is restored, none formed");
    assert_eq!(expected_classes["load"].0, 5);
    assert_eq!(expected_classes["pressure_region"].0, 0, "the supported subset has no pressure region");
    let expected_model = engine_model(&text, Some(&sidecar), None);
    assert_eq!(reply["review"]["model_sha256"], hex(&expected_model));
    assert_eq!(reply["review"]["model_bytes"], expected_model.len());
    assert!(expected_model.len() as u64 <= MBF_IMPORT_RESULT_MAX_BYTES);
    // Adopt.
    let adopted = adopt(&registry, &imports, &json!({"sourceId": source_id, "declaration": null, "reviewSha256": reply["reviewSha256"]}));
    assert!(adopted.held_during_response > 0);
    let opened_doc = &adopted.value;
    assert_eq!(opened_doc["outcome"], "opened", "{opened_doc}");
    assert_eq!(opened_doc["file_name"], "swbpipe-model.mbf");
    assert_eq!(opened_doc["schema_status"], "current");
    assert_eq!(opened_doc["warnings"], json!([]));
    assert_eq!(opened_doc["reviewSha256"], reply["reviewSha256"]);
    assert_eq!(opened_doc["modelSha256"], hex(&expected_model));
    assert_eq!(opened_doc["byte_count"], expected_model.len());
    assert_eq!(opened_doc["document"], serde_json::from_slice::<Value>(&expected_model).unwrap(), "the adopted document is the engine's import");
    for member in ["nodes", "pipe_segments", "components", "supports", "load_cases", "materials", "sections"] {
        let restored = ids_of(&opened_doc["document"], member);
        assert!(!restored.is_empty(), "{member}");
        assert_eq!(restored, ids_of(&exported, member), "{member} ids are restored");
    }
    let loads = case_inventory(&exported, "primitive_loads");
    assert_eq!(loads.len(), 5);
    assert_eq!(case_inventory(&opened_doc["document"], "primitive_loads"), loads, "every primitive load keeps its id and case");
    assert_eq!(case_inventory(&exported, "pressure_regions"), BTreeSet::new());
    assert_eq!(case_inventory(&opened_doc["document"], "pressure_regions"), BTreeSet::new(), "no pressure region appears");
    assert!(exported.get("pressure_contract").is_none() && opened_doc["document"].get("pressure_contract").is_none());
    assert_ne!(opened_doc["document"]["project"]["id"], exported["project"]["id"], "an import is a new project");
    assert_eq!(assert_canonical_parity(&opened_doc["document"], &sidecar, true), 51);
    assert_released(&registry);
    // The keyed retirement drops the source; a later re-read or Adopt is refused retired, with no model.
    let retired = retire(&registry, &imports, 1, 1);
    assert_eq!((retired.cancelled, retired.duplicate), (0, false));
    assert_eq!(imports.source_id(), None);
    let after = review(&registry, &imports, &json!({"sourceId": source_id, "declaration": null}));
    assert_eq!(codes(&after.value), vec![SOURCE_RETIRED]);
    let late = adopt(&registry, &imports, &json!({"sourceId": source_id, "declaration": null, "reviewSha256": reply["reviewSha256"]})).value;
    assert_eq!(codes(&late), vec![SOURCE_RETIRED]);
    assert!(late.get("document").is_none());
    assert_released(&registry);
}

/// Unbound: the same MBF without its identity file reviews, completely, as needing a declaration per listed material;
/// the declared re-run is complete and adoptable, its ids are formed from MBF names for every class the source has
/// (no section is restored and no pressure region formed), and the adopted document is the engine's import with that
/// declaration and equals the export model under §18.2's comparison by CAEPIPE name. An identity file that appears
/// afterwards is a change. The refused source stays in custody until its retirement, after which Review and Adopt
/// are refused retired, even with the files as they were chosen.
#[test]
fn an_unbound_mbf_needs_its_declaration_and_imports_with_ids_formed_from_names() {
    let registry = real_worker();
    let imports = MbfImports::default();
    let sandbox = Sandbox::new("unbound");
    let mbf_path = sandbox.native_pair(&registry);
    let (text, sidecar) = (fs::read(&mbf_path).unwrap(), fs::read(sandbox.ids()).unwrap());
    fs::remove_file(sandbox.ids()).unwrap();
    let first = open(&registry, &imports, &mbf_path).value;
    assert_eq!(first["outcome"], "reviewed");
    assert_eq!(first["review"]["adoptable"], false);
    assert_eq!(first["review"]["binding"]["reason"], "no_identity_file");
    assert_eq!(first["review"]["source"]["mbf_sha256"], hex(&text));
    assert_eq!(first["review"]["source"]["identity_file"]["state"], "absent");
    assert_complete(&first["review"]);
    let needed = first["review"]["declaration_needed"].as_array().unwrap().clone();
    assert!(!needed.is_empty());
    let source_id = first["sourceId"].clone();
    // Adopt without the declaration is refused with the review's blocking code and no document.
    let refused = adopt(&registry, &imports, &json!({"sourceId": source_id, "declaration": null, "reviewSha256": first["reviewSha256"]})).value;
    assert_eq!(codes(&refused), vec![NOT_ADOPTABLE]);
    assert!(refused.get("document").is_none());
    let declaration = declaration_for(&needed);
    let declared = review(&registry, &imports, &json!({"sourceId": source_id, "declaration": declaration})).value;
    assert_eq!(declared["review"]["adoptable"], true, "{declared}");
    assert_eq!(declared["review"]["identity"]["basis"], "formed_from_mbf_names");
    assert_ne!(declared["reviewSha256"], first["reviewSha256"], "the declaration is bound into the review");
    assert_complete(&declared["review"]);
    assert!(declared["review"]["blocks"].as_object().unwrap().values().all(|blocked| *blocked == false), "{declared}");
    let exported: Value = serde_json::from_str(EXPORT_MODEL).unwrap();
    let mut expected_classes: BTreeMap<_, _> = source_identity_counts(&exported).into_iter().map(|(class, n)| (class, (n, 0, n))).collect();
    expected_classes.insert("section".into(), (0, 0, 0)); // an unbound import restores no library section
    assert_eq!(identity_classes(&declared["review"]), expected_classes, "every other class of the source is formed from MBF names");
    let adopted = adopt(&registry, &imports, &json!({"sourceId": source_id, "declaration": declaration, "reviewSha256": declared["reviewSha256"]})).value;
    assert_eq!(adopted["outcome"], "opened", "{adopted}");
    let declaration_bytes = serde_json::to_vec(&declaration).unwrap();
    assert_eq!(adopted["document"], serde_json::from_slice::<Value>(&engine_model(&text, None, Some(&declaration_bytes))).unwrap());
    assert!(ids_of(&adopted["document"], "nodes").iter().all(|id| id.starts_with("node:")));
    assert_eq!(case_inventory(&adopted["document"], "primitive_loads").len(), case_inventory(&exported, "primitive_loads").len());
    assert_eq!(case_inventory(&adopted["document"], "pressure_regions"), BTreeSet::new(), "no pressure region appears");
    assert_eq!(assert_canonical_parity(&adopted["document"], &sidecar, false), 47);
    // An identity file that appears beside the originally absent one is a change.
    fs::write(sandbox.ids(), sidecar).unwrap();
    let changed_reply = review(&registry, &imports, &json!({"sourceId": source_id, "declaration": declaration})).value;
    assert_eq!(codes(&changed_reply), vec![SOURCE_CHANGED]);
    // The refusals keep the source in custody until its retirement, which drops it for Review and Adopt alike, even
    // once the files are back as they were chosen.
    assert_eq!(imports.source_id(), source_id.as_u64());
    let retired = retire(&registry, &imports, 1, 1);
    assert_eq!((retired.cancelled, retired.duplicate), (0, false));
    assert_eq!(imports.source_id(), None);
    fs::remove_file(sandbox.ids()).unwrap();
    let after = review(&registry, &imports, &json!({"sourceId": source_id, "declaration": declaration})).value;
    assert_eq!(codes(&after), vec![SOURCE_RETIRED]);
    let late = adopt(&registry, &imports, &json!({"sourceId": source_id, "declaration": declaration, "reviewSha256": declared["reviewSha256"]})).value;
    assert_eq!(codes(&late), vec![SOURCE_RETIRED]);
    assert!(late.get("document").is_none());
    assert_released(&registry);
}

// -- The re-read ----------------------------------------------------------------------------------------------------

/// An unchanged re-read reviews byte for byte as the first; each change after selection is refused
/// `MBF-IMPORT-SOURCE-CHANGED`, and stays refused until a new Open.
#[test]
fn every_change_after_selection_is_refused_and_an_unchanged_reread_is_identical() {
    let registry = real_worker();
    type Change = fn(&Sandbox, &Path);
    let cases: [(&str, bool, Change); 6] = [
        ("an in-place edit", true, |_, mbf| {
            let mut bytes = fs::read(mbf).unwrap();
            let at = bytes.iter().position(|byte| byte.is_ascii_digit()).unwrap();
            bytes[at] = if bytes[at] == b'9' { b'8' } else { bytes[at] + 1 };
            let file = fs::OpenOptions::new().write(true).open(mbf).unwrap();
            std::os::unix::fs::FileExt::write_all_at(&file, &bytes, 0).unwrap();
        }),
        ("a replacement at the name", true, |sandbox, mbf| {
            let other = sandbox.0.join("other.mbf");
            fs::write(&other, fs::read(mbf).unwrap()).unwrap();
            fs::rename(other, mbf).unwrap();
        }),
        ("a removal", true, |_, mbf| fs::remove_file(mbf).unwrap()),
        ("an identity file removed", true, |sandbox, _| fs::remove_file(sandbox.ids()).unwrap()),
        ("an identity file edited", true, |sandbox, _| {
            let mut bytes = fs::read(sandbox.ids()).unwrap();
            bytes.push(b'\n');
            fs::write(sandbox.ids(), bytes).unwrap();
        }),
        ("a symlink at the name", false, |sandbox, mbf| {
            let target = sandbox.0.join("target.mbf");
            fs::rename(mbf, &target).unwrap();
            std::os::unix::fs::symlink(&target, mbf).unwrap();
        }),
    ];
    for (label, with_identity, change) in cases {
        let imports = MbfImports::default();
        let sandbox = Sandbox::new("changes");
        let mbf_path = sandbox.pair(with_identity);
        let first = open(&registry, &imports, &mbf_path).value;
        let source_id = first["sourceId"].clone();
        let again = review(&registry, &imports, &json!({"sourceId": source_id, "declaration": null})).value;
        assert_eq!(again["review"], first["review"], "{label}: an unchanged re-read is identical");
        assert_eq!(again["reviewSha256"], first["reviewSha256"]);
        change(&sandbox, &mbf_path);
        for _ in 0..2 {
            let reply = review(&registry, &imports, &json!({"sourceId": source_id, "declaration": null})).value;
            assert_eq!(codes(&reply), vec![SOURCE_CHANGED], "{label}");
        }
        let adopted = adopt(&registry, &imports, &json!({"sourceId": source_id, "declaration": null, "reviewSha256": first["reviewSha256"]})).value;
        assert_eq!(codes(&adopted), vec![SOURCE_CHANGED], "{label}");
        assert!(adopted.get("document").is_none());
    }
    assert_released(&registry);
}

/// Growth during the read (after the last block, before the end-of-file check) is a change, not a longer import.
#[test]
fn growth_during_the_reread_is_refused_by_the_end_of_file_check() {
    let registry = real_worker();
    let imports = MbfImports::default();
    let sandbox = Sandbox::new("growth");
    let mbf_path = sandbox.pair(true);
    let first = open(&registry, &imports, &mbf_path).value;
    let grown = AtomicBool::new(false);
    let size = fs::metadata(&mbf_path).unwrap().len();
    let target = mbf_path.clone();
    let grown_ref: &'static AtomicBool = Box::leak(Box::new(grown));
    test_hooks::AFTER_BLOCK.with(|hook| {
        *hook.borrow_mut() = Some(Box::new(move |path: &Path, offset: u64| {
            if path == target && offset == size && !grown_ref.swap(true, Ordering::SeqCst) {
                let mut file = fs::OpenOptions::new().append(true).open(path).unwrap();
                std::io::Write::write_all(&mut file, b"\n").unwrap();
            }
        }));
    });
    // The name check passes (same size) before the read; the growth happens during it.
    let reply = review(&registry, &imports, &json!({"sourceId": first["sourceId"], "declaration": null})).value;
    test_hooks::AFTER_BLOCK.with(|hook| *hook.borrow_mut() = None);
    assert!(grown_ref.load(Ordering::SeqCst));
    assert_eq!(codes(&reply), vec![SOURCE_CHANGED]);
    assert_released(&registry);
}

// -- Selection --------------------------------------------------------------------------------------------------------

fn mkfifo(path: &Path) {
    let status = std::process::Command::new("mkfifo").arg(path).status().unwrap();
    assert!(status.success());
}

/// Each non-regular, oversized or unreadable choice is refused by name before any worker, and a present identity file
/// that fails is refused, never set aside; no source is installed.
#[test]
fn non_regular_oversized_and_unreadable_choices_are_refused_by_name() {
    use std::os::unix::fs::PermissionsExt;
    let registry = real_worker();
    type Setup = fn(&Sandbox) -> PathBuf;
    let cases: [(&str, &str, Setup); 8] = [
        ("a symlinked MBF", OPEN_NOT_A_FILE, |sandbox| {
            let target = sandbox.pair(false);
            let link = sandbox.0.join("link.mbf");
            std::os::unix::fs::symlink(target, &link).unwrap();
            link
        }),
        ("a directory", OPEN_NOT_A_FILE, |sandbox| {
            let dir = sandbox.0.join("dir.mbf");
            fs::create_dir(&dir).unwrap();
            dir
        }),
        ("a FIFO", OPEN_NOT_A_FILE, |sandbox| {
            let fifo = sandbox.0.join("fifo.mbf");
            mkfifo(&fifo);
            fifo
        }),
        ("an MBF of 16 MiB + 1", OPEN_TOO_LARGE, |sandbox| {
            let big = sandbox.0.join("big.mbf");
            fs::File::create(&big).unwrap().set_len(MBF_MAX_BYTES + 1).unwrap();
            big
        }),
        ("a symlinked identity file", OPEN_NOT_A_FILE, |sandbox| {
            let mbf_path = sandbox.pair(true);
            let target = sandbox.0.join("ids-target.json");
            fs::rename(sandbox.ids(), &target).unwrap();
            std::os::unix::fs::symlink(target, sandbox.ids()).unwrap();
            mbf_path
        }),
        ("an identity directory", OPEN_NOT_A_FILE, |sandbox| {
            let mbf_path = sandbox.pair(false);
            fs::create_dir(sandbox.ids()).unwrap();
            mbf_path
        }),
        ("an identity file above 64 MiB", OPEN_TOO_LARGE, |sandbox| {
            let mbf_path = sandbox.pair(false);
            fs::File::create(sandbox.ids()).unwrap().set_len(IDENTITY_MAX_BYTES + 1).unwrap();
            mbf_path
        }),
        ("an unreadable identity file", OPEN_UNREADABLE, |sandbox| {
            let mbf_path = sandbox.pair(true);
            fs::set_permissions(sandbox.ids(), fs::Permissions::from_mode(0o000)).unwrap();
            mbf_path
        }),
    ];
    for (label, code, setup) in cases {
        let imports = MbfImports::default();
        let sandbox = Sandbox::new("selection");
        let chosen = setup(&sandbox);
        if code == OPEN_UNREADABLE && fs::File::open(sandbox.ids()).is_ok() {
            continue; // running with privileges that read a mode-000 file
        }
        let reply = open(&registry, &imports, &chosen).value;
        assert_eq!(codes(&reply), vec![code], "{label}: {reply}");
        assert_eq!(imports.source_id(), None, "{label}");
        assert_released(&registry);
    }
}

/// Run the actual acquisition race in a child owned by the bounded parent test below. The parent owns the sandbox
/// too, so killing a regressed blocking open closes its descriptors and still permits deterministic cleanup.
#[cfg(unix)]
#[test]
#[ignore = "invoked in a bounded subprocess by the FIFO acquisition regression"]
fn fifo_substitution_probe() {
    let mbf_path = PathBuf::from(std::env::var_os("SWBPIPE_MBF_FIFO_PROBE_PATH").expect("probe path"));
    let identity = std::env::var_os("SWBPIPE_MBF_FIFO_PROBE_IDENTITY").is_some();
    let swapped = if identity { mbf_path.with_file_name("swbpipe-model.swbpipe-ids.json") } else { mbf_path.clone() };
    let registry = real_worker();
    let imports = MbfImports::default();
    test_hooks::BEFORE_OPEN.with(|hook| {
        *hook.borrow_mut() = Some(Box::new(move |path: &Path| {
            if path == swapped {
                fs::remove_file(path).unwrap();
                mkfifo(path);
            }
        }));
    });
    let reply = open(&registry, &imports, &mbf_path).value;
    test_hooks::BEFORE_OPEN.with(|hook| *hook.borrow_mut() = None);
    assert_eq!(codes(&reply), vec![OPEN_NOT_A_FILE], "identity: {identity}");
    assert_eq!(imports.source_id(), None);
    assert_released(&registry);
}

/// The acquisition race for both files. A blocking-open regression is killed and reaped within the deadline,
/// then fails this test, rather than hanging the whole native suite. A cancellation at the seam installs no source.
#[cfg(unix)]
#[test]
fn a_fifo_substituted_before_the_open_is_refused_without_waiting_and_a_cancel_there_installs_nothing() {
    for identity in [false, true] {
        let sandbox = Sandbox::new("fifo-race");
        let mbf_path = sandbox.pair(true);
        let mut command = std::process::Command::new(std::env::current_exe().unwrap());
        command.args(["--ignored", "--exact", "mbf_import::tests::fifo_substitution_probe"])
            .env("SWBPIPE_MBF_FIFO_PROBE_PATH", &mbf_path);
        if identity { command.env("SWBPIPE_MBF_FIFO_PROBE_IDENTITY", "1"); }
        else { command.env_remove("SWBPIPE_MBF_FIFO_PROBE_IDENTITY"); }
        let mut child = command.spawn().expect("FIFO probe subprocess");
        let deadline = Instant::now() + Duration::from_secs(20);
        loop {
            if let Some(status) = child.try_wait().unwrap() {
                assert!(status.success(), "FIFO acquisition probe failed, identity: {identity}");
                break;
            }
            if Instant::now() >= deadline {
                let _ = child.kill();
                child.wait().expect("reap the blocked FIFO probe");
                panic!("FIFO acquisition blocked past its deadline, identity: {identity}");
            }
            std::thread::sleep(Duration::from_millis(5));
        }
    }
    // A retirement at the seam.
    let imports = MbfImports::default();
    let sandbox = Sandbox::new("fifo-cancel");
    let mbf_path = sandbox.pair(true);
    let (registry_ref, imports_ref): (&'static SolveJobRegistry, &'static MbfImports) = (Box::leak(Box::new(real_worker())), Box::leak(Box::new(imports.clone())));
    test_hooks::BEFORE_OPEN.with(|hook| {
        *hook.borrow_mut() = Some(Box::new(move |_: &Path| {
            retire(registry_ref, imports_ref, 9, 1);
        }));
    });
    let reply = open(registry_ref, &imports, &mbf_path).value;
    test_hooks::BEFORE_OPEN.with(|hook| *hook.borrow_mut() = None);
    assert_eq!(codes(&reply), vec![CANCELLED]);
    assert_eq!(imports.source_id(), None);
    assert_released(registry_ref);
}

/// The chooser held open while the import is retired: the run installs no source and replies cancelled; a dismissed
/// chooser replies `cancelled` and touches nothing.
#[test]
fn a_retirement_while_the_chooser_is_open_installs_no_source() {
    let registry = real_worker();
    let imports = MbfImports::default();
    let sandbox = Sandbox::new("chooser");
    let mbf_path = sandbox.pair(true);
    let reply = open_with(&registry, &imports, &json!({}), || {
        retire(&registry, &imports, 2, 1);
        Ok(Some(mbf_path.clone()))
    })
    .value;
    assert_eq!(codes(&reply), vec![CANCELLED]);
    assert_eq!(imports.source_id(), None);
    let dismissed = open_with(&registry, &imports, &json!({}), || Ok(None)).value;
    assert_eq!(dismissed, json!({"outcome": "cancelled"}));
    assert_released(&registry);
}

#[test]
fn a_retirement_before_registration_opens_no_chooser() {
    let registry = real_worker();
    let imports = MbfImports::default();
    let (r, state) = (registry.clone(), imports.clone());
    test_hooks::BEFORE_REGISTRATION.with(|hook| {
        *hook.borrow_mut() = Some(Box::new(move || { retire(&r, &state, 12, 1); }));
    });
    let reply = open_with(&registry, &imports, &json!({}), || panic!("a cancelled start must not open a chooser")).value;
    assert_eq!(codes(&reply), vec![CANCELLED]);
    assert_eq!(imports.source_id(), None);
    assert_released(&registry);
}

#[test]
fn an_older_dispatch_cannot_register_after_a_newer_one_or_replay_itself() {
    let registry = real_worker();
    let imports = MbfImports::default();
    let (r, state) = (registry.clone(), imports.clone());
    test_hooks::BEFORE_REGISTRATION.with(|hook| {
        *hook.borrow_mut() = Some(Box::new(move || {
            assert_eq!(open_with(&r, &state, &json!({REQUEST_SEQUENCE: 2}), || Ok(None)).value["outcome"], "cancelled");
        }));
    });
    let older = open_with(&registry, &imports, &json!({REQUEST_SEQUENCE: 1}), || panic!("older dispatch must not open a chooser")).value;
    assert_eq!(codes(&older), vec![CANCELLED]);
    let replay = open_with(&registry, &imports, &json!({REQUEST_SEQUENCE: 2}), || panic!("duplicate dispatch must not open a chooser")).value;
    assert_eq!(codes(&replay), vec![CANCELLED]);
    for invalid in [json!(null), json!(0), json!(-1), json!("absent"), json!("1"), json!(1.5), json!(1u64 << 53)] {
        let reply = open_with(&registry, &imports, &json!({REQUEST_SEQUENCE: invalid}), || panic!("invalid sequence must not choose")).value;
        assert_eq!(codes(&reply), vec![ARGS_INVALID]);
    }
    retire(&registry, &imports, 15, 1);
    assert_eq!(open_with(&registry, &imports, &json!({REQUEST_SEQUENCE: 1}), || Ok(None)).value["outcome"], "cancelled", "a new epoch has its own sequence space");
    assert_released(&registry);
}

#[test]
fn retirement_during_h7_admission_returns_no_model() {
    let registry = real_worker();
    let imports = MbfImports::default();
    let sandbox = Sandbox::new("cancel-reply");
    let first = open(&registry, &imports, &sandbox.pair(true)).value;
    let body = json!({"sourceId": first["sourceId"], "declaration": null, "reviewSha256": first["reviewSha256"]});
    let (waiting_tx, waiting_rx) = std::sync::mpsc::sync_channel(0);
    let (resume_tx, resume_rx) = std::sync::mpsc::sync_channel(0);
    std::thread::scope(|scope| {
        let run = scope.spawn(|| {
            crate::stored_read::native_lease_log::take();
            let mut blocked = false;
            crate::memory_pool::ledger_probe::set(move |_, _| {
                if !blocked && crate::stored_read::native_lease_log::last().1.is_some_and(|(what, _)| what == "the import's reply") {
                    blocked = true;
                    waiting_tx.send(()).unwrap();
                    resume_rx.recv_timeout(Duration::from_secs(20)).unwrap();
                }
            });
            let reply = adopt(&registry, &imports, &body).value;
            crate::memory_pool::ledger_probe::clear();
            reply
        });
        waiting_rx.recv_timeout(Duration::from_secs(20)).expect("Adopt reached H7 admission");
        assert_eq!(retire(&registry, &imports, 13, 1).cancelled, 1);
        resume_tx.send(()).unwrap();
        let reply = run.join().unwrap();
        assert_eq!(codes(&reply), vec![CANCELLED]);
        assert!(reply.get("document").is_none(), "an acknowledged cancellation wins before handoff");
    });
    assert_released(&registry);
}

#[test]
fn a_refused_h7_admission_hands_off_before_the_response_callback() {
    for already_cancelled in [false, true] {
        let registry = real_worker();
        registry.configure_pool(12 << 30, 1, false).unwrap();
        let memory = NativeLeaseMemory::unseeded(&registry);
        let imports = MbfImports::default();
        let entry = imports.enter(Some(json!(imports.epoch())), Some(json!(1))).unwrap();
        let registration = imports.register(entry, &registry);
        if already_cancelled {
            assert_eq!(retire(&registry, &imports, 15, 1).cancelled, 1);
        }
        let mut held = Held::default();
        crate::stored_read::native_lease_log::take();
        let reply = Reply::Refused {
            adopt: true,
            file_name: "invented.mbf".to_string(),
            diagnostics: vec![diag(REVIEW_STALE, "", "The review changed.")],
        };
        let reply: Value = respond_with(&memory, reply, &mut held, Some(&registration.control), |text| {
            let (_, attempted) = crate::stored_read::native_lease_log::last();
            let (what, bytes) = attempted.expect("H7 admission attempted");
            assert_eq!(what, "the import's reply");
            assert!(bytes > 1, "the captured one-byte budget forces H7 refusal");
            assert_eq!(retire(&registry, &imports, 15, 2).cancelled, 0, "the terminal refusal was already handed off");
            serde_json::from_str(&text).unwrap()
        });
        assert!(held.reply.is_none(), "H7 was not admitted");
        assert_eq!(codes(&reply), vec![if already_cancelled { CANCELLED } else { MEMORY_UNAVAILABLE }]);
        assert!(reply.get("document").is_none());
        drop(held);
        drop(registration);
        assert_released(&registry);
    }
}

#[test]
fn the_response_holds_h6_and_h7_separately_through_handoff() {
    let registry = real_worker();
    let imports = MbfImports::default();
    let sandbox = Sandbox::new("response-custody");
    let first = open(&registry, &imports, &sandbox.pair(true)).value;
    let body = ticketed(&imports, &json!({"sourceId": first["sourceId"], "declaration": null, "reviewSha256": first["reviewSha256"]}));
    let (raw_args, copy) = reserve(&registry, &raw(&body), &headers(Some(INGRESS_ARGS_JSON_V1)), ADOPT_COMMAND).unwrap();
    crate::stored_read::native_lease_log::take();
    let snapshot = Arc::new(Mutex::new(None));
    let captured = Arc::clone(&snapshot);
    let pool = Arc::clone(registry.pool());
    crate::memory_pool::ledger_probe::set(move |_, bytes| {
        if crate::stored_read::native_lease_log::last().1.is_some_and(|(what, _)| what == "the import's reply") {
            *captured.lock().unwrap() = Some((pool.reserved(LeasePurpose::Ingress), pool.reserved(LeasePurpose::Result), bytes));
        }
    });
    let reply: Value = adopt_command(&registry, &imports, raw_args, copy, START_WAIT, |text| {
        let (prior_ingress, result, h7) = snapshot.lock().unwrap().expect("H7 admission observed");
        assert!(result > 0, "H6 is a distinct live result lease, not H1");
        assert_eq!(registry.pool().reserved(LeasePurpose::Result), result, "H6 survives through response construction");
        assert_eq!(registry.pool().reserved(LeasePurpose::Ingress), prior_ingress + h7, "H7 survives in addition to H1/H2/H8");
        // A retirement after final response preparation must not claim it stopped this completed command.
        assert_eq!(retire(&registry, &imports, 14, 1).cancelled, 0);
        serde_json::from_str(&text).unwrap()
    }).unwrap();
    crate::memory_pool::ledger_probe::clear();
    assert_eq!(reply["outcome"], "opened");
    assert_released(&registry);
}

// -- Arguments, tickets and retirement --------------------------------------------------------------------------------

/// No caller path: a `path` or other extra member is refused before anything else, and no chooser or file is opened.
/// A missing, null, malformed, future or stale ticket is refused before any run; a bad declaration is refused by name.
#[test]
fn unknown_members_tickets_and_declarations_are_refused_before_any_file() {
    let registry = real_worker();
    let imports = MbfImports::default();
    let chose = AtomicUsize::new(0);
    for body in [json!({"path": "/tmp/x.mbf"}), json!({"expectedEpoch": 0, "file": "x"})] {
        let reply = open_with(&registry, &imports, &body, || {
            chose.fetch_add(1, Ordering::SeqCst);
            Ok(None)
        })
        .value;
        assert_eq!(codes(&reply), vec![ARGS_INVALID]);
    }
    for (ticket, code) in [(json!("absent"), EPOCH_INVALID), (Value::Null, EPOCH_INVALID), (json!("1"), EPOCH_INVALID), (json!(5), EPOCH_INVALID)] {
        let reply = open_with(&registry, &imports, &json!({ EXPECTED_EPOCH: ticket }), || {
            chose.fetch_add(1, Ordering::SeqCst);
            Ok(None)
        })
        .value;
        assert_eq!(codes(&reply), vec![code]);
    }
    retire(&registry, &imports, 1, 1);
    let stale = open_with(&registry, &imports, &json!({ EXPECTED_EPOCH: 0 }), || {
        chose.fetch_add(1, Ordering::SeqCst);
        Ok(None)
    })
    .value;
    assert_eq!(codes(&stale), vec![CANCELLED]);
    assert_eq!(chose.load(Ordering::SeqCst), 0, "no chooser opened");
    // Declarations: shape refusals, the wildcard included.
    let sandbox = Sandbox::new("declarations");
    let first = open(&registry, &imports, &sandbox.pair(false)).value;
    let source_id = first["sourceId"].clone();
    for declaration in [
        json!([]),
        json!({"*": {"origin": "project_invented", "source_name": "all"}}),
        json!({"CS1": {"origin": "vendor_library", "source_name": "x"}}),
        json!({"CS1": {"origin": "unknown", "source_name": "x"}}),
        json!({"CS1": {"origin": "user_supplied", "source_name": " "}}),
        json!({"CS1": {"origin": "user_supplied", "source_name": "x", "redistribution_status": "public_permissive"}}),
        json!({"CS1": {"origin": "user_supplied", "source_name": "n".repeat(513)}}),
    ] {
        let reply = review(&registry, &imports, &json!({"sourceId": source_id, "declaration": declaration})).value;
        assert_eq!(codes(&reply), vec![DECLARATION_INVALID], "{declaration}");
    }
    let missing = review(&registry, &imports, &json!({"sourceId": source_id})).value;
    assert_eq!(codes(&missing), vec![ARGS_INVALID]);
    for bad in [json!(0), json!(-1), json!("1")] {
        assert_eq!(codes(&review(&registry, &imports, &json!({"sourceId": bad, "declaration": null})).value), vec![ARGS_INVALID]);
    }
    // A raw body above the start's bound is an ingress refusal before any lease.
    let large = json!({ EXPECTED_EPOCH: 1, "pad": "x".repeat(OPEN_ARGS_MAX_BYTES) });
    assert!(matches!(reserve(&registry, &raw(&large), &headers(Some(INGRESS_ARGS_JSON_V1)), OPEN_COMMAND), Err(SolveJobError::InvalidRequest { message }) if message.starts_with("INGRESS-ARGS-TOO-LARGE")));
    // A JSON body (Tauri's fallback) is refused `INGRESS-RAW-BODY-REQUIRED`.
    assert!(matches!(reserve(&registry, &InvokeBody::Json(json!({})), &headers(Some(INGRESS_ARGS_JSON_V1)), REVIEW_COMMAND), Err(SolveJobError::InvalidRequest { message }) if message.starts_with("INGRESS-RAW-BODY-REQUIRED")));
    assert_released(&registry);
}

/// Keyed retirement: the JSON and raw forms are one path (an id delivered both ways is applied once); a duplicate is a
/// no-op; 64 page tokens are kept and the 65th exhausts the import; the epoch does not wrap; a source-id overflow
/// exhausts. An import retirement leaves the export's runs alone, and the export's retirement leaves the import alone.
#[test]
fn keyed_retirements_apply_once_exhaust_truthfully_and_stay_apart_from_the_export() {
    let registry = real_worker();
    let imports = MbfImports::default();
    let first = retire(&registry, &imports, 1, 1);
    assert_eq!((first.epoch, first.duplicate, first.exhausted), (1, false, false));
    let raw_form = json!({ RETIREMENT_ID: { "page": page(1), "sequence": 1 } });
    let again = cancel_command(&registry, &imports, &raw(&raw_form), &headers(Some(INGRESS_ARGS_JSON_V1))).unwrap();
    assert!(again.duplicate && again.epoch == 1, "the raw form of the same id is a duplicate");
    assert!(retire(&registry, &imports, 1, 1).duplicate);
    let malformed = cancel_command(&registry, &imports, &InvokeBody::Json(json!({ RETIREMENT_ID: {"page": "x", "sequence": 1} })), &headers(None));
    assert!(matches!(malformed, Err(CommandError::Command(message)) if message.starts_with(RETIREMENT_ID_INVALID)));
    for n in 2..=RETIREMENT_PAGES_MAX {
        assert!(!retire(&registry, &imports, n, 1).exhausted);
    }
    let exhausted = retire(&registry, &imports, RETIREMENT_PAGES_MAX + 1, 1);
    assert!(exhausted.exhausted);
    let refused = open_with(&registry, &imports, &json!({ EXPECTED_EPOCH: exhausted.epoch }), || panic!("no chooser")).value;
    assert_eq!(codes(&refused), vec![EPOCH_EXHAUSTED]);
    // Epoch overflow.
    let imports = MbfImports::default();
    imports.seed_epoch_for_tests(u64::MAX);
    let at_max = retire(&registry, &imports, 1, 1);
    assert!(at_max.exhausted && at_max.epoch == u64::MAX);
    // Source-id overflow.
    let imports = MbfImports::default();
    imports.seed_next_source_id_for_tests(u64::MAX);
    let sandbox = Sandbox::new("source-id");
    let overflow = open(&registry, &imports, &sandbox.pair(true)).value;
    assert_eq!(codes(&overflow), vec![EPOCH_EXHAUSTED]);
    assert_eq!(imports.source_id(), None);
    // Apart from the export: each retirement moves only its own epoch.
    let imports = MbfImports::default();
    let runs = crate::mbf_worker::MbfRuns::default();
    retire(&registry, &imports, 1, 1);
    assert_eq!(runs.epoch(), 0, "an import retirement leaves the export alone");
    let id = crate::mbf_worker::RetirementId { page: page(3), sequence: 1 };
    runs.retire_keyed(id, registry.pool());
    assert_eq!(imports.epoch(), 1, "an export retirement leaves the import alone");
    assert_released(&registry);
}

/// The performance session reset carries no page identity (#149) and never names the import's state.
#[test]
fn the_performance_reset_does_not_touch_the_import() {
    let lib = fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("src/lib.rs")).unwrap();
    let start = lib.find("fn dispose_all_preview_mechanics_results").expect("the reset");
    let body = &lib[start..start + lib[start..].find("\n}\n").unwrap()];
    assert!(!body.contains("mbf_import") && !body.contains("MbfImports"), "{body}");
}

// -- Adopt ------------------------------------------------------------------------------------------------------------

/// Adopt binds to the review the user saw: a changed declaration or another review's digest is stale and returns no
/// model; a retired source is refused retired.
#[test]
fn adopt_refuses_a_stale_review_or_a_retired_source_with_no_model() {
    let registry = real_worker();
    let imports = MbfImports::default();
    let sandbox = Sandbox::new("adopt-stale");
    let first = open(&registry, &imports, &sandbox.pair(true)).value;
    let source_id = first["sourceId"].clone();
    let other = json!({"swbpipe-unlisted": {"origin": "project_invented", "source_name": "x"}});
    for body in [
        json!({"sourceId": source_id, "declaration": other, "reviewSha256": first["reviewSha256"]}),
        json!({"sourceId": source_id, "declaration": null, "reviewSha256": "0".repeat(64)}),
    ] {
        let reply = adopt(&registry, &imports, &body).value;
        assert_eq!(codes(&reply), vec![REVIEW_STALE], "{reply}");
        assert!(reply.get("document").is_none());
        assert!(reply.get("file_name").is_some(), "the adopt refusal is the open outcome's shape");
    }
    assert_eq!(codes(&adopt(&registry, &imports, &json!({"sourceId": source_id, "declaration": null, "reviewSha256": "A".repeat(64)})).value), vec![ARGS_INVALID]);
    retire(&registry, &imports, 1, 1);
    let retired = adopt(&registry, &imports, &json!({"sourceId": source_id, "declaration": null, "reviewSha256": first["reviewSha256"]})).value;
    assert_eq!(codes(&retired), vec![SOURCE_RETIRED]);
    assert_released(&registry);
}

// -- The worker and its envelope ----------------------------------------------------------------------------------------

fn worker_envelope(mode: ImportMode, identity: bool) -> String {
    let (text, sidecar) = written_pair();
    let request = WorkerRequest { mode, label: "swbpipe-model.mbf", declaration: b"", mbf: &text, identity: identity.then_some(sidecar.as_slice()) };
    worker_import(&encode_worker_request(&request)).unwrap()
}

/// A successful review-mode result carries the adoptable candidate's digest and length and no model; the adopt-mode
/// result carries that same candidate, verified; the review is identical in both. Every engine import of the corpus
/// pair is the adopted bytes.
#[test]
fn review_mode_carries_no_model_and_adopt_mode_carries_the_verified_candidate() {
    let review_envelope = worker_envelope(ImportMode::Review, true);
    let adopt_envelope = worker_envelope(ImportMode::Adopt, true);
    let reviewed = decode_import_result(ImportMode::Review, review_envelope.as_bytes()).unwrap();
    let adopted = decode_import_result(ImportMode::Adopt, adopt_envelope.as_bytes()).unwrap();
    assert!(reviewed.adoptable && adopted.adoptable);
    assert!(reviewed.model.is_empty() && reviewed.model_bytes > 0);
    assert_eq!(adopted.model.len() as u64, reviewed.model_bytes);
    assert_eq!(reviewed.model_sha, adopted.model_sha);
    assert_eq!(&review_envelope.as_bytes()[reviewed.review.clone()], &adopt_envelope.as_bytes()[adopted.review.clone()]);
    let (text, sidecar) = written_pair();
    assert_eq!(&adopt_envelope.as_bytes()[adopted.model.clone()], engine_model(&text, Some(&sidecar), None).as_slice());
    // A non-adoptable adopt (unbound, no declaration) carries no model.
    let unbound = decode_import_result(ImportMode::Adopt, worker_envelope(ImportMode::Adopt, false).as_bytes()).unwrap();
    assert!(!unbound.adoptable && unbound.model.is_empty());
}

fn replace_header(envelope: &str, index: usize, value: &str) -> String {
    let mut parts: Vec<&str> = envelope.splitn(10, '\n').collect();
    parts[index] = value;
    parts.join("\n")
}

/// Tampered and malformed results are protocol refusals before any derived storage: a model in review mode, the other
/// mode's envelope, a review prefix or digest mismatch, a model digest mismatch, a non-ASCII model, overlong or
/// non-canonical header tokens, uppercase or non-hex digests, a review-digest sentinel and an inconsistent model
/// sentinel.
#[test]
fn every_tampered_or_malformed_envelope_is_a_protocol_refusal() {
    let review_envelope = worker_envelope(ImportMode::Review, true);
    let adopt_envelope = worker_envelope(ImportMode::Adopt, true);
    let refused = |mode: ImportMode, text: &str| {
        let result = decode_import_result(mode, text.as_bytes());
        assert!(matches!(&result, Err(item) if item.code == WORKER_PROTOCOL), "{:?}", result.map(|_| ()));
    };
    refused(ImportMode::Review, &adopt_envelope);
    refused(ImportMode::Adopt, &review_envelope);
    // A model in review mode: the adopt body under a review header.
    refused(ImportMode::Review, &replace_header(&adopt_envelope, 1, "1"));
    // The review's fixed prefix or digest.
    refused(ImportMode::Review, &review_envelope.replacen("\"adoptable\":true", "\"adoptable\":tru ", 1));
    refused(ImportMode::Review, &replace_header(&review_envelope, 5, &"0".repeat(64)));
    // The carried model: a changed byte (digest), a non-ASCII byte.
    let decoded = decode_import_result(ImportMode::Adopt, adopt_envelope.as_bytes()).unwrap();
    let mut changed = adopt_envelope.clone().into_bytes();
    changed[decoded.model.start + 1] = if changed[decoded.model.start + 1] == b' ' { b'\t' } else { b' ' };
    refused(ImportMode::Adopt, std::str::from_utf8(&changed).unwrap());
    let mut non_ascii = adopt_envelope.clone().into_bytes();
    non_ascii[decoded.model.start + 1] = 0xc3;
    assert!(decode_import_result(ImportMode::Adopt, &non_ascii).is_err());
    // Header tokens.
    refused(ImportMode::Review, &replace_header(&review_envelope, 4, &"A".repeat(64)));
    refused(ImportMode::Review, &replace_header(&review_envelope, 4, &"g".repeat(64)));
    refused(ImportMode::Review, &replace_header(&review_envelope, 4, &"0".repeat(65)));
    refused(ImportMode::Review, &replace_header(&review_envelope, 5, "-"));
    refused(ImportMode::Review, &replace_header(&review_envelope, 4, "-"));
    refused(ImportMode::Review, &replace_header(&review_envelope, 2, "importedd"));
    refused(ImportMode::Review, &replace_header(&review_envelope, 6, &"9".repeat(21)));
    refused(ImportMode::Review, &replace_header(&review_envelope, 6, "007"));
    refused(ImportMode::Review, &replace_header(&review_envelope, 3, "2"));
    refused(ImportMode::Review, &replace_header(&review_envelope, 0, "MBF1"));
    refused(ImportMode::Review, &review_envelope[..review_envelope.len() - 1]);
}

/// The reply's exact length is counted with the file name's escapes, and the worker's transfer decision and the
/// host's final count are one function.
#[test]
fn the_adopt_reply_length_is_exact_with_escapes() {
    for name in ["swbpipe-model.mbf", "a\"b\\c\n\u{1}é.mbf"] {
        let model = b"{\"a\":1}\n";
        let mut written = Vec::new();
        put_adopt_reply(&mut written, name, model, &[b'a'; 64], &[b'b'; 64]);
        assert_eq!(adopt_reply_len(name, model.len() as u64), written.len() as u64, "{name}");
        let parsed: Value = serde_json::from_slice(&written).unwrap();
        assert_eq!(parsed["file_name"], name);
        assert_eq!(parsed["byte_count"], model.len());
    }
}

// -- Ledger and cancellation --------------------------------------------------------------------------------------------

/// A review waiting for its grant is cancelled by a retirement well before its wait would end, holding nothing; a
/// newer review supersedes a running one, whose worker is killed and reaped; the scripted result is decoded.
#[test]
fn a_waiting_or_running_review_is_cancelled_and_releases_everything() {
    let gate = Gate::new("mbf-import-cancel");
    let registry = registry_with(gate.launcher());
    let imports = MbfImports::default();
    let sandbox = Sandbox::new("cancel");
    // Install a source without a worker: a real-worker open on a second registry sharing the state.
    let real = real_worker();
    let first = open(&real, &imports, &sandbox.pair(true)).value;
    let body = json!({"sourceId": first["sourceId"], "declaration": null});
    registry.pool().set_max_running_workers(0);
    std::thread::scope(|scope| {
        let began = Instant::now();
        let waiting = scope.spawn(|| review(&registry, &imports, &body).value);
        while !imports.run_in_progress() || registry.pool_status().last_refusal.is_none() {
            assert!(began.elapsed() < Duration::from_secs(30));
            std::thread::sleep(Duration::from_millis(2));
        }
        let retired = retire(&registry, &imports, 1, 1);
        assert_eq!(retired.cancelled, 1);
        assert_eq!(codes(&waiting.join().unwrap()), vec![CANCELLED]);
        assert!(began.elapsed() < START_WAIT + Duration::from_secs(1));
    });
    assert_released(&registry);
    // A running worker superseded by a newer review.
    let imports = MbfImports::default();
    let first = open(&real, &imports, &sandbox.pair(true)).value;
    let body = json!({"sourceId": first["sourceId"], "declaration": null});
    registry.pool().set_max_running_workers(4);
    let scripted = worker_envelope(ImportMode::Review, true);
    std::thread::scope(|scope| {
        let older = scope.spawn(|| review(&registry, &imports, &body).value);
        gate.wait_started(1);
        let newer = scope.spawn(|| review(&registry, &imports, &body).value);
        assert_eq!(codes(&older.join().unwrap()), vec![CANCELLED]);
        gate.wait_started(2);
        gate.release(&scripted);
        let reply = newer.join().unwrap();
        assert_eq!(reply["outcome"], "reviewed", "{reply}");
    });
    assert_released(&registry);
    assert_released(&real);
}

/// A retirement between re-read blocks ends the read at the next block, holding nothing.
#[test]
fn a_retirement_between_read_blocks_cancels_the_reread() {
    let registry = real_worker();
    let imports = MbfImports::default();
    let sandbox = Sandbox::new("between-blocks");
    let mbf_path = sandbox.0.join("big-comment.mbf");
    // An MBF of several blocks: the pair's MBF padded with comment lines is refused by the importer, but the read
    // happens first; only the read's cancellation is observed.
    let (text, _) = written_pair();
    let mut padded = text.clone();
    while padded.len() < 3 * READ_BLOCK_BYTES {
        padded.extend_from_slice(b"* invented padding comment\n");
    }
    fs::write(&mbf_path, &padded).unwrap();
    let first = open(&registry, &imports, &mbf_path).value;
    assert_eq!(first["outcome"], "reviewed", "{first}");
    let (registry_ref, imports_ref): (&'static SolveJobRegistry, &'static MbfImports) = (Box::leak(Box::new(real_worker())), Box::leak(Box::new(imports.clone())));
    let fired = Box::leak(Box::new(AtomicBool::new(false)));
    test_hooks::AFTER_BLOCK.with(|hook| {
        *hook.borrow_mut() = Some(Box::new(|_: &Path, offset: u64| {
            if offset == READ_BLOCK_BYTES as u64 && !fired.swap(true, Ordering::SeqCst) {
                retire(registry_ref, imports_ref, 4, 1);
            }
        }));
    });
    let reply = review(registry_ref, &imports, &json!({"sourceId": first["sourceId"], "declaration": null})).value;
    test_hooks::AFTER_BLOCK.with(|hook| *hook.borrow_mut() = None);
    assert!(fired.load(Ordering::SeqCst));
    assert_eq!(codes(&reply), vec![CANCELLED]);
    assert_released(registry_ref);
}

/// `result_limit`: a review-mode result above R + 4 KiB is refused before any result lease.
#[test]
fn a_review_result_above_its_limit_is_refused_before_any_result_lease() {
    let gate = Gate::new("mbf-import-limit");
    let registry = registry_with(gate.launcher());
    let imports = MbfImports::default();
    let sandbox = Sandbox::new("limit");
    let real = real_worker();
    let first = open(&real, &imports, &sandbox.pair(true)).value;
    let body = json!({"sourceId": first["sourceId"], "declaration": null});
    std::thread::scope(|scope| {
        let running = scope.spawn(|| review(&registry, &imports, &body).value);
        gate.wait_started(1);
        gate.release(&"x".repeat(REVIEW_RESULT_LIMIT as usize + 1));
        assert_eq!(codes(&running.join().unwrap()), vec![WORKER_PROTOCOL]);
    });
    assert_released(&registry);
}

/// The budget boundary: the argument lease, H4's envelope, the review's result share and the starting grant one byte
/// above the captured budget is refused by name before any worker; at the budget the worker starts.
#[test]
fn a_reread_one_byte_above_the_budget_is_refused_before_any_worker() {
    let gate = Gate::new("mbf-import-budget");
    let imports = MbfImports::default();
    let sandbox = Sandbox::new("budget");
    let real = real_worker();
    let first = open(&real, &imports, &sandbox.pair(true)).value;
    let (text, sidecar) = written_pair();
    let raw_len = request_len("swbpipe-model.mbf".len(), 0, text.len() as u64, Some(sidecar.len() as u64));
    let start = initial_grant(crate::solve_worker::PROCESS_RESERVE_BYTES, resident_envelope(raw_len)).unwrap();
    let scripted = worker_envelope(ImportMode::Review, true);
    for admitted in [false, true] {
        let registry = registry_with(gate.launcher());
        let body = ticketed(&imports, &json!({"sourceId": first["sourceId"], "declaration": null}));
        // The admitted argument lease and its parse growth are measured by a first admission, then the budget set
        // before the command's copy lease (whose admission captures it).
        let probe_registry = real_worker();
        let (probe_raw, probe_copy) = reserve(&probe_registry, &raw(&body), &headers(Some(INGRESS_ARGS_JSON_V1)), REVIEW_COMMAND).unwrap();
        let (_memory, _args, probe_lease) = admit(&probe_registry, probe_raw, probe_copy, REVIEW_COMMAND).unwrap();
        let argument = probe_lease.bytes();
        drop(probe_lease);
        let required = argument + allocation_envelope(raw_len).unwrap() + resident_envelope(REVIEW_RESULT_LIMIT) + start;
        let budget = if admitted { required } else { required - 1 };
        registry.configure_pool(12 << 30, budget, false).unwrap();
        let (raw_args, copy) = reserve(&registry, &raw(&body), &headers(Some(INGRESS_ARGS_JSON_V1)), REVIEW_COMMAND).unwrap();
        let started = gate.started_count();
        std::thread::scope(|scope| {
            let running = scope.spawn(|| review_command(&registry, &imports, raw_args, copy, START_WAIT, |text| text).unwrap());
            if admitted {
                gate.wait_started(started + 1);
                gate.release(&scripted);
                let reply: Value = serde_json::from_str(&running.join().unwrap()).unwrap();
                assert_eq!(reply["outcome"], "reviewed", "{reply}");
            } else {
                let reply: Value = serde_json::from_str(&running.join().unwrap()).unwrap();
                assert_eq!(codes(&reply), vec![MEMORY_UNAVAILABLE], "{reply}");
                assert!(reply["diagnostics"][0]["message"].as_str().unwrap().contains(&format!("\"required_bytes\":{required}")), "{reply}");
                assert_eq!(gate.started_count(), started, "no worker started");
            }
        });
        assert_released(&registry);
    }
}

// -- The only production caller of the import worker --------------------------------------------------------------------

#[derive(Default)]
struct Calls {
    items: Vec<String>,
    test_depth: usize,
    found: Vec<String>,
}

fn test_only(attrs: &[syn::Attribute]) -> bool {
    attrs.iter().any(|attr| attr.path().is_ident("test") || (attr.path().is_ident("cfg") && attr.parse_args::<syn::Ident>().is_ok_and(|ident| ident == "test")))
}

impl Calls {
    fn within(&mut self, attrs: &[syn::Attribute], item: Option<String>, visit: impl FnOnce(&mut Self)) {
        let test = test_only(attrs);
        self.test_depth += usize::from(test);
        let named = item.is_some();
        self.items.extend(item);
        visit(self);
        if named {
            self.items.pop();
        }
        self.test_depth -= usize::from(test);
    }
}

impl<'ast> syn::visit::Visit<'ast> for Calls {
    fn visit_item_fn(&mut self, item: &'ast syn::ItemFn) {
        self.within(&item.attrs, Some(item.sig.ident.to_string()), |this| syn::visit::visit_item_fn(this, item));
    }
    fn visit_impl_item_fn(&mut self, item: &'ast syn::ImplItemFn) {
        self.within(&item.attrs, Some(item.sig.ident.to_string()), |this| syn::visit::visit_impl_item_fn(this, item));
    }
    fn visit_item_mod(&mut self, item: &'ast syn::ItemMod) {
        self.within(&item.attrs, None, |this| syn::visit::visit_item_mod(this, item));
    }
    fn visit_path(&mut self, path: &'ast syn::Path) {
        if self.test_depth == 0 && path.segments.last().is_some_and(|last| last.ident == "worker_import" || last.ident == "import_mbf_json") {
            self.found.push(format!("{}: {}", self.items.last().map_or("-", String::as_str), path.segments.last().unwrap().ident));
        }
        syn::visit::visit_path(self, path);
    }
}

fn production_calls(sources: &[(&str, &str)]) -> Vec<String> {
    let mut found = Vec::new();
    for (file, text) in sources {
        let parsed = syn::parse_file(text).unwrap_or_else(|error| panic!("{file}: {error}"));
        let mut calls = Calls::default();
        syn::visit::Visit::visit_file(&mut calls, &parsed);
        found.extend(calls.found.into_iter().map(|item| format!("{file}: {item}")));
    }
    found.sort();
    found
}

/// In production code, task 5's `workload` arm is the only caller of `worker_import`, and no native source refers to
/// the wasm `import_mbf_json`; an added caller (the negative control) is found.
#[test]
fn task_5_is_the_only_production_caller_of_the_import_worker() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut texts = Vec::new();
    for entry in fs::read_dir(&dir).unwrap().flatten() {
        let name = entry.file_name().to_string_lossy().into_owned();
        if name.ends_with(".rs") && !name.ends_with("_tests.rs") {
            texts.push((name, fs::read_to_string(entry.path()).unwrap()));
        }
    }
    let sources: Vec<(&str, &str)> = texts.iter().map(|(name, text)| (name.as_str(), text.as_str())).collect();
    assert_eq!(production_calls(&sources), vec!["solve_worker.rs: workload: worker_import".to_string()]);
    let control = [("rogue.rs", "fn rogue(raw: &[u8]) { let _ = crate::mbf_import::worker_import(raw); }")];
    assert_eq!(production_calls(&control), vec!["rogue.rs: rogue: worker_import".to_string()]);
}

// -- The dialog's fixture: actual native replies --------------------------------------------------------------------------

const IMPORT_REPLIES: &str = "../../../fixtures/caepipe_mbf/invented/desktop_import/import_replies.json";

/// The replies the webview's tests read (`fixtures/caepipe_mbf/invented/desktop_import/import_replies.json`) are the
/// native commands' own replies, through task 5 in a real worker, for the export's saved pair: bound (open, adopt) and
/// unbound (open, declared review, adopt). Capture the response callback's string verbatim, never a re-serialization
/// of its parsed model. The companion covers the entire invented transcript and every input. Runtime provenance
/// inside a captured reply is not a public engineering-data disposition. Regenerate with `MBF_IMPORT_REPLIES_REGENERATE=1`.
#[test]
fn the_dialogs_import_replies_are_the_native_replies() {
    let registry = real_worker();
    let sandbox = Sandbox::new("replies");
    let mbf_path = sandbox.native_pair(&registry);
    let (mbf_bytes, identity_bytes) = written_pair();
    assert_eq!(fs::read(&mbf_path).unwrap(), mbf_bytes, "the actual saved MBF is the identified invented export");
    assert_eq!(fs::read(sandbox.ids()).unwrap(), identity_bytes, "the actual saved identity file is its companion");
    let imports = MbfImports::default();
    let bound_open = open(&registry, &imports, &mbf_path);
    let bound_adopt = adopt(&registry, &imports, &json!({"sourceId": bound_open.value["sourceId"], "declaration": null, "reviewSha256": bound_open.value["reviewSha256"]}));
    fs::remove_file(sandbox.ids()).unwrap();
    let imports = MbfImports::default();
    let unbound_open = open(&registry, &imports, &mbf_path);
    let declaration = declaration_for(unbound_open.value["review"]["declaration_needed"].as_array().unwrap());
    let declaration_json = serde_json::to_string(&declaration).unwrap();
    let unbound_declared = review(&registry, &imports, &json!({"sourceId": unbound_open.value["sourceId"], "declaration": declaration}));
    let unbound_adopt = adopt(&registry, &imports, &json!({"sourceId": unbound_open.value["sourceId"], "declaration": declaration, "reviewSha256": unbound_declared.value["reviewSha256"]}));
    assert_eq!(bound_adopt.value["outcome"], "opened");
    assert_eq!(unbound_adopt.value["outcome"], "opened");
    for review in [&bound_open.value, &unbound_open.value, &unbound_declared.value] {
        assert_eq!(review["review"]["source"]["mbf_sha256"], hex(&mbf_bytes));
    }
    assert_eq!(bound_open.value["review"]["source"]["identity_file"]["sha256"], hex(&identity_bytes));
    assert_eq!(bound_adopt.value["reviewSha256"], bound_open.value["reviewSha256"]);
    assert_eq!(unbound_adopt.value["reviewSha256"], unbound_declared.value["reviewSha256"]);
    assert_eq!(unbound_declared.value["review"]["declaration"]["sha256"], hex(declaration_json.as_bytes()));
    for (adopted, expected) in [
        (&bound_adopt.value, engine_model(&mbf_bytes, Some(&identity_bytes), None)),
        (&unbound_adopt.value, engine_model(&mbf_bytes, None, Some(declaration_json.as_bytes()))),
    ] {
        assert_eq!(adopted["modelSha256"], hex(&expected));
        assert_eq!(adopted["byte_count"], expected.len());
        assert_eq!(adopted["document"], serde_json::from_slice::<Value>(&expected).unwrap(), "every captured record comes from only the identified invented inputs");
    }
    for captured in [&bound_open, &bound_adopt, &unbound_open, &unbound_declared, &unbound_adopt] {
        assert!(!captured.text.contains(sandbox.0.to_str().unwrap()), "no temporary host path is captured");
    }
    let document = json!({
        "artifact_kind": "swbpipe.mbf_import_protocol_capture",
        "format": 1,
        "description": "Verbatim native response strings for transport tests, not a public engineering-model dataset; generated by mbf_import_tests.rs the_dialogs_import_replies_are_the_native_replies; public provenance and coverage are in import_replies.provenance.json",
        "inputs": {
            "source_model": "fixtures/caepipe_mbf/invented/desktop_export/export_model.json",
            "source_model_file_sha256": hex(EXPORT_MODEL.as_bytes()),
            "export_settings_json": EXPORT_SETTINGS,
            "saved_mbf_sha256": hex(&mbf_bytes),
            "saved_identity_file_sha256": hex(&identity_bytes),
            "declaration_json": declaration_json,
        },
        "replies": {
            "bound-open": bound_open.text,
            "bound-adopt": bound_adopt.text,
            "unbound-open": unbound_open.text,
            "unbound-declared": unbound_declared.text,
            "unbound-adopt": unbound_adopt.text,
        },
    });
    let text = format!("{}\n", serde_json::to_string_pretty(&document).unwrap());
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join(IMPORT_REPLIES);
    if std::env::var_os("MBF_IMPORT_REPLIES_REGENERATE").is_some() {
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(&path, &text).unwrap();
    }
    assert_eq!(fs::read_to_string(&path).unwrap_or_default(), text, "regenerate with MBF_IMPORT_REPLIES_REGENERATE=1");
    let provenance: Value = serde_json::from_slice(&fs::read(path.with_file_name("import_replies.provenance.json")).unwrap()).unwrap();
    for field in ["source_name", "source_location", "source_license", "contributor", "contributor_certification", "redistribution_status", "review_status"] {
        assert!(provenance[field].as_str().is_some_and(|value| !value.trim().is_empty()), "the public transcript needs {field}");
    }
    assert_eq!(provenance["redistribution_status"], "public_permissive");
    assert!(!matches!(provenance["review_status"].as_str(), Some("rejected" | "quarantined")));
    for group in ["inputs", "replies"] {
        let covered: std::collections::BTreeSet<_> = provenance["coverage"][group].as_array().unwrap().iter().map(|value| value.as_str().unwrap()).collect();
        let actual: std::collections::BTreeSet<_> = document[group].as_object().unwrap().keys().map(String::as_str).collect();
        assert_eq!(covered, actual, "all captured {group} share the explicit public provenance");
    }
    assert_released(&registry);
}
