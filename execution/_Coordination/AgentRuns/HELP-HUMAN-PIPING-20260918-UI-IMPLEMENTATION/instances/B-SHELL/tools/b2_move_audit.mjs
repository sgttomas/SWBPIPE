#!/usr/bin/env node
// b2_move_audit.mjs: audit for slice B2 (state extraction, no behaviour change).
//
// It answers one question mechanically: which statements of the base `src/App.tsx` arrived in the
// candidate byte for byte, which arrived changed, which are missing, and which candidate statements
// are new. It parses both sides with the TypeScript compiler API, so it compares statements, not lines.
//
// Usage, from anywhere inside the worktree:
//   node <this file> <base-rev> [<head-rev>]
// With no <head-rev> the candidate is the working tree. Output is a Markdown report on stdout.
// Exit status: 0 when nothing is CHANGED or MISSING and the effect order holds; 1 otherwise.
//
// What it compares:
//   1. Every top-level statement of the base App.tsx (imports excepted) against the top-level
//      statements of the candidate App.tsx and of every new or modified non-test module under
//      src/features/workspace/. A leading `export` is ignored, because a moved helper gains one.
//   2. Every statement of the base `AppSession` body (state cells, render-time ref syncs, memos,
//      handlers, effects, the early return, the JSX return) against the body statements of every
//      top-level function in those candidate files.
//   3. The sequence of useEffect / useLayoutEffect statements: the base order against the candidate.
// A statement's text includes the comments that lead it. "reindented" means identical once the
// leading whitespace of each line is dropped.
//
// What it does not compare: import statements, and whatever a module writes before its first
// statement that is not an import (a header comment above the imports). Section 5 prints that text
// for every new module so the report accounts for all of a new module's bytes; for App.tsx the
// import changes are read in the ordinary diff.

import { execFileSync } from "node:child_process";
import { readFileSync, readdirSync, statSync } from "node:fs";
import { createRequire } from "node:module";
import path from "node:path";

const require = createRequire(import.meta.url);
const ts = require("typescript");

const [baseRev, headRev] = process.argv.slice(2);
if (!baseRev) {
  console.error("usage: node b2_move_audit.mjs <base-rev> [<head-rev>]");
  process.exit(2);
}

const git = (...args) => execFileSync("git", args, { encoding: "utf8", maxBuffer: 1 << 28, stdio: ["ignore", "pipe", "ignore"] });
const repoRoot = git("rev-parse", "--show-toplevel").trim();
const DESKTOP = "projects/chirality-piping/apps/desktop";
const APP = `${DESKTOP}/src/App.tsx`;
const WORKSPACE_DIR = `${DESKTOP}/src/features/workspace`;

function showAt(rev, file) {
  try {
    return git("-C", repoRoot, "show", `${rev}:${file}`);
  } catch {
    return null;
  }
}

function listWorkspaceFiles(rev) {
  if (rev) {
    return git("-C", repoRoot, "ls-tree", "-r", "--name-only", rev, WORKSPACE_DIR)
      .split("\n")
      .filter(Boolean);
  }
  const out = [];
  const walk = (dir) => {
    for (const entry of readdirSync(path.join(repoRoot, dir))) {
      const rel = `${dir}/${entry}`;
      if (statSync(path.join(repoRoot, rel)).isDirectory()) walk(rel);
      else out.push(rel);
    }
  };
  walk(WORKSPACE_DIR);
  return out.sort();
}

const readHead = (file) => (headRev ? showAt(headRev, file) : readFileSync(path.join(repoRoot, file), "utf8"));

const baseAppText = showAt(baseRev, APP);
if (baseAppText === null) {
  console.error(`base App.tsx not found at ${baseRev}`);
  process.exit(2);
}

const headFiles = [APP];
for (const file of listWorkspaceFiles(headRev)) {
  if (!/\.tsx?$/.test(file) || /\.test\.tsx?$/.test(file)) continue;
  const before = showAt(baseRev, file);
  const after = readHead(file);
  if (after !== null && before !== after) headFiles.push(file);
}

