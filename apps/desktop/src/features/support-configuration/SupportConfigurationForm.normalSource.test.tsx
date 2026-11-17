import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import type { PreviewModel } from "../../types";
import { normalSourceDofs, SupportConfigurationForm } from "./SupportConfigurationForm";

vi.mock("../../services/hashService", () => ({ canonicalJsonString: vi.fn(async (value: unknown) => JSON.stringify(value)) }));
afterEach(cleanup);

const support = (id: string, restraints: string[], nonlinear?: Record<string, unknown>) =>
  ({ id, label: id, node: "node:a", restraints, provenance: "invented", ...(nonlinear ? { nonlinear } : {}) });

/** Invented model: a friction support whose normal is derived from `source`. */
function model(source: ReturnType<typeof support>, dof: string): PreviewModel {
  return {
    schema_version: "test", document_kind: "test", data_boundary: {},
    project: { id: "project:test", name: "Invented", description: "Synthetic test data", units: {} },
    analysis_status: { mechanics: "not_run", rule_check: "not_run", professional_acceptance: "not_run" },
    nodes: [{ id: "node:a", label: "A", provenance: "invented", position: { x: 0, y: 0, z: 0 } }],
    supports: [source, support("support:f", [], {
      behavior: "friction", dof: "UX", initial_state: "sticking", friction_coefficient: { value: 0.3, unit: "1" },
      normal_reaction_source: { support_ref: source.id, dof }
    })],
    materials: [], pipe_segments: [], components: [], diagnostics: [], load_cases: []
  } as unknown as PreviewModel;
}

describe("friction normal source (DEL-04-04 REQ-012)", () => {
  it("admits a linear support's restrained translational DOFs, or a one-way or lift-off support's own translational DOF", () => {
    expect(normalSourceDofs(support("s", ["UY", "RX", "UZ"]))).toEqual(["UY", "UZ"]);
    expect(normalSourceDofs(support("s", [], { behavior: "one_way", dof: "UZ" }))).toEqual(["UZ"]);
    expect(normalSourceDofs(support("s", [], { behavior: "lift_off", dof: "UY" }))).toEqual(["UY"]);
    expect(normalSourceDofs(support("s", [], { behavior: "one_way", dof: "RZ" }))).toEqual([]);
    expect(normalSourceDofs(support("s", [], { behavior: "gap", dof: "UZ" }))).toEqual([]);
    expect(normalSourceDofs(support("s", [], { behavior: "friction", dof: "UZ" }))).toEqual([]);
  });

  it("says what is admitted and queues a one-way source on its own DOF", async () => {
    const queue = vi.fn();
    render(<SupportConfigurationForm model={model(support("support:w", [], { behavior: "one_way", dof: "UZ", initial_state: "active", active_when: "positive_reaction" }), "UZ")}
      selection={{ type: "support", id: "support:f" }} onQueueIntent={queue} />);
    expect(screen.getByText(/Choose a linear support’s restrained translational DOF, or a one-way or lift-off support’s own translational DOF\./)).toBeInTheDocument();
    fireEvent.click(screen.getByText("Queue support configuration"));
    await waitFor(() => expect(queue).toHaveBeenCalledOnce());
    expect(JSON.parse(queue.mock.calls[0][0].change.after).nonlinear.normal_reaction_source).toEqual({ support_ref: "support:w", dof: "UZ" });
  });

  it("refuses a gap support as the source", async () => {
    const queue = vi.fn();
    render(<SupportConfigurationForm model={model(support("support:g", [], { behavior: "gap", dof: "UZ", initial_state: "inactive" }), "UZ")}
      selection={{ type: "support", id: "support:f" }} onQueueIntent={queue} />);
    fireEvent.click(screen.getByText("Queue support configuration"));
    expect(await screen.findByText(/Select a linear support and one of its restrained translational DOFs, or a one-way or lift-off support and its own translational DOF\./)).toBeInTheDocument();
    expect(queue).not.toHaveBeenCalled();
  });
});
