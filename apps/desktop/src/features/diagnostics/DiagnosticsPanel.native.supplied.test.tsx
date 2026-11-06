// C1 native custody (DEL-04-05 `c1-native-custody-plan.md`, "Native diagnostics presentation") with
// RT-A3 §4 (DEL-05-02 Work `combination-certificate.md` §2): the Diagnostics list and explanation of a
// native handle are the whole value's, classified by the same code with the facts Rust read of the
// held bytes and delivered in S (`diagnostics_presentation`). Every native reply here is the actual
// Rust command's recorded reply (`nativeReplayStore.ts`); no fact is authored here. Facts that are
// missing, malformed, another snapshot's or a disposed handle's attest nothing, and the guarded row
// members are never read. Unit transport replay; NOT native UI qualification. Committed fixtures and
// invented notes only.
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { cleanup, render, screen } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../../test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));

import corpus from "../../../../../fixtures/results/combination_certificate_corpus.json";
import successor from "../../../../../fixtures/results/retained_precision_combination_successor_sparse_interactive.json";
import legacySuccessor from "../../../../../fixtures/results/retained_precision_combination_successor_sparse_interactive_pre_certificate.json";
import {
  createNativeResult,
  forgetNativeJob,
  nativeCustody,
  codePointLess,
  nativeDiagnosticsPresentation,
  NATIVE_GUARDED_MEMBERS,
  NativeResultRowsGuarded,
  resetNativeResultsForTests,
  setNativeResultTransportForTests,
  type NativeResult
} from "../../services/nativeResult";
import { restoredNativeResult, type RestoredRun } from "../../services/nativeReopen";
import { awaitPreviewMechanicsJob, disposeDeliveredMechanicsResult, readNativeFreshResult, resetSessionResultCustodyForTests, startPreviewMechanicsJob } from "../../services/previewService";
import { nativeReplayReply } from "../../test/nativeReplayStore";
import { createSolveJobTransport, REPLAY_COMMANDS } from "../../test/nativeMechanicsReplay";
import { receivedFromText } from "../../test-support/receivedText";
import { registerRetainedPrecision } from "../results/retainedPrecisionStanding";
import { certificateAttestedByRetainedReader, documentCombinationIdentities, presentedResultDiagnostics, rosterCheckedByReader, SUPPLEMENTAL_DIAGNOSTIC_LABEL } from "../results/suppliedDiagnostics";
import { buildDiagnosticInterpretation } from "../results/resultInterpretation";
import { diagnosticsForSections } from "../report/renderableReportInput";
import { DiagnosticsPanel } from "./DiagnosticsPanel";
import type { Diagnostic, MechanicsResult, PreviewModel } from "../../types";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
type Entry = { name: string; document: MechanicsResult; model: PreviewModel };
const ROOT = resolve(__dirname, "../../../../../");
const documentOf = (name: string): Entry => structuredClone((corpus as unknown as { documents: Entry[] }).documents.find((e) => e.name === name)!);
const supplied = (message: string) => `${SUPPLEMENTAL_DIAGNOSTIC_LABEL}"${message}"`;
const CLAIM = "combination combination:all: all derived rows are bounded by 1e-30.";
function note(refs: string[], id: string, message = CLAIM): Diagnostic {
  return { id, code: "REVIEW_NOTE", severity: "info", source: "core/product_physics", affected_refs: refs, message } as Diagnostic;
}
function withNote(entry: Entry, diagnostic: Diagnostic): string {
  const document = structuredClone(entry.document);
  document.diagnostics.push(diagnostic);
  return JSON.stringify(document);
}
const firstCombination = (text: string) => [...documentCombinationIdentities(JSON.parse(text))].sort()[0];

