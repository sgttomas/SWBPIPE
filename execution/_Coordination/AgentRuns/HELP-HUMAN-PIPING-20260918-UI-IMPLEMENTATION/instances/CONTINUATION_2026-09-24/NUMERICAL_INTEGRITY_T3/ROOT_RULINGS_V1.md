# ROOT's early rulings on the V1 review

HELP_HUMAN (ROOT), 2026-09-26, relayed to the T3 manager by SendMessage and recorded here by the manager. Inputs: [REVIEW/RETURN.md](REVIEW/RETURN.md) (verdict BLOCKING) and the manager's [proposed dispositions](MANAGER_NOTES/V1_DISPOSITIONS.md). These rulings let the revisions start; ROOT rules the rest in the full disposition package.

1. **V1-B1 is fixed in D1 revision 2, and stays BLOCKING until V1 backchecks it.**
   - Sum every source-level contribution exactly and round once: loads, and combinations over retained states.
   - Put combination outputs under the stop rule.
   - Add V1's cancellation case and a combination case as negative controls.
   - Withdraw the §3.1 claim.
   - V1-S8: either prove that 1e-9 holds for every published quantity, or state and enforce the floor, for example by withholding quantities below it.
2. **V1-S11 is a silent-wrong path live on main.** T3 owns it at top priority.
   - Map it now: its exact reach, the routes, and a realistic-magnitude example as well as the extreme one.
   - Propose containment that can land early as its own small slice, before any general-method work. For example, detect input cancellation beyond binary64 and mark the result Sensitive, or refuse it.
   - If the containment touches only files T1 does not change, it may land before T1 merges.
   - R1, or an addendum to it, needs a reference case.
3. **Interface.**
   - The package must carry a concrete recommendation for when `load-reference-source-1` stops being fresh; ROOT rules on it there.
   - Adopted now: V1's split of S-E into S-E1 (the re-derivation core, kept) and S-E2 (the joined standing wiring, conditional).
   - Adopted now: one shared retirement gate for D1's I-3 and D2's R-B, covering coverage, budgets, standing in all three languages, and value agreement (V1-S2).
   - Name an owner for the successor-identity readers (V1-S3).
4. **V1-S5, capture refusal of |x| ≥ 2^53.** It makes a 1e16 N/m spring unsolvable on every route. It is a refusal, not a silent wrong, but it is a real capability hole. T3 owns the capture step; T6 owns the export carriers. D2 corrects its PR-5 predictions.
5. **The remaining SHOULD-FIXes (S4, S6, S7, S9, S10) go into the revisions.** For S6, prefer a reader rule that cannot diverge across languages, such as refusing when a host-exp field is used, over a one-ulp tolerance that depends on the host.

## Manager's application

- **Successor-identity readers (V1-S3):** D2 owns them, as the manager proposed (dispositions R-5).
- **The shared retirement gate:** D1 defines it, and D2 cites it (dispositions R-4).
- **S11:** mapped in [MANAGER_NOTES/S11_MAP.md](MANAGER_NOTES/S11_MAP.md). D1 designs the containment slice as the first deliverable of revision 2. R1 gets the RF-CANCEL addendum ([TASK_BRIEFS/R1_ADDENDUM_CANCEL.md](TASK_BRIEFS/R1_ADDENDUM_CANCEL.md)).
- **Still open for ROOT in the package:** the fate of the joined identity (dispositions R-3 (a)); the eligibility of historical physics-source-1 (R-3 (b)); historical all-selected source-blocks-1 (R-7).

## Further rulings (ROOT, 2026-09-26, after `065c9ff60`)

ROOT gave these rulings so the revisions do not wait for the package.

- **R-3(a): accepted.** `load-reference-source-1` stays fresh until D1's F3 (W1b) lands, and stops being fresh when F3 lands. S-E1 is built alongside F3. S-E2 is built only if F3 will not land within T3. D2's revision schedules S-E2 as conditional on that, and records the condition.
- **R-3(b): yes.** Historical physics-source-1 stays eligible. It has no known defect, and retiring it for fresh solves does not change the standing of results that were solved correctly. A defect found later by V1, P1 or the references reopens this ruling.
- **R-7: option (i).** Historical all-selected source-blocks-1 stays Current, with the notice and the summary rule-binding refusal, exactly as T0R ruled in R-2. The abs-sum summary is the only known-defective quantity, and it is already blocked from reliance. The other quantities are correct, so demoting them would withhold correct results with no correctness gain. D2's revision adopts (i) and records why (ii) is not taken.
- **S11 containment.** Whichever option is chosen must fail closed, per case, wherever it interacts with `source_recovery`'s bit-equality check. Knocking out correct retained-source recovery is acceptable only as a declared, temporary refusal, never as a silent loss of a result. The manager brings D1's `S11_CONTAINMENT.md` to ROOT as soon as it lands.
- R1's agent name is `a61ebde0db2f338eb`. ROOT has forwarded the RF-CANCEL addendum to it.

## S11 containment (ROOT, 2026-09-26, after `70f56b83e`)

- **The V1 check is authorized.** The manager sends `S11_CONTAINMENT.md` to V1 directly, read-only and with standard-library probes only. The questions: whether any value can be silently wrong, including through a producer that bypasses the ledger; whether the screen catches every absorption that breaches 1e-9 on published quantities; whether the source-recovery and receipt comparisons stay bit-consistent; whether restrained-DOF reactions and the nonlinear loop are covered; and whether the "three or more contributions" narrowing holds for realistic models (two adjacent element-load equivalents plus a nodal load on a shared node).
- **Pre-accepted, subject to V1 returning CLEAR on S11, or with its findings fixed and backchecked:**
  - C3-full, split into S11-K and S11-F. S11-K lands dormant as soon as the host is released, serialized with D1's K2 and K5 on `SA`. S11-F is the first facade slice after T1 merges.
  - No interim measure before T1 merges. Exposure needs three or more contributions and a gross-to-net ratio of about 1e7 or more; nothing committed has that, and T1 is in final qualification. **This ruling reopens if T1's merge slips materially.**
  - The envelope-level Sensitive side effect on the guard path is accepted. It is recorded as open work for T6, since case-scoped standing belongs to T6's result semantics.
  - The conservative flag (for example the 1e5-to-0.3 row) is accepted: flagging Sensitive when a result might be accurate is the correct direction.
- ROOT records the selection once V1 returns.

## Rulings on V1's S11 check (ROOT, 2026-09-26, after `56b651282`)

V1's check of `S11_CONTAINMENT.md` ([REVIEW/S11_CHECK.md](REVIEW/S11_CHECK.md)) is BLOCKING. **The pre-acceptance of C3-full above is suspended** until D1 revises S11 and V1 backchecks it CLEAR.

1. **S11-V1 (BLOCKING).** Exact summation extends to every post-solve recovery sum that folds element loads: element end forces (K_e·u − equivalent), station resultants, the stress-maximum sums and curved-bend intensities. All of them use the same exact accumulation and rounding function. The `straight_pipe` part is T1-disjoint and joins S11-K. Probe A becomes a test, alongside a mutation that restores the binary64 fold and must fail. **Invariant:** the containment never makes a quantity newly silent that C3-detect would have flagged; this is stated and tested on probe A.
2. **S11-V2 and S11-V3.** Ledger bypass is impossible by construction. The force vector can only be built from the ledger, and a test enumerates every force-accumulation site, so a new producer that skips the ledger fails. The ledger's contribution granularity is defined so that no producer pre-sums before pushing. The screen's floor is stated and recorded (as for V1-S8). The guard path is defence in depth, not the primary correctness mechanism.
3. **S11-V4.**
   - All T1 sites are named: `source_recovery.rs:609-667` including the eigen fold, `source_receipt.rs:218-219`, and `source_receipt.rs:320`. Missing `:320` would reintroduce the forbidden finalization Err.
   - The rounding is correctly rounded. `Expansion::rounded()` is not (V1 probe D), so a correctly rounded function is implemented, and an exact zero gives +0.0 everywhere.
   - D1 states where `Expansion::rounded()` is used on main today and whether any published or bit-compared value depends on it. If one does, that is a new finding for the map.
   - V1's 0.4.0 test is added: an eigen pair plus a nodal load at a shared node, a selected join, and successful finalization.
4. **S11-V5 (NOTE).** Friction terms in the nonlinear loop, and the absence of an in-loop load guard, are recorded as open with T5.
5. **S11-V6. The no-interim ruling stands, restated.** Exposure is governed by the gross-to-net ratio and the order of addition, not by the contribution count. Errors stay at about one rounding step of the gross load; V1's realistic case is about 3e-9 N absolute on a 1.3 N net load. Losing the whole response needs a ratio of 2^53 or more. It still reopens if T1's merge slips materially. **Correction to the S11 pre-acceptance above:** its rationale "exposure needs three or more contributions … nothing committed has that" is superseded by this restatement. Three or more contributions per DOF are normal in real models (V1 S11-V6). The earlier text is kept as history.
6. **S11-V7.** The S11-F record discloses that it changes result bits in real user models. It carries a fixture diff showing that committed fixtures are unchanged.

V1 backchecks S11 together with D1's revision-2 backcheck.

### Adopted text of the no-interim ruling (ROOT, verbatim; replaces both earlier rationales)

ROOT adopted the manager's restatement verbatim. It is the governing text of the S11 no-interim ruling. It replaces the rationale in the S11 pre-acceptance ("exposure needs three or more contributions …") and the wording of item 5 immediately above. Both earlier texts remain as history only.

> No interim measure before T1 merges. The error is at most about half a unit in the last place of the gross load on the DOF, so it reaches 1e-9 of the net only when gross exceeds net by about 1e7 in an unfavourable order. The fix repairs rather than contains. Reopens if T1's merge slips materially or P1 finds a Passed breach in a realistic model.

ROOT's crossing message confirms the manager's proposed dispositions for S11-V1 to V5 and V7, plus ROOT's two additions: the no-new-silence invariant tested on probe A, and D1 reporting where `Expansion::rounded()` is used on main today. All of these have been sent to D1.

## Decisions on S11 revision 2 (ROOT, 2026-09-26, after `4663cdbb6`)

- **D-S11-1: the zero witness, as D1 proposes.**
  - The ledger, the force vector and every bit-compared path use +0.
  - A replaced published expression keeps -0.0 only where the old expression is also an exact zero. This preserves committed evidence bytes, and -0.0 on an exact-zero diagnostic carries no wrong meaning.
  - **Condition:** the witness rule applies only to published diagnostic renderings, never to a value that feeds a bit-equality check or a receipt. V1 checks that boundary.
- **D-S11-2: yes.** `Expansion::rounded()` is replaced at all three FK sites, under the fixture stop rule.
- **D-S11-3: accepted.** S11-K carries live repairs in T1-disjoint files (`straight_pipe`, `load_case_algebra`, the FK rounding sites). It is therefore a full product slice with full gates: independent review, hosted CI including the surface-4 dual-viewport dispatch, a clean DEC-025 sweep, and the fixture stop rule. It lands as its own PR to main.
- **D-S11-4: yes.** No in-band marker; disclosure goes in the change records.
- **Backcheck.** S11 revision 2 goes to V1 now. DESIGN revision 2 gets a second, narrow backcheck when it lands.
- **New findings, added to the map:**
  - **N-S11-R** (low severity): the M03 intended-action residual (`FK/structural.rs:554`) is published in diagnostics using a naive, not correctly rounded, sum. It is an imprecise diagnostic value, not a wrong mechanics result, and D-S11-2 fixes it.
  - **Formation silent zero** (`FK/lib.rs:717-726`): an underflowed stiffness-coefficient product becomes 0 with no error. This is a silent input change on main. D1 states whether it can change a published result beyond 1e-9 in any realistic or RF-RANGE case. K2 (checked formation) is scheduled as an early T1-disjoint slice, next after S11-K, or bundled with S11-K if it shares files and review.

## Rulings on V1's backcheck of S11 revision 2 (ROOT, 2026-09-26, after `61b228543`)

Input: [REVIEW/S11_BACKCHECK.md](REVIEW/S11_BACKCHECK.md), BLOCKING on S11B-1 and S11B-2.

- **The manager's dispositions of S11B-2 to S11B-7 and the formation item are confirmed**, as sent to D1 for S11 revision 3.
- **S11B-1 does not block T1's merge.** S11B-1 is the prescribed-motion RHS `f − Σ K_fc·g`, folded in binary64 in FK (`FK/structural.rs:603-606`, `FK/lib.rs:870-877`). It is live on T1's 0.4.0 support-motion route. It is the same class and bound as S11: about one rounding step of the gross `K_fc·g` term.
  - The no-interim ruling is extended explicitly to T1's 0.4.0 support-motion route.
  - It keeps the same reopen triggers, plus one more: P1, or T1's final review, finds a Passed breach on a realistic settlement case.
- **Its priority rises.** The exact reduced RHS at both FK sites joins S11-K, and S11-K should land soon after T1 merges.
- ROOT tells T1's manager directly. T1 records S11B-1 in its checkpoint 7 as a known, routed open item (T3, S11-K), citing `61b228543`. The T3 manager does not contact T1.

## Rulings on V1's BACKCHECK_R2 (ROOT, 2026-09-26, after `3ea78add8`)

Input: [REVIEW/BACKCHECK_R2.md](REVIEW/BACKCHECK_R2.md). Verdict FINDINGS, nothing blocking; V1-B1 fully resolved.

- **S2-R.** Condition 3 of the retirement gate (standing in all three languages) is satisfied when every language implements identical, fail-closed standing for the whole family. It does not require every case to become eligible. For the joined family under H-a, a case using a host-exp (logarithmic) law reads `needs_recompute` in all three languages; that is correct standing, and it passes the condition. Those cases are simply not eligible. The same rule applies to any family with a declared out-of-scope subset. D1 records this in the gate definition, and D2 cites it.
- **S8-R: the floor is enforced, not just stated.**
  - Classify on the published binary64 value, never on the unpublished precision-p value.
  - R is an exact binary64 constant.
  - D2's G5 gains the S\* and classification check, and a rule for `absolute_verified` quantities: they are withheld from Current, or explicitly exempt with a stated reason.
  - A quantity below the floor is withheld or marked uncovered, and never counted as Passed.
