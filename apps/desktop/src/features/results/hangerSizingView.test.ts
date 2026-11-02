// DEL-04-11 increment 3a: the hanger-sizing display adapter on the committed family fixtures
// (Design `hanger-sizing.md` §8.1, §8.3).
import { describe, expect, it } from "vitest";
import twoSpanJson from "../../../../../fixtures/results/hanger_sizing_result_two_span.json";
import fourSpanJson from "../../../../../fixtures/results/hanger_sizing_result_four_span.json";
import refusedJson from "../../../../../fixtures/results/hanger_sizing_result_refused.json";
import twoSpanInput from "../../../../../fixtures/results/hanger_sizing_input_two_span.json";
import fourSpanInput from "../../../../../fixtures/results/hanger_sizing_input_four_span.json";
import { canonicalSha256HexCheckedV1 } from "../../services/hashService";
import type { HangerSizingDocument } from "./hangerSizingResult";
import {
  F_COLD_CONDITION_LABEL,
  F_COLD_LABEL,
  HANGER_SIZING_STANDING_NAMES,
  NO_RATIO_FORMED,
  hangerSizingView,
  type CandidateRowView,
  type HangerSizingReaderEvidence,
  type HangerSizingRefusedView,
  type HangerSizingSizedView
} from "./hangerSizingView";

const clone = <T,>(value: T): T => structuredClone(value);
const asDocument = (value: unknown) => clone(value) as HangerSizingDocument;
const twoSpan = () => asDocument(twoSpanJson);
const fourSpan = () => asDocument(fourSpanJson);
const refused = () => asDocument(refusedJson);
const library = () => clone(twoSpanInput.library) as Record<string, unknown>;

async function sized(document: HangerSizingDocument, evidence?: HangerSizingReaderEvidence | null, lib?: unknown): Promise<HangerSizingSizedView> {
  const view = await hangerSizingView(document, evidence, lib);
  if (view.status !== "sized") throw new Error("expected a sized view");
  return view;
}

function deepFreeze<T>(value: T): T {
  if (typeof value === "object" && value !== null) {
    for (const key of Reflect.ownKeys(value)) deepFreeze((value as Record<PropertyKey, unknown>)[key]);
    Object.freeze(value);
  }
  return value;
}

const verdict = (difference: string | null = null): HangerSizingReaderEvidence["verdict"] => ({
  closure: { status: "passed", difference: null },
  binding: { status: "passed", difference: null, not_checked: ["solve_request"] },
  arithmetic_consistent_with_published_positions: { status: "passed", difference: null },
  derivation: { status: "not_checked", difference: null },
  reproduction: { status: difference === null ? "not_checked" : "failed", difference }
});

const numbersIn = (value: unknown, out: number[] = []): number[] => {
  if (typeof value === "number") out.push(value);
  else if (Array.isArray(value)) value.forEach((item) => numbersIn(item, out));
  else if (typeof value === "object" && value !== null) Object.values(value).forEach((item) => numbersIn(item, out));
  return out;
};

const row = (view: HangerSizingSizedView, support: string): CandidateRowView => {
  const found = view.rows.find((entry) => entry.support === support);
  if (!found) throw new Error(`no row ${support}`);
  return found;
};

