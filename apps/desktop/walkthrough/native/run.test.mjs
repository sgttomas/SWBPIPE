// The native walkthrough's own parts that parse, remove or write: its command line, what it removes from the output
// folder, the driver it composes, and the part file and index it writes from the driver's records. The launch itself
// is the native-smoke runners' (their tests hold it).
import assert from "node:assert/strict";
import { mkdtempSync, readFileSync, readdirSync, rmSync, writeFileSync } from "node:fs";
import os from "node:os";
import path from "node:path";
import test from "node:test";
import vm from "node:vm";
import { composeIndex } from "../compose-index.mjs";
import { NATIVE_SURFACE, NATIVE_SURFACES, PICTURE_BASIS, readStepRecords, renderScreens, screensFromRecords, writePart } from "./render.mjs";
import { existsSync, mkdirSync, symlinkSync } from "node:fs";
import { createHash } from "node:crypto";
import { DEFAULT_JOURNEYS, JOURNEYS, composeDriver, emptyNativeOutput, parseArguments, placeFiles, placementsFor, readStoredRuns, storeCopiesFor, storedRunEvidence } from "./run.mjs";

function scratch(t) {
  const folder = mkdtempSync(path.join(os.tmpdir(), "native-walkthrough-test-"));
  t.after(() => rmSync(folder, { recursive: true, force: true }));
  return folder;
}

test("arguments: --build-only alone, journeys by name, the reference system alone", () => {
  assert.deepEqual(parseArguments(["--build-only"]), { buildOnly: true, journeys: [], out: null });
  assert.match(parseArguments(["--build-only", "--journey", "report"]).error, /takes no journey/);
  assert.match(parseArguments(["--build-only", "--out", "x"]).error, /takes no journey or output folder/);
  assert.deepEqual(parseArguments([], { defaultOut: "/o" }), { buildOnly: false, journeys: [...DEFAULT_JOURNEYS], out: "/o" });
  assert.deepEqual(parseArguments(["--journey", "report,exports", "--out", "/tmp/w"]).journeys, ["report", "exports"]);
  assert.match(parseArguments(["--journey", "reference-system,report"]).error, /runs alone/);
  assert.deepEqual(parseArguments(["--journey", "reference-system"]).journeys, ["reference-system"]);
  assert.match(parseArguments(["--journey", "orientation"]).error, /unknown: orientation/);
  assert.match(parseArguments(["--journey", "report,report"]).error, /named twice/);
  assert.match(parseArguments(["--journey"]).error, /needs a value/);
  assert.match(parseArguments(["--out", "--journey"]).error, /needs a value/);
  assert.match(parseArguments(["--out", "a", "--out", "b"]).error, /given twice/);
  assert.match(parseArguments(["--all"]).error, /unknown argument/);
});

test("journeys are numbered from 20, after the browser walkthrough's, each once", () => {
  const numbers = Object.values(JOURNEYS).map((journey) => journey.number);
  assert.ok(numbers.every((number) => number >= 20 && number <= 99));
  assert.equal(new Set(numbers).size, numbers.length);
});

test("the output folder: the replaced journeys' files go; browser and other native journeys stay; the index goes", (t) => {
  const out = scratch(t);
  const kept = ["101-orientation-first-open.png", "part-01-orientation.json", "2101-report-run.png", "part-21-report.json"];
  const removed = ["2001-solve-and-results-first-open.png", "2003-solve-and-results-solve-progress-FAILED.png", "part-20-solve-and-results.json", "INDEX.json", "index.html"];
  for (const name of [...kept, ...removed]) writeFileSync(path.join(out, name), "x");
  emptyNativeOutput(out, ["solve-and-results"]);
  assert.deepEqual(readdirSync(out).sort(), [...kept].sort());
});

test("the output folder: anything a walkthrough did not write is refused and nothing is removed", (t) => {
  const out = scratch(t);
  writeFileSync(path.join(out, "2001-solve-and-results-first-open.png"), "x");
  writeFileSync(path.join(out, "notes.txt"), "keep");
  assert.throws(() => emptyNativeOutput(out, ["solve-and-results"]), /did not write/);
  assert.deepEqual(readdirSync(out).sort(), ["2001-solve-and-results-first-open.png", "notes.txt"]);
});