- **SCALE-W.** The kernel half of formation-time scaling (`FK/lib.rs`) is assigned to K2b's write set.
- **F2-P.** The F2 erratum lands with the twist and extension kinds, their derivation and the no-pass rule. V1 backchecks it against V2's corrected counts (43 to 53 RF-WEAK and 3 RF-CANCEL).
- **Notes.**
  - S11B-1's FK sites are added to S11-K's write set.
  - D1 states the `digest_ok()` dependency.
  - The LEF-small correction is recorded.
  - The `retained/` mod declaration gets one owning slice.
- **Next.** S11 revision 3 is still required, for S11B-2 to S11B-7, and V1 backchecks it narrowly. After that, and after R1's narrow revision and V2's recheck, the manager brings ROOT the selection package.

### S2-R, final wording (ROOT, crossing message; supersedes the S2-R bullet above)

Gate condition 3 reads: **"the successor identity's standing is no worse than the retiring identity's, case by case, in all three languages."** Each language's standing must also be identical and fail-closed. The H-a log-law cases are listed as `needs_recompute` under both identities. The switch happens at F3, as R-3(a) ruled.

ROOT confirms the manager's other dispositions (S8-R, F2-P, the notes) and the consolidation into D1 revision 3, S11 revision 3 and a narrow D2 revision 3.

On SCALE-W, the two ROOT messages differ. The explicit ruling assigns the kernel half of formation-time scaling to K2b's write set, while the manager had proposed a new slice after K2a. The manager applies the explicit ruling (K2b) and has told ROOT.

ROOT confirms SCALE-W stays in **K2b**. The kernel half of formation-time scaling belongs with the W2 scaling work, and a separate slice would add a PR with no review benefit. The discrepancy noted above is resolved.

## D2 probe findings (ROOT, 2026-09-26, after `7a6e6f00b`)

- **F-P2 is confirmed.** A rejected, unrecoverable case blocks the whole invocation on every route, under the existing M03 rule. It is recorded as open with D1's F2 (the general method, which recovers such cases), and the corrected probe PR-2b goes into the implementation slice. The whole-invocation-blocking half stays with T6, as the case-scoped standing question already routed there. It fails closed, so no containment is needed.
- **F-P7** goes to T6's carrier-migration notes. binary64 renders 1e16 and 1e20 as integral literals that checked-profile consumers cannot read back, so every consumer of a carrier switches profile together.
- No other rulings are needed on these returns.

## P1's skew Passed breach (ROOT, 2026-09-26)

The finding: `RF-SKEW-T-CANT-OFF-122-r1e-04` is published Passed and Current-eligible on main (preview-physics-1, checks_passed, numerically_eligible). Its RX rotations are about 2.4e-9 relative off (8e-14 rad absolute), at 2.43 times the criterion in dense mode and 1.21 times in sparse mode. There is no cancellation, and the magnitudes are ordinary: a 144 N·m/rad rotational spring (k/a ≈ 1e-4), a 3 m member along (1,2,2), and mN·m tip moments. The 345 sibling passes.

1. **Classification.** This is a genuine silent-wrong finding against the protected criterion, and the most important T3 finding so far. It is a confirmed M03 finding. The "provisional" label is removed once P1 re-evaluates it against the frozen references (`ROOT_SELECTION_REFERENCES.md`).
2. **Containment: no interim measure now.** The error is about 2.4e-9 relative (8e-14 rad absolute) and bounded by ordinary conditioning, and an interim detector would be the same trigger design W1 needs. **It reopens** if P1 finds a Passed breach at 1e-7 relative or worse, or on a realistic multi-member model.
3. **D-5 is now a required decision in the package, not optional.** D1:
   - shows why 122 fails and 345 passes;
   - costs the three options: (a) a forward-error trigger at the Passed boundary, (b) always running W1 for skew models, (c) always running W1 wherever it applies;
   - gives the fraction of R1 cases each option would reroute, and the runtime cost;
   - says whether the trigger can land early as a kernel-local Passed→Sensitive demotion, T1-disjoint, ahead of W1.

   **ROOT's direction:** whatever is chosen, "Passed" must not be publishable for a case the reference suite shows breaching 1e-9. ROOT rules on the options in the package.
4. **P1 search.** P1 looks for more cases in this class across all of R1's families (Passed plus a breach, with no cancellation), and lists every one.
5. **M32 baseline.** P1's observation `RF-LARGE-CHAIN-n1000`, about 21 s and about 3.7 GB peak RSS under RLIMIT_AS 6 GiB on main's dense assembly, is recorded as the M32 baseline.

## Rulings on V1's BACKCHECK_R3 (ROOT, 2026-09-26, after `ef9cf487e`)

Input: [REVIEW/BACKCHECK_R3.md](REVIEW/BACKCHECK_R3.md). Verdict FINDINGS, nothing blocking. These rulings fold into D1's DESIGN revision 4, which also carries D5_TRIGGER, and into D2's revision 4.

- **R3B-1: accepted.** The derived-stress floor gets a pinned propagation factor: √2 for hypot rows, and √2·i for intensified rows, using the row's own i. It must be proven to keep the floor guarantee at 1e-9.
- **R3B-2: accepted.** A closed list of every published row kind, with one class per kind and `not_covered` as the default.
  - Per-case counts of withheld rows are reported in the gate and in P1's survey.
  - The cost that the max_stress headline and the reactions become `not_covered` on the ordinary pressure route is stated explicitly. That cost goes into the D-5 owner-impact framing alongside the trigger, because together they determine how much Current a realistic model loses.
- **R3B-3.** The section terms are carried in the receipt as bit strings. That is more robust than pinning every rounding step in three languages.
- **R3B-4.** G5b gains the stress kind, input-derived rows are classified, and G5c checks set equality.
- **R3B-5.** The site list is keyed by function plus match count.
- **N-1 to N-5: applied.**
  - N-1: the 293 and 303 counts are corrected.
  - N-2: the S\* ≥ 2^-988 exactness bound is recorded, and smaller S\* is handled explicitly.
  - N-3: compare classes, not bits.
  - N-4: S11-K's PR record carries the diff sizes.
  - N-5: the refusal codes are aligned.
- **Next.** V1 narrowly backchecks revision 4 together with D5_TRIGGER. That is the last design check before the package.

## D-5 (ROOT, 2026-09-26, after `7dc2655f9`)

Input: [DESIGN_NUMERICS/D5_TRIGGER.md](DESIGN_NUMERICS/D5_TRIGGER.md).

1. **Hard requirement: D1's reading is confirmed.** The VP-ROBUST "no Passed breach" gate names RF-CANCEL's 12 load-fold breaches as S11 exceptions until S11-F lands. They are covered by the S11 no-interim ruling and fixed by S11-K and S11-F, not by D-5. Once S11-F merges, the exceptions are removed and the gate must hold with none.
2. **V1** starts on D5_TRIGGER now, and answers three questions:
   - why the emulated error is 4–8× smaller than the product's observed 2.43;
   - whether EF's extended-precision re-formation can itself be wrong in the band;
   - whether (a2) can miss a formation-class breach that the 51 emulated cases do not represent.

   V1 checks revision 4 narrowly when it lands. That remains the last design check.
3. **D-5: pre-accepted, option O1.** (a2), the formation-error estimate, lands early as K-D5 after K3. It is kernel-local and T1-disjoint, and routes to W1 after F2. No interim measure before K-D5. **O2 is rejected**, because it withholds Current from correct models. Conditions:
   - V1 returns CLEAR on D5_TRIGGER, or its findings are fixed;
   - revision 4's combined withholding figure (D-5 plus R3B-2) does not show broad loss of Current on realistic models; if it does, ROOT takes the combined question to the owner;
   - K-D5's thresholds come from product data, and P1's measured 122 case is a required true positive in K-D5's tests.

ROOT confirms the selection with the package.

## D-15: withholding on today's exact-block-selected cases (ROOT, 2026-09-26, after `e94af71f2`)

**Not an owner question.** The gate-condition-3 ruling already decides it: a successor's standing must be no worse than the retiring identity's, case by case. If W1's successor would withhold 10–77% of the rows that physics-source-1 publishes as Current today (for example 62 of 82 for N05), that is worse. So exact-block selection is not retired for any domain where that holds.

- **Ordering.** Option A (withholding) is never allowed to cause a regression. Where the successor would withhold rows that are Current today, exact-block selection stays the selected method, and W1 does not replace it, until C (interval binding, owned by D2 as DD-13) or a proven B restores no-worse standing for those rows.
- **B is permitted only as a proof-carrying exemption.** A row is exempt only when it is exactly zero by construction, from a structural decoupling proven from the model's topology and restraints, not merely observed as 0.0. D1 and D2 may use it wherever that proof is mechanical; everything else waits for C.
- **Recommendation: C**, as D1 recommends. It goes into the package with its cost and slice.
- **The gate report shows the per-case withheld counts for both identities side by side**, so condition 3 is checked mechanically.
- V1's final narrow pass checks D-15's figures and this ordering. D2 mirrors D1's table additions: `input_derived` for prescribed-DOF displacement rows and echoed-input review kinds, and the `non_quantity` class.

## T1 merged (ROOT, 2026-09-26)

T1's PR963 is on main as `5aa4285c2`. The T1-overlap hold is lifted. origin/main is merged into the T3 branch before the first product-wiring slice (S11-F, then S-D and the rest). The numerics worktree's product basis moves from `c61a540ea` to `5aa4285c2` at that merge, and D1 and V1 cite the new line numbers from then on. S11-K opens its PR against main once I1 returns, and diffs T1's merged fixtures as pre-registered.

### D-15 option C (ROOT, crossing message)

**ROOT rules C within delegated correctness authority**, under one binding constraint.

- **Conservative binding.** A rule bound to q ± b is satisfied only if it holds for every value in [q − b, q + b]. Otherwise it fails or reads indeterminate. It can never turn a failing check into a passing one.
- **Why no owner question arises.** With that constraint, C affects only rows that would otherwise be withheld under A. It restores their usability with a verified bound, b = 2^-64·S\*, and does not change how any currently covered row binds. It is a correctness-preserving restoration of today's capability, not a new product semantic. An owner question would arise only if C changed how covered rows bind, or let a bound-straddling result pass.
- **Gate condition 3 is tightened** to compare row-level standing: per-case withheld counts for both identities, not just envelope standing.
- V1's BACKCHECK_R4 checks the conservative-binding rule, including its indeterminate case, and whether any withheld row is a nonzero value a user would rely on.
- This goes into the selection package.

## S-H and S11-F ordering (ROOT, 2026-09-26): a hard constraint

Context: P1's secondary observation. The historical typed entry (`run_linear_static_preview_with_mode`, reached from headless `run_preview_in_memory_mode`) publishes the G=1e80 RF-CANCEL cases as M03 Passed, with the net lost or grossly wrong. For example, RF-CANCEL-UDL-W1e80 publishes a rotation of 1.2e57 rad against an expected 4.6e-16 rad. Today the captured entry refuses these cases at capture (V1-S5). D2's S-H would let them solve on the captured route.

1. **S-H never lands before S11-F.** They may land in the same PR. If they are separate, S-H's PR must have S11-F already on main, and its tests must re-run the 1e80 RF-CANCEL cases through the captured route and show them repaired.
2. **S11-F's tests cover both entries:** the captured route (with S-H present if they land together), and the historical typed entry via headless `run_preview_in_memory_mode`. The tests include RF-CANCEL at 1e80 (F, M, ORTHO, INPLANE, UDL-W1e80). D1 confirms that the ledger sits in the shared `solve_load_case` path for both entries.
3. **The VP-ROBUST "no Passed breach" gate runs its cases through both entries.** The S11 exceptions are removed only when both entries are clean.
4. **P1's observation is recorded as secondary.** It is within the adopted S11 bound at an unrealistic ratio, and the typed route cannot mint Current, so it causes no reopen. Recorded fact: the headless typed route publishes M03 quality Passed on gross-wrong values at ratios ≥ 2^53, and S11-F is what removes it.

## D5_CHECK: BLOCKING on D5C-1 (ROOT, 2026-09-26, after `9f792d741`)

1. **D5C-1: V1's fix is adopted.** EF = K⁻¹(f − K_int·u), with the intended-system residual formed as one exact sum over re-formed contributions and the S11 ledger, reusing the FK intended-action audit machinery. **The O1 pre-acceptance stands,** conditional on V1 confirming the fixed EF in the final pass: no misses on V1's sweeps (the 230 absorbed-soft-spring cases and the 105 solve-error cases), plus P1's 122 case.
2. **The no-interim ruling is unchanged.** Its terms still hold: the committed fixtures are clean, and the realistic models are at 1.5e-13 or better. **Correction recorded:** the Passed-band breach class on main is broad in synthetic space. It covers axis-aligned soft springs and solve error at up to about 4e-9 relative, with cond between about 1e6 and 6.7e7. It is not a single case.
3. **K-D5 priority rises, and it moves ahead of K2a.** K2a only affects extreme sub-2^-1022 stiffness, while K-D5 guards a broad Passed-band class. The order becomes **S11-K → K-D5 → K2a → K1 → K2b → K5**, if the exact-residual EF does not need K3's Wide type. If it does, the order is **S11-K → K3 (the part K-D5 needs) → K-D5 → K2a**. D1 states which applies.
4. **Should-fix items.** D5C-2 to D5C-5 and N-3 go into revision 5. **D5C-2:** any stiffness contribution EF cannot re-form (curved, user matrix, joint) makes the case not eligible for Passed through K-D5; it is demoted until coverage exists. Silence is not acceptable. D1 reports whether this over-demotes any committed fixture.

## BACKCHECK_R4 (ROOT, 2026-09-26, after `e27181fb2`)

