# B-CANVAS proposal P2: a slice for the edge line (C1E)

From B-CANVAS (WORKING_ITEMS, Type 1, canvas lane manager; Claude Fable 5.1) to ROOT, 2026-09-19 (UTC). Status: **proposal for ROOT's decision**. Nothing described here has been built. Placeholders as in the lane brief.

## 0. What ROOT is asked to decide

| Ask | Decision | Manager's recommendation |
|---|---|---|
| ASK-1 | Add a slice, C1E, that draws the design's edge line. The brief's C1 to C6 contain no such slice. | Add it. |
| ASK-2 | Its place in the order. | After C1b and before C2. Reasons in §4. |
| ASK-3 | The order in which mechanisms are probed (§3), and that a screen-space pass is not tried without ROOT's separate word, because it changes the persistent renderer's frame. | A, then B, then C. D only on ROOT's word. |
| ASK-4 | Whether the rings at changes of section and at free ends are part of C1E or follow it (§2). They need two flags per pipe end from the typed model index, which the lane may read and whose file (`src/features/workspace/modelIndex.ts`) is in the lane's write scope. | In C1E if the chosen mechanism carries them at no measurable cost; otherwise returned as a follow-on with the numbers. |
| ASK-5 | Whether the lane branch may reach `main` between C1 and C1E. After C1 the light-theme figure is a 1.82:1 grey fill with no line (§1). | ROOT's call; the lane flags it and does not decide it. |

## 1. The gap

Design system §6.2: "Every tube has an edge line in `canvas.edge`, one pixel at any zoom, drawn at the silhouette and at every change of section", and in both themes "the edge is what carries the geometry". Its contrast findings (§2.9) say why: `canvas.pipe` is 1.97:1 on `canvas.bg` in light by design, and the edge is 7.88:1 on the ground and 4.00:1 on the tube; in dark the edge is 10.50:1 and 2.63:1. Both reference frames (`s2_model_light`, `s2_model_dark`) draw it.

Three accepted records assume it exists. D-72 item 2 keeps a device-pixel-ratio-1 observation because "the design's one-pixel edge line and fixed-size glyphs are specified for both". D-72 item 4 measures "orbit with result colour, edge lines and labels at Budget". The brief's C6 probes `canvas.edgeAlt`, the alternate colour of a line that no slice draws.

C1's first part shows what the gap costs. C1A-PALETTE measured the running product (`../returns/C1A-PALETTE_RETURN.md`, §10): in light the tubes went from a lit teal at about 6.5 to 7.5:1 on the ground to `canvas.pipe` at 1.82:1 on the held ground, and will be 1.97:1 once the ground is `canvas.bg`. That is the design working as specified with one of its two parts missing. Dark is better than before (4.44:1 against about 2:1).

## 2. Scope

In: a 1 CSS px line in `canvas.edge` at the silhouette of every tube-like element the product draws today (pipes in both geometry modes; bend, branch, rigid and expansion-joint placeholders), repainted live with the theme like every C1 role, dimmed with its element under C3, at device pixel ratios 1 and 2. Subject to ASK-4: a ring at each change of section and at each free end, so that "diameter changes read as steps in the edge".

Out, each with its reason:
- The per-element choice between `canvas.edge` and `canvas.edgeAlt`: that is C6. C1E reports what the chosen mechanism would make that choice cost.
- The edge on result colour, glyph strokes, insulation sleeves: result colouring and fixed-size glyphs are absent from this tranche.
- Swept elbows, tees with a saddle line, conical reducers: no slice of the brief builds fitting geometry. C1E edges the placeholder shapes as they are.
- The 1.5 px centreline for an element with no section: today such a pipe is drawn as a tube at the schematic radius (`actualRadii.get(key) ?? 0.052` in the instanced pipe builder), so this is a change of geometry, not of line. It is named here and left for C5, which owns the resting geometry.
- Node spheres: whether nodes keep a sphere is C5's. C1E gives them no line of their own.

Write scope: `{DESKTOP}/src/features/viewport/**` and its tests, and `src/features/workspace/modelIndex.ts` only if ASK-4 needs it. No stylesheet or token change: `canvas.edge` exists.

## 3. Mechanisms to probe

Facts at the lane base: the renderer is `new THREE.WebGLRenderer({ antialias: true })`, pixel ratio capped at 2, with no stencil buffer and no post-processing; the gizmo is drawn scissored by the same renderer. Pipes are one unit `CylinderGeometry(1, 1, 1, 10, 1, false)` (ten sides, capped) instanced per spatial chunk, scaled by radius and length in the instance matrix. After C1A every figure mesh uses one unlit material whose fragment shader already computes how far a fragment faces the viewer. Picking is analytic over the typed model index (`pointPickPrimitives`, `pickPointPrimitive`); it never reads the drawn meshes, so a display-only line cannot enter it.

