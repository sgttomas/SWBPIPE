import { useEffect } from "react";
import { act, cleanup, fireEvent, render, screen } from "@testing-library/react";
import { afterEach, beforeEach, expect, it, vi } from "vitest";
import type { SessionResult } from "../../services/nativeResult";

const invokeMock=vi.hoisted(() => vi.fn());
const context=vi.hoisted(() => ({result:null as unknown}));
vi.mock("@tauri-apps/api/core",async () => ({invoke:(await import("../../test/nativeIngress")).ingressDecodingInvoke(invokeMock)}));
vi.mock("../workspace/WorkspaceSessionContext",() => ({useOptionalSessionResults:() => ({result:context.result})}));
import { awaitPreviewMechanicsJob, readNativeFreshResult, resetSessionResultCustodyForTests, startPreviewMechanicsJob } from "../../services/previewService";
import { resetNativeResultsForTests } from "../../services/nativeResult";
import { createNativeMechanicsReplay, nativeMechanicsReplayPair } from "../../test/nativeMechanicsReplay";
import { C2ResultGate } from "./C2ResultGate";

beforeEach(() => { resetSessionResultCustodyForTests(); resetNativeResultsForTests(); context.result=null; });
afterEach(() => { cleanup(); invokeMock.mockReset(); delete (window as unknown as Record<string,unknown>).__TAURI_INTERNALS__; });

it("drops a ready explicit gate's old copy even when dormant props still name the old result",async () => {
  const replay=createNativeMechanicsReplay({profile:"preview"});
  invokeMock.mockImplementation(replay.invoke);
  (window as unknown as Record<string,unknown>).__TAURI_INTERNALS__={};
  const pair=nativeMechanicsReplayPair("sparse_interactive",{profile:"preview"});
  const read=async () => {
    const started=await startPreviewMechanicsJob(pair.model,"sparse_interactive",{custody:"native"});
    if (started.mode!=="backend_job") throw new Error("not native");
    const terminal=await awaitPreviewMechanicsJob(started.job_id);
    return (await readNativeFreshResult(started.job_id,terminal.result_bytes??0,{standingModel:pair.model,summaryModel:pair.model})).result;
  };
  const first=await read();
  const second=await read();
  const full=JSON.parse(pair.bytes);
  const load=vi.fn(async (_result:SessionResult) => ({result:full,analysisRun:null}));
  context.result=first;
  let cleanups=0;
  // A mounted C2 consumer may itself retain the parsed rows. Retiring the
  // source must unmount it without waiting for another explicit C2 request.
  function Consumer() { useEffect(() => () => { cleanups+=1; },[]); return <span data-testid="full-consumer">{full.run_id}</span>; }
  const gate=() => <C2ResultGate result={first} analysisRun={null} feature="comparisons" explicit load={load}>{() => <Consumer/>}</C2ResultGate>;
  const view=render(gate());
  await act(async () => { fireEvent.click(screen.getByTestId("c2-result-load")); });
  expect(screen.getByTestId("full-consumer")).toBeInTheDocument();
  context.result=second;
  view.rerender(gate());
  expect(screen.queryByTestId("full-consumer")).toBeNull();
  expect(cleanups).toBe(1);
  expect(load).toHaveBeenCalledTimes(1);
});
