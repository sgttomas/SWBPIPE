import { useMemo, useRef, useState, type KeyboardEvent, type PointerEvent as ReactPointerEvent, type WheelEvent } from "react";
import { GitCompare } from "lucide-react";
import {
  NOT_DRAWN_LABELS,
  overlayClasses,
  overlayClassWords,
  overlayEntityName,
  type ComparisonOverlayScene,
  type DrawnOverlayEntity,
  type NotDrawnReason,
  type OverlayClass,
  type OverlayClassStyle,
  type OverlayPoint
} from "../comparison/comparisonOverlay";
import type { EntityFamily } from "../comparison/stateComparison";

// The comparison overlay's read-only mode of the viewport pane (DEL-07-08 REQ-011, AC-003): while
// a comparison is shown on the model, the pane shows a schematic of the later compared model in
// fixed views, with its entities marked by class, the entities only in the earlier model as ghosts,
// a banner in words that names the two compared sides, says this is a schematic and not the working
// model and offers the way out, and a legend in words with the count of each class. The schematic
// is a projected line drawing (SVG): it opens no WebGL context beside the authoring viewport's,
// which stays mounted beneath, inert and untouched.
//
// Each class has a glyph and a line style besides its colour (DEL-07-06 REQ-011); the colours are
// the token source's, through the stylesheet. A marked entity (any class but Unchanged and
// Compared with a zero delta) is drawn above every unmarked one, on a halo, so no unmarked member
// covers it; where its projection has no length (a riser seen from above) it is drawn as a point;
// marks within about a glyph of each other in a view have their glyphs and labels stacked. Marked entities are labelled
// with their identifiers and are the keyboard stops; activating one, by pointer or by Enter or
// Space, asks for its row in the comparison table. Nothing here edits, selects or judges: a
// difference is shown, not judged (AX-003).

export type OverlayViewPreset = "iso" | "iso-opposite" | "front" | "top";

const VIEW_LABELS: Record<OverlayViewPreset, string> = {
  iso: "Isometric",
  "iso-opposite": "Isometric from the opposite corner",
  front: "Front",
  top: "Top"
};

/** The model's vertical axis for the views: Y, as the authoring viewport's presets take it, or Z. */
export type OverlayUpAxis = "y" | "z";

/** A point as the views see it: with Z up, the model is turned −90° about X (right-handed), so its Z
 * takes the place of the views' Y. */
export function upright(point: OverlayPoint, up: OverlayUpAxis): OverlayPoint {
  return up === "z" ? { x: point.x, y: point.z, z: -point.y } : point;
}

/** The classes an entity can have and still be context: drawn, counted, not a keyboard stop. */
const CONTEXT_CLASSES: ReadonlySet<OverlayClass> = new Set<OverlayClass>(["unchanged", "zero_delta"]);

/** Whether a drawn entity is marked: it has a class other than Unchanged and Compared with a zero delta. */
export function isMarkedEntity(entity: Pick<DrawnOverlayEntity, "classes">): boolean {
  return entity.classes.some((markClass) => !CONTEXT_CLASSES.has(markClass));
}

/** The 2-D projection of a point for a view, in drawing units with y downward. Front looks along
 * −Z, Top along −Y, Isometric from (+1, +1, +1) and the opposite isometric from (−1, +1, −1),
 * with Y up, as the authoring viewport's camera presets do. */
export function projectOverlayPoint(point: OverlayPoint, view: OverlayViewPreset): { x: number; y: number } {
  if (view === "front") return { x: point.x, y: -point.y };
  if (view === "top") return { x: point.x, y: point.z };
  if (view === "iso-opposite") return { x: (point.z - point.x) / Math.SQRT2, y: -(2 * point.y + point.x + point.z) / Math.sqrt(6) };
  return { x: (point.x - point.z) / Math.SQRT2, y: -(2 * point.y - point.x - point.z) / Math.sqrt(6) };
}

type Projected = {
  entity: DrawnOverlayEntity;
  marked: boolean;
  points: Array<{ x: number; y: number }>;
  anchor: { x: number; y: number };
  /** The segment's projection has no length in this view (a riser seen end on). */
  collapsed: boolean;
};

