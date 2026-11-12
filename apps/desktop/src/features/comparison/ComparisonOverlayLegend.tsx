import { GitCompare } from "lucide-react";
import {
  overlayClasses,
  overlayClassWords,
  type ComparisonOverlayKind,
  type NotDrawnReason,
  type OverlayClass,
  type OverlayClassCount,
  type OverlayClassStyle,
  type OverlayMark
} from "./comparisonOverlay";
import type { EntityFamily } from "./stateComparison";

// The comparison overlay's banner, legend and list of marks (DEL-07-08 REQ-011), shared by every
// presentation of the overlay: the schematic now, marks in the three-dimensional viewport later.
// A presentation gives the comparison kind, its own per-class counts, the entries it did not
// place, the marks it placed and a `locate` callback; the list calls `locate(ref, family)` and
// never a presentation's view state. Choosing an entry moves the presentation's view to it; it
// does not change the selection. Words only state classes and counts: a difference is shown, not
// judged (AX-003).

/** The classes that are context: drawn, counted, not marks. */
export const CONTEXT_CLASSES: ReadonlySet<OverlayClass> = new Set<OverlayClass>(["unchanged", "zero_delta"]);

/** One mark a presentation placed, as the list names it. */
export type PlacedOverlayMark = Readonly<{ ref: string; family: EntityFamily; name: string; classes: readonly OverlayClass[]; ghost: boolean }>;

/** Why an entry was not placed, in a word or two. */
const NOT_PLACED_WORDS: Readonly<Record<NotDrawnReason, string>> = {
  no_drawing: "no drawing",
  no_position: "no readable position",
  not_in_either_model: "in neither model",
  length_unit_differs: "length unit differs"
};

/** A class's line and glyph as the legend shows them, hidden from the accessibility tree (the
 * words beside it carry the class). */
function LegendSample({ style, ghost = false }: { style: OverlayClassStyle; ghost?: boolean }) {
  return (
    <svg className="comparison-overlay-sample" viewBox="0 0 36 12" width="36" height="12" aria-hidden="true" focusable="false" data-ghost={ghost ? "true" : undefined}>
      <line className="comparison-overlay-line" data-line={style.line} data-swatch={style.swatch} x1="1" y1="6" x2="24" y2="6" />
      <text className="comparison-overlay-glyph" data-swatch={style.swatch} x="30" y="10" textAnchor="middle" fontSize="11">{style.glyph}</text>
    </svg>
  );
}

export type ComparisonOverlayLegendProps = {
  kind: ComparisonOverlayKind;
  leftText: string;
  rightText: string;
  /** What this presentation is, for the banner's one sentence (for example "a schematic in fixed views, not the three-dimensional viewport"). */
  presentation: string;
  counts: readonly OverlayClassCount[];
  notPlaced: readonly Readonly<{ mark: OverlayMark; reason: NotDrawnReason }>[];
  placed: readonly PlacedOverlayMark[];
  /** The mark the presentation last moved its view to, or null. */
  located: string | null;
  locate: (ref: string, family: EntityFamily) => void;
  onLeave: () => void;
};

/** The banner (one sentence and the way out) and the legend with the list of marks. They are two
 * elements, `comparison-overlay-banner` and `comparison-overlay-legend`, for the presentation's layout. */
export function ComparisonOverlayLegend({ kind, leftText, rightText, presentation, counts, notPlaced, placed, located, locate, onLeave }: ComparisonOverlayLegendProps) {
  const styles = overlayClasses(kind);
  const side = kind === "states" ? "saved state" : "run";
  const notPlacedByClass = styles.flatMap((style) => {
    const count = notPlaced.filter((entry) => entry.mark.markClass === style.markClass).length;
    return count ? [`${style.label} ${count}`] : [];
  });
  const reasons = [...new Set(notPlaced.map((entry) => NOT_PLACED_WORDS[entry.reason]))];
  const glyphsOf = (classes: readonly OverlayClass[]) => classes.map((markClass) => styles.find((style) => style.markClass === markClass)!.glyph).join(" ");
  return (
    <>
      <div className="comparison-overlay-banner" role="note" data-testid="comparison-overlay-banner">
        <GitCompare size={16} aria-hidden="true" />
        <p>
          Comparison overlay: {presentation}, of the later {side}, {rightText}, against the earlier, {leftText}; it is not the
          working model and nothing in it can be edited.
        </p>
        <button type="button" data-testid="comparison-overlay-leave" onClick={onLeave}>Leave the comparison overlay</button>
      </div>
      <section className="comparison-overlay-legend" aria-label="Comparison overlay legend" data-testid="comparison-overlay-legend">
        <p className="viewport-legend-title">Legend</p>
        <ul className="comparison-overlay-classes">
          {styles.map((style) => {
            const count = counts.find((entry) => entry.markClass === style.markClass)!;
            return (
              <li key={style.markClass} data-testid={`comparison-overlay-legend-${style.markClass}`} data-line={style.line} data-glyph={style.glyph}>
                <LegendSample style={style} />
                <span>{style.label} {count.total}{CONTEXT_CLASSES.has(style.markClass) ? " (context)" : ""}</span>
              </li>
            );
          })}
          <li data-testid="comparison-overlay-legend-ghost">
            <LegendSample style={styles[0]} ghost />
            <span>Faint and hollow: only in the earlier {side}</span>
          </li>
        </ul>
        <p className="comparison-overlay-not-drawn" data-testid="comparison-overlay-not-drawn">
          {notPlaced.length === 0
            ? "Every classified entity is drawn."
            : `Not drawn ${notPlaced.length} (${notPlacedByClass.join(", ")}): ${reasons.join(", ")}; they are in the table.`}
        </p>
        <div className="viewport-legend-block">
          <p className="viewport-legend-title" id="comparison-overlay-marks-title">Marks ({placed.length})</p>
          {placed.length ? (
            <ul className="comparison-overlay-marks" aria-labelledby="comparison-overlay-marks-title" data-testid="comparison-overlay-marks">
              {placed.map((mark) => (
                <li key={`${mark.ghost ? "ghost" : "drawn"}:${mark.ref}`}>
                  <button
                    type="button"
                    data-testid={`comparison-overlay-mark-${mark.ref}`}
                    aria-current={located === mark.ref ? "location" : undefined}
                    title={`${mark.ref}: ${overlayClassWords(kind, mark.classes)}`}
                    onClick={() => locate(mark.ref, mark.family)}
                  >
                    <span aria-hidden="true" className="comparison-overlay-mark-glyph">{glyphsOf(mark.classes)}</span>{" "}
                    <span>{mark.name}</span>{" "}
                    <span className="muted">{overlayClassWords(kind, mark.classes.filter((markClass) => !CONTEXT_CLASSES.has(markClass)))}{mark.ghost ? `, earlier ${side} only` : ""}</span>
                  </button>
                </li>
              ))}
            </ul>
          ) : <p className="muted">No marked entity is drawn.</p>}
          <p className="muted">Choose a mark to move the view to it. Activate a mark in the drawing to open its row in the table.</p>
        </div>
      </section>
    </>
  );
}
