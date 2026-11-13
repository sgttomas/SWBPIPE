// DEL-16-03 VER-005's matrix: an imported batch and a retained record of each origin, carrying
// each person-entered member at the top level of its value, nested in an object and nested in
// an array, each refused before the review queue: intake throws before it stamps or returns a
// batch (the intake panel queues only what it returns), requeue returns a refusal and no batch;
// the stored record is unchanged; no value from the operation is in the refusal. Then a text
// value that reads as JSON naming such a member, refused in words that claim no structure.
// All data are invented.
import { describe, expect, it } from "vitest";
import type { OperationBatch } from "./operationBatchService";
import { requeueRetainedRecord, stampImportedBatch } from "./operationAuthorship";

const SECRET = "SECRET-4.2-person-value";
const VALUES = {
  modifiers: { flexibility_factor_user_value: { value: 4.2, unit: "none" }, source_reference: SECRET },
  bend_realization: { radius: SECRET },
  mechanics_interface: { solver_consumption: "curved_bend_macro_element", note: SECRET },
} as const;
const LABELS = { modifiers: "modifiers (k, SIFs, stiffness and their source)", bend_realization: "bend_realization (a bend realization)",
  mechanics_interface: "a curved-bend mechanics_interface (a bend realization)" } as const;
const PLACEMENTS = {
  "the top level": (member: string, value: unknown) => [{ id: "material:m", label: "M", [member]: value }, ""],
  "an object": (member: string, value: unknown) => [{ id: "material:m", label: "M", geometry: { [member]: value } }, " at geometry." + member],
  "an array": (member: string, value: unknown) => [{ id: "material:m", label: "M", terms: [{ factor: 1, [member]: value }] }, ` at terms[0].${member}`],
} as const;
const ORIGINS = {
  agent: { author_type: "agent", source: { source_ref: "s", source_channel: "embedded_agent", source_role: "agent" } },
  imported: { author_type: "user", source: { source_ref: "f", source_channel: "offline_intake", source_role: "imported" } },
  user: { author_type: "user", source: { source_ref: "me", source_channel: "local_desktop_preview", source_role: "gui_editor" } },
} as const;
const operation = (after: unknown, origin: object) => ({ operation_id: "op:m", operation_kind: "create", target: { object_type: "Material", ref: "material:m" },
  change: { change_id: "c", change_kind: "create_material", field_label: "m", field_path: "materials", before: "not_present", after: JSON.stringify(after), unit: "none", dimension: "dimensionless", source_note: "file" },
  rationale: "invented", ...origin });

describe("each person-entered member, in each placement, at intake and at requeue, for each origin", () => {
  for (const [member, value] of Object.entries(VALUES)) for (const [placement, place] of Object.entries(PLACEMENTS)) for (const [origin, claim] of Object.entries(ORIGINS)) {
    it(`${member} in ${placement}, ${origin}`, () => {
      const [after, at] = place(member, value);
      const label = `${LABELS[member as keyof typeof LABELS]}${at}`;
      const file = { batch_id: "file", operations: [operation(after, claim)] } as unknown as OperationBatch;
      const fileBefore = JSON.stringify(file);
      let returned: unknown = null, message = "";
      try { returned = stampImportedBatch(file); } catch (error) { message = (error as Error).message; }
      expect(returned).toBeNull();
      expect(message).toBe(`Operation 1 (op:m) of this file has a value that names ${label}, which only you enter. A value naming it cannot be told apart from it, even as text, so nothing was queued. Remove it from the file, then enter the value yourself in the component inspector.`);
      expect(message).not.toContain(SECRET);
      expect(JSON.stringify(file)).toBe(fileBefore);

      const stored = operation(after, claim);
      const storedBefore = JSON.stringify(stored);
      const result = requeueRetainedRecord(stored, 4);
      expect(result).toEqual({ refused: `This stored record has a value that names ${label}, which only you enter. A value naming it cannot be told apart from it, even as text. It was not queued for review; the record stays stored and nothing of yours is lost. Enter the value yourself in the component inspector, or prepare the change again without it.` });
      expect(result).not.toHaveProperty("batch");
      expect(JSON.stringify(result)).not.toContain(SECRET);
      expect(JSON.stringify(stored)).toBe(storedBefore);
    });
  }
});

describe("a text value that reads as JSON naming a person-entered member", () => {
  // A material label is literal text to the engine; the web view cannot tell it from a member
  // without the engine's field kinds, so it is refused, in words that claim no structure.
  const label = (origin: object) => ({ operation_id: "op:label", operation_kind: "modify", target: { object_type: "Material", ref: "material:m" },
    change: { change_id: "c", change_kind: "set_field", field_label: "Label", field_path: "label", before: "M", after: '{"modifiers":{}}', unit: "none", dimension: "dimensionless", source_note: "s" },
    rationale: "r", ...origin });
  it("is refused at intake and at requeue without claiming a member was found", () => {
    expect(() => stampImportedBatch({ batch_id: "file", operations: [label(ORIGINS.imported)] } as unknown as OperationBatch))
      .toThrow("Operation 1 (op:label) of this file has a value that names modifiers (k, SIFs, stiffness and their source), which only you enter. A value naming it cannot be told apart from it, even as text, so nothing was queued.");
    for (const claim of Object.values(ORIGINS)) {
      const result = requeueRetainedRecord(label(claim), 1);
      expect(result).toEqual({ refused: expect.stringMatching(/^This stored record has a value that names modifiers \(k, SIFs, stiffness and their source\), which only you enter\. A value naming it cannot be told apart from it, even as text\./) });
      expect(JSON.stringify(result)).not.toMatch(/carries|would not write/);
    }
  });
});
