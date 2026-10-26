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
// are screened explicitly here: every `provenance_record` at any depth, every
// `rule_pack_refs[]` entry and every `library_ref` resolved against the local
// store, with the library import's policy (own field, else provenance;
// `protected_suspected`, `quarantined` and `rejected` in either field) and the
// screener's own classifier (`classifyExportItem`). A blocked disposition
// refuses with the screener's code, naming the record; `private_only` and
// `unknown` save. A store read error refuses; a library the store does not
// hold is a `MODEL-LIBRARY-REF-UNRESOLVED` warning. The native save repeats
// all of this on the exact bytes it admits and never trusts this evidence.
//
// Nothing here transmits anything or records telemetry (F-PIP-1). A save never
// changes the session's model, hash or solve standing.
import { canonicalJsonString, canonicalSha256Hex } from "../../services/hashService";
import { invokeWithIngress, nativeFailureText } from "../../services/nativeIngress";
import { isTauriRuntime } from "../../services/nativeMenu";
import { trimLikeRust } from "../../services/rustTrim";
import { MODEL_SCHEMA_VERSION } from "../../services/modelSchemaVersion";
import { MODEL_LIBRARY_REF_UNRESOLVED, libraryRefSites, openLocalLibrary, type LibraryKind, type LibraryOpenRoute } from "../../services/libraryImportService";
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
  /** The `library_ref`'s path in the document. */
  path?: string;
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
  /** `sha256:<hex>` of the canonical text screened; the native save refuses other bytes. */
  payload_sha256: string;
};

/** The native command's arguments (`save_model_document_file`). */
export type ModelDocumentSaveArgs = {
  canonicalText: string;
  claimedModelHash: string;
  screening: ModelDocumentSaveScreening;
  localFirstEvidence: LocalFirstRouteEvidence;
};

export type ModelDocumentSavePreparation =
  | { status: "ready"; args: ModelDocumentSaveArgs; decisions: RedactionDecision[]; warnings: ModelFileDiagnostic[] }
  | { status: "refused"; diagnostics: ModelFileDiagnostic[] };

export type ModelDocumentSaveOutcome =
  | { outcome: "cancelled" }
  | { outcome: "saved"; file_name: string; byte_count: number; model_hash: string; replaced_existing: boolean; durability: string; warnings: ModelFileDiagnostic[] }
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

/** The library import's status resolution
 * (`library_import_document/src/lib.rs:297-372`): the item's own field wins
 * when it is a non-empty string, otherwise its `provenance`'s. */
function ownOrProvenance(item: unknown, key: "redistribution_status" | "review_status"): string | null {
  if (!isPlainObject(item)) return null;
  return text(item[key]) ?? (isPlainObject(item.provenance) ? text(item.provenance[key]) : null);
}

/** The dispositions the library import quarantines (`protected_suspected`,
 * `quarantined`) or rejects (`rejected`, in either field). */
function importPolicyBlocks(redistribution: string | null, review: string | null): string[] {
  const found: string[] = [];
  if (redistribution === "protected_suspected" || redistribution === "rejected") found.push(`redistribution_status ${redistribution}`);
  if (review === "quarantined" || review === "rejected") found.push(`review_status ${review}`);
  return found;
}

