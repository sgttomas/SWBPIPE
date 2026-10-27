/** The weight and centre-of-gravity summary panel (survey item A9; DEL-07-05 Design
 * `weight-cog-summary.md`). For the whole model, one line or the session's selected pipes,
 * it sums the weights the self-weight generator gives the pipes and components and forms
 * their centre of gravity. Model data: no solve is needed, and the generator's plan is read,
 * never queued or applied. The user enters the gravity; none is assumed.
 *
 * Self-contained: another surface (W-17's key-results panel, A6) can embed it with the same
 * props. Values follow the report's display rule (`reportQuantityDisplay.ts`). A view only: it
 * changes no model, result or export, and it persists nothing. */

import { useMemo, useRef, useState } from "react";
import { computeModelHash } from "../../services/hashService";
import { generateSelfWeightPlan } from "../../services/selfWeightPlanService";
import type { PreviewModel } from "../../types";
import { displayQuantity, displayQuantityParts } from "../report/reportQuantityDisplay";
import {
  computeWeightSummary,
  parseWeightGravity,
  weightSummaryScopes,
  type CategoryTotal,
  type WeightSummary
} from "./weightSummary";

export const WEIGHT_SUMMARY_TITLE = "Weight and Centre of Gravity";
export const WEIGHT_SUMMARY_LIST_LIMIT = 50;
export const WEIGHT_SUMMARY_INTRO =
  "Sums the weights the self-weight generator gives the pipes and components in scope, and their centre of gravity. It reads the model; no solve is needed, and the model is not changed. Enter the gravity to use; none is assumed.";
export const WEIGHT_SUMMARY_BOUNDARY_TEXT =
  "A summary of the model's stated weights as the self-weight generator gives them. It is not a lifting, rigging or structural weight, carries no allowance or contingency, and qualifies nothing.";

type GenerateFn = Parameters<typeof computeWeightSummary>[0]["generate"];
type HashFn = Parameters<typeof computeWeightSummary>[0]["hash"];

export type WeightSummaryPanelProps = Readonly<{
  model: PreviewModel;
  /** The session's selected pipes; a "Selected pipes" scope is offered when there are any. */
  selectedPipeRefs?: readonly string[];
  /** The self-weight generator; defaults to `generateSelfWeightPlan` (native command or Wasm engine). */
  generate?: GenerateFn;
  /** The canonical model hash; defaults to `computeModelHash`. */
  hash?: HashFn;
  /** A prefix for the panel's test ids, so two embeddings stay distinct. */
  testIdPrefix?: string;
}>;

type Computed = { summary: WeightSummary; model: PreviewModel; scopeId: string; refsKey: string; gravityKey: string };

const AXIS_LABELS = [["global_x", "Global X"], ["global_y", "Global Y"], ["global_z", "Global Z"]] as const;

/** The display rule's text, or a statement in place of a value it cannot show: the panel never throws. */
function safeQuantity(value: number, unit: string): string {
  try {
    return displayQuantity(value, unit);
  } catch {
    return "outside the representable range";
  }
}

function safeNumber(value: number, unit: string): { number: string; unit: string } {
  try {
    return displayQuantityParts(value, unit);
  } catch {
    return { number: "outside the representable range", unit };
  }
}

function totalsText(category: CategoryTotal): string {
  return category.totals.length > 0 ? category.totals.map((item) => safeQuantity(item.weight, item.unit)).join("; ") : "none";
}

const count = (value: number, word: string) => `${value} ${word}${value === 1 ? "" : "s"}`;

/** What a partial summary leaves out, in one phrase (Design §6). */
export function partialQualifier(summary: Extract<WeightSummary, { state: "available" }>): string {
  const parts = [`${summary.countedPipeCount} of ${count(summary.scope.pipeRefs.length, "pipe")} counted`];
  if (summary.componentsNotIncluded.length > 0) parts.push(`${count(summary.componentsNotIncluded.length, "component weight")} not included`);
  if (summary.scope.unresolved.length > 0) parts.push(`${count(summary.scope.unresolved.length, "unresolved scope reference")}`);
  return parts.join("; ");
}

