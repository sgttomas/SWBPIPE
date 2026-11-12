/** DEL-16-06 AX-002, AC-010: the embedded agent's solve tool through the actual workspace
 * session's solve route (handleRunWithAgentReceipt), with the native job transport replaying
 * committed producer bytes and a scripted stand-in for the native agent host. A route check
 * with scripted inputs (unit transport replay); NOT a native witness, and no supplier or model
 * runs. All data are invented. The blocked case solves the invented preview model, whose legacy
 * pressure primitives the producer refuses (run:preview-linear-static-blocked, MODEL_INCOMPLETE,
 * no rows), with the native replies recorded for that result (as App.test.tsx's
 * MODEL_INCOMPLETE case reads it). */
import { act, cleanup, render, waitFor } from "@testing-library/react";
import { isDeepStrictEqual } from "node:util";
import { afterEach, describe, expect, it, vi } from "vitest";
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../../test/nativeIngress")).ingressDecodingInvoke(invokeMock, { nativeReopen: "actual" }) }));
import { createSolveJobTransport, nativeMechanicsReplayPair, solveBytes } from "../../test/nativeMechanicsReplay";
import refusedPreviewModel from "../../../../../fixtures/product_preview/invented_preview_model.json";
import refusedPreviewSparse from "../../../../../fixtures/product_preview/invented_mechanics_result_preview_physics_1_sparse.json";
import { presentedResultDiagnostics } from "../results/suppliedDiagnostics";
import { boundedFindings } from "../../services/embeddedAgentSession";
import { resultRowCount } from "../../services/nativeResult";
import type { PreviewModel } from "../../types";
import { getLocalStorageCapability } from "../../services/projectService";
import { loadDesignKnowledge, resetSessionResultCustodyForTests } from "../../services/previewService";
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

/** The session over the replayed solve of the committed pressure pair, with a scripted agent
 * host: `call(tool, args)` queues one tool call; `published` holds each outcome and proof the
 * webview publishes. `answer` decides the solve's terminal shape. */
async function session(answer: { mode: "solved" | "too_large" | "blocked" }) {
  const pair = nativeMechanicsReplayPair("sparse_interactive", { profile: "pressure" });
  const { model, bytes } = answer.mode === "blocked"
    ? { model: structuredClone(refusedPreviewModel) as unknown as PreviewModel, bytes: solveBytes(structuredClone(refusedPreviewSparse)) }
    : pair;
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
  let current!: WorkspaceSession;
  function Harness() { current = useWorkspaceSession(); return null; }
  render(<Harness />);
  await waitFor(() => expect(current.model.modelHash).not.toBeNull());
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
  return { state: () => current, call, modes, startAgent, close: () => act(async () => { await current.agent.close(); }) };
}

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
    const shown = (s: WorkspaceSession) => ({
      runId: s.results.result?.run_id, status: s.results.result?.status, rows: s.results.result ? resultRowCount(s.results.result) : null,
      diagnostics: presentedResultDiagnostics(s.results.result),
      job: { state: s.results.solveJob.state, events: s.results.solveJob.events.map(({ event_id, state, result_available, diagnostic_count, result_row_count, analysis_status }) => ({ event_id, state, result_available, diagnostic_count, result_row_count, analysis_status })) }
    });
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
    const { state, call, modes, close, startAgent } = await session({ mode: "solved" });
    await startAgent();
    const { outcome } = await call("swbpipe_solve", { solverMode: "dense_scrutiny" });
    expect(modes).toEqual(["dense_scrutiny"]);
    expect(state().results.solverMode).toBe("dense_scrutiny");
    // The replay holds the sparse producer's bytes, which are not this dense invocation's: the
    // route refuses to publish them, and the agent is told the solve failed, never completed.
    expect(outcome.status).toBe("failed");
    expect(outcome.data.solverMode).toBe("dense_scrutiny");
    await close();
  }, 60000);
});
