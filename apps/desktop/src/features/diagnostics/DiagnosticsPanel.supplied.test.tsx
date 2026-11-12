// RT-A3 §4 (DEL-05-02 Work `combination-certificate.md` §2, the residual text boundary; review cert3
// findings 1–3): the Diagnostics panel lists and explains a result message outside the reader-checked
// combination roster as supplied, unverified text, its source text quoted whole, by the classification
// the report uses (`suppliedDiagnostics`). Nothing is read from a message's wording.
import { render, screen } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import corpus from "../../../../../fixtures/results/combination_certificate_corpus.json";
import successor from "../../../../../fixtures/results/retained_precision_combination_successor_sparse_interactive.json";
import legacySuccessor from "../../../../../fixtures/results/retained_precision_combination_successor_sparse_interactive_pre_certificate.json";
import { registerRetainedPrecision, retainedPrecisionRegistration } from "../results/retainedPrecisionStanding";
import { DiagnosticsPanel } from "./DiagnosticsPanel";
import { buildDiagnosticInterpretation } from "../results/resultInterpretation";
import { readerValidate, ResultReaderError } from "../results/resultReader";
import { CAP_REMAINDER_WARNING_CODE, documentCombinationIdentities, presentedResultDiagnostics, SUPPLEMENTAL_DIAGNOSTIC_LABEL, suppliedDiagnosticMessage } from "../results/suppliedDiagnostics";
import { diagnosticsForSections } from "../report/renderableReportInput";
import type { Diagnostic, MechanicsResult, PreviewModel } from "../../types";

const CLAIM = "combination comb:cancel: all derived rows are bounded by 1e-30.";
const supplied = (message: string) => `${SUPPLEMENTAL_DIAGNOSTIC_LABEL}"${message}"`;
const PROVENANCE = {
  source_name: "invented session", source_location: "local desktop session", source_license: "user_supplied_or_private",
  contributor: "user_local_session", contributor_certification: "not_asserted", redistribution_status: "private_only",
  review_status: "pending", privacy_classification: "private_project_data"
};
type Entry = { name: string; document: MechanicsResult; model: Record<string, unknown> & { materials: Record<string, unknown>[] } };

function exCantilever(): Entry {
  return structuredClone((corpus as unknown as { documents: Entry[] }).documents.find((e) => e.name === "ex_cantilever")!);
}
function note(refs: string[], id = "diagnostic:review:claim", message = CLAIM): Diagnostic {
  return { id, code: "REVIEW_NOTE", severity: "info", source: "core/product_physics", affected_refs: refs, message } as Diagnostic;
}
function panelModel(result: MechanicsResult): PreviewModel {
  return { diagnostics: [], project: { id: (result as { model_ref?: string }).model_ref }, rule_pack_refs: [], library_refs: [] } as unknown as PreviewModel;
}
/** The panel with `selected` chosen: its list entry's message and its explanation. */
function shown(result: MechanicsResult, selected: Diagnostic) {
  const view = render(<DiagnosticsPanel model={panelModel(result)} knowledge={null} result={result} selectedDiagnosticId={selected.id!} onSelectDiagnostic={() => {}} />);
  const listed = screen.getAllByTestId(`diagnostic-${selected.code}`).map((item) => item.querySelector("p")!.textContent);
  const explanation = screen.getByTestId("selected-diagnostic-explanation").querySelector("dd")!.textContent!;
  view.unmount();
  return { listed, explanation };
}

