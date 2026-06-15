import { useState } from "react";
import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";

const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", () => ({ invoke: invokeMock }));

import {
  DIMENSIONS,
  ExpressionComposer,
  collectRulePackVariables,
  defaultExpressionNode,
  isEditableNode,
  isTableNode,
  nodeKind,
  parseRulePackDocument,
  readFormulaDeclarations,
  setFormulaExpression,
  type AstNode,
  type RulePackDocument
} from "./ExpressionComposer";
import { buildDraftRulePackDocument } from "../../services/rulePackService";
import type { UnitCatalog } from "../../services/unitCatalogService";

// TP-C2-COMPOSER-001 — structured AST expression composer.
// Pure-helper tests pin the grammar v1.0.0 node encoding and the lossless
// document rewrite; component tests drive the composer through visible
// controls and assert the rewritten document.

const catalogFixture: UnitCatalog = {
  schema_version: "0.1.0",
  catalog_id: "unit-system:dec-018-si-dual-display",
  decision_basis: "DEC-018",
  calculation_basis: "si_canonical",
  storage_convention: "entered_units_preserved",
  entry_count: 5,
  entries: [
    {
      unit_id: "unit:meter",
      symbol: "m",
      dimension_id: "length",
      canonical: true,
      transform_kind: "identity",
      factor_representation: "1 m/m, SI canonical identity",
      offset_representation: null,
      provenance: "si_canonical",
      review_status: "accepted"
    },
    {
      unit_id: "unit:pascal",
      symbol: "Pa",
      dimension_id: "pressure",
      canonical: true,
      transform_kind: "identity",
      factor_representation: "1 Pa/Pa, SI canonical identity",
      offset_representation: null,
      provenance: "si_canonical",
      review_status: "accepted"
    },
    {
      unit_id: "unit:megapascal",
      symbol: "MPa",
      dimension_id: "pressure",
      canonical: false,
      transform_kind: "multiplicative",
      factor_representation: "1000000 Pa/MPa, exact public definition",
      offset_representation: null,
      provenance: "exact_public_definition",
      review_status: "accepted"
    },
    {
      unit_id: "unit:degree_celsius",
      symbol: "degC",
      dimension_id: "temperature",
      canonical: false,
      transform_kind: "affine",
      factor_representation: "1 K/degC interval, exact public definition",
      offset_representation: "273.15 K at 0 degC",
      provenance: "exact_public_definition",
      review_status: "accepted"
    },
    {
      unit_id: "unit:pound_force",
      symbol: "lbf",
      dimension_id: "force",
      canonical: false,
      transform_kind: "multiplicative",
      factor_representation: "0.45359237 kg/lb * 9.80665 m/s^2 per lbf",
      offset_representation: null,
      provenance: "conventional_public_constant",
      review_status: "accepted"
    }
  ],
  boundary: {
    source: "core/units open_pipe_stress_units catalog",
    protected_content_included: false,
    private_project_data_included: false,
    professional_approval_claimed: false,
    code_compliance_claimed: false
  }
};

afterEach(() => {
  cleanup();
  invokeMock.mockReset();
  delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
});

function expressionOf(document: RulePackDocument): AstNode {
  const formula = (document.formula_declarations as Array<Record<string, unknown>>)[0];
  const payload = formula.declaration_payload as Record<string, unknown>;
  return payload.expression_ast as AstNode;
}

