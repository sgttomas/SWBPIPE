import { describe, expect, it } from "vitest";
import { createHash } from "node:crypto";
import { readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
import { loadWasmEngine } from "../../services/wasmEngine/loadWasmEngine";
import { baseReaderInputs, type BaseReaderInput } from "../../test-support/baseReaderInputs";
import { ResultReaderError, readerDispatch, readerRowSemantics, readerTransport, readerValidate, rememberReceivedText, type ReaderName, type TransportReaderName } from "./resultReader";
// Item 2 only: the TypeScript readers, while they are present.
import { sourceContract as tsSourceContract, sourceSemanticBinding as tsBinding, numericalResultStanding as tsStanding } from "./numericalResultQuality";
import { validatePreviewPhysicsEvidence, validatePreviewPhysicsTransportMetadata } from "./previewPhysicsEvidence";
import { validatePhysicsEvidence, validatePhysicsTransportMetadata } from "./physicsResultEvidence";
import { validatePhysicsSourceRecovery, validatePhysicsSourceTransportMetadata } from "./physicsSourceRecovery";
import { validateSourceBlockRecovery } from "./sourceBlockRecovery";
import { validateLoadReferenceEvidence, validateLoadReferenceTransportMetadata } from "./loadReferenceEvidence";
import { validateLoadReferenceSourceEvidence, validateLoadReferenceSourceTransportMetadata, loadReferenceSourceLedgerTestSeam } from "./loadReferenceSourceEvidence";
import { resultSemantics, completeSourceMetadata, canonicalResultMetadata } from "./resultSemantics";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
type Entry = { id: string; parse?: string; dispatch?: Json; validate?: Record<string, Json>; transport?: Record<string, Json>; rows?: { sha256: string; signature_ids: string[] } };

/** R2 (b7-plan Unit R), the equivalence gate. Item 1, kept as the desktop reader's
 * conformance test: through wasm, every statement of the shared base corpus gives the
 * native Rust outcome committed by result_export's `base_reader_outcomes` (the dispatch,
 * each reader's validation and transport, each row's semantics by digest), and the
 * desktop's reader module (`resultReader.ts`) carries those outcomes unchanged when it
 * reads the statement's received text. */
const NATIVE = JSON.parse(readFileSync(resolve(__dirname, "../../../../../fixtures/results/base_reader_outcomes.json"), "utf8")).entries as Entry[];
const sha = (text: string) => createHash("sha256").update(text).digest("hex");
const sortKeys = (v: Json): Json => Array.isArray(v) ? v.map(sortKeys) : v && typeof v === "object" ? Object.fromEntries(Object.keys(v).sort().map(k => [k, sortKeys(v[k])])) : v;
/** A value read from its text, the text kept as received. */
function received(text: string | undefined): Json {
  if (text === undefined) return undefined;
  const value = JSON.parse(text);
  if (value && typeof value === "object") rememberReceivedText(value, text);
  return value;
}
function carried(run: () => unknown): Json {
  try { const value = run(); return { ok: true, ...(value as object ?? {}) }; } catch (error) {
    expect(error).toBeInstanceOf(ResultReaderError);
    const e = error as ResultReaderError;
    return { ok: false, error: { code: e.code, detail: e.detail } };
  }
}

const INPUTS = await baseReaderInputs();

describe("R2 equivalence gate, item 1: the wasm readers are the native Rust readers on the shared base corpus", () => {
  it("builds the native list's statements, in its order", () => {
    expect(INPUTS.map(x => x.id)).toEqual(NATIVE.map(x => x.id));
  });
  it.each(INPUTS.map((input, i) => [input.id, input, NATIVE[i]] as [string, BaseReaderInput, Entry]))("%s", async (_id, input, native) => {
    const engine = await loadWasmEngine();
    if (!input.parsed) {
      expect(native.parse).toBe("JSON_PARSE_REJECTED");
      expect(() => engine.readerDispatchJson(input.sourceJson)).toThrow(/WASM-ENGINE-INPUT-JSON-INVALID/);
      // A value JavaScript reads from such a text (a non-finite number) has no JSON text for the module.
      let value: Json = undefined;
      try { value = JSON.parse(input.sourceJson); } catch { return; }
      expect(readerDispatch(value).error?.code).toBe("JSON_PARSE_REJECTED");
      return;
    }
    const { sourceJson, modelJson, invocationJson } = input;
    const dispatch = JSON.parse(engine.readerDispatchJson(sourceJson, modelJson, invocationJson));
    expect(sortKeys(dispatch), "dispatch").toEqual(sortKeys(native.dispatch));
    const source = received(sourceJson), model = received(modelJson), invocation = received(invocationJson);
    expect(sortKeys(readerDispatch(source, model, invocation)), "dispatch, through the module").toEqual(sortKeys(dispatch));
    expect(Object.keys(native.validate ?? {}).sort()).toEqual([...input.readers].sort());
    for (const reader of input.readers) {
      const outcome = JSON.parse(engine.readerValidateJson(reader, sourceJson, modelJson, invocationJson));
      expect(sortKeys(outcome), `validate ${reader}`).toEqual(sortKeys(native.validate![reader]));
      expect(sortKeys(carried(() => readerValidate(reader as ReaderName, source, model, invocation))), `validate ${reader}, through the module`).toEqual(sortKeys(outcome));
    }
    expect(Object.keys(native.transport ?? {}).sort()).toEqual([...input.transports].sort());
    for (const reader of input.transports) {
      const outcome = JSON.parse(engine.readerTransportJson(reader, sourceJson));
      expect(sortKeys(outcome), `transport ${reader}`).toEqual(sortKeys(native.transport![reader]));
      expect(sortKeys(carried(() => readerTransport(reader as TransportReaderName, source))), `transport ${reader}, through the module`).toEqual(sortKeys(outcome));
    }
    expect(input.rowsJson !== null, "rows read").toBe(native.rows !== undefined);
    if (input.rowsJson !== null) {
      const rows = input.rowsJson.map(row => engine.readerRowSemanticsJson(row, sourceJson));
      expect(sha(`[${rows.join(",")}]`), "rows digest").toBe(native.rows!.sha256);
      const semantics = readerRowSemantics(source);
      rows.forEach((text, k) => {
        const outcome = JSON.parse(text);
        expect(native.rows!.signature_ids[k]).toBe(outcome.ok ? outcome.signature?.signature_id ?? null : outcome.error.code);
        expect(sortKeys(carried(() => semantics(source.results[k]))), `row ${k}, through the module`).toEqual(sortKeys(outcome));
      });
    }
  }, 120_000);
});

/* -------------------------------------------------------------------------- *
 * Item 2: wasm against the TypeScript readers, while both are present.
 * -------------------------------------------------------------------------- */
const codeOf = (text: string) => /^[A-Z][A-Z0-9_]*/.exec(text)?.[0] ?? text;
type Read = { ok: true; extra?: Json } | { ok: false; text: string; code: string };
async function ts(run: () => unknown): Promise<Read> {
  try { return { ok: true, extra: await run() }; } catch (e) { const text = e instanceof Error ? e.message : String(e); return { ok: false, text, code: codeOf(text) }; }
}
const rs = (outcome: Json): Read => outcome.ok ? { ok: true, extra: outcome } : { ok: false, text: outcome.error.detail ?? outcome.error.code, code: outcome.error.code };
function kind(a: Read, b: Read, passEqual: (a: Json, b: Json) => boolean = () => true): string {
  if (a.ok && b.ok) return passEqual(a.extra, b.extra) ? "same" : "both_pass_output_differs";
  if (!a.ok && !b.ok) return a.text === b.text ? "same" : a.code === b.code ? "declared_detail" : "declared_code";
  return a.ok ? "ts_pass_rust_refuse" : "ts_refuse_rust_pass";
}
const TS_VALIDATE: Record<string, (s: Json, m: Json, i: Json) => unknown> = {
  "preview-physics": (s, m) => validatePreviewPhysicsEvidence(s, m ?? undefined),
  "physics": (s, m) => validatePhysicsEvidence(s, m ?? undefined),
  // Without an invocation, the invocation-free ledger port (the TS twin of physics-source-1's validation without one).
  "physics-source": (s, _m, i) => i ? validatePhysicsSourceRecovery(s, i) : loadReferenceSourceLedgerTestSeam.validateLedger(s),
  "source-blocks": (s, _m, i) => validateSourceBlockRecovery(s, i),
  "load-reference": (s, m) => validateLoadReferenceEvidence(s, m ?? undefined),
  "load-reference-source": s => validateLoadReferenceSourceEvidence(s),
};
const TS_TRANSPORT: Record<string, (s: Json) => unknown> = {
  "preview-physics": s => validatePreviewPhysicsTransportMetadata(s),
  "physics": s => validatePhysicsTransportMetadata(s.contract_evidence),
  "physics-source": s => validatePhysicsSourceTransportMetadata(s),
  "load-reference": s => validateLoadReferenceTransportMetadata(s),
  "load-reference-source": s => validateLoadReferenceSourceTransportMetadata(s),
};
/** The classes a difference may take (T3's R2 rulings): both refuse with another code or
 * detail (parse-boundary refusals included); and three accepted differences, each by name. */
const ALLOWED = new Set(["same", "declared_code", "declared_detail", "declared_code(parse)"]);
const ACCEPTED: Record<string, (surface: string, id: string, item: Json) => boolean> = {
  // (1) The legacy 0.1 result: Rust's binding is null where TS throws.
  binding_differs: (s, id, item) => s === "results:binding" && id === "results/product_preview/invented_demo_result_legacy_0_1" && item.rust === null && item.ts.ok === false,
  // (2) Transported metadata: Rust's standing is needs_recompute where TS throws a TypeError.
  ts_throws: (s, id, item) => /^lrs?-transport:standing$/.test(s) && item.rust.status === "needs_recompute" && item.rust.eligible === false && /^Cannot read properties of undefined/.test(item.ts.text),
  // (3) The physics-source test seam: the invocation-free TS ledger port passes a relabelled joined statement Rust refuses.
  ts_pass_rust_refuse: (s, id) => s === "lrs:validate:physics-source" && id === "lrs/RELABEL-joined-as-physics-source-resealed",
};

describe("R2 equivalence gate, item 2: wasm against the TypeScript readers differs only in the declared classes", () => {
  it("counts and lists every difference", async () => {
    const engine = await loadWasmEngine();
    const counts: Record<string, Record<string, number>> = {};
    const lists: Record<string, Record<string, Json[]>> = {};
    const outside: Json[] = [];
    const note = (surface: string, k: string, item: Json) => {
      ((counts[surface] ??= {})[k] = (counts[surface][k] ?? 0) + 1);
      if (k === "same") return;
      ((lists[surface] ??= {})[k] ??= []).push(item);
      if (!ALLOWED.has(k) && !ACCEPTED[k]?.(surface, item.id, item)) outside.push({ surface, kind: k, ...item });
    };
    for (const input of INPUTS) {
      const group = input.id.split("/")[0];
      if (!input.parsed) {
        let parsed: Json;
        try { parsed = JSON.parse(input.sourceJson); } catch { note(`${group}:parse`, "same", { id: input.id }); continue; }
        for (const reader of input.readers) {
          const t = await ts(() => TS_VALIDATE[reader](structuredClone(parsed), null, null));
          note(`${group}:validate:${reader}`, t.ok ? "ts_pass_rust_refuse(parse)" : "declared_code(parse)", { id: input.id, ts: t.ok ? "pass" : t.text });
        }
        continue;
      }
      const sourceValue = JSON.parse(input.sourceJson);
      const model = input.modelJson === undefined ? null : JSON.parse(input.modelJson);
      const invocation = input.invocationJson === undefined ? null : JSON.parse(input.invocationJson);
      const dispatch = JSON.parse(engine.readerDispatchJson(input.sourceJson, input.modelJson, input.invocationJson));
      const tRoute = await ts(() => tsSourceContract(structuredClone(sourceValue)));
      const tsRoute = tRoute.ok ? tRoute.extra : "unsupported";
      note(`${group}:route`, tsRoute === dispatch.route ? "same" : "route_differs", { id: input.id, ts: tsRoute, rust: dispatch.route, rust_error: dispatch.error });
      if (tsRoute !== "unsupported" && dispatch.route !== "unsupported") {
        const t = await ts(() => tsBinding(structuredClone(sourceValue)));
        note(`${group}:binding`, t.ok && JSON.stringify(t.extra) === JSON.stringify(dispatch.binding) ? "same" : "binding_differs", { id: input.id, ts: t, rust: dispatch.binding });
      }
      for (const reader of input.readers) {
        const t = await ts(() => TS_VALIDATE[reader](structuredClone(sourceValue), model, invocation));
        const r = rs(JSON.parse(engine.readerValidateJson(reader, input.sourceJson, input.modelJson, input.invocationJson)));
        const passEqual = (a: Json, b: Json) => reader === "physics-source" && !invocation ? true
          : reader === "physics-source" || reader === "source-blocks" ? a.eligible === b.eligible && JSON.stringify(a.findings) === JSON.stringify(b.findings) : true;
        note(`${group}:validate:${reader}`, kind(t, r, passEqual), { id: input.id, ts: t.ok ? t.extra ?? "pass" : t.text, rust: r.ok ? r.extra : r.text });
      }
      for (const reader of input.transports) {
        const t = await ts(() => TS_TRANSPORT[reader](structuredClone(sourceValue)));
        const r = rs(JSON.parse(engine.readerTransportJson(reader, input.sourceJson)));
        note(`${group}:transport:${reader}`, kind(t, r), { id: input.id, ts: t.ok ? "pass" : t.text, rust: r.ok ? "pass" : r.text });
      }
      // Standing, after the registration the desktop performs (G4).
      const standingSource = structuredClone(sourceValue);
      if (tsRoute === "source_blocks" && invocation) await ts(() => validateSourceBlockRecovery(standingSource, invocation));
      if (tsRoute === "physics_source" && invocation) await ts(() => validatePhysicsSourceRecovery(standingSource, invocation));
      if (tsRoute === "load_reference_source") await ts(() => validateLoadReferenceSourceEvidence(standingSource));
      const tStanding = await ts(() => tsStanding(standingSource, model ?? undefined));
      const rStanding = dispatch.standing;
      let sk: string;
      if (!tStanding.ok) sk = "ts_throws";
      else {
        const t = tStanding.extra;
        if (t.eligible !== rStanding.eligible || t.status !== rStanding.status) sk = "eligibility_differs";
        else if (JSON.stringify(t.findings) === JSON.stringify(rStanding.findings)) sk = "same";
        else if (!t.eligible) {
          const tc = t.findings.map(codeOf), rc = rStanding.findings.map(codeOf);
          sk = JSON.stringify(tc) === JSON.stringify(rc) ? "declared_detail" : tc.length === rc.length ? "declared_code" : "ineligible_findings_differ";
        } else sk = "findings_differ";
      }
      note(`${group}:standing`, sk, { id: input.id, ts: tStanding.ok ? tStanding.extra : tStanding, rust: rStanding });
      if (input.rowsJson !== null) {
        (sourceValue.results ?? []).forEach((row: Json, k: number) => {
          const w = JSON.parse(engine.readerRowSemanticsJson(input.rowsJson![k], input.sourceJson));
          let t: Read;
          try {
            t = { ok: true, extra: { signature: resultSemantics(row, sourceValue), complete_metadata: completeSourceMetadata(row), canonical_metadata: canonicalResultMetadata(row, sourceValue) } };
          } catch (e) { const text = e instanceof Error ? e.message : String(e); t = { ok: false, text, code: codeOf(text) }; }
          note(`${group}:rows`, kind(t, rs(w), (a, b) => JSON.stringify(sortKeys(a.signature)) === JSON.stringify(sortKeys(b.signature))
            && a.complete_metadata === b.complete_metadata && JSON.stringify(sortKeys(a.canonical_metadata)) === JSON.stringify(sortKeys(b.canonical_metadata))), { id: input.id, row: row.id, ts: t, rust: w });
        });
      }
    }
    const out = process.env.R2_GATE_TS_OUT;
    if (out) writeFileSync(out, JSON.stringify({ counts, outside, lists }, null, 1));
    expect(outside, "differences outside the declared classes").toEqual([]);
  }, 3_600_000);
});
