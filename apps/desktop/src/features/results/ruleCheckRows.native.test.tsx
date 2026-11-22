// The displays that read the session's rule-check run (the stress and ratio table, the key results
// and the viewport's rule ratios) on a native result (C1 custody): the rows the run binds are read
// from the held result by id, and no full copy of the result enters the webview. Each display shows
// what it shows on the full rows (`resultSummaryParity.test.ts` holds the tables equal). The native
// solve's readers and the row lookups answer from the actual Rust commands' recorded replies
// (`nativeReplayStore.ts`). Unit transport replay; NOT native UI qualification. Invented data only.
import { act, cleanup, fireEvent, render, renderHook, screen, waitFor } from "@testing-library/react";
import { afterEach, beforeEach, expect, it, vi } from "vitest";

const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../../test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));

import { awaitPreviewMechanicsJob, disposeDeliveredMechanicsResult, readNativeFreshResult, resetSessionResultCustodyForTests, startPreviewMechanicsJob } from "../../services/previewService";
import { lookupNativeRows, NATIVE_RESULT_DISPOSED, nativeRowKey, resetNativeResultsForTests, type NativeResult } from "../../services/nativeResult";
import { RESULT_DETAIL_MAX_KEYS, type Lookup, type LookupRequest } from "../../services/resultDetail";
import { createNativeMechanicsReplay, nativeMechanicsReplayPair, REPLAY_COMMANDS } from "../../test/nativeMechanicsReplay";
import { healthStanding } from "../../test-support/resultOverlayFixtures";
import { ruleCheckRunOn } from "../../test-support/ruleCheckRunOn";
import { DisplayUnitsProvider } from "../display-units";
import { displayQuantityParts } from "../report/reportQuantityDisplay";
import { PipeViewport } from "../viewport/PipeViewport";
import { availableResultColourQuantities, buildResultColourOverlay, resultOverlayCases } from "../viewport/resultColourOverlay";
import { QUANTITY_UNAVAILABLE_SUFFIX } from "../viewport/ResultOverlayLegend";
import type { ViewportViewCommand } from "../viewport/viewportSelection";
import { buildKeyResults } from "./keyResults";
import { KeyResultsPanel } from "./KeyResultsPanel";
import { ruleCheckBoundResultIds as ruleCheckBoundIds, STRESS_RATIO_NO_RESULT, STRESS_RATIO_RULE_ROWS_PENDING, STRESS_RATIO_RUN_NOT_THIS_RESULT, buildStressRatioTable, stressRatioLocationText, stressRatioRuleRowsUnreadText } from "./stressRatioTable";
import { STRESS_RATIO_NO_LOCATION, STRESS_RATIO_NO_RATIO } from "./stressRatioTableCsv";
import { StressRatioTablePanel } from "./StressRatioTablePanel";
import { collectRuleCheckRows, readRuleCheckRows, RULE_CHECK_ROWS_KEPT_LIMIT_BYTES, useRuleCheckRows, utf8Length } from "./useRuleCheckRows";
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

