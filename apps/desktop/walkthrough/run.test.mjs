// The walkthrough command removes an earlier walkthrough's files from its
// output folder. These tests hold what it may remove, and that a count or a
// stopped journey reaches the index as recorded.
import assert from "node:assert/strict";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, rmSync, writeFileSync } from "node:fs";
import os from "node:os";
import path from "node:path";
import test from "node:test";
import { composeIndex } from "./compose-index.mjs";
import { emptyOutput, parseArguments } from "./run.mjs";

function scratch(t) {
  const folder = mkdtempSync(path.join(os.tmpdir(), "walkthrough-test-"));
  t.after(() => rmSync(folder, { recursive: true, force: true }));
  return folder;
}

test("an earlier walkthrough's own files are removed", (t) => {
  const out = scratch(t);
  for (const name of ["101-orientation-first-open.png", "702-rule-pack-new-draft-FAILED.png", "part-01-orientation.json", "INDEX.json", "index.html"]) writeFileSync(path.join(out, name), "x");
  emptyOutput(out);
  assert.deepEqual(readdirSync(out), []);
});

test("the native walkthrough's journeys (numbered from 20) stay when the browser walkthrough runs", (t) => {
  const out = scratch(t);
  for (const name of ["101-orientation-first-open.png", "part-01-orientation.json", "2001-solve-and-results-first-open.png", "part-20-solve-and-results.json", "INDEX.json"]) writeFileSync(path.join(out, name), "x");
  emptyOutput(out);
  assert.deepEqual(readdirSync(out).sort(), ["2001-solve-and-results-first-open.png", "part-20-solve-and-results.json"]);
});

test("a folder holding anything else is refused and nothing in it is removed", (t) => {
  for (const foreign of ["notes.txt", "101-orientation-first-open.png.bak", "Screenshot 1.png", "index.htm"]) {
    const out = scratch(t);
    writeFileSync(path.join(out, "101-orientation-first-open.png"), "x");
    writeFileSync(path.join(out, foreign), "keep");
    assert.throws(() => emptyOutput(out), /did not write/);
    assert.deepEqual(readdirSync(out).sort(), ["101-orientation-first-open.png", foreign].sort());
  }
});

test("a folder inside the output folder is refused, whatever its name", (t) => {
  const out = scratch(t);
  mkdirSync(path.join(out, "INDEX.json"));
  writeFileSync(path.join(out, "INDEX.json", "inner.txt"), "keep");
  assert.throws(() => emptyOutput(out), /did not write/);
  assert.ok(existsSync(path.join(out, "INDEX.json", "inner.txt")));
});

test("a missing output folder is created", (t) => {
  const out = path.join(scratch(t), "new", "folder");
  emptyOutput(out);
  assert.deepEqual(readdirSync(out), []);
});

test("arguments: unknown names, missing values and a malformed size are refused", () => {
  assert.throws(() => parseArguments(["--keep"]), /unknown argument/);
  assert.throws(() => parseArguments(["--out"]), /needs a value/);
  assert.throws(() => parseArguments(["--size", "wide"]), /WIDTHxHEIGHT/);
  assert.equal(parseArguments(["--size", "1280x800", "--grep", "loads"]).size, "1280x800");
});

test("the index carries each screen's counts, and names where a journey stopped", (t) => {
  const out = scratch(t);
  const counted = { words_on_screen: 12, words_in_page: 40, prose_blocks_on_screen: 1, buttons_on_screen: 3, inputs_on_screen: 2, longest_scroll_screens: 1, controls: ["button: Run <now>"] };
  writeFileSync(path.join(out, "part-01-first.json"), JSON.stringify({ number: 1, name: "first", title: "First & only", surface: "browser build", window: "1440x920", screens: [
    { number: 101, journey: "first", step: "open", note: "Opened", file: "101-first-open.png", ...counted },
    { number: 102, journey: "first", step: "solve", note: "Solve", file: "102-first-solve-FAILED.png", failed: "the control is disabled: No run yet" },
    { number: 103, journey: "first", step: "results", note: "Results", not_reached: true }
  ] }));
  const summary = composeIndex(out);
  assert.deepEqual(summary, { screens: 2, journeys: 1, stops: [{ journey: "first", step: "solve", reason: "the control is disabled: No run yet" }] });
  const index = JSON.parse(readFileSync(path.join(out, "INDEX.json"), "utf8"));
  assert.equal(index.screens[0].words_on_screen, 12);
  assert.deepEqual(index.journeys[0], { number: 1, name: "first", title: "First & only", surface: "browser build", window: "1440x920", screens: 1, stopped_at: "solve", not_reached: ["results"] });
  assert.equal(index.surface, "browser build");
  const html = readFileSync(path.join(out, "index.html"), "utf8");
  assert.match(html, /First &amp; only/);
  assert.match(html, /button: Run &lt;now&gt;/);
  assert.match(html, /A person stops here: the control is disabled: No run yet/);
});
