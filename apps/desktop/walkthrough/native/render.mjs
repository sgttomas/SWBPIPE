// The native walkthrough's pictures (README.md, "The launched app"): each step's snapshot of the launched app's
// document, rendered in Playwright's Chromium at the window's recorded inner size, after the app has ended and been
// censused. Chromium loads nothing: every request the snapshot would make is refused, so a picture holds only what
// the snapshot carried (its inlined stylesheets, its canvases' pixels, its pictures as data).
import { mkdirSync, readFileSync, readdirSync, writeFileSync } from "node:fs";
import path from "node:path";

export const NATIVE_SURFACE = "launched app, redrawn by Chromium";
export const PICTURE_BASIS = "This picture is the launched app's document, recorded in its own web view at this step and redrawn by Chromium; it is not a capture of the window. Fonts and form controls may differ in detail, and hover and focus rings are not shown.";

/** The step records the driver wrote beside `report` (`<report>.step-<number>.json`), in step order. */
export function readStepRecords(report) {
  const folder = path.dirname(report);
  const prefix = `${path.basename(report)}.step-`;
  return readdirSync(folder)
    .filter((name) => name.startsWith(prefix) && /^\d+\.json$/.test(name.slice(prefix.length)))
    .map((name) => JSON.parse(readFileSync(path.join(folder, name), "utf8")))
    .sort((a, b) => a.number - b.number);
}

const COUNT_KEYS = ["words_on_screen", "words_in_page", "prose_blocks_on_screen", "buttons_on_screen", "inputs_on_screen", "longest_scroll_screens", "controls"];

/**
 * The part file's screens from the step records, without pictures yet: what each step recorded, its counts, and
 * where its snapshot is kept. `launchFailure` (a string or null) is how the launch ended when it did not end
 * cleanly; when no step records a stop, it is recorded as the stop after the last step written.
 */
export function screensFromRecords(records, { journey, number, launchFailure = null }) {
  const screens = records.map((record) => {
    const screen = { number: record.number, journey, step: record.step, note: record.note };
    if (record.not_reached) return { ...screen, not_reached: true };
    if (record.failed) screen.failed = record.failed;
    for (const key of COUNT_KEYS) if (record[key] !== undefined) screen[key] = record[key];
    if (record.counts_error) screen.counts_error = record.counts_error;
    if (record.snapshot_error) screen.snapshot_error = record.snapshot_error;
    if (record.snapshot) {
      screen.window = `${record.snapshot.width}x${record.snapshot.height}`;
      if (record.snapshot.media) screen.media = record.snapshot.media;
      screen.view_3d = record.snapshot.view_3d;
      screen.canvases = record.snapshot.canvases;
      if (record.snapshot.notes?.length) screen.snapshot_notes = record.snapshot.notes;
    }
    return screen;
  });
  if (launchFailure && !screens.some((screen) => screen.failed)) {
    const last = screens.at(-1)?.number ?? number * 100;
    screens.push({ number: last + 1, journey, step: "launch-ended", note: "The launched app, after the last step it recorded", failed: `the launch did not end cleanly: ${launchFailure}` });
  }
  return screens;
}

/** The media preferences the web view answered, for Chromium to answer the same (light where none was recorded). */
export function mediaOf(snapshot) {
  const media = snapshot.media ?? {};
  return {
    colorScheme: media.color_scheme === "dark" ? "dark" : "light",
    reducedMotion: media.reduced_motion === "reduce" ? "reduce" : "no-preference",
    forcedColors: media.forced_colors === "active" ? "active" : "none"
  };
}

const fileName = (screen) => `${String(screen.number).padStart(3, "0")}-${screen.journey}-${screen.step}${screen.failed ? "-FAILED" : ""}.png`;

/**
 * Renders every recorded snapshot into `out` and returns the screens with their pictures. `snapshots` is where each
 * snapshot's HTML is kept (in the run's folder). `launchBrowser` is Playwright's `chromium.launch` (a seam).
 */
export async function renderScreens(records, screens, { out, snapshots, launchBrowser }) {
  mkdirSync(snapshots, { recursive: true });
  const byNumber = new Map(records.map((record) => [record.number, record]));
  const wanted = screens.filter((screen) => byNumber.get(screen.number)?.snapshot?.html);
  if (wanted.length === 0) return screens;
  const browser = await launchBrowser();
  try {
    for (const screen of wanted) {
      const snapshot = byNumber.get(screen.number).snapshot;
      const html = path.join(snapshots, `${screen.number}.html`);
      writeFileSync(html, snapshot.html);
      const context = await browser.newContext({
        viewport: { width: snapshot.width, height: snapshot.height }, deviceScaleFactor: 1,
        ...mediaOf(snapshot), serviceWorkers: "block", offline: true
      });
      try {
        // Nothing is loaded: the snapshot is self-contained, and anything it still names is refused.
        await context.route("**/*", (route) => (route.request().url().startsWith("data:") ? route.continue() : route.abort()));
        const page = await context.newPage();
        await page.setContent(snapshot.html, { waitUntil: "load", timeout: 60_000 });
        await page.evaluate(restoreDrawnState);
        await page.waitForTimeout(150);
        const file = fileName(screen);
        await page.screenshot({ path: path.join(out, file), animations: "disabled" });
        screen.file = file;
        screen.picture = PICTURE_BASIS;
      } catch (error) {
        screen.picture_error = String(error?.message ?? error).split("\n")[0];
      } finally {
        await context.close();
      }
    }
  } finally {
    await browser.close();
  }
  return screens;
}

// Run in Chromium on the redrawn document: the scroll positions, open modal dialogs and popovers, as recorded.
function restoreDrawnState() {
  for (const element of document.querySelectorAll("[data-walk-scroll]")) {
    const [top, left] = element.getAttribute("data-walk-scroll").split(",").map(Number);
    element.scrollTop = top;
    element.scrollLeft = left;
  }
  for (const dialog of document.querySelectorAll("dialog[data-walk-modal]")) {
    try { dialog.close(); dialog.showModal(); } catch { /* left as drawn */ }
  }
  for (const popover of document.querySelectorAll("[data-walk-popover-open]")) {
    try { popover.showPopover(); } catch { /* left as drawn */ }
  }
  for (const box of document.querySelectorAll("input[data-walk-indeterminate]")) box.indeterminate = true;
  const [top, left] = (document.documentElement.getAttribute("data-walk-window-scroll") ?? "0,0").split(",").map(Number);
  window.scrollTo(left, top);
}

/** Writes the journey's part file in the browser walkthrough's form, with its surface. */
export function writePart(out, { number, name, title, screens, launch }) {
  const windows = [...new Set(screens.map((screen) => screen.window).filter(Boolean))];
  const part = { number, name, title, surface: NATIVE_SURFACE, picture_basis: PICTURE_BASIS, window: windows.join(", ") || null, launch, screens };
  const file = path.join(out, `part-${String(number).padStart(2, "0")}-${name}.json`);
  writeFileSync(file, `${JSON.stringify(part, null, 2)}\n`);
  return file;
}
