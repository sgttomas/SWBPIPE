import { useEffect, useState, type ReactNode } from "react";
import {
  acceptedUnits,
  describeUnitBasis,
  loadUnitCatalog,
  unitDimensionValidationStatus,
  unitEntryMatchesDimension,
  type UnitCatalogRoute
} from "../../services/unitCatalogService";

// Structured AST expression composer (Phase C2 slice 2, TP-C2-COMPOSER-001).
//
// PRD §14.5 requires the rule-pack editor to provide an "Expression editor".
// Slice 1 (TP-C2-EDITOR-001) authored the formula expression as raw
// declarative-AST JSON inside the document textarea. This component replaces
// that hand-editing with a recursive, form/tree composer that builds the
// frozen grammar v1.0.0 typed AST (DEC-022) directly, so a user never writes
// node-tagged JSON by hand.
//
// Boundary (D-02b RULED, DEC-037): the composer is purely structured. It may
// show a labeled read-only one-way AST-to-text preview, but provides no
// writable expression text input and no parser. The typed AST remains the sole
// canonical, checksum-bound form (DEC-022); this component only reads and
// rewrites that AST.
//
// Table-backed nodes (interpolate / lookup) are authored through a structured
// table sub-editor (TP-C2-TABLENODE-001): table id, argument/result dimension
// and unit refs, and the {argument, result} rows, plus the recursive argument
// expression. Monotonicity and out-of-range remain evaluator-enforced
// diagnostics (surfaced as authoring guidance, never silently clamped here).
//
// Unit I/O (TP-UNITS-B2-RULEEXPRUNITS-001): expression literal/table unit refs
// use the reviewed DEC-018 catalog when the desktop backend exposes it.
// Browser preview keeps manual unit text instead of synthesizing a fallback
// catalog; stored out-of-catalog units remain visible and selectable as the
// current value.
//
// Lossless preservation: any unrecognized node tag and the refusal markers
// (unsupported_form / unsafe_host_access) are rendered read-only and preserved
// verbatim — the composer never silently drops or rewrites a subtree it does
// not edit. Untouched table rows and sibling subtrees round-trip unchanged.

export type AstNode = Record<string, unknown>;
export type RulePackDocument = Record<string, unknown>;

// The node types the composer can author as structured controls — now the
// full frozen grammar v1.0.0 set including the table-backed nodes. Only the
// refusal markers (unsupported_form / unsafe_host_access) and any unrecognized
// node tag are excluded: they are preserved when present but never authored.
export const EDITABLE_NODE_TYPES = [
  "literal",
  "variable_ref",
  "unary",
  "binary",
  "compare",
  "logical",
  "select",
  "aggregate",
  "interpolate",
  "lookup"
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
export const LOOKUP_MODES = ["exact", "step"] as const;

// The dimension vocabulary the document codec and schema accept
// (`core/rules/rule_pack_document` DIMENSION_TOKENS == schema `DimensionId`;
// the evaluator's `Dimension` enum encodes the same tokens). The
// unknown-dimension placeholder is the uppercase `"TBD"` token — exactly what
// the codec, schema, and draft builder use, not a lowercase variant — and is
// the honest authoring default (no silent dimension assumption — CONTRACT
// no-silent-defaults). A lowercase `"tbd"` would fail backend decode
// (`decode_quantity`) and `DimensionId` schema validation.
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
  "TBD"
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
  // Only { value, dimension, unit_ref }: the schema's ExpressionQuantity is
  // additionalProperties:false, and the relaxation flags unit_required /
  // dimension_check_required are deliberately NOT authorable in pack documents
  // — the codec (decode_quantity) and evaluator treat absent flags as true
  // (i.e. strict). Emitting them would make the default literal non-conformant
  // to rule_pack.schema.yaml. (An opened document's literal preserves whatever
  // flags it already carries; only this authoring default omits them.)
  return {
    node: "literal",
    quantity: {
      value: 0,
      dimension: "TBD",
      unit_ref: "TBD"
    }
  };
}

function defaultVariableRef(variables: RulePackVariable[]): AstNode {
  return { node: "variable_ref", variable_id: variables[0]?.id ?? "" };
}

/**
 * A schema-valid default user table: two strictly-increasing rows so an
 * `interpolate` node is immediately well-formed (the evaluator requires at
 * least two rows and strict monotonicity), with the honest `"TBD"`
 * dimension/unit placeholders the codec and schema accept.
 */