test("the composed driver is one script: the shared counts, the snapshot, then the driver", () => {
  const driver = readFileSync(new URL("./driver.js", import.meta.url), "utf8");
  const composed = composeDriver(driver);
  assert.doesNotThrow(() => new vm.Script(composed));
  const counts = composed.indexOf("const readScreenCounts = function readScreenCounts()");
  const snapshot = composed.indexOf("const takeSnapshot = async function takeSnapshot()");
  assert.ok(counts > 0 && snapshot > counts && composed.indexOf(driver) > snapshot);
  // The driver does not press Print, and starts no agent.
  assert.doesNotMatch(driver, /rendered-report-print"|report-print|toggle-agent|embedded_agent/);
});

test("the driver acts only through what a person can reach: every click goes through control() or acting()", () => {
  const driver = readFileSync(new URL("./driver.js", import.meta.url), "utf8");
  // control() and acting() refuse a control that is not shown or lies in a closed section, and record it on the step.
  const receivers = [...driver.matchAll(/([\w$.)\]]+)\.click\(\)/g)].map((match) => match[1]);
  const fromControl = new Set(["intent", "render"]);
  for (const receiver of receivers) assert.ok(receiver.endsWith(")") || fromControl.has(receiver), `a click on ${receiver} bypasses control() and acting()`);
  for (const name of fromControl) assert.match(driver, new RegExp(`const ${name} = await control\\(`));
  assert.match(driver, /details:not\(\[open\]\)/);
  assert.match(driver, /record\.acted_on = acted/);
});

test("records become screens: counts and the snapshot's facts kept, a stop and the steps after it as recorded", () => {
  const counts = { words_on_screen: 5, words_in_page: 9, prose_blocks_on_screen: 1, buttons_on_screen: 2, inputs_on_screen: 0, longest_scroll_screens: 1, controls: ["button: Run"] };
  const snapshot = { html: "<!doctype html><p>x</p>", width: 1440, height: 920, view_3d: "not captured", canvases: [{ label: "3D view", captured: false, reason: "its pixels read blank" }], notes: [] };
  const records = [
    { number: 2001, step: "first-open", note: "Opened", ...counts, snapshot },
    { number: 2002, step: "run", note: "Run", failed: "the control toolbar-run is disabled: \"No model\"", ...counts, snapshot },
    { number: 2003, step: "solve-progress", note: "Progress", not_reached: true }
  ];
  const screens = screensFromRecords(records, { journey: "solve-and-results", number: 20 });
  assert.equal(screens[0].window, "1440x920");
  assert.equal(screens[0].view_3d, "not captured");
  assert.equal(screens[0].words_on_screen, 5);
  assert.equal(screens[0].snapshot, undefined);
  assert.equal(screens[1].failed, records[1].failed);
  assert.deepEqual(screens[2], { number: 2003, journey: "solve-and-results", step: "solve-progress", note: "Progress", not_reached: true });
});

test("a launch that did not end cleanly is a stop after the last step recorded, unless a step already stopped", () => {
  const ended = screensFromRecords([{ number: 2001, step: "first-open", note: "Opened" }], { journey: "exports", number: 22, launchFailure: "LAUNCH-TIMED-OUT" });
  assert.deepEqual(ended.at(-1), { number: 2002, journey: "exports", step: "launch-ended", note: "The launched app, after the last step it recorded", failed: "the launch did not end cleanly: LAUNCH-TIMED-OUT" });
  assert.equal(screensFromRecords([], { journey: "exports", number: 22, launchFailure: "no report" })[0].number, 2201);
  const stopped = screensFromRecords([{ number: 2001, step: "a", note: "A", failed: "x" }], { journey: "exports", number: 22, launchFailure: "y" });
  assert.equal(stopped.length, 1);
});

