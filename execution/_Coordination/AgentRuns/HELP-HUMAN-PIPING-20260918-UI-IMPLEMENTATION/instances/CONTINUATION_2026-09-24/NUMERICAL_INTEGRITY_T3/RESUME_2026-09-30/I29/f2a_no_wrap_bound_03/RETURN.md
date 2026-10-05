# I29 — bounded no-wrap component return

**Partial derivation returned; full upstream no-wrap admission is NOT established.**
RV43-F1's actual proof obligation remains open. The corrected C1/backcheck fixed
the contract wording, not these source bounds. No present overflow, false result,
or defect in an already accepted numerical observation is alleged.

[DERIVATION.md](DERIVATION.md) supplies proposed raw SumWork increment/lifetime
bounds, factor/failure-scan bounds, the unguarded condition screen and post-condition
pivot tracker, both support tails, and explicit scalar guards. It also states the
still-uninstantiated whole-run composition. Fresh independent arithmetic/source
review is required before relying on any component.

## Result and finite usefulness

The typed interface uses actual source/preparation cardinalities and cache origins,
independent of wire/native/section field spelling. It preserves current prices,
precision schedule and work thresholds. Neither 20B/60B,3.75 GiB, returned counter
equality nor one bounded accumulator operation is used to prove a full lifetime.

The exact-integer checker uses only frozen count/storage fields:
- All 33 maintained H counts produce factor+condition+margin local upper sums below
  u64::MAX. Maximum 32,672,385,364 occurs on TREE10000 AX/ROT with F=60,000,
  H=569,964, W=17, Z=1,080,036 and B=1.
- All 193 VR cases with stored attempt counts also fit the local bound. Since those
  records omit F/W/B, the check uses the conservative monotone uppers
  F≤H, E≤H, W≤H, B≤H; it never substitutes F_upper into H−F.
  Maximum 7,358,724,998 occurs among TREE100 storage shapes. Eight cases have no
  stored attempt counts and receive no inference.
- These are arithmetic usefulness checks, not observed work, admission results
  or a new solver experiment. The comparison includes a full failure-scan bound
  beside mutually exclusive success work, intentionally overcounting.
  It does not close the omitted work paths. H/VR's support-free profiles do not
  supply a product support-tail witness.

The canonical inputs, formulas and exact rows are in _run_records/arithmetic.py,
ARITHMETIC.json and ARITHMETIC_ORIGINS.json. The script reads no measured charged
total as proof. Data origins bind the supplied source 49034a940f and count records.

## Coverage and exact remaining paths

FK paths below are core/solver/frame_kernel/src/structural/retained.

| Cell | Source / result | Remaining proof before full admission |
|---|---|---|
| C1 raw cumulative counters | wide_sum.rs:92–103,188–210,291–323,367,443,467,511,542. Atomic component bounds derived, including failures. | Complete lifetime call counts for every owner; clear/reset preserve counters; cloned ancestry and merge multiplicity cannot be discarded. |
| C2 wide costs | wide/multi.rs:984–1111,1153–1226. Finite width prices and saturating count/merge layers identified. | Bound full context-kind counts and all weighted totals, including repeated record/merge histories. |
| C3 factor/negative pair | factor.rs:447–601. Full local factor and unguarded failed-pivot scan bound derived. | Add preceding formation/residual-formation histories; distinguish local increment from whole Shared.total. |
| C4 condition/pivot tracker | factor.rs:695–809; bound.rs:185–214; adaptive.rs:1363–1409,688–791.11 solves and post-check margin bound derived. | Full lifetime incoming contexts/counters. Other tracker families need their own offered-count bound; tracker.offered+=1 at adaptive.rs:697 is not protected merely by a lazy-row storage cap. |
| R1 formation/assembly | assemble.rs:228–535,672–845; adaptive.rs:1296–1361; verify.rs:430–522. | Close member/directional inner-loop constants, per-entry contribution expansion, RHS/prescription multiplicities and both p/q formations; cover unguarded directional tails and stopped builds. |
| R2 solve/refinement/fallback/recovery | adaptive.rs:1449–1888; recover.rs:221–475. | Close residual-row products,≤3 corrections/evaluated-state fallback, repeated candidate comparisons and main recovery before its final guard. Derived support tail requires actual G/C counts and incoming history. |
| R3 verification and bounds | verify.rs:86–283,730–1224; bound.rs:365–559,669–711,753–918,1011–1145. | Close formation-scale repetitions, Uc/u/nl, shifted factors (at most3 rounds), per-block refusals/reset, estimate/charge/bound/W+ and all early returns. The post-last-guard formation-scale support tail is explicitly included as a separate derived component. |
| R4 stop rule/trackers | adaptive.rs:1937–2300,470–878. | Close per-layout/body/group tracker offers and clone histories; account legacy local exact-comparison counters even where their cost is unmetered. No wall-time bound follows from fresh scratch ownership. |
| R5 A1 certificate | adaptive.rs:2756–2890,2942–3379. | Bound all per-row certificate helpers, collected cloned deltas and accounting-aware round_up correction-loop work. It accumulates scratch costs; legacy directed_ratio's fresh-scratch argument is not its proof. A tight correction-step lemma was not completed here. |
| R6 aggregation/stages/schedule | adaptive.rs:1186–1205,1296–1428,1724–1730,1877,2288,3703–3723,3866–3892,4055–4088; verify.rs:522,743–747,1136,1188,1216. | Close ordinary additions, chronological subtractions, stage disjointness/merge multiplicity, physical/logical role transfer, record/case/invocation sums and exactness of every cached failure/success origin.4 solves/3 verification/3 decision-certificate maxima alone are insufficient. |
| R7 count/scalar arithmetic | adaptive.rs:1486–1500,1643–1653; factor.rs:577–584; bound.rs:946–955. | Proposed scalar restrictions cover128(Z+1),128F and ceil_sqrt r*r; derive/check the remaining usize/index/encoding/count expressions before preprocessing. They are obligations, not witnessed overflows. |
| R8 cases/combinations/reuse | adaptive.rs:4342–4442; combine.rs:79–129. | Finite request run count and actual incoming meter/cache bounds; copy full shared cost to each case, charge every actual build once to invocation, no global identity cache, no hidden retries. Budget failures are not cached. Preserve terminal exactness even where Refused currently loses attempts (C1 seam). |

