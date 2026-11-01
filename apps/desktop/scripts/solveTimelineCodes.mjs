#!/usr/bin/env node
// The solve timeline's exact code table (RT-U4, DEL-04-05; `src/services/solveTimelineCodes.ts`),
// generated from the sources whose codes the timeline can record. A string the timeline keeps
// must be one of these codes, so nothing identifier-shaped from model or user data can pass.
//
//   node apps/desktop/scripts/solveTimelineCodes.mjs           print the table
//   node apps/desktop/scripts/solveTimelineCodes.mjs --write   rewrite src/services/solveTimelineCodes.ts
//
// `src/services/solveTimelineCodes.test.ts` regenerates the table and fails on any difference,
// so a recording-path code added without the table (or a stale entry) fails the desktop tests.
//
// Sources:
// - solve path: code-led string literals of the webview's solve, take, decode, parse and
//   registration modules, of `handleRunWithAgentReceipt` (workspaceSession.ts), and of the native
//   solve job (`solve_jobs.rs`, `solve_worker.rs`, outside their tests, and the
//   `start_preview_mechanics_job` command in lib.rs);
// - reader verdicts: the code literals of `reader_verdict.rs` and `physics_evidence.rs` (the latter's
//   `require(…, "X")` checks publish `SOURCE_PHYSICS_X`), outside their tests;
// - the four reason codes `previewService.ts` records when registration does not happen.
// Comments are removed first. A code is UPPER_SNAKE or UPPER-KEBAB with at least two segments of two or more characters,
// immediately followed in its literal by `:`, a quote or white space.
import { readFileSync, writeFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const desktop = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const project = path.resolve(desktop, "..", "..");
export const TABLE_FILE = path.join(desktop, "src", "services", "solveTimelineCodes.ts");

// Each segment after the first holds a letter: a code is words (`UTF8`, `T1`), not a numbered
// reference such as a deliverable or decision id (`DEL-14-02`, `DEC-018`) or `SHA-256`.
const CODE = /["'`]([A-Z][A-Z0-9]+(?:[_-](?=[A-Z0-9]*[A-Z])[A-Z0-9]{2,})+)(?=[:"'`\s])/g;
const SOLVE_PATH_TS = [
  "apps/desktop/src/services/previewService.ts",
  "apps/desktop/src/services/memoryPool.ts",
  "apps/desktop/src/services/hashService.ts",
  "apps/desktop/src/services/inputManifestService.ts",
  "apps/desktop/src/services/analysisRunCompatibility.ts",
  "apps/desktop/src/features/workspace/solveJobAudit.ts",
  "apps/desktop/src/features/results/resultReader.ts",
  "apps/desktop/src/features/results/resultStanding.ts",
  "apps/desktop/src/features/results/retainedPrecisionStanding.ts",
  "apps/desktop/src/features/results/retainedPrecisionReader.ts"
];
const SOLVE_PATH_RUST = ["apps/desktop/src-tauri/src/solve_jobs.rs", "apps/desktop/src-tauri/src/solve_worker.rs"];
const READER_RUST = ["core/reporting/result_export/src/reader_verdict.rs", "core/reporting/result_export/src/physics_evidence.rs"];
export const LOCAL_REASONS = ["CAPTURE_UNAVAILABLE", "CAPTURE_MISMATCH", "CHANGED_DURING_VALIDATION", "UNCODED_REFUSAL"];

// Comments are not sources of codes (they name decisions, deliverables and constants).
const withoutComments = (text) => text.replace(/\/\*[\s\S]*?\*\//g, "").replace(/(^|[^:"'`\w])\/\/.*$/gm, "$1");
const read = (relative) => withoutComments(readFileSync(path.join(project, relative), "utf8"));
const codesIn = (text) => [...text.matchAll(CODE)].map((match) => match[1]);
const outsideTests = (text) => text.split(/\n#\[cfg\(test\)\]/)[0];
function between(text, start, end, label) {
  const from = text.indexOf(start);
  const to = text.indexOf(end, from + start.length);
  if (from < 0 || to < 0) throw new Error(`solveTimelineCodes: ${label}: anchors not found`);
  return text.slice(from, to);
}
const sorted = (values) => [...new Set(values)].sort();

/** The table, from the sources above. */
export function extractSolveTimelineCodes() {
  const solvePath = [
    ...SOLVE_PATH_TS.flatMap((file) => codesIn(read(file))),
    ...codesIn(between(read("apps/desktop/src/features/workspace/workspaceSession.ts"), "async function handleRunWithAgentReceipt", "\n  async function ", "handleRun")),
    ...SOLVE_PATH_RUST.flatMap((file) => codesIn(outsideTests(read(file)))),
    ...codesIn(between(read("apps/desktop/src-tauri/src/lib.rs"), "async fn start_preview_mechanics_job", "fn get_memory_pool_status", "start command"))
  ];
  const reader = READER_RUST.flatMap((file) => {
    const text = outsideTests(read(file));
    // A `require` check publishes its code prefixed; its bare argument is not a published code.
    const checks = new Set([...text.matchAll(/require\([^;]*?,\s*"([A-Z][A-Z0-9_]+)"\s*\)/gs)].map((match) => match[1]));
    return [...codesIn(text).filter((code) => !checks.has(code)), ...[...checks].map((check) => `SOURCE_PHYSICS_${check}`)];
  });
  return { solvePath: sorted(solvePath), reader: sorted(reader), local: [...LOCAL_REASONS] };
}

export function renderTable({ solvePath, reader, local }) {
  const list = (values) => values.map((value) => `  ${JSON.stringify(value)},`).join("\n");
  return `// GENERATED by apps/desktop/scripts/solveTimelineCodes.mjs --write; do not edit by hand.
// The exact codes the solve timeline may keep (solveTimeline.ts \`codeOrNull\`); see the script for
// their sources. solveTimelineCodes.test.ts fails when the sources and this table differ.

/** Codes of the solve, take, decode, parse and registration paths (webview and native). */
export const SOLVE_PATH_CODES: readonly string[] = [
${list(solvePath)}
];

/** Reader verdict codes (reader_verdict.rs, physics_evidence.rs). */
export const READER_VERDICT_CODES: readonly string[] = [
${list(reader)}
];

/** The reasons previewService records when a received result does not register. */
export const LOCAL_REASON_CODES: readonly string[] = [
${list(local)}
];
`;
}

if (process.argv[1] && path.resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  const table = renderTable(extractSolveTimelineCodes());
  if (process.argv.includes("--write")) writeFileSync(TABLE_FILE, table);
  else process.stdout.write(table);
}
