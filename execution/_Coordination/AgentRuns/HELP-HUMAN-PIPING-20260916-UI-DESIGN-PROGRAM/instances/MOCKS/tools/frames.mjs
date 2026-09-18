// The fifteen frame specifications: each returns the stage's inner HTML for one moment of the
// storyboard, composed from the shell, the components and the figure.
import * as M from "./model.mjs";
import * as U from "./ui.mjs";
import { figure, esc } from "./canvas.mjs";
const { mark, icon, num } = U;

const ACCEPT = "Results are engineering decision-support information. Acceptance, professional judgment, and any certification, sealing, or code-compliance determination remain with the responsible engineer and project authority.";
const MOCK_LABEL = `<div class="mocklabel">Mock rendering, not the engine's canvas · schematic isometric, Y up</div>`;
const RESULT_TABS = ["Summary", "Stresses", "Displacements", "Restraint loads", "Restraint summary", "Forces and moments", "Hangers"];
const MODEL_TABS = (counts) => [["Layout", counts.layout], ["Restraints", counts.restraints], ["Node data", counts.nodeData]];
const LOADS_TABS = [["Cases", 6], ["Load sets", 2], ["Loads", 3], ["Wind", 0], ["Seismic", 1]];
const ID03 = "Run 03 · 2026-09-17 15:21 · sha256:4df0f798… · solver 0.2.0 · rule pack sample-rules 1.2 · sha256:9b1c4e02… · settings S-03";

function shell(o) {
  const agent = o.agent === "column" ? U.agentColumn(o.agentCol) : U.agentStrip(o.agentStrip || {});
  return `${U.toolbar(o.toolbar)}${U.rail(o.rail)}<div class="surfaces ${o.agent === "column" ? "column" : "strip"}">${o.surfaces}</div>${agent}${U.statusbar(o.status)}${o.overlays || ""}`;
}
const chip = (label, cls, raw, domain) => ({ label, cls, title: `${raw} · authority: ${domain} · click for the run identity` });
const CH = {
  incomplete: chip("Model incomplete", "incomplete", "MODEL_INCOMPLETE", "Solver"),
  solved: chip("Mechanics solved", "solved", "MECHANICS_SOLVED", "Solver"),
  rules: chip("User rules checked", "solved", "USER_RULE_CHECKED", "Rule pack"),
  review: chip("Human review required", "review", "HUMAN_REVIEW_REQUIRED", "Human"),
};
const noRun = { Results: "No run yet", Review: "No run yet" };
const statusPop = (o) => `<div class="pop" style="position:absolute;left:${o.left}px;bottom:30px;min-width:300px"><div class="ttl"><span class="mono">${o.raw}</span><span class="chip ${o.cls}">${o.label}</span></div><div class="kv"><dt>Authority</dt><dd>${o.domain}</dd><dt>Run</dt><dd>${o.run}</dd><dt>Reason</dt><dd style="white-space:normal">${o.reason}</dd></div></div>`;
const canvasBox = (w, h, inner) => `<div class="canvas" style="width:${w}px;height:${h}px;position:relative;flex:none">${inner}</div>`;

