import { describe, expect, it } from "vitest";
import { LABEL_EDGE_PX, LABEL_LAYOUT_BOUND, LABEL_OFFSET_PX, LABEL_TARGET_GAP_PX, labelTargetBox, layoutLabels, labelPlacementOrder, type LabelBox, type LabelCandidate } from "./viewportLabelLayout";

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

describe("which labels come first on a crowded model", () => {
  it("places nodes that bound the system (a support, a line's end) before the other nodes", () => {
    const order = labelPlacementOrder([
      label("node:middle", "node", 0, 0),
      label("pipe:p", "pipe", 0, 0),
      label("node:anchor", "node", 0, 0, { boundary: true }),
      label("support:s", "support", 0, 0),
      label("node:free-end", "node", 0, 0, { boundary: true })
    ]).map((candidate) => candidate.key);
    expect(order).toEqual(["node:anchor", "node:free-end", "node:middle", "support:s", "pipe:p"]);
  });

  it("names a boundary node where an ordinary node beside it would take its place", () => {
    const placements = layoutLabels([
      label("node:middle", "node", 300, 200),
      label("node:a", "node", 300, 200), label("node:b", "node", 300, 200), label("node:c", "node", 300, 200),
      label("node:anchor", "node", 300, 200, { boundary: true })
    ], CANVAS);
    expect(placements.find((placement) => placement.key === "node:anchor")!.drawn).toBe(true);
  });
});

