# Source, consumer, layout and qualification manifest

This is a bounded candidate-inspection manifest, not a maintained write grant.
P=projects/chirality-piping; FK=P/core/solver/frame_kernel; K=FK/src/structural/
retained; H=P/core/solver/performance_harness; VR=P/validation/benchmarks/
numerical_robustness. Main source49034a940f. The consulted current maintained
FK/H/VR trees are byte-equivalent to that revision by scoped Git diff. Line
anchors are navigation; ORIGINS.json pins the bytes. New C1 reader/producer paths
below are prospective paths from that contract, not existing implementation.

## Numeric/accounting owners

| Source / exact seam | Required consequence | Targeted future verification |
|---|---|---|
| K/wide_sum.rs:77–105,112–124,176–212,218–323,344–445,465–543 | SumWork four increments/merge/LME checked; max_span max-only. Clone, clear and reset retain state. Pending base/carry checks precede raw mutation; status-only donor transfer; net/sign/round cannot hide local failure. | Raw near-MAX component and aggregate boundaries; zero add, carry tail, shift, scaling, two-term product, add_product_of's mutable b, partial refusal/full reset; cloned overflow and exact zero. |
| K/wide/multi.rs:984–1111,1119–1226 | WidthWork charge/merge/count sum/weighted sum; AttemptWork record/merge/three-width sum; WideContext clones. Preserve 2L,L²,(L+1)(64L+2),(L+2)(64L+2),12L,L²+4L. Context operation remains counted before numeric success/error. | All eight operation kinds at L4/8/16; count fit but weighted overflow; component fits but aggregate overflow; failed numeric operation still counted once. |
| K/adaptive.rs:88–111,219–281,1064–1205 | InvocationMeter/StageGuard and all 19 StageWork fields; distinguish budget room clamp from accounting delta; status before budget use; close_stopped cannot mask underflow. | Exact limit, one-over and simultaneous Case/Invocation precedence; actual overshoot preserved; stage>total becomes I; tainted room is not usable. |
| K/adaptive.rs:1234–1445,1713–1888,1909–1911,2066–2300 | Shared and Spent; t0…t5, own spent closures, t3−ctx−sum base, StopDecision; contexts recorded once, raw sum collected once, status-only local owners. | Success and each pre-guard/error/late-tail path; preserve partial stages and ordinary numerical reasons; no new charge for formerly unpriced owners. |
| K/verify.rs:86–287,392–544,734–1228 | VerifyShared/VerifySpent, all t snapshots, compound bound/charge deltas, local prescribed t/r/inf/one/au/d; collect scope status before every `?`, return and local drop. | Own/shared verification failures, support tail after last guard, stopped closure and Uc/S refusal retention; no repeated physical verification charge. |
| K/adaptive.rs:470–580,616–878,1460–1690,2140–2265 | Product/ratio scratch and clone observations; tracker lazy/table pruning and collapse; residual/fallback and decision local accumulators; offered sequence, held capacity arithmetic. | Dropped/pruned row carrying O/I still taints scope; early zero/infinity ratio return cannot hide clone failure; exact seq/held arithmetic. |
| K/adaptive.rs:2756–2890,2909–3035,3085–3091,3311–3380 | CertificateMeter collects all new local costs even on errors; clone_delta subtracts only known ancestor counters with statuses; PublicationSpent retains scope, total and original result. | MAX−MAX clone delta, invalid before/after, same-value foreign snapshot, pre-existing arithmetic error plus accounting loss, repeated round_up scratch, certificate rejection. |
| K/adaptive.rs:2553–2582,3574–3639,3653–3736,3783–3901,4055–4089 | AttemptRecord own/shared/stage/stop/verification totals; CaseBudget; success Arc cache and failure tuples; obtain/obtain_verify built flag. | First actual build vs repeated case reuse; cached non-budget failure; budget failure not cached; own+shared and S+V overflow; false built flag cannot erase status. |
| K/adaptive.rs:3554–3570,3937–4179,4183–4405,4416–4443; K/combine.rs:79–132 | Terminal envelope before Refused-vector loss; meter state independent of vector; finish_selected/evidence/cache clone; pre-schedule and after-work refusals; first-slot combination cache selection. | NegativeEnergy/Structure with records, true pre-schedule empty, later inexact case, exhausted next case, source/prep refusal without fabricated source/run; no refund or retry reset. |
| K/assemble.rs:189–535,539–648,672–845; K/factor.rs:447–815; K/recover.rs:221–475; K/bound.rs:185–209,294–321,365–918,946–955,1011–1267; K/directed.rs:32–170; K/ledger.rs:182–198 | These borrow context/sum owners rather than creating another price model. Guard every changed sum observation, all error/reset edges and terminally propagate accounting failures; retain numeric Span/Exponent block-local handling. Source/count admission protects scalar/index math separately. | Formation/directional, unguarded condition/pivot scan, support tails, shifted blocks, exact ledger add_to, directed step/clear; accounting stop must not become an A2 local refusal. |
| K/source.rs:320–348; K/factor.rs:86–106; K/directed.rs:174–198; K/adaptive.rs:344–365,407–437 | Fresh constant-size unpriced helper owners: no inherited charged total, bounded raw operations; preserve their numeric outputs. If bypassing scope-state transfer, independently prove these local counts and future source correspondence. | Chord s uses 2 raw adds+sign; chord c≤2 add_scaled+sign; determinant sum ≤24 raw adds+sign, contexts 6 and 12 TwoProducts; binary64_up2 adds+sign. Each bound is <2^18 per owner using I29's atom bounds. row_bound/intensified helpers have fresh tiny sums; directed correction scratch is fresh each iteration, while its context has only initial rounds/division. No cumulative loop bound is inferred. |
| K/source.rs:65–76,352–742; K/assemble.rs:539–648; K/factor.rs:223–401; K/adaptive.rs:894–976,1486–1500,1643–1653; K/bound.rs:946–955; K/recover.rs:101–198; K/ledger.rs:213 onward | Count/index/encoding/scalar admission BEFORE deriving native counts. Check raw lengths including integer-term limb length×64 before bit-index formation, support/combination multiplicities and actual widths/features. | Independent checked preflight boundaries; invalid counts must never reach6n, prefix sum, m,64m,r*r, narrowed encoding, allocation/index expression. This row is not discharged by sticky work. |
| FK/src/structural.rs:10–36 | Public re-exports and downstream exhaustive matches; later API choice must keep clean legacy views and full recorded custody. | Existing public source consumers, no duplicate numerical core or new price paths. |

