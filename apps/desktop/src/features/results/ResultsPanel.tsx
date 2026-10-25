import { PhysicsSourceEvidencePanel } from "./PhysicsSourceEvidencePanel";
import { isRetainedRoute } from "./numericalResultQuality";
import { sourceContract } from "./resultReader";
import { KnownSemanticNotices } from "./KnownSemanticNotices";
import { resultRowLabel } from "./knownSemanticLimitations";
import { componentBodyNotice } from "./componentBody";
import { retainedPrecisionStandingText } from "./retainedPrecisionStanding";
import type { ReactNode } from "react";
import { QuantityReadout } from "../display-units";
import { semanticDimension, semanticLabels, type RowLabels } from "./resultSemantics";
import { ChevronLeft, ChevronRight, Search, X } from "lucide-react";
import { useEffect, useMemo, useState } from "react";
import type { AnalysisRunEnvelope, DesignKnowledge, MechanicsGap, MechanicsResult, ResultInterpretation } from "../../types";
import { buildResultInterpretation, mechanicsGaps } from "./resultInterpretation";
import type { EntityRef } from "../../types";
import { numericalResultStanding } from "./resultStanding";
import { badgeOf, locationText, numericalHealthView, readNumericalHealth, rowHealthState, type NumericalHealthBadge, type NumericalHealthLocation, type NumericalHealthView } from "./numericalHealth";
import { DETAIL_SECTION_LABEL, LOCATED_CAUSES_LABEL, NUMERICAL_HEALTH_STATE_TEXT, SHOW_SOLVER_DIAGNOSTIC } from "./numericalHealthText";

const RESULT_PAGE_SIZE_OPTIONS = [50, 100, 200] as const;
const DEFAULT_RESULT_PAGE_SIZE = RESULT_PAGE_SIZE_OPTIONS[0];
type ResultFamily = "rotation" | "displacement" | "reaction" | "force" | "moment" | "stress" | "ratio" | "other";
type ResultFamilyFilter = "all" | ResultFamily;
type ResultFamilyCounts = Record<ResultFamily, number> & { total: number };
const RESULT_FAMILY_OPTIONS: { id: ResultFamilyFilter; label: string }[] = [
  { id: "all", label: "All" },
  { id: "displacement", label: "Displacement" },
  { id: "rotation", label: "Rotation" },
  { id: "reaction", label: "Reaction" },
  { id: "force", label: "Force" },
  { id: "moment", label: "Moment" },
  { id: "stress", label: "Stress" },
  { id: "ratio", label: "Ratio" },
  { id: "other", label: "Other" }
];
const EMPTY_RESULT_FAMILY_COUNTS: ResultFamilyCounts = {
  total: 0,
  displacement: 0,
  rotation: 0,
  reaction: 0,
  force: 0,
  moment: 0,
  stress: 0,
  ratio: 0,
  other: 0
};
const RESULT_UNIT_DISPLAY_ORDER = ["MPa", "N", "N*m", "mm", "rad"];

