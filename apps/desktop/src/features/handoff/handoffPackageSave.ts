// File > Save Handoff… (DEL-15-01 Design `canonical-model-export.md` §3, W-6b;
// FR-HAND-001/002, PRD §16.1). The package is a directory `<name>.swbhandoff/`
// holding the W-6a canonical model document (`model/model.json`) and a
// `schemas/handoff_package.schema.json` manifest (`handoff_package.json`) that
// binds it by SHA-256. The native command builds the manifest from the bytes it
// admits and the local store, publishes the package atomically and never trusts
// this webview's evidence for content (`src-tauri/src/handoff_package.rs`).
//
// The webview's part:
// - the explicit local-private intent (the dialog's own control) and the
//   package label;
// - the basis state: a saved state of the project whose model hash is the
//   session's is the basis; otherwise the user confirms a new state, kind
//   `handoff_basis`, default name "Handoff <date>", saved through W-12 S1's
//   store (`saveModelState`) before the package is published;
// - W-6a's admission and screening of the model bytes, under the governed route
//   `DOTH-HANDOFF-PACKAGE-008` (local-private, lossless, exact payload).
//
// Nothing here transmits anything or records telemetry (F-PIP-1). A save never
// changes the session's model, hash or solve standing.
import { invokeWithIngress, nativeFailureText } from "../../services/nativeIngress";
import { isTauriRuntime } from "../../services/nativeMenu";
import type { ModelStateSummary } from "../../services/modelStateService";
import type { ModelDocumentSaveArgs, ModelFileDiagnostic } from "../model-file/modelDocumentSave";
import { listedItems } from "../model-file/diagnosticList";

export const HANDOFF_PACKAGE_ROUTE_ID = "DOTH-HANDOFF-PACKAGE-008";
export const HANDOFF_SAVE_INTENT_LABEL =
  "Save my handoff package, including private project data, to a folder I choose";
export const HANDOFF_NEW_STATE_LABEL = "First save the current model as a new handoff-basis model state";
export const HANDOFF_SAVE_ROUTE = { routeId: HANDOFF_PACKAGE_ROUTE_ID, intentLabel: HANDOFF_SAVE_INTENT_LABEL } as const;

export const HANDOFF_SAVE_DESKTOP_ONLY = "HANDOFF-SAVE-DESKTOP-ONLY";
export const HANDOFF_SAVE_RECEIPT_INVALID = "HANDOFF-SAVE-RECEIPT-INVALID";
export const HANDOFF_STATE_NAME_REQUIRED = "HANDOFF-STATE-NAME-REQUIRED";
export const HANDOFF_LABEL_REQUIRED = "HANDOFF-LABEL-REQUIRED";

/** The native command's arguments (`save_handoff_package`). */
export type HandoffSaveArgs = ModelDocumentSaveArgs & { basisStateId: string; packageLabel: string };

export type HandoffSaveOutcome =
  | { outcome: "cancelled" }
  | {
      outcome: "saved";
      package_name: string;
      model_hash: string;
      export_hash: string;
      basis_state_id: string;
      replaced_existing: boolean;
      durability: string;
      warnings: ModelFileDiagnostic[];
    }
  | { outcome: "refused"; diagnostics: ModelFileDiagnostic[] };

/** The basis the dialog shows: a saved state of this model, or a new one to confirm. */
export type HandoffBasis =
  | { kind: "existing"; state: ModelStateSummary }
  | { kind: "new"; defaultName: string };

/** "Handoff <local date>", e.g. "Handoff 2026-10-24". */
export function defaultHandoffStateName(now: Date = new Date()): string {
  const pad = (value: number) => String(value).padStart(2, "0");
  return `Handoff ${now.getFullYear()}-${pad(now.getMonth() + 1)}-${pad(now.getDate())}`;
}

/** The saved state of the project whose model hash is the session's: the basis.
 * A project holds at most one state per model hash (W-12 S1). */
export function findBasisState(states: ReadonlyArray<ModelStateSummary>, projectId: string, modelHash: string): ModelStateSummary | null {
  return states.find((state) => state.project_id === projectId && state.model_hash === modelHash) ?? null;
}

export function handoffBasis(states: ReadonlyArray<ModelStateSummary>, projectId: string, modelHash: string, now?: Date): HandoffBasis {
  const state = findBasisState(states, projectId, modelHash);
  return state ? { kind: "existing", state } : { kind: "new", defaultName: defaultHandoffStateName(now) };
}

