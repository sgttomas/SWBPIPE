/** C1 native custody, slice 4 (DEL-04-05 `c1-native-custody-plan.md`): the results panel for a
 * result held natively. Its counts, frames and cases are S's (`results_panel`, the ports of
 * `countResultFamilies`, `countResultFrames` and `navigatorCases`); its rows are one bounded native
 * page under the panel's filters (`panelDetailFilter`), clamped to the last page from the query's
 * total as `paginateResults` clamps; each page's labels come from one batched native row-semantics
 * call; the selected row, its endpoint pair and the intensified-row notice come by bounded native
 * lookup or query. A query the native host refuses for its arguments (a lone surrogate in a search,
 * `INGRESS-ARGS-INVALID`) is shown as that typed refusal, never as no rows. The webview holds one
 * page of rows, never the envelope.
 *
 * The results navigator (DEL-07-05 `results-navigator.md`) steps all three axes as `ResultsPanel`
 * does: kinds and cases from S; supports and elements from the native navigator query
 * (`query_preview_mechanics_job_navigator`: the distinct entities of the rows the filters admit,
 * other than "Selected entity only", in first-publication order, with each one's first row), so a
 * step lands on the first matching published row, on its native page, without reading every row
 * here. A step whose list is still being read waits for it; a reply for a step, filter or result
 * that is no longer current is discarded. A group's CSV is written through the native stream in
 * pages (`resultGroupCsvStream`), byte for byte the CSV the full rows would give. */
import { useEffect, useMemo, useRef, useState, type KeyboardEvent } from "react";
import { QuantityReadout } from "../display-units";
import { CsvExportControl } from "../result-export/CsvExportControl";
import { CSV_SOFTWARE_VERSION, csvAnalysisRoute, csvModelHash, csvResultHash } from "../result-export/csvProvenance";
import { CSV_NOT_RECORDED } from "../result-export/csvTable";
import type { CsvExport, CsvStreamExport } from "../result-export/CsvExportControl";
import { displayConversionBasis, ENDPOINT_PAIR_CSV_KIND, endpointPairCsv, resultGroupCsvKind, type ResultRowsCsvRow } from "./resultRowsCsv";
import { resultGroupCsvStream } from "./resultGroupCsvStream";
import { canSurfaceRevealNow } from "../workspace/surfaceReveal";
import { useDisplayQuantityResolver, type DisplaySourceQuantity } from "../display-units";
import { hangerPreloadStanding } from "./hangerPreload";
import { knownSemanticNotices, resultRowLabel } from "./knownSemanticLimitations";
import { NUMERICAL_HEALTH_STATE_TEXT } from "./numericalHealthText";
import { rowHealthState, type NumericalHealthLocation, type NumericalHealthView } from "./numericalHealth";
import { ALL_CASES, isNavigatorExcludedTarget, navigatorKeyStep, stepIndex, type NavigatorAxis, type NavigatorCase } from "./resultsNavigator";
import { mechanicsGaps } from "./resultInterpretation";
import type { ResultsViewRequest } from "./ResultsPanel";
import {
  ALL_FRAMES,
  DEFAULT_RESULT_PAGE_SIZE,
  frameLabel,
  GoverningRatioState,
  groupResults,
  MechanicsGapLedger,
  NumericalHealthBadgeText,
  NAVIGATOR_AXIS_NOUN,
  NumericalHealthDetail,
  RESULT_FAMILY_OPTIONS,
  ResultControls,
  ResultsNavigator,
  resultFrame,
  RowHealthBadge,
  routeHealth,
  SectionBasisHeadlines,
  type NavigatorStep,
  type ResultFamilyFilter
} from "./ResultsPanel";
import {
  isNativeAnalysisRun,
  lookupNativeRows,
  nativeCustody,
  NativeQueryRefused,
  nativeQueryInvoke,
  queryNativeNavigator,
  queryNativeRows,
  readNativeRowSemantics,
  type NativeResult,
  type SessionAnalysisRun
} from "../../services/nativeResult";
import { panelDetailFilter, resultRowPages, type DetailFilter, type DetailRow, type NavigatorAxisRead, type RowSemanticsOutcome } from "../../services/resultDetail";
import { nativeFailureText } from "../../services/nativeIngress";
import type { DesignKnowledge, EntityRef, MechanicsResult, PreviewModel } from "../../types";
import type { RowLabels } from "./resultSemantics";
import type { RowSemantics } from "./resultReader";

type Row = MechanicsResult["results"][number];
type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
const PREVIEW_INTENSIFIED_KIND = "preview_intensified_bending_stress";
/** The navigator lists a panel keeps (one per filter it stepped through recently). */
const NAVIGATOR_LISTS_KEPT = 4;
const FAMILY_BY_TITLE: Readonly<Record<string, string>> = { Displacement: "displacement", Rotation: "rotation", Reaction: "reaction", Force: "force", Moment: "moment", Stress: "stress", Ratio: "ratio", Other: "other" };

/** Labels over the rows of a page from their batched native semantics, as `semanticLabels` reads
 * them (a refused row's semantics throws the reader's refusal). */
