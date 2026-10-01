# RV30 layout08_03 — concrete archive source review

**Ready at the source-review gate for one bounded standalone release run of the sealed overlay. No blocking finding.** ROOT retains runtime-slot/guard release authority. I did not compile or execute anything, and this return grants no runtime authorization or binding of measurement values.

Same independent TASK Type2 RV30, native child `/root/rv30_k6c_kernel` of ROOT `/root` HELP_HUMAN Agent0; no delegation. Actual start2026-10-01 13:23:24 UTC; deadline13:38:24 UTC. Completion is recorded after checks in VERIFICATION.json. The direct parent follow-up supplies this15-minute source-only scope and additive write fence. Instruction/skill origin hashes remain in ../kernel_01/INSTRUCTION_BINDING.json.

Reviewed K6C R/I21/layout08_prep:
- seal `d52b1f699836434850b696690f0bc8ba4ac6638e98f57f74784438bd729e0716`;
- full217-line OVERLAY.diff `0d6c1ff750241f76ffb15a590ae76b70b23581013e32fb78fb10899214d0d13a`;
- source40129a225d73860ac2a53da9a2fa73869df668f3;
- prepared archive `<WT>/scratch/i21/layout08/archive`.

The entire diff, ISOLATION.md, RETURN.md, RUNTIME_COMMAND.txt, type map and binding inventories were read. Every sealed payload was independently hashed. The source07 proposal remains the reviewed design basis.

## Exact archive and file checks

I independently read all227 original archive files from immutable40129 using read-only `git archive` in memory and compared their hashes to SOURCE_ORIGINALS. I also verified the existing source.tar's227 entries and pinned hash; no nonregular entry/symlink was present. I then hashed all228 current archive files and matched the complete ARCHIVE_FILES inventory.

Exactly:
- adaptive.rs has the append-only fixed hook; every original byte remains its prefix.
- structural.rs differs only by the two-line retained_api hook export.
- H/examples/i21_kernel_layout.rs is the sole new file.
-225 original files are unchanged. Independently regenerated unified diff equals the entire sealed OVERLAY.diff.
- All18 manifests/locks and K6Alloc are unchanged. H's lock contains nine local packages. All20 manifest path-dependency edges resolve inside the archive. No build script or package build override is present.
- The three concrete-container type-map line numbers match the actual archive text. No RuleTest, Kind, tracker, Dof, SpringKind, tuple-field or other target definition changed.
- Sibling `<WT>/k6c-layout08-target` was empty at review. Log/target are outside the archive. No Cargo config was found at the archive cwd/ancestors or effective Cargo home, and no checked relevant Rust/profile/target override environment variable was present in this reviewer process. These are review-time observations, not a guarantee about a later launch environment.

Detailed hashes/commands are in SOURCE_VERIFICATION.json; tar/dependency/isolation checks are in ISOLATION_CHECK.json.

## Concrete call path and ownership

All references below are the overlaid archive source. A=FK/src/structural/retained/adaptive.rs; E=H/examples/i21_kernel_layout.rs.

A:4533-4657 implements three direct concrete blocks:
BTreeMap<u32,usize> at4548;
BTreeMap<(RuleTest,u32,Kind),BoundedExtremeTracker> at4585;
BTreeSet<(RuleTest,u32,Kind)> at4624.
The set therefore instantiates actual std SetValZST; no unit-map substitute exists.

The proposal's two-array return adds a third fixed [bool;3] content-verdict array. That is a narrow implementation detail implementing the proposed post-sample checks, not a new heap report or scope expansion. Samples, booleans, fixed operands, array IntoIter state, options and callback pointers are inline/stack data.

Each block builds exactly12 operands before the first baseline. The tracker constructor gives two empty Vec children and scalar fields; no offer, lazy-table population, model or solver call occurs. There are exactly1+10+1 operand extractions. unwrap is reached only with an available array element, and extraction occurs before the first/split sampling window. The fitting insertion loop uses `&=`, so insertions are evaluated even if a prior uniqueness flag were false.

A:4549-4576,4586-4616,4625-4653 each implement:
1. operand extraction before baseline;
2. first insertion, live-container black_box, then current/calls;
3. ten fitting insertions;
4. twelfth operand extraction before the split baseline;
5. twelfth insertion, live-container black_box, then current/calls;
6. allocation-free content iteration, another live reference, explicit drop;
7. current/calls after drop.

The contents checks retain all keys and values through the post-insert sample. Tracker rows check expected keys, empty children, zero offered count, no best value and normal tracker limit. No child heap is created by these comparisons or drops. All operands have been consumed before the empty array iterator drops. The three containers run sequentially, and none survives the hook's return.

