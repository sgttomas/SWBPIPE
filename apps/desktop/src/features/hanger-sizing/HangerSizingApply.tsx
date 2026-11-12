/** The person's acts of applying a sized selection (DEL-04-11 REQ-008, REQ-012), shown beside the result. For each
 * selected candidate, or for every selection of the run at once (`runApplication.ts`), the person may prepare the
 * batch, read its before and after and its source notes, and queue it;
 * the queued batch goes to the same review as an imported hanger selection, and the model changes only when the person
 * accepts it there. Nothing here writes the model. A selection is offered only while the shown result is bound to
 * the present model and to the library the person has stored now under the id it names (`sizedSelection.ts`): the
 * library the form presently holds for that id, where it holds one, and the stored library read again at each act. A
 * prepared batch is dropped when the model, the result, either library or the busy state changes. The values written
 * come from the bound library content. */

import { useEffect, useRef, useState } from "react";
import type { OperationBatch } from "../../services/operationBatchService";
import type { PreviewModel } from "../../types";
import { buildHangerSelectionBatch } from "../hanger-selection/hangerSelection";
import type { HangerSizingSizedDocument } from "../results/hangerSizingResult";
import {
  asHangerDocument,
  HANGER_SIZING_APPLY_TEXT,
  HANGER_SIZING_APPLY_TITLE,
  HANGER_SIZING_LIBRARY_CHANGED_TEXT,
  HANGER_SIZING_LIBRARY_UNAVAILABLE_TEXT,
  HANGER_SIZING_MODEL_CHANGED_TEXT,
  HANGER_SIZING_MODEL_NOT_HASHABLE_TEXT,
  HANGER_SIZING_PRELOAD_PENDING_TEXT,
  hangerSizingApplyOffers,
  hangerSizingLibraryBound,
  type ApplyOffer,
  type ApplyOffers,
  type StoredLibraryRead
} from "./sizedSelection";
import {
  HANGER_SIZING_APPLY_RUN_TEXT,
  HANGER_SIZING_APPLY_RUN_TITLE,
  HANGER_SIZING_RUN_UNCHANGED_LABEL,
  HANGER_SIZING_RUN_UNCHANGED_TEXT,
  prepareRunApplication,
  runApplicationRefusal,
  runRefusalShown,
  type RunChange
} from "./runApplication";

type Props = Readonly<{
  document: HangerSizingSizedDocument;
  /** The library document the request sent: the content the values written come from. */
  library: unknown;
  /** The library the form presently holds under the id the result names, or undefined when it holds none for that id. */
  currentLibrary?: unknown;
  /** Reads the library stored now under the id the result names, at each act. The form always supplies it. */
  readStoredLibrary?: () => Promise<StoredLibraryRead>;
  model: PreviewModel;
  busy: boolean;
  onQueueBatch: (batch: OperationBatch) => void;
}>;

/** What a check or a prepared batch is bound to; it counts only while every member is the one shown. */
type Basis = Readonly<{ document: HangerSizingSizedDocument; model: PreviewModel; library: unknown; currentLibrary: unknown }>;
type LibraryStanding = "current" | "changed" | "unavailable";
type Checked = Basis & Readonly<{ offers: ApplyOffers; standing: LibraryStanding }>;
type Review = Basis & Readonly<{ support: string; variable: boolean; batch: OperationBatch }>;
type StoredRefusal = Basis & Readonly<{ standing: Exclude<LibraryStanding, "current"> }>;
type RunReview = Basis & Readonly<{
  batch: OperationBatch;
  changed: readonly RunChange[]; unchanged: readonly string[]; description: string;
}>;

const LIBRARY_TEXT = { changed: HANGER_SIZING_LIBRARY_CHANGED_TEXT, unavailable: HANGER_SIZING_LIBRARY_UNAVAILABLE_TEXT } as const;

