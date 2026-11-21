// Release exit criterion 18 (LIM-026), fail closed: a load case's "Support states: not confirmed"
// line is hidden only where the desktop has positively established that there is nothing to confirm:
// the model the result is read with is known and has no nonlinear support, or the reader's record
// gives the case an outcome that takes no line. Everywhere else every load case the indicator lists
// says its confirmation record could not be read: the confirmation read refused or failed, not made,
// the record unreadable under today's reader, or no model available. And the confirmation read is
// optional: a saved project always opens; what cannot be read is said, never the cause of a refused
// open. Paths: the standalone (whole-value) historical panel, a recorded run, and the native reopen
// (`restoredNativeResult`) with its read refused at the native command, with no model, with a model
// known to have no nonlinear support, and readable. Unit transport replay (`nativeReplayStore.ts`);
// NOT native UI qualification. Committed fixtures only.
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { cleanup, render, screen, waitFor } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
const invokeMock = vi.hoisted(() => vi.fn());
/** The native result commands the restored handle answered, in order, and the refusal to inject. */
const replay = vi.hoisted(() => ({ calls: [] as { command: string; args: Json }[], refuseConfirmations: false }));
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../../test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));
vi.mock("../../test/nativeReplayStore", async (importOriginal) => {
  const actual = await importOriginal<typeof import("../../test/nativeReplayStore")>();
  return {
    ...actual,
    nativeReplayReply: (command: string, args: Json, ...rest: unknown[]) => {
      replay.calls.push({ command, args: structuredClone(args) });
      // The native summary's capped lease refusing the model it was sent (`result_summary.rs`, "Capped
      // as the args lease was admitted"), as the command returns it.
      if (replay.refuseConfirmations && command === "read_preview_mechanics_job_summary" && args?.modelUse === "confirmations") {
        throw "INGRESS-ALLOCATION-REFUSED: read_preview_mechanics_job_summary: the interactive ingress cap does not admit this input";
      }
      return (actual.nativeReplayReply as (...a: unknown[]) => unknown)(command, args, ...rest);
    }
  };
});

import type { LocalProjectEnvelope, MechanicsResult, PreviewModel } from "../../types";
import { computeModelHash, computeProjectEnvelopeHash } from "../../services/hashService";
import { openLocalProjectNative, restoredNativeResult } from "../../services/nativeReopen";
import { isNativeJobLive, isNativeResult, nativeCustody, resetNativeResultsForTests } from "../../services/nativeResult";
import { resetSessionResultCustodyForTests } from "../../services/previewService";
import { resetNativeReopenEmulation } from "../../test/nativeReopenEmulation";
import { buildHistoricalRunContext, buildNativeHistoricalRunContext, buildRecordedRunContext, HistoricalRunPanel, type HistoricalRunContext } from "./HistoricalRunContext";
import { standaloneNumericalHealth } from "./ResultsPanel";
import { reportStanding } from "./resultStanding";
import { CONFIRMATION_RECORD_UNREAD_STATEMENT, CONFIRMATION_STATEMENTS, SUPPORT_STATES_LEAD, SUPPORT_STATES_UNREAD_LINE, supportStatesText } from "./integrityStatus";

const root = resolve(__dirname, "../../../../../");
const json = <T,>(path: string) => JSON.parse(readFileSync(resolve(root, path), "utf8")) as T;
const DEMO_MODEL = "fixtures/product_preview/invented_demo_model.json";
const DEMO_RESULT = "fixtures/product_preview/invented_demo_result_preview_physics_1_sparse.json";
/** The invented demo's result at main 6c3cb46d3, verbatim: recorded before RT-A4 under preview-physics-2. */
const DEMO_PRE_RT_A4 = "fixtures/persistence/recorded_pre_rt_a4_pf/preview_physics_2/invented_demo_result_preview_physics_1_sparse.json";
const PRE_RT_A4_TABLE = { id: "openpipestress.result_semantics/0.3.0/preview-physics-2", sha256: "1ccc9561cb4ce59d047f3dab7927a927c44a707efb3acb90e7b07a21b3bad0a2" };
const LINEAR_RESULT = "fixtures/results/preview_physics_connected_sparse.json";
const LINEAR_MODEL = "fixtures/model_operations/precision_connected_ui_model.json";
const UNREAD = `${SUPPORT_STATES_LEAD}. ${CONFIRMATION_RECORD_UNREAD_STATEMENT}`;
const SUMMARY = "read_preview_mechanics_job_summary";

let envelope: Json = null;
beforeEach(() => {
  resetSessionResultCustodyForTests();
  resetNativeResultsForTests();
  resetNativeReopenEmulation();
  replay.calls = [];
  replay.refuseConfirmations = false;
  invokeMock.mockImplementation(async (command: string) => {
    if (command === "open_local_project") return structuredClone(envelope);
    throw new Error(`UNEXPECTED_COMMAND: ${command}`);
  });
});
afterEach(() => {
  cleanup();
  invokeMock.mockReset();
  delete (window as Json).__TAURI_INTERNALS__;
});

