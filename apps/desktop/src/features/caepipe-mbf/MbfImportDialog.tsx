import { useEffect, useLayoutEffect, useMemo, useRef, useState } from "react";
import { createPortal } from "react-dom";
import {
  MBF_IMPORT_BOUNDARY_SENTENCE,
  adoptCapture,
  assumptionText,
  declarationFor,
  declarationKey,
  designationReasonText,
  fieldText,
  fieldWhole,
  importAdoptable,
  importBlockingCodes,
  importCodeText,
  importFailureDiagnostic,
  importReviewFormatCurrent,
  type MbfAdoptCapture,
  type MbfDeclarationEntry,
  type MbfDesignateImportArgs,
  type MbfImportDesignateOutcome,
  type MbfImportDiagnostic,
  type MbfImportOpenOutcome,
  type MbfImportReviewOutcome,
  type MbfImportReviewPayload,
  type MbfReviewImportArgs
} from "./mbfImport";
import type { MbfImportStarted } from "./mbfImportCancel";

/** The open Import MBF… dialog (DEL-17-04 Design `desktop-mbf-export-import.md` §4.1–§4.3, slice 5). The workspace
 * session opens it from File > Import MBF… and supplies the barrier's native starts (`mbfImportCancel.ts`) and its
 * Adopt, which hands the captured identities to the maintained Open transaction (`handleOpenModelDocument` with
 * `mbfImportSource`). */
export type MbfImportDialogRequest = {
  /** Focus returns here when the dialog closes. */
  opener: HTMLElement | null;
  /** Close, Escape or Cancel: releases the modal hold and sends the request's one retirement. */
  close: () => void;
  /** The native chooser, selection and first review. */
  open: () => Promise<MbfImportStarted<MbfImportOpenOutcome>>;
  /** A declaration re-run. */
  review: (args: MbfReviewImportArgs, stillCurrent?: () => boolean) => Promise<MbfImportStarted<MbfImportReviewOutcome>>;
  /** Designate original source… (§5.4): the model-document chooser and the review stating the designation, formed over
   * the review shown and the declaration it was formed for. */
  designate: (args: MbfDesignateImportArgs, stillCurrent?: () => boolean) => Promise<MbfImportStarted<MbfImportDesignateOutcome>>;
  /** Adopt: the session releases the modal hold and adopts through the Open transaction; the request's retirement is
   * then the import flow's. */
  adopt: (capture: MbfAdoptCapture) => void;
};

type Reviewed = Extract<MbfImportReviewOutcome, { outcome: "reviewed" }>;
/** A reviewed state: the review shown (`reviewed`, X when designated), the declaration it was formed for, and the
 * digest of the ordinary review Y it rests on (the review's own digest unless designated). */
type State =
  | { phase: "choosing" }
  | { phase: "reviewed"; reviewed: Reviewed; key: string; base: string }
  | { phase: "refused"; fileName: string; diagnostics: MbfImportDiagnostic[]; sourceId: number | null };

type Choice = { choice: "none" | "invented" | "user"; name: string };
const NO_CHOICE: Choice = { choice: "none", name: "" };

function entryOf(choice: Choice): MbfDeclarationEntry | null {
  if (choice.choice === "invented") return { origin: "project_invented", source_name: "project-invented example data" };
  if (choice.choice === "user" && choice.name.trim() !== "") return { origin: "user_supplied", source_name: choice.name };
  return null;
}

/** File > Import MBF…: the bounded review of what the import reads, refuses and assumes, the material declarations,
 * and Adopt. Modal like the export dialog: Tab stays on its controls, window shortcuts are held back, Escape closes,
 * focus returns to the opener. Adopt is available only for the current, complete, adoptable review formed for the
 * declaration the form holds, with every applied mapping assumption shown. */
