/** DEL-16-06 AX-002, AC-010: the embedded agent's solve tool through the actual workspace
 * session's solve route (handleRunWithAgentReceipt), with the native job transport replaying
 * committed producer bytes and a scripted stand-in for the native agent host. A route check
 * with scripted inputs (unit transport replay); NOT a native witness, and no supplier or model
 * runs. All data are invented. The blocked case solves the invented preview model, whose legacy
 * pressure primitives the producer refuses (run:preview-linear-static-blocked, MODEL_INCOMPLETE,
 * no rows), with the native replies recorded for that result (as App.test.tsx's
 * MODEL_INCOMPLETE case reads it). The witness case solves the request of
 * `fixtures/results/invented/agent_solve_witness`, whose committed raw result is the current
 * producer's exact bytes (kept current by `live_host.rs`
 * `the_blocked_retained_evaluation_witness_is_current_and_binds_only_as_rejected`): a blocked
 * document whose passing case's evaluation is retained inspection-only, with its prior outcome
 * (DEL-04-07 *Blocked document*). The Solve command's gate (DEL-07-07 REQ-008) runs first on
 * every route; its model checks answer only where a test scripts them, and are otherwise
 * unavailable, so the solve proceeds as before the gate. */
import { act, cleanup, render, waitFor } from "@testing-library/react";
import { isDeepStrictEqual } from "node:util";
import { afterEach, describe, expect, it, vi } from "vitest";
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../../test/nativeIngress")).ingressDecodingInvoke(invokeMock, { nativeReopen: "actual" }) }));
import { createSolveJobTransport, nativeMechanicsReplayPair, solveBytes } from "../../test/nativeMechanicsReplay";
import refusedPreviewModel from "../../../../../fixtures/product_preview/invented_preview_model.json";
import refusedPreviewSparse from "../../../../../fixtures/product_preview/invented_mechanics_result_preview_physics_1_sparse.json";
import witnessRequest from "../../../../../fixtures/results/invented/agent_solve_witness/blocked_retained_evaluation.request.json";
import witnessRaw from "../../../../../fixtures/results/invented/agent_solve_witness/blocked_retained_evaluation-sparse_interactive.raw.json?raw";
import { presentedResultDiagnostics } from "../results/suppliedDiagnostics";
import { boundedFindings } from "../../services/embeddedAgentSession";
import { resultRowCount } from "../../services/nativeResult";
import type { PreviewModel } from "../../types";
import { getLocalStorageCapability } from "../../services/projectService";
import { loadDesignKnowledge, resetSessionResultCustodyForTests } from "../../services/previewService";
import * as modelChecks from "../diagnostics/modelChecks";
import { useWorkspaceSession, type WorkspaceSession } from "./workspaceSession";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any

afterEach(() => {
  cleanup();
  vi.restoreAllMocks();
  invokeMock.mockReset();
  resetSessionResultCustodyForTests();
  window.localStorage.clear();
  delete (window as Json).__TAURI_INTERNALS__;
});

const TOO_LARGE = { code: "PREVIEW-SOLVE-RESULT-TOO-LARGE", result_bytes: 3_950_000_000, limit_bytes: 67_108_864 };
let sequence = 0;

/** A model-check report as the native check returns it; its findings are the solve group's. */
const checkReport = (diagnostics: unknown[] = []) => ({ checked: true, diagnostics, diagnostics_not_shown: 0, slot_findings: [], slot_findings_not_shown: 0 });
const BLOCKING_FINDING = { id: "d:invented-gate", code: "INVENTED_INPUT_MISSING", severity: "blocking", message: "invented missing value", affected_refs: [] };

/** The session over the replayed solve of the committed pressure pair, with a scripted agent
 * host: `call(tool, args)` queues one tool call; `published` holds each outcome and proof the
 * webview publishes. `answer` decides the solve's terminal shape, and `check` the Solve gate's
 * model checks: unset, they are unavailable (the solve proceeds); `"held"`, each waits in
 * `checks` until the test answers it; otherwise each returns that report. */
