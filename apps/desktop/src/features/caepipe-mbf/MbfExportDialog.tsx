import { useEffect, useLayoutEffect, useMemo, useRef, useState } from "react";
import { createPortal } from "react-dom";
import { canonicalJsonString } from "../../services/hashService";
import type { ModelHashEvidence, PreviewModel } from "../../types";
import {
  prepareModelDocumentSave,
  type LibraryResolution,
  type ModelFileDiagnostic
} from "../model-file/modelDocumentSave";
import {
  MBF_ASSUMPTIONS_SENTENCE,
  MBF_BOUNDARY_SENTENCE,
  MBF_EXPORT_INTENT_LABEL,
  MBF_EXPORT_OUTPUT_TOO_LARGE,
  MBF_EXPORT_REVIEW_STALE,
  MBF_EXPORT_ROUTE,
  MBF_OUTPUT_TOO_LARGE_SENTENCE,
  MBF_REVIEW_NATIVE_ONLY,
  MBF_REVIEW_NATIVE_ONLY_SENTENCE,
  MBF_REVIEW_TRUNCATED,
  MBF_REVIEW_TRUNCATED_SENTENCE,
  MBF_IDENTITY_FILE_SUFFIX,
  MBF_LOSS_DESCRIPTIONS,
  MBF_IDENTITY_SENTENCE,
  MBF_PROFILE_LABEL,
  MBF_PROFILE_OPEN_ITEMS,
  MBF_REPLACE_SENTENCE,
  MBF_UNDECLARED_MATERIAL_SENTENCE,
  defaultReferenceTemperature,
  familyLabel,
  lossDescription,
  mbfExportSettings,
  mbfReview,
  mbfSettingsJson,
  reviewAdmitsSave,
  reviewBlockingCodes,
  savedStatusLine,
  suggestedMbfName,
  type MbfExportSettings,
  type MbfReview,
  type MbfReviewEntry,
  type MbfReviewListCounts,
  type ReviewMbfExport,
  type SaveMbfExport
} from "./mbfExport";

/** The open Export MBF… dialog (DEL-17-04 Design `desktop-mbf-export-import.md` §3.1–§3.3, §3.9.5). Slice 4b.2 mounts
 * it from File > Export MBF… and the Exports section, and supplies `reviewMbfExport` and `saveMbfExport` (the native
 * `review_mbf_export` and `save_mbf_export`). */
export type MbfExportDialogRequest = {
  model: PreviewModel;
  modelHash: ModelHashEvidence | null;
  projectName: string;
  /** Focus returns here when the dialog closes. */
  opener: HTMLElement | null;
  close: () => void;
  saveMbfExport: SaveMbfExport;
  /** The native review (§3.9.5), or null in the browser-only route, which has no native review and no save: the
   * dialog then declines the review with `MBF-REVIEW-NATIVE-ONLY` and runs no writer. */
  reviewMbfExport: ReviewMbfExport | null;
  /** The model's library references resolved against the local store (`resolveLibraryDispositions`). */
  libraries?: LibraryResolution;
};

type ReviewState =
  | { key: string; phase: "running" }
  | { key: string; phase: "ready"; review: MbfReview }
  | { key: string; phase: "declined"; diagnostics: ModelFileDiagnostic[] }
  | { key: string; phase: "failed"; message: string };

const CLAIMED_HASH = /^sha256:[0-9a-f]{64}$/;

/** File > Export MBF…: the review of what the writer writes and leaves out, then the save of the model batch file
 * and its identity file through the native chooser. Modal like Save Model Document: Tab stays on its controls, window
 * shortcuts are held back, Escape closes, and focus returns to the opener. The review runs again whenever the model
 * or a setting changes; Save is available only for a current review of a written file, with the explicit choice. */
