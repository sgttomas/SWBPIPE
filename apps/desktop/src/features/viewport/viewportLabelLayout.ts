// Where the view's entity labels are drawn (DEL-07-01 REQ-002, REQ-012).
//
// A label is a small tag set off from its entity's anchor, never on it. Labels never overlap one
// another, the canvas furniture (the view cluster, the legend, the status line, the scale bar, the
// axis triad) or the canvas's edges. Candidates are placed in priority order and a label that has no
// free place is not drawn; it is drawn when its entity is hovered or selected, because those are
// placed first and always drawn.
//
// Priority: the selected entities (primary first), the hovered one, then nodes, supports,
// components and pipes, each kind in the order the label budget gave (`prioritizedLabelKeys`).
//
// Cost: the candidates are the label budget (at most LABEL_LAYOUT_BOUND); each tries at most four
// places against the boxes already placed, so one layout is at most 4 · n · (n + obstacles) box
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
};

export type LabelBox = { left: number; top: number; right: number; bottom: number };
export type LabelPlacement<K extends string = string> = { key: K; left: number; top: number; drawn: boolean };

/** The label budget's ceiling (`prioritizedLabelKeys`), and so the most labels one layout places. */
export const LABEL_LAYOUT_BOUND = 80;
/** How far a label stands off its anchor, in CSS pixels, so it never sits on the centreline. */
export const LABEL_OFFSET_PX = 6;
/** The margin kept clear inside the canvas's edges. */
export const LABEL_EDGE_PX = 4;

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

function overlaps(a: LabelBox, b: LabelBox): boolean {
  return a.left < b.right && a.right > b.left && a.top < b.bottom && a.bottom > b.top;
}

function inside(box: LabelBox, width: number, height: number): boolean {
  return box.left >= LABEL_EDGE_PX && box.top >= LABEL_EDGE_PX && box.right <= width - LABEL_EDGE_PX && box.bottom <= height - LABEL_EDGE_PX;
}

function clampInside(box: LabelBox, width: number, height: number): LabelBox {
  const w = box.right - box.left;
  const h = box.bottom - box.top;
  const left = Math.min(Math.max(box.left, LABEL_EDGE_PX), Math.max(LABEL_EDGE_PX, width - LABEL_EDGE_PX - w));
  const top = Math.min(Math.max(box.top, LABEL_EDGE_PX), Math.max(LABEL_EDGE_PX, height - LABEL_EDGE_PX - h));
  return { left, top, right: left + w, bottom: top + h };
}

/**
 * Places the labels. Each candidate takes the first of its four places that lies inside the canvas
 * and clear of every label already placed and of every obstacle. An emphasised label with no free
 * place takes its first place moved inside the canvas: it is always drawn. Any other label with no
 * free place is not drawn. Candidates past LABEL_LAYOUT_BOUND are not drawn.
 */
export function layoutLabels<K extends string>(
  candidates: readonly LabelCandidate<K>[],
  canvas: { width: number; height: number },
  obstacles: readonly LabelBox[] = []
): LabelPlacement<K>[] {
  const placed: LabelBox[] = [];
  const output: LabelPlacement<K>[] = [];
  const ordered = labelPlacementOrder(candidates);
  for (let index = 0; index < ordered.length; index += 1) {
    const candidate = ordered[index];
    if (index >= LABEL_LAYOUT_BOUND || !Number.isFinite(candidate.x) || !Number.isFinite(candidate.y)) {
      output.push({ key: candidate.key, left: 0, top: 0, drawn: false });
      continue;
    }
    const options = places(candidate);
    const free = options.find((box) => inside(box, canvas.width, canvas.height) &&
      !placed.some((other) => overlaps(box, other)) && !obstacles.some((other) => overlaps(box, other)));
    const box = free ?? (candidate.emphasised ? clampInside(options[0], canvas.width, canvas.height) : null);
    if (!box) {
      output.push({ key: candidate.key, left: 0, top: 0, drawn: false });
      continue;
    }
    placed.push(box);
    output.push({ key: candidate.key, left: box.left, top: box.top, drawn: true });
  }
  return output;
}
