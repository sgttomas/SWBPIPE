// Issue 588: the engine publishes one fixed run id for distinct runs, so a results view's
// transient state (page, kind filter, navigator entity, selected row) is held with the run's view
// identity, never its run id. Swapping directly from run A to run B with the same run id, without
// passing through no result, shows run B's rows with run B's own default view; a harmless re-render
// of run A keeps run A's view. Invented and fixture data only.
import { cleanup, fireEvent, render, screen } from "@testing-library/react";
import { afterEach, describe, expect, it } from "vitest";
import { buildBundledReferenceContext, buildHistoricalRunContext, buildNativeHistoricalRunContext, buildNativeRecordedRunContext, buildRecordedRunContext, HistoricalRunPanel, type HistoricalRunContext } from "./HistoricalRunContext";
import type { AnalysisRunRead } from "../../services/analysisRunService";
import type { NativeProjectEnvelope, NativeRunRead } from "../../services/nativeReopen";
import type { BundledMechanicsReference } from "../../services/previewService";
import { ResultsPanel, resultViewIdentity, type ResultsViewRequest } from "./ResultsPanel";
import { SUPPORT_STATE_ROW_KIND } from "./keyResults";
import { keyResult, keyRows } from "../../test-support/keyResultsFixtures";
import { row } from "../../test-support/resultOverlayFixtures";
import { buildAnalysisRunV03 } from "../../services/analysisRunCompatibility";
import { buildCurrentSessionInputManifest } from "../../services/inputManifestService";
import { computeModelHash, computeProjectEnvelopeHash } from "../../services/hashService";
import savedModel from "../../../../../fixtures/model_operations/precision_connected_ui_model.json";
import savedResult from "../../../../../fixtures/results/preview_physics_connected_sparse.json";
import type { LocalProjectEnvelope, MechanicsResult, PreviewModel } from "../../types";

afterEach(cleanup);

const pageSummary = () => screen.getByTestId("result-page-summary").textContent ?? "";

/** A saved project's envelope over the connected fixture (89 rows: two pages of 50), with the first
 * row's value moved by `shift` so two saves are distinct runs under the same engine run id, in
 * the fixture's project or `project`. */
async function savedEnvelope(shift: number, project?: string): Promise<LocalProjectEnvelope> {
  const model = structuredClone(savedModel) as unknown as PreviewModel;
  if (project) model.project.id = project;
  const mechanics_result = structuredClone(savedResult) as unknown as MechanicsResult;
  if (project) mechanics_result.model_ref = project;
  mechanics_result.results[0].value += shift;
  const inputManifest = await buildCurrentSessionInputManifest({ model, solver: { solver_name: mechanics_result.producer!.component_name, solver_version: mechanics_result.producer!.component_version, solver_build_ref: "synthetic@1", solver_mode: "sparse_interactive", settings: {} }, active_rule_packs: [], external_assets: [] });
  const analysis_run = await buildAnalysisRunV03(mechanics_result, inputManifest);
  const model_hash = await computeModelHash(model);
  const payload = { model, mechanics_result, analysis_run, model_hash, editor_intents: [], proposal: null, selected_review_target: null };
  return { ...payload, project_envelope_hash: await computeProjectEnvelopeHash(payload) } as unknown as LocalProjectEnvelope;
}

/** The app's mounting of a historical run (`App.tsx`): keyed on its view identity. */
function Shown({ context }: { context: HistoricalRunContext }) {
  return <HistoricalRunPanel key={context.viewKey} context={context} />;
}

