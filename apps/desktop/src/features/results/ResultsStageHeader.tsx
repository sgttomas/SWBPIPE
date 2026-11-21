/** The Results stage's header (DEL-07-05 REQ-011, REQ-023): what is being read and its standing,
 * held in view above the tables. It shows the run, the case or combination with its selector, the
 * result's standing line and, beneath it, one closed "Method and limits" that holds every method
 * sentence, assumption, exclusion and notice of the result, complete and as worded where they are
 * made. Nothing here forms a standing, a state or a word of one: the line, its hover sentence, the
 * per-case states and the support-state statements are passed in or read from the indicator's own
 * functions and constants. */
import { useLayoutEffect, useRef, type ReactNode } from "react";
import { ChevronRight } from "lucide-react";
import { checksPassedText, ELIGIBLE_RECOVERY_TEXT, NUMERICAL_HEALTH_STATE_TEXT, type NumericalHealthState } from "./numericalHealthText";
import { NUMERICAL_HEALTH_ORDER, rowHealthState, type NumericalHealthView } from "./numericalHealth";
import { needsSolverReportNote, SOLVER_REPORT_NOTE, type SupportStatesLine } from "./integrityStatus";
import type { ResultBasisRef } from "../../types";

/** Which run the page shows, in one terminology across the stage. */
export type ResultsRunKind = "current" | "reopened" | "reference";
export const RESULTS_RUN_LABELS: Readonly<Record<ResultsRunKind, string>> = Object.freeze({
  current: "Current run",
  reopened: "Reopened run (historical)",
  reference: "Bundled reference run (not a solve of this model)"
});
export type ResultsRunIdentity = Readonly<{ kind: ResultsRunKind; runId: string | null }>;

export const METHOD_AND_LIMITS_LABEL = "Method and limits";

/** The case the header reads: all cases, or one case or combination by its basis. */
export type HeaderCase = Readonly<{ basis: ResultBasisRef | null }> | null;

/** The cases in scope whose state the line does not state: each state word with its count, in the
 * indicator's order. Empty when every case in scope is in the line's state. */
export function differingCaseCounts(health: NumericalHealthView | null, lineState: NumericalHealthState | null, scope: HeaderCase): Array<readonly [NumericalHealthState, number]> {
  if (!health) return [];
  const states: NumericalHealthState[] = scope?.basis
    ? [rowHealthState(health, { basis_ref: scope.basis })].filter((state): state is NumericalHealthState => state !== null)
    : health.cases.map((item) => item.state);
  if (states.every((state) => state === lineState)) return [];
  return NUMERICAL_HEALTH_ORDER.map((state) => [state, states.filter((item) => item === state).length] as const).filter(([, count]) => count > 0);
}

/** The cases in scope the solver reports checks passed for, each with its sentence
 * (`checksPassedText`): the standing line's hover text and the first lines of "Method and limits". */
export function checksPassedSentences(health: NumericalHealthView | null, scope: HeaderCase): string[] {
  if (!health) return [];
  const cases = scope?.basis
    ? health.cases.filter((item) => scope.basis!.ref_type === "load_case" && item.caseId === scope.basis!.ref_id)
    : health.cases;
  return cases.filter((item) => item.state === "checks_passed").map((item) => checksPassedText(item.caseId));
}

/** The support-state line of each case in scope with nonlinear supports (`integrityStatus.ts`
 * `supportStatesLine`: its statement, and the solver's quoted reason with its note where the
 * statement says it is quoted below), one entry per distinct line with the cases it is stated for. */
export function supportStateLines(health: NumericalHealthView | null, scope: HeaderCase): Array<{ line: SupportStatesLine; cases: string[] }> {
  if (!health) return [];
  const lines = new Map<string, { line: SupportStatesLine; cases: string[] }>();
  for (const item of health.cases) {
    if (!item.supportStates) continue;
    if (scope?.basis && !(scope.basis.ref_type === "load_case" && scope.basis.ref_id === item.caseId)) continue;
    const key = JSON.stringify([item.supportStates.text, item.supportStates.detail, item.supportStates.detailNote]);
    const entry = lines.get(key) ?? { line: item.supportStates, cases: [] };
    entry.cases.push(item.caseId);
    lines.set(key, entry);
  }
  return [...lines.values()];
}

/** On the pressure-source route, the sentence that says which values the standing is about, where
 * the indicator gives it; it stays beside the standing. */