export function ResultsPanel({
  result,
  knowledge,
  analysisRun,
  selectedResultId,
  onSelectResult,
  numericalHealth,
  resolveCauseLocation,
  onSelectCauseLocation,
  onShowDiagnostic
}: {
  result: MechanicsResult | null;
  knowledge: DesignKnowledge | null;
  analysisRun: AnalysisRunEnvelope | null;
  selectedResultId: string | null;
  onSelectResult: (resultId: string) => void;
  /** The session's numerical health presentation for this result. Absent (a reopened
   * run), the panel reads it with no model, so no verdict-only hold applies. */
  numericalHealth?: NumericalHealthView | null;
  resolveCauseLocation?: (location: NumericalHealthLocation) => EntityRef | null;
  onSelectCauseLocation?: (entity: EntityRef) => void;
  onShowDiagnostic?: (diagnosticId: string) => void;
}) {
  const [filterText, setFilterText] = useState("");
  const [familyFilter, setFamilyFilter] = useState<ResultFamilyFilter>("all");
  // The page is the user's position in one run's rows, held with that run's id.
  // Any other run reads as page 1 in the very render that first shows it. A
  // reset effect would run only after commit, so a click landing in between
  // (a busy main thread) had its page change undone by the late reset.
  const runId = result?.run_id ?? null;
  const [pagePosition, setPagePosition] = useState<{ runId: string | null; index: number }>({ runId, index: 0 });
  const pageIndex = pagePosition.runId === runId ? pagePosition.index : 0;
  function setPageIndex(next: number | ((current: number) => number)) {
    setPagePosition((current) => {
      const index = current.runId === runId ? current.index : 0;
      return { runId, index: typeof next === "function" ? next(index) : next };
    });
  }
  const [pageSize, setPageSize] = useState<number>(DEFAULT_RESULT_PAGE_SIZE);
  // One reading of the source's row semantics per result, for every row.
  const labels = useMemo(() => (result ? semanticLabels(result) : null), [result]);
  const familyCounts = useMemo(
    () => (result && labels ? countResultFamilies(result.results, labels) : EMPTY_RESULT_FAMILY_COUNTS),
    [result, labels]
  );
  const filteredResults = useMemo(
    () => (result && labels ? filterResults(result.results, filterText, familyFilter, labels) : []),
    [familyFilter, filterText, result, labels]
  );
  const page = useMemo(
    () => paginateResults(filteredResults, pageIndex, pageSize),
    [filteredResults, pageIndex, pageSize]
  );
  const groups = result && labels ? groupResults(page.items, labels) : [];
  // Dispatch once per render; the row labels apply to preview-physics-2 and its successor only.
  const labelSource = useMemo(() => {
    try { return result && ["preview_physics", "retained_preview_physics", "retained_physics"].includes(sourceContract(result)) ? result : null; } catch { return null; }
  }, [result]);
  const interpretation = result
    ? buildResultInterpretation({ result, resultId: selectedResultId, knowledge, analysisRun })
    : null;
  const ownHealth = useMemo(() => (numericalHealth === undefined && result ? standaloneNumericalHealth(result) : null), [numericalHealth, result]);
  const route = result ? sourceContract(result) : "unsupported";
  const health = routeHealth(route, numericalHealth === undefined ? ownHealth : numericalHealth);
  useEffect(() => {
    setPageIndex((current) => Math.min(current, page.pageCount - 1));
  }, [page.pageCount]);

  function handleFilterChange(value: string) {
    setFilterText(value);
    setPageIndex(0);
  }

  function handleFamilyFilterChange(value: ResultFamilyFilter) {
    setFamilyFilter(value);
    setPageIndex(0);
  }

  function handlePageSizeChange(value: number) {
    setPageSize(value);
    setPageIndex(0);
  }

  return (
    <section className="panel results-panel" aria-label="Results" data-testid="results-panel">
      <div className="panel-title">Results</div>
      {result ? (
        <>
          {health ? (
            <p className="numerical-health-header-line" data-testid="numerical-health-header-line"><NumericalHealthBadgeText badge={health.badge} text={health.headline} testId="numerical-health-header" />{health.outputSentence ? <>. <span data-testid="numerical-health-output-sentence">{health.outputSentence}</span></> : null}</p>
          ) : null}
          {health ? <NumericalHealthDetail health={health} resolveCauseLocation={resolveCauseLocation} onSelectCauseLocation={onSelectCauseLocation} onShowDiagnostic={onShowDiagnostic} /> : null}
          <p data-testid="numerical-result-standing">{sourceContract(result) === "unsupported" ? "Unsupported numerical contract; historical values only. Needs recompute." : isRetainedRoute(sourceContract(result)) ? retainedPrecisionStandingText(result) : sourceContract(result) === "pressure_source" ? `Ordinary solve: ${result.numerical_quality!.status}. Producer recovery receipt: ${(result.source_block_recovery as {body:{status:string}}).body.status}. ${result.formulation_basis!.limitations.join(" ")} Current use is checked separately against the actual invocation and complete result evidence. Numerical checks do not establish engineering correctness.` : `Numerical integrity: ${result.numerical_quality!.status}. ${result.formulation_basis!.limitations.join(" ")} Numerical checks do not establish engineering correctness.`}</p>
          <KnownSemanticNotices result={result} testIdPrefix="results" />
          <SectionBasisHeadlines result={result} />
          <PhysicsSourceEvidencePanel result={result} />
          <ResultControls
            familyCounts={familyCounts}
            familyFilter={familyFilter}
            filterText={filterText}
            filteredCount={filteredResults.length}
            result={result}
            totalCount={result.results.length}
            page={page}
            pageSize={pageSize}
            onFamilyFilterChange={handleFamilyFilterChange}
            onFilterChange={handleFilterChange}
            onPageSizeChange={handlePageSizeChange}
            onPreviousPage={() => setPageIndex((current) => Math.max(0, current - 1))}
            onNextPage={() => setPageIndex((current) => Math.min(page.pageCount - 1, current + 1))}
          />
          <ResultDetail interpretation={interpretation} result={result} />
          <GoverningRatioState ratioCount={0} />
          <div className="result-groups">
            {groups.length > 0 ? (
              groups.map((group) => (
                <section
                  key={group.title}
                  className="result-group"
                  aria-label={`${group.title} results`}
                  data-testid={`result-group-${group.title.toLowerCase()}`}
                >
                  <h3>{group.title}</h3>
                  <table>
                    <thead>
                      <tr>
                        <th>ID</th>
                        <th>Entity</th>
                        <th>Location</th>
                        <th>Value</th>
                        <th>Entered source</th>
                      </tr>
                    </thead>
                    <tbody>
                      {group.items.map((item) => (
                        <tr
                          key={item.id}
                          aria-selected={item.id === selectedResultId}
                          className={item.id === selectedResultId ? "selected-result-row" : ""}
                          data-testid={`result-row-${item.id}`}
                          onClick={() => onSelectResult(item.id)}
                          onKeyDown={(event) => {
                            if (event.key === "Enter" || event.key === " ") {
                              event.preventDefault();
                              onSelectResult(item.id);
                            }
                          }}
                          tabIndex={0}
                        >
                          <td>{item.id}<small> {labels!.category(item)}</small>{labelSource && resultRowLabel(item, labelSource) ? <small data-testid={`result-row-label-${item.id}`}> {resultRowLabel(item, labelSource)}</small> : null}<RowHealthBadge health={health} item={item} />{item.component_body ? <small data-testid={`result-row-component-body-${item.id}`}> reference section of component body {item.component_body.component_ref}</small> : null}</td>
                          <td>{item.entity_ref}</td>
                          <td>{item.metadata?.location ?? "summary"}</td>
                          <td>
                            <QuantityReadout quantity={{ value: item.value, unit: item.unit, dimension_id: labels!.dimension(item) ?? "unknown" }} />
                          </td>
                          <td className="dual-unit-cell" data-testid={`result-row-dual-${item.id}`}>
                            <span>Entered: {item.value} {item.unit}</span>
                          </td>
                        </tr>
                      ))}
                    </tbody>
                  </table>
                </section>
              ))
            ) : (
              <p className="muted" data-testid="result-filter-empty">
                No computed preview result rows match this filter.
              </p>
            )}
            <MechanicsGapLedger gaps={mechanicsGaps()} />
          </div>
        </>
      ) : (
        <p className="muted">Run the bounded preview mechanics path to populate result summaries.</p>
      )}
    </section>
  );
}

