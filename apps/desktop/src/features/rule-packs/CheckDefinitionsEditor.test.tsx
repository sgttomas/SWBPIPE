import { useState } from "react";
import { cleanup, fireEvent, render, screen } from "@testing-library/react";
import { afterEach, describe, expect, it } from "vitest";
import {
  ACCEPTABILITY_BASES,
  ACCEPTABILITY_RELATIONS,
  ANALYSIS_STATUSES,
  CheckDefinitionsEditor,
  DIAGNOSTIC_CODES,
  DIAGNOSTIC_POLICY_FIELDS,
  defaultCheckDefinition,
  defaultDiagnosticPolicy,
  readCheckIds,
  readFormulaDeclarationIds,
  setCheckDefinitions
} from "./CheckDefinitionsEditor";
import type { DeclarationDocument } from "./DeclarationsEditor";
import { buildDraftRulePackDocument, draftPlaceholderProvenance } from "../../services/rulePackService";

// TP-C2-CHECKDEF-001 — check-definitions form builder. Pure-helper tests pin
// the schema-valid default shapes, the unique-id rule, and the lossless array
// rewrite; component tests drive the editor through visible controls and assert
// the rewritten document, including reference-picker binding, the result-status
// multi-select floor, diagnostic-policy edits, and the no-silent-snap escape
// hatches for out-of-vocabulary enums and unresolved references.

afterEach(cleanup);

function checksOf(document: DeclarationDocument): Array<Record<string, unknown>> {
  return document.check_definitions as Array<Record<string, unknown>>;
}

/** The draft template's single check with the given members overridden. */
function docWithCheck(patch: Record<string, unknown>): DeclarationDocument {
  const base = buildDraftRulePackDocument();
  const checks = base.check_definitions as Array<Record<string, unknown>>;
  return { ...base, check_definitions: [{ ...checks[0], ...patch }] };
}

