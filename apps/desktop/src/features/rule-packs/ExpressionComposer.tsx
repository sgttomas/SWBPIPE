import { useState, type ReactNode } from "react";

// Structured AST expression composer (Phase C2 slice 2, TP-C2-COMPOSER-001).
//
// PRD §14.5 requires the rule-pack editor to provide an "Expression editor".
// Slice 1 (TP-C2-EDITOR-001) authored the formula expression as raw
// declarative-AST JSON inside the document textarea. This component replaces
// that hand-editing with a recursive, form/tree composer that builds the
// frozen grammar v1.0.0 typed AST (DEC-022) directly, so a user never writes
// node-tagged JSON by hand.
//
// Boundary (D-02b AWAITING_RULING): the composer is purely structured. There
// is no writable expression *text* syntax and no text *rendering* of the AST
// — whether even a read-only rendering is permitted is itself an open D-02b
// question (§3 Q5). The typed AST remains the sole canonical, checksum-bound
// form (DEC-022); this component only reads and rewrites that AST.
//
// Lossless preservation: table-backed nodes (interpolate / lookup) need a
// table sub-editor that is the next C2 slice, and any unrecognized node tag
// is treated as opaque. Both are rendered read-only and preserved verbatim —
// the composer never silently drops or rewrites a subtree it does not edit.

export type AstNode = Record<string, unknown>;
export type RulePackDocument = Record<string, unknown>;

// The node types the composer can author as structured controls. Table nodes
// (interpolate/lookup) and the refusal markers (unsupported_form /
// unsafe_host_access) are intentionally excluded: they are preserved when
// present but not user-authorable in this slice.
export const EDITABLE_NODE_TYPES = [
  "literal",
  "variable_ref",
  "unary",
  "binary",
  "compare",
  "logical",
  "select",
  "aggregate"
] as const;
export type EditableNodeType = (typeof EDITABLE_NODE_TYPES)[number];

export const UNARY_OPERATORS = ["negate", "abs", "not"] as const;
export const BINARY_OPERATORS = ["add", "subtract", "multiply", "divide"] as const;
export const COMPARE_OPERATORS = [
  "less_than",
  "less_than_or_equal",
  "greater_than",
  "greater_than_or_equal",
  "equal",
  "not_equal"
] as const;
export const LOGICAL_OPERATORS = ["and", "or"] as const;
export const AGGREGATE_FUNCTIONS = ["min", "max"] as const;

// The snake_case Dimension vocabulary from
// `core/rules/expression_evaluator` (the evaluator's `Dimension` enum). Kept
// in sync by the conformance corpus; `tbd` is the honest authoring default
// (no silent dimension assumption — CONTRACT no-silent-defaults).
export const DIMENSIONS = [
  "dimensionless",
  "length",
  "mass",
  "time",
  "temperature",
  "temperature_interval",
  "angle",
  "rotation",
  "force",
  "moment",
  "pressure",
  "stress",
  "area",
  "volume",
  "density",
  "linear_stiffness",
  "rotational_stiffness",
  "displacement",
  "velocity",
  "acceleration",
  "thermal_conductivity",
  "specific_heat",
  "thermal_expansion_coefficient",
  "second_moment_area",
  "section_modulus",
  "mass_per_length",
  "volume_per_length",
  "slope",
  "tbd"
] as const;

export type RulePackVariable = {
  id: string;
  kind: "required_input" | "value_slot";
  name: string;
};

export type FormulaDeclaration = {
  index: number;
  id: string;
  kind: string;
  expression: AstNode;
};

export type ParsedRulePackDocument =
  | { ok: true; document: RulePackDocument }
  | { ok: false; error: string };

function asObject(value: unknown): Record<string, unknown> | null {
  if (typeof value !== "object" || value === null || Array.isArray(value)) return null;
  return value as Record<string, unknown>;
}

function asString(value: unknown): string | null {
  return typeof value === "string" ? value : null;
}

/** Parse the draft document text into an object, honestly reporting failure. */
export function parseRulePackDocument(text: string | null): ParsedRulePackDocument {
  if (text === null) return { ok: false, error: "no-draft" };
  let parsed: unknown;
  try {
    parsed = JSON.parse(text);
  } catch (error) {
    return { ok: false, error: `invalid-json: ${String(error)}` };
  }
  const document = asObject(parsed);
  if (!document) return { ok: false, error: "not-an-object" };
  return { ok: true, document };
}

/**
 * Collect the variable identifiers a `variable_ref` may bind: declared
 * required inputs and value slots. These are the rule pack's own declared
 * variables; solver-result fields are bound at evaluation time, not authored
 * here, so they are not invented into the picker.
 */
