/** DEL-08-01 REQ-024, "Package and user release": the user-release dialog's frame statements
 * come from the native host, word for word, for a result in its custody (an exact-route or a
 * pressure-free Timoshenko result); a pressure-free Euler–Bernoulli result has none; an exact-route result whose statements cannot be obtained is refused by
 * name. The native replies here are the committed files the Rust test
 * `the_native_host_gives_the_dialog_the_frame_statements_the_package_states` holds equal to
 * the native operation on each witness. Unit transport; every id is invented. */
import { readFileSync } from "node:fs";
import path from "node:path";
import { afterEach, describe, expect, it, vi } from "vitest";
import { createNativeResult, NATIVE_VERDICT_COMMAND, resetNativeResultsForTests, setNativeResultTransportForTests, type NativeCustody } from "../../services/nativeResult";
import { PRESSURE_CONTRACT_ID, PREVIEW_PHYSICS_CONTRACT_ID, PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID } from "../results/numericalResultQuality";
import { REPORT_FRAME_STATEMENTS_UNAVAILABLE, reportFrameStatements } from "./reportFrameStatements";

const root = path.resolve(__dirname, "../../../../..");
const DIR = "fixtures/reports/invented/report_standing_witnesses";
function committedFrameStatements(name: string): string[] {
  return JSON.parse(readFileSync(path.join(root, DIR, `${name}.frame_statements.json`), "utf8")).frame_statements;
}

afterEach(() => {
  setNativeResultTransportForTests(null);
  resetNativeResultsForTests();
});

function handle(jobId: string, contract: string, standing: NativeCustody["binding"]["standing"] = { eligible: false, findings: [] }) {
  return createNativeResult({
    jobId,
    origin: "fresh_job",
    resultBytes: 1,
    summary: { header: { envelope: { producer: { semantic_contract_id: contract } } } } as unknown as NativeCustody["summary"],
    verdict: {} as NativeCustody["verdict"],
    binding: { standingModelText: "{\"invented\":\"model\"}", invocationText: null, summaryModelText: "{}", standing },
    recovery: null
  });
}

describe("reportFrameStatements", () => {
  it("gives the native host's statements for an exact-route result in its custody, read with its standing's bound texts", async () => {
    const lines = committedFrameStatements("pressure_3_gap_unresolved");
    expect(lines).toHaveLength(2);
    const transport = vi.fn(async (_command: string, args: Record<string, unknown>) => ({
      document_kind: "swbpipe.desktop.report_frame_statements", version: 1, job_id: args.jobId, frame_statements: lines
    }));
    setNativeResultTransportForTests(transport);
    expect(await reportFrameStatements(handle("invented-frame-job-1", PRESSURE_CONTRACT_ID))).toEqual({ ok: true, statements: lines });
    expect(transport).toHaveBeenCalledWith(NATIVE_VERDICT_COMMAND, { jobId: "invented-frame-job-1", operation: "report_frame_statements", modelJson: "{\"invented\":\"model\"}" });
  });

  it("asks the native host for a pressure-free Timoshenko result in its custody, whose report carries a standing record", async () => {
    const transport = vi.fn(async (_command: string, args: Record<string, unknown>) => ({
      document_kind: "swbpipe.desktop.report_frame_statements", version: 1, job_id: args.jobId, frame_statements: []
    }));
    setNativeResultTransportForTests(transport);
    expect(await reportFrameStatements(handle("invented-frame-job-7", PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID))).toEqual({ ok: true, statements: [] });
    expect(transport).toHaveBeenCalledWith(NATIVE_VERDICT_COMMAND, { jobId: "invented-frame-job-7", operation: "report_frame_statements", modelJson: "{\"invented\":\"model\"}" });
    // Without native custody it is refused by name, as an exact-route result is.
    const held = await reportFrameStatements({ producer: { semantic_contract_id: PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID } });
    expect(held.ok === false && held.reason.startsWith(`${REPORT_FRAME_STATEMENTS_UNAVAILABLE}: `)).toBe(true);
  });

  it("gives none for a pressure-free Euler–Bernoulli result, and asks nothing", async () => {
    const transport = vi.fn();
    setNativeResultTransportForTests(transport);
    expect(await reportFrameStatements(handle("invented-frame-job-2", PREVIEW_PHYSICS_CONTRACT_ID))).toEqual({ ok: true, statements: [] });
    expect(await reportFrameStatements({ producer: { semantic_contract_id: PREVIEW_PHYSICS_CONTRACT_ID } })).toEqual({ ok: true, statements: [] });
    expect(transport).not.toHaveBeenCalled();
  });

  it("refuses by name where the statements cannot be obtained", async () => {
    setNativeResultTransportForTests(async () => { throw new Error(`${REPORT_FRAME_STATEMENTS_UNAVAILABLE}: invented reason`); });
    expect(await reportFrameStatements(handle("invented-frame-job-3", PRESSURE_CONTRACT_ID))).toEqual({ ok: false, reason: `${REPORT_FRAME_STATEMENTS_UNAVAILABLE}: invented reason` });
    setNativeResultTransportForTests(async () => { throw new Error("RESOURCE_UNAVAILABLE: invented"); });
    expect(await reportFrameStatements(handle("invented-frame-job-4", PRESSURE_CONTRACT_ID))).toEqual({ ok: false, reason: `${REPORT_FRAME_STATEMENTS_UNAVAILABLE}: RESOURCE_UNAVAILABLE: invented` });
    setNativeResultTransportForTests(async (_command, args) => ({ document_kind: "swbpipe.desktop.report_frame_statements", version: 1, job_id: `${String(args.jobId)}-other`, frame_statements: [] }));
    const other = await reportFrameStatements(handle("invented-frame-job-5", PRESSURE_CONTRACT_ID));
    expect(other.ok).toBe(false);
    // No native custody, or no native standing: nothing to read the statements from.
    const held = await reportFrameStatements({ producer: { semantic_contract_id: PRESSURE_CONTRACT_ID } });
    expect(held.ok === false && held.reason.startsWith(`${REPORT_FRAME_STATEMENTS_UNAVAILABLE}: `)).toBe(true);
    const unbound = await reportFrameStatements(handle("invented-frame-job-6", PRESSURE_CONTRACT_ID, null));
    expect(unbound.ok).toBe(false);
  });
});
