// Where the view's entity labels are drawn (DEL-07-01 REQ-002, REQ-012).
//
// A label is a small tag set off from its entity's anchor, never on it. Its pointer target is at
// least LABEL_TARGET_PX square, centred on the drawn plate (DEL-07-06 REQ-014, Target Size
// (Minimum) 2.5.8), and it is the target, not only the plate, that is kept apart: no drawn label's
// target overlaps another's, the canvas furniture (the view cluster, the legend, the status line,
// the scale bar, the axis triad) or the canvas's edges. Candidates are placed in priority order and a label that has no
// free place is not drawn; it is drawn when its entity is hovered or selected, because those are
// placed first and always drawn.
//
// Priority: the selected entities (primary first), the hovered one, then nodes, supports,
// components and pipes, each kind in the order the label budget gave (`prioritizedLabelKeys`).
//
// Cost: the candidates are the label budget (at most LABEL_LAYOUT_BOUND); each tries at most four
// places against the targets already placed, so one layout is at most 4 · n · (n + obstacles) box
// tests. The view recomputes it only when the camera, the model, the label set or the canvas size
// has changed.

export type LabelKind = "node" | "pipe" | "support" | "component";

export type LabelCandidate<K extends string = string> = {
  key: K;
  kind: LabelKind;
  /** The entity's anchor in canvas CSS pixels. */
  x: number;
  y: number;
  width: number;
  height: number;
  /** Selected or hovered: placed first and always drawn. */
  emphasised: boolean;
  /** Rank among emphasised labels: the primary selection 0, other selections, then the hover. */
  emphasisRank?: number;
  /** The place (0 to 3) this label took in the previous layout: tried first, so that a label
   * that becomes hovered or selected does not jump from under the pointer. */
  preferredPlace?: number;
};

export type LabelBox = { left: number; top: number; right: number; bottom: number };
export type LabelPlacement<K extends string = string> = { key: K; left: number; top: number; drawn: boolean; place: number };

/** The label budget's ceiling (`prioritizedLabelKeys`), and so the most labels one layout places. */
export const LABEL_LAYOUT_BOUND = 80;
/** How far a label stands off its anchor, in CSS pixels, so it never sits on the centreline. */
export const LABEL_OFFSET_PX = 6;
/** The margin kept clear between a plate and the canvas's edges. */
export const LABEL_EDGE_PX = 4;
/** The least width and height of a label's pointer target (WCAG 2.2 2.5.8), centred on its plate;
 * styles.css draws the same box as the label's transparent hit area. */
export const LABEL_TARGET_PX = 24;
/** The least space kept between two targets, and between a target and the furniture: the view
 * writes each place rounded to a whole pixel, which can move a box by half a pixel. */
export const LABEL_TARGET_GAP_PX = 1;

const KIND_ORDER: Record<LabelKind, number> = { node: 0, support: 1, component: 2, pipe: 3 };

/** The candidates in the order they are placed. Stable: equal ranks keep their given order. */
export function labelPlacementOrder<K extends string>(candidates: readonly LabelCandidate<K>[]): LabelCandidate<K>[] {
  return candidates
    .map((candidate, index) => ({ candidate, index }))
    .sort((a, b) => {
      const ea = a.candidate.emphasised ? 0 : 1;
      const eb = b.candidate.emphasised ? 0 : 1;
      if (ea !== eb) return ea - eb;
      if (ea === 0) {
        const ra = a.candidate.emphasisRank ?? 0;
        const rb = b.candidate.emphasisRank ?? 0;
        if (ra !== rb) return ra - rb;
      } else {
        const ka = KIND_ORDER[a.candidate.kind];
        const kb = KIND_ORDER[b.candidate.kind];
        if (ka !== kb) return ka - kb;
      }
      return a.index - b.index;
    })
    .map(({ candidate }) => candidate);
}

/** The four places a label may take, nearest first: above right, above left, below right, below left. */
function places(candidate: LabelCandidate): LabelBox[] {
  const { x, y, width, height } = candidate;
  const d = LABEL_OFFSET_PX;
  return [
    { left: x + d, top: y - d - height },
    { left: x - d - width, top: y - d - height },
    { left: x + d, top: y + d },
    { left: x - d - width, top: y + d }
  ].map(({ left, top }) => ({ left, top, right: left + width, bottom: top + height }));
}

