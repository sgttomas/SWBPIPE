// The frame specifications of the third pass (MOCKS-03): each returns the stage's inner HTML for one
// moment of the storyboard, composed from the shell, the components and the figure. The frames draw
// design system V1.2 (tokens.json 1.2) and the behaviour of UX specification V1.1, with the owner's
// rulings of 2026-09-18 on decision packet D-71 applied: one product name, no maturity sentence and no
// acceptance sentence anywhere, labels from the one table with their authority domain, the pointer rule.
import * as M from "./model.mjs";
import * as U from "./ui.mjs";
import { figure, esc } from "./canvas.mjs";
const { mark, icon, num } = U;

const MOCK_TEXT = "Mock rendering, not the engine's canvas";
const MOCK_LABEL = `<div class="mocklabel">${MOCK_TEXT} · schematic isometric, Y up</div>`;
const RESULT_TABS = ["Summary", "Stresses", "Displacements", "Restraint loads", "Restraint summary", "Forces and moments", "Hangers"];
const MODEL_TABS = (counts) => [["Layout", counts.layout], ["Restraints", counts.restraints], ["Node data", counts.nodeData]];
const LOADS_TABS = [["Cases", 6], ["Load sets", 2], ["Loads", 3], ["Wind", 0], ["Seismic", 1]];
const ID03 = "Run 03 · 2026-09-17 15:21 · sha256:4df0f798… · solver 0.2.0 · rule pack sample-rules 1.2 · sha256:9b1c4e02… · settings S-03";

function shell(o) {
  const agent = o.agent === "column" ? U.agentColumn(o.agentCol) : U.agentStrip(o.agentStrip || {});
  return `${U.toolbar(o.toolbar)}${U.rail(o.rail)}<div class="surfaces ${o.agent === "column" ? "column" : "strip"}">${o.surfaces}</div>${agent}${U.statusbar(o.status)}${o.overlays || ""}`;
}
// The status bar's chips are raw tokens; ui.mjs draws each from the one table with its domain (V1.2 §2.3).
const CH = { incomplete: "MODEL_INCOMPLETE", solved: "MECHANICS_SOLVED", rules: "USER_RULE_CHECKED", review: "HUMAN_REVIEW_REQUIRED" };
const noRun = { Results: "No run yet", Review: "No run yet" };
const canvasBox = (w, h, inner) => `<div class="canvas" style="width:${w}px;height:${h}px;position:relative;flex:none">${inner}</div>`;
// The toast (V1.2 §5.6, G-8): 320 px, an optional icon, the message, at most one action, a close control;
// at the bottom-right corner of the surfaces, 8 px above the status bar and 8 px left of the agent strip or column.
const toast = (ic, msg, act, right) => `<div class="toast" role="status" style="right:${right}px;bottom:32px">${icon(ic)}<span class="msg">${esc(msg)}</span><span class="act" role="button">${esc(act)}</span>${U.iconbtn("close", "Dismiss")}</div>`;

// ---------- state 1 ----------
function s1() {
  const issues = { count: 3, worst: "blocking" };
  // A new project opens the Model stage in Both view (the owner's direction on C-22; UX_SPEC V1.2 §10.1):
  // the layout table at 737 px with read-through off, the canvas at 603 px with the grid, the triad and node 10 as a point.
  const tbl = U.layoutTable({ nodes: [10, 20], readThrough: false, blank: { 10: ["section", "material", "load"] }, required: { 20: ["section", "material", "load"] }, edit: { node: 20, col: "DX", value: "3000", unit: "mm" }, selected: 20, zebra: false, marks: false });
  // The hint line names controls first and keys after them (UX_SPEC_V1 §3.2; V1.2 §7.6).
  const hint = `Click a cell to type · Add row adds row 30 (↩ on the last cell) · Paste in the footer pastes rows (⌘V) · Section, Material and Load propagate from the row above once entered · the asterisk marks what the solve needs`;
  const tables = `<div class="tables">${U.tabs(MODEL_TABS({ layout: 2, restraints: 0, nodeData: 0 }), "Layout")}<div class="tblscroll">${tbl.html}<div class="emptyhint">${hint}</div></div>${U.tfoot([{ n: 1, label: "element" }, { n: 3, label: "issues", color: "var(--issue-blocking)", keep: true }], ["Read-through off", "Origins"], { editSel: { rows: 2, selected: 1 }, edit: "Editing DX · node 20" })}</div>`;
  const fig = figure({ id: "s1", w: 603, h: 828, view: { A: 200, E: 28 }, nodes: [10], fitAll: true, restraints: false, loads: false, nodeData: false, margin: { top: 110, right: 40, bottom: 80, left: 40 } });
  const canvas = `<div class="canvas" style="width:603px;position:relative">${fig.svg}${U.hud({ needsRun: true })}${MOCK_LABEL}</div>`;
  const surfaces = `<div class="bothview">${tables}${canvas}</div>`;
  // The Run tooltip hangs under the button, right-aligned to it, so that it lies over the table and not over the HUD.
  const overlays = U.tip(`Run is unavailable — Section missing at node 20, Material missing at node 20`, "right:685px;top:42px");
  return shell({
    toolbar: { project: "Loop 4 header", saveState: "not saved", view: "Both", run: "disabled", issues },
    rail: { current: "Model", disabled: noRun, issues },
    surfaces,
    status: { chips: [CH.incomplete], issues, selection: "Node 20 · DX" },
    overlays,
  });
}

