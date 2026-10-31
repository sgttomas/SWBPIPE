// RT-E1 milestone 1: the webview link of the W9-A agent journey. The fixture's
// agent-form proposal (what swbpipe_propose_operations carries, validated by the
// catalog test in embedded_agent/catalog.rs) is stamped by the embedded-agent
// session into the native batch that the operation applier test
// (core/model_operations/operation_applier/tests/agent_w9a_journey.rs) applies to
// the same blank document and solves against the frozen W9-A references. Here the
// stamp and the blank are bound to that fixture and the batch is applied through
// the desktop's own batch service on the wasm engine. Solving needs the native
// route and is not exercised here. All data are invented.
import { describe, expect, it } from "vitest";
import fixture from "../../../../fixtures/model_operations/agent_w9a_journey.json";
import type { PreviewModel } from "../types";
import { stampAgentOperations, ADMITTED_OPERATION_KINDS } from "./embeddedAgentSession";
import { applyOperationBatch, validateOperationBatch } from "./operationBatchService";
import { buildBlankLocalModelDocument } from "./projectService";
import { computeModelHash } from "./hashService";

const routes = ["PF", "EX"] as const;
const { proposalHandle, requestRef, stamp } = fixture.host;

describe("embedded agent W9-A journey (webview link)", () => {
  it("starts from the desktop's own blank document", () => {
    expect(buildBlankLocalModelDocument(new Date(fixture.blank.createdAt))).toEqual(fixture.blank.model);
  });

  for (const route of routes) {
    const operations = fixture.routes[route].operations as Record<string, unknown>[];

    it(`stamps the ${route} agent-form proposal into the fixture's native intents`, () => {
      const native = stampAgentOperations(operations, proposalHandle, requestRef);
      expect(native).toEqual(operations.map(operation => ({
        ...operation, ...stamp, queue_id: proposalHandle,
        source: { source_ref: requestRef, source_channel: "embedded_agent", source_role: "agent" }
      })));
      const kinds = operations.map(operation => {
        const change = operation.change as Record<string, string>, target = operation.target as Record<string, string>;
        return [operation.operation_kind, target.object_type, change.change_kind, ...(change.change_kind === "update_load" || change.change_kind === "set_field" ? [change.field_path] : [])].join("/");
      });
      for (const kind of kinds) expect(ADMITTED_OPERATION_KINDS).toContain(kind);
    });

    it(`queues and applies the ${route} build from blank as one agent batch`, async () => {
      const blank = structuredClone(fixture.blank.model) as unknown as PreviewModel;
      const hash = await computeModelHash(blank);
      expect(hash).not.toBeNull();
      const batch = { batch_id: `agent-${proposalHandle}`, operations: stampAgentOperations(operations, proposalHandle, requestRef), source_model_hash: hash!.value };
      const preview = await validateOperationBatch(blank, batch, hash);
      expect(preview.validation.batch_validation_status).toBe("passed");
      expect(preview.applied_model).toBeNull();
      const outcome = await applyOperationBatch(blank, batch, hash);
      expect(outcome.validation.batch_validation_status).toBe("passed");
      const model = outcome.applied_model!;
      expect(model.nodes.map(node => node.id)).toEqual(["node:a", "node:c", "node:d"]);
      expect(model.pipe_segments.map(pipe => pipe.id)).toEqual(["pipe:a-c", "pipe:c-d"]);
      expect(model.supports.map(support => support.id)).toEqual(["support:a", "support:d"]);
      expect(model.load_cases[0].primitive_loads?.map(load => load.id)).toEqual(["thermal:pipe:a-c", "thermal:pipe:c-d"]);
      expect((model as unknown as Record<string, unknown>).pressure_contract).toEqual(route === "EX" ? { version: "3.0.0", mode: "exact_pressure_v3", shear_model: "euler_bernoulli" } : undefined);
      expect(blank).toEqual(fixture.blank.model);
    });
  }
});
