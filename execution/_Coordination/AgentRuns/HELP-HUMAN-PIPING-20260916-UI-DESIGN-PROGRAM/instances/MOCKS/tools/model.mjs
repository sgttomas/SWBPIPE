// The one fictional sample model that runs through every frame. Every value here is a
// placeholder chosen to be plausible to a stress engineer; nothing is engine output.
// Units: mm, °C, bar, N, N·m, MPa, N/mm. Vertical axis Y.

export const project = {
  name: "Loop 4 header",
  engineer: "R. Tufts",
  unitsLabel: "SI",
  vertical: "Y",
  description: "Pump P-401 discharge header to vessel V-402, with a branch to the header tie-in at N3.",
};

export const sections = [
  { name: "P1", nominal: "DN200", od: 219.1, wall: 8.2, ca: 1.5, millTol: 12.5, insThk: 50, insDensity: 130, origin: "Imported · sections.csv · 2026-09-16", bendR: 330 },
  { name: "P2", nominal: "DN150", od: 168.3, wall: 7.1, ca: 1.5, millTol: 12.5, insThk: 50, insDensity: 130, origin: "Imported · sections.csv · 2026-09-16", bendR: 250 },
  { name: "P3", nominal: "DN100", od: 114.3, wall: 6.0, ca: 1.5, millTol: 12.5, insThk: 40, insDensity: 130, origin: "Imported · sections.csv · 2026-09-16", bendR: 170 },
];

export const materials = [
  { name: "CS-A", desc: "Carbon steel A (sample)", E20: 203000, E150: 196000, alpha: 12.0, density: 7850, poisson: 0.3, Sc: 137.9, Sh150: 137.9, Sh80: 137.9, source: "user library cs-a.mlb · provenance recorded" },
  { name: "CS-B", desc: "Carbon steel B (sample)", E20: 203000, E150: 196000, alpha: 11.7, density: 7850, poisson: 0.3, Sc: 128.0, Sh150: 128.0, Sh80: 128.0, source: "not recorded (provenance warning)" },
];

export const loadSets = [
  { name: "OP1", T1: 150, P1: 12.0, T2: 40, P2: 12.0, sg: 1.0, note: "operating / standby" },
  { name: "OP2", T1: 80, P1: 6.0, T2: 40, P2: 6.0, sg: 1.0, note: "branch, operating / standby" },
];

// One row per node, carrying the element that arrives at it (the CAEPIPE grammar).
// `entered` lists the columns typed on that row; the rest of Section, Material and
// Load propagate from the row above by connectivity. The start node (10) carries no
// element, so its Section, Material and Load are empty; the first element (10–20) is
// where they are entered.
export const rows = [
  { node: 10, from: null, type: null, dx: 0, dy: 0, dz: 0, abs: true, section: null, material: null, load: null, entered: [] },
  { node: 20, from: 10, type: "Pipe", dx: 3000, dy: 0, dz: 0, section: "P1", material: "CS-A", load: "OP1", entered: ["section", "material", "load"] },
  { node: 30, from: 20, type: "Bend", dx: 1500, dy: 0, dz: 0, section: "P1", material: "CS-A", load: "OP1", entered: [], bendR: 330 },
  { node: 40, from: 30, type: "Pipe", dx: 0, dy: 2500, dz: 0, section: "P1", material: "CS-A", load: "OP1", entered: [] },
  { node: 50, from: 40, type: "Bend", dx: 0, dy: 1500, dz: 0, section: "P1", material: "CS-A", load: "OP1", entered: [], bendR: 330 },
  { node: 60, from: 50, type: "Pipe", dx: 0, dy: 0, dz: 1200, section: "P1", material: "CS-A", load: "OP1", entered: [] },
  { node: 70, from: 60, type: "Pipe", dx: 0, dy: 0, dz: 1300, section: "P1", material: "CS-A", load: "OP1", entered: [] },
  { node: 80, from: 70, type: "Pipe", dx: 0, dy: 0, dz: 1400, section: "P1", material: "CS-A", load: "OP1", entered: [] },
  { node: 90, from: 80, type: "Valve", dx: 0, dy: 0, dz: 600, section: "P1", material: "CS-A", load: "OP1", entered: [], valveWeight: 1200 },
  { node: 100, from: 90, type: "Reducer", dx: 0, dy: 0, dz: 400, section: "P1", material: "CS-A", load: "OP1", entered: [], reducer: { od1: 219.1, thk1: 8.2, od2: 168.3, thk2: 7.1 } },
  { node: 110, from: 100, type: "Bend", dx: 0, dy: 0, dz: 1600, section: "P2", material: "CS-A", load: "OP1", entered: ["section"], bendR: 250 },
  { node: 120, from: 110, type: "Pipe", dx: 0, dy: -1500, dz: 0, section: "P2", material: "CS-A", load: "OP1", entered: [] },
  { node: 130, from: 120, type: "Pipe", dx: 0, dy: -1500, dz: 0, section: "P2", material: "CS-A", load: "OP1", entered: [] },
  { node: 140, from: 70, type: "Pipe", dx: 2000, dy: 0, dz: 0, section: "P3", material: "CS-B", load: "OP2", entered: ["section", "material", "load"], branch: true },
  { node: 150, from: 140, type: "Bend", dx: 1000, dy: 0, dz: 0, section: "P3", material: "CS-B", load: "OP2", entered: [], bendR: 170 },
  { node: 160, from: 150, type: "Pipe", dx: 0, dy: -3000, dz: 0, section: "P3", material: "CS-B", load: "OP2", entered: [] },
];

