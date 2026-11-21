// The displays that read the session's rule-check run (the stress and ratio table, the key results
// and the viewport's rule ratios) on a native result (C1 custody): the rows the run binds are read
// from the held result by id, and no full copy of the result enters the webview. Each display shows
// what it shows on the full rows (`resultSummaryParity.test.ts` holds the tables equal). The native
// solve's readers and the row lookups answer from the actual Rust commands' recorded replies
// (`nativeReplayStore.ts`). Unit transport replay; NOT native UI qualification. Invented data only.
import { act, cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, beforeEach, expect, it, vi } from "vitest";

const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../../test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));

import { awaitPreviewMechanicsJob, readNativeFreshResult, resetSessionResultCustodyForTests, startPreviewMechanicsJob } from "../../services/previewService";
import { nativeRowKey, resetNativeResultsForTests, type NativeResult } from "../../services/nativeResult";
import { createNativeMechanicsReplay, nativeMechanicsReplayPair, REPLAY_COMMANDS } from "../../test/nativeMechanicsReplay";
import { healthStanding } from "../../test-support/resultOverlayFixtures";
import { ruleCheckRunOn } from "../../test-support/ruleCheckRunOn";
import { DisplayUnitsProvider } from "../display-units";
import { PipeViewport } from "../viewport/PipeViewport";
import type { ViewportViewCommand } from "../viewport/viewportSelection";
import { buildKeyResults } from "./keyResults";
import { KeyResultsPanel } from "./KeyResultsPanel";
import { STRESS_RATIO_RUN_NOT_THIS_RESULT, buildStressRatioTable, stressRatioLocationText } from "./stressRatioTable";
import { STRESS_RATIO_NO_LOCATION } from "./stressRatioTableCsv";
import { StressRatioTablePanel } from "./StressRatioTablePanel";
import type { MechanicsResult } from "../../types";

/** A one-entry catalog: the display-units provider reads one; no conversion is exercised here. */
const UNIT_CATALOG = {
  schema_version: "1.0.0", catalog_id: "unit-system:invented-test", decision_basis: "DEC-018", calculation_basis: "si_canonical",
  storage_convention: "entered_units_preserved", entry_count: 1,
  entries: [{ unit_id: "unit:ratio", symbol: "ratio", dimension_id: "dimensionless", canonical: true, transform_kind: "identity", factor_representation: "1", offset_representation: null, provenance: "si_canonical", review_status: "accepted" }]
};
const pair = nativeMechanicsReplayPair("sparse_interactive", { profile: "preview" });
const full = JSON.parse(pair.bytes) as MechanicsResult;
const standing = healthStanding({ eligible: true });
const commands = () => invokeMock.mock.calls.map(([command]) => command as string);
const sent = (command: string) => commands().filter((item) => item === command).length;

beforeEach(() => { resetSessionResultCustodyForTests(); resetNativeResultsForTests(); });
afterEach(() => {
  cleanup();
  invokeMock.mockReset();
  delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
});

