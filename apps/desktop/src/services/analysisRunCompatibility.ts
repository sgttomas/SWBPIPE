import { validateRetainedPrecision } from "../features/results/retainedPrecisionReader";
import { isRetainedRoute } from "../features/results/numericalResultQuality";
import { hasCurrentSourceContract, readerDispatch, readerValidate, ResultReaderError, sourceContract, sourceSemanticBinding } from "../features/results/resultReader";
import pressureTable from "../../../../fixtures/results/semantic_contract_v0_3_pressure_2.json";
import pressureSourceTable from "../../../../fixtures/results/semantic_contract_v0_3_pressure_source_1.json";
import previewPhysicsTable from "../../../../fixtures/results/semantic_contract_v0_3_preview_physics_2.json";
import previewPhysicsTimoshenkoTable from "../../../../fixtures/results/semantic_contract_v0_3_preview_physics_timoshenko_1.json";
import previewPhysicsRetainedTable from "../../../../fixtures/results/semantic_contract_v0_3_preview_physics_retained_1.json";
import physicsRetainedTable from "../../../../fixtures/results/semantic_contract_v0_3_physics_retained_1.json";
import type { AnalysisRunEnvelope, CanonicalResultDimension, MechanicsResult, ObjectRef, PreviewModel } from "../types";
import { canonicalSha256Hex, canonicalSha256HexCheckedV1, checkedJsonText } from "./hashService";
import { inputManifestRef, type CurrentSessionInputManifest } from "./inputManifestService";
import { libraryRefSites } from "./libraryImportService";
import { trimLikeRust } from "./rustTrim";
import { manifestExcludedRulePackIdentities, rulePackIdentity, rulePackRefDiagnosticEntries, rulePackRefRecordDiagnostics, type ExcludedRulePackRef } from "./rulePackRefDiagnostics";

/** The one current analysis-run version (b7-plan E1). */
export const ANALYSIS_RUN_V03 = "0.3.0";
/** A saved 0.1.0 or 0.2.0 record is refused by name; there is no migration. Re-run the analysis. */
export const ANALYSIS_RUN_SCHEMA_VERSION_RETIRED = "ANALYSIS_RUN_SCHEMA_VERSION_RETIRED";
const RETIRED_ANALYSIS_RUN_VERSIONS: readonly string[] = ["0.1.0", "0.2.0"];
export const CHECKED_PROFILE_V1 = "openpipestress_jcs_ijson_v1";
/** U6d (C1:162; D2 4.9.6): the record's receipt copy differs from the source's. */
export const ANALYSIS_RETAINED_PRECISION_RECEIPT_MISMATCH = "ANALYSIS_RETAINED_PRECISION_RECEIPT_MISMATCH";
/** U6d: a record of any other identity carries a retained-precision receipt. */
export const ANALYSIS_RETAINED_PRECISION_DOWNGRADE_FORBIDDEN = "ANALYSIS_RETAINED_PRECISION_DOWNGRADE_FORBIDDEN";
type ManifestEvidence = { manifest_ref: ObjectRef; manifest_sha256: string; manifest: Parameters<typeof manifestExcludedRulePackIdentities>[0] & { model_basis: { model_ref: string }; solver_basis: { solver_name: string; solver_version: string; solver_build_ref: string } } };
/** The record's provenance labels, Python's (`core/analysis_runs/compatibility.py`
 * `_build_analysis_run`) for the one current (0.3.0) record. */
const provenance = { source_name: "OpenPipeStress analysis record 0.3", source_location: "analysis_run.compatibility.v0.3", source_license: "project-governed", review_status: "pending", professional_claim: false };

/** The record builder's tables (DEL-14-02, as Python's `compatibility.py` record builder):
 * the pinned table each binding names. The table is
 * the one the Rust header dispatch binds (`resultReader.ts`); the record's row
 * interpretation below is the record contract's own, not the reader's signature.
 * B7b: the retired v2 tables (physics-1, physics-source-1, load-reference-1,
 * load-reference-source-1) are deleted; pressure-2 and pressure-source-1 each bind their
 * own table, as in Python's builder (`_TABLES`, `CURRENT_RECORD_CONTRACT_IDS`). */
type RecordTable = { rows: Array<{ signature_id: string; kind: string; unit: string; component: string | null; source_basis?: string; category: string; family: string | null; source_physical_semantic_dimension: string | null; derivative_target_dimension: string | null; legacy_declared_dimension: string | null; canonical_disposition: string }> };
const RECORD_TABLES: Readonly<Record<string, RecordTable>> = Object.freeze({
  [pressureTable.semantic_contract_id]: pressureTable,
  [pressureSourceTable.semantic_contract_id]: pressureSourceTable,
  [previewPhysicsTable.semantic_contract_id]: previewPhysicsTable,
  // DEL-05-03 pf-timoshenko.md §4: the pressure-free Timoshenko identity's own table.
  [previewPhysicsTimoshenkoTable.semantic_contract_id]: previewPhysicsTimoshenkoTable,
  [previewPhysicsRetainedTable.semantic_contract_id]: previewPhysicsRetainedTable,
  [physicsRetainedTable.semantic_contract_id]: physicsRetainedTable,
} as Record<string, RecordTable>);
/** Model document 0.8.0 (DEL-02-01 Design `model-document-0.8.0.md` §3.2; owner ruling
 * "0.8.0 a2": recorded runs keep their existing read and compare behaviour, with no
 * model-document-version gate). pressure-2's table changed only the model version token of
 * its admitted-families sentence (0.7.0 -> 0.8.0); every row, and so every row reading, is the
 * same. A run recorded under the 0.7.0-era table binds that table's exact digest, which is
 * read as the recorded basis of exactly this current table. Only the record validator reads
 * it: the record builder and the export bind the current table, and any other digest, or a
 * record binding the two tables at once, is refused as before. Model document 0.9.0 (Design
 * `model-document-0.9.0.md` §3.2) edits the same sentence's token again (0.8.0 -> 0.9.0), so a
 * run recorded under the 0.8.0-era table is read the same way. Python's twin is
 * `compatibility.py` `RECORDED_RUN_TABLE_BASES`. */
