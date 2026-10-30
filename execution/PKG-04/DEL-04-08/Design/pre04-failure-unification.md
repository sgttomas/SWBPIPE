# Source route: one failure behaviour for a selected source that cannot finalize

This design is the "Pre-0.4 route failure unification" unit of b7-plan's "After B7b", started by the owner on 2026-10-21 ("14 confirm, 15A, 16 start now."). Under 15A the finalization's full replay stays ([source-replay-bind-only.md](source-replay-bind-only.md), status line), so this unit keeps it and changes only what is published when finalization fails. The owner's earlier ruling (2026-10-21, "A accept your recommendations for the entire set.", item 8B) left R-A without this change and made unification its own unit ([source-work-nondyadic-inputs.md](source-work-nondyadic-inputs.md) §6).

**Status: as built (2026-10-22).** Implemented on branch `claude/piping-pre04-impl-20261021` (its PR states the final head), with the actual-source D3 in `retained-pricing-rules.md` §8.14. The owner approved §8's text on 2026-10-22 ("Decision 17 I approve option A."), and it is applied to the ScopeOfWork. Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

**As built, against this design.**
- **Method names.** The budget's fields became route-neutral (`join_withheld`, `join_failure`), but `record_load_state_join_failure` and `withholding_load_state_join` keep their names. The S3 record line is a basis-keyed pricing audit, which the line map cannot re-key through a rename.
- **The fail-closed form (2.2).** Before the request is re-derived, the first envelope is reduced to its fail-closed form (`fail_closed_form`). S1–S3's blocked envelope stays as it is; S4's source-labelled envelope keeps only its identity, so the dispatches would still refuse it as `Err`. That form is returned only if `requested` failed, which cannot happen after `parse` succeeded. It is dropped before the second run starts.
- **The Direct-entry test (section 6).** The registered build's D1 refuses pressure regions, so the pinned composite input is never admitted there. The test uses an invented PEXACT-A r1e-08 Timoshenko document, committed as `core/product_physics/tests/fixtures/pre04_failure_unification/pexact_a_r1e-08_zero_timo.request.json`: its bytes equal the shared route's republication, with two runs, no G-C and `Coexistence`.
- **Measured pins.** RF-14: c = 1,933,252 in both modes, declined at the screen at c + c/2 = 2,899,878, and republished at 2c = 3,866,504 with "captured source replay: … Exact(Budget)". At n05's 2c (357,072) its first failure is "composite recipe reservation", not the replay; the `mixed` and n06 tests run at other limits (500,000 and 357,071) and are not pinned at their 2c. The S2 test reproduces section 4's sub-windows at their edges.
- **D3** adds the republication phase X3 and prices the new cause texts (`retained-pricing-rules.md` §8.14, rules PU-*). All 80 evaluations fit. The tightest, merged dense W3, is 6,592,828 B below §8.13's.
- **The census on the implementation** equals the prototype's `screen` results field for field, on every set of section 5, and all 578 required triples select and pass.

Revision history of the design follows.

**Revision 2** answers round 1 of the dual review (Claude Opus 5.5 and Codex, on `ca65a51e9f`). Both found the rule sound, with repairs, and found no path that publishes a value from a source that did not finalize. The changes:
- **The screen (section 3).** The claim that the screen keeps earlier cases selected is withdrawn. A composite publishes only when every case qualifies (`validate_publication`), so a declined case beside a selected one always fails S4 (Opus). The recommendation stands on its other grounds, and its risk statement now rests on the non-replay finalization work.
- **The owner text (section 8).** REQ-003 (c) is restated whole. Its first sentence promises a receipt record that no exact-route publication can carry (Opus), and the new sentence promised a cause on every case, where only cases that attempt recovery carry one (both reviews). VER-003 also names the 0.4 republication tests.
- **The Direct entry (2.2).** Today's outcome is corrected: only S4 returns `Err`. The observer and its permit are released before the republication, and D3 prices the republication as its own phase (both reviews).
- **Section 2.1** names the population more broadly, including any pre-0.4 invocation with one selected and one declined case. It also covers a case whose ordinary attempt failed, and the diagnostic text kept from 0.4.
- **Section 4.** The reachability argument now covers budget refusals only, with the supports' own charges; a checked-norm refusal is reachable independently (Codex). The n05 window's two sub-windows are stated.
- **Corrections.** PEXACT-B's republication charge, the lib test count, the controls' denominator, the search description, and section 6's inventory: the `requested` accessor, `source_receipt.rs` and `source_recovery.rs` pins, and further texts.


**Revision 3** answers round 2 (Claude Opus 5.5 and Codex, on `43d9f22fe8`). Both confirmed every round-1 disposition and found no soundness defect. The wording repairs are to the summary, the n06 class (2.1), the cause population (cases declined without an attempt also carry it), REQ-003 (c)'s route phrase ("on either exact route" is undefined in the ScopeOfWork), §3's charge list and the controls' comparison count (Opus N1–N5; Codex R2-1, R2-2).
## Summary