function labelsOf(rows: readonly DetailRow[], outcomes: readonly RowSemanticsOutcome[]): RowLabels {
  const byRow = new Map<Row, RowSemanticsOutcome>();
  rows.forEach((entry, i) => byRow.set(entry.row, outcomes[i]));
  const semantics = (row: Row): RowSemantics => {
    const outcome = byRow.get(row);
    if (!outcome || !outcome.ok) throw new Error(outcome && !outcome.ok ? outcome.error.code : "ROW_SEMANTICS_UNREAD");
    return { signature: outcome.signature, complete_metadata: outcome.complete_metadata, canonical_metadata: outcome.canonical_metadata };
  };
  const signature = (row: Row) => semantics(row).signature;
  return {
    semantics,
    family: (row) => { try { const s = signature(row); return s?.category === "physical_quantity" ? s.family ?? "other" : "other"; } catch { return "other"; } },
    dimension: (row) => { try { return signature(row)?.derivative_target_dimension ?? null; } catch { return null; } },
    category: (row) => { try { return signature(row)?.category ?? "unknown"; } catch { return "integrity_failure"; } }
  };
}

type PageState =
  | { status: "reading" }
  | { status: "ready"; key: string; total: number; offset: number; rows: readonly DetailRow[]; labels: RowLabels }
  | { status: "refused"; key: string; text: string };