describe("CheckDefinitionsEditor pure helpers", () => {
  it("reads declared check and formula ids in order", () => {
    const draft = buildDraftRulePackDocument();
    expect(readCheckIds(draft)).toEqual(["user_check_1"]);
    expect(readFormulaDeclarationIds(draft)).toEqual(["user_formula_1"]);
    // Malformed / absent arrays read as empty, never throw.
    expect(readCheckIds({})).toEqual([]);
    expect(readFormulaDeclarationIds({ formula_declarations: "nope" })).toEqual([]);
  });

  it("builds the schema-shaped default diagnostic policy (all eight bindings)", () => {
    expect(defaultDiagnosticPolicy()).toEqual({
      rule_check_blocking: "RULE_CHECK_BLOCKING",
      missing_input: "RULE_INPUT_MISSING",
      unit_mismatch: "RULE_UNIT_MISMATCH",
      provenance_gap: "RULE_PROVENANCE_WARNING",
      redistribution_gap: "RULE_REDISTRIBUTION_WARNING",
      checksum_mismatch: "RULE_CHECKSUM_MISMATCH",
      protected_content_suspected: "RULE_PROTECTED_CONTENT_WARNING",
      evaluator_error: "RULE_EVALUATOR_ERROR"
    });
  });

  it("builds a schema-shaped default check bound to the first declared ids", () => {
    // toEqual pins the EXACT shape: every CheckDefinition required member, a
    // fresh unique id, single-ref arrays bound to the first declared
    // input/slot/formula, the user-rule result statuses, the default policy,
    // and private-by-default placeholder provenance.
    expect(defaultCheckDefinition(["user_check_1"], ["in1"], ["slot1"], ["f1"])).toEqual({
      check_id: "user_check_2",
      name: "User-defined check (placeholder)",
      description: "Replace with the user-defined acceptability check.",
      required_input_refs: [{ ref_id: "in1", ref_type: "required_input" }],
      value_slot_refs: [{ ref_id: "slot1", ref_type: "value_slot" }],
      formula_ref: { ref_id: "f1", ref_type: "formula" },
      acceptability_basis: "user_supplied_rule_pack",
      acceptability_relation: "less_than_or_equal",
      result_statuses: [
        "RULE_INPUTS_INCOMPLETE",
        "USER_RULE_CHECKED",
        "USER_RULE_FAILED",
        "HUMAN_REVIEW_REQUIRED"
      ],
      diagnostic_policy: defaultDiagnosticPolicy(),
      provenance: draftPlaceholderProvenance()
    });
  });

  it("falls back to the uppercase TBD ref placeholder when nothing is declared", () => {
    const check = defaultCheckDefinition([], [], [], []);
    expect(check.check_id).toBe("user_check_1");
    expect(check.required_input_refs).toEqual([{ ref_id: "TBD", ref_type: "required_input" }]);
    expect(check.value_slot_refs).toEqual([{ ref_id: "TBD", ref_type: "value_slot" }]);
    expect(check.formula_ref).toEqual({ ref_id: "TBD", ref_type: "formula" });
  });

  it("replaces the check array losslessly, preserving the rest of the document", () => {
    const draft = buildDraftRulePackDocument();
    const next = setCheckDefinitions(draft, [{ check_id: "x" }, { check_id: "y" }]);
    expect(readCheckIds(next)).toEqual(["x", "y"]);
    // Other members are untouched by reference-equal copy.
    expect(next.required_inputs).toBe(draft.required_inputs);
    expect(next.formula_declarations).toBe(draft.formula_declarations);
    expect(next.value_slots).toBe(draft.value_slots);
  });

  it("exposes only the schema enum vocabularies", () => {
    expect(ACCEPTABILITY_BASES).toEqual([
      "user_supplied_rule_pack",
      "owner_design_basis",
      "private_project_basis",
      "invented_non_engineering_example",
      "TBD"
    ]);
    expect(ACCEPTABILITY_RELATIONS).toEqual([
      "less_than",
      "less_than_or_equal",
      "greater_than",
      "greater_than_or_equal"
    ]);
    expect(ANALYSIS_STATUSES).toEqual([
      "MODEL_INCOMPLETE",
      "MECHANICS_SOLVED",
      "RULE_INPUTS_INCOMPLETE",
      "USER_RULE_CHECKED",
      "USER_RULE_FAILED",
      "HUMAN_REVIEW_REQUIRED"
    ]);
    expect(DIAGNOSTIC_CODES).toEqual([
      "RULE_CHECK_BLOCKING",
      "RULE_INPUT_MISSING",
      "RULE_UNIT_MISMATCH",
      "RULE_PROVENANCE_WARNING",
      "RULE_REDISTRIBUTION_WARNING",
      "RULE_CHECKSUM_MISMATCH",
      "RULE_PROTECTED_CONTENT_WARNING",
      "RULE_EVALUATOR_ERROR",
      "RULE_INCOMPLETE_DATA",
      "RULE_PROFESSIONAL_BOUNDARY_NOTICE"
    ]);
    expect(DIAGNOSTIC_POLICY_FIELDS).toEqual([
      "rule_check_blocking",
      "missing_input",
      "unit_mismatch",
      "provenance_gap",
      "redistribution_gap",
      "checksum_mismatch",
      "protected_content_suspected",
      "evaluator_error"
    ]);
  });
});

function Harness({ initial }: { initial: DeclarationDocument }) {
  const [document, setDocument] = useState<DeclarationDocument>(initial);
  return (
    <>
      <CheckDefinitionsEditor document={document} onChange={setDocument} disabled={false} />
      <pre data-testid="check-doc">{JSON.stringify(document)}</pre>
    </>
  );
}

function harnessDoc(): DeclarationDocument {
  return JSON.parse(screen.getByTestId("check-doc").textContent ?? "{}") as DeclarationDocument;
}

