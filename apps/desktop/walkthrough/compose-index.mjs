// Joins the journeys' part files into INDEX.json (for agents and for
// comparing two walkthroughs) and index.html (for a person to read). The
// browser walkthrough (run.mjs) and the native one (native/run.mjs) write part
// files of the same form into one folder; each journey states its surface: the
// browser build, or the launched app redrawn by Chromium.
import { readFileSync, readdirSync, writeFileSync } from "node:fs";
import path from "node:path";

const escapeHtml = (text) => String(text).replace(/[&<>"]/g, (character) => ({ "&": "&amp;", "<": "&lt;", ">": "&gt;", '"': "&quot;" })[character]);

export function readParts(out) {
  return readdirSync(out)
    .filter((entry) => /^part-\d{2}-[a-z0-9-]+\.json$/.test(entry))
    .sort()
    .map((entry) => JSON.parse(readFileSync(path.join(out, entry), "utf8")));
}

// A part written before journeys stated their surface is a browser one.
const surfaceOf = (journey) => journey.surface ?? "browser build";

export function composeIndex(out, { size = null } = {}) {
  const journeys = readParts(out);
  const surfaces = [...new Set(journeys.map(surfaceOf))];
  const windows = [...new Set(journeys.map((journey) => journey.window ?? size).filter(Boolean))];
  // Every screen carries its journey's revision, build and use case (DEL-07-06 VER-007: the index records, for each
  // screen, the revision, the build, the use case, the step, the picture and the words and controls on screen).
  const screens = journeys.flatMap((journey) => journey.screens.map((screen) => ({
    ...screen,
    revision: journey.revision ?? null,
    build: journey.build ?? null,
    use_case: journey.use_case ?? journey.title,
    ...(screen.file && !screen.redrawn ? { redrawn: false } : {})
  })));
  // The native surfaces not pictured, each named once with what prevents it: where the journeys' launches differ, the
  // entry of the first launch that left its evidence missing, so a surface counts as supplied only when every launch
  // supplied it.
  const bySurface = new Map();
  for (const item of journeys.flatMap((journey) => journey.native_surfaces ?? [])) {
    const held = bySurface.get(item.surface);
    if (!held || (item.missing_evidence && !held.missing_evidence)) bySurface.set(item.surface, item);
  }
  const nativeSurfaces = [...bySurface.values()];
  // Each launch's reading of the menu the app built (native/run.mjs nativeMenuRecord).
  const nativeMenus = journeys.filter((journey) => journey.native_menu).map((journey) => ({ ...journey.native_menu, build: journey.build ?? null }));
  const stops = screens.filter((screen) => screen.failed).map((screen) => ({ journey: screen.journey, step: screen.step, reason: screen.failed }));
  const index = {
    schema: "swbpipe-walkthrough-index/v1",
    surface: surfaces.join("; ") || "none",
    window: windows.join(", ") || null,
    marks_explained: {
      redrawn: "true where the picture is the application's document redrawn outside its window by Chromium, not a capture of the window",
      substitutions: "each substitution of the scripted-run build the step used: the file a chooser returns, a native menu command, a request to quit or to close the main window, the report print dialog",
      native_surfaces: "each native surface not pictured, what prevents it (the permission prompt, or the substitution) and whether that leaves evidence missing",
      native_menus: "for each launch of the app, the menu it built, read from it once by the scripted-run build: each menu's items in order with label, command identifier, enabled and ticked state, and separators; names, order and state, not appearance or placement",
      window_shown: "on a native screen, whether the run brought the app's window to the front (--show-window) so its web view drew; page_visibility is what the page reported at that step"
    },
    native_surfaces: nativeSurfaces,
    native_menus: nativeMenus,
    counts_explained: {
      words_on_screen: "words a person can read without scrolling: in the window, not clipped, not covered",
      words_in_page: "words in the page's whole text, off-screen and scrolled-away text included",
      prose_blocks_on_screen: "paragraphs and list items of prose at least partly on screen",
      buttons_on_screen: "buttons, tabs, menu items, links and disclosures on screen",
      inputs_on_screen: "fields, selectors and checkboxes on screen",
      longest_scroll_screens: "how many window-heights the tallest scrolling region on screen runs to"
    },
    journeys: journeys.map((journey) => ({
      number: journey.number,
      name: journey.name,
      title: journey.title,
      use_case: journey.use_case ?? journey.title,
      revision: journey.revision ?? null,
      build: journey.build ?? null,
      surface: surfaceOf(journey),
      window: journey.window ?? size ?? null,
      ...(journey.picture_basis ? { picture_basis: journey.picture_basis } : {}),
      ...(journey.launch ? { launch: journey.launch } : {}),
      ...(journey.window_shown ? { window_shown: journey.window_shown } : {}),
      ...(journey.built_placements ? { built_placements: journey.built_placements } : {}),
      ...(journey.stored_runs ? { stored_runs: journey.stored_runs } : {}),
      screens: journey.screens.filter((screen) => screen.file && !screen.failed).length,
      stopped_at: journey.screens.find((screen) => screen.failed)?.step ?? null,
      not_reached: journey.screens.filter((screen) => screen.not_reached).map((screen) => screen.step)
    })),
    screens
  };
  writeFileSync(path.join(out, "INDEX.json"), `${JSON.stringify(index, null, 2)}\n`);
  writeFileSync(path.join(out, "index.html"), renderHtml(index));
  return { screens: screens.filter((screen) => screen.file).length, journeys: journeys.length, stops };
}

function renderScreen(screen) {
  if (screen.not_reached) {
    return `<li class="screen missing"><h3>${screen.number} · ${escapeHtml(screen.step)}</h3><p>${escapeHtml(screen.note)}</p><p class="stop">Not reached: the journey stopped earlier.</p></li>`;
  }
  const counts = screen.words_on_screen === undefined ? "" : `<dl>
    <div><dt>Words on screen</dt><dd>${screen.words_on_screen}</dd></div>
    <div><dt>Words in page</dt><dd>${screen.words_in_page}</dd></div>
    <div><dt>Prose blocks</dt><dd>${screen.prose_blocks_on_screen}</dd></div>
    <div><dt>Buttons</dt><dd>${screen.buttons_on_screen}</dd></div>
    <div><dt>Inputs</dt><dd>${screen.inputs_on_screen}</dd></div>
    <div><dt>Scroll length</dt><dd>${screen.longest_scroll_screens}×</dd></div>
  </dl>`;
  const controls = screen.controls ? `<details><summary>${screen.controls.length} controls on screen</summary><ul class="controls">${screen.controls.map((control) => `<li>${escapeHtml(control)}</li>`).join("")}</ul></details>` : "";
  const stop = screen.failed ? `<p class="stop">A person stops here: ${escapeHtml(screen.failed)}</p>` : "";
  const basis = screen.picture ? `<p class="basis">${escapeHtml(screen.picture)}${screen.view_3d === "not captured" ? " The 3D view was not captured at this step: a labelled placeholder stands in its place." : ""}</p>` : "";
  const substituted = screen.substitutions?.length ? `<p class="substituted">Substituted (scripted-run build): ${screen.substitutions.map((item) => `${escapeHtml(item.kind)}: ${escapeHtml(item.what)}`).join("; ")}.</p>` : "";
  const unpictured = screen.picture_error ? `<p class="stop">No picture: the redraw failed (${escapeHtml(screen.picture_error)}).</p>` : "";
  const picture = screen.file ? `<a href="${escapeHtml(screen.file)}"><img loading="lazy" src="${escapeHtml(screen.file)}" alt="${escapeHtml(screen.note)}"></a>` : "";
  return `<li class="screen${screen.failed ? " stopped" : ""}" id="s${screen.number}"><h3>${screen.number} · ${escapeHtml(screen.step)}</h3><p>${escapeHtml(screen.note)}</p>${stop}${substituted}${picture}${basis}${unpictured}${counts}${controls}</li>`;
}

// The menu the app built, for reading: one launch's, the others said to match it or listed apart.
function renderMenus(readings) {
  const entry = (item) => item.kind === "separator" ? `<li class="separator" aria-label="separator">—</li>`
    : item.kind === "submenu" ? `<li>${escapeHtml(item.label)}${item.enabled ? "" : " (disabled)"}<ul>${item.items.map(entry).join("")}</ul></li>`
    : `<li>${escapeHtml(item.label)}${item.command_id ? ` <code>${escapeHtml(item.command_id)}</code>` : " (system item)"}${item.enabled === false ? " (disabled)" : ""}${item.checked === true ? " ✓" : ""}</li>`;
  const read = readings.filter((reading) => reading.read);
  const failed = readings.filter((reading) => !reading.read).map((reading) => `<p class="stop">${escapeHtml(reading.journey)}: the menu was not read (${escapeHtml(reading.error)}).</p>`).join("");
  if (read.length === 0) return `<h2>The native menu bar, as the app built it</h2>${failed}`;
  const first = JSON.stringify(read[0].menus);
  const differs = read.filter((reading) => JSON.stringify(reading.menus) !== first).map((reading) => reading.journey);
  const same = read.filter((reading) => !differs.includes(reading.journey)).map((reading) => reading.journey);
  return `<h2>The native menu bar, as the app built it</h2><p class="basis">Read from the launched app once per launch (${escapeHtml(read[0].read_when ?? "once")}), through its menu API: names, order, command identifiers, enabled and ticked state. Not a picture: how the system draws and places the menu bar is not shown. As read in ${escapeHtml(same.join(", "))}${differs.length ? `; it differed in ${escapeHtml(differs.join(", "))} (see INDEX.json native_menus)` : ""}.</p>${failed}<div class="menus">${read[0].menus.map((menu) => `<div class="menu"><h3>${escapeHtml(menu.label)}${menu.enabled === false ? " (disabled)" : ""}</h3><ul>${(menu.items ?? []).map(entry).join("")}</ul></div>`).join("")}</div>`;
}

function renderHtml(index) {
  const sections = index.journeys.map((journey) => {
    const screens = index.screens.filter((screen) => screen.journey === journey.name);
    const stopped = journey.stopped_at ? `<p class="stop">Stopped at “${escapeHtml(journey.stopped_at)}”; ${journey.not_reached.length} later step${journey.not_reached.length === 1 ? "" : "s"} not reached.</p>` : "";
    const surface = journey.surface === "browser build"
      ? `<p class="surface">Surface: the browser build${journey.window ? `, at ${escapeHtml(journey.window)}` : ""}; pictures are screenshots of the browser window.</p>`
      : `<p class="surface">Surface: the ${escapeHtml(journey.surface)}${journey.window ? `, at ${escapeHtml(journey.window)}` : ""}. ${escapeHtml(journey.picture_basis ?? "")}</p>`;
    const revision = journey.revision?.commit ? `${journey.revision.commit.slice(0, 12)}${journey.revision.uncommitted_paths ? ` with ${journey.revision.uncommitted_paths} uncommitted path(s)` : ""}` : "not recorded";
    const build = journey.build ? (typeof journey.build === "string" ? journey.build : `${journey.build.kind}${journey.build.app_sha256 ? `, app ${journey.build.app_sha256.replace(/^sha256:/, "").slice(0, 12)}` : ""}`) : "not recorded";
    const facts = `<p class="surface">Use case: ${escapeHtml(journey.use_case)}. Revision: ${escapeHtml(revision)}. Build: ${escapeHtml(build)}.</p>`;
    const builds = (journey.built_placements ?? []).map((item) => `<p class="surface">Placed ${escapeHtml(item.key)}: built by ${escapeHtml(item.build)} from ${escapeHtml(item.from)} (SQL SHA-256 ${escapeHtml(item.sql_sha256)}; built file SHA-256 ${escapeHtml(item.sha256)}, ${item.bytes} bytes).</p>`).join("");
    const stored = journey.stored_runs
      ? `<p class="surface">Stored runs: ${journey.stored_runs.error ? `not read (${escapeHtml(journey.stored_runs.error)})` : `${journey.stored_runs.unchanged ? "unchanged" : "not shown unchanged"} at ${escapeHtml(Object.keys(journey.stored_runs.at).join(", "))}: ${escapeHtml(journey.stored_runs.what)}`}.</p>`
      : "";
    return `<section id="j${journey.number}"><h2>${journey.number}. ${escapeHtml(journey.title)}</h2>${facts}${builds}${stored}${surface}${stopped}<ol class="screens">${screens.map(renderScreen).join("")}</ol></section>`;
  }).join("");
  const rows = index.screens.filter((screen) => screen.words_on_screen !== undefined).map((screen) =>
    `<tr><td><a href="#s${screen.number}">${screen.number}</a></td><td>${escapeHtml(screen.journey)}</td><td>${escapeHtml(screen.step)}</td><td>${[screen.redrawn ? "redrawn" : "", screen.substitutions?.length ? "substituted" : "", screen.window_shown ? "window shown" : ""].filter(Boolean).join(", ")}</td><td class="n">${screen.words_on_screen}</td><td class="n">${screen.words_in_page}</td><td class="n">${screen.buttons_on_screen}</td><td class="n">${screen.inputs_on_screen}</td><td class="n">${screen.longest_scroll_screens}</td></tr>`).join("");
  return `<!doctype html>
<html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1">
<title>SWBPIPE walkthrough</title>
<style>
:root { color-scheme: light dark; --ink: #1c2126; --muted: #5d6873; --line: #d5dbe1; --paper: #f6f7f9; --card: #fff; --stop: #a3320b; }
@media (prefers-color-scheme: dark) { :root { --ink: #e6eaee; --muted: #9aa5b0; --line: #333b44; --paper: #14181c; --card: #1c2126; --stop: #ff9a6b; } }
body { margin: 0; padding: 24px 16px 64px; background: var(--paper); color: var(--ink); font: 15px/1.5 system-ui, sans-serif; }
main { max-width: 1180px; margin: 0 auto; }
h1 { font-size: 24px; margin: 0 0 4px; } h2 { font-size: 18px; margin: 40px 0 8px; } h3 { font-size: 14px; margin: 0; }
p { margin: 4px 0; } .lede { color: var(--muted); max-width: 70ch; }
nav ol { padding-left: 20px; }
.screens { list-style: none; padding: 0; display: grid; gap: 16px; grid-template-columns: repeat(auto-fill, minmax(340px, 1fr)); }
.screen { background: var(--card); border: 1px solid var(--line); border-radius: 6px; padding: 12px; min-width: 0; }
.screen img { display: block; width: 100%; height: auto; border: 1px solid var(--line); margin: 8px 0; }
.screen.stopped { border-color: var(--stop); } .stop { color: var(--stop); font-weight: 600; overflow-wrap: anywhere; }
.screen.missing { opacity: 0.7; }
.basis, .surface { color: var(--muted); font-size: 13px; }
.menus { display: grid; gap: 12px; grid-template-columns: repeat(auto-fill, minmax(220px, 1fr)); font-size: 13px; }
.menu { background: var(--card); border: 1px solid var(--line); border-radius: 6px; padding: 8px 12px; min-width: 0; overflow-wrap: anywhere; }
.menu ul { padding-left: 16px; margin: 4px 0; } .menu .separator { list-style: none; color: var(--muted); }
.substituted { font-size: 13px; border-left: 3px solid var(--stop); padding-left: 6px; }
dl { display: grid; grid-template-columns: repeat(3, 1fr); gap: 4px 12px; margin: 0; } dl div { min-width: 0; }
dt { color: var(--muted); font-size: 12px; } dd { margin: 0; font-variant-numeric: tabular-nums; font-weight: 600; }
details { margin-top: 8px; } summary { cursor: pointer; color: var(--muted); font-size: 13px; }
.controls { max-height: 260px; overflow: auto; padding-left: 18px; font-size: 12px; }
.table-wrap { overflow-x: auto; } table { border-collapse: collapse; font-size: 13px; width: 100%; }
th, td { border-bottom: 1px solid var(--line); padding: 3px 8px; text-align: left; } .n { text-align: right; font-variant-numeric: tabular-nums; }
</style></head><body><main>
<h1>SWBPIPE walkthrough</h1>
<p class="lede">Screens of the application (${escapeHtml(index.surface)}${index.window ? `, at ${escapeHtml(index.window)}` : ""}), taken by driving its own controls. Each screen lists what a person has in front of them. A journey that stops shows where a person would be stuck.${index.journeys.some((journey) => journey.surface !== "browser build") ? " Journeys of the launched app are not captures of its window: each picture is the launched app's document, recorded in its own web view, redrawn by Chromium, so fonts and form controls may differ in detail. Each journey states its surface." : ""}</p>
<nav><ol>${index.journeys.map((journey) => `<li><a href="#j${journey.number}">${escapeHtml(journey.title)}</a>${journey.stopped_at ? ` <span class="stop">(stopped at “${escapeHtml(journey.stopped_at)}”)</span>` : ""}</li>`).join("")}</ol></nav>
${index.native_surfaces.length ? `<h2>Native surfaces not pictured</h2><ul class="surfaces">${index.native_surfaces.map((item) => `<li><strong>${escapeHtml(item.surface)}</strong>: ${escapeHtml(item.prevented_by)}. Instead: ${escapeHtml(item.instead)}.${item.missing_evidence ? ' <span class="stop">Missing evidence.</span>' : item.evidence === "supplied" ? " Evidence supplied." : ""}</li>`).join("")}</ul>` : ""}
${index.native_menus.length ? renderMenus(index.native_menus) : ""}
<h2>Every screen, by the numbers</h2>
<div class="table-wrap"><table><thead><tr><th>No.</th><th>Journey</th><th>Step</th><th>Marks</th><th class="n">Words on screen</th><th class="n">Words in page</th><th class="n">Buttons</th><th class="n">Inputs</th><th class="n">Scroll</th></tr></thead><tbody>${rows}</tbody></table></div>
${sections}
</main></body></html>
`;
}
