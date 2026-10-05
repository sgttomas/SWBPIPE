# Owner-to-source ledger

All numeric source line references below are to frozen I37 (RV51 CLEAR reported by ROOT),
fdae294643b798c1849da8b2e643085562593686 unless marked baseline. Files are under
projects/chirality-piping/core/solver/frame_kernel/src/structural/retained/.
Origins/hashes are in _run_records/ORIGINS.json. Roster is revisable, not acceptance.

| Owner / operation | Frozen source | Lifetime / charging instruction |
|---|---|---|
| Shared children + fixed work totals | adaptive.rs:1357,1421; factor.rs:419 | One allocation for each actual successful build; its member/matrix/factor child backings persist through all Arc users. Count partial children during failed construction even though no success Arc appears. |
| Solved / PrecisionState | adaptive.rs:1387,1887 | Case-specific Arc owns u and Recovered children. states Vec, pending, candidate and verifier are reference owners. VerificationThenCandidate is the same allocation. |
| RetainedFactor helper | factor.rs:640,672,705 | scaled/x/out/unscaled outputs may overlap parent rhs/condition vectors. Return moves Vec. No profile-row clone during ordinary solve. |
| Refinement/fallback | adaptive.rs:1765,1887 | Up to four evaluated snapshots, one current residual row buffer, nested helper and up to four retained fallback trackers; outer residual tracker also remains during fallback. |
| Recovery | recover.rs:239,495 | q_all and values move to Recovered; end_actions/spring/directional/reaction locals drop. Option reaction stride is separately bound. K4RST is a new byte allocation. |
| Verification shared | verify.rs:390,422; bound.rs:164,373,481,691–694 | VS Arc children; u_pass a/c is dominated by the existing four same-type F-buffer upper using maximum per-site law; retained c overlaps nl_pass at/bt/ct. Two distinct block-max vectors u/n_l each B remain. No fifth F buffer added. BlockRatios positions is a deep copy before estimates move. |
| Verification pass/report | verify.rs:601,723; adaptive.rs:4171 | Report children are moved; Arc creation adds Arc allocation only, not copied vectors. Remaining pass scratch drops. Summary creates three new vector backings. |
| Shift attempts | bound.rs:1019,1239 | ScaledProfile spans retries; current factor drops before next factor. current and next, in_flight high-water, results/refused, caller start/s_refused overlap. |
| Tracker lazy/table/prune | adaptive.rs:680–889 | lazy may be taken while table grows; table/drain + kept coexist, sort has workspace, kept shrink may move. Fields contain changed sum/error types. |
| Ordinary shared cache slots | adaptive.rs:3831,3839,3906 | Success Arc references; Err stores only inline Stop/WorkTotal/StageWork. Budget failure uncached. Cached error reuse is a new attempt but not another build. |
| Failed verification cache slots | adaptive.rs:3835,4116 | On new nonbudget failure, spent.refusals is original and cache receives a clone. Reuse clones cache vector into attempt. Each backing distinct. |
| Per-attempt refusal transfer | adaptive.rs:4203–4235 | Uc vector moves/clones as source dictates; S spent vector stays during extend; record<=2B, spent<=B. On error/early work-loss these owners drop only at actual scope exit. |
| Cache import | adaptive.rs:4911; combine.rs:163 | First occupied slot independently per precision; failed-vector clone once per selected slot, original operands remain. Success Arc payload counted once by identity, not once per operand. |
| Cache snapshot on selection | adaptive.rs:4564–4575 | clone local/group cache value: new failed-vector children, shared success Arc refs. Header inside selected Box; source header still exists. |
| Run geometry | adaptive.rs:4397,4671; factor.rs:60,132 | Deep clone from group including NotAssessed child vectors. Moved into terminal/evidence. Group still owns its original. |
| Attempts and selected clone | adaptive.rs:2729,4399,4710 | At most4 record backing slots; summaries/refusals separate children. finish clone overlaps original selected record. |
| Rule hats / optional floor | adaptive.rs:2284–2312,2337,2386,2430,2460–2490; verify.rs:794 | New rule hats Vec<[f64;2]>(b) coexists with report resolution, skip, coupled scales, trackers/summaries and optional separate phis/floor. Add V_rule_hats(b), plus actual old backing during moving collect; hats stays live during other growth. Earlier resolution_hats check result already dropped. Hats drops at rule-closure return; floor/summaries move to StopDecision. |
| Native publication/certificate | adaptive.rs:2950,3560 | Canonical layout/rounded values are scratch. Publication rows/scales and private radii become selected owners. Fixed CloneWork is stack. |
| Selected/evidence | adaptive.rs:3640,3680,4671 | Box plus prep/group Arc refs, cache/state children, Evidence children. No Report or StopDecision stored. |
| CoreRun/RunWork | adaptive.rs:4281,4302,4334 | One outcome and four inline totals. Refused attempts survive before legacy projection. No invented RecordedCase source/origins yet. |
| Case legacy adapter | adaptive.rs:4315; 4839–4890 | Refused attempts dropped; Unresolved retained. Existing solve_cases output is Vec<CaseOutcome>. Group tuple table overlaps all cases until return then drops; selected snapshots still reference success payloads. |
| RecordedCombination | combine.rs:79,119,165 | WithRun moves outcome and RunWork; pre-source enum stores before/after totals. Terminal geometry is dropped by mapping, not copied to outcome. |
| Combination legacy clear | combine.rs:91–102 | attempts.clear drops element child allocations but keeps Vec backing capacity. Returned length0 does not imply zero requested bytes. |
| I37 work ownership | work.rs:59,145; wide_sum.rs:222,785 | WorkTotal private fixed fields; WorkStream Cell, borrowed WorkSnapshot; CloneWork owns fixed ExactWideSum. No heap identity registry/box/string. Layout/profile still changes. |
| P1 source/group/CasePrep | P1 corrected packet; adaptive.rs:919,957,4797 | Existing owners enter P3 by identity. Do not reallocate census/clone upstream for free. Actual C2 new origins/API buffers remain nonzero unknown delta. |
| Historical canonical schedule | baseline performance_harness/src/k6/w1/envelope.rs:977 | Phase/lifetime crosscheck only; no automatic use of old width, Arc, tracker, enum, vector, tree or sort byte constants. |

