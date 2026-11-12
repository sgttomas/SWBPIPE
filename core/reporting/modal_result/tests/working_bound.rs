//! DEL-04-09: the modal reader's working-memory
//! bound, [`modal_reader_working_bound`], against a measurement. A counting
//! global allocator records, on the reading thread only, the live requested
//! bytes and the live bytes priced by the allocation envelope
//! max(16, next_power_of_two(size)) of each block (a reallocation counts its
//! old and new blocks together); the bound must cover the envelope-priced
//! peak of parsing the request and reading the document, for every committed
//! fixture, every corpus, sum-order, oracle and producer-edge entry (each with
//! and without its request) and adversarial texts.
use open_pipe_stress_modal_result::{modal_reader_sha256, modal_reader_working_bound, modal_reader_working_terms, read_modal_result_text, ModalSupplied};
use serde_json::{json, Value};
use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;

// ---------------------------------------------------------------------------
// The counting allocator.

struct Counting;

std::thread_local! {
    static ACTIVE: Cell<bool> = const { Cell::new(false) };
    static LIVE: Cell<i64> = const { Cell::new(0) };
    static PEAK: Cell<i64> = const { Cell::new(0) };
    static LIVE_REQUESTED: Cell<i64> = const { Cell::new(0) };
    static PEAK_REQUESTED: Cell<i64> = const { Cell::new(0) };
    static SIZES: Cell<usize> = const { Cell::new(0) };
}

/// The allocation envelope, stated here independently of the crate.
fn envelope(size: usize) -> i64 {
    if size == 0 {
        0
    } else {
        size.next_power_of_two().max(16) as i64
    }
}

fn record(added: usize, removed: usize) {
    let _ = ACTIVE.try_with(|active| {
        if !active.get() {
            return;
        }
        for (live, peak, price) in [(&LIVE, &PEAK, envelope as fn(usize) -> i64), (&LIVE_REQUESTED, &PEAK_REQUESTED, |s: usize| s as i64)] {
            let _ = live.try_with(|l| {
                let now = l.get() + price(added);
                let _ = peak.try_with(|p| p.set(p.get().max(now)));
                l.set(now - price(removed));
            });
        }
        if added > 0 {
            let _ = SIZES.try_with(|s| s.set(s.get().max(added)));
        }
    });
}

unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let p = unsafe { System.alloc(layout) };
        if !p.is_null() {
            record(layout.size(), 0);
        }
        p
    }
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        let p = unsafe { System.alloc_zeroed(layout) };
        if !p.is_null() {
            record(layout.size(), 0);
        }
        p
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) };
        record(0, layout.size());
    }
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        let p = unsafe { System.realloc(ptr, layout, new_size) };
        if !p.is_null() {
            record(new_size, layout.size());
        }
        p
    }
}

#[global_allocator]
static ALLOCATOR: Counting = Counting;

/// Runs `f` on this thread with the counters reset: (its result, the
/// envelope-priced peak, the requested peak, the largest single request).
fn measure<T>(f: impl FnOnce() -> T) -> (T, u64, u64, usize) {
    for c in [&LIVE, &PEAK, &LIVE_REQUESTED, &PEAK_REQUESTED] {
        c.with(|x| x.set(0));
    }
    SIZES.with(|s| s.set(0));
    ACTIVE.with(|a| a.set(true));
    struct Off;
    impl Drop for Off {
        fn drop(&mut self) {
            ACTIVE.with(|a| a.set(false));
        }
    }
    let off = Off;
    let out = f();
    drop(off);
    (out, PEAK.with(Cell::get) as u64, PEAK_REQUESTED.with(Cell::get) as u64, SIZES.with(Cell::get))
}

/// The bounded sequence: parse the request text, read the document text with
/// it, hold the verdict.
fn read_peak(document: &str, request: Option<&str>) -> (u64, u64) {
    let (held, peak, requested, _) = measure(|| {
        let request: Option<Value> = request.map(|text| serde_json::from_str(text).expect("a request text is JSON"));
        let verdict = read_modal_result_text(document, ModalSupplied { request: request.as_ref() });
        (verdict, request)
    });
    drop(held);
    (peak, requested)
}

