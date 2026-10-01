# Shared-estimator dependency and interface proposal

Proposal only; no manifest/lock/source edit has been made.

## Concrete dependency change

Add to VR/Cargo.toml dependencies:
`open_pipe_stress_solver_performance_harness = { path = "../../../core/solver/performance_harness" }`.

Keep the one kernel phase formula in H/src/k6/w1/counts.rs (or a later explicitly
authorized shared module owned by H). VR composes the same kernel result with
its own caller/global-envelope facts. Do not keep a second constants/formula port.

H's library has no global allocator registration. Its allocator belongs only
to k6_observe and k6_alloc, as declared in H/Cargo.toml. Normal dependency use
does not run/build H's bin as an application. Additional local dependencies
still add compile work to VR and must be considered in a later ROOT slot.

Current H lock has nine local packages; VR already has FK and sparse_direct.
The proposed dependency adds these seven package nodes to VR/Cargo.lock:

- open_pipe_stress_solver_performance_harness
- open_pipe_stress_nonlinear_integration
- open_pipe_stress_solver_diagnostics
- open_pipe_stress_curved_bend
- open_pipe_stress_nonlinear_supports
- open_pipe_stress_linear_supports
- open_pipe_stress_primitive_loads

Every edge in this closure was read from current Cargo.toml files; all are
path dependencies. No new registry package/version is indicated. Existing VR
serde_json/serde_core/serde/itoa/memchr/zmij and lock-only derive dependencies
retain their current identities. This is a manifest/lock graph conclusion,
not a cargo resolution/build result. H/Cargo.lock does not logically change
from this one-way dependency; no other lockfile edit is proposed. Future
Cargo output must be inspected rather than accepted wholesale.

VR seeded-faults forwards FK/mutation-controls. H also uses the same FK package
through path dependencies, so final feature unification must be checked for
normal and mutation builds. Do not let a normal admission claim inherit
test/mutation-only allocations. The new dependency creates no package cycle.

## Proposed data contract

The shared kernel entry takes `KernelShape` and `KernelAllocationFacts`.
Shape includes N/m/s/d/r/l/t/u/B/b/n/f/z/h, row/pre-dedup/free-degree and block/body
counts, exact encoding/string lengths and relevant fixture/source identities.
AllocationFacts separates target/type layouts from library capacity rules.
It returns named source/preparation/shared/solve/verify/shift/report/decision/
finish/refusal phase owner sums, plus max and selected-128 path max.

H and VR adapters obtain those descriptors from their actual model/source,
not from stale saved estimates. A counts-file route checks its model/source
identity against the current model and binds the missing shape facts without
requiring new numerical work. Original count/non-estimate output fields need
not change.

Consumer composition takes an explicit metric identity and named owners:

- HSourceSolveStages: model/frames/Args/runtime/saved attempts/prefix controls
  and formatting before stage end.
- VkScaleGlobal: selected Case/Model/Args/runtime plus the finite parse/count/
  control/lane/output/RCM/sparse phases in FORMULAS.

An unresolved layout/capacity/input cell or checked-u128 overflow returns
a named unavailable result; it does not silently substitute zero. How that
error maps to current CLI behavior is an implementation decision requiring
review; this packet does not change accepted admission rules.

Existing public estimate functions may remain compatibility adapters, but
they cannot pretend current W1Counts/VR Counts alone encode every required
capacity/string/consumer fact. Preserve the 193-case count/estimate test and
the 33 H fixture table, adding real spring/directional/body/multiplicity
descriptors. A generic source formula may conservatively overbound a branch;
a domain refusal is not a replacement for supporting an existing test case.

## Additional later write scope to name explicitly

VR/Cargo.toml, VR/Cargo.lock and its already planned scale/example/tests paths
are concrete. H counts.rs/tests/observations remain as originally proposed.

If the selected interface supplies model/caller facts at the existing binary
call sites, **H/src/bin/k6_observe/main.rs** is needed: estimate is invoked there,
not only in w1.rs. Original I21's list did not name main.rs. ROOT must explicitly
grant that path or select a compatible mechanism before implementation.
A changed H count-file parser would additionally require H/src/k6/counts.rs;
prefer deriving descriptors from the already loaded model at the binary call
site so no parser/schema change is needed. No new source path is granted here.

Final A1 reconciliation, independent full-bound review, implementation tests/
mutants, historical admission replay and ROOT's measurement slot still follow.

