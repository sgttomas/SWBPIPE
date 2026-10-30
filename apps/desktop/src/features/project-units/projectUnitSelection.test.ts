import { describe, expect, it } from "vitest";
import {
  PROJECT_UNIT_KEYS,
  SI_PROJECT_UNITS,
  US_CUSTOMARY_PROJECT_UNITS,
  buildBlankLoadStateModelDocument,
  buildBlankLocalModelDocument,
  type ProjectUnitMap
} from "../../services/projectService";
import { convertDisplayQuantities } from "../../services/displayQuantityService";
import { canonicalSha256Hex } from "../../services/hashService";
import type { UnitCatalogRoute } from "../../services/unitCatalogService";
import { CATALOG_FIXTURE_ROWS, unitCatalogFixture } from "../../test-support/unitCatalogFixture";
import { primitiveLoadDefaultUnit, primitiveLoadDefaultUnitChoice } from "../load-cases/primitiveLoadVocabulary";
import type { PreviewModel } from "../../types";
import {
  PROJECT_UNIT_DIMENSIONS,
  PROJECT_UNIT_PRESETS,
  canonicalCatalogEntry,
  checkComposedDefault,
  readProjectUnitKey,
  composedDefaultSymbol,
  composedDefaultsFor,
  fieldComposedDefault,
  linearStiffnessFieldDefault,
  projectTemperatureDefault,
  projectUnitPresetFor,
  projectUnitSelectionProblems,
  rotationalStiffnessFieldDefault
} from "./projectUnitSelection";

// W-17 slice (a), DEL-02-02 Design `project-unit-selection.md` §2.1-2.4 and §4.
const AT = new Date("2026-06-12T00:00:00Z");
const CATALOG: UnitCatalogRoute = { route: "tauri_unit_catalog", catalog: unitCatalogFixture() };
const PREVIEW: UnitCatalogRoute = { route: "unavailable_browser_preview", diagnostic: "UNIT-CATALOG-DESKTOP-ONLY: fixture" };
// sha256 (RFC 8785) of the SI blank before W-17 (`origin/main` f0b6ae40a0), when it had six keys and no angle,
// at model 0.8.0 (W-13 S1b and W-15 slice c: the version token is the only change).
const SIX_KEY_BLANK_SHA256 = "ed3a25c9c76a62b483a8d264644ad3c3f284d232bf753fbec78b5e56424841e0";

function withUnits(units: Record<string, string>): PreviewModel {
  const model = buildBlankLocalModelDocument(AT);
  model.project.units = units;
  return model;
}

describe("blank builders take a project unit map", () => {
  it("default to SI: today's six-key set plus angle rad, and nothing else changes", async () => {
    const blank = buildBlankLocalModelDocument(AT);
    expect(blank.project.units).toEqual({ length: "m", force: "N", moment: "N*m", pressure: "Pa", stress: "Pa", temperature: "degC", angle: "rad" });
    expect(buildBlankLocalModelDocument(AT, SI_PROJECT_UNITS)).toEqual(blank);
    const withoutAngle = structuredClone(blank);
    delete withoutAngle.project.units.angle;
    expect(await canonicalSha256Hex(withoutAngle)).toBe(SIX_KEY_BLANK_SHA256);
  });

  it("give exactly the seven keys, each a canonical catalog symbol for its dimension, for each preset", () => {
    for (const preset of PROJECT_UNIT_PRESETS) {
      for (const build of [buildBlankLocalModelDocument, buildBlankLoadStateModelDocument]) {
        const units = build(AT, preset.units).project.units;
        expect(Object.keys(units).sort()).toEqual([...PROJECT_UNIT_KEYS].sort());
        for (const key of PROJECT_UNIT_KEYS) {
          expect(canonicalCatalogEntry(CATALOG.catalog, units[key], PROJECT_UNIT_DIMENSIONS[key]), `${preset.id} ${key}=${units[key]}`).not.toBeNull();
        }
        expect(projectUnitSelectionProblems(CATALOG, units)).toEqual([]);
        expect(projectUnitSelectionProblems(PREVIEW, units)).toEqual([]);
      }
    }
    expect(US_CUSTOMARY_PROJECT_UNITS).toEqual({ length: "in", force: "lbf", moment: "lbf*in", pressure: "psi", stress: "psi", temperature: "degF", angle: "deg" });
  });

  it("write a custom map as given and refuse an incomplete one", () => {
    const custom: ProjectUnitMap = { ...SI_PROJECT_UNITS, length: "mm", pressure: "MPa", stress: "MPa" };
    expect(buildBlankLoadStateModelDocument(AT, custom).project.units).toEqual(custom);
    const incomplete = { ...SI_PROJECT_UNITS, angle: "" };
    expect(() => buildBlankLocalModelDocument(AT, incomplete)).toThrow("PROJECT-UNITS-INCOMPLETE: a new project needs project.units.angle");
  });

  it("name the preset a map matches, else Custom", () => {
    expect(projectUnitPresetFor(SI_PROJECT_UNITS)).toBe("si");
    expect(projectUnitPresetFor(US_CUSTOMARY_PROJECT_UNITS)).toBe("us_customary");
    expect(projectUnitPresetFor({ ...SI_PROJECT_UNITS, length: "mm" })).toBe("custom");
  });
});

