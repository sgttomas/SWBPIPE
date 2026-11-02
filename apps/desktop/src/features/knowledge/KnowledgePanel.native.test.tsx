// C1 slice 4 (DEL-04-05 `c1-native-custody-plan.md`): the Knowledge panel reads a native result's
// axial-force row by one bounded native lookup (the actual Rust command's recorded reply). A refused
// lookup is said as a refusal, never shown as a result without that row. Unit transport replay;
// NOT native UI qualification. Invented data only.
import { cleanup, render, screen } from "@testing-library/react";
import { afterEach, beforeEach, expect, it, vi } from "vitest";

const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../../test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));

import { awaitPreviewMechanicsJob, disposeDeliveredMechanicsResult, readNativeFreshResult, resetSessionResultCustodyForTests, startPreviewMechanicsJob } from "../../services/previewService";
import { resetNativeResultsForTests, type NativeResult } from "../../services/nativeResult";
import { createNativeMechanicsReplay, nativeMechanicsReplayPair, REPLAY_COMMANDS } from "../../test/nativeMechanicsReplay";
import { KnowledgePanel } from "./KnowledgePanel";
import { DiagnosticsPanel } from "../diagnostics/DiagnosticsPanel";
import type { MechanicsResult } from "../../types";

let refuseLookup = false;
let refuseLinked = false;
beforeEach(() => { resetSessionResultCustodyForTests(); resetNativeResultsForTests(); refuseLookup = false; refuseLinked = false; });
afterEach(() => {
  cleanup();
  invokeMock.mockReset();
  delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
});

async function nativeResult(): Promise<NativeResult> {
  const replay = createNativeMechanicsReplay({ profile: "preview" });
  invokeMock.mockImplementation((command: string, args?: unknown, options?: unknown) => {
    if (command === "lookup_preview_mechanics_job_rows" && refuseLookup) {
      return Promise.reject({ code: "RESOURCE_UNAVAILABLE", refusal: { kind: "headroom_unknown" }, pool: null });
    }
    if (command === "query_preview_mechanics_job_rows" && refuseLinked) {
      return Promise.reject({ code: "RESOURCE_UNAVAILABLE", refusal: { kind: "headroom_unknown" }, pool: null });
    }
    if (REPLAY_COMMANDS.includes(command)) return replay.invoke(command, args, options);
    return Promise.reject(new Error(`UNEXPECTED_COMMAND: ${command}`));
  });
  (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
  const pair = nativeMechanicsReplayPair("sparse_interactive", { profile: "preview" });
  const started = await startPreviewMechanicsJob(pair.model, "sparse_interactive", { custody: "native" });
  if (started.mode !== "backend_job") throw new Error("not a backend job");
  const terminal = await awaitPreviewMechanicsJob(started.job_id);
  return (await readNativeFreshResult(started.job_id, terminal.result_bytes ?? 0, { standingModel: pair.model, summaryModel: pair.model })).result;
}

it("shows the computed axial-force record from the native lookup: the same row the full rows give", async () => {
  const result = await nativeResult();
  // The oracle: the record's row over the full rows (the fixed id, else the first of its kind).
  const rows = (JSON.parse(nativeMechanicsReplayPair("sparse_interactive", { profile: "preview" }).bytes) as MechanicsResult).results;
  const expected = rows.find((row) => row.id === "result:force:pipe-P-120:axial") ?? rows.find((row) => row.kind === "element_local_axial_force")!;
  render(<KnowledgePanel knowledge={null} result={result} />);
  expect(await screen.findByTestId("knowledge-record-knowledge:computed-axial-force")).toHaveTextContent(`${expected.id} is`);
  expect(screen.queryByTestId("knowledge-axial-force-refused")).toBeNull();
  await disposeDeliveredMechanicsResult(result);
});

it("says a refused lookup is refused, and shows no axial-force record as if the row were absent", async () => {
  const result = await nativeResult();
  refuseLookup = true;
  render(<KnowledgePanel knowledge={null} result={result} />);
  expect(await screen.findByTestId("knowledge-axial-force-refused")).toHaveTextContent("The computed axial-force record could not be read from the result:");
  expect(screen.queryByTestId("knowledge-record-knowledge:computed-axial-force")).toBeNull();
  await disposeDeliveredMechanicsResult(result);
});

it("says a refused diagnostic-linked row query is refused", async () => {
  const result = await nativeResult();
  refuseLinked = true;
  const pair = nativeMechanicsReplayPair("sparse_interactive", {profile:"preview"});
  const model = {...pair.model,diagnostics:[...pair.model.diagnostics,{id:"diagnostic:linked-native",code:"LINKED_NATIVE",severity:"warning" as const,message:"Invented linked-row diagnostic",affected_refs:["pipe:P-120"]}]};
  render(<DiagnosticsPanel model={model} knowledge={null} result={result} selectedDiagnosticId="diagnostic:linked-native" onSelectDiagnostic={() => {}}/>);
  expect(await screen.findByTestId("diagnostic-linked-results-refused")).toHaveTextContent("The linked result query could not be read:");
  await disposeDeliveredMechanicsResult(result);
});
