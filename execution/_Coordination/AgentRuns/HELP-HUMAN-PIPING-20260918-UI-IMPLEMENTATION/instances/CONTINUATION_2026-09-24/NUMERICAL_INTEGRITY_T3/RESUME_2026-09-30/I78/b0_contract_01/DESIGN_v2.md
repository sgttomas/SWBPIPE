# I78: B0, F2a breadth: contract and identities (design, revision 01)

TASK (Type 2), I78, for ROOT (HELP_HUMAN, Agent 0), who is the return path. 2026-10-06 UTC.

**This revision replaces `DESIGN.md` (sha256 `25a07a66…`, kept unchanged beside it) and stands alone.** It answers RV105's review and ROOT's ruling on it:
- the review: `R/REVIEW_RV105/b0_01/REVIEW.md`, sha256 `d4807e9f16f3777b5e5616c71c3b743b77b635268e577eb3d7dd63aa47c2b008` (verified), and its `evidence/`;
- the ruling: RR "RV105 fails B0 as drafted on one local finding; I78 revises; RV101 confirms #1104's head".

`REVISION_01.md` maps each finding to the section that answers it. In this text, `[r01: X]` marks a passage that answers finding X.

**The brief:** `R/BRIEFS/B0_CONTRACT_AND_IDENTITIES.md`, sha256 `86bdccd2f8ec08b14e0c1d8b35929b505350af5cde994ab0cce46720cb17545f`.

**Documents only.** I made no source edits and no Git writes; Git reads used `GIT_OPTIONAL_LOCKS=0`. I ran no cargo, native, solver or test job, installed nothing, and wrote nothing to the system temp directory.

**Status.** This is proposed contract text for ROOT's selection, after RV105's confirmation. It installs nothing and reserves no name. Every choice it makes is in §8, with a recommendation and a decider. Owner-held items are prepared there, never decided.

**Notation.**
- **WT, NUM, P, T, R, RR** as in the dispatch. RR is append-only, so `RR:n` line numbers are stable.
- **PP** = `P/core/product_physics/src`. **RE** = `P/core/reporting/result_export`. **FK** = `P/core/solver/frame_kernel/src`.
- **PY** = `P/core/analysis_runs/retained_precision.py`. **RS** = `RE/src/retained_precision.rs`. **TS** = `P/apps/desktop/src/features/results/retainedPrecision.ts`.
- **C1** = `R/I32/f2a_wire_c1/WIRE_CONTRACT.md`. **C2** = `R/I32/f2a_wire_c2/CONTRACT_DELTA.md`. **C3** = `R/I52/prepared_public_contract_02/C3_DELTA.md` (selected with its ADDENDUM, RR:6950–6970).
- **DN** = `T/DESIGN_NUMERICS/DESIGN.md`. **D2** = `T/DESIGN_STANDING/DESIGN.md` (revision 5b.3). **CR** and **QUAL** = `T/IMPLEMENTATION/F2A_D1/CHANGE_RECORD.md` and `copies/QUALIFICATION.md`. **PLAN** = `R/I61/u8_plan_01/PLAN.md`. **PROBE** = `R/I68/u8_probe_01/PROBE.md`. **DOMAIN** = `R/I65/u4_g2_01/DOMAIN.md`.
- **SCHEMA** = `P/schemas/retained_precision_mp_v2.schema.json`. **CORPUS** = `P/fixtures/results/retained_precision_cases.json` (snapshot 07l: 17 bases, 286 mutations, 28 must-pass).
- **c** is the number of load cases in the invocation; **A** is the set of cases W1 attempts (§1, T-4); request order is the order of `model.load_cases`.
- **The published verdict** of case i is `numerical_quality.cases[i].solve_quality` in the ordinary envelope.

**Basis.**
- NUM `4e114af645`, which carries main `f8ed4f0551`, T6S's 19 files (PR #1104) and records. It is `b75069e6de` plus one records-only commit: the owner's decision that ROOT may raise M up to 6.0 GiB (RR "Owner decision: ROOT may raise M up to 6.0 GiB without asking"). This revision applies that decision in §6, §8 and §9.
- Against revision 1's basis `ecb541d63f`, the files this design cites are byte-identical: PP, RE `src`, FK, PY, RS, TS, SCHEMA and CORPUS (`git diff` empty). Only T6S's desktop, schema and test files differ.
- Code is cited by symbol. The input hashes are in §10.

## 0. Findings in brief