/** The held bytes of each case (texts as the native host would hold them). */
const CANONICAL = JSON.stringify(documentOf("pf_held").document);
const SHARED_ID = JSON.stringify(documentOf("pf_shared_id").document);
const TAMPERED = withNote(documentOf("pf_held"), note([firstCombination(CANONICAL)], "diagnostic:review:claim"));
const SHARED_ID_TAMPERED = withNote(documentOf("pf_shared_id"), note([firstCombination(SHARED_ID)], "diagnostic:review:shared"));
const PROSE = withNote(documentOf("pf_held"), note(["model"], "diagnostic:review:prose", "Supplied prose naming no combination."));
// Review of #143: a canonical source whose combination identities differ at one character, U+E000
// against U+10000 (both admitted model identities), where Rust's order (code points, its UTF-8 bytes)
// and JavaScript's UTF-16 `<` disagree.
const UNICODE = CANONICAL.replaceAll("comb:double", "comb:\uE000").replaceAll("comb:own", "comb:\u{10000}");
const RETAINED = JSON.stringify((successor as Json).source);
const RETAINED_LEGACY = JSON.stringify((legacySuccessor as Json).source);
const NO_COMBINATIONS = readFileSync(resolve(ROOT, "fixtures/results/preview_physics_connected_sparse.json"), "utf8");
const TENTH = JSON.stringify(JSON.parse(readFileSync(resolve(ROOT, "fixtures/persistence/recorded_pre_rt_a4_pf/physics_retained_1/main_252bef969/retained_precision_exact_successor_sparse_interactive.json"), "utf8")).source);
const TENTH_TABLE = { id: "openpipestress.result_semantics/0.3.0/physics-retained-1", sha256: "85c2678a78e794b44761c11efb73c215645c7e2560d802ac8ad6075e7804c691" };

/** Restored handles answered by the recorded Rust replies on their held text (and table). */
const held = new Map<string, { text: string; table: { id: string; sha256: string } | null }>();
let sequence = 0;
async function restoredHandle(text: string, table: { id: string; sha256: string } | null = null): Promise<NativeResult> {
  const handle = `restored-stored-result-diagnostics-${++sequence}`;
  held.set(handle, { text, table });
  const run = { handle, origin: "restored_saved_run", result_bytes: new TextEncoder().encode(text).length, findings: { recorded_table_read: table }, recorded_dispatch: null } as unknown as RestoredRun;
  return restoredNativeResult(run);
}

/** Every property read of a native handle (the guarded members throw when read; a read inside a
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
/** The panel's list entry and explanation message for `selected`. */
function shown(result: MechanicsResult | NativeResult, selected: Diagnostic) {
  const view = render(<DiagnosticsPanel model={panelModel(result)} knowledge={null} result={result} selectedDiagnosticId={selected.id!} onSelectDiagnostic={() => {}} />);
  const listed = screen.getAllByTestId(`diagnostic-${selected.code}`).map((item) => item.querySelector("p")!.textContent);
  const explanation = screen.getByTestId("selected-diagnostic-explanation").querySelector("dd")!.textContent!;
  view.unmount();
  return { listed, explanation };
}

beforeEach(() => {
  resetSessionResultCustodyForTests();
  resetNativeResultsForTests();
  held.clear();
  setNativeResultTransportForTests(async (command, args) => {
    const job = held.get(args.jobId as string);
    if (!job) throw new Error(`UNEXPECTED_JOB: ${String(args.jobId)}`);
    return nativeReplayReply(command, args, args.jobId as string, job.text, job.table);
  });
});
afterEach(() => {
  cleanup();
  setNativeResultTransportForTests(null);
  invokeMock.mockReset();
  delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
});

