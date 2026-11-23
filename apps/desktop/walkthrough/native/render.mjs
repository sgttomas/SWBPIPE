// The native walkthrough's pictures (README.md, "The launched app"): each step's snapshot of the launched app's
// document, rendered in Playwright's Chromium at the window's recorded inner size, after the app has ended and been
// censused. Chromium loads nothing: every request the snapshot would make is refused, so a picture holds only what
// the snapshot carried (its inlined stylesheets, its canvases' pixels, its pictures as data).
import { mkdirSync, readFileSync, readdirSync, writeFileSync } from "node:fs";
import path from "node:path";

export const NATIVE_SURFACE = "launched app, redrawn by Chromium";
export const MENU_SURFACE = "the native menu bar: its menus, their items' names and order";
export const VIEW_3D_SURFACE = "the 3D view's drawing, where a step says it was not captured";
export const PICTURE_BASIS = "This picture is the launched app's document, recorded in its own web view at this step and redrawn by Chromium; it is not a capture of the window. Fonts and form controls may differ in detail, and hover and focus rings are not shown.";

/**
 * The native surfaces an attended capture may declare it shows (its sidecar's `surfaces`) or state it could not show
 * (`prevented`): the fixed set, each with its key and the name the index gives it.
 */
export const SURFACE_KEYS = Object.freeze({
  window: "the application window itself (its frame, title bar, and how the document is drawn in it)",
  menu_bar: MENU_SURFACE,
  file_chooser: "the file chooser (open and save panels)",
  print_dialog: "the system print dialog",
  view_3d: VIEW_3D_SURFACE,
  other_native_dialog: "another native dialog (an alert, a sheet or a panel the system or the app raises)"
});
/** The four surfaces only an attended capture can supply; without one each is missing evidence. */
export const ATTENDED_ONLY_SURFACES = Object.freeze(["window", "menu_bar", "file_chooser", "print_dialog"]);
export const NOT_SUPPLIED = "not supplied: no attended capture of this surface in this walkthrough";

/**
 * The native surfaces a person meets, each with what the scripted runner does instead and, until an attended capture
 * in the same output folder supplies it (compose-index.mjs), missing evidence (DEL-07-06 VER-007: an uncaptured
 * surface without a record of what prevented its capture is missing evidence). The scripted runner itself takes no
 * capture of the window, pictures no menu bar and presses no Print. The 3D view's entry depends on the launch
 * (nativeSurfacesFor); this is its form when the launch supplied nothing for it.
 */
export const NATIVE_SURFACES = Object.freeze([
  {
    key: "window",
    surface: SURFACE_KEYS.window,
    captured: false,
    prevented_by: NOT_SUPPLIED,
    instead: "the scripted runner takes no capture of the window: each of its pictures is the app's document recorded in its own web view and redrawn by Chromium",
    evidence: "missing",
    missing_evidence: true
  },
  {
    key: "menu_bar",
    surface: MENU_SURFACE,
    captured: false,
    prevented_by: NOT_SUPPLIED,
    instead: "the scripted runner pictures no menu bar: menu commands are delivered through the menu's own dispatch and marked on each step that uses one; this launch did not read the menu the app built",
    evidence: "missing",
    missing_evidence: true
  },
  {
    key: "file_chooser",
    surface: SURFACE_KEYS.file_chooser,
    captured: false,
    prevented_by: NOT_SUPPLIED,
    instead: "substituted in the scripted-run build: the harness names the file the next chooser returns, no panel is shown, and each step that uses it is marked",
    evidence: "missing",
    missing_evidence: true
  },
  {
    key: "print_dialog",
    surface: SURFACE_KEYS.print_dialog,
    captured: false,
    prevented_by: NOT_SUPPLIED,
    instead: "the scripted runner presses no Print; in the scripted-run build the print document is recorded in place of the dialog",
    evidence: "missing",
    missing_evidence: true
  },
  {
    key: "view_3d",
    surface: VIEW_3D_SURFACE,
    captured: false,
    prevented_by: "the window was not brought to the front (the run was not given --show-window): the web view reports its page hidden and gives no animation frame, so the viewport draws nothing",
    instead: "a labelled placeholder in the picture, and the step's view_3d and snapshot_notes",
    evidence: "missing",
    missing_evidence: true
  }
]);

