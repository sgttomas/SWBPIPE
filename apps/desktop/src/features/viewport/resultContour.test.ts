import { describe, expect, it } from "vitest";
import {
  CONTOUR_MAX_PIECES_PER_INTERVAL,
  contourRings,
  contourValueLabels,
  interpolateStations,
  thresholdInOverlayUnit,
  type StationPoint
} from "./resultContour";
import type { ResultColourOverlay } from "./resultColourOverlay";

// The contour's display arithmetic (A4; DEL-07-05 Design `result-contour.md` §3 to §5). Invented
// values only; none has engineering meaning.

const FIVE: StationPoint[] = [
  { fraction: 0, value: 4 },
  { fraction: 0.25, value: 2 },
  { fraction: 0.5, value: 0 },
  { fraction: 0.75, value: 6 },
  { fraction: 1, value: 8 }
];

describe("interpolateStations", () => {
  it("is the stated value exactly at each station, ends included", () => {
    for (const station of FIVE) expect(interpolateStations(FIVE, station.fraction)).toBe(station.value);
  });

  it("is linear in the fraction between two adjacent stations", () => {
    expect(interpolateStations(FIVE, 0.125)).toBeCloseTo(3, 12);
    expect(interpolateStations(FIVE, 0.625)).toBeCloseTo(3, 12);
    expect(interpolateStations(FIVE, 0.8)).toBeCloseTo(6.4, 12);
  });

  it("extrapolates nothing: null before the first and after the last stated station", () => {
    const inner: StationPoint[] = [{ fraction: 0.25, value: 1 }, { fraction: 0.75, value: 3 }];
    expect(interpolateStations(inner, 0)).toBeNull();
    expect(interpolateStations(inner, 0.2499)).toBeNull();
    expect(interpolateStations(inner, 0.7501)).toBeNull();
    expect(interpolateStations(inner, 1)).toBeNull();
    expect(interpolateStations(FIVE, -0.01)).toBeNull();
    expect(interpolateStations(FIVE, 1.01)).toBeNull();
  });

  it("with one stated value: that value at its station and nothing elsewhere", () => {
    const one: StationPoint[] = [{ fraction: 0.5, value: 7 }];
    expect(interpolateStations(one, 0.5)).toBe(7);
    expect(interpolateStations(one, 0.4)).toBeNull();
    expect(interpolateStations(one, 0.6)).toBeNull();
    expect(interpolateStations([], 0.5)).toBeNull();
  });

  it("never interpolates across a station with no colourable value (null, NaN or infinite)", () => {
    for (const bad of [null, Number.NaN, Number.POSITIVE_INFINITY]) {
      const stations: StationPoint[] = [{ fraction: 0, value: 1 }, { fraction: 0.5, value: bad }, { fraction: 1, value: 3 }];
      expect(interpolateStations(stations, 0), String(bad)).toBe(1);
      expect(interpolateStations(stations, 0.25), String(bad)).toBeNull();
      expect(interpolateStations(stations, 0.5), String(bad)).toBeNull();
      expect(interpolateStations(stations, 0.75), String(bad)).toBeNull();
      expect(interpolateStations(stations, 1), String(bad)).toBe(3);
    }
  });

  it("blends nothing between adjacent stations of known opposite sign, and keeps each station's own value", () => {
    const opposite: StationPoint[] = [{ fraction: 0, value: 4, signed: true, sign: -1 }, { fraction: 0.5, value: 2, signed: true, sign: 1 }];
    expect(interpolateStations(opposite, 0)).toBe(4);
    expect(interpolateStations(opposite, 0.25)).toBeNull();
    expect(interpolateStations(opposite, 0.5)).toBe(2);
  });

  it("blends a signed quantity's stations of the same section sign or with a zero, and an unsigned quantity always", () => {
    const same: StationPoint[] = [{ fraction: 0, value: 4, signed: true, sign: 1 }, { fraction: 1, value: 2, signed: true, sign: 1 }];
    expect(interpolateStations(same, 0.5)).toBe(3);
    const zero: StationPoint[] = [{ fraction: 0, value: 0, signed: true, sign: 0 }, { fraction: 1, value: 2, signed: true, sign: 1 }];
    expect(interpolateStations(zero, 0.5)).toBe(1);
    expect(interpolateStations([{ fraction: 0, value: 4 }, { fraction: 1, value: 2 }], 0.5)).toBe(3);
  });

  it("blends nothing beside a signed station whose section sign cannot be established", () => {
    const unknown: StationPoint[] = [{ fraction: 0, value: 4, signed: true, sign: null }, { fraction: 1, value: 2, signed: true, sign: 1 }];
    expect(interpolateStations(unknown, 0)).toBe(4);
    expect(interpolateStations(unknown, 0.5)).toBeNull();
    expect(interpolateStations(unknown, 1)).toBe(2);
  });

  it("refuses a fraction that is not finite", () => {
    expect(interpolateStations(FIVE, Number.NaN)).toBeNull();
  });
});

