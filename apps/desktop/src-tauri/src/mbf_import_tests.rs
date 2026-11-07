//! File > Import MBF…'s native path (DEL-17-04 Design `desktop-mbf-export-import.md` §4.8.9, slice 5): the commands'
//! ingress and ledger, task 5 in a real worker process (this test binary run as the worker), the decode and the
//! replies, over the writer's own pair for the maintained invented export model
//! (`fixtures/caepipe_mbf/invented/desktop_export/`, whose recorded digests tie these bytes to what `save_mbf_export`
//! writes), bound and unbound; then the selection refusals and acquisition races, the re-read's change refusals,
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
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

const EXPORT_MODEL: &str = include_str!("../../../../fixtures/caepipe_mbf/invented/desktop_export/export_model.json");
const EXPORT_DIGESTS: &str = include_str!("../../../../fixtures/caepipe_mbf/invented/desktop_export/export_digests.json");

// -- Fixtures and helpers -------------------------------------------------------------------------------------------

fn hex(bytes: &[u8]) -> String {
    ascii(&sha256_hex(bytes)).to_string()
}

/// The writer's MBF and identity file for the invented export model under the desktop's default settings: the bytes
/// `save_mbf_export` writes (their digests are the recorded `degc-default` case's).
fn written_pair() -> (Vec<u8>, Vec<u8>) {
    let settings = mbf::export_settings_from_json(br#"{"title":"Invented MBF export","referenceTemperature":{"value":20,"unit":"degC"},"maxLayoutRows":null}"#).unwrap();
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
    let mut body = body.clone();
    match body.get(EXPECTED_EPOCH) {
        None => body[EXPECTED_EPOCH] = json!(imports.epoch()),
        Some(value) if value.as_str() == Some("absent") => drop(body.as_object_mut().unwrap().remove(EXPECTED_EPOCH)),
        Some(_) => {}
    }
    body
}

fn raw(body: &Value) -> InvokeBody {
    InvokeBody::Raw(serde_json::to_vec(body).unwrap())
}

/// A reply and whether the pool still held the result lease and the reply's lease while the response was built.
struct Replied {
    value: Value,
    held_during_response: u64,
}

fn parse_reply(text: String, registry: &SolveJobRegistry) -> (Value, u64) {
    let held = registry.pool_status().reserved_bytes;
    (serde_json::from_str(&text).unwrap_or_else(|error| panic!("the reply is JSON ({error}): {text}")), held)
}

fn open_with(registry: &SolveJobRegistry, imports: &MbfImports, body: &Value, choose: impl FnOnce() -> Result<Option<PathBuf>, String>) -> Replied {
    let body = ticketed(imports, body);
    let (raw_args, copy) = reserve(registry, &raw(&body), &headers(Some(INGRESS_ARGS_JSON_V1)), OPEN_COMMAND).unwrap();
    let (value, held_during_response) = open_command(registry, imports, raw_args, copy, START_WAIT, choose, |text| parse_reply(text, registry)).unwrap();
    Replied { value, held_during_response }
}

fn open(registry: &SolveJobRegistry, imports: &MbfImports, path: &Path) -> Replied {
    let path = path.to_path_buf();
    open_with(registry, imports, &json!({}), move || Ok(Some(path)))
}

fn review(registry: &SolveJobRegistry, imports: &MbfImports, body: &Value) -> Replied {
    let body = ticketed(imports, body);
    let (raw_args, copy) = reserve(registry, &raw(&body), &headers(Some(INGRESS_ARGS_JSON_V1)), REVIEW_COMMAND).unwrap();
    let (value, held_during_response) = review_command(registry, imports, raw_args, copy, START_WAIT, |text| parse_reply(text, registry)).unwrap();
    Replied { value, held_during_response }
}

fn adopt(registry: &SolveJobRegistry, imports: &MbfImports, body: &Value) -> Replied {
    let body = ticketed(imports, body);
    let (raw_args, copy) = reserve(registry, &raw(&body), &headers(Some(INGRESS_ARGS_JSON_V1)), ADOPT_COMMAND).unwrap();
    let (value, held_during_response) = adopt_command(registry, imports, raw_args, copy, START_WAIT, |text| parse_reply(text, registry)).unwrap();
    Replied { value, held_during_response }
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
        map.insert(name.as_str().unwrap().to_string(), json!({"origin": "project_invented", "source_name": "invented test data"}));
    }
    Value::Object(map)
}

