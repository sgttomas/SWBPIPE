// What a native-smoke launch is given, what it must not reach, and how a run is
// closed (`launch-env.mjs`, `run.mjs --print-launch-env`), without launching
// the app. Every folder here is invented under the system temporary folder;
// the real user's Library is never examined. Runs on macOS and Linux: the
// desktop test script runs it before Vitest (`apps/desktop/package.json`).
//
//   node --test apps/desktop/native-smoke/launch-env.test.mjs
import { test } from "node:test";
import assert from "node:assert/strict";
import { spawn, spawnSync } from "node:child_process";
import { copyFileSync, existsSync, lstatSync, mkdirSync, mkdtempSync, readdirSync, readFileSync, readlinkSync, realpathSync, rmSync, symlinkSync, utimesSync, writeFileSync } from "node:fs";
import os from "node:os";
import path from "node:path";
import { fileURLToPath } from "node:url";
import {
  HOME_KEYS, PARENT_ALLOWLIST, RUNNER_KEYS, STRICT_CHOOSER_ENV, awaitNoProcesses, awaitNoProcessesSync, buildLaunchEnv, censusOf, chooserLayout, closeRun, createThrowawayHome,
  keepOutOfHome, keptRelativePaths, readRunMarker, refusedLaunchEnv, refusedPlanPaths, refusedSmokeWindows, runMarkerRefusal, smokeWindows, statWatched, tripwireChanges, watchedLibraryPaths,
  writeRunMarker
} from "./launch-env.mjs";

const here = path.dirname(fileURLToPath(import.meta.url));
const CREDENTIAL = "SWBPIPE_TEST_INVENTED_PROVIDER_API_KEY";
const CREDENTIAL_VALUE = "sk-invented-0123456789abcdef";
const RUNNER = { SWBPIPE_NATIVE_SMOKE_DRIVER: "/invented/driver.js", SWBPIPE_NATIVE_SMOKE_PLAN: "{}", SWBPIPE_NATIVE_SMOKE_REPORT: "/invented/report.json" };
const NAMES = "CFFIXED_USER_HOME HOME SWBPIPE_NATIVE_SMOKE_DRIVER SWBPIPE_NATIVE_SMOKE_PLAN SWBPIPE_NATIVE_SMOKE_REPORT SWBPIPE_NATIVE_SMOKE_STRICT_CHOOSER TMPDIR";

function scratch(t) {
  const root = mkdtempSync(path.join(os.tmpdir(), "swbpipe-launch-env-test-"));
  t.after(() => rmSync(root, { recursive: true, force: true }));
  return root;
}

function built(t, parent) {
  const { home, tmpdir } = createThrowawayHome(scratch(t));
  return { home, tmpdir, env: buildLaunchEnv({ parent, home, tmpdir, runner: RUNNER }) };
}

test("(a) a credential-shaped variable in the runner's environment is absent from the launched app's", (t) => {
  const parent = { ...process.env, [CREDENTIAL]: CREDENTIAL_VALUE, AWS_SECRET_ACCESS_KEY: "invented", GITHUB_TOKEN: "invented", HOME: "/Users/invented-owner" };
  const { home, tmpdir, env } = built(t, parent);
  assert.equal(Object.hasOwn(env, CREDENTIAL), false);
  assert.equal(Object.values(env).includes(CREDENTIAL_VALUE), false);
  for (const key of ["AWS_SECRET_ACCESS_KEY", "GITHUB_TOKEN"]) assert.equal(Object.hasOwn(env, key), false, key);
  // Exactly the home variables, TMPDIR, the runner's own and the allowlist: nothing else.
  assert.deepEqual(Object.keys(env).sort(), [...new Set([...HOME_KEYS, ...RUNNER_KEYS, ...PARENT_ALLOWLIST.filter((key) => Object.hasOwn(parent, key))])].sort());
  assert.equal(env.HOME, home);
  assert.equal(env.CFFIXED_USER_HOME, home);
  assert.equal(env.TMPDIR, tmpdir);
  assert.deepEqual(refusedLaunchEnv(env), []);
});

