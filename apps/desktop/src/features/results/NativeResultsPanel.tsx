/** C1 native custody, slice 4 (DEL-04-05 `c1-native-custody-plan.md`): the results panel for a
 * result held natively. Its counts, frames and cases are S's (`results_panel`, the ports of
 * `countResultFamilies`, `countResultFrames` and `navigatorCases`); its rows are one bounded native
 * page under the panel's filters (`panelDetailFilter`), clamped to the last page from the query's
 * total as `paginateResults` clamps; each page's labels come from one batched native row-semantics
 * call; the selected row, its endpoint pair and the intensified-row notice come by bounded native
 * lookup or query. A query the native host refuses for its arguments (a lone surrogate in a search,
 * `INGRESS-ARGS-INVALID`) is shown as that typed refusal, never as no rows. The webview holds one
 * page of rows, never the envelope. */
import { useEffect, useMemo, useState } from "react";
import { QuantityReadout } from "../display-units";
import { CsvExportControl } from "../result-export/CsvExportControl";
import { CSV_SOFTWARE_VERSION, csvAnalysisRoute, csvModelHash, csvResultHash } from "../result-export/csvProvenance";
import { CSV_NOT_RECORDED } from "../result-export/csvTable";
import type { CsvExport } from "../result-export/CsvExportControl";
import { displayConversionBasis, resultGroupCsv, type ResultRowsCsvRow } from "./resultRowsCsv";
import { useDisplayQuantityResolver, type DisplaySourceQuantity } from "../display-units";
import { hangerPreloadStanding } from "./hangerPreload";
import { knownSemanticNotices, resultRowLabel } from "./knownSemanticLimitations";
import { NUMERICAL_HEALTH_STATE_TEXT } from "./numericalHealthText";
import { rowHealthState, type NumericalHealthLocation, type NumericalHealthView } from "./numericalHealth";
import { ALL_CASES, type NavigatorCase } from "./resultsNavigator";
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
  NumericalHealthDetail,
  ResultControls,
  resultFrame,
  RowHealthBadge,
  routeHealth,
  SectionBasisHeadlines
} from "./ResultsPanel";
import {
  isNativeAnalysisRun,
  lookupNativeRows,
  nativeCustody,
  NativeQueryRefused,
  nativeQueryInvoke,
  queryNativeRows,
  readNativeRowSemantics,
  type NativeResult,
  type SessionAnalysisRun
} from "../../services/nativeResult";
import { panelDetailFilter, resultRowPages, type DetailRow, type RowSemanticsOutcome } from "../../services/resultDetail";
import { nativeFailureText } from "../../services/nativeIngress";
import type { DesignKnowledge, EntityRef, MechanicsResult, PreviewModel } from "../../types";
import type { RowLabels } from "./resultSemantics";
import type { RowSemantics } from "./resultReader";

type Row = MechanicsResult["results"][number];
type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
const PREVIEW_INTENSIFIED_KIND = "preview_intensified_bending_stress";
/** The largest group CSV this interim path assembles in the webview (the existing download route's
 * bound). Above it the export is refused by name; the native chunked writer is a remaining step. */
