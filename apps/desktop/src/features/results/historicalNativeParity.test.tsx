// RT-A4 §10.4 and DEL-07-05 (C1 review F2, F3 and test strength): a reopened run is shown the same
// whether the webview holds it whole or natively. The same stored envelope is reopened both ways:
// whole (`buildHistoricalRunContext`, the webview's own readers over the full rows) and natively
// (the session's native open through `nativeReopenEmulation.ts`, whose restored handle's S, pages,
// lookups and row semantics are the actual Rust commands' recorded replies, `nativeReplayStore.ts`).
// Both are rendered by `HistoricalRunPanel`, which passes no numerical health, so each panel shows
// the run's own standalone health. The two panels must agree on the route, the standing line, the
// notices, the numerical-health header and detail, every row cell of the first page (category, row
// label, row health badge, frame) and of each headline or intensified row, a selected row's Result Detail endpoint pair (admitted by the row
// semantics' family) and its CSV, and every group's CSV (the
// whole panel's single save against the native stream's chunks), health cells included. Cases: each
// registry variant recorded before RT-A4 as recorded, and main's preview-physics-2 run recorded
// before RT-A4 and a current preview-physics-2 run, each healthy and with an invented sensitive,
// unresolved or failed case. Nothing here escalates standing: the native handle stays unbound, and
// a retained successor still says it needs recompute. Unit transport replay; NOT native UI
// qualification. Committed fixtures and invented variants only.
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { cleanup, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../../test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));

import type { AnalysisRunEnvelope, LocalProjectEnvelope, MechanicsResult, PreviewModel } from "../../types";
import { computeModelHash, computeProjectEnvelopeHash } from "../../services/hashService";
import { openLocalProjectNative } from "../../services/nativeReopen";
import { isNativeResult, nativeCustody, resetNativeResultsForTests } from "../../services/nativeResult";
import { resetSessionResultCustodyForTests } from "../../services/previewService";
import { resetNativeReopenEmulation } from "../../test/nativeReopenEmulation";
import { DisplayUnitsProvider } from "../display-units";
import type { DisplayQuantityRequest, DisplayQuantityResult } from "../../services/displayQuantityService";
import { buildHistoricalRunContext, buildNativeHistoricalRunContext, HistoricalRunPanel, type HistoricalRunContext } from "./HistoricalRunContext";
import { NUMERICAL_HEALTH_STATE_TEXT } from "./numericalHealthText";
import { N_HEADLINE, N_INTENSIFIED, PREVIEW_INTENSIFIED_KIND, PREVIEW_MAXIMUM_KIND } from "./knownSemanticLimitations";
import { isRetainedRoute } from "./numericalResultQuality";
import { displayRoute } from "./resultReader";
import { numericalResultStanding } from "./resultStanding";
import { semanticLabels } from "./resultSemantics";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
const root = resolve(__dirname, "../../../../../");
const text = (path: string) => readFileSync(resolve(root, path), "utf8");
const json = <T,>(path: string) => JSON.parse(text(path)) as T;
const HISTORY = "fixtures/persistence/recorded_pre_rt_a4_pf";
/** Recording the native requests (`nativeReplayStore.ts`): short waits, and every group's export and
 * every detail is attempted though an earlier one has no recorded reply yet, so one pass records the
 * next request of each. The comparisons hold only on a pass with every reply recorded. */
const RECORDING = !!process.env.NATIVE_REPLAY_RECORD;
const WAIT = { timeout: RECORDING ? 1_500 : 30_000 };
const attempt = async (step: () => Promise<void>) => { if (!RECORDING) return step(); try { await step(); } catch { /* recorded; the next pass reads on */ } };

type Stored = { source: MechanicsResult; record: AnalysisRunEnvelope; model: PreviewModel };
const retained = (dir: string, name: string) => (): Stored => {
  const document = json<Json>(`${HISTORY}/${dir}/${name}.json`);
  return { source: document.source, record: json<AnalysisRunEnvelope>(`${HISTORY}/${dir}/${name}.analysis_run.json`), model: document.invocation.request.model };
};
const preRtA4PreviewPhysics = (): Stored => {
  const transient = json<{ run_record: AnalysisRunEnvelope; input_manifest_canonical: string }>(`${HISTORY}/preview_physics_2/transient_run_input.json`);
  return { source: json(`${HISTORY}/preview_physics_2/preview_physics_connected_sparse.json`), record: transient.run_record, model: JSON.parse(transient.input_manifest_canonical).model_basis.model_payload };
};
/** A current preview-physics-2 run: the reopen corpus's saved run of the committed statement (its
 * record the builder corpus's, `historical_reopen_cases.json` `saved:route:…`). */
