// Model document 0.9.0 S3b (DEL-02-01 Work `model-document-0.9.0.md` §1.2, §1.3, §4.2, §11),
// owned by DEL-07-03: the load manager's "Conditions" control for one load case.
// - Pressure-free documents: a temperature key chosen from the keys present in the sets,
//   and the `added_weight: true` and `wind_exposure: true` marks. An unticked mark and an
//   unchosen key are absent ("not used"); `false` is never written. Pressure and contents
//   are shown refused by name and offer no input.
// - Documents with `pressure_contract`: the control is refused by name and offers no input.
// The whole ConditionUse object is written with `set_load_case_condition_use`, or removed
// with `clear_load_case_condition_use`, after the operation engine has checked it. The
// per-pipe resolution shown is the engine's own (`previewOperatingConditions`); the desktop
// computes no ΔT.
import { useEffect, useState } from "react";
import type { ConditionUse, PreviewModel } from "../../types";
import { previewOperatingConditions, type OperatingConditionPreview } from "../../services/operatingConditionPreviewService";
import { disabledBy, stableJson, useReseed } from "../model-data/modelDataIntents";
import { NOT_USED_BY_SOLVE, conditionKeys, conditionUseIntent, isPressureFree } from "./conditionSetIntents";
import { EngineFeedback, useEngineCheckedQueue, type EngineCheckedQueueProps } from "./engineCheckedQueue";

type LoadCase = PreviewModel["load_cases"][number];

export const REFUSED_EXACT = {
  code: "LOAD_CASE_CONDITION_USE_EXACT_UNSUPPORTED",
  text: "Condition use is refused on a document with a pressure contract until the exact route resolves it; no input is offered."
} as const;
export const REFUSED_PRESSURE = {
  code: "LOAD_CASE_CONDITION_PRESSURE_WITHOUT_PRESSURE_CONTRACT",
  text: "Pressure from conditions is refused: a pressure-free document has no pressure carrier."
} as const;
export const REFUSED_CONTENTS = {
  code: "LOAD_CASE_CONDITION_CONTENTS_UNSUPPORTED",
  text: `Contents density from conditions is refused; its consumer is not built (${NOT_USED_BY_SOLVE}).`
} as const;

const EDITED = new Set(["temperature", "added_weight", "wind_exposure"]);

type UseDraft = { temperature: string; addedWeight: boolean; windExposure: boolean };
const draftOf = (use: ConditionUse | undefined): UseDraft => ({
  temperature: typeof use?.temperature === "string" ? use.temperature : "",
  addedWeight: use?.added_weight === true,
  windExposure: use?.wind_exposure === true
});

/** The use the draft states. It starts from the stored use and changes only the members the
 * person changed; every other member keeps its exact stored value and presence, including a
 * stored value the engine refuses (for example `added_weight: false` or `temperature: null`),
 * so the engine judges it by name. A new use carries the chosen members only. */
export function useFromDraft(draft: UseDraft, stored: ConditionUse | undefined): ConditionUse {
  const base = draftOf(stored);
  const use: Record<string, unknown> = stored ? JSON.parse(JSON.stringify(stored)) : {};
  const put = (key: string, value: unknown) => { if (value === undefined) delete use[key]; else use[key] = value; };
  if (draft.temperature !== base.temperature) put("temperature", draft.temperature || undefined);
  if (draft.addedWeight !== base.addedWeight) put("added_weight", draft.addedWeight ? true : undefined);
  if (draft.windExposure !== base.windExposure) put("wind_exposure", draft.windExposure ? true : undefined);
  return use as ConditionUse;
}

/** Stored members the control edits whose stored value it cannot show (the engine refuses it). */
function refusedUseMembers(stored: ConditionUse | undefined): Array<[string, unknown]> {
  const record = (stored ?? {}) as Record<string, unknown>;
  return Object.entries(record).filter(([key, value]) => key === "temperature" ? typeof value !== "string" : (key === "added_weight" || key === "wind_exposure") && value !== true);
}

