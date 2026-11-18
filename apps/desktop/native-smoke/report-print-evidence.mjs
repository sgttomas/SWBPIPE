// The report print's native evidence, judged from the bytes the native-smoke build recorded (DEL-08-01 REQ-022,
// AC-011; DEL-07-11's scripted-run build). Shared by `run.mjs` and `report-standing-journey.mjs`.
//
// In the native-smoke build the report print presents no system dialog: on the print window's own handle, at the
// point of the native print, it records the print document (`native_smoke::record_report_print`). A driver presses
// Print, takes that record (`native_smoke_take_report_prints`) and closes the window
// (`native_smoke_close_report_print`). This judge recomputes everything it claims from the recorded document and the
// rendered canonical HTML, never from a text the driver derived:
//   - exactly one print reached the native print path, on a print window, served to it by the print protocol;
//   - its canonical SHA-256 is the rendered preview's and the one the panel shows;
//   - the recorded document's SHA-256 recomputes to the recorded print hash;
//   - the document is the canonical HTML with exactly the derived-view banner naming that hash, and nothing else;
//   - the standing line read from the printed document is the canonical document's (or both have none).
// It shows which document reached the native print path. It does not show that a dialog appeared, what the system's
// print preview composed, or how a print job ended; no such claim is made.
import { createHash } from "node:crypto";

export const PRINT_WINDOW_LABEL = /^report-print-[1-9][0-9]*$/;
const sha256 = (text) => createHash("sha256").update(text, "utf8").digest("hex");

// The renderer's derived-view banner (`report_renderer` `derived_print_view`), restated here as the judge's oracle.
export const derivedBanner = (canonicalSha256) =>
  `<div class="derived-banner" data-derived-view="true">DERIVED VIEW — not hash-bound evidence. Canonical rendered-report artifact SHA-256: ${canonicalSha256}</div>\n`;

const ENTITIES = { "&amp;": "&", "&lt;": "<", "&gt;": ">", "&quot;": "\"", "&#39;": "'" };
// The standing line as the renderer writes it (`<div class="standing-line">…</div>`, escaped text), or null.
export function standingLineOf(html) {
  const lines = [...String(html).matchAll(/<div class="standing-line">([^<]*)<\/div>/g)];
  if (lines.length !== 1) return lines.length === 0 ? null : { ambiguous: lines.length };
  return lines[0][1].replace(/&(?:amp|lt|gt|quot|#39);/g, (entity) => ENTITIES[entity]).trim();
}

/** The checks, each `{ name, ok, detail }`. `observed` is the driver's record of one Print press:
 * `{ status, events, closed }`. `canonicalHtml` is the rendered preview's document; `shownHash` the panel's
 * "Canonical HTML SHA-256: …" line. `expectStanding`, when given, is checked against the printed line's start. */
export function judgeReportPrint(label, observed, { canonicalHtml, shownHash, expectStanding } = {}) {
  const checks = [];
  const check = (name, ok, detail) => checks.push({ name: `${label}: ${name}`, ok: Boolean(ok), detail });
  const events = Array.isArray(observed?.events) ? observed.events : [];
  const event = events.length === 1 ? events[0] : null;
  check("Print reported its print window opened", /^Print window opened/.test(observed?.status ?? ""), observed?.status ?? observed?.error ?? null);
  check("exactly one print reached the native print path", events.length === 1, { events: events.length });
  check("on a print window of its own, served to it by the print protocol, with the dialog substituted",
    event && PRINT_WINDOW_LABEL.test(event.window_label ?? "") && event.served_to_print_window === true && event.dialog_presented === false,
    event && { window_label: event.window_label, served_to_print_window: event.served_to_print_window, dialog_presented: event.dialog_presented });
  const canonical = typeof canonicalHtml === "string" && canonicalHtml.length > 0 ? sha256(canonicalHtml) : null;
  check("its canonical SHA-256 is the rendered preview's and the one the panel shows",
    canonical !== null && event?.canonical_sha256_hex === canonical && shownHash === `Canonical HTML SHA-256: ${canonical}`,
    { recorded: event?.canonical_sha256_hex ?? null, preview: canonical, shown: shownHash ?? null });
  const printHtml = typeof event?.print_html === "string" ? event.print_html : null;
  const printed = printHtml === null ? null : sha256(printHtml);
  check("the recorded print document's SHA-256 recomputes to the recorded print hash",
    printed !== null && printed === event.print_sha256_hex, { recorded: event?.print_sha256_hex ?? null, recomputed: printed });
  const banner = canonical === null ? null : derivedBanner(canonical);
  const derivedOk = printHtml !== null && banner !== null && printHtml.split(banner).length === 2 && printHtml.replace(banner, "") === canonicalHtml;
  check("the printed document is the canonical HTML with exactly the derived-view banner naming its hash", derivedOk,
    printHtml === null ? "no print document recorded" : { banners: banner ? printHtml.split(banner).length - 1 : null, bytes: printHtml.length });
  const printedLine = printHtml === null ? undefined : standingLineOf(printHtml);
  const canonicalLine = canonical === null ? undefined : standingLineOf(canonicalHtml);
  const lineIsText = (line) => line === null || typeof line === "string";
  check("the printed document's standing line is the canonical document's",
    printedLine !== undefined && lineIsText(printedLine) && lineIsText(canonicalLine) && printedLine === canonicalLine
      && (expectStanding === undefined || (typeof printedLine === "string" && printedLine.startsWith(expectStanding))),
    { printed: printedLine ?? null, canonical: canonicalLine ?? null, expected_lead: expectStanding ?? null });
  check("the print window was closed afterwards", observed?.closed === true, observed?.closed ?? null);
  return checks;
}