const currentPreviewPhysics = (): Stored => {
  const base = "route:fixtures/results/preview_physics_connected_sparse.json";
  const saved = json<Json>("apps/desktop/src/test-support/native-reopen/historical_reopen_cases.json").cases.find((c: Json) => c.id === `saved:${base}`);
  const built = json<Json>("apps/desktop/src/test-support/analysis-run-v03/analysis_run_v03_cases.json").cases.find((c: Json) => c.id === base);
  return { source: json("fixtures/results/preview_physics_connected_sparse.json"), record: built.expected.record, model: saved.project.model };
};
const timoshenko = (): Stored => {
  // Main holds no model of this statement (the fixture README's "Gap"), as `recordedPreRtA4Display.test.tsx`.
  const source = json<MechanicsResult>(`${HISTORY}/preview_physics_timoshenko_1/preview_physics_timoshenko_straight_sparse.json`);
  const record = json<AnalysisRunEnvelope>(`${HISTORY}/preview_physics_timoshenko_1/preview_physics_timoshenko_straight_sparse.analysis_run.json`);
  const cases = [...new Set(source.results.flatMap((row) => (row.basis_ref?.ref_type === "load_case" ? [row.basis_ref.ref_id] : [])))];
  const model = { schema_version: "0.9.0", project: { id: source.model_ref }, model_settings: { shear_model: "timoshenko_annulus_energy_v1" }, load_cases: cases.map((id) => ({ id })), diagnostics: [] } as unknown as PreviewModel;
  return { source, record, model };
};

/** An invented health variant: the first load case's solve quality (and the envelope's status). */
type Quality = "as recorded" | "sensitive" | "unresolved" | "failed";
const STATE: Readonly<Record<Exclude<Quality, "as recorded">, keyof typeof NUMERICAL_HEALTH_STATE_TEXT>> = { sensitive: "sensitive", unresolved: "quality_unresolved", failed: "solve_failed" };
function withQuality(stored: Stored, quality: Quality): Stored {
  if (quality === "as recorded") return stored;
  const source = structuredClone(stored.source) as Json;
  source.numerical_quality.cases[0].solve_quality = quality;
  source.numerical_quality.status = quality;
  return { ...stored, source };
}

type Case = { name: string; table: string | null; stored: () => Stored; quality: Quality };
const QUALITIES: Quality[] = ["as recorded", "sensitive", "unresolved", "failed"];
const CASES: Case[] = [
  ...QUALITIES.map((quality): Case => ({ name: `preview-physics-2 recorded before RT-A4, ${quality}`, table: "1ccc9561cb4ce59d047f3dab7927a927c44a707efb3acb90e7b07a21b3bad0a2", stored: preRtA4PreviewPhysics, quality })),
  ...QUALITIES.map((quality): Case => ({ name: `current preview-physics-2, ${quality}`, table: null, stored: currentPreviewPhysics, quality })),
  { name: "preview-physics-timoshenko-1 recorded before RT-A4", table: "a7b83c37a4af9bf70f56513f80883003a2a3860387319311ea2e967dd0cf764b", stored: timoshenko, quality: "as recorded" },
  { name: "preview-physics-retained-1 recorded before RT-A4", table: "b001070e5fedeeb101522b22ce0d61bb7cf85f82e86b8d851713f8d9fc8c4c46", stored: retained("preview_physics_retained_1", "retained_precision_milestone_successor_sparse_interactive"), quality: "as recorded" },
  { name: "physics-retained-1 (main) recorded before RT-A4", table: "cf29fd5fc5d61de0f1133e9668b05f3d62ae8f696944fe04de7470029f5357dd", stored: retained("physics_retained_1/main", "retained_precision_exact_successor_sparse_interactive"), quality: "as recorded" },
  { name: "physics-retained-1 (#101) recorded before RT-A4", table: "045f6f0d8ce350246b21fb712d2dd14bf3a7f7f3690e66e062cbbf77d3696a3b", stored: retained("physics_retained_1/pr101", "retained_precision_exact_successor_sparse_interactive"), quality: "as recorded" },
  // the registry's tenth variant, main's exact successor at 252bef969 (85c2678a…).
  { name: "physics-retained-1 (main 252bef969) recorded before RT-A3 §4", table: "85c2678a78e794b44761c11efb73c215645c7e2560d802ac8ad6075e7804c691", stored: retained("physics_retained_1/main_252bef969", "retained_precision_exact_successor_sparse_interactive"), quality: "as recorded" },
];

