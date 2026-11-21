// RT-A4 / RT-A9: the three-axis static seismic template in the load manager (see
// `seismicTemplate.ts`). The person chooses every case and the unsigned basis; the preview
// states exactly what will be written, the κ = 0 differences, the unsigned combination and
// the solve's known gates (§5.1, §5.2). "Check and queue" has the operation engine check the
// four creates as one sequence (validate-only on the current model), reports its findings in
// place by code, and queues the four changes, in order, for Review/Apply (singly, or together
// through "Review queued changes as one batch"). Nothing here writes the model.
import { useEffect, useRef, useState } from "react";
import { ListPlus } from "lucide-react";
import { computeModelHash } from "../../services/hashService";
import { validateOperationBatch } from "../../services/operationBatchService";
import type { EditorOperationIntent, OperationOutcomeDiagnostic, PreviewModel } from "../../types";
import { EngineFeedback, isBlocking } from "../condition-sets/engineCheckedQueue";
import { VirtualTargetPicker } from "../workspace/VirtualTargetPicker";
import { NONLINEAR_MECHANICS_TEXT, constantEffortSupportIds, nonlinearSupportIds } from "./combinationAuthoring";
import { More } from "./LoadsPageParts";
import {
  SEISMIC_AXES,
  SEISMIC_TEMPLATE_APPLY_TEXT,
  SEISMIC_TEMPLATE_LABEL,
  emptySeismicTemplateDraft,
  proposeSeismicIds,
  seismicTemplateIntents,
  seismicTemplatePayloads,
  seismicTemplateProblem,
  type SeismicAxis,
  type SeismicTemplateDraft
} from "./seismicTemplate";

type Props = {
  model: PreviewModel;
  onQueueIntent: (intent: EditorOperationIntent) => void;
  queuedIntents?: EditorOperationIntent[];
  operationBusy?: boolean;
  preparationEpoch?: number;
  getPreparationEpoch?: () => number;
  /** Opens the page's New load case form and focuses its first field. */
  onCreateLoadCase?: () => void;
  /** Ids another draft in the panel proposes, so the template proposes different ones. */
  avoidIds?: readonly string[];
};

const AXIS_NAME: Record<SeismicAxis, string> = { x: "X", y: "Y", z: "Z" };

