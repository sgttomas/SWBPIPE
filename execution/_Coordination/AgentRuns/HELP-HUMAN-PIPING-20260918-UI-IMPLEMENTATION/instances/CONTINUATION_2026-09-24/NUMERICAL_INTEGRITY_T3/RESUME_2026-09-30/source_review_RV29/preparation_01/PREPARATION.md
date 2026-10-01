# RV29 independent LME preparation — reference paths frozen

**PREPARATORY ONLY.** This packet does not review or accept an implementation,
report a Rust/test/mutant pass, or close PC40–43/PM16. It fixes independent
primitive prices, operand ranges and two original conversion-path ledgers before
RV29 has read new helper counters. ROOT must provide the complete frozen candidate
and separate execution grant for the final review. No new implementation bytes
or author tests were read as numerical/work oracles.

TASK `/root/rv29_a1_source` reports directly to `/root` through native delegated
harness execution. RV29 is separate from I22, design and RV28; no delegation was
performed. This is fresh source-review preparation, not a model-diversity claim.

Aliases: P = projects/chirality-piping; FK = P/core/solver/frame_kernel;
R = P/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/
instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30.
The canonical packet is `<A1_WT>/<R>/source_review_RV29/preparation_01`.
`BASIS.json` records source-qualified origins and SHA256 hashes. The original
primitives at d01ad98a754698631f927709d08284c272de85e8 match the four inspected
files at 3bddc2b05f6106e969c7cf43373b230845c7cc66. The active review skill is
`<APP_WORKTREE>/.agents/skills/software-code-review/SKILL.md`, where APP_WORKTREE
is the Codex task workspace, not the product A1 checkout. No broad governance,
unselected workflow, unrelated role or old response packet was activated.

## Primitive prices and storage ranges

Accepted sources: retained/wide_sum.rs:74–103, 228–326, 333–371, 424–477,
508–548; retained/wide/multi.rs:986–1000, 1153–1228; retained/adaptive.rs:231–252,
379–407, 437–488. Line numbers refer to the immutable original source.

- SumWork LME is term + shift + net + rounded, saturating at aggregation.
  max_span is aggregated by maximum and has no additive LME price.
- add_raw strips low zero bits, sets `k=ceil(significant_bits/64)`, and bills
  k+1 term limbs plus any later carry propagation. The extra iteration is real
  even when its part is zero. Used is a padded occupied prefix, not the count
  of nonzero limbs and not the mathematical minimum representation.
- Lowering an existing anchor by d costs `2*min(old_used+floor(d/64)+1,128)`
  shift limbs, and sets used to that new prefix. The factor two covers both
  signs, including the all-zero magnitude. A zero raw term adds nothing.
- add_wide trims before add_raw and has no separate fixed-width multiply cost.
  add_wide_scaled bills M source limbs before add_raw, even if add_raw refuses.
- add_scaled of a nonempty source bills `source.used` for EACH sign before
  each add_raw. The zero sign still pays its scaling-loop cost. Factor zero
  or empty source returns without this cost. A mathematically zero but
  nonempty source is not equivalent to an empty source for work accounting.
- signum costs used net limbs. make_absolute calls signum. net costs 2*used.
  round of a nonempty sum nets, then converts the anchor to i64, then bills
  used rounded limbs and calls context.from_integer. A failed anchor cast
  occurs before rounded/context charges; a Wide exponent refusal occurs after
  them. Empty round still calls the context integer constructor, but has no
  accumulator net/rounded cost.
- Clone itself has no new LME price in the accepted scheme and copies counters.
  Existing counters are inherited evidence, not new work. Track delta of each
  clone plus fresh scratch separately. clear/reset keep counters; using them
  is not a free counter reset. This packet proposes no new price for copying,
  scanning unpriced metadata or binary64 value operations.