// -- The connected path ---------------------------------------------------------------------------------------------

/// Bound: the saved pair opens, its review is adoptable and bound, and Adopt returns the engine's import of those
/// bytes as the `NativeModelDocumentOutcome`, whose model digest is the review's and whose ids are the exported
/// model's for every class the export writes. The result and reply leases are held while each response is built;
/// nothing is held between commands; the retirement drops the source.
#[test]
fn a_bound_saved_pair_imports_through_task_5_with_the_exported_ids() {
    let registry = real_worker();
    let imports = MbfImports::default();
    let sandbox = Sandbox::new("bound");
    let mbf_path = sandbox.pair(true);
    let (text, sidecar) = written_pair();
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
    assert_eq!(reply["review"]["source"]["identity_file"]["sha256"], hex(&sidecar));
    assert!(!review_text.contains(sandbox.0.to_str().unwrap()), "no path reaches the webview");
    let expected_model = engine_model(&text, Some(&sidecar), None);
    assert_eq!(reply["review"]["model_sha256"], hex(&expected_model));
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
    let exported: Value = serde_json::from_str(EXPORT_MODEL).unwrap();
    for member in ["nodes", "pipe_segments", "components", "supports", "load_cases", "materials", "sections"] {
        let restored = ids_of(&opened_doc["document"], member);
        assert!(!restored.is_empty(), "{member}");
        assert_eq!(restored, ids_of(&exported, member), "{member} ids are restored");
    }
    assert_ne!(opened_doc["document"]["project"]["id"], exported["project"]["id"], "an import is a new project");
    assert_released(&registry);
    // The keyed retirement drops the source; a later re-read is refused retired.
    let retired = retire(&registry, &imports, 1, 1);
    assert_eq!((retired.cancelled, retired.duplicate), (0, false));
    assert_eq!(imports.source_id(), None);
    let after = review(&registry, &imports, &json!({"sourceId": source_id, "declaration": null}));
    assert_eq!(codes(&after.value), vec![SOURCE_RETIRED]);
    assert_released(&registry);
}