const RECORDED_RUN_TABLE_BASES: ReadonlyArray<Readonly<{ id: string; recorded: string; current: string }>> = Object.freeze([
  { id: "openpipestress.result_semantics/0.3.0/pressure-2",
    recorded: "3e2aac3cbe1dd967292dc2524cc7e12a35b162f14795c13b741acaa2b964625d",
    current: "032e861fc66bf5f34a081b104ec879a39831bfd6472b186954966ea57d65f025" },
  { id: "openpipestress.result_semantics/0.3.0/pressure-2",
    recorded: "5451fbf3e10e085c90b523baa7daf5fc723659e0d874d988d23807be7d2fefe6",
    current: "032e861fc66bf5f34a081b104ec879a39831bfd6472b186954966ea57d65f025" },
]);
/** The binding a record is checked against: the current table's, or the exact recorded basis
 * the record names when it is one of `RECORDED_RUN_TABLE_BASES` for that current table. */
export function recordedRunSemanticBinding(recorded: unknown, current: Readonly<{ id: string; sha256: string }>): { id: string; sha256: string } {
  const named = recorded && typeof recorded === "object" ? recorded as Record<string, unknown> : null;
  const basis = named && Object.keys(named).length === 2 ? RECORDED_RUN_TABLE_BASES.find(entry =>
    entry.id === current.id && entry.current === current.sha256 && named.id === entry.id && named.sha256 === entry.recorded) : undefined;
  return basis ? { id: basis.id, sha256: basis.recorded } : { id: current.id, sha256: current.sha256 };
}
function recordTable(source: MechanicsResult): RecordTable {
  const dispatch = readerDispatch(source);
  const table = dispatch.binding ? RECORD_TABLES[dispatch.binding.id] : undefined;
  if (!table) throw new Error("SOURCE_SEMANTIC_CONTRACT_UNSUPPORTED");
  return table;
}
/** A new-method discriminator only; old table signatures omit this field. */
function sourceBasisMatches(signature: { source_basis?: string }, row: MechanicsResult["results"][number]): boolean {
  return !("source_basis" in signature) || signature.source_basis === row.metadata?.basis;
}
/** The record's interpretation of each row of one source, against that source's table. */
export function analysisRowSemanticsReader(source: MechanicsResult): (row: MechanicsResult["results"][number]) => { semantic: RecordTable["rows"][number] | null; findings: string[] } {
  let contract: RecordTable | null = null, failure: unknown = null;
  try { contract = recordTable(source); } catch (error) { failure = error; }
  return row => {
    if (!contract) throw failure;
    return analysisRowSemanticsIn(contract, row);
  };
}
export function analysisRowSemantics(row: MechanicsResult["results"][number], source: MechanicsResult) {
  return analysisRowSemanticsReader(source)(row);
}
function analysisRowSemanticsIn(contract: RecordTable, row: MechanicsResult["results"][number]) {
  const byKind = contract.rows.filter((entry) => entry.kind === row.kind);
  if (!byKind.length) return { semantic: null, findings: ["SOURCE_SIGNATURE_UNKNOWN"] };
  const byUnit = byKind.filter((entry) => entry.unit === row.unit);
  if (!byUnit.length) throw new Error(`SOURCE_UNIT_CONTRADICTION: ${row.kind} / ${row.unit}`);
  const component = row.metadata?.component ?? null;
  const exact = byUnit.find((entry) => entry.component === component && sourceBasisMatches(entry, row));
  if (exact) {
    if (row.dimension && exact.legacy_declared_dimension && row.dimension !== exact.legacy_declared_dimension) throw new Error(`ANALYSIS-RUN-RESULT-DIMENSION-MISMATCH: ${row.id}`);
    return { semantic: exact, findings: [] as string[] };
  }
  const generic = byUnit.find((entry) => entry.component === null && sourceBasisMatches(entry, row));
  if (generic) {
    if (row.dimension && generic.legacy_declared_dimension && row.dimension !== generic.legacy_declared_dimension) throw new Error(`ANALYSIS-RUN-RESULT-DIMENSION-MISMATCH: ${row.id}`);
    return { semantic: generic, findings: component !== null ? [] : ["OPTIONAL_SOURCE_METADATA_MISSING"] };
  }
  if (byUnit.some(entry => entry.component === component || entry.component === null)) throw new Error(`SOURCE_BASIS_CONTRADICTION: ${row.kind}`);
  if (component !== null) throw new Error(`SOURCE_COMPONENT_CONTRADICTION: ${row.kind} / ${component}`);
  return { semantic: null, findings: ["SOURCE_COMPONENT_MISSING_SEMANTICS_UNAVAILABLE"] };
}