describe("canonical symbols only (§2.2)", () => {
  it("refuses the legacy C, aliases, other spellings, unknown symbols and dimension mismatches", () => {
    const cases: Array<[keyof ProjectUnitMap, string]> = [
      ["temperature", "C"], ["temperature", "degc"], ["temperature", "°C"], ["temperature", " degC"],
      ["moment", "N.m"], ["moment", "Nm"], ["moment", "N*mm"], ["length", "metre"], ["length", "N"],
      ["angle", "degree"], ["pressure", "kip"], ["stress", "lbf"], ["force", ""]
    ];
    for (const [key, symbol] of cases) {
      const problems = projectUnitSelectionProblems(CATALOG, { ...SI_PROJECT_UNITS, [key]: symbol });
      expect(problems.map((problem) => problem.key), `${key}=${JSON.stringify(symbol)}`).toEqual([key]);
    }
    expect(projectUnitSelectionProblems(CATALOG, { ...SI_PROJECT_UNITS, temperature: "C" })[0].message)
      .toBe("C is a legacy spelling; a new project uses the catalog symbol degC.");
    // Stress reads the pressure entries; any accepted symbol of the key's dimension is a valid custom choice.
    expect(projectUnitSelectionProblems(CATALOG, { ...SI_PROJECT_UNITS, length: "mm", stress: "MPa", temperature: "K", angle: "deg" })).toEqual([]);
  });

  it("without the catalog, accepts only preset symbols", () => {
    expect(projectUnitSelectionProblems(PREVIEW, { ...SI_PROJECT_UNITS, force: "lbf" })).toEqual([]);
    expect(projectUnitSelectionProblems(null, { ...SI_PROJECT_UNITS, length: "mm" }).map((problem) => problem.key)).toEqual(["length"]);
    expect(projectUnitSelectionProblems(PREVIEW, { ...SI_PROJECT_UNITS, temperature: "C" }).map((problem) => problem.key)).toEqual(["temperature"]);
  });
});

