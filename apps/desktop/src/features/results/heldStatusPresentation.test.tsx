// RT-A5 (DEL-05-02 `operating-case.md` §7.5; Work `combination-certificate.md` §2): a held gate
// notice's refusal text R is bound to its held-status record by the reader, not verified as true,
// and supplies no row bound, eligibility, certificate notice or standing. Every presenting path
// therefore quotes a diagnostic whose typed code is a held gate code whole as supplied text, even
// where the roster admitted it (`rosterChecked`): the Diagnostics panel's list and explanation, the
// result interpretation (`presentedResultDiagnostics`), the report projection
// (`diagnosticsForSections`, mirrored by Rust `report_package::bindings::expected_report_diagnostics`)
// and the run comparison. Keyed on the typed code (`HELD_STATUS_GATE_CODES`), never on wording; the
// result's raw bytes are never changed.
import { render, screen } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import heldCorpus from "../../../../../fixtures/results/preview_physics_held_status_corpus.json";
import { DiagnosticsPanel } from "../diagnostics/DiagnosticsPanel";
import { diagnosticsForSections } from "../report/renderableReportInput";
import { buildDiagnosticInterpretation } from "./resultInterpretation";
import { readerValidate } from "./resultReader";
import {
  HELD_STATUS_GATE_CODES,
  presentedResultDiagnostics,
  SUPPLEMENTAL_DIAGNOSTIC_LABEL,
  suppliedDiagnosticMessage
} from "./suppliedDiagnostics";
import type { Diagnostic, MechanicsResult, PreviewModel } from "../../types";

type Entry = { name: string; document: MechanicsResult; model: unknown };
const GATE = "diagnostic:preview-physics:combination-gate:13:comb:o-s-held";
const supplied = (message: string) => `${SUPPLEMENTAL_DIAGNOSTIC_LABEL}"${message}"`;
const PROVENANCE = {
  source_name: "invented session", source_location: "local desktop session", source_license: "user_supplied_or_private",
  contributor: "user_local_session", contributor_certification: "not_asserted", redistribution_status: "private_only",
  review_status: "pending", privacy_classification: "private_project_data"
};
// Claim-like refusals: nothing here or in the reader polices their language.
const CLAIMS = [
  "its rows are established within their stated bound",
  "every row is bounded by 1e-300 within its stated bound",
  "ESTABLISHED Within Its Stated BOUND",
  "les lignes sont établies — within their stated bound ≤ 2⁻¹⁰⁰⁰ ✓"
];

function held(name = "source_unavailable"): Entry {
  return structuredClone((heldCorpus as unknown as { documents: Entry[] }).documents.find((e) => e.name === name)!);
}
function notice(result: MechanicsResult): Diagnostic {
  return result.diagnostics.find((d) => d.id === GATE)!;
}
function record(result: MechanicsResult): { standing: { kind: string; reason: string } } {
  return (result.contract_evidence as unknown as Record<string, { standing: { kind: string; reason: string } }[]>).held_status_derivation[0];
}
function admitted(result: MechanicsResult): boolean {
  try { readerValidate("preview-physics", result); return true; } catch { return false; }
}
function panelModel(result: MechanicsResult): PreviewModel {
  return { diagnostics: [], project: { id: (result as { model_ref?: string }).model_ref }, rule_pack_refs: [], library_refs: [] } as unknown as PreviewModel;
}
/** The held notice as every presenting path shows it, and the raw bytes before and after. */
function presented(result: MechanicsResult) {
  const before = JSON.stringify(result);
  const raw = notice(result);
  const view = render(<DiagnosticsPanel model={panelModel(result)} knowledge={null} result={result} selectedDiagnosticId={raw.id!} onSelectDiagnostic={() => {}} />);
  const listed = screen.getAllByTestId(`diagnostic-${raw.code}`).map((item) => item.querySelector("p")!.textContent);
  const explanation = screen.getByTestId("selected-diagnostic-explanation").querySelector("dd")!.textContent!;
  view.unmount();
  const webview = presentedResultDiagnostics(result).find((d) => d.id === GATE)!.message;
  const interpretation = buildDiagnosticInterpretation({ model: panelModel(result), knowledge: null, result, diagnosticId: GATE })!.message;
  const report = diagnosticsForSections(result, PROVENANCE)[result.diagnostics.indexOf(raw)].message;
  expect(JSON.stringify(result)).toBe(before);
  return { raw: raw.message, listed, explanation, webview, interpretation, report };
}
function expectQuoted(result: MechanicsResult) {
  const shown = presented(result);
  for (const message of [shown.webview, shown.interpretation, shown.report]) expect(message).toBe(supplied(shown.raw));
  expect(shown.listed).toEqual([supplied(shown.raw)]);
  expect(shown.explanation.startsWith(supplied(shown.raw))).toBe(true);
}

