// What a native-smoke launch is given and what it must not reach
// (`launch-env.mjs`, `run.mjs --print-launch-env`), without launching the app.
// Every folder here is invented under the system temporary folder; the real
// user's Library is never examined.
//
//   node --test apps/desktop/native-smoke/launch-env.test.mjs
import { test } from "node:test";
import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { existsSync, mkdirSync, mkdtempSync, readdirSync, realpathSync, rmSync, utimesSync, writeFileSync } from "node:fs";
import os from "node:os";
import path from "node:path";
import { fileURLToPath } from "node:url";
import {
  HOME_KEYS, PARENT_ALLOWLIST, RUNNER_KEYS, buildLaunchEnv, createThrowawayHome, refusedLaunchEnv, statWatched, tripwireChanges, watchedLibraryPaths
} from "./launch-env.mjs";

const here = path.dirname(fileURLToPath(import.meta.url));
const CREDENTIAL = "SWBPIPE_TEST_INVENTED_PROVIDER_API_KEY";
const CREDENTIAL_VALUE = "sk-invented-0123456789abcdef";
const RUNNER = { SWBPIPE_NATIVE_SMOKE_DRIVER: "/invented/driver.js", SWBPIPE_NATIVE_SMOKE_PLAN: "{}", SWBPIPE_NATIVE_SMOKE_REPORT: "/invented/report.json" };

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
});

// The runner itself, in its no-launch mode: it takes no lock, builds nothing and launches nothing.
function printLaunchEnv(homeRoot, extra = []) {
  return spawnSync(process.execPath, [path.join(here, "run.mjs"), "--print-launch-env", "--home-root", homeRoot, ...extra], {
    encoding: "utf8",
    env: { ...process.env, [CREDENTIAL]: CREDENTIAL_VALUE }
  });
}
const printedHome = (stdout) => stdout.match(/^HOME and CFFIXED_USER_HOME: (.+)$/m)?.[1];

test("(c) --print-launch-env: the home is under --home-root, TMPDIR inside it, names only, and the home is removed on exit", { skip: process.platform !== "darwin" && "the runner is macOS only" }, (t) => {
  const root = scratch(t);
  const result = printLaunchEnv(root);
  assert.equal(result.status, 0, result.stderr);
  const home = printedHome(result.stdout);
  assert.ok(home && path.dirname(home) === realpathSync(root), result.stdout);
  assert.equal(result.stdout.match(/^TMPDIR: (.+)$/m)?.[1], path.join(home, "tmp"));
  assert.equal(result.stdout.match(/^launch environment variable names: (.+)$/m)?.[1], "CFFIXED_USER_HOME HOME SWBPIPE_NATIVE_SMOKE_DRIVER SWBPIPE_NATIVE_SMOKE_PLAN SWBPIPE_NATIVE_SMOKE_REPORT TMPDIR");
  assert.equal(result.stdout.includes(CREDENTIAL), false);
  assert.equal(result.stdout.includes(CREDENTIAL_VALUE), false);
  assert.equal(existsSync(home), false, "home removed on exit");
  assert.deepEqual(readdirSync(root), []);
});

test("(c) --print-launch-env --keep keeps the home", { skip: process.platform !== "darwin" && "the runner is macOS only" }, (t) => {
  const root = scratch(t);
  const result = printLaunchEnv(root, ["--keep"]);
  assert.equal(result.status, 0, result.stderr);
  const home = printedHome(result.stdout);
  assert.ok(home && path.dirname(home) === realpathSync(root) && existsSync(path.join(home, "tmp")), result.stdout);
});

test("(c) --home-root that does not exist is refused before anything is created", { skip: process.platform !== "darwin" && "the runner is macOS only" }, (t) => {
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
