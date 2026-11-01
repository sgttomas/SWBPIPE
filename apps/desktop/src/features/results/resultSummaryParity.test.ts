import { describe, expect, it } from "vitest";
import { createHash } from "node:crypto";
import { readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
import type { MechanicsResult, PreviewModel } from "../../types";
import { jsonTextKeepingNegativeZero } from "../../services/jsonText";
import { loadWasmEngine } from "../../services/wasmEngine/loadWasmEngine";
import {
  DISPLAY_COLUMNS,
  RESULT_SUMMARY_MALFORMED,
  SOURCE_OFFSETS,
  SOURCE_REFS,
  decodeResultSummary,
  displacementExtremesFromSummary,
  stressRatioTableFromSummary,
  summaryValue,
  type ResultSummary
} from "../../services/resultSummary";
import { summaryDisplayRows } from "../../services/resultSummary";
import { resultSummaryCases, resultSummaryCasesText, type CaseText, type SummaryCase } from "../../test-support/resultSummaryCases";
import { availableResultColourQuantities, buildResultColourOverlay, resultOverlayCases, RESULT_COLOUR_QUANTITIES } from "../viewport/resultColourOverlay";
import { buildReactionOverlay } from "../viewport/reactionOverlay";
import { buildDeformationOverlay, buildRotationOverlay } from "../viewport/deformationOverlay";
import { componentBodies } from "./componentBody";
import { buildDisplacementExtremes } from "./displacementExtremes";
import { buildKeyResults, withheldSupportStateRows } from "./keyResults";
import { numericalHealthView, parseNumericalHealth, type ResultOverlayStanding } from "./numericalHealth";
import { countResultFamilies, countResultFrames } from "./ResultsPanel";
import { navigatorCases } from "./resultsNavigator";
import { rememberReceivedText } from "./resultReader";
import { semanticLabels } from "./resultSemantics";
import { buildStressRatioTable } from "./stressRatioTable";
import { buildSupportLoadSummaries, supportLoadCases } from "./supportLoadSummary";

/** C1 slice 2 (DEL-04-05 `c1-native-custody-plan.md`): the TypeScript builders over the full rows
 * are the oracle of S, the native summary (`src-tauri/src/result_summary.rs`). For every case of
 * `resultSummaryCases.ts` (committed as `fixtures/results/result_summary_cases.json`), the response
 * the Rust test commits (`result_summary_outcomes.json`) is decoded and each table compared with
 * its builder's output on the same model and standing; the viewport overlays, which draw per
 * choice, are run on the rows S's display planes give back and compared with the same overlays on
 * the full rows, for every case, quantity and resolution. Row ids come from the planes' ordinals
 * (`resolveId`); nothing else is read from the full rows. */

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
type Row = MechanicsResult["results"][number];
type Outcome = { name: string; length: number; sha256: string; response: string };

const ROOT = resolve(__dirname, "../../../../..");
const CASES_PATH = resolve(ROOT, "fixtures/results/result_summary_cases.json");
const OUTCOMES = JSON.parse(readFileSync(resolve(ROOT, "fixtures/results/result_summary_outcomes.json"), "utf8")).cases as Outcome[];
const CASES = resultSummaryCases();
const ENVELOPE_MEMBERS = [
  "document_kind", "schema_version", "producer", "run_id", "model_ref", "accepted_model_state_mutated", "status", "summary",
  "numerical_quality", "formulation_basis", "professional_boundary", "diagnostics", "contract_evidence"
];
const PER_ROW_COMBINATION_EVIDENCE = ["rows", "recoveries", "missing_sources"];

function pointer(value: Json, path: string): Json {
  return path === "" ? value : path.slice(1).split("/").map((token) => token.replace(/~1/g, "/").replace(/~0/g, "~")).reduce((at, token) => at[token], value);
}
function textOf(reference: CaseText): string {
  if ("text" in reference) return reference.text;
  const text = readFileSync(resolve(ROOT, reference.path), "utf8");
  return reference.pointer === "" ? text : jsonTextKeepingNegativeZero(pointer(JSON.parse(text), reference.pointer));
}

/** The envelope S carries: its members, the per-row combination evidence counted, not copied. */
function expectedEnvelope(source: Json): { envelope: Json; counts: Json[]; omitted: string[] } {
  const envelope: Json = {};
  const counts: Json[] = [];
  for (const member of ENVELOPE_MEMBERS) {
    if (!(member in source)) continue;
    envelope[member] = structuredClone(source[member]);
  }
  const records = envelope.contract_evidence?.exact_combinations;
  if (Array.isArray(records)) {
    for (const record of records) {
      if (record === null || typeof record !== "object" || Array.isArray(record)) continue;
      const count: Json = { combination_id: record.combination_id ?? null };
      for (const key of PER_ROW_COMBINATION_EVIDENCE) {
        if (key in record) {
          count[key] = Array.isArray(record[key]) ? record[key].length : 0;
          delete record[key];
        }
      }
      counts.push(count);
    }
  }
  const omitted = Object.keys(source).filter((key) => key !== "results" && !ENVELOPE_MEMBERS.includes(key)).sort();
  return { envelope, counts, omitted };
}

/** The rows S's display planes give back: each display row's members, its value and its
 * `source_result_refs`, its id from its ordinal. */
const displayRows = summaryDisplayRows;

function standingOf(testCase: SummaryCase, sourceText: string, engine: Json): { health: Json; standing: ResultOverlayStanding | null } {
  if (testCase.standing === null) return { health: null, standing: null };
  const health = "health" in testCase.standing ? testCase.standing.health : JSON.parse(engine.numericalHealthJson(sourceText, JSON.stringify(testCase.standing.findings)));
  const eligible = testCase.standing.eligible;
  return { health, standing: { eligible, health: numericalHealthView(parseNumericalHealth(health), { eligible, verdictHold: false }) } };
}

describe("C1 slice 2: S is the TypeScript builders' output", () => {
  it("keeps the committed cases file equal to its generator", () => {
    const text = resultSummaryCasesText();
    if (process.env.RESULT_SUMMARY_CASES_WRITE) writeFileSync(CASES_PATH, text);
    expect(readFileSync(CASES_PATH, "utf8")).toBe(text);
    expect(OUTCOMES.map((outcome) => outcome.name)).toEqual(CASES.map((testCase) => testCase.name));
  });

  it.each(CASES.map((testCase, i) => [testCase.name, testCase, OUTCOMES[i]] as [string, SummaryCase, Outcome]))("%s", async (_name, testCase, outcome) => {
    const engine = await loadWasmEngine();
    const bytes = Buffer.from(outcome.response, "base64");
    expect(bytes.length).toBe(outcome.length);
    expect(createHash("sha256").update(bytes).digest("hex")).toBe(outcome.sha256);
    const summary = decodeResultSummary(new Uint8Array(bytes));
    const header = summary.header;
    const tables = header.tables;
    const sourceText = textOf(testCase.result);
    const source = JSON.parse(sourceText) as MechanicsResult;
    rememberReceivedText(source, sourceText);
    const model = JSON.parse(textOf(testCase.model)) as PreviewModel;
    const rows = source.results;
    const resolveId = (ordinal: number) => rows[ordinal].id;
    const { health, standing } = standingOf(testCase, sourceText, engine);

    // The header: the envelope members, the counts, the units and the numerical health.
    const expected = expectedEnvelope(source);
    expect(header.envelope, "envelope").toEqual(expected.envelope);
    expect(header.combination_evidence_counts, "combination evidence").toEqual(expected.counts);
    expect(header.omitted_members, "omitted members").toEqual(expected.omitted);
    expect(header.row_count).toBe(rows.length);
    expect(header.units, "units").toEqual([...new Set(rows.map((row) => row.unit))].map((unit) => unit ?? null));
    expect(header.numerical_health, "numerical health").toEqual(health);
    expect(header.standing, "standing").toEqual(standing ? { eligible: standing.eligible, health_view: standing.health !== null } : null);

    // The tables, each its builder's output.
    expect(tables.results_panel.family_counts, "families").toEqual(countResultFamilies(rows, semanticLabels(source)));
    const frames = [...countResultFrames(rows)];
    if (tables.results_panel.frame_counts.others_ordered) expect(tables.results_panel.frame_counts.counts, "frames").toEqual(frames);
    else expect([...tables.results_panel.frame_counts.counts].sort(), "frames").toEqual([...frames].sort());
    expect(tables.results_panel.navigator_cases, "navigator cases").toEqual(navigatorCases(rows));
    expect(tables.support_load_cases, "support load cases").toEqual(supportLoadCases(model, source));
    expect(tables.overlay_cases, "overlay cases").toEqual(resultOverlayCases(model, source));
    expect(displacementExtremesFromSummary(summary), "displacement extremes").toEqual(buildDisplacementExtremes({ model, result: source, standing }));
    expect(tables.support_load_summaries, "support load summaries").toEqual(buildSupportLoadSummaries({ model, result: source, standing }));
    expect(stressRatioTableFromSummary(summary, resolveId), "stress and ratio table").toEqual(buildStressRatioTable({ model, result: source, standing }));
    expect(tables.component_bodies, "component bodies").toEqual([...componentBodies(source)]);
    const keyResults: Json = { ...buildKeyResults({ model, result: source, standing }) };
    delete keyResults.standingText;
    expect(tables.key_results, "key results").toEqual(keyResults);
    expect(tables.withheld_support_state_rows, "withheld support-state rows").toEqual(withheldSupportStateRows(source, standing));
    expect(tables.colour_quantities, "colour quantities").toEqual(availableResultColourQuantities(model, source).map((quantity) => quantity.id));

    // The overlays, on the rows the display planes give back.
    const reduced = { ...header.envelope, results: displayRows(summary, resolveId) } as MechanicsResult;
    expect(availableResultColourQuantities(model, reduced)).toEqual(availableResultColourQuantities(model, source));
    const overlayCases = resultOverlayCases(model, source);
    for (const overlayCase of [null, ...overlayCases]) {
      expect(buildReactionOverlay({ model, result: reduced, standing, overlayCase }), `reactions ${overlayCase?.key}`)
        .toEqual(buildReactionOverlay({ model, result: source, standing, overlayCase }));
      for (const quantity of RESULT_COLOUR_QUANTITIES) {
        for (const resolution of ["member", "station"] as const) {
          expect(buildResultColourOverlay({ model, result: reduced, standing, quantity, overlayCase, resolution }), `colour ${quantity.id} ${overlayCase?.key} ${resolution}`)
            .toEqual(buildResultColourOverlay({ model, result: source, standing, quantity, overlayCase, resolution }));
        }
      }
    }
    // A blocked result's deformation states the full row count, which S states as `row_count`.
    if (source.status.mechanics === "MECHANICS_SOLVED") {
      const lengthUnit = (model as Json).project?.units?.length ?? "mm";
      for (const basisKey of [null, ...overlayCases.map((item) => item.key)]) {
        for (const options of [{ basisKey }, { basisKey, scale: { mode: "factor", factor: null }, modelLengthUnit: lengthUnit, modelExtent: 10 }] as const) {
          expect(buildDeformationOverlay(model, reduced, options), `deformation ${basisKey}`).toEqual(buildDeformationOverlay(model, source, options));
        }
        expect(buildRotationOverlay(model, reduced, { basisKey }), `rotation ${basisKey}`).toEqual(buildRotationOverlay(model, source, { basisKey }));
      }
    }
  });
});

describe("C1 slice 2: S's layout is read exactly or refused by name", () => {
  const response = () => new Uint8Array(Buffer.from(OUTCOMES.find((item) => item.name === "invented/stresses")!.response, "base64"));
  const refused = (bytes: Uint8Array, reason: string) => {
    expect(() => decodeResultSummary(bytes)).toThrow(`${RESULT_SUMMARY_MALFORMED}: ${reason}`);
  };

  it("refuses every truncation, a foreign prefix and trailing bytes", () => {
    const good = response();
    for (let length = 0; length < good.length; length += Math.max(1, Math.floor(good.length / 97))) {
      expect(() => decodeResultSummary(good.subarray(0, length)), `${length} bytes`).toThrow(RESULT_SUMMARY_MALFORMED);
    }
    expect(() => decodeResultSummary(good.subarray(0, good.length - 1))).toThrow(RESULT_SUMMARY_MALFORMED);
    const foreign = good.slice();
    foreign[0] = 0x58;
    refused(foreign, "not a result summary");
    const trailing = new Uint8Array(good.length + 1);
    trailing.set(good);
    refused(trailing, "the planes are not the response's remaining bytes");
    const long = good.slice();
    new DataView(long.buffer).setBigUint64(8, 2n ** 60n, true);
    refused(long, "the header runs past the end");
  });

  it("refuses extra, unknown and reordered plane descriptors", () => {
    const patched = (change: (header: Json) => void) => {
      const good = response();
      const headerLength = Number(new DataView(good.buffer).getBigUint64(8, true));
      const oldStart = Math.ceil((16 + headerLength) / 8) * 8;
      const header = JSON.parse(new TextDecoder().decode(good.subarray(16, 16 + headerLength)));
      change(header);
      const encoded = new TextEncoder().encode(JSON.stringify(header));
      const start = Math.ceil((16 + encoded.length) / 8) * 8;
      const out = new Uint8Array(start + good.length - oldStart);
      out.set(good.subarray(0, 8));
      new DataView(out.buffer).setBigUint64(8, BigInt(encoded.length), true);
      out.set(encoded, 16);
      out.set(good.subarray(oldStart), start);
      return out;
    };
    refused(patched((header) => header.planes.push({name: "extra", offset: header.planes_bytes, count: 0})), "plane descriptors do not match the summary schema");
    refused(patched((header) => { header.planes[0].name = "unknown"; }), "plane descriptors do not match the summary schema");
    refused(patched((header) => { [header.planes[0], header.planes[1]] = [header.planes[1], header.planes[0]]; }), "plane descriptors do not match the summary schema");
  });

  it("refuses a code, ordinal or offset the header does not admit", () => {
    const good = response();
    const summary = decodeResultSummary(good);
    const headerLength = Number(new DataView(good.buffer, good.byteOffset).getBigUint64(8, true));
    const start = Math.ceil((16 + headerLength) / 8) * 8;
    const offsetOf = (name: string) => start + summary.header.planes.find((item) => item.name === name)!.offset;
    const patched = (name: string, value: number) => {
      const bytes = good.slice();
      new DataView(bytes.buffer).setFloat64(offsetOf(name), value, true);
      return bytes;
    };
    refused(patched("display.kind", summary.header.values.length), "display.kind code");
    refused(patched("display.kind", 0.5), "display.kind code");
    refused(patched("display.ordinal", summary.header.row_count), "display ordinal");
    refused(patched("stress.case", -1), "stress case");
    refused(patched(SOURCE_OFFSETS, 1), "source offset");
    // Little-endian: the last ordinal's bytes, read big-endian, are another number.
    const ordinals = summary.planes.get("display.ordinal")!;
    const last = new DataView(good.buffer, good.byteOffset).getFloat64(offsetOf("display.ordinal") + (ordinals.length - 1) * 8, false);
    expect(ordinals[ordinals.length - 1]).toBeGreaterThan(0);
    expect(last).not.toBe(ordinals[ordinals.length - 1]);
  });
});