// ---------- state 2 ----------
function s2() {
  const issues = { count: 1, worst: "blocking" };
  const fig = figure({ id: "s2", w: 1000, h: 548, nodes: [10, 20, 30, 40], draft: { from: 40, axis: "Y", len: 1500, next: 50 }, restraints: false, loads: false, nodeData: false, selection: { node: 40 }, margin: { top: 70, right: 80, bottom: 70, left: 80 } });
  const lf = fig.overlays.lengthField;
  // The compass's pointer controls (V1.2 §5.6): Reverse, Place and Cancel to the right of the length field.
  const lenrow = `<div class="lenrow" style="left:${Math.round(lf.x)}px;top:${Math.round(lf.y)}px"><div class="lenfield"><span class="n">1500</span><span class="caret"></span><span class="u">mm</span></div>${U.iconbtn("reverse", "Reverse the axis (−)", { cls: "raised" })}${U.iconbtn("check", "Place node 50 (↩)", { cls: "raised" })}${U.iconbtn("close", "Cancel the route (⎋)", { cls: "raised" })}</div>`;
  const canvas = canvasBox(1000, 548, `${fig.svg}${U.hud({ pressed: ["route"], needsRun: true })}${lenrow}<div class="keys" style="left:8px;top:44px">Route · ↩ places 50 · ⇥ next axis · − reverses · ⎋ cancels</div>${MOCK_LABEL}`);
  const tbl = U.layoutTable({ nodes: [10, 20, 30, 40], readThrough: false, selected: 40, draftRow: { node: 50, from: 40, type: "Pipe", axis: "Y", len: 1500, section: "P1", material: "CS-A", load: "OP1" }, zebra: false, marks: false });
  const drawer = `<div class="drawer">${U.tabs(MODEL_TABS({ layout: 4, restraints: 0, nodeData: 0 }), "Layout", { collapse: true })}<div class="tblscroll">${tbl.html}</div>${U.tfoot([{ n: 3, label: "elements" }, { n: 1, label: "bend" }, { n: 1, label: "draft row", keep: true }], ["Read-through off", "Origins"], { sel: { rows: 4, selected: 1 } })}</div>`;
  // The routing block's three compact buttons read their names; the accelerators are in the tooltips (V1.2 §5.3).
  const routing = `<div class="sec-t">Routing from node 40</div><div class="r"><span class="l">Next node</span><span class="v">50</span></div><div class="r"><span class="l">Axis</span><span class="v">+Y</span></div><div class="r acc"><span class="l">Length</span><span class="v">1500 mm</span></div><div class="r"><span class="l">Carries</span><span class="v">P1 · CS-A · OP1 <span class="muted">· propagated</span></span></div><div class="acts"><span class="btn compact" role="button" title="Place node 50 (↩)">Place node 50</span><span class="btn compact" role="button" title="Bend at 40… (B)">Bend at 40…</span><span class="btn compact text" role="button" title="Cancel (⎋)">Cancel</span></div>`;
  const surfaces = `<div class="modelview"><div class="left">${canvas}${drawer}</div><div class="inspector">${U.inspector({ node: 40, routing, noAttach: true, issues: "none", when: "2026-09-17 10:34" })}</div></div>`;
  return shell({
    toolbar: { project: "Loop 4 header", saveState: "edited", view: "Model", run: "disabled", issues, undo: "place node 40" },
    rail: { current: "Model", disabled: noRun, issues },
    surfaces,
    status: { chips: [CH.incomplete], issues, selection: "Routing from node 40 · +Y 1500 mm" },
  });
}

// ---------- state 3 ----------
function s3() {
  const issues = { count: 0, worst: null };
  const nodes = M.rows.filter((r) => r.node <= 130).map((r) => r.node);
  const map = [["Node", "Node"], ["From", "From"], ["Type", "Type"], ["DX", "DX"], ["DY", "DY"], ["DZ", "DZ"], ["Sect", "Section"], ["Matl", "Material"], ["Load", "Load"], ["Sched", null], ["Note", null]];
  const maprow = map.map(([src, tgt]) => `<span class="m${tgt ? "" : " ign"}"><span class="src">${src}</span><span class="arrow">→</span><span class="sel">${tgt || "Ignore"}</span></span>`).join("");
  const preview = `<table class="ds" style="width:640px"><colgroup><col style="width:32px"><col style="width:56px"><col style="width:56px"><col style="width:100px"><col style="width:72px"><col style="width:72px"><col style="width:72px"><col style="width:72px"><col style="width:72px"><col style="width:64px"></colgroup><thead><tr><th class="gut"></th><th class="n">Node</th><th class="n">From</th><th>Type</th><th class="n">DX <span class="u">[mm]</span></th><th class="n">DY <span class="u">[mm]</span></th><th class="n">DZ <span class="u">[mm]</span></th><th>Section</th><th>Material</th><th>Load</th></tr></thead><tbody>
<tr class="draft"><td class="gut"></td><td class="n">140</td><td class="n">70</td><td>Pipe</td><td class="n">2000</td><td class="n">0</td><td class="n">0</td><td>P3</td><td>CS-B</td><td>OP2</td></tr>
<tr class="draft"><td class="gut"></td><td class="n">150</td><td class="n">140</td><td>Bend</td><td class="n">1000</td><td class="n">0</td><td class="n">0</td><td class="tick">P3</td><td class="tick">CS-B</td><td class="tick">OP2</td></tr>
<tr class="draft"><td class="gut"></td><td class="n">160</td><td class="n">150</td><td>Pipe</td><td class="n">0</td><td class="n">−3000</td><td class="n">0</td><td class="tick">P3</td><td class="tick">CS-B</td><td class="tick">OP2</td></tr></tbody></table>`;
  // The band's two buttons read their names; ↩ and ⎋ accelerate them and are in the tooltips (V1.2 §5.1, §9 row 66).
  const band = `<div class="pasteband" style="width:948px"><div class="hd"><b>Paste</b><span class="sec">3 rows from the clipboard · after node 130 · 2 columns ignored · node IDs taken from the source</span><span class="grow"></span><span class="btn compact text" role="button" title="Cancel (⎋)">Cancel</span><span class="btn compact primary" role="button" title="Paste 3 rows (↩)">Paste 3 rows</span></div><div class="maprow">${maprow}</div><div class="sec" style="font-size:12px;margin:8px 0 4px">Preview in the layout grammar · row 140 is a branch (From 70) · Section, Material and Load propagate where the source left them blank</div>${preview}</div>`;
  const tbl = U.layoutTable({ nodes, readThrough: true, edit: { node: 130, col: "DY", value: "−1500", unit: "mm" }, selected: 130, marks: false, zebra: true, afterRows: band });
  const surfaces = `<div class="tableview"><div class="region">${U.tabs(MODEL_TABS({ layout: 13, restraints: 0, nodeData: 0 }), "Layout")}<div class="tblscroll">${tbl.html}</div>${U.tfoot([{ n: 12, label: "elements" }, { n: 3, label: "bends" }, { n: 1, label: "valve" }, { n: 1, label: "reducer" }, { n: M.rows.filter((r) => r.from != null && r.node <= 130).reduce((t, r) => t + 3 - r.entered.length, 0), label: "propagated cells" }, { n: 0, label: "issues" }], ["Read-through on", "Origins"], { editSel: { rows: 13, selected: 1 }, edit: "Editing DY · node 130" })}</div></div>`;
  return shell({
    toolbar: { project: "Loop 4 header", saveState: "edited", view: "Table", run: "disabled", issues, undo: "type DZ at node 120" },
    rail: { current: "Model", disabled: noRun, issues },
    surfaces,
    status: { chips: [CH.incomplete], issues, selection: "Node 130 · DY" },
  });
}

