/** The cases of `apps/desktop/src/test-support/native-reopen/historical_reopen_cases.json`: saved
 * project runs and recorded runs with the findings the unchanged TypeScript contexts give them
 * over their full rows (`buildHistoricalRunContext`, `buildRecordedRunContext`), which the native
 * reopen (`src-tauri/src/analysis_run_v03_validate.rs`, C1 slice 6) must reproduce in the same
 * order from the stored texts (`stored_custody_tests.rs`). Written only with
 * HISTORICAL_REOPEN_CASES_WRITE=1 (`historicalReopenParity.test.ts`); otherwise byte-checked.
 *
 * - **Sources.** Every record the committed builder corpus holds
 *   (`analysis_run_v03_cases.json`, the TypeScript builder's own records over committed fixture
 *   results and invented inputs) as a saved project's run with its model, model hash and envelope
 *   hash; and variants of one record per route (preview-physics, pressure, pressure-source,
 *   retained): altered rows, checksums, versions, references, shapes and numbers, and the
 *   pressure-2 record bound to its 0.8-era recorded table basis. Recorded runs are the preview
 *   cases' records as recorded revisions of an invented state, with their variants.
 * - **Recorded table bases, verbatim.** main's recorded pressure-2 runs, one per recorded basis
 *   pressure-2 carried (`RECORDED_BASIS_FIXTURES`: each a record, its source and its request
 *   model, never rewritten; none of those bases is carried into pressure-3 and
 *   `RECORDED_RUN_TABLE_BASES` is empty, so each run is refused and its source is refused by
 *   name as `SOURCE_PRESSURE_2_RETIRED`, RT-L2, DEL-04-07), each as a recorded revision built as
 *   `recordedRunModel080.test.ts` and `recordedRunModel090Disclosure.test.ts` build it, and, when
 *   its model document is current, as a saved project's run (a project of a retired model
 *   document is refused for editing before its run is read; recorded runs keep their own format).
 *   The cases name the fixture directory instead of copying it; the recorded revision is the
 *   verbatim record with its patches (`record_patches`), and the native leg reads the same files.
 * - **As the old open read them.** Each value is what the webview received from the store
 *   (parsed from JSON, no received text kept), so the readers read a re-serialization, as they
 *   did for a reopened run. No received text, registration or capture is involved: saved and
 *   recorded data never register. */
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import type { AnalysisRunEnvelope, LocalProjectEnvelope } from "../types";
import { buildHistoricalRunContext, buildRecordedRunContext } from "../features/results/HistoricalRunContext";
import { readerDispatch } from "../features/results/resultReader";
import { analysisRecordProjection, manifestFixedMembers, recordedAnalysisRunRevision } from "../services/analysisRunCompatibility";
import { buildCurrentSessionInputManifest } from "../services/inputManifestService";
import { MODEL_SCHEMA_VERSION } from "../services/modelSchemaVersion";
import { canonicalSha256Hex, canonicalSha256HexCheckedV1, computeModelHash, computeProjectEnvelopeHash } from "../services/hashService";
import type { AnalysisRunRead } from "../services/analysisRunService";
import { heldText, type AnalysisRunV03Case } from "./analysisRunV03Cases";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
const ROOT = resolve(__dirname, "../../../../");

export const HISTORICAL_REOPEN_CASES = "apps/desktop/src/test-support/native-reopen/historical_reopen_cases.json";
const BUILDER_CASES = "apps/desktop/src/test-support/analysis-run-v03/analysis_run_v03_cases.json";
/** main's verbatim recorded pressure-2 runs, one per recorded table basis pressure-2 carried, in
 * its order (the native leg names the same directories). None is carried into pressure-3. */
export const RECORDED_BASIS_FIXTURES = [
  "fixtures/persistence/recorded_model_0_7_0",
  "fixtures/persistence/recorded_model_0_8_0",
  "fixtures/persistence/recorded_model_0_9_0_pre_2a",
  "fixtures/persistence/recorded_model_0_9_0_pre_2b",
  "fixtures/persistence/recorded_model_0_9_0_pre_ce",
  "fixtures/persistence/recorded_model_0_9_0_pre_rt_a4",
  "fixtures/persistence/recorded_model_0_9_0_pre_nl",
  "fixtures/persistence/recorded_model_0_9_0_pre_cert",
] as const;
const FIXTURE_RECORD = "pressure_plain_connected_sparse.analysis_run.json";
const FIXTURE_SOURCE = "pressure_connected_mechanics_sparse.json";
const FIXTURE_REQUEST = "exact_pressure_connected_request.json";

type Project = { model: Json; editor_intents: Json; proposal: Json; selected_review_target: Json; model_hash: Json; project_envelope_hash: Json };
/** The contexts' findings in RT-A4 §10.4's scopes (`findings`, `checkFindings`, `recordedTableRead`,
 * `historicalReaderFindings`), with the run's name and recomputed hash. */
