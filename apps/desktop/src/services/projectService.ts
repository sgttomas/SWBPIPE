import { invoke } from "@tauri-apps/api/core";
import { invokeWithIngress } from "./nativeIngress";
import { MODEL_SCHEMA_VERSION, RETIRED_MODEL_SCHEMA_VERSIONS } from "./modelSchemaVersion";
import { loadWasmEngine } from "./wasmEngine/loadWasmEngine";
import { modelDocumentSlotValidation } from "../features/model-file/modelDocumentShape";
import type {
  AgentProposal,
  AnalysisRunEnvelope,
  EditorOperationIntent,
  LocalProjectEnvelope,
  ModelDocumentVersionStatus,
  LocalProjectIndexEntry,
  LocalStorageCapability,
  MechanicsResult,
  ModelHashEvidence,
  PreviewModel,
  ProjectEnvelopeHashEvidence,
  SelectedReviewTarget
} from "../types";

// The browser preview's in-memory project store: one snapshot per project id,
// like the native SQLite store (HELP_HUMAN rulings 2026-10-10: saving one
// project never discards another). Created time is kept across writes, as the
// native upsert keeps it. "Most recent" follows write order.
type BrowserStoredProject = {
  envelope: LocalProjectEnvelope;
  createdAtUnix: number;
  updatedAtUnix: number;
  writeSequence: number;
};
const browserPreviewStore = new Map<string, BrowserStoredProject>();
let browserWriteSequence = 0;

function storeBrowserProject(stored: LocalProjectEnvelope): LocalProjectEnvelope {
  const nowUnix = Math.floor(Date.now() / 1000);
  const prior = browserPreviewStore.get(stored.summary.project_id);
  browserPreviewStore.set(stored.summary.project_id, {
    envelope: stored,
    createdAtUnix: prior?.createdAtUnix ?? nowUnix,
    updatedAtUnix: nowUnix,
    writeSequence: ++browserWriteSequence
  });
  return cloneEnvelope(stored);
}

function browserProjectsByRecency(): BrowserStoredProject[] {
  return [...browserPreviewStore.values()].sort((a, b) => b.writeSequence - a.writeSequence);
}

function hasTauriRuntime(): boolean {
  return typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;
}

// DEL-02-05 CLM-003 (b7-plan "Current model format"): MODEL_SCHEMA_VERSION is
// the one current model document version; its pressure_contract is optional and
// its absence makes the document pressure-free. Every other version is refused by
// name and nothing is migrated. Mirrors apps/desktop/src-tauri/src/model_document_version.rs.
export { MODEL_SCHEMA_VERSION, RETIRED_MODEL_SCHEMA_VERSIONS } from "./modelSchemaVersion";

function parseSemver(raw: string): [number, number, number] | null {
  const match = /^(\d+)\.(\d+)\.(\d+)$/.exec(raw);
  if (!match) return null;
  return [Number(match[1]), Number(match[2]), Number(match[3])];
}

// The one version evaluator (CLM-003): current, retired_schema (an older
// version, re-author), newer_than_supported or unsupported_schema. The
// document is never rewritten.
export function evaluateModelDocumentLocal(model: PreviewModel): ModelDocumentVersionStatus {
  const raw = typeof model.schema_version === "string" ? model.schema_version : "";
  if (raw === MODEL_SCHEMA_VERSION) {
    return {
      status: "current",
      source_schema_version: raw,
      target_schema_version: raw,
      detail:
        `Model document ${MODEL_SCHEMA_VERSION} is the current version; a pressure contract, when present, and solve eligibility are validated separately.`
    };
  }
  const version = parseSemver(raw);
  const current = parseSemver(MODEL_SCHEMA_VERSION)!;
  if (!version) {
    return {
      status: "unsupported_schema",
      source_schema_version: raw || "missing_schema_version",
      target_schema_version: MODEL_SCHEMA_VERSION,
      detail: "Model document has no valid semver schema_version; refusing to open it for editing without coercion."
    };
  }
  const compare = version[0] - current[0] || version[1] - current[1] || version[2] - current[2];
  if (compare > 0) {
    return {
      status: "newer_than_supported",
      source_schema_version: raw,
      target_schema_version: MODEL_SCHEMA_VERSION,
      detail: `Model document schema_version ${raw} is newer than the current ${MODEL_SCHEMA_VERSION}; refusing to open it for editing (no down-migration).`
    };
  }
  const detail = RETIRED_MODEL_SCHEMA_VERSIONS.includes(raw)
    ? `model document ${raw} is retired; re-author it as model document ${MODEL_SCHEMA_VERSION} (a pressure-free model omits pressure_contract). Nothing is migrated.`
    : `model document ${raw} is not a supported version; re-author it as model document ${MODEL_SCHEMA_VERSION}. Nothing is migrated.`;
  return {
    status: "retired_schema",
    source_schema_version: raw,
    target_schema_version: MODEL_SCHEMA_VERSION,
    detail: `MODEL_SCHEMA_VERSION_RETIRED: ${detail}`
  };
}

