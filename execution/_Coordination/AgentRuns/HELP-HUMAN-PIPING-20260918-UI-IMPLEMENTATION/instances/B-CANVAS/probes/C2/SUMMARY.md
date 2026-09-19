# C2-HALO: what was built, the two build copies, and what to measure

Written by C2-HALO (TASK, Claude Fable 5.1) on 2026-09-19. **No timed series was run by this child**: the brief leaves measurement to the lane's manager. Nothing below is a performance finding; the counts are by construction.

## Build copies (outside the repository, in the child's scratch folder)

| Copy | Form | `index.html` SHA-256 |
|---|---|---|
| `dist_c2_working` | (b): halo drawn over nearer geometry, masked by depth; renderer construction unchanged | `e9aca49e81b340ff09d61d8b1628d544d966cfb6db305b3b357b1d6b7e028db3` |
| `dist_c2_stencil` | (c): the same meshes and shaders, masked by a stencil buffer (`stencil: true` on the renderer) | `3c53bf6412961bdaf8fbeefc8fab643677f7218fa08d16ed72e6ff74b82f2922` |
| base (lane head, the manager's) | selection recolours | `fc785c1b16e5ca81f7a44af1e9ff2ef76a965922fd0dc8e07ea64b85e6a8180c` |

`npm run build:desktop` on the returned tree reproduces the working copy's hash, and so did the dist lane's own build. `stencil_candidate.patch` in this folder is the whole difference between the two forms (one line of `viewportResource.ts`, two material blocks of `viewportHalo.ts`); `git apply --check` accepts it over the working form. It is not applied in the tree.

Eighteen fixture pictures (10,000 pipes; one pipe selected, and everything selected; both themes; ratios 1 and 2) are equal pixel for pixel between the two copies.

## What a halo costs, by construction

Per shape that has a haloed element (a "shape" is one base geometry: the pipe prism, the node sphere, the support cone, and one per placeholder kind: seven at most), and per kind of halo (selection, hover): two instanced draws, one mask and one shell, whatever the number of haloed elements and of spatial chunks. With nothing selected and nothing hovered: no object, no draw, nothing in the ledger.

| Shape | Figure (per instance) | Mask (per haloed instance) | Shell (per haloed instance) |
|---|---|---|---|
| Pipe prism, 10 sides | 64 vertices, 40 triangles | the same geometry: 64, 40 | 30 edges: 120 vertices, 60 triangles |
| Node sphere 12 × 8 | 117 vertices, 168 triangles | 117, 168 | the same geometry grown along its normals: 117, 168 |
| Support cone, 4 sides | | the same geometry | 8 edges: 32 vertices, 16 triangles |
| Rigid box | 24, 12 | 24, 12 | 12 edges: 48, 24 |
| Branch prism (10) and expansion prism (12) | | the same geometry | 30 and 36 edges |
| Bend torus arc 8 × 18 | | the same geometry | 296 edges: 1,184 vertices, 592 triangles |

All 10,000 pipes selected: 2 draws, 1.84 M vertices and 1.0 M triangles a frame on top of the figure. Everything selected in the 10,000-pipe fixture (10,000 pipes, 10,001 nodes, 500 supports, 100 placeholders) adds the spheres' 2 draws, 2.34 M vertices and 3.36 M triangles, and a few small draws. The shells' fragment cost is small: almost every shell fragment fails the depth test against the mask (early), and the mask writes no colour.

Halo meshes are not frustum-culled (one mesh holds a shape's haloed instances from every chunk), so a close view with everything selected still submits every haloed instance's vertices. That is the case to watch in the orbit series at 10,000 selected, in both geometry modes.

Memory: 16 floats per haloed instance per mesh, capacity in powers of two and never shrunk while the shape lives: 16,384 × 64 bytes × 2 = 2.1 MB for all 10,000 pipes. The update on a change of selection: one pass to count and one to copy, no allocation per key; 1.0 ms median for 10,000 selected of 20,001 elements in the unit test on this host (an observation in jsdom, not a browser measurement). The per-instance colour repaint that a selection change already did is unchanged.

## Ledger counts, exactly (unit-tested)

- No halo: nothing. The ledger equals a resource's with no halo object at all.
- First selected pipe: `instanceMatrices` +2 (mask and shell meshes), `geometries` +1 (the prism's edge geometry), `materials` +2 (the one mask material; the selection edge-shell material).
- A selected node after that: `instanceMatrices` +2, `geometries` +0 (the sphere's shell is the sphere's own geometry), `materials` +1 (the selection normal-shell material).
- A hovered pipe after that: `instanceMatrices` +2, `geometries` +0 (shares the prism's edge geometry), `materials` +1 (the hover edge-shell material).
- Ceiling: five materials (mask; selection and hover, each of edge and normal form); one edge geometry per non-sphere shape; two instance buffers per shape and kind of halo.
- `textures`, `instanceColors` and the four mesh-kind counts never change: a halo mesh has no ownership kind and no instance colour, and no material has a texture.
- A mask never owns its geometry: the layer that built it counts and disposes it. Growth disposes the pair it replaces (+2 created, +2 disposed); a replaced layer disposes the pairs and edge geometries of the shapes that left; `dispose()` returns the ledger to the figure's own counts. `created − disposed = live` is asserted after each.

## What the manager might run

The orbit series of D-72 on `dist_c2_working` and `dist_c2_stencil` against the base, at 1,000 and 10,000 pipes, in both geometry modes, with 0, 1, 100 and all pipes selected; the point-pick and box series (a selection change now costs the halo update as well as the repaint it already cost); the settle check (the halo requests no frame of its own: it draws only in frames something else asked for); `resources` before and after select-all and deselect.