const SECTION_BASIS_HEADLINES = [
  { key: "max_open_formula_stress_nominal_section", basis: "nominal_section", label: "Nominal section (nominal wall)" },
  { key: "max_open_formula_stress_reduced_section", basis: "reduced_section", label: "Reduced section (wall minus mill tolerance and corrosion allowance)" }
] as const;

/** T4-U6 (D-5): an exact-contract result's stress headline on each section basis,
 * labelled by its basis. Shown only when the result carries the per-basis keys
 * (pressure-free results do not). A view in this panel, not an output surface: it
 * never picks a governing basis; each output surface keeps its own policy decision. */
function SectionBasisHeadlines({ result }: { result: MechanicsResult }) {
  const present = SECTION_BASIS_HEADLINES.filter((headline) => Object.hasOwn(result.summary, headline.key));
  if (present.length === 0) return null;
  return (
    <section className="result-detail-panel" aria-label="Stress headlines by section basis" data-testid="section-basis-headlines">
      <h3>Stress Headline per Section Basis</h3>
      <dl>
        {present.map((headline) => {
          const quantity = result.summary[headline.key];
          return (
            <div key={headline.key} data-testid={`section-basis-headline-${headline.basis}`}>
              <dt>{headline.label}</dt>
              <dd>
                {quantity === null
                  ? "withheld: a stress maximum on this basis is not available for every member in every load case"
                  : quantity && typeof quantity.value === "number" && typeof quantity.unit === "string"
                    ? <><QuantityReadout quantity={{ value: quantity.value, unit: quantity.unit, dimension_id: "stress" }} /> at {quantity.location_ref} ({quantity.result_ref})</>
                    : "not readable"}
              </dd>
            </div>
          );
        })}
      </dl>
      <small>Each section basis has its own maximum; no governing basis is chosen.</small>
    </section>
  );
}