test("(b) each refusal fires, by name", (t) => {
  const { env } = built(t, {});
  const named = (candidate, reason) => assert.ok(refusedLaunchEnv(candidate).some((line) => line.startsWith("LAUNCH-ENV-REFUSED") && line.includes(reason)), `${reason}: ${refusedLaunchEnv(candidate)}`);
  named({ ...env, [CREDENTIAL]: CREDENTIAL_VALUE }, `outside the allowlist and the runner's own: ${CREDENTIAL}`);
  named({ ...env, CFFIXED_USER_HOME: path.join(env.HOME, "elsewhere") }, "HOME and CFFIXED_USER_HOME differ");
  const { CFFIXED_USER_HOME: _fixed, ...withoutFixed } = env;
  named(withoutFixed, "HOME and CFFIXED_USER_HOME differ");
  named({ ...env, TMPDIR: os.tmpdir() }, "TMPDIR is not inside HOME");
  named({ ...env, TMPDIR: env.HOME }, "TMPDIR is not inside HOME");
  named({ ...env, TMPDIR: `${env.HOME}-sibling/tmp` }, "TMPDIR is not inside HOME");
  named({ ...env, TMPDIR: path.join(env.HOME, "..", "tmp") }, "TMPDIR is not inside HOME");
  const { TMPDIR: _tmp, ...withoutTmp } = env;
  named(withoutTmp, "TMPDIR is not inside HOME");
});

test("(b) today's launch expression ({ ...process.env, HOME, runner's }) would be refused", (t) => {
  const { home } = built(t, {});
  const today = { ...process.env, [CREDENTIAL]: CREDENTIAL_VALUE, HOME: home, ...RUNNER };
  const reasons = refusedLaunchEnv(today);
  assert.ok(reasons.some((line) => line.includes(CREDENTIAL)), reasons.join("\n"));
  assert.ok(reasons.some((line) => line.includes("HOME and CFFIXED_USER_HOME differ")));
  assert.ok(reasons.some((line) => line.includes("TMPDIR is not inside HOME")));
  assert.ok(reasons.some((line) => line.includes(`${STRICT_CHOOSER_ENV} is not 1`)));
});

// The runner itself, in its no-launch mode: it takes no lock, builds nothing and launches nothing.
function printLaunchEnv(homeRoot, extra = []) {
  return spawnSync(process.execPath, [path.join(here, "run.mjs"), "--print-launch-env", "--home-root", homeRoot, ...extra], {
    encoding: "utf8",
    env: { ...process.env, [CREDENTIAL]: CREDENTIAL_VALUE }
  });
}
const printedHome = (stdout) => stdout.match(/^HOME and CFFIXED_USER_HOME: (.+)$/m)?.[1];

test("(c) --print-launch-env: the home is under --home-root, TMPDIR inside it, the chooser answers inside it, names only, and the home is removed on exit", (t) => {
  const root = scratch(t);
  const result = printLaunchEnv(root);
  assert.equal(result.status, 0, result.stderr);
  const home = printedHome(result.stdout);
  assert.ok(home && path.dirname(home) === realpathSync(root), result.stdout);
  assert.equal(result.stdout.match(/^TMPDIR: (.+)$/m)?.[1], path.join(home, "tmp"));
  assert.equal(result.stdout.match(/^launch environment variable names: (.+)$/m)?.[1], NAMES);
  assert.equal(result.stdout.match(/^chooser answers: (.+)$/m)?.[1], `every one resolves inside the home, under ${path.join(home, "chooser")}`);
  assert.equal(result.stdout.includes(CREDENTIAL), false);
  assert.equal(result.stdout.includes(CREDENTIAL_VALUE), false);
  assert.equal(existsSync(home), false, "home removed on exit");
  assert.deepEqual(readdirSync(root), []);
});

test("(c) --print-launch-env --keep keeps the home", (t) => {
  const root = scratch(t);
  const result = printLaunchEnv(root, ["--keep"]);
  assert.equal(result.status, 0, result.stderr);
  const home = printedHome(result.stdout);
  assert.ok(home && path.dirname(home) === realpathSync(root) && existsSync(path.join(home, "tmp")), result.stdout);
});

test("(c) --home-root that does not exist is refused before anything is created", (t) => {
  const root = scratch(t);
  const result = printLaunchEnv(path.join(root, "missing"));
  assert.equal(result.status, 2);
  assert.match(result.stderr, /--home-root .* does not exist/);
  assert.deepEqual(readdirSync(root), []);
});