test("the step records are read in step order, and only this report's", (t) => {
  const folder = scratch(t);
  const report = path.join(folder, "report-exports.json");
  writeFileSync(`${report}.step-2210.json`, JSON.stringify({ number: 2210 }));
  writeFileSync(`${report}.step-2202.json`, JSON.stringify({ number: 2202 }));
  writeFileSync(`${report}.act.json`, JSON.stringify({ act: "x" }));
  writeFileSync(path.join(folder, "report-other.json.step-2201.json"), JSON.stringify({ number: 2201 }));
  writeFileSync(`${report}.step-2203.json.tmp`, "{");
  assert.deepEqual(readStepRecords(report).map((record) => record.number), [2202, 2210]);
});

test("pictures: each snapshot rendered at its recorded size with nothing loaded, and every picture says it is redrawn", async (t) => {
  const out = scratch(t);
  const calls = [];
  const browser = {
    newContext: async (options) => {
      calls.push(["context", options]);
      return {
        route: async (pattern) => calls.push(["route", pattern]),
        newPage: async () => ({
          setContent: async (html) => calls.push(["content", html.length]),
          evaluate: async () => {},
          waitForTimeout: async () => {},
          screenshot: async ({ path: file }) => { writeFileSync(file, "png"); calls.push(["shot", path.basename(file)]); }
        }),
        close: async () => calls.push(["close"])
      };
    },
    close: async () => calls.push(["browser-close"])
  };
  const records = [
    { number: 2201, step: "solved", note: "Solved", substitutions: [{ kind: "native menu command", what: "analyze.run delivered through the menu's own dispatch" }], snapshot: { html: "<p>a</p>", width: 1440, height: 900, media: { color_scheme: "dark", reduced_motion: "no-preference", forced_colors: "none" } } },
    { number: 2202, step: "exports-page", note: "Exports", failed: "x", snapshot: { html: "<p>b</p>", width: 1440, height: 900 } },
    { number: 2203, step: "later", note: "Later", not_reached: true }
  ];
  const screens = screensFromRecords(records, { journey: "exports", number: 22 });
  await renderScreens(records, screens, { out, snapshots: path.join(out, "..", path.basename(out) + "-snapshots"), launchBrowser: async () => browser });
  t.after(() => rmSync(path.join(out, "..", path.basename(out) + "-snapshots"), { recursive: true, force: true }));
  // At the recorded size and media preferences (light where none was recorded), offline.
  assert.deepEqual(calls.filter(([kind]) => kind === "context").map(([, options]) => [options.viewport, options.colorScheme, options.offline]), [[{ width: 1440, height: 900 }, "dark", true], [{ width: 1440, height: 900 }, "light", true]]);
  assert.ok(calls.some(([kind, pattern]) => kind === "route" && pattern === "**/*"));
  assert.deepEqual(screens.map((screen) => screen.file), ["2201-exports-solved.png", "2202-exports-exports-page-FAILED.png", undefined]);
  assert.ok(screens.slice(0, 2).every((screen) => screen.picture === PICTURE_BASIS));
  writePart(out, { number: 22, name: "exports", title: "Exports & more", use_case: "AC-007 Review and export results, in part", revision: { commit: "abc123", uncommitted_paths: 0 }, build: { kind: "scripted-run build", app_sha256: "sha256:feed" }, screens, launch: { ended_cleanly: true } });
  writeFileSync(path.join(out, "part-01-first.json"), JSON.stringify({ number: 1, name: "first", title: "First", surface: "browser build", window: "1440x920", screens: [] }));
  const summary = composeIndex(out);
  assert.equal(summary.journeys, 2);
  const index = JSON.parse(readFileSync(path.join(out, "INDEX.json"), "utf8"));
  assert.deepEqual(index.journeys.map((journey) => journey.surface), ["browser build", NATIVE_SURFACE]);
  assert.equal(index.journeys[1].window, "1440x900");
  assert.equal(index.journeys[1].stopped_at, "exports-page");
  // Every screen: its revision, build and use case; the redrawn and substituted marks; the native surfaces named once.
  const native = index.screens.filter((screen) => screen.journey === "exports");
  assert.ok(native.every((screen) => screen.revision.commit === "abc123" && screen.build.app_sha256 === "sha256:feed" && screen.use_case === "AC-007 Review and export results, in part"));
  assert.deepEqual(native.map((screen) => screen.redrawn ?? null), [true, true, null]);
  assert.equal(native[0].substitutions[0].kind, "native menu command");
  assert.equal(native[1].substitutions, undefined);
  assert.deepEqual(index.native_surfaces.map((item) => item.surface), NATIVE_SURFACES.map((item) => item.surface));
  assert.ok(index.native_surfaces.every((item) => item.prevented_by && typeof item.missing_evidence === "boolean"));
  assert.ok(index.native_surfaces.some((item) => /menu bar/.test(item.surface) && /Accessibility permission/.test(item.prevented_by)));
  assert.ok(index.native_surfaces.some((item) => /window/.test(item.surface) && /Screen Recording permission/.test(item.prevented_by)));
  const html = readFileSync(path.join(out, "index.html"), "utf8");
  assert.match(html, /redrawn by Chromium/);
  assert.match(html, /not a capture of the window/);
  assert.match(html, /Substituted \(scripted-run build\): native menu command/);
  assert.match(html, /Native surfaces not pictured/);
  // Once for the journey and once for each of its two pictures.
  assert.equal(html.split(PICTURE_BASIS).length - 1, 3);
});