Input: [REVIEW/BACKCHECK_R4.md](REVIEW/BACKCHECK_R4.md). BLOCKING only through D5C-1, which is already ruled; everything else is SHOULD-FIX. These rulings go into D1's revision 5 and D2's alignment revision.

- **R4-1: accepted.** The span-statics rows use k = 2√2 for the circular maximum and k = 4 for the open-formula summary, because the moment is rebuilt from one end's actions. The floor is proven at 1e-9 with those k.
- **R4-2: accepted, per the D-15 ruling.**
  - Retirement is domain by domain, and the condition is row-level: side-by-side per-case withheld counts, not envelope standing.
  - F2 does not retire exact-block in any domain where the successor withholds rows that are Current today. By V1's counts that includes multicase signed-companion and T1 eigen_motion, where 11 and 23 relied-upon non-zero values are involved (soft-support reaction moments, member torques, torsional shear).
  - **Coexistence rule:** no W1 attempt in an invocation where exact-block selects any case, so one envelope never mixes methods.
  - D1 corrects "almost all exact zeros" for those two cases.
- **R4-3: accepted. This is what the conservative-binding constraint requires.** D2's C design needs:
  - sound interval evaluation over the rule formula language (abs, divide, not, equal and the rest), with a three-valued pass / fail / indeterminate outcome, where indeterminate never counts as pass;
  - scope limited to `absolute_verified` rows;
  - b taken from the receipt, with outward-rounded endpoints;
  - an honest statement of the basis of b;
  - gate-count semantics for indeterminate.

  A formula operator that cannot be evaluated soundly makes the rule indeterminate.
- **R4-4: the tables are aligned.**
  - The intensified k_i rounds upward (conservative), so D1 changes to D2's rule.
  - All 11 mismatches resolve to one table, with D1's closed table as the source and D2 mirroring it.
  - D1 fixes its §5 inconsistency: twist and extension scales are harness-only, not in the receipt.
- **Also:**
  - D2 mirrors N-2;
  - the "no Passed breach" gate carries the named S11 exceptions as (case, quantity) pairs and runs through both entries (`b6fe1eb75`).
- **Next.** D1's revision 5 and D2's alignment revision get V1's narrow re-pass: the fixed EF on the 230, 105 and 122 cases, R4-1 to R4-4, and D-15. That is the last design check before the package.

## Main merged into the T3 branch