describe("composed defaults (§2.4)", () => {
  it("mm with N: the distributed default is N/mm, and the moment default is the moment key, never N*mm", () => {
    const model = withUnits({ ...SI_PROJECT_UNITS, length: "mm" });
    expect(primitiveLoadDefaultUnit(model, "distributed_force", "global_y", CATALOG)).toBe("N/mm");
    expect(primitiveLoadDefaultUnit(model, "concentrated_moment", "rotation_z", CATALOG)).toBe("N*m");
    expect(primitiveLoadDefaultUnit(model, "concentrated_moment", "rotation_z")).toBe("N*m");
  });

  it("moment: a declared key wins over the composition; without it, <force>*<length> is the checked fallback", () => {
    // Key present and different from the composition (kN*mm): the key wins, with or without a catalog.
    const keyed = withUnits({ length: "mm", force: "kN", moment: "lbf*ft" });
    for (const catalog of [CATALOG, PREVIEW, null]) {
      expect(primitiveLoadDefaultUnitChoice(keyed, "concentrated_moment", "rotation_z", catalog)).toEqual({ symbol: "lbf*ft", notice: null });
    }
    // Key absent, the composition is a catalog unit: it is the default (the bundled sample's case, m with N).
    expect(primitiveLoadDefaultUnitChoice(withUnits({ length: "m", force: "N" }), "concentrated_moment", "rotation_z", CATALOG)).toEqual({ symbol: "N*m", notice: null });
    expect(primitiveLoadDefaultUnit(withUnits({ length: "in", force: "lbf" }), "concentrated_moment", "rotation_z", CATALOG)).toBe("lbf*in");
    // Key absent, the composition is not a catalog unit: no default, the notice, no substitute.
    expect(primitiveLoadDefaultUnitChoice(withUnits({ length: "mm", force: "N" }), "concentrated_moment", "rotation_z", CATALOG))
      .toEqual({ symbol: "", notice: "N*mm is not a catalog unit; choose one" });
    // Without a catalog the fallback is unchecked, like the other compositions.
    expect(primitiveLoadDefaultUnitChoice(withUnits({ length: "mm", force: "N" }), "concentrated_moment", "rotation_z", PREVIEW)).toEqual({ symbol: "N*mm", notice: null });
    // Without force or length there is nothing to compose.
    expect(primitiveLoadDefaultUnit(withUnits({ length: "mm" }), "concentrated_moment", "rotation_z", CATALOG)).toBe("TBD");
  });

  it("a present but invalid moment key gives no default and its notice; it is never composed", () => {
    const cases: Array<[unknown, string]> = [
      ["", "project.units.moment is empty; choose one"],
      ["   ", "project.units.moment is empty; choose one"],
      [12, "project.units.moment is not a unit symbol; choose one"],
      [null, "project.units.moment is not a unit symbol; choose one"],
      [{ symbol: "N*m" }, "project.units.moment is not a unit symbol; choose one"]
    ];
    for (const [moment, notice] of cases) {
      const model = withUnits({ length: "m", force: "N" });
      (model.project.units as Record<string, unknown>).moment = moment;
      for (const catalog of [CATALOG, PREVIEW, null]) {
        expect(primitiveLoadDefaultUnitChoice(model, "concentrated_moment", "rotation_z", catalog), JSON.stringify(moment)).toEqual({ symbol: "", notice });
      }
    }
    // With the catalog loaded, a declared moment the catalog does not list for moment is invalid too.
    expect(primitiveLoadDefaultUnitChoice(withUnits({ length: "m", force: "N", moment: "N*mm" }), "concentrated_moment", "rotation_z", CATALOG))
      .toEqual({ symbol: "", notice: "project.units.moment (N*mm) is not a catalog unit for moment; choose one" });
    // An explicitly undefined key reads as absent and composes.
    const undefinedMoment = withUnits({ length: "m", force: "N" });
    (undefinedMoment.project.units as Record<string, unknown>).moment = undefined;
    expect(primitiveLoadDefaultUnitChoice(undefinedMoment, "concentrated_moment", "rotation_z", CATALOG)).toEqual({ symbol: "N*m", notice: null });
  });

  it("a present but invalid input key gives no default and its notice; it never composes", () => {
    expect(readProjectUnitKey({}, "length")).toEqual({ state: "absent" });
    expect(readProjectUnitKey({ length: undefined }, "length")).toEqual({ state: "absent" });
    expect(readProjectUnitKey({ length: " " }, "length")).toEqual({ state: "invalid", notice: "project.units.length is empty; choose one" });
    const blankLength = withUnits({ length: "  ", force: "N" });
    for (const category of ["distributed_force", "concentrated_moment"] as const) {
      expect(primitiveLoadDefaultUnitChoice(blankLength, category, "global_y", CATALOG)).toEqual({ symbol: "", notice: "project.units.length is empty; choose one" });
    }
    const numericForce = withUnits({ length: "m" });
    (numericForce.project.units as Record<string, unknown>).force = 1;
    expect(primitiveLoadDefaultUnitChoice(numericForce, "distributed_force", "global_y", PREVIEW)).toEqual({ symbol: "", notice: "project.units.force is not a unit symbol; choose one" });
    expect(composedDefaultSymbol("thermal_expansion", { temperature: "" })).toBeNull();
  });

  it("in with N: distributed force and linear stiffness start unselected with the notice, and nothing is substituted", () => {
    const units = { ...SI_PROJECT_UNITS, length: "in" };
    expect(primitiveLoadDefaultUnitChoice(withUnits(units), "distributed_force", "global_y", CATALOG))
      .toEqual({ symbol: "", notice: "N/in is not a catalog unit; choose one" });
    expect(checkComposedDefault(CATALOG, composedDefaultSymbol("linear_stiffness", units)!, "linear_stiffness"))
      .toEqual({ symbol: "", notice: "N/in is not a catalog unit; choose one" });
    expect(composedDefaultsFor(CATALOG, units)).toEqual([
      { kind: "distributed_force", label: "Distributed force", symbol: "N/in", inCatalog: false },
      { kind: "linear_stiffness", label: "Linear stiffness", symbol: "N/in", inCatalog: false },
      { kind: "thermal_expansion", label: "Thermal expansion", symbol: "1/degC", inCatalog: true }
    ]);
    // A legacy C temperature composes 1/C, which the catalog lacks.
    expect(checkComposedDefault(CATALOG, "1/C", "thermal_expansion_coefficient")).toEqual({ symbol: "", notice: "1/C is not a catalog unit; choose one" });
  });

  it("both presets: every composed default is a catalog unit", () => {
    for (const preset of PROJECT_UNIT_PRESETS) {
      const composed = composedDefaultsFor(CATALOG, preset.units);
      expect(composed).toHaveLength(3);
      expect(composed.filter((item) => !item.inCatalog), preset.id).toEqual([]);
      expect(primitiveLoadDefaultUnit(withUnits(preset.units), "distributed_force", "global_y", CATALOG)).toBe(composed[0].symbol);
    }
  });

  it("keeps the composition unchecked where no catalog is loaded, as before", () => {
    const units = { ...SI_PROJECT_UNITS, length: "in" };
    expect(primitiveLoadDefaultUnitChoice(withUnits(units), "distributed_force", "global_y", PREVIEW)).toEqual({ symbol: "N/in", notice: null });
    expect(primitiveLoadDefaultUnit(withUnits(units), "distributed_force", "global_y")).toBe("N/in");
    expect(composedDefaultsFor(null, units).every((item) => item.inCatalog === null)).toBe(true);
  });
});

