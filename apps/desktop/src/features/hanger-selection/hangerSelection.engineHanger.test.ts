// DEL-04-11 REQ-008: the structural predicate for a batch that writes a support's hanger selection, and its
// classification checked against the engine's own.
// - The behavioural check is the shared case table, fixtures/results/hanger_sizing_engine_kind_corpus.json: each case's
//   "engine" (how the engine classifies the value) and "gate" (whether a write of it is a hanger write). Only its
//   desktop half is exercised here until a test on the engine's side reads the same table.
// - The source pin reads core/product_physics/src/lib.rs as text. As review round 2 found, it catches a changed name or a
//   removed step in `support_hanger_type`, `is_variable_spring_hanger` and `is_constant_effort_support`, but not a
//   filter added after the fall back; the table is what checks behaviour.
import { readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import type { OperationBatch } from "../../services/operationBatchService";
import corpus from "../../../../../fixtures/results/hanger_sizing_engine_kind_corpus.json";
import {
  ENGINE_CONSTANT_EFFORT_SUPPORT_TYPES,
  ENGINE_VARIABLE_SPRING_HANGER_TYPES,
  engineHangerKind,
  RUST_WHITE_SPACE_CODE_POINTS,
  rustTrim,
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

type CorpusCase = { name: string; support: Record<string, unknown>; engine: "variable_spring" | "constant_effort" | "none" | "document_refused"; gate: boolean };
const CASES = (corpus as { cases: CorpusCase[] }).cases;

/** A created support of `value`, as the operation route takes it. */
const created = (value: Record<string, unknown>): OperationBatch => ({
  batch_id: "batch:create",
  operations: [{ operation_kind: "create", target: { object_type: "Support", ref: String(value.id) },
    change: { change_kind: "create_support", field_path: "supports", before: "not_present", after: JSON.stringify(value) } }] as unknown as OperationBatch["operations"]
});

describe("the shared engine-kind case table", () => {
  it("covers every outcome and holds the invariant: the gate never admits what the engine would treat as a hanger", () => {
    expect(new Set(CASES.map((entry) => entry.engine))).toEqual(new Set(["variable_spring", "constant_effort", "none", "document_refused"]));
    for (const entry of CASES) {
      if (entry.engine !== "none") expect(entry.gate, entry.name).toBe(true);
    }
  });

  for (const entry of CASES) {
    it(`${entry.name}: engine ${entry.engine}, gate ${entry.gate}`, () => {
      expect(engineHangerKind(entry.support) ?? "none").toBe(entry.engine);
      const before = { id: entry.support.id, node: "node:B", restraints: ["UZ"], family: "guide" };
      expect(writesSupportHangerSelection(configuration(entry.support, before)), "configuration").toBe(entry.gate);
      expect(writesSupportHangerSelection(created(entry.support)), "create").toBe(entry.gate);
    });
  }
});

describe("Rust's trim", () => {
  it("removes exactly the White_Space code points, unlike JavaScript's trim", () => {
    for (const point of RUST_WHITE_SPACE_CODE_POINTS) expect(rustTrim(`${String.fromCodePoint(point)}x${String.fromCodePoint(point)}`)).toBe("x");
    expect(rustTrim("\u0085x")).toBe("x");
    expect("\u0085x".trim()).toBe("\u0085x");
    expect(rustTrim("\ufeffx")).toBe("\ufeffx");
    expect("\ufeffx".trim()).toBe("x");
    expect(rustTrim(" \t\n ")).toBe("");
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

  it("reads a support deletion by the members the operation route dispatches on, and admits it: it writes no hanger", () => {
    const deletion = (changeKind: string, operationKind: string, after = "not_present"): OperationBatch => ({
      batch_id: "batch:delete",
      operations: [{ operation_kind: operationKind, target: { object_type: "Support", ref: "support:B" },
        change: { change_kind: changeKind, field_path: "supports", before: "Hanger B", after } }] as unknown as OperationBatch["operations"]
    });
    // The before is the support's label, not JSON; a deletion is still no hanger write.
    expect(writesSupportHangerSelection(deletion("delete_support", "delete"))).toBe(false);
    // Not a deletion by the route's own members: read as a possible write, and the label fails closed.
    expect(writesSupportHangerSelection(deletion("create_support", "delete"))).toBe(true);
    expect(writesSupportHangerSelection(deletion("delete_support", "create"))).toBe(true);
    expect(writesSupportHangerSelection(deletion("delete_support", "delete", JSON.stringify({ family: "spring_hanger" })))).toBe(true);
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