export function MbfImportDialog({ request }: { request: MbfImportDialogRequest | null }) {
  const dialog = useRef<HTMLElement>(null);
  const [state, setState] = useState<State>({ phase: "choosing" });
  const [pending, setPending] = useState(false);
  const [materials, setMaterials] = useState<string[]>([]);
  const [uncut, setUncut] = useState<string[]>([]);
  const [form, setForm] = useState<Record<string, Choice>>({});
  const [designationRefusal, setDesignationRefusal] = useState<MbfImportDiagnostic[] | null>(null);
  /** Bumped to send the ordinary review again (a designation shown is no longer current natively). */
  const [refresh, setRefresh] = useState(0);
  const generation = useRef(0);
  const live = useRef<MbfImportDialogRequest | null>(null);
  const reviewOwner = useRef<MbfImportDialogRequest | null>(null);

  useLayoutEffect(() => {
    reviewOwner.current = null;
    if (!request) {
      setState({ phase: "choosing" });
      setMaterials([]);
      setUncut([]);
      setForm({});
      setPending(false);
      setDesignationRefusal(null);
      return;
    }
    live.current = request;
    generation.current += 1;
    const mine = generation.current;
    setState({ phase: "choosing" });
    setPending(true);
    setMaterials([]);
    setUncut([]);
    setForm({});
    setDesignationRefusal(null);
    const opener = request.opener;
    const hold = (event: KeyboardEvent) => {
      if (event.key === "Escape") {
        event.preventDefault();
        event.stopImmediatePropagation();
        request.close();
        return;
      }
      if (event.key === "Tab") {
        const controls = [...(dialog.current?.querySelectorAll<HTMLElement>("input, select, button:not(:disabled)") ?? [])];
        if (controls.length === 0) return;
        const at = controls.indexOf(document.activeElement as HTMLElement);
        const next = event.shiftKey ? (at <= 0 ? controls.length - 1 : at - 1) : (at + 1) % controls.length;
        event.preventDefault();
        event.stopImmediatePropagation();
        controls[next].focus();
        return;
      }
      if (event.metaKey || event.ctrlKey) event.stopImmediatePropagation();
    };
    window.addEventListener("keydown", hold, true);
    dialog.current?.focus();
    void request.open().then(
      (started) => {
        if (generation.current !== mine || live.current !== request || !started.current()) return;
        setPending(false);
        const outcome = started.outcome;
        if (outcome.outcome === "cancelled") {
          request.close();
          return;
        }
        if (outcome.outcome === "refused") {
          setState({ phase: "refused", fileName: outcome.fileName, diagnostics: outcome.diagnostics, sourceId: null });
          return;
        }
        if (!importReviewFormatCurrent(outcome.review)) {
          setState({ phase: "refused", fileName: outcome.fileName, diagnostics: [importFailureDiagnostic("MBF-IMPORT-WORKER-PROTOCOL")], sourceId: null });
          return;
        }
        const needed = outcome.review.declaration_needed;
        setMaterials(needed.filter(fieldWhole));
        setUncut(needed.filter((name) => !fieldWhole(name)).map(fieldText));
        reviewOwner.current = request;
        setState({ phase: "reviewed", reviewed: outcome, key: declarationKey(null), base: outcome.reviewSha256 });
      },
      (error: unknown) => {
        if (generation.current !== mine || live.current !== request) return;
        setPending(false);
        setState({ phase: "refused", fileName: "", diagnostics: [importFailureDiagnostic(error)], sourceId: null });
      }
    );
    return () => {
      window.removeEventListener("keydown", hold, true);
      live.current = null;
      generation.current += 1;
      opener?.focus();
    };
  }, [request]);

  const declaration = useMemo(() => declarationFor(materials.map((name) => [name, entryOf(Object.hasOwn(form, name) ? form[name] : NO_CHOICE)] as const)), [materials, form]);
  const key = declarationKey(declaration);
  const sourceId = state.phase === "reviewed" ? state.reviewed.sourceId : state.phase === "refused" ? state.sourceId : null;
  const formedFor = state.phase === "reviewed" ? state.key : null;

  // Each declaration change sends a new review; Adopt waits for it. A superseded reply is discarded by generation.
  useEffect(() => {
    if (!request || sourceId === null || reviewOwner.current !== request) return;
    generation.current += 1;
    const mine = generation.current;
    if (formedFor === key) {
      setPending(false);
      return;
    }
    const current = () => generation.current === mine && live.current === request;
    setPending(true);
    void request.review({ sourceId, declaration }, current).then(
      (started) => {
        if (generation.current !== mine || live.current !== request || !started.current()) return;
        setPending(false);
        const outcome = started.outcome;
        if (outcome.outcome === "reviewed" && !importReviewFormatCurrent(outcome.review)) {
          setState({ phase: "refused", fileName: outcome.fileName, diagnostics: [importFailureDiagnostic("MBF-IMPORT-WORKER-PROTOCOL")], sourceId: null });
        } else if (outcome.outcome === "reviewed") setState({ phase: "reviewed", reviewed: outcome, key, base: outcome.reviewSha256 });
        else setState({ phase: "refused", fileName: outcome.fileName, diagnostics: outcome.diagnostics, sourceId });
      },
      (error: unknown) => {
        if (generation.current !== mine || live.current !== request) return;
        setPending(false);
        setState({ phase: "refused", fileName: "", diagnostics: [importFailureDiagnostic(error)], sourceId });
      }
    );
    // `form` also invalidates work when an edit returns to the already displayed declaration; `refresh` re-sends the
    // ordinary review when a designation shown is no longer current.
  }, [request, sourceId, key, form, refresh]);

  if (!request) return null;
  const reviewed = state.phase === "reviewed" ? state.reviewed : null;
  const current = reviewed !== null && !pending && formedFor === key;
  const adoptable = current && importAdoptable(reviewed.review);

  const adopt = () => {
    if (!adoptable || reviewed === null) return;
    const capture = adoptCapture(reviewed, declaration);
    if (capture === null) return;
    // Release the review body before the model is requested: the flow keeps only the capture.
    setState({ phase: "choosing" });
    request.adopt(capture);
  };

  const changeForm = (next: Parameters<typeof setForm>[0]) => {
    generation.current += 1; // Invalidate replies/retries synchronously, before the next effect runs.
    setDesignationRefusal(null);
    setForm(next);
  };

  // Designate original source… (§5.4). The native start retires any designation shown before its chooser opens, so
  // whatever it returns other than a new designation leaves the ordinary review to be formed again when X was shown.
  const designated = reviewed !== null && reviewed.review.identity.designation !== null;
  const designate = () => {
    if (!current || reviewed === null || state.phase !== "reviewed") return;
    generation.current += 1;
    const mine = generation.current;
    const shown = state;
    const isCurrent = () => generation.current === mine && live.current === request;
    const resetOrdinary = () => {
      if (!designated) return;
      setState({ ...shown, key: "", reviewed: shown.reviewed });
      setRefresh((value) => value + 1);
    };
    setPending(true);
    setDesignationRefusal(null);
    void request.designate({ sourceId: reviewed.sourceId, declaration, reviewSha256: shown.base }, isCurrent).then(
      (started) => {
        if (!isCurrent() || !started.current()) return;
        setPending(false);
        const outcome = started.outcome;
        if (outcome.outcome === "designated" && importReviewFormatCurrent(outcome.review) && outcome.baseReviewSha256 === shown.base) {
          const asReviewed: Reviewed = { outcome: "reviewed", sourceId: outcome.sourceId, fileName: outcome.fileName, reviewSha256: outcome.reviewSha256, review: outcome.review };
          setState({ phase: "reviewed", reviewed: asReviewed, key: shown.key, base: outcome.baseReviewSha256 });
          return;
        }
        if (outcome.outcome === "refused") setDesignationRefusal(outcome.diagnostics);
        else if (outcome.outcome === "designated") setDesignationRefusal([importFailureDiagnostic("MBF-IMPORT-WORKER-PROTOCOL")]);
        resetOrdinary();
      },
      (error: unknown) => {
        if (!isCurrent()) return;
        setPending(false);
        setDesignationRefusal([importFailureDiagnostic(error)]);
        resetOrdinary();
      }
    );
  };
  const setAll = (choice: Choice["choice"], name = "") => changeForm(Object.fromEntries(materials.map((material) => [material, { choice, name }])));

  return createPortal(
    <div className="replace-project-backdrop" data-testid="mbf-import-backdrop">
      <section
        ref={dialog}
        tabIndex={-1}
        role="dialog"
        aria-modal="true"
        aria-labelledby="mbf-import-title"
        className="replace-project-prompt panel mbf-export-dialog"
        data-testid="mbf-import-dialog"
      >
        <h2 id="mbf-import-title">Import model batch file</h2>
        {state.phase === "choosing" ? <p role="status" data-testid="mbf-import-choosing">Choose a model batch file (.mbf)…</p> : null}
        {state.phase === "refused" ? (
          <ul role="alert" data-testid="mbf-import-refusal">
            {state.diagnostics.map((item, index) => <li key={`${item.code}-${index}`}>{item.code}: {importCodeText(item.code)}</li>)}
          </ul>
        ) : null}
        {reviewed ? <ImportReview reviewed={reviewed} /> : null}
        {designationRefusal ? (
          <ul role="alert" data-testid="mbf-import-designation-refusal">
            {designationRefusal.map((item, index) => <li key={`${item.code}-${index}`}>{item.code}: {importCodeText(item.code)}</li>)}
          </ul>
        ) : null}
        {materials.length > 0 || uncut.length > 0 ? (
          <fieldset className="mbf-export-settings" data-testid="mbf-import-declaration">
            <legend>Material provenance</legend>
            <p>Each listed material needs its source: project-invented example data, or your own named source.</p>
            {materials.length > 1 ? (
              <label>
                All listed materials{" "}
                <select data-testid="mbf-import-all-origin" value="" onChange={(event) => setAll(event.target.value as Choice["choice"])}>
                  <option value="">Choose…</option>
                  <option value="invented">Project-invented example data</option>
                  <option value="user">My own source</option>
                </select>
              </label>
            ) : null}
            {materials.map((material) => {
              const choice = Object.hasOwn(form, material) ? form[material] : NO_CHOICE;
              return (
                <div key={material} data-testid={`mbf-import-material-${material}`}>
                  <label>
                    {material}{" "}
                    <select
                      aria-label={`Source of ${material}`}
                      value={choice.choice}
                      onChange={(event) => changeForm((previous) => ({ ...previous, [material]: { ...choice, choice: event.target.value as Choice["choice"] } }))}
                    >
                      <option value="none">Not declared</option>
                      <option value="invented">Project-invented example data</option>
                      <option value="user">My own source</option>
                    </select>
                  </label>
                  {choice.choice === "user" ? (
                    <input
                      aria-label={`Source name of ${material}`}
                      value={choice.name}
                      maxLength={512}
                      onChange={(event) => changeForm((previous) => ({ ...previous, [material]: { choice: "user", name: event.target.value } }))}
                    />
                  ) : null}
                </div>
              );
            })}
            {uncut.length > 0 ? <p role="alert">These names are too long for the review to carry in full and cannot be declared: {uncut.join(", ")}.</p> : null}
          </fieldset>
        ) : null}
        <p role="status" data-testid="mbf-import-pending">{pending && state.phase !== "choosing" ? "Reviewing…" : ""}</p>
        <p data-testid="mbf-import-boundary">{MBF_IMPORT_BOUNDARY_SENTENCE}</p>
        <div className="report-actions">
          <button type="button" data-testid="mbf-import-designate" disabled={!current} onClick={designate}>Designate original source…</button>
          <button type="button" data-testid="mbf-import-adopt" disabled={!adoptable} onClick={adopt}>Adopt</button>
          <button type="button" data-testid="mbf-import-close" onClick={() => request.close()}>Close</button>
        </div>
      </section>
    </div>,
    document.body
  );
}

