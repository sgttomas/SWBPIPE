// Draws the sample model as a schematic figure in inline SVG: the design system's figure
// language (§6) rendered in 2D by a fixed isometric projection. Every colour is a canvas
// token; the drawing is a mock rendering, not the engine's.
import { rows, coordinates, sections, restraints, loads, nodeData } from "./model.mjs";

const DEG = Math.PI / 180;
export const esc = (s) => String(s).replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;").replace(/"/g, "&quot;");
const f1 = (n) => (Math.round(n * 10) / 10).toFixed(1);
const r1 = (n) => Math.round(n * 10) / 10;

// Result scale colour for a ratio in [0, 1]: OKLCH interpolation between the seven anchors,
// expressed with color-mix so that it follows the theme's tokens.
export function scaleColor(r) {
  const t = Math.max(0, Math.min(1, r)) * 6;
  const i = Math.min(5, Math.floor(t));
  const f = t - i;
  const pct = Math.round((1 - f) * 100);
  if (pct >= 100) return `var(--result-scale-${i + 1})`;
  return `color-mix(in oklch, var(--result-scale-${i + 1}) ${pct}%, var(--result-scale-${i + 2}))`;
}

function sectionOf(name) { return sections.find((s) => s.name === name); }
// The section at a node: the arriving element's, or for a start node the leaving element's.
function secAt(row) { return row.section || rows.find((r) => r.from === row.node)?.section || sections[0].name; }

