// File > Save Model Document… (DEL-15-01 Design `canonical-model-export.md` §2,
// W-6a; FR-HAND-001, FR-MOD-001). The saved bytes are the unchecked RFC 8785
// (JCS) text of the session's model document, the serializer of the session's
// model hash (`hashService.canonicalJsonString`, wasm `canonical_json`), with no
// trailing newline, so the file's SHA-256 is the model hash.
//
// Before anything is sent to the native host the webview applies, in order:
// the explicit local-private intent (the dialog's own control; the file chooser
// alone is not the intent), a current owned model hash, the current version,
// the open-time checks (the desktop shape check and the W-13 document validator,
// as at open), the disposition screening below, and the governed route
// `DOTH-MODEL-DOC-007` (local-private, lossless, exact payload). The native
// command re-checks the route evidence, the open cap, the canonical text and
// hash, and the open path's document checks before it shows the chooser
// (`src-tauri/src/model_document_save.rs`).
//
// Disposition screening: the route projection passes `provenance_record`,
// `rule_pack_refs[]` and library dispositions through as opaque leaves, so they
// are screened explicitly here with the screener's own classifier
// (`classifyExportItem`). `protected_suspected`, `quarantined` and `rejected`
// refuse the save with the screener's code, naming the records; every other
// disposition, `private_only` and `unknown` included, saves.
//
// Nothing here transmits anything or records telemetry (F-PIP-1). A save never
// changes the session's model, hash or solve standing.
import { canonicalJsonString } from "../../services/hashService";
import { invokeWithIngress, nativeFailureText } from "../../services/nativeIngress";
import { isTauriRuntime } from "../../services/nativeMenu";
import { MODEL_SCHEMA_VERSION } from "../../services/modelSchemaVersion";
import { openLocalLibrary, type LibraryKind, type LibraryOpenRoute } from "../../services/libraryImportService";
import type { ModelHashEvidence, PreviewModel } from "../../types";
import {
  classifyExportItem,
  controlRouteExport,
  type LocalFirstRouteEvidence,
  type RedactionDecision
} from "../redaction-controls/redactionExportControls";
import { modelDocumentShapeDiagnostics, modelDocumentSlotValidation, type ModelFileDiagnostic } from "./modelDocumentShape";

export const MODEL_DOCUMENT_SAVE_ROUTE_ID = "DOTH-MODEL-DOC-007";
export const MODEL_DOCUMENT_SAVE_INTENT_LABEL =
  "Save my model document, including private project data, to a file I choose";
/** The open cap (`src-tauri/src/model_document_file.rs` MAX_MODEL_DOCUMENT_BYTES). */
export const MAX_MODEL_DOCUMENT_BYTES = 64 * 1024 * 1024;

export const MODEL_DOC_SAVE_DESKTOP_ONLY = "MODEL-DOC-SAVE-DESKTOP-ONLY";
export const MODEL_DOC_SAVE_INTENT_REQUIRED = "MODEL-DOC-SAVE-INTENT-REQUIRED";
export const MODEL_DOC_SAVE_HASH_PENDING = "MODEL-DOC-SAVE-HASH-PENDING";
export const MODEL_DOC_SAVE_VERSION_NOT_CURRENT = "MODEL-DOC-SAVE-VERSION-NOT-CURRENT";
export const MODEL_DOC_SAVE_TOO_LARGE = "MODEL-DOC-SAVE-TOO-LARGE";
export const MODEL_DOC_SAVE_ROUTE_BLOCKED = "MODEL-DOC-SAVE-ROUTE-BLOCKED";
export const MODEL_DOC_SAVE_RECEIPT_INVALID = "MODEL-DOC-SAVE-RECEIPT-INVALID";

export type { ModelFileDiagnostic };

/** A stored library's disposition for one `provenance_record.library_ref`
 * (`record_id: null` is the library's own provenance). */
export type ResolvedLibraryDisposition = {
  library_kind: string;
  library_id: string;
  record_id: string | null;
  redistribution_status: string | null;
  review_status: string | null;
};

export type ModelDocumentSaveScreening = {
  route_id: string;
  export_context: "local_private";
  explicit_local_private_intent: boolean;
  blocked: boolean;
  materialization_withheld: boolean;
  lossless_required: boolean;
  exact_payload_match: boolean;
  blocking_count: number;
};

