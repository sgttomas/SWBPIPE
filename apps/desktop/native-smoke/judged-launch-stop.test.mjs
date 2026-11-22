// The runners' production handler of a judged phase (`phaseContinuation`, `judged-launch.mjs`, DEL-07-11), run in
// child Node processes with the real exit (`exitAfterSignals`), as `run.mjs` and `mbf-journey.mjs` attach it: a stop
// must end the process before the next phase. Each child feeds a judgedPhase-shaped result through
// `Promise.resolve(result).then(handler)`, awaits it as the runners await `launch()`, and then writes a second-start
// marker; the marker exists only if the handler let the run go on. It also checks the runners' syntax tree: each
// launch function returns `judgedPhase(...)` with exactly this handler attached, every launch is awaited, and no exit
// is passed. Nothing is launched.
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
import { createHash } from "node:crypto";
import ts from "typescript";

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

// The runners' glue, read from their syntax tree (TypeScript's parser, a desktop devDependency), not their text, so
// that spacing, comments and strings cannot satisfy it. Returns the problems found; empty when the source is right.
function glueProblems(text, keys) {
  const file = ts.createSourceFile("runner.mjs", text, ts.ScriptTarget.Latest, true, ts.ScriptKind.JS);
  const problems = [];
  const identifiers = [];
  const visit = (node) => { if (ts.isIdentifier(node)) identifiers.push(node); ts.forEachChild(node, visit); };
  visit(file);
  const named = (name) => identifiers.filter((id) => id.text === name);
  // Both names are the shared module's own bindings, imported once and never declared again.
  for (const name of ["judgedPhase", "phaseContinuation"]) {
    const imported = file.statements.filter((statement) => ts.isImportDeclaration(statement) && statement.moduleSpecifier.text === "./judged-launch.mjs"
      && statement.importClause?.namedBindings?.elements?.some((element) => element.name.text === name && !element.propertyName));
    if (imported.length !== 1) problems.push(`${name} is not imported once from ./judged-launch.mjs`);
    const declared = named(name).filter((id) => id.parent && (ts.isVariableDeclaration(id.parent) || ts.isFunctionDeclaration(id.parent) || ts.isParameter(id.parent) || ts.isClassDeclaration(id.parent)) && id.parent.name === id);
    if (declared.length > 0) problems.push(`${name} is declared again in the runner`);
  }
  // One judgedPhase call, inside the launch function, whose result has exactly phaseContinuation({...}) attached,
  // directly, and that handled promise is what the launch function returns.
  const launchFunction = file.statements.find((statement) => ts.isFunctionDeclaration(statement) && statement.name?.text === "launch");
  if (!launchFunction) problems.push("no top-level function launch");
  const calls = named("judgedPhase").filter((id) => ts.isCallExpression(id.parent) && id.parent.expression === id);
  if (calls.length !== 1 || named("judgedPhase").length !== 2) problems.push(`judgedPhase is used other than in one call (${calls.length} calls)`);
  const call = calls[0]?.parent;
  const attach = call?.parent;
  const handled = attach?.parent;
  const okAttach = attach && ts.isPropertyAccessExpression(attach) && attach.expression === call && attach.name.text === "then"
    && handled && ts.isCallExpression(handled) && handled.expression === attach && handled.arguments.length === 1;
  if (!okAttach) problems.push("judgedPhase's result does not have exactly one .then(...) attached directly");
  const continuation = okAttach ? handled.arguments[0] : null;
  const okContinuation = continuation && ts.isCallExpression(continuation) && ts.isIdentifier(continuation.expression) && continuation.expression.text === "phaseContinuation"
    && continuation.arguments.length === 1 && ts.isObjectLiteralExpression(continuation.arguments[0]);
  if (!okContinuation) problems.push("the attached handler is not phaseContinuation({...})");
  if (named("phaseContinuation").length !== 2) problems.push("phaseContinuation is used other than in that one call");
  if (okAttach && !(ts.isReturnStatement(handled.parent) && handled.parent.expression === handled)) problems.push("the launch function does not return the handled promise itself");
  if (okAttach && launchFunction) {
    let owner = handled.parent;
    while (owner && !ts.isFunctionLike(owner)) owner = owner.parent;
    if (owner !== launchFunction) problems.push("the handled promise is not returned by the launch function");
  }
  if (okContinuation) {
    const given = continuation.arguments[0].properties.map((property) => property.name && ts.isIdentifier(property.name) ? property.name.text : `<${ts.SyntaxKind[property.kind]}>`);
    if (JSON.stringify(given) !== JSON.stringify(keys)) problems.push(`phaseContinuation's arguments are ${JSON.stringify(given)}, not ${JSON.stringify(keys)}`);
  }
  // Every use of launch other than its declaration is the callee of a call that is the direct operand of an await
  // with no function-like ancestor: the phase sequence itself waits at module scope, so that no awaited call can
  // sit inside a callback nobody awaits.
  const uses = named("launch").filter((id) => id !== launchFunction?.name);
  if (uses.length < 2) problems.push("its launches are not found");
  for (const id of uses) {
    const used = id.parent;
    const line = file.getLineAndCharacterOfPosition(id.getStart()).line + 1;
    if (!(ts.isCallExpression(used) && used.expression === id && ts.isAwaitExpression(used.parent) && used.parent.expression === used)) problems.push(`launch is used other than in an awaited call at line ${line}`);
    let owner = id.parent;
    while (owner && !ts.isFunctionLike(owner)) owner = owner.parent;
    if (owner) problems.push(`launch is called inside a function, not at module scope, at line ${line}`);
  }
  return problems;
}