## Constructive controls and limits

_run_records/owner_graph_checks.py constructs independent tiny root/edge graphs
with explicit expected reachable token sets and exact synthetic integer weights.
It checks sharing, equal-valued independent builds, failed-vector generations,
first-occupied import, report moves plus summary clone, partial failure, terminal
erasure versus clear, and old/new resize overlap. It executes no Rust/model/solver,
allocation measurement or production envelope. These controls verify graph
bookkeeping distinctions, not numerical reachability of a failed-vector witness,
actual capacities, byte coefficients, the complete source roster or a theorem.

The additional _run_records/hats_repair_02/checks.py source-anchors an independent
allocation-event control: the earlier check allocation is freed; report resolution,
new hats and optional floor are three simultaneous distinct tokens at stop rule.
It checks hats old/new backing overlap, hats retention during summary growth, and
hats drop with floor/report retained. Original controls/results are unchanged.

Current source alone does not prove a selected result can carry every theoretical
combination of failed slots; including possible copies is conservative. Removing
one requires an independently reviewed reachability invariant. No such invariant
was supplied or assumed.

## Remaining bounded qualification cells

| Cell | Exact remaining deliverable / dependency |
|---|---|
| P3 source roster review | RV52 found missing hats (RV52-1); correction awaits same-reviewer backcheck. Review and original raw evidence remain preserved. |
| I37 final source | ROOT reports RV51 CLEAR on unchanged fdae294. Any later source changes require affected hash/type/lifetime rebinding; no profile qualification follows. |
| C2 source binding | Recorded case/origin arrays, Prepared operand route, selected group choice, pre-source guards and retained ordinary source owners. Bound actual container/index capacities before construction. |
| Layout profile | Final toolchain/target/features and actual strides for changed sum/work/stage/attempt/error/cache/Shared/VS/retained/terminal types; Arc/Box alignment/control allocation. |
| Container/stack profile | Site-specific capacity/growth/clone/shrink/collect laws, BTree node and stable-sort workspace, old/new moving overlaps; fixed numerical call-stack max. |
| Certificate/private facade | I35 integration-02 draft was received through peer message during this task; unreviewed and not used as accepted weights. Add only after ROOT supplies accepted frozen basis and actual private producer/accessor buffers. |
| P4 caller | Captured-caller ordinary inputs/results and headless/post-PP finalization, serialization and export request window. No whole-process guarantee. |
| M1 integration | One upfront reservation computed using upstream counts before allocations; ordinary behavior/standing preserved on W1 decline. No new amount or scheduling policy selected. |

Repair disposition: ROOT commissioned this narrow RV52-1 correction after the
original pause return. Return for same RV52 backcheck; no wider work is authorized
by the correction. Original pause/timing records remain historical and unchanged.