// ---------- state 4 ----------
// Both view with the inspector docked on the canvas's right edge (decision 1; V1.2 §0): the table keeps 737,
// the inspector takes 300 from the canvas pane and the canvas draws in the remaining 303. Three moments:
//   docked    the camera is fitted (the engineer never moved it), so docking refits by itself (Q-20);
//   column    the agent column was open (canvas 470) and the engineer had placed the camera there; docking would
//             have left 170 px, under the 220 px minimum, so the column collapsed to its strip first, with the
//             toast, and the placed camera keeps its scale and centre (decision 1; UX_SPEC_V1 §10.9);
//   slideover the other order (Q-15): the inspector was docked and the agent column was opened; the column
//             opens and the inspector becomes the slide-over over the 470 px canvas while the column is open.
function s4both(mode = "docked") {
  const issues = { count: 1, worst: "warning" };
  const slide = mode === "slideover";
  const tbl = U.layoutTable({ readThrough: false, typeW: 96, selected: 20, state: { 10: "checked", 20: "checked", 30: "stale" }, warnCell: { 140: "material" }, originsOn: true, zebra: true, openMark: slide ? null : { node: 20, slot: "restraint" } });
  const tables = `<div class="tables">${U.tabs(MODEL_TABS({ layout: 16, restraints: 7, nodeData: 2 }), "Layout")}<div class="tblscroll">${tbl.html}</div>${U.tfoot([{ n: 15, label: "elements" }, { n: 7, label: "restraints" }, { n: 3, label: "loads" }, { n: 2, label: "node data" }, { n: 2, label: "checked" }, { n: 1, label: "stale", color: "var(--mark-checkedStale)" }], slide ? ["Origins"] : ["Read-through off", "Origins"], { sel: { rows: 16, selected: 1, checkTitle: "Clear check (⌘⇧K)" } })}</div>`;
  const H = 828, W = slide ? 470 : 303;
  const figOpts = { id: "s4", w: W, h: H, view: { A: 200, E: 28 }, selection: { node: 20 }, plates: W < 400 ? "selected" : "all", margin: { top: 90, right: 40, bottom: 90, left: 40 } };
  let fig, camera = "fitted", panX = 0;
  if (mode === "column") {
    camera = "placed at 470";
    const base = figure({ ...figOpts, camera: { w: 470, h: H } });
    const p20 = base.P[20];
    if (p20.x > W - 72) panX = W - 72 - p20.x; else if (p20.x < 72) panX = 72 - p20.x;
    fig = panX ? figure({ ...figOpts, camera: { w: 470, h: H, pan: { x: panX, y: 0 } } }) : base;
  } else fig = figure(figOpts);
  const inspector = U.inspector({ node: 20, docked: !slide, slide, checked: true, issues: "none", when: "2026-09-17 10:31" });
  // The narrow canvas stacks its furniture at the left edge (V1.2 §5.6, G-10): the two-row HUD, then the mock line.
  const canvas = slide
    ? `<div class="canvas" style="position:relative" data-camera="${camera}" data-pan="0">${fig.svg}${U.hud({ needsRun: true })}<div class="mocklabel narrow" style="top:44px">${MOCK_TEXT}</div>${inspector}</div>`
    : `<div class="canvas" style="width:${W}px;position:relative" data-camera="${camera}" data-pan="${Math.round(panX)}">${fig.svg}${U.hud({ needsRun: true, wrap: true })}<div class="mocklabel narrow">${MOCK_TEXT}</div></div>`;
  const surfaces = `<div class="bothview">${tables}${canvas}${slide ? "" : `<div class="inspector">${inspector}</div>`}</div>`;
  // The hover card on the open restraint mark, in the marks vocabulary's words (V1.2 §4).
  // The Marks column lies beyond the 574 px pane's right edge in the slide-over frame, so its tooltip is not drawn there.
  let overlays = slide ? "" : U.tip(`<span class="t">Restraint</span> · +Y, gap 3 mm, μ 0.30 · Open the row (⌘↩)`, "left:604px;top:158px");
  if (mode === "column") overlays += toast("agent", "Agent column collapsed to its strip: the docked inspector needs the width.", "Reopen", 52);
  const conversation = `<div class="msgs"><div class="msg me"><div class="who">R. Tufts · 10:58</div>Read the restraint at node 20 back to me against the survey note.</div><div class="msg"><div class="who">Agent · 10:58</div>The <a>Restraints row, node 20</a> reads +Y, gap 3 mm, μ 0.30, tag RS-01, entered 2026-09-17 10:31. The survey note is not in the record.<div class="tbdlist"><span class="tbd">${mark("warning")}Survey note for the gap at 20: TBD</span></div></div></div>`;
  return shell({
    agent: slide ? "column" : "strip",
    agentCol: { tab: "Conversation", counts: { proposals: 0, checks: 0, accepted: 1 }, state: "Idle", body: conversation },
    toolbar: { project: "Loop 4 header", saveState: "saved", view: "Both", run: "enabled", issues, inspectorOpen: true, inspectorTip: slide ? "Docked inspector needs a wider window" : "", agentOpen: slide, undo: "gap at node 20" },
    rail: { current: "Model", disabled: noRun, issues },
    surfaces,
    status: { chips: [], issues, selection: "Node 20 · 1 row" },
    overlays,
  });
}

function s4table() {
  const issues = { count: 1, worst: "warning" };
  const tbl = U.layoutTable({ readThrough: true, selected: 40, state: { 10: "checked", 20: "checked", 30: "stale" }, warnCell: { 140: "material" }, originsOn: true, zebra: true, openMark: { node: 40, slot: "load" }, expansions: { 20: U.joinedRestraints(20), 40: U.joinedLoads(40) } });
  const surfaces = `<div class="tableview"><div class="region">${U.tabs(MODEL_TABS({ layout: 16, restraints: 7, nodeData: 2 }), "Layout")}<div class="tblscroll">${tbl.html}</div>${U.tfoot([{ n: 15, label: "elements" }, { n: 7, label: "restraints" }, { n: 3, label: "loads" }, { n: 2, label: "node data" }, { n: 1, label: "issue", color: "var(--issue-warning)", keep: true }], ["2 joined rows open", "Read-through on", "Origins"], { sel: { rows: 16, selected: 1 } })}</div></div>`;
  return shell({
    toolbar: { project: "Loop 4 header", saveState: "saved", view: "Table", run: "enabled", issues, undo: "gap at node 20" },
    rail: { current: "Model", disabled: noRun, issues },
    surfaces,
    status: { chips: [], issues, selection: "Node 40 · 1 row · load row open" },
  });
}