- **Today** (section 1), a selected source that cannot finalize ends in one of two fail-closed forms on the pre-0.4 route:
  - a **blocked envelope** (`pressure-2` identity, `MODEL_INCOMPLETE`, no result rows) when the failure is in a case or in the captured source context;
  - a **run-level `Err("SOURCE_BLOCKS_FINALIZATION_FAILED")`** when it is in the invocation receipt.

  The 0.4 load/reference-state route republishes the invocation with every case on its ordinary route instead (ROOT CP3 SF-1R). Neither pre-0.4 form publishes an unproved bound, but neither gives REQ-003 (c)'s ordinary route or AX-005's "keeps its ordinary values and standing". Which form a many-case invocation takes depends on which reservation runs out first, so it moves as work shifts (R-A design §5.1). The population is wider than budget cliffs: any pre-0.4 invocation in which one case selects and another's attempt is declined fails the invocation receipt today, since a composite requires every case to qualify.
- **The single behaviour** (section 2). On both routes, a captured exact-route invocation whose selected source cannot finalize, at any finalization site, is republished on the same ledger with every case on its ordinary route (`pressure-2`, no receipt). The first failure is recorded as the cause, and every case whose attempt is declined or fails in the republication states it in its diagnostic. No published value comes from a source that did not finalize. This is SF-1R, extended to pre-0.4 and to the Direct entry. The run-level `Err` remains only as an invariant guard that no input reaches.
- **The screen** (section 3). Pre-0.4 gains the 0.4 route's replay screen, so both routes share one selection rule as well as one failure outcome. Measured, the screen costs no selection: the smallest margin over every selected case is 757,036. It saves the doomed attempts their republication: PEXACT-B charges 4.08–4.10M instead of 12.08–12.10M.
- **The misleading cause** (section 4). A support-norm recipe refusal leaves the support's rows unappended, and the row binding then reports "source functional row binding is not unique". No live reproducer was found on main, in the census sets or in 192 further invocations searched. A constructed control reaches it on `physics_source/n05` at private case limits 178,536–186,727. The repair records the support-norm refusal itself as the case's finalization cause, on both routes, and the row binding distinguishes a missing row from a duplicated one.
- **Measured** (section 5; scratch prototype, never merged; both solver modes).
  - All 24 blocked PEXACT runs and all 16 failing many-case invocations (32 runs) republish ordinarily. Their rows equal the typed route's bit for bit, and none carries a blocking diagnostic.
  - Every invocation that selects today selects with byte-identical bytes: the sweep, two-case, many-case and phi2 sets, the 35 controls and J7's 322 entries. All 578 required triples select and pass in every mode.
  - The constructed cliffs move as stated in section 7. No other test moves, except two structural custody checks that the Direct entry's republication must answer (section 2.2).
- **What moves** (section 6): the finalization arms of PP `lib.rs`, the pinned `the_pre04_composite_finalization_failure_is_unchanged_by_this_fallback` and RF-14's `pre04_selection_takes_no_replay_screen` (section 7, with reasons), RF §2.9 and the design and contract texts that quote it, and D3, since PP `lib.rs` is priced source.
- **Owner text** (section 8): REQ-003 (c) is restated whole. Its republication sentence generalizes from the load/reference-state group to every captured exact-route invocation, and its first sentence drops a receipt record that cannot occur. VER-003 names the republication tests of both routes.

## 0. Basis and terms

