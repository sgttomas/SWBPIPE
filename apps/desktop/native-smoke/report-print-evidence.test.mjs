// The report print's native evidence judge (`report-print-evidence.mjs`), on invented documents: a faithful record
// passes, and each way the record could disagree with the rendered document fails its own check.
//
//   node --test apps/desktop/native-smoke/report-print-evidence.test.mjs
import { test } from "node:test";
import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { NO_STANDING_LINE, derivedBanner, judgeReportPrint, standingLineOf } from "./report-print-evidence.mjs";

const sha256 = (text) => createHash("sha256").update(text, "utf8").digest("hex");
const LINE = "Numerical standing: L-1 sensitive &amp; conditional — invented";
const CANONICAL = `<!DOCTYPE html>\n<html lang="en">\n<head>\n<meta charset="utf-8">\n<title>Invented</title>\n</head>\n<body>\n<h1>Invented report</h1>\n<div class="standing-line">${LINE}</div>\n<p>Invented body</p>\n</body>\n</html>\n`;
const HASH = sha256(CANONICAL);
// As `derived_print_view` builds it: the banner directly after `<body>\n`.
const derive = (canonical, hash) => canonical.replace("<body>\n", `<body>\n${derivedBanner(hash)}`);
const PRINT = derive(CANONICAL, HASH);

function record(overrides = {}, event = {}) {
  const print_html = event.print_html ?? PRINT;
  return {
    status: "Print window opened with the derived print view of SHA-256 …; choose Print or Cancel in its dialog.",
    events: [{ print_sha256_hex: sha256(print_html), canonical_sha256_hex: HASH, served_to_print_window: true, window_label: "report-print-1", dialog_presented: false, ...event, print_html }],
    closed: true,
    ...overrides
  };
}
const OPTIONS = { canonicalHtml: CANONICAL, shownHash: `Canonical HTML SHA-256: ${HASH}`, expectStanding: "Numerical standing: L-1 sensitive" };
const STANDING = "the printed document's standing line is the canonical document's";
const NO_LINE = "neither the printed nor the canonical document has a standing line, as expected of this report";
const failed = (checks) => checks.filter((item) => !item.ok).map((item) => item.name.replace(/^print: /, ""));

test("a faithful record passes every check", () => {
  const checks = judgeReportPrint("print", record(), OPTIONS);
  assert.deepEqual(failed(checks), []);
  assert.equal(checks.length, 8);
  assert.equal(standingLineOf(PRINT), "Numerical standing: L-1 sensitive & conditional — invented");
});

test("the stated standing lead is checked against the printed line", () => {
  assert.deepEqual(failed(judgeReportPrint("print", record(), { ...OPTIONS, expectStanding: "Numerical standing: L-1 sensitive" })), []);
  assert.deepEqual(failed(judgeReportPrint("print", record(), { ...OPTIONS, expectStanding: "Numerical standing: L-1 passing" })), [STANDING]);
});

test("a caller that states no expectation fails, whatever the documents hold", () => {
  const { expectStanding: _unstated, ...unstated } = OPTIONS;
  for (const expectStanding of [undefined, null, "", "   ", { no_standing_line: true }])
    assert.deepEqual(failed(judgeReportPrint("print", record(), { ...unstated, expectStanding })), [STANDING], String(expectStanding));
  for (const expectStanding of [undefined, "   "]) {
    const check = judgeReportPrint("print", record(), { ...unstated, expectStanding }).find((item) => item.name === `print: ${STANDING}`);
    assert.match(check.detail.expectation, /not stated/, String(expectStanding));
  }
});

test("no record, or two, fails; a refused print fails", () => {
  assert.ok(failed(judgeReportPrint("print", record({ events: [] }), OPTIONS)).includes("exactly one print reached the native print path"));
  const two = record();
  two.events.push({ ...two.events[0], window_label: "report-print-2" });
  assert.ok(failed(judgeReportPrint("print", two, OPTIONS)).includes("exactly one print reached the native print path"));
  assert.ok(failed(judgeReportPrint("print", record({ status: "Not printed (REPORT-ROUTE-NOT-CURRENT): …" }), OPTIONS)).includes("Print reported its print window opened"));
  assert.ok(failed(judgeReportPrint("print", { error: "no element rendered-report-print" }, OPTIONS)).length >= 7);
  assert.ok(failed(judgeReportPrint("print", undefined, OPTIONS)).length >= 7);
});

test("a record not on a print window, not served to it, or with a dialog presented fails", () => {
  const name = "on a print window of its own, served to it by the print protocol, with the dialog substituted";
  for (const event of [{ window_label: "main" }, { window_label: "report-print-0" }, { served_to_print_window: false }, { dialog_presented: true }]) {
    assert.deepEqual(failed(judgeReportPrint("print", record({}, event), OPTIONS)), [name], JSON.stringify(event));
  }
});

test("a recorded canonical hash other than the preview's, or a shown hash other than it, fails", () => {
  const name = "its canonical SHA-256 is the rendered preview's and the one the panel shows";
  assert.deepEqual(failed(judgeReportPrint("print", record({}, { canonical_sha256_hex: "0".repeat(64) }), OPTIONS)), [name]);
  assert.deepEqual(failed(judgeReportPrint("print", record(), { ...OPTIONS, shownHash: `Canonical HTML SHA-256: ${"0".repeat(64)}` })), [name]);
  // A different preview: its hash is not the recorded one, and the document printed is not its derived view.
  const other = CANONICAL.replace("Invented body", "Other body");
  assert.deepEqual(failed(judgeReportPrint("print", record(), { ...OPTIONS, canonicalHtml: other, shownHash: `Canonical HTML SHA-256: ${sha256(other)}` })),
    [name, "the printed document is the canonical HTML with exactly the derived-view banner naming its hash"]);
});

