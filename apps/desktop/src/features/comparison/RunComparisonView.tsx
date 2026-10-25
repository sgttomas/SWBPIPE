import { useMemo, useState } from "react";
import { GitCompare } from "lucide-react";
import type { RunComparisonResult } from "../history/useModelStateHistory";
import type { RunComparisonSide } from "./runComparisonSides";
import {
  SUPPORTED_RESULT_FAMILIES,
  type NotComparedReason,
  type RunComparisonPair,
  type RunRowIdentity,
  type UnmatchedReason,
  type UnmatchedRunRow
} from "./runComparison";
import { COMPARISON_PAGE_ROWS, storedValueText } from "./StateComparisonView";

// The comparison of two runs (W-12 S3b; DEL-14-01 Design
// `desktop-states-runs-comparison.md` §4.2, §4.3): each side's standing, the
// counts, the context deltas (solver, settings, load cases and combinations), the
// solve diagnostics added and removed, and a table of mapped pairs and unmatched
// rows filterable by status, family and result id, paged at `COMPARISON_PAGE_ROWS`
// rows so every row is reachable. A delta is `right − left` in the rows' own unit;
// a pair the standing gate does not admit is shown `not compared` with its
// reasons. View-only: comparison export is v0.2 (FR-CMP-010). It reports
// differences and establishes nothing about either run (F-PIP-2, F-PIP-3).

export const NOT_COMPARED_LABELS: Record<NotComparedReason, string> = {
  semantic_identity_differs: "the runs' results have different semantic identities (route or producer contract)",
  left_run_needs_recompute: "the left run reads needs_recompute",
  right_run_needs_recompute: "the right run reads needs_recompute",
  family_not_compared: `no delta for this result family (deltas are computed for ${SUPPORTED_RESULT_FAMILIES.join(", ")} results, as in the reference comparison)`,
  units_differ: "the units differ (nothing is converted)",
  value_not_numeric: "a value is not a finite number"
};

const UNMATCHED_LABELS: Record<UnmatchedReason, string> = {
  no_row_with_this_result_id: "the other run has no row with this result id",
  identity_differs: "the other run's row with this result id differs in",
  duplicate_result_id: "this result id repeats within a run, so it is not mapped",
  result_id_missing: "the row has no result id"
};

type StatusFilter = "all" | "compared" | "changed" | "not_compared" | "left_only" | "right_only";

type TableRow = {
  key: string;
  status: Exclude<StatusFilter, "all" | "changed">;
  nonzero: boolean;
  identity: RunRowIdentity;
  left: string;
  right: string;
  outcome: string;
};

function basisText(identity: RunRowIdentity): string {
  return identity.basis_ref ? `${identity.basis_ref.ref_type} ${identity.basis_ref.ref_id}` : "(none)";
}

function quantity(value: unknown, unit: string): string {
  return typeof value === "number" ? `${String(value)} ${unit}`.trim() : storedValueText(value);
}

function pairRow(pair: RunComparisonPair): TableRow {
  if (pair.status === "compared") {
    return {
      key: `pair:${pair.result_id}`, status: "compared", nonzero: pair.delta !== 0, identity: pair.identity,
      left: quantity(pair.left, pair.unit), right: quantity(pair.right, pair.unit), outcome: `Δ ${quantity(pair.delta, pair.unit)}`
    };
  }
  return {
    key: `pair:${pair.result_id}`, status: "not_compared", nonzero: false, identity: pair.identity,
    left: quantity(pair.left, pair.left_unit), right: quantity(pair.right, pair.right_unit),
    outcome: `Not compared: ${pair.reasons.map((reason) => NOT_COMPARED_LABELS[reason]).join("; ")}.`
  };
}

function unmatchedRow(row: UnmatchedRunRow, index: number): TableRow {
  const reason = row.reason === "identity_differs" ? `${UNMATCHED_LABELS.identity_differs} ${row.differing_fields.join(", ")}` : UNMATCHED_LABELS[row.reason];
  const shown = quantity(row.value, row.unit);
  return {
    key: `${row.side}:${index}`, status: row.side === "left" ? "left_only" : "right_only", nonzero: false, identity: row.identity,
    left: row.side === "left" ? shown : "—", right: row.side === "right" ? shown : "—", outcome: `Unmatched: ${reason}.`
  };
}