export function modelDocumentVersionCheckStatus(
  model: PreviewModel,
  evaluatedStatus?: ModelDocumentVersionStatus["status"]
): ModelDocumentVersionStatus["status"] {
  return evaluatedStatus ?? evaluateModelDocumentLocal(model).status;
}

export function modelDocumentVersionCheckDiagnostic(
  status: ModelDocumentVersionStatus["status"]
): { code: string; severity: "warning" | "blocking"; message: string } | null {
  if (status === "current") return null;
  if (status === "retired_schema") {
    return {
      code: "PROJECT-VALIDATION-RETIRED-SCHEMA",
      severity: "blocking",
      message: `Project schema version is retired; re-author the model as model document ${MODEL_SCHEMA_VERSION}. Nothing is migrated.`
    };
  }
  if (status === "newer_than_supported") {
    return {
      code: "PROJECT-VALIDATION-NEWER-THAN-SUPPORTED-SCHEMA",
      severity: "blocking",
      message: `Project schema version is newer than the current model document ${MODEL_SCHEMA_VERSION}; the document is refused because no down-migration is allowed.`
    };
  }
  return {
    code: "PROJECT-VALIDATION-UNSUPPORTED-SCHEMA",
    severity: "blocking",
    message: `Project schema version is not a valid semver version; the current model document is ${MODEL_SCHEMA_VERSION}.`
  };
}

function requireEditableModelDocument(model: PreviewModel): ModelDocumentVersionStatus {
  const status = evaluateModelDocumentLocal(model);
  if (status.status !== "current") {
    throw new Error(
      `Model document refused for editing: status=${status.status}; source_schema_version=${status.source_schema_version}; target_schema_version=${status.target_schema_version}; ${status.detail}`
    );
  }
  refuseMalformedModelSlots(model);
  return status;
}

/** W-13 §5.3 (S2): the browser-preview project store refuses to create, save or
 * open a document whose model-data slots carry a blocking validator finding,
 * named with its JSON path, as the native store does in src-tauri
 * (`refuse_malformed_model_slots`). The check is `modelDocumentSlotValidation`,
 * which also refuses, by name, when the wasm engine that holds the validator is
 * not loaded or reports an input error: the store never adopts an unchecked
 * document. */
export function refuseMalformedModelSlots(model: PreviewModel): void {
  const { blocking } = modelDocumentSlotValidation(model);
  if (blocking.length > 0) {
    throw new Error(`Model document refused: malformed model-data slots: ${blocking.map((item) => `${item.code}${item.path ? ` at ${item.path}` : ""}: ${item.message}`).join(" | ")}`);
  }
}

/** Load the wasm engine (the validator's home) before a browser-preview store
 * check; a failed load leaves the engine absent, which the check then names. */
async function loadValidatorEngine(): Promise<void> {
  try {
    await loadWasmEngine();
  } catch {
    // Absence is reported by refuseMalformedModelSlots as WASM-ENGINE-ASSET-ABSENT.
  }
}

function localOnlyCapability(): LocalStorageCapability {
  return {
    engine: "Browser memory preview",
    bundled: true,
    fts5_available: false,
    network_required: false,
    daemon_required: false,
    telemetry_enabled: false,
    path_policy: "browser-session memory fallback; no repository-default writes",
    large_file_policy: "reference external files by path/hash metadata; do not silently copy large files",
    database_path: "browser-session memory fallback",
    compile_options: [],
    migration_framework: BROWSER_MIGRATION_FRAMEWORK,
    migration_status: BROWSER_MIGRATION_STATUS,
    store_schema_version: 0,
    store_schema_target_version: 0,
    migrations_applied_on_open: []
  };
}