// ---------- state 5 ----------
function s5() {
  const issues = { count: 1, worst: "warning" };
  // The combination editor as a row expansion under OCC1 (decision 14; V1.2 §5.1): the closing chevron and the
  // close control on its caption, and two buttons that read their names with the keys in their tooltips.
  const expansion = U.blockCap("Combination editor · OCC1", "· row expansion", "the combination editor") +
    `<div class="comprow"><span class="chip outline term">W</span><span class="sec">+</span><span class="chip outline term">P1</span><span class="sec">+</span><span class="chip outline term">SE1 <span class="sec">0.3 g X</span></span><span class="sec" style="margin-left:10px">available</span>${["P2", "T1", "T2", "SUS", "OPE1", "OPE2"].map((t) => `<span class="chip outline dim" role="button">${t}</span>`).join("")}</div>` +
    `<div class="comprow"><span class="combo">Stress type: Occasional</span><span class="combo">Rule: OCC-A1 · sample-rules 1.2</span><span class="grow"></span><span class="btn compact text" role="button" title="Cancel (⎋)">Cancel</span><span class="btn compact primary" role="button" title="Done (↩)">Done</span></div>`;
  const tbl = U.casesTable({ selected: "OCC1", editing: "OCC1", expansion });
  const head = `<div class="tblhead"><span class="name">Cases</span><span class="sec">generated from rule pack <span class="mono">sample-rules 1.2 · sha256:9b1c4e02…</span> on 2026-09-17 10:05</span><span class="btn compact" role="button">${icon("rules")}Generate from rule pack…</span><span class="grow"></span><span class="chipcap">${mark("displayonly", "var(--text-muted)")} Rule expression: Display only, not accepted as input</span></div>`;
  const surfaces = `<div class="tableview"><div class="region">${head}${U.tabs(LOADS_TABS, "Cases")}<div class="tblscroll">${tbl.html}</div>${U.tfoot([{ n: 4, label: "generated" }, { n: 1, label: "edited" }, { n: 1, label: "authored" }, { n: 4, label: "with a rule" }, { n: 2, label: "load sets" }], ["1 expansion open", "Origins", "Seismic SE1 · 0.3 g X"], { sel: { rows: 6, unit: "cases", selected: 1 } })}</div></div>`;
  return shell({
    toolbar: { project: "Loop 4 header", saveState: "edited", view: "Table", run: "enabled", issues, undo: "add case OCC1" },
    rail: { current: "Loads", disabled: noRun, issues },
    surfaces,
    status: { chips: [], issues, selection: "Case OCC1 · Expression · expansion open" },
  });
}

// ---------- state 6 ----------
function s6() {
  const issues = { count: 2, worst: "warning" };
  // One failure banner, on the page; the drawer opens filtered to the failing class and its selected row carries
  // the same link (decision 6; V1.2 §5.3). No status chip and an empty left end (Q-21): the stopped run's record
  // carries no status, and Model incomplete is not carried over from before the run.
  const banner = `<div class="banner warn">${mark("warning")}<span><b>Run 02 failed:</b> nonlinear support at node 20 did not converge.</span><a>Show node 20</a></div>`;
  const head = `<div class="tblhead" data-standing="failed"><span class="name">Stresses</span><span class="sec runtext" role="button" aria-haspopup="menu" title="Runs">Run 02 · 15:02 · stopped in OPE1${icon("expanded", "s12")}</span><span class="grow"></span><span class="btn compact text" role="button" title="Run identity" aria-expanded="false">${icon("info")}Run identity</span></div>`;
  const empty = `<div class="emptystate"><div class="es">No results — Run 02 stopped in OPE1 (case 3 of 6) at the iteration limit. A run's results are one immutable set, so a stopped run has none. Change the support at node 20 or the iteration limit in Run settings, then Run again.</div></div>`;
  const tables = `<div class="tables">${banner}${head}${U.tabs(RESULT_TABS, "Stresses")}${empty}${U.issuesDrawer(M.issuesAt.s6, { selected: M.issuesAt.s6[0], filter: "Nonlinear", link: "Show node 20" })}</div>`;
  const fig = figure({ id: "s6", w: 603, h: 828, view: { A: 200, E: 28 }, selection: { node: 20 }, issueNode: 20, loadsNeutral: true, margin: { top: 110, right: 40, bottom: 80, left: 40 } });
  const canvas = `<div class="canvas" style="width:603px;position:relative">${fig.svg}${U.hud({ needsRun: true })}${MOCK_LABEL}</div>`;
  const surfaces = `<div class="bothview">${tables}${canvas}</div>`;
  // The run log (decision 5; Q-16; G-12; V1.2 §5.2): its left edge at the canvas pane's left edge, which is further
  // right than the Run button's, so it lies over the canvas's top edge and the table's tab strip and header rows
  // stay uncovered. A completed step carries the Entered dot, the stopping step the warning triangle, a step not
  // run no glyph.
  const dot = mark("entered", "var(--mark-origin)");
  const runpop = `<div class="pop runlog" style="position:absolute;left:793px;top:50px"><div class="ttl">Run 02 · <span class="n">15:02:14–15:02:41</span> · settings S-02</div>` +
    `<div class="r">${dot}<b>Assembled</b><span class="n">0.3 s</span></div>` +
    `<div class="r">${dot}<b>W, SUS solved</b><span class="n">0.8 s</span></div>` +
    `<div class="r">${mark("warning", "var(--issue-warning)")}<b>OPE1 stopped</b><span>at iteration 50 · node 20</span><a>Show node 20</a></div>` +
    `<div class="r"><span class="noglyph"></span><b>OPE2, EXP1, EXP2, OCC1, hangers, rules</b><span>not run</span></div>` +
    `<div class="foot"><span class="btn compact" role="button">Run settings…</span><span class="btn compact" role="button">Run record</span><span class="grow"></span><span class="btn compact" role="button">${icon("run", "s12")}Run again</span></div></div>`;
  return shell({
    toolbar: { project: "Loop 4 header", saveState: "saved", view: "Both", run: "enabled", issues, undo: "gap at node 20" },
    rail: { current: "Results", disabled: { Review: "No solved run" }, caps: { Results: { text: "Failed", failed: true } }, issues },
    surfaces,
    status: { chips: [], issues, selection: "Node 20 · 1 row" },
    overlays: runpop,
  });
}

