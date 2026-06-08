import { AlertTriangle, Search, X } from "lucide-react";
import { useMemo, useState } from "react";
import type { DesignKnowledge, Diagnostic, DiagnosticInterpretation, MechanicsResult, PreviewModel } from "../../types";
import { buildDiagnosticInterpretation } from "../results/resultInterpretation";

export function DiagnosticsPanel({
  model,
  knowledge,
  result,
  selectedDiagnosticId,
  onSelectDiagnostic
}: {
  model: PreviewModel;
  knowledge: DesignKnowledge | null;
  result: MechanicsResult | null;
  selectedDiagnosticId: string | null;
  onSelectDiagnostic: (diagnosticId: string) => void;
}) {
  const [filterText, setFilterText] = useState("");
  const diagnostics: Diagnostic[] = [
    ...model.diagnostics,
    ...(knowledge?.diagnostics ?? []),
    ...(result?.diagnostics ?? [])
  ];
  const filteredDiagnostics = useMemo(
    () => filterDiagnostics(diagnostics, filterText),
    [diagnostics, filterText]
  );
  const interpretation = buildDiagnosticInterpretation({ model, knowledge, result, diagnosticId: selectedDiagnosticId });
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
      <DiagnosticDetail interpretation={interpretation} />
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

function DiagnosticDetail({ interpretation }: { interpretation: DiagnosticInterpretation | null }) {
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
                  .map((item) => `${item.id} (${item.value_label}; ${item.entity_ref})`)
                  .join(", ")
              : "none"
          }
          testId="selected-diagnostic-linked-results"
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
        <DetailLine label="Boundary" value={interpretation.professional_boundary} />
      </dl>
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
      item.source,
      item.message,
      ...(item.affected_refs ?? [])
    ]
      .filter((value): value is string => Boolean(value))
      .join(" ")
      .toLowerCase();
    return searchable.includes(query);
  });
}