/**
 * The native surfaces for one launch: the menu bar's entry says what the launch read of the menu the app built
 * (`menu`, run.mjs nativeMenuRecord), which is a reading, not the menu bar as drawn, so the entry stays missing
 * evidence; and the 3D view's entry says captured when every step that had the 3D view on screen read a drawn frame of
 * it; with the window shown (`windowShown`) and a step not captured, it says at how many steps and why not.
 */
export function nativeSurfacesFor({ menu = null, windowShown = false, screens = [] } = {}) {
  return NATIVE_SURFACES.map((item) => {
    if (item.key === "menu_bar" && menu?.read) {
      return {
        ...item,
        instead: `the scripted runner pictures no menu bar; the menu the app built was read from the app by the scripted-run build's native_smoke_menu_items once in this launch (${menu.read_when ?? "once"}) and recorded in the index's native_menus: each menu and its items in order, with their labels, command identifiers, enabled and ticked state, and separators. It establishes the names, order and enabled state as built; not how the system draws the menu bar, where it places it, or the application menu's title as the system shows it`,
        reading: "supplied"
      };
    }
    if (item.key === "menu_bar" && menu && !menu.read) return { ...item, instead: `${item.instead}: ${menu.error}` };
    if (item.key === "view_3d") {
      const on = screens.filter((screen) => screen.view_3d === "captured" || screen.view_3d === "not captured");
      const blank = on.filter((screen) => screen.view_3d === "not captured");
      const how = windowShown ? "the window was brought to the front for the run (--show-window)" : "the window was not brought to the front (the run was not given --show-window)";
      if (on.length > 0 && blank.length === 0) {
        return { ...item, captured: true, prevented_by: `nothing: ${how}, and the 3D view's pixels were read from its canvas in the web view at every step that showed it`, instead: "the 3D view's own pixels in each picture", evidence: "supplied", missing_evidence: false };
      }
      if (!windowShown) return item;
      const reasons = [...new Set(blank.flatMap((screen) => (screen.canvases ?? []).filter((canvas) => canvas.label === "3D view" && !canvas.captured).map((canvas) => canvas.reason)))];
      return {
        ...item,
        prevented_by: on.length === 0
          ? `${how}, but no step showed the 3D view`
          : `${how}, but at ${blank.length} of ${on.length} steps that showed the 3D view it was not captured: ${reasons.join("; ") || "no reason recorded"}`
      };
    }
    return item;
  });
}

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
    if (record.substitutions?.length) screen.substitutions = record.substitutions;
    if (record.acted_on?.length) screen.acted_on = record.acted_on;
    if (record.window_shown !== undefined) screen.window_shown = record.window_shown;
    if (record.page_visibility !== undefined) screen.page_visibility = record.page_visibility;
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
        screen.redrawn = true;
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

/**
 * Writes the journey's part file in the browser walkthrough's form, with its surface, the launch's reading of the
 * menu the app built (`native_menu`), whether its window was shown (`window_shown`, run.mjs windowShownRecord), and,
 * where the journey has them, the stores it built at placement and its stored-run evidence.
 */
export function writePart(out, { number, name, title, use_case, revision, build, screens, launch, native_menu = null, window_shown = null, built_placements, stored_runs }) {
  const windows = [...new Set(screens.map((screen) => screen.window).filter(Boolean))];
  const part = {
    number, name, title, use_case, revision, build, surface: NATIVE_SURFACE, picture_basis: PICTURE_BASIS,
    window: windows.join(", ") || null,
    native_surfaces: nativeSurfacesFor({ menu: native_menu, windowShown: window_shown?.shown === true, screens }),
    ...(native_menu ? { native_menu } : {}),
    ...(window_shown ? { window_shown } : {}),
    launch,
    ...(built_placements ? { built_placements } : {}), ...(stored_runs ? { stored_runs } : {}), screens
  };
  const file = path.join(out, `part-${String(number).padStart(2, "0")}-${name}.json`);
  writeFileSync(file, `${JSON.stringify(part, null, 2)}\n`);
  return file;
}
