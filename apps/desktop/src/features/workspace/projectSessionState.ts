import { useRef, useState } from "react";
import type {
  LocalProjectIndexEntry,
  LocalProjectSummary,
  LocalStorageCapability,
  ModelDocumentVersionStatus,
  ModelHashIntegrityEvidence,
  ProjectEnvelopeHashEvidence,
  ProjectEnvelopeHashIntegrityEvidence
} from "../../types";

/**
 * The project cells of the workspace session: the project request counter, the
 * storage capability, the project summary and index, the hash, integrity and
 * version evidence of the open project, and the project message, operation
 * and busy flag. It declares no effect and no handler.
 *
 * Called only by the session, which is `useWorkspaceSession` in
 * `workspaceSession.ts`. Its setters are not for components: the storage
 * capability is set by the session's mount effect, and every other cell here
 * only by its project handlers.
 */
export function useProjectSessionState() {
  // The project-command generation. Every command that can publish the project
  // status line advances it when it starts (the five project writes and opens,
  // List, Save Model Document, the recovery export, both Save Handoff stages)
  // or when it publishes an unsaved-edits refusal, so a response from an older
  // request can tell that a newer command has started.
  const projectRequest = useRef(0);
  const projectOperationOwner = useRef<symbol | null>(null);
  const [storageCapability, setStorageCapability] = useState<LocalStorageCapability | null>(null);
  const [projectSummary, setProjectSummary] = useState<LocalProjectSummary | null>(null);
  const [projectIndex, setProjectIndex] = useState<LocalProjectIndexEntry[] | null>(null);
  const [modelHashIntegrity, setModelHashIntegrity] = useState<ModelHashIntegrityEvidence | null>(null);
  const [projectEnvelopeHash, setProjectEnvelopeHash] = useState<ProjectEnvelopeHashEvidence | null>(null);
  const [modelDocumentVersion, setModelDocumentVersion] = useState<ModelDocumentVersionStatus | null>(null);
  const [projectEnvelopeHashIntegrity, setProjectEnvelopeHashIntegrity] =
    useState<ProjectEnvelopeHashIntegrityEvidence | null>(null);
  const [projectMessage, setProjectMessage] = useState("Local project store not opened.");
  const [projectOperation, setProjectOperation] = useState("not_started");
  const [projectBusy, setProjectBusy] = useState(false);
  return {
    projectRequest,
    projectOperationOwner,
    storageCapability, setStorageCapability,
    projectSummary, setProjectSummary,
    projectIndex, setProjectIndex,
    modelHashIntegrity, setModelHashIntegrity,
    projectEnvelopeHash, setProjectEnvelopeHash,
    modelDocumentVersion, setModelDocumentVersion,
    projectEnvelopeHashIntegrity, setProjectEnvelopeHashIntegrity,
    projectMessage, setProjectMessage,
    projectOperation, setProjectOperation,
    projectBusy, setProjectBusy
  };
}