type PlainObject = Record<string, unknown>;

function isPlainObject(value: unknown): value is PlainObject {
  return typeof value === "object" && value !== null && !Array.isArray(value);
}

function isDiagnostic(value: unknown): value is ModelFileDiagnostic {
  return isPlainObject(value) && typeof value.code === "string" && typeof value.path === "string" && typeof value.message === "string";
}

const SHA256 = /^sha256:[0-9a-f]{64}$/;

/** The native outcome, read strictly: anything else is a named refusal. */
export function readHandoffOutcome(value: unknown): HandoffSaveOutcome {
  if (isPlainObject(value)) {
    if (value.outcome === "cancelled") return { outcome: "cancelled" };
    if (value.outcome === "refused" && Array.isArray(value.diagnostics) && value.diagnostics.every(isDiagnostic)) {
      return { outcome: "refused", diagnostics: value.diagnostics };
    }
    if (value.outcome === "saved" && typeof value.package_name === "string" && typeof value.model_hash === "string" && SHA256.test(value.model_hash)
      && typeof value.export_hash === "string" && SHA256.test(value.export_hash) && typeof value.basis_state_id === "string"
      && typeof value.replaced_existing === "boolean" && typeof value.durability === "string"
      && Array.isArray(value.warnings) && value.warnings.every(isDiagnostic)) {
      return {
        outcome: "saved",
        package_name: value.package_name,
        model_hash: value.model_hash,
        export_hash: value.export_hash,
        basis_state_id: value.basis_state_id,
        replaced_existing: value.replaced_existing,
        durability: value.durability,
        warnings: value.warnings
      };
    }
  }
  return { outcome: "refused", diagnostics: [{ code: HANDOFF_SAVE_RECEIPT_INVALID, path: "", message: "The native save returned an unreadable outcome." }] };
}

/** The native save: admission, the chooser (documents folder) and the atomic publication. */
export async function saveHandoffPackage(args: HandoffSaveArgs): Promise<HandoffSaveOutcome> {
  if (!isTauriRuntime()) {
    return { outcome: "refused", diagnostics: [{ code: HANDOFF_SAVE_DESKTOP_ONLY, path: "", message: "Saving a handoff package needs the desktop app." }] };
  }
  const outcome = readHandoffOutcome(await invokeWithIngress<unknown>("save_handoff_package", args));
  if (outcome.outcome === "saved" && (outcome.model_hash !== args.claimedModelHash || outcome.basis_state_id !== args.basisStateId)) {
    return { outcome: "refused", diagnostics: [{ code: HANDOFF_SAVE_RECEIPT_INVALID, path: "", message: `The saved package ${outcome.package_name} does not name the session's model hash and basis state; do not rely on it.` }] };
  }
  return outcome;
}

function listed(diagnostics: ReadonlyArray<ModelFileDiagnostic>, limit: number, withMessage: boolean): string {
  return listedItems(diagnostics, limit, withMessage);
}

export function handoffSaveRefusalMessage(diagnostics: ReadonlyArray<ModelFileDiagnostic>): string {
  return `Save handoff refused: ${listed(diagnostics, 8, true)}`;
}

/** The status line for a native outcome; `basisNote` names a state saved for this handoff. */
export function handoffSaveMessage(outcome: HandoffSaveOutcome, basisNote = ""): string {
  const note = basisNote ? ` ${basisNote}` : "";
  if (outcome.outcome === "cancelled") return `Save handoff cancelled; nothing was written.${note}`;
  if (outcome.outcome === "refused") return `${handoffSaveRefusalMessage(outcome.diagnostics)}${note}`;
  const durability = outcome.durability === "durable" ? "" : " The package was saved, but its folder sync did not complete; its durability is uncertain.";
  const warnings = outcome.warnings.length > 0 ? ` Warnings: ${listed(outcome.warnings, 4, false)}.` : "";
  return `Saved handoff package ${outcome.package_name} (model hash ${outcome.model_hash}; export hash ${outcome.export_hash}; basis state ${outcome.basis_state_id}).${durability}${warnings}${note}`;
}

export function handoffSaveFailureMessage(error: unknown): string {
  return `Save handoff failed: ${nativeFailureText(error)}`;
}