type Expected = { findings: string[]; check_findings: string[]; recorded_table_read: Json; historical_reader_findings: string[]; run_id: string; envelope_payload_hash: string | null; model_hash: Json } | { absent: true };
/** A committed fixture file's value at a JSON pointer ("" for the whole file; a recorded case's
 * whole result is the file's exact text). Read, never copied or rewritten. */
export type FileRef = { path: string; pointer: string };
/** A saved case's run: embedded, or (`base`) the builder corpus case whose held text and record it is. */
export type SavedCase = { id: string; kind: "saved"; project: Project; project_text?: string; base?: string; fixture?: string; files?: { result: FileRef; record: FileRef }; patches?: Patch[]; reseal?: boolean; mechanics_result?: Json; analysis_run?: Json; expected: Expected };
type RecordedRead = { run: { state_id: string; producer_run_id: string; run_record_hash: string }; state: { state_id: string; project_id: string; model_hash: string }; record: Json; result_json: string; input_manifest_canonical: string };
/** A recorded case's read: embedded, or (`fixture`) a recorded-basis fixture's source text and its
 * record with `record_patches` applied. */
export type RecordedCase = {
  id: string; kind: "recorded";
  read: RecordedRead | (Omit<RecordedRead, "record" | "result_json"> & { fixture: string; record_patches: Patch[] }) | (Omit<RecordedRead, "record" | "result_json"> & { files: { result: FileRef; record: FileRef } });
  expected: Expected;
};
export type ReopenCase = SavedCase | RecordedCase;

const clone = <T>(value: T): T => JSON.parse(JSON.stringify(value));

/** One edit of a case's run or project, applied identically by the native leg
 * (`stored_custody_tests.rs`): `set` (the empty path replaces the whole value), `delete`, or
 * `append` to an array. */
export type Patch = { on: "result" | "record" | "project"; path: Array<string | number>; op: "set" | "delete" | "append"; value?: Json; raw_json?: string };
function applyPatches<T extends Record<string, Json>>(target: T, patches: Patch[]): T {
  for (const patch of patches) {
    // Raw stored text lets Rust read the exact integer while JS reads binary64.
    const value = patch.raw_json === undefined ? patch.value : JSON.parse(patch.raw_json);
    if (patch.path.length === 0) { (target as Json)[patch.on] = clone(value); continue; }
    let at: Json = target[patch.on];
    for (const key of patch.path.slice(0, -1)) at = at[key];
    const last = patch.path[patch.path.length - 1];
    if (patch.op === "set") at[last] = clone(value);
    else if (patch.op === "delete") delete at[last];
    else at[last].push(clone(value));
  }
  return target;
}

/** The model a builder case's record was made for: its manifest's, or an invented minimal one. */
function modelOf(c: AnalysisRunV03Case, source: Json): Json {
  if (c.kind === "preview") return JSON.parse(c.input_manifest.canonical_bytes).model_basis.model_payload;
  const payload = c.manifest?.manifest?.model_basis?.model_payload;
  if (payload && typeof payload === "object" && typeof payload.project?.id === "string") return payload;
  return { schema_version: "0.9.0", project: { id: source.model_ref, name: "Invented reopen model", units: {} }, load_cases: [], combinations: [] };
}

async function project(model: Json, result: Json, record: Json, editor: Json = [], drop: { modelHash?: boolean; envelope?: boolean } = {}): Promise<Project> {
  const modelHash = drop.modelHash ? null : await computeModelHash(model);
  const envelope = drop.envelope ? null : await computeProjectEnvelopeHash({ model, editor_intents: editor, proposal: null, selected_review_target: null, mechanics_result: result, analysis_run: record, model_hash: modelHash });
  return { model, editor_intents: editor, proposal: null, selected_review_target: null, model_hash: modelHash, project_envelope_hash: envelope };
}

async function savedExpected(p: Project, result: Json, record: Json): Promise<Expected> {
  const opened = { summary: {}, ...clone(p), mechanics_result: clone(result), analysis_run: clone(record) } as unknown as LocalProjectEnvelope;
  const context = await buildHistoricalRunContext(opened);
  if (!context) return { absent: true };
  return { findings: context.findings, check_findings: context.checkFindings, recorded_table_read: context.recordedTableRead, historical_reader_findings: context.historicalReaderFindings, run_id: context.runId, envelope_payload_hash: context.envelopePayloadHash, model_hash: null };
}

async function saved(id: string, p: Project, result: Json, record: Json, base?: string): Promise<SavedCase> {
  const expected = await savedExpected(p, result, record);
  return base ? { id, kind: "saved", project: p, base, expected } : { id, kind: "saved", project: p, mechanics_result: result, analysis_run: record, expected };
}
const size = (value: unknown) => JSON.stringify(value).length;

