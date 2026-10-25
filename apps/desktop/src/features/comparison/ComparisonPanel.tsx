import { semanticLabels, type RowLabels } from "../results/resultSemantics";
import { sourceContract } from "../results/resultReader";
import { KnownSemanticNotices } from "../results/KnownSemanticNotices";
import { QuantityReadout } from "../display-units";
import type { MechanicsResult } from "../../types";
import { GitCompare } from "lucide-react";
import type { ComparisonDelta, PreviewComparison } from "../../types";
import { type WithinRunBasisOption, type WithinRunComparisonBuild, type WithinRunPair, type WithinRunView } from "./withinRunComparison";
import { ComparisonPackageNotice } from "./ComparisonPackageNotice";

const MAX_VISIBLE_DELTAS = 4;

/** "Within this run" (W-12 S3c): the session's view of the case and combination
 * choice, the reason it is disabled, the choice handler and the id of the
 * comparison the report, handoff and export packages carry. */
export type WithinRunControls = {
  view: WithinRunView;
  disabledReason: string | null;
  onSelect: (pair: WithinRunPair) => void;
  /** What the packages carry (the choice's outcome, else the fixed pair's). */
  packaged: WithinRunComparisonBuild | null;
  /** The bases of a choice dropped because the Current run does not offer them. */
  dropped: string[] | null;
};

export function ComparisonPanel({
  comparison,
  result = null,
  onSelectResult,
  withinRun
}: {
  comparison: PreviewComparison | null;
  result?: MechanicsResult | null;
  onSelectResult: (resultId: string) => void;
  /** Absent, the panel shows `comparison` as given, with no choice. */
  withinRun?: WithinRunControls;
}) {
  const visibleDeltas = comparison?.result_deltas.slice(0, MAX_VISIBLE_DELTAS) ?? [];
  const view = withinRun?.view;
  return (
    <section className="panel comparison-panel" aria-label="Comparison workspace" data-testid="comparison-panel">
      <div className="panel-title">
        <GitCompare size={16} aria-hidden="true" />
        Comparison
      </div>
      <KnownSemanticNotices result={result} testIdPrefix="comparison" />
      {withinRun && view && view.status !== "no_result" ? <WithinRunChoice controls={withinRun} /> : null}
      {view?.status === "unavailable" ? (
        <p className="muted" data-testid="comparison-unavailable">{view.reason}</p>
      ) : view?.status === "ready" && view.refusal ? (
        <p role="status" data-testid="comparison-refused">Not compared: {view.refusal}</p>
      ) : comparison ? (
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
            {(() => {
              // One reading of the source's row semantics for every delta.
              const labels = result ? semanticLabels(result) : null;
              return visibleDeltas.map((delta) => (
                <ComparisonRow key={delta.mapping_id} delta={delta} dimension={comparisonDimension(delta, result, labels)} onSelectResult={onSelectResult} />
              ));
            })()}
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

function basisOptionText(option: WithinRunBasisOption): string {
  const name = option.label ? `${option.label} (${option.ref_id})` : option.ref_id;
  return `${name}; ${option.row_count} result row${option.row_count === 1 ? "" : "s"}`;
}

function WithinRunChoice({ controls }: { controls: WithinRunControls }) {
  const { view, disabledReason, onSelect, packaged, dropped } = controls;
  if (view.status === "no_result") return null;
  const pair = view.status === "ready" ? view.pair : null;
  const reasonId = "comparison-within-run-disabled-reason";
  const disabled = disabledReason !== null;
  return (
    <fieldset className="comparison-within-run" data-testid="comparison-within-run">
      <legend>Within this run</legend>
      {pair ? (
        <div className="comparison-within-run-selects">
          <label>
            Load case
            <select
              data-testid="comparison-left-select"
              value={pair.left}
              disabled={disabled}
              title={disabledReason ?? undefined}
              aria-describedby={disabled ? reasonId : undefined}
              onChange={(event) => onSelect({ left: event.target.value, right: pair.right })}
            >
              {view.options.loadCases.map((option) => (
                <option key={option.ref_id} value={option.ref_id}>{basisOptionText(option)}</option>
              ))}
            </select>
          </label>
          <label>
            Combination
            <select
              data-testid="comparison-right-select"
              value={pair.right}
              disabled={disabled}
              title={disabledReason ?? undefined}
              aria-describedby={disabled ? reasonId : undefined}
              onChange={(event) => onSelect({ left: pair.left, right: event.target.value })}
            >
              {view.options.combinations.map((option) => (
                <option key={option.ref_id} value={option.ref_id}>{basisOptionText(option)}</option>
              ))}
            </select>
          </label>
        </div>
      ) : null}
      {pair && disabled ? <small id={reasonId} className="muted" data-testid="comparison-within-run-disabled-reason">{disabledReason}</small> : null}
      <ComparisonPackageNotice packaged={packaged} dropped={dropped} testId="comparison-package-basis" />
    </fieldset>
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
    return "review-only comparison";
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

function comparisonDimension(delta: ComparisonDelta, result: MechanicsResult | null, labels: RowLabels | null): string {
  const right = result?.results.find((row) => row.id === delta.right_result_id);
  const left = result?.results.find((row) => row.id === delta.left_result_id);
  // Do not borrow metadata from a different result value or conflicting pair.
  if (!right || right.unit !== delta.unit || right.value !== delta.right_value) return "unknown";
  if (left && (left.unit !== delta.unit || left.value !== delta.left_value || (left.dimension && left.dimension !== right.dimension))) return "unknown";
  if (!result || !labels) return "unknown";
  const contract = sourceContract(result);
  // B7b: the exact routes are pressure-2 and pressure-source-1 (the v3 twins of the
  // retired physics-1 and physics-source-1).
  if (!["pressure", "pressure_source", "preview_physics"].includes(contract)) return "unknown";
  const rightDimension = labels.dimension(right);
  const leftDimension = left ? labels.dimension(left) : null;
  if (!left || !rightDimension || leftDimension !== rightDimension) return "unknown";
  if ((right.dimension && right.dimension !== rightDimension) || (left.dimension && left.dimension !== leftDimension)) return "unknown";
  return rightDimension;
}

function safeTestId(value: string): string {
  return value.replace(/[^a-zA-Z0-9:_-]+/g, "-");
}