export function collectRulePackVariables(document: RulePackDocument): RulePackVariable[] {
  const variables: RulePackVariable[] = [];
  const requiredInputs = document.required_inputs;
  if (Array.isArray(requiredInputs)) {
    for (const entry of requiredInputs) {
      const record = asObject(entry);
      const id = record ? asString(record.input_id) : null;
      if (id) {
        variables.push({
          id,
          kind: "required_input",
          name: (record && asString(record.name)) ?? id
        });
      }
    }
  }
  const valueSlots = document.value_slots;
  if (Array.isArray(valueSlots)) {
    for (const entry of valueSlots) {
      const record = asObject(entry);
      const id = record ? asString(record.slot_id) : null;
      if (id) {
        variables.push({
          id,
          kind: "value_slot",
          name: (record && (asString(record.slot_kind) ?? asString(record.name))) ?? id
        });
      }
    }
  }
  return variables;
}

/** Read the formula declarations and their expression ASTs from a document. */
export function readFormulaDeclarations(document: RulePackDocument): FormulaDeclaration[] {
  const declarations = document.formula_declarations;
  if (!Array.isArray(declarations)) return [];
  const formulas: FormulaDeclaration[] = [];
  declarations.forEach((entry, index) => {
    const record = asObject(entry);
    if (!record) return;
    const id = asString(record.formula_id) ?? `formula_${index}`;
    const kind = asString(record.formula_kind) ?? "derived_quantity";
    const payload = asObject(record.declaration_payload);
    const expression = (payload && asObject(payload.expression_ast)) ?? { node: "unrecognized" };
    formulas.push({ index, id, kind, expression });
  });
  return formulas;
}

/**
 * Return a new document with `formulaId`'s expression AST replaced. Every
 * other member (and every other formula) is preserved by structural copy.
 */
export function setFormulaExpression(
  document: RulePackDocument,
  formulaId: string,
  expression: AstNode
): RulePackDocument {
  const declarations = document.formula_declarations;
  if (!Array.isArray(declarations)) return document;
  const next = declarations.map((entry) => {
    const record = asObject(entry);
    if (!record || asString(record.formula_id) !== formulaId) return entry;
    const payload = asObject(record.declaration_payload) ?? {};
    return {
      ...record,
      declaration_payload: { ...payload, expression_ast: expression }
    };
  });
  return { ...document, formula_declarations: next };
}

/** The declared `node` tag of an AST node, or "unrecognized" when absent. */
export function nodeKind(node: AstNode): string {
  return asString(node.node) ?? "unrecognized";
}

export function isEditableNode(node: AstNode): boolean {
  return (EDITABLE_NODE_TYPES as readonly string[]).includes(nodeKind(node));
}

export function isTableNode(node: AstNode): boolean {
  const kind = nodeKind(node);
  return kind === "interpolate" || kind === "lookup";
}

function defaultLiteral(): AstNode {
  return {
    node: "literal",
    quantity: {
      value: 0,
      dimension: "tbd",
      unit_ref: "TBD",
      unit_required: true,
      dimension_check_required: true
    }
  };
}

function defaultVariableRef(variables: RulePackVariable[]): AstNode {
  return { node: "variable_ref", variable_id: variables[0]?.id ?? "" };
}

/** A sensible default node of `type`, used when creating or switching nodes. */
export function defaultExpressionNode(
  type: EditableNodeType,
  variables: RulePackVariable[]
): AstNode {
  switch (type) {
    case "literal":
      return defaultLiteral();
    case "variable_ref":
      return defaultVariableRef(variables);
    case "unary":
      return { node: "unary", operator: "negate", operand: defaultVariableRef(variables) };
    case "binary":
      return {
        node: "binary",
        operator: "add",
        left: defaultVariableRef(variables),
        right: defaultLiteral()
      };
    case "compare":
      return {
        node: "compare",
        operator: "less_than_or_equal",
        left: defaultVariableRef(variables),
        right: defaultLiteral()
      };
    case "logical":
      return {
        node: "logical",
        operator: "and",
        left: defaultExpressionNode("compare", variables),
        right: defaultExpressionNode("compare", variables)
      };
    case "select":
      return {
        node: "select",
        condition: defaultExpressionNode("compare", variables),
        then: defaultVariableRef(variables),
        else: defaultVariableRef(variables)
      };
    case "aggregate":
      return { node: "aggregate", function: "min", operands: [defaultVariableRef(variables)] };
    default:
      return defaultLiteral();
  }
}