async function nativeResult(profile: "preview" | "pressure" = "preview"): Promise<NativeResult> {
  const replay = createNativeMechanicsReplay({ profile });
  const solved = profile === "preview" ? pair : nativeMechanicsReplayPair("sparse_interactive", { profile });
  invokeMock.mockImplementation((command: string, args?: unknown, options?: unknown) => {
    if (command === "get_unit_catalog") return Promise.resolve(UNIT_CATALOG);
    if (REPLAY_COMMANDS.includes(command)) return replay.invoke(command, args, options);
    return Promise.reject(new Error(`UNEXPECTED_COMMAND: ${command}`));
  });
  (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
  const started = await startPreviewMechanicsJob(solved.model, "sparse_interactive", { custody: "native" });
  if (started.mode !== "backend_job") throw new Error("not a backend job");
  const terminal = await awaitPreviewMechanicsJob(started.job_id);
  return (await readNativeFreshResult(started.job_id, terminal.result_bytes ?? 0, { standingModel: solved.model, summaryModel: solved.model })).result;
}

/** The run's first rule category and the case of its first row. On this result the run's rows are
 * held by the numerical standing, so a category forms no ratio; the displays are held to what the
 * builders form on the full rows, whatever that is. */
function ruleGroup(table: ReturnType<typeof buildStressRatioTable>) {
  const group = table.groups.find((item) => item.source === "rule_category")!;
  return { group, caseKey: group.rows.find((row) => row.case)!.case!.key };
}

const keySpies = () => ({
  onEverySolveChange: vi.fn(), onShowRatioContour: vi.fn(), onShowDeflectedShape: vi.fn(), onShowSupport: vi.fn(),
  onShowContactStatus: vi.fn(), onOpenTable: vi.fn(), onBackToEditing: vi.fn(), onOpenRuleChecks: vi.fn(), onClose: vi.fn()
});

/** The viewport's quantity selector shows `id` selected, as an unavailable quantity. */
function expectRatioUnavailable(id: string) {
  const select = screen.getByTestId("viewport-overlay-quantity") as HTMLSelectElement;
  expect(select).toHaveValue(id);
  expect(select.selectedOptions[0]).toHaveTextContent(QUANTITY_UNAVAILABLE_SUFFIX.trim());
  expect(select.selectedOptions[0]).toBeDisabled();
}

const viewport = (run: ReturnType<typeof ruleCheckRunOn>, result: NativeResult, command: { current: ((value: ViewportViewCommand) => void) | null }) => (
  <PipeViewport model={pair.model} result={result} resultStanding={standing} ruleCheckRun={run} selection={{ type: "project", id: "project:invented" }} onSelect={vi.fn()} onQueueIntent={vi.fn()} viewCommandRef={command} />
);

it("the stress and ratio table, the key results and the viewport's rule ratios read the run's rows by id, as on the full rows", async () => {
  const result = await nativeResult();
  const run = ruleCheckRunOn(full);
  const boundIds = run.result.checks.flatMap((outcome) => outcome.bound_inputs.flatMap((input) => (input.result_id ? [input.result_id] : [])));
  expect(boundIds.length).toBeGreaterThan(0);
  const expected = buildStressRatioTable({ model: pair.model, result: full, standing, ruleCheckRun: run });
  expect(expected.run.state).toBe("used");
  const onSelectLocation = vi.fn();
  const command = { current: null as ((value: ViewportViewCommand) => void) | null };
  const tree = (selectedResultId: string | null) => (
    <DisplayUnitsProvider>
      <StressRatioTablePanel model={pair.model} result={result} standing={standing} ruleCheckRun={run} selectedResultId={selectedResultId} onSelectLocation={onSelectLocation} />
      <KeyResultsPanel model={pair.model} result={result} standing={standing} ruleCheckRun={run} everySolve={false} {...keySpies()} />
      {viewport(run, result, command)}
    </DisplayUnitsProvider>
  );
  const { rerender } = render(tree(null));

  // The table: the run is used, and the shown rule group's rows are located (members and stations)
  // and valued as on the full rows.
  await waitFor(() => expect(screen.getByTestId("stress-ratio-run")).toHaveTextContent(expected.run.note));
  const { group, caseKey } = ruleGroup(expected);
  fireEvent.change(screen.getByTestId("stress-ratio-group"), { target: { value: group.id } });
  let valued = 0;
  for (const row of group.rows) {
    const text = row.locations.length > 0 ? row.locations.map(stressRatioLocationText).join("; ") : STRESS_RATIO_NO_LOCATION;
    expect(screen.getByRole("button", { name: `Highlight ${text}, ${row.caseLabel}` })).toBeInTheDocument();
    // A ranked row shows its ratio (or why none is formed); a pinned row its incomplete or withheld reason.
    const cell = screen.queryByTestId(`stress-ratio-value-${row.key}`);
    const reason = screen.queryByTestId(`stress-ratio-reason-${row.key}`);
    if (cell) expect(cell).toHaveTextContent(row.outcome!.ratio !== null ? displayQuantityParts(row.outcome!.ratio, "").number : row.outcome!.ratioNote ?? STRESS_RATIO_NO_RATIO);
    else expect(reason).toHaveTextContent((row.outcome!.status === "RULE_INPUTS_INCOMPLETE" ? row.outcome!.incompleteReason : row.withheldReason) ?? "");
    valued += 1;
  }
  expect(valued).toBe(group.rows.length);
  // A bound row's location selects the row by its published id, and the selection highlights it.
  const boundRow = group.rows.find((row) => row.target.resultId !== null)!;
  const boundText = boundRow.locations.map(stressRatioLocationText).join("; ");
  fireEvent.click(screen.getByRole("button", { name: `Highlight ${boundText}, ${boundRow.caseLabel}` }));
  await waitFor(() => expect(onSelectLocation).toHaveBeenCalledWith({ resultId: boundRow.target.resultId, entityRef: boundRow.target.entityRef }));
  rerender(tree(boundRow.target.resultId));
  await waitFor(() => expect(screen.getByTestId(`stress-ratio-row-${boundRow.key}`)).toHaveAttribute("aria-current", "true"));

  // The key results: the ratio section the builder forms on the full rows.
  const expectedKey = buildKeyResults({ model: pair.model, result: full, standing, ruleCheckRun: run });
  expect(expectedKey.ratio.state).toBe("used");
  expect(screen.getByTestId("key-results-ratio-note")).toHaveAttribute("data-ratio-state", "used");
  expect(screen.getByTestId("key-results-ratio-note")).toHaveTextContent(expectedKey.ratio.note!);

  // The viewport: the run's ratio, as the builders form it on the full rows (offered and drawn, or
  // offered and held, or not offered and shown unavailable).
  const ratioQuantity = `rule-ratio:${group.id}`;
  const overlayCase = resultOverlayCases(pair.model, full).find((item) => item.key === caseKey)!;
  const offered = availableResultColourQuantities(pair.model, full, expected).find((item) => item.id === ratioQuantity) ?? null;
  act(() => command.current!({ type: "show-result-colour", quantityId: ratioQuantity, caseKey: overlayCase.key }));
  await waitFor(() => expect(screen.getByTestId("viewport-overlay-quantity")).toHaveValue(ratioQuantity));
  if (!offered) expectRatioUnavailable(ratioQuantity);
  else {
    const expectedColour = buildResultColourOverlay({ model: pair.model, result: full, standing, overlayCase, ratioTable: expected, quantity: offered });
    await waitFor(() => expect(screen.getByTestId("viewport-result-colour-legend")).toHaveAttribute("data-state", expectedColour.state));
    if (expectedColour.state === "available") {
      expect(screen.getByTestId("viewport-result-colour-min")).toHaveAttribute("title", String(expectedColour.minimum));
      expect(screen.getByTestId("viewport-result-colour-max")).toHaveAttribute("title", String(expectedColour.maximum));
    } else {
      expect(screen.getByTestId("viewport-result-colour-reason")).toHaveTextContent(expectedColour.reason!);
    }
    expect(screen.getByTestId("viewport-result-colour-no-value")).toHaveTextContent(`No value: ${expectedColour.noValueCount} of ${expectedColour.memberCount}`);
  }

  // The rows were read by id; no full copy was read or taken.
  expect(sent("lookup_preview_mechanics_job_rows")).toBeGreaterThanOrEqual(1);
  expect(sent("read_preview_mechanics_job_rows_all")).toBe(0);
  expect(sent("take_preview_mechanics_job_result")).toBe(0);
});

it("while the run's rows are read, the displays say so, keep the result's own stresses and draw no other quantity for the ratio", async () => {
  const result = await nativeResult();
  const replayed = invokeMock.getMockImplementation()!;
  let release!: () => void;
  const gate = new Promise<void>((resolve) => { release = resolve; });
  invokeMock.mockImplementation((command: string, args?: unknown, options?: unknown) =>
    command === "lookup_preview_mechanics_job_rows" ? gate.then(() => replayed(command, args, options)) : replayed(command, args, options));
  const run = ruleCheckRunOn(full);
  const expected = buildStressRatioTable({ model: pair.model, result: full, standing, ruleCheckRun: run });
  const command = { current: null as ((value: ViewportViewCommand) => void) | null };
  render(
    <DisplayUnitsProvider>
      <StressRatioTablePanel model={pair.model} result={result} standing={standing} ruleCheckRun={run} />
      <KeyResultsPanel model={pair.model} result={result} standing={standing} ruleCheckRun={run} everySolve={false} {...keySpies()} />
      {viewport(run, result, command)}
    </DisplayUnitsProvider>
  );
  await waitFor(() => expect(screen.getByTestId("stress-ratio-run")).toHaveTextContent(STRESS_RATIO_RULE_ROWS_PENDING));
  expect(screen.getByTestId("stress-ratio-panel")).not.toHaveTextContent(STRESS_RATIO_NO_RESULT);
  const emitted = expected.groups.filter((item) => item.source === "emitted_stress").map((item) => item.id);
  const offered = [...(screen.getByTestId("stress-ratio-group") as HTMLSelectElement).options].map((option) => option.value);
  expect(emitted.length).toBeGreaterThan(0);
  expect(offered).toEqual(expect.arrayContaining(emitted));
  expect(screen.getByTestId("key-results-ratio-note")).toHaveAttribute("data-ratio-state", "rows_pending");
  const { group, caseKey } = ruleGroup(expected);
  act(() => command.current!({ type: "show-result-colour", quantityId: `rule-ratio:${group.id}`, caseKey }));
  await waitFor(() => expect(screen.getByTestId("viewport-result-colour-reason")).toHaveTextContent(STRESS_RATIO_RULE_ROWS_PENDING));
  expect(screen.queryByTestId("viewport-result-colour-min")).toBeNull();
  expectRatioUnavailable(`rule-ratio:${group.id}`);
  // The table's ratio mode says why there is no ratio: the run's own note.
  expect(screen.getByTestId("stress-ratio-mode-reason")).toHaveTextContent(STRESS_RATIO_RULE_ROWS_PENDING);

  release();
  await waitFor(() => expect(screen.getByTestId("stress-ratio-run")).toHaveTextContent(expected.run.note));
  expect(screen.getByTestId("key-results-ratio-note")).toHaveAttribute("data-ratio-state", "used");
  await waitFor(() => expect(screen.getByTestId("viewport-overlay-quantity")).toHaveValue(`rule-ratio:${group.id}`));
  expect(sent("read_preview_mechanics_job_rows_all")).toBe(0);
});

it("a run binding a row the held result does not hold is read as another result's run", async () => {
  const result = await nativeResult();
  const run = ruleCheckRunOn(full, "invented:not-a-row-of-this-result");
  expect(buildStressRatioTable({ model: pair.model, result: full, standing, ruleCheckRun: run }).run.state).toBe("not_this_result");
  render(<StressRatioTablePanel model={pair.model} result={result} standing={standing} ruleCheckRun={run} />);
  await waitFor(() => expect(screen.getByTestId("stress-ratio-run")).toHaveTextContent(STRESS_RATIO_RUN_NOT_THIS_RESULT));
  expect(sent("read_preview_mechanics_job_rows_all")).toBe(0);
});

it("a refused read of the run's rows says the rows could not be read, in the refusal's words, and keeps the result's own stresses", async () => {
  const result = await nativeResult();
  // The one mocked reply: the lookup refused as the native command words a failed read.
  const REFUSAL = "RESULT-DETAIL-READ-FAILED: results changed between passes";
  const replayed = invokeMock.getMockImplementation()!;
  invokeMock.mockImplementation((command: string, args?: unknown, options?: unknown) =>
    command === "lookup_preview_mechanics_job_rows" ? Promise.reject(REFUSAL) : replayed(command, args, options));
  const run = ruleCheckRunOn(full);
  const expected = buildStressRatioTable({ model: pair.model, result: full, standing, ruleCheckRun: run });
  const command = { current: null as ((value: ViewportViewCommand) => void) | null };
  render(
    <DisplayUnitsProvider>
      <StressRatioTablePanel model={pair.model} result={result} standing={standing} ruleCheckRun={run} />
      <KeyResultsPanel model={pair.model} result={result} standing={standing} ruleCheckRun={run} everySolve={false} {...keySpies()} />
      {viewport(run, result, command)}
    </DisplayUnitsProvider>
  );
  const unread = stressRatioRuleRowsUnreadText(REFUSAL);
  await waitFor(() => expect(screen.getByTestId("stress-ratio-run")).toHaveTextContent(unread));
  // Not the state shown while the rows are read: the result's own stress groups are listed.
  expect(screen.queryByTestId("stress-ratio-unavailable")).toBeNull();
  expect(screen.getByTestId("stress-ratio-panel")).not.toHaveTextContent(STRESS_RATIO_NO_RESULT);
  expect(screen.getByTestId("key-results-ratio-note")).toHaveAttribute("data-ratio-state", "rows_unread");
  expect(screen.getByTestId("key-results-ratio-note")).toHaveTextContent(unread);
  const offered = [...(screen.getByTestId("stress-ratio-group") as HTMLSelectElement).options].map((option) => option.value);
  expect(offered).toEqual(expect.arrayContaining(expected.groups.filter((item) => item.source === "emitted_stress").map((item) => item.id)));
  // Only the ratio is withheld: the viewport says why and draws no other quantity in its place.
  const { group, caseKey } = ruleGroup(expected);
  act(() => command.current!({ type: "show-result-colour", quantityId: `rule-ratio:${group.id}`, caseKey }));
  await waitFor(() => expect(screen.getByTestId("viewport-result-colour-reason")).toHaveTextContent(unread));
  expect(screen.queryByTestId("viewport-result-colour-min")).toBeNull();
  expectRatioUnavailable(`rule-ratio:${group.id}`);
  expect(screen.getByTestId("stress-ratio-mode-reason")).toHaveTextContent(unread);
  expect(sent("lookup_preview_mechanics_job_rows")).toBeGreaterThanOrEqual(1);
  expect(sent("read_preview_mechanics_job_rows_all")).toBe(0);
});

/** The lookup's answer for `request` over invented rows: each id's last row, with its ordinal. */
function lookupOver(rows: MechanicsResult["results"]) {
  return async (request: LookupRequest): Promise<Lookup> => ({
    rowCount: rows.length, ordinals: [], kinds: [],
    ids: (request.ids ?? []).map((id) => {
      const ordinal = rows.map((row) => row.id).lastIndexOf(id);
      return ordinal < 0 ? null : { ordinal, row: rows[ordinal] } as unknown as Lookup["ids"][number];
    })
  });
}
const inventedRow = (id: string, location = "x") => ({
  id, kind: "invented_kind", entity_ref: `pipe:${id}`, value: 1, unit: "MPa", basis_ref: { ref_type: "load_case", ref_id: "lc" },
  metadata: { location, component: "c", basis: "b", extra: "not kept" }
}) as unknown as MechanicsResult["results"][number];

it("keeps only a bound row's native key, entity and location fields, page by page", async () => {
  const rows = Array.from({ length: RESULT_DETAIL_MAX_KEYS + 3 }, (_, i) => inventedRow(`r${i}`));
  const requests: LookupRequest[] = [];
  const lookup = lookupOver(rows);
  const kept = await collectRuleCheckRows(rows.map((row) => row.id), (request) => { requests.push(request); return lookup(request); });
  expect(requests.map((request) => request.ids!.length)).toEqual([RESULT_DETAIL_MAX_KEYS, 3]);
  expect(kept.get("r2")).toEqual({ id: nativeRowKey(2), entity_ref: "pipe:r2", metadata: { location: "x", component: "c", basis: "b" } });
  expect(Object.keys(kept.get("r2")!)).toEqual(["id", "entity_ref", "metadata"]);
});

it("refuses bound rows whose kept fields pass their limit, and a read no longer wanted", async () => {
  const long = "l".repeat(RULE_CHECK_ROWS_KEPT_LIMIT_BYTES / 2);
  const rows = [inventedRow("a", long), inventedRow("b", long)];
  await expect(collectRuleCheckRows(["a", "b"], lookupOver(rows))).rejects.toThrow(`RESULT-DETAIL-RESPONSE-LIMIT: the rule-check rows the displays keep pass their ${RULE_CHECK_ROWS_KEPT_LIMIT_BYTES}-byte limit.`);
  await expect(collectRuleCheckRows(["a"], lookupOver([inventedRow("a")]), () => { throw new Error("NOT-WANTED"); })).rejects.toThrow("NOT-WANTED");
});

it("refuses the run's rows as disposed once custody ends, whether the read is kept or still answering", async () => {
  const result = await nativeResult();
  const run = ruleCheckRunOn(full);
  expect((await readRuleCheckRows(result, run)).size).toBeGreaterThan(0);
  await disposeDeliveredMechanicsResult(result);
  // A kept read is not returned for a result no longer held.
  await expect(readRuleCheckRows(result, run)).rejects.toThrow(NATIVE_RESULT_DISPOSED);

  const other = await nativeResult();
  const replayed = invokeMock.getMockImplementation()!;
  let release!: () => void;
  const gate = new Promise<void>((resolve) => { release = resolve; });
  // The replies are answered while the result is held and delivered only after its custody ends.
  invokeMock.mockImplementation((command: string, args?: unknown, options?: unknown) => {
    if (command !== "lookup_preview_mechanics_job_rows") return replayed(command, args, options);
    const answered = replayed(command, args, options) as Promise<unknown>;
    return gate.then(() => answered);
  });
  const reading = readRuleCheckRows(other, run);
  const looking = lookupNativeRows(other, { ids: [ruleCheckBoundIds(run)[0]], match: "last" });
  await waitFor(() => expect(sent("lookup_preview_mechanics_job_rows")).toBeGreaterThanOrEqual(2));
  await disposeDeliveredMechanicsResult(other);
  release();
  // A reply that arrives after custody ended is neither returned by the lookup nor kept by the read,
  // though the command answered it while the result was held.
  await expect(looking).rejects.toThrow(NATIVE_RESULT_DISPOSED);
  await expect(reading).rejects.toThrow(NATIVE_RESULT_DISPOSED);
});

it("is ready at once with no rows for a run binding none, and drops an earlier run's rows while another is read", async () => {
  const result = await nativeResult();
  const run = ruleCheckRunOn(full);
  const unbound = { ...run, result: { ...run.result, checks: run.result.checks.map((check) => ({ ...check, bound_inputs: check.bound_inputs.map((input) => ({ ...input, result_id: undefined })) })) } } as typeof run;
  const before = sent("lookup_preview_mechanics_job_rows");
  const { result: status, rerender } = renderHook(({ which }) => useRuleCheckRows(result, which), { initialProps: { which: unbound } });
  expect(status.current).toEqual({ state: "ready", rows: new Map() });
  expect(sent("lookup_preview_mechanics_job_rows")).toBe(before);

  rerender({ which: run });
  expect(status.current.state).toBe("loading");
  await waitFor(() => expect(status.current.state).toBe("ready"));
  const replayed = invokeMock.getMockImplementation()!;
  let release!: () => void;
  const gate = new Promise<void>((resolve) => { release = resolve; });
  invokeMock.mockImplementation((command: string, args?: unknown, options?: unknown) =>
    command === "lookup_preview_mechanics_job_rows" ? gate.then(() => replayed(command, args, options)) : replayed(command, args, options));
  const next = ruleCheckRunOn(full, "invented:not-a-row-of-this-result");
  rerender({ which: next });
  // The earlier run's rows are not shown for the next run while it is read.
  expect(status.current.state).toBe("loading");
  release();
  await waitFor(() => expect(status.current.state).toBe("ready"));
});

it("shows no rows read while the result was held once its custody has ended, though nothing else changes", async () => {
  const result = await nativeResult();
  const run = ruleCheckRunOn(full);
  const { result: status, rerender } = renderHook(({ which }) => useRuleCheckRows(result, which), { initialProps: { which: run } });
  await waitFor(() => expect(status.current.state).toBe("ready"));
  await disposeDeliveredMechanicsResult(result);
  rerender({ which: run });
  expect(status.current.state).toBe("error");
  expect((status.current as { message: string }).message).toContain(NATIVE_RESULT_DISPOSED);
});

it("bounds the kept rows by their UTF-8 bytes", async () => {
  expect(["", "a", "é", "€", "😀", "a\u{d800}b"].map(utf8Length)).toEqual([0, 1, 2, 3, 4, 5]);
  // Fewer UTF-16 code units than the limit, more bytes.
  const wide = "é".repeat(RULE_CHECK_ROWS_KEPT_LIMIT_BYTES / 2 + 1);
  expect(wide.length).toBeLessThan(RULE_CHECK_ROWS_KEPT_LIMIT_BYTES);
  await expect(collectRuleCheckRows(["a"], lookupOver([inventedRow("a", wide)]))).rejects.toThrow("RESULT-DETAIL-RESPONSE-LIMIT");
});

it("names a shown emitted stress by its published id in its value's title", async () => {
  // This result's emitted stresses are admitted by its standing, so their values are shown.
  const pressure = nativeMechanicsReplayPair("sparse_interactive", { profile: "pressure" });
  const result = await nativeResult("pressure");
  const ids = new Set(pressure.source.results.map((row) => row.id));
  render(<DisplayUnitsProvider><StressRatioTablePanel model={pressure.model} result={result} standing={standing} /></DisplayUnitsProvider>);
  await waitFor(() => {
    const titles = [...document.querySelectorAll("[data-testid^='stress-ratio-value-'] span[title]")].map((item) => item.getAttribute("title")!);
    const named = titles.map((title) => /\(([^()]*)\)$/.exec(title)?.[1]).filter((id): id is string => !!id);
    expect(named.length).toBeGreaterThan(0);
    expect(named.every((id) => ids.has(id))).toBe(true);
  });
  expect(sent("read_preview_mechanics_job_rows_all")).toBe(0);
});
