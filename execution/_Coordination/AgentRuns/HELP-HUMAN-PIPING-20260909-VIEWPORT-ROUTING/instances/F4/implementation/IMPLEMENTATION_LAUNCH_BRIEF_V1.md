# F4 bounded friction implementation launch brief V1

Status: **SEALED AND HELD — NOT DISPATCHABLE UNTIL ROOT SOURCE RELEASE**

RequestedBy: WORKING_ITEMS Agent 1 `F4`, parent `/root`
RunID: `HELP-HUMAN-PIPING-20260909-VIEWPORT-ROUTING`
ParentInstanceID: `F4`
ChildInstanceID: `F4_IMPL`
Role: TASK Agent 2; nondelegating; requested and exposed configuration must be `gpt-5.6-sol`, high reasoning
WorkingRoot: `{REPO_ROOT}/projects/chirality-piping`
ScopePath: `{RUN_ROOT}/instances/F4/implementation`
TaskSkill: `software-bounded-implementation`
PackageID: `PKG-04`
DeliverableIDs: [`DEL-04-04`]
ApplyEdits: true, only after the root release dependency below is present and verified

## Objective

Repair the confirmed seed-dependent static Coulomb branch defect without adding path history, a tolerance, a schema, or a public result contract. Preserve the explicitly deferred first sliding warm-start iterate, then explicitly re-stick every post-force sliding candidate whose final force/motion/current-normal branch is inadmissible. Preserve exact-limit sticking, zero-limit frictionless release, no-contact inactivity, current-normal affine coupling, deterministic simultaneous row updates, existing diagnostics, and the established two-iteration valid super-limit result.

## Accepted basis

- Source commit: `533332349a4607eee561d4ef90fb05a62d86519e`.
- `core/solver/nonlinear_supports/src/lib.rs`: SHA-256 `f6c6e62994564dddfc84e60df63f2e2f3a293b1d48a5950469c4368509733062`.
- `core/solver/nonlinear_integration/src/lib.rs`: SHA-256 `6e163a47db60288179f844d473992d28a53599ad2204149093186c760723ad46`.
- Reconciled plan: `instances/F4/FRICTION_REPAIR_IMPLEMENTATION_PLAN_V1.md`, SHA-256 `8651bad68ea72425ee00d17fea200d3c1883404fdd65fe52cbb7e5f4b4c38566`.
- Validation matrix: `instances/F4/FRICTION_REPAIR_VALIDATION_MATRIX_V1.md`, SHA-256 `91a2789b856cf5244dc942fb414e86753332bbc452b1859238e512ad23f4251b`.
- Independent design refutation: `instances/FREFUTE/design/DESIGN_REFUTATION_V1.md`, SHA-256 `2fb285c5e48f7b03b4ff690506c57a4adf159d8d26be11e6b5684320a5f5e6f9`; all conditions are incorporated in the reconciled plan.
- Current-SHA executable proof: `{EVALUATION_ROOT}/returns/NS/RUNTIME_PROBE.md`, SHA-256 `44ba3704f668f36c3aabe4fe9bc22919a9b4f461aad8ba3e614586466dbaa087`.
- Project method profile: `{PROJECT_ROOT}/software-workflow.json` under `docs/SOFTWARE_WORKFLOW_PROFILE.md`.

## Release dependency

The dispatch payload must add the path and SHA-256 of a root-issued amendment that changes `F4_IMPL` from `HELD` to released for this exact two-file fence. If the amendment is absent, mismatched, expands the fence, or either source preimage hash differs, stop with `FAILED_INPUTS` and make no source edit. A root-granted isolated Cargo target/build slot must also be named before any Cargo command.

## Tasks and exact algorithm

1. Before editing, resolve `REPO_ROOT`, verify `HEAD`, the two source hashes, the reconciled plan/matrix hashes, and the root release amendment. Record status without staging or changing Git.
2. In `nonlinear_supports`, remove the displacement-only prior-sliding override. Refactor active-set iteration result formation so an integration-supplied slice of resolved friction states, expressed with existing `SupportStateRecord`, is applied before changed-support, residual, convergence, and diagnostic calculation. Reject unknown, duplicate, or non-friction overrides. Do not add a field to any public input/result struct.
3. In `nonlinear_integration`, preserve the no-force first iterate for an initially sliding support as an explicit warm-start exception and keep `sliding_force_deferred` as its convergence block. Extend private sliding-solve evidence with per-support applied-force presence and derived signed-normal branch validity.
4. For each current sliding row after the deferred first iterate, resolve the next state from the candidate's final quantities and branch evidence, all rows together:
   - preserve `Inactive` when the existing contact convention gives `N<=0`;
   - for computed `L=mu*N==0`, no applied-force record and `u_t!=0`, resolve `Sliding` without requiring the numerical free-DOF reaction to equal exact zero;
   - for `L>0`, resolve `Sliding` only when an applied-force record exists, final `u_t!=0`, and both applied force and reported support force oppose `u_t`;
   - otherwise resolve `Sticking`, even when roundoff makes the base classifier report `|R_t|>L`.