export function MbfExportDialog({ request }: { request: MbfExportDialogRequest | null }) {
  const dialog = useRef<HTMLElement>(null);
  const titleInput = useRef<HTMLInputElement>(null);
  const [title, setTitle] = useState("");
  const [temperature, setTemperature] = useState("");
  const [label, setLabel] = useState("");
  const [intent, setIntent] = useState(false);
  const [state, setState] = useState<ReviewState | null>(null);
  const [nonce, setNonce] = useState(0);
  const [saving, setSaving] = useState(false);
  const [refusal, setRefusal] = useState<ModelFileDiagnostic[]>([]);
  const [status, setStatus] = useState("");

  const temperatureUnit = defaultReferenceTemperature(request?.model.project?.units?.temperature).unit;
  // A new request (another model or project) starts from its defaults.
  useLayoutEffect(() => {
    if (!request) return;
    setTitle(request.projectName);
    setTemperature(String(defaultReferenceTemperature(request.model.project?.units?.temperature).value));
    setLabel("");
    setIntent(false);
    setRefusal([]);
    setStatus("");
    const opener = request.opener;
    titleInput.current?.focus();
    const hold = (event: KeyboardEvent) => {
      if (event.key === "Escape") {
        event.preventDefault();
        event.stopImmediatePropagation();
        request.close();
        return;
      }
      if (event.key === "Tab") {
        const controls = [...(dialog.current?.querySelectorAll<HTMLElement>("input, button:not(:disabled)") ?? [])];
        if (controls.length === 0) return;
        const at = controls.indexOf(document.activeElement as HTMLElement);
        const next = event.shiftKey ? (at <= 0 ? controls.length - 1 : at - 1) : (at + 1) % controls.length;
        event.preventDefault();
        event.stopImmediatePropagation();
        controls[next].focus();
        return;
      }
      if (event.metaKey || event.ctrlKey || event.altKey) {
        event.preventDefault();
        event.stopImmediatePropagation();
      }
    };
    window.addEventListener("keydown", hold, true);
    return () => {
      window.removeEventListener("keydown", hold, true);
      if (opener?.isConnected) requestAnimationFrame(() => opener.focus());
    };
  }, [request]);

  const settings: MbfExportSettings | null = useMemo(() => {
    const value = temperature.trim() === "" ? Number.NaN : Number(temperature);
    return Number.isFinite(value) ? mbfExportSettings(title, { value, unit: temperatureUnit }) : null;
  }, [title, temperature, temperatureUnit]);
  // The settings text the review runs with is the text the save sends.
  const settingsJson = settings ? mbfSettingsJson(settings) : null;
  // The review is current only for this model hash and these settings.
  const reviewKey = request && settingsJson ? `${request.modelHash?.value ?? "pending"}|${settingsJson}|${nonce}` : null;

  useEffect(() => {
    if (!request || !settingsJson || !reviewKey) {
      setState(null);
      return;
    }
    const review = request.reviewMbfExport;
    if (!review) {
      setState({ key: reviewKey, phase: "declined", diagnostics: [{ code: MBF_REVIEW_NATIVE_ONLY, path: "", message: MBF_REVIEW_NATIVE_ONLY_SENTENCE }] });
      return;
    }
    const claimedModelHash = request.modelHash?.value ?? "";
    if (!CLAIMED_HASH.test(claimedModelHash)) {
      setState({ key: reviewKey, phase: "running" });
      return;
    }
    let current = true;
    setState({ key: reviewKey, phase: "running" });
    void (async () => {
      try {
        const canonicalText = await canonicalJsonString(request.model);
        const outcome = await review({ canonicalText, claimedModelHash, settingsJson });
        if (!current) return;
        if (outcome.outcome === "reviewed") setState({ key: reviewKey, phase: "ready", review: mbfReview(outcome.review) });
        else setState({ key: reviewKey, phase: "declined", diagnostics: outcome.diagnostics });
      } catch (error) {
        if (current) setState({ key: reviewKey, phase: "failed", message: String(error instanceof Error ? error.message : error) });
      }
    })();
    return () => {
      current = false;
    };
  }, [request, settingsJson, reviewKey]);

  if (!request) return null;
  const review = state?.phase === "ready" && state.key === reviewKey ? state.review : null;
  const stale = state !== null && state.key !== reviewKey;
  const native = request.reviewMbfExport !== null;
  // Every current condition but the save in progress: a complete, eligible review of this model and these settings,
  // the explicit choice, and the native route.
  const permitted = reviewAdmitsSave(review) && intent && settingsJson !== null && native;
  const canSave = permitted && !saving;
  const suggestedName = suggestedMbfName(label);
  const identityName = suggestedName.replace(/\.mbf$/, MBF_IDENTITY_FILE_SUFFIX);

  const save = async () => {
    if (!canSave || !review || !settingsJson || !review.mbfSha256 || !review.sidecarSha256) return;
    setSaving(true);
    setRefusal([]);
    setStatus("");
    try {
      const prepared = await prepareModelDocumentSave({
        model: request.model,
        modelHash: request.modelHash,
        explicitLocalPrivateIntent: intent,
        libraries: request.libraries,
        route: MBF_EXPORT_ROUTE
      });
      if (prepared.status === "refused") {
        setRefusal(prepared.diagnostics);
        return;
      }
      const outcome = await request.saveMbfExport({
        ...prepared.args,
        settingsJson,
        suggestedName,
        reviewed: { mbfSha256: review.mbfSha256, sidecarSha256: review.sidecarSha256 }
      });
      if (outcome.outcome === "saved") {
        setStatus(savedStatusLine(outcome, review.lists.omitted.total));
      } else if (outcome.outcome === "refused") {
        setRefusal(outcome.diagnostics);
        // The save's own run formed its review: it replaces the one shown, and blocks Save while it blocks.
        if (outcome.review && reviewKey) setState({ key: reviewKey, phase: "ready", review: mbfReview(outcome.review) });
        // The model or settings moved since the review: review again.
        if (outcome.diagnostics.some((item) => item.code === MBF_EXPORT_REVIEW_STALE)) setNonce((value) => value + 1);
      }
    } catch (error) {
      setRefusal([{ code: "MBF-EXPORT-FAILED", path: "", message: String(error instanceof Error ? error.message : error) }]);
    } finally {
      setSaving(false);
    }
  };

  return createPortal(
    <div className="replace-project-backdrop" data-testid="mbf-export-backdrop">
      <section
        ref={dialog}
        role="dialog"
        aria-modal="true"
        aria-labelledby="mbf-export-title"
        aria-describedby="mbf-export-detail"
        className="replace-project-prompt panel mbf-export-dialog"
        data-testid="mbf-export-dialog"
      >
        <h2 id="mbf-export-title">Export Model Batch File</h2>
        <p id="mbf-export-detail">
          Writes the model of "{request.projectName}" as a model batch file (.mbf) and its identity file in a folder you
          choose, starting in your Documents folder. Both files hold your private project data. Nothing is sent anywhere,
          and the session's model and results are unchanged.
        </p>
        <fieldset className="mbf-export-settings">
          <legend>Settings</legend>
          <label>
            Title
            <input ref={titleInput} type="text" value={title} data-testid="mbf-export-title" onChange={(event) => setTitle(event.currentTarget.value)} />
          </label>
          <label>
            Reference temperature ({temperatureUnit === "degF" ? "°F" : "°C"})
            <input
              type="number"
              inputMode="decimal"
              value={temperature}
              data-testid="mbf-export-reference-temperature"
              onChange={(event) => setTemperature(event.currentTarget.value)}
            />
          </label>
          <label>
            File name label (optional)
            <input type="text" value={label} data-testid="mbf-export-label" onChange={(event) => setLabel(event.currentTarget.value)} />
          </label>
          <p data-testid="mbf-export-suggested-name">
            Suggested name: {suggestedName}. The identity file is saved beside it with the same name, as {identityName}.
          </p>
          {settings === null ? <p role="alert" data-testid="mbf-export-settings-invalid">Enter a reference temperature as a number.</p> : null}
        </fieldset>
        <div className="mbf-export-review" aria-live="polite" data-testid="mbf-export-review">
          {state === null ? null : stale || state.phase === "running" ? (
            <p data-testid="mbf-export-review-running">{stale ? "The model or a setting changed; the review runs again…" : "Reviewing what the file holds…"}</p>
          ) : state.phase === "failed" ? (
            <p role="alert" data-testid="mbf-export-review-failed">The review could not run: {state.message}</p>
          ) : state.phase === "declined" ? (
            <ul role="alert" data-testid="mbf-export-review-declined">
              {state.diagnostics.map((item, index) => (
                <li key={`${item.code}-${index}`}>{item.code}: {describeRefusal(item)}</li>
              ))}
            </ul>
          ) : review ? (
            <ReviewSections review={review} permitted={permitted} />
          ) : null}
        </div>
        <p data-testid="mbf-export-identity">{MBF_IDENTITY_SENTENCE}</p>
        <p data-testid="mbf-export-boundary">{MBF_BOUNDARY_SENTENCE}</p>
        <p data-testid="mbf-export-replace">{MBF_REPLACE_SENTENCE}</p>
        <label className="model-document-save-intent">
          <input type="checkbox" checked={intent} data-testid="mbf-export-intent" onChange={(event) => setIntent(event.currentTarget.checked)} />
          {MBF_EXPORT_INTENT_LABEL}
        </label>
        {refusal.length > 0 ? (
          <ul role="alert" data-testid="mbf-export-refusal">
            {refusal.map((item, index) => (
              <li key={`${item.code}-${index}`}>
                {/* A writer refusal is named by its code and the product-side description, not the engine's text; every
                    diagnostic of the native writer's refusal is an error. */}
                {item.code}{item.path ? ` (${item.path})` : ""}: {describeRefusal(item)}
              </li>
            ))}
          </ul>
        ) : null}
        <p role="status" data-testid="mbf-export-status">{status}</p>
        <div className="report-actions">
          <button type="button" data-testid="mbf-export-cancel" onClick={() => request.close()}>
            {status ? "Close" : "Cancel"}
          </button>
          <button type="button" data-testid="mbf-export-save" disabled={!canSave} onClick={() => void save()}>
            {saving ? "Saving…" : "Choose file and save…"}
          </button>
        </div>
      </section>
    </div>,
    document.body
  );
}

