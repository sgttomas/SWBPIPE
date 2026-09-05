import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { MaterialTemperatureForm } from "../material-temperature/MaterialTemperatureForm";
import { WindExposureForm } from "../wind-exposure/WindExposureForm";
import { SupportConfigurationForm } from "../support-configuration/SupportConfigurationForm";
import type { PreviewModel, EditorOperationIntent } from "../../types";
import { canonicalJsonString } from "../../services/hashService";
vi.mock("../../services/hashService", () => ({ canonicalJsonString: vi.fn(async (value: unknown) => JSON.stringify(value)) }));
afterEach(() => {
  cleanup();
  vi.mocked(canonicalJsonString).mockReset();
  vi.mocked(canonicalJsonString).mockImplementation(async (value) => JSON.stringify(value));
});
function model(): PreviewModel {
  return {
    schema_version: "test",
    document_kind: "test",
    data_boundary: {},
    project: {
      id: "project:test",
      name: "Invented",
      description: "Original synthetic test data",
      units: {}
    },
    analysis_status: {
      mechanics: "not_run",
      rule_check: "not_run",
      professional_acceptance: "not_run"
    },
    nodes: [{
      id: "node:a",
      label: "A",
      provenance: "invented",
      position: {
        x: 0,
        y: 0,
        z: 0
      }
    }],
    supports: [{
      id: "support:a",
      label: "A",
      node: "node:a",
      restraints: ["UY"],
      provenance: "invented"
    }],
    materials: [{
      id: "material:a",
      label: "A",
      elastic_modulus: { value: 2, unit: "MPa" },
      shear_modulus: { value: 1, unit: "MPa" },
      provenance: "invented"
    }],
    pipe_segments: [{
      id: "pipe:a",
      label: "A",
      from: "node:a",
      to: "node:b",
      material: "material:a",
      section: {},
      provenance: "invented"
    }],
    components: [],
    diagnostics: [],
    load_cases: [{
      id: "load:a",
      label: "Wind",
      kind: "wind",
      status: "draft",
      provenance: "invented",
      equivalent_static: { wind: { exposed_pipe_refs: ["pipe:a"] } }
    }]
  };
}
const enter = (label: string, value: string) => fireEvent.change(screen.getByLabelText(label), { target: { value } });
function quantity(label: string, value: string, unit: string) {
  enter(`${label} value`, value);
  enter(`${label} unit`, unit);
}
describe("rich authoring structured forms", () => {
  it("queues a partial material row with exact entered quantities and omitted unentered slots", async () => {
    const m = model(), original = JSON.stringify(m), queued: EditorOperationIntent[] = [];
    render(<MaterialTemperatureForm
      model={m}
      selection={{ type: "material", id: "material:a" }}
      onQueueIntent={i => queued.push(i)}
    />);
    fireEvent.click(screen.getByText("Add temperature point"));
    enter("Point 1 ID", "temperature:hot");
    quantity("Point 1 Temperature", "275", "degC");
    quantity("Point 1 Shear modulus", "123", "MPa");
    enter("Point 1 provenance", "user entered");
    fireEvent.click(screen.getByText("Queue temperature points"));
    await waitFor(() => expect(queued).toHaveLength(1));
    expect(queued[0].change).toMatchObject({
      change_kind: "set_field",
      field_path: "temperature_points",
      before: "not_present",
      unit: "none",
      dimension: "dimensionless"
    });
    expect(JSON.parse(queued[0].change.after)).toEqual([{
      id: "temperature:hot",
      temperature: { value: 275, unit: "degC" },
      shear_modulus: { value: 123, unit: "MPa" },
      provenance: "user entered"
    }]);
    expect(JSON.stringify(m)).toBe(original);
  });
  it("rejects half-entered quantities and duplicate material IDs", async () => {
    const queue = vi.fn();
    render(<MaterialTemperatureForm
      model={model()}
      selection={{ type: "material", id: "material:a" }}
      onQueueIntent={queue}
    />);
    fireEvent.click(screen.getByText("Add temperature point"));
    enter("Point 1 ID", "hot");
    enter("Point 1 Temperature unit", "K");
    fireEvent.click(screen.getByText("Queue temperature points"));
    expect(await screen.findByRole("alert")).toHaveTextContent("finite value");
    expect(queue).not.toHaveBeenCalled();
    fireEvent.click(screen.getByText("Clear point 1 temperature"));
    fireEvent.click(screen.getByText("Add temperature point"));
    enter("Point 2 ID", "hot");
    fireEvent.click(screen.getByText("Queue temperature points"));
    expect(await screen.findByRole("alert")).toHaveTextContent("unique");
    expect(queue).not.toHaveBeenCalled();
  });
  it("removes an existing point while canonicalizing the exact previous array", async () => {
    const m = model();
    m.materials![0].temperature_points = [{ id: "old", shear_modulus: { value: 1, unit: "MPa" } }];
    const queue = vi.fn();
    render(<MaterialTemperatureForm
      model={m}
      selection={{ type: "material", id: "material:a" }}
      onQueueIntent={queue}
    />);
    fireEvent.click(screen.getByText("Remove point 1"));
    fireEvent.click(screen.getByText("Queue temperature points"));
    await waitFor(() => expect(queue).toHaveBeenCalledOnce());
    expect(canonicalJsonString).toHaveBeenCalledWith(m.materials![0].temperature_points);
    expect(queue.mock.calls[0][0].change.after).toBe("[]");
  });
  it("drops an async result after selection changes", async () => {
    let finish!: (value: string) => void;
    vi.mocked(canonicalJsonString).mockImplementation(() => new Promise(resolve => {
      finish = resolve;
    }));
    const m = model();
    m.materials![0].temperature_points = [];
    const queue = vi.fn();
    const rendered = render(<MaterialTemperatureForm
      model={m}
      selection={{ type: "material", id: "material:a" }}
      onQueueIntent={queue}
    />);
    fireEvent.click(screen.getByText("Queue temperature points"));
    rendered.rerender(<MaterialTemperatureForm
      model={m}
      selection={{ type: "node", id: "node:a" }}
      onQueueIntent={queue}
    />);
    finish("[]");
    await Promise.resolve();
    expect(queue).not.toHaveBeenCalled();
  });
  it("atomically switches whole-pipe wind exposure to an entered partial span", async () => {
    const m = model(), queue = vi.fn(), original = JSON.stringify(m);
    render(<WindExposureForm
      model={m}
      selection={{ type: "load", id: "load:a" }}
      onQueueIntent={queue}
    />);
    fireEvent.click(screen.getByLabelText("Whole pipe pipe:a"));
    fireEvent.click(screen.getByText("Add exposure span"));
    enter("Span 1 pipe", "pipe:a");
    quantity("Span 1 start fraction", "0", "1");
    quantity("Span 1 end fraction", "0.75", "none");
    fireEvent.click(screen.getByText("Queue wind exposure"));
    await waitFor(() => expect(queue).toHaveBeenCalledOnce());
    const intent = queue.mock.calls[0][0];
    expect(intent.change.field_path).toBe("equivalent_static.wind.exposure");
    expect(canonicalJsonString).toHaveBeenCalledWith({ exposed_pipe_refs: ["pipe:a"], exposed_spans: [] });
    expect(JSON.parse(intent.change.after)).toEqual({
      exposed_pipe_refs: [], exposed_spans: [{
        pipe_ref: "pipe:a",
        start_fraction: { value: 0, unit: "1" },
        end_fraction: { value: 0.75, unit: "none" }
      }]
    });
    expect(JSON.stringify(m)).toBe(original);
  });
  it("blocks overlapping wind spans before queuing", async () => {
    const queue = vi.fn();
    render(<WindExposureForm
      model={model()}
      selection={{ type: "load", id: "load:a" }}
      onQueueIntent={queue}
    />);
    fireEvent.click(screen.getByLabelText("Whole pipe pipe:a"));
    for (let i = 1; i <= 2; i++) {
      fireEvent.click(screen.getByText("Add exposure span"));
      enter(`Span ${i} pipe`, "pipe:a");
      quantity(`Span ${i} start fraction`, "0", "1");
      quantity(`Span ${i} end fraction`, "0.5", "1");
    }
    fireEvent.click(screen.getByText("Queue wind exposure"));
    expect(await screen.findByRole("alert")).toHaveTextContent("overlap");
    expect(queue).not.toHaveBeenCalled();
  });
  it("creates an explicitly named support with nested stiffness through the creation envelope", async () => {
    const queue = vi.fn();
    render(<SupportConfigurationForm
      model={model()}
      selection={{ type: "node", id: "node:a" }}
      onQueueIntent={queue}
    />);
    enter("Support ID", "support:new");
    enter("Support label", "New spring");
    enter("Support provenance", "user entered");
    enter("Support family", "spring");
    fireEvent.click(screen.getByText("Add support stiffness"));
    enter("Support stiffness DOF", "UY");
    quantity("Support stiffness quantity", "2.5", "N/mm");
    fireEvent.click(screen.getByText("Queue support creation"));
    await waitFor(() => expect(queue).toHaveBeenCalledOnce());
    const intent = queue.mock.calls[0][0];
    expect(intent.change).toMatchObject({
      change_kind: "create_support",
      field_path: "supports",
      before: "not_present"
    });
    expect(JSON.parse(intent.change.after)).toMatchObject({
      id: "support:new",
      node: "node:a",
      family: "spring",
      stiffness: { dof: "UY", value: { value: 2.5, unit: "N/mm" } }
    });
  });
  it("preserves hanger quantities and removes optional support blocks explicitly", async () => {
    const m = model();
    m.supports[0].hanger = {
      hanger_type: "constant_effort_support",
      constant_load: { value: 7, unit: "lbf" },
      movement_limit: { value: 1, unit: "in" },
      source_reference: "user"
    };
    const original = JSON.stringify(m);
    const queue = vi.fn();
    render(<SupportConfigurationForm
      model={m}
      selection={{ type: "support", id: "support:a" }}
      onQueueIntent={queue}
    />);
    enter("Support family", "vertical_support");
    fireEvent.click(screen.getByLabelText("Restrain UY"));
    fireEvent.click(screen.getByLabelText("Restrain UZ"));
    fireEvent.click(screen.getByLabelText("Include hanger data"));
    fireEvent.click(screen.getByText("Queue support configuration"));
    await waitFor(() => expect(queue).toHaveBeenCalledOnce());
    expect(canonicalJsonString).toHaveBeenCalledWith({
      restraints: ["UY"],
      hanger: m.supports[0].hanger,
      provenance: "invented"
    });
    expect(JSON.parse(queue.mock.calls[0][0].change.after)).toEqual({
      restraints: ["UZ"],
      provenance: "invented",
      family: "vertical_support"
    });
    expect(JSON.stringify(m)).toBe(original);
  });
  it("queues a friction support using a restrained source DOF and explicit coefficient", async () => {
    const queue = vi.fn();
    render(<SupportConfigurationForm
      model={model()}
      selection={{ type: "node", id: "node:a" }}
      onQueueIntent={queue}
    />);
    enter("Support ID", "support:f");
    enter("Support label", "Friction");
    enter("Support provenance", "user");
    fireEvent.click(screen.getByLabelText("Include nonlinear behavior"));
    enter("Nonlinear behavior", "friction");
    enter("Nonlinear DOF", "UX");
    enter("Initial state", "sticking");
    quantity("friction coefficient", "0.2", "1");
    fireEvent.click(screen.getByLabelText("Derive normal reaction from a support"));
    enter("Reaction source support", "support:a");
    enter("Reaction source DOF", "UY");
    fireEvent.click(screen.getByText("Queue support creation"));
    await waitFor(() => expect(queue).toHaveBeenCalledOnce());
    expect(JSON.parse(queue.mock.calls[0][0].change.after).nonlinear).toEqual({
      behavior: "friction",
      dof: "UX",
      initial_state: "sticking",
      friction_coefficient: { value: 0.2, unit: "1" },
      normal_reaction_source: { support_ref: "support:a", dof: "UY" }
    });
  });
  it("blocks duplicate support IDs and competing queued updates", async () => {
    const m = model(), queue = vi.fn();
    const rendered = render(<SupportConfigurationForm
      model={m}
      selection={{ type: "node", id: "node:a" }}
      onQueueIntent={queue}
    />);
    enter("Support ID", "support:a");
    enter("Support label", "Duplicate");
    enter("Support provenance", "user");
    fireEvent.click(screen.getByText("Queue support creation"));
    expect(queue).not.toHaveBeenCalled();
    rendered.unmount();
    render(<SupportConfigurationForm
      model={m}
      selection={{ type: "support", id: "support:a" }}
      onQueueIntent={queue}
      queuedIntents={[{ target: { ref: "support:a" }, change: { field_path: "configuration" } } as EditorOperationIntent]}
    />);
    fireEvent.click(screen.getByText("Queue support configuration"));
    expect(await screen.findByRole("alert")).toHaveTextContent("pending change");
    expect(queue).not.toHaveBeenCalled();
  });
});