/// The largest peak / bound and bound / peak over a run, with their cases.
#[derive(Default)]
struct Tally {
    cases: usize,
    unread: usize,
    worst: (f64, String),
    loosest: (f64, String),
}

impl Tally {
    fn check(&mut self, name: &str, document: &str, request: Option<&str>) {
        let Some(bound) = modal_reader_working_bound(document, request) else {
            // Not JSON: the host refuses it unread, and so would the reader.
            assert!(serde_json::from_str::<Value>(document).is_err(), "{name}: no bound for a JSON document");
            assert!(request.is_none_or(|r| serde_json::from_str::<Value>(r).is_ok()));
            self.unread += 1;
            return;
        };
        let (peak, requested) = read_peak(document, request);
        assert!(requested <= peak);
        assert!(peak <= bound, "{name}: measured peak {peak} B exceeds the bound {bound} B");
        let ratio = peak as f64 / bound as f64;
        let slack = bound as f64 / peak.max(1) as f64;
        self.cases += 1;
        if ratio > self.worst.0 {
            self.worst = (ratio, format!("{name} (peak {peak} B, bound {bound} B)"));
        }
        if slack > self.loosest.0 {
            self.loosest = (slack, format!("{name} (peak {peak} B, bound {bound} B)"));
        }
        if std::env::var_os("WORKING_BOUND_TERMS").is_some() {
            eprintln!("{name}: peak {peak}, {:?}", modal_reader_working_terms(document, request).unwrap());
        }
    }
    fn report(&self, what: &str) {
        eprintln!("working bound, {what}: {} reads ({} texts not JSON, refused unread); max peak/bound {:.4} at {}; max bound/peak {:.2} at {}", self.cases, self.unread, self.worst.0, self.worst.1,
            self.loosest.0, self.loosest.1);
    }
}

// ---------------------------------------------------------------------------
// The committed texts and the corpus operations (copied from tests/corpus.rs).

const CORPUS: &str = include_str!("../../../../fixtures/results/modal_corpus.json");
const SUM_ORDER: &str = include_str!("../../../../fixtures/results/modal_sum_order_boundaries.json");
const ORACLE_CASES: &str = include_str!("../../../../fixtures/results/modal_oracle_cases.json");
const PRODUCER_EDGES: &str = include_str!("../../../../fixtures/results/modal_producer_edges.json");

const FIXTURES: [&str; 12] = ["ex_n05", "arc", "component_body", "state_one_way_pressed", "state_friction_sliding", "cantilever_whole_spectrum",
    "cantilever_cluster_extension", "cantilever_below_cutoff", "cantilever_subnormal_cutoff", "cantilever_sensitive", "refused_unseparated", "refused_no_bound"];

