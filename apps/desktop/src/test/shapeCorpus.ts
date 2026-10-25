// Test support: the shared desktop shape corpus
// (`fixtures/model_document/desktop_shape_cases.json`). Each case is a copy of
// the corpus's schema-valid base document with the case's edits applied in
// memory (`value` set at `path`, or the member removed when `remove` is true),
// or the case's `replace`. The native twin is
// `src-tauri/src/model_document_shape.rs` `corpus_case_document`.
import corpus from "../../../../fixtures/model_document/desktop_shape_cases.json";

export type ShapeFinding = { code: string; path: string; message: string };
type Edit = { path: Array<string | number>; value?: unknown; remove?: boolean };
export type ShapeCase = { case_id: string; edits?: Edit[]; replace?: unknown; expected: ShapeFinding[] };

export const SHAPE_CASES = corpus.cases as unknown as ShapeCase[];

export function shapeCaseDocument(item: ShapeCase): unknown {
  if ("replace" in item) return structuredClone(item.replace);
  const document = structuredClone(corpus.base) as Record<string, unknown>;
  for (const edit of item.edits ?? []) {
    const parents = edit.path.slice(0, -1);
    const last = edit.path[edit.path.length - 1];
    let target = document as Record<string | number, unknown>;
    for (const step of parents) target = target[step] as Record<string | number, unknown>;
    if (edit.remove) delete target[last];
    else target[last] = structuredClone(edit.value);
  }
  return document;
}
