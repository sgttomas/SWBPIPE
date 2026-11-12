import { useMemo, useState, type KeyboardEvent } from "react";
import { GitCompare } from "lucide-react";
import {
  NOT_DRAWN_LABELS,
  overlayClasses,
  overlayClassWords,
  overlayEntityName,
  type ComparisonOverlayScene,
  type DrawnOverlayEntity,
  type NotDrawnReason,
  type OverlayClassStyle,
  type OverlayPoint
} from "../comparison/comparisonOverlay";
import type { EntityFamily } from "../comparison/stateComparison";

// The comparison overlay's read-only mode of the viewport pane (DEL-07-08 REQ-011, AC-003): while
// a comparison is shown on the model, the pane shows the later compared model with its entities
// marked by class, the entities only in the earlier model as ghosts, a banner in words that names
// the two compared sides, says this is not the working model and offers the way out, and a legend
// in words with the count of each class. The drawing is a projected line drawing (SVG): it opens
// no WebGL context beside the authoring viewport's, which stays mounted beneath, inert and
// untouched. Each class has a glyph and a line style besides its colour (DEL-07-06 REQ-011); the
// colours are the token source's, through the stylesheet. Activating a marked entity, by pointer
// or by Enter or Space, asks for its row in the comparison table. Nothing here edits, selects or
// judges: a difference is shown, not judged (AX-003).

export type OverlayViewPreset = "iso" | "front" | "top";

const VIEW_LABELS: Record<OverlayViewPreset, string> = { iso: "Isometric", front: "Front", top: "Top" };

/** The 2-D projection of a point for a view, in drawing units with y downward. The views are the
 * authoring viewport's camera presets: Front looks along −Z, Top along −Y, Isometric from (+1, +1, +1),
 * with Y up. */
export function projectOverlayPoint(point: OverlayPoint, view: OverlayViewPreset): { x: number; y: number } {
  if (view === "front") return { x: point.x, y: -point.y };
  if (view === "top") return { x: point.x, y: point.z };
  return { x: (point.x - point.z) / Math.SQRT2, y: -(2 * point.y - point.x - point.z) / Math.sqrt(6) };
}

type Projected = { entity: DrawnOverlayEntity; points: Array<{ x: number; y: number }> };

function projectedScene(scene: ComparisonOverlayScene, view: OverlayViewPreset) {
  const projected: Projected[] = scene.drawn.map((entity) => ({
    entity,
    points: entity.geometry.kind === "point"
      ? [projectOverlayPoint(entity.geometry.at, view)]
      : [projectOverlayPoint(entity.geometry.from, view), projectOverlayPoint(entity.geometry.to, view)]
  }));
  // A loop, not argument spreading, so a model of any size fits.
  let minX = Infinity, maxX = -Infinity, minY = Infinity, maxY = -Infinity;
  for (const { points } of projected) {
    for (const point of points) {
      minX = Math.min(minX, point.x); maxX = Math.max(maxX, point.x);
      minY = Math.min(minY, point.y); maxY = Math.max(maxY, point.y);
    }
  }
  if (minX === Infinity) { minX = 0; maxX = 1; minY = 0; maxY = 1; }
  const extent = Math.max(maxX - minX, maxY - minY, 1e-9);
  const margin = extent * 0.08;
  return {
    projected,
    viewBox: `${minX - margin} ${minY - margin} ${maxX - minX + 2 * margin} ${maxY - minY + 2 * margin}`,
    glyphSize: extent * 0.035,
    pointRadius: extent * 0.008
  };
}

function primaryStyle(styles: readonly OverlayClassStyle[], entity: DrawnOverlayEntity): OverlayClassStyle | null {
  return entity.classes.length ? styles.find((style) => style.markClass === entity.classes[0]) ?? null : null;
}

function plural(count: number, one: string, many: string): string {
  return `${count} ${count === 1 ? one : many}`;
}

/** A class's line and glyph as the legend shows them, hidden from the accessibility tree (the
 * words beside it carry the class). */
function LegendSample({ style }: { style: OverlayClassStyle }) {
  return (
    <svg className="comparison-overlay-sample" viewBox="0 0 36 12" width="36" height="12" aria-hidden="true" focusable="false">
      <line className="comparison-overlay-line" data-line={style.line} data-swatch={style.swatch} x1="1" y1="6" x2="24" y2="6" />
      <text className="comparison-overlay-glyph" data-swatch={style.swatch} x="30" y="10" textAnchor="middle" fontSize="11">{style.glyph}</text>
    </svg>
  );
}

export type ComparisonOverlayModeProps = {
  scene: ComparisonOverlayScene;
  /** The earlier (left) and later (right) compared sides, in the table's words. */
  leftText: string;
  rightText: string;
  onLeave: () => void;
  /** A marked entity was activated: show its row in the comparison table. */
  onActivate: (ref: string, family: EntityFamily) => void;
};

