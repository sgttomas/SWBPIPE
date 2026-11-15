// DEL-04-11 REQ-008 / AC-008 and REQ-012: the person's act of applying a sized selection. The model and library are
// the invented two-span sizing input (read, not changed); each result document is formed here from invented sized
// values and bound by the desktop's checked hash to that model and library, as a sizing run binds them. Only the
// members the action reads are formed.
import { cleanup, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import twoSpanInput from "../../../../../fixtures/results/hanger_sizing_input_two_span.json";
import { canonicalJsonString, canonicalSha256HexCheckedV1 } from "../../services/hashService";
import type { PreviewModel } from "../../types";
import type { HangerSizingCandidate, HangerSizingSizedDocument } from "../results/hangerSizingResult";
import { HangerSizingApply } from "./HangerSizingApply";
import { HANGER_SIZING_APPLY_TEXT, HANGER_SIZING_MODEL_CHANGED_TEXT, HANGER_SIZING_NO_FIT_TEXT } from "./sizedSelection";

afterEach(cleanup);

const model = () => structuredClone(twoSpanInput.request.model) as unknown as PreviewModel;
const library = () => structuredClone(twoSpanInput.library) as Record<string, unknown>;
const LIBRARY = "invented.hangers.sizing";
const SIZING_SHA = "5".repeat(64);

async function recordSha(lib: Record<string, unknown>, hangerId: string) {
  const record = (lib.hanger_records as { hanger_id: string }[]).find((entry) => entry.hanger_id === hangerId);
  return canonicalSha256HexCheckedV1(record);
}

const common = (support: string, dof: "UX" | "UY" | "UZ") => ({
  support, node: "node:B", dof, family_policy: "variable_else_constant" as const, w_n: 5000, u_cold_m: 0, operating: [], span_m: [0, 0.001] as const,
  required_travel_m: 0.005, failure_counts: { variable: null, constant: null }
});
const variable = (support: string, dof: "UX" | "UY" | "UZ", hangerId: string, recordSha256: string): HangerSizingCandidate => ({
  ...common(support, dof), outcome: "selected",
  selected: { family: "variable_spring", hanger_id: hangerId, record_sha256: recordSha256, k_n_per_m: 200000, f_cold_n: 5197.5, f_cold_condition_n: 5197.5,
    envelope_n: [4600, 5716], operating: [{ case: "load:O1", f_n: 5000, variation: 0.04 }, { case: "load:O2", f_n: 5316, variation: 0.02 }] },
  governing: { travel_case: "load:O1", variation_case: "load:O1" },
  verification_assumption: { role: "spring", k_n_per_m: 200000, f_cold_n: 5197.5 },
  verification: null as never
});
const constantFallback = (support: string, hangerId: string, recordSha256: string): HangerSizingCandidate => ({
  ...common(support, "UZ"), outcome: "selected_by_fallback",
  selected: { family: "constant_effort", hanger_id: hangerId, record_sha256: recordSha256, load_n: 5000 },
  governing: { travel_case: "load:O1", variation_case: null },
  verification_assumption: { role: "force", force_n: 5000 },
  verification: null as never
});
const noFit = (support: string): HangerSizingCandidate => ({
  ...common(support, "UY"), outcome: "no_record_fits", selected: null, governing: null,
  verification_assumption: { role: "force", force_n: 5000 }, verification: null
});

async function sized(source: PreviewModel, lib: Record<string, unknown>, candidates: HangerSizingCandidate[]): Promise<HangerSizingSizedDocument> {
  return {
    status: "sized",
    inputs: {
      canonical_profile: "openpipestress_jcs_ijson_v1",
      sizing_request: { design_operating_case: "load:O1" },
      sizing_request_sha256: SIZING_SHA,
      solve_request: { sha256: await canonicalSha256HexCheckedV1({ model: source, materials: [] }) },
      library: { library_id: LIBRARY, sha256: await canonicalSha256HexCheckedV1(lib) }
    },
    result: { candidates }
  } as unknown as HangerSizingSizedDocument;
}

describe("applying a sized selection", () => {
  it("offers a selected variable record, prepares its batch only on the person's act, and queues it for review only on a second act", async () => {
    const source = model();
    const original = await canonicalJsonString(source);
    const lib = library();
    const document = await sized(source, lib, [variable("support:B", "UZ", "VS-200-L", await recordSha(lib, "VS-200-L"))]);
    const queue = vi.fn();
    render(<HangerSizingApply document={document} library={lib} model={source} busy={false} onQueueBatch={queue} />);
    expect(screen.getByText(HANGER_SIZING_APPLY_TEXT)).toBeInTheDocument();
    const prepare = await screen.findByRole("button", { name: "Prepare applying record VS-200-L to support:B" });
    expect(screen.queryByTestId("hanger-sizing-apply-review")).toBeNull();
    fireEvent.click(prepare);
    const review = await screen.findByTestId("hanger-sizing-apply-review");
    // Prepared, shown, and not queued: nothing reaches the review queue, and the model is untouched.
    expect(queue).not.toHaveBeenCalled();
    expect(JSON.parse(within(review).getByTestId("hanger-sizing-apply-before").textContent!)).toEqual({ family: "vertical_support", restraints: ["UZ"], provenance: "invented_test_value" });
    const after = JSON.parse(within(review).getByTestId("hanger-sizing-apply-after").textContent!);
    expect(after.restraints).toEqual(["UZ"]);
    expect(after.hanger).toMatchObject({ installed_load: { value: 5197.5, unit: "N" }, cold_load: { value: 5197.5, unit: "N" }, hot_load: { value: 5000, unit: "N" } });
    expect(within(review).getByTestId("hanger-sizing-apply-source-note")).toHaveTextContent(
      `Hanger sizing run with sizing request sha256 ${SIZING_SHA} and solve request sha256 ${document.inputs.solve_request.sha256}; ` +
      `library ${LIBRARY} sha256 ${document.inputs.library.sha256}; record VS-200-L sha256 ${document.result.candidates[0].selected!.record_sha256}.`
    );
    fireEvent.click(within(review).getByRole("button", { name: "Queue sized selection for review" }));
    expect(queue).toHaveBeenCalledTimes(1);
    const [batch] = queue.mock.calls[0];
    expect(batch.operations[0].audit_boundary).toMatchObject({ requires_user_acceptance: true, direct_model_mutation_allowed: false });
    expect(batch.operations[0].operation_status).toBe("proposed");
    expect(JSON.parse(batch.operations[0].change.after).restraints).toEqual(["UZ"]);
    expect(await canonicalJsonString(source)).toBe(original);
  });

  it("offers a fallback constant selection with its hot-load basis, and offers nothing for a candidate with no fitting record", async () => {
    const source = model();
    const lib = library();
    const document = await sized(source, lib, [constantFallback("support:B", "CE-1", await recordSha(lib, "CE-1")), noFit("support:C")]);
    render(<HangerSizingApply document={document} library={lib} model={source} busy={false} onQueueBatch={vi.fn()} />);
    fireEvent.click(await screen.findByRole("button", { name: "Prepare applying record CE-1 to support:B" }));
    const after = JSON.parse((await screen.findByTestId("hanger-sizing-apply-after")).textContent!);
    expect(after.hanger.constant_load).toEqual({ value: 5000, unit: "N" });
    expect(after.restraints).toEqual(["UZ"]);
    const none = screen.getByTestId("hanger-sizing-apply-support:C");
    expect(none).toHaveTextContent(HANGER_SIZING_NO_FIT_TEXT);
    expect(within(none).queryByRole("button")).toBeNull();
  });

  it("offers nothing when the record's stiffness direction disagrees with the sized direction", async () => {
    const source = model();
    const lib = library();
    const document = await sized(source, lib, [variable("support:B", "UZ", "VS-200-UY", await recordSha(lib, "VS-200-UY"))]);
    render(<HangerSizingApply document={document} library={lib} model={source} busy={false} onQueueBatch={vi.fn()} />);
    const row = await screen.findByText(/declares its stiffness on UY, but support:B was sized on UZ/);
    expect(row.closest("li")!.querySelector("button")).toBeNull();
  });

  it("offers nothing, and drops a prepared batch, once the model has changed since the sizing run", async () => {
    const source = model();
    const lib = library();
    const document = await sized(source, lib, [variable("support:B", "UZ", "VS-200-L", await recordSha(lib, "VS-200-L"))]);
    const queue = vi.fn();
    const { rerender } = render(<HangerSizingApply document={document} library={lib} model={source} busy={false} onQueueBatch={queue} />);
    fireEvent.click(await screen.findByRole("button", { name: "Prepare applying record VS-200-L to support:B" }));
    await screen.findByTestId("hanger-sizing-apply-review");
    const changed = model();
    changed.supports.find((support) => support.id === "support:B")!.provenance = "edited after sizing";
    rerender(<HangerSizingApply document={document} library={lib} model={changed} busy={false} onQueueBatch={queue} />);
    expect(await screen.findByTestId("hanger-sizing-apply-model-changed")).toHaveTextContent(HANGER_SIZING_MODEL_CHANGED_TEXT);
    expect(screen.queryByTestId("hanger-sizing-apply-review")).toBeNull();
    expect(screen.queryByRole("button")).toBeNull();
    expect(queue).not.toHaveBeenCalled();
    // An equal model (another object with the same content) is bound again.
    rerender(<HangerSizingApply document={document} library={lib} model={model()} busy={false} onQueueBatch={queue} />);
    await waitFor(() => expect(screen.getByRole("button", { name: "Prepare applying record VS-200-L to support:B" })).toBeInTheDocument());
  });

  it("offers nothing when the library is not the one the run bound", async () => {
    const source = model();
    const lib = library();
    const document = await sized(source, lib, [variable("support:B", "UZ", "VS-200-L", await recordSha(lib, "VS-200-L"))]);
    const other = library();
    (other.hanger_library as Record<string, unknown>).name = "Another invented library";
    render(<HangerSizingApply document={document} library={other} model={source} busy={false} onQueueBatch={vi.fn()} />);
    expect(await screen.findByText(/does not have the hash this run bound/)).toBeInTheDocument();
    expect(screen.queryByRole("button")).toBeNull();
  });
});