const RULE_STATUSES = ["RULE_INPUTS_INCOMPLETE", "USER_RULE_CHECKED", "USER_RULE_FAILED"];
export function sourceBasisReference(basis: unknown): ObjectRef | null {
  if (basis === undefined || basis === null) return null;
  const b = basis as Record<string, unknown>;
  if (typeof b !== "object" || Array.isArray(b) || Object.keys(b).length !== 2 || typeof b.ref_type !== "string" || !b.ref_type || typeof b.ref_id !== "string" || !b.ref_id) throw new Error("ANALYSIS_SOURCE_REFERENCE_INVALID");
  return { object_type: b.ref_type === "load_case" ? "LoadCase" : b.ref_type === "combination" ? "Combination" : "ResultBasis", ref: b.ref_id };
}
export function modelLoadBasisRefs(model: Pick<PreviewModel, "load_cases" | "combinations">): ObjectRef[] {
  return [...model.load_cases.map(c => ({ object_type: "LoadCase", ref: c.id })), ...(model.combinations ?? []).map(c => ({ object_type: "Combination", ref: c.id }))];
}
function validateSourceRuleStatus(source: MechanicsResult): void {
  const rule = source.status.rule_check;
  if (rule !== undefined && rule !== null && !RULE_STATUSES.includes(rule)) throw new Error("ANALYSIS_SOURCE_RULE_STATUS_INVALID");
}
function expectedLoadBasis(source: MechanicsResult, expected?: ObjectRef[]): ObjectRef[] {
  for (const value of [source.run_id, source.model_ref, ...source.results.map(r => r.id)]) if (typeof value !== "string" || !value) throw new Error("ANALYSIS_SOURCE_REFERENCE_INVALID");
  const rows = source.results.map(r => sourceBasisReference(r.basis_ref)).filter((r): r is ObjectRef => r !== null);
  if (expected === undefined) return rows.filter((r, i) => rows.findIndex(x => x.object_type === r.object_type && x.ref === r.ref) === i);
  if (!Array.isArray(expected) || expected.some((r, i) => !r || Object.keys(r).length !== 2 || !["LoadCase", "Combination", "ResultBasis"].includes(r.object_type) || typeof r.ref !== "string" || !r.ref || expected.slice(0, i).some(x => x.object_type === r.object_type && x.ref === r.ref))) throw new Error("ANALYSIS_LOAD_BASIS_INVALID");
  const required = [...rows, ...(source.numerical_quality?.cases ?? []).map(c => sourceBasisReference(c.basis_ref)).filter((r): r is ObjectRef => r !== null)];
  if (required.some(r => !expected.some(x => x.object_type === r.object_type && x.ref === r.ref))) throw new Error("ANALYSIS_LOAD_BASIS_SOURCE_SCOPE_MISMATCH");
  return structuredClone(expected);
}

export function analysisRecordProjection(record: AnalysisRunEnvelope): AnalysisRunEnvelope {
  const projected = structuredClone(record);
  const matches = projected.analysis_run.hashes.filter((hash) => hash.payload_scope === "analysis_run_record");
  if (matches.length > 1) throw new Error("ANALYSIS-RUN-RECORD-CHECKSUM-DUPLICATE");
  projected.analysis_run.hashes = projected.analysis_run.hashes.filter((hash) => hash.payload_scope !== "analysis_run_record");
  return projected;
}

/** A record's rule-pack reference diagnostics do not name exactly the references its input
 * manifest leaves out of `active_rule_packs`. */
export const ANALYSIS_RULE_PACK_REF_DIAGNOSTICS_MISMATCH = "ANALYSIS_RULE_PACK_REF_DIAGNOSTICS_MISMATCH";
/** A run recorded before rule-pack resolution (S2, #1596): the model has references, the
 * manifest's `active_rule_packs` is empty and the record names no exclusion. Refused by name
 * (not read leniently) so the user knows to re-record it; Rust and Python refuse it alike. */
export const ANALYSIS_RUN_RULE_PACKS_NOT_RESOLVED = "ANALYSIS-RUN-RULE-PACKS-NOT-RESOLVED";
const RULE_PACKS_NOT_RESOLVED_REFUSAL = `${ANALYSIS_RUN_RULE_PACKS_NOT_RESOLVED}: recorded before rule-pack resolution; re-record this run`;

/** The refusal for record rule-pack diagnostics naming `identities` against `manifest`'s
 * exclusions, or null when they agree. */
function rulePackDiagnosticsRefusal(identities: string[], manifest: Parameters<typeof manifestExcludedRulePackIdentities>[0]): string | null {
  const excluded = manifestExcludedRulePackIdentities(manifest);
  if (structurallyEqual(identities, excluded)) return null;
  const noneResolved = Array.isArray(manifest.active_rule_packs) && manifest.active_rule_packs.length === 0;
  return identities.length === 0 && excluded.length > 0 && noneResolved ? RULE_PACKS_NOT_RESOLVED_REFUSAL : ANALYSIS_RULE_PACK_REF_DIAGNOSTICS_MISMATCH;
}

/** The current (0.3.0) record; a result no current record contract reads is refused, never recorded in an older format. */
/** `rulePackExclusions`: the model's rule-pack references the solve left out of the input
 * manifest's `active_rule_packs` (`resolveModelRulePacks`), recorded as diagnostics after the
 * producer's, in the model's order. They must be exactly the references the manifest leaves out. */
export async function buildAnalysisRunV03(result: MechanicsResult, inputManifest: ManifestEvidence, ruleCheckStatus?: string | null, loadBasisRefs?: ObjectRef[], rulePackExclusions: readonly ExcludedRulePackRef[] = []): Promise<AnalysisRunEnvelope> {
  if (!hasCurrentSourceContract(result)) throw new Error("SOURCE_SEMANTIC_CONTRACT_UNSUPPORTED");
  return buildAnalysisRecord(result, inputManifest, sourceContract(result) as V03Route, ruleCheckStatus, loadBasisRefs, rulePackExclusions);
}
/** Every current-record route (Python's `CURRENT_RECORD_CONTRACT_IDS`). B7b: pressure-2 has a
 * record, as Python builds it: the ordinary exact route, with or without the
 * load/reference-state group, carrying neither contract_evidence nor source_block_recovery.
 * Holding a record grants no output by itself: each output surface applies its own policy
 * entry and Rust's standing (epoch 8, DEL-04-07 Design `desktop-exact-outputs.md`). */