describe("hangerSizingView: sized documents", () => {
  it("labels F_cold and F_C exactly and shows the variable selection's per-case lists in document order", async () => {
    const view = await sized(twoSpan(), null, library());
    expect(view.rows.map((entry) => entry.support)).toEqual(["support:B"]);
    const b = row(view, "support:B");
    expect(b.outcome).toBe("selected");
    expect(b.familyPolicy).toBe("variable");
    expect([b.node, b.dof]).toEqual(["node:B", "UZ"]);
    const selection = b.selection;
    if (selection?.family !== "variable_spring") throw new Error("variable selection expected");
    expect(selection.fCold).toEqual({ label: F_COLD_LABEL, quantity: { value: 5197.458880641297, unit: "N", text: "5197.458880641297 N" } });
    expect(selection.fColdCondition.label).toBe(F_COLD_CONDITION_LABEL);
    expect(F_COLD_LABEL).toBe("installed setting (at drawn position)");
    expect(F_COLD_CONDITION_LABEL).toBe("cold-condition load");
    expect(selection.k).toEqual({ value: 200000, unit: "N/m", text: "200000 N/m" });
    expect(selection.envelope.map((q) => q.value)).toEqual([4600, 5715.934209026074]);
    expect(selection.operating.map((entry) => entry.case)).toEqual(["load:O1", "load:O2"]);
    expect(selection.operating.map((entry) => entry.force.value)).toEqual([5000, 5315.934209026074]);
    expect(selection.operating[0].variation).toEqual({ state: "ratio", ratio: { value: 0.03949177612825936, unit: "ratio", text: "0.03949177612825936" } });
    expect(b.operating.map((entry) => [entry.case, entry.position.value])).toEqual([["load:O1", 0.000987294403206484], ["load:O2", -0.0005923766419238903]]);
    expect(b.governing).toEqual({ travelCase: "load:O1", variationCase: "load:O1" });
    expect(b.span.map((q) => q.value)).toEqual([-0.0005923766419238903, 0.000987294403206484]);
    expect(b.requiredTravel).toEqual({ value: 0.005579671045130375, unit: "m", text: "0.005579671045130375 m" });
    expect(b.verificationAssumption).toEqual({
      role: "spring",
      k: { value: 200000, unit: "N/m", text: "200000 N/m" },
      fCold: { label: F_COLD_LABEL, quantity: { value: 5197.458880641297, unit: "N", text: "5197.458880641297 N" } }
    });
    const verification = b.verification;
    if (verification?.family !== "variable_spring") throw new Error("variable verification expected");
    expect(verification.failedTests).toEqual([]);
    expect(verification.tPass).toBe(true);
    expect(verification.departure.value).toBe(0);
    expect(verification.fColdCondition).toEqual({ label: F_COLD_CONDITION_LABEL, quantity: { value: 5141.555929763012, unit: "N", text: "5141.555929763012 N" } });
    expect(verification.operating.map((entry) => entry.case)).toEqual(["load:O1", "load:O2"]);
    expect(b.failureCounts).toEqual({ variable: { state: "tested", counts: { E1: 1, E2: 1, L: 1, T: 1, V: 0, V0: 0 } }, constant: { state: "not_tested" } });
  });

  it("gives the basis block: every request input, the bound hashes, route, profile, cases and the document's own strings", async () => {
    const document = twoSpan();
    const view = await sized(document);
    expect(view.basis.sizingRequest).toEqual({
      weightCase: "load:W",
      weightSupportState: "own_solve",
      designOperatingCase: "load:O1",
      checkOperatingCases: ["load:O2"],
      candidates: [{ support: "support:B", familyPolicy: "variable" }],
      method: {
        loadDesign: "hot",
        variationReference: "operating_load",
        variationLimit: { value: 0.25, unit: "ratio", text: "0.25" },
        travelAllowance: { value: 0.002, unit: "m", text: "0.002 m" },
        selectionRule: "smallest_capacity"
      },
      solverMode: "dense_scrutiny"
    });
    expect(view.basis.library).toEqual({ libraryId: "invented.hangers.sizing", sha256: "e904866d00f0b665b9f4d7fc078b95fd583418d5b6174d09eb3bca0adfdc7861" });
    expect(view.basis.sizingRequestSha256).toBe(document.inputs.sizing_request_sha256);
    expect(view.basis.solveRequestSha256).toBe("966be96d808dae54a17143264c4648477647ad76c2b5d4ef44c61212bbbe6ac4");
    expect(view.basis.route).toBe("pressure_free");
    expect(view.basis.canonicalProfile).toBe("openpipestress_jcs_ijson_v1");
    expect(view.basis.basisStatement).toEqual(twoSpanJson.basis_statement);
    expect(view.basis.limitations).toEqual(twoSpanJson.limitations);
    expect(view.cases).toEqual([
      { case: "load:W", role: "weight", loadCategories: ["weight"], usesConditionSet: false },
      { case: "load:O1", role: "operating", loadCategories: ["concentrated_force", "weight"], usesConditionSet: false },
      { case: "load:O2", role: "operating", loadCategories: ["concentrated_force", "weight"], usesConditionSet: false }
    ]);
    expect(view.verificationStanding).toBe("unconditional");
    expect(view.disclosures).toEqual([]);
  });

  it("lists each solve with its label, case, derived hash and states, in document order", async () => {
    const view = await sized(twoSpan());
    expect(view.solves.map((solve) => solve.label)).toEqual(["S_W", "S_C", "S_O1", "S_O2", "V_C", "V_O1", "V_O2"]);
    expect(view.solves[3]).toEqual({
      label: "S_O2",
      case: "load:O2",
      derivedDocumentSha256: "590df0f06c498e3e017fcf697b75b6e2be034bf4ad931367b3b37731f7e4a615",
      states: [{ support: "support:C-lift", state: "active" }]
    });
    const four = await sized(fourSpan());
    expect(four.solves.every((solve) => solve.states.length === 0)).toBe(true);
  });

  it("shows a null variation as no ratio formed, never 0", async () => {
    const document = twoSpan() as unknown as { result: { candidates: { verification: { operating: { variation: number | null }[] } }[] } };
    document.result.candidates[0].verification.operating[1].variation = null;
    const view = await sized(document as unknown as HangerSizingDocument);
    const verification = view.rows[0].verification;
    if (verification?.family !== "variable_spring") throw new Error("variable verification expected");
    expect(verification.operating[1].variation).toEqual({ state: "no_ratio_formed", text: NO_RATIO_FORMED });
    expect(NO_RATIO_FORMED).toBe("no ratio formed");
    expect(verification.operating[0].variation.state).toBe("ratio");
  });

  it("shows the constant selection, the fallback, the no-fit row and null failure counts (four_span)", async () => {
    const view = await sized(fourSpan(), null, clone(fourSpanInput.library));
    expect(view.verificationStanding).toBe("conditional");
    expect(view.rows.map((entry) => [entry.support, entry.familyPolicy, entry.outcome])).toEqual([
      ["support:B", "variable_else_constant", "selected_by_fallback"],
      ["support:C", "constant", "selected"],
      ["support:D", "variable", "no_record_fits"]
    ]);
    const b = row(view, "support:B");
    expect(b.selection).toEqual({
      family: "constant_effort",
      hangerId: "CE-2",
      recordSha256: "76b38078001d0e7c6c339c987bdbb83f4d382cd4d81a72390c550f83e407e9e7",
      load: { value: 4571.428571428572, unit: "N", text: "4571.428571428572 N" }
    });
    expect(b.governing).toEqual({ travelCase: "load:O1", variationCase: null });
    expect(b.verificationAssumption).toEqual({ role: "force", force: { value: 4571.428571428572, unit: "N", text: "4571.428571428572 N" } });
    expect(b.verification?.family).toBe("constant_effort");
    expect(b.verification?.failedTests).toEqual([]);
    // A fallback shows both families' counts.
    expect(b.failureCounts.variable).toEqual({ state: "tested", counts: { E1: 1, E2: 1, L: 3, T: 1, V: 0, V0: 0 } });
    expect(b.failureCounts.constant).toEqual({ state: "tested", counts: { E1: 0, L: 1, T: 1 } });
    // Null counts are "not tested", distinct from zero counts.
    expect(row(view, "support:C").failureCounts.variable).toEqual({ state: "not_tested" });
    expect(row(view, "support:D").failureCounts.constant).toEqual({ state: "not_tested" });
    // No fit: the outcome is shown and nothing is put in its place.
    const d = row(view, "support:D");
    expect(d.selection).toBeNull();
    expect(d.governing).toBeNull();
    expect(d.verification).toBeNull();
    expect(d.record).toBeNull();
    expect(d.verificationAssumption).toEqual({ role: "force", force: { value: 4571.428571428572, unit: "N", text: "4571.428571428572 N" } });
    expect(d.w.value).toBe(4571.428571428572);
    expect(view.basis.sizingRequest?.checkOperatingCases).toEqual([]);
  });

  it("preserves every published number exactly, with no rounding", async () => {
    for (const document of [twoSpan(), fourSpan()]) {
      const view = await sized(document);
      const shown = numbersIn(view.rows);
      const published = numbersIn((document as HangerSizingDocument & { result: { candidates: unknown } }).result.candidates);
      for (const value of published) expect(shown.some((item) => Object.is(item, value))).toBe(true);
    }
    const four = await sized(fourSpan());
    expect(row(four, "support:B").uCold).toEqual({ value: 1.3469085432095433e-18, unit: "m", text: "1.3469085432095433e-18 m" });
    const two = await sized(twoSpan());
    expect(two.rows[0].uCold.value).toBe(5.942062022110673e-42);
    expect(two.rows[0].uCold.text).toBe("5.942062022110673e-42 m");
    const verification = two.rows[0].verification;
    if (verification?.family !== "variable_spring") throw new Error("variable verification expected");
    expect(verification.operating[0].position.value).toBe(0.0009872944032064837);
    expect(two.rows[0].operating[0].position.value).toBe(0.000987294403206484);
  });

  it("does not mutate its inputs", async () => {
    const document = deepFreeze(fourSpan());
    const lib = deepFreeze(clone(fourSpanInput.library));
    const hash = await canonicalSha256HexCheckedV1(document);
    const evidence = deepFreeze({ document_sha256: hash, verdict: verdict() });
    const before = JSON.stringify([document, lib, evidence]);
    const view = await sized(document, evidence, lib);
    expect(JSON.stringify([document, lib, evidence])).toBe(before);
    expect(view.readerStandings.binding.state).toBe("bound");
    expect(row(view, "support:B").record?.state).toBe("shown");
    // The view shares no arrays with the document.
    expect(view.basis.limitations).not.toBe(document.limitations);
    expect(view.cases[1].loadCategories).not.toBe(document.status === "sized" ? document.result.cases[1].load_categories : null);
    const refusedDoc = deepFreeze(refused());
    const refusedBefore = JSON.stringify(refusedDoc);
    await hangerSizingView(refusedDoc, null);
    expect(JSON.stringify(refusedDoc)).toBe(refusedBefore);
  });
});