describe("a native handle's Diagnostics are the whole value's", () => {
  const CASES: [string, string, { roster: boolean; quoted: "all" | "some" | "none" }][] = [
    ["current accepted canonical certificate source", CANONICAL, { roster: true, quoted: "some" }],
    ["shared-ID certificate source", SHARED_ID, { roster: true, quoted: "some" }],
    ["tampered: a supplied claim naming a combination", TAMPERED, { roster: false, quoted: "all" }],
    ["tampered shared-ID source", SHARED_ID_TAMPERED, { roster: false, quoted: "all" }],
    ["supplemental prose naming no combination", PROSE, { roster: true, quoted: "some" }],
    ["unregistered retained successor with the certificate member", RETAINED, { roster: false, quoted: "all" }],
    ["unregistered retained successor without the member", RETAINED_LEGACY, { roster: false, quoted: "all" }],
    ["a result without combinations", NO_COMBINATIONS, { roster: false, quoted: "none" }]
  ];
  it.each(CASES)("%s", async (_name, text, expected) => {
    const whole = receivedFromText<MechanicsResult>(text);
    const handle = await restoredHandle(text);
    const facts = nativeDiagnosticsPresentation(handle)!;
    expect(facts).not.toBeNull();
    // Each fact is its whole-value helper on the same bytes.
    expect([...facts.combinations].sort()).toEqual([...documentCombinationIdentities(whole)].sort());
    if (facts.combinations.size > 0) expect(facts.rosterChecked).toBe(rosterCheckedByReader(whole));
    expect(facts.rosterChecked).toBe(expected.roster);
    expect(facts.certificateAttested).toBe(certificateAttestedByRetainedReader(whole));
    // The list, by the same classifier, without reading a guarded member.
    const { proxy, reads } = watched(handle);
    const native = presentedResultDiagnostics(proxy);
    expect(guardedReads(reads)).toEqual([]);
    const presented = presentedResultDiagnostics(whole);
    expect(native).toEqual(presented);
    const quoted = presented.filter((d, i) => d.message !== whole.diagnostics[i].message).length;
    expect(quoted === 0 ? "none" : quoted === presented.length ? "all" : "some").toBe(expected.quoted);
    if (facts.combinations.size === 0) expect(native).toBe(handle.diagnostics);
    // The list entry and the explanation of the last diagnostic (the appended note where there is one).
    const selected = whole.diagnostics.at(-1)!;
    expect(shown(handle, selected)).toEqual(shown(whole, selected));
    const interpretation = (result: MechanicsResult | NativeResult) => buildDiagnosticInterpretation({ model: panelModel(whole), knowledge: null, result, diagnosticId: selected.id! })!.message;
    expect(interpretation(handle)).toBe(interpretation(whole));
  });

  it("decodes a population ordered by code point (Rust's order), where UTF-16 order disagrees, and keeps list and explanation parity", async () => {
    const whole = receivedFromText<MechanicsResult>(UNICODE);
    const ids = [...documentCombinationIdentities(whole)];
    const byCodePoint = [...ids].sort((a, b) => (codePointLess(a, b) ? -1 : codePointLess(b, a) ? 1 : 0));
    // The case discriminates: UTF-16 order is not code point order on these identities.
    expect([...ids].sort()).not.toEqual(byCodePoint);
    expect(ids.some((id) => id.includes("\uE000")) && ids.some((id) => id.includes("\u{10000}"))).toBe(true);
    const handle = await restoredHandle(UNICODE);
    const facts = nativeDiagnosticsPresentation(handle)!;
    expect(facts).not.toBeNull();
    expect([...facts.combinations]).toEqual(byCodePoint);
    expect(facts.rosterChecked).toBe(rosterCheckedByReader(whole));
    const { proxy, reads } = watched(handle);
    expect(presentedResultDiagnostics(proxy)).toEqual(presentedResultDiagnostics(whole));
    expect(guardedReads(reads)).toEqual([]);
    // Every diagnostic's list entry and classified explanation message; the last one's whole explanation
    // (as the cases above: a native explanation's linked-row sentence comes from an asynchronous row lookup).
    for (const selected of whole.diagnostics) {
      expect(shown(handle, selected).listed).toEqual(shown(whole, selected).listed);
      const interpretation = (result: MechanicsResult | NativeResult) => buildDiagnosticInterpretation({ model: panelModel(whole), knowledge: null, result, diagnosticId: selected.id! })!.message;
      expect(interpretation(handle)).toBe(interpretation(whole));
    }
    const last = whole.diagnostics.at(-1)!;
    expect(shown(handle, last)).toEqual(shown(whole, last));
    // The comparator is strict: equal identities are not less (so the decoder refuses a duplicate).
    expect(codePointLess(ids[0], ids[0])).toBe(false);
  });

  it("copies a registered retained successor's certificate notices only on the whole value (no native registration exists)", async () => {
    const doc = structuredClone(successor) as unknown as { source: MechanicsResult; invocation: unknown };
    const handle = await restoredHandle(RETAINED);
    const unregistered = presentedResultDiagnostics(receivedFromText<MechanicsResult>(RETAINED));
    await registerRetainedPrecision(doc.source, doc.invocation);
    const registered = presentedResultDiagnostics(doc.source);
    expect(registered).not.toEqual(unregistered);
    expect(registered.filter((d) => d.id?.startsWith("diagnostic:combination-certificate:")).every((d) => !d.message.startsWith(SUPPLEMENTAL_DIAGNOSTIC_LABEL))).toBe(true);
    // The native snapshot holds no retained registration: it reads as the unregistered value, its
    // certificate notices quoted with every other message.
    expect(nativeDiagnosticsPresentation(handle)!.certificateAttested).toBe(false);
    expect(presentedResultDiagnostics(handle)).toEqual(unregistered);
  });

  it("reads a registered retained successor without the certificate member as the native handle does (nothing to attest)", async () => {
    const doc = structuredClone(legacySuccessor) as unknown as { source: MechanicsResult; invocation: unknown };
    const handle = await restoredHandle(RETAINED_LEGACY);
    await registerRetainedPrecision(doc.source, doc.invocation);
    // Registered, but without the member: the whole value attests no certificate notice either.
    expect(certificateAttestedByRetainedReader(doc.source)).toBe(false);
    expect(nativeDiagnosticsPresentation(handle)!.certificateAttested).toBe(false);
    expect(presentedResultDiagnostics(handle)).toEqual(presentedResultDiagnostics(doc.source));
  });

  it("reads a historical run restored under its recorded table (the tenth registry variant) as the whole value", async () => {
    const handle = await restoredHandle(TENTH, TENTH_TABLE);
    expect(nativeCustody(handle).recordedTable).toEqual(TENTH_TABLE);
    const facts = nativeDiagnosticsPresentation(handle)!;
    expect(facts).not.toBeNull();
    expect(facts.rosterChecked).toBe(false);
    expect(facts.certificateAttested).toBe(false);
    const whole = receivedFromText<MechanicsResult>(TENTH);
    expect([...facts.combinations].sort()).toEqual([...documentCombinationIdentities(whole)].sort());
    expect(presentedResultDiagnostics(handle)).toEqual(presentedResultDiagnostics(whole));
  });

  it("reads a fresh solve's handle (registered with its model) by the source-only facts", async () => {
    const entry = documentOf("pf_held");
    const tampered = JSON.parse(TAMPERED) as MechanicsResult;
    const transport = createSolveJobTransport(() => TAMPERED);
    invokeMock.mockImplementation((command: string, args?: unknown, options?: unknown) => {
      if (REPLAY_COMMANDS.includes(command)) return transport.invoke(command, args, options);
      return Promise.reject(new Error(`UNEXPECTED_COMMAND: ${command}`));
    });
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
    setNativeResultTransportForTests(null);
    const started = await startPreviewMechanicsJob(entry.model, "sparse_interactive", { custody: "native" });
    if (started.mode !== "backend_job") throw new Error("not a backend job");
    const terminal = await awaitPreviewMechanicsJob(started.job_id);
    const { result } = await readNativeFreshResult(started.job_id, terminal.result_bytes ?? 0, { standingModel: entry.model, summaryModel: entry.model });
    expect(nativeCustody(result).origin).toBe("fresh_job");
    const facts = nativeDiagnosticsPresentation(result)!;
    expect(facts.rosterChecked).toBe(false);
    const whole = receivedFromText<MechanicsResult>(TAMPERED);
    expect(presentedResultDiagnostics(result)).toEqual(presentedResultDiagnostics(whole));
    expect(presentedResultDiagnostics(result).at(-1)!.message).toBe(supplied(tampered.diagnostics.at(-1)!.message));
    await disposeDeliveredMechanicsResult(result);
    // Disposed custody: no fact is read; every message is quoted.
    expect(nativeDiagnosticsPresentation(result)).toBeNull();
    expect(presentedResultDiagnostics(result).map((d) => d.message)).toEqual(tampered.diagnostics.map((d) => supplied(d.message)));
  });
});

