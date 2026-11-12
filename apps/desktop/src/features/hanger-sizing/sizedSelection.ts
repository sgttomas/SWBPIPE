/** Applying a sized selection (DEL-04-11 REQ-008, AX-002 "Results, not edits"). Sizing never writes the model: this
 * module only says which of a shown result's selections may be offered for application and with which sized values,
 * and the person's act prepares a batch through the one hanger-selection builder (`buildHangerSelectionBatch`), which
 * reaches the model only through the operation route on the person's acceptance.
 *
 * A selection is offered only while the shown result is bound to the present model: the solve request assembled from
 * the present model (`hangerSizingSolveRequest`, the same assembly the sizing request used, sent as its JSON text) has
 * the checked hash the result binds (`inputs.solve_request.sha256`). The record is the one the result binds: found by
 * `hanger_id` in the library document the request sent, whose checked hash is `inputs.library.sha256`, with its own
 * checked hash equal to the selection's `record_sha256`. */

import { canonicalSha256HexCheckedV1 } from "../../services/hashService";
import type { PreviewModel } from "../../types";
import type { HangerDocument, HangerSizingApplication, HangerTarget } from "../hanger-selection/hangerSelection";
import { HANGER_LOAD_NOT_APPLIED_AS_PRELOAD } from "../results/hangerPreload";
import type { HangerSizingCandidate, HangerSizingSizedDocument } from "../results/hangerSizingResult";
import { hangerSizingSolveRequest } from "./hangerSizingRequest";

export const HANGER_SIZING_APPLY_TITLE = "Apply a sized selection";
/** REQ-013: what applying is, in the claims registry's vocabulary. */
export const HANGER_SIZING_APPLY_TEXT =
  "Applying writes into the support the record that your own tests selected from your own hanger library, with the loads this run sized. " +
  "It is queued for your review and changes the model only when you accept it there. It judges no hanger's suitability for a service and determines no code compliance.";
export const HANGER_SIZING_MODEL_CHANGED_TEXT =
  "The model has changed since this sizing run, so its selections are not offered for application. Size again to apply a selection.";
export const HANGER_SIZING_MODEL_NOT_HASHABLE_TEXT =
  "The present model's solve request cannot be hashed, so this result cannot be shown to be bound to it and its selections are not offered for application.";
export const HANGER_SIZING_LIBRARY_CHANGED_TEXT =
  "The hanger library has changed since this sizing run, so its selections are not offered for application. Size again to apply a selection.";
export const HANGER_SIZING_LIBRARY_UNAVAILABLE_TEXT =
  "The hanger library this sizing run used is no longer stored or cannot be read, so its selections are not offered for application. Size again to apply a selection.";
/** Beside a selection of a run with two or more selections (DEL-04-11 REQ-008): the run's values are applied whole. */
export const HANGER_SIZING_APPLIED_WITH_RUN_TEXT =
  "This run sized its supports together, so this selection is applied only with the run's other selections, by the act below.";
export const HANGER_SIZING_NO_FIT_TEXT = "No record fits this support, so there is nothing to apply.";
/** Shown beside an application that includes a variable spring, until the solve applies a variable spring's cold load
 * (DEL-04-11 REQ-014); kept consistent with the results standing line (`results/hangerPreload.ts`). Remove this
 * constant, and its two uses in `HangerSizingApply.tsx`, when the preload lands. */
export const HANGER_SIZING_PRELOAD_PENDING_TEXT =
  "Until the solve applies a variable spring's cold load, a solve of the applied model carries the hanger's rate and not its sized cold load, " +
  `so it does not reproduce this run's verified positions at that hanger; the solve warns of this by name (${HANGER_LOAD_NOT_APPLIED_AS_PRELOAD}).`;

export type ModelBinding = "current" | "changed" | "not_hashable";

export type ApplyOffer =
  | Readonly<{ kind: "offered"; support: string; hangerId: string; target: HangerTarget; application: HangerSizingApplication }>
  | Readonly<{ kind: "not_offered"; support: string; reason: string }>;

export type ApplyOffers = Readonly<{ model: ModelBinding; offers: readonly ApplyOffer[] }>;

const isPlainObject = (value: unknown): value is Record<string, unknown> =>
  typeof value === "object" && value !== null && !Array.isArray(value);

async function checkedHash(value: unknown): Promise<string | null> {
  try {
    return await canonicalSha256HexCheckedV1(value);
  } catch {
    return null;
  }
}

/** Whether `document` is bound to `model`: the checked hash of the solve request assembled from it, as its JSON text is
 * sent, against the hash the result binds. */
export async function hangerSizingModelBinding(document: HangerSizingSizedDocument, model: PreviewModel): Promise<ModelBinding> {
  let sent: unknown;
  try {
    sent = JSON.parse(JSON.stringify(hangerSizingSolveRequest(model)));
  } catch {
    return "not_hashable";
  }
  const sha256 = await checkedHash(sent);
  if (sha256 === null) return "not_hashable";
  return sha256 === document.inputs.solve_request.sha256 ? "current" : "changed";
}

