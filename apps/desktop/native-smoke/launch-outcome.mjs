// The rule for judging a native-smoke launch (DEL-07-11 VER-008). Common to every runner that uses it: the app
// ended with exit status 0, no signal and no timeout, and the run's fresh evidence root holds a readable report
// carrying the nonce the runner gave that launch. A runner whose contract requires a census adds one: no process of
// the run's private app copy remains. `run.mjs` and `mbf-journey.mjs` require it (`judged-launch.mjs`); `perf.mjs`
// requires it too, taking its census after each launch and applying the rule in its classification and its kept-record
// judgment (`recordedCensusFailures`). Each failed condition is
// named. A failed launch never reads as a pass; its report, when there is one, stays readable for the observations
// it records. Kept evidence is judged from its recorded outcome: none, or an incomplete one, is not a pass.
import { randomBytes } from "node:crypto";
import { existsSync, readFileSync } from "node:fs";

/** A fresh nonce for one run's plan; the driver hands it back in its report. */
export function newNonce() {
  return randomBytes(16).toString("hex");
}

/** The report at `file`: { report } when it parses, { report: null, problem } otherwise. */
export function readReport(file) {
  if (!existsSync(file)) return { report: null, problem: "no report" };
  try {
    const report = JSON.parse(readFileSync(file, "utf8"));
    if (report === null || typeof report !== "object" || Array.isArray(report)) return { report: null, problem: "the report is not a JSON object" };
    return { report, problem: null };
  } catch (error) {
    return { report: null, problem: `the report does not parse (${error.message})` };
  }
}

/** The named failures of one launch; empty when it passed. `census` is `awaitNoProcesses`'s result, or
 * undefined where the runner takes no census. */
export function launchFailures({ status, signal, timedOut, census, report, problem = null, nonce }) {
  const failures = [];
  if (timedOut) failures.push("LAUNCH-TIMED-OUT: the app was killed at the runner's time limit");
  if (signal !== null && signal !== undefined) failures.push(`LAUNCH-SIGNALLED: the app ended on ${signal}`);
  if (status !== 0) failures.push(`LAUNCH-EXIT-NOT-ZERO: the app ended with status ${status === null || status === undefined ? "none" : status}`);
  if (census !== undefined && census?.resolved !== true) failures.push(`LAUNCH-CENSUS-UNRESOLVED: ${census?.reason ?? "no census"}`);
  if (report === null || report === undefined) failures.push(`LAUNCH-NO-REPORT: ${problem ?? "no report"}`);
  else if (typeof nonce !== "string" || nonce.length === 0) failures.push("LAUNCH-NO-NONCE: the runner gave this run no nonce");
  else if (report.nonce !== nonce) failures.push(`LAUNCH-REPORT-NOT-THIS-RUN: the report's nonce ${JSON.stringify(report.nonce ?? null)} is not this run's`);
  return failures;
}

/** The launch outcome a summary records, so that kept evidence can be judged again without a launch. */
export function recordedOutcome({ status, signal, timedOut, census, nonce, started, ended }) {
  return { status: status ?? null, signal: signal ?? null, timed_out: Boolean(timedOut), census: census ?? null, nonce: nonce ?? null, started: started ?? null, ended: ended ?? null };
}

const NOT_RECORDED = "LAUNCH-OUTCOME-NOT-RECORDED: this evidence records no complete launch outcome, so it cannot be judged a pass";
// A recorded outcome is complete only when it states how the app ended: its status, its signal (null when none)
// and whether the runner's time limit stopped it. A missing field is never read as a clean ending.
const completeOutcome = (outcome) => outcome !== null && typeof outcome === "object" && !Array.isArray(outcome)
  && Object.hasOwn(outcome, "status") && Object.hasOwn(outcome, "signal") && typeof outcome.timed_out === "boolean";

/** The named failures of kept evidence from a runner that takes no census: its recorded outcome judged with its
 * report. Evidence that recorded no complete outcome is LAUNCH-OUTCOME-NOT-RECORDED, whatever its report says.
 * Not for `perf.mjs`'s records, which are census-bearing: they are judged by `recordedCensusFailures`. */
export function recordedFailures(outcome, report, problem = null) {
  if (!completeOutcome(outcome)) return [NOT_RECORDED];
  return launchFailures({ status: outcome.status, signal: outcome.signal, timedOut: outcome.timed_out, census: undefined, report, problem, nonce: outcome.nonce });
}

/** The named failures of one phase of kept evidence from a runner that takes a census (`run.mjs`,
 * `mbf-journey.mjs`), from the run's `summary.json` (its `launches` record) and the phase's report. A summary
 * written before runners recorded launch outcomes, or an incomplete outcome, is not a pass.
 * The runner's contract decides whether a census is required; its absence from a stored record does not. These
 * runners require one, so a missing or null census here is LAUNCH-CENSUS-UNRESOLVED, never "no census taken", even
 * though `launchFailures` serves a runner without a census when it is given none. */
export function keptPhaseFailures(summary, phase, report, problem = null) {
  const outcome = summary?.launches?.[phase]?.outcome;
  if (!completeOutcome(outcome)) return [NOT_RECORDED];
  return launchFailures({ status: outcome.status, signal: outcome.signal, timedOut: outcome.timed_out, census: outcome.census ?? null, report, problem, nonce: outcome.nonce });
}

/** The named failures of kept evidence from a runner whose contract requires a census (`perf.mjs`): its recorded
 * outcome judged with its report, under the same contract as `keptPhaseFailures`. An incomplete outcome is
 * LAUNCH-OUTCOME-NOT-RECORDED; a missing or null census is LAUNCH-CENSUS-UNRESOLVED, never "no census taken", and
 * that includes a record written before the runner took a census. */
export function recordedCensusFailures(outcome, report, problem = null) {
  if (!completeOutcome(outcome)) return [NOT_RECORDED];
  return launchFailures({ status: outcome.status, signal: outcome.signal, timedOut: outcome.timed_out, census: outcome.census ?? null, report, problem, nonce: outcome.nonce });
}
