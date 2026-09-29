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
- **M03's acceptance floor** (FK `structural.rs:1832-1866`, `transform_roundoff`). The roundoff bound per axis-aligned entry c is about 2·gamma(24)·|c|, and `checked_value` refuses a subnormal bound. So M03 refuses when a *nonzero* entry is below about 2^-974.6. **[Scoped to axis-aligned members by K2A/RETURN_ADDENDUM_1; see there.]** Exact zeros are exempt. I6's threshold scan agrees: refused at 2^-981.8, passed at 2^-971.8.
- **Consequence.**
  - **1/L-lifted coefficients** (EA/L, GJ/L, 4EI/L, 2EI/L): a zeroed or wrongly rounded value has a true value of at most about 2^-1034. Against any retained nonzero entry that passes M03 (at least about 2^-974.6), that is a relative effect of at most about 2^-59.4, below the 1e-9 criterion. This is the only place the ~2^-60 figure holds. **[Superseded by "K2a product reach: correction 3 (ROOT)" below: the floor bounds element entries only, not ground springs.]**
  - **1/L²-lifted coefficients** (6EI/L²): the true value is at most about 2^-994, also below the floor. But against the smallest acceptable retained entry, the relative effect can reach about 2^-19.4 (about 1.4e-6), above the criterion. Whether an admissible input realizes that is **not established**. Manager's check: the manager, not I6, noticed this while recording. RETURN must derive it or probe it, and must not assume main is accurate there. **[Superseded by "K2a product reach: correction 3 (ROOT)" below: the floor bounds element entries only, not ground springs.]**
  - **1/L³-lifted coefficients** (12EI/L³): the true value can reach about 2^-955, above the floor. M03 accepts both an exact zero and a normal but grossly wrong value rounded from a least-subnormal numerator (reach_zero and reach_lef below).
  - **M03 does refuse** a partial underflow that leaves a nonzero subnormal-derived coefficient below the floor. **[Scoped to axis-aligned members by K2A/RETURN_ADDENDUM_1; see there.]**

**Cases on main** (`5ae22926e`, both entries, both modes, linear route and open gap; I6, using the main-built harness binary). Both are one member with L = 2^-39 m, OD 1e-11 m, wall 1e-12 m and G = 1e-100 Pa, with UY at N1 the only free DOF. All inputs are normal binary64. **[Evidenced on the captured entry only; the typed entry was not run: K2A/RETURN_ADDENDUM_1 §3.]**
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
- **Partial underflow that M03 does refuse:** probes A/B and the drafted partial-underflow case, where a nonzero coefficient is subnormal-derived. Main refuses these as UNRESOLVED (M03 `Range`). **[Scoped to axis-aligned members by K2A/RETURN_ADDENDUM_1; see there.]**

**Rulings:**
1. **Adopt a third product test (reach_zero) and a fourth (reach_lef),** each on both entries and in both modes, with the same structure:
   - in-test arithmetic proving the roundings (exactly 0, and the least subnormal with zero siblings);
   - main's actual standing pinned with the wrong published value: Sensitive on the linear route, and unresolved on the gap route. The linear route is pinned at kernel level through K-D5 on main's K. The gap route is pinned at kernel level if a route exists; otherwise the test cites the main harness records, and RETURN says so;
   - a Fraction reference, with expected values computed from the actual inputs;
   - K2a's refusal by name.
2. **RETURN derives, step by step:** every coefficient with its lift (the table above), the M03 floor at about 2^-974.6, and for each coefficient which zeros or roundings main bounds and which it does not.
3. **The benefit, restated:**
   - M03 refuses partial underflow **only when a nonzero coefficient is subnormal-derived** (below about 2^-974.6). **[Scoped to axis-aligned members by K2A/RETURN_ADDENDUM_1; see there.]**
   - Where a 12EI/L³ numerator rounds to exactly 0, or to the least subnormal with its siblings exactly 0 (the LEF pattern), M03 accepts. Main then publishes a grossly wrong value (99.95% in reach_zero, 32.4% in reach_lef), flagged only downstream: Sensitive through K-D5, or unresolved through the nonlinear proof. **[On skew members M03 also accepts other patterns (subnormal-derived 4EI/L and 2EI/L below the floor): K2A/RETURN_ADDENDUM_1; see there.]**
   - K2a refuses all of these at formation, by name, independent of M03, of K-D5 and of the nonlinear proof.
   - K2a also refuses a zero in 6EI/L², where main's accuracy is not established (see the consequence above).
   - **The cost is unchanged:** exact zeros in the 1/L-lifted coefficients (for example the spring-carried GJ/L case), where main was accurate, are now refused. **[Reworded by "K2a product reach: correction 3 (ROOT)" below.]**
4. **A new T3-close item.** K2a covers `local_stiffness` only. Check whether curved-element formation (curved_bend and arc_model), and any other stiffness-forming path, can produce the same unbounded zero or wrong rounding from terms scaled by 1/L² or 1/L³. I6 lists the paths from its caller scan. The item is outside K2a's scope; if reachable, route it to K5 or to its own slice.
5. **Lesson (standing), added to the product-run lesson:** a bound or general claim inferred from a few probes must be **derived**, and the derivation checked by someone other than its author, before a ruling rests on it. ROOT ruled twice on underived generalizations (the ~2^-60 bound, and M03 refusing partial underflow). There is no blame to I6, who found both errors.

## K2a product reach: correction 3 (ROOT, 2026-09-27)

**What is superseded.** Correction 2's 1/L and 1/L² consequence bullets (each marked in place). They bounded a zero's relative effect against "any retained nonzero entry that passes M03". But M03's floor comes from `transform_roundoff` and applies to **element entries only**. **[Per-entry only for axis-aligned members; on skew members the transformed bounds mix coefficients: K2A/RETURN_ADDENDUM_1.]** Ground springs added to K are not subject to it, and main accepts springs far below the floor. So neither bullet states a bound, and the "~2^-60 holds for 1/L" sentence is withdrawn. The premise was the **manager's**, added while recording correction 2, and falls under lesson 5. I6 found it while probing the 6EI/L² item.

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

## K2a dispositions and the axis-aligned pointer pass (ROOT, 2026-09-28; RV9's N1)

These happened before this entry existed, and are recorded here so that the V1 record is complete.
- **RV7's findings on K2a** (B1, S1–S3 and N1–N5) were answered by K2a's records-only `RETURN_ADDENDUM_1`. N2 was closed by per-site rows. The dispositions and the E2E and sweep standing for `aad23e82d` are in `IMPLEMENTATION/K2A_MERGE/RECORD.md`, "Gates".
- **The bracketed axis-aligned pointers** in corrections 1–3 above were added in `435a26971`, as K2A_REVIEW §8.3 asked. They follow correction 3, ruling 4: cite the RETURN section, and don't restate figures.

## The skew M03 pin: a tests-only follow-up before K2b (ROOT, 2026-09-28)

- **The routing.** K2a's merge record routed the pin to K1's pattern-path M03 tests, with K5 as the fallback. K1 merged without it (ROOT's miss; `K1_MERGE/RECORD.md`).
- **Pinning it now** costs little, and fixes M03's skew scope before K2b and F1b lean on M03. So it becomes its own tests-only slice, run by **I9** (`TASK_BRIEFS/I9_M03_SKEW_PIN.md`).
- **The product evidence:** a better-conditioned skew model (G comparable to E), run on pre-K2a main `134eefc24` and on current main. If pre-K2a main published a trusted wrong value, that is a stop, and it would change K2a's product-reach statement.
- **Gates:** those of a slice PR, with DEC-025 under the owner's Mac decision (`OWNER_DIRECTION.md`, 2026-09-28).

## K2b: kernel only, and the LEF expectation restated (ROOT, 2026-09-28)

- **Kernel only, like K1.**
  - K2b adds W2's scaled evidence, the b-rule and formation-time scaling (SCALE-W) as kernel entries and options. Existing entries keep today's behaviour byte for byte at b = 0.
  - PP's wiring and the `range_scaling:` evidence line are F1b's.
  - So K2b changes no published byte, and **runs no both-entry gate**. The parity tests and T9 are the evidence; the gate runs at F1b.
  - If the design cannot be met without changing an existing entry's behaviour, I10 stops and reports.
- **The LEF expectation, for the kernel half** (resolving part of `I7_F1_IMPLEMENTATION.md` addendum 1):
  - RF-RANGE **LEF-small never reaches formation.** `FrameElement::new` refuses it with `DegenerateAxis` at the 1e-12 m axis tolerance (verified by K1's K2a-interaction tests). The design's "LEF-small solved" is therefore unreachable and is withdrawn for K2b.
  - K2b must instead solve, at kernel level through scaled formation and each accurate to 1e-9 against an exact reference:
    - **LEF-large**, which reaches `local_stiffness` on the typed entry;
    - **K2a's formation-range cases with normal geometry**: reach_zero, reach_lef, the spring-carried G = 1e-300 case (recorded for K2b's scaling to restore), and the partial-underflow case. **[Restated by "K2b: rulings on I10's checkpoint-A stop (ROOT)" below, rulings A and C.]**
  - F1b's brief restates the product level separately before F1b spawns.
- **A plan before code:** I10 returns a plan at checkpoint 0 for ROOT's approval (`TASK_BRIEFS/I10_K2B_IMPLEMENTATION.md`).

## K2b: rulings on I10's checkpoint-0 plan (ROOT, 2026-09-28)

I10 read the design, the rulings and the code at `eb52114e9`, and asked for seven decisions before writing code.

1. **The parity of b: even b (E1). This amends the letter of §4.7 step 3, not its intent.**
   - **The finding.** The gate prepares the system with power-of-two diagonal scaling. With K' = 2^b·K and f' = 2^b·f:
     - for **even** b, the scale exponents shift by b/2 and the prepared matrix is bit-identical. So pivots, rcond, the screens, refinement, the audits and K-D5's EF are invariant, and u is bit-identical.
     - for **odd** b, factors of √2 enter the prepared matrix. Dense Cholesky's u then differs in its last bits, and rcond can flip the √ε Sensitive screen.
   - **Why the design's letter fails.** §4.7's promises ("every scaling is exact"; "displacements are unchanged"; "every M03 screen is componentwise-relative, so the scaled evidence is equivalent") hold only for even b. The floored midpoint of step 3 can be odd.
   - **The rule:**
     - let m = ⌊(b_lo + b_hi)/2⌋, floored toward −∞;
     - b = m if m is even;
     - otherwise m − 1 if that is ≥ b_lo;
     - otherwise m + 1 if that is ≤ b_hi;
     - if the window is a single odd point, refuse with the window reason.
   - **Conditions** (standing lesson: a bound or general claim must be derived and independently checked):
     - I10 writes the invariance derivation step by step in RETURN;
     - K2b's independent reviewer checks it;
     - tests pin it: forced even b gives a bitwise-equal u and an exactly unscaled report on normal models, in both representations and both modes;
     - an "odd midpoint" mutant must be killed.
   - `DESIGN.md` is hash-pinned and is not edited; this ruling supersedes step 3's midpoint for parity only.
2. **The census scope: approved as proposed (F2).**
   - Frames contribute:
     - the exponents of E and G;
     - the **predicted** exponent (the sum of operand exponents) of every intermediate and coefficient that K2a checks and that scales with b: E·A, G·J, k·E, (k·E)·I, and the 10 coefficients.
   - Users, springs, curved block entries and load terms contribute exact exponents.
   - Any subnormal census input is refused as "range: subnormal stiffness or load at formation", per step 2: its bits were lost before the kernel. This includes a frame operand (E, G, A, Iy, Iz, J, L).
   - The census runs only after step 1 fails with a range trigger, so no case solved at b = 0 changes.
   - Coefficients alone would admit b values that K2a's scaled intermediates still refuse (I10's L = 2^-39 example), so the intermediates stay in.
3. **The publication rule: the design's list only (F3).** **[Amended for residual records by "K2b: rulings on I10's checkpoint-A stop (ROOT)" below, ruling B.]**
   - Step 5's outcomes apply to the published **actions, reactions and residual records**, meaning the physical fields of residual and intended rows. For these:
     - normal is exact;
     - subnormal is published with its stated precision;
     - nonzero underflow or overflow makes the case NUMERICAL_INTEGRITY_UNRESOLVED ("range: publication outside binary64"), never flushed.
   - Other force-unit diagnostic fields, such as `contribution_rounding`, are unscaled with the same single-rounding function and published descriptively, as at b = 0. They never make a case unresolved. RETURN lists every field and its treatment.
4. **The load-ledger scaling: L1 approved as a declared write-set extension** (`AssembledForce::force_scaled(&self, b)` in `FK/load_ledger.rs`, one method).
   - Terms are scaled exactly: a Product scales one factor, or splits b across both, whichever stays normal.
   - Each net is re-rounded once at scale.
   - Nothing is pushed to a ledger.
   - The dropped S11-G records are shown by test or scan to be unread by the kernel.
   - It is declared in CHANGE_RECORD and RETURN.
5. **The pins and site tests: approved as declared, additive extensions** on K1's precedent.
   - The files are NI `s11k_tests.rs`, FK `s11_site_table.rs`, and PP `s11f_site_test.rs` (tests only).
   - No existing row, count or disposition changes.
   - The required mutants are each killed: a loop call to a scaled entry, plumbing outside the sibling, and a third definition.
   - The original pins' mutants are re-run, with their kill sites in RETURN. **One that is no longer killed stops the work.**
6. **The SA orchestrator `solve_with_force_scaling`: yes.**
   - It makes steps 1–5 testable at kernel level, and gives F1b one entry.
   - It must have zero product calls until F1b, pinned like K1's siblings.
7. **The refusal types: approved.**
   - New result and error types. There are **no new variants** in `StructuralError` or `FrameKernelError`, so PP's and diagnostics' exhaustive matches are untouched.
   - Refusals keep the step-1 trigger, so K2a's names survive where no feasible b exists.
   - The window text renders §4.7 step 3's template with integer exponents.

**Also recorded for F1b's list (I10's note):** loads that PP forms at b = 0 from out-of-range products (for example a thermal E·A·α·ΔT) have lost bits before the kernel, and the kernel cannot restore a term that underflowed to zero. F1b must form such loads under the chosen b, or refuse them.

## K3: spawn and rulings (ROOT, 2026-09-28)

- **K3 is spawned** as I11, in parallel with K2b (I10), because their write sets are disjoint (`TASK_BRIEFS/I11_K3_IMPLEMENTATION.md`). A TASK drafted the brief, and ROOT reviewed it and ruled on its questions. The rulings are in the brief, under "ROOT rulings for this slice".
- **The key scope finding.** `Wide<2>` is on the product path through K-D5, carrying the D-5 evidence line's EF values and `WideError`'s `Display`. So K3 builds its new widths **beside** `Wide<2>`, and a change to any existing `Wide<2>` result, `Debug` token or `Display` string stops the work.
- **Evidence and gates.** K3 adds no product caller, so the evidence is T9 at 112 of 112 (Mac-only), K-D5's suites unchanged, and a `Display` pin. **The both-entry gate is not run.**
- **K4's needs.** K4 cannot edit `wide.rs`, so K3 also supplies K4's arithmetic: widening and narrowing, TwoSum and TwoProduct, an exact-integer constructor, and per-width work counts. K4 adds the `exact_sum.rs` accessor.

## K2b: rulings on I10's checkpoint-A stop (ROOT, 2026-09-28)

I10 stopped at checkpoint A under ruling 1's stop clause. Two of the four restated product-reach cases were not solved. The evidence is `k2b_checkpoint_a_finding_spring_carried_and_partial_underflow_are_refused`, on `codex/piping-k2b-20260928` at `6ce4d694b`.