const BROWSER_MIGRATION_FRAMEWORK = "browser_memory_preview_no_sqlite_migration_ledger";
const BROWSER_MIGRATION_STATUS = "browser_memory_snapshot_no_sql_store_migrations_applicable";

function envelope(
  model: PreviewModel,
  editorIntents: EditorOperationIntent[],
  proposal: AgentProposal | null,
  selectedReviewTarget: SelectedReviewTarget | null,
  mechanicsResult: MechanicsResult | null,
  analysisRun: AnalysisRunEnvelope | null,
  modelHash: ModelHashEvidence | null,
  projectEnvelopeHash: ProjectEnvelopeHashEvidence | null,
  message: string
): LocalProjectEnvelope {
  // CLM-003 mirror: a document that is not current is refused by name; the
  // snapshot stores the document bytes as given.
  const documentStatus = requireEditableModelDocument(model);
  const snapshot = cloneModel(model);
  return {
    summary: {
      project_id: snapshot.project.id,
      project_name: snapshot.project.name,
      database_path: "browser-session memory fallback",
      storage_mode: "browser_memory_preview",
      migration_status: BROWSER_MIGRATION_STATUS,
      migration_framework: BROWSER_MIGRATION_FRAMEWORK,
      store_schema_version: 0,
      store_schema_target_version: 0,
      migrations_applied_on_open: [],
      fts_indexed: false,
      copied_external_files: false,
      editor_intent_count: editorIntents.length,
      proposal_count: proposal ? 1 : 0,
      selected_review_target_count: selectedReviewTarget ? 1 : 0,
      selected_review_target_ref: selectedReviewTargetRef(selectedReviewTarget),
      persisted_mechanics_result_count: mechanicsResult ? 1 : 0,
      persisted_analysis_run_count: analysisRun ? 1 : 0,
      persisted_analysis_run_ref: persistedAnalysisRunRef(analysisRun, mechanicsResult),
      persisted_model_hash_count: modelHash ? 1 : 0,
      persisted_model_hash_ref: modelHash?.value ?? "not_persisted",
      persisted_project_envelope_hash_count: projectEnvelopeHash ? 1 : 0,
      persisted_project_envelope_hash_ref: projectEnvelopeHash?.value ?? "not_persisted",
      ...unitRoundTripSummary(snapshot),
      message
    },
    model: snapshot,
    editor_intents: cloneEditorIntents(editorIntents),
    proposal: cloneProposal(proposal),
    selected_review_target: cloneSelectedReviewTarget(selectedReviewTarget),
    mechanics_result: cloneJson(mechanicsResult),
    analysis_run: cloneJson(analysisRun),
    model_hash: cloneJson(modelHash),
    project_envelope_hash: cloneJson(projectEnvelopeHash),
    model_document_version: documentStatus
  };
}

function cloneEnvelope(project: LocalProjectEnvelope): LocalProjectEnvelope {
  return {
    summary: { ...project.summary },
    model: cloneModel(project.model),
    editor_intents: cloneEditorIntents(project.editor_intents ?? []),
    proposal: cloneProposal(project.proposal),
    selected_review_target: cloneSelectedReviewTarget(project.selected_review_target),
    mechanics_result: cloneJson(project.mechanics_result),
    analysis_run: cloneJson(project.analysis_run),
    model_hash: cloneJson(project.model_hash),
    project_envelope_hash: cloneJson(project.project_envelope_hash),
    model_document_version: cloneJson(project.model_document_version)
  };
}

function cloneModel(model: PreviewModel): PreviewModel {
  return JSON.parse(JSON.stringify(model)) as PreviewModel;
}

function cloneEditorIntents(editorIntents: EditorOperationIntent[]): EditorOperationIntent[] {
  return JSON.parse(JSON.stringify(editorIntents)) as EditorOperationIntent[];
}

