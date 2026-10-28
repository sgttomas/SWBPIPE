// The contour's numbers display (survey item A4; DEL-07-05 Design `result-contour.md`): the stated
// values drawn on the canvas in place of colours, at their stations, in the legend's display unit.
// At most `CONTOUR_VALUE_LABEL_BUDGET` numbers, largest first; a number that would overlap one
// already placed, or that is outside the view, is not drawn. Placed after each main render through
// the resource's overlay label slot, so orbiting moves them with the figure. Display only.

import { useEffect, useRef, type MutableRefObject } from "react";
import * as THREE from "three";
import type { Vec3 } from "../../types";
import { useDisplayQuantity } from "../display-units";
import { formatLegendNumber } from "./ResultOverlayLegend";
import type { ResultOverlayDimension } from "./resultColourOverlay";
import type { ContourValueLabel } from "./resultContour";
import type { ViewportResource } from "./viewportResource";

export type ResultValueLabelAnchor = Readonly<{ label: ContourValueLabel; position: Readonly<Vec3> }>;

/** Where each label's anchor is in the drawing: along its member from the From node, in the
 * drawing's local coordinates. A label whose member has no drawable axis is left out. */
export function resultValueLabelAnchors(
  labels: readonly ContourValueLabel[],
  members: ReadonlyMap<string, Readonly<{ from: Readonly<Vec3>; to: Readonly<Vec3> }>>
): ResultValueLabelAnchor[] {
  return labels.flatMap((label) => {
    const member = members.get(label.memberId);
    if (!member) return [];
    const { from, to } = member;
    const t = label.fraction;
    return [{ label, position: { x: from.x + (to.x - from.x) * t, y: from.y + (to.y - from.y) * t, z: from.z + (to.z - from.z) * t } }];
  });
}

function ValueText({ value, unit, dimension }: { value: number; unit: string; dimension: ResultOverlayDimension }) {
  const display = useDisplayQuantity({ value, unit, dimension_id: dimension });
  if (dimension === "ratio") return <>{formatLegendNumber(value)}</>;
  return <>{formatLegendNumber(display.value)} {display.unit}</>;
}

export function ResultValueLabels({
  anchors,
  unit,
  dimension,
  resourceRef
}: {
  anchors: readonly ResultValueLabelAnchor[];
  unit: string;
  dimension: ResultOverlayDimension;
  resourceRef: MutableRefObject<ViewportResource | null>;
}) {
  const layerRef = useRef<HTMLDivElement | null>(null);
  useEffect(() => {
    const resource = resourceRef.current;
    const layer = layerRef.current;
    if (!resource || !layer) return;
    const points = anchors.map((anchor) => new THREE.Vector3(anchor.position.x, anchor.position.y, anchor.position.z));
    const projected = new THREE.Vector3();
    const place = () => {
      const width = Math.max(1, layer.clientWidth || resource.renderer.domElement.clientWidth);
      const height = Math.max(1, layer.clientHeight || resource.renderer.domElement.clientHeight);
      const occupied: Array<{ left: number; right: number; top: number; bottom: number }> = [];
      const items = layer.querySelectorAll<HTMLElement>("[data-value-label-index]");
      items.forEach((item) => {
        const point = points[Number(item.dataset.valueLabelIndex)];
        if (!point) return;
        projected.copy(point).project(resource.camera);
        if (projected.x < -1 || projected.x > 1 || projected.y < -1 || projected.y > 1 || projected.z < -1 || projected.z > 1) {
          item.style.display = "none";
          return;
        }
        const x = (projected.x * 0.5 + 0.5) * width;
        const y = (-projected.y * 0.5 + 0.5) * height;
        const halfWidth = Math.max(24, item.offsetWidth / 2);
        const box = { left: x - halfWidth, right: x + halfWidth, top: y - 9, bottom: y + 9 };
        const overlaps = occupied.some((placed) => box.left < placed.right && box.right > placed.left && box.top < placed.bottom && box.bottom > placed.top);
        item.style.display = overlaps ? "none" : "";
        if (overlaps) return;
        occupied.push(box);
        item.style.left = `${x}px`;
        item.style.top = `${y}px`;
      });
    };
    resource.setOverlayLabelUpdater(place);
    place();
    resource.invalidate();
    return () => resource.setOverlayLabelUpdater(null);
  }, [anchors, resourceRef]);
  return (
    <div className="viewport-value-labels" aria-label="Result values on the canvas" data-testid="viewport-result-values" ref={layerRef}>
      {anchors.map((anchor, index) => (
        <span
          key={`${anchor.label.memberId}:${anchor.label.fraction}`}
          className="viewport-value-label"
          data-value-label-index={index}
          data-testid={`viewport-result-value-${index}`}
          title={`${anchor.label.memberId}${anchor.label.oneValue ? " (one value for the member)" : ` at ${anchor.label.fraction} of its length`}: ${anchor.label.value} ${unit} (${anchor.label.rowId})`}
        >
          <ValueText value={anchor.label.value} unit={unit} dimension={dimension} />
        </span>
      ))}
    </div>
  );
}