fn fixture(name: &str) -> &'static str {
    match name {
        "modal_ex_n05.json" => include_str!("../../../../fixtures/results/modal_ex_n05.json"),
        "modal_ex_n05.request.json" => include_str!("../../../../fixtures/results/modal_ex_n05.request.json"),
        "modal_arc.json" => include_str!("../../../../fixtures/results/modal_arc.json"),
        "modal_arc.request.json" => include_str!("../../../../fixtures/results/modal_arc.request.json"),
        "modal_component_body.json" => include_str!("../../../../fixtures/results/modal_component_body.json"),
        "modal_component_body.request.json" => include_str!("../../../../fixtures/results/modal_component_body.request.json"),
        "modal_state_one_way_pressed.json" => include_str!("../../../../fixtures/results/modal_state_one_way_pressed.json"),
        "modal_state_one_way_pressed.request.json" => include_str!("../../../../fixtures/results/modal_state_one_way_pressed.request.json"),
        "modal_state_friction_sliding.json" => include_str!("../../../../fixtures/results/modal_state_friction_sliding.json"),
        "modal_state_friction_sliding.request.json" => include_str!("../../../../fixtures/results/modal_state_friction_sliding.request.json"),
        "modal_cantilever_whole_spectrum.json" => include_str!("../../../../fixtures/results/modal_cantilever_whole_spectrum.json"),
        "modal_cantilever_whole_spectrum.request.json" => include_str!("../../../../fixtures/results/modal_cantilever_whole_spectrum.request.json"),
        "modal_cantilever_cluster_extension.json" => include_str!("../../../../fixtures/results/modal_cantilever_cluster_extension.json"),
        "modal_cantilever_cluster_extension.request.json" => include_str!("../../../../fixtures/results/modal_cantilever_cluster_extension.request.json"),
        "modal_cantilever_below_cutoff.json" => include_str!("../../../../fixtures/results/modal_cantilever_below_cutoff.json"),
        "modal_cantilever_below_cutoff.request.json" => include_str!("../../../../fixtures/results/modal_cantilever_below_cutoff.request.json"),
        "modal_cantilever_subnormal_cutoff.json" => include_str!("../../../../fixtures/results/modal_cantilever_subnormal_cutoff.json"),
        "modal_cantilever_subnormal_cutoff.request.json" => include_str!("../../../../fixtures/results/modal_cantilever_subnormal_cutoff.request.json"),
        "modal_cantilever_sensitive.json" => include_str!("../../../../fixtures/results/modal_cantilever_sensitive.json"),
        "modal_cantilever_sensitive.request.json" => include_str!("../../../../fixtures/results/modal_cantilever_sensitive.request.json"),
        "modal_refused_unseparated.json" => include_str!("../../../../fixtures/results/modal_refused_unseparated.json"),
        "modal_refused_unseparated.request.json" => include_str!("../../../../fixtures/results/modal_refused_unseparated.request.json"),
        "modal_refused_no_bound.json" => include_str!("../../../../fixtures/results/modal_refused_no_bound.json"),
        "modal_refused_no_bound.request.json" => include_str!("../../../../fixtures/results/modal_refused_no_bound.request.json"),
        other => panic!("no committed modal fixture `{other}`"),
    }
}

fn pointer_parts(path: &str) -> Vec<String> {
    path.split('/').skip(1).map(|p| p.replace("~1", "/").replace("~0", "~")).collect()
}

fn parent_of<'a>(doc: &'a mut Value, parts: &[String]) -> &'a mut Value {
    let mut node = doc;
    for part in &parts[..parts.len() - 1] {
        node = match node {
            Value::Array(items) => &mut items[part.parse::<usize>().unwrap()],
            other => other.get_mut(part.as_str()).unwrap_or_else(|| panic!("no member `{part}`")),
        };
    }
    node
}

fn apply(doc: &mut Value, operations: &[Value]) {
    for op in operations {
        match op["op"].as_str().unwrap() {
            "replace" | "add" | "scale" => {
                let parts = pointer_parts(op["path"].as_str().unwrap());
                let last = parts.last().unwrap().clone();
                let parent = parent_of(doc, &parts);
                let slot = match parent {
                    Value::Array(items) => &mut items[last.parse::<usize>().unwrap()],
                    Value::Object(map) => {
                        if op["op"] == "add" {
                            map.insert(last.clone(), Value::Null);
                        }
                        map.get_mut(&last).unwrap()
                    }
                    _ => panic!("{op}"),
                };
                if op["op"] == "scale" {
                    let factor = op["factor"].as_f64().unwrap();
                    *slot = json!(slot.as_f64().unwrap() * factor);
                } else {
                    *slot = op["value"].clone();
                }
            }
            "scale_rows" => {
                let field = op["field"].as_str().unwrap();
                let factor = op["factor"].as_f64().unwrap();
                let rows = doc["tables"][op["table"].as_str().unwrap()]["rows"].as_array_mut().unwrap();
                for row in rows {
                    if op["where"].as_object().unwrap().iter().all(|(k, v)| row[k] == *v) {
                        let value = &mut row[field]["value"];
                        *value = json!(value.as_f64().unwrap() * factor);
                    }
                }
            }
            "withhold" => {
                doc["tables"][op["table"].as_str().unwrap()] = json!({"status": "withheld", "reason": "corpus: payload withheld"});
            }
            "remove_rows" => {
                let rows = doc["tables"][op["table"].as_str().unwrap()]["rows"].as_array_mut().unwrap();
                let matches = |row: &Value| op["where"].as_object().unwrap().iter().all(|(k, v)| row[k] == *v);
                rows.retain(|row| !matches(row));
                if let Some(prefix) = op["renumber_prefix"].as_str() {
                    for (i, row) in rows.iter_mut().enumerate() {
                        row["ordinal"] = json!(i);
                        row["row_id"] = json!(format!("{prefix}{i}"));
                    }
                }
            }
            "set_rows" => {
                let rows = doc["tables"][op["table"].as_str().unwrap()]["rows"].as_array_mut().unwrap();
                for row in rows.iter_mut().filter(|row| op["where"].as_object().unwrap().iter().all(|(k, v)| row[k] == *v)) {
                    row[op["field"].as_str().unwrap()] = op["value"].clone();
                }
            }
            other => panic!("unknown corpus operation `{other}`"),
        }
    }
}