describe("contourRings", () => {
  it("runs from the first to the last stated station, with every ring's value from the interpolation", () => {
    const rings = contourRings(FIVE, { maximum: 8 });
    expect(rings[0]).toEqual({ fraction: 0, kind: "value", position: 0.5 });
    expect(rings[rings.length - 1]).toEqual({ fraction: 1, kind: "value", position: 1 });
    for (const ring of rings) expect(ring.position * 8).toBeCloseTo(interpolateStations(FIVE, ring.fraction)!, 12);
    // Every stated station is a ring.
    for (const station of FIVE) expect(rings.some((ring) => ring.fraction === station.fraction)).toBe(true);
  });

  it("cuts an interval finely enough that a piece spans at most a twelfth of the scale, within the cap", () => {
    const rings = contourRings([{ fraction: 0, value: 0 }, { fraction: 1, value: 10 }], { maximum: 10 });
    expect(rings).toHaveLength(CONTOUR_MAX_PIECES_PER_INTERVAL + 1);
    const flat = contourRings([{ fraction: 0, value: 5 }, { fraction: 1, value: 5 }], { maximum: 10 });
    expect(flat.map((ring) => ring.fraction)).toEqual([0, 1]);
  });

  it("places a threshold crossing exactly, as a hard edge, and leaves the part at or below it uncoloured", () => {
    const rings = contourRings([{ fraction: 0, value: 2 }, { fraction: 1, value: 6 }], { maximum: 6, threshold: 3 });
    const edge = rings.filter((ring) => ring.fraction === 0.25);
    expect(edge).toEqual([{ fraction: 0.25, kind: "below", position: 0 }, { fraction: 0.25, kind: "value", position: 0.5 }]);
    expect(rings.filter((ring) => ring.fraction < 0.25).every((ring) => ring.kind === "below")).toBe(true);
    expect(rings.filter((ring) => ring.fraction > 0.25).every((ring) => ring.kind === "value")).toBe(true);
    // Falling: the coloured side comes first.
    const falling = contourRings([{ fraction: 0, value: 6 }, { fraction: 1, value: 2 }], { maximum: 6, threshold: 3 });
    expect(falling.filter((ring) => ring.fraction === 0.75).map((ring) => ring.kind)).toEqual(["value", "below"]);
  });

  it("puts the hard edge at a station whose value equals the threshold, so every value above it is coloured", () => {
    const rising = contourRings([{ fraction: 0, value: 5 }, { fraction: 1, value: 6 }], { maximum: 6, threshold: 5 });
    expect(rising.filter((ring) => ring.fraction === 0)).toEqual([{ fraction: 0, kind: "below", position: 0 }, { fraction: 0, kind: "value", position: 5 / 6 }]);
    expect(rising.filter((ring) => ring.fraction > 0).every((ring) => ring.kind === "value")).toBe(true);
    const falling = contourRings([{ fraction: 0, value: 6 }, { fraction: 1, value: 5 }], { maximum: 6, threshold: 5 });
    expect(falling.filter((ring) => ring.fraction === 1)).toEqual([{ fraction: 1, kind: "value", position: 5 / 6 }, { fraction: 1, kind: "below", position: 0 }]);
    expect(falling.filter((ring) => ring.fraction < 1).every((ring) => ring.kind === "value")).toBe(true);
    // No consecutive pair of rings at different fractions blends plain into colour.
    for (const rings of [rising, falling]) {
      rings.slice(1).forEach((ring, index) => {
        const previous = rings[index];
        if (previous.fraction !== ring.fraction) expect(new Set([previous.kind, ring.kind]).size, JSON.stringify([previous, ring])).toBe(1);
      });
    }
  });

  it("draws an interval beside a station with no value as one gap piece, never coloured, keeping each stated value", () => {
    const rings = contourRings([{ fraction: 0, value: 1 }, { fraction: 0.5, value: null }, { fraction: 1, value: 3 }], { maximum: 3 });
    expect(rings).toEqual([
      { fraction: 0, kind: "value", position: 1 / 3 },
      { fraction: 0, kind: "gap", position: 0 },
      { fraction: 0.5, kind: "gap", position: 0 },
      { fraction: 1, kind: "gap", position: 0 },
      { fraction: 1, kind: "value", position: 1 }
    ]);
  });

  it("without a band, draws a sign change as a gap piece with each station's value as a ring at the station", () => {
    const rings = contourRings([
      { fraction: 0, value: 4, signed: true, sign: -1 },
      { fraction: 0.5, value: 2, signed: true, sign: 1 },
      { fraction: 1, value: 2, signed: true, sign: 1 }
    ], { maximum: 4 });
    expect(rings).toEqual([
      { fraction: 0, kind: "value", position: 1 },
      { fraction: 0, kind: "gap", position: 0 },
      { fraction: 0.5, kind: "gap", position: 0 },
      { fraction: 0.5, kind: "value", position: 0.5 },
      { fraction: 1, kind: "value", position: 0.5 }
    ]);
  });

  it("with a band, keeps every stated station visible: a band of non-zero length in its own colour, clipped to the span", () => {
    // An isolated interior station between two gaps, and end stations beside gaps.
    const stations: StationPoint[] = [
      { fraction: 0, value: 4, signed: true, sign: -1 },
      { fraction: 0.5, value: 2, signed: true, sign: 1 },
      { fraction: 1, value: 3, signed: true, sign: -1 }
    ];
    const rings = contourRings(stations, { maximum: 4, bandHalfFraction: 0.05 });
    expect(rings).toEqual([
      { fraction: 0, kind: "value", position: 1 },
      { fraction: 0.05, kind: "value", position: 1 },
      { fraction: 0.05, kind: "gap", position: 0 },
      { fraction: 0.45, kind: "gap", position: 0 },
      { fraction: 0.45, kind: "value", position: 0.5 },
      { fraction: 0.55, kind: "value", position: 0.5 },
      { fraction: 0.55, kind: "gap", position: 0 },
      { fraction: 0.95, kind: "gap", position: 0 },
      { fraction: 0.95, kind: "value", position: 0.75 },
      { fraction: 1, kind: "value", position: 0.75 }
    ]);
  });

  it("with a band, meets the blend at the band's edge with the interpolated value", () => {
    const rings = contourRings([{ fraction: 0, value: 0 }, { fraction: 1, value: 4 }], { maximum: 4, bandHalfFraction: 0.1 });
    expect(rings.slice(0, 3)).toEqual([
      { fraction: 0, kind: "value", position: 0 },
      { fraction: 0.1, kind: "value", position: 0 },
      { fraction: 0.1, kind: "value", position: 0.1 }
    ]);
    expect(rings[rings.length - 1]).toEqual({ fraction: 1, kind: "value", position: 1 });
    for (const ring of rings.filter((item) => item.fraction > 0.1 && item.fraction < 0.9)) expect(ring.position * 4).toBeCloseTo(interpolateStations([{ fraction: 0, value: 0 }, { fraction: 1, value: 4 }], ring.fraction)!, 12);
  });

  it("has no rings for fewer than two stations", () => {
    expect(contourRings([{ fraction: 0.5, value: 1 }], { maximum: 1 })).toEqual([]);
  });
});

