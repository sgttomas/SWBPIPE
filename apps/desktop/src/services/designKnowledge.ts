import type { DesignKnowledge, Diagnostic, KnowledgeRecord, PreviewModel } from "../types";

/** A model, or a model ref (a result's `model_ref`), that a knowledge set may be bound to. */
export type KnowledgeModel = Pick<PreviewModel, "project"> | string | null | undefined;

/**
 * The design knowledge that belongs with a model: the knowledge when its declared `model_ref`
 * names that model's project, and none otherwise (another model, a knowledge with no
 * `model_ref`, no knowledge, no model). The session loads one knowledge set (`loadDesignKnowledge`)
 * and keeps it whatever model is open, so every place that shows, counts or carries the knowledge
 * for a model reads it through this family and never reads its `diagnostics` or `records` directly
 * (`designKnowledge.test.tsx` checks the source for that).
 */
export function designKnowledgeFor(knowledge: DesignKnowledge | null | undefined, model: KnowledgeModel): DesignKnowledge | null {
  const modelRef = typeof model === "string" ? model : model?.project?.id;
  if (!knowledge || typeof knowledge.model_ref !== "string" || !knowledge.model_ref || !modelRef) return null;
  return knowledge.model_ref === modelRef ? knowledge : null;
}

/** The design knowledge's diagnostics that belong with a model (`designKnowledgeFor`); none for
 * any other model. */
export function designKnowledgeDiagnostics(knowledge: DesignKnowledge | null | undefined, model: KnowledgeModel): Diagnostic[] {
  return designKnowledgeFor(knowledge, model)?.diagnostics ?? [];
}

/** The design knowledge's records that belong with a model (`designKnowledgeFor`); none for any
 * other model. */
export function designKnowledgeRecords(knowledge: DesignKnowledge | null | undefined, model: KnowledgeModel): KnowledgeRecord[] {
  return designKnowledgeFor(knowledge, model)?.records ?? [];
}