describe("CheckDefinitionsEditor component", () => {
  it("lists the declared checks from the draft template", () => {
    render(<Harness initial={buildDraftRulePackDocument()} />);
    expect(screen.getByTestId("rule-pack-check-definitions").textContent).toContain(
      "Check definitions (1)"
    );
    expect((screen.getByTestId("rule-pack-check-id") as HTMLInputElement).value).toBe(
      "user_check_1"
    );
  });

  it("adds a check with a fresh unique id bound to the first declared ids", () => {
    render(<Harness initial={buildDraftRulePackDocument()} />);
    fireEvent.click(screen.getByTestId("rule-pack-check-add"));
    const checks = checksOf(harnessDoc());
    expect(checks).toHaveLength(2);
    expect(checks[1].check_id).toBe("user_check_2");
    expect(checks[1].required_input_refs).toEqual([
      { ref_id: "user_required_input_1", ref_type: "required_input" }
    ]);
    expect(checks[1].value_slot_refs).toEqual([
      { ref_id: "user_limit_slot_1", ref_type: "value_slot" }
    ]);
    expect(checks[1].formula_ref).toEqual({ ref_id: "user_formula_1", ref_type: "formula" });
  });

  it("edits check identity and description and preserves the rest of the check", () => {
    render(<Harness initial={buildDraftRulePackDocument()} />);
    const policyBefore = checksOf(harnessDoc())[0].diagnostic_policy;
    const provenanceBefore = checksOf(harnessDoc())[0].provenance;
    fireEvent.change(screen.getByTestId("rule-pack-check-id"), { target: { value: "hoop_check" } });
    fireEvent.change(screen.getByTestId("rule-pack-check-name"), {
      target: { value: "Hoop stress acceptability" }
    });
    fireEvent.change(screen.getByTestId("rule-pack-check-description"), {
      target: { value: "Invented example check." }
    });
    const check = checksOf(harnessDoc())[0];
    expect(check).toMatchObject({
      check_id: "hoop_check",
      name: "Hoop stress acceptability",
      description: "Invented example check."
    });
    // Lossless: the members the edit did not touch round-trip verbatim.
    expect(check.diagnostic_policy).toEqual(policyBefore);
    expect(check.provenance).toEqual(provenanceBefore);
  });

  it("edits the acceptability basis through the closed-vocabulary select", () => {
    render(<Harness initial={buildDraftRulePackDocument()} />);
    fireEvent.change(screen.getByTestId("rule-pack-check-acceptability-basis"), {
      target: { value: "invented_non_engineering_example" }
    });
    expect(checksOf(harnessDoc())[0].acceptability_basis).toBe("invented_non_engineering_example");
  });

  it("rebinds the formula ref through a picker over declared formula ids", () => {
    const base = buildDraftRulePackDocument();
    const doc = {
      ...base,
      formula_declarations: [
        ...(base.formula_declarations as unknown[]),
        { formula_id: "alt_formula" }
      ]
    };
    render(<Harness initial={doc} />);
    fireEvent.change(screen.getByTestId("rule-pack-check-formula-ref"), {
      target: { value: "alt_formula" }
    });
    expect(checksOf(harnessDoc())[0].formula_ref).toEqual({
      ref_id: "alt_formula",
      ref_type: "formula"
    });
  });

  it("rebinds, adds, and removes input refs with the minItems floor", () => {
    const base = buildDraftRulePackDocument();
    const doc = {
      ...base,
      required_inputs: [
        ...(base.required_inputs as unknown[]),
        { input_id: "second_input" }
      ]
    };
    render(<Harness initial={doc} />);
    // The single template ref resolves; rebind it to the second declared input.
    expect(screen.getByTestId("rule-pack-check-input-ref-remove")).toHaveProperty("disabled", true);
    fireEvent.change(screen.getByTestId("rule-pack-check-input-ref"), {
      target: { value: "second_input" }
    });
    expect(checksOf(harnessDoc())[0].required_input_refs).toEqual([
      { ref_id: "second_input", ref_type: "required_input" }
    ]);

    // Add a second ref (defaults to the first declared id), then remove it.
    fireEvent.click(screen.getByTestId("rule-pack-check-input-ref-add"));
    let refs = checksOf(harnessDoc())[0].required_input_refs as unknown[];
    expect(refs).toHaveLength(2);
    expect(refs[1]).toEqual({ ref_id: "user_required_input_1", ref_type: "required_input" });
    fireEvent.click(screen.getAllByTestId("rule-pack-check-input-ref-remove")[1]);
    refs = checksOf(harnessDoc())[0].required_input_refs as unknown[];
    expect(refs).toEqual([{ ref_id: "second_input", ref_type: "required_input" }]);
  });

  it("edits one diagnostic-policy binding and preserves the other seven", () => {
    render(<Harness initial={buildDraftRulePackDocument()} />);
    fireEvent.change(screen.getByTestId("rule-pack-check-policy-unit_mismatch"), {
      target: { value: "RULE_INCOMPLETE_DATA" }
    });
    const policy = checksOf(harnessDoc())[0].diagnostic_policy as Record<string, unknown>;
    expect(policy.unit_mismatch).toBe("RULE_INCOMPLETE_DATA");
    // The seven untouched bindings are unchanged.
    expect(policy.missing_input).toBe("RULE_INPUT_MISSING");
    expect(policy.evaluator_error).toBe("RULE_EVALUATOR_ERROR");
  });

  it("toggles result statuses and enforces the minItems:1 floor", () => {
    render(<Harness initial={buildDraftRulePackDocument()} />);
    // Toggle a new status on (appended).
    fireEvent.click(screen.getByTestId("rule-pack-check-status-MODEL_INCOMPLETE"));
    expect(checksOf(harnessDoc())[0].result_statuses).toContain("MODEL_INCOMPLETE");
    // Toggle an existing status off.
    fireEvent.click(screen.getByTestId("rule-pack-check-status-USER_RULE_CHECKED"));
    expect(checksOf(harnessDoc())[0].result_statuses).not.toContain("USER_RULE_CHECKED");
  });

  it("disables unchecking the last remaining result status", () => {
    render(<Harness initial={docWithCheck({ result_statuses: ["USER_RULE_CHECKED"] })} />);
    expect(screen.getByTestId("rule-pack-check-status-USER_RULE_CHECKED")).toHaveProperty(
      "disabled",
      true
    );
    // A different (unchecked) status stays enabled — the floor blocks only the last checked one.
    expect(screen.getByTestId("rule-pack-check-status-USER_RULE_FAILED")).toHaveProperty(
      "disabled",
      false
    );
  });

  it("blocks removing the last check and supports add-then-remove losslessly", () => {
    render(<Harness initial={buildDraftRulePackDocument()} />);
    expect(screen.getByTestId("rule-pack-check-remove")).toHaveProperty("disabled", true);

    fireEvent.click(screen.getByTestId("rule-pack-check-add"));
    fireEvent.change(screen.getAllByTestId("rule-pack-check-id")[0], {
      target: { value: "kept_check" }
    });
    const firstBefore = checksOf(harnessDoc())[0];
    fireEvent.click(screen.getAllByTestId("rule-pack-check-remove")[1]);
    const checks = checksOf(harnessDoc());
    expect(checks).toHaveLength(1);
    expect(checks[0]).toEqual(firstBefore);
    expect(checks[0].check_id).toBe("kept_check");
  });

  it("editing one check preserves sibling checks verbatim", () => {
    render(<Harness initial={buildDraftRulePackDocument()} />);
    fireEvent.click(screen.getByTestId("rule-pack-check-add"));
    const secondBefore = checksOf(harnessDoc())[1];
    fireEvent.change(screen.getAllByTestId("rule-pack-check-name")[0], {
      target: { value: "changed" }
    });
    expect(checksOf(harnessDoc())[1]).toEqual(secondBefore);
  });

  it("surfaces an out-of-vocabulary acceptability basis as a (current) option", () => {
    render(<Harness initial={docWithCheck({ acceptability_basis: "future_basis_from_a_newer_schema" })} />);
    const select = screen.getByTestId("rule-pack-check-acceptability-basis") as HTMLSelectElement;
    expect(select.value).toBe("future_basis_from_a_newer_schema");
    expect(screen.getByText("(current) future_basis_from_a_newer_schema")).toBeTruthy();
  });

  it("authors the acceptability relation through the selector", () => {
    render(<Harness initial={buildDraftRulePackDocument()} />);
    const select = screen.getByTestId("rule-pack-check-acceptability-relation") as HTMLSelectElement;
    // The draft template authors the explicit default; the editor never relies
    // on the runner's absent->less_than_or_equal fallback.
    expect(select.value).toBe("less_than_or_equal");
    fireEvent.change(select, { target: { value: "greater_than_or_equal" } });
    expect(checksOf(harnessDoc())[0].acceptability_relation).toBe("greater_than_or_equal");
  });

  it("shows the less_than_or_equal default for a pack with no relation, without mutating it", () => {
    // Opening a pre-member pack: the runner defaults absent -> less_than_or_equal,
    // and the editor mirrors that as the displayed value WITHOUT writing the
    // member back (lossless round-trip until the user edits it).
    const { acceptability_relation: _omitted, ...checkWithout } = (
      buildDraftRulePackDocument().check_definitions as Array<Record<string, unknown>>
    )[0];
    void _omitted;
    render(
      <Harness initial={{ ...buildDraftRulePackDocument(), check_definitions: [checkWithout] }} />
    );
    const select = screen.getByTestId("rule-pack-check-acceptability-relation") as HTMLSelectElement;
    expect(select.value).toBe("less_than_or_equal");
    // No "(current)" escape option: less_than_or_equal is in vocabulary.
    expect(screen.queryByText(/\(current\)/)).toBeNull();
    expect("acceptability_relation" in checksOf(harnessDoc())[0]).toBe(false);
  });

  it("surfaces an out-of-vocabulary result status as a removable (current) option", () => {
    render(
      <Harness
        initial={docWithCheck({ result_statuses: ["USER_RULE_CHECKED", "FUTURE_STATUS"] })}
      />
    );
    expect(screen.getByText("(current) FUTURE_STATUS")).toBeTruthy();
    const checkbox = screen.getByTestId("rule-pack-check-status-FUTURE_STATUS") as HTMLInputElement;
    expect(checkbox.checked).toBe(true);
    expect(checkbox.disabled).toBe(false);
    fireEvent.click(checkbox);
    expect(checksOf(harnessDoc())[0].result_statuses).toEqual(["USER_RULE_CHECKED"]);
  });

  it("enforces the result-status floor even when the last status is out-of-vocabulary", () => {
    // The minItems:1 floor keys on the selected count, not the vocabulary, so a
    // single out-of-vocabulary "(current)" status cannot be unchecked either.
    render(<Harness initial={docWithCheck({ result_statuses: ["FUTURE_ONLY_STATUS"] })} />);
    const checkbox = screen.getByTestId(
      "rule-pack-check-status-FUTURE_ONLY_STATUS"
    ) as HTMLInputElement;
    expect(checkbox.checked).toBe(true);
    expect(checkbox.disabled).toBe(true);
  });

  it("surfaces an unresolved reference without snapping it to a declared id", () => {
    // The check refs a required input that is not declared (renamed/removed).
    const doc = docWithCheck({
      required_input_refs: [{ ref_id: "dangling_input", ref_type: "required_input" }]
    });
    render(<Harness initial={doc} />);
    const select = screen.getByTestId("rule-pack-check-input-ref") as HTMLSelectElement;
    expect(select.value).toBe("dangling_input");
    expect(screen.getByText("(unresolved) dangling_input")).toBeTruthy();
  });

  it("disables every control while a backend request is in flight", () => {
    render(
      <CheckDefinitionsEditor
        document={buildDraftRulePackDocument()}
        onChange={() => {}}
        disabled
        disabledReason="busy"
      />
    );
    expect(screen.getByTestId("rule-pack-check-id")).toHaveProperty("disabled", true);
    expect(screen.getByTestId("rule-pack-check-add")).toHaveProperty("disabled", true);
    expect(screen.getByTestId("rule-pack-check-acceptability-basis")).toHaveProperty(
      "disabled",
      true
    );
    expect(screen.getByTestId("rule-pack-check-status-USER_RULE_CHECKED")).toHaveProperty(
      "disabled",
      true
    );
  });
});
