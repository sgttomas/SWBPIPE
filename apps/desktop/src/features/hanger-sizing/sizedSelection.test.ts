// DEL-04-11 REQ-008 (review round 1 of pull request 258, finding 3): which published member each sized load is taken
// from. No committed product result has a variable selection under cold design or an operating array whose design case
// is not first (the two-span results are hot design with the design case first; the four-span result is cold design
// with constant selections only), so these documents are hand-built. They test only the selection of a published
// member by its name, not mechanics: every number is invented and chosen to be distinct from every other, so a wrong
// member cannot pass. The model and library are the invented two-span sizing input (read, not changed).
import { describe, expect, it } from "vitest";
import twoSpanInput from "../../../../../fixtures/results/hanger_sizing_input_two_span.json";
import fourSpanInput from "../../../../../fixtures/results/hanger_sizing_input_four_span.json";
import fourSpanResult from "../../../../../fixtures/results/hanger_sizing_result_four_span.json";
import { canonicalSha256HexCheckedV1 } from "../../services/hashService";
import type { PreviewModel } from "../../types";
import { buildHangerSelectionBatch } from "../hanger-selection/hangerSelection";
import type { HangerSizingCandidate, HangerSizingSizedDocument } from "../results/hangerSizingResult";
import { asHangerDocument, hangerSizingApplyOffers } from "./sizedSelection";

const model = () => structuredClone(twoSpanInput.request.model) as unknown as PreviewModel;
const library = () => structuredClone(twoSpanInput.library) as Record<string, unknown>;

const W = 5001.25;
const F_COLD = 5197.75;
const F_COLD_CONDITION = 5203.5;
const DESIGN_LOAD = 4987.125;
const OTHER_LOAD = 5316.375;

async function coldDesign(source: PreviewModel, lib: Record<string, unknown>, operating: { case: string; f_n: number; variation: number }[]): Promise<HangerSizingSizedDocument> {
  const record = (lib.hanger_records as { hanger_id: string }[]).find((entry) => entry.hanger_id === "VS-200-L");
  const candidate: HangerSizingCandidate = {
    support: "support:B", node: "node:B", dof: "UZ", family_policy: "variable", w_n: W, u_cold_m: 0.0003, operating: [], span_m: [0, 0.001],
    required_travel_m: 0.005, failure_counts: { variable: null, constant: null }, outcome: "selected",
    selected: { family: "variable_spring", hanger_id: "VS-200-L", record_sha256: await canonicalSha256HexCheckedV1(record), k_n_per_m: 200000,
      f_cold_n: F_COLD, f_cold_condition_n: F_COLD_CONDITION, envelope_n: [4600, 5716], operating },
    governing: { travel_case: "load:O1", variation_case: "load:O1" },
    verification_assumption: { role: "spring", k_n_per_m: 200000, f_cold_n: F_COLD },
    verification: null as never
  };
  return {
    status: "sized",
    inputs: {
      canonical_profile: "openpipestress_jcs_ijson_v1",
      sizing_request: { design_operating_case: "load:O1", method: { load_design: "cold" } },
      sizing_request_sha256: "6".repeat(64),
      solve_request: { sha256: await canonicalSha256HexCheckedV1({ model: source, materials: [] }) },
      library: { library_id: "invented.hangers.sizing", sha256: await canonicalSha256HexCheckedV1(lib) }
    },
    result: { candidates: [candidate] }
  } as unknown as HangerSizingSizedDocument;
}

describe("the sized loads a selection is applied with", () => {
  it("under cold design, takes the hot load from the selection's operating load in the named design case, found by name though not first", async () => {
    const source = model();
    const lib = library();
    // The design case load:O1 is second; the first entry is another operating case.
    const document = await coldDesign(source, lib, [{ case: "load:O2", f_n: OTHER_LOAD, variation: 0.02 }, { case: "load:O1", f_n: DESIGN_LOAD, variation: 0.04 }]);
    const { offers } = await hangerSizingApplyOffers(document, source, lib);
    const [offer] = offers;
    if (offer.kind !== "offered") throw new Error(offer.reason);
    expect(offer.application.loads).toEqual({ family: "variable_spring", coldLoadN: F_COLD, hotLoadN: DESIGN_LOAD, designOperatingCase: "load:O1" });
    // Not W, not the cold load, not the cold-condition load, not the first operating entry.
    for (const wrong of [W, F_COLD, F_COLD_CONDITION, OTHER_LOAD]) expect(offer.application.loads).not.toMatchObject({ hotLoadN: wrong });
    const batch = await buildHangerSelectionBatch(source, asHangerDocument(lib), offer.hangerId, offer.target, offer.application);
    const hanger = JSON.parse(batch.operations[0].change.after).hanger;
    expect(hanger.hot_load).toEqual({ value: DESIGN_LOAD, unit: "N" });
    expect(hanger.cold_load).toEqual({ value: F_COLD, unit: "N" });
    expect(hanger.installed_load).toEqual({ value: F_COLD, unit: "N" });
    expect(batch.operations[0].change.source_note).toContain(`hanger.hot_load ${DESIGN_LOAD} N, the sized operating load in the design operating case load:O1`);
  });

  it("offers nothing when the named design case is absent from the operating loads, whatever else they hold", async () => {
    const source = model();
    const lib = library();
    const document = await coldDesign(source, lib, [{ case: "load:O2", f_n: OTHER_LOAD, variation: 0.02 }]);
    const [offer] = (await hangerSizingApplyOffers(document, source, lib)).offers;
    expect(offer).toEqual({ kind: "not_offered", support: "support:B", reason: "The result gives no single load of support:B in the design operating case load:O1, so it is not offered." });
  });

  it("applies a constant selection of the product's cold-design result with its published hot-load basis", async () => {
    // The product's four-span result is cold design (`method.load_design: cold`); a constant selection's load is its
    // hot-load basis, published as `selected.load_n` and equal to `w_n`.
    const source = structuredClone(fourSpanInput.request.model) as unknown as PreviewModel;
    const lib = structuredClone(fourSpanInput.library) as Record<string, unknown>;
    const document = structuredClone(fourSpanResult) as unknown as HangerSizingSizedDocument;
    expect(document.inputs.sizing_request.method.load_design).toBe("cold");
    const offers = (await hangerSizingApplyOffers(document, source, lib)).offers;
    const loads = offers.map((offer) => (offer.kind === "offered" ? [offer.support, offer.application.loads] : [offer.support, null]));
    expect(loads).toEqual([
      ["support:B", { family: "constant_effort", constantLoadN: 4571.428571428572 }],
      ["support:C", { family: "constant_effort", constantLoadN: 3714.285714285714 }],
      ["support:D", null]
    ]);
  });
});
