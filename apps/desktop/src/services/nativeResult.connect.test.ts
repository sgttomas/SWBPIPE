// C1 slice 4: the connecting path of a fresh solve held
// natively, through the actual raw-body ingress, the replay transport and the actual Rust commands'
// recorded replies (`test/nativeReplayStore.ts`, `src-tauri/src/native_replay_tests.rs`): start with
// native custody, poll, the native verdict (delivered as a parsed JSON value), the registration's
// reader validations, the standing, S (an ArrayBuffer) and the handle. Nothing takes the result,
// and the handle refuses every full-row access. Unit transport replay; NOT native UI qualification.
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));

import { awaitPreviewMechanicsJob, discardPreviewMechanicsJobResult, disposeDeliveredMechanicsResult, loadNativeFullResult, readNativeFreshResult, resetSessionResultCustodyForTests, startPreviewMechanicsJob } from "./previewService";
import * as readerModule from "../features/results/resultReader";
import { decodeNativeVerdict, decodeNativeValidation, decodeNativeRouteHeader, requireRouteHeaderAgreement, readNativeRouteHeader, readNativeVerdict, setNativeResultTransportForTests, tauriNativeResultInvoke, forgetNativeJob, describeFullResultRefused, FullResultRefused, isNativeJobLive, isNativeResult, loadFullResult, nativeCustody, NativeResultRowsGuarded, resetNativeResultsForTests } from "./nativeResult";
import { createNativeMechanicsReplay, createSolveJobTransport, nativeMechanicsReplayPair, REPLAY_COMMANDS } from "../test/nativeMechanicsReplay";
import { readerDispatch, rememberReceivedText } from "../features/results/resultReader";
import type { MechanicsResult } from "../types";
import { beginSolveTimeline, markSolveStage, readSolveTimeline, resetSolveTimelineForTests } from "./solveTimeline";

