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
 * - **As the old open read them.** Each value is what the webview received from the store
 *   (parsed from JSON, no received text kept), so the readers read a re-serialization, as they
 *   did for a reopened run. No received text, registration or capture is involved: saved and
 *   recorded data never register. */
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import type { AnalysisRunEnvelope, LocalProjectEnvelope } from "../types";
import { buildHistoricalRunContext, buildRecordedRunContext } from "../features/results/HistoricalRunContext";
import { analysisRecordProjection, recordedAnalysisRunRevision } from "../services/analysisRunCompatibility";
import { canonicalSha256Hex, canonicalSha256HexCheckedV1, computeModelHash, computeProjectEnvelopeHash } from "../services/hashService";
import type { AnalysisRunRead } from "../services/analysisRunService";
import { heldText, type AnalysisRunV03Case } from "./analysisRunV03Cases";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
const ROOT = resolve(__dirname, "../../../../");

export const HISTORICAL_REOPEN_CASES = "apps/desktop/src/test-support/native-reopen/historical_reopen_cases.json";
const BUILDER_CASES = "apps/desktop/src/test-support/analysis-run-v03/analysis_run_v03_cases.json";

type Project = { model: Json; editor_intents: Json; proposal: Json; selected_review_target: Json; model_hash: Json; project_envelope_hash: Json };
type Expected = { findings: string[]; run_id: string; envelope_payload_hash: string | null; model_hash: Json } | { absent: true };
/** A saved case's run: embedded, or (`base`) the builder corpus case whose held text and record it is. */
export type SavedCase = { id: string; kind: "saved"; project: Project; base?: string; patches?: Patch[]; reseal?: boolean; mechanics_result?: Json; analysis_run?: Json; expected: Expected };
export type RecordedCase = {
  id: string; kind: "recorded";
  read: { run: { state_id: string; producer_run_id: string; run_record_hash: string }; state: { state_id: string; project_id: string; model_hash: string }; record: Json; result_json: string; input_manifest_canonical: string };
  expected: Expected;
};
export type ReopenCase = SavedCase | RecordedCase;

const clone = <T>(value: T): T => JSON.parse(JSON.stringify(value));

/** One edit of a case's run or project, applied identically by the native leg
 * (`stored_custody_tests.rs`): `set` (the empty path replaces the whole value), `delete`, or
 * `append` to an array. */
export type Patch = { on: "result" | "record" | "project"; path: Array<string | number>; op: "set" | "delete" | "append"; value?: Json };
function applyPatches<T extends Record<string, Json>>(target: T, patches: Patch[]): T {
  for (const patch of patches) {
    if (patch.path.length === 0) { (target as Json)[patch.on] = clone(patch.value); continue; }
    let at: Json = target[patch.on];
    for (const key of patch.path.slice(0, -1)) at = at[key];
    const last = patch.path[patch.path.length - 1];
    if (patch.op === "set") at[last] = clone(patch.value);
    else if (patch.op === "delete") delete at[last];
    else at[last].push(clone(patch.value));
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
  return { findings: context.findings, run_id: context.runId, envelope_payload_hash: context.envelopePayloadHash, model_hash: null };
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

async function recordedExpected(read: RecordedCase["read"]): Promise<Expected> {
  const context = await buildRecordedRunContext({ run: clone(read.run), state: clone(read.state), record: clone(read.record), result_json: read.result_json, input_manifest_canonical: read.input_manifest_canonical } as unknown as AnalysisRunRead);
  return { findings: context.findings, run_id: context.runId, envelope_payload_hash: null, model_hash: context.modelHash };
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
    const make = async (name: string, edit: (read: RecordedCase["read"]) => void = () => {}) => {
      const read: RecordedCase["read"] = {
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

export type HistoricalReopenCases = Readonly<{ document_kind: string; version: number; note: string; cases: ReopenCase[] }>;

/** Every case, in a fixed order, with the TypeScript contexts' findings. */
export async function historicalReopenCases(): Promise<HistoricalReopenCases> {
  const builder = JSON.parse(readFileSync(resolve(ROOT, BUILDER_CASES), "utf8")) as { cases: AnalysisRunV03Case[] };
  const base = builder.cases.filter(c => "record" in c.expected).map(c => ({ c, source: JSON.parse(heldText(c.source)), record: (c.expected as { record: Json }).record }));
  const cases: ReopenCase[] = [...(await savedCases(base)), ...(await recordedCases(base))];
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
