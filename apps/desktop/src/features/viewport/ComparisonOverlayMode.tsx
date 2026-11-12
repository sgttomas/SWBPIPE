import { useEffect, useMemo, useRef, useState, type KeyboardEvent, type PointerEvent as ReactPointerEvent, type WheelEvent } from "react";
import {
  comparisonOverlayScene,
  overlayClasses,
  overlayClassWords,
  overlayEntityName,
  type ComparisonOverlayScene,
  type DrawnOverlayEntity,
  type OverlayClassStyle,
  type OverlayPoint
} from "../comparison/comparisonOverlay";
import { ComparisonOverlayLegend, CONTEXT_CLASSES, type PlacedOverlayMark } from "../comparison/ComparisonOverlayLegend";
import type { EntityFamily } from "../comparison/stateComparison";
import type { ShownComparisonOverlay } from "../history/useModelStateHistory";
import { deriveModelUp, modelUpReadout, signedAxisText, type SignedAxis } from "./modelUp";

// The comparison overlay's schematic (DEL-07-08 REQ-011, AC-003): a read-only mode of the viewport
// pane that draws the later compared model as a projected line drawing (SVG) in fixed views, with its
// entities marked by class and the entities only in the earlier model as ghosts. It is a schematic,
// not the three-dimensional viewport, and opens no WebGL context beside the authoring viewport's,
// which stays mounted beneath, inert and untouched. It reads the session's overlay contract (kind,
// identifiers, the two sides' texts, the marks and the two models) and builds what it draws from it;
// the banner, legend and list of marks are the shared `ComparisonOverlayLegend`, given this
// schematic's own counts, what it did not draw and its own `locate`.
//
// Each class has a glyph and a line style besides its colour (DEL-07-06 REQ-011); the colours are
// the token source's, through the stylesheet. A marked entity (any class but Unchanged and
// Compared with a zero delta) is drawn above every unmarked one, on a halo, so no unmarked member
// covers it; where its projection has no length it is drawn as a point; marks within about a glyph
// of each other have their glyphs and labels stacked; labels are the names the model tree shows and
// stay inside the drawing. Marked entities are the keyboard stops; activating one, by pointer or by
// Enter or Space, asks for its row in the comparison table. The up direction defaults to the model's
// by the desktop's rule (`modelUp.ts`); the person's choice overrides it. Nothing here edits,
// selects or judges: a difference is shown, not judged (AX-003).

export type OverlayViewPreset = "iso" | "iso-opposite" | "front" | "top";

const VIEW_LABELS: Record<OverlayViewPreset, string> = {
  iso: "Isometric",
  "iso-opposite": "Isometric, opposite corner",
  front: "Front",
  top: "Top"
};

const UP_CHOICES: readonly SignedAxis[] = ["+x", "-x", "+y", "-y", "+z", "-z"];

/** A point turned so that the signed axis `up` is drawn upward (the views take +Y as up). Each turn
 * is a right-handed rotation: +Z up turns −90° about X, −Z up +90° about X, −Y up 180° about X, +X
 * up +90° about Z and −X up −90° about Z. */
export function upright(point: OverlayPoint, up: SignedAxis): OverlayPoint {
  const { x, y, z } = point;
  switch (up) {
    case "+z": return { x, y: z, z: -y };
    case "-z": return { x, y: -z, z: y };
    case "-y": return { x, y: -y, z: -z };
    case "+x": return { x: -y, y: x, z };
    case "-x": return { x: y, y: -x, z };
    default: return point;
  }
}

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

/** The most characters of a name a label shows; the title and the list hold the whole name. */
export const LABEL_MAX_CHARS = 28;
/** The fewest characters a label is cut to, however little room there is. */
const LABEL_MIN_CHARS = 6;
/** A monospace character's advance, as a share of the font size (the label's font is monospace). */
const MONO_ADVANCE = 0.62;

/** Where a label goes and what it shows, so that it stays inside the visible box: on the side of
 * its glyph with more room, cut with an ellipsis to the characters that fit there (at most
 * `LABEL_MAX_CHARS`, at least `LABEL_MIN_CHARS`). Distances are in drawing units. */