- At width L, Add/Sub/Round cost 2L, Mul L², Div (L+1)(64L+2), Sqrt
  (L+2)(64L+2), TwoSum 12L, TwoProduct L²+4L. Contexts charge before operations,
  including refused operations. RU conversion's two Round@L16 and one Div@L16
  cost 32+32+17,442 = **17,506**. Binary64 conversion, next_up/down, exact Wide
  widening, and Wide value methods are unpriced by the accepted scheme.

## Fixed reference path A: H=1+h, b=2

h=2^-1074. Reference formation is add_binary64(1), add_wide(0,true),
make_absolute, add_wide(h,false). It yields H work (term,shift,net,rounded)
**(4,38,2,0)**, used=19, anchor=-1074, max_span=1075. H's actual value needs
17 limbs, but its padded used range is [0,19). The denominator 1 has work
(2,0,0,0), used=2, anchor=0. Formation plus denominator costs **46**.

The reference is the original directed_ratio Up algorithm, not an invented
new helper order. RN1024(1+h)=1; division by 1 is 1; the initial binary64 value
is 1. Its correction compares c=1 (false), c=1+2^-52 (true), then the
predecessor 1 again (false). Hence RU64(H)=1+2^-52 and H<2. Conversion totals:

| New contribution | term | shift | net | rounded | LME |
|---|---:|---:|---:|---:|---:|
| num clone: is_zero + round | 0 | 0 | 57 | 19 | 76 |
| den clone: round | 0 | 0 | 4 | 2 | 6 |
| fresh reaches(1), first | 62 | 38 | 19 | 0 | 119 |
| fresh reaches(1+2^-52) | 62 | 36 | 18 | 0 | 116 |
| fresh reaches(1), predecessor | 62 | 38 | 19 | 0 | 119 |
| context: two Round, one Div | — | — | — | — | 17,506 |
| **conversion** | **186** | **112** | **117** | **21** | **17,942** |

Reference construction plus conversion is **17,988**, excluding the final H≤b
comparison and any new implementation's validation calls. These are deliberately
not claimed as a candidate total. `LEDGER.json` names every call, exact exponent
or significant range, padded used range, local delta and cumulative prefix.

The num clone inherits 44 LME and adds 76; the den clone inherits 2 and adds 6.
Their inherited 46 belong to construction, which the owner counts once.
Fresh product_reaches scratch costs 354. For this exact reference path, omitting
all new conversion-local sum work understates conversion by **436**; merging
full num/den counters instead of deltas overstates it by **46**. These are
analytic discriminators, not executed or killed PM16 mutants.

## Fixed reference path B: original row_bound(h,h)=3h

Original binary64 operations give absolute_bound(h)=h and the row-rounding
term h. The three add_binary64(h) calls cost six term limbs, retain used=2 and
anchor=-1074, and sum to 3h. `high` tracks the largest *term*, so this num's
recorded max_span is 1 even though its accumulated value has two significant
bits. It must not be silently replaced by a mathematical net span. Denominator
1 costs two. Exact 1024-bit round/divide gives 3h; reaches(3h) is true and
reaches(2h) is false, so no upward increment is needed.

| New contribution | term | shift | net | rounded | LME |
|---|---:|---:|---:|---:|---:|
| three h terms + denominator | 8 | 0 | 0 | 0 | 8 |
| num clone: is_zero + round | 0 | 0 | 6 | 2 | 8 |
| den clone: round | 0 | 0 | 4 | 2 | 6 |
| fresh reaches(3h) | 12 | 0 | 2 | 0 | 14 |
| fresh reaches(2h) | 12 | 6 | 3 | 0 | 21 |
| context: two Round, one Div | — | — | — | — | 17,506 |
| **complete original bound** | **32** | **6** | **15** | **4** | **17,563** |

Conversion alone is **17,555**. The num/den clones inherit 6/2 LME; their new
local work is 8/6. Fresh scratch costs 35. Omitting all new conversion-local
sum work misses **49**; duplicating num/den inheritance adds **8**. max_span
across this complete path is 2, by maximum. The new candidate must own this
formation/conversion once; simply calling original unaccounted row_bound and
then a second accounted reconstruction does not make both executions one call.
A harmless recomputation still has actual work and must be identified.