function SideText({ name, side }: { name: string; side: RunComparisonSide }) {
  const { standing } = side;
  return (
    <p className="muted" data-testid={`run-comparison-side-${name.toLowerCase()}`}>
      {name}: {side.description}
      {side.createdAt ? <>, recorded {side.createdAt}</> : null}
      {side.state ? <>, against saved state {side.state}</> : null}. Its result reads <strong>{standing.status}</strong> (route {standing.contract})
      {standing.findings.length ? <>: {standing.findings.join(", ")}</> : null}.
    </p>
  );
}

function testIdOf(row: TableRow): string {
  return `run-comparison-row-${row.status}-${row.identity.result_id}`.replace(/[^a-zA-Z0-9:_/-]+/g, "-");
}

export function RunComparisonView({ result, onClose }: { result: RunComparisonResult; onClose: () => void }) {
  const { comparison, left, right } = result;
  const [status, setStatus] = useState<StatusFilter>("all");
  const [family, setFamily] = useState<string>("all");
  const [idFilter, setIdFilter] = useState("");
  const [page, setPage] = useState(0);
  const filtered = <T,>(set: (value: T) => void) => (value: T) => { set(value); setPage(0); };
  const rows = useMemo(
    () => [...comparison.pairs.map(pairRow), ...comparison.unmatched.map(unmatchedRow)],
    [comparison]
  );
  const families = useMemo(() => [...new Set(rows.map((row) => row.identity.family ?? "other"))].sort(), [rows]);
  const matching = rows.filter((row) =>
    (status === "all" || (status === "changed" ? row.status === "compared" && row.nonzero : row.status === status))
    && (family === "all" || (row.identity.family ?? "other") === family)
    && (!idFilter.trim() || row.identity.result_id.includes(idFilter.trim()))
  );
  const pages = Math.max(1, Math.ceil(matching.length / COMPARISON_PAGE_ROWS));
  const shownPage = Math.min(page, pages - 1);
  const first = shownPage * COMPARISON_PAGE_ROWS;
  const visible = matching.slice(first, first + COMPARISON_PAGE_ROWS);
  const { summary } = comparison;
  const recordedNeedsRecompute = [left, right].some((side) => side.kind === "recorded" && !side.standing.eligible);
  const facts: Array<[string, string, number]> = [
    ["mapped", "Mapped row pairs", summary.mapped_pairs],
    ["compared", "Compared", summary.compared],
    ["changed", "Compared with a non-zero delta", summary.compared_nonzero],
    ["not-compared", "Not compared", summary.not_compared],
    ["left-only", "Only in the left run", summary.unmatched_left],
    ["right-only", "Only in the right run", summary.unmatched_right],
    ["context", "Context differences", summary.context_deltas],
    ["diagnostics-added", "Diagnostics added", summary.diagnostics_added],
    ["diagnostics-removed", "Diagnostics removed", summary.diagnostics_removed]
  ];
  return (
    <section className="panel state-comparison-panel" aria-label="Run comparison" data-testid="run-comparison">
      <div className="panel-title">
        <GitCompare size={16} aria-hidden="true" />
        Run comparison
      </div>
      <SideText name="Left" side={left} />
      <SideText name="Right" side={right} />
      <p className="muted" data-testid="run-comparison-basis">
        Rows are mapped automatically when their result id, family, entity, basis, dimension, component, location, metadata
        basis, coordinate system and sign convention are all equal; there is no manual mapping. A delta is right − left in
        the rows' own unit. It is computed only when both runs' results read integrity_checked, both carry the same result
        semantic identity, and the two units are equal; nothing is converted. This lists differences between two runs only
        and establishes nothing about either; human review is still required.
      </p>
      {comparison.run_gate.length ? (
        <p className="muted" role="note" data-testid="run-comparison-gate">
          No row pair is compared: {comparison.run_gate.map((reason) => NOT_COMPARED_LABELS[reason]).join("; ")}.
          {recordedNeedsRecompute
            ? " A recorded run is read from its stored result and saved snapshot only. Its live solve registration is not kept with the record, so a run on the pressure-source or retained route reads needs_recompute."
            : null}
        </p>
      ) : null}
      <div className="comparison-summary" data-testid="run-comparison-summary">
        {facts.map(([id, label, value]) => (
          <div className="comparison-fact" key={id} data-testid={`run-comparison-count-${id}`}>
            <span>{label}</span>
            <strong>{value}</strong>
          </div>
        ))}
      </div>
      <div data-testid="run-comparison-context">
        <strong>Solver, settings, load cases and combinations</strong>
        {comparison.context_deltas.length ? (
          <ul className="state-comparison-fields">
            {comparison.context_deltas.map((delta) => (
              <li key={delta.key} data-testid={`run-comparison-context-${delta.key}`.replace(/[^a-zA-Z0-9:_./-]+/g, "-")} data-kind={delta.kind}>
                <code>{delta.key}</code> <span className="muted">{delta.kind === "added" ? "only in the right run" : delta.kind === "removed" ? "only in the left run" : "changed"}</span>
                {delta.kind === "changed" ? <span>: left {storedValueText(delta.left)}; right {storedValueText(delta.right)}</span> : null}
              </li>
            ))}
          </ul>
        ) : <p className="muted">No differences.</p>}
      </div>
      <div data-testid="run-comparison-diagnostics">
        <strong>Solve diagnostics</strong>
        {comparison.diagnostics.length ? (
          <ul className="state-comparison-fields">
            {comparison.diagnostics.map((diagnostic, index) => (
              <li key={`${diagnostic.change}:${index}`} data-kind={diagnostic.change}>
                <span className="muted">{diagnostic.change === "added" ? "Only in the right run" : "Only in the left run"}:</span> <code>{diagnostic.code}</code>
                {diagnostic.affected_refs.length ? <> on {diagnostic.affected_refs.join(", ")}</> : null}
                {diagnostic.message ? <span className="muted"> ({storedValueText(diagnostic.message)})</span> : null}
              </li>
            ))}
          </ul>
        ) : <p className="muted">No diagnostic added or removed ({summary.diagnostics_common} in both).</p>}
      </div>
      <div className="report-actions" role="group" aria-label="Filter the run comparison">
        <label className="memory-pool-field">
          <span>Status</span>
          <select data-testid="run-comparison-filter-status" value={status} onChange={(event) => filtered(setStatus)(event.target.value as StatusFilter)}>
            <option value="all">All rows</option>
            <option value="compared">Compared</option>
            <option value="changed">Compared, non-zero delta</option>
            <option value="not_compared">Not compared</option>
            <option value="left_only">Only in the left run</option>
            <option value="right_only">Only in the right run</option>
          </select>
        </label>
        <label className="memory-pool-field">
          <span>Family</span>
          <select data-testid="run-comparison-filter-family" value={family} onChange={(event) => filtered(setFamily)(event.target.value)}>
            <option value="all">All families</option>
            {families.map((name) => <option key={name} value={name}>{name}</option>)}
          </select>
        </label>
        <label className="memory-pool-field">
          <span>Result id contains</span>
          <input data-testid="run-comparison-filter-id" value={idFilter} onChange={(event) => filtered(setIdFilter)(event.target.value)} />
        </label>
        <button type="button" data-testid="run-comparison-close" onClick={onClose}>Close comparison</button>
      </div>
      <table className="state-comparison-table" data-testid="run-comparison-table">
        <thead>
          <tr>
            <th scope="col">Result</th><th scope="col">Family</th><th scope="col">Entity</th><th scope="col">Basis</th>
            <th scope="col">Left</th><th scope="col">Right</th><th scope="col">Delta (right − left)</th>
          </tr>
        </thead>
        <tbody>
          {visible.map((row) => (
            <tr key={row.key} data-testid={testIdOf(row)} data-status={row.status}>
              <td><code>{row.identity.result_id}</code></td>
              <td>{row.identity.family ?? "other"}</td>
              <td><code>{row.identity.object_ref}</code></td>
              <td>{basisText(row.identity)}</td>
              <td>{row.left}</td>
              <td>{row.right}</td>
              <td>{row.outcome}</td>
            </tr>
          ))}
        </tbody>
      </table>
      <div className="report-actions" role="group" aria-label="Run comparison pages">
        <small className="report-note" data-testid="run-comparison-shown">
          {matching.length === 0 ? "No row matches the filters." : `Rows ${first + 1}–${first + visible.length} of ${matching.length}.`}
        </small>
        <button type="button" data-testid="run-comparison-page-previous" disabled={shownPage === 0} title={shownPage === 0 ? "This is the first page." : undefined} onClick={() => setPage(shownPage - 1)}>Previous</button>
        <button type="button" data-testid="run-comparison-page-next" disabled={shownPage >= pages - 1} title={shownPage >= pages - 1 ? "This is the last page." : undefined} onClick={() => setPage(shownPage + 1)}>Next</button>
      </div>
    </section>
  );
}