function GoverningRatioState({ ratioCount }: { ratioCount: number }) {
  const available = ratioCount > 0;
  return (
    <section aria-label="Governing ratio state" className="result-detail-panel" data-testid="governing-ratio-state">
      <h3>Governing Ratio</h3>
      <p data-testid="governing-ratio-status">
        {available
          ? `available; ${ratioCount} user-rule ratio row${ratioCount === 1 ? "" : "s"}; human review required`
          : "unavailable; no user-rule ratio rows are present in this result envelope; mechanics results remain reviewable but no governing rule-check ratio is inferred"}
      </p>
      <small>
        This view reports supplied ratio rows only; it does not synthesize allowables or code criteria.
      </small>
    </section>
  );
}

function ResultControls({
  familyCounts,
  familyFilter,
  filterText,
  filteredCount,
  result,
  totalCount,
  page,
  pageSize,
  onFamilyFilterChange,
  onFilterChange,
  onPageSizeChange,
  onPreviousPage,
  onNextPage
}: {
  familyCounts: ResultFamilyCounts;
  familyFilter: ResultFamilyFilter;
  filterText: string;
  filteredCount: number;
  result: MechanicsResult;
  totalCount: number;
  page: ResultPage;
  pageSize: number;
  onFamilyFilterChange: (value: ResultFamilyFilter) => void;
  onFilterChange: (value: string) => void;
  onPageSizeChange: (value: number) => void;
  onPreviousPage: () => void;
  onNextPage: () => void;
}) {
  const visibleFamilyOptions = RESULT_FAMILY_OPTIONS.filter(
    (option) => option.id === "all" || familyCounts[option.id as ResultFamily] > 0
  );
  const unitPolicy = buildResultUnitPolicy(result);

  return (
    <section className="result-controls" aria-label="Result filtering" data-testid="result-controls">
      <div className="report-line" data-testid="result-unit-policy">
        <span>Source result units</span>
        <strong>
          {unitPolicy.result_units.length > 0 ? unitPolicy.result_units.join(", ") : "none"} · {unitPolicy.result_row_count} rows ·
          entered units preserved; displayed quantities follow the display preference where available
        </strong>
      </div>
      <div className="result-family-row" role="group" aria-label="Result family filter">
        {visibleFamilyOptions.map((option) => {
          const count = option.id === "all" ? familyCounts.total : familyCounts[option.id as ResultFamily];
          const isActive = option.id === familyFilter;
          return (
            <button
              aria-pressed={isActive}
              className={isActive ? "result-family-active" : undefined}
              data-testid={`result-family-${option.id}`}
              key={option.id}
              onClick={() => onFamilyFilterChange(option.id)}
              type="button"
            >
              <span>{option.label}</span>
              <strong data-testid={`result-family-count-${option.id}`}>{count}</strong>
            </button>
          );
        })}
      </div>
      <div className="result-filter-row">
        <label>
          <Search size={14} aria-hidden="true" />
          <span>Filter results</span>
          <input
            aria-label="Filter result rows"
            data-testid="result-filter-input"
            onChange={(event) => onFilterChange(event.target.value)}
            placeholder="ID, entity, component, location, or basis"
            type="search"
            value={filterText}
          />
        </label>
        <span data-testid="result-filter-summary">
          {filteredCount} of {totalCount} results match filter
        </span>
        <button
          aria-label="Clear result filter"
          data-testid="clear-result-filter"
          disabled={!filterText}
          onClick={() => onFilterChange("")}
          type="button"
        >
          <X size={14} aria-hidden="true" />
        </button>
      </div>
      <div className="result-page-row" aria-label="Result page controls">
        <label className="result-page-size">
          <span>Rows</span>
          <select
            aria-label="Rows per result page"
            data-testid="result-page-size"
            onChange={(event) => onPageSizeChange(Number(event.target.value))}
            value={pageSize}
          >
            {RESULT_PAGE_SIZE_OPTIONS.map((option) => (
              <option key={option} value={option}>
                {option}
              </option>
            ))}
          </select>
        </label>
        <span data-testid="result-page-summary">
          Showing {page.start} to {page.end} of {filteredCount} matching results; page {page.pageIndex + 1} of {page.pageCount}
        </span>
        <div className="result-page-actions">
          <button
            aria-label="Previous result page"
            data-testid="previous-result-page"
            disabled={page.pageIndex === 0}
            onClick={onPreviousPage}
            type="button"
          >
            <ChevronLeft size={14} aria-hidden="true" />
            Previous
          </button>
          <button
            aria-label="Next result page"
            data-testid="next-result-page"
            disabled={page.pageIndex >= page.pageCount - 1}
            onClick={onNextPage}
            type="button"
          >
            Next
            <ChevronRight size={14} aria-hidden="true" />
          </button>
        </div>
      </div>
    </section>
  );
}

