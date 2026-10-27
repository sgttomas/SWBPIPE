// Model document 0.8.0's `model_settings.shear_model` on the desktop (DEL-02-01 Design
// model-document-0.8.0.md §1, §4): the type admits it, the intents reach the operation
// engine, which writes `euler_bernoulli` and refuses by name, and the document validator
// (the wasm export open and load use) admits both names and refuses the rest. Invented data.
import { readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { beforeAll, expect, it } from "vitest";
import type { PreviewModel } from "../../types";
import { canonicalJsonString, computeModelHash } from "../../services/hashService";
import { applyModelOperation, validateModelOperation, warmupOperationEngine } from "../../services/operationService";
import { modelDocumentSlotFindings } from "../model-file/modelDocumentShape";
import { clearShearModelIntent, setShearModelIntent } from "./modelSettingsIntents";

const projectRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "../../../../..");
const demo = () => JSON.parse(readFileSync(path.join(projectRoot, "fixtures/product_preview/invented_demo_model.json"), "utf8")) as PreviewModel;
const blocking = (codes: { code: string; severity: string }[]) => codes.filter((d) => d.severity === "blocking").map((d) => d.code);

beforeAll(async () => {
  expect((await warmupOperationEngine()).state).toBe("ready");
});

it("sets euler_bernoulli and clears it back to the absent form", async () => {
  const model = demo();
  expect(model.model_settings?.shear_model).toBeUndefined();
  const set = await setShearModelIntent(canonicalJsonString, model, "euler_bernoulli");
  const applied = await applyModelOperation(model, set, await computeModelHash(model));
  expect(blocking(applied.diagnostics)).toEqual([]);
  const next = applied.applied_model as PreviewModel;
  expect(next.model_settings?.shear_model).toBe("euler_bernoulli");
  const cleared = await applyModelOperation(next, await clearShearModelIntent(canonicalJsonString, next), await computeModelHash(next));
  expect(await canonicalJsonString(cleared.applied_model)).toBe(await canonicalJsonString(model));
});

it("refuses timoshenko, a pressure-contract document and an unknown name by name", async () => {
  const model = demo();
  const outcome = async (document: PreviewModel, name: string) =>
    blocking((await validateModelOperation(document, await setShearModelIntent(canonicalJsonString, document, name), await computeModelHash(document))).diagnostics);
  expect(await outcome(model, "timoshenko_annulus_energy_v1")).toEqual(["MODEL-SETTING-SHEAR-MODEL-TIMOSHENKO-UNSUPPORTED"]);
  expect(await outcome(model, "timoshenko")).toEqual(["MODEL-SETTING-SHEAR-MODEL-UNKNOWN"]);
  const exact = { ...model, pressure_contract: { version: "3.0.0", mode: "exact_pressure_v3", shear_model: "euler_bernoulli" } } as PreviewModel;
  expect(await outcome(exact, "euler_bernoulli")).toEqual(["MODEL-SETTING-SHEAR-MODEL-WITH-PRESSURE-CONTRACT"]);
});

it("the document validator admits both names and refuses null and other values", () => {
  for (const name of ["euler_bernoulli", "timoshenko_annulus_energy_v1"]) {
    expect(blocking(modelDocumentSlotFindings({ ...demo(), model_settings: { shear_model: name } }))).toEqual([]);
  }
  for (const value of [null, "Euler_Bernoulli", 1]) {
    expect(blocking(modelDocumentSlotFindings({ ...demo(), model_settings: { shear_model: value } }))).toEqual(["MODEL-SETTING-SHEAR-MODEL-UNKNOWN"]);
  }
});
