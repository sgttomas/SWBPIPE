// A9 panel (DEL-07-05 Design `weight-cog-summary.md` §6 and §7). Invented models and invented
// generator outputs; the last test runs the actual candidate engines on the bundled demo.
import { cleanup, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { loadPreviewModel } from "../../services/previewService";
import type { SelfWeightPlanOutcome } from "../../services/selfWeightPlanService";
import type { PreviewModel } from "../../types";
import { WeightSummaryPanel, WEIGHT_SUMMARY_BOUNDARY_TEXT } from "./WeightSummaryPanel";
import { COMPONENT_WEIGHT_METHOD, PIPE_WEIGHT_METHOD } from "./weightSummary";

afterEach(() => { cleanup(); vi.restoreAllMocks(); });

function model(): PreviewModel {
  const node = (id: string, x: number, y: number, z: number) => ({ id, label: id, position: { x, y, z }, provenance: "invented" });
  const pipe = (id: string, from: string, to: string) => ({ id, label: id, from, to, section: {}, material: "m", provenance: "invented" });
  return {
    schema_version: "invented", document_kind: "invented", data_boundary: {},
    project: { id: "p", name: "p", description: "", units: { length: "mm" } },
    analysis_status: { mechanics: "draft", rule_check: "not_run", professional_acceptance: "not_assessed" },
    nodes: [node("A", 0, 0, 0), node("B", 3000, 0, 0), node("C", 3000, 1000, 0)],
    pipe_segments: [pipe("P1", "A", "B"), pipe("P2", "B", "C"), pipe("P3", "C", "A")],
    lines: [{ id: "line:main", label: "Main", pipe_refs: ["P1"] }],
    supports: [], components: [{ id: "V", label: "V", kind: "valve", node: "C", provenance: "i", geometry: { weight: { value: 1, unit: "kN" } } }],
    load_cases: [], diagnostics: []
  } as unknown as PreviewModel;
}

// P1: 123.4567891 N/m over 3 m; P2's member carries valve V (1000 N, offset (0.0123456789, 0, 0) m); P3 is refused.
const evidence: Record<string, unknown[]> = {
  P1: [{ method: PIPE_WEIGHT_METHOD, pipe_id: "P1", contents_absent: false, insulation_absent: true, generated_payload: { magnitude: { value: -123.4567891, unit: "N/m" } } }],
  P2: [{ method: COMPONENT_WEIGHT_METHOD, component_id: "V", component_node: "C", omitted_pipe_id: "P2", weight_N: 1000, center_of_gravity_m: [0.0123456789, 0, 0], wrench_part: "force" }]
};
function outcome(refs: readonly string[]): SelfWeightPlanOutcome {
  const base = { document_kind: "openpipestress.desktop.self_weight_plan_outcome" as const, input_model_unchanged: true as const, source_model_hash: "sha256:x",
    mutation_route: "structured_operations_only" as const, requires_user_acceptance: true as const };
  if (refs.includes("P3")) return { ...base, status: "blocked", plan: null, diagnostics: [{ code: "SELF_WEIGHT_INPUT_INVALID", severity: "blocking", message: "explicit material density required", remediation: "r", affected_objects: ["P3"] }] };
  return { ...base, status: "ready", diagnostics: [], plan: { source_model_hash: "sha256:x", changes: [], scope_label: "selected_pipe_mass_only", source_evidence: refs.flatMap((ref) => evidence[ref] ?? []) } };
}
const hash = async () => ({ value: "sha256:x" });

function enter(value: string, unit = "m/s^2", axis = "global_z", prefix = "weight-summary") {
  fireEvent.change(screen.getByTestId(`${prefix}-gravity-value`), { target: { value } });
  fireEvent.change(screen.getByTestId(`${prefix}-gravity-unit`), { target: { value: unit } });
  fireEvent.change(screen.getByTestId(`${prefix}-gravity-axis`), { target: { value: axis } });
}

describe("WeightSummaryPanel", () => {
  it("asks for the gravity before computing and assumes none", () => {
    const generate = vi.fn(async (_m: PreviewModel, request: { pipe_refs: string[] }) => outcome(request.pipe_refs));
    render(<WeightSummaryPanel model={model()} generate={generate} hash={hash} />);
    fireEvent.click(screen.getByTestId("weight-summary-compute"));
    expect(screen.getByRole("alert")).toHaveTextContent("Enter the gravity value, its unit and its direction.");
    expect(generate).not.toHaveBeenCalled();
    expect(screen.getByText(WEIGHT_SUMMARY_BOUNDARY_TEXT)).toBeVisible();
  });

  it("shows the totals and centre of gravity under the display rule, with what is not counted", async () => {
    const generate = vi.fn(async (_m: PreviewModel, request: { pipe_refs: string[]; gravity: unknown; case_id: string }) => outcome(request.pipe_refs));
    const view = render(<WeightSummaryPanel model={model()} generate={generate} hash={hash} />);
    enter("-9.80665");
    fireEvent.click(screen.getByTestId("weight-summary-compute"));
    const table = await screen.findByTestId("weight-summary-table");
    expect(generate.mock.calls[0][1]).toMatchObject({ case_id: "weight-summary-view", pipe_refs: ["P1", "P2", "P3"], gravity: { value: -9.80665, unit: "m/s^2", axis: "global_z" }, source_model_hash: "sha256:x" });
    // 123.4567891 x 3 = 370.3703673 N; with 1000 N, 1370.3703673 N.
    expect(within(screen.getByTestId("weight-summary-row-pipes")).getByText("370.37 N")).toBeVisible();
    expect(within(screen.getByTestId("weight-summary-row-components")).getByText("1000 N")).toBeVisible();
    expect(within(screen.getByTestId("weight-summary-row-total")).getByText("1370.37 N")).toBeVisible();
    expect(table).toHaveAccessibleName("Weight summary: Whole model (3 pipes)");
    // x = (370.3703673 x 1500 + 1000 x 3012.3456789) / 1370.3703673 = 2603.6036... mm.
    const x = (123.4567891 * 3 * 1500 + 1000 * (3000 + 12.3456789)) / (123.4567891 * 3 + 1000);
    expect(screen.getByTestId("weight-summary-cog-x")).toHaveTextContent(new RegExp(`^${String(Number(x.toPrecision(6)))}$`));
    expect(screen.getByTestId("weight-summary-cog-y")).toHaveTextContent(String(Number((1000 * 1000 / (123.4567891 * 3 + 1000)).toPrecision(6))));
    expect(screen.getByTestId("weight-summary-cog-z")).toHaveTextContent(/^0$/);
    // P3 is not counted, so both values are labelled partial and the exclusion list is open.
    expect(screen.getByRole("columnheader", { name: "Centre of gravity of the counted weights (partial) (mm)" })).toBeVisible();
    expect(screen.getByRole("rowheader", { name: "Partial total (2 of 3 pipes counted)" })).toBeVisible();
    expect(screen.getByTestId("weight-summary-partial")).toHaveTextContent("Partial: 2 of 3 pipes counted. The total and centre of gravity are those of the counted weights only");
    expect(screen.getByTestId("weight-summary-not-counted")).toHaveAttribute("open");
    expect(screen.queryByRole("rowheader", { name: "Total" })).toBeNull();
    expect(screen.getByTestId("weight-summary-basis")).toHaveTextContent("under gravity -9.80665 m/s^2 along Global Z");
    expect(screen.getByTestId("weight-summary-basis")).toHaveTextContent("Contents entered the mass of 1 of 1 counted pipes, insulation of 0.");
    const notCounted = screen.getByTestId("weight-summary-not-counted");
    expect(notCounted).toHaveTextContent("1 of 3 pipes not counted");
    expect(notCounted).toHaveTextContent("P3: SELF_WEIGHT_INPUT_INVALID: explicit material density required");
    expect(screen.queryByTestId("weight-summary-components-not-included")).toBeNull();
    // No stored value appears at full precision anywhere in the markup.
    expect(view.container.innerHTML).not.toMatch(/370\.370|1370\.370|2603\.60[0-9]{2}/);
  });

  it("labels the total and centre of gravity unqualified only when nothing is left out", async () => {
    const generate = vi.fn(async (_m: PreviewModel, request: { pipe_refs: string[] }) => outcome(request.pipe_refs));
    render(<WeightSummaryPanel model={model()} generate={generate} hash={hash} />);
    enter("9.8");
    fireEvent.change(screen.getByTestId("weight-summary-scope"), { target: { value: "line:line:main" } });
    fireEvent.click(screen.getByTestId("weight-summary-compute"));
    await screen.findByTestId("weight-summary-table");
    expect(screen.getByRole("rowheader", { name: "Total" })).toBeVisible();
    expect(screen.getByRole("columnheader", { name: "Centre of gravity (mm)" })).toBeVisible();
    expect(screen.queryByTestId("weight-summary-partial")).toBeNull();
    expect(screen.getByTestId("weight-summary-summary")).toHaveAttribute("data-partial", "false");
  });

  it("labels a partial summary for a component weight or an unresolved reference left out", async () => {
    // The line holds P2, whose not-solver-consumed valve W the generator leaves out (P2's own weight is counted), and P9, which names no pipe.
    const m = model();
    m.lines = [{ id: "line:w", pipe_refs: ["P2", "P9"] }];
    m.components = [{ id: "W", label: "W", kind: "valve", node: "B", provenance: "i", geometry: { rigid_pipe_ref: "P2", weight: { value: 5, unit: "N" } } }] as PreviewModel["components"];
    const generate = vi.fn(async (): Promise<SelfWeightPlanOutcome> => ({ ...outcome(["P1"]),
      plan: { source_model_hash: "sha256:x", changes: [], scope_label: "selected_pipe_mass_only", source_evidence: [{ method: PIPE_WEIGHT_METHOD, pipe_id: "P2", contents_absent: true, insulation_absent: true, generated_payload: { magnitude: { value: -2, unit: "N/m" } } }] } }));
    render(<WeightSummaryPanel model={m} generate={generate} hash={hash} />);
    enter("9.8");
    fireEvent.change(screen.getByTestId("weight-summary-scope"), { target: { value: "line:line:w" } });
    fireEvent.click(screen.getByTestId("weight-summary-compute"));
    await screen.findByTestId("weight-summary-table");
    expect(screen.getByRole("rowheader", { name: "Partial total (1 of 1 pipe counted; 1 component weight not included; 1 unresolved scope reference)" })).toBeVisible();
    expect(within(screen.getByTestId("weight-summary-row-total")).getByText("2 N")).toBeVisible();
    expect(screen.getByRole("columnheader", { name: "Centre of gravity of the counted weights (partial) (mm)" })).toBeVisible();
    const omitted = screen.getByTestId("weight-summary-components-not-included");
    expect(omitted).toHaveAttribute("open");
    expect(omitted).toHaveTextContent("1 component weight not included");
    expect(omitted).toHaveTextContent("W: The self-weight generator does not apply its stated weight; the distributed weight of its mapped pipe P2 is counted instead.");
  });

  it("states an out-of-range total instead of throwing", async () => {
    const generate = vi.fn(async (): Promise<SelfWeightPlanOutcome> => ({ ...outcome(["P1"]),
      plan: { source_model_hash: "sha256:x", changes: [], scope_label: "selected_pipe_mass_and_component_weight", source_evidence: [
        { method: COMPONENT_WEIGHT_METHOD, component_id: "V", component_node: "A", omitted_pipe_id: "P1", weight_N: 1.193805208e308, center_of_gravity_m: [0, 0, 0], wrench_part: "force" },
        { method: COMPONENT_WEIGHT_METHOD, component_id: "U", component_node: "B", omitted_pipe_id: "P2", weight_N: 1.193805208e308, center_of_gravity_m: [0, 0, 0], wrench_part: "force" }] } }));
    render(<WeightSummaryPanel model={model()} generate={generate} hash={hash} />);
    enter("9.8");
    fireEvent.click(screen.getByTestId("weight-summary-compute"));
    expect(await screen.findByTestId("weight-summary-unavailable")).toHaveTextContent("The summed weight is outside the representable range; no total or centre of gravity is formed.");
    expect(screen.queryByTestId("weight-summary-table")).toBeNull();
    expect(screen.queryByTestId("weight-summary-error")).toBeNull();
  });

  it("marks a summary stale when a selection gains an unresolved reference", async () => {
    const generate = vi.fn(async (_m: PreviewModel, request: { pipe_refs: string[] }) => outcome(request.pipe_refs));
    const m = model();
    const view = render(<WeightSummaryPanel model={m} selectedPipeRefs={["P1"]} generate={generate} hash={hash} />);
    enter("9.8");
    fireEvent.change(screen.getByTestId("weight-summary-scope"), { target: { value: "selection" } });
    fireEvent.click(screen.getByTestId("weight-summary-compute"));
    await screen.findByTestId("weight-summary-table");
    expect(screen.getByRole("rowheader", { name: "Total" })).toBeVisible();
    view.rerender(<WeightSummaryPanel model={m} selectedPipeRefs={["P1", "missing"]} generate={generate} hash={hash} />);
    expect(screen.queryByTestId("weight-summary-table")).toBeNull();
    expect(screen.getByTestId("weight-summary-stale")).toBeVisible();
  });

  it("marks a summary stale when the gravity, scope or model changes", async () => {
    const generate = vi.fn(async (_m: PreviewModel, request: { pipe_refs: string[] }) => outcome(request.pipe_refs));
    const first = model();
    const view = render(<WeightSummaryPanel model={first} generate={generate} hash={hash} />);
    enter("9.8");
    fireEvent.click(screen.getByTestId("weight-summary-compute"));
    await screen.findByTestId("weight-summary-table");
    fireEvent.change(screen.getByTestId("weight-summary-gravity-value"), { target: { value: "9.81" } });
    expect(screen.queryByTestId("weight-summary-table")).toBeNull();
    expect(screen.getByTestId("weight-summary-stale")).toBeVisible();
    fireEvent.change(screen.getByTestId("weight-summary-gravity-value"), { target: { value: "9.8" } });
    expect(screen.getByTestId("weight-summary-table")).toBeVisible();
    view.rerender(<WeightSummaryPanel model={model()} generate={generate} hash={hash} />);
    expect(screen.getByTestId("weight-summary-stale")).toBeVisible();
  });

  it("offers each line and the selected pipes as scopes and sums only their pipes", async () => {
    const generate = vi.fn(async (_m: PreviewModel, request: { pipe_refs: string[] }) => outcome(request.pipe_refs));
    render(<WeightSummaryPanel model={model()} selectedPipeRefs={["P2", "P9"]} generate={generate} hash={hash} testIdPrefix="ws" />);
    const scope = screen.getByTestId("ws-scope") as HTMLSelectElement;
    expect([...scope.options].map((option) => option.textContent)).toEqual(["Whole model (3 pipes)", "Line Main (line:main) (1 pipe)", "Selected pipes (1)"]);
    enter("9.8", "m/s^2", "global_y", "ws");
    fireEvent.change(scope, { target: { value: "selection" } });
    fireEvent.click(screen.getByTestId("ws-compute"));
    await screen.findByTestId("ws-table");
    expect(generate.mock.calls.at(-1)![1].pipe_refs).toEqual(["P2"]);
    expect(within(screen.getByTestId("ws-row-total")).getByText("1000 N")).toBeVisible();
    expect(screen.getByTestId("ws-all-counted")).toBeVisible();
    expect(screen.getByTestId("ws-unresolved")).toHaveTextContent("1 scope reference names no model pipe: P9.");
    fireEvent.change(scope, { target: { value: "line:line:main" } });
    fireEvent.click(screen.getByTestId("ws-compute"));
    await waitFor(() => expect(screen.getByTestId("ws-summary").getAttribute("data-scope")).toBe("line:line:main"));
    expect(generate.mock.calls.at(-1)![1].pipe_refs).toEqual(["P1"]);
  });

  it("states the generator's refusal and forms no summary", async () => {
    const generate = vi.fn(async (): Promise<SelfWeightPlanOutcome> => ({ ...outcome(["P3"]), diagnostics: [{ code: "SELF_WEIGHT_INPUT_INVALID", severity: "blocking", message: "normalized acceleration must be finite and nonzero", remediation: "Fix it.", affected_objects: ["gravity"] }] }));
    render(<WeightSummaryPanel model={model()} generate={generate} hash={hash} />);
    enter("9.8", "furlong/s^2");
    fireEvent.click(screen.getByTestId("weight-summary-compute"));
    const unavailable = await screen.findByTestId("weight-summary-unavailable");
    expect(unavailable).toHaveTextContent("The self-weight generator refused the request; no weight is summed.");
    expect(unavailable).toHaveTextContent("normalized acceleration must be finite and nonzero Fix it.");
    expect(generate).toHaveBeenCalledOnce();
    expect(screen.queryByTestId("weight-summary-table")).toBeNull();
  });

  it("computes the bundled demo through the actual candidate engines", async () => {
    const demo = await loadPreviewModel();
    render(<WeightSummaryPanel model={demo} />);
    enter("-9.8", "m/s^2", "global_y");
    fireEvent.click(screen.getByTestId("weight-summary-compute"));
    await screen.findByTestId("weight-summary-table", undefined, { timeout: 10000 });
    expect(within(screen.getByTestId("weight-summary-row-total")).getByText("420 N")).toBeVisible();
    // Three of the four pipes are not counted: the 420 N is labelled partial, with the list open.
    expect(screen.getByRole("rowheader", { name: "Partial total (1 of 4 pipes counted)" })).toBeVisible();
    expect(screen.getByTestId("weight-summary-partial")).toBeVisible();
    expect(screen.getByTestId("weight-summary-cog-x")).toHaveTextContent(/^7\.72$/);
    expect(screen.getByTestId("weight-summary-not-counted")).toHaveTextContent("3 of 4 pipes not counted");
  });
});
