mod atomic_report_package_save;
mod command_ingress;
mod host_memory;
mod ingress;
mod memory_pool;
mod model_document_file;
mod model_document_save;
mod model_document_shape;
mod model_document_version;
mod model_states;
#[cfg(feature = "native-smoke")]
mod native_smoke;
mod native_result_download;
mod report_package_bridge;
mod solve_jobs;
mod solve_worker;
mod worker_alloc;

pub use solve_worker::{run_solve_worker, WORKER_FLAG as SOLVE_WORKER_FLAG};

use command_ingress::{respond, Args, ArgsDecoding, CommandError, NoCommandError};
use model_document_version::{evaluate_model_document, ModelDocumentVersionStatus};
use open_pipe_stress_library_import_document as library_import_document;
use open_pipe_stress_operation_applier::{apply_operation, validate_operation};
use open_pipe_stress_product_physics::PreviewSolverMode;
#[cfg(test)]
use open_pipe_stress_product_physics::{run_linear_static_preview_with_mode, LinearStaticPreviewRequest};
use open_pipe_stress_rule_check_runner as rule_check_runner;
use open_pipe_stress_rule_pack_document as rule_pack_document;
use open_pipe_stress_units::{catalog_definitions, UnitDefinition};
use rusqlite::{params, Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{
    collections::HashSet,
    fs,
    path::{Path, PathBuf},
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};
use tauri::menu::{CheckMenuItemBuilder, MenuBuilder, MenuItemKind, SubmenuBuilder};
use tauri::{AppHandle, Manager};
use tauri_plugin_dialog::DialogExt;

const PROJECT_STORE_FILE: &str = "openpipestress-projects.sqlite3";

#[derive(Debug, Deserialize)]
struct ReportPackageRedactionEvidence {
    route_id: String,
    decision_count: usize,
    finding_count: usize,
    blocking_count: usize,
}

#[derive(Debug, Deserialize)]
struct ReportPackageLocalFirstEvidence {
    route_id: String,
    export_context: String,
    storage_context: String,
    action: String,
    reason_code: String,
    blocked: bool,
    metadata_only: bool,
    explicit_local_private_intent: bool,
}

fn validate_report_package_local_first_evidence(
    evidence: Option<&ReportPackageLocalFirstEvidence>,
) -> Result<(), &'static str> {
    let Some(evidence) = evidence else {
        return Err(
            "REPORT-PACKAGE-LOCAL-FIRST-EVIDENCE-INVALID: metadata-only local-first route evidence is missing or blocked",
        );
    };
    if evidence.route_id != "DREP-PACKAGE-SAVE-009"
        || evidence.export_context != "local_private"
        || evidence.storage_context != "local_private"
        || evidence.action != "include_metadata_only"
        || evidence.reason_code != "PRIVATE_LOCAL_METADATA_ALLOWED"
        || evidence.blocked
        || !evidence.metadata_only
        || !evidence.explicit_local_private_intent
    {
        return Err(
            "REPORT-PACKAGE-LOCAL-FIRST-EVIDENCE-INVALID: metadata-only local-first route evidence is missing or blocked",
        );
    }
    Ok(())
}

#[derive(Debug, Serialize)]
struct ReportPackageMemberSummary {
    role: String,
    file_name: String,
    sha256_hex: String,
    byte_length: usize,
}

#[derive(Debug, Serialize)]
struct ReportPackageSaveReceipt {
    outcome: String,
    stage: String,
    code: String,
    container_file_name: String,
    byte_count: usize,
    package_identity_sha256_hex: String,
    container_sha256_hex: String,
    members: Vec<ReportPackageMemberSummary>,
    replaced_existing: bool,
    durability: String,
    redaction_route_id: String,
    redaction_decision_count: usize,
    redaction_finding_count: usize,
    redaction_blocking_count: usize,
    local_first_route_id: String,
    local_first_export_context: String,
    local_first_storage_context: String,
    local_first_action: String,
    local_first_reason_code: String,
    local_first_blocked: bool,
    local_first_metadata_only: bool,
    local_first_explicit_local_private_intent: bool,
    selected_basename: String,
    path_containment: String,
    limitation: String,
}

#[derive(Debug, Serialize)]
struct ReportPackageSaveCommandError {
    code: String,
    stage: String,
    message: String,
    os_error_kind: Option<String>,
    os_error_code: Option<i32>,
    cleanup_code: Option<String>,
    cleanup_message: Option<String>,
    selected_basename: String,
    path_containment: String,
    limitation: String,
}

impl ReportPackageSaveCommandError {
    fn bridge(message: String) -> Self {
        Self {
            code: message
                .split(':')
                .next()
                .unwrap_or("REPORT-PACKAGE-BRIDGE-FAILED")
                .to_string(),
            stage: "assemble".to_string(),
            message,
            os_error_kind: None,
            os_error_code: None,
            cleanup_code: None,
            cleanup_message: None,
            selected_basename: String::new(),
            path_containment: atomic_report_package_save::PATH_CONTAINMENT.to_string(),
            limitation: atomic_report_package_save::TOCTOU_LIMITATION.to_string(),
        }
    }
}

impl From<atomic_report_package_save::AtomicSaveError> for ReportPackageSaveCommandError {
    fn from(value: atomic_report_package_save::AtomicSaveError) -> Self {
        Self {
            code: value.code,
            stage: value.stage,
            message: value.message,
            os_error_kind: value.os_error_kind,
            os_error_code: value.os_error_code,
            cleanup_code: value.cleanup_code,
            cleanup_message: value.cleanup_message,
            selected_basename: value.selected_basename,
            path_containment: value.path_containment,
            limitation: value.limitation,
        }
    }
}

#[tauri::command]
// `blocking_save_file` may not execute on Tauri's main thread. Keeping the
// command async places the blocking dialog call on the async command runtime
// and leaves the native modal responsive.
async fn save_report_package(
    app: AppHandle,
    invoke: tauri::ipc::Request<'_>,
    registry: tauri::State<'_, SolveJobRegistry>,
) -> Result<tauri::ipc::Response, CommandError<ReportPackageSaveCommandError>> {
    // The typed request rebuilds its `Value` fields and builds typed vectors.
    let decoding = ArgsDecoding::Rebuilt { vec_expansion: command_ingress::REPORT_PACKAGE_VEC_EXPANSION };
    let (mut args, _lease) = command_ingress::admit(&registry, &invoke, "save_report_package", decoding)?;
    let (request, redaction_evidence, local_first_evidence) = save_report_package_args(&mut args)?;
    drop(args);
    // The lease is held through assembly and the save dialog.
    respond(
        save_report_package_decoded(app, request, redaction_evidence, local_first_evidence)
            .await
            .map_err(CommandError::Command),
    )
}

type ReportPackageArgs =
    (report_package_bridge::ReportPackageRequest, ReportPackageRedactionEvidence, Option<ReportPackageLocalFirstEvidence>);

/// `{request, redactionEvidence, localFirstEvidence?}`.
fn save_report_package_args(args: &mut Args) -> Result<ReportPackageArgs, solve_jobs::SolveJobError> {
    Ok((args.decode("request")?, args.decode("redactionEvidence")?, args.decode("localFirstEvidence")?))
}

async fn save_report_package_decoded(
    app: AppHandle,
    request: report_package_bridge::ReportPackageRequest,
    redaction_evidence: ReportPackageRedactionEvidence,
    local_first_evidence: Option<ReportPackageLocalFirstEvidence>,
) -> Result<ReportPackageSaveReceipt, ReportPackageSaveCommandError> {
    save_report_package_with_picker(request, redaction_evidence, local_first_evidence, |file_name| {
        app.dialog()
            .file()
            .set_file_name(file_name)
            .add_filter("SWBPIPE Project Package", &["opsproj"])
            .blocking_save_file()
            .map(|selected| selected.into_path().map_err(|error| error.to_string()))
    })
}

/// The W-18 slice (a) native hold (DEL-08-01 Design
/// `report-package-current-results.md` §2.6, "Slice (a) hold"). From package
/// version 2.0.0 the shared producer admits Current pressure-free requests,
/// and the TypeScript availability gate is not an IPC boundary. Until slice
/// (b1) installs the content-screened admission, this desktop command refuses
/// every request before evidence use, package assembly, the save dialog or any
/// write. It does not close the shared producer or the headless runner. Slice
/// (b1) removes this hold and its test.
const REPORT_PACKAGE_SAVE_HOLD: &str =
    "REPORT-PACKAGE-FRESH-RESULT-UNAVAILABLE: The report package is unavailable for fresh results.";

fn report_package_save_hold() -> Result<(), ReportPackageSaveCommandError> {
    Err(ReportPackageSaveCommandError::bridge(REPORT_PACKAGE_SAVE_HOLD.to_string()))
}

/// The save after argument decoding; `choose` is the save dialog (a counting
/// seam in tests): `None` is a cancel, `Some(Err)` an unusable path.
fn save_report_package_with_picker(
    request: report_package_bridge::ReportPackageRequest,
    redaction_evidence: ReportPackageRedactionEvidence,
    local_first_evidence: Option<ReportPackageLocalFirstEvidence>,
    choose: impl FnOnce(&str) -> Option<Result<PathBuf, String>>,
) -> Result<ReportPackageSaveReceipt, ReportPackageSaveCommandError> {
    report_package_save_hold()?;
    if redaction_evidence.route_id != "DREP-PACKAGE-SAVE-009"
        || redaction_evidence.blocking_count != 0
    {
        return Err(ReportPackageSaveCommandError::bridge(
            "REPORT-PACKAGE-REDACTION-EVIDENCE-INVALID: controlled route evidence is missing or blocked".to_string(),
        ));
    }
    validate_report_package_local_first_evidence(local_first_evidence.as_ref())
        .map_err(|message| ReportPackageSaveCommandError::bridge(message.to_string()))?;
    let package = report_package_bridge::assemble_request(request)
        .map_err(ReportPackageSaveCommandError::bridge)?;
    if package.export_blocked {
        return Err(ReportPackageSaveCommandError::bridge(format!(
            "REPORT-PACKAGE-PRODUCER-BLOCKED: {}",
            package.blocking_reasons.join(" | ")
        )));
    }

    let Some(selected) = choose(&package.container_file_name) else {
        return Ok(ReportPackageSaveReceipt {
            outcome: "cancelled".to_string(),
            stage: "picker".to_string(),
            code: "REPORT-PACKAGE-SAVE-CANCELLED".to_string(),
            container_file_name: package.container_file_name,
            byte_count: 0,
            package_identity_sha256_hex: package.package_identity_sha256_hex,
            container_sha256_hex: package.container_sha256_hex,
            members: package
                .members
                .into_iter()
                .map(|member| ReportPackageMemberSummary {
                    role: member.role,
                    file_name: member.file_name,
                    sha256_hex: member.sha256_hex,
                    byte_length: member.byte_length,
                })
                .collect(),
            replaced_existing: false,
            durability: "not_applicable".to_string(),
            redaction_route_id: redaction_evidence.route_id,
            redaction_decision_count: redaction_evidence.decision_count,
            redaction_finding_count: redaction_evidence.finding_count,
            redaction_blocking_count: redaction_evidence.blocking_count,
            local_first_route_id: local_first_evidence
                .as_ref()
                .expect("validated local-first evidence")
                .route_id
                .clone(),
            local_first_export_context: local_first_evidence
                .as_ref()
                .expect("validated local-first evidence")
                .export_context
                .clone(),
            local_first_storage_context: local_first_evidence
                .as_ref()
                .expect("validated local-first evidence")
                .storage_context
                .clone(),
            local_first_action: local_first_evidence
                .as_ref()
                .expect("validated local-first evidence")
                .action
                .clone(),
            local_first_reason_code: local_first_evidence
                .as_ref()
                .expect("validated local-first evidence")
                .reason_code
                .clone(),
            local_first_blocked: local_first_evidence
                .as_ref()
                .expect("validated local-first evidence")
                .blocked,
            local_first_metadata_only: local_first_evidence
                .as_ref()
                .expect("validated local-first evidence")
                .metadata_only,
            local_first_explicit_local_private_intent: local_first_evidence
                .as_ref()
                .expect("validated local-first evidence")
                .explicit_local_private_intent,
            selected_basename: String::new(),
            path_containment: atomic_report_package_save::PATH_CONTAINMENT.to_string(),
            limitation: atomic_report_package_save::TOCTOU_LIMITATION.to_string(),
        });
    };
    let destination = selected.map_err(|error| {
        ReportPackageSaveCommandError::bridge(format!(
            "REPORT-PACKAGE-PICKER-PATH-INVALID: {error}"
        ))
    })?;
    let save =
        atomic_report_package_save::atomic_save_bytes(&destination, &package.container_bytes)?;
    Ok(ReportPackageSaveReceipt {
        outcome: "saved".to_string(),
        stage: "complete".to_string(),
        code: if save.durability == "durable" {
            "REPORT-PACKAGE-SAVED".to_string()
        } else {
            "REPORT-PACKAGE-SAVED-DURABILITY-UNCERTAIN".to_string()
        },
        container_file_name: package.container_file_name,
        byte_count: package.container_bytes.len(),
        package_identity_sha256_hex: package.package_identity_sha256_hex,
        container_sha256_hex: package.container_sha256_hex,
        members: package
            .members
            .into_iter()
            .map(|member| ReportPackageMemberSummary {
                role: member.role,
                file_name: member.file_name,
                sha256_hex: member.sha256_hex,
                byte_length: member.byte_length,
            })
            .collect(),
        replaced_existing: save.replaced_existing,
        durability: save.durability,
        redaction_route_id: redaction_evidence.route_id,
        redaction_decision_count: redaction_evidence.decision_count,
        redaction_finding_count: redaction_evidence.finding_count,
        redaction_blocking_count: redaction_evidence.blocking_count,
        local_first_route_id: local_first_evidence
            .as_ref()
            .expect("validated local-first evidence")
            .route_id
            .clone(),
        local_first_export_context: local_first_evidence
            .as_ref()
            .expect("validated local-first evidence")
            .export_context
            .clone(),
        local_first_storage_context: local_first_evidence
            .as_ref()
            .expect("validated local-first evidence")
            .storage_context
            .clone(),
        local_first_action: local_first_evidence
            .as_ref()
            .expect("validated local-first evidence")
            .action
            .clone(),
        local_first_reason_code: local_first_evidence
            .as_ref()
            .expect("validated local-first evidence")
            .reason_code
            .clone(),
        local_first_blocked: local_first_evidence
            .as_ref()
            .expect("validated local-first evidence")
            .blocked,
        local_first_metadata_only: local_first_evidence
            .as_ref()
            .expect("validated local-first evidence")
            .metadata_only,
        local_first_explicit_local_private_intent: local_first_evidence
            .as_ref()
            .expect("validated local-first evidence")
            .explicit_local_private_intent,
        selected_basename: save.selected_basename,
        path_containment: save.path_containment,
        limitation: save.limitation,
    })
}

/// The bundled invented demo model and its design knowledge, compiled into the
/// binary (F-PIP-1: invented bundled fixtures only). The app starts on the demo;
/// any other model document reaches the session through `open_model_document_file`.
const BUNDLED_DEMO_MODEL: &str =
    include_str!("../../../../fixtures/product_preview/invented_demo_model.json");
const BUNDLED_DESIGN_KNOWLEDGE: &str =
    include_str!("../../../../fixtures/product_preview/invented_design_knowledge.json");

fn bundled_document(text: &str) -> Result<Value, String> {
    serde_json::from_str(text).map_err(|error| error.to_string())
}

/// Test-only reader for committed fixtures, resolved from this crate's manifest
/// directory rather than the process working directory.
#[cfg(test)]
fn read_fixture(file_name: &str) -> Result<Value, String> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../../fixtures/product_preview")
        .join(file_name);
    let text = fs::read_to_string(&path).map_err(|error| format!("{}: {error}", path.display()))?;
    serde_json::from_str(&text).map_err(|error| error.to_string())
}

#[derive(Debug, Serialize)]
struct UnitCatalogResponse {
    schema_version: &'static str,
    catalog_id: &'static str,
    decision_basis: &'static str,
    calculation_basis: &'static str,
    storage_convention: &'static str,
    entry_count: usize,
    entries: Vec<UnitCatalogEntry>,
    boundary: UnitCatalogBoundary,
}

#[derive(Debug, Serialize)]
struct UnitCatalogEntry {
    unit_id: &'static str,
    symbol: &'static str,
    dimension_id: &'static str,
    canonical: bool,
    transform_kind: &'static str,
    factor_representation: &'static str,
    offset_representation: Option<&'static str>,
    provenance: &'static str,
    review_status: &'static str,
}

#[derive(Debug, Serialize)]
struct UnitCatalogBoundary {
    source: &'static str,
    protected_content_included: bool,
    private_project_data_included: bool,
    professional_approval_claimed: bool,
    code_compliance_claimed: bool,
}

fn unit_transform_kind(definition: &UnitDefinition) -> &'static str {
    if definition.canonical {
        "identity"
    } else {
        definition.transform_to_canonical.kind.as_schema_value()
    }
}

fn unit_catalog_entry(definition: UnitDefinition) -> UnitCatalogEntry {
    UnitCatalogEntry {
        unit_id: definition.id.catalog_id(),
        symbol: definition.symbol,
        dimension_id: definition.dimension.as_str(),
        canonical: definition.canonical,
        transform_kind: unit_transform_kind(&definition),
        factor_representation: definition.factor_representation,
        offset_representation: definition.offset_representation,
        provenance: definition.provenance.as_schema_value(),
        review_status: definition.review_status.as_schema_value(),
    }
}

fn desktop_unit_catalog() -> UnitCatalogResponse {
    let entries = catalog_definitions()
        .map(unit_catalog_entry)
        .collect::<Vec<_>>();
    UnitCatalogResponse {
        schema_version: "0.1.0",
        catalog_id: "unit-system:dec-018-si-dual-display",
        decision_basis: "DEC-018",
        calculation_basis: "si_canonical",
        storage_convention: "entered_units_preserved",
        entry_count: entries.len(),
        entries,
        boundary: UnitCatalogBoundary {
            source: "core/units open_pipe_stress_units catalog",
            protected_content_included: false,
            private_project_data_included: false,
            professional_approval_claimed: false,
            code_compliance_claimed: false,
        },
    }
}

#[derive(Debug, Serialize)]
struct StorageCapability {
    engine: &'static str,
    bundled: bool,
    fts5_available: bool,
    network_required: bool,
    daemon_required: bool,
    telemetry_enabled: bool,
    path_policy: &'static str,
    large_file_policy: &'static str,
    database_path: String,
    compile_options: Vec<String>,
    migration_framework: &'static str,
    migration_status: String,
    store_schema_version: i64,
    store_schema_target_version: i64,
    migrations_applied_on_open: Vec<String>,
}

#[derive(Debug, Serialize)]
struct LocalProjectSummary {
    project_id: String,
    project_name: String,
    database_path: String,
    storage_mode: &'static str,
    migration_status: String,
    migration_framework: &'static str,
    store_schema_version: i64,
    store_schema_target_version: i64,
    migrations_applied_on_open: Vec<String>,
    fts_indexed: bool,
    copied_external_files: bool,
    editor_intent_count: usize,
    proposal_count: usize,
    selected_review_target_count: usize,
    selected_review_target_ref: String,
    persisted_mechanics_result_count: usize,
    persisted_analysis_run_count: usize,
    persisted_analysis_run_ref: String,
    persisted_model_hash_count: usize,
    persisted_model_hash_ref: String,
    persisted_project_envelope_hash_count: usize,
    persisted_project_envelope_hash_ref: String,
    unit_round_trip_status: String,
    unit_round_trip_checked_ref_count: usize,
    unit_round_trip_signature: String,
    message: String,
}

#[derive(Debug, Serialize)]
struct LocalProjectIndexEntry {
    project_id: String,
    project_name: String,
    storage_mode: &'static str,
    created_at_unix: i64,
    updated_at_unix: i64,
}

#[derive(Debug, Serialize)]
struct LocalProjectEnvelope {
    summary: LocalProjectSummary,
    model: Value,
    editor_intents: Value,
    proposal: Value,
    selected_review_target: Value,
    mechanics_result: Value,
    analysis_run: Value,
    model_hash: Value,
    project_envelope_hash: Value,
    /// The document's version status (DEL-02-05 CLM-003): always `current` for an
    /// envelope the store returns, since every other version is refused.
    model_document_version: Value,
}

/// The `save_local_project` request. Decoded by
/// [`SaveLocalProjectRequest::from_value`], which moves the documents it
/// carries out of the parsed body instead of rebuilding them.
#[derive(Debug)]
struct SaveLocalProjectRequest {
    project_id: String,
    project_name: String,
    model: Value,
    editor_intents: Value,
    proposal: Value,
    selected_review_target: Value,
    mechanics_result: Value,
    analysis_run: Value,
    model_hash: Value,
    project_envelope_hash: Value,
    /// The stored revision the user chose to Replace in the prompt, if any.
    /// See `refuse_unconfirmed_replacement`.
    confirmed_revision: Option<String>,
    /// The stored envelope hash value this session last opened or saved, sent
    /// when the session is that stored project (an ordinary save in place).
    expected_base: Option<String>,
}

impl SaveLocalProjectRequest {
    /// The serde semantics the derived decoding had (required `project_id`,
    /// `project_name` and `model`; every other field defaults to `null` or
    /// `None`; unknown fields ignored), with every field moved.
    fn from_value(value: Value) -> Result<Self, command_ingress::ArgsError> {
        let mut fields = command_ingress::Fields::of(value, "struct SaveLocalProjectRequest")?;
        Ok(Self {
            project_id: fields.required("project_id")?,
            project_name: fields.required("project_name")?,
            model: fields.required_value("model")?,
            editor_intents: fields.value_or_null("editor_intents"),
            proposal: fields.value_or_null("proposal"),
            selected_review_target: fields.value_or_null("selected_review_target"),
            mechanics_result: fields.value_or_null("mechanics_result"),
            analysis_run: fields.value_or_null("analysis_run"),
            model_hash: fields.value_or_null("model_hash"),
            project_envelope_hash: fields.value_or_null("project_envelope_hash"),
            confirmed_revision: fields.optional("confirmed_revision")?,
            expected_base: fields.optional("expected_base")?,
        })
    }
}

#[derive(Debug, Deserialize)]
struct SelectedReviewTarget {
    target_type: String,
    id: String,
}

fn app_store_path(app: &AppHandle) -> Result<PathBuf, String> {
    let dir = app
        .path()
        .app_local_data_dir()
        .map_err(|error| error.to_string())?;
    fs::create_dir_all(&dir).map_err(|error| error.to_string())?;
    // DEC-101 persistence compatibility: the bundle identifier was renamed, and
    // the local data directory is keyed by it. Carry a store left under the
    // former identifier forward once. A failed carry-forward never blocks
    // startup: the product falls through to an empty store at the new location.
    if let Err(error) = carry_forward_legacy_project_store(&dir) {
        eprintln!("legacy project store was not carried forward: {error}");
    }
    Ok(dir.join(PROJECT_STORE_FILE))
}

/// The bundle identifier the product carried before the DEC-101 rename.
const LEGACY_BUNDLE_IDENTIFIER: &str = "org.openpipestress.technical-preview";
const STORE_SIDECAR_SUFFIXES: [&str; 2] = ["-wal", "-shm"];

fn store_sidecar_path(store: &Path, suffix: &str) -> PathBuf {
    let mut name = store.as_os_str().to_os_string();
    name.push(suffix);
    PathBuf::from(name)
}

/// Pure decision over paths: the legacy store to copy from, or `None`.
/// `None` when the new location already holds a store (never overwritten),
/// when the new directory has no parent, when the new directory is itself the
/// legacy directory, or when no legacy store exists. The legacy directory is
/// the new directory's sibling named by the former identifier, so no platform
/// path is hard-coded.
fn legacy_project_store_source(new_dir: &Path) -> Option<PathBuf> {
    if new_dir.join(PROJECT_STORE_FILE).exists() {
        return None;
    }
    let legacy_dir = new_dir.parent()?.join(LEGACY_BUNDLE_IDENTIFIER);
    if legacy_dir == new_dir {
        return None;
    }
    let legacy_store = legacy_dir.join(PROJECT_STORE_FILE);
    legacy_store.is_file().then_some(legacy_store)
}

const CARRY_FORWARD_TEMPORARY_SUFFIX: &str = ".carry-forward-partial";

/// Carries a legacy store forward when `legacy_project_store_source` says so.
/// The legacy store is opened read-only and SQLite writes one consistent copy
/// with `VACUUM INTO` a temporary sibling, which is then renamed into place.
/// The read takes a proper lock, so a store in WAL mode (including one the
/// former app still has open) is carried with its committed, uncheckpointed
/// rows, and no sidecar file is ever copied. The legacy store is never moved,
/// deleted or modified, and its journal mode is not changed. A store already
/// at the new location is never overwritten. Returns whether a store was
/// carried forward.
fn carry_forward_legacy_project_store(new_dir: &Path) -> Result<bool, String> {
    let Some(legacy_store) = legacy_project_store_source(new_dir) else {
        return Ok(false);
    };
    let new_store = new_dir.join(PROJECT_STORE_FILE);
    let temporary = store_sidecar_path(&new_store, CARRY_FORWARD_TEMPORARY_SUFFIX);

    // Leftovers of a killed earlier attempt. No main store exists at the new
    // location (the decision function checked), so a temporary file or a
    // sidecar here is never user data, and a stale sidecar must not be paired
    // with the fresh main file.
    let mut leftovers = vec![temporary.clone()];
    for suffix in STORE_SIDECAR_SUFFIXES {
        leftovers.push(store_sidecar_path(&new_store, suffix));
        leftovers.push(store_sidecar_path(&temporary, suffix));
    }
    leftovers.push(store_sidecar_path(&temporary, "-journal"));
    for leftover in &leftovers {
        if leftover.exists() {
            fs::remove_file(leftover).map_err(|error| error.to_string())?;
        }
    }

    let outcome = (|| -> Result<(), String> {
        let legacy = Connection::open_with_flags(
            &legacy_store,
            rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY | rusqlite::OpenFlags::SQLITE_OPEN_NO_MUTEX,
        )
        .map_err(|error| error.to_string())?;
        legacy
            .busy_timeout(std::time::Duration::from_secs(5))
            .map_err(|error| error.to_string())?;
        let target = temporary
            .to_str()
            .ok_or_else(|| "the new store path is not valid UTF-8".to_string())?;
        legacy
            .execute("VACUUM INTO ?1", [target])
            .map_err(|error| error.to_string())?;
        drop(legacy);
        if new_store.exists() {
            return Err("a store appeared at the new location during the copy".to_string());
        }
        fs::rename(&temporary, &new_store).map_err(|error| error.to_string())
    })();
    if let Err(error) = outcome {
        // Remove only what this attempt created; the legacy store and any store
        // at the new location are never touched.
        for leftover in &leftovers {
            if leftover != &store_sidecar_path(&new_store, "-wal")
                && leftover != &store_sidecar_path(&new_store, "-shm")
            {
                let _ = fs::remove_file(leftover);
            }
        }
        return Err(error);
    }
    Ok(true)
}

const STORE_SCHEMA_TARGET_VERSION: i64 = 12;
const STORE_MIGRATION_FRAMEWORK: &str = "versioned_sqlite_user_version_migration_ledger";

struct StoreMigration {
    version: i64,
    migration_id: &'static str,
    apply: fn(&Connection) -> Result<(), String>,
}

#[derive(Debug, Clone, Serialize)]
struct StoreMigrationEvidence {
    migration_framework: &'static str,
    store_schema_version_before_open: i64,
    store_schema_version: i64,
    store_schema_target_version: i64,
    migrations_applied_on_open: Vec<String>,
    pending_migration_count: i64,
    migration_status: String,
}

fn store_migrations() -> Vec<StoreMigration> {
    vec![
        StoreMigration {
            version: 1,
            migration_id: "store-v1-base-project-and-fts-tables",
            apply: migrate_v1_base_tables,
        },
        StoreMigration {
            version: 2,
            migration_id: "store-v2-editor-intents-column",
            apply: |connection| {
                ensure_column(
                    connection,
                    "editor_intents_json",
                    "ALTER TABLE local_projects ADD COLUMN editor_intents_json TEXT NOT NULL DEFAULT '[]'",
                )
            },
        },
        StoreMigration {
            version: 3,
            migration_id: "store-v3-proposal-column",
            apply: |connection| {
                ensure_column(
                    connection,
                    "proposal_json",
                    "ALTER TABLE local_projects ADD COLUMN proposal_json TEXT NOT NULL DEFAULT 'null'",
                )
            },
        },
        StoreMigration {
            version: 4,
            migration_id: "store-v4-selected-review-target-column",
            apply: |connection| {
                ensure_column(
                    connection,
                    "selected_review_target_json",
                    "ALTER TABLE local_projects ADD COLUMN selected_review_target_json TEXT NOT NULL DEFAULT 'null'",
                )
            },
        },
        StoreMigration {
            version: 5,
            migration_id: "store-v5-mechanics-result-column",
            apply: |connection| {
                ensure_column(
                    connection,
                    "mechanics_result_json",
                    "ALTER TABLE local_projects ADD COLUMN mechanics_result_json TEXT NOT NULL DEFAULT 'null'",
                )
            },
        },
        StoreMigration {
            version: 6,
            migration_id: "store-v6-analysis-run-column",
            apply: |connection| {
                ensure_column(
                    connection,
                    "analysis_run_json",
                    "ALTER TABLE local_projects ADD COLUMN analysis_run_json TEXT NOT NULL DEFAULT 'null'",
                )
            },
        },
        StoreMigration {
            version: 7,
            migration_id: "store-v7-model-hash-column",
            apply: |connection| {
                ensure_column(
                    connection,
                    "model_hash_json",
                    "ALTER TABLE local_projects ADD COLUMN model_hash_json TEXT NOT NULL DEFAULT 'null'",
                )
            },
        },
        StoreMigration {
            version: 8,
            migration_id: "store-v8-project-envelope-hash-column",
            apply: |connection| {
                ensure_column(
                    connection,
                    "project_envelope_hash_json",
                    "ALTER TABLE local_projects ADD COLUMN project_envelope_hash_json TEXT NOT NULL DEFAULT 'null'",
                )
            },
        },
        // Evidence-only ledger of applied model-document migrations (DEC-019).
        // The model-document schema *version* authority stays in-document.
        StoreMigration {
            version: 9,
            migration_id: "store-v9-model-document-migration-ledger-column",
            apply: |connection| {
                ensure_column(
                    connection,
                    "model_migration_ledger_json",
                    "ALTER TABLE local_projects ADD COLUMN model_migration_ledger_json TEXT NOT NULL DEFAULT '[]'",
                )
            },
        },
        // Local, user-controlled rule-pack documents (Phase C2). Private rule
        // packs stay in this local store and are never committed to the
        // repository or transmitted (OPS-K-PRIV-1).
        StoreMigration {
            version: 10,
            migration_id: "store-v10-local-rule-packs-table",
            apply: |connection| {
                connection
                    .execute_batch(
                        "
                        CREATE TABLE IF NOT EXISTS local_rule_packs (
                            project_id TEXT NOT NULL,
                            rule_pack_id TEXT NOT NULL,
                            document_json TEXT NOT NULL,
                            created_at_unix INTEGER NOT NULL,
                            updated_at_unix INTEGER NOT NULL,
                            PRIMARY KEY (project_id, rule_pack_id)
                        );
                        ",
                    )
                    .map_err(|error| error.to_string())
            },
        },
        // Local, user-controlled imported private libraries (Phase C3). Only
        // accepted (PRIVATE_LOCAL_ONLY) material/section/component imports are
        // written here; suspected-protected or blocked imports are never
        // stored. Private libraries stay in this local store and are never
        // committed to the repository or transmitted (OPS-K-PRIV-1, PRD §13.5).
        StoreMigration {
            version: 11,
            migration_id: "store-v11-local-libraries-table",
            apply: |connection| {
                connection
                    .execute_batch(
                        "
                        CREATE TABLE IF NOT EXISTS local_libraries (
                            project_id TEXT NOT NULL,
                            library_kind TEXT NOT NULL,
                            library_id TEXT NOT NULL,
                            document_json TEXT NOT NULL,
                            created_at_unix INTEGER NOT NULL,
                            updated_at_unix INTEGER NOT NULL,
                            PRIMARY KEY (project_id, library_kind, library_id)
                        );
                        ",
                    )
                    .map_err(|error| error.to_string())
            },
        },
        // Named immutable model states and their labels (W-12 S1, DEL-14-01
        // Design `desktop-states-runs-comparison.md` §2). Recorded runs are S2's v13.
        StoreMigration {
            version: 12,
            migration_id: "store-v12-model-states-tables",
            apply: model_states::migrate_v12_model_states,
        },
    ]
}

fn migrate_v1_base_tables(connection: &Connection) -> Result<(), String> {
    connection
        .execute_batch(
            "
            CREATE TABLE IF NOT EXISTS local_projects (
                project_id TEXT PRIMARY KEY,
                project_name TEXT NOT NULL,
                model_json TEXT NOT NULL,
                created_at_unix INTEGER NOT NULL,
                updated_at_unix INTEGER NOT NULL
            );
            CREATE VIRTUAL TABLE IF NOT EXISTS local_project_fts USING fts5(
                project_id UNINDEXED,
                project_name,
                model_text
            );
            ",
        )
        .map_err(|error| error.to_string())
}

fn store_user_version(connection: &Connection) -> Result<i64, String> {
    connection
        .query_row("PRAGMA user_version", [], |row| row.get(0))
        .map_err(|error| error.to_string())
}

fn open_project_store(path: &Path) -> Result<(Connection, StoreMigrationEvidence), String> {
    let connection = Connection::open(path).map_err(|error| error.to_string())?;
    // A second writer waits this long for the store's write lock, then fails
    // visibly ("database is locked") rather than writing.
    connection
        .busy_timeout(std::time::Duration::from_secs(5))
        .map_err(|error| error.to_string())?;
    let migration = apply_store_migrations(&connection)?;
    Ok((connection, migration))
}

/// One current store version: a store whose `user_version` is above this build's
/// target was written by a newer build. It is refused by name before anything
/// is written (no pragma that writes, no migration), never opened as current.
fn refuse_newer_store(version: i64) -> Result<(), String> {
    if version > STORE_SCHEMA_TARGET_VERSION {
        return Err(format!(
            "STORE-SCHEMA-NEWER: the local project store is at store schema v{version}, newer than this build's v{STORE_SCHEMA_TARGET_VERSION}; it was written by a newer SWBPIPE build. Nothing was read or written; open it with that build."
        ));
    }
    Ok(())
}

// Each migration step is idempotent (IF NOT EXISTS / column probes), so legacy
// stores written before the user_version ledger existed reconcile cleanly from
// version 0 without data loss.
fn apply_store_migrations(connection: &Connection) -> Result<StoreMigrationEvidence, String> {
    refuse_newer_store(store_user_version(connection)?)?;
    connection
        .execute_batch(
            "
            PRAGMA foreign_keys = ON;
            PRAGMA journal_mode = WAL;
            ",
        )
        .map_err(|error| error.to_string())?;
    let version_before = store_user_version(connection)?;
    let mut migrations_applied = Vec::new();
    for migration in store_migrations() {
        if migration.version > version_before {
            (migration.apply)(connection)?;
            connection
                .execute_batch(&format!("PRAGMA user_version = {}", migration.version))
                .map_err(|error| error.to_string())?;
            migrations_applied.push(migration.migration_id.to_string());
        }
    }
    let version_after = store_user_version(connection)?;
    let migration_status = if migrations_applied.is_empty() {
        format!("current_store_schema_v{version_after}_no_pending_migrations")
    } else {
        format!("migrated_on_open_store_schema_v{version_before}_to_v{version_after}")
    };
    Ok(StoreMigrationEvidence {
        migration_framework: STORE_MIGRATION_FRAMEWORK,
        store_schema_version_before_open: version_before,
        store_schema_version: version_after,
        store_schema_target_version: STORE_SCHEMA_TARGET_VERSION,
        migrations_applied_on_open: migrations_applied,
        pending_migration_count: STORE_SCHEMA_TARGET_VERSION - version_after,
        migration_status,
    })
}

fn ensure_column(
    connection: &Connection,
    column_name: &str,
    alter_sql: &str,
) -> Result<(), String> {
    let mut statement = connection
        .prepare("PRAGMA table_info(local_projects)")
        .map_err(|error| error.to_string())?;
    let column_names = statement
        .query_map([], |row| row.get::<_, String>(1))
        .map_err(|error| error.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| error.to_string())?;
    if !column_names.iter().any(|name| name == column_name) {
        connection
            .execute(alter_sql, [])
            .map_err(|error| error.to_string())?;
    }
    Ok(())
}

fn sqlite_compile_options(connection: &Connection) -> Result<Vec<String>, String> {
    let mut statement = connection
        .prepare("PRAGMA compile_options")
        .map_err(|error| error.to_string())?;
    let options = statement
        .query_map([], |row| row.get::<_, String>(0))
        .map_err(|error| error.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| error.to_string())?;
    Ok(options)
}

fn fts5_available(connection: &Connection) -> bool {
    connection
        .execute_batch(
            "
            CREATE VIRTUAL TABLE IF NOT EXISTS temp.openpipestress_fts_probe USING fts5(value);
            DROP TABLE temp.openpipestress_fts_probe;
            ",
        )
        .is_ok()
}

fn now_unix_seconds() -> Result<i64, String> {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|error| error.to_string())?;
    Ok(duration.as_secs() as i64)
}

fn project_name_from_model(model: &Value) -> String {
    model
        .get("project")
        .and_then(|project| project.get("name"))
        .and_then(Value::as_str)
        .filter(|name| !name.trim().is_empty())
        .unwrap_or("Untitled SWBPIPE Project")
        .trim()
        .to_string()
}

fn project_id_from_model(model: &Value) -> String {
    model
        .get("project")
        .and_then(|project| project.get("id"))
        .and_then(Value::as_str)
        .filter(|id| !id.trim().is_empty())
        .unwrap_or("project:untitled-local")
        .trim()
        .to_string()
}

/// Write one project row and its search row (unguarded). Used by tests, the
/// packaged self-test and the smoke harness; the app's commands write through
/// `guarded_write_project_rows` on their own `BEGIN IMMEDIATE` transaction.
fn upsert_project(
    connection: &mut Connection,
    project_id: &str,
    project_name: &str,
    model: &Value,
    editor_intents: &Value,
    proposal: &Value,
    selected_review_target: &Value,
    mechanics_result: &Value,
    analysis_run: &Value,
    model_hash: &Value,
    project_envelope_hash: &Value,
) -> Result<(), String> {
    let transaction = immediate_transaction(connection)?;
    write_project_rows(&transaction, project_id, project_name, model, editor_intents, proposal, selected_review_target,
        mechanics_result, analysis_run, model_hash, project_envelope_hash)?;
    transaction.commit().map_err(|error| error.to_string())
}

/// The replacement check and the write, on a transaction the caller opened
/// with `BEGIN IMMEDIATE` (HELP_HUMAN rulings 2026-10-10). The store's write lock
/// is held from before the caller's preparation reads through the check and the
/// write, so two writers (two windows or processes) cannot both pass the check
/// for the same id: the second waits, then sees the first's row and is refused.
fn guarded_write_project_rows(
    transaction: &rusqlite::Transaction<'_>,
    confirmed_revision: Option<&str>,
    expected_base: Option<&str>,
    project_id: &str,
    project_name: &str,
    model: &Value,
    editor_intents: &Value,
    proposal: &Value,
    selected_review_target: &Value,
    mechanics_result: &Value,
    analysis_run: &Value,
    model_hash: &Value,
    project_envelope_hash: &Value,
) -> Result<(), String> {
    refuse_unconfirmed_replacement(transaction, project_id, confirmed_revision, expected_base)?;
    write_project_rows(transaction, project_id, project_name, model, editor_intents, proposal, selected_review_target,
        mechanics_result, analysis_run, model_hash, project_envelope_hash)
}

fn immediate_transaction(connection: &mut Connection) -> Result<rusqlite::Transaction<'_>, String> {
    connection
        .transaction_with_behavior(rusqlite::TransactionBehavior::Immediate)
        .map_err(|error| error.to_string())
}

fn write_project_rows(
    transaction: &rusqlite::Transaction<'_>,
    project_id: &str,
    project_name: &str,
    model: &Value,
    editor_intents: &Value,
    proposal: &Value,
    selected_review_target: &Value,
    mechanics_result: &Value,
    analysis_run: &Value,
    model_hash: &Value,
    project_envelope_hash: &Value,
) -> Result<(), String> {
    let model_json = serde_json::to_string(model).map_err(|error| error.to_string())?;
    let editor_intents_json =
        serde_json::to_string(editor_intents).map_err(|error| error.to_string())?;
    let proposal_json = serde_json::to_string(proposal).map_err(|error| error.to_string())?;
    let selected_review_target_json =
        serde_json::to_string(selected_review_target).map_err(|error| error.to_string())?;
    let mechanics_result_json =
        serde_json::to_string(mechanics_result).map_err(|error| error.to_string())?;
    let analysis_run_json =
        serde_json::to_string(analysis_run).map_err(|error| error.to_string())?;
    let model_hash_json = serde_json::to_string(model_hash).map_err(|error| error.to_string())?;
    let project_envelope_hash_json =
        serde_json::to_string(project_envelope_hash).map_err(|error| error.to_string())?;
    let search_text = format!(
        "{model_json}\n{editor_intents_json}\n{proposal_json}\n{selected_review_target_json}\n{analysis_run_json}"
    );
    let now = now_unix_seconds()?;
    transaction
        .execute(
            "
            INSERT INTO local_projects (project_id, project_name, model_json, editor_intents_json, proposal_json, selected_review_target_json, mechanics_result_json, analysis_run_json, model_hash_json, project_envelope_hash_json, created_at_unix, updated_at_unix)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?11)
            ON CONFLICT(project_id) DO UPDATE SET
                project_name = excluded.project_name,
                model_json = excluded.model_json,
                editor_intents_json = excluded.editor_intents_json,
                proposal_json = excluded.proposal_json,
                selected_review_target_json = excluded.selected_review_target_json,
                mechanics_result_json = excluded.mechanics_result_json,
                analysis_run_json = excluded.analysis_run_json,
                model_hash_json = excluded.model_hash_json,
                project_envelope_hash_json = excluded.project_envelope_hash_json,
                updated_at_unix = excluded.updated_at_unix
            ",
            params![
                project_id,
                project_name,
                model_json,
                editor_intents_json,
                proposal_json,
                selected_review_target_json,
                mechanics_result_json,
                analysis_run_json,
                model_hash_json,
                project_envelope_hash_json,
                now
            ],
        )
        .map_err(|error| error.to_string())?;
    transaction
        .execute(
            "DELETE FROM local_project_fts WHERE project_id = ?1",
            params![project_id],
        )
        .map_err(|error| error.to_string())?;
    transaction
        .execute(
            "
            INSERT INTO local_project_fts (project_id, project_name, model_text)
            VALUES (?1, ?2, ?3)
            ",
            params![project_id, project_name, search_text],
        )
        .map_err(|error| error.to_string())?;
    Ok(())
}

struct StoredProjectRecord {
    project_id: String,
    project_name: String,
    model: Value,
    editor_intents: Value,
    proposal: Value,
    selected_review_target: Value,
    mechanics_result: Value,
    analysis_run: Value,
    model_hash: Value,
    project_envelope_hash: Value,
}

fn load_project(
    connection: &Connection,
    project_id: Option<&str>,
) -> Result<Option<StoredProjectRecord>, String> {
    type StoredRow = (
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
    );
    fn stored_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<StoredRow> {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?,
            row.get::<_, String>(3)?,
            row.get::<_, String>(4)?,
            row.get::<_, String>(5)?,
            row.get::<_, String>(6)?,
            row.get::<_, String>(7)?,
            row.get::<_, String>(8)?,
            row.get::<_, String>(9)?,
        ))
    }
    let row = match project_id {
        Some(id) => connection
            .query_row(
                "
                SELECT project_id, project_name, model_json, editor_intents_json, proposal_json, selected_review_target_json, mechanics_result_json, analysis_run_json, model_hash_json, project_envelope_hash_json
                FROM local_projects
                WHERE project_id = ?1
                ",
                params![id],
                stored_row,
            )
            .optional()
            .map_err(|error| error.to_string())?,
        None => connection
            .query_row(
                "
                SELECT project_id, project_name, model_json, editor_intents_json, proposal_json, selected_review_target_json, mechanics_result_json, analysis_run_json, model_hash_json, project_envelope_hash_json
                FROM local_projects
                ORDER BY updated_at_unix DESC, project_id ASC
                LIMIT 1
                ",
                [],
                stored_row,
            )
            .optional()
            .map_err(|error| error.to_string())?,
    };
    row.map(
        |(
            id,
            name,
            model_json,
            editor_intents_json,
            proposal_json,
            selected_review_target_json,
            mechanics_result_json,
            analysis_run_json,
            model_hash_json,
            project_envelope_hash_json,
        )| {
            let model =
                serde_json::from_str::<Value>(&model_json).map_err(|error| error.to_string())?;
            let editor_intents = serde_json::from_str::<Value>(&editor_intents_json)
                .map_err(|error| error.to_string())?;
            let proposal =
                serde_json::from_str::<Value>(&proposal_json).map_err(|error| error.to_string())?;
            let selected_review_target =
                serde_json::from_str::<Value>(&selected_review_target_json)
                    .map_err(|error| error.to_string())?;
            let mechanics_result = serde_json::from_str::<Value>(&mechanics_result_json)
                .map_err(|error| error.to_string())?;
            let analysis_run = serde_json::from_str::<Value>(&analysis_run_json)
                .map_err(|error| error.to_string())?;
            let model_hash = serde_json::from_str::<Value>(&model_hash_json)
                .map_err(|error| error.to_string())?;
            let project_envelope_hash = serde_json::from_str::<Value>(&project_envelope_hash_json)
                .map_err(|error| error.to_string())?;
            Ok(StoredProjectRecord {
                project_id: id,
                project_name: name,
                model,
                editor_intents,
                proposal,
                selected_review_target,
                mechanics_result,
                analysis_run,
                model_hash,
                project_envelope_hash,
            })
        },
    )
    .transpose()
}

fn list_projects(connection: &Connection) -> Result<Vec<LocalProjectIndexEntry>, String> {
    let mut statement = connection
        .prepare(
            "
            SELECT project_id, project_name, created_at_unix, updated_at_unix
            FROM local_projects
            ORDER BY updated_at_unix DESC, project_id ASC
            ",
        )
        .map_err(|error| error.to_string())?;
    let entries = statement
        .query_map([], |row| {
            Ok(LocalProjectIndexEntry {
                project_id: row.get::<_, String>(0)?,
                project_name: row.get::<_, String>(1)?,
                storage_mode: "local_sqlite",
                created_at_unix: row.get::<_, i64>(2)?,
                updated_at_unix: row.get::<_, i64>(3)?,
            })
        })
        .map_err(|error| error.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| error.to_string())?;
    Ok(entries)
}

fn stored_project_entry(connection: &Connection, project_id: &str) -> Result<Option<LocalProjectIndexEntry>, String> {
    Ok(list_projects(connection)?.into_iter().find(|entry| entry.project_id == project_id))
}

/// The stored project's envelope hash value, "" when it records none.
fn stored_envelope_hash_value(connection: &Connection, project_id: &str) -> Result<String, String> {
    let text: Option<String> = connection
        .query_row(
            "SELECT project_envelope_hash_json FROM local_projects WHERE project_id = ?1",
            params![project_id],
            |row| row.get(0),
        )
        .optional()
        .map_err(|error| error.to_string())?;
    let hash: Value = text
        .map(|text| serde_json::from_str(&text).unwrap_or(Value::Null))
        .unwrap_or(Value::Null);
    Ok(hash.get("value").and_then(Value::as_str).unwrap_or("").to_string())
}

/// The stored revision of a project as the replace prompt shows it: its
/// envelope hash and its last-saved time. A Replace confirms exactly this.
/// The stored row's content fingerprint is part of the revision, so two writes
/// in one second differ unless they stored identical bytes, which loses nothing.
/// FNV-1a 64 (a fixed, stable function), not a security hash.
fn stored_revision(connection: &Connection, entry: &LocalProjectIndexEntry) -> Result<String, String> {
    let row: String = connection
        .query_row(
            "SELECT model_json || char(31) || editor_intents_json || char(31) || proposal_json || char(31) ||
                    selected_review_target_json || char(31) || mechanics_result_json || char(31) ||
                    analysis_run_json || char(31) || model_hash_json || char(31) ||
                    project_envelope_hash_json
             FROM local_projects WHERE project_id = ?1",
            params![entry.project_id],
            |row| row.get(0),
        )
        .map_err(|error| error.to_string())?;
    let fingerprint = row.bytes().fold(0xcbf2_9ce4_8422_2325_u64, |hash, byte| {
        (hash ^ u64::from(byte)).wrapping_mul(0x0000_0100_0000_01b3)
    });
    Ok(format!(
        "{}@{}@{fingerprint:016x}",
        stored_envelope_hash_value(connection, &entry.project_id)?,
        entry.updated_at_unix
    ))
}

/// HELP_HUMAN rulings (2026-10-10): the app never silently replaces a stored
/// project. A write onto a stored `project_id` is admitted when
/// - `confirmed_revision` equals the stored revision: the user chose Replace in
///   a prompt that showed this revision; or
/// - `expected_base` equals the stored envelope hash: the session is that stored
///   project, at the revision it last opened or saved (an ordinary save in place).
/// Otherwise it is refused: `PROJECT-BASE-MOVED` when the revision the user or
/// the session last saw is no longer the stored one (another window or process
/// saved that id since), `PROJECT-ID-EXISTS` for any other stored project.
/// Callers run it inside the write's `BEGIN IMMEDIATE` transaction.
fn refuse_unconfirmed_replacement(
    connection: &Connection,
    project_id: &str,
    confirmed_revision: Option<&str>,
    expected_base: Option<&str>,
) -> Result<(), String> {
    let Some(entry) = stored_project_entry(connection, project_id)? else {
        return Ok(());
    };
    let stored = format!(
        "project `{}` (\"{}\", last saved at unix {})",
        entry.project_id, entry.project_name, entry.updated_at_unix
    );
    if let Some(confirmed) = confirmed_revision {
        return if confirmed == stored_revision(connection, &entry)? {
            Ok(())
        } else {
            Err(format!(
                "PROJECT-BASE-MOVED: {stored} changed after the replace prompt showed it; replacing it needs the user's confirmation."
            ))
        };
    }
    match expected_base {
        // A row without an envelope hash has no revision to compare: it asks.
        Some(base) if !base.is_empty() && base == stored_envelope_hash_value(connection, project_id)? => Ok(()),
        Some(_) => Err(format!(
            "PROJECT-BASE-MOVED: {stored} changed after this session opened or saved it; replacing it needs the user's confirmation."
        )),
        None => Err(format!(
            "PROJECT-ID-EXISTS: {stored} is already stored; replacing it needs the user's confirmation."
        )),
    }
}

#[derive(Debug, Serialize)]
struct StoredProjectEntry {
    #[serde(flatten)]
    entry: LocalProjectIndexEntry,
    /// What a Replace of this project confirms (`stored_revision`).
    revision: String,
    /// The saved model states attached to this project id; a Replace keeps
    /// them, and the prompt says how many (W-12 §2).
    saved_state_count: i64,
}

/// The stored project with this id, if any, and its revision, for the replace prompt.
#[tauri::command]
fn find_local_project(app: AppHandle, project_id: String) -> Result<Option<StoredProjectEntry>, String> {
    let path = app_store_path(&app)?;
    let (connection, _migration) = open_project_store(&path)?;
    stored_project_entry(&connection, &project_id)?
        .map(|entry| {
            Ok(StoredProjectEntry {
                revision: stored_revision(&connection, &entry)?,
                saved_state_count: model_states::saved_state_count(&connection, &entry.project_id)?,
                entry,
            })
        })
        .transpose()
}

fn normalized_editor_intents(editor_intents: Option<Value>) -> Value {
    match editor_intents {
        Some(value) if value.is_array() => value,
        _ => json!([]),
    }
}

fn normalized_proposal(proposal: Option<Value>) -> Value {
    match proposal {
        Some(value) if value.is_object() => value,
        _ => Value::Null,
    }
}

fn normalized_selected_review_target(selected_review_target: Option<Value>) -> Value {
    match selected_review_target {
        Some(value) if value.is_object() => value,
        _ => Value::Null,
    }
}

fn proposal_count(proposal: &Value) -> usize {
    if proposal.is_object() {
        1
    } else {
        0
    }
}

fn editor_intent_count(editor_intents: &Value) -> usize {
    editor_intents.as_array().map_or(0, Vec::len)
}

fn selected_review_target_count(selected_review_target: &Value) -> usize {
    if selected_review_target.is_object() {
        1
    } else {
        0
    }
}

fn object_count(value: &Value) -> usize {
    if value.is_object() {
        1
    } else {
        0
    }
}

fn persisted_analysis_run_ref(analysis_run: &Value, mechanics_result: &Value) -> String {
    analysis_run
        .get("analysis_run")
        .and_then(|run| run.get("run_id"))
        .and_then(Value::as_str)
        .or_else(|| mechanics_result.get("run_id").and_then(Value::as_str))
        .map(str::to_string)
        .unwrap_or_else(|| "not_persisted".to_string())
}

fn persisted_model_hash_ref(model_hash: &Value) -> String {
    model_hash
        .get("value")
        .and_then(Value::as_str)
        .map(str::to_string)
        .unwrap_or_else(|| "not_persisted".to_string())
}

fn persisted_project_envelope_hash_ref(project_envelope_hash: &Value) -> String {
    project_envelope_hash
        .get("value")
        .and_then(Value::as_str)
        .map(str::to_string)
        .unwrap_or_else(|| "not_persisted".to_string())
}

fn selected_review_target_ref(selected_review_target: &Value) -> String {
    let Some(target) = selected_review_target.as_object() else {
        return "not_selected".to_string();
    };
    let target_type = target
        .get("target_type")
        .and_then(Value::as_str)
        .unwrap_or("unknown");
    let id = target
        .get("id")
        .and_then(Value::as_str)
        .unwrap_or("unknown");
    format!("{target_type}: {id}")
}

fn unit_round_trip_summary(model: &Value) -> (String, usize, String) {
    let mut unit_refs: Vec<String> = Vec::new();
    let mut missing_refs: Vec<String> = Vec::new();
    if let Some(units) = model
        .get("project")
        .and_then(|project| project.get("units"))
        .and_then(Value::as_object)
    {
        for (dimension, unit) in units {
            match unit.as_str().filter(|value| !value.is_empty()) {
                Some(unit) => unit_refs.push(format!("project.units.{dimension}={unit}")),
                None => missing_refs.push(format!("project.units.{dimension}")),
            }
        }
    }
    if let Some(materials) = model.get("materials").and_then(Value::as_array) {
        for material in materials {
            let material_id = value_id(material);
            collect_quantity_unit(
                &mut unit_refs,
                &mut missing_refs,
                format!("materials.{material_id}.elastic_modulus"),
                material.get("elastic_modulus"),
            );
            collect_quantity_unit(
                &mut unit_refs,
                &mut missing_refs,
                format!("materials.{material_id}.shear_modulus"),
                material.get("shear_modulus"),
            );
            collect_quantity_unit(
                &mut unit_refs,
                &mut missing_refs,
                format!("materials.{material_id}.poisson_ratio"),
                material.get("poisson_ratio"),
            );
            if let Some(points) = material.get("temperature_points").and_then(Value::as_array) {
                for point in points {
                    let point_id = value_id(point);
                    for field in ["temperature", "elastic_modulus", "shear_modulus", "poisson_ratio", "thermal_expansion_coefficient"] {
                        collect_quantity_unit(
                            &mut unit_refs,
                            &mut missing_refs,
                            format!("materials.{material_id}.temperature_points.{point_id}.{field}"),
                            point.get(field),
                        );
                    }
                }
            }
            collect_quantity_unit(
                &mut unit_refs,
                &mut missing_refs,
                format!("materials.{material_id}.thermal_expansion_coefficient"),
                material.get("thermal_expansion_coefficient"),
            );
        }
    }
    if let Some(sections) = model.get("sections").and_then(Value::as_array) {
        for section in sections {
            let section_id = value_id(section);
            if let Some(properties) = section.get("properties").and_then(Value::as_object) {
                for (field, quantity) in properties {
                    collect_quantity_unit(
                        &mut unit_refs,
                        &mut missing_refs,
                        format!("sections.{section_id}.{field}"),
                        Some(quantity),
                    );
                }
            }
        }
    }
    if let Some(segments) = model.get("pipe_segments").and_then(Value::as_array) {
        for segment in segments {
            let segment_id = value_id(segment);
            if let Some(section) = segment.get("section").and_then(Value::as_object) {
                for (field, quantity) in section {
                    collect_quantity_unit(
                        &mut unit_refs,
                        &mut missing_refs,
                        format!("pipe_segments.{segment_id}.section.{field}"),
                        Some(quantity),
                    );
                }
            }
        }
    }
    if let Some(load_cases) = model.get("load_cases").and_then(Value::as_array) {
        for load_case in load_cases {
            let load_case_id = value_id(load_case);
            if let Some(regions) = load_case.get("pressure_regions").and_then(Value::as_array) {
                for region in regions {
                    let region_id = value_id(region);
                    collect_quantity_unit(
                        &mut unit_refs,
                        &mut missing_refs,
                        format!("load_cases.{load_case_id}.pressure_regions.{region_id}.pressure"),
                        region.get("pressure"),
                    );
                }
            }
            if let Some(primitive_loads) =
                load_case.get("primitive_loads").and_then(Value::as_array)
            {
                for primitive_load in primitive_loads {
                    let primitive_id = value_id(primitive_load);
                    collect_quantity_unit(
                        &mut unit_refs,
                        &mut missing_refs,
                        format!(
                            "load_cases.{load_case_id}.primitive_loads.{primitive_id}.magnitude"
                        ),
                        primitive_load.get("magnitude"),
                    );
                }
            }
        }
    }
    unit_refs.sort();
    let signature = if unit_refs.is_empty() {
        "no_unit_metadata".to_string()
    } else {
        unit_refs.join("|")
    };
    let status = if !unit_refs.is_empty() && missing_refs.is_empty() {
        "unit_metadata_preserved_in_local_project_envelope"
    } else {
        "unit_metadata_missing_review_required"
    };
    (status.to_string(), unit_refs.len(), signature)
}

fn collect_quantity_unit(
    unit_refs: &mut Vec<String>,
    missing_refs: &mut Vec<String>,
    reference: String,
    value: Option<&Value>,
) {
    let Some(value) = value else {
        return;
    };
    if let Some(unit) = value
        .get("unit")
        .and_then(Value::as_str)
        .filter(|unit| !unit.is_empty())
    {
        unit_refs.push(format!("{reference}={unit}"));
    } else if value.get("value").is_some() {
        missing_refs.push(reference);
    }
}

fn value_id(value: &Value) -> String {
    value
        .get("id")
        .and_then(Value::as_str)
        .unwrap_or("unidentified")
        .to_string()
}

fn project_summary(
    project_id: String,
    project_name: String,
    model: &Value,
    database_path: PathBuf,
    migration: &StoreMigrationEvidence,
    editor_intents: &Value,
    proposal: &Value,
    selected_review_target: &Value,
    mechanics_result: &Value,
    analysis_run: &Value,
    model_hash: &Value,
    project_envelope_hash: &Value,
    message: String,
) -> LocalProjectSummary {
    let (unit_round_trip_status, unit_round_trip_checked_ref_count, unit_round_trip_signature) =
        unit_round_trip_summary(model);
    LocalProjectSummary {
        project_id,
        project_name,
        database_path: database_path.display().to_string(),
        storage_mode: "local_sqlite",
        migration_status: migration.migration_status.clone(),
        migration_framework: migration.migration_framework,
        store_schema_version: migration.store_schema_version,
        store_schema_target_version: migration.store_schema_target_version,
        migrations_applied_on_open: migration.migrations_applied_on_open.clone(),
        fts_indexed: true,
        copied_external_files: false,
        editor_intent_count: editor_intent_count(editor_intents),
        proposal_count: proposal_count(proposal),
        selected_review_target_count: selected_review_target_count(selected_review_target),
        selected_review_target_ref: selected_review_target_ref(selected_review_target),
        persisted_mechanics_result_count: object_count(mechanics_result),
        persisted_analysis_run_count: object_count(analysis_run),
        persisted_analysis_run_ref: persisted_analysis_run_ref(analysis_run, mechanics_result),
        persisted_model_hash_count: object_count(model_hash),
        persisted_model_hash_ref: persisted_model_hash_ref(model_hash),
        persisted_project_envelope_hash_count: object_count(project_envelope_hash),
        persisted_project_envelope_hash_ref: persisted_project_envelope_hash_ref(
            project_envelope_hash,
        ),
        unit_round_trip_status,
        unit_round_trip_checked_ref_count,
        unit_round_trip_signature,
        message,
    }
}

#[tauri::command]
fn get_unit_catalog() -> UnitCatalogResponse {
    desktop_unit_catalog()
}

/// Model-sized commands take their arguments as a raw body admitted under an
/// interactive ingress lease (`command_ingress`): header
/// `x-swbpipe-ingress: args-json-v1`, body = the UTF-8 JSON object of the
/// camelCase arguments. Each command's arguments are decoded by its `*_args`
/// function; the response is serialized while the lease is held.
#[tauri::command]
fn render_calculation_report(
    invoke: tauri::ipc::Request<'_>,
    registry: tauri::State<'_, SolveJobRegistry>,
) -> Result<tauri::ipc::Response, CommandError> {
    let (mut args, _lease) = command_ingress::admit(&registry, &invoke, "render_calculation_report", ArgsDecoding::Moved)?;
    respond(render_calculation_report_args(&mut args))
}

fn render_calculation_report_args(args: &mut Args) -> Result<Value, CommandError> {
    let input = args.value("input")?;
    render_calculation_report_value(input).map_err(CommandError::Command)
}

fn render_calculation_report_value(input: Value) -> Result<Value, String> {
    // DEC-021 (A7): deterministic hash-bound HTML rendering with the
    // three-point protected-content gates evaluated in the renderer crate.
    // The derived print view is non-hash-bound and names the canonical hash.
    let renderable: open_pipe_stress_report_renderer::RenderableReportInput =
        serde_json::from_value(input).map_err(|error| format!("RENDER-INPUT-INVALID: {error}"))?;
    let outcome = open_pipe_stress_report_renderer::render_calculation_report(&renderable);
    let derived_print_html =
        open_pipe_stress_report_renderer::derived_print_view(&outcome.html, &outcome.sha256_hex);
    let mut payload = serde_json::to_value(&outcome).map_err(|error| error.to_string())?;
    payload["derived_print_html"] = Value::String(derived_print_html);
    Ok(payload)
}

/// The default session model: the valid demo (no retired legacy pressure, no
/// refused joint), which the product solves.
#[tauri::command]
fn load_preview_model() -> Result<Value, String> {
    bundled_document(BUNDLED_DEMO_MODEL)
}

#[tauri::command]
fn load_design_knowledge() -> Result<Value, String> {
    bundled_document(BUNDLED_DESIGN_KNOWLEDGE)
}

/// File > Open Model Document…: the user chooses a model document file; it is
/// read, parsed and checked (`model_document_file`), and returned unchanged or
/// refused with named diagnostics. The webview checks the desktop document shape
/// before the session adopts it. Async so the blocking chooser stays off the main
/// thread (as `save_report_package`).
#[tauri::command]
async fn open_model_document_file(app: AppHandle) -> Result<model_document_file::ModelDocumentOpenOutcome, String> {
    let Some(path) = choose_model_document_path(&app)? else {
        return Ok(model_document_file::ModelDocumentOpenOutcome::Cancelled);
    };
    tauri::async_runtime::spawn_blocking(move || model_document_file::read_model_document_file(&path))
        .await
        .map_err(|error| error.to_string())
}

/// File > Save Model Document… (DEL-15-01 W-6a): the session's model document
/// as its RFC 8785 canonical text, admitted by `model_document_save` (route
/// evidence, the open cap, the canonical-text and hash check, the open path's
/// document checks) before the chooser is shown, then written by the atomic
/// writer as `.json`. Cancel is an outcome, not an error. Async so the blocking
/// chooser stays off the main thread (as `save_report_package`); the ingress
/// lease is held through admission, the chooser and the write.
#[tauri::command]
async fn save_model_document_file(
    app: AppHandle,
    invoke: tauri::ipc::Request<'_>,
    registry: tauri::State<'_, SolveJobRegistry>,
) -> Result<tauri::ipc::Response, CommandError<NoCommandError>> {
    let (mut args, _lease) = command_ingress::admit(&registry, &invoke, "save_model_document_file", ArgsDecoding::Moved)?;
    let request = model_document_save::save_model_document_args(&mut args)?;
    drop(args);
    respond(Ok(save_model_document_decoded(app, request).await))
}

async fn save_model_document_decoded(
    app: AppHandle,
    request: model_document_save::ModelDocumentSaveRequest,
) -> model_document_save::ModelDocumentSaveOutcome {
    use model_document_save::{ModelDocumentSaveOutcome as Outcome, PICKER_PATH_INVALID, WORKER_FAILED};
    if let Err(diagnostic) =
        model_document_save::validate_model_document_save_evidence(&request.screening, &request.local_first)
    {
        return Outcome::refused(vec![diagnostic]);
    }
    let model_document_save::ModelDocumentSaveRequest { canonical_text, claimed_model_hash, screening, .. } = request;
    let store_app = app.clone();
    let admitted = match tauri::async_runtime::spawn_blocking(move || {
        // The local library store, opened only if the document references a
        // library, as `open_local_library` opens it. A read error refuses.
        let mut connection: Option<Connection> = None;
        let mut lookup = |project_id: &str, library_kind: &str, library_id: &str| -> Result<Option<Value>, String> {
            if connection.is_none() {
                connection = Some(open_project_store(&app_store_path(&store_app)?)?.0);
            }
            let connection = connection.as_ref().expect("opened above");
            // The identities as `open_local_library` normalizes them.
            load_library(connection, project_id.trim(), library_kind.trim(), library_id.trim()).map(|row| row.map(|row| row.document))
        };
        model_document_save::admit_model_document_text(canonical_text, &claimed_model_hash, &screening.payload_sha256, &mut lookup)
    })
    .await
    {
        Ok(Ok(admitted)) => admitted,
        Ok(Err(diagnostics)) => return Outcome::refused(diagnostics),
        Err(_) => return Outcome::refused_one(WORKER_FAILED, "The save's admission worker did not complete."),
    };
    let directory = model_document_save::default_save_directory(app.path().document_dir().ok(), app.path().home_dir().ok());
    let destination = match choose_model_document_save_path(&app, &admitted.suggested_file_name, directory) {
        Ok(Some(destination)) => destination,
        Ok(None) => return Outcome::Cancelled,
        Err(message) => return Outcome::refused_one(PICKER_PATH_INVALID, message),
    };
    tauri::async_runtime::spawn_blocking(move || model_document_save::write_admitted_model_document(&destination, admitted))
        .await
        .unwrap_or_else(|_| Outcome::refused_one(WORKER_FAILED, "The save's write worker did not complete; the destination may be unchanged."))
}

fn choose_model_document_save_path(
    app: &AppHandle,
    suggested_file_name: &str,
    directory: Option<PathBuf>,
) -> Result<Option<PathBuf>, String> {
    #[cfg(feature = "native-smoke")]
    if let Some(path) = native_smoke::take_chosen_path(app) {
        return Ok(Some(path));
    }
    let mut chooser = app
        .dialog()
        .file()
        .set_file_name(suggested_file_name)
        .add_filter("SWBPIPE model document", &["json"]);
    if let Some(directory) = directory {
        chooser = chooser.set_directory(directory);
    }
    let Some(selected) = chooser.blocking_save_file() else {
        return Ok(None);
    };
    selected.into_path().map(Some).map_err(|error| error.to_string())
}

fn choose_model_document_path(app: &AppHandle) -> Result<Option<PathBuf>, String> {
    #[cfg(feature = "native-smoke")]
    if let Some(path) = native_smoke::take_chosen_path(app) {
        return Ok(Some(path));
    }
    let Some(selected) = app
        .dialog()
        .file()
        .add_filter("SWBPIPE model document", &["json"])
        .blocking_pick_file()
    else {
        return Ok(None);
    };
    selected.into_path().map(Some).map_err(|error| error.to_string())
}

fn solve_preview_mechanics(model_payload: Value) -> Result<Value, String> {
    solve_preview_mechanics_value_with_mode(model_payload, PreviewSolverMode::default())
}

/// The producer's exact serialized bytes for the desktop's solve request
/// (`{"model": <model>, "materials": []}`): what the solve commands and jobs hand
/// the desktop, which reads its result and registers it from these bytes (b7-plan
/// R1, "Exact bytes to the desktop"), never a re-encoded value.
fn solve_preview_mechanics_with_mode(
    model_payload: Value,
    solver_mode: PreviewSolverMode,
) -> Result<String, String> {
    // The model moves into the request (no deep copy); `ingress` prices this wrapper.
    let request = ingress::native_request(model_payload);
    serde_json::to_string(&open_pipe_stress_product_physics::run_linear_static_preview_value_with_mode(request, solver_mode)?)
        .map_err(|error| error.to_string())
}

/// The same solve read as a value, for the native self-tests that inspect it.
fn solve_preview_mechanics_value_with_mode(
    model_payload: Value,
    solver_mode: PreviewSolverMode,
) -> Result<Value, String> {
    serde_json::from_str(&solve_preview_mechanics_with_mode(model_payload, solver_mode)?)
        .map_err(|error| error.to_string())
}

/// A solve always carries the session's model document; there is no implicit model.
#[cfg(test)]
fn resolve_solve_model_payload(model: Option<Value>) -> Result<Value, String> {
    model.ok_or_else(|| "PREVIEW-SOLVE-MODEL-REQUIRED: a solve carries the session's model document.".to_string())
}

use solve_jobs::{
    SolveJobCancellationReceipt, SolveJobError, SolveJobRegistry, SolveJobStartReceipt, SolveJobStatusReport,
    SolveResultDisposal, SolveResultsDisposal,
};

/// Starts a preview solve job (the desktop's one solve path). The body is the
/// raw UTF-8 JSON of the model document (`Uint8Array`/`ArrayBuffer`, sent as
/// `application/octet-stream`); header `x-swbpipe-solver-mode` selects
/// `sparse_interactive` or `dense_scrutiny` (absent: the default). The body is
/// prescanned and its ingress lease admitted before it is copied; the model is
/// materialized and solved only in the job's capped worker process. A JSON
/// body is refused: Tauri would already have materialized it unpriced.
#[tauri::command]
async fn start_preview_mechanics_job(
    request: tauri::ipc::Request<'_>,
    state: tauri::State<'_, SolveJobRegistry>,
) -> Result<SolveJobStartReceipt, SolveJobError> {
    let tauri::ipc::InvokeBody::Raw(body) = request.body() else {
        return Err(SolveJobError::InvalidRequest {
            message: "PREVIEW-SOLVE-RAW-BODY-REQUIRED: send the model document as raw UTF-8 JSON bytes.".into(),
        });
    };
    let mode = match request.headers().get(solve_jobs::SOLVER_MODE_HEADER) {
        None => None,
        Some(value) => Some(value.to_str().map_err(|_| SolveJobError::InvalidRequest {
            message: "unsupported preview solver mode header".into(),
        })?),
    };
    state.start(body, mode)
}

#[tauri::command]
fn poll_preview_mechanics_job(
    state: tauri::State<'_, SolveJobRegistry>,
    job_id: String,
) -> Result<SolveJobStatusReport, SolveJobError> {
    state.status(&job_id)
}

/// The producer's exact result bytes as a raw response (an `ArrayBuffer` in
/// the webview). The job's result lease grows to 2R first; the webview must
/// call `dispose_preview_mechanics_result` once it has dropped its copy.
#[tauri::command]
fn take_preview_mechanics_job_result(
    state: tauri::State<'_, SolveJobRegistry>,
    job_id: String,
) -> Result<tauri::ipc::Response, SolveJobError> {
    state.take_result(&job_id).map(tauri::ipc::Response::new)
}

#[tauri::command]
fn dispose_preview_mechanics_result(
    state: tauri::State<'_, SolveJobRegistry>,
    job_id: String,
) -> Result<SolveResultDisposal, SolveJobError> {
    state.dispose_result(&job_id)
}

/// Session reset: releases every retained-but-untaken result and every webview
/// custody lease, and abandons every queued or running job (a running job's
/// worker process is killed; it never publishes a result, and its supervisor
/// releases its owners after reaping the worker). The frontend calls it on page load, when a fresh webview provably
/// holds no result copy and no handle on the previous page's jobs. Returns
/// `{released_results, released_bytes, abandoned_jobs}`.
#[tauri::command]
fn dispose_all_preview_mechanics_results(state: tauri::State<'_, SolveJobRegistry>) -> SolveResultsDisposal {
    state.dispose_all_results()
}

#[tauri::command]
fn cancel_preview_mechanics_job(
    state: tauri::State<'_, SolveJobRegistry>,
    job_id: String,
    cancellation_token: String,
) -> Result<SolveJobCancellationReceipt, String> {
    state.cancel(&job_id, &cancellation_token)
}

#[tauri::command]
fn get_memory_pool_status(state: tauri::State<'_, SolveJobRegistry>) -> memory_pool::PoolStatus {
    state.pool_status()
}

#[tauri::command]
fn configure_memory_pool(
    state: tauri::State<'_, SolveJobRegistry>,
    pool_bytes: u64,
    invocation_budget_bytes: u64,
    acknowledge_above_recommended: bool,
) -> Result<memory_pool::PoolStatus, SolveJobError> {
    state.configure_pool(pool_bytes, invocation_budget_bytes, acknowledge_above_recommended)
}

fn build_sample_agent_proposal(
    result: Value,
    selected_target: Option<SelectedReviewTarget>,
) -> Value {
    let diagnostics = result
        .get("diagnostics")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    let results = result
        .get("results")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    let primary_ref = results
        .iter()
        .find(|item| {
            item.get("id").and_then(Value::as_str) == Some("result:force:pipe-P-120:axial")
        })
        .and_then(|item| item.get("id").and_then(Value::as_str))
        .or_else(|| {
            results
                .iter()
                .find(|item| {
                    item.get("kind").and_then(Value::as_str) == Some("element_local_axial_force")
                })
                .and_then(|item| item.get("id").and_then(Value::as_str))
        })
        .or_else(|| {
            diagnostics
                .iter()
                .find(|item| item.get("severity").and_then(Value::as_str) == Some("warning"))
                .and_then(|item| item.get("id").and_then(Value::as_str))
        })
        .unwrap_or("diagnostic:physics:rule-inputs-missing")
        .to_string();
    let (target_ref, target_kind) = selected_target
        .map(|target| (target.id, target.target_type.replace('_', " ")))
        .unwrap_or_else(|| (primary_ref, "computed mechanics".to_string()));
    let proposal = json!({
        "schema_version": "0.1.0",
        "document_kind": "openpipestress.product_preview.agent_proposal",
        "proposal_id": "proposal:physics-diagnostic-review",
        "model_ref": result.get("model_ref").cloned().unwrap_or_else(|| json!("project:invented-demo-loop-01")),
        "prompt": "Review current computed mechanics diagnostics and suggest a non-mutating follow-up.",
        "operation": {
            "operation_id": "op:review-computed-diagnostic",
            "operation_kind": "attach_design_knowledge",
            "operation_status": "draft_user_review_required",
            "affected_entity_ids": [target_ref],
            "changes": [
                {
                    "change_id": "change:add-review-note",
                    "change_kind": "attach_design_knowledge",
                    "target_ref": target_ref,
                    "before": format!("No review note attached for the selected {target_kind} context."),
                    "after": format!("Attach review note referencing the current computed preview {target_kind} context.")
                }
            ]
        },
        "rationale": format!("Generated from current preview mechanics context; selected review reference is {target_ref}. This narrative is review-only and does not mutate accepted model state."),
        "assumptions": [
            "The model is invented and not a project basis.",
            "No protected criteria, allowables, or owner-standard values are available."
        ],
        "validation": {
            "schema_validation": "passed",
            "constraint_validation": "warning_computed_context_requires_human_review",
            "diff_preview_status": "generated_from_computed_context",
            "application_status": "not_applied"
        },
        "audit_boundary": {
            "requires_user_acceptance": true,
            "mutates_accepted_model_state": false,
            "acceptance_recorded_as_review_only": true
        },
        "professional_boundary": {
            "human_review_required": true,
            "software_makes_compliance_claim": false,
            "software_makes_certification_claim": false,
            "software_makes_sealing_claim": false,
            "software_makes_approval_claim": false
        }
    });
    json!({
        "proposal": proposal,
        "application_status": "not_applied",
        "accepted_model_mutated": false
    })
}

#[tauri::command]
fn sample_agent_proposal(
    invoke: tauri::ipc::Request<'_>,
    registry: tauri::State<'_, SolveJobRegistry>,
) -> Result<tauri::ipc::Response, CommandError> {
    let (mut args, _lease) = command_ingress::admit(&registry, &invoke, "sample_agent_proposal", ArgsDecoding::Moved)?;
    respond(sample_agent_proposal_args(&mut args))
}

fn sample_agent_proposal_args(args: &mut Args) -> Result<Value, CommandError> {
    let mechanics_result = args.opt_value("mechanicsResult");
    let selected_target = args.decode::<Option<SelectedReviewTarget>>("selectedTarget")?;
    sample_agent_proposal_value(mechanics_result, selected_target).map_err(CommandError::Command)
}

fn sample_agent_proposal_value(
    mechanics_result: Option<Value>,
    selected_target: Option<SelectedReviewTarget>,
) -> Result<Value, String> {
    // The sample proposal reviews a result the session already has; it never
    // solves an implicit model.
    let result = mechanics_result.ok_or_else(|| {
        "PREVIEW-SOLVE-MODEL-REQUIRED: the sample proposal needs the session's mechanics result.".to_string()
    })?;
    Ok(build_sample_agent_proposal(result, selected_target))
}

#[tauri::command]
fn validate_model_operation(
    invoke: tauri::ipc::Request<'_>,
    registry: tauri::State<'_, SolveJobRegistry>,
) -> Result<tauri::ipc::Response, CommandError> {
    let (mut args, _lease) = command_ingress::admit(&registry, &invoke, "validate_model_operation", ArgsDecoding::Moved)?;
    respond(validate_model_operation_args(&mut args))
}

/// `{model, intent, claimedModelHash?}`.
fn model_operation_args(args: &mut Args) -> Result<(Value, Value, Option<Value>), CommandError> {
    Ok((args.value("model")?, args.value("intent")?, args.opt_value("claimedModelHash")))
}

fn validate_model_operation_args(args: &mut Args) -> Result<Value, CommandError> {
    let (model, intent, claimed_model_hash) = model_operation_args(args)?;
    validate_model_operation_value(model, intent, claimed_model_hash).map_err(CommandError::Command)
}

fn validate_model_operation_value(
    model: Value,
    intent: Value,
    claimed_model_hash: Option<Value>,
) -> Result<Value, String> {
    let outcome = validate_operation(&model, &intent, claimed_model_hash.as_ref());
    serde_json::to_value(outcome).map_err(|error| error.to_string())
}

#[tauri::command]
fn apply_model_operation(
    invoke: tauri::ipc::Request<'_>,
    registry: tauri::State<'_, SolveJobRegistry>,
) -> Result<tauri::ipc::Response, CommandError> {
    let (mut args, _lease) = command_ingress::admit(&registry, &invoke, "apply_model_operation", ArgsDecoding::Moved)?;
    respond(apply_model_operation_args(&mut args))
}

fn apply_model_operation_args(args: &mut Args) -> Result<Value, CommandError> {
    let (model, intent, claimed_model_hash) = model_operation_args(args)?;
    apply_model_operation_value(model, intent, claimed_model_hash).map_err(CommandError::Command)
}

fn apply_model_operation_value(
    model: Value,
    intent: Value,
    claimed_model_hash: Option<Value>,
) -> Result<Value, String> {
    let outcome = apply_operation(&model, &intent, claimed_model_hash.as_ref());
    serde_json::to_value(outcome).map_err(|error| error.to_string())
}

#[tauri::command]
fn generate_self_weight_operation_plan(
    invoke: tauri::ipc::Request<'_>,
    registry: tauri::State<'_, SolveJobRegistry>,
) -> Result<tauri::ipc::Response, CommandError<NoCommandError>> {
    let (mut args, _lease) =
        command_ingress::admit(&registry, &invoke, "generate_self_weight_operation_plan", ArgsDecoding::Moved)?;
    respond(generate_self_weight_operation_plan_args(&mut args))
}

fn generate_self_weight_operation_plan_args(args: &mut Args) -> Result<Value, CommandError<NoCommandError>> {
    let (model, request) = (args.value("model")?, args.value("request")?);
    Ok(open_pipe_stress_self_weight_wasm::generate_plan(&model, &request))
}

#[tauri::command]
fn validate_model_operation_batch(
    invoke: tauri::ipc::Request<'_>,
    registry: tauri::State<'_, SolveJobRegistry>,
) -> Result<tauri::ipc::Response, CommandError<NoCommandError>> {
    let (mut args, _lease) =
        command_ingress::admit(&registry, &invoke, "validate_model_operation_batch", ArgsDecoding::Moved)?;
    respond(model_operation_batch_args(&mut args, false))
}

#[tauri::command]
fn apply_model_operation_batch(
    invoke: tauri::ipc::Request<'_>,
    registry: tauri::State<'_, SolveJobRegistry>,
) -> Result<tauri::ipc::Response, CommandError<NoCommandError>> {
    let (mut args, _lease) = command_ingress::admit(&registry, &invoke, "apply_model_operation_batch", ArgsDecoding::Moved)?;
    respond(model_operation_batch_args(&mut args, true))
}

/// `{model, batch, claimedModelHash?}`, validated or applied.
fn model_operation_batch_args(args: &mut Args, apply: bool) -> Result<Value, CommandError<NoCommandError>> {
    let (model, batch, claimed_model_hash) = (args.value("model")?, args.value("batch")?, args.opt_value("claimedModelHash"));
    Ok(if apply {
        open_pipe_stress_operation_applier::apply_operation_batch(&model, &batch, claimed_model_hash.as_ref())
    } else {
        open_pipe_stress_operation_applier::validate_operation_batch(&model, &batch, claimed_model_hash.as_ref())
    })
}

#[tauri::command]
fn convert_display_quantities(
    invoke: tauri::ipc::Request<'_>,
    registry: tauri::State<'_, SolveJobRegistry>,
) -> Result<tauri::ipc::Response, CommandError<NoCommandError>> {
    let (mut args, _lease) = command_ingress::admit(&registry, &invoke, "convert_display_quantities", ArgsDecoding::Moved)?;
    respond(convert_display_quantities_args(&mut args))
}

fn convert_display_quantities_args(args: &mut Args) -> Result<Value, CommandError<NoCommandError>> {
    let input = args.value("input")?;
    Ok(open_pipe_stress_operation_applier::convert_display_quantities(&input))
}

#[tauri::command]
fn get_local_storage_capability(app: AppHandle) -> Result<StorageCapability, String> {
    let path = app_store_path(&app)?;
    let (connection, migration) = open_project_store(&path)?;
    Ok(StorageCapability {
        engine: "SQLite",
        bundled: true,
        fts5_available: fts5_available(&connection),
        network_required: false,
        daemon_required: false,
        telemetry_enabled: false,
        path_policy: "app-local user data path; no repository-default writes",
        large_file_policy:
            "reference external files by path/hash metadata; do not silently copy large files",
        database_path: path.display().to_string(),
        compile_options: sqlite_compile_options(&connection)?,
        migration_framework: migration.migration_framework,
        migration_status: migration.migration_status.clone(),
        store_schema_version: migration.store_schema_version,
        store_schema_target_version: migration.store_schema_target_version,
        migrations_applied_on_open: migration.migrations_applied_on_open.clone(),
    })
}

fn refuse_model_document(status: &ModelDocumentVersionStatus) -> String {
    format!(
        "Model document refused for editing: status={}; source_schema_version={}; target_schema_version={}; {}",
        status.status, status.source_schema_version, status.target_schema_version, status.detail
    )
}

/// W-13 §5.3: the one document validator's blocking findings, named with
/// their JSON paths, refuse the document. Warnings never refuse.
fn refuse_malformed_model_slots(model: &Value) -> Result<(), String> {
    let blocking: Vec<String> = open_pipe_stress_operation_applier::validate_model_document(model)
        .into_iter()
        .filter(open_pipe_stress_operation_applier::DocumentDiagnostic::is_blocking)
        .map(|finding| format!("{} at {}: {}", finding.code, finding.path, finding.message))
        .collect();
    if blocking.is_empty() {
        return Ok(());
    }
    Err(format!("Model document refused: malformed model-data slots: {}", blocking.join(" | ")))
}

/// The one-current-version check alone (DEL-02-05 CLM-003): a document that is
/// not current is refused by name. Project persistence adds the blocking-slot
/// refusal (`check_model_document_for_persist`); a saved model state uses this
/// alone, since a state may hold an incomplete model (W-12 §2, `MODEL_INCOMPLETE`).
fn check_model_document_version(model: &Value) -> Result<ModelDocumentVersionStatus, String> {
    let status = evaluate_model_document(model);
    if status.is_refused() {
        return Err(refuse_model_document(&status));
    }
    Ok(status)
}

/// The version check for create/save (DEL-02-05 CLM-003): a document that is
/// not current is refused by name. A current document is persisted exactly as
/// received, with the received hash evidence; nothing is migrated, normalized
/// or re-stamped.
fn check_model_document_for_persist(model: &Value) -> Result<ModelDocumentVersionStatus, String> {
    let status = check_model_document_version(model)?;
    // W-13 §5.3 (S2): create and save refuse a document whose model-data
    // slots carry a blocking validator finding, named with its JSON path;
    // nothing is repaired. Warnings never refuse.
    refuse_malformed_model_slots(model)?;
    Ok(status)
}

#[cfg(test)]
fn hash_value_string(hash: &Value) -> String {
    hash.get("value")
        .and_then(Value::as_str)
        .unwrap_or("not_reported")
        .to_string()
}

#[cfg(test)]
fn model_payload_hash(model: &Value) -> String {
    format!(
        "sha256:{}",
        open_pipe_stress_operation_applier::sha256_hex(
            &open_pipe_stress_operation_applier::canonical_json(model)
        )
    )
}

#[cfg(test)]
fn computed_model_hash(model: &Value, project_id: &str) -> Value {
    json!({
        "algorithm": "sha256",
        "canonicalization": "rfc8785_jcs",
        "payload_scope": "model_payload",
        "payload_ref": project_id,
        "value": model_payload_hash(model),
        "hash_status": "computed_local_preview"
    })
}

#[cfg(any(test, feature = "native-smoke"))]
fn computed_project_envelope_hash(
    project_id: &str,
    model: &Value,
    editor_intents: &Value,
    proposal: &Value,
    selected_review_target: &Value,
    mechanics_result: &Value,
    analysis_run: &Value,
    model_hash: &Value,
) -> Value {
    let payload = json!({
        "model": model,
        "editor_intents": editor_intents,
        "proposal": proposal,
        "selected_review_target": selected_review_target,
        "mechanics_result": mechanics_result,
        "analysis_run": analysis_run,
        "model_hash": model_hash
    });
    let value = format!(
        "sha256:{}",
        open_pipe_stress_operation_applier::sha256_hex(
            &open_pipe_stress_operation_applier::canonical_json(&payload)
        )
    );
    json!({
        "algorithm": "sha256",
        "canonicalization": "rfc8785_jcs",
        "payload_scope": "project_envelope_payload",
        "payload_excludes": "storage_summary_and_envelope_hash_carrier_fields",
        "payload_ref": project_id,
        "value": value,
        "hash_status": "computed_local_preview"
    })
}

#[tauri::command]
fn create_local_project(
    app: AppHandle,
    invoke: tauri::ipc::Request<'_>,
    registry: tauri::State<'_, SolveJobRegistry>,
) -> Result<tauri::ipc::Response, CommandError> {
    let (mut args, _lease) = command_ingress::admit(&registry, &invoke, "create_local_project", ArgsDecoding::Moved)?;
    respond(create_local_project_args(&mut args, || app_store_path(&app)))
}

/// `{model, editorIntents?, proposal?, selectedReviewTarget?, mechanicsResult?,
/// analysisRun?, modelHash?, projectEnvelopeHash?, confirmedRevision?}`.
fn create_local_project_args(
    args: &mut Args,
    store: impl FnOnce() -> Result<PathBuf, String>,
) -> Result<LocalProjectEnvelope, CommandError> {
    let model = args.value("model")?;
    let editor_intents = args.opt_value("editorIntents");
    let proposal = args.opt_value("proposal");
    let selected_review_target = args.opt_value("selectedReviewTarget");
    let mechanics_result = args.opt_value("mechanicsResult");
    let analysis_run = args.opt_value("analysisRun");
    let model_hash = args.opt_value("modelHash");
    let project_envelope_hash = args.opt_value("projectEnvelopeHash");
    let confirmed_revision = args.decode::<Option<String>>("confirmedRevision")?;
    store()
        .and_then(|path| {
            create_project_in_store(path, model, editor_intents, proposal, selected_review_target, mechanics_result, analysis_run, model_hash, project_envelope_hash, confirmed_revision)
        })
        .map_err(CommandError::Command)
}

/// `create_local_project` on a given store: one `BEGIN IMMEDIATE` transaction
/// from the preparation's reads through the replacement check, the write and
/// the commit (HELP_HUMAN rulings 2026-10-10). The race tests call this path.
fn create_project_in_store(
    path: PathBuf,
    model: Value,
    editor_intents: Option<Value>,
    proposal: Option<Value>,
    selected_review_target: Option<Value>,
    mechanics_result: Option<Value>,
    analysis_run: Option<Value>,
    model_hash: Option<Value>,
    project_envelope_hash: Option<Value>,
    confirmed_revision: Option<String>,
) -> Result<LocalProjectEnvelope, String> {
    let (mut connection, migration) = open_project_store(&path)?;
    // One write transaction from the preparation's reads to the commit.
    let transaction = immediate_transaction(&mut connection)?;
    let project_id = project_id_from_model(&model);
    let project_name = project_name_from_model(&model);
    let editor_intents = normalized_editor_intents(editor_intents);
    let proposal = normalized_proposal(proposal);
    let selected_review_target = normalized_selected_review_target(selected_review_target);
    let mechanics_result = mechanics_result.unwrap_or(Value::Null);
    let analysis_run = analysis_run.unwrap_or(Value::Null);
    let model_hash = model_hash.unwrap_or(Value::Null);
    let project_envelope_hash = project_envelope_hash.unwrap_or(Value::Null);
    let document_version = check_model_document_for_persist(&model)?;
    guarded_write_project_rows(
        &transaction,
        confirmed_revision.as_deref(),
        None,
        &project_id,
        &project_name,
        &model,
        &editor_intents,
        &proposal,
        &selected_review_target,
        &mechanics_result,
        &analysis_run,
        &model_hash,
        &project_envelope_hash,
    )?;
    transaction.commit().map_err(|error| error.to_string())?;
    Ok(LocalProjectEnvelope {
        summary: project_summary(
            project_id,
            project_name,
            &model,
            path,
            &migration,
            &editor_intents,
            &proposal,
            &selected_review_target,
            &mechanics_result,
            &analysis_run,
            &model_hash,
            &project_envelope_hash,
            "Created local SQLite project snapshot without external file copies.".to_string(),
        ),
        model,
        editor_intents,
        proposal,
        selected_review_target,
        mechanics_result,
        analysis_run,
        model_hash,
        project_envelope_hash,
        model_document_version: serde_json::to_value(document_version)
            .map_err(|error| error.to_string())?,
    })
}

#[tauri::command]
fn open_local_project(
    app: AppHandle,
    project_id: Option<String>,
) -> Result<Option<LocalProjectEnvelope>, String> {
    open_project_in_store(app_store_path(&app)?, project_id.as_deref())
}

/// `open_local_project` on a given store.
fn open_project_in_store(
    path: PathBuf,
    project_id: Option<&str>,
) -> Result<Option<LocalProjectEnvelope>, String> {
    let (connection, migration) = open_project_store(&path)?;
    load_project(&connection, project_id)?
        .map(|record| {
            // DEL-02-05 CLM-003: the restored document is opened exactly as
            // stored when its version is current; any other version is refused
            // by name (re-author), never migrated.
            let status = evaluate_model_document(&record.model);
            if status.is_refused() {
                return Err(refuse_model_document(&status));
            }
            // W-13 §5.3: a stored document with a malformed model-data slot is
            // refused by name, never repaired.
            refuse_malformed_model_slots(&record.model)?;
            let model = record.model;
            Ok(LocalProjectEnvelope {
                summary: project_summary(
                    record.project_id,
                    record.project_name,
                    &model,
                    path.clone(),
                    &migration,
                    &record.editor_intents,
                    &record.proposal,
                    &record.selected_review_target,
                    &record.mechanics_result,
                    &record.analysis_run,
                    &record.model_hash,
                    &record.project_envelope_hash,
                    "Opened local SQLite project snapshot.".to_string(),
                ),
                model,
                editor_intents: record.editor_intents,
                proposal: record.proposal,
                selected_review_target: record.selected_review_target,
                mechanics_result: record.mechanics_result,
                analysis_run: record.analysis_run,
                model_hash: record.model_hash,
                project_envelope_hash: record.project_envelope_hash,
                model_document_version: serde_json::to_value(status)
                    .map_err(|error| error.to_string())?,
            })
        })
        .transpose()
}

#[tauri::command]
fn list_local_projects(app: AppHandle) -> Result<Vec<LocalProjectIndexEntry>, String> {
    let path = app_store_path(&app)?;
    let (connection, _migration) = open_project_store(&path)?;
    list_projects(&connection)
}

#[tauri::command]
fn save_local_project(
    app: AppHandle,
    invoke: tauri::ipc::Request<'_>,
    registry: tauri::State<'_, SolveJobRegistry>,
) -> Result<tauri::ipc::Response, CommandError> {
    let (mut args, _lease) = command_ingress::admit(&registry, &invoke, "save_local_project", ArgsDecoding::Moved)?;
    respond(save_local_project_args(&mut args, || app_store_path(&app)))
}

/// `{request}` (snake_case fields, see `SaveLocalProjectRequest`).
fn save_local_project_args(
    args: &mut Args,
    store: impl FnOnce() -> Result<PathBuf, String>,
) -> Result<LocalProjectEnvelope, CommandError> {
    let request = args.decode_with("request", SaveLocalProjectRequest::from_value)?;
    store().and_then(|path| save_project_in_store(path, request)).map_err(CommandError::Command)
}

/// `save_local_project` on a given store, with the same single transaction as
/// `create_project_in_store`.
fn save_project_in_store(path: PathBuf, request: SaveLocalProjectRequest) -> Result<LocalProjectEnvelope, String> {
    let (mut connection, migration) = open_project_store(&path)?;
    // One write transaction from the preparation's reads to the commit.
    let transaction = immediate_transaction(&mut connection)?;
    let editor_intents = normalized_editor_intents(Some(request.editor_intents));
    let proposal = normalized_proposal(Some(request.proposal));
    let selected_review_target =
        normalized_selected_review_target(Some(request.selected_review_target));
    let mechanics_result = request.mechanics_result;
    let analysis_run = request.analysis_run;
    let model = request.model;
    let model_hash = request.model_hash;
    let project_envelope_hash = request.project_envelope_hash;
    let document_version = check_model_document_for_persist(&model)?;
    guarded_write_project_rows(
        &transaction,
        request.confirmed_revision.as_deref(),
        request.expected_base.as_deref(),
        &request.project_id,
        &request.project_name,
        &model,
        &editor_intents,
        &proposal,
        &selected_review_target,
        &mechanics_result,
        &analysis_run,
        &model_hash,
        &project_envelope_hash,
    )?;
    transaction.commit().map_err(|error| error.to_string())?;
    Ok(LocalProjectEnvelope {
        summary: project_summary(
            request.project_id,
            request.project_name,
            &model,
            path,
            &migration,
            &editor_intents,
            &proposal,
            &selected_review_target,
            &mechanics_result,
            &analysis_run,
            &model_hash,
            &project_envelope_hash,
            "Saved current project model snapshot without external file copies.".to_string(),
        ),
        model,
        editor_intents,
        proposal,
        selected_review_target,
        mechanics_result,
        analysis_run,
        model_hash,
        project_envelope_hash,
        model_document_version: serde_json::to_value(document_version)
            .map_err(|error| error.to_string())?,
    })
}

// ---------------------------------------------------------------------------
// Local rule-pack storage and validation commands (Phase C2 backend seam).
//
// Rule packs are separate user-controlled documents
// (schemas/rule_pack.schema.yaml), not members of the preview model
// document. They persist only in the local SQLite store
// (local_rule_packs, store v10) and are never committed to the repository,
// transmitted, or exported by default (OPS-K-PRIV-1, PRD §17.3). Validation
// and checksum computation route through core/rules via
// open_pipe_stress_rule_pack_document; statuses are software findings only
// and never professional, certification, sealing, authentication, or
// code-compliance claims (OPS-K-AUTH-1).
// ---------------------------------------------------------------------------

#[derive(Debug, Serialize)]
struct LocalRulePackIndexEntry {
    project_id: String,
    rule_pack_id: String,
    rule_pack_name: String,
    rule_pack_version: String,
    lifecycle_status: String,
    privacy_class: String,
    storage_mode: &'static str,
    created_at_unix: i64,
    updated_at_unix: i64,
}

#[derive(Debug, Serialize)]
struct LocalRulePackEnvelope {
    project_id: String,
    rule_pack_id: String,
    storage_mode: &'static str,
    created_at_unix: i64,
    updated_at_unix: i64,
    document: Value,
    validation: Value,
    message: String,
}

#[derive(Debug, Serialize)]
struct LocalRulePackDeleteReceipt {
    project_id: String,
    rule_pack_id: String,
    deleted: bool,
    message: String,
}

fn rule_pack_document_string(document: &Value, pointer: &str) -> String {
    document
        .pointer(pointer)
        .and_then(Value::as_str)
        .unwrap_or("TBD")
        .to_string()
}

fn rule_pack_id_from_document(document: &Value) -> Result<String, String> {
    document
        .pointer("/metadata/rule_pack_id")
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|id| !id.is_empty())
        .map(str::to_string)
        .ok_or_else(|| {
            "rule-pack document must carry a non-empty metadata.rule_pack_id".to_string()
        })
}

fn rule_pack_validation_value(document: &Value, public_export_requested: bool) -> Value {
    serde_json::to_value(rule_pack_document::validate_rule_pack_document(
        document,
        public_export_requested,
    ))
    .unwrap_or_else(|error| {
        json!({
            "document_kind": "openpipestress.rule_pack.document_validation",
            "serialization_error": error.to_string(),
        })
    })
}

/// The validation embedded in a blocked-run response (W-17, DEL-06-02
/// `Design/rule-pack-unit-check.md` §2.6): the whole validation without its
/// `unit_check`. Unit-check diagnostics carry pack ids and unit tokens, which
/// may be private; they appear only in validation responses to the local Rule
/// Pack Manager (`validate_rule_pack`, save and open), never in run responses.
fn run_response_rule_pack_validation_value(document: &Value) -> Value {
    let mut validation = rule_pack_validation_value(document, false);
    if let Some(members) = validation.as_object_mut() {
        members.remove("unit_check");
    }
    validation
}

struct StoredRulePackRow {
    document: Value,
    created_at_unix: i64,
    updated_at_unix: i64,
}

fn upsert_rule_pack(
    connection: &Connection,
    project_id: &str,
    rule_pack_id: &str,
    document: &Value,
) -> Result<(i64, i64), String> {
    let document_json = serde_json::to_string(document).map_err(|error| error.to_string())?;
    let now = now_unix_seconds()?;
    connection
        .execute(
            "
            INSERT INTO local_rule_packs (
                project_id, rule_pack_id, document_json, created_at_unix, updated_at_unix
            )
            VALUES (?1, ?2, ?3, ?4, ?4)
            ON CONFLICT(project_id, rule_pack_id) DO UPDATE SET
                document_json = excluded.document_json,
                updated_at_unix = excluded.updated_at_unix
            ",
            params![project_id, rule_pack_id, document_json, now],
        )
        .map_err(|error| error.to_string())?;
    let created_at: i64 = connection
        .query_row(
            "SELECT created_at_unix FROM local_rule_packs WHERE project_id = ?1 AND rule_pack_id = ?2",
            params![project_id, rule_pack_id],
            |row| row.get(0),
        )
        .map_err(|error| error.to_string())?;
    Ok((created_at, now))
}

fn load_rule_pack(
    connection: &Connection,
    project_id: &str,
    rule_pack_id: &str,
) -> Result<Option<StoredRulePackRow>, String> {
    connection
        .query_row(
            "
            SELECT document_json, created_at_unix, updated_at_unix
            FROM local_rule_packs
            WHERE project_id = ?1 AND rule_pack_id = ?2
            ",
            params![project_id, rule_pack_id],
            |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, i64>(1)?,
                    row.get::<_, i64>(2)?,
                ))
            },
        )
        .optional()
        .map_err(|error| error.to_string())?
        .map(|(document_json, created_at_unix, updated_at_unix)| {
            serde_json::from_str(&document_json)
                .map(|document| StoredRulePackRow {
                    document,
                    created_at_unix,
                    updated_at_unix,
                })
                .map_err(|error| error.to_string())
        })
        .transpose()
}

fn list_rule_packs(
    connection: &Connection,
    project_id: Option<&str>,
) -> Result<Vec<LocalRulePackIndexEntry>, String> {
    let mut statement = connection
        .prepare(
            "
            SELECT project_id, rule_pack_id, document_json, created_at_unix, updated_at_unix
            FROM local_rule_packs
            WHERE (?1 IS NULL OR project_id = ?1)
            ORDER BY project_id, rule_pack_id
            ",
        )
        .map_err(|error| error.to_string())?;
    let rows = statement
        .query_map(params![project_id], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, i64>(3)?,
                row.get::<_, i64>(4)?,
            ))
        })
        .map_err(|error| error.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| error.to_string())?;
    rows.into_iter()
        .map(
            |(project_id, rule_pack_id, document_json, created_at_unix, updated_at_unix)| {
                let document: Value =
                    serde_json::from_str(&document_json).map_err(|error| error.to_string())?;
                Ok(LocalRulePackIndexEntry {
                    project_id,
                    rule_pack_id,
                    rule_pack_name: rule_pack_document_string(&document, "/metadata/name"),
                    rule_pack_version: rule_pack_document_string(
                        &document,
                        "/metadata/rule_pack_version",
                    ),
                    lifecycle_status: rule_pack_document_string(
                        &document,
                        "/metadata/lifecycle_status",
                    ),
                    privacy_class: rule_pack_document_string(
                        &document,
                        "/classification/privacy_class",
                    ),
                    storage_mode: "local_sqlite",
                    created_at_unix,
                    updated_at_unix,
                })
            },
        )
        .collect()
}

fn delete_rule_pack(
    connection: &Connection,
    project_id: &str,
    rule_pack_id: &str,
) -> Result<bool, String> {
    let affected = connection
        .execute(
            "DELETE FROM local_rule_packs WHERE project_id = ?1 AND rule_pack_id = ?2",
            params![project_id, rule_pack_id],
        )
        .map_err(|error| error.to_string())?;
    Ok(affected > 0)
}

#[tauri::command]
fn validate_rule_pack(
    invoke: tauri::ipc::Request<'_>,
    registry: tauri::State<'_, SolveJobRegistry>,
) -> Result<tauri::ipc::Response, CommandError> {
    let (mut args, _lease) = command_ingress::admit(&registry, &invoke, "validate_rule_pack", ArgsDecoding::Moved)?;
    respond(validate_rule_pack_args(&mut args))
}

fn validate_rule_pack_args(args: &mut Args) -> Result<Value, CommandError> {
    let document = args.value("document")?;
    let public_export_requested = args.decode::<Option<bool>>("publicExportRequested")?;
    validate_rule_pack_value(document, public_export_requested).map_err(CommandError::Command)
}

fn validate_rule_pack_value(
    document: Value,
    public_export_requested: Option<bool>,
) -> Result<Value, String> {
    Ok(rule_pack_validation_value(
        &document,
        public_export_requested.unwrap_or(false),
    ))
}

/// Run a rule pack's checks against a solved model and return per-check
/// outcomes (completion plan C4). The pack is validated first; a pack with
/// blocking validation findings is reported as `RULE_INPUTS_INCOMPLETE` without
/// running checks. Solver values are resolved from the solved mechanics
/// envelope supplied via `solved_envelope` (a solve-job result; this command
/// does not solve) using the caller's `{input_id, result_id}`
/// selectors; user values and value-slot limits come from
/// `supplied_value_bindings`. This emits only the automatic rule-check statuses
/// and makes no professional, certification, sealing, authentication, approval,
/// or code-compliance claim.
#[tauri::command]
fn run_rule_checks(
    app: AppHandle,
    invoke: tauri::ipc::Request<'_>,
    registry: tauri::State<'_, SolveJobRegistry>,
) -> Result<tauri::ipc::Response, CommandError> {
    let (mut args, _lease) = command_ingress::admit(&registry, &invoke, "run_rule_checks", ArgsDecoding::Moved)?;
    respond(run_rule_checks_args(&mut args, &|| app_store_path(&app)))
}

/// `{rulePackDocument, model?, solvedEnvelope?, solverResultBindings?,
/// suppliedValueBindings?, projectId?, sourceBlockInvocation?}`.
fn run_rule_checks_args(args: &mut Args, store: &dyn Fn() -> Result<PathBuf, String>) -> Result<Value, CommandError> {
    let rule_pack_document = args.value("rulePackDocument")?;
    let model = args.opt_value("model");
    let solved_envelope = args.opt_value("solvedEnvelope");
    let solver_result_bindings = args.opt_value("solverResultBindings");
    let supplied_value_bindings = args.opt_value("suppliedValueBindings");
    let project_id = args.decode::<Option<String>>("projectId")?;
    let source_block_invocation = args.opt_value("sourceBlockInvocation");
    run_rule_checks_value(
        store,
        rule_pack_document,
        model,
        solved_envelope,
        solver_result_bindings,
        supplied_value_bindings,
        project_id,
        source_block_invocation,
    )
    .map_err(CommandError::Command)
}

#[allow(clippy::too_many_arguments)]
fn run_rule_checks_value(
    store: &dyn Fn() -> Result<PathBuf, String>,
    rule_pack_document: Value,
    model: Option<Value>,
    solved_envelope: Option<Value>,
    solver_result_bindings: Option<Value>,
    supplied_value_bindings: Option<Value>,
    project_id: Option<String>,
    source_block_invocation: Option<Value>,
) -> Result<Value, String> {
    let supplied_model = model.as_ref().ok_or(
        "RULE_NUMERICAL_CASE_COVERAGE_UNAVAILABLE: supply the current model with explicit load cases and recompute mechanics",
    )?;
    // Every solve is admitted through the solve-job path and its memory pool;
    // this command never runs an unadmitted solve of its own.
    let envelope = solved_envelope.ok_or(
        "RULE_CHECK_SOLVED_ENVELOPE_REQUIRED: run the solve job and supply its result",
    )?;
    let invocation = source_block_invocation;
    qualify_rule_mechanics_with_context(supplied_model, &envelope, invocation.as_ref())?;
    // Resolve `private_library_value` inputs from the local private-library
    // store (C3 rule-pack <-> library reference wiring). Library values are read
    // at run time and never embedded in the rule pack (IP boundary). A reference
    // that cannot be resolved is omitted, so the input stays unsupplied and the
    // check blocks at RULE_INPUTS_INCOMPLETE — never a silent pass.
    let resolved_project = resolve_rule_check_project_id(
        project_id.as_deref(),
        model.as_ref(),
        Some(&envelope),
    );
    let library_value_bindings = match resolved_project {
        Some(project) => Some(resolve_library_value_bindings(
            store,
            &project,
            &rule_pack_document,
        )?),
        None => None,
    };
    run_rule_checks_core(
        rule_pack_document,
        envelope,
        solver_result_bindings,
        supplied_value_bindings,
        library_value_bindings,
    )
}

/// Numerical interpretation only: input/build/authentic-source authorization remains
/// with the current-source caller. Never derive coverage from result rows.
fn qualify_rule_mechanics(model: &Value, envelope: &Value) -> Result<(), String> {
    qualify_rule_mechanics_with_context(model, envelope, None)
}
fn qualify_rule_mechanics_with_context(model: &Value, envelope: &Value, invocation: Option<&Value>) -> Result<(), String> {
    if let Some(context) = invocation {
        if context.pointer("/request/model") != Some(model) { return Err("RULE_NUMERICAL_INVOCATION_MODEL_MISMATCH".into()); }
    }
    if envelope.pointer("/status/mechanics").and_then(Value::as_str) != Some("MECHANICS_SOLVED") {
        return Err("RULE_MECHANICS_NOT_SOLVED: solve the current model before binding solver results".into());
    }
    let project = model.pointer("/project/id").and_then(Value::as_str)
        .filter(|id| !id.trim().is_empty())
        .ok_or("RULE_NUMERICAL_CASE_COVERAGE_UNAVAILABLE: current model project.id is required")?;
    if envelope["model_ref"].as_str() != Some(project) {
        return Err("RULE_NUMERICAL_PROJECT_MISMATCH: recompute for the supplied model".into());
    }
    let cases = model["load_cases"].as_array().filter(|cases| !cases.is_empty())
        .ok_or("RULE_NUMERICAL_CASE_COVERAGE_UNAVAILABLE: supply nonempty current model.load_cases")?;
    let mut ids = std::collections::HashSet::new();
    let mut refs = Vec::with_capacity(cases.len());
    for case in cases {
        let id = case["id"].as_str().filter(|id| !id.trim().is_empty())
            .ok_or("RULE_NUMERICAL_CASE_COVERAGE_UNAVAILABLE: every load case needs an explicit id")?;
        if !ids.insert(id) {
            return Err("RULE_NUMERICAL_CASE_COVERAGE_UNAVAILABLE: duplicate load case id".into());
        }
        refs.push(json!({"ref_type":"load_case", "ref_id":id}));
    }
    // T0R: only a fresh identity is rule-eligible (precision-1 and
    // source-blocks-1 are refused by name, B1 and B4/D1 (b)).
    // A2 item 10: validate first, so a tampered source is unsupported.
    let standing = open_pipe_stress_result_export::semantic_contract::numerical_use_standing_with_context(envelope, &refs, invocation);
    let contract = envelope.pointer("/producer/semantic_contract_id").and_then(Value::as_str).unwrap_or_default();
    if standing != "unsupported" {
        if let Some(reason) = open_pipe_stress_result_export::semantic_contract::standing_reason(envelope) {
            return Err(format!("RULE_NUMERICAL_NEEDS_RECOMPUTE: {reason}"));
        }
    }
    // A retired 0.1.0 source carries no producer id; its reader refuses it, so it is unsupported below.
    if !contract.is_empty() && !open_pipe_stress_result_export::semantic_contract::is_fresh_identity(contract) {
        return Err("RULE_NUMERICAL_SOURCE_UNSUPPORTED: recompute with a supported producer and semantic contract".into());
    }
    match standing {
        "numerically_eligible" => Ok(()),
        "unsupported" => Err("RULE_NUMERICAL_SOURCE_UNSUPPORTED: recompute with a supported producer and semantic contract".into()),
        _ => Err("RULE_NUMERICAL_NEEDS_RECOMPUTE: complete passing or source-qualified numerical evidence is required for every current load case".into()),
    }
}

/// Store-free rule-check orchestration. The Tauri command resolves library
/// references from the local store into `library_value_bindings`, then calls
/// this; tests target it directly with already-resolved bindings. It requires
/// the solved envelope: it never solves (every solve runs in the pool's slot).
fn run_rule_checks_core(
    rule_pack_document: Value,
    envelope: Value,
    solver_result_bindings: Option<Value>,
    supplied_value_bindings: Option<Value>,
    library_value_bindings: Option<Value>,
) -> Result<Value, String> {
    let validation = rule_pack_document::validate_rule_pack_document(&rule_pack_document, false);
    if validation.has_blocking_findings {
        return Ok(json!({
            "document_kind": rule_check_runner::DOCUMENT_KIND,
            "rule_pack_id": rule_pack_document
                .pointer("/metadata/rule_pack_id")
                .and_then(Value::as_str)
                .unwrap_or_default(),
            "grammar_version": rule_pack_document
                .pointer("/grammar_version")
                .and_then(Value::as_str)
                .unwrap_or_default(),
            "aggregate_status": "RULE_INPUTS_INCOMPLETE",
            "checks": [],
            "rule_pack_validation": run_response_rule_pack_validation_value(&rule_pack_document),
            "professional_boundary_notice": rule_check_runner::PROFESSIONAL_BOUNDARY_NOTICE,
        }));
    }

    // Solver values for `solver_result` inputs come from two sources: an
    // authored in-pack `solver_result_ref` (the additive solver-result-selector
    // member, canonical) and the legacy caller-supplied `{input_id, result_id}`
    // selectors (a fallback). An input that carries an authored reference is
    // governed by it alone — the caller-supplied selector for that input is
    // dropped (no run-time override; the authored reference even blocks the
    // input when it does not resolve, never a caller rescue). Inputs without an
    // authored reference still bind from caller-supplied selectors (backward
    // compatible: a pack with no authored references behaves exactly as before).
    let mut refused_solver_results = Vec::new();
    let (mut solver_results, authored_solver_input_ids) =
        resolve_authored_solver_result_bindings(&rule_pack_document, &envelope, &mut refused_solver_results);
    let mut caller_refused = Vec::new();
    let caller_solver_results =
        resolve_solver_result_bindings(solver_result_bindings.as_ref(), &envelope, &mut caller_refused)?;
    solver_results.extend(
        caller_solver_results
            .into_iter()
            .filter(|binding| !authored_solver_input_ids.contains(&binding.input_id)),
    );
    refused_solver_results.extend(
        caller_refused
            .into_iter()
            .filter(|refusal| !authored_solver_input_ids.contains(&refusal.input_id)),
    );
    let supplied_values = parse_supplied_value_bindings(supplied_value_bindings.as_ref())?;
    let library_values = parse_library_value_bindings(library_value_bindings.as_ref())?;

    let current_statuses = match envelope
        .pointer("/status/mechanics")
        .and_then(Value::as_str)
    {
        Some("MECHANICS_SOLVED") => vec![rule_check_runner::AnalysisStatus::MechanicsSolved],
        _ => vec![rule_check_runner::AnalysisStatus::ModelIncomplete],
    };

    let result = rule_check_runner::run_rule_checks(&rule_check_runner::RuleCheckRunInput {
        rule_pack_document: &rule_pack_document,
        solver_results,
        refused_solver_results,
        supplied_values,
        library_values,
        current_statuses,
    });

    serde_json::to_value(result).map_err(|error| error.to_string())
}

/// Resolve the project id used to scope the private-library lookup: an explicit
/// `project_id`, else the model's `/project/id`, else the solved envelope's
/// `/model_ref` (all of which carry the project id).
fn resolve_rule_check_project_id(
    explicit: Option<&str>,
    model: Option<&Value>,
    solved_envelope: Option<&Value>,
) -> Option<String> {
    let from = |value: Option<&str>| {
        value
            .map(str::trim)
            .filter(|id| !id.is_empty())
            .map(str::to_string)
    };
    from(explicit)
        .or_else(|| {
            from(
                model
                    .and_then(|m| m.pointer("/project/id"))
                    .and_then(Value::as_str),
            )
        })
        .or_else(|| {
            from(
                solved_envelope
                    .and_then(|e| e.pointer("/model_ref"))
                    .and_then(Value::as_str),
            )
        })
}

/// Resolve every `private_library_value` required input that carries a
/// `library_value_ref` to a `{input_id, value, unit, ...}` binding by reading
/// the referenced record's value slot from the local private-library store.
/// Unresolvable references are omitted (the input then blocks, never a silent
/// pass). Returns a JSON array consumed by `parse_library_value_bindings`.
fn resolve_library_value_bindings(
    store: &dyn Fn() -> Result<PathBuf, String>,
    project_id: &str,
    document: &Value,
) -> Result<Value, String> {
    let Some(required_inputs) = document.get("required_inputs").and_then(Value::as_array) else {
        return Ok(Value::Array(Vec::new()));
    };
    let needs_store = required_inputs.iter().any(|input| {
        input.pointer("/source_kind").and_then(Value::as_str) == Some("private_library_value")
            && input.get("library_value_ref").is_some()
    });
    if !needs_store {
        return Ok(Value::Array(Vec::new()));
    }

    let path = store()?;
    let (connection, _migration) = open_project_store(&path)?;
    resolve_library_value_bindings_with_connection(&connection, project_id, document)
}

/// Store-bound core of `resolve_library_value_bindings`, separated so tests can
/// drive it with an in-memory connection.
fn resolve_library_value_bindings_with_connection(
    connection: &Connection,
    project_id: &str,
    document: &Value,
) -> Result<Value, String> {
    let Some(required_inputs) = document.get("required_inputs").and_then(Value::as_array) else {
        return Ok(Value::Array(Vec::new()));
    };
    let mut bindings: Vec<Value> = Vec::new();
    for input in required_inputs {
        if input.pointer("/source_kind").and_then(Value::as_str) != Some("private_library_value") {
            continue;
        }
        let Some(reference) = input.get("library_value_ref") else {
            continue;
        };
        let input_id = input
            .pointer("/input_id")
            .and_then(Value::as_str)
            .unwrap_or_default();
        let library_kind = reference
            .pointer("/library_kind")
            .and_then(Value::as_str)
            .unwrap_or_default();
        let library_id = reference
            .pointer("/library_id")
            .and_then(Value::as_str)
            .unwrap_or_default();
        let record_id = reference
            .pointer("/record_id")
            .and_then(Value::as_str)
            .unwrap_or_default();
        let slot_id = reference
            .pointer("/slot_id")
            .and_then(Value::as_str)
            .unwrap_or_default();
        if input_id.is_empty()
            || library_kind.is_empty()
            || library_id.is_empty()
            || record_id.is_empty()
            || slot_id.is_empty()
        {
            continue;
        }
        let Some(stored) = load_library(connection, project_id, library_kind, library_id)? else {
            continue;
        };
        let Some((value, unit)) =
            extract_library_slot_value(&stored.document, library_kind, record_id, slot_id)
        else {
            continue;
        };
        bindings.push(json!({
            "input_id": input_id,
            "value": value,
            "unit": unit,
            "library_kind": library_kind,
            "library_id": library_id,
            "record_id": record_id,
            "slot_id": slot_id,
        }));
    }
    Ok(Value::Array(bindings))
}

/// Read `{value, unit}` from a library record's value slot, for every private
/// library kind a rule-pack `library_value_ref` can target. Per the library
/// schemas the record id, slot array(s), slot id, and unit metadata differ by
/// kind:
///
/// - `material`: `material_records[material_id].allowables[allowable_id]`
///   (the `user_rule_allowable` / `private_reference` rule-check source);
/// - `section`: `section_records[section_id].dimensions[dimension_id]` or
///   `…properties[property_id]`;
/// - `component`: `component_records[component_id].fields[field_id]`.
///
/// Material values carry `value.unit_ref.ref_id` (a unit reference); section and
/// component values carry a plain `value.unit` string. Returns `None` — so the
/// caller omits the binding and the check blocks, never a silent pass — when the
/// kind is unknown or the record/slot/value is absent or non-numeric. The
/// private value is read here at run time and is never embedded in the rule pack
/// (IP boundary).
fn extract_library_slot_value(
    document: &Value,
    library_kind: &str,
    record_id: &str,
    slot_id: &str,
) -> Option<(f64, String)> {
    let value = match library_kind {
        "material" => find_library_slot_value(
            document,
            "material_records",
            "material_id",
            record_id,
            &[("allowables", "allowable_id")],
            slot_id,
        ),
        "section" => find_library_slot_value(
            document,
            "section_records",
            "section_id",
            record_id,
            &[
                ("dimensions", "dimension_id"),
                ("properties", "property_id"),
            ],
            slot_id,
        ),
        "component" => find_library_slot_value(
            document,
            "component_records",
            "component_id",
            record_id,
            &[("fields", "field_id")],
            slot_id,
        ),
        _ => None,
    }?;
    let magnitude = parse_quantity_magnitude(value.get("magnitude"))?;
    // Material values reference a unit (`unit_ref.ref_id`); section/component
    // values carry a plain `unit` string. The two are mutually exclusive per the
    // schemas (`additionalProperties: false`), so read whichever this slot has.
    let unit = value
        .pointer("/unit_ref/ref_id")
        .or_else(|| value.get("unit"))
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_string();
    Some((magnitude, unit))
}

/// Find a value slot inside a library record and return its `value` object.
/// Looks up the record by `record_id_key == record_id` in `records_member`, then
/// scans each `(slot_array, slot_id_key)` in order for `slot_id_key == slot_id`,
/// returning the first match's `value`. `None` if the record, every slot array,
/// the matching slot, or its `value` is absent.
fn find_library_slot_value<'a>(
    document: &'a Value,
    records_member: &str,
    record_id_key: &str,
    record_id: &str,
    slot_arrays: &[(&str, &str)],
    slot_id: &str,
) -> Option<&'a Value> {
    let record = document
        .get(records_member)
        .and_then(Value::as_array)?
        .iter()
        .find(|record| record.get(record_id_key).and_then(Value::as_str) == Some(record_id))?;
    for (slot_array, slot_id_key) in slot_arrays {
        let matched = record
            .get(*slot_array)
            .and_then(Value::as_array)
            .and_then(|slots| {
                slots
                    .iter()
                    .find(|slot| slot.get(*slot_id_key).and_then(Value::as_str) == Some(slot_id))
            });
        if let Some(value) = matched.and_then(|slot| slot.get("value")) {
            return Some(value);
        }
    }
    None
}

/// Parse a library quantity magnitude that the schemas allow to be either a JSON
/// number or a numeric string. `None` when absent or non-numeric.
fn parse_quantity_magnitude(magnitude: Option<&Value>) -> Option<f64> {
    match magnitude {
        Some(Value::Number(number)) => number.as_f64(),
        Some(Value::String(text)) => text.trim().parse::<f64>().ok(),
        _ => None,
    }
}

/// Parse caller-resolved `{input_id, value, unit, library_kind, library_id,
/// record_id, slot_id}` library values for `private_library_value` inputs.
fn parse_library_value_bindings(
    bindings: Option<&Value>,
) -> Result<Vec<rule_check_runner::LibraryValueBinding>, String> {
    let Some(bindings) = bindings.filter(|value| !value.is_null()) else {
        return Ok(Vec::new());
    };
    let array = bindings
        .as_array()
        .ok_or_else(|| "library_value_bindings must be an array".to_string())?;
    let mut parsed = Vec::new();
    for binding in array {
        let input_id = binding
            .pointer("/input_id")
            .and_then(Value::as_str)
            .ok_or_else(|| "each library value binding requires input_id".to_string())?;
        let value = binding
            .pointer("/value")
            .and_then(Value::as_f64)
            .ok_or_else(|| {
                format!("library value binding '{input_id}' requires a numeric value")
            })?;
        let str_field = |key: &str| {
            binding
                .pointer(&format!("/{key}"))
                .and_then(Value::as_str)
                .unwrap_or_default()
                .to_string()
        };
        parsed.push(rule_check_runner::LibraryValueBinding {
            input_id: input_id.to_string(),
            value,
            unit: str_field("unit"),
            library_kind: str_field("library_kind"),
            library_id: str_field("library_id"),
            record_id: str_field("record_id"),
            slot_id: str_field("slot_id"),
        });
    }
    Ok(parsed)
}

/// Read `{value, unit}` from a solved mechanics envelope's `results[]` row whose
/// `id` matches `result_id`. Returns `None` when no row matches, or the matching
/// row lacks a numeric `value` or a string `unit` — so the caller omits the
/// binding and the required input stays unsupplied (the check blocks, never a
/// silent pass). Shared by the caller-supplied selector path and the authored
/// `solver_result_ref` path so both address result rows identically. A row the
/// shared `rule_binding_refusal` helper refuses (T0R) yields `Err(reason)`; the
/// caller records it as a refused input, reported incomplete with the reason.
fn solver_result_row_value(envelope: &Value, result_id: &str) -> Result<Option<(f64, String)>, &'static str> {
    let Some(row) = envelope
        .pointer("/results")
        .and_then(Value::as_array)
        .and_then(|rows| rows.iter().find(|row| row.pointer("/id").and_then(Value::as_str) == Some(result_id)))
    else {
        return Ok(None);
    };
    if let Some(reason) = open_pipe_stress_result_export::semantic_contract::rule_binding_refusal(envelope, row) {
        return Err(reason);
    }
    let Some(value) = row.pointer("/value").and_then(Value::as_f64) else { return Ok(None) };
    let Some(unit) = row.pointer("/unit").and_then(Value::as_str) else { return Ok(None) };
    Ok(Some((value, unit.to_string())))
}

/// Resolve caller-supplied `{input_id, result_id}` selectors against a solved
/// mechanics envelope's `results[]`, reading `value`/`unit` by result id. A
/// selector with no matching row (or a row missing value/unit) is omitted, so
/// the required input is treated as unsupplied and the check blocks — never a
/// silent pass. This is the legacy caller-supplied solver binding; an authored
/// `solver_result_ref` in the pack supersedes it (see
/// [`resolve_authored_solver_result_bindings`]).
fn resolve_solver_result_bindings(
    selectors: Option<&Value>,
    envelope: &Value,
    refused: &mut Vec<rule_check_runner::RefusedSolverResult>,
) -> Result<Vec<rule_check_runner::SolverResultBinding>, String> {
    let Some(selectors) = selectors.filter(|value| !value.is_null()) else {
        return Ok(Vec::new());
    };
    let array = selectors
        .as_array()
        .ok_or_else(|| "solver_result_bindings must be an array".to_string())?;
    let mut bindings = Vec::new();
    for selector in array {
        let input_id = selector
            .pointer("/input_id")
            .and_then(Value::as_str)
            .ok_or_else(|| "each solver_result binding requires input_id".to_string())?;
        let result_id = selector
            .pointer("/result_id")
            .and_then(Value::as_str)
            .ok_or_else(|| "each solver_result binding requires result_id".to_string())?;
        match solver_result_row_value(envelope, result_id) {
            Ok(Some((value, unit))) => bindings.push(rule_check_runner::SolverResultBinding {
                input_id: input_id.to_string(),
                result_id: result_id.to_string(),
                value,
                unit,
            }),
            Ok(None) => {}
            Err(reason) => refused.push(rule_check_runner::RefusedSolverResult {
                input_id: input_id.to_string(),
                reason: reason.to_string(),
            }),
        }
    }
    Ok(bindings)
}

/// Resolve every `solver_result` required input that carries an authored
/// `solver_result_ref` (the additive solver-result-selector schema member;
/// PROPOSAL, precedent `DEC-038` / `library_value_ref`) to a
/// [`rule_check_runner::SolverResultBinding`] by reading the referenced row from
/// the solved envelope's `results[]` by id. The authored reference is the
/// canonical, in-pack form of the previously caller-supplied solver binding.
///
/// Returns the resolved bindings plus the set of input ids that carry an
/// authored reference (resolvable or not), so the caller can exclude those
/// inputs from the legacy caller-supplied fallback: an authored reference
/// governs its input alone — the caller cannot override or rescue it at run
/// time (run-time override is a deferred non-goal, matching the
/// `library_value_ref` ruling). An unresolvable reference yields no binding (its
/// input then blocks at `RULE_INPUTS_INCOMPLETE`, never a silent pass).
fn resolve_authored_solver_result_bindings(
    document: &Value,
    envelope: &Value,
    refused: &mut Vec<rule_check_runner::RefusedSolverResult>,
) -> (Vec<rule_check_runner::SolverResultBinding>, HashSet<String>) {
    let mut bindings = Vec::new();
    let mut authored_input_ids = HashSet::new();
    let Some(required_inputs) = document.get("required_inputs").and_then(Value::as_array) else {
        return (bindings, authored_input_ids);
    };
    for input in required_inputs {
        if input.pointer("/source_kind").and_then(Value::as_str) != Some("solver_result") {
            continue;
        }
        let Some(reference) = input.get("solver_result_ref") else {
            continue;
        };
        let input_id = input
            .pointer("/input_id")
            .and_then(Value::as_str)
            .unwrap_or_default();
        if input_id.is_empty() {
            // No usable key; treat as if it carried no authored reference.
            continue;
        }
        // The input carries an authored reference: it is governed by that
        // reference alone, even if the reference does not resolve.
        authored_input_ids.insert(input_id.to_string());
        let result_id = reference
            .pointer("/result_id")
            .and_then(Value::as_str)
            .unwrap_or_default();
        if result_id.is_empty() {
            continue;
        }
        match solver_result_row_value(envelope, result_id) {
            Ok(Some((value, unit))) => bindings.push(rule_check_runner::SolverResultBinding {
                input_id: input_id.to_string(),
                result_id: result_id.to_string(),
                value,
                unit,
            }),
            Ok(None) => {}
            Err(reason) => refused.push(rule_check_runner::RefusedSolverResult {
                input_id: input_id.to_string(),
                reason: reason.to_string(),
            }),
        }
    }
    (bindings, authored_input_ids)
}

/// Parse caller-supplied `{ref_id, value, unit, dimension}` user values (for
/// `user_supplied_rule_value` inputs and value-slot limits).
fn parse_supplied_value_bindings(
    bindings: Option<&Value>,
) -> Result<Vec<rule_check_runner::SuppliedValueBinding>, String> {
    let Some(bindings) = bindings.filter(|value| !value.is_null()) else {
        return Ok(Vec::new());
    };
    let array = bindings
        .as_array()
        .ok_or_else(|| "supplied_value_bindings must be an array".to_string())?;
    let mut parsed = Vec::new();
    for binding in array {
        let ref_id = binding
            .pointer("/ref_id")
            .and_then(Value::as_str)
            .ok_or_else(|| "each supplied value binding requires ref_id".to_string())?;
        let value = binding
            .pointer("/value")
            .and_then(Value::as_f64)
            .ok_or_else(|| format!("supplied value binding '{ref_id}' requires a numeric value"))?;
        let unit = binding
            .pointer("/unit")
            .and_then(Value::as_str)
            .unwrap_or_default()
            .to_string();
        let dimension = binding
            .pointer("/dimension")
            .and_then(Value::as_str)
            .unwrap_or_default()
            .to_string();
        parsed.push(rule_check_runner::SuppliedValueBinding {
            ref_id: ref_id.to_string(),
            value,
            unit,
            dimension,
        });
    }
    Ok(parsed)
}

#[tauri::command]
fn compute_rule_pack_document_checksum(
    invoke: tauri::ipc::Request<'_>,
    registry: tauri::State<'_, SolveJobRegistry>,
) -> Result<tauri::ipc::Response, CommandError> {
    let (mut args, _lease) =
        command_ingress::admit(&registry, &invoke, "compute_rule_pack_document_checksum", ArgsDecoding::Moved)?;
    respond(compute_rule_pack_document_checksum_args(&mut args))
}

fn compute_rule_pack_document_checksum_args(args: &mut Args) -> Result<Value, CommandError> {
    let document = args.value("document")?;
    compute_rule_pack_document_checksum_value(document).map_err(CommandError::Command)
}

fn compute_rule_pack_document_checksum_value(document: Value) -> Result<Value, String> {
    rule_pack_document::compute_rule_pack_checksum(&document)
        .map_err(|error| error.to_string())
        .and_then(|computed| serde_json::to_value(computed).map_err(|error| error.to_string()))
}

#[tauri::command]
fn save_local_rule_pack(
    app: AppHandle,
    invoke: tauri::ipc::Request<'_>,
    registry: tauri::State<'_, SolveJobRegistry>,
) -> Result<tauri::ipc::Response, CommandError> {
    let (mut args, _lease) = command_ingress::admit(&registry, &invoke, "save_local_rule_pack", ArgsDecoding::Moved)?;
    respond(save_local_rule_pack_args(&mut args, || app_store_path(&app)))
}

/// `{projectId, document}`.
fn save_local_rule_pack_args(
    args: &mut Args,
    store: impl FnOnce() -> Result<PathBuf, String>,
) -> Result<LocalRulePackEnvelope, CommandError> {
    let project_id = args.decode::<String>("projectId")?;
    let document = args.value("document")?;
    save_local_rule_pack_value(store, project_id, document).map_err(CommandError::Command)
}

fn save_local_rule_pack_value(
    store: impl FnOnce() -> Result<PathBuf, String>,
    project_id: String,
    document: Value,
) -> Result<LocalRulePackEnvelope, String> {
    let project_id = project_id.trim().to_string();
    if project_id.is_empty() {
        return Err("project_id must be non-empty".to_string());
    }
    let rule_pack_id = rule_pack_id_from_document(&document)?;
    let path = store()?;
    let (connection, _migration) = open_project_store(&path)?;
    let (created_at_unix, updated_at_unix) =
        upsert_rule_pack(&connection, &project_id, &rule_pack_id, &document)?;
    let validation = rule_pack_validation_value(&document, false);
    Ok(LocalRulePackEnvelope {
        project_id,
        rule_pack_id,
        storage_mode: "local_sqlite",
        created_at_unix,
        updated_at_unix,
        document,
        validation,
        message: "Saved rule-pack document to the local SQLite store only; drafts with \
                  findings remain saveable and are never transmitted or committed."
            .to_string(),
    })
}

#[tauri::command]
fn open_local_rule_pack(
    app: AppHandle,
    project_id: String,
    rule_pack_id: String,
) -> Result<Option<LocalRulePackEnvelope>, String> {
    let path = app_store_path(&app)?;
    let (connection, _migration) = open_project_store(&path)?;
    Ok(
        load_rule_pack(&connection, project_id.trim(), rule_pack_id.trim())?.map(|row| {
            let validation = rule_pack_validation_value(&row.document, false);
            LocalRulePackEnvelope {
                project_id: project_id.trim().to_string(),
                rule_pack_id: rule_pack_id.trim().to_string(),
                storage_mode: "local_sqlite",
                created_at_unix: row.created_at_unix,
                updated_at_unix: row.updated_at_unix,
                document: row.document,
                validation,
                message: "Opened rule-pack document from the local SQLite store.".to_string(),
            }
        }),
    )
}

#[tauri::command]
fn list_local_rule_packs(
    app: AppHandle,
    project_id: Option<String>,
) -> Result<Vec<LocalRulePackIndexEntry>, String> {
    let path = app_store_path(&app)?;
    let (connection, _migration) = open_project_store(&path)?;
    list_rule_packs(&connection, project_id.as_deref().map(str::trim))
}

#[tauri::command]
fn delete_local_rule_pack(
    app: AppHandle,
    project_id: String,
    rule_pack_id: String,
) -> Result<LocalRulePackDeleteReceipt, String> {
    let path = app_store_path(&app)?;
    let (connection, _migration) = open_project_store(&path)?;
    let deleted = delete_rule_pack(&connection, project_id.trim(), rule_pack_id.trim())?;
    Ok(LocalRulePackDeleteReceipt {
        project_id: project_id.trim().to_string(),
        rule_pack_id: rule_pack_id.trim().to_string(),
        deleted,
        message: if deleted {
            "Deleted rule-pack document from the local SQLite store.".to_string()
        } else {
            "No stored rule-pack document matched; nothing was deleted.".to_string()
        },
    })
}

// ---------------------------------------------------------------------------
// Library-import provenance validation seam (Phase C3, TP-C3-IMPORTCMD-001).
//
// Exposes the runtime library-import provenance crate
// (open_pipe_stress_library_import_document, the DEL-03-07 port) through the
// desktop boundary so the GUI import wizard (a later C3 slice) can validate an
// already-parsed material/section/component import payload before it is stored
// in a local-only private library. Validation is a pure function over the
// payload — no persistence and no file parsing in this seam. Every status is a
// software finding only; the command makes no legal license, redistribution,
// certification, sealing, authentication, or code-compliance determination
// (OPS-K-AUTH-1, IP boundary). Unsupported library_kind / intended_visibility
// tokens are rejected, never guessed.
// ---------------------------------------------------------------------------

const LIBRARY_IMPORT_BOUNDARY_NOTICE: &str =
    "Library-import validation reports software findings only over an already-parsed import \
     payload. It makes no legal license, redistribution, certification, sealing, \
     authentication, or code-compliance determination; unresolved redistribution rights and \
     suspected protected content remain findings for human review.";

fn library_import_validation_value(
    payload: &Value,
    library_kind: &str,
    intended_visibility: &str,
) -> Result<Value, String> {
    let result = library_import_document::validate_library_import_tokens(
        payload,
        library_kind,
        intended_visibility,
    )?;
    let has_blocking_findings = result
        .findings
        .iter()
        .any(|finding| finding.severity == "blocking" || finding.severity == "quarantine");
    let findings = serde_json::to_value(&result.findings).map_err(|error| error.to_string())?;
    let library_kind_token =
        serde_json::to_value(result.library_kind).map_err(|error| error.to_string())?;
    let intended_visibility_token =
        serde_json::to_value(result.intended_visibility).map_err(|error| error.to_string())?;
    Ok(json!({
        "document_kind": "openpipestress.library_import.validation",
        "outcome": result.outcome,
        "library_kind": library_kind_token,
        "intended_visibility": intended_visibility_token,
        "accepted": result.accepted,
        "has_blocking_findings": has_blocking_findings,
        "findings": findings,
        "diagnostics": result.diagnostics(),
        "professional_boundary_notice": LIBRARY_IMPORT_BOUNDARY_NOTICE,
    }))
}

#[tauri::command]
fn validate_library_import(
    invoke: tauri::ipc::Request<'_>,
    registry: tauri::State<'_, SolveJobRegistry>,
) -> Result<tauri::ipc::Response, CommandError> {
    let (mut args, _lease) = command_ingress::admit(&registry, &invoke, "validate_library_import", ArgsDecoding::Moved)?;
    respond(validate_library_import_args(&mut args))
}

/// `{payload, libraryKind, intendedVisibility}`.
fn validate_library_import_args(args: &mut Args) -> Result<Value, CommandError> {
    let payload = args.value("payload")?;
    let library_kind = args.decode::<String>("libraryKind")?;
    let intended_visibility = args.decode::<String>("intendedVisibility")?;
    validate_library_import_value(payload, library_kind, intended_visibility).map_err(CommandError::Command)
}

fn validate_library_import_value(
    payload: Value,
    library_kind: String,
    intended_visibility: String,
) -> Result<Value, String> {
    library_import_validation_value(&payload, &library_kind, &intended_visibility)
}

// ---------------------------------------------------------------------------
// Local-only private-library persistence (Phase C3, TP-C3-LIBSTORE-001).
//
// Imported material/section/component libraries persist in the local SQLite
// store only (local_libraries, store v11) and are never committed to the
// repository, transmitted, or exported by default (OPS-K-PRIV-1, PRD §13.5,
// §17.3). The store is private-by-default: save re-validates every document at
// the import boundary with intended_visibility = "private" and **only stores
// an accepted (PRIVATE_LOCAL_ONLY) import**. Suspected-protected (QUARANTINE)
// and otherwise-blocked (REJECTED) imports are refused — the findings are
// returned so the caller can surface them, but nothing is written. This is the
// conservative IP-boundary posture (it differs from the rule-pack store, which
// keeps a user's own in-progress draft saveable, because a library import
// carries external provenance/redistribution/protected-content risk that the
// DEL-03-07 checker exists to gate). Validation statuses are software findings
// only (OPS-K-AUTH-1).
// ---------------------------------------------------------------------------

#[derive(Debug, Serialize)]
struct LocalLibraryIndexEntry {
    project_id: String,
    library_kind: String,
    library_id: String,
    library_name: String,
    privacy_class: String,
    storage_mode: &'static str,
    created_at_unix: i64,
    updated_at_unix: i64,
}

#[derive(Debug, Serialize)]
struct LocalLibraryEnvelope {
    project_id: String,
    library_kind: String,
    library_id: String,
    storage_mode: &'static str,
    created_at_unix: i64,
    updated_at_unix: i64,
    document: Value,
    validation: Value,
    message: String,
}

#[derive(Debug, Serialize)]
struct LocalLibrarySaveResult {
    project_id: String,
    library_kind: String,
    library_id: String,
    stored: bool,
    storage_mode: &'static str,
    created_at_unix: Option<i64>,
    updated_at_unix: Option<i64>,
    document: Value,
    validation: Value,
    message: String,
}

#[derive(Debug, Serialize)]
struct LocalLibraryDeleteReceipt {
    project_id: String,
    library_kind: String,
    library_id: String,
    deleted: bool,
    message: String,
}

struct StoredLibraryRow {
    document: Value,
    created_at_unix: i64,
    updated_at_unix: i64,
}

fn library_metadata_key(library_kind: &str) -> Result<&'static str, String> {
    match library_kind {
        "material" => Ok("material_library"),
        "section" => Ok("section_library"),
        "component" => Ok("component_library"),
        "hanger" => Ok("hanger_library"),
        _ => Err(format!("unsupported library_kind: {library_kind}")),
    }
}

fn library_id_from_document(document: &Value, library_kind: &str) -> Result<String, String> {
    let key = library_metadata_key(library_kind)?;
    document
        .pointer(&format!("/{key}/library_id"))
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|id| !id.is_empty())
        .map(str::to_string)
        .ok_or_else(|| format!("library document must carry a non-empty {key}.library_id"))
}

fn library_document_string(document: &Value, library_kind: &str, field: &str) -> String {
    library_metadata_key(library_kind)
        .ok()
        .and_then(|key| document.pointer(&format!("/{key}/{field}")).cloned())
        .as_ref()
        .and_then(Value::as_str)
        .unwrap_or("TBD")
        .to_string()
}

/// The private-library persistence policy: only an accepted import
/// (PRIVATE_LOCAL_ONLY) may be written. A blocked (REJECTED) or suspected-
/// protected (QUARANTINE) import is refused — its findings are surfaced, but
/// nothing is stored. Reads the `accepted` flag from a
/// `library_import_validation_value` envelope.
fn library_import_is_storable(validation: &Value) -> bool {
    validation
        .get("accepted")
        .and_then(Value::as_bool)
        .unwrap_or(false)
}

fn upsert_library(
    connection: &Connection,
    project_id: &str,
    library_kind: &str,
    library_id: &str,
    document: &Value,
) -> Result<(i64, i64), String> {
    let document_json = serde_json::to_string(document).map_err(|error| error.to_string())?;
    let now = now_unix_seconds()?;
    connection
        .execute(
            "
            INSERT INTO local_libraries (
                project_id, library_kind, library_id, document_json, created_at_unix, updated_at_unix
            )
            VALUES (?1, ?2, ?3, ?4, ?5, ?5)
            ON CONFLICT(project_id, library_kind, library_id) DO UPDATE SET
                document_json = excluded.document_json,
                updated_at_unix = excluded.updated_at_unix
            ",
            params![project_id, library_kind, library_id, document_json, now],
        )
        .map_err(|error| error.to_string())?;
    let created_at: i64 = connection
        .query_row(
            "SELECT created_at_unix FROM local_libraries WHERE project_id = ?1 AND library_kind = ?2 AND library_id = ?3",
            params![project_id, library_kind, library_id],
            |row| row.get(0),
        )
        .map_err(|error| error.to_string())?;
    Ok((created_at, now))
}

fn load_library(
    connection: &Connection,
    project_id: &str,
    library_kind: &str,
    library_id: &str,
) -> Result<Option<StoredLibraryRow>, String> {
    connection
        .query_row(
            "
            SELECT document_json, created_at_unix, updated_at_unix
            FROM local_libraries
            WHERE project_id = ?1 AND library_kind = ?2 AND library_id = ?3
            ",
            params![project_id, library_kind, library_id],
            |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, i64>(1)?,
                    row.get::<_, i64>(2)?,
                ))
            },
        )
        .optional()
        .map_err(|error| error.to_string())?
        .map(|(document_json, created_at_unix, updated_at_unix)| {
            serde_json::from_str(&document_json)
                .map(|document| StoredLibraryRow {
                    document,
                    created_at_unix,
                    updated_at_unix,
                })
                .map_err(|error| error.to_string())
        })
        .transpose()
}

fn list_libraries(
    connection: &Connection,
    project_id: Option<&str>,
) -> Result<Vec<LocalLibraryIndexEntry>, String> {
    let mut statement = connection
        .prepare(
            "
            SELECT project_id, library_kind, library_id, document_json, created_at_unix, updated_at_unix
            FROM local_libraries
            WHERE (?1 IS NULL OR project_id = ?1)
            ORDER BY project_id, library_kind, library_id
            ",
        )
        .map_err(|error| error.to_string())?;
    let rows = statement
        .query_map(params![project_id], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, i64>(4)?,
                row.get::<_, i64>(5)?,
            ))
        })
        .map_err(|error| error.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| error.to_string())?;
    rows.into_iter()
        .map(
            |(
                project_id,
                library_kind,
                library_id,
                document_json,
                created_at_unix,
                updated_at_unix,
            )| {
                let document: Value =
                    serde_json::from_str(&document_json).map_err(|error| error.to_string())?;
                Ok(LocalLibraryIndexEntry {
                    library_name: library_document_string(&document, &library_kind, "name"),
                    privacy_class: library_document_string(
                        &document,
                        &library_kind,
                        "privacy_class",
                    ),
                    project_id,
                    library_kind,
                    library_id,
                    storage_mode: "local_sqlite",
                    created_at_unix,
                    updated_at_unix,
                })
            },
        )
        .collect()
}

fn delete_library(
    connection: &Connection,
    project_id: &str,
    library_kind: &str,
    library_id: &str,
) -> Result<bool, String> {
    let affected = connection
        .execute(
            "DELETE FROM local_libraries WHERE project_id = ?1 AND library_kind = ?2 AND library_id = ?3",
            params![project_id, library_kind, library_id],
        )
        .map_err(|error| error.to_string())?;
    Ok(affected > 0)
}

#[tauri::command]
fn save_local_library(
    app: AppHandle,
    invoke: tauri::ipc::Request<'_>,
    registry: tauri::State<'_, SolveJobRegistry>,
) -> Result<tauri::ipc::Response, CommandError> {
    let (mut args, _lease) = command_ingress::admit(&registry, &invoke, "save_local_library", ArgsDecoding::Moved)?;
    respond(save_local_library_args(&mut args, || app_store_path(&app)))
}

/// `{projectId, libraryKind, document}`.
fn save_local_library_args(
    args: &mut Args,
    store: impl FnOnce() -> Result<PathBuf, String>,
) -> Result<LocalLibrarySaveResult, CommandError> {
    let project_id = args.decode::<String>("projectId")?;
    let library_kind = args.decode::<String>("libraryKind")?;
    let document = args.value("document")?;
    save_local_library_value(store, project_id, library_kind, document).map_err(CommandError::Command)
}

fn save_local_library_value(
    store: impl FnOnce() -> Result<PathBuf, String>,
    project_id: String,
    library_kind: String,
    document: Value,
) -> Result<LocalLibrarySaveResult, String> {
    let project_id = project_id.trim().to_string();
    if project_id.is_empty() {
        return Err("project_id must be non-empty".to_string());
    }
    let library_id = library_id_from_document(&document, &library_kind)?;
    // The private library store validates at the import boundary with private
    // visibility and only persists an accepted import.
    let validation = library_import_validation_value(&document, &library_kind, "private")?;
    if !library_import_is_storable(&validation) {
        return Ok(LocalLibrarySaveResult {
            project_id,
            library_kind,
            library_id,
            stored: false,
            storage_mode: "local_sqlite",
            created_at_unix: None,
            updated_at_unix: None,
            document,
            validation,
            message: "Import not stored: validation findings block this library from the local \
                      private store. Resolve the blocking/quarantine findings and re-import; \
                      suspected protected content is never stored."
                .to_string(),
        });
    }
    let path = store()?;
    let (connection, _migration) = open_project_store(&path)?;
    let (created_at_unix, updated_at_unix) = upsert_library(
        &connection,
        &project_id,
        &library_kind,
        &library_id,
        &document,
    )?;
    Ok(LocalLibrarySaveResult {
        project_id,
        library_kind,
        library_id,
        stored: true,
        storage_mode: "local_sqlite",
        created_at_unix: Some(created_at_unix),
        updated_at_unix: Some(updated_at_unix),
        document,
        validation,
        message: "Stored library document to the local SQLite store only; private libraries are \
                  never transmitted or committed."
            .to_string(),
    })
}

#[tauri::command]
fn open_local_library(
    app: AppHandle,
    project_id: String,
    library_kind: String,
    library_id: String,
) -> Result<Option<LocalLibraryEnvelope>, String> {
    let path = app_store_path(&app)?;
    let (connection, _migration) = open_project_store(&path)?;
    let stored = load_library(
        &connection,
        project_id.trim(),
        library_kind.trim(),
        library_id.trim(),
    )?;
    let Some(row) = stored else {
        return Ok(None);
    };
    let validation =
        library_import_validation_value(&row.document, library_kind.trim(), "private")?;
    Ok(Some(LocalLibraryEnvelope {
        project_id: project_id.trim().to_string(),
        library_kind: library_kind.trim().to_string(),
        library_id: library_id.trim().to_string(),
        storage_mode: "local_sqlite",
        created_at_unix: row.created_at_unix,
        updated_at_unix: row.updated_at_unix,
        document: row.document,
        validation,
        message: "Opened library document from the local SQLite store.".to_string(),
    }))
}

#[tauri::command]
fn list_local_libraries(
    app: AppHandle,
    project_id: Option<String>,
) -> Result<Vec<LocalLibraryIndexEntry>, String> {
    let path = app_store_path(&app)?;
    let (connection, _migration) = open_project_store(&path)?;
    list_libraries(&connection, project_id.as_deref().map(str::trim))
}

#[tauri::command]
fn delete_local_library(
    app: AppHandle,
    project_id: String,
    library_kind: String,
    library_id: String,
) -> Result<LocalLibraryDeleteReceipt, String> {
    let path = app_store_path(&app)?;
    let (connection, _migration) = open_project_store(&path)?;
    let deleted = delete_library(
        &connection,
        project_id.trim(),
        library_kind.trim(),
        library_id.trim(),
    )?;
    Ok(LocalLibraryDeleteReceipt {
        project_id: project_id.trim().to_string(),
        library_kind: library_kind.trim().to_string(),
        library_id: library_id.trim().to_string(),
        deleted,
        message: if deleted {
            "Deleted library document from the local SQLite store.".to_string()
        } else {
            "No stored library document matched; nothing was deleted.".to_string()
        },
    })
}

/// One `provenance_record.library_ref` of the model, as the webview sends it
/// for the store check (W-13 §3.5; W-17 §2.7).
#[derive(Debug, Deserialize)]
struct ModelLibraryRef {
    entity_ref: String,
    library_kind: String,
    library_id: String,
    #[serde(default)]
    library_version: Option<String>,
    #[serde(default)]
    record_id: Option<String>,
}

/// A store finding on a model library reference. Both codes are warnings: the
/// model's values stand, and the recorded version is never updated here.
#[derive(Debug, Serialize, PartialEq)]
struct LibraryRefFinding {
    code: &'static str,
    severity: &'static str,
    entity_ref: String,
    library_kind: String,
    library_id: String,
    record_id: Option<String>,
    recorded_version: Option<String>,
    stored_version: Option<String>,
    message: String,
}

const MODEL_LIBRARY_REF_UNRESOLVED: &str = "MODEL-LIBRARY-REF-UNRESOLVED";
const MODEL_LIBRARY_REF_VERSION_CHANGED: &str = "MODEL-LIBRARY-REF-VERSION-CHANGED";

fn library_records_key(library_kind: &str) -> Option<(&'static str, &'static str)> {
    match library_kind {
        "material" => Some(("material_records", "material_id")),
        "section" => Some(("section_records", "section_id")),
        "component" => Some(("component_records", "component_id")),
        "hanger" => Some(("hanger_records", "hanger_id")),
        _ => None,
    }
}

/// `library_version` of a stored record: "sha256:" + the SHA-256 of its RFC
/// 8785 (JCS) form, the digest the desktop records at insert and apply.
fn library_record_version(record: &Value) -> String {
    format!(
        "sha256:{}",
        open_pipe_stress_operation_applier::sha256_hex(&open_pipe_stress_operation_applier::canonical_json(record))
    )
}

/// Compare each reference with the project's local store: a missing library or
/// record is `MODEL-LIBRARY-REF-UNRESOLVED`; a stored record whose digest
/// differs from the recorded `library_version` is
/// `MODEL-LIBRARY-REF-VERSION-CHANGED`.
fn library_ref_findings(
    connection: &Connection,
    project_id: &str,
    references: &[ModelLibraryRef],
) -> Result<Vec<LibraryRefFinding>, String> {
    let mut findings = Vec::new();
    for reference in references {
        let finding = |code, stored_version: Option<String>, message: String| LibraryRefFinding {
            code,
            severity: "warning",
            entity_ref: reference.entity_ref.clone(),
            library_kind: reference.library_kind.clone(),
            library_id: reference.library_id.clone(),
            record_id: reference.record_id.clone(),
            recorded_version: reference.library_version.clone(),
            stored_version,
            message,
        };
        let stored = match library_records_key(&reference.library_kind) {
            Some(_) => load_library(connection, project_id, &reference.library_kind, &reference.library_id)?,
            None => None,
        };
        let Some(row) = stored else {
            findings.push(finding(
                MODEL_LIBRARY_REF_UNRESOLVED,
                None,
                format!(
                    "{} names {} library `{}`, which the local store does not hold for project `{project_id}`. \
                     The model's values stand. A project saved under a new id reads its references as unresolved \
                     until the library is imported into that project.",
                    reference.entity_ref, reference.library_kind, reference.library_id
                ),
            ));
            continue;
        };
        let (records_key, id_key) = library_records_key(&reference.library_kind).expect("checked above");
        let record = reference.record_id.as_deref().and_then(|record_id| {
            row.document
                .get(records_key)
                .and_then(Value::as_array)
                .and_then(|records| records.iter().find(|record| record.get(id_key).and_then(Value::as_str) == Some(record_id)))
        });
        let Some(record) = record else {
            findings.push(finding(
                MODEL_LIBRARY_REF_UNRESOLVED,
                None,
                format!(
                    "{} names record `{}` of {} library `{}`, which the stored library does not hold. The model's values stand.",
                    reference.entity_ref,
                    reference.record_id.as_deref().unwrap_or("(none recorded)"),
                    reference.library_kind,
                    reference.library_id
                ),
            ));
            continue;
        };
        let stored_version = library_record_version(record);
        if let Some(recorded) = &reference.library_version {
            if recorded != &stored_version {
                findings.push(finding(
                    MODEL_LIBRARY_REF_VERSION_CHANGED,
                    Some(stored_version),
                    format!(
                        "The stored record `{}` of {} library `{}` has changed since {} recorded it. The model's values and \
                         recorded version stand; re-apply from the library to adopt the stored record.",
                        reference.record_id.as_deref().unwrap_or(""),
                        reference.library_kind,
                        reference.library_id,
                        reference.entity_ref
                    ),
                ));
            }
        }
    }
    Ok(findings)
}

/// Checks the model's library references against the local store at project
/// open and load and on library-list refresh (W-13 §3.5). Read-only.
#[tauri::command]
fn check_model_library_refs(
    app: AppHandle,
    project_id: String,
    references: Vec<ModelLibraryRef>,
) -> Result<Vec<LibraryRefFinding>, String> {
    let path = app_store_path(&app)?;
    let (connection, _migration) = open_project_store(&path)?;
    library_ref_findings(&connection, project_id.trim(), &references)
}

const NATIVE_MENU_DOM_EVENT: &str = "openpipestress-native-menu-command";

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct NativeShellState {
    project_name: Option<String>,
    #[serde(default)]
    model_edited: bool,
    #[serde(default)]
    project_busy: bool,
    stage: String,
    view: String,
    theme: String,
    density: String,
    results_stage_enabled: bool,
    review_stage_enabled: bool,
    inspector_open: bool,
    can_undo: bool,
    can_redo: bool,
    can_run: bool,
    can_cancel: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct NativeShellStateReceipt {
    window_title: String,
    stage: String,
    view: String,
    theme: String,
    density: String,
}

#[derive(Debug, PartialEq, Eq)]
enum NativeMenuMutation {
    Checked(&'static str, bool),
    Enabled(&'static str, bool),
}

fn normalized_project_name(project_name: Option<&str>) -> Option<String> {
    project_name
        .map(|name| name.split_whitespace().collect::<Vec<_>>().join(" "))
        .filter(|name| !name.is_empty())
}

fn native_window_title(project_name: Option<&str>, model_edited: bool) -> String {
    normalized_project_name(project_name)
        .map(|name| {
            if model_edited {
                format!("{name} · Edited — SWBPIPE")
            } else {
                format!("{name} — SWBPIPE")
            }
        })
        .unwrap_or_else(|| "SWBPIPE".to_string())
}

fn validate_native_shell_state(state: &NativeShellState) -> Result<(), String> {
    fn one_of(value: &str, field: &str, allowed: &[&str]) -> Result<(), String> {
        if allowed.contains(&value) {
            Ok(())
        } else {
            Err(format!(
                "NATIVE-SHELL-STATE-INVALID: unsupported {field} value {value:?}"
            ))
        }
    }

    one_of(
        &state.stage,
        "stage",
        &["model", "loads", "results", "review"],
    )?;
    one_of(&state.view, "view", &["table", "model", "both"])?;
    one_of(&state.theme, "theme", &["light", "dark", "system"])?;
    one_of(&state.density, "density", &["comfortable", "compact"])?;
    if state.stage == "review" && state.view != "table" {
        return Err("NATIVE-SHELL-STATE-INVALID: Review stage must use Table view".to_string());
    }
    Ok(())
}

fn native_menu_mutations(state: &NativeShellState) -> Vec<NativeMenuMutation> {
    let mut mutations = Vec::new();
    for (id, value) in [
        ("view.stage.model", state.stage == "model"),
        ("view.stage.loads", state.stage == "loads"),
        ("view.stage.results", state.stage == "results"),
        ("view.stage.review", state.stage == "review"),
        ("view.view.table", state.view == "table"),
        ("view.view.model", state.view == "model"),
        ("view.view.both", state.view == "both"),
        ("view.inspector", state.inspector_open),
        ("view.theme.light", state.theme == "light"),
        ("view.theme.dark", state.theme == "dark"),
        ("view.theme.system", state.theme == "system"),
        ("view.density.comfortable", state.density == "comfortable"),
        ("view.density.compact", state.density == "compact"),
    ] {
        mutations.push(NativeMenuMutation::Checked(id, value));
    }
    for (id, value) in [
        ("file.new-local", !state.project_busy),
        ("file.new-blank", !state.project_busy),
        ("file.open-model", !state.project_busy),
        ("file.open-local", !state.project_busy),
        ("file.save-local", !state.project_busy),
        ("file.save-model-document", !state.project_busy),
        ("file.list-local", !state.project_busy),
        ("view.view.table", true),
        ("view.view.model", state.stage != "review"),
        ("view.view.both", state.stage != "review"),
        ("view.inspector", state.view == "both"),
        ("view.stage.results", state.results_stage_enabled),
        ("view.stage.review", state.review_stage_enabled),
        ("edit.undo", state.can_undo),
        ("edit.redo", state.can_redo),
        ("analyze.run", state.can_run),
        ("analyze.cancel", state.can_cancel),
    ] {
        mutations.push(NativeMenuMutation::Enabled(id, value));
    }
    mutations
}

fn find_native_menu_item<R: tauri::Runtime>(
    items: Vec<MenuItemKind<R>>,
    id: &str,
) -> Option<MenuItemKind<R>> {
    for item in items {
        if item.id().0 == id {
            return Some(item);
        }
        if let Some(submenu) = item.as_submenu() {
            if let Some(found) = find_native_menu_item(submenu.items().unwrap_or_default(), id) {
                return Some(found);
            }
        }
    }
    None
}

fn set_native_menu_item_enabled<R: tauri::Runtime>(
    menu: &tauri::menu::Menu<R>,
    id: &str,
    enabled: bool,
) -> Result<(), String> {
    let item = find_native_menu_item(menu.items().map_err(|error| error.to_string())?, id)
        .ok_or_else(|| format!("NATIVE-MENU-ITEM-MISSING: {id}"))?;
    match item {
        MenuItemKind::MenuItem(item) => item.set_enabled(enabled),
        MenuItemKind::Check(item) => item.set_enabled(enabled),
        _ => return Err(format!("NATIVE-MENU-ITEM-KIND-INVALID: {id}")),
    }
    .map_err(|error| error.to_string())
}

fn set_native_menu_item_checked<R: tauri::Runtime>(
    menu: &tauri::menu::Menu<R>,
    id: &str,
    checked: bool,
) -> Result<(), String> {
    let item = find_native_menu_item(menu.items().map_err(|error| error.to_string())?, id)
        .ok_or_else(|| format!("NATIVE-MENU-ITEM-MISSING: {id}"))?;
    item.as_check_menuitem()
        .ok_or_else(|| format!("NATIVE-MENU-ITEM-KIND-INVALID: {id}"))?
        .set_checked(checked)
        .map_err(|error| error.to_string())
}

#[tauri::command]
fn sync_native_shell_state(
    app: AppHandle,
    state: NativeShellState,
) -> Result<NativeShellStateReceipt, String> {
    validate_native_shell_state(&state)?;
    let menu = app
        .menu()
        .ok_or_else(|| "NATIVE-MENU-UNAVAILABLE: application menu is not installed".to_string())?;
    for mutation in native_menu_mutations(&state) {
        match mutation {
            NativeMenuMutation::Checked(id, checked) => {
                set_native_menu_item_checked(&menu, id, checked)?
            }
            NativeMenuMutation::Enabled(id, enabled) => {
                set_native_menu_item_enabled(&menu, id, enabled)?
            }
        }
    }
    let title = native_window_title(state.project_name.as_deref(), state.model_edited);
    app.get_webview_window("main")
        .ok_or_else(|| "NATIVE-WINDOW-UNAVAILABLE: main webview is not installed".to_string())?
        .set_title(&title)
        .map_err(|error| error.to_string())?;
    Ok(NativeShellStateReceipt {
        window_title: title,
        stage: state.stage,
        view: state.view,
        theme: state.theme,
        density: state.density,
    })
}

fn native_menu_dispatch_script(command_id: &str) -> String {
    let command_json = serde_json::to_string(command_id)
        .expect("serializing a native menu command id to JSON cannot fail");
    format!(
        "window.dispatchEvent(new CustomEvent('{NATIVE_MENU_DOM_EVENT}', {{ detail: {command_json} }}));"
    )
}

fn dispatch_native_menu_command<R: tauri::Runtime>(app: &tauri::AppHandle<R>, command_id: &str) {
    let Some(webview) = app.get_webview_window("main") else {
        eprintln!("native menu command dropped: main webview is unavailable");
        return;
    };
    if let Err(error) = webview.eval(&native_menu_dispatch_script(command_id)) {
        eprintln!("native menu command dispatch failed: {error}");
    }
}

fn restore_native_check_item_after_activation<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
    command_id: &str,
) {
    let Some(menu) = app.menu() else {
        return;
    };
    let Some(item) = find_native_menu_item(menu.items().unwrap_or_default(), command_id) else {
        return;
    };
    let Some(item) = item.as_check_menuitem() else {
        return;
    };
    // muda toggles check items before delivering the menu event. React owns
    // the actual shell state, so restore the last synchronized value first;
    // the resulting React state change will then synchronize the next value.
    if let Ok(checked) = item.is_checked() {
        if let Err(error) = item.set_checked(!checked) {
            eprintln!("native menu check-state restore failed: {error}");
        }
    }
}

// Native macOS menu bar (TP-R3UX-CADSHELL). Each custom item carries the same
// command id the in-DOM menu uses; on click the backend injects one typed DOM
// event into the main webview, where the React command sink dispatches it.
fn build_app_menu<R: tauri::Runtime>(
    handle: &tauri::AppHandle<R>,
) -> tauri::Result<tauri::menu::Menu<R>> {
    let app_menu = SubmenuBuilder::new(handle, "SWBPIPE")
        .about(None)
        .separator()
        .quit()
        .build()?;

    let file = SubmenuBuilder::new(handle, "File")
        .text("file.new-local", "New Local Project")
        .text("file.new-blank", "New Blank Project")
        .separator()
        .text("file.open-model", "Open Model Document…")
        .text("file.open-local", "Open Local Project…")
        .text("file.list-local", "List Local Projects")
        .separator()
        .text("file.save-local", "Save Local Project")
        .text("file.save-model-document", "Save Model Document…")
        .text("file.save-report-package", "Save Report Package…")
        .build()?;

    let edit = SubmenuBuilder::new(handle, "Edit")
        .text("edit.undo", "Undo Model Edit")
        .text("edit.redo", "Redo Model Edit")
        .undo()
        .redo()
        .separator()
        .cut()
        .copy()
        .paste()
        .select_all()
        .build()?;

    let view_table = CheckMenuItemBuilder::with_id("view.view.table", "Table View")
        .checked(false)
        .accelerator("CmdOrCtrl+1")
        .build(handle)?;
    let view_model = CheckMenuItemBuilder::with_id("view.view.model", "Model View")
        .checked(false)
        .accelerator("CmdOrCtrl+2")
        .build(handle)?;
    let view_both = CheckMenuItemBuilder::with_id("view.view.both", "Both View")
        .checked(true)
        .accelerator("CmdOrCtrl+3")
        .build(handle)?;
    let stage_model = CheckMenuItemBuilder::with_id("view.stage.model", "Model Stage")
        .checked(true)
        .build(handle)?;
    let stage_loads = CheckMenuItemBuilder::with_id("view.stage.loads", "Loads Stage")
        .checked(false)
        .build(handle)?;
    let stage_results = CheckMenuItemBuilder::with_id("view.stage.results", "Results Stage")
        .enabled(false)
        .checked(false)
        .build(handle)?;
    let stage_review = CheckMenuItemBuilder::with_id("view.stage.review", "Review Stage")
        .enabled(false)
        .checked(false)
        .build(handle)?;
    let inspector = CheckMenuItemBuilder::with_id("view.inspector", "Inspector")
        .checked(false)
        .accelerator("CmdOrCtrl+I")
        .build(handle)?;
    let theme_light = CheckMenuItemBuilder::with_id("view.theme.light", "Light")
        .checked(false)
        .build(handle)?;
    let theme_dark = CheckMenuItemBuilder::with_id("view.theme.dark", "Dark")
        .checked(false)
        .build(handle)?;
    let theme_system = CheckMenuItemBuilder::with_id("view.theme.system", "System")
        .checked(true)
        .build(handle)?;
    let density_comfortable =
        CheckMenuItemBuilder::with_id("view.density.comfortable", "Comfortable Density")
            .checked(true)
            .build(handle)?;
    let density_compact = CheckMenuItemBuilder::with_id("view.density.compact", "Compact Density")
        .checked(false)
        .build(handle)?;

    let view = SubmenuBuilder::new(handle, "View")
        .item(&view_table)
        .item(&view_model)
        .item(&view_both)
        .separator()
        .item(&stage_model)
        .item(&stage_loads)
        .item(&stage_results)
        .item(&stage_review)
        .separator()
        .text("view.tree", "Model Tree")
        .item(&inspector)
        .separator()
        .text("view.section.operations", "Operation Apply")
        .text("view.section.loads", "Load Cases")
        .text("view.section.libraries", "Libraries")
        .text("view.section.rule-packs", "Rule Packs")
        .text("view.section.solve", "Solve")
        .text("view.section.results", "Results")
        .text("view.section.report", "Report")
        .text("view.section.project", "Project")
        .text("view.section.exports", "Exports")
        .text("view.section.evidence", "Audit & Boundaries")
        .separator()
        .text("view.issues", "Issues")
        .text("view.audit", "Audit & Boundaries Drawer")
        .separator()
        .text("view.close-panels", "Close Panel (show viewport)")
        .separator()
        .item(&theme_light)
        .item(&theme_dark)
        .item(&theme_system)
        .separator()
        .item(&density_comfortable)
        .item(&density_compact)
        .build()?;

    let insert = SubmenuBuilder::new(handle, "Insert")
        .text("insert.node", "Node")
        .text("insert.pipe", "Pipe Run")
        .text("insert.support", "Support")
        .text("insert.component", "Component")
        .separator()
        .text("insert.load", "Load Case")
        .build()?;

    let analyze = SubmenuBuilder::new(handle, "Analyze")
        .text("analyze.run", "Run Mechanics Preview")
        .text("analyze.cancel", "Cancel Run")
        .separator()
        .text("analyze.rule-checks", "Rule Checks")
        .build()?;

    MenuBuilder::new(handle)
        .item(&app_menu)
        .item(&file)
        .item(&edit)
        .item(&view)
        .item(&insert)
        .item(&analyze)
        .build()
}

fn packaged_saved_edited_load_intent() -> Value {
    serde_json::from_str(r#"{
  "operation_id": "op:workflow-human-load",
  "operation_kind": "modify",
  "operation_status": "proposed",
  "author_type": "user",
  "target": {
    "object_type": "Load",
    "ref": "load:WF"
  },
  "change": {
    "change_id": "change:workflow-human-load",
    "change_kind": "update_load",
    "field_label": "Tip force",
    "field_path": "primitive_loads.0.magnitude.value",
    "before": "350",
    "after": "500",
    "unit": "N",
    "dimension": "force",
    "source_note": "invented_synthetic_workflow_input"
  },
  "validation": {
    "schema_validation": "not_run",
    "constraint_validation": "not_run",
    "unit_validation": "not_run",
    "diff_preview_status": "not_generated",
    "application_status": "not_applied"
  },
  "audit_boundary": {
    "mutation_route": "structured_operations_only",
    "direct_model_mutation_allowed": false,
    "requires_user_acceptance": true,
    "mutates_accepted_model_state": false
  },
  "professional_boundary": {
    "human_review_required": true,
    "software_makes_compliance_claim": false,
    "software_makes_certification_claim": false,
    "software_makes_sealing_claim": false,
    "software_makes_approval_claim": false,
    "software_makes_authentication_claim": false
  },
  "rationale": "Synthetic native workflow lifecycle witness"
}"#).expect("frozen invented human intent parses")
}

fn packaged_saved_edited_load_self_test_at(store_path: &Path) -> Result<Value, String> {
    // Embedded frozen synthetic model: the executable never reads repository
    // fixtures. The document is the current version, saved as authored; its
    // version is the shared constant, never a literal.
    let mut model: Value = serde_json::from_str(r#"{
  "document_kind": "openpipestress.product_preview.model",
  "data_boundary": {
    "public_examples_policy": "invented_or_cleared_data_only",
    "protected_source_policy": "no_bundled_protected_owner_or_standards_data",
    "private_data_policy": "no_private_project_data",
    "professional_boundary": "technical_preview_requires_human_engineering_review"
  },
  "project": {
    "id": "project:workflow-cantilever",
    "name": "Invented workflow cantilever",
    "description": "Synthetic Euler-Bernoulli lifecycle witness; not an engineering acceptance.",
    "units": {
      "length": "m",
      "force": "N",
      "angle": "rad",
      "pressure": "Pa",
      "temperature": "degC",
      "stress": "MPa"
    }
  },
  "materials": [
    {
      "id": "material:invented-carbon-steel",
      "label": "Invented carbon-steel-like material",
      "elastic_modulus": {
        "value": 200000000000,
        "unit": "Pa"
      },
      "shear_modulus": {
        "value": 77000000000,
        "unit": "Pa"
      },
      "thermal_expansion_coefficient": {
        "value": 1.2e-05,
        "unit": "1/degC"
      },
      "provenance": "invented_example_no_material_standard"
    }
  ],
  "analysis_status": {
    "mechanics": "ready_for_preview_diagnostics",
    "rule_check": "not_performed_user_rule_inputs_missing",
    "professional_acceptance": "not_provided"
  },
  "nodes": [
    {
      "id": "node:WF-ROOT",
      "label": "Root",
      "position": {
        "x": 0,
        "y": 2.4,
        "z": 0
      },
      "provenance": "invented_synthetic_workflow_input"
    },
    {
      "id": "node:WF-TIP",
      "label": "Tip",
      "position": {
        "x": 3.2,
        "y": 2.4,
        "z": 0
      },
      "provenance": "invented_synthetic_workflow_input"
    }
  ],
  "pipe_segments": [
    {
      "id": "pipe:WF",
      "label": "Cantilever",
      "from": "node:WF-ROOT",
      "to": "node:WF-TIP",
      "section": {
        "outside_diameter": {
          "value": 0.168,
          "unit": "m"
        },
        "wall_thickness": {
          "value": 0.007,
          "unit": "m"
        }
      },
      "material": "material:invented-carbon-steel",
      "y_reference": {
        "x": 0,
        "y": 0,
        "z": 1
      },
      "provenance": "invented_synthetic_workflow_input"
    }
  ],
  "supports": [
    {
      "id": "support:WF-ROOT",
      "label": "Root anchor",
      "node": "node:WF-ROOT",
      "family": "anchor",
      "restraints": [
        "UX",
        "UY",
        "UZ",
        "RX",
        "RY",
        "RZ"
      ],
      "provenance": "invented_synthetic_workflow_input"
    }
  ],
  "components": [],
  "load_cases": [
    {
      "id": "load:WF",
      "label": "Synthetic tip force",
      "kind": "primitive_user_load",
      "status": "preview_only",
      "primitive_loads": [
        {
          "id": "load:WF-FY",
          "category": "occasional",
          "target": {
            "type": "node",
            "node": "node:WF-TIP"
          },
          "direction": "global_y",
          "magnitude": {
            "value": 350,
            "unit": "N"
          },
          "dimension": "force",
          "provenance": "invented_synthetic_workflow_input"
        }
      ],
      "provenance": "invented_synthetic_workflow_input"
    }
  ],
  "combinations": [],
  "diagnostics": []
}"#)
        .map_err(|error| format!("PACKAGED-SMOKE-FIXTURE-INVALID: {error}"))?;
    model["schema_version"] = json!(open_pipe_stress_product_physics::MODEL_SCHEMA_VERSION);
    let hash = |payload: &Value| format!("sha256:{}", open_pipe_stress_operation_applier::sha256_hex(
        &open_pipe_stress_operation_applier::canonical_json(payload)));
    let baseline_hash = hash(&model);
    if baseline_hash != "sha256:6ab326322a5e10568a71048a22cc7cb633b4422a0ade86e021debc01f0f6ddcf" {
        return Err(format!("PACKAGED-SMOKE-FROZEN-BASELINE-HASH-MISMATCH: {baseline_hash}"));
    }
    let baseline = solve_preview_mechanics(model.clone())?;
    let human_intent = packaged_saved_edited_load_intent();
    let human_intent_hash = hash(&human_intent);
    if human_intent_hash != "sha256:a4d7c3d16898d85ba83596dc8e85347aa966146fde78b79b9097cdb14a0c8401" {
        return Err("PACKAGED-SMOKE-FROZEN-HUMAN-INTENT-HASH-MISMATCH".into());
    }
    let human_review = validate_model_operation_value(model.clone(), human_intent.clone(), None)?;
    if human_review["validation"]["schema_validation"] != json!("passed") {
        return Err("PACKAGED-SMOKE-HUMAN-REVIEW-FAILED".into());
    }
    let applied = apply_model_operation_value(model.clone(), human_intent, None)?;
    if applied["validation"]["application_status"] != json!("applied_to_session_model")
        || applied["acceptance"]["acceptance_is_professional_approval"] != json!(false) {
        return Err("PACKAGED-SMOKE-OPERATION-NOT-APPLIED".into());
    }
    let edited_model = applied["applied_model"].clone();
    let edited_hash = hash(&edited_model);
    if edited_hash != "sha256:042bf3a9b2a2b52aba14bf759bb34ffd0f4c125422cbb1aa9520d28e0ab69012" {
        return Err(format!("PACKAGED-SMOKE-FROZEN-EDIT-HASH-MISMATCH: {edited_hash}"));
    }
    let agent_intent: Value = serde_json::from_str(r#"{
  "operation_id": "op:workflow-agent-load",
  "operation_kind": "modify",
  "operation_status": "proposed",
  "author_type": "agent",
  "target": {
    "object_type": "Load",
    "ref": "load:WF"
  },
  "change": {
    "change_id": "change:workflow-agent-load",
    "change_kind": "update_load",
    "field_label": "Tip force",
    "field_path": "primitive_loads.0.magnitude.value",
    "before": "350",
    "after": "500",
    "unit": "N",
    "dimension": "force",
    "source_note": "invented_synthetic_workflow_input"
  },
  "validation": {
    "schema_validation": "not_run",
    "constraint_validation": "not_run",
    "unit_validation": "not_run",
    "diff_preview_status": "not_generated",
    "application_status": "not_applied"
  },
  "audit_boundary": {
    "mutation_route": "structured_operations_only",
    "direct_model_mutation_allowed": false,
    "requires_user_acceptance": true,
    "mutates_accepted_model_state": false
  },
  "professional_boundary": {
    "human_review_required": true,
    "software_makes_compliance_claim": false,
    "software_makes_certification_claim": false,
    "software_makes_sealing_claim": false,
    "software_makes_approval_claim": false,
    "software_makes_authentication_claim": false
  },
  "rationale": "Synthetic native workflow lifecycle witness"
}"#).expect("frozen invented agent intent parses");
    let agent_intent_hash = hash(&agent_intent);
    if agent_intent_hash != "sha256:89e98532e3854148803b53dec91c43db0e628305b86d379bec743612557c0336" {
        return Err("PACKAGED-SMOKE-FROZEN-AGENT-INTENT-HASH-MISMATCH".into());
    }
    let agent_review = validate_model_operation_value(model.clone(), agent_intent.clone(), None)?;
    let agent_applied = apply_model_operation_value(model.clone(), agent_intent, None)?;
    if agent_review["validation"]["schema_validation"] != json!("passed")
        || hash(&agent_applied["applied_model"]) != edited_hash {
        return Err("PACKAGED-SMOKE-AGENT-HUMAN-EQUIVALENCE-FAILED".into());
    }
    // Native evidence checks exact checkpoint payloads; the UI's history
    // handlers and synchronous busy gate are covered separately in desktop tests.
    let undo_model = model.clone();
    let redo_model = edited_model.clone();
    if hash(&undo_model) != baseline_hash || hash(&redo_model) != edited_hash {
        return Err("PACKAGED-SMOKE-CHECKPOINT-HASH-MISMATCH".into());
    }
    let attachments = json!({
        "pipe_from": edited_model["pipe_segments"][0]["from"],
        "pipe_to": edited_model["pipe_segments"][0]["to"],
        "material": edited_model["pipe_segments"][0]["material"],
        "support_node": edited_model["supports"][0]["node"],
        "load_node": edited_model["load_cases"][0]["primitive_loads"][0]["target"]["node"]
    });
    if attachments != json!({"pipe_from":"node:WF-ROOT","pipe_to":"node:WF-TIP","material":"material:invented-carbon-steel","support_node":"node:WF-ROOT","load_node":"node:WF-TIP"}) {
        return Err("PACKAGED-SMOKE-ATTACHMENTS-CHANGED".into());
    }
    let edited_result = solve_preview_mechanics(edited_model.clone())?;
    let received_model_hash = json!({"algorithm":"sha256","canonicalization":"rfc8785_jcs","payload_scope":"model_payload","payload_ref":"project:workflow-cantilever","value":edited_hash,"hash_status":"computed_local_preview"});
    let store_migration = {
        let (mut connection, migration) = open_project_store(store_path)?;
        let document_status = check_model_document_for_persist(&edited_model)?;
        upsert_project(&mut connection, "project:workflow-cantilever", "Invented workflow cantilever",
            &edited_model, &json!([]), &Value::Null, &Value::Null, &edited_result,
            &Value::Null, &received_model_hash, &Value::Null)?;
        json!({"store_schema_version":migration.store_schema_version,"document_status":document_status.status,"document_version":document_status.source_schema_version})
    };
    let (connection, reopen_migration) = open_project_store(store_path)?;
    let restored = load_project(&connection, Some("project:workflow-cantilever"))?
        .ok_or_else(|| "PACKAGED-SMOKE-RESTORED-PROJECT-MISSING".to_string())?;
    drop(connection);
    if restored.model_hash["value"] != json!(hash(&restored.model)) || restored.mechanics_result != edited_result
        || !restored.analysis_run.is_null() || !restored.project_envelope_hash.is_null() {
        return Err(format!(
            "PACKAGED-SMOKE-RESTORED-EVIDENCE-MISMATCH: model_claim={}; actual={}; mechanics_equal={}; analysis_null={}; envelope_null={}",
            restored.model_hash["value"],
            hash(&restored.model),
            restored.mechanics_result == edited_result,
            restored.analysis_run.is_null(),
            restored.project_envelope_hash.is_null()
        ));
    }
    // Pure unchanged save preserves the existing fields, including deliberately
    // absent analysis/input-manifest/envelope evidence. No native run builder
    // exists here, so it would be false to invent that binding.
    {
        let (mut connection, _) = open_project_store(store_path)?;
        upsert_project(&mut connection, "project:workflow-cantilever", "Invented workflow cantilever",
            &restored.model, &restored.editor_intents, &restored.proposal, &restored.selected_review_target,
            &restored.mechanics_result, &restored.analysis_run, &restored.model_hash,
            &restored.project_envelope_hash)?;
    }
    let (connection, _) = open_project_store(store_path)?;
    let resaved = load_project(&connection, Some("project:workflow-cantilever"))?
        .ok_or_else(|| "PACKAGED-SMOKE-RESAVED-PROJECT-MISSING".to_string())?;
    drop(connection);
    if resaved.mechanics_result != edited_result || resaved.model_hash != restored.model_hash
        || !resaved.analysis_run.is_null() || !resaved.project_envelope_hash.is_null() {
        return Err("PACKAGED-SMOKE-UNCHANGED-SAVE-EVIDENCE-MISMATCH".into());
    }
    let solved = solve_preview_mechanics(restored.model)?;
    let value = |result: &Value, id: &str, unit: &str| -> Result<f64, String> {
        result["results"].as_array().and_then(|rows| rows.iter().find(|row| row["id"] == json!(id) && row["unit"] == json!(unit)))
            .and_then(|row| row["value"].as_f64()).filter(|number| number.is_finite())
            .ok_or_else(|| format!("PACKAGED-SMOKE-RESULT-MISSING: {id} {unit}"))
    };
    let tolerance = 0.5e-6 + 1.0e-10;
    let mut witnesses = Vec::new();
    for (result, force, uy, rz) in [(&baseline, 350.0, 1.663071716046058, 0.0007795648668965897),
        (&edited_result, 500.0, 2.375816737208654, 0.0011136640955665567),
        (&solved, 500.0, 2.375816737208654, 0.0011136640955665567)] {
        if result["status"]["mechanics"] != json!("MECHANICS_SOLVED") || result["model_ref"] != json!("project:workflow-cantilever") {
            return Err(format!("PACKAGED-SMOKE-SOLVE-FAILED: force={force}; status={}; model_ref={}; diagnostics={}", result["status"], result["model_ref"], result["diagnostics"]));
        }
        let quantities = [
            ("result:disp:node-WF-TIP:uy", "mm", uy),
            ("result:disp:node-WF-TIP:rz", "rad", rz),
            // T0R preview-physics-1: signed support-on-pipe components replace the retired resultant.
            ("result:support-action:7:load:WF:15:support:WF-ROOT:Fy", "N", -force),
            ("result:support-action:7:load:WF:15:support:WF-ROOT:Mz", "N*m", -force * 3.2),
            ("result:force:pipe-WF:shear-z", "N", force),
            ("result:moment:pipe-WF:bending-y", "N*m", -force * 3.2)
        ];
        let mut published = Vec::new();
        for (id, unit, expected) in quantities {
            let actual = value(result, id, unit)?;
            if (actual - expected).abs() > tolerance {
                return Err(format!("PACKAGED-SMOKE-ANALYTICAL-MISMATCH: {id} {actual} expected {expected} {unit}"));
            }
            published.push(json!({"result_id":id,"unit":unit,"actual":actual,"expected":expected}));
        }
        witnesses.push(json!({"force_N":force,"published":published}));
    }
    if solved["results"] != edited_result["results"] {
        return Err("PACKAGED-SMOKE-RESOLVE-RESULTS-MISMATCH".into());
    }
    Ok(json!({
        "status":"PASS", "route":"packaged_binary_saved_edited_load_self_test", "fixture_class":"invented_public",
        "project_id":"project:workflow-cantilever",
        "operation":{"target":"load:WF/load:WF-FY","before":350,"after":500,"unit":"N","application_status":"applied_to_session_model","professional_approval":false},
        "hashes":{"baseline":baseline_hash,"edit":edited_hash,"undo_checkpoint":hash(&undo_model),"redo_checkpoint":hash(&redo_model),"equivalent_agent":hash(&agent_applied["applied_model"]),"human_intent":human_intent_hash,"agent_intent":agent_intent_hash},
        "attachments":attachments,
        "persistence":{"storage":"isolated_file_backed_sqlite","writer_connection_closed_before_restore":true,"restored_value":500,"restored_unit":"N","store_migration":store_migration,"reopen_store_schema_version":reopen_migration.store_schema_version,"historical_result_preserved":true,"unchanged_save_preserves_evidence":true,"analysis_run":"missing_no_native_builder","input_manifest":"missing_not_invented","project_envelope_hash":"missing_preserved"},
        "solve":{"mechanics":solved["status"]["mechanics"],"model_ref":solved["model_ref"],"result_rows":solved["results"].as_array().map_or(0, Vec::len),"edited_result_differs_from_baseline":true,"fresh_resolve_matches_saved_result_rows":true,"analytical_witnesses":witnesses,"publication_tolerance_declared_units":tolerance,"reaction_transform":"y_reference global Z: local Z=-global Y and local Y=global Z; end-i fz=+F and my=-FL correspond to global root Fy=-F and Mz=-FL; public support rows are signed support-on-pipe global components"},
        "boundary":{"network":false,"telemetry":false,"private_data":false,"protected_content":false,"repository_write":false}
    }))
}

/// Run the native packaged-binary persistence proof in a unique, disposable
/// file-backed SQLite store and remove that store before returning.
/// Its solves run outside the memory pool: this is a separate self-test process mode, not an app job.
pub fn run_packaged_saved_edited_load_self_test() -> Result<Value, String> {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|error| error.to_string())?
        .as_nanos();
    let smoke_dir = std::env::temp_dir().join(format!(
        "openpipestress-packaged-edited-load-smoke-{}-{nonce}",
        std::process::id()
    ));
    fs::create_dir(&smoke_dir).map_err(|error| format!("PACKAGED-SMOKE-TEMP-CREATE: {error}"))?;
    let store_path = smoke_dir.join(PROJECT_STORE_FILE);
    let result = packaged_saved_edited_load_self_test_at(&store_path);
    fs::remove_dir_all(&smoke_dir)
        .map_err(|error| format!("PACKAGED-SMOKE-TEMP-CLEANUP: {error}"))?;
    result
}

#[tauri::command]
async fn save_local_result_json(
    app: AppHandle,
    admission: tauri::State<'_, native_result_download::SaveAdmission>,
    invoke: tauri::ipc::Request<'_>,
    registry: tauri::State<'_, SolveJobRegistry>,
) -> Result<tauri::ipc::Response, CommandError<native_result_download::SaveError>> {
    let (mut args, lease) = command_ingress::admit(&registry, &invoke, "save_local_result_json", ArgsDecoding::Moved)?;
    let request = save_local_result_json_args(&mut args)?;
    drop(args);
    // The worker owns admission and the ingress lease until completion/drop,
    // independently of the awaiting command or frontend component lifetime.
    let permit = admission.admit().map_err(CommandError::Command)?;
    let outcome = tauri::async_runtime::spawn_blocking(move || {
        let (_permit, _lease) = (permit, lease);
        native_result_download::save_request(request, || app.path().download_dir())
    })
    .await
    .unwrap_or_else(|_| Err(native_result_download::SaveError::worker()));
    respond(outcome.map_err(CommandError::Command))
}

/// `{request}` (snake_case fields, see `native_result_download::SaveRequest`).
fn save_local_result_json_args(args: &mut Args) -> Result<native_result_download::SaveRequest, solve_jobs::SolveJobError> {
    args.decode("request")
}

/// The app's commands; the `native-smoke` build appends the harness's own.
macro_rules! app_invoke_handler {
    ($($extra:path),* $(,)?) => {
        tauri::generate_handler![
            load_preview_model,
            load_design_knowledge,
            open_model_document_file,
            save_model_document_file,
            start_preview_mechanics_job,
            poll_preview_mechanics_job,
            take_preview_mechanics_job_result,
            dispose_preview_mechanics_result,
            dispose_all_preview_mechanics_results,
            cancel_preview_mechanics_job,
            get_memory_pool_status,
            configure_memory_pool,
            sample_agent_proposal,
            validate_model_operation,
            apply_model_operation,
            validate_model_operation_batch,
            generate_self_weight_operation_plan,
            apply_model_operation_batch,
            get_unit_catalog,
            convert_display_quantities,
            get_local_storage_capability,
            create_local_project,
            open_local_project,
            list_local_projects,
            find_local_project,
            save_local_project,
            model_states::save_model_state,
            model_states::list_model_states,
            model_states::read_model_state,
            model_states::edit_model_state_labels,
            model_states::delete_model_state,
            sync_native_shell_state,
            validate_rule_pack,
            compute_rule_pack_document_checksum,
            save_local_rule_pack,
            open_local_rule_pack,
            list_local_rule_packs,
            delete_local_rule_pack,
            run_rule_checks,
            validate_library_import,
            save_local_library,
            open_local_library,
            list_local_libraries,
            check_model_library_refs,
            delete_local_library,
            render_calculation_report,
            save_report_package,
            save_local_result_json $(, $extra)*
        ]
    };
}

pub fn run() {
    let builder = tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .manage(SolveJobRegistry::new(memory_pool::MemoryPool::new(Arc::new(host_memory::MacHostMemory))))
        .manage(native_result_download::SaveAdmission::default())
        .menu(|handle| build_app_menu(handle))
        .on_menu_event(|app, event| {
            restore_native_check_item_after_activation(app, &event.id().0);
            dispatch_native_menu_command(app, &event.id().0);
        });
    #[cfg(not(feature = "native-smoke"))]
    let builder = builder.invoke_handler(app_invoke_handler!());
    #[cfg(feature = "native-smoke")]
    let builder = native_smoke::install(builder).invoke_handler(app_invoke_handler!(
        native_smoke::native_smoke_choose,
        native_smoke::native_smoke_menu,
        native_smoke::native_smoke_plan,
        native_smoke::native_smoke_log,
        native_smoke::native_smoke_external_save,
        native_smoke::native_smoke_finish,
    ));
    builder
        .run(tauri::generate_context!())
        .expect("error while running SWBPIPE");
}

/// The ordinary solve's bytes for a supplied model, read as a value, for tests
/// that inspect them (the job path publishes the same bytes).
#[cfg(test)]
fn run_preview_mechanics_value(model: Option<Value>) -> Result<Value, String> {
    solve_preview_mechanics_with_mode(resolve_solve_model_payload(model)?, PreviewSolverMode::default())
        .map(|bytes| serde_json::from_str(&bytes).expect("the producer's bytes are JSON"))
}

#[cfg(test)]
mod legacy_store_carry_forward_tests {
    use super::*;

    fn unique_root(label: &str) -> PathBuf {
        let root = std::env::temp_dir().join(format!(
            "swbpipe-carry-forward-{label}-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("clock")
                .as_nanos()
        ));
        fs::create_dir_all(&root).expect("root");
        root
    }

    fn dirs(root: &Path) -> (PathBuf, PathBuf) {
        let new_dir = root.join("com.chirality.swbpipe");
        let legacy_dir = root.join(LEGACY_BUNDLE_IDENTIFIER);
        fs::create_dir_all(&new_dir).expect("new dir");
        fs::create_dir_all(&legacy_dir).expect("legacy dir");
        (new_dir, legacy_dir)
    }

    /// A legacy store in WAL mode whose rows are committed but not
    /// checkpointed. The returned connection stays open, as the former app's
    /// would, so the rows live only in the -wal file.
    fn legacy_wal_store(legacy_dir: &Path, rows: &[&str]) -> (PathBuf, Connection) {
        let legacy_store = legacy_dir.join(PROJECT_STORE_FILE);
        let writer = Connection::open(&legacy_store).expect("legacy open");
        writer
            .execute_batch("PRAGMA journal_mode = WAL; PRAGMA wal_autocheckpoint = 0; CREATE TABLE marker (name TEXT NOT NULL);")
            .expect("legacy schema");
        for row in rows {
            writer
                .execute("INSERT INTO marker (name) VALUES (?1)", [row])
                .expect("legacy row");
        }
        let wal = store_sidecar_path(&legacy_store, "-wal");
        assert!(fs::metadata(&wal).expect("wal exists").len() > 0, "rows must be uncheckpointed");
        (legacy_store, writer)
    }

    fn marker_rows(store: &Path) -> Vec<String> {
        let reader = Connection::open_with_flags(store, rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY)
            .expect("read new store");
        let mut statement = reader.prepare("SELECT name FROM marker ORDER BY rowid").expect("prepare");
        let rows = statement
            .query_map([], |row| row.get::<_, String>(0))
            .expect("query")
            .collect::<Result<Vec<_>, _>>()
            .expect("rows");
        rows
    }

    #[test]
    fn new_store_present_copies_nothing_and_is_never_overwritten() {
        let root = unique_root("a");
        let (new_dir, legacy_dir) = dirs(&root);
        fs::write(new_dir.join(PROJECT_STORE_FILE), b"new").expect("new store");
        let (_legacy_store, _writer) = legacy_wal_store(&legacy_dir, &["legacy"]);
        // A sidecar beside an existing new store belongs to that store and stays.
        let new_wal = store_sidecar_path(&new_dir.join(PROJECT_STORE_FILE), "-wal");
        fs::write(&new_wal, b"new-wal").expect("new wal");

        assert_eq!(legacy_project_store_source(&new_dir), None);
        assert_eq!(carry_forward_legacy_project_store(&new_dir), Ok(false));
        assert_eq!(fs::read(new_dir.join(PROJECT_STORE_FILE)).expect("read"), b"new");
        assert_eq!(fs::read(&new_wal).expect("read wal"), b"new-wal");
        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn new_absent_and_legacy_present_carries_uncheckpointed_wal_rows_and_leaves_legacy_byte_identical() {
        let root = unique_root("b");
        let (new_dir, legacy_dir) = dirs(&root);
        let (legacy_store, writer) = legacy_wal_store(&legacy_dir, &["first", "second"]);
        let legacy_wal = store_sidecar_path(&legacy_store, "-wal");
        let main_before = fs::read(&legacy_store).expect("legacy main");
        let wal_before = fs::read(&legacy_wal).expect("legacy wal");

        assert_eq!(legacy_project_store_source(&new_dir), Some(legacy_store.clone()));
        assert_eq!(carry_forward_legacy_project_store(&new_dir), Ok(true));

        let new_store = new_dir.join(PROJECT_STORE_FILE);
        assert_eq!(marker_rows(&new_store), vec!["first".to_string(), "second".to_string()]);
        assert!(!store_sidecar_path(&new_store, CARRY_FORWARD_TEMPORARY_SUFFIX).exists());
        // The legacy main file and its write-ahead log are byte-identical, and
        // the legacy store is still in WAL mode with its rows readable.
        assert_eq!(fs::read(&legacy_store).expect("legacy main after"), main_before);
        assert_eq!(fs::read(&legacy_wal).expect("legacy wal after"), wal_before);
        let mode: String = writer
            .query_row("PRAGMA journal_mode", [], |row| row.get(0))
            .expect("journal mode");
        assert_eq!(mode, "wal");
        // A second call finds the new store and carries nothing.
        assert_eq!(carry_forward_legacy_project_store(&new_dir), Ok(false));
        drop(writer);
        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn both_absent_copies_nothing() {
        let root = unique_root("c");
        let new_dir = root.join("com.chirality.swbpipe");
        fs::create_dir_all(&new_dir).expect("new dir");
        assert_eq!(legacy_project_store_source(&new_dir), None);
        assert_eq!(carry_forward_legacy_project_store(&new_dir), Ok(false));
        assert!(!new_dir.join(PROJECT_STORE_FILE).exists());
        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn leftovers_of_a_killed_attempt_neither_corrupt_nor_block_the_carry_forward() {
        let root = unique_root("d");
        let (new_dir, legacy_dir) = dirs(&root);
        let (_legacy_store, writer) = legacy_wal_store(&legacy_dir, &["kept"]);
        let new_store = new_dir.join(PROJECT_STORE_FILE);
        // A stale WAL and shared-memory file with no main store, and a partial
        // temporary file: none of them is user data.
        let stale_wal = store_sidecar_path(&new_store, "-wal");
        let stale_shm = store_sidecar_path(&new_store, "-shm");
        let stale_temporary = store_sidecar_path(&new_store, CARRY_FORWARD_TEMPORARY_SUFFIX);
        fs::write(&stale_wal, b"stale write-ahead log from a killed attempt").expect("stale wal");
        fs::write(&stale_shm, b"stale shm").expect("stale shm");
        fs::write(&stale_temporary, b"partial").expect("stale temporary");

        assert_eq!(carry_forward_legacy_project_store(&new_dir), Ok(true));
        assert!(!stale_temporary.exists());
        assert_ne!(
            fs::read(&stale_wal).unwrap_or_default(),
            b"stale write-ahead log from a killed attempt".to_vec()
        );
        assert_eq!(marker_rows(&new_store), vec!["kept".to_string()]);
        let check = Connection::open(&new_store).expect("open new store");
        let integrity: String = check
            .query_row("PRAGMA integrity_check", [], |row| row.get(0))
            .expect("integrity");
        assert_eq!(integrity, "ok");
        drop(writer);
        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn an_unreadable_legacy_store_fails_without_leaving_files_or_touching_the_legacy_store() {
        let root = unique_root("f");
        let (new_dir, legacy_dir) = dirs(&root);
        let legacy_store = legacy_dir.join(PROJECT_STORE_FILE);
        fs::write(&legacy_store, b"this is not a sqlite database file at all").expect("legacy bytes");

        assert!(carry_forward_legacy_project_store(&new_dir).is_err());
        assert!(!new_dir.join(PROJECT_STORE_FILE).exists());
        assert!(!store_sidecar_path(&new_dir.join(PROJECT_STORE_FILE), CARRY_FORWARD_TEMPORARY_SUFFIX).exists());
        assert_eq!(
            fs::read(&legacy_store).expect("legacy after"),
            b"this is not a sqlite database file at all"
        );
        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn legacy_directory_is_the_new_directorys_sibling_and_never_itself() {
        let root = unique_root("e");
        let legacy_dir = root.join(LEGACY_BUNDLE_IDENTIFIER);
        fs::create_dir_all(&legacy_dir).expect("legacy dir");
        assert_eq!(legacy_project_store_source(&legacy_dir), None);
        let _ = fs::remove_dir_all(root);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    fn precision_model() -> Value {
        let rehearsal = read_fixture("r2_from_blank_rehearsal.json").unwrap();
        let (mut model, _) = apply_rehearsal_steps(&rehearsal);
        model["schema_version"] = json!(open_pipe_stress_product_physics::MODEL_SCHEMA_VERSION);
        model
    }

    // Synthetic metadata is exclusively a consumer-policy fixture, never a claim
    // that the product solve below has passed the structural integrity gate.
    // The policy fixture carries the
    // fresh preview-physics-2 identity with closed evidence and one table-signed row.
    fn preview_policy_envelope(model: &Value) -> Value {
        let cases = model["load_cases"].as_array().unwrap();
        let mut envelope = demo_solved_envelope(0.125);
        envelope["diagnostics"] = json!([]);
        envelope["schema_version"] = json!("0.2.0");
        envelope["model_ref"] = model["project"]["id"].clone();
        envelope["producer"] = json!({"component_name":"open_pipe_stress_product_physics","component_version":"0.2.0","semantic_contract_id":"openpipestress.result_semantics/0.3.0/preview-physics-2"});
        envelope["formulation_basis"] = json!({"profile_id":"product_preview_mechanics_v1","limitations":open_pipe_stress_result_export::semantic_contract::preview_physics_contract()["supported_profile_limitations"].clone()});
        envelope["results"][0]["kind"] = json!("global_nodal_rotation_x");
        envelope["results"][0]["unit"] = json!("rad");
        envelope["results"][0]["entity_ref"] = json!("node:policy-fixture");
        envelope["results"][0]["basis_ref"] = json!({"ref_type":"load_case","ref_id":cases[0]["id"]});
        envelope["results"][0]["metadata"] = json!({"component":"nodal_rotation_x","coordinate_system":"global","location":"node","basis":"invented_policy_fixture","sign_convention":"invented policy fixture; no engineering meaning"});
        envelope["summary"] = json!({"max_displacement":null,"max_open_formula_stress":null});
        envelope["contract_evidence"] = json!({"preview_cases":cases.iter().map(|case| json!({"load_case_id":case["id"],"pipe_stress_extrema":[],"stress_maximum_coverage":{"complete":true,"unavailable_pipe_ids":[],"outside_domain_pipe_ids":[]},"support_attribution":{"attributed_support_ids":[],"withheld":[]},"intensified_measures":[]})).collect::<Vec<_>>(),"combination_gates":[]});
        envelope["numerical_quality"] = json!({"value_representation":"finite_binary64","publication_quantization":"none","integrity_policy":"M03-INTEGRITY-v1","status":"checks_passed","cases":model["load_cases"].as_array().unwrap().iter().map(|case| json!({"basis_ref":{"ref_type":"load_case","ref_id":case["id"]},"structural_status":"passive_model_basis","solve_quality":"checks_passed","model_matrix_fidelity":"represented_equations_retained","accuracy_evidence":"not_claimed","evidence_refs":["result:stress:demo"]})).collect::<Vec<_>>()});
        envelope
    }

    #[test]
    fn precision_native_rule_qualification_requires_actual_complete_case_coverage() {
        let mut model = precision_model();
        model["load_cases"].as_array_mut().unwrap().push(json!({"id":"load:second"}));
        let envelope = preview_policy_envelope(&model);
        qualify_rule_mechanics(&model, &envelope).unwrap();
        // The same complete evidence under the retired precision-1 identity is unreadable (B1).
        let mut retired = envelope.clone();
        retired["producer"]["semantic_contract_id"] = json!("openpipestress.result_semantics/0.3.0/precision-1");
        retired.as_object_mut().unwrap().remove("contract_evidence");
        assert!(qualify_rule_mechanics(&model, &retired).unwrap_err().contains("RULE_NUMERICAL_SOURCE_UNSUPPORTED"));
        for status in ["not_assessed", "unresolved", "failed"] {
            let mut bad = envelope.clone(); bad["numerical_quality"]["status"] = json!(status);
            assert!(qualify_rule_mechanics(&model, &bad).unwrap_err().contains("NEEDS_RECOMPUTE"));
        }
        for mutation in 0..10 {
            let mut m = model.clone(); let mut e = envelope.clone();
            match mutation {
                0 => e["numerical_quality"]["cases"] = json!([]),
                1 => { e["numerical_quality"]["cases"].as_array_mut().unwrap().pop(); },
                2 => m["load_cases"] = json!([]),
                3 => m["load_cases"][1]["id"] = m["load_cases"][0]["id"].clone(),
                4 => e["model_ref"] = json!("project:other"),
                5 => e["producer"]["semantic_contract_id"] = json!("unknown"),
                6 => e["numerical_quality"]["cases"][0]["evidence_refs"] = json!([]),
                7 => m["load_cases"][0]["id"] = Value::Null,
                8 => e["status"]["mechanics"] = json!("MODEL_INCOMPLETE"),
                _ => e = demo_solved_envelope(0.125),
            }
            assert!(qualify_rule_mechanics(&m, &e).is_err(), "mutation {mutation}");
        }
    }

    fn precision_transport(model: &Value, source: &Value) -> Value {
        let root = std::env::temp_dir().join(format!("ops-precision-{}-{}", std::process::id(), std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
        std::fs::create_dir(&root).unwrap();
        let db = root.join("isolated.sqlite");
        let project = model["project"]["id"].as_str().unwrap();
        // This is the native command's JSON serialization boundary, in source units.
        let mut bridge: Value = serde_json::from_str(&serde_json::to_string(source).unwrap()).unwrap();
        let original_hash = model_payload_hash(source);
        let mh = computed_model_hash(model, project);
        let eh = computed_project_envelope_hash(project, model, &json!([]), &Value::Null, &Value::Null, &bridge, &Value::Null, &mh);
        for _ in 0..2 {
            let mut connection = Connection::open(&db).unwrap();
            apply_store_migrations(&connection).unwrap();
            assert_eq!(check_model_document_for_persist(model).unwrap().status, "current");
            let (persisted, saved_mh, saved_eh) = (model.clone(), mh.clone(), eh.clone());
            upsert_project(&mut connection, project, "Invented precision witness", &persisted, &json!([]), &Value::Null, &Value::Null, &bridge, &Value::Null, &saved_mh, &saved_eh).unwrap();
            connection.close().unwrap();
            let reopened = Connection::open(&db).unwrap();
            let loaded = load_project(&reopened, Some(project)).unwrap().unwrap();
            assert_eq!(loaded.model_hash, mh); assert_eq!(loaded.project_envelope_hash, eh);
            assert_eq!(model_payload_hash(&loaded.mechanics_result), original_hash);
            assert_eq!(loaded.mechanics_result, bridge);
            bridge = loaded.mechanics_result;
            reopened.close().unwrap();
        }
        let text = serde_json::to_string(&bridge).unwrap();
        let encoded: String = text.as_bytes().iter().map(|b| format!("%{b:02X}")).collect();
        let request = serde_json::from_value(json!({"href":format!("data:application/json;charset=utf-8,{encoded}"),"file_name":"openpipestress-preview-results-precision.json","screening":{"route_id":"DOTH-JSON-001","export_context":"local_private","explicit_local_private_intent":true,"blocked":false,"materialization_withheld":false,"lossless_required":true,"exact_payload_match":true,"blocking_count":0},"local_first":{"route_id":"DOTH-JSON-001","export_context":"local_private","storage_context":"local_private","action":"include_metadata_only","reason_code":"PRIVATE_LOCAL_METADATA_ALLOWED","blocked":false,"metadata_only":true,"explicit_local_private_intent":true}})).unwrap();
        let receipt = native_result_download::save_request(request, || Ok::<_, ()>(root.clone())).unwrap();
        let bytes = std::fs::read(root.join(receipt.file_name)).unwrap();
        assert_eq!(bytes, text.as_bytes());
        let downloaded: Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(model_payload_hash(&downloaded), original_hash);
        std::fs::remove_dir_all(root).unwrap();
        downloaded
    }

    #[test]
    fn precision_native_same_unit_edge_bits_store_download_and_rule_binding() {
        let model = precision_model();
        let vectors: Value = serde_json::from_str(include_str!("../../../../fixtures/canonical_hash/binary64_transport_vectors.json")).unwrap();
        for vector in vectors["vectors"].as_array().unwrap() {
            let bits = u64::from_str_radix(vector["bits_hex"].as_str().unwrap(), 16).unwrap();
            let value = f64::from_bits(bits);
            assert!(value.is_finite());
            // Preserve the existing I-JSON unsafe-integer exclusion.
            assert!(value.fract() != 0.0 || value.abs() <= 9_007_199_254_740_991.0);
            let decimal: f64 = serde_json::from_str(vector["decimal"].as_str().unwrap()).unwrap();
            assert_eq!(decimal.to_bits(), bits, "{}", vector["id"]);
            let mut source = preview_policy_envelope(&model);
            source["results"][0]["value"] = json!(value);
            let output = precision_transport(&model, &source);
            qualify_rule_mechanics(&model, &output).unwrap();
            let bindings = resolve_solver_result_bindings(Some(&demo_solver_selectors()), &output, &mut Vec::new()).unwrap();
            assert_eq!(bindings[0].value.to_bits(), bits, "{}", vector["id"]);
        }
        // Canonical hashing may normalize -0 to +0; all shared nonzero vectors
        // above retain their exact bits, with no unsafe-integer guard relaxation.
        assert_eq!(model_payload_hash(&json!({"value": -0.0_f64})), model_payload_hash(&json!({"value": 0.0_f64})));
        // b7-plan E2: the retired producer-less 0.1.0 carrier passes through unchanged
        // and is never qualified (its reader refuses it by name).
        let mut historical = read_fixture("invented_demo_result_preview_physics_1_sparse.json").unwrap();
        historical["schema_version"] = json!("0.1.0");
        for key in ["producer", "numerical_quality", "formulation_basis", "contract_evidence"] {
            historical.as_object_mut().unwrap().remove(key);
        }
        let mut historical_model = model.clone();
        historical_model["project"]["id"] = historical["model_ref"].clone();
        let output = precision_transport(&historical_model, &historical);
        assert_eq!(output, historical);
        assert!(qualify_rule_mechanics(&historical_model, &output).unwrap_err().starts_with("RULE_NUMERICAL_SOURCE_UNSUPPORTED"));
    }

    #[test]
    fn source_blocks_native_actual_invocation_store_and_rule_context() {
        // B4/D1 (b): source blocks remain only on the exact route's composites, so this
        // control reads pressure-source-1's committed request (B7b: an exact-contract
        // 0.7.0 model with 3.0.0/exact_pressure_v3, the v3 twin of physics-source-1's).
        let request: Value = serde_json::from_str(include_str!("../../../../fixtures/product_preview/pressure_source/physics_source/n05.request.json")).unwrap();
        let model = &request["model"];
        for mode in [PreviewSolverMode::DenseScrutiny, PreviewSolverMode::SparseInteractive] {
            let solved = solve_preview_mechanics_value_with_mode(model.clone(), mode).unwrap();
            assert_eq!(solved["producer"]["semantic_contract_id"], "openpipestress.result_semantics/0.3.0/pressure-source-1");
            assert_eq!(solved["source_block_recovery"]["body"]["status"], "qualified");
            let invocation = json!({"request":request,"solver_mode":mode.as_str()});
            qualify_rule_mechanics_with_context(model, &solved, Some(&invocation)).unwrap();
            let retained = precision_transport(model, &solved);
            assert_eq!(retained, solved);
            // Persistence retains inspectable source, not an invocation token.
            assert!(qualify_rule_mechanics(model, &retained).is_err());
            qualify_rule_mechanics_with_context(model, &retained, Some(&invocation)).unwrap();
            let mut changed = invocation.clone();
            changed["solver_mode"] = json!(if mode == PreviewSolverMode::DenseScrutiny {"sparse_interactive"} else {"dense_scrutiny"});
            assert!(qualify_rule_mechanics_with_context(model, &retained, Some(&changed)).is_err());
            changed = invocation.clone(); changed["request"]["model"]["project"]["description"] = json!("changed after actual native solve");
            assert!(qualify_rule_mechanics_with_context(model, &retained, Some(&changed)).is_err());
        }
    }

    #[test]
    fn actual_exact_native_route_uses_derived_g_and_preserves_explicit_model_custody() {
        let mut model: Value = serde_json::from_str(include_str!(
            "../../../../fixtures/model_operations/exact_pressure_authoring_model.json"
        )).unwrap();
        model["materials"][0]["shear_modulus"] = json!({"value":77e9,"unit":"Pa"});
        for mode in [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny] {
            let source = solve_preview_mechanics_value_with_mode(model.clone(), mode).unwrap();
            assert_eq!(source["status"]["mechanics"], "MECHANICS_SOLVED");
            assert_eq!(source["producer"]["semantic_contract_id"], "openpipestress.result_semantics/0.3.0/pressure-2");
            let row = source["results"].as_array().unwrap().iter().find(|r|
                r["entity_ref"] == "node:fixture-tip" && r["kind"] == "global_nodal_rotation_x"
                && r["basis_ref"]["ref_id"] == "case:six-component-load"
            ).unwrap();
            let derived_g = 200e9 / (2.0 * (1.0 + 0.3));
            let j = std::f64::consts::PI / 2.0 * (0.06_f64.powi(4) - 0.05_f64.powi(4));
            let expected = 400.0 / (derived_g * j);
            assert_eq!(row["unit"], "rad");
            assert!(((row["value"].as_f64().unwrap() - expected) / expected).abs() <= 1e-9);
            let invocation = json!({"request":{"model":model,"materials":[]},"solver_mode":mode.as_str()});
            qualify_rule_mechanics_with_context(&model, &source, Some(&invocation)).unwrap();
            assert_eq!(precision_transport(&model, &source), source);
            assert_eq!(model["materials"][0]["shear_modulus"]["value"], 77e9);
        }
    }

    // Epoch 8 (DEL-04-07 Design desktop-exact-outputs §6.1, VER-007): the backend's
    // unchanged rule-check qualification and row binding, read on committed exact
    // fixtures. The desktop jsdom tests mock this backend, so these are its check.
    fn exact_rule_fixture(relative: &str) -> Value {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..").join(relative);
        let text = fs::read_to_string(&path).unwrap_or_else(|error| panic!("{}: {error}", path.display()));
        serde_json::from_str(&text).unwrap()
    }

    const EXACT_RULE_MODES: [&str; 2] = ["sparse_interactive", "dense_scrutiny"];

    /// No row of an admitted exact source is refused by the shared binding helper,
    /// and a selector binds a published row by id with its exact bits and unit.
    fn assert_exact_rows_bind(source: &Value, label: &str) {
        let rows = source["results"].as_array().unwrap();
        assert!(!rows.is_empty(), "{label}");
        for row in rows {
            assert_eq!(open_pipe_stress_result_export::semantic_contract::rule_binding_refusal(source, row), None, "{label}: {}", row["id"]);
        }
        let row = rows.iter().find(|row| row["value"].as_f64().is_some() && row["unit"].is_string()).unwrap();
        let selectors = json!([{"input_id":"exact_actual", "result_id":row["id"]}]);
        let mut refused = Vec::new();
        let bindings = resolve_solver_result_bindings(Some(&selectors), source, &mut refused).unwrap();
        assert!(refused.is_empty(), "{label}");
        assert_eq!(bindings.len(), 1, "{label}");
        assert_eq!(bindings[0].result_id, row["id"].as_str().unwrap(), "{label}");
        assert_eq!(bindings[0].value.to_bits(), row["value"].as_f64().unwrap().to_bits(), "{label}");
        assert_eq!(bindings[0].unit, row["unit"].as_str().unwrap(), "{label}");
    }

    #[test]
    fn exact_rule_check_pressure_plain_admitted_without_invocation_and_rows_bind() {
        let request = exact_rule_fixture("core/product_physics/tests/fixtures/exact_pressure_connected_request.json");
        let model = &request["model"];
        for mode in ["sparse", "dense"] {
            let label = format!("pressure_connected_mechanics_{mode}");
            let source = exact_rule_fixture(&format!("fixtures/results/{label}.json"));
            assert_eq!(source["producer"]["semantic_contract_id"], "openpipestress.result_semantics/0.3.0/pressure-2", "{label}");
            assert!(source["contract_evidence"].get("load_reference_states").is_none(), "{label}");
            qualify_rule_mechanics_with_context(model, &source, None).unwrap_or_else(|error| panic!("{label}: {error}"));
            assert_exact_rows_bind(&source, &label);
        }
    }

    #[test]
    fn exact_rule_check_pressure_load_reference_admitted_without_invocation_and_rows_bind() {
        for name in ["connected", "pressure"] {
            let request = exact_rule_fixture(&format!("fixtures/product_preview/pressure/load_reference/{name}.request.json"));
            let model = &request["model"];
            for mode in EXACT_RULE_MODES {
                let label = format!("pressure/load_reference/{name}-{mode}");
                let source = exact_rule_fixture(&format!("fixtures/product_preview/pressure/load_reference/{name}-{mode}.raw.json"));
                assert_eq!(source["producer"]["semantic_contract_id"], "openpipestress.result_semantics/0.3.0/pressure-2", "{label}");
                assert!(source["contract_evidence"]["load_reference_states"].as_array().is_some_and(|states| !states.is_empty()), "{label}");
                qualify_rule_mechanics_with_context(model, &source, None).unwrap_or_else(|error| panic!("{label}: {error}"));
                assert_exact_rows_bind(&source, &label);
            }
        }
    }

    #[test]
    fn exact_rule_check_pressure_source_plain_admitted_with_registered_invocation_and_rows_bind() {
        for name in ["fields", "mixed", "mixed_units", "n05", "n05_unicode", "n05_units", "n06"] {
            let request = exact_rule_fixture(&format!("fixtures/product_preview/pressure_source/physics_source/{name}.request.json"));
            let model = &request["model"];
            for mode in EXACT_RULE_MODES {
                let label = format!("pressure_source/physics_source/{name}-{mode}");
                let source = exact_rule_fixture(&format!("fixtures/product_preview/pressure_source/physics_source/{name}-{mode}.raw.json"));
                assert_eq!(source["producer"]["semantic_contract_id"], "openpipestress.result_semantics/0.3.0/pressure-source-1", "{label}");
                // The registered invocation, as resultStanding.ts builds it.
                let invocation = json!({"request":request,"solver_mode":mode});
                qualify_rule_mechanics_with_context(model, &source, Some(&invocation)).unwrap_or_else(|error| panic!("{label}: {error}"));
                assert_exact_rows_bind(&source, &label);
                // Without its invocation the backend's standing cannot be eligible.
                assert!(qualify_rule_mechanics_with_context(model, &source, None).is_err(), "{label}");
            }
        }
    }

    #[test]
    fn exact_rule_check_pressure_source_load_reference_refused_needs_recompute_with_invocation() {
        for name in ["eigen_motion", "fields", "mixed", "n05", "n06"] {
            let request = exact_rule_fixture(&format!("fixtures/product_preview/pressure_source/load_reference_source/{name}.request.json"));
            let model = &request["model"];
            for mode in EXACT_RULE_MODES {
                let label = format!("pressure_source/load_reference_source/{name}-{mode}");
                let source = exact_rule_fixture(&format!("fixtures/product_preview/pressure_source/load_reference_source/{name}-{mode}.raw.json"));
                assert_eq!(source["producer"]["semantic_contract_id"], "openpipestress.result_semantics/0.3.0/pressure-source-1", "{label}");
                let invocation = json!({"request":request,"solver_mode":mode});
                let error = qualify_rule_mechanics_with_context(model, &source, Some(&invocation)).unwrap_err();
                assert!(error.starts_with("RULE_NUMERICAL_NEEDS_RECOMPUTE"), "{label}: {error}");
                // R2 (fixtures/results/base_reader_outcomes.json) reads this source as needs_recompute.
                assert_eq!(open_pipe_stress_result_export::semantic_contract::numerical_use_standing_with_context(
                    &source,
                    &model["load_cases"].as_array().unwrap().iter().map(|case| json!({"ref_type":"load_case","ref_id":case["id"]})).collect::<Vec<_>>(),
                    Some(&invocation),
                ), "needs_recompute", "{label}");
            }
        }
    }

    #[test]
    fn precision_native_actual_tiny_signed_cantilever_survives_store_and_download() {
        for force in [-1.0e-6_f64, 1.0e-6] {
            let mut model = precision_model();
            model["load_cases"][0]["primitive_loads"][0]["magnitude"]["value"] = json!(force);
            let solved = run_preview_mechanics_value(Some(model.clone())).unwrap();
            assert_eq!(solved["status"]["mechanics"], "MECHANICS_SOLVED");
            assert_eq!(solved["schema_version"], "0.2.0");
            assert_eq!(solved["producer"]["component_version"], "0.2.0");
            // T0R: a fresh non-exact solve publishes preview-physics-2.
            assert_eq!(solved["producer"]["semantic_contract_id"], "openpipestress.result_semantics/0.3.0/preview-physics-2");
            let output = precision_transport(&model, &solved);
            // Elementary Euler-Bernoulli tip-force oracle, independent of solver assembly.
            let inertia = std::f64::consts::PI * (0.114_f64.powi(4) - 0.102_f64.powi(4)) / 64.0;
            for (kind, expected, unit) in [("global_nodal_displacement_y", force * 3.0_f64.powi(3) / (3.0 * 200e9 * inertia) * 1000.0, "mm"), ("global_nodal_rotation_z", force * 3.0_f64.powi(2) / (2.0 * 200e9 * inertia), "rad")] {
                let row = output["results"].as_array().unwrap().iter().find(|row| row["entity_ref"] == "node:R2-110" && row["kind"] == kind).unwrap();
                let actual = row["value"].as_f64().unwrap();
                assert_eq!(row["unit"], unit);
                assert!(actual != 0.0 && actual.signum() == force.signum());
                assert!((actual - expected).abs() <= expected.abs() * 1e-10, "{kind}: {actual} != {expected}");
                let selectors = json!([{"input_id":"tiny", "result_id":row["id"]}]);
                let bindings = resolve_solver_result_bindings(Some(&selectors), &output, &mut Vec::new()).unwrap();
                assert_eq!(bindings[0].value.to_bits(), actual.to_bits());
            }
            // Preserve the actual gate output; no passing metadata is manufactured.
            assert_eq!(output["numerical_quality"], solved["numerical_quality"]);
        }
    }

    fn allowed_report_package_local_first_evidence() -> ReportPackageLocalFirstEvidence {
        ReportPackageLocalFirstEvidence {
            route_id: "DREP-PACKAGE-SAVE-009".to_string(),
            export_context: "local_private".to_string(),
            storage_context: "local_private".to_string(),
            action: "include_metadata_only".to_string(),
            reason_code: "PRIVATE_LOCAL_METADATA_ALLOWED".to_string(),
            blocked: false,
            metadata_only: true,
            explicit_local_private_intent: true,
        }
    }

    #[test]
    fn report_package_local_first_evidence_validator_allows_exact_evidence() {
        let evidence = allowed_report_package_local_first_evidence();
        assert_eq!(
            validate_report_package_local_first_evidence(Some(&evidence)),
            Ok(())
        );
    }

    #[test]
    fn report_package_local_first_evidence_validator_rejects_missing_or_mismatched_evidence() {
        assert!(validate_report_package_local_first_evidence(None).is_err());

        let mut evidence = allowed_report_package_local_first_evidence();
        evidence.route_id = "DREP-JSON-002".to_string();
        assert!(validate_report_package_local_first_evidence(Some(&evidence)).is_err());

        let mut evidence = allowed_report_package_local_first_evidence();
        evidence.metadata_only = false;
        assert!(validate_report_package_local_first_evidence(Some(&evidence)).is_err());
    }

    #[test]
    fn report_package_local_first_evidence_validator_rejects_blocked_evidence() {
        let mut evidence = allowed_report_package_local_first_evidence();
        evidence.blocked = true;
        evidence.action = "block_storage".to_string();
        evidence.reason_code = "LOCAL_PRIVATE_INTENT_REQUIRED".to_string();
        evidence.explicit_local_private_intent = false;

        assert!(validate_report_package_local_first_evidence(Some(&evidence)).is_err());
    }

    #[test]
    fn packaged_binary_self_test_persists_restores_and_solves_edited_load() {
        let evidence = run_packaged_saved_edited_load_self_test().expect("frozen cantilever file-native lifecycle passes");
        assert_eq!(evidence["status"], json!("PASS"));
        assert_eq!(evidence["operation"]["after"], json!(500));
        assert_eq!(evidence["persistence"]["restored_value"], json!(500));
        assert_eq!(evidence["persistence"]["historical_result_preserved"], json!(true));
        assert_eq!(evidence["persistence"]["unchanged_save_preserves_evidence"], json!(true));
        assert_eq!(evidence["persistence"]["input_manifest"], json!("missing_not_invented"));
        assert_eq!(evidence["hashes"]["baseline"], evidence["hashes"]["undo_checkpoint"]);
        assert_eq!(evidence["hashes"]["edit"], evidence["hashes"]["redo_checkpoint"]);
        assert_eq!(evidence["hashes"]["edit"], evidence["hashes"]["equivalent_agent"]);
        assert_eq!(evidence["solve"]["mechanics"], json!("MECHANICS_SOLVED"));
        assert_eq!(evidence["solve"]["fresh_resolve_matches_saved_result_rows"], json!(true));
        assert_eq!(evidence["solve"]["analytical_witnesses"].as_array().unwrap().len(), 3);
        assert_eq!(evidence["boundary"]["repository_write"], json!(false));
    }

    #[test]
    fn native_menu_dispatch_script_targets_the_main_dom_command_event() {
        assert_eq!(
            native_menu_dispatch_script("view.section.report"),
            "window.dispatchEvent(new CustomEvent('openpipestress-native-menu-command', { detail: \"view.section.report\" }));"
        );
        let escaped = native_menu_dispatch_script("view.section.\"report\"");
        assert!(escaped.contains("view.section.\\\"report\\\""));
    }

    fn sample_native_shell_state() -> NativeShellState {
        NativeShellState {
            project_name: Some("Loop 4 header".to_string()),
            model_edited: false,
            project_busy: false,
            stage: "loads".to_string(),
            view: "table".to_string(),
            theme: "dark".to_string(),
            density: "compact".to_string(),
            results_stage_enabled: true,
            review_stage_enabled: false,
            inspector_open: false,
            can_undo: true,
            can_redo: false,
            can_run: true,
            can_cancel: false,
        }
    }

    #[test]
    fn native_shell_state_accepts_only_supported_menu_domains() {
        let state = sample_native_shell_state();
        assert!(validate_native_shell_state(&state).is_ok());

        for (field, value) in [
            ("stage", "libraries"),
            ("view", "split"),
            ("theme", "auto"),
            ("density", "dense"),
        ] {
            let mut invalid = sample_native_shell_state();
            match field {
                "stage" => invalid.stage = value.to_string(),
                "view" => invalid.view = value.to_string(),
                "theme" => invalid.theme = value.to_string(),
                "density" => invalid.density = value.to_string(),
                _ => unreachable!(),
            }
            let error = validate_native_shell_state(&invalid).unwrap_err();
            assert!(error.contains(field));
            assert!(error.contains(value));
        }

        let mut invalid_review = sample_native_shell_state();
        invalid_review.stage = "review".to_string();
        invalid_review.view = "both".to_string();
        assert_eq!(
            validate_native_shell_state(&invalid_review).unwrap_err(),
            "NATIVE-SHELL-STATE-INVALID: Review stage must use Table view"
        );
    }

    #[test]
    fn native_shell_state_projects_checks_and_enabled_actions() {
        let mutations = native_menu_mutations(&sample_native_shell_state());
        for expected in [
            NativeMenuMutation::Checked("view.stage.loads", true),
            NativeMenuMutation::Checked("view.stage.model", false),
            NativeMenuMutation::Checked("view.view.table", true),
            NativeMenuMutation::Checked("view.theme.dark", true),
            NativeMenuMutation::Checked("view.density.compact", true),
            NativeMenuMutation::Enabled("view.stage.results", true),
            NativeMenuMutation::Enabled("view.stage.review", false),
            NativeMenuMutation::Enabled("view.inspector", false),
            NativeMenuMutation::Enabled("edit.undo", true),
            NativeMenuMutation::Enabled("edit.redo", false),
            NativeMenuMutation::Enabled("analyze.run", true),
            NativeMenuMutation::Enabled("analyze.cancel", false),
        ] {
            assert!(mutations.contains(&expected), "missing {expected:?}");
        }
    }

    #[test]
    fn native_shell_state_disables_all_project_commands_while_busy() {
        for busy in [false, true] {
            let mut state = sample_native_shell_state();
            state.project_busy = busy;
            let mutations = native_menu_mutations(&state);
            for id in ["file.new-local", "file.new-blank", "file.open-model", "file.open-local", "file.save-local", "file.save-model-document", "file.list-local"] {
                assert!(mutations.contains(&NativeMenuMutation::Enabled(id, !busy)));
            }
            assert!(mutations.contains(&NativeMenuMutation::Enabled("analyze.run", true)));
        }
    }

    #[test]
    fn native_window_title_uses_normalized_project_identity() {
        assert_eq!(
            native_window_title(Some("Loop 4 header"), true),
            "Loop 4 header · Edited — SWBPIPE"
        );
        assert_eq!(native_window_title(None, true), "SWBPIPE");
        assert_eq!(native_window_title(None, false), "SWBPIPE");
        assert_eq!(native_window_title(Some("  \n\t"), false), "SWBPIPE");
        assert_eq!(
            native_window_title(Some("  Loop 4\nheader  "), false),
            "Loop 4 header — SWBPIPE"
        );
    }

    #[test]
    fn native_window_minimum_is_1280_by_800() {
        let config: Value = serde_json::from_str(include_str!("../tauri.conf.json"))
            .expect("Tauri configuration parses");
        let window = &config["app"]["windows"][0];
        assert_eq!(window["minWidth"], json!(1280));
        assert_eq!(window["minHeight"], json!(800));
    }

    fn unit_entry<'a>(catalog: &'a UnitCatalogResponse, unit_id: &str) -> &'a UnitCatalogEntry {
        catalog
            .entries
            .iter()
            .find(|entry| entry.unit_id == unit_id)
            .unwrap_or_else(|| panic!("unit catalog entry not found: {unit_id}"))
    }

    #[test]
    fn get_unit_catalog_exposes_b2_schema_facing_metadata() {
        let catalog = get_unit_catalog();

        assert_eq!(catalog.decision_basis, "DEC-018");
        assert_eq!(catalog.calculation_basis, "si_canonical");
        assert_eq!(catalog.storage_convention, "entered_units_preserved");
        assert_eq!(catalog.entry_count, catalog.entries.len());
        assert_eq!(catalog.entry_count, open_pipe_stress_units::catalog().len());
        assert!(!catalog.boundary.protected_content_included);
        assert!(!catalog.boundary.private_project_data_included);
        assert!(!catalog.boundary.professional_approval_claimed);
        assert!(!catalog.boundary.code_compliance_claimed);

        let ids = catalog
            .entries
            .iter()
            .map(|entry| entry.unit_id)
            .collect::<BTreeSet<_>>();
        assert_eq!(ids.len(), catalog.entries.len());
        assert!(catalog
            .entries
            .iter()
            .all(|entry| !entry.factor_representation.trim().is_empty()));
        assert!(catalog
            .entries
            .iter()
            .all(|entry| entry.review_status == "accepted"));

        let inch = unit_entry(&catalog, "unit:inch");
        assert_eq!(inch.symbol, "in");
        assert_eq!(inch.dimension_id, "length");
        assert_eq!(inch.transform_kind, "multiplicative");
        assert!(inch.factor_representation.contains("25.4 mm/in"));
        assert_eq!(inch.provenance, "exact_public_definition");

        let fahrenheit = unit_entry(&catalog, "unit:degree_fahrenheit");
        assert_eq!(fahrenheit.transform_kind, "affine");
        assert!(fahrenheit
            .offset_representation
            .expect("fahrenheit offset")
            .contains("459.67"));

        assert_eq!(
            unit_entry(&catalog, "unit:pound_force").provenance,
            "conventional_public_constant"
        );
        assert_eq!(
            unit_entry(&catalog, "unit:newton_per_meter_linear").provenance,
            "project_governed_decision"
        );
        assert_eq!(
            unit_entry(&catalog, "unit:meter").transform_kind,
            "identity"
        );

        let payload = serde_json::to_value(&catalog).expect("catalog serializes");
        assert_eq!(payload["entries"][0]["unit_id"].is_string(), true);
        assert_eq!(
            payload["boundary"]["code_compliance_claimed"],
            Value::Bool(false)
        );
    }

    #[test]
    fn render_calculation_report_command_renders_fixture_unblocked() {
        let fixture_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../../fixtures/reports/invented/calculation_report_fixture.json");
        let fixture: Value =
            serde_json::from_str(&std::fs::read_to_string(fixture_path).expect("fixture read"))
                .expect("fixture parses");
        let provenance = json!({
            "source_name": "Invented SWBPIPE renderer test",
            "source_location": "apps/desktop/src-tauri/src/lib.rs",
            "source_license": "project_fixture",
            "contributor": "SWBPIPE",
            "contributor_certification": "Invented non-engineering data only.",
            "redistribution_status": "invented_non_engineering_example",
            "review_status": "accepted",
            "privacy_classification": "invented_public_example"
        });
        let input = json!({
            "report_title": "Invented Calculation Report",
            "calculation_report": fixture["calculation_report"],
            "report_sections": {
                "report_section_id": "invented-report-sections-001",
                "model_ref": {"ref_type": "model", "ref_id": "invented-model"},
                "run_ref": {"ref_type": "analysis_run", "ref_id": "invented-run"},
                "diagnostics": [],
                "analysis_status_disclosures": [
                    {
                        "status": "MECHANICS_SOLVED",
                        "source": {"ref_type": "solver", "ref_id": "invented-solver"},
                        "affected_object": {"ref_type": "model", "ref_id": "invented-model"},
                        "explanation": "Invented preview mechanics solved.",
                        "human_review_required": true,
                        "human_acceptance_ref": null
                    },
                    {
                        "status": "HUMAN_REVIEW_REQUIRED",
                        "source": {"ref_type": "report_renderer", "ref_id": "invented-render"},
                        "affected_object": {"ref_type": "report", "ref_id": "invented-report-001"},
                        "explanation": "Human professional review is required before any reliance.",
                        "human_review_required": true,
                        "human_acceptance_ref": null
                    }
                ],
                "provenance_notes": [provenance],
                "user_supplied_values": [],
                "assumptions": [],
                "limitations": [
                    {
                        "limitation_id": "invented-limitation-001",
                        "source": {"ref_type": "report_renderer", "ref_id": "invented-render"},
                        "affected_scope": {"ref_type": "model", "ref_id": "invented-model"},
                        "statement": "Invented preview data only; not engineering output.",
                        "effect": {
                            "mechanics_solve_qualified": true,
                            "user_rule_check_qualified": false,
                            "report_completeness": "qualified",
                            "human_review_required": true
                        },
                        "provenance": provenance
                    }
                ],
                "unresolved_tbds": [],
                "professional_boundary": {
                    "human_review_required": true,
                    "software_makes_compliance_claim": false,
                    "software_makes_certification_claim": false,
                    "software_makes_sealing_claim": false,
                    "software_makes_approval_claim": false,
                    "software_makes_authentication_claim": false
                }
            },
            "result_rows": [
                {
                    "row_id": "invented-row-001",
                    "label": "Max bending stress (invented)",
                    "case_ref": "load_case:invented-weight",
                    "quantity_display": "12.5 MPa",
                    "source_ref": "result_envelope:invented-result"
                }
            ]
        });

        let outcome = render_calculation_report_value(input).expect("command renders");

        assert_eq!(outcome["export_blocked"], Value::Bool(false));
        assert_eq!(outcome["sha256_hex"].as_str().expect("hash").len(), 64);
        assert!(outcome["html"]
            .as_str()
            .expect("html")
            .starts_with("<!DOCTYPE html>"));
        let html = outcome["html"].as_str().expect("html");
        assert!(html.contains("Unit storage convention"));
        assert!(html.contains("Model units"));
        assert!(html.contains("force=N, length=m, stress=MPa"));
        let derived = outcome["derived_print_html"]
            .as_str()
            .expect("derived view");
        assert!(derived.contains("DERIVED VIEW"));
        assert!(derived.contains(outcome["sha256_hex"].as_str().expect("hash")));

        let rejected = render_calculation_report_value(json!({"report_title": "broken"}));
        assert!(rejected
            .expect_err("invalid input")
            .starts_with("RENDER-INPUT-INVALID"));
    }

    fn rehearsal_step_text<'a>(step: &'a Value, key: &str) -> &'a str {
        step.get(key)
            .and_then(Value::as_str)
            .unwrap_or_else(|| panic!("rehearsal step missing string field `{key}`"))
    }

    fn rehearsal_intent(step: &Value) -> Value {
        let payload = step
            .get("payload")
            .unwrap_or_else(|| panic!("rehearsal step {} missing payload", step["operation_id"]));
        json!({
            "operation_id": rehearsal_step_text(step, "operation_id"),
            "operation_kind": rehearsal_step_text(step, "operation_kind"),
            "operation_status": "proposed",
            "author_type": "user",
            "source": {
                "source_ref": "fixtures/product_preview/r2_from_blank_rehearsal.json",
                "source_channel": "local_desktop_preview",
                "source_role": "a12_rehearsal_script"
            },
            "target": step["target"].clone(),
            "change": {
                "change_id": format!(
                    "change:{}",
                    rehearsal_step_text(step, "operation_id").trim_start_matches("op:")
                ),
                "change_kind": rehearsal_step_text(step, "change_kind"),
                "field_label": rehearsal_step_text(step, "field_label"),
                "field_path": rehearsal_step_text(step, "field_path"),
                "before": rehearsal_step_text(step, "before"),
                "after": serde_json::to_string(payload).expect("rehearsal payload serializes"),
                "unit": rehearsal_step_text(step, "unit"),
                "dimension": rehearsal_step_text(step, "dimension"),
                "source_note": rehearsal_step_text(step, "source_note")
            },
            "validation": {
                "schema_validation": "not_run",
                "constraint_validation": "not_run",
                "unit_validation": "not_run",
                "diff_preview_status": "not_generated",
                "application_status": "not_applied"
            },
            "audit_boundary": {
                "mutation_route": "structured_operations_only",
                "direct_model_mutation_allowed": false,
                "requires_user_acceptance": true,
                "mutates_accepted_model_state": false
            },
            "professional_boundary": {
                "human_review_required": true,
                "software_makes_compliance_claim": false,
                "software_makes_certification_claim": false,
                "software_makes_sealing_claim": false,
                "software_makes_approval_claim": false,
                "software_makes_authentication_claim": false
            },
            "rationale": "A12 from-blank rehearsal step; proves the structured-operation seam composes without raw model-file editing."
        })
    }

    fn apply_rehearsal_steps(rehearsal: &Value) -> (Value, Vec<Value>) {
        let mut model = rehearsal["blank_model"].clone();
        let mut receipts = Vec::new();
        let steps = rehearsal["steps"]
            .as_array()
            .expect("rehearsal steps array");
        for step in steps {
            let intent = rehearsal_intent(step);
            let outcome = apply_model_operation_value(model, intent, None)
                .unwrap_or_else(|error| panic!("rehearsal operation command failed: {error}"));
            assert_eq!(
                outcome["validation"]["application_status"],
                json!("applied_to_session_model"),
                "rehearsal step {} should apply; diagnostics={:?}",
                step["operation_id"],
                outcome["diagnostics"]
            );
            assert_eq!(
                outcome["acceptance"]["acceptance_is_professional_approval"],
                json!(false)
            );
            model = outcome["applied_model"].clone();
            receipts.push(outcome);
        }
        (model, receipts)
    }

    fn count_array(value: &Value, key: &str) -> usize {
        value
            .get(key)
            .and_then(Value::as_array)
            .unwrap_or_else(|| panic!("missing array `{key}`"))
            .len()
    }

    fn read_report_fixture() -> Value {
        let fixture_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../../fixtures/reports/invented/calculation_report_fixture.json");
        serde_json::from_str(&std::fs::read_to_string(fixture_path).expect("fixture read"))
            .expect("fixture parses")
    }

    fn provenance(source_location: &str) -> Value {
        json!({
            "source_name": "SWBPIPE A12 from-blank rehearsal",
            "source_location": source_location,
            "source_license": "project_fixture",
            "contributor": "SWBPIPE",
            "contributor_certification": "Invented non-engineering data only.",
            "redistribution_status": "invented_non_engineering_example",
            "review_status": "accepted",
            "privacy_classification": "invented_public_example"
        })
    }

    fn report_row_from_result(result: &Value) -> Value {
        let row_id = result["id"].as_str().unwrap_or("result:unknown");
        let kind = result["kind"].as_str().unwrap_or("result");
        let unit = result["unit"].as_str().unwrap_or("unit");
        let value = result["value"]
            .as_f64()
            .map(|number| number.to_string())
            .unwrap_or_else(|| "TBD".to_string());
        let case_ref = result
            .get("basis_ref")
            .and_then(Value::as_object)
            .map(|basis| {
                format!(
                    "{}:{}",
                    basis
                        .get("ref_type")
                        .and_then(Value::as_str)
                        .unwrap_or("basis"),
                    basis.get("ref_id").and_then(Value::as_str).unwrap_or("TBD")
                )
            })
            .unwrap_or_else(|| "TBD".to_string());
        json!({
            "row_id": format!("row:{row_id}"),
            "label": kind,
            "case_ref": case_ref,
            "quantity_display": format!("{value} {unit}"),
            "source_ref": format!("result:{row_id}")
        })
    }

    fn rehearsal_report_input(model: &Value, solved: &Value) -> Value {
        let mut fixture = read_report_fixture();
        let model_id = model["project"]["id"].as_str().expect("rehearsal model id");
        let project_name = model["project"]["name"]
            .as_str()
            .expect("rehearsal project name");
        let run_id = solved["run_id"].as_str().expect("rehearsal run id");
        let source_location = "fixtures/product_preview/r2_from_blank_rehearsal.json";
        let provenance = provenance(source_location);

        fixture["calculation_report"]["report_id"] = json!(format!("report:{run_id}"));
        fixture["calculation_report"]["model_input_summary"]["project_ref"] =
            json!({"ref_type": "project", "ref_id": model_id});
        fixture["calculation_report"]["model_input_summary"]["model_ref"] =
            json!({"ref_type": "model", "ref_id": model_id});
        // The model hash names the summary's model (checksum subjects,
        // DEL-08-01 Design §2.4).
        fixture["calculation_report"]["model_input_summary"]["model_hash"]["payload_ref"] =
            json!({"ref_type": "model", "ref_id": model_id});
        fixture["calculation_report"]["model_input_summary"]["persistence_ref"] = json!({"ref_type": "project_persistence", "ref_id": "session_state_only_not_yet_saved"});
        fixture["calculation_report"]["model_input_summary"]["unit_system_ref"] =
            json!({"ref_type": "unit_system", "ref_id": "a12-rehearsal-si-local"});
        let model_units = model["project"]["units"]
            .as_object()
            .map(|units| {
                let mut sorted_keys = units.keys().collect::<Vec<_>>();
                sorted_keys.sort();
                let mut sorted = serde_json::Map::new();
                for key in sorted_keys {
                    sorted.insert(key.clone(), units[key].clone());
                }
                Value::Object(sorted)
            })
            .unwrap_or_else(|| json!({}));
        let mut result_units = solved["results"]
            .as_array()
            .expect("rehearsal result rows")
            .iter()
            .filter_map(|row| row.get("unit").and_then(Value::as_str))
            .map(str::to_string)
            .collect::<Vec<_>>();
        result_units.sort();
        result_units.dedup();
        fixture["calculation_report"]["model_input_summary"]["unit_display_summary"] = json!({
            "storage_convention": "entered_units_preserved",
            "model_units": model_units,
            "result_units": result_units,
            "quantity_display_policy": "display result-row values with their explicit units; no report-time conversion",
            "conversion_performed": false
        });
        fixture["calculation_report"]["model_input_summary"]["provenance"] = provenance.clone();
        fixture["calculation_report"]["load_case_summary"] = Value::Array(
            model["load_cases"]
                .as_array()
                .expect("rehearsal load cases")
                .iter()
                .map(|load_case| {
                    json!({
                        "load_ref": {
                            "ref_type": "load_case",
                            "ref_id": load_case["id"].as_str().unwrap_or("load:TBD")
                        },
                        "label": load_case["label"].as_str().unwrap_or("TBD"),
                        "basis": load_case["kind"].as_str().unwrap_or("TBD"),
                        "source": { "ref_type": "model", "ref_id": model_id },
                        "provenance": provenance.clone()
                    })
                })
                .collect(),
        );
        fixture["calculation_report"]["analysis_status"] = json!([
            "HUMAN_REVIEW_REQUIRED",
            solved["status"]["mechanics"]
                .as_str()
                .unwrap_or("MODEL_INCOMPLETE"),
            solved["status"]["rule_check"]
                .as_str()
                .unwrap_or("RULE_INPUTS_INCOMPLETE")
        ]);
        fixture["calculation_report"]["provenance"] = provenance.clone();

        let first_result = solved["results"]
            .as_array()
            .and_then(|rows| rows.first())
            .expect("rehearsal solve emits at least one result row");
        let report_sections = json!({
            "report_section_id": format!("sections:{run_id}"),
            "model_ref": {"ref_type": "model", "ref_id": model_id},
            "run_ref": {"ref_type": "analysis_run", "ref_id": run_id},
            "diagnostics": [],
            "analysis_status_disclosures": [
                {
                    "status": solved["status"]["mechanics"].as_str().unwrap_or("MECHANICS_SOLVED"),
                    "source": {"ref_type": "analysis_run", "ref_id": run_id},
                    "affected_object": {"ref_type": "model", "ref_id": model_id},
                    "explanation": "A12 invented from-blank rehearsal solved through the backend preview mechanics command.",
                    "human_review_required": true,
                    "human_acceptance_ref": null
                },
                {
                    "status": "HUMAN_REVIEW_REQUIRED",
                    "source": {"ref_type": "report_renderer", "ref_id": format!("report:{run_id}")},
                    "affected_object": {"ref_type": "report", "ref_id": format!("report:{run_id}")},
                    "explanation": "Human professional review is required before any reliance.",
                    "human_review_required": true,
                    "human_acceptance_ref": null
                }
            ],
            "provenance_notes": [provenance.clone()],
            "user_supplied_values": [],
            "assumptions": [],
            "limitations": [
                {
                    "limitation_id": format!("limitation:{run_id}:technical-preview"),
                    "source": {"ref_type": "report_renderer", "ref_id": format!("report:{run_id}")},
                    "affected_scope": {"ref_type": "model", "ref_id": model_id},
                    "statement": "Output over invented user-local data; not validated engineering output.",
                    "effect": {
                        "mechanics_solve_qualified": true,
                        "user_rule_check_qualified": false,
                        "report_completeness": "qualified",
                        "human_review_required": true
                    },
                    "provenance": provenance.clone()
                }
            ],
            "unresolved_tbds": [],
            "professional_boundary": {
                "human_review_required": true,
                "software_makes_compliance_claim": false,
                "software_makes_certification_claim": false,
                "software_makes_sealing_claim": false,
                "software_makes_approval_claim": false,
                "software_makes_authentication_claim": false
            }
        });

        json!({
            "report_title": format!("{project_name} - Calculation Report"),
            "calculation_report": fixture["calculation_report"].clone(),
            "report_sections": report_sections,
            "result_rows": [report_row_from_result(first_result)]
        })
    }

    #[test]
    fn r2_from_blank_rehearsal_authors_solves_and_renders_report() {
        let rehearsal =
            read_fixture("r2_from_blank_rehearsal.json").expect("A12 rehearsal fixture loads");
        assert_eq!(
            rehearsal["boundary"]["protected_content_included"],
            json!(false)
        );
        assert_eq!(
            rehearsal["boundary"]["private_payload_included"],
            json!(false)
        );

        let (model, receipts) = apply_rehearsal_steps(&rehearsal);
        assert_eq!(
            receipts.len(),
            rehearsal["steps"].as_array().expect("steps").len()
        );
        let expected = &rehearsal["expected"];
        assert_eq!(
            count_array(&model, "nodes"),
            expected["nodes"].as_u64().unwrap() as usize
        );
        assert_eq!(
            count_array(&model, "pipe_segments"),
            expected["pipe_segments"].as_u64().unwrap() as usize
        );
        assert_eq!(
            count_array(&model, "supports"),
            expected["supports"].as_u64().unwrap() as usize
        );
        assert_eq!(
            count_array(&model, "materials"),
            expected["materials"].as_u64().unwrap() as usize
        );
        assert_eq!(
            count_array(&model, "sections"),
            expected["sections"].as_u64().unwrap() as usize
        );
        assert_eq!(
            count_array(&model, "load_cases"),
            expected["load_cases"].as_u64().unwrap() as usize
        );
        assert_eq!(
            count_array(&model, "combinations"),
            expected["combinations"].as_u64().unwrap() as usize
        );

        let solved = run_preview_mechanics_value(Some(model.clone()))
            .expect("from-blank authored model solves through backend mechanics");
        assert_eq!(solved["model_ref"], model["project"]["id"]);
        assert_eq!(
            solved["status"]["mechanics"], expected["mechanics_status"],
            "unexpected rehearsal solve status; diagnostics={:?}",
            solved["diagnostics"]
        );
        assert!(
            solved["results"].as_array().expect("result rows").len()
                >= expected["minimum_result_rows"].as_u64().unwrap() as usize
        );
        assert!(solved["results"]
            .as_array()
            .expect("result rows")
            .iter()
            .any(|row| row["basis_ref"]["ref_id"] == json!("combination:R2-C-100")));

        let report_input = rehearsal_report_input(&model, &solved);
        let rendered =
            render_calculation_report_value(report_input).expect("A7 renderer accepts rehearsal report");
        assert_eq!(
            rendered["export_blocked"],
            expected["report_export_blocked"]
        );
        assert_eq!(rendered["sha256_hex"].as_str().expect("hash").len(), 64);
        let html = rendered["html"].as_str().expect("rendered html");
        assert!(html.contains("R2 From Blank Rehearsal"));
        assert!(html.contains("Unit storage convention"));
        assert!(html.contains("entered_units_preserved"));
        assert!(html.contains("Model units"));
        assert!(html.contains("Result units"));
        assert!(html.contains("human_review_required"));
        assert!(!html.contains("<script"));
        assert!(!html.contains("code compliant"));
    }

    #[test]
    fn r2_from_blank_saved_project_opens_solves_and_renders_report() {
        let rehearsal =
            read_fixture("r2_from_blank_rehearsal.json").expect("A12 rehearsal fixture loads");
        let (model, receipts) = apply_rehearsal_steps(&rehearsal);
        assert_eq!(
            receipts.len(),
            rehearsal["steps"].as_array().expect("steps").len()
        );

        let mut connection = Connection::open_in_memory().expect("in-memory sqlite opens");
        let migration =
            apply_store_migrations(&connection).expect("project store migrations apply");
        assert!(fts5_available(&connection));

        let project_id = model["project"]["id"]
            .as_str()
            .expect("rehearsal project id");
        let project_name = model["project"]["name"]
            .as_str()
            .expect("rehearsal project name");
        upsert_project(
            &mut connection,
            project_id,
            project_name,
            &model,
            &json!([]),
            &Value::Null,
            &Value::Null,
            &Value::Null,
            &Value::Null,
            &Value::Null,
            &Value::Null,
        )
        .expect("A12 authored model saves to local project store");

        let loaded = load_project(&connection, Some(project_id))
            .expect("saved A12 project loads")
            .expect("saved A12 project exists");
        assert_eq!(loaded.project_id, project_id);
        assert_eq!(loaded.project_name, project_name);
        assert_eq!(loaded.model, model);
        assert_eq!(loaded.mechanics_result, Value::Null);
        assert_eq!(loaded.analysis_run, Value::Null);

        let solved = run_preview_mechanics_value(Some(loaded.model.clone()))
            .expect("saved authored model solves through backend mechanics");
        assert_eq!(solved["model_ref"], json!(project_id));
        assert_eq!(
            solved["status"]["mechanics"],
            rehearsal["expected"]["mechanics_status"]
        );
        assert!(
            solved["results"].as_array().expect("result rows").len()
                >= rehearsal["expected"]["minimum_result_rows"]
                    .as_u64()
                    .unwrap() as usize
        );

        let rendered = render_calculation_report_value(rehearsal_report_input(&loaded.model, &solved))
            .expect("saved-project report renders");
        assert_eq!(
            rendered["export_blocked"],
            rehearsal["expected"]["report_export_blocked"]
        );
        assert_eq!(rendered["sha256_hex"].as_str().expect("hash").len(), 64);
        let html = rendered["html"].as_str().expect("rendered html");
        assert!(html.contains("Unit storage convention"));
        assert!(html.contains("entered_units_preserved"));
        assert!(html.contains("Result units"));

        let summary = project_summary(
            loaded.project_id.clone(),
            loaded.project_name.clone(),
            &loaded.model,
            PathBuf::from(":memory:"),
            &migration,
            &json!([]),
            &Value::Null,
            &Value::Null,
            &solved,
            &json!({
                "analysis_run": {
                    "run_id": solved["run_id"].clone()
                }
            }),
            &Value::Null,
            &Value::Null,
            "Saved A12 authored project opened, solved, and rendered.".to_string(),
        );
        assert_eq!(summary.persisted_mechanics_result_count, 1);
        assert_eq!(
            summary.unit_round_trip_status,
            "unit_metadata_preserved_in_local_project_envelope"
        );
        assert!(summary.unit_round_trip_checked_ref_count > 0);
        assert_eq!(
            summary.persisted_analysis_run_ref,
            solved["run_id"].as_str().expect("run id")
        );
    }

    #[test]
    fn explicit_exact_model_round_trips_native_store_and_unit_metadata_unchanged() {
        let mut connection = Connection::open_in_memory().unwrap();
        apply_store_migrations(&connection).unwrap();
        let model: Value = serde_json::from_str(include_str!(
            "../../../../fixtures/model_operations/exact_pressure_authoring_model.json"
        )).unwrap();
        let project_id = model["project"]["id"].as_str().unwrap();
        let version = check_model_document_for_persist(&model).unwrap();
        assert_eq!(version.status, "current");
        // B7b: the exact form is a contract-present 0.7.0 document (3.0.0/exact_pressure_v3).
        assert_eq!(version.target_schema_version, "0.7.0");
        let prepared = model.clone();
        upsert_project(
            &mut connection, project_id, "Explicit exact-profile fixture", &prepared,
            &json!([]), &Value::Null, &Value::Null, &Value::Null, &Value::Null,
            &Value::Null, &Value::Null,
        ).unwrap();
        let loaded = load_project(&connection, Some(project_id)).unwrap().unwrap();
        assert_eq!(loaded.model, model);
        let reopened = evaluate_model_document(&loaded.model);
        assert_eq!(reopened.status, "current");
        assert_eq!(reopened.target_schema_version, "0.7.0");
        let (status, _, signature) = unit_round_trip_summary(&loaded.model);
        assert_eq!(status, "unit_metadata_preserved_in_local_project_envelope");
        assert!(signature.contains("poisson_ratio=1"));
        assert!(signature.contains("pressure_regions.region:fixture-pressure.pressure=kPa"));
        assert!(!signature.contains("shear_modulus"));
        for mode in [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny] {
            let request: LinearStaticPreviewRequest = serde_json::from_value(json!({
                "model": loaded.model.clone(), "materials": [],
            })).unwrap();
            let result = run_linear_static_preview_with_mode(request, mode);
            assert_eq!(result.status.mechanics, "MECHANICS_SOLVED", "{:?}", result.diagnostics);
            assert_eq!(result.producer.semantic_contract_id,
                "openpipestress.result_semantics/0.3.0/pressure-2");
            assert_eq!(result.contract_evidence.as_ref().unwrap()["exact_cases"].as_array().unwrap().len(), 2);
        }
    }

    fn stored_json_column(connection: &Connection, column: &str, project_id: &str) -> String {
        connection
            .query_row(
                &format!("SELECT {column} FROM local_projects WHERE project_id = ?1"),
                params![project_id],
                |row| row.get::<_, String>(0),
            )
            .unwrap()
    }

    /// T1 WP2, on the current format (B7b): a 0.7.0 exact-contract model with the
    /// load/reference-state group is saved and reopened exactly as any current
    /// document is, and its saved pressure-2 result keeps its raw hash and its
    /// resolved evidence bytes. The model and result are committed producer
    /// fixtures (invented data). The open step mirrors `open_local_project`.
    #[test]
    fn load_reference_model_and_saved_result_round_trip_native_store_byte_exactly() {
        let request: Value = serde_json::from_str(include_str!(
            "../../../../fixtures/product_preview/pressure/load_reference/connected.request.json"
        )).unwrap();
        let model = request["model"].clone();
        assert_eq!(model["schema_version"], "0.7.0");
        let result: Value = serde_json::from_str(include_str!(
            "../../../../fixtures/product_preview/pressure/load_reference/connected-sparse_interactive.raw.json"
        )).unwrap();
        assert_eq!(result["producer"]["semantic_contract_id"],
            "openpipestress.result_semantics/0.3.0/pressure-2");
        let project = model["project"]["id"].as_str().unwrap().to_string();
        let model_bytes = serde_json::to_string(&model).unwrap();
        let result_bytes = serde_json::to_string(&result).unwrap();
        let raw_result_hash = model_payload_hash(&result);
        let evidence_bytes =
            serde_json::to_string(&result["contract_evidence"]["load_reference_states"]).unwrap();
        let mh = computed_model_hash(&model, &project);
        let eh = computed_project_envelope_hash(
            &project, &model, &json!([]), &Value::Null, &Value::Null, &result, &Value::Null, &mh,
        );
        let root = std::env::temp_dir().join(format!(
            "ops-load-reference-{}-{}",
            std::process::id(),
            std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()
        ));
        std::fs::create_dir(&root).unwrap();
        let db = root.join("isolated.sqlite");
        let mut bridge = result.clone();
        let mut current = model.clone();
        for _ in 0..2 {
            let mut connection = Connection::open(&db).unwrap();
            apply_store_migrations(&connection).unwrap();
            let status = check_model_document_for_persist(&current).unwrap();
            assert_eq!(status.status, "current");
            assert_eq!(status.source_schema_version, "0.7.0");
            assert_eq!(status.target_schema_version, "0.7.0");
            let (persisted, saved_mh, saved_eh) = (current.clone(), mh.clone(), eh.clone());
            assert_eq!(persisted, model);
            upsert_project(
                &mut connection, &project, "Invented load-reference witness", &persisted, &json!([]),
                &Value::Null, &Value::Null, &bridge, &Value::Null, &saved_mh, &saved_eh,
            ).unwrap();
            connection.close().unwrap();

            let reopened = Connection::open(&db).unwrap();
            assert_eq!(stored_json_column(&reopened, "model_json", &project), model_bytes);
            assert_eq!(stored_json_column(&reopened, "mechanics_result_json", &project), result_bytes);
            let loaded = load_project(&reopened, Some(&project)).unwrap().unwrap();
            let status = evaluate_model_document(&loaded.model);
            assert_eq!(status.status, "current");
            assert_eq!(status.target_schema_version, "0.7.0");
            let opened = loaded.model;
            assert_eq!(opened, model);
            assert_eq!(serde_json::to_string(&opened).unwrap(), model_bytes);
            assert_eq!(model_payload_hash(&opened), hash_value_string(&mh));
            assert_eq!(loaded.model_hash, mh);
            assert_eq!(loaded.project_envelope_hash, eh);
            assert_eq!(model_payload_hash(&loaded.mechanics_result), raw_result_hash);
            assert_eq!(
                serde_json::to_string(&loaded.mechanics_result["contract_evidence"]["load_reference_states"]).unwrap(),
                evidence_bytes
            );
            bridge = loaded.mechanics_result;
            current = opened;
            reopened.close().unwrap();
        }
        std::fs::remove_dir_all(root).unwrap();

        // The reopened document is the product's own current input: it solves on
        // the exact route (pressure-2 with the group) and reproduces the saved evidence.
        let solved = run_linear_static_preview_with_mode(
            serde_json::from_value(json!({"model": current, "materials": []})).unwrap(),
            PreviewSolverMode::SparseInteractive,
        );
        assert_eq!(solved.producer.semantic_contract_id,
            "openpipestress.result_semantics/0.3.0/pressure-2");
        assert_eq!(
            solved.contract_evidence.as_ref().unwrap()["load_reference_states"],
            result["contract_evidence"]["load_reference_states"]
        );
    }

    /// T1 WP2 (D3), on the current format (B7b): exact-contract and pressure-free
    /// 0.7.0 documents persist unchanged with no load/reference-state key added; a
    /// pressure-free 0.7.0 document that carries the load/reference-state group
    /// (which requires the exact contract) is kept as authored (not stripped, not
    /// re-stamped) and the product blocks on solve.
    #[test]
    fn earlier_documents_persist_unchanged_and_carried_load_reference_keys_block_on_solve() {
        fn round_trip(model: &Value) -> (Value, ModelDocumentVersionStatus) {
            let mut connection = Connection::open_in_memory().unwrap();
            apply_store_migrations(&connection).unwrap();
            let project = model["project"]["id"].as_str().unwrap().to_string();
            let status = check_model_document_for_persist(model).unwrap();
            let persisted = model.clone();
            upsert_project(
                &mut connection, &project, "Invented earlier-version witness", &persisted, &json!([]),
                &Value::Null, &Value::Null, &Value::Null, &Value::Null, &Value::Null, &Value::Null,
            ).unwrap();
            assert_eq!(
                stored_json_column(&connection, "model_json", &project),
                serde_json::to_string(model).unwrap()
            );
            let loaded = load_project(&connection, Some(&project)).unwrap().unwrap();
            let reopened = evaluate_model_document(&loaded.model);
            assert_eq!(reopened.status, "current");
            (loaded.model, status)
        }
        fn carries_load_reference_keys(model: &Value) -> bool {
            model.get("reference_configurations").is_some()
                || model["materials"].as_array().is_some_and(|materials| {
                    materials.iter().any(|material| material.get("expansion_laws").is_some())
                })
                || model["load_cases"].as_array().is_some_and(|cases| {
                    cases.iter().any(|case| case.get("analysis_state").is_some())
                })
        }

        /// The document without its exact contract and the contract's own members
        /// (case pressure regions and section corrosion allowances): pressure-free.
        fn pressure_free(model: &Value) -> Value {
            let mut free = model.clone();
            free.as_object_mut().unwrap().remove("pressure_contract");
            for case in free["load_cases"].as_array_mut().unwrap() {
                case.as_object_mut().unwrap().remove("pressure_regions");
            }
            for pipe in free["pipe_segments"].as_array_mut().unwrap() {
                if let Some(section) = pipe.get_mut("section").and_then(Value::as_object_mut) {
                    section.remove("corrosion_allowance");
                }
            }
            free
        }

        let exact: Value = serde_json::from_str(include_str!(
            "../../../../fixtures/model_operations/exact_pressure_authoring_model.json"
        )).unwrap();
        assert_eq!(exact["schema_version"], open_pipe_stress_product_physics::MODEL_SCHEMA_VERSION);
        let free = pressure_free(&exact);
        for model in [&exact, &free] {
            assert!(!carries_load_reference_keys(model));
            let (loaded, status) = round_trip(model);
            assert_eq!(&loaded, model);
            assert_eq!(status.target_schema_version, model["schema_version"].as_str().unwrap());
            assert!(!carries_load_reference_keys(&loaded));
        }

        let request: Value = serde_json::from_str(include_str!(
            "../../../../fixtures/product_preview/pressure/load_reference/connected.request.json"
        )).unwrap();
        let carried = pressure_free(&request["model"]);
        assert_eq!(carried["schema_version"], "0.7.0");
        assert!(carries_load_reference_keys(&carried));
        let (loaded, status) = round_trip(&carried);
        assert_eq!(loaded, carried);
        assert_eq!(status.target_schema_version, "0.7.0");
        for mode in [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny] {
            let solved = run_linear_static_preview_with_mode(
                serde_json::from_value(json!({"model": loaded.clone(), "materials": []})).unwrap(),
                mode,
            );
            assert!(
                solved.diagnostics.iter().any(|diagnostic| diagnostic.severity == "blocking"
                    && diagnostic.code == "LOAD_STATE_CONTRACT_VERSION_MISMATCH"),
                "{:?}", solved.diagnostics
            );
            assert_ne!(solved.status.mechanics, "MECHANICS_SOLVED");
        }
    }

    #[test]
    fn local_project_store_uses_sqlite_fts5_and_round_trips_model_snapshot() {
        let mut connection = Connection::open_in_memory().expect("in-memory sqlite opens");
        let migration =
            apply_store_migrations(&connection).expect("project store migrations apply");
        assert!(fts5_available(&connection));

        let model = json!({
        "schema_version": "0.7.0",
        "document_kind": "openpipestress.product_preview.model",
            "project": {
                "id": "project:test-local",
                "name": "Test Local Project",
                "units": {
                    "length": "m",
                    "force": "N"
                }
            },
            "materials": [
                {
                    "id": "material:test",
                    "elastic_modulus": {
                        "value": 200000000000.0,
                        "unit": "Pa"
                    },
                    "shear_modulus": {
                        "value": 77000000000.0,
                        "unit": "Pa"
                    }
                }
            ],
            "pipe_segments": [
                {
                    "id": "pipe:test",
                    "section": {
                        "outside_diameter": {
                            "value": 0.1,
                            "unit": "m"
                        }
                    }
                }
            ],
            "load_cases": [
                {
                    "id": "load:test",
                    "primitive_loads": [
                        {
                            "id": "load:test:F",
                            "magnitude": {
                                "value": 1.0,
                                "unit": "N"
                            }
                        }
                    ]
                }
            ]
        });
        let editor_intents = json!([
            {
                "queue_id": "editor-intent-1",
                "operation_id": "op:editor-intent-material:invented-carbon-steel-elastic_modulus.value",
                "validation": {
                    "application_status": "not_applied"
                },
                "audit_boundary": {
                    "mutates_accepted_model_state": false
                },
                "professional_boundary": {
                    "software_makes_compliance_claim": false
                }
            }
        ]);
        let proposal = json!({
            "proposal_id": "proposal:physics-diagnostic-review",
            "operation": {
                "operation_id": "op:review-computed-diagnostic",
                "operation_status": "draft_user_review_required"
            },
            "validation": {
                "application_status": "not_applied"
            },
            "audit_boundary": {
                "mutates_accepted_model_state": false
            },
            "professional_boundary": {
                "software_makes_compliance_claim": false
            }
        });
        let selected_review_target = json!({
            "target_type": "result",
            "id": "result:stress:pipe-P-120:end-j:torsional-shear"
        });
        let mechanics_result = json!({
            "run_id": "run:preview-linear-static-001",
            "model_ref": "project:test-local",
            "results": [
                {
                    "id": "result:force:pipe-P-120:axial",
                    "kind": "element_local_axial_force"
                }
            ],
            "diagnostics": []
        });
        let analysis_run = json!({
            "deliverable_id": "DEL-14-02",
            "analysis_run": {
                "run_id": "run:preview-linear-static-001",
                "run_kind": "mechanics_solve",
                "analysis_status": ["HUMAN_REVIEW_REQUIRED", "MECHANICS_SOLVED"]
            }
        });
        let model_hash = json!({
            "algorithm": "sha256",
            "canonicalization": "rfc8785_jcs",
            "payload_scope": "model_payload",
            "payload_ref": "project:test-local",
            "value": "sha256:0123abcd0123abcd0123abcd0123abcd0123abcd0123abcd0123abcd0123abcd",
            "hash_status": "computed_local_preview"
        });
        let project_envelope_hash = json!({
            "algorithm": "sha256",
            "canonicalization": "rfc8785_jcs",
            "payload_scope": "project_envelope_payload",
            "payload_excludes": "storage_summary_and_envelope_hash_carrier_fields",
            "payload_ref": "project:test-local",
            "value": "sha256:4567ef014567ef014567ef014567ef014567ef014567ef014567ef014567ef01",
            "hash_status": "computed_local_preview"
        });
        upsert_project(
            &mut connection,
            "project:test-local",
            "Test Local Project",
            &model,
            &editor_intents,
            &proposal,
            &selected_review_target,
            &mechanics_result,
            &analysis_run,
            &model_hash,
            &project_envelope_hash,
        )
        .expect("project snapshot saves");

        let loaded = load_project(&connection, Some("project:test-local"))
            .expect("project snapshot loads")
            .expect("saved project exists");
        assert_eq!(loaded.project_id, "project:test-local");
        assert_eq!(loaded.project_name, "Test Local Project");
        assert_eq!(loaded.model, model);
        assert_eq!(loaded.editor_intents, editor_intents);
        assert_eq!(loaded.proposal, proposal);
        assert_eq!(loaded.selected_review_target, selected_review_target);
        assert_eq!(loaded.mechanics_result, mechanics_result);
        assert_eq!(loaded.analysis_run, analysis_run);
        assert_eq!(loaded.model_hash, model_hash);
        assert_eq!(loaded.project_envelope_hash, project_envelope_hash);
        assert_eq!(
            loaded.editor_intents[0]["validation"]["application_status"],
            json!("not_applied")
        );
        assert_eq!(
            loaded.editor_intents[0]["audit_boundary"]["mutates_accepted_model_state"],
            json!(false)
        );
        assert_eq!(
            loaded.editor_intents[0]["professional_boundary"]["software_makes_compliance_claim"],
            json!(false)
        );
        assert_eq!(
            loaded.proposal["operation"]["operation_id"],
            json!("op:review-computed-diagnostic")
        );
        assert_eq!(
            loaded.proposal["validation"]["application_status"],
            json!("not_applied")
        );
        assert_eq!(
            loaded.proposal["audit_boundary"]["mutates_accepted_model_state"],
            json!(false)
        );
        assert_eq!(
            loaded.selected_review_target["id"],
            json!("result:stress:pipe-P-120:end-j:torsional-shear")
        );
        assert_eq!(
            loaded.analysis_run["analysis_run"]["run_id"],
            json!("run:preview-linear-static-001")
        );
        let summary = project_summary(
            "project:test-local".to_string(),
            "Test Local Project".to_string(),
            &model,
            PathBuf::from(":memory:"),
            &migration,
            &editor_intents,
            &proposal,
            &selected_review_target,
            &mechanics_result,
            &analysis_run,
            &model_hash,
            &project_envelope_hash,
            "Saved test project snapshot.".to_string(),
        );
        assert_eq!(summary.editor_intent_count, 1);
        assert_eq!(summary.proposal_count, 1);
        assert_eq!(summary.selected_review_target_count, 1);
        assert_eq!(
            summary.selected_review_target_ref,
            "result: result:stress:pipe-P-120:end-j:torsional-shear"
        );
        assert_eq!(summary.persisted_mechanics_result_count, 1);
        assert_eq!(summary.persisted_analysis_run_count, 1);
        assert_eq!(
            summary.persisted_analysis_run_ref,
            "run:preview-linear-static-001"
        );
        assert_eq!(summary.persisted_model_hash_count, 1);
        assert_eq!(
            summary.persisted_model_hash_ref,
            "sha256:0123abcd0123abcd0123abcd0123abcd0123abcd0123abcd0123abcd0123abcd"
        );
        assert_eq!(summary.persisted_project_envelope_hash_count, 1);
        assert_eq!(
            summary.persisted_project_envelope_hash_ref,
            "sha256:4567ef014567ef014567ef014567ef014567ef014567ef014567ef014567ef01"
        );
        assert_eq!(
            summary.unit_round_trip_status,
            "unit_metadata_preserved_in_local_project_envelope"
        );
        assert_eq!(summary.unit_round_trip_checked_ref_count, 6);
        assert!(summary
            .unit_round_trip_signature
            .contains("project.units.length=m"));
        assert!(summary
            .unit_round_trip_signature
            .contains("load_cases.load:test.primitive_loads.load:test:F.magnitude=N"));
        let empty_summary = project_summary(
            "project:test-local".to_string(),
            "Test Local Project".to_string(),
            &model,
            PathBuf::from(":memory:"),
            &migration,
            &json!([]),
            &Value::Null,
            &Value::Null,
            &Value::Null,
            &Value::Null,
            &Value::Null,
            &Value::Null,
            "Saved empty test project snapshot.".to_string(),
        );
        assert_eq!(empty_summary.persisted_mechanics_result_count, 0);
        assert_eq!(empty_summary.persisted_analysis_run_count, 0);
        assert_eq!(empty_summary.persisted_analysis_run_ref, "not_persisted");
        assert_eq!(empty_summary.persisted_model_hash_count, 0);
        assert_eq!(empty_summary.persisted_model_hash_ref, "not_persisted");
        assert_eq!(empty_summary.persisted_project_envelope_hash_count, 0);
        assert_eq!(
            empty_summary.persisted_project_envelope_hash_ref,
            "not_persisted"
        );
        assert_eq!(
            empty_summary.unit_round_trip_signature,
            summary.unit_round_trip_signature
        );

        let indexed_count: i64 = connection
            .query_row(
                "SELECT count(*) FROM local_project_fts WHERE local_project_fts MATCH ?1",
                params!["editor"],
                |row| row.get(0),
            )
            .expect("fts5 query succeeds");
        assert_eq!(indexed_count, 1);
        let proposal_indexed_count: i64 = connection
            .query_row(
                "SELECT count(*) FROM local_project_fts WHERE local_project_fts MATCH ?1",
                params!["review"],
                |row| row.get(0),
            )
            .expect("fts5 proposal query succeeds");
        assert_eq!(proposal_indexed_count, 1);
        let target_indexed_count: i64 = connection
            .query_row(
                "SELECT count(*) FROM local_project_fts WHERE local_project_fts MATCH ?1",
                params!["torsional"],
                |row| row.get(0),
            )
            .expect("fts5 selected target query succeeds");
        assert_eq!(target_indexed_count, 1);
    }

    #[test]
    fn store_migration_ledger_reports_fresh_open_and_idempotent_reopen_evidence() {
        let connection = Connection::open_in_memory().expect("in-memory sqlite opens");
        let first_open = apply_store_migrations(&connection).expect("fresh store migrations apply");
        assert_eq!(first_open.migration_framework, STORE_MIGRATION_FRAMEWORK);
        assert_eq!(first_open.store_schema_version_before_open, 0);
        assert_eq!(first_open.store_schema_version, STORE_SCHEMA_TARGET_VERSION);
        assert_eq!(
            first_open.store_schema_target_version,
            STORE_SCHEMA_TARGET_VERSION
        );
        assert_eq!(first_open.pending_migration_count, 0);
        assert_eq!(
            first_open.migrations_applied_on_open,
            vec![
                "store-v1-base-project-and-fts-tables",
                "store-v2-editor-intents-column",
                "store-v3-proposal-column",
                "store-v4-selected-review-target-column",
                "store-v5-mechanics-result-column",
                "store-v6-analysis-run-column",
                "store-v7-model-hash-column",
                "store-v8-project-envelope-hash-column",
                "store-v9-model-document-migration-ledger-column",
                "store-v10-local-rule-packs-table",
                "store-v11-local-libraries-table",
                "store-v12-model-states-tables",
            ]
        );
        assert_eq!(
            first_open.migration_status,
            "migrated_on_open_store_schema_v0_to_v12"
        );

        let second_open =
            apply_store_migrations(&connection).expect("current store reopen succeeds");
        assert_eq!(
            second_open.store_schema_version_before_open,
            STORE_SCHEMA_TARGET_VERSION
        );
        assert_eq!(
            second_open.store_schema_version,
            STORE_SCHEMA_TARGET_VERSION
        );
        assert!(second_open.migrations_applied_on_open.is_empty());
        assert_eq!(second_open.pending_migration_count, 0);
        assert_eq!(
            second_open.migration_status,
            "current_store_schema_v12_no_pending_migrations"
        );
    }

    #[test]
    fn store_migration_ledger_reconciles_legacy_store_and_preserves_rows() {
        let connection = Connection::open_in_memory().expect("in-memory sqlite opens");
        connection
            .execute_batch(
                "
                CREATE TABLE local_projects (
                    project_id TEXT PRIMARY KEY,
                    project_name TEXT NOT NULL,
                    model_json TEXT NOT NULL,
                    created_at_unix INTEGER NOT NULL,
                    updated_at_unix INTEGER NOT NULL
                );
                INSERT INTO local_projects (
                    project_id, project_name, model_json, created_at_unix, updated_at_unix
                ) VALUES (
                    'project:legacy-local',
                    'Legacy Local Project',
                    '{\"project\":{\"id\":\"project:legacy-local\",\"name\":\"Legacy Local Project\"}}',
                    100,
                    100
                );
                ",
            )
            .expect("legacy pre-ledger store shape creates");
        assert_eq!(
            store_user_version(&connection).expect("user_version reads"),
            0
        );

        let migration = apply_store_migrations(&connection).expect("legacy store migrations apply");
        assert_eq!(migration.store_schema_version_before_open, 0);
        assert_eq!(migration.store_schema_version, STORE_SCHEMA_TARGET_VERSION);
        assert_eq!(migration.migrations_applied_on_open.len(), 12);
        assert_eq!(
            migration.migration_status,
            "migrated_on_open_store_schema_v0_to_v12"
        );

        let loaded = load_project(&connection, Some("project:legacy-local"))
            .expect("legacy project loads after migration")
            .expect("legacy project row preserved");
        assert_eq!(loaded.project_id, "project:legacy-local");
        assert_eq!(loaded.project_name, "Legacy Local Project");
        assert_eq!(loaded.editor_intents, json!([]));
        assert_eq!(loaded.proposal, Value::Null);
        assert_eq!(loaded.selected_review_target, Value::Null);
        assert_eq!(loaded.mechanics_result, Value::Null);
        assert_eq!(loaded.analysis_run, Value::Null);
        assert_eq!(loaded.model_hash, Value::Null);
        assert_eq!(loaded.project_envelope_hash, Value::Null);
    }

    #[test]
    fn store_migration_ledger_stages_v7_store_to_v8_applying_only_pending_migration() {
        let connection = Connection::open_in_memory().expect("in-memory sqlite opens");
        connection
            .execute_batch(
                "
                CREATE TABLE local_projects (
                    project_id TEXT PRIMARY KEY,
                    project_name TEXT NOT NULL,
                    model_json TEXT NOT NULL,
                    editor_intents_json TEXT NOT NULL DEFAULT '[]',
                    proposal_json TEXT NOT NULL DEFAULT 'null',
                    selected_review_target_json TEXT NOT NULL DEFAULT 'null',
                    mechanics_result_json TEXT NOT NULL DEFAULT 'null',
                    analysis_run_json TEXT NOT NULL DEFAULT 'null',
                    model_hash_json TEXT NOT NULL DEFAULT 'null',
                    created_at_unix INTEGER NOT NULL,
                    updated_at_unix INTEGER NOT NULL
                );
                CREATE VIRTUAL TABLE local_project_fts USING fts5(
                    project_id UNINDEXED,
                    project_name,
                    model_text
                );
                INSERT INTO local_projects (
                    project_id, project_name, model_json, created_at_unix, updated_at_unix
                ) VALUES (
                    'project:v7-local',
                    'V7 Local Project',
                    '{\"project\":{\"id\":\"project:v7-local\",\"name\":\"V7 Local Project\"}}',
                    100,
                    100
                );
                PRAGMA user_version = 7;
                ",
            )
            .expect("v7-shape store creates");
        assert_eq!(
            store_user_version(&connection).expect("user_version reads"),
            7
        );

        let migration = apply_store_migrations(&connection).expect("v7 store migrations apply");
        assert_eq!(migration.store_schema_version_before_open, 7);
        assert_eq!(migration.store_schema_version, STORE_SCHEMA_TARGET_VERSION);
        assert_eq!(
            migration.migrations_applied_on_open,
            vec![
                "store-v8-project-envelope-hash-column",
                "store-v9-model-document-migration-ledger-column",
                "store-v10-local-rule-packs-table",
                "store-v11-local-libraries-table",
                "store-v12-model-states-tables"
            ]
        );
        assert_eq!(
            migration.migration_status,
            "migrated_on_open_store_schema_v7_to_v12"
        );

        let loaded = load_project(&connection, Some("project:v7-local"))
            .expect("v7 project loads after staged migration")
            .expect("v7 project row preserved");
        assert_eq!(loaded.project_id, "project:v7-local");
        assert_eq!(loaded.model_hash, Value::Null);
        assert_eq!(loaded.project_envelope_hash, Value::Null);
    }

    #[test]
    fn local_project_store_lists_project_index_ordered_by_most_recent_update() {
        let mut connection = Connection::open_in_memory().expect("in-memory sqlite opens");
        apply_store_migrations(&connection).expect("project store migrations apply");
        assert_eq!(
            list_projects(&connection).expect("empty store lists").len(),
            0
        );

        let first_model = json!({
            "project": { "id": "project:test-alpha", "name": "Alpha Project" }
        });
        let second_model = json!({
            "project": { "id": "project:test-beta", "name": "Beta Project" }
        });
        upsert_project(
            &mut connection,
            "project:test-alpha",
            "Alpha Project",
            &first_model,
            &json!([]),
            &Value::Null,
            &Value::Null,
            &Value::Null,
            &Value::Null,
            &Value::Null,
            &Value::Null,
        )
        .expect("first project saves");
        upsert_project(
            &mut connection,
            "project:test-beta",
            "Beta Project",
            &second_model,
            &json!([]),
            &Value::Null,
            &Value::Null,
            &Value::Null,
            &Value::Null,
            &Value::Null,
            &Value::Null,
        )
        .expect("second project saves");
        connection
            .execute(
                "UPDATE local_projects SET updated_at_unix = updated_at_unix + 60 WHERE project_id = ?1",
                params!["project:test-beta"],
            )
            .expect("recency update applies");

        let entries = list_projects(&connection).expect("populated store lists");
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].project_id, "project:test-beta");
        assert_eq!(entries[0].project_name, "Beta Project");
        assert_eq!(entries[0].storage_mode, "local_sqlite");
        assert!(entries[0].created_at_unix > 0);
        assert!(entries[0].updated_at_unix >= entries[0].created_at_unix);
        assert_eq!(entries[1].project_id, "project:test-alpha");
        assert!(entries[0].updated_at_unix > entries[1].updated_at_unix);

        let opened_by_id = load_project(&connection, Some("project:test-alpha"))
            .expect("open by id loads")
            .expect("alpha project exists");
        assert_eq!(opened_by_id.project_id, "project:test-alpha");
        assert_eq!(opened_by_id.model, first_model);
        let opened_most_recent = load_project(&connection, None)
            .expect("open most recent loads")
            .expect("most recent project exists");
        assert_eq!(opened_most_recent.project_id, "project:test-beta");
    }

    #[test]
    fn agent_proposal_preserves_selected_review_target_without_mutation_or_claims() {
        let diagnostic_id = "diagnostic:combination:combination-C-OPER-ALT:result-stress-pipe-P-130:COMBINATION_STRESS_SUMMARY_SKIPPED";
        let result = json!({
            "model_ref": "project:invented-loop-01",
            "results": [
                {
                    "id": "result:force:pipe-P-120:axial",
                    "kind": "element_local_axial_force"
                }
            ],
            "diagnostics": [
                {
                    "id": diagnostic_id,
                    "severity": "warning"
                }
            ]
        });
        let output = build_sample_agent_proposal(
            result,
            Some(SelectedReviewTarget {
                target_type: "diagnostic".to_string(),
                id: diagnostic_id.to_string(),
            }),
        );
        let proposal = output.get("proposal").expect("proposal envelope exists");
        assert_eq!(
            proposal["operation"]["affected_entity_ids"][0],
            json!(diagnostic_id)
        );
        assert_eq!(
            proposal["operation"]["changes"][0]["target_ref"],
            json!(diagnostic_id)
        );
        assert_eq!(
            proposal["operation"]["operation_status"],
            json!("draft_user_review_required")
        );
        assert_eq!(
            proposal["validation"]["application_status"],
            json!("not_applied")
        );
        assert!(proposal["rationale"]
            .as_str()
            .expect("rationale is text")
            .contains(&format!("selected review reference is {diagnostic_id}")));
        assert_eq!(output["accepted_model_mutated"], json!(false));
        assert_eq!(
            proposal["audit_boundary"]["mutates_accepted_model_state"],
            json!(false)
        );
        assert_eq!(
            proposal["professional_boundary"]["software_makes_compliance_claim"],
            json!(false)
        );
        assert_eq!(
            proposal["professional_boundary"]["software_makes_approval_claim"],
            json!(false)
        );
    }

    /// b7-plan R1, "Exact bytes to the desktop": for the G10 milestone model, the solve
    /// command and the job hand the desktop the producer's exact bytes (PP's pins in
    /// `core/product_physics/tests/g10_desktop_milestone_model.rs`), the committed copies
    /// the desktop's tests replay are those bytes, and the native save writes received
    /// bytes unchanged. With `G10_DESKTOP_RESULT_OUT` set, writes the copies there.
    #[test]
    fn solves_hand_the_desktop_the_producers_exact_bytes() {
        const PINS: [(&str, usize, &str); 2] = [
            ("sparse_interactive", 80884, "d599dd9a3f6482743acc9076de157796ec41e3c86335884835b9c56f42cafae3"),
            ("dense_scrutiny", 80908, "cd9812a90e8f85ad050456aa26621bf17cf7b0ca9c64c4d2e215292b115137a2"),
        ];
        let fixtures = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../fixtures/product_preview");
        let model: Value = serde_json::from_str(include_str!(
            "../../../../fixtures/product_preview/rf_skew_t_cant_off_122_r1e-04.desktop_model.json"
        ))
        .unwrap();
        for (mode, len, digest) in PINS {
            let bytes = solve_preview_mechanics_with_mode(model.clone(), PreviewSolverMode::from_wire(mode).unwrap()).unwrap();
            let sha = open_pipe_stress_operation_applier::sha256_hex(&bytes);
            assert_eq!((bytes.len(), sha.as_str()), (len, digest), "{mode}: the ordinary solve's bytes are the producer's");
            // The job path (raw ingress, capped worker process, take) hands over the same bytes.
            let (status, taken) = solve_jobs::test_support::run_job(&model, Some(mode));
            assert_eq!(status.state, "completed", "{mode}");
            assert_eq!(taken.as_deref(), Some(bytes.as_str()), "{mode}: the job's bytes");
            let copy = fixtures.join(format!("rf_skew_t_cant_off_122_r1e-04.desktop_result_{mode}.json"));
            if let Ok(dir) = std::env::var("G10_DESKTOP_RESULT_OUT") {
                fs::write(std::path::Path::new(&dir).join(copy.file_name().unwrap()), &bytes).unwrap();
                continue;
            }
            assert_eq!(fs::read_to_string(&copy).unwrap(), bytes, "{mode}: the desktop tests' copy");
            // The native save writes the received bytes, byte for byte.
            let root = std::env::temp_dir().join(format!("ops-g10-exact-bytes-{}-{mode}", std::process::id()));
            let _ = fs::remove_dir_all(&root);
            fs::create_dir(&root).unwrap();
            let href: String = bytes.bytes().map(|b| format!("%{b:02X}")).collect();
            let request: native_result_download::SaveRequest = serde_json::from_value(json!({
                "href": format!("data:application/json;charset=utf-8,{href}"), "file_name": "openpipestress-preview-results-g10.json",
                "screening": {"route_id":"DOTH-JSON-001","export_context":"local_private","explicit_local_private_intent":true,"blocked":false,
                    "materialization_withheld":false,"lossless_required":true,"exact_payload_match":true,"blocking_count":0},
                "local_first": {"route_id":"DOTH-JSON-001","export_context":"local_private","storage_context":"local_private","action":"include_metadata_only",
                    "reason_code":"PRIVATE_LOCAL_METADATA_ALLOWED","blocked":false,"metadata_only":true,"explicit_local_private_intent":true}
            })).unwrap();
            let saved = native_result_download::save_request(request, || Ok::<_, ()>(root.clone())).unwrap_or_else(|e| panic!("{mode}: {}", e.code));
            let written = fs::read(root.join(&saved.file_name)).unwrap();
            fs::remove_dir_all(&root).unwrap();
            assert_eq!(written, bytes.as_bytes(), "{mode}: the saved bytes are the received bytes");
        }
    }

    #[test]
    fn solve_job_seam_completes_and_reports_result_without_cancellation() {
        let (status, taken) = solve_jobs::test_support::run_job(&derived_invented_model(), None);
        assert_eq!(status.state, "completed");
        assert!(!status.cancellation_requested);
        assert_eq!(status.cancellation_status, "not_requested");
        assert_eq!(status.cancellation_scope, "worker_process_terminated");
        assert_eq!(status.resource.route_pricing, "ordinary_worker_capped");
        let result: Value = serde_json::from_str(&taken.expect("completed job publishes a result")).unwrap();
        assert!(!result["results"].as_array().expect("result rows present").is_empty());
        assert_eq!(status.error_message, None);
        // The default payload is the bundled demo, which solves.
        assert_bundled_demo_solved(
            &solve_preview_mechanics(load_preview_model().expect("bundled demo loads"))
                .expect("bundled demo solves"),
        );
    }

    #[test]
    fn run_preview_mechanics_uses_supplied_model_payload() {
        // A solve always carries a model; there is no implicit model.
        let refused = run_preview_mechanics_value(None).expect_err("a solve without a model is refused");
        assert!(refused.starts_with("PREVIEW-SOLVE-MODEL-REQUIRED"), "{refused}");
        assert_bundled_demo_solved(
            &run_preview_mechanics_value(Some(load_preview_model().expect("bundled demo loads"))).expect("bundled demo solves"),
        );
        let mut model = derived_invented_model();
        model["project"]["id"] = json!("project:edited-solve-command");
        model["materials"][0]["elastic_modulus"]["value"] = json!(195000000000.0);

        let solved = run_preview_mechanics_value(Some(model)).expect("supplied model solves");

        assert_eq!(solved["model_ref"], json!("project:edited-solve-command"));
        assert_eq!(solved["status"]["mechanics"], json!("MECHANICS_SOLVED"));
        assert!(
            solved["results"]
                .as_array()
                .expect("result rows present")
                .len()
                > 0
        );
    }

    /// U3 (D-2 A1), on the current format (B7b): a 0.7.0 document declaring the
    /// retired 1.0.0/legacy_pressure_v1 or 2.0.0/exact_straight_pressure_v2 contract
    /// opens unchanged (no migration rewrites it), and its solve is refused with
    /// PRESSURE_MODEL_REAUTHOR_REQUIRED; nothing is published.
    #[test]
    fn retired_legacy_pressure_label_opens_unchanged_and_its_solve_is_refused() {
        for contract in [
            json!({"version": "1.0.0", "mode": "legacy_pressure_v1"}),
            json!({"version": "2.0.0", "mode": "exact_straight_pressure_v2"}),
        ] {
            let mut labelled: Value = serde_json::from_str(include_str!(
                "../../../../fixtures/model_operations/exact_pressure_authoring_model.json"
            ))
            .unwrap();
            labelled["pressure_contract"] = contract.clone();
            for case in labelled["load_cases"].as_array_mut().unwrap() {
                case.as_object_mut().unwrap().remove("pressure_regions");
            }
            let before = labelled.clone();
            let opened = evaluate_model_document(&labelled);
            assert_eq!(opened.status, "current", "{contract}");
            assert_eq!(opened.target_schema_version, "0.7.0", "{contract}");
            assert_eq!(labelled, before);

            let solved = run_preview_mechanics_value(Some(labelled)).expect("refusal is a published envelope");
            assert_eq!(solved["status"]["mechanics"], json!("MODEL_INCOMPLETE"), "{contract}");
            assert!(solved["results"].as_array().expect("result rows").is_empty(), "{contract}");
            assert!(solved["diagnostics"].as_array().expect("diagnostics").iter().any(|d| {
                d["code"] == json!("PRESSURE_MODEL_REAUTHOR_REQUIRED")
                    && d["severity"] == json!("blocking")
                    && d["affected_refs"] == json!(["pressure_contract"])
            }), "{contract}: {}", solved["diagnostics"]);
        }
    }

    #[test]
    fn run_preview_mechanics_reports_blank_model_incomplete_without_defaults() {
        let blank = json!({
            "schema_version": "0.7.0",
            "document_kind": "openpipestress.product_preview.model",
            "data_boundary": {
                "public_examples_policy": "blank_user_created_local_document_no_bundled_engineering_values",
                "protected_source_policy": "no_protected_standards_content_inserted",
                "private_data_policy": "local_user_document_not_committed_to_repository",
                "professional_boundary": "human_review_required_no_software_approval_claim"
            },
            "project": {
                "id": "project:blank-local-rust-test",
                "name": "Blank Local Model",
                "description": "User-created local blank model document.",
                "units": {"length": "m", "force": "N"}
            },
            "analysis_status": {
                "mechanics": "MODEL_INCOMPLETE",
                "rule_check": "RULE_INPUTS_INCOMPLETE",
                "professional_acceptance": "NOT_PROVIDED"
            },
            "materials": [],
            "nodes": [],
            "pipe_segments": [],
            "supports": [],
            "components": [],
            "load_cases": [],
            "combinations": [],
            "diagnostics": []
        });

        let solved = run_preview_mechanics_value(Some(blank))
            .expect("blank model produces an honest result envelope");
        let diagnostics = solved["diagnostics"]
            .as_array()
            .expect("diagnostics should be present");

        assert_eq!(solved["model_ref"], json!("project:blank-local-rust-test"));
        assert_eq!(solved["status"]["mechanics"], json!("MODEL_INCOMPLETE"));
        assert!(solved["results"]
            .as_array()
            .expect("result rows array present")
            .is_empty());
        assert!(diagnostics
            .iter()
            .any(|item| item["code"] == json!("NODE_INPUT_MISSING")));
        assert!(diagnostics
            .iter()
            .any(|item| item["code"] == json!("PIPE_INPUT_MISSING")));
        assert!(diagnostics
            .iter()
            .any(|item| item["code"] == json!("LOAD_INPUT_MISSING")));
    }

    #[test]
    fn solve_job_seam_uses_supplied_model_payload() {
        let mut model = derived_invented_model();
        model["project"]["id"] = json!("project:edited-solve-job");
        model["materials"][0]["elastic_modulus"]["value"] = json!(195000000000.0);

        let (status, taken) = solve_jobs::test_support::run_job(&model, None);
        assert_eq!(status.state, "completed");
        let result: Value = serde_json::from_str(&taken.expect("completed job publishes a result")).unwrap();
        assert_eq!(result["model_ref"], json!("project:edited-solve-job"));
        assert_eq!(result["status"]["mechanics"], json!("MECHANICS_SOLVED"));
        assert!(!result["results"].as_array().expect("result rows present").is_empty());
        assert_eq!(status.error_message, None);
    }

    #[test]
    fn solve_job_cancelled_before_execution_never_starts_the_solver() {
        let registry = solve_jobs::test_support::real_registry(memory_pool::test_host::ScriptedHost::normal());
        // Other reservations fill the pool, so the job stays queued.
        let held = hold_the_pool(&registry);
        let body = serde_json::to_vec(&load_preview_model().expect("bundled demo loads")).unwrap();
        let receipt = registry.start(&body, None).expect("job starts");
        assert_eq!(receipt.state, "queued");
        assert!(receipt.backend_cancellation_token.starts_with("solve-cancel-token-"));

        let cancel = registry.cancel(&receipt.job_id, &receipt.backend_cancellation_token).expect("cancel receipt");
        assert!(cancel.accepted);
        assert_eq!(cancel.cancellation_status, "cancellation_requested_awaiting_cooperative_checkpoint");
        assert!(!cancel.cancellation_success_claimed);

        let status = solve_jobs::test_support::wait_terminal(&registry, &receipt.job_id);
        drop(held);
        assert_eq!(status.state, "cancelled");
        assert!(status.cancellation_requested);
        assert_eq!(status.cancellation_status, "cancelled_before_solver_start");
        assert!(!status.result_available);
        assert_eq!(registry.pool().reserved_total(), 0, "the input lease dropped with the worker");
    }

    #[test]
    fn solve_job_cancellation_after_completion_is_rejected_without_success_claim() {
        let registry = solve_jobs::test_support::real_registry(memory_pool::test_host::ScriptedHost::normal());
        let receipt = registry.start(&serde_json::to_vec(&derived_invented_model()).unwrap(), None).expect("job starts");
        assert_eq!(solve_jobs::test_support::wait_terminal(&registry, &receipt.job_id).state, "completed");

        let cancel = registry.cancel(&receipt.job_id, &receipt.backend_cancellation_token).expect("cancel receipt");
        assert!(!cancel.accepted);
        assert_eq!(cancel.cancellation_status, "request_after_terminal_state_completed_no_cancellation_performed");
        assert_eq!(cancel.job_state, "completed");
        assert!(!cancel.cancellation_success_claimed);

        let status = registry.status(&receipt.job_id).expect("status available");
        assert_eq!(status.state, "completed");
        assert!(status.result_available);
    }

    #[test]
    fn solve_job_cancellation_with_invalid_token_is_rejected_and_does_not_flag_the_job() {
        let registry = solve_jobs::test_support::real_registry(memory_pool::test_host::ScriptedHost::normal());
        let held = hold_the_pool(&registry);
        let receipt = registry.start(&serde_json::to_vec(&derived_invented_model()).unwrap(), None).expect("job starts");

        let cancel = registry.cancel(&receipt.job_id, "wrong-token").expect("cancel receipt");
        assert!(!cancel.accepted);
        assert_eq!(cancel.cancellation_status, "rejected_invalid_cancellation_token");

        let status = registry.status(&receipt.job_id).expect("status available");
        assert!(!status.cancellation_requested);
        assert_eq!(status.cancellation_status, "not_requested");

        assert!(registry.status("backend-solve-job-missing").is_err());
        assert!(registry.cancel("backend-solve-job-missing", "token").is_err());
        drop(held);
        assert_eq!(solve_jobs::test_support::wait_terminal(&registry, &receipt.job_id).state, "completed");
    }

    #[test]
    fn solve_job_with_invalid_model_payload_fails_with_recorded_error() {
        let (status, taken) = solve_jobs::test_support::run_job(&json!({"not_a_preview_model": true}), None);
        assert_eq!(status.state, "failed");
        assert_eq!(taken, None);
        assert!(!status.result_available);
        assert!(status.error_message.is_some());
    }

    /// Reserves the pool but for room for a start's ingress lease (and not
    /// for a worker grant), so a job stays queued until the lease drops.
    fn hold_the_pool(registry: &solve_jobs::SolveJobRegistry) -> memory_pool::Lease {
        let pool_bytes = registry.pool_status().pool_bytes;
        registry.pool().acquire(pool_bytes - (1 << 20), memory_pool::LeasePurpose::Ingress).expect("the pool is free")
    }

    // PP's derived pressure-free, joint-free invented model. The bundled default
    // (`invented_demo_model.json`) is the same model under its own project identity.
    fn derived_invented_model() -> Value {
        serde_json::from_str(include_str!(
            "../../../../core/product_physics/tests/fixtures/preview_physics_invented_model.json"
        ))
        .expect("derived invented model parses")
    }

    fn assert_bundled_demo_solved(solved: &Value) {
        assert_eq!(solved["model_ref"], json!("project:invented-demo-loop-01"));
        assert_eq!(solved["status"]["mechanics"], json!("MECHANICS_SOLVED"));
        // DEL-04-01 kd5-nonlinear-active-set.md §5: the invented demo's converged
        // nonlinear-support cases are confirmed and published CHECKS_PASSED (two
        // info diagnostics, each with the confirmation's evidence line).
        assert_eq!(solved["numerical_quality"]["status"], json!("checks_passed"));
        let diagnostics = solved["diagnostics"].as_array().expect("diagnostics");
        assert_eq!(
            diagnostics
                .iter()
                .filter(|d| d["code"] == json!("NUMERICAL_INTEGRITY_CHECKS_PASSED")
                    && d["message"].as_str().is_some_and(|m| m.contains("Converged support state: established at 128-bit precision")))
                .count(),
            2
        );
        assert!(!diagnostics.iter().any(|d| d["code"] == json!("NUMERICAL_INTEGRITY_SENSITIVE")));
        assert!(!diagnostics.iter().any(|d| d["severity"] == json!("blocking")));
        assert!(!solved["results"].as_array().expect("result rows").is_empty());
    }

    /// U3 (ROOT, I114 round 2): the app's default session model solves out of the
    /// box, in both solver modes, through the native load and default-solve seams.
    #[test]
    fn default_session_model_solves_in_both_modes() {
        let model = load_preview_model().expect("default session model loads");
        assert_eq!(model["project"]["id"], json!("project:invented-demo-loop-01"));
        assert_eq!(resolve_solve_model_payload(Some(model.clone())).expect("supplied payload"), model);
        assert!(resolve_solve_model_payload(None).is_err(), "no implicit model");
        for mode in [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny] {
            let solved = solve_preview_mechanics_value_with_mode(model.clone(), mode).expect("default model solves");
            assert_bundled_demo_solved(&solved);
            assert_eq!(
                solved["producer"]["semantic_contract_id"],
                json!("openpipestress.result_semantics/0.3.0/preview-physics-2")
            );
        }
        assert_bundled_demo_solved(&run_preview_mechanics_value(Some(model)).expect("default solve command"));
    }

    fn fixture_inspector_intent(before: &str, after: &str) -> Value {
        json!({
            "operation_id": "op:editor-intent-material-elastic-modulus",
            "operation_kind": "modify",
            "operation_status": "proposed",
            "author_type": "user",
            "target": { "object_type": "Material", "ref": "material:invented-carbon-steel" },
            "change": {
                "change_id": "change:material-elastic-modulus",
                "change_kind": "set_field",
                "field_label": "Elastic modulus",
                "field_path": "elastic_modulus.value",
                "before": before,
                "after": after,
                "unit": "Pa",
                "dimension": "stress",
                "source_note": "unit metadata required"
            },
            "validation": {
                "schema_validation": "not_run",
                "constraint_validation": "not_run",
                "unit_validation": "not_run",
                "diff_preview_status": "not_generated",
                "application_status": "not_applied"
            },
            "audit_boundary": {
                "mutation_route": "structured_operations_only",
                "direct_model_mutation_allowed": false,
                "requires_user_acceptance": true,
                "mutates_accepted_model_state": false
            },
            "professional_boundary": {
                "human_review_required": true,
                "software_makes_compliance_claim": false,
                "software_makes_certification_claim": false,
                "software_makes_sealing_claim": false,
                "software_makes_approval_claim": false,
                "software_makes_authentication_claim": false
            },
            "rationale": "command-path test edit"
        })
    }

    fn fixture_load_magnitude_intent(before: &str, after: &str) -> Value {
        json!({
            "operation_id": "op:test-load-L-100-Y-magnitude",
            "operation_kind": "modify",
            "operation_status": "proposed",
            "author_type": "user",
            "target": { "object_type": "Load", "ref": "load:L-100" },
            "change": {
                "change_id": "change:test-load-L-100-Y-magnitude",
                "change_kind": "update_load",
                "field_label": "load:L-100-Y magnitude",
                "field_path": "primitive_loads.1.magnitude.value",
                "before": before,
                "after": after,
                "unit": "N",
                "dimension": "force",
                "source_note": "explicit load data persistence/solve regression"
            },
            "validation": {
                "schema_validation": "not_run",
                "constraint_validation": "not_run",
                "unit_validation": "not_run",
                "diff_preview_status": "not_generated",
                "application_status": "not_applied"
            },
            "audit_boundary": {
                "mutation_route": "structured_operations_only",
                "direct_model_mutation_allowed": false,
                "requires_user_acceptance": true,
                "mutates_accepted_model_state": false
            },
            "professional_boundary": {
                "human_review_required": true,
                "software_makes_compliance_claim": false,
                "software_makes_certification_claim": false,
                "software_makes_sealing_claim": false,
                "software_makes_approval_claim": false,
                "software_makes_authentication_claim": false
            },
            "rationale": "saved edited load data solve regression"
        })
    }

    fn result_value(result: &Value, result_id: &str) -> f64 {
        result["results"]
            .as_array()
            .expect("result rows present")
            .iter()
            .find(|item| item["id"].as_str() == Some(result_id))
            .and_then(|item| item["value"].as_f64())
            .unwrap_or_else(|| panic!("missing numeric result row {result_id}"))
    }

    #[test]
    fn apply_model_operation_command_applies_inspector_intent_to_bundled_fixture_model() {
        let model =
            read_fixture("invented_demo_model.json").expect("bundled preview model loads");
        let intent = fixture_inspector_intent("200000000000", "195000000000");
        let outcome =
            apply_model_operation_value(model.clone(), intent, None).expect("command returns outcome");

        assert_eq!(
            outcome["validation"]["application_status"],
            "applied_to_session_model"
        );
        assert_eq!(
            outcome["acceptance"]["acceptance_basis"],
            "user_initiated_apply_in_local_session"
        );
        assert_eq!(
            outcome["acceptance"]["acceptance_is_professional_approval"],
            json!(false)
        );
        assert_eq!(
            outcome["applied_model"]["materials"][0]["elastic_modulus"]["value"],
            json!(195000000000.0)
        );
        // The input model the command received is untouched; the applied model
        // is a new document, and downstream solve still accepts it.
        assert_eq!(
            model["materials"][0]["elastic_modulus"]["value"],
            json!(200000000000_i64)
        );
        // The edited bundled demo still solves through the preview mechanics path.
        let solved = run_preview_mechanics_value(Some(outcome["applied_model"].clone()))
            .expect("edited model still solves through the preview mechanics path");
        assert_eq!(solved["status"]["mechanics"], json!("MECHANICS_SOLVED"));
        assert!(solved["results"]
            .as_array()
            .map(|rows| !rows.is_empty())
            .unwrap_or(false));
    }

    #[test]
    fn saved_edited_load_model_round_trips_and_solves_from_restored_payload() {
        let mut connection = Connection::open_in_memory().expect("in-memory sqlite opens");
        apply_store_migrations(&connection).expect("store migrations apply");

        let mut model = derived_invented_model();
        let baseline = solve_preview_mechanics(model.clone()).expect("baseline fixture model solves");
        assert_eq!(baseline["status"]["mechanics"], json!("MECHANICS_SOLVED"));
        model["project"]["id"] = json!("project:edited-load-roundtrip");
        model["project"]["name"] = json!("Edited Load Roundtrip");

        let outcome =
            apply_model_operation_value(model, fixture_load_magnitude_intent("350", "425"), None)
                .expect("load magnitude operation applies");
        assert_eq!(
            outcome["validation"]["application_status"],
            json!("applied_to_session_model")
        );
        assert_eq!(
            outcome["acceptance"]["persistence_status"],
            json!("session_state_only_not_yet_saved")
        );
        let edited_model = outcome["applied_model"].clone();
        assert_eq!(
            edited_model["load_cases"][0]["primitive_loads"][1]["magnitude"]["value"].as_f64(),
            Some(425.0)
        );

        let received_model_claim = json!({
            "algorithm": "sha512",
            "canonicalization": "historical_mislabel",
            "value": "raw-received-model-claim"
        });
        let received_envelope_claim = json!(["raw", "received", "envelope", "claim"]);
        // The derived fixture is a current (0.7.0) document: it persists exactly
        // as edited, with the received hash claims kept raw; nothing is migrated
        // and no evidence is minted.
        let document_status = check_model_document_for_persist(&edited_model)
            .expect("edited current model document is ready to persist");
        assert_eq!(document_status.status, "current");
        assert_eq!(
            edited_model["schema_version"],
            json!(open_pipe_stress_product_physics::MODEL_SCHEMA_VERSION)
        );
        let (model_to_persist, model_hash, envelope_hash) =
            (edited_model, received_model_claim, received_envelope_claim);

        upsert_project(
            &mut connection,
            "project:edited-load-roundtrip",
            "Edited Load Roundtrip",
            &model_to_persist,
            &json!([]),
            &Value::Null,
            &Value::Null,
            &Value::Null,
            &Value::Null,
            &model_hash,
            &envelope_hash,
        )
        .expect("edited load model persists to the local store");

        let restored = load_project(&connection, Some("project:edited-load-roundtrip"))
            .expect("store read succeeds")
            .expect("edited project row exists");
        assert_eq!(restored.project_id, "project:edited-load-roundtrip");
        assert_eq!(
            restored.model["load_cases"][0]["primitive_loads"][1]["magnitude"]["value"].as_f64(),
            Some(425.0)
        );
        assert_eq!(restored.model_hash, model_hash);

        let restored_solve =
            solve_preview_mechanics(restored.model).expect("restored edited model solves");
        assert_eq!(
            restored_solve["model_ref"],
            json!("project:edited-load-roundtrip")
        );
        assert_eq!(
            restored_solve["status"]["mechanics"],
            json!("MECHANICS_SOLVED")
        );
        assert!(
            restored_solve["results"]
                .as_array()
                .expect("result rows present")
                .len()
                > 0
        );
        assert!(result_value(&restored_solve, "result:disp:node-N-140").is_finite());
    }

    #[test]
    fn persist_preparation_refuses_retired_newer_and_unsupported_model_documents() {
        for version in ["0.1.0", "0.2.0", "0.5.0", "0.6.0"] {
            let retired = json!({ "schema_version": version, "project": { "id": "project:retired", "name": "Retired" } });
            let error = check_model_document_for_persist(&retired)
                .expect_err("retired documents must be refused by name");
            assert!(error.contains("retired_schema") && error.contains("MODEL_SCHEMA_VERSION_RETIRED"), "{error}");
            assert!(error.contains(version) && error.contains("re-author"), "{error}");
        }
        let newer = json!({ "schema_version": "0.7.1", "project": { "id": "project:newer", "name": "Newer" } });
        let error = check_model_document_for_persist(&newer)
            .expect_err("newer-than-supported documents must be refused");
        assert!(error.contains("newer_than_supported"), "{error}");

        let unsupported =
            json!({ "project": { "id": "project:unversioned", "name": "Unversioned" } });
        let error = check_model_document_for_persist(&unsupported)
            .expect_err("documents without a valid schema_version must be refused");
        assert!(error.contains("unsupported_schema"), "{error}");
    }

    #[test]
    fn persist_preparation_keeps_current_documents_unchanged() {
        let model = json!({ "schema_version": open_pipe_stress_product_physics::MODEL_SCHEMA_VERSION, "project": { "id": "project:current", "name": "Current" } });
        let status = check_model_document_for_persist(&model).expect("current documents persist");
        assert_eq!(status.status, "current");
        assert_eq!(status.source_schema_version, "0.7.0");
        assert_eq!(status.target_schema_version, "0.7.0");
    }

    /// B7a: a project stored as a retired version (here a 0.1.0 row a previous
    /// build wrote) is refused by name on open and on save; its stored bytes are
    /// never rewritten, and no migration evidence is written.
    #[test]
    fn stored_retired_documents_are_refused_on_open_and_save_without_rewrite() {
        let root = std::env::temp_dir().join(format!(
            "ops-retired-model-{}-{}",
            std::process::id(),
            std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()
        ));
        std::fs::create_dir(&root).unwrap();
        let db = root.join("isolated.sqlite");
        let stored_model = json!({ "schema_version": "0.1.0", "document_kind": "openpipestress.product_preview.model", "project": { "id": "project:retired", "name": "Retired" } });
        {
            let (mut connection, _) = open_project_store(&db).unwrap();
            upsert_project(&mut connection, "project:retired", "Retired", &stored_model, &json!([]),
                &Value::Null, &Value::Null, &Value::Null, &Value::Null, &Value::Null, &Value::Null).unwrap();
        }
        let (connection, _) = open_project_store(&db).unwrap();
        let record = load_project(&connection, Some("project:retired")).unwrap().unwrap();
        let status = evaluate_model_document(&record.model);
        assert!(status.is_refused());
        let refusal = refuse_model_document(&status);
        assert!(refusal.contains("retired_schema") && refusal.contains("re-author"), "{refusal}");
        drop(connection);
        let error = save_project_in_store(db.clone(), SaveLocalProjectRequest::from_value(json!({
            "project_id": "project:retired", "project_name": "Retired", "model": stored_model.clone(),
            "expected_base": null
        })).unwrap()).expect_err("a retired document is not saved");
        assert!(error.contains("retired_schema"), "{error}");
        let (connection, _) = open_project_store(&db).unwrap();
        let after = load_project(&connection, Some("project:retired")).unwrap().unwrap();
        assert_eq!(after.model, stored_model, "the stored bytes are never rewritten");
        drop(connection);
        std::fs::remove_dir_all(root).unwrap();
    }

    /// W-13 §5.3 at project load: a stored document carrying every model-data
    /// slot opens exactly as stored with its model hash; a stored document with
    /// a malformed slot is refused by name and its stored bytes are untouched.
    #[test]
    fn project_load_validates_model_data_slots() {
        let root = std::env::temp_dir().join(format!(
            "ops-slot-model-{}-{}",
            std::process::id(),
            std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()
        ));
        std::fs::create_dir(&root).unwrap();
        let db = root.join("isolated.sqlite");
        let every_slot: Value = serde_json::from_str(include_str!(
            "../../../../fixtures/model_document/invented_every_slot_model.json"
        )).unwrap();
        let mut malformed = every_slot.clone();
        malformed["project"]["id"] = json!("project:slots-malformed");
        malformed["constraints"][0]["constraint_kind"] = json!("equipment_interface");
        {
            let (mut connection, _) = open_project_store(&db).unwrap();
            for (id, model) in [("project:slots", &every_slot), ("project:slots-malformed", &malformed)] {
                upsert_project(&mut connection, id, "Invented slots", model, &json!([]),
                    &Value::Null, &Value::Null, &Value::Null, &Value::Null,
                    &computed_model_hash(model, id), &Value::Null).unwrap();
            }
        }
        let opened = open_project_in_store(db.clone(), Some("project:slots")).unwrap().unwrap();
        assert_eq!(opened.model, every_slot);
        assert_eq!(hash_value_string(&opened.model_hash), model_payload_hash(&every_slot));
        let error = open_project_in_store(db.clone(), Some("project:slots-malformed"))
            .expect_err("a malformed slot is refused at project load");
        assert!(error.contains("MODEL-CONSTRAINT-KIND-INVALID at constraints[0].constraint_kind"), "{error}");
        let (connection, _) = open_project_store(&db).unwrap();
        let after = load_project(&connection, Some("project:slots-malformed")).unwrap().unwrap();
        assert_eq!(after.model, malformed, "the stored bytes are never rewritten");
        drop(connection);
        std::fs::remove_dir_all(root).unwrap();
    }

    /// W-13 §5.4 (S2): a desktop-emitted document carrying every model-data
    /// member goes through Save Local Project and back with equal content and
    /// model hash; an edit (create a line) saved in place and its undo (the
    /// inverse operation) saved again return the original document and hash.
    /// Save refuses a document with a blocking validator finding, by name, and
    /// leaves the stored bytes as they were.
    #[test]
    fn save_local_project_round_trips_every_slot_and_refuses_malformed_slots() {
        let root = std::env::temp_dir().join(format!(
            "ops-slot-save-{}-{}",
            std::process::id(),
            std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()
        ));
        std::fs::create_dir(&root).unwrap();
        let db = root.join("isolated.sqlite");
        let golden: Value = serde_json::from_str(include_str!(
            "../../../../fixtures/model_document/desktop_emitted.json"
        )).unwrap();
        let model = golden["entries"].as_array().unwrap().last().unwrap()["document"].clone();
        for slot in ["lines", "unresolved_assumptions", "constraints", "equipment_interfaces", "rule_pack_refs"] {
            assert!(model[slot].as_array().is_some_and(|items| !items.is_empty()), "{slot}");
        }
        // W-17: the round trip carries one library-applied entity.
        assert!(model["materials"].as_array().unwrap().iter().any(|m| m.pointer("/provenance_record/library_ref/library_version").is_some()));
        let project_id = model["project"]["id"].as_str().unwrap().to_string();
        let save = |model: &Value, confirmed: Option<String>| save_project_in_store(db.clone(), SaveLocalProjectRequest::from_value(json!({
            "project_id": project_id, "project_name": "Invented slots", "model": model,
            "model_hash": computed_model_hash(model, &project_id), "confirmed_revision": confirmed
        })).unwrap());
        let confirmed = || {
            let (connection, _) = open_project_store(&db).unwrap();
            let entry = stored_project_entry(&connection, &project_id).unwrap().unwrap();
            Some(stored_revision(&connection, &entry).unwrap())
        };

        save(&model, None).expect("an every-slot document saves");
        let opened = open_project_in_store(db.clone(), Some(&project_id)).unwrap().unwrap();
        assert_eq!(opened.model, model);
        assert_eq!(hash_value_string(&opened.model_hash), model_payload_hash(&model));

        let line = json!({"id": "line:L-900", "label": "Invented line", "pipe_refs": []});
        let intent = |kind: &str, change_kind: &str, before: String, after: String| json!({
            "operation_id": format!("operation:{change_kind}"), "operation_kind": kind, "operation_status": "proposed",
            "author_type": "user", "target": {"object_type": "Line", "ref": "line:L-900"},
            "change": {"change_id": format!("change:{change_kind}"), "change_kind": change_kind, "field_label": "Line",
                "field_path": "lines", "before": before, "after": after, "unit": "none", "dimension": "dimensionless",
                "source_note": "invented"},
            "validation": {"schema_validation": "not_run", "constraint_validation": "not_run", "unit_validation": "not_run",
                "diff_preview_status": "not_generated", "application_status": "not_applied"},
            "audit_boundary": {"mutation_route": "structured_operations_only", "direct_model_mutation_allowed": false,
                "requires_user_acceptance": true, "mutates_accepted_model_state": false},
            "professional_boundary": {"human_review_required": true, "software_makes_compliance_claim": false,
                "software_makes_certification_claim": false, "software_makes_sealing_claim": false,
                "software_makes_approval_claim": false, "software_makes_authentication_claim": false},
            "rationale": "invented"
        });
        let canonical = open_pipe_stress_operation_applier::canonical_json;
        let edited = open_pipe_stress_operation_applier::apply_operation(&opened.model,
            &intent("create", "create_line", "not_present".into(), canonical(&line)), None)
            .applied_model.expect("create_line applies");
        save(&edited, confirmed()).expect("the edit saves in place");
        let reopened = open_project_in_store(db.clone(), Some(&project_id)).unwrap().unwrap();
        assert_eq!(reopened.model, edited);
        let undone = open_pipe_stress_operation_applier::apply_operation(&reopened.model,
            &intent("delete", "delete_line", canonical(&line), "not_present".into()), None)
            .applied_model.expect("the inverse applies");
        save(&undone, confirmed()).expect("the undo saves in place");
        let restored = open_project_in_store(db.clone(), Some(&project_id)).unwrap().unwrap();
        assert_eq!(restored.model, model);
        assert_eq!(model_payload_hash(&restored.model), model_payload_hash(&model));

        let mut malformed = model.clone();
        malformed["lines"][0]["pipe_refs"] = json!(["pipe:P-absent"]);
        let error = save(&malformed, confirmed()).expect_err("save refuses a malformed slot");
        assert!(error.contains("MODEL-LINE-PIPE-MISSING at lines[0].pipe_refs[0]"), "{error}");
        let still = open_project_in_store(db.clone(), Some(&project_id)).unwrap().unwrap();
        assert_eq!(still.model, model, "the stored bytes are untouched");
        std::fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn a_current_document_persists_with_its_raw_received_claims() {
        let mut connection = Connection::open_in_memory().expect("in-memory sqlite opens");
        apply_store_migrations(&connection).expect("store migrations apply");
        let model = json!({ "schema_version": open_pipe_stress_product_physics::MODEL_SCHEMA_VERSION, "project": { "id": "project:unchanged-history", "name": "Unchanged History" } });
        let raw_model_claim = json!(["received", { "algorithm": "sha512" }]);
        let raw_envelope_claim = json!("received-malformed-envelope");
        let status = check_model_document_for_persist(&model).expect("current document persists");
        assert_eq!(status.status, "current");
        upsert_project(
            &mut connection,
            "project:unchanged-history",
            "Unchanged History",
            &model,
            &json!([]),
            &Value::Null,
            &Value::Null,
            &json!("malformed-received-result"),
            &json!(["malformed-received-analysis"]),
            &raw_model_claim,
            &raw_envelope_claim,
        )
        .expect("current row persists");
        let loaded = load_project(&connection, Some("project:unchanged-history")).unwrap().unwrap();
        assert_eq!(loaded.model, model);
        assert_eq!(loaded.model_hash, raw_model_claim);
        assert_eq!(loaded.project_envelope_hash, raw_envelope_claim);
    }

    #[test]
    fn validate_model_operation_command_blocks_stale_intent_without_mutation() {
        let model =
            read_fixture("invented_demo_model.json").expect("bundled preview model loads");
        let intent = fixture_inspector_intent("123", "195000000000");
        let outcome =
            validate_model_operation_value(model, intent, None).expect("command returns outcome");

        assert_eq!(outcome["validation"]["application_status"], "not_applied");
        assert_eq!(
            outcome["validation"]["before_state_validation"],
            "blocked_stale"
        );
        assert!(outcome["applied_model"].is_null());
        let codes: Vec<&str> = outcome["diagnostics"]
            .as_array()
            .expect("diagnostics array")
            .iter()
            .filter_map(|item| item["code"].as_str())
            .collect();
        assert!(codes.contains(&"OP-STALE-BEFORE-VALUE"));
    }

    fn example_rule_pack_document() -> Value {
        let cwd = std::env::current_dir().expect("cwd");
        let candidates = [
            cwd.join("../../examples/rule_packs/invented_demo.yaml"),
            cwd.join("../../../examples/rule_packs/invented_demo.yaml"),
            cwd.join("examples/rule_packs/invented_demo.yaml"),
        ];
        let path = candidates
            .into_iter()
            .find(|path| path.exists())
            .expect("invented example rule pack found");
        let raw = fs::read_to_string(path).expect("example rule pack readable");
        serde_json::from_str(&raw).expect("example rule pack is strict JSON")
    }

    fn demo_solved_envelope(actual: f64) -> Value {
        demo_solved_envelope_with_unit(actual, "demo_unit")
    }

    fn demo_solved_envelope_with_unit(actual: f64, unit: &str) -> Value {
        json!({
            "status": {
                "mechanics": "MECHANICS_SOLVED",
                "rule_check": "RULE_INPUTS_INCOMPLETE"
            },
            "results": [
                {
                    "id": "result:stress:demo",
                    "kind": "stress",
                    "value": actual,
                    "unit": unit,
                    "entity_ref": "pipe:demo"
                }
            ]
        })
    }

    fn demo_solver_selectors() -> Value {
        json!([{ "input_id": "demo_actual_quantity", "result_id": "result:stress:demo" }])
    }

    fn demo_supplied_values() -> Value {
        json!([
            { "ref_id": "demo_limit_quantity", "value": 100.0, "unit": "demo_unit", "dimension": "stress" },
            { "ref_id": "demo_limit_slot", "value": 1.0, "unit": "ratio", "dimension": "dimensionless" }
        ])
    }

    fn demo_supplied_values_with_stress_unit(value: f64, unit: &str) -> Value {
        json!([
            { "ref_id": "demo_limit_quantity", "value": value, "unit": unit, "dimension": "stress" },
            { "ref_id": "demo_limit_slot", "value": 1.0, "unit": "ratio", "dimension": "dimensionless" }
        ])
    }

    fn example_rule_pack_with_stress_unit(unit: &str) -> Value {
        let mut pack = example_rule_pack_document();
        pack["required_inputs"][0]["quantity_intent"]["unit_ref"] = json!(unit);
        pack["required_inputs"][1]["quantity_intent"]["unit_ref"] = json!(unit);
        let computed =
            rule_pack_document::compute_rule_pack_checksum(&pack).expect("checksum recomputes");
        pack["checksums"]["rule_pack_checksum"]["value"] = json!(computed.value);
        pack
    }

    #[test]
    fn run_rule_checks_command_passing_demo_reports_user_rule_checked() {
        let outcome = run_rule_checks_core(
            example_rule_pack_document(),
            demo_solved_envelope(50.0),
            Some(demo_solver_selectors()),
            Some(demo_supplied_values()),
            None,
        )
        .expect("rule checks run");
        assert_eq!(outcome["aggregate_status"], json!("USER_RULE_CHECKED"));
        assert_eq!(outcome["rule_pack_id"], json!("invented_demo_rule_pack"));
        assert_eq!(outcome["checks"][0]["status"], json!("USER_RULE_CHECKED"));
        assert_eq!(outcome["checks"][0]["computed_value"]["value"], json!(0.5));
    }

    #[test]
    fn run_rule_checks_command_normalizes_compatible_rule_check_units() {
        let outcome = run_rule_checks_core(
            example_rule_pack_with_stress_unit("Pa"),
            demo_solved_envelope_with_unit(0.05, "MPa"),
            Some(demo_solver_selectors()),
            Some(demo_supplied_values_with_stress_unit(100.0, "kPa")),
            None,
        )
        .expect("rule checks run");

        assert_eq!(outcome["aggregate_status"], json!("USER_RULE_CHECKED"));
        assert_eq!(outcome["checks"][0]["computed_value"]["value"], json!(0.5));
        let bound_inputs = outcome["checks"][0]["bound_inputs"]
            .as_array()
            .expect("bound inputs");
        let actual = bound_inputs
            .iter()
            .find(|binding| binding["input_id"] == json!("demo_actual_quantity"))
            .expect("actual input");
        assert_eq!(actual["value"], json!(50_000.0));
        assert_eq!(actual["unit"], json!("Pa"));
        let limit = bound_inputs
            .iter()
            .find(|binding| binding["input_id"] == json!("demo_limit_quantity"))
            .expect("limit input");
        assert_eq!(limit["value"], json!(100_000.0));
        assert_eq!(limit["unit"], json!("Pa"));
    }

    #[test]
    fn run_rule_checks_command_failing_case_reports_user_rule_failed() {
        let outcome = run_rule_checks_core(
            example_rule_pack_document(),
            demo_solved_envelope(150.0),
            Some(demo_solver_selectors()),
            Some(demo_supplied_values()),
            None,
        )
        .expect("rule checks run");
        assert_eq!(outcome["aggregate_status"], json!("USER_RULE_FAILED"));
        assert_eq!(outcome["checks"][0]["status"], json!("USER_RULE_FAILED"));
    }

    #[test]
    fn run_rule_checks_command_missing_input_reports_rule_inputs_incomplete() {
        // No solver selectors -> the solver_result input is unsupplied.
        let outcome = run_rule_checks_core(
            example_rule_pack_document(),
            demo_solved_envelope(50.0),
            None,
            Some(demo_supplied_values()),
            None,
        )
        .expect("rule checks run");
        assert_eq!(outcome["aggregate_status"], json!("RULE_INPUTS_INCOMPLETE"));
        assert!(outcome["checks"][0]["computed_value"].is_null());
    }

    #[test]
    fn run_rule_checks_command_rejects_blocking_invalid_pack() {
        let outcome = run_rule_checks_core(
            json!({ "not": "a valid rule pack" }),
            demo_solved_envelope(50.0),
            Some(demo_solver_selectors()),
            Some(demo_supplied_values()),
            None,
        )
        .expect("command returns");
        assert_eq!(outcome["aggregate_status"], json!("RULE_INPUTS_INCOMPLETE"));
    }

    #[test]
    fn run_rule_checks_command_emits_only_status_vocabulary() {
        let outcome = run_rule_checks_core(
            example_rule_pack_document(),
            demo_solved_envelope(50.0),
            Some(demo_solver_selectors()),
            Some(demo_supplied_values()),
            None,
        )
        .expect("rule checks run");
        let serialized = serde_json::to_string(&outcome).expect("serializes");
        for forbidden in [
            "HUMAN_APPROVED_FOR_PROJECT",
            "Certified",
            "Sealed",
            "code_compliant",
        ] {
            assert!(
                !serialized.contains(forbidden),
                "command output must not contain {forbidden}"
            );
        }
    }

    #[test]
    fn local_rule_pack_store_round_trips_documents_per_project() {
        let connection = Connection::open_in_memory().expect("in-memory sqlite opens");
        apply_store_migrations(&connection).expect("store migrations apply");

        let document = example_rule_pack_document();
        let rule_pack_id = rule_pack_id_from_document(&document).expect("example carries an id");
        let (created_at, updated_at) =
            upsert_rule_pack(&connection, "project:test-local", &rule_pack_id, &document)
                .expect("rule pack upserts");
        assert!(created_at > 0);
        assert!(updated_at >= created_at);

        let listed =
            list_rule_packs(&connection, Some("project:test-local")).expect("rule packs list");
        assert_eq!(listed.len(), 1);
        assert_eq!(listed[0].rule_pack_id, rule_pack_id);
        assert_eq!(listed[0].rule_pack_name, "Invented Demonstration Rule Pack");
        assert_eq!(listed[0].storage_mode, "local_sqlite");
        assert_eq!(listed[0].lifecycle_status, "accepted_public_example");

        let other_project =
            list_rule_packs(&connection, Some("project:other")).expect("other project lists");
        assert!(other_project.is_empty());

        let loaded = load_rule_pack(&connection, "project:test-local", &rule_pack_id)
            .expect("rule pack loads")
            .expect("stored row present");
        assert_eq!(loaded.document, document);

        // Update keeps created_at, advances updated_at semantics (same-second
        // updates produce equal stamps; never earlier).
        let (created_again, _updated_again) =
            upsert_rule_pack(&connection, "project:test-local", &rule_pack_id, &document)
                .expect("rule pack re-upserts");
        assert_eq!(created_again, created_at);

        let deleted = delete_rule_pack(&connection, "project:test-local", &rule_pack_id)
            .expect("rule pack deletes");
        assert!(deleted);
        let deleted_again = delete_rule_pack(&connection, "project:test-local", &rule_pack_id)
            .expect("second delete returns");
        assert!(!deleted_again);
    }

    // --- C3 <-> C4 library-reference wiring (TP-C3C4-LIBREF-001) -------------

    fn material_library_with_allowable() -> Value {
        json!({
            "material_library": { "library_id": "lib:test-materials" },
            "material_records": [
                {
                    "material_id": "mat:demo-steel",
                    "allowables": [
                        {
                            "allowable_id": "allow:Sh",
                            "allowable_kind": "user_rule_allowable",
                            "value": { "magnitude": 100.0, "unit_ref": { "ref_id": "demo_unit" } }
                        }
                    ]
                }
            ]
        })
    }

    fn section_library_with_slots() -> Value {
        // Section value slots carry a plain `unit` string (not `unit_ref`), per
        // the section schema's QuantityValue; values live in `dimensions` and
        // `properties` arrays keyed by `dimension_id` / `property_id`.
        json!({
            "section_library": { "library_id": "lib:test-sections" },
            "section_records": [
                {
                    "section_id": "sec:demo-pipe",
                    "dimensions": [
                        {
                            "dimension_id": "dim:od",
                            "dimension_kind": "outside_diameter",
                            "value": { "magnitude": 168.3, "unit": "mm", "dimension": "outside_diameter" }
                        }
                    ],
                    "properties": [
                        {
                            "property_id": "prop:area",
                            "property_kind": "cross_section_area",
                            "value": { "magnitude": 2300.0, "unit": "mm2", "dimension": "cross_section_area" }
                        }
                    ]
                }
            ]
        })
    }

    fn component_library_with_field() -> Value {
        // Component value slots also carry a plain `unit` string; values live in
        // a `fields` array keyed by `field_id` (per the component schema).
        json!({
            "component_library": { "library_id": "lib:test-components" },
            "component_records": [
                {
                    "component_id": "cmp:demo-bend",
                    "fields": [
                        {
                            "field_id": "fld:sif",
                            "field_kind": "sif_user_value",
                            "value": { "magnitude": 2.5, "unit": "dimensionless", "dimension": "dimensionless" }
                        }
                    ]
                }
            ]
        })
    }

    fn pack_with_library_ref() -> Value {
        pack_with_library_ref_to(json!({
            "library_kind": "material",
            "library_id": "lib:test-materials",
            "record_id": "mat:demo-steel",
            "slot_id": "allow:Sh"
        }))
    }

    fn pack_with_library_ref_to(reference: Value) -> Value {
        let mut pack = example_rule_pack_document();
        // Repoint demo_limit_quantity to a private-library value reference.
        pack["required_inputs"][1]["source_kind"] = json!("private_library_value");
        pack["required_inputs"][1]["library_value_ref"] = reference;
        // Re-stamp the checksum so validation does not block on a stale hash
        // (the checksum covers the document excluding the `checksums` member).
        let computed =
            rule_pack_document::compute_rule_pack_checksum(&pack).expect("checksum recomputes");
        pack["checksums"]["rule_pack_checksum"]["value"] = json!(computed.value);
        pack
    }

    /// Give the demo pack's `demo_actual_quantity` (the `solver_result` input,
    /// required_inputs[0]) an authored in-pack `solver_result_ref` to a solved
    /// result row, then re-stamp the checksum so validation does not block.
    fn pack_with_solver_result_ref(result_id: &str) -> Value {
        let mut pack = example_rule_pack_document();
        pack["required_inputs"][0]["solver_result_ref"] = json!({ "result_id": result_id });
        let computed =
            rule_pack_document::compute_rule_pack_checksum(&pack).expect("checksum recomputes");
        pack["checksums"]["rule_pack_checksum"]["value"] = json!(computed.value);
        pack
    }

    #[test]
    fn resolve_authored_solver_result_bindings_resolves_ref_and_records_input_id() {
        let pack = pack_with_solver_result_ref("result:stress:demo");
        let envelope = demo_solved_envelope(42.0);
        let (bindings, authored_ids) = resolve_authored_solver_result_bindings(&pack, &envelope, &mut Vec::new());
        assert_eq!(bindings.len(), 1);
        assert_eq!(bindings[0].input_id, "demo_actual_quantity");
        assert_eq!(bindings[0].result_id, "result:stress:demo");
        assert_eq!(bindings[0].value, 42.0);
        assert_eq!(bindings[0].unit, "demo_unit");
        assert!(authored_ids.contains("demo_actual_quantity"));
    }

    #[test]
    fn resolve_authored_solver_result_bindings_records_input_id_but_omits_unresolvable_binding() {
        // An authored ref to a missing row records the input id (so the
        // caller-supplied fallback is excluded) but yields no binding: the input
        // then blocks, never a silent pass and never a caller rescue.
        let pack = pack_with_solver_result_ref("result:absent");
        let envelope = demo_solved_envelope(42.0);
        let (bindings, authored_ids) = resolve_authored_solver_result_bindings(&pack, &envelope, &mut Vec::new());
        assert!(bindings.is_empty());
        assert!(authored_ids.contains("demo_actual_quantity"));
    }

    #[test]
    fn extract_library_slot_value_reads_material_allowable() {
        let document = material_library_with_allowable();
        let resolved =
            extract_library_slot_value(&document, "material", "mat:demo-steel", "allow:Sh")
                .expect("material allowable resolves");
        assert_eq!(resolved, (100.0, "demo_unit".to_string()));
        // Unknown record/slot -> None (input then blocks).
        assert!(
            extract_library_slot_value(&document, "material", "mat:absent", "allow:Sh").is_none()
        );
        assert!(extract_library_slot_value(
            &document,
            "material",
            "mat:demo-steel",
            "allow:absent"
        )
        .is_none());
        // A material document has no `section_records`, so a section lookup over
        // it finds nothing -> None (the kind/document mismatch blocks).
        assert!(
            extract_library_slot_value(&document, "section", "mat:demo-steel", "allow:Sh")
                .is_none()
        );
    }

    #[test]
    fn extract_library_slot_value_reads_section_dimension_and_property() {
        let document = section_library_with_slots();
        // A dimension slot resolves with its plain `unit` string.
        assert_eq!(
            extract_library_slot_value(&document, "section", "sec:demo-pipe", "dim:od"),
            Some((168.3, "mm".to_string()))
        );
        // A property slot in the same record resolves too (second slot array).
        assert_eq!(
            extract_library_slot_value(&document, "section", "sec:demo-pipe", "prop:area"),
            Some((2300.0, "mm2".to_string()))
        );
        // Unknown record / slot -> None (input then blocks).
        assert!(extract_library_slot_value(&document, "section", "sec:absent", "dim:od").is_none());
        assert!(
            extract_library_slot_value(&document, "section", "sec:demo-pipe", "dim:absent")
                .is_none()
        );
    }

    #[test]
    fn extract_library_slot_value_reads_component_field() {
        let document = component_library_with_field();
        assert_eq!(
            extract_library_slot_value(&document, "component", "cmp:demo-bend", "fld:sif"),
            Some((2.5, "dimensionless".to_string()))
        );
        assert!(
            extract_library_slot_value(&document, "component", "cmp:absent", "fld:sif").is_none()
        );
        assert!(
            extract_library_slot_value(&document, "component", "cmp:demo-bend", "fld:absent")
                .is_none()
        );
    }

    #[test]
    fn extract_library_slot_value_returns_none_for_unknown_kind() {
        let document = section_library_with_slots();
        assert!(extract_library_slot_value(&document, "TBD", "sec:demo-pipe", "dim:od").is_none());
        assert!(extract_library_slot_value(&document, "", "sec:demo-pipe", "dim:od").is_none());
    }

    #[test]
    fn library_value_ref_resolves_from_local_store() {
        let connection = Connection::open_in_memory().expect("in-memory sqlite opens");
        apply_store_migrations(&connection).expect("store migrations apply");
        upsert_library(
            &connection,
            "project:lib-test",
            "material",
            "lib:test-materials",
            &material_library_with_allowable(),
        )
        .expect("library upserts");

        let bindings = resolve_library_value_bindings_with_connection(
            &connection,
            "project:lib-test",
            &pack_with_library_ref(),
        )
        .expect("resolves");
        let array = bindings.as_array().expect("array");
        assert_eq!(array.len(), 1);
        assert_eq!(array[0]["input_id"], json!("demo_limit_quantity"));
        assert_eq!(array[0]["value"], json!(100.0));
        assert_eq!(array[0]["unit"], json!("demo_unit"));
        assert_eq!(array[0]["library_id"], json!("lib:test-materials"));
    }

    #[test]
    fn library_value_ref_resolves_section_from_local_store() {
        // The resolve path is kind-generic: a section reference resolves a
        // section property slot (plain `unit` string) from the same store.
        let connection = Connection::open_in_memory().expect("in-memory sqlite opens");
        apply_store_migrations(&connection).expect("store migrations apply");
        upsert_library(
            &connection,
            "project:lib-test",
            "section",
            "lib:test-sections",
            &section_library_with_slots(),
        )
        .expect("library upserts");

        let pack = pack_with_library_ref_to(json!({
            "library_kind": "section",
            "library_id": "lib:test-sections",
            "record_id": "sec:demo-pipe",
            "slot_id": "prop:area"
        }));
        let bindings =
            resolve_library_value_bindings_with_connection(&connection, "project:lib-test", &pack)
                .expect("resolves");
        let array = bindings.as_array().expect("array");
        assert_eq!(array.len(), 1);
        assert_eq!(array[0]["input_id"], json!("demo_limit_quantity"));
        assert_eq!(array[0]["value"], json!(2300.0));
        assert_eq!(array[0]["unit"], json!("mm2"));
        assert_eq!(array[0]["library_kind"], json!("section"));
        assert_eq!(array[0]["library_id"], json!("lib:test-sections"));
        assert_eq!(array[0]["slot_id"], json!("prop:area"));
    }

    #[test]
    fn library_value_ref_without_stored_library_resolves_to_no_binding() {
        let connection = Connection::open_in_memory().expect("in-memory sqlite opens");
        apply_store_migrations(&connection).expect("store migrations apply");
        // No library saved -> reference cannot resolve -> no binding (blocks).
        let bindings = resolve_library_value_bindings_with_connection(
            &connection,
            "project:lib-test",
            &pack_with_library_ref(),
        )
        .expect("resolves");
        assert!(bindings.as_array().expect("array").is_empty());
    }

    #[test]
    fn run_rule_checks_core_threads_library_binding_to_user_rule_checked() {
        // actual 50 / library limit 100 = 0.5 <= ratio_limit 1.0 -> pass.
        let library_bindings = json!([{
            "input_id": "demo_limit_quantity",
            "value": 100.0,
            "unit": "demo_unit",
            "library_kind": "material",
            "library_id": "lib:test-materials",
            "record_id": "mat:demo-steel",
            "slot_id": "allow:Sh"
        }]);
        let outcome = run_rule_checks_core(
            pack_with_library_ref(),
            demo_solved_envelope(50.0),
            Some(demo_solver_selectors()),
            Some(demo_supplied_values()),
            Some(library_bindings),
        )
        .expect("rule checks run");
        assert_eq!(outcome["aggregate_status"], json!("USER_RULE_CHECKED"));
        let bound = outcome["checks"][0]["bound_inputs"]
            .as_array()
            .expect("bound inputs")
            .iter()
            .find(|binding| binding["input_id"] == json!("demo_limit_quantity"))
            .expect("library-sourced input present");
        assert_eq!(bound["supplied"], json!(true));
        assert!(bound["note"]
            .as_str()
            .expect("note")
            .contains("private library"));
    }

    #[test]
    fn run_rule_checks_core_without_library_binding_blocks_library_input() {
        // private_library_value input with no resolved binding -> blocks (never
        // a silent pass).
        let outcome = run_rule_checks_core(
            pack_with_library_ref(),
            demo_solved_envelope(50.0),
            Some(demo_solver_selectors()),
            Some(demo_supplied_values()),
            None,
        )
        .expect("rule checks run");
        assert_eq!(outcome["aggregate_status"], json!("RULE_INPUTS_INCOMPLETE"));
    }

    #[test]
    fn run_rule_checks_core_authored_solver_result_ref_resolves_without_caller_selector() {
        // The authored in-pack solver_result_ref binds the solver value end to
        // end with NO caller-supplied selector: actual 50 / limit 100 = 0.5
        // <= ratio_limit 1.0 -> pass. This is the additive solver-result-selector
        // member retiring the caller-supplied solver binding.
        let outcome = run_rule_checks_core(
            pack_with_solver_result_ref("result:stress:demo"),
            demo_solved_envelope(50.0),
            None,
            Some(demo_supplied_values()),
            None,
        )
        .expect("rule checks run");
        assert_eq!(outcome["aggregate_status"], json!("USER_RULE_CHECKED"));
        assert_eq!(outcome["checks"][0]["status"], json!("USER_RULE_CHECKED"));
        assert_eq!(outcome["checks"][0]["computed_value"]["value"], json!(0.5));
        let bound = outcome["checks"][0]["bound_inputs"]
            .as_array()
            .expect("bound inputs")
            .iter()
            .find(|binding| binding["input_id"] == json!("demo_actual_quantity"))
            .expect("solver-sourced input present");
        assert_eq!(bound["supplied"], json!(true));
        assert_eq!(bound["result_id"], json!("result:stress:demo"));
    }

    #[test]
    fn run_rule_checks_core_authored_solver_result_ref_unresolvable_blocks_over_caller_selector() {
        // The authored ref points at a missing row, so the solver_result input
        // stays unsupplied and the check blocks -- even though a caller-supplied
        // selector for the SAME input would resolve. An authored reference is
        // canonical: the caller cannot override or rescue it (run-time override
        // is a deferred non-goal). Never a silent pass.
        let outcome = run_rule_checks_core(
            pack_with_solver_result_ref("result:absent"),
            demo_solved_envelope(50.0),
            Some(demo_solver_selectors()),
            Some(demo_supplied_values()),
            None,
        )
        .expect("rule checks run");
        assert_eq!(outcome["aggregate_status"], json!("RULE_INPUTS_INCOMPLETE"));
    }

    #[test]
    fn validate_rule_pack_command_reports_example_pack_clean_and_draft_findings() {
        let document = example_rule_pack_document();
        let outcome = validate_rule_pack_value(document, Some(true)).expect("validation runs");
        assert_eq!(outcome["has_blocking_findings"], json!(false));
        assert_eq!(outcome["checksum"]["match_status"], json!("match"));
        assert_eq!(outcome["grammar_version_supported"], json!(true));
        assert_eq!(outcome["publicly_exportable"], json!(true));
        assert_eq!(outcome["formulas"][0]["decode_status"], json!("decoded"));
        let notice = outcome["professional_boundary_notice"]
            .as_str()
            .expect("notice present");
        assert!(notice.contains(
            "Document validation results are engineering decision-support information \
             computed from user-supplied rule-pack content."
        ));
        assert!(notice.contains("Human review remains required."));

        // A draft missing its grammar_version is a blocking software finding
        // (never best-effort), but the document remains a valid draft shape.
        let mut draft = example_rule_pack_document();
        draft
            .as_object_mut()
            .expect("object")
            .remove("grammar_version");
        let draft_outcome = validate_rule_pack_value(draft, None).expect("draft validation runs");
        assert_eq!(draft_outcome["has_blocking_findings"], json!(true));
        let codes: Vec<&str> = draft_outcome["document_findings"]
            .as_array()
            .expect("document findings array")
            .iter()
            .filter_map(|finding| finding["code"].as_str())
            .collect();
        assert!(codes.contains(&"MISSING_GRAMMAR_VERSION"));
    }

    /// W-17 (DEL-06-02 `Design/rule-pack-unit-check.md` §4): the example
    /// pack with a second check whose formula subtracts quantities in two
    /// different units (and a second required input for it).
    fn example_rule_pack_with_unit_inconsistent_check(other_unit: &str, restamp: bool) -> Value {
        let mut pack = example_rule_pack_document();
        let mut other_input = pack["required_inputs"][1].clone();
        other_input["input_id"] = json!("demo_other_quantity");
        other_input["quantity_intent"]["unit_ref"] = json!(other_unit);
        pack["required_inputs"]
            .as_array_mut()
            .expect("required inputs")
            .push(other_input);
        let mut formula = pack["formula_declarations"][0].clone();
        formula["formula_id"] = json!("demo_mismatch_expression");
        formula["declaration_payload"]["expression_ast"] = json!({
            "node": "binary",
            "operator": "subtract",
            "left": {"node": "variable_ref", "variable_id": "demo_actual_quantity"},
            "right": {"node": "variable_ref", "variable_id": "demo_other_quantity"}
        });
        formula["input_refs"] = json!([
            {"ref_id": "demo_actual_quantity", "ref_type": "required_input"},
            {"ref_id": "demo_other_quantity", "ref_type": "required_input"}
        ]);
        pack["formula_declarations"]
            .as_array_mut()
            .expect("formulas")
            .push(formula);
        let mut check = pack["check_definitions"][0].clone();
        check["check_id"] = json!("demo_inconsistent_check");
        check["formula_ref"]["ref_id"] = json!("demo_mismatch_expression");
        check["required_input_refs"] = json!([
            {"ref_id": "demo_actual_quantity", "ref_type": "required_input"},
            {"ref_id": "demo_other_quantity", "ref_type": "required_input"}
        ]);
        pack["check_definitions"]
            .as_array_mut()
            .expect("checks")
            .push(check);
        if restamp {
            let computed =
                rule_pack_document::compute_rule_pack_checksum(&pack).expect("checksum recomputes");
            pack["checksums"]["rule_pack_checksum"]["value"] = json!(computed.value);
        }
        pack
    }

    #[test]
    fn run_rule_checks_core_runs_other_checks_when_one_is_unit_inconsistent() {
        let pack = example_rule_pack_with_unit_inconsistent_check("other_demo_unit", true);
        let validation =
            validate_rule_pack_value(pack.clone(), None).expect("validation runs");
        assert_eq!(validation["has_blocking_findings"], json!(false));
        assert_eq!(validation["unit_check"]["status"], json!("findings"));
        assert_eq!(
            validation["unit_check"]["checks"][0]["status"],
            json!("consistent")
        );
        let inconsistent = &validation["unit_check"]["checks"][1];
        assert_eq!(inconsistent["check_id"], json!("demo_inconsistent_check"));
        assert_eq!(inconsistent["status"], json!("findings"));
        assert_eq!(inconsistent["findings"][0]["code"], json!("UnitMismatch"));
        assert_eq!(inconsistent["findings"][0]["subject"], json!("add_subtract"));
        assert_eq!(inconsistent["findings"][0]["severity"], json!("blocking"));

        let mut supplied = demo_supplied_values();
        supplied.as_array_mut().expect("supplied").push(json!({
            "ref_id": "demo_other_quantity",
            "value": 10.0,
            "unit": "other_demo_unit",
            "dimension": "stress"
        }));
        let outcome = run_rule_checks_core(
            pack,
            demo_solved_envelope(50.0),
            Some(demo_solver_selectors()),
            Some(supplied),
            None,
        )
        .expect("rule checks run");
        assert!(outcome.get("rule_pack_validation").is_none());
        assert_eq!(outcome["checks"][0]["check_id"], json!("demo_training_check"));
        assert_eq!(outcome["checks"][0]["status"], json!("USER_RULE_CHECKED"));
        assert_eq!(outcome["checks"][1]["check_id"], json!("demo_inconsistent_check"));
        assert_eq!(outcome["checks"][1]["status"], json!("RULE_INPUTS_INCOMPLETE"));
        let runtime: Vec<(&str, &str)> = outcome["checks"][1]["evaluator_findings"]
            .as_array()
            .expect("evaluator findings")
            .iter()
            .map(|finding| {
                (
                    finding["code"].as_str().unwrap_or_default(),
                    finding["subject_id"].as_str().unwrap_or_default(),
                )
            })
            .collect();
        assert_eq!(runtime, vec![("UnitMismatch", "add_subtract")]);
    }

    #[test]
    fn unit_check_private_tokens_reach_validation_but_never_a_blocked_run_response() {
        // Adversarial private strings as a unit token and as a value-slot id.
        const PRIVATE_UNIT: &str = "PRIVATE-UNIT-7f3a<script>\"'";
        const PRIVATE_SLOT: &str = "PRIVATE-SLOT-91c2</div>";
        let mut pack = example_rule_pack_with_unit_inconsistent_check(PRIVATE_UNIT, false);
        pack["check_definitions"][0]["value_slot_refs"]
            .as_array_mut()
            .expect("slot refs")
            .push(json!({"ref_id": PRIVATE_SLOT, "ref_type": "value_slot"}));

        let validation = validate_rule_pack_value(pack.clone(), None).expect("validation runs");
        let unit_check = serde_json::to_string(&validation["unit_check"]).expect("serializes");
        assert!(
            unit_check.contains("PRIVATE-UNIT-7f3a<script>"),
            "{unit_check}"
        );
        assert!(unit_check.contains("PRIVATE-SLOT-91c2</div>"), "{unit_check}");
        // The checksum was not restamped, so validation blocks the run.
        assert_eq!(validation["has_blocking_findings"], json!(true));

        let outcome = run_rule_checks_core(
            pack,
            demo_solved_envelope(50.0),
            Some(demo_solver_selectors()),
            Some(demo_supplied_values()),
            None,
        )
        .expect("blocked run returns a response");
        assert_eq!(outcome["aggregate_status"], json!("RULE_INPUTS_INCOMPLETE"));
        assert_eq!(outcome["checks"], json!([]));
        let embedded = &outcome["rule_pack_validation"];
        assert!(embedded.is_object(), "{outcome}");
        assert_eq!(embedded["has_blocking_findings"], json!(true));
        assert!(embedded.get("unit_check").is_none(), "{embedded}");
        let response = serde_json::to_string(&outcome).expect("serializes");
        assert!(!response.contains("PRIVATE-"), "{response}");
        assert!(!response.contains("unit_check"), "{response}");
    }

    #[test]
    fn compute_rule_pack_document_checksum_command_matches_stamped_example_value() {
        let document = example_rule_pack_document();
        let stamped = document
            .pointer("/checksums/rule_pack_checksum/value")
            .and_then(Value::as_str)
            .expect("stamped value present")
            .to_string();
        let computed = compute_rule_pack_document_checksum_value(document).expect("checksum computes");
        assert_eq!(computed["value"], json!(stamped));
        assert_eq!(computed["payload_excludes"], json!(["checksums"]));
        assert_eq!(computed["grammar_version_bound"], json!(true));
        assert_eq!(
            computed["basis"],
            json!("rfc8785_jcs_sha256_excluding_checksums")
        );
    }

    #[test]
    fn rule_pack_drafts_with_findings_remain_saveable_locally() {
        let connection = Connection::open_in_memory().expect("in-memory sqlite opens");
        apply_store_migrations(&connection).expect("store migrations apply");

        let mut draft = example_rule_pack_document();
        draft
            .as_object_mut()
            .expect("object")
            .remove("grammar_version");
        let rule_pack_id = rule_pack_id_from_document(&draft).expect("id survives draft state");
        upsert_rule_pack(&connection, "project:test-local", &rule_pack_id, &draft)
            .expect("draft with blocking findings still saves locally");
        let loaded = load_rule_pack(&connection, "project:test-local", &rule_pack_id)
            .expect("draft loads")
            .expect("draft row present");
        assert_eq!(loaded.document, draft);
    }

    fn valid_private_material_payload() -> Value {
        json!({
            "material_library": {
                "provenance": {
                    "source_name": "Invented source",
                    "source_location": "tests",
                    "source_license": "public test license",
                    "contributor": "SWBPIPE",
                    "contributor_certification": "invented non-engineering value",
                    "redistribution_status": "private_only",
                    "review_status": "accepted"
                }
            },
            "material_records": []
        })
    }

    #[test]
    fn validate_library_import_command_accepts_private_payload_and_carries_boundary_notice() {
        let outcome = validate_library_import_value(
            valid_private_material_payload(),
            "material".to_string(),
            "private".to_string(),
        )
        .expect("validation runs");
        assert_eq!(outcome["outcome"], json!("PRIVATE_LOCAL_ONLY"));
        assert_eq!(outcome["accepted"], json!(true));
        assert_eq!(outcome["has_blocking_findings"], json!(false));
        assert_eq!(outcome["library_kind"], json!("material"));
        assert_eq!(outcome["intended_visibility"], json!("private"));
        assert!(outcome["findings"]
            .as_array()
            .expect("findings array")
            .is_empty());
        let notice = outcome["professional_boundary_notice"]
            .as_str()
            .expect("notice present");
        assert!(notice.contains("software findings only"));
        // The seam never asserts a compliance/approval claim.
        assert!(!notice.to_lowercase().contains("certified"));
    }

    #[test]
    fn validate_library_import_command_blocks_missing_metadata_with_diagnostics() {
        let outcome =
            validate_library_import_value(json!({}), "material".to_string(), "private".to_string())
                .expect("validation runs");
        assert_eq!(outcome["outcome"], json!("REJECTED"));
        assert_eq!(outcome["accepted"], json!(false));
        assert_eq!(outcome["has_blocking_findings"], json!(true));
        let codes: Vec<&str> = outcome["findings"]
            .as_array()
            .expect("findings array")
            .iter()
            .filter_map(|finding| finding["code"].as_str())
            .collect();
        assert!(codes.contains(&"IMPORT_LIBRARY_METADATA_MISSING"));
        assert!(codes.contains(&"IMPORT_RECORD_SET_MISSING"));
        // The PKG-02 import-boundary diagnostic envelope rides alongside the
        // findings for downstream surfaces.
        let diagnostic = outcome["diagnostics"]
            .as_array()
            .expect("diagnostics array")
            .iter()
            .find(|entry| entry["code"] == json!("IMPORT_LIBRARY_METADATA_MISSING"))
            .expect("metadata diagnostic present");
        assert_eq!(diagnostic["class"], json!("import_boundary"));
    }

    #[test]
    fn validate_library_import_command_rejects_unsupported_tokens() {
        let kind_err =
            validate_library_import_value(json!({}), "widget".to_string(), "private".to_string())
                .expect_err("unsupported kind must reject");
        assert!(kind_err.contains("widget"));
        let visibility_err =
            validate_library_import_value(json!({}), "material".to_string(), "secret".to_string())
                .expect_err("unsupported visibility must reject");
        assert!(visibility_err.contains("secret"));
    }

    fn storable_material_payload(library_id: &str) -> Value {
        json!({
            "material_library": {
                "library_id": library_id,
                "name": "Invented private material library",
                "privacy_class": "private_user_data",
                "provenance": {
                    "source_name": "Invented source",
                    "source_location": "tests",
                    "source_license": "private test basis",
                    "contributor": "SWBPIPE",
                    "contributor_certification": "invented non-engineering value",
                    "redistribution_status": "private_only",
                    "review_status": "accepted"
                }
            },
            "material_records": []
        })
    }

    fn quarantined_material_payload(library_id: &str) -> Value {
        json!({
            "material_library": {
                "library_id": library_id,
                "name": "Suspected protected material",
                "provenance": {
                    "source_name": "Invented source",
                    "source_location": "tests",
                    "source_license": "unknown",
                    "contributor": "SWBPIPE",
                    "contributor_certification": "invented non-engineering value",
                    "redistribution_status": "protected_suspected",
                    "review_status": "accepted"
                }
            },
            "material_records": []
        })
    }

    #[test]
    fn model_library_refs_warn_when_unresolved_or_changed_and_never_update_the_version() {
        let connection = Connection::open_in_memory().expect("in-memory sqlite opens");
        apply_store_migrations(&connection).expect("store migrations apply");
        let mut document = storable_material_payload("matlib.invented.alpha");
        document["material_records"] = json!([{"material_id":"mat.invented.alpha","name":"Invented material alpha","properties":[]}]);
        let record_id = document["material_records"][0]["material_id"].as_str().unwrap().to_string();
        upsert_library(&connection, "project:lib-refs", "material", "matlib.invented.alpha", &document).expect("upserts");
        let stored = library_record_version(&document["material_records"][0]);
        // The digest is the SHA-256 of the record's RFC 8785 form.
        assert_eq!(stored, format!("sha256:{}", open_pipe_stress_operation_applier::sha256_hex(&open_pipe_stress_operation_applier::canonical_json(&document["material_records"][0]))));
        let reference = |library_id: &str, record: &str, version: &str| ModelLibraryRef {
            entity_ref: "material:inserted".into(),
            library_kind: "material".into(),
            library_id: library_id.into(),
            library_version: Some(version.into()),
            record_id: Some(record.into()),
        };
        let current = [reference("matlib.invented.alpha", &record_id, &stored)];
        assert!(library_ref_findings(&connection, "project:lib-refs", &current).unwrap().is_empty());
        // Another project id (Save as new project) does not hold the library.
        let other = library_ref_findings(&connection, "project:renamed", &current).unwrap();
        assert_eq!(other[0].code, MODEL_LIBRARY_REF_UNRESOLVED);
        let missing = library_ref_findings(&connection, "project:lib-refs", &[reference("matlib.absent", &record_id, &stored), reference("matlib.invented.alpha", "mat.absent", &stored)]).unwrap();
        assert_eq!(missing.iter().map(|f| (f.code, f.severity)).collect::<Vec<_>>(), vec![(MODEL_LIBRARY_REF_UNRESOLVED, "warning"); 2]);
        // The stored record changes: the version warning names both digests,
        // and the recorded version is reported, not replaced.
        document["material_records"][0]["name"] = json!("Invented material alpha, revised");
        upsert_library(&connection, "project:lib-refs", "material", "matlib.invented.alpha", &document).expect("upserts");
        let changed = library_ref_findings(&connection, "project:lib-refs", &current).unwrap();
        assert_eq!(changed.len(), 1);
        assert_eq!(changed[0].code, MODEL_LIBRARY_REF_VERSION_CHANGED);
        assert_eq!(changed[0].severity, "warning");
        assert_eq!(changed[0].recorded_version.as_deref(), Some(stored.as_str()));
        assert_eq!(changed[0].stored_version, Some(library_record_version(&document["material_records"][0])));
        assert_eq!(current[0].library_version.as_deref(), Some(stored.as_str()));
    }

    #[test]
    fn local_library_store_round_trips_accepted_import_per_project() {
        let connection = Connection::open_in_memory().expect("in-memory sqlite opens");
        apply_store_migrations(&connection).expect("store migrations apply");

        let document = storable_material_payload("matlib.invented.alpha");
        let library_id = library_id_from_document(&document, "material").expect("id extracts");
        assert_eq!(library_id, "matlib.invented.alpha");

        upsert_library(
            &connection,
            "project:test-local",
            "material",
            &library_id,
            &document,
        )
        .expect("accepted import stores");
        // A second project keeps its own row under the same library_id.
        upsert_library(
            &connection,
            "project:other-local",
            "material",
            &library_id,
            &document,
        )
        .expect("second project stores independently");

        let loaded = load_library(&connection, "project:test-local", "material", &library_id)
            .expect("library loads")
            .expect("row present");
        assert_eq!(loaded.document, document);

        let entries = list_libraries(&connection, Some("project:test-local")).expect("lists");
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].library_kind, "material");
        assert_eq!(entries[0].library_id, "matlib.invented.alpha");
        assert_eq!(entries[0].library_name, "Invented private material library");
        assert_eq!(entries[0].privacy_class, "private_user_data");
        assert_eq!(entries[0].storage_mode, "local_sqlite");

        let deleted = delete_library(&connection, "project:test-local", "material", &library_id)
            .expect("delete runs");
        assert!(deleted);
        // The other project's row is untouched by the scoped delete.
        assert_eq!(
            list_libraries(&connection, Some("project:other-local"))
                .expect("lists other")
                .len(),
            1
        );
        let deleted_again =
            delete_library(&connection, "project:test-local", "material", &library_id)
                .expect("second delete returns");
        assert!(!deleted_again);
    }

    #[test]
    fn local_library_store_gate_admits_accepted_and_refuses_quarantined_imports() {
        // The persistence gate keys off the import-boundary verdict: an accepted
        // private import is storable; a suspected-protected (QUARANTINE) import
        // is refused so it never lands in the local store.
        let accepted = storable_material_payload("matlib.invented.alpha");
        let accepted_validation =
            library_import_validation_value(&accepted, "material", "private").expect("validates");
        assert_eq!(accepted_validation["outcome"], json!("PRIVATE_LOCAL_ONLY"));
        assert!(library_import_is_storable(&accepted_validation));

        let quarantined = quarantined_material_payload("matlib.invented.suspect");
        let quarantined_validation =
            library_import_validation_value(&quarantined, "material", "private")
                .expect("validates");
        assert_eq!(quarantined_validation["outcome"], json!("QUARANTINE"));
        assert!(!library_import_is_storable(&quarantined_validation));
        let codes: Vec<&str> = quarantined_validation["findings"]
            .as_array()
            .expect("findings array")
            .iter()
            .filter_map(|finding| finding["code"].as_str())
            .collect();
        assert!(codes.contains(&"IMPORT_PROTECTED_CONTENT_SUSPECTED"));
    }

    #[test]
    fn hanger_library_metadata_and_local_storage_preserve_accepted_source() {
        let document: Value = serde_json::from_str(include_str!(
            "../../../../fixtures/hanger/invented_hanger_library_valid.json"
        ))
        .expect("accepted invented hanger fixture parses");
        let validation = library_import_validation_value(&document, "hanger", "private")
            .expect("hanger import validates");
        assert_eq!(validation["outcome"], json!("PRIVATE_LOCAL_ONLY"));
        assert!(library_import_is_storable(&validation));
        let library_id = library_id_from_document(&document, "hanger").expect("hanger id extracts");
        assert_eq!(library_id, "invented.hangers");
        assert!(library_id_from_document(&document, "material").is_err());
        let connection = Connection::open_in_memory().expect("in-memory sqlite opens");
        apply_store_migrations(&connection).expect("store migrations apply");
        upsert_library(&connection, "project:hanger-test", "hanger", &library_id, &document)
            .expect("accepted hanger import stores");
        let loaded = load_library(&connection, "project:hanger-test", "hanger", &library_id)
            .expect("hanger library loads")
            .expect("hanger row present");
        assert_eq!(loaded.document, document);
        let entries = list_libraries(&connection, Some("project:hanger-test")).expect("lists");
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].library_kind, "hanger");
        assert_eq!(entries[0].library_name, "Invented fixture only");
        assert_eq!(entries[0].privacy_class, "private_user_supplied");
    }

    #[test]
    fn library_id_extraction_requires_metadata_and_supported_kind() {
        let document = storable_material_payload("matlib.invented.alpha");
        // A kind/metadata mismatch (section key absent) is rejected, not guessed.
        let mismatch = library_id_from_document(&document, "section")
            .expect_err("section id absent from a material document");
        assert!(mismatch.contains("section_library.library_id"));
        // An unsupported kind is rejected at the metadata-key boundary.
        let unsupported = library_metadata_key("widget").expect_err("unsupported kind must reject");
        assert!(unsupported.contains("widget"));
    }

    /// Every converted command through the shared ingress helper with a raw
    /// args body (`command_ingress::admit_body`), decoded by the command's own
    /// `*_args` function, against the command's plain function.
    mod ingress_commands {
        use super::*;
        use crate::command_ingress::test_support::{admit_json, check_decode_bound, registry};
        use crate::memory_pool::test_host::ScriptedHost;
        use crate::solve_jobs::SolveJobError;
        use tauri::ipc::IpcResponse;

        fn admit(command: &'static str, args: Value) -> (Args, memory_pool::Lease) {
            let registry = registry(ScriptedHost::normal());
            admit_json(&registry, command, ArgsDecoding::Moved, &args).unwrap_or_else(|e| panic!("{command}: {e:?}"))
        }

        fn invalid_args(error: CommandError<impl std::fmt::Debug>) -> String {
            match error {
                CommandError::Ingress(SolveJobError::InvalidRequest { message }) => message,
                other => panic!("not an invalid argument: {other:?}"),
            }
        }

        fn command_error<E: std::fmt::Debug>(error: CommandError<E>) -> E {
            match error {
                CommandError::Command(error) => error,
                other => panic!("not a command error: {other:?}"),
            }
        }

        fn response_json(response: tauri::ipc::Response) -> Value {
            match response.body().unwrap() {
                tauri::ipc::InvokeResponseBody::Json(text) => serde_json::from_str(&text).unwrap(),
                tauri::ipc::InvokeResponseBody::Raw(_) => panic!("raw response"),
            }
        }

        fn scratch_store(label: &str) -> PathBuf {
            let root = std::env::temp_dir().join(format!(
                "swbpipe-ingress-{label}-{}-{}",
                std::process::id(),
                SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()
            ));
            fs::create_dir_all(&root).unwrap();
            root.join(PROJECT_STORE_FILE)
        }

        #[test]
        fn model_operation_and_quantity_commands_decode_raw_bodies() {
            let model = load_preview_model().unwrap();
            let intent = fixture_load_magnitude_intent("350", "425");
            let body = json!({"model": model, "intent": intent, "claimedModelHash": null});
            let (mut args, _lease) = admit("validate_model_operation", body.clone());
            let validated = validate_model_operation_args(&mut args).unwrap();
            assert_eq!(validated, validate_model_operation_value(model.clone(), intent.clone(), None).unwrap());
            let (mut args, _lease) = admit("apply_model_operation", body);
            let applied = apply_model_operation_args(&mut args).unwrap();
            assert_eq!(applied, apply_model_operation_value(model.clone(), intent.clone(), None).unwrap());
            // The response is the same JSON Tauri would have sent.
            let response = respond::<_, String>(Ok(applied.clone())).unwrap();
            assert_eq!(response_json(response), applied);
            // A missing required argument is a typed invalid request.
            let (mut args, _lease) = admit("apply_model_operation", json!({"model": model}));
            let message = invalid_args(apply_model_operation_args(&mut args).unwrap_err());
            assert!(message.contains("invalid args `intent` for command `apply_model_operation`"), "{message}");

            let batch = json!({"batch_id": "batch:test", "operations": [intent]});
            for apply in [false, true] {
                let command = if apply { "apply_model_operation_batch" } else { "validate_model_operation_batch" };
                let (mut args, _lease) = admit(command, json!({"model": model, "batch": batch, "claimedModelHash": {"value": "x"}}));
                let direct = if apply {
                    open_pipe_stress_operation_applier::apply_operation_batch(&model, &batch, Some(&json!({"value": "x"})))
                } else {
                    open_pipe_stress_operation_applier::validate_operation_batch(&model, &batch, Some(&json!({"value": "x"})))
                };
                assert_eq!(model_operation_batch_args(&mut args, apply).unwrap(), direct, "{command}");
            }
            let request = json!({"gravity": "default"});
            let (mut args, _lease) = admit("generate_self_weight_operation_plan", json!({"model": model, "request": request}));
            assert_eq!(
                generate_self_weight_operation_plan_args(&mut args).unwrap(),
                open_pipe_stress_self_weight_wasm::generate_plan(&model, &request)
            );
            let input = json!({"model": model, "quantities": []});
            let (mut args, _lease) = admit("convert_display_quantities", json!({"input": input}));
            assert_eq!(
                convert_display_quantities_args(&mut args).unwrap(),
                open_pipe_stress_operation_applier::convert_display_quantities(&input)
            );
        }

        #[test]
        fn report_rendering_and_sample_proposal_commands_decode_raw_bodies() {
            let broken = json!({"report_title": "broken"});
            let (mut args, _lease) = admit("render_calculation_report", json!({"input": broken}));
            let error = command_error(render_calculation_report_args(&mut args).unwrap_err());
            assert_eq!(Err(error), render_calculation_report_value(broken));

            let result = json!({"result_kind": "invented", "nodes": []});
            let target = json!({"target_type": "node", "id": "N1"});
            let (mut args, _lease) = admit("sample_agent_proposal", json!({"mechanicsResult": result, "selectedTarget": target}));
            let direct = sample_agent_proposal_value(
                Some(result.clone()),
                Some(SelectedReviewTarget { target_type: "node".into(), id: "N1".into() }),
            );
            assert_eq!(sample_agent_proposal_args(&mut args).unwrap(), direct.unwrap());
            let (mut args, _lease) = admit("sample_agent_proposal", json!({"selectedTarget": null}));
            let error = command_error(sample_agent_proposal_args(&mut args).unwrap_err());
            assert!(error.starts_with("PREVIEW-SOLVE-MODEL-REQUIRED"));
            // The command's own failure still serializes as a plain string.
            assert_eq!(serde_json::to_value(CommandError::<String>::Command(error.clone())).unwrap(), json!(error));
            let (mut args, _lease) = admit("sample_agent_proposal", json!({"mechanicsResult": result, "selectedTarget": "N1"}));
            let message = invalid_args(sample_agent_proposal_args(&mut args).unwrap_err());
            assert!(message.contains("`selectedTarget`") && message.contains("invalid type: string"), "{message}");
        }

        #[test]
        fn project_store_commands_decode_raw_bodies() {
            let store = scratch_store("projects");
            let mut model = load_preview_model().unwrap();
            model["project"]["id"] = json!("project:ingress-create");
            let (mut args, _lease) = admit("create_local_project", json!({"model": model, "confirmedRevision": null}));
            let created = create_local_project_args(&mut args, || Ok(store.clone())).unwrap();
            assert_eq!(created.model["project"]["id"], json!("project:ingress-create"));
            assert_eq!(serde_json::to_value(&created).unwrap()["editor_intents"], json!([]));

            let mut saved_model = model.clone();
            saved_model["project"]["id"] = json!("project:ingress-save");
            let request = json!({"project_id": "project:ingress-save", "project_name": "Saved", "model": saved_model, "unknown": 1});
            let (mut args, _lease) = admit("save_local_project", json!({"request": request}));
            let saved = save_local_project_args(&mut args, || Ok(store.clone())).unwrap();
            assert_eq!(saved.model["project"]["id"], json!("project:ingress-save"));
            assert_eq!(saved.summary.project_name, "Saved");
            // The command's own refusal (an unconfirmed replacement) stays a plain string.
            let (mut args, _lease) = admit("save_local_project", json!({"request": request}));
            let refused = command_error(save_local_project_args(&mut args, || Ok(store.clone())).unwrap_err());
            assert!(refused.starts_with("PROJECT-ID-EXISTS"), "{refused}");
            let (mut args, _lease) = admit("create_local_project", json!({"model": model, "confirmedRevision": 7}));
            let message = invalid_args(create_local_project_args(&mut args, || Ok(store.clone())).unwrap_err());
            assert!(message.contains("`confirmedRevision`"), "{message}");
            fs::remove_dir_all(store.parent().unwrap()).unwrap();
        }

        /// `SaveLocalProjectRequest::from_value` keeps the derived decoding's
        /// semantics while moving its documents.
        #[test]
        fn save_local_project_request_decoding_keeps_its_field_semantics() {
            let request = SaveLocalProjectRequest::from_value(json!({
                "project_id": "p", "project_name": "n", "model": {"m": 1}, "proposal": null,
                "confirmed_revision": null, "expected_base": "h", "extra": [1]
            }))
            .unwrap();
            assert_eq!((request.project_id.as_str(), request.project_name.as_str()), ("p", "n"));
            assert_eq!(request.model, json!({"m": 1}));
            assert_eq!((request.editor_intents, request.proposal), (Value::Null, Value::Null));
            assert_eq!((request.confirmed_revision, request.expected_base.as_deref()), (None, Some("h")));
            for (value, expected) in [
                (json!({"project_name": "n", "model": {}}), "missing field `project_id`"),
                (json!({"project_id": "p", "project_name": "n"}), "missing field `model`"),
                (json!({"project_id": 1, "project_name": "n", "model": {}}), "invalid type: integer `1`, expected a string"),
                (json!({"project_id": "p", "project_name": "n", "model": {}, "expected_base": []}), "invalid type: sequence"),
                (json!("text"), "invalid type: string \"text\", expected struct SaveLocalProjectRequest"),
            ] {
                let error = SaveLocalProjectRequest::from_value(value).unwrap_err().to_string();
                assert!(error.starts_with(expected), "{error}");
            }
        }

        #[test]
        fn rule_pack_and_rule_check_commands_decode_raw_bodies() {
            let document = example_rule_pack_document();
            let (mut args, _lease) = admit("validate_rule_pack", json!({"document": document, "publicExportRequested": true}));
            assert_eq!(validate_rule_pack_args(&mut args).unwrap(), validate_rule_pack_value(document.clone(), Some(true)).unwrap());
            let (mut args, _lease) = admit("validate_rule_pack", json!({"document": document, "publicExportRequested": "yes"}));
            assert!(invalid_args(validate_rule_pack_args(&mut args).unwrap_err()).contains("`publicExportRequested`"));
            let (mut args, _lease) = admit("compute_rule_pack_document_checksum", json!({"document": document}));
            assert_eq!(
                compute_rule_pack_document_checksum_args(&mut args).unwrap(),
                compute_rule_pack_document_checksum_value(document.clone()).unwrap()
            );
            let store = scratch_store("rule-packs");
            let (mut args, _lease) = admit("save_local_rule_pack", json!({"projectId": " project:ingress ", "document": document}));
            let saved = save_local_rule_pack_args(&mut args, || Ok(store.clone())).unwrap();
            assert_eq!((saved.project_id.as_str(), saved.document.clone()), ("project:ingress", document.clone()));
            let (mut args, _lease) = admit("save_local_rule_pack", json!({"projectId": " ", "document": document}));
            assert_eq!(command_error(save_local_rule_pack_args(&mut args, || Ok(store.clone())).unwrap_err()), "project_id must be non-empty");

            let (mut args, _lease) =
                admit("run_rule_checks", json!({"rulePackDocument": document, "solvedEnvelope": {}, "projectId": null}));
            let error = command_error(run_rule_checks_args(&mut args, &|| Ok(store.clone())).unwrap_err());
            assert!(error.starts_with("RULE_NUMERICAL_CASE_COVERAGE_UNAVAILABLE"), "{error}");
            let (mut args, _lease) = admit("run_rule_checks", json!({"rulePackDocument": document, "projectId": 5}));
            assert!(invalid_args(run_rule_checks_args(&mut args, &|| Ok(store.clone())).unwrap_err()).contains("`projectId`"));
            fs::remove_dir_all(store.parent().unwrap()).unwrap();
        }

        #[test]
        fn library_commands_decode_raw_bodies() {
            let payload = valid_private_material_payload();
            let (mut args, _lease) =
                admit("validate_library_import", json!({"payload": payload, "libraryKind": "material", "intendedVisibility": "private"}));
            assert_eq!(
                validate_library_import_args(&mut args).unwrap(),
                validate_library_import_value(payload.clone(), "material".into(), "private".into()).unwrap()
            );
            let store = scratch_store("libraries");
            let document = storable_material_payload("matlib.invented.ingress");
            let (mut args, _lease) =
                admit("save_local_library", json!({"projectId": "project:ingress", "libraryKind": "material", "document": document}));
            let saved = save_local_library_args(&mut args, || Ok(store.clone())).unwrap();
            assert!(saved.stored);
            assert_eq!(saved.library_id, "matlib.invented.ingress");
            let (mut args, _lease) = admit("save_local_library", json!({"projectId": "project:ingress", "document": document}));
            assert!(invalid_args(save_local_library_args(&mut args, || Ok(store.clone())).unwrap_err()).contains("`libraryKind`"));
            fs::remove_dir_all(store.parent().unwrap()).unwrap();
        }

        fn witness_report_request() -> Value {
            let witness: Value = serde_json::from_str(include_str!(
                "../../../../validation/witness/inputs/del1005_export_results_success_input.json"
            ))
            .unwrap();
            witness["export_results"].clone()
        }

        fn redaction_evidence() -> Value {
            json!({"route_id": "DREP-PACKAGE-SAVE-009", "decision_count": 1, "finding_count": 0, "blocking_count": 0})
        }

        fn local_first_evidence() -> Value {
            json!({
                "route_id": "DREP-PACKAGE-SAVE-009", "export_context": "local_private", "storage_context": "local_private",
                "action": "include_metadata_only", "reason_code": "PRIVATE_LOCAL_METADATA_ALLOWED", "blocked": false,
                "metadata_only": true, "explicit_local_private_intent": true
            })
        }

        /// W-18 slice (a) native hold (DEL-08-01 Design §2.6): an otherwise
        /// valid 2.0.0 request with clean caller-supplied evidence, decoded
        /// through the command's own argument path, is refused before the save
        /// dialog or any write. Control: the shared producer alone assembles
        /// the same request unblocked, so the refusal is the hold's. Slice (b1)
        /// replaces this test with the screened admission's.
        #[test]
        fn report_package_save_is_held_until_native_screening() {
            let request = witness_report_request();
            let direct: report_package_bridge::ReportPackageRequest = serde_json::from_value(request.clone()).unwrap();
            let control = report_package_bridge::assemble_request(direct).expect("the witness request assembles");
            assert!(!control.export_blocked, "{:?}", control.blocking_reasons);

            let registry = registry(ScriptedHost::normal());
            let decoding = ArgsDecoding::Rebuilt { vec_expansion: command_ingress::REPORT_PACKAGE_VEC_EXPANSION };
            let body = json!({"request": request, "redactionEvidence": redaction_evidence(), "localFirstEvidence": local_first_evidence()});
            let (mut args, _lease) = admit_json(&registry, "save_report_package", decoding, &body).unwrap();
            let (decoded, redaction, local_first) = save_report_package_args(&mut args).unwrap();
            let sandbox = std::env::temp_dir().join(format!("ops-w18a-hold-{}", std::process::id()));
            fs::create_dir_all(&sandbox).unwrap();
            let mut picks = 0usize;
            let outcome = save_report_package_with_picker(decoded, redaction, local_first, |_| {
                picks += 1;
                Some(Ok(sandbox.join("held.opsproj")))
            });
            let error = match outcome {
                Err(error) => error,
                Ok(receipt) => panic!("the hold must refuse, got {}", receipt.code),
            };
            assert_eq!(error.code, "REPORT-PACKAGE-FRESH-RESULT-UNAVAILABLE");
            assert_eq!(error.message, REPORT_PACKAGE_SAVE_HOLD);
            assert_eq!(error.stage, "assemble");
            assert_eq!(picks, 0, "no save dialog is requested");
            assert_eq!(fs::read_dir(&sandbox).unwrap().count(), 0, "nothing is written");
            fs::remove_dir_all(&sandbox).unwrap();
        }

        #[test]
        fn report_package_command_decodes_its_typed_request() {
            let request = witness_report_request();
            let registry = registry(ScriptedHost::normal());
            let decoding = ArgsDecoding::Rebuilt { vec_expansion: command_ingress::REPORT_PACKAGE_VEC_EXPANSION };
            let body = json!({"request": request, "redactionEvidence": redaction_evidence(), "localFirstEvidence": local_first_evidence()});
            let (mut args, _lease) = admit_json(&registry, "save_report_package", decoding, &body).unwrap();
            let (decoded, redaction, local_first) = save_report_package_args(&mut args).unwrap();
            assert_eq!(redaction.route_id, "DREP-PACKAGE-SAVE-009");
            validate_report_package_local_first_evidence(local_first.as_ref()).unwrap();
            // The same typed request serde_json's own decoding gives.
            let direct: report_package_bridge::ReportPackageRequest = serde_json::from_value(request.clone()).unwrap();
            assert_eq!(format!("{decoded:?}"), format!("{direct:?}"));
            // Absent local-first evidence decodes as None (the command then refuses it).
            let (mut args, _lease) =
                admit_json(&registry, "save_report_package", decoding, &json!({"request": request, "redactionEvidence": redaction_evidence()}))
                    .unwrap();
            assert!(save_report_package_args(&mut args).unwrap().2.is_none());
            let (mut args, _lease) = admit_json(&registry, "save_report_package", decoding, &json!({"request": {}})).unwrap();
            let message = match save_report_package_args(&mut args).unwrap_err() {
                SolveJobError::InvalidRequest { message } => message,
                other => panic!("{other:?}"),
            };
            assert!(message.contains("`request`") && message.contains("missing field"), "{message}");
        }

        #[test]
        fn local_result_save_command_decodes_its_typed_request() {
            let dir = scratch_store("result-save").parent().unwrap().to_path_buf();
            let text = "{\"value\":350}";
            let href = format!(
                "data:application/json;charset=utf-8,{}",
                text.bytes().map(|b| format!("%{b:02X}")).collect::<String>()
            );
            let request = json!({
                "href": href, "file_name": "openpipestress-preview-results-run-1.json",
                "screening": {"route_id": "DOTH-JSON-001", "export_context": "local_private", "explicit_local_private_intent": true,
                    "blocked": false, "materialization_withheld": false, "lossless_required": true, "exact_payload_match": true,
                    "blocking_count": 0},
                "local_first": {"route_id": "DOTH-JSON-001", "export_context": "local_private", "storage_context": "local_private",
                    "action": "include_metadata_only", "reason_code": "PRIVATE_LOCAL_METADATA_ALLOWED", "blocked": false,
                    "metadata_only": true, "explicit_local_private_intent": true}
            });
            let (mut args, _lease) = admit("save_local_result_json", json!({"request": request}));
            let decoded = save_local_result_json_args(&mut args).unwrap();
            let receipt = native_result_download::save_request(decoded, || Ok::<_, ()>(dir.clone())).unwrap();
            assert_eq!(fs::read(dir.join(&receipt.file_name)).unwrap(), text.as_bytes());
            // deny_unknown_fields still refuses, with a bounded message.
            let mut unknown = request.clone();
            unknown["directory"] = json!("/tmp");
            let (mut args, _lease) = admit("save_local_result_json", json!({"request": unknown}));
            let message = match save_local_result_json_args(&mut args).unwrap_err() {
                SolveJobError::InvalidRequest { message } => message,
                other => panic!("{other:?}"),
            };
            assert!(message.contains("unknown field `directory`"), "{message}");
            fs::remove_dir_all(&dir).unwrap();
        }

        /// The typed decodings of the moved commands allocate within their
        /// lease for representative and adversarial bodies (long strings in
        /// refused positions, long valid strings).
        #[test]
        fn typed_argument_decodings_stay_within_the_moved_lease() {
            let long = "\u{85}\u{1}".repeat(30_000);
            let model = load_preview_model().unwrap();
            let project_bodies = [
                json!({"request": {"project_id": "p", "project_name": "n", "model": model, "mechanics_result": {"r": [1.5, 2.5]}}}),
                json!({"request": long.clone()}),
                json!({"request": {"project_id": long.clone(), "project_name": [long.clone()], "model": {}}}),
                json!({"request": {"project_id": "p", "project_name": "n", "model": {}, "confirmed_revision": {"k": long.clone()}}}),
            ];
            for body in &project_bodies {
                let raw = serde_json::to_vec(body).unwrap();
                check_decode_bound(&raw, ArgsDecoding::Moved, |args| {
                    let _ = save_local_project_args(args, || Err("no store in the bound test".to_string()));
                });
            }
            for target in [json!({"target_type": "node", "id": long.clone()}), json!(long.clone()), json!([long.clone()]), json!({"id": 1})] {
                let raw = serde_json::to_vec(&json!({"mechanicsResult": {"a": 1}, "selectedTarget": target})).unwrap();
                check_decode_bound(&raw, ArgsDecoding::Moved, |args| {
                    let _ = args.decode::<Option<SelectedReviewTarget>>("selectedTarget");
                });
            }
            let decoding = ArgsDecoding::Rebuilt { vec_expansion: command_ingress::REPORT_PACKAGE_VEC_EXPANSION };
            let mut refused_evidence = redaction_evidence();
            refused_evidence["blocking_count"] = json!(long.clone());
            let mut refused_local_first = local_first_evidence();
            refused_local_first["blocked"] = json!(long.clone());
            for (redaction, local_first) in [
                (redaction_evidence(), local_first_evidence()),
                (refused_evidence, local_first_evidence()),
                (redaction_evidence(), refused_local_first),
            ] {
                let body = json!({"request": witness_report_request(), "redactionEvidence": redaction, "localFirstEvidence": local_first});
                let raw = serde_json::to_vec(&body).unwrap();
                let _ = check_decode_bound(&raw, decoding, |args| save_report_package_args(args).map(drop));
            }
            let screening = json!({"route_id": "DOTH-JSON-001", "export_context": "local_private",
                "explicit_local_private_intent": true, "blocked": false, "materialization_withheld": false,
                "lossless_required": true, "exact_payload_match": true, "blocking_count": 0});
            for (href, blocked) in [(json!(format!("data:application/json;charset=utf-8,{}", "%7B%7D".repeat(50_000))), json!(false)), (json!("x"), json!(long.clone()))] {
                let mut screening = screening.clone();
                screening["blocked"] = blocked;
                let request = json!({"href": href, "file_name": "f.json", "screening": screening, "local_first": screening});
                let raw = serde_json::to_vec(&json!({"request": request})).unwrap();
                let _ = check_decode_bound(&raw, ArgsDecoding::Moved, |args| save_local_result_json_args(args).map(drop));
            }
        }
    }

    /// The node-proximity tolerance control's path (DEL-02-01 Design §3.8;
    /// DEL-04-06 Design `model-topology-checks.md`): the desktop applies
    /// `set_model_setting` / `clear_model_setting` through the apply command, and
    /// the native solve reads the result. An anchored node 3 mm from node:N-140
    /// warns at the 5 mm default and not at a 2 mm tolerance; one 8 mm away warns
    /// at 1 cm (10 mm) and, reset to the default, does not. Invented data only.
    #[test]
    fn a_tolerance_set_through_the_apply_command_changes_which_nodes_warn() {
        fn with_near_node(gap_m: f64) -> Value {
            let mut model = read_fixture("invented_demo_model.json").expect("bundled preview model loads");
            assert!(model.get("model_settings").is_none());
            let position = model["nodes"].as_array().unwrap().iter().find(|node| node["id"] == "node:N-140").unwrap()["position"].clone();
            let y = position["y"].as_f64().unwrap() + gap_m;
            model["nodes"].as_array_mut().unwrap().push(json!({"id":"node:near","position":{"x":position["x"],"y":y,"z":position["z"]},"provenance":"invented_test_node"}));
            model["supports"].as_array_mut().unwrap().push(json!({"id":"support:near","node":"node:near","family":"anchor","restraints":["UX","UY","UZ","RX","RY","RZ"],"provenance":"invented_test_support"}));
            model
        }
        fn intent(model: &Value, change_kind: &str, after: &str, unit: &str, dimension: &str) -> Value {
            let before = model
                .pointer("/model_settings/node_proximity_tolerance")
                .map(open_pipe_stress_operation_applier::canonical_json)
                .unwrap_or_else(|| "not_present".into());
            json!({
                "operation_id": format!("op:model-settings:{change_kind}"), "operation_kind": "modify", "operation_status": "proposed",
                "author_type": "user", "target": {"object_type": "Project", "ref": model["project"]["id"]},
                "change": {"change_id": format!("change:model-settings:{change_kind}"), "change_kind": change_kind,
                    "field_label": "Node-proximity tolerance", "field_path": "model_settings.node_proximity_tolerance",
                    "before": before, "after": after, "unit": unit, "dimension": dimension, "source_note": "invented test input"},
                "validation": {"schema_validation":"not_run","constraint_validation":"not_run","unit_validation":"not_run",
                    "diff_preview_status":"not_generated","application_status":"not_applied"},
                "audit_boundary": {"mutation_route":"structured_operations_only","direct_model_mutation_allowed":false,
                    "requires_user_acceptance":true,"mutates_accepted_model_state":false},
                "professional_boundary": {"human_review_required":true,"software_makes_compliance_claim":false,
                    "software_makes_certification_claim":false,"software_makes_sealing_claim":false,
                    "software_makes_approval_claim":false,"software_makes_authentication_claim":false},
                "rationale": "Invented node-proximity tolerance test"
            })
        }
        fn applied(model: &Value, intent: Value) -> Value {
            let outcome = apply_model_operation_value(model.clone(), intent, None).expect("command returns outcome");
            assert_eq!(outcome["validation"]["application_status"], "applied_to_session_model", "{}", outcome["diagnostics"]);
            outcome["applied_model"].clone()
        }
        fn near_pairs(model: &Value, mode: PreviewSolverMode) -> Vec<Value> {
            let solved = solve_preview_mechanics_value_with_mode(model.clone(), mode).expect("solve returns an envelope");
            let diagnostics = solved["diagnostics"].as_array().cloned().unwrap_or_default();
            assert!(diagnostics.iter().all(|item| item["severity"] != "blocking"), "{diagnostics:?}");
            diagnostics
                .into_iter()
                .filter(|item| item["code"] == "NODE_NEAR_DUPLICATE")
                .map(|item| item["affected_refs"].clone())
                .collect()
        }
        let pair = json!(["node:N-140", "node:near"]);
        for mode in [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny] {
            let three_mm = with_near_node(0.003);
            assert_eq!(near_pairs(&three_mm, mode), vec![pair.clone()], "3 mm warns at the default");
            let two_mm_tolerance = applied(&three_mm, intent(&three_mm, "set_model_setting", "2", "mm", "length"));
            assert_eq!(two_mm_tolerance["model_settings"], json!({"node_proximity_tolerance": {"value": 2.0, "unit": "mm"}}));
            assert!(near_pairs(&two_mm_tolerance, mode).is_empty(), "3 mm does not warn at 2 mm");

            let eight_mm = with_near_node(0.008);
            assert!(near_pairs(&eight_mm, mode).is_empty(), "8 mm does not warn at the default");
            let one_cm_tolerance = applied(&eight_mm, intent(&eight_mm, "set_model_setting", "10", "mm", "length"));
            assert_eq!(near_pairs(&one_cm_tolerance, mode), vec![pair.clone()], "8 mm warns at 1 cm");
            let reset = applied(&one_cm_tolerance, intent(&one_cm_tolerance, "clear_model_setting", "not_present", "none", "dimensionless"));
            assert!(reset.get("model_settings").is_none(), "reset never writes the default");
            assert!(near_pairs(&reset, mode).is_empty(), "reset to the default, 8 mm does not warn");
        }
    }
}

#[cfg(test)]
mod replace_confirmation_tests {
    use super::*;

    fn scratch_store(label: &str) -> PathBuf {
        let root = std::env::temp_dir().join(format!(
            "swbpipe-{label}-{}-{}",
            std::process::id(),
            SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()
        ));
        fs::create_dir_all(&root).unwrap();
        let store = root.join(PROJECT_STORE_FILE);
        drop(open_project_store(&store).unwrap());
        store
    }

    fn invented_model(id: &str, name: &str) -> Value {
        let mut model = load_preview_model().unwrap();
        model["project"]["id"] = json!(id);
        model["project"]["name"] = json!(name);
        model
    }

    /// An unconfirmed write through the commands' own store path, alternating
    /// create and save as the two writers' commands.
    fn unconfirmed_write(store: &Path, id: &str, name: &str, save: bool) -> Result<(), String> {
        let model = invented_model(id, name);
        if save {
            let request = SaveLocalProjectRequest::from_value(json!({
                "project_id": id, "project_name": name, "model": model
            })).unwrap();
            save_project_in_store(store.to_path_buf(), request).map(|_| ())
        } else {
            create_project_in_store(store.to_path_buf(), model, None, None, None, None, None, None, None, None).map(|_| ())
        }
    }

    /// Writer A holds the store's write lock between its check and its write.
    /// Writer B, a second connection as another window or process would hold,
    /// cannot run its check meanwhile: it waits, then sees A's row and is refused.
    /// (With the check outside the transaction, B's check would read the empty
    /// store while A held the lock, and B would then overwrite A's project.)
    #[test]
    fn a_second_writer_cannot_pass_the_check_while_the_first_is_writing() {
        let store = scratch_store("check-and-write");
        let id = "project:invented-race";
        let (mut first, _) = open_project_store(&store).unwrap();
        let transaction = first.transaction_with_behavior(rusqlite::TransactionBehavior::Immediate).unwrap();
        refuse_unconfirmed_replacement(&transaction, id, None, None).expect("A: nothing stored yet");
        let second_store = store.clone();
        let second = std::thread::spawn(move || unconfirmed_write(&second_store, id, "Writer B", false));
        std::thread::sleep(std::time::Duration::from_millis(300));
        assert!(!second.is_finished(), "B must wait for A's write lock before it can check");
        write_project_rows(&transaction, id, "Writer A", &invented_model(id, "Writer A"), &json!([]), &Value::Null,
            &Value::Null, &Value::Null, &Value::Null, &Value::Null, &Value::Null).unwrap();
        transaction.commit().unwrap();
        let refused = second.join().unwrap().expect_err("B is refused");
        assert!(refused.starts_with("PROJECT-ID-EXISTS: project `project:invented-race` (\"Writer A\""), "{refused}");
        let (connection, _) = open_project_store(&store).unwrap();
        assert_eq!(stored_project_entry(&connection, id).unwrap().unwrap().project_name, "Writer A");
        fs::remove_dir_all(store.parent().unwrap()).unwrap();
    }

    /// Two writers race for the same new id: exactly one writes; the other is refused.
    #[test]
    fn two_concurrent_writers_to_the_same_new_id_never_both_write() {
        for round in 0..20 {
            let store = scratch_store("concurrent-writers");
            let id = format!("project:invented-race-{round}");
            let barrier = Arc::new(std::sync::Barrier::new(2));
            let writers: Vec<_> = [("Writer A", false), ("Writer B", round % 2 == 1)].into_iter().map(|(name, save)| {
                let (store, id, barrier) = (store.clone(), id.clone(), Arc::clone(&barrier));
                std::thread::spawn(move || { barrier.wait(); unconfirmed_write(&store, &id, name, save).map(|()| name) })
            }).collect();
            let results: Vec<_> = writers.into_iter().map(|writer| writer.join().unwrap()).collect();
            let written: Vec<_> = results.iter().filter_map(|result| result.as_ref().ok().copied()).collect();
            assert_eq!(written.len(), 1, "round {round}: {results:?}");
            assert!(results.iter().any(|result| matches!(result, Err(error) if error.starts_with("PROJECT-ID-EXISTS"))), "round {round}: {results:?}");
            let (connection, _) = open_project_store(&store).unwrap();
            assert_eq!(stored_project_entry(&connection, &id).unwrap().unwrap().project_name, written[0]);
            fs::remove_dir_all(store.parent().unwrap()).unwrap();
        }
    }

    #[test]
    fn a_stored_project_is_never_replaced_without_confirmation() {
        let root = std::env::temp_dir().join(format!(
            "swbpipe-replace-confirmation-{}-{}",
            std::process::id(),
            SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()
        ));
        fs::create_dir_all(&root).unwrap();
        let (mut connection, _) = open_project_store(&root.join(PROJECT_STORE_FILE)).unwrap();
        let model = load_preview_model().unwrap();
        let id = model["project"]["id"].as_str().unwrap().to_string();
        // Nothing stored yet: a first save needs no confirmation.
        refuse_unconfirmed_replacement(&connection, &id, None, None).expect("first save");
        upsert_project(&mut connection, &id, "Invented demo", &model, &json!([]), &Value::Null, &Value::Null,
            &Value::Null, &Value::Null, &Value::Null, &Value::Null).unwrap();
        let refused = refuse_unconfirmed_replacement(&connection, &id, None, None).expect_err("unconfirmed replacement");
        assert!(refused.starts_with("PROJECT-ID-EXISTS: project `project:invented-demo-loop-01` (\"Invented demo\", last saved at unix "), "{refused}");
        let entry = stored_project_entry(&connection, &id).unwrap().unwrap();
        let shown = stored_revision(&connection, &entry).unwrap();
        refuse_unconfirmed_replacement(&connection, &id, Some(&shown), None).expect("Replace of the revision shown");
        refuse_unconfirmed_replacement(&connection, "project:another", None, None).expect("another id");
        // A save in place is admitted only from the stored revision.
        let base = json!({"algorithm": "sha256", "value": "sha256:base"});
        upsert_project(&mut connection, &id, "Invented demo", &model, &json!([]), &Value::Null, &Value::Null,
            &Value::Null, &Value::Null, &Value::Null, &base).unwrap();
        refuse_unconfirmed_replacement(&connection, &id, None, Some("sha256:base")).expect("save in place");
        let moved = refuse_unconfirmed_replacement(&connection, &id, None, Some("sha256:older")).expect_err("stale base");
        assert!(moved.starts_with("PROJECT-BASE-MOVED: project `project:invented-demo-loop-01` (\"Invented demo\", last saved at unix "), "{moved}");
        // A Replace confirms the revision the prompt showed; one saved since is not confirmed.
        let entry = stored_project_entry(&connection, &id).unwrap().unwrap();
        let current = stored_revision(&connection, &entry).unwrap();
        assert!(current.starts_with(&format!("sha256:base@{}@", entry.updated_at_unix)), "{current}");
        // Two different writes in the same second have different revisions (the
        // row fingerprint), even without an envelope hash.
        upsert_project(&mut connection, &id, "Invented demo", &model, &json!(["edited"]), &Value::Null, &Value::Null,
            &Value::Null, &Value::Null, &Value::Null, &base).unwrap();
        let after_edit = stored_revision(&connection, &stored_project_entry(&connection, &id).unwrap().unwrap()).unwrap();
        assert_ne!(after_edit, current);
        upsert_project(&mut connection, &id, "Invented demo", &model, &json!([]), &Value::Null, &Value::Null,
            &Value::Null, &Value::Null, &Value::Null, &base).unwrap();
        let current = stored_revision(&connection, &stored_project_entry(&connection, &id).unwrap().unwrap()).unwrap();
        refuse_unconfirmed_replacement(&connection, &id, Some(&current), Some("sha256:older")).expect("Replace of the shown revision");
        let replaced_unseen = refuse_unconfirmed_replacement(&connection, &id, Some(&shown), None).expect_err("revision saved after the prompt");
        assert!(replaced_unseen.starts_with("PROJECT-BASE-MOVED") && replaced_unseen.contains("after the replace prompt showed it"), "{replaced_unseen}");
        upsert_project(&mut connection, &id, "Invented demo", &model, &json!([]), &Value::Null, &Value::Null,
            &Value::Null, &Value::Null, &Value::Null, &Value::Null).unwrap();
        assert!(refuse_unconfirmed_replacement(&connection, &id, None, Some("")).unwrap_err().starts_with("PROJECT-BASE-MOVED"), "no stored hash: asks");
        let entry = stored_project_entry(&connection, &id).unwrap().expect("entry");
        assert_eq!((entry.project_id.as_str(), entry.project_name.as_str()), (id.as_str(), "Invented demo"));
        assert!(stored_project_entry(&connection, "project:another").unwrap().is_none());
        drop(connection);
        fs::remove_dir_all(&root).unwrap();
    }
}