export function ConditionUseControl(props: EngineCheckedQueueProps & { loadCase: LoadCase | null }) {
  const { model, loadCase } = props;
  const stored = loadCase?.condition_use;
  const [draft, setDraft] = useState<UseDraft>(() => draftOf(stored));
  useReseed([loadCase?.id ?? "", stored ?? null], () => setDraft(draftOf(stored)));
  const queue = useEngineCheckedQueue(props);
  if (!loadCase) return null;
  const pressureFree = isPressureFree(model);
  const retained = [...Object.entries(stored ?? {}).filter(([key]) => !EDITED.has(key)), ...refusedUseMembers(stored)];
  const keys = conditionKeys(model);
  const next = useFromDraft(draft, stored);
  const empty = Object.keys(next).length === 0;
  const unchanged = stored !== undefined && stableJson(next) === stableJson(stored);
  return (
    <section className="load-condition-use" aria-label="Conditions" data-testid="load-condition-use">
      <h3>Conditions of {loadCase.label || loadCase.id}</h3>
      <p data-testid="load-condition-use-current">
        {stored ? `Stored use: ${describeUse(stored)}.` : "This case uses no conditions."}
      </p>
      {!pressureFree ? (
        <p role="note" data-testid="load-condition-use-refused-exact"><strong>Refused: {REFUSED_EXACT.code}</strong> — {REFUSED_EXACT.text}</p>
      ) : (
        <>
          <label>Temperature from condition key
            <select aria-label="Temperature condition key" data-testid="load-condition-use-temperature" value={draft.temperature}
              onChange={(event) => setDraft({ ...draft, temperature: event.target.value })}>
              <option value="">not used</option>
              {draft.temperature && !keys.includes(draft.temperature) ? <option value={draft.temperature}>{draft.temperature} (in no set)</option> : null}
              {keys.map((key) => <option key={key} value={key}>{key}</option>)}
            </select>
          </label>
          {keys.length === 0 ? <p className="muted" data-testid="load-condition-use-no-keys">No condition set states a condition key yet.</p> : null}
          <label>
            <input type="checkbox" aria-label="Added weight from condition sets" data-testid="load-condition-use-added-weight" checked={draft.addedWeight}
              onChange={(event) => setDraft({ ...draft, addedWeight: event.target.checked })} />
            Added weight from the sets (added_weight: true; unticked is absent, not used)
          </label>
          <label>
            <input type="checkbox" aria-label="Wind exposure from condition sets" data-testid="load-condition-use-wind-exposure" checked={draft.windExposure}
              onChange={(event) => setDraft({ ...draft, windExposure: event.target.checked })} />
            Wind exposure from the sets (wind_exposure: true; unticked is absent, not used)
          </label>
          <ul className="muted" aria-label="Uses refused on this document" data-testid="load-condition-use-refused">
            <li data-testid="load-condition-use-refused-pressure"><strong>Pressure: refused, {REFUSED_PRESSURE.code}</strong> — {REFUSED_PRESSURE.text}</li>
            <li data-testid="load-condition-use-refused-contents"><strong>Contents density: refused, {REFUSED_CONTENTS.code}</strong> — {REFUSED_CONTENTS.text}</li>
          </ul>
          <button type="button" data-testid="load-condition-use-queue"
            {...disabledBy(queue.busy, [empty, "Choose a temperature key or tick a mark; to stop using conditions, clear the use."], [unchanged, "The stored use is already this."])}
            onClick={() => void queue.submit((canonicalize) => conditionUseIntent(canonicalize, loadCase, next))}>
            Check and queue condition use
          </button>
        </>
      )}
      {retained.length ? (
        <p data-testid="load-condition-use-retained">
          <strong>Retained as authored</strong> (not edited here, or stored as a value the engine refuses): {retained.map(([key, value]) => `${key}: ${JSON.stringify(value)}`).join("; ")}.
          {pressureFree ? " It is written back with the use, and the engine judges it by name." : ""}
        </p>
      ) : null}
      <button type="button" data-testid="load-condition-use-clear" {...disabledBy(queue.busy, [stored === undefined, "This case uses no conditions."])}
        onClick={() => void queue.submit((canonicalize) => conditionUseIntent(canonicalize, loadCase, null))}>
        Check and queue clear
      </button>
      <EngineFeedback error={queue.error} notice={queue.notice} diagnostics={queue.diagnostics} testId="load-condition-use" />
      <ConditionResolutionPreview model={model} caseId={loadCase.id} />
    </section>
  );
}