function defaultUserTable(): Record<string, unknown> {
  return {
    table_id: "user_table_1",
    argument_dimension: "TBD",
    argument_unit_ref: "TBD",
    result_dimension: "TBD",
    result_unit_ref: "TBD",
    rows: [
      { argument: 0, result: 0 },
      { argument: 1, result: 0 }
    ]
  };
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
    case "interpolate":
      return {
        node: "interpolate",
        table: defaultUserTable(),
        argument: defaultVariableRef(variables)
      };
    case "lookup":
      return {
        node: "lookup",
        table: defaultUserTable(),
        mode: "exact",
        argument: defaultVariableRef(variables)
      };
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

function tableOf(node: AstNode): Record<string, unknown> {
  return asObject(node.table) ?? {};
}

/** The raw `rows` array of a table node, preserved verbatim for lossless edits. */
function rawTableRows(table: Record<string, unknown>): unknown[] {
  return Array.isArray(table.rows) ? table.rows : [];
}

function rowNumber(row: unknown, key: "argument" | "result"): number {
  const record = asObject(row);
  const value = record ? record[key] : undefined;
  return typeof value === "number" ? value : 0;
}

function expressionOperator(node: AstNode, fallback: string): string {
  return asString(node.operator) ?? fallback;
}

function renderLiteralQuantity(node: AstNode): string {
  const quantity = quantityOf(node);
  const value = typeof quantity.value === "number" || typeof quantity.value === "string" ? String(quantity.value) : "?";
  const unit = asString(quantity.unit_ref) ?? "TBD";
  const dimension = asString(quantity.dimension) ?? "TBD";
  return `${value} ${unit} [${dimension}]`;
}

function renderTableName(node: AstNode): string {
  return asString(tableOf(node).table_id) ?? "table:TBD";
}

/**
 * Deterministic read-only rendering for DEC-037's permitted one-way AST text
 * preview. This is display-only: no parser consumes this string, and the AST
 * remains the canonical authored/checksummed rule-pack member.
 */
export function renderExpressionText(node: AstNode): string {
  const kind = nodeKind(node);
  if (kind === "literal") return renderLiteralQuantity(node);
  if (kind === "variable_ref") return asString(node.variable_id) ?? "<missing_variable>";
  if (kind === "unary") {
    const operand = renderExpressionText(childNode(node, "operand"));
    const operator = expressionOperator(node, "negate");
    if (operator === "negate") return `-(${operand})`;
    return `${operator}(${operand})`;
  }
  if (kind === "binary" || kind === "compare" || kind === "logical") {
    const operator = expressionOperator(node, kind === "compare" ? "less_than_or_equal" : "and");
    const textOperator =
      {
        add: "+",
        subtract: "-",
        multiply: "*",
        divide: "/",
        less_than: "<",
        less_than_or_equal: "<=",
        greater_than: ">",
        greater_than_or_equal: ">=",
        equal: "==",
        not_equal: "!=",
        and: "and",
        or: "or"
      }[operator] ?? operator;
    return `(${renderExpressionText(childNode(node, "left"))} ${textOperator} ${renderExpressionText(
      childNode(node, "right")
    )})`;
  }
  if (kind === "select") {
    return `if ${renderExpressionText(childNode(node, "condition"))} then ${renderExpressionText(
      childNode(node, "then")
    )} else ${renderExpressionText(childNode(node, "else"))}`;
  }
  if (kind === "aggregate") {
    const aggregateFunction = asString(node.function) ?? "min";
    const operands = operandsOf(node).map((operand) => renderExpressionText(operand));
    return `${aggregateFunction}(${operands.join(", ")})`;
  }
  if (kind === "interpolate") {
    return `interpolate(${renderTableName(node)}, ${renderExpressionText(childNode(node, "argument"))})`;
  }
  if (kind === "lookup") {
    const mode = asString(node.mode) ?? "exact";
    return `lookup:${mode}(${renderTableName(node)}, ${renderExpressionText(childNode(node, "argument"))})`;
  }
  if (kind === "unsupported_form" || kind === "unsafe_host_access") {
    return `[refusal:${kind}]`;
  }
  return `[unrecognized:${kind}]`;
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
  unitCatalogRoute: UnitCatalogRoute | null;
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

// Shared dimension picker over the codec/schema vocabulary. Reused by the
// literal editor and both table dimension fields so the unknown-dimension
// placeholder (`"TBD"`) stays consistent and codec/schema-valid everywhere.
// Exported so the declarations editor (required-input / value-slot
// quantity_intent dimensions) draws from the same single vocabulary.
export function DimensionSelect({
  testId,
  value,
  disabled,
  onChange
}: {
  testId: string;
  value: string;
  disabled: boolean;
  onChange: (next: string) => void;
}) {
  // No silent display-snap: a stored token outside the current vocabulary
  // (e.g. from a newer schema) surfaces as a "(current)" option rather than
  // being shown as "TBD" while the document still holds the real token —
  // matching the declarations editor's EnumSelect honesty (CONTRACT
  // no-silent-defaults). A schema-conformant document never hits this path
  // (DimensionId is closed and "TBD" is in DIMENSIONS); nothing is written
  // unless the user picks an option.
  const known = (DIMENSIONS as readonly string[]).includes(value);
  return (
    <select
      data-testid={testId}
      value={value}
      disabled={disabled}
      onChange={(event) => onChange(event.target.value)}
    >
      {!known ? <option value={value}>(current) {value}</option> : null}
      {DIMENSIONS.map((option) => (
        <option key={option} value={option}>
          {option}
        </option>
      ))}
    </select>
  );
}

type UnitOption = {
  symbol: string;
  label: string;
};

function hasTauriInternals(): boolean {
  return typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;
}

function unitOptions(route: UnitCatalogRoute | null, dimensionId: string, currentSymbol: string): UnitOption[] {
  const current = currentSymbol.trim() || "TBD";
  const basis = describeUnitBasis(route, current, dimensionId);
  const fallback = { symbol: basis.symbol, label: basis.label };
  if (route?.route !== "tauri_unit_catalog") return [fallback];

  const options = acceptedUnits(route.catalog)
    .filter((entry) => unitEntryMatchesDimension(entry, dimensionId))
    .sort((left, right) => Number(right.canonical) - Number(left.canonical) || left.symbol.localeCompare(right.symbol))
    .map((entry) => {
      const entryBasis = describeUnitBasis(route, entry.symbol, dimensionId);
      return { symbol: entry.symbol, label: entryBasis.label };
    });

  if (!options.some((option) => option.symbol === fallback.symbol)) {
    options.unshift(fallback);
  }
  return options.length > 0 ? options : [fallback];
}

function UnitRefField({
  testId,
  value,
  dimension,
  disabled,
  unitCatalogRoute,
  onChange
}: {
  testId: string;
  value: string;
  dimension: string;
  disabled: boolean;
  unitCatalogRoute: UnitCatalogRoute | null;
  onChange: (next: string) => void;
}) {
  if (unitCatalogRoute?.route !== "tauri_unit_catalog") {
    return (
      <input
        type="text"
        data-testid={testId}
        value={value}
        disabled={disabled}
        onChange={(event) => onChange(event.target.value)}
      />
    );
  }

  const options = unitOptions(unitCatalogRoute, dimension, value);
  return (
    <select
      data-testid={testId}
      value={value.trim() || "TBD"}
      disabled={disabled}
      onChange={(event) => onChange(event.target.value)}
    >
      {options.map((option) => (
        <option key={option.symbol} value={option.symbol}>
          {option.label}
        </option>
      ))}
    </select>
  );
}

function ExpressionNodeEditor(props: NodeEditorProps) {
  const { node, onChange, variables, disabled, disabledReason, depth, label, unitCatalogRoute } = props;
  const kind = nodeKind(node);

  // Read-only, preserved nodes: the refusal markers (unsupported_form /
  // unsafe_host_access) and any unrecognized node tag. The subtree round-trips
  // unchanged. (Table-backed nodes are editable — handled below.)
  if (!isEditableNode(node)) {
    const explanation =
      kind === "unsupported_form" || kind === "unsafe_host_access"
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
    const dimension = asString(quantity.dimension) ?? "TBD";
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
          <DimensionSelect
            testId="rule-pack-literal-dimension"
            value={dimension}
            disabled={disabled}
            onChange={(next) => updateQuantity({ dimension: next })}
          />
        </label>
        <label className="rule-pack-node-field">
          <span>unit_ref</span>
          <UnitRefField
            testId="rule-pack-literal-unit"
            value={unitRef}
            dimension={dimension}
            disabled={disabled}
            unitCatalogRoute={unitCatalogRoute}
            onChange={(next) => updateQuantity({ unit_ref: next })}
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
          unitCatalogRoute={unitCatalogRoute}
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
          unitCatalogRoute={unitCatalogRoute}
          depth={depth + 1}
          label="left"
        />
        <ExpressionNodeEditor
          node={childNode(node, "right")}
          onChange={(next) => onChange({ ...node, right: next })}
          variables={variables}
          disabled={disabled}
          disabledReason={disabledReason}
          unitCatalogRoute={unitCatalogRoute}
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
          unitCatalogRoute={unitCatalogRoute}
          depth={depth + 1}
          label="condition"
        />
        <ExpressionNodeEditor
          node={childNode(node, "then")}
          onChange={(next) => onChange({ ...node, then: next })}
          variables={variables}
          disabled={disabled}
          disabledReason={disabledReason}
          unitCatalogRoute={unitCatalogRoute}
          depth={depth + 1}
          label="then"
        />
        <ExpressionNodeEditor
          node={childNode(node, "else")}
          onChange={(next) => onChange({ ...node, else: next })}
          variables={variables}
          disabled={disabled}
          disabledReason={disabledReason}
          unitCatalogRoute={unitCatalogRoute}
          depth={depth + 1}
          label="else"
        />
      </NodeShell>
    );
  }

  if (isTableNode(node)) {
    const isLookup = kind === "lookup";
    const table = tableOf(node);
    const tableId = asString(table.table_id) ?? "";
    const argumentDimension = asString(table.argument_dimension) ?? "TBD";
    const argumentUnitRef = asString(table.argument_unit_ref) ?? "TBD";
    const resultDimension = asString(table.result_dimension) ?? "TBD";
    const resultUnitRef = asString(table.result_unit_ref) ?? "TBD";
    const mode = asString(node.mode) ?? "exact";
    const rows = rawTableRows(table);
    const updateTable = (patch: Record<string, unknown>) =>
      onChange({ ...node, table: { ...table, ...patch } });
    // Patch only the touched row; every other row object round-trips verbatim.
    const updateRow = (index: number, patch: Record<string, unknown>) =>
      updateTable({
        rows: rows.map((row, position) =>
          position === index ? { ...(asObject(row) ?? {}), ...patch } : row
        )
      });
    const addRow = () => {
      const lastArgument = rows.length > 0 ? rowNumber(rows[rows.length - 1], "argument") : -1;
      updateTable({ rows: [...rows, { argument: lastArgument + 1, result: 0 }] });
    };
    const removeRow = (index: number) =>
      updateTable({ rows: rows.filter((_, position) => position !== index) });
    const removeRowReason = rows.length <= 1 ? "A table needs at least one row." : undefined;
    return (
      <NodeShell kind={kind} label={label} depth={depth}>
        {typeSelect}
        {isLookup ? (
          <label className="rule-pack-node-field">
            <span>mode</span>
            <select
              data-testid="rule-pack-lookup-mode"
              value={LOOKUP_MODES.includes(mode as (typeof LOOKUP_MODES)[number]) ? mode : "exact"}
              disabled={disabled}
              onChange={(event) => onChange({ ...node, mode: event.target.value })}
            >
              {LOOKUP_MODES.map((option) => (
                <option key={option} value={option}>
                  {option}
                </option>
              ))}
            </select>
          </label>
        ) : null}
        <label className="rule-pack-node-field">
          <span>table_id</span>
          <input
            type="text"
            data-testid="rule-pack-table-id"
            value={tableId}
            disabled={disabled}
            onChange={(event) => updateTable({ table_id: event.target.value })}
          />
        </label>
        <label className="rule-pack-node-field">
          <span>arg dimension</span>
          <DimensionSelect
            testId="rule-pack-table-argument-dimension"
            value={argumentDimension}
            disabled={disabled}
            onChange={(next) => updateTable({ argument_dimension: next })}
          />
        </label>
        <label className="rule-pack-node-field">
          <span>arg unit_ref</span>
          <UnitRefField
            testId="rule-pack-table-argument-unit"
            value={argumentUnitRef}
            dimension={argumentDimension}
            disabled={disabled}
            unitCatalogRoute={unitCatalogRoute}
            onChange={(next) => updateTable({ argument_unit_ref: next })}
          />
        </label>
        <label className="rule-pack-node-field">
          <span>result dimension</span>
          <DimensionSelect
            testId="rule-pack-table-result-dimension"
            value={resultDimension}
            disabled={disabled}
            onChange={(next) => updateTable({ result_dimension: next })}
          />
        </label>
        <label className="rule-pack-node-field">
          <span>result unit_ref</span>
          <UnitRefField
            testId="rule-pack-table-result-unit"
            value={resultUnitRef}
            dimension={resultDimension}
            disabled={disabled}
            unitCatalogRoute={unitCatalogRoute}
            onChange={(next) => updateTable({ result_unit_ref: next })}
          />
        </label>
        <small className="rule-pack-node-readonly" data-testid="rule-pack-table-guidance">
          Row arguments must be strictly increasing; interpolate needs at least two rows. The
          evaluator blocks out-of-range arguments — it never extrapolates or clamps.
        </small>
        {rows.map((row, index) => (
          <div className="rule-pack-table-row" data-testid="rule-pack-table-row" key={`row-${index}`}>
            <label className="rule-pack-node-field">
              <span>argument</span>
              <input
                type="number"
                data-testid="rule-pack-table-row-argument"
                value={String(rowNumber(row, "argument"))}
                disabled={disabled}
                onChange={(event) => {
                  const next = Number(event.target.value);
                  if (Number.isFinite(next)) updateRow(index, { argument: next });
                }}
              />
            </label>
            <label className="rule-pack-node-field">
              <span>result</span>
              <input
                type="number"
                data-testid="rule-pack-table-row-result"
                value={String(rowNumber(row, "result"))}
                disabled={disabled}
                onChange={(event) => {
                  const next = Number(event.target.value);
                  if (Number.isFinite(next)) updateRow(index, { result: next });
                }}
              />
            </label>
            <button
              type="button"
              data-testid="rule-pack-table-remove-row"
              disabled={disabled || rows.length <= 1}
              title={disabled ? disabledReason : removeRowReason}
              onClick={() => removeRow(index)}
            >
              Remove row
            </button>
          </div>
        ))}
        <button
          type="button"
          data-testid="rule-pack-table-add-row"
          disabled={disabled}
          title={disabled ? disabledReason : undefined}
          onClick={addRow}
        >
          Add row
        </button>
        <ExpressionNodeEditor
          node={childNode(node, "argument")}
          onChange={(next) => onChange({ ...node, argument: next })}
          variables={variables}
          disabled={disabled}
          disabledReason={disabledReason}
          unitCatalogRoute={unitCatalogRoute}
          depth={depth + 1}
          label="argument"
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
            unitCatalogRoute={unitCatalogRoute}
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
  const [unitCatalogRoute, setUnitCatalogRoute] = useState<UnitCatalogRoute | null>(null);
  const [selectedFormulaId, setSelectedFormulaId] = useState<string | null>(null);
  const activeFormulaId =
    selectedFormulaId && formulas.some((formula) => formula.id === selectedFormulaId)
      ? selectedFormulaId
      : (formulas[0]?.id ?? null);
  const activeFormula = formulas.find((formula) => formula.id === activeFormulaId) ?? null;

  useEffect(() => {
    if (!hasTauriInternals()) {
      setUnitCatalogRoute({
        route: "unavailable_browser_preview",
        diagnostic:
          "UNIT-CATALOG-DESKTOP-ONLY: browser preview preserves stored expression unit_ref text " +
          "and does not synthesize a DEC-018 fallback catalog."
      });
      return undefined;
    }
    let active = true;
    loadUnitCatalog()
      .then((route) => {
        if (active) setUnitCatalogRoute(route);
      })
      .catch((error) => {
        if (!active) return;
        setUnitCatalogRoute({
          route: "unavailable_browser_preview",
          diagnostic: `UNIT-CATALOG-LOAD-FAILED: ${String(error)}`
        });
      });
    return () => {
      active = false;
    };
  }, []);

  return (
    <div className="report-list rule-pack-expression-composer" data-testid="rule-pack-expression-composer">
      <strong>Expression composer (structured AST; no writable text syntax - DEC-037)</strong>

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
            <>
              <small className="rule-pack-node-readonly" data-testid="rule-pack-expression-unit-policy">
                {expressionUnitPolicySummary(unitCatalogRoute, activeFormula.expression)}
              </small>
              <div
                className="operation-record"
                data-testid="rule-pack-expression-text-preview"
                aria-label="Read-only expression text preview"
              >
                <small>
                  Read-only AST-to-text preview (DEC-037); this text is not editable and is never
                  parsed. The checksum-bound rule-pack member remains the structured AST.
                </small>
                <code>{renderExpressionText(activeFormula.expression)}</code>
              </div>
              <ExpressionNodeEditor
                node={activeFormula.expression}
                onChange={(next) => onChange(setFormulaExpression(document, activeFormula.id, next))}
                variables={variables}
                disabled={disabled}
                disabledReason={disabledReason}
                unitCatalogRoute={unitCatalogRoute}
                depth={0}
                label="expression"
              />
            </>
          ) : null}
        </>
      )}
    </div>
  );
}

