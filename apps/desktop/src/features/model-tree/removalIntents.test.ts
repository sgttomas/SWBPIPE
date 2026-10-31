// The removal intents' `before` texts must be the summaries the operation applier
// compares, or the inspector's own Delete is refused as stale. Each deletion here is
// applied by the actual wasm engine on an invented model whose labels carry padding
// and whose coordinates print with an exponent in ECMAScript.
import { describe, expect, it } from "vitest";
import { applyOperationBatch } from "../../services/operationBatchService";
import { computeModelHash } from "../../services/hashService";
import { bulkChangeModel } from "../../test-support/bulkChangeModel";
import type { EditorOperationIntent, PreviewModel } from "../../types";
import { applierDisplayNumber, buildDeleteNodeIntent, buildDeletePipeIntent, buildDeleteSupportIntent } from "./removalIntents";

const PROV = "invented removal-intent test input";

function paddedModel(): PreviewModel {
  const model = bulkChangeModel();
  const pipe = model.pipe_segments[0];
  model.nodes.push(
    { id: "node:E", label: "  E padded ", position: { x: 1e-7, y: 2.5e21, z: 0.1 + 0.2 }, provenance: PROV },
    { id: "node:F", label: "\tF\n", position: { x: -1.5e-8, y: -0, z: 123456789.125 }, provenance: PROV }
  );
  model.pipe_segments.push({ ...structuredClone(pipe), id: "pipe:P4", label: " P4 ", from: "node:E", to: "node:F" });
  model.supports.push({ id: "support:E", label: "  Stop E  ", node: "node:E", family: "line_stop", restraints: ["UX"], provenance: PROV } as PreviewModel["supports"][number]);
  return model;
}

async function apply(model: PreviewModel, intent: EditorOperationIntent) {
  const hash = await computeModelHash(model);
  return applyOperationBatch(model, { batch_id: `batch:${intent.operation_id}`, operations: [intent], source_model_hash: hash!.value }, hash);
}

describe("removal intents", () => {
  it("send the applier's own deletion summaries", () => {
    const model = paddedModel();
    expect(buildDeleteSupportIntent(model.supports.at(-1)!, model).change.before).toBe("Stop E");
    expect(buildDeletePipeIntent(model.pipe_segments.at(-1)!, model).change.before).toBe("P4; node:E->node:F; material=material:steel");
    expect(buildDeleteNodeIntent(model.nodes.at(-2)!, model).change.before).toBe("E padded; x=0.0000001; y=2500000000000000000000; z=0.30000000000000004");
    expect(buildDeleteNodeIntent(model.nodes.at(-1)!, model).change.before).toBe("F; x=-0.000000015; y=0; z=123456789.125");
  });

  it("delete a padded support, pipe and nodes through the engine", async () => {
    let model = paddedModel();
    const stale = buildDeleteNodeIntent(model.nodes.find(node => node.id === "node:E")!, model);
    stale.change.before = "  E padded ; x=1e-7; y=2.5e+21; z=0.30000000000000004";
    expect((await apply(model, stale)).validation.batch_validation_status).not.toBe("passed");
    const steps: ((current: PreviewModel) => EditorOperationIntent)[] = [
      current => buildDeleteSupportIntent(current.supports.find(support => support.id === "support:E")!, current),
      current => buildDeletePipeIntent(current.pipe_segments.find(pipe => pipe.id === "pipe:P4")!, current),
      current => buildDeleteNodeIntent(current.nodes.find(node => node.id === "node:E")!, current),
      current => buildDeleteNodeIntent(current.nodes.find(node => node.id === "node:F")!, current)
    ];
    for (const step of steps) {
      const intent = step(model);
      const outcome = await apply(model, intent);
      expect(outcome.validation.batch_validation_status, `${intent.target.ref}: ${JSON.stringify(outcome.diagnostics)}`).toBe("passed");
      model = outcome.applied_model!;
    }
    expect(model.nodes.map(node => node.id)).toEqual(["node:A", "node:B", "node:C", "node:D"]);
    expect(model.supports.map(support => support.id)).not.toContain("support:E");
  });

  it("format numbers as the applier's display_number does", () => {
    expect([0, -0, 2, -3, 1e15, 2.5e21, 0.5, 1e-7, -1.5e-8, 1.2345e-12, 0.1 + 0.2, 123456789.125].map(applierDisplayNumber))
      .toEqual(["0", "0", "2", "-3", "1000000000000000", "2500000000000000000000", "0.5", "0.0000001", "-0.000000015", "0.0000000000012345", "0.30000000000000004", "123456789.125"]);
  });
});
