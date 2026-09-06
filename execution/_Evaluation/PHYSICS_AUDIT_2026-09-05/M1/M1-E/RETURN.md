# M1-E elements and assembly audit return

RUN_STATUS: SUCCESS
ControlSurface: MERGED
TaskProfile: NONE
TaskSkill: NONE
ScopePath: {WORKING_ROOT}/execution/_Evaluation/PHYSICS_AUDIT_2026-09-05/M1/M1-E
ToolsUsed:
- shell git (read-only source basis and status)
- shell rg, sed, cat, wc, shasum (read/search)
- python3 compare.py (independent numerical references)
- cargo run --manifest-path harness/Cargo.toml --offline (isolated actual production-crate calls)
ToolPolicyCompliance: PASS
WriteAuthorization: ALLOWED_WRITE_TARGETS
ActualModel: UNKNOWN (not exposed)
Nondelegation: instruction+config asserted; no child spawned.

## Basis and coverage

Derivative evidence against source `2be412ccea62bdc4bd96deb082c46d7a792076ea`, approved plan SHA `4732089daf9fbed72d25cbe9e8c57072211b64eb5956d55f096a340d5176d4c2`, decomp0.12/SCA009/DAG010. All production bodies in frame_kernel, straight_pipe, curved_bend, and linear_supports were read: respectively lines1–1169,1–1480,1–1062,1–672. `SOURCE_MANIFEST.json` binds hashes and baseline equality. Selected product consumers and representative existing test bodies were inspected. Full product, linear solver numerics, and nonlinear algorithms are other specialists' responsibilities.

Relevant requirements: frame DEL-04-01 REQ001–012; straight DEL-04-02 RQ001–007; linear supports DEL-04-03 R01–R12; expansion-joint DEL-03-06 R001/005/007/008. Contract OPS-K-MECH1/2, UNIT1, DATA2, SOLVER1 and REPORT1 apply. `COVERAGE.csv` records reachability and exclusions. Paired deliverable status/memory preserve IN_PROGRESS; DEL-04-01 mechanics assessment owner hold and rigid-component PDU013 COG hold remain unchanged. Rigid/semi-rigid component slots are data/provenance evidence, not an implemented rigid mechanical element.

## Reproduced findings

1. **M1-E-001 — finite-length user-stiffness objectivity concern, high severity/high confidence.** A 2m joint with lateral stiffness2000N/m reacts to a rigid infinitesimal-rotation basis with ±4000N transverse forces, unbalanced internal moment8000Nm and energy4000J. The ordinary frame gives zero for that basis. This is linear algebra: multiplying the entire basis by0.001 yields ±4N,8Nm and0.004J, so the finding does not rely on a finite one-radian rotation. `UserStiffnessElement` links relative endpoint translations independently of rotations. The product maps this element to distinct pipe endpoint coordinates; it is not a zero-length coincident spring. Constitutive intent must be resolved before selecting a remedy.
2. **M1-E-002 — support API validation, medium/high confidence.** Public `SupportQuantity::new(-123, TranslationalStiffness)` is accepted as a spring. Separately, a positive spring with two declared DOFs is accepted and only the first DOF is prepared. These are lower-level callable contracts; no claim that native schema validation accepts them.
3. **M1-E-003 — arithmetic overflow, medium/high confidence.** Finite positive section inputs can produce `Ok` stiffness with40 nonfinite entries. Extreme synthetic robustness case; no realistic-model numerical failure demonstrated. Assembly/recovery callers need consistent finite-result checking, subject to M1-L cross-checking.
4. **M1-E-004 — direct point-load station validation, low/high confidence.** Direct station recovery silently ignores a public point-force literal at fraction2 although constructor/equivalent-load APIs reject out-of-range station values. The product's current interpolation path does not call this direct API; future reuse should not rely on constructor-only validation.

`FINDINGS.csv` separates severity from reproduced confidence, source corroboration, and unknown coverage. None of these findings has been repaired.

## W3 and W5 cross-manager evidence

**W3:** actual frame solve reproduces root force[-120,-350,+240]N and root moment[-80,-420,-745]Nm from independent statics. Product source norm includes declared rotational DOFs and labels the combined norm N. Actual product-path reproduction is an explicit I1 dependency; M1-E does not claim to have executed it.

**W5:** actual straight kernel under a clamped/clamped L2m member with q=-100N/m returns root Mz=100/3Nm, midpoint Mz=-50/3Nm, matching independent segment equilibrium. The straight kernel's own station algorithm is correct for this witness. Product source bypasses it and interpolates raw signed opposite-end actions; actual product-path reproduction belongs to I1. This also motivates checking even unloaded-span sign conventions, not just distributed-load polynomial shape.

## Positive bounded evidence