/** The native command's arguments (`save_model_document_file`). */
export type ModelDocumentSaveArgs = {
  canonicalText: string;
  claimedModelHash: string;
  screening: ModelDocumentSaveScreening;
  localFirstEvidence: LocalFirstRouteEvidence;
};

export type ModelDocumentSavePreparation =
  | { status: "ready"; args: ModelDocumentSaveArgs; decisions: RedactionDecision[] }
  | { status: "refused"; diagnostics: ModelFileDiagnostic[] };

export type ModelDocumentSaveOutcome =
  | { outcome: "cancelled" }
  | { outcome: "saved"; file_name: string; byte_count: number; model_hash: string; replaced_existing: boolean; durability: string }
  | { outcome: "refused"; diagnostics: ModelFileDiagnostic[] };

type PlainObject = Record<string, unknown>;

function isPlainObject(value: unknown): value is PlainObject {
  return typeof value === "object" && value !== null && !Array.isArray(value);
}

function text(value: unknown): string | null {
  return typeof value === "string" && value.length > 0 ? value : null;
}

type DispositionItem = { path: string; record: string; item: PlainObject };

/** A model document is the user's private project data (§2), so every
 * disposition it carries is screened as such: private data needs the explicit
 * intent, and a quarantined or rejected disposition blocks in every context. */
const MODEL_DOCUMENT_PRIVACY_CLASS = "private_project_data";

/** The record a `provenance_record` belongs to: its id, else its path. */
function recordName(owner: PlainObject, path: string): string {
  return text(owner.id) ?? path;
}

/** Every disposition the save screens: each `provenance_record` (at any depth),
 * each `rule_pack_refs[]` entry and each resolved library disposition. */
export function modelDocumentDispositionItems(
  model: unknown,
  resolvedLibraries: ReadonlyArray<ResolvedLibraryDisposition> = []
): DispositionItem[] {
  const items: DispositionItem[] = [];
  const walk = (value: unknown, path: string) => {
    if (Array.isArray(value)) {
      value.forEach((entry, index) => walk(entry, `${path}[${index}]`));
      return;
    }
    if (!isPlainObject(value)) return;
    for (const [key, child] of Object.entries(value)) {
      const at = path ? `${path}.${key}` : key;
      if (key === "provenance_record" && isPlainObject(child)) {
        items.push({
          path: at,
          record: recordName(value, path || "$"),
          item: {
            field_class: "provenance_record",
            record_id: recordName(value, path || "$"),
            privacy_classification: MODEL_DOCUMENT_PRIVACY_CLASS,
            redistribution_status: child.redistribution_status,
            review_status: child.review_status
          }
        });
      } else {
        walk(child, at);
      }
    }
  };
  walk(model, "");
  const refs = isPlainObject(model) && Array.isArray(model.rule_pack_refs) ? model.rule_pack_refs : [];
  refs.forEach((ref, index) => {
    if (!isPlainObject(ref)) return;
    const path = `rule_pack_refs[${index}]`;
    const record = text(ref.rule_pack_id) ?? path;
    items.push({
      path,
      record,
      item: { field_class: "rule_pack_ref", record_id: record, privacy_classification: MODEL_DOCUMENT_PRIVACY_CLASS, redistribution_status: ref.redistribution_status }
    });
  });
  resolvedLibraries.forEach((library) => {
    const record = `${library.library_kind}:${library.library_id}${library.record_id ? `#${library.record_id}` : ""}`;
    items.push({
      path: `library_ref(${record})`,
      record,
      item: {
        field_class: "library_ref",
        record_id: record,
        privacy_classification: MODEL_DOCUMENT_PRIVACY_CLASS,
        redistribution_status: library.redistribution_status ?? undefined,
        review_status: library.review_status ?? undefined
      }
    });
  });
  return items;
}

export type DispositionScreening = { decisions: RedactionDecision[]; blocking: ModelFileDiagnostic[] };

