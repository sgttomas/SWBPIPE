/** The oracle of the native Record Run's and Save Project's stored forms (DEL-04-05
 * `c1-native-custody-plan.md` slice 5): `apps/desktop/src/test-support/analysis-run-v03/persistence_forms.json`,
 * written by the unchanged TypeScript paths. Written only with PERSISTENCE_FORMS_WRITE=1
 * (`persistenceForms.test.ts`); otherwise byte-checked. The native leg
 * (`native_persistence_tests.rs`) writes the same forms from the held bytes and must match.
 *
 * - **Texts.** For every preview case of `analysis_run_v03_cases.json` with a record, the held
 *   text's `jsonTextKeepingNegativeZero(JSON.parse(text))` (what `analysisRunService.recordAnalysisRun`
 *   sends as `result_json`) and `JSON.stringify(JSON.parse(text))` (what Save Project's raw body
 *   carries for `mechanics_result`), each by its SHA-256 and length; and invented edge texts
 *   (array-index keys, repeated keys, negative zeros, integral floats, number boundaries, escapes,
 *   non-ASCII and supplementary-plane keys, whitespace, a number JavaScript reads as infinite),
 *   written out whole.
 * - **Envelope hashes.** For each preview case, `computeProjectEnvelopeHash` over invented small
 *   project fields (the case's model, editor intents, a proposal and a review target, and
 *   `computeModelHash` of the model) with the parsed result and the case's record, and with
 *   neither (a model-only save).
 * - Outside `fixtures/`, beside the cases it reads (their records are desktop-shaped). */
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { computeModelHash, computeProjectEnvelopeHash } from "../services/hashService";
import { jsonTextKeepingNegativeZero } from "../services/jsonText";
import type { EditorOperationIntent, PreviewModel } from "../types";
import { ANALYSIS_RUN_V03_CASES, heldText } from "./analysisRunV03Cases";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
const ROOT = resolve(__dirname, "../../../../");

export const PERSISTENCE_FORMS = "apps/desktop/src/test-support/analysis-run-v03/persistence_forms.json";

const digest = (text: string) => ({ sha256: createHash("sha256").update(text, "utf8").digest("hex"), length: new TextEncoder().encode(text).byteLength });

/** Invented JSON texts the writers must read as `JSON.parse` does. */
export const EDGE_TEXTS: readonly string[] = [
  '{"b":1,"a":2,"10":3,"2":4,"01":5,"-1":6,"4294967294":7,"4294967295":8,"":9,"0":10}',
  '{"a":1,"b":{"x":1},"a":3,"b":[2]}',
  "[-0,-0.0,0,-0e5,1.0,1e21,1e-7,1.5e300,123456789012345678901234567890,0.1,5e-324,1.7976931348623157e308,9007199254740993,-9007199254740993,2.5E-3,100]",
  '{"s":"\\u0041\\u00e9\\n\\t\\"\\\\\\/\\u2028\\u001f\\u007f\u00e9\ud83d\ude00\\ud83d\\ude00","\\u00e9key":"x","\ud83d\ude00":1,"\\uffff":2,"z\u00e9":[]}',
  ' { "x" : [ 1 , { "y" : null , "z" : true } , false ] ,\n\t"w" : { } , "v" : [ ] } ',
  '"just a string"',
  "123",
  "null",
  '{"__proto__":{"a":1},"constructor":2,"toString":"x"}',
  '{"big":1e400,"small":-1e400}',
  '[[[[[{"deep":[{"deeper":{"deepest":-0.0}}]}]]]]]',
];

const EDITOR_INTENTS: EditorOperationIntent[] = [
  { intent_id: "intent:invented-1", operation: "set_node_coordinates", target_ref: "node:invented", note: "Invented intent." } as unknown as EditorOperationIntent,
];
const PROPOSAL = { proposal_id: "proposal:invented", summary: "Invented proposal.", operations: [] };

export async function persistenceForms(): Promise<Json> {
  const committed = JSON.parse(readFileSync(resolve(ROOT, ANALYSIS_RUN_V03_CASES), "utf8"));
  const cases: Json[] = [];
  for (const c of committed.cases as Json[]) {
    if (c.kind !== "preview" || !("record" in c.expected)) continue;
    const text = heldText(c.source);
    const parsed = JSON.parse(text);
    const manifest = JSON.parse(c.input_manifest.canonical_bytes);
    const model = manifest.model_basis.model_payload as PreviewModel;
    const selected = { target_type: "result", id: String(parsed.results?.[0]?.id ?? "result:none") };
    const modelHash = await computeModelHash(model);
    const fields = { model, editor_intents: EDITOR_INTENTS, proposal: PROPOSAL as never, selected_review_target: selected as never, model_hash: modelHash };
    cases.push({
      id: c.id,
      js_text: digest(jsonTextKeepingNegativeZero(parsed)),
      stringify_text: digest(JSON.stringify(parsed)),
      save: {
        editor_intents: EDITOR_INTENTS,
        proposal: PROPOSAL,
        selected_review_target: selected,
        model_hash: modelHash,
        envelope_hash: await computeProjectEnvelopeHash({ ...fields, mechanics_result: parsed, analysis_run: c.expected.record }),
        model_only_envelope_hash: await computeProjectEnvelopeHash({ ...fields, mechanics_result: null, analysis_run: null }),
      },
    });
  }
  const edges = EDGE_TEXTS.map((text) => {
    const parsed = JSON.parse(text);
    return { text, js_text: jsonTextKeepingNegativeZero(parsed), stringify_text: JSON.stringify(parsed) };
  });
  return {
    document_kind: "swbpipe.desktop.persistence_forms",
    version: 1,
    note: "Written by persistenceFormsCases.ts from the unchanged TypeScript paths; PERSISTENCE_FORMS_WRITE=1 rewrites it.",
    cases,
    edges,
  };
}

export const persistenceFormsText = (file: Json) => `${JSON.stringify(file, null, 1)}\n`;
