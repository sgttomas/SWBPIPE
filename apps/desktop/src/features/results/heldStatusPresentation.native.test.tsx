// RT-A5 `minuend_final` held-status presentation (DEL-05-02 `operating-case.md` §7.5; Work
// `combination-certificate.md` §2) on a C1 native handle (DEL-04-05 `c1-native-custody-plan.md`,
// "Native diagnostics presentation"): a diagnostic whose typed code is a quoted held-status code
// (`HELD_STATUS_QUOTED_CODES`: the five held gates and HELD_STATUS_RELEASED) is quoted whole after
// the supplemental label in the Diagnostics list and explanation, the Design Knowledge records and the
// missing-data warnings, even where the native `roster_checked` fact Rust read of the held bytes is
// true; a claim-like held refusal is quoted and changes no standing; and every presentation equals the
// whole value's on the same bytes, without reading a guarded member. The held bytes are the #122
// reader corpus (`preview_physics_held_status_corpus.json`, the producer's output), as
// `heldStatusPresentation.test.tsx` reads it; every native reply is the actual Rust command's recorded
// reply (`nativeReplayStore.ts`) and no fact is authored here. Unit transport replay; NOT native UI
// qualification.
import { cleanup, render, screen, waitFor } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it } from "vitest";
import heldCorpus from "../../../../../fixtures/results/preview_physics_held_status_corpus.json";
import demoModel from "../../../../../fixtures/product_preview/invented_demo_model.json";
import {
  nativeCustody,
  nativeDiagnosticsPresentation,
  NATIVE_GUARDED_MEMBERS,
  resetNativeResultsForTests,
  setNativeResultTransportForTests,
  type NativeResult
} from "../../services/nativeResult";
import { restoredNativeResult, type RestoredRun } from "../../services/nativeReopen";
import { resetSessionResultCustodyForTests } from "../../services/previewService";
import { nativeReplayReply } from "../../test/nativeReplayStore";
import { receivedFromText } from "../../test-support/receivedText";
import { DiagnosticsPanel } from "../diagnostics/DiagnosticsPanel";
import { KnowledgePanel, physicsRecords } from "../knowledge/KnowledgePanel";
import { buildMissingDataBlockingPacket } from "../missing-data/MissingDataBlockingPanel";
import { heldStatusStandings, knownSemanticNotices } from "./knownSemanticLimitations";
import { buildDiagnosticInterpretation } from "./resultInterpretation";
import {
  documentCombinationIdentities,
  HELD_STATUS_QUOTED_CODES,
  presentedResultDiagnostics,
  rosterCheckedByReader,
  SUPPLEMENTAL_DIAGNOSTIC_LABEL
} from "./suppliedDiagnostics";
import type { Diagnostic, MechanicsResult, PreviewModel } from "../../types";

type Entry = { name: string; document: MechanicsResult };
const GATE = "diagnostic:preview-physics:combination-gate:13:comb:o-s-held";
const RELEASED = "diagnostic:preview-physics:held-status-released:13:comb:o-s-held";
const GATES = ["HELD_STATUS_SOURCE_UNAVAILABLE", "HELD_STATUS_SOURCE_UNQUALIFIED", "HELD_STATUS_BINDING", "HELD_STATUS_SLIDING_GROUND_DIFFERS", "HELD_STATUS_SOLVE_FAILED"];
const supplied = (message: string) => `${SUPPLEMENTAL_DIAGNOSTIC_LABEL}"${message}"`;
// Claim-like refusals (`heldStatusPresentation.test.tsx`'s): "established", a tiny bound, Unicode.
const CLAIMS = [
  "its rows are established within their stated bound",
  "every row is bounded by 1e-300 within its stated bound",
  "ESTABLISHED Within Its Stated BOUND",
  "les lignes sont établies — within their stated bound ≤ 2⁻¹⁰⁰⁰ ✓"
];

function document(name = "source_unavailable"): MechanicsResult {
  return structuredClone((heldCorpus as unknown as { documents: Entry[] }).documents.find((e) => e.name === name)!.document);
}
function notice(result: { diagnostics: Diagnostic[] }, id: string): Diagnostic {
  return result.diagnostics.find((d) => d.id === id)!;
}
type HeldRecord = { standing: { kind: string; reason: string } };
function heldRecord(result: MechanicsResult): HeldRecord {
  return (result.contract_evidence as unknown as Record<string, HeldRecord[]>).held_status_derivation[0];
}
/** The corpus document with its held standing's refusal and its gate notice's message a claim. */
function claimed(claim: string): MechanicsResult {
  const result = document();
  heldRecord(result).standing.reason = claim;
  notice(result, GATE).message = `Combination withheld: ${claim}.`;
  return result;
}
/** The corpus gate notice re-coded (as `heldStatusPresentation.test.tsx` does): the rule reads the code. */
function recoded(code: string, index: number): MechanicsResult {
  const result = document();
  const gate = notice(result, GATE);
  gate.code = code;
  gate.message = `Combination withheld: ${CLAIMS[index % CLAIMS.length]}.`;
  return result;
}

