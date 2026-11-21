/** DEL-16-03, Commit, with a fresh solved result held: a commit clears the result, its analysis run, its input
 * manifest and its rule-check aggregate, and reports that it did. The solve is a unit transport replay of committed
 * producer bytes through mocked IPC (as agentSolve.session.test.tsx); the operation engine's batch commands are
 * answered by the operation applier crate compiled to wasm, with the arguments the session sent. All data are
 * invented; NOT a native UI qualification witness. */
import { act, cleanup, render, waitFor } from "@testing-library/react";
import { isDeepStrictEqual } from "node:util";
import { afterEach, expect, it, vi } from "vitest";
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../../test/nativeIngress")).ingressDecodingInvoke(invokeMock, { nativeReopen: "actual" }) }));
import { createSolveJobTransport, nativeMechanicsReplayPair } from "../../test/nativeMechanicsReplay";
import { getLocalStorageCapability } from "../../services/projectService";
import { loadDesignKnowledge, resetSessionResultCustodyForTests } from "../../services/previewService";
import { loadWasmEngine } from "../../services/wasmEngine/loadWasmEngine";
import { buildGridOperationIntent, type GridColumn } from "./table/modelTableAdapter";
import type { EditorBuilder } from "../../services/operationAuthorship";
import { useWorkspaceSession } from "./workspaceSession";
import { RuleRevisionGenerationGate } from "./solveGates";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
type Session = ReturnType<typeof useWorkspaceSession>;

afterEach(() => {
  cleanup();
  vi.restoreAllMocks();
  invokeMock.mockReset();
  resetSessionResultCustodyForTests();
  window.localStorage.clear();
  delete (window as Json).__TAURI_INTERNALS__;
});

type CoordinateInput = { nodeId: string; value: string };
/** The grid cell's own builder for a node's z coordinate, on typed values. */
const coordinate: EditorBuilder<CoordinateInput> = {
  name: "grid-cell",
  role: "gui_editor",
  input: { members: ["nodeId", "value"], check: (input) => typeof input.nodeId === "string" && typeof input.value === "string" ? null : "The grid cell takes a node and a value." },
  build: (model, input, id) => {
    const target = model.nodes.find((node) => node.id === input.nodeId)!;
    const column: GridColumn = { key: "z", label: "Z", fieldPath: "position.z", objectType: "Node", changeKind: "set_field", dimension: "length",
      sourceNote: "entered coordinate", unit: () => model.project.units.length, value: () => String(target.position.z) };
    return { intent: buildGridOperationIntent({ column, model, row: { id: target.id, label: target.label, type: "node", raw: target, searchText: "" }, sequence: id.sequence, value: input.value, interaction: "cell" }) };
  }
};

async function solvedSession() {
  const pair = nativeMechanicsReplayPair("sparse_interactive", { profile: "pressure" });
  const storage = await getLocalStorageCapability(), knowledge = await loadDesignKnowledge();
  const engine = loadWasmEngine();
  const transport = createSolveJobTransport((request) => {
    if (!isDeepStrictEqual(request.model, pair.model)) throw new Error("REPLAY_REQUEST_MISMATCH");
    return pair.bytes;
  }); // each session its own job ids (the default prefix is unique per transport)
  invokeMock.mockImplementation(async (command: string, args: Json, options?: unknown) => {
    switch (command) {
      case "get_local_storage_capability": return storage;
      case "load_design_knowledge": return knowledge;
      case "sync_native_shell_state": return null;
      case "check_model_library_refs": return [];
      case "load_preview_model": if (args === undefined) return structuredClone(pair.model); break;
      case "find_local_project": return null;
      case "list_model_states": return [];
      case "list_analysis_runs": return [];
      case "cancel_model_checks": return null;
      case "validate_model_operation_batch":
      case "apply_model_operation_batch": {
        const run = (await engine)[command === "apply_model_operation_batch" ? "applyOperationBatchJson" : "validateOperationBatchJson"];
        return JSON.parse(run(JSON.stringify(args.model), JSON.stringify(args.batch), JSON.stringify(args.claimedModelHash)));
      }
    }
    return transport.invoke(command, args, options);
  });
  (window as Json).__TAURI_INTERNALS__ = {};
  let current!: Session;
  function Harness() { current = useWorkspaceSession(); return null; }
  render(<Harness />);
  await waitFor(() => expect(current.model.modelHash).not.toBeNull());
  await act(async () => { await current.results.handleRun(); });
  await waitFor(() => expect(current.results.analysisRun).not.toBeNull(), { timeout: 20000 });
  const results = current.results;
  await act(async () => {
    await results.handleRuleCheckAggregate("USER_RULE_CHECKED", { projectSessionGeneration: current.model.projectSessionGeneration,
      modelRevision: current.model.uiModelRevision, result: results.currentSolvedResult, inputManifest: results.inputManifest });
  });
  await waitFor(() => expect(current.results.ruleCheckAggregate).toBe("USER_RULE_CHECKED"));
  return () => current;
}

