// DEL-04-11 REQ-008: the structural predicate for a batch that writes a support's hanger selection, and its
// classification pinned to the engine's own. The engine's reading of a support as a hanger
// (core/product_physics/src/lib.rs: `support_hanger_type`, `is_variable_spring_hanger`, `is_constant_effort_support`)
// is read here as source text and compared with the desktop's mirror, so that the two cannot drift silently. The table
// below runs invented support values through both the mirror and the predicate.
import { readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import type { OperationBatch } from "../../services/operationBatchService";
import {
  ENGINE_CONSTANT_EFFORT_SUPPORT_TYPES,
  ENGINE_VARIABLE_SPRING_HANGER_TYPES,
  engineHangerKind,
  SUPPORT_HANGER_FIELD_PATHS,
  writesSupportHangerSelection
} from "./hangerSelection";

const ENGINE_SOURCE = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "../../../../../core/product_physics/src/lib.rs");

/** The body of `fn <name>(` in the engine's source, up to its closing brace at the start of a line. */
function engineFunction(source: string, name: string): string {
  const start = source.indexOf(`fn ${name}(`);
  if (start < 0) throw new Error(`The engine function ${name} was not found in ${ENGINE_SOURCE}; the desktop's hanger classification cannot be checked against it.`);
  const end = source.indexOf("\n}\n", start);
  if (end < 0) throw new Error(`The end of the engine function ${name} was not found.`);
  return source.slice(start, end + 2);
}

/** The quoted names in the `matches!` arm of an engine function. */
function matchedNames(source: string, name: string): string[] {
  const body = engineFunction(source, name);
  const arm = /matches!\(\s*support_hanger_type\(support\),\s*Some\(([^)]*)\)\s*\)/.exec(body);
  if (!arm) throw new Error(`The engine function ${name} no longer matches support_hanger_type(support) against Some(<names>).`);
  return [...arm[1].matchAll(/"([^"]*)"/g)].map((match) => match[1]).sort();
}

describe("the desktop's hanger classification against the engine's", () => {
  const source = readFileSync(ENGINE_SOURCE, "utf8");

  it("names the same variable spring hanger types as is_variable_spring_hanger", () => {
    expect(matchedNames(source, "is_variable_spring_hanger"), "is_variable_spring_hanger").toEqual([...ENGINE_VARIABLE_SPRING_HANGER_TYPES].sort());
  });

  it("names the same constant-effort types as is_constant_effort_support", () => {
    expect(matchedNames(source, "is_constant_effort_support"), "is_constant_effort_support").toEqual([...ENGINE_CONSTANT_EFFORT_SUPPORT_TYPES].sort());
  });

  it("reads the effective type as support_hanger_type does: the trimmed, non-blank hanger_type, else the trimmed family", () => {
    const body = engineFunction(source, "support_hanger_type").replace(/\s+/g, "");
    const steps = [
      ".hanger.as_ref()",
      ".and_then(|hanger|hanger.hanger_type.as_deref())",
      ".map(str::trim)",
      ".filter(|value|!value.is_empty())",
      ".or_else(||support.family.as_deref().map(str::trim))"
    ];
    let from = 0;
    for (const step of steps) {
      const at = body.indexOf(step, from);
      expect(at, `support_hanger_type no longer has, in order, ${step}`).toBeGreaterThanOrEqual(0);
      from = at + step.length;
    }
  });
});

const configuration = (after: unknown, before: unknown = { family: "guide", restraints: ["UY"] }): OperationBatch => ({
  batch_id: "batch:table",
  operations: [{
    target: { object_type: "Support", ref: "support:S" },
    change: { field_path: "configuration", before: typeof before === "string" ? before : JSON.stringify(before), after: typeof after === "string" ? after : JSON.stringify(after) }
  }] as unknown as OperationBatch["operations"]
});

describe("the classification and the predicate on invented support values", () => {
  const table: { name: string; value: Record<string, unknown>; kind: ReturnType<typeof engineHangerKind>; writes: boolean }[] = [
    { name: "family spring_hanger with no hanger member", value: { family: "spring_hanger", restraints: ["UZ"] }, kind: "variable_spring", writes: true },
    { name: "hanger_type variable_spring_hanger with a trailing space", value: { family: "guide", hanger: { hanger_type: "variable_spring_hanger " } }, kind: "variable_spring", writes: true },
    { name: "a blank hanger_type with family constant_effort_support", value: { family: "constant_effort_support", hanger: { hanger_type: "  " } }, kind: "constant_effort", writes: true },
    { name: "hanger_type variable_spring_hanger with family guide", value: { family: "guide", hanger: { hanger_type: "variable_spring_hanger" } }, kind: "variable_spring", writes: true },
    { name: "family spring with a top-level stiffness and no hanger member (a plain spring)", value: { family: "spring", stiffness: { dof: "UZ", value: { value: 1, unit: "N/m" } } }, kind: null, writes: false },
    { name: "family guide", value: { family: "guide", restraints: ["UY"] }, kind: null, writes: false },
    { name: "a hanger member that is null", value: { family: "guide", hanger: null }, kind: null, writes: true },
    { name: "an empty hanger member", value: { family: "guide", hanger: {} }, kind: null, writes: true },
    { name: "family constant_effort_support padded with spaces", value: { family: " constant_effort_support " }, kind: "constant_effort", writes: true }
  ];
  for (const { name, value, kind, writes } of table) {
    it(`${name}: ${kind ?? "not a hanger"} to the engine; ${writes ? "writes" : "does not write"} a hanger selection`, () => {
      expect(engineHangerKind(value)).toBe(kind);
      expect(writesSupportHangerSelection(configuration(value))).toBe(writes);
    });
  }

  it("reads the value replaced as well as the value written, and an unreadable value fails closed", () => {
    // Removing a hanger: the before carries it and the after does not.
    expect(writesSupportHangerSelection(configuration({ family: "guide" }, { family: "variable_spring_hanger" }))).toBe(true);
    expect(writesSupportHangerSelection(configuration("{not json", { family: "guide" }))).toBe(true);
    expect(writesSupportHangerSelection(configuration({ family: "line_stop" }, { family: "guide" }))).toBe(false);
  });

  it("reads only Support operations on the listed field paths", () => {
    expect(SUPPORT_HANGER_FIELD_PATHS).toEqual(["configuration", "supports"]);
    const on = (objectType: string, fieldPath: string, after: unknown): OperationBatch => ({
      batch_id: "batch:paths",
      operations: [{ target: { object_type: objectType, ref: "x" }, change: { field_path: fieldPath, before: "not_present", after: JSON.stringify(after) } }] as unknown as OperationBatch["operations"]
    });
    expect(writesSupportHangerSelection(on("Support", "supports", { family: "spring_hanger" }))).toBe(true);
    expect(writesSupportHangerSelection(on("Support", "label", { family: "spring_hanger" }))).toBe(false);
    expect(writesSupportHangerSelection(on("Component", "configuration", { hanger: {} }))).toBe(false);
  });
});