test("placement: each declared fixture goes to its path inside the home, {app-data}/ to the app's own data folder", (t) => {
  const root = scratch(t);
  const home = path.join(root, "home");
  mkdirSync(path.join(root, "fixtures", "models"), { recursive: true });
  mkdirSync(home);
  writeFileSync(path.join(root, "fixtures", "models", "a.json"), "{\"a\":1}");
  writeFileSync(path.join(root, "fixtures", "store.sqlite3"), "store");
  const placements = placementsFor([
    { key: "model", at: "chooser/models/a.json", from: "fixtures/models/a.json" },
    { key: "store", at: "{app-data}/openpipestress-projects.sqlite3", from: "fixtures/store.sqlite3" }
  ], { home, identifier: "com.example.app", projectRoot: root });
  const placed = placeFiles(placements);
  assert.deepEqual(placed, {
    model: path.join(home, "chooser", "models", "a.json"),
    store: path.join(home, "Library", "Application Support", "com.example.app", "openpipestress-projects.sqlite3")
  });
  assert.equal(readFileSync(placed.model, "utf8"), "{\"a\":1}");
  assert.equal(readFileSync(placed.store, "utf8"), "store");
  // Never over a file already there.
  assert.throws(() => placeFiles(placements), /EEXIST/);
});

test("placement: a path outside the home, a source outside fixtures/, a link, a missing file or a repeat is refused by name", (t) => {
  const root = scratch(t);
  const home = path.join(root, "home");
  mkdirSync(path.join(root, "fixtures"), { recursive: true });
  mkdirSync(home);
  writeFileSync(path.join(root, "fixtures", "a.json"), "{}");
  writeFileSync(path.join(root, "secret.json"), "{}");
  symlinkSync(path.join(root, "secret.json"), path.join(root, "fixtures", "link.json"));
  const options = { home, identifier: "id", projectRoot: root };
  const refused = (entry) => assert.throws(() => placementsFor([entry], options), /place\[0\]/);
  refused({ key: "a", at: "/etc/a.json", from: "fixtures/a.json" });
  refused({ key: "a", at: "chooser/../../a.json", from: "fixtures/a.json" });
  refused({ key: "a", at: "", from: "fixtures/a.json" });
  refused({ key: "a", at: "{home}/a.json", from: "fixtures/a.json" });
  refused({ key: "a", at: "a.json", from: "secret.json" });
  refused({ key: "a", at: "a.json", from: "fixtures/../secret.json" });
  refused({ key: "a", at: "a.json", from: "fixtures/link.json" });
  refused({ key: "a", at: "a.json", from: "fixtures/missing.json" });
  refused({ key: "A b", at: "a.json", from: "fixtures/a.json" });
  assert.throws(() => placementsFor([{ key: "a", at: "a.json", from: "fixtures/a.json" }, { key: "a", at: "b.json", from: "fixtures/a.json" }], options), /declared twice/);
  assert.deepEqual(readdirSync(home), []);
});

