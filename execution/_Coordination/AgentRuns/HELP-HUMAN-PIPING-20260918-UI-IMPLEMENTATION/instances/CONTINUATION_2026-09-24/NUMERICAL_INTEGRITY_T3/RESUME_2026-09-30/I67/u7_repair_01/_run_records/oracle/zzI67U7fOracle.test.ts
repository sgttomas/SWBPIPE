// I67 U7 slice F part 2, the TS oracle-diff dumper (scratch lanes only, never committed).
// Copied into src/features/results/ of a lane and run there on I66's 380 inputs
// (I67_ORACLE_INPUTS; inputs.json sha256 52026a92...), with the candidate's case file
// (I67_ORACLE_CASES) for the TS delivery of carrier and declared inputs. Per input:
// - the reader: outcome, classes digest, invocation_bound, numerical_eligible, standing;
//   its transport check; the carriers' transport route;
// - `live`: the input's own invocation registered with a stand-in live capture (`() => true`),
//   standing for its requested refs and the summary for its invocation's cases (TS's
//   standing rule with the capture binding held true; comparable to Python and Rust);
// - `ipc` (carrier, declared and milestone inputs): TS's real delivery through mocked IPC,
//   as the integration test's applyShared (capture, `capture: "none"`, current_model_edits);
// - the binding refusals of every row, for inputs marked `binding`.
import { it, vi } from "vitest";
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", () => ({ invoke: invokeMock }));
import { createHash } from "node:crypto";
import { readFileSync, writeFileSync } from "node:fs";
import type { MechanicsResult, PreviewModel } from "../../types";
import { numericalResultStanding, sourceContract, sourceContractTransport } from "./numericalResultQuality";
import { RetainedPrecisionError, validateRetainedPrecision, validateRetainedPrecisionTransport } from "./retainedPrecision";
import { classificationSummary, registerRetainedPrecision, retainedPrecisionStanding } from "./retainedPrecisionStanding";
import { ruleBindingRefusal } from "./knownSemanticLimitations";
import { runPreviewMechanics } from "../../services/previewService";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
const digest = (v: unknown) => createHash("sha256").update(JSON.stringify(v)).digest("hex").slice(0, 16);
const casesModel = (ids: string[]) => ({ load_cases: ids.map(id => ({ id })) }) as unknown as PreviewModel;
const standingOf = (s: MechanicsResult, model: PreviewModel | null) => {
  const route = sourceContract(s);
  const r = route === "retained_preview_physics" ? retainedPrecisionStanding(s, model) : null;
  const n = numericalResultStanding(s, model);
  return { route, token: r ? r.standing : route === "unsupported" ? "unsupported" : `route:${route}`, findings: r ? r.findings : n.findings, status: n.status, eligible: n.eligible };
};
async function deliver(source: MechanicsResult, model: PreviewModel | null, mode: string): Promise<MechanicsResult> {
  (window as Json).__TAURI_INTERNALS__ = {};
  invokeMock.mockReset();
  invokeMock.mockImplementation(async () => structuredClone(source));
  return runPreviewMechanics(model, mode as Json);
}

it("i67 u7f oracle dump", async () => {
  const inputs: Json[] = JSON.parse(readFileSync(process.env.I67_ORACLE_INPUTS!, "utf8"));
  const caseFile = JSON.parse(readFileSync(process.env.I67_ORACLE_CASES!, "utf8"));
  const rows: Record<string, Json> = {};
  for (const record of inputs) {
    const source = record.source as MechanicsResult, invocation = record.invocation as Json, requested = (record.requested as Json[]).map(r => r.ref_id) as string[];
    const row: Json = {};
    try {
      const v = await validateRetainedPrecision(structuredClone(source), invocation == null ? undefined : structuredClone(invocation));
      Object.assign(row, { reader: "pass", classes: digest(v.classifications), invocation_bound: v.invocation_bound, numerical_eligible: v.numerical_eligible, reader_standing: v.standing });
    } catch (e) { row.reader = e instanceof RetainedPrecisionError ? `${e.gate}:${e.code}` : `THROW:${String(e)}`; }
    row.reader_transport = await validateRetainedPrecisionTransport(structuredClone(source)).then(v => [v.invocation_bound, v.numerical_eligible, v.standing, v.classifications.length], (e: Json) => `${e.gate}:${e.code}`);
    row.transport = await sourceContractTransport(structuredClone(source)).then(r => `ok:${r}`, (e: Error) => `err:${e.message}`);
    // live: the input's own invocation, with a stand-in live capture.
    const own = structuredClone(source);
    await registerRetainedPrecision(own, invocation == null ? undefined : structuredClone(invocation), () => true).catch(() => undefined);
    row.live = standingOf(own, casesModel(requested));
    row.live_summary = classificationSummary(own, invocation?.request?.model ? casesModel(invocation.request.model.load_cases.map((c: Json) => c.id)) : null);
    if (record.binding) row.binding = digest((own.results ?? []).map((r: Json) => ruleBindingRefusal(own, r)));
    // ipc: TS's real delivery (carrier, declared and milestone inputs).
    const [kind, ...rest] = String(record.key).split("|");
    let spec: Json = null;
    if (kind === "carrier") spec = caseFile.cases.find((c: Json) => c.id === rest[0]);
    if (kind === "declared") spec = caseFile.declared_differences.find((e: Json) => e.id === rest[0]).forms.find((f: Json) => f.label === rest[1]);
    if (kind === "milestone") spec = { invocation: rest[1] === "without_invocation" ? null : "fixture", requested: rest[1] === "other_requested" ? record.requested : "invocation", edits: [] };
    if (spec) {
      const literal = spec.invocation !== null && spec.invocation !== "fixture";
      const fromFixture = spec.invocation === "fixture", captured = fromFixture && !Object.hasOwn(spec, "capture");
      const mode = invocation?.solver_mode ?? "sparse_interactive";
      const captureModel = captured ? structuredClone(invocation.request.model) as PreviewModel : null;
      const received = await deliver(structuredClone(source), captureModel, mode);
      if (literal) await registerRetainedPrecision(received, structuredClone(invocation)).catch(() => undefined);
      else if (fromFixture && !captured) await registerRetainedPrecision(received, structuredClone(invocation)).catch(() => undefined);
      let model: PreviewModel = fromFixture && spec.requested === "invocation" ? (captureModel ?? structuredClone(invocation.request.model)) : casesModel(requested);
      if (Object.hasOwn(spec, "current_model_edits")) {
        model = structuredClone(model);
        for (const edit of spec.current_model_edits) { let at = model as Json; for (const k of edit.path.slice(0, -1)) at = at[k]; at[edit.path[edit.path.length - 1]] = edit.value; }
      }
      row.ipc = standingOf(received, model);
      row.ipc_summary = classificationSummary(received, model);
    }
    rows[record.key] = row;
  }
  writeFileSync(process.env.I67_ORACLE_OUT!, JSON.stringify(rows));
  console.log(Object.keys(rows).length, "rows");
}, 600_000);
