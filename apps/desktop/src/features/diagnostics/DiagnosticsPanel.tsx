import { isNativeResult, nativeCustody, nativeQueryInvoke, type SessionResult } from "../../services/nativeResult";
import { resultRowPages } from "../../services/resultDetail";
import { useKnowledgeAxialRow } from "../knowledge/KnowledgePanel";
import type { ReactNode } from "react";
import { ResultQuantity } from "../toolkit/ResultQuantity";
import { AlertTriangle, Search, X } from "lucide-react";
import { useEffect, useMemo, useRef, useState } from "react";
import type { DiagnosticFilterRequest } from "./diagnosticFilterRequest";
import type { DesignKnowledge, Diagnostic, DiagnosticInterpretation, MechanicsResult, PreviewModel } from "../../types";
import { buildDiagnosticInterpretation } from "../results/resultInterpretation";
import { modelDocumentDiagnostics } from "../model-data/documentDiagnostics";
import { currentLibraryRefFindings, libraryRefDiagnostics, usePublishedLibraryRefFindings } from "../library/libraryRefFindings";

export function DiagnosticsPanel({
  model,
  knowledge,
  result,
  selectedDiagnosticId,
  filterRequest = null,
  onSelectDiagnostic
}: {
  model: PreviewModel;
  knowledge: DesignKnowledge | null;
  result: SessionResult | null;
  selectedDiagnosticId: string | null;
  /** A request to show exactly this diagnostic (each new sequence applies once; a
   * cleared request clears the filter it set). */
  filterRequest?: DiagnosticFilterRequest | null;
  onSelectDiagnostic: (diagnosticId: string) => void;
}) {
  const [filterText, setFilterText] = useState(filterRequest?.diagnosticId ?? "");
  // The requested id, matched exactly while the filter text is that id.
  const [pinnedId, setPinnedId] = useState<string | null>(filterRequest?.diagnosticId ?? null);
  const pinnedRef = useRef(pinnedId);
  pinnedRef.current = pinnedId;
  const requestSequence = filterRequest?.sequence;
  const requestId = filterRequest?.diagnosticId;
  useEffect(() => {
    if (requestSequence !== undefined && requestId !== undefined) {
      setFilterText(requestId);
      setPinnedId(requestId);
    } else if (pinnedRef.current !== null) {
      setFilterText("");
      setPinnedId(null);
    }
  }, [requestSequence, requestId]);
  // W-13 §6: the document validator's findings join the model's own list.
  const documentDiagnostics = useMemo(() => modelDocumentDiagnostics(model), [model]);
  // W-13 §3.5: the store's library-reference warnings, keyed to the entity.
  const publishedLibraryRefs = usePublishedLibraryRefFindings();
  const libraryRefs = useMemo(() => libraryRefDiagnostics(currentLibraryRefFindings(model, publishedLibraryRefs)), [model, publishedLibraryRefs]);
  const diagnostics: Diagnostic[] = [
    ...model.diagnostics,
    ...documentDiagnostics,
    ...libraryRefs,
    ...(knowledge?.diagnostics ?? []),
    ...(result?.diagnostics ?? [])
  ];
  const filteredDiagnostics = useMemo(
    () => pinnedId !== null && filterText === pinnedId
      ? diagnostics.filter((item) => (item.id ?? item.code) === pinnedId)
      : filterDiagnostics(diagnostics, filterText),
    [diagnostics, filterText, pinnedId]
  );
  const nativeLinked = useNativeLinkedResults(model, knowledge, result, selectedDiagnosticId);
  const axial = useKnowledgeAxialRow(result);
  const interpretation = buildDiagnosticInterpretation({ model, knowledge, result, diagnosticId: selectedDiagnosticId, nativeLinkedResults: nativeLinked, knowledgeAxialRow: axial.row });
  return (
    <section className="panel diagnostics-panel" aria-label="Diagnostics" data-testid="diagnostics-panel">
      <div className="panel-title">
        <AlertTriangle size={16} />
        Diagnostics
      </div>
      <DiagnosticControls
        filterText={filterText}
        filteredDiagnostics={filteredDiagnostics}
        totalCount={diagnostics.length}
        onFilterChange={setFilterText}
      />
      <DiagnosticDetail interpretation={interpretation} result={result} />
      <div className="diagnostic-list">
        {filteredDiagnostics.length > 0 ? (
          filteredDiagnostics.map((item, index) => (
            <button
              className={`diagnostic ${item.severity} ${item.id === selectedDiagnosticId ? "active" : ""}`}
              data-testid={`diagnostic-${item.code}`}
              key={`${item.id ?? item.code}-${index}`}
              onClick={() => onSelectDiagnostic(item.id ?? item.code)}
              type="button"
            >
              <strong>{item.code}</strong>
              <p>{item.message}</p>
            </button>
          ))
        ) : (
          <p className="muted" data-testid="diagnostic-filter-empty">
            No diagnostics match this filter.
          </p>
        )}
      </div>
    </section>
  );
}