export function WeightSummaryPanel({
  model, selectedPipeRefs = [], generate = generateSelfWeightPlan, hash = computeModelHash, testIdPrefix = "weight-summary"
}: WeightSummaryPanelProps) {
  const scopes = useMemo(() => weightSummaryScopes(model, selectedPipeRefs), [model, selectedPipeRefs]);
  const [input, setInput] = useState({ scopeId: "model", value: "", unit: "", axis: "" });
  const [computed, setComputed] = useState<Computed | null>(null);
  const [pending, setPending] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const token = useRef(0);
  const scope = scopes.find((item) => item.id === input.scopeId) ?? scopes[0];
  // Resolved and unresolved references both: a newly unresolved reference changes the summary.
  const refsKey = JSON.stringify([scope.pipeRefs, scope.unresolved]);
  const gravityKey = JSON.stringify([input.value.trim(), input.unit.trim(), input.axis]);
  const current = computed !== null && computed.model === model && computed.scopeId === scope.id
    && computed.refsKey === refsKey && computed.gravityKey === gravityKey;
  const tid = (suffix: string) => `${testIdPrefix}-${suffix}`;

  async function compute() {
    const parsed = parseWeightGravity(input);
    if ("error" in parsed) { setError(parsed.error); return; }
    const mine = ++token.current;
    setError(null);
    setPending(true);
    const basis = { model, scopeId: scope.id, refsKey, gravityKey };
    try {
      const summary = await computeWeightSummary({ model, scope, gravity: parsed.gravity, hash, generate });
      if (mine === token.current) setComputed({ summary, ...basis });
    } catch (caught) {
      if (mine === token.current) setError(caught instanceof Error ? caught.message : String(caught));
    } finally {
      if (mine === token.current) setPending(false);
    }
  }

  const summary = current ? computed.summary : null;
  return (
    <section className="result-detail-panel weight-summary" aria-label="Weight and centre of gravity" data-testid={tid("panel")}>
      <h3>{WEIGHT_SUMMARY_TITLE}</h3>
      <p>{WEIGHT_SUMMARY_INTRO}</p>
      <div className="weight-summary-controls">
        <label>
          Scope{" "}
          <select data-testid={tid("scope")} value={scope.id} onChange={(event) => setInput({ ...input, scopeId: event.target.value })}>
            {scopes.map((item) => <option key={item.id} value={item.id}>{item.label}</option>)}
          </select>
        </label>
        <label>
          Gravity value{" "}
          <input data-testid={tid("gravity-value")} inputMode="decimal" value={input.value} onChange={(event) => setInput({ ...input, value: event.target.value })} />
        </label>
        <label>
          Gravity unit{" "}
          <input data-testid={tid("gravity-unit")} value={input.unit} placeholder="m/s^2" onChange={(event) => setInput({ ...input, unit: event.target.value })} />
        </label>
        <label>
          Gravity direction{" "}
          <select data-testid={tid("gravity-axis")} value={input.axis} onChange={(event) => setInput({ ...input, axis: event.target.value })}>
            <option value="">Choose an axis</option>
            {AXIS_LABELS.map(([value, label]) => <option key={value} value={value}>{label}</option>)}
          </select>
        </label>
        <button type="button" data-testid={tid("compute")} disabled={pending} onClick={() => void compute()}>Compute summary</button>
      </div>
      {pending ? <p role="status" data-testid={tid("pending")}>Computing the summary…</p> : null}
      {error ? <p role="alert" data-testid={tid("error")}>{error}</p> : null}
      {computed !== null && !current && !pending ? (
        <p className="muted" data-testid={tid("stale")}>The model, scope or gravity has changed since the last summary; compute it again.</p>
      ) : null}
      {summary?.state === "unavailable" ? (
        <div data-testid={tid("unavailable")}>
          <p>{summary.reason}</p>
          {summary.diagnostics.length > 0 ? (
            <ul>{summary.diagnostics.map((item, index) => <li key={`${item.code}-${index}`}>{item.code}: {item.message} {item.remediation}</li>)}</ul>
          ) : null}
        </div>
      ) : null}
      {summary?.state === "available" ? <SummaryView summary={summary} tid={tid} /> : null}
      <small>{WEIGHT_SUMMARY_BOUNDARY_TEXT}</small>
    </section>
  );
}

