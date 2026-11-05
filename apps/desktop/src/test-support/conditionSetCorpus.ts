// Model document 0.9.0 S3b tests: base documents from S3a's committed contract corpus
// (`fixtures/model_operations/condition_set_contract_corpus.json`), read from the
// repository. No document is authored here.
import { readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import type { PreviewModel } from "../types";

const projectRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "../../../..");
type CorpusCase = { case_id: string; base_model: PreviewModel };
let cases: CorpusCase[] | null = null;

/** A fresh copy of the base document of the corpus case whose id starts with `prefix`. */
export function corpusBase(prefix: string): PreviewModel {
  cases ??= (JSON.parse(readFileSync(path.join(projectRoot, "fixtures/model_operations/condition_set_contract_corpus.json"), "utf8")) as { cases: CorpusCase[] }).cases;
  const found = cases.find((item) => item.case_id.startsWith(prefix));
  if (!found) throw new Error(`no condition-set corpus case ${prefix}`);
  return structuredClone(found.base_model);
}

/** The same document for the workspace session, which reads `components` and `diagnostics`:
 * the corpus omits both collections, and empty ones state the same (none). */
export function sessionCorpusBase(prefix: string): PreviewModel {
  const model = corpusBase(prefix);
  model.components ??= [];
  model.diagnostics ??= [];
  return model;
}