function quantityOf(node: AstNode): Record<string, unknown> {
  return asObject(node.quantity) ?? {};
}

function childNode(node: AstNode, key: string): AstNode {
  return asObject(node[key]) ?? { node: "unrecognized" };
}

function operandsOf(node: AstNode): AstNode[] {
  const operands = node.operands;
  if (!Array.isArray(operands)) return [];
  return operands.map((entry) => asObject(entry) ?? { node: "unrecognized" });
}

// ---------------------------------------------------------------------------
// Components
// ---------------------------------------------------------------------------

type NodeEditorProps = {
  node: AstNode;
  onChange: (next: AstNode) => void;
  variables: RulePackVariable[];
  disabled: boolean;
  disabledReason?: string;
  depth: number;
  label: string;
};

function NodeShell({
  kind,
  label,
  depth,
  children
}: {
  kind: string;
  label: string;
  depth: number;
  children: ReactNode;
}) {
  return (
    <div
      className="rule-pack-node operation-record"
      data-testid="rule-pack-node"
      data-node-kind={kind}
      style={{ marginLeft: depth > 0 ? 12 : 0 }}
    >
      <small className="rule-pack-node-label">
        {label}: <strong data-testid="rule-pack-node-kind">{kind}</strong>
      </small>
      {children}
    </div>
  );
}

function NodeTypeSelect({
  kind,
  variables,
  disabled,
  onChange
}: {
  kind: string;
  variables: RulePackVariable[];
  disabled: boolean;
  onChange: (next: AstNode) => void;
}) {
  return (
    <label className="rule-pack-node-field">
      <span>node</span>
      <select
        data-testid="rule-pack-node-type"
        value={kind}
        disabled={disabled}
        onChange={(event) =>
          onChange(defaultExpressionNode(event.target.value as EditableNodeType, variables))
        }
      >
        {EDITABLE_NODE_TYPES.map((type) => (
          <option key={type} value={type}>
            {type}
          </option>
        ))}
      </select>
    </label>
  );
}

