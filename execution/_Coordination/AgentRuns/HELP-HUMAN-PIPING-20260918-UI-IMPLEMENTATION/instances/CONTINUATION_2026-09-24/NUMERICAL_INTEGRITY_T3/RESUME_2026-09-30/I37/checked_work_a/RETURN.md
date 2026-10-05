# I37 checked-work implementation — checkpoint A return

Coherent maintained-code candidate for ROOT's source read and fresh independent
implementation review. This is a bounded prerequisite checkpoint, not complete
F2a/C2, a qualified memory profile, an accepted receipt, or product activation.

Native TASK `/root/i37_f2a_checked_work_code`, parent `/root` (ROOT HELP_HUMAN),
started 2026-10-02 22:36:01 UTC. The substantive checkpoint was returned at
22:56 UTC. Final source was checked before this early return; actual completion
and process state are in `_run_records/FINAL_STATE.json`. No delegation or
Git/index/API write occurred. HEAD remains
`14ff4e86dd549aa6c22804de5913e25c8ab4d20a`; the maintained-source basis was
`49034a940f3f8cd3f3da4d4cbc839943b808063d`.

## Exact candidate

38 maintained Rust paths changed. Every exact path and SHA-256 is in
`_run_records/FINAL_CANDIDATE_B_HASHES.json`, whose SHA-256 is
`9cb31e29817aa8f94ed32594e853ac7c5cac0fc6e1ea86b90fadf0821dd000aa`.
The informational `WRITE_INVENTORY.json` also covers the return and raw evidence.
New work tests register in the granted `retained/work.rs`; no separate registration
path was needed. The only extra maintained path is `tests/retained_k4/models.rs`,
authorized by ROOT at `62e93690941` solely for fallible-call success assertions.
Its models, comparisons, slack and expected values are unchanged.

Origins and input hashes are in `ORIGINS.json` and `CONTINUATION_ORIGINS.json`
under `_run_records`. ROOT's priority clarification at `247c3392ad9` and raw
CountRange clarification at `b2ba2e7199b` are included. No workflow/skill body or
additional role was selected. The work ran directly from the bounded TASK brief.

## Implemented behavior

- `WorkFault`, private-bit `WorkStatus`, private-field `WorkTotal`, and borrowed
  non-ZST `WorkStream`/`WorkSnapshot` preserve exact MAX, checked arithmetic,
  sticky fault union, foreign-owner refusal and checked deltas. Diagnostic
  amounts are unavailable on loss; legacy getters return MAX then.
- Width/context and raw-sum accounting checks prospective charges before the
  affected arithmetic or mutation. Raw insertion reserves the actual pending
  base term across shifts/carries. Index/containment failures cannot write out
  of bounds or discard a nonzero top limb. Compound insertion failures poison
  partial numeric values; full reset clears value poison while retaining work
  loss. Sign/zero/absolute/net observations are fallible.
- `CloneWork` owns one persistent inherited clone and computes its own operation
  deltas. A caller cannot inject before/after snapshots or replace its owner.
  The certificate collects delta work on both success and refusal.
- Stage totals, build/solve/verification/stop/publication totals, guards, cached
  totals, case work and invocation work are checked. Same-cohort stage snapshots
  include initially zero secondary contexts. A2 cannot reset accounting loss
  into a block refusal; tracker collapse propagates it before pruning.
- The charge bundle accumulates actual invocation events, including status on
  zero-price reuse, before returning a failure. An earlier numerical stop stays
  in the physical failed attempt and in `UnresolvedReason::WorkAccounting.prior`.
  Accounting loss terminates before escalation or selection. A bad invocation
  meter prevents subsequent source preparation in `solve_cases`.
- Private `CoreRun` retains `ExecutionOutcome` plus `RunWork` (case, invocation
  before, actual increment, after). Refused attempts survive there until the
  explicit legacy adapter. `RecordedCombination` distinguishes pre-source
  refusal from a real core run and exposes its typed custody. Its early branches
  cannot mutate the exclusively borrowed meter, so their repeated snapshot is
  truthful. No source/group/build origin inventory was fabricated.
- H uses checked meter/attempt/stage values, fallible segment/prefix aggregation
  and fallible writers. An explicit accounting terminal is also rejected when
  an external caller supplies an exact empty compatibility view. Accounting
  failure reaches the existing failing-run exit before saved attempts or later
  observations. VR refuses the case record, records a harness failure and keeps
  `None`; its examples stop before printing/writing a missing record.
- Clean numerical prices and fields are unchanged. Manual Debug implementations
  preserve existing exact-field order and omit the new state then. The focused
  old price/oracle controls and one committed VR record's pretty JSON bytes
  pass. This is not a full historical observation-program parity claim.

## Scalar/index guards and boundaries

`PrimitiveSource` checks raw list and UTF-8/child lengths, 6n, source encoding
length, Q/u32 state encoding, sparse/prefix products and relevant layouts before
its affected native construction. Validated free-count guards precede profile
construction and imply the free-block sentinel bound: at a new assignment,
`positions.len() <= F-1 < u32::MAX`. Combination preparation checks operand
count, every actual identity's u32 prefix, combined length, load-count sum and
prescription product before those constructions. Residual/factor multipliers,
tracker sequence/capacity arithmetic and ceil_sqrt's square comparisons are
checked without physical or timing cutoffs.