/** How far the target reaches past the plate on each side, horizontally and vertically. */
function targetReach(candidate: { width: number; height: number }): { dx: number; dy: number } {
  return { dx: Math.max(0, (LABEL_TARGET_PX - candidate.width) / 2), dy: Math.max(0, (LABEL_TARGET_PX - candidate.height) / 2) };
}

/** A plate's pointer target: the plate grown to at least LABEL_TARGET_PX on each axis, centred. */
export function labelTargetBox(plate: LabelBox): LabelBox {
  const { dx, dy } = targetReach({ width: plate.right - plate.left, height: plate.bottom - plate.top });
  return { left: plate.left - dx, top: plate.top - dy, right: plate.right + dx, bottom: plate.bottom + dy };
}

function overlaps(a: LabelBox, b: LabelBox): boolean {
  return a.left < b.right && a.right > b.left && a.top < b.bottom && a.bottom > b.top;
}

/** Whether a target comes within LABEL_TARGET_GAP_PX of another box. */
function crowds(target: LabelBox, other: LabelBox): boolean {
  const g = LABEL_TARGET_GAP_PX;
  return overlaps({ left: target.left - g, top: target.top - g, right: target.right + g, bottom: target.bottom + g }, other);
}

/** The plate keeps LABEL_EDGE_PX from the canvas's edges and its target stays inside the canvas. */
function margins(box: LabelBox): { x: number; y: number } {
  const { dx, dy } = targetReach({ width: box.right - box.left, height: box.bottom - box.top });
  return { x: Math.max(LABEL_EDGE_PX, dx), y: Math.max(LABEL_EDGE_PX, dy) };
}

function inside(box: LabelBox, width: number, height: number): boolean {
  const m = margins(box);
  return box.left >= m.x && box.top >= m.y && box.right <= width - m.x && box.bottom <= height - m.y;
}

function clampInside(box: LabelBox, width: number, height: number): LabelBox {
  const w = box.right - box.left;
  const h = box.bottom - box.top;
  const m = margins(box);
  const left = Math.min(Math.max(box.left, m.x), Math.max(m.x, width - m.x - w));
  const top = Math.min(Math.max(box.top, m.y), Math.max(m.y, height - m.y - h));
  return { left, top, right: left + w, bottom: top + h };
}

/**
 * Places the labels. Each candidate takes the first of its four places whose plate lies inside the
 * canvas and whose target (labelTargetBox) keeps LABEL_TARGET_GAP_PX from every target already
 * placed and from every obstacle. An emphasised label with no free
 * place takes its first place moved inside the canvas: it is always drawn. Any other label with no
 * free place is not drawn. Candidates past LABEL_LAYOUT_BOUND are not drawn.
 */
export function layoutLabels<K extends string>(
  candidates: readonly LabelCandidate<K>[],
  canvas: { width: number; height: number },
  obstacles: readonly LabelBox[] = []
): LabelPlacement<K>[] {
  /** The targets of the labels placed so far. */
  const placed: LabelBox[] = [];
  const output: LabelPlacement<K>[] = [];
  const ordered = labelPlacementOrder(candidates);
  for (let index = 0; index < ordered.length; index += 1) {
    const candidate = ordered[index];
    if (index >= LABEL_LAYOUT_BOUND || !Number.isFinite(candidate.x) || !Number.isFinite(candidate.y)) {
      output.push({ key: candidate.key, left: 0, top: 0, drawn: false, place: -1 });
      continue;
    }
    const options = places(candidate);
    const preferred = candidate.preferredPlace;
    const order = preferred !== undefined && preferred >= 0 && preferred < options.length
      ? [preferred, ...[0, 1, 2, 3].filter((index) => index !== preferred)]
      : [0, 1, 2, 3];
    const targets = options.map(labelTargetBox);
    const free = order.find((index) => inside(options[index], canvas.width, canvas.height) &&
      !placed.some((other) => crowds(targets[index], other)) && !obstacles.some((other) => crowds(targets[index], other)));
    const place = free ?? (candidate.emphasised ? order[0] : -1);
    if (place < 0) {
      output.push({ key: candidate.key, left: 0, top: 0, drawn: false, place: -1 });
      continue;
    }
    const box = free !== undefined ? options[place] : clampInside(options[place], canvas.width, canvas.height);
    placed.push(labelTargetBox(box));
    output.push({ key: candidate.key, left: box.left, top: box.top, drawn: true, place });
  }
  return output;
}
