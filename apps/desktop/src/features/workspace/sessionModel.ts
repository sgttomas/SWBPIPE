// Helpers for the session's model: the undo and redo checkpoint, the UI identity
// hash of a model assignment, the entity that an applied operation selects, and
// the model clone. No React.

import type { EntityRef, OperationOutcome, PreviewModel } from "../../types";

export type SessionModelCheckpoint = {
  checkpoint_id: string;
  operation_id: string;
  model: PreviewModel;
  selection: EntityRef;
};

export function uiModelIdentityHash(model: PreviewModel): string {
  const text = JSON.stringify(model);
  let first = 0x811c9dc5;
  let second = 0x9e3779b9;
  for (let index = 0; index < text.length; index += 1) {
    const unit = text.charCodeAt(index);
    first = Math.imul(first ^ unit, 0x01000193);
    second = Math.imul(second ^ unit, 0x85ebca6b);
  }
  return `ui-model-fnv32x2:${(first >>> 0).toString(16).padStart(8, "0")}${
    (second >>> 0).toString(16).padStart(8, "0")
  }:${text.length}`;
}

export function selectionForOperationOutcome(outcome: OperationOutcome): EntityRef | null {
  const selectionTypeByObjectType: Record<string, EntityRef["type"]> = {
    Material: "material",
    Section: "section",
    Node: "node",
    Element: "pipe",
    Component: "component",
    Support: "support",
    Load: "load",
    Combination: "combination"
  };
  const type = selectionTypeByObjectType[outcome.target_object_type];
  return type ? { type, id: outcome.target_ref } : null;
}

export function clonePreviewModel(model: PreviewModel): PreviewModel {
  return JSON.parse(JSON.stringify(model)) as PreviewModel;
}