/** A record resealed after an edit: its `analysis_run_record` checksum recomputed. */
async function resealed(record: Json): Promise<Json> {
  const out = clone(record);
  const claim = out.analysis_run.hashes.find((hash: Json) => hash.payload_scope === "analysis_run_record");
  claim.value = await canonicalSha256HexCheckedV1(analysisRecordProjection(out));
  return out;
}

const routeOf = (record: Json): string => String(record?.analysis_run?.reproducibility?.semantic_contract?.id ?? "");

async function savedCases(base: Array<{ c: AnalysisRunV03Case; source: Json; record: Json }>): Promise<SavedCase[]> {
  const out: SavedCase[] = [];
  for (const { c, source, record } of base) {
    const model = modelOf(c, source);
    out.push(await saved(`saved:${c.id}`, await project(model, source, record), source, record, c.id));
  }
  // The variants embed their runs: the smallest record of each route with at least two rows.
  const pick = (needle: string) => base
    .filter(({ record, source }) => routeOf(record).includes(needle) && source.results.length >= 2)
    .sort((a, b) => size(a.source) + size(a.record) - size(b.source) - size(b.record))[0];
  const picks = [["preview", pick("preview-physics-2")], ["pressure", pick("pressure-2")], ["source", pick("pressure-source")], ["retained", pick("retained")]] as const;
  for (const [route, found] of picks) {
    if (!found) throw new Error(`no ${route} record in the builder corpus`);
    const { c, source, record } = found;
    const model = modelOf(c, source);
    const variant = async (name: string, patches: Patch[], opts: { reseal?: boolean; editor?: Json; drop?: { modelHash?: boolean; envelope?: boolean }; projectPatches?: Patch[] } = {}) => {
      const run = applyPatches({ result: clone(source), record: clone(record) }, patches);
      if (opts.reseal) run.record = await resealed(run.record);
      const p = await project(model, run.result, run.record, opts.editor ?? [], opts.drop ?? {});
      const patchedProject = applyPatches({ project: p }, opts.projectPatches ?? []).project as Project;
      out.push({
        id: `saved-variant:${route}:${name}`, kind: "saved", project: patchedProject, base: c.id, patches, ...(opts.reseal ? { reseal: true } : {}),
        expected: await savedExpected(patchedProject, run.result, run.record)
      });
    };
    const hashIndex = (scope: string) => record.analysis_run.hashes.findIndex((h: Json) => h.payload_scope === scope);
    await variant("row-value-changed", [{ on: "result", path: ["results", 0, "value"], op: "set", value: (source.results[0].value ?? 0) + 1 }]);
    await variant("row-id-missing", [{ on: "result", path: ["results", 0, "id"], op: "delete" }]);
    await variant("row-unsafe-integer", [{ on: "result", path: ["results", 0, "value"], op: "set", value: 9007199254740992 }]);
    await variant("wide-stored-result-integer", [{ on: "result", path: ["edge_probe"], op: "set", raw_json: "[9007199254740993,12345678901234567890,-9007199254740993]" }]);
    const wideModel = { ...clone(model), edge_probe: JSON.parse("9007199254740993") };
    const wideProject = await project(wideModel, source, record);
    const projectText = JSON.stringify(wideProject).replace('"edge_probe":9007199254740992', '"edge_probe":9007199254740993');
    const receivedProject = JSON.parse(projectText) as Project;
    out.push({
      id: `saved-variant:${route}:wide-stored-model-integer`, kind: "saved", project: receivedProject,
      project_text: projectText, base: c.id, expected: await savedExpected(receivedProject, source, record)
    });
    await variant("duplicate-row-id", [{ on: "result", path: ["results", 1, "id"], op: "set", value: source.results[0].id }]);
    await variant("record-checksum-altered", [{ on: "record", path: ["analysis_run", "hashes", hashIndex("analysis_run_record"), "value"], op: "set", value: "0".repeat(64) }]);
    await variant("record-retired-0.2.0", [{ on: "record", path: ["schema_version"], op: "set", value: "0.2.0" }]);
    await variant("record-version-unknown", [{ on: "record", path: ["schema_version"], op: "set", value: "0.4.0" }]);
    await variant("record-absent", [{ on: "record", path: [], op: "set", value: null }]);
    await variant("result-absent", [{ on: "result", path: [], op: "set", value: null }]);
    await variant("both-hashes-absent", [], { drop: { modelHash: true, envelope: true } });
    await variant("model-hash-stale", [], { projectPatches: [{ on: "project", path: ["model_hash", "value"], op: "set", value: `sha256:${"1".repeat(64)}` }] });
    await variant("envelope-hash-stale", [], { projectPatches: [{ on: "project", path: ["project_envelope_hash", "value"], op: "set", value: `sha256:${"2".repeat(64)}` }] });
    await variant("model-ref-other", [{ on: "result", path: ["model_ref"], op: "set", value: "model:another-project" }]);
    await variant("state-ref-other", [{ on: "record", path: ["analysis_run", "model_state_ref"], op: "set", value: { object_type: "ModelState", ref: "state:another:preview" } }], { reseal: true });
    await variant("run-id-other", [{ on: "record", path: ["analysis_run", "run_id"], op: "set", value: "run:another" }], { reseal: true });
    await variant("result-refs-string", [{ on: "record", path: ["analysis_run", "result_refs"], op: "set", value: "x".repeat(source.results.length) }]);
    await variant("hashes-not-array", [{ on: "record", path: ["analysis_run", "hashes"], op: "set", value: {} }]);
    await variant("analysis-run-null", [{ on: "record", path: ["analysis_run"], op: "set", value: null }]);
    await variant("record-scalar", [{ on: "record", path: [], op: "set", value: "not a record" }]);
    await variant("result-scalar", [{ on: "result", path: [], op: "set", value: 42 }]);
    await variant("received-hash-prefixed", [{ on: "record", path: ["analysis_run", "hashes", hashIndex("received_result"), "value"], op: "set", value: `sha256:${record.analysis_run.hashes[hashIndex("received_result")].value}` }]);
    await variant("diagnostics-extra", [{ on: "record", path: ["analysis_run", "diagnostics"], op: "append", value: { source_annotation: { code: "X", severity: "info", message: "invented" } } }], { reseal: true });
    await variant("statuses-duplicated", [{ on: "record", path: ["analysis_run", "analysis_status"], op: "set", value: ["HUMAN_REVIEW_REQUIRED", "HUMAN_REVIEW_REQUIRED", "MECHANICS_SOLVED"] }], { reseal: true });
    await variant("producer-other", [{ on: "record", path: ["analysis_run", "solver_version", "solver_version"], op: "set", value: "9.9.9" }], { reseal: true });
    await variant("row-ref-index-shifted", [{ on: "record", path: ["analysis_run", "result_refs", 0, "source_row_index"], op: "set", value: 1 }], { reseal: true });
    await variant("row-ref-category-other", [{ on: "record", path: ["analysis_run", "result_refs", 0, "category"], op: "set", value: "invented" }], { reseal: true });
    await variant("contract-other", [{ on: "record", path: ["analysis_run", "reproducibility", "semantic_contract"], op: "set", value: { id: "invented", sha256: "0".repeat(64) } }], { reseal: true });
    await variant("editor-intents-null", [], { editor: null });
    if (route === "pressure") {
      // A run recorded under the 0.8-era pressure-2 table (S1's `RECORDED_RUN_TABLE_BASES`):
      // the record binds that exact digest; read as the recorded basis of the current table.
      const basis = (sha: string): Patch[] => [
        { on: "record", path: ["analysis_run", "reproducibility", "semantic_contract", "sha256"], op: "set", value: sha },
        ...record.analysis_run.result_refs.map((_: Json, index: number) => ({ on: "record", path: ["analysis_run", "result_refs", index, "semantic_contract", "sha256"], op: "set", value: sha }) as Patch)
      ];
      await variant("recorded-0.8-table-basis", basis("5451fbf3e10e085c90b523baa7daf5fc723659e0d874d988d23807be7d2fefe6"), { reseal: true });
      await variant("unknown-table-basis", basis("6".repeat(64)), { reseal: true });
    }
    if (route === "retained") await variant("receipt-copy-altered", [{ on: "record", path: ["analysis_run", "retained_precision"], op: "set", value: { invented: true } }], { reseal: true });
    if (route === "source") await variant("recovery-copy-altered", [{ on: "record", path: ["analysis_run", "source_block_recovery"], op: "set", value: { invented: true } }], { reseal: true });
  }
  out.push(await saved("saved:neither", await project({ schema_version: "0.9.0", project: { id: "model:empty", name: "Empty", units: {} }, load_cases: [] }, null, null), null, null));
  return out;
}