it("a commit with a fresh result held clears the result, its analysis run, its input manifest and its rule-check aggregate, and says so", async () => {
  const state = await solvedSession();
  const held = state().results;
  expect([held.result, held.analysisRun, held.inputManifest, held.ruleCheckAggregate].map((value) => value !== null)).toEqual([true, true, true, true]);
  const node = state().model.model!.nodes[1];
  let outcome: Json;
  await act(async () => { outcome = await state().editorBinding.bindCommit(coordinate)({ nodeId: node.id, value: String(node.position.z + 0.5) }); });
  expect(outcome).toMatchObject({ status: "committed", resultsCleared: true });
  const cleared = state().results;
  expect([cleared.result, cleared.analysisRun, cleared.inputManifest, cleared.ruleCheckAggregate]).toEqual([null, null, null, null]);
  expect(state().model.model!.nodes[1].position.z).toBe(node.position.z + 0.5);
  expect(state().operations.undoStack).toHaveLength(1);
}, 60_000);

/** A commit with a checkpoint listener that throws `thrown` after publication, a fresh result and a proposal held:
 * committed with a notice, and every one of the ten clears is seen. */
async function commitWithFailingListener(thrown: () => unknown) {
  const state = await solvedSession();
  await act(async () => { await state().results.handleProposal(); });
  await waitFor(() => expect(state().results.proposal).not.toBeNull());
  const held = state().results;
  expect([held.result, held.analysisRun, held.inputManifest, held.ruleCheckAggregate].map((value) => value !== null)).toEqual([true, true, true, true]);
  state().operations.sessionEvents.listen((event) => event.type === "checkpoint" ? () => { throw thrown(); } : undefined);
  const ruleGate = vi.spyOn(RuleRevisionGenerationGate.prototype, "invalidate");
  const node = state().model.model!.nodes[1];
  let outcome: Json;
  await act(async () => { outcome = await state().editorBinding.bindCommit(coordinate)({ nodeId: node.id, value: String(node.position.z + 0.5) }); });
  expect(outcome).toMatchObject({ status: "committed", resultsCleared: true });
  const after = state().results;
  // The ten clears: the report package's computed state, the rule revision gate, the historical run, the result,
  // the analysis run, the input manifest, the rule-check aggregate, the proposal, the selected review target and the
  // solve job. The model write does not invalidate the rule revision gate, so its call here is the commit's clear.
  // The selected review target is set only by opening or creating a project, which this harness does not do; it is
  // asserted as left clear.
  expect({ busy: after.reportPackageBusy, redaction: after.reportPackageRedaction, route: after.reportPackageRoute }).toEqual({ busy: false, redaction: null, route: null });
  expect(ruleGate).toHaveBeenCalled();
  expect([after.historicalRun, after.result, after.analysisRun, after.inputManifest, after.ruleCheckAggregate, after.proposal, after.selectedReviewTarget])
    .toEqual([null, null, null, null, null, null, null]);
  const batchId = state().operations.undoStack[0].submitted_id!;
  expect(after.solveJob.job_id).toBe("job:preview-linear-static:session-batch");
  expect(after.solveJob.events[0].message).toContain(`Applied batch local session model operation ${batchId};`);
  expect(state().model.model!.nodes[1].position.z).toBe(node.position.z + 0.5);
  return outcome;
}

it("a listener's failure after publication, with a fresh result held: the commit is committed with a notice, and all ten clears are made", async () => {
  const outcome = await commitWithFailingListener(() => new Error("invented listener failure"));
  expect(outcome.notice).toContain("invented listener failure");
}, 60_000);

it("a listener throwing an object with no prototype after publication: committed with a fixed notice, and all ten clears are made (Q3)", async () => {
  const outcome = await commitWithFailingListener(() => Object.create(null));
  expect(outcome.notice).toContain("A view of the session did not take this change: a failure that could not be put into words");
}, 60_000);