describe("hangerSizingView: library detail gate", () => {
  it("shows record details from the exact bound library, unconverted", async () => {
    const view = await sized(twoSpan(), null, library());
    expect(view.library).toEqual({ state: "bound", libraryId: "invented.hangers.sizing", sha256: "e904866d00f0b665b9f4d7fc078b95fd583418d5b6174d09eb3bca0adfdc7861" });
    expect(view.rows[0].record).toEqual({
      state: "shown",
      hangerId: "VS-200-L",
      recordSha256: "008305de2dab833901ab4d9d4bd7bf29f7399670f12c9cedf7714502bb1683de",
      name: "Invented VS-200-L",
      hangerType: "variable_spring_hanger",
      stiffness: { magnitude: 200000, unit: "N/m", text: "200000 N/m" },
      workingLoadRange: { minimum: { magnitude: 4000, unit: "N", text: "4000 N" }, maximum: { magnitude: 6000, unit: "N", text: "6000 N" } },
      travelRange: { magnitude: 50, unit: "mm", text: "50 mm" }
    });
    const four = await sized(fourSpan(), null, clone(fourSpanInput.library));
    const c = row(four, "support:C").record;
    expect(c).toMatchObject({ state: "shown", hangerId: "CE-1", stiffness: null });
  });

  it("names library_not_supplied when no library is given", async () => {
    for (const absent of [undefined, null]) {
      const view = await sized(fourSpan(), null, absent);
      expect(view.library).toEqual({ state: "not_shown", reason: "library_not_supplied" });
      expect(row(view, "support:B").record).toEqual({ state: "not_shown", hangerId: "CE-2", reason: "library_not_supplied" });
      expect(row(view, "support:C").record).toEqual({ state: "not_shown", hangerId: "CE-1", reason: "library_not_supplied" });
      expect(row(view, "support:D").record).toBeNull();
    }
  });

  it("names library_not_hashable for a library outside the checked profile", async () => {
    const unsafe = library();
    (unsafe.hanger_records as Record<string, unknown>[])[0].name = Number.NaN;
    const big = library();
    (big.hanger_library as Record<string, unknown>).extra = 2 ** 60;
    const cyclic = library();
    cyclic.self = cyclic;
    for (const lib of [unsafe, big, cyclic]) {
      const view = await sized(twoSpan(), null, lib);
      expect(view.library).toEqual({ state: "not_shown", reason: "library_not_hashable" });
      expect(view.rows[0].record).toEqual({ state: "not_shown", hangerId: "VS-200-L", reason: "library_not_hashable" });
    }
  });

  it("names library_hash_mismatch for any other library, with no partial display", async () => {
    const renamed = library();
    (renamed.hanger_records as Record<string, unknown>[])[8].name = "Changed elsewhere";
    const reordered = library();
    (reordered.hanger_records as unknown[]).reverse();
    for (const lib of [renamed, reordered, "not a library", { hanger_records: [] }]) {
      const view = await sized(twoSpan(), null, lib);
      expect(view.library).toEqual({ state: "not_shown", reason: "library_hash_mismatch" });
      expect(view.rows[0].record).toEqual({ state: "not_shown", hangerId: "VS-200-L", reason: "library_hash_mismatch" });
    }
  });

  it("names library_id_mismatch when the document's id disagrees with the hash-matched library", async () => {
    const document = twoSpan() as unknown as { inputs: { library: { library_id: string } } };
    document.inputs.library.library_id = "another.library";
    const view = await sized(document as unknown as HangerSizingDocument, null, library());
    expect(view.library).toEqual({ state: "not_shown", reason: "library_id_mismatch" });
    expect(view.rows[0].record).toEqual({ state: "not_shown", hangerId: "VS-200-L", reason: "library_id_mismatch" });
  });

  it("does not show a record whose hash differs from record_sha256, or that is not found", async () => {
    const mismatched = twoSpan() as unknown as { result: { candidates: { selected: { record_sha256: string; hanger_id: string } }[] } };
    mismatched.result.candidates[0].selected.record_sha256 = "0".repeat(64);
    const view = await sized(mismatched as unknown as HangerSizingDocument, null, library());
    expect(view.library.state).toBe("bound");
    expect(view.rows[0].record).toEqual({ state: "not_shown", hangerId: "VS-200-L", reason: "record_hash_mismatch" });
    // Another record's hash under this id is still a mismatch.
    const other = twoSpan() as unknown as typeof mismatched;
    other.result.candidates[0].selected.record_sha256 = (fourSpanJson.result.candidates[0].selected as { record_sha256: string }).record_sha256;
    expect((await sized(other as unknown as HangerSizingDocument, null, library())).rows[0].record).toMatchObject({ state: "not_shown", reason: "record_hash_mismatch" });
    const missing = twoSpan() as unknown as typeof mismatched;
    missing.result.candidates[0].selected.hanger_id = "VS-ABSENT";
    expect((await sized(missing as unknown as HangerSizingDocument, null, library())).rows[0].record).toEqual({ state: "not_shown", hangerId: "VS-ABSENT", reason: "record_not_found" });
  });
});

