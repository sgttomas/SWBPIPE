import { cleanup, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import type { PreviewModel } from "../../types";
import { changeFormControl } from "../../test-support/workspaceTestControls";
import { SupportConfigurationForm } from "./SupportConfigurationForm";

vi.mock("../../services/hashService", () => ({ canonicalJsonString: vi.fn(async (value: unknown) => JSON.stringify(value)) }));
afterEach(cleanup);

/** Invented model: every entity and value is synthetic test input. */
function model(): PreviewModel {
  return {
    schema_version: "test", document_kind: "test", data_boundary: {},
    project: { id: "project:test", name: "Invented", description: "Synthetic test data", units: {} },
    analysis_status: { mechanics: "not_run", rule_check: "not_run", professional_acceptance: "not_run" },
    nodes: [{ id: "node:a", label: "A", provenance: "invented", position: { x: 0, y: 0, z: 0 } }],
    supports: [{ id: "support:a", label: "Invented hanger", node: "node:a", restraints: ["UY"], provenance: "invented" }],
    materials: [], pipe_segments: [], components: [], diagnostics: [], load_cases: []
  } as unknown as PreviewModel;
}

/** The named controls inside a container, in document (and so Tab) order. */
const controlNames = (container: HTMLElement) =>
  Array.from(container.querySelectorAll<HTMLElement>('input[aria-label], [role="combobox"][aria-label]')).map((control) => control.getAttribute("aria-label"));

const quantity = (name: string) => [`${name} value`, `${name} unit`];

describe("support configuration grouping", () => {
  it("groups the hanger's fields under titled runs, keeping their order and names", () => {
    render(<SupportConfigurationForm model={model()} selection={{ type: "support", id: "support:a" }} onQueueIntent={vi.fn()} />);
    fireEvent.click(screen.getByLabelText("Include hanger data"));
    const hanger = screen.getByRole("group", { name: "Hanger" });
    expect(controlNames(hanger)).toEqual([
      "Hanger type",
      ...quantity("Hanger installed load"), ...quantity("Hanger cold load"), ...quantity("Hanger hot load"), ...quantity("Hanger constant load"),
      ...quantity("Hanger travel range"), ...quantity("Hanger movement limit"),
      "Hanger manufacturer reference", "Hanger source reference", "Hanger load side review reference", "Hanger mechanics consumption"
    ]);
    expect(controlNames(within(hanger).getByRole("group", { name: "Loads" }))).toEqual([
      ...quantity("Hanger installed load"), ...quantity("Hanger cold load"), ...quantity("Hanger hot load"), ...quantity("Hanger constant load")
    ]);
    expect(controlNames(within(hanger).getByRole("group", { name: "Travel" }))).toEqual([
      ...quantity("Hanger travel range"), ...quantity("Hanger movement limit")
    ]);
    expect(controlNames(within(hanger).getByRole("group", { name: "Sources and references" }))).toEqual([
      "Hanger manufacturer reference", "Hanger source reference", "Hanger load side review reference", "Hanger mechanics consumption"
    ]);
    // Each quantity still states its expected dimension.
    expect(within(hanger).getByRole("group", { name: "Hanger travel range" })).toHaveTextContent("Expected dimension: length.");
    expect(within(hanger).getByRole("group", { name: "Hanger cold load" })).toHaveTextContent("Expected dimension: force.");
  });

  it("groups nonlinear behavior's selections apart from its quantities, keeping their order and names", () => {
    render(<SupportConfigurationForm model={model()} selection={{ type: "support", id: "support:a" }} onQueueIntent={vi.fn()} />);
    fireEvent.click(screen.getByLabelText("Include nonlinear behavior"));
    const nonlinear = screen.getByRole("group", { name: "Nonlinear behavior" });
    expect(controlNames(within(nonlinear).getByRole("group", { name: "Behavior and state" }))).toEqual([
      "Nonlinear behavior", "Nonlinear DOF", "Initial state", "active when", "contact when", "closes when"
    ]);
    expect(controlNames(within(nonlinear).getByRole("group", { name: "Gap, friction and normal reaction" }))).toEqual([
      ...quantity("gap"), ...quantity("friction coefficient"), ...quantity("normal reaction")
    ]);
    expect(within(nonlinear).getByLabelText("Derive normal reaction from a support")).toBeInTheDocument();
  });

  it("keeps the six restraints in one fieldset and queues the same payload through the grouped fields", async () => {
    const queue = vi.fn();
    render(<SupportConfigurationForm model={model()} selection={{ type: "support", id: "support:a" }} onQueueIntent={queue} />);
    expect(controlNames(screen.getByRole("group", { name: "Restrained degrees of freedom" }))).toEqual(
      ["UX", "UY", "UZ", "RX", "RY", "RZ"].map((dof) => `Restrain ${dof}`)
    );
    fireEvent.click(screen.getByLabelText("Include hanger data"));
    changeFormControl(screen.getByLabelText("Hanger cold load value"), { target: { value: "12.5" } });
    changeFormControl(screen.getByLabelText("Hanger cold load unit"), { target: { value: "kN" } });
    changeFormControl(screen.getByLabelText("Hanger movement limit value"), { target: { value: "25" } });
    changeFormControl(screen.getByLabelText("Hanger movement limit unit"), { target: { value: "mm" } });
    changeFormControl(screen.getByLabelText("Hanger source reference"), { target: { value: "invented catalogue" } });
    fireEvent.click(screen.getByText("Queue support configuration"));
    await waitFor(() => expect(queue).toHaveBeenCalledOnce());
    expect(JSON.parse(queue.mock.calls[0][0].change.after)).toEqual({
      restraints: ["UY"],
      provenance: "invented",
      hanger: { cold_load: { value: 12.5, unit: "kN" }, movement_limit: { value: 25, unit: "mm" }, source_reference: "invented catalogue" }
    });
  });
});
