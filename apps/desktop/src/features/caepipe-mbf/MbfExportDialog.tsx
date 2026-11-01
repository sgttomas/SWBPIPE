import { useEffect, useLayoutEffect, useMemo, useRef, useState } from "react";
import { createPortal } from "react-dom";
import { canonicalJsonString } from "../../services/hashService";
import { mbfSettingsJson, renderMbfExport, type MbfExportSettings, type WasmMbfEngine } from "../../services/mbfEngine";
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
  MBF_EXPORT_REVIEW_STALE,
  MBF_EXPORT_ROUTE,
  MBF_IDENTITY_FILE_SUFFIX,
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
  reviewAdmitsSave,
  savedStatusLine,
  suggestedMbfName,
  type MbfReview,
  type MbfReviewEntry,
  type SaveMbfExport
} from "./mbfExport";

/** The open Export MBF… dialog (DEL-17-04 Design `desktop-mbf-export-import.md` §3.1–§3.3). Slice 4b mounts it from
 * File > Export MBF… and the Exports section, and supplies `saveMbfExport` (the native `save_mbf_export`). */
export type MbfExportDialogRequest = {
  model: PreviewModel;
  modelHash: ModelHashEvidence | null;
  projectName: string;
  /** Focus returns here when the dialog closes. */
  opener: HTMLElement | null;
  close: () => void;
  saveMbfExport: SaveMbfExport;
  /** The model's library references resolved against the local store (`resolveLibraryDispositions`). */
  libraries?: LibraryResolution;
  /** The wasm writer; the loaded `public/mbf-engine` build by default. */
  engine?: WasmMbfEngine;
};

type ReviewState =
  | { key: string; phase: "running" }
  | { key: string; phase: "ready"; review: MbfReview }
  | { key: string; phase: "failed"; message: string };

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
    let current = true;
    setState({ key: reviewKey, phase: "running" });
    void (async () => {
      try {
        const text = await canonicalJsonString(request.model);
        const outcome = await renderMbfExport(text, settingsJson, request.engine);
        if (!current) return;
        if (outcome.status === "failed") setState({ key: reviewKey, phase: "failed", message: `${outcome.failure.code}: ${outcome.failure.message}` });
        else setState({ key: reviewKey, phase: "ready", review: mbfReview(outcome) });
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
  const canSave = reviewAdmitsSave(review) && intent && !saving && settingsJson !== null;
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
        setStatus(savedStatusLine(outcome, review.omitted.length));
      } else if (outcome.outcome === "refused") {
        setRefusal(outcome.diagnostics);
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
          ) : review ? (
            <ReviewSections review={review} />
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
                {/* A writer refusal is named by its code and the product-side description, not the engine's text. */}
                {item.code}{item.path ? ` (${item.path})` : ""}: {item.code.startsWith("MBF-SI-") ? lossDescription(item.code) : item.message}
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

function EntryItem({ entry }: { entry: MbfReviewEntry }) {
  return (
    <li>
      {entry.code}{entry.entity ? ` (${entry.entity})` : ""}: {entry.description}
    </li>
  );
}

/** The review of §3.3: the profile by its product label, each loss entry by its code, entity and product-side
 * description, and each unit conversion by its exact factor. The writer's own messages and assumptions are not shown. */
function ReviewSections({ review }: { review: MbfReview }) {
  return (
    <>
      <section data-testid="mbf-export-profile">
        <h3>Profile</h3>
        <p>
          {MBF_PROFILE_LABEL}. Open items: {MBF_PROFILE_OPEN_ITEMS.join(", ")} (open).
        </p>
      </section>
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
        </section>
      ) : null}
      {review.omitted.length > 0 ? (
        <section data-testid="mbf-export-omitted">
          <h3>Omitted ({review.omitted.length})</h3>
          <p>These are not in the file. You can still save.</p>
          <ul>
            {review.omitted.map((entry, index) => <EntryItem key={`${entry.entity}-${entry.code}-${index}`} entry={entry} />)}
          </ul>
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
      {review.warnings.length > 0 ? (
        <section data-testid="mbf-export-warnings">
          <h3>Warnings</h3>
          <ul>
            {review.warnings.map((entry, index) => <EntryItem key={`${entry.code}-${index}`} entry={entry} />)}
          </ul>
        </section>
      ) : null}
      {review.undeclaredMaterials.length > 0 ? (
        <section data-testid="mbf-export-materials">
          <h3>Materials</h3>
          <p>{review.undeclaredMaterials.join(", ")}: {MBF_UNDECLARED_MATERIAL_SENTENCE}.</p>
        </section>
      ) : null}
    </>
  );
}