// Absolute coordinates by walking the rows.
export function coordinates() {
  const c = {};
  for (const r of rows) {
    if (r.from == null) c[r.node] = { x: r.dx, y: r.dy, z: r.dz };
    else { const f = c[r.from]; c[r.node] = { x: f.x + r.dx, y: f.y + r.dy, z: f.z + r.dz }; }
  }
  return c;
}

export const restraints = [
  { node: 10, tag: "N1", type: "Anchor", direction: "all", gap: null, mu: null, stiffness: "rigid", cnode: null, note: "pump P-401 discharge nozzle", origin: "entered" },
  { node: 20, tag: "RS-01", type: "+Y", direction: "Y", gap: 3, mu: 0.3, stiffness: "rigid", cnode: null, note: "resting support, 3 mm as surveyed", origin: "entered" },
  { node: 60, tag: "H1", type: "Variable spring", direction: "Y", gap: null, mu: null, stiffness: "designed", cnode: null, library: "Vendor-A springs", maxVar: 25, note: "to be designed", origin: "accepted", proposal: "P-09" },
  { node: 80, tag: "RS-02", type: "Rigid (Y)", direction: "Y", gap: 0, mu: null, stiffness: "rigid", cnode: null, note: "rod from steel", origin: "entered" },
  { node: 120, tag: "G-01", type: "Guide", direction: "X, Z", gap: 2, mu: 0.3, stiffness: "rigid", cnode: null, note: "", origin: "entered" },
  { node: 130, tag: "N2", type: "Anchor", direction: "all", gap: null, mu: null, stiffness: "rigid", cnode: null, note: "vessel V-402 nozzle", origin: "entered" },
  { node: 160, tag: "N3", type: "Anchor", direction: "all", gap: null, mu: null, stiffness: "rigid", cnode: null, note: "header tie-in", origin: "entered" },
];

export const loads = [
  { node: 40, kind: "Force", direction: "Y", value: -2000, unit: "N", case: "W", note: "instrument bridle weight", origin: "entered" },
  { node: 130, kind: "Displacement", direction: "Y", value: 4.0, unit: "mm", case: "T1", note: "vessel growth at the nozzle", origin: "entered" },
  { node: 130, kind: "Displacement", direction: "Y", value: 1.2, unit: "mm", case: "T2", note: "vessel growth, standby", origin: "entered" },
];

export const nodeData = [
  { node: 70, kind: "Branch connection", value: "Welding tee", detail: "SIF from rule pack sample-rules 1.2", origin: "entered" },
  { node: 90, kind: "Flange", value: "WN", detail: "weight 220 N", origin: "entered" },
];

export const rulePack = { name: "sample-rules", version: "1.2", sha: "sha256:9b1c4e02…", source: "user · private · invented values for this sample, no code content" };

export const rules = [
  { id: "SUS-A1", name: "Sustained", expr: "S_L ≤ S_h", allowable: { "CS-A": 137.9, "CS-B": 128.0 } },
  { id: "EXP-A1", name: "Expansion range", expr: "S_E ≤ f · (1.25 S_c + 0.25 S_h)", allowable: { "CS-A": 206.8, "CS-B": 192.0 } },
  { id: "OCC-A1", name: "Occasional", expr: "S_L + S_O ≤ 1.33 S_h", allowable: { "CS-A": 183.4, "CS-B": 170.2 } },
];

