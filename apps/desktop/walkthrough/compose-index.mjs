// Joins the journeys' part files into INDEX.json (for agents and for
// comparing two walkthroughs) and index.html (for a person to read). The
// browser walkthrough (run.mjs) and the native one (native/run.mjs) write part
// files of the same form into one folder; each journey states its surface: the
// browser build, or the launched app redrawn by Chromium.
import { createHash } from "node:crypto";
import { lstatSync, readFileSync, readdirSync, writeFileSync } from "node:fs";
import path from "node:path";
import { ATTENDED_ONLY_SURFACES, NATIVE_SURFACES, NOT_SUPPLIED, SURFACE_KEYS } from "./native/render.mjs";
import { readSidecar } from "./native/sidecar.mjs";

// The surface of an attended launch's part (native/run.mjs --attended): its screens are captures of the actual window.
export const ATTENDED_SURFACE = "launched app, its actual window driven by an attendant";
const attendedJourney = (journey) => journey.surface === ATTENDED_SURFACE;

const escapeHtml = (text) => String(text).replace(/[&<>"]/g, (character) => ({ "&": "&amp;", "<": "&lt;", ">": "&gt;", '"': "&quot;" })[character]);

export function readParts(out) {
  return readdirSync(out)
    .filter((entry) => /^part-\d{2}-[a-z0-9-]+\.json$/.test(entry))
    .sort()
    .map((entry) => JSON.parse(readFileSync(path.join(out, entry), "utf8")));
}

// A part written before journeys stated their surface is a browser one.
const surfaceOf = (journey) => journey.surface ?? "browser build";

const SURFACE_ORDER = Object.keys(SURFACE_KEYS);
const sha256File = (file) => {
  try {
    const stat = lstatSync(file);
    return stat.isFile() ? `sha256:${createHash("sha256").update(readFileSync(file)).digest("hex")}` : null;
  } catch {
    return null;
  }
};
const sidecarOf = (file) => file.replace(/\.png$/i, ".json");

/**
 * Whether each attended capture, and each sidecar recorded without a picture, is still the one its launch recorded
 * (native/run.mjs readCaptures `binding`, a record's `sidecar_sha256`): a capture's binding names its part's revision
 * (commit and driver) and build, and the picture and sidecar on disk have the recorded SHA-256. Each sidecar is read
 * once (native/sidecar.mjs readSidecar), and when its bytes have the recorded hash its claims are parsed from those
 * same bytes: what a capture or a record is counted for is what it says now and said when its launch ended, never the
 * fields the part saved. Returns { checks: Map file -> null (intact) | why, claims: Map file -> the verified claims,
 * unbound: [{ file, why }] }, where `unbound` lists every file in the folder's captures/ that no attended launch
 * recorded: added after its launch ended, it supplies nothing.
 */
export function attendedBindings(out, journeys) {
  const checks = new Map();
  const claims = new Map();
  const recorded = new Set();
  // A sidecar on disk against its recorded hash: { why } or { claims } parsed from the bytes that were hashed.
  const verify = (file, sha256, what) => {
    if (!out) return { why: `${what} was not read against its recorded hash` };
    const read = readSidecar(path.join(out, file));
    if (!read.sidecar_sha256) return { why: `${what} was removed after its launch ended (${read.why})` };
    if (read.sidecar_sha256 !== sha256) return { why: `${what} on disk is not the one its launch recorded: it was edited after the launch ended` };
    if (read.why) return { why: `${what} says nothing usable (${read.why})` };
    return { claims: read };
  };
  for (const part of journeys.filter(attendedJourney)) {
    for (const screen of part.screens ?? []) {
      recorded.add(screen.file);
      recorded.add(sidecarOf(screen.file));
      if (screen.capture !== "the actual window") continue;
      const binding = screen.binding;
      let why = null;
      if (!binding) why = "it is not bound to its launch: no hashes were recorded when the launch ended";
      else if (binding.revision?.commit !== part.revision?.commit || (binding.revision?.driver_sha256 ?? null) !== (part.revision?.driver_sha256 ?? null) || binding.build_app_sha256 !== (part.build?.app_sha256 ?? null)) why = "its binding does not name its launch's revision, driver and build";
      else if (out) {
        const picture = sha256File(path.join(out, screen.file));
        if (picture === null) why = "the picture was removed after its launch ended";
        else if (picture !== binding.picture_sha256) why = "the picture on disk is not the one its launch recorded: it was replaced after the launch ended";
        else {
          const sidecar = verify(sidecarOf(screen.file), binding.sidecar_sha256, "its sidecar");
          if (sidecar.why) why = sidecar.why;
          else claims.set(screen.file, sidecar.claims);
        }
      } else why = "it was not read against its recorded hashes";
      checks.set(screen.file, why);
    }
    for (const record of part.captures?.records_without_picture ?? []) {
      recorded.add(record.file);
      const sidecar = record.sidecar_sha256 ? verify(record.file, record.sidecar_sha256, "this sidecar") : { why: "this sidecar is not bound to its launch: no hash was recorded when the launch ended" };
      if (sidecar.why) checks.set(record.file, sidecar.why);
      else { checks.set(record.file, null); claims.set(record.file, sidecar.claims); }
    }
    for (const item of part.captures?.other ?? []) recorded.add(item.file);
  }
  const unbound = [];
  const folder = out ? path.join(out, "captures") : null;
  const stat = folder ? (() => { try { return lstatSync(folder); } catch { return null; } })() : null;
  if (stat?.isDirectory() && !stat.isSymbolicLink() && journeys.some(attendedJourney)) {
    for (const name of readdirSync(folder).sort()) {
      const file = `captures/${name}`;
      if (!recorded.has(file)) unbound.push({ file, why: "added to captures/ after its launch ended: bound to no launch, it supplies nothing" });
    }
  }
  return { checks, claims, unbound };
}
/** The surfaces the scripted-run build substitutes: by DEL-07-06 VER-007 each is supplied or missing, never unseen. */
export const SUBSTITUTED_BY_BUILD = Object.freeze({
  file_chooser: "the scripted-run build substitutes the file chooser (a queued answer, or the strict chooser's end of the application): by DEL-07-06 VER-007 it is supplied or missing, never unseen",
  print_dialog: "the scripted-run build substitutes the print dialog (the print document is recorded and no dialog is shown): by DEL-07-06 VER-007 it is supplied or missing, never unseen"
});
/** An attendant's complete statement that a surface could not be shown, in words. */
const statementOf = (record) => `at the step "${record.step}", ${SURFACE_KEYS[record.surface] ?? record.surface} was attempted and "${record.observed}" was observed; the obstacle is the ${record.obstacle === "system" ? "system's" : "application's"}: ${record.cause}`;
const keyOf = (item) => item.key ?? SURFACE_ORDER.find((key) => SURFACE_KEYS[key] === item.surface) ?? null;
const shortCommit = (commit) => (typeof commit === "string" ? commit.slice(0, 12) : "unrecorded");

/**
 * The index's native surfaces. Each launch's parts name the surfaces it met (render.mjs nativeSurfacesFor); where the
 * launches differ, the first entry that leaves evidence missing is kept. Then the attended launches' parts in the same
 * folder decide each surface of the fixed set (render.mjs SURFACE_KEYS):
 * - supplied, when an attended capture of the actual window declares it in its sidecar's `surfaces`, its launch is at
 *   the revision AND build of the walkthrough's screens, and the capture is still the one its launch recorded
 *   (attendedBindings); the entry lists those captures with their revision and build. A capture at another revision
 *   or build, one replaced, removed or edited since, or a sidecar with no picture, supplies nothing and is listed in
 *   `not_supplying` with why;
 * - otherwise unseen, as the attendant states, when a capture's sidecar or a sidecar with no picture, at that revision
 *   and build, holds a statement (`prevented`) naming the step attempted, the surface, what was observed and whether
 *   the obstacle is the system's or the application's; the index gives the statement in full, for the reader to
 *   accept or not. A statement lacking any of the four makes nothing unseen and is listed in `not_supplying`. The file
 *   chooser and the print dialog are substituted by the scripted-run build, so they are supplied or missing, never
 *   unseen, whatever a sidecar says (SUBSTITUTED_BY_BUILD). The runner writes no statement of its own;
 * - otherwise, for the window, the menu bar, the file chooser and the print dialog (and another native dialog once
 *   anything names it), missing evidence: NOT_SUPPLIED, never a permission prompt. The 3D view keeps its own logic
 *   (captured, or not and why, at each step under --show-window), and an attended capture can supply it too.
 * The walkthrough's screens are the scripted native journeys' (or, with none, the attended launches'). A surface name
 * a sidecar gives outside the set is listed in `unexplained`, not dropped. A browser-only index lists no surface.
 */
export function nativeSurfacesOf(journeys, { bindings = { checks: new Map(), claims: new Map(), unbound: [] } } = {}) {
  const bySurface = new Map();
  for (const item of journeys.flatMap((journey) => journey.native_surfaces ?? [])) {
    const id = keyOf(item) ?? item.surface;
    const held = bySurface.get(id);
    if (!held || (item.missing_evidence && !held.missing_evidence)) bySurface.set(id, item);
  }
  const native = journeys.filter((journey) => surfaceOf(journey) !== "browser build");
  if (native.length === 0) return { surfaces: [...bySurface.values()], unexplained: [] };
  const attendedParts = native.filter(attendedJourney);
  const scripted = native.filter((journey) => !attendedJourney(journey));
  const basis = scripted.length > 0 ? scripted : attendedParts;
  const commits = [...new Set(basis.map((journey) => journey.revision?.commit ?? null))];
  const drivers = [...new Set(basis.map((journey) => journey.revision?.driver_sha256 ?? null))];
  const builds = [...new Set(basis.map((journey) => journey.build?.app_sha256 ?? null))];
  const dirty = basis.filter((journey) => journey.revision?.uncommitted_paths !== 0);
  const short = (digest) => shortCommit(String(digest).replace(/^sha256:/, ""));
  const whyNotPart = (part) => {
    const revision = part.revision;
    if (typeof revision?.commit !== "string") return "its revision is not recorded";
    if (commits.length !== 1) return `the walkthrough's screens are at ${commits.length} revisions`;
    if (commits[0] === null) return "the walkthrough's screens record no revision";
    if (revision.commit !== commits[0]) return `it is at revision ${shortCommit(revision.commit)}, not the walkthrough's ${shortCommit(commits[0])}`;
    // A commit names the source only when nothing was uncommitted, and the driver given to the app is not in it.
    if (revision.uncommitted_paths !== 0) return `its revision had ${revision.uncommitted_paths ?? "an unrecorded number of"} uncommitted path(s): the commit does not name the source it ran`;
    if (dirty.length > 0) return `the walkthrough's screens were taken with uncommitted paths (${dirty.map((journey) => journey.name).join(", ")}): the commit does not name the source they ran`;
    const driver = revision.driver_sha256 ?? null;
    if (driver === null) return "its driver's digest is not recorded";
    if (drivers.length !== 1 || drivers[0] === null) return "the walkthrough's screens record no single driver";
    if (driver !== drivers[0]) return `it ran another driver (${short(driver)}, not the walkthrough's ${short(drivers[0])})`;
    const build = part.build?.app_sha256 ?? null;
    if (build === null) return "its build is not recorded";
    if (builds.length !== 1 || builds[0] === null) return "the walkthrough's screens record no single build";
    if (build !== builds[0]) return `it is of another build (app ${short(build)}, not the walkthrough's ${short(builds[0])})`;
    return null;
  };
  const supplies = new Map();
  const attempts = new Map();
  const notSupplying = new Map();
  const unexplained = [];
  const add = (map, key, value) => map.set(key, [...(map.get(key) ?? []), value]);
  // The attendant's statements that a surface could not be shown, from a capture or a sidecar with no picture.
  const stated = (record, ref, partWhy) => {
    const kind = "statement that it was unseen";
    if (Object.hasOwn(SUBSTITUTED_BY_BUILD, record.surface)) return add(notSupplying, record.surface, { ...ref, kind, why: SUBSTITUTED_BY_BUILD[record.surface] });
    // Not counted, and kept in full: the attendant's statement, and why it makes nothing unseen.
    if (record.why) return add(notSupplying, record.surface, { ...ref, kind, statement: { step: record.step, observed: record.observed, obstacle: record.obstacle, cause: record.cause }, why: `the attendant's statement does not count: ${record.why}` });
    const attempt = { ...ref, step_attempted: record.step, observed: record.observed, obstacle: record.obstacle, cause: record.cause, statement: statementOf(record) };
    return partWhy === null ? add(attempts, record.surface, attempt) : add(notSupplying, record.surface, { ...attempt, kind, why: partWhy });
  };
  for (const part of attendedParts) {
    const at = { revision: part.revision ?? null, build: part.build ?? null };
    const partWhy = whyNotPart(part);
    const claim = (key, ref, map, kind, why = partWhy) => (why === null ? add(map, key, ref) : add(notSupplying, key, { ...ref, kind, why }));
    // What each capture and record claims is read from its verified sidecar bytes (attendedBindings); without them,
    // what the part saved is listed as not counted, with why, and counts for nothing.
    const claimsOf = (file, saved) => {
      const verified = bindings.claims?.get(file) ?? null;
      return { from: verified ?? saved, why: verified ? null : (bindings.checks.get(file) ?? "its sidecar was not read against its recorded hash") };
    };
    for (const screen of part.screens ?? []) {
      if (screen.capture !== "the actual window") continue;
      const ref = { file: screen.file, number: screen.number, step: screen.step, ...at };
      const { from, why: bound } = claimsOf(screen.file, screen);
      for (const key of from.surfaces ?? []) claim(key, ref, supplies, "capture", partWhy ?? bound);
      for (const record of [...(from.prevented ?? []), ...(from.prevented_incomplete ?? [])]) stated(record, ref, partWhy ?? bound);
      for (const name of from.surfaces_unexplained ?? []) unexplained.push({ name, file: screen.file, member: "surfaces" });
      for (const record of from.prevented_unexplained ?? []) unexplained.push({ name: record.surface, file: screen.file, member: "prevented", observed: record.observed });
    }
    for (const record of part.captures?.records_without_picture ?? []) {
      const ref = { file: record.file, step: record.step ?? null, ...at };
      const { from, why: bound } = claimsOf(record.file, record);
      for (const key of from.surfaces ?? []) add(notSupplying, key, { ...ref, kind: "capture", why: "a sidecar with no picture shows nothing" });
      for (const item of [...(from.prevented ?? []), ...(from.prevented_incomplete ?? [])]) stated(item, ref, partWhy ?? bound);
      for (const name of from.surfaces_unexplained ?? []) unexplained.push({ name, file: record.file, member: "surfaces" });
      for (const item of from.prevented_unexplained ?? []) unexplained.push({ name: item.surface, file: record.file, member: "prevented", observed: item.observed });
    }
  }
  const decide = (key, item) => {
    const base = item ?? NATIVE_SURFACES.find((entry) => entry.key === key) ?? { surface: SURFACE_KEYS[key], captured: false, instead: "none: the scripted runner does not meet it" };
    const extra = notSupplying.has(key) ? { not_supplying: notSupplying.get(key) } : {};
    if (supplies.has(key)) return { ...base, key, captured: true, evidence: "supplied", missing_evidence: false, prevented_by: "nothing: an attended capture of the actual window shows it", captures: supplies.get(key), ...extra };
    if (key === "view_3d" && base.evidence === "supplied") return { ...base, key, ...extra };
    if (attempts.has(key)) return { ...base, key, captured: false, evidence: "unseen", unseen: true, missing_evidence: false, prevented_by: `unseen, as the attendant states: ${attempts.get(key).map((attempt) => attempt.statement).join("; ")}`, attempts: attempts.get(key), ...extra };
    if (key === "view_3d") return { ...base, key, ...extra };
    return { ...base, key, captured: false, evidence: "missing", missing_evidence: true, prevented_by: NOT_SUPPLIED, ...extra };
  };
  const decided = [];
  const keys = new Set();
  for (const item of bySurface.values()) {
    const key = keyOf(item);
    if (key) keys.add(key);
    decided.push(key ? decide(key, item) : item);
  }
  for (const key of SURFACE_ORDER) {
    if (keys.has(key)) continue;
    if (ATTENDED_ONLY_SURFACES.includes(key) || supplies.has(key) || attempts.has(key) || notSupplying.has(key)) decided.push(decide(key, null));
  }
  const rank = (item) => { const at = SURFACE_ORDER.indexOf(item.key ?? ""); return at === -1 ? SURFACE_ORDER.length : at; };
  return { surfaces: decided.map((item, index) => [item, index]).sort(([a, i], [b, j]) => rank(a) - rank(b) || i - j).map(([item]) => item), unexplained };
}

export function composeIndex(out, { size = null } = {}) {
  const journeys = readParts(out);
  const surfaces = [...new Set(journeys.map(surfaceOf))];
  const windows = [...new Set(journeys.map((journey) => journey.window ?? size).filter(Boolean))];
  // Each attended capture's binding to its launch, checked against the files as they now stand (attendedBindings).
  const bindings = attendedBindings(out, journeys);
  // Every screen carries its journey's revision, build and use case (DEL-07-06 VER-007: the index records, for each
  // screen, the revision, the build, the use case, the step, the picture and the words and controls on screen).
  const screens = journeys.flatMap((journey) => journey.screens.map((screen) => ({
    ...screen,
    revision: journey.revision ?? null,
    build: journey.build ?? null,
    use_case: screen.use_case ?? journey.use_case ?? journey.title,
    ...(screen.file && !screen.redrawn ? { redrawn: false } : {}),
    ...(bindings.checks.has(screen.file) ? { binding_check: bindings.checks.get(screen.file) ?? "intact: the picture and its sidecar are the ones its launch recorded" } : {})
  })));
  // The native surfaces, each named once: supplied, unseen as the attendant states, or missing evidence (nativeSurfacesOf).
  const { surfaces: nativeSurfaces, unexplained: unexplainedSurfaces } = nativeSurfacesOf(journeys, { bindings });
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
      native_surfaces: "each native surface of the launched app: supplied (the attended captures of the actual window that show it, with their revision and build), unseen, as the attendant states (a statement naming the step attempted, the surface, what was observed, whether the obstacle is the system's or the application's and what in it kept the surface from the capture, given in full for the reader to accept or not; never the file chooser or the print dialog, which the scripted-run build substitutes, and never an account naming the attendant's own tool or its setup or leaving the reason unresolved, which is missing evidence), or missing evidence (not supplied: no attended capture of it in this walkthrough); and what the scripted runner does instead",
      native_menus: "for each launch of the app, the menu it built, read from it once by the scripted-run build: each menu's items in order with label, command identifier, enabled and ticked state, and separators; names, order and state, not appearance or placement",
      window_shown: "on a native screen, whether the run brought the app's window to the front (--show-window) so its web view drew; page_visibility is what the page reported at that step",
      ...(journeys.some(attendedJourney) ? {
        capture: "on an attended launch's screen: \"the actual window\" where the picture is a capture of the launched app's actual window that the attendant put in captures/ with a sidecar saying what it shows, redrawn false and scripted false; \"unexplained\" where a picture has no such sidecar, and nothing is claimed for it",
        stated_by_attendant: "on an attended capture: what the attendant states in its sidecar (the use case, the step, what was done since the previous capture, the note, when it was taken, the surfaces it shows), not checked by the runner except taken_at against the picture's file time (taken_at_check)",
        counts_source: "on an attended capture: whose the words and controls on screen are, and when: the application's own report nearest at or before the time the attendant states, recorded by the launch's driver as the page settled",
        attended_substitutions: "on an attended capture: each act the launch stopped and each substitution the build made, with its time, since the previous capture"
      } : {})
    },
    native_surfaces: nativeSurfaces,
    ...(unexplainedSurfaces.length > 0 ? { native_surfaces_unexplained: unexplainedSurfaces } : {}),
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
      not_reached: journey.screens.filter((screen) => screen.not_reached).map((screen) => screen.step),
      ...(attendedJourney(journey) ? { stopped_by_launch: (journey.launch?.stopped_by_launch ?? []).map((item) => ({ at: typeof item.at === "number" ? new Date(item.at).toISOString() : null, control: item.control ?? null, what: item.what ?? null })) } : {}),
      ...(attendedJourney(journey) ? { unexplained: [...(journey.captures?.unexplained ?? []), ...[...bindings.checks].filter(([, why]) => why).map(([file, why]) => ({ file, why: `${/\.json$/i.test(file) ? "a sidecar recorded without a picture that counts for nothing" : "a capture that supplies nothing"}: ${why}` })), ...bindings.unbound] } : {})
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
  const attended = screen.capture === "the actual window";
  const substituted = !screen.substitutions?.length ? ""
    : attended ? `<p class="substituted">Since ${escapeHtml(screen.substitutions_since ?? "the previous capture")}, stopped by the launch or substituted by the build: ${screen.substitutions.map((item) => `${escapeHtml(item.at)}, ${escapeHtml(item.kind)}${item.control ? ` (${escapeHtml(item.control)})` : ""}: ${escapeHtml(item.what)}`).join("; ")}</p>`
    : `<p class="substituted">Substituted (scripted-run build): ${screen.substitutions.map((item) => `${escapeHtml(item.kind)}: ${escapeHtml(item.what)}`).join("; ")}.</p>`;
  const stated = attended && screen.stated_by_attendant ? `<p class="basis">Stated by the attendant, with the step and the note above: use case ${escapeHtml(screen.stated_by_attendant.use_case)}; taken at ${escapeHtml(screen.stated_by_attendant.taken_at)} (${escapeHtml(screen.taken_at_check ?? "not checked")}); done since the previous capture: ${screen.stated_by_attendant.acted.map(escapeHtml).join("; ")}${screen.stated_by_attendant.surfaces?.length ? `; surfaces shown: ${screen.stated_by_attendant.surfaces.map(escapeHtml).join(", ")}` : ""}.</p>` : "";
  const source = attended && screen.counts_source ? `<p class="basis">Words and controls on screen: ${escapeHtml(screen.counts_source)}.${screen.controls_cut ? ` ${escapeHtml(screen.controls_cut)}.` : ""}</p>` : "";
  const unbound = screen.binding_check && !screen.binding_check.startsWith("intact") ? `<p class="stop">Not counted as evidence: ${escapeHtml(screen.binding_check)}.</p>` : "";
  const unpictured = screen.picture_error ? `<p class="stop">No picture: the redraw failed (${escapeHtml(screen.picture_error)}).</p>` : "";
  const picture = screen.file ? `<a href="${escapeHtml(screen.file)}"><img loading="lazy" src="${escapeHtml(screen.file)}" alt="${escapeHtml(screen.note)}"></a>` : "";
  return `<li class="screen${screen.failed ? " stopped" : ""}" id="s${screen.number}"><h3>${screen.number} · ${escapeHtml(screen.step)}</h3><p>${escapeHtml(screen.note)}</p>${stop}${stated}${substituted}${picture}${basis}${unbound}${unpictured}${source}${counts}${controls}</li>`;
}

// One native surface: supplied (with the captures that show it), unseen as the attendant states, or missing evidence.
function renderSurface(item) {
  const revision = (ref) => `revision ${escapeHtml(shortCommit(ref.revision?.commit))}${ref.build?.app_sha256 ? `, app ${escapeHtml(String(ref.build.app_sha256).replace(/^sha256:/, "").slice(0, 12))}` : ""}`;
  const elsewhere = item.not_supplying?.length ? ` Not counted: ${item.not_supplying.map((ref) => `${escapeHtml(ref.file ?? ref.source ?? "a record")} (${escapeHtml(ref.why)})`).join("; ")}.` : "";
  if (item.evidence === "supplied" && item.captures?.length) {
    return `<li><strong>${escapeHtml(item.surface)}</strong>: supplied by attended capture${item.captures.length === 1 ? "" : "s"} ${item.captures.map((ref) => `<a href="${escapeHtml(ref.file)}">${escapeHtml(ref.step ?? ref.file)}</a> (${revision(ref)})`).join(", ")}. Evidence supplied.${elsewhere}</li>`;
  }
  if (item.evidence === "unseen") return `<li><strong>${escapeHtml(item.surface)}</strong>: ${escapeHtml(item.prevented_by)}. For the reader to accept or not: the runner does not check it.${elsewhere}</li>`;
  return `<li><strong>${escapeHtml(item.surface)}</strong>: ${escapeHtml(item.prevented_by)}. Instead: ${escapeHtml(item.instead)}.${item.missing_evidence ? ' <span class="stop">Missing evidence.</span>' : item.evidence === "supplied" ? " Evidence supplied." : ""}${elsewhere}</li>`;
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
    const unexplained = journey.unexplained?.length ? `<p class="stop">In this launch's captures, supplying nothing:</p><ul class="unexplained">${journey.unexplained.map((item) => `<li>${escapeHtml(item.file)}: ${escapeHtml(item.why)}</li>`).join("")}</ul>` : "";
    const stops = journey.stopped_by_launch?.length ? `<p class="substituted">Stopped by the launch, while the attendant worked:</p><ul class="stops">${journey.stopped_by_launch.map((item) => `<li>${escapeHtml(item.at ?? "at an unrecorded time")}, ${escapeHtml(item.control ?? "an unnamed control")}: ${escapeHtml(item.what ?? "")}</li>`).join("")}</ul>` : "";
    const onScreen = journey.launch?.attendant_on_screen;
    const agent = onScreen?.stated ? `<p class="substituted">Started with an agent on the screen: ${escapeHtml(onScreen.marks_before_start?.length ? onScreen.marks_before_start.join("; ") : "no mark was found directly before the start")}. ${escapeHtml(onScreen.stated_by)} stated that ${escapeHtml(onScreen.statement)} (${escapeHtml(onScreen.option)}).</p>` : "";
    const builds = (journey.built_placements ?? []).map((item) => `<p class="surface">Placed ${escapeHtml(item.key)}: built by ${escapeHtml(item.build)} from ${escapeHtml(item.from)} (SQL SHA-256 ${escapeHtml(item.sql_sha256)}; built file SHA-256 ${escapeHtml(item.sha256)}, ${item.bytes} bytes).</p>`).join("");
    const stored = journey.stored_runs
      ? `<p class="surface">Stored runs: ${journey.stored_runs.error ? `not read (${escapeHtml(journey.stored_runs.error)})` : `${journey.stored_runs.unchanged ? "unchanged" : "not shown unchanged"} at ${escapeHtml(Object.keys(journey.stored_runs.at).join(", "))}: ${escapeHtml(journey.stored_runs.what)}`}.</p>`
      : "";
    return `<section id="j${journey.number}"><h2>${journey.number}. ${escapeHtml(journey.title)}</h2>${facts}${builds}${stored}${surface}${agent}${stopped}${stops}${unexplained}<ol class="screens">${screens.map(renderScreen).join("")}</ol></section>`;
  }).join("");
  const rows = index.screens.filter((screen) => screen.words_on_screen !== undefined).map((screen) =>
    `<tr><td><a href="#s${screen.number}">${screen.number}</a></td><td>${escapeHtml(screen.journey)}</td><td>${escapeHtml(screen.step)}</td><td>${[screen.redrawn ? "redrawn" : "", screen.capture === "the actual window" ? "attended capture; counts the application's own" : "", screen.substitutions?.length ? "substituted" : "", screen.window_shown ? "window shown" : ""].filter(Boolean).join(", ")}</td><td class="n">${screen.words_on_screen}</td><td class="n">${screen.words_in_page}</td><td class="n">${screen.buttons_on_screen}</td><td class="n">${screen.inputs_on_screen}</td><td class="n">${screen.longest_scroll_screens}</td></tr>`).join("");
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
<p class="lede">Screens of the application (${escapeHtml(index.surface)}${index.window ? `, at ${escapeHtml(index.window)}` : ""}), taken by driving its own controls. Each screen lists what a person has in front of them. A journey that stops shows where a person would be stuck.${index.journeys.some((journey) => journey.surface !== "browser build" && !attendedJourney(journey)) ? " Journeys of the launched app are not captures of its window: each picture is the launched app's document, recorded in its own web view, redrawn by Chromium, so fonts and form controls may differ in detail. Each journey states its surface." : ""}${index.journeys.some(attendedJourney) ? " An attended launch's pictures are captures of the actual window, taken by the attendant who drove it, not redrawn and not scripted; each is evidence only of what it shows." : ""}</p>
<nav><ol>${index.journeys.map((journey) => `<li><a href="#j${journey.number}">${escapeHtml(journey.title)}</a>${journey.stopped_at ? ` <span class="stop">(stopped at “${escapeHtml(journey.stopped_at)}”)</span>` : ""}</li>`).join("")}</ol></nav>
${index.native_surfaces.length ? `<h2>Native surfaces not pictured, and those an attended capture supplies</h2><ul class="surfaces">${index.native_surfaces.map(renderSurface).join("")}</ul>${(index.native_surfaces_unexplained ?? []).length ? `<p class="stop">Unexplained surface names in sidecars: ${index.native_surfaces_unexplained.map((item) => `${escapeHtml(item.name)} (${escapeHtml(item.file)}, ${escapeHtml(item.member)})`).join("; ")}.</p>` : ""}` : ""}
${index.native_menus.length ? renderMenus(index.native_menus) : ""}
<h2>Every screen, by the numbers</h2>
<div class="table-wrap"><table><thead><tr><th>No.</th><th>Journey</th><th>Step</th><th>Marks</th><th class="n">Words on screen</th><th class="n">Words in page</th><th class="n">Buttons</th><th class="n">Inputs</th><th class="n">Scroll</th></tr></thead><tbody>${rows}</tbody></table></div>
${sections}
</main></body></html>
`;
}
