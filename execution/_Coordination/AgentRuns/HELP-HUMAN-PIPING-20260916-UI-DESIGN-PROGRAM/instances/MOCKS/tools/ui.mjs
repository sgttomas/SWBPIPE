// HTML renderers for the shell and the components, following the design system specimen (V1.2).
// The status and evidence labels and the agent's card classes are read from the design system's
// tokens.json (labels, agentCardClasses): no label is written by hand in a frame.
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import * as M from "./model.mjs";
import { esc, scaleColor } from "./canvas.mjs";

const TOKENS = JSON.parse(fs.readFileSync(path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..", "..", "DESIGN-SYSTEM", "tokens.json"), "utf8"));
export const LABELS = TOKENS.labels;
export const CARD_CLASSES = TOKENS.agentCardClasses;
// A label chip (V1.2 §2.3): the authority domain, a middle dot, the label; the raw token is the
// chip's tooltip and its data-label, so that it is reachable in place and the lint can check it.
export function labelChip(raw, extraTitle = "") {
  const l = LABELS[raw]; if (!l) throw new Error("no such label: " + raw);
  return `<span class="chip ${l.chip} lbl" data-label="${raw}" title="${raw}${extraTitle ? " · " + esc(extraTitle) : ""}"><span class="dom">${esc(l.domain)}</span><span class="sep">·</span>${esc(l.label)}</span>`;
}
// An agent card's class word (V1.2 §5.4, ruling 8): one of the five, or Note for the engineer's own comment.
export function kindChip(kind) {
  if (kind !== "Note" && !CARD_CLASSES.includes(kind)) throw new Error("not a card class: " + kind);
  return `<span class="chip outline kind" data-kind="${esc(kind)}">${esc(kind)}</span>`;
}
export const iconbtn = (name, title, o = {}) => `<span class="iconbtn${o.cls ? " " + o.cls : ""}${o.off ? " off" : ""}" role="button" title="${esc(title)}">${icon(name, o.size === 16 ? "" : "s12")}</span>`;

export const num = (v, d = 0) => { if (v == null || v === "") return ""; const s = Math.abs(v).toFixed(d); return (v < 0 ? "−" : "") + s; };
export const icon = (name, cls = "") => `<svg class="ic ${cls}"><use href="#i-${name}"/></svg>`;
export const mark = (name, color, title) => `<svg class="mk"${color ? ` style="color:${color}"` : ""}${title ? ` title="${esc(title)}"` : ""}><use href="#m-${name}"/></svg>`;

export function sprite() {
  return `<svg style="display:none" aria-hidden="true">
  <symbol id="i-model" viewBox="0 0 16 16"><circle cx="4" cy="12" r="1.6"/><circle cx="12" cy="4" r="1.6"/><path d="M4 10.4V7a3 3 0 0 1 3-3h3.4"/></symbol>
  <symbol id="i-loads" viewBox="0 0 16 16"><path d="M2 13.5h12"/><path d="M8 2v8.5"/><path d="M5 7.5 8 10.5l3-3"/></symbol>
  <symbol id="i-results" viewBox="0 0 16 16"><path d="M2.5 4h11"/><path d="M2.5 8h7"/><path d="M2.5 12h9"/></symbol>
  <symbol id="i-review" viewBox="0 0 16 16"><rect x="3" y="2" width="10" height="12" rx="1.2"/><path d="M6 9.5 10.5 5"/><path d="M5.5 11l.5-1.5"/></symbol>
  <symbol id="i-libraries" viewBox="0 0 16 16"><rect x="3" y="2.5" width="10" height="2.6" rx=".8"/><rect x="3" y="6.7" width="10" height="2.6" rx=".8"/><rect x="3" y="10.9" width="10" height="2.6" rx=".8"/></symbol>
  <symbol id="i-rules" viewBox="0 0 16 16"><g transform="rotate(-45 8 8)"><rect x="1.5" y="6" width="13" height="4" rx=".8"/><path d="M4.5 6v1.5M7 6v2.5M9.5 6v1.5M12 6v2.5"/></g></symbol>
  <symbol id="i-issues" viewBox="0 0 16 16"><circle cx="8" cy="8" r="6"/><path d="M8 5v3.5"/><circle cx="8" cy="11" r=".7" fill="currentColor" stroke="none"/></symbol>
  <symbol id="i-table" viewBox="0 0 16 16"><rect x="2" y="2" width="12" height="12" rx="1"/><path d="M2 6h12M2 10h12M6 2v12M10 2v12"/></symbol>
  <symbol id="i-cube" viewBox="0 0 16 16"><path d="M8 2l6 3v6l-6 3-6-3V5z"/><path d="M8 8l6-3M8 8 2 5M8 8v6"/></symbol>
  <symbol id="i-both" viewBox="0 0 16 16"><rect x="2" y="2" width="12" height="12" rx="1"/><path d="M8 2v12M2 6h6M2 10h6"/><circle cx="11" cy="8" r="1.5"/></symbol>
  <symbol id="i-fit" viewBox="0 0 16 16"><path d="M2 5.5V2h3.5M10.5 2H14v3.5M14 10.5V14h-3.5M5.5 14H2v-3.5"/></symbol>
  <symbol id="i-presets" viewBox="0 0 16 16"><path d="M8 2l6 3v6l-6 3-6-3V5z"/><path d="M8 8l6-3M8 8 2 5M8 8v6"/><path d="M8 2l6 3-6 3-6-3z" fill="currentColor" fill-opacity=".28" stroke="none"/></symbol>
  <symbol id="i-section" viewBox="0 0 16 16"><path d="M8 2l6 3v6l-6 3-6-3V5z"/><path d="M1.5 10 14.5 5.5" stroke-dasharray="2 1.5"/></symbol>
  <symbol id="i-isolate" viewBox="0 0 16 16"><rect x="2" y="2" width="5" height="5" rx=".8" fill="currentColor" stroke="none"/><rect x="9" y="2" width="5" height="5" rx=".8" stroke-dasharray="1.5 1.5"/><rect x="2" y="9" width="5" height="5" rx=".8" stroke-dasharray="1.5 1.5"/><rect x="9" y="9" width="5" height="5" rx=".8" stroke-dasharray="1.5 1.5"/></symbol>
  <symbol id="i-hide" viewBox="0 0 16 16"><path d="M2 8s2.5-4 6-4 6 4 6 4-2.5 4-6 4-6-4-6-4z"/><circle cx="8" cy="8" r="1.8"/><path d="M3 13 13 3"/></symbol>
  <symbol id="i-labels" viewBox="0 0 16 16"><path d="M2.5 3.5h7l4 4.5-4 4.5h-7z"/><circle cx="5.5" cy="8" r=".8" fill="currentColor" stroke="none"/><circle cx="8.5" cy="8" r=".8" fill="currentColor" stroke="none"/></symbol>
  <symbol id="i-deform" viewBox="0 0 16 16"><path d="M2 12c3 0 3-8 6-8s3 8 6 8"/><path d="M2 8h12" stroke-dasharray="1.5 2" stroke-width="1"/></symbol>
  <symbol id="i-probe" viewBox="0 0 16 16"><circle cx="8" cy="8" r="4.5"/><path d="M8 1.5v3M8 11.5v3M1.5 8h3M11.5 8h3"/><circle cx="8" cy="8" r=".9" fill="currentColor" stroke="none"/></symbol>
  <symbol id="i-route" viewBox="0 0 16 16"><path d="M2.5 13.5V7h5V3h5"/><path d="M10.5 1l2.5 2-2.5 2"/><circle cx="2.5" cy="13.5" r="1.2"/></symbol>
  <symbol id="i-restrain" viewBox="0 0 16 16"><path d="M2 5h12"/><path d="M8 5.5 4.5 12h7z"/></symbol>
  <symbol id="i-addrow" viewBox="0 0 16 16"><rect x="2" y="5" width="12" height="6" rx="1"/><path d="M8 5.8v4.4M5.8 8h4.4"/></symbol>
  <symbol id="i-delrow" viewBox="0 0 16 16"><rect x="2" y="5" width="12" height="6" rx="1"/><path d="M5.8 8h4.4"/></symbol>
  <symbol id="i-sort" viewBox="0 0 16 16"><path d="M5 13V3M2.5 5.5 5 3l2.5 2.5"/><path d="M11 3v10M8.5 10.5 11 13l2.5-2.5"/></symbol>
  <symbol id="i-filter" viewBox="0 0 16 16"><path d="M2 3h12l-4.5 5.5v4l-3 1.5V8.5z"/></symbol>
  <symbol id="i-columns" viewBox="0 0 16 16"><path d="M4 2.5v11M8 2.5v11M12 2.5v11"/></symbol>
  <symbol id="i-paste" viewBox="0 0 16 16"><rect x="3" y="3" width="10" height="11" rx="1"/><path d="M6 3V2h4v1"/><path d="M5.5 7.5h5M5.5 10.5h5M8 7.5v3" stroke-width="1"/></symbol>
  <symbol id="i-expand" viewBox="0 0 16 16"><path d="M6 3.5 10.5 8 6 12.5"/></symbol>
  <symbol id="i-expanded" viewBox="0 0 16 16"><path d="M3.5 6 8 10.5 12.5 6"/></symbol>
  <symbol id="i-since" viewBox="0 0 16 16"><path d="M3 8.5a5.5 5.5 0 1 0 1.6-3.9"/><path d="M3 2.5v2.5h2.5"/><path d="M8.5 5.5v3l2 1.5"/></symbol>
  <symbol id="i-unchecked" viewBox="0 0 16 16"><rect x="2.5" y="2.5" width="11" height="11" rx="1.5"/></symbol>
  <symbol id="i-origins" viewBox="0 0 16 16"><circle cx="8" cy="8" r="1.8" fill="currentColor" stroke="none"/><path d="M8 2v2M8 12v2M2 8h2M12 8h2M3.8 3.8l1.4 1.4M10.8 10.8l1.4 1.4M12.2 3.8l-1.4 1.4M5.2 10.8l-1.4 1.4" stroke-width="1"/></symbol>
  <symbol id="i-agent" viewBox="0 0 16 16"><path d="M3 3h10a1 1 0 0 1 1 1v6a1 1 0 0 1-1 1H7l-3 2.5V11H3a1 1 0 0 1-1-1V4a1 1 0 0 1 1-1z"/><path d="M8 5v4M6 7h4" stroke-width="1"/></symbol>
  <symbol id="i-run" viewBox="0 0 16 16"><path d="M5 3l8 5-8 5z" fill="currentColor" stroke="none"/></symbol>
  <symbol id="i-stop" viewBox="0 0 16 16"><rect x="4" y="4" width="8" height="8" rx="1" fill="currentColor" stroke="none"/></symbol>
  <symbol id="i-search" viewBox="0 0 16 16"><circle cx="7" cy="7" r="4.5"/><path d="M10.5 10.5 14 14"/></symbol>
  <symbol id="i-info" viewBox="0 0 16 16"><circle cx="8" cy="8" r="6"/><path d="M8 7.5v4"/><circle cx="8" cy="5.2" r=".7" fill="currentColor" stroke="none"/></symbol>
  <symbol id="i-check" viewBox="0 0 16 16"><path d="M3 8.5 6.5 12 13 4.5"/></symbol>
  <symbol id="i-close" viewBox="0 0 16 16"><path d="M4 4l8 8M12 4l-8 8"/></symbol>
  <symbol id="i-lock" viewBox="0 0 16 16"><rect x="3.5" y="7" width="9" height="7" rx="1"/><path d="M5.5 7V5a2.5 2.5 0 0 1 5 0v2"/></symbol>
  <symbol id="i-play" viewBox="0 0 16 16"><path d="M5 3l8 5-8 5z" fill="currentColor" stroke="none"/></symbol>
  <symbol id="m-entered" viewBox="0 0 12 12"><circle cx="6" cy="6" r="1.6" fill="currentColor" stroke="none"/></symbol>
  <symbol id="m-accepted" viewBox="0 0 12 12"><path d="M6 1.5 10.5 6 6 10.5 1.5 6z"/><circle cx="6" cy="6" r="1" fill="currentColor" stroke="none"/></symbol>
  <symbol id="m-propagated" viewBox="0 0 12 12"><path d="M6 1.5v6M3.5 5 6 7.5 8.5 5"/><path d="M2.5 10.5h7"/></symbol>
  <symbol id="m-generated" viewBox="0 0 12 12"><path d="M6 1.5 7.2 4.8 10.5 6 7.2 7.2 6 10.5 4.8 7.2 1.5 6l3.3-1.2z" fill="currentColor" stroke="none"/></symbol>
  <symbol id="m-imported" viewBox="0 0 12 12"><path d="M6 1.5v6M3.5 5 6 7.5 8.5 5"/><path d="M1.5 8v2.5h9V8"/></symbol>
  <symbol id="m-proposed" viewBox="0 0 12 12"><path d="M6 1.2 10.8 6 6 10.8 1.2 6z" fill="currentColor" stroke="none"/></symbol>
  <symbol id="m-checked" viewBox="0 0 12 12"><path d="M2 6.5 5 9.5 10 3.5"/></symbol>
  <symbol id="m-stale" viewBox="0 0 12 12"><path d="M2 6.5 5 9.5 10 3.5" stroke-dasharray="2 1.5"/><circle cx="10" cy="9.5" r="1.4" fill="currentColor" stroke="none"/></symbol>
  <symbol id="m-stalerun" viewBox="0 0 12 12"><circle cx="6" cy="6" r="4.5" stroke-dasharray="2 1.5"/><circle cx="6" cy="6" r="1.2" fill="currentColor" stroke="none"/></symbol>
  <symbol id="m-restraint" viewBox="0 0 12 12"><path d="M1.5 3.5h9"/><path d="M6 4 3.5 9.5h5z"/></symbol>
  <symbol id="m-load" viewBox="0 0 12 12"><path d="M6 1v6M3.5 4.5 6 7l2.5-2.5"/><circle cx="6" cy="10" r="1.4" fill="currentColor" stroke="none"/></symbol>
  <symbol id="m-nodedata" viewBox="0 0 12 12"><path d="M1.5 8.5h9"/><path d="M6 8.5V3"/><circle cx="6" cy="8.5" r="1.3" fill="currentColor" stroke="none"/></symbol>
  <symbol id="m-blocking" viewBox="0 0 12 12"><path d="M4 1.5h4l2.5 2.5v4L8 10.5H4L1.5 8V4z" fill="currentColor" stroke="none"/></symbol>
  <symbol id="m-warning" viewBox="0 0 12 12"><path d="M6 1.5 11 10.5H1z" fill="currentColor" stroke="none"/><path d="M6 4.6v2.8" stroke="var(--surface-panel)" stroke-width="1.4"/><circle cx="6" cy="9" r=".7" fill="var(--surface-panel)" stroke="none"/></symbol>
  <symbol id="m-info" viewBox="0 0 12 12"><circle cx="6" cy="6" r="4.75"/><path d="M6 5.5v3"/><circle cx="6" cy="3.6" r=".6" fill="currentColor" stroke="none"/></symbol>
  <symbol id="m-required" viewBox="0 0 12 12"><path d="M6 1.5v9M2.1 3.75l7.8 4.5M9.9 3.75l-7.8 4.5" stroke-width="1.4"/></symbol>
  <symbol id="m-historical" viewBox="0 0 12 12"><circle cx="6" cy="6" r="4.5"/><path d="M6 3.5V6l1.8 1.3"/></symbol>
  <symbol id="m-displayonly" viewBox="0 0 12 12"><rect x="1.5" y="1.5" width="9" height="9" rx="1" stroke-dasharray="1.5 1.2"/><path d="M3.5 4.5h5M3.5 6.5h5M3.5 8.5h3" stroke-width="1"/></symbol>
  <symbol id="m-sortdesc" viewBox="0 0 12 12"><path d="M6 1.5v9M2.5 7 6 10.5 9.5 7"/></symbol>
  <symbol id="i-units" viewBox="0 0 16 16"><path d="M2 11h12"/><path d="M4 11V7M7 11V5M10 11V7M13 11V5" stroke-width="1"/></symbol>
  <symbol id="i-preview" viewBox="0 0 16 16"><path d="M8.5 14H4a1 1 0 0 1-1-1V3a1 1 0 0 1 1-1h5l3 3v2"/><path d="M9 2v3h3"/><circle cx="10.8" cy="10.8" r="2.2"/><path d="M12.4 12.4 14.3 14.3"/></symbol>
  <symbol id="i-export" viewBox="0 0 16 16"><path d="M2.5 9.5v3a1 1 0 0 0 1 1h9a1 1 0 0 0 1-1v-3"/><path d="M8 10V2.5"/><path d="M5 5.5 8 2.5l3 3"/></symbol>
  <symbol id="i-snapshot" viewBox="0 0 16 16"><path d="M2.5 5.5a1 1 0 0 1 1-1h1.6l1-1.5h3.8l1 1.5h1.6a1 1 0 0 1 1 1v6a1 1 0 0 1-1 1h-9a1 1 0 0 1-1-1z"/><circle cx="8" cy="8.4" r="2.3"/></symbol>
  <symbol id="i-undo" viewBox="0 0 16 16"><path d="M5.5 3 2.5 6l3 3"/><path d="M2.5 6H9a4 4 0 0 1 0 8H6"/></symbol>
  <symbol id="i-redo" viewBox="0 0 16 16"><path d="M10.5 3l3 3-3 3"/><path d="M13.5 6H7a4 4 0 0 0 0 8h3"/></symbol>
  <symbol id="i-inspector" viewBox="0 0 16 16"><rect x="2" y="3" width="12" height="10" rx="1"/><path d="M10 3v10"/><path d="M11.5 6h1M11.5 8h1" stroke-width="1"/></symbol>
  <symbol id="i-send" viewBox="0 0 16 16"><path d="M2.5 8 13.5 2.5 10 13.5 7.8 9.2z"/><path d="M7.8 9.2 13.5 2.5"/></symbol>
  <symbol id="i-copy" viewBox="0 0 16 16"><rect x="5.5" y="5.5" width="8" height="8" rx="1"/><path d="M10.5 5.5V3.5a1 1 0 0 0-1-1h-6a1 1 0 0 0-1 1v6a1 1 0 0 0 1 1h2"/></symbol>
  <symbol id="i-reverse" viewBox="0 0 16 16"><path d="M3 5.5h10M10.5 3 13 5.5 10.5 8"/><path d="M13 10.5H3M5.5 8 3 10.5 5.5 13"/></symbol>
  <symbol id="i-collapse" viewBox="0 0 16 16"><path d="M3.5 6 8 10.5 12.5 6"/><path d="M3 13h10"/></symbol>
  <symbol id="m-comment" viewBox="0 0 12 12"><path d="M2 2.5h8a.8.8 0 0 1 .8.8v4.4a.8.8 0 0 1-.8.8H5.5L3.5 10.5V8.5H2a.8.8 0 0 1-.8-.8V3.3A.8.8 0 0 1 2 2.5z"/></symbol>
</svg>`;
}

// ---------- shell ----------
export function toolbar(o) {
  const seg = ["Table", "Model", "Both"].map((v) => `<span aria-pressed="${o.view === v}" title="${v} view (⌘${{ Table: 1, Model: 2, Both: 3 }[v]})">${icon(v === "Table" ? "table" : v === "Model" ? "cube" : "both")}${v}</span>`).join("");
  let run;
  if (o.run === "disabled") run = `<span class="btn disabled" id="runbtn">${icon("run")}Run</span>`;
  else if (o.run === "running") run = `<span class="btn primary runbtn" id="runbtn"><span class="prog" style="width:${o.runProgress || 46}%"></span>${icon("stop")}Running…</span>`;
  else run = `<span class="btn primary" id="runbtn">${icon("run")}Run</span>`;
  const worst = o.issues?.worst;
  const issuesBtn = `<span class="btn" title="Issues (⌘⇧I)">${worst ? `<svg class="ic" style="color:var(--issue-${worst})"><use href="#i-issues"/></svg>` : icon("issues")}Issues <span class="n">${o.issues?.count ?? 0}</span></span>`;
  // Undo and Redo: the one undo stack's pointer controls (V1.2 §5.2; the pointer rule). The tooltip names the operation.
  const undo = o.undo ? `<span class="btn icononly" title="Undo: ${esc(o.undo)} (⌘Z)">${icon("undo")}</span>` : `<span class="btn icononly disabled" title="Nothing to undo">${icon("undo")}</span>`;
  const redo = o.redo ? `<span class="btn icononly" title="Redo: ${esc(o.redo)} (⇧⌘Z)">${icon("redo")}</span>` : `<span class="btn icononly disabled" title="Nothing to redo">${icon("redo")}</span>`;
  // The two toggles of the right-hand panels, in the order the panels sit on the screen (V1.2 §5.2).
  const inspOff = o.view === "Table" ? "Table view opens rows in place" : o.view === "Model" ? "The inspector is always docked in Model view" : null;
  const insp = inspOff ? `<span class="btn disabled" id="insptoggle" aria-pressed="false" title="Inspector (⌘I) · ${inspOff}">${icon("inspector")}Inspector</span>`
    : `<span class="btn${o.inspectorOpen ? " latched" : ""}" id="insptoggle" aria-pressed="${!!o.inspectorOpen}" title="Inspector (⌘I)${o.inspectorTip ? " · " + esc(o.inspectorTip) : ""}">${icon("inspector")}Inspector</span>`;
  const agent = `<span class="btn${o.agentOpen ? " latched" : ""}" id="agenttoggle" aria-pressed="${!!o.agentOpen}" title="Agent (⌘⇧G)">${icon("agent")}Agent</span>`;
  return `<div class="toolbar">
  <div class="lights" aria-hidden="true"><i></i><i></i><i></i></div>
  <span class="wordmark">SWBPIPE</span>
  <span class="project">${esc(o.project)} <span class="st">· ${esc(o.saveState)}</span></span>
  <span class="undoredo">${undo}${redo}</span>
  <div class="seg" role="group" aria-label="View">${seg}</div>
  <div class="centre">${run}${issuesBtn}${insp}${agent}</div>
  <span class="grow"></span>
  <span class="combo unitsbtn" title="Display units">${esc(o.units || "SI")}</span>
  <span class="search" title="Search or command (⌘K)">${icon("search")}Search or command… <span class="k">⌘K</span></span>
</div>`;
}

export function rail(o) {
  const it = (name, ic, extra = "") => {
    const on = o.current === name; const off = o.disabled?.[name];
    return `<div class="it${on ? " on" : ""}${off ? " off" : ""}"${off ? ` title="${esc(off)}"` : ""}>${extra}${icon(ic, "s20")}${name}${o.caps?.[name] ? `<span class="cap${o.caps[name].stale ? " stale" : ""}${o.caps[name].failed ? " failed" : ""}">${esc(o.caps[name].text)}</span>` : ""}</div>`;
  };
  const cnt = o.issues?.count ? `<span class="cnt n${o.issues.worst === "warning" ? " warn" : ""}">${o.issues.count}</span>` : "";
  return `<nav class="rail" aria-label="Stages">${it("Model", "model")}${it("Loads", "loads")}${it("Results", "results")}${it("Review", "review")}<div class="spacer"></div><hr>${it("Libraries", "libraries")}${it("Rules", "rules")}${it("Issues", "issues", cnt)}</nav>`;
}

export function statusbar(o) {
  // The chips are label chips from the one table (V1.2 §2.3): none, one or two. The About control
  // opens About directly; it has no popover (V1.2 §5.2).
  const left = (o.chips || []).map((raw) => labelChip(raw, o.chipNote || "")).join("") + (o.leftText ? `<span>${o.leftText}</span>` : "");
  const worst = o.issues?.worst;
  return `<div class="statusbar">${left}<span role="button" title="Issues (⌘⇧I)">${worst ? `<svg class="ic s12" style="color:var(--issue-${worst})"><use href="#i-issues"/></svg> ` : ""}Issues <span class="n">${o.issues?.count ?? 0}</span></span><span>${o.selection || ""}</span><span class="grow"></span><span>${esc(o.units || "SI")}</span><span class="about" role="button" title="About SWBPIPE…">${icon("info")}</span></div>`;
}

export function agentStrip(o) {
  // The whole strip is a button that opens the column (V1.2 §5.4).
  return `<div class="agentstrip" role="button" title="Agent (⌘⇧G) · ${o.badge || 0} open proposal${o.badge === 1 ? "" : "s"} · opens the column">${icon("agent")}${o.badge ? `<span class="badge n">${o.badge}</span>` : ""}${o.working ? `<span class="work" title="Working…"></span>` : ""}<span class="vlabel">Agent</span></div>`;
}

export function agentColumn(o) {
  const tabs = [["Conversation", null], ["Proposals", o.counts?.proposals], ["Checks", o.counts?.checks], ["Accepted", o.counts?.accepted]].map(([t, c]) => `<span class="${o.tab === t ? "on" : ""}">${t}${c ? `<span class="n">${c}</span>` : ""}</span>`).join("");
  // The header's collapse control (V1.2 §5.4): the column collapses to its strip; it has no close.
  return `<aside class="agentcol"><div class="hd"><span class="ttl">Agent</span><span class="sec">${esc(o.state || "Idle")}</span><span style="flex:1"></span>${iconbtn("collapse", "Collapse to the strip (⌘⇧G)", { size: 16, cls: "plain" })}</div><div class="tabs">${tabs}</div><div class="body">${o.body}</div>${sendRow(o.draft)}</aside>`;
}
// The engineer's input with the Send control at its right end, disabled while the field is empty (V1.2 §5.4).
export function sendRow(text) {
  return `<div class="sendrow"><span class="${text ? "has" : ""}">${text ? esc(text) : "Ask the agent…"}</span>${iconbtn("send", "Send (⌘↩)", { size: 16, off: !text })}</div>`;
}

// ---------- tables ----------
const originGlyph = (origin, title) => origin && origin !== "entered" ? mark({ accepted: "accepted", propagated: "propagated", generated: "generated", imported: "imported", proposed: "proposed" }[origin], origin === "proposed" ? "var(--proposal-new)" : "var(--mark-origin)", title) : "";
const stateGlyph = (st, title) => ({
  checked: mark("checked", "var(--mark-checked)", title || "Checked by R. Tufts · 2026-09-17 14:02 · bound to this row's content · not a software status"),
  stale: mark("stale", "var(--mark-checkedStale)", title || "Checked by R. Tufts · 2026-09-17 14:02 · the row changed since · Check again or Clear"),
  blocking: mark("blocking", "var(--issue-blocking)", title), warning: mark("warning", "var(--issue-warning)", title), info: mark("info", "var(--issue-info)", title),
  comment: mark("comment", "var(--text-secondary)", title),
}[st] || "");
// The gutter's header cell is the select-all control (V1.2 §5.1); the specimen draws it as an empty header cell.
export const GUT_TH = `<th class="gut" role="button" title="Select all rows (⌘A)"></th>`;
export const gutter = (origin, state, titles = {}) => `<td class="gut"><div class="slots"><span class="slot">${originGlyph(origin, titles.origin)}</span><span class="slot">${stateGlyph(state, titles.state)}</span></div></td>`;

export function tabs(list, active, o = {}) {
  // o.collapse: the Model-view drawer's one control, the collapse chevron at the strip's right end (V1.2 §5.3).
  return `<div class="tabs">${list.map((t) => { const [name, cnt] = Array.isArray(t) ? t : [t, null]; return `<span class="tab${name === active ? " on" : ""}">${esc(name)}${cnt != null ? `<span class="cnt n">${cnt}</span>` : ""}</span>`; }).join("")}${o.collapse ? `<span class="grow"></span>${iconbtn("collapse", "Collapse the table drawer (⎋)", { size: 16, cls: "plain" })}` : ""}</div>`;
}

// The footer (V1.2 §5.1): the counts line. o.sel = { rows, unit, selected, readOnly, proposed } draws the
// selected count with its clear control and the selection group of five icon buttons; o.edit draws the edit
// chip in the same place. Either takes the place of the counts that follow (items without keep), which
// return when the selection clears; an item with keep is a standing line (the governing line, a proposal's
// standing, "1 not yet designed"), not a count.
export function tfoot(items, right = [], o = {}) {
  const item = (i) => `<span class="f"${i.color ? ` style="color:${i.color}"` : ""}><b${i.color ? ' style="color:inherit"' : ""}>${i.n}</b> ${i.label}</span>`;
  let lead = "", rest = items;
  if (o.sel || o.edit) {
    const s = o.sel || o.editSel;
    rest = items.filter((i) => i.keep);
    if (s) lead += `<span class="f selcount"><b>${s.rows} ${s.unit || "rows"}</b> · ${s.selected} selected ${iconbtn("close", "Clear the selection (⎋)", { cls: "tiny" })}</span>`;
    if (o.edit) lead += `<span class="editchip">${esc(o.edit)} <span class="btn compact" role="button" title="Apply (↩)">Apply</span><span class="btn compact text" role="button" title="Cancel (⎋)">Cancel</span></span>`;
    else {
      const ro = s.readOnly ? "This table is read-only" : null;
      lead += `<span class="selgroup">${iconbtn("addrow", ro ? `Insert row below · ${ro}` : "Insert row below (⌥↩)", { size: 16, off: !!ro })}${iconbtn("delrow", ro ? `Delete rows · ${ro}` : "Delete rows (⌫)", { size: 16, off: !!ro })}${iconbtn("copy", "Copy rows (⌘C)", { size: 16 })}${iconbtn("paste", ro ? `Paste · ${ro}` : "Paste (⌘V)", { size: 16, off: !!ro })}${iconbtn("check", s.checkTitle || "Check rows (⌘⇧K)", { size: 16 })}</span>`;
      if (s.proposed) lead += `<span class="chip outline act" role="button" title="Accept row (⌘⇧A)">${icon("check", "s12")}Accept row</span><span class="chip outline act" role="button" title="Reject row (⌘⇧R)">${icon("close", "s12")}Reject row</span>`;
    }
  }
  return `<div class="tfoot">${lead}${rest.map(item).join("")}<span class="grow"></span>${right.map((r) => `<span class="chip outline">${esc(r)}</span>`).join("")}</div>`;
}
// The expansion chevron at the right edge of the cell that owns an expansion (V1.2 §5.1).
export const xchev = (title, open = false) => `<svg class="ic s12 xchev" role="button"><title>${esc(title)}</title><use href="#i-${open ? "expanded" : "expand"}"/></svg>`;
// The caption line of a row expansion: the closing chevron, the caption, and the close control at its right end.
export const blockCap = (text, muted, what) => `<div class="cap"><span class="iconbtn plain tiny" role="button" title="Close ${esc(what)} (⎋)">${icon("expanded", "s12")}</span><span>${text}${muted ? ` <span class="muted">${muted}</span>` : ""}</span><span class="grow"></span>${iconbtn("close", "Close (⎋)")}</div>`;

export function layoutTable(o) {
  const rowsIn = M.rows.filter((r) => !o.nodes || o.nodes.includes(r.node));
  const rt = o.readThrough;
  const typeW = o.typeW || 100;
  const cols = [["gut", 32], ["Node", 56, "n"], ["From", 56, "n"], ["Type", typeW], ["DX", 72, "n", "mm"], ["DY", 72, "n", "mm"], ["DZ", 72, "n", "mm"], ["Section", 72], ["Material", 72], ["Load", 64]];
  if (rt) cols.push(["T1", 56, "n", "°C"], ["P1", 56, "n", "bar"], ["T2", 56, "n", "°C"], ["P2", 56, "n", "bar"]);
  cols.push(["Marks", 72, "marks"]);
  const width = cols.reduce((s, c) => s + c[1], 0);
  const colgroup = `<colgroup>${cols.map((c) => `<col style="width:${c[1]}px">`).join("")}</colgroup>`;
  const thead = `<thead><tr>${cols.map((c) => c[0] === "gut" ? GUT_TH : `<th class="${c[2] === "n" ? "n" : ""}">${c[0] === "Marks" ? "Marks" : esc(c[0])}${c[3] ? ` <span class="u">[${c[3]}]</span>` : ""}</th>`).join("")}</tr></thead>`;
  const restr = (n) => M.restraints.find((r) => r.node === n), loadOf = (n) => M.loads.filter((l) => l.node === n), nd = (n) => M.nodeData.find((x) => x.node === n);
  const body = [];
  let prev = null;
  for (const r of rowsIn) {
    const sel = o.selected === r.node; const prop = o.prop?.[r.node];
    const req = o.required?.[r.node] || [];
    const ls = M.loadSets.find((s) => s.name === r.load);
    const cell = (col, val, cls = "", title = "") => {
      const isFocus = o.focus && o.focus.node === r.node && o.focus.col === col;
      const isEdit = o.edit && o.edit.node === r.node && o.edit.col === col;
      if (isEdit) return `<td class="n edit"><div class="in">${esc(o.edit.value)}<span class="caret"></span><span class="unit">${o.edit.unit}</span></div></td>`;
      const pcell = prop?.[col];
      if (pcell) return `<td class="${cls} tick tprop" title="${esc(col)}: ${esc(pcell[0])} → ${esc(pcell[1])} · Accept this row">${pcell[2] ? `<span class="old">${esc(pcell[0])}</span>` : ""}<span class="new">${esc(pcell[1])}</span></td>`;
      return `<td class="${cls}${isFocus ? " focuscell" + (sel ? " selcell" : "") : ""}"${title ? ` title="${esc(title)}"` : ""}>${val}</td>`;
    };
    const propCell = (col, name) => {
      if (name == null || o.blank?.[r.node]?.includes(col.toLowerCase())) return `<td class="muted">—</td>`;
      if (req.includes(col.toLowerCase())) return `<td title="Required to solve · ${col}"><svg class="mk req"><use href="#m-required"/></svg></td>`;
      const propagated = !r.entered.includes(col.toLowerCase()) && r.from != null;
      const warn = o.warnCell?.[r.node] === col.toLowerCase();
      return cell(col, esc(name), propagated ? `tick${warn ? " twarn" : ""}` : (warn ? "tick twarn" : ""), warn ? "Provenance · Material source not recorded" : (propagated ? `Propagated from node ${r.from} (${col}) · typing over makes it entered` : ""));
    };
    const fromImplied = r.from === prev;
    const marksHtml = o.marks === false ? `<td class="marks"></td>` : `<td class="marks"><div class="slots">${restr(r.node) ? `<span class="slot${o.openMark?.node === r.node && o.openMark.slot === "restraint" ? " open" : ""}" title="Restraint · ${esc(restr(r.node).type)}${restr(r.node).gap != null && restr(r.node).gap > 0 ? `, gap ${restr(r.node).gap} mm` : ""}${restr(r.node).mu ? `, μ ${restr(r.node).mu.toFixed(2)}` : ""} · Open the row (⌘↩)">${mark("restraint")}</span>` : `<span class="slot"></span>`}${loadOf(r.node).length ? `<span class="slot${o.openMark?.node === r.node && o.openMark.slot === "load" ? " open" : ""}" title="Load · ${esc(loadOf(r.node).map((l) => `${l.kind} ${num(l.value, l.kind === "Force" ? 0 : 1)} ${l.unit} in ${l.direction} (${l.case})`).join("; "))} · Open the row (⌘↩)">${mark("load")}</span>` : `<span class="slot"></span>`}${nd(r.node) ? `<span class="slot" title="Node data · ${esc(nd(r.node).kind)} ${esc(nd(r.node).value)} · Open the row (⌘↩)">${mark("nodedata")}</span>` : `<span class="slot"></span>`}</div></td>`;
    const state = o.state?.[r.node];
    const stTitle = o.stateTitle?.[r.node];
    const origin = restr(r.node)?.origin === "accepted" && o.originsOn ? "accepted" : null;
    const tr = [`<tr class="${sel ? "sel " : ""}${prop ? "prop " : ""}">`];
    tr.push(gutter(prop ? "proposed" : origin, state, { origin: prop ? `Proposed by the agent · ${prop.id} · Accept (⌘⇧A) · Reject (⌘⇧R)` : (origin ? "Accepted from proposal P-09 by R. Tufts · 2026-09-17 11:40" : ""), state: stTitle }));
    tr.push(cell("Node", `<span class="n">${r.node}</span>`, "n"));
    tr.push(r.from == null ? `<td class="n muted">—</td>` : cell("From", String(r.from), `n${fromImplied ? " muted" : ""}`, fromImplied ? "" : "A branch: From names an earlier node"));
    tr.push(r.type == null ? `<td class="muted">—</td>` : cell("Type", esc(r.type) + (sel && !(o.edit && o.edit.node === r.node && o.edit.col === "Type") ? xchev("Element fields (⌘↩)") : ""), "haschev"));
    for (const [k, v] of [["DX", r.dx], ["DY", r.dy], ["DZ", r.dz]]) tr.push(cell(k, num(v, 0), `n${r.abs ? " tick" : ""}`, r.abs ? "Absolute coordinates" : ""));
    tr.push(propCell("Section", r.section)); tr.push(propCell("Material", r.material)); tr.push(propCell("Load", r.load));
    if (rt) { if (ls && r.load && !req.includes("load") && !o.blank?.[r.node]?.includes("load")) for (const [k, d] of [["T1", 0], ["P1", 2], ["T2", 0], ["P2", 2]]) tr.push(`<td class="n rt tick" title="Read through from load set ${ls.name} · typing over edits the set or forks a new one">${num(ls[k], d)}</td>`); else tr.push(`<td></td><td></td><td></td><td></td>`); }
    tr.push(marksHtml);
    tr.push("</tr>");
    body.push(tr.join(""));
    if (o.expansions?.[r.node]) body.push(`<tr class="expand"><td colspan="${cols.length}"><div class="block">${o.expansions[r.node]}</div></td></tr>`);
    prev = r.node;
  }
  if (o.draftRow) {
    const d = o.draftRow;
    body.push(`<tr class="draft"><td class="gut"></td><td class="n">${d.node}</td><td class="n">${d.from}</td><td>${esc(d.type)}</td><td class="n${d.axis === "X" ? " acc" : ""}">${d.axis === "X" ? num(d.len) : ""}</td><td class="n${d.axis === "Y" ? " acc" : ""}">${d.axis === "Y" ? num(d.len) : ""}</td><td class="n${d.axis === "Z" ? " acc" : ""}">${d.axis === "Z" ? num(d.len) : ""}</td><td class="tick">${esc(d.section)}</td><td class="tick">${esc(d.material)}</td><td class="tick">${esc(d.load)}</td>${rt ? "<td></td><td></td><td></td><td></td>" : ""}<td class="marks"></td></tr>`);
  }
  if (o.afterRows) body.push(`<tr class="expand band"><td colspan="${cols.length}">${o.afterRows}</td></tr>`);
  if (o.addRow !== false) body.push(`<tr class="addrow" role="button" title="Add a row with the next node number (⌥↩ inserts below the focused row)"><td class="gut"></td><td colspan="${cols.length - 1}">Add row</td></tr>`);
  if (o.nextRow) {
    body.push(`<tr class="draft"><td class="gut"></td><td class="n muted">${o.nextRow.node}</td><td class="n muted">${o.nextRow.from}</td><td class="muted">${o.nextRow.type || ""}</td><td class="n"></td><td class="n"></td><td class="n"></td><td></td><td></td><td></td>${rt ? "<td></td><td></td><td></td><td></td>" : ""}<td class="marks"></td></tr>`);
  }
  return { html: `<table class="ds${rowsIn.length > 8 && o.zebra !== false ? " zebra" : ""}" style="width:${width}px" aria-label="Layout table">${colgroup}${thead}<tbody>${body.join("")}</tbody></table>`, width };
}

export function joinedRestraints(node, o = {}) {
  const rs = M.restraints.filter((r) => r.node === node);
  const row = (r) => `<tr class="${o.sel ? "sel" : ""}">${gutter(r.origin === "accepted" ? "accepted" : null, null, { origin: `Accepted from proposal ${r.proposal} by R. Tufts · 2026-09-17 11:40` })}<td>${esc(r.type)}</td><td>${esc(r.direction)}</td><td class="n">${r.gap != null ? num(r.gap) : "—"}</td><td class="n">${r.mu != null ? r.mu.toFixed(2) : "—"}</td><td class="n${r.stiffness === "rigid" ? " muted" : ""}">${esc(r.stiffness)}</td><td class="n muted">${r.cnode ?? "—"}</td><td>${esc(r.tag)}</td></tr>`;
  return `${blockCap(`Restraints · node ${node}`, "· joined row", "the joined row")}<table class="ds" style="width:714px"><colgroup><col style="width:32px"><col style="width:120px"><col style="width:80px"><col style="width:80px"><col style="width:80px"><col style="width:124px"><col style="width:126px"><col style="width:72px"></colgroup><thead><tr><th class="gut"></th><th>Type</th><th>Direction</th><th class="n">Gap <span class="u">[mm]</span></th><th class="n">Friction μ</th><th class="n">Stiffness <span class="u">[N/mm]</span></th><th class="n">Connecting node</th><th>Tag</th></tr></thead><tbody>${rs.map(row).join("")}<tr class="addrow" role="button"><td class="gut"></td><td colspan="7">Add restraint…</td></tr></tbody></table>`;
}

export function joinedLoads(node) {
  const ls = M.loads.filter((l) => l.node === node);
  const row = (l) => `<tr>${gutter(null, null)}<td>${esc(l.kind)}</td><td>${esc(l.direction)}</td><td class="n">${num(l.value, l.kind === "Force" ? 0 : 1)}</td><td>${esc(l.unit)}</td><td>${esc(l.case)}</td><td class="sec">${esc(l.note)}</td></tr>`;
  return `${blockCap(`Loads · node ${node}`, "· joined row from the Loads stage", "the joined row")}<table class="ds" style="width:640px"><colgroup><col style="width:32px"><col style="width:120px"><col style="width:80px"><col style="width:88px"><col style="width:56px"><col style="width:64px"><col style="width:200px"></colgroup><thead><tr><th class="gut"></th><th>Kind</th><th>Direction</th><th class="n">Value</th><th>Unit</th><th>Case</th><th>Note</th></tr></thead><tbody>${ls.map(row).join("")}<tr class="addrow" role="button"><td class="gut"></td><td colspan="6">Add load…</td></tr></tbody></table>`;
}

export function restraintsTable(o) {
  const cols = [["gut", 32], ["Node", 56, "n"], ["Tag", 56], ["Type", o.prop ? 176 : 118], ["Direction", 76], ["Gap", 80, "n", "mm"], ["Friction μ", 80, "n"], ["Stiffness", 124, "n", "N/mm"], ["Connecting node", 126, "n"], ["Library", 148], ["Max variation", 128, "n", "%"], ["Note", o.noteW || 200]].filter((c) => !(o.noCnode && c[0] === "Connecting node") && !(o.noNote && c[0] === "Note"));
  const width = cols.reduce((s, c) => s + c[1], 0);
  const th = cols.map((c) => c[0] === "gut" ? GUT_TH : `<th class="${c[2] || ""}">${esc(c[0])}${c[3] ? ` <span class="u">[${c[3]}]</span>` : ""}</th>`).join("");
  const rows = M.restraints.map((r) => {
    const prop = o.prop?.[r.node]; const sel = o.selected === r.node;
    const pc = (col, val, cls = "") => { const p = prop?.fields?.[col]; if (p) return `<td class="${cls} tick tprop" title="${col}: ${esc(p[0])} → ${esc(p[1])} · Accept this row"><span class="old">${esc(p[0])}</span><span class="new">${esc(p[1])}</span></td>`; return `<td class="${cls}">${val}</td>`; };
    return `<tr class="${sel ? "sel " : ""}${prop ? "prop" : ""}">${gutter(prop ? "proposed" : (r.origin === "accepted" ? "accepted" : null), o.state?.[r.node], { origin: prop ? `Proposed by the agent · P-12 · Accept (⌘⇧A) · Reject (⌘⇧R)` : (r.origin === "accepted" ? `Accepted from proposal ${r.proposal} by R. Tufts · 2026-09-17 11:40` : "") })}<td class="n">${r.node}</td><td>${esc(r.tag)}</td>${pc("Type", esc(r.type))}<td>${esc(r.direction)}</td>${pc("Gap", r.gap != null ? num(r.gap) : "—", "n")}<td class="n">${r.mu != null ? r.mu.toFixed(2) : "—"}</td><td class="n${r.stiffness === "rigid" ? " muted" : ""}">${esc(r.stiffness)}</td>${o.noCnode ? "" : '<td class="n muted">—</td>'}${pc("Library", r.library ? esc(r.library) : "—", r.library ? "" : "muted")}${pc("Max variation", r.maxVar != null ? num(r.maxVar) : "—", "n")}${o.noNote ? "" : `<td class="sec">${esc(r.note)}</td>`}</tr>`;
  });
  return { html: `<table class="ds" style="width:${width}px" aria-label="Restraints"><colgroup>${cols.map((c) => `<col style="width:${c[1]}px">`).join("")}</colgroup><thead><tr>${th}</tr></thead><tbody>${rows.join("")}${o.noAdd ? "" : `<tr class="addrow" role="button"><td class="gut"></td><td colspan="${cols.length - 1}">Add row</td></tr>`}</tbody></table>`, width };
}

export function casesTable(o) {
  const cols = [["gut", 32], ["Case", 72], ["Expression", 140], ["Stress type", 96], ["Rule", 72], ["Origin", 256], ["Rule expression", 260]].filter((c) => !(o.noRuleExpr && c[0] === "Rule expression"));
  const width = cols.reduce((s, c) => s + c[1], 0);
  const ruleOf = (id) => M.rules.find((r) => r.id === id);
  const rows = M.cases.map((c) => {
    const sel = o.selected === c.name;
    const originText = c.origin === "generated" ? `Generated · ${c.who}` : c.origin === "edited" ? `Edited by ${c.who} · was ${c.was} (generated)` : `Authored by ${c.who}`;
    const rule = c.rule ? ruleOf(c.rule) : null;
    const exprCell = o.editing === c.name ? `<td class="n focuscell haschev${sel ? " selcell" : ""}">${esc(c.expr)}${xchev("Combination editor (⌘↩)", true)}</td>` : `<td class="n">${esc(c.expr)}</td>`;
    // The combination editor is a row expansion under the case (decision 14; V1.1 §5.1), never a popover.
    const expansion = o.editing === c.name && o.expansion ? `<tr class="expand"><td colspan="${cols.length}"><div class="block">${o.expansion}</div></td></tr>` : "";
    return `<tr class="${sel ? "sel" : ""}">${gutter(c.origin === "generated" ? "generated" : null, null, { origin: `Generated by rule pack ${c.who} · ${c.when}` })}<td>${esc(c.name)}</td>${exprCell}<td>${esc(c.type)}</td><td>${c.rule ? esc(c.rule) : '<span class="muted">—</span>'}</td><td class="sec" title="${esc(originText)} · ${esc(c.when)}">${esc(originText)}</td>${o.noRuleExpr ? "" : `<td>${rule ? `<span class="dispo">${esc(rule.expr)}</span>` : '<span class="muted">no rule</span>'}</td>`}</tr>${expansion}`;
  });
  return { html: `<table class="ds" style="width:${width}px" aria-label="Load cases"><colgroup>${cols.map((c) => `<col style="width:${c[1]}px">`).join("")}</colgroup><thead><tr>${cols.map((c) => c[0] === "gut" ? (o.noAdd ? `<th class="gut"></th>` : GUT_TH) : `<th>${esc(c[0])}</th>`).join("")}</tr></thead><tbody>${rows.join("")}${o.noAdd ? "" : `<tr class="addrow" role="button" title="Add a case (⌥↩ inserts below the focused row)"><td class="gut"></td><td colspan="${cols.length - 1}">Add case</td></tr>`}</tbody></table>`, width };
}

export const ratioCell = (r, extra = "") => `<td class="n${extra}">${r.toFixed(2)}<span class="bar"><i style="width:${Math.round(Math.min(1, r) * 100)}%;background:${scaleColor(r)}"></i></span></td>`;

export function stressTable(o) {
  const rows = o.rows;
  const cols = [["gut", 32], ["Node", 56, "n"], ["Element", 80, "n"], ["Case", 64], ["Stress", 96, "n", "MPa"], ["Allowable", 116, "n", "MPa"], ["Ratio", 120, "n"], ["Rule", 72], ["Pack", 48, "n"]];
  const width = cols.reduce((s, c) => s + c[1], 0);
  const th = cols.map((c) => c[0] === "gut" ? GUT_TH : `<th class="${c[2] || ""}"${c[0] === "Ratio" && o.menuOpen ? ' style="overflow:visible"' : ""}>${esc(c[0])}${c[3] ? ` <span class="u">[${c[3]}]</span>` : ""}${c[0] === "Ratio" ? `<svg class="mk sort" style="color:var(--text-secondary);vertical-align:-1px;margin-left:4px" aria-label="sorted descending"><use href="#m-sortdesc"/></svg>` : ""}${c[0] === "Ratio" && o.menuOpen ? o.menuOpen : ""}</th>`).join("");
  const body = rows.map((r) => `<tr class="${o.selected === r.node ? "sel" : ""}">${gutter(null, o.state?.[r.node], { state: o.stateTitle?.[r.node] })}<td class="n">${r.node}</td><td class="n">${r.element}</td><td>${r.case}</td><td class="n${o.stale ? " stale" : ""}">${r.stress.toFixed(1)}</td><td class="n${o.stale ? " stale" : ""}">${r.allowable.toFixed(1)}</td>${ratioCell(r.ratio, o.stale ? " stale" : "")}<td>${r.rule}</td><td class="n">${r.pack}</td></tr>`).join("");
  return { html: `<table class="ds zebra" style="width:${width}px" aria-label="Stresses"><colgroup>${cols.map((c) => `<col style="width:${c[1]}px">`).join("")}</colgroup><thead><tr>${th}</tr></thead><tbody>${body}</tbody></table>`, width };
}

export function hangerTable(o) {
  const cols = [["gut", 32], ["Node", 56, "n"], ["Tag", 56], ["Type", 108], ["Design load", 116, "n", "N"], ["Travel", 90, "n", "mm"], ["Library", 124], ["Size", 64], ["Rate", 98, "n", "N/mm"], ["Cold load", 102, "n", "N"], ["Hot load", 94, "n", "N"], ["Variation", 100, "n", "%"]];
  const width = cols.reduce((s, c) => s + c[1], 0);
  const th = cols.map((c) => c[0] === "gut" ? GUT_TH : `<th class="${c[2] || ""}">${esc(c[0])}${c[3] ? ` <span class="u">[${c[3]}]</span>` : ""}</th>`).join("");
  const st = o.stale ? " stale" : "";
  const rows = M.hangers.map((h) => `<tr class="${o.selected === h.node ? "sel" : ""}">${gutter(null, o.state?.[h.node], { state: o.stateTitle?.[h.node] })}<td class="n">${h.node}</td><td>${esc(h.tag)}</td><td>${esc(h.type)}</td><td class="n${st}">${num(h.designLoad)}</td><td class="n${st}">+${h.travel.toFixed(1)}</td><td>${esc(h.library)}</td><td class="n haschev${st}">${esc(h.size)}${o.selection === h.node ? xchev("Hanger selection (⌘↩)", true) : ""}</td><td class="n${st}">${num(h.rate)}</td><td class="n${st}">${num(h.coldLoad)}</td><td class="n${st}">${num(h.hotLoad)}</td><td class="n${st}">${h.variation.toFixed(1)}</td></tr>${o.selection === h.node ? `<tr class="expand"><td colspan="${cols.length}"><div class="block">${hangerSelection(h, { state: o.selectionState })}</div></td></tr>` : ""}`);
  if (o.pending) rows.push(`<tr class="${o.selected === 80 ? "sel" : ""}">${gutter("accepted", null, { origin: "Accepted from proposal P-12 by R. Tufts · 2026-09-17 16:31 · rationale on the record" })}<td class="n">80</td><td>RS-02</td><td>Variable spring</td><td class="n muted">—</td><td class="n muted">—</td><td>Vendor-A springs</td><td class="muted">—</td><td class="n muted">—</td><td class="n muted">—</td><td class="n muted">—</td><td class="n muted">—</td></tr>`);
  return { html: `<table class="ds" style="width:${width}px" aria-label="Hangers"><colgroup>${cols.map((c) => `<col style="width:${c[1]}px">`).join("")}</colgroup><thead><tr>${th}</tr></thead><tbody>${rows.join("")}</tbody></table>`, width };
}
// Hanger selection, the hanger row's expansion (V1.2 §5.1; ruling 5): the caption line, the content
// boundary's short variant once, verbatim, then the library's candidate sizes sorted by variation.
export function hangerSelection(h, o = {}) {
  const lib = M.hangerLibrary;
  const cand = M.hangerCandidates[h.node];
  const rows = cand.map((c) => { const off = c.reason ? ' class="offrow"' : ""; return `<tr${off}><td>${esc(c.size)}</td><td class="n">${num(c.rate)}</td><td class="n">${num(c.coldLoad)}</td><td class="n">${num(c.hotLoad)}</td><td class="n">${c.variation.toFixed(1)}</td><td class="sec">${esc(c.reason || c.range)}</td><td>${c.reason ? "" : c.size === h.size ? `<span class="muted">selected</span>` : `<span class="btn compact text" role="button">Select</span>`}</td></tr>`; }).join("");
  return `${blockCap(`Hanger selection · node ${h.node} · ${esc(lib.name)} · user import ${lib.imported}${o && o.state ? ` · ${o.state}` : ""}`, "", "the hanger selection")}<div class="boundary">no protected standards content; code-specific data is user-supplied</div><table class="ds" style="width:862px"><colgroup><col style="width:64px"><col style="width:98px"><col style="width:102px"><col style="width:94px"><col style="width:100px"><col style="width:332px"><col style="width:72px"></colgroup><thead><tr><th>Size</th><th class="n">Rate <span class="u">[N/mm]</span></th><th class="n">Cold load <span class="u">[N]</span></th><th class="n">Hot load <span class="u">[N]</span></th><th class="n">Variation <span class="u">[%]</span></th><th>Working range <span class="u">[N]</span></th><th></th></tr></thead><tbody>${rows}</tbody></table>`;
}

// ---------- results header ----------
export function resultsHead(o) {
  // The header band (V1.2 §5.1). The Run identity control's disclosure opens under the band and holds one
  // thing, the run identity line in the mono face: no sentence, no heading and no space for one (ruling 3).
  // The evidence chip is drawn only when the run record carries evidence and only for a Current run.
  const disc = o.discOpen ? `<div class="discwrap"><div class="disc"><div class="mono sec">${esc(o.identity)}</div></div></div>` : "";
  const caseSel = o.caseSel ? (o.envelope ? `<span class="combo disabled" title="Envelope is on: the governing case per row">Case: all</span>` : `<span class="combo">Case: ${esc(o.caseSel)}</span>`) : "";
  const evidence = o.evidence && (o.standing || "current") === "current" ? labelChip(o.evidence) : "";
  return `<div class="tblhead" data-standing="${o.standing || "current"}"><span class="name">${esc(o.name)}</span><span class="sec runtext" role="button" aria-haspopup="menu" title="Runs">${esc(o.run)}${icon("expanded", "s12")}</span>${caseSel}${o.envelope != null ? `<span class="switch${o.envelope ? " on" : ""}"><i></i>Envelope</span>` : ""}${evidence}${o.extra || ""}<span class="grow"></span>${o.controls || ""}<span class="btn compact text" role="button" title="Run identity" aria-expanded="${!!o.discOpen}">${icon("info")}Run identity</span></div>${o.band || ""}${disc}`;
}
// The two bands that can sit under a results header, never both at once (V1.2 §5.1, run standing).
export function staleBand(o) {
  return `<div class="staleband">${mark("stalerun", "var(--stale-ink)")}<span class="body"><b>Model changed since ${esc(o.run)}:</b> ${esc(o.change)}. The values below are from the model as solved.</span><span class="btn compact" role="button">Run again</span></div>`;
}
export function historicalBand(o = {}) {
  return `<div class="histband">${mark("historical", "var(--historical-ink)")}<span><span style="font-weight:500">Historical saved run</span> · Run a fresh solve to establish current results.</span>${iconbtn("info", "About historical runs", { cls: "right" + (o.open ? " on" : "") })}</div>`;
}
export function historicalPopover(o) {
  return `<div class="pop histpop" style="${o.style}"><div>Historical results cannot drive current overlays, rule checks, comparisons or report readiness.</div><div class="sec small">Recorded on ${esc(o.run)}</div><div class="chiprow">${o.statuses.map((raw) => labelChip(raw)).join("")}</div><div class="rawrow mono sec">${o.statuses.join(" · ")}</div></div>`;
}

// ---------- inspector ----------
const OPENROW = `<svg class="ic s12 openrow" role="button"><title>Open the row (⌘↩)</title><use href="#i-expand"/></svg>`;
export function inspector(o) {
  const r = (l, v, cls = "") => `<div class="r ${cls}"><span class="l">${esc(l)}</span><span class="v${cls.includes("left") ? " left" : ""}">${v}</span></div>`;
  const row = M.rows.find((x) => x.node === o.node);
  const c = M.coordinates()[o.node];
  const sec = M.sections.find((s) => s.name === row.section);
  const ls = M.loadSets.find((s) => s.name === row.load);
  const rs = o.noAttach ? [] : M.restraints.filter((x) => x.node === o.node), lds = o.noAttach ? [] : M.loads.filter((x) => x.node === o.node), nds = o.noAttach ? [] : M.nodeData.filter((x) => x.node === o.node);
  const parts = [];
  parts.push(`<div class="hd"><span class="ttl">Node ${o.node}</span><span class="sec">${row.from != null ? `Element ${row.from}–${row.node} · ${esc(row.type)}` : "Start node"}</span><span style="flex:1"></span>${o.slide || o.docked ? iconbtn("close", o.slide ? "Close the inspector (⎋) · Docked inspector needs a wider window" : "Close the inspector (⎋)", { size: 16, cls: "plain" }) : ""}</div>`);
  parts.push(`<div class="body">`);
  if (o.routing) parts.push(o.routing);
  parts.push(`<div class="sec-t">Geometry</div>${r("DX DY DZ", `<span class="n">${num(row.dx)} · ${num(row.dy)} · ${num(row.dz)}</span> <span class="muted">mm</span>`)}${r("X Y Z", `<span class="n">${num(c.x)} · ${num(c.y)} · ${num(c.z)}</span> <span class="muted">mm</span>`)}${row.bendR ? r("Bend radius", `<span class="n">${row.bendR}</span> <span class="muted">mm</span>`) : ""}`);
  parts.push(`<div class="sec-t">Section and material</div>${r("Section", `${esc(row.section)} <span class="sec">· ${sec.nominal} · ${sec.od} × ${sec.wall} mm</span>`)}${r("Material", `${esc(row.material)} <span class="sec">· <a>Libraries</a></span>`)}`);
  parts.push(`<div class="sec-t">Load set</div>${r("Set", `${esc(row.load)}${row.entered.includes("load") ? "" : ' <span class="sec">· propagated</span>'}`)}${r("T1 · P1", `<span class="n">${num(ls.T1)} °C · ${num(ls.P1, 2)} bar</span>`, "rt")}${r("T2 · P2", `<span class="n">${num(ls.T2)} °C · ${num(ls.P2, 2)} bar</span>`, "rt")}<div class="acts"><span class="btn compact">Edit set</span><span class="btn compact">Fork set</span></div>`);
  const items = [];
  for (const x of rs) { items.push(`<div class="r"><span class="l" style="width:auto">${mark("restraint", "var(--mark-attachment)")} ${esc(x.type)}</span><span class="v">${x.gap ? `gap ${x.gap} mm · ` : ""}${x.mu ? `μ ${x.mu.toFixed(2)} · ` : ""}${esc(x.tag)} ${OPENROW}</span></div>`); if (o.proposedRestraint && x.node === o.node) items.push(`<div class="r prop"><span class="l" style="width:auto;color:var(--proposal-new)">${mark("proposed", "var(--proposal-new)")} Proposed · P-12</span><span class="v" style="color:var(--proposal-new)">Variable spring · Vendor-A · 25 %</span></div>`); }
  for (const x of lds) items.push(`<div class="r"><span class="l" style="width:auto">${mark("load", "var(--mark-attachment)")} ${esc(x.kind)} ${esc(x.direction)}</span><span class="v">${num(x.value, x.kind === "Force" ? 0 : 1)} ${x.unit} · ${x.case} ${OPENROW}</span></div>`);
  for (const x of nds) items.push(`<div class="r"><span class="l" style="width:auto">${mark("nodedata", "var(--mark-attachment)")} ${esc(x.kind)}</span><span class="v">${esc(x.value)} ${OPENROW}</span></div>`);
  parts.push(`<div class="sec-t">Restraints, loads and node data</div>${items.length ? `<div class="sub">${items.join("")}</div>` : r("", '<span class="muted">none on this node</span>', "left")}<div class="acts"><span class="btn compact text">+ Restraint</span><span class="btn compact text">+ Load</span><span class="btn compact text">+ Node data</span></div>`);
  parts.push(`<div class="sec-t">Issues on this node</div>${r("", `<span class="muted">${o.issues || "none"}</span>`, "left")}`);
  parts.push(`<div class="sec-t">Origin and Checked</div>${r("Entered", `<span class="sec">R. Tufts · ${o.when || "2026-09-17 10:31"}</span>`)}${o.checked ? `<div class="r"><span class="l">Checked ${mark("checked", "var(--mark-checked)")}</span><span class="v"><span class="sec">R. Tufts · 2026-09-17 14:02</span></span></div>` : r("Checked", '<span class="muted">not checked</span>')}<div class="acts"><span class="btn compact" role="button" title="${o.checked ? "Clear check" : "Check"} (⌘⇧K)">${o.checked ? "Clear check" : "Check"}</span></div>`);
  parts.push(`<div class="sec-t">${icon("expand")} Provenance <span class="muted" style="font-weight:400">· section and material records</span></div>`);
  parts.push(`</div>`);
  return `<div class="insp${o.slide ? " slideover" : ""}${o.docked ? " docked" : ""}">${parts.join("")}</div>`;
}

// ---------- issues drawer ----------
export function issuesDrawer(issues, o = {}) {
  const groups = {};
  const shown = o.filter ? issues.filter((i) => i.cls === o.filter) : issues;
  for (const i of shown) (groups[i.cls] = groups[i.cls] || []).push(i);
  const order = ["Invalid model", "Blocks solve", "Blocks rule check", "Provenance", "Assumption", "Nonlinear", "Content boundary", "Note"];
  // The row's overflow (V1.2 §5.3, G-11): 26 px, never wraps; the glyph, the class words, the entity and, on the
  // selected row, the link keep their natural widths, the entity and the link right-aligned; the message takes
  // what is left and truncates with an ellipsis, its full text in the row's tooltip.
  const sevMark = (i) => mark(i.sev === "blocking" ? "blocking" : i.sev === "warning" ? "warning" : "info", `var(--issue-${i.sev})`);
  const g = order.filter((k) => groups[k]).map((k) => `<div class="grp">${sevMark(groups[k][0])}${esc(k)}<span class="muted n">${groups[k].length}</span></div>${groups[k].map((i) => `<div class="row${o.selected === i ? " sel" : ""}" role="button" title="${esc(i.msg)}">${sevMark(i)}<span class="cls">${esc(i.cls)}</span><span class="msg">${esc(i.msg)}</span><span class="ent">${esc(i.entity)}</span>${o.selected === i && o.link ? `<a>${esc(o.link)}</a>` : ""}</div>`).join("")}`).join("");
  return `<div class="issues"><div class="hd">Issues <span class="n">${issues.length}</span>${o.filter ? `<span class="chip outline on" role="button" title="Clear the filter">${esc(o.filter)} ${shown.length} ×</span><span class="chip outline">All classes ${issues.length}</span>` : `<span class="chip outline">All classes</span>`}<span style="flex:1"></span><span class="btn compact filterbtn" role="button" aria-haspopup="menu" title="Filter">Filter</span>${iconbtn("close", "Close (⎋)", { size: 16, cls: "plain" })}</div><div style="overflow:hidden;flex:1">${g}</div></div>`;
}

// ---------- canvas overlays ----------
export function hud(o = {}) {
  // One group of ten tools, Fit first at every canvas width (V1.2 §5.6, G-10). A tool that cannot act is
  // disabled with its reason in its tooltip; o.needsRun disables Deformation and Probe with "Needs a current run".
  const b = [["fit", "Fit (F)"], ["presets", "View"], ["section", "Section"], ["isolate", "Isolate selection (I)"], ["hide", "Hide selection (H)"], ["labels", "Node labels (L): Budget"], ["deform", "Deformation (D)"], ["probe", "Probe (P)"], ["route", "Route (R)"], ["restrain", "Add restraint (S)"]];
  const off = o.needsRun ? ["deform", "probe"] : (o.off || []);
  return `<div class="hud${o.wrap ? " wrap" : ""}" aria-label="Canvas HUD">${b.map(([k, t]) => `<span role="button" data-tool="${k}" title="${t}${off.includes(k) ? " · Needs a current run" : ""}" aria-pressed="${(o.pressed || []).includes(k)}"${off.includes(k) ? ' class="off" aria-disabled="true"' : ""}>${icon(k)}</span>`).join("")}</div>`;
}

export function legend(o) {
  // For a Stale or a Historical run the legend is a note card with no scale, no range and no marker (V1.2 §5.6).
  if (o.note) return `<div class="legend note" aria-label="Legend" style="${o.style || ""}"><div class="t"><span>${esc(o.note)}</span></div><div class="l"><span>No result colour on the current model</span></div></div>`;
  const top = Math.round((1 - o.value / o.range) * 150);
  return `<div class="legend" aria-label="Legend"><div class="t"><span>${esc(o.quantity)}</span><span class="sec" style="font-weight:400">${esc(o.caseName)}</span></div><div class="vbar"><div class="b"><i style="top:${top}px"></i></div><div class="ticks">${[1, 0.75, 0.5, 0.25, 0].map((t) => `<span class="n">${(t * o.range).toFixed(2)}</span>`).join("")}</div><div class="mark"><span style="top:${top - 7}px" class="n">${o.value.toFixed(2)} · node ${o.node}</span></div></div><div class="l"><span>Rule ${esc(o.rule)} · pack ${esc(o.pack)}</span></div><div class="l"><span>Range 0 – ${o.range.toFixed(2)}</span><span class="chip outline" role="button" style="height:18px">set</span></div><div class="l"><span><span class="unsolved"></span> Unsolved</span><span class="muted">none</span></div></div>`;
}

export function probe(o) {
  // A pinned card carries a close control at its top right (V1.2 §5.6); the footer's evidence chip is a label chip.
  return `<div class="probe" role="dialog" aria-label="Probe" style="left:${o.x}px;top:${o.y}px">${iconbtn("close", "Close (⎋)", { cls: "pclose" })}<div class="r"><span>Node</span><span class="n">${o.node}</span></div><div class="r"><span>Element</span><span class="n">${o.element}</span></div><div class="r"><span>Case</span><span>${o.case}</span></div><div class="r"><span>Stress</span><span class="n">${o.stress.toFixed(1)} MPa</span></div><div class="r"><span>Allowable</span><span class="n">${o.allowable.toFixed(1)} MPa</span></div><div class="r"><span>Ratio</span><span class="n">${o.ratio.toFixed(2)}<span class="bar"><i style="width:${Math.round(o.ratio * 100)}%;background:${scaleColor(o.ratio)}"></i></span></span></div><div class="r"><span>Rule</span><span>${o.rule}</span></div><div class="r"><span>Pack</span><span class="n">${o.pack}</span></div><div class="foot"><span>${o.run} · pinned</span>${o.evidence ? labelChip(o.evidence) : ""}</div></div>`;
}

// ---------- agent panel ----------
export function proposalCard(p, o = {}) {
  const decisions = o.decisions || {};
  const rowsHtml = p.rows.map((r, i) => {
    const d = decisions[i];
    // The decided row's receipt (V1.2 §5.4; Q-18): the done check, the first changed field with its new value,
    // the count of further fields, the time and Undo. One line; the new value truncates first; the old value is in
    // the tooltip and in the diff, which the chevron opens again.
    if (d === "accepted") return `<div class="difftitle"><span>${esc(r.table)} · node ${r.node}</span><span>row ${i + 1} of ${p.rows.length}</span></div><div class="accrow" title="${esc(`${r.fields[0][0]}: ${r.fields[0][1]} → ${r.fields[0][2]}`)}"><span class="iconbtn plain tiny" role="button" title="Show the diff">${icon("expand", "s12")}</span><span class="done">${icon("check", "s12")}</span><span class="what">${esc(r.fields[0][0])} <span class="new">${esc(r.fields[0][2])}</span></span>${r.fields.length > 1 ? `<span class="more n">+${r.fields.length - 1}</span>` : ""}<span class="state">accepted 16:31 · <a role="button">Undo</a></span></div>`;
    const acts = d === "rejected" ? `<span class="state">rejected · Undo</span>` : `<span class="iconbtn" title="Accept row (⌘⇧A)">${icon("check", "s12")}</span><span class="iconbtn" title="Reject row (⌘⇧R)">${icon("close", "s12")}</span>`;
    return `<div class="difftitle"><span>${esc(r.table)} · node ${r.node}</span><span>row ${i + 1} of ${p.rows.length}${d ? ` · ${d}` : " · pending"}</span></div><table class="diff"><colgroup><col style="width:76px"><col style="width:58px"><col><col style="width:54px"></colgroup><thead><tr><th>Field</th><th>Old</th><th>New</th><th></th></tr></thead><tbody>${r.fields.map((f, j) => `<tr><td>${esc(f[0])}</td><td class="old">${esc(f[1])}</td><td class="${f[2] === "TBD" ? "" : "new"}">${f[2] === "TBD" ? `<span class="tbd">${mark("warning")}TBD</span>` : esc(f[2])}</td><td class="acts">${j === 0 ? acts : ""}</td></tr>`).join("")}</tbody></table>`;
  }).join("");
  const remaining = p.rows.filter((_, i) => !decisions[i]).length;
  return `<div class="card prop"><p class="title">${esc(p.title)}</p>${p.asked ? `<div class="asked">Asked by ${esc(p.asked.who)} · ${p.asked.when}: “${esc(p.asked.text)}”</div>` : ""}<div class="meta"><b>Proposal ${p.id}</b> · draft until accepted · ${p.rows.length} rows in ${esc([...new Set(p.rows.map((r) => r.table))].join(", "))} · ${p.when.slice(11)}</div>${rowsHtml}<h5>Rationale</h5><p>${esc(p.rationale)}</p><h5>Constraints considered</h5><ul>${p.constraints.map((c) => `<li>${esc(c)}</li>`).join("")}</ul><h5>TBD</h5><ul>${p.tbd.map((c) => `<li><span class="tbd">${mark("warning")}${esc(c)}</span></li>`).join("")}</ul><div class="sec" style="margin-top:6px;font-size:12px">${esc(p.validation)}</div><div class="consequence">Accepting a row changes the model. Results from Run 03 stay readable, marked stale, until the next run.</div><div class="actions"><span class="btn compact text" role="button">Reject ${remaining === p.rows.length ? "proposal" : "remaining"}</span><span class="btn compact primary">Accept ${remaining === p.rows.length ? "all rows" : `remaining (${remaining} row${remaining === 1 ? "" : "s"})`}</span></div></div>`;
}

export function commentCard(c) {
  // The kind chip is one of the agent's class words (ruling 8) or Note for the engineer's own; a Draft's
  // action pair is Insert and Discard, every other card's Resolve or Reopen (V1.2 §5.5).
  const acts = c.kind === "Draft" ? `<span class="btn compact" role="button">Insert</span><span class="btn compact text" role="button">Discard</span>`
    : c.state === "resolved" ? `<span class="btn compact text" role="button">Reopen</span><span class="muted" style="font-size:11px;line-height:22px">resolved 09:41</span>` : `<span class="btn compact" role="button">Resolve</span>`;
  return `<div class="comment${c.state === "resolved" ? " resolved" : ""}"><div class="h">${kindChip(c.kind)}<a>${esc(c.ref)}</a><span class="who">${esc(c.who)} · ${c.when}</span></div>${esc(c.text)}<div class="act">${acts}</div></div>`;
}

export function kvpop(pairs, style = "") {
  return `<div class="pop" style="position:absolute;${style}"><div class="kv">${pairs.map(([k, v]) => `<dt>${k}</dt><dd>${v}</dd>`).join("")}</div></div>`;
}
export function tip(html, style) { return `<div class="tip" style="${style}">${html}</div>`; }