function buildResultUnitPolicy(result: MechanicsResult) {
  const resultUnits = [...new Set(result.results.map((item) => item.unit))].sort(compareResultUnits);

  return {
    evidence_id: "unit-policy-evidence:result-view-preview",
    unit_system_ref: "unit-system:dec-018-si-dual-display",
    source_result_ref: result.run_id,
    storage_convention: "entered_units_preserved",
    result_units: resultUnits,
    result_row_count: result.results.length,
    conversion_policy: "result_view_preserves_result_row_units_without_conversion",
    conversion_performed: false,
    decision_basis_refs: ["DEC-018", "DEL-02-02", "DEL-07-05"],
    private_payload_included: false,
    protected_content_included: false
  };
}

function compareResultUnits(left: string, right: string) {
  const leftIndex = RESULT_UNIT_DISPLAY_ORDER.indexOf(left);
  const rightIndex = RESULT_UNIT_DISPLAY_ORDER.indexOf(right);
  if (leftIndex >= 0 && rightIndex >= 0) return leftIndex - rightIndex;
  if (leftIndex >= 0) return -1;
  if (rightIndex >= 0) return 1;
  return left.localeCompare(right);
}

function ResultDetail({ interpretation, result }: { interpretation: ResultInterpretation | null; result: MechanicsResult | null }) {
  if (!interpretation) {
    return (
      <section className="result-detail-panel" aria-label="Result detail" data-testid="result-detail-panel">
        <h3>Result Detail</h3>
        <p className="muted">Select a computed result row to inspect recovery context and review boundary.</p>
      </section>
    );
  }

  return (
    <section className="result-detail-panel" aria-label="Result detail" data-testid="result-detail-panel">
      <h3>Result Detail</h3>
      <dl>
        <DetailLine label="Result" value={interpretation.result_id} testId="selected-result-id" />
        <DetailLine label="Family" value={interpretation.family} />
        <DetailLine label="Entity ref" value={interpretation.entity_ref} testId="selected-result-entity-ref" />
        <DetailLine label="Value" value={<SemanticResultQuantity result={result} id={interpretation.result_id} fallback={interpretation.value_label} />} />
        <DetailLine label="Component" value={interpretation.component} testId="selected-result-component" />
        <DetailLine label="Coordinate system" value={interpretation.coordinate_system} testId="selected-result-coordinate-system" />
        <DetailLine label="Location" value={interpretation.location} testId="selected-result-location" />
        <DetailLine label="Recovery basis" value={interpretation.recovery_basis} testId="selected-result-recovery-basis" />
        <DetailLine label="Sign convention" value={interpretation.sign_convention} testId="selected-result-sign-convention" />
        {(() => {
          // W-15 slice d (DEL-03-05 Design §5): a component body's row is a reference-section stress.
          const body = result?.results.find((row) => row.id === interpretation.result_id)?.component_body;
          return body ? <DetailLine label="Component body" value={componentBodyNotice(interpretation.entity_ref, body)} testId="selected-result-component-body" /> : null;
        })()}
        <DetailLine
          label="Source results"
          value={
            interpretation.source_result_refs.length > 0
              ? interpretation.source_result_refs.join(", ")
              : "not a combined result"
          }
          testId="selected-result-source-refs"
        />
        <DetailLine
          label="Diagnostics"
          value={
            interpretation.linked_diagnostics.length > 0
              ? interpretation.linked_diagnostics.map((item) => item.id ?? item.code).join(", ")
              : "none linked"
          }
        />
        <DetailLine
          label="Knowledge"
          value={
            interpretation.linked_knowledge.length > 0
              ? interpretation.linked_knowledge.map((item) => item.id).join(", ")
              : "none linked"
          }
        />
        <DetailLine
          label="Source run"
          value={`${interpretation.source_run.audit_ref}; ${interpretation.source_run.run_id}; ${interpretation.source_run.result_hash_count} result value hashes`}
        />
        <DetailLine
          label="Envelope hash"
          value={interpretation.source_run.envelope_hash_available ? "available" : "not available"}
        />
        <DetailLine label="Boundary" value={interpretation.professional_boundary} />
      </dl>
      {interpretation.endpoint_pair ? <EndpointPairTable interpretation={interpretation} result={result} /> : null}
    </section>
  );
}

