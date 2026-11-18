// The evidence journeys' runner, without a build or a launch (`node --test`): it refuses a command line it does not
// accept before it takes the build lock, and its source starts the app only through the shared launch pieces.
import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { readFileSync } from "node:fs";
import path from "node:path";
import test from "node:test";
import vm from "node:vm";
import { fileURLToPath } from "node:url";

const here = path.dirname(fileURLToPath(import.meta.url));
const runner = path.join(here, "evidence-journeys.mjs");

// Run the maintained driver against a modelled DOM/IPC. Virtual time makes a swallowed refusal fail the test
// without waiting five minutes; no native command, app, chooser or real timer is used.
async function referenceOpen(outcome) {
  let now = 0;
  let shown = "Local project store not opened.";
  let sparse = false;
  const menus = [];
  const sleeps = [];
  let finish;
  const completed = new Promise((resolve) => { finish = resolve; });
  const plan = { phase: "reference", nonce: "reference-open-test", bundled_project_id: "demo", project_id: "reference", open: "/test-home/reference_system.json" };
  const context = {
    CSS: { escape: (value) => value },
    navigator: { userAgent: "test" },
    Date: class extends Date { static now() { return now; } },
    setTimeout: (callback, ms) => { sleeps.push(ms); now += ms + 1; queueMicrotask(callback); },
    document: {
      querySelector: (selector) => {
        const id = /^\[data-testid="(.*)"\]$/.exec(selector)?.[1];
        if (id === "desktop-preview-shell" || id === "tree-row-project-demo") return {};
        if (id === "local-project-message") return { textContent: shown };
        if (id === "tree-row-project-reference") return shown.startsWith("Opened model document ") ? {} : null;
        if (id === "workspace-section-solve") return { offsetParent: {} };
        if (id === "solver-mode-sparse") return { click: () => { sparse = true; }, getAttribute: () => String(sparse) };
        return null;
      }
    },
    __TAURI_INTERNALS__: {
      invoke: async (command, args) => {
        if (command === "native_smoke_plan") return plan;
        if (["native_smoke_log", "native_smoke_checkpoint", "native_smoke_choose"].includes(command)) return null;
        if (command === "native_smoke_menu") {
          menus.push(args.commandId);
          if (args.commandId === "file.open-model") { shown = outcome; return null; }
          if (args.commandId === "analyze.run") throw new Error("TEST-SOLVE-REACHED");
        }
        if (command === "native_smoke_finish") { finish(args.report); return null; }
        throw new Error(`Unexpected test IPC ${command}`);
      }
    }
  };
  context.window = context;
  vm.runInNewContext(readFileSync(path.join(here, "evidence-journeys.js"), "utf8"), context);
  return { report: await completed, menus, sleeps };
}

test("a named document-open refusal fails the reference journey immediately with every diagnostic", async () => {
  const refusal = "Open model document refused (reference_system.json): MODEL-FILE-SHAPE at data_boundary: data_boundary is missing (object required). | MODEL-FILE-SHAPE at diagnostics: diagnostics is missing (array required).";
  const { report, menus, sleeps } = await referenceOpen(refusal);
  assert.equal(report.ok, false);
  assert.equal(report.nonce, "reference-open-test");
  assert.equal(report.opened, refusal);
  assert.equal(report.error.split("\n")[0], refusal);
  assert.deepEqual(menus, ["file.open-model"]);
  assert.deepEqual(sleeps, [], "a known refusal does not wait for the absent tree or invoke solve");
});

test("an opened reference document still advances to the desktop solve command", async () => {
  const opened = "Opened model document reference_system.json (1234 bytes; schema_version 0.9.0, current). Not saved as a local project yet.";
  const { report, menus } = await referenceOpen(opened);
  assert.equal(report.opened, opened);
  assert.deepEqual(menus, ["file.open-model", "analyze.run"]);
  assert.equal(report.error.split("\n")[0], "TEST-SOLVE-REACHED");
});

test("a command line the runner does not accept is refused before the build lock, any build or any launch", () => {
  for (const args of [[], ["--skip-build", "--all"], ["--all", "--library-listing", "/x"], ["--build-only", "--all"], ["--journey", "reference-system,report-exact"]]) {
    const result = spawnSync(process.execPath, [runner, ...args], { encoding: "utf8", timeout: 60_000, env: { PATH: process.env.PATH ?? "/usr/bin:/bin" } });
    const said = `${result.stdout}\n${result.stderr}`;
    assert.equal(result.status, 2, `${args.join(" ")}: ${said}`);
    assert.match(said, /evidence journeys: (macOS only|unknown arguments|choose --all|--build-only builds|.*runs alone)/, said);
    assert.doesNotMatch(said, /^\$ \(/m, "no build command was started");
    assert.doesNotMatch(said, /launching|== .*: phase|build lock/, "neither the lock nor a launch was reached");
  }
});

test("the runner starts the app only through judgedPhase or, in the declared fault phase, runUnderLock, and signals no process itself", () => {
  const source = readFileSync(runner, "utf8");
  assert.doesNotMatch(source, /\bimport\s*\{[^}]*\bspawn\b[^}]*\}\s*from\s*"node:child_process"/, "no direct spawn");
  assert.doesNotMatch(source, /process\.kill\(|\.kill\("SIG/, "no signal sent by the runner itself");
  const starts = [...source.matchAll(/runUnderLock\(([^,]+),/g)].map((match) => match[1].trim());
  // The build's commands (`run` and runCargo) and the declared fault phase's one launch of the app itself.
  assert.deepEqual(starts.sort(), ["binary", "command", "command"].sort());
  assert.equal((source.match(/judgedPhase\(\{/g) ?? []).length, 1, "one judged launch path");
  assert.match(source, /runUnderLock\(binary, \[\], \{[^}]*abort: controller\.signal \}\)/);
});
