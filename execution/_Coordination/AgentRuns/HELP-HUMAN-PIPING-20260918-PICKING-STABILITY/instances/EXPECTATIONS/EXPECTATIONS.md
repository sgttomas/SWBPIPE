# Frozen independent analytic expectations

Status: FROZEN; implementation may proceed against these expectations. Basis: 1f977a9352ddd370ed54b8cb363b2a2777f56213. This derivative analytic packet consumes the retained POINT58_REPAIR_HANDOFF and POINT58_CAUSE/matrix.json; it does not amend authoritative decomposition, old failures, oracle, or numerical policy. Scope DEL-07-01 / PKG-07 / SOW-020 / OBJ-006; F-PIP-2 / DEC-081 applies.

## Independent mathematical contract

Let a unit ray be O+tD with t>=0 and a segment A+u(B-A) with 0<=u<=1. Capsule miss is min ||O+tD-A-u(B-A)|| / r over that constrained domain. For a sphere, closest-ray miss is ||(C-O)-max(0,(C-O)·D)D|| / r. Evaluate the small residual directly rather than subtracting large squared lengths. Intersection entry/exit remain signed line parameters and are separately clipped to the camera interval: do not clamp the sphere intersection center projection while solving its entry/exit.

An endpoint sharing a node is a genuine tie only when that endpoint is the constrained closest segment point, both requested radii agree, and entries belong to the same existing front group. Then both misses are the identical endpoint residual / r; existing node priority applies. Sharing an endpoint alone does NOT justify forcing a tie: a segment interior can be nearer, and different requested radii or entries can give a real distinction.

Keep relative entry grouping 1e-9 and absolute normalized-miss grouping 1e-12, plus component/support/node/pipe then displayed order/key. Neither tolerance nor frozen oracle identities may change.

## Recorded regression acceptance

The retained matrix has 32 calculation rows plus four float32 annotation rows. All 32 independent oracle winners are {type: node, id: node:UIF-08786}. Freeze those 32 identities across attempts 1/2/4/5, before/after reconstructed cameras, intended/delivered/X-only/Y-only pointer coordinates. Intended (839.3801013839062,657.5079070039999), delivered (839.380126953125,657.5079345703125). Require repaired exported picker to match all 32; baseline historical 28 mismatches remain unchanged evidence. Historical camera matrix bits are unavailable: this is reconstruction acceptance, not exact historical replay. Do not substitute the desired identity without verifying endpoint/radius/front conditions.

## Analytic control geometries

Use O=(0,0,0), D=(0,0,1), effective radius r=1, camera clip containing t=10, and primitive radii dominating the pixel floor. These exact-real constructions freeze expected behavior independently of implementation. Reversed segment endpoints must produce the same results.

| Control | Geometry | Expected |
|---|---|---|
| Endpoint tie | Node C=(0.6,0,10.8); capsule A=C, B=(2.6,0,10.8) | Both entry 10 and miss 0.6; node wins |
| Genuine pipe win at equal front | Node C=(0.6,0,10.8); capsule A=(0.2,0,10+sqrt(0.96)), B=(2.2,0,10+sqrt(0.96)) | Entries 10; misses node 0.6, pipe 0.2; pipe wins |
| Genuine node win at equal front | Node C=(0.2,0,10+sqrt(0.96)); capsule A=(0.6,0,10.8), B=(2.6,0,10.8) | Entries 10; misses node 0.2, pipe 0.6; node wins |
| Interior pipe control | Node C=(0.6,0,10.8); capsule A=(-2,0.2,10+sqrt(0.96)), B=(2,0.2,10+sqrt(0.96)) | Pipe closest point u=0.5, miss 0.2; entries 10; pipe wins |
| Front grouping dominates miss | Node C=(0,0,12); capsule A=(0.6,0,10.8), B=(2.6,0,10.8) | Pipe entry 10 precedes node entry 11; pipe wins despite node miss 0 |
| Zero segment | A=B=C=(0.6,0,10.8) | Capsule equivalent to sphere |
| Behind ray | Sphere C=(0.6,0,-10.8), capsule extending away in x at that z | No visible hit; do not turn an infinite-line closest point into a forward hit |

For an origin-containing shape, preserve signed interval math and valid near-plane clipping. For a ray-segment constrained closest point, if t clamps at 0, re-optimize u on the segment for O; if u clamps, recompute t at that endpoint. Independently clamping an unconstrained pair once is insufficient. Include parallel/nearly parallel segment axes, endpoint reversal, and zero-length segments to expose denominator/branch traps. Genuine near-distinct controls should also use miss differences above 1e-12 but far below 0.4; hold front entries tied analytically via z=10+sqrt(1-x*x), e.g. x=.6 versus .600001, where the smaller miss wins without a policy change.

## Translation and residual traps

Common exact translation of model, camera, and render origin preserves local inputs and therefore winners. Freeze controls with a large exactly representable common shift and corresponding render-origin shift; also exercise equivalent local-origin choices while preserving authored geometry. Numeric residual tolerance must reflect representability: do not require recovery of small geometry already rounded away in huge authored coordinates.

Recomputing distance from two far-away world points can still cancel. Prefer origin-relative residual components; endpoint reconstruction via midpoint+half-axis can differ from the literal stored endpoint and reintroduce sphere/capsule disagreement. Simply replacing the final squared-distance evaluation is not proof that the closest-point parameter itself is correct or stable. Do not force all attached pipes to the node miss, globally prefer nodes, widen epsilon, normalize against a different radius, or change ray clipping.

## Provenance and execution

Baseline product and oracle were read with git show (not mutable worktree source). SHA256:
- viewportSelection.ts: c0c09ebdb05a49565bf61e576ff0b391037916c614f5add8f05f4270539f5f8e
- point-hit-oracle.mjs: d9f4e01f80b62c777b3b48645b3d2180e7327fb0e335358c5aba6b4f96b992b4
- retained matrix.json: 08456c7abc10f96b443807718d229e8999cc84076e77e85f0a653fb178e09af8
- retained oracle-detail.mjs: 751b54a3bd26cc3d8c2574c036de96a7b8790585b6e234b4513dc3f82029ef45
- retained compare.cjs: 716e8bb746739de398107faeb27ffecf490712042bc7e8e0b6b9d690e136f50c

Commands: git rev-parse --show-toplevel; git show <basis>:<product/oracle paths>; read handoff/diagnosis/brief/project AGENTS/TASK; Python JSON inspection and hashlib digest calculation. One exploratory git show used a nonexistent packages/piping-ui path and failed harmlessly; corrected to the declared project viewport path. An initial matrix inspection assumed all rows had oracle fields; corrected to identify four annotations and 32 calculations. No runtime, build, browser, benchmark, or product tests executed. No implementation source examined or changed. Mathematical controls are derived, not claimed executed tests.

Closure: analytic freeze complete; product correctness remains open until implementation tests and fresh independent review. Next owner: root HELP_HUMAN to release implementation and retain these immutable expectations for acceptance. No blockers to implementation release. Configured gpt-6-astra/low per caller, no model introspection; Type2 scope/nondelegation instruction+config asserted.
