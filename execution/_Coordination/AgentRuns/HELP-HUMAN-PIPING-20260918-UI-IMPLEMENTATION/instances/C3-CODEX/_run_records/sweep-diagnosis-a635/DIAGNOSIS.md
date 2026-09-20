# Edge-resource assertion diagnosis — no repair applied

WORKING_ITEMS /root/i1_manager, actual gpt-6-astra/high, under ROOT HELP_HUMAN.
No child dispatched. software-defect-diagnosis applied. Owned head 9fb4b88b, failed sweep
a635b01ad67e85a10b542e889f0de29715358e49. Entire desktop src Git tree matches:
24c4da4d47dd6b558f4198664a58411de8848db6. Seven relevant production/test files also
compared byte-for-byte with a635 and match (REPRODUCTION.json). No source/Git mutation.

## Reproduction

One selected test, one Vitest worker, reproduces exit 1 at viewportFigureEdge.test.ts:365:
expected [modelLayer, edged], received [modelLayer, edged, viewport-dimmed].
Raw command, Node version, source/brief/skill hashes and complete output are retained in
REPRODUCTION.json and edge-reproduction.log. Vitest 4.1.10: 1 failed, 23 skipped. No
other case, browser, native, build, full suite or performance probe was run. The original
full-sweep log/JSON was read only and remains a failure (1488 passed/3 failed; later
surfaces not_run), never substituted with this narrow reproduction.

## Cause and requirement

High confidence: legitimate C3 presentation resource plus an obsolete global scene-graph
assumption in an edge-specific test; not an edge-material product regression.

The same test's preceding assertions 343–358 pass: the edged mesh initially has no children,
no extra geometry attributes/userData structure, and exactly the same live resources as its
plain counterpart (1 mesh/geometry/material/matrix/colour buffer ; 0 textures).
Its last block then admits ONLY the edged mesh through ViewportResource.replaceLayer.
That method now calls prepareViewportDimming BEFORE ledger admission (resource.ts:736).
The preparation unconditionally creates one shared-geometry, cloned-material InstancedMesh
child for every registered nonempty chunk, independently of edge configuration
(viewportDimmingPresentation.ts:66–108). The observed extra object's name, empty ownership
list and 0.2 opacity match this C3 companion. Traversal therefore includes 3 objects.

Original C1E brief stage 1 requires that no object be added BY THE LINE. The C1E return
states the edge-on versus edge-off ledger is equal, with no added draw/geometry/material/
texture/buffer. FigureMaterial still implements the line within its existing shader and
was not changed by C3. Those requirements remain correct. The former total graph assertion
conflates zero edge incremental cost with the viewport's complete post-C3 allocation.
C3's additional companion was explicitly authorized/accounted separately; suppressing it
for edged meshes would break isolation. The loud unrelated JSDOM messages in the complete
sweep are not this failure's cause; the isolated mock-resource test reproduces the graph
mismatch without changing renderer construction.

## Precise proposed test-only repair for ROOT disposition

Keep all pre-admission assertions 343–358 unchanged. Replace only final admission block
360–365/comment with symmetrical plain and edged resource fixtures:

1. Admit plain through one edgeResource fixture and edged through a second identical
   edgeResource fixture, both at light/DPR 2, using replaceLayer.
2. For each, retain exact graph checks: one admitted mesh plus exactly one C3 companion;
   traversal exactly [layer, mesh, companion]; companion is an InstancedMesh sharing the
   parent's geometry, with count 0 while isolation is inactive. No arbitrary child exclusion.
3. Compare complete post-admission ledger.live objects between plain and edged, and assert
   exact C3-inclusive totals: pipeMeshes 2, geometries 1, materials 2, textures 0,
   instanceMatrices 2, instanceColors 2. All unmentioned live fields must still agree.
4. Retain geometry attribute equality and shared geometry checks, so an edge-introduced
   object/geometry/material/buffer/texture still fails. Explain that the companion belongs
   to common C3 presentation rather than the edge option. Do not merely change 2 to 3 or
   discard the failing check.

No production source, shader, picking test, first-profile evidence, tolerance or benchmark
limit changes. Existing C3 cycle/teardown tests already cover companion retirement; the
repair should preserve those, not duplicate a new lifecycle suite here. After disposition,
run the affected case first and then the 24-case edge file, one worker. ROOT retains the
whole-batch backcheck and fresh complete sweep; this diagnosis authorizes neither repair
nor qualification. All 83 protected files still match the C3 baseline.

Residual limitation: proposed post-admission equality is source-derived, not an executed
new probe or repaired-test pass. No drawing/GPU/performance result is claimed. C4 remains
held and unrelated. Standard F-PIP-2/DEC-081 fence applies.
