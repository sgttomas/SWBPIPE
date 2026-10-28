// The TS version check gives every state in the shared table
// (`fixtures/model_document/version_states.json`), as the applier, the native
// desktop and Python persistence do (DEL-02-05 CLM-003; DEL-02-01 Design
// `model-document-0.8.0.md` §3.1).
import { describe, expect, it } from "vitest";
import table from "../../../../fixtures/model_document/version_states.json";
import type { PreviewModel } from "../types";
import { evaluateModelDocumentLocal } from "./projectService";

describe("model-document version states", () => {
  it("match the shared table", () => {
    expect(table.cases.length).toBeGreaterThanOrEqual(10);
    for (const entry of table.cases as Array<Record<string, unknown>>) {
      const model = { document_kind: "openpipestress.product_preview.model" } as Record<string, unknown>;
      if ("schema_version" in entry) model.schema_version = entry.schema_version;
      const evaluated = evaluateModelDocumentLocal(model as unknown as PreviewModel);
      expect(evaluated, String(entry.label)).toEqual({
        status: entry.status,
        source_schema_version: entry.source_schema_version,
        target_schema_version: entry.target_schema_version,
        detail: entry.detail
      });
    }
  });
});