/** The screener's decision for every disposition; the blocking ones as named diagnostics. */
export function screenModelDocumentDispositions(
  model: unknown,
  options: { explicitLocalPrivateIntent: boolean; resolvedLibraries?: ReadonlyArray<ResolvedLibraryDisposition> }
): DispositionScreening {
  const decisions: RedactionDecision[] = [];
  const blocking: ModelFileDiagnostic[] = [];
  for (const { path, record, item } of modelDocumentDispositionItems(model, options.resolvedLibraries)) {
    const decision = classifyExportItem(item, {
      exportContext: "local_private",
      explicitLocalPrivateIntent: options.explicitLocalPrivateIntent,
      path
    });
    decisions.push(decision);
    if (decision.action !== "block_export") continue;
    const shown = [
      decision.redistribution_status !== "unknown" ? `redistribution_status ${decision.redistribution_status}` : null,
      decision.review_status !== "unknown" ? `review_status ${decision.review_status}` : null
    ].filter(Boolean).join(", ");
    blocking.push({
      code: decision.reason_code,
      path,
      message: `${record} (${String(item.field_class)}${shown ? `: ${shown}` : ""}) blocks the save; request human review of the record's disposition first.`
    });
  }
  return { decisions, blocking };
}

const LIBRARY_KINDS: ReadonlySet<string> = new Set(["material", "section", "component", "hanger"]);
const LIBRARY_RECORD_ID_KEY: Record<LibraryKind, string> = {
  material: "material_id",
  section: "section_id",
  component: "component_id",
  hanger: "hanger_id"
};

/** The distinct `provenance_record.library_ref`s of a model. */
export function modelLibraryRefs(model: unknown): Array<{ library_kind: LibraryKind; library_id: string; record_id: string | null }> {
  const found = new Map<string, { library_kind: LibraryKind; library_id: string; record_id: string | null }>();
  const walk = (value: unknown) => {
    if (Array.isArray(value)) { value.forEach(walk); return; }
    if (!isPlainObject(value)) return;
    for (const [key, child] of Object.entries(value)) {
      if (key === "provenance_record" && isPlainObject(child) && isPlainObject(child.library_ref)) {
        const ref = child.library_ref;
        const kind = text(ref.library_kind);
        const id = text(ref.library_id);
        if (kind && LIBRARY_KINDS.has(kind) && id) {
          const recordId = text(ref.record_id);
          found.set(JSON.stringify([kind, id, recordId]), { library_kind: kind as LibraryKind, library_id: id, record_id: recordId });
        }
      } else {
        walk(child);
      }
    }
  };
  walk(model);
  return [...found.values()];
}

function disposition(item: unknown, key: "redistribution_status" | "review_status"): string | null {
  if (!isPlainObject(item)) return null;
  // The library import's rule: the item's own field wins, else its provenance's.
  return text(item[key]) ?? (isPlainObject(item.provenance) ? text(item.provenance[key]) : null);
}

/** Resolves each library reference against the local library store: the
 * library's own disposition and, where the record is found, the record's.
 * A library the store does not hold (or a browser session without the store)
 * resolves to nothing; the model's own `provenance_record` disposition is
 * still screened. */
export async function resolveLibraryDispositions(
  model: PreviewModel,
  open: (projectId: string, kind: LibraryKind, id: string) => Promise<LibraryOpenRoute> = openLocalLibrary
): Promise<ResolvedLibraryDisposition[]> {
  const refs = modelLibraryRefs(model);
  if (refs.length === 0) return [];
  const resolved: ResolvedLibraryDisposition[] = [];
  const libraries = new Map<string, Promise<LibraryOpenRoute | null>>();
  for (const ref of refs) {
    const key = JSON.stringify([ref.library_kind, ref.library_id]);
    if (!libraries.has(key)) libraries.set(key, open(model.project.id, ref.library_kind, ref.library_id).catch(() => null));
    const route = await libraries.get(key)!;
    const document = route?.route === "tauri_backend" ? route.envelope?.document : null;
    if (!isPlainObject(document)) continue;
    const library = document[`${ref.library_kind}_library`];
    const libraryProvenance = isPlainObject(library) ? library.provenance : null;
    const libraryEntry: ResolvedLibraryDisposition = {
      library_kind: ref.library_kind,
      library_id: ref.library_id,
      record_id: null,
      redistribution_status: disposition({ provenance: libraryProvenance }, "redistribution_status"),
      review_status: disposition({ provenance: libraryProvenance }, "review_status")
    };
    if (!resolved.some((entry) => entry.record_id === null && entry.library_kind === ref.library_kind && entry.library_id === ref.library_id)) {
      resolved.push(libraryEntry);
    }
    if (!ref.record_id) continue;
    const records = document[`${ref.library_kind}_records`];
    const record = Array.isArray(records)
      ? records.find((entry) => isPlainObject(entry) && entry[LIBRARY_RECORD_ID_KEY[ref.library_kind]] === ref.record_id)
      : undefined;
    if (!record) continue;
    resolved.push({
      library_kind: ref.library_kind,
      library_id: ref.library_id,
      record_id: ref.record_id,
      redistribution_status: disposition(record, "redistribution_status") ?? libraryEntry.redistribution_status,
      review_status: disposition(record, "review_status") ?? libraryEntry.review_status
    });
  }
  return resolved;
}