test("(d) the tripwire names a watched path that appears, disappears or changes, in an invented Library", (t) => {
  const library = path.join(scratch(t), "Library");
  const names = { identifier: "com.invented.product", names: ["Invented", "invented-desktop"] };
  const watched = watchedLibraryPaths(library, names);
  // A fixed list of exact paths under the Library, each named for the product.
  for (const file of watched) {
    assert.equal(path.relative(library, file).split(path.sep).length, 2, file);
    assert.ok([names.identifier, ...names.names, `${names.identifier}.binarycookies`, `${names.identifier}.savedState`, `${names.identifier}.plist`].includes(path.basename(file)), file);
  }
  const support = path.join(library, "Application Support", names.identifier);
  const caches = path.join(library, "Caches", "Invented");
  const prefs = path.join(library, "Preferences", `${names.identifier}.plist`);
  mkdirSync(support, { recursive: true });
  mkdirSync(caches, { recursive: true });
  mkdirSync(path.dirname(prefs), { recursive: true });
  // An unrelated file under the same folders is not watched.
  writeFileSync(path.join(library, "Preferences", "com.other.plist"), "x");
  const before = statWatched(watched);
  assert.deepEqual(tripwireChanges(before, statWatched(watched)), []);

  const webkit = path.join(library, "WebKit", "invented-desktop");
  mkdirSync(webkit, { recursive: true });
  rmSync(caches, { recursive: true });
  utimesSync(support, new Date(2001, 1, 1), new Date(2001, 1, 1));
  writeFileSync(path.join(library, "Preferences", "com.other.plist"), "changed");
  assert.deepEqual(tripwireChanges(before, statWatched(watched)).sort(), [
    `TRIPWIRE: ${caches} disappeared`,
    `TRIPWIRE: ${support} changed its modification time`,
    `TRIPWIRE: ${webkit} appeared`
  ].sort());
  writeFileSync(prefs, "x");
  assert.ok(tripwireChanges(before, statWatched(watched)).includes(`TRIPWIRE: ${prefs} appeared`));
});

// F1. Choosers: refuse, never show a native dialog.

test("F1(b) the launch environment sets the strict chooser switch to 1, and an environment without it is refused", (t) => {
  const { env } = built(t, {});
  assert.equal(env[STRICT_CHOOSER_ENV], "1");
  const { [STRICT_CHOOSER_ENV]: _strict, ...withoutStrict } = env;
  for (const candidate of [withoutStrict, { ...env, [STRICT_CHOOSER_ENV]: "0" }, { ...env, [STRICT_CHOOSER_ENV]: "" }]) {
    assert.ok(refusedLaunchEnv(candidate).includes(`LAUNCH-ENV-REFUSED: ${STRICT_CHOOSER_ENV} is not 1`), JSON.stringify(refusedLaunchEnv(candidate)));
  }
});

test("F1(b) every chooser answer of the layout resolves inside the home; a path beside the home, a link out of it and the home itself are refused", (t) => {
  const root = scratch(t);
  const { home } = createThrowawayHome(root);
  const layout = chooserLayout(home);
  for (const folder of layout.folders) mkdirSync(folder);
  const plan = {
    refusals: [{ path: path.join(layout.models, "not-json.json") }],
    documents: [{ path: path.join(layout.models, "demo.json"), case: "demo", project_id: "project:invented" }],
    exact: { path: path.join(layout.models, "exact.json"), report_package: layout.exactReportPackageSave },
    model_document: layout.modelDocumentSave,
    report_package: layout.reportPackageSave,
    handoff: layout.handoffSave,
    blank: { anchor: "node:W4-A", probe: "result:disp:node-W4-B" }
  };
  assert.deepEqual(refusedPlanPaths(plan, home), []);
  // The layout before this repair: the answers in the evidence root, beside the home.
  const beside = path.join(path.dirname(home), "saved-model-documents", "saved-demo.json");
  assert.deepEqual(refusedPlanPaths({ ...plan, model_document: { first: beside } }, home), [`LAUNCH-PLAN-REFUSED: model_document.first (${beside}) does not resolve inside the home`]);
  const outside = mkdtempSync(path.join(root, "outside-"));
  symlinkSync(outside, path.join(layout.area, "link-out"));
  const linked = path.join(layout.area, "link-out", "saved.json");
  assert.equal(refusedPlanPaths({ handoff: { path: linked } }, home).length, 1);
  assert.equal(refusedPlanPaths({ handoff: { path: home } }, home).length, 1);
  assert.equal(refusedPlanPaths({ documents: [{ path: `${home}-sibling/demo.json` }] }, home).length, 1);
});