// The stop-handling orchestration as last independently read: the two runners and judged-launch.mjs, whole. The
// property G1 protects belongs to each runner's whole orchestration (an app start written outside launch is an
// ordinary edit the tree check cannot see), so the pin is over the whole file; the depth of re-reading follows the
// diff (PIN_MESSAGE). Recompute after the applicable reading clears; merging main does not waive that reading:
//   shasum -a 256 apps/desktop/native-smoke/run.mjs apps/desktop/native-smoke/mbf-journey.mjs apps/desktop/native-smoke/judged-launch.mjs
const PIN = {
  "run.mjs": "568cf8017a3840d23c1b3c431048666c0d49e49b4541b359794c97a7822e8a36",
  "mbf-journey.mjs": "34ca4d4e08e739918dcdceba80e6c3b4c6e8bf2e4164a18fa67fc5e43e49bd71",
  "judged-launch.mjs": "ab6469bbd7443955e32613afa773f8a9875fa7c7f4f8b2f154a816fd26ac5cfc"
};
const PIN_MESSAGE = (file) => [
  `${file} changed since its stop handling was last independently read. Before this pin is updated, the independent reader of the pull request that changes it re-reads it, and that pull request quotes the reading:`,
  "- for a diff confined to journey content (observations, assertions, messages), the reader establishes from the diff that no launch path, app binding or phase sequence changed, and says so in one sentence;",
  "- for a diff that touches any of them, the reader reads in full that every path that starts an app goes through judgedPhase with phaseContinuation attached and returned, that the phase sequence awaits each launch at module scope, and that phaseContinuation receives the production state and evidence root.",
  "The author refreshes the pin after that reading clears. A merge of main may reuse an earlier independent reading only when its pinned file has exactly the reviewed bytes; the pull request names that reading and which side supplied them. Any different merged bytes require the reading above before recomputing the pin."
].join("\n");
const sha256 = (text) => createHash("sha256").update(text, "utf8").digest("hex");

const RUNNERS = [["run.mjs", ["prefix", "phase", "state"]], ["mbf-journey.mjs", ["prefix", "phase", "evidenceRoot"]]];