type V03Route = "pressure" | "pressure_source" | "preview_physics" | "retained_preview_physics" | "retained_physics";
async function buildAnalysisRecord(result: MechanicsResult, inputManifest: ManifestEvidence, route: V03Route, ruleCheckStatus?: string | null, loadBasisRefs?: ObjectRef[], rulePackExclusions: readonly ExcludedRulePackRef[] = []): Promise<AnalysisRunEnvelope> {
  // Preview-evidence check: a preview-physics-2 source is recorded only after its closed reader checks pass.
  if (sourceContract(result) === "preview_physics") readerValidate("preview-physics", result);
  // As Python's `_source_contract`: a pressure-2 source is recorded only after its route's reader passes.
  if (sourceContract(result) === "pressure") readerValidate("pressure", result);
  // U6d: a successor is recorded only after the accepted reader passes (G0-G7, no invocation).
  if (isRetainedRoute(route)) await validateRetainedPrecision(result);
  if (route === "pressure_source") await validateRetainedRecoverySource(result);
  validateSourceRuleStatus(result); expectedLoadBasis(result, loadBasisRefs);
  if (inputManifest.manifest.model_basis.model_ref !== result.model_ref) throw new Error("ANALYSIS-RUN-INPUT-MANIFEST-MODEL-MISMATCH");
  const rulePackDiagnostics = rulePackRefRecordDiagnostics(rulePackExclusions);
  const builtRefusal = rulePackDiagnosticsRefusal(rulePackDiagnostics.map((item) => item.source_annotation.affected_refs[0]), inputManifest.manifest);
  if (builtRefusal) throw new Error(builtRefusal);
  const semanticBinding = sourceSemanticBinding(result);
  const rawResult = structuredClone(result);
  const interpret = analysisRowSemanticsReader(result);
  const resultRefs = await Promise.all(rawResult.results.map(async (row, sourceRowIndex) => {
    const { semantic, findings } = interpret(row);
    return {
      result_ref: { object_type: "Result" as const, ref: row.id }, source_row_index: sourceRowIndex,
      category: semantic?.category ?? "unknown", source_dimension: (semantic?.source_physical_semantic_dimension ?? null) as CanonicalResultDimension | null,
      result_family: semantic?.family ?? null,
      semantic_contract: { ...semanticBinding, signature_id: semantic?.signature_id ?? null },
      interpretation: { status: semantic?.canonical_disposition ?? "unavailable", findings },
      source_annotation: { kind: row.kind, unit: row.unit, metadata: structuredClone(row.metadata ?? null) },
      hash_refs: [{ algorithm: "sha256" as const, canonicalization: CHECKED_PROFILE_V1, payload_ref: { object_type: "Result" as const, ref: row.id }, payload_scope: "result_row", value: await canonicalSha256HexCheckedV1(row) }],
      privacy_classification: "source_evidence",
      provenance,
    };
  }));
  const effectiveRule = ruleCheckStatus ?? rawResult.status.rule_check ?? "RULE_INPUTS_INCOMPLETE";
  if (!RULE_STATUSES.includes(effectiveRule)) throw new Error("ANALYSIS_RULE_STATUS_INVALID");
  const effectiveLoadBasisRefs = expectedLoadBasis(result, loadBasisRefs);
  const statuses = Array.from(new Set(["HUMAN_REVIEW_REQUIRED", rawResult.status.mechanics, effectiveRule].filter(Boolean))).sort();
  const runRef = { object_type: "AnalysisRun", ref: rawResult.run_id } as ObjectRef;
  const record: AnalysisRunEnvelope = {
    schema_version: ANALYSIS_RUN_V03, deliverable_id: "DEL-14-02", package_id: "PKG-14", scope_item: "SOW-072", objectives: ["OBJ-016"],
    run_contract_status: { record_contract: "strict_analysis_run_v0_3", result_binding: "received_mechanics_result", external_validation_boundary: "reference_only_not_determined_by_software" },
    analysis_run: {
      run_id: rawResult.run_id, run_name: `${rawResult.run_id} analysis record`, run_kind: "mechanics_solve", created_at: null, model_state_ref: transientModelStateRef(rawResult.model_ref),
      solver_version: { solver_name: inputManifest.manifest.solver_basis.solver_name, solver_version: inputManifest.manifest.solver_basis.solver_version, build_ref: { object_type: "ExternalReference", ref: inputManifest.manifest.solver_basis.solver_build_ref } },
      settings_ref: { object_type: "SolverSettings", ref: `solver-settings:${inputManifest.manifest_ref.ref}:${inputManifest.manifest_sha256}` }, unit_system_ref: { object_type: "UnitSystem", ref: `unit-system:${rawResult.model_ref}:${inputManifest.manifest_sha256}` }, load_basis_refs: effectiveLoadBasisRefs,
      diagnostics: [...rawResult.diagnostics.map((item) => ({ source_annotation: structuredClone(item) })), ...rulePackDiagnostics], rule_pack_refs: [], library_refs: [],
      result_refs: resultRefs, hashes: [{ algorithm: "sha256", canonicalization: CHECKED_PROFILE_V1, payload_ref: { object_type: "ResultEnvelope", ref: `result-envelope:${rawResult.run_id}` }, payload_scope: "received_result", value: await canonicalSha256HexCheckedV1(rawResult) }], analysis_status: statuses,
      reproducibility: { input_manifest_refs: [structuredClone(inputManifest.manifest_ref)], input_manifest_hashes: [{ algorithm: "sha256", canonicalization: "rfc8785_jcs", payload_ref: structuredClone(inputManifest.manifest_ref), payload_scope: "input_manifest", value: inputManifest.manifest_sha256 }], semantic_contract: semanticBinding, determinism_notes: ["created_at_unavailable", "model_state_ref_and_solver_settings_unit_basis_bound_by_input_manifest"], unresolved_tbd: [] },
      immutability_policy: { run_record_is_read_only: true, mutation_policy: "changes_create_new_immutable_record_revision", new_mechanics_run_required_for_record_revision: false, record_revision_identity: "analysis_run_record_sha256", hash_invalidates_external_acceptance: true },
      professional_boundary: {
        human_review_required: true,
        software_makes_compliance_claim: false,
        software_makes_certification_claim: false,
        software_makes_sealing_claim: false,
        software_makes_approval_claim: false,
        software_makes_authentication_claim: false
      },
      provenance,
    },
  };
  // Python mirror: pressure-source-1 (with or without load/reference states) is a
  // source record, carrying its physical evidence and receipt.
  if (route === "pressure_source") record.analysis_run.contract_evidence = structuredClone(result.contract_evidence);
  if (route === "pressure_source") record.analysis_run.source_block_recovery = structuredClone(result.source_block_recovery);
  // U6d (C1:162; D2 4.9.6): the complete receipt is copied whole, never recomputed or repaired.
  if (isRetainedRoute(route)) record.analysis_run.retained_precision = structuredClone(result.retained_precision);
  record.analysis_run.hashes.unshift({ algorithm: "sha256", canonicalization: CHECKED_PROFILE_V1, payload_ref: runRef, payload_scope: "analysis_run_record", value: await canonicalSha256HexCheckedV1(analysisRecordProjection(record)) });
  await validateAnalysisRunV03(record, result, loadBasisRefs);
  return record;
}