function cloneProposal(proposal: AgentProposal | null | undefined): AgentProposal | null {
  if (!proposal) return null;
  return JSON.parse(JSON.stringify(proposal)) as AgentProposal;
}

function cloneSelectedReviewTarget(
  selectedReviewTarget: SelectedReviewTarget | null | undefined
): SelectedReviewTarget | null {
  if (!selectedReviewTarget) return null;
  return JSON.parse(JSON.stringify(selectedReviewTarget)) as SelectedReviewTarget;
}

function cloneJson<T>(value: T | null | undefined): T | null {
  if (!value) return null;
  return JSON.parse(JSON.stringify(value)) as T;
}

function selectedReviewTargetRef(selectedReviewTarget: SelectedReviewTarget | null): string {
  if (!selectedReviewTarget) return "not_selected";
  return `${selectedReviewTarget.target_type}: ${selectedReviewTarget.id}`;
}

function persistedAnalysisRunRef(
  analysisRun: AnalysisRunEnvelope | null,
  mechanicsResult: MechanicsResult | null
): string {
  return analysisRun?.analysis_run?.run_id ?? mechanicsResult?.run_id ?? "not_persisted";
}

function unitRoundTripSummary(model: PreviewModel): {
  unit_round_trip_status: string;
  unit_round_trip_checked_ref_count: number;
  unit_round_trip_signature: string;
} {
  const unitRefs: string[] = [];
  const missingRefs: string[] = [];
  for (const [dimension, unit] of Object.entries(model.project.units ?? {})) {
    if (unit.length > 0) {
      unitRefs.push(`project.units.${dimension}=${unit}`);
    } else {
      missingRefs.push(`project.units.${dimension}`);
    }
  }
  for (const material of model.materials ?? []) {
    collectQuantityUnit(unitRefs, missingRefs, `materials.${material.id}.elastic_modulus`, material.elastic_modulus);
    collectQuantityUnit(unitRefs, missingRefs, `materials.${material.id}.shear_modulus`, material.shear_modulus);
    collectQuantityUnit(unitRefs, missingRefs, `materials.${material.id}.poisson_ratio`, material.poisson_ratio);
    for (const point of material.temperature_points ?? []) {
      for (const field of ["temperature", "elastic_modulus", "shear_modulus", "poisson_ratio", "thermal_expansion_coefficient"] as const) {
        collectQuantityUnit(unitRefs, missingRefs, `materials.${material.id}.temperature_points.${point.id}.${field}`, point[field]);
      }
    }
    collectQuantityUnit(
      unitRefs,
      missingRefs,
      `materials.${material.id}.thermal_expansion_coefficient`,
      material.thermal_expansion_coefficient
    );
  }
  for (const loadCase of model.load_cases) {
    for (const region of loadCase.pressure_regions ?? []) {
      collectQuantityUnit(unitRefs, missingRefs, `load_cases.${loadCase.id}.pressure_regions.${region.id}.pressure`, region.pressure);
    }
  }
  for (const section of model.sections ?? []) {
    for (const [field, quantity] of Object.entries(section.properties ?? {})) {
      collectQuantityUnit(unitRefs, missingRefs, `sections.${section.id}.${field}`, quantity);
    }
  }
  for (const segment of model.pipe_segments ?? []) {
    for (const [field, quantity] of Object.entries(segment.section ?? {})) {
      collectQuantityUnit(unitRefs, missingRefs, `pipe_segments.${segment.id}.section.${field}`, quantity);
    }
  }
  for (const loadCase of model.load_cases ?? []) {
    for (const primitiveLoad of loadCase.primitive_loads ?? []) {
      const primitiveId = typeof primitiveLoad.id === "string" ? primitiveLoad.id : "unidentified";
      collectQuantityUnit(
        unitRefs,
        missingRefs,
        `load_cases.${loadCase.id}.primitive_loads.${primitiveId}.magnitude`,
        primitiveLoad.magnitude
      );
    }
  }
  const signature = unitRefs.sort().join("|");
  return {
    unit_round_trip_status:
      unitRefs.length > 0 && missingRefs.length === 0
        ? "unit_metadata_preserved_in_local_project_envelope"
        : "unit_metadata_missing_review_required",
    unit_round_trip_checked_ref_count: unitRefs.length,
    unit_round_trip_signature: signature || "no_unit_metadata"
  };
}