test("F1(c) what a run keeps is copied out of the home before the home is removed; links are copied as links", (t) => {
  const root = scratch(t);
  const { home } = createThrowawayHome(root);
  const layout = chooserLayout(home);
  for (const folder of layout.folders) mkdirSync(folder);
  writeFileSync(layout.modelDocumentSave.first, "saved");
  writeFileSync(path.join(home, "Downloads", "result.json"), "{}");
  const store = path.join(home, "Library", "Application Support", "com.invented.product");
  mkdirSync(store, { recursive: true });
  writeFileSync(path.join(store, "openpipestress-projects.sqlite3"), "db");
  const outside = path.join(root, "outside.txt");
  writeFileSync(outside, "not to be copied");
  symlinkSync(outside, path.join(layout.area, "link-out"));
  const evidence = mkdtempSync(path.join(root, "evidence-"));
  const copied = keepOutOfHome(home, evidence, keptRelativePaths("com.invented.product"));
  assert.deepEqual(copied, ["chooser", "Downloads", path.join("Library", "Application Support", "com.invented.product")]);
  rmSync(home, { recursive: true });
  const kept = path.join(evidence, "kept");
  assert.equal(readFileSync(path.join(kept, "chooser", "saved-model-documents", "saved-demo.json"), "utf8"), "saved");
  assert.equal(readFileSync(path.join(kept, "Downloads", "result.json"), "utf8"), "{}");
  assert.equal(readFileSync(path.join(kept, "Library", "Application Support", "com.invented.product", "openpipestress-projects.sqlite3"), "utf8"), "db");
  assert.ok(lstatSync(path.join(kept, "chooser", "link-out")).isSymbolicLink());
  assert.equal(readlinkSync(path.join(kept, "chooser", "link-out")), outside);
});

test("F1(d) a home that is not empty at creation is refused before anything is added to it", (t) => {
  const root = scratch(t);
  const planted = (prefix) => {
    const folder = mkdtempSync(prefix);
    writeFileSync(path.join(folder, "planted"), "x");
    return folder;
  };
  assert.throws(() => createThrowawayHome(root, { mkdtemp: planted }), /HOME-NOT-EMPTY: the new home .* already holds 1 entry at creation/);
  const [home] = readdirSync(root);
  assert.deepEqual(readdirSync(path.join(root, home)), ["planted"], "neither tmp nor Downloads was added");
  const { home: fresh } = createThrowawayHome(root);
  assert.deepEqual(readdirSync(fresh).sort(), ["Downloads", "tmp"]);
});

// F2. Termination: a census of the processes running the private copy.

// A private copy of this Node executable, running until it is killed: a stand-in for the app's private copy.
function privateCopy(t) {
  const executable = path.join(scratch(t), "private-copy");
  copyFileSync(process.execPath, executable);
  const child = spawn(executable, ["-e", "setInterval(() => {}, 1000)"], { stdio: "ignore" });
  t.after(() => { try { child.kill("SIGKILL"); } catch { /* gone */ } });
  return { executable, child };
}
const until = async (predicate, ms) => {
  const deadline = Date.now() + ms;
  while (!predicate() && Date.now() < deadline) await new Promise((resolve) => setTimeout(resolve, 50));
};