fn reseal_table(table: &mut Value) {
    if table["status"] != "published" {
        return;
    }
    let mut checksums = Vec::new();
    for row in table["rows"].as_array_mut().unwrap() {
        let mut body = row.as_object().unwrap().clone();
        body.remove("checksum");
        let checksum = modal_reader_sha256(&Value::Object(body)).unwrap();
        row["checksum"] = json!(checksum);
        checksums.push(json!(checksum));
    }
    table["row_count"] = json!(checksums.len());
    table["digest"] = json!(modal_reader_sha256(&Value::Array(checksums)).unwrap());
}

fn reseal(doc: &mut Value) {
    if let Some(tables) = doc.get_mut("tables").and_then(Value::as_object_mut) {
        for table in tables.values_mut() {
            reseal_table(table);
        }
    }
    if let Some(table) = doc.get_mut("refusals") {
        reseal_table(table);
    }
}

fn entry_text(entry: &Value) -> String {
    let raw = fixture(entry["document"].as_str().unwrap());
    if let Some(replacements) = entry.get("text_operations").and_then(Value::as_array) {
        let mut text = raw.to_string();
        for r in replacements {
            text = text.replacen(r["find"].as_str().unwrap(), r["replace"].as_str().unwrap(), 1);
        }
        return text;
    }
    let mut doc: Value = serde_json::from_str(raw).unwrap();
    apply(&mut doc, entry["operations"].as_array().unwrap());
    if entry["reseal"] == true {
        reseal(&mut doc);
    }
    serde_json::to_string(&doc).unwrap()
}

fn entry_request(entry: &Value) -> Option<String> {
    entry["request"].as_str().map(|name| {
        let mut request: Value = serde_json::from_str(fixture(name)).unwrap();
        apply(&mut request, entry.get("request_operations").and_then(Value::as_array).map_or(&[][..], |v| v.as_slice()));
        serde_json::to_string(&request).unwrap()
    })
}

fn check_entries(tally: &mut Tally, source: &str, key: &str) {
    let corpus: Value = serde_json::from_str(source).unwrap();
    for entry in corpus[key].as_array().unwrap() {
        let name = entry["id"].as_str().unwrap_or("?").to_string();
        let text = entry_text(entry);
        let request = entry_request(entry);
        tally.check(&format!("{name} (with its request)"), &text, request.as_deref());
        tally.check(&format!("{name} (no request)"), &text, None);
    }
}

// ---------------------------------------------------------------------------
// Pins measured here.

/// `working_bound.rs` prices a `BTreeMap` node as 8 + 11·(key + value) + 4,
/// rounded to 8, plus 12·8 for an internal node: 632 / 728 for
/// `BTreeMap<String, Value>` (serde_json's `Map`), 280 / 376 for the row
/// checksum's `BTreeMap<&str, &Value>`.
#[test]
fn btree_node_sizes_are_the_formula() {
    let leaf_and_internal = |make: &dyn Fn(usize)| {
        let (_, _, _, leaf) = measure(|| make(1));
        let (_, _, _, internal) = measure(|| make(12));
        (leaf, internal)
    };
    let owned = leaf_and_internal(&|n| {
        let mut map = serde_json::Map::new();
        for i in 0..n {
            map.insert(format!("{i:02}"), Value::Null);
        }
        drop(map);
    });
    assert_eq!(owned, (632, 728));
    let value = Value::Null;
    let keys: Vec<String> = (0..12).map(|i| format!("{i:02}")).collect();
    let borrowed = leaf_and_internal(&|n| {
        let map: std::collections::BTreeMap<&str, &Value> = keys.iter().take(n).map(|k| (k.as_str(), &value)).collect();
        drop(map);
    });
    // `collect` sorts into a bulk-built tree; one node of each kind either way.
    assert_eq!(borrowed, (280, 376));
}