- Mixed axial/bending-y/bending-z/torsion cantilever tip response differs from independent elementary-beam calculation by at most5.43e-20 in native displacement/rotation entries; root reactions differ by1.14e-13 in their respective units.
- Straight-element reversal gives exactly equal permuted stiffness. Six rigid modes produce zero or roundoff-scale residuals. Twelve deterministically rotated, translated two-straight-element assemblies preserve assembly energy and local/global work to4.27e-14J absolute for the specimens.
- Partial uniform span0.2–0.7 plus point force at0.4 conserves resultant force and moment in all three load directions. Weight hook yields23×9.81=225.63N/m; fixed axial effect120N remains constant at three stations. These are invented fixtures, not physical acceptance values.
- Ten curved angle/factor combinations (five angles0.05–2.5rad, factors1/1 and2/3) agree with independent geometry-based Simpson integration of all36 flexibility entries: maximum error normalized by maximum expected matrix entry2.03e-13. Fifty arc-station outputs combining uniform and radial pressure loading agree to2.49e-13 normalized by the specimen maximum. This norm mixes component magnitudes only as a diagnostic relative scaling; individual physical units remain in raw outputs and no engineering acceptance criterion is implied.
- Twenty consistent curved load vectors satisfy independently integrated virtual work (uniform and radial pressure): maximum normalized difference1.76e-11 after increasing inner quadrature64→256. Both outputs are retained; reference refinement was numerical convergence work, not alteration of expectations to match production. Uniform and pressure resultant/moment identities also agree for these cases.
- Existing V1 suites report frame36, straight33, curved23, linear-support14 passed. `EXISTING_TEST_EVIDENCE.json` binds V1 source record. Existing tests corroborate regression coverage but are not independent physical validation.

## Decisions and repair boundaries

For M1-E-001, do not silently add beam coupling, penalty rigidity, or a new flexibility formulation. Two materially different contracts are possible: an objective finite-end connector with explicitly defined reference point/transport of endpoint motion and supplied lateral/angular stiffness meaning; or an explicitly external-frame relative-translation spring model whose restrictions and external reaction interpretation are documented. A zero-length spring interpretation cannot simply be assumed because current constructor rejects coincident coordinates. Owner/architecture must select the physical contract. A bounded reproducer/diagnostic or temporary explicit limitation can be prepared without adopting new physics; no production mutation before whole-baseline acceptance.

The validation findings can be repaired at their existing API boundaries without new engineering thresholds: reject shapes/values inconsistent with the declared scalar spring and station contracts, and return existing error types for nonfinite computed values. Verify callers before claiming public compatibility unchanged. W3/W5 repairs must consume I1's actual product evidence and cross-check schemas/sign conventions.

## Reproduction

From this directory, use an isolated target (manager slot required for compilation):

```sh
CARGO_NET_OFFLINE=true CARGO_TARGET_DIR=./harness/target cargo run --manifest-path harness/Cargo.toml --offline > replay_raw.json
python3 compare.py
```

`compare.py` reads `raw.json`; compare replay_raw.json with the immutable baseline raw.json first. To re-evaluate changed source, copy the bundle to a new authorized snapshot location and use the replay as that copy's raw.json. Do not overwrite the frozen evidence. Run from the same relative position under WORKING_ROOT, or rebase only the harness's local dependency paths in a new recorded copy. Preserve expected equations and baseline output. Initial compile had one JSON-macro syntax error, fixed only in the harness; initial source/error retained under `_run_records`. Both authorized successful Rust runs completed; no suite was duplicated.

Outputs:
- RETURN.md, FINDINGS.csv, COVERAGE.csv, REFERENCES.md, SOURCE_MANIFEST.json
- harness/, raw.json, expected.json, comparison.json, curved_load_work.json, compare.py
- EXISTING_TEST_EVIDENCE.json, _run_records/, MANIFEST.json

MISSING:
- Arbitrary3D rotated curved-load reference, full mixed curved/straight/user product model, hardware-specific connector physical contract, and broad geometry/conditioning envelope are not independently established here.
- No new independent thermal/pressure constitutive or material model audit; axial hook only. Those belong I1/V1.
- Full source production logic is reviewed, but no claim of branch-complete new testing or exhaustive malformed-input exploration.

NEEDS_HUMAN_RULING:
- Finite-end user-stiffness constitutive semantics/remedy if not already defined by a governing decision; this witness supplies no manufacturer/engineering model.

DEPENDENCY_NOTES:
- I1 actual W3/W5 product reproductions; M1-L arithmetic/scaling review; V1 independent reference review.
- Complete whole baseline and fresh independent report review before any production repair.

AppliedChanges:
- Evidence and harness files only inside assigned M1-E subtree; production hashes unchanged.

## Handoff

Closure verdict: bounded audit execution complete with open findings and named gaps; not an engine-correctness, lifecycle, release, or engineering acceptance verdict. Accepted upstream remains the frozen source/plan/decomposition basis above. This packet is derivative; no authoritative deliverable state, gate, or pointer changed. Manager must validate this return, reproduce critical evidence, and consume the immutable MANIFEST. Production source changes invalidate source-bound observations and require rerunning original witnesses into a new snapshot.