describe("hangerSizingView: reader standings as bound evidence", () => {
  it("shows supplied standings only when their document hash is this document's checked hash", async () => {
    const document = twoSpan();
    const hash = await canonicalSha256HexCheckedV1(document);
    const view = await sized(document, { document_sha256: hash, verdict: verdict("V_O1: reproduced value differs") });
    expect(view.readerStandings.binding).toEqual({ state: "bound", documentSha256: hash });
    expect(view.readerStandings.standings).toEqual([
      { name: "closure", status: "passed", difference: null },
      { name: "binding", status: "passed", difference: null, notChecked: ["solve_request"] },
      { name: "arithmetic_consistent_with_published_positions", status: "passed", difference: null },
      { name: "derivation", status: "not_checked", difference: null },
      { name: "reproduction", status: "failed", difference: "V_O1: reproduced value differs" }
    ]);
  });

  const allNotChecked = HANGER_SIZING_STANDING_NAMES.map((name) => ({ name, status: "not_checked", difference: null }));

  it("shows every standing as not_checked without evidence", async () => {
    for (const absent of [undefined, null]) {
      const view = await sized(twoSpan(), absent);
      expect(view.readerStandings).toEqual({ binding: { state: "unbound", reason: "evidence_not_supplied" }, standings: allNotChecked });
    }
  });

  it("shows every standing as not_checked for evidence bound to another document, however it reads", async () => {
    const twoHash = await canonicalSha256HexCheckedV1(twoSpan());
    const four = await sized(fourSpan(), { document_sha256: twoHash, verdict: verdict() });
    expect(four.readerStandings).toEqual({ binding: { state: "unbound", reason: "evidence_bound_to_other_document" }, standings: allNotChecked });
    // Evidence computed before a one-number change no longer binds.
    const changed = twoSpan() as unknown as { result: { candidates: { w_n: number }[] } };
    changed.result.candidates[0].w_n = 5000.000000000001;
    const stale = await sized(changed as unknown as HangerSizingDocument, { document_sha256: twoHash, verdict: verdict() });
    expect(stale.readerStandings.binding).toEqual({ state: "unbound", reason: "evidence_bound_to_other_document" });
    expect(stale.readerStandings.standings).toEqual(allNotChecked);
  });

  it("shows every standing as not_checked for malformed evidence or a caller flag", async () => {
    const hash = await canonicalSha256HexCheckedV1(twoSpan());
    const malformed: unknown[] = [
      { trusted: true },
      { valid: true, document_sha256: hash },
      { document_sha256: hash },
      { document_sha256: hash.toUpperCase(), verdict: verdict() },
      { document_sha256: hash, verdict: { ...verdict(), closure: { status: "ok", difference: null } } },
      { document_sha256: hash, verdict: { ...verdict(), binding: { status: "passed", difference: null } } },
      true,
      "passed"
    ];
    for (const evidence of malformed) {
      const view = await sized(twoSpan(), evidence as HangerSizingReaderEvidence);
      expect(view.readerStandings).toEqual({ binding: { state: "unbound", reason: "evidence_malformed" }, standings: allNotChecked });
    }
  });
});