test("a recorded print hash that does not recompute from the recorded document fails", () => {
  assert.deepEqual(failed(judgeReportPrint("print", record({}, { print_sha256_hex: "f".repeat(64) }), OPTIONS)),
    ["the recorded print document's SHA-256 recomputes to the recorded print hash"]);
  const missing = record();
  delete missing.events[0].print_html;
  assert.ok(failed(judgeReportPrint("print", missing, OPTIONS)).includes("the recorded print document's SHA-256 recomputes to the recorded print hash"));
});

test("a document other than the canonical HTML with exactly its banner fails, even when its own hash recomputes", () => {
  const name = "the printed document is the canonical HTML with exactly the derived-view banner naming its hash";
  // No banner; the banner twice; a banner naming another hash; an added element.
  for (const print_html of [CANONICAL, derive(PRINT, HASH), derive(CANONICAL, "0".repeat(64)), PRINT.replace("</body>", "<p>added</p>\n</body>")]) {
    assert.deepEqual(failed(judgeReportPrint("print", record({}, { print_html }), OPTIONS)), [name], print_html.slice(0, 120));
  }
});

test("a printed standing line other than the canonical one fails its own check", () => {
  const print_html = PRINT.replace(LINE, "Numerical standing: L-1 passing — invented");
  assert.deepEqual(failed(judgeReportPrint("print", record({}, { print_html }), OPTIONS)),
    ["the printed document is the canonical HTML with exactly the derived-view banner naming its hash", STANDING]);
  // A printed line that starts with the stated lead and still differs from the canonical line fails on the
  // comparison alone: the derived-view check fails with it, since the documents differ.
  const sameLead = PRINT.replace(LINE, "Numerical standing: L-1 sensitive &amp; conditional — altered");
  assert.ok(standingLineOf(sameLead).startsWith(OPTIONS.expectStanding));
  assert.deepEqual(failed(judgeReportPrint("print", record({}, { print_html: sameLead }), OPTIONS)),
    ["the printed document is the canonical HTML with exactly the derived-view banner naming its hash", STANDING]);
  // Two standing lines in the printed document are not one line.
  const doubled = PRINT.replace("<p>Invented body</p>", `<div class="standing-line">${LINE}</div>`);
  assert.ok(failed(judgeReportPrint("print", record({}, { print_html: doubled }), OPTIONS)).includes(STANDING));
});

test("a report expected to have no standing line passes only when both documents were read and neither has one", () => {
  const canonical = CANONICAL.replace(`<div class="standing-line">${LINE}</div>\n`, "");
  const hash = sha256(canonical);
  const options = { canonicalHtml: canonical, shownHash: `Canonical HTML SHA-256: ${hash}` };
  const faithful = record({}, { print_html: derive(canonical, hash), canonical_sha256_hex: hash });
  const none = judgeReportPrint("print", faithful, { ...options, expectStanding: NO_STANDING_LINE });
  assert.deepEqual(failed(none), []);
  assert.equal(none.length, 8);
  // The same documents with nothing stated, or with a lead stated, fail: three nulls are not a comparison.
  assert.deepEqual(failed(judgeReportPrint("print", faithful, options)), [STANDING]);
  assert.deepEqual(failed(judgeReportPrint("print", faithful, { ...options, expectStanding: "Numerical standing" })), [STANDING]);
  // A document that has a line fails the expectation of none: on both sides, on the printed side only, and on the
  // canonical side only.
  assert.deepEqual(failed(judgeReportPrint("print", record(), { ...OPTIONS, expectStanding: NO_STANDING_LINE })), [NO_LINE]);
  assert.ok(failed(judgeReportPrint("print", record({}, { print_html: PRINT, canonical_sha256_hex: hash }), { ...options, expectStanding: NO_STANDING_LINE })).includes(NO_LINE));
  assert.ok(failed(judgeReportPrint("print", faithful, { ...OPTIONS, expectStanding: NO_STANDING_LINE })).includes(NO_LINE));
  // Two lines on the canonical side are not "no line".
  const twice = CANONICAL.replace("<p>Invented body</p>", `<div class="standing-line">${LINE}</div>`);
  assert.ok(failed(judgeReportPrint("print", faithful, { canonicalHtml: twice, shownHash: `Canonical HTML SHA-256: ${sha256(twice)}`, expectStanding: NO_STANDING_LINE })).includes(NO_LINE));
  // Either document unread fails it: no print recorded, or no canonical document given.
  assert.ok(failed(judgeReportPrint("print", record({ events: [] }), { ...options, expectStanding: NO_STANDING_LINE })).includes(NO_LINE));
  assert.ok(failed(judgeReportPrint("print", faithful, { shownHash: options.shownHash, expectStanding: NO_STANDING_LINE })).includes(NO_LINE));
  // An empty document, on either side, was not read.
  const empty = record({}, { print_html: "", canonical_sha256_hex: hash });
  assert.ok(failed(judgeReportPrint("print", empty, { ...options, expectStanding: NO_STANDING_LINE })).includes(NO_LINE));
  assert.ok(failed(judgeReportPrint("print", faithful, { canonicalHtml: "", shownHash: options.shownHash, expectStanding: NO_STANDING_LINE })).includes(NO_LINE));
});

test("a print window left open fails", () => {
  assert.deepEqual(failed(judgeReportPrint("print", record({ closed: false }), OPTIONS)), ["the close confirmed that the print window it selected left the window registry (not matched to the recorded print's window)"]);
});
