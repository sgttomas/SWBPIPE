# The casing (P1 ASK-4): options, costs, and what the pair rule reads, for the second profile's freeze package

Prepared by B-CANVAS on 2026-09-19 at ROOT's request. The decision is the owner's; nothing here is built. Numbers are from `../probes/C2/MANAGER_SERIES.md` (the probe's simplified pair rule, halo alone, centreline mode, fitted camera, device pixel ratio 1) and from the token file. Placeholders as in the lane brief.

**The fact.** The selection halo (`canvas.selection`) reads 4.36:1 light and 7.39:1 dark on the ground, 2.21:1 and 1.85:1 on a tube, 1.81:1 and 1.42:1 on the edge line. In the dense fixtures a halo lies mostly over other elements. With no casing the halo alone passes the pair rule in 42 and 43 of 200 samples at 1,000 pipes (light, dark) and 1 of 200 at 10,000 pipes in both themes; it was seen in all 800.

| Option | What it is | What it costs | What the pair rule reads |
|---|---|---|---|
| **A. A 1 CSS px casing in `canvas.bg` outside the halo** | The shell drawn once more, 3 px wide in `canvas.bg`, behind the 2 px halo (same mask, same meshes' matrices); hover gets none or 1 px by the owner's word | One more instanced draw per shape for selection (2 → 3); about +60 triangles per selected pipe; by C2's measured cost of the first shell, of the order of +0.2 to +0.3 ms a frame with all 10,000 selected, to be measured; no new resource kind; a change to the adopted presentation (design system §6.6 names no casing) | Expected to hold wherever the halo is drawn, since each halo pixel has a ground-coloured pixel within 2 CSS px by construction; to be shown by the same probe (the lane can build it as a probe-only copy first, by the procedure in `PROBE_ONLY_BUILD_PROCEDURE.md`) |
| **B. No casing; the halo as designed** | Nothing | Nothing | The shortfall above. The second profile's local check would fail for most samples in these fixtures; the lane does not recommend redefining the check to fit the drawing |
| **C. No casing; the second profile measures on a cue that is not the halo** | The first profile's diamond stays as the instrument's cue (it passes 199 to 200 of 200 today because its rim is `canvas.bg`) | Keeps an instrument-only mark in the product's picture, stronger than the halo and burying the figure when everything is selected | Passes, but measures the diamond's presentation and not the design's selection; WCAG 2.2 SC 1.4.11 for the halo over a tube stays unanswered |
| **D. A casing only where the halo is not on ground** | A shader test of what lies under the halo | Needs a read of the scene's colour or depth: a render target, which is a renderer-frame change | Not recommended: the cost of form (c) and more, for the look of option A on open ground, where A is invisible anyway |

**The lane's view, for ROOT to weigh:** A is the honest answer to both the instrument and SC 1.4.11, costs one draw, and is invisible on open ground; its look (width, whether hover has one) is a closing-pass matter. If the owner chooses A, the lane builds it behind the probe first and brings the measured frame cost and the pair counts before it enters a slice.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