/** A code the writer emits (named by the product-side description), as opposed to the save's own codes. */
function isWriterCode(code: string): boolean {
  return code.startsWith("MBF-SI-") || Object.hasOwn(MBF_LOSS_DESCRIPTIONS, code);
}

/** A refusal's text: a writer code by its product-side description, a review condition by its sentence, else the
 * native message (the save's and the review's own text, never the engine's). */
function describeRefusal(item: ModelFileDiagnostic): string {
  if (isWriterCode(item.code)) return lossDescription(item.code, true);
  if (item.code === MBF_REVIEW_TRUNCATED) return MBF_REVIEW_TRUNCATED_SENTENCE;
  if (item.code === MBF_EXPORT_OUTPUT_TOO_LARGE) return MBF_OUTPUT_TOO_LARGE_SENTENCE;
  return item.message;
}

/** The entries of a list the review does not show, and how many of them block. */
function NotShown({ counts, testId }: { counts: MbfReviewListCounts; testId: string }) {
  if (counts.not_shown === 0) return null;
  return (
    <p data-testid={testId}>
      {counts.not_shown} more {counts.not_shown === 1 ? "entry is" : "entries are"} not shown
      {counts.not_shown_blocking > 0 ? `, ${counts.not_shown_blocking} of which block the save.` : "; none of them blocks the save on its own."}
    </p>
  );
}