// ---------- state 1 ----------
function s1() {
  const issues = { count: 3, worst: "blocking" };
  const tbl = U.layoutTable({ nodes: [10, 20], readThrough: true, blank: { 10: ["section", "material", "load"] }, required: { 20: ["section", "material", "load"] }, edit: { node: 20, col: "DX", value: "3000", unit: "mm" }, selected: 20, zebra: false, marks: false });
  const surfaces = `<div class="tableview"><div class="region">${U.tabs(MODEL_TABS({ layout: 2, restraints: 0, nodeData: 0 }), "Layout")}<div class="tblscroll">${tbl.html}<div class="emptyhint">↩ commits and adds row 30 · ⇥ moves right · ⌘V pastes rows · Section, Material and Load propagate from the row above once entered · the asterisk marks what the solve needs</div></div>${U.tfoot([{ n: 2, label: "rows" }, { n: 1, label: "element" }, { n: 3, label: "issues", color: "var(--issue-blocking)" }], ["Read-through on", "Origins"])}</div></div>`;
  const overlays = U.tip(`<span class="t">Run is unavailable</span><br>Section missing at node 20, Material missing at node 20`, "left:656px;top:54px") +
    `<div class="pop" style="position:absolute;right:8px;bottom:30px;min-width:260px"><div class="row"><a>About SWB Piping Designer…</a></div><div class="row">Technical preview — not a released product.</div></div>`;
  return shell({
    toolbar: { project: "Loop 4 header", saveState: "not saved", view: "Table", run: "disabled", issues },
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
  const canvas = canvasBox(1000, 548, `${fig.svg}${U.hud({ pressed: ["route"], off: ["deform", "probe"] })}<div class="lenfield" style="left:${Math.round(lf.x)}px;top:${Math.round(lf.y)}px"><span class="n">1500</span><span class="caret"></span><span class="u">mm</span></div><div class="keys" style="left:8px;top:44px">Route · ↩ places 50 · ⇥ next axis · − reverses · ⎋ cancels</div>${MOCK_LABEL}`);
  const tbl = U.layoutTable({ nodes: [10, 20, 30, 40], readThrough: false, selected: 40, draftRow: { node: 50, from: 40, type: "Pipe", axis: "Y", len: 1500, section: "P1", material: "CS-A", load: "OP1" }, zebra: false, marks: false });
  const drawer = `<div class="drawer">${U.tabs(MODEL_TABS({ layout: 4, restraints: 0, nodeData: 0 }), "Layout")}<div class="tblscroll">${tbl.html}</div>${U.tfoot([{ n: 4, label: "rows" }, { n: 3, label: "elements" }, { n: 1, label: "bend" }, { n: 1, label: "draft row" }], ["Read-through off", "Origins"])}</div>`;
  const routing = `<div class="sec-t">Routing from node 40</div><div class="r"><span class="l">Next node</span><span class="v">50</span></div><div class="r"><span class="l">Axis</span><span class="v">+Y <span class="muted">· ⇥ cycles X Y Z</span></span></div><div class="r acc"><span class="l">Length</span><span class="v">1500 mm</span></div><div class="r"><span class="l">Carries</span><span class="v">P1 · CS-A · OP1 <span class="muted">· propagated</span></span></div><div class="acts"><span class="btn compact">Bend at 40… <span class="muted">B</span></span><span class="btn compact text">Cancel <span class="muted">⎋</span></span></div>`;
  const surfaces = `<div class="modelview"><div class="left">${canvas}${drawer}</div><div class="inspector">${U.inspector({ node: 40, routing, noAttach: true, issues: "none", when: "2026-09-17 10:34" })}</div></div>`;
  return shell({
    toolbar: { project: "Loop 4 header", saveState: "edited", view: "Model", run: "disabled", issues },
    rail: { current: "Model", disabled: noRun, issues },
    surfaces,
    status: { chips: [CH.incomplete], issues, selection: "Routing from node 40 · +Y 1500 mm" },
  });
}

// ---------- state 3 ----------
function s3() {
  const issues = { count: 0, worst: null };
  const nodes = M.rows.filter((r) => r.node <= 130).map((r) => r.node);
  const tbl = U.layoutTable({ nodes, readThrough: true, edit: { node: 130, col: "DY", value: "−1500", unit: "mm" }, selected: 130, marks: false, zebra: true });
  const map = [["Node", "Node"], ["From", "From"], ["Type", "Type"], ["DX", "DX"], ["DY", "DY"], ["DZ", "DZ"], ["Sect", "Section"], ["Matl", "Material"], ["Load", "Load"], ["Sched", null], ["Note", null]];
  const maprow = map.map(([src, tgt]) => `<span class="m${tgt ? "" : " ign"}"><span class="src">${src}</span><span class="arrow">→</span><span class="sel">${tgt || "Ignore"}</span></span>`).join("");
  const preview = `<table class="ds" style="width:640px"><colgroup><col style="width:32px"><col style="width:56px"><col style="width:56px"><col style="width:100px"><col style="width:72px"><col style="width:72px"><col style="width:72px"><col style="width:72px"><col style="width:72px"><col style="width:64px"></colgroup><thead><tr><th class="gut"></th><th class="n">Node</th><th class="n">From</th><th>Type</th><th class="n">DX <span class="u">[mm]</span></th><th class="n">DY <span class="u">[mm]</span></th><th class="n">DZ <span class="u">[mm]</span></th><th>Section</th><th>Material</th><th>Load</th></tr></thead><tbody>
<tr class="draft"><td class="gut"></td><td class="n">140</td><td class="n">70</td><td>Pipe</td><td class="n">2000</td><td class="n">0</td><td class="n">0</td><td>P3</td><td>CS-B</td><td>OP2</td></tr>
<tr class="draft"><td class="gut"></td><td class="n">150</td><td class="n">140</td><td>Bend</td><td class="n">1000</td><td class="n">0</td><td class="n">0</td><td class="tick">P3</td><td class="tick">CS-B</td><td class="tick">OP2</td></tr>
<tr class="draft"><td class="gut"></td><td class="n">160</td><td class="n">150</td><td>Pipe</td><td class="n">0</td><td class="n">−3000</td><td class="n">0</td><td class="tick">P3</td><td class="tick">CS-B</td><td class="tick">OP2</td></tr></tbody></table>`;
  const band = `<div class="pasteband" style="width:964px"><div class="hd"><b>Paste</b><span class="sec">3 rows from the clipboard · after node 130 · 2 columns ignored · node IDs taken from the source</span><span class="grow"></span><span class="btn compact text">Cancel <span class="muted">⎋</span></span><span class="btn compact primary">Paste 3 rows <span style="opacity:.8">↩</span></span></div><div class="maprow">${maprow}</div><div class="sec" style="font-size:12px;margin:8px 0 4px">Preview in the layout grammar · row 140 is a branch (From 70) · Section, Material and Load propagate where the source left them blank</div>${preview}</div>`;
  const surfaces = `<div class="tableview"><div class="region">${U.tabs(MODEL_TABS({ layout: 13, restraints: 0, nodeData: 0 }), "Layout")}<div class="tblscroll">${tbl.html}${band}</div>${U.tfoot([{ n: 13, label: "rows" }, { n: 12, label: "elements" }, { n: 3, label: "bends" }, { n: 1, label: "valve" }, { n: 1, label: "reducer" }, { n: M.rows.filter((r) => r.from != null && r.node <= 130).reduce((t, r) => t + 3 - r.entered.length, 0), label: "propagated cells" }, { n: 0, label: "issues" }], ["Read-through on", "Origins"])}</div></div>`;
  return shell({
    toolbar: { project: "Loop 4 header", saveState: "edited", view: "Table", run: "disabled", issues },
    rail: { current: "Model", disabled: noRun, issues },
    surfaces,
    status: { chips: [CH.incomplete], issues, selection: "Node 130 · DY" },
  });
}

// ---------- state 4 ----------
function s4both() {
  const issues = { count: 1, worst: "warning" };
  const tbl = U.layoutTable({ readThrough: false, typeW: 96, selected: 20, state: { 10: "checked", 20: "checked", 30: "stale" }, warnCell: { 140: "material" }, originsOn: true, zebra: true, openMark: { node: 20, slot: "restraint" } });
  const tables = `<div class="tables">${U.tabs(MODEL_TABS({ layout: 16, restraints: 7, nodeData: 2 }), "Layout")}<div class="tblscroll">${tbl.html}</div>${U.tfoot([{ n: 16, label: "rows" }, { n: 15, label: "elements" }, { n: 7, label: "restraints" }, { n: 3, label: "loads" }, { n: 2, label: "node data" }, { n: 2, label: "checked" }, { n: 1, label: "stale", color: "var(--mark-checkedStale)" }], ["Read-through off", "Origins"])}</div>`;
  const fig = figure({ id: "s4", w: 603, h: 828, view: { A: 200, E: 28 }, selection: { node: 20 }, plateSide: "left", margin: { top: 90, right: 200, bottom: 90, left: 24 } });
  const canvas = `<div class="canvas" style="width:603px;position:relative">${fig.svg}${U.hud({ off: ["deform", "probe"] })}<div class="mocklabel" style="left:16px;transform:none">Mock rendering, not the engine's canvas</div>${U.inspector({ node: 20, slide: true, checked: true, issues: "none", when: "2026-09-17 10:31" })}</div>`;
  const surfaces = `<div class="bothview">${tables}${canvas}</div>`;
  const overlays = U.tip(`<span class="t">Restraint</span> +Y · gap 3 mm · μ 0.30 · RS-01<br><span class="t">⌘↩ opens the row here</span>`, "left:604px;top:158px");
  return shell({
    toolbar: { project: "Loop 4 header", saveState: "saved", view: "Both", run: "enabled", issues },
    rail: { current: "Model", disabled: noRun, issues },
    surfaces,
    status: { chips: [], issues, selection: "Node 20 · 1 row" },
    overlays,
  });
}

function s4table() {
  const issues = { count: 1, worst: "warning" };
  const tbl = U.layoutTable({ readThrough: true, selected: 40, state: { 10: "checked", 20: "checked", 30: "stale" }, warnCell: { 140: "material" }, originsOn: true, zebra: true, openMark: { node: 40, slot: "load" }, expansions: { 20: U.joinedRestraints(20), 40: U.joinedLoads(40) } });
  const surfaces = `<div class="tableview"><div class="region">${U.tabs(MODEL_TABS({ layout: 16, restraints: 7, nodeData: 2 }), "Layout")}<div class="tblscroll">${tbl.html}</div>${U.tfoot([{ n: 16, label: "rows" }, { n: 15, label: "elements" }, { n: 7, label: "restraints" }, { n: 3, label: "loads" }, { n: 2, label: "node data" }, { n: 2, label: "joined rows open" }, { n: 1, label: "issue", color: "var(--issue-warning)" }], ["Read-through on", "Origins"])}</div></div>`;
  return shell({
    toolbar: { project: "Loop 4 header", saveState: "saved", view: "Table", run: "enabled", issues },
    rail: { current: "Model", disabled: noRun, issues },
    surfaces,
    status: { chips: [], issues, selection: "Node 40 · 1 row · load row open" },
  });
}

// ---------- state 5 ----------
function s5() {
  const issues = { count: 1, worst: "warning" };
  const composer = `<div class="pop cellmenu" style="min-width:420px;top:26px"><div class="ttl">Combination · OCC1 <span class="sec" style="font-weight:400">authored by R. Tufts · 10:20</span></div><div class="row" style="gap:6px;flex-wrap:wrap;height:auto;padding:2px 0"><span class="termchip">W</span><span class="muted">+</span><span class="termchip">P1</span><span class="muted">+</span><span class="termchip">SE1 <span class="sec">0.3 g X</span></span><span class="btn compact text">+ term</span></div><div class="row" style="gap:6px;height:auto;padding:4px 0 2px"><span class="sec" style="font-size:12px">Terms:</span>${["W", "P1", "P2", "T1", "T2", "SE1", "SUS", "OPE1", "OPE2"].map((t) => `<span class="termchip dim">${t}</span>`).join("")}</div><hr><div class="row"><span class="sec" style="width:90px">Stress type</span><span class="combo">Occasional</span><span class="sec" style="width:40px;margin-left:12px">Rule</span><span class="combo">OCC-A1</span><span class="sec" style="font-size:12px">sample-rules 1.2</span></div><div class="foot"><span class="btn compact text">Cancel <span class="muted">⎋</span></span><span class="btn compact primary">Done <span style="opacity:.8">↩</span></span></div></div>`;
  const tbl = U.casesTable({ selected: "OCC1", editing: "OCC1", composer });
  const head = `<div class="tblhead"><span class="name">Cases</span><span class="sec">generated from rule pack <span class="mono">sample-rules 1.2 · sha256:9b1c4e02…</span> on 2026-09-17 10:05</span><span class="btn compact">${icon("rules")}Generate from rule pack…</span><span class="grow"></span><span class="chipcap">${mark("displayonly", "var(--text-muted)")} Rule expression: Display only, not accepted as input</span></div>`;
  const surfaces = `<div class="tableview"><div class="region">${head}${U.tabs(LOADS_TABS, "Cases")}<div class="tblscroll">${tbl.html}</div>${U.tfoot([{ n: 6, label: "cases" }, { n: 4, label: "generated" }, { n: 1, label: "edited" }, { n: 1, label: "authored" }, { n: 4, label: "with a rule" }, { n: 2, label: "load sets" }], ["Origins", "Seismic SE1 · 0.3 g X"])}</div></div>`;
  return shell({
    toolbar: { project: "Loop 4 header", saveState: "edited", view: "Table", run: "enabled", issues },
    rail: { current: "Loads", disabled: noRun, issues },
    surfaces,
    status: { chips: [], issues, selection: "Case OCC1 · Expression" },
  });
}

// ---------- state 6 ----------
function s6() {
  const issues = { count: 2, worst: "warning" };
  const banner = `<div class="banner warn">${mark("warning")}<span>Run 02 failed: nonlinear support at node 20 did not converge.</span><a>Show node 20</a></div>`;
  const head = `<div class="tblhead"><span class="name">Stresses</span><span class="sec">Run 02 · 15:02 · stopped in OPE1</span><span class="grow"></span><span class="btn compact text" aria-expanded="false">${icon("info")}Information</span></div>`;
  const empty = `<div class="emptystate"><div class="es"><b>No results</b>Run 02 stopped in OPE1 (case 3 of 6) at the iteration limit. A run's results are one immutable set, so a stopped run has none. Change the support at node 20 or the iteration limit in Run settings, then Run again.</div></div>`;
  const tables = `<div class="tables">${banner}${head}${U.tabs(RESULT_TABS, "Stresses")}${empty}${U.issuesDrawer(M.issuesAt.s6, { selected: M.issuesAt.s6[0], filter: "Nonlinear" })}</div>`;
  const fig = figure({ id: "s6", w: 603, h: 828, view: { A: 200, E: 28 }, selection: { node: 20 }, issueNode: 20, loadsNeutral: true, margin: { top: 110, right: 40, bottom: 80, left: 40 } });
  const canvas = `<div class="canvas" style="width:603px;position:relative">${fig.svg}${U.hud({ off: ["deform", "probe"] })}${MOCK_LABEL}</div>`;
  const surfaces = `<div class="bothview">${tables}${canvas}</div>`;
  const log = [["assemble", "ok", "0.2 s"], ["W", "ok", "0.4 s"], ["SUS", "ok", "0.3 s"], ["OPE1", "fail", "stopped at iteration 50 · node 20"], ["OPE2", "skip", "not run"], ["EXP1", "skip", "not run"], ["EXP2", "skip", "not run"], ["OCC1", "skip", "not run"], ["hangers", "skip", "not run"], ["rules", "skip", "not run"]];
  const runpop = `<div class="pop runlog" style="position:absolute;left:585px;top:50px;min-width:380px"><div class="ttl">Run 02 <span class="sec" style="font-weight:400">15:02:14 – 15:02:41 · settings S-02 · iteration limit 50</span></div>${log.map(([c, s, t]) => `<div class="r"><span class="${s}">${s === "ok" ? icon("check", "s12") : s === "fail" ? icon("close", "s12") : "–"}</span><span class="c">${c}</span><span class="t">${t}</span></div>`).join("")}<div class="foot"><span class="btn compact text">Run settings…</span><span class="btn compact">Run record</span><span class="btn compact primary">${icon("run", "s12")}Run again</span></div></div>`;
  const chipPop = statusPop({ left: 8, raw: "MODEL_INCOMPLETE", cls: "incomplete", label: "Model incomplete", domain: "Solver", run: "Run 02 · 2026-09-17 15:02 · failed", reason: "The nonlinear support at node 20 did not converge within the iteration limit (50), so the mechanics of this model are not solved." });
  return shell({
    toolbar: { project: "Loop 4 header", saveState: "saved", view: "Both", run: "enabled", issues },
    rail: { current: "Results", disabled: { Review: "No solved run" }, caps: { Results: { text: "Failed", failed: true } }, issues },
    surfaces,
    status: { chips: [CH.incomplete], issues, selection: "Node 20 · 1 row" },
    overlays: runpop + chipPop,
  });
}

// ---------- state 7 ----------
function s7both() {
  const issues = { count: 1, worst: "warning" };
  const rows = M.stressRows("EXP1");
  const tbl = U.stressTable({ rows, selected: 70 });
  const head = U.resultsHead({ name: "Stresses", run: "Run 03 · solved 15:21 · immutable", caseSel: "EXP1", envelope: false, evidence: "INTERNALLY_VERIFIED", discOpen: true, identity: ID03 });
  const tables = `<div class="tables">${head}${U.tabs(RESULT_TABS, "Stresses")}<div class="tblscroll">${tbl.html}</div>${U.tfoot([{ n: 15, label: "elements" }, { n: 6, label: "cases solved" }, { n: "0.72", label: "governing · EXP1 · node 70 · EXP-A1 · pack 1.2" }], ["Sorted by ratio"])}</div>`;
  const ratios = Object.fromEntries(rows.map((r) => [r.node, r.ratio]));
  const fig = figure({ id: "s7", w: 603, h: 828, view: { A: 200, E: 28 }, colorBy: { ratios }, selection: { element: [60, 70] }, loads: false, margin: { top: 100, right: 40, bottom: 80, left: 40 } });
  const p70 = fig.P[70];
  let px = Math.round(p70.x + 18), py = Math.round(p70.y + 18);
  px = Math.min(px, 603 - 300 - 8); py = Math.min(py, 828 - 262);
  if (px + 300 > 603 - 236 && py < 300) py = 300;
  const probe = U.probe({ x: px, y: py, node: 70, element: "60–70", case: "EXP1", stress: 148.9, allowable: 206.8, ratio: 0.72, rule: "EXP-A1", pack: "1.2", run: "Run 03" });
  const canvas = `<div class="canvas" style="width:603px;position:relative">${fig.svg}${U.hud({ pressed: ["probe"] })}${U.legend({ quantity: "Ratio · expansion", caseName: "EXP1", value: 0.72, range: 1.0, node: 70, rule: "EXP-A1", pack: "1.2" })}${probe}${MOCK_LABEL}</div>`;
  const surfaces = `<div class="bothview">${tables}${canvas}</div>`;
  return shell({
    toolbar: { project: "Loop 4 header", saveState: "saved", view: "Both", run: "enabled", issues },
    rail: { current: "Results", issues },
    surfaces,
    status: { chips: [CH.solved, CH.rules], issues, selection: "Element 60–70 · node 70" },
  });
}

function s7table() {
  const issues = { count: 1, worst: "warning" };
  const rows = M.envelopeRows();
  const menu = `<div class="pop cellmenu" style="min-width:260px"><div class="ttl">Ratio <span class="sec" style="font-weight:400">= Stress / Allowable</span></div><div class="row"><span class="k on">${icon("check", "s12")}</span>Sort descending<span class="key">⌘↓</span></div><div class="row"><span class="k"></span>Sort ascending<span class="key">⌘↑</span></div><hr><div class="row"><span class="k on">${icon("check", "s12")}</span>Data bar · 1.0 tick</div><div class="row"><span class="k"></span>Colour by scale</div><hr><div class="row">Filter ratio ≥ <span class="input n" style="width:64px;height:22px">0.50</span><span class="key">3 of 15 rows</span></div><div class="row"><span class="k"></span>Unchecked rows only</div><hr><div class="row sec" style="font-size:12px">Rule and pack are their own columns · width 120 · <a>reset</a></div></div>`;
  const tbl = U.stressTable({ rows, selected: 70, menuOpen: menu });
  const head = U.resultsHead({ name: "Stresses", run: "Run 03 · solved 15:21 · immutable", caseSel: "EXP1", envelope: true, evidence: "INTERNALLY_VERIFIED", discOpen: false, identity: ID03 }).replace(`<span class="combo">Case: EXP1</span>`, `<span class="combo" style="color:var(--text-disabled)" title="The case selector is inactive while Envelope is on">Case: all</span>`);
  const surfaces = `<div class="tableview"><div class="region">${head}${U.tabs(RESULT_TABS, "Stresses")}<div class="tblscroll">${tbl.html}</div>${U.tfoot([{ n: 15, label: "elements" }, { n: 6, label: "cases in the envelope" }, { n: "0.72", label: "governing · EXP1 · node 70 · EXP-A1 · pack 1.2" }], ["Envelope: governing case per element", "Sorted by ratio"])}</div></div>`;
  return shell({
    toolbar: { project: "Loop 4 header", saveState: "saved", view: "Table", run: "enabled", issues },
    rail: { current: "Results", issues },
    surfaces,
    status: { chips: [CH.solved, CH.rules], issues, selection: "Element 60–70 · node 70" },
  });
}

// ---------- state 8 ----------
function s8table() {
  const issues = { count: 1, worst: "warning" };
  const tbl = U.hangerTable({ stale: true, selected: 80, pending: true });
  const head = U.resultsHead({ name: "Hangers", run: "Run 03 · solved 15:21 · immutable", evidence: "INTERNALLY_VERIFIED", discOpen: false, identity: ID03 });
  const band = `<div class="banner warn">${mark("stale", "var(--mark-checkedStale)")}<span>Model changed since Run 03: proposal P-12 row 1 accepted at 16:31 (variable spring at node 80). The values below are from the model as solved; Run 03 stays the solve basis until the next run.</span><a>Run again</a></div>`;
  const lib = `<div class="stagecap">${icon("libraries", "s12")}Hanger library <b style="font-weight:500;color:var(--text-primary)">Vendor-A springs</b> · user import 2026-09-15 · 24 sizes · source recorded in Libraries · design rule: cold load = hot load + rate × travel · max variation 25 %</div>`;
  const surfaces = `<div class="tableview"><div class="region">${head}${band}${U.tabs(RESULT_TABS, "Hangers")}${lib}<div class="tblscroll">${tbl.html}</div>${U.tfoot([{ n: 2, label: "hanger locations" }, { n: 1, label: "designed in Run 03" }, { n: 1, label: "not yet designed" }, { n: 1, label: "proposal row pending", color: "var(--proposal-new)" }], ["Library: Vendor-A springs"])}</div></div>`;
  const card = U.proposalCard(M.proposals[1], { decisions: { 0: "accepted" } });
  const body = `${card}<div class="qa">${mark("accepted", "var(--mark-origin)")}Accepted record · P-09 (11:40) · P-12 row 1 (16:31)</div><div class="qa">${icon("agent", "s12")}Ask the agent… <span class="muted">⌘⇧G</span></div>`;
  return shell({
    agent: "column",
    agentCol: { tab: "Proposals", counts: { proposals: 1, checks: 0, accepted: 2 }, state: "Idle · P-12 open", body },
    toolbar: { project: "Loop 4 header", saveState: "edited", view: "Table", run: "enabled", issues, agentOpen: true },
    rail: { current: "Results", caps: { Results: { text: "Stale", stale: true } }, issues },
    surfaces,
    status: { chips: [], issues, selection: "Node 80 · 1 row" },
  });
}

function s8model() {
  const issues = { count: 1, worst: "warning" };
  const fig = figure({ id: "s8", w: 1000, h: 548, ghost: { node: 80, id: "P-12" }, selection: { node: 80 }, loadsNeutral: true, margin: { top: 70, right: 60, bottom: 92, left: 60 } });
  const canvas = canvasBox(1000, 548, `${fig.svg}${U.hud({})}${MOCK_LABEL}`);
  const prop = { 80: { fields: { Type: ["Rigid (Y)", "Variable spring"], Library: ["—", "Vendor-A springs"], "Max variation": ["—", "25"] } }, 20: { fields: { Gap: ["3", "0"] } } };
  const tbl = U.restraintsTable({ prop, selected: 80, noAdd: true, noCnode: true, noNote: true });
  const drawer = `<div class="drawer">${U.tabs(MODEL_TABS({ layout: 16, restraints: 7, nodeData: 2 }), "Restraints")}<div class="tblscroll">${tbl.html}</div>${U.tfoot([{ n: 7, label: "restraints" }, { n: 2, label: "proposed rows · P-12 · draft until accepted", color: "var(--proposal-new)" }], ["Accept row ⌘⇧A", "Reject row ⌘⇧R", "Origins"])}</div>`;
  const surfaces = `<div class="modelview"><div class="left">${canvas}${drawer}</div><div class="inspector">${U.inspector({ node: 80, proposedRestraint: true, issues: "none", when: "2026-09-17 10:52" })}</div></div>`;
  return shell({
    agentStrip: { badge: 1 },
    toolbar: { project: "Loop 4 header", saveState: "saved", view: "Model", run: "enabled", issues },
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
  const body = rs.map((r) => `<tr>${U.gutter(r.origin === "accepted" ? "accepted" : null, r.node === 20 ? "comment" : null, { origin: `Accepted from proposal ${r.proposal} by R. Tufts`, state: "Comment · Open issue · P-12 row 2 pending" })}<td class="n">${r.node}</td><td>${r.tag}</td><td>${esc(r.type)}</td><td class="n">${r.gap != null ? num(r.gap) : "—"}</td><td class="n">${r.mu != null ? r.mu.toFixed(2) : "—"}</td><td>${r.library || "—"}</td><td class="n">${r.maxVar != null ? num(r.maxVar) : "—"}</td></tr>`).join("");
  return `<table class="ds" style="width:${width}px"><colgroup>${cols.map((c) => `<col style="width:${c[1]}px">`).join("")}</colgroup><thead><tr>${cols.map((c) => c[0] === "gut" ? `<th class="gut"></th>` : `<th class="${c[2] || ""}">${c[0]}${c[3] ? ` <span class="u">[${c[3]}]</span>` : ""}</th>`).join("")}</tr></thead><tbody>${body}</tbody></table>`;
}

function s9() {
  const issues = { count: 1, worst: "warning" };
  const outline = M.review.outline.map((o) => `<div class="${o.on ? "on" : ""}"><span class="num">${o.n}</span><span>${esc(o.name)}</span>${o.fixed ? `<span class="lock" title="Fixed section from the record">${icon("lock")}</span>` : ""}${o.state ? `<span class="st ${o.state}">${o.state}</span>` : ""}</div>`).join("");
  const cases = U.casesTable({ noRuleExpr: true, noAdd: true });
  const content = `<div class="content"><h3>4. Model <span class="stt">edited · iteration 2</span></h3><p>Loop 4 header runs from the pump P-401 discharge nozzle (node 10, anchor N1) to the vessel V-402 nozzle (node 130, anchor N2), with a branch from the welding tee at node 70 to the header tie-in at node 160 (anchor N3): 16 node rows, 15 elements, sections P1 (DN200), P2 (DN150) and P3 (DN100), materials CS-A and CS-B.</p><p>Vertical support is a +Y resting support with a 3 mm gap at node 20 (as surveyed), the variable spring hanger H1 at node 60 (Vendor-A size A-3) and <span class="del">a rigid rod RS-02 at node 80 from the steel</span> <span class="ins">a variable spring hanger RS-02 at node 80 (Vendor-A, design load 4 180 N; accepted from proposal P-12 row 1 on 2026-09-17 16:31)</span>. A guide G-01 with a 2 mm gap holds the riser at node 120. <span class="del">The OPE1 load on the vessel nozzle was 7.4 kN.</span> <span class="ins">The OPE1 load on the vessel nozzle is 3.9 kN in Run 04.</span></p><div class="cap">${icon("lock")}Live from the model record · Restraints · nodes 20, 60, 80 · not editable here<span class="grow"></span><span class="editsleg"><span class="ins">inserted</span><span class="del">removed</span> since Iteration 1</span></div><div class="live">${liveRestraints([20, 60, 80])}</div><h3>5. Load cases <span class="stt">drafted</span></h3><p>Six cases from rule pack sample-rules 1.2, one edited (EXP1 as OPE1 − SUS) and one authored (OCC1 with SE1 at 0.3 g in X).</p><div class="cap">${icon("lock")}Live from the model record · Cases · not editable here</div><div class="live">${cases.html}</div></div>`;
  const comments = `<div class="comments"><div class="hd">Comments <span class="n">4</span><span class="grow" style="flex:1"></span><span class="btn compact text">+ Comment</span></div><div class="filters"><span class="chip outline" style="background:var(--selection-band);color:var(--accent-text);border-color:transparent">All 4</span><span class="chip outline">Open 3</span><span class="chip outline">Resolved 1</span><span class="chip outline">Checks 2</span><span class="chip outline">Notes 1</span><span class="chip outline">Mine 1</span></div>${M.review.comments.map(U.commentCard).join("")}</div>`;
  const page = `<div class="reviewpage"><div class="hd"><span class="ttl">${esc(M.review.title)}</span><span class="combo">Iteration 2 · 09:40 · Run 04</span><span class="sec" style="font-size:12px">compared with Iteration 1 · 2026-09-17 15:40 · Run 03</span><span class="switch on"><i></i>Show edits</span><span class="grow" style="flex:1"></span><span class="btn compact">${icon("results", "s12")}Report preview</span><span class="btn compact">Export…</span></div><div class="accept">${ACCEPT}</div><div class="cols"><div class="outline">${outline}<div style="border:0;color:var(--text-muted);font-size:12px;margin-top:6px">${icon("lock", "s12")} fixed sections come from the record</div></div>${content}${comments}</div></div>`;
  return shell({
    agentStrip: { badge: 1 },
    toolbar: { project: "Loop 4 header", saveState: "saved", view: "Table", run: "enabled", issues },
    rail: { current: "Review", issues },
    surfaces: page,
    status: { chips: [CH.rules, CH.review], issues, selection: "Section 4 · Model" },
  });
}

export const frames = [
  { file: "s1_table_light", state: 1, stateName: "New project, the first node row", stage: "Model", view: "Table", theme: "light", look: "The required marks on row 20, the Run button's reason in its tooltip, the empty gutter and marks column, and the status bar's information popover with the maturity line.", body: s1, tests: "State 1 (Table): an empty model's first row with the required marks and the Run button's reason; the gutter and marks column empty." },
  { file: "s2_model_light", state: 2, stateName: "Routing in the canvas with direct distance entry", stage: "Model", view: "Model", theme: "light", look: "The routing compass at node 40 with the +Y axis active, the length field with 1500 mm, the draft ghost to node 50, and the draft row appearing in the drawer.", body: s2, tests: "State 2 (Model, also dark): the compass and direct distance entry writing rows into the drawer; the draft ghost against the dark ground; label budget on a small model." },
  { file: "s2_model_dark", state: 2, stateName: "Routing in the canvas with direct distance entry", stage: "Model", view: "Model", theme: "dark", look: "The same moment under the dark tokens: the draft ghost and compass against the dark ground, the label budget on a small model.", body: s2, tests: "State 2 (Model, also dark): the draft ghost against the dark ground; label budget on a small model." },
  { file: "s3_table_light", state: 3, stateName: "Editing the layout table: propagation, keyboard entry, the paste band", stage: "Model", view: "Table", theme: "light", look: "Propagation ticks on Section, Material and Load; the read-through T and P columns; the edit cell on row 130 DY; the paste band with two ignored columns and its three-row preview; the footer counts.", body: s3, tests: "State 3 (Table): propagation marks after keyboard entry; the paste band with two unmapped columns; the footer counts." },
  { file: "s4_both_light", state: 4, stateName: "Restraint and load tables; marks on the node rows; glyphs; the inspector slide-over", stage: "Model", view: "Both", theme: "light", look: "Restraint, load and node-data marks in the marks column; the hover card on the open restraint mark; the glyphs with direction arrows, the gap and friction labels in the canvas; the inspector slide-over for node 20 over the canvas; checked and stale marks in the gutter.", body: s4both, tests: "State 4 (Both, also Table): restraint and load marks on node rows, the glyphs with their direction arrows, gap and friction labels; the inspector slide-over open." },
  { file: "s4_table_light", state: 4, stateName: "Restraint and load tables; marks on the node rows", stage: "Model", view: "Table", theme: "light", look: "The load mark on node 40 and the restraint mark on node 20 opened as joined rows in place; the read-through columns; the provenance tick on the CS-B cell at node 140.", body: s4table, tests: "State 4 (also Table): restraint and load marks on node rows; the joined rows the marks open." },
  { file: "s5_table_light", state: 5, stateName: "Load cases: generated rows edited, an authored case, the display-only expression", stage: "Loads", view: "Table", theme: "light", look: "Generated rows with the origin glyph, EXP1 edited (was T1), OCC1 authored with its combination editor open; the rule-expression column in its dashed display-only frame with the caption once in the header.", body: s5, tests: "State 5 (Table): generated, edited and authored case rows with their origin glyphs; the display-only expression frame." },
  { file: "s6_both_light", state: 6, stateName: "Running, then the failure banner, the issues drawer and the status chip", stage: "Results", view: "Both", theme: "light", look: "The run log under the Run button ending in the OPE1 stop; the failure banner with Show node 20; the issues drawer open on the Nonlinear class; node 20 selected in the canvas; the status chip Model incomplete with its popover open.", body: s6, tests: "State 6 (Both): the running Run button, then the failure banner and the issues drawer; the status chip and its popover." },
  { file: "s7_both_light", state: 7, stateName: "Results: the stress table, the case selector and Envelope, the evidence chip, the header disclosure; the coloured model", stage: "Results", view: "Both", theme: "light", look: "The results header with the case selector, the Envelope switch, the evidence chip and the information disclosure open with the acceptance sentence and the run identity; the stress table with data bars and the 1.0 tick; the coloured model with the legend and the pinned probe at node 70.", body: s7both, tests: "State 7 (Both, also Table, also dark): the stress table with data bars, the case selector and Envelope, the evidence chip, the results header disclosure open; the coloured model with the legend and probe." },
  { file: "s7_table_light", state: 7, stateName: "Results in the Table view with Envelope on", stage: "Results", view: "Table", theme: "light", look: "Envelope on: the governing case per element; the Ratio column menu open; the disclosure closed; the footer with the governing ratio, rule ID and pack version.", body: s7table, tests: "State 7 (also Table): the stress table with data bars, the case selector and Envelope, the evidence chip." },
  { file: "s7_both_dark", state: 7, stateName: "Results in the Both view under the dark tokens", stage: "Results", view: "Both", theme: "dark", look: "The same screen as s7_both_light under the dark tokens: the result scale on the tubes, the legend and the probe in the dark theme.", body: s7both, tests: "State 7 (also dark): the coloured model with the legend and probe in both themes." },
  { file: "s8_table_light", state: 8, stateName: "Hanger design against a user library; a proposal accepted row by row", stage: "Results", view: "Table", theme: "light", look: "The Hangers table with H1 designed from the Vendor-A library and its values hatched stale since the model changed; the agent column open with proposal P-12, row 1 accepted at 16:31 and row 2 pending; the Accept remaining and Reject remaining controls; the Accepted count.", body: s8table, tests: "State 8 (Table, also Model): the hanger table against a user-supplied library; Accept row by row; the Accepted record." },
  { file: "s8_model_light", state: 8, stateName: "A proposal landing as proposed rows and as a ghost in the canvas", stage: "Model", view: "Model", theme: "light", look: "Proposal P-12 as banded rows in the Restraints drawer with old and new values; the ghost spring beside the rigid rod at node 80 in the canvas; the proposed line in the inspector; the agent strip badge.", body: s8model, tests: "State 8 (also Model): a proposal landing as banded rows with old and new values and as a ghost in the canvas." },
  { file: "s9_table_light", state: 9, stateName: "The Review page", stage: "Review", view: "Table", theme: "light", look: "Three columns: the outline with fixed and editable sections, the content at section 4 with edits shown and two live tables, the comment stream with filters; the iteration selector; the acceptance sentence once at the top.", body: s9, tests: "State 9 (Table, also dark): the three-column Review page with live tables, edits shown and hidden, two iterations, the comment stream, and the acceptance sentence once at the top." },
  { file: "s9_table_dark", state: 9, stateName: "The Review page under the dark tokens", stage: "Review", view: "Table", theme: "dark", look: "The same Review page under the dark tokens: the inserted and removed text, the live tables and the comment cards in the dark theme.", body: s9, tests: "State 9 (also dark): the Review page in the dark theme." },
];