// ---------- state 7 ----------
// standing "current": the coloured model, the legend's scale, the pinned probe, the evidence chip, two chips.
// standing "historical" (ruling 6; R-9; V1.2 §5.1 run standing): the project reopened with its saved Run 03, which
// is Historical always: the band in the product's rendered text with its popover open, no evidence chip, no status
// chip, the rail caption Historical, the neutral figure, the legend's note card, Deformation and Probe disabled.
function s7both(standing = "current") {
  const issues = { count: 1, worst: "warning" };
  const hist = standing === "historical";
  const rows = M.stressRows("EXP1");
  const tbl = U.stressTable({ rows, selected: 70 });
  const head = U.resultsHead({ name: "Stresses", run: hist ? "Run 03 · solved 2026-09-17 15:21 · immutable" : "Run 03 · solved 15:21 · immutable", caseSel: "EXP1", envelope: false, evidence: "INTERNALLY_VERIFIED", standing, discOpen: !hist, identity: ID03, band: hist ? U.historicalBand({ open: true }) : "" });
  const tables = `<div class="tables">${head}${U.tabs(RESULT_TABS, "Stresses")}<div class="tblscroll">${tbl.html}</div>${U.tfoot([{ n: 6, label: "cases solved" }, { n: "0.72", label: "governing · EXP1 · node 70 · EXP-A1 · pack 1.2", keep: true }], ["Sorted by ratio"], { sel: { rows: 15, unit: "elements", selected: 1, readOnly: true } })}</div>`;
  const ratios = Object.fromEntries(rows.map((r) => [r.node, r.ratio]));
  const fig = figure({ id: "s7", w: 603, h: 828, view: { A: 200, E: 28 }, colorBy: hist ? null : { ratios }, selection: { element: [60, 70] }, loads: false, margin: { top: 100, right: 40, bottom: 80, left: 40 } });
  let furniture;
  if (hist) furniture = `${U.hud({ needsRun: true })}${U.legend({ note: "Historical saved run · Run 03" })}`;
  else {
    const p70 = fig.P[70];
    let px = Math.round(p70.x + 18), py = Math.round(p70.y + 18);
    px = Math.min(px, 603 - 300 - 8); py = Math.min(py, 828 - 262);
    if (px + 300 > 603 - 236 && py < 300) py = 300;
    const probe = U.probe({ x: px, y: py, node: 70, element: "60–70", case: "EXP1", stress: 148.9, allowable: 206.8, ratio: 0.72, rule: "EXP-A1", pack: "1.2", run: "Run 03", evidence: "INTERNALLY_VERIFIED" });
    furniture = `${U.hud({ pressed: ["probe"] })}${U.legend({ quantity: "Ratio · expansion", caseName: "EXP1", value: 0.72, range: 1.0, node: 70, rule: "EXP-A1", pack: "1.2" })}${probe}`;
  }
  const canvas = `<div class="canvas" style="width:603px;position:relative" data-edges='${JSON.stringify(fig.edges)}'>${fig.svg}${furniture}${MOCK_LABEL}</div>`;
  const surfaces = `<div class="bothview">${tables}${canvas}</div>`;
  const overlays = hist ? U.historicalPopover({ run: "Run 03", statuses: ["MECHANICS_SOLVED", "USER_RULE_CHECKED"], style: "position:absolute;left:361px;top:112px;width:424px" }) : "";
  return shell({
    toolbar: { project: "Loop 4 header", saveState: "saved", view: "Both", run: "enabled", issues, undo: hist ? null : "gap at node 20" },
    rail: { current: "Results", disabled: hist ? { Review: "Needs a current run" } : {}, caps: hist ? { Results: { text: "Historical" } } : {}, issues },
    surfaces,
    status: { chips: hist ? [] : [CH.solved, CH.rules], issues, selection: "Element 60–70 · node 70" },
    overlays,
  });
}

function s7table() {
  const issues = { count: 1, worst: "warning" };
  const rows = M.envelopeRows();
  const menu = `<div class="pop cellmenu" style="min-width:260px"><div class="ttl">Ratio <span class="sec" style="font-weight:400">= Stress / Allowable</span></div><div class="row"><span class="k on">${icon("check", "s12")}</span>Sort descending<span class="key">⌘↓</span></div><div class="row"><span class="k"></span>Sort ascending<span class="key">⌘↑</span></div><hr><div class="row"><span class="k on">${icon("check", "s12")}</span>Data bar · 1.0 tick</div><div class="row"><span class="k"></span>Colour by scale</div><hr><div class="row">Filter ratio ≥ <span class="input n" style="width:64px;height:22px">0.50</span><span class="key">3 of 15 rows</span></div><div class="row"><span class="k"></span>Unchecked rows only</div><hr><div class="row sec" style="font-size:12px">Rule and pack are their own columns · width 120 · <a>reset</a></div></div>`;
  const tbl = U.stressTable({ rows, selected: 70, menuOpen: menu });
  // Envelope on: the case selector stays visible and disabled, reading "Case: all" (decision 8).
  const head = U.resultsHead({ name: "Stresses", run: "Run 03 · solved 15:21 · immutable", caseSel: "EXP1", envelope: true, evidence: "INTERNALLY_VERIFIED", discOpen: false, identity: ID03 });
  const surfaces = `<div class="tableview"><div class="region">${head}${U.tabs(RESULT_TABS, "Stresses")}<div class="tblscroll">${tbl.html}</div>${U.tfoot([{ n: 6, label: "cases in the envelope" }, { n: "0.72", label: "governing · EXP1 · node 70 · EXP-A1 · pack 1.2", keep: true }], ["Envelope: governing case per element", "Sorted by ratio"], { sel: { rows: 15, unit: "elements", selected: 1, readOnly: true } })}</div></div>`;
  return shell({
    toolbar: { project: "Loop 4 header", saveState: "saved", view: "Table", run: "enabled", issues, undo: "gap at node 20" },
    rail: { current: "Results", issues },
    surfaces,
    status: { chips: [CH.solved, CH.rules], issues, selection: "Element 60–70 · node 70" },
  });
}

// ---------- state 8 ----------
function s8table() {
  const issues = { count: 1, worst: "warning" };
  // The run is Stale (V1.2 §5.1, run standing): the stale band in ROOT's settled wording, the values hatched, the
  // rail caption Stale, no status chip and no evidence chip. The hanger selection is open under H1 as the row's
  // expansion, with the content boundary's short variant once (ruling 5).
  const tbl = U.hangerTable({ stale: true, selected: 80, pending: true, selection: 60, selectionState: "Stale" });
  const band = U.staleBand({ run: "Run 03", change: "proposal P-12 row 1 accepted at 16:31 (variable spring at node 80)" });
  const head = U.resultsHead({ name: "Hangers", run: "Run 03 · solved 15:21 · immutable", evidence: "INTERNALLY_VERIFIED", standing: "stale", discOpen: false, identity: ID03, band });
  const lib = `<div class="stagecap">${icon("libraries", "s12")}Hanger library <b style="font-weight:500;color:var(--text-primary)">Vendor-A springs</b> · user import 2026-09-15 · 24 sizes · source recorded in Libraries · max variation 25 %</div>`;
  const surfaces = `<div class="tableview"><div class="region">${head}${U.tabs(RESULT_TABS, "Hangers")}${lib}<div class="tblscroll">${tbl.html}</div>${U.tfoot([{ n: 1, label: "stale", keep: true }, { n: 1, label: "not yet designed", keep: true }, { n: 1, label: "proposal row pending", color: "var(--proposal-new)", keep: true }], ["1 expansion open"], { sel: { rows: 2, unit: "hanger locations", selected: 1, readOnly: true } })}</div></div>`;
  const card = U.proposalCard(M.proposals[1], { decisions: { 0: "accepted" } });
  const body = `${card}<div class="qa">${mark("accepted", "var(--mark-origin)")}Accepted: P-09 · 11:40 · P-12 row 1 · 16:31</div>`;
  return shell({
    agent: "column",
    agentCol: { tab: "Proposals", counts: { proposals: 1, checks: 0, accepted: 2 }, state: "Idle · P-12 open", body },
    toolbar: { project: "Loop 4 header", saveState: "edited", view: "Table", run: "enabled", issues, agentOpen: true, undo: "accept P-12 row 1" },
    rail: { current: "Results", caps: { Results: { text: "Stale", stale: true } }, issues },
    surfaces,
    status: { chips: [], issues, selection: "Node 80 · 1 row" },
    overlays: toast("check", "Accepted P-12 row 1", "Undo", 348),
  });
}