test("F2 the census finds a running private copy and, at the bound, names it as unresolved termination; once it ends the census finds none", async (t) => {
  const { executable, child } = privateCopy(t);
  await until(() => { const census = censusOf(executable); return census.ok && census.pids.includes(child.pid); }, 5000);
  const census = censusOf(executable);
  assert.ok(census.ok, census.reason);
  assert.deepEqual(census.pids, [child.pid]);
  const unresolved = await awaitNoProcesses(executable, { boundMs: 300, pollMs: 100 });
  assert.equal(unresolved.resolved, false);
  assert.equal(unresolved.reason, `process ${child.pid} still ran ${executable} after 300 ms`);
  assert.equal(awaitNoProcessesSync(executable, { boundMs: 200, pollMs: 100 }).resolved, false);
  child.kill("SIGKILL");
  await new Promise((resolve) => child.once("exit", resolve));
  assert.deepEqual(await awaitNoProcesses(executable, { boundMs: 5000, pollMs: 100 }), { resolved: true });
  assert.deepEqual(awaitNoProcessesSync(executable, { boundMs: 5000, pollMs: 100 }), { resolved: true });
});

test("F2 a census that cannot be taken is unresolved termination, never zero processes", async () => {
  const failed = () => ({ ok: false, reason: "invented failure" });
  assert.deepEqual(await awaitNoProcesses("/invented/app", { boundMs: 1000, census: failed }), { resolved: false, reason: "the census could not be taken: invented failure" });
  assert.deepEqual(awaitNoProcessesSync("/invented/app", { boundMs: 1000, census: failed }), { resolved: false, reason: "the census could not be taken: invented failure" });
  // macOS's census reads lsof: none only on its silent exit 1; any error is a census not taken.
  const lsof = (status, stdout, stderr, error) => censusOf("/invented/app", { platform: "darwin", lsof: "/invented/lsof", self: 1, run: () => ({ status, stdout, stderr, error }) });
  assert.deepEqual(lsof(0, "p4242\nftxt\n", ""), { ok: true, pids: [4242] });
  assert.deepEqual(lsof(1, "", ""), { ok: true, pids: [] });
  assert.equal(lsof(1, "", "lsof: status error on /invented/app: No such file or directory").ok, false);
  assert.equal(lsof(null, "", "", Object.assign(new Error("spawn"), { code: "ENOENT" })).ok, false);
  assert.equal(censusOf("/invented/app", { platform: "darwin", lsof: undefined }).ok, false);
  assert.equal(censusOf(path.join(os.tmpdir(), "swbpipe-invented-missing-executable"), { platform: "linux" }).ok, false);
});

test("F2 a later invocation recognises the marker an interrupted or unresolved run left, and its refusal names what it found", (t) => {
  const marker = path.join(scratch(t), "target", "native-smoke-run.json");
  assert.equal(readRunMarker(marker), null);
  const record = { pid: 4242, started: "Thu Nov 12 10:00:00 2026", home: "/invented/home", executable: "/invented/run-x/openpipestress-desktop", evidence_root: "/invented/evidence", reason: "process 4243 still ran /invented/run-x/openpipestress-desktop after 20000 ms" };
  writeRunMarker(marker, record);
  assert.deepEqual(readRunMarker(marker), record);
  const lines = runMarkerRefusal(marker, readRunMarker(marker), { alive: false, census: { ok: true, pids: [4243] } }).join("\n");
  assert.match(lines, /EARLIER-RUN-UNRESOLVED: .* refusing to start/);
  for (const part of ["pid 4242", "not running", record.reason, record.home, record.executable, "process 4243", `rm -rf '${record.home}'`, "rm -rf '/invented/run-x'", `rm -f '${marker}'`]) assert.ok(lines.includes(part), part);
  assert.match(runMarkerRefusal(marker, readRunMarker(marker), { alive: true, census: { ok: false, reason: "lsof was not found" } }).join("\n"), /STILL RUNNING[\s\S]*could not be taken \(lsof was not found\)/);
  writeFileSync(marker, "{ truncated");
  assert.equal(readRunMarker(marker).unreadable, true);
  assert.match(runMarkerRefusal(marker, readRunMarker(marker), { alive: false, census: { ok: false, reason: "-" } }).join("\n"), /cannot be read/);
});

// F3. The final tripwire on every exit the runner can catch.

function tripwireCase(t, change) {
  const library = path.join(scratch(t), "Library");
  const watched = watchedLibraryPaths(library, { identifier: "com.invented.product", names: [] });
  const before = statWatched(watched);
  if (change) mkdirSync(path.join(library, "Caches", "com.invented.product"), { recursive: true });
  return { before, after: statWatched(watched), done: false };
}
const RESOLVED = { resolved: true };

