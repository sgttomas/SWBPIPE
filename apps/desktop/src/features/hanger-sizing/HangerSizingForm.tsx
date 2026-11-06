/** Hanger sizing in the desktop (DEL-04-11 increment 3c part 2; Design `hanger-sizing.md` §2, §8.1, §13): the request
 * form, the native sizing (`services/hangerSizingService.ts`, worker task 4) and 3b's read-only table
 * (`HangerSizingPanel`), bound to the reply's document, its evidence and the library the request named.
 *
 * Every input is the person's own (REQ-001, AC-012): the form starts with no library, no candidate, no case and no
 * method choice, and nothing is pre-filled. `weight_support_state` is `own_solve`, the only admitted value, shown as a
 * fixed fact. The sizing never writes the model; a result is not recorded or saved. A newer request supersedes the
 * one in progress, and only the latest request's reply is shown, and only while this project session is current. */

import { useEffect, useMemo, useRef, useState } from "react";
import type { PreviewModel } from "../../types";
import { listLocalLibraries, openLocalLibrary, type LocalLibraryIndexEntry } from "../../services/libraryImportService";
import { retireHangerSizing, sizeHangers, type HangerSizingCompletion } from "../../services/hangerSizingService";
import { HangerSizingPanel } from "../results/HangerSizingPanel";
import type { HangerSizingFamilyPolicy } from "../results/hangerSizingResult";
import { VirtualMultiTargetPicker } from "../workspace/VirtualTargetPicker";
import { assembleHangerSizingInput, EMPTY_HANGER_SIZING_FORM, WEIGHT_SUPPORT_STATE, type HangerSizingFormState } from "./hangerSizingRequest";

export const HANGER_SIZING_FORM_TITLE = "Size hangers";
export const HANGER_SIZING_CLAIM_TEXT =
  "Sizing selects records from your own hanger library by the tests you enter here, in the native worker. It does not change the model, and a result is not saved or recorded. The result's own limitations are listed with it.";

const FAMILY_POLICIES: readonly [HangerSizingFamilyPolicy, string][] = [
  ["variable", "Variable spring"],
  ["constant", "Constant effort"],
  ["variable_else_constant", "Variable, else constant"]
];

type Props = Readonly<{
  model: PreviewModel;
  /** Another operation is in progress: the form is read-only meanwhile. */
  busy?: boolean;
}>;

type Shown = Readonly<{ completion: HangerSizingCompletion; library: Record<string, unknown> }>;
/** An opened library document, with the id it was opened for: a request sends it only while that id is the one
 * selected, so a late completion of an earlier selection can never stand in for the current one. */
type OpenedLibrary = Readonly<{ libraryId: string; document: Record<string, unknown> }>;

function Choice<T extends string>({ label, value, options, onChange, testId }: { label: string; value: T | ""; options: readonly [T, string][]; onChange: (value: T | "") => void; testId: string }) {
  return (
    <label>
      {label}
      <select value={value} data-testid={testId} onChange={(event) => onChange(event.target.value as T | "")}>
        <option value="">Choose…</option>
        {options.map(([option, text]) => <option key={option} value={option}>{text}</option>)}
      </select>
    </label>
  );
}

