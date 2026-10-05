# RV67 — late observation seam backcheck

**Disposition: callsite and clone-removal direction are sound; two callback
corrections remain before this exact proposal is clear.** No whole-C0 acceptance,
source grant or permission to resume solver/product runs follows from this review.
The verified numerical method is not reopened.

Same independent TASK Type 2 `/root/rv67_prepared_producer_design`, parent ROOT
HELP_HUMAN `/root`, native continuation, no descendants. Receipt 2026-10-03
10:28:02 UTC. Frozen dispatch:
`d520aba770b79c935d3479d49c32ac99b07ed08f`.
Subject: R/verification/i51_c0_seam_proposal_03, where R is this packet's
RESUME_2026-09-30 ancestor. Source reconstructed from immutable
`8bbc04e8a36643cd8e96ee7118a141e119b55b3f` plus C1_SOURCE.patch SHA-256
`4a906c15ca6c35b0ab6c4e1a8460d7847f219a6489ccd99159fb085bb932cfff`.
All three reconstructed pre-proposal source hashes match C0_SEAM_PROPOSAL.json;
all three proposed patch hashes match the frozen manifest. No changing CODE was
read or edited. Line numbers below refer either to the frozen callback patch or
the reconstructed proposed retained_product.rs recorded in RECONSTRUCTION.json.

## RV67-L1 — blocking: account the transient marker and its failure restoration

Callback patch lines139–141, proposed retained_product.rs:3052–3054, writes
`prepared_late_active=true`, invokes case_source, then writes false. Neither
store enters MapWrite accounting. These are real successful-path writes; they
also execute on a nested capture-error path. The other newly introduced marker
writes are explicitly charged. The selected entered-work/failed-prefix contract
therefore is not met by this proposal.

The current unconditional final assignment does restore false after an ordinary
case_source error return, including a nested accounting fault. Preserve that
property. Simply inserting a fallible `capture_entry(MapWrite)?` before reset is
incorrect: AdapterWork::enter at proposed:2882–2895 returns false without increment
once a fault exists, so an early-return cleanup could leave the latch true.

**Required correction:** preferably split the actual old-to-old/source capture
into a closed inner function invoked by the default early path and the prepared
late path, so no temporary mutable bypass flag or reset is needed. Alternatively
specify an entered, non-fallible restoration mechanism that retains its attempted
cleanup work/fault state and unconditionally restores the flag while preserving
the original error. Do not invent a successful exact count after overflow or
precharge an operation that is later not entered as though it executed.

The new guard helpers also need accounting-aware comparison disposition.
`prepared_case_seen` and `prepared_case_source` call `adapter.same` inside a
negated if condition and return Association before the following `require` when
that comparison itself sets an accounting fault (patch99–106,125–132). On an
identity-equal input with a KeyProbe/ValidationEntry/IdentityByteRead fault, this
manufactures an identity-mismatch primary error. Evaluate and check the producing
comparison status before treating false as a semantic mismatch. Retain both the
original cause and aggregate accounting status according to the selected rule.
This correction concerns the newly introduced helpers, not a broad old-code audit.

## RV67-L2 — blocking: require positive completed-observation custody before capture

Callback patch125–140, proposed retained_product.rs:3038–3053, checks the early
case marker, count/id/scope and supplied source_selected flag, but not the already
available successful `solver_observations` completion. A concrete permitted state
trace is: normalized/invocation capture; early case_source records case_seen;
then invoke prepared_case_source(false, same valid borrowed operands) **before**
solver_observations. Its guard passes and old-to-old capture constructs SourceParts
and the old PrimitiveSource. Missing observations are detected only by finish's
later bind_observations, after this source work has happened.

The proposed actual lib callsite is late enough; no observed normal-path ordering
failure is alleged. The gap is the callback's required earlier/missing/foreign-hook
refusal contract. Its own call count is not proof that the ordinary producer
completed its observations. A final envelope refusal cannot undo source preparation
that was entered before the positive completion condition.

**Required correction:** before setting the permit or entering source capture,
require the successful immutable observation record, observation_calls=1, matching
case and invocation mode/capture custody, and consistent parity presence, using
counted checks and their actual accounting status. These facts already exist in
solver_observations (:600–687) and the ownership portion of bind_observations
(:689–715); a narrow shared custody validator can avoid replay or allocation.
Keep the final envelope's complete metadata/value/presence checks as well. The
new callsite must still pass its actual final selection flag; observation completion
alone is not a replacement exact-selection permit.