export function HangerSizingApply({ document, library, currentLibrary, readStoredLibrary, model, busy, onQueueBatch }: Props) {
  const [checked, setChecked] = useState<Checked | null>(null);
  const [storedRefusal, setStoredRefusal] = useState<StoredRefusal | null>(null);
  const [review, setReview] = useState<Review | null>(null);
  const [runReview, setRunReview] = useState<RunReview | null>(null);
  const [runRefusal, setRunRefusal] = useState("");
  const [error, setError] = useState("");
  const [pending, setPending] = useState(false);
  const generation = useRef(0);

  useEffect(() => {
    const token = ++generation.current;
    setError("");
    setRunRefusal("");
    setPending(false);
    void (async () => {
      const offers = await hangerSizingApplyOffers(document, model, library);
      const standing: LibraryStanding = currentLibrary === undefined || await hangerSizingLibraryBound(document, currentLibrary) ? "current" : "changed";
      if (token === generation.current) setChecked({ document, model, library, currentLibrary, offers, standing });
    })();
  }, [document, model, library, currentLibrary]);

  // Only a check of exactly what is shown counts; a check of an earlier model, result or library offers nothing.
  const shownBasis = (basis: Basis | null) =>
    basis !== null && basis.document === document && basis.model === model && basis.library === library && basis.currentLibrary === currentLibrary;
  const refusedStanding = shownBasis(storedRefusal) ? storedRefusal!.standing : null;
  const libraryStanding: LibraryStanding | null = !shownBasis(checked) ? null : refusedStanding ?? checked!.standing;
  const offers = shownBasis(checked) && libraryStanding === "current" ? checked!.offers : null;
  const currentReview = shownBasis(review) && !busy ? review : null;
  const currentRunReview = shownBasis(runReview) && !busy ? runReview : null;
  const runOffer = offers?.model === "current" ? runApplicationRefusal(document, offers) : null;

  /** The act's check of the library stored now: true when it is the one the result binds. Otherwise the acts are
   * withdrawn with the reason. */
  async function storedLibraryBound(token: number): Promise<boolean> {
    if (readStoredLibrary === undefined) return true;
    const stored = await readStoredLibrary();
    const standing: LibraryStanding = stored.kind === "unavailable" ? "unavailable" : await hangerSizingLibraryBound(document, stored.document) ? "current" : "changed";
    if (token !== generation.current) return false;
    if (standing === "current") return true;
    setStoredRefusal({ document, model, library, currentLibrary, standing });
    return false;
  }

  async function prepare(offer: Extract<ApplyOffer, { kind: "offered" }>) {
    const token = ++generation.current;
    setReview(null);
    setRunReview(null);
    setRunRefusal("");
    setError("");
    setPending(true);
    try {
      if (!(await storedLibraryBound(token))) return;
      // Bound again at the act: the batch is built from the model it is checked against.
      const again = await hangerSizingApplyOffers(document, model, library);
      const still = again.offers.find((other) => other.support === offer.support);
      if (token !== generation.current) return;
      setChecked({ document, model, library, currentLibrary, offers: again, standing: "current" });
      if (again.model !== "current" || still?.kind !== "offered") return;
      const batch = await buildHangerSelectionBatch(model, asHangerDocument(library), still.hangerId, still.target, still.application);
      if (token === generation.current) setReview({ document, model, library, currentLibrary, support: still.support, variable: still.application.loads.family === "variable_spring", batch });
    } catch (cause) {
      if (token === generation.current) setError(cause instanceof Error ? cause.message : String(cause));
    } finally {
      if (token === generation.current) setPending(false);
    }
  }

  async function prepareRun() {
    const token = ++generation.current;
    setReview(null);
    setRunReview(null);
    setRunRefusal("");
    setError("");
    setPending(true);
    try {
      if (!(await storedLibraryBound(token))) return;
      // Bound again at the act, and every selection built against the model it is checked against.
      const prepared = await prepareRunApplication(document, model, library);
      if (token !== generation.current) return;
      if (prepared.kind === "not_offered") setRunRefusal(prepared.reason);
      else setRunReview({ document, model, library, currentLibrary, batch: prepared.batch, changed: prepared.changed, unchanged: prepared.unchanged, description: prepared.description });
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
      {libraryStanding === null ? <p role="status" data-testid="hanger-sizing-apply-checking">Checking that this result is bound to the present model…</p> : null}
      {libraryStanding === "changed" || libraryStanding === "unavailable"
        ? <p role="status" data-testid="hanger-sizing-apply-library-changed">{LIBRARY_TEXT[libraryStanding]}</p> : null}
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
          {currentReview.variable ? <p data-testid="hanger-sizing-apply-preload-pending">{HANGER_SIZING_PRELOAD_PENDING_TEXT}</p> : null}
          <button type="button" disabled={busy || pending} onClick={() => { onQueueBatch(currentReview.batch); setReview(null); }}>
            Queue sized selection for review
          </button>
        </div>
      ) : null}
      {review && !currentReview && !busy ? <p role="status">This prepared application is stale. Prepare it again.</p> : null}
      {offers?.model === "current" ? (
        <div data-testid="hanger-sizing-apply-run">
          <h5>{HANGER_SIZING_APPLY_RUN_TITLE}</h5>
          <p>{HANGER_SIZING_APPLY_RUN_TEXT}</p>
          {runOffer === null ? (
            <button type="button" disabled={busy || pending} onClick={() => void prepareRun()}>
              Prepare applying every selection of this run
            </button>
          ) : <p data-testid="hanger-sizing-apply-run-not-offered">{runRefusalShown(runOffer)}</p>}
          {runRefusal && runOffer === null ? <p role="alert" data-testid="hanger-sizing-apply-run-refused">{runRefusal}</p> : null}
        </div>
      ) : null}
      {currentRunReview ? (
        <div data-testid="hanger-sizing-apply-run-review">
          <h5>Review the configuration replacement of {currentRunReview.changed.length === 1 ? "one support" : `${currentRunReview.changed.length} supports`}</h5>
          <p data-testid="hanger-sizing-apply-run-description">{currentRunReview.description}</p>
          <p>Supports changed:</p>
          <ul data-testid="hanger-sizing-apply-run-changed">
            {currentRunReview.changed.map((change) => <li key={change.support}>{change.support}: record {change.hangerId}</li>)}
          </ul>
          {currentRunReview.unchanged.length > 0 ? (
            <>
              <p>{HANGER_SIZING_RUN_UNCHANGED_LABEL}:</p>
              <ul data-testid="hanger-sizing-apply-run-unchanged">
                {currentRunReview.unchanged.map((support) => <li key={support}>{support}</li>)}
              </ul>
              <p>{HANGER_SIZING_RUN_UNCHANGED_TEXT}</p>
            </>
          ) : null}
          {currentRunReview.changed.some((change) => change.family === "variable_spring")
            ? <p data-testid="hanger-sizing-apply-run-preload-pending">{HANGER_SIZING_PRELOAD_PENDING_TEXT}</p> : null}
          <p>Previous top-level stiffness and nonlinear settings of each changed support will be removed. Review each support's complete before and after configuration and the source of each sized value.</p>
          {currentRunReview.batch.operations.filter((operation) => operation.change.field_path === "configuration").map((operation) => (
            <details open key={operation.change.change_id} data-testid={`hanger-sizing-apply-run-${operation.target.ref}`}>
              <summary>{operation.target.ref}: before and after</summary>
              <pre data-testid="hanger-sizing-apply-run-before">{operation.change.before}</pre>
              <pre data-testid="hanger-sizing-apply-run-after">{operation.change.after}</pre>
              <p data-testid="hanger-sizing-apply-run-source-note">{operation.change.source_note}</p>
            </details>
          ))}
          <button type="button" disabled={busy || pending} onClick={() => { onQueueBatch(currentRunReview.batch); setRunReview(null); }}>
            Queue every sized selection of this run for review
          </button>
        </div>
      ) : null}
      {runReview && !currentRunReview && !busy ? <p role="status">This prepared application of the run is stale. Prepare it again.</p> : null}
    </section>
  );
}