// Load cases as generated by sample-rules 1.2 from load sets OP1/OP2 (thermal states T1, T2),
// one row edited by the engineer and one authored.
export const cases = [
  { name: "SUS", expr: "W + P1", type: "Sustained", rule: "SUS-A1", origin: "generated", who: "sample-rules 1.2", when: "2026-09-17 10:05" },
  { name: "OPE1", expr: "W + P1 + T1", type: "Operating", rule: null, origin: "generated", who: "sample-rules 1.2", when: "2026-09-17 10:05" },
  { name: "OPE2", expr: "W + P2 + T2", type: "Operating", rule: null, origin: "generated", who: "sample-rules 1.2", when: "2026-09-17 10:05" },
  { name: "EXP1", expr: "OPE1 − SUS", type: "Expansion", rule: "EXP-A1", origin: "edited", who: "R. Tufts", when: "2026-09-17 10:12", was: "T1" },
  { name: "EXP2", expr: "T2", type: "Expansion", rule: "EXP-A1", origin: "generated", who: "sample-rules 1.2", when: "2026-09-17 10:05" },
  { name: "OCC1", expr: "W + P1 + SE1", type: "Occasional", rule: "OCC-A1", origin: "authored", who: "R. Tufts", when: "2026-09-17 10:20" },
];

export const seismic = [{ name: "SE1", g: 0.3, direction: "X", note: "static, user-supplied factor" }];

export const runs = [
  { id: "Run 01", when: "2026-09-16 17:48", state: 4, outcome: "solved", note: "weight only, superseded by model changes" },
  { id: "Run 02", when: "2026-09-17 15:02", state: 7, outcome: "failed", note: "nonlinear support at node 20 did not converge after 50 iterations", settings: "S-02 · iteration limit 50" },
  { id: "Run 03", when: "2026-09-17 15:21", state: 7, outcome: "solved", note: "6 cases · hanger design pass · rules checked", settings: "S-03 · iteration limit 200 · converged in 74 iterations", sha: "sha256:4df0f798…", solver: "0.2.0", evidence: "INTERNALLY_VERIFIED" },
  { id: "Run 04", when: "2026-09-18 09:12", state: 8, outcome: "solved", note: "after P-12 row 1 (variable spring at 80)", settings: "S-03", sha: "sha256:b71e20c4…", solver: "0.2.0", evidence: "INTERNALLY_VERIFIED" },
];

// Elements in file order: [from, to].
export const elements = rows.filter((r) => r.from != null).map((r) => [r.from, r.node]);

function matOf(node) { return rows.find((r) => r.node === node).material; }

// Run 03 stresses [MPa] at the To node of each element, per case. Ratios are computed
// against the rule allowable for the element's material.
const S = {
  EXP1: { 20: 26.4, 30: 118.3, 40: 64.1, 50: 131.0, 60: 58.8, 70: 148.9, 80: 96.2, 90: 33.5, 100: 30.1, 110: 89.6, 120: 52.4, 130: 47.9, 140: 77.3, 150: 42.6, 160: 21.7 },
  SUS: { 20: 41.5, 30: 35.0, 40: 48.2, 50: 33.1, 60: 36.9, 70: 55.2, 80: 61.3, 90: 46.0, 100: 31.8, 110: 29.2, 120: 27.5, 130: 24.0, 140: 38.4, 150: 26.1, 160: 22.8 },
  EXP2: { 20: 4.1, 30: 18.2, 40: 9.9, 50: 20.2, 60: 9.1, 70: 22.9, 80: 14.8, 90: 5.2, 100: 4.6, 110: 13.8, 120: 8.1, 130: 7.4, 140: 11.9, 150: 6.6, 160: 3.3 },
  OCC1: { 20: 70.2, 30: 52.4, 40: 81.0, 50: 49.7, 60: 55.4, 70: 88.1, 80: 92.5, 90: 69.0, 100: 47.7, 110: 43.8, 120: 41.3, 130: 36.0, 140: 57.6, 150: 39.2, 160: 34.2 },
};
const RULE_OF = { EXP1: "EXP-A1", EXP2: "EXP-A1", SUS: "SUS-A1", OCC1: "OCC-A1" };