export function labelPlacement(
  text: string,
  anchorX: number,
  gap: number,
  box: { x: number; width: number },
  fontSize: number
): { side: "start" | "end"; text: string } {
  const right = box.x + box.width - anchorX - gap;
  const left = anchorX - box.x - gap;
  const side = right >= left ? "start" : "end";
  const room = Math.max(right, left);
  const fits = Math.floor(room / (fontSize * MONO_ADVANCE));
  const limit = Math.max(LABEL_MIN_CHARS, Math.min(LABEL_MAX_CHARS, fits));
  const characters = [...text];
  return { side, text: characters.length > limit ? `${characters.slice(0, limit - 1).join("")}…` : text };
}

type Projected = {
  entity: DrawnOverlayEntity;
  marked: boolean;
  points: Array<{ x: number; y: number }>;
  anchor: { x: number; y: number };
  /** The segment's projection has no length in this view (a member seen end on). */
  collapsed: boolean;
};

type Box = { x: number; y: number; width: number; height: number };

function projectedScene(scene: ComparisonOverlayScene, view: OverlayViewPreset, up: SignedAxis) {
  const project = (point: OverlayPoint) => projectOverlayPoint(upright(point, up), view);
  const all: Projected[] = scene.drawn.map((entity) => {
    const points = entity.geometry.kind === "point" ? [project(entity.geometry.at)] : [project(entity.geometry.from), project(entity.geometry.to)];
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

const ZOOM_STEP = 1.5;
const MAX_ZOOM = 200;
/** The zoom a located mark is shown at, at least. */
const LOCATE_ZOOM = 4;
/** On-screen sizes, in CSS pixels, of the glyphs, the labels and the points. */
const GLYPH_PX = 15;
const LABEL_PX = 11;
const POINT_PX = 3.5;

export type ComparisonOverlayModeProps = {
  overlay: ShownComparisonOverlay;
  onLeave: () => void;
  /** A marked entity was activated: show its row in the comparison table. */
  onActivate: (ref: string, family: EntityFamily) => void;
};

export function ComparisonOverlayMode({ overlay, onLeave, onActivate }: ComparisonOverlayModeProps) {
  const [view, setView] = useState<OverlayViewPreset>("iso");
  const derivedUp = useMemo(() => deriveModelUp(overlay.models.right), [overlay.models.right]);
  const [chosenUp, setChosenUp] = useState<SignedAxis | null>(null);
  const up = chosenUp ?? derivedUp.up;
  const [labels, setLabels] = useState(true);
  // Zoom (1 is the whole schematic) and the centre of the visible part, in drawing units; null centre is the fit's.
  const [zoom, setZoom] = useState(1);
  const [centre, setCentre] = useState<{ x: number; y: number } | null>(null);
  const [located, setLocated] = useState<string | null>(null);
  const [pixels, setPixels] = useState<{ width: number; height: number } | null>(null);
  const drag = useRef<{ pointerId: number; x: number; y: number; centre: { x: number; y: number } } | null>(null);
  const svgRef = useRef<SVGSVGElement | null>(null);
  const styles = overlayClasses(overlay.kind);
  const scene = useMemo(
    () => comparisonOverlayScene(overlay.kind, overlay.marks, overlay.models.left, overlay.models.right),
    [overlay.kind, overlay.marks, overlay.models.left, overlay.models.right]
  );
  const drawing = useMemo(() => projectedScene(scene, view, up), [scene, view, up]);
  // The drawing's size on screen, so glyphs, labels and points keep one size at any width and zoom.
  useEffect(() => {
    const svg = svgRef.current;
    if (!svg || typeof ResizeObserver === "undefined") return;
    const measure = () => {
      const rect = svg.getBoundingClientRect();
      setPixels(rect.width > 0 && rect.height > 0 ? { width: rect.width, height: rect.height } : null);
    };
    measure();
    const observer = new ResizeObserver(measure);
    observer.observe(svg);
    return () => observer.disconnect();
  }, []);
  const fitCentre = { x: drawing.fit.x + drawing.fit.width / 2, y: drawing.fit.y + drawing.fit.height / 2 };
  const shownCentre = centre ?? fitCentre;
  const box: Box = { width: drawing.fit.width / zoom, height: drawing.fit.height / zoom, x: shownCentre.x - drawing.fit.width / zoom / 2, y: shownCentre.y - drawing.fit.height / zoom / 2 };
  // Drawing units per CSS pixel (preserveAspectRatio meet scales the box uniformly); without a
  // measured size, a share of the box.
  const unit = pixels ? Math.max(box.width / pixels.width, box.height / pixels.height) : Math.max(box.width, box.height) / 900;
  // The box actually visible: meet centres the box and shows more of the drawing along one axis.
  const visible: Box = pixels
    ? { width: pixels.width * unit, height: pixels.height * unit, x: shownCentre.x - (pixels.width * unit) / 2, y: shownCentre.y - (pixels.height * unit) / 2 }
    : box;
  const glyphSize = GLYPH_PX * unit;
  const labelSize = LABEL_PX * unit;
  const pointRadius = POINT_PX * unit;
  // Marks whose glyphs fall in one cell of about a glyph's size are stacked, so their glyphs and
  // labels do not print over each other (exactly coincident marks included).
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
  const sides = overlay.kind === "states" ? "saved state" : "run";
  const placed: PlacedOverlayMark[] = useMemo(() => {
    const order = new Map(styles.map((style, index) => [style.markClass, index] as const));
    const first = (entity: DrawnOverlayEntity) => order.get(entity.classes.find((markClass) => !CONTEXT_CLASSES.has(markClass))!) ?? 0;
    return scene.drawn
      .filter(isMarkedEntity)
      .slice()
      .sort((a, b) => first(a) - first(b) || a.label.localeCompare(b.label))
      .map((entity) => ({ ref: entity.ref, family: entity.family, name: entity.label, classes: entity.classes, ghost: entity.ghost }));
  }, [scene, styles]);
  const activate = (entity: DrawnOverlayEntity) => onActivate(entity.ref, entity.family);
  const onKeyDown = (event: KeyboardEvent<SVGGElement>, entity: DrawnOverlayEntity) => {
    if (event.key === "Enter" || event.key === " ") {
      event.preventDefault();
      activate(entity);
    }
  };
  const resetView = () => { setZoom(1); setCentre(null); };
  const zoomBy = (factor: number, at?: { x: number; y: number }) => {
    const next = Math.min(MAX_ZOOM, Math.max(1, zoom * factor));
    if (next === 1) { resetView(); return; }
    const focus = at ?? shownCentre;
    // Keep the drawing point under the pointer where it is.
    setCentre({ x: focus.x + (shownCentre.x - focus.x) * (zoom / next), y: focus.y + (shownCentre.y - focus.y) * (zoom / next) });
    setZoom(next);
  };
  /** The schematic's own locate: centre the mark and zoom in on it; the selection is untouched. */
  const locate = (ref: string, family: EntityFamily) => {
    const item = drawing.projected.find((candidate) => candidate.entity.ref === ref && candidate.entity.family === family && candidate.marked);
    if (!item) return;
    setZoom(Math.max(zoom, LOCATE_ZOOM));
    setCentre(item.anchor);
    setLocated(ref);
  };
  const toDrawing = (clientX: number, clientY: number) => {
    const rect = svgRef.current?.getBoundingClientRect();
    if (!rect || rect.width === 0 || rect.height === 0) return shownCentre;
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
    if (!start || start.pointerId !== event.pointerId) return;
    setCentre({ x: start.centre.x - (event.clientX - start.x) * unit, y: start.centre.y - (event.clientY - start.y) * unit });
  };
  const endDrag = () => { drag.current = null; };
  const pan = (dx: number, dy: number) => setCentre({ x: shownCentre.x + dx * box.width * 0.2, y: shownCentre.y + dy * box.height * 0.2 });
  const upReadout = chosenUp ? `Up ${signedAxisText(chosenUp)}, chosen. ${modelUpReadout(derivedUp)}` : modelUpReadout(derivedUp);
  return (
    <section
      className="comparison-overlay"
      aria-label="Comparison overlay"
      data-testid="comparison-overlay"
      data-kind={overlay.kind}
      onKeyDown={(event) => { if (event.key === "Escape") { event.stopPropagation(); onLeave(); } }}
    >
      <ComparisonOverlayLegend
        kind={overlay.kind}
        leftText={overlay.leftText}
        rightText={overlay.rightText}
        presentation="a schematic in fixed views, not the three-dimensional viewport"
        counts={scene.counts}
        notPlaced={scene.notDrawn}
        placed={placed}
        located={located}
        locate={locate}
        onLeave={onLeave}
      />
      <div className="viewport-toolbar-controls comparison-overlay-tools" role="toolbar" aria-label="Schematic view">
        <label className="viewport-legend-field">
          <span>View</span>
          <select data-testid="comparison-overlay-view" value={view} onChange={(event) => { setView(event.target.value as OverlayViewPreset); resetView(); }}>
            {(Object.keys(VIEW_LABELS) as OverlayViewPreset[]).map((preset) => <option key={preset} value={preset}>{VIEW_LABELS[preset]}</option>)}
          </select>
        </label>
        <label className="viewport-legend-field">
          <span>Up</span>
          <select data-testid="comparison-overlay-up" value={chosenUp ?? "model"} onChange={(event) => { setChosenUp(event.target.value === "model" ? null : event.target.value as SignedAxis); resetView(); }}>
            <option value="model">From the model ({signedAxisText(derivedUp.up)})</option>
            {UP_CHOICES.map((choice) => <option key={choice} value={choice}>{signedAxisText(choice)}</option>)}
          </select>
        </label>
        <small className="muted" data-testid="comparison-overlay-up-readout">{upReadout}</small>
        <div className="viewport-display-toggles" role="group" aria-label="Zoom and pan">
          <button type="button" data-testid="comparison-overlay-zoom-in" disabled={zoom >= MAX_ZOOM} onClick={() => zoomBy(ZOOM_STEP)}>Zoom in</button>
          <button type="button" data-testid="comparison-overlay-zoom-out" disabled={zoom <= 1} onClick={() => zoomBy(1 / ZOOM_STEP)}>Zoom out</button>
          <button type="button" data-testid="comparison-overlay-fit" disabled={zoom === 1} onClick={resetView}>Fit</button>
          <button type="button" aria-label="Pan left" disabled={zoom === 1} onClick={() => pan(-1, 0)}>←</button>
          <button type="button" aria-label="Pan up" disabled={zoom === 1} onClick={() => pan(0, -1)}>↑</button>
          <button type="button" aria-label="Pan down" disabled={zoom === 1} onClick={() => pan(0, 1)}>↓</button>
          <button type="button" aria-label="Pan right" disabled={zoom === 1} onClick={() => pan(1, 0)}>→</button>
        </div>
        <div className="viewport-display-toggles" role="group" aria-label="Labels">
          <button type="button" data-testid="comparison-overlay-labels" aria-pressed={labels} className={labels ? "active" : ""} onClick={() => setLabels((value) => !value)}>Labels</button>
        </div>
        <small className="muted" data-testid="comparison-overlay-zoom">{zoom === 1 ? "Whole schematic" : `Zoom ×${zoom.toFixed(1)}`}</small>
      </div>
      <svg
        ref={svgRef}
        className="comparison-overlay-drawing"
        data-testid="comparison-overlay-drawing"
        data-view={view}
        data-up={up}
        viewBox={`${box.x} ${box.y} ${box.width} ${box.height}`}
        preserveAspectRatio="xMidYMid meet"
        role="group"
        aria-label={`Schematic of the later ${sides} with the comparison's marks, ${VIEW_LABELS[view]} view, up ${signedAxisText(up)}`}
        onWheel={onWheel}
        onPointerDown={onPointerDown}
        onPointerMove={onPointerMove}
        onPointerUp={endDrag}
        onPointerCancel={endDrag}
      >
        {drawing.projected.map(({ entity, marked, points, anchor, collapsed }) => {
          const style = primaryStyle(styles, entity);
          const glyphs = entity.classes.map((markClass) => styles.find((candidate) => candidate.markClass === markClass)!.glyph).join(" ");
          const name = `${overlayEntityName(entity)} (${entity.label})${entity.ghost ? `, only in the earlier ${sides}, drawn as a ghost` : ""}: ${overlayClassWords(overlay.kind, entity.classes)}`;
          const line = style?.line ?? "context";
          const swatch = style?.swatch ?? "context";
          const asPoint = points.length === 1 || collapsed;
          const stack = stacks.get(entity) ?? 0;
          const glyphY = anchor.y - glyphSize * 0.4 + stack * glyphSize * 1.15;
          const label = labelPlacement(`${entity.label}${entity.ghost ? " (earlier only)" : ""}`, anchor.x, glyphSize * 0.9, visible, labelSize);
          const labelX = label.side === "start" ? anchor.x + glyphSize * 0.9 : anchor.x - glyphSize * 0.9;
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
              data-located={located === entity.ref && marked ? "true" : undefined}
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
                <text className="comparison-overlay-label" data-side={label.side} x={labelX} y={glyphY} fontSize={labelSize} strokeWidth={labelSize * 0.25}
                  textAnchor={label.side} aria-hidden="true">
                  {label.text}
                </text>
              ) : null}
            </g>
          );
        })}
      </svg>
    </section>
  );
}
