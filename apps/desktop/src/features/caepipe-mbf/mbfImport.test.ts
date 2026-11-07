// DEL-17-04 slice 5 (Design `desktop-mbf-export-import.md` §4.3): the import review's product presentation. Every
// code the importer and the native commands can emit has a product description, every structured assumption the
// engine records has one formed from its parameters (conditional branches included), an unknown or cut assumption
// blocks Adopt, the declaration never carries the engine's `*`, and no product source reaches the wasm importer.
import { readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";
import { describe, expect, it } from "vitest";
import importReplies from "../../../../../fixtures/caepipe_mbf/invented/desktop_import/import_replies.json";
import {
  MBF_IMPORT_ASSUMPTIONS,
  MBF_IMPORT_DESCRIPTIONS,
  MBF_IMPORT_ASSUMPTION_UNAVAILABLE,
  MBF_IMPORT_MODEL_TOO_LARGE,
  MBF_IMPORT_REVIEW_TRUNCATED,
  adoptCapture,
  assumptionText,
  declarationFor,
  importAdoptable,
  importBlockingCodes,
  type MbfImportReviewOutcome,
  type MbfImportReviewPayload
} from "./mbfImport";

const ROOT = join(__dirname, "../../../../..");
const ENGINE = join(ROOT, "core/handoff/caepipe_mbf_engine/src");
const read = (path: string) => readFileSync(path, "utf8");
const reviewed = (name: string) => structuredClone((importReplies.replies as Record<string, unknown>)[name]) as Extract<MbfImportReviewOutcome, { outcome: "reviewed" }>;

describe("the import review's product presentation", () => {
  it("describes every code the importer and the native commands emit", () => {
    const sources = [...readdirSync(join(ENGINE, "importer")).map((name) => join(ENGINE, "importer", name)), join(ENGINE, "sidecar.rs")];
    const engineCodes = new Set(sources.flatMap((path) => [...read(path).matchAll(/"(MBF-IMPORT-[A-Z0-9-]+)"/g)].map((match) => match[1])));
    const native = read(join(ROOT, "apps/desktop/src-tauri/src/mbf_import.rs"));
    const nativeCodes = new Set([...native.matchAll(/const [A-Z_]+: &str = "((?:MBF-IMPORT|MBF-OPEN)-[A-Z0-9-]+)"/g)].map((match) => match[1]));
    expect(engineCodes.size).toBeGreaterThan(80);
    expect(nativeCodes.size).toBeGreaterThan(15);
    const missing = [...engineCodes, ...nativeCodes].filter((code) => !Object.hasOwn(MBF_IMPORT_DESCRIPTIONS, code));
    expect(missing).toEqual([]);
    // Product text states SWBPIPE's behaviour, never facts about another program.
    for (const text of Object.values(MBF_IMPORT_DESCRIPTIONS)) expect(text).not.toMatch(/CAEPIPE|technical reference/i);
  });

  it("describes every structured assumption the engine records, from its parameters", () => {
    const ids = [...read(join(ENGINE, "importer/mod.rs")).split("pub(crate) const ASSUMPTION_IDS")[1].split("];")[0].matchAll(/"([a-z_0-9]+)"/g)].map((match) => match[1]);
    expect(ids).toHaveLength(16);
    expect(Object.keys(MBF_IMPORT_ASSUMPTIONS).sort()).toEqual([...ids].sort());
    const text = (id: string, parameters: Record<string, string>) => assumptionText({ id, parameters });
    expect(text("units_and_axes", { unit_mode: "English", mbf_version: "15.10", angles: "deg", rotation: "z_to_y_exact", vertical_axis: "Z", profile_id: "p" }))
      .toContain("(X, Y, Z) to (X, Z, -Y)");
    expect(text("units_and_axes", { unit_mode: "SI", mbf_version: "15.10", angles: "rad", rotation: "none", vertical_axis: "Y", profile_id: "p" })).toContain("Y is vertical");
    expect(text("bare_metal_weight", { density_unit: "kgf/m3", weight_unit: "N/m", standard_gravity: "9.80665" })).toContain("9.80665");
    expect(text("bare_metal_weight", { density_unit: "lbf/in3", weight_unit: "lbf/in", standard_gravity: "none" })).not.toContain("gravity");
    expect(text("thermal_difference_zero_fields", { thermal_change: "T1-T_ref", corrosion_allowance_under_pressure: "true" })).toContain("except the corrosion allowance");
    expect(text("pressure_regions", { unit: "bar", region_count: "2", contract: "3.0.0/exact_pressure_v3", shear_model: "s", regions: "restored", closure: "sidecar_restored", pressure_case: "sustained", corrosion_allowance: "zero_stated" }))
      .toContain("restored from the identity file");
    const formed = text("pressure_regions", { unit: "bar", region_count: "1", contract: "3.0.0/exact_pressure_v3", shear_model: "s", regions: "formed", closure: "caller_declared_corner_wall", corner_override: "explicit_node_declaration", pressure_case: "sustained", corrosion_allowance: "zero_stated" });
    expect(formed).toContain("Closure at each real end is declared by the caller");
    expect(formed).toContain("unless an explicit node closure declaration overrides it");
    const guide = text("support_guide", { stiffness_unit: "N/mm", element_selection: "preceding_else_following" });
    expect(guide).toContain("preceding element, falling back to the following element");
    expect(guide).toContain("axis-aligned element");
    expect(text("binding_convention", { basis: "profile_decision_pd1", convention: "load_reference", effect: "carried_until_changed", mbf_version: "15.10" })).toContain("still open");
    for (const description of Object.values(MBF_IMPORT_ASSUMPTIONS)) expect(String(description)).not.toMatch(/CAEPIPE/);
  });

  it("adopts only a complete, adoptable review whose every assumption it can show", () => {
    const bound = reviewed("bound-open");
    expect(importAdoptable(bound.review)).toBe(true);
    expect(adoptCapture(bound, null)).toMatchObject({ sourceId: bound.sourceId, reviewSha256: bound.reviewSha256, modelSha256: bound.review.model_sha256, binding: "bound" });
    const unknown: MbfImportReviewPayload = { ...bound.review, assumptions: [...bound.review.assumptions, { id: "invented_new_assumption", parameters: {} }] };
    expect(importBlockingCodes(unknown)).toEqual([MBF_IMPORT_ASSUMPTION_UNAVAILABLE]);
    const cut: MbfImportReviewPayload = { ...bound.review, assumptions: [{ id: "spring_stiffness_unit", parameters: { unit: { prefix: "N", bytes: 600, sha256: "0".repeat(64) } } }] };
    expect(importAdoptable(cut)).toBe(false);
    const truncated: MbfImportReviewPayload = { ...bound.review, completeness: "truncated" };
    expect(importBlockingCodes(truncated)).toEqual([MBF_IMPORT_REVIEW_TRUNCATED]);
    const large: MbfImportReviewPayload = { ...bound.review, transfer_limit: "beyond", adoptable: false };
    expect(importBlockingCodes(large)).toEqual([MBF_IMPORT_MODEL_TOO_LARGE]);
    expect(adoptCapture({ ...bound, review: large }, null)).toBeNull();
    expect(importAdoptable(reviewed("unbound-open").review)).toBe(false);
    // The capture carries bounded identities only.
    expect(Object.keys(adoptCapture(bound, null)!).sort()).toEqual(["binding", "declaration", "fileName", "formedFromNames", "modelSha256", "restored", "reviewSha256", "sourceId"]);
  });

  it("sends one entry per declared material and never the engine's wildcard", () => {
    expect(declarationFor([["M1", { origin: "project_invented", source_name: "x" }], ["*", { origin: "user_supplied", source_name: "y" }], ["M2", null]]))
      .toEqual({ M1: { origin: "project_invented", source_name: "x" } });
    expect(declarationFor([["M1", { origin: "user_supplied", source_name: "  " }]])).toBeNull();
  });

  it("keeps prototype-shaped material names as own declaration entries through JSON transport", () => {
    const entry = { origin: "user_supplied" as const, source_name: "my invented source" };
    const declaration = declarationFor([["__proto__", entry], ["constructor", entry], ["toString", entry]])!;
    expect(Object.getPrototypeOf(declaration)).toBeNull();
    const transported = JSON.parse(JSON.stringify(declaration));
    for (const name of ["__proto__", "constructor", "toString"]) {
      expect(Object.hasOwn(transported, name)).toBe(true);
      expect(transported[name]).toEqual(entry);
    }
  });

  it("no product source imports the wasm MBF importer", () => {
    const files: string[] = [];
    const walk = (dir: string) => {
      for (const entry of readdirSync(dir, { withFileTypes: true })) {
        const path = join(dir, entry.name);
        if (entry.isDirectory()) walk(path);
        else if (/\.(ts|tsx)$/.test(entry.name) && !/\.test\.(ts|tsx)$/.test(entry.name) && !path.includes("/test/")) files.push(path);
      }
    };
    walk(join(ROOT, "apps/desktop/src"));
    // `services/mbfEngine.ts` is the wasm engine's unconnected test resource (§4.8.1 (a)); nothing in the product may
    // load it or call its importer.
    const uses = (text: string) => /from\s+["'][^"']*\/mbfEngine["']|importMbfJson|import_mbf_json/.test(text);
    const offending = files.filter((path) => !path.endsWith("services/mbfEngine.ts") && uses(read(path)));
    expect(offending).toEqual([]);
    // Negative controls: the scan sees an import of the resource and a call of its importer.
    expect(uses('import { loadMbfEngine } from "../../services/mbfEngine";')).toBe(true);
    expect(uses("engine.importMbfJson(bytes, null, options)")).toBe(true);
  });
});