/** Restored handles answered by the recorded Rust replies on their held text. */
const held = new Map<string, string>();
let sequence = 0;
async function restoredHandle(text: string): Promise<NativeResult> {
  const handle = `restored-stored-result-held-status-${++sequence}`;
  held.set(handle, text);
  const run = { handle, origin: "restored_saved_run", result_bytes: new TextEncoder().encode(text).length, findings: { recorded_table_read: null }, recorded_dispatch: null } as unknown as RestoredRun;
  return restoredNativeResult(run);
}
/** A native handle and the whole value of the same bytes. */
async function both(result: MechanicsResult): Promise<{ handle: NativeResult; whole: MechanicsResult; text: string }> {
  const text = JSON.stringify(result);
  return { handle: await restoredHandle(text), whole: receivedFromText<MechanicsResult>(text), text };
}

/** Every property read of a native handle (a guarded member throws when read; a read inside a
 * swallowing try would not surface, so the reads are recorded). */
function watched(handle: NativeResult): { proxy: NativeResult; reads: PropertyKey[] } {
  const reads: PropertyKey[] = [];
  const proxy = new Proxy(handle, { get(target, key) { reads.push(key); return Reflect.get(target, key, target); } });
  return { proxy, reads };
}
const guardedReads = (reads: PropertyKey[]) => reads.filter((key) => (NATIVE_GUARDED_MEMBERS as readonly PropertyKey[]).includes(key) || key === "toJSON");

function panelModel(result: { model_ref?: unknown }): PreviewModel {
  return { diagnostics: [], project: { id: result.model_ref }, rule_pack_refs: [], library_refs: [] } as unknown as PreviewModel;
}

type Presented = { listed: (string | null)[]; explanation: string; interpretation: string; record: string; knowledge: string | null };
/** Every presenting path's message for the diagnostic `id`: the Diagnostics list entry and
 * explanation, the interpretation, the Design Knowledge record and panel paragraph. A native
 * explanation's linked-row sentence comes from an asynchronous row lookup (the replayed Rust page), so
 * a native reading waits for the whole value's explanation (`settled`) before it is taken. */
async function presented(result: MechanicsResult | NativeResult, id: string, model: PreviewModel, settled?: string): Promise<Presented> {
  const raw = notice(result, id);
  let view = render(<DiagnosticsPanel model={model} knowledge={null} result={result} selectedDiagnosticId={id} onSelectDiagnostic={() => {}} />);
  const listed = screen.getAllByTestId(`diagnostic-${raw.code}`).map((item) => item.querySelector("p")!.textContent);
  const explained = () => screen.getByTestId("selected-diagnostic-explanation").querySelector("dd")!.textContent!;
  if (settled !== undefined) await waitFor(() => expect(explained()).toBe(settled));
  const explanation = explained();
  view.unmount();
  const interpretation = buildDiagnosticInterpretation({ model, knowledge: null, result, diagnosticId: id })!.message;
  const record = physicsRecords(result).find((r) => r.id === `knowledge:${id}`)!;
  view = render(<KnowledgePanel knowledge={null} result={result} />);
  const knowledge = screen.getByTestId(`knowledge-record-${record.id}`).querySelector("p")!.textContent;
  view.unmount();
  return { listed, explanation, interpretation, record: record.summary, knowledge };
}

/** The missing-data packet's warnings with a nonlinear warning appended to the bytes (the only kind
 * whose message a warning copies, through its presented form). */
function withNonlinearWarning(result: MechanicsResult, message: string): MechanicsResult {
  const changed = structuredClone(result);
  changed.diagnostics.push({ id: "diagnostic:nonlinear:invented-review", code: "NONLINEAR_SUPPORT_NONCONVERGENCE", severity: "warning", message, affected_refs: ["case:O"] } as Diagnostic);
  return changed;
}

beforeEach(() => {
  resetSessionResultCustodyForTests();
  resetNativeResultsForTests();
  held.clear();
  setNativeResultTransportForTests(async (command, args) => {
    const text = held.get(args.jobId as string);
    if (text === undefined) throw new Error(`UNEXPECTED_JOB: ${String(args.jobId)}`);
    return nativeReplayReply(command, args, args.jobId as string, text, null);
  });
});
afterEach(() => {
  cleanup();
  setNativeResultTransportForTests(null);
});

