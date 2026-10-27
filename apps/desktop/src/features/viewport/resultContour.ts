// The result contour along members (survey item A4; DEL-07-05 Design `result-contour.md`). Pure
// display arithmetic over the stations a result states: the one interpolation the contour uses,
// the rings the tube is drawn with, the stated values the numbers display labels and the
// threshold in the overlay's unit. Nothing here forms an engineering quantity, a limit or a status,
// and nothing writes to the model, the result or any export.

import { aboveThreshold, type ResultColourOverlay } from "./resultColourOverlay";

/** One stated station: its fraction along the member (0 at the From node, 1 at the To node) and
 * its value, or null where the station has no colourable value. */
export type StationPoint = Readonly<{ fraction: number; value: number | null }>;

function colourable(value: number | null): value is number {
  return value !== null && Number.isFinite(value);
}

/**
 * The contour's value at a fraction along a member: linear in the fraction between the two
 * adjacent stated stations, and the stated value exactly at a station. Null where there is no
 * stated value to show:
 * - before the first or after the last stated station (nothing is extrapolated);
 * - at a station with no colourable value (null or not finite), and anywhere between it and its
 *   neighbours (nothing is interpolated across it);
 * - for no stations, and away from the single station of a member with one.
 * `stations` are in increasing fraction, each fraction once. A display device only: it states no
 * value between stations that the result does not.
 */
export function interpolateStations(stations: readonly StationPoint[], fraction: number): number | null {
  if (!Number.isFinite(fraction) || stations.length === 0) return null;
  const first = stations[0];
  const last = stations[stations.length - 1];
  if (fraction < first.fraction || fraction > last.fraction) return null;
  for (let index = 0; index < stations.length; index += 1) {
    const station = stations[index];
    if (fraction === station.fraction) return colourable(station.value) ? station.value : null;
    const next = stations[index + 1];
    if (!next || fraction > next.fraction) continue;
    if (fraction === next.fraction) continue;
    if (!colourable(station.value) || !colourable(next.value) || next.fraction <= station.fraction) return null;
    const t = (fraction - station.fraction) / (next.fraction - station.fraction);
    return station.value + (next.value - station.value) * t;
  }
  return null;
}

/** A ring of the contour tube: where it is along the member, and what it shows there. "value" is a
 * position on the result scale; "below" is under the threshold (the plain pipe colour); "gap" has
 * no colourable value (the no-value colour). Two consecutive rings at one fraction are a hard edge. */
export type ContourRing = Readonly<{ fraction: number; kind: "value" | "below" | "gap"; position: number }>;

/** The finest ring spacing on the scale: a piece of the tube spans at most this part of the
 * scale, so its two end colours are close and the drawn blend between them follows the scale. */
export const CONTOUR_PIECES_PER_SCALE = 12;
/** The most pieces one interval between stations is drawn with. */
export const CONTOUR_MAX_PIECES_PER_INTERVAL = 12;

function valueRing(fraction: number, value: number, maximum: number, threshold: number | null): ContourRing {
  return aboveThreshold(value, threshold)
    ? { fraction, kind: "value", position: maximum > 0 ? value / maximum : 0 }
    : { fraction, kind: "below", position: 0 };
}

/**
 * The rings a member's contour tube is drawn with, from its first stated station to its last.
 * Each interval between stated stations with two colourable values is cut into pieces whose
 * values come from `interpolateStations`, finely enough that each piece spans at most
 * 1 / `CONTOUR_PIECES_PER_SCALE` of the scale. A threshold crossing inside an interval is placed
 * exactly, as a hard edge. An interval beside a station with no colourable value is one gap piece.
 */
