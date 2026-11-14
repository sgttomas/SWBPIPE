/** Applying every selection of one sizing run in one batch (DEL-04-11 REQ-008, AX-002 "Results, not edits").
 *
 * A sizing run sizes its candidates together: every hot-load basis comes from one weight solve with all candidates
 * rigid, and every position from solves with all candidates replaced by their forces. Once one selection is applied the
 * model has changed and the run is no longer bound to it, so the run's selections are applied together or not at all.
 * The batch is the join, in the run's candidate order, of what the one hanger-selection builder
 * (`buildHangerSelectionBatch` with its sized application) produces for each selected candidate, each built against
 * the present model. A candidate with no fitting record is left as it is and named. The batch goes to the same review
 * and acceptance route as a single selection; the operation route applies it as one batch on the person's acceptance,
 * and one undo restores the model as it was. Nothing here writes the model. */

import type { OperationBatch } from "../../services/operationBatchService";
import type { PreviewModel } from "../../types";
import { buildHangerSelectionBatch } from "../hanger-selection/hangerSelection";
import type { HangerSizingSizedDocument } from "../results/hangerSizingResult";
import { asHangerDocument, HANGER_SIZING_MODEL_CHANGED_TEXT, HANGER_SIZING_MODEL_NOT_HASHABLE_TEXT, hangerSizingApplyOffers, type ApplyOffers } from "./sizedSelection";

export const HANGER_SIZING_APPLY_RUN_TITLE = "Apply every selection of this run";
/** REQ-013: what applying the run is, in the claims registry's vocabulary. */
export const HANGER_SIZING_APPLY_RUN_TEXT =
  "Applying the run writes, in one batch, into each support with a selection the record that your own tests selected from your own hanger library, with the loads this run sized. " +
  "The run's supports were sized together, so its selections are applied together or not at all. The batch is queued for your review and changes the model only when you accept it there, " +
  "and one undo restores the model as it was. It judges no hanger's suitability for a service and determines no code compliance.";
export const HANGER_SIZING_RUN_UNCHANGED_LABEL = "Left unchanged: no record fits";
/** REQ-005: a run with an unfitted candidate verified it as a force equal to its hot-load basis. */
export const HANGER_SIZING_RUN_UNCHANGED_TEXT =
  "This run's verification assumed each of these supports carries a force equal to its hot-load basis; applying leaves them as they are, so the applied model does not carry that assumption.";
export const HANGER_SIZING_RUN_NOTHING_TEXT = "No record fits any support of this run, so there is nothing to apply.";

export type RunChange = Readonly<{ support: string; hangerId: string; family: "variable_spring" | "constant_effort" }>;

export type RunApplication =
  | Readonly<{ kind: "prepared"; batch: OperationBatch; changed: readonly RunChange[]; unchanged: readonly string[]; description: string }>
  | Readonly<{ kind: "not_offered"; reason: string }>;

const FAMILY_TEXT = { variable_spring: "variable spring", constant_effort: "constant effort" } as const;

/** The batch's shown description: each support changed with its record, and each support left unchanged. */
export function describeRunApplication(changed: readonly RunChange[], unchanged: readonly string[]): string {
  const applied = `Applies ${changed.length} sized selection${changed.length === 1 ? "" : "s"} of this run in one batch: ` +
    changed.map((change) => `${change.support}, record ${change.hangerId} (${FAMILY_TEXT[change.family]})`).join("; ") + ".";
  return unchanged.length === 0 ? applied : `${applied} ${HANGER_SIZING_RUN_UNCHANGED_LABEL}: ${unchanged.join(", ")}.`;
}

const WHOLE_OR_NOTHING = "The run is applied whole or not at all, so none of its selections is offered.";

/** Why the run as a whole is not offered: the reason, and the candidate that stops it when one does. */
export type RunRefusal = Readonly<{ reason: string; support?: string }>;

/** Whether the run as a whole may be offered, from the per-candidate offers: null when it may, else the refusal, whose
 * reason names the candidate that stops it. A candidate with no fitting record does not stop it. */
export function runApplicationRefusal(document: HangerSizingSizedDocument, offers: ApplyOffers): RunRefusal | null {
  if (offers.model === "changed") return { reason: HANGER_SIZING_MODEL_CHANGED_TEXT };
  if (offers.model === "not_hashable") return { reason: HANGER_SIZING_MODEL_NOT_HASHABLE_TEXT };
  const candidates = document.result.candidates;
  if (candidates.every((candidate) => candidate.outcome === "no_record_fits")) return { reason: HANGER_SIZING_RUN_NOTHING_TEXT };
  for (const candidate of candidates) {
    if (candidate.outcome === "no_record_fits") continue;
    const offer = offers.offers.find((entry) => entry.support === candidate.support);
    const why = offer === undefined ? "It has no application offer." : offer.kind === "not_offered" ? offer.reason : null;
    if (why !== null) return { support: candidate.support, reason: `${candidate.support}: ${why} ${WHOLE_OR_NOTHING}` };
  }
  return null;
}

/** The run's refusal as shown beside the per-candidate rows, which already give a stopping candidate's own reason. */
export function runRefusalShown(refusal: RunRefusal): string {
  return refusal.support === undefined ? refusal.reason : `${refusal.support} cannot be applied, for the reason given beside it. ${WHOLE_OR_NOTHING}`;
}

/** The person's act: prepares the one batch for every selection of `document`, checked again against the present
 * `model` and `library`, or says why the run is not offered. */
export async function prepareRunApplication(document: HangerSizingSizedDocument, model: PreviewModel, library: unknown): Promise<RunApplication> {
  const offers = await hangerSizingApplyOffers(document, model, library);
  const refused = runApplicationRefusal(document, offers);
  if (refused !== null) return { kind: "not_offered", reason: refused.reason };
  const operations: OperationBatch["operations"] = [];
  const changed: RunChange[] = [];
  const unchanged: string[] = [];
  for (const candidate of document.result.candidates) {
    if (candidate.outcome === "no_record_fits") {
      unchanged.push(candidate.support);
      continue;
    }
    const offer = offers.offers.find((entry) => entry.support === candidate.support);
    if (offer?.kind !== "offered") return { kind: "not_offered", reason: `${candidate.support} is not offered, so the run is not applied.` };
    try {
      const built = await buildHangerSelectionBatch(model, asHangerDocument(library), offer.hangerId, offer.target, offer.application);
      operations.push(...built.operations);
    } catch (cause) {
      const message = cause instanceof Error ? cause.message : String(cause);
      return { kind: "not_offered", reason: `${candidate.support}: ${message} ${WHOLE_OR_NOTHING}` };
    }
    changed.push({ support: candidate.support, hangerId: offer.hangerId, family: offer.application.loads.family });
  }
  return {
    kind: "prepared",
    batch: { batch_id: `batch:hanger-sizing-run:${crypto.randomUUID()}`, operations },
    changed,
    unchanged,
    description: describeRunApplication(changed, unchanged)
  };
}