/// Unbound: the same MBF without its identity file reviews as needing a declaration per listed material; the declared
/// re-run is adoptable, its ids are formed from MBF names, and the adopted document is the engine's import with that
/// declaration. An identity file that appears afterwards is a change.
#[test]
fn an_unbound_mbf_needs_its_declaration_and_imports_with_ids_formed_from_names() {
    let registry = real_worker();
    let imports = MbfImports::default();
    let sandbox = Sandbox::new("unbound");
    let mbf_path = sandbox.pair(false);
    let (text, sidecar) = written_pair();
    let first = open(&registry, &imports, &mbf_path).value;
    assert_eq!(first["outcome"], "reviewed");
    assert_eq!(first["review"]["adoptable"], false);
    assert_eq!(first["review"]["binding"]["reason"], "no_identity_file");
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
    let adopted = adopt(&registry, &imports, &json!({"sourceId": source_id, "declaration": declaration, "reviewSha256": declared["reviewSha256"]})).value;
    assert_eq!(adopted["outcome"], "opened", "{adopted}");
    let declaration_bytes = serde_json::to_vec(&declaration).unwrap();
    assert_eq!(adopted["document"], serde_json::from_slice::<Value>(&engine_model(&text, None, Some(&declaration_bytes))).unwrap());
    assert!(ids_of(&adopted["document"], "nodes").iter().all(|id| id.starts_with("node:")));
    // An identity file that appears beside the originally absent one is a change.
    fs::write(sandbox.ids(), sidecar).unwrap();
    let changed_reply = review(&registry, &imports, &json!({"sourceId": source_id, "declaration": declaration})).value;
    assert_eq!(codes(&changed_reply), vec![SOURCE_CHANGED]);
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

/// The acquisition race: a regular file checked by `lstat` and replaced by a FIFO before its open. Nonblocking
/// acquisition refuses the non-regular handle at once instead of waiting for a writer (a blocking open would hang this
/// test past its deadline), for the MBF and for the identity file. A cancellation at the same seam installs no source
/// and returns every lease.
#[test]
fn a_fifo_substituted_before_the_open_is_refused_without_waiting_and_a_cancel_there_installs_nothing() {
    let registry = real_worker();
    for identity in [false, true] {
        let imports = MbfImports::default();
        let sandbox = Sandbox::new("fifo-race");
        let mbf_path = sandbox.pair(true);
        let raced = if identity { sandbox.ids() } else { mbf_path.clone() };
        let swapped = raced.clone();
        test_hooks::BEFORE_OPEN.with(|hook| {
            *hook.borrow_mut() = Some(Box::new(move |path: &Path| {
                if path == swapped {
                    fs::remove_file(path).unwrap();
                    mkfifo(path);
                }
            }));
        });
        let began = Instant::now();
        let reply = open(&registry, &imports, &mbf_path).value;
        test_hooks::BEFORE_OPEN.with(|hook| *hook.borrow_mut() = None);
        assert!(began.elapsed() < Duration::from_secs(20), "the open did not wait for a writer");
        assert_eq!(codes(&reply), vec![OPEN_NOT_A_FILE], "identity: {identity}");
        assert_eq!(imports.source_id(), None);
        assert_released(&registry);
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
    let body = ticketed(&imports, &json!({"sourceId": first["sourceId"], "declaration": null}));
    let (text, sidecar) = written_pair();
    let raw_len = request_len("swbpipe-model.mbf".len(), 0, text.len() as u64, Some(sidecar.len() as u64));
    let start = initial_grant(crate::solve_worker::PROCESS_RESERVE_BYTES, resident_envelope(raw_len)).unwrap();
    let scripted = worker_envelope(ImportMode::Review, true);
    for admitted in [false, true] {
        let registry = registry_with(gate.launcher());
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
/// unbound (open, declared review, adopt). Regenerate with `MBF_IMPORT_REPLIES_REGENERATE=1`.
#[test]
fn the_dialogs_import_replies_are_the_native_replies() {
    let registry = real_worker();
    let sandbox = Sandbox::new("replies");
    let mbf_path = sandbox.pair(true);
    let imports = MbfImports::default();
    let bound_open = open(&registry, &imports, &mbf_path).value;
    let bound_adopt = adopt(&registry, &imports, &json!({"sourceId": bound_open["sourceId"], "declaration": null, "reviewSha256": bound_open["reviewSha256"]})).value;
    fs::remove_file(sandbox.ids()).unwrap();
    let imports = MbfImports::default();
    let unbound_open = open(&registry, &imports, &mbf_path).value;
    let declaration = declaration_for(unbound_open["review"]["declaration_needed"].as_array().unwrap());
    let unbound_declared = review(&registry, &imports, &json!({"sourceId": unbound_open["sourceId"], "declaration": declaration})).value;
    let unbound_adopt = adopt(&registry, &imports, &json!({"sourceId": unbound_open["sourceId"], "declaration": declaration, "reviewSha256": unbound_declared["reviewSha256"]})).value;
    assert_eq!(bound_adopt["outcome"], "opened");
    assert_eq!(unbound_adopt["outcome"], "opened");
    let document = json!({
        "description": "Native File > Import MBF… replies (DEL-17-04 §4.7) for the invented export model's saved pair, formed by worker task 5 through the commands; generated by mbf_import_tests.rs the_dialogs_import_replies_are_the_native_replies",
        "declaration": declaration,
        "replies": {
            "bound-open": bound_open,
            "bound-adopt": bound_adopt,
            "unbound-open": unbound_open,
            "unbound-declared": unbound_declared,
            "unbound-adopt": unbound_adopt,
        },
    });
    let text = format!("{}\n", serde_json::to_string_pretty(&document).unwrap());
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join(IMPORT_REPLIES);
    if std::env::var_os("MBF_IMPORT_REPLIES_REGENERATE").is_some() {
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(&path, &text).unwrap();
    }
    assert_eq!(fs::read_to_string(&path).unwrap_or_default(), text, "regenerate with MBF_IMPORT_REPLIES_REGENERATE=1");
    assert_released(&registry);
}
