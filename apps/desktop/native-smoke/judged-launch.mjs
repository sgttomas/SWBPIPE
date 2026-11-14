// One judged launch of a phase (DEL-07-11), shared by `run.mjs` and `mbf-journey.mjs` after their own refusals:
// the three marks directly before the start (`marks-check.mjs`), the start through the runner's wrapper
// (`runUnderLock`), the census of the private copy, the one rule of `launch-outcome.mjs` over the exit and the
// report, the phase's record and check, and the run's summary saved when the phase stops the run. The runner turns a
// returned `stop` into its exit, so no further phase starts. `judged-launch.test.mjs` drives this path with a
// stand-in executable and controlled responses to the marks commands; the runners pass their bound app copy.
// `perf.mjs` starts its app beside its sampler and applies the marks check and the rule itself.
import { writeFileSync } from "node:fs";
import { awaitNoProcesses } from "./launch-env.mjs";
import { launchFailures, readReport, recordedOutcome } from "./launch-outcome.mjs";
import { marksCheck, marksRefuse } from "./marks-check.mjs";
import { runUnderLock } from "../scripts/worktreeBuildLock.mjs";

export const MARK_PRESENT = "NATIVE-SMOKE-MARK-PRESENT";
export const UNRESOLVED_TERMINATION = "UNRESOLVED TERMINATION";

/**
 * Launches `command commandArgs` (the wrapper that runs `executable`) for `phase` unless a mark is present, judges
 * it, records it in `launches[phase]` and reports it through `check(checkName, ok, detail)`.
 * `beforeStart` runs after the marks check, directly before the start (the run marker). When the phase stops the run,
 * `summaryPath` receives `{ ...summary(), launches, refused | stopped }` and the result carries `stop` and `exitCode`:
 * a mark present or a marks check that could not be made (exit 2, nothing started), or an unresolved census
 * (exit 1, the failed launch recorded and named). Otherwise the result is `{ report, read, outcome, failures, census }`;
 * `report` is the report as read, or `{ ok: false, error }` when there is none, so that a failed launch's
 * observations stay readable. `marksRun` and `census` are internal seams for the test; the runners leave them unset.
 */
export async function judgedPhase({
  phase, checkName, launches, check, summaryPath, summary = () => ({}),
  executable, command, commandArgs, cwd, env, stdio = ["ignore", "inherit", "inherit"], timeoutMs, reportPath, nonce,
  censusBoundMs, beforeStart = () => {}, marksRun, census = awaitNoProcesses
}) {
  const marks = marksRun ? marksCheck(marksRun) : marksCheck();
  if (marksRefuse(marks)) {
    launches[phase] = { marks, launched: false };
    writeFileSync(summaryPath, JSON.stringify({ ...summary(), launches, refused: MARK_PRESENT }, null, 2));
    return { stop: MARK_PRESENT, exitCode: 2, marks };
  }
  beforeStart();
  const started = new Date().toISOString();
  const { status, signal, timedOut } = await runUnderLock(command, commandArgs, { cwd, env, stdio, timeoutMs });
  const ended = new Date().toISOString();
  // The census: no process may still run the private copy (a solve worker runs it in its own process group,
  // which runUnderLock does not track).
  const termination = await census(executable, { boundMs: censusBoundMs });
  // Judged and recorded whatever the census found, so that an unresolved census is a named, recorded failure.
  const read = readReport(reportPath);
  const failures = launchFailures({ status, signal, timedOut, census: termination, report: read.report, problem: read.problem, nonce });
  const outcome = recordedOutcome({ status, signal, timedOut, census: termination, nonce, started, ended });
  launches[phase] = { marks, outcome, failures };
  check(checkName, failures.length === 0, failures.length === 0 ? undefined : failures);
  if (!termination?.resolved) {
    writeFileSync(summaryPath, JSON.stringify({ ...summary(), launches, stopped: UNRESOLVED_TERMINATION }, null, 2));
    return { stop: UNRESOLVED_TERMINATION, exitCode: 1, marks, outcome, failures, census: termination, read };
  }
  return { report: read.report ?? { ok: false, error: `${read.problem} (exit ${status ?? signal})` }, read, outcome, failures, census: termination, marks };
}