export function contourRings(
  stations: readonly StationPoint[],
  options: { maximum: number; threshold?: number | null }
): ContourRing[] {
  const threshold = options.threshold ?? null;
  const maximum = options.maximum;
  if (stations.length < 2) return [];
  const rings: ContourRing[] = [];
  const push = (ring: ContourRing) => {
    const previous = rings[rings.length - 1];
    if (previous && previous.fraction === ring.fraction && previous.kind === ring.kind && previous.position === ring.position) return;
    rings.push(Object.freeze(ring));
  };
  for (let index = 0; index + 1 < stations.length; index += 1) {
    const from = stations[index];
    const to = stations[index + 1];
    if (!colourable(from.value) || !colourable(to.value)) {
      push({ fraction: from.fraction, kind: "gap", position: 0 });
      push({ fraction: to.fraction, kind: "gap", position: 0 });
      continue;
    }
    const span = maximum > 0 ? Math.abs(to.value - from.value) / maximum : 0;
    const pieces = Math.min(CONTOUR_MAX_PIECES_PER_INTERVAL, Math.max(1, Math.ceil(span * CONTOUR_PIECES_PER_SCALE)));
    const fractions = Array.from({ length: pieces + 1 }, (_, piece) =>
      piece === pieces ? to.fraction : from.fraction + (to.fraction - from.fraction) * (piece / pieces));
    let crossing: number | null = null;
    if (threshold !== null && (from.value - threshold) * (to.value - threshold) < 0) {
      crossing = from.fraction + (to.fraction - from.fraction) * ((threshold - from.value) / (to.value - from.value));
    }
    for (const fraction of fractions) {
      if (crossing !== null && fraction >= crossing) {
        // The two sides of the crossing, at the crossing itself, in the direction of travel.
        const rising = to.value > from.value;
        const below: ContourRing = { fraction: crossing, kind: "below", position: 0 };
        const above: ContourRing = { fraction: crossing, kind: "value", position: maximum > 0 ? threshold! / maximum : 0 };
        push(rising ? below : above);
        push(rising ? above : below);
        const atCrossing = fraction === crossing;
        crossing = null;
        if (atCrossing) continue;
      }
      const value = interpolateStations(stations, fraction);
      push(value === null ? { fraction, kind: "gap", position: 0 } : valueRing(fraction, value, maximum, threshold));
    }
  }
  return rings;
}

/** A number the numbers display shows: a stated value at a stated station, or a member's one value
 * (`oneValue`), drawn at its midpoint. Never an interpolated value. */
export type ContourValueLabel = Readonly<{ memberId: string; fraction: number; value: number; rowId: string; oneValue: boolean }>;

/** The most numbers drawn at once: the largest stated values first. */
export const CONTOUR_VALUE_LABEL_BUDGET = 60;

/**
 * The stated values the numbers display can label, largest first, at most `budget` of them, and
 * how many there were. Under a threshold only values above it are labelled. Only stated values
 * are labelled: a station's own value, or a member's one value.
 */
export function contourValueLabels(
  overlay: ResultColourOverlay,
  options: { threshold?: number | null; budget?: number } = {}
): { labels: ContourValueLabel[]; candidateCount: number } {
  if (overlay.state !== "available") return { labels: [], candidateCount: 0 };
  const candidates: ContourValueLabel[] = [];
  for (const [memberId, member] of overlay.members) {
    if (member.stations.length >= 2) {
      for (const station of member.stations) {
        if (!colourable(station.value) || !aboveThreshold(station.value, options.threshold)) continue;
        candidates.push(Object.freeze({ memberId, fraction: station.fraction, value: station.value, rowId: station.rowIds[0] ?? "", oneValue: false }));
      }
    } else if (aboveThreshold(member.value, options.threshold)) {
      candidates.push(Object.freeze({ memberId, fraction: 0.5, value: member.value, rowId: member.rowId, oneValue: true }));
    }
  }
  candidates.sort((a, b) => b.value - a.value || a.memberId.localeCompare(b.memberId) || a.fraction - b.fraction);
  return { labels: candidates.slice(0, options.budget ?? CONTOUR_VALUE_LABEL_BUDGET), candidateCount: candidates.length };
}

const NUMBER_TEXT = /^[+-]?(?:\d+(?:\.\d*)?|\.\d+)(?:[eE][+-]?\d+)?$/;

/** The threshold the user typed, in the overlay's stored unit, or null when the text is empty or
 * not a finite number. The text is in the legend's display unit: `displayPerStored` is how many
 * display units one stored unit is, from the display-units path (1 when the legend shows the
 * stored unit). Every overlay dimension converts by a factor alone. */
export function thresholdInOverlayUnit(text: string, displayPerStored: number): number | null {
  const trimmed = text.trim();
  if (!NUMBER_TEXT.test(trimmed) || !Number.isFinite(displayPerStored) || displayPerStored <= 0) return null;
  const value = Number(trimmed) / displayPerStored;
  return Number.isFinite(value) ? value : null;
}
