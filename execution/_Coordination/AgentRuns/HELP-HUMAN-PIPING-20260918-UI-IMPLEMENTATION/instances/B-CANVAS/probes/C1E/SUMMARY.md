# Probe summary for slice C1E (the edge line)

Written by C1E-EDGE (TASK, Type 2; Claude Fable 5.1, `claude-fable-5-1`) on 2026-09-19. Path placeholders are those of the sealed brief `{LANE}/briefs/C1E-EDGE.md`.

## State: no series has been run

**No probe series and no `resources` run of this slice exists.** The brief allows a series only once `{LANE}/tools/README.md` exists and `{LANE}/tools/STATUS.md` is gone. The slice was ready to measure at 01:00 local time on 2026-09-19. From then the two conditions were checked every five minutes for ninety minutes, to 02:30, and neither held at any check. The last evidence file of T1-ORBIT-PROBE-2 under `{LANE}/probes/T1/` was written at 00:20:52 and nothing followed it. The rule for keeping the line has therefore not been applied, and nothing here says whether the owner's limits hold with the line.

`{LANE}/tools/orbit_probe.mjs` had SHA-256 `ac5290bcb93fba13f45c8f198b6cb13ee617a2b52766426cdc81f1d344598368` at this slice's launch and was changed by its own child at 00:17:17; its SHA-256 at the end of the wait is in this slice's return.

## What is ready for whoever runs the series

Two build copies are kept outside the repository, in the scratch directory the launch message names.

| Variant | Build | `index.html` SHA-256 |
|---|---|---|
| off | the base build at lane head `b415b43df`, supplied by the manager | `e79fafbe1595df3c446519a105447b8edd38afe8b79077f5cd9a245cf1bd7dfb` |
| on, stage 1 | the line on pipes only (`dist_stage1`) | `c43b10550124d44a9dbbf3b39ed5debb3ac3d936058f51c6e85748afb91af16e` |
| on, final | the line on pipes and on the four placeholder kinds (`dist_stage2`); `npm run build:desktop` on the returned tree reproduces this hash | `0933b9d42a30898a44b3c582ebd56359a1f8b2be4b0299598ad06f01259044d1` |

The pipe material's shader is the same in the stage-1 and the final build: stage 2 changed only which placeholder materials ask for an outline. Stage 3 was not built, so there is no third candidate.

The series the brief requires, all as interleaved pairs `off@<base>=;on@<final>=` with `--pairs 3`, labels on, ratio 2:

1. Decision set (six): 10,000 pipes, light, 1440x900, `schematic` and `actual-od`, each with `vsync` and `uncapped`; then 10,000 pipes `actual-od` with each pacing at the larger window that T1's summary names (T1's status note gives 1646x1168 for a 1000x828 canvas in today's layout).
2. Final set, beyond those six: the four 1440x900 series at 1,000 pipes; the two `schematic` series at the larger window; one dark series at 10,000 pipes.
3. `resources` on the final build at 1,000 and at 10,000 pipes.

## What the unmeasured line costs, by construction

These are statements about the code, not measurements.

- Draw calls, triangles, vertices, geometries, materials, textures, instance buffers and scene objects: unchanged. The line is a branch-free addition to the fragment shader of materials that already existed, and a per-vertex addition to their vertex shader.
- Per pipe fragment: six screen-space derivatives, three `length`, three divisions, one `smoothstep`, two `mix`, one `step`. Per pipe vertex (64 per instance: 22 on the side, 21 on each cap): one 4x4 matrix product, six dot products, one `atan`, one `acos`, two `sin`, two `cos`, two `normalize`.
- T1's status note records that on this host the uncapped orbit at 10,000 pipes is bound by the main thread (about 2.0 ms a frame, unchanged by ratio, window or a GPU synchronisation each frame) and that the tool does not see GPU cost under that bound. The line adds no main-thread work per frame. A series on this host may therefore show no difference for a reason that is about the host, and a GPU-bound host could differ.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