async function recordedExpected(read: RecordedRead): Promise<Expected> {
  const context = await buildRecordedRunContext({ run: clone(read.run), state: clone(read.state), record: clone(read.record), result_json: read.result_json, input_manifest_canonical: read.input_manifest_canonical } as unknown as AnalysisRunRead);
  return { findings: context.findings, check_findings: context.checkFindings, recorded_table_read: context.recordedTableRead, historical_reader_findings: context.historicalReaderFindings, run_id: context.runId, envelope_payload_hash: null, model_hash: context.modelHash };
}

async function recordedCases(base: Array<{ c: AnalysisRunV03Case; source: Json; record: Json }>): Promise<RecordedCase[]> {
  const out: RecordedCase[] = [];
  // The four smallest preview records, embedded.
  const previews = base.filter(({ c }) => c.kind === "preview").sort((a, b) => size(a.source) + size(a.record) - size(b.source) - size(b.record)).slice(0, 2);
  for (const [index, { c, source, record }] of previews.entries()) {
    if (c.kind !== "preview") continue;
    const manifestText = c.input_manifest.canonical_bytes;
    const manifest = JSON.parse(manifestText);
    const stateId = `state:reopen:${index}`;
    const revision = await recordedAnalysisRunRevision(record as AnalysisRunEnvelope, { stateId, createdAt: "2026-11-01T00:00:00.000Z", inputManifest: manifest });
    const modelHash = `sha256:${await canonicalSha256Hex(manifest.model_basis.model_payload)}`;
    const runHash = revision.analysis_run.hashes.find(h => h.payload_scope === "analysis_run_record")!.value;
    const make = async (name: string, edit: (read: RecordedRead) => void = () => {}) => {
      const read: RecordedRead = {
        run: { state_id: stateId, producer_run_id: revision.analysis_run.run_id, run_record_hash: runHash },
        state: { state_id: stateId, project_id: source.model_ref, model_hash: modelHash },
        record: clone(revision), result_json: JSON.stringify(source), input_manifest_canonical: manifestText,
      };
      edit(read);
      out.push({ id: `recorded:${c.id}${name ? `:${name}` : ""}`, kind: "recorded", read, expected: await recordedExpected(read) });
    };
    await make("");
    if (index === 0) {
      await make("state-hash-other", read => { read.state.model_hash = `sha256:${"3".repeat(64)}`; });
      await make("run-record-hash-other", read => { read.run.run_record_hash = "4".repeat(64); });
      await make("producer-run-other", read => { read.run.producer_run_id = "run:another"; });
      await make("run-state-other", read => { read.run.state_id = "state:another"; });
      await make("result-not-json", read => { read.result_json = "{not json"; });
      await make("manifest-not-json", read => { read.input_manifest_canonical = "{not json"; });
      await make("project-other", read => { read.state.project_id = "model:another"; });
      await make("created-at-absent", read => { read.record.analysis_run.created_at = null; });
    }
  }
  return out;
}

