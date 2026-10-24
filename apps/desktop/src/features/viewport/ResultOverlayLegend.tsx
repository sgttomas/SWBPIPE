// The canvas legend of the result overlays (design system 2.6 and 5.6): the case selector, the
// result colour scale with its quantity, unit, case, five ticks, minimum and maximum and the count
// of members with no value, and the support reaction arrows' summary. Everything here is a label
// or a number in text tokens; the scale bar and the no-value swatch are the only coloured marks.
// The numerical standing lines are the results surface's own texts.

import { useDisplayQuantity } from "../display-units";
import type { ResultOverlayStanding } from "../results/numericalHealth";
import type { EntityRef } from "../../types";
import {
  resultColourTicks,
  type ResultColourOverlay,
  type ResultColourQuantity,
  type ResultOverlayCase,
  type ResultOverlayDimension
} from "./resultColourOverlay";
import type { ReactionOverlay, ReactionVector } from "./reactionOverlay";

/** Shown where the session has no numerical health presentation for the result. */
export const NUMERICAL_HEALTH_UNAVAILABLE_TEXT = "Numerical health is not available for this result.";
export const RESULT_COLOUR_BOUNDARY_TEXT =
  "Colour shows the magnitude of emitted result rows, from 0 to the largest member value. It marks no limit.";
export const REACTION_BOUNDARY_TEXT =
  "Arrows show emitted support reactions on the pipe in global axes; lengths are scaled to the largest drawn reaction of each kind, not to a physical length.";

/** Four significant figures and a true minus sign (design system 1.1). */
export function formatLegendNumber(value: number | string): string {
  if (typeof value !== "number" || !Number.isFinite(value)) return String(value);
  if (value === 0) return "0";
  const magnitude = Math.abs(value);
  const text = magnitude >= 1e-3 && magnitude < 1e6 ? String(Number(magnitude.toPrecision(4))) : magnitude.toExponential(3);
  return `${value < 0 ? "−" : ""}${text}`;
}

function LegendQuantity({ value, unit, dimension, testId }: { value: number; unit: string; dimension: ResultOverlayDimension; testId?: string }) {
  const display = useDisplayQuantity({ value, unit, dimension_id: dimension });
  return <span className="viewport-legend-value" data-display-status={display.status} data-testid={testId} title={`${value} ${unit}`}>{formatLegendNumber(display.value)}{" "}{display.unit}</span>;
}

function LegendUnit({ value, unit, dimension }: { value: number; unit: string; dimension: ResultOverlayDimension }) {
  return <>{useDisplayQuantity({ value, unit, dimension_id: dimension }).unit}</>;
}

/** The results surface's numerical standing lines for the case: the health header, the case's own
 * headline (or a combination's inherited text) and the output sentence when there is one. */
export function standingLines(standing: ResultOverlayStanding | null | undefined, overlayCase: ResultOverlayCase | null): string[] {
  const health = standing?.health ?? null;
  if (!health) return [NUMERICAL_HEALTH_UNAVAILABLE_TEXT];
  const lines = [health.headline];
  if (overlayCase) {
    const own = overlayCase.basis.ref_type === "load_case"
      ? health.cases.find((item) => item.caseId === overlayCase.basis.ref_id)?.headline
      : overlayCase.basis.ref_type === "combination" ? health.combinationState(overlayCase.basis.ref_id)?.text : undefined;
    if (own && own !== health.headline) lines.push(`${overlayCase.basis.ref_id}: ${own}`);
  }
  if (health.outputSentence) lines.push(health.outputSentence);
  return lines;
}

export type OverlayFocus = Readonly<{ ref: EntityRef; source: "hover" | "selection" }> | null;