const parse = (name, text) => ts.createSourceFile(name, text, ts.ScriptTarget.Latest, true, ts.ScriptKind.TSX);

function statementText(sf, stmt, stripExport) {
  const comments = sf.text.slice(stmt.getFullStart(), stmt.getStart(sf)).trim();
  let decl = stmt.getText(sf);
  if (stripExport) decl = decl.replace(/^export\s+(default\s+)?/, "");
  return comments ? `${comments}\n${decl}` : decl;
}

const dedent = (text) => text.split("\n").map((line) => line.replace(/^\s+/, "")).join("\n");

function statementName(stmt) {
  if (ts.isFunctionDeclaration(stmt) || ts.isClassDeclaration(stmt) || ts.isTypeAliasDeclaration(stmt) || ts.isInterfaceDeclaration(stmt)) {
    return stmt.name ? stmt.name.text : "(anonymous)";
  }
  if (ts.isVariableStatement(stmt)) {
    const names = [];
    const collect = (binding) => {
      if (ts.isIdentifier(binding)) names.push(binding.text);
      else for (const element of binding.elements) if (!ts.isOmittedExpression(element)) collect(element.name);
    };
    for (const declaration of stmt.declarationList.declarations) collect(declaration.name);
    return names.join(",");
  }
  return null;
}

function effectKind(stmt) {
  if (!ts.isExpressionStatement(stmt) || !ts.isCallExpression(stmt.expression)) return null;
  const callee = stmt.expression.expression;
  return ts.isIdentifier(callee) && (callee.text === "useEffect" || callee.text === "useLayoutEffect") ? callee.text : null;
}