export function recoveryNote(health: NumericalHealthView | null): string | null {
  return health?.route === "pressure_source" && health.cases.some((item) => item.sentences.includes(ELIGIBLE_RECOVERY_TEXT)) ? ELIGIBLE_RECOVERY_TEXT : null;
}

export function ResultsStageHeader({
  identity, caseControl, actions, standing, lineState, health, scope, healthLine, method
}: {
  identity: ResultsRunIdentity;
  /** The case or combination selector. */
  caseControl: ReactNode;
  /** Controls of the stage that act on what is read (shown at the end of the identity row). */
  actions?: ReactNode;
  /** The standing line's text, as the route states it. */
  standing: ReactNode;
  /** The indicator state the line states, where the line states one. */
  lineState: NumericalHealthState | null;
  health: NumericalHealthView | null;
  scope: HeaderCase;
  /** The indicator's own header line, shown in full inside "Method and limits". */
  healthLine: ReactNode;
  /** Everything else the stage says about the method and the limits of the result, in order. */
  method: ReactNode;
}) {
  const sentences = checksPassedSentences(health, scope);
  const differing = differingCaseCounts(health, lineState, scope);
  const support = supportStateLines(health, scope);
  const recovery = recoveryNote(health);
  const ref = useRef<HTMLElement>(null);
  // Table headers under the stage stick just below this header: its height is published to the
  // stage as --results-header-h whenever it changes (Method and limits opened, a line wrapping).
  useLayoutEffect(() => {
    const header = ref.current;
    const stage = header?.closest<HTMLElement>(".results-panel");
    if (!header || !stage || typeof ResizeObserver === "undefined") return;
    const publish = () => stage.style.setProperty("--results-header-h", `${header.offsetHeight}px`);
    publish();
    const observer = new ResizeObserver(publish);
    observer.observe(header);
    return () => observer.disconnect();
  }, []);
  return (
    <header className="results-stage-header" data-testid="results-stage-header" ref={ref}>
      <div className="results-stage-identity">
        <span className="results-stage-run" data-testid="results-stage-run" data-run-kind={identity.kind}>
          {RESULTS_RUN_LABELS[identity.kind]}{identity.runId ? <code>{identity.runId}</code> : null}
        </span>
        <span className="results-stage-case">{caseControl}</span>
        {actions ? <span className="results-stage-actions">{actions}</span> : null}
      </div>
      <p className="result-standing-line" data-testid="numerical-result-standing" title={sentences.length > 0 ? sentences.join("\n") : undefined}>{standing}</p>
      {differing.length > 0 || health?.outputSentence ? (
        <p className="results-stage-differs" data-testid="results-stage-case-states">
          {differing.length > 0 ? <span>By load case: {differing.map(([state, count]) => `${count} ${NUMERICAL_HEALTH_STATE_TEXT[state]}`).join(", ")}.</span> : null}
          {health?.outputSentence ? <span> {health.outputSentence}</span> : null}
        </p>
      ) : null}
      {recovery ? <p className="results-stage-note" data-testid="results-stage-recovery-note">{recovery}</p> : null}
      {/* Each line's statement is an element of its own, and the solver's quoted reason with its note
          stands directly beneath it, as in the indicator's detail. */}
      {support.map(({ line, cases }) => (
        <div key={`${line.text}|${line.detail ?? ""}`} className="results-stage-support" data-testid="results-stage-support-states">
          <p className="results-stage-note" data-testid="results-stage-support-states-text">
            {line.text}{scope?.basis ? null : <span className="results-stage-note-cases"> ({cases.join(", ")})</span>}
          </p>
          {line.detail ? (
            <blockquote className="results-stage-support-detail" data-testid="results-stage-support-states-detail">
              {line.detail}{line.detailNote ? <span data-testid="results-stage-support-states-detail-note"> {line.detailNote}</span> : null}
            </blockquote>
          ) : null}
        </div>
      ))}
      <details className="results-method" data-testid="results-method">
        <summary><ChevronRight size={14} aria-hidden="true" className="results-method-chevron" />{METHOD_AND_LIMITS_LABEL}</summary>
        <div className="results-method-body">
          {sentences.map((text) => (
            <p key={text} data-testid="results-method-standing-sentence">
              {text}{needsSolverReportNote({ message: text }) ? <span data-testid="results-method-solver-report-note"> {SOLVER_REPORT_NOTE}</span> : null}
            </p>
          ))}
          {healthLine}
          {method}
        </div>
      </details>
    </header>
  );
}