**A. Spring-carried (G = 1e-300 Pa): it stays a named refusal. This restates the K2b LEF ruling's second bullet for this case.**
- **The finding.** At the rule's b, formation passes K2a, but M03's contribution audit (`audit_contributions`) refuses the solve with `Range("exact radix loses represented bits")`.
  - GJ/L is about 2^-1082 times the spring it is absorbed into. The audit's measure of that absorbed difference lies below binary64 in the equilibrated units.
  - Force scaling preserves every ratio, so no b changes this. The limit is M03's audit, not formation range.
- **The ruling.** K2b does not change the audit. Changing it would change b = 0 behaviour, which is outside K2b's scope and needs its own design ruling.
  - The case remains refused, with K2a's name `GJ/L: G*J` kept as the trigger, so availability is as under K2a.
  - K2a ruling 2's note that the case was "recorded for K2b's scaling to restore" is answered: **force scaling cannot restore it.**
- **Routed:** W1 (K4 and F2a), whose 64-bit exponent may reach it, is to evaluate it. The case goes on K4's list.

**B. Partial underflow: residual records are published descriptively, with an explicit outcome. This amends checkpoint-0 ruling 3.**
- **The finding.** At b = 898 the solve is accurate to within 1e-9. But one intended-action residual record's physical field (about 2^-1076.5) underflows when unscaled, and ruling 3 refused the case over that record.
- **Why refusing is wrong.**
  - At b = 0 the kernel already publishes these physical fields descriptively, including subnormal or zero values.
  - The gate's basis is the normalized fields and the integer exponents, and those unscale exactly under even b.
  - Refusing an accurate result over a diagnostic record would be stricter than b = 0.
- **The ruling:**
  - step 5's refusal ("range: publication outside binary64") applies to published **actions and reactions**;
  - the physical fields of residual and intended-action rows are unscaled with the same single rounding and **carry an explicit representability outcome**: normal, subnormal with its precision, or underflow or overflow, never a silent zero;
  - they never make a case unresolved.
- This departs from the letter of §4.7 step 5, which lists residual records. The design's intent ("never flushed") is kept, because the outcome is explicit. RETURN states the departure and K2b's reviewer checks it.

**C. LEF-large: the accuracy clause, restated.**
- LEF-large solves **bit-identically to its RF base case, times exact powers of two**, with the base case's standing:
  - CONT is Passed and within 1e-9;
  - CHAIN and SKEW are Sensitive, as their bases are today: flagged, not claimed accurate.