function describe(sf, stmt) {
  const name = statementName(stmt);
  const line = sf.getLineAndCharacterOfPosition(stmt.getStart(sf)).line + 1;
  const kind = effectKind(stmt) ?? (ts.isVariableStatement(stmt) ? "VariableStatement" : ts.SyntaxKind[stmt.kind]);
  return `${kind}${name ? ` \`${name}\`` : ""} @${line}`;
}

function topLevelFunctions(sf) {
  return sf.statements.filter((stmt) => ts.isFunctionDeclaration(stmt) && stmt.body);
}

// ---- base ----
const baseSf = parse("base/App.tsx", baseAppText);
const baseSession = topLevelFunctions(baseSf).find((fn) => fn.name?.text === "AppSession");
if (!baseSession) {
  console.error("base AppSession not found");
  process.exit(2);
}
const baseTop = baseSf.statements.filter(
  (stmt) => !ts.isImportDeclaration(stmt) && stmt !== baseSession
);
const baseBody = [...baseSession.body.statements];

// ---- head ----
const headTop = []; // { file, stmt, sf, text, used }
const headBody = []; // { file, fn, stmt, sf, text, index, used }; filled after the top-level pass
const headSources = [];
for (const file of headFiles) {
  const text = readHead(file);
  if (text === null) continue;
  const sf = parse(file, text);
  headSources.push({ file, sf });
  for (const stmt of sf.statements) {
    if (ts.isImportDeclaration(stmt)) continue;
    headTop.push({ file, stmt, sf, text: statementText(sf, stmt, true), rawText: statementText(sf, stmt, false), used: false });
  }
}

// A candidate function that is itself a base top-level function (moved or kept whole) is audited as a
// whole in section 1; only the bodies of the other candidate functions are audited statement by statement.
function collectHeadBodies() {
  for (const entry of headTop) {
    if (entry.used || !ts.isFunctionDeclaration(entry.stmt) || !entry.stmt.body) continue;
    entry.bodyAudited = true;
    entry.stmt.body.statements.forEach((stmt, index) => {
      headBody.push({ file: entry.file, fn: entry.stmt.name?.text ?? "(anonymous)", stmt, sf: entry.sf, text: statementText(entry.sf, stmt, false), index, used: false });
    });
  }
}

const short = (file) => file.replace(`${DESKTOP}/`, "");
let problems = 0;
const out = [];
const say = (line = "") => out.push(line);

function unifiedDiff(a, b) {
  const left = a.split("\n");
  const right = b.split("\n");
  const rows = [];
  // Small LCS diff; statements are short enough for the quadratic table.
  const table = Array.from({ length: left.length + 1 }, () => new Uint32Array(right.length + 1));
  for (let i = left.length - 1; i >= 0; i -= 1) {
    for (let j = right.length - 1; j >= 0; j -= 1) {
      table[i][j] = left[i] === right[j] ? table[i + 1][j + 1] + 1 : Math.max(table[i + 1][j], table[i][j + 1]);
    }
  }
  // Every line is kept, then unchanged runs are trimmed to three lines of context.
  let i = 0;
  let j = 0;
  while (i < left.length && j < right.length) {
    if (left[i] === right[j]) { rows.push(`  ${left[i]}`); i += 1; j += 1; }
    else if (table[i + 1][j] >= table[i][j + 1]) { rows.push(`- ${left[i]}`); i += 1; }
    else { rows.push(`+ ${right[j]}`); j += 1; }
  }
  while (i < left.length) { rows.push(`- ${left[i]}`); i += 1; }
  while (j < right.length) { rows.push(`+ ${right[j]}`); j += 1; }
  const keep = rows.map(() => false);
  rows.forEach((row, index) => {
    if (row.startsWith("  ")) return;
    for (let near = Math.max(0, index - 3); near <= Math.min(rows.length - 1, index + 3); near += 1) keep[near] = true;
  });
  const trimmed = [];
  rows.forEach((row, index) => {
    if (keep[index]) trimmed.push(row);
    else if (trimmed.at(-1) !== "  ...") trimmed.push("  ...");
  });
  return trimmed;
}

function match(baseStmt, candidates, stripExport) {
  const text = statementText(baseSf, baseStmt, stripExport);
  let hit = candidates.find((candidate) => !candidate.used && candidate.text === text);
  if (hit) { hit.used = true; return { status: "identical", hit }; }
  hit = candidates.find((candidate) => !candidate.used && dedent(candidate.text) === dedent(text));
  if (hit) { hit.used = true; return { status: "reindented", hit }; }
  const name = statementName(baseStmt);
  const kind = baseStmt.kind;
  hit = name
    ? candidates.find((candidate) => !candidate.used && candidate.stmt.kind === kind && statementName(candidate.stmt) === name)
    : null;
  if (hit) { hit.used = true; return { status: "CHANGED", hit, diff: unifiedDiff(text, hit.text) }; }
  return { status: "MISSING", text };
}

say(`# B2 move audit`);
say();
say(`Base: \`${baseRev}\`. Candidate: ${headRev ? `\`${headRev}\`` : "the working tree"}.`);
say(`Candidate files compared: ${headFiles.map((file) => `\`${short(file)}\``).join(", ")}.`);
say();

say(`## 1. Top-level statements of the base App.tsx (${baseTop.length})`);
say();
say(`| Base statement | Result | Where |`);
say(`|---|---|---|`);
const topDiffs = [];
for (const stmt of baseTop) {
  const result = match(stmt, headTop, true);
  if (result.status === "CHANGED" || result.status === "MISSING") problems += 1;
  const where = result.hit ? `\`${short(result.hit.file)}\`` : "";
  say(`| ${describe(baseSf, stmt)} | ${result.status} | ${where} |`);
  if (result.status === "CHANGED") topDiffs.push({ title: describe(baseSf, stmt), diff: result.diff });
  if (result.status === "MISSING") topDiffs.push({ title: describe(baseSf, stmt), diff: result.text.split("\n").map((line) => `- ${line}`) });
}
say();

collectHeadBodies();

say(`## 2. Statements of the base AppSession body (${baseBody.length})`);
say();
say(`| # | Base statement | Result | Where |`);
say(`|---|---|---|---|`);
const bodyDiffs = [];
baseBody.forEach((stmt, index) => {
  const result = match(stmt, headBody, false);
  if (result.status === "CHANGED" || result.status === "MISSING") problems += 1;
  const where = result.hit ? `\`${short(result.hit.file)}\` in \`${result.hit.fn}\`` : "";
  say(`| ${index + 1} | ${describe(baseSf, stmt)} | ${result.status} | ${where} |`);
  if (result.status === "CHANGED") bodyDiffs.push({ title: describe(baseSf, stmt), diff: result.diff });
  if (result.status === "MISSING") bodyDiffs.push({ title: describe(baseSf, stmt), diff: result.text.split("\n").map((line) => `- ${line}`) });
});
say();