The source-driven composition in DERIVATION §E is sufficient only if every open
bound and transfer node is supplied and checked. Missing functions are not zero.
The maximum unguarded work for the entire solver is therefore still unproved;
a factor-row/screen/support-tail bound cannot substitute for it.

## Smallest concrete alternative for ROOT to assess

- PROPOSAL: If ROOT chooses a maintained change, use explicit checked/sticky counter exactness
  - Evidence: RV43-F1; corrected C1 §2; raw/stage/aggregate sites above; snapshot differences at adaptive.rs:2756–2762.
  - Change: Add a compact monotone status (Exact / Overflowed / Inconsistent) to the native work-evidence path; use checked updates at every raw/plain-add/difference boundary, preserve status through clears, clones, merges, cache slots, Spent/attempt/terminal results and invocation accumulation. Separately keep cheap checked count/scalar predicates for index/multiplier/r*r hazards. F2a may emit exact charge evidence only from Exact provenance.
  - Why: Makes lost-history detection explicit without changing operation prices, the precision schedule or the 20B/60B allocation. It is narrower than pretending an unfinished global call-count theorem exists.
  - Risk: This is not implemented or authorized. A new field/status can affect layouts, source profiles, cache/terminal APIs and readers; the changed candidate needs a bounded manifest, fresh source review and affected qualification. It does not bound unguarded runtime or provide memory/OOM guarantees.
  - Status: PROPOSED

**Existing saturation plus a final MAX sentinel check is insufficient.** Raw SumWork
and StageWork/plain totals can wrap before saturation. Merely replacing raw +=
with saturation is also not an established end-to-end fix: MAX−MAX snapshot or
clone-delta subtraction can become0, and local/terminal histories can be discarded.
Saturation could be used as the numeric fallback only if its exactness loss is
recorded before subtraction and propagated everywhere—effectively sticky state.
Do not call a saturated/clipped/wrapped value exact.

Minimum affected families and consumer seams:
1. SumWork raw term/shift/net/rounded updates, LME total/merge, clear/reset/clone.
2. WidthWork charge/merge/weighted products, AttemptWork record/merge/totals.
3. StageWork::add/add_to/total/close_stopped; plain total expressions, t_i−t_j,
   compound stage additions and sum_work_delta. Underflow is Inconsistent,
   not a fabricated zero or an automatic saturated lower bound.
4. CaseBudget, InvocationMeter, own/shared/verification/stop/certificate totals;
   preserve cached status with origin work and propagate to reused records.
5. Tracker.offered and scalar count arithmetic are separately checked or
   source-bounded; a work-status bit cannot repair already-wrapped m/r*r inputs.
6. Spent/VerifySpent/PublicationSpent, Refused terminal evidence and combination
   returns, then the C1 native→logical projection/receipt. A later inexact case
   cannot silently retain an “exact” invocation charge; use C1's ordinary fallback
   without refunding already spent work or fabricating missing attempts.

No new exception/panic recovery is proposed. A nonoverflowing call must retain
its existing prices, values, classes and evidence. If overflow occurs, a status
must survive to refusal/finalization; a sentinel alone must not disappear in
arithmetic or an omitted record. Exact behavior and tests require ROOT's separate
contract/source grant; this alternative does not itself discharge upfront F1.

MISSING: complete R1–R8 closure and independent review of these local components.
NEEDS_HUMAN_RULING: no new owner question; ROOT chooses further finite derivation
or the separately reviewed counter seam. No implementation is commissioned here.
DEPENDENCY_NOTES: C1 remains implementation-blocked on actual no-wrap provenance.
Full aggregate memory, wire/native/geometry completion and engineering use remain
outside this packet. Original accepted observations and protected criteria stand.

Receipt 2026-10-02 20:21:14 UTC; new-analysis cutoff 20:56:14; deadline 21:06:14.
Direct native TASK /root/i29_w1_limits under ROOT /root; no delegation.
Only R/I29/f2a_no_wrap_bound_03 was written. Origins/session and informational
inventory are supplied; no acceptance seal, source change or runtime is claimed.