export function NativeResultsPanel({
  result,
  analysisRun,
  selectedResultId,
  onSelectResult,
  numericalHealth,
  resolveCauseLocation,
  onSelectCauseLocation,
  onShowDiagnostic,
  selectedEntity = null,
  viewRequest = null,
  model = null,
  exportAnalysisRun = null
}: {
  result: NativeResult;
  knowledge: DesignKnowledge | null;
  analysisRun: SessionAnalysisRun | null;
  selectedResultId: string | null;
  onSelectResult: (resultId: string) => void;
  numericalHealth?: NumericalHealthView | null;
  resolveCauseLocation?: (location: NumericalHealthLocation) => EntityRef | null;
  onSelectCauseLocation?: (entity: EntityRef) => void;
  onShowDiagnostic?: (diagnosticId: string) => void;
  selectedEntity?: EntityRef | null;
  viewRequest?: ResultsViewRequest | null;
  model?: PreviewModel | null;
  exportAnalysisRun?: SessionAnalysisRun | null;
}) {
  const displayResolver = useDisplayQuantityResolver();
  const custody = nativeCustody(result);
  const tables = custody.summary.header.tables.results_panel as Json;
  const runId = result.run_id;
  const [filterText, setFilterText] = useState("");
  const [selectionOnly, setSelectionOnly] = useState(false);
  const [frameFilter, setFrameFilter] = useState<string>(ALL_FRAMES);
  const [familyFilter, setFamilyFilter] = useState<string>("all");
  const [caseFilter, setCaseFilter] = useState<string>(ALL_CASES);
  const [pageSize, setPageSize] = useState<number>(DEFAULT_RESULT_PAGE_SIZE);
  const [pagePosition, setPagePosition] = useState<{ runId: string; index: number }>({ runId, index: 0 });
  const pageIndex = pagePosition.runId === runId ? pagePosition.index : 0;
  const setPageIndex = (next: number | ((current: number) => number)) => setPagePosition((current) => {
    const index = current.runId === runId ? current.index : 0;
    return { runId, index: typeof next === "function" ? next(index) : next };
  });
  const [appliedRequest, setAppliedRequest] = useState<number | null>(null);
  const [requestedKind, setKindFilter] = useState<{ runId: string; kind: string; withheld: Readonly<Record<string, string>> } | null>(null);
  const kindFilter = requestedKind && requestedKind.runId === runId ? requestedKind : null;
  if (viewRequest && viewRequest.serial !== appliedRequest && viewRequest.runId === runId) {
    setAppliedRequest(viewRequest.serial);
    setKindFilter({ runId: viewRequest.runId, kind: viewRequest.kind, withheld: viewRequest.withheld });
    setFilterText("");
    setFamilyFilter("all");
    setCaseFilter(ALL_CASES);
    setFrameFilter(ALL_FRAMES);
    setSelectionOnly(false);
    setPagePosition({ runId, index: 0 });
  }
  const selectionEntityId = selectedEntity && selectedEntity.type !== "project" ? selectedEntity.id : null;
  const selectionOnlyWithoutEntity = selectionOnly && selectionEntityId === null;
  const familyCounts = tables.family_counts;
  const frameCounts = useMemo(() => new Map<string, number>((tables.frame_counts.counts as [string, number][]).map(([frame, count]) => [frame, count])), [tables]);
  const cases = tables.navigator_cases as NavigatorCase[];
  const activeFrame = frameFilter !== ALL_FRAMES && frameCounts.has(frameFilter) ? frameFilter : ALL_FRAMES;
  const activeCase = caseFilter !== ALL_CASES && cases.some((item) => item.key === caseFilter) ? caseFilter : ALL_CASES;
  const filter = useMemo(() => panelDetailFilter({
    filterText, familyFilter, frame: activeFrame, caseKey: activeCase, kind: kindFilter?.kind ?? null,
    selectionEntity: selectionOnly ? selectionEntityId : null
  }), [filterText, familyFilter, activeFrame, activeCase, kindFilter, selectionOnly, selectionEntityId]);
  const filterKey = JSON.stringify(filter);
  const requestKey = `${custody.jobId}\u0000${filterKey}\u0000${pageIndex}\u0000${pageSize}`;
  const [pageState, setPageState] = useState<PageState>({ status: "reading" });
  useEffect(() => {
    if (selectionOnlyWithoutEntity) return;
    let live = true;
    (async () => {
      let read = await queryNativeRows(result, filter, pageIndex * pageSize, pageSize);
      // The native query returns an empty page past the end; the panel shows the last page, as
      // `paginateResults` clamps a page index from the row count.
      if (read.rows.length === 0 && read.total > 0) {
        const last = Math.max(0, Math.ceil(read.total / pageSize) - 1);
        read = await queryNativeRows(result, filter, last * pageSize, pageSize);
        if (live) setPageIndex(last);
      }
      const outcomes = read.rows.length ? await readNativeRowSemantics(result, read.rows.map((entry) => entry.ordinal)) : [];
      if (live) setPageState({ status: "ready", key: requestKey, total: read.total, offset: read.offset, rows: read.rows, labels: labelsOf(read.rows, outcomes) });
    })().catch((error) => {
      if (!live) return;
      setPageState({ status: "refused", key: requestKey, text: error instanceof NativeQueryRefused ? `The native result query was refused: ${error.message}` : `The result rows could not be read: ${nativeFailureText(error)}` });
    });
    return () => { live = false; };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [requestKey, selectionOnlyWithoutEntity]);
  const current = pageState.status !== "reading" && pageState.key === requestKey ? pageState : null;
  const ready = current?.status === "ready" ? current : null;
  const filteredCount = selectionOnlyWithoutEntity ? 0 : ready?.total ?? 0;
  const pageCount = Math.max(1, Math.ceil(filteredCount / pageSize));
  const safeIndex = Math.min(pageIndex, pageCount - 1);
  const items = ready && !selectionOnlyWithoutEntity ? ready.rows.map((entry) => entry.row) : [];
  const page = { items, pageIndex: safeIndex, pageCount, start: filteredCount === 0 ? 0 : safeIndex * pageSize + 1, end: Math.min(safeIndex * pageSize + items.length, filteredCount) };
  const labels = ready?.labels ?? labelsOf([], []);
  const groups = groupResults(items, labels);
  const route = custody.verdict.route;
  const health = routeHealth(route, numericalHealth ?? null);
  const labelSource = route === "preview_physics" ? result : null;
  const intensified = useNativeKindPresence(result, PREVIEW_INTENSIFIED_KIND);
  const notices = useMemo(() => knownSemanticNotices(result as never, { hasIntensified: intensified }), [result, intensified]);
  const recovery = custody.recovery;
  const standingLine = route === "unsupported"
    ? "Unsupported numerical contract; historical values only. Needs recompute."
    : route === "pressure_source"
      ? `Ordinary solve: ${result.numerical_quality!.status}. Producer recovery receipt: ${String(recovery?.status ?? "unavailable")}. ${result.formulation_basis!.limitations.join(" ")} Current use is checked separately against the actual invocation and complete result evidence. Numerical checks do not establish engineering correctness.`
      : `Numerical integrity: ${result.numerical_quality!.status}. ${result.formulation_basis!.limitations.join(" ")} Numerical checks do not establish engineering correctness.`;
  const csvRecord = isNativeAnalysisRun(exportAnalysisRun) || exportAnalysisRun === null ? exportAnalysisRun : null;
  const exportGroup = async (title: string): Promise<CsvStreamExport> => {
    // Every row the filters admit in this group, across all pages, by bounded native pages, written
    // through the native stream: one page of rows (and its text) held here at a time.
    const groupFilter = { ...filter, family: FAMILY_BY_TITLE[title] ?? "other" };
    const pages = async function* (): AsyncGenerator<readonly ResultRowsCsvRow[]> {
      for await (const chunk of resultRowPages(nativeQueryInvoke(result), custody.jobId, groupFilter)) {
        const outcomes = await readNativeRowSemantics(result, chunk.rows.map((entry) => entry.ordinal));
        const chunkLabels = labelsOf(chunk.rows, outcomes);
        const rows: ResultRowsCsvRow[] = [];
        for (const { row: item } of chunk.rows) {
          const quantity: DisplaySourceQuantity = { value: item.value, unit: item.unit, dimension_id: chunkLabels.dimension(item) ?? "unknown" };
          const state = rowHealthState(health, item);
          rows.push({
            id: item.id, category: chunkLabels.category(item), label: (labelSource && resultRowLabel(item, labelSource as never)) || "",
            health: state ? NUMERICAL_HEALTH_STATE_TEXT[state] : "", withheld: kindFilter?.withheld[item.id] ?? "",
            componentBody: item.component_body ? `reference section of component body ${item.component_body.component_ref}` : "",
            entity: item.entity_ref, location: item.metadata?.location ?? "summary", frame: frameLabel(resultFrame(item)), quantity,
            display: await displayResolver.resolve(quantity)
          });
        }
        yield rows;
      }
    };
    const filters = [`case: ${activeCase}`, `family: ${familyFilter}`, `frame: ${activeFrame}`, `text: ${filterText.trim() || "none"}`, `kind: ${kindFilter?.kind ?? "any"}`, `selected entity only: ${selectionOnly ? selectionEntityId ?? "none selected" : "off"}`].join("; ");
    const streamed = resultGroupCsvStream({
      title, pages,
      provenance: { modelHash: model ? await csvModelHash(model) : CSV_NOT_RECORDED, resultHash: csvResultHash(result, csvRecord), analysisRoutes: [csvAnalysisRoute(result)] },
      softwareVersion: CSV_SOFTWARE_VERSION,
      caseOrFilter: `group: ${title}; ${filters}`,
      caseQualifier: activeCase !== ALL_CASES ? activeCase : undefined,
      preference: displayResolver.preference,
      convertRaw: displayResolver.convertRaw
    });
    return { fileName: streamed.fileName, jobId: custody.jobId, chunks: streamed.chunks };
  };

  /* ---- A1: the results navigator (DEL-07-05 `results-navigator.md`), on native lists ---- */
  const panelRef = useRef<HTMLElement>(null);
  const kindIds = RESULT_FAMILY_OPTIONS
    .filter((option) => option.id === "all" || (familyCounts as Record<string, number>)[option.id] > 0)
    .map((option) => option.id);
  const caseKeys = [ALL_CASES, ...cases.map((item) => item.key)];
  // The rows every filter but "Selected entity only" admits: the navigator's supports and elements.
  const navigatorFilterFor = (family: string, caseKey: string): DetailFilter => panelDetailFilter({ filterText, familyFilter: family, frame: activeFrame, caseKey, kind: kindFilter?.kind ?? null });
  const navigatorFilter = navigatorFilterFor(familyFilter, activeCase);
  const navigatorKey = `${custody.jobId}\u0000${JSON.stringify(navigatorFilter)}`;
  const lists = useRef<{ jobId: string; byKey: Map<string, Promise<NavigatorAxisRead>> }>({ jobId: custody.jobId, byKey: new Map() });
  /** The entity list under `filter`, one native query per filter; the panel keeps the last few. */
  const readNavigator = (filterAt: DetailFilter): Promise<NavigatorAxisRead> => {
    if (lists.current.jobId !== custody.jobId) lists.current = { jobId: custody.jobId, byKey: new Map() };
    const byKey = lists.current.byKey;
    const key = `${custody.jobId}\u0000${JSON.stringify(filterAt)}`;
    let pending = byKey.get(key);
    if (pending) byKey.delete(key);
    else {
      pending = queryNativeNavigator(result, filterAt);
      const kept = pending;
      // A refusal is not kept: a later step reads afresh.
      kept.catch(() => { if (byKey.get(key) === kept) byKey.delete(key); });
    }
    byKey.set(key, pending);
    while (byKey.size > NAVIGATOR_LISTS_KEPT) byKey.delete(byKey.keys().next().value!);
    return pending;
  };
  const [navigatorState, setNavigatorState] = useState<{ key: string; read: NavigatorAxisRead } | { key: string; refused: string } | null>(null);
  useEffect(() => {
    let live = true;
    readNavigator(navigatorFilter).then(
      (read) => { if (live) setNavigatorState({ key: navigatorKey, read }); },
      (error) => { if (live) setNavigatorState({ key: navigatorKey, refused: refusalText(error) }); }
    );
    return () => { live = false; };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [navigatorKey]);
  const navigatorCurrent = navigatorState?.key === navigatorKey ? navigatorState : null;
  const navigatorRead = navigatorCurrent && "read" in navigatorCurrent ? navigatorCurrent.read : null;
  const entities = navigatorRead?.entities ?? [];
  // The selected row, by bounded lookup (`results.find` by id), shared with the detail below.
  const selected = useNativeSelectedRow(result, selectedResultId);
  const selectedPending = selectedResultId !== null && selected === null;
  const selectedRow = selected?.row ?? null;
  // The navigator's support or element: the one a step chose, else the session's. A newly selected
  // result row, or a new selection, made anywhere moves it (once its row is read); a new run clears it.
  const [tracked, setTracked] = useState(() => ({ entity: selectionEntityId, selection: selectionEntityId, resultId: null as string | null, runId }));
  let navEntity = tracked.entity;
  if ((tracked.selection !== selectionEntityId || tracked.resultId !== selectedResultId || tracked.runId !== runId) && !(tracked.resultId !== selectedResultId && selectedPending)) {
    let entity = tracked.runId !== runId ? null : tracked.entity;
    if (tracked.resultId !== selectedResultId && selectedRow) entity = selectedRow.entity_ref;
    else if (tracked.selection !== selectionEntityId) entity = selectionEntityId;
    navEntity = entity;
    setTracked({ entity, selection: selectionEntityId, resultId: selectedResultId, runId });
  }
  const entityAt = navEntity === null || !navigatorRead ? -1 : navigatorRead.entities.indexOf(navEntity);
  // The current row: the selected row when it is the entity's and the filters admit it, else the
  // entity's first row (`firstRowOf`). Admission is known without a query when the selected row is
  // that first row or on the page shown (whose filter adds only the selected entity); otherwise
  // one probe asks.
  const selectedOrdinal = selected?.ordinal ?? null;
  const probeNeeded = selectedRow !== null && selectedOrdinal !== null && entityAt >= 0 && selectedRow.entity_ref === navEntity
    && navigatorRead!.firstOrdinals[entityAt] !== selectedOrdinal && !(ready?.rows.some((entry) => entry.ordinal === selectedOrdinal) ?? false);
  const probe = useNavigatorProbe(result, navigatorFilter, navigatorKey, probeNeeded ? selectedOrdinal : null);
  const selectedIsCurrent = selectedRow !== null && entityAt >= 0 && selectedRow.entity_ref === navEntity
    && (navigatorRead!.firstOrdinals[entityAt] === selectedOrdinal || (ready?.rows.some((entry) => entry.ordinal === selectedOrdinal) ?? false) || probe === true);
  const firstRowId = useNativeRowId(result, !selectedIsCurrent && entityAt >= 0 ? navigatorRead!.firstOrdinals[entityAt] : null);
  const currentRowId = selectedIsCurrent ? selectedRow!.id : firstRowId;
  const [endNotice, setEndNotice] = useState<string | null>(null);
  const [reveal, setReveal] = useState<{ rowId: string; focus: boolean } | null>(null);
  // A step's reply applies only while it is the latest step, on the same result and filters.
  const latest = useRef({ step: 0, navigatorKey, result, mounted: true });
  latest.current.navigatorKey = navigatorKey;
  latest.current.result = result;
  useEffect(() => {
    latest.current.mounted = true;
    return () => { latest.current.mounted = false; };
  }, []);

  /** The step's target position on each axis; null where the axis cannot move; "reading" while the
   * entity list is read (the step then waits for it). */
  function stepTarget(axis: NavigatorAxis, delta: 1 | -1): number | null | "reading" {
    if (axis === "kind") return stepIndex(kindIds.length, kindIds.indexOf(familyFilter as ResultFamilyFilter), delta);
    if (axis === "case") return stepIndex(caseKeys.length, caseKeys.indexOf(activeCase), delta);
    if (!navigatorRead) return "reading";
    return stepIndex(navigatorRead.entities.length, navEntity === null ? -1 : navigatorRead.entities.indexOf(navEntity), delta);
  }

  const focusStepButton = (focus: NavigatorStep | null) => {
    if (focus) panelRef.current?.querySelector<HTMLElement>(`[data-testid="results-navigator-${focus.axis}-${focus.delta > 0 ? "next" : "previous"}"]`)?.focus();
  };

  /** Shows `entity`'s first row under `filterAt`: its native page, its row in view, and the session's
   * selection of it (through `onSelectResult`, as a click on the row). Nothing else changes. `focus`
   * is the key step that came from a result row: focus moves to the new current row, or, when the
   * destination has none, to that step's own button. */
  async function follow(entity: string | null, filterAt: DetailFilter, focus: NavigatorStep | null, step: number) {
    const expected = `${custody.jobId}\u0000${JSON.stringify(filterAt)}`;
    const current = () => latest.current.mounted && latest.current.step === step && latest.current.result === result && latest.current.navigatorKey === expected;
    if (entity === null) {
      setPageIndex(0);
      focusStepButton(focus);
      return;
    }
    try {
      const read = await readNavigator(filterAt);
      if (!current()) return;
      const at = read.entities.indexOf(entity);
      if (at < 0) {
        setPageIndex(0);
        focusStepButton(focus);
        return;
      }
      // The exact published row (its own id, by ordinal).
      const found = await lookupNativeRows(result, { ordinals: [read.firstOrdinals[at]], match: "first" });
      const row = found.ordinals[0]?.row;
      if (!current() || !row) return;
      setTracked((now) => ({ ...now, entity }));
      setPageIndex(selectionOnly ? 0 : Math.floor(read.firstIndices[at] / pageSize));
      setReveal({ rowId: row.id, focus: focus !== null });
      onSelectResult(row.id);
    } catch (error) {
      if (current()) setEndNotice(`The step could not be completed: ${refusalText(error)}`);
    }
  }

  function chooseCase(caseKey: string, focus: NavigatorStep | null = null) {
    const step = ++latest.current.step;
    setEndNotice(null);
    setCaseFilter(caseKey);
    setPageIndex(0);
    void follow(navEntity, navigatorFilterFor(familyFilter, caseKey), focus, step);
  }

  function navigate(axis: NavigatorAxis, delta: 1 | -1, fromRow = false) {
    const focus = fromRow ? { axis, delta } : null;
    const target = stepTarget(axis, delta);
    if (target === "reading") {
      // Wait for this filter's list, then step from the entity current at that moment.
      const step = ++latest.current.step;
      const filterAt = navigatorFilter;
      void readNavigator(filterAt).then((read) => {
        if (!latest.current.mounted || latest.current.step !== step || latest.current.navigatorKey !== navigatorKey) return;
        const index = stepIndex(read.entities.length, navEntity === null ? -1 : read.entities.indexOf(navEntity), delta);
        if (index === null) setEndNotice(`${delta > 0 ? "Last" : "First"} ${NAVIGATOR_AXIS_NOUN[axis]} reached.`);
        else {
          setEndNotice(null);
          setTracked((now) => ({ ...now, entity: read.entities[index] }));
          void follow(read.entities[index], filterAt, focus, step);
        }
      }, (error) => { if (latest.current.step === step && latest.current.mounted) setEndNotice(`The step could not be completed: ${refusalText(error)}`); });
      return;
    }
    if (target === null) {
      setEndNotice(`${delta > 0 ? "Last" : "First"} ${NAVIGATOR_AXIS_NOUN[axis]} reached.`);
      return;
    }
    setEndNotice(null);
    if (axis === "kind") {
      const step = ++latest.current.step;
      const family = kindIds[target];
      setFamilyFilter(family);
      setPageIndex(0);
      void follow(navEntity, navigatorFilterFor(family, activeCase), focus, step);
    } else if (axis === "case") {
      chooseCase(caseKeys[target], focus);
    } else {
      const step = ++latest.current.step;
      const entity = navigatorRead!.entities[target];
      setTracked((now) => ({ ...now, entity }));
      void follow(entity, navigatorFilter, focus, step);
    }
  }

  function handleNavigatorKeyDown(event: KeyboardEvent<HTMLElement>) {
    if (event.defaultPrevented || isNavigatorExcludedTarget(event.target)) return;
    const step = navigatorKeyStep(event);
    if (!step) return;
    event.preventDefault();
    // From a result row, focus moves with the current row so the row being left can unmount.
    const fromRow = event.target instanceof Element && event.target.closest("tr[data-testid^='result-row-']") !== null;
    navigate(step.axis, step.delta, fromRow);
  }

  useEffect(() => {
    if (!reveal) return;
    const row = [...(panelRef.current?.querySelectorAll<HTMLElement>("tr[data-testid^='result-row-']") ?? [])]
      .find((element) => element.dataset.testid === `result-row-${reveal.rowId}`);
    if (!row) return;
    if (canSurfaceRevealNow(row)) row.scrollIntoView?.({ block: "nearest" });
    if (reveal.focus) row.focus();
    setReveal(null);
  }, [reveal, pageState]);

  return (
    <section className="panel results-panel" aria-label="Results" data-testid="results-panel" data-result-custody="native" ref={panelRef} onKeyDown={handleNavigatorKeyDown}>
      <div className="panel-title">Results</div>
      {health ? (
        <p className="numerical-health-header-line" data-testid="numerical-health-header-line"><NumericalHealthBadgeText badge={health.badge} text={health.headline} testId="numerical-health-header" />{health.outputSentence ? <>. <span data-testid="numerical-health-output-sentence">{health.outputSentence}</span></> : null}</p>
      ) : null}
      {health ? <NumericalHealthDetail health={health} resolveCauseLocation={resolveCauseLocation} onSelectCauseLocation={onSelectCauseLocation} onShowDiagnostic={onShowDiagnostic} /> : null}
      <p data-testid="numerical-result-standing">{standingLine}{hangerPreloadStanding(result) ? ` ${hangerPreloadStanding(result)}` : null}</p>
      {notices.map((notice) => <p className="muted" key={notice.id} data-testid={`results-notice-${notice.id}`}>{notice.text}</p>)}
      <SectionBasisHeadlines result={result as never} />
      {recovery?.present ? (
        <p data-testid="physics-source-evidence-native">Producer recovery receipt (recorded labels; they grant no eligibility): status {String(recovery.status)}; {recovery.caseCount} recorded case(s); {recovery.physicalCaseCount} physical case(s); ids distinct: {String(recovery.receiptIdsDistinct)}; coverage matches: {String(recovery.coverageMatches)}.</p>
      ) : null}
      <ResultControls
        familyCounts={familyCounts}
        familyFilter={familyFilter as never}
        filterText={filterText}
        filteredCount={filteredCount}
        result={result}
        totalCount={custody.summary.header.row_count}
        page={page}
        pageSize={pageSize}
        onFamilyFilterChange={(value) => { setFamilyFilter(value); setPageIndex(0); }}
        onFilterChange={(value) => { setFilterText(value); setPageIndex(0); }}
        onPageSizeChange={(value) => { setPageSize(value); setPageIndex(0); }}
        onPreviousPage={() => setPageIndex((index) => Math.max(0, index - 1))}
        onNextPage={() => setPageIndex((index) => Math.min(pageCount - 1, index + 1))}
        selectionEntityId={selectionEntityId}
        selectionOnly={selectionOnly}
        onSelectionOnlyChange={(value) => { setSelectionOnly(value); setPageIndex(0); }}
        frameCounts={frameCounts}
        frameFilter={activeFrame}
        onFrameFilterChange={(value) => { setFrameFilter(value); setPageIndex(0); }}
      />
      {kindFilter ? (
        <p className="results-kind-filter" data-testid="results-kind-filter">
          Showing only rows of kind <code>{kindFilter.kind}</code>.{" "}
          <button type="button" data-testid="results-kind-filter-clear" onClick={() => { setKindFilter(null); setPageIndex(0); }}>Show all kinds</button>
        </p>
      ) : null}
      <ResultsNavigator
        kinds={kindIds}
        kind={familyFilter as ResultFamilyFilter}
        cases={cases}
        caseKey={activeCase}
        entities={entities}
        entity={navEntity}
        currentRowId={currentRowId}
        rowCount={filteredCount}
        rowsReading={!current && !selectionOnlyWithoutEntity}
        selectionOnlyWithoutEntity={selectionOnlyWithoutEntity}
        health={health}
        endNotice={endNotice}
        entitiesReading={navigatorCurrent === null ? "reading the supports and elements natively…" : "refused" in navigatorCurrent ? `the supports and elements could not be read: ${navigatorCurrent.refused}` : null}
        canStep={(axis, delta) => stepTarget(axis, delta) !== null}
        onStep={(axis, delta) => navigate(axis, delta)}
        onChooseCase={(caseKey) => chooseCase(caseKey)}
      />
      <NativeResultDetail result={result} selectedResultId={selectedResultId} selected={selected} analysisRun={analysisRun} exportEndpointPair={async (pair) => {
        const rows = await Promise.all(pair.map(async (item) => {
          const quantity: DisplaySourceQuantity = { value: item.value, unit: item.unit, dimension_id: item.dimension ?? "unknown" };
          return { location: item.metadata?.location ?? "summary", resultId: item.id, fallback: `${item.value} ${item.unit}`, quantity, display: await displayResolver.resolve(quantity) };
        }));
        return endpointPairCsv({
          rows, provenance: { modelHash: model ? await csvModelHash(model) : CSV_NOT_RECORDED, resultHash: csvResultHash(result, csvRecord), analysisRoutes: [csvAnalysisRoute(result)] },
          softwareVersion: CSV_SOFTWARE_VERSION,
          unitBasis: await displayConversionBasis({ preference: displayResolver.preference, rows: rows.map((row) => ({ quantity: row.quantity, display: row.display })), convertRaw: displayResolver.convertRaw }),
          caseOrFilter: `endpoint pair of ${selectedResultId ?? "the selected result"}`
        });
      }} />
      <GoverningRatioState ratioCount={0} />
      {current?.status === "refused" ? <p role="alert" data-testid="result-query-refused">{current.text}</p> : null}
      {!current && !selectionOnlyWithoutEntity ? <p role="status" data-testid="result-page-reading">Reading the result rows natively…</p> : null}
      <div className="result-groups">
        {groups.length > 0 ? groups.map((group) => (
          <section key={group.title} className="result-group" aria-label={`${group.title} results`} data-testid={`result-group-${group.title.toLowerCase()}`}>
            <h3>{group.title}</h3>
            <CsvExportControl tableLabel={`${group.title} results`} testId={`result-group-${group.title.toLowerCase()}-export-csv`} stream={() => exportGroup(group.title)} />
            <table data-csv-export={resultGroupCsvKind(group.title)}>
              <thead><tr><th>ID</th><th>Entity</th><th>Location</th><th title="The coordinate frame the row's value is stated in, as the row states it">Frame</th><th>Value</th><th>Entered source</th></tr></thead>
              <tbody>
                {group.items.map((item) => (
                  <tr key={item.id} aria-selected={item.id === selectedResultId} className={item.id === selectedResultId ? "selected-result-row" : ""} data-testid={`result-row-${item.id}`}
                    onClick={() => onSelectResult(item.id)}
                    onKeyDown={(event) => { if (event.key === "Enter" || event.key === " ") { event.preventDefault(); onSelectResult(item.id); } }}
                    tabIndex={0}>
                    <td>{item.id}<small> {labels.category(item)}</small>{labelSource && resultRowLabel(item, labelSource as never) ? <small data-testid={`result-row-label-${item.id}`}> {resultRowLabel(item, labelSource as never)}</small> : null}<RowHealthBadge health={health} item={item} />{kindFilter?.withheld[item.id] ? <small data-testid={`result-row-withheld-${item.id}`}> {kindFilter.withheld[item.id]}</small> : null}{item.component_body ? <small data-testid={`result-row-component-body-${item.id}`}> reference section of component body {item.component_body.component_ref}</small> : null}</td>
                    <td>{item.entity_ref}</td>
                    <td>{item.metadata?.location ?? "summary"}</td>
                    <td data-testid={`result-row-frame-${item.id}`}>{frameLabel(resultFrame(item))}</td>
                    <td><QuantityReadout quantity={{ value: item.value, unit: item.unit, dimension_id: labels.dimension(item) ?? "unknown" }} /></td>
                    <td className="dual-unit-cell" data-testid={`result-row-dual-${item.id}`}><span>Entered: {item.value} {item.unit}</span></td>
                  </tr>
                ))}
              </tbody>
            </table>
          </section>
        )) : current?.status === "ready" ? <p className="muted" data-testid="result-filter-empty">No computed preview result rows match this filter.</p> : null}
        <MechanicsGapLedger gaps={mechanicsGaps()} />
      </div>
    </section>
  );
}

/** Whether a native result holds a row of `kind` (`results.some`), by one bounded lookup. */
function useNativeKindPresence(result: NativeResult, kind: string): boolean {
  const [found, setFound] = useState<{ of: NativeResult; present: boolean } | null>(null);
  useEffect(() => {
    let live = true;
    lookupNativeRows(result, { kinds: [kind], match: "first" }).then((lookup) => { if (live) setFound({ of: result, present: lookup.kinds[0] !== null }); }, () => { if (live) setFound({ of: result, present: false }); });
    return () => { live = false; };
  }, [result, kind]);
  return found?.of === result ? found.present : false;
}

/** A row the panel reads by bounded native lookup, with its ordinal. */
type ReadRow = Readonly<{ id: string; row: Row | null; ordinal: number | null }>;

/** The selected row (`results.find` by id), by one bounded native lookup per result and id; null
 * while it is read. A refused lookup reads as no row. */
function useNativeSelectedRow(result: NativeResult, selectedResultId: string | null): ReadRow | null {
  const [read, setRead] = useState<{ of: NativeResult } & ReadRow | null>(null);
  useEffect(() => {
    if (!selectedResultId) return;
    let live = true;
    lookupNativeRows(result, { ids: [selectedResultId], match: "first" }).then((found) => {
      const hit = found.ids[0];
      if (live) setRead({ of: result, id: selectedResultId, row: hit?.row ?? null, ordinal: hit?.ordinal ?? null });
    }, () => { if (live) setRead({ of: result, id: selectedResultId, row: null, ordinal: null }); });
    return () => { live = false; };
  }, [result, selectedResultId]);
  return selectedResultId && read?.of === result && read.id === selectedResultId ? read : null;
}

/** The id of the row at `ordinal` (its exact published row), by bounded lookup; null without one. */
function useNativeRowId(result: NativeResult, ordinal: number | null): string | null {
  const [read, setRead] = useState<{ of: NativeResult; ordinal: number; id: string | null } | null>(null);
  useEffect(() => {
    if (ordinal === null) return;
    let live = true;
    lookupNativeRows(result, { ordinals: [ordinal], match: "first" }).then(
      (found) => { if (live) setRead({ of: result, ordinal, id: found.ordinals[0]?.row.id ?? null }); },
      () => { if (live) setRead({ of: result, ordinal, id: null }); }
    );
    return () => { live = false; };
  }, [result, ordinal]);
  return ordinal !== null && read?.of === result && read.ordinal === ordinal ? read.id : null;
}

/** Whether the navigator's filter admits the row at `ordinal` (one probe of the native navigator
 * query); null while read, refused, or with no ordinal to probe. */
function useNavigatorProbe(result: NativeResult, filter: DetailFilter, filterKey: string, ordinal: number | null): boolean | null {
  const [read, setRead] = useState<{ of: NativeResult; key: string; ordinal: number; admitted: boolean } | null>(null);
  useEffect(() => {
    if (ordinal === null) return;
    let live = true;
    queryNativeNavigator(result, filter, ordinal).then(
      (answer) => { if (live && answer.probe) setRead({ of: result, key: filterKey, ordinal, admitted: answer.probe.admitted }); },
      () => undefined
    );
    return () => { live = false; };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [result, filterKey, ordinal]);
  return ordinal !== null && read?.of === result && read.key === filterKey && read.ordinal === ordinal ? read.admitted : null;
}

/** The text of a refused native read. */
function refusalText(error: unknown): string {
  return error instanceof NativeQueryRefused ? `the native result query was refused: ${error.message}` : nativeFailureText(error);
}

/** The selected row's detail, from the panel's bounded lookup: its members, its source rows' ids,
 * and its endpoint pair (the same entity's end rows of the same component, frame and basis, by query). */
function NativeResultDetail({ result, selectedResultId, selected, analysisRun, exportEndpointPair }: { result: NativeResult; selectedResultId: string | null; selected: ReadRow | null; analysisRun: SessionAnalysisRun | null; exportEndpointPair: (pair: Row[]) => Promise<CsvExport> }) {
  const [pairRead, setPairRead] = useState<{ of: ReadRow; pair: Row[] } | null>(null);
  useEffect(() => {
    if (!selected) return;
    let live = true;
    (async () => {
      const row = selected.row;
      let pair: Row[] = [];
      const metadata = row?.metadata;
      if (row && metadata && ["end_i", "end_j"].includes(metadata.location)) {
        for (const station of ["end_i", "end_j"]) {
          const page = await queryNativeRows(result, { entities: [row.entity_ref], station }, 0, 1000);
          pair.push(...page.rows.map((entry) => entry.row).filter((candidate) => candidate.metadata?.component === metadata.component && candidate.metadata?.coordinate_system === metadata.coordinate_system && candidate.metadata?.basis === metadata.basis));
        }
        if (pair.length < 2) pair = [];
      }
      if (live) setPairRead({ of: selected, pair });
    })().catch(() => { if (live) setPairRead({ of: selected, pair: [] }); });
    return () => { live = false; };
  }, [result, selected]);
  if (!selectedResultId) return null;
  if (!selected || pairRead?.of !== selected) return <p role="status" data-testid="result-detail-reading">Reading the selected result natively…</p>;
  const row = selected.row;
  const read = pairRead;
  if (!row) return <p className="muted" data-testid="result-detail-unavailable">The selected result id is not a row of this result.</p>;
  const hashes = analysisRun ? 1 : 0;
  return (
    <section className="result-detail" aria-label="Selected result" data-testid="result-detail">
      <p data-testid="result-detail-id"><strong>{row.id}</strong> {row.kind} at {row.entity_ref} ({row.metadata?.location ?? "summary"}): {row.value} {row.unit}</p>
      <p data-testid="result-detail-frame">Frame: {row.metadata?.coordinate_system ?? "result_envelope"}; component {row.metadata?.component ?? row.kind}; sign convention {row.metadata?.sign_convention ?? "not specified in result metadata"}; result value checksums in the run record: {hashes}.</p>
      {row.source_result_refs?.length ? <p data-testid="result-detail-sources">Source results: {row.source_result_refs.join(", ")}</p> : null}
      {read.pair.length ? (
        <section className="endpoint-pair" aria-label="Endpoint pair results" data-testid="endpoint-pair-table">
          <CsvExportControl tableLabel="Endpoint pair results" testId="endpoint-pair-export-csv" build={() => exportEndpointPair(read.pair)} />
          <table data-csv-export={ENDPOINT_PAIR_CSV_KIND}>
            <tbody>{read.pair.map((item) => <tr key={item.id}><td>{item.metadata?.location}</td><td>{item.id}</td><td>{item.value} {item.unit}</td></tr>)}</tbody>
          </table>
        </section>
      ) : null}
    </section>
  );
}
