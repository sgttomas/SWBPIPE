// DEL-04-11 REQ-008 / AC-008 and REQ-012: the person's act of applying every selection of one sizing run. The model and
// library are the invented four-span sizing input (read, not changed). The mixed run (a variable and a constant selection
// and one no-fit support) is formed here and bound by the desktop's checked hash, as in `runApplication.test.ts`; the
// constant-only run is the product's own four-span result (`fixtures/results/hanger_sizing_result_four_span.json`).
import { cleanup, fireEvent, render, screen, within } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import fourSpanInput from "../../../../../fixtures/results/hanger_sizing_input_four_span.json";
import fourSpanResult from "../../../../../fixtures/results/hanger_sizing_result_four_span.json";
import { canonicalJsonString, canonicalSha256HexCheckedV1 } from "../../services/hashService";
import type { PreviewModel } from "../../types";
import type { HangerSizingCandidate, HangerSizingSizedDocument } from "../results/hangerSizingResult";
import { HangerSizingApply } from "./HangerSizingApply";
import { HANGER_SIZING_APPLY_RUN_TEXT, HANGER_SIZING_RUN_UNCHANGED_TEXT } from "./runApplication";
import { HANGER_SIZING_APPLIED_WITH_RUN_TEXT, HANGER_SIZING_MODEL_CHANGED_TEXT, HANGER_SIZING_PRELOAD_PENDING_TEXT } from "./sizedSelection";

afterEach(cleanup);

const model = () => structuredClone(fourSpanInput.request.model) as unknown as PreviewModel;
const library = () => structuredClone(fourSpanInput.library) as Record<string, unknown>;
const productResult = () => structuredClone(fourSpanResult) as unknown as HangerSizingSizedDocument;
const PREPARE_RUN = "Prepare applying every selection of this run";
const QUEUE_RUN = "Queue every sized selection of this run for review";

async function recordSha(lib: Record<string, unknown>, hangerId: string) {
  return canonicalSha256HexCheckedV1((lib.hanger_records as { hanger_id: string }[]).find((entry) => entry.hanger_id === hangerId));
}

const common = (support: string, node: string) => ({
  support, node, dof: "UZ" as const, family_policy: "variable_else_constant" as const, w_n: 5000, u_cold_m: 0, operating: [], span_m: [0, 0.001] as const,
  required_travel_m: 0.005, failure_counts: { variable: null, constant: null }
});

/** The mixed run, or the run of only the candidates `keep` names. */
async function mixedRun(source: PreviewModel, lib: Record<string, unknown>, keep: readonly string[] = ["support:B", "support:C", "support:D"]): Promise<HangerSizingSizedDocument> {
  const candidates: HangerSizingCandidate[] = [
    { ...common("support:B", "node:B"), outcome: "selected",
      selected: { family: "variable_spring", hanger_id: "VS-200-L", record_sha256: await recordSha(lib, "VS-200-L"), k_n_per_m: 200000, f_cold_n: 5197.5,
        f_cold_condition_n: 5197.5, envelope_n: [4600, 5716], operating: [{ case: "load:O1", f_n: 5000, variation: 0.04 }] },
      governing: { travel_case: "load:O1", variation_case: "load:O1" },
      verification_assumption: { role: "spring", k_n_per_m: 200000, f_cold_n: 5197.5 }, verification: null as never },
    { ...common("support:C", "node:C"), outcome: "selected",
      selected: { family: "constant_effort", hanger_id: "CE-1", record_sha256: await recordSha(lib, "CE-1"), load_n: 3714.285714285714 },
      governing: { travel_case: "load:O1", variation_case: null },
      verification_assumption: { role: "force", force_n: 3714.285714285714 }, verification: null as never },
    { ...common("support:D", "node:D"), outcome: "no_record_fits", selected: null, governing: null,
      verification_assumption: { role: "force", force_n: 4571.428571428572 }, verification: null }
  ];
  return {
    status: "sized",
    inputs: {
      canonical_profile: "openpipestress_jcs_ijson_v1",
      sizing_request: { design_operating_case: "load:O1" },
      sizing_request_sha256: "5".repeat(64),
      solve_request: { sha256: await canonicalSha256HexCheckedV1({ model: source, materials: [] }) },
      library: { library_id: "invented.hangers.sizing", sha256: await canonicalSha256HexCheckedV1(lib) }
    },
    result: { candidates: candidates.filter((candidate) => keep.includes(candidate.support)) }
  } as unknown as HangerSizingSizedDocument;
}