function ExpressionNodeEditor(props: NodeEditorProps) {
  const { node, onChange, variables, disabled, disabledReason, depth, label } = props;
  const kind = nodeKind(node);

  // Read-only, preserved nodes: table-backed (next slice) and anything the
  // composer does not recognize. The subtree round-trips unchanged.
  if (!isEditableNode(node)) {
    const explanation = isTableNode(node)
      ? `Table-backed ${kind} node. The structured composer does not yet edit table nodes; edit it in the document JSON below. Preserved unchanged.`
      : kind === "unsupported_form" || kind === "unsafe_host_access"
        ? `Refusal marker (${kind}); blocking by design. Preserved; not user-authorable in the composer.`
        : `Unrecognized node (${kind}). Preserved as-is; edit it in the document JSON below.`;
    return (
      <NodeShell kind={kind} label={label} depth={depth}>
        <small className="rule-pack-node-readonly" data-testid="rule-pack-node-readonly">
          {explanation}
        </small>
      </NodeShell>
    );
  }

  const typeSelect = (
    <NodeTypeSelect kind={kind} variables={variables} disabled={disabled} onChange={onChange} />
  );

  if (kind === "literal") {
    const quantity = quantityOf(node);
    const value = typeof quantity.value === "number" ? quantity.value : 0;
    const dimension = asString(quantity.dimension) ?? "tbd";
    const unitRef = asString(quantity.unit_ref) ?? "TBD";
    const updateQuantity = (patch: Record<string, unknown>) =>
      onChange({ ...node, quantity: { ...quantity, ...patch } });
    return (
      <NodeShell kind={kind} label={label} depth={depth}>
        {typeSelect}
        <label className="rule-pack-node-field">
          <span>value</span>
          <input
            type="number"
            data-testid="rule-pack-literal-value"
            value={String(value)}
            disabled={disabled}
            onChange={(event) => {
              const next = Number(event.target.value);
              if (Number.isFinite(next)) updateQuantity({ value: next });
            }}
          />
        </label>
        <label className="rule-pack-node-field">
          <span>dimension</span>
          <select
            data-testid="rule-pack-literal-dimension"
            value={DIMENSIONS.includes(dimension as (typeof DIMENSIONS)[number]) ? dimension : "tbd"}
            disabled={disabled}
            onChange={(event) => updateQuantity({ dimension: event.target.value })}
          >
            {DIMENSIONS.map((option) => (
              <option key={option} value={option}>
                {option}
              </option>
            ))}
          </select>
        </label>
        <label className="rule-pack-node-field">
          <span>unit_ref</span>
          <input
            type="text"
            data-testid="rule-pack-literal-unit"
            value={unitRef}
            disabled={disabled}
            onChange={(event) => updateQuantity({ unit_ref: event.target.value })}
          />
        </label>
      </NodeShell>
    );
  }

  if (kind === "variable_ref") {
    const variableId = asString(node.variable_id) ?? "";
    const known = variables.some((variable) => variable.id === variableId);
    return (
      <NodeShell kind={kind} label={label} depth={depth}>
        {typeSelect}
        <label className="rule-pack-node-field">
          <span>variable</span>
          <select
            data-testid="rule-pack-variable-ref"
            value={variableId}
            disabled={disabled}
            onChange={(event) => onChange({ ...node, variable_id: event.target.value })}
          >
            {variables.length === 0 ? <option value="">(no declared inputs or slots)</option> : null}
            {!known && variableId !== "" ? (
              <option value={variableId}>(current) {variableId}</option>
            ) : null}
            {variables.map((variable) => (
              <option key={variable.id} value={variable.id}>
                {variable.id} ({variable.kind})
              </option>
            ))}
          </select>
        </label>
        {variableId === "" ? (
          <small className="rule-pack-node-readonly" data-testid="rule-pack-variable-ref-empty">
            No variable selected; validation will report a missing reference until one is chosen.
          </small>
        ) : null}
      </NodeShell>
    );
  }

  if (kind === "unary") {
    const operator = asString(node.operator) ?? "negate";
    return (
      <NodeShell kind={kind} label={label} depth={depth}>
        {typeSelect}
        <label className="rule-pack-node-field">
          <span>operator</span>
          <select
            data-testid="rule-pack-unary-operator"
            value={operator}
            disabled={disabled}
            onChange={(event) => onChange({ ...node, operator: event.target.value })}
          >
            {UNARY_OPERATORS.map((option) => (
              <option key={option} value={option}>
                {option}
              </option>
            ))}
          </select>
        </label>
        <ExpressionNodeEditor
          node={childNode(node, "operand")}
          onChange={(next) => onChange({ ...node, operand: next })}
          variables={variables}
          disabled={disabled}
          disabledReason={disabledReason}
          depth={depth + 1}
          label="operand"
        />
      </NodeShell>
    );
  }

  if (kind === "binary" || kind === "compare" || kind === "logical") {
    const operator = asString(node.operator) ?? "";
    const operators =
      kind === "binary"
        ? BINARY_OPERATORS
        : kind === "compare"
          ? COMPARE_OPERATORS
          : LOGICAL_OPERATORS;
    const testId =
      kind === "binary"
        ? "rule-pack-binary-operator"
        : kind === "compare"
          ? "rule-pack-compare-operator"
          : "rule-pack-logical-operator";
    return (
      <NodeShell kind={kind} label={label} depth={depth}>
        {typeSelect}
        <label className="rule-pack-node-field">
          <span>operator</span>
          <select
            data-testid={testId}
            value={operator}
            disabled={disabled}
            onChange={(event) => onChange({ ...node, operator: event.target.value })}
          >
            {operators.map((option) => (
              <option key={option} value={option}>
                {option}
              </option>
            ))}
          </select>
        </label>
        <ExpressionNodeEditor
          node={childNode(node, "left")}
          onChange={(next) => onChange({ ...node, left: next })}
          variables={variables}
          disabled={disabled}
          disabledReason={disabledReason}
          depth={depth + 1}
          label="left"
        />
        <ExpressionNodeEditor
          node={childNode(node, "right")}
          onChange={(next) => onChange({ ...node, right: next })}
          variables={variables}
          disabled={disabled}
          disabledReason={disabledReason}
          depth={depth + 1}
          label="right"
        />
      </NodeShell>
    );
  }

  if (kind === "select") {
    return (
      <NodeShell kind={kind} label={label} depth={depth}>
        {typeSelect}
        <ExpressionNodeEditor
          node={childNode(node, "condition")}
          onChange={(next) => onChange({ ...node, condition: next })}
          variables={variables}
          disabled={disabled}
          disabledReason={disabledReason}
          depth={depth + 1}
          label="condition"
        />
        <ExpressionNodeEditor
          node={childNode(node, "then")}
          onChange={(next) => onChange({ ...node, then: next })}
          variables={variables}
          disabled={disabled}
          disabledReason={disabledReason}
          depth={depth + 1}
          label="then"
        />
        <ExpressionNodeEditor
          node={childNode(node, "else")}
          onChange={(next) => onChange({ ...node, else: next })}
          variables={variables}
          disabled={disabled}
          disabledReason={disabledReason}
          depth={depth + 1}
          label="else"
        />
      </NodeShell>
    );
  }

  // aggregate
  const aggregateFunction = asString(node.function) ?? "min";
  const operands = operandsOf(node);
  const removeDisabledReason =
    operands.length <= 1 ? "An aggregate needs at least one operand." : undefined;
  return (
    <NodeShell kind={kind} label={label} depth={depth}>
      {typeSelect}
      <label className="rule-pack-node-field">
        <span>function</span>
        <select
          data-testid="rule-pack-aggregate-function"
          value={aggregateFunction}
          disabled={disabled}
          onChange={(event) => onChange({ ...node, function: event.target.value })}
        >
          {AGGREGATE_FUNCTIONS.map((option) => (
            <option key={option} value={option}>
              {option}
            </option>
          ))}
        </select>
      </label>
      {operands.map((operand, index) => (
        <div className="rule-pack-operand" key={`operand-${index}`}>
          <ExpressionNodeEditor
            node={operand}
            onChange={(next) =>
              onChange({
                ...node,
                operands: operands.map((current, position) => (position === index ? next : current))
              })
            }
            variables={variables}
            disabled={disabled}
            disabledReason={disabledReason}
            depth={depth + 1}
            label={`operand ${index + 1}`}
          />
          <button
            type="button"
            data-testid="rule-pack-aggregate-remove-operand"
            disabled={disabled || operands.length <= 1}
            title={disabled ? disabledReason : removeDisabledReason}
            onClick={() =>
              onChange({
                ...node,
                operands: operands.filter((_, position) => position !== index)
              })
            }
          >
            Remove operand
          </button>
        </div>
      ))}
      <button
        type="button"
        data-testid="rule-pack-aggregate-add-operand"
        disabled={disabled}
        title={disabled ? disabledReason : undefined}
        onClick={() =>
          onChange({ ...node, operands: [...operands, defaultVariableRef(variables)] })
        }
      >
        Add operand
      </button>
    </NodeShell>
  );
}

