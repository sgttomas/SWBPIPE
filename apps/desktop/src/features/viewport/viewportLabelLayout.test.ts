import { describe, expect, it } from "vitest";
import { LABEL_EDGE_PX, LABEL_LAYOUT_BOUND, LABEL_OFFSET_PX, layoutLabels, labelPlacementOrder, type LabelBox, type LabelCandidate } from "./viewportLabelLayout";

const CANVAS = { width: 600, height: 400 };

function label(key: string, kind: LabelCandidate["kind"], x: number, y: number, extra: Partial<LabelCandidate> = {}): LabelCandidate {
  return { key, kind, x, y, width: 40, height: 16, emphasised: false, ...extra };
}

function boxOf(placement: { left: number; top: number }, candidate: LabelCandidate): LabelBox {
  return { left: placement.left, top: placement.top, right: placement.left + candidate.width, bottom: placement.top + candidate.height };
}

function overlap(a: LabelBox, b: LabelBox) {
  return a.left < b.right && a.right > b.left && a.top < b.bottom && a.bottom > b.top;
}

describe("label placement order", () => {
  it("places the selection, then the hover, then nodes, supports, components and pipes", () => {
    const order = labelPlacementOrder([
      label("pipe:a", "pipe", 0, 0),
      label("component:a", "component", 0, 0),
      label("hover", "pipe", 0, 0, { emphasised: true, emphasisRank: 2 }),
      label("support:a", "support", 0, 0),
      label("node:a", "node", 0, 0),
      label("primary", "pipe", 0, 0, { emphasised: true, emphasisRank: 0 }),
      label("node:b", "node", 0, 0)
    ]).map((candidate) => candidate.key);
    expect(order).toEqual(["primary", "hover", "node:a", "node:b", "support:a", "component:a", "pipe:a"]);
  });
});

describe("label layout", () => {
  it("sets a label off its anchor, never on it", () => {
    const candidate = label("node:a", "node", 300, 200);
    const [placement] = layoutLabels([candidate], CANVAS);
    expect(placement.drawn).toBe(true);
    const box = boxOf(placement, candidate);
    expect(box.left).toBe(300 + LABEL_OFFSET_PX);
    expect(box.bottom).toBe(200 - LABEL_OFFSET_PX);
    expect(300 >= box.left && 300 <= box.right && 200 >= box.top && 200 <= box.bottom).toBe(false);
  });

  it("never draws two labels over each other; the lower priority one is not drawn", () => {
    // Five labels at one anchor: four places, so four are drawn, and the pipe (last) is not.
    const candidates = [
      label("pipe:p", "pipe", 300, 200),
      label("node:n", "node", 300, 200),
      label("support:s", "support", 300, 200),
      label("component:c", "component", 300, 200),
      label("node:m", "node", 300, 200)
    ];
    const placements = layoutLabels(candidates, CANVAS);
    const drawn = placements.filter((placement) => placement.drawn);
    expect(drawn.map((placement) => placement.key).sort()).toEqual(["component:c", "node:m", "node:n", "support:s"]);
    expect(placements.find((placement) => placement.key === "pipe:p")!.drawn).toBe(false);
    const boxes = drawn.map((placement) => boxOf(placement, candidates.find((candidate) => candidate.key === placement.key)!));
    for (let i = 0; i < boxes.length; i += 1) for (let j = i + 1; j < boxes.length; j += 1) expect(overlap(boxes[i], boxes[j])).toBe(false);
  });

  it("draws a hidden label when its entity is hovered or selected, and keeps it inside the canvas", () => {
    const crowd = ["node:a", "node:b", "node:c", "node:d"].map((key) => label(key, "node", 300, 200));
    const pipe = label("pipe:p", "pipe", 300, 200);
    expect(layoutLabels([...crowd, pipe], CANVAS).find((placement) => placement.key === "pipe:p")!.drawn).toBe(false);
    const hovered = { ...pipe, emphasised: true, emphasisRank: 1 };
    const placements = layoutLabels([...crowd, hovered], CANVAS);
    expect(placements.find((placement) => placement.key === "pipe:p")!.drawn).toBe(true);
    // The selected label at the canvas's corner is moved inside, not cut.
    const corner = label("node:corner", "node", 598, 2, { emphasised: true, emphasisRank: 0 });
    const [placement] = layoutLabels([corner], CANVAS);
    const box = boxOf(placement, corner);
    expect(box.left).toBeGreaterThanOrEqual(LABEL_EDGE_PX);
    expect(box.top).toBeGreaterThanOrEqual(LABEL_EDGE_PX);
    expect(box.right).toBeLessThanOrEqual(CANVAS.width - LABEL_EDGE_PX);
    expect(box.bottom).toBeLessThanOrEqual(CANVAS.height - LABEL_EDGE_PX);
  });

  it("keeps labels inside the canvas and clear of the canvas furniture", () => {
    // The view cluster at the top right; a node just under it takes a place below its anchor.
    const cluster: LabelBox = { left: 300, top: 8, right: 592, bottom: 32 };
    const candidate = label("node:a", "node", 400, 40);
    const [placement] = layoutLabels([candidate], CANVAS, [cluster]);
    expect(placement.drawn).toBe(true);
    expect(overlap(boxOf(placement, candidate), cluster)).toBe(false);
    expect(placement.top).toBe(40 + LABEL_OFFSET_PX);
    // A label that cannot be placed inside the canvas without covering furniture is not drawn.
    const edge = label("node:edge", "node", 596, 396);
    const foot: LabelBox = { left: 500, top: 340, right: 600, bottom: 400 };
    expect(layoutLabels([edge], CANVAS, [foot])[0].drawn).toBe(false);
  });

  it(`places at most ${LABEL_LAYOUT_BOUND} labels, and one layout of a full budget costs well under a frame`, () => {
    const candidates: LabelCandidate[] = [];
    for (let index = 0; index < 120; index += 1) {
      const kind = (["node", "pipe", "support", "component"] as const)[index % 4];
      candidates.push(label(`${kind}:${index}`, kind, 40 + (index * 37) % 520, 30 + (index * 53) % 340, { width: 34 + (index % 5) * 6 }));
    }
    const placements = layoutLabels(candidates, CANVAS);
    expect(placements.filter((placement) => placement.drawn).length).toBeLessThanOrEqual(LABEL_LAYOUT_BOUND);
    const budget = candidates.slice(0, LABEL_LAYOUT_BOUND);
    const runs = 500;
    const started = performance.now();
    for (let run = 0; run < runs; run += 1) layoutLabels(budget, CANVAS);
    const perLayoutMs = (performance.now() - started) / runs;
    // Recorded for the return; the bound is generous so a loaded host cannot fail it.
    console.info(`label layout: ${LABEL_LAYOUT_BOUND} candidates, ${perLayoutMs.toFixed(4)} ms per layout (mean of ${runs})`);
    expect(perLayoutMs).toBeLessThan(2);
  });
});
