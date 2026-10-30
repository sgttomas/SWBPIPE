import { useMemo, useRef, useState } from "react";
import type { DesignKnowledge, ModelHashEvidence, PreviewModel } from "../../types";
import { modelIndexFor } from "./modelIndex";

/**
 * The model cells of the workspace session: the model and the design knowledge,
 * the project-session generation and the UI model revision with their refs, the
 * model assignment, the model hash, the revision and current-model refs that the
 * stale-response guards read, and the model index. It declares no effect and no
 * handler.
 *
 * Called only by the session, which is `useWorkspaceSession` in
 * `workspaceSession.ts`. Its setters are not for components: the model changes
 * only in the session's mount effect and in its operation, undo, redo and
 * project handlers.
 */
export function useModelSessionState() {
  const [model, setModel] = useState<PreviewModel | null>(null);
  const [knowledge, setKnowledge] = useState<DesignKnowledge | null>(null);
  const [projectSessionGeneration, setProjectSessionGeneration] = useState(0);
  const projectSessionGenerationRef = useRef(0);
  const [uiModelRevision, setUiModelRevision] = useState(0);
  const uiModelRevisionRef = useRef(0);
  const modelPublicationGenerationRef = useRef(0);
  const [modelAssignment, setModelAssignment] = useState<{
    status: "started" | "committed";
    generation: number;
    indexGeneration: string;
    identityHash: string;
    startedAt: number;
    committedAt: number | null;
  } | null>(null);
  const [modelHash, setModelHash] = useState<ModelHashEvidence | null>(null);
  const modelRevision = useRef(0);
  const currentModel = useRef<PreviewModel | null>(null);
  const activeModelIndex = useMemo(
    () => model ? modelIndexFor(model, projectSessionGeneration, uiModelRevision) : null,
    [model, projectSessionGeneration, uiModelRevision]
  );
  return {
    model, setModel,
    knowledge, setKnowledge,
    projectSessionGeneration, setProjectSessionGeneration,
    projectSessionGenerationRef,
    uiModelRevision, setUiModelRevision,
    uiModelRevisionRef,
    modelPublicationGenerationRef,
    modelAssignment, setModelAssignment,
    modelHash, setModelHash,
    modelRevision,
    currentModel,
    activeModelIndex
  };
}