/** Every disposition the save screens: each `provenance_record` at any depth
 * (including inside another), each `rule_pack_refs[]` entry and each resolved
 * library disposition. The native save screens the same set
 * (`src-tauri/src/model_document_save.rs`). */
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
        const record = recordName(value, path || "$");
        items.push({
          path: at,
          record,
          item: {
            field_class: "provenance_record",
            record_id: record,
            privacy_classification: MODEL_DOCUMENT_PRIVACY_CLASS,
            redistribution_status: ownOrProvenance(child, "redistribution_status") ?? undefined,
            review_status: ownOrProvenance(child, "review_status") ?? undefined
          }
        });
      } else if (key === "rule_pack_refs" && Array.isArray(child)) {
        child.forEach((ref, index) => {
          if (!isPlainObject(ref)) return;
          const refAt = `${at}[${index}]`;
          const record = text(ref.rule_pack_id) ?? refAt;
          items.push({
            path: refAt,
            record,
            item: {
              field_class: "rule_pack_ref",
              record_id: record,
              privacy_classification: MODEL_DOCUMENT_PRIVACY_CLASS,
              redistribution_status: ownOrProvenance(ref, "redistribution_status") ?? undefined,
              review_status: ownOrProvenance(ref, "review_status") ?? undefined
            }
          });
        });
      }
      // The walk continues below every screened record.
      walk(child, at);
    }
  };
  walk(model, "");
  resolvedLibraries.forEach((library) => {
    const record = `${library.library_kind}:${library.library_id}${library.record_id ? `#${library.record_id}` : ""}`;
    items.push({
      path: library.path ?? `library_ref(${record})`,
      record,
      item: {
        field_class: library.record_id ? "stored library record" : "stored library",
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

/** The library import's policy, then the screener's decision, for every
 * disposition; the blocking ones as named diagnostics with the screener's code. */
export function screenModelDocumentDispositions(
  model: unknown,
  options: { explicitLocalPrivateIntent: boolean; resolvedLibraries?: ReadonlyArray<ResolvedLibraryDisposition> }
): DispositionScreening {
  const decisions: RedactionDecision[] = [];
  const blocking: ModelFileDiagnostic[] = [];
  const named = (record: string, item: PlainObject, shown: string) =>
    `${record} (${String(item.field_class)}${shown ? `: ${shown}` : ""}) blocks the save; request human review of the record's disposition first.`;
  for (const { path, record, item } of modelDocumentDispositionItems(model, options.resolvedLibraries)) {
    const decision = classifyExportItem(item, {
      exportContext: "local_private",
      explicitLocalPrivateIntent: options.explicitLocalPrivateIntent,
      path
    });
    decisions.push(decision);
    const policy = importPolicyBlocks(text(item.redistribution_status), text(item.review_status));
    if (policy.length > 0) {
      blocking.push({ code: "PROTECTED_CONTENT_BLOCKED", path, message: named(record, item, policy.join(", ")) });
      continue;
    }
    if (decision.action !== "block_export") continue;
    const shown = [
      decision.redistribution_status !== "unknown" ? `redistribution_status ${decision.redistribution_status}` : null,
      decision.review_status !== "unknown" ? `review_status ${decision.review_status}` : null
    ].filter(Boolean).join(", ");
    blocking.push({ code: decision.reason_code, path, message: named(record, item, shown) });
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

export type ModelLibraryRef = { path: string; library_kind: LibraryKind; library_id: string; record_id: string | null };

/** Every `library_ref` of a model, at any depth, with its path: the walk the
 * open-time store check uses (`libraryRefSites`). */
export function modelLibraryRefs(model: unknown): ModelLibraryRef[] {
  return libraryRefSites(model).flatMap(({ path, reference }) => {
    // Identities as the store's open path normalizes them (`open_local_library` trims).
    const kind = trimmed(reference.library_kind);
    const id = trimmed(reference.library_id);
    return kind && LIBRARY_KINDS.has(kind) && id
      ? [{ path, library_kind: kind as LibraryKind, library_id: id, record_id: trimmed(reference.record_id) }]
      : [];
  });
}

/** A library identity as the native store path trims it (Rust's rule, not `trim()`). */
function trimmed(value: unknown): string | null {
  return text(typeof value === "string" ? trimLikeRust(value) : value);
}

/** The library import's verdict inputs for one stored library object or record:
 * every `{redistribution_status, review_status}` pair it judges. Material,
 * section and component libraries judge the object itself (own field, else
 * provenance). Hanger libraries (`library_import_document/src/lib.rs:797-829`)
 * judge every object in it that carries a provenance or a disposition member,
 * and the provenance alone where an own status differs from it. The native
 * port is `stored_item_blocking`; both are pinned by the parity table
 * `fixtures/model_document/library_disposition_parity.json`. */
export function storedDispositions(kind: string, item: PlainObject): Array<{ redistribution_status: string | null; review_status: string | null }> {
  const pair = (object: unknown) => ({
    redistribution_status: ownOrProvenance(object, "redistribution_status"),
    review_status: ownOrProvenance(object, "review_status")
  });
  if (kind !== "hanger") return [pair(item)];
  const pairs: Array<{ redistribution_status: string | null; review_status: string | null }> = [];
  const visit = (value: unknown, path: string) => {
    if (Array.isArray(value)) { value.forEach((entry, index) => visit(entry, `${path}[${index}]`)); return; }
    if (!isPlainObject(value)) return;
    const judged = "provenance" in value
      || (!path.endsWith(".provenance") && ["redistribution_status", "review_status", "privacy_class"].some((key) => key in value));
    if (judged) {
      pairs.push(pair(value));
      const provenance = value.provenance;
      if (isPlainObject(provenance) && (["redistribution_status", "review_status"] as const)
        .some((key) => key in value && !sameJson(value[key], provenance[key]))) {
        pairs.push(pair({ provenance }));
      }
    }
    for (const [key, child] of Object.entries(value)) visit(child, `${path}.${key}`);
  };
  visit(item, "$");
  return pairs;
}

export const MODEL_DOC_SAVE_LIBRARY_STORE_UNAVAILABLE = "MODEL-DOC-SAVE-LIBRARY-STORE-UNAVAILABLE";
// The store check's code (`libraryImportService` / `check_model_library_refs`, W-13 §3.5).
export { MODEL_LIBRARY_REF_UNRESOLVED };

/** The library references resolved against the local store. `storeError` is
 * set when a lookup failed: the save then refuses. A library or record the
 * store does not hold is no disposition; it is listed in `warnings`. */
export type LibraryResolution = {
  dispositions: ResolvedLibraryDisposition[];
  warnings: ModelFileDiagnostic[];
  storeError: ModelFileDiagnostic | null;
};

export const NO_LIBRARY_REFERENCES: LibraryResolution = { dispositions: [], warnings: [], storeError: null };

/** Resolves each library reference against the local library store with the
 * library import's policy: the library's own disposition and, where the
 * record is found, the record's (each its own field, else its provenance's).
 * The model's own `provenance_record` dispositions are screened regardless. */
export async function resolveLibraryDispositions(
  model: PreviewModel,
  open: (projectId: string, kind: LibraryKind, id: string) => Promise<LibraryOpenRoute> = openLocalLibrary
): Promise<LibraryResolution> {
  const refs = modelLibraryRefs(model);
  if (refs.length === 0) return NO_LIBRARY_REFERENCES;
  const dispositions: ResolvedLibraryDisposition[] = [];
  const warnings: ModelFileDiagnostic[] = [];
  const libraries = new Map<string, Promise<{ route: LibraryOpenRoute } | { error: unknown }>>();
  const screened = new Set<string>();
  for (const ref of refs) {
    const key = JSON.stringify([ref.library_kind, ref.library_id]);
    const named = `${ref.library_kind}:${ref.library_id}`;
    if (!libraries.has(key)) {
      libraries.set(key, open(model.project.id, ref.library_kind, ref.library_id).then((route) => ({ route }), (error: unknown) => ({ error })));
    }
    const looked = await libraries.get(key)!;
    if ("error" in looked) {
      return {
        dispositions,
        warnings,
        storeError: { code: MODEL_DOC_SAVE_LIBRARY_STORE_UNAVAILABLE, path: ref.path, message: `The local library store could not be read to resolve ${named} (${String(looked.error)}); nothing was saved.` }
      };
    }
    const document = looked.route.route === "tauri_backend" ? looked.route.envelope?.document : null;
    if (!isPlainObject(document)) {
      warnings.push({ code: MODEL_LIBRARY_REF_UNRESOLVED, path: ref.path, message: `The local library store holds no ${ref.library_kind} library \`${ref.library_id}\`; its disposition is not known here.` });
      continue;
    }
    const library = document[`${ref.library_kind}_library`];
    if (!screened.has(key) && isPlainObject(library)) {
      for (const pair of storedDispositions(ref.library_kind, library)) {
        dispositions.push({ path: ref.path, library_kind: ref.library_kind, library_id: ref.library_id, record_id: null, ...pair });
      }
    }
    screened.add(key);
    if (!ref.record_id || screened.has(`${key}#${ref.record_id}`)) continue;
    screened.add(`${key}#${ref.record_id}`);
    // Every record with this id is screened (a clear first record never hides a
    // blocked duplicate); ids compare trimmed, so a padded id never escapes.
    const records = document[`${ref.library_kind}_records`];
    const matching = (Array.isArray(records) ? records : [])
      .filter((entry): entry is PlainObject => isPlainObject(entry) && trimmed(entry[LIBRARY_RECORD_ID_KEY[ref.library_kind]]) === ref.record_id);
    if (matching.length === 0) {
      warnings.push({ code: MODEL_LIBRARY_REF_UNRESOLVED, path: ref.path, message: `The stored ${ref.library_kind} library \`${ref.library_id}\` holds no record \`${ref.record_id}\`; its disposition is not known here.` });
      continue;
    }
    for (const record of matching) {
      for (const pair of storedDispositions(ref.library_kind, record)) {
        dispositions.push({ path: ref.path, library_kind: ref.library_kind, library_id: ref.library_id, record_id: ref.record_id, ...pair });
      }
    }
  }
  return { dispositions, warnings, storeError: null };
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
  libraries?: LibraryResolution;
  /** The governed local-private route of the save: `DOTH-MODEL-DOC-007` by
   * default; File > Save Handoff… (W-6b) admits the same bytes under
   * `DOTH-HANDOFF-PACKAGE-008`, with its own intent label. */
  route?: { routeId: string; intentLabel: string };
}): Promise<ModelDocumentSavePreparation> {
  const { model, modelHash } = input;
  const routeId = input.route?.routeId ?? MODEL_DOCUMENT_SAVE_ROUTE_ID;
  if (input.explicitLocalPrivateIntent !== true) {
    return refused(MODEL_DOC_SAVE_INTENT_REQUIRED, `Saving the model document needs the explicit choice "${input.route?.intentLabel ?? MODEL_DOCUMENT_SAVE_INTENT_LABEL}".`);
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
  const libraries = input.libraries ?? NO_LIBRARY_REFERENCES;
  if (libraries.storeError) return { status: "refused", diagnostics: [libraries.storeError] };
  const dispositions = screenModelDocumentDispositions(model, {
    explicitLocalPrivateIntent: true,
    resolvedLibraries: libraries.dispositions
  });
  if (dispositions.blocking.length > 0) return { status: "refused", diagnostics: dispositions.blocking };
  const controlled = controlRouteExport(model, {
    routeId,
    exportContext: "local_private",
    explicitLocalPrivateIntent: true,
    requireLosslessMaterialization: true
  });
  const localFirst = controlled.summary.local_first;
  if (!localFirst || controlled.blocked || controlled.payload === null) {
    const reason = localFirst?.blocked ? localFirst.reason_code : controlled.summary.materialization_withheld
      ? "CANONICAL_PAYLOAD_MATERIALIZATION_WITHHELD" : `${controlled.summary.blocking_count} blocking finding(s)`;
    return refused(MODEL_DOC_SAVE_ROUTE_BLOCKED, `The ${routeId} route control blocked the save (${reason}).`);
  }
  const exact = sameJson(controlled.payload, model);
  if (!exact) {
    return refused(MODEL_DOC_SAVE_ROUTE_BLOCKED, `The ${routeId} route control changed the document (CANONICAL_PAYLOAD_MATERIALIZATION_CHANGED).`);
  }
  const canonicalText = await canonicalJsonString(model);
  // The hash of the very document screened, which is the hash of `canonicalText`.
  const payloadSha256 = `sha256:${await canonicalSha256Hex(model)}`;
  const byteCount = new TextEncoder().encode(canonicalText).length;
  if (byteCount > MAX_MODEL_DOCUMENT_BYTES) {
    return refused(MODEL_DOC_SAVE_TOO_LARGE, `The model document is ${byteCount} bytes; a model document may be at most ${MAX_MODEL_DOCUMENT_BYTES} bytes, so it would not re-open.`);
  }
  return {
    status: "ready",
    decisions: [...dispositions.decisions, ...controlled.decisions],
    warnings: libraries.warnings,
    args: {
      canonicalText,
      claimedModelHash: modelHash.value,
      screening: {
        route_id: routeId,
        export_context: "local_private",
        explicit_local_private_intent: true,
        blocked: false,
        materialization_withheld: controlled.summary.materialization_withheld,
        lossless_required: true,
        exact_payload_match: exact,
        blocking_count: controlled.summary.blocking_count,
        payload_sha256: payloadSha256
      },
      localFirstEvidence: localFirst
    }
  };
}

function isDiagnostic(value: unknown): value is ModelFileDiagnostic {
  return isPlainObject(value) && typeof value.code === "string" && typeof value.path === "string" && typeof value.message === "string";
}

/** The native save's outcome, checked field by field (also the recovery export's, `storedDocumentExport.ts`). */
export function readOutcome(value: unknown): ModelDocumentSaveOutcome {
  if (isPlainObject(value)) {
    if (value.outcome === "cancelled") return { outcome: "cancelled" };
    if (value.outcome === "refused" && Array.isArray(value.diagnostics) && value.diagnostics.every(isDiagnostic)) {
      return { outcome: "refused", diagnostics: value.diagnostics };
    }
    if (value.outcome === "saved" && typeof value.file_name === "string" && typeof value.byte_count === "number"
      && typeof value.model_hash === "string" && typeof value.replaced_existing === "boolean" && typeof value.durability === "string"
      && Array.isArray(value.warnings) && value.warnings.every(isDiagnostic)) {
      return {
        outcome: "saved",
        file_name: value.file_name,
        byte_count: value.byte_count,
        model_hash: value.model_hash,
        replaced_existing: value.replaced_existing,
        durability: value.durability,
        warnings: value.warnings
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
  const shown = outcome.warnings.slice(0, 4).map((item) => `${item.code}${item.path ? ` at ${item.path}` : ""}`);
  const more = outcome.warnings.length > shown.length ? ` (+${outcome.warnings.length - shown.length} more)` : "";
  const warnings = shown.length > 0 ? ` Warnings: ${shown.join(" | ")}${more}.` : "";
  return `Saved model document ${outcome.file_name} (${outcome.byte_count} bytes; model hash ${outcome.model_hash}).${durability}${warnings}`;
}

/** The status line for a failed save (an invoke error, a memory-pool refusal). */
export function modelDocumentSaveFailureMessage(error: unknown): string {
  return `Save model document failed: ${nativeFailureText(error)}`;
}
