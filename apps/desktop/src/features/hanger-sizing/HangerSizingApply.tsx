/** The person's act of applying a sized selection (DEL-04-11 REQ-008, REQ-012), shown beside the result. For each
 * selected candidate the person may prepare the batch, read its before and after and its source note, and queue it;
 * the queued batch goes to the same review as an imported hanger selection, and the model changes only when the person
 * accepts it there. Nothing here writes the model. A selection is offered only while the shown result is bound to
 * the present model (`sizedSelection.ts`); a prepared batch is dropped when the model, the result or the busy state
 * changes. */

import { useEffect, useRef, useState } from "react";
import type { OperationBatch } from "../../services/operationBatchService";
import type { PreviewModel } from "../../types";
import { buildHangerSelectionBatch } from "../hanger-selection/hangerSelection";
import type { HangerSizingSizedDocument } from "../results/hangerSizingResult";
import {
  asHangerDocument,
  HANGER_SIZING_APPLY_TEXT,
  HANGER_SIZING_APPLY_TITLE,
  HANGER_SIZING_MODEL_CHANGED_TEXT,
  HANGER_SIZING_MODEL_NOT_HASHABLE_TEXT,
  hangerSizingApplyOffers,
  type ApplyOffer,
  type ApplyOffers
} from "./sizedSelection";

type Props = Readonly<{
  document: HangerSizingSizedDocument;
  /** The library document the request sent. */
  library: unknown;
  model: PreviewModel;
  busy: boolean;
  onQueueBatch: (batch: OperationBatch) => void;
}>;

type Checked = Readonly<{ document: HangerSizingSizedDocument; model: PreviewModel; library: unknown; offers: ApplyOffers }>;
type Review = Readonly<{ document: HangerSizingSizedDocument; model: PreviewModel; support: string; batch: OperationBatch }>;

export function HangerSizingApply({ document, library, model, busy, onQueueBatch }: Props) {
  const [checked, setChecked] = useState<Checked | null>(null);
  const [review, setReview] = useState<Review | null>(null);
  const [error, setError] = useState("");
  const [pending, setPending] = useState(false);
  const generation = useRef(0);

  useEffect(() => {
    const token = ++generation.current;
    setError("");
    setPending(false);
    void hangerSizingApplyOffers(document, model, library).then((offers) => {
      if (token === generation.current) setChecked({ document, model, library, offers });
    });
  }, [document, model, library]);

  // Only a check of exactly what is shown counts; a check of an earlier model or result offers nothing.
  const offers = checked && checked.document === document && checked.model === model && checked.library === library ? checked.offers : null;
  const currentReview = review && review.document === document && review.model === model && !busy ? review : null;

  async function prepare(offer: Extract<ApplyOffer, { kind: "offered" }>) {
    const token = ++generation.current;
    setReview(null);
    setError("");
    setPending(true);
    try {
      // Bound again at the act: the batch is built from the model it is checked against.
      const again = await hangerSizingApplyOffers(document, model, library);
      const still = again.offers.find((other) => other.support === offer.support);
      if (token !== generation.current) return;
      setChecked({ document, model, library, offers: again });
      if (again.model !== "current" || still?.kind !== "offered") return;
      const batch = await buildHangerSelectionBatch(model, asHangerDocument(library), still.hangerId, still.target, still.application);
      if (token === generation.current) setReview({ document, model, support: still.support, batch });
    } catch (cause) {
      if (token === generation.current) setError(cause instanceof Error ? cause.message : String(cause));
    } finally {
      if (token === generation.current) setPending(false);
    }
  }

  return (
    <section aria-label={HANGER_SIZING_APPLY_TITLE} data-testid="hanger-sizing-apply">
      <h4>{HANGER_SIZING_APPLY_TITLE}</h4>
      <p>{HANGER_SIZING_APPLY_TEXT}</p>
      {offers === null ? <p role="status" data-testid="hanger-sizing-apply-checking">Checking that this result is bound to the present model…</p> : null}
      {offers?.model === "changed" ? <p role="status" data-testid="hanger-sizing-apply-model-changed">{HANGER_SIZING_MODEL_CHANGED_TEXT}</p> : null}
      {offers?.model === "not_hashable" ? <p role="status" data-testid="hanger-sizing-apply-model-not-hashable">{HANGER_SIZING_MODEL_NOT_HASHABLE_TEXT}</p> : null}
      {offers?.model === "current" ? (
        <ul>
          {offers.offers.map((offer) => (
            <li key={offer.support} data-testid={`hanger-sizing-apply-${offer.support}`}>
              {offer.kind === "offered" ? (
                <button type="button" disabled={busy || pending} onClick={() => void prepare(offer)}>
                  Prepare applying record {offer.hangerId} to {offer.support}
                </button>
              ) : (
                <p>{offer.support}: {offer.reason}</p>
              )}
            </li>
          ))}
        </ul>
      ) : null}
      {error ? <p role="alert" data-testid="hanger-sizing-apply-error">{error}</p> : null}
      {currentReview ? (
        <div data-testid="hanger-sizing-apply-review">
          <h5>Review the configuration replacement of {currentReview.support}</h5>
          <p>Previous top-level stiffness and nonlinear settings will be removed. Review the complete before and after configuration and the source of each sized value.</p>
          <details open>
            <summary>Before and after</summary>
            <pre data-testid="hanger-sizing-apply-before">{currentReview.batch.operations[0].change.before}</pre>
            <pre data-testid="hanger-sizing-apply-after">{currentReview.batch.operations[0].change.after}</pre>
          </details>
          <p data-testid="hanger-sizing-apply-source-note">{currentReview.batch.operations[0].change.source_note}</p>
          <button type="button" disabled={busy || pending} onClick={() => { onQueueBatch(currentReview.batch); setReview(null); }}>
            Queue sized selection for review
          </button>
        </div>
      ) : null}
      {review && !currentReview && !busy ? <p role="status">This prepared application is stale. Prepare it again.</p> : null}
    </section>
  );
}