/** The record's own checksum. A retired 0.1.0 or 0.2.0 record is refused by name (re-run the analysis). */
export async function verifyAnalysisRunRecord(record: AnalysisRunEnvelope): Promise<"match" | "mismatch" | "unverifiable"> {
  if (RETIRED_ANALYSIS_RUN_VERSIONS.includes(record.schema_version)) throw new Error(`${ANALYSIS_RUN_SCHEMA_VERSION_RETIRED}: analysis-run ${record.schema_version} is no longer read; re-run the analysis to record a current ${ANALYSIS_RUN_V03} analysis run`);
  if (record.schema_version !== ANALYSIS_RUN_V03) throw new Error(`ANALYSIS-RUN-SCHEMA-VERSION-UNSUPPORTED: ${record.schema_version}`);
  const hashes = record.analysis_run.hashes.filter((hash) => hash.payload_scope === "analysis_run_record");
  if (hashes.length !== 1) return "unverifiable";
  const claim = hashes[0];
  if (claim.algorithm !== "sha256" || claim.canonicalization !== CHECKED_PROFILE_V1 || claim.payload_ref.object_type !== "AnalysisRun" || claim.payload_ref.ref !== record.analysis_run.run_id) return "mismatch";
  return claim.value === await canonicalSha256HexCheckedV1(analysisRecordProjection(record)) ? "match" : "mismatch";
}

// --- Recorded revisions (W-12 S2; DEL-14-01 Design `desktop-states-runs-comparison.md` §3) ---
// Record Run stores a new immutable revision of a solve's transient record, tied
// to a saved model state. The native store (`apps/desktop/src-tauri/src/analysis_runs.rs`)
// allocates `created_at` and computes the revision's hash; this builder is the
// TypeScript mirror its parity test checks (`recordedRunParity.test.ts`), as
// Python's `recorded_revision` is (`core/analysis_runs/compatibility.py`).

/** The transient record's determinism note that a recorded revision drops (its `created_at` is set). */
export const CREATED_AT_UNAVAILABLE = "created_at_unavailable";

/** The stored state a recorded revision names, with the input manifest that binds it. */
export type RecordedStateBinding = {
  stateId: string;
  /** The state's model hash, `sha256:<hex>`. */
  modelHash: string;
  /** The recorded run's stored input manifest. */
  inputManifest: CurrentSessionInputManifest;
};

/** A transient record's synthetic state ref (`state:<model ref>:preview`). */
export function transientModelStateRef(modelRef: string): ObjectRef {
  return { object_type: "ModelState", ref: `state:${modelRef}:preview` };
}

/** Code-point order (as Rust's `str` and Python's `str` order), not UTF-16 order. */
function codePointCompare(left: string, right: string): number {
  const a = Array.from(left), b = Array.from(right);
  for (let index = 0; index < Math.min(a.length, b.length); index += 1) {
    const difference = a[index].codePointAt(0)! - b[index].codePointAt(0)!;
    if (difference !== 0) return difference;
  }
  return a.length - b.length;
}

/** The recorded revision's `rule_pack_refs`: the manifest's `active_rule_packs`, in its order, as
 * `{RulePack, rulePackIdentity(id, version)}` (`%` and `@` percent-encoded in each component). */
export function recordedRulePackRefs(manifest: Pick<CurrentSessionInputManifest, "active_rule_packs">): ObjectRef[] {
  if (!Array.isArray(manifest.active_rule_packs)) throw new Error("ANALYSIS-RUN-MANIFEST-INVALID: the input manifest has no active_rule_packs list.");
  return manifest.active_rule_packs.map((pack) => {
    if (typeof pack?.rule_pack_id !== "string" || !pack.rule_pack_id || typeof pack.version !== "string" || !pack.version) throw new Error("ANALYSIS-RUN-MANIFEST-INVALID: an active rule pack has no id or version.");
    return { object_type: "RulePack", ref: rulePackIdentity(pack.rule_pack_id, pack.version) };
  });
}

/** The recorded revision's `library_refs`: every `library_ref` site of the solved model
 * that the one walk finds (`libraryRefSites`, pinned with the native walk to
 * `fixtures/model_document/library_ref_sites.json`), its identity normalized as the
 * native walk normalizes it (`library_ref_identity`: Rust's White_Space trim,
 * `trimLikeRust`; nothing when blank or not text). `library_version` is kept verbatim
 * when it is text. A site whose kind or id is blank or not text names no library and is
 * omitted. Once each, in code-point order, as
 * `library:<kind>/<id>[@<version>][#<record id>]`. */