test("each runner returns exactly this handler's promise from its launch function, awaits every launch, and passes no exit (syntax tree)", () => {
  for (const [file, keys] of RUNNERS) assert.deepEqual(glueProblems(readFileSync(path.join(here, file), "utf8"), keys), [], file);
});

test("the stop-handling orchestration is the source last independently read (the pins)", () => {
  for (const [file, digest] of Object.entries(PIN)) assert.equal(sha256(readFileSync(path.join(here, file))), digest, PIN_MESSAGE(file));
});

test("the glue check refuses each way around it the readers found or that would bring back an ignored stop", () => {
  const text = readFileSync(path.join(here, "run.mjs"), "utf8");
  const keys = RUNNERS[0][1];
  const glue = '  }).then(phaseContinuation({ prefix: "native smoke", phase, state }));';
  assert.ok(text.includes(glue), "the run.mjs glue line is where these mutations expect it");
  const mutations = {
    // The reader's R1: a handler before the shared one that removes the stop, spaced past a text match.
    "an intervening handler": text.replace(glue, '  }).then ((result) => { delete result.stop; return result; }).then(phaseContinuation({ prefix: "native smoke", phase, state }));'),
    "a handler after it": text.replace(glue, glue.replace("}));", "})).then((report) => report);")),
    "the handled promise not returned": text.replace("  return judgedPhase({", "  void judgedPhase({").replace(glue, glue + "\n  // return judgedPhase({...}).then(phaseContinuation({ prefix, phase, state }));"),
    "a launch not awaited": text.replace("const second = await launch(", "// const second = await launch(\nconst second = launch ("),
    "an exit passed": text.replace(glue, glue.replace("phase, state }", "phase, state, exit: (code) => code }")),
    "the glue only in a string": text.replace(glue, '  });\n  const glue = "}).then(phaseContinuation({ prefix, phase, state }))";').replace("  return judgedPhase({", "  return judgedPhase({"),
    // The second reader's valid shadowing case: an inner parameter that shadows the shared handler.
    "an inner parameter shadowing phaseContinuation": text.replace("function launch(phase, extra = {}) {", "function launch(phase, extra = {}, phaseContinuation = () => (result) => result.report ?? result.read?.report) {"),
    // The second reader's R1b: an awaited launch inside an async callback nobody awaits.
    "a detached launch sequence": text.replace("const first = await launch(", "void (async () => { await launch(\"reopen\"); })();\nconst first = await launch(")
  };
  const reasons = {
    "an inner parameter shadowing phaseContinuation": /phaseContinuation is declared again/,
    "a detached launch sequence": /launch is called inside a function, not at module scope/
  };
  for (const [name, mutated] of Object.entries(mutations)) {
    assert.notEqual(mutated, text, `${name}: the mutation applied`);
    const problems = glueProblems(mutated, keys);
    assert.ok(problems.length > 0, `${name}: refused`);
    if (reasons[name]) assert.ok(problems.some((problem) => reasons[name].test(problem)), `${name}: refused for its own reason (${JSON.stringify(problems)})`);
  }
});

test("a second app-start path (the second reader's R1a) passes the tree check, and the pin refuses it", () => {
  const text = readFileSync(path.join(here, "run.mjs"), "utf8");
  const head = "function launch(phase, extra = {}) {";
  const early = text.replace(head, `${head}\n  if (phase === "open-solve-save") {\n    return runUnderLock("/usr/bin/caffeinate", ["-di", binary], { cwd: root }).then(() => ({ ok: true }));\n  }`);
  assert.notEqual(early, text, "the mutation applied");
  // The tree check reads the one handled return and does not exclude another path: this is its stated limit.
  assert.deepEqual(glueProblems(early, RUNNERS[0][1]), [], "the tree check alone accepts it");
  // The protection is the pin over the runner.
  assert.notEqual(sha256(early), PIN["run.mjs"], "the pin refuses it");
});
