import type { Diagnostic, PreviewModel } from "../../types";
import type { ModelCheckFinding, ModelCheckRun } from "../diagnostics/modelChecks";

// A run token is acquired synchronously before the first await. This closes the
// native-menu double-dispatch window and also lets model/open invalidation make
// every callback from an older solve inert, even when an adapter reuses a job
// identifier.
export class SolveRunGenerationGate {
  private generation = 0;
  private active: number | null = null;
  private cancelRequested = false;

  tryStart(): number | null {
    if (this.active !== null) return null;
    this.active = ++this.generation;
    this.cancelRequested = false;
    return this.active;
  }

  current(): number | null {
    return this.active;
  }

  isCurrent(token: number): boolean {
    return this.active === token;
  }

  requestCancellation(token: number): boolean {
    if (!this.isCurrent(token)) return false;
    this.cancelRequested = true;
    return true;
  }

  isCancellationRequested(token: number): boolean {
    return this.isCurrent(token) && this.cancelRequested;
  }

  invalidate(): void {
    this.generation += 1;
    this.active = null;
    this.cancelRequested = false;
  }

  finish(token: number): boolean {
    if (!this.isCurrent(token)) return false;
    this.active = null;
    this.cancelRequested = false;
    return true;
  }
}

export class RuleRevisionGenerationGate {
  private generation = 0;

  start(): number {
    return ++this.generation;
  }

  isCurrent(token: number): boolean {
    return token === this.generation;
  }

  invalidate(): void {
    this.generation += 1;
  }
}

export function commitModelAfterSolveInvalidation(
  gate: SolveRunGenerationGate,
  revision: { current: number },
  commit: () => void
): void {
  revision.current += 1;
  gate.invalidate();
  commit();
}

/** Whether the Solve command is offered for a model (DEL-07-07 REQ-008,
 * DEL-07-04 AC-001), read from the model checks (DEL-04-06 REQ-013), whose
 * findings equal the solver's leading diagnostics (DEL-04-06 AC-008):
 * - `open`: the checks ran on this model value and no solve-blocking finding stands;
 * - `unknown`: no check run is current for this model value, or the checks did
 *   not run; the Solve command proceeds as it would without the gate;
 * - `withheld`: a solve-blocking finding stands. Only the solve group (the
 *   solve's own input, restraint and topology checks) withholds; a slot or
 *   library-reference finding, and a rule-check-blocking finding, do not.
 * `findings` are the checks' own findings, unaltered: each `diagnostic` is the
 * record the checks returned, and `diagnostics` lists those same records in the
 * same order. `notListedBlocking` counts the solve group's blocking or error
 * findings the bounded report counted but did not list. `refusal` is the solve's
 * own text where it cannot read the model. `model` is the model value the checks
 * ran on (the run's own, which is the model the reading was asked for). */
export type SolveGateReading =
  | Readonly<{ kind: "open" }>
  | Readonly<{ kind: "unknown"; reason: "not_checked" | "model_changed" | "checks_unavailable" }>
  | Readonly<{
      kind: "withheld";
      model: PreviewModel;
      findings: readonly ModelCheckFinding[];
      diagnostics: readonly Diagnostic[];
      notListedBlocking: number;
      refusal: string | null;
    }>;

/** A finding's severity that withholds Solve when the solve group reports it. */
export function isSolveBlockingSeverity(severity: Diagnostic["severity"]): boolean {
  return severity === "blocking" || severity === "error";
}

/** The Solve command's gate for `model`, read from the session's last model-check
 * run. A run is current only for the model value it read (`run.model === model`,
 * the Model checks list's own staleness rule). Pure: it reads, and changes nothing. */
export function readSolveGate(model: PreviewModel | null, run: ModelCheckRun | null): SolveGateReading {
  if (!model || !run) return { kind: "unknown", reason: "not_checked" };
  if (run.model !== model) return { kind: "unknown", reason: "model_changed" };
  if (run.solve.kind === "unavailable") return { kind: "unknown", reason: "checks_unavailable" };
  const findings = run.findings.filter((finding) => finding.group === "solve" && isSolveBlockingSeverity(finding.diagnostic.severity));
  const notListedBlocking = run.notShownBlockingByGroup.solve;
  const refusal = run.solve.kind === "refused" ? run.solve.refusal : null;
  if (findings.length === 0 && notListedBlocking === 0 && refusal === null) return { kind: "open" };
  return { kind: "withheld", model: run.model, findings, diagnostics: findings.map((finding) => finding.diagnostic), notListedBlocking, refusal };
}

/** The reason a withheld Solve command states wherever it is offered (the
 * toolbar's Run, the Analyze menu, the Solve panel). It names how many findings
 * stand and where they are listed; the findings themselves are named by the
 * Solve panel's notice and the Model checks list. */
export function solveWithheldReason(reading: Extract<SolveGateReading, { kind: "withheld" }>): string {
  const count = reading.findings.length + reading.notListedBlocking;
  const parts: string[] = [];
  if (count > 0) parts.push(`${count} solve-blocking ${count === 1 ? "finding stands" : "findings stand"} for this model`);
  if (reading.refusal !== null) parts.push(`the solve cannot read this model (${reading.refusal})`);
  return `Solve is withheld: ${parts.join("; ")}. The findings are listed in Model checks, in the Issues drawer.`;
}