let envelope: Json = null;
let saved: string[] = [];
let streams: { begin: Json; appends: Json[] }[] = [];
beforeEach(() => {
  resetSessionResultCustodyForTests();
  resetNativeResultsForTests();
  resetNativeReopenEmulation();
  saved = [];
  streams = [];
  invokeMock.mockImplementation(async (command: string, args: Json) => {
    switch (command) {
      // The legacy open the emulated native open reads the stored envelope from.
      case "open_local_project": return structuredClone(envelope);
      // The whole panel's single save of a table (the browser panel's CSV).
      case "save_local_result_json": {
        const href = args.request.href as string;
        const csv = decodeURIComponent(href.slice("data:text/csv;charset=utf-8,".length));
        saved.push(csv);
        return { outcome: "saved", file_name: args.request.file_name, byte_count: new TextEncoder().encode(csv).length, replaced_existing: false, durability: "not_guaranteed", path_containment: "best_effort_non_adversarial" };
      }
      // The native CSV stream, as `native_csv_stream.rs` answers a well-formed sequence (as
      // `NativeResultsPanel.native.test.tsx`); the chunks are kept to compare.
      case "begin_local_csv_stream":
        streams.push({ begin: args, appends: [] });
        return { outcome: "streaming", stream_id: "0123456789abcdef0123456789abcdef", file_name: args.request.file_name, sequence: 1, byte_count: new TextEncoder().encode(args.request.text).length };
      case "append_local_csv_stream":
        streams.at(-1)!.appends.push(args);
        return { outcome: "streaming", sequence: args.sequence + 1, byte_count: 0 };
      case "finish_local_csv_stream":
        return { outcome: "saved", file_name: streams.at(-1)!.begin.request.file_name, byte_count: args.byteCount, replaced_existing: false, durability: "not_guaranteed", path_containment: "best_effort_non_adversarial" };
      default: throw new Error(`UNEXPECTED_COMMAND: ${command}`);
    }
  });
});
afterEach(() => {
  cleanup();
  invokeMock.mockReset();
  delete (window as Json).__TAURI_INTERNALS__;
});

async function storedEnvelope({ source, record, model }: Stored): Promise<Json> {
  const payload = { model, mechanics_result: source, analysis_run: record, model_hash: await computeModelHash(model), editor_intents: [], proposal: null, selected_review_target: null };
  return { ...payload, project_envelope_hash: await computeProjectEnvelopeHash(payload as never), summary: { project_id: (model as Json).project?.id ?? "", message: "Opened the invented stored project." } };
}

/** Every SI readout converts (a stand-in converter), as `recordedPreRtA4Display.test.tsx`. */
const converter = async (requests: DisplayQuantityRequest[]): Promise<DisplayQuantityResult[]> =>
  requests.map((request) => ({ id: request.id, status: "converted", value: request.value, unit: request.to_unit }));

type Shown = { standing: string; notices: string[][]; health: string | null; healthDetail: string | null; rows: string[][] };
/** What the panel shows, apart from the rows' readouts (the native page's are its own cells). */
function shown(panel: HTMLElement): Shown {
  const text = (testId: string) => panel.querySelector(`[data-testid='${testId}']`)?.textContent ?? null;
  return {
    standing: text("numerical-result-standing") ?? "",
    notices: [...panel.querySelectorAll("[data-testid^='results-notice-']")].map((n) => [n.getAttribute("data-testid")!, n.textContent ?? ""]),
    health: text("numerical-health-header-line"),
    healthDetail: text("numerical-health-detail"),
    rows: [...panel.querySelectorAll<HTMLElement>("tr[data-testid^='result-row-']")].map((row) => [row.dataset.testid!, row.querySelector("td")?.textContent ?? "", row.querySelector("[data-testid^='result-row-frame-']")?.textContent ?? ""]),
  };
}