type Replay = ReturnType<typeof createNativeMechanicsReplay>;
function install(replay: { invoke: Replay["invoke"] }) {
  invokeMock.mockImplementation((command: string, args?: unknown, options?: unknown) => {
    if (command === "dispose_all_preview_mechanics_results") return Promise.resolve({ released_results: 0, released_bytes: 0, abandoned_jobs: 0 });
    if (REPLAY_COMMANDS.includes(command)) return replay.invoke(command, args, options);
    return Promise.reject(new Error(`UNEXPECTED_COMMAND: ${command}`));
  });
  (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
}
beforeEach(() => { resetSessionResultCustodyForTests(); resetNativeResultsForTests(); resetSolveTimelineForTests(); });
afterEach(() => {
  vi.restoreAllMocks();
  invokeMock.mockReset();
  setNativeResultTransportForTests(null);
  delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
});

describe("C1 slice 4: a fresh solve's result is read natively and never taken", () => {
  // Bounded diagnosis of the A500 packet's coarse mark. Freeze each actual
  // awaited native call in turn, while replaying committed small Rust replies.
  // No solve/worker/App or timing/allocation workload is executed here.
  for (const pendingPhase of ["header", "registration", "standing", "summary"] as const) {
    it(`the existing reader-start mark also covers a pending ${pendingPhase} call`, async () => {
      const replay = createNativeMechanicsReplay({ profile: "pressure" });
      install(replay);
      const pair = nativeMechanicsReplayPair("sparse_interactive", { profile: "pressure" });
      const started = await startPreviewMechanicsJob(pair.model, "sparse_interactive", { custody: "native" });
      if (started.mode !== "backend_job") throw new Error("not native");
      const terminal = await awaitPreviewMechanicsJob(started.job_id);
      const timeline = beginSolveTimeline({ solver_mode: "sparse_interactive" });
      timeline.jobStarted(started.job_id);
      markSolveStage("job_done", { result_bytes: terminal.result_bytes ?? 0 });
      const forward = invokeMock.getMockImplementation()!;
      const phases: string[] = [];
      const boundaries: {phase: string; edge: string; status: string}[] = [];
      globalThis.__SWBPIPE_NATIVE_SMOKE_READER_PHASE__ = async event => { boundaries.push(event); };
      let unblock: (() => void) | undefined;
      invokeMock.mockImplementation(async (command: string, raw?: unknown, options?: unknown) => {
        const args = raw as { reader?: string; modelJson?: string; invocationJson?: string };
        const phase = command === "read_preview_mechanics_job_summary" ? "summary"
          : command !== "read_preview_mechanics_job_verdict" ? null
          : (raw as {operation?:string})?.operation === "route_header" ? "header" : args?.reader ? "registration" : "standing";
        if (phase) phases.push(phase);
        if (phase === pendingPhase) await new Promise<void>(resolve => { unblock = resolve; });
        return forward(command, raw, options);
      });
      const reading = readNativeFreshResult(started.job_id, terminal.result_bytes ?? 0, { standingModel: pair.model, summaryModel: pair.model });
      try {
        await vi.waitFor(() => expect(unblock).toBeTypeOf("function"));
        expect(phases.at(-1)).toBe(pendingPhase);
        expect(boundaries.at(-1)).toMatchObject({phase: pendingPhase, edge: "enter", status: "pending"});
        expect(boundaries.filter(event => event.edge === "exit")).toHaveLength(phases.length - 1);
        expect(readSolveTimeline()?.marks.at(-1)?.stage).toBe("reader_verdict_start");
      } finally { unblock?.(); }
      const native = (await reading).result;
      expect(phases).toEqual(["header", "registration", "standing", "summary"]);
      expect(boundaries).toHaveLength(8);
      expect(boundaries.at(-1)).toMatchObject({phase: "summary", edge: "exit", status: "ok"});
      // Distinct metadata, validation and dispatch caches; final standing is reused.
      expect(phases.filter(phase => phase !== "summary")).toHaveLength(3);
      expect(readSolveTimeline()?.marks.at(-1)?.stage).toBe("reader_verdict_end");
      await disposeDeliveredMechanicsResult(native);
    });
  }
  it("keeps the entered await and records a refusal when a pending small replay is disposed", async () => {
    const replay = createNativeMechanicsReplay({profile:"pressure"});
    install(replay);
    const pair = nativeMechanicsReplayPair("sparse_interactive", {profile:"pressure"});
    const started = await startPreviewMechanicsJob(pair.model,"sparse_interactive",{custody:"native"});
    if (started.mode !== "backend_job") throw new Error("not native");
    const terminal = await awaitPreviewMechanicsJob(started.job_id);
    const events: {phase:string; edge:string; status:string}[]=[];
    globalThis.__SWBPIPE_NATIVE_SMOKE_READER_PHASE__=async event=>{events.push(event);};
    const forward=invokeMock.getMockImplementation()!;
    let resume!:()=>void;
    invokeMock.mockImplementation(async(command:string,args?:unknown,options?:unknown)=>{
      const reply=await forward(command,args,options);
      if(command === "read_preview_mechanics_job_verdict" && (args as {operation?:string})?.operation === "route_header") await new Promise<void>(resolve=>{resume=resolve;});
      return reply;
    });
    const reading=readNativeFreshResult(started.job_id,terminal.result_bytes??0,{standingModel:pair.model,summaryModel:pair.model});
    const refused=expect(reading).rejects.toThrow("NATIVE-RESULT-DISPOSED");
    await vi.waitFor(()=>expect(resume).toBeTypeOf("function"));
    expect(events).toEqual([expect.objectContaining({phase:"header",edge:"enter",status:"pending"})]);
    await discardPreviewMechanicsJobResult(started.job_id);
    resume();await refused;
    expect(events.some(event=>event.edge === "exit" && event.status === "error")).toBe(true);
    expect(events.some(event=>event.phase === "summary")).toBe(false);
  });
  it("names the run record when its full-copy size is refused", () => {
    const refusal = new FullResultRefused("RESULT-DETAIL-WEBVIEW-LIMIT", 65, 64, "record_bytes=65", "record");
    expect(describeFullResultRefused(refusal,"reports")).toContain("This run record is 65 bytes");
  });
  it("releases a copied result when its required re-registration fails", async () => {
    const replay=createNativeMechanicsReplay({profile:"preview"});
    install(replay);
    const pair=nativeMechanicsReplayPair("sparse_interactive",{profile:"preview"});
    const started=await startPreviewMechanicsJob(pair.model,"sparse_interactive",{custody:"native"});
    if (started.mode!=="backend_job") throw new Error("not native");
    const terminal=await awaitPreviewMechanicsJob(started.job_id);
    const native=(await readNativeFreshResult(started.job_id,terminal.result_bytes??0,{standingModel:pair.model,summaryModel:pair.model})).result;
    const validation=vi.spyOn(readerModule,"readerValidate").mockImplementation(() => {throw new Error("SOURCE_SEMANTIC_CONTRACT_UNSUPPORTED");});
    await expect(loadNativeFullResult(native)).rejects.toThrow("C2-RESULT-REGISTRATION-REFUSED: SOURCE_SEMANTIC_CONTRACT_UNSUPPORTED");
    expect(invokeMock.mock.calls.map(([command]) => command)).toContain("release_preview_mechanics_job_full_copy");
    validation.mockRestore();
    // A failed copy is not cached, so the next request can be registered.
    expect((await loadNativeFullResult(native)).run_id).toBe(native.run_id);
    await disposeDeliveredMechanicsResult(native);
  });
  it("propagates a registration resource refusal and forgets the failed job's cached reads", async () => {
    const replay = createNativeMechanicsReplay({profile:"preview"});
    install(replay);
    const forward=invokeMock.getMockImplementation()!;
    const refusal={code:"RESOURCE_UNAVAILABLE",refusal:{kind:"headroom_unknown"},pool:null};
    invokeMock.mockImplementation((command: string,args?: unknown,options?: unknown) => {
      if (command==="read_preview_mechanics_job_verdict" && (args as {reader?:string})?.reader) return Promise.reject(refusal);
      return forward(command,args,options);
    });
    const pair=nativeMechanicsReplayPair("sparse_interactive",{profile:"preview"});
    const started=await startPreviewMechanicsJob(pair.model,"sparse_interactive",{custody:"native"});
    if (started.mode!=="backend_job") throw new Error("not native");
    const terminal=await awaitPreviewMechanicsJob(started.job_id);
    await expect(readNativeFreshResult(started.job_id,terminal.result_bytes??0,{standingModel:pair.model,summaryModel:pair.model})).rejects.toMatchObject({refusal:{kind:"headroom_unknown"}});
    const verdictRequests=invokeMock.mock.calls.filter(([command])=>command === "read_preview_mechanics_job_verdict").map(([,args])=>args as Record<string,unknown>);
    expect(verdictRequests[0].operation).toBe("route_header");
    expect(verdictRequests.at(-1)?.reader).toBe("preview-physics");
    expect(invokeMock.mock.calls.some(([command])=>command === "read_preview_mechanics_job_summary")).toBe(false);
    await discardPreviewMechanicsJobResult(started.job_id);
    expect(isNativeJobLive(started.job_id)).toBe(false);
    expect(replay.nativeHeld()).toEqual([]);
    await expect(readNativeFreshResult(started.job_id,terminal.result_bytes??0,{standingModel:pair.model,summaryModel:pair.model})).rejects.toThrow("NATIVE-RESULT-DISPOSED");
  });
  it("rejects a metadata digest that disagrees with the compulsory full reading", async () => {
    const replay=createNativeMechanicsReplay({profile:"pressure"});install(replay);
    setNativeResultTransportForTests(async(command,args)=>{
      const value=await tauriNativeResultInvoke(command,args);
      return args.operation === "route_header" ? {...value as object,binding:{...(value as {binding:object}).binding,sha256:"0".repeat(64)}} : value;
    });
    const pair=nativeMechanicsReplayPair("sparse_interactive",{profile:"pressure"});
    const started=await startPreviewMechanicsJob(pair.model,"sparse_interactive",{custody:"native"});
    if(started.mode !== "backend_job")throw new Error("not native");
    const terminal=await awaitPreviewMechanicsJob(started.job_id);
    await expect(readNativeFreshResult(started.job_id,terminal.result_bytes??0,{standingModel:pair.model,summaryModel:pair.model})).rejects.toThrow("differs from the compulsory full verdict");
    const requests=invokeMock.mock.calls.filter(([command])=>command === "read_preview_mechanics_job_verdict").map(([,args])=>args as Record<string,unknown>);
    expect(requests[0].operation).toBe("route_header");
    expect(requests.some(args=>args.reader === "pressure")).toBe(true);
    expect(requests.some(args=>args.operation === undefined && args.reader === undefined && typeof args.modelJson === "string")).toBe(true);
    await discardPreviewMechanicsJobResult(started.job_id);
    expect(replay.nativeHeld()).toEqual([]);
  });
  for (const profile of ["preview", "pressure"] as const) {
    it(`${profile}: verdict, registration, standing and S through the actual ingress and the actual commands' replies`, async () => {
      const replay = createNativeMechanicsReplay({ profile });
      install(replay);
      const pair = nativeMechanicsReplayPair("sparse_interactive", { profile });
      const started = await startPreviewMechanicsJob(pair.model, "sparse_interactive", { custody: "native" });
      if (started.mode !== "backend_job") throw new Error("not a backend job");
      const receipt = started;
      const terminal = await awaitPreviewMechanicsJob(receipt.job_id);
      expect(terminal.state).toBe("completed");
      const reading = await readNativeFreshResult(receipt.job_id, terminal.result_bytes ?? 0, { standingModel: pair.model, summaryModel: pair.model });
      const native = reading.result;
      expect(isNativeResult(native)).toBe(true);
      expect(reading.unregistered).toBeNull();
      // The oracle: the full rows read by the existing (wasm) readers, in the test only.
      const full = JSON.parse(pair.bytes) as MechanicsResult;
      rememberReceivedText(full, pair.bytes);
      const legacy = readerDispatch(full, pair.model).standing;
      expect(reading.standing.eligible).toBe(legacy.eligible);
      expect(reading.standing.findings).toEqual([...legacy.findings]);
      expect(nativeCustody(native).verdict.route).toBe(readerDispatch(full).route);
      expect(nativeCustody(native).summary.header.row_count).toBe(full.results.length);
      expect(native.run_id).toBe(full.run_id);
      expect(native.model_ref).toBe(full.model_ref);
      expect(native.diagnostics).toEqual(full.diagnostics);
      // The handle holds no rows and refuses every full-row access, by name.
      expect(() => (native as unknown as MechanicsResult).results).toThrow(NativeResultRowsGuarded);
      expect(() => JSON.stringify(native)).toThrow("NATIVE-RESULT-ROWS-GUARDED");
      expect(() => structuredClone(native)).toThrow();
      expect(() => readerDispatch(native, undefined, undefined).route).not.toThrow();
      // Nothing took the result; the job stays held natively until disposal.
      const sent = invokeMock.mock.calls.map((call) => call[0]);
      expect(sent).not.toContain("take_preview_mechanics_job_result");
      expect(sent).not.toContain("read_preview_mechanics_job_rows_all");
      expect(sent).toEqual(expect.arrayContaining(["read_preview_mechanics_job_verdict", "read_preview_mechanics_job_summary"]));
      expect(replay.nativeHeld()).toEqual([receipt.job_id]);
      await disposeDeliveredMechanicsResult(native);
      expect(replay.nativeHeld()).toEqual([]);
    });
  }

  it("reads the verdict as Tauri delivers a JSON body (a parsed value), and refuses text or bytes", () => {
    const value = { ok: true, route: "preview_physics", binding: null, error: null, standing: { route: "preview_physics", token: "needs_recompute", status: "needs_recompute", eligible: false, findings: ["X"] } };
    expect(decodeNativeVerdict(value).route).toBe("preview_physics");
    expect(() => decodeNativeVerdict(JSON.stringify(value))).toThrow("NATIVE-VERDICT-MALFORMED: the delivery is text");
    expect(() => decodeNativeVerdict(new TextEncoder().encode(JSON.stringify(value)).buffer)).toThrow("NATIVE-VERDICT-MALFORMED");
  });

  it("C2's full copy: refused by name one byte above the limit, delivered whole at it", async () => {
    const pair = nativeMechanicsReplayPair("sparse_interactive", { profile: "preview" });
    const length = new TextEncoder().encode(pair.bytes).byteLength;
    for (const [limit, refused] of [[length - 1, true], [length, false]] as const) {
      resetNativeResultsForTests();
      const transport = createSolveJobTransport(() => pair.bytes, { fullCopyLimitBytes: limit });
      install({ invoke: transport.invoke });
      const started = await startPreviewMechanicsJob(pair.model, "sparse_interactive", { custody: "native" });
      if (started.mode !== "backend_job") throw new Error("not a backend job");
      const receipt = started;
      await awaitPreviewMechanicsJob(receipt.job_id);
      const native = (await readNativeFreshResult(receipt.job_id, length, { standingModel: pair.model, summaryModel: pair.model })).result;
      const load = loadFullResult(native, async (text) => JSON.parse(text) as MechanicsResult);
      if (refused) {
        const error = await load.catch((caught) => caught);
        expect(error).toBeInstanceOf(FullResultRefused);
        expect(error).toMatchObject({ code: "RESULT-DETAIL-WEBVIEW-LIMIT", resultBytes: length, limitBytes: length - 1 });
      } else {
        expect((await load).run_id).toBe(native.run_id);
      }
      await disposeDeliveredMechanicsResult(native);
    }
  });
});


describe("the distinct native route header grants no verdict or registration", () => {
  function pair(job = "header-job") {
    const full = readerDispatch(nativeMechanicsReplayPair("sparse_interactive", {profile:"pressure"}).source);
    const header = {document_kind:"swbpipe.desktop.result_route_header",version:1,job_id:job,header_status:"recognized",route:full.route,binding:full.binding,refusal:null};
    return {full,header};
  }
  it("strictly decodes only its exact shape and old decoders reject it", () => {
    const {header,full}=pair();
    const decoded=decodeNativeRouteHeader(header,"header-job");
    expect(Object.isFrozen(decoded)).toBe(true);
    expect(() => decodeNativeVerdict(header)).toThrow("NATIVE-VERDICT-MALFORMED");
    expect(() => decodeNativeValidation(header)).toThrow("NATIVE-VERDICT-MALFORMED");
    expect(() => decodeNativeRouteHeader(full,"header-job")).toThrow("NATIVE-VERDICT-MALFORMED");
    for(const value of [JSON.stringify(header),new TextEncoder().encode(JSON.stringify(header)),{...header,job_id:"foreign"},{...header,version:2},{...header,ok:true},{...header,standing:full.standing},{...header,model_ref:"x"},{...header,route:"unsupported"},{...header,header_status:"refused"},{...header,binding:{...header.binding,sha256:"wrong"}},{...header,binding:{...header.binding,id:"foreign"}}]) {
      expect(()=>decodeNativeRouteHeader(value,"header-job")).toThrow("NATIVE-VERDICT-MALFORMED");
    }
  });
  it("checks the actual table digest against the compulsory full verdict before use", () => {
    const {header,full}=pair();
    requireRouteHeaderAgreement(decodeNativeRouteHeader(header,"header-job"),full);
    const wrong=decodeNativeRouteHeader({...header,binding:{...header.binding,sha256:"0".repeat(64)}},"header-job");
    expect(()=>requireRouteHeaderAgreement(wrong,full)).toThrow("differs from the compulsory full verdict");
    const refused={document_kind:header.document_kind,version:1,job_id:"header-job",header_status:"refused",route:"unsupported",binding:null,refusal:{code:"SOURCE_NUMERICAL_CONTRACT_UNSUPPORTED",detail:null}};
    expect(decodeNativeRouteHeader(refused,"header-job").route).toBe("unsupported");
    expect(()=>requireRouteHeaderAgreement(decodeNativeRouteHeader(refused,"header-job"),full)).toThrow("differs");
  });
  it("keeps metadata and full-dispatch caches distinct with no fallback", async () => {
    const {header,full}=pair();const sent: unknown[]=[];
    setNativeResultTransportForTests(async(_command,args)=>{sent.push(args);return args.operation === "route_header" ? header : full;});
    expect(await readNativeRouteHeader("header-job")).toEqual(header);
    await readNativeRouteHeader("header-job");
    expect(await readNativeVerdict("header-job",{})).toEqual(full);
    expect(sent).toEqual([{jobId:"header-job",operation:"route_header"},{jobId:"header-job"}]);
    forgetNativeJob("header-job");
    expect(()=>readNativeRouteHeader("header-job")).toThrow("NATIVE-RESULT-DISPOSED");
    expect(sent).toHaveLength(2);
  });
  it("rejects disposed or foreign late metadata and removes failed metadata cache entries", async () => {
    const {header}=pair();let reply!: (value: unknown)=>void;let calls=0;
    setNativeResultTransportForTests(async()=>{calls++;return new Promise(resolve=>{reply=resolve;});});
    const pending=readNativeRouteHeader("header-job");
    forgetNativeJob("header-job");reply(header);
    await expect(pending).rejects.toThrow("NATIVE-RESULT-DISPOSED");
    resetNativeResultsForTests();
    const foreign=readNativeRouteHeader("header-job");reply({...header,job_id:"foreign"});
    await expect(foreign).rejects.toThrow("NATIVE-VERDICT-MALFORMED");
    const good=readNativeRouteHeader("header-job");reply(header);
    await expect(good).resolves.toEqual(header);
    expect(calls).toBe(3);
  });
});