function collectQuantityUnit(unitRefs: string[], missingRefs: string[], ref: string, value: unknown): void {
  if (value === undefined || value === null) return;
  if (hasUnit(value)) {
    unitRefs.push(`${ref}=${value.unit}`);
    return;
  }
  if (hasQuantityValue(value)) {
    missingRefs.push(ref);
  }
}

function hasUnit(value: unknown): value is { unit: string } {
  return (
    typeof value === "object" &&
    value !== null &&
    "unit" in value &&
    typeof (value as { unit?: unknown }).unit === "string" &&
    (value as { unit: string }).unit.length > 0
  );
}

function hasQuantityValue(value: unknown): boolean {
  return typeof value === "object" && value !== null && "value" in value;
}

export async function getLocalStorageCapability(): Promise<LocalStorageCapability> {
  if (!hasTauriRuntime()) return localOnlyCapability();
  return invoke<LocalStorageCapability>("get_local_storage_capability");
}

// W-17 slice (a) (DEL-02-02 Design `project-unit-selection.md` §2.2-2.3): every new
// project carries these seven keys, each a canonical DEC-018 catalog symbol for its
// dimension. The map is the record; a preset is only UI shorthand.
export const PROJECT_UNIT_KEYS = ["length", "force", "moment", "pressure", "stress", "temperature", "angle"] as const;
export type ProjectUnitKey = typeof PROJECT_UNIT_KEYS[number];
export type ProjectUnitMap = Record<ProjectUnitKey, string>;
/** SI: today's blank set plus `angle: rad`. The default. */
export const SI_PROJECT_UNITS: Readonly<ProjectUnitMap> = Object.freeze({
  length: "m", force: "N", moment: "N*m", pressure: "Pa", stress: "Pa", temperature: "degC", angle: "rad"
});
export const US_CUSTOMARY_PROJECT_UNITS: Readonly<ProjectUnitMap> = Object.freeze({
  length: "in", force: "lbf", moment: "lbf*in", pressure: "psi", stress: "psi", temperature: "degF", angle: "deg"
});

export function buildBlankLocalModelDocument(
  createdAt: Date = new Date(),
  units: Readonly<ProjectUnitMap> = SI_PROJECT_UNITS
): PreviewModel {
  const stamp = createdAt.toISOString().replace(/\.\d{3}Z$/, "Z");
  const token = stamp.replace(/[^0-9TZ]/g, "").toLowerCase();
  const projectId = `project:blank-local-${token}`;
  return {
    schema_version: MODEL_SCHEMA_VERSION,
    document_kind: "openpipestress.product_preview.model",
    data_boundary: {
      public_examples_policy: "blank_user_created_local_document_no_bundled_engineering_values",
      protected_source_policy: "no_protected_standards_content_inserted",
      private_data_policy: "local_user_document_not_committed_to_repository",
      professional_boundary: "human_review_required_no_software_approval_claim"
    },
    project: {
      id: projectId,
      name: "Blank Local Model",
      description:
        "User-created local blank model document. Add explicit nodes, pipe runs, supports, materials, loads, and combinations through structured editors before solving.",
      units: Object.fromEntries(PROJECT_UNIT_KEYS.map((key) => {
        const symbol = units[key];
        if (typeof symbol !== "string" || !symbol) throw new Error(`PROJECT-UNITS-INCOMPLETE: a new project needs project.units.${key}`);
        return [key, symbol];
      }))
    },
    analysis_status: {
      mechanics: "MODEL_INCOMPLETE",
      rule_check: "RULE_INPUTS_INCOMPLETE",
      professional_acceptance: "NOT_PROVIDED"
    },
    materials: [],
    sections: [],
    nodes: [],
    pipe_segments: [],
    supports: [],
    components: [],
    load_cases: [],
    combinations: [],
    diagnostics: [
      {
        id: "diagnostic:blank-project:authoring-target",
        code: "BLANK_PROJECT_AUTHORING_TARGET",
        severity: "blocking",
        source: "apps/desktop/src/services/projectService.ts",
        affected_refs: [projectId],
        message:
          "Blank local model document is intentionally incomplete; no fixture entities, hidden loads, or engineering defaults were inserted."
      }
    ]
  };
}