describe("the catalog fixture against the core unit engine", () => {
  it("lists only units the core accepts, and the compositions it treats as misses are refused by the core", async () => {
    const accepted = await convertDisplayQuantities(CATALOG_FIXTURE_ROWS.map(([symbol, dimension], index) => ({
      id: `row-${index}`, value: 1, from_unit: symbol, to_unit: symbol, dimension_id: dimension
    })));
    expect(accepted.filter((item) => item.status !== "converted")).toEqual([]);
    const misses: Array<[string, string]> = [
      ["N*mm", "moment"], ["kN*mm", "moment"], ["N/in", "force_per_length"], ["kN/in", "force_per_length"], ["1/C", "thermal_expansion_coefficient"],
      // Slice (c) §7.3: the acceleration entries revised (ii) needs are not yet in the core catalog;
      // §7.6: nor is the US rotational stiffness. This expectation moves when they are added.
      ["mm/s^2", "acceleration"], ["in/s^2", "acceleration"], ["ft/s^2", "acceleration"], ["lbf*in/deg", "rotational_stiffness"]
    ];
    const refused = await convertDisplayQuantities(misses.map(([symbol, dimension], index) => ({
      id: `miss-${index}`, value: 1, from_unit: symbol, to_unit: symbol, dimension_id: dimension
    })));
    expect(refused.map((item) => item.status)).toEqual(misses.map(() => "unavailable"));
    for (const [symbol, dimension] of misses) expect(canonicalCatalogEntry(CATALOG.catalog, symbol, dimension)).toBeNull();
  });
});