Additional inspected boundary: K/wide.rs:843–940 has K3a's separate WorkCounter/
WideArith (L2), used by ordinary formation_check.rs and its tests, not the W1
L4/8/16 AttemptWork ledger. K/wide/multi.rs:96 explicitly preserves it. Do not
silently include its unpriced operations in W1. It is outside the proposed W1
counter mutation set; shared WideError/API changes, if later chosen, require its
compatibility tests. FK/src/exact_sum.rs:69–77 has the legacy ExactAccumulator's
checked carry access and AccumulatorOverflow; it has no SumWork. Ledger/prescribed
uses remain unmetered with their existing refusal/poison discipline, not newly priced.

## Consumers and layout consequences

| Actual/proposed consumer | Consequence before correspondence is claimed |
|---|---|
| H/src/k6/w1/staged.rs:100–275,292–365 | attempts_of currently hides Refused traces; stage_sum/own_total/charged_by/work_closes/stages_complete and per-precision aggregation currently saturate or plain-add. Equality alone is not exactness. Inspect and gate all these helpers on qualified E or a recorded adapter; keep old exact-output bytes. |
| H/src/bin/k6_observe/w1.rs:104,127 onward; main.rs saved attempt vectors | E output formatting remains identical. Non-E cannot emit existing exact parity/charge claims. Keep original records unchanged; any additional error evidence needs an explicit format contract and bounded memory. Saved/current/prefix vectors may coexist. |
| VR/src/records.rs:20–125; lane.rs:234–246 | Work/stages/charged are serialized without state; Refused has an exhaustive pattern and empty attempts. Preserve E `vk-case-record-v1` bytes and existing outcome text. Fail or use a separately qualified recorded route on non-E; never print a small fallback number as exact. |
| H/src/k6/w1/counts.rs:332–400 | EXACT_WIDE_SUM_BYTES=2144, TRACKER_ENTRY_BYTES=4304 and of_this_build AttemptRecord size are source/build facts, not enduring guarantees. Re-derive raw work layouts, alignment and nested tracker/fallback entries. A current-host size getter alone does not qualify a reference profile. |
| H/src/k6/w1/envelope.rs:20–35,577–608,791–792,896–924,1042–1043,1203–1231 | Source40129Rust1971Aarch64V1 freezes old layout/request assumptions. Revisit 4304 lazy/fallback strides,40 table entries if stop enum changes,1168/200 tree-node facts,720 Shared fixed payload, VERIFY_ARC,1464 call/3328 attempts,1976 finish, and old/new growth overlaps. Do not silently use this profile for modified types. |
| H/src/k6/w1/h_envelope.rs:14–24,415–438; VR/src/envelope.rs:575–591,963–1000,1504 onward | Wrapper profiles transitively depend on canonical kernel facts; H saved attempts/prefix ownership and VR parsed/printed record trees need reconciliation. VR's exact 19-stage/18-member attempt record formula must remain right for unchanged E format or be explicitly revised for any new format. No duplicated memory core. |
| FK Shared/VerifyShared/Spent/VerifySpent/StopDecision/PublicationSpent/AttemptRecord; cache failure Result/Option tuples; GroupCache; RetainedEvidence/RetainedSolve | New flags can change padding, alignment, enum niche/discriminant, Arc allocation payload, stack scope, Vec stride and Clone-derived owned storage. Raw/width/stage fields affect nested parents. Shared Arc copies share payload; failed-slot stages/refusal vectors deep-copy where Clone does so. Refused recorded vectors can survive longer and add real retained/copy peaks. |
| I29 memory O5–O13 and O16; F2a P1–P5 | Recount source/prep/case/group/cache owners, active/reused/failed slots, all returned terminal envelopes, scope status and scratch, physical trace plus logical references, invocation ledger, PP ordinary-base/W1-draft coexistence, hashes/diagnostics and PP/headless/native transfer/completion. A two-bit logical state is not a two-bit allocation. No old size or byte-policy allowance is asserted. |
| C1 prospective PP/retained_receipt.rs, retained_source.rs and routing; Rust result_export/retained_precision.rs, Python analysis_runs/retained_precision.py, TS results/retainedPrecision.ts and registration services | Same exactness prerequisite, safe-range validation and ordinary fallback; statuses remain internal unless a separately reviewed diagnostic contract exposes them. Work-validity checks must precede C1 B/T split, reader arithmetic and async final registration. Hash self-consistency does not establish actual producer provenance. |
| C1 schemas/retained_precision_mp_v2.schema.json and carrier branches; shared reader corpus | Exact successor schema can remain numeric-only because non-E never finalizes. New unknown/overflow/inconsistent diagnostics and recorded terminal API require explicit mapping review, not casual insertion of a field in an existing closed object. |