async function session(answer: { mode: "solved" | "too_large" | "blocked" | "witness"; check?: "held" | Json; replayMode?: "sparse_interactive" | "dense_scrutiny" }) {
  const pair = nativeMechanicsReplayPair(answer.replayMode ?? "sparse_interactive", { profile: "pressure" });
  const { model, bytes } = answer.mode === "blocked"
    ? { model: structuredClone(refusedPreviewModel) as unknown as PreviewModel, bytes: solveBytes(structuredClone(refusedPreviewSparse)) }
    : answer.mode === "witness"
      ? { model: structuredClone(witnessRequest.model) as unknown as PreviewModel, bytes: witnessRaw }
      : pair;
  const checks: { resolve: (value: unknown) => void; reject: (error: unknown) => void }[] = [];
  const storage = await getLocalStorageCapability(), knowledge = await loadDesignKnowledge();
  const modes: unknown[] = [];
  const transport = createSolveJobTransport((request) => {
    modes.push(request.mode);
    if (!isDeepStrictEqual(request.model, model)) throw new Error("REPLAY_REQUEST_MISMATCH");
    return bytes;
  }, { jobIdPrefix: `unit-agent-solve:${++sequence}` });
  const notices: string[] = [], published: { outcome: Json; proof: Json }[] = [];
  const calls = new Map<string, Json>();
  let basis: Json = null, reference = 0;
  invokeMock.mockImplementation(async (command: string, args: Json, options?: unknown) => {
    switch (command) {
      case "get_local_storage_capability": return storage;
      case "load_design_knowledge": return knowledge;
      case "sync_native_shell_state": return null;
      case "check_model_library_refs": return [];
      case "load_preview_model": if (args === undefined) return structuredClone(model); break;
      case "find_local_project": return null;
      case "list_model_states": return [];
      case "list_analysis_runs": return [];
      case "check_preview_model":
        if (answer.check === undefined) break;
        if (answer.check === "held") return new Promise((resolve, reject) => { checks.push({ resolve, reject }); });
        return structuredClone(answer.check);
      case "cancel_model_checks": return null;
      case "embedded_agent_host": {
        const request = args.request as Json;
        switch (request.kind) {
          case "register": basis = request.basis; return { workspaceHandle: "workspace-invented", applicationGeneration: 1, ownerCapability: "owner-invented" };
          case "start": return { status: "starting" };
          case "basis": basis = request.basis; return {};
          case "poll": return { status: "ready", notices: notices.splice(0), messages: [], closed: false, turnActive: true, turnStatus: "inProgress" };
          case "claim": return { raw: JSON.stringify(calls.get(request.requestRef)) };
          case "publish_actual": published.push({ outcome: JSON.parse(request.outcome), proof: request.proof }); return { recorded: true };
          case "complete": return { delivery: "queued" };
          default: return {};
        }
      }
      case "poll_preview_mechanics_job": {
        const status = await transport.invoke(command, args, options);
        if (answer.mode !== "too_large") return status;
        return { ...status, state: "refused", result_available: false, result_bytes: null, resource_refusal: null, result_too_large: TOO_LARGE,
          error_message: `PREVIEW-SOLVE-RESULT-TOO-LARGE: ${status.job_id}'s result is too large` };
      }
    }
    return transport.invoke(command, args, options);
  });
  (window as Json).__TAURI_INTERNALS__ = {};
  // The checks run by themselves when the model opens (issue 530); that check is held, never
  // completing, so the Solve command's gate finds no current run and runs its own, as these tests
  // follow it.
  const opening = vi.spyOn(modelChecks, "runModelChecks").mockImplementation(() => new Promise(() => {}));
  let current!: WorkspaceSession;
  function Harness() { current = useWorkspaceSession(); return null; }
  render(<Harness />);
  await waitFor(() => expect(current.model.modelHash).not.toBeNull());
  await waitFor(() => expect(opening).toHaveBeenCalled());
  opening.mockRestore();
  const startAgent = () => act(async () => { await current.agent.start("r".repeat(48)); });
  const call = async (tool: string, extra: Json = {}) => {
    const requestRef = `invented-session:${++reference}`;
    calls.set(requestRef, { requestRef, threadId: "thread-1", turnId: "turn-1", callId: `call-${reference}`, tool,
      basis: { workspaceHandle: "workspace-invented", applicationGeneration: 1, modelRevision: basis.modelRevision, modelHash: basis.modelHash, stateId: null, runId: null },
      arguments: { workspaceHandle: "workspace-invented", claimedModelHash: basis.modelHash, ...extra } });
    notices.push(requestRef);
    await waitFor(() => expect(published.length).toBe(reference), { timeout: 20000 });
    return published[reference - 1];
  };
  return { state: () => current, call, modes, checks, startAgent, close: () => act(async () => { await current.agent.close(); }) };
}