async function nativeResult(): Promise<NativeResult> {
  const replay = createNativeMechanicsReplay({ profile: "preview" });
  invokeMock.mockImplementation((command: string, args?: unknown, options?: unknown) => {
    if (command === "get_unit_catalog") return Promise.resolve(UNIT_CATALOG);
    if (REPLAY_COMMANDS.includes(command)) return replay.invoke(command, args, options);
    return Promise.reject(new Error(`UNEXPECTED_COMMAND: ${command}`));
  });
  (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
  const started = await startPreviewMechanicsJob(pair.model, "sparse_interactive", { custody: "native" });
  if (started.mode !== "backend_job") throw new Error("not a backend job");
  const terminal = await awaitPreviewMechanicsJob(started.job_id);
  return (await readNativeFreshResult(started.job_id, terminal.result_bytes ?? 0, { standingModel: pair.model, summaryModel: pair.model })).result;
}

const keySpies = () => ({
  onEverySolveChange: vi.fn(), onShowRatioContour: vi.fn(), onShowDeflectedShape: vi.fn(), onShowSupport: vi.fn(),
  onShowContactStatus: vi.fn(), onOpenTable: vi.fn(), onBackToEditing: vi.fn(), onOpenRuleChecks: vi.fn(), onClose: vi.fn()
});

it("the stress and ratio table, the key results and the viewport's rule ratios read the run's rows by id, as on the full rows", async () => {
  const result = await nativeResult();
  const run = ruleCheckRunOn(full);
  const boundIds = run.result.checks.flatMap((outcome) => outcome.bound_inputs.flatMap((input) => (input.result_id ? [input.result_id] : [])));
  expect(boundIds.length).toBeGreaterThan(0);
  const expected = buildStressRatioTable({ model: pair.model, result: full, standing, ruleCheckRun: run });
  expect(expected.run.state).toBe("used");
  const onSelectLocation = vi.fn();
  const command = { current: null as ((value: ViewportViewCommand) => void) | null };
  render(
    <DisplayUnitsProvider>
      <StressRatioTablePanel model={pair.model} result={result} standing={standing} ruleCheckRun={run} onSelectLocation={onSelectLocation} />
      <KeyResultsPanel model={pair.model} result={result} standing={standing} ruleCheckRun={run} everySolve={false} {...keySpies()} />
      <PipeViewport model={pair.model} result={result} resultStanding={standing} ruleCheckRun={run} selection={{ type: "project", id: "project:invented" }} onSelect={vi.fn()} onQueueIntent={vi.fn()} viewCommandRef={command} />
    </DisplayUnitsProvider>
  );

  // The table: the run is used, and the shown rule group's rows are located as on the full rows.
  await waitFor(() => expect(screen.getByTestId("stress-ratio-run")).toHaveTextContent(expected.run.note));
  const group = expected.groups[0];
  expect(group.source).toBe("rule_category");
  for (const row of group.rows) {
    const text = row.locations.length > 0 ? row.locations.map(stressRatioLocationText).join("; ") : STRESS_RATIO_NO_LOCATION;
    expect(screen.getByRole("button", { name: `Highlight ${text}, ${row.caseLabel}` })).toBeInTheDocument();
  }
  // A bound row's location names the row by the native table's row key, as its stress rows do.
  const boundRow = group.rows.find((row) => row.target.resultId !== null)!;
  const boundText = boundRow.locations.map(stressRatioLocationText).join("; ");
  fireEvent.click(screen.getByRole("button", { name: `Highlight ${boundText}, ${boundRow.caseLabel}` }));
  const ordinal = full.results.map((row) => row.id).lastIndexOf(boundRow.target.resultId!);
  expect(onSelectLocation).toHaveBeenCalledWith({ resultId: nativeRowKey(ordinal), entityRef: boundRow.target.entityRef });

  // The key results: the ratio section the builder forms on the full rows.
  const expectedKey = buildKeyResults({ model: pair.model, result: full, standing, ruleCheckRun: run });
  expect(expectedKey.ratio.state).toBe("used");
  expect(screen.getByTestId("key-results-ratio-note")).toHaveAttribute("data-ratio-state", "used");
  expect(screen.getByTestId("key-results-ratio-note")).toHaveTextContent(expectedKey.ratio.note!);

  // The viewport: the run's ratio quantities are offered on the result colour.
  const ratioQuantity = `rule-ratio:${group.id}`;
  act(() => command.current!({ type: "show-result-colour", quantityId: ratioQuantity, caseKey: group.rows[0].case!.key }));
  await waitFor(() => expect(screen.getByTestId("viewport-overlay-quantity")).toHaveValue(ratioQuantity));

  // The rows were read by id; no full copy was read or taken.
  expect(sent("lookup_preview_mechanics_job_rows")).toBeGreaterThanOrEqual(1);
  expect(sent("read_preview_mechanics_job_rows_all")).toBe(0);
  expect(sent("take_preview_mechanics_job_result")).toBe(0);
});

it("a run binding a row the held result does not hold is read as another result's run", async () => {
  const result = await nativeResult();
  const run = ruleCheckRunOn(full, "invented:not-a-row-of-this-result");
  expect(buildStressRatioTable({ model: pair.model, result: full, standing, ruleCheckRun: run }).run.state).toBe("not_this_result");
  render(<StressRatioTablePanel model={pair.model} result={result} standing={standing} ruleCheckRun={run} />);
  await waitFor(() => expect(screen.getByTestId("stress-ratio-run")).toHaveTextContent(STRESS_RATIO_RUN_NOT_THIS_RESULT));
  expect(sent("read_preview_mechanics_job_rows_all")).toBe(0);
});
