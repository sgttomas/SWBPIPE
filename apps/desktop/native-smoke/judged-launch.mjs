// One judged launch of a phase (DEL-07-11), shared by `run.mjs` and `mbf-journey.mjs` after their own refusals:
// the three marks directly before the start (`marks-check.mjs`), the start through the runner's wrapper
// (`runUnderLock`), the census of the private copy, the one rule of `launch-outcome.mjs` over the exit and the
// report, the phase's record and check, and the run's summary saved when the phase stops the run; then
// `phaseContinuation`, the runners' one handler of that result, which turns a stop into the process's exit, so no
// further phase starts. `judged-launch.test.mjs` drives the launch path with a stand-in executable and controlled
// responses to the marks commands; `judged-launch-stop.test.mjs` runs the handler in child processes with the real
// exit. The runners pass their bound app copy.
// `perf.mjs` starts its app beside its sampler and applies the marks check and the rule itself.
import { writeFileSync } from "node:fs";
import { awaitNoProcesses } from "./launch-env.mjs";
import { launchFailures, readReport, recordedOutcome } from "./launch-outcome.mjs";
import { marksCheck, marksRefuse } from "./marks-check.mjs";
import { exitAfterSignals, runUnderLock } from "../scripts/worktreeBuildLock.mjs";

export const MARK_PRESENT = "NATIVE-SMOKE-MARK-PRESENT";
export const UNRESOLVED_TERMINATION = "UNRESOLVED TERMINATION";
/** The marks of an agent operating the screen, which the caller's statement that it is the launch's attendant covers. */
export const ATTENDANT_EXEMPT_MARKS = Object.freeze(["sky_computer_use_service_launchd", "sky_computer_use_service_process", "codex_app_server_stdio"]);
export const ATTENDANT_STATEMENT = "the agent operating the screen is this launch's attendant";

/**
 * Whether `marks` (marksCheck) refuse a start: as marksRefuse, except that with `attendant`, the statement of an
 * attended start (walkthrough/native/run.mjs --attended --attendant-on-screen <who>), the marks of an agent operating
 * the screen do not; a check that could not be made, or another application's candidate bundle, refuses as always.
 * { refuse, exempted, invalid? }: `exempted` names each mark present that the statement covered; a statement given
 * whose stated_by is blank refuses the start, with `invalid`, even with no mark present.
 */
export function marksDecision(marks, attendant = null) {
  // A statement given is valid or the start is refused, whatever the marks: it is checked before they are.
  if (attendant !== null && attendant !== undefined && !(typeof attendant.stated_by === "string" && attendant.stated_by.trim() !== "")) {
    return { refuse: true, exempted: [], invalid: "the attendant's statement names no one who makes it (stated_by is blank)" };
  }
  if (!marksRefuse(marks)) return { refuse: false, exempted: [] };
  if (!attendant || marks.check_failures.length > 0 || marks.app_v4_candidate_bundle) return { refuse: true, exempted: [] };
  return { refuse: false, exempted: ATTENDANT_EXEMPT_MARKS.filter((key) => marks[key] === true) };
}

/**
 * Launches `command commandArgs` (the wrapper that runs `executable`) for `phase` unless a mark is present, judges
 * it, records it in `launches[phase]` and reports it through `check(checkName, ok, detail)`.
 * `beforeStart` runs after the marks check, directly before the start (the run marker). When the phase stops the run,
 * `summaryPath` receives `{ ...summary(), launches, refused | stopped }` and the result carries `stop` and `exitCode`:
 * a mark present or a marks check that could not be made (exit 2, nothing started), or an unresolved census
 * (exit 1, the failed launch recorded and named). Otherwise the result is `{ report, read, outcome, failures, census }`;
 * `report` is the report as read, or `{ ok: false, error }` when there is none, so that a failed launch's
 * observations stay readable. `marksRun` and `census` are internal seams for the test; the runners leave them unset.
 * `abort`, optional, is an AbortSignal handed to runUnderLock, which owns the started child: aborting it ends the
 * launch with SIGKILL to that child (its own group) and the phase's record keeps runUnderLock's receipt as `abort`.
 * Absent, nothing changes.
 * `attendant`, optional, is an attended start's statement `{ stated_by }` (marksDecision): with it the marks of an
 * agent operating the screen do not refuse the start, `beforeStart` is given `{ marks, exempted }`, and the phase's
 * record keeps `attendant: { stated_by, statement, exempted }`. Absent, nothing changes.
 */