// ---------------------------------------------------------------------------
// The committed texts.

#[test]
fn the_bound_covers_every_committed_fixture() {
    let mut tally = Tally::default();
    for name in FIXTURES {
        let document = fixture(&format!("modal_{name}.json"));
        let request = fixture(&format!("modal_{name}.request.json"));
        tally.check(&format!("{name} (with its request)"), document, Some(request));
        tally.check(&format!("{name} (no request)"), document, None);
    }
    tally.report("committed fixtures");
}

#[test]
fn the_bound_covers_every_corpus_entry() {
    let mut tally = Tally::default();
    check_entries(&mut tally, CORPUS, "entries");
    check_entries(&mut tally, SUM_ORDER, "entries");
    tally.report("corpus and sum-order entries");
    let mut tally = Tally::default();
    check_entries(&mut tally, ORACLE_CASES, "cases");
    check_entries(&mut tally, PRODUCER_EDGES, "entries");
    tally.report("oracle cases and producer edges");
}

// ---------------------------------------------------------------------------
// Adversarial texts.

/// `doc` with its reported modes repeated `k` times (modes, computed modes,
/// clusters, participation and shapes renumbered; every table resealed). The
/// closure passes; the relations read it (and fail where its values do).
fn repeat_modes(doc: &mut Value, k: usize) {
    let t = |doc: &Value, name: &str| doc["tables"][name]["rows"].as_array().unwrap().clone();
    let (modes, computed, participation, shapes) = (t(doc, "modes"), t(doc, "computed_modes"), t(doc, "participation"), t(doc, "shapes"));
    let n = modes.len();
    let ndof = shapes.len() / n;
    let total = n * k;
    let mut new_modes = Vec::new();
    let mut new_computed = Vec::new();
    let mut new_clusters = Vec::new();
    let mut new_participation = Vec::new();
    let mut new_shapes = Vec::new();
    let cluster_template = t(doc, "clusters")[0].clone();
    for j in 0..total {
        let id = j + 1;
        let mut m = modes[j % n].clone();
        m["mode_index"] = json!(id);
        m["row_id"] = json!(format!("mode:{id}"));
        m["ordinal"] = json!(j);
        m["computed_ref"] = json!(format!("computed:{id}"));
        m["cluster_ref"] = json!(format!("cluster:{id}"));
        new_modes.push(m);
        let mut c = computed[j % n].clone();
        c["computed_index"] = json!(id);
        c["row_id"] = json!(format!("computed:{id}"));
        c["ordinal"] = json!(j);
        c["cluster_ref"] = json!(format!("cluster:{id}"));
        c["reported"] = json!(true);
        c["mode_ref"] = json!(format!("mode:{id}"));
        new_computed.push(c);
        let mut cl = cluster_template.clone();
        cl["cluster_index"] = json!(id);
        cl["row_id"] = json!(format!("cluster:{id}"));
        cl["ordinal"] = json!(j);
        cl["first_computed_ref"] = json!(format!("computed:{id}"));
        cl["last_computed_ref"] = json!(format!("computed:{id}"));
        cl["reported"] = json!(true);
        new_clusters.push(cl);
        for (d, dir) in ["x", "y", "z"].iter().enumerate() {
            let mut p = participation[(j % n) * 3 + d].clone();
            p["mode_ref"] = json!(format!("mode:{id}"));
            p["row_id"] = json!(format!("participation:mode:{id}:{dir}"));
            p["ordinal"] = json!(3 * j + d);
            new_participation.push(p);
        }
        for dof in 0..ndof {
            let mut s = shapes[(j % n) * ndof + dof].clone();
            let dof_ref = s["dof_ref"].as_str().unwrap().to_string();
            s["mode_ref"] = json!(format!("mode:{id}"));
            s["row_id"] = json!(format!("shape:mode:{id}:{dof_ref}"));
            s["ordinal"] = json!(j * ndof + dof);
            new_shapes.push(s);
        }
    }
    for (name, rows) in [("modes", new_modes), ("computed_modes", new_computed), ("clusters", new_clusters), ("participation", new_participation),
        ("shapes", new_shapes)] {
        doc["tables"][name]["rows"] = Value::Array(rows);
    }
    reseal(doc);
}