say(`## 3. Effect order`);
say();
const baseEffects = baseBody.filter((stmt) => effectKind(stmt)).map((stmt) => statementText(baseSf, stmt, false));
const effectsByFn = new Map();
for (const entry of headBody) {
  if (!effectKind(entry.stmt)) continue;
  const key = `${short(entry.file)} in ${entry.fn}`;
  if (!effectsByFn.has(key)) effectsByFn.set(key, []);
  effectsByFn.get(key).push(entry.text);
}
say(`Base AppSession declares ${baseEffects.length} effects. Candidate functions that declare effects: ${[...effectsByFn.keys()].map((key) => `\`${key}\` (${effectsByFn.get(key).length})`).join(", ") || "none"}.`);
say();
if (effectsByFn.size === 1) {
  const [sequence] = [...effectsByFn.values()];
  const same = sequence.length === baseEffects.length && sequence.every((text, index) => text === baseEffects[index]);
  if (!same) problems += 1;
  say(same
    ? `PASS: one candidate function declares all ${sequence.length} effects, in the base order, each byte for byte.`
    : `FAIL: the candidate's effect sequence differs from the base sequence (count ${sequence.length} against ${baseEffects.length}, or order, or text).`);
} else {
  problems += 1;
  say(`REVIEW: effects are declared in ${effectsByFn.size} candidate functions; their relative order depends on the order the hooks are called and is not proven by this tool.`);
}
say();

const newTop = headTop.filter((entry) => !entry.used && !entry.bodyAudited);
const auditedFns = headTop.filter((entry) => entry.bodyAudited);
const newBody = headBody.filter((entry) => !entry.used);
say(`## 4. Candidate statements with no base counterpart`);
say();
say(`These are the edits a reviewer reads: ${newTop.length} top-level, ${newBody.length} inside function bodies.`);
say(`Candidate functions audited statement by statement (their signatures are new or changed): ${auditedFns.map((entry) => `\`${entry.stmt.name?.text}\` in \`${short(entry.file)}\``).join(", ") || "none"}.`);
say();
for (const entry of newTop) {
  say(`### top level, \`${short(entry.file)}\`: ${describe(entry.sf, entry.stmt)}`);
  say();
  say("```ts");
  say(entry.rawText);
  say("```");
  say();
}
for (const entry of newBody) {
  say(`### \`${short(entry.file)}\` in \`${entry.fn}\`: ${describe(entry.sf, entry.stmt)}`);
  say();
  say("```ts");
  say(entry.text);
  say("```");
  say();
}

say(`## 5. Headers and imports of the new modules (not compared)`);
say();
for (const { file, sf } of headSources) {
  if (showAt(baseRev, file) !== null) continue;
  const firstCode = sf.statements.find((stmt) => !ts.isImportDeclaration(stmt));
  const lastImport = [...sf.statements].reverse().find((stmt) => ts.isImportDeclaration(stmt));
  const end = lastImport ? lastImport.getEnd() : firstCode ? firstCode.getFullStart() : sf.text.length;
  const head = sf.text.slice(0, end).trim();
  say(`### \`${short(file)}\``);
  say();
  say("```ts");
  say(head || "(no header and no import)");
  say("```");
  say();
}

if (topDiffs.length || bodyDiffs.length) {
  say(`## 6. Differences for CHANGED and MISSING statements`);
  say();
  for (const item of [...topDiffs, ...bodyDiffs]) {
    say(`### ${item.title}`);
    say();
    say("```diff");
    for (const row of item.diff) say(row);
    say("```");
    say();
  }
}

say(`Result: ${problems === 0 ? "PASS" : `${problems} item(s) to review`}.`);
process.stdout.write(`${out.join("\n")}\n`);
process.exit(problems === 0 ? 0 : 1);