ROOT's final source-read question identified the arbitrary raw integer slice
entry. The common local `integer_magnitude_bits` guard now checks length*64
before `WideContext::from_integer` charges and before raw rounding/accumulation
indexes bit coordinates. It returns a distinct count refusal, not Span,
Exponent or accounting loss. `_run_records/RAW_MAGNITUDE_CALLERS.txt` and its
hashed source origins retain the caller proof: production calls are ledger
projection (its canonical magnitude derives from `ExactAccumulator`'s fixed
68-limb array) and ExactWideSum rounding (at most 128 limbs, or the one-limb zero).
The raw boundary is guarded independently of those current caller limits.

New truthful count channels are `SourceError::CountRange`,
`WideError::CountRange`, `SumRefusal::CountRange`, `AttemptStop::CountRange`,
`UnresolvedReason::CountRange`, and the combination preparation/outcome count
refusal. Later C2 defensive reason maps must represent these separately from
numerical and work failures; WorkAccounting mappings must retain fault and prior.
No new product diagnostic policy or receipt schema was selected here.

The generic native source accepts arbitrary explicit stations; it does not build
3m stations. Future C2 product adapters still must check 3m and 3*member+j before
constructing their parts. The legacy public `Dof::from_global` remains infallible;
its maintained internal source-derived calls are bounded by this constructor,
but arbitrary upstream caller inputs are not qualified by A. The same applies
to caller-owned wire/map ordinals and the future captured-request census.
No complete pre-execution memory/admission permit is implemented or claimed.

## Final-source verification

All commands use absolute manifests, one Cargo lane, `-j 4`,
`RUST_TEST_THREADS=2`, and separate per-manifest targets under WT. Every invocation
has a 1,200-second wall limit. No timeout occurred. `COMMAND_INDEX.json` records
all exact commands, exits, environment and targets; complete raw outputs remain
beside their command records. Final B records reference the preserved B hash file;
intermediate candidate hash files and failures remain distinct.

| Final B record | Result |
|---|---|
| finalb01_kernel_debug | 17 checked-work controls pass, debug overflow checking |
| finalb02_kernel_release | The same 17 controls pass, optimized |
| finalb03_golden_prices | Unchanged golden work ledger test passes |
| finalb04_certificate_prices | pc40/41/42/44: four price/terminal/replay controls pass |
| finalb05_raw_oracle | Existing targeted exact-sum oracle vectors pass |
| finalb06_inventory | Three S11 inventory/scanner tests pass |
| finalb07_h_public | Three small public-boundary/checked-consumer tests pass |
| finalb08_h_writer | Explicit accounting terminal cannot emit/digest an exact empty view |
| finalb09_h_partial | Existing partial-build stage/charge closure test passes |
| finalb10_vr_bytes | One committed clean record preserves bytes; unavailable work is refused |
| finalb11_vr_examples | VR lib and both affected example consumers compile |
| finalb12_shared_error_compatibility | Read-only formation shared-error fail-closed test passes |

Earlier compile/check failures are preserved: check01 (five implementation
compile errors), check03_tests (fallible helper/signature adaptations, including
the initially ungranted models helper), and test06 (one test pattern still using
the legacy outcome type). They were repaired in the authorized source/test paths;
no numerical expected value, tolerance, reference or fixture was changed.

The S11 table changes classify the actual replacement of integer raw increments
with checked composition and add the new work module to scanned sources. They
are a source-site inventory update, not a weakened numerical assertion.
The old caller-supplied delta test was migrated to exercise real persistent
CloneWork operations because arbitrary before/after injection was removed.

## Remaining integration and qualification

No known accounting-loss escape remains in the converted instrumented schedule
or H/VR writer paths after this bounded audit. That is not independent review
coverage. Full C2 case/source/group/build origin inventories, successor producer
and readers, safe-JSON projection, G5 enforcement, product adapters and caller
admission remain incomplete. No geometric evaluator or product W1 activation
was added.

No memory profile is qualified. The added flags/totals and error payloads affect
SumWork, WidthWork, AttemptWork, StageWork, ExactWideSum, AttemptRecord,
AttemptStop, UnresolvedReason, Shared/VerifyShared, evaluated/cache enum payloads,
RetainedSolve, CoreRun/RunWork, RecordedCombination, H W1Solve and VR CaseRun.
New layout/alignments, Arc/cache/vector ownership and lifetimes, snapshot/clone
stack costs, terminal buffers and H/VR formatter/Value composition need measured
candidate/toolchain/target facts and independent profile review. Historical K6c/H/
VR profiles and records were untouched; their layout tests were not run or waived.
No byte allowance, memory bound or host-size qualification follows from compilation.

ROOT must freeze/read the actual diff, commission fresh implementation review,
resolve its findings and complete later integration/profile and required clean
candidate gates. Full DEC-025/T9/both-entry, large/timing/model-scale runs, native
GUI, vector regeneration and the full observation programme were outside A and
were not run. No acceptance, merge or release claim is made.

All owned build/test processes have exited. The existing memguard PID 5387 remains
running. No host guard/script, Cargo/lockfile, instruction, other project or
historical profile/fixture was changed.