/** What the desktop publishes and shows for a solve: the result, its presented findings and the job. */
const shown = (s: WorkspaceSession) => ({
  runId: s.results.result?.run_id, status: s.results.result?.status, rows: s.results.result ? resultRowCount(s.results.result) : null,
  diagnostics: presentedResultDiagnostics(s.results.result),
  job: { state: s.results.solveJob.state, events: s.results.solveJob.events.map(({ event_id, state, result_available, diagnostic_count, result_row_count, analysis_status }) => ({ event_id, state, result_available, diagnostic_count, result_row_count, analysis_status })) }
});

describe("the agent's solve through the workspace's solve route", () => {
  it("reports a solved run as completed, with its run and no findings, and leaves the person's selection alone", async () => {
    const { state, call, close, startAgent } = await session({ mode: "solved" });
    await startAgent();
    const { outcome, proof } = await call("swbpipe_solve");
    expect(outcome.status).toBe("completed");
    expect(outcome.data).toMatchObject({ runHandle: expect.any(String), runId: expect.any(String), solverMode: "sparse_interactive", diagnostics: [], diagnosticsNotListed: 0, failure: null });
    expect(proof).toMatchObject({ kind: "run", status: "completed", nativeJobId: expect.any(String) });
    expect(state().results.solverMode).toBe("sparse_interactive");
    await close();
  }, 60000);

  it("reports a result the solver blocked as rejected with its findings and no run, publishing what a person's Run publishes", async () => {
    // The person's Run of the same blocked model: what the desktop publishes and shows.
    const person = await session({ mode: "blocked" });
    await act(async () => { await person.state().results.handleRun(); });
    await waitFor(() => expect(person.state().results.solveJob.state).toBe("completed"));
    const byPerson = shown(person.state());
    expect(byPerson.runId).toBe("run:preview-linear-static-blocked");
    expect(byPerson.status?.mechanics).toBe("MODEL_INCOMPLETE");
    expect(byPerson.rows).toBe(0);
    expect(byPerson.diagnostics.length).toBeGreaterThan(0);
    cleanup();
    resetSessionResultCustodyForTests();

    const agent = await session({ mode: "blocked" });
    await agent.startAgent();
    const { outcome, proof } = await agent.call("swbpipe_solve");
    expect(outcome.status).toBe("rejected");
    expect(outcome.message).not.toMatch(/completed/i);
    expect(outcome.data).toMatchObject({ runHandle: null, runId: null, health: "MODEL_INCOMPLETE", failure: null });
    const expected = boundedFindings(byPerson.diagnostics, 0);
    expect(outcome.data.diagnostics).toEqual(expected.diagnostics);
    expect(outcome.data.diagnosticsNotListed).toBe(expected.notListed);
    expect(outcome.data.diagnostics.map((d: Json) => d.code)).toContain("PRESSURE_MODEL_REAUTHOR_REQUIRED");
    expect(proof).toMatchObject({ kind: "run", status: "rejected", handle: null, runId: null, nativeJobId: expect.any(String) });
    // No run the agent can read: a result read names no retained run.
    const read = await agent.call("swbpipe_read_results", { runHandle: "r".repeat(48), page: { cursor: null, limit: 5 } });
    expect(read.outcome.status).toBe("unavailable");
    // The desktop published exactly what the person's Run published.
    await waitFor(() => expect(agent.state().results.solveJob.state).toBe("completed"));
    expect(shown(agent.state())).toEqual(byPerson);
    await agent.close();
  }, 60000);

  it("never reads a later solve's rows through an earlier solve's handle, though both carry the producer's one run id", async () => {
    const { call, close, startAgent } = await session({ mode: "solved" });
    await startAgent();
    const first = await call("swbpipe_solve");
    const second = await call("swbpipe_solve");
    expect(first.outcome.status).toBe("completed");
    expect(second.outcome.status).toBe("completed");
    // The producer gives every solve the same run id: it identifies neither.
    expect(second.outcome.data.runId).toBe(first.outcome.data.runId);
    expect(second.proof.nativeJobId).not.toBe(first.proof.nativeJobId);
    const queries = () => invokeMock.mock.calls.filter(([command]) => command === "query_preview_mechanics_job_rows").length;
    const before = queries();
    const read = await call("swbpipe_read_results", { runHandle: first.outcome.data.runHandle, page: { cursor: null, limit: 5 } });
    expect(read.outcome.status).toBe("unavailable");
    expect(read.outcome.data).toBeNull();
    // Refused before any native read: the later job's rows were never queried for it.
    expect(queries()).toBe(before);
    await close();
  }, 60000);

  it("reports a job refused for its result's size as failed, with the job's message, and never retries it", async () => {
    const { state, call, modes, close, startAgent } = await session({ mode: "too_large" });
    await startAgent();
    const { outcome, proof } = await call("swbpipe_solve");
    expect(outcome.status).toBe("failed");
    expect(outcome.data).toMatchObject({ runHandle: null, runId: null, diagnostics: [], failure: { kind: "result_too_large" } });
    expect(outcome.data.failure.message).toMatch(/^The solve finished, but its result is 3\.68 GiB/);
    expect(proof).toMatchObject({ status: "failed", nativeJobId: null });
    expect(modes).toHaveLength(1);
    expect(state().results.solveJob.state).toBe("refused");
    await close();
  }, 60000);

  it("runs the mode the agent names and sets the Solve panel's selection to it", async () => {
    const { state, call, modes, close, startAgent } = await session({ mode: "solved", replayMode: "dense_scrutiny" });
    await startAgent();
    const { outcome } = await call("swbpipe_solve", { solverMode: "dense_scrutiny" });
    expect(modes).toEqual(["dense_scrutiny"]);
    expect(state().results.solverMode).toBe("dense_scrutiny");
    // Replay the dense producer's bytes for the dense request and require publication;
    // an absent reader reply or any other failure cannot satisfy this mode control.
    expect(outcome.status).toBe("completed");
    expect(outcome.data.runHandle).toEqual(expect.any(String));
    expect(outcome.data.solverMode).toBe("dense_scrutiny");
    await close();
  }, 60000);
  it("reports the producer's blocked document with a retained inspection-only evaluation as rejected, with the presented findings and no run", async () => {
    // The person's Run of the witness request: what the desktop publishes and shows.
    const person = await session({ mode: "witness" });
    await act(async () => { await person.state().results.handleRun(); });
    await waitFor(() => expect(person.state().results.solveJob.state).toBe("completed"));
    const byPerson = shown(person.state());
    expect(byPerson.runId).toBe("run:preview-linear-static-blocked");
    expect(byPerson.status?.mechanics).toBe("MODEL_INCOMPLETE");
    expect(byPerson.rows).toBe(0);
    const codes = byPerson.diagnostics.map((d) => d.code);
    expect(codes).toContain("NUMERICAL_INTEGRITY_INSPECTION_ONLY");
    expect(codes).toContain("NUMERICAL_INTEGRITY_UNRESOLVED");
    expect(codes).not.toContain("NUMERICAL_INTEGRITY_CHECKS_PASSED");
    cleanup();
    resetSessionResultCustodyForTests();

    const agent = await session({ mode: "witness" });
    await agent.startAgent();
    const { outcome, proof } = await agent.call("swbpipe_solve");
    expect(outcome.status).toBe("rejected");
    expect(outcome.message).toMatch(/^Not solved: the model was refused/);
    expect(outcome.data).toMatchObject({ runHandle: null, runId: null, health: "MODEL_INCOMPLETE", solverMode: "sparse_interactive", failure: null });
    // The findings are those the Diagnostics panel presents, bounded; none is invented or dropped.
    const expected = boundedFindings(byPerson.diagnostics, 0);
    expect(outcome.data.diagnostics).toEqual(expected.diagnostics);
    expect(outcome.data.diagnosticsNotListed).toBe(expected.notListed);
    const inspection = outcome.data.diagnostics.filter((d: Json) => d.code === "NUMERICAL_INTEGRITY_INSPECTION_ONLY");
    expect(inspection).toHaveLength(1);
    expect(inspection[0].message).toMatch(/^Inspection only \(prior_outcome=NUMERICAL_INTEGRITY_CHECKS_PASSED\): this document is blocked \(MODEL_INCOMPLETE\);/);
    expect(outcome.data.diagnostics.map((d: Json) => d.code)).toContain("NUMERICAL_INTEGRITY_UNRESOLVED");
    // The proof names the job whose held bytes the native host reads as not solved.
    expect(proof).toMatchObject({ kind: "run", status: "rejected", handle: null, runId: null, nativeJobId: expect.any(String) });
    const read = await agent.call("swbpipe_read_results", { runHandle: "r".repeat(48), page: { cursor: null, limit: 5 } });
    expect(read.outcome.status).toBe("unavailable");
    await waitFor(() => expect(agent.state().results.solveJob.state).toBe("completed"));
    expect(shown(agent.state())).toEqual(byPerson);
    await agent.close();
  }, 60000);
});

