// RT-E1 milestone 2: the webview link of the component journey. The fixture's
// agent proposals are stamped by the embedded-agent session and applied phase by
// phase through the desktop batch service on the wasm engine; each correction's
// `before` text comes from the session's record producer (the texts
// swbpipe_read_records returns), so the applier itself confirms they are exact.
// The operation applier test (core/model_operations/operation_applier/tests/
// agent_m2_component_journey.rs) adds the person's own k and SIF entries and the
// solve. All data are invented.
import { describe, expect, it } from "vitest";
import fixture from "../../../../fixtures/model_operations/agent_m2_component_journey.json";
import type { PreviewModel } from "../types";
import { componentsAwaitingUserModifiers, recordBefore, stampAgentOperations, type RecordFamily } from "./embeddedAgentSession";
import { applyOperationBatch } from "./operationBatchService";
import { canonicalJsonString, computeModelHash } from "./hashService";

const { proposalHandle, requestRef } = fixture.host;
const FAMILY: Record<string, RecordFamily> = { component: "components", support: "supports" };

async function resolveBefore(model: PreviewModel, placeholder: string): Promise<string> {
  const [, kind, name, key] = placeholder.split(":");
  const family = FAMILY[kind], id = `${kind}:${name}`;
  const record = (model as unknown as Record<string, Record<string, unknown>[]>)[family].find(entry => entry.id === id)!;
  const before = await recordBefore(family, record, model, await canonicalJsonString(record));
  expect(before[key], placeholder).toEqual(expect.any(String));
  return before[key];
}

describe("embedded agent milestone 2 component journey (webview link)", () => {
  it("applies every phase as one agent batch, corrections with record-producer before texts", async () => {
    let model = structuredClone(fixture.blank.model) as unknown as PreviewModel;
    for (const proposal of fixture.proposals) {
      const operations = await Promise.all((proposal.operations as Record<string, unknown>[]).map(async operation => {
        const change = operation.change as Record<string, string>;
        return change.before.startsWith("BEFORE:") ? { ...operation, change: { ...change, before: await resolveBefore(model, change.before) } } : operation;
      }));
      expect(operations.length).toBeLessThanOrEqual(16);
      const hash = await computeModelHash(model);
      const batch = { batch_id: `agent-${proposal.phase}`, operations: stampAgentOperations(operations, proposalHandle, requestRef), source_model_hash: hash!.value };
      const outcome = await applyOperationBatch(model, batch, hash);
      expect(outcome.validation.batch_validation_status, `${proposal.phase}: ${JSON.stringify(outcome.diagnostics)}`).toBe("passed");
      model = outcome.applied_model!;
      if (proposal.phase === "components") expect(componentsAwaitingUserModifiers(model)).toEqual(["component:bend-n3", "component:tee-n2"]);
    }
    expect(model.components.map(component => component.kind)).toEqual(["bend", "tee", "reducer", "valve", "expansion_joint"]);
    expect(model.supports.find(support => support.id === "support:n4-guide")?.restraints).toEqual(["UX"]);
  });

  it("stops reporting a component once the person's k, SIFs and source are entered", () => {
    const model = { components: [
      { id: "b", kind: "bend", modifiers: { sif_user_value: { value: 1.5, unit: "none" }, flexibility_factor_user_value: { value: 2, unit: "none" }, source_reference: "person" } },
      { id: "r", kind: "bend", mechanics_interface: { solver_consumption: "curved_bend_macro_element" }, modifiers: { sif_user_value: { value: 1.5, unit: "none" }, flexibility_factor_user_value: { value: 2, unit: "none" }, source_reference: "person" } },
      { id: "rp", kind: "bend", mechanics_interface: { solver_consumption: "curved_bend_macro_element" }, modifiers: { sif_in_plane_user_value: { value: 2.1, unit: "none" }, sif_out_of_plane_user_value: { value: 1.8, unit: "none" }, flexibility_factor_user_value: { value: 2, unit: "none" }, source_reference: "person" } },
      { id: "t", kind: "tee", modifiers: { branch_header_sif_user_value: { value: 1.4, unit: "none" }, branch_branch_sif_user_value: { value: 1.6, unit: "none" }, flexibility_factor_user_value: { value: 1, unit: "none" }, source_reference: " " } },
      { id: "v", kind: "valve" }
    ] } as unknown as PreviewModel;
    // A realized bend with only the scalar SIF still awaits the directional pair.
    expect(componentsAwaitingUserModifiers(model)).toEqual(["r", "t"]);
  });
});