async function libraryReason(document: HangerSizingSizedDocument, library: unknown): Promise<string | null> {
  if (!isPlainObject(library)) return "The library this run used is not available, so no selection is offered.";
  if ((await checkedHash(library)) !== document.inputs.library.sha256) return "The library document does not have the hash this run bound, so no selection is offered.";
  const header = library.hanger_library;
  if (!isPlainObject(header) || header.library_id !== document.inputs.library.library_id) return "The library document is not the one this run named, so no selection is offered.";
  return null;
}

async function offerFor(document: HangerSizingSizedDocument, candidate: HangerSizingCandidate, library: Record<string, unknown>): Promise<ApplyOffer> {
  const support = candidate.support;
  if (candidate.outcome === "no_record_fits") return { kind: "not_offered", support, reason: HANGER_SIZING_NO_FIT_TEXT };
  const selected = candidate.selected;
  const records = Array.isArray(library.hanger_records) ? library.hanger_records : [];
  const matches = records.filter((record) => isPlainObject(record) && record.hanger_id === selected.hanger_id);
  if (matches.length !== 1) return { kind: "not_offered", support, reason: `Record ${selected.hanger_id} is not found once in the library this run used, so it is not offered.` };
  const record = matches[0] as Record<string, unknown>;
  if ((await checkedHash(record)) !== selected.record_sha256) return { kind: "not_offered", support, reason: `Record ${selected.hanger_id} does not have the hash this run bound, so it is not offered.` };
  const hanger = isPlainObject(record.hanger) ? record.hanger : {};
  const stiffness = isPlainObject(hanger.stiffness) ? hanger.stiffness : null;
  const stiffnessDof = typeof stiffness?.dof === "string" ? stiffness.dof.toUpperCase() : null;
  if (selected.family === "variable_spring" && stiffnessDof === null)
    return { kind: "not_offered", support, reason: `Record ${selected.hanger_id} declares no stiffness direction, so it is not offered.` };
  if (stiffnessDof !== null && stiffnessDof !== candidate.dof)
    return { kind: "not_offered", support, reason: `Record ${selected.hanger_id} declares its stiffness on ${stiffnessDof}, but ${support} was sized on ${candidate.dof}, so it is not offered.` };
  const run = {
    sizingRequestSha256: document.inputs.sizing_request_sha256,
    solveRequestSha256: document.inputs.solve_request.sha256,
    libraryId: document.inputs.library.library_id,
    librarySha256: document.inputs.library.sha256,
    hangerId: selected.hanger_id,
    recordSha256: selected.record_sha256
  };
  const target: HangerTarget = { supportId: support, restraints: [candidate.dof] };
  if (selected.family === "constant_effort") {
    return { kind: "offered", support, hangerId: selected.hanger_id, target, application: { run, loads: { family: "constant_effort", constantLoadN: selected.load_n } } };
  }
  const designCase = document.inputs.sizing_request.design_operating_case;
  const design = selected.operating.filter((entry) => entry.case === designCase);
  if (design.length !== 1) return { kind: "not_offered", support, reason: `The result gives no single load of ${support} in the design operating case ${designCase}, so it is not offered.` };
  return {
    kind: "offered", support, hangerId: selected.hanger_id, target,
    application: { run, loads: { family: "variable_spring", coldLoadN: selected.f_cold_n, hotLoadN: design[0].f_n, designOperatingCase: designCase } }
  };
}

/** The library as the person has it stored now, read again at the act: its document, or `unavailable` when it is no
 * longer stored, cannot be read, or has blocking import findings. */
export type StoredLibraryRead = Readonly<{ kind: "read"; document: unknown }> | Readonly<{ kind: "unavailable" }>;

/** Whether `library` is the library `document` binds: the one it names, with the checked hash it binds. */
export async function hangerSizingLibraryBound(document: HangerSizingSizedDocument, library: unknown): Promise<boolean> {
  return (await libraryReason(document, library)) === null;
}

/** The number of the run's candidates with a selection; a candidate with no fitting record is never counted. */
export const hangerSizingSelectedCount = (document: HangerSizingSizedDocument) =>
  document.result.candidates.filter((candidate) => candidate.outcome === "selected" || candidate.outcome === "selected_by_fallback").length;

/** For each candidate of `document`, whether its selection is offered for application to `model`, and why not. */
export async function hangerSizingApplyOffers(document: HangerSizingSizedDocument, model: PreviewModel, library: unknown): Promise<ApplyOffers> {
  const binding = await hangerSizingModelBinding(document, model);
  const candidates = document.result.candidates;
  if (binding !== "current") {
    const reason = binding === "changed" ? HANGER_SIZING_MODEL_CHANGED_TEXT : HANGER_SIZING_MODEL_NOT_HASHABLE_TEXT;
    return { model: binding, offers: candidates.map((candidate) => ({ kind: "not_offered", support: candidate.support, reason })) };
  }
  const refused = await libraryReason(document, library);
  if (refused !== null || !isPlainObject(library)) {
    return { model: binding, offers: candidates.map((candidate) => ({ kind: "not_offered", support: candidate.support, reason: refused ?? "" })) };
  }
  const offers: ApplyOffer[] = [];
  for (const candidate of candidates) offers.push(await offerFor(document, candidate, library));
  return { model: binding, offers };
}

/** The library document the result binds, as the hanger-selection builder reads it. */
export const asHangerDocument = (library: unknown) => library as HangerDocument;