describe("a native handle quotes its held-status notices as the whole value does", () => {
  // The corpus' own held notices: its two gates and HELD_STATUS_RELEASED, each on bytes the reader admits.
  // (Each held text is kept whole in the replay corpus, so one released document stands for the four.)
  const NATURAL: [string, string, string][] = [
    ["source_unavailable", GATE, "HELD_STATUS_SOURCE_UNAVAILABLE"],
    ["sliding_ground_differs", GATE, "HELD_STATUS_SLIDING_GROUND_DIFFERS"],
    ["h2_released", RELEASED, "HELD_STATUS_RELEASED"]
  ];
  it.each(NATURAL)("%s: quoted on every path though the native roster_checked fact is true", async (name, id, code) => {
    const { handle, whole } = await both(document(name));
    const facts = nativeDiagnosticsPresentation(handle)!;
    expect(facts).not.toBeNull();
    // The facts are Rust's on the held bytes, each the whole-value helper's: the roster admitted these bytes.
    expect(facts.rosterChecked).toBe(true);
    expect(rosterCheckedByReader(whole)).toBe(true);
    expect([...facts.combinations].sort()).toEqual([...documentCombinationIdentities(whole)].sort());
    const raw = notice(whole, id);
    expect(raw.code).toBe(code);
    expect(notice(handle, id).message).toBe(raw.message);
    const model = panelModel(whole);
    const wholePresented = await presented(whole, id, model);
    const { proxy, reads } = watched(handle);
    const native = await presented(proxy, id, model, wholePresented.explanation);
    expect(guardedReads(reads)).toEqual([]);
    expect(native.listed).toEqual([supplied(raw.message)]);
    expect(native.explanation.startsWith(supplied(raw.message))).toBe(true);
    for (const message of [native.interpretation, native.record, native.knowledge]) expect(message).toBe(supplied(raw.message));
    // The whole value's presentation on the same bytes.
    expect(native).toEqual(wholePresented);
    // A roster entry naming the held combination stays copied on both (the held notice alone is quoted).
    const roster = presentedResultDiagnostics(handle).filter((d) => !HELD_STATUS_QUOTED_CODES.has(d.code) && d.affected_refs?.includes("comb:o-s-held"));
    expect(roster.length).toBeGreaterThan(0);
    for (const d of roster) expect(d.message).toBe(notice(whole, d.id!).message);
    expect(presentedResultDiagnostics(handle)).toEqual(presentedResultDiagnostics(whole));
  });

  it("quotes each of the six held-status codes (the gates re-coded on the corpus notice) as the whole value, by the typed code", async () => {
    const seen = new Set<string>();
    const cases = [...GATES.map((code, index) => ({ code, result: recoded(code, index), id: GATE })), { code: "HELD_STATUS_RELEASED", result: document("h2_released"), id: RELEASED }];
    for (const { code, result, id } of cases) {
      const { handle, whole } = await both(result);
      const facts = nativeDiagnosticsPresentation(handle)!;
      expect(facts, code).not.toBeNull();
      expect(facts.rosterChecked, code).toBe(rosterCheckedByReader(whole));
      const raw = notice(whole, id).message;
      const model = panelModel(whole);
      const wholePresented = await presented(whole, id, model);
      const { proxy, reads } = watched(handle);
      const native = await presented(proxy, id, model, wholePresented.explanation);
      expect(guardedReads(reads), code).toEqual([]);
      expect(native.listed, code).toEqual([supplied(raw)]);
      for (const message of [native.interpretation, native.record, native.knowledge]) expect(message, code).toBe(supplied(raw));
      expect(native, code).toEqual(wholePresented);
      // Every computed diagnostic record states the Diagnostics panel's presented message, as the whole value's.
      const records = (result: MechanicsResult | NativeResult) => physicsRecords(result).filter((r) => r.id.startsWith("knowledge:diagnostic:")).map((r) => [r.id, r.summary]);
      expect(records(handle), code).toEqual(records(whole));
      seen.add(code);
    }
    expect([...seen].sort()).toEqual([...HELD_STATUS_QUOTED_CODES].sort());
  });

  it("states a nonlinear warning's message in the missing-data warnings as the whole value presents it", async () => {
    const model = demoModel as unknown as PreviewModel;
    for (const name of ["source_unavailable"]) {
      const { handle, whole } = await both(withNonlinearWarning(document(name), CLAIMS[1]));
      const { proxy, reads } = watched(handle);
      const native = buildMissingDataBlockingPacket({ model, result: proxy });
      expect(guardedReads(reads), name).toEqual([]);
      const wholePacket = buildMissingDataBlockingPacket({ model, result: whole });
      expect(native.warnings, name).toEqual(wholePacket.warnings);
      const warning = native.warnings.find((w) => w.warning_id === "nonlinear:diagnostic:nonlinear:invented-review")!;
      expect(warning.message, name).toBe(presentedResultDiagnostics(whole).find((d) => d.code === "NONLINEAR_SUPPORT_NONCONVERGENCE")!.message);
      // No held notice's raw text reaches the packet except quoted after the label.
      for (const d of whole.diagnostics.filter((d) => HELD_STATUS_QUOTED_CODES.has(d.code))) {
        expect(JSON.stringify(native).split(JSON.stringify(supplied(d.message)).slice(1, -1)).join("").includes(JSON.stringify(d.message).slice(1, -1)), name).toBe(false);
      }
    }
  });
});