export function HangerSizingForm({ model, busy = false }: Props) {
  const [form, setForm] = useState<HangerSizingFormState>(EMPTY_HANGER_SIZING_FORM);
  const [libraries, setLibraries] = useState<LocalLibraryIndexEntry[]>([]);
  const [library, setLibrary] = useState<OpenedLibrary | null>(null);
  // Each selection's open carries its generation; only the latest selection's completion is applied.
  const librarySelection = useRef(0);
  const [notice, setNotice] = useState<string | null>(null);
  const [missing, setMissing] = useState<readonly string[]>([]);
  const [running, setRunning] = useState(false);
  const [shown, setShown] = useState<Shown | null>(null);
  const projectId = model.project.id;
  // A request in progress belongs to this mount (one project session): unmounting retires it.
  useEffect(() => () => { void retireHangerSizing(); }, []);

  const supportOptions = useMemo(() => model.supports.map((support) => ({ value: support.id, label: support.label || support.id, keywords: [support.node] })), [model.supports]);
  const caseOptions = useMemo(() => model.load_cases.map((loadCase) => [loadCase.id, loadCase.label ? `${loadCase.label} (${loadCase.id})` : loadCase.id] as [string, string]), [model.load_cases]);
  const update = (patch: Partial<HangerSizingFormState>) => { setForm((previous) => ({ ...previous, ...patch })); setMissing([]); };

  async function refreshLibraries() {
    setNotice(null);
    const result = await listLocalLibraries(projectId);
    if (result.route !== "tauri_backend") { setNotice(result.diagnostic); return; }
    setLibraries(result.entries.filter((entry) => entry.library_kind === "hanger"));
  }

  async function chooseLibrary(libraryId: string) {
    const generation = ++librarySelection.current;
    update({ libraryId });
    setLibrary(null);
    setNotice(null);
    if (!libraryId) return;
    const result = await openLocalLibrary(projectId, "hanger", libraryId);
    // A newer selection was made while this one opened: its result, success or failure, is dropped.
    if (generation !== librarySelection.current) return;
    if (result.route !== "tauri_backend") { setNotice(result.diagnostic); return; }
    if (!result.envelope) { setNotice("This library no longer exists."); return; }
    if (!result.envelope.validation.accepted || result.envelope.validation.has_blocking_findings) {
      setNotice("The stored hanger library has blocking import findings.");
      return;
    }
    setLibrary({ libraryId, document: result.envelope.document });
  }

  function chooseCandidates(supports: readonly string[]) {
    const policies = new Map(form.candidates.map((candidate) => [candidate.support, candidate.policy]));
    update({ candidates: supports.map((support) => ({ support, policy: policies.get(support) ?? "" })) });
  }

  async function size() {
    // Only the document opened for the selected id is sent; any other is no library at all.
    const selected = library !== null && library.libraryId === form.libraryId ? library.document : null;
    const assembled = assembleHangerSizingInput(form, model, selected);
    if (!assembled.ok) { setMissing(assembled.missing); return; }
    const requestLibrary = assembled.input.library;
    setRunning(true);
    const completion = await sizeHangers(assembled.input);
    // A stale completion (a newer request, a retirement) never replaces the shown result.
    if (!completion.current()) return;
    setRunning(false);
    setShown({ completion, library: requestLibrary });
  }

  async function cancel() {
    await retireHangerSizing();
    setRunning(false);
  }

  const reply = shown?.completion.kind === "replied" ? shown.completion : null;
  const failure = shown?.completion.kind === "failed" ? shown.completion : null;
  return (
    <section id="hanger-sizing" aria-label="Hanger sizing" data-testid="hanger-sizing-form">
      <h3>{HANGER_SIZING_FORM_TITLE}</h3>
      <p>{HANGER_SIZING_CLAIM_TEXT}</p>
      <fieldset disabled={busy || running}>
        <legend>Hanger library</legend>
        <button type="button" onClick={() => void refreshLibraries()}>Refresh hanger libraries</button>
        <label>
          Hanger library
          <select value={form.libraryId} data-testid="hanger-sizing-library" onChange={(event) => void chooseLibrary(event.target.value)}>
            <option value="">Choose…</option>
            {libraries.map((entry) => <option key={entry.library_id} value={entry.library_id}>{entry.library_name}</option>)}
          </select>
        </label>
      </fieldset>
      <fieldset disabled={busy || running}>
        <legend>Candidates</legend>
        <VirtualMultiTargetPicker label="Candidate supports" options={supportOptions} values={form.candidates.map((candidate) => candidate.support)} onChange={chooseCandidates} testId="hanger-sizing-candidates" />
        <ul>
          {form.candidates.map((candidate, index) => (
            <li key={candidate.support}>
              <Choice label={`Family policy of ${candidate.support}`} value={candidate.policy} options={FAMILY_POLICIES} testId={`hanger-sizing-policy-${index}`}
                onChange={(policy) => update({ candidates: form.candidates.map((other) => (other.support === candidate.support ? { support: other.support, policy } : other)) })} />
            </li>
          ))}
        </ul>
      </fieldset>
      <fieldset disabled={busy || running}>
        <legend>Cases</legend>
        <Choice label="Weight case" value={form.weightCase} options={caseOptions} testId="hanger-sizing-weight-case" onChange={(weightCase) => update({ weightCase })} />
        <p data-testid="hanger-sizing-weight-support-state">Support state the hot-load basis is formed in: the weight case's own solve (<code>{WEIGHT_SUPPORT_STATE}</code>, the only state admitted).</p>
        <Choice label="Design operating case" value={form.designOperatingCase} options={caseOptions} testId="hanger-sizing-design-case" onChange={(designOperatingCase) => update({ designOperatingCase })} />
        <fieldset>
          <legend>Further operating cases to check</legend>
          {caseOptions.map(([caseId, text]) => (
            <label key={caseId}>
              <input type="checkbox" checked={form.checkOperatingCases.includes(caseId)} onChange={(event) => update({
                checkOperatingCases: event.target.checked ? [...form.checkOperatingCases, caseId] : form.checkOperatingCases.filter((other) => other !== caseId),
                noCheckOperatingCases: false
              })} />
              {text}
            </label>
          ))}
          <label>
            <input type="checkbox" data-testid="hanger-sizing-no-check-cases" checked={form.noCheckOperatingCases}
              onChange={(event) => update({ noCheckOperatingCases: event.target.checked, checkOperatingCases: [] })} />
            None: check only the design operating case
          </label>
        </fieldset>
      </fieldset>
      <fieldset disabled={busy || running}>
        <legend>Method</legend>
        <Choice label="Load design" value={form.loadDesign} options={[["hot", "Hot (the spring carries the hot-load basis in the design operating case)"], ["cold", "Cold (the spring carries it in the cold condition)"]]} testId="hanger-sizing-load-design" onChange={(loadDesign) => update({ loadDesign })} />
        <Choice label="Variation reference" value={form.variationReference} options={[["operating_load", "Operating load"], ["design_load", "Design load"]]} testId="hanger-sizing-variation-reference" onChange={(variationReference) => update({ variationReference })} />
        <label>Variation limit (ratio)<input inputMode="decimal" value={form.variationLimit} data-testid="hanger-sizing-variation-limit" onChange={(event) => update({ variationLimit: event.target.value })} /></label>
        <label>Travel allowance at each end (m)<input inputMode="decimal" value={form.travelAllowanceM} data-testid="hanger-sizing-travel-allowance" onChange={(event) => update({ travelAllowanceM: event.target.value })} /></label>
        <Choice label="Selection rule" value={form.selectionRule} options={[["smallest_capacity", "Smallest capacity"], ["least_variation", "Least variation"]]} testId="hanger-sizing-selection-rule" onChange={(selectionRule) => update({ selectionRule })} />
        <Choice label="Solver mode" value={form.solverMode} options={[["sparse_interactive", "Sparse interactive"], ["dense_scrutiny", "Dense scrutiny"]]} testId="hanger-sizing-solver-mode" onChange={(solverMode) => update({ solverMode })} />
      </fieldset>
      <button type="button" disabled={busy || running} data-testid="hanger-sizing-run" onClick={() => void size()}>Size hangers</button>
      {running ? <><p role="status" data-testid="hanger-sizing-running">Sizing in the native worker…</p><button type="button" data-testid="hanger-sizing-cancel" onClick={() => void cancel()}>Cancel sizing</button></> : null}
      {notice !== null ? <p role="alert" data-testid="hanger-sizing-notice">{notice}</p> : null}
      {missing.length > 0 ? (
        <div role="alert" data-testid="hanger-sizing-missing">
          <p>The request is incomplete; nothing was sent:</p>
          <ul>{missing.map((item) => <li key={item}>{item}</li>)}</ul>
        </div>
      ) : null}
      {failure ? <p role="alert" data-testid="hanger-sizing-failure"><code>{failure.code}</code>: {failure.message}</p> : null}
      {reply && shown ? <HangerSizingPanel document={reply.document} evidence={reply.evidence} library={shown.library} /> : null}
    </section>
  );
}
