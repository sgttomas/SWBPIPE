import { useEffect } from "react";
import { act, cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, beforeEach, expect, it, vi } from "vitest";
import type { SessionAnalysisRun, SessionResult } from "../../services/nativeResult";

const invokeMock=vi.hoisted(() => vi.fn());
const context=vi.hoisted(() => ({result:null as unknown}));
vi.mock("@tauri-apps/api/core",async () => ({invoke:(await import("../../test/nativeIngress")).ingressDecodingInvoke(invokeMock)}));
vi.mock("../workspace/WorkspaceSessionContext",() => ({useOptionalSessionResults:() => ({result:context.result})}));
import { awaitPreviewMechanicsJob, readNativeFreshResult, resetSessionResultCustodyForTests, startPreviewMechanicsJob } from "../../services/previewService";
import { FullResultRefused, RESULT_DETAIL_WEBVIEW_LIMIT_BYTES, resetNativeResultsForTests } from "../../services/nativeResult";
import { ReportPackageSaveStatus } from "../report/RenderedReportPanel";
import type { ReportPackageSaveRoute } from "../../services/reportPackageSaveService";
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

it("asks the loader for the run record exactly when it was given one, and loads again when one is given later",async () => {
  const replay=createNativeMechanicsReplay({profile:"preview"});
  invokeMock.mockImplementation(replay.invoke);
  (window as unknown as Record<string,unknown>).__TAURI_INTERNALS__={};
  const pair=nativeMechanicsReplayPair("sparse_interactive",{profile:"preview"});
  const started=await startPreviewMechanicsJob(pair.model,"sparse_interactive",{custody:"native"});
  if (started.mode!=="backend_job") throw new Error("not native");
  const terminal=await awaitPreviewMechanicsJob(started.job_id);
  const native=(await readNativeFreshResult(started.job_id,terminal.result_bytes??0,{standingModel:pair.model,summaryModel:pair.model})).result;
  const full=JSON.parse(pair.bytes);
  const record={analysis_run:{run_id:native.run_id}} as unknown as SessionAnalysisRun;
  const load=vi.fn(async (_result:SessionResult,request?:{record:boolean}) => ({result:full,analysisRun:request?.record ? record as never : null}));
  context.result=native;
  const gate=(analysisRun:SessionAnalysisRun|null) => (
    <C2ResultGate result={native} analysisRun={analysisRun} feature="the report" load={load}>
      {(loaded) => <span data-testid="full-consumer">{loaded.analysisRun ? "with record" : "without record"}</span>}
    </C2ResultGate>
  );
  const view=render(gate(null));
  expect(await screen.findByTestId("full-consumer")).toHaveTextContent("without record");
  // The handle guards its rows, so the call is read argument by argument.
  expect(load.mock.calls.at(-1)?.[0]).toBe(native);
  expect(load.mock.calls.at(-1)?.[1]).toEqual({record:false});
  view.rerender(gate(record));
  await waitFor(() => expect(screen.getByTestId("full-consumer")).toHaveTextContent("with record"));
  // The handle guards its rows, so the call is read argument by argument.
  expect(load.mock.calls.at(-1)?.[0]).toBe(native);
  expect(load.mock.calls.at(-1)?.[1]).toEqual({record:true});
  expect(load).toHaveBeenCalledTimes(2);
});

// A request that did not proceed is announced here; a save's own outcome is shown here and
// announced by the session's report output strip, which keeps it (issue 508), not again here.
it.each([
  ["a request that did not proceed", { route: "not_requested", diagnostic: "Not saved: a report package was already being saved." }, true],
  ["the oversized record's refusal", { route: "redaction_blocked", diagnostic: `RESULT-DETAIL-WEBVIEW-LIMIT (record_bytes=${RESULT_DETAIL_WEBVIEW_LIMIT_BYTES + 1}, limit_bytes=${RESULT_DETAIL_WEBVIEW_LIMIT_BYTES}): the run record is above the size the webview can hold whole.` }, false]
] as const)("under a refused gate, %s is shown below the refusal (announced here: %s)", async (_name, route, announcedHere) => {
  const replay=createNativeMechanicsReplay({profile:"preview"});
  invokeMock.mockImplementation(replay.invoke);
  (window as unknown as Record<string,unknown>).__TAURI_INTERNALS__={};
  const pair=nativeMechanicsReplayPair("sparse_interactive",{profile:"preview"});
  const started=await startPreviewMechanicsJob(pair.model,"sparse_interactive",{custody:"native"});
  if (started.mode!=="backend_job") throw new Error("not native");
  const terminal=await awaitPreviewMechanicsJob(started.job_id);
  const native=(await readNativeFreshResult(started.job_id,terminal.result_bytes??0,{standingModel:pair.model,summaryModel:pair.model})).result;
  context.result=native;
  const record={analysis_run:{run_id:native.run_id}} as unknown as SessionAnalysisRun;
  const load=vi.fn(async () => { throw new FullResultRefused("RESULT-DETAIL-WEBVIEW-LIMIT", RESULT_DETAIL_WEBVIEW_LIMIT_BYTES + 1, RESULT_DETAIL_WEBVIEW_LIMIT_BYTES, "record_bytes", "record"); });
  render(
    <C2ResultGate result={native} analysisRun={record} feature="the rendered report" load={load}
      withheld={<ReportPackageSaveStatus route={route as ReportPackageSaveRoute} />}>
      {() => <span data-testid="full-consumer" />}
    </C2ResultGate>
  );
  const refusal=await screen.findByTestId("c2-result-refused");
  expect(screen.queryByTestId("full-consumer")).toBeNull();
  const shown=screen.getByTestId("report-package-save-status");
  expect(shown.textContent).toBe(route.diagnostic);
  const announced=screen.getAllByRole("status").filter((status) => status.textContent?.includes(route.diagnostic));
  expect(announced).toEqual(announcedHere ? [shown] : []);
  // The gate's own refusal is a separate status, before it; the wrapper adds no role.
  expect(refusal.compareDocumentPosition(shown) & Node.DOCUMENT_POSITION_FOLLOWING).toBeTruthy();
  expect(screen.getByTestId("c2-result-withheld")).not.toHaveAttribute("role");
});