export function ComparisonOverlayMode({ scene, leftText, rightText, onLeave, onActivate }: ComparisonOverlayModeProps) {
  const [view, setView] = useState<OverlayViewPreset>("iso");
  const styles = overlayClasses(scene.kind);
  const drawing = useMemo(() => projectedScene(scene, view), [scene, view]);
  const notDrawnTotal = scene.notDrawn.length;
  const reasons = [...new Set(scene.notDrawn.map((entry) => entry.reason))] as NotDrawnReason[];
  const sides = scene.kind === "states" ? "saved state" : "run";
  const activate = (entity: DrawnOverlayEntity) => onActivate(entity.ref, entity.family);
  const onKeyDown = (event: KeyboardEvent<SVGGElement>, entity: DrawnOverlayEntity) => {
    if (event.key === "Enter" || event.key === " ") {
      event.preventDefault();
      activate(entity);
    }
  };
  return (
    <section
      className="comparison-overlay"
      aria-label="Comparison overlay"
      data-testid="comparison-overlay"
      data-kind={scene.kind}
      onKeyDown={(event) => { if (event.key === "Escape") { event.stopPropagation(); onLeave(); } }}
    >
      <div className="comparison-overlay-banner" role="note" data-testid="comparison-overlay-banner">
        <GitCompare size={16} aria-hidden="true" />
        <p>
          <strong>Comparison overlay. This is not the working model.</strong>{" "}
          It draws the later {sides}, {rightText}, compared with the earlier {sides}, {leftText}. Entities only in the
          earlier {sides} are drawn as ghosts in their own positions. Nothing here can be edited; the working model is unchanged.
        </p>
        <label className="comparison-overlay-view">
          <span>View</span>
          <select data-testid="comparison-overlay-view" value={view} onChange={(event) => setView(event.target.value as OverlayViewPreset)}>
            {(Object.keys(VIEW_LABELS) as OverlayViewPreset[]).map((preset) => <option key={preset} value={preset}>{VIEW_LABELS[preset]}</option>)}
          </select>
        </label>
        <button type="button" data-testid="comparison-overlay-leave" onClick={onLeave}>Leave the comparison overlay</button>
      </div>
      <div className="comparison-overlay-body">
        <svg
          className="comparison-overlay-drawing"
          data-testid="comparison-overlay-drawing"
          viewBox={drawing.viewBox}
          preserveAspectRatio="xMidYMid meet"
          role="group"
          aria-label={`Drawing of the later ${sides} with the comparison's marks`}
        >
          {drawing.projected.map(({ entity, points }) => {
            const style = primaryStyle(styles, entity);
            const glyphs = entity.classes.map((markClass) => styles.find((candidate) => candidate.markClass === markClass)!.glyph).join(" ");
            const anchor = points.length === 2 ? { x: points[0].x / 2 + points[1].x / 2, y: points[0].y / 2 + points[1].y / 2 } : points[0];
            const marked = entity.classes.length > 0;
            const name = `${overlayEntityName(entity)}${entity.ghost ? `, only in the earlier ${sides}, drawn as a ghost` : ""}: ${overlayClassWords(scene.kind, entity.classes)}`;
            return (
              <g
                key={`${entity.ghost ? "ghost" : "drawn"}:${entity.ref}`}
                className="comparison-overlay-entity"
                data-testid={`comparison-overlay-entity-${entity.ref}`}
                data-classes={entity.classes.join(" ")}
                data-ghost={entity.ghost ? "true" : "false"}
                data-line={style?.line ?? "context"}
                data-glyph={glyphs}
                role={marked ? "button" : undefined}
                tabIndex={marked ? 0 : undefined}
                aria-label={marked ? name : undefined}
                onClick={marked ? () => activate(entity) : undefined}
                onKeyDown={marked ? (event) => onKeyDown(event, entity) : undefined}
              >
                {points.length === 2 ? (
                  <line className="comparison-overlay-line" data-line={style?.line ?? "context"} data-swatch={style?.swatch ?? "context"}
                    x1={points[0].x} y1={points[0].y} x2={points[1].x} y2={points[1].y} vectorEffect="non-scaling-stroke" />
                ) : (
                  <circle className="comparison-overlay-point" data-line={style?.line ?? "context"} data-swatch={style?.swatch ?? "context"}
                    cx={anchor.x} cy={anchor.y} r={drawing.pointRadius} vectorEffect="non-scaling-stroke" />
                )}
                {marked ? (
                  <text className="comparison-overlay-glyph" data-swatch={style?.swatch} x={anchor.x} y={anchor.y - drawing.glyphSize * 0.4}
                    fontSize={drawing.glyphSize} textAnchor="middle" aria-hidden="true">{glyphs}</text>
                ) : null}
              </g>
            );
          })}
        </svg>
        <section className="comparison-overlay-legend" aria-label="Comparison overlay legend" data-testid="comparison-overlay-legend">
          <strong>Legend</strong>
          <ul>
            {styles.map((style) => {
              const count = scene.counts.find((entry) => entry.markClass === style.markClass)!;
              return (
                <li key={style.markClass} data-testid={`comparison-overlay-legend-${style.markClass}`} data-line={style.line} data-glyph={style.glyph}>
                  <LegendSample style={style} />
                  <span>
                    {style.label} (glyph {style.glyph}, {style.line === "thin" ? "thin" : style.line} line): {plural(count.total, "entity", "entities")}
                    {count.notDrawn ? `, ${count.notDrawn} not drawn` : ""}
                  </span>
                </li>
              );
            })}
            <li data-testid="comparison-overlay-legend-ghost">
              <span>Ghost (faint, its class's line): an entity only in the earlier {sides}, in its own position there.</span>
            </li>
          </ul>
          <p data-testid="comparison-overlay-not-drawn">
            {notDrawnTotal === 0
              ? "Every classified entity is drawn."
              : `${plural(notDrawnTotal, "classified entity is", "classified entities are")} not drawn (${styles
                .map((style) => scene.counts.find((entry) => entry.markClass === style.markClass)!)
                .filter((count) => count.notDrawn > 0)
                .map((count) => `${count.label}: ${count.notDrawn}`)
                .join("; ")}): ${reasons.map((reason) => NOT_DRAWN_LABELS[reason]).join("; ")}. They are listed in the comparison table.`}
          </p>
          <p className="muted">Activate a marked entity to show its row in the comparison table.</p>
        </section>
      </div>
    </section>
  );
}