The manager merged origin/main (at `0883c2108`, which contains T1's `5aa4285c2`) into the T3 branch as `303609725`, after BACKCHECK_R4 returned and before any product-wiring slice. From here, D1, D2 and V1 cite product line numbers at the merged tree.

## I1's S11-K stop-rule report (ROOT, 2026-09-26)

Context: I1 stopped under the fixture stop rule.
- **A (pre-registered):** T1's support-motion fixtures, as expected.
- **B1 (unregistered):** 6 values per file in `fixtures/results/preview_physics_invented_{dense,sparse}.json` change, about 4 ulp from the live E4/E6 exact station sums, plus a near-zero stress moving at roundoff scale.
- **B2 (unregistered):** T1's `load_reference_fallback_uz` raws change their Debug residual text, from KS1–KS3.
- **B3 (informational):** a qualification input's Debug text changes.
- **C (regression):** `validation/benchmarks/nonlinear` `multisupport_acceptance_inventory_uses_narrow_dec_046_policy` fails with a 2.6e-16 N residual against DEC-046's exact-zero limit. The nonlinear active-set loop prescribes nonzero closed-gap displacements, so KS1–KS3 are live on main there, contrary to S11 §4.6 and §8.1.

**Rulings:**

- **C: option (c), provided I1 confirms a clean separation.**
  - The nonlinear loop's closed-gap prescribed solves call a named, unchanged binary64 legacy function. Linear callers get the exact KS1–KS3 path.
  - A test pins the nonlinear loop to the legacy variant.
  - If the separation isn't clean, option (a) applies: KS exact on the typed path only, with the S11B-1 fix going live with S11-F. (c) is tried first.
  - **DEC-046's zero limits are not touched (option (b) rejected).** They are protected and T5-owned.
  - **Recorded as T5 open work:** the nonlinear loop's closed-gap prescribed solves stay binary64 until T5 designs its residual policy, together with the friction fold. S11-F's design states the same.
  - D1 corrects S11 §4.6 and §8.1 in revision 5: KS sites are live on main in the nonlinear loop. V1 covers this in its re-pass. **Lesson recorded:** both the design and its review missed the nonlinear loop's nonzero prescribed values. The checks must look for every caller of a changed kernel function, not only the product's linear path.
- **B1: accepted as the intended repair.** Before regenerating, I1 lists every TS, Python and result_export test or fixture that pins these bytes, measures the diff of every derived document of A and B1, and reports both to ROOT. Regeneration is by the actual producer only, never by hand, and the S11-K change record discloses it with sizes.
- **B2:** under (c), it is added to the pre-registered list (same class as A).
- **A:** proceeds as pre-registered. Its derived documents are included in the measured diff report before regeneration.
- **B3:** recorded as informational.

## D5C-5: the S11 exception list (ROOT, 2026-09-26, after `490f02982`)

- The S11 exceptions in the VP-ROBUST "no Passed breach" gate are exactly P1's recorded (entry, case, quantity) triples, pinned per entry: **106 triples in 13 cases on the captured entry, and 168 in 22 cases on the typed entry.** They include RF-CANCEL-UDL-W1e8. ROOT's earlier "12" was a miscount.
- The list must be empty after S11-F.
- It is re-pinned from P1's final `results.json` once P1 re-evaluates against the frozen references.
- **Any triple outside the list is a gate failure, not a new exception.**

## BACKCHECK_R5 (ROOT, 2026-09-26, after `0874bed35`)

Input: [REVIEW/BACKCHECK_R5.md](REVIEW/BACKCHECK_R5.md). Verdict FINDINGS, nothing blocking. **The D5C-1 condition is met, so O1 is final, subject to R5-4.**

1. **R5-3 (urgent, for I1).** The KS caller list also covers:
   - `product_equilibrium::evaluate`, which calls `evaluate_original_residual` (KS3) from the nonlinear loop (nonlinear lib.rs:1957) and from SA:1030;
   - `linear_supports::apply_linear_supports`, which calls KS2 (linear_supports lib.rs:467, :487), used by the mechanics benchmarks with imposed displacements (6 sites).

   Option (c) applies to both. Every nonlinear-loop caller, including `product_equilibrium` on the loop path, is pinned to the legacy binary64 variant with a pinning test. For SA:1030 and `linear_supports`, I1 reports whether they are linear callers. If they are, they get the exact path, and any benchmark or fixture bit change goes through the stop rule and is reported to ROOT before regeneration. D1 corrects the S11 caller list.
2. **R5-1: V1's one-rule fix to `b_proof` is accepted** (the reaction rule for loaded fixed-fixed members). D1 adopts it, and D2's S-J mirrors it. Committed proven counts are unchanged.
3. **R5-2.** `input_derived_dofs` is exactly the kernel's non-free DOFs, with the product's family rule stated explicitly. Spring hangers and constant-effort supports are free DOFs carrying applied force or stiffness, not rigid.
4. **R5-5: accepted.** Condition 3 adds a check-level comparison: the committed rule packs run under both identities, and the successor must not turn a decided check (pass or fail) into indeterminate. Where it does, that domain does not retire.
5. **N-1.** b is the exact point only when S\* = 0. For 0 < S\* < 2^-1011, b is rounded up to the smallest positive subnormal, not 0. **N-2 to N-6:** applied.
6. **R5-4: data first.** D5C-2 demotes every Passed invocation containing a realized DEC-070 curved bend until W1c, and real piping models usually have elbows. Before ROOT decides whether this goes to the owner, D1 provides the following, and V1 checks it:
   - how a user realizes a curved bend today (default or opt-in), and whether any committed or realistic invented model uses one;
   - how many realistic models would lose Current;
   - whether a bound-based alternative for curved contributions is feasible now, without T4, and its cost. Examples: re-forming the curved element's stiffness in Wide precision from its inputs for EF, or a documented formation-error bound for the curved element;
   - whether T4's null-space confirmation is a prerequisite for that.

   If a sound alternative exists within T3, ROOT selects it. Otherwise ROOT takes "demote realized curved bends until T4/W1c" to the owner, with the numbers. **The package may be assembled around R5-4, with R5-4 flagged as open.**

### R5-4 and process (ROOT, crossing message)

- **R5-4: proceed as the manager proposed.**
  - D1 designs a conservative formation-error bound for curved contributions, entered into EF, so a model is demoted only when the bound threatens 1e-9.
  - D1 counts the committed and realistic invented models that realize curved bends, and states whether T4's null-space confirmation is a prerequisite for the bound.
  - V1 checks that the bound is sound.
- **If no sound bound exists within T3,** ROOT receives owner options with numbers and takes them to the owner with its recommendation. ROOT's current leaning:
  - it recommends against (b), exempting with a caveat, which is a silent pass with a note;
  - it leans to (a), demote until W1c, if the affected models are few;
  - it leans to (c), delay K-D5, only if (a) would demote most realistic models.
- **Process: agreed.** One narrow D1/D2 follow-up covers R5-1, R5-2, R5-5, N-1 to N-6 and R5-3 (the S11 caller list). V1 verifies only those fixes and R5-4's bound. Then the selection package is assembled, with no further full review round.

## S11 exception re-pin (ROOT, 2026-09-26, after P1's follow-up `d28f69dd7`): supersedes the counts in D5C-5

- **Correction.** The "106 captured / 168 typed" counts in D5C-5 above were not P1's record. They came from D1's *prediction* script `DESIGN_NUMERICS/_run_records/s11_exceptions.py` (revision 5). That script takes every quantity key of each case's discriminating R1 negative control. Those keys include the midspan bending magnitudes `Mb.M1.mid` and `Mb.M2.mid`, which are reference quantities but not product results: P1 counts them `not_published`. Removing those 46 predicted triples (20 `Mb.M1.mid` and 26 `Mb.M2.mid`) gives P1's observed set exactly. No observed breach lies outside the prediction. D1's text and this record then quoted the prediction as "P1's record". The error was the attribution; the prediction itself was stated as a prediction.
- **The re-pinned list** is `GATE/S11_EXCEPTIONS.json`, generated by `GATE/pin_s11_exceptions.py` from P1's final `DETECTION/results.json` (sha256 `a1188624…`). It covers frozen-reference cases only, as (entry, case, quantity) triples:
  - **captured: 88 triples in 13 cases;**
  - **typed: 140 triples in 22 cases.** That is the same 88 (the typed entry is bit-identical to the captured entry on those cases), plus the 52 triples from the nine G = 1e80 cases the captured entry refuses at capture.
  - P1's own probes (S11-PROBE-A-*) and V1-CHECK-L-* are excluded. The probes become S11-K/F test cases with exact expected nets.
- **RF-SKEW-T-CANT-OFF-122-r1e-04 is not an exception.** It is K-D5's required true positive on both entries.
- The list must be empty after S11-F, and any triple outside it is a gate failure. That is unchanged.
- The 106/168 figures in `DESIGN_NUMERICS/DESIGN.md` (D5C-5 row, §4.10), `S11_CONTAINMENT.md` (the D5C-5 row, §8) and `D5_TRIGGER.md` are superseded here, and D1 corrects them in revision 5a.

## R5-4: curved bends under K-D5 (ROOT, 2026-09-26, on D1's note `4bc3e0696`)

- **Pre-accepted.** EF re-forms curved contributions in `Wide<2>` from their binary64 inputs, as an objective element built from the actual chord. The Wide arctangent is added to K3a. No owner question arises, and options (a), (b) and (c) fall away.
- **Final acceptance depends on V1's targeted verification:**
  - the re-formation is sound and rotation-consistent by construction;
  - it misses none of the 4 curved Passed breaches on main;
  - it demotes none of the 12 realistic elbow case-modes (E1–E6);
  - reusing the product's binary64 curved matrix misses 2 of the 4, so re-formation is required.
- **Findings recorded** (STAGE0_MAP §6):
  - Main publishes curved-bend Passed breaches (4, up to 1.48×). This is an M03 extension in the skew class, under the no-interim ruling; K-D5 is the fix.
  - The product's curved element is not rotation-consistent on binary64 inputs. It is routed to T4 and W1c, and goes into T4's graph row in the next records PR.
- D5C-2's demotion stays as the fail-closed default for any future family EF cannot re-form.

## D-14 scope: self-weight (ROOT, 2026-09-26, after `28d96084f`)

- ROOT agrees with the manager's reading. Generated self-weight is an authoring-time model operation that writes a `distributed_force` intensity into the model document, where the user can see and edit it. It is therefore the model's input, not a solve-time derivation.
- D-14's exact-from-inputs rule applies only to the solve-time seismic and wind equivalent-static generators. No owner options are needed.
- The other RF-ELOAD definition answers (`MANAGER_NOTES/RF_ELOAD_DEFINITIONS.md`) are accepted.

## D-14 slice correction and revision 5a (ROOT, 2026-09-26, after `16bcbf369`)

- **Correction.** D-14's equivalent-static extension goes in **F3**, with the W1b element-load work, not F2b. ROOT's "F2b" was a slip: generated equivalent-static loads are uniform element loads and need W1b's element-load machinery. It lands atomic with F3's RF-ELOAD gate. Until then, the case is refused as `equivalent-static unsupported`. D1's revision 5a already places it this way.
- The rest of D1's revision 5a (`16bcbf369`) stands as ruled. It goes to D2 for revision 5b, then to V1's targeted verification.

## D2 revision 5b choices (ROOT, 2026-09-26, after `41f018355`)

1. **Undecided counting: accepted.** In the check-level gate (R5-5), withheld-row refusals count as undecided. This is conservative and correct: a check the user could decide yesterday and cannot today is a regression, whatever the reason.
2. **Nonlinear-support rule: accepted, narrowed.** It must never refuse the solve. A case with a nonlinear support is **not selected** for W1 or the retained-precision route, so it gets no proof and no successor identity. It keeps its ordinary result with its ordinary standing, exactly as today. This is T5's domain, and T3 must not remove a result the product publishes today.
   - D2 narrows its rule to that: `INPUT_DOF_MISMATCH` may apply only where a receipt *claims* selection of such a case, as a reader integrity refusal of the receipt, never of the ordinary result.
   - D1 mirrors it in the W1 selection rule, together with the trimmed/untrimmed support-family detail D2 took from `PP`.
   - V1 checks both.

## RF-ELOAD refutation (V3) rulings (ROOT, 2026-09-26, after `3bb46bc62`)

- **D-14 clarification (V3 Q2 / F3).** "Computed exactly from the user's inputs" means exact from the **binary64 inputs as the document stores them** (D1 §4.2). It is the same principle as re-forming stiffness from binary64 primitives. The authored decimal text is not the source. The represented basis for RF-ELOAD-CANCEL-SEIS-G1e7 and -G1e8 stands.
- **V3 Q1 / F1: add a variant; do not edit G1e8.** The author adds RF-ELOAD-CANCEL-SEIS-G1e8-R (ROOT's second message; its first said "G1e8b") with V3's inputs (gen.g_factor.Z = −0.23, D = 143.44076231504138435), on the represented basis. Expected: NC-FLOAT-SUM fails about 9.44×, NC-BIN64-PRODUCT about 89.6×, finite_input about 8.55e-9. Every existing case and value stays byte-identical. G1e8's NC-FLOAT-SUM control is relabelled as the benign-rounding case.
- **In the same revision:**
  - F1 text: the NC-FLOAT-SUM control text and README finding 3 say what the control rounds.
  - F8: retire NC-LOST-SOFT at r1e-06.
  - F4–F6: the control texts specify the defect exactly as V3 found it (the lever rule lumps full-span loads 50/50; ALPHA-TIMES-INTERVAL and SUBTRACT-DILATIONS drop the fit; COMB-DIFF's NC-MAG-SUM is |Mb_A − Mb_B|; corrected 2026-09-26 by ROOT after V3's delta check at `1daa512d4`, where V3 withdrew its F6 misreading; ROOT's ruling text had said "Mb_A − Mb_B").
  - V3's §6.2 labels on the 21 kept controls, and its re-scale decision.
  - F9: a symmetric CANCEL-FEM gross column, or one labelled review-only.
  - F10: a README harness-mapping note (station labels under an i/j swap; tp_phys_008's N sign).
  - F7: no change to the rule, which V3 rules sound and ROOT accepts. The README says it is a new rule, since R1 refuses that case.
- **Check.** V3 re-refutes the delta only: the new case by its own route, the changed or retired controls, and byte identity of every pre-existing value. Then RF-ELOAD comes to ROOT for selection, before F3.
- **Carried into F3's brief (D-14):** a **required** kernel-level test that each generated load enters the ledger as the exact product of its binary64 inputs at working precision, with no binary64 intermediate. No non-cancelling reference can observe D-14.

## RF-ELOAD selection (ROOT, 2026-09-26)

RF-ELOAD revision 1 at `b6927f783` is selected and frozen: see `ROOT_SELECTION_RF_ELOAD.md`. The N1 and N2 wording errata are recorded there and not applied. The COMB-DIFF ruling-text correction to |Mb_A − Mb_B| (dated above) is ROOT's.

## K3a arctangent and host disk (ROOT, 2026-09-26)

- **K3a arctangent: accepted.** I2's proved bound of 23.6 ulp at p = 128 satisfies `ROOT_SELECTION_DESIGNS.md` C1 as a proof. The measured worst, 5.41 ulp over I2's wider angle set (V1 measured 2.69 over 13 angles), is recorded alongside it. The test tolerance covers 5.41 and stays at or below 23.6.
- **Disk.** ROOT deleted finished scratch (t3-p1, which matches the committed DETECTION/RETURN.md, and T1's WP7 scratch copy). The floor stays at about 8 GB. Once RV1 has returned and no cargo process is using them, the manager may prune `t3-target`, delete t3-s11k scratch if S11-K needs no fixes, and delete `<scratch>/rv1-s11k-scratch`. `<wt>/engine` is left alone, because it is the node_modules source for the desktop tests.

## S11-K regeneration and hash-pin approvals (ROOT, 2026-09-26, on I1's pre-regeneration report `14354efdb`)

ROOT relayed these decisions by SendMessage; they are recorded here as relayed. The recording was late: RV1's S3 found it missing (`REVIEW/S11K_REVIEW.md`, `65e98c259`). The substance was verified by RV1 on the candidate.

1. **Regeneration: approved.** "Regenerate A, B1 and B2 with their producers, and then the 12 derived files with the §5 producers. Producers only, no hand edits. The change record discloses every size, and I1 explains the sparse invocation_work publication_charged −12."
2. **Hash pins: approved.** "I1's write set extends to the two hash-pin constants (result_export/tests/load_reference_contract.rs and tests/test_load_reference_readers.py FROZEN), in the same PR, with old → new hashes disclosed. They pin exactly the pre-registered A raws."
3. **TS suite:** "yes, run the full desktop suite locally before the PR." The package-lock at the S11-K head was byte-identical to the engine worktree's, so node_modules was symlinked from there rather than running `npm ci`, and the links were removed before committing. The desktop vitest and build were run after regeneration.
4. **The index touch:** "noted and accepted as harmless. The rule stands: no Git writes by TASKs."

**Then (ROOT):** the S11-K PR needs an independent review of the full diff (the §8.1 items, the R3 fixes, R5-3's caller classification, the option-(c) pins), hosted CI, the full dual-viewport dispatch, and a clean DEC-025 sweep. **PR scope (ROOT, later):** option (b), a fresh branch off origin/main (`codex/piping-s11k-pr-20260926`), carrying the S11-K commits only.

## I4's F12 stop: formation-class exceptions and slice S11-G (ROOT, 2026-09-27, option (c))

I4 (S11-F) held F12 correctly: S11-F clears 221 of the 228 pinned triples. The remaining 7 are formation and formed-term errors, not load absorption, so the ledger cannot repair them.

1. **Re-pin, and a correction of the record.**
   - **The correction.** P1's class heuristic and the 2026-09-26 re-pin misclassified these 7 triples as S11 class. They are pre-existing on main (P1 baseline), and S11-F makes none of them worse. I4 shows each is bit-identical, or no worse, between base and candidate.
   - **The re-pin.** The 7 are moved into a separate, exactly pinned formation-class list, `GATE/FORMATION_EXCEPTIONS.json`: 7 triples, 14 rows as (entry, case, quantity, mode).
     - RF-CANCEL-UDL-W1e80 th.S1.RZ (typed): load formation.
     - RF-CANCEL-UDL-W1e8 th.S1.RZ (captured and typed): load formation.
     - RF-CANCEL-F-G1e80-GnG-INPLANE Mb.M1.j and Mb.M2.i (typed): formed K_e·u.
     - RF-CANCEL-M-G1e80-GnG-INPLANE Mb.M2.i and Mb.M2.j (typed): formed K_e·u.
   - **The S11 list** (`GATE/S11_EXCEPTIONS.json`) is now 221 triples: 87 in 12 cases captured, 134 in 18 cases typed. It must be **empty after S11-F** (F12 in its literal form). The gate fails on any triple outside the two lists. Both lists are generated by `GATE/pin_s11_exceptions.py` from P1's results.
2. **The reopen trigger is met.** The INPLANE rows exceed 1e-7 relative, and UDL-W1e80 publishes 1.2e57 rad as Passed on the typed entry. That meets the no-interim reopen trigger, so the interim response is a formation-noise guard, **slice S11-G**, without waiting for F2 and F3.
   - D1 writes a narrow design note: the **load-row guard first** (Σ over *formed* terms of |term|·(formation bound), against 1e-9·max(|net|, scale)), then the **recovery-side guard** on formed K_e·u end actions, as a second part or a small follow-on.
   - **The note distinguishes formed terms from input terms.** A nodal load or other exactly represented input carries zero formation noise, so legitimate exact-input cancellation (including the 221 triples S11-F repairs) is never demoted.
   - It is fail-closed: demote to Sensitive, never refuse, and no new envelope field. If a field would be needed, it stops and goes to ROOT.
   - **V1 checks the note:** all 7 caught; none of the 221 repaired triples, and no RF-CANCEL, RF-SKEW or committed-fixture row, demoted; and the committed-byte forecast.
3. **Sequencing.**
   - S11-F lands first, with the formation-class pin.
   - S11-G is implemented next, on main after S11-F, and lands **before F2 and F3 and before any W1 retirement**.
   - The formation list is empty when S11-G lands, or the note justifies, per row, why that row waits for F2/F3's exact formation; if so, it goes to ROOT.
   - K-D5 may land before or after S11-G, whichever is ready first, with the PP conflict boundary named.
4. **Smaller items:**
   - F10 runs under the test-only historical pressure scope, recorded as such.
   - N05's transverse-tip retained-replay budget overflow: confirmed on base, it is pre-existing, recorded and routed to T3's N05 item. If it is a regression, S11-F stops.

## S11-F fixture stop: conditional pre-approval of regeneration (ROOT, 2026-09-27; recorded before regeneration)

ROOT relayed this by SendMessage; it is recorded here as relayed, before any regeneration (the lesson from S11-K's S3). If the measured pre-regeneration report (`IMPLEMENTATION/S11F/PRE_REGENERATION_REPORT.md`) meets these conditions, I4 regenerates without coming back to ROOT. If any condition fails, I4 stops and the report goes to ROOT.

**A (load_reference/connected): approved.** "Regenerate the raws, their derived documents and the two hash pins, by the actual producers only," provided the pre-regeneration report shows all of the following:
- every changed leaf is Debug/diagnostic text, a digest or hash, or a work unit derived from text length (like S11-K's −12);
- no published result value and no status or diagnostic code moves;
- the hash pins' old → new values are stated;
- every pinning test is identified and green after regeneration.

**B (physics_thermal_ui_model): approved**, provided all of the following:
- I4 names the actual producer, and shows it reproduces the committed dense file byte for byte on base;
- S11-F's changes are only the listed reaction residues: a restrained-DOF support_reaction component and its magnitude, at noise level (well under 1e-9 of the case's load scale), with no status change;
- the line-2709 sparse drift is explained (producer, commit or platform) and disclosed separately in the change record as pre-existing drift corrected by regeneration, not as an S11-F effect;
- all named consumers (result_export physics_contract and physics_source_contract, the three Python tests, the desktop parity test) are green after regeneration.
"If no producer reproduces the committed files on base, stop and bring me the provenance question before regenerating B."

**Both:** regeneration is by the producers only, and the regenerated bytes must equal the pre-regeneration measurement byte for byte. S11-F's CHANGE_RECORD cites this entry.

## Amendment: the "no worse" condition for formation-class rows (ROOT, 2026-09-27)

I4's base-against-candidate comparison of the 14 formation rows: UDL-W1e80 is bit-identical; the four INPLANE triples are much better (about 1e9× → 628–5767× the criterion); **the 4 UDL-W1e8 th.S1.RZ rows (captured and typed, dense and sparse) are 3% worse (46.47× → 47.99×).** The cause is that the FEM terms carry formation error, S11-F publishes their exact correctly rounded net (0.4916666902601719), and base's binary64 fold happened to land slightly closer to the intended 0.49166666666666667.

- **Amended condition for formation-class rows:** the published value is the correctly rounded net of the represented terms, **and** the row stays exactly pinned in `GATE/FORMATION_EXCEPTIONS.json`.
- **The 3% worsening is recorded** in S11-F's CHANGE_RECORD, in the formation list's entry for those rows (`GATE/pin_s11_exceptions.py` and `FORMATION_EXCEPTIONS.json`), and here. The 10 bit-identical-or-better rows are disclosed as such.
- **S11-G's load-row guard is required to catch all 4 UDL-W1e8 rows** (added to D1's S11-G brief).
- **N05:** the transverse-tip retained-replay budget overflow is pre-existing on base in both modes, and routed to T3's N05 item. Accepted.

## S11-G note: rulings, conditional on V1's check (ROOT, 2026-09-27, on `633460fb4`)

- **(a) The exact signed defect instead of the a-priori Σ|term|·bound: accepted.** D1 showed the a-priori form fails its own coverage conditions: it demotes UDL-W1e5 and probe A. **The exact defect must itself be computed exactly, with no binary64 intermediate.** V1 confirms that, and confirms that the stated-bound fallback (curved consistent vectors, exact-pressure operands) is conservative.
- **(b) The 8 INPLANE rows wait for F2: accepted provisionally.** The reopen trigger is met, so this does not rest on D1's word alone. V1 must establish:
  - (i) that the INPLANE class is reachable only at synthetic scales on the typed entry (the captured entry refuses it), and that D1's scan of realistic and committed models shows no row near the criterion by this mechanism;
  - (ii) which 16 case-modes R-b would falsely demote, and whether they are realistic or committed models, or synthetic.
  **If (i) fails, or if R-b's false demotions are all synthetic, R-b ships in S11-G:** fail-closed availability loss on synthetic cases beats wrong Passed values. Otherwise the 8 rows re-point to "F2 (W1a)", F2 is required to empty them, and **F2 is prioritized as the next facade slice after S11-G**.
- **(c) Case-level demotion (with source_eligible = false for a fired case): accepted.** It fails closed; case-scoped standing is T6's.
- **The S11-G PR must show a zero regeneration diff, or it stops.**

## S11-G after V1's S11G_CHECK (ROOT, 2026-09-27, on `30233f93a`)

- **(b)(i) failed, so R-b ships in S11-G.** V1 showed the INPLANE mechanism is reachable through an ordinary authored nodal load on the captured entry, and in the realistic RF-LARGE-CONT-n00100 at 202–25984× the row-relative criterion. V1's classification shows R-b is mostly right: of its 16 extra case-modes, 8 (WEAK, LARGE-CONT) are genuinely wrong rows, and only 8 (LFRAME, synthetic) are false demotions, which are acceptable. **The 8 INPLANE formation rows stay in S11-G's scope, not F2's.**
- **D1 writes S11-G note revision 2:**
  1. **B-1:** add R-b and forecast it by script against every committed fixture case-mode (P1 estimates row noise above 1e-9 in 30 of 114).
     - For each case-mode where R-b fires, report whether the row really is wrong against an exact reference, or is a false positive, and which of them K-D5 already demotes (so redundancy is known).
     - **If R-b fires on any committed fixture, that is a status change under the stop rule** and goes to ROOT with per-row evidence before implementation. Truly wrong rows being demoted is the intended repair (likely approved, with disclosure); false positives need a refined rule.
  2. **SF-1:** the defect is computed in one exact accumulator with no binary64 intermediates, in the specification as well as the script.
  3. **SF-2:** the fallback bounds are genuinely conservative. That means the exact-pressure operands' actual rounding count, and the curved bound including cond(F), cancellation and libm, or those contributions routed to "cannot bound", which demotes.
  4. **SF-3:** T10 is non-vacuous (a case whose ordinary solve runs recovery), so that M7 and M8 are killed.
  5. **SF-4: fixed, not just disclosed.** A pure-thermal or pure-pressure straight run between anchors must not be demoted. Fix the free-row scale collapse (for example with a floor tied to element-level force magnitudes or the intended loads), and pin V1's probe as a test that must stay silent.
  6. **The NOTEs:** the INPLANE labels corrected against I4's records, and "the error is in u" corrected to V1's figures.
- **V1 then checks revision 2:** B-1's forecast, the SF items, SF-4's fix, and no committed fixture falsely demoted. The zero-regeneration-diff rule stands, except for demotions ROOT approves under point 1.
- S11-F is unaffected and proceeds.
- **Clarifications (ROOT, 2026-09-27):**
  - (i) SF-4 is a required fix: the S* floor, or an equivalent, is in the design. V1's collinear-run probe is a test that stays silent, and all 6 UDL rows stay caught.
  - (ii) R-b is decided on evidence. D1 forecasts at least **R-b** as written and **R-b′**, which is R-b restricted to rows at or above the V1-S8 floor (|q| ≥ about 5.4e-11·S*) or the contract floor at that row. Each is run over the committed fixtures, the frozen references and the 221, reporting: the INPLANE rows caught, and whether each is a Passed breach under today's predicate; the false demotions (rows correct under today's predicate), split committed, frozen-realistic and synthetic; and the overlap with K-D5.
  - **Default:** the variant that catches every INPLANE row that is a published Passed breach under today's predicate, with no committed-fixture demotion of a row correct under that predicate. Otherwise the numbers go to ROOT.

## S11-G note revision 2: rulings, conditional on V1's delta check (ROOT, 2026-09-27, on `a5137da0f`)

1. **R-b′ selected**, by ROOT's default rule. It catches all 8 INPLANE rows, each a Passed breach under today's predicate; it demotes no committed row that is correct today; and its 12 false demotions are all synthetic. V1 confirms the committed forecast (0 firings) and the synthetic classification of the 12.
2. **SF-4's per-row floor, 2^-10·Σ|self-equilibrated formed terms|: accepted.** V1 confirms that the collinear and pure-pressure runs are silent, that the 6 UDL catches are unchanged, and that the floor cannot be gamed to hide a real formation error (a counterexample search: a row with a real defect below the floor but above the criterion).
3. **SF-2's CannotBound for curved consistent vectors, with its availability loss: accepted.** Realized curved bends are opt-in, and no committed model uses one. The loss is disclosed in S11-G's CHANGE_RECORD, and W1c with T4 removes it.
4. **The formation list is emptied when S11-G merges,** provided S11-G's gate run shows all 14 rows published non-Passed on both entries and both modes. Until then it stays pinned.
5. **K-D5 on INPLANE** (1.6× and 2.0× its trigger in D1's recal): I3 confirms on the merged state. If K-D5 does demote them, the overlap is recorded as defence in depth, not a conflict; S11-G's R-b′ still ships.

After V1's PASS, the manager writes the S11-G implementation brief. S11-G goes on main after S11-F merges. I4's formation_rows.json input is pinned by full hash (`c548f51999b4df16…`) in `DESIGN_NUMERICS/_run_records/inputs/` until I4's commit lands.

## S11-G revision 2 after V1's delta check (ROOT, 2026-09-27, on `a3ddc4dea`)

1. **DB-1 accepted as V1 proposes.** The SF-4 floor applies **only to the self-equilibrated part of the defect**, with two exact accumulators, and a net formation defect is never floored.
   - D1 adds test **T6b** (V1's counterexample: a UDL-type fixed-end defect at a free row, masked by two anchored thermal members; it must fire) and its killing mutation.
   - **Residual accepted for now:** a junction of self-equilibrated terms only, with a genuine small net, is hidden by any noise-silencing floor. It is disclosed in the note and in S11-G's CHANGE_RECORD, and routed to W1/F2, which removes it.
2. **DS-1 accepted. Ruling 5 of "S11-G note revision 2" is amended:** K-D5 does **not** demote the INPLANE cases after S11-F. D1's 1.6× and 2.0× came from recal_d5's pre-S11-F folded force, and V1's exact-residual EF with the S11-F force gives 2·EF ≈ 1.3e-6 to 3.2e-6 of the trigger. **R-b′ is the only catch; there is no defence in depth there.**
   - The note's §1, §5 and §6.4 are corrected.
   - I3 is told to expect K-D5 to stay silent on these cases, and not to "fix" that.
   - R-b′'s tests are therefore load-bearing, and the implementation review mutation-checks them hard.
3. **V1's NOTEs go into the implementation brief:**
   - the underflow condition on the FMA error term;
   - the constant 1e-9 made safe by strict rounding or an exact rational;
   - a source pin tying the routing site to the tested predicate (M7);
   - a skew-member unit test of the bound (the M14 |Tu| variant).
4. **Next:** D1 writes revision 2.1, V1 checks the diff only, and on PASS the manager writes the S11-G implementation brief.

## Selection: the S11-G design (ROOT, 2026-09-27)

**S11-G's design is selected at revision 2.1** (`DESIGN_NUMERICS/S11G_GUARD.md`, sha256 `7c052c9e…`, commit `ba5d26924`), on V1's PASS (`REVIEW/S11G_CHECK.md` delta-2.1, sha256 `a064bc93…`, commit `02d071f88`). ROOT's earlier rulings are its conditions:
- **R-b′** is the recovery guard;
- **the SF-4 floor applies to the self-equilibrated defect only,** with two exact accumulators; a net formation defect is never floored;
- **CannotBound for curved consistent vectors,** with its availability loss disclosed in S11-G's CHANGE_RECORD;
- **the formation list is emptied at S11-G's merge,** provided all 14 rows are published non-Passed on both entries and both modes;
- **R-b′'s tests are load-bearing** (the only INPLANE catch after S11-F);
- **the DN-4 residual** (a self-equilibrated-only junction with a genuine small net) is disclosed, and routed to W1/F2.

V1's two implementation NOTEs go into I5's brief:
- **D21-1:** the exact test |A_net| > 12·(T0 − B), with ±12B and ∓12T0 added exactly into the accumulator copy, and no rounded comparison;
- **D21-2:** the scaled-RoundedProduct underflow fallback is γ2·|value| + |k|·2^-1074.

S11-G is implemented by I5 on main after S11-F merges, and reviewed by RV4.

## K-D5 mutation M31b: accepted as equivalent at the criterion (ROOT, 2026-09-27) — SUPERSEDED

**Superseded (2026-09-27):** this acceptance is withdrawn. See "K-D5 mutation M31b: equivalence withdrawn" below. The text is kept as the record of the earlier ruling.

**The mutant.** M31b is the chord-only form of the design's mutation 31. K_t is still re-formed, but the intended element's H is built from the product's formula chord R(cos φ − 1, R sin φ, 0), using binary64 R, atan2, cos and sin, instead of the actual node chord. The formula evaluated at p (M31b0) also survives. Every other K-D5 mutation is killed: M23, M26, M27, M28, **M31a** (the whole-matrix form, where K_int is the product's binary64 matrix), M32a and M32b.

**Evidence (I3):**
- On CSKEW_8_5, the mutant's trigger equals the correct check's to 4+ digits.
- Admissible radius-mismatch elbow:
  - the centre is shifted −6.5e-10·R, giving |ri| − |rj| = 9.2e-10 relative, inside the product's 1e-9 tolerance;
  - the formula chord differs from the actual chord by about 1.4e-10 m per component, confirmed on an instrumented build;
  - triggers are 1.681 (mutant) against 1.685 (correct);
  - the chord error moves EF by about 0.002 of the criterion.
- **The mechanism:** a rigid-rotation force pair from a chord error has only a second-order net moment on the soft mode.
- **Consistency:** this agrees with V1's `REVIEW/VERIFY_R5.md` item 16, which reproduced mutation 31 only through the shared matrix.

**Ruling: option (a) accepted.** M31b is recorded as an equivalent mutant at the unchanged 1e-9 criterion. Nothing is weakened. The conditions are:
1. **What stays required:**
   - the test `kd5_curved_intended_element_uses_the_actual_chord` (the admissible-mismatch elbow, k_X = 30; it demotes in both modes and kills M31a);
   - M31a's kill;
   - the actual-chord implementation (R5-4 §2 step 6).

   The equivalence is about the test's power, not the implementation: the code must still use the actual chord.
2. **The K-D5 reviewer (RV-K-D5) gets a specific attack:** construct an admissible product model on which M31b is observable at the criterion. Candidates include:
   - small bend angles;
   - near-π angles short of the AngleDomain refusal;
   - extreme R/L;
   - stiff-X or soft-Y combinations;
   - the most distorted admissible centre.

   If RV-K-D5 finds such a model, it becomes a required test, and M31b must be killed before merge.
3. **Records:**
   - The equivalence is recorded, with these numbers, in K-D5's RETURN and CHANGE_RECORD and here.
   - The claim in `DESIGN_NUMERICS/R5_4_CURVED.md` and `D5_TRIGGER.md` that "H from the product's chord misses k_X = 8.5" holds only for the whole-matrix form, M31a. It is corrected at the next D1 touch, or noted in the K-D5 record.

## S11-G implementation: I5 rulings (ROOT, 2026-09-27)

These are on I5's three items from base `43b8f83aa`, before any build.

**1. Ordering: accepted, with the manager's condition.**
- **The approach:** R-b′ needs the published rows, which exist only after the straight element-recovery loop. The load-row finding therefore goes in at `append_integrity_report` on both call sites. After the loop, R-b′'s finding amends the same integrity diagnostic, through the same helper and the same no-op rule, and only while its code is still `CHECKS_PASSED`. Diagnostic order and byte layout are unchanged. If both guards fire, the load-row sentence stands.
- **The condition:** I5 enumerates every reader of the diagnostic code, the envelope's `solve_quality` or the case standing between the append and R-b′'s amendment. **Any reader in that window stops the work and comes back to the manager.**
- **Required tests:** R-b′ alone, and both guards firing.

**2. The erratum at B = T0 = 0: accepted.**
- **The change:** the first clause fires on `B > 0 && B ≥ T0`, instead of the design's `B ≥ T0`. The design's own T6 pin (CHECKS_PASSED) decides this. D21-1's exact second test and the A_se third test are unchanged.
- **Required:**
  - boundary tests: B = T0 = 0 with A = 0 does not fire; A_net ≠ 0 fires; |A_se| > 12·Tf fires; B > 0 with B = T0 fires;
  - a mutation restoring the old clause, which T6 must kill;
  - the deviation from S11G_GUARD.md rev 2.1 recorded in CHANGE_RECORD.
- RV4 checks it. No D1 or V1 round is needed, because the change is confined to the degenerate point.

**3. Receipt coverage on the captured entry: option (b), with conditions.**
- **The edge:** a multi-case captured invocation in which one case is source-selected, and another case is demoted from Passed by a guard. `OrdinaryAttempt::wire` then requires the published outcome to equal the ordinary checks_passed, finalization fails, and the invocation is refused with the blocking `SOURCE_BLOCK_RECOVERY_FINALIZATION_FAILED`.
- **Why (b):** "never refuse" in S11-G is a design rule against losing availability. The hard constraint is never publishing a silently wrong value, and this edge fails closed. The proper fix is a receipt contract change, and it belongs with the finalization owner, not inside a guard slice.
- **Conditions:**
  - **(a) Scope of the residual.** I5 checks whether the same refusal already happens on main through S11-F's `LOAD_CONTRIBUTION_ABSORBED` demotion, and through K-D5's formation-check demotion, in a multi-case captured invocation. Record the actual reach per slice. If S11-F already has it on main, it is an existing residual that S11-G widens, and the record says so.
  - **(b) The test.** The pinning test is labelled a characterization of a known residual, not desired behaviour. It asserts that the invocation is refused with the finalization code, that no case value is published, and that single-case captured invocations are demoted and not refused. Its comment references the owning item.
  - **(c) Ownership.** The "demoted-ordinary" receipt form, including the reader update, is routed to T3's composite `SOURCE_BLOCKS_FINALIZATION_FAILED` item. **It must close before T3 closes**; it is not deferred past T3. It goes into the work graph's T3 row in the next records PR.
  - **(d) Disclosure.** It is disclosed in S11-G's CHANGE_RECORD and PR body as a fail-closed availability residual. **RV4 confirms that it fails closed on every reachable path; any path that publishes a value is BLOCKING.**

The cargo token order is unchanged: I4, then I3, then I5.

## S11-G ruling 3, revision 1: path 2 goes back to the design (ROOT, 2026-09-27)

**What I5 found** (from the code, on `3d844fea4`, before any build):
- **The corrected reach (condition a, accepted).** S11-F's `LOAD_CONTRIBUTION_ABSORBED` and K-D5's formation check have **no** receipt residual. Their Sensitive verdict is on the kernel `StructuralReport` (FK `structural.rs:1400` on main; kd5 `structural.rs:1479-1492`), so the case enters routing and gets a receipt entry. S11-G is the only slice with the residual, by two paths:
  - **Path 1:** a Passed case demoted beside a source-selected case. This needs per-case stiffness, because Sensitive otherwise follows the invocation-wide rcond.
  - **Path 2:** the design's routing gate (`source_eligible … && load_row_finding.is_none()`, S11G_GUARD rev 2.1 §3.5). An already-Sensitive case whose load-row guard fires gets no retained-source attempt, and so no receipt entry. Coverage then fails, and the invocation is refused. I5 reports this as reachable in ordinary models, for example an N05-type case beside a formation-noisy second case.

**The revised ruling.** Ruling 3 accepted option (b) on the premise of a narrow edge. Path 2 would refuse whole invocations in ordinary models, which is an availability regression introduced by S11-G, so it is not an acceptable disclosed residual. **S11-G must not ship with path 2.**
1. **Path 2 goes back to the design.** D1 writes a narrow S11G_GUARD delta, **revision 2.2**, limited to the routing gate. It answers:
   - why the design withholds the retained-source attempt from an already-Sensitive case whose load-row guard fires;
   - whether giving that case the attempt is sound, with the guard's Sensitive demotion and diagnostic applied whatever the attempt's outcome, so that receipt coverage holds with no receipt contract change;
   - if that is unsound, the smallest in-design alternative that avoids refusal. A receipt contract change is out of scope for S11-G.

   V1 then runs a delta check. ROOT selects 2.2 on V1's PASS, by the default rule, and I5 implements it. I5 carries on with everything else meanwhile.
2. **Path 1 stays under ruling 3:** a disclosed, fail-closed residual, owned by the composite `SOURCE_BLOCKS_FINALIZATION_FAILED` item, closing before T3 closes. It is preferable for 2.2's fix to remove it too. The characterization test stays for whatever residual remains after 2.2.
3. **Condition (b) remains a gate.** I5's consumer enumeration continues. If any consumer can present, export or qualify rows from a refused envelope, that is BLOCKING whatever 2.2 does, and it comes to ROOT.
4. **Condition (a), the corrected reach, is accepted.** It is recorded in the work graph's T3 row once I5's test confirms it, correcting the earlier framing.

D1 and V1 do writing and analysis only; no cargo unless the manager grants a slot. The cargo order stays I3, then I5.

**Note (ROOT, 2026-09-27, on D1's revision 2.2 at `3c80158e9`): moving R-b′ before routing is declined for S11-G.**
- D1 recorded, without proposing, the option of moving R-b′ before routing so that it feeds `needs_source_recovery`. That would also remove path 1's R-b′ variant.
- It is declined because it changes R-b′'s semantics, which V1 checked and ROOT selected at 2.1.
- **The R-b′ variant of path 1 stays a disclosed residual under ruling 3.** It was judged narrow at the time. (Corrected 2026-09-27 by the C1 note below: per-case modulus bases are sufficient; they are not shown to be necessary.) (Wording clarified 2026-09-27 on the PR1004 backcheck, B-N1, from "sufficient, not necessary"; the meaning is unchanged.) It fails closed: the captured entry returns `Err` and no envelope. It is characterized by T20. It is owned by the composite `SOURCE_BLOCKS_FINALIZATION_FAILED` item, which closes before T3 closes.
- The finalization item may take the option up, with its own design and check.
- ROOT selects 2.2 on V1's PASS, by the default rule. I5 implements it in its cargo slot, confirming T18 and the T19/T20 multi-basis selection by run.

**Note (ROOT, 2026-09-27, on RV4's interim finding C1 during the PR1003 review): the R-b′ path-1 residual is reachable. Ruling 3 is unchanged.**
- **Construction C1 (RV4):** N05's cantilever with per-case modulus bases. Case A, the cancelling tip torques on the base basis, is Sensitive and selected by recovery. Case B, on a soft basis (E = 1 Pa, G = 0.4 Pa), carries only nodal inputs (tip F_y = 1 N, M_z = 1e-7 N·m).
- **The outcome:** on the captured entry, in both modes, the invocation returns `Err("SOURCE_BLOCKS_FINALIZATION_FAILED")` with no envelope. Case B alone is Passed in its report, and R-b′ demotes it as a true catch (a genuine error of 6.1e-9 relative). With m = 0.5, R-b′ is silent and the invocation returns Ok with a receipt. The typed entry publishes case B as Sensitive.
- **Ruling 3 stands, and moving R-b′ before routing stays declined for S11-G.** C1 uses per-case modulus bases. Single-basis reach through FK's load audit and through K-D5 (V1's N1, the design's E-2) is **not refuted**. The refusal needs a pre-0.4 captured invocation (0.4.0 republishes), a Sensitive source-selected case, and an R-b′ catch on another case. **On that basis the ruling is unchanged:** it fails closed (an `Err` with no envelope; nothing wrong is published), it is confined to the pre-0.4 captured entry, and the finalization item closes it before T3 closes. (Corrected 2026-09-27 on the PR1004 review, S1. The earlier text listed per-case modulus bases as a necessary condition.)
- **Ownership:** the composite `SOURCE_BLOCKS_FINALIZATION_FAILED` item owns it, treats it as **demonstrated** with C1 as its test case, and closes it before T3 closes. Once K-D5 merges, the item re-attempts a single-basis construction through K-D5's per-case formation check.
- **The S11-G repair (tests and records only):** T20 is rebuilt around C1, and the disclosures read "reachable (C1), fail-closed, needs per-case modulus bases and a pre-0.4 captured invocation". RV4 delta-checks that commit. (Corrected 2026-09-27: the accurate wording is "reachable (C1, which uses per-case modulus bases; single-basis reach not refuted), fail-closed, confined to a pre-0.4 captured invocation".)

## Selection: S11-G revision 2.2 (ROOT, 2026-09-27)

**S11G_GUARD revision 2.2 is selected** (`DESIGN_NUMERICS/S11G_GUARD.md`, sha256 `680fecdd…`, commit `3c80158e9`), on V1's delta-2.2 PASS (`REVIEW/S11G_CHECK.md`, sha256 `ec0efce7…`; 1 SHOULD-FIX, 4 NOTE). **D22-1's preferred fix is a binding condition.** 2.2 supersedes 2.1's routing gate. The rest of 2.1 and every earlier S11-G ruling stand.

**What 2.2 does:**
- **G-1:** the routing gate is removed.
- **G-2:** a load-row finding routes like a Sensitive verdict, and `OrdinaryAttempt` records the outcome as sensitive. This is a constructor argument only; V1 confirmed it is not a receipt contract change.
- **G-3:** `decline_formation()`, which V1 found load-bearing in a 0.4.0 subnormal eigen-load corner.

This removes path 2 and path 1's load-row variant. Path 1's R-b′ variant stays under ruling 3.

**1. D22-1 is required, not disclosed.**
- **The problem.** G-2's new attempt on a Passed, guard-fired case charges recovery work up front (SRec:458-472) that neither main nor 2.1 spends. Near the invocation limit, that can cost a later case its selection or refuse the invocation. This is the same class of availability regression ruled out for path 2.
- **The fix I5 implements, per V1's preferred fix:**
  - When main would not attempt the case (report Passed, no `Err`) and the load-row finding is `Some`, record the formation decline **without running the attempt**: a zero-work `RecoveryFailure` plus the same info diagnostic, with existing codes accepted under WORK_LEDGER and FAILURE_CATEGORY.
  - Already-Sensitive guard-fired cases keep the real attempt, so T18's byte-equality with main holds.
- **Required tests:**
  - the invocation budget equals main's on a Passed, guard-fired case;
  - the reader accepts the zero-work entry;
  - a mutation that restores the charged attempt is killed.
- **Records.** D1 adds a short 2.2 erratum (§0.2) with the D22-1 text, and fixes the mislabelled trace check. This is records only and does not block I5 starting on G-1 to G-3. RV4 checks the fix against V1's specification.

**2. The NOTEs:**
- **N1: accepted as information.** The R-b′ residual is reachable through a single basis via FK's load audit (for example an unaudited range row (1e15, −1e15, 1e-300)), and through K-D5 once merged, not only through per-case modulus bases. The residual stays under ruling 3 as recorded. I5 may use the single-basis construction for T19 and T20, which also settles N3 if multi-basis selection does not confirm. The work graph's reach wording is updated to match.
- **N2:** no action.
- **N3:** settled through N1.
- **N4:** T20 is labelled pre-0.4 only. On 0.4.0 the case is republished (CP3 SF-1), not refused; this is recorded.

**3. I5 implements** G-1 to G-3 plus the D22-1 zero-work decline. In its cargo slot it confirms T18, T19 and T20 by run. RV4's basis is 2.2 plus the D22-1 erratum.

**Note on T18–T20 and T6a (ROOT, 2026-09-27, on I5's first runs):**
- **The run evidence.** T18 passes by run in both modes: path 2 is removed, the captured invocation publishes with a receipt, and case B keeps today's integrity bytes. T19 passes by run on a per-case modulus-basis construction: path 1's load-row variant is removed, case B gets the zero-work unsupported entry, and nothing is refused. **T19 settles V1's N3: multi-basis selection works.** T21 and T22 pass.
- **T20: option (a).** I5 could not construct a selected case A beside a case B that R-b′ demotes. It tried three constructions:
  1. A single-basis load-audit row modelled on V1's N1 (V1's row is (1e15, −1e15, 1e-300); I5 used (4e15, −4e15, 1e-300), with PR1002's G = 4e15): recovery refuses case A on the exact radix.
  2. The INPLANE geometry with N05's torsion spring: `UnsupportedBlock { order: 3 }`.
  3. Two disjoint bodies: `UnsupportedBlock { order: 4 }`.
  In each, R-b′ demotes case B without refusal, but no case is selected.
  - T20 is rewritten as a characterization of the actual behaviour: published, no receipt, case B demoted, not refused. A comment names the residual and its owner.
  - RETURN records the three constructions and their outcomes.
  - The residual is disclosed as **"not demonstrated reachable"**. It stays under ruling 3, owned by the composite finalization item, which closes before T3 closes.
  - **Superseded (2026-09-27):** RV4's construction C1 shows the residual is reachable and fails closed. See the C1 note under "S11-G ruling 3, revision 1". T20 is rebuilt around C1, and the disclosure is corrected.
  - **RV4 attempts a construction independently.** If one refuses, RV4 checks that it fails closed (no envelope). It is BLOCKING only if it publishes a value.
  - The work graph notes that K-D5, once merged, may supply a selected case A through its per-case formation check. The finalization item re-attempts the construction then.
- **T6a's pressure run** (the straight-thrust `RoundedProduct` family, refused on fresh solves with `PRESSURE_MODEL_REAUTHOR_REQUIRED`). The manager's direction is upheld: run it through any solve path, including a legacy replay, that reaches the family. Otherwise record the unreachability with file:line evidence, keep a unit-level test of the family's bound, and disclose the deviation for RV4.

## K-D5: the combined-tree gate after S11-G (ROOT, 2026-09-27)

**The pre-S11-G gate passes.** On 3befacff4 plus I3's 8 addendum-4 edits, against the lists at `59fff0d9e`, both entries:
- 888 runs;
- the trusted breach triples are exactly FORMATION_EXCEPTIONS' 7, and none of the former 221 re-breaches;
- RF-SKEW-T-CANT-OFF-122-r1e-04 is the only standing change against P1 (sensitive/needs_recompute on both entries and in both modes).

**After the forward merge of S11-G (main `b24b3d536`),** the combined candidate needs, before RV5:
- the affected suites;
- the T9 fixture diff against main;
- the full both-entry gate against main's **empty** lists, where any trusted breach is a FAIL.

**The ruling: run the full 888, and skip nothing.** It is ordered in two parts, so that RV5 is not held up:
- **Part 1:** every run except the 4 known dense timeouts (RF-LARGE-CHAIN-n01000-ROT and RF-LARGE-TREE-n01000-AX, dense, on both entries). About 50 min. Its result is reported as soon as it finishes.
- **Part 2:** the 4 timeout cases, run immediately after, **with no other cargo on the host.** Contention would bias them toward timing out, which would make the confirmation meaningless. If any of them finishes and publishes, its result is checked against the empty lists like any other run, and a trusted breach is a FAIL.
- **The gate's verdict is the union of both parts,** on the same binary and the same combined head.

**RV5** may be spawned on the combined candidate once part 1, the suites and T9 are green. It reads and traces during part 2, and gets no cargo until part 2 finishes.

Conflict resolution in the forward merge is I3's. Any design question about how K-D5's formation check composes with S11-G's guard (the integrity diagnostic, the no-op rule, routing, D22-1) comes to ROOT.

## K-D5 mutation M31b: equivalence withdrawn (ROOT, 2026-09-27)

**M31b equivalence (`c2042fd9c`) withdrawn on RV5's counterexample, confirmed in Rust. M31b and M31b0 must be killed by a required test before merge.**

- **Why the acceptance was wrong.** It rested on the shift in the maximum-row trigger alone (about 0.002 of the criterion). RV5 showed that the chord error also gives a **first-order** translation error θ·δc at node j on the stiff rows, not only the second-order soft-mode moment. On CSKEW_30_RADIUS_MISMATCH, the row uz at node 1 already carries 0.433 of the criterion from the chord alone. No blame attaches to I3's analysis: it answered the question ROOT asked, which was the wrong question.
- **The confirmation.** RV5 built a `git archive` copy of `2409de83e` (tree `c954590e…`) and ran NI tests with toolchain 1.97.1, against exact references. Admissible models (mismatch ≤ 1e-9, well conditioned) publish Passed with an actual error above the criterion, and the correct check demotes them in both modes:
  - CANT60_PLANAR: actual 1.1019, trigger 2.2037;
  - CANT30_SKEW: actual 1.9005, trigger 3.8010;
  - CANT10_SKEW: actual 6.484;
  - PP_UTM (X 5e6, φ 5°, PP's own centre): actual 1.126;
  - CANT90_PLANAR: actual 0.640, which is condition (b) only.

  **With the M31b patch (and M31b0), none of them demotes, and the published errors are 1.10–6.48 of the criterion.** That is condition (a), a silently wrong Passed value. Near-π admissible models are harmless (actual at most 1.4e-5). The logs are in `REVIEW/_run_records/kd5_review/rust/`.
- **The required tests (ROOT's item 3, final).** They are test-only: the implementation already uses the actual chord (R5-4 §2 step 6).
  - The 60° planar and 30° skew cases, with exact references, on both entries and in both modes. The M31b and M31b0 patches each fail at a behavioural assertion.
  - A product-level demotion test at X 5e6, Y 3.5e6, φ 5°, R 0.3. It publishes SENSITIVE, and the M31b patch makes it publish CHECKS_PASSED.
  - A product-level control at PP_UTM (X 5e5): not demoted, with the published error below half the criterion. The earlier 5e5 figure was an artefact of repr-converted inputs.
- **Records.** K-D5's RETURN §7 and CHANGE_RECORD correct the claim and keep the earlier text marked superseded. **The design claim** — DESIGN.md revision 5a.2 §9 item 31, "build its H from the product's chord … The skew-plane elbow cantilever at k_X = 8.5 (dense) is then missed", with the M31b-equivalence reading of it — **is superseded by this section.** DESIGN.md is hash-pinned by its selection (`fb62ef4a…`), so it is not edited. The chord-only form of mutation 31 is not equivalent, and it is killed by the required tests above.

## F1 split (ROOT, 2026-09-27)

**F1 is split.** The design's §6 places F1 after S11-F, but F1's write set consumes K1 (the kernel sparse assembly) and K2b (formation-time scaling). The kernel order is K2a → K1 → K2b, so F1 as written cannot start yet.
- **F1a starts now** (from main `5ae22926e`, TASK I7): the D-5 evidence line (K-D5's `FormationCheck` rendered as one line in the integrity diagnostic, only when present, composed with S11-G's layout and no-op rule) and SUP-17 (the message text). **SUP-17 committed-byte changes follow the fixture stop rule, and ROOT approves before any regeneration.**
- **F1b waits for K1 and K2b:** the facade sparse wiring and W2 at formation. It is a later TASK, from the F1b part of `TASK_BRIEFS/I7_F1_IMPLEMENTATION.md`.
- **The facade path is now:** S11-F ✓ → S11-G ✓ → F1a → (after K1 and K2b) F1b → F2a (with D2's S-G1) → S-I → F2b per domain → F3.

## F1a: D-5 evidence-line format (ROOT, 2026-09-27)

K-D5's `FormationCheck` record, present only when the check demotes, is rendered as **one evidence line** in the integrity diagnostic. The line is modelled on W2's `range_scaling: force_scale_exponent=<b>; basis=exact power-of-two`. The templates, exactly:

- estimate: `formation_check: reason=estimate; row=<node>:<DOF>; doubled_correction=<2|w_i|>; scale=<max(|q_i|,S*)>; trigger_ratio=<2|w_i|/(1e-9*scale)>`
- unavailable: `formation_check: reason=formation_check_unavailable; detail=<detail>`

**Conditions:**
1. **The `reason=` tokens** are exactly K-D5's existing reason identifiers: the string form K-D5 already defines if there is one, otherwise the snake_case of the enum variant. None is invented. Each one is pinned by a test against the enum, so a rename breaks the test.
2. **Field order is fixed** as above. f64 values use `{:?}`, as the Debug report does, including `inf` (the zero-scale clause). `<node>:<DOF>` reuses `integrity_dof_label`, with its `global_dof=<i>` fallback.
3. **Composition:** the line follows S11-G's guard sentence, if there is one, separated by one space, under S11-G's no-op rule. A case with no `FormationCheck` has diagnostics byte-identical to main, and a test proves it.

**Also agreed:**
- A private `formation_check: Option<FormationCheck>` on `PreviewLinearSolve`, plus a parameter to `append_integrity_report`, is acceptable only if it appears in no serialized or published shape. I7 shows that no serde derive or JSON output picks it up; an unchanged committed-fixture diff is the proof.
- Updating the stale comment at K-D5's `tests/formation_check_runtime.rs:88-91` is fine.
- SUP-17 changes no committed bytes (only product_physics `lib.rs`: the message, a comment and the one pinning test; 52 historical `execution/**` records that nothing reads stay as they are). Nothing is regenerated.

## K2a: product reach and the availability trade-off (ROOT, 2026-09-27) — ruling 1 SUPERSEDED

**Superseded in part (2026-09-27):** ruling 1's premise is refuted at product level: main publishes no wrong value through partial underflow. Rulings 2 and 3 stand, and the benefit of the trade-off is restated. See "K2a product reach: correction (ROOT)" below. The text is kept as the record of the earlier ruling.

**I6's finding.** The formation silent zero is **product-reachable on both entries** through material or section values. There is no lower bound on magnitude anywhere on the input path (at `5ae22926e`):
- **Capture** (captured entry only, `core/serialization/canonical_json/src/lib.rs:94-116`, `validate_checked_value`): values must be finite, and integral floats must satisfy |x| ≤ 2^53 − 1. The typed entry has no checked-JSON step.
- **Unit normalization** (`PP lib.rs:7243`): conversion only.
- **Section** (`PP lib.rs:8441-8450`, `derive_pipe_section`): OD and wall must be finite and > 0, with 2·wall < OD.
- **Section properties** (`straight_pipe/src/lib.rs:393-398`): E, G, A, Iy, Iz and J must be positive and finite.
- **Modulus bases** (`PP lib.rs:8241`, `:8259`): E and G must be finite and > 0.
- **The only floor is geometric:** FK `AXIS_TOLERANCE` = 1e-12 m. This is what stops RF-RANGE LEF-small (DegenerateAxis). LEF-large is refused at capture.

**The ruling:**
1. **I6 adds a product-level partial-underflow case.** Main publishes a *wrong* value there (actual error above 1e-9 against an exact or extended-precision reference computed in the test), and K2a refuses it on both entries and in both modes. This case justifies K2a on the product route, and its precondition proves main's value is wrong and by how much. **If no admissible input yields a wrong published value above 1e-9, I6 stops and reports; it does not force one.** **[Superseded by "K2a product reach: correction (ROOT)" below: main publishes no wrong value through partial underflow where M03 applies, and the wrong-value requirement is withdrawn. See also corrections 2 and 3.]**
2. **The spring-carried case stays as a test:** a 2 m member, OD 1e-6 m, wall 1e-7 m, E 2e11 Pa, G 1e-300 Pa. G·J rounds to 0, the spring carries the torque, and main is accurate to 1e-9.
   - **Its refusal is accepted as K2a's intended interim.** The design forms or publishes no zero or subnormal coefficient, and scaling comes with K2b and F1b.
   - CHANGE_RECORD discloses it plainly as **an availability change on admissible but physically absurd inputs, where main's value was accurate.** **[Reworded by "K2a product reach: correction 3 (ROOT)" below: 1/L-lifted zeros, where main's published value was within its K-D5-limited criterion.]**
   - It is recorded on the K2b and F1b list as a case K2b's scaling should restore.
3. **The missing lower magnitude bound on inputs** is a finding for the input-validation owner. It is routed out of T3, not fixed here.

## S11-G performance finding withdrawn; the method for performance claims (ROOT, 2026-09-27)

- **Withdrawn:** the S11-G performance finding (about +15–20% on dense 1000-member solves), recorded from the K-D5 timing comparison. It compared the S11-G-bearing probes against I3's earlier, non-interleaved pre-S11-G gate runs (413/423 s), which had different builds and host conditions.
- **The settling run:** I3R's interleaved comparison of `72d5ff864` against `b24b3d536` on RF-LARGE-CHAIN-n01000-AX and RF-LARGE-TREE-n01000-ROT, dense, with probes built fresh from `git archive`. It shows S11-G's cost within noise: −0.4% (CHAIN-AX) and +0.9% (TREE-ROT), against a same-binary spread of up to 1.8% (`IMPLEMENTATION/S11G_TIMING/`). The item is removed from the T3-close list. `KD5_MERGE/ADDENDUM_1.md` supersedes the statement in the K-D5 merge record.
- **Method (standing):** performance claims come only from **interleaved runs of probes built fresh from archives,** on a quiet host, with load recorded per run. A comparison against an earlier, non-interleaved run is not evidence.

## K2a product reach: correction (ROOT, 2026-09-27) — bound and M03 clauses SUPERSEDED

**Superseded in part (2026-09-27):** the exact-zero statements below (the ~2^-60 bound, and "checks-passed and accurate" as a general claim) are wrong for 6EI/L² and 12EI/L³. So is the benefit clause's "S11-K's M03 already refuses partial underflow". See "K2a product reach: correction 2 (ROOT)" below. The text is kept as the record of the earlier ruling.

**What is refuted.** Ruling 1 of "K2a: product reach and the availability trade-off" (`81dbd95ac`) assumed a product-level partial-underflow case on which main publishes a *wrong* value. ROOT's directive also quoted a 5.9% error and a nonlinear variant published checks-passed. Both are refuted at product level: they came from formation-level arithmetic, not from product output. ROOT adopted the recommendation before a product run existed. I6's stop, under ruling 1's stop clause, was correct.

**I6's product runs on main** (`5ae22926e`, both entries, both modes, linear and nonlinear gap; evidence in `IMPLEMENTATION/K2A/_run_records/product_reach/`):
- **The drafted case** (5.9% coefficient error) is refused as NUMERICAL_INTEGRITY_UNRESOLVED with `Range("product overflow or underflow")`. Its subnormal coefficients trip M03 `checked_product` (FK `structural.rs:377`). The nonlinear loop's iterations also pass through structural integrity.
- **The all-final-coefficients-normal pair** (L = 2^-39 m, the smallest dyadic length above the 1e-12 m axis tolerance; coefficient errors 4.3e-6 and 8.0e-9) is refused as UNRESOLVED with `Range("arithmetic outside normal range")`.
- **The threshold scan** (E scaled by 2^k): main refuses until the smallest coefficient is at least about 2^-972.
- **Exact zeros** pass M03, because zero entries are exempt. This is the spring-carried case: checks-passed and accurate. **[Superseded by "K2a product reach: correction 2 (ROOT)" below.]**

**Rulings on the stop:**
1. **The partial-underflow wrong-value requirement is withdrawn.** The drafted test is kept, restated truthfully: main refuses the case as UNRESOLVED (M03 `Range`) on both variants and in both modes, and K2a refuses it earlier, by name (`NumericalRange`). Its precondition pins main's actual M03 refusal. The L = 2^-39 pair and the threshold scan stay as evidence in `_run_records`.
2. **The availability trade-off stands, with its benefit restated.** K2a proceeds as designed.
   - **Benefit:** K2a corrects no published value on the product route; S11-K's M03 already refuses partial underflow. **[Superseded by "K2a product reach: correction 2 (ROOT)" below: M03 does not refuse partial underflow in general, and main can publish grossly wrong values (untrusted) that K2a refuses.]** Its value is a formation-layer guarantee that does not depend on M03: a named, earlier refusal, and protection for every consumer of `local_stiffness` that does not pass through M03's check. I6 lists those consumers from its caller scan (for example K-D5's re-formation and curved_bend).
   - **Cost:** refusing exact-zero cases on physically absurd inputs, where main's value was accurate. **[Reworded by "K2a product reach: correction 3 (ROOT)" below: 1/L-lifted zeros, where main's published value was within its K-D5-limited criterion.]**
   - **The bound claim** (an exact zero moves a published value by at most about 2^-60 relative) must be derived step by step in K2a's RETURN, not asserted. K2a's reviewer checks it. **[Superseded by "K2a product reach: correction 2 (ROOT)" below.]**
3. **Lesson (standing):** a claim about product behaviour needs a product run. This is the counterpart of the performance lesson: formation-level or adapter-level arithmetic is not evidence of what the product publishes.

## K2a product reach: correction 2 (ROOT, 2026-09-27) — consequence bullets SUPERSEDED

**Superseded in part (2026-09-27):** the 1/L and 1/L² consequence bullets, and the cost clause's wording, are superseded by "K2a product reach: correction 3 (ROOT)" below. The text is kept as the record of the earlier ruling.

**What is superseded.** Two clauses of "K2a product reach: correction" (`bcc84ee06`), each marked in place:
- **Ruling 2's bound clause:** an exact zero moves a published value by at most about 2^-60 relative.
- **Ruling 2's benefit clause:** "S11-K's M03 already refuses partial underflow".

Both are generalizations from a few probes that were never derived. I6 found both errors while deriving the bound for RETURN, and reported each before it was relied on.

**The derivations.** Evidence: `IMPLEMENTATION/K2A/_run_records/product_reach/` (the zero_probe and reach_lef probes, and NOTES.txt). K2a's reviewer checks both in RETURN.
- **Coefficient lifts in `local_stiffness`** (FK `lib.rs:699-747` at `5ae22926e`). Each coefficient is a binary64 product of modulus and section property, divided by a power of L:

  | Coefficient | Numerator (evaluation order) | Lift |
  |---|---|---|
  | axial EA/L | E·A | 1/L |
  | torsion GJ/L | G·J | 1/L |
  | 4EIy/L, 4EIz/L | (4·E)·I | 1/L |
  | 2EIy/L, 2EIz/L | (2·E)·I | 1/L |
  | 6EIy/L², 6EIz/L² | (6·E)·I | 1/L² |
  | 12EIy/L³, 12EIz/L³ | (12·E)·I | 1/L³ |

  With L ≥ 1e-12 m (the FK axis tolerance), a numerator that rounds to 0, or to the least subnormal 2^-1074, has a true value below about 2^-1074. The true coefficient is then at most about 2^-1074/L^k: about 2^-1034 for k = 1, about 2^-994 for k = 2, and about 2^-955 for k = 3.
- **M03's acceptance floor** (FK `structural.rs:1832-1866`, `transform_roundoff`). The roundoff bound per axis-aligned entry c is about 2·gamma(24)·|c|, and `checked_value` refuses a subnormal bound. So M03 refuses when a *nonzero* entry is below about 2^-974.6. Exact zeros are exempt. I6's threshold scan agrees: refused at 2^-981.8, passed at 2^-971.8.
- **Consequence.**
  - **1/L-lifted coefficients** (EA/L, GJ/L, 4EI/L, 2EI/L): a zeroed or wrongly rounded value has a true value of at most about 2^-1034. Against any retained nonzero entry that passes M03 (at least about 2^-974.6), that is a relative effect of at most about 2^-59.4, below the 1e-9 criterion. This is the only place the ~2^-60 figure holds. **[Superseded by "K2a product reach: correction 3 (ROOT)" below: the floor bounds element entries only, not ground springs.]**
  - **1/L²-lifted coefficients** (6EI/L²): the true value is at most about 2^-994, also below the floor. But against the smallest acceptable retained entry, the relative effect can reach about 2^-19.4 (about 1.4e-6), above the criterion. Whether an admissible input realizes that is **not established**. Manager's check: the manager, not I6, noticed this while recording. RETURN must derive it or probe it, and must not assume main is accurate there. **[Superseded by "K2a product reach: correction 3 (ROOT)" below: the floor bounds element entries only, not ground springs.]**
  - **1/L³-lifted coefficients** (12EI/L³): the true value can reach about 2^-955, above the floor. M03 accepts both an exact zero and a normal but grossly wrong value rounded from a least-subnormal numerator (reach_zero and reach_lef below).
  - **M03 does refuse** a partial underflow that leaves a nonzero subnormal-derived coefficient below the floor.

**Cases on main** (`5ae22926e`, both entries, both modes, linear route and open gap; I6, using the main-built harness binary). Both are one member with L = 2^-39 m, OD 1e-11 m, wall 1e-12 m and G = 1e-100 Pa, with UY at N1 the only free DOF. All inputs are normal binary64.
- **reach_zero:**
  - E = 6.4e-280 Pa, so (12·E)·I (about 2.23e-324) rounds to exactly 0. Main's 12EI/L³ is 0 against a true value of about 3.70e-289.
  - A ground spring of 3.7e-289 N/m carries the load, P = 9.25e-290 N.
  - Main publishes u_y = 250 mm against an exact 125.03 mm, 99.95% wrong.
  - I6's message also quoted the spring as 1.85e-289 N/m. That was a slip, which had reached the drafted test constant; I6 has corrected the constant, and RETURN records the reconciliation.
- **reach_lef** (the LEF-small pattern):
  - E = 9.6e-280 Pa, with no spring and P = 2.05e-289 N.
  - (12·E)·I (about 3.34e-324) rounds to the least subnormal, and (6·E)·I, (4·E)·I and (2·E)·I round to 0.
  - Main's 12EIz/L³ is 8.21e-289, normal but 48% high against a true value of 5.55e-289, and its siblings are exactly 0. M03 accepts.
  - Main publishes u_y = 249.72 mm against an exact 369.55 mm, 32.4% wrong.
- **Main's standing** is the same in both cases:
  - linear route: NUMERICAL_INTEGRITY_SENSITIVE in both modes, from K-D5's re-formation; exact-block recovery is unavailable.
  - open gap: unresolved (RECOVERY_BASIS_UNQUALIFIED), because the nonlinear proof hits the exact-radix range.
- **No trusted route was found on main.** On the linear route before K-D5 (`c61a540ea`), main's standing is **not established**. No claim is made either way.
- **Partial underflow that M03 does refuse:** probes A/B and the drafted partial-underflow case, where a nonzero coefficient is subnormal-derived. Main refuses these as UNRESOLVED (M03 `Range`).

**Rulings:**
1. **Adopt a third product test (reach_zero) and a fourth (reach_lef),** each on both entries and in both modes, with the same structure:
   - in-test arithmetic proving the roundings (exactly 0, and the least subnormal with zero siblings);
   - main's actual standing pinned with the wrong published value: Sensitive on the linear route, and unresolved on the gap route. The linear route is pinned at kernel level through K-D5 on main's K. The gap route is pinned at kernel level if a route exists; otherwise the test cites the main harness records, and RETURN says so;
   - a Fraction reference, with expected values computed from the actual inputs;
   - K2a's refusal by name.
2. **RETURN derives, step by step:** every coefficient with its lift (the table above), the M03 floor at about 2^-974.6, and for each coefficient which zeros or roundings main bounds and which it does not.
3. **The benefit, restated:**
   - M03 refuses partial underflow **only when a nonzero coefficient is subnormal-derived** (below about 2^-974.6).
   - Where a 12EI/L³ numerator rounds to exactly 0, or to the least subnormal with its siblings exactly 0 (the LEF pattern), M03 accepts. Main then publishes a grossly wrong value (99.95% in reach_zero, 32.4% in reach_lef), flagged only downstream: Sensitive through K-D5, or unresolved through the nonlinear proof.
   - K2a refuses all of these at formation, by name, independent of M03, of K-D5 and of the nonlinear proof.
   - K2a also refuses a zero in 6EI/L², where main's accuracy is not established (see the consequence above).
   - **The cost is unchanged:** exact zeros in the 1/L-lifted coefficients (for example the spring-carried GJ/L case), where main was accurate, are now refused. **[Reworded by "K2a product reach: correction 3 (ROOT)" below.]**
4. **A new T3-close item.** K2a covers `local_stiffness` only. Check whether curved-element formation (curved_bend and arc_model), and any other stiffness-forming path, can produce the same unbounded zero or wrong rounding from terms scaled by 1/L² or 1/L³. I6 lists the paths from its caller scan. The item is outside K2a's scope; if reachable, route it to K5 or to its own slice.
5. **Lesson (standing), added to the product-run lesson:** a bound or general claim inferred from a few probes must be **derived**, and the derivation checked by someone other than its author, before a ruling rests on it. ROOT ruled twice on underived generalizations (the ~2^-60 bound, and M03 refusing partial underflow). There is no blame to I6, who found both errors.

## K2a product reach: correction 3 (ROOT, 2026-09-27)

**What is superseded.** Correction 2's 1/L and 1/L² consequence bullets (each marked in place). They bounded a zero's relative effect against "any retained nonzero entry that passes M03". But M03's floor comes from `transform_roundoff` and applies to **element entries only**. Ground springs added to K are not subject to it, and main accepts springs far below the floor. So neither bullet states a bound, and the "~2^-60 holds for 1/L" sentence is withdrawn. The premise was the **manager's**, added while recording correction 2, and falls under lesson 5. I6 found it while probing the 6EI/L² item.

**Rulings:**
1. **What limits trusted publication.** On the linear route it is K-D5's re-formation trigger: an **estimate-based** demotion, not a proved bound on the true error. I6's probes reach_six2 and reach_gj (`IMPLEMENTATION/K2A/_run_records/product_reach/`, NOTES items 6 and 7) are consistent with it: runs published checks-passed stayed within the criterion, and larger errors were demoted to Sensitive. The nonlinear route ended unresolved in every probe. Pre-K-D5 behaviour is not claimed.
2. **No fifth test.** The 6EI/L² item produced no admissible case in which main publishes a trusted value wrong beyond the criterion.
3. **The cost clause, reworded:** K2a refuses 1/L-lifted zeros where main's published value was within its K-D5-limited criterion (for example the spring-carried GJ/L case).
4. **Process: rulings stop restating numeric bounds on this topic.** The authoritative statement of K2a's product reach is the derivation section of I6's K2a RETURN, checked by K2a's independent reviewer. Rulings and the work graph cite it by section and do not restate its figures. Where the figures in corrections 1 and 2 differ from that section, the RETURN section governs.

## K1: spawn timing and no both-entry gate (ROOT, 2026-09-28)

- **Spawn.** K1 (I8) is spawned now, not when K2a's PR opens, because the write sets are disjoint. K2a writes `FK/lib.rs`, `diagnostics` and new tests. K1 writes `FK/structural.rs`, new `FK/structural/sparse.rs`, `sparse_direct` and `structural_adapter.rs`. The worktree is `<wt>/k1`, on branch `codex/piping-k1-20260928` from main `134eefc24`, with target `<wt>/k1-target`.
- **The brief's conditions stand** (`TASK_BRIEFS/I8_K1_IMPLEMENTATION.md`):
  1. K1 does not edit `FK/lib.rs`. If it would need to, it stops, and K1 moves to after K2a's merge.
  2. K1's PR cannot merge before K2a's.
- **K2a-interaction tests.** I8 drafts them read-only against K2a's branch, and lands them after K2a merges and main is merged into K1's branch.
- **No both-entry gate for K1.** K1 is kernel only and changes no published byte. The evidence is the parity tests and T9 (the committed-fixture diff). The gate runs at F1b, when `PP` switches to the sparse representation.
- **Host.** One heavy job at a time. I6 prunes between phases. I8 builds only named crates in its slot. Before deleting anything hash-cited to free disk, ask ROOT.

## K1: the S11 site table for sparse.rs and formation_check.rs (ROOT, 2026-09-28)

- **Issue.** `frame_kernel/tests/s11_site_table.rs` (S11 §4.3 rule 7, R3-3) scans only its fixed SOURCES list. K1's new `FK/structural/sparse.rs` would carry accumulation sites the table cannot see. K-D5's `FK/structural/formation_check.rs` is not in SOURCES either; the T3 manager found this while ruling on K1.
- **sparse.rs: option (a) endorsed.** K1 adds sparse.rs to SOURCES as a declared, additive write-set extension, under five conditions:
  1. no existing row, count or disposition changes;
  2. each sparse.rs site has a disposition under S11 §2.5. Stiffness-side sites are explicit exemptions with reasons, and any load, force or RHS accumulation goes through `ExactAccumulator` (count 0);
  3. a binary64-fold mutant in sparse.rs is killed by the table;
  4. the extension is declared in CHANGE_RECORD and RETURN;
  5. a site that fits no existing disposition stops the work, and I8 reports it.
- **formation_check.rs rides K1's PR as a separate commit,** in the same test file.
  - It adds per-function rows and dispositions, with no change to existing rows. Stiffness and re-formation sites are explicit exemptions, with reasons.
  - A binary64-fold mutant in formation_check.rs must be killed by the table.
  - **If any formation_check.rs site is a load, force or RHS accumulation in plain binary64** (a real rule-7 violation, not a table gap), I8 stops and reports it as a K-D5 finding before touching anything. That would be product code, not a table edit.
  - It is declared in K1's CHANGE_RECORD and RETURN as a separate item. K1's reviewer covers both commits.

## K1: extending the K-D5 and option-(c) source pins for the sparse siblings (ROOT, 2026-09-28)

- **Issue (I8).** K1's brief requires pattern-taking `SA` entries beside the existing ones, plain and with the formation check, and also that existing suites stay unchanged. Two text pins in `nonlinear_integration/src/s11k_tests.rs` encode "exactly one definition":
  - `kd5_nonlinear_sources_name_no_formation_check_entry_point`: one definition of the formation-checked entry, with its plumbing tokens only in that body;
  - `option_c_structural_adapter_legacy_variants_reach_only_binary64_entry_points`: it blanks only the first definition of each listed signature, then forbids exact-entry tokens elsewhere.

  The sparse siblings in `SparseAssemblyEvidence` are second definitions, so they trip both pins. `StructuralSystem::assembled(` also matches inside `SparseStructuralSystem::assembled(`. The invariant the pins protect still holds: the nonlinear loop and the legacy binary64 variants reach no exact or formation-check entry. I8 declined to rename or move code to evade the scans.
- **Approved, in the manager's tightened form.** The edits to `s11k_tests.rs` are additive:
  - blanking and definition counts are scoped by impl block: `AssemblyEvidence`'s originals as today, plus `SparseAssemblyEvidence`'s named siblings only, with exactly one definition of the formation-checked entry in each;
  - tokens match on identifier boundaries;
  - `solve_binary64` and `solve_structural_sparse_binary64` stay checked as today;
  - zero crate calls of the new siblings, and exactly one product call (PP's `solve_preview_reduced_system`) until F1b;
  - **required:** a behavioural pin that the loop reaches neither the dense nor the pattern formation entry;
  - **required mutants, each killed:** an exact-entry call inside `solve_binary64`; a loop call to the pattern formation entry; a third definition in a new impl.
- **No weakening, proved.** The mutants the original pins were written against are re-run, and each must still be killed by the extended pins at a named assertion. These are K-D5's E4 patch (and E1–E3 where they touch these pins), and the option-(c) mutants recorded in S11-K's and K-D5's records. They are listed with their kill sites in K1's RETURN.
- It is declared in K1's CHANGE_RECORD and RETURN as a pin extension. K1's independent reviewer checks it against the pins' original intent.
- **Rejected:** dropping the pattern formation entry from K1. The F1b interface stays.

## K1: the S11-F site test's KERNEL list, and the handoff (ROOT, 2026-09-28)

- **sparse.rs in `product_physics/tests/s11f_site_test.rs`'s KERNEL list: approved,** under the site-table conditions. The addition is additive only, a mutant in sparse.rs is killed by the test, and it is declared as a write-set extension into product_physics **tests only**. It lands as its own hunk.
- **The C3-detect helper** I8 removed from `sparse_direct`: if it was pre-existing code, it is restored and routed to ROOT. If it was I8's own draft, no action is needed. `IMPLEMENTATION/K1/WIP_STATE.md` records which.
- **Handoff.** K1 moves to a local session on the owner's Mac (`HANDOFF_2026-09-28_TO_LOCAL.md`). I8 ran no cargo, and its work is committed as one WIP commit, "K1 WIP (handoff; not reviewed)", on `codex/piping-k1-20260928`. K2a finishes in the cloud session.
