// The runners' production handler of a judged phase (`phaseContinuation`, `judged-launch.mjs`, DEL-07-11), run in
// child Node processes with the real exit (`exitAfterSignals`), as `run.mjs` and `mbf-journey.mjs` attach it: a stop
// must end the process before the next phase. Each child feeds a judgedPhase-shaped result through
// `Promise.resolve(result).then(handler)`, awaits it as the runners await `launch()`, and then writes a second-start
// marker; the marker exists only if the handler let the run go on. It also checks, in the runners' source, that each
// attaches exactly this handler to `judgedPhase`'s result and passes it no exit of its own. Nothing is launched.
// Runs on macOS and Linux.
//
//   node --test apps/desktop/native-smoke/judged-launch-stop.test.mjs
import { test, before, after } from "node:test";
import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { existsSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import os from "node:os";
import path from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";

const here = path.dirname(fileURLToPath(import.meta.url));
let dir, child;

// The child: one result through the runner's handler, then the second start.
const CHILD = (moduleUrl) => `
import { writeFileSync } from "node:fs";
import { MARK_PRESENT, UNRESOLVED_TERMINATION, phaseContinuation } from ${JSON.stringify(moduleUrl)};
const [mode, form, marker] = process.argv.slice(2);
const state = form === "run" ? { launched: true, termination: null } : null;
process.on("exit", (code) => process.stdout.write("\\n" + JSON.stringify({ code, state })));
const marks = { check_failures: [] };
const unresolved = { resolved: false, reason: "process 7 still ran the copy" };
const result = {
  mark: { stop: MARK_PRESENT, exitCode: 2, marks },
  census: { stop: UNRESOLVED_TERMINATION, exitCode: 1, marks, outcome: { timed_out: false }, census: unresolved, failures: ["LAUNCH-CENSUS-UNRESOLVED"], read: { report: { ok: true } } },
  clean: { report: { ok: true, nonce: "n" }, marks, outcome: { timed_out: false }, census: { resolved: true }, failures: [] }
}[mode];
// The runners' own arguments: run.mjs gives its state, mbf-journey.mjs its evidence root.
const handler = form === "run"
  ? phaseContinuation({ prefix: "native smoke", phase: "p", state })
  : phaseContinuation({ prefix: "mbf journey", phase: "p", evidenceRoot: "/invented/root" });
const report = await Promise.resolve(result).then(handler);
writeFileSync(marker, JSON.stringify(report));
`;

before(() => {
  dir = mkdtempSync(path.join(os.tmpdir(), "judged-launch-stop-test-"));
  child = path.join(dir, "child.mjs");
  writeFileSync(child, CHILD(pathToFileURL(path.join(here, "judged-launch.mjs")).href));
});
after(() => rmSync(dir, { recursive: true, force: true }));

/** One result through one runner's handler in a child process: its exit code, its state, and whether the run went on. */
function handle(mode, form) {
  const marker = path.join(dir, `second-start-${mode}-${form}`);
  const run = spawnSync(process.execPath, [child, mode, form, marker], { encoding: "utf8", timeout: 30_000 });
  assert.equal(run.error, undefined);
  const last = run.stdout.trim().split("\n").at(-1);
  const { code, state } = JSON.parse(last);
  return { status: run.status, code, state, wentOn: existsSync(marker), report: existsSync(marker) ? JSON.parse(readFileSync(marker, "utf8")) : null, stderr: run.stderr };
}

for (const form of ["run", "mbf"]) {
  test(`${form}: a mark stop ends the process with exit 2 before the next phase, touching no state`, () => {
    const out = handle("mark", form);
    assert.deepEqual([out.status, out.code, out.wentOn], [2, 2, false], out.stderr);
    if (form === "run") assert.equal(out.state.termination, null);
    assert.match(out.stderr, /NATIVE-SMOKE-MARK-PRESENT/);
  });

  test(`${form}: an unresolved census ends the process with exit 1 before the next phase`, () => {
    const out = handle("census", form);
    assert.deepEqual([out.status, out.code, out.wentOn], [1, 1, false], out.stderr);
    if (form === "run") assert.deepEqual(out.state.termination, { resolved: false, reason: "process 7 still ran the copy" });
    assert.match(out.stderr, /UNRESOLVED TERMINATION after phase p/);
  });

  test(`${form}: a result that does not stop goes on to the next phase with its report`, () => {
    const out = handle("clean", form);
    assert.deepEqual([out.status, out.code, out.wentOn], [0, 0, true], out.stderr);
    assert.deepEqual(out.report, { ok: true, nonce: "n" });
    if (form === "run") assert.deepEqual(out.state.termination, { resolved: true });
  });
}

test("each runner returns exactly this handler's promise from its launch function, awaits every launch, and passes no exit", () => {
  for (const [file, keys] of [["run.mjs", ["prefix", "phase", "state"]], ["mbf-journey.mjs", ["prefix", "phase", "evidenceRoot"]]]) {
    const text = readFileSync(path.join(here, file), "utf8");
    const starts = [...text.matchAll(/judgedPhase\(\{/g)];
    assert.equal(starts.length, 1, `${file}: one judgedPhase call`);
    // The launch function RETURNS the handled promise: attached and ignored would be the uncalled handler again.
    assert.equal(text.slice(starts[0].index - "return ".length, starts[0].index), "return ", `${file}: the launch function returns judgedPhase(...)`);
    const attached = /\n {2}\}\)\.then\(phaseContinuation\(\{([^{}]*)\}\)\);\n\}/.exec(text.slice(starts[0].index));
    assert.ok(attached, `${file}: return judgedPhase(...).then(phaseContinuation({...})); ends the launch function`);
    // And every call of launch() is awaited before anything after it runs.
    const calls = [...text.matchAll(/(^|[^\w.])launch\(/gm)].filter((match) => !text.slice(Math.max(0, match.index - 9), match.index + 1).includes("function"));
    assert.ok(calls.length >= 2, `${file}: its launches are found`);
    for (const call of calls) assert.equal(text.slice(call.index - 5, call.index + call[1].length), "await" + call[1], `${file}: launch() awaited at offset ${call.index}`);
    const between = text.slice(starts[0].index, starts[0].index + attached.index);
    assert.doesNotMatch(between, /\.(then|catch|finally)\(/, `${file}: nothing else is attached to judgedPhase's result`);
    const given = attached[1].split(",").map((part) => part.split(":")[0].trim()).filter(Boolean);
    assert.deepEqual(given, keys, `${file}: phaseContinuation's arguments`);
    assert.ok(!given.includes("exit"), `${file}: the runner passes no exit`);
  }
});