5. Do not re-stick solely for a derived signed-normal branch mismatch. If contact and tangential direction are admissible, retain `Sliding`, block convergence with the existing derived-normal branch gate, and let the next iteration retry from the observed current normal sign. If contact or tangential direction is also invalid, those state rules govern. Preserve the existing PR-760 three-iteration sign-flip tests.
6. Apply resolved tangential states before active-set residual/convergence/diagnostic formation. Keep tangential consistency and derived-normal branch admissibility as separate defensive convergence gates. An inconsistent tangential post-force branch must change next state; merely setting convergence false is insufficient.
7. Preserve the existing exact constrained sticking solve and inclusive `|R_t|<=L` comparator. Below and at the computed limit stick; the adjacent representable value above slides. Add no epsilon band.
8. Preserve existing force/reaction signs, same-iterate derived-normal simultaneous affine coupling, signed-normal branch validation, support/state ordering, nonconvergence code/severity, and zero-coefficient/no-contact behavior. Update assumptions that currently describe unconditional anti-chatter persistence.
9. Add co-located tests in the same two files implementing every applicable row of the frozen validation matrix. Include the exact current defect; both load signs, seeds and modes; below/equal/above representable boundary; `mu=0`; `N<=0`; valid super-limit two-iteration compatibility; sub-limit sliding-seed three-iteration recovery and cap-2 honest failure; current derived normal including the normal-branch-only 3-iteration retry; mixed sub/super rows with swapped order; and roundoff-shaped base-cone disagreement that proves explicit re-sticking.
10. Run the authorized focused checks only in the root-assigned isolated target. Validate the final changed paths, capture source hashes and diff hash, and return. Do not stage, commit, build-integrate, or dispatch review.

## Allowed write targets

- `{PROJECT_ROOT}/core/solver/nonlinear_supports/src/lib.rs`
- `{PROJECT_ROOT}/core/solver/nonlinear_integration/src/lib.rs`
- `{RUN_ROOT}/instances/F4/implementation/**` for TASK run record, return, validation, status, and evidence only; `IMPLEMENTATION_LAUNCH_BRIEF_V1.md` itself is immutable

## Permitted tools and checks

Repository read/search, `apply_patch`, SHA-256, read-only Git status/diff, and the skill's repository-native scope/check helpers are permitted. Root specifically authorizes the following focused Cargo commands only after assigning a unique `CARGO_TARGET_DIR`; that explicit grant controls over the skill's default registered-check preference:

```text
cargo test --manifest-path projects/chirality-piping/core/solver/nonlinear_supports/Cargo.toml
cargo test --manifest-path projects/chirality-piping/core/solver/nonlinear_integration/Cargo.toml
```

Do not run a blanket repository build, install, network command, evidence sweep, downstream benchmark, product crate, or shared-target command in this child. Those run after the source diff is frozen and independently reviewed, under manager/root scheduling.

## Expected outputs

- the two implemented source files with co-located tests;
- `{ScopePath}/RETURN.md` describing behavior, exact files, pre/post hashes, diff hash, test results, residual risks, and any needed amendment;
- `{ScopePath}/VALIDATION.json` with normalized commands, isolated target, exit codes, test counts where exposed, source/diff hashes, scope check, and confirmation that no external solver comparison ran;
- `{ScopePath}/STATUS.json` with `SUCCESS | FAILED | FAILED_INPUTS`, source-effect status, review hold, and next owner;
- the required `{ScopePath}/_run_records/TASK_RUN_*.md` following `AGENT_TASK.md`.

## Acceptance criteria

- Only the two source files and F4 implementation evidence changed.
- All matrix oracles added within the fence pass in both solve modes where specified.
- Valid super-limit sliding from either seed remains a two-iteration result; sub-limit sliding seed re-sticks after its post-force trial and converges on iteration 3; cap 2 fails visibly.
- The exact adjacent-representable boundary cases prove no hidden tolerance.
- Zero-limit behavior depends on computed zero limit plus absence of an applied branch, not exact numerical reaction equality.
- A post-force inconsistent row explicitly becomes sticking before residual/convergence calculation, including a case where the base cone comparison alone says sliding.
- Same-iterate current-normal, derived-normal-only sign-flip retry, and multi-row order oracles remain unchanged.
- No public struct/schema, state variant, diagnostic code/severity, tolerance, history, product adapter, or dependency/lifecycle surface changes.
- Both focused Cargo commands pass in the isolated root-granted slot.

## Exclusions and escalation

No writes outside the exact targets; no Git mutation; no benchmark/product/deliverable/authority/dependency/DAG/lifecycle/decision/receipt change; no external solver claim; no broader NS-02/03/04 repair. If the method requires another source path, a public contract, a new tolerance/history/state, or a consequential choice beyond the reconciled plan, stop without widening scope and return the evidence to F4/root.

The completed implementation remains held for a fresh read-only Agent 2 review of 100% of the frozen diff. It is not accepted, publishable, or ready for CHANGE until that review passes and WORKING_ITEMS validates fan-in.