describe("ExpressionComposer pure helpers", () => {
  it("collects declared required-input and value-slot variables", () => {
    const variables = collectRulePackVariables(buildDraftRulePackDocument());
    expect(variables.map((variable) => variable.id)).toEqual([
      "user_required_input_1",
      "user_limit_slot_1"
    ]);
    expect(variables[0].kind).toBe("required_input");
    expect(variables[1].kind).toBe("value_slot");
  });

  it("builds default nodes for every editable type with the frozen encoding", () => {
    const variables = collectRulePackVariables(buildDraftRulePackDocument());
    // toEqual (not toMatchObject) pins the EXACT quantity shape: the schema's
    // ExpressionQuantity is additionalProperties:false and allows only value /
    // dimension / unit_ref (the unit_required / dimension_check_required
    // relaxation flags are not authorable in pack documents — absent means
    // true). The unknown-dimension placeholder is the uppercase "TBD" token the
    // codec (decode_quantity) and schema DimensionId accept — never "tbd".
    expect(defaultExpressionNode("literal", variables)).toEqual({
      node: "literal",
      quantity: { value: 0, dimension: "TBD", unit_ref: "TBD" }
    });
    expect(defaultExpressionNode("variable_ref", variables)).toEqual({
      node: "variable_ref",
      variable_id: "user_required_input_1"
    });
    expect(defaultExpressionNode("unary", variables)).toMatchObject({
      node: "unary",
      operator: "negate"
    });
    expect(defaultExpressionNode("binary", variables)).toMatchObject({
      node: "binary",
      operator: "add"
    });
    expect(defaultExpressionNode("compare", variables)).toMatchObject({
      node: "compare",
      operator: "less_than_or_equal"
    });
    expect(defaultExpressionNode("logical", variables)).toMatchObject({
      node: "logical",
      operator: "and"
    });
    expect(defaultExpressionNode("select", variables)).toMatchObject({ node: "select" });
    expect(defaultExpressionNode("aggregate", variables)).toMatchObject({
      node: "aggregate",
      function: "min",
      operands: [{ node: "variable_ref" }]
    });
    // Table nodes: a schema-valid default table (two strictly-increasing rows
    // so interpolate is immediately well-formed) with TBD placeholders, plus
    // the recursive argument child. lookup additionally carries an exact mode.
    expect(defaultExpressionNode("interpolate", variables)).toEqual({
      node: "interpolate",
      table: {
        table_id: "user_table_1",
        argument_dimension: "TBD",
        argument_unit_ref: "TBD",
        result_dimension: "TBD",
        result_unit_ref: "TBD",
        rows: [
          { argument: 0, result: 0 },
          { argument: 1, result: 0 }
        ]
      },
      argument: { node: "variable_ref", variable_id: "user_required_input_1" }
    });
    expect(defaultExpressionNode("lookup", variables)).toMatchObject({
      node: "lookup",
      mode: "exact",
      table: { table_id: "user_table_1", rows: [{ argument: 0 }, { argument: 1 }] }
    });
  });

  it("offers only the codec/schema dimension vocabulary with the uppercase TBD placeholder", () => {
    // Guards the slice-2 regression: a lowercase "tbd" default produced
    // documents that fail backend decode and DimensionId schema validation.
    expect(DIMENSIONS).toContain("TBD");
    expect(DIMENSIONS).not.toContain("tbd");
  });

  it("classifies node kinds (editable vs table vs unknown)", () => {
    expect(nodeKind({ node: "compare" })).toBe("compare");
    expect(nodeKind({})).toBe("unrecognized");
    expect(isEditableNode({ node: "logical" })).toBe(true);
    // Table nodes are now editable (slice 3); refusal markers and unknown
    // tags remain non-editable and are preserved read-only.
    expect(isEditableNode({ node: "interpolate" })).toBe(true);
    expect(isEditableNode({ node: "lookup" })).toBe(true);
    expect(isEditableNode({ node: "unsafe_host_access" })).toBe(false);
    expect(isEditableNode({ node: "future_feature" })).toBe(false);
    expect(isTableNode({ node: "lookup" })).toBe(true);
    expect(isTableNode({ node: "literal" })).toBe(false);
  });

  it("reads formula declarations and rewrites one expression while preserving siblings", () => {
    const document: RulePackDocument = {
      formula_declarations: [
        {
          formula_id: "f1",
          formula_kind: "derived_quantity",
          declaration_payload: { payload_kind: "declarative_ast", expression_ast: { node: "variable_ref", variable_id: "a" } }
        },
        {
          formula_id: "f2",
          formula_kind: "boolean_check",
          declaration_payload: { expression_ast: { node: "literal", quantity: { value: 1 } } }
        }
      ]
    };
    const formulas = readFormulaDeclarations(document);
    expect(formulas.map((formula) => formula.id)).toEqual(["f1", "f2"]);

    const next = setFormulaExpression(document, "f1", { node: "compare", operator: "equal" });
    const decls = next.formula_declarations as Array<Record<string, unknown>>;
    expect((decls[0].declaration_payload as Record<string, unknown>).expression_ast).toEqual({
      node: "compare",
      operator: "equal"
    });
    // The untouched sibling and the payload's other members are preserved.
    expect(decls[1]).toEqual((document.formula_declarations as unknown[])[1]);
    expect((decls[0].declaration_payload as Record<string, unknown>).payload_kind).toBe(
      "declarative_ast"
    );
  });

  it("parses document text honestly", () => {
    expect(parseRulePackDocument(null)).toEqual({ ok: false, error: "no-draft" });
    expect(parseRulePackDocument("{not json").ok).toBe(false);
    expect(parseRulePackDocument("[]").ok).toBe(false);
    const parsed = parseRulePackDocument('{"a":1}');
    expect(parsed).toEqual({ ok: true, document: { a: 1 } });
  });
});