The two callback function pointers supplied by E:21-22 are exactly the unchanged allocator's current/calls functions. The example registers that existing allocator file at E:4-8. Other archived allocator declarations belong to separate bin/test targets, not this example. Main calls no worker, file/env/CLI parser, logger, framework or numerical path. The archive contains no discovered constructor/link-section/export/thread-spawn/rayon marker or build script; its nine local crates have no external locked dependency startup path. This corroborates the direct call-path inspection; the search alone is not the isolation proof.

Source isolation is sufficient for this prospective standalone run. K6Alloc remains process-global, and two atomic loads are not a transactional or per-site snapshot. The runtime must preserve no concurrent registered-allocator users throughout every baseline-to-drop interval. Cargo/compiler processes have separate address spaces and do not share these counters. Environment thread-count settings or matching counter deltas are not substitutes for the single-process source argument. Unexpected runtime interference still returns no binding.

## Validation and output interpretation

E:24-45 validates only after all three trials finish. All size derivations use checked subtraction/multiplication/addition. The code demands:
- one successful call on first insertion;
- no extra calls/bytes through insertion11;
- two successful calls on insertion12;
- positive leaf and internal requests;
- after-drop current equals baseline and calls remain unchanged;
- total retained delta equals2L+I;
- correct contents and non-test/non-debug-assertions example cfg.

The arithmetic cannot silently accept overflow through two equal None values: positive leaf/internal plus m2=m1 imply m3>m2>m0, so the independent m3-m0 side is Some if those preconditions hold. An overflowed total consequently fails equality.

E:48 is the first println, after all windows, content checks, drops and validity arithmetic. Five exact size_of/align_of pairs were captured at E:14-20 for the original requested targets. Labels and array indices agree. The REQUEST label states alignment=UNBOUND. The final result says COUNTERS_AND_CONTENT_PASS rather than E_max or acceptance.

Interpret any per-row REQUEST as provisional until the complete log, exit status, source/build identity and isolation are reviewed. An earlier REQUEST can appear before a later row prints NO_BINDING; it is not an overall pass. Require all three valid rows, all five correctly bound actual-type outputs, final COUNTERS_AND_CONTENT_PASS and exit0 for the intended composite witness. Printed example cfg checks do not independently attest the dependency compiler/cfg; verbose compiler invocations and pinned toolchain/target/source identity must do that.

The raw before/after byte and call samples plus content booleans are printed, so later reviewers can recompute all request derivations instead of trusting only the labels. No private-node alignment can be inferred from any result.

## Compile profile and release boundary

RUNTIME_COMMAND.txt selects the existing H manifest, this one example, release, offline/locked, aarch64-apple-darwin, toolchain1.97.1, incremental0, -j4 and verbose compiler output. It clears inherited seeded/Rust flag and wrapper overrides named there; no feature/test option or manifest/lock/config change is introduced. The new hook is not under the preceding test-module cfg: that attribute applies to its mod item only.

The planned normal-production source types and fixed empty-tracker path are consistent with this command. I have not checked compiler success or actual executable behavior. A later release must still bind actual full rustc commit, effective FK/H cfg/features/profile, unchanged overlay/allocator/manifests/locks, guard/operator supervision and source isolation at launch. The absence of review-time config/overrides is not permission to skip that evidence.

black_box and inline(never) improve witness observability but do not guarantee allocation survival. The GlobalAlloc/black_box limits from source07_02 remain. Missing, extra or optimized-away requests, failed cleanup/content/cfg, overflow, source/build drift, unavailable tool/dependency or inability to establish isolation means no binding and stop. Do not repair around it by changing allocator/observer, inspecting pointer alignment, adding a monitor, changing type definitions, installing dependencies or running an alternate probe.

## Remaining claim boundary

This concrete source review covers the exact three-file overlay and prepared archive for one prospective run. It reports no runtime facts, does not accept node sizes, and is not product implementation acceptance.

T1/T2 remain unmeasured until successful source/build/runtime review; private BTree node alignment remains UNBOUND even on successful byte measurements. C1 source input capacities, C2 finite descriptors, O2 final-A1 reconciliation and W1 staged-H/prefix/VR global caller-consumer composition remain open. No complete E_max, admission replay, solver qualification or release follows.

Only this additive review subtree was written. No Rust, build, diagnostic/probe, model/solver, network, installation, new tooling, Git/index mutation or delegation occurred. Git reads used GIT_OPTIONAL_LOCKS=0; archive inspection/hash/diff/dependency parsing were in memory. The initial broad textual search matched ordinary words such as vector and produced noisy truncated output; a corrected exact-marker search returned no constructor/worker matches. No conclusion relies on the truncated search.