/** T1 WP3 (D3): a blank document for load/reference-state work, beside the unchanged
 * blank path. B7b: it is the current model version with the exact contract the load/reference-state
 * group requires, the current 3.0.0/exact_pressure_v3 (since T4-U8 with the authoring
 * default timoshenko_annulus_energy_v1, D-6/O3), and an empty
 * reference-configuration list; it carries no materials, sections, cases or reference
 * values, and no document is upgraded to produce it. */
export const BLANK_LOAD_STATE_MODEL_SCHEMA_VERSION = MODEL_SCHEMA_VERSION;
export const BLANK_LOAD_STATE_PRESSURE_CONTRACT = { version: "3.0.0", mode: "exact_pressure_v3", shear_model: "timoshenko_annulus_energy_v1" } as const;
export function buildBlankLoadStateModelDocument(
  createdAt: Date = new Date(),
  units: Readonly<ProjectUnitMap> = SI_PROJECT_UNITS
): PreviewModel {
  const { schema_version: blankVersion, ...blank } = buildBlankLocalModelDocument(createdAt, units);
  void blankVersion;
  return {
    schema_version: BLANK_LOAD_STATE_MODEL_SCHEMA_VERSION,
    pressure_contract: { ...BLANK_LOAD_STATE_PRESSURE_CONTRACT },
    reference_configurations: [],
    ...blank,
    project: { ...blank.project, name: `Blank Local Load/reference Model ${MODEL_SCHEMA_VERSION}` }
  };
}

export async function createLocalProject(
  model: PreviewModel,
  editorIntents: EditorOperationIntent[] = [],
  proposal: AgentProposal | null = null,
  selectedReviewTarget: SelectedReviewTarget | null = null,
  mechanicsResult: MechanicsResult | null = null,
  analysisRun: AnalysisRunEnvelope | null = null,
  modelHash: ModelHashEvidence | null = null,
  projectEnvelopeHash: ProjectEnvelopeHashEvidence | null = null,
  confirmedRevision: string | null = null
): Promise<LocalProjectEnvelope> {
  if (!hasTauriRuntime()) {
    await loadValidatorEngine();
    refuseUnconfirmedBrowserReplacement(model.project.id, confirmedRevision, null);
    return storeBrowserProject(envelope(
      model,
      editorIntents,
      proposal,
      selectedReviewTarget,
      mechanicsResult,
      analysisRun,
      modelHash,
      projectEnvelopeHash,
      "Created local browser-preview project snapshot without external file copies."
    ));
  }
  return invokeWithIngress<LocalProjectEnvelope>("create_local_project", {
    model,
    editorIntents,
    proposal,
    selectedReviewTarget,
    mechanicsResult,
    analysisRun,
    modelHash,
    projectEnvelopeHash,
    confirmedRevision
  });
}

/** A stored project's index entry and the revision a Replace of it confirms.
 * `saved_state_count` (W-12 §2): the saved model states a Replace keeps; the
 * native store reports it, the browser preview keeps no states. */
export type StoredProjectEntry = LocalProjectIndexEntry & { revision: string; saved_state_count?: number };

function browserRevision(stored: BrowserStoredProject): string {
  return `${stored.envelope.project_envelope_hash?.value ?? ""}@${stored.writeSequence}`;
}

/** HELP_HUMAN rulings (2026-10-10): the app never silently replaces a stored
 * project. Mirrors the native store's `refuse_unconfirmed_replacement`: a write
 * onto a stored id needs a Replace of the revision the prompt showed
 * (`confirmedRevision`), or `expectedBase` equal to the stored envelope hash (a
 * save in place from the stored revision). */