describe("a historical run's view identity", () => {
  it("is the same for the same saved run and distinct for two saved runs sharing a run id", async () => {
    const a = (await buildHistoricalRunContext(await savedEnvelope(0)))!;
    const again = (await buildHistoricalRunContext(await savedEnvelope(0)))!;
    const b = (await buildHistoricalRunContext(await savedEnvelope(1)))!;
    expect(a.runId).toBe(b.runId);
    expect(a.viewKey).toMatch(/^saved-result-sha256:[^:]+:[0-9a-f]{64}$/u);
    expect(again.viewKey).toBe(a.viewKey);
    expect(b.viewKey).not.toBe(a.viewKey);
  });

  it("is distinct for two projects storing byte-identical results", async () => {
    const a = (await buildHistoricalRunContext(await savedEnvelope(0)))!;
    const copied = (await buildHistoricalRunContext(await savedEnvelope(0, "project:copied")))!;
    expect(copied.viewKey).toMatch(/^saved-result-sha256:/u);
    expect(copied.viewKey).not.toBe(a.viewKey);
  });

  it("never keys a malformed saved result by its hash: two projects storing {} stay distinct", async () => {
    const malformed = async (project?: string) => {
      const saved = await savedEnvelope(0, project);
      (saved as unknown as { mechanics_result: unknown }).mechanics_result = {};
      return (await buildHistoricalRunContext(saved))!;
    };
    const a = await malformed();
    const b = await malformed("project:other");
    expect(a.mechanicsResult).toBeNull();
    expect(a.viewKey).toMatch(/^saved-envelope-payload-sha256:/u);
    expect(b.viewKey).toMatch(/^saved-envelope-payload-sha256:/u);
    expect(b.viewKey).not.toBe(a.viewKey);
    expect(a.viewKey).toBe((await malformed()).viewKey);
  });

  it("falls back to the stored envelope hash, then to a session-unique key, when nothing else identifies the run", async () => {
    const unhashable = async (envelopeHash: boolean) => {
      const saved = await savedEnvelope(0);
      (saved as unknown as { mechanics_result: unknown }).mechanics_result = "corrupt";
      // The envelope cannot be recomputed (a value the canonical hash refuses), so no payload hash is known.
      (saved.model as unknown as Record<string, unknown>).unhashable = BigInt(1);
      if (!envelopeHash) (saved as unknown as { project_envelope_hash: unknown }).project_envelope_hash = null;
      return (await buildHistoricalRunContext(saved))!;
    };
    const stored = await unhashable(true);
    expect(stored.envelopePayloadHash).toBeNull();
    expect(stored.viewKey).toMatch(/^saved-envelope-sha256:/u);
    const first = await unhashable(false);
    const second = await unhashable(false);
    expect(first.viewKey).toMatch(/^unidentified:\d+$/u);
    expect(second.viewKey).toMatch(/^unidentified:\d+$/u);
    expect(second.viewKey).not.toBe(first.viewKey);
  });

  it("keys a recorded run by its recording id", async () => {
    const read = (recording: string) => ({
      run: { recording_id: recording, run_record_hash: "sha256:record", state_id: "state:S", producer_run_id: savedResult.run_id },
      state: { state_id: "state:S", project_id: "project:P", model_hash: "sha256:model" },
      record: null, result_json: JSON.stringify(savedResult), input_manifest_canonical: "{}"
    }) as unknown as AnalysisRunRead;
    const a = await buildRecordedRunContext(read("R-1"));
    const b = await buildRecordedRunContext(read("R-2"));
    expect(a.runId).toBe(b.runId);
    expect(a.viewKey).toBe("recording:R-1");
    expect(b.viewKey).toBe("recording:R-2");
  });

  it("keys a native recorded run by its recording id (an empty one falls through), else its record, result or handle", () => {
    const read = (runRecording: string, sourceRecording: string | null, recordHash = "", resultPresent = true) => ({
      run: { recording_id: runRecording, run_record_hash: recordHash },
      restored: restored({ recording_id: sourceRecording }, resultPresent, true)
    }) as unknown as NativeRunRead;
    expect(buildNativeRecordedRunContext(read("R-1", null)).viewKey).toBe("recording:R-1");
    expect(buildNativeRecordedRunContext(read("", "R-2")).viewKey).toBe("recording:R-2");
    expect(buildNativeRecordedRunContext(read("", null, "sha256:rec")).viewKey).toBe("recorded-run-record-sha256:sha256%3Arec");
    expect(buildNativeRecordedRunContext(read("", null)).viewKey).toMatch(/^recorded-result-sha256:project%3AP:a{64}$/u);
    expect(buildNativeRecordedRunContext(read("", null, "", false)).viewKey).toBe("native-handle:restored-stored-result-1");
  });

  it("keys a native saved run without a renderable result by its envelope, else its handle", async () => {
    const opened = (payloadHash: string | null) => ({ stored_run: { state: "restored", ...restored({}, true, false, payloadHash) }, project_envelope_hash: null, model_hash: null }) as unknown as NativeProjectEnvelope;
    expect((await buildNativeHistoricalRunContext(opened("abc")))!.viewKey).toBe("saved-envelope-payload-sha256:abc");
    expect((await buildNativeHistoricalRunContext(opened(null)))!.viewKey).toBe("native-handle:restored-stored-result-1");
  });

  it("keys a bundled reference by its fixture, and never collides on an empty one", () => {
    const reference = (fixture_ref: string) => ({ model: savedModel, source: savedResult, provenance: { fixture_ref, notice: "" } }) as unknown as BundledMechanicsReference;
    expect(buildBundledReferenceContext(reference("fixture:a")).viewKey).toBe("bundled-reference:fixture%3Aa");
    const first = buildBundledReferenceContext(reference("")).viewKey;
    expect(first).toMatch(/^unidentified:/u);
    expect(buildBundledReferenceContext(reference("")).viewKey).not.toBe(first);
  });

  it("HistoricalRunPanel: run B, swapped in directly with run A's run id, opens on its own first page", async () => {
    const a = (await buildHistoricalRunContext(await savedEnvelope(0)))!;
    const b = (await buildHistoricalRunContext(await savedEnvelope(1)))!;
    const { rerender } = render(<Shown context={a} />);
    fireEvent.click(screen.getByTestId("next-result-page"));
    expect(pageSummary()).toMatch(/page 2 of 2$/u);
    rerender(<Shown context={a} />);
    expect(pageSummary()).toMatch(/page 2 of 2$/u);
    rerender(<Shown context={b} />);
    expect(pageSummary()).toMatch(/page 1 of 2$/u);
    expect(screen.getByTestId(`result-row-${savedResult.results[0].id}`)).toHaveTextContent(String(savedResult.results[0].value + 1));
  });

  it("HistoricalRunPanel: run A's selected row does not stay selected on run B, even without a remount", async () => {
    const a = (await buildHistoricalRunContext(await savedEnvelope(0)))!;
    const b = (await buildHistoricalRunContext(await savedEnvelope(1)))!;
    const id = savedResult.results[0].id;
    const { rerender } = render(<HistoricalRunPanel context={a} />);
    fireEvent.click(screen.getByTestId(`result-row-${id}`));
    expect(screen.getByTestId(`result-row-${id}`)).toHaveAttribute("aria-selected", "true");
    rerender(<HistoricalRunPanel context={a} />);
    expect(screen.getByTestId(`result-row-${id}`)).toHaveAttribute("aria-selected", "true");
    rerender(<HistoricalRunPanel context={b} />);
    expect(screen.getByTestId(`result-row-${id}`)).toHaveAttribute("aria-selected", "false");
  });
});