describe("a claim-like held refusal on a native handle is quoted and changes no standing", () => {
  it.each(CLAIMS)("%s", async (claim) => {
    const benign = await both(document());
    const { handle, whole } = await both(claimed(claim));
    const facts = nativeDiagnosticsPresentation(handle)!;
    expect(facts).not.toBeNull();
    // The reader binds R to its record and admits the bytes; the native fact says so, as the whole value's helper.
    expect(facts.rosterChecked).toBe(true);
    expect(rosterCheckedByReader(whole)).toBe(true);
    const raw = notice(whole, GATE).message;
    const model = panelModel(whole);
    const wholePresented = await presented(whole, GATE, model);
    const { proxy, reads } = watched(handle);
    const native = await presented(proxy, GATE, model, wholePresented.explanation);
    expect(native.listed).toEqual([supplied(raw)]);
    for (const message of [native.interpretation, native.record, native.knowledge]) expect(message).toBe(supplied(raw));
    // heldStatusStandings reads contract_evidence only (in S's header), never a guarded member. The
    // withheld record (outcome HELD_STATUS_SOURCE_UNAVAILABLE) states no held standing: the claimed
    // refusal R is never presented as one.
    const standings = heldStatusStandings(proxy);
    expect(guardedReads(reads)).toEqual([]);
    expect(native).toEqual(wholePresented);
    expect(standings).toEqual(heldStatusStandings(whole));
    expect(standings).toEqual([]);
    const heldNotices = (source: MechanicsResult | NativeResult) => knownSemanticNotices(source, null, { hasIntensified: false }).filter((n) => n.id.startsWith("held-status:"));
    expect(heldNotices(handle)).toEqual(heldNotices(whole));
    // Nothing else moves: the other presented diagnostics, the held standing's kind, the native standing,
    // the numerical quality and the held combination's certificate record are the benign bytes'.
    expect(presentedResultDiagnostics(handle).filter((d) => d.id !== GATE)).toEqual(presentedResultDiagnostics(benign.handle).filter((d) => d.id !== GATE));
    expect(standings.map((s) => [s.combination_id, s.kind])).toEqual(heldStatusStandings(benign.handle).map((s) => [s.combination_id, s.kind]));
    expect(nativeCustody(handle).verdict.standing).toEqual(nativeCustody(benign.handle).verdict.standing);
    expect([nativeCustody(handle).verdict.ok, nativeCustody(handle).verdict.route]).toEqual([nativeCustody(benign.handle).verdict.ok, nativeCustody(benign.handle).verdict.route]);
    expect(handle.numerical_quality).toEqual(benign.handle.numerical_quality);
    const certificate = (source: MechanicsResult | NativeResult) => (source.contract_evidence as unknown as Record<string, { combinations: { combination_id: string; assessment: string; groups: unknown[] }[] }>)
      .combination_certificate.combinations.find((r) => r.combination_id === "comb:o-s-held")!;
    expect([certificate(handle).assessment, certificate(handle).groups]).toEqual(["not_assessed", []]);
    expect(certificate(handle)).toEqual(certificate(benign.handle));
  });
});

describe("held-status standings on a native handle", () => {
  it("are the whole value's on the same bytes: an unavailable source, a differing sliding ground, a release", async () => {
    for (const name of ["source_unavailable", "sliding_ground_differs", "h2_released"]) {
      const { handle, whole } = await both(document(name));
      // A withheld record states no held standing; the admitted release states its own.
      expect(heldStatusStandings(whole).length, name).toBe(name === "h2_released" ? 1 : 0);
      const { proxy, reads } = watched(handle);
      expect(heldStatusStandings(proxy), name).toEqual(heldStatusStandings(whole));
      expect(guardedReads(reads), name).toEqual([]);
    }
  });
});
