import { AlertTriangle } from "lucide-react";
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
  const diagnostics: Diagnostic[] = [
    ...model.diagnostics,
    ...(knowledge?.diagnostics ?? []),
    ...(result?.diagnostics ?? [])
  ];
  const interpretation = buildDiagnosticInterpretation({ model, knowledge, result, diagnosticId: selectedDiagnosticId });
  return (
    <section className="panel diagnostics-panel" aria-label="Diagnostics" data-testid="diagnostics-panel">
      <div className="panel-title">
        <AlertTriangle size={16} />
        Diagnostics
      </div>
      <DiagnosticDetail interpretation={interpretation} />
      <div className="diagnostic-list">
        {diagnostics.map((item, index) => (
          <button
            className={`diagnostic ${item.severity} ${item.id === selectedDiagnosticId ? "active" : ""}`}
            data-testid={`diagnostic-${item.code}`}
            key={`${item.code}-${index}`}
            onClick={() => onSelectDiagnostic(item.id ?? item.code)}
            type="button"
          >
            <strong>{item.code}</strong>
            <p>{item.message}</p>
          </button>
        ))}
      </div>
    </section>
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
