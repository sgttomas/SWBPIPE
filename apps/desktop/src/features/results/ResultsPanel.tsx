import { ChevronLeft, ChevronRight, Search, X } from "lucide-react";
import { useEffect, useMemo, useState } from "react";
import type { AnalysisRunEnvelope, DesignKnowledge, MechanicsGap, MechanicsResult, ResultInterpretation } from "../../types";
import { buildResultInterpretation, mechanicsGaps } from "./resultInterpretation";
import { convertForDisplay, formatConverted } from "../../services/unitConversion";

const RESULT_PAGE_SIZE_OPTIONS = [50, 100, 200] as const;
const DEFAULT_RESULT_PAGE_SIZE = RESULT_PAGE_SIZE_OPTIONS[0];
type ResultFamily = "displacement" | "reaction" | "force" | "moment" | "stress" | "ratio" | "other";
type ResultFamilyFilter = "all" | ResultFamily;
type ResultFamilyCounts = Record<ResultFamily, number> & { total: number };
const RESULT_FAMILY_OPTIONS: { id: ResultFamilyFilter; label: string }[] = [
  { id: "all", label: "All" },
  { id: "displacement", label: "Displacement" },
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
  onSelectResult
}: {
  result: MechanicsResult | null;
  knowledge: DesignKnowledge | null;
  analysisRun: AnalysisRunEnvelope | null;
  selectedResultId: string | null;
  onSelectResult: (resultId: string) => void;
}) {
  const [filterText, setFilterText] = useState("");
  const [familyFilter, setFamilyFilter] = useState<ResultFamilyFilter>("all");
  const [pageIndex, setPageIndex] = useState(0);
  const [pageSize, setPageSize] = useState<number>(DEFAULT_RESULT_PAGE_SIZE);
  const familyCounts = useMemo(
    () => (result ? countResultFamilies(result.results) : EMPTY_RESULT_FAMILY_COUNTS),
    [result]
  );
  const filteredResults = useMemo(
    () => (result ? filterResults(result.results, filterText, familyFilter) : []),
    [familyFilter, filterText, result]
  );
  const page = useMemo(
    () => paginateResults(filteredResults, pageIndex, pageSize),
    [filteredResults, pageIndex, pageSize]
  );
  const groups = result ? groupResults(page.items) : [];
  const interpretation = result
    ? buildResultInterpretation({ result, resultId: selectedResultId, knowledge, analysisRun })
    : null;
  useEffect(() => {
    setPageIndex(0);
  }, [result?.run_id]);
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
          <ResultDetail interpretation={interpretation} />
          <GoverningRatioState ratioCount={familyCounts.ratio} />
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
                        <th>Dual (≈)</th>
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
                          <td>{item.id}</td>
                          <td>{item.entity_ref}</td>
                          <td>{item.metadata?.location ?? "summary"}</td>
                          <td>
                            {item.value} {item.unit}
                          </td>
                          <td className="dual-unit-cell" data-testid={`result-row-dual-${item.id}`}>
                            <DualUnitCompanion value={item.value} unit={item.unit} />
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
        This view reports supplied ratio rows only. It does not synthesize allowables, code criteria, compliance, or professional approval.
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
        <span>Result units</span>
        <strong>
          {unitPolicy.result_units.length > 0 ? unitPolicy.result_units.join(", ") : "none"} · {unitPolicy.result_row_count} rows ·
          entered units preserved (SI/US companion shown for reference)
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

function DualUnitCompanion({ value, unit }: { value: number; unit: string }) {
  const companion = convertForDisplay(value, unit);
  if (!companion) return <>—</>;
  return (
    <>
      ≈ {formatConverted(companion.value)} {companion.unit}
    </>
  );
}

function compareResultUnits(left: string, right: string) {
  const leftIndex = RESULT_UNIT_DISPLAY_ORDER.indexOf(left);
  const rightIndex = RESULT_UNIT_DISPLAY_ORDER.indexOf(right);
  if (leftIndex >= 0 && rightIndex >= 0) return leftIndex - rightIndex;
  if (leftIndex >= 0) return -1;
  if (rightIndex >= 0) return 1;
  return left.localeCompare(right);
}

function ResultDetail({ interpretation }: { interpretation: ResultInterpretation | null }) {
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
        <DetailLine label="Value" value={interpretation.value_label} />
        <DetailLine label="Component" value={interpretation.component} testId="selected-result-component" />
        <DetailLine label="Coordinate system" value={interpretation.coordinate_system} testId="selected-result-coordinate-system" />
        <DetailLine label="Location" value={interpretation.location} testId="selected-result-location" />
        <DetailLine label="Recovery basis" value={interpretation.recovery_basis} testId="selected-result-recovery-basis" />
        <DetailLine label="Sign convention" value={interpretation.sign_convention} testId="selected-result-sign-convention" />
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
      {interpretation.endpoint_pair ? <EndpointPairTable interpretation={interpretation} /> : null}
    </section>
  );
}

function EndpointPairTable({ interpretation }: { interpretation: ResultInterpretation }) {
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
              <td>{item.value_label}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </section>
  );
}

function DetailLine({ label, value, testId }: { label: string; value: string; testId?: string }) {
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

function groupResults(resultItems: MechanicsResult["results"]) {
  const specs: { title: string; family: ResultFamily }[] = [
    { title: "Displacement", family: "displacement" },
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
      items: resultItems.filter((item) => resultFamilyKey(item) === spec.family)
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

function countResultFamilies(resultItems: MechanicsResult["results"]): ResultFamilyCounts {
  const counts: ResultFamilyCounts = { ...EMPTY_RESULT_FAMILY_COUNTS, total: resultItems.length };
  for (const item of resultItems) {
    counts[resultFamilyKey(item)] += 1;
  }
  return counts;
}

function filterResults(
  resultItems: MechanicsResult["results"],
  filterText: string,
  familyFilter: ResultFamilyFilter
): MechanicsResult["results"] {
  const query = filterText.trim().toLowerCase();

  return resultItems.filter((item) => {
    if (familyFilter !== "all" && resultFamilyKey(item) !== familyFilter) return false;
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

function resultFamilyKey(result: MechanicsResult["results"][number]): ResultFamily {
  const kind = result.kind.toLowerCase();
  const id = result.id.toLowerCase();
  if (kind.includes("displacement") || id.includes("disp")) return "displacement";
  if (kind.includes("reaction") || id.includes("reaction")) return "reaction";
  if (kind.includes("force") || id.includes("force")) return "force";
  if (kind.includes("moment") || id.includes("moment")) return "moment";
  if (kind.includes("stress") || id.includes("stress")) return "stress";
  if (hasResultSemanticToken(kind, "ratio") || hasResultSemanticToken(id, "ratio")) return "ratio";
  return "other";
}

function hasResultSemanticToken(value: string, expected: string): boolean {
  return value.split(/[^a-z0-9]+/).includes(expected);
}