type Box = { x: number; y: number; width: number; height: number };

function projectedScene(scene: ComparisonOverlayScene, view: OverlayViewPreset, up: OverlayUpAxis) {
  const project = (point: OverlayPoint) => projectOverlayPoint(upright(point, up), view);
  const all: Projected[] = scene.drawn.map((entity) => {
    const points = entity.geometry.kind === "point"
      ? [project(entity.geometry.at)]
      : [project(entity.geometry.from), project(entity.geometry.to)];
    const anchor = points.length === 2 ? { x: points[0].x / 2 + points[1].x / 2, y: points[0].y / 2 + points[1].y / 2 } : points[0];
    return { entity, marked: isMarkedEntity(entity), points, anchor, collapsed: false };
  });
  // A loop, not argument spreading, so a model of any size fits.
  let minX = Infinity, maxX = -Infinity, minY = Infinity, maxY = -Infinity;
  for (const { points } of all) {
    for (const point of points) {
      minX = Math.min(minX, point.x); maxX = Math.max(maxX, point.x);
      minY = Math.min(minY, point.y); maxY = Math.max(maxY, point.y);
    }
  }
  if (minX === Infinity) { minX = 0; maxX = 1; minY = 0; maxY = 1; }
  const extent = Math.max(maxX - minX, maxY - minY, 1e-9);
  const tolerance = extent * 1e-4;
  for (const item of all) {
    if (item.points.length === 2) item.collapsed = Math.hypot(item.points[1].x - item.points[0].x, item.points[1].y - item.points[0].y) <= tolerance;
  }
  // Painting order: context first, then ghosts, then live marks, so no unmarked entity covers a mark.
  const rank = (item: Projected) => (item.marked ? (item.entity.ghost ? 1 : 2) : 0);
  const projected = all.map((item, index) => ({ item, index })).sort((a, b) => rank(a.item) - rank(b.item) || a.index - b.index).map(({ item }) => item);
  const margin = extent * 0.08;
  const fit: Box = { x: minX - margin, y: minY - margin, width: maxX - minX + 2 * margin, height: maxY - minY + 2 * margin };
  return { projected, fit };
}