describe("the agent's solve through the Solve command's gate", () => {
  it("reports a withheld Solve as rejected with the gate's findings and reason and no job; nothing starts, as for a person's Run", async () => {
    const agent = await session({ mode: "solved", check: checkReport([BLOCKING_FINDING]) });
    await agent.startAgent();
    const { outcome, proof } = await agent.call("swbpipe_solve");
    expect(outcome.status).toBe("rejected");
    expect(outcome.data).toMatchObject({ runHandle: null, runId: null, health: "NOT_SOLVED", solverMode: "sparse_interactive", failure: null, diagnosticsNotListed: 0 });
    expect(outcome.data.diagnostics.map((d: Json) => d.code)).toEqual(["INVENTED_INPUT_MISSING"]);
    expect(outcome.message).toContain("Solve is withheld: 1 solve-blocking finding stands for this model");
    expect(proof).toMatchObject({ kind: "run", status: "rejected", handle: null, runId: null, nativeJobId: null });
    expect(agent.modes).toEqual([]);
    const s = agent.state();
    expect(s.results.solveGate.kind).toBe("withheld");
    expect(s.results.result).toBeNull();
    expect(s.results.running).toBe(false);
    // The person's Run of the same model: the same gate, nothing started.
    let request: Json;
    await act(async () => { request = await agent.state().results.handleSolveRequest(); });
    expect(request).toMatchObject({ gate: { kind: "withheld" }, receipt: undefined, solve: { kind: "rejected", jobId: null, modelHash: null } });
    expect(agent.modes).toEqual([]);
    await agent.close();
  }, 60000);

  it("reports a Solve cancelled during its model check as failed (cancelled) with no job; a person's cancelled Run is unchanged", async () => {
    const agent = await session({ mode: "solved", check: "held" });
    await agent.startAgent();
    const pending = agent.call("swbpipe_solve");
    await waitFor(() => expect(agent.checks).toHaveLength(1));
    expect(agent.state().results.running).toBe(true);
    act(() => { agent.state().results.handleCancelRun(); });
    // The native cancel ends the check, which then replies; its reading changes nothing.
    await act(async () => { agent.checks[0].resolve(checkReport()); });
    const { outcome, proof } = await pending;
    expect(outcome.status).toBe("failed");
    expect(outcome.data).toMatchObject({ runHandle: null, runId: null, diagnostics: [], failure: { kind: "cancelled" } });
    expect(outcome.data.failure.message).toMatch(/cancelled while the model checks it runs first were running; nothing was started/);
    expect(proof).toMatchObject({ status: "failed", nativeJobId: null });
    expect(agent.modes).toEqual([]);
    expect(agent.state().results.running).toBe(false);
    // The person's Run, cancelled the same way.
    let request: Promise<Json> = Promise.resolve(null);
    act(() => { request = agent.state().results.handleSolveRequest(); });
    await waitFor(() => expect(agent.checks).toHaveLength(2));
    act(() => { agent.state().results.handleCancelRun(); });
    let person: Json;
    await act(async () => { agent.checks[1].resolve(checkReport()); person = await request; });
    expect(person).toMatchObject({ gate: { kind: "cancelled" }, receipt: undefined, solve: { kind: "failed", failure: "cancelled" } });
    expect(agent.modes).toEqual([]);
    await agent.close();
  }, 60000);

  it("reports a Solve whose model check a later check retired as failed (superseded) with no job; nothing starts", async () => {
    const agent = await session({ mode: "solved", check: "held" });
    await agent.startAgent();
    const pending = agent.call("swbpipe_solve");
    await waitFor(() => expect(agent.checks).toHaveLength(1));
    // The person runs the model checks while the Solve's check is pending: theirs is newer.
    let checking: Promise<void> = Promise.resolve();
    act(() => { checking = agent.state().chrome.handleRunModelChecks(); });
    await waitFor(() => expect(agent.checks).toHaveLength(2));
    await act(async () => { agent.checks[0].resolve(checkReport()); });
    const { outcome, proof } = await pending;
    expect(outcome.status).toBe("failed");
    expect(outcome.data).toMatchObject({ runHandle: null, runId: null, diagnostics: [], failure: { kind: "superseded" } });
    expect(outcome.data.failure.message).toMatch(/ended without a reading \(a later model check replaced it\); nothing was started/);
    expect(proof).toMatchObject({ status: "failed", nativeJobId: null });
    expect(agent.modes).toEqual([]);
    await act(async () => { agent.checks[1].resolve(checkReport()); await checking; });
    expect(agent.state().results.running).toBe(false);
    await agent.close();
  }, 60000);
});
