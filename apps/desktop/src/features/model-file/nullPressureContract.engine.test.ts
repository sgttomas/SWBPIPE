// Agent 0 (2026-11-05, S-K1 applier slice): the webview's document-open check is the
// applier's one shared validator through the wasm engine, so a present
// `pressure_contract: null` is refused with the same code, path and message the Rust
// validator gives (`operation_applier/tests/null_pressure_contract.rs`); absence and a
// present object are not. Native open (`src-tauri/src/model_document_file.rs`) calls the
// same `validate_model_document`.
import { beforeAll, describe, expect, it } from "vitest";
import modelFixture from "../../../../../fixtures/model_operations/exact_pressure_authoring_model.json";
import { loadWasmEngine } from "../../services/wasmEngine/loadWasmEngine";
import { modelDocumentSlotFindings } from "./modelDocumentShape";

const MESSAGE = "pressure_contract is null; an authored object is required, and absence means the pressure-free route.";

const model = (): Record<string, unknown> => structuredClone(modelFixture) as Record<string, unknown>;
const contractFindings = (document: unknown) => modelDocumentSlotFindings(document).filter((item) => item.path === "pressure_contract");

beforeAll(async () => {
  await loadWasmEngine();
});

describe("a present null pressure_contract on webview document open", () => {
  it("is refused by name, as the Rust validator refuses it", () => {
    const document = model();
    document.pressure_contract = null;
    expect(contractFindings(document).map(({ code, severity, path, message, origin }) => ({ code, severity, path, message, origin })))
      .toEqual([{ code: "MODEL-SLOT-SHAPE", severity: "blocking", path: "pressure_contract", message: MESSAGE, origin: "validator" }]);
  });

  it("is not absence, and a present object or absence gives no such finding", () => {
    const absent = model();
    delete absent.pressure_contract;
    expect(contractFindings(absent)).toEqual([]);
    expect(contractFindings(model())).toEqual([]);
    const retired = model();
    retired.pressure_contract = { version: "2.0.0", mode: "exact_straight_pressure_v2" };
    expect(contractFindings(retired).some((item) => item.message === MESSAGE)).toBe(false);
  });
});