export function SeismicTemplateSection(props: Props) {
  const { model, operationBusy } = props;
  const [draft, setDraft] = useState<SeismicTemplateDraft>(() => emptySeismicTemplateDraft(model, props.avoidIds));
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState("");
  const [notice, setNotice] = useState("");
  const [diagnostics, setDiagnostics] = useState<OperationOutcomeDiagnostic[]>([]);
  const [createLoadCaseNotice, setCreateLoadCaseNotice] = useState("");
  const latest = useRef(props);
  latest.current = props;
  // The draft as it stands now: a check started on an earlier draft queues nothing.
  const latestDraft = useRef(draft);
  latestDraft.current = draft;
  const live = useRef(false);
  useEffect(() => { live.current = true; return () => { live.current = false; }; }, []);

  // As the create-combination draft does: a model update that takes a drafted id re-proposes
  // the four; a chosen case that is no longer present is cleared, never replaced.
  useEffect(() => {
    setDraft((current) => {
      const exists = (id: string) => model.load_cases.some((loadCase) => loadCase.id === id);
      const taken = (id: string) => (model.combinations ?? []).some((combination) => combination.id === id);
      const ids = [...Object.values(current.differenceIds), current.combinationId];
      const next = {
        ...current,
        baseId: exists(current.baseId) ? current.baseId : "",
        axisIds: { x: exists(current.axisIds.x) ? current.axisIds.x : "", y: exists(current.axisIds.y) ? current.axisIds.y : "", z: exists(current.axisIds.z) ? current.axisIds.z : "" },
        ...(ids.some(taken) ? proposeSeismicIds(model, latest.current.avoidIds) : {})
      };
      return JSON.stringify(next) === JSON.stringify(current) ? current : next;
    });
  }, [model]);

  const loadCaseOptions = model.load_cases.map((loadCase) => ({ value: loadCase.id, label: loadCase.label || loadCase.id }));
  const problem = seismicTemplateProblem(model, draft);
  const constantEffort = constantEffortSupportIds(model);
  const nonlinear = nonlinearSupportIds(model);
  const epoch = (current: Props) => current.getPreparationEpoch?.() ?? current.preparationEpoch ?? 0;

  function reset() { setError(""); setNotice(""); setDiagnostics([]); }

  async function checkAndQueue() {
    if (busy || operationBusy || problem) return;
    const captured = props;
    const capturedEpoch = epoch(captured);
    const capturedDraft = draft;
    const intents = seismicTemplateIntents(captured.model, capturedDraft);
    setBusy(true);
    reset();
    try {
      const refs = new Set(intents.map((intent) => intent.target.ref));
      const pending = captured.queuedIntents?.find((queued) => queued.target.object_type === "Combination" && refs.has(queued.target.ref));
      if (pending) throw new Error(`Review or remove the pending change for ${pending.target.ref} before queuing the template.`);
      const outcome = await validateOperationBatch(captured.model, { batch_id: `seismic-template-${draft.combinationId.trim()}`, operations: intents }, await computeModelHash(captured.model));
      const current = latest.current;
      if (!live.current) return;
      if (epoch(current) !== capturedEpoch || current.model !== captured.model || current.operationBusy || current.queuedIntents !== captured.queuedIntents) {
        throw new Error("The model or pending changes changed. Review this draft and queue it again.");
      }
      if (latestDraft.current !== capturedDraft) {
        throw new Error("The template changed while the operation engine was checking it; nothing was queued. Review the preview and check again.");
      }
      setDiagnostics(outcome.diagnostics);
      if (outcome.diagnostics.some(isBlocking) || outcome.validation.batch_validation_status !== "passed") {
        setError("The template was refused by the operation engine; nothing was queued.");
        return;
      }
      for (const intent of intents) current.onQueueIntent(intent);
      setNotice(
        `Checked by the operation engine as one sequence and queued as four changes for Review/Apply: ${intents.map((intent) => intent.target.ref).join(", ")}. ` +
          `${SEISMIC_TEMPLATE_APPLY_TEXT} Applied singly, apply them in this order: ${capturedDraft.combinationId.trim()} names the three differences, so applied before them the engine refuses it (OP-COMBINATION-TERM-LOAD-NOT-FOUND).`
      );
    } catch (caught) {
      if (live.current) setError(caught instanceof Error ? caught.message : String(caught));
    } finally {
      if (live.current) setBusy(false);
    }
  }

  const payloads = problem ? null : seismicTemplatePayloads(draft);
  const setAxis = (axis: SeismicAxis, value: string) => setDraft((current) => ({ ...current, axisIds: { ...current.axisIds, [axis]: value } }));
  const setFactor = (axis: SeismicAxis, value: string) => setDraft((current) => ({ ...current, factors: { ...current.factors, [axis]: value } }));
  const setDifferenceId = (axis: SeismicAxis, value: string) => setDraft((current) => ({ ...current, differenceIds: { ...current.differenceIds, [axis]: value } }));

  return (
    <section className="combination-create-editor seismic-template" aria-label={`${SEISMIC_TEMPLATE_LABEL} template`} data-testid="seismic-template">
      <p className="muted" data-testid="seismic-template-cases-note">
        Choose a base case and, for each axis, a case holding the base's loads plus that axis's static excitation; the template creates no load case.{" "}
        {props.onCreateLoadCase ? (
          <button
            type="button"
            data-testid="seismic-template-create-load-case"
            onClick={() => {
              props.onCreateLoadCase?.();
              setCreateLoadCaseNotice("New load case is open above with its label field focused. Create the axis-state case there, then enter the base's loads and the excitation in that case's Loads.");
            }}
          >
            New load case
          </button>
        ) : null}
      </p>
      {createLoadCaseNotice ? <p role="status" className="muted" data-testid="seismic-template-create-load-case-status">{createLoadCaseNotice}</p> : null}
      <fieldset className="combination-create-controls" disabled={busy} aria-busy={busy} data-testid="seismic-template-inputs">
        <legend className="visually-hidden">Template inputs</legend>
        <VirtualTargetPicker label="Seismic base case" testId="seismic-template-base" options={loadCaseOptions} value={draft.baseId} onChange={(value) => setDraft((current) => ({ ...current, baseId: value }))} />
        {SEISMIC_AXES.map((axis) => (
          <VirtualTargetPicker key={axis} label={`${AXIS_NAME[axis]} axis-state case (base + ${AXIS_NAME[axis]} excitation)`} testId={`seismic-template-axis-${axis}`} options={loadCaseOptions} value={draft.axisIds[axis]} onChange={(value) => setAxis(axis, value)} />
        ))}
        <label>
          <span>Unsigned combination</span>
          <select aria-label="Seismic unsigned combination basis" data-testid="seismic-template-basis" value={draft.basis} onChange={(event) => setDraft((current) => ({ ...current, basis: event.target.value as SeismicTemplateDraft["basis"] }))}>
            <option value="">Choose…</option>
            <option value="srss">srss (square root of the sum of squares)</option>
            <option value="absolute_sum">absolute_sum</option>
          </select>
        </label>
        {SEISMIC_AXES.map((axis) => (
          <label key={`factor-${axis}`}>
            <span>{AXIS_NAME[axis]} difference factor</span>
            <input aria-label={`${AXIS_NAME[axis]} seismic difference factor`} data-testid={`seismic-template-factor-${axis}`} inputMode="decimal" value={draft.factors[axis]} onChange={(event) => setFactor(axis, event.target.value)} />
          </label>
        ))}
        <More testId="seismic-template-more">
          {SEISMIC_AXES.map((axis) => (
            <label key={axis}>
              <span>{AXIS_NAME[axis]} difference id</span>
              <input aria-label={`${AXIS_NAME[axis]} seismic difference id`} data-testid={`seismic-template-difference-id-${axis}`} value={draft.differenceIds[axis]} onChange={(event) => setDifferenceId(axis, event.target.value)} />
            </label>
          ))}
          <label>
            <span>Combination id</span>
            <input aria-label="Seismic combination id" data-testid="seismic-template-combination-id" value={draft.combinationId} onChange={(event) => setDraft((current) => ({ ...current, combinationId: event.target.value }))} />
          </label>
        </More>
        <button className="loads-primary-button" type="button" data-testid="seismic-template-queue" disabled={Boolean(problem) || busy || Boolean(operationBusy)} title={problem ?? "Check the four combinations with the operation engine and queue them"} aria-describedby="seismic-template-status" onClick={() => void checkAndQueue()}>
          <ListPlus size={14} aria-hidden="true" />
          Check and queue
        </button>
      </fieldset>
      <p className="muted" id="seismic-template-status" data-testid="seismic-template-status">
        {problem ? `Not ready: ${problem}.` : busy ? "Checking with the operation engine…" : "Ready: review the preview, then check and queue."}
      </p>
      <div className="seismic-template-preview" data-testid="seismic-template-preview" aria-label="Seismic template preview">
        {payloads ? (
          <ol data-testid="seismic-template-records">
            {payloads.map((payload, index) => (
              <li key={payload.id} data-testid={`seismic-template-record-${index}`}>
                {index < 3
                  ? `${payload.id}: result_state_subtraction, ${String(payload.minuend_id)} − ${String(payload.subtrahend_id)} (κ = 0; no difference_basis)`
                  : `${payload.id}: ${String(payload.basis)} over ${(payload.terms as Array<{ combination: string; factor: number }>).map((term) => `combination ${term.combination} x ${term.factor}`).join(", ")} (unsigned; not an equilibrium state)`}
              </li>
            ))}
          </ol>
        ) : null}
        {constantEffort.length ? (
          <p role="note" data-testid="seismic-template-gate-fixed-load">
            This model carries constant-effort support{constantEffort.length > 1 ? "s" : ""} {constantEffort.join(", ")}; the solve decides whether it consumes {constantEffort.length > 1 ? "them" : "it"}.
          </p>
        ) : null}
        {nonlinear.length ? (
          <p role="note" data-testid="seismic-template-gate-nonlinear">
            This model carries nonlinear support{nonlinear.length > 1 ? "s" : ""} {nonlinear.join(", ")}. {NONLINEAR_MECHANICS_TEXT}
          </p>
        ) : null}
      </div>
      <EngineFeedback error={error} notice={notice} diagnostics={diagnostics} testId="seismic-template-engine" />
    </section>
  );
}