function Harness({ initial }: { initial: RulePackDocument }) {
  const [document, setDocument] = useState<RulePackDocument>(initial);
  return (
    <>
      <ExpressionComposer document={document} onChange={setDocument} disabled={false} />
      <pre data-testid="harness-doc">{JSON.stringify(document)}</pre>
    </>
  );
}

function harnessExpression(): AstNode {
  const document = JSON.parse(screen.getByTestId("harness-doc").textContent ?? "{}") as RulePackDocument;
  return expressionOf(document);
}

describe("ExpressionComposer component", () => {
  it("lists declared variables in the browser", () => {
    render(<Harness initial={buildDraftRulePackDocument()} />);
    const browser = screen.getByTestId("rule-pack-variable-browser").textContent ?? "";
    expect(browser).toContain("user_required_input_1 (required_input)");
    expect(browser).toContain("user_limit_slot_1 (value_slot)");
  });

  it("switches the root node type and rewrites the expression AST", () => {
    render(<Harness initial={buildDraftRulePackDocument()} />);
    // The draft starts with a single variable_ref root node.
    expect(harnessExpression()).toEqual({ node: "variable_ref", variable_id: "user_required_input_1" });

    fireEvent.change(screen.getAllByTestId("rule-pack-node-type")[0], {
      target: { value: "compare" }
    });
    const compare = harnessExpression();
    expect(compare.node).toBe("compare");
    expect(compare.operator).toBe("less_than_or_equal");
    expect((compare.left as AstNode).node).toBe("variable_ref");
    expect((compare.right as AstNode).node).toBe("literal");
  });

  it("edits a literal value, dimension, and unit", () => {
    render(<Harness initial={buildDraftRulePackDocument()} />);
    fireEvent.change(screen.getAllByTestId("rule-pack-node-type")[0], { target: { value: "literal" } });
    fireEvent.change(screen.getByTestId("rule-pack-literal-value"), { target: { value: "42.5" } });
    fireEvent.change(screen.getByTestId("rule-pack-literal-dimension"), { target: { value: "stress" } });
    fireEvent.change(screen.getByTestId("rule-pack-literal-unit"), { target: { value: "invented_stress_unit" } });
    expect(harnessExpression()).toMatchObject({
      node: "literal",
      quantity: { value: 42.5, dimension: "stress", unit_ref: "invented_stress_unit" }
    });
  });

  it("uses the DEC-018 catalog for literal unit refs in desktop mode", async () => {
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
    invokeMock.mockResolvedValue(catalogFixture);
    render(<Harness initial={buildDraftRulePackDocument()} />);

    await waitFor(() => expect(invokeMock).toHaveBeenCalledWith("get_unit_catalog"));
    fireEvent.change(screen.getAllByTestId("rule-pack-node-type")[0], { target: { value: "literal" } });
    fireEvent.change(screen.getByTestId("rule-pack-literal-dimension"), {
      target: { value: "stress" }
    });

    const unitSelect = screen.getByTestId("rule-pack-literal-unit") as HTMLSelectElement;
    await waitFor(() => expect(Array.from(unitSelect.options).map((option) => option.value)).toContain("MPa"));
    const optionValues = Array.from(unitSelect.options).map((option) => option.value);
    expect(optionValues).toContain("Pa");
    expect(optionValues).not.toContain("m");
    expect(optionValues).not.toContain("lbf");

    fireEvent.change(unitSelect, { target: { value: "MPa" } });
    expect(harnessExpression()).toMatchObject({
      node: "literal",
      quantity: { dimension: "stress", unit_ref: "MPa" }
    });
  });

  it("keeps browser-preview literal unit refs as manual text fields", () => {
    render(<Harness initial={buildDraftRulePackDocument()} />);
    fireEvent.change(screen.getAllByTestId("rule-pack-node-type")[0], { target: { value: "literal" } });

    const unitField = screen.getByTestId("rule-pack-literal-unit") as HTMLInputElement;
    expect(unitField.tagName).toBe("INPUT");
    fireEvent.change(unitField, { target: { value: "MPa" } });
    expect(invokeMock).not.toHaveBeenCalled();
    expect(harnessExpression()).toMatchObject({
      node: "literal",
      quantity: { unit_ref: "MPa" }
    });
  });

  it("adds and removes aggregate operands and blocks dropping the last one", () => {
    render(<Harness initial={buildDraftRulePackDocument()} />);
    fireEvent.change(screen.getAllByTestId("rule-pack-node-type")[0], { target: { value: "aggregate" } });
    expect((harnessExpression().operands as unknown[]).length).toBe(1);
    // Last-operand removal is blocked with a stated reason.
    expect(screen.getByTestId("rule-pack-aggregate-remove-operand")).toHaveProperty("disabled", true);

    fireEvent.click(screen.getByTestId("rule-pack-aggregate-add-operand"));
    expect((harnessExpression().operands as unknown[]).length).toBe(2);
    fireEvent.click(screen.getAllByTestId("rule-pack-aggregate-remove-operand")[0]);
    expect((harnessExpression().operands as unknown[]).length).toBe(1);
  });

  it("switches the root node to an interpolate table and seeds a schema-valid default", () => {
    render(<Harness initial={buildDraftRulePackDocument()} />);
    fireEvent.change(screen.getAllByTestId("rule-pack-node-type")[0], {
      target: { value: "interpolate" }
    });
    const node = harnessExpression();
    expect(node.node).toBe("interpolate");
    const table = node.table as Record<string, unknown>;
    expect(table.table_id).toBe("user_table_1");
    expect(table.argument_dimension).toBe("TBD");
    expect(table.result_dimension).toBe("TBD");
    expect((table.rows as unknown[]).length).toBe(2);
    expect((node.argument as AstNode).node).toBe("variable_ref");
    // interpolate carries no mode field; only lookup does.
    expect(screen.queryByTestId("rule-pack-lookup-mode")).toBeNull();
  });

  it("edits a lookup table's id, dimensions, units, mode, and rows", () => {
    const document = setFormulaExpression(buildDraftRulePackDocument(), "user_formula_1", {
      node: "lookup",
      mode: "exact",
      table: {
        table_id: "user_table_1",
        argument_dimension: "TBD",
        argument_unit_ref: "TBD",
        result_dimension: "TBD",
        result_unit_ref: "TBD",
        rows: [
          { argument: 0, result: 0 },
          { argument: 1, result: 0 }
        ]
      },
      argument: { node: "variable_ref", variable_id: "user_required_input_1" }
    });
    render(<Harness initial={document} />);

    fireEvent.change(screen.getByTestId("rule-pack-table-id"), { target: { value: "temp_factor" } });
    fireEvent.change(screen.getByTestId("rule-pack-table-argument-dimension"), {
      target: { value: "temperature" }
    });
    fireEvent.change(screen.getByTestId("rule-pack-table-argument-unit"), {
      target: { value: "invented_degc" }
    });
    fireEvent.change(screen.getByTestId("rule-pack-table-result-dimension"), {
      target: { value: "stress" }
    });
    fireEvent.change(screen.getByTestId("rule-pack-table-result-unit"), {
      target: { value: "invented_stress" }
    });
    fireEvent.change(screen.getByTestId("rule-pack-lookup-mode"), { target: { value: "step" } });
    fireEvent.change(screen.getAllByTestId("rule-pack-table-row-result")[0], {
      target: { value: "1.5" }
    });

    const node = harnessExpression();
    expect(node.mode).toBe("step");
    expect(node.table).toMatchObject({
      table_id: "temp_factor",
      argument_dimension: "temperature",
      argument_unit_ref: "invented_degc",
      result_dimension: "stress",
      result_unit_ref: "invented_stress"
    });
    expect((node.table as Record<string, unknown>).rows).toEqual([
      { argument: 0, result: 1.5 },
      { argument: 1, result: 0 }
    ]);
  });

  it("uses the DEC-018 catalog for table unit refs and preserves out-of-catalog values", async () => {
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
    invokeMock.mockResolvedValue(catalogFixture);
    const document = setFormulaExpression(buildDraftRulePackDocument(), "user_formula_1", {
      node: "interpolate",
      table: {
        table_id: "legacy_table",
        argument_dimension: "temperature",
        argument_unit_ref: "legacy_temperature_unit",
        result_dimension: "stress",
        result_unit_ref: "legacy_stress_unit",
        rows: [
          { argument: 0, result: 0 },
          { argument: 1, result: 0 }
        ]
      },
      argument: { node: "variable_ref", variable_id: "user_required_input_1" }
    });
    render(<Harness initial={document} />);

    await waitFor(() => expect(invokeMock).toHaveBeenCalledWith("get_unit_catalog"));
    const argumentUnit = screen.getByTestId("rule-pack-table-argument-unit") as HTMLSelectElement;
    const resultUnit = screen.getByTestId("rule-pack-table-result-unit") as HTMLSelectElement;
    await waitFor(() => expect(argumentUnit.value).toBe("legacy_temperature_unit"));
    await waitFor(() => expect(resultUnit.value).toBe("legacy_stress_unit"));
    expect(screen.getByText("legacy_temperature_unit, catalog mismatch")).toBeTruthy();
    expect(screen.getByText("legacy_stress_unit, catalog mismatch")).toBeTruthy();

    fireEvent.change(argumentUnit, { target: { value: "degC" } });
    fireEvent.change(resultUnit, { target: { value: "Pa" } });
    const table = harnessExpression().table as Record<string, unknown>;
    expect(table).toMatchObject({
      argument_dimension: "temperature",
      argument_unit_ref: "degC",
      result_dimension: "stress",
      result_unit_ref: "Pa"
    });
  });

  it("adds and removes table rows and blocks dropping the last one", () => {
    render(<Harness initial={buildDraftRulePackDocument()} />);
    fireEvent.change(screen.getAllByTestId("rule-pack-node-type")[0], {
      target: { value: "interpolate" }
    });
    expect(screen.getAllByTestId("rule-pack-table-row")).toHaveLength(2);

    fireEvent.click(screen.getByTestId("rule-pack-table-add-row"));
    let rows = (harnessExpression().table as Record<string, unknown>).rows as Array<
      Record<string, number>
    >;
    // The appended row keeps strict monotonicity (last argument + 1).
    expect(rows).toHaveLength(3);
    expect(rows[2].argument).toBe(2);

    // Remove the middle row; the others survive in order.
    fireEvent.click(screen.getAllByTestId("rule-pack-table-remove-row")[1]);
    rows = (harnessExpression().table as Record<string, unknown>).rows as Array<
      Record<string, number>
    >;
    expect(rows.map((row) => row.argument)).toEqual([0, 2]);

    fireEvent.click(screen.getAllByTestId("rule-pack-table-remove-row")[1]);
    rows = (harnessExpression().table as Record<string, unknown>).rows as Array<
      Record<string, number>
    >;
    expect(rows).toHaveLength(1);
    // The last row cannot be removed (schema requires at least one row).
    expect(screen.getByTestId("rule-pack-table-remove-row")).toHaveProperty("disabled", true);
  });

  it("edits a table node's argument expression while preserving the table verbatim", () => {
    const tableNode: AstNode = {
      node: "interpolate",
      table: {
        table_id: "t1",
        argument_dimension: "temperature",
        argument_unit_ref: "invented_temp",
        result_dimension: "stress",
        result_unit_ref: "invented_stress",
        rows: [
          { argument: 1, result: 2 },
          { argument: 4, result: 8 }
        ]
      },
      argument: { node: "variable_ref", variable_id: "user_required_input_1" }
    };
    const document = setFormulaExpression(buildDraftRulePackDocument(), "user_formula_1", tableNode);
    render(<Harness initial={document} />);

    // Two node-type selects: [0] the root interpolate, [1] the argument child.
    // Switch the argument to a literal; the table rows and metadata must be
    // untouched — only the argument subtree changes.
    fireEvent.change(screen.getAllByTestId("rule-pack-node-type")[1], { target: { value: "literal" } });
    const node = harnessExpression();
    expect((node.argument as AstNode).node).toBe("literal");
    expect(node.table).toEqual(tableNode.table);
  });

  it("preserves an unrecognized node when an editable sibling is edited", () => {
    const document = buildDraftRulePackDocument();
    const unknownNode: AstNode = { node: "future_feature", custom_field: "value", nested: { keep: true } };
    const withUnknownSibling = setFormulaExpression(document, "user_formula_1", {
      node: "logical",
      operator: "and",
      left: { node: "compare", operator: "less_than", left: { node: "variable_ref", variable_id: "user_required_input_1" }, right: { node: "literal", quantity: { value: 1, dimension: "TBD", unit_ref: "TBD" } } },
      right: unknownNode
    });
    render(<Harness initial={withUnknownSibling} />);
    // The unknown sibling renders read-only with an honest explanation.
    expect(screen.getByTestId("rule-pack-node-readonly").textContent).toContain("Unrecognized node");

    // Edit the editable side (flip the root logical operator); the unknown
    // sibling subtree must survive intact, not be dropped or rewritten.
    fireEvent.change(screen.getByTestId("rule-pack-logical-operator"), { target: { value: "or" } });
    const expression = harnessExpression();
    expect(expression.operator).toBe("or");
    expect(expression.right).toEqual(unknownNode);
  });

  it("preserves a refusal-marker node read-only with no node-type selector", () => {
    // Refusal markers (unsupported_form / unsafe_host_access) are always
    // read-only. This pins the preservation guarantee the removed table
    // read-only test used to hold: a preserved node exposes NO node-type
    // selector (so it can never be silently switched/replaced) and the entire
    // subtree round-trips byte-for-byte.
    const marker: AstNode = { node: "unsafe_host_access", request: "filesystem" };
    const document = setFormulaExpression(buildDraftRulePackDocument(), "user_formula_1", marker);
    render(<Harness initial={document} />);
    expect(screen.getByTestId("rule-pack-node-readonly").textContent).toContain("Refusal marker");
    expect(screen.queryAllByTestId("rule-pack-node-type")).toHaveLength(0);
    expect(harnessExpression()).toEqual(marker);
  });
});
