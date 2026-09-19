import { useRef, useState } from "react";
import type {
  LocalProjectIndexEntry,
  LocalProjectSummary,
  LocalStorageCapability,
  ModelDocumentMigrationStatus,
  ModelHashIntegrityEvidence,
  ModelMigrationLedgerRecord,
  ProjectEnvelopeHashEvidence,
  ProjectEnvelopeHashIntegrityEvidence
} from "../../types";

/**
 * The project cells of the workspace session: the project request counter, the
 * storage capability, the project summary and index, the hash, integrity and
 * migration evidence of the open project, and the project message, operation
 * and busy flag. It declares no effect and no handler.
 *
 * Called only by the session, which is `AppSession` in `App.tsx`. Its setters
 * are not for components: the storage capability is set by the session's mount
 * effect, and every other cell here only by its project handlers.
 */
export function useProjectSessionState() {
  const projectRequest = useRef(0);
  const [storageCapability, setStorageCapability] = useState<LocalStorageCapability | null>(null);
  const [projectSummary, setProjectSummary] = useState<LocalProjectSummary | null>(null);
  const [projectIndex, setProjectIndex] = useState<LocalProjectIndexEntry[] | null>(null);
  const [modelHashIntegrity, setModelHashIntegrity] = useState<ModelHashIntegrityEvidence | null>(null);
  const [projectEnvelopeHash, setProjectEnvelopeHash] = useState<ProjectEnvelopeHashEvidence | null>(null);
  const [modelDocumentMigration, setModelDocumentMigration] = useState<ModelDocumentMigrationStatus | null>(null);
  const [modelMigrationLedger, setModelMigrationLedger] = useState<ModelMigrationLedgerRecord[]>([]);
  const [projectEnvelopeHashIntegrity, setProjectEnvelopeHashIntegrity] =
    useState<ProjectEnvelopeHashIntegrityEvidence | null>(null);
  const [projectMessage, setProjectMessage] = useState("Local project store not opened.");
  const [projectOperation, setProjectOperation] = useState("not_started");
  const [projectBusy, setProjectBusy] = useState(false);
  return {
    projectRequest,
    storageCapability, setStorageCapability,
    projectSummary, setProjectSummary,
    projectIndex, setProjectIndex,
    modelHashIntegrity, setModelHashIntegrity,
    projectEnvelopeHash, setProjectEnvelopeHash,
    modelDocumentMigration, setModelDocumentMigration,
    modelMigrationLedger, setModelMigrationLedger,
    projectEnvelopeHashIntegrity, setProjectEnvelopeHashIntegrity,
    projectMessage, setProjectMessage,
    projectOperation, setProjectOperation,
    projectBusy, setProjectBusy
  };
}