function DiagnosticControls({
  filterText,
  filteredDiagnostics,
  totalCount,
  onFilterChange
}: {
  filterText: string;
  filteredDiagnostics: Diagnostic[];
  totalCount: number;
  onFilterChange: (value: string) => void;
}) {
  const severityCounts = countBySeverity(filteredDiagnostics);
  return (
    <section className="diagnostic-controls" aria-label="Diagnostic filtering" data-testid="diagnostic-controls">
      <label>
        <Search size={14} aria-hidden="true" />
        <span>Filter diagnostics</span>
        <input
          aria-label="Filter diagnostics"
          data-testid="diagnostic-filter-input"
          onChange={(event) => onFilterChange(event.target.value)}
          placeholder="Code, severity, source, message, or affected ref"
          type="search"
          value={filterText}
        />
      </label>
      <span data-testid="diagnostic-filter-summary">
        {filteredDiagnostics.length} of {totalCount} diagnostics visible
      </span>
      <button
        aria-label="Clear diagnostic filter"
        data-testid="clear-diagnostic-filter"
        disabled={!filterText}
        onClick={() => onFilterChange("")}
        type="button"
      >
        <X size={14} aria-hidden="true" />
      </button>
      <div className="diagnostic-severity-summary" aria-label="Diagnostic severity summary">
        <SeverityCount label="Blocking" value={severityCounts.blocking} testId="diagnostic-severity-blocking" />
        <SeverityCount label="Errors" value={severityCounts.error} testId="diagnostic-severity-error" />
        <SeverityCount label="Warnings" value={severityCounts.warning} testId="diagnostic-severity-warning" />
        <SeverityCount label="Info" value={severityCounts.info} testId="diagnostic-severity-info" />
      </div>
    </section>
  );
}

function SeverityCount({ label, value, testId }: { label: string; value: number; testId: string }) {
  return (
    <span data-testid={testId}>
      <strong>{value}</strong> {label}
    </span>
  );
}

/** A native result's rows a diagnostic names (`affected_refs` as row ids or entity refs), in
 * publication order as `results.filter` lists them: two bounded native queries, paged, unioned by
 * ordinal. Null while they read (and for a full result). */
function useNativeLinkedResults(model: PreviewModel, knowledge: DesignKnowledge | null, result: SessionResult | null, diagnosticId: string | null): DiagnosticInterpretation["linked_results"] | null {
  const refs = useMemo(() => {
    if (!result || !isNativeResult(result) || !diagnosticId) return null;
    const diagnostic = [...model.diagnostics, ...(knowledge?.diagnostics ?? []), ...result.diagnostics].find((candidate) => (candidate.id ?? candidate.code) === diagnosticId);
    return (diagnostic?.affected_refs ?? []).slice(0, 1000);
  }, [model, knowledge, result, diagnosticId]);
  const [read, setRead] = useState<{ of: unknown; key: string; rows: DiagnosticInterpretation["linked_results"] } | null>(null);
  const key = refs ? JSON.stringify(refs) : "";
  useEffect(() => {
    if (!result || !isNativeResult(result) || !refs) return;
    let live = true;
    (async () => {
      const found = new Map<number, MechanicsResult["results"][number]>();
      if (refs.length) {
        for (const filter of [{ ids: refs }, { entities: refs }]) {
          for await (const page of resultRowPages(nativeQueryInvoke(result), nativeCustody(result).jobId, filter)) page.rows.forEach((entry) => found.set(entry.ordinal, entry.row));
        }
      }
      const rows = [...found.entries()].sort((a, b) => a[0] - b[0]).map(([, item]) => ({ id: item.id, kind: item.kind, entity_ref: item.entity_ref, value_label: `${item.value} ${item.unit}`, unit: item.unit, unit_source: "result_envelope" as const }));
      if (live) setRead({ of: result, key, rows });
    })().catch(() => { if (live) setRead({ of: result, key, rows: [] }); });
    return () => { live = false; };
  }, [result, refs, key]);
  if (!result || !isNativeResult(result)) return null;
  return read && read.of === result && read.key === key ? read.rows : null;
}

