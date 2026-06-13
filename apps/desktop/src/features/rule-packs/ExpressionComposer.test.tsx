import { useState } from "react";
import { cleanup, fireEvent, render, screen } from "@testing-library/react";
import { afterEach, describe, expect, it } from "vitest";
import {
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

// TP-C2-COMPOSER-001 — structured AST expression composer.
// Pure-helper tests pin the grammar v1.0.0 node encoding and the lossless
// document rewrite; component tests drive the composer through visible
// controls and assert the rewritten document.

afterEach(cleanup);

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
    expect(defaultExpressionNode("literal", variables)).toMatchObject({
      node: "literal",
      quantity: { value: 0, dimension: "tbd", unit_ref: "TBD" }
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
  });

  it("classifies node kinds (editable vs table vs unknown)", () => {
    expect(nodeKind({ node: "compare" })).toBe("compare");
    expect(nodeKind({})).toBe("unrecognized");
    expect(isEditableNode({ node: "logical" })).toBe(true);
    expect(isEditableNode({ node: "interpolate" })).toBe(false);
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

  it("preserves a table-backed node read-only instead of editing or dropping it", () => {
    const document = buildDraftRulePackDocument();
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
    const withTable = setFormulaExpression(document, "user_formula_1", tableNode);
    render(<Harness initial={withTable} />);
    expect(screen.getByTestId("rule-pack-node-readonly").textContent).toContain("Table-backed");
    // No node-type select is offered for a preserved node, so it cannot be
    // silently replaced. Assert the ENTIRE subtree (table rows and all) round-
    // trips byte-for-byte — a partial match would not catch a dropped table.
    expect(screen.queryAllByTestId("rule-pack-node-type")).toHaveLength(0);
    expect(harnessExpression()).toEqual(tableNode);
  });

  it("preserves an unrecognized node when an editable sibling is edited", () => {
    const document = buildDraftRulePackDocument();
    const unknownNode: AstNode = { node: "future_feature", custom_field: "value", nested: { keep: true } };
    const withUnknownSibling = setFormulaExpression(document, "user_formula_1", {
      node: "logical",
      operator: "and",
      left: { node: "compare", operator: "less_than", left: { node: "variable_ref", variable_id: "user_required_input_1" }, right: { node: "literal", quantity: { value: 1, dimension: "tbd", unit_ref: "TBD" } } },
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
});
