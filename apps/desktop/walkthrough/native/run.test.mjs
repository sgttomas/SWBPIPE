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
import { mkdirSync, symlinkSync } from "node:fs";
import { DEFAULT_JOURNEYS, JOURNEYS, composeDriver, emptyNativeOutput, parseArguments, placeFiles, placementsFor } from "./run.mjs";

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