describe("facts that are missing, malformed or not the handle's attest nothing", () => {
  /** The handle's custody with S's header changed (a real S reply, rebuilt). */
  function withHeader(handle: NativeResult, change: (header: Json) => Json, custody: Partial<ReturnType<typeof nativeCustody>> = {}): NativeResult {
    const own = nativeCustody(handle);
    const header = change(structuredClone(own.summary.header));
    return createNativeResult({ ...own, summary: { ...own.summary, header }, ...custody });
  }
  const allQuoted = (handle: NativeResult) => handle.diagnostics.map((d) => supplied(d.message));

  it("quotes every message when the facts are missing or malformed", async () => {
    const handle = await restoredHandle(CANONICAL);
    expect(presentedResultDiagnostics(handle).some((d) => !d.message.startsWith(SUPPLEMENTAL_DIAGNOSTIC_LABEL))).toBe(true);
    const variants: [string, (h: Json) => Json][] = [
      ["missing", (h) => { delete h.diagnostics_presentation; return h; }],
      ["unknown member", (h) => ({ ...h, diagnostics_presentation: { ...h.diagnostics_presentation, attested: true } })],
      ["another version", (h) => ({ ...h, diagnostics_presentation: { ...h.diagnostics_presentation, version: 2 } })],
      ["a text flag", (h) => ({ ...h, diagnostics_presentation: { ...h.diagnostics_presentation, roster_checked: "true" } })],
      ["unsorted identities", (h) => ({ ...h, diagnostics_presentation: { ...h.diagnostics_presentation, combination_identities: [...h.diagnostics_presentation.combination_identities].reverse() } })],
      ["roster and certificate both", (h) => ({ ...h, diagnostics_presentation: { ...h.diagnostics_presentation, certificate_attested: true } })],
      ["roster without a population", (h) => ({ ...h, diagnostics_presentation: { ...h.diagnostics_presentation, combination_identities: [] } })],
      ["a malformed digest", (h) => ({ ...h, diagnostics_presentation: { ...h.diagnostics_presentation, binding: { ...h.diagnostics_presentation.binding, source_sha256: "x" } } })]
    ];
    for (const [name, change] of variants) {
      const changed = withHeader(handle, change);
      expect(nativeDiagnosticsPresentation(changed), name).toBeNull();
      expect(presentedResultDiagnostics(changed).map((d) => d.message), name).toEqual(allQuoted(handle));
    }
    // Unavailable facts attest nothing, whatever a diagnostic names (an empty ref included).
    const named = withHeader(handle, (h) => {
      delete h.diagnostics_presentation;
      h.envelope.diagnostics.push(note([""], "diagnostic:review:empty-ref"), note([firstCombination(CANONICAL)], "diagnostic:review:named"));
      return h;
    });
    expect(presentedResultDiagnostics(named).map((d) => d.message)).toEqual(allQuoted(named));
  });

  it("refuses another snapshot's facts: another length, route, table or recorded table, and mixed sources", async () => {
    const canonical = await restoredHandle(CANONICAL);
    const tampered = await restoredHandle(TAMPERED);
    const tenth = await restoredHandle(TENTH, TENTH_TABLE);
    // The tampered handle's custody carrying the canonical handle's S (mixed sources): its facts would
    // copy the claim; they are another snapshot's, so nothing is attested.
    const mixed = createNativeResult({ ...nativeCustody(tampered), summary: nativeCustody(canonical).summary });
    expect(nativeDiagnosticsPresentation(mixed)).toBeNull();
    expect(presentedResultDiagnostics(mixed).map((d) => d.message)).toEqual(allQuoted(mixed));
    const own = nativeCustody(canonical);
    for (const [name, custody] of [
      ["another length", { resultBytes: own.resultBytes + 1 }],
      ["another route", { verdict: nativeCustody(tenth).verdict }],
      ["another table", { verdict: { ...own.verdict, binding: { id: own.verdict.binding!.id, sha256: "0".repeat(64) } } }],
      ["a recorded table the facts were not read under", { recordedTable: TENTH_TABLE }]
    ] as const) {
      const changed = withHeader(canonical, (h) => h, custody as never);
      expect(nativeDiagnosticsPresentation(changed), name).toBeNull();
      expect(presentedResultDiagnostics(changed).map((d) => d.message), name).toEqual(allQuoted(canonical));
    }
    // The recorded handle without its table, likewise.
    const untabled = createNativeResult({ ...nativeCustody(tenth), recordedTable: null });
    expect(nativeDiagnosticsPresentation(untabled)).toBeNull();
    // Each handle reads its own facts, in either order (no reuse across snapshots).
    for (const [handle, text] of [[tampered, TAMPERED], [canonical, CANONICAL], [tampered, TAMPERED]] as const) {
      expect(presentedResultDiagnostics(handle)).toEqual(presentedResultDiagnostics(receivedFromText<MechanicsResult>(text)));
    }
  });

  it("reads nothing once the custody has ended (a disposed or stale handle)", async () => {
    const handle = await restoredHandle(CANONICAL);
    forgetNativeJob(nativeCustody(handle).jobId);
    expect(nativeDiagnosticsPresentation(handle)).toBeNull();
    expect(presentedResultDiagnostics(handle).map((d) => d.message)).toEqual(allQuoted(handle));
  });
});

describe("the report reads a whole value, never a native handle", () => {
  it("does not accept a native handle (by type) and throws on its guarded rows (at run time)", async () => {
    const handle = await restoredHandle(CANONICAL);
    const provenance = { recorded_by: "invented" } as never;
    // @ts-expect-error a native handle is not a report input (`NotNativeResult`): the report reads the C2 full copy.
    expect(() => diagnosticsForSections(handle, provenance)).toThrow(NativeResultRowsGuarded);
    // The whole value (the C2 copy of the same bytes) reads as the report always did.
    expect(diagnosticsForSections(receivedFromText<MechanicsResult>(CANONICAL), provenance).length).toBe(handle.diagnostics.length);
  });
});