function DiagnosticDetail({ interpretation, result }: { interpretation: DiagnosticInterpretation | null; result: SessionResult | null }) {
  if (!interpretation) {
    return (
      <section className="diagnostic-detail-panel" aria-label="Diagnostic detail" data-testid="diagnostic-detail-panel">
        <h3>Diagnostic Detail</h3>
        <p className="muted">Select a diagnostic to inspect affected refs, linked results, and review context.</p>
      </section>
    );
  }

  return (
    <section className="diagnostic-detail-panel" aria-label="Diagnostic detail" data-testid="diagnostic-detail-panel">
      <h3>Diagnostic Detail</h3>
      <dl>
        <DetailLine label="Diagnostic" value={interpretation.diagnostic_id} testId="selected-diagnostic-id" />
        <DetailLine label="Code" value={interpretation.code} testId="selected-diagnostic-code" />
        <DetailLine label="Class" value={interpretation.diagnostic_class} testId="selected-diagnostic-class" />
        <DetailLine label="Severity" value={interpretation.severity} />
        <DetailLine label="Source" value={interpretation.source} />
        <DetailLine
          label="Affected refs"
          value={interpretation.affected_refs.length > 0 ? interpretation.affected_refs.join(", ") : "none"}
          testId="selected-diagnostic-affected-refs"
        />
        <DetailLine
          label="Linked results"
          value={
            interpretation.linked_results.length > 0
              ? interpretation.linked_results
                  .map((item) => <span key={item.id}>{item.id} (<ResultQuantity result={result} id={item.id} fallback={item.value_label} />; {item.entity_ref}) </span>)
              : "none"
          }
          testId="selected-diagnostic-linked-results"
        />
        <DetailLine
          label="Unit context"
          value={diagnosticUnitContext(interpretation)}
          testId="diagnostic-unit-context"
        />
        <DetailLine
          label="Knowledge"
          value={
            interpretation.linked_knowledge.length > 0
              ? interpretation.linked_knowledge.map((item) => item.id).join(", ")
              : "none linked"
          }
        />
        <DetailLine label="Explanation" value={interpretation.review_explanation} testId="selected-diagnostic-explanation" />
        <DetailLine label="Remediation" value={interpretation.remediation} testId="selected-diagnostic-remediation" />
        <DetailLine label="Provenance" value={interpretation.provenance} testId="selected-diagnostic-provenance" />
        <DetailLine label="Boundary" value={interpretation.professional_boundary} />
      </dl>
    </section>
  );
}

function diagnosticUnitContext(interpretation: DiagnosticInterpretation): string {
  const units = Array.from(new Set(interpretation.linked_results.map((item) => item.unit)));
  return [
    `linked_results=${interpretation.linked_results.length}`,
    `units=${units.length > 0 ? units.join(",") : "none"}`,
    "source=result_envelope",
    "conversion=false"
  ].join("; ");
}

function DetailLine({ label, value, testId }: { label: string; value: ReactNode; testId?: string }) {
  return (
    <div data-testid={testId}>
      <dt>{label}</dt>
      <dd>{value}</dd>
    </div>
  );
}

function countBySeverity(diagnostics: Diagnostic[]): Record<Diagnostic["severity"], number> {
  return diagnostics.reduce(
    (counts, item) => {
      counts[item.severity] += 1;
      return counts;
    },
    { blocking: 0, error: 0, warning: 0, info: 0 }
  );
}

function filterDiagnostics(diagnostics: Diagnostic[], filterText: string): Diagnostic[] {
  const query = filterText.trim().toLowerCase();
  if (!query) return diagnostics;

  return diagnostics.filter((item) => {
    const searchable = [
      item.id,
      item.code,
      item.severity,
      item.diagnostic_class,
      item.class,
      item.source,
      item.message,
      item.remediation,
      ...(item.affected_refs ?? [])
    ]
      .filter((value): value is string => Boolean(value))
      .join(" ")
      .toLowerCase();
    return searchable.includes(query);
  });
}