function s8model() {
  const issues = { count: 1, worst: "warning" };
  const fig = figure({ id: "s8", w: 1000, h: 548, ghost: { node: 80, id: "P-12" }, selection: { node: 80 }, loadsNeutral: true, margin: { top: 70, right: 60, bottom: 92, left: 60 } });
  const canvas = canvasBox(1000, 548, `${fig.svg}${U.hud({})}${MOCK_LABEL}`);
  const prop = { 80: { fields: { Type: ["Rigid (Y)", "Variable spring"], Library: ["—", "Vendor-A springs"], "Max variation": ["—", "25"] } }, 20: { fields: { Gap: ["3", "0"] } } };
  const tbl = U.restraintsTable({ prop, selected: 80, noAdd: true, noCnode: true, noNote: true });
  const drawer = `<div class="drawer">${U.tabs(MODEL_TABS({ layout: 16, restraints: 7, nodeData: 2 }), "Restraints", { collapse: true })}<div class="tblscroll">${tbl.html}</div>${U.tfoot([{ n: 2, label: "proposed rows · P-12 · draft until accepted", color: "var(--proposal-new)", keep: true }], ["Origins"], { sel: { rows: 7, unit: "restraints", selected: 1, proposed: true } })}</div>`;
  const surfaces = `<div class="modelview"><div class="left">${canvas}${drawer}</div><div class="inspector">${U.inspector({ node: 80, proposedRestraint: true, issues: "none", when: "2026-09-17 10:52" })}</div></div>`;
  return shell({
    agentStrip: { badge: 1 },
    toolbar: { project: "Loop 4 header", saveState: "saved", view: "Model", run: "enabled", issues, undo: "gap at node 20" },
    rail: { current: "Model", issues },
    surfaces,
    status: { chips: [CH.solved, CH.rules], issues, selection: "Node 80 · 1 row · 2 proposed rows" },
  });
}

// ---------- state 9 ----------
function liveRestraints(nodes) {
  const cols = [["gut", 32], ["Node", 56, "n"], ["Tag", 56], ["Type", 112], ["Gap", 80, "n", "mm"], ["Friction μ", 80, "n"], ["Library", 124], ["Max variation", 128, "n", "%"]];
  const width = cols.reduce((s, c) => s + c[1], 0);
  const rs = M.restraints.filter((r) => nodes.includes(r.node)).map((r) => ({ ...r }));
  const r80 = rs.find((r) => r.node === 80); Object.assign(r80, { type: "Variable spring", library: "Vendor-A springs", maxVar: 25, gap: null, origin: "accepted", proposal: "P-12" });
  const body = rs.map((r) => `<tr>${U.gutter(r.origin === "accepted" ? "accepted" : null, r.node === 20 ? "comment" : null, { origin: `Accepted from proposal ${r.proposal} by R. Tufts`, state: "Comment by Agent · 09:33 · Show in comments" })}<td class="n">${r.node}</td><td>${r.tag}</td><td>${esc(r.type)}</td><td class="n">${r.gap != null ? num(r.gap) : "—"}</td><td class="n">${r.mu != null ? r.mu.toFixed(2) : "—"}</td><td>${r.library || "—"}</td><td class="n">${r.maxVar != null ? num(r.maxVar) : "—"}</td></tr>`).join("");
  return `<table class="ds" style="width:${width}px"><colgroup>${cols.map((c) => `<col style="width:${c[1]}px">`).join("")}</colgroup><thead><tr>${cols.map((c) => c[0] === "gut" ? `<th class="gut"></th>` : `<th class="${c[2] || ""}">${c[0]}${c[3] ? ` <span class="u">[${c[3]}]</span>` : ""}</th>`).join("")}</tr></thead><tbody>${body}</tbody></table>`;
}

// o.open: which of the page's two menus the frame shows open, "iteration" (Q-17) or "kind" (Q-22).
function s9(o = {}) {
  const issues = { count: 1, worst: "warning" };
  const outline = M.review.outline.map((x) => `<div class="${x.on ? "on" : ""}"><span class="num">${x.n}</span><span>${esc(x.name)}</span>${x.fixed ? `<span class="lock" title="Fixed section from the record">${icon("lock")}</span>` : ""}${x.state ? `<span class="st ${x.state}">${x.state}</span>` : ""}</div>`).join("");
  const cases = U.casesTable({ noRuleExpr: true, noAdd: true });
  const content = `<div class="content"><h3>4. Model <span class="stt">edited · iteration 2</span></h3><p>Loop 4 header runs from the pump P-401 discharge nozzle (node 10, anchor N1) to the vessel V-402 nozzle (node 130, anchor N2), with a branch from the welding tee at node 70 to the header tie-in at node 160 (anchor N3): 16 node rows, 15 elements, sections P1 (DN200), P2 (DN150) and P3 (DN100), materials CS-A and CS-B.</p><p>Vertical support is a +Y resting support with a 3 mm gap at node 20 (as surveyed), the variable spring hanger H1 at node 60 (Vendor-A size A-3) and <span class="del">a rigid rod RS-02 at node 80 from the steel</span> <span class="ins">a variable spring hanger RS-02 at node 80 (Vendor-A, design load 4 180 N; accepted from proposal P-12 row 1 on 2026-09-17 16:31)</span>. A guide G-01 with a 2 mm gap holds the riser at node 120. <span class="del">The OPE1 load on the vessel nozzle was 7.4 kN.</span> <span class="ins">The OPE1 load on the vessel nozzle is 3.9 kN in Run 04.</span></p><div class="cap">${icon("lock")}Live from the model record · Restraints · nodes 20, 60, 80 · not editable here<span class="grow"></span><span class="editsleg"><span class="ins">inserted</span><span class="del">removed</span> since Iteration 1</span></div><div class="live">${liveRestraints([20, 60, 80])}</div><h3>5. Load cases <span class="stt">drafted</span></h3><p>Six cases from rule pack sample-rules 1.2, one edited (EXP1 as OPE1 − SUS) and one authored (OCC1 with SE1 at 0.3 g in X).</p><div class="cap">${icon("lock")}Live from the model record · Cases · not editable here</div><div class="live">${cases.html}</div></div>`;
  // The filter row (Q-22; V1.2 §5.5): four chips for state and ownership and one Kind menu for the agent's four kinds.
  const C = M.review.comments;
  const cnt = (fn) => C.filter(fn).length;
  const filters = [["All", C.length, true], ["Open", cnt((c) => c.state === "open")], ["Resolved", cnt((c) => c.state === "resolved")], ["Mine", cnt((c) => c.who === "R. Tufts")]].map(([n, c, on]) => `<span class="chip outline${on ? " on" : ""}" role="button">${n} ${c}</span>`).join("");
  const kinds = [["Checks", "Check"], ["Open issues", "Open issue"], ["Drafts", "Draft"], ["Evidence summaries", "Evidence summary"]].map(([label, k]) => [label, cnt((c) => c.kind === k)]).filter(([, n]) => n > 0);
  const kindMenu = o.open === "kind" ? `<div class="pop kindmenu"><div class="row" role="button"><span class="sel">${icon("check", "s12")}</span>All kinds</div><hr>${kinds.map(([l, n]) => `<div class="row" role="button"><span class="sel"></span>${l}<span class="key n">${n}</span></div>`).join("")}</div>` : "";
  const comments = `<div class="comments"><div class="hd">Comments <span class="n">${C.length}</span><span class="grow" style="flex:1"></span><span class="btn compact text" role="button">+ Comment</span></div><div class="filters">${filters}<span class="kindwrap"><span class="combo" role="button" aria-expanded="${o.open === "kind"}" title="Kind">Kind: all</span>${kindMenu}</span></div>${C.map(U.commentCard).join("")}</div>`;
  // The header (decision 10; Q-17, Q-19, G-7; V1.2 §5.5): the iteration combobox, whose menu ends in "Compare with…";
  // while something is compared, the compared-with combobox and Show edits; three plain buttons of one weight with icons.
  const it = M.review.iterations;
  const iterMenu = o.open === "iteration" ? `<div class="pop itermenu"><div class="row" role="button"><span class="sel">${icon("check", "s12")}</span>Iteration 2 · 09:40 · Run 04</div><div class="row" role="button"><span class="sel"></span>${it[0].name} · ${it[0].when} · ${it[0].run}</div><hr><div class="row" role="button"><span class="sel"></span>Compare with…</div></div>` : "";
  const page = `<div class="reviewpage"><div class="hd"><span class="ttl">${esc(M.review.title)}</span><span class="iterwrap"><span class="combo" role="button" aria-expanded="${o.open === "iteration"}">Iteration 2 · 09:40 · Run 04</span>${iterMenu}</span><span class="combo cmp" role="button" title="compared with Iteration 1 · 2026-09-17 15:40 · Run 03"><span class="tx">compared with Iteration 1 · 2026-09-17 15:40 · Run 03</span></span><span class="switch on" role="button"><i></i>Show edits</span><span class="grow" style="flex:1"></span><span class="btn" role="button">${icon("snapshot")}Snapshot…</span><span class="btn" role="button">${icon("preview")}Report preview</span><span class="btn" role="button">${icon("export")}Export…</span></div><div class="cols"><div class="outline">${outline}<div class="note">${icon("lock", "s12")} fixed sections come from the record</div></div>${content}${comments}</div></div>`;
  return shell({
    agentStrip: { badge: 1 },
    toolbar: { project: "Loop 4 header", saveState: "saved", view: "Table", run: "enabled", issues, undo: "edit report §4" },
    rail: { current: "Review", issues },
    surfaces: page,
    status: { chips: [CH.solved, CH.rules, CH.review], issues, selection: "Section 4 · Model" },
  });
}