function NotShown({ review, list }: { review: MbfImportReviewPayload; list: keyof MbfImportReviewPayload["counts"]["lists"] }) {
  const counts = review.counts.lists[list];
  if (counts.not_shown === 0) return null;
  return <p data-testid={`mbf-import-${list}-not-shown`}>{counts.not_shown} more not shown{counts.not_shown_blocking > 0 ? `, ${counts.not_shown_blocking} of which block adoption` : ""}.</p>;
}

/** The bounded review (§4.3): source and binding, completeness and blocking codes, refusals, document check, mapping
 * assumptions with product descriptions, loss entries, target options, identities and units. No engine message or
 * assumption sentence is shown. */
function ImportReview({ reviewed }: { reviewed: Reviewed }) {
  const { review } = reviewed;
  const blocked = importBlockingCodes(review);
  const identityFile = review.source.identity_file;
  return (
    <div className="mbf-export-review" aria-live="polite" data-testid="mbf-import-review">
      <section data-testid="mbf-import-source">
        <h3>File</h3>
        <p>
          {reviewed.fileName} ({review.source.mbf_bytes} bytes). Identity file:{" "}
          {identityFile.state === "absent" ? "none beside it." : `present (${identityFile.bytes} bytes).`}
        </p>
        <p data-testid="mbf-import-binding">
          {review.binding.state === "bound"
            ? "Bound: the identity file matches these bytes, so the exported ids are restored."
            : review.binding.reason === "identity_file_for_other_bytes"
              ? "Unbound: the identity file beside it is for other bytes. Ids are formed from node numbers."
              : review.binding.reason === "no_identity_file"
                ? "Unbound: no identity file. Ids are formed from node numbers."
                : "Unbound: the identity file was not applied. Ids are formed from node numbers."}
        </p>
      </section>
      <section data-testid="mbf-import-completeness">
        <h3>Review</h3>
        <p data-testid="mbf-import-completeness-state">
          {review.completeness === "complete"
            ? "Complete: every refusal, loss entry and assumption is listed."
            : `Truncated: ${review.counts.not_shown} entries are not shown, ${review.counts.not_shown_blocking} of which block adoption.`}
        </p>
      </section>
      {blocked.length > 0 ? (
        <section role="alert" data-testid="mbf-import-blocked">
          <h3>Adopt is not offered</h3>
          <ul>{blocked.map((code) => <li key={code}>{code}: {importCodeText(code)}</li>)}</ul>
        </section>
      ) : null}
      {review.refusals.length > 0 || review.counts.lists.refusals.not_shown > 0 ? (
        <section data-testid="mbf-import-refusals">
          <h3>Refused (blocks adoption)</h3>
          <ul>
            {review.refusals.map((item, index) => (
              <li key={index}>{fieldText(item.code)}{item.line > 0 ? ` (line ${item.line}` : " ("}{fieldText(item.entity) ? `${item.line > 0 ? ", " : ""}${fieldText(item.entity)})` : ")"}: {importCodeText(fieldText(item.code))}</li>
            ))}
          </ul>
          <NotShown review={review} list="refusals" />
        </section>
      ) : null}
      {review.document_check.length > 0 ? (
        <section data-testid="mbf-import-document-check">
          <h3>Model document check (blocks adoption)</h3>
          <ul>{review.document_check.map((item, index) => <li key={index}>{fieldText(item.code)}{fieldText(item.path) ? ` at ${fieldText(item.path)}` : ""}</li>)}</ul>
        </section>
      ) : null}
      <section data-testid="mbf-import-assumptions">
        <h3>Mapping assumptions</h3>
        {review.assumptions.length > 0 ? (
          <ul>
            {review.assumptions.map((entry, index) => {
              const text = assumptionText(entry);
              return <li key={index} data-assumption={fieldText(entry.id)}>{text ?? `${fieldText(entry.id)}: no description is available, so the import cannot be adopted.`}</li>;
            })}
          </ul>
        ) : <p>None applied.</p>}
        <NotShown review={review} list="assumptions" />
      </section>
      {review.loss_entries.length > 0 ? (
        <section data-testid="mbf-import-loss">
          <h3>Loss report</h3>
          <ul>
            {review.loss_entries.map((item, index) => (
              <li key={index}>{fieldText(item.code)} ({fieldText(item.category)}{fieldText(item.entity) ? `, ${fieldText(item.entity)}` : ""}): {importCodeText(fieldText(item.code))}</li>
            ))}
          </ul>
          <NotShown review={review} list="loss_entries" />
        </section>
      ) : null}
      {review.target_options.length > 0 ? (
        <section data-testid="mbf-import-target-options">
          <h3>Options</h3>
          <ul>{review.target_options.map((item, index) => <li key={index}>{fieldText(item.name)}: Recorded, not applied.</li>)}</ul>
        </section>
      ) : null}
      <section data-testid="mbf-import-identity">
        <h3>Identities</h3>
        <p>{review.identity.basis === "restored_from_identity_file" ? "Identity file applied; each class lists restored and formed ids." : "Ids formed from node numbers."}</p>
        {review.identity.source_completeness === "established_for_identity_set" ? (
          <p data-testid="mbf-import-source-completeness">
            Established for the identity set: the designated document has exactly the same ids as this import in each class. This concerns ids only; it does not
            establish that the two models are equivalent, and the loss report still applies.
          </p>
        ) : (
          <p data-testid="mbf-import-source-completeness">These counts describe this import. They do not establish that every original source identity was preserved. The loss report also includes fields and other information the file cannot recover.</p>
        )}
        <ul>
          {review.identity.classes.filter((item) => item.count > 0).map((item) => (
            <li key={item.class}>{item.class.replace(/_/g, " ")}: {item.count} ({item.restored} restored, {item.formed_from_mbf_names} formed)</li>
          ))}
        </ul>
      </section>
      {review.identity.designation ? <Designation designation={review.identity.designation} /> : null}
      <section data-testid="mbf-import-units">
        <h3>Units</h3>
        <p>
          File: {fieldText(review.units.unit_mode)}, angles in {fieldText(review.units.angles)}, {fieldText(review.units.vertical_axis)} vertical.{" "}
          {review.units.project_units === "not_available"
            ? "The project units are not available."
            : `Project units: ${Object.entries(review.units.project_units).map(([name, value]) => `${name} ${fieldText(value)}`).join(", ")}.`}
        </p>
      </section>
    </div>
  );
}

/** The designated source the review states (§5.4): the file, the outcome and why it is not established. */
function Designation({ designation }: { designation: NonNullable<MbfImportReviewPayload["identity"]["designation"]> }) {
  return (
    <section data-testid="mbf-import-designation">
      <h3>Designated original source</h3>
      <p>
        {fieldText(designation.file_name)} ({designation.bytes} bytes):{" "}
        {designation.outcome === "established_for_identity_set"
          ? "this MBF and its identity file are exactly what SWBPIPE writes from it, and the import has the same ids in each class."
          : "the identity set is not established."}
      </p>
      {designation.reasons.length > 0 ? (
        <ul data-testid="mbf-import-designation-reasons">
          {designation.reasons.map((reason) => <li key={reason} data-reason={reason}>{designationReasonText(reason)}</li>)}
        </ul>
      ) : null}
      {designation.differing_classes.length > 0 ? (
        <p data-testid="mbf-import-designation-classes">Classes whose ids differ: {designation.differing_classes.map((name) => name.replace(/_/g, " ")).join(", ")}.</p>
      ) : null}
    </section>
  );
}