export function recordedLibraryRefs(modelPayload: unknown): ObjectRef[] {
  const identity = (value: unknown): string | null => {
    const trimmed = typeof value === "string" ? trimLikeRust(value) : "";
    return trimmed ? trimmed : null;
  };
  const found = new Set<string>();
  for (const { reference } of libraryRefSites(modelPayload)) {
    const kind = identity(reference.library_kind);
    const id = identity(reference.library_id);
    if (!kind || !id) continue;
    const record = identity(reference.record_id);
    found.add(`library:${kind}/${id}${typeof reference.library_version === "string" ? `@${reference.library_version}` : ""}${record ? `#${record}` : ""}`);
  }
  return [...found].sort(codePointCompare).map((ref) => ({ object_type: "Library", ref }));
}

/** The recorded revision of a transient record: `created_at`, the stored state's ref,
 * the manifest's rule packs and the solved model's library references, the
 * `created_at_unavailable` note dropped and the record hash recomputed. The received-result
 * checksum and every other member are unchanged. */
export async function recordedAnalysisRunRevision(transient: AnalysisRunEnvelope, binding: { stateId: string; createdAt: string; inputManifest: CurrentSessionInputManifest }): Promise<AnalysisRunEnvelope> {
  const record = structuredClone(transient);
  const run = record.analysis_run;
  run.created_at = binding.createdAt;
  run.model_state_ref = { object_type: "ModelState", ref: binding.stateId };
  run.rule_pack_refs = recordedRulePackRefs(binding.inputManifest);
  run.library_refs = recordedLibraryRefs(binding.inputManifest.model_basis.model_payload);
  if (Array.isArray(run.reproducibility.determinism_notes)) run.reproducibility.determinism_notes = run.reproducibility.determinism_notes.filter((note) => note !== CREATED_AT_UNAVAILABLE);
  const claim = run.hashes.find((hash) => hash.payload_scope === "analysis_run_record");
  if (!claim) throw new Error("ANALYSIS-RUN-RECORD-INVALID: the record has no analysis_run_record checksum.");
  claim.value = await canonicalSha256HexCheckedV1(analysisRecordProjection(record));
  return record;
}

/** Structural JSON equality: object member order is not significant, array order is,
 * and an extra or missing member is a difference (as Rust's and Python's equality). */
export function structurallyEqual(left: unknown, right: unknown): boolean {
  if (Array.isArray(left) || Array.isArray(right)) {
    return Array.isArray(left) && Array.isArray(right) && left.length === right.length && left.every((item, index) => structurallyEqual(item, right[index]));
  }
  if (left && right && typeof left === "object" && typeof right === "object") {
    const a = left as Record<string, unknown>, b = right as Record<string, unknown>;
    const keys = Object.keys(a);
    return keys.length === Object.keys(b).length && keys.every((key) => Object.hasOwn(b, key) && structurallyEqual(a[key], b[key]));
  }
  return Object.is(left, right) || (typeof left === "number" && left === right);
}

/** The recorded form's load basis: the stored manifest's load cases, then its combinations. */
export function manifestLoadBasisRefs(manifest: Pick<CurrentSessionInputManifest, "load_basis">): ObjectRef[] {
  return [
    ...manifest.load_basis.load_cases.map((loadCase) => ({ object_type: "LoadCase", ref: loadCase.id })),
    ...(manifest.load_basis.combinations ?? []).map((combination) => ({ object_type: "Combination", ref: combination.id }))
  ];
}

/** Every record member the stored input manifest fixes, derived from the manifest (never
 * from the record), with the refusal each difference gets: the manifest's ref and checksum,
 * the solver identity and build ref, the settings and unit-system refs, the load basis,
 * and the rule-pack and library references. */
export async function manifestFixedMembers(manifest: CurrentSessionInputManifest): Promise<Array<{ member: keyof AnalysisRunEnvelope["analysis_run"] | "input_manifest_refs" | "input_manifest_hashes"; expected: unknown; code: string }>> {
  const sha = await canonicalSha256Hex(manifest);
  const modelRef = manifest.model_basis.model_ref;
  const manifestRef = { object_type: "InputManifest", ref: inputManifestRef(modelRef, sha) };
  return [
    { member: "input_manifest_refs", expected: [manifestRef], code: "ANALYSIS_RECORDED_INPUT_MANIFEST_MISMATCH" },
    { member: "input_manifest_hashes", expected: [{ algorithm: "sha256", canonicalization: "rfc8785_jcs", payload_ref: manifestRef, payload_scope: "input_manifest", value: sha }], code: "ANALYSIS_RECORDED_INPUT_MANIFEST_MISMATCH" },
    { member: "solver_version", expected: { solver_name: manifest.solver_basis.solver_name, solver_version: manifest.solver_basis.solver_version, build_ref: { object_type: "ExternalReference", ref: manifest.solver_basis.solver_build_ref } }, code: "ANALYSIS_RECORDED_SOLVER_MISMATCH" },
    { member: "settings_ref", expected: { object_type: "SolverSettings", ref: `solver-settings:${manifestRef.ref}:${sha}` }, code: "ANALYSIS_RECORDED_SETTINGS_MISMATCH" },
    { member: "unit_system_ref", expected: { object_type: "UnitSystem", ref: `unit-system:${modelRef}:${sha}` }, code: "ANALYSIS_RECORDED_UNITS_MISMATCH" },
    { member: "load_basis_refs", expected: manifestLoadBasisRefs(manifest), code: "ANALYSIS_RECORDED_LOAD_BASIS_MISMATCH" },
    { member: "rule_pack_refs", expected: recordedRulePackRefs(manifest), code: "ANALYSIS_RECORDED_REFERENCES_MISMATCH" },
    { member: "library_refs", expected: recordedLibraryRefs(manifest.model_basis.model_payload), code: "ANALYSIS_RECORDED_REFERENCES_MISMATCH" }
  ];
}

/** A recorded revision's binding to its stored state: its ref names the state; its
 * `created_at` is set and the `created_at_unavailable` note is gone; the stored manifest
 * is the source's project's and its model payload hashes to the state's model hash; and
 * every member the manifest fixes is the manifest's. */
async function validateRecordedBinding(record: AnalysisRunEnvelope, source: MechanicsResult, binding: RecordedStateBinding): Promise<void> {
  const run = record.analysis_run;
  if (!structurallyEqual(run.model_state_ref, { object_type: "ModelState", ref: binding.stateId })) throw new Error("ANALYSIS_SOURCE_MODEL_STATE_MISMATCH");
  if (typeof run.created_at !== "string" || !run.created_at || run.reproducibility.determinism_notes?.includes(CREATED_AT_UNAVAILABLE)) throw new Error("ANALYSIS_RECORDED_REVISION_TIME_MISSING");
  if (binding.inputManifest.model_basis?.model_ref !== source.model_ref) throw new Error("ANALYSIS_RECORDED_INPUT_MANIFEST_MISMATCH");
  if (`sha256:${await canonicalSha256Hex(binding.inputManifest.model_basis.model_payload)}` !== binding.modelHash) throw new Error("ANALYSIS_RECORDED_MODEL_STATE_HASH_MISMATCH");
  for (const { member, expected, code } of await manifestFixedMembers(binding.inputManifest)) {
    const actual = member === "input_manifest_refs" || member === "input_manifest_hashes" ? run.reproducibility?.[member] : run[member];
    if (!structurallyEqual(actual, expected)) throw new Error(code);
  }
}

/** Source interpretation validation is distinct from checksum verification and producer authentication.
 * A transient record (no `recordedState`) names its synthetic state; a recorded revision
 * (W-12 S2) names the stored state passed in, checked against that state's model hash. */
export async function validateAnalysisRunV03(record: AnalysisRunEnvelope, source: MechanicsResult, expectedBasisRefs?: ObjectRef[], recordedState?: RecordedStateBinding): Promise<void> {
  const same = (a: unknown, b: unknown): boolean => {
    if (a === undefined || b === undefined) return a === b;
    const order = (v: any): any => Array.isArray(v) ? v.map(order) : v && typeof v === "object" ? Object.fromEntries(Object.keys(v).sort().map(k => [k, order(v[k])])) : v;
    return JSON.stringify(order(JSON.parse(checkedJsonText(a)))) === JSON.stringify(order(JSON.parse(checkedJsonText(b))));
  };
  if (!hasCurrentSourceContract(source) || record.schema_version !== ANALYSIS_RUN_V03 || record.run_contract_status.record_contract !== "strict_analysis_run_v0_3") throw new Error("ANALYSIS_SOURCE_CONTRACT_VERSION_MISMATCH");
  if (sourceContract(source) === "preview_physics") readerValidate("preview-physics", source);
  // B7b: a pressure-2 record is read as Python's validator reads it, after the route's reader.
  if (sourceContract(source) === "pressure") readerValidate("pressure", source);
  validateSourceRuleStatus(source);
  const run = record.analysis_run;
  // The recorded form (W-12 S2): every member the stored manifest fixes is checked first,
  // against the manifest; its load basis is the manifest's, never a caller's or the record's.
  if (recordedState) {
    await validateRecordedBinding(record, source, recordedState);
    const manifestBasis = manifestLoadBasisRefs(recordedState.inputManifest);
    if (expectedBasisRefs !== undefined && !structurallyEqual(expectedBasisRefs, manifestBasis)) throw new Error("ANALYSIS_RECORDED_LOAD_BASIS_MISMATCH");
    expectedBasisRefs = manifestBasis;
  }
  if (sourceContract(source) === "pressure_source") {
    await validateRetainedRecoverySource(source);
    if (!same(run.source_block_recovery, source.source_block_recovery)) throw new Error("ANALYSIS_SOURCE_BLOCK_RECEIPT_MISMATCH");
  } else if (Object.hasOwn(run, "source_block_recovery")) throw new Error("SOURCE_RECOVERY_METADATA_CONTRADICTION");
  if (sourceContract(source) === "pressure_source") {
    if (!same(run.contract_evidence, source.contract_evidence)) throw new Error("ANALYSIS_PHYSICS_SOURCE_EVIDENCE_MISMATCH");
  } else if (Object.hasOwn(run, "contract_evidence")) throw new Error("ANALYSIS_PHYSICS_SOURCE_DOWNGRADE_FORBIDDEN");
  // U6d: the successor's record carries its complete receipt, equal to the source's
  // (which the accepted reader verifies); no other record carries one.
  if (isRetainedRoute(sourceContract(source))) {
    await validateRetainedPrecision(source);
    if (!same(run.retained_precision, source.retained_precision)) throw new Error(ANALYSIS_RETAINED_PRECISION_RECEIPT_MISMATCH);
  } else if (Object.hasOwn(run, "retained_precision")) throw new Error(ANALYSIS_RETAINED_PRECISION_DOWNGRADE_FORBIDDEN);
  // The producer's diagnostics, then any rule-pack reference diagnostics (each exactly the
  // one its code and identity determine); a recorded revision's name exactly the references
  // its stored manifest leaves out of active_rule_packs. Their codes are under the record hash.
  const producerDiagnostics = source.diagnostics.map(source_annotation => ({ source_annotation }));
  if (!Array.isArray(run.diagnostics) || run.diagnostics.length < producerDiagnostics.length || !same(run.diagnostics.slice(0, producerDiagnostics.length), producerDiagnostics)) throw new Error("ANALYSIS_SOURCE_DIAGNOSTICS_MISMATCH");
  const rulePackEntries = rulePackRefDiagnosticEntries(run.diagnostics.slice(producerDiagnostics.length));
  if (!rulePackEntries) throw new Error("ANALYSIS_SOURCE_DIAGNOSTICS_MISMATCH");
  const recordedRefusal = recordedState ? rulePackDiagnosticsRefusal(rulePackEntries.map((entry) => entry.identity), recordedState.inputManifest) : null;
  // The named refusal of a run recorded before rule-pack resolution comes only after every
  // other check below passes (the record checksum last), so corruption keeps its own code.
  const deferredRefusal = recordedRefusal === RULE_PACKS_NOT_RESOLVED_REFUSAL ? recordedRefusal : null;
  if (recordedRefusal && !deferredRefusal) throw new Error(recordedRefusal);
  if (!["MECHANICS_SOLVED", "MODEL_INCOMPLETE"].includes(source.status.mechanics)) throw new Error("ANALYSIS_SOURCE_MECHANICS_STATUS_INVALID");
  const statuses = run.analysis_status;
  if (!Array.isArray(statuses) || statuses.length !== 3 || new Set(statuses).size !== 3 || !statuses.includes(source.status.mechanics) || !statuses.includes("HUMAN_REVIEW_REQUIRED") || statuses.filter(s => RULE_STATUSES.includes(s)).length !== 1) throw new Error("ANALYSIS_SOURCE_STATUS_MISMATCH");
  if (!same(run.load_basis_refs, expectedLoadBasis(source, expectedBasisRefs))) throw new Error("ANALYSIS_SOURCE_LOAD_BASIS_MISMATCH");
  if (run.solver_version?.solver_name !== source.producer!.component_name || run.solver_version?.solver_version !== source.producer!.component_version) throw new Error("ANALYSIS_SOURCE_PRODUCER_MISMATCH");
  if (run.run_id !== source.run_id) throw new Error("ANALYSIS_SOURCE_RUN_MISMATCH");
  if (!recordedState && !same(run.model_state_ref, transientModelStateRef(source.model_ref))) throw new Error("ANALYSIS_SOURCE_MODEL_STATE_MISMATCH");
  const contract = recordedRunSemanticBinding(run.reproducibility?.semantic_contract, sourceSemanticBinding(source));
  if (!same(run.reproducibility.semantic_contract, contract)) throw new Error("ANALYSIS_SEMANTIC_CONTRACT_MISMATCH");
  const checksum = async (scope: string, ref: ObjectRef, value: unknown) => ({algorithm: "sha256", canonicalization: CHECKED_PROFILE_V1, payload_ref: ref, payload_scope: scope, value: await canonicalSha256HexCheckedV1(value)});
  if (!same(run.hashes.filter(h => h.payload_scope === "received_result"), [await checksum("received_result", {object_type:"ResultEnvelope",ref:`result-envelope:${source.run_id}`}, source)])) throw new Error("ANALYSIS_RECEIVED_SOURCE_MISMATCH");
  if (source.results.length !== run.result_refs.length || new Set(source.results.map(r => r.id)).size !== source.results.length) throw new Error("ANALYSIS_ROW_ACCOUNTING_MISMATCH");
  const interpret = analysisRowSemanticsReader(source);
  for (const [index, row] of source.results.entries()) {
    const actual = run.result_refs[index], {semantic, findings} = interpret(row), ref = {object_type:"Result",ref:row.id};
    if (actual.source_row_index !== index || !same(actual.result_ref, ref) || !same(actual.semantic_contract, {...contract,signature_id:semantic?.signature_id ?? null})
      || !same(actual.hash_refs, [await checksum("result_row",ref,row)])) throw new Error("ANALYSIS_ROW_SOURCE_BINDING_MISMATCH");
    if (actual.category !== (semantic?.category ?? "unknown") || actual.source_dimension !== (semantic?.source_physical_semantic_dimension ?? null) || actual.result_family !== (semantic?.family ?? null)
      || !same(actual.interpretation, {status:semantic?.canonical_disposition ?? "unavailable",findings})
      || !same(actual.source_annotation, {kind:row.kind,unit:row.unit,metadata:row.metadata ?? null})) throw new Error("ANALYSIS_ROW_INTERPRETATION_MISMATCH");
  }
  if (await verifyAnalysisRunRecord(record) !== "match") throw new Error("ANALYSIS_RECORD_CHECKSUM_MISMATCH");
  if (deferredRefusal) throw new Error(deferredRefusal);
}

/** A pressure-source-1 source is recorded, and a saved record revalidated, only after
 * Rust's full raw reader passes (no invocation), as Python's `_source_contract` reads it
 * (DEL-14-02: a refused source is never recorded). The shallow `source-recovery` check
 * omits raw rows, so a coherently resealed invalid source would pass it. Standing and
 * eligibility stay separate. */
export async function validateRetainedRecoverySource(source: MechanicsResult): Promise<void> {
  const route = sourceContract(source);
  if (route !== "pressure_source") throw new Error("SOURCE_RECOVERY_METADATA_CONTRADICTION");
  try { readerValidate("pressure-source", source); }
  catch (error) {
    // The record/export surfaces keep their refusal vocabulary over Rust's verdict.
    if (error instanceof ResultReaderError) {
      if (error.code === "SOURCE_BLOCKS_RECEIPT_HASH") throw new Error("SOURCE_RECOVERY_RECEIPT_HASH");
      if (error.code === "SOURCE_BLOCKS_PUBLICATION_HASH") throw new Error("SOURCE_RECOVERY_PUBLICATION_HASH");
    }
    throw error;
  }
}