type Panel = { shown: Shown; csv: Map<string, string>; details: Map<string, string>; labelled: Map<string, string> };
/** Renders the historical panel, waits for `expected` (the whole panel's, for the native one), then
 * exports every group's CSV and each `detailIds` row's endpoint pair (Result Detail). */
async function read(context: HistoricalRunContext, expected: Panel | null, detailIds: readonly string[], labelIds: readonly string[]): Promise<Panel> {
  render(<DisplayUnitsProvider initialPreference="SI" converter={converter}><HistoricalRunPanel context={context} /></DisplayUnitsProvider>);
  const panel = screen.getByTestId("historical-run-context");
  if (expected) await waitFor(() => expect(shown(panel)).toEqual(expected.shown), WAIT);
  else await waitFor(() => expect(panel.querySelectorAll("tr[data-testid^='result-row-']").length).toBeGreaterThan(0));
  const out: Panel = { shown: shown(panel), csv: new Map(), details: new Map(), labelled: new Map() };
  for (const button of [...panel.querySelectorAll<HTMLElement>("[data-testid^='result-group-'][data-testid$='-export-csv']")]) {
    const testId = button.dataset.testid!;
    await attempt(async () => {
      const [before, begun] = [saved.length, streams.length];
      fireEvent.click(button);
      await waitFor(() => expect(within(panel).getByTestId(`${testId}-status`)).toHaveTextContent(/^Saved /), WAIT);
      if (saved.length > before) out.csv.set(testId, saved.at(-1)!);
      else {
        expect(streams.length, testId).toBe(begun + 1);
        const stream = streams.at(-1)!;
        out.csv.set(testId, [stream.begin.request.text, ...stream.appends.map((item: Json) => item.chunk.text)].join(""));
      }
    });
  }
  // The labelled rows (a headline or intensified row, wherever it is paged): each one's row cell.
  for (const id of labelIds) {
    await attempt(async () => {
      fireEvent.change(screen.getByTestId("result-filter-input"), { target: { value: id } });
      const cell = async () => (await within(panel).findByTestId(`result-row-${id}`, {}, WAIT)).querySelector("td")?.textContent ?? "";
      if (expected) await waitFor(async () => expect(await cell()).toBe(expected.labelled.get(id)), WAIT);
      out.labelled.set(id, await cell());
    });
  }
  // Result Detail's endpoint pair: its rows are admitted by the selected row's family, read from the
  // row semantics under the run's table (`readNativeEndpointPair`'s family gate natively); its CSV.
  for (const id of detailIds) {
    await attempt(async () => {
      fireEvent.change(screen.getByTestId("result-filter-input"), { target: { value: id } });
      fireEvent.click(await within(panel).findByTestId(`result-row-${id}`, {}, WAIT));
      const button = await within(panel).findByTestId("endpoint-pair-export-csv", {}, WAIT);
      const before = saved.length;
      fireEvent.click(button);
      await waitFor(() => expect(within(panel).getByTestId("endpoint-pair-export-csv-status")).toHaveTextContent(/^Saved /), WAIT);
      expect(saved.length).toBe(before + 1);
      out.details.set(id, saved.at(-1)!);
    });
  }
  cleanup();
  return out;
}

/** The first headline row and the first intensified row, if any. */
function labelRows(source: MechanicsResult): string[] {
  return [PREVIEW_MAXIMUM_KIND, PREVIEW_INTENSIFIED_KIND].flatMap((kind) => source.results.find((row) => row.kind === kind)?.id ?? []);
}

/** The rows whose Result Detail is compared: the first end_i row of a force, moment or stress
 * family under the run's table, whose endpoint pair the detail then shows. */
function detailRows(source: MechanicsResult, table: Readonly<{ id: string; sha256: string }> | null): string[] {
  const labels = semanticLabels(source, table);
  const row = source.results.find((item) => item.metadata?.location === "end_i" && ["force", "moment", "stress"].includes(labels.family(item)));
  return row ? [row.id] : [];
}

/** RT-L2 (DEL-04-07 REQ-009): the refusal of a pressure-2 result, and of a successor whose base is one. */
const PRESSURE_2_RETIRED_TEXT = "SOURCE_PRESSURE_2_RETIRED: pressure-2 results predate the pressure elongation setting and are no longer read; re-solve the model";
/** Whether a recorded statement's base identity statement (its first limitation) names a pressure-2 result. */
function pressure2Base(source: MechanicsResult): boolean {
  const first = (source.formulation_basis as { limitations?: unknown[] } | undefined)?.limitations?.[0];
  return typeof first === "string" && first.startsWith("3.0.0/exact_pressure_v3 under pressure-2 semantics.");
}