describe("Diagnostics panel: supplied diagnostic text", () => {
  it("quotes the review's EX cantilever model-ref note as supplied in the list and the explanation", () => {
    const e = exCantilever();
    const result = e.document;
    const claim = note(["model"]);
    result.diagnostics.push(claim);
    expect(readerValidate("pressure", result)).toBeDefined();
    const { listed, explanation } = shown(result, claim);
    expect(listed).toEqual([supplied(CLAIM)]);
    expect(explanation.startsWith(`${supplied(CLAIM)} It is linked to model.`)).toBe(true);
    // A roster entry the reader admitted is copied; a message naming no combination is supplied text.
    const presented = presentedResultDiagnostics(result);
    result.diagnostics.forEach((d, index) => {
      const names = (d.affected_refs ?? []).some((ref) => documentCombinationIdentities(result).has(ref));
      expect(presented[index].message, d.id).toBe(names ? d.message : supplied(d.message));
    });
    const certificate = result.diagnostics.find((d) => d.code === "COMBINATION_ROWS_WITHIN_STATED_BOUND" && d.affected_refs?.[0] === "comb:cancel")!;
    expect(shown(result, certificate).listed).toContain(certificate.message);
    // The interpretation reads the same classification; nothing else in it changes.
    const interpretation = buildDiagnosticInterpretation({ model: panelModel(result), knowledge: null, result, diagnosticId: claim.id! })!;
    expect(interpretation.message).toBe(supplied(CLAIM));
    expect(interpretation.affected_refs).toEqual(["model"]);
  });

  it("does not let a model material sharing a combination id unroster a claim (review cert3 finding 2)", () => {
    const e = exCantilever();
    const model = { ...e.model, materials: [...e.model.materials, { ...e.model.materials[0], id: "comb:cancel" }] };
    // The canonical document with the colliding model: admitted, its roster entries copied by the report.
    expect(readerValidate("pressure", e.document, model)).toBeDefined();
    expect(documentCombinationIdentities(e.document).has("comb:cancel")).toBe(true);
    // A claim under that id: both readings refuse it as a roster extra, so no reader-checked population
    // leaves it out while the report's population names it.
    const result = structuredClone(e.document);
    const claim = note(["comb:cancel"], "diagnostic:review:collision");
    result.diagnostics.push(claim);
    for (const reading of [undefined, model]) {
      let refusal: unknown = null;
      try { readerValidate("pressure", result, reading); } catch (error) { refusal = error; }
      expect((refusal as ResultReaderError | null)?.code).toBe("SOURCE_COMBINATION_CERTIFICATE_ROSTER_EXTRA");
    }
    // The panel, on the refused result, checks no roster entry: every message is supplied text.
    const { listed, explanation } = shown(result, claim);
    expect(listed).toEqual([supplied(CLAIM)]);
    expect(explanation.startsWith(supplied(CLAIM))).toBe(true);
    expect(presentedResultDiagnostics(result).map((d) => d.message)).toEqual(result.diagnostics.map((d) => supplied(d.message)));
    // The report's source-only projection names comb:cancel as the readers do; it is admitted only after the
    // reader, which refuses this source.
    expect(diagnosticsForSections(result, PROVENANCE).at(-1)!.message).toBe(CLAIM);
  });

  it("quotes a retained successor's combination prose as supplied: changed selected prose and an appended note", () => {
    const changed = structuredClone(successor.source) as unknown as MechanicsResult;
    const selected = changed.diagnostics.find((d) => d.code === "RETAINED_PRECISION_SELECTED" && d.affected_refs?.[0] === "combination:ab")!;
    const prose = "combination combination:ab: all derived rows are bounded by 1e-30.";
    selected.message = prose;
    expect(shown(changed, selected).listed).toContain(supplied(prose));
    expect(shown(changed, selected).explanation.startsWith(supplied(prose))).toBe(true);

    const appended = structuredClone(successor.source) as unknown as MechanicsResult & { retained_precision: { body: { combinations: { basis_ref: { ref_id: string }; diagnostic_refs: string[] }[] } } };
    const claim = note(["combination:ab"], "diagnostic:review:unknown", prose);
    appended.diagnostics.push(claim);
    appended.retained_precision.body.combinations.find((c) => c.basis_ref.ref_id === "combination:ab")!.diagnostic_refs.push(claim.id!);
    const { listed, explanation } = shown(appended, claim);
    expect(listed).toEqual([supplied(prose)]);
    expect(explanation.startsWith(supplied(prose))).toBe(true);
    // The retained base read does not check the roster's text, so no message of a successor is copied.
    expect(presentedResultDiagnostics(appended).map((d) => d.message)).toEqual(appended.diagnostics.map((d) => supplied(d.message)));
  });

  it("copies a registered retained successor's certificate notices and nothing else (§8.53 D3 (b))", async () => {
    // On the registered build the successor carries the ordinary run's certificate member; the retained reader's G7
    // base read validates it whole on the projection, whose diagnostics are the successor's own, so the notices in
    // the certificate's typed vocabulary are exactly the ones the validated member re-derives.
    const CERTIFICATE = "diagnostic:combination-certificate:";
    const doc = structuredClone(successor) as unknown as { source: MechanicsResult; invocation: unknown };
    const source = doc.source;
    const notices = source.diagnostics.filter((d) => d.id?.startsWith(CERTIFICATE));
    expect(notices.map((d) => d.code)).toEqual(["COMBINATION_ROWS_NOT_ASSESSED"]);
    expect((source.contract_evidence as Record<string, unknown>).combination_certificate).toBeDefined();
    // Unregistered: every message of a successor with combinations is supplied text, the notices included.
    expect(presentedResultDiagnostics(source).map((d) => d.message)).toEqual(source.diagnostics.map((d) => supplied(d.message)));
    await registerRetainedPrecision(source, doc.invocation);
    expect(retainedPrecisionRegistration(source)?.validation).toBeTruthy();
    const presented = presentedResultDiagnostics(source);
    source.diagnostics.forEach((d, index) => {
      expect(presented[index].message, d.id).toBe(d.id?.startsWith(CERTIFICATE) ? d.message : supplied(d.message));
    });
    expect(shown(source, notices[0]).listed).toEqual([notices[0].message]);
    // Bytes the registration did not validate (a forged notice in the vocabulary appended) are not attested.
    const forged = structuredClone(source);
    forged.diagnostics.push({ ...notices[0], id: `${CERTIFICATE}99:combination:ab`, message: CLAIM });
    expect(retainedPrecisionRegistration(forged)).toBeNull();
    expect(presentedResultDiagnostics(forged).map((d) => d.message)).toEqual(forged.diagnostics.map((d) => supplied(d.message)));
    // The legacy successor (registered before the certificate, no member) makes no certificate claim: registered,
    // every message stays supplied.
    const legacy = structuredClone(legacySuccessor) as unknown as { source: MechanicsResult; invocation: unknown };
    await registerRetainedPrecision(legacy.source, legacy.invocation);
    expect(retainedPrecisionRegistration(legacy.source)?.validation).toBeTruthy();
    expect(legacy.source.diagnostics.some((d) => d.id?.startsWith(CERTIFICATE))).toBe(false);
    expect(presentedResultDiagnostics(legacy.source).map((d) => d.message)).toEqual(legacy.source.diagnostics.map((d) => supplied(d.message)));
  });

  it("leaves a result without combinations unchanged", () => {
    const result = { run_id: "run:invented", summary: {}, results: [], diagnostics: [note(["model"]), note(["case:a"], "diagnostic:review:case")] } as unknown as MechanicsResult;
    expect(presentedResultDiagnostics(result)).toBe(result.diagnostics);
    const { listed, explanation } = shown(result, result.diagnostics[0]);
    expect(listed).toEqual([CLAIM, CLAIM]);
    expect(explanation.startsWith(`${CLAIM} It is linked to model.`)).toBe(true);
  });
});

