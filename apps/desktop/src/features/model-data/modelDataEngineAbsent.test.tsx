// W-13 §5.3 (S2 review): without the wasm engine that holds the document
// validator, the browser-preview store refuses to store or open a document by
// name (it never adopts an unchecked document), and the diagnostics list says
// the model-data slots were not checked. Invented data only.
import { readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it, vi } from "vitest";
import type { PreviewModel } from "../../types";

const engine = vi.hoisted(() => ({ absent: false }));
vi.mock("../../services/wasmEngine/loadWasmEngine", async (importOriginal) => {
  const actual = await importOriginal<typeof import("../../services/wasmEngine/loadWasmEngine")>();
  return {
    ...actual,
    loadedWasmEngine: () => (engine.absent ? null : actual.loadedWasmEngine()),
    loadWasmEngine: () => (engine.absent ? Promise.reject(new Error("absent for this test")) : actual.loadWasmEngine())
  };
});

import { openLocalProject, saveLocalProject } from "../../services/projectService";
import { modelDocumentDiagnostics } from "./documentDiagnostics";

const projectRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "../../../../..");
const everySlot = () => JSON.parse(readFileSync(path.join(projectRoot, "fixtures/model_document/invented_every_slot_model.json"), "utf8")) as PreviewModel;

describe("browser-preview store and diagnostics without the validator engine", () => {
  it("refuses save and open by name and lists the document as unchecked", async () => {
    const model = everySlot();
    model.project.id = "project:engine-absent";
    engine.absent = false;
    await saveLocalProject(model);

    engine.absent = true;
    await expect(saveLocalProject({ ...model, project: { ...model.project, id: "project:engine-absent-2" } }))
      .rejects.toThrow(/WASM-ENGINE-ASSET-ABSENT/);
    await expect(openLocalProject("project:engine-absent")).rejects.toThrow(/WASM-ENGINE-ASSET-ABSENT/);
    const diagnostics = modelDocumentDiagnostics(model);
    expect(diagnostics.map((item) => [item.code, item.severity])).toEqual([["WASM-ENGINE-ASSET-ABSENT", "warning"]]);
    expect(diagnostics[0].message).toMatch(/^The model-data slots were not checked/);

    engine.absent = false;
    expect((await openLocalProject("project:engine-absent"))?.model).toEqual(model);
    expect(modelDocumentDiagnostics(model)).toEqual([]);
  });
});
