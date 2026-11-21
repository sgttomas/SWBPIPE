#!/usr/bin/env node
// The screenshot walkthrough's command (README.md):
//
//   npm run walkthrough --workspace apps/desktop [-- --out DIR] [--size WIDTHxHEIGHT] [--grep TEXT]
//
// Empties the output folder of an earlier walkthrough, drives the journeys in
// one browser, then writes INDEX.json and index.html from what was recorded.
// The exit status is 0 when the walkthrough ran to its end, whatever a person
// would have met on the way: a journey that stops is a finding in the index,
// not a failure of this command. It is 1 when the walkthrough itself could not
// run, or wrote no screens.
import { spawnSync } from "node:child_process";
import { existsSync, mkdirSync, readdirSync, rmSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { composeIndex } from "./compose-index.mjs";

const here = path.dirname(fileURLToPath(import.meta.url));
const desktopRoot = path.resolve(here, "..");

export function parseArguments(argv) {
  const options = { out: path.join(here, "output"), size: null, grep: null };
  for (let index = 0; index < argv.length; index += 1) {
    const name = argv[index];
    const value = argv[index + 1];
    if (!["--out", "--size", "--grep"].includes(name)) throw new Error(`walkthrough: unknown argument ${name}`);
    if (value === undefined) throw new Error(`walkthrough: ${name} needs a value`);
    options[name.slice(2)] = value;
    index += 1;
  }
  if (options.size !== null && !/^\d{3,4}x\d{3,4}$/.test(options.size)) throw new Error("walkthrough: --size is WIDTHxHEIGHT, for example 1440x920");
  options.out = path.resolve(options.out);
  return options;
}

// Only files this command wrote are removed: an output folder that holds
// anything else is refused, so a mistyped --out cannot empty a folder of work.
// Nothing below the folder is touched, and a folder inside it is foreign too.
const OWN_FILE = /^(\d{3}-[a-z0-9-]+(-FAILED)?\.png|part-\d{2}-[a-z0-9-]+\.json|INDEX\.json|index\.html)$/;

export function emptyOutput(out) {
  if (!existsSync(out)) { mkdirSync(out, { recursive: true }); return; }
  const entries = readdirSync(out, { withFileTypes: true });
  const foreign = entries.filter((entry) => !entry.isFile() || !OWN_FILE.test(entry.name)).map((entry) => entry.name);
  if (foreign.length > 0) {
    throw new Error(`walkthrough: ${out} holds files this command did not write (${foreign.slice(0, 5).join(", ")}); choose an empty folder with --out`);
  }
  for (const entry of entries) rmSync(path.join(out, entry.name));
}

function main() {
  const options = parseArguments(process.argv.slice(2));
  emptyOutput(options.out);
  const env = { ...process.env, SWBPIPE_WALKTHROUGH_OUT: options.out };
  if (options.size) env.SWBPIPE_WALKTHROUGH_SIZE = options.size;
  const playwright = ["playwright", "test", "--config", path.join(here, "playwright.walkthrough.config.ts")];
  if (options.grep) playwright.push("--grep", options.grep);
  const run = spawnSync("npx", playwright, { cwd: desktopRoot, env, stdio: "inherit" });
  if (run.error) throw run.error;
  const index = composeIndex(options.out, { size: options.size ?? "1440x920" });
  console.log(`walkthrough: ${index.screens} screens over ${index.journeys} journeys in ${options.out}`);
  for (const stop of index.stops) console.log(`walkthrough: ${stop.journey} stopped at "${stop.step}": ${stop.reason}`);
  console.log(`walkthrough: open ${path.join(options.out, "index.html")}`);
  if (index.screens === 0) {
    console.error(`walkthrough: no screens were written (playwright exit ${run.status})`);
    process.exit(1);
  }
}

if (process.argv[1] && path.resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  try {
    main();
  } catch (error) {
    console.error(error instanceof Error ? error.message : String(error));
    process.exit(1);
  }
}
