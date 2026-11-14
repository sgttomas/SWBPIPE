// One rule for judging a native-smoke launch (DEL-07-11 VER-003): a launch passes only when the app ended
// cleanly (exit status 0, no signal, no timeout), the census after it found no process of the run's private
// app copy, and the run's fresh evidence root holds a readable report carrying this run's nonce. Each failed
// condition is named. A failed launch never reads as a pass; its report, when there is one, stays readable for
// the observations it records. `run.mjs` and `mbf-journey.mjs` judge each launch with it; `perf.mjs` applies the
// same rule in its own classification. Evidence that recorded no launch outcome is judged
// LAUNCH-OUTCOME-NOT-RECORDED: not a pass.
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

/** The named failures of kept evidence: its recorded outcome judged with its report. Evidence that recorded
 * no outcome is LAUNCH-OUTCOME-NOT-RECORDED, whatever its report says. */
export function recordedFailures(outcome, report, problem = null) {
  if (!outcome || typeof outcome !== "object") return ["LAUNCH-OUTCOME-NOT-RECORDED: this evidence records no launch outcome, so it cannot be judged a pass"];
  return launchFailures({ status: outcome.status, signal: outcome.signal, timedOut: outcome.timed_out, census: outcome.census ?? undefined, report, problem, nonce: outcome.nonce });
}