function overlayWith(members: Array<[string, number, Array<[number, number | null]>]>): ResultColourOverlay {
  return {
    state: "available", reason: null, quantity: null, case: null, resolution: "station", unit: "MPa",
    members: new Map(members.map(([id, value, stations]) => [id, {
      value, rowId: `${id}:max`, position: 0,
      stations: stations.map(([fraction, stationValue]) => ({
        fraction, value: stationValue, rowId: stationValue === null ? null : `${id}@${fraction}`, rowIds: [`${id}@${fraction}`], signed: false, sign: null
      }))
    }])),
    memberCount: members.length, noValueCount: 0, heldRowCount: 0, stationMemberCount: 0, gapStationCount: 0, signGapIntervalCount: 0,
    unplacedRowCount: 0, noRatioCount: 0, minimum: 0, maximum: 10
  };
}

describe("contourValueLabels (numbers in place of colours)", () => {
  const overlay = overlayWith([
    ["pipe:P-1", 9, [[0, 9], [0.5, null], [1, 4]]],
    ["pipe:P-2", 6, []]
  ]);

  it("labels only stated values: each station's own value and a member's one value, largest first", () => {
    const { labels, candidateCount } = contourValueLabels(overlay);
    expect(candidateCount).toBe(3);
    expect(labels.map((label) => [label.memberId, label.fraction, label.value, label.oneValue, label.rowId])).toEqual([
      ["pipe:P-1", 0, 9, false, "pipe:P-1@0"],
      ["pipe:P-2", 0.5, 6, true, "pipe:P-2:max"],
      ["pipe:P-1", 1, 4, false, "pipe:P-1@1"]
    ]);
  });

  it("under a threshold, labels only values above it; and keeps to the budget", () => {
    expect(contourValueLabels(overlay, { threshold: 6 }).labels.map((label) => label.value)).toEqual([9]);
    const capped = contourValueLabels(overlay, { budget: 1 });
    expect(capped.labels).toHaveLength(1);
    expect(capped.candidateCount).toBe(3);
  });

  it("labels nothing for an overlay that is not available", () => {
    expect(contourValueLabels({ ...overlay, state: "held" })).toEqual({ labels: [], candidateCount: 0 });
  });
});

describe("thresholdInOverlayUnit", () => {
  it("reads the text in the legend's unit and states it in the stored unit", () => {
    expect(thresholdInOverlayUnit("3", 1)).toBe(3);
    // Stored in Pa, shown in MPa: one stored unit is 1e-6 display units.
    expect(thresholdInOverlayUnit("120", 1e-6)).toBeCloseTo(1.2e8, 0);
    expect(thresholdInOverlayUnit(" 2.5e1 ", 1)).toBe(25);
  });

  it("is null for empty or non-numeric text or a factor that cannot convert", () => {
    for (const text of ["", " ", "abc", "1,5", "1e", "--1"]) expect(thresholdInOverlayUnit(text, 1), text).toBeNull();
    expect(thresholdInOverlayUnit("1", 0)).toBeNull();
    expect(thresholdInOverlayUnit("1", Number.NaN)).toBeNull();
  });
});
