// DEL-04-11 REQ-008 (review round 1 of pull request 258, finding 3): the hanger-selection builder refuses a sized load
// that is zero or not finite, for each sized member, and builds nothing. The model and library are the invented
// two-span sizing input (read, not changed); the run identity is invented, as in `hangerSelection.sized.test.ts`.
import { describe, expect, it } from "vitest";
import twoSpanInput from "../../../../../fixtures/results/hanger_sizing_input_two_span.json";
import type { PreviewModel } from "../../types";
import { buildHangerSelectionBatch, type HangerDocument, type HangerSizingApplication } from "./hangerSelection";

const model = () => structuredClone(twoSpanInput.request.model) as unknown as PreviewModel;
const library = () => structuredClone(twoSpanInput.library) as unknown as HangerDocument;
const TARGET = { supportId: "support:B", restraints: ["UZ"] };
const run = (hangerId: string) => ({
  sizingRequestSha256: "1".repeat(64), solveRequestSha256: "2".repeat(64), libraryId: "invented.hangers.sizing",
  librarySha256: "3".repeat(64), hangerId, recordSha256: "4".repeat(64)
});
const REFUSED = "A sized load is not a positive finite force; nothing is applied.";
const BAD = [0, -0, Number.NaN, Number.POSITIVE_INFINITY, Number.NEGATIVE_INFINITY] as const;

describe("the builder's refusal of a sized load that is not a positive finite force", () => {
  for (const value of BAD) {
    it(`refuses a constant load of ${String(value)}`, async () => {
      const sizing: HangerSizingApplication = { run: run("CE-1"), loads: { family: "constant_effort", constantLoadN: value } };
      await expect(buildHangerSelectionBatch(model(), library(), "CE-1", TARGET, sizing)).rejects.toThrow(REFUSED);
    });

    it(`refuses a variable spring's cold load, and its hot load, of ${String(value)}`, async () => {
      const loads = (coldLoadN: number, hotLoadN: number): HangerSizingApplication =>
        ({ run: run("VS-200-L"), loads: { family: "variable_spring", coldLoadN, hotLoadN, designOperatingCase: "load:O1" } });
      await expect(buildHangerSelectionBatch(model(), library(), "VS-200-L", TARGET, loads(value, 5000))).rejects.toThrow(REFUSED);
      await expect(buildHangerSelectionBatch(model(), library(), "VS-200-L", TARGET, loads(5197.5, value))).rejects.toThrow(REFUSED);
    });
  }
});