Potential alignment example, **not a measured or selected layout**: one byte flag
in 40-byte SumWork may round it to48; a flag in 64-byte WidthWork may round it to72;
three widths may grow AttemptWork by24;19×8-byte StageWork may grow 152→160.
ExactWideSum may absorb some change in existing 16-byte padding, or may grow.
Only a frozen concrete layout can establish which. Do not multiply the speculative
8-byte deltas and claim a complete memory bound. Snapshot/terminal choices and
enum changes may dominate. Historical Source40129 profiles/observations retain
their own identity; a changed candidate needs new correspondence and reviewed
profile facts even if selected numerical/observation bytes compare identical.

## Finite implementation-test manifest (not run)

- FK tests/retained_wide_k3/k3_tests.rs: preserved width prices and clean K3a parity;
  add near-boundary state tests without weakening current saturation expectations
  for any retained compatibility view.
- FK tests/retained_k4/wide_sum_tests.rs, directed_tests.rs, ledger_tests.rs:
  pre-mutation raw headroom, partial/full reset, donor/clone status and clean bits.
- FK tests/retained_k4/adaptive_tests.rs, kf1_tracker_tests.rs, kf3_tests.rs,
  publication_tests.rs, combine_tests.rs, bound_tests.rs, factor_tests.rs,
  recover_tests.rs, method_tests.rs: named rows above, exact terminal/cache/case
  accounting, status erasure mutations, stage closure, unchanged numerical vectors.
- FK tests/s11_site_table.rs: arithmetic-site inventory changes are reviewed as
  changed source evidence, not bypassed; no oracle/tolerance/price weakening.
- H tests/k6b_w1.rs, k6b_export.rs, k6c_envelope.rs, k6c_h_envelope.rs;
  VR tests/k6c_envelope.rs and existing maintained H/VR record comparison paths:
  clean output identity and profile/layout consequences on the actual candidate.
- C1 W01–W10/C01–C06 and reader contract fixtures across Rust/Python/TS: add O/I,
  unknown provenance, MAX−MAX, zero-coefficient cache status, missing terminal,
  later inexact invocation, exact unsafe-JSON count, unchanged clean projection.
  Existing shared truth/oracles stay intact. No solver/compiler/host run occurred
  in I34; these are later qualification obligations.