describe("a reopened run reads alike held whole and held natively (C1 review F2, F3)", () => {
  it.each(CASES)("$name", async (item) => {
    const stored = withQuality(item.stored(), item.quality);
    envelope = await storedEnvelope(stored);
    const whole = (await buildHistoricalRunContext(envelope as LocalProjectEnvelope))!;
    const table = item.table ? { id: stored.source.producer!.semantic_contract_id, sha256: item.table } : null;
    expect(whole.recordedTableRead).toEqual(table);
    (window as Json).__TAURI_INTERNALS__ = {};
    const native = (await buildNativeHistoricalRunContext((await openLocalProjectNative(null))!))!;
    const result = native.mechanicsResult!;
    expect(isNativeResult(result)).toBe(true);
    expect(native.recordedTableRead).toEqual(table);
    const route = displayRoute(whole.mechanicsResult!, table);
    expect(displayRoute(result, table)).toBe(route);

    const details = detailRows(whole.mechanicsResult!, table);
    const labelled = labelRows(whole.mechanicsResult!);
    const expected = await read(whole, null, details, labelled);
    const actual = await read(native, expected, details, labelled);
    expect(actual.shown).toEqual(expected.shown);
    expect([...actual.csv.keys()]).toEqual([...expected.csv.keys()]);
    for (const [testId, csv] of expected.csv) expect(actual.csv.get(testId), testId).toBe(csv);
    expect(actual.details).toEqual(expected.details);
    expect(actual.labelled).toEqual(expected.labelled);
    expect(expected.labelled.size).toBe(labelled.length);
    expect(expected.csv.size).toBeGreaterThan(0);
    // RT-L2 (DEL-04-07 REQ-009): a successor whose recorded base identity statement names a pressure-2
    // result predates the pressure-elongation setting and is refused by name with its message to
    // re-solve, held whole and held natively alike; it shows no Result Detail endpoint pair.
    const retired = pressure2Base(stored.source);
    if (retired) {
      for (const context of [whole, native]) expect(context.historicalReaderFindings).toContain(PRESSURE_2_RETIRED_TEXT);
      expect(expected.shown.standing).toContain("Needs recompute.");
      expect(expected.details.size).toBe(0);
    } else {
      expect(expected.details.size).toBe(details.length);
      if (!isRetainedRoute(route)) expect(details.length).toBe(1);
    }

    // The comparison is not vacuous: an ordinary route shows the run's own health, a retained route
    // its restriction; the invented case state reaches the badges and the CSV's health cells.
    if (retired) {
      expect(expected.shown.health).toBeNull();
    } else if (isRetainedRoute(route)) {
      expect(expected.shown.standing).toContain("needs recompute");
      expect(expected.shown.notices.map(([id]) => id)).toContain("results-notice-retained-precision-unvalidated");
      expect(expected.shown.health).toBeNull();
    } else {
      expect(expected.shown.health).not.toBeNull();
      expect(expected.shown.rows.some(([, cell]) => Object.values(NUMERICAL_HEALTH_STATE_TEXT).some((state) => cell.includes(state)))).toBe(true);
      if (item.quality !== "as recorded") {
        const state = NUMERICAL_HEALTH_STATE_TEXT[STATE[item.quality]];
        expect(expected.shown.health).toContain(state);
        expect([...expected.csv.values()].some((csv) => csv.includes(`,${state},`))).toBe(true);
      }
    }
    // F3: a preview statement's, and a reopened preview successor's, headline and intensified rows
    // carry their labels, natively too.
    if (route === "preview_physics" || route === "retained_preview_physics") {
      expect(labelled.length).toBeGreaterThan(0);
      for (const [id, cell] of expected.labelled) expect(cell.includes(N_HEADLINE) || cell.includes(N_INTENSIFIED), `${id}: ${cell}`).toBe(true);
    }
    // Nothing escalates: the native handle is never bound to a standing.
    const custody = nativeCustody(result as never);
    expect(custody.binding.standing).toBeNull();
    expect(numericalResultStanding(result).eligible).toBe(false);
    expect(custody.summary.header.standing?.eligible ?? false).toBe(false);
  }, 180_000);
});
