import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import type { PreviewModel } from "../../types";
import type { RulePackDocumentValidation } from "../../services/rulePackService";
import { RulePackManagerPanel } from "./RulePackManagerPanel";

// W-17 (DEL-06-02 Design/rule-pack-unit-check.md §3): the "Unit consistency"
// group and the summary's unit_check status. jsdom has no Tauri runtime, so
// the validate route is stubbed with a backend-shaped validation; every id
// and unit below is invented.

const validation = vi.hoisted((): RulePackDocumentValidation => ({
  document_kind: "openpipestress.rule_pack.document_validation",
  rule_pack_id: "invented_pack",
  rule_pack_name: "Invented",
  rule_pack_version: "0.1.0",
  schema_version: "0.4.0",
  grammar_version: "1.0.0",
  grammar_version_supported: true,
  supported_grammar_versions: ["1.0.0"],
  evaluator_grammar_version: "1.0.0",
  document_findings: [],
  lifecycle_findings: [],
  lifecycle_blocked: false,
  formulas: [],
  unit_check: {
    status: "findings",
    checks: [
      {
        check_id: "invented_consistent_check",
        formula_id: "invented_ratio",
        status: "consistent",
        findings: []
      },
      {
        check_id: "invented_mismatch_check",
        formula_id: "invented_sum",
        status: "findings",
        findings: [
          {
            code: "UnitMismatch",
            severity: "blocking",
            subject: "add_subtract",
            path: "formula_declarations[1].declaration_payload.expression_ast",
            operands: [
              { kind: "quantity", dimension: "stress", unit_ref: "MPa" },
              { kind: "quantity", dimension: "stress", unit_ref: "Pa" }
            ],
            message: "operand units differ and the evaluator does not convert units"
          },
          {
            code: "RULE_PACK_LIMIT_CANDIDATE_MISMATCH",
            severity: "conditional",
            subject: "invented_slot",
            path: "check_definitions[1].value_slot_refs[1]",
            operands: [],
            message: "the check fails if this slot supplies the limit at run time"
          }
        ]
      },
      {
        check_id: "invented_marker_check",
        formula_id: "invented_marker",
        status: "not_checkable",
        findings: []
      }
    ],
    declaration_findings: [
      {
        code: "RULE_PACK_UNIT_NOT_IN_CATALOG",
        severity: "warning",
        subject: "invented_input",
        path: "required_inputs[0].quantity_intent.unit_ref",
        operands: [{ kind: "quantity", dimension: "stress", unit_ref: "invented_unit" }],
        message: "a value in any other unit will be refused at run time"
      }
    ]
  },
  checksum: { declared_value: null, computed: null, match_status: "not_computable" },
  publicly_exportable: false,
  professional_boundary_notice: "Human review remains required.",
  has_blocking_findings: false
}));

vi.mock("../../services/rulePackService", async (importOriginal) => {
  const actual = await importOriginal<typeof import("../../services/rulePackService")>();
  return {
    ...actual,
    validateRulePack: vi.fn(async () => ({ route: "tauri_backend", validation }))
  };
});

const modelStub = {
  project: { id: "project:invented-unit-check", name: "Invented Unit Check Project" }
} as unknown as PreviewModel;

afterEach(cleanup);

describe("RulePackManagerPanel unit consistency", () => {
  it("renders the unit-check group, each check's status and its findings", async () => {
    render(<RulePackManagerPanel model={modelStub} />);
    fireEvent.click(screen.getByTestId("rule-pack-new-draft"));
    fireEvent.click(screen.getByTestId("rule-pack-validate"));
    await waitFor(() => expect(screen.getByTestId("rule-pack-unit-check")).toBeTruthy());

    expect(screen.getByTestId("rule-pack-validation-summary").textContent).toContain(
      "unit_check=findings"
    );
    // blocking_findings keeps its meaning: unit-check findings do not enter it.
    expect(screen.getByTestId("rule-pack-validation-summary").textContent).toContain(
      "blocking_findings=false"
    );
    const group = screen.getByTestId("rule-pack-unit-check");
    expect(group.textContent).toContain("Unit consistency");
    expect(screen.getByTestId("rule-pack-unit-check-note").textContent).toBe(
      "Units of solver results and entered values are checked when the check runs."
    );
    expect(
      screen.getByTestId("rule-pack-unit-check-status-invented_consistent_check").textContent
    ).toBe("status=consistent");
    expect(
      screen.getByTestId("rule-pack-unit-check-status-invented_mismatch_check").textContent
    ).toBe("status=findings");
    expect(
      screen.getByTestId("rule-pack-unit-check-status-invented_marker_check").textContent
    ).toBe("status=not_checkable");

    const mismatch = screen.getByTestId("rule-pack-unit-check-invented_mismatch_check");
    expect(mismatch.textContent).toContain("UnitMismatch (blocking) subject=add_subtract");
    expect(mismatch.textContent).toContain(
      "path=formula_declarations[1].declaration_payload.expression_ast"
    );
    expect(mismatch.textContent).toContain("operands=stress [MPa] vs stress [Pa]");
    expect(mismatch.textContent).toContain("RULE_PACK_LIMIT_CANDIDATE_MISMATCH (conditional)");

    const declarations = screen.getByTestId("rule-pack-unit-check-declarations");
    expect(declarations.textContent).toContain("RULE_PACK_UNIT_NOT_IN_CATALOG (warning)");
    expect(declarations.textContent).toContain("path=required_inputs[0].quantity_intent.unit_ref");
    expect(screen.getAllByTestId("rule-pack-unit-check-finding")).toHaveLength(3);
  });
});
