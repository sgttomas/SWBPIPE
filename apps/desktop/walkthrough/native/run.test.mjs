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
import { MENU_SURFACE, NATIVE_SURFACE, NATIVE_SURFACES, PICTURE_BASIS, VIEW_3D_SURFACE, nativeSurfacesFor, readStepRecords, renderScreens, screensFromRecords, writePart } from "./render.mjs";
import { mkdirSync, symlinkSync } from "node:fs";
import { DEFAULT_JOURNEYS, JOURNEYS, composeDriver, emptyNativeOutput, nativeMenuRecord, parseArguments, placeFiles, placementsFor, screenLockState, windowShownRecord } from "./run.mjs";

function scratch(t) {
  const folder = mkdtempSync(path.join(os.tmpdir(), "native-walkthrough-test-"));
  t.after(() => rmSync(folder, { recursive: true, force: true }));
  return folder;
}

test("arguments: --build-only alone, journeys by name, the reference system alone", () => {
  assert.deepEqual(parseArguments(["--build-only"]), { buildOnly: true, journeys: [], out: null, showWindow: false });
  assert.match(parseArguments(["--build-only", "--journey", "report"]).error, /takes no journey/);
  assert.match(parseArguments(["--build-only", "--out", "x"]).error, /takes no journey, output folder/);
  assert.match(parseArguments(["--build-only", "--show-window"]).error, /--show-window/);
  // Off unless given; given, it is the only change.
  assert.deepEqual(parseArguments([], { defaultOut: "/o" }), { buildOnly: false, journeys: [...DEFAULT_JOURNEYS], out: "/o", showWindow: false });
  assert.deepEqual(parseArguments(["--show-window"], { defaultOut: "/o" }), { buildOnly: false, journeys: [...DEFAULT_JOURNEYS], out: "/o", showWindow: true });
  assert.equal(parseArguments(["--journey", "prepared-project", "--show-window", "--out", "/w"]).showWindow, true);
  assert.match(parseArguments(["--show-window", "--show-window"]).error, /given twice/);
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
  const snapshot = composed.indexOf("const takeSnapshot = async function takeSnapshot(");
  assert.ok(counts > 0 && snapshot > counts && composed.indexOf(driver) > snapshot);
  // The driver does not press Print, and starts no agent.
  assert.doesNotMatch(driver, /rendered-report-print"|report-print|toggle-agent|embedded_agent/);
});

test("the driver reads the built menu once per launch and shows the window only when the plan asks", async () => {
  const driver = readFileSync(new URL("./driver.js", import.meta.url), "utf8");
  const launch = async (plan) => {
    const calls = [];
    const listeners = {};
    const document = {
      visibilityState: "hidden",
      querySelector: (selector) => (/desktop-preview-shell|tree-row-project/.test(selector) ? {} : null),
      querySelectorAll: () => [],
      hasFocus: () => false,
      body: {}
    };
    let finished;
    const done = new Promise((resolve) => { finished = resolve; });
    const window = {
      __TAURI_INTERNALS__: {
        invoke: async (command, args) => {
          calls.push([command, args]);
          if (command === "native_smoke_plan") return { ...plan, nonce: "n", journey: "probe", timeouts: { ui: 50, step: 50 } };
          if (command === "native_smoke_show_window") { document.visibilityState = "visible"; return { visible: true, focused: true, minimized: false, always_on_top: true }; }
          if (command === "native_smoke_menu_items") return [{ kind: "submenu", label: "File", enabled: true, items: [{ kind: "separator" }] }];
          if (command === "native_smoke_finish") finished(args.report);
          return null;
        }
      },
      addEventListener: (name, listener) => { listeners[name] = listener; }
    };
    // The driver's journeys table, with a probe journey of no steps.
    const script = composeDriver(driver.replace('const JOURNEYS = { "solve-and-results"', 'const JOURNEYS = { probe: async () => {}, "solve-and-results"'));
    vm.runInNewContext(script, { window, document, navigator: { userAgent: "test" }, CSS: { escape: (text) => text }, setTimeout, Date, JSON, Promise, String, Error, Boolean, Object, Math, Array, Set, Map, encodeURIComponent, getComputedStyle: () => ({}), requestAnimationFrame: (callback) => setTimeout(callback, 0), innerWidth: 1, innerHeight: 1, devicePixelRatio: 1, matchMedia: () => ({ matches: false }) });
    const report = await done;
    return { calls, report, document };
  };
  const plain = await launch({ show_window: false });
  assert.equal(plain.report.ok, true, plain.report.error);
  assert.equal(plain.calls.filter(([command]) => command === "native_smoke_menu_items").length, 1);
  assert.equal(plain.calls.filter(([command]) => command === "native_smoke_show_window").length, 0);
  const menu = plain.calls.find(([command, args]) => command === "native_smoke_checkpoint" && args.name === "menu")[1].report;
  assert.equal(menu.menus[0].label, "File");
  assert.equal(plain.calls.some(([command, args]) => command === "native_smoke_checkpoint" && args.name === "window"), false);
  const shown = await launch({ show_window: true });
  assert.equal(shown.calls.filter(([command]) => command === "native_smoke_show_window").length, 1);
  const window = shown.calls.find(([command, args]) => command === "native_smoke_checkpoint" && args.name === "window")[1].report;
  assert.equal(window.window.visible, true);
  assert.equal(window.page_visibility, "visible");
  // The window is shown before the menu is read, both before the journey starts.
  const order = shown.calls.map(([command, args]) => (command === "native_smoke_checkpoint" ? args.name : command));
  assert.ok(order.indexOf("native_smoke_show_window") < order.indexOf("native_smoke_menu_items"));
});

test("the menu reading and the window's outcome become the part file's records", () => {
  const menus = [{ kind: "submenu", label: "File", enabled: true, items: [{ kind: "item", label: "Open Model Document…", command_id: "file.open-model", enabled: true, system: false }, { kind: "separator" }] }];
  assert.deepEqual(nativeMenuRecord({ report: { nonce: "n", read_when: "once", menus } }, { journey: "exports", number: 22 }), {
    read: true, journey: "exports", number: 22, read_when: "once",
    source: "native_smoke_menu_items: the application menu as the app built and installed it, read item by item through the menu API", menus
  });
  assert.deepEqual(nativeMenuRecord({ report: { error: "NATIVE-MENU-UNAVAILABLE" } }, { journey: "exports", number: 22 }), { read: false, journey: "exports", number: 22, error: "NATIVE-MENU-UNAVAILABLE" });
  assert.match(nativeMenuRecord({ report: null, problem: "no report" }, { journey: "exports", number: 22 }).error, /no reading of the menu \(no report\)/);
  assert.deepEqual(windowShownRecord(false, null), { option: false, shown: false, why: "--show-window was not given: the window is left where the system put it" });
  assert.equal(windowShownRecord(true, null).shown, false);
  assert.match(windowShownRecord(true, { error: "boom" }).why, /failed: boom/);
  assert.equal(windowShownRecord(true, { window: { visible: true }, page_visibility: "visible" }).shown, true);
  assert.match(windowShownRecord(true, { window: { visible: true }, page_visibility: "hidden" }).why, /still reported itself hidden/);
  const locked = windowShownRecord(true, { window: { visible: true }, page_visibility: "hidden" }, { before: true, after: true });
  assert.deepEqual(locked.screen_locked, { before: true, after: true });
  assert.match(locked.why, /the host's screen was locked/);
  assert.doesNotMatch(windowShownRecord(true, { window: { visible: true }, page_visibility: "hidden" }, { before: false, after: false }).why, /locked/);
  assert.equal(screenLockState("<dict><key>IOConsoleUsers</key><array><dict><key>CGSSessionScreenIsLocked</key>\n<true/></dict></array></dict>"), true);
  assert.equal(screenLockState("<dict><key>IOConsoleUsers</key><array><dict><key>kCGSSessionOnConsoleKey</key><true/></dict></array></dict>"), false);
  assert.equal(screenLockState("nothing"), null);
  assert.equal(screenLockState(undefined), null);
});

test("native surfaces: the menu bar's reading recorded but not as the menu bar drawn, the 3D view captured or why not, and missing wins in the index", (t) => {
  const menu = { read: true, read_when: "once", menus: [] };
  const menuEntry = (surfaces) => surfaces.find((item) => item.surface === MENU_SURFACE);
  const viewEntry = (surfaces) => surfaces.find((item) => item.surface === VIEW_3D_SURFACE);
  assert.equal(menuEntry(nativeSurfacesFor()).missing_evidence, true);
  const read = menuEntry(nativeSurfacesFor({ menu }));
  assert.equal(read.missing_evidence, true);
  assert.equal(read.evidence, "missing");
  assert.equal(read.reading, "supplied");
  assert.equal(read.captured, false);
  assert.match(read.instead, /names, order and enabled state as built; not how the system draws the menu bar, where it places it/);
  assert.equal(read.prevented_by, NOT_SUPPLIED);
  assert.match(menuEntry(nativeSurfacesFor({ menu: { read: false, error: "no menu" } })).instead, /: no menu$/);
  const drawn = [{ view_3d: "captured" }, { view_3d: "not on screen" }, { view_3d: "captured" }];
  assert.equal(viewEntry(nativeSurfacesFor({ windowShown: true, screens: drawn })).missing_evidence, false);
  assert.equal(viewEntry(nativeSurfacesFor({ windowShown: true, screens: drawn })).captured, true);
  const partly = viewEntry(nativeSurfacesFor({ windowShown: true, screens: [...drawn, { view_3d: "not captured", canvases: [{ label: "3D view", captured: false, reason: "its pixels read blank" }] }] }));
  assert.equal(partly.missing_evidence, true);
  assert.match(partly.prevented_by, /--show-window\), but at 1 of 3 steps that showed the 3D view it was not captured: its pixels read blank/);
  assert.match(viewEntry(nativeSurfacesFor({ windowShown: false, screens: [{ view_3d: "not captured" }] })).prevented_by, /not given --show-window/);
  // In the index, one launch that left the evidence missing keeps the surface missing.
  const out = scratch(t);
  writePart(out, { number: 20, name: "solve-and-results", title: "A", use_case: "u", screens: [], native_menu: { ...menu, journey: "solve-and-results", number: 20 }, window_shown: { option: true, shown: true }, launch: { ended_cleanly: true, window_shown: true } });
  writePart(out, { number: 22, name: "exports", title: "B", use_case: "u", screens: [], native_menu: { read: false, journey: "exports", number: 22, error: "x" }, launch: { ended_cleanly: true } });
  composeIndex(out);
  const index = JSON.parse(readFileSync(path.join(out, "INDEX.json"), "utf8"));
  assert.equal(menuEntry(index.native_surfaces).missing_evidence, true);
  assert.deepEqual(index.native_menus.map((reading) => [reading.journey, reading.read]), [["solve-and-results", true], ["exports", false]]);
  assert.deepEqual(index.journeys[0].window_shown, { option: true, shown: true });
  assert.match(readFileSync(path.join(out, "index.html"), "utf8"), /The native menu bar, as the app built it/);
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
  // The window, the menu bar, the chooser and the print dialog: missing evidence, not supplied, never a permission prompt.
  for (const key of ["window", "menu_bar", "file_chooser", "print_dialog"]) {
    const entry = index.native_surfaces.find((item) => item.key === key);
    assert.deepEqual([entry.missing_evidence, entry.evidence, entry.prevented_by], [true, "missing", NOT_SUPPLIED], key);
  }
  assert.ok(index.native_surfaces.every((item) => !/permission/.test(item.prevented_by)));
  assert.deepEqual(index.native_menus, []);
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

// ---------------------------------------------------------------- the attended launch (--attended)

import { constants as fsConstants, copyFileSync, lstatSync } from "node:fs";
import { ATTENDED, ATTENDED_SURFACE, CAPTURE_BASIS, UNEXPLAINED_BASIS, attendedEnding, attendedStartLines, attendedWatch, emptyAttendedOutput, parseCommandLine, readCaptures, readProcessTable, writeAttendedPart } from "./run.mjs";
import { judgedPhase } from "../../native-smoke/judged-launch.mjs";
import { censusOf } from "../../native-smoke/launch-env.mjs";
import { acquireBuildLock } from "../../scripts/worktreeBuildLock.mjs";

test("attended arguments: the option, its defaults, and every refusal by name", () => {
  assert.deepEqual(parseCommandLine(["--attended", "--out", "/o"]), { buildOnly: false, journeys: [], out: "/o", showWindow: true, attended: { start: null, limitMinutes: 20, hidden: false } });
  assert.deepEqual(parseCommandLine(["--attended", "--hidden", "--limit-minutes", "1", "--out", "/o"]).attended, { start: null, limitMinutes: 1, hidden: true });
  assert.equal(parseCommandLine(["--attended", "--hidden", "--out", "/o"]).showWindow, false);
  assert.equal(parseCommandLine(["--attended", "--journey", "prepared-project", "--out", "/o"]).attended.start, "prepared-project");
  assert.equal(parseCommandLine(["--attended", "--journey", "reference-system", "--limit-minutes", "60", "--out", "/o"]).attended.start, "reference-system");
  // Refusals.
  assert.match(parseCommandLine(["--attended", "--build-only", "--out", "/o"]).error, /cannot be given with --build-only/);
  assert.match(parseCommandLine(["--build-only", "--attended"]).error, /cannot be given with --build-only/);
  assert.match(parseCommandLine(["--attended", "--journey", "prepared-project,reference-system", "--out", "/o"]).error, /at most one journey/);
  assert.match(parseCommandLine(["--attended", "--journey", "solve-and-results", "--out", "/o"]).error, /places nothing before launch/);
  assert.match(parseCommandLine(["--attended", "--journey", "orientation", "--out", "/o"]).error, /unknown journey orientation/);
  for (const limit of ["0", "61", "x", "1.5", "-1", ""]) assert.ok(parseCommandLine(["--attended", "--limit-minutes", limit, "--out", "/o"]).error, limit);
  assert.match(parseCommandLine(["--attended", "--limit-minutes", "61", "--out", "/o"]).error, /from 1 to 60/);
  assert.match(parseCommandLine(["--hidden"]).error, /--hidden is an option of --attended/);
  assert.match(parseCommandLine(["--journey", "report", "--hidden"]).error, /--hidden is an option of --attended/);
  assert.match(parseCommandLine(["--limit-minutes", "5"]).error, /--limit-minutes is an option of --attended/);
  assert.match(parseCommandLine(["--attended"]).error, /needs --out/);
  assert.match(parseCommandLine(["--attended", "--show-window", "--out", "/o"]).error, /unless --hidden/);
  assert.match(parseCommandLine(["--attended", "--attended", "--out", "/o"]).error, /given twice/);
  assert.match(parseCommandLine(["--attended", "--hidden", "--hidden", "--out", "/o"]).error, /given twice/);
  assert.match(parseCommandLine(["--attended", "--out", "/o", "--grep", "x"]).error, /unknown argument --grep/);
  // Without --attended the journeys' command line is unchanged.
  assert.deepEqual(parseCommandLine(["--journey", "report", "--out", "/w"]), { ...parseArguments(["--journey", "report", "--out", "/w"]), attended: null });
});

test("attended start lines: one per fact, a script can read them, the last says ready", () => {
  const lines = attendedStartLines({ pid: 4242, boundApp: "/b/app", launchCopy: "/c/app", finishFile: "/r/attended-finish", captures: "/o/captures", limitMinutes: 20, endsAt: "2026-01-01T00:20:00.000Z", shown: true, start: null });
  const facts = Object.fromEntries(lines.map((line) => line.match(/^attended\.([a-z_]+)=(.*)$/).slice(1)));
  assert.equal(facts.pid, "4242");
  assert.equal(facts.bound_app, "/b/app");
  assert.equal(facts.window_title_ends_with, "SWBPIPE");
  assert.equal(facts.finish_file, "/r/attended-finish");
  assert.equal(facts.captures, "/o/captures");
  assert.equal(facts.limit_minutes, "20");
  assert.match(facts.chooser, /strict chooser ends the application if a file chooser is opened/);
  assert.equal(lines.at(-1), "attended.ready=yes");
});

test("attended output: an earlier attended part goes, other journeys and an earlier captures folder are kept, and the captures folder refuses a new attended run", (t) => {
  const out = scratch(t);
  for (const name of ["part-21-report.json", "2101-report-run.png", "part-90-attended.json", "INDEX.json"]) writeFileSync(path.join(out, name), "x");
  emptyAttendedOutput(out);
  assert.deepEqual(readdirSync(out).sort(), ["2101-report-run.png", "part-21-report.json"]);
  mkdirSync(path.join(out, "captures"));
  writeFileSync(path.join(out, "captures", "a.png"), "x");
  // A journey run into the same folder keeps the attendant's folder as it is.
  emptyNativeOutput(out, ["report"]);
  assert.deepEqual(readdirSync(out), ["captures"]);
  assert.deepEqual(readdirSync(path.join(out, "captures")), ["a.png"]);
  assert.throws(() => emptyAttendedOutput(out), /earlier attended launch's captures/);
});

test("an attended launch's ending: each way named from the receipt of the owner of the child; a failed start and an unreported start never pass", () => {
  const start = { nonce: "n", ok: true };
  const outcome = (fields) => ({ status: null, signal: null, timed_out: false, ...fields });
  const delivered = { delivered: true, target: "group 7", signal: "SIGKILL" };
  const ending = (fields) => attendedEnding({ start, nonce: "n", limitMinutes: 5, ...fields });
  assert.deepEqual([ending({ outcome: outcome({ status: 0 }) }).by, ending({ outcome: outcome({ status: 0 }) }).ok], ["app-exited", true]);
  assert.equal(ending({ outcome: outcome({ status: 3 }) }).ok, false);
  assert.equal(ending({ outcome: outcome({ signal: "SIGSEGV" }) }).ok, false);
  assert.equal(ending({ outcome: outcome({ signal: "SIGKILL" }), receipt: delivered, stop: { by: "finish-file" } }).by, "finish-file");
  assert.equal(ending({ outcome: outcome({ signal: "SIGKILL" }), receipt: delivered, stop: { by: "limit" } }).by, "limit");
  assert.equal(ending({ outcome: outcome({ signal: "SIGKILL", timed_out: true }) }).by, "limit");
  // A SIGKILL the runner did not deliver is not the finish file's; a stop after the app exited signalled nothing.
  assert.deepEqual([ending({ outcome: outcome({ signal: "SIGKILL" }), stop: { by: "finish-file" } }).by, ending({ outcome: outcome({ signal: "SIGKILL" }), stop: { by: "finish-file" } }).ok], ["app-exited", false]);
  const after = ending({ outcome: outcome({ status: 0 }), receipt: null, stop: { by: "finish-file" } });
  assert.deepEqual([after.by, after.ok], ["app-exited", true]);
  assert.match(after.what, /nothing was signalled \(the application had already exited\)/);
  assert.equal(ending({ outcome: outcome({ status: 86 }) }).by, "strict-chooser");
  const failed = attendedEnding({ outcome: outcome({ status: 0 }), start: { nonce: "n", ok: false, error: "the journey declares nothing to open" }, nonce: "n" });
  assert.deepEqual([failed.by, failed.ok], ["start-failed", false]);
  assert.equal(attendedEnding({ outcome: outcome({ status: 0 }), start: { nonce: "other", ok: true }, nonce: "n" }).ok, false);
  assert.equal(attendedEnding({ outcome: null }).ok, false);
});

test("captures: each PNG with its sidecar is a capture of the actual window; without one it is unexplained, not dropped; nothing is followed", (t) => {
  const root = scratch(t);
  const folder = path.join(root, "captures");
  mkdirSync(folder);
  const png = (width, height) => { const head = Buffer.alloc(33); Buffer.from([0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a]).copy(head); head.writeUInt32BE(13, 8); head.write("IHDR", 12, "latin1"); head.writeUInt32BE(width, 16); head.writeUInt32BE(height, 20); return head; };
  writeFileSync(path.join(folder, "01-menu.png"), png(1440, 920));
  writeFileSync(path.join(folder, "01-menu.json"), JSON.stringify({ step: "file-menu-open", note: "The File menu, opened from the menu bar", acted: ["clicked File in the menu bar"] }));
  writeFileSync(path.join(folder, "02-bare.png"), png(800, 600));
  writeFileSync(path.join(folder, "03-bad.png"), png(10, 10));
  writeFileSync(path.join(folder, "03-bad.json"), JSON.stringify({ step: "", note: "n", acted: [] }));
  writeFileSync(path.join(folder, "04-fake.png"), "not a png");
  writeFileSync(path.join(folder, "04-fake.json"), JSON.stringify({ step: "s", note: "n", acted: [] }));
  writeFileSync(path.join(folder, "05-orphan.json"), JSON.stringify({ step: "s", note: "no picture", acted: [] }));
  writeFileSync(path.join(folder, "notes.txt"), "x");
  symlinkSync(path.join(folder, "01-menu.png"), path.join(folder, "06-link.png"));
  const before = readdirSync(folder).map((name) => [name, lstatSync(path.join(folder, name)).mtimeMs]);
  const found = readCaptures(folder);
  assert.deepEqual(found.screens.map((screen) => [screen.number, screen.file, screen.capture]), [
    [9001, "captures/01-menu.png", "the actual window"], [9002, "captures/02-bare.png", "unexplained"], [9003, "captures/03-bad.png", "unexplained"], [9004, "captures/04-fake.png", "unexplained"]
  ]);
  const [capture, bare, bad, fake] = found.screens;
  assert.deepEqual([capture.step, capture.note, capture.acted, capture.redrawn, capture.scripted, capture.picture, capture.png.width, capture.png.height], ["file-menu-open", "The File menu, opened from the menu bar", ["clicked File in the menu bar"], false, false, CAPTURE_BASIS, 1440, 920]);
  assert.match(bare.unexplained, /no sidecar/);
  assert.match(bad.unexplained, /names no step/);
  assert.match(fake.unexplained, /not a PNG/);
  assert.ok([bare, bad, fake].every((screen) => screen.picture === UNEXPLAINED_BASIS && screen.redrawn === false && screen.scripted === false));
  assert.deepEqual(found.records_without_picture.map((item) => [item.file, item.step]), [["captures/05-orphan.json", "s"]]);
  assert.deepEqual(found.other.map((item) => [item.file, item.why]), [["captures/06-link.png", "a link, not followed"], ["captures/notes.txt", "neither a PNG picture nor a sidecar"]]);
  // The runner neither takes nor alters a capture.
  assert.deepEqual(readdirSync(folder).map((name) => [name, lstatSync(path.join(folder, name)).mtimeMs]), before);
  assert.deepEqual(readCaptures(path.join(root, "none")), { screens: [], records_without_picture: [], other: [] });
});

test("the attended part joins the index beside the journeys, marked as captures of the actual window, the journeys' entries unchanged", (t) => {
  const out = scratch(t);
  const journeyScreens = screensFromRecords([{ number: 2401, step: "first-open", note: "Opened", window: "1440x920" }], { journey: "prepared-project", number: 24 });
  writePart(out, { number: 24, name: "prepared-project", title: "P", use_case: "u", revision: { commit: "abc", uncommitted_paths: 0 }, build: { kind: "k" }, screens: journeyScreens, launch: { ended_cleanly: true } });
  composeIndex(out);
  const journeysOnly = JSON.parse(readFileSync(path.join(out, "INDEX.json"), "utf8"));
  const htmlBefore = readFileSync(path.join(out, "index.html"), "utf8");
  const screens = [{ number: 9001, journey: "attended", file: "captures/a.png", redrawn: false, scripted: false, step: "s", note: "n", acted: ["x"], capture: "the actual window", picture: CAPTURE_BASIS }];
  writeAttendedPart(out, { title: "Attended", revision: { commit: "abc", uncommitted_paths: 0 }, build: { kind: "k" }, launch: { attended: true, scripted_steps_after_start: 0, ended_by: "finish-file" }, captures: { screens, records_without_picture: [], other: [] } });
  composeIndex(out);
  const index = JSON.parse(readFileSync(path.join(out, "INDEX.json"), "utf8"));
  assert.deepEqual(index.journeys.map((journey) => [journey.number, journey.surface]), [[24, journeysOnly.journeys[0].surface], [90, ATTENDED_SURFACE]]);
  assert.deepEqual(index.journeys[0], journeysOnly.journeys[0]);
  assert.deepEqual(index.screens.filter((screen) => screen.journey === "prepared-project"), journeysOnly.screens);
  assert.equal(index.journeys[1].launch.scripted_steps_after_start, 0);
  const captured = index.screens.find((screen) => screen.journey === "attended");
  assert.deepEqual([captured.redrawn, captured.scripted, captured.capture, captured.picture], [false, false, "the actual window", CAPTURE_BASIS]);
  assert.match(index.marks_explained.capture, /capture of the launched app's actual window/);
  assert.equal(journeysOnly.marks_explained.capture, undefined);
  const html = readFileSync(path.join(out, "index.html"), "utf8");
  assert.match(html, /An attended launch's pictures are captures of the actual window/);
  assert.match(html, /Journeys of the launched app are not captures of its window/);
  assert.doesNotMatch(htmlBefore, /attended launch/);
});

test("the driver's attended start: the start checkpoint, then no step and no finish; a failed start is recorded and ends the app", async () => {
  const driver = readFileSync(new URL("./driver.js", import.meta.url), "utf8");
  const launch = async (plan, waitMs = 400) => {
    const calls = [];
    const document = { visibilityState: "hidden", querySelector: (selector) => (/desktop-preview-shell|tree-row-project/.test(selector) ? {} : null), querySelectorAll: () => [], hasFocus: () => false, body: {} };
    let finished = null;
    const window = {
      __TAURI_INTERNALS__: {
        invoke: async (command, args) => {
          calls.push([command, args]);
          if (command === "native_smoke_plan") return { ...plan, nonce: "n", journey: "attended", timeouts: { ui: 50, step: 50 } };
          if (command === "native_smoke_menu_items") return [];
          if (command === "native_smoke_clear_choices") return 0;
          if (command === "native_smoke_finish") finished = args.report;
          return null;
        }
      },
      addEventListener: () => {}
    };
    vm.runInNewContext(composeDriver(driver), { window, document, navigator: { userAgent: "test" }, CSS: { escape: (text) => text }, setTimeout, Date, JSON, Promise, String, Error, Boolean, Object, Math, Array, Set, Map, encodeURIComponent, getComputedStyle: () => ({}), requestAnimationFrame: (callback) => setTimeout(callback, 0), innerWidth: 1, innerHeight: 1, devicePixelRatio: 1, matchMedia: () => ({ matches: false }) });
    await new Promise((resolve) => setTimeout(resolve, 1500 + waitMs));
    const checkpoints = calls.filter(([command]) => command === "native_smoke_checkpoint").map(([, args]) => args);
    return { calls, finished, checkpoints };
  };
  const plain = await launch({ attended: true, show_window: false, start: null });
  const start = plain.checkpoints.find((item) => item.name === "start")?.report;
  assert.deepEqual([start?.nonce, start?.attended, start?.ok, start?.start], ["n", true, true, null]);
  assert.equal(plain.finished, null, "the driver never ends an attended launch that started");
  assert.deepEqual(plain.checkpoints.map((item) => item.name), ["menu", "start"]);
  assert.equal(plain.calls.some(([command]) => command === "native_smoke_menu" || command === "native_smoke_choose"), false);
  const failed = await launch({ attended: true, show_window: false, start: "x", open: { by: "unknown" } });
  const failedStart = failed.checkpoints.find((item) => item.name === "start")?.report;
  assert.deepEqual([failedStart?.ok, failedStart?.error], [false, "the journey declares an unknown way to open: unknown"]);
  assert.match(failed.finished?.error ?? "", /the attended start failed/);
  assert.ok(failed.calls.some(([command]) => command === "native_smoke_clear_choices"));
});

// The three ways an attended launch ends, and the strict chooser's exit, through the real judged launch and the watcher,
// with a stand-in app as `native-smoke/judged-launch.test.mjs` stubs it: a private copy of this Node executable
// running an invented script, so that the census finds it as it finds the app's copy. The runner holds a build lock in
// a scratch folder, so the launch leads a process group of its own as it does in the command.
const ATTENDED_STAND_IN = `
const fs = require("node:fs");
const plan = JSON.parse(process.env.SWBPIPE_NATIVE_SMOKE_PLAN);
const report = process.env.SWBPIPE_NATIVE_SMOKE_REPORT;
fs.writeFileSync(report + ".start.json", JSON.stringify({ nonce: plan.nonce, attended: true, ok: true }));
if (plan.mode === "quit") setTimeout(() => process.exit(0), 1500);
else if (plan.mode === "chooser") setTimeout(() => process.exit(86), 1500);
else setInterval(() => {}, 1000);
`;

test("the ways an attended launch ends, each through runUnderLock's abort and each censused", { skip: !["darwin", "linux"].includes(process.platform) && "the census runs on macOS and Linux" }, async (t) => {
  const dir = scratch(t);
  const executable = path.join(dir, "stand-in-app");
  copyFileSync(process.execPath, executable, fsConstants.COPYFILE_FICLONE);
  const script = path.join(dir, "stand-in.cjs");
  writeFileSync(script, ATTENDED_STAND_IN);
  const lock = acquireBuildLock({ purpose: "attended launch test", root: dir });
  try {
    const marksAbsent = (command) => ({ status: command.endsWith("pgrep") ? 1 : 0, stdout: "", stderr: "", error: undefined });
    const nonce = "0123456789abcdef0123456789abcdef";
    const wrapper = process.platform === "darwin" ? ["/usr/bin/caffeinate", ["-di", executable, script]] : [executable, [script]];
    const pause = (ms) => new Promise((resolve) => setTimeout(resolve, ms));
    async function attendedLaunch(mode, { timeoutMs = 30_000, limitMs = 60_000, finishAfterReady = false, finishBeforeStart = false, finishAfterExit = false } = {}) {
      const root = mkdtempSync(path.join(dir, `root-${mode}-`));
      const report = path.join(root, "report-attended.json");
      const finishFile = path.join(root, ATTENDED.finish_file);
      const launches = {};
      const lines = [];
      let ready;
      const readyNow = new Promise((resolve) => { ready = resolve; });
      const watch = attendedWatch({ executable, finishFile, startReport: `${report}.start.json`, census: censusOf, limitMs, pollMs: 100, onReady: ({ pid }) => {
        lines.push(...attendedStartLines({ pid, boundApp: executable, launchCopy: executable, finishFile, captures: root, limitMinutes: 1, endsAt: "t", shown: false, start: null }));
        ready(pid);
      } });
      const result = judgedPhase({
        phase: mode, checkName: "judged", launches, check: () => {}, summaryPath: path.join(root, "summary.json"),
        executable, command: wrapper[0], commandArgs: wrapper[1], cwd: root, stdio: ["ignore", "ignore", "ignore"],
        env: { ...process.env, SWBPIPE_NATIVE_SMOKE_PLAN: JSON.stringify({ mode, nonce }), SWBPIPE_NATIVE_SMOKE_REPORT: report },
        timeoutMs, reportPath: `${report}.start.json`, nonce, censusBoundMs: 5_000, marksRun: marksAbsent, abort: watch.signal,
        beforeStart: () => {
          // Before the child exists and before its pid is known: the stop is delivered once it does.
          if (finishBeforeStart) writeFileSync(finishFile, "");
          watch.start({ nonce });
        }
      });
      if (finishAfterReady) { await readyNow; writeFileSync(finishFile, ""); }
      const judged = await result;
      if (finishAfterExit) { writeFileSync(finishFile, ""); await pause(400); }
      const watched = watch.stop();
      const receipt = launches[mode].abort;
      const ending = attendedEnding({ outcome: launches[mode].outcome, receipt, stop: watched.stop, start: watched.start, nonce, limitMinutes: 1 });
      return { judged, watched, receipt, ending, lines, outcome: launches[mode].outcome };
    }
    const quit = await attendedLaunch("quit");
    assert.deepEqual([quit.ending.by, quit.ending.ok], ["app-exited", true], quit.ending.what);
    assert.equal(quit.judged.census.resolved, true);
    assert.deepEqual([quit.watched.stop, quit.receipt], [null, null]);
    assert.match(quit.lines.find((line) => line.startsWith("attended.pid=")) ?? "", /^attended\.pid=\d+$/);
    assert.equal(quit.lines.at(-1), "attended.ready=yes");

    const finished = await attendedLaunch("idle", { finishAfterReady: true });
    assert.deepEqual([finished.ending.by, finished.ending.ok], ["finish-file", true], finished.ending.what);
    assert.deepEqual([finished.receipt.delivered, finished.receipt.signal], [true, "SIGKILL"], JSON.stringify(finished.receipt));
    assert.deepEqual([finished.outcome.signal, finished.outcome.timed_out], ["SIGKILL", false]);
    assert.equal(finished.judged.census.resolved, true);
    assert.deepEqual(censusOf(executable), { ok: true, pids: [] });

    const early = await attendedLaunch("idle", { finishBeforeStart: true });
    assert.deepEqual([early.ending.by, early.receipt.delivered, early.outcome.signal], ["finish-file", true, "SIGKILL"], early.ending.what);
    assert.equal(early.judged.census.resolved, true);

    const exited = await attendedLaunch("quit", { finishAfterExit: true });
    assert.deepEqual([exited.ending.by, exited.ending.ok, exited.watched.stop.by, exited.receipt, exited.outcome.signal, exited.outcome.status], ["app-exited", true, "finish-file", null, null, 0], exited.ending.what);
    assert.match(exited.ending.what, /nothing was signalled/);

    const limited = await attendedLaunch("idle", { limitMs: 2_500 });
    assert.deepEqual([limited.ending.by, limited.ending.ok], ["limit", true], limited.ending.what);
    assert.deepEqual([limited.receipt.delivered, limited.outcome.signal, limited.outcome.timed_out], [true, "SIGKILL", false]);
    assert.equal(limited.judged.census.resolved, true);

    const backstop = await attendedLaunch("idle", { timeoutMs: 2_500 });
    assert.deepEqual([backstop.ending.by, backstop.outcome.timed_out, backstop.outcome.signal, backstop.receipt], ["limit", true, "SIGKILL", null], backstop.ending.what);

    const chooser = await attendedLaunch("chooser");
    assert.deepEqual([chooser.ending.by, chooser.ending.ok], ["strict-chooser", true], chooser.ending.what);
    assert.equal(chooser.judged.census.resolved, true);
    assert.ok(readProcessTable().has(process.pid));
  } finally {
    lock.release();
  }
});

// ---------------------------------------------------------------- native surfaces from attended captures

import { NOT_SUPPLIED, SURFACE_KEYS } from "./render.mjs";

/** A folder with one journey's part at `journeyRevision` and an attended part at `captureRevision` whose captures carry `sidecars`. */
function surfacesIndex(t, { journeyRevision = "abc", captureRevision = "abc", sidecars = [], launch = {} } = {}) {
  const out = scratch(t);
  writePart(out, { number: 22, name: "exports", title: "E", use_case: "u", revision: { commit: journeyRevision, uncommitted_paths: 0 }, build: { kind: "k", app_sha256: "sha256:feed" }, screens: [], native_menu: { read: true, read_when: "once", menus: [], journey: "exports", number: 22 }, launch: { ended_cleanly: true } });
  const folder = path.join(out, "captures");
  mkdirSync(folder);
  const png = Buffer.alloc(33);
  Buffer.from([0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a]).copy(png);
  png.write("IHDR", 12, "latin1");
  sidecars.forEach((sidecar, index) => {
    const base = `${String(index + 1).padStart(2, "0")}-capture`;
    if (sidecar.picture !== false) writeFileSync(path.join(folder, `${base}.png`), png);
    const { picture, ...claims } = sidecar;
    writeFileSync(path.join(folder, `${base}.json`), JSON.stringify({ step: `step-${index + 1}`, note: "n", acted: ["looked"], ...claims }));
  });
  const captures = readCaptures(folder);
  writeAttendedPart(out, { title: "Attended", revision: { commit: captureRevision, uncommitted_paths: 0 }, build: { kind: "k", app_sha256: "sha256:beef" }, launch: { attended: true, prevented: [], ...launch }, captures });
  composeIndex(out);
  const index = JSON.parse(readFileSync(path.join(out, "INDEX.json"), "utf8"));
  return { index, html: readFileSync(path.join(out, "index.html"), "utf8"), captures, entry: (key) => index.native_surfaces.find((item) => item.key === key) };
}

test("native surfaces: with no attended capture, the window, menu bar, chooser and print dialog are each missing evidence, not supplied", (t) => {
  const { entry, index, html } = surfacesIndex(t);
  for (const key of ["window", "menu_bar", "file_chooser", "print_dialog"]) {
    assert.deepEqual([entry(key).missing_evidence, entry(key).evidence, entry(key).prevented_by, entry(key).captured], [true, "missing", NOT_SUPPLIED, false], key);
  }
  assert.equal(entry("other_native_dialog"), undefined);
  assert.match(index.marks_explained.native_surfaces, /missing evidence \(not supplied: no attended capture of it in this walkthrough\)/);
  assert.match(html, /not supplied: no attended capture of this surface in this walkthrough\. Instead: the scripted runner takes no capture of the window/);
  // Only the attended part's own record is the scripted runner's statement: it presses no Print.
  assert.match(entry("print_dialog").instead, /the scripted runner presses no Print/);
});

test("native surfaces: a capture declaring menu_bar at the walkthrough's revision supplies it and is named; the others stay missing", (t) => {
  const { entry, html } = surfacesIndex(t, { sidecars: [{ surfaces: ["menu_bar"] }] });
  const menuBar = entry("menu_bar");
  assert.deepEqual([menuBar.evidence, menuBar.missing_evidence, menuBar.captured], ["supplied", false, true]);
  assert.deepEqual(menuBar.captures.map((ref) => [ref.file, ref.step, ref.revision.commit, ref.build.app_sha256]), [["captures/01-capture.png", "step-1", "abc", "sha256:beef"]]);
  assert.equal(entry("window").missing_evidence, true);
  assert.match(html, /supplied by attended capture <a href="captures\/01-capture\.png">step-1<\/a> \(revision abc, app beef\)/);
});

test("native surfaces: a capture at another revision than the walkthrough's screens supplies nothing, and says why", (t) => {
  const { entry, html } = surfacesIndex(t, { journeyRevision: "abc", captureRevision: "def", sidecars: [{ surfaces: ["menu_bar", "window"] }] });
  for (const key of ["menu_bar", "window"]) {
    assert.deepEqual([entry(key).evidence, entry(key).missing_evidence], ["missing", true], key);
    assert.deepEqual(entry(key).not_supplying.map((ref) => [ref.file, ref.why]), [["captures/01-capture.png", "it is at revision def, not the walkthrough's abc"]], key);
  }
  assert.match(html, /Not counted: captures\/01-capture\.png \(it is at revision def, not the walkthrough's abc\)/);
});

test("native surfaces: a recorded attempt and what prevented it makes a surface unseen; nothing else does", (t) => {
  const { entry, html } = surfacesIndex(t, {
    sidecars: [{ prevented: { surface: "print_dialog", what: "Print opened the print view's window; this build shows no print dialog" } }, { picture: false, prevented: [{ surface: "other_native_dialog", what: "the alert closed before it could be captured" }] }],
    launch: { prevented: [{ surface: "file_chooser", what: "the strict chooser ended the application before any panel (exit 86)" }] }
  });
  assert.deepEqual([entry("print_dialog").evidence, entry("print_dialog").unseen, entry("print_dialog").missing_evidence, entry("print_dialog").prevented_by], ["unseen", true, false, "Print opened the print view's window; this build shows no print dialog"]);
  assert.equal(entry("print_dialog").attempts[0].file, "captures/01-capture.png");
  assert.deepEqual([entry("file_chooser").evidence, entry("file_chooser").attempts[0].source], ["unseen", "the attended launch's record"]);
  assert.deepEqual([entry("other_native_dialog").evidence, entry("other_native_dialog").attempts[0].file], ["unseen", "captures/02-capture.json"]);
  assert.equal(entry("window").evidence, "missing");
  assert.match(html, /unseen; an attempt was recorded and was prevented: the strict chooser ended the application/);
  // At another revision an attempt does not count.
  const elsewhere = surfacesIndex(t, { captureRevision: "def", sidecars: [{ prevented: { surface: "print_dialog", what: "x" } }] });
  assert.deepEqual([elsewhere.entry("print_dialog").evidence, elsewhere.entry("print_dialog").not_supplying[0].kind], ["missing", "attempt"]);
});

test("native surfaces: an unknown surface name in a sidecar is listed as unexplained, not dropped; a malformed claim leaves the capture unexplained", (t) => {
  const { entry, index, html, captures } = surfacesIndex(t, { sidecars: [{ surfaces: ["menu_bar", "dock"], prevented: { surface: "touch_bar", what: "none here" } }, { surfaces: "window" }, { prevented: { surface: "window" } }] });
  assert.deepEqual(captures.screens.map((screen) => screen.capture), ["the actual window", "unexplained", "unexplained"]);
  assert.match(captures.screens[1].unexplained, /surfaces is not a list/);
  assert.match(captures.screens[2].unexplained, /prevented is not \{ surface, what \}/);
  assert.equal(entry("menu_bar").evidence, "supplied");
  assert.deepEqual(index.native_surfaces_unexplained, [{ name: "dock", file: "captures/01-capture.png", member: "surfaces" }, { name: "touch_bar", file: "captures/01-capture.png", member: "prevented", what: "none here" }]);
  assert.match(html, /Unexplained surface names in sidecars: dock \(captures\/01-capture\.png, surfaces\); touch_bar/);
  // An unexplained capture's claims count for nothing.
  assert.equal(entry("window").evidence, "missing");
});

test("native surfaces: the 3D view keeps its own logic, and an attended capture can supply it too", (t) => {
  const { entry } = surfacesIndex(t);
  assert.equal(entry("view_3d").missing_evidence, true);
  assert.match(entry("view_3d").prevented_by, /not given --show-window/);
  const captured = surfacesIndex(t, { sidecars: [{ surfaces: ["view_3d"] }] });
  assert.deepEqual([captured.entry("view_3d").evidence, captured.entry("view_3d").captures[0].file], ["supplied", "captures/01-capture.png"]);
  assert.deepEqual(Object.keys(SURFACE_KEYS), ["window", "menu_bar", "file_chooser", "print_dialog", "view_3d", "other_native_dialog"]);
});

// ---------------------------------------------------------------- a refused launch changes nothing

import { spawnSync as spawnChild } from "node:child_process";
import { outputRefusals } from "./run.mjs";

/** Every file under `folder`, by relative path, with its bytes (hex), and every folder: what "intact" compares. */
function folderBytes(folder) {
  const seen = {};
  const walk = (dir, prefix) => {
    for (const entry of readdirSync(dir, { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
      const relative = `${prefix}${entry.name}`;
      if (entry.isDirectory()) { seen[`${relative}/`] = "folder"; walk(path.join(dir, entry.name), `${relative}/`); }
      else seen[relative] = readFileSync(path.join(dir, entry.name)).toString("hex");
    }
  };
  walk(folder, "");
  return seen;
}

/** A completed attended output folder: a journey's part and picture, the attended part, both indexes and captures. */
function completedAttendedFolder(t) {
  const out = scratch(t);
  writePart(out, { number: 24, name: "prepared-project", title: "P", use_case: "u", revision: { commit: "abc", uncommitted_paths: 0 }, build: { kind: "k" }, screens: [], launch: { ended_cleanly: true } });
  writeFileSync(path.join(out, "2401-prepared-project-first-open.png"), "picture");
  writeAttendedPart(out, { title: "Attended", revision: { commit: "abc", uncommitted_paths: 0 }, build: { kind: "k" }, launch: { attended: true }, captures: { screens: [], records_without_picture: [], other: [] } });
  composeIndex(out);
  mkdirSync(path.join(out, "captures"));
  writeFileSync(path.join(out, "captures", "01.png"), "capture");
  writeFileSync(path.join(out, "captures", "01.json"), "{}");
  return out;
}

test("a refused attended launch changes nothing: the completed output folder is byte for byte intact after each refusal", (t) => {
  const out = completedAttendedFolder(t);
  const before = folderBytes(out);
  assert.ok(before["INDEX.json"] && before["index.html"] && before["part-90-attended.json"] && before["captures/01.png"]);
  assert.match(outputRefusals(out, { attended: true }), /earlier attended launch's captures/);
  assert.deepEqual(folderBytes(out), before);
  assert.throws(() => emptyAttendedOutput(out), /earlier attended launch's captures/);
  assert.deepEqual(folderBytes(out), before);
  // A file a walkthrough did not write is refused first, for an attended launch and a journey's alike.
  writeFileSync(path.join(out, "notes.txt"), "keep");
  const withNotes = folderBytes(out);
  assert.match(outputRefusals(out, { attended: true }), /did not write \(notes\.txt\)/);
  assert.match(outputRefusals(out), /did not write \(notes\.txt\)/);
  assert.throws(() => emptyNativeOutput(out, ["prepared-project"]), /did not write/);
  assert.deepEqual(folderBytes(out), withNotes);
  // Without --attended the captures folder is no refusal.
  rmSync(path.join(out, "notes.txt"));
  assert.equal(outputRefusals(out), null);
});

test("the command itself refuses an attended launch into a completed attended folder and leaves it intact", (t) => {
  const out = completedAttendedFolder(t);
  const before = folderBytes(out);
  const run = spawnChild(process.execPath, [new URL("./run.mjs", import.meta.url).pathname, "--attended", "--hidden", "--out", out], { encoding: "utf8", timeout: 120_000 });
  assert.equal(run.status, 2, run.stderr);
  if (process.platform === "darwin") assert.match(run.stderr, /earlier attended launch's captures|build lock/);
  assert.deepEqual(folderBytes(out), before);
});
