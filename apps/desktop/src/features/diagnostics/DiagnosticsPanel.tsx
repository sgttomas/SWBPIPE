import { isNativeResult, nativeCustody, nativeQueryInvoke, type SessionResult } from "../../services/nativeResult";
import { resultRowPages } from "../../services/resultDetail";
import { nativeFailureText } from "../../services/nativeIngress";
import { useKnowledgeAxialRow } from "../knowledge/KnowledgePanel";
import type { ReactNode } from "react";
import { ResultQuantity } from "../toolkit/ResultQuantity";
import { AlertTriangle, Search, X } from "lucide-react";
import { Fragment, useEffect, useMemo, useRef, useState } from "react";
import type { DiagnosticFilterRequest } from "./diagnosticFilterRequest";
import type { DesignKnowledge, DiagnosticInterpretation, MechanicsResult, PreviewModel } from "../../types";
import { buildDiagnosticInterpretation } from "../results/resultInterpretation";
import { usePublishedLibraryRefFindings } from "../library/libraryRefFindings";
import { presentedResultDiagnostics } from "../results/suppliedDiagnostics";
import { SOLVER_MESSAGES_HEADING } from "../results/integrityStatus";
import { countItemsBySeverity, diagnosticsListOf, groupIssues, type IssueItem, type IssueMember, type IssueSource } from "./issueItems";
import { IssueItemView, NO_OWN_DETAIL_REASON } from "./IssueItemView";

const HOME_LABELS: Record<IssueSource, string> = { "model-checks": "Model checks", operation: "Operation diagnostics", diagnostics: "Diagnostics" };

export function DiagnosticsPanel({
  model,
  knowledge,
  result,
  selectedDiagnosticId,
  filterRequest = null,
  onSelectDiagnostic,
  items: givenItems,
  allItems = null,
  onPinnedChange
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
  /** Told the diagnostic this list shows as requested (null when none), so the drawer shows that
   * report's item once: here, and not again in the list that otherwise holds it. */
  onPinnedChange?: (diagnosticId: string | null) => void;
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
  const reportCount = items.reduce((count, item) => count + item.members.length, 0);
  // A requested diagnostic: its item, wherever the drawer files it, led by that report.
  const pinned = pinnedId !== null && filterText === pinnedId ? pinnedId : null;
  const isRequested = (member: IssueMember) => member.source === "diagnostics" && (member.diagnostic.id ?? member.diagnostic.code) === pinned;
  const filteredItems = useMemo(
    () => pinned !== null
      ? (allItems ?? items).filter((item) => item.members.some(isRequested))
      : filterItems(items, filterText),
    [items, allItems, filterText, pinned]
  );
  useEffect(() => { onPinnedChange?.(pinned); }, [pinned, onPinnedChange]);
  // The solver's own messages are the result's diagnostics: in the drawer's items, the members
  // `diagnosticsListOf` gives the origin "solve", which it gives to the result's diagnostics and to
  // nothing else. The list carries one heading sentence directly above the first listed item that
  // holds one, and the detail one where the selected diagnostic is one.
  const firstSolverItem = filteredItems.find((item) => item.members.some((member) => member.origin === "solve"))?.key ?? null;
  const resultDiagnostics = useMemo(() => presentedResultDiagnostics(result), [result]);
  const selectedIsSolverMessage = selectedDiagnosticId !== null && resultDiagnostics.some((item) => (item.id ?? item.code) === selectedDiagnosticId);
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
        reportCount={reportCount}
        requested={pinned !== null}
        onFilterChange={setFilterText}
      />
      {nativeLinked.error ? <p role="status" data-testid="diagnostic-linked-results-refused">{nativeLinked.error}</p> : null}
      <DiagnosticDetail interpretation={interpretation} result={result} solverMessage={selectedIsSolverMessage} />
      <div className="diagnostic-list">
        {filteredItems.length > 0 ? (
          filteredItems.map((item) => {
            const lead = pinned !== null ? item.members.find(isRequested) ?? item.lead : item.lead;
            const active = item.members.some((member) => member.diagnostic.id !== undefined && member.diagnostic.id === selectedDiagnosticId);
            const opens = lead.source === "diagnostics" && lead.ownDetail;
            return (
              <Fragment key={item.key}>
              {item.key === firstSolverItem ? <p className="muted" data-testid="diagnostic-list-solver-messages-heading">{SOLVER_MESSAGES_HEADING}</p> : null}
              <IssueItemView
                item={item}
                lead={lead}
                selectedDiagnosticId={selectedDiagnosticId}
                onSelectDiagnostic={onSelectDiagnostic}
                main={(body) => (
                  <button
                    aria-disabled={opens ? undefined : true}
                    className={`diagnostic ${item.severity} ${active ? "active" : ""}`}
                    data-testid={`diagnostic-${item.code}`}
                    onClick={() => { if (opens) onSelectDiagnostic(lead.diagnostic.id ?? lead.diagnostic.code); }}
                    title={opens ? undefined : lead.source === "diagnostics" ? NO_OWN_DETAIL_REASON : "Open the reports below to choose one."}
                    type="button"
                  >
                    {body}
                  </button>
                )}
              />
              </Fragment>
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
  reportCount,
  requested,
  onFilterChange
}: {
  filterText: string;
  filteredItems: readonly IssueItem[];
  totalCount: number;
  reportCount: number;
  /** The list shows one requested diagnostic's item. */
  requested: boolean;
  onFilterChange: (value: string) => void;
}) {
  const elsewhere = [...new Set(filteredItems.filter((item) => item.home !== "diagnostics").map((item) => HOME_LABELS[item.home]))];
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
        {requested
          ? filteredItems.length === 0
            ? "The requested report is not in the current list."
            : `Showing the requested report: ${filteredItems.length} ${filteredItems.length === 1 ? "item" : "items"}${elsewhere.length > 0 ? `, otherwise listed under ${elsewhere.join(" and ")}` : ""}.`
          : `${filteredItems.length} of ${totalCount} ${totalCount === 1 ? "item" : "items"} shown${reportCount !== totalCount ? ` (${reportCount} ${reportCount === 1 ? "report" : "reports"})` : ""}`}
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

function DiagnosticDetail({ interpretation, result, solverMessage }: { interpretation: DiagnosticInterpretation | null; result: SessionResult | null; solverMessage: boolean }) {
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
      {solverMessage ? <p className="muted" data-testid="diagnostic-detail-solver-messages-heading">{SOLVER_MESSAGES_HEADING}</p> : null}
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