function expressionUnitPolicySummary(route: UnitCatalogRoute | null, expression: AstNode): string {
  const refs = collectExpressionUnitRefs(expression);
  return [
    "unit_policy=stored_unit_refs_preserved",
    `catalog_route=${unitCatalogRouteStatus(route)}`,
    "conversion=false",
    `expression_refs=${refs.length ? refs.map((ref) => expressionUnitRefStatus(route, ref)).join(",") : "none"}`
  ].join("; ");
}

type ExpressionUnitRef = {
  ref: string;
  dimension: string;
  unit: string;
};

function collectExpressionUnitRefs(node: AstNode, path = "expression"): ExpressionUnitRef[] {
  const kind = nodeKind(node);
  const refs: ExpressionUnitRef[] = [];
  if (kind === "literal") {
    const quantity = quantityOf(node);
    refs.push({
      ref: `${path}.quantity`,
      dimension: asString(quantity.dimension) ?? "TBD",
      unit: asString(quantity.unit_ref) ?? "TBD"
    });
  }
  if (isTableNode(node)) {
    const table = tableOf(node);
    refs.push({
      ref: `${path}.table.argument`,
      dimension: asString(table.argument_dimension) ?? "TBD",
      unit: asString(table.argument_unit_ref) ?? "TBD"
    });
    refs.push({
      ref: `${path}.table.result`,
      dimension: asString(table.result_dimension) ?? "TBD",
      unit: asString(table.result_unit_ref) ?? "TBD"
    });
  }

  switch (kind) {
    case "unary":
      return [...refs, ...collectExpressionUnitRefs(childNode(node, "operand"), `${path}.operand`)];
    case "binary":
    case "compare":
    case "logical":
      return [
        ...refs,
        ...collectExpressionUnitRefs(childNode(node, "left"), `${path}.left`),
        ...collectExpressionUnitRefs(childNode(node, "right"), `${path}.right`)
      ];
    case "select":
      return [
        ...refs,
        ...collectExpressionUnitRefs(childNode(node, "condition"), `${path}.condition`),
        ...collectExpressionUnitRefs(childNode(node, "then"), `${path}.then`),
        ...collectExpressionUnitRefs(childNode(node, "else"), `${path}.else`)
      ];
    case "aggregate":
      return [
        ...refs,
        ...operandsOf(node).flatMap((operand, index) =>
          collectExpressionUnitRefs(operand, `${path}.operands.${index}`)
        )
      ];
    case "interpolate":
    case "lookup":
      return [...refs, ...collectExpressionUnitRefs(childNode(node, "argument"), `${path}.argument`)];
    default:
      return refs;
  }
}

function expressionUnitRefStatus(route: UnitCatalogRoute | null, ref: ExpressionUnitRef): string {
  const status = unitDimensionValidationStatus(route, ref.unit, ref.dimension);
  return `${ref.ref}=${status}(unit=${ref.unit};dimension=${ref.dimension})`;
}

function unitCatalogRouteStatus(route: UnitCatalogRoute | null): string {
  if (!route) return "loading";
  if (route.route === "unavailable_browser_preview") return "browser_preview_manual_entry";
  return "tauri_dec018_catalog";
}