export function figure(o) {
  const w = o.w, h = o.h;
  const A = (o.view?.A ?? 40) * DEG, E = (o.view?.E ?? 28) * DEG;
  const cosA = Math.cos(A), sinA = Math.sin(A), cosE = Math.cos(E), sinE = Math.sin(E);
  const proj = (p) => ({ x: p.x * cosA - p.z * sinA, y: -p.y * cosE + (p.x * sinA + p.z * cosA) * sinE });
  const axis = { X: proj({ x: 1, y: 0, z: 0 }), Y: proj({ x: 0, y: 1, z: 0 }), Z: proj({ x: 0, y: 0, z: 1 }) };
  const C = coordinates();
  const include = o.nodes ? new Set(o.nodes) : new Set(rows.map((r) => r.node));
  const nodes = rows.filter((r) => include.has(r.node));
  const elements = nodes.filter((r) => r.from != null && include.has(r.from)).map((r) => ({ from: r.from, to: r.node, row: r }));
  const byNode = Object.fromEntries(rows.map((r) => [r.node, r]));

  // Fit.
  const pts = nodes.map((r) => proj(C[r.node]));
  if (o.draft) { const s = C[o.draft.from]; const d = { X: [1, 0, 0], Y: [0, 1, 0], Z: [0, 0, 1] }[o.draft.axis]; pts.push(proj({ x: s.x + d[0] * o.draft.len, y: s.y + d[1] * o.draft.len, z: s.z + d[2] * o.draft.len })); }
  const minx = Math.min(...pts.map((p) => p.x)), maxx = Math.max(...pts.map((p) => p.x));
  const miny = Math.min(...pts.map((p) => p.y)), maxy = Math.max(...pts.map((p) => p.y));
  const m = Object.assign({ top: 56, right: 40, bottom: 56, left: 40 }, o.margin || {});
  // The camera. The fit is computed for the canvas the engineer set it in (o.camera.w × o.camera.h;
  // by default this canvas) and the figure is drawn into w × h at that scale with the same world
  // point at the centre, shifted by o.camera.pan: the docked inspector keeps the camera scale and
  // centre and pans only as far as needed to keep the selected node in view (V1.1 §0, decision 1).
  const cw = o.camera?.w ?? w, ch = o.camera?.h ?? h;
  const scale = Math.min((cw - m.left - m.right) / (maxx - minx || 1), (ch - m.top - m.bottom) / (maxy - miny || 1)) * (o.zoom ?? 1);
  const ox = m.left + ((cw - m.left - m.right) - (maxx - minx) * scale) / 2 - minx * scale - (cw - w) / 2 + (o.camera?.pan?.x ?? 0);
  const oy = m.top + ((ch - m.top - m.bottom) - (maxy - miny) * scale) / 2 - miny * scale - (ch - h) / 2 + (o.camera?.pan?.y ?? 0);
  const S = (p) => { const q = proj(p); return { x: ox + q.x * scale, y: oy + q.y * scale }; };
  const P = {}; for (const r of nodes) P[r.node] = S(C[r.node]);
  const unit = (v) => { const l = Math.hypot(v.x, v.y) || 1; return { x: v.x / l, y: v.y / l }; };
  const dirOf = (a, b) => unit({ x: b.x - a.x, y: b.y - a.y });
  const norm = (u) => { let n = { x: -u.y, y: u.x }; if (n.y < 0 || (n.y === 0 && n.x < 0)) n = { x: -n.x, y: -n.y }; return n; };
  const dia = (name) => Math.max(5, sectionOf(name).od * scale);
  const diaAt = (node) => dia(secAt(byNode[node]));

  const out = [];
  const defs = [];
  defs.push(`<pattern id="hatch-${o.id}" width="5" height="5" patternUnits="userSpaceOnUse" patternTransform="rotate(45)"><path d="M0 0V5" stroke="var(--canvas-glyph)" stroke-width="1"/></pattern>`);
  defs.push(`<clipPath id="clip-${o.id}"><rect width="${w}" height="${h}"/></clipPath>`);

  // Ground grid on y = 0 (isometric).
  if (o.grid !== false) {
    const g = [];
    const xs = [], zs = [];
    for (let x = -1000; x <= 9000; x += 500) xs.push(x);
    for (let z = -1500; z <= 8000; z += 500) zs.push(z);
    for (const x of xs) { const a = S({ x, y: 0, z: zs[0] }), b = S({ x, y: 0, z: zs[zs.length - 1] }); g.push(`<line x1="${f1(a.x)}" y1="${f1(a.y)}" x2="${f1(b.x)}" y2="${f1(b.y)}" stroke="var(--canvas-grid${x % 1000 === 0 ? "Major" : "Minor"})"/>`); }
    for (const z of zs) { const a = S({ x: xs[0], y: 0, z }), b = S({ x: xs[xs.length - 1], y: 0, z }); g.push(`<line x1="${f1(a.x)}" y1="${f1(a.y)}" x2="${f1(b.x)}" y2="${f1(b.y)}" stroke="var(--canvas-grid${z % 1000 === 0 ? "Major" : "Minor"})"/>`); }
    out.push(`<g clip-path="url(#clip-${o.id})" stroke-width="1">${g.join("")}</g>`);
  }

  // Element geometry: start and end points allowing for bends at tips.
  const tp = {}; // bend tangent points per node: {in, out}
  for (const r of nodes) {
    if (r.type !== "Bend") continue;
    const next = elements.find((e) => e.from === r.node && !e.row.branch);
    const prev = elements.find((e) => e.to === r.node);
    if (!next || !prev) continue;
    const R = r.bendR || sectionOf(r.section).bendR;
    const T = C[r.node], Aa = C[prev.from], Bb = C[next.to];
    const u = (a, b) => { const l = Math.hypot(b.x - a.x, b.y - a.y, b.z - a.z); return { x: (b.x - a.x) / l, y: (b.y - a.y) / l, z: (b.z - a.z) / l }; };
    const ui = u(Aa, T), uo = u(T, Bb);
    tp[r.node] = { in: { x: T.x - ui.x * R, y: T.y - ui.y * R, z: T.z - ui.z * R }, out: { x: T.x + uo.x * R, y: T.y + uo.y * R, z: T.z + uo.z * R } };
  }
  const startOf = (e) => (tp[e.from] && !e.row.branch ? S(tp[e.from].out) : P[e.from]);
  const endOf = (e) => (tp[e.to] ? S(tp[e.to].in) : P[e.to]);
  const colorOf = (e) => {
    if (o.colorBy && o.colorBy.ratios[e.to] != null) return scaleColor(o.colorBy.ratios[e.to]);
    if (e.row.type === "Rigid") return "var(--canvas-pipeShade)";
    return "var(--canvas-pipe)";
  };
  const elemKey = (e) => `${e.from}-${e.to}`;
  const selectedEl = o.selection?.element ? `${o.selection.element[0]}-${o.selection.element[1]}` : null;

  // Halos first (under everything).
  const halos = [];
  for (const e of elements) {
    if (selectedEl === elemKey(e) || (o.hover && `${o.hover[0]}-${o.hover[1]}` === elemKey(e))) {
      const a = startOf(e), b = endOf(e);
      const d = dia(e.row.section);
      const col = selectedEl === elemKey(e) ? "var(--canvas-selection)" : "var(--canvas-hover)";
      halos.push(`<line x1="${f1(a.x)}" y1="${f1(a.y)}" x2="${f1(b.x)}" y2="${f1(b.y)}" stroke="${col}" stroke-width="${f1(d + 8)}" stroke-linecap="round" opacity=".9"/>`);
      if (tp[e.to]) { const t = P[e.to], q = S(tp[e.to].out); halos.push(`<path d="M${f1(b.x)} ${f1(b.y)}Q${f1(t.x)} ${f1(t.y)} ${f1(q.x)} ${f1(q.y)}" fill="none" stroke="${col}" stroke-width="${f1(d + 8)}" stroke-linecap="round" opacity=".9"/>`); }
    }
  }
  if (o.selection?.node != null && !o.selection.element) {
    const p = P[o.selection.node]; if (p) halos.push(`<circle cx="${f1(p.x)}" cy="${f1(p.y)}" r="${f1(diaAt(o.selection.node) / 2 + 7)}" fill="none" stroke="var(--canvas-selection)" stroke-width="2"/>`);
  }
  out.push(`<g>${halos.join("")}</g>`);

  // Tubes: edge, then fill, then shade. Bends drawn with their arriving element's colour.
  const tubes = [];
  const overlays = [];
  const tubeSegs = [];
  for (const e of elements) {
    const a = startOf(e), b = endOf(e);
    const d = dia(e.row.section);
    const col = colorOf(e);
    const u = dirOf(a, b), n = norm(u);
    tubeSegs.push([a, b, d / 2]);
    if (tp[e.to]) { const t = P[e.to], q = S(tp[e.to].out); tubeSegs.push([b, t, d / 2], [t, q, d / 2]); }
    if (e.row.type === "Reducer") {
      const d1 = dia(e.row.reducer ? sectionOf(e.row.section).name : e.row.section);
      const nextRow = rows.find((r) => r.from === e.to);
      const d2 = nextRow ? dia(nextRow.section) : d1 * 0.77;
      const pts4 = [[a.x + n.x * d1 / 2, a.y + n.y * d1 / 2], [b.x + n.x * d2 / 2, b.y + n.y * d2 / 2], [b.x - n.x * d2 / 2, b.y - n.y * d2 / 2], [a.x - n.x * d1 / 2, a.y - n.y * d1 / 2]];
      tubes.push(`<polygon points="${pts4.map((p) => f1(p[0]) + "," + f1(p[1])).join(" ")}" style="fill:${col}" stroke="var(--canvas-edge)" stroke-width="1"/>`);
      continue;
    }
    tubes.push(`<line x1="${f1(a.x)}" y1="${f1(a.y)}" x2="${f1(b.x)}" y2="${f1(b.y)}" stroke="var(--canvas-edge)" stroke-width="${f1(d + 2)}" stroke-linecap="butt"/>`);
    tubes.push(`<line x1="${f1(a.x)}" y1="${f1(a.y)}" x2="${f1(b.x)}" y2="${f1(b.y)}" style="stroke:${col}" stroke-width="${f1(d)}" stroke-linecap="butt"/>`);
    const so = d * 0.3;
    tubes.push(`<line x1="${f1(a.x + n.x * so)}" y1="${f1(a.y + n.y * so)}" x2="${f1(b.x + n.x * so)}" y2="${f1(b.y + n.y * so)}" stroke="var(--canvas-pipeShade)" stroke-width="${f1(Math.max(1.5, d * 0.22))}" stroke-linecap="butt" opacity=".45"/>`);
    if (tp[e.to]) {
      const t = P[e.to], q = S(tp[e.to].out);
      tubes.push(`<path d="M${f1(b.x)} ${f1(b.y)}Q${f1(t.x)} ${f1(t.y)} ${f1(q.x)} ${f1(q.y)}" fill="none" stroke="var(--canvas-edge)" stroke-width="${f1(d + 2)}" stroke-linecap="butt"/>`);
      tubes.push(`<path d="M${f1(b.x)} ${f1(b.y)}Q${f1(t.x)} ${f1(t.y)} ${f1(q.x)} ${f1(q.y)}" fill="none" style="stroke:${col}" stroke-width="${f1(d)}" stroke-linecap="butt"/>`);
      // tangent ticks
      const n2 = norm(dirOf(t, q));
      overlays.push(`<line x1="${f1(b.x - n.x * (d / 2 + 3))}" y1="${f1(b.y - n.y * (d / 2 + 3))}" x2="${f1(b.x + n.x * (d / 2 + 3))}" y2="${f1(b.y + n.y * (d / 2 + 3))}" stroke="var(--canvas-edge)" stroke-width="1"/>`);
      overlays.push(`<line x1="${f1(q.x - n2.x * (d / 2 + 3))}" y1="${f1(q.y - n2.y * (d / 2 + 3))}" x2="${f1(q.x + n2.x * (d / 2 + 3))}" y2="${f1(q.y + n2.y * (d / 2 + 3))}" stroke="var(--canvas-edge)" stroke-width="1"/>`);
    }
    if (e.row.type === "Valve") {
      const mx = (a.x + b.x) / 2, my = (a.y + b.y) / 2, L = Math.max(14, Math.hypot(b.x - a.x, b.y - a.y) * 0.6), hw = d * 0.9;
      const p1 = { x: mx - u.x * L / 2, y: my - u.y * L / 2 }, p2 = { x: mx + u.x * L / 2, y: my + u.y * L / 2 };
      const bow = [[p1.x + n.x * hw, p1.y + n.y * hw], [p2.x - n.x * hw, p2.y - n.y * hw], [p2.x + n.x * hw, p2.y + n.y * hw], [p1.x - n.x * hw, p1.y - n.y * hw]];
      overlays.push(`<polygon points="${bow.map((p) => f1(p[0]) + "," + f1(p[1])).join(" ")}" fill="var(--canvas-glyphFill)" stroke="var(--canvas-glyph)" stroke-width="1.5" stroke-linejoin="round"/>`);
      overlays.push(`<line x1="${f1(mx)}" y1="${f1(my)}" x2="${f1(mx)}" y2="${f1(my - hw - 12)}" stroke="var(--canvas-glyph)" stroke-width="1.5"/><line x1="${f1(mx - 5)}" y1="${f1(my - hw - 12)}" x2="${f1(mx + 5)}" y2="${f1(my - hw - 12)}" stroke="var(--canvas-glyph)" stroke-width="1.5"/>`);
    }
  }
  out.push(`<g>${tubes.join("")}</g>`);

  // Tee saddle at branch points: a short arc where the branch meets the run.
  for (const e of elements.filter((x) => x.row.branch)) {
    const p = P[e.from]; const d = diaAt(e.from); const db = dia(e.row.section);
    const u = dirOf(p, endOf(e));
    overlays.push(`<path d="M${f1(p.x + u.x * d / 2 - u.y * db / 2)} ${f1(p.y + u.y * d / 2 + u.x * db / 2)}q${f1(u.x * 4)} ${f1(u.y * 4)} ${f1(u.y * db)} ${f1(-u.x * db)}" fill="none" stroke="var(--canvas-edge)" stroke-width="1"/>`);
  }
  out.push(`<g>${overlays.join("")}</g>`);

  // Occupied boxes for label placement.
  const occupied = [];
  const occupy = (x, y, ww, hh) => occupied.push({ x, y, w: ww, h: hh });
  const arrow = (x1, y1, x2, y2, col, sw = 2, head = 7) => {
    const u = unit({ x: x2 - x1, y: y2 - y1 }); const n = { x: -u.y, y: u.x };
    const bx = x2 - u.x * head, by = y2 - u.y * head;
    return `<line x1="${f1(x1)}" y1="${f1(y1)}" x2="${f1(bx)}" y2="${f1(by)}" style="stroke:${col}" stroke-width="${sw}" stroke-linecap="round"/><polygon points="${f1(x2)},${f1(y2)} ${f1(bx + n.x * head * 0.5)},${f1(by + n.y * head * 0.5)} ${f1(bx - n.x * head * 0.5)},${f1(by - n.y * head * 0.5)}" style="fill:${col}"/>`;
  };
  const plate = (x, y, text, opts = {}) => {
    const ww = 8 + text.length * (opts.size === 11 ? 6.1 : 7.2);
    // A plate whose anchor (the node it names) is outside the visible canvas is not drawn, so that
    // a panned camera (docked inspector) does not clamp the plates of hidden nodes to the edges.
    if (opts.anchor && (opts.anchor.x < -4 || opts.anchor.x > w + 4 || opts.anchor.y < -4 || opts.anchor.y > h + 4)) return "";
    x = Math.max(2, Math.min(w - ww - 2, x)); y = Math.max(2, Math.min(h - 20, y));
    occupy(x, y, ww, 18);
    return `<rect x="${f1(x)}" y="${f1(y)}" width="${f1(ww)}" height="18" rx="2" fill="var(--canvas-labelBg)"${opts.edge ? ` stroke="${opts.edge}" stroke-width="1.5"` : ""}/><text x="${f1(x + 4)}" y="${f1(y + 13)}" font-size="${opts.size || 12}" fill="var(--canvas-label)">${esc(text)}</text>`;
  };

  // Restraint glyphs.
  const glyphs = [];
  const rest = o.restraints === false ? [] : restraints.filter((r) => include.has(r.node) && !(o.omitRestraints || []).includes(r.node));
  for (const r of rest) {
    const p = P[r.node]; const row = byNode[r.node]; const d = diaAt(r.node); const rr = d / 2;
    const el = elements.find((e) => e.to === r.node) || elements.find((e) => e.from === r.node);
    const u = el ? dirOf(startOf(el), endOf(el)) : { x: 1, y: 0 };
    const n = { x: -u.y, y: u.x };
    const G = "var(--canvas-glyph)", GF = "var(--canvas-glyphFill)";
    if (r.type === "Anchor") {
      const L = rr + 9, T = 4;
      const c = [[p.x + n.x * L - u.x * T, p.y + n.y * L - u.y * T], [p.x + n.x * L + u.x * T, p.y + n.y * L + u.y * T], [p.x - n.x * L + u.x * T, p.y - n.y * L + u.y * T], [p.x - n.x * L - u.x * T, p.y - n.y * L - u.y * T]];
      glyphs.push(`<polygon points="${c.map((q) => f1(q[0]) + "," + f1(q[1])).join(" ")}" fill="url(#hatch-${o.id})" stroke="${G}" stroke-width="1.5"/>`);
      occupy(p.x - L, p.y - L, 2 * L, 2 * L);
    } else if (r.type === "+Y") {
      const gap = r.gap ? 4 : 0; const top = p.y + rr + gap;
      glyphs.push(`<polygon points="${f1(p.x)},${f1(top)} ${f1(p.x - 8)},${f1(top + 13)} ${f1(p.x + 8)},${f1(top + 13)}" fill="${GF}" stroke="${G}" stroke-width="1.5" stroke-linejoin="round"/>`);
      glyphs.push(`<line x1="${f1(p.x - 13)}" y1="${f1(top + 15)}" x2="${f1(p.x + 13)}" y2="${f1(top + 15)}" stroke="${G}" stroke-width="1.5"/>`);
      if (r.mu) glyphs.push(`<path d="M${f1(p.x - 9)} ${f1(top + 15)}l-3 4M${f1(p.x - 3)} ${f1(top + 15)}l-3 4M${f1(p.x + 3)} ${f1(top + 15)}l-3 4M${f1(p.x + 9)} ${f1(top + 15)}l-3 4" stroke="${G}" stroke-width="1"/>`);
      glyphs.push(arrow(p.x, top + 32, p.x, top + 19, G, 1.5, 5));
      if (r.gap) glyphs.push(`<line x1="${f1(p.x - 3)}" y1="${f1(p.y + rr)}" x2="${f1(p.x + 3)}" y2="${f1(p.y + rr)}" stroke="${G}" stroke-width="1"/>`);
      occupy(p.x - 14, p.y + rr, 28, 36);
      { const txt = `+Y · gap ${r.gap} mm · μ ${r.mu.toFixed(2)}`; const ww = 8 + txt.length * 6.1; glyphs.push(plate(o.plateSide === "left" ? p.x - 12 - ww : p.x + 12, top + 12, txt, { size: 11, anchor: p })); }
    } else if (r.type === "Rigid (Y)") {
      const top = p.y - rr - 30;
      glyphs.push(`<line x1="${f1(p.x)}" y1="${f1(p.y - rr)}" x2="${f1(p.x)}" y2="${f1(top)}" stroke="${G}" stroke-width="2"/>`);
      glyphs.push(`<line x1="${f1(p.x - 8)}" y1="${f1(top)}" x2="${f1(p.x + 8)}" y2="${f1(top)}" stroke="${G}" stroke-width="2"/><path d="M${f1(p.x - 6)} ${f1(top)}l-3 -4M${f1(p.x)} ${f1(top)}l-3 -4M${f1(p.x + 6)} ${f1(top)}l-3 -4" stroke="${G}" stroke-width="1"/>`);
      glyphs.push(arrow(p.x + 7, p.y - rr - 4, p.x + 7, p.y - rr - 16, G, 1.5, 5));
      glyphs.push(arrow(p.x + 7, p.y - rr - 16, p.x + 7, p.y - rr - 4, G, 1.5, 5));
      occupy(p.x - 9, top - 4, 18, rr + 34);
      glyphs.push(plate(p.x + 14, top - 2, `Rigid Y · ${r.tag}`, { size: 11, anchor: p }));
    } else if (r.type === "Variable spring") {
      const top = p.y - rr - 40;
      glyphs.push(`<line x1="${f1(p.x)}" y1="${f1(p.y - rr)}" x2="${f1(p.x)}" y2="${f1(top + 22)}" stroke="${G}" stroke-width="1.5"/>`);
      glyphs.push(`<rect x="${f1(p.x - 7)}" y="${f1(top)}" width="14" height="22" rx="1.5" fill="${GF}" stroke="${G}" stroke-width="1.5"/>`);
      glyphs.push(`<path d="M${f1(p.x)} ${f1(top + 3)}l4 3-8 3 8 3-8 3 8 3-4 3" fill="none" stroke="${G}" stroke-width="1"/>`);
      glyphs.push(`<line x1="${f1(p.x)}" y1="${f1(top)}" x2="${f1(p.x)}" y2="${f1(top - 8)}" stroke="${G}" stroke-width="1.5"/><line x1="${f1(p.x - 8)}" y1="${f1(top - 8)}" x2="${f1(p.x + 8)}" y2="${f1(top - 8)}" stroke="${G}" stroke-width="2"/>`);
      glyphs.push(arrow(p.x + 11, p.y - rr - 2, p.x + 11, p.y - rr - 14, G, 1.5, 5));
      occupy(p.x - 9, top - 10, 24, rr + 52);
      glyphs.push(plate(p.x + 14, top + 2, `VS · ${r.tag}`, { size: 11, anchor: p }));
    } else if (r.type === "Guide") {
      const gap = r.gap ? 4 : 0; const L = rr + 6;
      const side = (s) => {
        const cx = p.x + n.x * (rr + gap + 3) * s, cy = p.y + n.y * (rr + gap + 3) * s;
        const a = { x: cx - u.x * L, y: cy - u.y * L }, b = { x: cx + u.x * L, y: cy + u.y * L };
        const lip = { x: -n.x * 4 * s, y: -n.y * 4 * s };
        return `<path d="M${f1(a.x - lip.x)} ${f1(a.y - lip.y)}L${f1(a.x)} ${f1(a.y)}L${f1(b.x)} ${f1(b.y)}L${f1(b.x - lip.x)} ${f1(b.y - lip.y)}" fill="none" stroke="${G}" stroke-width="1.5"/>` + arrow(cx + n.x * 14 * s, cy + n.y * 14 * s, cx + n.x * 3 * s, cy + n.y * 3 * s, G, 1.5, 5);
      };
      glyphs.push(side(1) + side(-1));
      occupy(p.x - rr - 22, p.y - L - 4, 2 * rr + 44, 2 * L + 8);
      glyphs.push(plate(p.x + rr + 18, p.y + 4, `Guide X Z · gap ${r.gap} mm · μ ${r.mu.toFixed(2)}`, { size: 11, anchor: p }));
    }
  }
  // Node data: flange discs.
  if (o.nodeData !== false) for (const nd of nodeData.filter((x) => include.has(x.node) && x.kind === "Flange")) {
    const p = P[nd.node]; const d = diaAt(nd.node); const el = elements.find((e) => e.to === nd.node);
    const u = dirOf(startOf(el), endOf(el)); const n = { x: -u.y, y: u.x }; const L = d / 2 + 3;
    for (const s of [-2.5, 2.5]) glyphs.push(`<line x1="${f1(p.x + u.x * s - n.x * L)}" y1="${f1(p.y + u.y * s - n.y * L)}" x2="${f1(p.x + u.x * s + n.x * L)}" y2="${f1(p.y + u.y * s + n.y * L)}" stroke="var(--canvas-glyph)" stroke-width="2"/>`);
  }
  out.push(`<g>${glyphs.join("")}</g>`);

  // Proposal ghost: a spring beside the current glyph at a node.
  if (o.ghost) {
    const p = P[o.ghost.node]; const d = diaAt(o.ghost.node); const rr = d / 2; const GH = "var(--canvas-proposalGhost)";
    const x = p.x + 22, top = p.y - rr - 40;
    const g = [];
    g.push(`<line x1="${f1(x)}" y1="${f1(p.y - rr)}" x2="${f1(x)}" y2="${f1(top + 22)}" stroke="${GH}" stroke-width="1.5" stroke-dasharray="4 3"/>`);
    g.push(`<rect x="${f1(x - 7)}" y="${f1(top)}" width="14" height="22" rx="1.5" fill="none" stroke="${GH}" stroke-width="1.5" stroke-dasharray="4 3"/>`);
    g.push(`<path d="M${f1(x)} ${f1(top + 3)}l4 3-8 3 8 3-8 3 8 3-4 3" fill="none" stroke="${GH}" stroke-width="1"/>`);
    g.push(`<line x1="${f1(x)}" y1="${f1(top)}" x2="${f1(x)}" y2="${f1(top - 8)}" stroke="${GH}" stroke-width="1.5" stroke-dasharray="4 3"/><line x1="${f1(x - 8)}" y1="${f1(top - 8)}" x2="${f1(x + 8)}" y2="${f1(top - 8)}" stroke="${GH}" stroke-width="2" stroke-dasharray="4 3"/>`);
    // removed current glyph: dashed outline stays as the rigid rod (drawn solid above); ghost label
    out.push(`<g opacity=".7">${g.join("")}</g>`);
    occupy(x - 9, top - 10, 24, rr + 52);
    out.push(plate(x + 12, top + 30, `Variable spring · ${o.ghost.id}`, { size: 11, edge: GH, anchor: p }));
  }

  // Load vectors.
  const vec = [];
  if (o.loads !== false) for (const l of loads.filter((x) => include.has(x.node) && x.case !== "T2")) {
    const p = P[l.node]; const d = diaAt(l.node); const rr = d / 2;
    const col = o.loadsNeutral ? "var(--canvas-vector)" : (l.kind === "Force" ? "var(--cat-1)" : "var(--cat-3)");
    if (l.kind === "Force") {
      vec.push(arrow(p.x, p.y - rr - 48, p.x, p.y - rr - 2, col, 2, 8));
      occupy(p.x - 5, p.y - rr - 50, 10, 50);
      vec.push(plate(p.x + 8, p.y - rr - 52, `${l.value} N · ${l.case}`, { anchor: p }));
    } else {
      const base = p.y + rr + 40;
      vec.push(`<line x1="${f1(p.x - 7)}" y1="${f1(base)}" x2="${f1(p.x + 7)}" y2="${f1(base)}" style="stroke:${col}" stroke-width="2"/>`);
      vec.push(arrow(p.x, base, p.x, p.y + rr + 4, col, 2, 8));
      occupy(p.x - 8, p.y + rr, 16, 42);
      vec.push(plate(p.x + 10, base - 10, `+${l.value.toFixed(1)} mm · ${l.case}`, { anchor: p }));
    }
  }
  out.push(`<g>${vec.join("")}</g>`);

  // Draft ghost and compass.
  const overlaysHTML = {};
  if (o.draft) {
    const s = C[o.draft.from]; const dv = { X: [1, 0, 0], Y: [0, 1, 0], Z: [0, 0, 1] }[o.draft.axis];
    const e = S({ x: s.x + dv[0] * o.draft.len, y: s.y + dv[1] * o.draft.len, z: s.z + dv[2] * o.draft.len });
    const p = P[o.draft.from]; const d = diaAt(o.draft.from);
    // The draft ghost (decision 12; V1.1 §5.6): a faint tube outline at the draft's diameter
    // (canvas.draft at 30%, 1 px), a thin dashed centreline (1.5 px, 6/4 dash) and the draft
    // node's plate with a canvas.draft edge at the tip.
    { const u = dirOf(p, e), n = norm(u), r = d / 2;
      const L = (a, b) => `M${f1(a.x)} ${f1(a.y)}L${f1(b.x)} ${f1(b.y)}`;
      const a1 = { x: p.x + n.x * r, y: p.y + n.y * r }, a2 = { x: e.x + n.x * r, y: e.y + n.y * r }, b1 = { x: p.x - n.x * r, y: p.y - n.y * r }, b2 = { x: e.x - n.x * r, y: e.y - n.y * r };
      out.push(`<g fill="none" stroke="var(--canvas-draft)"><path d="${L(a1, a2)}${L(b1, b2)}${L(a2, b2)}" stroke-width="1" opacity=".3"/><line x1="${f1(p.x)}" y1="${f1(p.y)}" x2="${f1(e.x)}" y2="${f1(e.y)}" stroke-width="1.5" stroke-dasharray="6 4"/></g>`); }
    tubeSegs.push([p, e, d / 2]);
    const comp = [];
    for (const ax of ["X", "Y", "Z"]) {
      const a = unit(axis[ax]); const active = ax === o.draft.axis; const L = active ? 56 : 40;
      const col = `var(--canvas-axis${ax})`;
      comp.push(arrow(p.x, p.y, p.x + a.x * L, p.y + a.y * L, col, active ? 2 : 1, active ? 7 : 5));
      comp.push(`<text x="${f1(p.x + a.x * (L + 10) - 3)}" y="${f1(p.y + a.y * (L + 10) + 4)}" font-size="11" style="fill:${col}">${ax}</text>`);
      if (active) overlaysHTML.lengthField = { x: p.x + a.x * L + 26, y: p.y + a.y * L - 30 };
    }
    out.push(`<g>${comp.join("")}</g>`);
    out.push(`<circle cx="${f1(p.x)}" cy="${f1(p.y)}" r="3" fill="var(--canvas-draft)"/>`);
    overlaysHTML.draftEnd = e;
    out.push(plate(e.x + 8, e.y - 22, String(o.draft.next), { edge: "var(--canvas-draft)", anchor: e }));
  }

  // The triad and the scale reference occupy the canvas's bottom corners before the labels are placed.
  const scaleRefL = 1000 * scale * Math.hypot(axis.X.x, axis.X.y);
  const scaleRefX = w - 24 - Math.max(scaleRefL, 60);
  occupy(0, h - 92, 96, 92);
  if (o.scaleRef !== false) occupy(scaleRefX - 6, h - (o.loads !== false ? 46 : 36), Math.max(scaleRefL, 60) + 100, 40);

  // Node labels with a placement search.
  const labels = [];
  const labelNodes = o.labels === "off" ? [] : nodes.map((r) => r.node);
  const dirsAt = (nd) => { const ds = []; for (const e of elements) { if (e.from === nd) ds.push(dirOf(P[nd], endOf(e))); if (e.to === nd) ds.push(dirOf(P[nd], startOf(e))); } return ds; };
  const overlap = (a, b) => !(a.x + a.w + 2 < b.x || b.x + b.w + 2 < a.x || a.y + a.h + 2 < b.y || b.y + b.h + 2 < a.y);
  const segDist = (px, py, a, b) => { const vx = b.x - a.x, vy = b.y - a.y; const L2 = vx * vx + vy * vy || 1; let t = ((px - a.x) * vx + (py - a.y) * vy) / L2; t = Math.max(0, Math.min(1, t)); return Math.hypot(px - (a.x + vx * t), py - (a.y + vy * t)); };
  const onTube = (box) => { const cx = box.x + box.w / 2, cy = box.y + box.h / 2; for (const [a, b, rr] of tubeSegs) { if (segDist(cx, cy, a, b) < rr + 11) return true; const c1 = segDist(box.x, cy, a, b), c2 = segDist(box.x + box.w, cy, a, b); if (Math.min(c1, c2) < rr + 4) return true; } return false; };
  for (const nd of labelNodes) {
    const p = P[nd]; const text = String(nd); const ww = 8 + text.length * 7.2; const rr = diaAt(nd) / 2;
    if (p.x < -8 || p.x > w + 8 || p.y < -8 || p.y > h + 8) continue; // outside the panned camera's view
    const ds = dirsAt(nd); const away = unit({ x: -ds.reduce((s, v) => s + v.x, 0), y: -ds.reduce((s, v) => s + v.y, 0) });
    const cands = [[rr + 6, -rr - 20], [rr + 6, -9], [rr + 6, rr + 4], [-ww - rr - 6, -rr - 20], [-ww - rr - 6, -9], [-ww - rr - 6, rr + 4], [-ww / 2, -rr - 24], [-ww / 2, rr + 8], [rr + 14, -rr - 32], [-ww - rr - 14, rr + 14], [rr + 22, -9], [-ww - rr - 22, -9], [-ww / 2, -rr - 40], [-ww / 2, rr + 24]];
    cands.sort((a, b) => { const ca = unit({ x: a[0] + ww / 2, y: a[1] + 9 }), cb = unit({ x: b[0] + ww / 2, y: b[1] + 9 }); return (cb.x * away.x + cb.y * away.y) - (ca.x * away.x + ca.y * away.y); });
    let placed = null;
    for (const c of cands) { const box = { x: p.x + c[0], y: p.y + c[1], w: ww, h: 18 }; if (box.x < 2 || box.y < 2 || box.x + ww > w - 2 || box.y + 18 > h - 2) continue; if (!occupied.some((b) => overlap(box, b)) && !onTube(box)) { placed = box; break; } }
    if (!placed) for (const c of cands) { const box = { x: p.x + c[0], y: p.y + c[1], w: ww, h: 18 }; if (box.x < 2 || box.y < 2 || box.x + ww > w - 2 || box.y + 18 > h - 2) continue; if (!occupied.some((b) => overlap(box, b))) { placed = box; break; } }
    // Budget (decision 13; V1.1 §6.5): a plate that cannot be placed without overlap yields, except
    // the current row's node, which is always labelled.
    const isCurrent = o.selection?.node === nd || (o.selection?.element && o.selection.element[1] === nd);
    if (!placed) { if (!isCurrent) continue; const c = cands[0]; placed = { x: p.x + c[0], y: p.y + c[1], w: ww, h: 18 }; }
    occupy(placed.x, placed.y, placed.w, placed.h);
    const edge = (o.selection?.node === nd || (o.selection?.element && o.selection.element[1] === nd)) ? "var(--canvas-selection)" : (o.ghost?.node === nd ? "var(--canvas-proposalGhost)" : (o.issueNode === nd ? "var(--issue-warning)" : null));
    labels.push(plate(placed.x, placed.y, text, { edge }));
  }
  out.push(`<g class="labels">${labels.join("")}</g>`);

  // Triad (bottom-left) and the scale reference (bottom-right).
  const tri = [];
  const tx = 34, ty = h - 30;
  for (const ax of ["X", "Y", "Z"]) { const a = unit(axis[ax]); tri.push(arrow(tx, ty, tx + a.x * 34, ty + a.y * 34, `var(--canvas-axis${ax})`, 2, 6)); tri.push(`<text x="${f1(tx + a.x * 46 - 4)}" y="${f1(ty + a.y * 46 + 4)}" font-size="11" style="fill:var(--canvas-axis${ax})">${ax === "Y" ? "Y up" : ax}</text>`); }
  out.push(`<g>${tri.join("")}</g>`);
  if (o.scaleRef !== false) {
    const L = scaleRefL;
    const sx = scaleRefX, sy = h - 22;
    out.push(`<g><line x1="${f1(sx)}" y1="${f1(sy)}" x2="${f1(sx + L)}" y2="${f1(sy)}" stroke="var(--canvas-edge)" stroke-width="1"/><line x1="${f1(sx)}" y1="${f1(sy - 4)}" x2="${f1(sx)}" y2="${f1(sy + 4)}" stroke="var(--canvas-edge)" stroke-width="1"/><line x1="${f1(sx + L)}" y1="${f1(sy - 4)}" x2="${f1(sx + L)}" y2="${f1(sy + 4)}" stroke="var(--canvas-edge)" stroke-width="1"/><text x="${f1(sx)}" y="${f1(sy - 8)}" font-size="11" fill="var(--canvas-label)">1 m</text>${o.loads !== false ? `${arrow(sx, sy - 30, sx + 40, sy - 30, "var(--canvas-vector)", 2, 6)}<text x="${f1(sx + 46)}" y="${f1(sy - 26)}" font-size="11" fill="var(--canvas-label)">1000 N</text>` : ""}</g>`);
  }

  const svg = `<svg class="fig" viewBox="0 0 ${w} ${h}" width="${w}" height="${h}" role="img" aria-label="Model figure, mock rendering"><defs>${defs.join("")}</defs><rect width="${w}" height="${h}" fill="var(--canvas-bg)"/>${out.join("")}</svg>`;
  return { svg, P, scale, overlays: overlaysHTML, w, h };
}
