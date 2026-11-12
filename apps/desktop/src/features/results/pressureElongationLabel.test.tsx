// RT-L2 (DEL-04-07): each load case's result
// header states "Pressure elongation included" or "Pressure elongation not included" from the
// case's pressure evidence; a combination states nothing of its own.
import { render, screen } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import type { NavigatorCase } from "./resultsNavigator";
import { ResultsNavigator } from "./ResultsPanel";
import {
  PRESSURE_ELONGATION_INCLUDED_LABEL,
  PRESSURE_ELONGATION_NOT_INCLUDED_LABEL,
  pressureElongationLabel
} from "./pressureElongationLabel";

const region = (caseId: string, regionId: string, value: unknown) =>
  ({ load_case_id: caseId, region_id: regionId, pressure_elongation: value });
const evidence = {
  pressure: [
    region("case:operating", "region:a", "included"),
    region("case:operating", "region:b", "included"),
    region("case:sustained", "region:a", "not_included")
  ]
};
const loadCase = (id: string) => ({ ref_type: "load_case", ref_id: id }) as const;

describe("pressureElongationLabel", () => {
  it("states each load case's setting from its pressure regions", () => {
    expect(pressureElongationLabel(evidence, loadCase("case:operating"))).toBe("Pressure elongation included");
    expect(pressureElongationLabel(evidence, loadCase("case:sustained"))).toBe("Pressure elongation not included");
    expect(PRESSURE_ELONGATION_INCLUDED_LABEL).toBe("Pressure elongation included");
    expect(PRESSURE_ELONGATION_NOT_INCLUDED_LABEL).toBe("Pressure elongation not included");
  });

  it("states nothing for a combination, a pressure-free case, no basis or no evidence", () => {
    expect(pressureElongationLabel(evidence, { ref_type: "combination", ref_id: "case:operating" })).toBeNull();
    expect(pressureElongationLabel(evidence, loadCase("case:weight"))).toBeNull();
    expect(pressureElongationLabel(evidence, null)).toBeNull();
    expect(pressureElongationLabel(undefined, loadCase("case:operating"))).toBeNull();
    expect(pressureElongationLabel({ pressure: "not a list" }, loadCase("case:operating"))).toBeNull();
  });

  it("states nothing when the regions disagree or state no known value", () => {
    const mixed = { pressure: [region("case:x", "a", "included"), region("case:x", "b", "not_included")] };
    expect(pressureElongationLabel(mixed, loadCase("case:x"))).toBeNull();
    for (const value of [undefined, "on", true, "constructor"]) {
      expect(pressureElongationLabel({ pressure: [region("case:x", "a", value)] }, loadCase("case:x"))).toBeNull();
    }
  });
});

describe("the results navigator's case header", () => {
  const cases: NavigatorCase[] = [
    { key: "load_case:case:operating", label: "Load case case:operating", basis: loadCase("case:operating"), envelope: false },
    { key: "load_case:case:sustained", label: "Load case case:sustained", basis: loadCase("case:sustained"), envelope: false },
    { key: "combination:case:operating", label: "Combination case:operating", basis: { ref_type: "combination", ref_id: "case:operating" }, envelope: false }
  ];
  const show = (caseKey: string) => render(
    <ResultsNavigator
      kinds={["all"]} kind="all" cases={cases} caseKey={caseKey} entities={[]} entity={null} currentRowId={null} rowCount={1}
      selectionOnlyWithoutEntity={false} health={null} endNotice={null} canStep={() => false} onStep={() => {}} onChooseCase={() => {}}
      contractEvidence={evidence}
    />
  );

  it.each([
    ["load_case:case:operating", "Pressure elongation included"],
    ["load_case:case:sustained", "Pressure elongation not included"]
  ])("shows the chosen load case's label (%s)", (caseKey, label) => {
    show(caseKey);
    expect(screen.getByTestId("results-navigator-case-pressure-elongation")).toHaveTextContent(label);
  });

  it.each(["combination:case:operating", "all"])("shows none for a combination or every case (%s)", (caseKey) => {
    show(caseKey);
    expect(screen.queryByTestId("results-navigator-case-pressure-elongation")).toBeNull();
  });
});