| | Mechanism | Added per frame | New GPU resources | Foundation | Known weakness |
|---|---|---|---|---|---|
| A | A band inside the figure material: a fragment within 1 px of its tube's silhouette takes `canvas.edge`. The width in pixels comes from the instance's radius and the projection; the caps carry the ring. | A few shader operations. No draw call, no vertex. | None. | None beyond the C1A material. | The drawn outline is the ten-sided polygon's. The band narrows at some orbit angles once a tube is wider than about 40 px on screen. The line lies inside the silhouette, not outside it. |
| B | An inverted hull: the same instanced geometry drawn again, back faces only, pushed out 1 px in screen space, in `canvas.edge`. | One more instanced draw per chunk per tube kind; tube vertices twice. | One mesh and one material per chunk, sharing geometry and instance matrices. | The ownership ledger must count a shared geometry once; `disposeObjectChildren` is involved. Instancing, chunking and invalidation are unchanged. | Silhouette only: a crease facing the viewer gets no line. Small ticks appear where one tube's end stands proud of the next. |
| C | Analytic silhouette lines: per pipe, the two tangent lines of its cylinder computed in a vertex shader and drawn as instanced 1 px quads (the technique of three's `LineSegments2`, which the product does not use today); ring loops at flagged ends. | One instanced draw per chunk; eight vertices per pipe. | One instanced geometry per chunk. | New instanced objects in the ledger. | Straight tubes only; the placeholders would still need A or B. The most shader work of the three. |
| D | A screen-space pass over a depth, normal or identity buffer. | A second scene pass or multiple render targets, and a full-screen pass over up to 5.2 million device pixels at 1440 × 900 and ratio 2. | Render targets. | Changes the persistent renderer's frame: render to a target, then composite. Returned to ROOT, not tried inside a slice. | Cost, and the change to the foundation. |

The probe tries A first because it adds no draw, then B, then C if A's faceting shows at the reference views and B's cost is too high. A and B can be combined: A for tubes, B for placeholders.

C2's halo lies outside the silhouette, 2 px in `canvas.selection` and 1 px in `canvas.hover`, and is drawn for selected or hovered elements only. It belongs to mechanism B's family whatever C1E chooses. If C1E keeps B, C2 reuses it; if C1E keeps A, C2 builds B for the few elements that carry a halo.

## 4. Why before C2

1. It is the largest per-frame cost in the lane that nobody has measured, and D-72 measures with it on. A finding against it should come early.
2. The halo sits against the same silhouette. Settling how the outline is found once avoids building it twice, and proposal P1's halo rule is easier to freeze against a settled outline.
3. C6 cannot start without it.
4. Until it lands, every light-theme screenshot the lane returns shows a figure with no line, and review of C2 to C5 would be looking at a drawing the design never intended.

If ROOT prefers C2 first so that P1's cue exists sooner, nothing here prevents it. C2 would then choose mechanism B for itself and C1E would follow.

## 5. How it is measured

The brief's rule applies: per-frame work is built behind a probe and kept only if D-72's limits hold. The lane's probe guides the lane; the qualification runs are ROOT's.

- Picking first: `viewportSelection.test.ts` passes before anything else and is not edited.
- A guidance script under this lane's `tools/` (not under `e2e/`), run through `with_e2e_lock.sh` against the production build, opens the instrument's two fixtures by the product's own open control, fits the camera, and orbits with real pointer drags for ten seconds after two of warm-up, in both geometry modes, with the edge off and with each mechanism. It records frame-interval p50, p95 and maximum, `renderer.info` draw calls and triangles, the owned-resource ledger before and after two theme switches, two geometry-mode switches and a layer replacement, and whether any frame is requested after the viewport settles. It edits nothing under `e2e/ui-foundation/**` and reads no limit from there.
- The measurement is made at today's canvas size and says so, because the shell lane's layout (603 × 828 and 1000 × 828) does not exist on this branch yet. It is repeated when it does.
- Kept only if orbit p95 stays within 16.7 ms centreline and 33.3 ms real outside diameter at 10,000 pipes with the edge on, the ledger is unchanged across the switches, and the viewport still settles. The numbers with the edge off are reported beside them. If no mechanism holds the limits, the lane returns the numbers and the fallback question goes to ROOT; the lane does not thin the line, drop it at a pipe count, or touch a limit.

## 6. Checks and return

As the brief's "A slice" paragraph: the picking test first and last; `npm run test:desktop`; `npm run build:desktop`; both Playwright lanes through the lock with `PLAYWRIGHT_WORKERS=1`; the claims lint; screenshots under `../screenshots/C1E/` in both themes, both geometry modes, device pixel ratios 1 and 2, at the fitted camera and at a close view where a tube is wider than 40 px. Unit tests: the line takes `canvas.edge` in both themes and repaints in place with no resource created or disposed; a hidden element has no line; a dimmed element's line dims; no display-only object is registered as a pick input. Semantic change to name: the figure gains a line it never had; element outlines darken in light and lighten in dark.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
