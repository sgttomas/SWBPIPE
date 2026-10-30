// RT-S1/RT-U1: the "New support" form's family. All six rigid DOFs, with no stiffness, pre-select
// "anchor" visibly as a suggestion the user can change; the family is written only by the user's
// queue; a DOF unchecked updates or clears the suggestion; an explicit pick is never replaced by a
// later checkbox change. All entities are invented test input.
import { cleanup, fireEvent, render, screen } from "@testing-library/react";
import type { ComponentProps } from "react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { buildBlankLocalModelDocument } from "../../services/projectService";
import { changeFormControl, compactOptionRecords, expectFormControlValue } from "../../test-support/workspaceTestControls";
import type { EditorOperationIntent, PreviewModel } from "../../types";
import { DisplayUnitsProvider } from "../display-units";
import { SUPPORT_FAMILIES } from "../results/displacementExtremes";
import { PropertyInspector } from "./PropertyInspector";

afterEach(cleanup);

function inventedModel(): PreviewModel {
  const model = buildBlankLocalModelDocument(new Date("2026-01-01T00:00:00Z"));
  model.project.units = { length: "mm", force: "N", pressure: "MPa", temperature: "C", angle: "rad" };
  model.nodes = [{ id: "node:A", label: "Node A", position: { x: 0, y: 0, z: 0 }, provenance: "invented node" }];
  model.supports = [];
  return model;
}

function mount(model = inventedModel()) {
  const queue = vi.fn<(intent: EditorOperationIntent) => void>();
  const props: ComponentProps<typeof PropertyInspector> = { model, selection: { type: "node", id: "node:A" }, onQueueIntent: queue };
  render(<DisplayUnitsProvider initialPreference="SI"><PropertyInspector {...props} /></DisplayUnitsProvider>);
  return { model, queue };
}

const byId = (testId: string) => screen.getByTestId(testId);
const family = () => byId("create-support-family");
const suggestion = () => screen.queryByTestId("create-support-family-suggestion");
const toggle = (...dofs: string[]) => dofs.forEach((dof) => fireEvent.click(byId(`create-support-restraint-${dof}`)));
/** Queue the form and return the support it would create. */
function queued(queue: ReturnType<typeof mount>["queue"]) {
  fireEvent.click(byId("queue-create-support-intent"));
  return JSON.parse(queue.mock.calls.at(-1)![0].change.after) as Record<string, unknown>;
}

describe("the New support form's family", () => {
  it("offers no family, then the whole shared SupportFamily list, disabling with a visible reason each family it cannot supply", () => {
    mount();
    const hanger = "Needs hanger data: use Support configuration";
    expect(compactOptionRecords(family())).toEqual([
      { value: "", label: "Not stated", disabled: false },
      { value: "anchor", label: "Anchor", disabled: false },
      { value: "guide", label: "Guide", disabled: false },
      { value: "line_stop", label: "Line stop", disabled: false },
      { value: "vertical_support", label: "Vertical support", disabled: false },
      { value: "spring", label: "Spring (Needs stiffness with its DOF: use Support configuration)", disabled: true },
      { value: "variable_spring_hanger", label: `Variable spring hanger (${hanger})`, disabled: true },
      { value: "spring_hanger", label: `Spring hanger (${hanger})`, disabled: true },
      { value: "constant_effort_support", label: `Constant-effort support (${hanger})`, disabled: true },
      { value: "nonlinear", label: "Nonlinear support (Needs a nonlinear law: use Support configuration)", disabled: true }
    ]);
    expect(compactOptionRecords(family()).slice(1).map((item) => item.value)).toEqual(SUPPORT_FAMILIES.map((item) => item.value));
  });

  it("does not take a disabled family, so none is queued", () => {
    const { queue } = mount();
    toggle("RX", "RY", "RZ");
    const control = family();
    if (control.getAttribute("aria-expanded") !== "true") fireEvent.click(control);
    const list = document.getElementById(control.getAttribute("aria-controls")!)!;
    const option = Array.from(list.querySelectorAll<HTMLElement>('[role="option"]')).find((item) => item.dataset.value === "spring_hanger")!;
    expect(option).toHaveAttribute("aria-disabled", "true");
    fireEvent.click(option);
    expectFormControlValue(family(), "anchor");
    expect(queued(queue).family).toBe("anchor");
  });

  it("suggests anchor, visibly, when all six rigid DOFs are checked, and writes it only on queue", () => {
    const { model, queue } = mount();
    const before = JSON.stringify(model);
    expect(suggestion()).toBeNull();
    expectFormControlValue(family(), "");
    toggle("RX", "RY", "RZ");
    expect(suggestion()).toHaveTextContent("Suggested from restraints: anchor; you can change it");
    expectFormControlValue(family(), "anchor");
    expect(queue).not.toHaveBeenCalled();
    expect(JSON.stringify(model)).toBe(before);
    expect(queued(queue)).toMatchObject({ restraints: ["UX", "UY", "UZ", "RX", "RY", "RZ"], family: "anchor" });
    expect(JSON.stringify(model)).toBe(before);
  });

  it("writes the family the user changes the suggestion to", () => {
    const { queue } = mount();
    toggle("RX", "RY", "RZ");
    changeFormControl(family(), { target: { value: "guide" } });
    expectFormControlValue(family(), "guide");
    expect(suggestion()).toHaveTextContent(/^Suggested from restraints: anchor$/);
    expect(queued(queue).family).toBe("guide");
  });

  it("clears the suggestion when a DOF is unchecked or a stiffness is entered, and writes no family", () => {
    const { queue } = mount();
    toggle("RX", "RY", "RZ");
    expectFormControlValue(family(), "anchor");
    toggle("RZ");
    expect(suggestion()).toBeNull();
    expectFormControlValue(family(), "");
    expect(queued(queue)).not.toHaveProperty("family");
    toggle("RX", "RY", "RZ");
    expectFormControlValue(family(), "anchor");
    fireEvent.change(byId("create-support-stiffness"), { target: { value: "1500" } });
    expect(suggestion()).toBeNull();
    expect(queued(queue)).not.toHaveProperty("family");
  });

  it("keeps an explicit pick through later checkbox changes, showing the suggestion beside it", () => {
    const { queue } = mount();
    changeFormControl(family(), { target: { value: "line_stop" } });
    toggle("RX", "RY", "RZ");
    expectFormControlValue(family(), "line_stop");
    expect(suggestion()).toHaveTextContent(/^Suggested from restraints: anchor$/);
    toggle("UY");
    expectFormControlValue(family(), "line_stop");
    expect(suggestion()).toBeNull();
    expect(queued(queue)).toMatchObject({ restraints: ["UX", "UZ", "RX", "RY", "RZ"], family: "line_stop" });
  });

  it("keeps an explicit Not stated over the anchor suggestion", () => {
    const { queue } = mount();
    toggle("RX", "RY", "RZ");
    changeFormControl(family(), { target: { value: "" } });
    toggle("RZ"); toggle("RZ");
    expectFormControlValue(family(), "");
    expect(queued(queue)).not.toHaveProperty("family");
  });

  it("starts a fresh suggestion after a queue", () => {
    const { queue } = mount();
    changeFormControl(family(), { target: { value: "guide" } });
    queued(queue);
    toggle("RX", "RY", "RZ");
    expectFormControlValue(family(), "anchor");
    expect(suggestion()).toHaveTextContent("; you can change it");
  });
});