- **Basis.** Main `ca97f93f3b` (#1502). Paths are relative to `projects/chirality-piping/`. PP is `core/product_physics/src/`. Line numbers are on that main.
- **Prototype.** Branch `claude/piping-pre04-proto-20261021` on that main, never merged and with no PR (section 5). Its switch `PRE04_PROTO` selects main's behaviour (`off`), section 2's (`on`), or section 2's with the pre-0.4 screen (`screen`).
- **Tags.** **[M]** measured on the prototype, **[D]** derived from the code, **[C]** from a committed design.
- **Terms.**
  - *Selected source*: a case's `SelectedSourceRecovery` after its live attempt passes selection (PP `lib.rs:4980–5011`).
  - *Finalization*: everything from selection to an attached receipt. Its sites are S1–S4 (section 1.1).
  - *Republication*: a second run of the same captured invocation on the same `SourceRecoveryBudget`, with the join withheld, so that every successful attempt is declined (`SourceRecoveryBudget::withholding_load_state_join`, PP `lib.rs:966–978`; `SelectedSourceRecovery::decline_withheld`, PP `source_recovery.rs:284–293`).
  - *Screen*: SF-1's selection-time reservation `reserve_captured_replay`, which declines a case whose live charge c exceeds its case limit less c (PP `source_recovery.rs:268–279`).

## 1. Today [D]

### 1.1 Where a selected source can fail to finalize

| Site | Where | What fails | Scope |
|---|---|---|---|
| S1 | PP `lib.rs:3060–3072` | the captured request yields no composite context | invocation |
| S2 | PP `lib.rs:5543–5557` | a support's norm recipe (`composite_support_norms`: its `RECIPE_WORK` reservation or its checked norm) | case |
| S3 | PP `lib.rs:6590–6635` | row binding (`source_row_bindings`, 6672–6697), then `composite_exact`: its reservation, the captured rebuild and full replay, the binding check, row binding and the derived checks (bind-only design §1.1) | case |
| S4 | PP `lib.rs:3119–3137`; `source_receipt.rs:743–` | `finalize_composite`: the invocation publication reservations, the receipt policy, the composite publication reservation, `validate_publication` (including "composite requires source selection and complete case qualification", `source_receipt/composite.rs:1066–1073`), identity, coverage, the numerical-summary check and identifier uniqueness | invocation |

### 1.2 The pre-0.4 route

- **S1, S2 and S3** push a blocking diagnostic (`SOURCE_BLOCK_RECOVERY_FINALIZATION_FAILED`, or S2's `SOURCE_BLOCK_RECOVERY_DERIVED_UNAVAILABLE` followed by S3's). The case loop then returns `blocked_envelope` (PP `lib.rs:2923–2925`, 13963–14018): the ordinary `pressure-2` identity, `MODEL_INCOMPLETE`, empty physical evidence and no result rows. Later cases are not solved. Because the identity is ordinary, `source_finalization_failed` (2508–2511) does not fire, and the product entries publish this envelope (R-A design §1).
- **S4** leaves an envelope labelled `pressure-source-1` with no receipt. `ordinary_dispatch` (2489–2506) and `permitted_run` (3207–) then return `Err("SOURCE_BLOCKS_FINALIZATION_FAILED")`: no envelope at all.
- **A declined case beside a selected one is an S4 failure.** A declined or failed case enters the receipt as `FinalizedSourceBlockCase::failed`, with `qualified: false` (`source_receipt.rs:657–689`), and `validate_publication` refuses a composite with any unqualified case (`source_receipt/composite.rs:1066–1073`). So no exact-route publication carries a failed case: none of the 34 committed source receipts does. Any pre-0.4 invocation with one selected and one declined case returns `Err` today; the pinned test of section 7 is one.
- The pinned `the_pre04_composite_finalization_failure_is_unchanged_by_this_fallback` (PP `source_receipt/load_state_fallback_tests.rs:328–347`) asserts that `Err` on an S4 input, and calls it "Characterization only, not an endorsement … a separate open finding assigned to T3".
- **No screen.** Pre-0.4 selection does not reserve the replay (`lib.rs:4993`, "Pre-0.4 selection is unchanged"; RF §2.9; RF-14).

### 1.3 The 0.4 route (SF-1 and SF-1R)

- **Screen.** Selection first calls `reserve_captured_replay` (`lib.rs:4994–5011`). SF-1R states its role: "The replay reservation is a screen. … The guarantee is the fallback" (bind-only design §1.3).
- **Fallback.** S1, S3 and S4 call `record_load_state_join_failure` with "captured source context: …", "case <id>: …" or "invocation receipt: …"; the first cause is kept (`get_or_insert`, 979–981). `run_linear_static_preview_captured` (2513–2549) then runs the invocation again on the same ledger with the join withheld. Every successful attempt is declined at "invocation join withheld", and its `SOURCE_BLOCK_RECOVERY_UNAVAILABLE` diagnostic appends "; the invocation publishes every case on its ordinary route because its selected join could not finalize: <cause>" (5016–5020). A case's own refusal is reported as itself. The ledger is not refunded (CP4 review N-2), so the whole invocation, fallback included, stays within 64,000,000.
- **S2** records nothing itself. Its case then fails S3's row binding with "not unique", and that becomes the republication's stated cause (section 4).

### 1.4 Why the pre-0.4 form moves

A many-case invocation shares one ledger. Whether its first failure is a case's own replay (S3), a support-norm recipe (S2) or the composite or invocation publication reservation (S4) depends on how much earlier cases spent. So a work change, such as R-A, moves an invocation between the blocked envelope and the run-level `Err`: R-A's census moved ten one way and two the other (R-A design §5.1). The user-visible outcome of an input therefore depends on charging details that carry no engineering meaning.

### 1.5 Against the commitments

- **REQ-003 (c)**: "The case stays on its ordinary route, and the receipt records the failure stage and code." Its second sentence states the republication for "an invocation [that] carries a load/reference-state group whose join cannot finalize". Neither pre-0.4 form keeps the ordinary route.
- **AX-005**: "A case that cannot be verified keeps its ordinary values and standing." The blocked envelope drops them for every case of the invocation; the `Err` drops the envelope.
- **REQ-003 (e)**: nothing is relaxed in either form, and nothing would be under section 2.
- R-A design §1 records the gap: "So today these cases publish **no result rows**, not ordinary results. That is fail-closed, but it is not REQ-003 (c)'s ordinary route with a receipt failure record."

## 2. The single behaviour

### 2.1 The rule

**On both routes, a captured exact-route invocation whose selected source cannot finalize, at any of S1–S4, is republished on the same ledger with every case on its ordinary route (`pressure-2`, no receipt).**

- **The cause.** The first failure is recorded, with its site: "captured source context: …", "case <id>: …" (S2 and S3) or "invocation receipt: …". It is appended to the `SOURCE_BLOCK_RECOVERY_UNAVAILABLE` diagnostic of every case whose attempt is declined or fails in the republication, whether withheld, refused on its own, or declined without an attempt (a load-row formation finding or a range-formation decline, `lib.rs:4964–4975`), as on 0.4 today. A case that needs no recovery and has no such decline, or is not eligible, carries no such diagnostic. At least one case carries the cause: the case selected in the first run needs recovery again, since the ordinary solve is unchanged.
- **One residual, fail-closed.** In the selected path, a `?` return from the case solver (for example `require_finite_mechanics(vector)?`, `lib.rs:5529`) ends in `solver_blocked` without recording a failure. That is unreachable while the projections are finite, and is fail-closed if reached (Opus review, round 1).
- **Never a bound without its proof.** In the republication every successful attempt is declined (by the screen, the formation guard or the withheld join), so no case is selected, the invocation is not a composite, and no source projection, receipt or `pressure-source-1` label is published. The published rows are the ordinary route's rows (section 5 checks them against the typed route bit for bit).
- **Both pre-0.4 forms map to this one outcome.** An invocation that moves between S2, S3 and S4 as work shifts keeps the same outcome; only its stated cause moves.
- **The ledger.** The republication continues the first run's ledger, as SF-1R does. Its own attempts are charged again and may decline at `Budget` when little remains; they are declined either way. The invocation's total stays within 64,000,000, and no limit, reservation or gate changes (REQ-003 (e)).
- **A case whose ordinary attempt failed.** Where a selection replaced a rejected ordinary attempt (`lib.rs:5050–5061`), the republication has no selection to replace it, so the case's blocked ordinary result stands, as REQ-003 (d) states. The committed `physics_source/n06` requests are of this class: they finalize in the census controls, with `ordinary_attempt.outcome` "rejected" in their receipts, but no input the census republished is. The implementation forces a finalization failure on one to test it.
- **The diagnostic text.** The republication's text is 0.4's, byte for byte ("invocation join withheld"; "… because its selected join could not finalize: <cause>"). "Join" names the source composite on both routes (DEL-04-07 `pressure-contract.md`, D7), and one text keeps 0.4's committed bytes unchanged.
- **The run-level `Err`.** In a republication no case is selected, so `source_finalization_failed` cannot hold. The check stays in `ordinary_dispatch` and `permitted_run` as an invariant guard, failing closed if a defect ever reached it. Capture-parse refusals (`CapturedInvocation::parse`) are a different `Err` and are unchanged.

### 2.2 The Direct entry

`permitted_run` (PP `lib.rs:3207–`) runs the observed ordinary run itself, so SF-1R does not reach it today. In a pre-0.4 permitted invocation, an S4 failure returns `Err`; an S1–S3 failure publishes the blocked envelope and continues to G-B and G-C, where the observer records that the selected source suppressed its prepared source (`retained_product.rs:4974`). (No 0.4 invocation reaches `permitted_run`: `w1_route` gives load/reference-state models no W1 route, and they take `ordinary_dispatch`.)

Under the rule, every S1–S4 failure in `permitted_run` republishes in the same way on the same budget and records `W1Fallback::Coexistence`: W1 does not start, and the ordinary bytes are published with no notice (REQ-003 (a), "coexistence with retained-source recovery"). The implementation's Direct-entry test asserts that those bytes equal the shared route's republication of the same request.
- **Why not start W1 on the republication.** G-B and G-C observed the first run, in which a source was selected; the observer is not re-armed for a second run. Running W1 after a republication would be a coverage extension with its own pricing and review, outside this unit.
- **The republication's request.** The first run consumes and normalizes its typed request, so the republication needs another. On 0.4 today the wrapper clones the request before every run (`lib.rs:2526–2529`). The Direct entry forbids that: RV82 N9's single-parse custody test (`retained_facade_tests.rs`, `u3_n9_single_parse_custody`) requires the permitted path to move or borrow the parse's two halves, "never re-derived", and lists `request.clone()` among the forbidden texts. The prototype's clone fails that test, and the report-count test beside it (`u3_permitted_outputs_keep_the_report_and_gate_order`, 3 permitted outputs carrying the report), in every mode [M].
- **The rule here.** Both routes take the republication's request from the capture, through `CapturedInvocation::requested` (`source_receipt.rs:152–154`, made `pub(super)`), after the first run's envelope is reduced to its fail-closed form (dropped before the second run starts; see "As built") and, on the Direct entry, after the observer (`ProductCapture`, which owns the permit, its observations, sources and parked cases; `retained_product.rs:181–297`) is dropped too. The prototype kept both alive; the implementation does not. That is the deserialization `parse` performs (`serde_json::from_value` of the same custody bytes, 131–132), and finalization's captured rebuild already calls it within the same invocation. So:
  - a run that finalizes holds no copy of its request; on 0.4 that removes today's up-front clone;
  - a republication never holds two runs' state, nor the first run's observer;
  - the request that runs is still the parse's value, and W1, the only consumer of the custody pairing, does not start on a republication.
- **The custody checks change, with reasons.** `u3_n9_single_parse_custody` keeps every forbidden text and gains one stated exception: exactly one `capture.requested()` in the permitted section, in the republication arm that ends in `W1Fallback::Coexistence`. `u3_permitted_outputs_keep_the_report_and_gate_order` counts four permitted outputs carrying the report, the fourth being the republication. The reason is that the Direct entry must republish, and of the ways to obtain a second typed request this is the one that adds no copy to a finalizing run and no parse of anything but the custody bytes. If `requested` failed, which it cannot after `parse` succeeded on the same bytes, the invocation keeps today's fail-closed form.
- **D3.** PP `lib.rs`, `source_receipt.rs` (full-file guard, `retained-pricing-rules.md` §7.1) and `source_recovery.rs` (pinned by `b7a_formation_model`) are priced source. The registered profile prices one observed ordinary run per permitted invocation, so the D3 prices the republication as its own phase in the phase model, not only by re-pinning the changed files, and registers under `retained-pricing-rules.md` §8 with independent review.

### 2.3 What does not change

- The full replay, every limit, reservation, gate and charge (15A; REQ-003 (e)).
- Selection, except as section 3 decides for the screen.
- Every invocation that finalizes: its envelope and receipt bytes (section 5).
- The 0.4 route's behaviour, except: an S2 refusal's stated cause, and an absent bound row reading "missing" instead of "not unique" (section 4; after the S2 repair no product path leaves a row absent, so "missing" would mark a defect); and the up-front request clone, which goes (2.2).
- The readers, schemas and published formats. A republished envelope is an ordinary `pressure-2` envelope, which the readers already accept.

## 3. The screen

**Recommendation: (b), one screen on both routes.**
- **(a) Pre-0.4 has no screen, as today.** The routes share one failure outcome and differ in one selection rule.
- **(b) Pre-0.4 gains the 0.4 screen.** One selection rule on both routes. A case whose live charge exceeds half its case limit is declined at selection instead of failing finalization later. The invocation's outcome is ordinary either way: a declined case beside a selected one also fails S4, since a composite requires every case to qualify (1.2), so the screen does not keep other cases selected. A case that would have finalized within its limit although its live charge exceeds half of it would lose selection.
- **(c) Neither route screens.** Changes recorded 0.4 behaviour and its committed tests (`reviewer_scenario_a_…`, the headless SF-1 cliff), for no measured gain.

**Measured [M]** (section 5):
- **No lost selection.** Over every case that selects today, in both solver modes, the screen margin (case limit − 2c) is positive. The smallest is 757,036 (phi2, c = 3,621,482). The sets' minima are 2,889,230 (controls), 1,447,884 (sweep), 1,444,416 (two-case) and 1,604,132 (many-case). J7 and all 578 required triples are identical in `on` and `screen`.
- **Earlier, cheaper failure.** PEXACT-B Timoshenko (16 runs) is declined at the screen, c = 4,075,078–4,102,647, for a whole charge of 4.08–4.10M. Without the screen it selects, fails at "captured source replay … Exact(Budget)" and republishes, charging 12,075,003–12,102,582 over two attempts. PEXACT-A r1e-08 (c ≈ 3.737M, margin +526,494) passes the screen and republishes under both.
- **Many-case.** On the four many-case invocations whose case 10 fails its replay, the screen declines case 10 (c = 2.82–3.07M against case limits of 4.31–5.39M). The invocation receipt then fails, here at the composite publication reservation, and the invocation republishes, as without the screen.

**Why (b).**
- The two routes then behave identically: one selection rule and one failure outcome. SF-1R's own statement of the pair, "The replay reservation is a screen. … The guarantee is the fallback", holds on both.
- Declining at selection gives the same ordinary outcome as a later finalization failure, with about a third of the work on PEXACT-B, and a cause that names the reservation.
- **The risk.** The screen reserves an amount equal to c, but finalization's cost is not c. The full replay can cost slightly less than the live attempt (bind-only design §3.3: a complete replay of 4,056,554 against c = 4,102,647 on PEXACT-B r1e-08), while finalization also charges its own reservation, the binding check, row binding and derived checks on the case ledger (RF §2.9, measured before R-A on PIN-OFF-345: finalization charged 3.33M and 3.21M, beyond finalization-entry charges of 2.90M and 2.78M, which include the live attempt, so finalization exceeded c). On every measured selection the non-replay work makes finalization cost more than c: over published single-case selections, finalization less c is at least 468,591 (n06), 553,583 (sweep) and 558,387 (phi2) (Opus review, round 2). That bounds the margin from below, which is why no margin is close to zero. A case whose whole finalization cost less than its live charge, with c above half its limit, would lose selection under the screen. None is measured, and such a decline is fail-closed: the invocation keeps its ordinary route and the case's diagnostic states the screen.

RF-14's pin moves accordingly (section 7).

## 4. The misleading cause

**Mechanism [D].** When `composite_support_norms` refuses (PP `lib.rs:5543–5557`), the case pushes the blocking `SOURCE_BLOCK_RECOVERY_DERIVED_UNAVAILABLE` without appending that support's signed result rows. At S3, `source_row_bindings` looks up each retained functional's row; the support's `support_reaction_component_v2` rows are absent, and its test `matched.len() != 1` (6693) reports "source functional row binding is not unique: <support>/support_reaction_component_v2/node". The outcome is fail-closed, but the stated cause hides the `Budget` refusal. The path is in the case solver both routes share; on 0.4 the same text becomes the republication's cause (R-A design §5.1).

**Reproducer [M].**
- **No live reproducer found on main.** "not unique" occurs in none of the census sets of section 5. A bounded search found none in 192 further invocations, sparse mode: the 12 decimal Z-hinge structures at 11 to 14 cases with the later cases' moments scaled by 1 + i/10, and 10-case and 2-case invocations scaled by 1 + i/k for k in {20, 7, 5, 4, 3, 13}. R-A §5.1's witness remains historical (bind-only design §7).
- **Constructed control.** The committed `fixtures/product_preview/pressure_source/physics_source/n05.request.json`, run through `run_linear_static_preview_captured` with a private `SourceRecoveryBudget { per_case_limit }`; its live charge is c = 178,536 in both modes.
  - Limits 178,536–182,631: both the `anchor` and `independent-spring` support-norm reservations refuse. 182,632–186,727: only `independent-spring` refuses. Below 178,536 the live attempt is declined; above 186,727 finalization fails at a different stage. Nothing is charged between selection and the first support norm (charged = c at the refusal).
  - In the lower window, main (`off`): a blocked envelope with no rows; two blocking `SOURCE_BLOCK_RECOVERY_DERIVED_UNAVAILABLE` ("composite recipe reservation: RecoveryFailure { stage: "receipt finalization", … Exact(Budget) … }"), then `SOURCE_BLOCK_RECOVERY_FINALIZATION_FAILED` "source functional row binding is not unique: anchor/support_reaction_component_v2/node".
  - In the lower window, with the repair and without the screen (`on`): an ordinary republication (133 rows, two attempts, charged 357,072) whose cause reads "case case: support anchor norm: composite recipe reservation: RecoveryFailure { … Exact(Budget) … }". Neither "not unique" nor "missing" appears.
  - With the screen, every limit in that window is below 2c = 357,072, so the case is declined at selection and S2 is not reached; the outcome is the same ordinary publication, with the screen as its cause.
- **Reachability under section 3.** S2 has two kinds of refusal.
  - *Budget.* A screened selection leaves at least c of its case ledger. The support norms run before the member-maximum recipes and `composite_exact` (`lib.rs:5546`), and each charges `RECIPE_WORK` (4,096) plus its checks. So a budget refusal at S2 needs the supports' cumulative recipe charges to exceed c, which no measured input approaches.
  - *Checked norm.* A norm's own checks can refuse independently of work: its representative or publication range, or its protected criterion (`source_receipt/composite.rs:278–303`, 368–381). The screen does not prevent that.

  The repair is kept on both routes, so that either kind is reported as itself and, under section 2, republishes.
- **Its test.** The n05 control needs the screen out of the way. The maintained test uses a `#[cfg(test)]` switch that skips the screen for its own thread only, as the private `SourceRecoveryBudget` limits are test-only (REQ-003 (e): overrides only in `#[cfg(test)]` code). No product build can set it.

**Repair.**
- The support-norm refusal is recorded as the case's finalization cause, "support <id> norm: <error>", and the case fails at S3 with it, without consulting the row binding.
- `source_row_bindings` reports an absent row as "source functional row binding is missing: …" and keeps "not unique" for more than one match. A missing row then never reads as a duplicate.
- Both routes take the repair. On 0.4 it changes only the stated cause of a republication that S2 triggers.

## 5. Measurements [M]

**Prototype.** Branch `claude/piping-pre04-proto-20261021` at `8c333cf547` (prototype `9840f9028b`, a test-only attempt probe, the census harness and two mover probes; all census code `#[ignore]`), on main `ca97f93f3b`. Built at the hosted-CI product_physics profile; the runner and the declared-problem test binary built from the same tree. Every figure below covers both solver modes, except that three O4 controls are sparse-only.

| Inputs (runs) | `off` (main) | `on` | `screen` |
|---|---|---|---|
| phi2 twins, all variants (416) | 360 selected, 28 ordinary, 28 blocked | 360 selected, 52 ordinary, 4 blocked | as `on` |
| many-case, 36 invocations of 8–10 cases (72) | 40 selected, 24 `Err`, 8 blocked | 40 selected, 32 ordinary | as `on` |
| single-member sweep (960) | all selected | all selected | all selected |
| two-case, EB and Timoshenko (240) | all selected | all selected | all selected |
| committed controls (35 mode entries over 19 requests) | all selected; equal to committed bytes where committed (29 of 35: the 24 raw controls, and the 5 O4 entries through the O4 document) | identical to `off` | identical to `off` |
| J7 (322 entries) | equal to its pins | equal to its pins | equal to its pins |
| required triples, `j2`/`rf`/`judged`/`phi2` (578) | all select and pass | all select and pass | all select and pass |

- **phi2.** The 24 runs blocked today are PEXACT-A r1e-08 and both PEXACT-B, Timoshenko, failing at "captured source replay … expected invocation replay … Exact(Budget)" (charged 7,999,869–7,999,989). All 24 republish ordinarily. The 4 runs blocked in every mode are `P122-RF-CONTROL-r1e-12` (dense; Euler–Bernoulli; zero, ca, mill and both allowances): their ordinary route itself is blocked ("NumericallyUnresolved … structural pivot"), so they are outside this unit. Through the runner the PEXACT 24 move from `MODEL_INCOMPLETE` to `MECHANICS_SOLVED`.
- **Many-case.** Per solver mode, the 16 failing invocations are as the bind-only census measured: 8 `Err` at the composite publication reservation, 4 `Err` at the invocation publication reservation and 4 blocked at case 10's replay. All republish ordinarily. Their ledgers end at 63,998,887–63,999,998 over 18–20 attempts. The republication's attempts meet an almost spent ledger, so most decline at "exact context/source closure … Exact(Budget)" with the withheld cause appended, rather than at "invocation join withheld". Either way they are declined.
- **Republications.** Every ordinary outcome that main did not publish (24 phi2 and 32 many-case runs, and the pinned composite input in both modes) has rows equal to the typed route's (`run_linear_static_preview_with_mode`: id, kind, value bits, unit), no blocking diagnostic, numerical quality `sensitive`, and the Rust reader's `pressure` verdict ok.
- **Preservation.** Every input selected under `off` is selected under `on` with byte-identical JSON, and under `screen` too. No selection is lost (section 3's margins).
- **Existing tests.** Under `off` the product_physics lib (988 tests, 96 ignored) passes except the two custody checks of section 2.2, which the prototype's Direct-entry clone breaks in every mode. Under `on` and `screen` the two pinned tests of section 7 also move, and nothing else: the 37 integration test files, the doc tests and the headless `load_reference_route_tests` and `load_reference_cli` pass in all three modes, and s11g T20 does not move.
- **Jobs.** All through `tools/t3_slot.sh`, from START 2026-10-21T22:56:46Z to END 23:23:14Z in `/Users/ryan/dev/chirality-t3/guard/cargo_jobs.log`: 43 jobs, 39 rc=0. The four rc=101 are a phi2 list run with relative paths (22:59:58Z, rerun clean) and the three lib-test runs whose failures are listed above (23:08:24Z, 23:13:24Z, 23:17:58Z). Raw output is in host scratch, not committed: `scratch/pre04-proto/run1/`.
- **Reviewed.** Both round-1 reviewers re-aggregated these figures from the raw outputs. Codex also compared the saved runner bytes of the 360 selected phi2, 32 `rf` and 18 `judged` outputs: unchanged under `on` and `screen`.
- **Not run.** The Python verifier on the republished envelopes (they carry no receipt); a comparison of the republications' `numerical_quality` with the typed route's beyond its status; MJ9's accounting as its own check (covered by byte equality); the misleading-cause search in dense mode; and the registered build's Direct entry, which no census entry reaches. The implementation's tests cover the last where the registered build allows (section 6).

## 6. What moves

**Product code (PP `lib.rs`).**
- `run_linear_static_preview_captured`: the republication covers every captured exact-route invocation and takes its request from the capture (2.2); the up-front clone goes.
- S1, S3 and S4 record the failure on both routes; S2 records its own cause (section 4).
- Selection screens and then declines a successful attempt when the join is withheld, on both routes, and appends the withheld cause on both (4991–5020). Pre-0.4 gains the screen (section 3).
- `permitted_run` republishes (2.2).
- `source_row_bindings` distinguishes a missing row (section 4).
- PP `source_receipt.rs`: `CapturedInvocation::requested` becomes `pub(super)`.
- PP `source_recovery.rs`: the screen's doc comment ("resolved … cases only", 258–267) and `decline_withheld`'s, made route-neutral.
- Naming: `load_state_join_withheld` and `load_state_join_failure` become route-neutral, with their doc comments.

**Tests** (section 7 for the two pinned ones and the two custody checks).
- New, in both modes, each republishing ordinarily with rows equal to the typed route's:
  - a pre-0.4 S3 failure (per case) and an S4 failure (invocation);
  - an S2 failure with its repaired cause;
  - a mixed invocation with a case that needs no recovery and has no load-row finding, which carries no cause diagnostic;
  - a case whose ordinary attempt failed (`physics_source/n06` class), whose blocked ordinary result stands (2.1);
  - a Direct-entry republication, in the registered build, byte-equal to the shared route's.
- Moved, with reasons: the two pinned tests (section 7) and the two custody checks (section 2.2). No other committed test moves [M].
- The cause repair is tested at the n05 control's window with the screen skipped by a `#[cfg(test)]` thread-local switch (section 4); the switch exists only in test builds and changes no product limit or gate.

**Design and contract text.**
- RF §2.9, §5.2 (6), §8 RF-14 and §11's "no new publication rule" item.
- DEL-04-07 `pressure-contract.md` lines 24 ("the pre-0.4 path retains its named refusal") and 48 ("Failed source admission or finalization remains a named refusal").
- R-A design §1 and §6 (including the owner-decision line "The PEXACT φ = 2 documents and the over-limit many-case invocations keep their fail-closed outcome", annotated, not rewritten), bind-only design §1.1 and §7, b7-plan's "After B7b" item: status notes pointing here.
- `pressure-contract.md:24` sits under "D7, HELP_HUMAN 2026-10-10": its sentence is revised with a note that this unit supersedes it.
- The FK doc comment at `exact_boundary.rs:340–350`, and the PP comments at `lib.rs:4993`, `lib.rs:14452–14461` (K-D5 F3's disposition) and in `s11g_tests.rs` T20, which name the pre-0.4 refusal.
- DEL-04-08 `deliverable.yaml`'s DEL-04-07 need: the ordinary exact route's republication.

**D3.** PP `lib.rs`, `source_receipt.rs` and `source_recovery.rs` are priced. The implementation re-prices with `tools/retained_pricing`, with the republication as its own phase (2.2), and registers under `retained-pricing-rules.md` §8, with independent review. No reviewed input (`REVIEWED_INPUTS`) moves.

**Unchanged.** Limits, the replay, schemas, readers, the Python verifier, J7's expected classification and every frozen reference.

## 7. The pinned tests

**`the_pre04_composite_finalization_failure_is_unchanged_by_this_fallback`** (PP `source_receipt/load_state_fallback_tests.rs:328`).
- *Today* it asserts `Err("SOURCE_BLOCKS_FINALIZATION_FAILED")` for the committed `physics_source/mixed` request with case 1's `modulus_basis_ref` removed, an S4 failure ("composite requires source selection and complete case qualification").
- *Proposed*: renamed `a_pre04_composite_finalization_failure_republishes_ordinarily`. It asserts the ordinary publication: no receipt, the `pressure-2` identity, `MECHANICS_SOLVED`, no blocking diagnostic, rows bit-equal to the typed route's in both modes, and the withheld cause "invocation receipt: composite requires source selection and complete case qualification" in the selected case's diagnostic.
- *Reason for the change.* The expected `Err` was recorded as a characterization, "not an endorsement", of an outcome left open; it does not meet REQ-003 (c) or AX-005 (1.5); and the owner commissioned one failure behaviour for both routes (b7-plan, "After B7b"; "16 start now."). The new expectation is the outcome REQ-003 (c) already states for 0.4. No limit, criterion or reference moves to obtain it.

**RF-14, `pre04_selection_takes_no_replay_screen`** (PP `source_recovery.rs:3070–3091`).
- *Today*: at a private case limit c + c/2, a pre-0.4 congruence selection is not declined at the screen; it selects, and its finalization takes the blocking `SOURCE_BLOCK_RECOVERY_FINALIZATION_FAILED`.
- *Proposed*: renamed `pre04_selection_takes_the_replay_screen`. At the same limit c + c/2 the case is declined at "captured replay reservation" with `Exact(Budget)`, nothing is selected, one attempt is charged, and the invocation publishes ordinarily with rows equal to the typed route's [M: declined at "captured replay reservation … charged: 1933252, rejected: 1933252, limit: 2899878"]. A second assertion, at a limit that passes the screen but cannot fit finalization (2c is expected to be one, since finalization repeats the live work; the implementation measures and pins it), reaches the republication with the cause "case case: captured source replay: … Exact(Budget)" [M: under `on` at c + c/2 that cause reads "… charged: 2899853, rejected: 97, limit: 2899878"].
- *Reason.* RF-14 pinned the asymmetry this unit removes ("Pre-0.4 publication behaviour changed (for example a replay screen added)"). Its mutant row is restated, not removed: "The routes' selection or failure behaviour diverge (a screen or a republication missing on either route)", killed by the restated test and by the 0.4 tests that already pin both.

**RF §2.9 (change, with reasons).** Its "No new publication rule" paragraph is replaced by: both routes screen selection by the captured replay's reservation and republish ordinarily when a selected source still cannot finalize (this design, sections 2 and 3). Its table stands: those cases finalize within 8,000,000. The reason is the commissioned unification; §2.9 recorded the asymmetry as the state then, and RF made no claim for it beyond "the publication paths stay as they are".

## 8. Owner-reserved text

These are ScopeOfWork changes, the owner's to approve. The implementation PR applies only approved text.

**REQ-003 (c)**, whole.
- Old: "**(c) Retained-source attempt declined or failed.** The case stays on its ordinary route, and the receipt records the failure stage and code (for example `unsupported_family`). If an invocation carries a load/reference-state group whose join cannot finalize, every case is republished on its ordinary route (`pressure-2`) without a receipt."
- New: "**(c) Retained-source attempt declined or failed.** The case stays on its ordinary route, and its `SOURCE_BLOCK_RECOVERY_UNAVAILABLE` diagnostic states the failure stage and cause. A `pressure-source-1` publication requires every case to qualify. If a captured invocation's selected source cannot finalize, whether or not the invocation carries a load/reference-state group, every case is republished on its ordinary route (`pressure-2`) without a receipt, and each case whose retained-source attempt is declined or fails in the republication states the first finalization failure in that diagnostic."
- *Why the first sentence changes too.* Its receipt record cannot occur: a declined or failed case is unqualified, and a composite with an unqualified case is refused (1.2). So today the sentence describes no published outcome, and after this unit its case is the republication. If the owner prefers to keep the first sentence, only the second and third need to change for this unit; the first would then remain inaccurate.

**VER-003**, its list gains, after the `b7b_admission_tests.rs` item: "`core/product_physics/src/source_receipt/load_state_fallback_tests.rs`, both routes' republication: on 0.4, `a_selected_case_whose_own_finalization_fails_after_the_reservation_falls_back` and `reviewer_scenario_b_composite_with_sensitive_pressure_case_publishes_the_ordinary_route`; on pre-0.4, `a_pre04_composite_finalization_failure_republishes_ordinarily`, `a_pre04_case_finalization_failure_republishes_ordinarily` and `a_support_norm_refusal_is_the_stated_cause`; and `pre04_selection_takes_the_replay_screen` in `core/product_physics/src/source_recovery.rs`;". The implementation may add the remaining new tests of section 6 to the same item, under their final names.

AC-002, AC-003, AX-005 and REQ-003 (a), (b), (d) and (e) need no change (both reviews concur). VER-004's registered-branch checks cover the Direct entry's D3.

## 9. Verification and review

- **This design.** Dual review on the same revision, by an independent Claude Opus 5.5 reviewer and an independent Codex reviewer, each also scrutinizing section 2's claim that no unproved bound is published and section 5's figures. Rounds and dispositions are recorded in the PR that carries this file.
- **The implementation.**
  - the new tests of section 6 and the restated pins of section 7, in both modes;
  - the census inputs of section 5 on the implementation, with every finalizing invocation byte-identical to main;
  - VER-002's checks and the J7 census, unchanged;
  - the actual-source D3;
  - the independent numerical review that Root `AGENTS.md` and AX-006 require, on the final head.
