# Finite tests and mutant acceptance — plan, all UNRUN

The oracle is fixed. Existing expected availability, R7/A1/A2 test predicates,
precision ceiling, source fixtures and reference values are not rewritten.
New controls below require independent exact expectations before they become
acceptance evidence. This file authorizes no run or source patch.

## Targeted tests in new publication_tests.rs

| IDs | Finite checks | Acceptance |
|---|---|---|
| PC-01..03 | Full E formulas at P256/P512/P1024, one fixture per row family and every term individually nonzero | Exact H matches independent dyadic algebra; P is report precision once. |
| PC-04 | H uses final x, not retained c: x=h, c=v=5h/4, E=0 | Exact H=h/4, not zero. |
| PC-05..08 | Absolute H just below/equal/above b; H>0 with b=0; H=0/b=0; positive-H/x=0 | Only H<=b passes, never observed-zero exemption. |
| PC-09..12 | Public decimal boundary plus A_exact<A_f64 and A_f64<A_exact controls; preserve source operation order | Both sharper and public comparisons enforced; exact boundary equality passes. Independent RV28/addendum rounding discriminators copied as fixed inputs with hashes, not imported from implementation. |
| PC-13..16 | Below/at/above 2^-988; threshold tie/adjacent values; changed max winner; raw subnormal four-direction coupling | Final publisher class/bound consumed, no stale or sequentially coupled scale. Synthetic gate controls are not solver source witnesses. |
| PC-17..20 | O9 Underflow/Overflow; input-derived dominant row; exact combined prescription; prescription-containing magnitude | Correct membership and exact one-round input publication; magnitude remains independently certified. |
| PC-21..24 | p512 Force/Moment floors zero/subnormal/dominant/non-dominant; no T/R or lower-p floor | Identical existing floor bits; H comparison never bypassed. |
| PC-25..30 | Missing/negative fields, precision pair, length/row/body/source mismatch, nonfinite allowance and radius/class tags | Typed terminal certificate error, never zero or success; no panic/zip truncation. |
| PC-31..35 | RU64 zero, H<h, normal upward rounding, max finite, overflow; RU64(H) above a non-dyadic relative threshold | Present +0 distinct from absent; no positive H stored zero; acceptance on exact H, not rounded radius. |
| PC-36..39 | Draft move/drop after first/late-row rejection and conversion stop, RetainedSolve Clone, mismatched radius lookup, fresh combination certificate | Publication/radius pairing preserved; no stale/partial box or old report borrow. |
| PC-40..43 | Budget at exact charged cost and one unit below, Case/Invocation precedence, Span/Exponent stop, directed-conversion partial work | Stage sum=own work, attempt charges=meter, stopped work retained; no double-billed inherited clone counters. |
| PC-44..46 | Numeric H rejection reuses verification, p512 ceiling, malformed report terminal | Stable first row/predicate; no Accepted/Verified on failure; no precision >1024. |

A shared production arithmetic helper, not duplicated implementation, supplies H;
expected values come from separately stated exact equations. Tests exercise
public schedule behavior as well as private algebra. Old decide tests continue
checking old gate order even where the new gate would mask an old mutant.

## Fixed source controls and independent truth

First real repaired-source discriminator is the unchanged C17 primitive and
frozen truth. It must reject the known p128 false publication. A later selected
result must pass the frozen strict oracle AND bare-b absolute checks, or a named
refusal/unresolved must be returned subject to protected availability rules.
Do not assert an unobserved later precision or H value.

Replay exactly B01–B16 once per ROOT-granted candidate stage. Preserve their
frozen truth and report newly honest refusals/precision changes explicitly.
C18–C24 remain unrun unless ROOT separately grants them. Source provenance for a
repaired probe must name the new candidate; never keep G0's audit SOURCE_COMMIT
while quietly linking repaired code. Use additive probe copies, preserve G0.

Three concrete additional source proposals, no parameter sweep:
- F→M: two-node x-axis member, L=2^100, E=G=Iy=Iz=1, A=L, J=1;
  root fixed, tip Ux free, other tip DOFs fixed; three distinct global Ux
  springs at tip, each k=1; separate tip force 5h. EA/L=1, total scalar
  stiffness 4, u=5h/4. Each spring action and member axial action has magnitude
  5h/4; these are the only nonzero force rows and their rounded maxima are h.
  This supplies a nonexact raw force maximum before L multiplication.
- M→F: same axis, L=2^-100, E=G=A=Iy=Iz=1, J=L; root fixed, tip Rx free,
  three distinct global Rx springs k=1; tip torque 5h. GJ/L=1, total scalar
  stiffness 4, rotation=5h/4; nonzero moment rows each have magnitude 5h/4,
  rounded maximum h before division by L.
- Free-zero rotation: L=2^100, E=G=Iy=Iz=1, A=J=4L; root fixed,
  tip Ux and Rx free, others fixed, tip force 5h and no torque/springs.
  Exact u=5h/4 and Rx=0; rounded translation h gives zero inverse-coupled
  rotation scale. This repairs B10's missing free-rotation-row coverage.
  Do not assume generic W_plus is zero or predict selection.