export function describeUse(use: ConditionUse): string {
  return Object.entries(use).map(([key, value]) => `${key} ${typeof value === "string" ? value : JSON.stringify(value)}`).join(", ") || "empty";
}

/** The engine's resolution of the stored document for one case: the records the solve's
 * lowering appends (generated thermal ΔT, added weight) and the resolved wind-exposed pipes,
 * with the engine's findings naming this case. Read-only; nothing is computed here. */
export function ConditionResolutionPreview({ model, caseId }: { model: PreviewModel; caseId: string }) {
  const [state, setState] = useState<{ model: PreviewModel; preview?: OperatingConditionPreview; error?: string } | null>(null);
  useEffect(() => {
    let active = true;
    previewOperatingConditions(model)
      .then((preview) => { if (active) setState({ model, preview }); })
      .catch((error: unknown) => { if (active) setState({ model, error: error instanceof Error ? error.message : String(error) }); });
    return () => { active = false; };
  }, [model]);
  const current = state?.model === model ? state : null;
  const preview = current?.preview;
  const resolvedCase = preview?.cases.find((item) => item.case_id === caseId);
  const findings = preview ? [...(preview.diagnostics ?? [])].filter((item) => (item.affected_refs ?? []).includes(caseId) || !(item.affected_refs ?? []).length || (item.affected_refs ?? []).includes("model")) : [];
  return (
    <section aria-label="Engine resolution preview" data-testid="condition-resolution-preview">
      <h4>Engine resolution (stored model)</h4>
      <p className="muted">The operation engine's own resolution of the stored conditions for this case. It states what the solve would generate; it does not say the model solves.</p>
      {!current ? <p role="status">Resolving through the engine…</p> : null}
      {current?.error ? <p role="alert" data-testid="condition-resolution-preview-error">The engine could not resolve the conditions: {current.error}</p> : null}
      {preview?.refusal ? <p role="alert" data-testid="condition-resolution-preview-refusal">The engine could not read this document: {preview.refusal}</p> : null}
      {preview && !preview.refusal ? (
        <>
          <p data-testid="condition-resolution-preview-status">{preview.resolved ? "Resolved by the engine." : "Not resolved: the engine refuses the conditions below."}</p>
          {resolvedCase ? (
            <>
              <table data-testid="condition-resolution-preview-records">
                <caption>Generated loads for {caseId}</caption>
                <thead><tr><th scope="col">Pipe</th><th scope="col">Category</th><th scope="col">Magnitude</th><th scope="col">Direction</th><th scope="col">Dimension</th></tr></thead>
                <tbody>
                  {(resolvedCase.generated_primitive_loads ?? []).length === 0 ? <tr><td colSpan={5}>No generated loads.</td></tr> : null}
                  {(resolvedCase.generated_primitive_loads ?? []).map((record) => (
                    <tr key={record.id} data-testid={`condition-resolution-preview-record-${record.id}`}>
                      <td>{typeof record.target === "string" ? record.target : record.target.pipe}</td>
                      <td>{record.category}</td>
                      <td>{record.magnitude.value} {record.magnitude.unit}</td>
                      <td>{record.direction}</td>
                      <td>{record.dimension}</td>
                    </tr>
                  ))}
                </tbody>
              </table>
              <p data-testid="condition-resolution-preview-wind">
                {resolvedCase.wind_exposed_pipe_refs === null
                  ? "Wind exposure is not taken from the sets in this case."
                  : `Wind-exposed pipes from the sets: ${resolvedCase.wind_exposed_pipe_refs.length ? resolvedCase.wind_exposed_pipe_refs.join(", ") : "none"}.`}
              </p>
            </>
          ) : preview.resolved ? <p data-testid="condition-resolution-preview-none">This case uses no conditions.</p> : null}
          {findings.length ? (
            <ul aria-label="Engine resolution findings" data-testid="condition-resolution-preview-findings">
              {findings.map((item, index) => (
                <li key={`${item.code}:${index}`} data-testid={`condition-resolution-preview-finding-${item.code}`}>
                  <strong>{item.severity}: {item.code}</strong> — {item.message}
                </li>
              ))}
            </ul>
          ) : null}
        </>
      ) : null}
    </section>
  );
}