test("every declared journey placement names a committed fixture and an open the driver knows", () => {
  const projectRoot = path.resolve(import.meta.dirname, "..", "..", "..", "..");
  for (const [name, spec] of Object.entries(JOURNEYS)) {
    if (!spec.place) continue;
    const placements = placementsFor(spec.place, { home: "/nonexistent-home", identifier: "id", projectRoot });
    assert.ok(placements.length > 0, name);
    assert.ok(["model-document", "local-project"].includes(spec.open?.by), name);
    if (spec.open.file) assert.ok(placements.some((item) => item.key === spec.open.file), name);
  }
});

test("stored runs: each project's stored texts by SHA-256, as placed and at each copy; a changed or missing copy is not unchanged", async (t) => {
  const { DatabaseSync } = await import("node:sqlite");
  const root = scratch(t);
  const store = (file, record) => {
    const db = new DatabaseSync(file);
    db.exec("CREATE TABLE local_projects (project_id TEXT PRIMARY KEY, mechanics_result_json TEXT, analysis_run_json TEXT)");
    db.prepare("INSERT INTO local_projects VALUES (?, ?, ?)").run("project:a", "{\"r\":1}", record);
    db.close();
  };
  const report = path.join(root, "report-x.json");
  store(path.join(root, "placed.sqlite3"), "{\"k\":1}");
  store(`${report}.store-after-open.store.sqlite3`, "{\"k\":1}");
  store(`${report}.store-after-save.store.sqlite3`, "{\"k\":2}");
  const placed = await readStoredRuns(path.join(root, "placed.sqlite3"));
  assert.equal(placed.length, 1);
  assert.match(placed[0].result_sha256, /^[0-9a-f]{64}$/);
  const same = await storedRunEvidence({ names: ["after-open"], report, placed });
  assert.equal(same.unchanged, true);
  assert.deepEqual(same.at["after-open"], placed);
  const changed = await storedRunEvidence({ names: ["after-open", "after-save", "never"], report, placed });
  assert.equal(changed.unchanged, false);
  assert.notEqual(changed.at["after-save"][0].record_sha256, placed[0].record_sha256);
  assert.match(changed.at.never.missing, /no copy/);
});

test("store copies: each is a name, never a path; a journey that declares none has none", () => {
  const store = [{ key: "store", at: "{app-data}/openpipestress-projects.sqlite3", from: "fixtures/s.sql", build: "sqlite3-from-sql" }];
  assert.deepEqual(storeCopiesFor({ place: store, store_copies: ["after-open", "after-save"] }), ["after-open", "after-save"]);
  assert.deepEqual(storeCopiesFor({}), []);
  assert.deepEqual(storeCopiesFor(JOURNEYS["prepared-project"]), []);
  for (const outside of ["../store", "/tmp/store", "a/b", "chooser/models/x.sqlite3", "Upper", "", "x".repeat(27)]) {
    assert.throws(() => storeCopiesFor({ place: store, store_copies: [outside] }), new RegExp(`store_copies\\[0\\] \\(${outside.replace(/[.*+?^${}()|[\]\\/]/g, "\\$&")}\\): a store copy is named by .*it is not a path`), outside);
  }
  assert.throws(() => storeCopiesFor({ place: store, store_copies: ["a", "a"] }), /twice/);
  assert.throws(() => storeCopiesFor({ place: [], store_copies: ["a"] }), /needs a store placed at \{app-data\}/);
  assert.throws(() => storeCopiesFor({ place: store, store_copies: "after-open" }), /a list of names/);
});

test("the refused-stored-run journey builds its store from SQL text and copies it after the open and the save", () => {
  const spec = JOURNEYS["refused-stored-run"];
  assert.equal(spec.number, 25);
  assert.deepEqual(spec.place.map((item) => [item.at, item.build]), [["{app-data}/openpipestress-projects.sqlite3", "sqlite3-from-sql"]]);
  assert.match(spec.place[0].from, /^fixtures\/.*\.sql$/);
  assert.equal(spec.open.by, "local-project");
  const driver = readFileSync(new URL("./driver.js", import.meta.url), "utf8");
  for (const name of storeCopiesFor(spec)) assert.match(driver, new RegExp(`storeCopy\\("${name}"\\)`));
});

