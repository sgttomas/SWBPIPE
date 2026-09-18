// HTML renderers for the shell and the components, following the design system specimen.
import * as M from "./model.mjs";
import { esc, scaleColor } from "./canvas.mjs";

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
  <symbol id="m-comment" viewBox="0 0 12 12"><path d="M2 2.5h8a.8.8 0 0 1 .8.8v4.4a.8.8 0 0 1-.8.8H5.5L3.5 10.5V8.5H2a.8.8 0 0 1-.8-.8V3.3A.8.8 0 0 1 2 2.5z"/></symbol>
</svg>`;
}

// ---------- shell ----------
export function toolbar(o) {
  const seg = ["Table", "Model", "Both"].map((v) => `<span aria-pressed="${o.view === v}">${icon(v === "Table" ? "table" : v === "Model" ? "cube" : "both")}${v}</span>`).join("");
  let run;
  if (o.run === "disabled") run = `<span class="btn disabled" id="runbtn">${icon("run")}Run</span>`;
  else if (o.run === "running") run = `<span class="btn primary runbtn" id="runbtn"><span class="prog" style="width:${o.runProgress || 46}%"></span>${icon("stop")}Running…</span>`;
  else run = `<span class="btn primary" id="runbtn">${icon("run")}Run</span>`;
  const worst = o.issues?.worst;
  const issuesBtn = `<span class="btn">${worst ? `<svg class="ic" style="color:var(--issue-${worst})"><use href="#i-issues"/></svg>` : icon("issues")}Issues <span class="n">${o.issues?.count ?? 0}</span></span>`;
  return `<div class="toolbar">
  <div class="lights" aria-hidden="true"><i></i><i></i><i></i></div>
  <span class="wordmark">SWBPIPE</span>
  <span class="project">${esc(o.project)} <span class="st">· ${esc(o.saveState)}</span></span>
  <div class="seg" role="group" aria-label="View">${seg}</div>
  <div class="centre">${run}${issuesBtn}<span class="btn${o.agentOpen ? " pressed" : ""}"${o.agentOpen ? ' style="background:var(--selection-band);color:var(--accent-text);border-color:transparent"' : ""}>${icon("agent")}Agent</span></div>
  <span class="grow"></span>
  <span class="combo">${esc(o.units || "SI")}</span>
  <span class="search">${icon("search")}Search or command… <span class="k">⌘K</span></span>
