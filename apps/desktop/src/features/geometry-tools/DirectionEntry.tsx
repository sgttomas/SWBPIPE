// B5 (DEL-07-01 Design `split-and-slope-entry.md` §3): the end-point entry aid. It computes the
// end point from the start node and a length with a direction, an angle from the horizontal, or a
// "1 in n" slope, shows the exact coordinates, and on "Use these coordinates" hands exactly those
// numbers to the form that emits the existing operation. It writes nothing itself.
import { useEffect, useId, useState } from "react";
import {
  AXES,
  DIRECTION_MODE_LABELS,
  UP_DIRECTIONS,
  computeDirectionEnd,
  directionDraftIssue,
  emptyDirectionDraft,
  engineConvert,
  exactCoordinateText,
  type Convert,
  type DirectionDraft,
  type DirectionMode,
  type DirectionResult,
  type UpDirection,
  type Vec3
} from "./directionArithmetic";

type Props = {
  /** Names the surface in every accessible label, e.g. "Route end" or "Reconnect point". */
  context: string;
  projectLengthUnit: string | undefined;
  /** The start node's stored position, or null with the reason it is not available. */
  start: { id: string; position: Vec3 } | null;
  startReason?: string;
  disabled?: boolean;
  /** Receives the computed coordinates (project length unit), exactly as displayed. */
  onUse: (position: Vec3, unit: string) => void;
  convert?: Convert;
};

const UP_LABELS: Record<Exclude<UpDirection, "">, string> = { "+x": "+X is up", "-x": "−X is up", "+y": "+Y is up", "-y": "−Y is up", "+z": "+Z is up", "-z": "−Z is up" };

export function DirectionEntry({ context, projectLengthUnit, start, startReason, disabled = false, onUse, convert = engineConvert }: Props) {
  const reasonId = useId();
  const [draft, setDraft] = useState<DirectionDraft>(() => emptyDirectionDraft(projectLengthUnit ?? ""));
  const [computed, setComputed] = useState<{ result: DirectionResult; draft: DirectionDraft; start: Vec3 } | null>(null);
  const [failure, setFailure] = useState("");
  const set = <K extends keyof DirectionDraft>(key: K, value: DirectionDraft[K]) => setDraft((previous) => ({ ...previous, [key]: value }));
  const issue = !start ? (startReason ?? "Choose the start node first.") : directionDraftIssue(draft, projectLengthUnit);

  useEffect(() => {
    setComputed(null);
    setFailure("");
    if (issue || !start) return;
    let live = true;
    computeDirectionEnd(draft, start.position, projectLengthUnit, convert)
      .then((result) => { if (live) setComputed({ result, draft, start: { ...start.position } }); })
      .catch((error: unknown) => { if (live) setFailure(error instanceof Error ? error.message : String(error)); });
    return () => { live = false; };
  }, [draft, start?.position.x, start?.position.y, start?.position.z, start?.id, projectLengthUnit, issue, convert]);

  const sameStart = (a: Vec3, b: Vec3) => AXES.every((axis) => Object.is(a[axis], b[axis]));
  const current = computed && computed.draft === draft && start && sameStart(computed.start, start.position) ? computed.result : null;
  const reason = issue || failure || (current ? "" : "Computing…");
  const label = (text: string) => `${context}: ${text}`;
  const unit = projectLengthUnit?.trim() ?? "";
  const horizontal = draft.mode === "angle" || draft.mode === "slope";

  return <fieldset className="direction-entry" disabled={disabled} aria-describedby={reason ? reasonId : undefined}>
    <legend>{context}: end point from a length and direction</legend>
    <p>Entry aid only. The direction, angle or slope is not saved: the model stores the node coordinates computed below{start ? `, from ${start.id}` : ""}.</p>
    <label>{label("Entry method")}
      <select aria-label={label("Entry method")} value={draft.mode} onChange={(event) => set("mode", event.target.value as DirectionMode)}>
        <option value="">Choose a method</option>
        {(Object.keys(DIRECTION_MODE_LABELS) as Array<Exclude<DirectionMode, "">>).map((mode) => <option key={mode} value={mode}>{DIRECTION_MODE_LABELS[mode]}</option>)}
      </select>
    </label>
    {draft.mode ? <>
      <label>{label(draft.mode === "slope" ? "Horizontal run" : "Length along the element")}
        <input aria-label={label(draft.mode === "slope" ? "Horizontal run" : "Length along the element")} inputMode="decimal" value={draft.length} onChange={(event) => set("length", event.target.value)} />
      </label>
      <label>{label("Length unit")}
        <input aria-label={label("Length unit")} value={draft.lengthUnit} onChange={(event) => set("lengthUnit", event.target.value)} />
      </label>
      {horizontal ? <label>{label("Up direction")}
        <select aria-label={label("Up direction")} value={draft.up} onChange={(event) => set("up", event.target.value as UpDirection)}>
          <option value="">State which direction is up</option>
          {UP_DIRECTIONS.map((up) => <option key={up} value={up}>{UP_LABELS[up]}</option>)}
        </select>
      </label> : null}
      <fieldset>
        <legend>{horizontal ? "Horizontal direction (its up component must be 0)" : "Direction vector (any length; it is normalized)"}</legend>
        {AXES.map((axis) => <label key={axis}>{label(`${horizontal ? "Horizontal direction" : "Direction"} ${axis.toUpperCase()}`)}
          <input aria-label={label(`${horizontal ? "Horizontal direction" : "Direction"} ${axis.toUpperCase()}`)} inputMode="decimal" value={draft.direction[axis]}
            onChange={(event) => set("direction", { ...draft.direction, [axis]: event.target.value })} />
        </label>)}
      </fieldset>
      {draft.mode === "angle" ? <>
        <label>{label("Angle from the horizontal (positive rises)")}
          <input aria-label={label("Angle from the horizontal (positive rises)")} inputMode="decimal" value={draft.angle} onChange={(event) => set("angle", event.target.value)} />
        </label>
        <label>{label("Angle unit")}
          <input aria-label={label("Angle unit")} value={draft.angleUnit} onChange={(event) => set("angleUnit", event.target.value)} />
        </label>
      </> : null}
      {draft.mode === "slope" ? <>
        <label>{label("Slope 1 in n: rise or fall over the horizontal run, n")}
          <input aria-label={label("Slope 1 in n: rise or fall over the horizontal run, n")} inputMode="decimal" value={draft.slopeN} onChange={(event) => set("slopeN", event.target.value)} />
        </label>
        <label>{label("Along the horizontal direction the line")}
          <select aria-label={label("Along the horizontal direction the line")} value={draft.slopeSense} onChange={(event) => set("slopeSense", event.target.value as DirectionDraft["slopeSense"])}>
            <option value="">Choose falls or rises</option>
            <option value="falls">falls</option>
            <option value="rises">rises</option>
          </select>
        </label>
      </> : null}
    </> : null}
    <p role="status" aria-label={label("Computed end point")} data-testid={`direction-entry-result-${context.toLowerCase().replaceAll(" ", "-")}`}>
      {current
        ? `Computed end point (${unit}): X ${exactCoordinateText(current.position.x)}, Y ${exactCoordinateText(current.position.y)}, Z ${exactCoordinateText(current.position.z)}`
        : "No end point computed."}
    </p>
    {reason ? <p id={reasonId}>{reason}</p> : null}
    <button type="button" disabled={!current} aria-describedby={reason ? reasonId : undefined}
      onClick={() => { if (current) onUse(current.position, unit); }}>
      {`Use these coordinates for the ${context.toLowerCase()}`}
    </button>
  </fieldset>;
}