const SENTENCES: Record<string, string> = {
  [MBF_REVIEW_TRUNCATED]: MBF_REVIEW_TRUNCATED_SENTENCE,
  [MBF_EXPORT_OUTPUT_TOO_LARGE]: MBF_OUTPUT_TOO_LARGE_SENTENCE
};

function EntryItem({ entry }: { entry: MbfReviewEntry }) {
  return (
    <li>
      {entry.code}{entry.entity ? ` (${entry.entity})` : ""}: {entry.description}
    </li>
  );
}

/** The review of §3.3: the profile by its product label, whether the review is complete and within the output
 * ceiling, each listed entry by its code, entity and product-side description with the count of those not shown, and
 * each unit conversion by its exact factor. The writer's own messages and assumptions are not shown. */
function ReviewSections({ review, permitted }: { review: MbfReview; permitted: boolean }) {
  const blocked = reviewBlockingCodes(review);
  return (
    <>
      <section data-testid="mbf-export-profile">
        <h3>Profile</h3>
        <p>
          {MBF_PROFILE_LABEL}. Open items: {MBF_PROFILE_OPEN_ITEMS.join(", ")} (open).
        </p>
      </section>
      <section data-testid="mbf-export-completeness">
        <h3>Review</h3>
        <p data-testid="mbf-export-completeness-state">
          {review.complete
            ? "Complete: every entry of the loss report is listed."
            : `Truncated: ${review.notShown} ${review.notShown === 1 ? "entry is" : "entries are"} not shown, ${review.notShownBlocking} of which block the save.`}
        </p>
        <p data-testid="mbf-export-ceiling">
          {review.outputWithinCeiling
            ? "The files are within the 64 MiB an export may write."
            : "The files would be larger than the 64 MiB an export may write."}
        </p>
      </section>
      {blocked.length > 0 ? (
        <section role="alert" data-testid="mbf-export-blocked">
          <h3>Not offered (blocks the save)</h3>
          <ul>
            {blocked.map((code) => <li key={code}>{code}: {SENTENCES[code]}</li>)}
          </ul>
        </section>
      ) : null}
      {review.notWritten.length > 0 ? (
        <section data-testid="mbf-export-not-written">
          <h3>Not written (blocks the save)</h3>
          <p>Save is unavailable while any part of the model is not written.</p>
          {review.notWritten.map((group) => (
            <div key={group.family} data-testid={`mbf-export-not-written-${group.family}`}>
              <h4>{familyLabel(group.family)}</h4>
              <ul>
                {group.entries.map((entry, index) => <EntryItem key={`${entry.entity}-${entry.code}-${index}`} entry={entry} />)}
              </ul>
            </div>
          ))}
          <NotShown counts={review.lists.not_written} testId="mbf-export-not-written-not-shown" />
        </section>
      ) : review.lists.not_written.not_shown > 0 ? (
        <section data-testid="mbf-export-not-written">
          <h3>Not written (blocks the save)</h3>
          <NotShown counts={review.lists.not_written} testId="mbf-export-not-written-not-shown" />
        </section>
      ) : null}
      {review.refusedUnexplained ? (
        <section data-testid="mbf-export-refused">
          <h3>Not written (blocks the save)</h3>
          <p>SWBPIPE refused to write the file without naming an entry. Save is unavailable.</p>
        </section>
      ) : null}
      {review.lists.omitted.total > 0 ? (
        <section data-testid="mbf-export-omitted">
          <h3>Omitted ({review.lists.omitted.total})</h3>
          <p>
            {review.status !== "written" || review.blocking
              ? "These would not be in the file either."
              : permitted
                ? "These are not in the file. You can still save."
                : "These are not in the file."}
          </p>
          <ul>
            {review.omitted.map((entry, index) => <EntryItem key={`${entry.entity}-${entry.code}-${index}`} entry={entry} />)}
          </ul>
          <NotShown counts={review.lists.omitted} testId="mbf-export-omitted-not-shown" />
        </section>
      ) : null}
      <section data-testid="mbf-export-conversions">
        <h3>Unit conversions</h3>
        {review.unitConversions.length > 0 ? (
          <ul>
            {review.unitConversions.map((item) => (
              <li key={item.unit}>
                {item.unit}: {item.definition}{item.factor ? ` (exact factor ${item.factor})` : ""}
              </li>
            ))}
          </ul>
        ) : (
          <p>None: the model's units are written as they are, or by powers of ten.</p>
        )}
        <p data-testid="mbf-export-assumptions">{MBF_ASSUMPTIONS_SENTENCE}</p>
      </section>
      {review.lists.warnings.total > 0 ? (
        <section data-testid="mbf-export-warnings">
          <h3>Warnings</h3>
          <ul>
            {review.warnings.map((entry, index) => <EntryItem key={`${entry.code}-${index}`} entry={entry} />)}
          </ul>
          <NotShown counts={review.lists.warnings} testId="mbf-export-warnings-not-shown" />
        </section>
      ) : null}
      {review.lists.undeclared_materials.total > 0 ? (
        <section data-testid="mbf-export-materials">
          <h3>Materials</h3>
          {review.undeclaredMaterials.length > 0 ? <p>{review.undeclaredMaterials.join(", ")}: {MBF_UNDECLARED_MATERIAL_SENTENCE}.</p> : null}
          <NotShown counts={review.lists.undeclared_materials} testId="mbf-export-materials-not-shown" />
        </section>
      ) : null}
    </>
  );
}