/** The patches that turn `before` into `after` (`applyPatches` on the record). */
function recordPatches(before: Json, after: Json, path: Array<string | number> = []): Patch[] {
  if (JSON.stringify(before) === JSON.stringify(after)) return [];
  const isObject = (value: Json) => value !== null && typeof value === "object" && !Array.isArray(value);
  if (Array.isArray(before) && Array.isArray(after) && before.length === after.length) {
    return before.flatMap((item: Json, index: number) => recordPatches(item, after[index], [...path, index]));
  }
  if (isObject(before) && isObject(after)) {
    const out: Patch[] = Object.keys(before).filter(key => !(key in after)).map(key => ({ on: "record", path: [...path, key], op: "delete" }) as Patch);
    for (const key of Object.keys(after)) {
      if (key in before) out.push(...recordPatches(before[key], after[key], [...path, key]));
      else out.push({ on: "record", path: [...path, key], op: "set", value: clone(after[key]) });
    }
    return out;
  }
  return [{ on: "record", path, op: "set", value: clone(after) }];
}

/** The retired refusal's code and message, as the reader states it. */
export const PRESSURE_2_RETIRED_TEXT = "SOURCE_PRESSURE_2_RETIRED: pressure-2 results predate the pressure elongation setting and are no longer read; re-solve the model";
/** A reopened pressure-2 run tells the user it predates the pressure elongation setting and must be
 * re-solved: the retired code and message as its standing and its check, never the unsupported or
 * version-mismatch collapse. */
function toldRetired(fixture: string, expected: Json): void {
  const collapsed = ["SOURCE_NUMERICAL_CONTRACT_UNSUPPORTED", "ANALYSIS_SOURCE_CONTRACT_VERSION_MISMATCH"];
  if (!expected.findings.includes(PRESSURE_2_RETIRED_TEXT) || !expected.check_findings.includes(PRESSURE_2_RETIRED_TEXT)) throw new Error(`${fixture}: the reopened run is not refused as retired (${JSON.stringify(expected.findings)})`);
  if (expected.findings.some((finding: string) => collapsed.includes(finding))) throw new Error(`${fixture}: the retired refusal is collapsed (${JSON.stringify(expected.findings)})`);
}