describe("applying every selection of a sizing run", () => {
  it("prepares one batch only on the person's act, shows the supports changed and left unchanged and the preload sentence, and queues it once on a second act", async () => {
    const source = model();
    const original = await canonicalJsonString(source);
    const lib = library();
    const queue = vi.fn();
    render(<HangerSizingApply document={await mixedRun(source, lib)} library={lib} model={source} busy={false} onQueueBatch={queue} />);
    expect(await screen.findByText(HANGER_SIZING_APPLY_RUN_TEXT)).toBeInTheDocument();
    const prepare = await screen.findByRole("button", { name: PREPARE_RUN });
    // Two selections: no per-candidate act; each selection says it is applied only with the run.
    expect(screen.queryByRole("button", { name: /^Prepare applying record/ })).toBeNull();
    expect(screen.getByTestId("hanger-sizing-apply-support:B")).toHaveTextContent(`support:B: record VS-200-L. ${HANGER_SIZING_APPLIED_WITH_RUN_TEXT}`);
    expect(screen.getByTestId("hanger-sizing-apply-support:C")).toHaveTextContent(`support:C: record CE-1. ${HANGER_SIZING_APPLIED_WITH_RUN_TEXT}`);
    expect(screen.queryByTestId("hanger-sizing-apply-run-review")).toBeNull();
    expect(screen.queryByText(HANGER_SIZING_PRELOAD_PENDING_TEXT)).toBeNull();

    fireEvent.click(prepare);
    const review = await screen.findByTestId("hanger-sizing-apply-run-review");
    expect(queue).not.toHaveBeenCalled();
    expect(within(review).getByTestId("hanger-sizing-apply-run-description")).toHaveTextContent(
      "Applies 2 sized selections of this run in one batch: support:B, record VS-200-L (variable spring); support:C, record CE-1 (constant effort). " +
      "Left unchanged: no record fits: support:D."
    );
    expect(within(within(review).getByTestId("hanger-sizing-apply-run-changed")).getAllByRole("listitem").map((item) => item.textContent))
      .toEqual(["support:B: record VS-200-L", "support:C: record CE-1"]);
    expect(within(within(review).getByTestId("hanger-sizing-apply-run-unchanged")).getAllByRole("listitem").map((item) => item.textContent))
      .toEqual(["support:D"]);
    expect(within(review).getByText(HANGER_SIZING_RUN_UNCHANGED_TEXT)).toBeInTheDocument();
    expect(within(review).getByTestId("hanger-sizing-apply-run-preload-pending")).toHaveTextContent(HANGER_SIZING_PRELOAD_PENDING_TEXT);
    const variable = within(review).getByTestId("hanger-sizing-apply-run-support:B");
    expect(JSON.parse(within(variable).getByTestId("hanger-sizing-apply-run-after").textContent!).hanger.cold_load).toEqual({ value: 5197.5, unit: "N" });
    const constant = within(review).getByTestId("hanger-sizing-apply-run-support:C");
    expect(JSON.parse(within(constant).getByTestId("hanger-sizing-apply-run-before").textContent!)).toEqual({ family: "vertical_support", restraints: ["UZ"], provenance: "invented_test_value" });
    expect(within(constant).getByTestId("hanger-sizing-apply-run-source-note")).toHaveTextContent("Sized value: hanger.constant_load 3714.285714285714 N, the hot-load basis.");
    expect(within(review).queryByTestId("hanger-sizing-apply-run-support:D")).toBeNull();

    fireEvent.click(within(review).getByRole("button", { name: QUEUE_RUN }));
    expect(queue).toHaveBeenCalledTimes(1);
    const [batch] = queue.mock.calls[0];
    expect(batch.operations.map((operation: { target: { ref: string } }) => operation.target.ref)).toEqual(["support:B", "support:B", "support:C", "support:C"]);
    expect(screen.queryByTestId("hanger-sizing-apply-run-review")).toBeNull();
    expect(await canonicalJsonString(source)).toBe(original);
  });

  it("shows no preload sentence for a run whose selections are all constant (the product's four-span result)", async () => {
    const source = model();
    const lib = library();
    render(<HangerSizingApply document={productResult()} library={lib} model={source} busy={false} onQueueBatch={vi.fn()} />);
    fireEvent.click(await screen.findByRole("button", { name: PREPARE_RUN }));
    const review = await screen.findByTestId("hanger-sizing-apply-run-review");
    expect(within(review).getByTestId("hanger-sizing-apply-run-description")).toHaveTextContent("support:B, record CE-2 (constant effort); support:C, record CE-1 (constant effort)");
    expect(screen.queryByText(HANGER_SIZING_PRELOAD_PENDING_TEXT)).toBeNull();
  });

  it("offers a run of exactly one selection by the per-candidate act alone, the no-fit support named, and no whole-run act", async () => {
    const source = model();
    const lib = library();
    render(<HangerSizingApply document={await mixedRun(source, lib, ["support:B", "support:D"])} library={lib} model={source} busy={false} onQueueBatch={vi.fn()} />);
    expect(await screen.findByRole("button", { name: "Prepare applying record VS-200-L to support:B" })).toBeInTheDocument();
    expect(screen.getByTestId("hanger-sizing-apply-support:D")).toHaveTextContent("No record fits this support");
    expect(screen.queryByTestId("hanger-sizing-apply-run")).toBeNull();
    expect(screen.queryByRole("button", { name: PREPARE_RUN })).toBeNull();
    expect(screen.queryByText(HANGER_SIZING_APPLIED_WITH_RUN_TEXT, { exact: false })).toBeNull();
  });

  it("shows the preload sentence beside a single variable selection and not beside a single constant one", async () => {
    const source = model();
    const lib = library();
    render(<HangerSizingApply document={await mixedRun(source, lib, ["support:B"])} library={lib} model={source} busy={false} onQueueBatch={vi.fn()} />);
    fireEvent.click(await screen.findByRole("button", { name: "Prepare applying record VS-200-L to support:B" }));
    expect(await screen.findByTestId("hanger-sizing-apply-preload-pending")).toHaveTextContent(HANGER_SIZING_PRELOAD_PENDING_TEXT);
    cleanup();
    render(<HangerSizingApply document={await mixedRun(source, lib, ["support:C"])} library={lib} model={source} busy={false} onQueueBatch={vi.fn()} />);
    fireEvent.click(await screen.findByRole("button", { name: "Prepare applying record CE-1 to support:C" }));
    await screen.findByText(/Review the configuration replacement of support:C/);
    expect(screen.queryByText(HANGER_SIZING_PRELOAD_PENDING_TEXT)).toBeNull();
  });

  it("offers the run's act not at all once the model has changed, and drops a prepared run batch", async () => {
    const source = model();
    const lib = library();
    const document = await mixedRun(source, lib);
    const queue = vi.fn();
    const { rerender } = render(<HangerSizingApply document={document} library={lib} model={source} busy={false} onQueueBatch={queue} />);
    fireEvent.click(await screen.findByRole("button", { name: PREPARE_RUN }));
    await screen.findByTestId("hanger-sizing-apply-run-review");
    const changed = model();
    changed.supports.find((support) => support.id === "support:D")!.provenance = "edited after sizing";
    rerender(<HangerSizingApply document={document} library={lib} model={changed} busy={false} onQueueBatch={queue} />);
    expect(await screen.findByTestId("hanger-sizing-apply-model-changed")).toHaveTextContent(HANGER_SIZING_MODEL_CHANGED_TEXT);
    expect(screen.queryByTestId("hanger-sizing-apply-run-review")).toBeNull();
    expect(screen.queryByRole("button")).toBeNull();
    expect(queue).not.toHaveBeenCalled();
  });

  it("names the support that stops the run and offers no run act", async () => {
    const source = model();
    const lib = library();
    const document = await mixedRun(source, lib);
    const stopped = document.result.candidates[0] as { selected: { hanger_id: string; record_sha256: string } };
    stopped.selected.hanger_id = "VS-200-UY";
    stopped.selected.record_sha256 = await recordSha(lib, "VS-200-UY");
    render(<HangerSizingApply document={document} library={lib} model={source} busy={false} onQueueBatch={vi.fn()} />);
    expect(await screen.findByTestId("hanger-sizing-apply-run-not-offered")).toHaveTextContent(
      "support:B cannot be applied, for the reason given beside it. The run is applied whole or not at all, so none of its selections is offered."
    );
    expect(screen.getByTestId("hanger-sizing-apply-support:B")).toHaveTextContent("Record VS-200-UY declares its stiffness on UY, but support:B was sized on UZ");
    expect(screen.queryByRole("button", { name: PREPARE_RUN })).toBeNull();
  });
});