/** A restored stored run (the native reopen's reply), invented: one handle, one project. */
function restored(source: { recording_id?: string | null }, resultPresent: boolean, renderable: boolean, payloadHash: string | null = null) {
  return {
    handle: "restored-stored-result-1", origin: "restored_saved_run",
    source: { kind: "project", project_id: "project:P", recording_id: null, result_sha256: "a".repeat(64), record_sha256: null, ...source },
    result_bytes: 2, record_bytes: 0, result_present: resultPresent, record_present: false,
    findings: { findings: [], check_findings: [], recorded_table_read: null, historical_reader_findings: [], run_id: savedResult.run_id, renderable, mechanics: null, envelope_payload_hash: payloadHash, model_hash: null },
    recorded_dispatch: null, record_header: null, record_header_bytes: null
  };
}

describe("ResultsPanel's view identity", () => {
  /** Run A and run B: distinct results (one row's value apart) with the same run id. */
  function runs(): [MechanicsResult, MechanicsResult] {
    const rows = [...keyRows(), ...Array.from({ length: 120 }, (_, i) => row(`d:extra:${i}`, "displacement_magnitude", `node:X${i}`, i, "mm"))];
    const a = keyResult(rows);
    const b = keyResult(rows.map((item, i) => (i === 0 ? { ...item, value: item.value + 1 } : item)));
    expect(b.run_id).toBe(a.run_id);
    return [a, b];
  }
  const view = (result: MechanicsResult, request: ResultsViewRequest | null = null, runIdentityKey?: string) => (
    <ResultsPanel result={result} knowledge={null} analysisRun={null} selectedResultId={null} onSelectResult={() => undefined} viewRequest={request} runIdentityKey={runIdentityKey} />
  );

  it("derives a stable identity per result and never the bare run id", () => {
    const [a, b] = runs();
    expect(resultViewIdentity(a)).toBe(resultViewIdentity(a));
    expect(resultViewIdentity(b)).not.toBe(resultViewIdentity(a));
    expect(resultViewIdentity(a)).not.toContain(a.run_id);
    // The same run re-delivered as a new object (a reload or re-read) is the same view.
    expect(resultViewIdentity(structuredClone(a))).toBe(resultViewIdentity(a));
    expect(resultViewIdentity(a, "recording:R-1")).toBe(resultViewIdentity(b, "recording:R-1"));
    expect(resultViewIdentity(null)).toBeNull();
  });

  it("resets the page index when swapped directly from run A to run B, and keeps it across a re-render of run A", () => {
    const [a, b] = runs();
    const { rerender } = render(view(a));
    fireEvent.click(screen.getByTestId("next-result-page"));
    expect(pageSummary()).toMatch(/page 2 of 3$/u);
    rerender(view(a));
    expect(pageSummary()).toMatch(/page 2 of 3$/u);
    rerender(view(b));
    expect(pageSummary()).toMatch(/page 1 of 3$/u);
  });

  it("resets the kind filter when swapped directly to run B, even with run A's request still given", () => {
    const [a, b] = runs();
    const request: ResultsViewRequest = { serial: 1, runId: a.run_id, viewKey: resultViewIdentity(a)!, kind: SUPPORT_STATE_ROW_KIND, withheld: {} };
    const { rerender } = render(view(a, request));
    expect(screen.getByTestId("results-kind-filter")).toBeInTheDocument();
    rerender(view(a, request));
    expect(screen.getByTestId("results-kind-filter")).toBeInTheDocument();
    rerender(view(b, request));
    expect(screen.queryByTestId("results-kind-filter")).toBeNull();
    expect(pageSummary()).toMatch(/page 1 of 3$/u);
    // A new request, made for run B, applies to it.
    rerender(view(b, { ...request, serial: 2, viewKey: resultViewIdentity(b)! }));
    expect(screen.getByTestId("results-kind-filter")).toBeInTheDocument();
  });

  it("resets the navigator's entity when swapped directly to run B, and keeps it across a re-render of run A", () => {
    const [a, b] = runs();
    const { rerender } = render(view(a));
    const entity = () => screen.getByTestId("results-navigator-entity-position").textContent ?? "";
    expect(entity()).toMatch(/^None chosen; /u);
    fireEvent.click(screen.getByTestId("results-navigator-entity-next"));
    const chosen = entity();
    expect(chosen).not.toMatch(/^None chosen/u);
    rerender(view(a));
    expect(entity()).toBe(chosen);
    rerender(view(b));
    expect(entity()).toMatch(/^None chosen; /u);
  });

  it("never applies a request made for run A to run B, though B publishes A's run id and has seen no request", () => {
    const [a, b] = runs();
    const forA: ResultsViewRequest = { serial: 1, runId: a.run_id, viewKey: resultViewIdentity(a)!, kind: SUPPORT_STATE_ROW_KIND, withheld: { "d:A": "withheld for run A" } };
    render(view(b, forA));
    expect(screen.queryByTestId("results-kind-filter")).toBeNull();
    cleanup();
    render(view(a, forA));
    expect(screen.getByTestId("results-kind-filter")).toBeInTheDocument();
  });

  it("holds the view with a caller's stored identity: a new object for the same stored run keeps it, another run's resets it", () => {
    const [a, b] = runs();
    const { rerender } = render(view(a, null, "recording:R-1"));
    fireEvent.click(screen.getByTestId("next-result-page"));
    rerender(view({ ...a }, null, "recording:R-1"));
    expect(pageSummary()).toMatch(/page 2 of 3$/u);
    rerender(view(b, null, "recording:R-2"));
    expect(pageSummary()).toMatch(/page 1 of 3$/u);
  });
});