</div>`;
}

export function rail(o) {
  const it = (name, ic, extra = "") => {
    const on = o.current === name; const off = o.disabled?.[name];
    return `<div class="it${on ? " on" : ""}${off ? " off" : ""}"${off ? ` title="${esc(off)}"` : ""}>${extra}${icon(ic, "s20")}${name}${o.caps?.[name] ? `<span class="cap${o.caps[name].stale ? " stale" : ""}">${esc(o.caps[name].text)}</span>` : ""}</div>`;
  };
  const cnt = o.issues?.count ? `<span class="cnt n${o.issues.worst === "warning" ? " warn" : ""}">${o.issues.count}</span>` : "";
  return `<nav class="rail" aria-label="Stages">${it("Model", "model")}${it("Loads", "loads")}${it("Results", "results")}${it("Review", "review")}<div class="spacer"></div><hr>${it("Libraries", "libraries")}${it("Rules", "rules")}${it("Issues", "issues", cnt)}</nav>`;
}

export function statusbar(o) {
  const left = (o.chips || []).map((c) => `<span class="chip ${c.cls}" title="${esc(c.title)}">${esc(c.label)}</span>`).join("") + (o.leftText ? `<span>${o.leftText}</span>` : "");
  const worst = o.issues?.worst;
  return `<div class="statusbar">${left}<span>${worst ? `<svg class="ic s12" style="color:var(--issue-${worst})"><use href="#i-issues"/></svg> ` : ""}Issues <span class="n">${o.issues?.count ?? 0}</span></span><span>${o.selection || ""}</span><span class="grow"></span><span>${esc(o.units || "SI")}</span>${icon("info")}</div>`;
}

export function agentStrip(o) {
  return `<div class="agentstrip" title="Agent · ${o.badge || 0} open proposal${o.badge === 1 ? "" : "s"} · ⌘⇧G">${icon("agent")}${o.badge ? `<span class="badge n">${o.badge}</span>` : ""}${o.working ? `<span class="work" title="Working…"></span>` : ""}<span class="vlabel">Agent</span></div>`;
}

export function agentColumn(o) {
  const tabs = [["Conversation", null], ["Proposals", o.counts?.proposals], ["Checks", o.counts?.checks], ["Accepted", o.counts?.accepted]].map(([t, c]) => `<span class="${o.tab === t ? "on" : ""}">${t}${c ? `<span class="n">${c}</span>` : ""}</span>`).join("");
  return `<aside class="agentcol"><div class="hd"><span class="ttl">Agent</span><span class="sec">${esc(o.state || "Idle")}</span><span style="flex:1"></span>${icon("close")}</div><div class="tabs">${tabs}</div><div class="body">${o.body}</div></aside>`;
}

// ---------- tables ----------
const originGlyph = (origin, title) => origin && origin !== "entered" ? mark({ accepted: "accepted", propagated: "propagated", generated: "generated", imported: "imported", proposed: "proposed" }[origin], origin === "proposed" ? "var(--proposal-new)" : "var(--mark-origin)", title) : "";
const stateGlyph = (st, title) => ({
  checked: mark("checked", "var(--mark-checked)", title || "Checked by R. Tufts · 2026-09-17 14:02 · bound to this row's content · not a software status"),
  stale: mark("stale", "var(--mark-checkedStale)", title || "Checked by R. Tufts · 2026-09-17 14:02 · the row changed since · Check again or Clear"),
  blocking: mark("blocking", "var(--issue-blocking)", title), warning: mark("warning", "var(--issue-warning)", title), info: mark("info", "var(--issue-info)", title),
  comment: mark("comment", "var(--text-secondary)", title),
}[st] || "");
export const gutter = (origin, state, titles = {}) => `<td class="gut"><div class="slots"><span class="slot">${originGlyph(origin, titles.origin)}</span><span class="slot">${stateGlyph(state, titles.state)}</span></div></td>`;

export function tabs(list, active) {
  return `<div class="tabs">${list.map((t) => { const [name, cnt] = Array.isArray(t) ? t : [t, null]; return `<span class="tab${name === active ? " on" : ""}">${esc(name)}${cnt != null ? `<span class="cnt n">${cnt}</span>` : ""}</span>`; }).join("")}</div>`;
}

export function tfoot(items, right = []) {
  return `<div class="tfoot">${items.map((i) => `<span class="f"${i.color ? ` style="color:${i.color}"` : ""}><b${i.color ? ' style="color:inherit"' : ""}>${i.n}</b> ${i.label}</span>`).join("")}<span class="grow"></span>${right.map((r) => `<span class="chip outline">${esc(r)}</span>`).join("")}</div>`;
}

export function layoutTable(o) {
  const rowsIn = M.rows.filter((r) => !o.nodes || o.nodes.includes(r.node));
  const rt = o.readThrough;
  const typeW = o.typeW || 100;
  const cols = [["gut", 32], ["Node", 56, "n"], ["From", 56, "n"], ["Type", typeW], ["DX", 72, "n", "mm"], ["DY", 72, "n", "mm"], ["DZ", 72, "n", "mm"], ["Section", 72], ["Material", 72], ["Load", 64]];
  if (rt) cols.push(["T1", 56, "n", "°C"], ["P1", 56, "n", "bar"], ["T2", 56, "n", "°C"], ["P2", 56, "n", "bar"]);
  cols.push(["Marks", 72, "marks"]);
  const width = cols.reduce((s, c) => s + c[1], 0);
  const colgroup = `<colgroup>${cols.map((c) => `<col style="width:${c[1]}px">`).join("")}</colgroup>`;
  const thead = `<thead><tr>${cols.map((c) => c[0] === "gut" ? `<th class="gut"></th>` : `<th class="${c[2] === "n" ? "n" : ""}">${c[0] === "Marks" ? "Marks" : esc(c[0])}${c[3] ? ` <span class="u">[${c[3]}]</span>` : ""}</th>`).join("")}</tr></thead>`;
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
    const marksHtml = o.marks === false ? `<td class="marks"></td>` : `<td class="marks"><div class="slots">${restr(r.node) ? `<span class="slot${o.openMark?.node === r.node && o.openMark.slot === "restraint" ? " open" : ""}" title="Restraint · ${esc(restr(r.node).type)}${restr(r.node).gap != null && restr(r.node).gap > 0 ? `, gap ${restr(r.node).gap} mm` : ""}${restr(r.node).mu ? `, μ ${restr(r.node).mu.toFixed(2)}` : ""} · ⌘↩ opens the row">${mark("restraint")}</span>` : `<span class="slot"></span>`}${loadOf(r.node).length ? `<span class="slot${o.openMark?.node === r.node && o.openMark.slot === "load" ? " open" : ""}" title="Load · ${esc(loadOf(r.node).map((l) => `${l.kind} ${num(l.value, l.kind === "Force" ? 0 : 1)} ${l.unit} in ${l.direction} (${l.case})`).join("; "))} · ⌘↩ opens the row">${mark("load")}</span>` : `<span class="slot"></span>`}${nd(r.node) ? `<span class="slot" title="Node data · ${esc(nd(r.node).kind)} ${esc(nd(r.node).value)} · ⌘↩ opens the row">${mark("nodedata")}</span>` : `<span class="slot"></span>`}</div></td>`;
    const state = o.state?.[r.node];
    const stTitle = o.stateTitle?.[r.node];
    const origin = restr(r.node)?.origin === "accepted" && o.originsOn ? "accepted" : null;
    const tr = [`<tr class="${sel ? "sel " : ""}${prop ? "prop " : ""}">`];
    tr.push(gutter(prop ? "proposed" : origin, state, { origin: prop ? `Proposed by the agent · ${prop.id} · Accept ⌘⇧A · Reject ⌘⇧R` : (origin ? "Accepted from proposal P-09 by R. Tufts · 2026-09-17 11:40" : ""), state: stTitle }));
    tr.push(cell("Node", `<span class="n">${r.node}</span>`, "n"));
    tr.push(r.from == null ? `<td class="n muted">—</td>` : cell("From", String(r.from), `n${fromImplied ? " muted" : ""}`, fromImplied ? "" : "A branch: From names an earlier node"));
    tr.push(r.type == null ? `<td class="muted">—</td>` : cell("Type", esc(r.type)));
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
  if (o.nextRow) {
    body.push(`<tr class="draft"><td class="gut"></td><td class="n muted">${o.nextRow.node}</td><td class="n muted">${o.nextRow.from}</td><td class="muted">${o.nextRow.type || ""}</td><td class="n"></td><td class="n"></td><td class="n"></td><td></td><td></td><td></td>${rt ? "<td></td><td></td><td></td><td></td>" : ""}<td class="marks"></td></tr>`);
  }
  return { html: `<table class="ds${rowsIn.length > 8 && o.zebra !== false ? " zebra" : ""}" style="width:${width}px" aria-label="Layout table">${colgroup}${thead}<tbody>${body.join("")}</tbody></table>`, width };
}

export function joinedRestraints(node, o = {}) {
  const rs = M.restraints.filter((r) => r.node === node);
  const row = (r) => `<tr class="${o.sel ? "sel" : ""}">${gutter(r.origin === "accepted" ? "accepted" : null, null, { origin: `Accepted from proposal ${r.proposal} by R. Tufts · 2026-09-17 11:40` })}<td>${esc(r.type)}</td><td>${esc(r.direction)}</td><td class="n">${r.gap != null ? num(r.gap) : "—"}</td><td class="n">${r.mu != null ? r.mu.toFixed(2) : "—"}</td><td class="n${r.stiffness === "rigid" ? " muted" : ""}">${esc(r.stiffness)}</td><td class="n muted">${r.cnode ?? "—"}</td><td>${esc(r.tag)}</td></tr>`;
  return `<div class="cap">${icon("expanded")} Restraints · node ${node} <span class="muted">· joined row · ⎋ closes</span></div><table class="ds" style="width:714px"><colgroup><col style="width:32px"><col style="width:120px"><col style="width:80px"><col style="width:80px"><col style="width:80px"><col style="width:124px"><col style="width:126px"><col style="width:72px"></colgroup><thead><tr><th class="gut"></th><th>Type</th><th>Direction</th><th class="n">Gap <span class="u">[mm]</span></th><th class="n">Friction μ</th><th class="n">Stiffness <span class="u">[N/mm]</span></th><th class="n">Connecting node</th><th>Tag</th></tr></thead><tbody>${rs.map(row).join("")}<tr><td class="gut"></td><td class="muted" colspan="7">Add restraint…</td></tr></tbody></table>`;
}

export function joinedLoads(node) {
  const ls = M.loads.filter((l) => l.node === node);
  const row = (l) => `<tr>${gutter(null, null)}<td>${esc(l.kind)}</td><td>${esc(l.direction)}</td><td class="n">${num(l.value, l.kind === "Force" ? 0 : 1)}</td><td>${esc(l.unit)}</td><td>${esc(l.case)}</td><td class="sec">${esc(l.note)}</td></tr>`;
  return `<div class="cap">${icon("expanded")} Loads · node ${node} <span class="muted">· joined row from the Loads stage · ⎋ closes</span></div><table class="ds" style="width:640px"><colgroup><col style="width:32px"><col style="width:120px"><col style="width:80px"><col style="width:88px"><col style="width:56px"><col style="width:64px"><col style="width:200px"></colgroup><thead><tr><th class="gut"></th><th>Kind</th><th>Direction</th><th class="n">Value</th><th>Unit</th><th>Case</th><th>Note</th></tr></thead><tbody>${ls.map(row).join("")}<tr><td class="gut"></td><td class="muted" colspan="6">Add load…</td></tr></tbody></table>`;
}

export function restraintsTable(o) {
  const cols = [["gut", 32], ["Node", 56, "n"], ["Tag", 56], ["Type", o.prop ? 176 : 118], ["Direction", 76], ["Gap", 80, "n", "mm"], ["Friction μ", 80, "n"], ["Stiffness", 124, "n", "N/mm"], ["Connecting node", 126, "n"], ["Library", 148], ["Max variation", 128, "n", "%"], ["Note", o.noteW || 200]].filter((c) => !(o.noCnode && c[0] === "Connecting node") && !(o.noNote && c[0] === "Note"));
  const width = cols.reduce((s, c) => s + c[1], 0);
  const th = cols.map((c) => c[0] === "gut" ? `<th class="gut"></th>` : `<th class="${c[2] || ""}">${esc(c[0])}${c[3] ? ` <span class="u">[${c[3]}]</span>` : ""}</th>`).join("");
  const rows = M.restraints.map((r) => {
    const prop = o.prop?.[r.node]; const sel = o.selected === r.node;
    const pc = (col, val, cls = "") => { const p = prop?.fields?.[col]; if (p) return `<td class="${cls} tick tprop" title="${col}: ${esc(p[0])} → ${esc(p[1])} · Accept this row"><span class="old">${esc(p[0])}</span><span class="new">${esc(p[1])}</span></td>`; return `<td class="${cls}">${val}</td>`; };
    return `<tr class="${sel ? "sel " : ""}${prop ? "prop" : ""}">${gutter(prop ? "proposed" : (r.origin === "accepted" ? "accepted" : null), o.state?.[r.node], { origin: prop ? `Proposed by the agent · P-12 · Accept ⌘⇧A · Reject ⌘⇧R` : (r.origin === "accepted" ? `Accepted from proposal ${r.proposal} by R. Tufts · 2026-09-17 11:40` : "") })}<td class="n">${r.node}</td><td>${esc(r.tag)}</td>${pc("Type", esc(r.type))}<td>${esc(r.direction)}</td>${pc("Gap", r.gap != null ? num(r.gap) : "—", "n")}<td class="n">${r.mu != null ? r.mu.toFixed(2) : "—"}</td><td class="n${r.stiffness === "rigid" ? " muted" : ""}">${esc(r.stiffness)}</td>${o.noCnode ? "" : '<td class="n muted">—</td>'}${pc("Library", r.library ? esc(r.library) : "—", r.library ? "" : "muted")}${pc("Max variation", r.maxVar != null ? num(r.maxVar) : "—", "n")}${o.noNote ? "" : `<td class="sec">${esc(r.note)}</td>`}</tr>`;
  });
  return { html: `<table class="ds" style="width:${width}px" aria-label="Restraints"><colgroup>${cols.map((c) => `<col style="width:${c[1]}px">`).join("")}</colgroup><thead><tr>${th}</tr></thead><tbody>${rows.join("")}${o.noAdd ? "" : `<tr><td class="gut"></td><td class="muted" colspan="${cols.length - 1}">Add restraint… <span class="muted">(or click a node in the canvas with Add restraint, S)</span></td></tr>`}</tbody></table>`, width };
}

export function casesTable(o) {
  const cols = [["gut", 32], ["Case", 72], ["Expression", 140], ["Stress type", 96], ["Rule", 72], ["Origin", 256], ["Rule expression", 260]].filter((c) => !(o.noRuleExpr && c[0] === "Rule expression"));
  const width = cols.reduce((s, c) => s + c[1], 0);
  const ruleOf = (id) => M.rules.find((r) => r.id === id);
  const rows = M.cases.map((c) => {
    const sel = o.selected === c.name;
    const originText = c.origin === "generated" ? `Generated · ${c.who}` : c.origin === "edited" ? `Edited by ${c.who} · was ${c.was} (generated)` : `Authored by ${c.who}`;
    const rule = c.rule ? ruleOf(c.rule) : null;
    const exprCell = o.editing === c.name ? `<td class="focuscell${sel ? " selcell" : ""}" style="overflow:visible"><span class="n">${esc(c.expr)}</span>${o.composer || ""}</td>` : `<td class="n">${esc(c.expr)}</td>`;
    return `<tr class="${sel ? "sel" : ""}">${gutter(c.origin === "generated" ? "generated" : null, null, { origin: `Generated by rule pack ${c.who} · ${c.when}` })}<td>${esc(c.name)}</td>${exprCell}<td>${esc(c.type)}</td><td>${c.rule ? esc(c.rule) : '<span class="muted">—</span>'}</td><td class="sec" title="${esc(originText)} · ${esc(c.when)}">${esc(originText)}</td>${o.noRuleExpr ? "" : `<td>${rule ? `<span class="dispo">${esc(rule.expr)}</span>` : '<span class="muted">no rule</span>'}</td>`}</tr>`;
  });
  return { html: `<table class="ds" style="width:${width}px" aria-label="Load cases"><colgroup>${cols.map((c) => `<col style="width:${c[1]}px">`).join("")}</colgroup><thead><tr>${cols.map((c) => c[0] === "gut" ? `<th class="gut"></th>` : `<th>${esc(c[0])}</th>`).join("")}</tr></thead><tbody>${rows.join("")}${o.noAdd ? "" : `<tr><td class="gut"></td><td class="muted" colspan="${cols.length - 1}">Add case… <span class="muted">⌥↩</span></td></tr>`}</tbody></table>`, width };
}

export const ratioCell = (r, extra = "") => `<td class="n${extra}">${r.toFixed(2)}<span class="bar"><i style="width:${Math.round(Math.min(1, r) * 100)}%;background:${scaleColor(r)}"></i></span></td>`;

export function stressTable(o) {
  const rows = o.rows;
  const cols = [["gut", 32], ["Node", 56, "n"], ["Element", 80, "n"], ["Case", 64], ["Stress", 96, "n", "MPa"], ["Allowable", 116, "n", "MPa"], ["Ratio", 120, "n"], ["Rule", 72], ["Pack", 48, "n"]];
  const width = cols.reduce((s, c) => s + c[1], 0);
  const th = cols.map((c) => c[0] === "gut" ? `<th class="gut"></th>` : `<th class="${c[2] || ""}"${c[0] === "Ratio" && o.menuOpen ? ' style="overflow:visible"' : ""}>${esc(c[0])}${c[3] ? ` <span class="u">[${c[3]}]</span>` : ""}${c[0] === "Ratio" ? `<svg class="mk sort" style="color:var(--text-secondary);vertical-align:-1px;margin-left:4px" aria-label="sorted descending"><use href="#m-sortdesc"/></svg>` : ""}${c[0] === "Ratio" && o.menuOpen ? o.menuOpen : ""}</th>`).join("");
  const body = rows.map((r) => `<tr class="${o.selected === r.node ? "sel" : ""}">${gutter(null, o.state?.[r.node], { state: o.stateTitle?.[r.node] })}<td class="n">${r.node}</td><td class="n">${r.element}</td><td>${r.case}</td><td class="n${o.stale ? " stale" : ""}">${r.stress.toFixed(1)}</td><td class="n${o.stale ? " stale" : ""}">${r.allowable.toFixed(1)}</td>${ratioCell(r.ratio, o.stale ? " stale" : "")}<td>${r.rule}</td><td class="n">${r.pack}</td></tr>`).join("");
  return { html: `<table class="ds zebra" style="width:${width}px" aria-label="Stresses"><colgroup>${cols.map((c) => `<col style="width:${c[1]}px">`).join("")}</colgroup><thead><tr>${th}</tr></thead><tbody>${body}</tbody></table>`, width };
}

export function hangerTable(o) {
  const cols = [["gut", 32], ["Node", 56, "n"], ["Tag", 56], ["Type", 108], ["Design load", 116, "n", "N"], ["Travel", 90, "n", "mm"], ["Library", 124], ["Size", 64], ["Rate", 98, "n", "N/mm"], ["Cold load", 102, "n", "N"], ["Hot load", 94, "n", "N"], ["Variation", 100, "n", "%"]];
  const width = cols.reduce((s, c) => s + c[1], 0);
  const th = cols.map((c) => c[0] === "gut" ? `<th class="gut"></th>` : `<th class="${c[2] || ""}">${esc(c[0])}${c[3] ? ` <span class="u">[${c[3]}]</span>` : ""}</th>`).join("");
  const st = o.stale ? " stale" : "";
  const rows = M.hangers.map((h) => `<tr class="${o.selected === h.node ? "sel" : ""}">${gutter(null, o.state?.[h.node], { state: o.stateTitle?.[h.node] })}<td class="n">${h.node}</td><td>${esc(h.tag)}</td><td>${esc(h.type)}</td><td class="n${st}">${num(h.designLoad)}</td><td class="n${st}">+${h.travel.toFixed(1)}</td><td>${esc(h.library)}</td><td class="n${st}">${esc(h.size)}</td><td class="n${st}">${num(h.rate)}</td><td class="n${st}">${num(h.coldLoad)}</td><td class="n${st}">${num(h.hotLoad)}</td><td class="n${st}">${h.variation.toFixed(1)}</td></tr>`);
  if (o.pending) rows.push(`<tr>${gutter("accepted", null, { origin: "Accepted from proposal P-12 by R. Tufts · 2026-09-17 16:31 · rationale on the record" })}<td class="n">80</td><td>RS-02</td><td>Variable spring</td><td class="n muted">—</td><td class="n muted">—</td><td>Vendor-A springs</td><td class="muted">—</td><td class="n muted">—</td><td class="n muted">—</td><td class="n muted">—</td><td class="n muted">—</td></tr>`);
  if (o.proposed) rows.push(`<tr class="prop">${gutter("proposed", null, { origin: "Proposed by the agent · P-12 · Accept ⌘⇧A · Reject ⌘⇧R" })}<td class="n">80</td><td>RS-02</td><td class="tick tprop"><span class="new">Variable spring</span></td><td class="n tick tprop"><span class="new">4 180</span></td><td class="tick tprop"><span class="tbd">${mark("warning", "var(--issue-warning)")}TBD</span></td><td class="tick tprop"><span class="new">Vendor-A springs</span></td><td class="muted">—</td><td class="n muted">—</td><td class="n muted">—</td><td class="n muted">—</td><td class="n muted">—</td></tr>`);
  return { html: `<table class="ds" style="width:${width}px" aria-label="Hangers"><colgroup>${cols.map((c) => `<col style="width:${c[1]}px">`).join("")}</colgroup><thead><tr>${th}</tr></thead><tbody>${rows.join("")}</tbody></table>`, width };
}

// ---------- results header ----------
export function resultsHead(o) {
  const disc = o.discOpen ? `<div style="padding:8px 8px 0"><div class="disc" style="max-width:640px">Results are engineering decision-support information. Acceptance, professional judgment, and any certification, sealing, or code-compliance determination remain with the responsible engineer and project authority.<div class="mono sec" style="margin-top:4px">${esc(o.identity)}</div></div></div>` : "";
  return `<div class="tblhead"><span class="name">${esc(o.name)}</span><span class="sec">${esc(o.run)}</span>${o.caseSel ? `<span class="combo">Case: ${esc(o.caseSel)}</span>` : ""}${o.envelope != null ? `<span class="switch${o.envelope ? " on" : ""}"><i></i>Envelope</span>` : ""}${o.evidence ? `<span class="chip solved" title="${o.evidence}">${o.evidence === "INTERNALLY_VERIFIED" ? "Internally verified" : "Prover correlated"}</span>` : ""}${o.extra || ""}<span class="grow"></span>${o.controls || ""}<span class="btn compact text" aria-expanded="${!!o.discOpen}">${icon("info")}Information</span></div>${disc}`;
}

// ---------- inspector ----------
export function inspector(o) {
  const r = (l, v, cls = "") => `<div class="r ${cls}"><span class="l">${esc(l)}</span><span class="v${cls.includes("left") ? " left" : ""}">${v}</span></div>`;
  const row = M.rows.find((x) => x.node === o.node);
  const c = M.coordinates()[o.node];
  const sec = M.sections.find((s) => s.name === row.section);
  const ls = M.loadSets.find((s) => s.name === row.load);
  const rs = o.noAttach ? [] : M.restraints.filter((x) => x.node === o.node), lds = o.noAttach ? [] : M.loads.filter((x) => x.node === o.node), nds = o.noAttach ? [] : M.nodeData.filter((x) => x.node === o.node);
  const parts = [];
  parts.push(`<div class="hd"><span class="ttl">Node ${o.node}</span><span class="sec">${row.from != null ? `Element ${row.from}–${row.node} · ${esc(row.type)}` : "Start node"}</span><span style="flex:1"></span>${o.slide ? icon("close") : ""}</div>`);
  parts.push(`<div class="body">`);
  if (o.routing) parts.push(o.routing);
  parts.push(`<div class="sec-t">Geometry</div>${r("DX DY DZ", `<span class="n">${num(row.dx)} · ${num(row.dy)} · ${num(row.dz)}</span> <span class="muted">mm</span>`)}${r("X Y Z", `<span class="n">${num(c.x)} · ${num(c.y)} · ${num(c.z)}</span> <span class="muted">mm</span>`)}${row.bendR ? r("Bend radius", `<span class="n">${row.bendR}</span> <span class="muted">mm</span>`) : ""}`);
  parts.push(`<div class="sec-t">Section and material</div>${r("Section", `${esc(row.section)} <span class="sec">· ${sec.nominal} · ${sec.od} × ${sec.wall} mm</span>`)}${r("Material", `${esc(row.material)} <span class="sec">· <a>Libraries</a></span>`)}`);
  parts.push(`<div class="sec-t">Load set</div>${r("Set", `${esc(row.load)}${row.entered.includes("load") ? "" : ' <span class="sec">· propagated</span>'}`)}${r("T1 · P1", `<span class="n">${num(ls.T1)} °C · ${num(ls.P1, 2)} bar</span>`, "rt")}${r("T2 · P2", `<span class="n">${num(ls.T2)} °C · ${num(ls.P2, 2)} bar</span>`, "rt")}<div class="acts"><span class="btn compact">Edit set</span><span class="btn compact">Fork set</span></div>`);
  const items = [];
  for (const x of rs) { items.push(`<div class="r"><span class="l" style="width:auto">${mark("restraint", "var(--mark-attachment)")} ${esc(x.type)}</span><span class="v">${x.gap ? `gap ${x.gap} mm · ` : ""}${x.mu ? `μ ${x.mu.toFixed(2)} · ` : ""}${esc(x.tag)} <span class="muted">⌘↩</span></span></div>`); if (o.proposedRestraint && x.node === o.node) items.push(`<div class="r prop"><span class="l" style="width:auto;color:var(--proposal-new)">${mark("proposed", "var(--proposal-new)")} Proposed · P-12</span><span class="v" style="color:var(--proposal-new)">Variable spring · Vendor-A · 25 %</span></div>`); }
  for (const x of lds) items.push(`<div class="r"><span class="l" style="width:auto">${mark("load", "var(--mark-attachment)")} ${esc(x.kind)} ${esc(x.direction)}</span><span class="v">${num(x.value, x.kind === "Force" ? 0 : 1)} ${x.unit} · ${x.case} <span class="muted">⌘↩</span></span></div>`);
  for (const x of nds) items.push(`<div class="r"><span class="l" style="width:auto">${mark("nodedata", "var(--mark-attachment)")} ${esc(x.kind)}</span><span class="v">${esc(x.value)} <span class="muted">⌘↩</span></span></div>`);
  parts.push(`<div class="sec-t">Restraints, loads and node data</div>${items.length ? `<div class="sub">${items.join("")}</div>` : r("", '<span class="muted">none on this node</span>', "left")}<div class="acts"><span class="btn compact text">+ Restraint</span><span class="btn compact text">+ Load</span><span class="btn compact text">+ Node data</span></div>`);
  parts.push(`<div class="sec-t">Issues on this node</div>${r("", `<span class="muted">${o.issues || "none"}</span>`, "left")}`);
  parts.push(`<div class="sec-t">Origin and Checked</div>${r("Entered", `<span class="sec">R. Tufts · ${o.when || "2026-09-17 10:31"}</span>`)}${o.checked ? `<div class="r"><span class="l">Checked ${mark("checked", "var(--mark-checked)")}</span><span class="v"><span class="sec">R. Tufts · 2026-09-17 14:02</span></span></div>` : r("Checked", '<span class="muted">not checked</span>')}<div class="acts"><span class="btn compact">${o.checked ? "Clear check" : "Check"} <span class="muted">⌘⇧K</span></span></div>`);
  parts.push(`<div class="sec-t">${icon("expand")} Provenance <span class="muted" style="font-weight:400">· section and material records</span></div>`);
  parts.push(`</div>`);
  return `<div class="insp${o.slide ? " slideover" : ""}">${parts.join("")}</div>`;
}

// ---------- issues drawer ----------
export function issuesDrawer(issues, o = {}) {
  const groups = {};
  const shown = o.filter ? issues.filter((i) => i.cls === o.filter) : issues;
  for (const i of shown) (groups[i.cls] = groups[i.cls] || []).push(i);
  const order = ["Invalid model", "Blocks solve", "Blocks rule check", "Provenance", "Assumption", "Nonlinear", "Content boundary", "Note"];
  const g = order.filter((k) => groups[k]).map((k) => `<div class="grp">${mark(groups[k][0].sev === "blocking" ? "blocking" : groups[k][0].sev === "warning" ? "warning" : "info", `var(--issue-${groups[k][0].sev})`)}${esc(k)}<span class="muted n">${groups[k].length}</span></div>${groups[k].map((i) => `<div class="row${o.selected === i ? " sel" : ""}"><span class="cls">${esc(i.cls)}</span><span>${esc(i.msg)}</span><span class="ent">${esc(i.entity)}</span></div>`).join("")}`).join("");
  return `<div class="issues"><div class="hd">Issues <span class="n">${issues.length}</span>${o.filter ? `<span class="chip outline" style="background:var(--selection-band);color:var(--accent-text);border-color:transparent">${esc(o.filter)} ${shown.length} ×</span><span class="chip outline">All classes ${issues.length}</span>` : `<span class="chip outline">All classes</span>`}<span class="chip outline">Unchecked rows</span><span style="flex:1"></span>${icon("close")}</div><div style="overflow:hidden;flex:1">${g}</div></div>`;
}

// ---------- canvas overlays ----------
export function hud(o = {}) {
  const b = [["fit", "Fit (F)"], ["presets", "View: Iso"], ["section", "Section"], ["isolate", "Isolate selection (I)"], ["hide", "Hide selection (H)"], ["labels", "Node labels (L)"], ["deform", "Deformation (D)"], ["probe", "Probe (P)"], ["route", "Route (R)"], ["restrain", "Add restraint (S)"]];
  return `<div class="hud" aria-label="Canvas HUD">${b.map(([k, t]) => `<span title="${t}" aria-pressed="${(o.pressed || []).includes(k)}"${(o.off || []).includes(k) ? ' class="off"' : ""}>${icon(k)}</span>`).join("")}</div>`;
}

export function legend(o) {
  const top = Math.round((1 - o.value / o.range) * 150);
  return `<div class="legend" aria-label="Legend"><div class="t"><span>${esc(o.quantity)}</span><span class="sec" style="font-weight:400">${esc(o.caseName)}</span></div><div class="vbar"><div class="b"><i style="top:${top}px"></i></div><div class="ticks">${[1, 0.75, 0.5, 0.25, 0].map((t) => `<span class="n">${(t * o.range).toFixed(2)}</span>`).join("")}</div><div class="mark"><span style="top:${top - 7}px" class="n">${o.value.toFixed(2)} · node ${o.node}</span></div></div><div class="l"><span>Rule ${esc(o.rule)} · pack ${esc(o.pack)}</span></div><div class="l"><span>Range 0 – ${o.range.toFixed(2)}</span><span class="chip outline" style="height:18px">set</span></div><div class="l"><span><span class="unsolved"></span> Unsolved</span><span class="muted">none</span></div></div>`;
}

export function probe(o) {
  return `<div class="probe" role="dialog" aria-label="Probe" style="left:${o.x}px;top:${o.y}px"><div class="r"><span>Node</span><span class="n">${o.node}</span></div><div class="r"><span>Element</span><span class="n">${o.element}</span></div><div class="r"><span>Case</span><span>${o.case}</span></div><div class="r"><span>Stress</span><span class="n">${o.stress.toFixed(1)} MPa</span></div><div class="r"><span>Allowable</span><span class="n">${o.allowable.toFixed(1)} MPa</span></div><div class="r"><span>Ratio</span><span class="n">${o.ratio.toFixed(2)}<span class="bar"><i style="width:${Math.round(o.ratio * 100)}%;background:${scaleColor(o.ratio)}"></i></span></span></div><div class="r"><span>Rule</span><span>${o.rule}</span></div><div class="r"><span>Pack</span><span class="n">${o.pack}</span></div><div class="foot"><span>${o.run} · pinned</span><span class="chip solved" title="INTERNALLY_VERIFIED">Internally verified</span></div></div>`;
}

// ---------- agent panel ----------
export function proposalCard(p, o = {}) {
  const decisions = o.decisions || {};
  const rowsHtml = p.rows.map((r, i) => {
    const d = decisions[i];
    if (d === "accepted") return `<div class="difftitle"><span>${esc(r.table)} · node ${r.node}</span><span>row ${i + 1} of ${p.rows.length}</span></div><div class="accrow"><span class="iconbtn done" title="Accepted 16:31">${icon("check", "s12")}</span><span>${esc(r.fields[0][0])} <span class="old">${esc(r.fields[0][1])}</span><span class="new">${esc(r.fields[0][2])}</span>${r.fields.length > 1 ? ` +${r.fields.length - 1}` : ""}</span><span class="state">accepted 16:31 · <a>Undo</a></span></div>`;
    const acts = d === "rejected" ? `<span class="state">rejected · Undo</span>` : `<span class="iconbtn" title="Accept row ⌘⇧A">${icon("check", "s12")}</span><span class="iconbtn" title="Reject row ⌘⇧R">${icon("close", "s12")}</span>`;
    return `<div class="difftitle"><span>${esc(r.table)} · node ${r.node}</span><span>row ${i + 1} of ${p.rows.length}${d ? ` · ${d}` : " · pending"}</span></div><table class="diff"><colgroup><col style="width:76px"><col style="width:58px"><col><col style="width:54px"></colgroup><thead><tr><th>Field</th><th>Old</th><th>New</th><th></th></tr></thead><tbody>${r.fields.map((f, j) => `<tr><td>${esc(f[0])}</td><td class="old">${esc(f[1])}</td><td class="${f[2] === "TBD" ? "" : "new"}">${f[2] === "TBD" ? `<span class="tbd">${mark("warning")}TBD</span>` : esc(f[2])}</td><td class="acts">${j === 0 ? acts : ""}</td></tr>`).join("")}</tbody></table>`;
  }).join("");
  const remaining = p.rows.filter((_, i) => !decisions[i]).length;
  return `<div class="card prop"><p class="title">${esc(p.title)}</p>${p.asked ? `<div class="asked">Asked by ${esc(p.asked.who)} · ${p.asked.when}: “${esc(p.asked.text)}”</div>` : ""}<div class="meta"><b>${p.id}</b> · draft until accepted · ${p.rows.length} rows · ${p.when.slice(11)}</div>${rowsHtml}<h5>Rationale</h5><p>${esc(p.rationale)}</p><h5>Constraints considered</h5><ul>${p.constraints.map((c) => `<li>${esc(c)}</li>`).join("")}</ul><h5>TBD</h5><ul>${p.tbd.map((c) => `<li><span class="tbd">${mark("warning")}${esc(c)}</span></li>`).join("")}</ul><div class="sec" style="margin-top:6px;font-size:12px">${esc(p.validation)}</div><div class="consequence">Accepting a row changes the model: current results are cleared and the run is kept as historical.</div><div class="actions"><span class="btn compact text">Reject ${remaining === p.rows.length ? "proposal" : "remaining"}</span><span class="btn compact primary">Accept ${remaining === p.rows.length ? "all rows" : `remaining (${remaining} row${remaining === 1 ? "" : "s"})`}</span></div></div>`;
}

export function commentCard(c) {
  return `<div class="comment${c.state === "resolved" ? " resolved" : ""}"><div class="h"><span class="chip outline">${esc(c.kind)}</span><a>${esc(c.ref)}</a><span class="who">${esc(c.who)} · ${c.when}</span></div>${esc(c.text)}<div class="act">${c.state === "resolved" ? `<span class="btn compact text">Reopen</span><span class="muted" style="font-size:11px;line-height:22px">resolved 09:41</span>` : `<span class="btn compact">Resolve</span>`}</div></div>`;
}

export function kvpop(pairs, style = "") {
  return `<div class="pop" style="position:absolute;${style}"><div class="kv">${pairs.map(([k, v]) => `<dt>${k}</dt><dd>${v}</dd>`).join("")}</div></div>`;
}
export function tip(html, style) { return `<div class="tip" style="${style}">${html}</div>`; }
