// DEL-04-11 REQ-008 (review round 1 of pull request 258, finding 1): a sized selection is offered, and a batch prepared,
// only while the library the person has stored now under the id the result names is the one the result binds. The
// model and library are the invented four-span sizing input (read, not changed); the run (a variable and a constant
// selection and one no-fit support) is formed here and bound by the desktop's checked hash, as in
// `runApplication.test.ts`. A revised library is the same library with another name, under the same id.
import { cleanup, fireEvent, render, screen } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import fourSpanInput from "../../../../../fixtures/results/hanger_sizing_input_four_span.json";
import { canonicalSha256HexCheckedV1 } from "../../services/hashService";
import type { PreviewModel } from "../../types";
import type { HangerSizingCandidate, HangerSizingSizedDocument } from "../results/hangerSizingResult";
import { HangerSizingApply } from "./HangerSizingApply";
import { HANGER_SIZING_LIBRARY_CHANGED_TEXT, HANGER_SIZING_LIBRARY_UNAVAILABLE_TEXT, type StoredLibraryRead } from "./sizedSelection";

afterEach(cleanup);

const model = () => structuredClone(fourSpanInput.request.model) as unknown as PreviewModel;
const library = () => structuredClone(fourSpanInput.library) as Record<string, unknown>;
const revised = () => {
  const changed = library();
  (changed.hanger_library as Record<string, unknown>).name = "Invented hanger sizing fixture, revised";
  return changed;
};
const SINGLE = "Prepare applying record VS-200-L to support:B";
const RUN = "Prepare applying every selection of this run";
// The per-candidate act is offered for a run of exactly one selection (support:B, with support:D unfitted); the
// apply-all act for the run of two.
const ACTS = [
  { act: "the per-candidate act", prepare: SINGLE, review: "hanger-sizing-apply-review", keep: ["support:B", "support:D"] },
  { act: "the apply-all act", prepare: RUN, review: "hanger-sizing-apply-run-review", keep: ["support:B", "support:C", "support:D"] }
] as const;

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

const stored = (document: unknown): (() => Promise<StoredLibraryRead>) => vi.fn(async () => ({ kind: "read" as const, document }));

describe("the library a sized selection is bound to", () => {
  it("offers nothing when the library the form holds under that id has been revised since the run", async () => {
    const source = model();
    const lib = library();
    const document = await mixedRun(source, lib);
    render(<HangerSizingApply document={document} library={lib} currentLibrary={revised()} readStoredLibrary={stored(revised())} model={source} busy={false} onQueueBatch={vi.fn()} />);
    expect(await screen.findByTestId("hanger-sizing-apply-library-changed")).toHaveTextContent(HANGER_SIZING_LIBRARY_CHANGED_TEXT);
    expect(screen.queryByRole("button")).toBeNull();
  });

  for (const { act, prepare, review, keep } of ACTS) {
    it(`withdraws ${act} when the stored library, read again at the act, has been revised, is missing or cannot be read`, async () => {
      const source = model();
      const lib = library();
      const document = await mixedRun(source, lib, keep);
      for (const [read, text] of [
        [stored(revised()), HANGER_SIZING_LIBRARY_CHANGED_TEXT],
        [vi.fn(async () => ({ kind: "unavailable" as const })), HANGER_SIZING_LIBRARY_UNAVAILABLE_TEXT]
      ] as const) {
        const queue = vi.fn();
        // The form still holds the library as it was when sizing ran; the store has moved on.
        render(<HangerSizingApply document={document} library={lib} currentLibrary={library()} readStoredLibrary={read} model={source} busy={false} onQueueBatch={queue} />);
        fireEvent.click(await screen.findByRole("button", { name: prepare }));
        expect(await screen.findByTestId("hanger-sizing-apply-library-changed")).toHaveTextContent(text);
        expect(read).toHaveBeenCalledTimes(1);
        expect(screen.queryByTestId(review)).toBeNull();
        expect(screen.queryByRole("button")).toBeNull();
        expect(queue).not.toHaveBeenCalled();
        cleanup();
      }
    });

    it(`drops a batch prepared by ${act} when the library the form holds under that id is revised, so it cannot be queued`, async () => {
      const source = model();
      const lib = library();
      const document = await mixedRun(source, lib, keep);
      const current = library();
      const queue = vi.fn();
      const { rerender } = render(<HangerSizingApply document={document} library={lib} currentLibrary={current} readStoredLibrary={stored(current)} model={source} busy={false} onQueueBatch={queue} />);
      fireEvent.click(await screen.findByRole("button", { name: prepare }));
      await screen.findByTestId(review);
      const changed = revised();
      rerender(<HangerSizingApply document={document} library={lib} currentLibrary={changed} readStoredLibrary={stored(changed)} model={source} busy={false} onQueueBatch={queue} />);
      expect(await screen.findByTestId("hanger-sizing-apply-library-changed")).toHaveTextContent(HANGER_SIZING_LIBRARY_CHANGED_TEXT);
      expect(screen.queryByTestId(review)).toBeNull();
      expect(screen.queryByRole("button")).toBeNull();
      expect(queue).not.toHaveBeenCalled();
    });
  }

  it("drops a prepared batch when the form no longer holds the library (another library chosen), and prepares again only against the stored library", async () => {
    const source = model();
    const lib = library();
    const document = await mixedRun(source, lib);
    const read = stored(library());
    const { rerender } = render(<HangerSizingApply document={document} library={lib} currentLibrary={library()} readStoredLibrary={read} model={source} busy={false} onQueueBatch={vi.fn()} />);
    fireEvent.click(await screen.findByRole("button", { name: RUN }));
    await screen.findByTestId("hanger-sizing-apply-run-review");
    rerender(<HangerSizingApply document={document} library={lib} currentLibrary={undefined} readStoredLibrary={read} model={source} busy={false} onQueueBatch={vi.fn()} />);
    expect(screen.queryByTestId("hanger-sizing-apply-run-review")).toBeNull();
    fireEvent.click(await screen.findByRole("button", { name: RUN }));
    expect(await screen.findByTestId("hanger-sizing-apply-run-review")).toBeInTheDocument();
    expect(read).toHaveBeenCalledTimes(2);
  });
});
