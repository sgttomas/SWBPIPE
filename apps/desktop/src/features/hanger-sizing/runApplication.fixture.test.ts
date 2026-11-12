// DEL-04-11 REQ-008: the model the desktop writes when it applies the whole-run batch of a product sizing result,
// pinned in a committed fixture so that a native test (`core/analysis/hanger_sizing/tests/applied_agreement.rs`) can
// solve the very bytes the desktop writes and the two cannot drift. Each triple is a committed input, its committed
// result, whose binding holds as committed, with no re-binding, and the applied fixture. The batch is applied by the wasm
// operation engine, the route the person's acceptance runs. A fixture holds the applied model, not the batch, whose
// operation and change ids are random:
// - `hanger_sizing_applied_four_span.json`: the four-span run (B and C constant, D no fit, conditional);
// - `hanger_sizing_applied_four_span_constant.json`: the four-span run with B and C constant (unconditional).
//
// With `HANGER_SIZING_WRITE_FIXTURES=1` (the switch the Rust generator uses) this test writes the fixture, as the
// repository's other hanger fixtures are written: pretty JSON with sorted keys and a trailing newline. Without it, it
// asserts that what the desktop produces today equals the committed file exactly.
import { readFileSync, writeFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import { canonicalJsonString, canonicalSha256HexCheckedV1, computeModelHash } from "../../services/hashService";
import { applyOperationBatch } from "../../services/operationBatchService";
import type { PreviewModel } from "../../types";
import type { HangerSizingSizedDocument } from "../results/hangerSizingResult";
import { prepareRunApplication } from "./runApplication";

const RESULTS = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "../../../../../fixtures/results");
const read = (name: string): unknown => JSON.parse(readFileSync(path.join(RESULTS, name), "utf8"));

/** (input, result, applied) triples: the committed input and result, and the applied fixture written or checked. */
const TRIPLES = [
  ["hanger_sizing_input_four_span.json", "hanger_sizing_result_four_span.json", "hanger_sizing_applied_four_span.json"],
  ["hanger_sizing_input_four_span_constant.json", "hanger_sizing_result_four_span_constant.json", "hanger_sizing_applied_four_span_constant.json"]
] as const;
const writing = () => process.env.HANGER_SIZING_WRITE_FIXTURES === "1";

/** The value with every object's keys in sorted order, as the Rust generator's pretty writer orders them. */
function sortedKeys(value: unknown): unknown {
  if (Array.isArray(value)) return value.map(sortedKeys);
  if (value === null || typeof value !== "object") return value;
  return Object.fromEntries(Object.keys(value).sort().map((key) => [key, sortedKeys((value as Record<string, unknown>)[key])]));
}

describe("the model applied from a product sizing result", () => {
  it.each(TRIPLES)("%s with %s is the committed %s", async (inputName, resultName, appliedName) => {
    const input = read(inputName) as { request: { model: unknown }; library: unknown };
    const model = structuredClone(input.request.model) as unknown as PreviewModel;
    const library = structuredClone(input.library) as Record<string, unknown>;
    const result = read(resultName) as HangerSizingSizedDocument;
    // The committed result binds the committed input's solve request as it stands.
    expect(await canonicalSha256HexCheckedV1(input.request)).toBe(result.inputs.solve_request.sha256);

    const prepared = await prepareRunApplication(result, model, library);
    if (prepared.kind !== "prepared") throw new Error(prepared.reason);
    const outcome = await applyOperationBatch(model, prepared.batch, await computeModelHash(model));
    expect(outcome.validation.application_status, JSON.stringify(outcome.diagnostics)).toBe("applied_to_session_model");
    expect(outcome.simulation_disposition).toBe("committed_as_one_batch");

    const produced = sortedKeys({
      source: { input: inputName, result: resultName },
      batch_after_model: outcome.applied_model,
      changed_supports: prepared.changed.map((change) => ({ support: change.support, hanger_id: change.hangerId, family: change.family })),
      unchanged_supports: prepared.unchanged
    });
    const text = `${JSON.stringify(produced, null, 2)}\n`;
    const fixture = path.join(RESULTS, appliedName);
    if (writing()) writeFileSync(fixture, text);

    const committed = JSON.parse(readFileSync(fixture, "utf8")) as { batch_after_model: unknown };
    expect(committed, "rerun with HANGER_SIZING_WRITE_FIXTURES=1").toEqual(JSON.parse(text));
    expect(await canonicalJsonString(committed.batch_after_model)).toBe(await canonicalJsonString(outcome.applied_model));
  });
});
