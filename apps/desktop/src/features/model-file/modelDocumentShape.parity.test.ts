// The webview half of the shared shape corpus (DEL-15-01 W-6a repair 1): the
// native save runs a port of `modelDocumentShapeDiagnostics`
// (`src-tauri/src/model_document_shape.rs`), whose test reads the same invented
// cases. Findings compare as sets.
import { describe, expect, it } from "vitest";
import corpus from "../../../../../fixtures/model_document/desktop_shape_cases.json";
import { modelDocumentShapeDiagnostics } from "./modelDocumentShape";

type Finding = { code: string; path: string; message: string };
const sorted = (items: Finding[]) => items.map(({ code, path, message }) => [code, path, message].join("\u0000")).sort();

describe("desktop shape corpus shared with the native save", () => {
  it("has the invented cases", () => {
    expect(corpus.cases.length).toBeGreaterThanOrEqual(10);
  });

  it.each(corpus.cases.map((item) => [item.case_id, item] as const))("%s", (_id, item) => {
    expect(sorted(modelDocumentShapeDiagnostics(item.document))).toEqual(sorted(item.expected as Finding[]));
  });
});