function primaryStyle(styles: readonly OverlayClassStyle[], entity: DrawnOverlayEntity): OverlayClassStyle | null {
  const markClass = entity.classes.find((candidate) => !CONTEXT_CLASSES.has(candidate)) ?? entity.classes[0];
  return markClass ? styles.find((style) => style.markClass === markClass) ?? null : null;
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

const ZOOM_STEP = 1.5;
const MAX_ZOOM = 200;

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
  const [up, setUp] = useState<OverlayUpAxis>("y");
  const [labels, setLabels] = useState(true);
  // Zoom (1 is the whole schematic) and the centre of the visible part, in drawing units; null centre is the fit's.
  const [zoom, setZoom] = useState(1);
  const [centre, setCentre] = useState<{ x: number; y: number } | null>(null);
  const drag = useRef<{ pointerId: number; x: number; y: number; centre: { x: number; y: number } } | null>(null);
  const svgRef = useRef<SVGSVGElement | null>(null);
  const styles = overlayClasses(scene.kind);
  const drawing = useMemo(() => projectedScene(scene, view, up), [scene, view, up]);
  const fitCentre = { x: drawing.fit.x + drawing.fit.width / 2, y: drawing.fit.y + drawing.fit.height / 2 };
  const shownCentre = centre ?? fitCentre;
  const box: Box = { width: drawing.fit.width / zoom, height: drawing.fit.height / zoom, x: shownCentre.x - drawing.fit.width / zoom / 2, y: shownCentre.y - drawing.fit.height / zoom / 2 };
  // Glyphs, labels and points keep their size on screen at any zoom.
  const scale = Math.max(box.width, box.height);
  const glyphSize = scale * 0.03;
  const labelSize = scale * 0.018;
  const pointRadius = scale * 0.006;
  // Marks whose glyphs would fall in one cell of about a glyph's size on screen are stacked, so
  // their glyphs and labels do not print over each other (exactly coincident marks included).
  const stacks = useMemo(() => {
    const cell = glyphSize * 1.2;
    const used = new Map<string, number>();
    const result = new Map<DrawnOverlayEntity, number>();
    for (const item of drawing.projected) {
      if (!item.marked) continue;
      const key = `${Math.round(item.anchor.x / cell)}:${Math.round(item.anchor.y / cell)}`;
      const index = used.get(key) ?? 0;
      used.set(key, index + 1);
      result.set(item.entity, index);
    }
    return result;
  }, [drawing, glyphSize]);
  const notDrawnTotal = scene.notDrawn.length;
  const reasons = [...new Set(scene.notDrawn.map((entry) => entry.reason))] as NotDrawnReason[];
  const sides = scene.kind === "states" ? "saved state" : "run";
  const markedCount = drawing.projected.filter((item) => item.marked).length;
  const contextCount = drawing.projected.length - markedCount;
  const activate = (entity: DrawnOverlayEntity) => onActivate(entity.ref, entity.family);
  const onKeyDown = (event: KeyboardEvent<SVGGElement>, entity: DrawnOverlayEntity) => {
    if (event.key === "Enter" || event.key === " ") {
      event.preventDefault();
      activate(entity);
    }
  };
  const changeView = (next: OverlayViewPreset) => { setView(next); setZoom(1); setCentre(null); };
  const zoomBy = (factor: number, at?: { x: number; y: number }) => {
    const next = Math.min(MAX_ZOOM, Math.max(1, zoom * factor));
    if (next === 1) { setZoom(1); setCentre(null); return; }
    const focus = at ?? shownCentre;
    // Keep the drawing point under the pointer where it is.
    setCentre({ x: focus.x + (shownCentre.x - focus.x) * (zoom / next), y: focus.y + (shownCentre.y - focus.y) * (zoom / next) });
    setZoom(next);
  };
  const toDrawing = (clientX: number, clientY: number) => {
    const rect = svgRef.current?.getBoundingClientRect();
    if (!rect || rect.width === 0 || rect.height === 0) return shownCentre;
    // preserveAspectRatio meet: the box is scaled uniformly and centred.
    const unit = Math.max(box.width / rect.width, box.height / rect.height);
    return { x: shownCentre.x + (clientX - rect.left - rect.width / 2) * unit, y: shownCentre.y + (clientY - rect.top - rect.height / 2) * unit };
  };
  const onWheel = (event: WheelEvent<SVGSVGElement>) => {
    if (event.deltaY === 0) return;
    zoomBy(event.deltaY < 0 ? 1.2 : 1 / 1.2, toDrawing(event.clientX, event.clientY));
  };
  const onPointerDown = (event: ReactPointerEvent<SVGSVGElement>) => {
    if (event.button !== 0 || zoom === 1) return;
    drag.current = { pointerId: event.pointerId, x: event.clientX, y: event.clientY, centre: shownCentre };
  };
  const onPointerMove = (event: ReactPointerEvent<SVGSVGElement>) => {
    const start = drag.current;
    const rect = svgRef.current?.getBoundingClientRect();
    if (!start || start.pointerId !== event.pointerId || !rect || rect.width === 0) return;
    const unit = Math.max(box.width / rect.width, box.height / rect.height);
    setCentre({ x: start.centre.x - (event.clientX - start.x) * unit, y: start.centre.y - (event.clientY - start.y) * unit });
  };
  const endDrag = () => { drag.current = null; };
  const pan = (dx: number, dy: number) => setCentre({ x: shownCentre.x + dx * box.width * 0.2, y: shownCentre.y + dy * box.height * 0.2 });
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
          It is a schematic of the compared models in fixed views, not the three-dimensional viewport. It draws the later {sides},
          {" "}{rightText}, compared with the earlier {sides}, {leftText}. Entities only in the earlier {sides} are drawn as ghosts in
          their own positions. Nothing here can be edited; the working model is unchanged.
        </p>
        <button type="button" data-testid="comparison-overlay-leave" onClick={onLeave}>Leave the comparison overlay</button>
      </div>
      <div className="comparison-overlay-tools" role="toolbar" aria-label="Schematic view">
        <label className="comparison-overlay-view">
          <span>View</span>
          <select data-testid="comparison-overlay-view" value={view} onChange={(event) => changeView(event.target.value as OverlayViewPreset)}>
            {(Object.keys(VIEW_LABELS) as OverlayViewPreset[]).map((preset) => <option key={preset} value={preset}>{VIEW_LABELS[preset]}</option>)}
          </select>
        </label>
        <label className="comparison-overlay-view">
          <span>Up axis</span>
          <select data-testid="comparison-overlay-up" value={up} onChange={(event) => { setUp(event.target.value as OverlayUpAxis); setZoom(1); setCentre(null); }}>
            <option value="y">Y (as the viewport)</option>
            <option value="z">Z</option>
          </select>
        </label>
        <button type="button" data-testid="comparison-overlay-zoom-in" disabled={zoom >= MAX_ZOOM} onClick={() => zoomBy(ZOOM_STEP)}>Zoom in</button>
        <button type="button" data-testid="comparison-overlay-zoom-out" disabled={zoom <= 1} onClick={() => zoomBy(1 / ZOOM_STEP)}>Zoom out</button>
        <button type="button" data-testid="comparison-overlay-fit" disabled={zoom === 1} onClick={() => { setZoom(1); setCentre(null); }}>Fit</button>
        <span className="comparison-overlay-pan" role="group" aria-label="Pan">
          <button type="button" aria-label="Pan left" disabled={zoom === 1} onClick={() => pan(-1, 0)}>←</button>
          <button type="button" aria-label="Pan up" disabled={zoom === 1} onClick={() => pan(0, -1)}>↑</button>
          <button type="button" aria-label="Pan down" disabled={zoom === 1} onClick={() => pan(0, 1)}>↓</button>
          <button type="button" aria-label="Pan right" disabled={zoom === 1} onClick={() => pan(1, 0)}>→</button>
        </span>
        <label className="comparison-overlay-labels">
          <input type="checkbox" data-testid="comparison-overlay-labels" checked={labels} onChange={(event) => setLabels(event.target.checked)} />
          <span>Label marked entities</span>
        </label>
        <small className="muted" data-testid="comparison-overlay-zoom">{zoom === 1 ? "Whole schematic" : `Zoom ×${zoom.toFixed(1)}`}</small>
      </div>
      <div className="comparison-overlay-body">
        <svg
          ref={svgRef}
          className="comparison-overlay-drawing"
          data-testid="comparison-overlay-drawing"
          data-view={view}
          viewBox={`${box.x} ${box.y} ${box.width} ${box.height}`}
          preserveAspectRatio="xMidYMid meet"
          role="group"
          aria-label={`Schematic of the later ${sides} with the comparison's marks, ${VIEW_LABELS[view]} view, ${up.toUpperCase()} up`}
          onWheel={onWheel}
          onPointerDown={onPointerDown}
          onPointerMove={onPointerMove}
          onPointerUp={endDrag}
          onPointerCancel={endDrag}
        >
          {drawing.projected.map(({ entity, marked, points, anchor, collapsed }) => {
            const stack = stacks.get(entity) ?? 0;
            const style = primaryStyle(styles, entity);
            const glyphs = entity.classes.map((markClass) => styles.find((candidate) => candidate.markClass === markClass)!.glyph).join(" ");
            const name = `${overlayEntityName(entity)}${entity.ghost ? `, only in the earlier ${sides}, drawn as a ghost` : ""}: ${overlayClassWords(scene.kind, entity.classes)}`;
            const line = style?.line ?? "context";
            const swatch = style?.swatch ?? "context";
            const asPoint = points.length === 1 || collapsed;
            const glyphY = anchor.y - glyphSize * 0.4 + stack * glyphSize * 1.15;
            return (
              <g
                key={`${entity.ghost ? "ghost" : "drawn"}:${entity.ref}`}
                className="comparison-overlay-entity"
                data-testid={`comparison-overlay-entity-${entity.ref}`}
                data-classes={entity.classes.join(" ")}
                data-marked={marked ? "true" : "false"}
                data-ghost={entity.ghost ? "true" : "false"}
                data-line={line}
                data-glyph={glyphs}
                data-collapsed={collapsed ? "true" : undefined}
                role={marked ? "button" : undefined}
                tabIndex={marked ? 0 : undefined}
                aria-label={marked ? name : undefined}
                onClick={marked ? () => activate(entity) : undefined}
                onKeyDown={marked ? (event) => onKeyDown(event, entity) : undefined}
              >
                <title>{name}</title>
                {marked ? (asPoint
                  ? <circle className="comparison-overlay-halo" cx={anchor.x} cy={anchor.y} r={pointRadius * 1.8} vectorEffect="non-scaling-stroke" />
                  : <line className="comparison-overlay-halo" x1={points[0].x} y1={points[0].y} x2={points[1].x} y2={points[1].y} vectorEffect="non-scaling-stroke" />) : null}
                {asPoint ? (
                  <circle className="comparison-overlay-point" data-line={line} data-swatch={swatch} data-ghost={entity.ghost ? "true" : undefined}
                    cx={anchor.x} cy={anchor.y} r={pointRadius * (collapsed ? 1.4 : 1)} vectorEffect="non-scaling-stroke" />
                ) : (
                  <line className="comparison-overlay-line" data-line={line} data-swatch={swatch}
                    x1={points[0].x} y1={points[0].y} x2={points[1].x} y2={points[1].y} vectorEffect="non-scaling-stroke" />
                )}
                {marked ? (
                  <text className="comparison-overlay-glyph" data-swatch={style?.swatch} x={anchor.x} y={glyphY}
                    fontSize={glyphSize} strokeWidth={glyphSize * 0.18} textAnchor="middle" aria-hidden="true">{glyphs}</text>
                ) : null}
                {marked && labels ? (
                  <text className="comparison-overlay-label" x={anchor.x + glyphSize * 0.9} y={glyphY} fontSize={labelSize} strokeWidth={labelSize * 0.25} aria-hidden="true">
                    {entity.ref}{entity.ghost ? ` (earlier ${sides} only)` : ""}
                  </text>
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
              const context = CONTEXT_CLASSES.has(style.markClass);
              return (
                <li key={style.markClass} data-testid={`comparison-overlay-legend-${style.markClass}`} data-line={style.line} data-glyph={style.glyph}>
                  <LegendSample style={style} />
                  <span>
                    {style.label} (glyph {style.glyph}, {style.line} line): {plural(count.total, "entity", "entities")}
                    {count.notDrawn ? `, ${count.notDrawn} not drawn` : ""}
                    {context ? "; drawn as context, not marked" : ""}
                  </span>
                </li>
              );
            })}
            <li data-testid="comparison-overlay-legend-ghost">
              <span>Ghost (faint, labelled “earlier {sides} only”): an entity only in the earlier {sides}, in its own position there.</span>
            </li>
          </ul>
          <p data-testid="comparison-overlay-drawn">
            {plural(markedCount, "marked entity is", "marked entities are")} drawn above {plural(contextCount, "context entity", "context entities")}.
            A member seen end on is drawn as a point.
          </p>
          <p data-testid="comparison-overlay-not-drawn">
            {notDrawnTotal === 0
              ? "Every classified entity is drawn."
              : `${plural(notDrawnTotal, "classified entity is", "classified entities are")} not drawn (${styles
                .map((style) => scene.counts.find((entry) => entry.markClass === style.markClass)!)
                .filter((count) => count.notDrawn > 0)
                .map((count) => `${count.label}: ${count.notDrawn}`)
                .join("; ")}): ${reasons.map((reason) => NOT_DRAWN_LABELS[reason]).join("; ")}. They are listed in the comparison table.`}
          </p>
          <p className="muted">Activate a marked entity to show its row in the comparison table. Scroll or use Zoom to enlarge; drag or use the arrows to pan.</p>
        </section>
      </div>
    </section>
  );
}
