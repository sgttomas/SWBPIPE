# Complete frozen-candidate independent review

Disposition: **HOLD for one actionable apply-boundary defect.** The complete 19-file source/test/manifest diff was reviewed. Reviewer TASK `/root/authoring_manager/self_weight_review`, parent `/root/authoring_manager`, used the project software-code-review skill and authored no product implementation.

Base: `f702b439536c6e76af8e1c81ee536d5685e87907`. Patch SHA256: `5d9134f528a4f423d105a07575a9dc1c9fe53ef663678b41b89ea5033278fa12`. The frozen manifest and patch are copied in [complete_review](complete_review/). All 19 current hashes matched. The read-only scope validator passed with the 19 files and bounded manager evidence directory allowed. [MANIFEST.json](complete_review/MANIFEST.json) records commands, hashes, parentage, and execution limits.

## P1 — Direct generated-refresh apply can overwrite a modified primitive

Location: `core/model_operations/operation_applier/src/rich_authoring.rs:1028–1048`, with the overwrite at `:1068–1069` in this frozen source.

The GENERATED_V2 branch validates historical-record shape, pipe identity, allowed changed fields, the new snapshot, and current full-model hash. It never establishes that the current primitive still has its exact historical generated payload. The magnitude is explicitly omitted from the unchanged-field comparison at line 1041. The normal planner's inspector checks eligibility, but public single-operation and imported-batch routes call this applier directly without authenticating a planner result.

Concrete trigger: take a valid v2 generated load, explicitly change its magnitude to −30 N/m, and retain its generation provenance. Submit a generated_self_weight update with the exact current before-array/current hash, an after-array restoring the source-consistent magnitude, and consistent new v2 snapshot/provenance. Preserve all other fields and any legacy string. The shown guards pass and replace the manual magnitude. The resulting model can pass the inspector as Fresh because the replacement matches the source. An ambiguous or modified v1 magnitude has the same missing eligibility gate.

The manager confirmed that this is not an intentional raw-edit escape: modified records may only be explicitly preserved as manual through this route. This is a source-level contract defect. The reviewer did not execute the trigger under the no-test boundary.

Repair: expose a source-owned pure historical eligibility check from self_weight, using exact original-payload comparison and conservative v1 recognition, and require it before managed overwrite. Share it with inspection. Do not duplicate a legacy mass formula, infer intent with tolerance, or trust a submitted status. Retain exact manual-preservation behavior. The proposed dependency edge requires its owning integration approval and actual-candidate checks. Add direct API negative controls for modified v2 and ambiguous/modified v1, plus valid migration/refresh/manual positives.

The current standalone applier fixture is only structural: it records mass 1 for OD .1 / wall .01 / density 1000 and lacks some historical fields/deterministic identity. A historical eligibility check should reject it. Use source-faithful fixture records instead of weakening the new guard to retain those passes.

## Prior repairs and full review coverage

- Retargeted manual apply is repaired: original-pipe binding moved into managed refresh; manual preservation validates the current target and exact physical retention.
- Deleted original source is repaired: historical validation and payload comparison precede current-source generation. The connected imported-retarget test applies, reopens, and solves a fixed −30 N/m load against independent force/moment values.
- Mill deduction is repaired to Dimension::Length. The connected real record includes 1 mm deduction, contents, and insulation, doubles density, refreshes through the applier, reopens, and solves against independent mass/equilibrium calculations.
- New v2 requires the stable mass tag and source-area calculation; v1 retains the unchanged historical helper. Known v1 requires explicit method refresh, with literal legacy-string retention and bounded carry-forward. Unknown v2 methods fail closed. Retained physical inputs independently constrain the snapshot and mass.
- The geometry helper matches the exact PHYSICS handoff. Pressure_exact changes are visibility only: no Scaled numerical expressions, pressure formulas, DTOs, or package version changed. Product lib changes are the geometry module declaration and pre-normalization validation seam.
- Shared section/cache resolution remains on a clone; exact normalized dependencies exclude unrelated stiffness/provenance/span changes. Normal solver validation still owns invalid base models. Positive area/mass/intensity range failures are explicit; solid bore/zero insulation controls remain allowed.
- Optional batch source hash is compared with the complete initial document before simulation. Without the field, prior caller-hash/preflight/simulation behavior remains. Before-array, count/order/IDs, unchanged records, compatible units, and atomic publication remain enforced. Those guards do not cure the eligibility defect.
- UI strict refresh request, preserve checkbox, source-bound queueing, and existing model/input/selection/epoch/busy invalidation were traced. Native commands forward Value to the same core functions. No native command or persisted DTO changed. Prepared real-WASM UI tests were reviewed but not run.

No other actionable source finding was identified. The manual-detach route remains distributed-force/global-axis/current-element specific; no broader category-conversion capability is claimed. Literal legacy-string retention is required for method refresh; manual provenance currently retains the parsed original record as an object, as shown in code and UI tests.

## Evidence and outstanding work

All six retained log hashes match their result records. Counts are 21 core, 5 area, 9 connected, 5 facade, 7 applier, and 10 ordinary batch: **57 passed**. Filtered tests in the ordinary-batch invocation are excluded. Suite records retain manifest hashes; full source/test binding relies on the manager freeze and execution transcript. The actual mass-probe result separately records reviewed self_weight/helper/pressure hashes.

The post-stable probe records seven ready outcomes and seven direct/JSON equal outcomes against this reviewer's reference hash `1d91c763caf4a1aedd4efdc4b67603392f7e5da3cfb58abc48e32de36dfdc6c5`. Maximum recorded relative error is approximately 1.95e−16 under the declared 1e−12 relative criterion with no absolute floor. Original failed parity and wrong-mass evidence remains. These are generated-load arithmetic controls, not thin-pipe structural qualification.

Outstanding: source guard correction and adversarial direct-API evidence; revised-candidate review coverage; conditional maintained WASM generation, focused UI tests, TypeScript check; applicable ROOT native/final clean-candidate checks. No product test/build/native/browser/network/Git mutation was performed by this reviewer. This hold is for a concrete source defect and does not establish UI runtime success, engineering acceptance, whole-M35 closure, or merge readiness.