// DEL-04-07: the cancellation-screen exception's warning. The pressure readers bind its presence, its whole
// message and its references to the exception record, so on a result the reader admitted it is a product
// diagnostic, copied as it stands even where the result has combinations and the warning names none of them;
// a result the reader refuses attests nothing, and the warning is then quoted as supplied text like any other.
describe("Diagnostics panel: the cancellation-screen exception's warning", () => {
  const MESSAGE = "At interior nodes of this case's pressure regions the cap loads of adjacent members do not cancel exactly in this model. The remainder comes from small differences in bore or alignment within each region's representation limits, from the rounding of each member's direction and bore area, or both. The solver retains it as a pressure load. The exception to the pressure cancellation screen that admitted this case does not establish that its effect is negligible; the case's numerical standing concerns the recorded geometry as computed, not an idealised region in which those loads cancel.";
  const warning = (refs: string[]): Diagnostic => ({ id: "diagnostic:pressure-runtime:invented:PRESSURE_REGION_CAP_REMAINDER_RETAINED", code: CAP_REMAINDER_WARNING_CODE,
    severity: "warning", source: "core/product_physics/src/pressure_runtime.rs", affected_refs: refs, message: MESSAGE }) as Diagnostic;

  it("is copied on a reader-admitted result with combinations and quoted where the reader admitted nothing", () => {
    const combinations = new Set(["comb:invented"]);
    const refs = ["case:invented", "region:invented", "node:invented"];
    expect(suppliedDiagnosticMessage(warning(refs), combinations, true)).toBe(MESSAGE);
    expect(suppliedDiagnosticMessage(warning(refs), combinations, false)).toBe(supplied(MESSAGE));
    expect(suppliedDiagnosticMessage({ ...warning(refs), code: "INVENTED_CODE" }, combinations, true)).toBe(supplied(MESSAGE));
    expect(suppliedDiagnosticMessage(warning(refs), new Set(), false)).toBe(MESSAGE);
  });

  it("is refused by the reader on a case without the exception's record, and then quoted as supplied", () => {
    const e = exCantilever();
    const result = e.document;
    const caseId = (result.contract_evidence as { exact_cases: { load_case_id: string }[] }).exact_cases[0].load_case_id;
    const forged = warning([caseId, "region:invented", "node:invented"]);
    result.diagnostics.push(forged);
    let refusal: unknown = null;
    try { readerValidate("pressure", result); } catch (error) { refusal = error; }
    expect((refusal as ResultReaderError | null)?.code).toBe("SOURCE_PHYSICS_RHS_SCREEN_EXCEPTION_WARNING_UNRECORDED");
    const { listed } = shown(result, forged);
    expect(listed).toEqual([supplied(MESSAGE)]);
  });
});
