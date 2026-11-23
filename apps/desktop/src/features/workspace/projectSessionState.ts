import { useCallback, useRef, useState } from "react";
import { useSequencedState } from "./reportSourceSequence";
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
export function useProjectSessionState(advanceReportSource?: () => number) {
  // The project-command generation. Every command that can publish the project
  // status line advances it when it starts (the five project writes and opens,
  // List, Save Model Document, the recovery export, both Save Handoff stages)
  // or when it publishes an unsaved-edits refusal, so a response from an older
  // request can tell that a newer command has started.
  const projectRequest = useRef(0);
  const projectOperationOwner = useRef<symbol | null>(null);
  const [storageCapability, setStorageCapability] = useState<LocalStorageCapability | null>(null);
  // DEL-08-01 REQ-022: the project summary is report-bound (the report source sequence).
  const [projectSummary, setProjectSummary] = useSequencedState<LocalProjectSummary | null>(null, advanceReportSource);
  const [projectIndex, setProjectIndex] = useState<LocalProjectIndexEntry[] | null>(null);
  const [modelHashIntegrity, setModelHashIntegrity] = useState<ModelHashIntegrityEvidence | null>(null);
  const [projectEnvelopeHash, setProjectEnvelopeHash] = useState<ProjectEnvelopeHashEvidence | null>(null);
  const [modelDocumentVersion, setModelDocumentVersion] = useState<ModelDocumentVersionStatus | null>(null);
  const [projectEnvelopeHashIntegrity, setProjectEnvelopeHashIntegrity] =
    useState<ProjectEnvelopeHashIntegrityEvidence | null>(null);
  const [projectStatus, setProjectStatus] = useState<{ message: string; details: string | null }>({
    message: "Local project store not opened.",
    details: null
  });
  // Issue 519: every publication of the status line, counted when it is made (identical text
  // included), so that a write settling late can tell whether anything was published since it began.
  const projectMessagePublications = useRef(0);
  const setProjectMessage = useCallback(
    (next: string | ((prev: string) => string), details: string | null = null) => {
      projectMessagePublications.current += 1;
      setProjectStatus((prev) => ({
        message: typeof next === "function" ? next(prev.message) : next,
        details: typeof next === "function" ? prev.details : details
      }));
    },
    []
  );
  /** Issue 519: a write's outcome that arrives after the session moved on. Where nothing was
   * published since the write began (`seen`), `replacement` takes the line and its details are
   * cleared; otherwise `addition` is added to the newer message, whose details stay. */
  const settleLateProjectMessage = useCallback((seen: number, replacement: string, addition: string) => {
    const unchanged = projectMessagePublications.current === seen;
    projectMessagePublications.current += 1;
    setProjectStatus((prev) => unchanged ? { message: replacement, details: null } : { message: `${prev.message}${addition}`, details: prev.details });
  }, []);
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
    projectMessage: projectStatus.message,
    projectMessageDetails: projectStatus.details,
    setProjectMessage,
    projectMessagePublications, settleLateProjectMessage,
    projectOperation, setProjectOperation,
    projectBusy, setProjectBusy
  };
}