const GROUP_CSV_ROW_LIMIT = 200_000;
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
  const exportGroup = async (title: string): Promise<CsvExport> => {
    // Every row the filters admit in this group, across all pages, by bounded native pages.
    const rows: ResultRowsCsvRow[] = [];
    const groupFilter = { ...filter, family: FAMILY_BY_TITLE[title] ?? "other" };
    for await (const chunk of resultRowPages(nativeQueryInvoke(result), custody.jobId, groupFilter)) {
      if (chunk.total > GROUP_CSV_ROW_LIMIT) throw new Error(`RESULT-GROUP-CSV-NATIVE-STREAM-PENDING: ${chunk.total} rows in ${title}, above the ${GROUP_CSV_ROW_LIMIT} this interim export assembles; narrow the filters (case, frame, entity) to export them in parts. The native chunked writer is a remaining step.`);
      const outcomes = await readNativeRowSemantics(result, chunk.rows.map((entry) => entry.ordinal));
      const chunkLabels = labelsOf(chunk.rows, outcomes);
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
    }
    const filters = [`case: ${activeCase}`, `family: ${familyFilter}`, `frame: ${activeFrame}`, `text: ${filterText.trim() || "none"}`, `kind: ${kindFilter?.kind ?? "any"}`, `selected entity only: ${selectionOnly ? selectionEntityId ?? "none selected" : "off"}`].join("; ");
    return resultGroupCsv({
      title, rows,
      provenance: { modelHash: model ? await csvModelHash(model) : CSV_NOT_RECORDED, resultHash: csvResultHash(result, csvRecord), analysisRoutes: [csvAnalysisRoute(result)] },
      softwareVersion: CSV_SOFTWARE_VERSION,
      unitBasis: await displayConversionBasis({ preference: displayResolver.preference, rows, convertRaw: displayResolver.convertRaw }),
      caseOrFilter: `group: ${title}; ${filters}`,
      caseQualifier: activeCase !== ALL_CASES ? activeCase : undefined
    });
  };
  return (
    <section className="panel results-panel" aria-label="Results" data-testid="results-panel" data-result-custody="native">
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
      <section className="result-controls results-navigator" aria-label="Results navigator" data-testid="results-navigator">
        <label>
          <span>Load case or combination</span>
          <select aria-label="Load case or combination" data-testid="results-navigator-case-select" value={activeCase} onChange={(event) => { setCaseFilter(event.target.value); setPageIndex(0); }}>
            <option value={ALL_CASES}>All cases</option>
            {cases.map((item) => <option key={item.key} value={item.key}>{item.label ?? item.key}</option>)}
          </select>
        </label>
        <p className="muted" role="status" data-testid="results-navigator-native-note">Kind and element stepping read every filtered row; for a result held natively they are not yet available. Use the family, case, frame and entity filters to reach any row.</p>
      </section>
      <NativeResultDetail result={result} selectedResultId={selectedResultId} analysisRun={analysisRun} />
      <GoverningRatioState ratioCount={0} />
      {current?.status === "refused" ? <p role="alert" data-testid="result-query-refused">{current.text}</p> : null}
      {!current && !selectionOnlyWithoutEntity ? <p role="status" data-testid="result-page-reading">Reading the result rows natively…</p> : null}
      <div className="result-groups">
        {groups.length > 0 ? groups.map((group) => (
          <section key={group.title} className="result-group" aria-label={`${group.title} results`} data-testid={`result-group-${group.title.toLowerCase()}`}>
            <h3>{group.title}</h3>
            <CsvExportControl tableLabel={`${group.title} results`} testId={`result-group-${group.title.toLowerCase()}-export-csv`} build={() => exportGroup(group.title)} />
            <table data-csv-export={`results-${group.title.toLowerCase()}`}>
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
        )) : current ? <p className="muted" data-testid="result-filter-empty">No computed preview result rows match this filter.</p> : null}
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

/** The selected row's detail, by bounded native lookup: its members, its source rows' ids, and its
 * endpoint pair (the same entity's end rows of the same component, frame and basis, by query). */
function NativeResultDetail({ result, selectedResultId, analysisRun }: { result: NativeResult; selectedResultId: string | null; analysisRun: SessionAnalysisRun | null }) {
  const [read, setRead] = useState<{ id: string; row: Row | null; pair: Row[] } | null>(null);
  useEffect(() => {
    if (!selectedResultId) return;
    let live = true;
    (async () => {
      const found = await lookupNativeRows(result, { ids: [selectedResultId], match: "first" });
      const row = found.ids[0]?.row ?? null;
      let pair: Row[] = [];
      const metadata = row?.metadata;
      if (row && metadata && ["end_i", "end_j"].includes(metadata.location)) {
        for (const station of ["end_i", "end_j"]) {
          const page = await queryNativeRows(result, { entities: [row.entity_ref], station }, 0, 1000);
          pair.push(...page.rows.map((entry) => entry.row).filter((candidate) => candidate.metadata?.component === metadata.component && candidate.metadata?.coordinate_system === metadata.coordinate_system && candidate.metadata?.basis === metadata.basis));
        }
        if (pair.length < 2) pair = [];
      }
      if (live) setRead({ id: selectedResultId, row, pair });
    })().catch(() => { if (live) setRead({ id: selectedResultId, row: null, pair: [] }); });
    return () => { live = false; };
  }, [result, selectedResultId]);
  if (!selectedResultId) return null;
  if (!read || read.id !== selectedResultId) return <p role="status" data-testid="result-detail-reading">Reading the selected result natively…</p>;
  const row = read.row;
  if (!row) return <p className="muted" data-testid="result-detail-unavailable">The selected result id is not a row of this result.</p>;
  const hashes = analysisRun ? 1 : 0;
  return (
    <section className="result-detail" aria-label="Selected result" data-testid="result-detail">
      <p data-testid="result-detail-id"><strong>{row.id}</strong> {row.kind} at {row.entity_ref} ({row.metadata?.location ?? "summary"}): {row.value} {row.unit}</p>
      <p data-testid="result-detail-frame">Frame: {row.metadata?.coordinate_system ?? "result_envelope"}; component {row.metadata?.component ?? row.kind}; sign convention {row.metadata?.sign_convention ?? "not specified in result metadata"}; result value checksums in the run record: {hashes}.</p>
      {row.source_result_refs?.length ? <p data-testid="result-detail-sources">Source results: {row.source_result_refs.join(", ")}</p> : null}
      {read.pair.length ? (
        <table data-testid="endpoint-pair-table" aria-label="Endpoint pair results">
          <tbody>{read.pair.map((item) => <tr key={item.id}><td>{item.metadata?.location}</td><td>{item.id}</td><td>{item.value} {item.unit}</td></tr>)}</tbody>
        </table>
      ) : null}
    </section>
  );
}
