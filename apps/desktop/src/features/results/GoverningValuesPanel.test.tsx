// The governing values at the head of the Results stage: each value is a control naming its entity,
// its case and its unit; activating it asks for its row. Invented data.
import { cleanup, fireEvent, render, screen } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { GoverningValuesPanel } from "./GoverningValuesPanel";
import { GOVERNING_REACTION_COLUMNS, type GoverningValues } from "./governingValues";

afterEach(cleanup);

const values: GoverningValues = {
  displacement: { value: -5.123456789, unit: "mm", dimension: "displacement", entityLabel: "Riser elbow", caseKey: "load_case:L-1", caseLabel: "Load case L-1", component: "uy", nodeId: "node:B" },
  reactions: GOVERNING_REACTION_COLUMNS.map((column) => column === "Fx"
    ? { value: -300, unit: "N", dimension: "force", entityLabel: "Anchor A", caseKey: "load_case:L-2", caseLabel: "Load case L-2", component: "Fx", supportId: "support:S-A", column }
    : { absent: "None admitted", column }),
  stresses: { absent: "None admitted" },
  ratios: { absent: "No rule pack" }
};

describe("the governing values panel", () => {
  it("shows each value with its unit, entity and case on one control, at the report's display precision", () => {
    const onOpenDisplacement = vi.fn();
    render(<GoverningValuesPanel values={values} onOpenDisplacement={onOpenDisplacement} onOpenReaction={vi.fn()} onOpenStress={vi.fn()} onOpenWeight={vi.fn()} />);
    const displacement = screen.getByTestId("governing-displacement-value");
    expect(displacement.tagName).toBe("BUTTON");
    expect(displacement.textContent).toContain("-5.12346 mm");
    expect(displacement.textContent).toContain("Riser elbow");
    expect(displacement.textContent).toContain("Load case L-1");
    fireEvent.click(displacement);
    expect(onOpenDisplacement).toHaveBeenCalledWith(values.displacement);
    expect(screen.getByTestId("governing-reaction-Fx").textContent).toContain("-300 N");
    expect(screen.getByTestId("governing-ratio-absent").textContent).toBe("No rule pack");
    expect(screen.getByTestId("governing-reaction-Mz-absent").textContent).toBe("None admitted");
  });
});