describe("hangerSizingView: refused documents", () => {
  it("lists every refusal in order with the bound inputs as published", async () => {
    const view = await hangerSizingView(refused(), null);
    expect(view.status).toBe("refused");
    const refusedView = view as HangerSizingRefusedView;
    expect(refusedView.refusals).toEqual([
      { code: "HANGER_SIZING_CANDIDATE_UNKNOWN", names: ["support:missing"], message: "the candidate does not meet the sizing rules of Design §3" }
    ]);
    expect(refusedView.basis.sizingRequest?.candidates).toEqual([
      { support: "support:B", familyPolicy: "variable" },
      { support: "support:missing", familyPolicy: "constant" }
    ]);
    expect(refusedView.basis.sizingRequestSha256).toBe("b5343f6afddd1716902e57ce0c59359768ecd1ce89442b460e2a6fdde2c40b47");
    expect(refusedView.basis.limitations).toEqual(refusedJson.limitations);
    expect(refusedView.basis.basisStatement).toEqual(refusedJson.basis_statement);
    expect(refusedView.readerStandings.binding).toEqual({ state: "unbound", reason: "evidence_not_supplied" });
    expect("rows" in refusedView).toBe(false);
  });

  it("shows null bound inputs as null and keeps several refusals in order", async () => {
    const document = refused() as unknown as Record<string, unknown> & { inputs: Record<string, unknown>; refusals: unknown[] };
    document.inputs.sizing_request = null;
    document.inputs.sizing_request_sha256 = null;
    document.inputs.solve_request = { sha256: null };
    document.inputs.library = { library_id: null, sha256: null };
    document.refusals = [
      { code: "HANGER_SIZING_INPUT_NOT_ADMITTED", names: ["duplicate_key", "$.sizing.method"], message: "the input is not admitted" },
      { code: "HANGER_SIZING_LIBRARY_NOT_ADMITTED", names: [], message: "the library is not admitted" }
    ];
    const hash = await canonicalSha256HexCheckedV1(document);
    const view = (await hangerSizingView(document as unknown as HangerSizingDocument, { document_sha256: hash, verdict: verdict() })) as HangerSizingRefusedView;
    expect(view.basis.sizingRequest).toBeNull();
    expect(view.basis.sizingRequestSha256).toBeNull();
    expect(view.basis.solveRequestSha256).toBeNull();
    expect(view.basis.library).toEqual({ libraryId: null, sha256: null });
    expect(view.refusals.map((refusal) => refusal.code)).toEqual(["HANGER_SIZING_INPUT_NOT_ADMITTED", "HANGER_SIZING_LIBRARY_NOT_ADMITTED"]);
    expect(view.refusals[1].names).toEqual([]);
    expect(view.readerStandings.binding).toEqual({ state: "bound", documentSha256: hash });
  });
});