function refuseUnconfirmedBrowserReplacement(projectId: string, confirmedRevision: string | null, expectedBase: string | null) {
  const existing = browserPreviewStore.get(projectId);
  if (!existing) return;
  const stored = `project \`${projectId}\` ("${existing.envelope.summary.project_name}", last saved at unix ${existing.updatedAtUnix})`;
  if (confirmedRevision !== null) {
    if (confirmedRevision === browserRevision(existing)) return;
    throw new Error(`PROJECT-BASE-MOVED: ${stored} changed after the replace prompt showed it; replacing it needs the user's confirmation.`);
  }
  if (expectedBase === null) throw new Error(`PROJECT-ID-EXISTS: ${stored} is already stored; replacing it needs the user's confirmation.`);
  const storedBase = existing.envelope.project_envelope_hash?.value ?? "";
  if (expectedBase === "" || expectedBase !== storedBase) {
    throw new Error(`PROJECT-BASE-MOVED: ${stored} changed after this session opened or saved it; replacing it needs the user's confirmation.`);
  }
}

/** Empty the browser preview's store, as a fresh page load does. The test
 * setup calls this between cases so no case inherits a stored project. */
export function clearBrowserPreviewStore(): void {
  browserPreviewStore.clear();
}

/** The stored project with this id, if any, and its revision, for the replace prompt. */
export async function findLocalProject(projectId: string): Promise<StoredProjectEntry | null> {
  if (!hasTauriRuntime()) {
    const stored = browserPreviewStore.get(projectId);
    const entry = (await listLocalProjects()).find((candidate) => candidate.project_id === projectId);
    return stored && entry ? { ...entry, revision: browserRevision(stored) } : null;
  }
  return invoke<StoredProjectEntry | null>("find_local_project", { projectId });
}

export async function openLocalProject(projectId: string | null = null): Promise<LocalProjectEnvelope | null> {
  if (!hasTauriRuntime()) {
    const stored = projectId ? browserPreviewStore.get(projectId) : browserProjectsByRecency()[0];
    if (!stored) return null;
    await loadValidatorEngine();
    requireEditableModelDocument(stored.envelope.model);
    const opened = cloneEnvelope(stored.envelope);
    opened.summary.message = projectId
      ? `Opened local browser-preview project snapshot by id ${projectId}.`
      : "Opened local browser-preview project snapshot.";
    return opened;
  }
  return invoke<LocalProjectEnvelope | null>("open_local_project", { projectId });
}

export async function listLocalProjects(): Promise<LocalProjectIndexEntry[]> {
  if (!hasTauriRuntime()) {
    return browserProjectsByRecency().map((stored) => ({
      project_id: stored.envelope.summary.project_id,
      project_name: stored.envelope.summary.project_name,
      storage_mode: "browser_memory_preview",
      created_at_unix: stored.createdAtUnix,
      updated_at_unix: stored.updatedAtUnix
    }));
  }
  return invoke<LocalProjectIndexEntry[]>("list_local_projects");
}

export async function saveLocalProject(
  model: PreviewModel,
  editorIntents: EditorOperationIntent[] = [],
  proposal: AgentProposal | null = null,
  selectedReviewTarget: SelectedReviewTarget | null = null,
  mechanicsResult: MechanicsResult | null = null,
  analysisRun: AnalysisRunEnvelope | null = null,
  modelHash: ModelHashEvidence | null = null,
  projectEnvelopeHash: ProjectEnvelopeHashEvidence | null = null,
  confirmedRevision: string | null = null,
  expectedBase: string | null = null
): Promise<LocalProjectEnvelope> {
  if (!hasTauriRuntime()) {
    await loadValidatorEngine();
    refuseUnconfirmedBrowserReplacement(model.project.id, confirmedRevision, expectedBase);
    return storeBrowserProject(envelope(
      model,
      editorIntents,
      proposal,
      selectedReviewTarget,
      mechanicsResult,
      analysisRun,
      modelHash,
      projectEnvelopeHash,
      "Saved local browser-preview project snapshot without external file copies."
    ));
  }
  return invokeWithIngress<LocalProjectEnvelope>("save_local_project", {
    request: {
      project_id: model.project.id,
      project_name: model.project.name,
      model,
      editor_intents: editorIntents,
      proposal,
      selected_review_target: selectedReviewTarget,
      mechanics_result: mechanicsResult,
      analysis_run: analysisRun,
      model_hash: modelHash,
      project_envelope_hash: projectEnvelopeHash,
      confirmed_revision: confirmedRevision,
      expected_base: expectedBase
    }
  });
}
