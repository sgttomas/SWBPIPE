import { describe, expect, it } from "vitest";
import { buildBlankLocalModelDocument } from "../../services/projectService";
import type { PreviewModel } from "../../types";
import {
  defaultPrimitiveLoadDirection,
  optionalString,
  primitiveLoadCategoryFromValue,
  primitiveLoadDefaultUnit,
  primitiveLoadDirectionOptions,
  primitiveLoadDofIsRotational,
  primitiveLoadDraftDimension,
  primitiveLoadUsesPipeTarget,
  primitiveLoadUsesSupportTarget,
  type PrimitiveLoadCategory
} from "./primitiveLoadVocabulary";

// Invented project unit metadata; the vocabulary only echoes what the model declares.
function withUnits(units: Record<string, string>): PreviewModel {
  const model = buildBlankLocalModelDocument(new Date("2026-01-01T00:00:00Z"));
  model.project.units = units;
  return model;
}

const CATEGORIES: PrimitiveLoadCategory[] = ["concentrated_force", "distributed_force", "concentrated_moment", "thermal", "imposed_displacement"];

describe("primitive load category vocabulary", () => {
  it("maps each category to its target kind, direction choices, default direction and dimension", () => {
    const table = CATEGORIES.map((category) => {
      const direction = defaultPrimitiveLoadDirection(category);
      return {
        category,
        target: primitiveLoadUsesSupportTarget(category) ? "support" : primitiveLoadUsesPipeTarget(category) ? "pipe" : "node",
        options: primitiveLoadDirectionOptions(category).join(","),
        direction,
        dimension: primitiveLoadDraftDimension(category, direction)
      };
    });
    expect(table).toEqual([
      { category: "concentrated_force", target: "node", options: "global_x,global_y,global_z", direction: "global_y", dimension: "force" },
      { category: "distributed_force", target: "pipe", options: "global_x,global_y,global_z", direction: "global_y", dimension: "force_per_length" },
      { category: "concentrated_moment", target: "node", options: "rotation_x,rotation_y,rotation_z", direction: "rotation_z", dimension: "moment" },
      { category: "thermal", target: "pipe", options: "global_x,global_y,global_z", direction: "global_z", dimension: "temperature_interval" },
      { category: "imposed_displacement", target: "support", options: "UX,UY,UZ,RX,RY,RZ", direction: "UZ", dimension: "displacement" }
    ]);
    for (const category of CATEGORIES) {
      expect(primitiveLoadDirectionOptions(category)).toContain(defaultPrimitiveLoadDirection(category));
    }
  });

  it("splits imposed displacement into displacement and rotation by DOF", () => {
    for (const dof of ["UX", "UY", "UZ"]) {
      expect(primitiveLoadDofIsRotational(dof)).toBe(false);
      expect(primitiveLoadDraftDimension("imposed_displacement", dof)).toBe("displacement");
    }
    for (const dof of ["RX", "RY", "RZ"]) {
      expect(primitiveLoadDofIsRotational(dof)).toBe(true);
      expect(primitiveLoadDraftDimension("imposed_displacement", dof)).toBe("rotation");
    }
    // Only the imposed-displacement category reads the DOF.
    expect(primitiveLoadDofIsRotational("rotation_z")).toBe(false);
    expect(primitiveLoadDraftDimension("concentrated_moment", "RX")).toBe("moment");
  });

  it("reads unknown category values as concentrated force", () => {
    for (const category of CATEGORIES) expect(primitiveLoadCategoryFromValue(category)).toBe(category);
    expect(primitiveLoadCategoryFromValue("pressure")).toBe("concentrated_force");
    expect(primitiveLoadCategoryFromValue("")).toBe("concentrated_force");
    expect(primitiveLoadCategoryFromValue("Thermal")).toBe("concentrated_force");
  });

  it("derives default units only from declared project units", () => {
    const model = withUnits({ force: "kN", length: "mm", moment: "kN*m", temperature: "C", angle: "rad" });
    expect(primitiveLoadDefaultUnit(model, "concentrated_force", "global_y")).toBe("kN");
    expect(primitiveLoadDefaultUnit(model, "distributed_force", "global_y")).toBe("kN/mm");
    // W-17 §2.4: the moment reads the moment key; it is not composed as kN*mm.
    expect(primitiveLoadDefaultUnit(model, "concentrated_moment", "rotation_z")).toBe("kN*m");
    // The legacy "C" spelling is offered as the temperature-interval symbol degC.
    expect(primitiveLoadDefaultUnit(model, "thermal", "global_z")).toBe("degC");
    expect(primitiveLoadDefaultUnit(model, "imposed_displacement", "UZ")).toBe("mm");
    expect(primitiveLoadDefaultUnit(model, "imposed_displacement", "RX")).toBe("rad");
    expect(primitiveLoadDefaultUnit(withUnits({ temperature: "degF" }), "thermal", "global_z")).toBe("degF");
  });

  it("reports TBD instead of inventing a unit when the project does not declare one", () => {
    const empty = withUnits({});
    for (const [category, direction] of [
      ["concentrated_force", "global_y"], ["distributed_force", "global_y"], ["concentrated_moment", "rotation_z"],
      ["thermal", "global_z"], ["imposed_displacement", "UZ"], ["imposed_displacement", "RZ"]
    ] as const) {
      expect(primitiveLoadDefaultUnit(empty, category, direction)).toBe("TBD");
    }
    // A compound unit needs both parts. W-17 §2.4: a present but blank part is not
    // absent; the compositions give no default ("", unselected) rather than TBD.
    const forceOnly = withUnits({ force: "N", length: "  " });
    expect(primitiveLoadDefaultUnit(forceOnly, "concentrated_force", "global_y")).toBe("N");
    expect(primitiveLoadDefaultUnit(forceOnly, "distributed_force", "global_y")).toBe("");
    expect(primitiveLoadDefaultUnit(forceOnly, "concentrated_moment", "rotation_z")).toBe("");
    expect(primitiveLoadDefaultUnit(withUnits({ force: "N" }), "distributed_force", "global_y")).toBe("TBD");
    expect(primitiveLoadDefaultUnit(withUnits({ force: "N" }), "concentrated_moment", "rotation_z")).toBe("TBD");
    // W-17: the blank document declares angle (rad), so a rotational imposed displacement has a unit.
    const blank = buildBlankLocalModelDocument(new Date("2026-01-01T00:00:00Z"));
    expect(primitiveLoadDefaultUnit(blank, "imposed_displacement", "RY")).toBe("rad");
    expect(primitiveLoadDefaultUnit(blank, "thermal", "global_z")).toBe("degC");
  });

  it("treats only non-blank strings as present", () => {
    expect(optionalString("kN")).toBe("kN");
    expect(optionalString(" kN ")).toBe(" kN ");
    expect(optionalString("   ")).toBeNull();
    expect(optionalString("")).toBeNull();
    expect(optionalString(5)).toBeNull();
    expect(optionalString(undefined)).toBeNull();
  });
});
