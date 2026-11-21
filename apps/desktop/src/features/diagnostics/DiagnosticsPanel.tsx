import { isNativeResult, nativeCustody, nativeQueryInvoke, type SessionResult } from "../../services/nativeResult";
import { resultRowPages } from "../../services/resultDetail";
import { nativeFailureText } from "../../services/nativeIngress";
import { useKnowledgeAxialRow } from "../knowledge/KnowledgePanel";
import type { ReactNode } from "react";
import { ResultQuantity } from "../toolkit/ResultQuantity";
import { AlertTriangle, Search, X } from "lucide-react";
import { useEffect, useMemo, useRef, useState } from "react";
import type { DiagnosticFilterRequest } from "./diagnosticFilterRequest";
import type { DesignKnowledge, DiagnosticInterpretation, MechanicsResult, PreviewModel } from "../../types";
import { buildDiagnosticInterpretation } from "../results/resultInterpretation";
import { usePublishedLibraryRefFindings } from "../library/libraryRefFindings";
import { countItemsBySeverity, diagnosticsListOf, groupIssues, type IssueItem } from "./issueItems";
import { IssueItemView } from "./IssueItemView";

export function DiagnosticsPanel({
  model,
  knowledge,
  result,
  selectedDiagnosticId,
  filterRequest = null,
  onSelectDiagnostic,
  items: givenItems,
  allItems = null
}: {
  model: PreviewModel;
  knowledge: DesignKnowledge | null;
  result: SessionResult | null;
  selectedDiagnosticId: string | null;
  /** A request to show exactly this diagnostic (each new sequence applies once; a
   * cleared request clears the filter it set). */
  filterRequest?: DiagnosticFilterRequest | null;
  onSelectDiagnostic: (diagnosticId: string) => void;
  /** The Issues drawer's items this list shows (`issueItems.ts`); without them the
   * diagnostics below are grouped here. */
  items?: readonly IssueItem[];
  /** Every item the drawer lists: a requested diagnostic is found among them wherever its item
   * is filed (a solve diagnostic the model checks also found is filed under Model checks). */
  allItems?: readonly IssueItem[] | null;
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
  // The list: the model's own diagnostics, the document validator's findings, the
  // store's library-reference warnings keyed to the entity, design knowledge's, and
  // the result's. On a result with combinations, a message outside the reader-checked
  // roster is listed as supplied, unverified text (`suppliedDiagnostics`), as the
  // report quotes it. A code repeated on one object is one item (`issueItems.ts`).
  const publishedLibraryRefs = usePublishedLibraryRefFindings();
  const ownItems = useMemo(() => {
    if (givenItems) return null;
    const { diagnostics, shownDiagnostics, diagnosticOrigins } = diagnosticsListOf(model, knowledge, result, publishedLibraryRefs);
    return groupIssues({ model, diagnostics, shownDiagnostics, diagnosticOrigins });
  }, [givenItems, model, knowledge, result, publishedLibraryRefs]);
  const items = givenItems ?? ownItems ?? [];
  const diagnosticCount = items.reduce((count, item) => count + item.members.length, 0);
  const filteredItems = useMemo(
    () => pinnedId !== null && filterText === pinnedId
      ? (allItems ?? items).filter((item) => item.members.some((member) => member.source === "diagnostics" && (member.diagnostic.id ?? member.diagnostic.code) === pinnedId))
      : filterItems(items, filterText),
    [items, allItems, filterText, pinnedId]
  );
  const nativeLinked = useNativeLinkedResults(model, knowledge, result, selectedDiagnosticId);
  const axial = useKnowledgeAxialRow(result);
  const interpretation = buildDiagnosticInterpretation({ model, knowledge, result, diagnosticId: selectedDiagnosticId, nativeLinkedResults: nativeLinked.rows, knowledgeAxialRow: axial.row });
  return (
    <section className="panel diagnostics-panel" aria-label="Diagnostics" data-testid="diagnostics-panel">
      <div className="panel-title">
        <AlertTriangle size={16} />
        Diagnostics
        {givenItems ? <span className="issues-section-count" data-testid="diagnostics-item-count" aria-label={`${items.length} ${items.length === 1 ? "item" : "items"}`}>{items.length}</span> : null}
      </div>
      <DiagnosticControls
        filterText={filterText}
        filteredItems={filteredItems}
        totalCount={items.length}
        diagnosticCount={diagnosticCount}
        onFilterChange={setFilterText}
      />
      {nativeLinked.error ? <p role="status" data-testid="diagnostic-linked-results-refused">{nativeLinked.error}</p> : null}
      <DiagnosticDetail interpretation={interpretation} result={result} />
      <div className="diagnostic-list">
        {filteredItems.length > 0 ? (
          filteredItems.map((item) => {
            const first = item.members[0].diagnostic;
            const active = item.members.some((member) => member.diagnostic.id !== undefined && member.diagnostic.id === selectedDiagnosticId);
            return (
              <IssueItemView
                key={item.key}
                item={item}
                selectedDiagnosticId={selectedDiagnosticId}
                onSelectDiagnostic={onSelectDiagnostic}
                main={(body) => (
                  <button
                    className={`diagnostic ${item.severity} ${active ? "active" : ""}`}
                    data-testid={`diagnostic-${item.code}`}
                    onClick={() => onSelectDiagnostic(first.id ?? first.code)}
                    type="button"
                  >
                    <strong>{item.code}</strong>
                    {body}
                  </button>
                )}
              />
            );
          })
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
  filteredItems,
  totalCount,
  diagnosticCount,
  onFilterChange
}: {
  filterText: string;
  filteredItems: readonly IssueItem[];
  totalCount: number;
  diagnosticCount: number;
  onFilterChange: (value: string) => void;
}) {
  const severityCounts = countItemsBySeverity(filteredItems);
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
        {filteredItems.length} of {totalCount} {totalCount === 1 ? "item" : "items"} shown
        {diagnosticCount !== totalCount ? ` (${diagnosticCount} diagnostics)` : ""}
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
function useNativeLinkedResults(model: PreviewModel, knowledge: DesignKnowledge | null, result: SessionResult | null, diagnosticId: string | null): { rows: DiagnosticInterpretation["linked_results"] | null; error: string | null } {
  const refs = useMemo(() => {
    if (!result || !isNativeResult(result) || !diagnosticId) return null;
    const diagnostic = [...model.diagnostics, ...(knowledge?.diagnostics ?? []), ...result.diagnostics].find((candidate) => (candidate.id ?? candidate.code) === diagnosticId);
    return (diagnostic?.affected_refs ?? []).slice(0, 1000);
  }, [model, knowledge, result, diagnosticId]);
  const [read, setRead] = useState<{ of: unknown; key: string; rows: DiagnosticInterpretation["linked_results"]; error?: string } | null>(null);
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
    })().catch((error) => { if (live) setRead({ of: result, key, rows: [], error: `The linked result query could not be read: ${nativeFailureText(error)}` }); });
    return () => { live = false; };
  }, [result, refs, key]);
  const current = result && isNativeResult(result) && read?.of === result && read.key === key ? read : null;
  return { rows: current?.rows ?? null, error: current?.error ?? null };
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

function filterItems(items: readonly IssueItem[], filterText: string): IssueItem[] {
  const query = filterText.trim().toLowerCase();
  if (!query) return [...items];
  return items.filter((item) => item.members.some(({ diagnostic, shown }) => {
    const searchable = [
      diagnostic.id,
      diagnostic.code,
      diagnostic.severity,
      diagnostic.diagnostic_class,
      diagnostic.class,
      diagnostic.source,
      diagnostic.message,
      shown.message,
      diagnostic.remediation,
      ...(diagnostic.affected_refs ?? [])
    ]
      .filter((value): value is string => Boolean(value))
      .join(" ")
      .toLowerCase();
    return searchable.includes(query);
  }));
}
