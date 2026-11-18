// The reference system (DEL-09-01, `fixtures/reference_system/SPECIFICATION.md` §2) opens in the
// desktop as the generator writes it: the committed file passes the desktop's own shape check and
// the check File > Open Model Document applies (`checkNativeOutcome`: the shape check, then the wasm
// document validator) with no refusal. Without its data boundary and diagnostics list it is refused
// by name, which is what the generator once wrote.
import { readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import { checkNativeOutcome } from "../../services/modelDocumentFile";
import { MODEL_FILE_SHAPE, modelDocumentShapeDiagnostics } from "./modelDocumentShape";

const fixture = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "../../../../../fixtures/reference_system/reference_system.json");
const text = readFileSync(fixture, "utf8");

describe("the reference system opens in the desktop", () => {
  it("passes the desktop's shape check as committed", () => {
    expect(modelDocumentShapeDiagnostics(JSON.parse(text))).toEqual([]);
  });

  it("passes the open path's checks as committed", () => {
    const outcome = checkNativeOutcome({ outcome: "opened", file_name: "reference_system.json", byte_count: text.length, schema_status: "current", document: JSON.parse(text) });
    expect(outcome.outcome === "refused" ? outcome.diagnostics.slice(0, 5) : []).toEqual([]);
    expect(outcome.outcome).toBe("opened");
  });

  it("states its data boundary and carries no authored diagnostic", () => {
    const document = JSON.parse(text) as { data_boundary: Record<string, string>; diagnostics: unknown[] };
    expect(Object.keys(document.data_boundary).sort()).toEqual(["private_data_policy", "professional_boundary", "protected_source_policy", "public_examples_policy"]);
    expect(document.data_boundary.public_examples_policy).not.toContain("blank_user_created_local_document");
    expect(document.diagnostics).toEqual([]);
  });

  it("is refused by name without its data boundary and diagnostics list", () => {
    const { data_boundary: _boundary, diagnostics: _diagnostics, ...without } = JSON.parse(text) as Record<string, unknown>;
    expect(modelDocumentShapeDiagnostics(without).map((item) => [item.code, item.path])).toEqual([
      [MODEL_FILE_SHAPE, "data_boundary"],
      [MODEL_FILE_SHAPE, "diagnostics"]
    ]);
  });
});