async function storedEnvelope(model: PreviewModel, source: MechanicsResult): Promise<Json> {
  const payload = { model, mechanics_result: source, analysis_run: null, model_hash: await computeModelHash(model), editor_intents: [], proposal: null, selected_review_target: null };
  return { ...payload, project_envelope_hash: await computeProjectEnvelopeHash(payload as never), summary: { project_id: (model as Json).project?.id ?? "", message: "Opened the invented stored project." } };
}
/** The saved project opened natively, as the session opens it (`openLocalProjectNative`, then the context). */
async function openedNatively(model: PreviewModel, source: MechanicsResult): Promise<HistoricalRunContext> {
  envelope = await storedEnvelope(model, source);
  (window as Json).__TAURI_INTERNALS__ = {};
  return (await buildNativeHistoricalRunContext((await openLocalProjectNative(null))!))!;
}
const summaryReads = () => replay.calls.filter((call) => call.command === SUMMARY).map((call) => ({ model: call.args.modelJson === "null" ? "none" : "the saved model", modelUse: call.args.modelUse ?? null }));
/** Each load case's support-state line in the rendered indicator, by case id (null where it shows none). */
function linesShown(caseIds: readonly string[]): Record<string, string | null> {
  return Object.fromEntries(caseIds.map((id) => [id, screen.queryByTestId(`numerical-health-case-support-states-${id}`)?.textContent ?? null]));
}
const caseIdsOf = (result: MechanicsResult) => (result.numerical_quality!.cases as Json[]).map((c) => c.basis_ref.ref_id as string);

describe("the could-not-be-read statement", () => {
  it("is the desktop's own, apart from the reader's outcomes", () => {
    expect(CONFIRMATION_RECORD_UNREAD_STATEMENT).toBe("Not confirmed: this result's support-state confirmation record could not be read here, so what the solver reported for this load case is not shown.");
    expect(Object.values(CONFIRMATION_STATEMENTS)).not.toContain(CONFIRMATION_RECORD_UNREAD_STATEMENT);
    expect(SUPPORT_STATES_UNREAD_LINE).toEqual({ text: UNREAD, detail: null, detailNote: null });
  });
});

describe("the standalone historical panel", () => {
  it("a run recorded before RT-A4 with nonlinear supports: today's reader refuses its record, so each case says so", () => {
    const source = json<MechanicsResult>(DEMO_PRE_RT_A4), model = json<PreviewModel>(DEMO_MODEL);
    let readable = false;
    try { const reading = reportStanding(source, model); readable = reading.readability.readable && reading.readability.solved; } catch { readable = false; }
    expect(readable, "today's support-state reader refuses the recorded run").toBe(false);
    const health = standaloneNumericalHealth(source, PRE_RT_A4_TABLE, model)!;
    expect(health.cases.map((c) => c.caseId)).toEqual(["load:L-100", "load:L-200"]);
    for (const c of health.cases) expect(c.supportStates, c.caseId).toEqual(SUPPORT_STATES_UNREAD_LINE);
  });

  it("no model: the desktop cannot tell whether the model has a nonlinear support, so each case says so", () => {
    for (const path of [DEMO_RESULT, LINEAR_RESULT]) {
      const health = standaloneNumericalHealth(json<MechanicsResult>(path), null, null)!;
      expect(health.cases.length, path).toBeGreaterThan(0);
      for (const c of health.cases) expect(c.supportStates, `${path} ${c.caseId}`).toEqual(SUPPORT_STATES_UNREAD_LINE);
    }
  });

  it("a model known to have no nonlinear support: no line", () => {
    const health = standaloneNumericalHealth(json<MechanicsResult>(LINEAR_RESULT), null, json<PreviewModel>(LINEAR_MODEL))!;
    expect(health.cases.length).toBeGreaterThan(0);
    for (const c of health.cases) expect(c.supportStates, c.caseId).toBeNull();
  });

  it("a readable current result: each case shows the reader's statement for its outcome, as before", async () => {
    const source = json<MechanicsResult>(DEMO_RESULT), model = json<PreviewModel>(DEMO_MODEL);
    const reading = reportStanding(source, model);
    const outcomes = Object.fromEntries((reading.numerical_standing as Json).cases.map((c: Json) => [c.case_id, c.confirmation.outcome]));
    expect(Object.values(outcomes)).toEqual(["solver_reported_established", "solver_reported_established"]);
    const context = (await buildHistoricalRunContext(await storedEnvelope(model, source) as LocalProjectEnvelope))!;
    render(<HistoricalRunPanel context={context} />);
    await waitFor(() => expect(screen.getAllByTestId(/^numerical-health-case-support-states-/).length).toBe(2));
    expect(linesShown(caseIdsOf(source))).toEqual(Object.fromEntries(Object.entries(outcomes).map(([id, outcome]) => [id, supportStatesText(outcome)])));
  });

  it("a recorded run whose manifest gives no valid model: each case says its record could not be read", async () => {
    const source = json<MechanicsResult>(DEMO_RESULT);
    const read = {
      run: { project_id: source.model_ref, recording_id: "rec-1", state_id: "state-1", producer_run_id: source.run_id },
      state: { state_id: "state-1", project_id: source.model_ref, model_hash: "0".repeat(64) },
      result_json: JSON.stringify(source), input_manifest_canonical: "{not a manifest", record: null
    };
    const context = (await buildRecordedRunContext(read as never))!;
    expect(context.readModel ?? null).toBeNull();
    render(<HistoricalRunPanel context={context} />);
    await waitFor(() => expect(screen.getAllByTestId(/^numerical-health-case-support-states-/).length).toBe(2));
    for (const line of Object.values(linesShown(caseIdsOf(source)))) expect(line).toBe(UNREAD);
  });
});