/** Each recorded-basis fixture, verbatim, as a recorded revision and (current model documents) a saved project's run. */
async function recordedBasisCases(): Promise<{ saved: SavedCase[]; recorded: RecordedCase[] }> {
  const saved: SavedCase[] = [], recorded: RecordedCase[] = [];
  for (const fixture of RECORDED_BASIS_FIXTURES) {
    const text = (name: string) => readFileSync(resolve(ROOT, fixture, name), "utf8");
    const name = fixture.slice(fixture.lastIndexOf("/") + 1);
    const record = JSON.parse(text(FIXTURE_RECORD)), sourceText = text(FIXTURE_SOURCE), source = JSON.parse(sourceText);
    const model = JSON.parse(text(FIXTURE_REQUEST)).model;
    // RT-L2: a pressure-2 source is refused by name, never read as a current result.
    const refusal = readerDispatch(source).error?.code;
    if (refusal !== "SOURCE_PRESSURE_2_RETIRED") throw new Error(`${fixture}: the pressure-2 source is not refused as retired (${refusal})`);
    // As a saved project (current model documents only): the request's model, the verbatim source and record.
    if (model.schema_version === MODEL_SCHEMA_VERSION) {
      const p = await project(clone(model), source, record);
      const expected = await savedExpected(p, source, record);
      if (!("findings" in expected) || expected.findings.length === 0) throw new Error(`${fixture}: a saved pressure-2 run reopened clean`);
      toldRetired(fixture, expected);
      saved.push({ id: `saved-recorded-basis:${name}`, kind: "saved", project: p, fixture, expected });
    }
    // As a recorded revision of an invented state, built as the webview tests build it.
    const authored = clone(model);
    for (const c of authored.load_cases) { c.kind ||= "sustained"; c.status ||= "ready_for_preview_diagnostics"; c.provenance ||= "invented review input"; }
    const manifest = await buildCurrentSessionInputManifest({ model: authored, solver: { solver_name: source.producer.component_name, solver_version: source.producer.component_version, solver_build_ref: record.analysis_run.solver_version.build_ref.ref, solver_mode: "sparse_interactive", settings: {} }, active_rule_packs: [], external_assets: [] });
    const run = clone(record);
    for (const { member, expected } of await manifestFixedMembers(manifest.manifest)) {
      if (member === "input_manifest_refs" || member === "input_manifest_hashes") run.analysis_run.reproducibility[member] = expected; else run.analysis_run[member] = expected;
    }
    const stateId = `state:recorded-basis:${name}`;
    const revision = await recordedAnalysisRunRevision(run as AnalysisRunEnvelope, { stateId, createdAt: "2026-11-01T00:00:00.000Z", inputManifest: manifest.manifest });
    const runHash = revision.analysis_run.hashes.find(h => h.payload_scope === "analysis_run_record")!.value;
    const read: RecordedRead = {
      run: { state_id: stateId, producer_run_id: source.run_id, run_record_hash: runHash },
      state: { state_id: stateId, project_id: model.project.id, model_hash: `sha256:${await canonicalSha256Hex(manifest.manifest.model_basis.model_payload)}` },
      record: revision, result_json: sourceText, input_manifest_canonical: manifest.canonical_bytes,
    };
    const record_patches = recordPatches(record, revision);
    const patched = applyPatches({ record: clone(record) } as Record<string, Json>, record_patches).record;
    if (await canonicalSha256Hex(patched) !== await canonicalSha256Hex(revision)) throw new Error(`${fixture}: the record patches do not give the recorded revision`);
    const { record: _record, result_json: _result, ...rest } = read;
    const expected = await recordedExpected(read);
    if (!("findings" in expected) || expected.findings.length === 0) throw new Error(`${fixture}: a pressure-2 run reopened clean`);
    toldRetired(fixture, expected);
    recorded.push({ id: `recorded-basis:${name}`, kind: "recorded", read: { ...rest, fixture, record_patches }, expected });
  }
  return { saved, recorded };
}

const PRE_RT_A4 = "fixtures/persistence/recorded_pre_rt_a4_pf";
const file = (path: string, pointer = ""): FileRef => ({ path, pointer });
function fileValue(ref: FileRef): Json {
  const value = JSON.parse(readFileSync(resolve(ROOT, ref.path), "utf8"));
  return ref.pointer.split("/").slice(1).reduce((at: Json, key: string) => at[key.replace(/~1/g, "/").replace(/~0/g, "~")], value);
}
/** RT-A4 §10.4: main's runs recorded before RT-A4 (`recorded_pre_rt_a4_pf`, verbatim), one per
 * registry variant with a renderable record, each as a saved project's run (its model as
 * `recordedPreRtA4Display.test.tsx` reopens it); the preview-physics-2 run also with its binding's
 * digest unknown (read under the current tables: no recorded read, no historical finding), and
 * main's recorded revision of it as a recorded run read from the store. */