export function stressRows(caseName) {
  const rule = rules.find((r) => r.id === RULE_OF[caseName]);
  return elements.map(([f, t]) => {
    const s = S[caseName][t];
    const allow = rule.allowable[matOf(t)];
    return { node: t, element: `${f}–${t}`, case: caseName, stress: s, allowable: allow, ratio: s / allow, rule: rule.id, pack: rulePack.version };
  }).sort((a, b) => b.ratio - a.ratio);
}

export function envelopeRows() {
  return elements.map(([f, t]) => {
    let best = null;
    for (const c of Object.keys(S)) {
      const rule = rules.find((r) => r.id === RULE_OF[c]);
      const allow = rule.allowable[matOf(t)];
      const ratio = S[c][t] / allow;
      if (!best || ratio > best.ratio) best = { node: t, element: `${f}–${t}`, case: c, stress: S[c][t], allowable: allow, ratio, rule: rule.id, pack: rulePack.version };
    }
    return best;
  }).sort((a, b) => b.ratio - a.ratio);
}

export const governing = { node: 70, element: "60–70", case: "EXP1", stress: 148.9, allowable: 206.8, ratio: 0.72, rule: "EXP-A1", pack: "1.2" };

// Run 03 displacements in OPE1 [mm], selected nodes (for the rationale and the review text).
export const displacementsOPE1 = { 40: { dy: 3.9 }, 50: { dy: 6.2 }, 60: { dy: 5.9 }, 70: { dy: 4.1 }, 80: { dy: 0.0 }, 120: { dy: 4.3 }, 130: { dy: 4.0 } };

// Run 03 restraint loads [kN, kN·m], selected.
export const restraintLoads = {
  80: { W: { fy: 4.18 }, OPE1: { fy: -9.8 } },
  130: { OPE1: { fy: 7.4, mz: 3.1 } },
  20: { W: { fy: 2.9 }, OPE1: { fy: 0.0, note: "open (lift 2.1 mm > gap 3 mm? no: lift 2.1 mm, gap 3 mm, not engaged)" } },
};

// Hanger design pass, Run 03 (one hanger location, H1 at node 60).
export const hangers = [
  { node: 60, tag: "H1", type: "Variable spring", designLoad: 5980, travel: 5.9, library: "Vendor-A springs", size: "A-3", rate: 120, coldLoad: 6688, hotLoad: 5980, variation: 11.8, origin: "designed in Run 03" },
];

export const hangerLibrary = { name: "Vendor-A springs", imported: "2026-09-15", sizes: 24, provenance: "user import · source recorded in Libraries" };

// Run 04 (after P-12 row 1): the stress rows the Review page's live table shows (Envelope).
export const run04Envelope = [
  { node: 70, element: "60–70", case: "EXP1", stress: 119.9, allowable: 206.8, ratio: 0.58, rule: "EXP-A1", pack: "1.2" },
  { node: 50, element: "40–50", case: "EXP1", stress: 112.4, allowable: 206.8, ratio: 0.54, rule: "EXP-A1", pack: "1.2" },
  { node: 30, element: "20–30", case: "EXP1", stress: 103.8, allowable: 206.8, ratio: 0.50, rule: "EXP-A1", pack: "1.2" },
  { node: 80, element: "70–80", case: "OCC1", stress: 84.7, allowable: 183.4, ratio: 0.46, rule: "OCC-A1", pack: "1.2" },
  { node: 40, element: "30–40", case: "OCC1", stress: 79.3, allowable: 183.4, ratio: 0.43, rule: "OCC-A1", pack: "1.2" },
  { node: 110, element: "100–110", case: "EXP1", stress: 76.1, allowable: 206.8, ratio: 0.37, rule: "EXP-A1", pack: "1.2" },
];