function sqlFixture(t) {
  const root = scratch(t);
  const home = path.join(root, "home");
  mkdirSync(path.join(root, "fixtures"), { recursive: true });
  mkdirSync(home);
  const sql = "PRAGMA journal_mode = WAL;\nPRAGMA user_version = 3;\nBEGIN TRANSACTION;\nCREATE TABLE local_projects (project_id TEXT PRIMARY KEY, mechanics_result_json TEXT, analysis_run_json TEXT);\nINSERT INTO \"local_projects\" VALUES('project:a','{}','{\"k\":1}');\nCOMMIT;\n";
  writeFileSync(path.join(root, "fixtures", "store.sql"), sql);
  const entry = { key: "store", at: "{app-data}/openpipestress-projects.sqlite3", from: "fixtures/store.sql", build: "sqlite3-from-sql" };
  return { root, home, sql, entry, options: { home, identifier: "com.example.app", projectRoot: root } };
}

test("placement sqlite3-from-sql: the outside-home, outside-fixtures, link and missing-file refusals hold for the kind", (t) => {
  const { root, home, entry, options } = sqlFixture(t);
  writeFileSync(path.join(root, "secret.sql"), "");
  symlinkSync(path.join(root, "secret.sql"), path.join(root, "fixtures", "link.sql"));
  const refused = (change, pattern) => assert.throws(() => placementsFor([{ ...entry, ...change }], options), pattern);
  refused({ at: "/etc/store.sqlite3" }, /place\[0\] \(store\): at must be a relative path inside the home/);
  refused({ at: "chooser/../../store.sqlite3" }, /place\[0\] \(store\): at must be a relative path inside the home/);
  refused({ from: "secret.sql" }, /place\[0\] \(store\): from must be a committed path under fixtures/);
  refused({ from: "fixtures/../secret.sql" }, /place\[0\] \(store\): from must be a committed path under fixtures/);
  refused({ from: "fixtures/link.sql" }, /place\[0\] \(store\): fixtures\/link.sql is not a regular file/);
  refused({ from: "fixtures/missing.sql" }, /place\[0\] \(store\): fixtures\/missing.sql does not exist/);
  refused({ build: "shell" }, /place\[0\] \(store\): build must be "sqlite3-from-sql"/);
  assert.deepEqual(readdirSync(home), []);
});

test("placement sqlite3-from-sql: an absent sqlite3 refuses by name before anything is placed", (t) => {
  const { home, entry, options } = sqlFixture(t);
  assert.throws(() => placementsFor([entry], { ...options, sqlite3: path.join(home, "no-sqlite3") }), /place\[0\] \(store\): sqlite3-from-sql needs .*no-sqlite3, which is not there to run/);
  assert.deepEqual(readdirSync(home), []);
});

test("placement sqlite3-from-sql: sqlite3 is run by its path with a fixed argument list and the SQL on standard input; a failure places nothing", (t) => {
  const { sql, entry, options } = sqlFixture(t);
  const placements = placementsFor([entry], { ...options, sqlite3: process.execPath });
  const calls = [];
  const failing = (command, args, opts) => { calls.push({ command, args, input: String(opts.input), cwd: opts.cwd, env: opts.env, shell: opts.shell }); const error = new Error("exit 1"); error.stderr = Buffer.from("Parse error near line 1\n"); throw error; };
  assert.throws(() => placeFiles(placements, { sqlite3: "/usr/bin/sqlite3", run: failing }), /place \(store\): sqlite3-from-sql: \/usr\/bin\/sqlite3 failed \(Parse error near line 1\)/);
  assert.equal(calls.length, 1);
  assert.equal(calls[0].command, "/usr/bin/sqlite3");
  assert.deepEqual(calls[0].args, ["-bail", "-batch", "built.sqlite3"]);
  assert.equal(calls[0].input, sql);
  assert.equal(calls[0].shell, undefined);
  assert.deepEqual(calls[0].env, { HOME: calls[0].cwd });
  assert.ok(calls[0].cwd.startsWith(path.dirname(placements[0].to)));
  // Nothing placed, and the build folder is gone.
  assert.deepEqual(readdirSync(path.dirname(placements[0].to)), []);
});