1. **D1 attempts W1 on every admitted case, but the readers reject a selected case whose published verdict is `checks_passed`.**
   - `retained_w1` attempts the one case with no quality check (PLAN §1.1).
   - All three readers refuse a `selected` case whose verdict is `checks_passed` (D6b, RR:8118).
   - The Passed cases that reach W1 today are mostly **W2-published**: the ordinary attempt failed with Range, and W2 then published a Passed report. W6, W-C1 (two-body case B) and the one-body pair are all of this kind, in both modes (PROBE §2 and §4; RV105's `evidence/probe_quality.log`). One-body A reached a native Selected before its proof refused.
   - In D1 this is harmless: a Passed case that selected would fall back at precommit, with the plain bytes and one notice.
   - **With c cases it is not harmless.** One Passed case that selects would make precommit refuse the whole successor, and the other cases' selections would be lost with it.
   - So the n-case transaction applies the trigger per case, **keyed on the published verdict** (§1, T-4; decision 1) `[r01: B-1]`.
   - The trigger's warrant is DN §4.3 and RR:2190 ("W1 is selected only for Sensitive and D-5-routed cases"). `not_required` is defined by C1:101 and D2 §4.9.2 ("the ordinary attempt passed"). C2:164 is D6b's quality-binding warrant ("No checks_passed status is inferred from W1 recovery") `[r01: N-11]`.
2. **Consequences for the B1 witnesses** `[r01: B-1]`.
   - **Two-body case B** is W2-published with verdict `CHECKS_PASSED`, so under T-4 it is `not_required`, not "unavailable with `{space: unresolved, tag: ceiling}`".
   - **RS refuses that case today.** RS's `not_required` rule also requires `initial` to be a Passed report and `w2` to be `not_triggered`; PY and TS do not. B1's three-reader change aligns RS to PY and TS (§3.3; decision 9).
   - **The Ceiling row** needs a case whose verdict is not `checks_passed` and whose native run ends at Ceiling. The three-case W-C2 of §1.4 still stands: A selected, B `not_required`, and C (A's loads plus B's) to be established by B1's probe (decision 2).
   - **W-C1 and the QUAL §4 W6 stack witness** stop reaching W1, because both have verdict `CHECKS_PASSED`; they are re-based. W2, W2b and the `attempted_examples` inputs have unrecorded verdicts, which B1's probe records first (§1.3; S-1).
3. **F-1: the biconditional as briefed is not faithful to the producer.**
   - The dense parity row comes from a second legacy lane, `legacy_dense_observation` (`solve_dense`). That lane has an absolute pivot guard (`DENSE_SOLVE_ZERO_PIVOT_GUARD` = 1e-12) and fails silently, with no row and no diagnostic.
   - The mode row records a different lane, the sparse-entry solve (`solve_preview_reduced_system`, `legacy`), with its own absolute 1e-12 guard (`SPARSE_SOLVE_ZERO_PIVOT_GUARD`).
   - The two lanes eliminate in different orders, so the mode row can record the observation as performed while no parity row exists. This is reachable in principle and not witnessed.
   - I give the briefed text (A) and a faithful text (B), and recommend B (§3; decision 8).
4. **The three G8s and G5s also diverge in other ways.** These first-failure differences were never exercised, because every base so far is Sensitive with a report and no W2.
   - **Scope:** RS checks the mode and parity rows on selected cases only; TS checks the mode row on every case; PY checks neither.
   - **Code:** RS uses `PREPARATION_MISMATCH`; TS uses `INVOCATION_MISMATCH`.
   - **Mode code 3:** TS admits it in sparse; the producer never emits it.
   - **Requested mode:** PY does not check `ordinary_attempts[i].requested_mode`.
   - **`not_required`** (G5) `[r01: B-1]`: RS demands a report with no W2; PY and TS do not.
   - F-1's alignment takes all five (§3.3; decision 9).
5. **D38's representation beside a selected case cannot be emitted by the n-case producer I propose.**
   - The native stage is one C2 `CaseBatchCall` (T-8), and the kernel gives every submitted case a Run.
   - A native failure before any Run therefore stops the whole call, so no case is selected and nothing is published.
   - The relaxation is still made, because C1:103 and C3:167 make the shape legal (RR:8823, RR:9113–9120). Its pin is a labelled synthetic corpus case (§2; decision 11).
   - The S-2 contract reading (a CaseSource with no Call) is settled by C2 §3: a source is registered at construction. Its digests can be recomputed from the binding (PY `_native_source_encoding`).
6. **`openpipestress.result_semantics/0.3.0/physics-retained-1` and its profile `exact_straight_retained_w1a_v2` are already reserved,** prospectively and in scope, since 2026-10-03 (RR:6960–6970; `R/verification/rv69_c3_selection_02/CHECKS.json`).
   - My recheck of the **full form**, at `b75069e6de` outside `P/execution`, finds no occurrence of either.
   - The bare `physics-retained-1` occurs once, in a T6S comment (`outputPolicy.ts`), which is not a wire value. So rechecks use the full form `[r01: N-9]`.
   - B0 confirms the reservation and states what B3 inherits (§5). B3 also needs its own formation definition, because C3 §1 excludes exact-profile E/ν.
7. **Three code sites count one case.**
   - D1.9's `l` row reads `load_cases[0]` only.
   - The `LoadCasesCapacity` row's cap is `LOAD_CASES` = 1.
   - G-B's `CaseLoads` reads one case.
   
   The dense in-build maximum leaves 28,389,922 B under 0.9 M (QUAL §3). §6 restates every cap row by scope and gives B1's re-pricing approach. It selects no M and no cap value.
   - **At today's M,** a fit for c ≥ 2 at D1's model caps is unlikely `[r01: N-10]`.
   - **Since the owner's decision of 2026-10-06,** ROOT may select a new M up to 6.0 GiB (6,442,450,944 B) by D-7's method, without asking. At that M, the 0.9 M budget is 5,798,205,849 B, about 2.2 GB above today's dense maximum.
   - So the probable path for B1 is a ROOT selection of M ≤ 6.0 GiB, not an owner decision. Only M above 6.0 GiB, or a supported-machine statement, stays owner-held (§6, §9; decisions 16 and 17).
8. **Nothing here changes the breadth order (U8 → B0 → B1 and B6 → PR-B1 → B2 and B3 → B4 if ruled → PR-B2 → B7 → B8), or the owner's F2a order.**
   - B1 gains a probe step, an audit, witness re-basings and one more RS alignment, plus a memory contingency (§9).
   - B2 needs ROOT to reserve C3a's names.
   - B3 needs its exact formation definition, and a three-reader G8 widening for the zero-pressure route (§5).

## 1. The receipt and freeze transaction for n cases (B1)

### 1.1 Scope

B1 admits c ≥ 1 load cases, with no combinations and no components. Every other D1 clause stays as it is and applies to every case.
- **D1.5 applies to every case:** no pressure regions, no equivalent static, and no modulus basis reference or temperature. So there is one default material basis (decision 7).
- **D1.4 and D1.9 are restated** in §6.
- **Combinations are B2's** (§4).

### 1.2 Contract text: T-1 to T-13

- **T-1. Admission (G-A).** One permit per invocation, by the D1 predicate as widened in §6. A refusal keeps the ordinary route exactly, as today.
- **T-2. One ordinary run.** The observed ordinary run covers all c cases and is unchanged. The one `ProductCapture` holds, per requested case:
  - its ordinary seed;
  - its solver observations (the mode row, and whether a parity row was produced);
  - its case-source inputs.
  
  The capture is one owner for the invocation.
- **T-3. Invocation gates, in this order.** Each decides for every case at once.
  - (a) **Domain** (load-state or exact-pressure model): the ordinary route. No notice.
  - (b) **Source finalization failure:** `Err`, as today.
  - (c) **Coexistence (DN §4.4).** If exact-block selected any case (the ordinary envelope carries `source_block_recovery`), the invocation publishes exactly the ordinary bytes. W1 is not attempted for any case, and there is no notice and no reservation.
  - (d) **G-B late refusal:** the ordinary route. No notice.
  - (e) **G-C.** Its attempt fact counts the **requested** cases `[r01: N-1]`. It holds only when the capture has exactly one seed per requested load case and every seed's `initial` is set. Otherwise G-C declines with exact bytes and no notice.
    - Today's `ordinary_solve_attempted` iterates only the seeds that exist. A run that blocks at case k < c−1 leaves later cases without seeds, and it would still return true.
    - Every other G-C fact is priced per invocation (§6).
- **T-4. The per-case trigger,** keyed on the published verdict `[r01: B-1]` (DN §4.3; RR:2190; C1:101; D2 §4.9.2; decision 1).
  - Case i is **`not_required`** exactly when `numerical_quality.cases[i].solve_quality == checks_passed`. G-C has already established that every case was attempted. A W2-published case whose published report passes is therefore `not_required`.
  - A case whose quality entry is absent, or whose verdict is anything else (`sensitive`, `unresolved`, `failed`), is in **A**, subject to the exclusion below.
  - **The DN §4.3 exclusion (decision 21)** `[r01: N-2]`. A case is not in A when its seed's `initial` is `structural_failure` with error tag `mechanism`, `asymmetric` or `invalid_input`, and its `w2` is not `published`. DN §4.3 says the trigger never fires for these. Such a case gets no product attempt and no notice; it stays an ordinary failed case.
  - A `not_required` or excluded case gets no product attempt, no source, no Run, no work and no notice.
  - **If A is empty,** the invocation publishes exactly the ordinary bytes, with no notice and no successor. This is a new typed `W1Fallback` variant, `NoTriggeredCase`. It is distinct from `LegacySeed::NotRequired` and from the `not_required` disposition `[r01: N-9]`.
  - For c = 1, T-4 is the only behaviour change from D1. §1.3 lists its effect on committed witnesses.
- **T-5. Notices reserved per case** (R-2, generalized).
  - Before any W1 work, one diagnostic slot and one maximum-length message are reserved for each case in A:
    - id `diagnostic:retained-precision:<case id>:unavailable`;
    - code `RETAINED_PRECISION_UNAVAILABLE`;
    - the N1 text, with capacity for C1:68's receipt-encoding suffix.
  - If any reservation fails, or any id collides with an existing diagnostic or with another reserved notice, the result is `NoticeReservation`: exact bytes, no notice and no W1 work.
- **T-6. Invocation custody.** The prepared-ordinary preconditions are checked once, for every case:
  - `MECHANICS_SOLVED`;
  - the preview contract id;
  - no exact-block selection;
  - each case's observations bound to the envelope (`bind_observations`, per case).
  
  A failure is a whole-invocation Preparation fallback, which publishes the T-12 notices. For c = 1 this is `tiny_spring`'s path, unchanged: its verdict is `unresolved` (`ASSEMBLY_UNRESOLVED`, a NumericallyUnresolved class, which DN triggers).
- **T-7. Per-case preparation,** over A in request order (decision 4).
  - Each case in A gets its own C3 `ProductAttempt`: at most one per owner, with ids in actual start order. The attempt records the preparation that case actually performed.
  - On success, the case's prepared `CaseSource` is registered in `body.sources` at construction (C2 §3). Its `preparation.attempt_ref` names that attempt's own index; today `bind_preparation` fixes it at 0.
  - **A preparation-stage failure** makes that case `unavailable`: phase `preparation`, cause `prepared_product_failure` naming the attempt, `run: null`, `source_ref: null`, and **no `CaseSource`** for it `[r01: S-2]`. This is what PY's `_g5_stages` and TS's `productAttempts` require: preparation `completed` exactly when a source exists. It is also what the producer does today, since `serialize_unavailable` refuses a prepared source without a call. The other cases continue.
- **T-8. Native** (C2 §4; decision 3).
  - One `CaseBatchCall` covers the prepared sources of A, in request order, through one `RecordedInvocation` meter (Li = 60,000,000,000) with the case limit (20,000,000,000) for each case.
  - Groups form by full stiffness-byte equality inside the call. Under D1.5 every case shares one stiffness, so there is one group, and its slot builds are shared, as C2 §4 meters them.
  - Every submitted case gets a Run (C2: an exhausted-before-start case still has a Run, with `group: null`).
    - A Run ending `selected` continues to T-9.
    - A Run ending `unresolved` or `refused` makes the case `unavailable`, as `kernel_<kind>` with phase `kernel`, with its Run.
  - A failure of the call itself before any Run (origin capacity, count range, reservations, meter construction) applies to every submitted case. Each is then unavailable with D38's representation (§2), no case can be selected, and T-12 applies.
- **T-9. Freeze,** per case whose Run is `selected`: the dual-readout proof, certificate, observables and G5a (C3).
  - A failure makes the case `unavailable`: `facade_certificate`, phase `facade`, with its selected Run.
  - The ordinary envelope stays one untouched owner throughout. Per-case candidates do not own it.
- **T-10. The outcome.** If no case is selected, T-12 applies (D2 §4.9.2: a successor needs at least one selected case; RR:8436). Otherwise T-11 runs.
- **T-11. Staging, serialization, precommit and transfer.** T-11 runs only when at least one Run is selected, so the call exists.
  - **Staging.** One copy of the ordinary envelope. The successor identity and profile are set once.
    - For each selected case, in request order: overlay its rows (values, the `recovery_method` token, the maxima patches); omit its legacy disclosure under T1 (a); append its `RETAINED_PRECISION_SELECTED` diagnostic.
    - Then, for each unavailable case, in request order: append its receipt-backed `RETAINED_PRECISION_UNAVAILABLE` diagnostic (`UNAVAILABLE_MESSAGE`). Its legacy disclosure is kept and referenced (D39).
    - `not_required` and excluded cases get nothing.
    - So each selected or unavailable case has exactly one diagnostic (G4).
    - A staging fault abandons the successor.
  - **The receipt body:**
    - `cases[]`: one entry per requested case, in request order, each `selected`, `unavailable` or `not_required`. A case excluded by decision 21 cannot appear in a successor, because its failed solve blocks the envelope.
    - `ordinary_attempts[]`: one per requested case;
    - `product_attempts[]`: in actual start order;
    - `sources[]`: in registration order;
    - `material_bases[]`: one entry (D1.5);
    - `calls[]`: the one `CaseBatchCall`;
    - `groups[]` and `builds[]`: as recorded;
    - `work.execution_order`: every Run in actual order;
    - `work.charged`: the call's `invocation_after`;
    - `legacy_source_work[]`: per D39.
    
    `[r01: N-3]` the empty-`calls` and zero-`charged` branches are removed, since they cannot occur here.
  - **Cumulative snapshots** `[r01: S-3]`.
    - `ProductAttempt.adapter`, and every other C3 field that C3 §3 calls a "cumulative prefix of the actual owning ProductCapture", is a snapshot of the one shared capture.
    - **The record point is the attempt's terminal stage:** the snapshot is taken when the attempt's last entered stage completes or fails. That is preparation for a T-7 failure, native for a T-8 non-selected Run, and the last proof stage for T-9.
    - Snapshots are not ordered in `product_attempts[]` (start) order. An earlier-started attempt can end later than a later-started one.
    - No reader checks an order among snapshots, and nothing sums them (C3 §3).
  - **A serializer failure of any kind abandons the successor** (decision 5). B1 does not use the per-case `receipt_failure` demotion: the shape stays schema-legal and reader-legal (D19), but the producer does not emit it. C1:68 already requires abandonment when a run cannot be encoded.
  - **A known B1 limit** `[r01: N-5]`. `ordinary_value` (PP `retained_wire.rs`) refuses `Untranslated` (`ordinary_attempts[].formation.recovery_finding`) for any seed with R-b′'s `recovery_demoted`. T-11 serializes every case's ordinary attempt, attempted or not, so one R-b′-demoted case abandons every other case's successor. It is fail-safe. A wire member for R-b′ is a later contract question (RR:9019, F3).
  - **Precommit:** the accepted Rust reader, with the actual invocation. Its first failure abandons the successor.
  - **The transfer** only moves values, as in D1.
- **T-12. The fallback publication** (all cases unavailable, or abandonment after T-6, T-8 or T-11). It publishes:
  - the ordinary bytes, unchanged;
  - then one N1 notice per case in A, in request order, from the space reserved at T-5, with no allocation.
  
  **The receipt-encoding detail** `[r01: N-4]`.
  - **Scope.** The detail is appended only when the abandonment cause is a serializer failure whose check is one of C1:68's details (`work_counter_range`, `work_counter_inconsistent`, `saturation_not_excluded`) or `publication_hash_range` (PP `receipt_encoding_detail`). Every other cause leaves every notice plain.
  - **Placement.** It goes only on the notices of cases that were `selected` when the successor was abandoned (decision 6). The detail names the invocation's cause, which may come from another case's run. A case that was unavailable for its own cause keeps the plain notice.
  
  No receipt is published. `serialize_unavailable` stays private.
  
  B1 extends `u3_r2_base_readers_accept_the_unavailable_notice`, which covers one notice, to an ordinary envelope carrying several N1 notices.
- **T-13. Counting.** For any c, a permitted invocation still has one ordinary run and reaches G-C once (`ONE_RUN_THROUGH_G_C`).

**The outcome table** `[r01: N-3]`:

| Outcome | Published | Notices |
|---|---|---|
| T-1 refusal; T-3 (a), (c), (d) or (e); `StackReservation` (reserved-stack spawn fails; the ordinary route runs on the caller thread); `PermitUnbound` (unreachable); `NoTriggeredCase`; `NoticeReservation` | Ordinary bytes, exactly | None |
| T-6 custody failure; T-8 call failure; no case selected; T-11 abandonment (staging, serializer, precommit) | Ordinary bytes, then notices | One per case in A; receipt-encoding detail only per T-12 |
| One or more cases selected, and T-11 completes | The successor | None (unavailable cases carry their receipt-backed diagnostic) |

### 1.3 The effect of T-4 on committed witnesses `[r01: B-1, S-1]`

Verdicts are from PROBE's `I68_ORDINARY` lines, in both modes, and RV105's classification, which marks W2-published cases by `SOURCE_BLOCK_RECOVERY_UNAVAILABLE` with a Passed verdict (`needs_source_recovery`).

| Witness or test | Published verdict | Under T-4 |
|---|---|---|
| Milestone; L = 0; two-body case A | Sensitive (two-body A is W2-published, b = 518) | Unchanged |
| `first_load_only` (RV93 N-5) | Sensitive | Unchanged: Candidate, one notice |
| `tiny_spring` | `unresolved` (`ASSEMBLY_UNRESOLVED`), attempted, envelope blocked | Unchanged: in A; T-6 custody fails; one notice |
| **W-C1** (two-body case B, `u8_real_input_fallbacks_append_one_notice`) | `checks_passed`, W2-published | **Changes:** `not_required`, `NoTriggeredCase`, exact bytes, no notice |
| **QUAL §4 W6** (`witness_w6_force_scaled`, `Fallback("Native")`) | `checks_passed`, W2-published | **Changes:** `NoTriggeredCase` |
| **QUAL §4 W2** (`witness_w2_cap_maximal`, `Fallback("Preparation")`) and **W2b** (`witness_w2b_cap_maximal_solvable`, `Fallback("Candidate")`, the only full native run at the cap-maximal counts) | **Not recorded** | **Changes if `checks_passed`** (S-1) |
| `registered_g_c_declines_only_unattempted_solves`, over `attempted_examples` (milestone; the 1e-300 spring; K2a's deferred formation; the `rejected_stress_range` pair) | The milestone and the pair are Sensitive; the spring is `unresolved`; K2a's is not recorded | The test still passes (it accepts any `Some(_)`). Its doc line ("a solve that ran but failed still reaches W1") may stop holding for any example whose verdict is `checks_passed` after W2 (S-1) |
| The `u3_*` and `u3g2_*` facade tests; W1, W2-deep, W3, W4, W7, headroom | Milestone (Sensitive), or exact-selected | Unchanged (RV105 §2) |
| `retained_product_tests.rs` (`prepare_observed`, which bypasses `retained_w1`) | n/a | Unchanged, because T-4 sits in the transaction, not in preparation |
| CORPUS | Every base is Sensitive with a report and no W2 | No entry changes; B1 adds entries (§3.4) |

**B1's probe records first, before T-4 lands** `[r01: S-1]`:
- the published verdict and the seed's `initial` and `w2` for every QUAL §4 witness input (W1 to W7, W2-deep, headroom);
- the same for every `attempted_examples` input;
- for decision 21, the seed's structural error tag of every attempted failure.

Any input that becomes `not_required` (or excluded) under T-4 is re-based, if its witness role still needs W1, on a Sensitive input with the same shape. W2b's cap-maximal native run is the one most likely to need it.

### 1.4 W-C2 as B1's acceptance witness (decision 2) `[r01: B-1]`

**The proposal:** a three-case invocation on U8's two-body model (`u8_two_body_case_a`), in both modes.

| Case | Loads | Expected | Status | Established? |
|---|---|---|---|---|
| A | Milestone moments on body 0 | W2-published, Sensitive, b = 518, native Selected (PROBE §4; the F-1 dump: `initial: structural_failure/range`, `w2: published, 518`) | `selected` | Yes, one case at a time. Dense needs F-1's repair |
| B | W6's tip force and torque on body 1 | W2-published, `CHECKS_PASSED` (PROBE §4; RV105) | `not_required` | Verdict yes. **Readers: needs RS's `not_required` alignment** (§3.3), since PY and TS admit it today |
| C | A's loads plus B's loads | Verdict not `checks_passed` (expected Sensitive, from body 0); native Ceiling (from body 1's rows) | `unavailable`, Run `kernel_terminal` `{kind: unresolved, reason: {space: unresolved, tag: ceiling}}` | **No. B1's probe establishes it first** |

**Why three cases:**
- One producer-solved base then carries all three statuses beside each other.
- Case B gives the first producer-emitted `not_required`, and it is W2-published, the exact shape B-1 found unpinned.
- It is D38's pin base (§2) and F-1's dense base (§3).

**The stop rule.** If case C's verdict is `checks_passed`, or its native run does not end at Ceiling, B1 returns with its probe record. Fallback constructions:
- (i) case C = B's loads plus a small moment on body 0 that K-D5 demotes;
- (ii) an input on body 1 alone whose verdict is Sensitive, built from W6's geometry.

**Re-basing.** W-C1 and the W6 stack witness are re-based on case C alone, as a one-case D1 input, if C alone gives the same outcome. That keeps the force-scaled path, since C is range-scaled.

The counts are inside D1's model caps (n 4, m 2, g 5, materials 2). Only D1.4 changes (§6).

## 2. D38's reader relaxation (B1, three readers)

**The rule today.** A native stage that was entered implies a Run.
- RS `g5_products`: the `if st["native"] != "not_entered"` block requires `run_ref` to be non-null.
- PY `_g5_stages`: the `else` branch requires `run_ref` and `run`.
- TS `productAttempts`: `(stage.native === 'not_entered') === (a.run_ref === null)`.

**The relaxed rule R-D38** (G5, `RETAINED_PRECISION_PRODUCT_ATTEMPT_MISMATCH`), for every product attempt `a` and its case:
1. `a.run_ref` is null exactly when `case.run` is null. Unchanged (C3:167).
2. If native is `not_entered`, `run_ref` is null. Unchanged.
3. If native is `completed`, `run_ref` is non-null and the Run's terminal is `selected`. Unchanged.
4. If native is `failed`, exactly one of these holds:
   - **(4a)** `run_ref` is non-null and the terminal is `unresolved` or `refused`. Unchanged.
   - **(4b) D38.** All of the following:
     - `run_ref` and `case.run` are both null;
     - `a.result` is `unavailable` with `error.kind == "capture"`, carrying the actual cause;
     - the stage record is `done(1, [failed])`: preparation completed, native failed, every later stage `not_entered`, and `proof` null;
     - the case is `unavailable`, with cause `prepared_product_failure` naming `a` and reason `(source_unavailable, preparation)` (the existing D4d mapping for capture with no Run);
     - `a.source_ref` is non-null, resolves to a `CaseSource` whose `preparation` binds `a`, and **equals `case.source_ref`** `[r01: N-6]` (TS already requires `c.source_ref === a.source_ref`; RS and PY add it);
     - no Run, `Call.source_refs` entry, Group `source_refs` entry, Build or `execution_order` entry names this case or that source.

Every other shape stays refused. This is a ruled change (RR:8823), not a silent weakening.

**The test-hook D38 shape stays refused, by design** `[r01: N-6]`. Today's hook path (`withdraw_next_native_source`) withdraws the source, and the D1 serializer emits `source_ref: null` with native failed. Under (4b), a native-stage capture failure always has a registered prepared source, so a shape with native failed and no source matches neither (4a) nor (4b).

**The contract reading for S-2** (RR:9120: "D38 with a prepared source").
- C2 §3 registers a case source "after PrimitiveSource::new and map validation succeed, regardless of final selected/unavailable outcome". Its `kernel_source_sha256` and `stiffness_sha256` are raw sha256 of the source's own K4SRC and K4STF encodings, so they do not depend on a kernel call.
- G8 already recomputes both from the binding (PY `_native_source_encoding`).
- So (4b)'s `CaseSource` is emitted with those digests and no Call.
- The producer's `serialize_unavailable` keeps refusing S-2 in B1 (`Untranslated`, `sources[].kernel_source_sha256`). The shape cannot be emitted beside a selected case under T-8, so B1 has no producer reason to emit it (decision 11).
- T-7's preparation-stage failure is a different shape: no source, `native` `not_entered`, and the existing rules (S-2 of RV105).

**B1's obligation.** Every one-case D38 receipt so far failed first at G3 ("no selected case", RR:9109). Any later check that assumes a prepared source has a Call or Run has therefore never been exercised. B1 lists each such check in all three readers and either relaxes it to (4b) or shows it does not apply.

**The shared corpus pin** (B1's snapshot after 07l):
- **One base, `d38_beside_selected`,** labelled "synthetic: not producer-emittable under T-8". It is derived from W-C2's producer-solved base by rewriting case C into (4b)'s shape, with every hash resealed `[r01: N-6]`:
  - remove case C's Run and its `execution_order` entry;
  - remove the Builds whose origin is case C's Run. Case A ran first in the batch, and C2 §4 forbids an earlier selected snapshot from seeing later slots, so case A's snapshot references none of them;
  - remove case C's source from the call's and the group's `source_refs`;
  - **keep the shared group and case A's builds,** which A's Run references;
  - recompute `charged` and the call's after-value;
  - the cause is a typed `CaptureError::Origin` value.
- **One must-pass entry,** with the invocation: G0–G8 pass, and the standing is `needs_recompute` (an unavailable case).
- **Mutations.** Each has one first failure that all three readers share. B1 fixes the gate and code from the readers.
  - m1: `error.kind` set to `native`;
  - m2: native set to `completed`;
  - m3: `run_ref` non-null while `run` is null;
  - m4: `execution_order` still lists case C;
  - m5: `proof_start` set to `completed`;
  - m6: `source_ref` null while preparation is completed;
  - m7: case C's source listed in the call's `source_refs`;
  - m8: `case.source_ref` differs from `a.source_ref`.

**Re-qualification.** The Rust reader is on the D1 call graph (precommit). Its D38 change is re-qualified once, together with F-1 and the `not_required` alignment (RR:10474; QUAL §11).

## 3. F-1: the parity-row rule, and the rest of B1's reader alignment

### 3.1 The facts

**The producer emits a case's `sparse_live_path_dense_parity_relative_delta` row only when all of these hold** (PP `solve_load_case_observed` and `append_sparse_live_path_evidence`):
- the mode is `dense_scrutiny`;
- the basis was formed (not range-deferred);
- `w2_publication` is `None`, that is b = 0 (OQ5, RR:1285);
- the dense reduction succeeds;
- `legacy_dense_observation` (`solve_dense`) succeeds;
- the sparse-entry assembly and its solve succeed, and the dimensions match;
- the delta, the relative delta and the residual are finite.

**When the last three fail,** a warning `SPARSE_LIVE_PATH_EVIDENCE_UNAVAILABLE` names the case. A `solve_dense` failure has no diagnostic at all.

**The mode row's observation fields** (`original_profile_entries`, `ordered_profile_entries`, the two half-bandwidths, `nonpositive_pivots`, `pivot_condition_ratio_proxy`, `max_abs_sparse_residual`) record the sparse-entry lane in `solve_preview_reduced_system` (`legacy`), in both modes. At b ≠ 0 every one of them reads `not_observed`.

**The two lanes can disagree.** Both have absolute 1e-12 pivot guards, but they eliminate in different orders (partial pivoting in natural order; LDLᵀ in RCM order). A small Schur pivot can fall below the guard in one order and not the other. I found no committed witness.

In dense mode the observation-lane guard cannot refuse: its estimate, 24 B per profile entry, is below the dense guard's 96 B per entry for the same dimension.

**The producer's capture already binds** the produced and the published parity presence (`parity_produced`; `bind_observations`), and refuses a sparse parity row.

### 3.2 The contract text

**Text A, as briefed.** For every case, the parity row is present exactly when the mode is dense and the mode row records the DEC-050/053 observation as performed. It is absent at b ≠ 0, and there is at most one per case.
- "Performed" would mean that the mode row's `metadata.basis` carries `original_profile_entries=<decimal>`, not `not_observed`.
- **A defect:** a dense case whose `solve_dense` lane failed carries a mode row recorded as performed and no parity row. Text A refuses that faithful base.

**Text B (recommended; decision 8).** At G8, `RETAINED_PRECISION_PREPARATION_MISMATCH`, for every case of the receipt (selected, unavailable and `not_required`, because their rows all come from the one ordinary run). Let `o = ordinary_attempts[case.ordinary.attempt_ref]`.
- **P1.** Exactly one `linear_solver_mode_basis` row has `basis_ref` equal to the case, and its value is the mode code: 1 in `sparse_interactive`, 2 in `dense_scrutiny`.
- **P2.** At most one `sparse_live_path_dense_parity_relative_delta` row has `basis_ref` equal to the case.
- **P3.** In `sparse_interactive`, there is no parity row.
- **P4.** If `o.w2.kind == "published"` (b ≠ 0, a typed fact that G5's D6c already validates), there is no parity row (OQ5). The producer suppresses the observation exactly when `w2_publication` is `Some`, and records the seed `Published` at the same site.
- An absent parity row in dense mode at b = 0 is admitted.
- **The disclosed limit:** deleting that row after resealing is not detected. The base preview-physics-1 route checks the row in no reader either, so the successor is no weaker than its base. The row is `non_quantity`, carries no reliance, and the publication hash binds it.

**Optional P5 (decision 10; I recommend deferring it).** If `o.w2.kind == "published"`, the mode row's observation fields all read `not_observed`. That is OQ5's disclosure half. It needs a parser for the basis text grammar in three languages and adds no reliance.

### 3.3 B1's three-reader alignment: order, codes and the `not_required` rule (decision 9)

**G8** (F-1). In G8, after the invocation, project, model-scope and material checks, a per-case loop runs in request order:
1. `o.requested_mode == invocation.solver_mode`;
2. `o.material_basis_ref`;
3. P1;
4. P2–P4.

All four use `RETAINED_PRECISION_PREPARATION_MISMATCH`, RS's current code.
- **TS:** switches its mode-row and requested-mode codes from `INVOCATION_MISMATCH`, and drops mode code 3.
- **PY:** adds the requested-mode check and P1–P4 in that position.
- **RS:** widens its check from selected cases to every case, and replaces "exactly one parity row iff dense" with P2–P4.

**G5** `[r01: B-1]`. The `not_required` rule, in all three readers, is G5 `RETAINED_PRECISION_ATTEMPT_MISMATCH`, in the ordinary class. A `not_required` case requires `product_attempt_ref` null, `initial.kind != "not_attempted"`, and a verdict of `checks_passed`.
- **RS changes:** it drops the conjuncts `initial.kind == "report"`, `initial.outcome == "checks_passed"` and `w2.kind == "not_triggered"` from its `not_required` rule.
- **PY and TS** already have exactly this rule (`_g5_ordinary`; `ordinaryAttempts`).
- **Each reader keeps its existing report-outcome equality:** where `initial` is a report, its outcome equals the verdict. RS's extra `w2 == not_triggered` guard on that equality is equivalent under D6c, since W2 runs only after a failure.

**One re-qualification** covers D38, F-1 and this rule, because the RS reader is on the D1 call graph.

### 3.4 The corpus (B1's snapshot) `[r01: N-7, B-1]`

CORPUS has no milestone base; the milestone successors exist only as `fixtures/results/retained_precision_milestone_successor_*.json`. B1 uses CORPUS's producer-solved L = 0 bases instead. Dense L = 0 has a parity row at b = 0. Adding the milestone as D-U6-5 copies is optional.

- **Must-pass:**
  - the dense and sparse W-C2 bases: case A selected at b = 518 (dense: no parity row); case B `not_required` and W2-published; case C unavailable;
  - the existing `u8_l0_isolated_node_dense_scrutiny` and `u8_l0_isolated_node_sparse_interactive`.
- **F-1 mutations:**
  - dense L = 0's parity row copied onto W-C2's dense case A (P4);
  - dense L = 0's parity row duplicated (P2);
  - a parity row added to sparse L = 0 (P3);
  - mode code 3 in sparse L = 0 (P1);
  - `requested_mode` flipped in one case.
- **`not_required` mutations** on W-C2 case B:
  - its verdict set to `sensitive` while still `not_required`;
  - its `initial` set to `not_attempted`;
  - its `product_attempt_ref` set non-null.
  
  B1 fixes each first failure from the three readers. B6's corpus edits, including mutation 277 (`g7_not_required_quality_enum_invalid`), come before this snapshot.
- **Under Text A only,** one more mutation removes dense L = 0's parity row, and A refuses it.

### 3.5 Owner-held check

This aligns readers to OQ5 as approved and to C1:101's `not_required`. It changes no dense or lane ceiling and no observation framing. Successors are not public before B8, so no public meaning changes (RR:12344, "Not owner-held").

## 4. C3a: the preparation-only operand (B2)

### 4.1 The gap

RR:7048–7056 records it.
- A retained mechanics combination solves its own combined source (C1 §5). C2 §3 builds that source from its operands' `CaseSource`s. Under C3, the selected cases' sources are prepared: their section terms come from the annular inputs, so their stiffness differs from the ordinary K.
- A `not_required` operand has no prepared source. C3 forbids it a product attempt ("not_required requires null"), and any C3 `ProductAttempt` that is not `Ready` is unavailable. So C3 cannot represent preparing that operand without making it unavailable (an availability defect) or `Ready` (fake readiness).
- Solving the operand case on demand would be a hidden solve.
- C1 and C2 already anticipate operands with no selected snapshot:
  - C1 §5 `cache_inputs`: "No snapshot for a prepared ordinary source means an empty slots array";
  - C2 §4: "Prepared ordinary operands import none".

### 4.2 The contract text (option P1, recommended; decision 12)

- **A new body member,** `operand_preparations: [OperandPreparation]`, in actual start order, empty if none.
- **`OperandPreparation`** is closed, and every member is required:
  ```text
  { id:U, definition_id:"RP-PREPARED-ORDINARY-DUAL-v1",
    owner_ref:{kind:"case",index:U}, ordinary_attempt_ref:U, material_basis_ref:U,
    purpose:"combination_operand", requested_by:[U],          // combination indices, ascending, non-empty
    source_ref:null|U,
    result:{kind:"prepared"} | {kind:"refused", error:PublicFailure},  // error kind "preparation" only
    stage:"completed"|"failed",
    preparation:{members:[PreparedMember]},
    adapter:AdapterTrace, operational:{old:[MemberOperational], new:[MemberOperational]} }
  ```
  All types are C3's. A refused record has `source_ref: null` and no `CaseSource`, as T-7.
- **`CaseSource.preparation`** gains a second closed branch, `{operand_preparation_ref:U, sha256:H}`, beside C3's `{attempt_ref:U, sha256:H}`. Existing receipts keep their bytes.
- **Its sha256** is H(`retained_precision_operand_preparation_v1`, payload). The domain name is proposed here, for ROOT's reservation. The payload is C3 §2's semantic preparation payload, with `ordinary_attempt_ref`, `owner_ref` and `purpose` from the record. It has no `source_ref`, work or hash fields, so no cycle is possible.
- **`adapter`'s record point** is the record's terminal stage, as in T-11, and is not ordered relative to other records.

**The rules:**
1. **Who gets one.** Only a `not_required` case whose source some mechanics combination needs on the retained route. At most one per owner, shared by every combination in `requested_by`.
   - A selected case, or an unavailable case with a prepared source, supplies its attempt's `CaseSource`.
   - An unavailable case without a prepared source makes the combination `retained_unavailable`. There is no second preparation (C3: "C3 does not authorize retries").
2. **What it never is.**
   - It never has a native stage, proof, Run, Call, Group or Build.
   - It never changes its owner case's status, rows, diagnostics, `numerical_quality` or standing, and never puts a method token on its rows (G6).
   - It has no `Ready` state.
   - Its work is C3 preparation work, recorded as it actually happened. It is not W1 LME and is not added to any budget.
3. **`prepared`.** The `CaseSource` is registered (C2 §3), and the `CombinationSource` operands reference it. The combination's own Call and Run follow C2 §4, with no imports from this operand.
4. **`refused`.**
   - Every combination in `requested_by` is `retained_unavailable`, with reason `{code: "combination_unresolved", phase: "preparation", cause: {kind: "operand_preparation_failure", operand_preparation_ref: U}}`, and with `run`, `source_ref` and `call_ref` all null.
   - A null `call_ref` is legal only with this cause. No kernel call was made; C2's `pre_source_refusal` has a Call because the kernel itself refused.
   - The combination's rows keep their ordinary values. Its own unavailable diagnostic names the combination (C1 G4, combination analogue). Standing follows C1 §6.
   - The owner case gets no notice: nothing it publishes changed.
5. **Readers, three languages:**
   - G1: shapes;
   - G2: encoding;
   - G3: coverage. Each record's `requested_by` combinations reference its source; every `not_required` operand of a retained combination has exactly one record; the owner case is `not_required`;
   - G5: stage, result and work consistency, as C3's preparation stage, with `RETAINED_PRECISION_PRODUCT_ATTEMPT_MISMATCH` and `RETAINED_PRECISION_WORK_MISMATCH`;
   - G8: the old and new tuple equalities and the preparation binding, as C3, with `RETAINED_PRECISION_PREPARATION_MISMATCH`.
6. **The schema and the registration.** B2 lifts `combinations` from `maxItems 0` and adds these `$defs`. The schema is one of the 13 reviewed statics, so PR-B2 carries a registration diff. That was already in PLAN §2.1's B2 row.

**Two disclosed properties:**
- A retained combination's rows are its own solve on the prepared stiffness. A `not_required` operand's rows are binary64 on the ordinary K. The published combination is therefore not bit-for-bit the factor-weighted sum of its operands' published rows. C1 §5 already accepts this ("Own solve/certification/classes").
- Under D1.5, every operand's preparation repeats the same section preparation. Sharing it would need its own contract, because C3's payload is owner-bound; B2 does not share it.

**Alternatives:**
- **P2:** reuse `ProductAttempt` with a new result kind `prepared_operand`. This rewrites C3's `not_required` rule, D19/D20 and D37's stage tables in all three readers.
- **P3:** retain a combination only when every operand is selected, and otherwise publish it as `ordinary` (`no_retained_mechanics`). This leaves the gap open, which RR:7051 forbids for full F2a. P3 is acceptable only as an explicitly declared interim.

## 5. `physics-retained-1`: the reservation, and what B3 inherits

**The reservation.**
- `openpipestress.result_semantics/0.3.0/physics-retained-1` and its profile `exact_straight_retained_w1a_v2` were reserved prospectively, in scope, with the C1/C3 names (RR:6960–6970; CHECKS.json `matches: []` at `92790ca0cc`).
- **The recheck** (`git grep -F`, full form, outside `P/execution`) finds 0 hits for either name at `ecb541d63f` and at `b75069e6de` (RV105 agrees at `f12fed9d69`).
- The bare `physics-retained-1` has one comment hit in T6S's `outputPolicy.ts`, which is not a wire value. **ROOT's recheck uses the full form** `[r01: N-9]`.
- **B0's text:** the identity is reserved for physics-1's exact-route successor, with the profile `exact_straight_retained_w1a_v2`. It is not installed. B3 installs it atomically with its three readers and carriers (DN §6, F2a's atomic rule; D2 §4.9.1).

**What B3 inherits (decision 14):**
1. **The table.** A new `P/fixtures/results/semantic_contract_v0_3_physics_retained_1.json`, built the way the preview successor's table was built (RV78: eight places, RR:8795ff):
   - the contract id and profile;
   - `inherited_semantic_contract_sha256` = sha256 of `semantic_contract_v0_3_physics_1.json`'s raw bytes (`9a2cf626…`, QUAL §5);
   - `receipt_policy`, `receipt_schema`, `product_formation_definitions`, `accuracy_classification` and `formation_warrant`;
   - physics-1's rows, unchanged.
   
   **The successor-list precedent** `[r01: N-8]`. Base tables carry `reserved_inactive_successors`: physics-1 lists `source-blocks-1` and `precision-2`, and load-reference-1 lists `load-reference-source-1`. The retained precedent, though, left preview-physics-1's list unchanged when `preview-physics-retained-1` was added. **Recommended: follow the retained precedent. physics-1's table bytes are not edited.** It is a reviewed static, and editing it would change the hash that its successors inherit. The choice is part of decision 14.
2. **The receipt.** The same C1/C2/C3 family, with the exact-route specifics:
   - the material `shear_origin` is `derived_e_nu` (C2 §3);
   - `section_terms.geometry.route` is `exact`;
   - normalized OD and effective wall per `SOURCE_ODWALL_EXPECTATIONS` (C2 §3, RV42-1).
3. **A formation definition of its own.** C3 §1 says the exact table does not claim `RP-PREPARED-ORDINARY-DUAL-v1` applies to exact-profile E/ν. B3 designs it; I suggest the name `RP-PREPARED-EXACT-DUAL-v1` (0 hits at `b75069e6de`). Its name, its H domain and its RV69-style review are ROOT's reservation at B3's design, not now.
4. **Admission** (DN §4.3, W1a):
   - model 0.3.0 exact, with an explicitly empty pressure-region list;
   - no exact-block selection, under T-3 (c). Physics-source-1 is the exact route's exact-block method, and every invocation where it selects any case stays exactly as today.
   
   This widens D1.3, so it is a widening re-qualification (PLAN §2.1).
5. **The readers:** S-G1's `<physics-retained>` branch in all three languages.
   - It inherits physics-1's `contract_evidence.exact_cases`, `pressure` and `connector`.
   - Its G8 uses physics-source-1's `actual_materials` through S-C (D2 §4.9.1, and G8's row in §4.9.3).
6. **The carriers:** new branches in `results.v0.3.schema.yaml`, `analysis_run.v0.3.schema.json` and `stress_neutral_export.v0.3.schema.json` (D2 §4.9.6). T6S's per-route output policy gets this route's entry; until then the route refuses (I74 PLAN §4.3).
7. **B3's second promised route** is 0.3.0 `legacy_pressure_v1` with zero pressure (DN §4.3). It publishes preview-physics-1 today (`mechanics_producer_for_model`), so its successor stays `preview-physics-retained-1`, with no new identity (decision 15).
   - **But it needs more than a D1.3 widening** `[r01: N-8]`. The model carries `pressure_contract {1.0.0, legacy_pressure_v1}`. All three readers' G8 require `pressure_contract` to be null, and `family_clauses` refuses it under D1.3.
   - So B3 also carries a three-reader G8 widening for this one contract value, with zero pressure, and its corpus pins and mutations.

## 6. The caps per invocation, and M's re-pricing approach for B1

**Today (QUAL §6; DOMAIN §2; PP `retained_memory.rs`):**
- `cap_rows` prices `l` from `load_cases[0]` only: the census sets `primitive_loads` at index 0. `LOADS` = 128.
- `LoadCasesCapacity` is capped by `LOAD_CASES` = 1. `family_clauses` refuses `load_cases.len() != 1` at D1.4.
- G-B's `late_observations` reads one case's `CaseLoads`.
- The profile was generated at `l ≤ 128` for one case (`profile_tree.json` basis).
- The dense W3 maximum is 0.8929 M, 28,389,922 B under 0.9 M. Its text-error budget is 1.81 % of TAV_W (QUAL §3).

**The restated cap rows** (contract text; the values are B1's study's, and ROOT selects them):

| Row | Scope | Today | B1 |
|---|---|---|---|
| Load cases `c` | Per invocation | ≤ 1 (D1.4) | ≤ C |
| Loads per case `l_i` | Per case | `load_cases[0]` ≤ 128 | Every case ≤ l |
| Total loads `Σ l_i` | Per invocation | (= l) | ≤ L |
| Combinations, components | Per invocation | 0 | 0 (B2 widens combinations) |
| n, m, g, Σr, s, materials, temperature points, sections | Per invocation (one model) | D1.9 | Unchanged unless the study trades them for C |
| Text, raw census, digest, control bytes | Per invocation (whole request) | D1.9, D1.11 | Unchanged |
| G-B `CaseLoads` | Per case and total | One case | max_i l_i ≤ l and Σ ≤ L |
| G-C facts | Per invocation | One case | Per invocation, with the attempt fact over every requested case (T-3 (e)) |

**The re-pricing approach** (B1, starting with PLAN decision 9's 3–4 h read-only study by I65):
1. Add c and Σl to the census and to the TEXT tool's variables (`g5_profile.py`, `profile_tree.json`).
2. Classify every roster row and atom as per invocation, per requested case, or per attempted case (|A| ≤ c) `[r01: N-11]`.
   - **Per requested case:** ordinary seeds and observations, ordinary results and diagnostics per case (P_final ≤ 2,115 at the caps), receipt case entries and ordinary attempts.
   - **Per attempted case:** the case-source inputs that become prepared `CaseSource`s, product attempts and their traces, native Runs, records and slot snapshots, frozen candidates, staged rows, N1 notices (`NOTICE` and `NOTICE_moving` × |A|).
   - **Per invocation, under one group:** slot builds.
   - T-8, T-9 and T-11 keep every attempted case's Run and frozen candidate live until staging. So the W3 peak carries |A| of them at once `[r01: N-10]`.
3. Restate each row's monotonicity lemma in (c, l, Σl).
4. Evaluate E_mov,max + R per mode at the proposed caps, by `cap_priced_maximum` and `admission_bound`, against the 0.9 M margin at the registered M = 4,026,531,840 B.
5. **If the caps do not fit at today's M,** ROOT has two levers, which it may combine: lower the caps (C, l, L, or the model caps), or select a new M up to 6.0 GiB.
   - The second is the owner's decision of 2026-10-06. A new M is selected the way D-7 selects: by measurement, by the 0.9 M margin rule, recorded as a ruling, and with its unit's re-qualification. A new `threshold_bytes` in `REGISTERED_PROFILES` is part of B1's registration diff.
   - At 6,442,450,944 B the 0.9 M budget is 5,798,205,849 B: 2,202,717,115 B above today's dense E_mov,max + R (3,595,488,734 B), and 2,222,427,563 B above the sparse one.
   - **B0 selects no M and no cap values.**
6. **Owner-held:** M above 6.0 GiB, and any supported-machine statement of M (D-7 makes no RSS, stack, concurrency or machine claim).
   - With 28.4 MB of dense margin today and |A| candidates live at W3, c ≥ 2 at D1's model caps is unlikely to fit at today's M. So step 5's M lever is a probable path for B1 `[r01: N-10]`.
   - Whether 6.0 GiB suffices at ROOT's target caps is the study's measurement. Only if it does not is the owner asked (§9).
7. **The re-qualification obligations of a widening** (PLAN §2.1):
   - G5 profile, G6 maximum;
   - the S1 stack witnesses, including a multi-case witness and the re-based W6, W2 and W2b where §1.3 requires it `[r01: S-1]`;
   - the registration diff;
   - QUAL §11's carry: RV87's by-type non-candidate sweep, or an explicit-row rule, before re-registering.

## 7. Carried notes, each placed in the unit that owns it

| Unit | Note | Source |
|---|---|---|
| **B1** | W-C2 as in §1.4: three cases; a probe first; the Ceiling case is C; case B is the W2-published `not_required` pin. The two-body helpers are `u8_two_body_case_a` and `u8_two_body_case_b` in `PP/retained_facade_tests.rs`. W-C1 and W6 are re-based (§1.3) | PLAN §1.3; RR:12344; this design |
| **B1** | The probe's verdict audit of every QUAL §4 witness input and every `attempted_examples` input, before T-4 lands; re-base any that become `not_required` or excluded (§1.3) `[r01: S-1]` | RV105 S-1; ROOT's ruling |
| **B1** | D38's relaxation and pin (§2); F-1's rule, the G8 alignment and RS's `not_required` alignment, with their corpus (§3); one re-qualification for all three | RR:9117; RR:12344; RV105 B-1 |
| **B1** | T-3 (e)'s attempt fact counts requested cases (N-1). T-12's base-reader test extends to several notices (N-4). R-b′'s whole-invocation abandonment is a known limit (N-5) | RV105 N-1, N-4, N-5 |
| **B1** | RV97 R2-N-2: reword the module doc of `RE/tests/retained_precision_contract.rs` ("These do not establish execution" beside "listed producer-solved bases") | RR:12813 |
| **B1** | At the same touch of `PP/retained_facade_tests.rs`, I77's two code-line citations become symbols: `zz_rv93.rs:292–315` → `zz_rv93_input_fallbacks` (in `R/REVIEW_RV93/u3_grant2_01/evidence/probe/zz_rv93.rs`), and `retained_memory_witness_tests.rs :181–199` → `w6_input()` | RR:12898–12902 |
| **B1** | T6S consistency: if B1's re-pin changes the pinned successors, regenerate T6S-2's goldens and rerun the T6S suites. A changed disclosure meaning goes back to ROOT | I74 PLAN §4.3 |
| **PR-B1** | I76's item (b) with RV101 NT-1. RV95 N-5's direct `#[cfg(test)]` test in `RE/src/source_blocks.rs` calls `integer` directly at 2^53−1 (admitted), 2^53 and `u64::MAX` (both `SOURCE_BLOCKS_INTEGER`). It must not rely on the four fields whose schema maximum is tighter (case `work.limit` 4,000,000; `invocation_work.limit`, `.charged` and `.publication_charged` 64,000,000), which the schema refuses first. It also covers `failure.block_order` (the 13th field; RV101 A2-N3) and the composite physics-source receipt | RR:12552ff; RR:13164; I76 RETURN §4 |
| **B6** | Mutation 277 (`g7_not_required_quality_enum_invalid`) as a one-entry slice, together with N-3's G7 code alignment (TS aligns to `SOURCE_NUMERICAL_CASE_INVALID`; the Rust base validators unchanged) | RR:12697; RR:11246; PLAN decision 11 |
| **B6** | F-U6b-2: a transport validator for the retained successor in PY, matching RS's. PY keeps refusing transported successors and successor packages until then, as declared. TS's twin was closed in U6 (RR:10449) | RR:9922; RR:10449; I74 decision 8 |
| **B6** | RV78-N1's rehash-index rule, and RV94 N-5 (PY ignores `expected_by_reader`), as already routed. B6's corpus edits come before B1's snapshot | RR:9461; PLAN §2.1 |
| **B2** | C3a (§4). T6S: check stress-neutral `load_case_ref` and the AnalysisRun basis refs for combination cases on a successor, and rerun the T6S suites | I74 PLAN §4.3 |
| **B3** | §5, including the `legacy_pressure_v1` G8 widening and the successor-list precedent (N-8). T6S: the exhaustive output policy will not compile until B3 adds the route's entry (semantic table, `contract_evidence` kind, transport validator); until then the route refuses | I74 PLAN §4.3; RV105 N-8 |
| **S-I2** | D-U6-2's text "withheld from rule binding and reliance" becomes inaccurate for `absolute_verified` once intervals bind. One constant per language (Rust `class_disclosure`, the TS module, the stress-neutral finding) changes together. RV101 NT-9 joins it | I74 PLAN §4.3; RR:13165 |
| **B8 (owner-held)** | R-2: whether a successor's stress-neutral package may read ready while its D-U6-2 class rows carry withheld unit witnesses. Today's convention blocks it. Prepared for the owner, not decided | RR:12600ff |

## 8. Decisions

The owner-held list is the work graph's T3 section. None of the decisions below is owner-held except where marked; for those I prepare and do not decide. **[changed]** and **[new]** mark revision 01.

| # | Decision | Recommendation | Decider |
|---|---|---|---|
| 1 **[changed]** | The per-case trigger (T-4), keyed on the published verdict: `not_required` ⇔ `numerical_quality.cases[i].solve_quality == checks_passed`, for every c ≥ 1, including c = 1. The empty-A fallback is named `NoTriggeredCase` | **Adopt.** DN §4.3 and RR:2190 already rule that W1 runs only for Sensitive and D-5-routed cases. A W2-published case whose report passes resolved its Range error, so it is not triggered. Without the rule, one Passed case that selects (W2-published or not) destroys a multi-case successor (D6b), and an unavailable Passed case needlessly demotes the envelope. It changes the Direct entry's registered-build output for Passed one-case inputs, including W6's PHYS-R4-geometry input: no W1, and no notice. That is not public meaning, since there is no product caller, and PHYS-R4's publication and availability are unchanged. The owner is informed (ROOT's ruling) | ROOT |
| 2 **[changed]** | W-C2's construction, and the re-basing of the witnesses | **Three cases** (A selected; B W2-published `not_required`; C Ceiling unavailable), with C established by a B1 probe and the stop rule of §1.4. Re-base W-C1 and W6 on C alone. First audit the verdicts of every QUAL §4 witness (W2 and W2b included) and every `attempted_examples` input, and re-base any that become `not_required` (S-1) | ROOT |
| 3 | The native stage for c cases | **One `CaseBatchCall`** over the prepared sources of A, sharing the group cache (C2 §4) | ROOT |
| 4 **[changed]** | The preparation per case | **Per case, with each attempt recording its actual work** (C3 as written). No shared-preparation reference. A preparation-stage failure publishes `source_ref: null` and no `CaseSource` (S-2). The snapshot record point is the attempt's terminal stage, with no order among snapshots (S-3) | ROOT |
| 5 **[changed]** | Which failures abandon the whole successor | **T-6 custody, a T-8 call failure, no case selected, staging, every serializer failure, precommit.** No per-case `receipt_failure` demotion in B1. R-b′'s `recovery_demoted` on any case abandons the successor: a known B1 limit (N-5) | ROOT |
| 6 **[changed]** | The notices on a fallback | **One N1 notice per case in A,** in request order. The receipt-encoding detail is appended only for a serializer failure with a C1:68 detail or `publication_hash_range`, only on cases selected at abandonment, and it names the invocation's cause (N-4) | ROOT |
| 7 | D1.5 in B1 | **Keep it for every case** (one default material basis). Per-case modulus bases are a later widening | ROOT |
| 8 | F-1's rule form | **Text B** (P1–P4: at most one; none in sparse; none at b ≠ 0, by the typed `w2`), not Text A's biconditional. A false-rejects a faithful base when `solve_dense` fails | ROOT |
| 9 **[changed]** | B1's three-reader alignment | **G8:** every case; `RETAINED_PRECISION_PREPARATION_MISMATCH`; requested mode, then material basis, then P1, then P2–P4; TS drops mode code 3. **G5:** RS's `not_required` rule aligned to PY and TS (B-1). Corpus as §3.4, on the L = 0 bases (N-7) | ROOT |
| 10 | P5, OQ5's disclosure check of the mode row's text | **Defer.** It needs a three-language text grammar and adds no reliance | ROOT (observation framing itself is owner-held and does not change) |
| 11 **[changed]** | D38: the pin and the S-2 reading | **A labelled synthetic pin** (§2). A `CaseSource` with no Call is registered at construction, with digests from its own encodings. (4b) requires `case.source_ref == a.source_ref`. The test-hook shape (source withdrawn) stays refused by design. The pin keeps the shared group and A's builds (N-6). `serialize_unavailable` keeps refusing S-2 in B1 | ROOT |
| 12 | C3a's representation | **P1** (a separate `operand_preparations[]` and a second `CaseSource.preparation` branch). The new spellings (`operand_preparations`, `OperandPreparation`, `combination_operand`, `operand_preparation_failure`, `retained_precision_operand_preparation_v1`) are reserved by ROOT after a collision check. Mine at `b75069e6de` (outside `P/execution`) found none, except `combination_operand` as a substring of two unrelated Rust identifiers (`check_combination_operand_load_case` in `operation_applier`; `combination_operand_lengths` in FK `origins.rs`); neither is a wire value | ROOT |
| 13 | C3a's failure | **The combination is `retained_unavailable`** with the typed cause and a null `call_ref`. No notice on the owner case | ROOT |
| 14 **[changed]** | `physics-retained-1` | **Confirm the 2026-10-03 reservation** (full-form recheck clean); B3 inherits as in §5. physics-1's table is untouched, including its `reserved_inactive_successors`, following the retained precedent (N-8). B3's exact formation definition (suggested `RP-PREPARED-EXACT-DUAL-v1`) is named and reserved at B3's design | ROOT |
| 15 **[changed]** | B3's zero-pressure `legacy_pressure_v1` route | **Under `preview-physics-retained-1`,** with no new identity. It needs a D1.3 widening and a three-reader G8 widening for `pressure_contract {1.0.0, legacy_pressure_v1}` at zero pressure (N-8) | ROOT |
| 16 **[changed]** | The cap rows, the re-pricing, and M up to 6.0 GiB | **§6's scopes and method,** with per-attempted-case sources (N-11) and |A| live candidates at W3 (N-10). The cap values come from I65's study, and ROOT selects them. ROOT may also select a new M ≤ 6.0 GiB (6,442,450,944 B) under D-7, by measurement and the 0.9 M margin, as a ruling with re-registration (the owner's decision of 2026-10-06). This is the probable B1 path | ROOT |
| 17 **[changed]** | M above 6.0 GiB, or a supported-machine statement of M | **Prepared, not decided.** It is needed only if ROOT's target caps cannot fit under 0.9 M at 6.0 GiB (§6, step 6). The owner's decision of 2026-10-06 moved M ≤ 6.0 GiB to ROOT (decision 16) | **Owner** |
| 18 | R-2 for B8 | **Carried, not decided** (§7) | **Owner** |
| 19 | The native-app witnesses (G10's ordinary-route half; B8's native Current, including the panels) | **Unchanged by B0** | **Owner** |
| 20 | The placement of the carried notes | **As in §7** | ROOT |
| 21 **[new]** | DN §4.3's exclusion: a case whose ordinary attempt failed with Mechanism, Asymmetric or InvalidInput (the seed's `structural_failure` tag `mechanism`, `asymmetric` or `invalid_input`, with no W2 publication) | **Apply DN's exclusion:** such a case is not in A, gets no product attempt and no notice, and if A becomes empty the invocation takes `NoTriggeredCase` (exact bytes). Reasons: DN §4.3 says the trigger never fires for these. Their notice would claim that a recovery route was available. Such an envelope is blocked, so no successor is lost; only the notice changes. B1's probe records the tag for every committed attempted failure (§1.3). **Alternative:** keep D1's behaviour (a T-6 notice for every attempted failure), which changes no test (N-2) | ROOT |

**Owner-held items not touched:** dense and lane ceilings (F-1 changes neither guard); PHYS-R4's named refusal and availability (W-C2 uses PHYS-R4's body as a test input only; decision 1 stops a dev/test W1 attempt on W6's PHYS-R4 geometry, as RR:2190 already rules, and leaves PHYS-R4's publication unchanged); observation framing; KF3 and KF2; public meaning.

## 9. Effects on the breadth order and the owner's F2a order

**No finding changes the breadth order or the owner's F2a order.** The additions:
- **B1:**
  - the W-C2 probe step before W-C2's commit (about 2–3 h, U8-0's method);
  - the verdict audit of the QUAL §4 witnesses and `attempted_examples` (S-1; about 1–2 h, in the same probe);
  - re-basing W-C1, the W6 stack witness, and W2 or W2b if the audit requires it (about 2–5 h, inside B1's widening re-qualification);
  - RS's `not_required` alignment with its corpus pair (B-1; about 1–2 h, in the same three-reader change and re-qualification);
  - the G8 alignment beyond F-1 (§3.3), in the same change.
  
  **These, with the M selection record below, add about 7–14 h agent and 2–3 h review** to PLAN's B1 estimate (30–45 h agent and 10–14 h review), so about 37–59 h agent and 12–17 h review.
- **The memory contingency** `[r01: N-10]`.
  - At D1's model caps, c ≥ 2 is unlikely to fit under 0.9 M of today's M: there is 28.4 MB of dense margin, and |A| Runs and frozen candidates are live at W3.
  - Since the owner's decision of 2026-10-06, ROOT may select M up to 6.0 GiB without asking (§6, step 5). So the expected path is that B1's study proposes caps and an M ≤ 6.0 GiB, and ROOT selects both under D-7.
  - The owner is reached only if ROOT's target caps cannot fit under 0.9 M at 6.0 GiB, or if a supported-machine statement is wanted.
  - A new M is a re-registration item inside B1's widening re-qualification (G5, G6 and the registration diff, already planned). About 1–2 h of extra agent work for the selection record; no new gate.
  - **Contingency:** keep the case-C probe and the reader alignment independent of the cap and M choice, so they proceed while the study runs.
- **B2:** ROOT reserves C3a's names before implementation. The schema change is already a PR-B2 registration item.
- **B3:** its exact formation definition is designed, reviewed and reserved before implementation, RV69-style. The `legacy_pressure_v1` G8 widening (N-8) is in B3's three-reader change.
- **For ROOT, if decision 1 is not adopted:** multi-case successors would need either a D6b relaxation (DN §4.3, C1:101 and RR:2190 all say otherwise) or a producer rule that discards Passed-case selections. RV105 §2 confirms that neither is faithful.

## 10. What I read, and limits

**Read for revision 01:**
- RV105's REVIEW and its `evidence/` (`reader_rules.txt`, `probe_quality.log`, `corpus_bases.log`, `input_shas.log`, `collisions.txt`);
- ROOT's ruling, and RR "Owner decision: ROOT may raise M up to 6.0 GiB without asking", with the work graph's owner-held list as amended by it;
- RR:2186–2194;
- TS `ordinaryAttempts`;
- PP `retained_wire.rs` `ordinary_value` (`recovery_demoted`) and `retained_memory.rs` `ordinary_solve_attempted`;
- `retained_memory_witness_tests.rs` W2, W2b and W4, and `retained_memory_law_tests.rs` `attempted_examples`.

**Read for revision 1, unchanged since:**
- the brief; `AGENTS.md` and `agents/AGENT_TASK.md`; the work graph's T3 section;
- RR: 1270–1300, 6945–6990, 7040–7060, 7250–7270, 8032–8037, 8098–8132, 8415–8445, 8676–8835, 9005–9176, 9455–9465, 11240–11250, 11898–11966, 12344–12460, 12485–12633, 12690–12708, 12768–12913, 13144–13230;
- PLAN; CR; QUAL; DN §4.3–§4.6, §5 and §6; D2 §4.9.1–§4.9.2, §4.9.3's G8 row, §4.9.4's standing rows and §4.9.6;
- C1, C2 and C3 in full; DOMAIN §1–§4; PROBE §0–§4 and its `I68_ORDINARY` log lines; I74 PLAN §4.3; I76 RETURN §4; RV101's NT-1;
- CHECKS.json; the SCHEMA definitions named in revision 1; the three semantic tables' headers.

**Code read, unchanged at `b75069e6de`:**
- PP `lib.rs`: `W1Fallback`, `permitted_dispatch`, `permitted_run`, `ReservedNotice`, `w1_case_id`, `retained_w1`, `solve_preview_reduced_system`, `append_linear_solver_mode_evidence`, `append_sparse_live_path_evidence`, the observation-lane and dense guards, and the OQ5 sites;
- PP `retained_product.rs`: `solver_observations`, `bind_observations`, `prepared_case_seen`, `prepare_owned_case`, `solve_native`;
- PP `retained_wire.rs`: `successor_envelope`, `one_case`, `one_run`, `finish`, `bind_preparation`, `serialize_unavailable`, `ordinary_value`;
- PP `retained_memory.rs`: `caps`, `family_clauses`, `cap_rows`, `late_observations`, `complete_observations`, `ordinary_solve_attempted`;
- PP tests: `retained_facade_tests.rs` (U8's block), `retained_memory_witness_tests.rs`, `retained_memory_law_tests.rs`;
- RS `g0`, `g5_products`, `g8`, `rows_for`, `row_kind`; PY `_g5_stages`, `_g5_ordinary`, `_g5_products`, `_g8`; TS `productAttempts`, `ordinaryAttempts` and G8's case loop;
- FK `solve_dense` and `sparse_direct`'s pivot guard; `source_blocks::integer`.

**Input hashes (sha256):**

| File | sha256 |
|---|---|
| The brief | `86bdccd2…` (full hash above) |
| RV105 REVIEW | `d4807e9f…` (full hash above) |
| DESIGN.md (revision 1, kept) | `25a07a66…8a0c` |
| PLAN | `f274a614…3def` |
| CR | `f4207994…d476` |
| QUAL | `8edbf4b4…2c29` |
| DN | `fb62ef4a…7a74` |
| D2 | `993f5f3a…4c8d` |
| C1 | `c8ab2318…67e3` |
| C2 | `923da0b9…0869` |
| C3 | `fd00d2c1…292e` |
| I74 PLAN | `0350c918…2ed9` |
| PROBE | `ba5f7df6…6cfb` |
| DOMAIN | `08a72dde…c9fd` |
| CHECKS.json | `289bd974…b012` |
| U8_MERGE/RECORD.md | `9155d527…4a74` |
| SCHEMA | `07951eda…b61c` (equal to QUAL §5's reviewed input) |
| CORPUS | `5ac13296…2ccd` (RV105's `corpus_bases.log`) |

**Limits:**
- **Nothing was run.** Every producer outcome is quoted from a record (PROBE; QUAL; RV105's classification), or it is marked for B1's probe: W-C2's case C, the verdicts of W2, W2b and K2a's deferred formation, decision 21's error tags, and any other committed input that becomes `not_required` or excluded.
- **That two-body B and the one-body pair are W2-published** is RV105's inference from records and code. W6's is recorded in QUAL.
- **The `solve_dense` / sparse-lane divergence** (§3.1) is shown from code and pivot arithmetic, with no witness.
- **I did not audit** every reader check that might assume a prepared source has a Call or Run (§2, B1's obligation), nor every PP test outside the retained files.
- **The cap values and the profile arithmetic** are B1's study; §6 and §9 state the method and the expected direction only.
- **The C3a names, B3's definition name and `NoTriggeredCase`** are proposals. Collision checks (`git grep -F`, outside `P/execution`) were run at `b75069e6de`, with decision 12's two substring hits the only ones.