All parameters are finite binary64; derived primitives are normal. These source
equations are proposed oracle briefs, not independently checked results.
ROOT's oracle owner must verify complete row truth/sign/range and freeze
primitive bits before execution. Named refusal is not a witness of false claim
or a general exclusion proof. Reuse existing all-row station, directional/global
spring, support, reaction and prescribed-magnitude tests as additional fixed
regression fixtures; no new engineering library or large frame.

## Required concrete mutation targets

Execute only on granted disposable source copies after the real implementation
is frozen. Preserve each exact unified patch, mutant source/hash, command,
unmutated control, failing test/raw output and restoration verification.
No permanent seeded.rs extension is required for these local mutants.

| Mutant | Exact intended code mutation | Discriminator |
|---|---|---|
| PM-01 | Replace new certificate verdict by accept after old R7 passes | C17 reproduces false p128 and fails fixed oracle/bare-b check. |
| PM-02 | Substitute retained candidate c for final published x in H difference | PC-04 requires h/4. |
| PM-03 | Replace complete E by epsilon*S_pub | Positive E, x=v=0,S=0 certificate must refuse. |
| PM-04 | Omit W_plus addition | Present positive W_plus discriminator. |
| PM-05 | Omit magnitude 2^(1-P)|v| | W_plus=0, nonzero v magnitude discriminator. |
| PM-06 | Omit 69*2^-P*E_q | Force fixture with E_q>0, other E terms zero. |
| PM-07 | Omit W addition | Force fixture W>0. |
| PM-08 | Omit W*2^-P addition | Set exact acceptance boundary between W and W*(1+2^-P). |
| PM-09 | Omit C addition | Force fixture C>0, other terms zero. |
| PM-10 | Accept x=0 or b=0 without positive-H test | PC-05..08; loaded free-zero row. |
| PM-11 | Use qualified b*(1+delta) instead of bare b | H strictly between b and qualified b. |
| PM-12 | Enforce only public or only larger sharper allowance | Both opposite A_exact/A_f64 rounding discriminators and decimal boundary. |
| PM-13 | Round H down before comparison or RU storage | H barely above b / positive H<h control. |
| PM-14 | Use nearest radius conversion or let sentinel/negative-zero become zero | PC-31..39 tag/range checks. |
| PM-15 | Mutate accepted publication bits/class/floor/radius after certificate | Paired-draft integrity and final equality controls. |
| PM-16 | Omit new local accumulator delta / merge whole cloned counters | PC-40..43 exact accounting closure/short-budget checks. |
| PM-17 | Ignore candidate O9 or include input-derived maxima | Existing O9/input-derived tests plus PC-17..20. |
| PM-18 | Apply floor at wrong precision/kind or couple from updated maxima | PC-13..16/21..24 and existing floor/class tests. |

Each is a named unrun obligation until a real executed test kills the concrete
patch. Missing discrimination is an open test gap, not a killed mutant.
Facade-only conversion-error/G5a-auto-pass mutants are deferred with the facade
write set; no generic interval or unit engine is added here.

Run existing 15 registered V-K seeded faults and protected R7/A1/A2 mutants
under their existing harnesses when granted. NONE must pass; each required
mutant must be killed by an actual relevant predicate/evidence failure; unknown
fault IDs must fail closed. Snapshot drift alone is insufficient to establish
the numerical obligation when the new H gate masks an old defect. Preserve
direct old-R7 decision tests and criterion-specific discriminators.

## Finite command/resource proposal and gates

No commands below have been run. ROOT must grant each host slot using the
existing guard/supervision, toolchain 1.97.1, offline/locked, -j4, incremental=0,
RUST_TEST_THREADS=2, per-slice target and ROOT-bound VENV.

1. cargo test --offline --locked -j 4 --manifest-path <FK>/Cargo.toml publication_tests
2. Existing FK retained_k4 suites (including method_tests/classification/
   adaptive/verify/kf3/combination tests), without changing goldens.
3. One newly pinned C17 source probe and fixed B tranche plus accepted new
   three-source controls; frozen comparator followed by exact bare-b comparison
   of its independently computed error. Keep raw/non-source predicate distinctions.
4. H k6b_export, k6b_w1 and prefix/record parity tests, then VR existing tests
   and read-only vk_records output. Exact available test filters are validated
   against the frozen implementation before launch, not silently widened.
5. Granted per-mutant jobs with exact patches; existing run_seeded_faults.py/
   feature guard. Inherit GIT_OPTIONAL_LOCKS=0 for nested read-only Git helpers;
   no helper edits. No hidden Rust build inside a Python harness without a slot.
6. ROOT disposition of observation deltas, independent full-diff review and
   exact-candidate required CI/DEC-025/GEN-8/T9/both-entry/practitioner gates.
   Final K6c accounting/admission replay and W1-T4 stay separately owned.

No large dense experiment, timeout/cap increase, precision extension or automatic
retry is included. A failing protected availability/standing criterion, surviving
required mutant, false claim or guard/supervision gap stops its acceptance path.