function SummaryView({ summary, tid }: { summary: Extract<WeightSummary, { state: "available" }>; tid: (suffix: string) => string }) {
  const counted = summary.pipes.count + summary.components.count;
  const lengthUnit = safeNumber(0, summary.lengthUnit).unit;
  const gravity = safeQuantity(summary.gravity.value, summary.gravity.unit);
  const axis = AXIS_LABELS.find(([value]) => value === summary.gravity.axis)?.[1] ?? summary.gravity.axis;
  const notCounted = summary.notCounted;
  const notIncluded = summary.componentsNotIncluded;
  const qualifier = summary.partial ? partialQualifier(summary) : null;
  const totalLabel = qualifier ? `Partial total (${qualifier})` : "Total";
  const cogLabel = qualifier ? "Centre of gravity of the counted weights (partial)" : "Centre of gravity";
  return (
    <div data-testid={tid("summary")} data-scope={summary.scope.id} data-partial={summary.partial ? "true" : "false"}>
      {qualifier ? (
        <p className="weight-summary-partial" role="note" data-testid={tid("partial")}>
          Partial: {qualifier}. The total and centre of gravity are those of the counted weights only; what is left out is listed below.
        </p>
      ) : null}
      <table aria-label={`Weight summary: ${summary.scope.label}`} data-testid={tid("table")}>
        <thead>
          <tr><th scope="col">Category</th><th scope="col">Counted</th><th scope="col">Weight</th></tr>
        </thead>
        <tbody>
          <tr data-testid={tid("row-pipes")}>
            <th scope="row">Pipes (metal with any stated contents and insulation)</th>
            <td>{summary.pipes.count}</td>
            <td>{totalsText(summary.pipes)}</td>
          </tr>
          <tr data-testid={tid("row-components")}>
            <th scope="row">Components (weight in place of their member's)</th>
            <td>{summary.components.count}</td>
            <td>{totalsText(summary.components)}</td>
          </tr>
          <tr data-testid={tid("row-total")}>
            <th scope="row">{totalLabel}</th>
            <td>{counted}</td>
            <td>{summary.total ? safeQuantity(summary.total.weight, summary.total.unit) : <small>{summary.note}</small>}</td>
          </tr>
        </tbody>
      </table>
      {summary.centreOfGravity ? (
        <table aria-label={cogLabel} data-testid={tid("cog")}>
          <thead>
            <tr><th scope="col">{cogLabel} ({lengthUnit})</th><th scope="col">X</th><th scope="col">Y</th><th scope="col">Z</th></tr>
          </thead>
          <tbody>
            <tr>
              <th scope="row">Global</th>
              {(["x", "y", "z"] as const).map((axisKey) => {
                const parts = safeNumber(summary.centreOfGravity![axisKey], summary.lengthUnit);
                return <td key={axisKey} data-testid={tid(`cog-${axisKey}`)} title={`${parts.number} ${parts.unit}`}>{parts.number}</td>;
              })}
            </tr>
          </tbody>
        </table>
      ) : (
        <p data-testid={tid("no-cog")}>{summary.note}</p>
      )}
      <p data-testid={tid("basis")}>
        Pipe weights: the generator's intensity under gravity {gravity} along {axis}, over each pipe's straight span, acting at its midpoint. Component weights: as the generator normalizes them, at the node plus the stated centre-of-gravity offset. Contents entered the mass of {summary.contentsCount} of {summary.pipes.count} counted pipes, insulation of {summary.insulationCount}. The breakdown into pipe metal, contents and insulation is not given by the generator.
      </p>
      {summary.scope.unresolved.length > 0 ? (
        <small data-testid={tid("unresolved")}>{summary.scope.unresolved.length} scope reference{summary.scope.unresolved.length === 1 ? " names" : "s name"} no model pipe: {summary.scope.unresolved.join(", ")}.</small>
      ) : null}
      {notCounted.length > 0 ? (
        <details open data-testid={tid("not-counted")}>
          <summary>{notCounted.length} of {summary.scope.pipeRefs.length} pipes not counted</summary>
          <ul>
            {notCounted.slice(0, WEIGHT_SUMMARY_LIST_LIMIT).map((item) => <li key={item.pipeRef}>{item.pipeRef}: {item.reason}</li>)}
            {notCounted.length > WEIGHT_SUMMARY_LIST_LIMIT ? <li>and {notCounted.length - WEIGHT_SUMMARY_LIST_LIMIT} more</li> : null}
          </ul>
        </details>
      ) : (
        <p data-testid={tid("all-counted")}>Every pipe in scope is counted.</p>
      )}
      {notIncluded.length > 0 ? (
        <details open data-testid={tid("components-not-included")}>
          <summary>{count(notIncluded.length, "component weight")} not included</summary>
          <ul>
            {notIncluded.slice(0, WEIGHT_SUMMARY_LIST_LIMIT).map((item, index) => <li key={`${item.componentId}-${index}`}>{item.componentId}: {item.reason}</li>)}
            {notIncluded.length > WEIGHT_SUMMARY_LIST_LIMIT ? <li>and {notIncluded.length - WEIGHT_SUMMARY_LIST_LIMIT} more</li> : null}
          </ul>
        </details>
      ) : null}
    </div>
  );
}
