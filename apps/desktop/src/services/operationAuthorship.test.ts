// DEL-16-03 / DEL-16-06 author routes: offline intake and retained-record requeue
// stamp agent or imported content `agent` host-side, so a file or stored record that
// claims `user` cannot satisfy the applier's refusals of an agent-written k or other
// component modifier after the person's Apply. The final case runs the stamped import
// through the desktop batch service on the wasm engine. All data are invented.
import { describe, expect, it } from "vitest";
import type { OperationBatch } from "./operationBatchService";
import { applyOperationBatch } from "./operationBatchService";
import { computeModelHash } from "./hashService";
import { loadPreviewModel } from "./previewService";
import { parseOfflineBatch } from "../features/offline-proposal-intake/OfflineProposalIntakePanel";
import { requeueRetainedRecord, stampImportedBatch, writesUserEnteredModifier } from "./operationAuthorship";

const op = (over: Record<string, unknown>) => ({ operation_id: "op:x", operation_kind: "modify", operation_status: "proposed",
  target: { object_type: "Component", ref: "component:C-110" },
  change: { change_id: "c:x", change_kind: "set_field", field_label: "x", field_path: "modifiers.sif_user_value.value", before: "TBD", after: "{\"value\":1.5,\"unit\":\"none\"}", unit: "none", dimension: "dimensionless", source_note: "file" },
  validation: { schema_validation: "not_run", constraint_validation: "not_run", unit_validation: "not_run", diff_preview_status: "not_generated", application_status: "not_applied" },
  rationale: "file rationale",
  audit_boundary: { mutation_route: "structured_operations_only", direct_model_mutation_allowed: false, requires_user_acceptance: true, mutates_accepted_model_state: false },
  professional_boundary: { human_review_required: true, software_makes_compliance_claim: false, software_makes_certification_claim: false, software_makes_sealing_claim: false, software_makes_approval_claim: false, software_makes_authentication_claim: false },
  ...over });

