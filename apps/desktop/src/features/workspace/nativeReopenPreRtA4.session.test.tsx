// RT-A4 §10.4, C1's connecting path (DEL-04-05 `c1-native-custody-plan.md`, DEL-05-02
// `absolute-sum-srss-combinations.md` "Recorded results from before RT-A4"): main's
// preview-physics-2 run recorded before RT-A4, reopened through the actual native commands
// (`native_reopen_pre_rt_a4_replies.json`, written by stored_custody_tests.rs from a real store:
// the native open, then each request the historical panel sends on the restored handle) and shown
// by the historical panel exactly as the webview's own reader shows the whole run
// (`recordedPreRtA4Display.test.tsx`): as recorded under its table, no current-only verdict, never
// Current. With NATIVE_REOPEN_RECORD=1 the panel's requests are written to
// `native_reopen_pre_rt_a4_requests.json` for the Rust test to answer. Committed fixtures only.
import { cleanup, render, screen, waitFor, within } from "@testing-library/react";
import { isDeepStrictEqual } from "node:util";
import { readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
import { afterEach, describe, expect, it, vi } from "vitest";
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../../test/nativeIngress")).ingressDecodingInvoke(invokeMock, { nativeReopen: "actual" }) }));
import { decodeNativeProjectOpen } from "../../services/nativeReopen";
import { isNativeResult, nativeCustody } from "../../services/nativeResult";
import { buildNativeHistoricalRunContext, HistoricalRunPanel } from "../results/HistoricalRunContext";
import { displayRoute, readerDispatch } from "../results/resultReader";
import { numericalResultStanding } from "../results/resultStanding";
import { knownSemanticNotices, N_HEADLINE } from "../results/knownSemanticLimitations";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
const REPLIES = resolve(__dirname, "../../test-support/native-reopen/native_reopen_pre_rt_a4_replies.json");
const REQUESTS = resolve(__dirname, "../../test-support/native-reopen/native_reopen_pre_rt_a4_requests.json");
const CASES = resolve(__dirname, "../../test-support/native-reopen/historical_reopen_cases.json");
const RECORD = process.env.NATIVE_REOPEN_RECORD === "1";
const HANDLE = "restored-stored-result-replay-pre-rt-a4";
const TABLE = { id: "openpipestress.result_semantics/0.3.0/preview-physics-2", sha256: "1ccc9561cb4ce59d047f3dab7927a927c44a707efb3acb90e7b07a21b3bad0a2" };
/** What a current-only reader shows for a statement it refuses (`recordedPreRtA4Display.test.tsx`). */
const CURRENT_ONLY = ["integrity_failure", "Unsupported numerical contract", "SOURCE_SEMANTIC_CONTRACT_UNSUPPORTED", "SOURCE_PREVIEW_PHYSICS_FORMULATION_BASIS",
  "RESULT_READER_UNAVAILABLE", "JSON_PARSE_REJECTED", "ANALYSIS_SOURCE_CONTRACT_VERSION_MISMATCH", "ANALYSIS_SEMANTIC_CONTRACT_MISMATCH", "SOURCE_RECORDED_BINDING"];

afterEach(() => { cleanup(); invokeMock.mockReset(); delete (window as Json).__TAURI_INTERNALS__; });

function substitute(value: unknown, from: string, to: string): unknown {
  if (value === from) return to;
  if (Array.isArray(value)) return value.map((item) => substitute(item, from, to));
  if (value && typeof value === "object") return Object.fromEntries(Object.entries(value).map(([key, item]) => [key, substitute(item, from, to)]));
  return value;
}
function delivered(reply: Json): unknown {
  if (!reply.ok) throw reply.error;
  if (reply.body === "json") return structuredClone(reply.value);
  const bytes = Buffer.from(reply.base64, "base64");
  return bytes.buffer.slice(bytes.byteOffset, bytes.byteOffset + bytes.byteLength);
}

/** The native open's actual reply, and the handle's requests answered by their actual commands. */
async function reopened() {
  let document: Json = { open: null, requests: [] };
  try { document = JSON.parse(readFileSync(REPLIES, "utf8")); } catch { if (!RECORD) throw new Error("no pre-RT-A4 reopen replies"); }
  invokeMock.mockImplementation(async (command: string, args: Json) => {
    if (args?.jobId === HANDLE) {
      const request = substitute(args, HANDLE, "$HANDLE");
      const found = (document.requests as Json[]).find((entry) => entry.command === command && isDeepStrictEqual(entry.args, request));
      if (found) return found.reply.body === "json" ? substitute(delivered(found.reply), "$HANDLE", HANDLE) : delivered(found.reply);
      if (RECORD) {
        let kept: Json[] = [];
        try { kept = JSON.parse(readFileSync(REQUESTS, "utf8")).requests; } catch { kept = []; }
        if (!kept.some((entry) => entry.command === command && isDeepStrictEqual(entry.args, request))) kept.push({ command, args: request });
        writeFileSync(REQUESTS, `${JSON.stringify({ document_kind: "swbpipe.desktop.native_reopen_pre_rt_a4_requests", note: "Written by nativeReopenPreRtA4.session.test.tsx (NATIVE_REOPEN_RECORD=1): the requests the historical panel sends on the restored pre-RT-A4 handle; stored_custody_tests.rs answers each with its actual command.", requests: kept }, null, 1)}\n`);
      }
      throw new Error(`NATIVE_REOPEN_REPLAY: ${command} ${JSON.stringify(request)} has no recorded reply (NATIVE_REOPEN_RECORD=1, then NATIVE_REOPEN_REPLIES_WRITE=1 in stored_custody_tests)`);
    }
    throw new Error(`NATIVE_REOPEN_REPLAY: unexpected ${command}`);
  });
  (window as Json).__TAURI_INTERNALS__ = {};
  const opened = decodeNativeProjectOpen(substitute(document.open.reply.value, "$HANDLE", HANDLE));
  const context = (await buildNativeHistoricalRunContext(opened!))!;
  return { document, context };
}

describe("a run recorded before RT-A4 reopens through the native commands (C1, RT-A4 §10.4)", () => {
  it("is held natively, read under its recorded table, shown as recorded, and never Current", async () => {
    const { context } = await reopened();
    const expected = (JSON.parse(readFileSync(CASES, "utf8")).cases as Json[]).find((c) => c.id === "saved-pre-rt-a4:preview-physics-2").expected;
    // The native reopen's findings are the webview's own reader's over the whole run, in their scopes.
    expect(context.findings).toEqual(expected.findings);
    expect(context.checkFindings).toEqual(expected.check_findings);
    expect(context.recordedTableRead).toEqual(TABLE);
    expect(context.historicalReaderFindings).toEqual(expected.historical_reader_findings);
    const result = context.mechanicsResult!;
    expect(isNativeResult(result)).toBe(true);
    const custody = nativeCustody(result as never);
    expect(custody.origin).toBe("restored_saved_run");
    expect(custody.recordedTable).toEqual(TABLE);
    // Classified under its table for display; the current reader still refuses it, and it has no standing.
    expect(displayRoute(result, TABLE)).toBe("preview_physics");
    expect(displayRoute(result)).toBe(readerDispatch(result).route);
    expect(readerDispatch(result).binding?.sha256).not.toBe(TABLE.sha256);
    expect(readerDispatch(result).standing.findings).toContain("SOURCE_PREVIEW_PHYSICS_FORMULATION_BASIS");
    expect(numericalResultStanding(result).eligible).toBe(false);
    render(<HistoricalRunPanel context={context} />);
    const standing = await screen.findByTestId("numerical-result-standing");
    expect(standing.textContent!.startsWith(`As recorded under ${TABLE.id} (sha256 ${TABLE.sha256}); historical evidence only, not a current standing. Numerical integrity: `)).toBe(true);
    const panel = screen.getByTestId("historical-run-context");
    await waitFor(() => expect(within(panel).getAllByTestId(/^result-row-/).length).toBeGreaterThan(0));
    await waitFor(() => expect(panel.textContent).not.toContain("Reading"));
    for (const marker of CURRENT_ONLY) expect(panel.textContent!.includes(marker), marker).toBe(false);
    // Review F2: the run shows its own numerical health, read natively with its recorded standing's findings.
    expect(within(panel).getByTestId("numerical-health-header-line").textContent).toBe("Numerical health: checks passed");
    // The notices are the table's, exactly as the webview's reader gives them for the whole recorded
    // statement under it: its stress headline (it has no intensified row and no combination gate).
    const whole = JSON.parse(readFileSync(resolve(__dirname, "../../../../../fixtures/persistence/recorded_pre_rt_a4_pf/preview_physics_2/preview_physics_connected_sparse.json"), "utf8"));
    const expectedNotices = [{ id: "headline-label", text: `Stress headline: ${N_HEADLINE}.` }];
    expect(knownSemanticNotices(whole, TABLE)).toEqual(expectedNotices);
    expect(knownSemanticNotices(result, TABLE, { hasIntensified: false })).toEqual(expectedNotices);
    await waitFor(() => expect([...panel.querySelectorAll("[data-testid^='results-notice-']")].map((n) => ({ id: n.getAttribute("data-testid")!.slice("results-notice-".length), text: n.textContent })))
      .toEqual(expectedNotices));
  });

  it("refuses any other table and never falls back to the current reader", async () => {
    const { context } = await reopened();
    const result = context.mechanicsResult!;
    for (const other of [{ ...TABLE, sha256: "0".repeat(64) }, { id: "openpipestress.result_semantics/0.3.0/preview-physics-timoshenko-1", sha256: "a7b83c37a4af9bf70f56513f80883003a2a3860387319311ea2e967dd0cf764b" }]) {
      expect(displayRoute(result, other)).toBe("unsupported");
      expect(knownSemanticNotices(result, other)).toEqual([]);
    }
    render(<HistoricalRunPanel context={{ ...context, recordedTableRead: { ...TABLE, sha256: "0".repeat(64) } }} />);
    const standing = await screen.findByTestId("numerical-result-standing");
    expect(standing.textContent).toContain("Unsupported numerical contract");
    await waitFor(() => expect(screen.getByTestId("historical-run-context").textContent).toContain("integrity_failure"));
  });
});