export const frames = [
  { file: "s1_both_light", state: 1, stateName: "New project, the first node row", stage: "Model", view: "Both", theme: "light", body: s1,
    tests: "State 1 (Both; C-22, UX_SPEC V1.2 §10.1): a new project opens the Model stage in Both view: an empty model's first row with the required marks and the Run button's reason; the canvas with the grid, the triad and node 10 as a point; the one chip; the footer's edit chip with Apply (V1.3 row 102).",
    look: "Solver · Model incomplete as the one chip; the toolbar's Undo and Redo disabled and the Inspector toggle available in Both view; the canvas pane holding node 10 alone on the ground grid with Deformation and Probe disabled; the footer's edit chip (Editing DX · node 20) with its Apply and Cancel buttons; the Add row line and the hint line that names controls before keys; the information glyph at the status bar's right end, which opens About and hangs no popover." },
  { file: "s2_model_light", state: 2, stateName: "Routing in the canvas with direct distance entry", stage: "Model", view: "Model", theme: "light", body: s2,
    tests: "V1.2 §8: state 2 with the compass's Reverse, Place and Cancel beside the length field and Place node 50 in the routing block (the pointer rule).",
    look: "The three 22 px buttons to the right of the length field and the short stub opposite the active +Y axis; the routing block's three named buttons; the hint strip, whose every key now has a control; the footer's selection group; the drawer's collapse chevron; Deformation and Probe disabled with Needs a current run." },
  { file: "s2_model_dark", state: 2, stateName: "Routing in the canvas with direct distance entry", stage: "Model", view: "Model", theme: "dark", body: s2,
    tests: "State 2 in dark: the compass's buttons, the draft ghost and the hint strip against the dark ground.",
    look: "The same moment under the dark tokens: the raised compass buttons on the dark canvas, the draft ghost's centreline and outline, the latched Route tool on pressed.fill." },
  { file: "s3_table_light", state: 3, stateName: "Editing the layout table: propagation, keyboard entry, the paste band", stage: "Model", view: "Table", theme: "light", body: s3,
    tests: "State 3 (Table): propagation marks after keyboard entry; the paste band above the rows it will create, with two unmapped columns; the footer's edit chip.",
    look: "The paste band between row 130 and the Add row line, its two buttons reading Cancel and Paste 3 rows, names only, with the keys in their tooltips (C-20, variant A); the edit cell on row 130 DY and the edit chip in the footer, which takes the place of the counts while the cell is edited." },
  { file: "s4_both_light", state: 4, stateName: "Restraint and load tables; marks on the node rows; glyphs; the inspector docked", stage: "Model", view: "Both", theme: "light", body: () => s4both("docked"),
    tests: "V1.2 §8: state 4 Both with the inspector docked, the Inspector toggle latched, the two-row HUD at 303 px with Fit first and the mock line under the stack (G-10); the fitted camera refitting on dock (Q-20).",
    look: "The canvas at 303 px with the whole model in view, because the camera was fitted and docking refits by itself; the HUD as two rows of five, 154 px wide, Fit first; the Inspector toggle latched beside the Agent toggle; the inspector's close control; the footer's selection group with Clear check, since the selected row is Checked." },
  { file: "s4_both_light_column", state: 4, stateName: "The same moment with the agent column open before the inspector docked", stage: "Model", view: "Both", theme: "light", body: () => s4both("column"),
    tests: "V1.2 §0 and UX_SPEC_V1 §10.9: the agent column collapses to its strip first; the toast component (G-8) with its action and close control; a placed camera kept on dock (decision 1).",
    look: "The toast at the bottom-right corner of the surfaces, 320 px, over the inspector's foot: the message, Reopen and the close control, and no key to press; the Agent toggle not latched; the canvas keeping the camera the engineer placed in the 470 px canvas, so the model is cropped, unlike s4_both_light." },
  { file: "s4_both_light_slideover", state: 4, stateName: "The other order: the agent column opened while the inspector was docked", stage: "Model", view: "Both", theme: "light", body: () => s4both("slideover"),
    tests: "V1.2 §8 (Q-15): the agent column reopened with the inspector docked: the column opens and the inspector becomes the slide-over over the 470 px canvas.",
    look: "Both toggles latched; the surfaces at 1044 px split 574 / 470; the inspector as a slide-over over the canvas's right 300 px with its close control; 170 px of canvas left in view, with the one-row HUD running under the slide-over (Q-24); the agent's Conversation with the Send control and the column's collapse control." },
  { file: "s4_table_light", state: 4, stateName: "Restraint and load tables; marks on the node rows", stage: "Model", view: "Table", theme: "light", body: s4table,
    tests: "State 4 (Table): restraint and load marks on node rows; the joined rows the marks open, each with its closing chevron and close control; the expansion chevron in the Type cell.",
    look: "The two joined rows' caption lines with the chevron at the left and the close control at the right; the expansion chevron at the right edge of the selected row's Type cell; the Add row line; the footer's selection group of five." },
  { file: "s5_table_light", state: 5, stateName: "Load cases: generated rows edited, an authored case, the combination editor as a row expansion", stage: "Loads", view: "Table", theme: "light", body: s5,
    tests: "State 5: the combination editor as a row expansion under OCC1, opened by the Expression cell's chevron; Cancel and Done reading their names.",
    look: "The expanded chevron in OCC1's Expression cell; the editor's caption with its close control; Cancel and Done without key glyphs; the Add case line; the rule-expression column in its dashed display-only frame with the caption once in the header." },
  { file: "s6_both_light", state: 6, stateName: "After a failed run: the run log over the canvas, the single failure banner, the drawer filtered", stage: "Results", view: "Both", theme: "light", body: s6,
    tests: "V1.2 §8: state 6 with the run log over the canvas's top edge and the dot and triangle glyphs (Q-16, G-12); the empty left end of the status bar (Q-21); the drawer row's overflow (G-11).",
    look: "The run log's left edge at the canvas pane's left edge, the table's tab strip and banner uncovered; the Entered dot on the two completed steps, the triangle on the stopped one, none on the step not run; no chip at the status bar's left end; the drawer's selected row with its message truncated and the entity and the link right-aligned; the drawer's Filter menu button beside the class chips (C-24; V1.3 row 96) and its close control." },
  { file: "s7_both_light", state: 7, stateName: "Results: the stress table, the case selector and Envelope, the evidence chip, the run identity; the coloured model", stage: "Results", view: "Both", theme: "light", body: () => s7both("current"),
    tests: "State 7 (Both), a Current run: chips with their domains in the status bar, the results header and the probe; the Run identity disclosure holding the identity line alone; canvas.edgeAlt on the result-coloured elements (R-6).",
    look: "Solver · Mechanics solved and Rule pack · User-rule checked; Evidence · Internally verified beside the run name and in the probe's footer; the disclosure with one mono line and no sentence; the pale edge line on the darker teal elements and the dark edge on the two palest; the pinned probe's close control." },
  { file: "s7_table_light", state: 7, stateName: "Results in the Table view with Envelope on", stage: "Results", view: "Table", theme: "light", body: s7table,
    tests: "State 7 (Table): Envelope on with the case selector visible and disabled; the Ratio column menu; the read-only table's selection group.",
    look: "Case: all, disabled; the Ratio column menu; the footer's selection group with Insert, Delete and Paste disabled on a read-only table and Copy rows and Check rows live (Q-23)." },
  { file: "s7_both_dark", state: 7, stateName: "Results in the Both view under the dark tokens", stage: "Results", view: "Both", theme: "dark", body: () => s7both("current"),
    tests: "State 7 dark: the re-anchored result scale with canvas.edgeAlt, the dark line, on the brighter elements (R-6).",
    look: "The brightest teal steps carrying a dark edge line where V1.1 had a light line at 1.10:1; the legend bar and the data bars on bar.track; the label chips under the dark tokens." },
  { file: "s7_both_light_historical", state: 7, stateName: "A Historical run: the project reopened with its saved Run 03", stage: "Results", view: "Both", theme: "light", body: () => s7both("historical"),
    tests: "V1.2 §8 (ruling 6; R-9): the historical band in the product's rendered text with its popover, the neutral canvas, the legend's note card, no status chip.",
    look: "The warm hatched band and its information control, the popover with the second sentence and the run's recorded statuses as label chips with their raw tokens; no evidence chip in the header; the rail caption Historical and Review disabled; the figure in the neutral pipe colour; the note card in the legend's place; Deformation and Probe disabled." },
  { file: "s8_table_light", state: 8, stateName: "Hanger design against a user library; a proposal accepted row by row; the model changed since the run", stage: "Results", view: "Table", theme: "light", body: s8table,
    tests: "V1.2 §8: state 8 under a Stale run: the stale band in ROOT's settled wording, hatched values, the rail caption Stale, no chip; the receipt line (Q-18); the consequence line; the toast with an action (G-8); hanger selection with the content boundary's short variant (ruling 5); no State column, the state said in the footer and the expansion's caption (C-21).",
    look: "The band's two sentences and Run again; no evidence chip beside the run name; the hanger selection open under H1 with its one boundary line and the candidate sizes sorted by variation; the receipt line on the card, one line with the new value and +4; the consequence line; the toast Accepted P-12 row 1 with Undo beside the agent column; the Send control under the card." },
  { file: "s8_model_light", state: 8, stateName: "A proposal landing as proposed rows and as a ghost in the canvas", stage: "Model", view: "Model", theme: "light", body: s8model,
    tests: "State 8 (Model): a proposal landing as banded rows with old and new values and as a ghost in the canvas; the footer's Accept row and Reject row chips joining the selection group.",
    look: "The selected proposed row with the band, the bar and the diamond; the footer with the selection group and the two chips; the two chips of the Current run, because this is the moment before row 1 is accepted." },
  { file: "s9_table_light", state: 9, stateName: "The Review page", stage: "Review", view: "Table", theme: "light", body: () => s9({ open: "iteration" }),
    tests: "V1.2 §8: state 9 with the header's icons and plain Export… (Q-19, G-7), Compare with… in the iteration menu (Q-17), the four chips and the Kind menu (Q-22), an Evidence summary card and a Draft card (ruling 8), and the Review page's three status chips (C-23, variant B).",
    look: "The columns beginning directly under the header's hairline; the iteration menu open with Compare with… after its separator; three buttons of one weight with their icons and no accent on the page; the comment stream's one filter row; the Kind button reading Kind: all; the Draft card's Insert and Discard; Solver · Mechanics solved, Rule pack · User-rule checked and Human · Human review required." },
  { file: "s9_table_dark", state: 9, stateName: "The Review page under the dark tokens", stage: "Review", view: "Table", theme: "dark", body: () => s9({ open: "kind" }),
    tests: "State 9 (dark): the Review page in the dark theme, with the Kind menu open (Q-22).",
    look: "The Kind menu with All kinds first and checked, a separator, then the agent's four kinds with their counts (V1.3 row 94); three status chips; the five class chips on the cards; the inserted and removed text and the live tables under the dark tokens." },
];
