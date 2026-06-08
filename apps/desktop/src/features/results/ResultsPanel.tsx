import { ChevronLeft, ChevronRight, Search, X } from "lucide-react";
import { useEffect, useMemo, useState } from "react";
import type { AnalysisRunEnvelope, DesignKnowledge, MechanicsGap, MechanicsResult, ResultInterpretation } from "../../types";
import { buildResultInterpretation, mechanicsGaps } from "./resultInterpretation";

const RESULT_PAGE_SIZE_OPTIONS = [50, 100, 200] as const;
const DEFAULT_RESULT_PAGE_SIZE = RESULT_PAGE_SIZE_OPTIONS[0];

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
  const [pageIndex, setPageIndex] = useState(0);
  const [pageSize, setPageSize] = useState<number>(DEFAULT_RESULT_PAGE_SIZE);
  const filteredResults = useMemo(
    () => (result ? filterResults(result.results, filterText) : []),
    [filterText, result]
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
            filterText={filterText}
            filteredCount={filteredResults.length}
            totalCount={result.results.length}
            page={page}
            pageSize={pageSize}
            onFilterChange={handleFilterChange}
            onPageSizeChange={handlePageSizeChange}
            onPreviousPage={() => setPageIndex((current) => Math.max(0, current - 1))}
            onNextPage={() => setPageIndex((current) => Math.min(page.pageCount - 1, current + 1))}
          />
          <ResultDetail interpretation={interpretation} />
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

function ResultControls({
  filterText,
  filteredCount,
  totalCount,
  page,
  pageSize,
  onFilterChange,
  onPageSizeChange,
  onPreviousPage,
  onNextPage
}: {
  filterText: string;
  filteredCount: number;
  totalCount: number;
  page: ResultPage;
  pageSize: number;
  onFilterChange: (value: string) => void;
  onPageSizeChange: (value: number) => void;
  onPreviousPage: () => void;
  onNextPage: () => void;
}) {
  return (
    <section className="result-controls" aria-label="Result filtering" data-testid="result-controls">
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
  const specs = [
    { title: "Displacement", match: (kind: string) => kind === "displacement_magnitude" },
    { title: "Reaction", match: (kind: string) => kind === "reaction_resultant" },
    { title: "Force", match: (kind: string) => kind.includes("_force") },
    { title: "Moment", match: (kind: string) => kind.includes("_moment") },
    { title: "Stress", match: (kind: string) => kind.includes("stress") }
  ];

  return specs
    .map((spec) => ({
      title: spec.title,
      items: resultItems.filter((item) => spec.match(item.kind))
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

function filterResults(resultItems: MechanicsResult["results"], filterText: string): MechanicsResult["results"] {
  const query = filterText.trim().toLowerCase();
  if (!query) return resultItems;

  return resultItems.filter((item) => {
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