describe("emphasised labels and a label's own entity", () => {
  it("draws the primary's label and gives every other selected label a free place or none: no two targets meet", () => {
    // Six selected entities at one projected point, around a crowd.
    const selected = ["node:p", "node:s1", "node:s2", "node:s3", "node:s4", "node:s5"].map((key, index) =>
      label(key, "node", 300, 200, { emphasised: true, emphasisRank: index === 0 ? 0 : 1 }));
    const crowd = Array.from({ length: 12 }, (_, index) => label(`pipe:${index}`, "pipe", 290 + index * 2, 196 + index));
    const candidates = [...crowd, ...selected];
    const drawn = layoutLabels(candidates, CANVAS).filter((placement) => placement.drawn);
    expect(drawn.map((placement) => placement.key)).toContain("node:p");
    const targets = drawn.map((placement) => labelTargetBox(boxOf(placement, candidates.find((candidate) => candidate.key === placement.key)!)));
    for (let i = 0; i < targets.length; i += 1) for (let j = i + 1; j < targets.length; j += 1) expect(overlap(targets[i], targets[j])).toBe(false);
  });

  it("keeps a selected label off the canvas furniture when it has any free place", () => {
    const cluster: LabelBox = { left: 280, top: 150, right: 600, bottom: 196 };
    const primary = label("node:p", "node", 300, 200, { emphasised: true, emphasisRank: 0 });
    const [placement] = layoutLabels([primary], CANVAS, [cluster]);
    expect(overlap(labelTargetBox(boxOf(placement, primary)), cluster)).toBe(false);
  });

  it("never covers its own anchor or its own entity's line, so a press beside the label reaches the entity", () => {
    // A pipe rising to the right through its anchor: the above-right place would cover it.
    const pipe = label("pipe:p", "pipe", 300, 200, { avoid: [{ x1: 260, y1: 240, x2: 340, y2: 160 }] });
    const [placement] = layoutLabels([pipe], CANVAS);
    expect(placement.drawn).toBe(true);
    expect(placement.place).not.toBe(0);
    const target = labelTargetBox(boxOf(placement, pipe));
    expect(300 >= target.left && 300 <= target.right && 200 >= target.top && 200 <= target.bottom).toBe(false);
    for (let step = 0; step <= 80; step += 1) {
      const x = 260 + step;
      const y = 240 - step;
      expect(x >= target.left && x <= target.right && y >= target.top && y <= target.bottom).toBe(false);
    }
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

  it("keeps a label where it was when it becomes hovered, so it does not move from under the pointer", () => {
    // Above right is taken by the node, so the pipe went above left (place 1).
    const node = label("node:a", "node", 300, 200);
    const pipe = label("pipe:p", "pipe", 300, 200);
    const before = layoutLabels([node, pipe], CANVAS).find((placement) => placement.key === "pipe:p")!;
    expect(before.place).toBe(1);
    // Hovered, the pipe is placed first; with its previous place preferred it stays there.
    const after = layoutLabels([node, { ...pipe, emphasised: true, emphasisRank: 2, preferredPlace: before.place }], CANVAS);
    const hovered = after.find((placement) => placement.key === "pipe:p")!;
    expect([hovered.left, hovered.top]).toEqual([before.left, before.top]);
    expect(after.find((placement) => placement.key === "node:a")!.drawn).toBe(true);
  });

  it("keeps each drawn label's 24 px pointer target clear of every other target, the furniture and the edges", () => {
    // A plate 16 px tall and one 14 px wide: their targets are 24 by 24 at least, centred on the plate.
    expect(labelTargetBox({ left: 100, top: 50, right: 140, bottom: 66 })).toEqual({ left: 100, top: 46, right: 140, bottom: 70 });
    expect(labelTargetBox({ left: 100, top: 50, right: 114, bottom: 66 })).toEqual({ left: 95, top: 46, right: 119, bottom: 70 });
    // Two anchors 20 px apart vertically: the plates' above-right places (16 px tall, 4 px apart)
    // would not touch, but their targets would, so the second takes another place.
    const upper = label("node:upper", "node", 300, 180);
    const lower = label("node:lower", "node", 300, 200);
    const placements = layoutLabels([upper, lower], CANVAS);
    const first = placements.find((placement) => placement.key === "node:upper")!;
    const second = placements.find((placement) => placement.key === "node:lower")!;
    expect(first.place).toBe(0);
    const upperPlate = boxOf(first, upper);
    const lowerPlateAtPlace0 = boxOf({ left: 300 + LABEL_OFFSET_PX, top: 200 - LABEL_OFFSET_PX - 16 }, lower);
    expect(overlap(upperPlate, lowerPlateAtPlace0)).toBe(false);
    expect(overlap(labelTargetBox(upperPlate), labelTargetBox(lowerPlateAtPlace0))).toBe(true);
    expect(second.drawn).toBe(true);
    expect(second.place).not.toBe(0);
    // A crowd: every drawn target is at least 24 by 24, inside the canvas, and overlaps no other
    // target and no obstacle.
    const cluster: LabelBox = { left: 400, top: 8, right: 592, bottom: 32 };
    const crowd: LabelCandidate[] = [];
    for (let index = 0; index < 60; index += 1) {
      crowd.push(label(`node:${index}`, "node", 30 + (index * 47) % 540, 20 + (index * 29) % 360, { width: 12 + (index % 6) * 8 }));
    }
    const drawn = layoutLabels(crowd, CANVAS, [cluster]).filter((placement) => placement.drawn);
    expect(drawn.length).toBeGreaterThan(10);
    const targets = drawn.map((placement) => labelTargetBox(boxOf(placement, crowd.find((candidate) => candidate.key === placement.key)!)));
    for (const target of targets) {
      expect(target.right - target.left).toBeGreaterThanOrEqual(24);
      expect(target.bottom - target.top).toBeGreaterThanOrEqual(24);
      expect(target.left).toBeGreaterThanOrEqual(0);
      expect(target.top).toBeGreaterThanOrEqual(0);
      expect(target.right).toBeLessThanOrEqual(CANVAS.width);
      expect(target.bottom).toBeLessThanOrEqual(CANVAS.height);
      expect(overlap(target, cluster)).toBe(false);
    }
    // And a pixel apart, so the view's rounding of each place to a whole pixel cannot make two meet.
    const grown = targets.map((target) => ({ left: target.left - LABEL_TARGET_GAP_PX, top: target.top - LABEL_TARGET_GAP_PX, right: target.right + LABEL_TARGET_GAP_PX, bottom: target.bottom + LABEL_TARGET_GAP_PX }));
    for (let i = 0; i < targets.length; i += 1) for (let j = i + 1; j < targets.length; j += 1) expect(overlap(grown[i], targets[j])).toBe(false);
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