async function preRtA4Cases(): Promise<{ saved: SavedCase[]; recorded: RecordedCase[] }> {
  const pf2 = `${PRE_RT_A4}/preview_physics_2`, timoshenko = `${PRE_RT_A4}/preview_physics_timoshenko_1`;
  const transient = file(`${pf2}/transient_run_input.json`);
  const retained = (dir: string, name: string) => ({ result: file(`${PRE_RT_A4}/${dir}/${name}.json`, "/source"), record: file(`${PRE_RT_A4}/${dir}/${name}.analysis_run.json`), model: () => fileValue(file(`${PRE_RT_A4}/${dir}/${name}.json`, "/invocation/request/model")) });
  const variants: Array<{ name: string; result: FileRef; record: FileRef; model: () => Json }> = [
    { name: "preview-physics-2", result: file(`${pf2}/preview_physics_connected_sparse.json`), record: file(transient.path, "/run_record"),
      model: () => JSON.parse(fileValue(file(transient.path, "/input_manifest_canonical"))).model_basis.model_payload },
    { name: "preview-physics-timoshenko-1", result: file(`${timoshenko}/preview_physics_timoshenko_straight_sparse.json`), record: file(`${timoshenko}/preview_physics_timoshenko_straight_sparse.analysis_run.json`),
      // Main holds no model of this statement (fixture README, "Gap"): one naming its project and load cases.
      model: () => {
        const source = fileValue(file(`${timoshenko}/preview_physics_timoshenko_straight_sparse.json`));
        const cases = [...new Set((source.results as Json[]).flatMap((row) => (row.basis_ref?.ref_type === "load_case" ? [row.basis_ref.ref_id] : [])))];
        return { schema_version: "0.9.0", project: { id: source.model_ref }, model_settings: { shear_model: "timoshenko_annulus_energy_v1" }, load_cases: cases.map((id) => ({ id })), diagnostics: [] };
      } },
    { name: "preview-physics-retained-1", ...retained("preview_physics_retained_1", "retained_precision_milestone_successor_sparse_interactive") },
    { name: "physics-retained-1-main", ...retained("physics_retained_1/main", "retained_precision_exact_successor_sparse_interactive") },
    { name: "physics-retained-1-pr101", ...retained("physics_retained_1/pr101", "retained_precision_exact_successor_sparse_interactive") },
    // RT-A3 increment 4: main's RT-A4 exact successor (040ef9a6…, a recorded binding only).
    { name: "physics-retained-1-main-bbf384dec", ...retained("physics_retained_1/main_bbf384dec", "retained_precision_exact_successor_sparse_interactive") },
    // RT-A3 §4: main's exact successor at 252bef969 (85c2678a…, a recorded binding only).
    { name: "physics-retained-1-main-252bef969", ...retained("physics_retained_1/main_252bef969", "retained_precision_exact_successor_sparse_interactive") },
  ];
  const saved: SavedCase[] = [];
  for (const variant of variants) {
    const result = fileValue(variant.result), record = fileValue(variant.record);
    const p = await project(clone(variant.model()), result, record);
    saved.push({ id: `saved-pre-rt-a4:${variant.name}`, kind: "saved", project: p, files: { result: variant.result, record: variant.record }, expected: await savedExpected(p, result, record) });
    if (variant.name === "preview-physics-2") {
      // An unknown digest names no variant: the run is read under the current tables, as before.
      const patches: Patch[] = [{ on: "record", path: ["analysis_run", "reproducibility", "semantic_contract", "sha256"], op: "set", value: "0".repeat(64) }];
      const patched = applyPatches({ record: clone(record) } as Record<string, Json>, patches).record;
      const q = await project(clone(variant.model()), result, patched);
      saved.push({ id: `saved-pre-rt-a4:${variant.name}:unknown-digest`, kind: "saved", project: q, files: { result: variant.result, record: variant.record }, patches, expected: await savedExpected(q, result, patched) });
    }
  }
  // main's recorded revision of its preview-physics-2 run, as the store returns it.
  const witness = fileValue(file(`${pf2}/recorded_run_witness.json`));
  const manifestText = fileValue(file(transient.path, "/input_manifest_canonical")) as string;
  const read: RecordedRead = {
    run: { state_id: witness.state_id, producer_run_id: witness.recorded_record.analysis_run.run_id, run_record_hash: witness.run_record_hash },
    state: { state_id: witness.state_id, project_id: JSON.parse(manifestText).model_basis.model_ref, model_hash: witness.state_model_hash },
    record: witness.recorded_record, result_json: readFileSync(resolve(ROOT, `${pf2}/preview_physics_connected_sparse.json`), "utf8"), input_manifest_canonical: manifestText,
  };
  const { record: _record, result_json: _result, ...rest } = read;
  const recorded: RecordedCase[] = [{ id: "recorded-pre-rt-a4:preview-physics-2", kind: "recorded",
    read: { ...rest, files: { result: file(`${pf2}/preview_physics_connected_sparse.json`), record: file(`${pf2}/recorded_run_witness.json`, "/recorded_record") } },
    expected: await recordedExpected(read) }];
  return { saved, recorded };
}