describe("the native reopen", () => {
  it("the confirmation read refused: the project opens, its tables show, each case says so, the handle stays live", async () => {
    replay.refuseConfirmations = true;
    const source = json<MechanicsResult>(DEMO_RESULT);
    const context = await openedNatively(json<PreviewModel>(DEMO_MODEL), source);
    const result = context.mechanicsResult!;
    expect(isNativeResult(result)).toBe(true);
    // The confirmation read, then the plain read exactly as before round 1's repair.
    expect(summaryReads()).toEqual([{ model: "the saved model", modelUse: "confirmations" }, { model: "none", modelUse: null }]);
    const custody = nativeCustody(result as never);
    expect(custody.supportConfirmations).toBe("not_read");
    expect(isNativeJobLive(custody.jobId)).toBe(true);
    render(<HistoricalRunPanel context={context} />);
    const panel = screen.getByTestId("historical-run-context");
    await waitFor(() => expect(panel.querySelectorAll("tr[data-testid^='result-row-']").length).toBeGreaterThan(0));
    await waitFor(() => expect(screen.getAllByTestId(/^numerical-health-case-support-states-/).length).toBe(2));
    for (const line of Object.values(linesShown(caseIdsOf(source)))) expect(line).toBe(UNREAD);
    // Nothing was disposed: the handle still answers.
    expect(replay.calls.some((call) => call.command === "dispose_preview_mechanics_result")).toBe(false);
    expect(isNativeJobLive(custody.jobId)).toBe(true);
  });

  it("no model to read with: one plain read, and each case says its record could not be read", async () => {
    const context = await openedNatively(json<PreviewModel>(DEMO_MODEL), json<MechanicsResult>(DEMO_RESULT));
    replay.calls = [];
    resetNativeResultsForTests();
    const result = await restoredNativeResult(context.restored!, null);
    expect(summaryReads()).toEqual([{ model: "none", modelUse: null }]);
    expect(nativeCustody(result as never).supportConfirmations).toBe("not_read");
  });

  it("a saved model known to have no nonlinear support: one plain read and no line", async () => {
    const source = json<MechanicsResult>(LINEAR_RESULT);
    const context = await openedNatively(json<PreviewModel>(LINEAR_MODEL), source);
    expect(summaryReads()).toEqual([{ model: "none", modelUse: null }]);
    expect(nativeCustody(context.mechanicsResult as never).supportConfirmations).toBe("no_nonlinear_support");
    render(<HistoricalRunPanel context={context} />);
    await waitFor(() => expect(screen.getByTestId("numerical-health-detail")).toBeTruthy());
    expect(screen.queryAllByTestId(/^numerical-health-case-support-states-/)).toEqual([]);
  });

  it("the confirmation read answered: each case shows the reader's statement, as before", async () => {
    const source = json<MechanicsResult>(DEMO_RESULT), model = json<PreviewModel>(DEMO_MODEL);
    const context = await openedNatively(model, source);
    expect(summaryReads()).toEqual([{ model: "the saved model", modelUse: "confirmations" }]);
    expect(nativeCustody(context.mechanicsResult as never).supportConfirmations).toBe("read");
    render(<HistoricalRunPanel context={context} />);
    await waitFor(() => expect(screen.getAllByTestId(/^numerical-health-case-support-states-/).length).toBe(2));
    const outcomes = (reportStanding(source, model).numerical_standing as Json).cases.map((c: Json) => [c.case_id, supportStatesText(c.confirmation.outcome)]);
    expect(linesShown(caseIdsOf(source))).toEqual(Object.fromEntries(outcomes));
  });
});