// Exact decoded-JSON equality: the route projection changed no fact.
function sameJson(left: unknown, right: unknown): boolean {
  if (left === right) return true;
  if (left === null || right === null || typeof left !== "object" || typeof right !== "object") return false;
  if (Array.isArray(left) !== Array.isArray(right)) return false;
  const a = left as PlainObject;
  const b = right as PlainObject;
  const keys = Object.keys(a).filter((key) => a[key] !== undefined);
  const otherKeys = Object.keys(b).filter((key) => b[key] !== undefined);
  return keys.length === otherKeys.length && keys.every((key) => Object.hasOwn(b, key) && sameJson(a[key], b[key]));
}

function refused(code: string, message: string, path = ""): ModelDocumentSavePreparation {
  return { status: "refused", diagnostics: [{ code, path, message }] };
}

/** Builds the native save's arguments, or refuses by name. */
export async function prepareModelDocumentSave(input: {
  model: PreviewModel;
  modelHash: ModelHashEvidence | null;
  explicitLocalPrivateIntent: boolean;
  resolvedLibraries?: ReadonlyArray<ResolvedLibraryDisposition>;
}): Promise<ModelDocumentSavePreparation> {
  const { model, modelHash } = input;
  if (input.explicitLocalPrivateIntent !== true) {
    return refused(MODEL_DOC_SAVE_INTENT_REQUIRED, `Saving the model document needs the explicit choice "${MODEL_DOCUMENT_SAVE_INTENT_LABEL}".`);
  }
  if (!modelHash || modelHash.algorithm !== "sha256" || modelHash.canonicalization !== "rfc8785_jcs"
    || modelHash.payload_scope !== "model_payload" || !/^sha256:[0-9a-f]{64}$/.test(modelHash.value)) {
    return refused(MODEL_DOC_SAVE_HASH_PENDING, "The session's model hash is not computed yet; try again once it is shown.");
  }
  if (model.schema_version !== MODEL_SCHEMA_VERSION) {
    return refused(MODEL_DOC_SAVE_VERSION_NOT_CURRENT, `schema_version is ${String(model.schema_version)}; only model document ${MODEL_SCHEMA_VERSION} is saved.`, "schema_version");
  }
  const shape = modelDocumentShapeDiagnostics(model);
  if (shape.length > 0) return { status: "refused", diagnostics: shape };
  const slots = modelDocumentSlotValidation(model);
  if (slots.blocking.length > 0) return { status: "refused", diagnostics: slots.blocking };
  const dispositions = screenModelDocumentDispositions(model, {
    explicitLocalPrivateIntent: true,
    resolvedLibraries: input.resolvedLibraries
  });
  if (dispositions.blocking.length > 0) return { status: "refused", diagnostics: dispositions.blocking };
  const controlled = controlRouteExport(model, {
    routeId: MODEL_DOCUMENT_SAVE_ROUTE_ID,
    exportContext: "local_private",
    explicitLocalPrivateIntent: true,
    requireLosslessMaterialization: true
  });
  const localFirst = controlled.summary.local_first;
  if (!localFirst || controlled.blocked || controlled.payload === null) {
    const reason = localFirst?.blocked ? localFirst.reason_code : controlled.summary.materialization_withheld
      ? "CANONICAL_PAYLOAD_MATERIALIZATION_WITHHELD" : `${controlled.summary.blocking_count} blocking finding(s)`;
    return refused(MODEL_DOC_SAVE_ROUTE_BLOCKED, `The ${MODEL_DOCUMENT_SAVE_ROUTE_ID} route control blocked the save (${reason}).`);
  }
  const exact = sameJson(controlled.payload, model);
  if (!exact) {
    return refused(MODEL_DOC_SAVE_ROUTE_BLOCKED, `The ${MODEL_DOCUMENT_SAVE_ROUTE_ID} route control changed the document (CANONICAL_PAYLOAD_MATERIALIZATION_CHANGED).`);
  }
  const canonicalText = await canonicalJsonString(model);
  const byteCount = new TextEncoder().encode(canonicalText).length;
  if (byteCount > MAX_MODEL_DOCUMENT_BYTES) {
    return refused(MODEL_DOC_SAVE_TOO_LARGE, `The model document is ${byteCount} bytes; a model document may be at most ${MAX_MODEL_DOCUMENT_BYTES} bytes, so it would not re-open.`);
  }
  return {
    status: "ready",
    decisions: [...dispositions.decisions, ...controlled.decisions],
    args: {
      canonicalText,
      claimedModelHash: modelHash.value,
      screening: {
        route_id: MODEL_DOCUMENT_SAVE_ROUTE_ID,
        export_context: "local_private",
        explicit_local_private_intent: true,
        blocked: false,
        materialization_withheld: controlled.summary.materialization_withheld,
        lossless_required: true,
        exact_payload_match: exact,
        blocking_count: controlled.summary.blocking_count
      },
      localFirstEvidence: localFirst
    }
  };
}