function ReactionReadout({ force, moment, supportId }: { force?: ReactionVector; moment?: ReactionVector; supportId: string }) {
  const axes = (vector: ReactionVector, names: readonly [string, string, string], dimension: ResultOverlayDimension) => (
    <>
      {(["x", "y", "z"] as const).map((axis, index) => (
        <span key={axis}>{names[index]} <LegendQuantity value={vector.components[axis]} unit={vector.unit} dimension={dimension} /> </span>
      ))}
      <span>|{names[0][0]}| <LegendQuantity value={vector.emittedMagnitude ?? vector.length} unit={vector.unit} dimension={dimension} /></span>
    </>
  );
  return (
    <p className="viewport-legend-focus" data-testid="viewport-reaction-focus">
      <strong>{supportId}</strong>{" "}
      {force ? <span data-testid="viewport-reaction-focus-force">{axes(force, ["Fx", "Fy", "Fz"], "force")}</span> : null}
      {force && moment ? "; " : null}
      {moment ? <span data-testid="viewport-reaction-focus-moment">{axes(moment, ["Mx", "My", "Mz"], "moment")}</span> : null}
      {!force && !moment ? "no drawn reaction for this case" : null}
    </p>
  );
}

export function ResultOverlayLegend({
  cases,
  overlayCase,
  onCaseChange,
  quantities,
  quantity,
  onQuantityChange,
  colour,
  reactions,
  standing,
  focus
}: {
  cases: readonly ResultOverlayCase[];
  overlayCase: ResultOverlayCase | null;
  onCaseChange: (key: string) => void;
  quantities: readonly ResultColourQuantity[];
  quantity: ResultColourQuantity | null;
  onQuantityChange: (id: string) => void;
  colour: ResultColourOverlay | null;
  reactions: ReactionOverlay | null;
  standing: ResultOverlayStanding | null | undefined;
  focus: OverlayFocus;
}) {
  const lines = standingLines(standing, overlayCase);
  const focusMember = colour?.state === "available" && focus?.ref.type === "pipe" ? colour.members.get(focus.ref.id) : undefined;
  const focusSupport = reactions && focus?.ref.type === "support" ? focus.ref.id : null;
  return (
    <section className="viewport-result-legend" aria-label="Result overlay legend" data-testid="viewport-result-legend">
      <label className="viewport-legend-field">
        <span>Case</span>
        <select aria-label="Result overlay case" data-testid="viewport-overlay-case" disabled={cases.length === 0} value={overlayCase?.key ?? ""} onChange={(event) => onCaseChange(event.target.value)}>
          {cases.length === 0 ? <option value="">No case</option> : null}
          {cases.map((item) => <option key={item.key} value={item.key}>{item.label}</option>)}
        </select>
      </label>
      {colour ? (
        <div className="viewport-legend-block" data-testid="viewport-result-colour-legend" data-state={colour.state}>
          <label className="viewport-legend-field">
            <span>Quantity</span>
            <select aria-label="Result colour quantity" data-testid="viewport-overlay-quantity" disabled={quantities.length === 0} value={quantity?.id ?? ""} onChange={(event) => onQuantityChange(event.target.value)}>
              {quantities.length === 0 ? <option value="">No quantity</option> : null}
              {quantities.map((item) => <option key={item.id} value={item.id}>{item.label}</option>)}
            </select>
          </label>
          {colour.state === "available" && colour.quantity && colour.unit !== null && colour.maximum !== null && colour.minimum !== null ? (
            <>
              <p className="viewport-legend-title" data-testid="viewport-result-colour-title">
                {colour.quantity.label} [<LegendUnit value={colour.maximum} unit={colour.unit} dimension={colour.quantity.dimension} />] · {colour.case?.label}
              </p>
              <div className="viewport-result-scale" data-testid="viewport-result-colour-scale">
                <div className="viewport-result-scale-bar" aria-hidden="true">
                  {focusMember ? <span className="viewport-result-scale-marker" data-testid="viewport-result-colour-marker" style={{ bottom: `${focusMember.position * 100}%` }} /> : null}
                </div>
                <ol className="viewport-result-scale-ticks" aria-label="Scale ticks">
                  {resultColourTicks(colour).map((tick, index) => (
                    <li key={index} data-testid={`viewport-result-colour-tick-${index}`}>
                      <LegendQuantity value={tick} unit={colour.unit!} dimension={colour.quantity!.dimension} />
                    </li>
                  )).reverse()}
                </ol>
              </div>
              <p data-testid="viewport-result-colour-range">
                Members: min <LegendQuantity value={colour.minimum} unit={colour.unit} dimension={colour.quantity.dimension} testId="viewport-result-colour-min" />
                {" · "}max <LegendQuantity value={colour.maximum} unit={colour.unit} dimension={colour.quantity.dimension} testId="viewport-result-colour-max" />
              </p>
              {focusMember && focus ? (
                <p className="viewport-legend-focus" data-testid="viewport-result-colour-focus">
                  <strong>{focus.ref.id}</strong> <LegendQuantity value={focusMember.value} unit={colour.unit} dimension={colour.quantity.dimension} /> <small>({focusMember.rowId})</small>
                </p>
              ) : null}
            </>
          ) : (
            <p className="viewport-legend-title" data-testid="viewport-result-colour-reason">{colour.reason}</p>
          )}
          <p className="viewport-legend-no-value" data-testid="viewport-result-colour-no-value">
            <span className="viewport-result-no-value-swatch" aria-hidden="true" />
            No value: {colour.noValueCount} of {colour.memberCount} member{colour.memberCount === 1 ? "" : "s"}
            {colour.heldRowCount > 0 ? <> ({colour.heldRowCount} row{colour.heldRowCount === 1 ? "" : "s"} withheld by the numerical standing)</> : null}
          </p>
          <small>{RESULT_COLOUR_BOUNDARY_TEXT}</small>
        </div>
      ) : null}
      {reactions ? (
        <div className="viewport-legend-block" data-testid="viewport-reaction-legend" data-state={reactions.state}>
          <p className="viewport-legend-title">Support reactions · {reactions.case?.label ?? "no case"}</p>
          {reactions.state === "available" ? (
            <p data-testid="viewport-reaction-summary">
              Force arrows {[...reactions.forces.values()].filter((item) => item.displayLength > 0).length}
              {reactions.maxForce ? <>; largest |F| <LegendQuantity value={reactions.maxForce.value} unit={reactions.maxForce.unit} dimension="force" testId="viewport-reaction-max-force" /></> : null}
              {" · "}Moment glyphs {[...reactions.moments.values()].filter((item) => item.displayLength > 0).length}
              {reactions.maxMoment ? <>; largest |M| <LegendQuantity value={reactions.maxMoment.value} unit={reactions.maxMoment.unit} dimension="moment" testId="viewport-reaction-max-moment" /></> : null}
            </p>
          ) : null}
          {reactions.reason ? <p data-testid="viewport-reaction-reason">{reactions.reason}</p> : null}
          {reactions.withheld.length > 0 ? (
            <p data-testid="viewport-reaction-withheld">
              Ambiguous attribution is withheld: {reactions.withheld.length} support{reactions.withheld.length === 1 ? "" : "s"} with no arrow ({reactions.withheld.map((item) => `${item.supportId}, ${item.reason}`).join("; ")})
            </p>
          ) : null}
          {reactions.withoutRows.length > 0 ? <p data-testid="viewport-reaction-without-rows">No reaction rows for this case: {reactions.withoutRows.length} support{reactions.withoutRows.length === 1 ? "" : "s"}</p> : null}
          {reactions.frameNotDrawn.length > 0 ? <p data-testid="viewport-reaction-frame">Not drawn, rows not in the global frame: {reactions.frameNotDrawn.join(", ")}</p> : null}
          {reactions.heldRowCount > 0 ? <p data-testid="viewport-reaction-held">{reactions.heldRowCount} reaction row{reactions.heldRowCount === 1 ? "" : "s"} withheld by the numerical standing</p> : null}
          {focusSupport ? <ReactionReadout supportId={focusSupport} force={reactions.forces.get(focusSupport)} moment={reactions.moments.get(focusSupport)} /> : <small>Hover or select a support for its values.</small>}
          <small>{REACTION_BOUNDARY_TEXT}</small>
        </div>
      ) : null}
      <div className="viewport-legend-standing" data-testid="viewport-overlay-standing">
        {lines.map((line, index) => <p key={index}>{line}</p>)}
      </div>
    </section>
  );
}