/// `doc` with `m` more nodes (six DOFs each, all free, a diagonal mass entry
/// each, a zero shape component in every mode), resealed; the guard's
/// dimensions follow.
fn add_nodes(doc: &mut Value, m: usize, id_len: usize) {
    let t = |doc: &Value, name: &str| doc["tables"][name]["rows"].as_array().unwrap().clone();
    let (mut nodes, mut dofs, mut mass, shapes, modes) = (t(doc, "nodes"), t(doc, "dofs"), t(doc, "mass_matrix"), t(doc, "shapes"), t(doc, "modes"));
    let old_ndof = dofs.len();
    let n = modes.len();
    let mut free = dofs.iter().filter(|r| !r["free_index"].is_null()).count();
    let components = ["ux", "uy", "uz", "rx", "ry", "rz"];
    let mut new_dofs = Vec::new();
    for j in 0..m {
        let name = format!("x{}{j}", "0".repeat(id_len.saturating_sub(1)));
        let mut node = nodes[0].clone();
        node["node_id"] = json!(name);
        node["row_id"] = json!(format!("node:{name}"));
        node["ordinal"] = json!(nodes.len());
        for axis in ["x", "y", "z"] {
            node[axis]["value"] = json!(j as f64 + 0.5);
        }
        nodes.push(node);
        for (c, component) in components.iter().enumerate() {
            let dof_ref = format!("dof:{name}:{component}");
            let mut dof = dofs[c].clone();
            dof["node_ref"] = json!(format!("node:{name}"));
            dof["component"] = json!(component);
            dof["free_index"] = json!(free);
            free += 1;
            dof["row_id"] = json!(dof_ref.clone());
            dof["ordinal"] = json!(dofs.len());
            dofs.push(dof);
            let template = mass.iter().find(|r| r["pair_class"] == if c < 3 { "translation_translation" } else { "rotation_rotation" }).unwrap_or(&mass[0]).clone();
            let mut entry = template;
            entry["row_dof_ref"] = json!(dof_ref.clone());
            entry["col_dof_ref"] = json!(dof_ref.clone());
            entry["row_id"] = json!(format!("mass:{dof_ref}|{dof_ref}"));
            entry["ordinal"] = json!(mass.len());
            mass.push(entry);
            new_dofs.push((dof_ref, c));
        }
    }
    let mut new_shapes = Vec::new();
    for mode in 0..n {
        for dof in 0..old_ndof {
            let mut s = shapes[mode * old_ndof + dof].clone();
            s["ordinal"] = json!(new_shapes.len());
            new_shapes.push(s);
        }
        for (dof_ref, c) in &new_dofs {
            let mut s = shapes[mode * old_ndof + c].clone();
            s["dof_ref"] = json!(dof_ref);
            s["value"]["value"] = json!(0.0);
            s["row_id"] = json!(format!("shape:mode:{}:{dof_ref}", mode + 1));
            s["ordinal"] = json!(new_shapes.len());
            new_shapes.push(s);
        }
    }
    doc["guard"]["global_dimension"] = json!(dofs.len());
    doc["guard"]["free_dimension"] = json!(free);
    for (name, rows) in [("nodes", nodes), ("dofs", dofs), ("mass_matrix", mass), ("shapes", new_shapes)] {
        doc["tables"][name]["rows"] = Value::Array(rows);
    }
    reseal(doc);
}