// The proposals.
export const proposals = [
  {
    id: "P-09", title: "Add a variable spring hanger at node 60", when: "2026-09-17 11:40", state: "accepted",
    rows: [{ table: "Restraints", node: 60, fields: [["Type", "—", "Variable spring"], ["Library", "—", "Vendor-A springs"], ["Max variation", "—", "25 %"]], decision: "accepted", at: "2026-09-17 11:40" }],
    rationale: "The top of the riser at 60 rises about 6 mm at T1 with no vertical support between 20 and 80.",
  },
  {
    id: "P-12", title: "Replace the rigid support at node 80 with a variable spring", when: "2026-09-17 16:22",
    asked: { who: "R. Tufts", when: "16:20", text: "Nozzle load at 130 looks high in OPE1. Suggest support changes." },
    rows: [
      { table: "Restraints", node: 80, fields: [["Type", "Rigid (Y)", "Variable spring"], ["Library", "—", "Vendor-A springs"], ["Max variation", "—", "25 %"], ["Design load", "—", "4 180 N"], ["Travel", "—", "TBD"]] },
      { table: "Restraints", node: 20, fields: [["Gap", "3 mm", "0 mm"]] },
    ],
    rationale: "The rigid rod at 80 holds the header down against the riser's thermal rise (about 6 mm at T1): OPE1 reaction −9.8 kN against +4.18 kN in W, and 7.4 kN on the vessel nozzle (130). A spring set at the W load carries the weight and lets the header rise; closing the gap at 20 lets the +Y support carry weight in OPE1 too.",
    constraints: ["Design load: Run 03 load at 80 in W, 4.18 kN", "Travel: TBD until a run with the spring", "Library: Vendor-A springs (the only one)", "The gap at 20 is recorded as surveyed"],
    tbd: ["Travel at 80 · needs a run with the spring", "Keep the gap at 20 as surveyed? (row 2)"],
    validation: "Schema and constraints: passed",
  },
];

// Issues at each moment.
export const issuesAt = {
  s1: [
    { cls: "Blocks solve", sev: "blocking", msg: "Section missing", entity: "node 20" },
    { cls: "Blocks solve", sev: "blocking", msg: "Material missing", entity: "node 20" },
    { cls: "Blocks rule check", sev: "blocking", msg: "Load set (T, P) missing", entity: "node 20" },
  ],
  s3: [],
  s4: [{ cls: "Provenance", sev: "warning", msg: "Material CS-B · source not recorded", entity: "node 140" }],
  s6: [
    { cls: "Nonlinear", sev: "warning", msg: "+Y support at 20 alternated open / closed in OPE1 · iteration limit 50 reached", entity: "node 20" },
    { cls: "Provenance", sev: "warning", msg: "Material CS-B · source not recorded", entity: "node 140" },
  ],
  s7: [{ cls: "Provenance", sev: "warning", msg: "Material CS-B · source not recorded", entity: "node 140" }],
};

// Review page content (iteration 2, Run 04).
export const review = {
  title: "Review · Loop 4 header · Run 04",
  iterations: [{ name: "Iteration 1", when: "2026-09-17 15:40", run: "Run 03" }, { name: "Iteration 2", when: "2026-09-18 09:40", run: "Run 04", current: true }],
  outline: [
    { n: 1, name: "Notice", fixed: true, state: "" },
    { n: 2, name: "Identity", fixed: true, state: "" },
    { n: 3, name: "Libraries and rule packs", fixed: true, state: "" },
    { n: 4, name: "Model", fixed: false, state: "edited", on: true },
    { n: 5, name: "Load cases", fixed: false, state: "drafted" },
    { n: 6, name: "Assumptions and warnings", fixed: true, state: "" },
    { n: 7, name: "Results · Stresses", fixed: false, state: "live" },
    { n: 8, name: "Results · Restraint loads", fixed: false, state: "live" },
    { n: 9, name: "Hanger selection", fixed: false, state: "drafted" },
    { n: 10, name: "Review/signoff block", fixed: true, state: "empty" },
  ],
  comments: [
    { kind: "Check", ref: "Cases · EXP1, EXP2", who: "Agent", when: "09:31", text: "EXP1 is the algebraic OPE1 − SUS while EXP2 is the pure thermal T2. If the lift-off at 20 is meant to be captured, EXP2 would follow the same form. Which is intended?", state: "open" },
    { kind: "Open issue", ref: "Restraints · node 20", who: "Agent", when: "09:33", text: "P-12 row 2 (close the 3 mm gap at 20) is still pending. §4 says the gap is as surveyed; if that stands, reject the row so the proposal closes.", state: "open" },
    { kind: "Check", ref: "Stresses · 70 · EXP1", who: "Agent", when: "09:35", text: "The governing ratio fell from 0.72 (Run 03) to 0.58 with the spring at 80. The SIF at the welding tee comes from sample-rules 1.2; its provenance is recorded.", state: "resolved" },
    { kind: "Note", ref: "Hangers · node 80", who: "R. Tufts", when: "09:38", text: "Re-check the cold load at 80 once the vendor confirms size A-4.", state: "open" },
  ],
};