describe("canonical support family authoring", () => {
  it("offers exactly nine canonical choices with readable labels and no inferred configuration", () => {
    render(<SupportConfigurationForm model={model()} selection={{ type: "support", id: "support:a" }} onQueueIntent={vi.fn()} />);
    const select = screen.getByLabelText("Support family") as HTMLSelectElement;
    expect(Array.from(select.options).filter(option => !option.disabled).map(option => [option.value, option.textContent])).toEqual([
      ["anchor", "Anchor"], ["guide", "Guide"], ["line_stop", "Line stop"], ["vertical_support", "Vertical support"],
      ["spring", "Spring"], ["variable_spring_hanger", "Variable spring hanger"], ["spring_hanger", "Spring hanger"],
      ["constant_effort_support", "Constant-effort support"], ["nonlinear", "Nonlinear support"]
    ]);
    enter("Support family", "vertical_support");
    expect(screen.getByLabelText("Restrain UY")).toBeChecked();
    expect(screen.getByLabelText("Restrain UZ")).not.toBeChecked();
    enter("Support family", "nonlinear");
    expect(screen.getByLabelText("Include nonlinear behavior")).not.toBeChecked();
    expect(screen.getByLabelText("Include hanger data")).not.toBeChecked();
  });

  it.each([undefined, null, "Unsupported family", "", " line_stop ", "LineStop"])("preserves the exact source family %s until an explicit canonical choice", async family => {
    const m = model();
    if (family !== undefined) Object.assign(m.supports[0], { family });
    const original = JSON.stringify(m), queue = vi.fn();
    render(<SupportConfigurationForm model={m} selection={{ type: "support", id: "support:a" }} onQueueIntent={queue} />);
    const select = screen.getByLabelText("Support family") as HTMLSelectElement;
    expect(select.selectedOptions[0]).toHaveTextContent(family === undefined ? "Not provided" : family === null ? "Null source value" : "Unsupported source value");
    fireEvent.click(screen.getByText("Queue support configuration"));
    await waitFor(() => expect(queue).toHaveBeenCalledOnce());
    const intent = queue.mock.calls[0][0];
    const before = JSON.parse(intent.change.before), after = JSON.parse(intent.change.after);
    expect(after).toEqual(before);
    if (family === undefined) expect(after).not.toHaveProperty("family"); else expect(after.family).toBe(family);
    enter("Support family", "line_stop");
    fireEvent.click(screen.getByText("Queue support configuration"));
    await waitFor(() => expect(queue).toHaveBeenCalledTimes(2));
    expect(JSON.parse(queue.mock.calls[1][0].change.after)).toEqual({ ...before, family: "line_stop" });
    expect(JSON.stringify(m)).toBe(original);
  });

  it.each([["line_stop", "UX"], ["vertical_support", "UZ"]])("creates %s with only the explicitly selected %s restraint", async (family, dof) => {
    const m = model(), original = JSON.stringify(m), queue = vi.fn();
    render(<SupportConfigurationForm model={m} selection={{ type: "node", id: "node:a" }} onQueueIntent={queue} />);
    enter("Support ID", "support:new"); enter("Support label", "Invented family"); enter("Support provenance", "original test");
    enter("Support family", family);
    for (const candidate of ["UX", "UY", "UZ", "RX", "RY", "RZ"]) expect(screen.getByLabelText(`Restrain ${candidate}`)).not.toBeChecked();
    fireEvent.click(screen.getByLabelText(`Restrain ${dof}`));
    fireEvent.click(screen.getByText("Queue support creation"));
    await waitFor(() => expect(queue).toHaveBeenCalledOnce());
    expect(JSON.parse(queue.mock.calls[0][0].change.after)).toEqual({ id: "support:new", label: "Invented family", node: "node:a", restraints: [dof], provenance: "original test", family });
    expect(JSON.stringify(m)).toBe(original);
  });
});