export type ExpressionComposerProps = {
  document: RulePackDocument;
  onChange: (next: RulePackDocument) => void;
  disabled: boolean;
  disabledReason?: string;
};

/**
 * The full composer surface: a variable browser, a formula selector, and the
 * recursive node editor for the selected formula's expression AST. Edits are
 * written straight back into a structurally-copied document and handed to
 * `onChange`; the parent owns serialization and persistence.
 */
export function ExpressionComposer({
  document,
  onChange,
  disabled,
  disabledReason
}: ExpressionComposerProps) {
  const variables = collectRulePackVariables(document);
  const formulas = readFormulaDeclarations(document);
  const [selectedFormulaId, setSelectedFormulaId] = useState<string | null>(null);
  const activeFormulaId =
    selectedFormulaId && formulas.some((formula) => formula.id === selectedFormulaId)
      ? selectedFormulaId
      : (formulas[0]?.id ?? null);
  const activeFormula = formulas.find((formula) => formula.id === activeFormulaId) ?? null;

  return (
    <div className="report-list rule-pack-expression-composer" data-testid="rule-pack-expression-composer">
      <strong>Expression composer (structured AST; no text syntax — D-02b)</strong>

      <div className="rule-pack-variable-browser" data-testid="rule-pack-variable-browser">
        <small>
          Declared variables ({variables.length}):{" "}
          {variables.length === 0
            ? "none yet — add a required input or value slot in the document below."
            : variables.map((variable) => `${variable.id} (${variable.kind})`).join(", ")}
        </small>
      </div>

      {formulas.length === 0 ? (
        <small data-testid="rule-pack-composer-empty">
          No formula declarations in this document; add one in the document JSON below to compose an
          expression.
        </small>
      ) : (
        <>
          <label className="rule-pack-node-field">
            <span>formula</span>
            <select
              data-testid="rule-pack-formula-select"
              value={activeFormulaId ?? ""}
              disabled={disabled}
              onChange={(event) => setSelectedFormulaId(event.target.value)}
            >
              {formulas.map((formula) => (
                <option key={formula.id} value={formula.id}>
                  {formula.id} ({formula.kind})
                </option>
              ))}
            </select>
          </label>
          {activeFormula ? (
            <ExpressionNodeEditor
              node={activeFormula.expression}
              onChange={(next) => onChange(setFormulaExpression(document, activeFormula.id, next))}
              variables={variables}
              disabled={disabled}
              disabledReason={disabledReason}
              depth={0}
              label="expression"
            />
          ) : null}
        </>
      )}
    </div>
  );
}
