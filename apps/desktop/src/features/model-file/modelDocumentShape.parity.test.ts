// The webview half of the shared shape corpus (DEL-15-01 W-6a repair 1): the
// native save runs a port of `modelDocumentShapeDiagnostics`
// (`src-tauri/src/model_document_shape.rs`), whose test reads the same invented
// cases. Findings compare as sets.
import { describe, expect, it } from "vitest";
import { SHAPE_CASES, shapeCaseDocument, type ShapeFinding } from "../../test/shapeCorpus";
import { modelDocumentShapeDiagnostics } from "./modelDocumentShape";

const sorted = (items: ShapeFinding[]) => items.map(({ code, path, message }) => [code, path, message].join("\u0000")).sort();

describe("desktop shape corpus shared with the native save", () => {
  it("has the invented cases", () => {
    expect(SHAPE_CASES.length).toBeGreaterThanOrEqual(10);
  });

  it.each(SHAPE_CASES.map((item) => [item.case_id, item] as const))("%s", (_id, item) => {
    expect(sorted(modelDocumentShapeDiagnostics(shapeCaseDocument(item)))).toEqual(sorted(item.expected));
  });
});