function EndpointPairTable({ interpretation, result }: { interpretation: ResultInterpretation; result: MechanicsResult | null }) {
  if (!interpretation.endpoint_pair) return null;

  return (
    <section className="endpoint-pair" aria-label="Endpoint pair results" data-testid="endpoint-pair-table">
      <h4>Endpoint Pair</h4>
      <table>
        <thead>
          <tr>
            <th>Location</th>
            <th>Result</th>
            <th>Value</th>
          </tr>
        </thead>
        <tbody>
          {interpretation.endpoint_pair.values.map((item) => (
            <tr key={item.result_id}>
              <td>{item.location}</td>
              <td>{item.result_id}</td>
              <td><SemanticResultQuantity result={result} id={item.result_id} fallback={item.value_label} /></td>
            </tr>
          ))}
        </tbody>
      </table>
    </section>
  );
}

function DetailLine({ label, value, testId }: { label: string; value: ReactNode; testId?: string }) {
  return (
    <div data-testid={testId}>
      <dt>{label}</dt>
      <dd>{value}</dd>
    </div>
  );
}

function MechanicsGapLedger({ gaps }: { gaps: MechanicsGap[] }) {
  return (
    <section className="mechanics-gap-ledger" aria-label="Mechanics gap ledger" data-testid="mechanics-gap-ledger">
      <h3>Mechanics Gap Ledger</h3>
      <div className="gap-list">
        {gaps.map((gap) => (
          <article key={gap.id} data-testid={gap.id}>
            <strong>{gap.capability}</strong>
            <small>{gap.status.replaceAll("_", " ")}</small>
            <p>{gap.review_note}</p>
          </article>
        ))}
      </div>
    </section>
  );
}