describe("held gate notices: always quoted as supplied text", () => {
  it("keys the rule on the five typed codes only", () => {
    expect([...HELD_STATUS_GATE_CODES].sort()).toEqual([
      "HELD_STATUS_BINDING", "HELD_STATUS_SLIDING_GROUND_DIFFERS", "HELD_STATUS_SOLVE_FAILED", "HELD_STATUS_SOURCE_UNAVAILABLE", "HELD_STATUS_SOURCE_UNQUALIFIED"
    ]);
    // The same message under another code follows the ordinary classification.
    const other = { code: "REVIEW_NOTE", message: "the minuend case case:O has no converged nonlinear support state to lend", affected_refs: ["comb:o-s-held"] };
    expect(suppliedDiagnosticMessage(other, new Set(["comb:o-s-held"]), true)).toBe(other.message);
    expect(suppliedDiagnosticMessage({ ...other, code: "HELD_STATUS_BINDING" }, new Set(["comb:o-s-held"]), true)).toBe(supplied(other.message));
    expect(suppliedDiagnosticMessage({ ...other, code: "HELD_STATUS_BINDING" }, new Set(), false)).toBe(supplied(other.message));
  });

  it("quotes a roster-checked held notice on every path and keeps the raw bytes", () => {
    for (const name of ["source_unavailable", "sliding_ground_differs"]) {
      const result = held(name).document;
      expect(admitted(result)).toBe(true);
      // A roster entry naming the combination stays copied; the held notice alone is quoted.
      const headline = presentedResultDiagnostics(result).find((d) => d.code === "PREVIEW_HEADLINE_SCOPE_LOAD_CASES")!;
      expect(headline.message).toBe(result.diagnostics.find((d) => d.code === "PREVIEW_HEADLINE_SCOPE_LOAD_CASES")!.message);
      expectQuoted(result);
    }
  });

  it("quotes a claim-like refusal and changes no other presentation or standing", () => {
    const benign = held().document;
    const benignPresented = presentedResultDiagnostics(benign).filter((d) => d.id !== GATE);
    for (const claim of CLAIMS) {
      const result = held().document;
      record(result).standing.reason = claim;
      notice(result).message = `Combination withheld: ${claim}.`;
      expect(admitted(result)).toBe(true);
      expectQuoted(result);
      expect(presentedResultDiagnostics(result).filter((d) => d.id !== GATE)).toEqual(benignPresented);
      expect(result.numerical_quality).toEqual(benign.numerical_quality);
      expect(result.results).toEqual(benign.results);
      const certificate = (result.contract_evidence as unknown as Record<string, { combinations: { combination_id: string; assessment: string; groups: unknown[] }[] }>)
        .combination_certificate.combinations.find((r) => r.combination_id === "comb:o-s-held")!;
      expect([certificate.assessment, certificate.groups]).toEqual(["not_assessed", []]);
    }
  });

  it("quotes it where the binding is tampered, missing or unread, and on a stale source", () => {
    const tampered = held().document;
    notice(tampered).message = "Combination withheld: the minuend case case:O lent a state.";
    const missing = held().document;
    (missing.contract_evidence as unknown as Record<string, unknown[]>).held_status_derivation = [];
    const unbound = held().document;
    (unbound.contract_evidence as unknown as Record<string, unknown[]>).combination_gates = [];
    for (const [label, result] of [["tampered", tampered], ["missing", missing], ["unbound", unbound]] as const) {
      expect(admitted(result), label).toBe(false);
      expectQuoted(result);
    }
    // Stale: a source stating a superseded semantic contract identity. Whatever the reader makes of
    // it, the held notice is quoted, because the rule reads the typed code, not the reading.
    const stale = held().document;
    (stale.producer as unknown as Record<string, string>).semantic_contract_id = "openpipestress.result_semantics/0.3.0/preview-physics-1";
    expectQuoted(stale);
    // Unread: a result no ordinary route reader reads here (a retained successor's shape) is never roster-checked.
    const unread = held().document;
    (unread as unknown as Record<string, unknown>).retained_precision = {};
    expectQuoted(unread);
  });
});