function isDiagnostic(value: unknown): value is ModelFileDiagnostic {
  return isPlainObject(value) && typeof value.code === "string" && typeof value.path === "string" && typeof value.message === "string";
}

function readOutcome(value: unknown): ModelDocumentSaveOutcome {
  if (isPlainObject(value)) {
    if (value.outcome === "cancelled") return { outcome: "cancelled" };
    if (value.outcome === "refused" && Array.isArray(value.diagnostics) && value.diagnostics.every(isDiagnostic)) {
      return { outcome: "refused", diagnostics: value.diagnostics };
    }
    if (value.outcome === "saved" && typeof value.file_name === "string" && typeof value.byte_count === "number"
      && typeof value.model_hash === "string" && typeof value.replaced_existing === "boolean" && typeof value.durability === "string") {
      return {
        outcome: "saved",
        file_name: value.file_name,
        byte_count: value.byte_count,
        model_hash: value.model_hash,
        replaced_existing: value.replaced_existing,
        durability: value.durability
      };
    }
  }
  return { outcome: "refused", diagnostics: [{ code: MODEL_DOC_SAVE_RECEIPT_INVALID, path: "", message: "The native save returned an unreadable outcome." }] };
}

/** The native save: admission, the chooser (documents folder) and the atomic `.json` write. */
export async function saveModelDocumentFile(args: ModelDocumentSaveArgs): Promise<ModelDocumentSaveOutcome> {
  if (!isTauriRuntime()) {
    return { outcome: "refused", diagnostics: [{ code: MODEL_DOC_SAVE_DESKTOP_ONLY, path: "", message: "Saving a model document file needs the desktop app." }] };
  }
  const outcome = readOutcome(await invokeWithIngress<unknown>("save_model_document_file", args));
  if (outcome.outcome === "saved" && outcome.model_hash !== args.claimedModelHash) {
    return { outcome: "refused", diagnostics: [{ code: MODEL_DOC_SAVE_RECEIPT_INVALID, path: "", message: `The saved file's hash ${outcome.model_hash} is not the session's model hash ${args.claimedModelHash}; do not rely on ${outcome.file_name}.` }] };
  }
  return outcome;
}

/** One status line naming every refusal diagnostic. */
export function modelDocumentSaveRefusalMessage(diagnostics: ReadonlyArray<ModelFileDiagnostic>): string {
  const shown = diagnostics.slice(0, 8).map((item) => `${item.code}${item.path ? ` at ${item.path}` : ""}: ${item.message}`);
  const more = diagnostics.length > shown.length ? ` (+${diagnostics.length - shown.length} more)` : "";
  return `Save model document refused: ${shown.join(" | ")}${more}`;
}

/** The status line for a native outcome. */
export function modelDocumentSaveMessage(outcome: ModelDocumentSaveOutcome): string {
  if (outcome.outcome === "cancelled") return "Save model document cancelled; nothing was written.";
  if (outcome.outcome === "refused") return modelDocumentSaveRefusalMessage(outcome.diagnostics);
  const durability = outcome.durability === "durable" ? "" : " The file was saved, but its directory sync did not complete; its durability is uncertain.";
  return `Saved model document ${outcome.file_name} (${outcome.byte_count} bytes; model hash ${outcome.model_hash}).${durability}`;
}

/** The status line for a failed save (an invoke error, a memory-pool refusal). */
export function modelDocumentSaveFailureMessage(error: unknown): string {
  return `Save model document failed: ${nativeFailureText(error)}`;
}