describe("operation authorship on agent and imported routes", () => {
  it("recognizes every route that writes a value only the person enters", () => {
    expect(writesUserEnteredModifier(op({}))).toMatchObject({ route: "refused" });
    expect(writesUserEnteredModifier(op({ change: { change_kind: "set_field", field_path: "bend_realization" } }))).toMatchObject({ route: "refused" });
    expect(writesUserEnteredModifier(op({ target: { object_type: "Node", ref: "n" }, change: { change_kind: "insert_bend_at_corner", field_path: "bend_at_corner" } }))).toMatchObject({ route: "refused" });
    expect(writesUserEnteredModifier(op({ change: { change_kind: "insert_component_symbol", field_path: "components", after: "{\"modifiers\":{}}" } }))).toMatchObject({ route: "refused" });
    expect(writesUserEnteredModifier(op({ change: { change_kind: "apply_library_record", field_path: "components", after: "{\"modifiers.sif_user_value\":{}}" } }))).toMatchObject({ route: "refused" });
    expect(writesUserEnteredModifier(op({ change: { change_kind: "set_field", field_path: "geometry.bend_radius.value" } }))).toBeNull();
    expect(writesUserEnteredModifier(op({ target: { object_type: "Node", ref: "n" }, change: { change_kind: "set_field", field_path: "position.x" } }))).toBeNull();
  });

  it("requeues stored records with a host-derived author", () => {
    const ordinaryChange = { change_kind: "set_field", field_path: "position.x", before: "0", after: "1" };
    const node = { target: { object_type: "Node", ref: "n" }, change: ordinaryChange };
    const agent = requeueRetainedRecord(op({ ...node, author_type: "user", source: { source_ref: "s", source_channel: "embedded_agent", source_role: "agent" } }), 3);
    expect("batch" in agent && agent.batch.operations[0].author_type).toBe("agent");
    const imported = requeueRetainedRecord(op({ ...node, author_type: "user", source: { source_ref: "f", source_channel: "offline_intake", source_role: "imported" } }), 0);
    expect("batch" in imported && imported.batch.operations[0].author_type).toBe("agent");
    const claimedUser = requeueRetainedRecord(op({ author_type: "user" }), 1);
    expect(claimedUser).toEqual({ refused: expect.stringContaining("author cannot be verified") });
    const ordinary = requeueRetainedRecord(op({ author_type: "user", target: { object_type: "Node", ref: "n" }, change: { change_kind: "set_field", field_path: "position.x", before: "0", after: "1" } }), 2);
    expect("batch" in ordinary && ordinary.batch.operations[0].author_type).toBe("user");
    expect(requeueRetainedRecord(op({ author_type: undefined }), 4)).toEqual({ refused: expect.stringContaining("does not say who proposed it") });
    expect(requeueRetainedRecord("not an operation", 5)).toEqual({ refused: expect.any(String) });
  });

  it("a file claiming the user cannot write k or a SIF: intake refuses it, and the engine still refuses it stamped agent", async () => {
    const model = await loadPreviewModel();
    const file = { batch_id: "batch:claimed-user", operations: [
      op({ operation_id: "op:sif", author_type: "user", source: { source_ref: "me", source_channel: "local_desktop_preview", source_role: "gui_editor" } }),
      op({ operation_id: "op:k", author_type: "user", change: { ...op({}).change, change_id: "c:k", field_path: "modifiers.flexibility_factor_user_value.value", after: "{\"value\":2,\"unit\":\"none\"}" } })
    ] } as unknown as OperationBatch;
    expect(() => stampImportedBatch(parseOfflineBatch(JSON.stringify(file)))).toThrow(/^Operation 1 \(op:sif\) of this file carries modifiers\.sif_user_value\.value \(a component modifier\), a value only you enter; an imported operation cannot write it, so nothing was queued\./);
    // The applier's own refusal, should any route stamp such an operation agent.
    for (const [index, operation] of file.operations.entries()) {
      const single = { batch_id: `import-${index}`, operations: [{ ...operation, author_type: "agent", source: { source_ref: `offline-import:batch:claimed-user:${index + 1}`, source_channel: "offline_intake", source_role: "imported" } }] } as unknown as OperationBatch;
      const hash = await computeModelHash(model);
      const outcome = await applyOperationBatch(model, single, hash);
      expect(outcome.applied_model).toBeNull();
      const codes = outcome.operation_outcomes.flatMap(step => step.diagnostics.map(d => d.code)).concat(outcome.diagnostics.map(d => d.code));
      expect(codes).toContain(index === 0 ? "OP-COMPONENT-MODIFIER-USER-ENTRY-REQUIRED" : "OP-BEND-FLEXIBILITY-USER-ENTRY-REQUIRED");
    }
  });

  const SECRET = "SECRET-4.2-person-value";
  const carrying = {
    modifiers: { change: { change_kind: "create_material", field_path: "materials", after: JSON.stringify({ id: "material:m", label: "M", modifiers: { flexibility_factor_user_value: { value: 4.2, unit: "none" }, source_reference: SECRET } }) }, target: { object_type: "Material", ref: "material:m" }, operation_kind: "create" },
    bend_realization: { change: { change_kind: "create_node", field_path: "nodes", after: JSON.stringify({ id: "node:n", bend_realization: { note: SECRET } }) }, target: { object_type: "Node", ref: "node:n" }, operation_kind: "create" },
    mechanics_interface: { change: { change_kind: "insert_component_symbol", field_path: "components", after: JSON.stringify({ id: "component:b", kind: "bend", label: SECRET, mechanics_interface: { solver_consumption: "curved_bend_macro_element" } }) }, target: { object_type: "Component", ref: "component:b" }, operation_kind: "insert" },
  } as const;
  const LABELS = { modifiers: "modifiers (k, SIFs, stiffness and their source)", bend_realization: "bend_realization (a bend realization)", mechanics_interface: "a curved-bend mechanics_interface (a bend realization)" };

  it("offline intake refuses each member form by name before anything is queued, and queues a clean batch as before", () => {
    for (const [member, over] of Object.entries(carrying)) {
      const batch = { batch_id: "file", operations: [op({ operation_id: "op:clean", target: { object_type: "Node", ref: "n" }, change: { change_kind: "set_field", field_path: "position.x", before: "0", after: "1" } }), op({ operation_id: "op:carrying", ...over })] } as unknown as OperationBatch;
      let message = "";
      try { stampImportedBatch(batch); } catch (error) { message = (error as Error).message; }
      expect(message, member).toBe(`Operation 2 (op:carrying) of this file carries ${LABELS[member as keyof typeof LABELS]}, a value only you enter; an imported operation cannot write it, so nothing was queued. Remove it from the file, then enter the value yourself in the component inspector.`);
      expect(message).not.toContain(SECRET);
    }
    const clean = { batch_id: "file", operations: [op({ operation_id: "op:clean", target: { object_type: "Node", ref: "n" }, change: { change_kind: "set_field", field_path: "position.x", before: "0", after: "1" } })] } as unknown as OperationBatch;
    expect(stampImportedBatch(clean).batch.operations[0]).toMatchObject({ operation_id: "op:clean", author_type: "agent", source: { source_channel: "offline_intake" } });
  });

  it("a stored user record that creates a component with modifiers keeps its own refusal; a stray modifiers member on a material gets the new one", () => {
    const component = op({ author_type: "user", operation_kind: "insert", change: { change_kind: "insert_component_symbol", field_path: "components", before: "not_present",
      after: JSON.stringify({ id: "component:t", kind: "tee", modifiers: { branch_header_sif_user_value: { value: 2.4, unit: "1" } } }) } });
    expect(requeueRetainedRecord(component, 8)).toEqual({ refused: "This stored record writes a value only you enter (a flexibility factor k, a SIF or another component modifier), and its author cannot be verified after reopening. Enter the value again in the component inspector." });
    const material = op({ author_type: "user", operation_kind: "create", target: { object_type: "Material", ref: "material:m" }, change: { change_kind: "create_material", field_path: "materials", before: "not_present",
      after: JSON.stringify({ id: "material:m", modifiers: {} }) } });
    expect(requeueRetainedRecord(material, 9)).toEqual({ refused: expect.stringMatching(/^This stored record carries modifiers \(k, SIFs, stiffness and their source\), a value only you enter, which the operation engine would not write for it\. It was not queued for review; the record stays stored/) });
  });

  it("intake leaves a non-operation entry and a non-object after text for the engine, without a refusal of its own", () => {
    const batch = { batch_id: "file", operations: ["not an operation", op({ operation_id: "op:text", change: { change_kind: "create_material", field_path: "materials", after: "not json" } }),
      op({ operation_id: "op:array", change: { change_kind: "create_material", field_path: "materials", after: "[1,2]" } })] } as unknown as OperationBatch;
    const { batch: stamped } = stampImportedBatch(batch);
    expect(stamped.operations[0]).toBe("not an operation");
    expect(stamped.operations.slice(1).map(operation => operation.author_type)).toEqual(["agent", "agent"]);
  });

  it("the reviewer's nested combination is refused at intake and at requeue, naming its path", () => {
    const review = '{"id":"combination:review-nested","label":"Review example","basis":"mechanics","terms":[{"load_case":"load:RS-SUS","factor":1,"modifiers":{"sif_user_value":{"value":2.4,"unit":"1"}}}],"provenance":"invented review example"}';
    const operation = op({ operation_id: "op:review", operation_kind: "create", target: { object_type: "Combination", ref: "combination:review-nested" },
      change: { change_kind: "create_combination", field_path: "combinations", before: "not_present", after: review } });
    expect(() => stampImportedBatch({ batch_id: "file", operations: [operation] } as unknown as OperationBatch))
      .toThrow("Operation 1 (op:review) of this file carries modifiers (k, SIFs, stiffness and their source) at terms[0].modifiers, a value only you enter; an imported operation cannot write it, so nothing was queued.");
    for (const author of [{ author_type: "agent", source: { source_ref: "s", source_channel: "embedded_agent", source_role: "agent" } }, { author_type: "user" }]) {
      const result = requeueRetainedRecord({ ...operation, ...author }, 3);
      expect(result).toEqual({ refused: expect.stringMatching(/^This stored record carries modifiers \(k, SIFs, stiffness and their source\) at terms\[0\]\.modifiers, a value only you enter/) });
      expect((result as { refused: string }).refused).not.toContain("2.4");
    }
    let deep: unknown = { leaf: 1 };
    for (let i = 0; i < 40; i++) deep = [deep];
    const bounded = op({ operation_id: "op:deep", change: { change_kind: "create_material", field_path: "materials", after: JSON.stringify(deep) } });
    expect(() => stampImportedBatch({ batch_id: "file", operations: [bounded] } as unknown as OperationBatch)).toThrow(/^Operation 1 \(op:deep\) of this file carries an after value nested deeper than 32 levels or with more than 50000 members, which is not checked for values only the person enters, so nothing was queued\./);
    expect(requeueRetainedRecord({ ...bounded, author_type: "user" }, 4)).toEqual({ refused: expect.stringMatching(/^This stored record carries an after value nested deeper than 32 levels.*It was not queued for review; the record stays stored and nothing of yours is lost\./) });
    // The member-count bound, at both entries.
    const wide = op({ operation_id: "op:wide", change: { change_kind: "create_material", field_path: "materials", after: JSON.stringify(Array.from({ length: 50_001 }, () => 0)) } });
    expect(() => stampImportedBatch({ batch_id: "file", operations: [wide] } as unknown as OperationBatch)).toThrow(/^Operation 1 \(op:wide\) of this file carries an after value nested deeper than 32 levels or with more than 50000 members, which is not checked for values only the person enters, so nothing was queued\./);
    expect(requeueRetainedRecord({ ...wide, author_type: "agent", source: { source_ref: "s", source_channel: "embedded_agent", source_role: "agent" } }, 5)).toEqual({ refused: expect.stringMatching(/^This stored record carries an after value nested deeper than 32 levels or with more than 50000 members.*the record stays stored and nothing of yours is lost\./) });
  });

  it("requeue refuses a stored agent, imported or user record carrying such a member, and leaves it stored", () => {
    for (const [member, over] of Object.entries(carrying)) for (const author of [
      { author_type: "agent", source: { source_ref: "s", source_channel: "embedded_agent", source_role: "agent" } },
      { author_type: "user", source: { source_ref: "f", source_channel: "offline_intake", source_role: "imported" } },
      { author_type: "user" },
    ]) {
      const stored = op({ operation_id: "op:stored", ...over, ...author });
      const before = JSON.stringify(stored);
      const result = requeueRetainedRecord(stored, 7);
      expect(result, `${member} ${JSON.stringify(author)}`).toEqual({ refused: `This stored record carries ${LABELS[member as keyof typeof LABELS]}, a value only you enter, which the operation engine would not write for it. It was not queued for review; the record stays stored and nothing of yours is lost. Enter the value yourself in the component inspector, or prepare the change again without it.` });
      expect(JSON.stringify(stored)).toBe(before);
      expect((result as { refused: string }).refused).not.toContain(SECRET);
    }
  });

});
