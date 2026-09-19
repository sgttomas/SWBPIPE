#!/usr/bin/env node
// b2_order_check.mjs: the call-order companion of b2_move_audit.mjs for slice B2.
//
// The move audit proves that statements arrived byte for byte; it cannot see the order in which they
// now run. This tool shows that order. For `AppSession` and every `use…` hook in the candidate's
// `src/App.tsx` and non-test modules under `src/features/workspace/`, it lists the base `AppSession`
// statement number of each body statement, in candidate order ("new" for a statement with no base
// counterpart), and reports:
//   - whether the base numbers increase inside the function (base order kept);
//   - how many hook calls sit under a condition, a loop or a nested function (must be 0);
//   - for `AppSession`, whether any hook is called after the early return (must be false).
// Read with the list of which hook is called when, this is the whole render order of the session.
//
// Usage, from anywhere inside the worktree:
//   node <this file> <base-rev> [<head-rev>]
// With no <head-rev> the candidate is the working tree. Exit status 0 when every function passes.

import { execFileSync } from "node:child_process";
import { readFileSync, readdirSync } from "node:fs";
import { createRequire } from "node:module";
import path from "node:path";

const require = createRequire(import.meta.url);
const ts = require("typescript");

const [baseRev, headRev] = process.argv.slice(2);
if (!baseRev) {
  console.error("usage: node b2_order_check.mjs <base-rev> [<head-rev>]");
  process.exit(2);
}
const git = (...args) => execFileSync("git", args, { encoding: "utf8", maxBuffer: 1 << 28, stdio: ["ignore", "pipe", "ignore"] });
const repoRoot = git("rev-parse", "--show-toplevel").trim();
const DESKTOP = "projects/chirality-piping/apps/desktop";
const APP = `${DESKTOP}/src/App.tsx`;
const WORKSPACE_DIR = `${DESKTOP}/src/features/workspace`;

const parse = (name, text) => ts.createSourceFile(name, text, ts.ScriptTarget.Latest, true, ts.ScriptKind.TSX);
const statementText = (sf, stmt) => {
  const comments = sf.text.slice(stmt.getFullStart(), stmt.getStart(sf)).trim();
  return comments ? `${comments}\n${stmt.getText(sf)}` : stmt.getText(sf);
};

const baseSf = parse("base/App.tsx", git("-C", repoRoot, "show", `${baseRev}:${APP}`));
const baseSession = baseSf.statements.find((stmt) => ts.isFunctionDeclaration(stmt) && stmt.name?.text === "AppSession");
const baseNumber = new Map();
baseSession.body.statements.forEach((stmt, index) => baseNumber.set(statementText(baseSf, stmt), index + 1));

const files = headRev
  ? git("-C", repoRoot, "ls-tree", "-r", "--name-only", headRev, WORKSPACE_DIR).split("\n").filter(Boolean)
  : readdirSync(path.join(repoRoot, WORKSPACE_DIR)).map((entry) => `${WORKSPACE_DIR}/${entry}`);
const candidates = [APP, ...files.filter((file) => /\.tsx?$/.test(file) && !/\.test\.tsx?$/.test(file)).sort()];
const read = (file) => (headRev ? git("-C", repoRoot, "show", `${headRev}:${file}`) : readFileSync(path.join(repoRoot, file), "utf8"));

const isHookCall = (node) => ts.isCallExpression(node) && ts.isIdentifier(node.expression) && /^use[A-Z]/.test(node.expression.text);
const guards = (node) =>
  ts.isIfStatement(node) || ts.isConditionalExpression(node) || ts.isForStatement(node) || ts.isForOfStatement(node) ||
  ts.isForInStatement(node) || ts.isWhileStatement(node) || ts.isDoStatement(node) || ts.isSwitchStatement(node) ||
  ts.isTryStatement(node) || ts.isFunctionDeclaration(node) || ts.isArrowFunction(node) || ts.isFunctionExpression(node) ||
  (ts.isBinaryExpression(node) && [ts.SyntaxKind.AmpersandAmpersandToken, ts.SyntaxKind.BarBarToken, ts.SyntaxKind.QuestionQuestionToken].includes(node.operatorToken.kind));

let failures = 0;
console.log(`# B2 call-order check`);
console.log();
console.log(`Base: \`${baseRev}\`. Candidate: ${headRev ? `\`${headRev}\`` : "the working tree"}.`);
console.log();
for (const file of candidates) {
  const sf = parse(file, read(file));
  for (const fn of sf.statements) {
    if (!ts.isFunctionDeclaration(fn) || !fn.body) continue;
    const name = fn.name?.text ?? "";
    if (name !== "AppSession" && !/^use[A-Z]/.test(name)) continue;
    const statements = [...fn.body.statements];
    const numbers = statements.map((stmt) => baseNumber.get(statementText(sf, stmt)) ?? null);
    const known = numbers.filter((value) => value !== null);
    const increasing = known.every((value, index) => index === 0 || value > known[index - 1]);

    let conditionalHookCalls = 0;
    const visit = (node, guarded) => {
      if (guarded && isHookCall(node)) conditionalHookCalls += 1;
      const next = guarded || guards(node);
      ts.forEachChild(node, (child) => visit(child, next));
    };
    statements.forEach((stmt) => visit(stmt, false));

    // A top-level hook call placed after a statement that can return early.
    let sawEarlyReturn = false;
    let hookAfterEarlyReturn = false;
    for (const stmt of statements) {
      let direct = false;
      const direct_visit = (node) => {
        if (guards(node)) return;
        if (isHookCall(node)) direct = true;
        ts.forEachChild(node, direct_visit);
      };
      direct_visit(stmt);
      if (sawEarlyReturn && direct) hookAfterEarlyReturn = true;
      if (ts.isIfStatement(stmt) && /\breturn\b/.test(stmt.getText(sf))) sawEarlyReturn = true;
    }

    const ok = increasing && conditionalHookCalls === 0 && !hookAfterEarlyReturn;
    if (!ok) failures += 1;
    console.log(`## \`${name}\` in \`${file.replace(`${DESKTOP}/`, "")}\`: ${ok ? "PASS" : "FAIL"}`);
    console.log();
    console.log(`${statements.length} statements, ${numbers.filter((value) => value === null).length} new; base numbers ${increasing ? "increase" : "DO NOT increase"}; hook calls under a condition, loop or nested function: ${conditionalHookCalls}; hook call after an early return: ${hookAfterEarlyReturn}.`);
    console.log();
    console.log("`" + numbers.map((value) => value ?? "new").join(" ") + "`");
    console.log();
  }
}
console.log(`Result: ${failures === 0 ? "PASS" : `${failures} function(s) FAIL`}.`);
process.exit(failures === 0 ? 0 : 1);
