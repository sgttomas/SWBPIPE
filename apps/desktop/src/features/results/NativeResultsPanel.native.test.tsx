// C1 slice 4 (DEL-04-05 `c1-native-custody-plan.md`; DEL-07-05 `results-navigator.md`; DEL-08-04
// `csv-export.md`): the results panel for a result held natively, through the actual raw-body
// ingress and the actual Rust commands' recorded replies (`test/nativeReplayStore.ts`,
// `src-tauri/src/native_replay_tests.rs`):
// - the navigator's entity axis equals `navigatorEntities` and `firstRowOf` over the full rows at
//   each filter boundary, with probes;
// - stepping every axis (buttons and key chords) lands on the first matching published row on its
//   native page, across pages, and stops at both ends; stale and disposed replies are discarded;
// - a lone surrogate in the search is the typed refusal, on the rows and on the entity list;
// - every group's CSV, written through the native stream in pages, is byte for byte the CSV the
//   full rows give through the unchanged formatters (the browser panel's `saveCsvTable` text).
// Unit transport replay; NOT native UI qualification. The paged source is invented: the preview
// fixture's rows with its node rows copied under new ids and entities, so entities begin past page 1.
import { act, cleanup, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { useState } from "react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../../test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));

import { awaitPreviewMechanicsJob, disposeDeliveredMechanicsResult, readNativeFreshResult, resetSessionResultCustodyForTests, startPreviewMechanicsJob } from "../../services/previewService";
import { nativeCustody, queryNativeNavigator, resetNativeResultsForTests, type NativeResult } from "../../services/nativeResult";
import { panelDetailFilter } from "../../services/resultDetail";
import { createSolveJobTransport, nativeMechanicsReplayPair, REPLAY_COMMANDS, solveBytes } from "../../test/nativeMechanicsReplay";
import { rememberReceivedText } from "./resultReader";
import { semanticLabels } from "./resultSemantics";
import { firstRowOf, navigatorEntities, rowCaseKey } from "./resultsNavigator";
import { ResultsPanel, rowsUnderFilters, countResultFrames, standaloneNumericalHealth } from "./ResultsPanel";
import type { NumericalHealthView } from "./numericalHealth";
import type { MechanicsResult, PreviewModel } from "../../types";

type Row = MechanicsResult["results"][number];
const pair = nativeMechanicsReplayPair("sparse_interactive", { profile: "preview" });

/** The invented paged source: the preview fixture's rows, then its global (node) rows seven more
 * times, copy `c` under ids and entities suffixed `:c<c>`: 173 rows over 19 entities, whose
 * copies begin past the first page of 50. */
function pagedSource(): string {
  const source = JSON.parse(pair.bytes) as MechanicsResult;
  const rows: Row[] = [...source.results];
  const global = source.results.filter((row) => row.kind.startsWith("global"));
  for (let copy = 1; copy < 8; copy += 1) {
    for (const row of global) rows.push({ ...row, id: `${row.id}:c${copy}`, entity_ref: `${row.entity_ref}:c${copy}` });
  }
  return solveBytes({ ...source, results: rows });
}
const SOURCES = { fixture: pair.bytes, paged: pagedSource() } as const;
type SourceName = keyof typeof SOURCES;

type Captured = { begin: Record<string, unknown>[]; append: Record<string, unknown>[]; finish: Record<string, unknown>[]; abort: Record<string, unknown>[]; saved: string[] };
let captured: Captured;
let hold: ((command: string, args: Record<string, unknown>) => Promise<void> | null) | null = null;

function install(source: string) {
  const transport = createSolveJobTransport(() => source, { jobIdPrefix: "native-panel" });
  invokeMock.mockImplementation(async (command: string, args?: unknown, options?: unknown) => {
    const decoded = args as Record<string, unknown>;
    const waiting = hold?.(command, decoded);
    if (waiting) await waiting;
    // A lone surrogate in an argument: the native ingress refuses it typed, before parsing
    // (`ingress::prescan`; `the_navigator_refuses_by_name_and_prices_its_list` pins that refusal on
    // the actual command). A request JSON holding one cannot be kept in the replay's requests file
    // (serde refuses it there), so its refusal is answered here in the command's serialized shape.
    const plainArgs = !!args && typeof args === "object" && !ArrayBuffer.isView(args) && !(args instanceof ArrayBuffer);
    const lone = plainArgs && /(?:[\ud800-\udbff](?![\udc00-\udfff])|(?<![\ud800-\udbff])[\udc00-\udfff])/.exec(JSON.stringify(decoded ?? null).replace(/\\u(d[89a-f][0-9a-f]{2})/gi, (_, hex) => String.fromCharCode(parseInt(hex, 16))));
    if (lone) throw { code: "RESOURCE_UNAVAILABLE", refusal: { kind: "ingress_invalid", reason: "lone leading surrogate at byte 0" }, pool: null };
    // The native CSV stream, as `native_csv_stream.rs` answers a well-formed sequence (its own
    // refusals and file writes are the Rust tests'); the texts are kept to compare.
    if (command === "begin_local_csv_stream") {
      captured.begin.push(decoded);
      const text = (decoded.request as { text: string }).text;
      return { outcome: "streaming", stream_id: "0123456789abcdef0123456789abcdef", file_name: (decoded.request as { file_name: string }).file_name, sequence: 1, byte_count: new TextEncoder().encode(text).length };
    }
    if (command === "append_local_csv_stream") {
      captured.append.push(decoded);
      return { outcome: "streaming", sequence: (decoded.sequence as number) + 1, byte_count: 0 };
    }
    if (command === "finish_local_csv_stream") {
      captured.finish.push(decoded);
      const name = (captured.begin.at(-1)!.request as { file_name: string }).file_name;
      return { outcome: "saved", file_name: name, byte_count: decoded.byteCount, replaced_existing: false, durability: "not_guaranteed", path_containment: "best_effort_non_adversarial" };
    }
    if (command === "abort_local_csv_stream") {
      captured.abort.push(decoded);
      return { outcome: "aborted", partial_file_name: null, cleanup: "removed" };
    }
    if (command === "save_local_result_json") {
      // The browser panel's single save of the same table (the full rows' CSV).
      const href = (decoded.request as { href: string }).href;
      const text = decodeURIComponent(href.slice("data:text/csv;charset=utf-8,".length));
      captured.saved.push(text);
      return { outcome: "saved", file_name: (decoded.request as { file_name: string }).file_name, byte_count: new TextEncoder().encode(text).length, replaced_existing: false, durability: "not_guaranteed", path_containment: "best_effort_non_adversarial" };
    }
    if (REPLAY_COMMANDS.includes(command)) return transport.invoke(command, args, options);
    throw new Error(`UNEXPECTED_COMMAND: ${command}`);
  });
  (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
}

async function nativeResult(name: SourceName): Promise<{ result: NativeResult; full: MechanicsResult; text: string }> {
  const text = SOURCES[name];
  install(text);
  const started = await startPreviewMechanicsJob(pair.model, "sparse_interactive", { custody: "native" });
  if (started.mode !== "backend_job") throw new Error("not a backend job");
  const terminal = await awaitPreviewMechanicsJob(started.job_id);
  const reading = await readNativeFreshResult(started.job_id, terminal.result_bytes ?? 0, { standingModel: pair.model, summaryModel: pair.model });
  const full = JSON.parse(text) as MechanicsResult;
  rememberReceivedText(full, text);
  return { result: reading.result, full, text };
}

beforeEach(() => {
  resetSessionResultCustodyForTests();
  resetNativeResultsForTests();
  captured = { begin: [], append: [], finish: [], abort: [], saved: [] };
  hold = null;
});
afterEach(() => {
  cleanup();
  invokeMock.mockReset();
  delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
});

/** The panel with the session's selection held as the app holds it (a step selects through it). */
function Harness({ result, model, onSelected, numericalHealth = null }: { result: NativeResult | MechanicsResult; model: PreviewModel; onSelected?: (id: string) => void; numericalHealth?: NumericalHealthView | null }) {
  const [selected, setSelected] = useState<string | null>(null);
  return (
    <ResultsPanel
      result={result as never}
      knowledge={null}
      analysisRun={null}
      selectedResultId={selected}
      onSelectResult={(id) => { setSelected(id); onSelected?.(id); }}
      model={model}
      numericalHealth={numericalHealth}
    />
  );
}

const position = (axis: string) => screen.getByTestId(`results-navigator-${axis}-position`).textContent ?? "";

describe("the native navigator's entity axis", () => {
  for (const name of ["fixture", "paged"] as const) {
    it(`${name}: equals navigatorEntities and firstRowOf over the full rows at each filter boundary, with probes`, async () => {
      const { result, full } = await nativeResult(name);
      const labels = semanticLabels(full);
      const families = [...new Set(full.results.map((row) => labels.family(row)))];
      const cases = [...new Set(full.results.map(rowCaseKey))];
      const frames = [...countResultFrames(full.results).keys()];
      const states = [
        { filterText: "", familyFilter: "all", frame: "all", caseKey: "all" },
        ...families.map((familyFilter) => ({ filterText: "", familyFilter, frame: "all", caseKey: "all" })),
        ...cases.map((caseKey) => ({ filterText: "", familyFilter: "all", frame: "all", caseKey })),
        ...frames.map((frame) => ({ filterText: "", familyFilter: "all", frame, caseKey: "all" })),
        { filterText: " TIP ", familyFilter: "all", frame: "all", caseKey: "all" },
        { filterText: "no row says this", familyFilter: "all", frame: "all", caseKey: "all" },
        { filterText: "", familyFilter: families[0], frame: "all", caseKey: cases[0], kind: full.results[0].kind }
      ];
      let crossing = 0;
      const reads = await Promise.all(states.map((state) => queryNativeNavigator(result, panelDetailFilter(state))));
      for (const [at, state] of states.entries()) {
        const rows = rowsUnderFilters(full.results, labels, state as never).filter((row) => !("kind" in state) || row.kind === state.kind);
        const read = reads[at];
        const entities = navigatorEntities(rows);
        expect(read.entities, JSON.stringify(state)).toEqual(entities);
        expect(read.total).toBe(rows.length);
        entities.forEach((entity, i) => {
          const first = firstRowOf(rows, entity)!;
          expect(read.firstIndices[i]).toBe(rows.indexOf(first));
          expect(read.firstOrdinals[i]).toBe(full.results.indexOf(first));
          if (read.firstIndices[i] >= 50) crossing += 1;
        });
      }
      if (name === "paged") expect(crossing).toBeGreaterThan(5);
      // Probes: an admitted row at its position, a row the filter leaves out.
      const family = families.find((item) => item !== "other") ?? families[0];
      const state = { filterText: "", familyFilter: family, frame: "all", caseKey: "all" };
      const rows = rowsUnderFilters(full.results, labels, state as never);
      const inside = full.results.indexOf(rows.at(-1)!);
      const outside = full.results.findIndex((row) => !rows.includes(row));
      expect((await queryNativeNavigator(result, panelDetailFilter(state), inside)).probe).toEqual({ ordinal: inside, admitted: true, index: rows.length - 1 });
      expect((await queryNativeNavigator(result, panelDetailFilter(state), outside)).probe).toEqual({ ordinal: outside, admitted: false, index: null });
      await disposeDeliveredMechanicsResult(result);
    }, 60_000);
  }
});

describe("stepping the native results navigator", () => {
  it("steps every entity across native pages to its first published row, stops at both ends, and steps kind and case by chord", async () => {
    const { result, full } = await nativeResult("paged");
    const selections: string[] = [];
    render(<Harness result={result} model={pair.model} onSelected={(id) => selections.push(id)} />);
    const panel = screen.getByTestId("results-panel");
    const labels = semanticLabels(full);
    const rows = rowsUnderFilters(full.results, labels, { filterText: "", familyFilter: "all", frame: "all", caseKey: "all" });
    const entities = navigatorEntities(rows);
    await waitFor(() => expect(position("entity")).toBe(`None chosen; ${entities.length} in this table`));
    const pages = new Set<number>();
    for (let i = 0; i < entities.length; i += 1) {
      fireEvent.click(screen.getByTestId("results-navigator-entity-next"));
      const first = firstRowOf(rows, entities[i])!;
      expect(position("entity")).toBe(`${entities[i]} (${i + 1} of ${entities.length})`);
      await waitFor(() => expect(selections.at(-1)).toBe(first.id));
      // The row is shown, selected, on its own native page (50 rows a page: past page 1 it is
      // in the document only if that page was read).
      await waitFor(() => expect(screen.getByTestId(`result-row-${first.id}`)).toHaveAttribute("aria-selected", "true"));
      pages.add(Math.floor(rows.indexOf(first) / 50));
      await waitFor(() => expect(screen.getByTestId("results-navigator-status")).toHaveTextContent(`${entities[i]}, row ${first.id}.`));
    }
    expect(pages.size).toBeGreaterThan(3);
    // At the last entity Next stops (no wrap); the selection stays.
    const before = selections.length;
    expect(screen.getByTestId("results-navigator-entity-next")).toHaveAttribute("aria-disabled", "true");
    fireEvent.click(screen.getByTestId("results-navigator-entity-next"));
    expect(screen.getByTestId("results-navigator-status")).toHaveTextContent("Last support or element reached.");
    expect(selections.length).toBe(before);
    // The chord from a result row: the previous entity, focus following its row.
    const selectedRow = screen.getByTestId(`result-row-${selections.at(-1)}`);
    fireEvent.keyDown(selectedRow, { key: "ArrowLeft", altKey: true });
    const previous = firstRowOf(rows, entities.at(-2)!)!;
    await waitFor(() => expect(selections.at(-1)).toBe(previous.id));
    await waitFor(() => expect(document.activeElement?.getAttribute("data-testid")).toBe(`result-row-${previous.id}`));
    // Kind by chord: the next table kind with the entity kept, on its first row there (or none).
    const entity = entities.at(-2)!;
    fireEvent.keyDown(panel, { key: "ArrowRight", altKey: true, shiftKey: true });
    const family = position("kind");
    expect(family).not.toMatch(/^All kinds/);
    const familyId = [...new Set(full.results.map((row) => labels.family(row)))].find((id) => family.toLowerCase().startsWith(id)) ?? null;
    if (familyId) {
      const kindRows = rowsUnderFilters(full.results, labels, { filterText: "", familyFilter: familyId as never, frame: "all", caseKey: "all" });
      const expected = firstRowOf(kindRows, entity);
      if (expected) await waitFor(() => expect(selections.at(-1)).toBe(expected.id));
    }
    // Case by chord.
    fireEvent.keyDown(panel, { key: "ArrowDown", altKey: true });
    expect(position("case")).not.toMatch(/^All cases/);
    // No model edit, nothing but selection: the panel only called its selection callback.
    expect(invokeMock.mock.calls.map(([command]) => command).filter((command) => !REPLAY_COMMANDS.includes(command))).toEqual([]);
    await act(async () => { await disposeDeliveredMechanicsResult(result); });
  }, 120_000);

  it("discards a stale step's reply, and a step on a disposed result selects nothing", async () => {
    const { result, full } = await nativeResult("paged");
    const selections: string[] = [];
    render(<Harness result={result} model={pair.model} onSelected={(id) => selections.push(id)} />);
    const labels = semanticLabels(full);
    await waitFor(() => expect(position("entity")).toMatch(/^None chosen; \d+ in this table$/));
    // Hold the lookup the first entity step makes; a second step supersedes it.
    let release: () => void = () => undefined;
    hold = (command) => command === "lookup_preview_mechanics_job_rows" && selections.length === 0 ? new Promise<void>((resolve) => { release = () => { hold = null; resolve(); }; }) : null;
    fireEvent.click(screen.getByTestId("results-navigator-entity-next"));
    const rows = rowsUnderFilters(full.results, labels, { filterText: "", familyFilter: "all", frame: "all", caseKey: "all" });
    const entities = navigatorEntities(rows);
    await waitFor(() => expect(position("entity")).toBe(`${entities[0]} (1 of ${entities.length})`));
    // The second step reads its lookup only after the first is released.
    fireEvent.click(screen.getByTestId("results-navigator-entity-next"));
    expect(position("entity")).toBe(`${entities[1]} (2 of ${entities.length})`);
    release();
    await waitFor(() => expect(selections.at(-1)).toBe(firstRowOf(rows, entities[1])!.id));
    expect(selections).not.toContain(firstRowOf(rows, entities[0])!.id);
    // A step whose reply arrives after the result's disposal selects nothing and throws nothing.
    let releaseLate: () => void = () => undefined;
    hold = (command) => command === "lookup_preview_mechanics_job_rows" ? new Promise<void>((resolve) => { releaseLate = resolve; }) : null;
    const count = selections.length;
    fireEvent.click(screen.getByTestId("results-navigator-entity-next"));
    await act(async () => { await disposeDeliveredMechanicsResult(result); });
    hold = null;
    releaseLate();
    await new Promise((resolve) => setTimeout(resolve, 20));
    expect(selections.length).toBe(count);
    expect(nativeCustody(result).jobId).toBeTruthy();
  }, 60_000);

  it("shows a lone surrogate in the search as the typed refusal, on the rows and on the entity list", async () => {
    const { result } = await nativeResult("fixture");
    render(<Harness result={result} model={pair.model} />);
    fireEvent.change(screen.getByTestId("result-filter-input"), { target: { value: "\ud800" } });
    expect(await screen.findByTestId("result-query-refused")).toHaveTextContent(/could not be read: Refused by the native memory pool: .*lone leading surrogate/);
    await waitFor(() => expect(position("entity")).toMatch(/could not be read: .*(surrogate|refused)/i));
    expect(screen.queryByTestId("result-filter-empty")).toBeNull();
    await act(async () => { await disposeDeliveredMechanicsResult(result); });
  }, 60_000);
});

describe("a native group's CSV through the native stream", () => {
  // The fixture's groups; crossing native pages is the generator's own test (page sizes 1 to 1000)
  // and the stepping test's (the paged source), so the paged source's CSV adds no case here.
  for (const name of ["fixture"] as const) {
    it(`${name}: every group's streamed CSV is byte for byte the full rows' CSV`, async () => {
      const { result, full } = await nativeResult(name);
      // The full rows' CSV: the browser panel's own export of each group, through the single save.
      // The same health on both sides (the session gives both panels its own).
      const health = standaloneNumericalHealth(full);
      const fullView = render(<Harness result={full} model={pair.model} numericalHealth={health} />);
      const groups = [...fullView.container.querySelectorAll<HTMLElement>("[data-testid^='result-group-'][data-testid$='-export-csv']")].map((button) => button.dataset.testid!);
      expect(groups.length).toBeGreaterThan(1);
      const expected = new Map<string, string>();
      for (const testId of groups) {
        fireEvent.click(within(fullView.container).getByTestId(testId));
        await waitFor(() => expect(within(fullView.container).getByTestId(`${testId}-status`)).toHaveTextContent(/^Saved /));
        expected.set(testId, captured.saved.at(-1)!);
      }
      fullView.unmount();
      const nativeView = render(<Harness result={result} model={pair.model} numericalHealth={health} />);
      for (const testId of groups) {
        const button = await within(nativeView.container).findByTestId(testId);
        const begun = captured.begin.length;
        const appended = captured.append.length;
        fireEvent.click(button);
        await waitFor(() => expect(within(nativeView.container).getByTestId(`${testId}-status`)).toHaveTextContent(/^Saved /));
        const begin = captured.begin[begun];
        const appends = captured.append.slice(appended);
        expect(appends.map((item) => item.sequence)).toEqual(appends.map((_, i) => i + 1));
        const streamed = [(begin.request as { text: string }).text, ...appends.map((item) => (item.chunk as { text: string }).text)].join("");
        expect(streamed, testId).toBe(expected.get(testId));
        expect(begin.jobId).toBe(nativeCustody(result).jobId);
        expect(captured.finish.at(-1)).toMatchObject({ sequence: appends.length + 1, byteCount: new TextEncoder().encode(streamed).length });
        // Every chunk carries its own screening evidence on the CSV route with explicit intent.
        for (const chunk of [begin.request, ...appends.map((item) => item.chunk)] as { screening: Record<string, unknown>; local_first: Record<string, unknown> }[]) {
          expect(chunk.screening).toMatchObject({ route_id: "DOTH-CSV-011", export_context: "local_private", explicit_local_private_intent: true, blocked: false, exact_payload_match: true });
          expect(chunk.local_first).toMatchObject({ route_id: "DOTH-CSV-011", reason_code: "PRIVATE_LOCAL_METADATA_ALLOWED", explicit_local_private_intent: true });
        }
      }
      expect(captured.abort).toEqual([]);
      nativeView.unmount();
      await disposeDeliveredMechanicsResult(result);
    }, 120_000);
  }
});
