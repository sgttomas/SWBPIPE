/** C1 slice 5's persistence: the oracle of today's stored forms.
 *
 * `apps/desktop/src/test-support/analysis-run-v03/persistence_forms.json` equals its generator
 * (`test-support/persistenceFormsCases.ts`): the unchanged TypeScript paths' Record Run result
 * text (`jsonTextKeepingNegativeZero`), Save Project's result text (`JSON.stringify`) and the
 * session's project envelope hashes (`computeProjectEnvelopeHash`). The native leg
 * (`native_persistence_tests.rs`) writes and hashes the same from the held bytes.
 * PERSISTENCE_FORMS_WRITE=1 rewrites the file. */
import { describe, expect, it } from "vitest";
import { readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
import { EDGE_TEXTS, PERSISTENCE_FORMS, persistenceForms, persistenceFormsText } from "../test-support/persistenceFormsCases";
import { jsonTextKeepingNegativeZero } from "./jsonText";

const ROOT = resolve(__dirname, "../../../../");
const WRITE = process.env.PERSISTENCE_FORMS_WRITE === "1";

describe("the native persistence forms' oracle", () => {
  it("the committed forms are the TypeScript paths' texts and hashes", async () => {
    const file = await persistenceForms();
    const text = persistenceFormsText(file);
    const path = resolve(ROOT, PERSISTENCE_FORMS);
    if (WRITE) writeFileSync(path, text);
    expect(readFileSync(path, "utf8") === text, "the committed forms differ from the generator (PERSISTENCE_FORMS_WRITE=1 rewrites them)").toBe(true);
    expect(file.cases.length).toBeGreaterThanOrEqual(5);
    expect(file.edges.length).toBe(EDGE_TEXTS.length);
  }, 900_000);

  it("the edge texts exercise what they name", () => {
    const [indexKeys, repeated, numbers] = EDGE_TEXTS.map((text) => jsonTextKeepingNegativeZero(JSON.parse(text)));
    expect(indexKeys).toBe('{"0":10,"2":4,"10":3,"4294967294":7,"b":1,"a":2,"01":5,"-1":6,"4294967295":8,"":9}');
    expect(repeated).toBe('{"a":3,"b":[2]}');
    expect(numbers.startsWith("[-0,-0,0,-0,1,1e+21,1e-7,1.5e+300,1.2345678901234568e+29,")).toBe(true);
    expect(JSON.stringify(JSON.parse(EDGE_TEXTS[9]))).toBe('{"big":null,"small":null}');
  });
});