// W-17 slice (c), DEL-02-02 Design §7.2, §7.3, §7.6.
describe("slice (c) field defaults", () => {
  it("project temperature: the stored symbol, C as degC, and \"\" for an absent, empty, whitespace-only or non-string key", () => {
    expect(projectTemperatureDefault({ temperature: "degF" })).toBe("degF");
    expect(projectTemperatureDefault({ temperature: "C" })).toBe("degC");
    for (const units of [{}, { temperature: "" }, { temperature: "  " }, { temperature: 20 }]) expect(projectTemperatureDefault(units)).toBe("");
    // The thermal primitive load shows the same reading, with TBD for none.
    expect(primitiveLoadDefaultUnit(withUnits({ temperature: "C" }), "thermal", "global_x")).toBe("degC");
    expect(primitiveLoadDefaultUnit(withUnits({ temperature: " " }), "thermal", "global_x")).toBe("TBD");
  });

  it("acceleration composes <length>/s^2: m/s^2 for m; the named notice where the catalog lacks it; the absent key named", () => {
    expect(fieldComposedDefault(CATALOG, "acceleration", { length: "m" })).toEqual({ symbol: "m/s^2", notice: null, missingKey: null });
    for (const length of ["mm", "in", "ft"]) {
      expect(fieldComposedDefault(CATALOG, "acceleration", { length })).toEqual({ symbol: "", notice: `${length}/s^2 is not a catalog unit; choose one`, missingKey: null });
    }
    expect(fieldComposedDefault(CATALOG, "acceleration", {})).toEqual({ symbol: "", notice: null, missingKey: "length" });
    expect(fieldComposedDefault(CATALOG, "acceleration", { length: " " })).toEqual({ symbol: "", notice: "project.units.length is empty; choose one", missingKey: null });
    expect(fieldComposedDefault(PREVIEW, "acceleration", { length: "in" })).toEqual({ symbol: "in/s^2", notice: null, missingKey: null });
  });

  it("area composes <length>^2, a catalog unit for m, mm and in", () => {
    for (const length of ["m", "mm", "in"]) expect(fieldComposedDefault(CATALOG, "area", { length }).symbol).toBe(`${length}^2`);
    expect(fieldComposedDefault(CATALOG, "area", { length: "ft" }).notice).toBe("ft^2 is not a catalog unit; choose one");
  });

  it("linear and rotational stiffness: SI and the demo keep N/m and N*m/rad; US gives lbf/in and no rotational default", () => {
    const demo = { length: "m", force: "N", angle: "rad", pressure: "Pa", temperature: "degC", stress: "MPa" };
    for (const units of [SI_PROJECT_UNITS, demo]) {
      expect(linearStiffnessFieldDefault(CATALOG, units).symbol).toBe("N/m");
      expect(rotationalStiffnessFieldDefault(CATALOG, units).symbol).toBe("N*m/rad");
    }
    expect(linearStiffnessFieldDefault(CATALOG, US_CUSTOMARY_PROJECT_UNITS).symbol).toBe("lbf/in");
    expect(rotationalStiffnessFieldDefault(CATALOG, US_CUSTOMARY_PROJECT_UNITS)).toEqual({ symbol: "", notice: "lbf*in/deg is not a catalog unit; choose one", missingKey: null });
    expect(linearStiffnessFieldDefault(CATALOG, { length: "m" })).toEqual({ symbol: "", notice: null, missingKey: "force" });
    expect(rotationalStiffnessFieldDefault(CATALOG, { length: "m", force: "N" })).toEqual({ symbol: "", notice: null, missingKey: "angle" });
    expect(rotationalStiffnessFieldDefault(CATALOG, { angle: "rad" })).toEqual({ symbol: "", notice: null, missingKey: "force" });
    expect(rotationalStiffnessFieldDefault(CATALOG, { moment: "N*m", angle: "" })).toEqual({ symbol: "", notice: "project.units.angle is empty; choose one", missingKey: null });
  });
});
