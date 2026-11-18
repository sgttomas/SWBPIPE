/** A sizing run's application with its precondition (DEL-04-11 REQ-008, AX-002). The one way any caller prepares a
 * sized selection for the review queue: the batch for the run, built by the one hanger-selection builder from the
 * library the person has stored now, and the precondition the queue asks at the final Apply, which reads the stored
 * library again and holds only while it is the one the result binds. The two are returned together so that they are
 * registered together; the session refuses a sized batch that comes without its precondition. */

import { canonicalSha256HexCheckedV1 } from "../../services/hashService";
import { openLocalLibrary } from "../../services/libraryImportService";
import type { OperationBatch } from "../../services/operationBatchService";
import type { PreviewModel } from "../../types";
import type { HangerSizingSizedDocument } from "../results/hangerSizingResult";
import type { BatchPrecondition } from "../workspace/workspaceSession";
import { HANGER_SIZING_RUN_NOTHING_TEXT, prepareRunApplication, type RunChange } from "./runApplication";
import {
  HANGER_SIZING_LIBRARY_CHANGED_TEXT,
  HANGER_SIZING_LIBRARY_UNAVAILABLE_TEXT,
  HANGER_SIZING_MODEL_CHANGED_TEXT,
  HANGER_SIZING_MODEL_NOT_HASHABLE_TEXT,
  hangerSizingLibraryBound,
  hangerSizingModelBinding,
  hangerSizingSelectedCount,
  type StoredLibraryRead
} from "./sizedSelection";

/** Reads the hanger library stored now under `libraryId` in project `projectId`. */
export type StoredHangerLibraryReader = (projectId: string, libraryId: string) => Promise<StoredLibraryRead>;

/** The hanger library stored now: its document only when it opens with no blocking import finding; otherwise
 * `unavailable` (no longer stored, unreadable, refused, or not on the native host). Never throws. */
export async function readStoredHangerLibrary(projectId: string, libraryId: string): Promise<StoredLibraryRead> {
  try {
    const result = await openLocalLibrary(projectId, "hanger", libraryId);
    if (result.route !== "tauri_backend" || !result.envelope) return { kind: "unavailable" };
    if (!result.envelope.validation.accepted || result.envelope.validation.has_blocking_findings) return { kind: "unavailable" };
    return { kind: "read", document: result.envelope.document };
  } catch {
    return { kind: "unavailable" };
  }
}

/** Why a run's application is not prepared: a closed word, with the plain reason shown to the person. */
export type SizedApplicationRefusal = "model_changed" | "library_changed" | "library_unavailable" | "nothing_to_apply" | "candidate_not_applicable";

export type SizedApplication =
  | Readonly<{
      kind: "prepared";
      batch: OperationBatch;
      /** Asked by the queue at the final Apply: null while the stored library is the one the result binds. */
      precondition: BatchPrecondition;
      changed: readonly RunChange[];
      unchanged: readonly string[];
      description: string;
    }>
  | Readonly<{ kind: "refused"; refusal: SizedApplicationRefusal; reason: string }>;

type LibraryRefusal = Readonly<{ refusal: "library_changed" | "library_unavailable"; reason: string }>;
const LIBRARY_CHANGED: LibraryRefusal = { refusal: "library_changed", reason: HANGER_SIZING_LIBRARY_CHANGED_TEXT };
const LIBRARY_UNAVAILABLE: LibraryRefusal = { refusal: "library_unavailable", reason: HANGER_SIZING_LIBRARY_UNAVAILABLE_TEXT };

async function checkedHash(value: unknown): Promise<string | null> {
  try {
    return await canonicalSha256HexCheckedV1(value);
  } catch {
    return null;
  }
}

/** Whether `stored` is the library `document` binds: the library's checked hash, and each applied record's. */
async function storedLibraryRefusal(document: HangerSizingSizedDocument, stored: StoredLibraryRead): Promise<LibraryRefusal | null> {
  if (stored.kind === "unavailable") return LIBRARY_UNAVAILABLE;
  if (!(await hangerSizingLibraryBound(document, stored.document))) return LIBRARY_CHANGED;
  const records = (stored.document as { hanger_records?: unknown }).hanger_records;
  const list = Array.isArray(records) ? records as { hanger_id?: unknown }[] : [];
  for (const candidate of document.result.candidates) {
    if (candidate.outcome === "no_record_fits") continue;
    const matches = list.filter((record) => record?.hanger_id === candidate.selected.hanger_id);
    if (matches.length !== 1 || (await checkedHash(matches[0])) !== candidate.selected.record_sha256) return LIBRARY_CHANGED;
  }
  return null;
}

async function readSafely(read: StoredHangerLibraryReader, projectId: string, libraryId: string): Promise<StoredLibraryRead> {
  try {
    return await read(projectId, libraryId);
  } catch {
    return { kind: "unavailable" };
  }
}

/** The run's application to `model` with its precondition, or why it is refused. `read` reads the stored library;
 * the desktop's is `readStoredHangerLibrary`. The batch is built from the stored library's content, which is the
 * bound content when it is offered. */
export async function prepareSizedApplication(
  document: HangerSizingSizedDocument,
  model: PreviewModel,
  projectId: string,
  read: StoredHangerLibraryReader = readStoredHangerLibrary
): Promise<SizedApplication> {
  const binding = await hangerSizingModelBinding(document, model);
  if (binding !== "current") {
    return { kind: "refused", refusal: "model_changed", reason: binding === "changed" ? HANGER_SIZING_MODEL_CHANGED_TEXT : HANGER_SIZING_MODEL_NOT_HASHABLE_TEXT };
  }
  if (hangerSizingSelectedCount(document) === 0) return { kind: "refused", refusal: "nothing_to_apply", reason: HANGER_SIZING_RUN_NOTHING_TEXT };
  const libraryId = document.inputs.library.library_id;
  const stored = await readSafely(read, projectId, libraryId);
  const refused = await storedLibraryRefusal(document, stored);
  if (refused !== null || stored.kind !== "read") return { kind: "refused", ...(refused ?? LIBRARY_UNAVAILABLE) };
  const prepared = await prepareRunApplication(document, model, stored.document);
  if (prepared.kind === "not_offered") return { kind: "refused", refusal: "candidate_not_applicable", reason: prepared.reason };
  const precondition: BatchPrecondition = async () => {
    const again = await storedLibraryRefusal(document, await readSafely(read, projectId, libraryId));
    return again === null ? null : again.reason;
  };
  return { kind: "prepared", batch: prepared.batch, precondition, changed: prepared.changed, unchanged: prepared.unchanged, description: prepared.description };
}
