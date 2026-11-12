// DEL-04-11 REQ-008: the model the desktop writes when it applies the whole-run batch of the product's four-span result,
// pinned in `fixtures/results/hanger_sizing_applied_four_span.json` so that a native test can solve the very bytes the
// desktop writes and the two cannot drift. The batch is built from the committed input and result
// (`hanger_sizing_input_four_span.json`, `hanger_sizing_result_four_span.json`), whose binding holds as committed, with
// no re-binding, and is applied by the wasm operation engine, the route the person's acceptance runs. The fixture holds
// the applied model, not the batch, whose operation and change ids are random.
//
// With `HANGER_SIZING_WRITE_FIXTURES=1` (the switch the Rust generator uses) this test writes the fixture, as the
// repository's other hanger fixtures are written: pretty JSON with sorted keys and a trailing newline. Without it, it
// asserts that what the desktop produces today equals the committed file exactly.
import { readFileSync, writeFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import fourSpanInput from "../../../../../fixtures/results/hanger_sizing_input_four_span.json";
import fourSpanResult from "../../../../../fixtures/results/hanger_sizing_result_four_span.json";
import { canonicalJsonString, canonicalSha256HexCheckedV1, computeModelHash } from "../../services/hashService";
import { applyOperationBatch } from "../../services/operationBatchService";
import type { PreviewModel } from "../../types";
import type { HangerSizingSizedDocument } from "../results/hangerSizingResult";
import { prepareRunApplication } from "./runApplication";

const FIXTURE = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "../../../../../fixtures/results/hanger_sizing_applied_four_span.json");
const writing = () => process.env.HANGER_SIZING_WRITE_FIXTURES === "1";

/** The value with every object's keys in sorted order, as the Rust generator's pretty writer orders them. */
function sortedKeys(value: unknown): unknown {
  if (Array.isArray(value)) return value.map(sortedKeys);
  if (value === null || typeof value !== "object") return value;
  return Object.fromEntries(Object.keys(value).sort().map((key) => [key, sortedKeys((value as Record<string, unknown>)[key])]));
}

describe("the model applied from the product's four-span sizing result", () => {
  it("is the committed fixture", async () => {
    const model = structuredClone(fourSpanInput.request.model) as unknown as PreviewModel;
    const library = structuredClone(fourSpanInput.library) as Record<string, unknown>;
    const result = structuredClone(fourSpanResult) as unknown as HangerSizingSizedDocument;
    // The committed result binds the committed input's solve request as it stands.
    expect(await canonicalSha256HexCheckedV1(fourSpanInput.request)).toBe(result.inputs.solve_request.sha256);

    const prepared = await prepareRunApplication(result, model, library);
    if (prepared.kind !== "prepared") throw new Error(prepared.reason);
    const outcome = await applyOperationBatch(model, prepared.batch, await computeModelHash(model));
    expect(outcome.validation.application_status, JSON.stringify(outcome.diagnostics)).toBe("applied_to_session_model");
    expect(outcome.simulation_disposition).toBe("committed_as_one_batch");

    const produced = sortedKeys({
      source: { input: "hanger_sizing_input_four_span.json", result: "hanger_sizing_result_four_span.json" },
      batch_after_model: outcome.applied_model,
      changed_supports: prepared.changed.map((change) => ({ support: change.support, hanger_id: change.hangerId, family: change.family })),
      unchanged_supports: prepared.unchanged
    });
    const text = `${JSON.stringify(produced, null, 2)}\n`;
    if (writing()) writeFileSync(FIXTURE, text);

    const committed = JSON.parse(readFileSync(FIXTURE, "utf8")) as { batch_after_model: unknown };
    expect(committed, "rerun with HANGER_SIZING_WRITE_FIXTURES=1").toEqual(JSON.parse(text));
    expect(await canonicalJsonString(committed.batch_after_model)).toBe(await canonicalJsonString(outcome.applied_model));
  });
});