test("placement sqlite3-from-sql: /usr/bin/sqlite3 builds the store at its destination and both SHA-256s are recorded", { skip: !existsSync("/usr/bin/sqlite3") && "no /usr/bin/sqlite3 on this host" }, async (t) => {
  const { sql, entry, options } = sqlFixture(t);
  const placements = placementsFor([entry], options);
  const built = [];
  const placed = placeFiles(placements, { built });
  const store = placed.store;
  assert.deepEqual(readdirSync(path.dirname(store)), ["openpipestress-projects.sqlite3"]);
  const bytes = readFileSync(store);
  assert.equal(bytes.subarray(0, 16).toString("latin1"), "SQLite format 3\0");
  assert.equal(bytes[18], 2, "WAL mode");
  const sha = (data) => createHash("sha256").update(data).digest("hex");
  assert.deepEqual(built, [{ key: "store", from: "store.sql", build: "sqlite3-from-sql", sql_sha256: sha(sql), sha256: sha(bytes), bytes: bytes.length }]);
  assert.equal((await readStoredRuns(store))[0].project_id, "project:a");
  // Never over a file already there.
  assert.throws(() => placeFiles(placements, { built: [] }), /exists/);
});

test("the index carries a journey's built placements and stored runs, and a journey without them has neither", (t) => {
  const out = scratch(t);
  const screens = [{ number: 2501, journey: "x", step: "opened", note: "n" }];
  writePart(out, { number: 25, name: "x", title: "X", use_case: "u", revision: null, build: null, screens, launch: null, built_placements: [{ key: "store", sql_sha256: "a", sha256: "b", bytes: 1, build: "sqlite3-from-sql", from: "s.sql" }], stored_runs: { what: "w", placed: [], at: { o: [] }, unchanged: true } });
  writePart(out, { number: 24, name: "y", title: "Y", use_case: "u", revision: null, build: null, screens: [{ number: 2401, journey: "y", step: "opened", note: "n" }], launch: null });
  composeIndex(out);
  const index = JSON.parse(readFileSync(path.join(out, "INDEX.json"), "utf8"));
  const [y, x] = index.journeys;
  assert.equal(x.built_placements[0].sha256, "b");
  assert.equal(x.stored_runs.unchanged, true);
  assert.ok(!("built_placements" in y) && !("stored_runs" in y));
  assert.deepEqual(Object.keys(x).filter((key) => !["built_placements", "stored_runs"].includes(key)), Object.keys(y));
});

test("placement sqlite3-from-sql: a WAL left with content, or any other file beside the store, refuses and places nothing", (t) => {
  const { entry, options } = sqlFixture(t);
  const placements = placementsFor([entry], { ...options, sqlite3: process.execPath });
  const leaving = (extra) => (command, args, opts) => {
    writeFileSync(path.join(opts.cwd, "built.sqlite3"), "store");
    for (const [name, text] of Object.entries(extra)) writeFileSync(path.join(opts.cwd, name), text);
  };
  assert.throws(() => placeFiles(placements, { run: leaving({ "built.sqlite3-wal": "frames", "built.sqlite3-shm": "" }) }), /sqlite3 left built.sqlite3-shm, built.sqlite3-wal/);
  assert.throws(() => placeFiles(placements, { run: leaving({ "other": "x" }) }), /sqlite3 left other/);
  assert.deepEqual(readdirSync(path.dirname(placements[0].to)), []);
  // An empty WAL (this platform's sqlite3 keeps one after a clean close) goes with its index.
  const placed = placeFiles(placements, { run: leaving({ "built.sqlite3-wal": "", "built.sqlite3-shm": "index" }) });
  assert.deepEqual(readdirSync(path.dirname(placed.store)), ["openpipestress-projects.sqlite3"]);
});