function groupResults(resultItems: MechanicsResult["results"], labels: RowLabels) {
  const specs: { title: string; family: ResultFamily }[] = [
    { title: "Displacement", family: "displacement" },
    { title: "Rotation", family: "rotation" },
    { title: "Reaction", family: "reaction" },
    { title: "Force", family: "force" },
    { title: "Moment", family: "moment" },
    { title: "Stress", family: "stress" },
    { title: "Ratio", family: "ratio" },
    { title: "Other", family: "other" }
  ];

  return specs
    .map((spec) => ({
      title: spec.title,
      items: resultItems.filter((item) => resultFamilyKey(item, labels) === spec.family)
    }))
    .filter((group) => group.items.length > 0);
}

type ResultPage = {
  items: MechanicsResult["results"];
  pageIndex: number;
  pageCount: number;
  start: number;
  end: number;
};

function paginateResults(resultItems: MechanicsResult["results"], pageIndex: number, pageSize: number): ResultPage {
  const pageCount = Math.max(1, Math.ceil(resultItems.length / pageSize));
  const safePageIndex = Math.min(Math.max(pageIndex, 0), pageCount - 1);
  const startIndex = safePageIndex * pageSize;
  const endIndex = Math.min(startIndex + pageSize, resultItems.length);

  return {
    items: resultItems.slice(startIndex, endIndex),
    pageIndex: safePageIndex,
    pageCount,
    start: resultItems.length === 0 ? 0 : startIndex + 1,
    end: endIndex
  };
}

function countResultFamilies(resultItems: MechanicsResult["results"], labels: RowLabels): ResultFamilyCounts {
  const counts: ResultFamilyCounts = { ...EMPTY_RESULT_FAMILY_COUNTS, total: resultItems.length };
  for (const item of resultItems) {
    counts[resultFamilyKey(item, labels)] += 1;
  }
  return counts;
}

function filterResults(
  resultItems: MechanicsResult["results"],
  filterText: string,
  familyFilter: ResultFamilyFilter,
  labels: RowLabels
): MechanicsResult["results"] {
  const query = filterText.trim().toLowerCase();

  return resultItems.filter((item) => {
    if (familyFilter !== "all" && resultFamilyKey(item, labels) !== familyFilter) return false;
    if (!query) return true;
    const searchable = [
      item.id,
      item.kind,
      item.entity_ref,
      item.unit,
      String(item.value),
      item.basis_ref?.ref_type,
      item.basis_ref?.ref_id,
      item.metadata?.component,
      item.metadata?.coordinate_system,
      item.metadata?.location,
      item.metadata?.basis,
      item.metadata?.sign_convention,
      ...(item.source_result_refs ?? [])
    ]
      .filter((value): value is string => Boolean(value))
      .join(" ")
      .toLowerCase();
    return searchable.includes(query);
  });
}

function resultFamilyKey(result: MechanicsResult["results"][number], labels: RowLabels): ResultFamily {
  return labels.family(result) as ResultFamily;
}
function SemanticResultQuantity({result,id,fallback}:{result:MechanicsResult|null;id:string;fallback:string}) {
  const row=result?.results.find(x=>x.id===id);
  return row?<QuantityReadout quantity={{value:row.value,unit:row.unit,dimension_id:semanticDimension(row, result ?? undefined)??"unknown"}}/>:<span>{fallback}</span>;
}

function hasResultSemanticToken(value: string, expected: string): boolean {
  return value.split(/[^a-z0-9]+/).includes(expected);
}

/* -------------------------------------------------------------------------- *
 * Numerical health (DEL-07-05 Design `numerical-health-indicator.md` §3, §4)
 * -------------------------------------------------------------------------- */
/** A reopened run's indicator: read with the reader's findings for the result alone (no
 * model, so the verdict-only flag cannot hold) and no hold. */
function standaloneNumericalHealth(result: MechanicsResult): NumericalHealthView | null {
  let findings: readonly string[] | null = null, eligible = false;
  try { const standing = numericalResultStanding(result); findings = standing.findings; eligible = standing.eligible; } catch { return null; }
  return numericalHealthView(readNumericalHealth(result, findings), { eligible, verdictHold: false });
}
/** The indicator only where its route agrees with the result's route; otherwise none
 * (today's text). */