## What the frozen source establishes

- The ordinary branch iterates actual model load cases; invocation selection is
  `any(solve.source_selected)` at lib.rs:2563. Normalized capture rejects combinations
  and unsupported profiles. The new early and late guards both require exactly
  one model case, no combinations and matching actual case id. For this scope
  alone, this completed case's false selection flag settles the invocation's
  no-exact-selection condition. It cannot be generalized to multiple cases or
  combinations.
- `source_selected` is set from selected_source.is_some() at original lib.rs:4905,
  before the selected value is taken for source finalization. The proposed hook
  sits after that finalization and immediately before the only final successful
  LoadCaseSolve return. A selected source stays signalled true even if its later
  finalization fails. That true flag refuses before case_source/SourceParts.
- Early case_source at lib.rs:3476 and the late hook receive the same model,
  built/material basis, load_case, restrained/spring slices and immutable local
  load_application/thermal/pressure values from this one function activation.
  No reparsing, copied substitute or repeated ordinary solve is needed. Actual
  solver-observation capture remains earlier at3914 and final envelope checks
  remain in finish/prepare_case.
- Missing early/late markers and duplicate late calls are explicitly refused;
  duplicate count is entered before refusal, and no second source capture follows.
  Existing capture failures persist and block PreparedCase. The additional positive
  observation guard in L2 is needed before the source-work boundary, rather than
  relying solely on the eventual final check.
- The callback patch removes DeferredCaseInputs, deferred storage/capacity fields,
  deferred replay and its full PreviewModel/BuiltModel/material/case/application/
  boundary/load clone graph. It replaces them with bounded state and one case-id
  copy; that id is later copied again by the existing capture and both prefixes
  must be counted. It does not merely rename the broad graph. Existing member,
  support-child-list/string/source-vector owners still belong to incomplete C0.
- With observer=None the additional call is absent. With the default diagnostic
  observer the new method immediately returns because prepared_probe is false;
  its original early capture/native diagnostic remains unchanged. The callback
  does not mutate ordinary results, source-budget state or public diagnostics.

## Required exact controls and fence consequence

Before any solver/product runtime is restored, ROOT should inspect the applied
correction and the allowed local checks. Required seam controls are:

1. Valid one-case hook ordering captures the old source exactly once from actual
   inputs; no deferred broad snapshot survives. Final ordinary envelope bytes and
   default-observer behavior stay unchanged.
2. Missing early hook, missing observations, early late-hook invocation, duplicate
   late hook, wrong case/mode/observation owner, inconsistent parity presence,
   multiple cases, combinations and true source_selected all refuse before any
   disallowed source construction. Record source/preparation/native counts, not
   merely a final error string. A missing late hook remains a final refusal.
3. Inject accounting faults at every new comparison/marker boundary, including
   inside old-to-old capture and at cleanup if a latch remains. Retain entered
   prefixes/capacities, correct accounting cause and original capture failure;
   assert no active bypass latch survives and no second capture/preparation runs.
4. Final envelope identity, solved status, no source-block envelope, complete
   ancillary metadata/value/presence and source custody checks still reject
   mutations after the late hook. Existing exact-block and ordinary compatibility
   checks stay protected; their execution remains subject to ROOT's runtime hold.

The justified additional source fence is only
`projects/chirality-piping/core/product_physics/src/lib.rs` at the proposed
five-line late observer call (original insertion near4955). Retained callback and
its controls remain in the already selected retained_product.rs and
retained_product_tests.rs paths. No other lib arithmetic, solver, maximum,
selection/public routing or output edit is justified by this review. Conditional
S11 inventory changes retain the previous exact evidence/unchanged-first rule.
ROOT decides the scope grant; this packet does not apply any hunk.

The test patch merely changes a diagnostic print and supplies none of the new
negative controls by itself. No compiler, model, native solver or test run was
performed here. The FIRST_NATIVE evidence remains genuine p128/P256 admission
in both modes with its explicit premature-C0 sequencing miss. It does not qualify
these unapplied patches, complete helper/return-temporary/copy accounting or the
rest of C0. Further solver/product runs remain held. The unfinished full C0
schedule was neither demanded as a prerequisite to this narrow review nor accepted.
