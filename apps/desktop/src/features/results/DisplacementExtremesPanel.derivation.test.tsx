// A8 render cost (DEL-07-05 Design `displacement-extremes.md` §7): the panel derives once per
// (model, result, standing) identity. Control changes (filter, sort, cases, page), the selected node
// and the test-id prefix reuse it. Invented data only (`resultOverlayFixtures`).
import { cleanup, fireEvent, render, screen } from "@testing-library/react";
import { afterEach, expect, it, vi } from "vitest";
const derivations = vi.hoisted(() => ({ count: 0 }));
vi.mock("./displacementExtremes", async (original) => {
  const actual = await original<typeof import("./displacementExtremes")>();
  return {
    ...actual,
    buildDisplacementExtremes: (input: Parameters<typeof actual.buildDisplacementExtremes>[0]) => {
      derivations.count += 1;
      return actual.buildDisplacementExtremes(input);
    }
  };
});
import { healthStanding, overlayModel, overlayResult, row } from "../../test-support/resultOverlayFixtures";
import { DisplacementExtremesPanel } from "./DisplacementExtremesPanel";

afterEach(cleanup);

it("derives once per input identity and not on control changes", () => {
  derivations.count = 0;
  const model = overlayModel();
  const rows = () => [
    row("ux", "global_nodal_displacement_x", "node:A", 1, "mm"),
    row("uy", "global_nodal_displacement_y", "node:B", 2, "mm", { basis_ref: { ref_type: "load_case", ref_id: "load:L-2" } })
  ];
  const result = overlayResult(rows());
  const standing = healthStanding({ eligible: true });
  const view = render(<DisplacementExtremesPanel model={model} result={result} standing={standing} />);
  expect(derivations.count).toBe(1);
  fireEvent.change(screen.getByTestId("displacement-extremes-filter"), { target: { value: "supports" } });
  fireEvent.change(screen.getByTestId("displacement-extremes-sort"), { target: { value: "uy" } });
  fireEvent.click(screen.getByTestId("displacement-extremes-case-load_case:load:L-2"));
  view.rerender(<DisplacementExtremesPanel model={model} result={result} standing={standing} selectedNodeId="node:A" />);
  view.rerender(<DisplacementExtremesPanel model={model} result={result} standing={standing} selectedNodeId="node:A" testIdPrefix="displacement-extremes" />);
  expect(screen.getByTestId("displacement-extremes-row-node:A").getAttribute("aria-current")).toBe("true");
  expect(derivations.count).toBe(1);
  view.rerender(<DisplacementExtremesPanel model={model} result={overlayResult(rows())} standing={standing} />);
  expect(derivations.count).toBe(2);
  view.rerender(<DisplacementExtremesPanel model={overlayModel()} result={result} standing={standing} />);
  expect(derivations.count).toBe(3);
  view.rerender(<DisplacementExtremesPanel model={overlayModel()} result={result} standing={healthStanding({ eligible: true })} />);
  expect(derivations.count).toBe(4);
});
