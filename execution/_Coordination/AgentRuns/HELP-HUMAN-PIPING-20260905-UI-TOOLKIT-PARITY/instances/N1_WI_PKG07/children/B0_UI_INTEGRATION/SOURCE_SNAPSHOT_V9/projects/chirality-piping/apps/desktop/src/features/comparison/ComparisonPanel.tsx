import { QuantityReadout } from "../display-units";
import type { MechanicsResult } from "../../types";
import { GitCompare } from "lucide-react";
import type { ComparisonDelta, PreviewComparison } from "../../types";

const MAX_VISIBLE_DELTAS = 4;

export function ComparisonPanel({
  comparison,
  result = null,
  onSelectResult
}: {
  comparison: PreviewComparison | null;
  result?: MechanicsResult | null;
  onSelectResult: (resultId: string) => void;
}) {
  const visibleDeltas = comparison?.result_deltas.slice(0, MAX_VISIBLE_DELTAS) ?? [];
  return (
    <section className="panel comparison-panel" aria-label="Comparison workspace" data-testid="comparison-panel">
      <div className="panel-title">
        <GitCompare size={16} aria-hidden="true" />
        Comparison
      </div>
      {comparison ? (
        <>
          <div className="comparison-summary" data-testid="comparison-summary">
            <ComparisonFact label="Reference" value={`${comparison.left.basis_ref.ref}; ${comparison.left.result_count} rows`} />
            <ComparisonFact label="Target" value={`${comparison.right.basis_ref.ref}; ${comparison.right.result_count} rows`} />
            <ComparisonFact
              label="Mapped"
              value={`${comparison.summary.comparable_result_pairs} comparable pairs; ${comparison.summary.unmatched_left_results} reference-only; ${comparison.summary.unmatched_right_results} target-only`}
            />
            <ComparisonFact
              label="Tolerance"
              value={`${comparison.summary.tolerance_status}; profile=${comparison.summary.tolerance_profile_ref}`}
              testId="comparison-tolerance-status"
            />
            <ComparisonFact
              label="Source unit evidence"
              value={comparisonUnitPolicy(comparison)}
              testId="comparison-unit-policy"
            />
            <ComparisonFact
              label="Boundary"
              value={comparisonBoundary(comparison)}
              testId="comparison-boundary"
            />
          </div>
          <p className="muted" data-testid="comparison-mapping-basis">
            {comparison.summary.mapping_basis}
          </p>
          <div className="comparison-table" data-testid="comparison-delta-table">
            <div className="comparison-delta-header" aria-hidden="true">
              <span>Target result</span>
              <span>Entity</span>
              <span>Location</span>
              <span>Delta</span>
            </div>
            {visibleDeltas.map((delta) => (
              <ComparisonRow key={delta.mapping_id} delta={delta} dimension={comparisonDimension(delta, result)} onSelectResult={onSelectResult} />
            ))}
          </div>
          <small className="report-note" data-testid="comparison-diagnostics">
            {comparison.diagnostics.length} comparison diagnostic{comparison.diagnostics.length === 1 ? "" : "s"}; top{" "}
            {visibleDeltas.length} deltas shown from {comparison.result_deltas.length} mapped pairs.
          </small>
        </>
      ) : (
        <p className="muted" data-testid="comparison-empty">
          Run mechanics preview to populate the local comparison workspace from stable result IDs and explicit source
          result references.
        </p>
      )}
    </section>
  );
}

function ComparisonFact({ label, value, testId }: { label: string; value: string; testId?: string }) {
  return (
    <div className="comparison-fact" data-testid={testId}>
      <span>{label}</span>
      <strong>{value}</strong>
    </div>
  );
}

function ComparisonRow({
  delta,
  dimension,
  onSelectResult
}: {
  delta: ComparisonDelta;
  dimension: string;
  onSelectResult: (resultId: string) => void;
}) {
  return (
    <div className="comparison-delta-shell" data-testid={`comparison-row-${safeTestId(delta.right_result_id)}`}>
      <button
        className="comparison-delta-row"
        data-testid={`comparison-select-${safeTestId(delta.right_result_id)}`}
        onClick={() => onSelectResult(delta.right_result_id)}
        type="button"
      >
        <span>
          {delta.right_result_id}
          <small>{delta.classification.replaceAll("_", " ")}</small>
        </span>
        <span>{delta.entity_ref}</span>
        <span>
        {delta.component}; {delta.location}
        </span>
        <span>
          <span data-testid={`comparison-delta-${safeTestId(delta.right_result_id)}`}><QuantityReadout quantity={{ value: delta.raw_delta, unit: delta.unit, dimension_id: dimension === "temperature" ? "temperature_interval" : dimension }} /></span>
          <small data-testid={`comparison-reference-${safeTestId(delta.right_result_id)}`}>Reference: <QuantityReadout quantity={{ value: delta.left_value, unit: delta.unit, dimension_id: dimension }} /></small>
          <small data-testid={`comparison-target-${safeTestId(delta.right_result_id)}`}>Target: <QuantityReadout quantity={{ value: delta.right_value, unit: delta.unit, dimension_id: dimension }} /></small>
        </span>
      </button>
    </div>
  );
}

function comparisonBoundary(comparison: PreviewComparison): string {
  if (
    comparison.professional_boundary.human_review_required &&
    !comparison.professional_boundary.software_makes_compliance_claim &&
    !comparison.professional_boundary.software_makes_certification_claim &&
    !comparison.professional_boundary.software_makes_sealing_claim &&
    !comparison.professional_boundary.software_makes_approval_claim &&
    !comparison.professional_boundary.software_makes_authentication_claim
  ) {
    return "review-only comparison; acceptance and professional judgment remain with the responsible engineer";
  }
  return "comparison boundary requires attention";
}

function comparisonUnitPolicy(comparison: PreviewComparison): string {
  const policy = comparison.unit_policy_evidence;
  const units = policy.matched_result_units.length > 0 ? policy.matched_result_units.join(",") : "none";
  return [
    `units=${units}`,
    `matching=equal_explicit_units`,
    `conversion=${String(policy.conversion_performed)}`,
    `tolerance=${policy.tolerance_status}`
  ].join("; ");
}

function comparisonDimension(delta: ComparisonDelta, result: MechanicsResult | null): string {
  const right = result?.results.find((row) => row.id === delta.right_result_id);
  const left = result?.results.find((row) => row.id === delta.left_result_id);
  // Do not borrow metadata from a different result value or conflicting pair.
  if (!right || right.unit !== delta.unit || right.value !== delta.right_value) return "unknown";
  if (left && (left.unit !== delta.unit || left.value !== delta.left_value || (left.dimension && left.dimension !== right.dimension))) return "unknown";
  return right.dimension ?? "unknown";
}

function safeTestId(value: string): string {
  return value.replace(/[^a-zA-Z0-9:_-]+/g, "-");
}