test("F3 the comparison runs on a normal end, a failed run, an exception and an interruption; it turns a success into a failure, never the reverse", (t) => {
  const changed = closeRun({ exitCode: 0, termination: RESOLVED, lockKept: false, tripwire: tripwireCase(t, true) });
  assert.equal(changed.exitCode, 1);
  assert.equal(changed.afterLast, true);
  assert.ok(changed.lines.some((line) => /^TRIPWIRE: .*Caches\/com\.invented\.product appeared$/.test(line)), changed.lines.join("\n"));
  assert.deepEqual(closeRun({ exitCode: 0, termination: RESOLVED, lockKept: false, tripwire: tripwireCase(t, false) }).exitCode, 0);
  // A failed run (1), an exception (1) and SIGINT, SIGTERM, SIGHUP (130, 143, 129) keep their codes, changed or not.
  for (const exitCode of [1, 130, 143, 129]) {
    for (const change of [false, true]) {
      const closed = closeRun({ exitCode, termination: RESOLVED, lockKept: false, tripwire: tripwireCase(t, change) });
      assert.equal(closed.exitCode, exitCode, `${exitCode} ${change}`);
      assert.equal(closed.lines.some((line) => line.startsWith("TRIPWIRE:")), change);
    }
  }
  // Done already in the run's own checks: not compared again.
  assert.deepEqual(closeRun({ exitCode: 0, termination: RESOLVED, lockKept: false, tripwire: { ...tripwireCase(t, true), done: true } }).lines, []);
  // Before any baseline (no launch reached): nothing to compare.
  assert.deepEqual(closeRun({ exitCode: 2, termination: null, lockKept: false, tripwire: null }), { exitCode: 2, lines: [], retain: false, afterLast: false });
});

test("F3 with a process of the run possibly alive the snapshot is recorded but is not the after-last observation, and the run fails and is retained", (t) => {
  const unresolved = { resolved: false, reason: "process 4243 still ran /invented/app after 20000 ms" };
  for (const exitCode of [0, 1, 130]) {
    const closed = closeRun({ exitCode, termination: unresolved, lockKept: false, tripwire: tripwireCase(t, true) });
    assert.equal(closed.exitCode, exitCode === 0 ? 1 : exitCode);
    assert.equal(closed.retain, true);
    assert.equal(closed.afterLast, false);
    assert.ok(closed.lines.includes(`UNRESOLVED TERMINATION: ${unresolved.reason}`));
    assert.ok(closed.lines.some((line) => line.includes("it is not the after-last observation")));
    assert.ok(closed.lines.some((line) => line.startsWith("TRIPWIRE:")));
  }
  const kept = closeRun({ exitCode: 0, termination: RESOLVED, lockKept: true, tripwire: tripwireCase(t, false) });
  assert.equal(kept.exitCode, 1);
  assert.equal(kept.retain, true);
  assert.equal(kept.afterLast, false);
});

test("F2 the smoke build's web view is non-persistent in every window, and a window that would persist is refused", (t) => {
  const product = JSON.parse(readFileSync(path.join(here, "..", "src-tauri", "tauri.conf.json"), "utf8")).app.windows;
  assert.ok(product.length > 0);
  assert.ok(product.every((window) => window.incognito !== true), "the product's own windows are not changed");
  const smoke = smokeWindows(product);
  assert.equal(smoke.length, product.length);
  assert.ok(smoke.every((window) => window.incognito === true && window.backgroundThrottling === "disabled"));
  assert.deepEqual(refusedSmokeWindows(smoke), []);
  // Today's override (background throttling alone) would be refused, and so would no window at all.
  const today = product.map((window) => ({ ...window, backgroundThrottling: "disabled" }));
  assert.ok(refusedSmokeWindows(today).length === product.length && refusedSmokeWindows(today).every((line) => line.startsWith("WEBVIEW-STORE-REFUSED")));
  assert.ok(refusedSmokeWindows([])[0].startsWith("WEBVIEW-STORE-REFUSED"));
  // And the runner says so in its no-launch mode.
  const result = printLaunchEnv(scratch(t));
  assert.equal(result.status, 0, result.stderr);
  assert.equal(result.stdout.match(/^web view store: (.+)$/m)?.[1], "non-persistent in every window of the smoke build");
});