## Terminal and budget discrimination

The finite terminal table uses existing primitives with invented exact inputs:

| Path | Work before/refusal | Expected primitive result |
|---|---:|---|
| add_integer(1,0); add_integer(1,-8128) | 2 | Span: proposed span 8129>8128 refuses before shift/raw-add |
| add_integer(1,0); add_wide_scaled(Wide4(2^-8128),1,0) | 6 | Span: four scaling limbs precede the same raw refusal |
| add_integer(1,2^62+1); round(ctx16) | 40 | Exponent: raw2 + net4 + rounded2 + charged Round32 |

No refusal may erase completed work. No continuation on Span/Exponent is
introduced. Those original AttemptStops are terminal. The fixed H and bound
fixtures cannot themselves naturally trigger these terminal cases; the separate
algebra controls isolate existing semantics without enlarging numerical scope.

StageGuard rejects only when used>room, with Case checked before Invocation.
For each ACTUAL frozen checkpoint at cumulative local amount C and other-work
base B, set that scope's external room to B+C−1; retain the other scope's room
large enough to isolate the intended stop. Verify equality B+C passes that
checkpoint. The table enumerates prefix amounts, not new checkpoints. Select
one actual conversion/comparison checkpoint and the final comparison boundary.
If earlier checkpoints share the same C, the earliest check wins; a zero-cost
checkpoint cannot be distinguished by the same budget. A stop can include the
full just-completed operation, so do not cap recorded work at the budget.

For reference A, the cumulative local totals including its 46-LME setup are
192 after both clone rounds, 17,634 after division, then 17,753, 17,869 and
17,988 after the three reaches calls. Corresponding one-below limits are
191, 17,633, 17,752, 17,868 and 17,987, plus B. For original small-A1 bound
formation, those amounts are 86 after clone rounds, 17,528 after division,
17,542 after reaches(3h), and 17,563 after reaches(2h). These budget choices
remain conditional on an actual frozen checkpoint there.

Successful, numeric-rejected and stopped paths must each match the final frozen
absolute ledger, then close candidate stage/case/invocation accounting. A lower
numerical bound can make H>b, but its exact bits/formation can change work;
choose that finite rejection fixture and freeze its own path before execution.
Stage equality or a budget derived from observed counters cannot replace this.

## Dependencies on frozen source — explicit remaining work

1. Exact order used to build H, including sign/nonnegative validation and
   make_absolute/is_zero calls; whether report terms are added directly or
   through scaled/scratch accumulators. H=1+h alone does not fix used or work.
2. Whether the new RU helper preserves the original initial test/correction
   sequence; placement of clone snapshots, all local work-delta merges, and
   merges on every early return. No observed counter may choose these values.
3. Final H≤b comparison construction: cloning H versus adding its value to a
   fresh accumulator changes cost, even though truth is unchanged. Predicate
   order relative to RU64 affects rejected paths. The explicit b=2 comparison
   remains pending; no unconditional final-comparison total is fabricated.
4. Where provisional classify/row_bound is executed and accounted; account
   exact small-A1 formation/conversion once per actual execution, including
   whether it is computed again in finalization.
5. Exact StageGuard bases and check locations, whether a terminal primitive
   result is captured/charged before return, and how an already completed
   arithmetic refusal interacts with a later budget check. Preserve accepted
   terminal precedence rather than invent a new order.
6. Full final H≤b and finite numeric-rejected tables, boundary budgets, source
   candidate identity/diff, ordinary outcomes, policy/readers, all row families,
   storage/radii/lifetimes, independent source truth and required mutants.

These are review dependencies, not confirmed defects in an unread implementation.
No new price, tolerance, accounting framework, solver experiment, protected-test
change, implementation repair, Git integration, release or acceptance is made.