- K2b adds no accuracy beyond the ordinary path. W1 is the route to accuracy on those bases.
- reach_zero, reach_lef, PHYS-R4 (b = 536, against the design's "≈ 500") and partial underflow (under B) are solved and accurate, as required.

## K3: rulings on I11's checkpoint-0 plan (ROOT, 2026-09-28)

I11's plan keeps K3a's `Wide<2>` code untouched. `wide.rs` gains one module line, a doc pointer, one appended `WideError` variant (`OperandPrecision`) with its `Display` arm, and the allowance relabels. All new arithmetic goes in a child module. The rulings on its eight open positions:

1. **Approved: the child module `retained/wide/multi.rs`,** under brief ruling 10. Its test module is declared from `multi.rs` and pointed at `FK/tests/retained_wide_k3/k3_tests.rs`. It stays a unit-test module in FK's own suite, and so in CI. Both are declared in CHANGE_RECORD.
2. **Dead-code labels must name the real consumer.** This amends the brief's blanket "K4 API".
   - Each allowance names the slice that will actually call the item.
   - K4 runs at L ≥ 4, so K3a's `Wide<2>`-only items get no K4 caller:
     - `atan_positive` is labelled "later-slice API (W1c; K3 Q6)";
     - `Wide<2>` accessors with no known consumer are labelled "K3a API, no caller yet (reviewed at T3 close)".
   - The T3-close list gains that review.
3. **Approved:** `from_integer` of a zero magnitude returns +0 whatever the sign flag, following D1 §4.1.2 ("an exact zero is +0.0").
4. **Approved:** the accumulator cross-check allows one more difference. The conversion gives `Normal(−0.0)` for an exact −0 (the Q5 convention); the accumulator never returns −0.0.
5. **Approved:** the literal §7.3-13 and §7.3-16 analogues are kept where they are lossy (at low p), and a scaled variant, whose small term is about 2^-(p+16) of the large one, is added at every p. The fold must lose that small term.
6. **Approved:** about 5 MB of test data, with the brief's 1,000 sample records per stream kept. The digests cover every record.
7. **Q7 stays open until checkpoint A,** when ROOT rules on the measured debug wall times. The bit-serial ÷ and √ (64L + 2 bits, K3a's small-trust-base choice) are approved. Knuth's algorithm D is left to a later optimization with its own vectors.
8. **For K4's brief:** the new core exists at L = 2 only in tests, so K4's p = 128 and 192 run at L = 4 (together with Q8's ceiling question).

## K2a product reach: main's skew standing is now established (ROOT, 2026-09-28)

K2a's `RETURN_ADDENDUM_1` §1.3 left open whether pre-K2a main refuses skew cases downstream in general. I9's product runs answer it, and RV10 reproduced them byte for byte (`IMPLEMENTATION/M03_SKEW_PIN/RETURN.md` §6; `REVIEW/M03_SKEW_PIN_REVIEW.md`). As correction 3, ruling 4 requires, the figures stay in those sections and are not restated here.
- **Pre-K2a main (`134eefc24`) published no trusted value** on the probed skew cases: no checks_passed or numerically_eligible result.
- **It did publish skew cases as Sensitive (untrusted)** at intermediate G/E, some wrong beyond the criterion. So refusal downstream is **not general**: main flagged these cases and did not refuse them. This matches correction 3's statement that K-D5's estimate-based demotion is what limits trusted publication.
- **Current main refuses every probed case at formation, by name** (K2a).
- **The claim "6EI/L² is the limiting coefficient" holds only in RV7's configuration.** RV10's S2 and the scoped §3.4 give the counterexamples. Among them is the torsion analogue of B1 (a subnormal-derived GJ/L), which K2a's `GJ/L: G*J` refusal covers on current main.
- **Added to the T3-close list:** M03's element-entry floor is an axis-aligned argument, and its skew scope in general is a documented limitation. The skew M03 pin (PR #1038) guards it in RV7's configuration only. The item asks whether M03's bound should be made orientation-robust, or whether formation-time checks (K2a) and W1 are the accepted defence. ROOT decides at T3 close.

## K3: Q7, the differential's debug cost (ROOT, 2026-09-28) — SUPERSEDED

**Superseded (2026-09-28)** by "K3: Q7 reversed — the test profile is withdrawn (ROOT)" below. Its premise, that results cannot change with optimization level, is false for `powi` over compile-time constants. The text is kept as the record of the earlier ruling.

- **The measurement.** I11's checkpoint A was taken on the Mac, as observations and not a performance claim. With K3's 10^6-operation streams per precision (the §4.11 minimum), FK's full debug test suite takes about 232 s, against about 6 s before. The L = 16 streams dominate: the bit-serial ÷ and √ over 1,026 steps. Hosted CI runners are slower, and the numerical job has a 45-minute budget across all 39 manifests.
- **The ruling:** add a `[profile.test]` section to `FK/Cargo.toml` as a declared K3 change, and keep every stream at full count in the default suite. §7.4's placement is kept, with no `#[ignore]` and no reduced count.
  - Use the smallest `opt-level` (1, else 2) that brings FK's suite to about a minute on the Mac.
  - Set `debug-assertions = true` and `overflow-checks = true` explicitly, so that test semantics do not change.
  - There is no lockfile change.
- **Why the results cannot change.** Rust gives IEEE-754 binary64 semantics at every optimization level: no fast-math, no reassociation, and no FMA contraction by default. The K3 arithmetic is integer-only. FK's own bit-exact tests (K3a's vectors, the K1 parity, K-D5's and K2a's pins) prove the claim on this tree.
- **Conditions, checked at B:**
  - FK's full suite passes with the same test list under the new profile;
  - K3a's and K-D5's pins are unchanged;
  - the measured wall times at each opt-level tried are recorded;
  - hosted CI's numerical job time on K3's PR is recorded in the merge record.
- The design's §7.4 placement stands. K3's reviewer checks the profile change.

## K2b: the b-rule's window misses the solve's range; "no third attempt" is not equivalent (ROOT, 2026-09-28)

- **ROOT's framing was wrong.** At checkpoint C, ROOT asked I10 to record the "third attempt" mutant as "equivalent by construction, pending the reviewer's check". I10 derived three mechanisms that do make a refusal persist across the window: formation (census margins), ratio invariance (audit-limited cases), and subnormal-u persistence. It then **disproved** the general claim.
  - The gate's equilibrated right-hand side, the triangular-solve intermediates and y scale by 2^(b/2). They are neither b-invariant nor covered by the census, and the midpoint rule does not centre them.
  - **The counterexample** (a scratch probe on `8e6698282`; `IMPLEMENTATION/K2B/_run_records/retry_probe/`): an axial chain with E = 2^440, and UX loads of 2^-1010 N and 1 N.
    - The rule chooses b = 312 and refuses the case ("range: scaled evaluation outside normal range").
    - Forced b = 500 and 572 give Passed, within 1e-9 of the exact reference.
  - Its reach needs a load about 1,450 binary orders below the stiffness in the same row, so realistic reach is nil. It fails safe: a named refusal, and nothing wrong is published.
  - The standing lesson applied. The derivation was required rather than assumed, and it found the error before any ruling rested on it.
- **Rulings:**
  1. **K2b keeps §4.7 step 4 as designed:** one scaled evaluation, and no third attempt.
  2. **Pin the case with a test** (tests only, added after checkpoint C), labelled a **documented limitation of the b-rule, not desired behaviour**, citing this section. It asserts:
     - the named refusal at the rule's b, in both modes and both representations;
     - that a forced b in the window solves it within 1e-9.

     The "third attempt" mutant (a retry within the window) must then be killed behaviourally. RETURN §13.3 and the mutation table are updated.
  3. **A design finding, on the T3-close list:** the b-rule's census omits the solve's right-hand-side and intermediate range. Candidate refinements are a per-row load-to-stiffness term in the census, or a bounded, deterministic retry. **F1b's brief must consider it** before F1b wires b into the product. No refinement lands in K2b.
- **The even-b derivation's premise** (RETURN §4) is also recorded: every rounded operation is zero or normal at both scales. The dense Cholesky factor and the triangular solves have no range checks, so the kernel does not enforce the premise. **[Corrected by "K2b: rulings on RV11's review" below (RV11-3): the dense Cholesky, the triangular solves and the skyline LDLᵀ are range-checked at the head. What was unchecked is the reaction and member-action arithmetic, and the rounding boundary into the normal range.]** The forced-b tests pin its consequences. K2b's reviewer checks the derivation and states whether the premise's scope is adequately disclosed.

## K3: Q7 reversed — the test profile is withdrawn (ROOT, 2026-09-28)

- **What happened.** ROOT merged main (with the skew M03 pin) into K3 and ran FK's suite under K3's `[profile.test] opt-level = 1`. One test failed: `m03_skew_scope.rs::m03_skew_pin_rv7_cases_outcomes_and_figures` (2EI/L's exact error came out 1.1364e-13 against the pinned 1.1378e-13).
  - The test computes the pipe's second moment from compile-time constants with `od.powi(4)`.
  - At opt-level ≥ 1, LLVM constant-folds `powi` through the host's `pow`. At opt-level 0 it calls the runtime repeated multiplication. The two differ by an ulp, and the pinned figure moved.
- **ROOT's premise was false.** The Q7 ruling said "Rust gives IEEE-754 binary64 semantics at every optimization level … so the results cannot change". That holds for + − × ÷ √. It does **not** hold for functions of unspecified precision, such as `powi`, when the compiler evaluates them at compile time.
  - ROOT asserted the claim without deriving or checking it: a breach of the standing lesson on general claims. The skew pin, which was written and reviewed at opt-level 0, caught it.
  - **The product is not affected.** Its `powi` inputs are runtime data, and T9's release-built outputs have always matched the debug-built tests.
- **The measurement that settles it.** Hosted CI's whole "Numerical cargo suite" job (all 39 manifests) takes about 4–5.5 minutes on current main (runs 36380608240 and 36391476996), against its 45-minute budget. K3's full-count streams at opt-level 0 fit easily. The profile was never needed for CI, and it adds a class of opt-level-dependent test behaviour that FK alone would carry.
- **The ruling:**
  1. **K3 reverts the `[profile.test]` section** in `FK/Cargo.toml`. FK's tests build at opt-level 0, like every other crate's.
  2. **Every stream stays at full count** in the default suite (§4.11, §7.4).
  3. **The P1/P2 guard test stays.** It still asserts that overflow checks and debug assertions are on in FK's test build.
  4. **The evidence is re-run at opt-level 0 on the merged tree:**
     - FK's full suite, including the skew pin's tests;
     - the full mutation table, with the NONE control first;
     - the measured debug wall time.
  5. **Hosted CI's numerical-job time** on K3's PR goes in the merge record.
- **A lesson, recorded:** a test must not depend on how the compiler evaluates a function of unspecified precision. Tests that need exact section properties should compute them with explicit, ordered arithmetic on runtime values, or on `black_box`ed constants. This is a T3-close note on test hygiene.

## K2b: rulings on RV11's review (ROOT, 2026-09-28)

RV11 reviewed PR #1040 at `087b3a088`: **FAIL**, with 1 BLOCKING, 3 SHOULD-FIX and 7 NOTE findings (`REVIEW/K2B_REVIEW.md`). Its checks of the kernel-only claim (a lexer scan, and the b = 0 probe re-run at 439/439), the 13-step even-b derivation, the census and the pin, and ruling B all hold. The rulings:

1. **RV11-1 (BLOCKING): reactions and member actions at scale must never publish a wrong value labelled Normal.**
   - `SparseStiffness::force_scaled_reactions` forms K′·u at 2^b with the unchecked binary64 `multiply`, then unscales. RV11's probe F-A2 publishes reactions of 0, labelled Normal, where today's E12 gives ±1.38e-300 N.
   - **Fix, fail-closed:** every product and partial sum of the reaction at scale is checked. A value that leaves the normal range (and is not an exact zero) refuses the reaction with the step-5 refusal. It is never published as Normal. An exact alternative (an `ExactAccumulator` sum of exact products) is acceptable if the implementer prefers it and pins it the same way.
   - **K2b also provides F1b a kernel function for member actions at scale, built the same way,** and RETURN §15's recipe points to it. The recipe must no longer publish end shears through unchecked arithmetic.
   - **Tests:** F-A2 and its 2^80-larger variant must be refused, or be correct with a normal outcome. They must never be a wrong Normal. Add a mutant that restores the unchecked multiply; it must be killed.
2. **RV11-2 (SHOULD-FIX, fixed in K2b):** `exact_normal_scaling` must refuse a result that is normal only because it rounded up from the subnormal range: the scaled value must be exact. Add a test, and a mutant.
3. **RV11-3 (SHOULD-FIX, records):** RETURN §4's premise disclosure and §6's "exact sum of K′·u" are corrected to what the code does. ROOT's own sentence at `97000ab9f` is corrected in place above.
4. **RV11-4 (SHOULD-FIX, tests):** tests pin the census scope so that both surviving mutants are killed: `Product` recorded at e(x) alone, and curved slots left out of the census. Their effect is availability only, but ruling 2 defines the scope and a test must hold it.
5. **Process:**
   - I10 fixes these on the K2b branch;
   - it re-runs the affected evidence: the targeted tests, the suites against the Mac baseline, the b = 0 probe, T9, and the mutation table's affected rows plus the new mutants;
   - it adds a RETURN addendum;
   - RV11 then delta-checks the fix commits.
   - DEC-025 is re-run on the new head on a quieter host. At `087b3a088`, `App.test.tsx`'s workspace render timed out at 30 s under host load (load average above 8, with two reviewers building). K2b changes no TypeScript; the timeout is not raised, and the surface is re-run.

## K4: spawn and rulings (ROOT, 2026-09-28)

- **K4 (W1a, the retained-precision kernel method) is spawned as I12** from main `e7d930d49`, where K1, K3 and K2b are merged. A TASK drafted the brief (`TASK_BRIEFS/I12_K4_IMPLEMENTATION.md`); ROOT reviewed it, and the rulings on its questions Q1–Q12 are in the brief. In short:
  - kernel only, with no gate;
  - the multi-term exact sum in K4's own file;
  - the `exact_sum.rs` accessor approved;
  - the ceiling: no p + 64 residual, with the argument derived and checked;
  - budgets: the mechanism only, with no limits, and **F2a does not merge without ROOT's limits** from K6 and V-P; **[Amended: from K6 and V-K, with V-P revisiting them after F2a. See "K4: Q5 amended" below (RV13-S3).]**
  - a kernel `DirectionalSpring`;
  - RCM ported into `factor.rs`;
  - the site-table extension;
  - the export by the first consumer;
  - canonical encodings, hashed downstream;
  - K3's outcome type;
  - one slice.
- **Design text made stale by K1, K2b and K3** [or found inconsistent within the design (RV13-N9)], recorded here as rulings. `DESIGN.md` stays hash-pinned.
  1. §4.1.1's `wide.rs` is now `wide.rs` (K3a, L = 2) plus `wide/multi.rs` (K3, L = 4, 8 and 16). K4's p = 128 and 192 run at L = 4.
  2. §4.1.2's "one accumulation discipline … shared with `FK/exact_sum.rs`" holds for binary64 terms only. p-bit sums use K4's primitive.
  3. §4.1.2 item 5's projection is K3's `from_integer` plus K4's netting accessor.
  4. §4.1.3's "RCM … shared with the binary64 sparse path" cannot be shared across crates. K4 ports it, and V-K tests the equality.
  5. §4.1.4 with §4.1.6: there is no p + 64 residual at the 1024 ceiling (Q4).
  6. §4.1.7's "No implementing slice ships without them" binds the product-wiring slice (F2a), per C4's measurement order. **[Amended: the limits come from K6 and V-K, which precede F2a; V-P follows F2a and revisits them. See "K4: Q5 amended" below.]**
  7. §4.1.1's springs are extended with the kernel-only `DirectionalSpring`, for §4.10's directional kernel springs.
  8. §5 item 7's outcomes are carried by two types, K2b's `Representability` and K3's `Binary64Outcome`. F2a unifies them, including the zero conventions.
  9. MOD-D's export gap: the first consumer outside FK adds the `pub use`.
  10. §4.10 and §7.1's "LEF-small … solved": the ordinary route refuses it with `DegenerateAxis` (K2b ruling). Whether W1's source admits LEF-small is V-K's question; RF-RANGE is not in K4's row.

## K2b: ROOT's decisions at RV11's delta checks (ROOT, 2026-09-28; recorded late, RV13-S2)

These decisions were stated in ROOT's resume messages to I10. [Correction (RV13-D1): the committed evidence puts decisions 1 and 2 at RV11's resume; only item 3's rulings went to I10.] Until RV13's review of records PR #1042 found the gap, they were recorded only in K2b's RETURN (addenda 1 and 2) and in RV11's review. They are recorded here with their original dates. Nothing in them is new.
1. **The reaction check applies at every b, b = 0 included** (`IMPLEMENTATION/K2B/RETURN.md`, "The check applies at every b", where I10 left it "ROOT's call"; `REVIEW/K2B_REVIEW.md`, "ROOT's two decisions, against the code").
   - Where today's E12 would flush a subnormal product at b = 0, `force_scaled_reactions` refuses. F1b may keep today's E12 at b = 0 for byte identity.
   - RV11D-N2: the check is stricter than flushing requires, which costs availability only. **F1b's gate measures that cost.**
2. **`force_scaled_end_actions` stays off the pin list,** covered indirectly through the pinned `ForceScale` token. **Reversed** after RV11D-N1 (the `Default::default()` evasion, `RV11D-PIN-ACTIONS-EVASION`).
   - Both publication helpers, `force_scaled_end_actions` and `force_scaled_spring_action`, are named in `FORCE_SCALED_ENTRY_POINTS`.
   - The pin's doc records the text-pin limit beside RV8-N4's (`RETURN.md` A2.4).
3. **The rulings on RV11's first delta check** (ROOT's message; `RETURN.md`, addendum 2):
   - RV11D-1: a checked spring-action helper, pinned with probe F-S and a mutant;
   - RV11D-2: tests that kill the two surviving action mutants;
   - N1: both helpers on the pin list (item 2's reversal);
   - N3: the doc sentence corrected;
   - N2: recorded, with no change.
4. **For F1b,** in addition to `K2B_MERGE/RECORD.md`'s list: RV11D-N2 (item 1), and RV11D-N3 (no end-action variant takes load terms at scale).

## K4: Q5 amended (ROOT, 2026-09-28; RV13-S3)

- **The error.** The Q5 ruling said F2a does not merge without ROOT's limits, "set from the K6 and V-P measurements (C4)". But V-P follows F2a in DESIGN §6 (the V-P row, "after F1 and F2a") and in the selected order. As written, F2a could never merge.
- **Amended (RV13's option (a)):**
  - F2a does not merge without ROOT's per-case and per-invocation limits.
  - ROOT sets them from K6's measurements and V-K's kernel-lane runs, both of which precede F2a, together with K4's deterministic work counts. §4.1.7 asks for "W3/W5 measurements", which this admits.
  - V-P's product-lane measurements follow F2a, and confirm or revise the limits. A revision is its own ruling.
  - [Added (RV13-D2): this amendment creates the dependency "K6 and V-K before F2a's merge"; the selected order did not state it. It departs from C4's and D-8's "from the K6 and V-P measurements" for W1's budgets, and from nothing else.]
- Item 6 of K4's stale-design list, and the brief's Q5 ruling (`TASK_BRIEFS/I12_K4_IMPLEMENTATION.md`), carry bracketed pointers here. K4's own scope is unaffected: it ships the mechanism only.

## K4: rulings on I12's checkpoint-0 plan (ROOT, 2026-09-28)

I12's plan (`<wt>/scratch/i12/CHECKPOINT0_PLAN.md`, sha256 `e1253faa…`, 591 lines) is **approved as written**, with the rulings below. The plan and its scratch probes go into K4's `_run_records/` at checkpoint D.
- **Approved as positions or refinements,** each recorded in RETURN with the design's words:
  - the multi-term sum, `retained/wide_sum.rs`: stack magnitudes, a span limit of 8,128 bits that refuses, and one rounding through `from_integer`;
  - values only ever widen, and each lower-precision quantity is rounded once from its exact sum;
  - the pivot screen, the residual gate and the stop rule are decided exactly, with no binary64 γ;
  - directed roundings of the evidence ratios (the summary ratios upward, the pivot margin downward);
  - radix equilibration as in M03, with rcond on the equilibrated K;
  - RCM on the structural free–free pattern. V-K's equality test feeds both paths the same adjacency;
  - the negative-energy allowance;
  - frame dot products formed exactly. This is a refinement of D1's "every operation rounded to p": it is strictly more accurate, with a single rounding per step;
  - the brief's item C holds at p = 53 on normal results only, with the exact-projection form added;
  - item E's K-D5 cross-check is a test-only port;
  - factor reuse. Each case's limit counts the full shared work, and the invocation meter counts it once;
  - the budget API (required parameters, no `Default`, no numbers);
  - the canonical encodings;
  - O3, ids on members, springs, stations and groups;
  - O4, a span refusal is terminal and the maximum span is recorded;
  - O6, the K4-M16 control;
  - O7, the combination rules;
  - O9, unpublishable rows are listed, excluded from S\* and the classification, and F2a decides their standing;
  - O10;
  - O12, M03's coalesced denominator. [Reversed: see "D1 revision 5a.3: rulings on V4's verification", V4-S3 (RV16-S4).]
- **O1: Q6 amended. A body with a directional ground that does not span R³ skips the geometric witness; it is not refused.**
  - **The problem.** As ruled, Q6 refused a body whose directional springs of one kind do not span R³. That refuses RF-SKEW-T-PIN-AX's six cases. Each is restrained: both nodes are translation-pinned, and one rotational spring lies along the member axis. So §4.10's not-covered set would change, which is a stop item in the brief.
  - **Amended rule:**
    - Per node and kind, the directional springs **together with that node's global-axis springs and rigid DOFs of the kind** count as grounding the kind fully when their directions span R³, decided exactly. The kind is then passed to `assess_rigid_body` as grounded at that node.
    - A body with any remaining directional ground that does not span is **not assessed geometrically**: no witness is sought, and none of `assess_rigid_body`'s outcomes is used for it. It proceeds as `NumericallyUnresolved` does.
    - The attempt evidence records it: geometry not assessed, a non-spanning directional ground at node N, kind K.
  - **Why this is safe.** "A witnessed mechanism is refused and never escalated" (§4.1.3) is unchanged. Such a body is simply not witnessed. A true mechanism there fails the pivot screen or the rcond test at every p, or is rejected by the stop rule, so it ends unresolved and is never published. The design already relies on that for `NumericallyUnresolved` bodies. The cost is work only.
  - **Scope.** Directional springs are kernel-only; the product never builds one, so the product's geometry-first path is unchanged. RF-SKEW is 36 compared.
  - **Routed:** a full geometric treatment of partial directional grounds belongs to W4/K5's generalized assessment (§4.9). RETURN derives the safety argument above, and K4's reviewer checks it.
- **O2: the optional `SupportGroup` list is approved,** so that the per-support magnitude rows and `reaction_resultant` are formed at p and checked by the stop rule. F2a maps the product's supports onto groups.
- **O5: an evidence-level kill of K4-M11 suffices, under two conditions.**
  1. The killing assertion is on K4's returned attempt evidence, its correction count and golden work. F2a publishes these as evidence; they are not internal state.
  2. RETURN derives the published-value equivalence step by step, and the reviewer checks it independently.

  This follows the M31b lesson: an equivalence is accepted only when derived and checked, never by assertion.
- **O8: approved.** The generator emulates the method bit for bit for three tiny cases (N05 and N06 at 128 and 256, and one skew member at 128), and pins their retained-state sha256. If the emulation proves disproportionate at A, stop and report before falling back to Rust-produced goldens; do not fall back silently.
- **O11: measure at A.** If FK's debug suite grows by more than 20 minutes at opt-level 0, report it before B. Tests are never reduced or ignored. Splitting across test functions for parallelism is fine.
- **Q5, amended** (this file, "K4: Q5 amended"): K4 is unaffected. It ships the mechanism only.
- **The ceiling argument** (§11's two routes) goes into RETURN step by step, naming the uncertified step in each route. K4's independent reviewer checks it, as ruled under Q4.

## F1b: spawn and rulings (ROOT, 2026-09-28)

- **F1b (facade: the sparse wiring, the dense-scrutiny guard, and W2 at formation in the product) is spawned as I13** from main `e7d930d49`, where K1 and K2b are merged.
  - A TASK drafted the brief (`TASK_BRIEFS/I13_F1B_IMPLEMENTATION.md`); ROOT reviewed it and ruled on Q1–Q14 in the brief. In short:
    - the nonlinear loop stays out, and goes to T5;
    - W2 engages after the ordinary attempt and after exact-block, on linear invocations only;
    - at b ≠ 0 it admits only frames, ground springs, restraints, prescribed motion and nodal loads;
    - today's publication functions at b = 0;
    - R-b′'s Sensitive demotion is accepted;
    - W2 refusals are `NUMERICAL_INTEGRITY_UNRESOLVED`, with the exact template fixed at checkpoint 0;
    - the `range_scaling:` line is bounded by S11-G's `NAMED` limit;
    - a provisional dense-scrutiny ceiling (6 GiB estimated) and no sparse ceiling;
    - source recovery's dense view for n ≤ 256 only;
    - PHYS-R4 restated as a named refusal, if A2 confirms it;
    - the pin and site-test edits under K1's adapted conditions;
    - the b-rule as merged;
    - ROOT supplies the Mac gate baseline;
    - one slice.
  - K4 (I12) runs in parallel; their write sets are disjoint.
- **Design text made stale or found inconsistent,** recorded here as rulings. `DESIGN.md` stays hash-pinned.
  1. **The F1 row's and §4.8's line citations have drifted.** The assembly is now `PP:1826`, `:1957` and `:2276`; the reduction `:2726-2735`; the reactions `:3119`; `solve_preview_reduced_system` `:4392`; `dense_fallback_message` `:4490` (all on `e7d930d49`).
  2. **"The nonlinear loop moves to sparse"** (the F1 row, §4.6 and §4.8) conflicts with the selection's hard constraint, and K1's pattern path has no binary64 binding. The move goes to T5 (Q1).
  3. **"LEF-small and LEF-large solved"** (the F1 row, §4.10 and §7.1) is restated per entry in the brief's Scope §6:
     - LEF-small is refused at model build on both entries;
     - LEF-large is refused at capture on the captured entry;
     - on the typed entry, W2 publishes LEF-large Sensitive (Q5).
  4. **§4.7's "the public fixture then passes the evidence stage"** is unreachable with K2b's census, because the fixture's own exact-pressure operand is subnormal at formation. It is restated as a named refusal once A2 confirms it (Q10). The scaled formation of pressure operands goes on the T3-close list.
  5. **§4.7's admitted range assumes that loads are exact inputs.** PP forms some loads at b = 0 from products, and a term formed to exactly zero is invisible to the census. So W2 admits only authored nodal loads at b ≠ 0 (Q3).
  6. **§4.7's "the case stays `NUMERICAL_INTEGRITY_UNRESOLVED`"** is wrong for K2a formation refusals, which main publishes as `SOLVER_SYSTEM_BLOCKED` at the invocation. F1b publishes W2 refusals per case as `NUMERICAL_INTEGRITY_UNRESOLVED` (Q6).
  7. **§4.7's evidence line and §5 item 6** give no template for subnormal precision or record outcomes, and no composition with F1a's line (Q7).
  8. **§4.7's mechanism** (a private exponent on `AssemblyEvidence`, and a scaling `solve()`) was implemented by K2b as force-scaled siblings plus an orchestrator.
  9. **§4.8's "ROOT picks both from measurement," with C4:** V-P follows F1b, and K6 has not run. The ceilings are provisional (Q8).
  10. **§4.8's dense view for n ≤ 256** also concerns `source_receipt.rs`'s replay, which assembles its own dense K. That file is unchanged (Q9).
  11. **The design does not order W2 against exact-block,** and §4.4's coexistence rule covers W1 only. W2 runs after exact-block (Q2).
- **Added to the T3-close list:**
  - PHYS-R4 with pressure (the scaled formation of exact-pressure operands);
  - a scale-aware R-b′ bound (Q5(b)), which is also an input to F2a;
  - W2's coverage beyond nodal loads (Q3(b)), if a real case needs it.
- **The owner is told** of the provisional dense-scrutiny ceiling (Q8), which is a new refusal class for very large dense-scrutiny models, and of the PHYS-R4 restatement.

## F1b: rulings on I13's checkpoint-0 plan (ROOT, 2026-09-28)

I13's plan (`<wt>/scratch/i13/CHECKPOINT0_PLAN.md`, sha256 `ba90de63…`, 856 lines) is **approved**, with the rulings below. It goes into F1b's `_run_records/` at checkpoint D.
- **Approved as planned:**
  - the A1/A2 split: A1 is a pure refactor at b = 0 with no deferral, byte-identical everywhere;
  - `BasisStiffness` and the case loop (§2);
  - W2 only in the final failure arm;
  - the receipt's `OrdinaryAttempt` formed after W2 (t10b's single `passed(` call kept);
  - PP's two-arm range classifier, pinned against the orchestrator;
  - the publication path table (§3.4);
  - the guard's constant of 96 bytes per n² entry, as counted in the code. At 6 GiB the provisional ceiling is 8,192 DOFs, that is 1,364 chain members. OQ12's placement and input are also approved;
  - Q9's dense view with a shared `DENSE_SOURCE_DOF_LIMIT`;
  - the RV11D-N2 method (§13.3);
  - the C tests' sizes and bounds;
  - derivations D1–D9, each written in full in RETURN and checked independently;
  - the evidence-line template (§4.1).
- **OQ1: c2, not c1.** The census scope, meaning which terms are counted, is part of the b-rule. Five loops copied from SA's private `force_scale_census` would be a replicated rule, and K5 will edit SA.
  - The template prints `force_scale_exponent=<b>` only where the orchestrator's outcome carries b. Elsewhere it prints `range_scaling: attempted` with no b field, or `none` where no b exists (steps 2–3).
  - RV11-N2's b on the step-4 and non-range paths waits for an SA change that puts b on the refusal: K5 or F2a, recorded on F2a's input list.
- **OQ2: option B,** a zero-work named decline for a formation-range case. D2 derives that nothing is lost, and the reviewer checks it.
- **OQ3:** the `solve_ordinary(input, limits, attempt_scale)` wrapper, the one-line t10b anchor amendment and the lexer pin are approved.
- **OQ4:** approved. A `Structural(e)` failure at the chosen b keeps `append_integrity_failure`'s code mapping, under the W2 template. Formation and census errors are `NUMERICAL_INTEGRITY_UNRESOLVED`.
- **OQ5:** approved. The DEC-050/053 observation lanes do not run at b ≠ 0, and the mode row's observation fields are published as not observed, disclosed.
- **OQ6:** approved and disclosed. Rows derived from published values stay today's binary64, and a non-finite result keeps today's refusal.
- **OQ7:** confirmed. Admission runs after the orchestrator.
- **OQ8:** approved as a declared write-set extension: `formation_guard.rs`'s `const NAMED` becomes `pub(crate) const NAMED`. It is a visibility change only, and S11-G's tests must be unchanged.
- **OQ9:** confirmed. Call-site-only updates in `src/source_receipt/load_state_join_tests.rs` and `load_state_tests.rs`; any assertion change stops the work.
- **OQ10, OQ11, OQ14 and OQ15:** approved.
- **OQ13, narrowed.** At b ≠ 0 a zero nodal load term is refused only when its authored value is nonzero, that is when formation at b = 0 produced a zero from a nonzero input. An authored zero is admitted.
  - If the authored value is not available at that point, refuse every zero term and disclose it.
  - Subnormal formed terms are already refused by the census.
- **The refusal template (§4.2)** is fixed as proposed, with OQ1's c2.
- **The C1 list** (28 runs, proposed from G1's Mac base data) is ruled at A2 with the product-run table, as the brief says.
- **The gate's comparison scope.** G1's baseline (`GATE_BASELINE_MAC_E7D930D49/`) keeps P1's `run.envelope` summary bytes, not the full `MechanicsEnvelope`.
  - At A2, I13 states exactly which published fields that summary covers and which it omits, including diagnostic messages, receipts and rows.
  - If it omits published bytes, I13 proposes a probe variant that also emits the sha256 of the full serialized envelope, without changing `run_one`'s classification. ROOT then has G1 re-run the base's part 1 with the variant, which takes about 5 minutes, before B.

## K5: spawn and rulings (ROOT, 2026-09-28)

- **K5 (W4: the constrained-body witness and the curved rule) is spawned as I14** from main `24dea2dae`, whose product tree equals `e7d930d49`'s.
  - A TASK drafted the brief (`TASK_BRIEFS/I14_K5_IMPLEMENTATION.md`); ROOT's rulings on Q1–Q11 are in the brief. In short:
    - W4 only in the four `selected` formation-checked branches;
    - curved slots qualified by their matched source, with a derived condition;
    - T4's confirmation not a prerequisite;
    - a libm-free screen in the new function;
    - ties for today's element, with a T4 tripwire;
    - directional ground rows in K5's API;
    - a new mixed-family basis text; [Reversed to Q7 (a), and K5-C3 retired: see "K5: rulings on I14's checkpoint-0 plan" (RV16-S4).]
    - gate part 1 as a regression net;
    - no site-table change;
    - the curved-formation item not K5's;
    - one slice.
  - K5 runs in parallel with K4 (I12) and F1b (I13); none of the three edits another's files. **Its checkpoint 0 is a plan only.** ROOT serializes its heavy phases (builds, T9, mutation batches, gate) against the other two.
- **Design text made stale or found inconsistent** (the brief's list, items 1–17), recorded here as rulings. `DESIGN.md` stays hash-pinned.
  1. §4.9's and §2.5's citations have drifted; the brief re-locates them on the base.
  2. §4.9's SUP-17 paragraph was delivered by F1a.
  3. §4.9 does not say which entries W4 applies to. The answer is the four selected branches (Q1).
  4. §4.9's curved screen conflicts with R5-4, which says `curved_formation` does not bound the chord mismatch. Slots are qualified by source, and the screen is evidence (Q2).
  5. §4.9's "Coordination" and D-9 (T4 confirms) conflict with R5-4 §5 and §4.3.1. T4's confirmation is not a prerequisite (Q3).
  6. §4.9's ties assume positive stiffness, but §4.3.1 and D5C-2 call lateral zero "the only realized form". In fact the ordinary route realizes no user element. Ties are for today's element with positive stiffnesses (Q5).
  7. T4's M07 repair will change the user element's zero-energy set; a tripwire guards it (Q5).
  8. §4.9's "existing SVD rank screen" is platform-dependent through `hypot`. The new function is libm-free (Q4), and `assess_rigid_body`'s `hypot` goes on the T3-close list.
  9. §4.9 gives no data path for curved node coordinates. They come from `curved_sources`, which only the selected entries receive.
  10. §4.9's "with a reason" has no carrier in `StructuralReport`, which is Debug-published. The reason lives in K5's return types.
  11. The K5 row omits the two site tests that scan SA. Neither changes (Q9).
  12. §6's and D-10's serialization of K5 with the `SA` and `FK/structural.rs` sharers is stale. K5 runs in parallel with K4 and F1b.
  13. §4.1.3's "W4 generalizes": W1b (F3) and W1c will call K5's function.
  14. K4's O1 amendment routed directional grounds to W4, whose grounds were DOF-indexed. K5's API accepts directional rows (Q6).
  15. The curved-formation T3-close item's "to K5" route is withdrawn; it stays on the T3-close list (Q10).
  16. RF-MECH has no user or curved mechanism, so W4 has no frozen reference. Its tests are constructed with an exact generator.
  17. The K5 row's "seeded negative" becomes an explicit or unmatched slot, and the screen is recorded evidence (Q2).
- **Added to the T3-close list:**
  - `assess_rigid_body`'s `hypot` dependence (frame-only bodies);
  - wiring K4's geometry-first check to K5's directional rows.

## F1b: I13's A2 stop on the non-finite mechanics test (ROOT, 2026-09-28)

- **The stop.** With W2, `lib.rs` `tests::audit_nonfinite_computed_mechanics_never_publishes_solved_rows` behaves differently. Its model is linear, with a 1e308 N load.
  - It now range-triggers, W2 publishes the direct values, and the derived stresses overflow.
  - The invocation is then blocked by today's `SOLVER_SYSTEM_BLOCKED` ("computed mechanics must be finite") and `ELEMENT_FORCE_RECOVERY_FAILED`, not by the case's `NUMERICAL_INTEGRITY_UNRESOLVED`.
  - No wrong value is published. This is OQ6 as ruled.
- **Ruling: (a), tightened.**
  - Only the test's third assertion changes, and it asserts the one deterministic new outcome in both modes: the range line, then the non-finite block.
  - The no-solved-rows assertions are unchanged.
  - It is an approved assertion change, declared in CHANGE_RECORD and RETURN.
  - OQ6 stands; option (b) is not taken, and (c), a changed test input, is refused.
- **A new C1 outcome class:** W2 publishes at b ≠ 0, then the derived-row non-finite check refuses the invocation.

## K4: A1 findings F-1 to F-3, the stop rule's blind spot (ROOT, 2026-09-28)

I12's A1 (`cef218a10`) found that the stop rule, which compares p with 2p and forms S\* from computed values only, **cannot see information lost identically at both precisions.** The probes are in `<wt>/scratch/i12/a1_probes/`.
- **F-1: a combination published wrong zeros labelled exact.**
  - The combination was formed as Σcᵢuᵢ of operands whose loads differ by a relative 2^-1060. It was selected at 128 with every row 0, as `AbsoluteVerified{bound 0}`. The truth is nonzero.
  - **Ruling: option (a).** A combination is its own solve.
    - Its right-hand side is one exact expansion of the combined ledger Σcᵢfᵢ and the combined prescribed coupling, rounded once to p. Its prescribed values are the exact combination, rounded once.
    - It reuses the cached factor of the shared stiffness source, and runs its own schedule and stop rule.
    - The operands' results never change.
  - The Σcᵢuᵢ formulation is withdrawn. This refines §4.1.1 and §4.1.2 ("combinations … formed exactly and rounded once") within their intent: with a linear, shared K, the two are equal in exact arithmetic, and (a) avoids the cancellation.
  - B1-E must still be caught, and the CEIL-NET truth reproduced.
- **F-2: a single case published wrong zeros labelled exact.**
  - One member, a prescribed ux = 1 (EA/L = 512), and a load of 2^-300.
  - u cannot represent 1 + 2^-309 at 128 or 256, so the end actions and reactions are 0 at both, and S\*(force) = 0. The case is selected at 128 as exact, but the truth is N = 2^-300.
- **F-3: an availability gap.** An unloaded body moved rigidly by prescribed values leaks reactions of about 2^-(p−27) at every p, and ends Unresolved at the ceiling. It is safe; nothing wrong is published.
- **F-2 and F-3 are a design gap in D1 §4.1.6 and §4.1.6.1** (S\* has no resolution floor tied to the elastic-action scale of the quantities' formation). **They refute §4.1.9's claim** that only a precision-independent, common-mode error can pass the stop rule. A precision-dependent loss below both p's and 2p's resolution passes too.
  - The fix changes S\*'s meaning, and so D2's published-row S\* and its G5a–G5c checks, which may then need the scale published as evidence. **ROOT does not improvise it.**
  - A design TASK drafts an addendum, "D1 revision 5a.3: the stop rule's resolution floor". It gives options, a derivation, the effects on D2 and on the discriminating controls, and a recommendation. It is independently verified, and ROOT selects.
  - **K4 does not reach checkpoint B until the addendum is selected and implemented in K4.** Until then, no K4 test pins the present behaviour of F-2 or F-3.
- **K4's ceiling argument** (§11, route 1 step 10 and route 2) leans on §4.1.9's claim. RETURN must restate it in light of the addendum.
- **The F2a gate is unchanged:** F2a does not merge without this addendum implemented, as well as the budget limits.

## K5: rulings on I14's checkpoint-0 plan (ROOT, 2026-09-28)

I14's plan (`<wt>/scratch/i14/CHECKPOINT0_PLAN.md`, sha256 `7f50c788…`, 943 lines [942 lines with a final newline (RV16-N1)]) is **approved**, with the rulings below.
- **Approved as planned:**
  - the FK API: `assess_constrained_bodies` over one connected body, the `ConstrainedGround` rows including directional rows, `TieRefusal`, and canonical input ordering;
  - the tie reduction, six unknowns per body, derived in RETURN against the full stacked map;
  - the libm-free screen: a power-of-two characteristic length, the `sqrt` forms of the Jacobi `hypot`, and m counting nonzero rows;
  - the exact witness with canonical scaling;
  - curved qualification by K-D5's source match plus coordinate agreement, with the Q2 condition derived in RETURN;
  - the SA wiring on the four selected bodies only, with no public signature change and the contact-seed guard closed by derivation;
  - Q9, no site-table change;
  - the test, mutant and product-run lists;
  - gate part 1 compared directly with G1's baseline, since the product trees are identical;
  - positions P1–P11.
- **The Mac-main product run is recorded.** The constructed curved mechanism is refused `NUMERICAL_INTEGRITY_UNRESOLVED` (a pivot failure) on main at 0 m, about 1 km and 5e6 m, on both entries and in both modes. So on this corpus K5-C1 changes a refusal's code to `NUMERICAL_INTEGRITY_PHYSICAL_MECHANISM` with a direction, and **removes no published result.** Checkpoint B's corpus uses dyadic geometry, because non-dyadic bends at 5e6 m are refused earlier by the curved-bend radius check.
- **S1: Q7 is reversed to (a). The basis text is unchanged on every entry.**
  - A selected-only text breaks at least seven K-D5 tests that compare the selected branch's report `Debug` with the unselected path's. A text on every entry would change nonlinear invocations, which is forbidden.
  - The text "physical rigid-null witness unqualified for bodies containing user/curved elements" now under-claims on the selected branches. That is conservative, and is recorded.
  - Correcting it is on the T3-close list and F2a's input list, where the publication types are unified.
  - K5-C3 no longer exists.
- **Brief correction, recorded:** `invented_preview_model.json` is refused at `PP:1711` (legacy pressure), not at the joint check. The conclusion, that no element is built, holds.

## F1b: rulings on I13's A2 (ROOT, 2026-09-28)

A2 is committed as `e215c6007` on `codex/piping-f1b-20260928`. Every Scope §6 prediction was confirmed by a product run.
- **(a) The admission order is approved.** Thermal/eigen, pressure thrust and exact-pressure operands are now checked before the element-primitive check. Only the reported family names change; the set of refused cases is the same.
- **(b) Three admission checks cannot be reached in the product:** user-stiffness element, pressure thrust, and non-nodal term. They are pinned at unit level, and RETURN derives why each is unreachable.
- **(c) Coexistence: I13's derivation is accepted, with a condition.** [Superseded: RV17-1 showed that exact-block does select range-triggered cases (CX-F, CX-G), so the next sentence is false. See "F1b: rulings on RV17's review".] Exact-block cannot select a range-triggered case: its exact solve fails at `exact_radix` or at source closure, and large-magnitude triggers never reach the captured entry.
  - The candidates were tried on Mac main, and the gate base has 16 selections, none after a Range.
  - **Condition:** the derivation is written step by step in RETURN, and F1b's independent reviewer checks it (the M31b lesson). The candidate-side test stays.
- **(d) The mixed captured invocation is pinned as observed.** It finalizes with a `partial` receipt: A `qualified`, B `unsupported` with OQ2's decline, and B's `ordinary_attempt` following W2's verdict (OQ15).
- **(e) and (f) are approved under Q10 and Q11:** the NI K2b pin's product half as the declared table `F1B_PRODUCT_SITES` and `F1B_PRODUCT_NEVER`, and the restated linear variants.
- **The C1 list is approved: 28 runs,** equal to the proposed list:
  - 14 published: CHAIN-E-1000 ×4 at b = 540, Sensitive; THIN-B ×4 at b = 536, `CHECKS_PASSED`; the three LEF-large cases, typed ×2 each, at b = −702;
  - 6 refused by W2's template;
  - 8 in the new class "W2 published at b ≠ 0, then the derived-row non-finite check refuses": CONT-E-1000 and SKEW-E-1000 ×4 each, where `displacement_magnitude` overflows.
  - The gate's PASS requires C1 ⊆ this list, **0 trusted breaches** (the published range cases are checked against the references), and C3 byte identity elsewhere. [Also required: no C2 sparse run aborts at the heap cap (`TASK_BRIEFS/I13_F1B_IMPLEMENTATION.md` PASS conditions; RV16-S2).]
- **The full-envelope probe variant is approved,** and it is the method for every gate from now on:
  - P1's `main.rs` plus `full_envelope()`, which emits `run.envelope_sha256` over `serde_json::to_vec(&MechanicsEnvelope)`;
  - `sha2` is added to the probe's own `Cargo.toml`;
  - `run.envelope` is unchanged, and `compare.classify` is unaffected.
  - P1's summary omits published bytes (I13's A2 §7), so C3 byte identity is judged on the full hash. **G1 re-runs the base's part 1 with the variant** before any gate compares against it.
- **RV11D-N2:** 0 refused and 0 mismatched across 766 runs of b = 0 publications (3,666 reactions, 251 spring actions, 3,456 end-action sets).
  - The 13 gate cases of 1,000 or more members are observed in F1b's gate slot, typed and in sparse mode, where the candidate completes them.
  - The 10,000-member cases are included only if sparse mode completes them within the gate's limits; otherwise their omission is disclosed.
- **The b-rule (Q12):** of the 38 `ScaledEvaluation` refusals probed at every even b in [−1100, 1100], only K2b's documented limitation chain has a solving b. It stays a documented limitation, and the refinement stays on the T3-close list.

## D1 revision 5a.3: rulings on V4's verification (ROOT, 2026-09-28)

V4 (`DESIGN_NUMERICS/REV_5A3_CANDIDATE/V4_VERIFICATION.md`, sha256 `0222d0ec…`): **NOT VERIFIED**, with 0 BLOCKING, 7 SHOULD-FIX and 13 NOTEs.
- **What held:** option (iii)'s rule held under V4's independent emulator, which is built from K4's Rust with its own exact oracle.
  - On 200 saturating models, today's rule made 38 false claims and (iii) made none.
  - Φ = 2^-438·Ê is correct.
  - V4 re-counted λ from K4's Rust at ≤ 68g units, against DS1's 139g; both are within 2^8.
- **DS1 revises the candidate to resolve every SHOULD-FIX item, and V4 then runs a delta check.** ROOT selects only after VERIFIED. ROOT's decisions on the two items that are ROOT's to make:
  - **V4-S2: the premise becomes a runtime check, not an accepted risk.** The premise, that the solve's backward error carried into forces and moments stays within V, remains a conjecture.
    - DS1's §8.1 estimator is adopted: one extra correction solve at 2p from the exact residual K4 already forms. A case whose estimate exceeds V escalates, and at the ceiling it is Unresolved. [Amended: the estimate is tested at W ≤ V/4, and the gate is hybrid; see "D1 revision 5a.3: rulings on V4's delta check at R2" (RV16-S4).]
    - The revision specifies it exactly, with its work charged and its mutant.
    - Rationale: the standing lesson. A claim relied on for publication is derived or checked, never assumed.
  - **V4-S3: my O12 ruling is reversed.** The residual gate's denominator becomes the bounded operator, at contribution level, not M03's coalesced |K||u|. [Amended: the hybrid bounded-denominator gate; see "D1 revision 5a.3: rulings on V4's delta check at R2" (RV16-S4).]
    - V4 found in emulation that the coalesced form refuses an ordinary loaded cantilever at every precision when y_reference has a chord component: y_ref (3,4,5) on a (3,4,0) run, with a gate ratio of about 6e15.
    - The revision specifies the denominator. K4 builds a Rust control for that cantilever and confirms it before checkpoint B.
- **The other SHOULD-FIX items, resolved in the revision:**
  - **S1:** the demotion claim is restated with its threshold, |q| < 2^-472·Ê.
  - **S4:** V, Φ and G5b's item 6a are computed from the same binary64 ê, bit for bit.
  - **S5:** g, the directional blocks, the unpublishable rows and E's rounding direction are specified computably.
  - **S6:** behavioural controls are added for M2, M7 and M10.
  - **S7:** a published-data lower bound on ê (V4's) is added to G5a, so a D2 reader can check it at p = 512.
- **NOTEs to carry:**
  - **the saturated-assembled-entry reaction** (−1.5 N published `relative_verified` against a truth of 0): add a control;
  - **the combination's prescribed rows are rounded twice when published** (1 + 2^-53 + 2^-150 publishes 1.0 at p = 128). **This is a K4 defect, independent of S\*.** K4 fixes it now: publish from the exact sum, rounded once;
  - **the binary64 publication rounding** can exceed b by up to 2^-24·b [2^-23·b, per V4-R7's erratum (RV16-S4)]. The guarantee's statement says so; the gap already existed in 5a.2;
  - Lemma 2's binade-boundary hypothesis is corrected.

## Resume after the pause, and F1b's heap-cap finding (ROOT, 2026-09-28)

- **Resumed** from `PAUSE_2026-09-28.md`. The Mac stayed up and the memory guard kept running. Main is unchanged at `24dea2dae`. GEN-8 passes on the pause commits.
- **K5 (PR #1044):** hosted CI is green on `b379e5b27`, and the full-SHA dispatch 36459966791 (target_base `24dea2dae`) succeeded. RV14's review resumes from its pause state.
- **F1b gate part 1** (candidate `948e0bb99`, full-envelope, against G1's full base):
  - [Part 1 FAILED its own gate condition: 4 C2 sparse runs aborted at the heap cap, which the F1b brief makes a stop; I13's comparison reads `RESULT: FAIL`. `gate_check`, which checks trusted breaches only, passed (RV16-S2).] `gate_check` PASS; C3 has 832 runs with 0 differences; C1 is exactly the ruled 28; all 12 dense C2 runs get the guard's refusal.
  - Of the 12 sparse C2 runs, 8 complete with named M03 refusals. **4 abort at the heap cap:** RF-LARGE-CONT-n10000-AX and -ROT, both entries.
  - The allocation is in main's unchanged DEC-050/053 observation lane: `solve_symmetric_system_from_entries` → `SymmetricProfileMatrix::from_entries`, in identity order. For CONT n10000 that profile is 675,179,982 entries (5.4 GB), and it is built only to report `original_profile_entry_count` and `original_max_half_bandwidth`. On main the same runs aborted earlier, at the dense K.
- **Ruling: guard the observation lane in PP (F1b's write set).**
  - Before the lane runs, PP estimates its original-order profile from the pattern in O(nnz), with no allocation.
  - If the lane's estimated bytes exceed the provisional 6 GiB ceiling (the named constant of Q8), the lane is not run, and its observation fields are published as not observed, with a named reason. This follows OQ5's precedent at b ≠ 0.
  - **Conditions:**
    1. **Derived in RETURN and checked by the reviewer:** no case main publishes can exceed the lane ceiling, because main's dense path needs far more memory first. So no byte changes for any case main publishes.
    2. **Tests:** the estimate equals the lane's actual profile count on the B corpus; there is a lowered-ceiling unit test through the `#[cfg(test)]` hook; and there is a mutant that removes the guard.
    3. The PP suite and T9 are re-run.
    4. **Gate part 1 is re-run in full on the new head,** and then part 2. The 4 CONT sparse runs must no longer abort. What they publish or refuse instead goes in C2's table.
  - A sparse ceiling for the main solve stays unset in F1b (Q8). Only this observation lane is guarded.
- **D1 5a.3:** V4's delta check of R2 resumes from its pause state, including its open lead (a saturated stiffness entry amplified by a soft mode).

## D1 revision 5a.3: rulings on V4's delta check at R2 (ROOT, 2026-09-28)

V4's delta check at R2 (appended to `V4_VERIFICATION.md`, sha256 `c2f5539b…`): **NOT VERIFIED**, with 1 BLOCKING, 1 SHOULD-FIX and 6 NOTEs.
- **V4-R1 (BLOCKING): the estimate misses a saturated assembled stiffness entry.**
  - LEVER2 is an exactly representable lever with gain 2^90, a prescribed rigid translation, and a tip spring 2^-580 of its assembled diagonal, so the spring is lost in every assembled entry up to 576 bits.
  - R2 selects it at 256 with W/V = 0, and publishes 0 `absolute_verified` against a truth about 1,000 times its bound.
  - This refutes R2's claim of a 2^72 margin for the K^q term.
  - **Ruling: adopt V4's demonstrated fix.** W's residual is formed as one exact sum over the element contributions [and the directional-block entries (RV16-N3)] and spring stiffnesses, not over K^q's rounded assembled entries. In V4's emulation, LEVER2 is then refused and no control changes.
- **V4-R2 (SHOULD-FIX): the estimate recomputes its residual on the final state,** not reusing the gate's last evaluation. The R1 fix requires this anyway. SEEDED-COMMON must be caught.
- **DS1's two departures from ROOT's rulings are confirmed,** on V4's derivations:
  - **the hybrid gate:** acceptance on the bounded denominator, with refinement driven by the coalesced ratio. What is published is decided by the stop rule, V and W on the final state;
  - **the estimate's test W ≤ V/4:** resolution (68) plus estimate (64) is 132 of V's 256 units, whereas a test at V would allow 324.
- **NOTEs to carry into R3:**
  - V4-R5: apply the bounded test to the best state, not the last;
  - §1's "20 selected at 128" is labelled emulation-only;
  - the Lemma 2 proof is repaired;
  - G5a item 4 takes values after unit conversion;
  - V4's two errata are recorded as corrected.
- **Next:** DS1 writes R3, and V4 runs a delta check. ROOT selects only after VERIFIED. K4 stays blocked until then.

## K5: rulings on RV14's delta check at 28517eaaa (ROOT, 2026-09-28)

- **Verdict accepted:** PASS, 0 BLOCKING. RV14's four SHOULD-FIX findings and N1, N2, N5 are resolved; RV14-M1 to M4 are each killed by a new test (review committed at `f3e50948b`).
- **RV14-D1 (SHOULD-FIX): fix before merge.** I14 adds RV14's construction as a `P` expectation (nodes (0,0,0) and (2^1023,0,0), grounds d2 to d5, directional row n = (2^-60, −1, 0) at node 0; expected: refused as "parameters not representable"), and shows mutant RV14-M5 (finiteness test only) killed.
  - **Host timing:** I14 writes the test now but builds and runs nothing until F1b's gate part 2 has finished, because part 2's timed runs need a quiet host.
  - **Review:** RV14 checks the delta.
- **RV14-D2 (NOTE): accepted as a conservative limitation, not changed in K5.** A refused witness ends the candidate search with `NumericallyUnresolved`, even when a later candidate would publish.
  - **Why not change it now:** the result is a refusal, never a wrong value, and it is reachable only through the FK API. Moving on to the next candidate would change FK's result classes and the probe hash, which would reopen the oracle and probe checks for no correctness gain.
  - **Where it is recorded:** I14 records it in K5's RETURN as a known limitation, and it joins the T3-close list as a candidate refinement.
- **DEC-025:** after the D1 fix and RV14's delta, on the final head, once F1b's part 2 frees the Mac.

## D1 revision 5a.3: rulings on V4's delta check at R3 (ROOT, 2026-09-28)

- **Verdict accepted:** NOT VERIFIED, with 1 BLOCKING (V4-T1) and 5 NOTEs [4 NOTEs, T2–T5; V4's verdict line says 5 (RV16-N1)]. V4's fix for V4-R1 is implemented faithfully, and V4-R2 to R8 are resolved.
- **V4-T1: adopt V4's closure, both parts together.**
  1. **The 3p + 64 option becomes specified text.** W's contributions are formed at q_W = 3p + 64 bits, capped at 1024. The extra formation pass is charged as work.
  2. **The runtime charge.** Add to §4.1.6.3 the test C_q = 62.5·2^-q_W·F·est·‖a_q·S‖₁·‖S·Ā·|u|‖_∞ ≤ 60·2^-2p·ê(body, kind), inside the 124 units V leaves; at p = 512, test it against 2^-22·b.
     - **The terms:** est is the verification's Hager–Higham estimate of ‖K̃⁻¹‖₁, S is K4's radix equilibration, Ā·|u| comes from E's reaction pass, and a_q is the recovery row.
     - **F is pinned:** it is the same allowance that the condition screen and W's accuracy already rest on, with no new uncertified step.
     - **On failure:** the precision is not selected at this p, and the next p is tried.
- **Rigour required in R4.** R4 bounds e_q beyond first order: the second-order remainder is bounded, or dominated, under the design's own screens, for example ‖K⁻¹δK‖ ≤ 1/2 with a factor of 2. Alternatively R4 states exactly which first-order step remains and why the screens make it safe. No step is left "argued".
- **Tests and mutants in R4's emulator:**
  - LEVER2 and TILT-LEVER are charged out or refused;
  - all 44 controls and the 20 probe cases keep their R3 precisions and classes at q_W;
  - a mutant that drops the charge is killed by a control that the charge alone refuses; [if no admissible control exists, R4 derives the equivalence and reports it, and ROOT rules (RV16-N2). R4 built one: CHARGE-SLENDER.]
  - a mutant that uses q = 2p + 64 for W is detected.
- **NOTEs:**
  - **V4-T2:** R4 drops its citation of V4's false R2 statement and records the erratum.
  - **V4-T3, T4, T5:** recorded as V4 states them. M16's survival is acceptable.
- **The limitation that remains, disclosed:** the honesty guarantee rests, to the factor F, on Hager–Higham's uncertified norm estimate. The condition screen and W already rest on it, and this closure adds no new reliance. R4 states it in §9, and ROOT carries it to the owner's list.
- **Next:** DS1 writes R4, V4 runs a delta check, and ROOT selects only after VERIFIED. K4 stays blocked until then.

## K6: spawn and rulings (ROOT, 2026-09-28)

- **K6 (harness observations) is spawned as I15** from main `41aeb2a02` [correction: the K6 branch was created at main `56dd72334`, which changes only `projects/chirality-app-v4/**`; the piping tree is the same (RV16-S1)]. It departs from the selected kernel order ("K4 … then K6 and V-K"): K6's binary64 half runs before K4, and K6b keeps the W1 part after K4 (RV16-N6). Its piping tree equals `24dea2dae`'s; K1 is merged, and K6's row needs only K1.
  - A TASK drafted the brief (`TASK_BRIEFS/I15_K6_IMPLEMENTATION.md`); ROOT reviewed it.
  - The rulings on Q1–Q13 are in the brief's "ROOT rulings for this slice". In short: binary64 now, W1 in a K6b after K4; SA's path through an in-repo path dependency; an 8 GiB cap, a heap cap at C − 512 MiB, and the admission rule; one ceiling run at 16 GiB, last and alone; a runner pytest under conditions; no dense run at 10,000 members or more.
- **Why now:** K6's measurements are what ROOT needs to replace F1b's provisional 6 GiB dense-scrutiny and observation-lane ceilings, and, with V-K's runs and K4's work counts, to set W1's budget limits before F2a merges ("K4: Q5 amended").
- **Host:** checkpoint 0 is read and design only. Builds wait until F1b's gate part 2 releases the Mac. Observation runs happen only in slots ROOT grants.
- **Design text made stale** (the brief's items 1–12), recorded as rulings; `DESIGN.md` stays hash-pinned:
  1. K6's inputs are kernel models; the RF-LARGE product requests are P1's, hashed in `gen_out_sha256.txt:122-145`.
  2. §4.8's stage list is read at public-API boundaries, including the geometry and formation-check stages the product path now runs.
  3. The only Linux peaks known are P1's product-level ones. On the Mac, admission uses them or the derived estimate times the measured ratio (Q3).
  4. The in-process heap cap from the platform calibration joins §4.8's host protection on macOS.
  5. "For W1, limbs per entry" moves to K6b, after K4.
  6. The kernel runner's home is `H/runner/`. §4.10's `numerical_robustness` home serves V-K's and V-P's product-level runs, which reuse the runner by path.
  7. D-8's and C4's "K6 and V-P" is read as amended: limits from K6 and V-K, revisited by V-P.
  8. §2.1's "about 100 bytes per n² entry" is superseded for the guard by F1b's count of 96. K6 measures the actual-to-estimate ratio.
  9. The DEC-050/053 legacy LU call is at `PP:2508` on `d1cc97ce4`. The identity-order lane is observed too (Q12).
  10. K6's sparse observation is a new pattern path; the old harness "sparse" path is left unchanged.
  11. §7.2's RF-LARGE expectation is now measured at product level (P1, and F1b's gate); K6 adds the kernel level.
  12. The DEC-050/053 pytest pins that read `H/src/lib.rs` constrain K6's edit there to one `pub mod` line.

## K5: rulings on RV14's review (ROOT, 2026-09-28; recorded late, RV16-S3)

These rulings were given in ROOT's message to I14 after RV14's review. They are recorded in K5's RETURN §16 on the K5 branch (`95c7501a7`) and in RV14's delta check, and are recorded here with their original date. Nothing in them is new.

- **RV14's review at `b379e5b27`:** PASS; 0 BLOCKING, 4 SHOULD-FIX, 5 NOTEs (`REVIEW/K5_REVIEW.md`, `3a17799e4`).
- **All four SHOULD-FIX findings are fixed before merge,** each with a test and a mutant killed from a clean archive:
  - RV14-1: two FK cycle-band cases;
  - RV14-2: an SA test on RV14's P1;
  - RV14-3: a PP test on `constructed_mechanism_r0.2_o0`;
  - RV14-4: `rigid_parameters` is exact, or the witness is refused.
- **RV14-4's behaviour.** ROOT offered I14 two options: refuse a witness whose [t/L, θ] is not exactly representable, or publish the parameters in a form that is always exact. RV14 had offered publishing `None`, or [t, θ]. I14 chose the refusal, with the named reason `CONSTRAINED_WITNESS_PARAMETERS_UNREPRESENTABLE`, and ROOT accepts it.
  - **Why:** the field keeps its documented meaning ([t/L, θ], as `assess_rigid_body` publishes it) for every published witness, and the refusal is conservative.
  - **Consequence:** on RV14's 4,226-case FK corpus, exactly the 349 witnesses that published a non-finite component change from W to U. Every other result is byte-identical. The affected cases have subnormal spans, reachable through the FK API only.
- **NOTEs:**
  - N1 (stale line numbers) and N2 (spring grounds killed only by `unwrap` panics) are fixed;
  - N3 and N4 take no action;
  - N5 (main moved) is resolved by ROOT's merge of main.
- **Erratum (RV16-N11).** The K5 merge commit `28517eaaa`'s message says "Merge main `df6d59e3c`", but its second parent is `65e2d6c2a`. RV14's delta check states the parents correctly. K5's merge record will disclose this.

## K6: rulings on I15's checkpoint-0 plan (ROOT, 2026-09-28)

I15's plan (`IMPLEMENTATION/K6/PLAN_CHECKPOINT0.md`, sha256 `4b4d9b27…`, 633 lines) is **approved**, with the rulings below. RV16-N4's ruling is implemented: every n² mode at 10,000 members or more, and CONT n10000 `lane-id`, is refused by name, independently in the runner and in the binary.

- **N10 (the known dense timeouts at 1,000 members):** record, finish the running tier, then stop and report.
  - I15's hypothesis, which is not yet a finding: the dense pivot screen refuses a pivot the skyline passes, and the refusal then runs the O(n⁴) dense witness.
  - If K6's stage timings or outcome classes confirm it, the result is a **solver finding on main's dense path**, not K6's to fix. ROOT routes it. It bears on the F1b gate's part-2 timeouts at the same sizes.
- **N11 (the slot):** B splits into separately granted slots:
  - **B1:** everything except T3b and the ceiling run;
  - **B2:** T3b, dense and lane-lu at 1,000 members;
  - **B3:** the Q4 ceiling run, last and alone.

  Use `--repeats 1` for the two timeout cases and for lane-lu; their memory figure is deterministic. The two-hour stop applies per slot.
- **N7:** approved. After the host is released, `--counts-only` may run at 1,000 and 10,000 members at A1–A2, one process at a time, under a 512 MiB cap. It does O(nnz) work with no n² allocation, so it is not a "run above 100 members" under Q13.
- **N9:** approved. Admission uses 96·n² + E_base. The claim ratio stays against F1b's bare 96·n².
- **N8:** approved. There are two peak models, with the cap enforced on the in-place one, and ρ = max(RSS, move).
- **N4:** confirmed. RF-LARGE's `y_reference` follows P1's rule.
- **N5 and N19:** approved. The copies of SA's and PP's private items are pinned by test E and a mutant each.
- **N6:** approved. After K5 merges, K6 merges main and re-runs E. If E fails on frame-only models, that is a stop.
- **N15, the DEC-025 sandbox:** at A2, run the pytest wrapper once through the DEC-025 sweep entry, invoked as ROOT invokes it. If `ps` or `killpg` is denied, the test fails and is reported; nothing is skipped.
- **N18:** approved. `memorystatus_level ≥ 80` before each run, and the memguard log is checked after each tier.
- **N1–N3, N12–N14, N16, N17, N20:** approved as proposed. N14's 128×128 grid stays conditional, as the plan states.
- **Next:** A1 (the Rust side) starts when ROOT releases the host, after F1b's gate part 2.

## Records PR #1049 merged (ROOT, 2026-09-29)

- **Merged:** [PR1049](https://github.com/sgttomas/chirality/pull/1049) at head `720924cbc`, merge `0256decc6`, 2026-09-29 00:09:20Z, with `--match-head-commit`.
- **Review:** RV16's review at `04553ad05` PASSED: 0 BLOCKING, 5 SHOULD-FIX (all fixed in `adf43e1c5`), 11 NOTEs. Its delta check at `720924cbc` also PASSED, with 3 optional NOTEs.
- **Hosted CI on the head:** 7 passed and 6 were skipped, as selected for a records-only change.
- **RV16's delta NOTEs:**
  - D1: the work graph is updated after merge.
  - D2: R4:3 repeats V4's "5 NOTEs" miscount of its R3 check; the correct count is 4. R4 is not edited while V4 verifies it, and the erratum is recorded here.
  - D3: the PR body's blank-line disclosure also covers two K5 review run records that are not scripts.

## D1 revision 5a.3: rulings on V4's delta check at R4 (ROOT, 2026-09-29)

- **Verdict accepted:** NOT VERIFIED, with 0 BLOCKING, 4 SHOULD-FIX (V4-U1 to U4) and 5 NOTEs (U5 to U9).
  - V4-T1 is closed as ruled: the charge is implemented as specified, and the Lemmas, Theorem and Corollary hold step by step, apart from U5 and U8.
  - Lemma B's count is checked against K4's Rust source, including every stage DS1 had not re-checked.
  - No false claim was found.
- **V4-U1: adopt the certified bound. F·est is removed from every step of the guarantee.**
  - **The bound:** Uc = U/(1 − U·γ_m·‖|L|D|Lᵀ|‖₁), with U = ‖M(L)⁻ᵀD⁻¹M(L)⁻¹e‖_∞ taken from the verification's own LDLᵀ factor. Computed upward on nonnegative data, it bounds ‖K̃_P⁻¹‖₁ from above (derived).
  - **Where it replaces F·est:** in the charge (t₁, t₃), in the θ check, and in W⁺ and W's own accuracy wherever they use F·est. The Hager–Higham estimate may remain only where it serves availability (screening or escalation). It never bears on honesty.
  - **Why:** V4 demonstrated that, within the design's screens, the estimator's miss is unbounded (8.9e26 on a 12-DOF frame; 3.3e66 at p = 256), so no pinned F is a bound. Uc costs one substitution pair and one pass. On all 195 of DS1's states it is at least the exact norm and at most 0.0015·F·est, so it costs no availability there. Where it is loose, only availability is lost.
  - **Consequence:** R5's §9 must state whether any uncertified step remains. The owner-list limitation ("the guarantee rests on Hager–Higham to F") is withdrawn once R5 is VERIFIED.
- **V4-U2: adopt.** θ is tested per body, and the g check applies only to members in bodies that carry data. V4's two new controls must return to 128 under R5, and M20 and M23 must still be killed, on honesty if a case exists, otherwise on availability, stated as such.
- **V4-U3: adopt.** SEEDED-SOFT joins §7 and K4's controls, and kills M22 with a false claim under the mutant. M21 and M14 still survive: R5 builds a killing case for each, or derives that another test implies each and records the guard as kept for the derivation.
- **V4-U4: adopt.** F3's W1b obligation (§6.5) is corrected: the formed-load error is charged at its true size, with no 2^(q_W−6) under-charge, and the recovery side (E and the reaction-row count) is added.
- **NOTEs U5 to U9: fix in R5's text.**
  - U5: Lemma C's inequality at p = 512.
  - U6: M14's slack arithmetic.
  - U7: the M10 paragraph, whose error is first order.
  - U8: θ's rounding direction, and the ∞-norm the Theorem needs where the charge uses the 1-norm.
  - U9: the reliance wording, which changes with U1.
- **Also for R5:** RV16-D2's miscount (R4:3 says "5 NOTEs" for V4's R3 check; there were 4).
- **Next:** DS1 writes R5, V4 runs a delta check, and ROOT selects only after VERIFIED. K4 stays blocked until then.

## F1b: gate re-run on 130445db2 accepted; the Mac is released (ROOT, 2026-09-29)

- **Result: PASS** (I13; records in `<wt>/scratch/i13/gate2/`, SHA256SUMS over 3,456 files, verified by ROOT).
- **Part 1:** 884 runs against G1's full base (`runs.jsonl` `9139140c…`); candidate `runs.jsonl` `30d99bf0…`.
  - `gate_check` PASS: 0 trusted breach triples.
  - C3: 832 runs, 0 differences.
  - C1: exactly the ruled 28, with rows identical to the `948e0bb99` gate.
  - Nothing changed outside C1 and C2.
  - **C2: 0 sparse heap-cap aborts,** so the condition part 1 failed on `948e0bb99` now holds.
    - The 4 CONT n10000 sparse runs now publish `MECHANICS_SOLVED`, `sensitive` and `needs_recompute`, with `SPARSE_OBSERVATION_LANE_NOT_RUN` and the lane's fields `not_observed`. Peak RSS is 4.82–5.17 GiB.
    - The 12 dense runs get the guard's refusal. The 8 other sparse n10000 runs get named M03 refusals.
  - Against the `948e0bb99` gate, 880 of 884 runs are identical. Only those 4 CONT runs differ.
- **Part 2:** all 8 dense 1,000-member runs time out at 1,800 s on both sides, with no base/candidate mismatch. Each run's load is recorded, and there was one load wait. This matches K6's predicted dense-path timeouts (K6 plan, N10); K6's stage timings will show where the time goes.
- **RV11D-N2 on CONT n10000:** reactions and end actions are equal bit for bit (AX and ROT). 8 of the 13 cases are now observed, all equal.
- **Next for F1b:** D (CHANGE_RECORD, RETURN, records), then the PR, an independent reviewer, hosted CI with the full-SHA dispatch, DEC-025 and GEN-8.
- **Host released** at about 02:10Z. The order of use:
  - K5 stage 2 (I14; FK K5 tests and RV14-M5) and K6 A1 (I15) may build concurrently, each within its own 2-cargo-job limit.
  - Quiet-host slots follow, one at a time: DEC-025 for K5, then DEC-025 for F1b, then K6's B1, B2 and B3.

## D1 revision 5a.3: rulings on V4's delta check at R5 (ROOT, 2026-09-29)

- **Verdict accepted:** NOT VERIFIED, with 1 BLOCKING on availability (V4-V1), 1 SHOULD-FIX (V4-V2) and 5 NOTEs (V3 to V7).
- **Honesty is confirmed:**
  - Lemma D is correct against K4's `factor.rs` (`81f81f24…`).
  - In V4's stress test, Uc never fell below the exact norm in 46,315 factors where it existed.
  - "No uncertified step remains in the honesty guarantee" holds.
- **V4-V1: do not accept the loss. Add V4's shifted-factorization bound and take the smaller of the two.**
  - **Why:** Uc's comparison matrix loses every sign cancellation, and on R1's own RF-LARGE frames it grows geometrically along the elimination. At 100 members, five of six models leave 128. At 1,000 members, five of six would be Unresolved at every precision, however well conditioned they are. RF-LARGE-scale frames are what the product must solve, so this loss is not acceptable.
  - **The bound:**
    - Factor K̃_P − σI with the same loop. If every pivot is positive, then λ_min(K̃_P) > σ′ := σ − γ_m·N_L′ − (the shift's rounding), by Weyl and Lemma D's step 1, so ‖K̃_P⁻¹‖₁ ≤ √n/σ′. Form every step with directed rounding.
    - σ may be chosen from est; that is an availability use only.
    - A failed shift halves σ, a bounded number of times, then falls back to Uc alone.
    - The certified bound used is min(Uc, the shift bound), with the work charged.
  - **R5's R1 lane is extended to RF-LARGE** at 10, 100 and 1,000 members, in K4's elimination order, within emulation limits. R6's §1 and §9 report its selections honestly.
- **V4-V2: adopt per-body Uc and per-body shift bounds,** from the same passes. M17 and M24 then lose their kills. They are recorded as kept for the derivation, as M21 is, unless R6 builds a single-body kill.
- **NOTEs V3 to V7: fix in R6.**
  - V3: M21 is vacuous only for published W1a cases (DESIGN §4.2 admits nonzero support motion). Restate the scope, and drop "no other test implies it".
  - V4: the 1 + 2^-9.5 factor is derived, not measured.
  - V5: THETA-STUB's over-breadth in a decoupled zero-state block. Narrow θ further, or record why not.
  - V6: an encoding rule for a Uc of 2^1024 or more in the receipt.
  - V7: V4's own R3 count erratum.
- **Next:** DS1 writes R6, V4 runs a delta check, and ROOT selects only after VERIFIED. K4 stays blocked until then.

## K6: rulings at I15's A1 stop (ROOT, 2026-09-29)

- **The stop was correct.** Test D breached the DEC-053 parity basis (1e-9 relative) on three 100-member models: CHAIN-ROT at 7.15e-8, TREE-AX at 1.45e-8 and TREE-ROT at 4.78e-8.
  - In all three, both modes are **Sensitive**.
  - Test E shows both vectors are SA's own results, bit for bit. Bitwise K holds and the outcome classes agree.
  - Every Passed model is within 5.9e-11.
  - This agrees with P1's product-level finding at the same sizes (`DETECTION/RETURN.md:242-249`).
- **Ruling: option (a).** The DEC-053 basis is asserted where both modes are Passed. Where a mode is Sensitive, the delta is recorded and not asserted.
  - **Why:** a Sensitive publication carries no accuracy claim at that level. The product publishes it as `needs_recompute`, and that demotion is M03's statement that the integrity checks could not confirm the result. The outcome classes must still agree in every case; that stays asserted.
  - **Recorded as a stale-design item:** §4.8 item 3's "published quantities agree within the DEC-053 basis" is read as applying to Passed publications.
- **Questions:**
  1. `--counts-file` is approved: observation runs take counts from the counts-only record, checked by the model's canonical digest, with the estimates recomputed and the refusals kept.
  2. ρ for admission uses RSS net of a process baseline, measured in the same slot by a no-op run of the binary. Admission at 1,000 members or more uses ρ measured at 100 members or more.
  3. The CONT n10000 lane counts (675,174,982 ROT, 562,627,485 AX; F1b's 675,179,982 is the full-block form) are recorded as context. The run stays never-run.
  4. The TREE identity-bound erratum (75n − 72) and 5, the lane-id `ledger` stage, are accepted.
  5. The gap between product-level and kernel-level RSS (F1b's CONT n10000 at 4.8–5.2 GiB, against K6's kernel sparse estimate of about 0.38 GiB) is recorded. B1's ascent will measure the kernel side.
- **Disclosed slip:** I15 ran `git add -N` and `git reset -q` on `H/` in the k6 worktree, two index operations its brief forbids. The net effect is nil: the index is empty and HEAD is unchanged. Recorded; no further action.
- **Next:** A2 (the runner, its tests, the pytest wrapper, `--plan`, `--smoke`). While K5's DEC-025 sweep runs on the Mac, I15 writes code only.

## K5 merged (ROOT, 2026-09-29)

- **Merged:** [PR1044](https://github.com/sgttomas/chirality/pull/1044) at head `babcf5e65`, merge `1cdeae2c1`, 2026-09-29 03:18:22Z. The merge record is `IMPLEMENTATION/K5_MERGE/RECORD.md`.
- **DEC-025 on `babcf5e65`:**
  - the suites match the Mac baseline, except K5's added tests (FK +18, NI +13, PP +4) and the three known Mac platform failures;
  - pytest 3023 passed;
  - the wasm and production builds passed;
  - vitest failed once on `App.test.tsx`'s known 30 s render timeout, and the immediate re-run passed 2822 of 2822. K5 changes no desktop file.
- **Consequences:**
  - F1b merges second, so it merges main and re-runs its suites, T9 and gate part 1, which is quick at about 6 minutes, before its merge.
  - K6 merges main and re-runs test E (N6).

## K6: A2 accepted; schedule approved; B1 granted (ROOT, 2026-09-29)

- **A2 accepted.**
  - H's suite passes, and the runner's 27 tests pass, including the live watchdog and the negative control.
  - N15: the wrapper passes through the DEC-025 pytest surface's invocation, so `ps`, `pgrep` and process-group kills work in that context.
  - `--smoke`: staged is identical to SA's entry on 84 of 84 lines. The watchdog killed its child at 136,480 KiB against a 128 MiB cap, within one poll's growth. The heap-cap abort was classified `heap_cap_abort`.
- **Committed on the K6 branch:** A1 and A2 at `9ababe4f2`, with the records moved to the branch as for K5 and F1b. Main `1cdeae2c1`, which includes K5, is merged in at `1f354c20b`. It overlaps no K6 file.
- **N6:** before B, re-run test E and H's full suite on `1f354c20b`. A failure of E on frame-only models is a stop.
- **The schedule is approved** (`_run_records/a2/plan.txt`, sha256 `93cd36d5…`, 138 rows).
  - 14 rows are refused by name.
  - Grid 128×128 is deferred by name until ROOT rules on it after 96×96.
  - T6 (the ceiling run) is decided at B3, by the measured ρ under the rule as written.
- **B1 is granted:** T1 → T2 → T3a → T4 → T5.
  - B's binary is built from `git archive 1f354c20b`, and every record carries `--source-commit`.
  - **An added host condition** for timed runs, as the gate used: before each run, wait while the 1-minute load average is above 8, until it is below 6; record the load at start and end. Other agents are building intermittently. Memory figures are deterministic; timings carry the recorded load.
  - Send `--project` after T2. B2 and B3 are separate grants.

## F1b: rulings on RV17's review (ROOT, 2026-09-29)

- **Verdict accepted:** PASS at `f183e1fa9`, with 0 BLOCKING, 4 SHOULD-FIX and 5 NOTEs (`REVIEW/F1B_REVIEW.md`). All four SHOULD-FIX findings are fixed before merge.
- **RV17-1: A2(c) is re-ruled.**
  - My A2(c) premise, "exact-block cannot select a range-triggered case", is **false**. CX-F and CX-G (a 1 m member, EA/L ≈ 2^16 N/m, a tip load of 3·2^-1016 or 2^-1015 N) are range-triggered, and main publishes them `MECHANICS_SOLVED` through exact-block, with a receipt.
  - **Coexistence still holds, on a different ground.** The selected exact-block arm comes before the W2 arm, so wherever exact-block selects, F1b publishes main's bytes by construction. RV17 confirmed this on all 10 selected runs, on the recorded probe and on its own build.
  - **Fix:** I13 corrects D10, dropping the false step 4 and resting coexistence on the ordering. I13 also adds the brief's positive coexistence test with CX-F or CX-G (F1b equals main in full-envelope sha256, on both entries and in both modes), with a mutant that moves W2 before exact-block, killed.
- **RV17-2 and RV17-3: add the tests.**
  - Pin the nonlinear blocked envelope in full, which kills RV17-M1 (`let linear = true;`).
  - Add a two-spring W2 publication test, which kills RV17-M2 (every spring action taken from the first spring).
- **RV17-4: disclose it; "within the criterion" means trusted publications only.** 8 of the 14 published C1 runs are outside R1's exact references (up to 9.7e-8, and 1.7e-5 for SKEW-LEF-large). All are Sensitive and equal main's same-family Sensitive pattern.
  - This matches the K6 ruling: Sensitive publications carry no accuracy claim at that level, and `gate_check` counts trusted publications.
  - I13 records it in RETURN's C1 section.
- **NOTEs:**
  - N1 (RV17-M4 changes only the refusal text on an input main also refuses): recorded.
  - N2 (the lane's peak is about 16P + 24P′, while the guard estimates 24P for the identity build): recorded in RETURN. K6 measures the lane against both (Q12). The ceiling ruling will consider it; D12 is unaffected.
  - N3 (96 B per n² is the n² coefficient, not a total-heap bound; the largest admitted model is 3.1 MB under the ceiling): recorded. K6's B3 ceiling run measures it.
  - N4 and N5: recorded.
- **Next:** I13 makes the fixes (tests and records), with mutants from clean archives. Then RV17 runs a delta check, followed by hosted CI with the full-SHA dispatch, DEC-025 in a quiet slot (after K6's B1) and GEN-8.

## K6: B1 accepted; B2 granted (ROOT, 2026-09-29)

- **B1 accepted.** Records are committed on the K6 branch at `962dd4e3b`, with the runner fixes made during the slot, each tested and disclosed.
  - 110 of 125 rows ran, all ok. 14 were refused by name, and grid 128×128 was deferred.
  - All 21 cross-mode pairs agree on class, and staged equals SA's entry on every run.
  - N6 held: test E passes after K5's merge.
- **Findings, observations only:**
  - **Kernel sparse memory is linear in members:** the heap slope is 0.999–1.002 for CHAIN, TREE and CONT from 10 to 10,000 members. This confirms §4.8's single claim at kernel level.
  - **F1b's dense estimate is sound in its coefficient:** heap is 1.004–1.057 × 96·n² at 100 members. B2 adds 1,000 members.
  - **F1b's lane estimate:**
    - for CONT, heap-move / 24·P_id is 0.66–1.33, and RSS is 1.77× at 1,000 members;
    - for CHAIN and TREE, other buffers dominate (4.5–9.0×).
    - This bears on RV17-N2 (the lane's peak is about 16P + 24P′) and on the lane ceiling.
  - **The product-level gap:** CONT n10000 sparse is about 250 MiB heap in the kernel, against 4.8–5.2 GiB RSS at product level (F1b's gate). F1b's peak attribution puts most of the product's memory in result-row publication, not the solver.
    - **The dense-scrutiny and lane ceilings therefore cannot be set from kernel figures alone.** V-P's product-level measurements are needed. Recorded for the ceiling ruling.
- **B2 is granted (T3b):** dense and lane-lu at 1,000 members, with a projection of 1.53 h. **Grid 128×128 is admitted into B2's remainder** (E_adm × the measured ρ ≈ 1.54 GiB, within C/2; projected 3–4 min).
  - The two known dense timeouts (N10) run once each, and their stage timings are the record N10 needs.
  - The load-wait rule applies.
- **B3** (the ceiling run) is decided after B2 sets ρ from CHAIN dense at 1,000 members.

## K6: rulings at I15's B2 stop (Q3's premise measure) (ROOT, 2026-09-29)

- **The stop was correct.** The first dense 1,000-member run (CHAIN-n01000-AX, 5 of 5 repeats, 622 s, Sensitive in both modes) confirmed Q3's premise on requested heap (3,319 MiB) and on the macOS physical footprint (3,411 MiB), both below P1's Linux 3,605 MiB. It refuted the premise on RSS (4,782 MiB, above C/2).
  - I15's hypothesis, not tested: macOS keeps freed large blocks resident, so RSS accumulates across the stages' rising and falling heap peaks. Linux glibc unmaps large blocks, so P1's Linux `ru_maxrss` compares with the footprint, not with macOS RSS.
- **Ruling: option (a), with a safety condition.**
  - **Q3's premise and ρ are read on the macOS physical footprint** (`time -l` peak memory footprint; the measure macOS uses for memory pressure). It is the one comparable with P1's Linux RSS. RSS stays recorded beside it on every run.
  - **The watchdog stays on RSS,** at C, as the conservative host guard. It is unchanged.
  - **Added condition:** a run admitted by the footprint rule must also have a projected RSS of at most 0.8·C, where projected RSS = the footprint estimate × the largest RSS-to-footprint ratio measured in the same family and mode. Otherwise it is deferred by name.
    - For B2's remaining dense 1,000-member runs, that is about 4.8 GiB ≤ 6.4 GiB, so admitted.
    - For B3 at C = 16 GiB, about 8.7 GiB ≤ 12.8 GiB, so admitted, subject to B3's own grant.
  - **Why:** the rule exists to keep runs clear of the watchdog and the host safe. The footprint is the macOS measure of memory pressure; the heap cap still bounds live heap deterministically; the RSS watchdog still bounds resident memory; and the added condition keeps expected RSS below the watchdog with margin.
- **Recorded for the ceiling ruling.** At 1,000 members, F1b's dense estimate is exact in heap (1.005 × 96·n²) and close in footprint (1.033), but macOS RSS reaches 1.448 × 96·n².
  - So a product run just under the provisional 6 GiB dense ceiling could show about 8.7 GiB resident on macOS.
  - The ceiling bounds estimated heap, not resident memory. This goes to the owner's ceiling decision, with the product-level gap from B1.
- **Other figures:**
  - dense heap slope 1.975 over 100–1,000 members;
  - the dense `factor` stage takes 78.7 s at 1,000 members for the Cholesky (not an N10 case).
- **B2 resumes** under this ruling: runs 098–108, then grid 128×128. **B3** stays a separate grant.

## F1b merged (ROOT, 2026-09-29)

- **Merged:** [PR1052](https://github.com/sgttomas/chirality/pull/1052) at head `6fa422979`, merge `59cb20073`, 2026-09-29 05:31:23Z. The merge record is `IMPLEMENTATION/F1B_MERGE/RECORD.md`.
- **The gates, on the final head:**
  - RV17 PASSED, with its delta check PASSED as well;
  - hosted CI was green, including Linux's numerical suite, the first cross-platform check of the three hash pins;
  - the dispatch (36524065976) succeeded;
  - DEC-025: F1b's 41 tests are the only suite change, pytest and vitest were clean, and so were the builds;
  - the src-tauri suite: 116 of 116;
  - GEN-8.
- **Native witnesses: ruled as join items** (§7.5), as the brief proposed. The gate covers the product's solve path on both entries, the src-tauri suite covers the desktop's Rust side, and F1b changes no native shell or desktop file.
- **Still provisional, for the owner:** the dense-scrutiny and observation-lane ceilings at 6 GiB. The kernel-to-product gap and macOS RSS inflation are recorded from K6; V-P's product-level runs are needed as well.
- **Next on the facade path:** F2a, after D1 5a.3 is selected, K4 merges, and ROOT sets the budget limits from K6 and V-K.

## D1 revision 5a.3: rulings on V4's delta check at R6 (ROOT, 2026-09-29)

- **Verdict accepted:** NOT VERIFIED, with 0 BLOCKING, 1 SHOULD-FIX (V4-W1) and 2 NOTEs.
- **The design is confirmed:**
  - V4-V1 is resolved, and Lemma E is correct step by step.
  - V4's stress test (95,142 shifted factorizations, 14,186 bounds) found no case of S below the exact norm.
  - "No uncertified step remains" holds: the estimate only screens and chooses σ.
  - The per-block decomposition is sound.
  - RF-LARGE availability is confirmed independently. At 1,000 members, S/‖K̃⁻¹‖₁ is 2^7.09 to 2^7.29 against the exact binary64 norm, and S is at least the norm on all six.
- **V4-W1: fix the evidence in R7, with no design change.**
  - emu6 indexes the Hager–Higham estimator's vectors by elimination rank; K4 indexes them by free position.
  - R7 re-indexes emu6 as K4 does and reruns the estimate-dependent evidence: the mutants, the sweep, the controls and the HH-FOOL figures.
  - It corrects the HH-FOOL statements: the hidden block's miss is 2^37.8 at m40 and 2^97.8 at m100.
  - It records M24 as kept for the derivation, unless another control kills it. K4's control list must not expect HH-SLENDER-m40 to kill M24.
- **V4-W2:** R7 labels M27's "no selection kill at design precisions" as argued, since it rests on the estimate-based condition screen. M27's low-precision kill stands.
- **V4-W3:** R7 states that S is polynomially loose only when the estimate is within about 8× of the norm, and that otherwise availability falls back on Uc. It also states that up to three extra factorizations are charged to the work budget, and that their cost on large models is for K6b and V-K to measure.
- **Next:** DS1 writes R7, V4 runs a delta check, and ROOT selects only after VERIFIED.

## K6: B2 accepted; the dense-path finding (N10); B3 granted (ROOT, 2026-09-29)

- **B2 accepted.** Records and the B2-stop rule are committed on the K6 branch at `3799e3764`.
  - 13 runs were measured.
  - Every RSS stayed below its projection, at 0.61–0.97 of it.
  - Dense heap is 1.005 × 96·n² at 1,000 members, which confirms F1b's dense-estimate coefficient.
  - Factor time scales as n³.
- **Finding N10, confirmed, on main's dense path, not K6's code.** This is recorded as a T3 finding.
  - On RF-LARGE-CHAIN-n01000-ROT and TREE-n01000-AX, the dense Cholesky refuses a pivot at DOF 6001 or 6002 of 6006, after 65–86 s. The O(n⁴) dense pair witness then runs until the 1,800 s kill.
  - The time budget is not checked inside the witness.
  - The sparse path publishes both models as Sensitive, so dense scrutiny both disagrees in class and does not end.
  - F1b's gate part 2 shows the same timeouts at product level, on base and candidate alike. The behaviour predates F1b.
  - **Route:** a kernel follow-up slice, on the T3-close list, to be scheduled by ROOT. The slice bounds the dense witness, with a budget check inside it and a size limit or a cheaper witness above one, and it examines why the dense pivot screen refuses where the skyline passes, since I15's 2j + 2 screen-bound hypothesis is not proven.
  - Until then, product users running dense scrutiny on such models wait until they cancel the background job. The solve runs as a cancellable job.
- **B3 is granted:** K6-CEIL-CHAIN-n01364-AX dense, at C = 16 GiB with the heap cap at 15.5 GiB.
  - It is admitted under the ruled rule: a 7.78 GiB footprint estimate, within 8 GiB, and a projected RSS of 11.29 GiB, within 12.8 GiB.
  - It runs once, alone on the host apart from DS1's single `nice -n 19` emulator process, with the memory guard running.
  - If the dense factor refuses and the witness runs to the timeout, the memory figures at the refusal point are still the record B3 needs.

## K6: B3 accepted (ROOT, 2026-09-29)

- **B3 (the Q4 ceiling run), K6-CEIL-CHAIN-n01364-AX dense, 8,190 DOFs:** ok, 5 repeats, Sensitive, with no factor refusal and no watchdog or heap-cap event. It is committed on the K6 branch.
  - **Heap peak:** 6,459,755,398 B, which is 1.0032 × F1b's 96·n² (6,439,305,600 B) and 1.0027 × F1b's 6 GiB provisional ceiling (6,442,450,944 B).
  - **Footprint:** 1.0078 × 96·n².
  - **RSS:** 1.1010 × 96·n², which is 0.585 of the admission projection.
  - **The macOS RSS excess over footprint is not a function of size.** It was 1.402 at CHAIN n1000 under load and 1.093 at the ceiling.
- **Host context accepted.** A process outside this work (a Codex app-server job in another project folder) ran at nice 0 during B3. B3's recorded quantities are per-process and memory is deterministic, so it is accepted as disclosed. Timing is context only. ROOT did not touch the process.
- **Recorded for the ceiling ruling (owner-facing):**
  - At the kernel level, F1b's dense estimate is accurate to about 0.3% in heap at the ceiling. Actual resident memory on macOS runs 1.1–1.45× above it.
  - At product level the overhead is larger (K6 B1 and F1b's attribution).
  - The ceiling's final value is the owner's decision, with V-P's product-level measurements.
- **Next:** C (mutants), then D (records), then review.

## D1 revision 5a.3 SELECTED (ROOT, 2026-09-29)

- **V4's verdict at R7: VERIFIED**, with 0 BLOCKING, 0 SHOULD-FIX and 0 NOTEs (`REV_5A3_CANDIDATE/V4_VERIFICATION.md`, "Delta check at R7").
  - V4 reproduced DS1's claims with its own emulator, confirmed that the estimator's re-indexing matches K4's `condition`, and read all 57 hunks of the R6→R7 diff.
- **Selected: D1 revision 5a.3**, as specified by `REV_5A3_CANDIDATE/D1_REV_5A3_SSTAR_RESOLUTION_R7.md` (sha256 `5502aef9803f05f42e93c2a05a0de495401e3f94fe92adb19fc2a0c3f80d8f42`).
  - The governing text is R7's **§5, "The exact amended text (addendum blocks)"**, §5.1 to §5.9. It amends `DESIGN.md` revision 5a.2 at §4.1.3, §4.1.4, §4.1.6 (with the new §4.1.6.1 item 6a, §4.1.6.2 and §4.1.6.3), §4.1.9 and §5 item 1.
  - `DESIGN.md` stays hash-pinned at 5a.2. The addendum governs where it applies, and R7's derivation sections (§2 to §4 and §6 to §10) are its warrant.
  - The earlier candidates (the candidate and R2 to R6) and V4's intermediate verdicts are history.
- **What the selection establishes:**
  - W1a's honesty guarantee: every accepted precision satisfies b, derived with **no uncertified step**. The Hager–Higham estimate only screens and chooses the shift.
  - The formation charge with the certified bound B = min(Uc, S), per block. Uc is the comparison-matrix bound (Lemma D), and S is the shifted-factorization bound (Lemma E).
  - The Φ floor at 512, the verification estimate at W ≤ V/4, the hybrid bounded-denominator gate, and the best-state bounded test.
  - **Owner-list update:** the earlier limitation, "the guarantee rests to a factor F on an uncertified estimate", is **withdrawn**. R4's F was replaced by the certified bounds.
- **Open or argued, from V4's list, none of which is a step of the honesty guarantee:**
  - **Kept for the derivation, with no kill:** M17, M21, M24, and M27 at design precision.
  - **Availability-only kills:** M20, M23, M25, M26, M28 and M29.
  - **Argued:** M27's no-kill claim; the θ-binding interpretation of THETA-STUB-COUPLED; LEVER2's outcome under every elimination order.
  - **Measured, not proved:** the availability figures, and that no block is both est-fooled and Uc-loose.
  - **Standing premises:** Lemma B's count as V4 recounted it against the Rust; Lemmas D and E tied to K4's factor loop as read; K* nonsingular per body.
  - **Out of scope for now:** W1b until F3 meets its §6.5 obligations; the shift's cost on large models (K6b, V-K).
- **K4's obligations, now unblocked.** K4 implements R7 §5 in Rust, within its write set (`FK/structural/retained/**`), and adds:
  - directed wide rounding (every operation up, the one denominator down, per Lemmas D and E);
  - the shifted-pivot variant of the factor loop;
  - per-block bounds (Uc_c, S_c, B_c) over the free-free pattern's connected blocks;
  - W's contributions at q_W = min(3p + 64, 1024);
  - the charge C_q, the θ and g checks, W⁺, E, V, Φ, and the hybrid gate.
  - **Tests:**
    - E-UNIT, E-HEADROOM, E-ESTIMATE, E-CHARGE, E-UC and SD-G5;
    - controls: LEVER2 at three gains, TILT-LEVER, SEEDED-COMMON, SEEDED-SOFT, CHARGE-SLENDER, HH-FOOL-m, HH-SLENDER-m40 and THETA-STUB-COUPLED, and RF-LARGE at 10 and 100 members. **HH-SLENDER-m40 must not be expected to kill M24.**
    - mutants: those R7 §7 lists as killable, with the kept-for-derivation guards recorded as such.
  - K4's `retained/factor.rs` is the loop Lemmas D and E were read against. Any change to its pivot or rounding logic reopens those lemmas.

## K4: rulings on I12's A3-0 plan for revision 5a.3 (ROOT, 2026-09-29)

I12's plan (`IMPLEMENTATION/K4/PLAN_A3_5A3.md`, sha256 `07186550…`, 784 lines, committed on the K4 branch) is **approved**, with the rulings below.
- **The principle for every question:** where a choice bears on honesty, R7's text governs. Where it affects availability only, K4's practical form is accepted, and any difference from emu7 must be explained in RETURN.
- `factor.rs`'s `factor()`, `pivot_passes`, `negative_pair`, `solve_scaled` and `solve` stay byte-identical, which keeps Lemmas D and E tied to the loop as read.

**The questions:**
1. **Q1:** GEN is the bit oracle, and emu7 is the selection-level cross-check. Explain every difference.
2. **Q2:** ‖ā_q S‖₁'s stages are rounded upward, per R7 item 6. This bears on honesty, so R7 governs.
3. **Q3:** K4's est_c rounding (block sums rounded once, then a division) is accepted. It is availability only, and RETURN states that.
4. **Q4:** the read-only est_c observer in `condition` and the L and D read accessors are accepted as outside "pivot or rounding logic". A test pins `factor()`'s L and D bits unchanged.
5. **Q5:** the shifted loop is a separate operation-for-operation copy, bound by the loop-parity test and K4-M35.
6. **Q6:** best-state comparison is exact, with `ExactWideSum::add_product_of`, as recommended. A span overflow stops with `Span`, and that is recorded as an availability outcome.
7. **Q7:** use emu7's rejection order: (a), (b), `uc`, θ, g, then (d).
8. **Q8:** E rounding to +∞ gives a terminal `Unresolved(ResolutionScaleUnencodable)`, which F2a maps to `receipt_encoding`.
9. **Q9:** a body with no data block has no B_b entry and θ = 0. F2a and D2 confirm this at their slices; routed.
10. **Q10:** E-ESTIMATE's reference is GEN's exact rational solution.
11. **Q11:** E-HEADROOM uses the P state's own ê. At a verification it is the stop rule's E.
12. **Q12:** accepted, on one condition.
    - A support group's E is one exact sum over all contributors of its components, rounded once, in the direction R7 §3.1 prescribes for E.
    - RETURN must derive that this form meets every requirement R7's lemmas place on E for the group's published magnitude. It is not measured by emu7.
    - The reviewer checks the derivation. It is honesty-relevant, because ê sets both V and the allowances.
13. **Q13:** measure RF-LARGE-100's debug runtime at A3b. Any model over 60 s moves to an `--ignored` lane, with CHAIN-AX and TREE-AX kept in the default suite.
14. **Q14:** pin CEIL5A3 for K4-M24 only after GEN confirms that both operands are selected and the combination is Unresolved.
15. **Q15:** yes. Include the low-precision stress that kills M27.
16. **Q16:** `not_covered` stays empty in K4, since F2a owns the not-covered classes. Remove the pending marker at D.
17. **Q17:** d^b is formed lazily, only when the gate falls back. Work stays charged where it is incurred.
18. **Q18:** confirmed. The verification pass runs only on states used as verifications, the 2p state.

**Checkpoints:** A3a (the bounds and E, with no behaviour change), then A3b (the method and every control), then B, C and D.
- **Host:** as the plan states — one cargo job, `-j 4`, `RUST_TEST_THREADS=2`, and memguard checks.

## K6: rulings at C and D (ROOT, 2026-09-29; the C ruling was given in session and is recorded here)

- **C's stop:** K6-M4 (the watchdog kills the child only) and K6-M26 (the resume skip counts a not_run record as measured) survived the first pass.
  - ROOT approved I15's two tests, added to existing classes in `runner/test_k6_runner.py`: `LiveLimit.test_kills_take_the_whole_group` and `PlanAdmission.test_measured_runs_are_kept_and_not_run_records_are_run`.
  - After the re-runs, 26 of 26 are killed (K6 branch, `014b2ae04`).
- **D accepted:** committed on the K6 branch at `ae3320b5a`. Main `59cb20073` (F1b) is merged in at `3e90176c6`, and H's suite and test E pass there. No product byte changes (Scope 8).
  - **Disclosed:** the observation packet was written at D, not at B, within the write set, and it regenerates byte for byte from the committed records.
  - **Disclosed:** the provenance lock copy no longer matches `H/Cargo.lock`. Nothing reads it, and it is outside K6's write set.
  - **A limit:** the binary's time-budget and first-repeat stops were reached by no observation and have no dedicated test. The reviewer judges whether a test is needed before merge.
- **Next:** PR, an independent reviewer (RV18), hosted CI with the full-SHA dispatch, DEC-025 and GEN-8.

## K6: rulings on RV18's review (ROOT, 2026-09-29)

- **Verdict accepted:** PASS at `ae3320b5a`, with 0 BLOCKING, 4 SHOULD-FIX and 8 NOTEs (`REVIEW/K6_REVIEW.md`).
  - RV18 verified independently that no product byte changes, that test E is strict, and that the allocator's accounting and the refusals are right.
  - It found that the counts match its own port, the models match R1, and the packet and D's tables regenerate byte for byte.
- **All four SHOULD-FIX findings are fixed before merge,** each killing RV18's mutant from a clean archive:
  - **RV18-1:** a `k6_bin` test of the time-budget and first-repeat stops (RV18-M1, M2).
  - **RV18-2:** a `k6_bin` test that the summary's heap peak equals the maximum of the stage peaks, and that stage peaks restart (RV18-M3, M4).
  - **RV18-3:** the runner kills the observation process group in a `finally` and maps SIGTERM to a clean exit, with a no-survivor test for a SIGTERM to the runner mid-run.
  - **RV18-4:** scrub the four machine-path lines in `_run_records/c/logs/py-K6-M5.log`, and regenerate the K6 SHA256SUMS.
- **NOTEs fixed where cheap:**
  - N1–N3: tests for the poll interval (M6), home-path scrubbing (M7) and the DEC-025-sweep check (M8). Also scrub `time -v`'s output file on Linux.
  - N6: key CONT n10000's lane-id refusal on the canonical model's family and size, not its id, with a test.
  - N7: correct CHANGE_RECORD's base (`56dd72334`). Note in RETURN that run 137's `slot` reads B1 though it ran in B2, since raw records are not edited, and that the provenance lock copy was already stale on main.
  - N4, N5 and N8: recorded.
- **Next:** I15 fixes, then RV18 runs a delta check, followed by hosted CI with the dispatch, DEC-025 and GEN-8.

## K6 merged (ROOT, 2026-09-29)

- **Merged:** [PR1053](https://github.com/sgttomas/chirality/pull/1053) at head `cd325c1fe`, merge `7ac7b1c37`, 2026-09-29 09:52:09Z. The merge record is `IMPLEMENTATION/K6_MERGE/RECORD.md`.
- **The gates:**
  - RV18 PASSED, with its delta check PASSED;
  - hosted CI was green, and the dispatch (36548351414) succeeded;
  - GEN-8 passed;
  - DEC-025: K6's own tests are the only suite change, and pytest, vitest and the builds are clean.
- **operation_applier's 0-test result in the sweep** was a build failure in the shared sweep target: two `serde_json` versions, the mechanism not proven. It re-ran alone on the head with a fresh target and passed 194 of 194. ROOT removed the shared target.
  - **Procedure note for DEC-025:** start each sweep with a fresh target.
- **Next on the kernel path:**
  - K4 (A3b in progress);
  - then K6b (W1 observations) and V-K;
  - then ROOT's budget limits, F2a, and the owner's ceiling decision with V-P.

## K4: rulings at A3b (ROOT, 2026-09-29)

- **A3b accepted,** committed on the K4 branch at `bb7757ac5`.
  - Every control equals GEN's outcomes, and every selected control is honest and passes G5a.
  - RF-LARGE at 10 and 100 members is honest at 128.
  - E-CHARGE is bit-equal to GEN on 360 states.
  - `factor.rs` is unchanged since A3a.
- **Outcomes that moved from 5a.2 are accepted,** as the selected design's intended behaviour:
  1. **DIRECTIONAL-SPAN: 128 → Unresolved(Ceiling).** [Correction (I12 at B): 5a.2 selected it at **256**, not 128, so the change is 256 → Unresolved(Ceiling). 5a.2's publication was within b, checked against GEN's exact solution (worst ratio 1.5e-24; RETURN §22.3). It was not a false claim: 5a.3 withholds a correct publication, which is the availability loss accepted below.]
     - Its springs span R³ only through 2^-52 (κ ≈ 2^104). The solve's error stays above V/4 at every precision, so 5a.3 withholds it.
     - This is an availability loss, and it is honest.
     - **RETURN must state whether 5a.2's 128-bit publication was within b,** checked against GEN's exact solution. If it was not, it is a false claim that 5a.3 now withholds, and it is recorded as such.
     - DIRECTIONAL-WELL keeps the directional-spring recovery compared.
  2. **CEIL-A and CEIL-B → Unresolved(ResolutionScaleUnencodable).** E overflows under a 2^1013-rad rigid rotation (Q8). F-1's control moves to CEIL-S (P = 2^900, ε = 2^-160).
  3. **RIGID-UNLOADED: Unresolved → 512,** as R7 predicts.
- **The edge case, E finite while ê = fl(L_b·E_fo) overflows:** map it to `Unresolved(ResolutionScaleUnencodable)`, the same terminal reason as an E overflow, with a unit test. Do not escalate: an infinite V can never pass. Currently it ends as `Arithmetic(NonFinite)`, which is withheld and honest, but mislabelled.
- **Figures:** asserting at R7's two-figure rounding is accepted.
- **RETURN §22,** the Q12 support-group E derivation and the certified-bound conditions, is noted for the reviewer.
- **Next:**
  - **B:** FK, SD and NI suites; the 39-manifest suites on the Mac; K4 is kernel only, so T9 and the gate are not run (K4's brief).
  - **C:** mutants, including R7 §7's killable list and K4-M34 to M40.
  - **D:** records.

## K4: B accepted (ROOT, 2026-09-29)

- **B accepted.** It is committed on the K4 branch at `8a59458a1`, with main `7ac7b1c37` (K6) merged in at `8f8023a20` and no FK overlap.
  - The 39 manifests were run on main's piping source with K4's FK overlaid. Only frame_kernel changes: 267 → 389, K4's 122 tests. The only failures are the three known Mac ones.
  - FK 389, SD 30 and NI 134 pass.
  - The path and dependency scan shows no product path reaches K4's code: everything is `pub(crate)` inside a private `mod retained`. So T9 and the gate are not run.
- **Both A3b additions are done:**
  - DIRECTIONAL-SPAN's 5a.2 publication is shown within b, with a test;
  - the ê-overflow edge case maps to `ResolutionScaleUnencodable`, with a unit test and a new control, EHAT-OVERFLOW.
- **Next:** C (mutants), then D (records), then review.