const PRE_RT_A5_1B = "fixtures/persistence/recorded_pre_rt_a5_1b_pf";
const PRE_RT_A5_MF = "fixtures/persistence/recorded_pre_rt_a5_mf_pf";
/** Operating-case slice 1b (PF1b, DEL-05-02 `operating-case.md` "As built (1b)"): main's
 * preview-physics-2 run recorded before 1b restated the combination policy (verbatim; it binds
 * e7b93f41…, a registry variant), as a saved project's run and as main's recorded revision of it.
 * RT-A5 `minuend_final` (`operating-case.md` §7.6): main's run recorded before it restated the
 * policy again (`recorded_pre_rt_a5_mf_pf`, verbatim; it binds f41b9d3e…), the same two ways.
 * Main holds no record of a timoshenko-1 or retained-1 run before either (the fixture READMEs'
 * "Gap"). */
async function prePolicyRestatementCases(dir = PRE_RT_A5_1B, tag = "pre-rt-a5-1b"): Promise<{ saved: SavedCase[]; recorded: RecordedCase[] }> {
  const pf2 = `${dir}/preview_physics_2`;
  const transient = file(`${pf2}/transient_run_input.json`);
  const resultRef = file(`${pf2}/preview_physics_connected_sparse.json`), recordRef = file(transient.path, "/run_record");
  const result = fileValue(resultRef), record = fileValue(recordRef);
  const manifestText = fileValue(file(transient.path, "/input_manifest_canonical")) as string;
  const p = await project(clone(JSON.parse(manifestText).model_basis.model_payload), result, record);
  const saved: SavedCase[] = [{ id: `saved-${tag}:preview-physics-2`, kind: "saved", project: p, files: { result: resultRef, record: recordRef }, expected: await savedExpected(p, result, record) }];
  const witness = fileValue(file(`${pf2}/recorded_run_witness.json`));
  const read: RecordedRead = {
    run: { state_id: witness.state_id, producer_run_id: witness.recorded_record.analysis_run.run_id, run_record_hash: witness.run_record_hash },
    state: { state_id: witness.state_id, project_id: JSON.parse(manifestText).model_basis.model_ref, model_hash: witness.state_model_hash },
    record: witness.recorded_record, result_json: readFileSync(resolve(ROOT, resultRef.path), "utf8"), input_manifest_canonical: manifestText,
  };
  const { record: _record, result_json: _result, ...rest } = read;
  const recorded: RecordedCase[] = [{ id: `recorded-${tag}:preview-physics-2`, kind: "recorded",
    read: { ...rest, files: { result: resultRef, record: file(`${pf2}/recorded_run_witness.json`, "/recorded_record") } },
    expected: await recordedExpected(read) }];
  return { saved, recorded };
}

export type HistoricalReopenCases = Readonly<{ document_kind: string; version: number; note: string; cases: ReopenCase[] }>;

/** Every case, in a fixed order, with the TypeScript contexts' findings. */
export async function historicalReopenCases(): Promise<HistoricalReopenCases> {
  const builder = JSON.parse(readFileSync(resolve(ROOT, BUILDER_CASES), "utf8")) as { cases: AnalysisRunV03Case[] };
  const base = builder.cases.filter(c => "record" in c.expected).map(c => ({ c, source: JSON.parse(heldText(c.source)), record: (c.expected as { record: Json }).record }));
  const basis = await recordedBasisCases();
  const preRtA4 = await preRtA4Cases();
  const preRtA51b = await prePolicyRestatementCases();
  const preRtA5Mf = await prePolicyRestatementCases(PRE_RT_A5_MF, "pre-rt-a5-mf");
  const cases: ReopenCase[] = [...(await savedCases(base)), ...basis.saved, ...preRtA4.saved, ...preRtA51b.saved, ...preRtA5Mf.saved, ...(await recordedCases(base)), ...basis.recorded, ...preRtA4.recorded, ...preRtA51b.recorded, ...preRtA5Mf.recorded];
  const ids = new Set(cases.map(c => c.id));
  if (ids.size !== cases.length) throw new Error("duplicate case ids");
  return {
    document_kind: "swbpipe.desktop.historical_reopen_cases",
    version: 1,
    note: "Written by apps/desktop/src/test-support/historicalReopenCases.ts (HISTORICAL_REOPEN_CASES_WRITE=1): the unchanged TypeScript historical contexts' findings over full rows, which the native reopen must reproduce in order. Committed-fixture and invented inputs only.",
    cases,
  };
}

/** The file's text: one case per line. */
export function reopenCasesText(file: HistoricalReopenCases): string {
  const { cases, ...head } = file;
  const opening = JSON.stringify(head).slice(0, -1);
  return `${opening},"cases":[\n${cases.map(c => JSON.stringify(c)).join(",\n")}\n]}\n`;
}
