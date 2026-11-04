// RT-A4 §10.4 display sweep (DEL-05-02 Design `absolute-sum-srss-combinations.md`, "Recorded results
// from before RT-A4"): a run recorded before RT-A4 is refused by every current reader (its
// LIMITATIONS[6] is the old table's), so any display surface that classifies it with a current-only
// reader shows a false current-only verdict. This sweep renders every surface a reopened pre-RT-A4
// run reaches (HistoricalRunPanel and the whole ResultsPanel subtree: rows, labels, notices, Result
// Detail and its endpoint pair, quantity readouts, every CSV export; the recorded comparison view
// and its CSV; the History run check's context) for each registry variant with a renderable
// record, and requires none of the current readers' refusal texts to appear. Fixtures:
// fixtures/persistence/recorded_pre_rt_a4_pf (verbatim history).
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { useState, type ReactElement } from "react";
import { cleanup, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import type { AnalysisRunEnvelope, LocalProjectEnvelope, MechanicsResult, PreviewModel } from "../../types";
import type { AnalysisRunRead } from "../../services/analysisRunService";
import type { ModelStateSummary } from "../../services/modelStateService";
import { computeModelHash, computeProjectEnvelopeHash } from "../../services/hashService";
import { DisplayUnitsProvider } from "../display-units";
import type { DisplayQuantityRequest, DisplayQuantityResult } from "../../services/displayQuantityService";
import { buildHistoricalRunContext, buildRecordedRunContext, HistoricalRunPanel, type HistoricalRunContext } from "./HistoricalRunContext";
import { displayRoute, readerDispatch, recordedReaderDispatch } from "./resultReader";
import { numericalResultStanding, recordedResultStanding } from "./resultStanding";
import { ResultsPanel } from "./ResultsPanel";
import { semanticDimension, semanticLabels } from "./resultSemantics";
import { knownSemanticNotices, resultRowLabel } from "./knownSemanticLimitations";
import { buildResultInterpretation } from "./resultInterpretation";
import { RunComparisonView } from "../comparison/RunComparisonView";
import { compareRunInputs, flattenRunResult, resultSemanticIdentity } from "../comparison/runComparison";
import { recordedRunSide, type RunComparisonSide } from "../comparison/runComparisonSides";

const saved: string[] = [];
vi.mock("../result-export/csvSave", () => ({
  saveCsvTable: vi.fn(async (input: { text: string; fileName: string }) => {
    saved.push(input.text);
    return { mode: "browser", file_name: input.fileName, byte_count: input.text.length };
  })
}));
afterEach(() => { cleanup(); saved.length = 0; });

const root = resolve(__dirname, "../../../../../");
const fixture = (path: string) => readFileSync(resolve(root, "fixtures/persistence/recorded_pre_rt_a4_pf", path), "utf8");
const json = <T,>(path: string) => JSON.parse(fixture(path)) as T;
type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any

/** What a current-only reader shows for a statement it refuses: the row category of a refused
 * row, the standing line of an unsupported route, the dispatch's refusal codes. */
const CURRENT_ONLY_VERDICTS = [
  "integrity_failure",
  "Unsupported numerical contract",
  "SOURCE_SEMANTIC_CONTRACT_UNSUPPORTED",
  "SOURCE_PREVIEW_PHYSICS_FORMULATION_BASIS",
  "RESULT_READER_UNAVAILABLE",
  "JSON_PARSE_REJECTED",
  "ANALYSIS_SOURCE_CONTRACT_VERSION_MISMATCH",
  "ANALYSIS_SEMANTIC_CONTRACT_MISMATCH",
  "SOURCE_RECORDED_BINDING",
  // A readout whose dimension the reader did not supply cannot convert.
  "catalog target for unknown",
] as const;
function expectNoCurrentOnlyVerdict(where: string, text: string) {
  for (const marker of CURRENT_ONLY_VERDICTS) expect(text.includes(marker), `${where}: "${marker}"`).toBe(false);
}

type Opened = { source: MechanicsResult; record: AnalysisRunEnvelope; model: PreviewModel };
/** `twin` is the statement's current counterpart in fixtures/results (the same rows and metadata;
 * only its formulation basis, and a successor's receipt, differ): the oracle of what the panel
 * shows for those rows under their own table. */
type Variant = { name: string; sha256: string; open: () => Opened; twin: (() => MechanicsResult) | null };
const results = (name: string, member?: string) => () => {
  const document = JSON.parse(readFileSync(resolve(root, "fixtures/results", name), "utf8"));
  return (member ? document[member] : document) as MechanicsResult;
};
const retained = (dir: string, name: string) => (): Opened => {
  const document = json<Json>(`${dir}/${name}.json`);
  return { source: document.source, record: json<AnalysisRunEnvelope>(`${dir}/${name}.analysis_run.json`), model: document.invocation.request.model };
};
const VARIANTS: Variant[] = [
  { name: "preview-physics-2", sha256: "1ccc9561cb4ce59d047f3dab7927a927c44a707efb3acb90e7b07a21b3bad0a2", open: () => {
    const transient = json<{ run_record: AnalysisRunEnvelope; input_manifest_canonical: string }>("preview_physics_2/transient_run_input.json");
    return { source: json("preview_physics_2/preview_physics_connected_sparse.json"), record: transient.run_record, model: JSON.parse(transient.input_manifest_canonical).model_basis.model_payload };
  }, twin: results("preview_physics_connected_sparse.json") },
  { name: "preview-physics-timoshenko-1", sha256: "a7b83c37a4af9bf70f56513f80883003a2a3860387319311ea2e967dd0cf764b", open: () => {
    // Main holds no model of this statement (fixture README, "Gap"): the reopened project carries a
    // model naming the statement's project and its load cases, so its model checks find what they find.
    const source = json<MechanicsResult>("preview_physics_timoshenko_1/preview_physics_timoshenko_straight_sparse.json");
    const record = json<AnalysisRunEnvelope>("preview_physics_timoshenko_1/preview_physics_timoshenko_straight_sparse.analysis_run.json");
    const cases = [...new Set(source.results.flatMap((row) => (row.basis_ref?.ref_type === "load_case" ? [row.basis_ref.ref_id] : [])))];
    const model = { schema_version: "0.9.0", project: { id: source.model_ref }, model_settings: { shear_model: "timoshenko_annulus_energy_v1" }, load_cases: cases.map((id) => ({ id })), diagnostics: [] } as unknown as PreviewModel;
    return { source, record, model };
  }, twin: results("preview_physics_timoshenko_straight_sparse.json") },
  { name: "preview-physics-retained-1", sha256: "b001070e5fedeeb101522b22ce0d61bb7cf85f82e86b8d851713f8d9fc8c4c46",
    open: retained("preview_physics_retained_1", "retained_precision_milestone_successor_sparse_interactive"),
    twin: results("retained_precision_milestone_successor_sparse_interactive.json", "source") },
  { name: "physics-retained-1 (main)", sha256: "cf29fd5fc5d61de0f1133e9668b05f3d62ae8f696944fe04de7470029f5357dd",
    open: retained("physics_retained_1/main", "retained_precision_exact_successor_sparse_interactive"),
    twin: results("retained_precision_exact_successor_sparse_interactive.json", "source") },
  { name: "physics-retained-1 (#101)", sha256: "045f6f0d8ce350246b21fb712d2dd14bf3a7f7f3690e66e062cbbf77d3696a3b",
    open: retained("physics_retained_1/pr101", "retained_precision_exact_successor_sparse_interactive"), twin: null },  { name: "physics-retained-1 (main, RT-A4)", sha256: "040ef9a613607cb6cb4aab6f34646481efd81e060681c364b72df4e129c8c034",
    open: retained("physics_retained_1/main_bbf384dec", "retained_precision_exact_successor_sparse_interactive"), twin: null },
  { name: "physics-retained-1 (main, before the combination certificate)", sha256: "85c2678a78e794b44761c11efb73c215645c7e2560d802ac8ad6075e7804c691",
    open: retained("physics_retained_1/main_252bef969", "retained_precision_exact_successor_sparse_interactive"), twin: null },
];

async function reopened(variant: Variant): Promise<HistoricalRunContext> {
  const { source, record, model } = variant.open();
  const payload = { model, mechanics_result: source, analysis_run: record, model_hash: await computeModelHash(model), editor_intents: [], proposal: null, selected_review_target: null };
  return (await buildHistoricalRunContext({ ...payload, project_envelope_hash: await computeProjectEnvelopeHash(payload as never) } as unknown as LocalProjectEnvelope))!;
}

/** Every SI readout converts (the converter is a stand-in), so a readout that does not is one
 * whose dimension is unknown. */
const converter = async (requests: DisplayQuantityRequest[]): Promise<DisplayQuantityResult[]> =>
  requests.map((request) => ({ id: request.id, status: "converted", value: request.value, unit: request.to_unit }));

/** One representative row of every kind, with every end_i row (so each endpoint pair shows). */
function representatives(source: MechanicsResult): MechanicsResult["results"] {
  const seen = new Set<string>();
  return source.results.filter((row) => {
    const key = `${row.kind}\u0000${row.metadata?.location === "end_i" ? "end_i" : ""}`;
    if (seen.has(key)) return false;
    seen.add(key);
    return true;
  });
}

async function exportAll(container: HTMLElement): Promise<string[]> {
  const before = saved.length;
  const buttons = [...container.querySelectorAll<HTMLButtonElement>("button[data-testid$='-export-csv']")];
  for (const button of buttons) fireEvent.click(button);
  await waitFor(() => expect(saved.length).toBe(before + buttons.length));
  return saved.slice(before);
}

/** A current result in the results panel, as the session shows one (no recorded table). */
function CurrentPanel({ result }: { result: MechanicsResult }) {
  const [selected, setSelected] = useState<string | null>(null);
  return <ResultsPanel result={result} knowledge={null} analysisRun={null} selectedResultId={selected} onSelectResult={setSelected} />;
}

type Walk = { rows: Map<string, { cell: string; detail: string }>; notices: string[]; texts: string[] };
/** Selects each representative row (Result Detail, its endpoint pair and its CSV), then exports
 * every results table; the panel's text at each step, and every CSV, are collected. */
async function walk(panelElement: ReactElement, source: MechanicsResult, labels: ReturnType<typeof semanticLabels>): Promise<Walk> {
  const view = render(<DisplayUnitsProvider initialPreference="SI" converter={converter}>{panelElement}</DisplayUnitsProvider>);
  const panel = screen.getByTestId("results-panel");
  const out: Walk = { rows: new Map(), notices: [...panel.querySelectorAll("[data-testid^='results-notice-']")].map((n) => n.textContent ?? ""), texts: [] };
  const filter = screen.getByTestId("result-filter-input");
  for (const row of representatives(source)) {
    fireEvent.change(filter, { target: { value: row.id } });
    const cell = within(panel).getByTestId(`result-row-${row.id}`);
    fireEvent.click(cell);
    const detail = screen.getByTestId("result-detail-panel");
    expect(screen.getByTestId("selected-result-id")).toHaveTextContent(row.id);
    const family = labels.family(row);
    // Displacement and stress rows carry their dimension: their readouts convert.
    if (family === "displacement" || family === "stress") {
      expect(labels.dimension(row), row.id).not.toBeNull();
      await waitFor(() => expect(cell.querySelector("[data-display-status]")!.getAttribute("data-display-status"), `${row.id} row`).toBe("converted"));
      await waitFor(() => expect(detail.querySelector("[data-display-status]")!.getAttribute("data-display-status"), `${row.id} detail`).toBe("converted"));
    }
    out.rows.set(row.id, { cell: cell.textContent ?? "", detail: detail.textContent ?? "" });
    out.texts.push(view.container.textContent ?? "", ...await exportAll(detail));
  }
  fireEvent.change(filter, { target: { value: "" } });
  fireEvent.change(screen.getByTestId("result-page-size"), { target: { value: "200" } });
  const csv = await exportAll(panel);
  expect(csv.length).toBeGreaterThan(0);
  out.texts.push(view.container.textContent ?? "", ...csv);
  cleanup();
  return out;
}

describe("a reopened run recorded before RT-A4 shows no current-only verdict on any display surface", () => {
  it.each(VARIANTS)("$name: HistoricalRunPanel, every row's detail, readouts and CSV exports", async (variant) => {
    const context = await reopened(variant);
    const source = context.mechanicsResult!;
    const binding = { id: source.producer!.semantic_contract_id, sha256: variant.sha256 };
    expect(context.recordedTableRead).toEqual(binding);
    expect(recordedReaderDispatch(source, binding).ok).toBe(true);
    // The standing stays historical, labelled with its table; the retained restriction stays.
    render(<HistoricalRunPanel context={context} />);
    const standing = screen.getByTestId("numerical-result-standing").textContent ?? "";
    expect(standing.startsWith(`As recorded under ${binding.id} (sha256 ${binding.sha256}); historical evidence only, not a current standing. `)).toBe(true);
    if (variant.name.includes("retained")) expect(standing).toContain("needs recompute");
    cleanup();
    const recorded = await walk(<HistoricalRunPanel context={context} />, source, semanticLabels(source, binding));
    recorded.texts.forEach((text, i) => expectNoCurrentOnlyVerdict(`${variant.name} [${i}]`, text));
    // Under its own table the statement reads as its current twin does: the same row cells
    // (category, row label, health), Result Detail (family, readouts, endpoint pair) and notices.
    if (variant.twin) {
      const twin = variant.twin();
      const current = await walk(<CurrentPanel result={twin} />, twin, semanticLabels(twin));
      expect(recorded.rows).toEqual(current.rows);
      expect(recorded.notices).toEqual(current.notices);
    }
    // Nothing here widens current admission: the current reader still does not read the statement
    // under its recorded table, and a refused statement stays refused.
    expect(readerDispatch(source).binding?.sha256 ?? null).not.toBe(variant.sha256);
    if (!variant.name.startsWith("physics-retained-1")) expect(numericalResultStanding(source).eligible).toBe(false);
  });
});

describe("the recorded table is the only authority the display paths take", () => {
  it.each(VARIANTS)("$name: an unknown or refused binding never falls back to current semantics", (variant) => {
    const { source } = variant.open();
    const id = source.producer!.semantic_contract_id;
    const other = VARIANTS.find((entry) => entry.open().source.producer!.semantic_contract_id !== id)!;
    const row = source.results.find((r) => r.kind === "global_nodal_displacement_x") ?? source.results[0];
    for (const bad of [{ id, sha256: "0".repeat(64) }, { id: other.open().source.producer!.semantic_contract_id, sha256: other.sha256 }, { id: "openpipestress.result_semantics/0.3.0/pressure-2", sha256: variant.sha256 }]) {
      expect(displayRoute(source, bad)).toBe("unsupported");
      expect(knownSemanticNotices(source, bad)).toEqual([]);
      expect(resultRowLabel(row, source, bad)).toBeNull();
      const labels = semanticLabels(source, bad);
      expect(source.results.every((r) => labels.category(r) === "integrity_failure" && labels.dimension(r) === null)).toBe(true);
      expect(buildResultInterpretation({ result: source, resultId: row.id, knowledge: null, analysisRun: null, recordedTableRead: bad })!.family).toBe("integrity_failure");
      expect(semanticDimension(row, source, bad)).toBeNull();
    }
    // Without a table every one of them is the current reader's, unchanged.
    expect(displayRoute(source)).toBe(readerDispatch(source).route);
    expect(knownSemanticNotices(source, null)).toEqual(knownSemanticNotices(source));
    expect(semanticDimension(row, source, null)).toBe(semanticLabels(source).dimension(row));
  });
});

// main's recorded revision of its preview-physics-2 run, as the store returns it.
function witnessRead(): AnalysisRunRead {
  const transient = json<{ input_manifest_canonical: string }>("preview_physics_2/transient_run_input.json");
  const witness = json<Json>("preview_physics_2/recorded_run_witness.json");
  const projectId = JSON.parse(transient.input_manifest_canonical).model_basis.model_ref as string;
  const state = {
    project_id: projectId, state_id: witness.state_id, state_kind: "post_solve_snapshot", created_at: witness.created_at, parent_state_id: null,
    model_hash: witness.state_model_hash, record_hash: "r", labels_updated_at: witness.created_at, child_state_ids: [], recorded_run_count: 1,
    analysis_status: ["HUMAN_REVIEW_REQUIRED"], state_name: "After solve", tags: [], notes: [], assumptions: []
  } as ModelStateSummary;
  return {
    run: {
      project_id: projectId, recording_id: witness.recording_id, state_id: witness.state_id, producer_run_id: witness.recorded_record.analysis_run.run_id,
      created_at: witness.created_at, run_record_hash: witness.run_record_hash, result_hash: "r", labels_updated_at: witness.created_at,
      analysis_status: witness.recorded_record.analysis_run.analysis_status, run_name: "Witness run", tags: [], notes: [], assumptions: []
    },
    state, record: structuredClone(witness.recorded_record), result_json: fixture("preview_physics_2/preview_physics_connected_sparse.json"),
    input_manifest_canonical: transient.input_manifest_canonical
  } as AnalysisRunRead;
}

const source = (read: AnalysisRunRead) => JSON.parse(read.result_json) as MechanicsResult;
/** Every compared physical-quantity row keeps the family and dimension its table gives it. */
function expectPhysicalRowsIdentified(comparison: ReturnType<typeof compareRunInputs>, result: MechanicsResult, binding?: { id: string; sha256: string }) {
  const labels = semanticLabels(result, binding ?? { id: result.producer!.semantic_contract_id, sha256: VARIANTS[0].sha256 });
  const byId = new Map(result.results.map((row) => [row.id, row]));
  const physical = comparison.pairs.filter((pair) => labels.category(byId.get(pair.identity.result_id)!) === "physical_quantity");
  expect(physical.length).toBeGreaterThan(0);
  for (const pair of physical) {
    expect(pair.identity.family, pair.identity.result_id).not.toBeNull();
    expect(pair.identity.dimension, pair.identity.result_id).not.toBeNull();
  }
}

async function renderComparison(left: RunComparisonSide, right: RunComparisonSide, projectId: string) {
  const comparison = compareRunInputs(left.input, right.input);
  const view = render(<RunComparisonView result={{ projectId, left, right, comparison } as never} onClose={() => {}} />);
  return { comparison, view, csv: await exportAll(view.container) };
}

describe("a recorded comparison of two pre-RT-A4 runs under the same table shows no current-only verdict", () => {
  it("preview-physics-2: main's recorded run against itself, through the store's read (History run check and compare)", async () => {
    const read = witnessRead();
    // The History panel's run check is this context: its findings carry no current-only verdict.
    const check = await buildRecordedRunContext(read);
    expectNoCurrentOnlyVerdict("run check findings", [...check.findings, ...check.historicalReaderFindings].join(" "));
    const left = await recordedRunSide(read), right = await recordedRunSide(witnessRead());
    expect(left.recordedTableRead).toEqual({ id: left.recordedTableRead!.id, sha256: VARIANTS[0].sha256 });
    const { comparison, view, csv } = await renderComparison(left, right, read.state.project_id);
    expect(comparison.run_gate).toEqual([]);
    expect(comparison.summary.compared).toBeGreaterThan(0);
    expectPhysicalRowsIdentified(comparison, source(read));
    [view.container.textContent ?? "", ...csv].forEach((text, i) => expectNoCurrentOnlyVerdict(`comparison [${i}]`, text));
  });

  it.each(VARIANTS.slice(1))("$name: two reopened records under the same table", async (variant) => {
    const { source, model } = variant.open();
    const binding = { id: source.producer!.semantic_contract_id, sha256: variant.sha256 };
    const standing = recordedResultStanding(source, binding, model);
    const input = { rows: flattenRunResult(source, binding), standing, semantic_identity: resultSemanticIdentity(source, binding), context: {}, diagnostics: [] } as never;
    const side = (id: string) => ({ kind: "recorded", id, description: id, createdAt: null, state: null, standing, recordedTableRead: binding, input } as unknown as RunComparisonSide);
    const { comparison, view, csv } = await renderComparison(side("a"), side("b"), source.model_ref);
    expectPhysicalRowsIdentified(comparison, source, binding);
    if (variant.name.includes("retained")) {
      // The retained restriction stays: no live registration, so needs recompute and no row compared.
      expect(standing.eligible).toBe(false);
      expect(comparison.run_gate).toContain("left_run_needs_recompute");
      expect(comparison.summary.compared).toBe(0);
    } else expect(comparison.summary.compared).toBeGreaterThan(0);
    // Historical against current stays refused.
    const mixed = compareRunInputs(input, { ...(input as Json), semantic_identity: resultSemanticIdentity(source) });
    expect(mixed.run_gate).toContain("semantic_identity_differs");
    [view.container.textContent ?? "", ...csv].forEach((text, i) => expectNoCurrentOnlyVerdict(`${variant.name} comparison [${i}]`, text));
  });
});