/// Rebinds `doc` to `request` (its request, model and materials hashes and
/// its run id), so the reader derives the model's mass from it.
fn rebind(doc: &mut Value, request: &Value) {
    let b = &mut doc["binding"];
    b["request_sha256"] = json!(modal_reader_sha256(request).unwrap());
    b["model_sha256"] = json!(modal_reader_sha256(&request["model"]).unwrap());
    b["materials_sha256"] = json!(modal_reader_sha256(request.get("materials").unwrap_or(&json!([]))).unwrap());
    let run = modal_reader_sha256(&json!({"request_sha256": b["request_sha256"], "integrity_parameters_sha256": b["integrity_parameters_sha256"]})).unwrap();
    doc["run"]["run_id"] = json!(format!("modal-run:{}", &run[..16]));
}

#[test]
fn the_bound_covers_adversarial_texts() {
    let mut tally = Tally::default();
    let base: Value = serde_json::from_str(fixture("modal_ex_n05.json")).unwrap();
    let base_request = fixture("modal_ex_n05.request.json");

    // Deep nesting: 127 arrays; 127 objects, each with 40 keys before the next.
    let deep_arrays = format!("{}{}", "[".repeat(127), "]".repeat(127));
    tally.check("127 nested arrays", &deep_arrays, None);

    // Review regression: refusal codes live from closure through binding.
    // 1,100 resealed refusal rows with 1,025-byte codes, and a request of
    // 1,024 strings of 129 `\u0001` escapes (a large binding-phase hash).
    let mut refused: Value = serde_json::from_str(fixture("modal_refused_no_bound.json")).unwrap();
    let row = refused["refusals"]["rows"][0].clone();
    refused["refusals"]["rows"] = Value::Array((0..1100).map(|i| {
        let mut row = row.clone();
        row["row_id"] = json!(format!("refusal:{i}"));
        row["ordinal"] = json!(i);
        row["code"] = json!("A".repeat(1025));
        row
    }).collect());
    reseal(&mut refused);
    let escapes = format!("[{}]", vec![format!("\"{}\"", "\\u0001".repeat(129)); 1024].join(","));
    tally.check("1,100 long refusal codes beside an escaped request", &serde_json::to_string(&refused).unwrap(), Some(&escapes));
    let mut deep = String::new();
    for level in 0..127 {
        deep.push('{');
        for k in 0..40 {
            deep.push_str(&format!("\"k{level}_{k}\":{k},"));
        }
        deep.push_str("\"z\":");
    }
    deep.push('0');
    deep.push_str(&"}".repeat(127));
    tally.check("127 nested objects of 41 keys", &deep, Some(&deep));

    // Many short keys, as the document and as an undeclared member of one.
    let many_keys = format!("{{{}}}", (0..200_000).map(|i| format!("\"{i:x}\":{i}")).collect::<Vec<_>>().join(","));
    tally.check("200,000 short keys", &many_keys, None);
    let mut with_member = base.clone();
    with_member["binding"]["options"] = serde_json::from_str(&many_keys).unwrap();
    tally.check("200,000 option members", &serde_json::to_string(&with_member).unwrap(), Some(base_request));

    // Long strings: a 4 MB label, a 1 MB label of escaped control characters,
    // a 64 KiB node id in every row that names it.
    let mut long = base.clone();
    long["admission"]["parameter_label"] = json!("L".repeat(4 << 20));
    tally.check("a 4 MB label", &serde_json::to_string(&long).unwrap(), Some(base_request));
    long["admission"]["parameter_label"] = json!("\u{1}".repeat(1 << 20));
    tally.check("a 1 MB escaped label", &serde_json::to_string(&long).unwrap(), Some(base_request));
    let mut long_ids = base.clone();
    add_nodes(&mut long_ids, 2, 64 << 10);
    tally.check("two 64 KiB node ids", &serde_json::to_string(&long_ids).unwrap(), Some(base_request));

    // Many small numbers; hard-to-round floats with 760 digits.
    let numbers = format!("[{}]", vec!["0"; 500_000].join(","));
    tally.check("500,000 zeros", &numbers, Some(&numbers));
    let hard = format!("1.00000000000000011102230246251565404236316680908203125{}1", "0".repeat(700));
    let mut floats = base.clone();
    floats["binding"]["integrity_parameters"]["sturm_gap_log_positions"] = serde_json::from_str(&format!("[{}]", vec![hard.as_str(); 2_000].join(","))).unwrap();
    let text = serde_json::to_string(&floats).unwrap().replace("1.0000000000000002", &hard);
    tally.check("2,000 hard floats", &text, Some(base_request));

    // Large tables: 300 modes (shapes 3,600 rows); 400 more nodes (2,412 DOFs,
    // 7,236 shape rows); and 40,000 resealed shape rows of the wrong count.
    let mut modes = base.clone();
    repeat_modes(&mut modes, 100);
    tally.check("300 modes", &serde_json::to_string(&modes).unwrap(), Some(base_request));
    tally.check("300 modes (no request)", &serde_json::to_string(&modes).unwrap(), None);
    let mut nodes = base.clone();
    add_nodes(&mut nodes, 400, 1);
    tally.check("402 nodes", &serde_json::to_string(&nodes).unwrap(), Some(base_request));
    let mut both = nodes.clone();
    repeat_modes(&mut both, 10);
    tally.check("402 nodes and 30 modes", &serde_json::to_string(&both).unwrap(), Some(base_request));
    let mut wrong = base.clone();
    let row = wrong["tables"]["shapes"]["rows"][0].clone();
    wrong["tables"]["shapes"]["rows"] = Value::Array((0..40_000).map(|i| {
        let mut r = row.clone();
        r["ordinal"] = json!(i);
        r["row_id"] = json!(format!("shape:{i}"));
        r
    }).collect());
    reseal(&mut wrong);
    tally.check("40,000 shape rows of the wrong count", &serde_json::to_string(&wrong).unwrap(), Some(base_request));

    // A refused document with 20,000 refusal rows.
    let mut refused: Value = serde_json::from_str(fixture("modal_refused_no_bound.json")).unwrap();
    let row = refused["refusals"]["rows"][0].clone();
    refused["refusals"]["rows"] = Value::Array((0..20_000).map(|i| {
        let mut r = row.clone();
        r["ordinal"] = json!(i);
        r["row_id"] = json!(format!("refusal:{i}"));
        r["diagnostic_id"] = json!(format!("refusal:{i}"));
        r["message"] = json!("m".repeat(i % 300));
        r
    }).collect());
    reseal(&mut refused);
    tally.check("20,000 refusal rows", &serde_json::to_string(&refused).unwrap(), Some(fixture("modal_refused_no_bound.request.json")));

    // A bound request with 3,000 pipe segments (the model-derived mass and
    // the census run on it), and with 3,000 supports and long ids.
    let mut request: Value = serde_json::from_str(base_request).unwrap();
    let pipe = request["model"]["pipe_segments"][0].clone();
    let pipes: Vec<Value> = (0..3_000).map(|i| {
        let mut p = pipe.clone();
        p["id"] = json!(format!("p{i:0>200}"));
        p
    }).collect();
    request["model"]["pipe_segments"] = Value::Array(pipes);
    let mut bound = base.clone();
    rebind(&mut bound, &request);
    tally.check("3,000 pipes in the bound request", &serde_json::to_string(&bound).unwrap(), Some(&serde_json::to_string(&request).unwrap()));
    let support = request["model"]["supports"][0].clone();
    request["model"]["supports"] = Value::Array((0..3_000).map(|i| {
        let mut s = support.clone();
        s["id"] = json!(format!("s{i:0>200}"));
        s
    }).collect());
    let mut bound = base.clone();
    rebind(&mut bound, &request);
    tally.check("3,000 pipes and supports in the bound request", &serde_json::to_string(&bound).unwrap(), Some(&serde_json::to_string(&request).unwrap()));

    // Texts that are not JSON have no bound (the host refuses them unread).
    for text in ["", "{", "[1,]", "\"\u{1}\""] {
        assert_eq!(modal_reader_working_bound(text, None), None, "{text:?}");
        assert_eq!(modal_reader_working_bound("{}", Some(text)), None, "{text:?}");
    }
    tally.report("adversarial texts");
}