export async function judgedPhase({
  phase, checkName, launches, check, summaryPath, summary = () => ({}),
  executable, command, commandArgs, cwd, env, stdio = ["ignore", "inherit", "inherit"], timeoutMs, reportPath, nonce,
  censusBoundMs, beforeStart = () => {}, marksRun, census = awaitNoProcesses, abort, attendant
}) {
  const marks = marksRun ? marksCheck(marksRun) : marksCheck();
  const decision = attendant ? marksDecision(marks, attendant) : { refuse: marksRefuse(marks), exempted: [] };
  const stated = attendant ? { attendant: { stated_by: attendant.stated_by ?? null, statement: ATTENDANT_STATEMENT, exempted: decision.exempted, ...(decision.invalid ? { invalid: decision.invalid } : {}) } } : {};
  if (decision.refuse) {
    launches[phase] = { marks, launched: false, ...stated };
    writeFileSync(summaryPath, JSON.stringify({ ...summary(), launches, refused: MARK_PRESENT }, null, 2));
    return { stop: MARK_PRESENT, exitCode: 2, marks };
  }
  if (attendant) beforeStart({ marks, exempted: decision.exempted });
  else beforeStart();
  const started = new Date().toISOString();
  const { status, signal, timedOut, abort: abortReceipt } = await runUnderLock(command, commandArgs, { cwd, env, stdio, timeoutMs, ...(abort ? { abort } : {}) });
  const ended = new Date().toISOString();
  // The census: no process may still run the private copy (a solve worker runs it in its own process group,
  // which runUnderLock does not track).
  const termination = await census(executable, { boundMs: censusBoundMs });
  // Judged and recorded whatever the census found, so that an unresolved census is a named, recorded failure.
  const read = readReport(reportPath);
  const failures = launchFailures({ status, signal, timedOut, census: termination, report: read.report, problem: read.problem, nonce });
  const outcome = recordedOutcome({ status, signal, timedOut, census: termination, nonce, started, ended });
  launches[phase] = { marks, outcome, failures, ...(abort ? { abort: abortReceipt ?? null } : {}), ...stated };
  check(checkName, failures.length === 0, failures.length === 0 ? undefined : failures);
  if (!termination?.resolved) {
    writeFileSync(summaryPath, JSON.stringify({ ...summary(), launches, stopped: UNRESOLVED_TERMINATION }, null, 2));
    return { stop: UNRESOLVED_TERMINATION, exitCode: 1, marks, outcome, failures, census: termination, read };
  }
  return { report: read.report ?? { ok: false, error: `${read.problem} (exit ${status ?? signal})` }, read, outcome, failures, census: termination, marks };
}

/**
 * The handler both runners attach to `judgedPhase`'s result: `judgedPhase(...).then(phaseContinuation({...}))`.
 * It logs the marks found. A stop ends the process through `exit` (the real `exitAfterSignals`, which never resolves
 * normally), so the runner's next phase never starts: a mark stop before `state` is touched, an unresolved census
 * after `state.termination` is set. Otherwise it records the census in `state` and returns the report for the checks
 * that follow, a failed launch's included. `state` is `run.mjs`'s run state (its exit handler takes a census when
 * `termination` is still null); `mbf-journey.mjs` gives none. `exit` is an internal seam: the runners never pass it,
 * and `judged-launch-stop.test.mjs` fails if one does.
 */
export function phaseContinuation({ prefix, phase, state = null, evidenceRoot = null, exit = exitAfterSignals }) {
  return (result) => {
    console.log(`phase ${phase}: marks, directly before the launch: ${JSON.stringify(result.marks)}`);
    if (result.stop === MARK_PRESENT) {
      console.error(`${prefix}: ${MARK_PRESENT}: an agent may be operating the App on the screen, or the marks could not be checked; not launched`);
      return exit(result.exitCode);
    }
    if (state) state.termination = result.census;
    if (result.outcome?.timed_out) console.error(`phase ${phase}: timed out`);
    if (result.stop) {
      const where = evidenceRoot ? `; evidence in ${evidenceRoot}` : "";
      console.error(`${prefix}: ${result.stop} after phase ${phase}: ${result.census?.reason ?? "no census"}; no further phase starts${where}`);
      return exit(result.exitCode);
    }
    if (state) console.log(`phase ${phase}: the census finds no process running the private app copy`);
    // A failed launch's report stays readable for the checks that follow.
    return result.report;
  };
}