function routeHealth(route: string, health: NumericalHealthView | null | undefined): NumericalHealthView | null {
  if (!health) return null;
  if (health.route === "pressure_source") return route === "pressure_source" ? health : null;
  return route === "preview_physics" || route === "pressure" ? health : null;
}

function NumericalHealthBadgeText({ badge, text, testId }: { badge: NumericalHealthBadge; text: string; testId?: string }) {
  return <span className={`numerical-health-badge numerical-health-${badge}`} data-badge={badge} data-testid={testId}>{text}</span>;
}

function RowHealthBadge({ health, item }: { health: NumericalHealthView | null; item: MechanicsResult["results"][number] }) {
  // No row badge on the pressure-source route: those rows' values are the producer's recovery.
  const state = rowHealthState(health, item);
  if (!state) return null;
  return <small> <NumericalHealthBadgeText badge={badgeOf(state)} text={NUMERICAL_HEALTH_STATE_TEXT[state]} testId={`result-row-health-${item.id}`} /></small>;
}

function NumericalHealthDetail({ health, resolveCauseLocation, onSelectCauseLocation, onShowDiagnostic }: {
  health: NumericalHealthView;
  resolveCauseLocation?: (location: NumericalHealthLocation) => EntityRef | null;
  onSelectCauseLocation?: (entity: EntityRef) => void;
  onShowDiagnostic?: (diagnosticId: string) => void;
}) {
  return (
    // Collapsed when every load case passed its checks; open otherwise.
    <details className="result-detail-panel numerical-health-detail" aria-label={DETAIL_SECTION_LABEL} data-testid="numerical-health-detail" open={health.cases.some((c) => c.state !== "checks_passed") || health.cases.length === 0}>
      <summary>{DETAIL_SECTION_LABEL}</summary>
      {health.cases.map((c) => (
        <article key={c.caseId} data-testid={`numerical-health-case-${c.caseId}`}>
          <h4>{c.caseId} <NumericalHealthBadgeText badge={c.badge} text={c.headline} testId={`numerical-health-case-badge-${c.caseId}`} /></h4>
          <p data-testid={`numerical-health-case-text-${c.caseId}`}>{c.sentences.join(" ")}</p>
          {c.conditioning ? <p data-testid={`numerical-health-case-conditioning-${c.caseId}`}>{c.conditioning}</p> : null}
          {c.locations.length > 0 ? (
            <div data-testid={`numerical-health-case-locations-${c.caseId}`}>
              <span>{LOCATED_CAUSES_LABEL}: </span>
              <ul>
                {c.locations.map((location, index) => {
                  const entity = location.kind === "text" ? null : resolveCauseLocation?.(location) ?? null;
                  return (
                    <li key={`${index}:${locationText(location)}`}>
                      {entity && onSelectCauseLocation
                        ? <button type="button" className="link-button" data-testid={`numerical-health-location-${c.caseId}-${index}`} onClick={() => onSelectCauseLocation(entity)}>{locationText(location)}</button>
                        : <span data-testid={`numerical-health-location-${c.caseId}-${index}`}>{locationText(location)}</span>}
                    </li>
                  );
                })}
              </ul>
            </div>
          ) : null}
          {c.integrityDiagnosticId && onShowDiagnostic ? (
            <button type="button" data-testid={`numerical-health-show-diagnostic-${c.caseId}`} onClick={() => onShowDiagnostic(c.integrityDiagnosticId!)}>{SHOW_SOLVER_DIAGNOSTIC}</button>
          ) : null}
        </article>
      ))}
      {health.combinations.map((m) => (
        <article key={m.combinationId} data-testid={`numerical-health-combination-${m.combinationId}`}>
          <h4>{m.combinationId} <NumericalHealthBadgeText badge={m.badge} text={NUMERICAL_HEALTH_STATE_TEXT[m.state]} testId={`numerical-health-combination-badge-${m.combinationId}`} /></h4>
          <p>{m.text}</p>
        </article>
      ))}
    </details>
  );
}
