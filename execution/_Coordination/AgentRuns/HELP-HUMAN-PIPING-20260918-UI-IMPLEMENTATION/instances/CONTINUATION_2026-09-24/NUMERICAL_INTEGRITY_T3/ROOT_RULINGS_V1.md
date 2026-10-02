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

## K4: C accepted; the honesty predicate tightened at D (ROOT, 2026-09-29)

- **C accepted.**
  - Every killable mutant is killed: R7 §7's list, K4-M34 to M40, and K4's earlier mutants and D-series.
  - K4-M24's CEIL5A3 kill is confirmed.
  - The four derivation guards (R7-M17, M21, M24, and M27 at design precision) move no control. As R7 expects, each is caught at evidence or unit level.
  - The NONE controls pass.
- **I12's note 4: tighten the honesty predicate at D.**
  - The controls' `compare_honest` accepts 1e-9 relative, or the absolute bound for absolute-verified rows. That is far looser than the published claim, so some mutants' false claims show only as selection changes.
  - At D, `compare_honest` checks each selected row against the claim it publishes: its exact bound, with the binary64 publication rounding stated, where the row carries one; otherwise the stop rule's relative bound, plus the publication rounding.
  - Then re-run the controls test, and the evidence pass for R7-M1, K4-M24 and K4-M37. Record which of their false claims are now caught as dishonest.
  - If a selected unmutated control fails the tightened check, that is a **stop**: an honesty finding.
- **Notes 1–3 are recorded:**
  - REACTIONS-ONLY stays at 256 under R7-M3 in K4;
  - K4-M12 and D13 are killed at bit level only, since the corrections repair the fold;
  - K4-M30 now describes the design, and its inverse, K4-M30i, is killed.
- **Next:** D.

## K4: D accepted; to review (ROOT, 2026-09-29)

- **D accepted.** ROOT committed it for I12 on the K4 branch at `7d8fa9c0e`; I12 made no Git write.
  - ROOT checked the five files' sha256 against I12's return, the `_run_records/` `SHA256SUMS` (214 files; its own sha256 `f79cfe57…`), rustfmt on the three test files, and a scan of the records for machine paths and model identifiers. All pass.
- **The tightened predicate meets the C ruling.** `compare_honest` checks each selected row against the claim it publishes, with the binary64 rounding stated (RETURN §22.9). No selected unmutated control fails it: 99 controls and 5,490 rows, the worst at 0.28 of its allowance; the recovery test's 43 models pass. So there is no stop.
- **The evidence pass is recorded as returned:** F-2-SPOS (R7-M1) and CEIL5A3 (K4-M24) are newly caught as dishonest; all eight K4-M37 moves are caught.
- **PRESCRIBED-TAIL and PRESCRIBED-TAIL-FREE under R7-M1: accepted as a limit of the test's binary64 expectations,** not of the method.
  - The false b = 0 at 128 is real, as R7 counts it. Its truth, 2^-1091 N, lies below binary64's smallest subnormal, so a binary64 expectation cannot show the claim failing.
  - The mutant is killed regardless, by its selection changes; unmutated, the case is selected at 512 and its floored b covers the truth.
  - **Routed to the T3-close list:** expectation files that mark an exact value as nonzero (an underflow marker), so that a false b = 0 on such a row is caught directly. V-K may carry it.
- **I12's two new arguments go to the reviewer; ROOT adopts neither as design text.**
  1. **§6 item 4:** that θ ≤ 1/2 with the certified B forces a data-carrying block's K\* to be nonsingular, by Lemma C's Neumann step. This is I12's reading of R7's proof, not a sentence R7 states. The honesty guarantee does not rest on it: the no-data-block residue stays under R7's standing premise, "K\* nonsingular per body".
  2. **§9 step 12:** route 1's constant under the bounded majorant, ‖Ā‖₁ ≤ 9·g_max·‖|K|_contrib‖₁, derived in the unscaled 1-norm and argued in the equilibrated norm. It is availability, not honesty, and no 1024 verification on the controls uses the bounded gate. It is recorded as one of K4's limits (RETURN §19).
- **Next:**
  - the PR, with hosted CI and the full-SHA dispatch;
  - an independent reviewer (RV19), directed to RETURN §22.1, §22.2, §9 steps 10 to 12, §6 item 4 and §22.9;
  - then DEC-025 with a fresh target, GEN-8, the merge and the merge record.

## K4: rulings on RV19's review (ROOT, 2026-09-29)

RV19 (`REVIEW/K4_REVIEW.md`, sha256 `d906539e…`; records `REVIEW/_run_records/k4_review/`, 47 files and `SHA256SUMS` `8b8e2beb…`) reviewed head `7d8fa9c0e`: **FAIL**, with 1 BLOCKING, 5 SHOULD-FIX and 7 NOTEs. ROOT checked RV19's records (`SHA256SUMS` passes; no machine paths) and read RV19-1's code at `adaptive.rs:1496-1544` and `:2159-2176`.

- **RV19-1 (BLOCKING) is confirmed. Fix it before merge.**
  - `scales_at`, the stop rule's S\*, skips only input-derived rows, so a row the candidate cannot publish (binary64 overflow or underflow) still enters S\*_2p. `classify_rows_floored` excludes such rows from S\*_pub, as O9 requires. The two scales can then differ by any factor, and R7 §5.2's premise, that S\*_pub is within a relative 2^-64 + 2^-52 of S\*_2p, fails. OVF-ROT-928 publishes Rz = −2^901 as `relative_verified` against an exact 0.
  - **The fix:** apply O9 to the stop rule. `scales_at` skips every row whose candidate value has no binary64 value, so both scales are formed from the same rows. This only removes rows from S\*, which makes the stop rule stricter. It changes availability, never honesty. I12 checks every other user of `scales_at` and states the argument in RETURN.
  - **Add OVF-ROT-928 as a control**, in GEN and `outcomes.txt`. Add the reverted fix as a mutant killed by it. OVF-ROT-900 stays honest and unchanged.
  - RV19's probe of this fix moved no control. I12 confirms that on the full suite, the N5 streams included.
- **RV19-6 is ruled as a design defect that K4 fixes now: D1 revision 5a.3, amendment A1.** A selected row outside its claim is BLOCKING by K4's brief, whatever its cause.
  - **The defect.** D1 §4.1.6's revision-4 rule classes every row `absolute_verified` once S\* < 2^-988, with b = fl↑(2^-64·S\*). R7 §5.2's published-value bound, |q_pub − q\*| ≤ b·(1 + 2^-22), assumes |q| < 2^-34·S\*, which that rule does not give. The binary64 rounding of a row near S\* is then up to 2^11·b. TINY-S-995 misses its claim on seven rows, by 222 to 819 times.
  - **The amendment.** Where 0 < S\* < 2^-988, each `absolute_verified` row carries its own bound, which includes its publication rounding:
    - b_row = fl↑(fl↑(2^-64·S\*) + fl↑(2^-53·|q_pub|) + 2^-1074).
    - **Derivation:** |q_pub − q\*| ≤ |q_pub − q_p| + |q_p − q\*|. Rounding to nearest gives |q_pub − q_p| ≤ 2^-53·|q_pub| + 2^-1075. The accepted candidate gives |q_p − q\*| ≤ 2^-64·S\* within the factor R7 §5.2 already carries. So |q_pub − q\*| ≤ b_row·(1 + 2^-22), or 2^-21 at 512.
    - **Why only there.** Where S\* ≥ 2^-988, an absolute row has |q| < 2^-34·S\*, and its rounding is below 2^-23·b, as R7 §5.2 argues. So b is unchanged there, and b = 0 is unchanged at S\* = 0.
    - **Relative rows** cannot occur where S\* < 2^-988, and where S\* ≥ 2^-988 they are normal numbers. So their claim is unchanged.
  - **K4 implements the amendment.** GEN follows it. TINY-S-995 becomes a control, with an expectation precise enough that the unamended code fails it: a mutant reverting the amendment must be killed. Any control whose bound bits change is listed.
  - **Routed:** D2's G5 checks b_row (D2's input list). RV19 checks the derivation and the implementation at its delta check. ROOT adopts the amendment into the design text as a ruling, because `DESIGN.md` stays hash-pinned.
- **RV19-2 (SHOULD-FIX): fix it in K4.**
  - `compare_honest` fails, instead of skipping, when a published row with a value has no expectation.
  - An `Underflow` or `Overflow` row is checked against its exact value's range. **The expectation files carry a range marker** for exact values outside binary64's range: underflow (with the sign) or overflow. This is the marker ROOT routed at D, now done in K4. It is removed from the T3-close list. With it, R7-M1's false b = 0 on PRESCRIBED-TAIL and PRESCRIBED-TAIL-FREE should be caught directly; record whether it is.
  - **The 13 selected controls without expectations** get GEN's exact expectations where GEN can compute them (HH-FOOL, HH-SLENDER-m40, N03-RX, R115-SEED3 and RF-LARGE at 10 members). Any that cannot are named, with the reason.
  - **RF-LARGE at 100 members:** run G5a on the six frames, and `compare_honest` where exact expectations are practical. Otherwise correct RETURN §12.6, §22.8 and CHANGE_RECORD to say exactly what is checked.
  - Every statement of the form "every selected control" must match what the tests check.
- **RV19-3 (SHOULD-FIX): fix it in K4.** `RetainedCombination::solve` refuses operands whose stations or support groups differ, with a `CombinationReason`, and a test (RV19's t = 0.25 against 0.75 case).
- **RV19-4 (SHOULD-FIX): fix it in K4.** Add a control with a support group carrying a directional spring, so that RV19-M6 is killed.
- **RV19-5 (SHOULD-FIX): fix it in K4.** Add a unit test that pins ‖SĀS‖ as the larger of the 1-norm and the ∞-norm, killing RV19-M2.
- **§6 item 4: confirmed by RV19.** ROOT adopts it as a ruling. θ_c ≤ 1/2 with the certified B_c makes a data-carrying block's K\*_c nonsingular, by Lemma C's Neumann step, under Lemma B's standing premise. A block with no data stays under R7's premise, "K\* nonsingular per body".
- **The NOTEs:**
  - N1 and N2 are corrected in RETURN.
  - N6: the tests' `powi` is replaced by exact constants.
  - N4 (the release-mode `break` in `wide_sum`): make it a returned error, if that is small; otherwise record it.
  - N3, N5 and N7 are recorded; strengthening them is optional.
- **After the fixes:**
  - re-run FK's full suite and K4's suite, with the controls token-equal to GEN (with the new controls);
  - `gen_k4_vectors.py --check`;
  - the mutants the fixes touch, and the new ones: the reverted RV19-1 fix, the reverted amendment, RV19-M2 and RV19-M6;
  - the evidence pass for R7-M1, K4-M24 and K4-M37 under the new checks;
  - then RETURN addendum 1 and the records. RV19 checks the delta.

## K4: rulings on RV19's delta check at a5fa0eaf7 (ROOT, 2026-09-29)

- **RV19's delta check PASSES:** 0 BLOCKING, 1 SHOULD-FIX and 4 NOTEs (`REVIEW/K4_REVIEW.md`, "Delta check at a5fa0eaf7", sha256 `319701f6…`; records `REVIEW/_run_records/k4_review/delta/`, with `SHA256SUMS` now covering 80 files, `ab9f24d8…`).
  - RV19-1 is closed on both the overflow and the underflow side.
  - Amendment A1 is independently re-derived and implemented as ruled. Only PT-B's and PTF-B's bounds change, and TINY-S-995 kills the reverted amendment.
  - RV19's oracle finds every selected publication honest: 120 publications (8,272 rows) and the six 100-member frames (15,378 rows).
- **The gates on `a5fa0eaf7` are green:**
  - hosted CI: runs 36584733672, 36584733559, 36584733590 and 36584733821;
  - the full-SHA dispatch 36584771469 (target_base `7ac7b1c37`);
  - DEC-025, clean;
  - GEN-8.
  - Since the head changes for RV19-D4, all of them re-run on the final head.
- **RV19-D4 (SHOULD-FIX): fix it before merge.** It is a test gap, but on the fix of a BLOCKING honesty finding. Add an SD-G5-style vector on `decide` in which the candidate overflows and the verification does not, and the underflow pair. Assert that such a row sets no S\*. Show RV19's mutant RV19-D4 killed.
- **DN2:** RETURN's "every selected … combination" must match what is tested. Either give PRECISION-RULE expectations and run it, or correct the wording.
- **DN1, DN3 and DN4 are recorded.** They affect no check.

## K4 merged (ROOT, 2026-09-29)

- **Merged:** [PR1054](https://github.com/sgttomas/chirality/pull/1054) at head `5a46a6278`, merge `ab02ee3a6`, 2026-09-29 16:25:25Z. The merge record is `IMPLEMENTATION/K4_MERGE/RECORD.md`.
- **The gates:**
  - RV19 FAILED the first head on RV19-1, a false publication. It PASSED the fixes on its delta check and confirmed the final head with no findings;
  - hosted CI was green, and the dispatch (36593106169) succeeded. The numerical job took 18.3 to 20.1 min;
  - GEN-8 passed;
  - DEC-025, with a fresh sweep target: K4's own tests are the only suite change, and pytest, vitest and the builds are clean.
- **D1 revision 5a.3 now includes amendment A1** (the per-row b where 0 < S\* < 2^-988), by ROOT's ruling on RV19-6. `DESIGN.md` stays hash-pinned.
- **Procedure note for DEC-025:** remove the canonical sweep summary from the sweep worktree after copying it. A leftover summary makes the next sweep see a dirty tree.
- **Next on the kernel path:**
  - K6b (I16) and V-K (I17), from main `ab02ee3a6`. K6b's A0 adds the `retained` export, and V-K reuses that commit;
  - then ROOT's W1 limits, F2a, and the owner's ceiling decision with V-P.

## K6b and V-K: spawn (ROOT, 2026-09-29)

- **K6b is spawned as I16** (`TASK_BRIEFS/I16_K6B_IMPLEMENTATION.md`), on branch `codex/piping-k6b-20260929` in `<wt>/k6b`, from main `ab02ee3a6` (K4 merged).
- **V-K is spawned as I17** (`TASK_BRIEFS/I17_VK_IMPLEMENTATION.md`), on branch `codex/piping-vk-20260929` in `<wt>/vk`, from the same main. ROOT cherry-picks K6b's A0 export commit onto it once that commit exists.
- **The briefs cite K4 at `7d8fa9c0e`.** The merged head is `5a46a6278`. RETURN §16's export list is unchanged by the review fixes, apart from `CombinationReason::OperandsDiffer` (RV19-3). Re-locate every line on main.
- **Checkpoint 0 is read and design only** for both. There is no build until ROOT approves each plan.

## V-K: rulings on I17's checkpoint-0 plan (ROOT, 2026-09-29)

The plan is `IMPLEMENTATION/VK/PLAN_CHECKPOINT0.md` on the V-K branch (sha256 `adaf697a…`; committed by ROOT). **It is approved, with these rulings.**

- **Q1:** V-P adds `product_physics`; V-K does not.
- **Q2:** the generated, committed cases file with R1's sha256 pinned and `--check`, is approved.
  - Expected values are R1's decimal strings, byte for byte.
  - Comparisons are decided exactly by an in-crate big-integer engine, with no registry dependency. The engine gets its own unit tests, including against Python's `fractions` on sampled rows, and the reviewer checks it.
  - The 1,000- and 10,000-member models are generated on demand, with their sha256 committed.
- **Q3:** CI runs every case up to 100 members. RF-LARGE at 1,000 and 10,000 members run as examples. Report VR's measured CI time at A1; more than 3 min added is reported.
- **Q4:** the 14 seeded faults are approved.
  - VK-F06, VK-F07 and VK-R28 may be killed on evidence: the outcome, attempts and selected precision against V-K's committed per-case records. These are returned evidence, which F2a publishes (O5's condition).
  - VK-F17's extra check is approved: every `not_covered` comparison's row must be `absolute_verified`.
  - **The selector** (`K4R/seeded.rs`, cfg-gated, reading `FK_SEEDED_FAULT` once) and its cfg-gated `mod` line count as fault sites. An unknown id panics.
  - With the feature off and outside `cfg(test)`, the code is absent. FK's own test build must be unchanged in effect: FK's suite passes with the variable unset.
- **Q5:** approved. V-K runs its own models, one release process each, under K6's runner, with the heap cap at C − 512 MiB.
  - The ascent is 100 → 1,000 → 10,000. 10,000 members run only as ROOT approves, and after K6b's timed runs.
  - A 30-minute slot per tier. V-K's scale runs record the outcome, work and time with its load, and are not a timing claim; timing is K6b's.
- **Q6 and Q7:** approved.
  - The one-off bit comparison against K4's `r1_cases.txt` and `r1_large.txt` is recorded. Every difference must be one of the plan's §4.4 list, explained.
  - Bit identity is asserted for list permutations only.
- **Q8 (the export) binds K6b's A0 as well, and extends it.** RETURN §16's list names types, but a consumer outside FK also needs:
  - the fields of the plain-data input types, to build a source: `SourceParts`, `StraightMember`, `Spring`, `DirectionalSpring`, `Constraint`, `NodalLoad`, `Station`, `SupportGroup` and `Dof`;
  - the fields of the output and evidence records, to read them: `Publication`, `PublishedRow`, `RetainedEvidence`, `AttemptRecord`, `StageWork`, `StorageCounts` and the other evidence types in the list;
  - the methods `Binary64Outcome::value`, `Component::{index, from_index, ALL}` and `Dof::{global, from_global}`.
  - **Types whose invariants matter keep private fields and public accessors:** `PrimitiveSource` (built only through `new`), `RetainedSolve`, `PrecisionState`, `CaseLimit` and `InvocationMeter`.
  - **I16 implements A0 and I17 confirms it covers V-K's needs** before ROOT commits A0. The exact list is recorded in A0's commit message and in both RETURNs.
  - Nothing that lets a caller supply a matrix, factor, closure or label becomes public.
- **The conflicts:**
  - **C1 (RF-RANGE twist and extension):** the exact power-of-two pre-scaling is approved. It must give bits identical to §4.10's formula wherever that formula's k_t and k_a are finite and normal, and a test asserts this on every such case. Record the derivation.
  - **C2:** approved as planned. §7.3 items 5, 16, 3 and the relabelling half of 10 are not seeded in V-K, and V-K adds no cases outside R1. For each, cite K4's killing control from K4's mutation table, or record it as undiscriminated by R1.
  - **C3:** the kill on evidence for §7.3 item 7 is approved; the design's "recovered" is recorded as inexact.
  - **C4:** the parity items are §4.8 items 1–3, at up to 100 members.
  - **C5:** confirmed. V-K's scale runs use R1's own rounding and are separate from K6b's.
  - **C7:** approved. No site goes in the byte-identical `factor.rs` functions or in `bound.rs`'s shift loop. The scans run before any site is written, and a flagged site is a stop.
  - **C8:** `hypot` is decided exactly.
  - **C9:** compute it both ways and report.
  - **C10:** a k = 0 spring refused by K4's source counts as RF-MECH's refusal only if the refusal names it (the `SourceError`). Record it.
  - C11 and C12 are recorded.
- **Next:**
  - A1 without the export, now: the generator, the cases file, the exact engine and the parts of VR that do not call `retained`. One cargo job at `-j 4`.
  - A1's kernel lane waits for A0. ROOT cherry-picks A0 onto the V-K branch once it is committed on K6b's.

## K6b: rulings on I16's checkpoint-0 plan, and A0's export (ROOT, 2026-09-29)

The plan is `IMPLEMENTATION/K6B/PLAN_CHECKPOINT0.md` on the K6b branch (sha256 `7d6b7af3…`; committed by ROOT at `c0436769f`). **It is approved, with these rulings.**

- **C-1 (the flat `pub use` does not compile): the facade module is approved.** Declare `pub mod retained_api { pub use super::retained::…; }` in `FK/structural.rs`, right after the private `mod retained;`. K4's names stay as they are (`adaptive::POLICY` included), nothing new enters `structural`'s namespace, and the product scan is one grep for `retained_api`.
- **C-2 (`PrecisionState` exposes the private `Solved`):** `PrecisionState` and `RetainedSolve::state` stay crate-private. No `#[allow(private_interfaces)]`. Neither K6b nor V-K uses them. If F2a needs the retained-state digest (§4.1.8), it asks for an accessor that exposes no private type.
- **What A0 exports** is RETURN §16's list, as extended by "V-K: rulings on I17's checkpoint-0 plan" Q8:
  - the fields of the plain-data input types and of the output and evidence records;
  - the methods `Binary64Outcome::value`, `Component::{index, from_index, ALL}` and `Dof::{global, from_global}`;
  - `Binary64Outcome`'s variants, which become public with the enum;
  - the fields of `AttemptWork`, `WidthWork` and `SumWork` only if V-K or K6b reads them. I17 says whether V-K needs them.
  - **Types whose invariants matter keep private fields:** `PrimitiveSource`, `RetainedSolve`, `CaseLimit` and `InvocationMeter`.
  - Nothing that lets a caller supply a matrix, factor, closure or label becomes public.
  - Remove the `#[allow(dead_code)] // F2a API` markers on items that are now public.
  - **Before ROOT commits A0, I17 confirms it covers V-K's needs.** The final count of items, methods and fields goes in A0's commit message and in both RETURNs.
- **Q1:** approved: option (a), per-precision memory from budget-truncated prefix calls through the public API, with the overshoot derived. The 512 and 1024 increments stay derived; no escalating invented model.
- **Q2:** approved: the mode is `w1a`, with a new JSONL kind `attempt`. "Reused unchanged" means K6's existing kinds keep their schema; adding a kind is allowed.
- **Q3:** the four tiers are approved.
  - `CaseLimit` and `InvocationMeter` are both `u64::MAX`, recorded in each run's start line; a budget outcome is a stop.
  - W1-T4 (10,000 members) runs only as ROOT approves, after W1-T3 has measured ρ.
  - **W1 counts-only runs at 1,000 and 10,000 members during A2 are approved** (O(nnz), no solve, a 512 MiB cap), as for K6's N7.
- **Q4 to Q7:** approved as planned.
  - K6's tiers keep a frozen four-mode tuple, so K6's 138 rows are unchanged.
  - K6b's row checks at R1's 1e-9 are not the honesty check (that is K4's and V-K's); a failure is still a stop.
- **R1's rows in H's CI test:** `include_str!` of `K4T/r1_large.txt` is approved. The test asserts that file's sha256, so a change to it is visible.
- **The brief-to-code conflicts are recorded:**
  - K6's profile counts cannot stand for W1's storage;
  - a wide value takes 8L + 16 bytes;
  - RF-LARGE's memory is dominated by the per-member operators;
  - swapping Iy and Iz is equivalent here;
  - `solve_cases` gets an equality test only.
- **Next:** A0 now. Report it as soon as it compiles and FK's suite passes; one cargo job at `-j 4`. I17 works on V-K's parts that do not call `retained`.

## K6b A0 accepted; the export is on both branches (ROOT, 2026-09-29)

- **A0 is accepted.** ROOT committed it in two commits on the K6b branch:
  - `bb89e4f8f`: FK's `retained_api` export. It is visibility only: 240 `pub(crate)` → `pub` edits, 20 dead-code markers removed, and the facade block. It covers 72 items, 38 methods and 130 fields.
  - `fdbf132d4`: H's test that the export suffices from outside FK.
- **ROOT's checks:**
  - every removed line in FK's diff is a `pub(crate)` or a dead-code marker, and every added line is `pub` or the facade;
  - the eleven files' sha256 equal I16's return;
  - I16 reports FK's full suite at 394 passed, 0 warnings, and rustfmt clean;
  - `retained_api` is named only by `FK/structural.rs` and H's test.
- **I17 confirmed the export covers V-K.** `Kind::{ALL, index}` stay crate-private.
- **The export commit alone is cherry-picked onto the V-K branch** as `3018343c2`, with the same patch-id as `bb89e4f8f`. H's test stays on K6b.
  - Whichever PR merges first carries the export to main. The other merges main, and the identical change merges cleanly.
- **Next:** K6b's A1 (the W1 mode and the adapter), and V-K's A1 kernel lane. Each uses one cargo job.

## V-K: rulings on I17's A1 stop (THIN-A and THIN-B) (ROOT, 2026-09-29)

- **The stop.** RF-RANGE-THIN-A and THIN-B (R1's PHYS-R4 geometry: OD 4e-77 m, L = 1 m) end `Unresolved(Ceiling)` through W1a.
  - The attempts: 128 is rejected by the stop rule, 256 by the stop rule, 512 by the charge test (d), and 1024 verifies with nothing above it.
  - EA/(12EI/L³) ≈ 6.7e152, about 2^507, so no candidate at 512 bits or below can reach the stop rule's 2^-64 accuracy.
  - K4 publishes nothing: the result is honest, but unavailable. V-K's adapter is not the cause, as I17's probe with K4's y_reference shows. Every other of V-K's 201 CI cases passes as projected.
- **Ruling, subject to one confirmation. I17 first runs K4's generator (`K4T/gen_k4_vectors.py`, K4's bit oracle for the design) on THIN-A and THIN-B.** GEN must give the same attempt chain and outcome. If it does, this is the design's limit, not a K4 defect. If it does not, it is a K4 finding and a stop.
- **Given that confirmation, THIN-A and THIN-B are W1a's coverage limit,** and VP-ROBUST's kernel-lane gate (§4.10) is amended as follows:
  - V-K commits an **expected-unresolved list**, THIN-A and THIN-B with their reason, pinned like the `not_covered` list.
  - Those cases must end honestly unresolved, with no rows published.
  - Their comparisons are never counted as passes. They are reported as a separate count.
  - A case that leaves or joins the list blocks the gate until ROOT reviews it.
  - Every other case keeps §4.10's rule: an unsolved case fails.
- **Why this is acceptable:**
  - It is not a product regression. On the ordinary route, W2's scaling solves the PHYS-R4 geometry (K2b's rulings: "PHYS-R4 (b = 536) … solved and accurate").
  - W1 is selected only for Sensitive and D-5-routed cases.
  - W1's ladder tops out at a 512-bit candidate by design (R7). An intrinsic stiffness spread near 2^507 is beyond it, and W1a withholds honestly.
- **Routed:**
  - **To F2a and V-P:** THIN's product standing on the ordinary route, and what F2a publishes if such a case is routed to W1 and W1 ends unresolved.
  - **To the owner's PHYS-R4 decision** (F1b's rulings): this fact is added to it.
  - **To D1 and the T3-close list:** whether W1 should reach beyond 512 (a 1024 candidate with a 2048 verification) or treat decoupled stiffness spreads separately. It is not needed now.
- **The three input-derived rows** (`RF-WEAK-W-AX-rho1e-12` `u.N5.UX/UY/UZ`, at a restrained node): I17's reading is confirmed. Rule 2a makes rows at restrained DOFs `input_derived`, exact prescriptions. So they are exempt from the "`not_covered` implies `absolute_verified`" check, pinned to those three.
- **§7.3 item 5 (no escalation)** is now killable on THIN's attempt evidence. Add it to the seeded faults at A2 (VK-F05).
- **Next, for I17:**
  - the GEN confirmation;
  - the expected-unresolved list and its test;
  - A1's remaining items: §5.3's parity at up to 100 members, and Q7's recorded observations;
  - then return A1.

## K6b: A1 accepted; K4's stop-rule memory finding (ROOT, 2026-09-29)

- **A1 is accepted.** ROOT committed it at `73031b134` on the K6b branch, with 18 files in H.
  - ROOT checked the new files' sha256 against I16's return, that FK has no change beyond A0, and that no machine paths appear.
  - The results:
    - the `w1a` mode, with `attempt` and `prefix` records; the new kind `prefix` is allowed;
    - the adapter's K4SRC bytes equal an independent Python writer's on 24 of 24 RF-LARGE models;
    - the six frames at 10 and 100 members are selected at 128, and R1's predicate passes on every row;
    - H's debug suite grows 65.3 s → 78.8 s.
- **The finding (confirmed by ROOT at `K4R/adaptive.rs:533-590`):**
  - K4's `ExtremeTracker` keeps every row whose 64-bit ratio approximation lies within `WINDOW_ULPS` (2^13) of the running extreme, each as two `ExactWideSum` values, about 4.3 KB, and keeps them until `decide` returns.
  - So the stop rule's memory depends on the data: nearly every row when many ratios tie.
  - At 100 members it adds 0.3–4.9 MB. The worst case at 10,000 members is about 3.2 GB (6.4 GB with `Vec` slack).
  - **It affects no published value.** It matters for W1's per-case memory limit and for admission.
- **Rulings:**
  - **E_max keeps the worst-case tracker term.** Admission uses the upper bound derived from the code, with ρ measured.
  - W1-T4 (10,000 members) runs only as ROOT approves, after W1-T3's prefixes at 1,000 members show how the kept count grows. The plan's W1-T4 row is superseded.
  - **Routed: a kernel follow-up (KF1) bounds the tracker without changing any result.** For example, collapse the kept entries to their exact extreme when their count passes a threshold. It needs its own review; K4's golden work counts would change.
    - It is paired with K6's N10 (bounding the dense witness) as the kernel follow-up slice.
    - **ROOT's W1 memory limit is not set until KF1 lands, or until the limit accounts for the worst case explicitly.**
- **Next:** A2: the runner's W1 tiers, `--plan`, `--smoke`, the approved counts-only runs at 1,000 and 10,000 members (no solve, 512 MiB cap) and the release projection. No W1 solve above 100 members before ROOT approves the schedule.

## K6b: A2 accepted; W1-T1 to W1-T3 approved (ROOT, 2026-09-29)

- **A2 is accepted.** ROOT committed it at `f4d40dd17` on the K6b branch. ROOT checked the four files' sha256 against I16's return and found no machine paths.
  - K6's four modes are frozen, and its 138 rows are unchanged, checked field by field.
  - The W1 tiers add 132 interleaved rows.
  - The runner suite passes 44 of 44, and the wrapper with the DEC-050/053 pins 46 of 46.
  - The counts-only runs cover every sealed model at up to 10,000 members, with a heap of at most 197 MB under the 512 MiB cap.
  - The smoke selected all 21 models at 128, with 0 parity failures.
- **The schedule is approved** (`plan.txt`, sha256 `091ee187…`), with B's binary built in release from a `git archive` of `f4d40dd17`:
  - **W1-T1 and W1-T2 in slot K6B-S1, and W1-T3 in slot K6B-S2,** with pass-1 prefixes at 1,000 members. One observation process at a time, the quiet-host wait, the load recorded per run, and the memory guard running.
  - **W1-T4 stays deferred.** ROOT rules on it from W1-T3's measured prefixes and tracker figures, and from KF1's status.
  - A watchdog kill, a heap-cap abort, a budget outcome or a parity failure on an admitted run is a stop.
- **The slot:** ROOT grants K6B-S1 and K6B-S2 now. I17 holds cargo and heavy Python until I16 reports B's runs done.
  - An unrelated long-running external process (one core, outside this repository) keeps the load near 5. It is recorded, not waited out. Timings are observations with their load.

## KF1: spawn (ROOT, 2026-09-29)

- **KF1 (the stop-rule tracker bound) is spawned as I18** (`TASK_BRIEFS/I18_KF1_IMPLEMENTATION.md`), on branch `codex/piping-kf1-20260929` in `<wt>/kf1`, from main `ab02ee3a6`. [Correction: the branch was created at main `8cca91701`, PR #1055, which changes only `projects/chirality-app-v4/**`. The piping tree is the same.]
- **Checkpoint 0 is a plan only.** Builds wait until K6b's timed slot ends.
- **K6's N10** (bounding the dense witness, and examining the dense screen's operation count) is split out as **KF2**. It is product-reaching, so its gate is heavier, and it is briefed separately. It does not block W1's limits.
- **Order for W1's memory limit:** KF1 merges. Then K6b merges main and measures W1-T4 (10,000 members) on the bounded tracker, subject to ROOT's W1-T4 ruling.

## K6b: B's W1-T1 to W1-T3 accepted; W1-T4 approved before KF1 (ROOT, 2026-09-29)

- **W1-T1 to W1-T3 are accepted.** I16 ran 108 processes, all ok, with no stop (records in `<wt>/scratch/i16/b/`, which move to `_run_records/` at D).
  - All 42 W1 processes selected 128 and verified at 256, with every parity item true.
  - R1's unchanged predicate passes on all 10,078 comparisons, decided exactly. The worst is 1.75e-6 of the allowance.
  - At 1,000 members: 0.56–1.58 s per call, 0.80–1.41 ns per limb-multiply equivalent, and 86–117 MiB of heap. W1 takes 6.8–14.2 times the binary64 sparse entry's time.
  - The stop rule costs 9–21% of the call. 5a.3's shift costs one shifted factorization, except on CONT-AX, which needs none.
  - Heap grows with a log-log slope of 0.96–0.99.
- **The tracker measured at 1,000 members** keeps 0–8,773 entries, at most 5.5% of its worst-case term. The kept fraction of rows does not grow with size (CONT 0.39, 0.43, 0.33 at 10, 100 and 1,000 members).
- **W1-T4 (10,000 members) is approved now, in its own slot K6B-S3,** as scheduled: 5 repeats, prefixes in pass 1. This supersedes the order in "KF1: spawn".
  - **Why before KF1:** it measures the unbounded tracker's actual size on RF-LARGE at 10,000 members, which KF1's review and ROOT's memory limit need.
  - **Why it is safe:** the heap cap (7.5 GiB) is below E_max (8.6–8.9 GiB), so a worst-case tracker would end in a heap-cap abort, not strain the host. The admission rule, with ρ measured at 100 members or more (0.17–0.27), admits every row.
  - A heap-cap abort here is recorded as a finding for KF1, not a stop of K6b.
  - Any other stop condition still stops.
- **After KF1 merges,** ROOT decides whether K6b re-measures a subset on the bounded tracker, for the stop rule's changed work, before the W1 limits are set.

## V-K: THIN confirmed by GEN; the expected-unresolved list stands (ROOT, 2026-09-29)

- **GEN confirms THIN.** K4's generator, run on THIN-A and THIN-B with its own adapter `r1_adapt` and G = E/(2(1 + ν)) stated exactly, gives K4's chain exactly:
  - 128 rejected by the stop rule at row 7;
  - 256 rejected by the stop rule at row 14;
  - 512 rejected by the charge at row 14;
  - Ceiling.
  - The record is `IMPLEMENTATION/VK/_run_records/a1/gen_thin_confirm.*`.
- **So THIN is W1a's design limit,** and the ruling "V-K: rulings on I17's A1 stop" takes effect. The expected-unresolved list is `VR/cases/expected_unresolved.json`: THIN-A and THIN-B, with log2 of the stiffness spread 507.67.
- **RF-RANGE** gives 2,590 passes, 80 structural zeros, 50 expected unresolved and 0 failures.
- **Also recorded from I17's A1 work:**
  - §5.3's parity: K is bitwise equal between the sparse and dense paths, and the outcome class is the same, on RF-LARGE at up to 100 members and on RF-MECH.
  - Q7's invariance observations: offsets are bit-identical, and relabelled variants differ only by the roundings that follow RCM's order.
- **I17 resumes** its debug suite and returns A1 when K6b's slot K6B-S3 ends.

## K6b: W1-T4 stopped by the binary's backstop; deferred until KF1 (ROOT, 2026-09-29)

- **What happened.** Row 247 (CHAIN-n10000-AX, `w1a`, pass 1) was admitted by the runner, on E_max × measured ρ ≈ 1.7 GB. The binary then refused it, `estimate_exceeds_half_cap`: K6's backstop (`main.rs:658`) compares the raw E_max, 9.23 GB, with half the heap cap, 3.75 GiB.
  - There was no solve: the heap peak was 3.9 MB. Nothing else in W1-T4 ran, and the memory guard logged no kill.
  - "The binary refused a run the runner admitted" is a runner stop, and I16 stopped correctly.
  - **The backstop worked as designed.** The miss, I16's own, was that A2 tested the runner's admission but not the backstop, for a mode whose ρ is far below 1.
- **Ruling: option (c). W1-T4 stays deferred until KF1 merges.**
  - The binary's backstop stays independent of the runner. Option (a), passing the runner's figure to the binary, would make the backstop trust the runner, which defeats it.
  - Option (b), dropping the tracker term for `w1a` only, is superseded by KF1: once KF1 bounds the tracker, E_max falls to about 2.6 GiB and passes the backstop unchanged.
  - The unbounded tracker's size at 10,000 members is no longer needed. W1-T3 measured it at 1,000 members (at most 5.5% of the worst case, with the kept fraction not growing with size), which is enough to motivate KF1. W1-T4 should measure the bounded code, which is what will ship and what the limits need.
- **Then:**
  - K6b's branch merges main after KF1.
  - K6b re-runs W1-T3 (about 3 minutes; KF1 changes the stop rule's work) and runs W1-T4, on a binary rebuilt from that commit, in slots ROOT grants.
  - Row 247's refused record is voided (renamed, with the reason), as K6 did.
- **K6b proceeds now to C (mutants),** one cargo job. D waits for W1-T4.
- **A new mutant for C:** the runner's admission is checked against the binary's backstop. A test asserts that, for every admitted row, the binary's own check admits it too, or the row is deferred by name. This closes I16's miss.

## K6b: C's mutation table; M10's test approved (ROOT, 2026-09-29)

- **C's runner change is accepted.** ROOT committed it on the K6b branch. `admission()` defers by name any row the binary's backstop would refuse, and defers a row whose counts line has no estimate for its mode.
  - K6's 138 rows and K6b's 270-row plan are unchanged. The runner suite passes 45 of 45.
- **The mutation table:** NONE passes, and 17 of 19 mutants are killed, M14 among them (the backstop omitted).
  - **M3e (Iy and Iz swapped)** is equivalent, and the equivalence is derived: Iy = Iz in every section used.
  - **M10 (the prefix limit b_j − 1)** survives. K4's budget tests fall before each segment's final work on these models, so the prefixes stop at the same place.
- **I16's assertion is approved:** each prefix limit equals its segment's exact end. Add it to `tests/k6b_w1.rs`, and re-run NONE and M10 from clean copies; M10 must be killed.
- **Next:** K6b waits for KF1. Then it merges main, re-runs W1-T3, runs W1-T4, and moves to D.

## KF1: rulings on I18's checkpoint-0 plan (ROOT, 2026-09-29)

The plan is `IMPLEMENTATION/KF1/PLAN_CHECKPOINT0.md` on the KF1 branch (sha256 `2129c337…`; committed by ROOT at `d0566126e`). **It is approved, with these rulings.**

- **The design is approved:** a `BoundedExtremeTracker` with at most T = 64 lazy rows and a pruned table of evaluated entries, collapsing when full.
  - The invariant argument is approved as the basis: bit-identical `finish` results, refusals included, for any collapse schedule.
  - The reviewer checks it independently.
- **Scope (I18's decision 1): KF1 bounds every `ExtremeTracker` site,** not only `rule`'s three: the pivot-margin (`adaptive.rs:999`), residual-gate (`:1236`) and fallback (`:1374`) trackers as well.
  - **Why:** W1's memory limit must rest on a bound that does not depend on the data. Leaving about 1.5 GB of data-dependent memory at 10,000 members would defeat it.
  - **The brief's item 3 is amended:** the work recorded in the stop rule, refinement and fallback stages may change. A tracker that today is dropped unevaluated is evaluated at a collapse, and that work is charged and budgeted where it is spent.
  - **Nothing else may change:** every published row, class, bound, attempt role and reason, and outcome under unlimited budgets.
  - Golden work counts are re-pinned where they move, each with its derivation.
  - The old `ExtremeTracker` is removed if no caller remains.
- **Many bodies (decision 2): include the shared cap across a call's trackers,** so a model with many small bodies is bounded too. State the resulting bound per call.
- **The budget boundary (decision 3)** is accepted and recorded. No W1 limits exist yet, and ROOT sets them from the measurements after KF1.
- **The test hook (decision 4)** is accepted only under `#[cfg(test)]`. A thread-local override of T must not exist in a non-test build.
- **After KF1 merges,** I16 recomputes K6b's E_max from KF1's code, for all trackers, before W1-T4.
- **Next:** checkpoint A. K6b's timed slot is over, so builds may run: one cargo job at `-j 4`.

## V-K: A1 accepted (ROOT, 2026-09-29)

- **A1 is accepted.** ROOT committed it at `37bff1780` on the V-K branch.
  - ROOT checked the key files' sha256, that both `SHA256SUMS` files verify (the cases and the kernel-lane observations), that no machine paths appear, and that there is no FK change.
  - 40 tests pass. Every one of the 201 CI cases passes, apart from THIN's two expected-unresolved cases; the `not_covered` set equals the committed list; all 506 discriminating controls fail.
  - §5.3's parity: K is bitwise equal between the sparse and dense paths, with matching classes, at up to 100 members.
  - VR's measured CI cost is a 3.8 s build and 37 s of tests on the Mac.
- **RF-MECH-DISC-CHAIN100 and -SPRING (103 members) take about 300 s each in dense mode.** This is K6's N10 dense-witness cost (KF2), not a new finding. They run in the `vk_records` example, not in CI, and CI's parity stays at 100 members or fewer.
- **Next:** A2: the seeded faults, the FK `mutation-controls` feature (VK-F05 included) and the kill matrix.
  - **KF1 is changing `ExtremeTracker` and its call sites in `K4R/adaptive.rs` now.** Keep V-K's fault sites off the tracker code and its callers' tracker lines, so the later merge of main is mechanical.
  - If KF1 merges first, V-K merges main and re-runs its fault matrix.

## KF1: checkpoint A accepted; the S11 site-table row authorized (ROOT, 2026-09-29)

- **A is accepted, pending one declared edit.**
  - `BoundedExtremeTracker` uses T = 64, and a shared `TrackerSet` caps a call at G = 512 unevaluated rows. All seven sites are converted, and the old tracker is removed.
  - **The differential test** against a verbatim copy of K4's tracker runs 828 streams at T = 1, 2, 3, 5 and 64, in both directions. `finish` is bit-identical, including refusals.
  - **The model-level differential** runs 131 controls, 4 combinations and the six 100-member frames at T = ∞, 1 and 64. Everything is identical except the width-16 work.
  - **Golden work** does not move. A new pin covers the collapsing frames.
  - All 10 mutants are killed. `gen_k4_vectors.py --check` passes 23 of 23.
  - **The bound:** about 2.3 MB of unevaluated rows per call in the stop rule, 275 KB for the pivot margin, 275 KB per residual-gate evaluation, and 1.1 MB for the fallback, against about 3.2 GB today at 10,000 members.
- **The site table: authorized.** Add one declared, additive row to `FK/tests/s11_site_table.rs`, for `adaptive.rs` `offer`: 2 integer accumulations, the row count and the held capacity. It is in KF1's write set by this ruling, as K4's rows were (Q8). No existing row changes.
- **The work findings are accepted:**
  - At T = 64 only the six 100-member frames move, and only in the stop rule: +2.2 to +13.4 M LME, at most about +15% of the stop rule's work (about 2% of the call). [Correction (I18 at D): ROOT misread I18's figure. The measured change is +21% to +159% of the stop rule's own work, and +2.5% to +16.2% of the case's total work (KF1 `RETURN.md` §5). The choice of T is reopened below.]
  - The remaining extra work comes from collapsed rows that a later best drops. It is bounded by one exact evaluation per row, whatever T is.
  - Cutting it further would need the approximation lemma in the correctness basis. That is declined: the accuracy-free argument stands.
  - T = 64 is kept.
- **For K6b's E_max:** the fallback's per-state row list (about n_f × 4.3 KB) is not a tracker. It is proportional to the model, not the data, and I16 includes it.
- **Next:** add the row, re-run the site-table tests and FK's full suite, then D: RETURN, CHANGE_RECORD, `_run_records/` and SHA256SUMS.

## KF1: D received; T reopened and set to 512 (ROOT, 2026-09-29)

- **D is received and committed,** on the KF1 branch: A at `68db15d41` and D at `d267a755b`. FK's full suite passes at 401; the site table gains the one declared row.
- **T is reopened on the corrected figures** (see the bracketed correction in "KF1: checkpoint A accepted"). At T = 64 the six 100-member frames gain +2.5% to +16.2% of their case work. I18's probe shows 0 extra at T = 512.
  - T never affects correctness: the invariant holds for any collapse schedule.
  - T = 512 with G = 8T = 4096 bounds a call's unevaluated rows at about 17.6 MB, plus the fallback's 4 × 2.2 MB and the tables. That is still data-independent and small next to W1's measured heap (about 100 MB at 1,000 members).
  - Memory is only used when the data keeps that many rows; the bound only caps it.
- **Ruling: T = 512 and G = 8T.**
  - The model-level differential runs at T = ∞, 1 and 512.
  - The T = 64 collapse pin is kept through the test hook, so collapse work stays exercised at model level.
  - Re-run the golden pins, NONE and the T-sensitive mutants (M7 no collapse, M8 shared cap ignored).
  - Record it in RETURN and CHANGE_RECORD as addendum 1, with the work table at T = 512.
- **The approximation lemma stays out of the correctness basis.**

## V-K: A2 accepted (ROOT, 2026-09-29)

- **A2 is accepted.** ROOT committed it at `c1fea8574` on the V-K branch.
  - **The FK diff is insertions only** (186 lines). Every fault site sits behind `#[cfg(any(test, feature = "mutation-controls"))]`, and the feature is declared in FK's `Cargo.toml` with no dependency change.
  - **The kill matrix** (`VR/observations/seeded/`, `SHA256SUMS` verifying): NONE passes 40 of 40, and all 15 faults are killed.
    - VK-F05 and VK-F06 are killed on THIN's attempts and outcome; VK-F07 and VK-R28 on evidence, as ruled.
    - VK-R28 raises CHAIN-n00100-AX to 512 and TREE-n00100-AX to 256, as R7 §7 predicts.
  - The §7.3 items R1 cannot discriminate are cited to K4's killing mutants: 3 (D3), the relabelling half of 10 (D10), and 16 (D16a and D16b).
  - FK's suite with the variable unset passes 394; the feature guard passes; K4's source scan and the S11 site table pass.
- **Next for V-K:** C, the harness mutants. B (the scale runs at 1,000 and 10,000 members) waits for KF1's merge: V-K merges main, re-runs the kill matrix, then runs B in one slot ROOT grants.

## KF1: to review (ROOT, 2026-09-29)

- **Addendum 1 (T = 512) is accepted,** committed at `1854911d1`.
  - No control or 100-member frame gains any work.
  - The stream differential is extended to 882 runs.
  - FK's full suite passes 401. NONE passes, and KF1-M7 and KF1-M8 are killed.
- **The PR:** [#1056](https://github.com/sgttomas/chirality/pull/1056). The full-SHA dispatch is 36621651732 (target_base `8cca91701`). GEN-8 passes on `1854911d1`.
- **The independent reviewer is RV20,** directed first to result equality at every site and collapse schedule, then to the memory bound, the work and the tests.
- **After merge:**
  - K6b merges main and recomputes E_max from KF1's code, then re-runs W1-T3 and runs W1-T4.
  - V-K merges main, re-runs its kill matrix, and runs B.

## V-K: C's harness mutants; three tests added (ROOT, 2026-09-29)

- **C ran 18 harness mutants and 2 controls** (NONE, and NONE-GEN, whose regeneration is byte-identical). 15 are killed and 3 survive:
  - **VK-H6:** max(|obs|, scale) in place of max(|exp|, scale);
  - **VK-H9:** an absolute-range pass counted as a pass;
  - **VK-H11:** a passing discriminating control dropped.
- **None of the three is a harness defect.** R1's committed CI data never reaches those paths.
  - Two tests that plan §14.1 promised were not delivered at A1: the absolute-range path on R1's five rows, and the controls test.
  - A1's unit test for H6 does not discriminate what its comment claims.
- **Ruling: option (a).** Add `VR/tests/engine.rs` with I17's three constructed-input tests, as drafted in `_run_records/c/proposed_tests_engine.rs.txt`:
  - obs = 1 with exp on either side of it;
  - R1's five sub-range rows judged as `PassAbsoluteRange`;
  - a passing discriminating control listed as undiscriminated.
  - Re-run only NONE, VK-H6, VK-H9 and VK-H11 from clean copies with the tests added. Adding tests cannot un-kill the other 15, so their results stand, recorded as from the first run.
  - Commit the matrix as `VR/observations/harness/harness_matrix.jsonl`.
- **The "data-equivalent" harness paths** (for example a failing comparison recorded as a pass) are evidenced by A2's kill matrix, whose faults show every comparison kind is live. That argument is recorded, not run as mutants.
- **Delete `<wt>/vk-mut/c`** (2.4 GB). The records keep every hash.
- **B still waits for KF1's merge.**

## V-K: C accepted (ROOT, 2026-09-29)

- **C is accepted.** ROOT committed it at `e24e911e6` on the V-K branch.
  - All 18 harness mutants are killed, and NONE (43 of 43) and NONE-GEN pass.
  - VK-H6, H9 and H11 are each killed by exactly its own new test in `VR/tests/engine.rs`.
  - `observations/harness/SHA256SUMS` verifies. No machine paths appear, and the mutant copies are deleted.
- **Next for V-K:** start D's records that do not depend on B. When KF1 merges: merge main, re-run the A2 kill matrix, run B in a slot ROOT grants, then finish D.

## KF1: rulings on RV20's review (ROOT, 2026-09-29)

RV20 (`REVIEW/KF1_REVIEW.md`, sha256 `d1cde558…`; records `REVIEW/_run_records/kf1_review/`) reviewed head `1854911d1`: **PASS**, with 0 BLOCKING, 1 SHOULD-FIX and 5 NOTEs.
- **Equality:** RV20 checked RETURN §3's proof step by step and found no difference over 36,000 streams, including probes with keys unrelated to values and arbitrary collapse schedules.
- The shared cap holds over 400 rounds. A key-only replay reproduces the work pins independently. FK's full suite passes 401.

**Fix before merge, as with earlier slices' SHOULD-FIX findings:**
- **RV20-1:** in the shared-cap test, assert per round that the bounded run's ctx16 work is at least the reference run's. This kills RV20-M4, a shared-cap collapse charged to a throwaway context.
- **N1:** add RV20's order test, which kills M5 (`RuleTest` reordered). M1 and M2 are recorded as equivalent: every reachable refusal is `Span`. [Correction (RV20, C-N1): the evidence is narrower. Every refusal constructed so far is `Span`, and it is not proven that no other stop is reachable. M1 and M2 are equivalent on every input constructed. The equality proof does not depend on this.]
- **N3:** restate the memory figures in RETURN.
  - The transient peak is G + T = 4,608 rows (19.8 MB), because `Vec` growth briefly holds both buffers; a standalone tracker peaks at 1.5T.
  - The tables' unconditional bound is at most 40 B per row kept in the window; the "one value entry" figure is practical only.
- **N5:** RETURN §0 and the head of CHANGE_RECORD point to addendum 1 for the shipped values: T = 512, G = 4096.
- **N2 and N4 are recorded.** M12 (fallback collapse work uncharged) is the disclosed limit; the code charges it correctly by reading. "No control gains work at T = 512" is printed, not asserted, and K6b re-measures at W1's sizes.

**Then:** CI and the dispatch on the new head, DEC-025 on it (the run under way on `1854911d1` is kept as the earlier head's record), GEN-8, a delta confirmation by RV20, and the merge.
- **RV20 confirms the final head `66adfede4`: PASS,** with no findings apart from one NOTE (C-N1, corrected above in brackets).
  - The change touches no `src/` file.
  - RV20-M4 and RV20-M5 are killed from clean archives.
  - N3's figures check.
  - SHA256SUMS verifies 42 of 42.

## KF1 merged (ROOT, 2026-09-29)

- **Merged:** [PR1056](https://github.com/sgttomas/chirality/pull/1056) at head `66adfede4`, merge `0f5d8c7b4`, 2026-09-29 21:34:25Z. The merge record is `IMPLEMENTATION/KF1_MERGE/RECORD.md`.
- **The gates:**
  - RV20 PASSED the review and confirmed the final head;
  - hosted CI was green, and the dispatch (36628173972) succeeded;
  - DEC-025 was clean on the final head: only frame_kernel changes, 394 → 402;
  - GEN-8 passed.
- **Next:**
  - **K6b (I16):** merge main, recompute E_max from KF1's code, then re-run W1-T3 and run W1-T4 in slot K6B-S3. I17 holds cargo during the slot.
  - **V-K (I17):** merge main, re-run the A2 kill matrix, then B in its own slot after K6b's.
  - **Then** ROOT's W1 limits from K6, K6b, V-K and K4's work counts, and F2a.

## K6b: main merged; E_max from KF1; slot K6B-S3 approved (ROOT, 2026-09-29)

- **ROOT merged main `0f5d8c7b4`** (KF1) into the K6b branch as `b86081221`, and into the V-K branch as `485320e95`. There were no conflicts.
  - [Correction: ROOT's first message asked both implementers to prepare the merge themselves. That was withdrawn before any merge ran, because a merge writes the index, which TASKs may not do. I16 disclosed a `git fetch`, which updates only remote-tracking refs.]
- **E_max now follows KF1's bounds at every site.** ROOT committed it as `082990c8d`.
  - It includes the pivot-margin, residual-gate and fallback sites, which were missing before KF1.
  - At 10,000 members E_max is 2.78–2.86 GB, down from 9.2–9.5, and the binary's backstop admits all six W1-T4 models.
  - H passes 70 of 70, the runner 45 of 45, and the wrapper with the pins 47 of 47.
- **Slot K6B-S3 is approved,** with a fresh records folder:
  - W1-T1 to W1-T3 re-run (about 2.5 min), which gives ρ against the new E and the ascent;
  - then W1-T4 (about 15–17 min).
  - The binary is a release build from a `git archive` of `082990c8d`.
  - B's pre-KF1 records stay as they are, as the before-KF1 comparison.
- **The slot starts when I17 finishes V-K's post-merge checks.** I16 may build the release binary now. A heap-cap abort or any other stop condition stops the tier.

## K6b: slot K6B-S3 stopped at W1-T4's first row; W1 at 10,000 members ends in Span (ROOT, 2026-09-29)

- **What ran.**
  - **W1-T1 to W1-T3, re-run on the post-KF1 binary** (sha256 `4f55137b…`, from `082990c8d`, into fresh records): 108 processes, all ok, with every parity item true and every `w1a` row selected at 128.
    - ρ_fp against the new E: 0.10–0.12 at 10 members, 0.26–0.38 at 100, and 0.38–0.40 at 1,000; the DEC-053 nine reach up to 0.48.
    - The stop rule adds no measurable heap at 1,000 members. KF1's bounded tracker fits under the verification's earlier peak.
  - **W1-T4, row 247 (CHAIN-n10000-AX, `w1a`),** is deterministic over 5 repeats:
    - the 128 candidate is rejected by its verification;
    - the 256 verification's shared build stops with `Span`, so the outcome is `Unresolved(ExactSumSpan)`;
    - 6.84e9 LME are charged, the heap peak is 832 MiB, and the call takes about 5 s.
    - K6b's parity item `w1_stages_equal_totals` then failed, and the runner stopped the tier correctly.
- **(a) The parity item: K6b fixes its own check,** in H only, as I16 proposes:
  - the stage identity is checked on completed builds only;
  - on a failed build, the check is that the stage sum is at most the charged total, and the unstaged remainder goes on the attempt line;
  - the outcome's per-precision shared work uses the charged totals;
  - a test reaches the failed-build path at a small size, and a mutant is killed.
  - **The FK side is routed to KF3.** The verification's shared build (`verify.rs:471-485`) records no stage for the partial `uc` work when `gamma_m` or `uc_bounds` stops. The charged total is right: `verify_precision` charges all of it. So the evidence's stage breakdown under-reports on this error path, and F2a will publish that evidence.
- **(b) The availability finding is routed to KF3,** after the data below.
  - **ROOT's reading of the code** (`bound.rs:405-420`): `uc_bounds` runs directed recurrences (`u_pass`, `nl_pass`) over the factor's profile. On a long chain the comparison-matrix bound grows geometrically, so its exact sums exceed `ExactWideSum`'s span, and the whole attempt stops.
  - **But B = min(Uc, S) per block** (R7, Lemmas D and E), and S is an independent certified bound. Treating a Uc that cannot be formed as +∞ keeps every step certified: B = S. The attempt stops only if neither bound is available.
  - **Honesty is unaffected:** no published value changes, only whether W1 can publish.
  - **KF3 will:**
    - treat a Uc (and likewise an S) that cannot be formed as unavailable, not as an attempt stop, with R7's text checked for any other reader of Uc;
    - record the partial stage work on every error path.
  - KF3 is briefed after W1-T4's re-run shows which of the six 10,000-member models stop, and where.
- **(c) W1-T4:**
  - row 247's record is kept as the stop's evidence, not voided;
  - after (a), ROOT commits the fix and grants a slot in which all 24 W1-T4 rows run on the rebuilt binary, in a new records folder (about 10–15 minutes);
  - after KF3 merges, W1-T4 runs once more; those are the figures ROOT's W1 limits use.
- **D waits** for the final W1-T4.

## V-K: B prepared; ExactSumSpan recorded, not a stop (ROOT, 2026-09-29)

- **B's code is committed** at `64470c6ba`: `VR/src/scale.rs`, `examples/vk_scale.rs` and `runner/vk_scale_runner.py`.
  - The small-size check passes: the records `vk_scale` emits at 10 and 100 members are byte-identical to the committed ones, and VR passes 44 of 44.
  - The admission estimate is a cited copy of K6b's E_max on KF1's trackers, because VR cannot depend on the harness. Deduplicating it once K6b merges is recorded as a follow-up.
- **The large model files** are generated with `gen_vk_cases.py --large` and checked against `large_models.sha256`. B's binary is a release build from a `git archive` of `64470c6ba`.
- **ExactSumSpan in B:**
  - an `Unresolved(ExactSumSpan)` outcome at 10,000 members is the KF3 availability finding;
  - it is recorded with its attempts and work, and its rows are reported as a separate `unresolved_availability` count, never as passes;
  - the tier continues;
  - it is a named, narrow runner exception, with a test;
  - any other unresolved reason, a covered-row failure or any other stop condition still stops;
  - V3's final figures re-run after KF3.
- **Order of slots:** K6b's W1-T4 re-run first (short), then V-K's B.

## V-K B accepted; KF3 spawned (ROOT, 2026-09-29)

- **V-K's B is accepted.** ROOT committed its records at `f5379a5d4` on the V-K branch.
  - **V1 (100 members) and V2 (1,000):** 12 of 12 selected at 128, every row passes, and C9's floor sets are empty.
  - **V3 (10,000):** CONT-n10000-AX is selected at 128, with 211 passes and 4 absolute-range passes out of 215. The other five end `Unresolved(ExactSumSpan)` in the 256 verification's shared build, before `uc`, recorded under the named exception. [Correction (ROOT, 2026-09-30, RV25-N4): "before `uc`" should read "before the Uc bounds complete". The shared build recorded no `uc` stage for the partial work, and KF3's diagnosis places the stop inside `uc_bounds`.]
  - No watchdog kill, heap-cap abort or memory-guard kill occurred.
  - W1 at 10,000 members takes 5–10 s per call and about 0.82 GB of heap, against E_max of 2.7 GB (ρ 0.32–0.37). [Correction (ROOT, 2026-09-30, RV25-S2): these GB figures are MiB divided by 1,000, the E_adm slip again. VK RETURN §14.3 gives a heap of 816.5–835.0 MiB (0.86–0.88 GB) and E_max 2,676–2,752 MiB (2.81–2.89 GB). ρ is unaffected.]
- **KF3 is spawned as I19** (`TASK_BRIEFS/I19_KF3_IMPLEMENTATION.md`), on branch `codex/piping-kf3-20260929` in `<wt>/kf3`, from main `0f5d8c7b4`. It covers:
  - the diagnosis of the Span;
  - D1 revision 5a.3 **amendment A2**, which ROOT records when KF3's plan confirms the reading: a certified bound (Uc or S) that cannot be formed is +∞, so B = min over the available bounds, and the attempt stops only if a block needs a bound and none is available;
  - partial stage work recorded on every error path.
  - **Checkpoint 0 is diagnosis and plan only.** Builds wait until K6b's current slot ends.
- **After KF3 merges:** K6b and V-K merge main and re-run their 10,000-member tiers. Those figures, with K6, K4's counts and the rest of K6b's and V-K's measurements, are the basis for ROOT's W1 limits.

## K6b: slot K6B-S4 (b3) accepted; D now, with W1-T4 pre-KF3; V-K's PR to review (ROOT, 2026-09-29)

- **b3 is accepted.** It ran 132 processes, all ok, with 0 parity failures and 0 deferrals, on the binary `20b67776…` built from `4eeb206c0`.
  - **K6b's six 10,000-member rows agree with V-K's B.** CONT-AX is selected at 128; the other five end `Unresolved(ExactSumSpan)` in the 256 verification's `uc` stage, after bounded and wide formation. The fixed parity check passes on all of them.
  - **CONT-AX at 10,000 members:**
    - the call charges 8.23e9 LME, with a heap of 814 MiB and a median call of 6.09 s, 7.6 times the binary64 sparse entry;
    - its stop rule is 2.60e9 LME, 31.5% of the call.
  - W1's heap grows with a log-log slope of 0.92–0.98 from 10 to 10,000 members.
  - ρ_fp is at most 0.43 against the new E, and E_adm at 10,000 members is 2.65–2.73 GiB. [Correction (I16 at D): E_adm at 10,000 members is 2,649–2,727 MiB, which is 2.59–2.66 GiB (2.78–2.86 GB). The b3 report had divided MiB by 1,000.]
  - **A K6 observation, not a stop:** K6's own sparse estimate gives ρ_fp up to 1.75 at 10,000 members (CHAIN-AX). The excess is outside the heap: heap/E is 0.64–0.73.
- **D proceeds now, as for V-K.** K6b's records mark W1-T4's 10,000-member figures pre-KF3.
  - After KF3 merges, K6b merges main and re-runs W1-T4 as a records addendum, which may follow K6b's merge.
  - ROOT's W1 limits use the post-KF3 figures.
- **V-K's PR** [#1057](https://github.com/sgttomas/chirality/pull/1057) is open. The dispatch is 36640221444. The independent reviewer is RV21.

## KF3: rulings on I19's diagnosis and plan; D1 revision 5a.3 amendment A2 (ROOT, 2026-09-29)

The plan is `IMPLEMENTATION/KF3/PLAN_CHECKPOINT0.md` on the KF3 branch (sha256 `721fc2d6…`). **It is approved.**

- **The diagnosis is accepted.** The five frames stop in `build_verify_shared`, then `uc_bounds`, then `u_pass`: an `add_toward` whose addends lie more than 8,128 bits apart.
  - **U_c grows geometrically along the chain:** log2 U_c ≈ α·N + β, with α from 1.74 to 6.6 bits per member on the five frames. The true norm grows only polynomially.
  - The first refusal is predicted between about 1,200 and 4,500 members.
  - It is not an exponent overflow, and neither `gamma_m`, `nl_pass` nor `bounds_from` is involved.
- **D1 revision 5a.3, amendment A2 (ROOT's ruling):**
  - a Uc_c or S_c whose formation is refused, by `Span` or `Exponent`, is unavailable. It is treated as R7's existing "does not exist" (+∞);
  - B_c is the minimum over the bounds that were formed;
  - `u_pass` and `nl_pass` mark only the refused block and skip its remaining rows, and every other block runs as today;
  - **honesty:** every step of the guarantee (Lemmas A to C, the Theorem and the Corollary) uses B_c only through B_c ≥ ‖K̃_c⁻¹‖₁. A2's B_c is the minimum of a nonempty set of formed bounds, each certified by Lemma D or E, and a refusal changes no formed value. So A2 affects availability only. The reviewer checks this against every reader in plan §3.
  - `DESIGN.md` stays hash-pinned; the amendment is this ruling.
- **The decisions in plan §11:**
  1. **A block with data and no bound, after a refusal:** the attempt stops with that refusal, as today.
  2. **A refusal stop takes precedence** over another block's `uc` rejection. The outcome is deterministic and conservative.
  3. **The refusal evidence** goes on `AttemptRecord` (`bound_refusals`) only.
  4. **The constructed slender chain** is approved as the CI model (a planar chain along (3,4,0), under about 200 members). Report its debug CI time, and ask if it exceeds about 60 s.
  5. **The test-only hook that removes S** for the W2 test: reuse V-K's `seeded` module if V-K is on main first; otherwise add a `#[cfg(test)]` hook in KF3.
  6. **The scale driver for B:** main's drivers if V-K and K6b have merged first. Otherwise, their `vk_scale` and `k6_observe` built from `git archive` copies of their branches, which is read-only.
  7. **`wide_sum.rs`: one method, authorized,** added to KF3's write set: a full accumulator reset after a refusal, as a safeguard, with a unit test.
  8. **No early exit or "sticky" directed add.** Token-level identity on RF-LARGE-100 is kept.
  9. **est_c stays out of A2.**
- **Partial stages:** all four builds (`build_shared`, `solve_case_at`, `build_verify_shared` and `verify_state`) add the charged work their stages do not record to the stage in progress, so the stages sum to the charged total on every path.
- **Merge order with K6b** (whose test asserts `shared_stages.uc == 0` on a stopped build, and whose parity check tolerates unstaged work):
  - **whichever of KF3 and K6b merges second** updates that test and tightens the parity check back to equality, in its own PR;
  - if KF3 is second, `performance_harness/tests/k6b_w1.rs` and `src/k6/w1/staged.rs` join KF3's write set for that purpose only.
- **Next:** checkpoint A. Builds may run, with one cargo job at `-j 4`; RV21 is reviewing V-K alongside.

## K6b: D accepted; PR to review (ROOT, 2026-09-29)

- **D is committed** on the K6b branch: `126fcb9f3` (the packet code and b3's packet) and `1123d19b9` (RETURN, CHANGE_RECORD and 1,774 run-record files, 21 MB, with oversized dumps trimmed and their sha256 kept).
  - Both `SHA256SUMS` verify, and no machine paths appear.
  - W1-T4 is marked pre-KF3 throughout, and the post-KF3 re-run is RETURN addendum 1.
- **I16's corrections and notes are recorded:**
  - E_adm at 10,000 members (bracketed above);
  - KF1 moves the stop rule's cost from memory to work: at 1,000 members the heap increment is gone, and the work rises 1.2–2.4×;
  - the 141 B constant in the sparse cross-check (N-4) is a note.
- **The PR** is open, with its full-SHA dispatch. The independent reviewer is RV22.

## V-K: rulings on RV21's review (ROOT, 2026-09-29)

RV21 (`REVIEW/VK_REVIEW.md`, sha256 `eab89fb5…`; records `REVIEW/_run_records/vk_review/`) reviewed head `3fd1baff3`: **PASS**, with 0 BLOCKING, 2 SHOULD-FIX and 5 NOTEs.
- **No way was found for the harness to pass a wrong answer on a covered row:**
  - 225,405 engine vectors against RV21's own `Fraction` oracle;
  - 129,968 wrong answers through the harness's own path, every one failed, with the exact boundary pinned to the ulp on 16,778 rows;
  - nine whole-case probes.
- **Also confirmed by RV21:**
  - its independent adapter matches every committed model;
  - the floor lists re-derive as 46, 3 and 2;
  - FK is unchanged with the feature off, and FK's suite passes 402;
  - A0 is visibility only;
  - B's exception is narrow;
  - every `SHA256SUMS` verifies.

**Fix before merge:**
- **RV21-1:** the feature guard also flags a manifest that enables VR's `seeded-faults`, which enables FK's `mutation-controls` indirectly. Use RV21's one-line fix and its self-test.
- **RV21-2:** add RV21's two drafted tests, a wrong observation of an out-of-range row fails, and an `Overflow` row fails. They kill RV21's mutants H4 and H6.
- **N1:** RETURN and CHANGE_RECORD are brought to the final head's counts.

**Recorded, optional:**
- **N2:** the generator's subprocess `--model` check (RV21 ran it: 191 runs).
- **N3:** a CI check that `not_covered.json` equals the committed set.
- **N4 and N5.**

**Then:** CI and the dispatch on the new head, DEC-025 (which gains VR's manifest), GEN-8, RV21's confirmation, and the merge.
- **RV21 confirms V-K's final head `5f0d39426`: PASS, with no new finding.**
  - The change touches no `src/` file in FK or VR.
  - RV21-1 is as drafted.
  - RV21-H4 and RV21-H6 are each killed by exactly their own test, and NONE passes 47 of 47.
  - N3's set check catches a planted duplicate.
  - The records match the head, and every `SHA256SUMS` verifies.
  - GEN-8 passes on `5f0d39426`.

## K6b: rulings on RV22's review (ROOT, 2026-09-29)

RV22 (`REVIEW/K6B_REVIEW.md`; records `REVIEW/_run_records/k6b_review/`) reviewed head `1123d19b9`: **PASS**, with 0 BLOCKING, 3 SHOULD-FIX and 7 NOTEs.
- **No product byte changes.** A0's patch-id equals both A0 commits.
- **The W1 mode:** RV22's K4SRC decode matches R1's models; its release `w1a` runs reproduce b3 byte for byte; the work closure has 0 discrepancies over 330 outcomes.
- **The records:** 934 RETURN values trace to the raw JSONL with 0 mismatches; the packet regenerates; the merge `b86081221` is clean.

**Fix all three SHOULD-FIX findings before merge.** ROOT's W1 limits will rest on E_max and on the stage evidence.
- **RV22-1:** tighten `stages_equal_totals` on stopped builds to at most one side short (`own == own_total || shared == shared_total`), each short side at most its total. Report `stages_complete` as unstaged == (0, 0). Add a test with RV22's probe, the 1-LME under-record on a stop-rule stop, and kill it.
- **RV22-2:** E_max must be an upper bound on every phase.
  - Add the 1024 verification's live vectors (about 3n + 8n_f wide values, and `recover`'s output) and the shift's profile clone (`bound.rs:560`) to the verification phase.
  - Add the solve-phase fallback items (`abar_q`, `evaluated`, `rhs`, `u_free`, the per-state u) under the move model.
  - Regenerate `counts.jsonl`, and state the change in RETURN: +5.4% on CHAIN and TREE and +2.1% on CONT at 10,000 members, and 0 at 1,000 or fewer. b3's admissions are unaffected. [Correction (I16): the change at 1,000 members is +3.28% (CHAIN, TREE) and +2.33% (CONT), because under the move model the fallback's items outgrow the tracker term there; at 10 and 100 members and on the nine it is +0.04–0.22%. b3's admissions are still unaffected: 0 decisions change.]
- **RV22-3:** add a test that recomputes every committed `counts.jsonl` line's E_max and E_sel128 from the code, and one that exercises the solve-phase terms at a size where they bind. Kill RV22-M6.

**The NOTEs are recorded:** RETURN's load range, the move-model heap/E column, the adapter through K6's section formula, and the two further unstaged paths, which go to KF3. RV22-M5 (the binary's prefix parity always true) and RV22-M7 (the backstop against the RSS cap) are killed if cheap, and otherwise recorded; RV22-M2 is equivalent on single-case models.

**Then:** CI and the dispatch on the new head, DEC-025, GEN-8, RV22's confirmation, and the merge.

## KF3: checkpoint A accepted (ROOT, 2026-09-29)

- **A is accepted.** ROOT committed it on the KF3 branch: `29c0b69e4` (the code and tests) and `a7ec4981a` (the checkpoint record).
  - **Amendment A2 is implemented as ruled:**
    - a refused bound marks only its block;
    - B = min over the formed bounds;
    - a block with data and no bound stops with the refusal, which outranks a `uc` rejection;
    - budget stops are never refusals;
    - refusals are recorded per block.
  - **Partial stage work** is staged on every path of all four builds.
  - **No control changes** outcome, row, class, bound or golden work; only the new `kf3.txt` controls move.
  - FK's full suite passes, `gen --check` is byte-identical on every earlier file, and 11 mutants are killed.
- **The constructed CI model is 390 members,** not under about 200. I19 found that slender sections shift log2 U_c by a constant and leave its growth rate unchanged. KF3-UC-SPAN, a chain along (−1, 12, −12) growing about 20.8 bits per member, is the smallest that refuses.
  - On main it ends `Unresolved(ExactSumSpan)`. Under KF3 it is selected at 128 with B = S_c, and is honest against GEN on 8,983 checks, the worst at 0.969 of its allowance, with G5a passing.
  - Its debug CI time is 38–64 s for the KF3 tests, which is accepted.
- **B (the scale evidence) waits for V-K's merge,** which carries `retained_api` and `vk_scale` to main. ROOT then merges main into KF3, and I19 runs B with `vk_scale` built from a `git archive` of that merged head, in a slot ROOT grants.
- **RV22's findings are closed** on the K6b branch.
  - **RV22-1:** at most one short side on a stopped build.
    - **I16's extension is accepted:** a stopped candidate that charged stop-rule work is held to equality. The decision runs only after its builds complete, and only that rule catches RV22's probe.
  - **RV22-2:** E_max bounds every modelled phase. At 10,000 members it is 2.64–2.81 GiB, and b3's admissions are unchanged.
  - **RV22-3:** tests recompute every committed line's estimate.
  - Of the new mutants, 10 of 11 are killed. RV22-M5B, the binary's call site forced true, survives and is recorded: no binary run can produce a false prefix, and the library form (M5L) is killed.
- **Then:** CI and the dispatch on the new head, DEC-025 after V-K's, GEN-8, RV22's confirmation, and the merge.

## V-K merged (ROOT, 2026-09-30)

- **Merged:** [PR1057](https://github.com/sgttomas/chirality/pull/1057) at head `5f0d39426`, merge `f8400d290`, 2026-09-30 00:26:51Z. The merge record is `IMPLEMENTATION/VK_MERGE/RECORD.md`.
- **The gates:**
  - RV21 PASSED the review and confirmed the final head;
  - hosted CI was green, and the dispatch (36646861753) succeeded;
  - DEC-025 was clean: the only change is the new `numerical_robustness` (47 tests), and frame_kernel is unchanged at 402;
  - GEN-8 passed.
- **FK's `retained_api` export is now on main.** K6b's identical A0 merges cleanly.
- **Next:**
  - ROOT merges main into KF3 and K6b;
  - KF3's B (the scale evidence) runs with main's `vk_scale`;
  - K6b's gates continue.

## Main merged into K6b and KF3 (ROOT, 2026-09-30)

- **K6b:** ROOT merged main `f8400d290` (V-K) as `597c81ba4`.
  - The conflicts in `retained/adaptive.rs` and `verify.rs` were resolved to main's versions. K6b's FK change was exactly A0 (patch-id `b43efeb4`), and A0 is identical on main, so main's FK (A0 plus V-K's gated sites) is the merged result.
  - K6b's FK now equals main's. The full-SHA dispatch 36650532005 runs on the new head.
- **KF3:** ROOT started the merge of main `f8400d290`. There is one conflict hunk, in `AttemptRecord`: A0 made `verification_shared_work` and `verification_shared_built_here` public, and KF3 added `bound_refusals` beside them.
  - **Resolution rule:** under A0's rule (the fields of evidence records are public; "V-K: rulings on I17's checkpoint-0 plan" Q8), `bound_refusals` is `pub`. `BlockRefusal`, and any type it carries, becomes `pub` and is added to the `retained_api` facade.
  - I19 resolves the conflict markers by editing only; ROOT stages and commits the merge.
- **KF3's write set gains, for the merge only:**
  - `FK/src/structural.rs`'s facade lines, for `BlockRefusal`;
  - **VR's committed records** (`VR/observations/**`), if KF3's partial-stage recording or `bound_refusals` changes them. They are regenerated with VR's own tools, only the expected fields may change, and VR's suite and kill matrix must pass. V-K's tests compare its records byte for byte, and V-K is now on main.
- **K6b's `uc == 0` test and parity check** follow the earlier rule: whichever of KF3 and K6b merges second updates them.
- **RV22 confirms K6b's `011911e4e`: PASS.** RV22-1, RV22-2 and RV22-3 are closed.
  - RV22's independent itemization is at or below E_max on all 33 lines, and 0 of b3's 132 admissions change.
  - RV22-M5B is acceptable as recorded.
  - **Two new NOTEs, recorded:**
    - C-N1: a stop inside the solve (`into_solve_128`) has no test, so RV22-C4 survives;
    - C-N2: on a stopped verification build, the two builds' summed stages can hide one build's shortfall. This is within the ruling, and KF3's equality closes it.
  - Because K6b's head is now ROOT's merge `597c81ba4`, RV22 checks that the merge adds exactly main's delta and that H passes on the merged tree.
- **RV22's merge check of `597c81ba4`: PASS, with no findings.**
  - Path by path over 59,471 paths, there are 0 mismatches. The first-parent diff is main's delta less A0, which both sides carry.
  - FK equals main's byte for byte.
  - The remerge diff covers only the two resolved conflicts, each keeping main's side.
  - H passes 74 tests plus `k6_alloc`, and the runner 47 of 47, on a clean archive.

## K6b merged (ROOT, 2026-09-30)

- **Merged:** [PR1058](https://github.com/sgttomas/chirality/pull/1058) at head `597c81ba4`, merge `78f55f927`, 2026-09-30 01:08:38Z. The merge record is `IMPLEMENTATION/K6B_MERGE/RECORD.md`.
- **The gates, on `597c81ba4`:**
  - RV22 PASSED the review, confirmed `011911e4e`, and passed the merge check of `597c81ba4`;
  - hosted CI was green: the four pull_request runs and the full-SHA dispatch 36650532005;
  - DEC-025 was clean against V-K's Mac run (keyed by manifest path): the only suite change is performance_harness, 54 → 74 (K6b's tests). frame_kernel is unchanged, and the failing tests are exactly the three known Mac platform tests. pytest passed 3070 (V-K's 3062, plus K6b's 8 runner tests: `test_k6_runner.py` has 39 → 47), vitest 2822 of 2822, and both builds exited 0;
  - GEN-8 passed.
- **Still routed from K6b:**
  - W1-T4 at 10,000 members re-runs after KF3 merges, as a records addendum (RETURN addendum 1);
  - VR's cited copy of E_max is deduplicated against K6b's, with RV22-2's terms;
  - RV22's C-N1 (no test of a stop inside the solve) stays open as a NOTE.

## KF3: main merged; K6b's parity restored in KF3; B's slot granted (ROOT, 2026-09-30)

- **The merge of main `f8400d290` (V-K) is committed as `c0473301e`.** I19 resolved the one conflict by editing only; ROOT staged and committed.
  - ROOT checked that each resolved file differs from main's side only by the ruled visibility: `bound_refusals` `pub` in adaptive.rs; `BlockRefusal`, `BoundRefusal`, `RefusalKind`, `BoundPass` and `CertifiedBound` `pub` in bound.rs; and one facade line in structural.rs.
  - I19 reports that FK (348 lib, 7 integration files, 6 doc-tests), VR (47) and H build without warnings and pass, and that VR's records regenerate byte for byte, so no VR record changes.
- **Main `78f55f927` (K6b) is merged as `e114b23c1`,** with no conflict. K6b changes no FK file.
- **KF3 merges second, so KF3 updates K6b's checks (the earlier rule).** KF3's write set gains:
  - `performance_harness/src/k6/w1/staged.rs`: `stages_equal_totals` requires equality on every attempt, completed or stopped. Its doc comment says so and cites KF3. `unstaged` and `stages_complete` stay, since the attempt line records them (RV22-1). They are now always (0, 0) and true, and a test asserts that on a stopped build;
  - `performance_harness/tests/k6b_w1.rs`: the `uc == 0` assertion on the budget-stopped build becomes the partial `uc` work KF3 records, and the relaxation tests (one short side accepted) now expect `false`. RV22's C-N2 closes here;
  - `performance_harness/runner/**`, only if the runner encodes the relaxation.
  - Anything else in H is a stop. H's suite (`--all-targets`, with `k6_alloc`) and the runner's suite must pass.
- **B's slot is granted now.** No other slice is building, and ROOT's records work uses no cargo.
  - The slot covers the H update above, then B: RF-LARGE-CHAIN-n10000-AX through W1, and the other five 10,000-member RF-LARGE frames with main's `vk_scale`, each with its outcome and, where it publishes, its honesty against R1.
  - It ends with I19's report.

## KF2: spawn (ROOT, 2026-09-30)

- **KF2 (K6's N10) is spawned as I20** (`TASK_BRIEFS/I20_KF2_IMPLEMENTATION.md`), on branch `codex/piping-kf2-20260930` in `<wt>/kf2`, from main `78f55f927` (K6b merged).
- **ROOT's reading** (for I20 to confirm or refute):
  - the dense negative-pair witness is O(n⁴): every pair allocates an n-vector, re-validates, and scans n² entries;
  - K1's sparse witness already evaluates a pair in O(1), in the dense order.
  - So the dense witness can be made O(n²) with every result bit-identical, errors included. That is the slice.
- **The dense pivot screen** (2j + 2 against the skyline's 2(i − first_i) + 2) is **diagnosed and proposed only.** Changing it changes published dense classes, so it needs ROOT's ruling, and possibly an owner-facing note.
- **Product-reaching gates:** review, T9 (112 of 112), the both-entry gate (part 1 byte-identical; part 2's four dense N10 runs must end within 1,800 s), CI with the dispatch, DEC-025, GEN-8 and the src-tauri suite.
- **Host:** KF3's slot B is running, so checkpoint 0 is reading only. I20's first build waits for ROOT's word.
- **KF2 does not block W1's limits or F2a.** It is on the T3-close list.

## KF3: checkpoint B accepted; two findings for D (ROOT, 2026-09-30)

- **Accepted and committed on the KF3 branch as `ae831ca51`:** H's parity update and B's records.
  - **H:** `stages_equal_totals` holds every attempt to equality. The budget-stopped build carries its partial `uc` work, the one-short-side probes expect `false` (RV22's C-N2 closed), and a new test finds nothing unstaged at every segment end and midpoint of CHAIN-n00010-AX. H's suite and the runner's 47 pass. The runner needed no change.
  - **B:** `vk_scale` in release, from a `git archive` of `e114b23c1`, with V-K's runner unedited.
    - V1 and V2 (12 models) are byte-identical to V-K's B, charged work included.
    - **At 10,000 members:**
      - CHAIN-AX, CHAIN-ROT and CONT-ROT move from `Unresolved(ExactSumSpan)` to **selected at 128**, and pass R1 on every row (103, 103, and 214 plus 1 absolute-range of 215). Uc is refused in the forward pass, so B = S_c, with one shifted factorization each. Charged work is 10.1, 12.1 and 10.4 G LME, against 6.7, 8.3 and 6.8 before KF3.
      - CONT-AX is unchanged.
      - **TREE-AX and TREE-ROT** move from `Unresolved(ExactSumSpan)` to `Unresolved(Ceiling)`. R7's verification estimate (b) rejects them at 128, 256 and 512, on member 1's end force at the root, and the ceiling is reached after the 1024 verification. They publish nothing, before and after, and charge 65.3 and 74.2 G LME (6.6 and 8.1 before).
    - Every published row passes R1, and every C9 S_full set is empty.
    - The refusals' rows match the plan's forward-pass prediction on all five frames.
- **Finding KF3-B1: the TREE frames at 10,000 members are an availability loss of estimate (b), not of KF3.** They are honest: nothing is published.
  - **For D:** from the existing records only, with no new heavy run, give (b)'s estimate against its threshold at the rejecting row for each precision. Classify the loss as a design limit (like THIN) or slack in (b). ROOT rules from that.
  - **For W1's limits:** a case budget ends these cases earlier. Their charged work before `Ceiling` (65–74 G LME) is recorded.
  - **RETURN must say plainly** that the runner summary's `fail` column for these two frames counts unpublished rows, and that no wrong value was published.
- **Finding KF3-B2: the measured heap exceeds E_max.** [Correction (ROOT, 2026-09-30, RV25-S1): the figures below compare against VR's stale port of E_max, and like for like K6b's final formula bounds the measured peaks (see the correction in "KF3: D accepted; KF3-B1 and KF3-B2 routed"). KF3-B2 stands on the code derivation of an under-count, not on a measured excess.] TREE-AX peaks at 2,889.9 MiB against E_max 2,750 (ρ ≈ 1.05), and TREE-ROT at 2,891.5 against 2,752. The ruling on RV22-2 requires E_max to be an upper bound on every phase.
  - **For D:** derive, with file:line, which allocations are alive at the peak, and whether pre-KF3 code on the same path holds them. There are two possibilities:
    - a K6b omission that no earlier run reached (I19's reading: the shifted factorization's two profile copies at 1024 bits);
    - a KF3 change in what is alive.
  - **Routing, once derived:**
    - a K6b omission goes to K6b's post-KF3 follow-up, which becomes a small H PR with its own review: the E_max term, `counts.jsonl` regenerated, and W1-T4 re-run;
    - a KF3-induced term is fixed in KF3 before its PR.
  - **Either way, ROOT's W1 limits wait for E_max to bound every phase at 10,000 members.**
- **Next:** checkpoint D, with KF3-B1 and KF3-B2 derived.

## KF2: rulings on I20's checkpoint-0 plan (ROOT, 2026-09-30)

- **The plan is accepted,** and committed on the KF2 branch as `573bd3835` (`IMPLEMENTATION/KF2/PLAN_CHECKPOINT0.md`, sha256 `76337c43…`).
- **ROOT checked the equality argument (§4) and accepts it:**
  - **The design:** an O(1) guard per pair repeats `verify_negative_direction`'s exact arithmetic on the pair's four cells, in its order. Only a pair the guard marks as a witness is passed to the unchanged verifier, which produces the published value.
  - **Why it holds:**
    - validation's result is the same at every call;
    - the direction check always passes;
    - the verifier's nonzero cells are exactly the guard's cells, in the same order, with the same operations;
    - so both reach the same first error or the same verdict.
  - Zero couplings are still visited, so the equality holds for every value of the type, not only on `prepare_bound`'s invariants.
- **The cost:** ROOT confirmed that the old witness is O(n⁴). The prediction at 6,006 DOFs is about 31 days, a lower bound. At K6's kill about 0.06% of the pairs had been searched. The new witness is predicted at about 1 s; A measures it.
- **Q1: (a), approved.** The tests go in a new `FK/structural/kf2_witness_tests.rs`, and FKS gains one `#[cfg(test)] mod kf2_witness_tests;` line beside `s11f_tests` and `s11k_tests`.
- **Q2: approved.** N10's two models run through H's unchanged `k6_observe`, built in release from a `git archive` of the KF2 tree. H is not edited.
- **Q3: the screen is declined for KF2 and split out.** KF2 stays byte-identical.
  - **What a change would alter:** I20 showed a profile-based operation count is honest, because the dense factor's entries before a row's first nonzero are exact ±0. But the change alters every dense report's bytes, since `PivotEvidence.operation_count` and `.screen` are published (PP:1106-1107): T9's 56 dense outputs, and the dense part-1 envelopes. It also changes dense classes. That is certain on the N10 pair and likely on RF-CHAIN-A and RF-CHAIN-T at n10-r1e-12; RF-SKEW-T-PIN-OFF-122 and RF-WEAK-W-L at r1e-12 are candidates.
  - **Routed:** a separate dense-screen slice on the T3-close list, with an owner-facing note, since dense scrutiny's published standing changes. It is not scheduled yet.
- **Q4: no budget in the witness.**
  - **Recorded for the owner:** the dense route observes cancellation only before the solve and before publication (APP:1688-1692, :1711-1717). A cancelled dense job keeps its thread and memory until the solve returns. After KF2 the dense factor is the long step (65–188 s at 1,000 members). [Correction (ROOT, 2026-09-30, RV25-N3): 65–86 s at 1,000 members; 166–188 s at K6's 1,364-member ceiling (B3).]
  - Routed as an observation to T6 and T9; it is not T3 scope.
- **Q5: the site-table row is authorized:** `("FK/structural.rs", "negative_pair_witness_counted", 6, …)`, one declared, additive row.
- **Q6: yes.** SD's suite is added at A (SD:37 and `k1_tests.rs:371` call the witness).
- **Q7: confirmed.** The one private helper is `negative_pair_witness_counted`, holding the loop, the counts and the pair evaluation inline.
- **Builds may start now:** one cargo job at `-j 4`, `<wt>/kf2-target`, with the memory guard running. No timed slot is held; KF3 is writing D.

## KF3: D accepted; KF3-B1 and KF3-B2 routed; PR to review (ROOT, 2026-09-30)

- **D is accepted, and committed on the KF3 branch as `b8c55c92e`** (records only; the code equals `ae831ca51`'s). It holds RETURN (sha256 `a582135f…`), CHANGE_RECORD (`b9fca3a6…`), `_run_records/{b,d,h,merge}/` and SHA256SUMS (115 entries, verified by ROOT).
- **The write set is confirmed by ROOT against main:** FK's facade line (the merge ruling), `K4R/{adaptive,bound,verify,wide_sum}.rs` (`wide_sum.rs` under ruling 7), K4T's tests, GEN and `kf3.txt`, and H's `staged.rs` and `k6b_w1.rs` (the parity ruling).
- **KF3-B1 (the TREE frames' `Ceiling`) is classified:**
  - Estimate (b) rejects on member 1's end force at the root. Its ratio Ŵ/(2^(6−P)·ê) is independent of P, so no precision passes, as with THIN.
  - I19's probe of R1's comb tree (AX) gives c = 64 × ratio:
    - 20.5 at 1,000 members and 25.3 at 2,000 (both selected);
    - 74.1 at 3,000, 64.8 at 4,000, 142.9 at 6,000 and 177.1 at 8,000.
  - **At 3,000 and 4,000 members this is slack in (b):** R7's Corollary would still hold (c ≤ 127 at p = 128 and 256).
  - **At 6,000 members and above, it is beyond what R7's split of λ = 2^8 leaves for (b)** while (d) keeps 60. That is a design limit of the split, and 10,000 members is taken to be the same, by extrapolation.
  - **Routed:** a D1 design question: re-splitting λ between (b) and (d), and whether (b) can be sharpened on tree roots. It is not KF3's, and not a T3-close blocker.
  - **Owner-facing:** W1a publishes no retained-precision result for R1's large trees at about 3,000 members and more. The binary64 route publishes them with its ordinary class, as today. This joins THIN on the owner's availability list, beside the PHYS-R4 question.
- **KF3-B2 (heap above E_max) is a K6b omission, not KF3-induced.**
  - Against K6b's final formula, the peak inside `nl_pass` on the shifted factor at 1024 (bound.rs:1040) exceeds E_max by 19.5 MB (AX) and 19.4 MB (ROT). [Correction (ROOT, 2026-09-30, RV25-S1): that comparison is not like for like. It sets `vk_scale`'s measured peak against K6b's E_max with `k6_observe`'s fixed term. On `vk_scale`'s own fixed term, K6b's final formula **bounds** both measured peaks, by 7.0 and 7.5 MB (KF3 RETURN §13); that fixed term is itself an estimate. KF3-B2 stands on the code derivation alone: H's pass term under-counts the 1024 shift by a net 10,799,688 B (`at`/`bt`/`ct` omitted, `work` counted after it is freed, `OPTION_EXTRA` over-counted), so E_max is not yet shown to bound that phase. On main the call is at `bound.rs:1059`. The routing to K6c is unchanged.] [Correction (ROOT, 2026-09-30, RV25-D1): the finding is stronger than this bracket says. By construction, K6b's formula does not bound this phase: taken from the code, the phase needs 3,021,565,490 B (AX) against E_max 3,010,765,802 B (KF3 RETURN §13, :440-445). Only a *measured* excess is unshown; the `vk_scale` peak stays inside E_max through slack elsewhere, not through this term.]
    - H's pass term leaves out `at`, `bt` and `ct` (3·nf·w).
    - It counts `work`, which is freed before `nl_pass`.
    - It over-counts `Option<Wide>` by 8 B per entry.
    - Net, it is short by 10,799,688 B at the 1024 shift.
  - Pre-KF3 code holds the same allocations on the same path. KF3 only makes the path reachable (A2's shift, where Uc used to stop the attempt at 256).
  - I19's B-time reading ("two profile copies") described VR's stale port, not K6b's formula; the correction is recorded in RETURN.
  - **Routed to a K6b follow-up (K6c), a small H PR with its own review, after KF3 merges:**
    - E_max's pass and shift terms corrected;
    - the overlap of `uc_bounds`' transients checked;
    - `counts.jsonl` regenerated;
    - W1-T4 re-run on the post-KF3 code (RETURN addendum 1's content);
    - **VR's stale E_max port** (`VR/src/scale.rs`, 153 MB below K6b's final formula; V-K's runner admits with it) replaced by K6b's estimate, deduplicating it.
  - **ROOT's W1 limits wait for K6c.**
- **T9 and the both-entry gate stay unrun for KF3.** `retained_api` is public in FK, but no product crate names it (the V-K and K6b scans). KF3's product-crate reach is unchanged; the reviewer re-runs the scan.
- **KF3's stale doc NOTE** (`H/src/bin/k6_observe/w1.rs:6-10`) goes to K6c.
- **The PR is opened** from `b8c55c92e`, with its full-SHA dispatch. **RV23 is the independent reviewer,** directed to:
  - A2's honesty argument, against every reader of Uc, S and B;
  - the partial-stage identity on all four builds;
  - the controls' invariance;
  - KF3-UC-SPAN's honesty against GEN;
  - the H parity update;
  - B's records.
- **DEC-025 and GEN-8 run on the final head after RV23.**

## K6c briefed; V-K's V3 addendum satisfied by KF3's B (ROOT, 2026-09-30)

- **K6c's brief is committed:** `TASK_BRIEFS/I21_K6C_IMPLEMENTATION.md`. It covers:
  - E_max re-derived phase by phase against the post-KF3 kernel and corrected (KF3-B2);
  - VR's stale port deduplicated;
  - W1-T4 re-run post-KF3, as K6b's reserved addendum 1, in `T3/IMPLEMENTATION/K6C/`, since K6b's records are hash-bound;
  - KF3's stale-doc NOTE;
  - RV22's C-N1 if it fits.
- **I21 is spawned after KF3 merges.** Its base must carry KF3's kernel.
- **V-K's routed V3 re-run** ("V-K merged", Routed) **is satisfied by KF3's B.** B ran V-K's runner and `vk_scale`, unedited, on `e114b23c1`, whose FK equals KF3's final head `b8c55c92e`'s. It covers V1 to V3, outcomes and R1 honesty (`KF3/_run_records/b/`).
  - The E_max column in those records comes from VR's stale port. It is superseded by K6c's corrected estimate. K6c re-checks B's admission decisions against the corrected estimate; ROOT has not verified them. [Correction (ROOT, 2026-09-30, RV25-N2): as first written (`e48774292`), the previous sentence ended "…, and no admission decision there relied on it at the margin". That was an unverified claim, replaced in place 12 s later (`ffc489f52`) with no bracket. The bracket is added here.]
- **ROOT's W1 limits wait for K6c's merge** (the corrected E_max and the post-KF3 W1-T4).

## KF2: checkpoint A accepted; B granted (ROOT, 2026-09-30)

- **A is accepted, and committed on the KF2 branch as `1b10121fa`.**
  - **The code:** FKS's `negative_pair_witness` now delegates to the private `negative_pair_witness_counted`.
    - ROOT read the diff. The guard repeats `verify_negative_direction`'s arithmetic cell for cell, in its order, and only a witness verdict builds a direction and calls the unchanged verifier.
    - There is one test-module line (Q1) and one site-table row (Q5).
  - **The tests:** the differential tests against verbatim reference copies (n = 2 to 100) pass, with 142 witnesses, 98 no-witness cases, and all 9 exact allowance ties found. The count tests pass, and 11 mutants are killed.
  - **N10's two models** (6,006 DOFs, dense, through H's unchanged `k6_observe` in release):
    - each ends in the factor's refusal (DOF 6001 and 6002, after about 66 s);
    - the witness takes 0.94 s and 0.91 s and finds no pair;
    - each process exits at about 203 s, where before they were killed at 1,800 s.
  - **Suites:**
    - FK 415 (1 ignored: the release cost test), site table 3, SD 30, NI with SA 134, and H 74 with `k6_alloc`;
    - PP passes except the known Mac platform test `t13`, which fails identically on the base.
- **B is granted now:** T9, the both-entry gate and the src-tauri suite, as the brief's "Gates" and F1b's (`I13_F1B_IMPLEMENTATION.md`) define them.
  - **T9:** 112 of 112 byte-identical, plus F1b's extra corpus (16), from `git archive` copies of base `78f55f927` and candidate `1b10121fa`.
  - **Gate part 1:** 884 runs on the candidate, **against a fresh Mac run of base `78f55f927`**, with P1's probe and the calibration's heap cap. PASS needs:
    - every run byte-identical to the base (envelope sha256 per case, mode and entry);
    - 0 trusted breaches;
    - no heap-cap abort.
    - FK has changed since F1b's gate, in paths the product does not reach, so a fresh base keeps the comparison unambiguous.
  - **Gate part 2:** the four dense N10 runs (both entries), candidate only.
    - Each must end within 1,800 s, with its outcome recorded. The expected outcome is the factor's refusal on both entries, since the witness finds no pair.
    - The base's timeouts are already recorded (F1b's gate and K6's B2).
    - The class disagreement with sparse (Sensitive) remains, and is the split-out screen slice's.
  - **The src-tauri suite,** as F1b ran it (`F1B_MERGE/src_tauri/`).
  - **Host:** one cargo job at `-j 4`, the memory guard running, and no timing compared. RV23 (KF3's reviewer) is building beside it, which is allowed.

## KF2: checkpoint B accepted; D now (ROOT, 2026-09-30)

- **B is accepted, and committed on the KF2 branch** (`_run_records/b/`, SHA256SUMS `247459f7…`, verified by ROOT). It ran on this Mac, with base main `78f55f927` and candidate `1b10121fa`, as `git archive` trees that differ only in KF2's three FK files.
  - **T9:** 112 of 112 byte-identical, and F1b's extra corpus 16 of 16. The base also equals the platform calibration's Mac hashes.
  - **Gate part 1: PASS against a fresh base run.**
    - All 884 runs are identical: outcomes, exit codes, summary and full envelopes (818 each), and error text.
    - `gate_check` finds 0 trusted breach triples on both sides, and there is no heap-cap abort or timeout.
    - Sixteen dense runs reach the witness (FX-NP-A-ulp-* and the four r1e-12 cases). The witness finds no pair, and they are byte-identical to the base.
    - ROOT checked the two uncommitted 606 MB `runs.jsonl` files against their recorded sha256 (base `c42981e5…`, candidate `11dfe829…`). They stay in `<wt>/scratch/i20/b/gate/`, as F1b's did.
  - **Gate part 2:** the four dense N10 runs, both entries, end in the dense factor's refusal (`NUMERICAL_INTEGRITY_UNRESOLVED`, DOF 6001 and 6002) in 67–68 s each. They were previously killed at 1,800 s.
  - **src-tauri:** 116 passed, as in F1b's run.
- **D now:** RETURN, CHANGE_RECORD and SHA256SUMS. Then ROOT opens the PR with its dispatch.
- **I20's scratch** (`<wt>/scratch/i20/b`, `<wt>/kf2-target`) is kept until KF2 merges, for its reviewer.

## KF3: rulings on RV23's review (ROOT, 2026-09-30)

RV23 (`REVIEW/KF3_REVIEW.md`, sha256 `96060f9e…`; records `REVIEW/_run_records/kf3_review/`) reviewed head `b8c55c92e`: **PASS**, with 0 BLOCKING, 1 SHOULD-FIX and 5 NOTEs. RV23 found no way for A2 to publish a wrong value.
- **A2's honesty:** every reader of B goes through `certificates`/`certified`, and blocks are contiguous, so no pass reads another block's rows. RV23 built three oracles of its own:
  - synthetic multi-block refusals, where B ≥ the exact norm and B = min over the formed bounds;
  - 52 forced-refusal W1 runs on 16 controls;
  - KF3-UC-SPAN in closed form.
- **The partial-stage identity:** 0 mismatches over 3,611 W1 runs with case limits on 11 models, and over 2,011 on H's model.
- **B:** reproduced byte for byte on CHAIN-n10000-AX and TREE-n10000-AX.
- **Reach:** no product crate names `retained_api` or `retained`.
- **Mutants:** 18 plus NONE, of which 15 are killed and 3 survive (RV23-N1's M4b, and the two equivalents in RV23-N4).

**Rulings:**
- **RV23-1 (SHOULD-FIX): fix it before merge.** Carry the refusals out on the error path.
  - **The defect:** a refusal recorded in the verification's shared build (`uc_bounds`), or in `shift_schedule` (S), must reach `AttemptRecord::bound_refusals` even when a budget stop follows it in the same build. Today it is dropped after `vs?` (`adaptive.rs:3028-3030`).
  - **The test:** RV23's probe (KF3-UC-SPAN with the case limit 1 to 100 LME short of the shared build's total) must record the refusal on the `Failed(Stop(Budget(Case)))` attempt. Add a test for each of the two sites, and a mutant for each.
  - **No outcome, row, class, bound or work count may change;** evidence only.
  - The claim "on every path" in the `AttemptRecord` doc, RETURN §3.1 and CHANGE_RECORD then holds as written.
- **RV23-N1: add the test.** Precedence in `verify_state` (a refusal stop outranks `uc`) gets a test that kills M4b. It is cheap, and the rule is a ruled decision (plan §11 item 2).
- **RETURN §12's wording** (RV23's routing check): "c is a property of the model" holds only where (b) binds. Correct it in a RETURN addendum; the routing is unchanged.
- **RV23-N2, N3, N4 and N5 are recorded,** with no change. N5 is already routed to K6c.
- **Then:** RV23 confirms the new head. After that come CI with the dispatch, DEC-025, GEN-8 and the merge.

## KF2: D accepted; PR to review (ROOT, 2026-09-30)

- **D is accepted, and committed on the KF2 branch as `f2b8c85a2`:** RETURN (`fe19dbc6…`), CHANGE_RECORD (`1457a990…`) and SHA256SUMS (`5b51af43…`, verified by ROOT). The code diff against main is exactly the three FK files.
- **The PR is [#1060](https://github.com/sgttomas/chirality/pull/1060),** with the full-SHA dispatch 36664104717 (target_base `78f55f927`).
- **RV24 is the independent reviewer,** directed to:
  - the equality argument, errors included;
  - the differential tests' coverage and the reference copies' fidelity;
  - the mutants;
  - B's records: T9, the gate's two parts and src-tauri;
  - the callers' reach;
  - the screen diagnosis, as a claim check only.
- **Then:** CI, DEC-025 on the final head, GEN-8 and the merge.

## KF2: rulings on RV24's review (ROOT, 2026-09-30)

RV24 (`REVIEW/KF2_REVIEW.md`, sha256 `6785aa13…`; records `REVIEW/_run_records/kf2_review/`) reviewed head `f2b8c85a2`: **PASS**, with 0 BLOCKING, 1 SHOULD-FIX and 5 NOTEs.
- **The equality holds.** RV24 read the code line by line against the base. Its own differential harness, whose oracle is a verbatim copy generated from the base's bytes, found 0 differences in debug and release. It covered:
  - 24,000 corrupted systems, with errors before and after witnesses;
  - 2,864 built systems;
  - 134,400 steps around the verdict's crossing;
  - skewed pairs and 29 adversarial cases.
- **I20's reference copies are verbatim,** apart from the names and a dropped `pub`.
- **B's records hold.** RV24 checked the uncommitted `runs.jsonl` hashes: only the time and memory fields differ over 884 records. RV24 rebuilt the probe from its own archives: a 20-run sample is byte-identical, an instrumented head matches 860 of 860 part-1 runs (10,000-member runs excluded under the host rule), and N10 CHAIN-ROT reproduces, with all 17,997,000 pairs visited.

**Rulings:**
- **RV24-1 (SHOULD-FIX): add the tests before merge. It is test-only.** Three single-edit regressions of the guard survive the committed tests and are killed only by RV24's harness:
  - the coupling cells swapped (RV24-M1);
  - the source cell transposed (RV24-M5);
  - the allowance charged for four cells whatever `terms` is (RV24-M4b).

  I20 adopts RV24's cases (`_run_records/kf2_review/`):
  - the asymmetric-source error-order cases;
  - a skewed edge scan;
  - edge scans for 2- and 3-term pairs.

  Each of the three mutants must then be killed by the committed tests. No product file other than the test file may change.
- **RV24-N1:** the two equivalent mutants are recorded.
- **RV24-N2:** the witness also runs on the `sparse_interactive` route (SA:2017 serves both modes), in the nonlinear loop (NI `lib.rs:1990` and `:2013`) and in SD:37. The caller list is complete and the signature unchanged. **ROOT amends the PR text to name these routes.**
- **RV24-N3:** no T9 output reaches the witness, so T9 is an invariance check only. B's SUMMARY misstates the eight captured-entry FX-NP-A runs, which publish a recovered result, not the refusal. The correction goes in a RETURN addendum, since B's records are hash-bound.
- **RV24-N4:** plan §7's screen claims check out. RV24's notes are carried to the split-out dense-screen slice:
  - measure the refusing rows' pivots;
  - enumerate class changes by running, not by prediction;
  - treat the published `operation_count` change as a contract note.
- **RV24-N5:** the cost reproduces (0.966 s at n = 6,006; the old search grows about ×16 per doubling). Recorded.
- **Then:** RV24 confirms the new head, then CI, then DEC-025 on the final head, then GEN-8 and the merge. If KF3 merges first, KF2 merges main, and RV24's confirmation includes a merge check.

## KF3 and KF2: review fixes committed; confirmations requested (ROOT, 2026-09-30)

- **KF3 `aa83f6796`** (by I19; RV23-1 and RV23-N1):
  - **The fix:** the caller owns the refusal slots, so a stopped build's Uc refusals and the S refusals kept when `shift_schedule` stops reach `bound_refusals`. It is evidence only.
  - **Tests and mutants:** RV23's probe, a swept three-block S test, and the `verify_state` precedence test. Six mutants are killed, including M4b.
  - **Suites:** FK 351 lib, 7 integration files and 6 doc-tests; `gen --check` 24 of 24.
  - **CI:** the dispatch is 36669370536. RV23 is confirming.
- **KF2 `1c7558df5`** (by I20; RV24-1):
  - **The change:** RV24's cases are adopted into `kf2_witness_tests.rs`, and RV24-M1, M4b and M5 are killed from clean archives. No product file changes. FK passes 417.
  - **I20 corrects RV24-N3's count:** 4 of the 16 witness runs (the captured-entry FX-NP-A runs) publish a recovered result, and 12 the refusal. RV24 checks this at confirmation.
  - **CI:** the dispatch is 36669470111. RV24 is confirming.
- **Merge order:** KF3 first, if RV23 confirms. KF2 then merges main, and RV24 adds a merge check.

## KF3: RV23 confirms aa83f6796 (ROOT, 2026-09-30)

- **RV23's confirmation at `aa83f6796`: PASS.** There is no new BLOCKING or SHOULD-FIX finding, and one new NOTE (`REVIEW/KF3_REVIEW.md`, "Confirmation at aa83f6796"; sha256 `0030aa39…`).
  - **RV23-1 is closed on both sites:**
    - the Uc probe now records `[block 0, Uc, Span, Backward, row 66]` on the budget-stopped attempt;
    - RV23's new two-case probe shows a cached non-budget failure passing its refusal to the later case;
    - RV23's three-block S sweep (799 case rooms) never loses, changes or invents a refusal.
  - **M4b is killed** by the new `verify_state` precedence test.
  - **Nothing else changed.** Every earlier probe re-runs byte-identical, apart from the refusals now recorded. FK's suite and `gen --check` pass on a clean archive, and product-crate reach is unchanged.
- **RV23C-N1 (NOTE) is recorded, with no test now.** The cache's refusal-keeping on a non-budget failure is shown by RV23's probe, but it is not covered by I19's tests. It needs a non-budget, non-refusal stop after a refusal, and FK has no hook to force one; honesty is unaffected.
  - **Routed:** if a fault hook for an `Arithmetic` stop is added (V-K's `mutation-controls` sites are the natural home), a two-case test goes with it.
- **The merge path:** CI on `aa83f6796` (dispatch 36669370536), DEC-025 (running now), GEN-8, then the merge.

## KF3 merged (ROOT, 2026-09-30)

- **Merged:** [PR1059](https://github.com/sgttomas/chirality/pull/1059) at head `aa83f6796`, merge `dd61120ff`, 2026-09-30 05:28:58Z. The merge record is `IMPLEMENTATION/KF3_MERGE/RECORD.md`.
- **The gates, on `aa83f6796`:**
  - RV23 PASSED the review and confirmed the head;
  - hosted CI was green, and the dispatch (36669370536) succeeded;
  - DEC-025 was clean: the only changes are frame_kernel 402 → 417 and harness 74 → 75, exactly KF3's added tests;
  - GEN-8 passed.
- **Disclosed: ROOT merged without first checking that main had moved** (to `45ffd91d1`, PR #1061). The check made right after shows that #1061 changes only `projects/chirality-app-v4/**`, and that the merged piping, `tools/` and `.github/` trees equal the gated head's byte for byte. The gates therefore cover what merged. The lesson is added to the handoff's §7.2.
- **Next:**
  - KF2 has merged main `dd61120ff` as `522167ac6` (clean; `structural.rs` is main's plus exactly KF2's delta). RV24's merge check, CI and DEC-025 are running.
  - K6c spawns after the pause, on a base that carries KF3.

## KF2 merged (ROOT, 2026-09-30)

- **Merged:** [PR1060](https://github.com/sgttomas/chirality/pull/1060) at head `522167ac6`, merge `7ad3a9adf`, 2026-09-30 06:08:13Z. The merge record is `IMPLEMENTATION/KF2_MERGE/RECORD.md`. ROOT checked immediately before the merge that main had not moved from `dd61120ff`.
- **The gates:**
  - RV24 PASSED the review, confirmed `1c7558df5`, and passed the merge check of `522167ac6` with no findings;
  - B's T9 (112 of 112), gate part 1 (884 of 884), part 2 (four runs in 67–68 s) and src-tauri (116) pass;
  - hosted CI was green, and the dispatch (36673660523) succeeded;
  - DEC-025 was clean: the only change is frame_kernel 417 → 432 plus 1 ignored, exactly KF2's added tests;
  - GEN-8 passed.
- **Routed:** the dense-screen slice (with RV24-N4's notes), and the dense cancellation note to T6 and T9.

## Records PR #1062: rulings on RV25's review (ROOT, 2026-09-30)

RV25 (`REVIEW/RECORDS_PR1062_REVIEW.md`, sha256 `45b27642…`; records `REVIEW/_run_records/records_pr1062_review/`) reviewed head `b0e357985`: **PASS**, with 0 BLOCKING, 2 SHOULD-FIX and 10 NOTEs.
- **What held:**
  - **Scope:** 1,031 paths, all under `_Coordination/`; the remerge diff is empty.
  - **Append-only:** one insert-only "[Superseded …]" bracket; every other change appends.
  - **Hashes:** all 25 SHA256SUMS files verify.
  - **Leaks:** GEN-8 passes.
  - **The nine merge records** match GitHub and Git exactly, KF3's disclosed slip included.
  - **Figures:** 25 sha256 prefixes and more than 90 figures match their sources.

**Both SHOULD-FIX findings are fixed before merge, as for PR #1049. Each is a ROOT figure error:**
- **RV25-S1:** KF3-B2 was stated as a measured excess of 19.5/19.4 MB over K6b's E_max. That comparison is not like for like. On `vk_scale`'s own fixed term, K6b's formula bounds both peaks by 7.0 and 7.5 MB (KF3 RETURN §13). The finding stands on the code derivation of a net 10,799,688 B under-count at the 1024 shift.
  - **Corrected in place with brackets:** the KF3-B2 lines in "KF3: checkpoint B accepted" and "KF3: D accepted"; K6c's brief (its premise, and a like-for-like bar); the work graph's state paragraph; and the handoff §8.1. [Correction (ROOT, 2026-09-30, RV25-D2): the work graph's paragraph and handoff §8.1 are text new in this PR, and were reworded directly, not bracketed. This commit message said the same wrong thing.]
  - `bound.rs:1040` is noted as `:1059` on main.
- **RV25-S2:** V-K B's "0.82 GB" and "2.7 GB" were MiB/1,000. They are bracketed with VK RETURN §14.3's figures.

**The NOTEs:**
- **Fixed:**
  - N2: a bracket at the admissions sentence, and handoff §7.2 rewritten; the count is now seven;
  - N3: the dense factor's time, bracketed;
  - N4: "before `uc`", bracketed;
  - N5: the `REVIEW/_run_records/SHA256SUMS` exception added to handoff §6;
  - N6: the handoff's THIN credit, the line count and the §1.4 list;
  - N7: `sweep_kf2` is kept as the Mac baseline, and the handoff overrides KF2_MERGE's prune list;
  - N8: "APP" is replaced by `P/apps/desktop/src-tauri/src/lib.rs` in the work graph and the handoff.
- **Disclosed in the PR body:**
  - N1: the work graph's :62 carries RV16-D1's one-clause rewording, and the rulings append starts at "K6: rulings on I15's checkpoint-0 plan";
  - N9: whitespace in raw logs and 3 lines of `KF2_REVIEW.md`, and executable `.sh.txt` copies. The latter are already common on main, so they are left as they are.
- **N10, recorded here:** `K5_MERGE/RECORD.md` says its baseline's piping tree equals main `b37331092`'s. It differs by 741 records-only `_Coordination/` paths, and no product or test path, so the suites are unaffected. The hash-bound record is not edited.
- **Then:** RV25's delta check of the fix commit, then the merge.

## Records PR #1062 merged; T3 paused for the audit (ROOT, 2026-09-30)

- **Merged:** [PR1062](https://github.com/sgttomas/chirality/pull/1062) at head `7b59a1efc`, merge `490b75bd9`, 2026-09-30 07:10:41Z, with `--match-head-commit`. ROOT checked immediately before the merge that main had not moved from `7ad3a9adf`.
- **Review:**
  - RV25's review at `b0e357985`: PASS, with 0 BLOCKING, 2 SHOULD-FIX (S1 and S2, both ROOT figure errors, fixed in `18b625268`) and 10 NOTEs;
  - its delta check at `18b625268`: PASS, with NOTEs D1–D4, fixed in `7b59a1efc`; [Correction (ROOT, 2026-09-30, RV26-N10): D3 was fixed in the PR body, not in `7b59a1efc`. RV25's final review is sha256 `0ab9812e…`.]
  - its delta check at `7b59a1efc`: PASS, with no new finding.
  - The last delta check's section and records (`delta_7b59a1efc/`) are committed here, after the merge, as for RV16.
- **Hosted CI on the head:** 7 passed and 6 were skipped, as selected for a records-only change. GEN-8 passed at each head.
- **Main now carries:**
  - every T3 record through the audit pause;
  - the work graph's state at the pause;
  - `HANDOFF_2026-09-30_AUDIT_PAUSE.md` and its tools.
- **This section, and RV25's last delta records,** stay on numerics until the next records PR.
- **T3 is paused.** No agent is running, and no slice PR is open. On resumption, follow the handoff's §10.

## Operating notes for the pause; a second records PR (ROOT, 2026-09-30)

- **At the owner's request,** ROOT writes `OPERATING_NOTES_2026-09-30.md`. It succeeds the 2026-09-28 operating notes and complements the handoff: how ROOT ran each slice and checked agents' reports, the judgment calls, ROOT's own errors and the fix that works, the environment's quirks, working with the owner, and what ROOT would do differently.
  - The handoff and the work graph's pause paragraph gain a pointer to it; both are insert-only.
- **The owner chose to land it on main now,** through a small records PR with an independent records review (RV26). It carries this file, the pointers, and the numerics commits since PR #1062 (RV25's last delta check and "Records PR #1062 merged").
- **The notes are practice, not rulings.** Where they touch a ruling, this file governs.

## Records PR #1063: rulings on RV26's review (ROOT, 2026-09-30)

RV26 (`REVIEW/RECORDS_PR1063_REVIEW.md`, sha256 `60813369…`; records `REVIEW/_run_records/records_pr1063_review/`) reviewed head `21e2285e3`: **PASS**, with 0 BLOCKING, 2 SHOULD-FIX and 10 NOTEs.
- **Scope, append-only, hashes, leaks and GEN-8 all pass.** Every named finding and event in the operating notes matches its source.
- **Both SHOULD-FIX findings are fixed before merge, in the notes' own new text:**
  - **RV26-S1:** KF2's B "took a few hours" was another ROOT figure error. Its recorded runs span about 28 minutes. The notes now give the recorded figure and flag the error.
  - **RV26-S2:** the notes admitted one process slip; the records show at least four. The notes now list them.
- **The NOTEs:**
  - **Fixed in the notes:** N1–N7 and N5/N6's marking of statements that rest on session experience or on the owner's words in this session.
    - **N6:** the owner's words are not added to `OWNER_DIRECTION.md`. They are working preferences stated in this session, not directions, and the notes now say so.
  - **Insert-only on main's text:**
    - N7: the handoff's pointer is disambiguated;
    - N8: K6c's brief now requires mutant diffs;
    - N9: the handoff's prune inventory is completed;
    - N10: a bracket in "Records PR #1062 merged".
- **Then:** RV26's delta check, and the merge.
- **RV26's delta check at `95bb2e700`: PASS,** with 1 new SHOULD-FIX (D1) and 3 NOTEs.
  - **D1** is fixed in the notes: from K5 on, DEC-025 ran on each exact final head. On 2026-09-28, M03's and K3's sweeps were carried to a tests-and-records-only head, as their merge records disclose.
  - D2 to D4 are also fixed, in the notes and in the handoff's inventory line, which is new in this PR. The notes gain the next free numbers and a reviewer-prompt shape, since no reviewer prompt has been committed since RV13.
  - Then RV26's short confirmation, and the merge.
- **RV26's confirmation at `2a6f87562`: PASS,** with two optional NOTEs, C1 and C2. Both are fixed in the notes: C1 cites the M03 and K3 merge records' own justification of their sweep carry-over, and C2 corrects "RV19" to "RV14". RV26 confirms the fix, then the merge.

## Records PR #1063 merged (ROOT, 2026-09-30)

- **Merged:** [PR1063](https://github.com/sgttomas/chirality/pull/1063) at head `a2768ce49`, merge `74b3c7313`, 2026-09-30 12:48:09Z, with `--match-head-commit`. ROOT checked immediately before the merge that main had not moved from `490b75bd9`.
- **Review:**
  - RV26's review at `21e2285e3`: PASS, with 2 SHOULD-FIX (fixed) and 10 NOTEs;
  - its delta check at `95bb2e700`: PASS, with D1 fixed and D2–D4;
  - its confirmations at `2a6f87562` and `a2768ce49`: PASS, with C1 and C2 fixed.
  - RV26's last confirmation section is committed here, after the merge.
- **Hosted CI:** 7 passed and 6 were skipped, as selected for a records-only change. GEN-8 passed at each head.
- **Main now carries `OPERATING_NOTES_2026-09-30.md`** beside the handoff. T3 stays paused for the owner's audit.


## Audit accepted; bounded numerical recovery authorized (ROOT, 2026-09-30)

The owner reviewed ROOT's verification and recommendation, asked for the work-graph
account and Agent 0 orchestration plan, then directed: "Proceed accordingly."
The accepted plan and initial briefs are in RESUME_2026-09-30/. These are execution
briefs, not reusable instructions or acceptance of a new numerical design.

- **AUD-T3-01: accepted as SHOULD-FIX; open.** Subnormal row rounding before
  multiplication by L or division by L refutes the scale-transfer premise.
  ROOT reran the arithmetic and checked it by hand (verification/arithmetic.json).
  No unmutated false solver publication is established by the audit or preserved
  B01/B02 evidence. Such a witness makes this BLOCKING.
- **Explicit reliance hold:** F2a must not rely on the published-bound guarantee
  until the complete coupling argument, realized-source/control investigation and
  independently reviewed correction or proved exclusion close the finding.
  An amendment by ruling receives independent design-level re-derivation before
  reliance; the original A1 ruling does not discharge the missing premise.
- **Owner-selected direction:** bounded diagnosis and a conservative valid
  publication guarantee, with named refusal wherever it cannot be established.
  Extreme-model solvability is not the objective. No physical cutoff, new bound
  formula, D2 contract or availability change is selected here. Concrete changes
  to the published contract or supported domain return to the owner.
- **A2:** the audit and post-merge review sustain its conditional minimum-of-
  formed-certified-bounds argument. This is not universal runtime qualification,
  A1 closure or a memory-bound proof.
- **AUD-T3-02: adopt the replay erratum.** Original SHA256SUMS use their containing
  directory except REVIEW, DESIGN_NUMERICS, DESIGN_STANDING, REFERENCES and
  REFERENCES_ELOAD: each _run_records/SHA256SUMS uses its named parent.
  Preserve the original manifests.
- **AUD-T3-03: adopt the exception ledger.** S11-K, K2b and K6 disclosed
  intent-to-add/reset pairs; K6b disclosed fetch; S11-K's separate fast-forward
  was expressly authorized by its fix brief. Reported nil net effect does not
  establish no transient write or complete command-history compliance. ROOT
  owns every current Git/index mutation, including fetch and staging.
- **AUD-T3-04: accepted NOTE; preservation open.** Original Mac suite logs and
  KF2 gate JSONL are available; ROOT matched the latter to their original hash
  record. Preserve originals and sanitized recoverable copies before pruning.
  Local availability alone is not completed custody. K4's dated-generator-input
  and sparse-checkout limitation remains disclosed; no tooling repair is granted.
- **AUD-REV-N1: adopt input binding.** A script's constant BASE is not proof of
  its working inputs. ROOT's verification identifies the actual checkout, original
  script and principal input hashes, matched before replay. Preserve historical
  audit files and label changed-input analyses as new evidence.
- **PR #1064:** merged at 3bddc2b05f6106e969c7cf43373b230845c7cc66 before independent
  review. The later Response/instances/AUDIT-REVIEW/REVIEW.md at
  520d7dfb790bcedabc03e92b9692884ce295be54 confirms the defect, qualified as to
  reachability, with no blocking or SHOULD-FIX review finding. Its timing does
  not become pre-merge coverage or a future review waiver.
- **PR #1066 disposition authorized:** close unmerged, retain its branch and head
  520d7dfb790bcedabc03e92b9692884ce295be54. Its records are incomplete inputs,
  not numerical remediation or accepted design. Record actual closure after
  success. No guard development is carried forward.
- **Attribution correction:** the owner identified the supporting-tool drift and
  stopped the response. Its "ROOT's scope correction" wording must not obscure
  the owner's intervention. Old hash-bound records stay unchanged.
- **K6c remains required:** original I21 brief, full phase derivation, H/VR
  deduplication, final admissions and W1-T4 against the settled kernel. Sensitivity
  replay is not E_max proof; unsealed source_finish_01 is input only.
  ROOT's W1 limits wait for closure.

Response means the HELP-HUMAN-PIPING-20260930-T3-AUDIT-RESPONSE run under the
project coordination AgentRuns directory. Its REPLAY_AND_PROCESS_ERRATA.md
supplies the adopted errata with original evidence pointers.

ROOT retains Git, rulings, graph and host scheduling. A fresh WORKING_ITEMS manager
integrates I22/I21; the independent oracle and fresh reviewers report to ROOT.
TASKs make no Git/index writes or delegations. Initial grants are source-only;
actual parentage and scope are recorded at dispatch. Use the existing M5 guard.
A tooling blocker returns to ROOT and the owner.

After A1/K6c and W1 limits: F2a/S-G1, S-I, S-J where required, F2b per domain,
F3/S-G2/S-E1, V-P and join, plus T3-close obligations. Owner choices on ceilings,
PHYS-R4/availability, observation framing, KF3-B1 and KF2 dense screen stay open.
Every PR, records included, retains exact-final-head gates and fresh independent
review under the owner's current resumption direction.

## PR #1066 closed unmerged (ROOT, 2026-09-30)

ROOT closed PR #1066 under the owner's accepted disposition. GitHub reports CLOSED,
closedAt 2026-10-01T04:44:25Z, with head
520d7dfb790bcedabc03e92b9692884ce295be54. A fresh git ls-remote confirms the
response branch still points to that head. Nothing was merged or deleted.
Its numerical results, conditional designs, guard and unfinished K6c packet
retain the limits stated in the preceding ruling and scope handoff.

## Resumption dispatch and raw-evidence return (ROOT, 2026-09-30)

The native WORKING_ITEMS recovery manager /root/t3_recovery_manager and its
fresh I22/I21 TASKs are running source-only checkpoint work. ROOT's independent
oracle /root/a1_exact_oracle was stopped after it disclosed premature expected-
answer exposure; its only return is provenance, not an accepted oracle result.
A fresh /root/a1_oracle_fresh excludes old expected-answer/output records until
its independent derivation is frozen. DISPATCH.json and DISPATCH_UPDATES.jsonl
record parentage, supplied bases and scopes. No Rust experiment has been granted. [Correction (ROOT, 2026-09-30, RV27-S1): this describes the initial source-only dispatch. The later BRIEFS/I22_G0_BUILD.md in this records candidate authorizes one probe build only; no solver case or numerical acceptance is granted here.]

The preservation TASK returned a sealed packet in RESUME_2026-09-30/evidence.
ROOT verified its seal and file hashes and independently restored both compressed
gate files to their original recorded hashes (ROOT_EVIDENCE_CHECK.json).
The evidence README gives inventory, final-head bindings, transformations and
restore instructions. ROOT accepts the existing-policy size warnings for the
two canonical gate gzip copies; no storage tool or policy is changed.
This prepares Git recovery of sanitized logs and original gate bytes; independent
records review and remote integration remain pending. The original raw scratch
and current Mac baseline stay intact; no pruning is authorized. The generator-
input/sparse-checkout limitation remains separately disclosed.

The manager disclosed uncertainty about optional index stat-cache refresh from
its initial read-only git status commands. No explicit mutation was reported;
the record does not assert a complete no-write history. Subsequent delegated
Git reads require GIT_OPTIONAL_LOCKS=0. No index repair or historical rewrite
is authorized by this clarification.

- **Evidence location correction (same ROOT integration):** GEN-8 detected the
  literal scan patterns in the returned COMMANDS.md as an unclassified active
  surface. The complete sealed packet was relocated byte-for-byte to
  RESUME_2026-09-30/_run_records/aud_t3_04/. EVIDENCE_RELOCATION.json maps its
  original return prefix to the current one; the original seal is unchanged.
  No test, checker or sealed evidence text was edited. The first GEN-8 failure
  remains recorded; rerun is required before the records candidate merges.


## Records resumption: RV27-S1 corrected (ROOT, 2026-09-30)

RV27 independently reviewed candidate 90b6bcbbf64b13975211038bf3f33bb87273e646.
Its REVIEW.md is SHA256 8bb6773bfb27f2e11a3b4f2a2dd1e740cc92fcb8f1a74fcc4ca0792a619658f6,
under REVIEW/RESUME_RECORDS_RV27 on numerics. Its sole SHOULD-FIX, RV27-S1,
is corrected here: the current graph identifies initial source-only dispatch
and the later included G0 build-only authorization. The dispatch ruling retains
its historical words with an explicit correction bracket. This frozen records
slice contains no solver-case grant or numerical acceptance. ROOT verified
RV27's seal; the same reviewer must confirm this correction before merge.

## AUD-T3-01 escalated to BLOCKING: C17 source witness (ROOT, 2026-09-30)

ROOT read the unchanged C17 raw output, rehashed it and the G0 binary, checked
the independently frozen exact truth and recomputed the interval inequality.
The independent oracle TASK separately confirmed it without changing its frozen
truth or comparator. I22/c_01 and oracle_fresh/addendum_02_C17 preserve the result.

C17 is an admitted unmutated PrimitiveSource selected at p128/P256. D:0 and M:0
publish +0, AbsoluteVerified with bound bits 0000001000000000. Exact primitive
force balance gives 9*2^-1041 in model length units; b=2^-1038, so error/b=9/8.
Even b*(1+2^-22) is too small: the exact qualified ratio is 4718592/4194305.
These are two false kernel row publications, not merely a proof counterexample.

Raw stdout SHA256: 841167ed887a71b0e2a904647e1f217758fad304baaaf0d669f5c47035dc797a.
Binary SHA256: bcbe897204ec702b99529d25e6d0213d0132af5e6086e0397fae3a8f8ef8a08f.
Source: 3bddc2b05f6106e969c7cf43373b230845c7cc66, FK-identical to d01ad98...
The run uses the unchanged probe, no seeded feature, and the G2 fixed limits.
The source-level comparator and independent derivation are in the A1 worktree's
RESUME_2026-09-30 records; ROOT is preserving them as their own pinned Git basis.

AUD-T3-01 is BLOCKING under the audit's existing escalation rule. F2a reliance
remains held. C18-C24 are unrun; the C sequence stopped after this first witness,
with no repeat, source repair or new search. The corrected post-read path error
does not change any solver output. B remains honest within its recorded coverage.

No product/native false publication is established. The literal C17 numeric
transport is excluded by the current product magnitude profile; equivalent
product/unit exclusion is not proved. This qualification does not excuse a
false publication by the admitted kernel API.

A fresh HELPS_HUMANS design assignment now independently derives a warranted
correction and its contract/availability effects. A fresh design reviewer must
re-derive it before reliance. No new bound, cutoff or implementation is selected
by this escalation. K6c stays unaccepted and tied to the settled A1 kernel.

- **Durable C17 basis:** complete diagnostic/oracle/manager packets are committed
  and pushed on the retained A1 branch at a6b40d2d036acac556e28f28f5b482a4adb39333.
  RESUME_2026-09-30/C17_EVIDENCE_BINDING.json pins paths, raw/binary hashes,
  manifests, scope and restore method. The records PR now incorporates the
  blocking escalation before merge; earlier review confirmations cover their
  named historical candidates only. A fresh design agent is executing the
  committed bounded brief; no repair or design alternative is accepted.

## A1 certificate correction selected for bounded implementation (ROOT, 2026-10-01)

ROOT selects the sealed independent design and additive correction, after
RV28's independent re-derivation and VERIFIED backcheck. Source-qualified
identities/hashes are in RESUME_2026-09-30/BRIEFS/I22_IMPLEMENTATION_A.md.
RV28-1 is closed at specification level; its conversion-work note remains an
implementation/review obligation. This satisfies independent design checking
before reliance; it does not accept unimplemented code.

The selected correction certifies H=|actual published x-verification v|+E using
the complete existing R7 error formulas and prerequisites. Absolute rows require
H<=the unchanged b. Relative rows require the public decimal predicate and both
the exact and current binary64 tighter allowances. An uncertifiable candidate
escalates or refuses by name. Existing scale/class/bound bits and source domain
are not widened. [Correction (ROOT, 2026-10-01): the unchanged
quantity is the construction at the same selected candidate/floor, not equality
of old and new output bits. Escalation can activate the existing p512 floor and
change scales/bounds. ROOT_REPAIRED_B_FIELD_COMPARISON.json under
RESUME_2026-09-30/verification binds that effect to the old/new raw hashes;
values/ranges/classes remain equal for these B cases. No new public formula or
source-domain cutoff is selected.] Private upward binary64 radii accompany the identical certified
publication; they are not a new public receipt radius. The addendum pins SI
normalization/binding and later facade obligations.

This bare-b choice implements the owner's already accepted conservative-refusal
direction. Qualified intervals, outward public bounds/scales, input cutoffs and
reversal of protected availability remain unselected decisions. No fresh owner
approval is created for faithful implementation of this choice.

ROOT reserves M03-INTEGRITY-MP-v2 in the current kernel policy mechanism, keeping
the method token. Historical v1 records are not retroactively certified.
F2a/D2 recognition, final-row/unit/derived certification and runtime qualification
remain required before product reliance.

I22 may author only the four maintained files in the committed A grant, with
additive evidence. No build/test/solver/mutant or snapshot refresh is yet granted.
The source branch now carries current main at 0ce33d7e89a306cfc01f3ce29f421f4dbd02e716.
ROOT will verify the core diff/hashes, commission a fresh source reviewer and
run every required gate on the actual candidate before merge. A1 is still
BLOCKING, K6c unaccepted and F2a held.


## Audit resumption records merged (ROOT, 2026-10-01)

PR1068 merged at 2026-10-01T06:38:11Z as
546e05a159a58f5ceffe1d315373bc2f31982ea8, exact candidate
729e80b5c2a33b278623a850d9c75c25372b688c. ROOT checked main a38617d
immediately before the guarded merge. RV27's final independent confirmation,
full-SHA CI, exact-head Mac DEC-025 comparison (including the disclosed isolated
missing-suite recovery), and GEN-8 are recorded in
IMPLEMENTATION/RESUME_RECORDS_MERGE/RECORD.md. No numerical repair is accepted.
AUD-T3-01 remains BLOCKING, F2a held and K6c unaccepted. Original raw evidence
remains preserved; no pruning is authorized. The owner stopped the response drift.

## A1 first compile: bounded matcher repair (ROOT, 2026-10-01)

The B compile stopped before tests on two new-test type-inference errors and
one exhaustive existing test matcher. ROOT grants only the diagnostic
PublicationEnclosure arm in FK/tests/retained_k4/method_tests.rs as a fifth
maintained path, plus explicit Wide::<4> in the new tests. Existing expected
tokens/fixtures/assertions remain protected. The committed B compile addendum
sets the exact scope; this is no availability or numerical acceptance.

## A1 compiled-source checkpoint and reach (ROOT, 2026-10-01)

ROOT froze the exact compiler repair at dd1f70d8ba85b19f7d948bca6ee08a44bbb12ae1.
I22 implementation_b_fix_01 records the preceding successful focused tests on
3cf296e plus that exact patch; they are not falsely rebound to a later launch.
Full numerical/runtime/mutant acceptance is still pending. The C brief grants
finite source-control and new-test work, plus RV29's independent fixed-head FK
suite. No old expected outcome or protected availability is changed.

The actual caller trace, independently checked in RV29 candidate_01, reaches
H/validation only. The legacy product solver uses unchanged retained arithmetic
and seeded hooks, not the changed adaptive route. Accordingly T9/both-entry
are not triggered by this slice alone; all other standing exact-candidate gates
remain. Reassess if the source/write set gains product reach.

## Independent ledger span correction (ROOT, 2026-10-01)

RV29's candidate ledger understated the successful b=2 algebra route's aggregate
max_span_bits by one. ROOT and the manager independently checked the frozen
source: the comparison term span is 1-(-1074)+1=1076, while H alone spans1075.
The priced work and budgets do not change. The original sealed ledger stays
unchanged; RV29 provides an additive correction. The C span brief permits only
the warranted new-test expectation/rationale repair before the next bounded
run. This is no production defect closure or blanket permission to rebaseline.

## A1 S11 inventory: declare the added integer site (ROOT, 2026-10-01)

The full fixed-head FK suite found one source-inventory mismatch in addition
to the two legacy work-golden assertions. ROOT and RV29 classified the added
run_schedule c+=1 as the existing integer precision-schedule index advancing
after certificate rejection. The exact S11 inventory row may change5 to6 with
that annotation; all scanner rules/other rows remain. The dedicated committed
brief bounds the edit/check and same-reviewer backcheck. No floating summation
site or protected numerical criterion is excused.

## A1 checkpoint D: preserve old work oracles and close test gaps (ROOT, 2026-10-01)

The complete source-run/review evidence now warrants the exact test-only D grant
in RESUME_2026-09-30/BRIEFS/A1_CHECKPOINT_D.md. I22 may isolate the new certificate
component in the two named existing work-golden tests while keeping every old
numerical literal and unrelated criterion. This supersedes the earlier optional
evidence-patch-only step; the actual diff and same-reviewer backcheck remain
required. The independently checked relative-control fixture is accepted as
unit-level arithmetic truth, not a source-model result. No production change,
old fixture rewrite, new public formula or availability relaxation is selected.
A1 and K6c retain their separate closure boundaries under the owner's sequence.

## A1 VR records: include certificate work (ROOT, 2026-10-01)

The unchanged kernel-lane generator on immutable40129 produces only the five
accounting-field deltas documented in RESUME_2026-09-30/verification/
vr_record_refresh_01/FIELD_DIFF.json on A1. ROOT checked all field comparisons
and charge closure. Accuracy, outcomes, precision, classes, controls and
not-covered expectations stay fixed; invariance/parity records are identical.
The ten generated family JSONs and kernel_lane/SHA256SUMS may be refreshed to
include the new certificate work. No other observation or criterion is granted.
The failed preflight and original hashes remain evidence. Same-reviewer
backcheck and an affected-suite rerun are required; this is not A1 closure.

## A1 historical fault mappings selected for bounded replay (ROOT, 2026-10-01)

ROOT read I23's exact patches and RV29 protected_06. The precise selected subset
and runtime fences are in RESUME_2026-09-30/BRIEFS/A1_G1_PROTECTED.md. Reconstructed
A2 patches retain original IDs without claiming missing original bytes. D4u's
current underflow-only variant preserves both old overflow assertions and must
reach the old underflow criterion. This selects a faithful finite discriminator,
not a weakened expectation or an executed kill.

The historically optional RV23C-N1 cache probe remains optional: independent
source comparison confirms its failure path unchanged. R7 derivation/retired
and RV23-N4 reachable-path qualifications remain their sourced dispositions.
All other unreviewed or masked mappings remain open; the new certificate's mere
refusal is not evidence that an intended old numerical assertion was exercised.
No design contract, availability claim or A1 closure is accepted here.

## G1 host-tool stop; owner direction required (ROOT, 2026-10-01)

G1 passed all selected unmutated baselines, then stopped before any mutant
execution. The existing system patch command reported a system temporary-file
permission error while returning exit0. The mandatory postimage check found
the original source unchanged and prevented continuation. Exact command, raw
hashes and expected/actual source hashes are in
RESUME_2026-09-30/I22/protected_g1/runtime_01 on A1. No kill or survivor is claimed.

The owner's explicit instruction to stop and report tool blockers governs.
No TMPDIR or permission change, retry, alternative patch mechanism or host-tool
development was performed. ROOT requested direction for a bounded trial of
the existing apply_patch tool on the already frozen G01 patch, with no host
configuration change. That question is pending; no approval is assumed.
Further experiment work is held. Existing source-only returns and evidence
preservation may finish, with no new host jobs.

E1/E2's new-publication variants have independent RV29 runtime confirmation,
and the full FK and H/VR preflights passed. This is not A1 merge acceptance:
historical fault work and final-head gates remain, draft PR1070 stays unmerged,
F2a is held, and K6c has no accepted complete E_max.

## Owner resumption and bounded patch retry succeeded (ROOT, 2026-10-01)

The owner directed: “Please resume and carry on as you intended.” ROOT stated
and acted on this as authorization for the previously requested ten-minute
existing-apply_patch retry. G01's exact frozen postimage, intended Phi assertion
failure and both untouched controls passed their required checks; evidence is
RESUME_2026-09-30/verification/tool_resume_01 on A1. No host configuration,
permission, environment or tool development occurred. Prior failed evidence
remains sealed. The operational hold is lifted prospectively for bounded work;
A1 remains BLOCKING pending final verification/merge, F2a held, K6c unaccepted.

## K6c narrow layout bindings independently checked (ROOT, 2026-10-01)

ROOT read I21 layout08_run and RV30 layout08_03/backcheck_run and verified
their seals. The actual three kernel node-request pairs and five named type
size/alignment facts are usable in the conditional kernel ledger on their
exact immutable40129/Rust1.97.1/aarch64 release basis. RV30 independently checked
the complete raw samples, source/build/binary and no-extra-allocation/drop
conditions. Source_07's queue recurrence is also independently checked.

This is deliberately narrow: no private node alignment is inferred, the
process subtraction baseline is not an allowance, and no complete E_max,
admission or W1 acceptance follows. C1/C2/O2/W1 remain open. No allocator,
observer, guard, dependency or maintained code was changed. Later final-source
reconciliation and full-bound review remain required before reliance.

## K6c H caller composition reviewed conditionally (ROOT, 2026-10-01)

ROOT read I21 source_10 and RV30 h_caller_05 and verified both seals. The
finite application-owner composition, grammar and sample-edge distinctions
are usable as conditional terms. RV30 found no blocking defect in that scope.
Adopt its N1 geometry-leaf narrowing and N2 registered-allocator boundary:
the remaining floating-format premise does not require a general geometry
error audit, and private Thread layouts are not a K6 requested-byte obligation.
The original source_10 packet remains unchanged; the review is additive.

H-R0 is only surviving registered owners and later reached growth on the finite
main-thread path; H-R1 is the selected stdout mutex child; H-R2 is the directly
reached byte-write/flush path and dropped errors; H-F0 is finite default-f64
Debug. Planned path/model inputs and checked final implementation remain to
bind. These are precise open premises, not free estimator inputs or measured
allowances. No full E_max, prelaunch admission, W1 or F2a acceptance follows.
VR's finite caller composition continues separately under its source-only brief.

The source_10 and h_caller_05 records cited here are on the separately preserved
K6c branch at `79ab5428470ca747a484ea145ea979e5bb965c96`, under
RESUME_2026-09-30/I21/source_10 and source_review_RV30/h_caller_05.
Their presence on that branch is not a merged K6c implementation.

## A1 completed verification subsets and exact-tail correction (ROOT, 2026-10-01)

ROOT read and verified RV29 g1_final_12, g2_runtime_13 and g4_runtime_14.
Those selected subsets are closed with their precise assertion/control and
source/build qualifications. Counts and exceptions remain at their REVIEW
sections; the extra G01 baseline and all uncredited operational attempts are
preserved. This is not complete historical coverage or A1 acceptance.

G3's R09/R10 whole-loop attempts stopped at earlier corpus rows and receive
no credit for the explicitly required witnesses. Their original controls were
subsequently run under separate grants. The additive diagnostics retain the
same rows, original upper expectations, production faults and original test
prefix. RV29 tail_sources_15 corrected both I23's and its own earlier prediction
arithmetic because ExactWideSum.used includes spare zero limbs. Correction11
and tail_correction_16 preserve those errors and change only the predicted
fault behavior/eligibility: R09 yields +8p0 directly; R10 yields Z+.
No protected unmutated expectation, criterion or original corpus changed.

The original G3 window was closed at its quiescent R14 boundary. ROOT selected
the bounded reuse of existing source checks before Cargo, retaining postchecks,
then independently reviewed that exact helper delta and preparation seals.
Each remaining copy still needs its own manager-issued conditional release
and an immediate hard source/binding check before build. Sequential semantic
inspection, controls and stop rules remain. No new host guard/tool was built.
Fresh runtime grants are in A1_TAIL_RUNTIME_AND_G3_REMAINDER.md; old deadlines
remain historical, and no automatic extension is granted.

## K6c direct-path proof and artifact request facts (ROOT, 2026-10-01)

ROOT read source13, metric_design_02, I23 private_request_artifacts_08 and
RV30 h_request_bindings_10. H12-F1's public Mutex/Once forwarding gap is closed
by the additive source proof. The two allocation arguments are usable only on
the review's exact layout08 executable basis: the named pal mutex and first
ThreadInfo registry leaf. See h_request_bindings_10 RETURN sections1–2 for
values, units, source attribution and native instruction evidence. They were
not inferred from the observed startup baseline or a mirror type.

The library archive-reader incompatibility was preserved and that lookup
stopped; the separately authorized existing executable supplied the evidence.
No runtime witness, library rebuild or tool replacement occurred. Final-H
specialization/build correspondence remains mandatory; this does not supply
a final-H or whole-process E_max bound. The direct witness proposal remains
unrun and unselected because existing executable evidence was available.

VR caller identities/schema conclusions remain conditional on their stated
construction/configuration premises. RV30 vr_caller_07 and serde_binding_08
plus external_inputs_06 supply the checked source, package-byte and input
bindings without a fresh canonical replay. V-EXACT derivation is separately
under review. Complete caller composition, checked implementation, final A1,
admission replay and measurements remain open; W1/F2a are not released.
These K6c packets are preserved at branch commit
8d80dbdb326514e167ccf16b8e173bd91c7441f4 under RESUME_2026-09-30.


## A1 named-witness continuity and K6c finite callers (ROOT, 2026-10-01)

ROOT read RV29 g3_prefix_tail_17 and the later preemption_18/r33_source_19
reviews. G3's prefix and corrected tail diagnostics are independently closed
as recorded in g3_prefix_tail_17; original earlier-case failures retain no
named-witness credit. Runtime04's R33 failure occurs on N01 before its named
N05/N06 witness, and remains uncredited. ROOT's runtime05 disposition permits
its unchanged original control and the remaining original schedule while an
additive N05/p128 diagnostic preserves the same helper and fixed oracle.
The exact conditional runtime grant is BRIEFS/A1_R33_NAMED_RUNTIME.md.
R51's source-only contingency is not an observed failure and stays unrun if
its original test qualifies. No numerical criterion or source expectation is
weakened; A1 remains open and F2a held until complete verification and merge.

ROOT read I21 source14 and RV30 finite_callers_12 and verified their seals.
The fixed-input floor inequalities and finite-from_f64 premise on the actual
vk_scale caller roster are usable conditional facts, with scope and figures
at finite_callers_12/RETURN.md sections “Actual basis and integer input check”
and “Actual observation routes and remaining138 rows”. The broader lane's
selected-numerator obligations remain open. A separate bounded numerical
consequence derivation has been assigned; finite tags and old passing runs
are not substitutes. No whole-process E_max or admission is accepted.

The K6c proposal and review are preserved on its branch at
4ee34724be3d4895016ba7ce9909f5e31a4770be. K6C's maintained adaptive.rs still
matches its own older committed KF3 source (Git blob5448ca262ab0346c138052e39b9a205b82739842),
with no working-tree modification. Its difference from the explicitly frozen
A1 basis is the not-yet-integrated A1 correction, not an unexpected source edit.
RV30 correctly reviewed the separately preserved full-hash-matched40129 bytes.
Final A1/build reconciliation remains required before implementation reliance.


## R46 historical-filter correction (ROOT, 2026-10-01)

ROOT read the original K4 review D.4, its RV19-D2 delta log and current source,
then I23 r46_diagnosis_14 and RV29 r46_diagnosis_22. The frozen fault is correct,
active and non-equivalent. ROOT's recovered focused-filter selection was wrong:
that focused unit test also passed in the historical run. The actual historical
kill was the unchanged classification-vector test's set19 equality. Restore
that existing discriminator under BRIEFS/A1_R46_MAPPING_DISPOSITION.md; no new
input, source, expected value or test is selected. The original survivor and
RV29's earlier mapping endorsement remain sealed; the diagnosis supersedes
only that endorsement. No kill is credited until the corrected run and controls
are independently checked. This is a recovery-mapping error, not evidence of
a new production accuracy defect. R33's separate named diagnostic may proceed
under its bounded grant; final G3/V-K/A1 acceptance remains open.

## K6c bounded numerator consequence and sparse correction (ROOT, 2026-10-01)

ROOT read the complete selected-numerator derivation and RV28's independent
selected_numerators_01/REVIEW.md, then verified both seals. Accept the bounded
source consequence on its exact fixed roster, immutable40129 source and inherited
accepted R7 premises. The review's sections “Full retained T/R implication” and
“Actual J-end recovery and rounded quotient” supply the warrant; “Every-row
result” supplies the exact arithmetic and figure basis. No successful selection
is promised. This closes the named remaining observation-finiteness premise;
changed inputs, recovery, row mapping or gates require reassessment.

ROOT also read sparse_13, metric_design_07_sparse_correction and RV30's
sparse_correction_14 return and verified their seals. SP13-F1 is closed by the
additive HFactor validation-owner correction. Reuse the original sparse proposal
only together with that correction and its review. The review demonstrates no
fixed-roster or composed-envelope undercount; its remaining private-node,
profile, formatting/runtime and final-build cells remain open.

These records are preserved on K6c at a900bcc69bbbc949214ff0be23bc073941332089.
They grant no complete E_max, implementation, admission, W1 or F2a acceptance.
ROOT selected one bounded canonical K0 assembly after the active serializer
review, through H's existing library and distinct H/VR caller terms. No contract
alternative from metric_design_06_closure_assessment is selected. The assembly
must expose exact residual cells and their numerical impact before any further
proof work is commissioned; BRIEFS/I21_K0_CANONICAL_ASSEMBLY.md governs.


## G3 execution complete; portable evidence placement (ROOT, 2026-10-01)

ROOT read and verified the runtime06 child/manager returns. Every remaining
G3 registration now has its required runtime evidence; independent aggregate
review is still pending. The earlier R33 and R09/R10 wrong-witness attempts
and focused R46 survivor retain no credit. Runtime06 preserves the original
window and ROOT's explicit prospective bounded continuation; no automatic
extension is inferred. R51's original named witness qualified, so its prepared
contingency remains unrun. V-K proceeds under its separate conditional grant.

The hosted interim head4972f4ef failed GEN-8 on machine-path provenance in
unclassified review JSON. ROOT read the failure, existing classifier and prior
relocation precedent; I23 independently diagnosed the records-placement cause.
Thirteen complete sealed review packets moved into source_review_RV29/
_run_records with every payload and manifest byte unchanged, recorded in
R/REVIEW_EVIDENCE_RELOCATION_2026-10-01.json on A1 at9ef9508dea.
[Correction to physical-path references in earlier records: apply that ledger's
old/new packet-prefix map for current replay. Pinned historical commits retain
original locations; internal packet-relative manifests remain unchanged.]
No checker, policy, source, oracle or numerical criterion changed. The unchanged
check passes on clean9ef9508dea; verification/gen8_portability_04 preserves the
result. Final-head GEN-8 and all other merge gates remain required.

ROOT read RV30 serializer_15 and verified its seal. Its finite schema/owner
recurrences are usable only with the named unresolved terms and the review's
refusal-argument/Phase.fields clarifications. The single K0 assembly actually
started17:51:15 UTC with a fixed18:21:15 end; manager/k0_assembly_16 records the
native dispatch. No further proof programme, estimator implementation, contract
alternative, E_max, admission or W1 acceptance is granted by that assignment.


## G3 independently closed (ROOT, 2026-10-01)

ROOT read g3_final_25/REVIEW.md and verified the whole-packet seal
 ea44ed6892482a455382ed9337697e92a3d553baa6c270bf014d66f24022ae08.
Accept its bounded G3 closure, including the exact registration/control
reconciliation, corrected historical R46 mapping, original qualified R51 and
both R52 predicates. Figures and qualifications are at its registration table
and full-command reconciliation; prior wrong-witness failures and the focused
survivor remain uncredited. RV29 also independently verified the complete
byte/mode/blob-preserving evidence relocation. The records are committed on
A1 at ab2d4ce9522d317a2310ca5545a719a8f8423367.

V-K remains active under its own bounded grant. This ruling supplies no final
A1, F2a, E_max or W1 acceptance; exact-candidate review and all final gates remain.


## Session-limit recovery checkpoint (ROOT, 2026-10-01)

The owner warned of an imminent session limit. ROOT is preserving the ongoing
authorized work and quiescent handbacks; this is not completion or a new owner-
approval hold. RESUME_2026-09-30/SESSION_LIMIT_HANDOFF_2026-10-01.md records exact
source/evidence reuse, branch/host recovery, open F03 contribution attribution,
remaining V-K/final A1 gates and unreviewed K6c proposals. No new experiment is
launched by this checkpoint. All prior numerical and acceptance holds remain.

ROOT read and verified RV29 vk_f01_runtime_28. Accept its separate release F01
numerical-unavailability witness within the stated scope; original debugP09
remains unqualified. P10's later control and F02 evidence await their own review.
F03's complete two-family observation restores controls but returns an explicit
contribution-attribution gap; it receives no fault credit at this checkpoint.
No generic Ceiling, Pivot location or seed name substitutes for its criterion.

K6c's latest prefix, formatting and eight artifact-specific node-pair returns
are preserved as proposals awaiting independent review. No complete E_max,
checkpoint0, implementation, admission, W1 or F2a acceptance is granted.


## Owner resumed T3; F02 and actual VR node facts (ROOT, 2026-10-01)

The owner explicitly directed “You may resume now.” ROOT verified the saved
M5/guard/process/Git state and restored bounded native work. Main546e05a was
already an ancestor of numerics, so no redundant merge commit or rebase was
created. RESUME_2026-09-30/RESUME_2026-10-01_1934.json binds entry checks and
supplied instruction origins. Old runtime deadlines remain historical; new
source/review grants are in the resumption briefs.

ROOT read and verified RV29 vk_mid_30. Accept F02's exact registered value
witness and P10/P12 controls within that review's scope. Its full F03 provenance,
controls and sequences are verified, but the source/soft-root consequence still
awaits independent review; P09/P13 retain unqualified status. The original
criterion does not impose an unrecorded universal reconstruction of every private
omitted term, nor permit a generic failure/enum to replace the named warrant.

ROOT read and verified RV30 vr_nodes_17. The eight actual leaf/internal pairs
are usable only on their separately bound release/debug artifact bases; see
its result table and “Build provenance and remaining limits” for units and
qualifications. Real source representation and typed allocation argument flows
supply those facts, not mirror layouts or equal-size type substitutions. Final
ordinary-production vk_scale correspondence and actual populations remain open.
The prefix and formatting proposals remain under independent review, with no
complete E_max, checkpoint0, implementation or admission acceptance.


## F03 attribution closed; honest runtime-plan supersession (ROOT, 2026-10-01)

ROOT read the complete designer source consequence and RV29
vk_f03_attribution_31/REVIEW.md, verified the seals and accepted its separate
release F03 outcome witness for BOTH registered families. Exact root-minimum
and material-decrease checks, CHAIN formed Rayleigh argument, SKEW linkage,
zero/tie qualifications and full runtime/control basis are at those sources.
The original whole-mutant criterion does not require a root-only counterfactual
or disclosure of every private omitted term. It still requires the proved
material soft-root link; generic Ceiling/Pivot/seed labels were not substituted.

P09/P13 remain unqualified debug attempts. Supersede P15 explicitly as UNRUN:
its RF-SKEW registration is covered by the independently qualified release
witness. P14 retains its actual debug rf_chain returned-control role. P16 runs
as a fresh debug rf_skew NONE baseline, never a return for unrun P15. Then
P17–P53 retain original order, predicates and controls under the fresh bounded
BRIEFS/A1_VK_REMAINDER_03.md grant. All old schedule/evidence bytes remain.
This does not close the remaining V-K matrix or final A1 gates.

## Prefix receiver correction and wrapped-error facts (ROOT, 2026-10-01)

ROOT read RV28's full prefix review, the additive correction09 and its same-
reviewer backcheck. [Correction (ROOT, 2026-10-01): the full backcheck read and
ROOT payload-seal verification occurred at19:55:57 UTC, immediately after the
original19:55:40 entry; that entry initially relied on the reviewer's sealed
handback summary. The subsequent full check agrees, with no changed verdict.]
RV28-PREFIX-1 is closed at source-design level: the fresh
free-DOF receiver and layout construction/growth coexist at the stated Counts
initializer edges, and all receiver temporaries die before the cut. Reuse
packet08 only with correction09 and the backcheck. The existing successful
single-main entry proof supplies its pre-args registered peak; final VR
correspondence and stated failed-startup/foreign/panic interfaces stay explicit.

ROOT read RV30 format_stream_18 and wrapped_errors_19 and verified their seals.
The finite formatter/hash/stream topology and the three actual request facts
are usable only within their reviewed source/artifact scopes. The conditional
wrapped-error substitution and units are in wrapped_errors_19's “Direct caller
and topology distinction”; it is not a process peak or final vk_scale fact.
Static-message pools remain under separate independent review. Complete K0/H/VR
numbers, checked implementation, final-build binding, admission and W1 remain
unaccepted; the next H numeric task is source arithmetic, not a new tool.


## V-K F17 criterion gap and conditional numeric candidates (ROOT, 2026-10-01)

ROOT read the full runtime05 child/manager returns and RV29 vk_completed_34
review, verified every manifest payload, and preserved them in A1 commit
1453a780bd. Accept only the reviewed P14/P16/P20–P26 scope and its distinctions:
P16 is a fresh baseline; P20 restores F04; F08/F10/F13 reach their registered
bitwise, canonical-byte and numerical predicates with actual returned controls.
All source figures and qualifications remain at that review's table.

Runtime05/RETURN.md records P28's successful original RF-WEAK return and P29's
missing RF-SKEW CLASS witness. Preserve the stop: P30–P53 are unrun. Neither
Ceiling nor FLOOR differences qualify for F17's protected CLASS criterion.
A source diagnosis and a separately reviewed, bounded existing-record diagnostic
are being prepared. They grant no acceptance amendment. Any measured conflict
requiring a protected-criterion change returns to the owner under project
AGENTS.md, Software checks. A1 acceptance/merge and F2a reliance remain held.

ROOT read manager/h_numeric_19/RETURN.md and the VR caller candidate's full
RETURN, SUMMARY and METHOD_AND_JOIN. Their source-arithmetic candidates are
preserved, not accepted E_max: H is under RV30 independent review; VR is under
RV28 review with five explicit kernel joins still outstanding. Numeric maxima,
units and conditional artifact/owner qualifications are at those source tables.
The binary half-heap check and calibrated runner admission are separate; no
chronological admission, required measurement or final production correspondence
is inferred. No supporting tool development is authorized.

Resumption check at20:38:18 UTC: actual M5 Max,18 cores,137438953472 bytes,
existing guard5387, no cargo/rustc process. Fresh origin/main remains546e05a159.
The remote PR1070 interim head2e1adc4015 has passing completed checks; that is
not final-head review/dispatch/DEC-025/GEN-8 closure. Maintained A1 code remains
at the reviewed40129 basis. ROOT retains all Git/index authority.


## Conditional H arithmetic verified; VR phase corrections pending (ROOT, 2026-10-01)

ROOT read and verified the full RV30 h_numeric_23/RETURN.md and h_summary_24
backcheck, with the additive h_numeric_summary_correction_01. Accept the H
numeric19 source/owner/arithmetic candidate conditionally on its stated ordinary-
production source/type/library/request-site/entry assumptions. H23-F1 and H23-N1
are closed. Exact byte figures and comparison bases are at h_numeric_23's
“Numerical result and historical boundary”; the MiB display is truncated and
not an outward-rounded upper. Actual Python floating products and both integer
conversions govern the projected-RSS summary, not an exact-real replacement.

This closes the conditional H arithmetic review only. Final artifact/profile
correspondence, maintained checked estimator, same-binary validation, corrected
chronological admission/ascent/RSS replay and W1-T4 measurements remain. No
complete E_max, K6c checkpoint0, run admission, W1 limit or F2a acceptance follows.

ROOT read and verified RV28 vr_numbers_06/REVIEW.md. Its two SHOULD-FIX findings
remain pending a bounded additive correction and same-reviewer backcheck:
retained failures during expected-list initialization and persistent typed LIST
during nonselected diagnostics. The review's “Exact numerical consequence”
proves both repairs are dominated by the existing common-outcome maximum on
the fixed roster; that does not make the original named phase addends complete.
The VR kernel join stays unreleased until those corrections are verified.


## Corrected VR caller interfaces accepted for bounded join (ROOT, 2026-10-01)

ROOT read the full metric_design_11_vr_correction/CORRECTION.md and RV28
vr_correction_07/BACKCHECK.md, verified their seals and all payloads, and accepts
the original caller candidate only together with the authenticated overlay.
RV28-VRNUM-1/-2 are closed. The corrected interfaces retain prior failure owners
and the initialized typed LIST at the reviewed phases; the common-outcome
maximum is unchanged on the fixed roster. Exact replacements, units and limits
are in that backcheck. This grants no implicit kernel value or complete E_max.

Release BRIEFS/I21_VR_KERNEL_JOIN.md for one bounded numeric composition and
like-for-like historical peak comparison. Ordinary-production correspondence,
checked implementation/deduplication, admission replay and required measurements
remain. A1's F17 owner-decision hold is unchanged and independently managed.


## Owner adopted the scoped F17 validation amendment (ROOT, 2026-10-01)

The owner answered “Approve the scoped F17 amendment (Recommended)” to the
explicit question recorded in RESUME_2026-09-30/OWNER_DECISION_F17_2026-10-01.md:
accept independently verified certificate rejection when A1 prevents publication,
preserve normal accuracy/class checks and reachable CLASS witnesses, and permit
the same rule for RF-CANCEL only after its own controlled run and independent
review. This is an actual owner decision, not inferred from elapsed time.

Adopt the exact independently reviewed scope in RV29
vk_f17_runtime_36/OWNER_DECISION_TEXT.md (review seal
a779da2a62c1fce29c0005e6e945ce785f1f76ea6e56d0b1303436bc5c284070;
A1 evidence commit80c35e37466435e408268eb19e037e56f5a787cf).
ROOT read the full review/proposal, verified every payload and accepts the
controlled RF-SKEW release result as certificate prevention after standing R7,
within its exact source/case/control scope. P30's actual debug return is closed.
P29 remains permanently unqualified with zero CLASS credit. The protected twist
rows were not observed: an earlier different force row prevented whole-case
publication. No per-row class witness is invented.

The original unmutated correspondence/exception lists, references, tolerances,
class checks and stored records remain. Actual CLASS evidence remains required
on registered not-covered comparisons outside prescribed exceptions wherever
publication reaches them. Only source-linked PublicRelative, SharperExact or
SharperBinary64 rejection in the reviewed relative branch after R7 can qualify
the alternative. Generic Ceiling, FLOOR/work/record drift, missing publication,
AbsoluteBound, malformed/terminal/budget/arithmetic/verification failure or
another fault receives no substitute credit. Unexplained attribution/control
failure remains a hold. Any changed source/corpus reopens affected warrants.

The same rule is prospective only for the remaining originally registered
F17 RF-CANCEL family on this unchanged A1 source. It grants no unrun credit,
no certificate bypass and no other fault amendment. ROOT separately releases
the prepared remaining original schedule. Complete V-K/A1, final-head gates,
merge, E_max, W1 and F2a acceptance remain open.


## RF-CANCEL F17 prevention qualified; remaining V-K resumed (ROOT, 2026-10-01)

ROOT read the complete RV29 vk_f17_cancel_37/REVIEW.md, verified its seal
2201dba8973e050e1da1e4dfd0049332ae494d51ecc5353a73c3eb546c7431ee
and every payload, and accepts its separate controlled RF-CANCEL observation
under the owner's already adopted F17-only certificate-prevention rule. The
report's “Actual release observation” and “Protected quantity distinctions”
state the exact case/attempt figures and what was and was not observed.
This applies the existing owner decision, without another amendment.

P31 remains permanently unqualified with zero CLASS credit. P32's actual debug
return is closed. The earlier different displacement row prevented whole-case
publication; no protected displacement/station class, H or later predicate was
observed. Normal frozen records and all original correspondence checks restore.
Together with the earlier qualified RF-WEAK CLASS witness and RF-SKEW prevention,
the registered F17 routes have their scoped dispositions and controls. No other
fault criterion is changed and no final V-K/A1 acceptance follows.

Close the stopped runtime06 and diagnostic windows. Release the remaining
original P33-P53 schedule only under BRIEFS/A1_VK_REMAINDER_07.md's fresh bound.
Final full candidate review and exact-head CI/dispatch/DEC-025/GEN-8 still apply;
AUD-T3-01 and F2a reliance remain held until actual A1 closure.


## Conditional VR composition and reference-profile interface (ROOT, 2026-10-01)

ROOT read the complete RV30 vr_join_25/RETURN.md, verified all payloads and
accepts the fixed-roster VR source/owner/numeric composition conditionally on
its stated profile, input and launch premises. The report's “Numerical and
historical result” supplies exact figures, units, comparison windows and
artifact qualifications. No observed peak supplies an arithmetic term. This
completes the conditional H/VR numeric candidates for their selected rosters,
not final E_max, implementation, artifact qualification or admission replay.

ROOT also read and verified the full metric_design_12_profile_binding proposal
and independent RV28 profile_binding_08 review. Select named immutable
ReferenceKernel/H/VRProfile premises with the existing external exact archive/
build/launch qualification. The earlier Verified-current-build factory was an
unaccepted sketch, not an adopted universal direct-CLI refusal policy. This
selection changes no protected test, ordinary CLI/CI behavior, admission
predicate, metric/window or solver contract and introduces no host tool.
Reference identities index conditional premises; they do not attest a current
executable or transfer M5/debug/seeded facts to Linux or ordinary production.
Unknown/mismatched artifacts receive no qualified reliance or measurement grant
at the external gate. No automatic in-process enforcement is claimed.

Implementation must keep each fact bundle and kernel/caller composition
coherent, reject missing/invalid descriptors rather than fill them with zero,
and preserve all existing executable, allocator and public-type checks.
ROOT's source read confirms VR's original scale test requires storage identities
and estimate ordering on its complete factored CI roster. Before implementation
release, a bounded adapter compatibility check will establish how the full
existing interface obtains its required descriptors without silently extending
the fixed-roster proof, keeping a stale estimate port or weakening that test.
The remaining qualification, checked implementation, admission and measurement
obligations are unchanged; no K0 implementation release is implied here.

## Required S1 survivor in the remaining A1 matrix (ROOT, 2026-10-01)

ROOT read and verified runtime07 child/manager returns. P33's exact VK-S1
rf_finite test survived and supplies no registered numerical witness. P34-P53
are unrun. The old runtime window is closed; no automatic retry, alternate
filter or later call follows. The F17 decision supplies no S1 substitute.
I23 has a bounded source-only diagnosis under the selected software-defect-
diagnosis skill; its return and independent review precede further action.
A1 acceptance/merge and F2a reliance remain held. No production fault is inferred
merely from the surviving test.


## S1 source mapping corrected, original numerical criterion retained (ROOT, 2026-10-01)

ROOT read and verified I23 diagnosis21 and RV29 vk_s1_mapping_38. The observed
P33 survivor is explained by the empty spring arrays in RF-FINITE, which make
the S1 site unreachable. The original maintained register required directional-
spring VALUE evidence and ran the broad suite; our later frozen A1 mapping
introduced the incorrect rf_finite restriction. Prior preparation/review missed
that source reach. Preserve that attribution and the original records.

Accept the independently reviewed source-ready correction to the existing
rf_skew test, which supplies a loaded nonunit direction and unchanged numerical
oracles. The review's source and exact fixture checks establish eligibility,
not a runtime kill. This changes no protected numerical criterion and imports
none of the F17 amendment. P33 remains an unqualified survivor. The separate
BRIEFS/A1_VK_S1_RETARGET.md grant permits only its actual P34 return and controlled
replacement triplet; later calls and A1 acceptance remain held pending results.


## S1 corrected VALUE witness qualified (ROOT, 2026-10-01)

ROOT read the complete RV29 vk_s1_result_39/REVIEW.md and verified seal
baee8b1fceccaad51c7ee7a322df4dabc096328ccb9ac6f459b6a95b54f3f6d5
and every payload. Accept the separately controlled rf_skew S1 observation
under its unchanged original numerical VALUE criterion. The report's “Every
witness checked” supplies exact counts, case/key/reference attribution and
source linkage. P34 and both new normal controls restore every original check.

P33 remains a zero-credit survivor caused by our earlier spring-free mapping;
no source/test/oracle/criterion or frozen historical record changed. No F17 or
nonpublication substitute is used. Close the S1 retarget window and release
only original P35-P53 under the fresh BRIEFS/A1_VK_REMAINDER_08.md grant.
Complete V-K/A1 and all final-head gates remain open.


## Complete-context API and five-field contract selected (ROOT, 2026-10-01)

ROOT read the full metric_design_13_api_contract contract and independent RV28
api_contract_09 review, verified all seals/payloads, and selects this conditional
interface design. Capture checked source facts before the existing source drop;
obtain input-history facts while the existing raw/typed owners are available;
keep coherent immutable reference profiles and external executable qualification.
The actual CLI remains RF-LARGE-only. The unchanged broader storage/order test
uses an explicit mathematical single-family reference invocation, not an asserted
CLI execution or bound on its load_all process.

Adopt the reviewed named model/fixed/decide/full/selected128 meanings and owner-
subset ordering proof. No +1, forced ordering, stale port, partial global value,
missing-cell zero, fixture exclusion or unqualified current-build assertion is
permitted. Every original test assertion and ordinary CLI/admission rule remains.
The review's six instantiation/translation cells remain real prerequisites; this
is not full adapter implementation or complete E_max acceptance.

Commission only bounded reuse/instantiation of existing generic equations and
fixed inputs: kernel terms on the required reference roster, each family's
caller/input composition and result mappings, and the reach-dependent triple-key
set premise. No new host tool or generic library proof programme is commissioned.
Unknown concrete facts must be returned by name; code translation and any added
allocation costs still require their own explicit later grant and review.


## S2 mapping corrected; remaining observability limits preserved (ROOT, 2026-10-01)

ROOT read the complete I23 remaining-map audit and RV29 vk_s2_mapping_40 review,
verified every payload, and accepts source readiness of the correction to
existing rf_finite. The review's “S2 cause and original criterion” proves the
zero-reference sign invariance and separate spring-action branch; its cited
unchanged finite-family references establish eligibility, not a predicted kill.
The error was our later frozen narrowing of the original reaction/VALUE register.
P35 remains an unqualified survivor. The failed-to-dispatch P36 preparation is
preserved as a manager timing overrun, not a test result or host-tool defect.

The fresh BRIEFS/A1_VK_S2_RETARGET.md window includes the actual P36 control and
controlled corrected witness. No protected criterion is weakened. Remaining
F05/F06/F07/R28 mappings have their existing specific observables; R02's schema
does not expose the numerical estimate/charge values. Only an attributable
existing named R7/acceptance change could qualify that current observation;
work-only or certificate-only drift cannot. A missing witness remains a stop,
not permission for new instrumentation or a waiver. All final A1 gates remain.


## S2 corrected VALUE witness qualified; remaining A1 validation (ROOT, 2026-10-01)

ROOT read the full RV29 vk_s2_result_41/REVIEW.md, verified all payloads and
seal97cb6c7a3542616af3e9a86ca8e43d5e4c85e94608ae1c71fccabd5436c7bd6f,
and accepts the corrected S2 observation under the unchanged nonzero-reaction
VALUE criterion. The review's Actual numerical witness and Controls and history
sections state the exact case/key/reference attribution and actual returned
controls. P35 remains permanently zero-credit; its actual P36 return is closed.
The expired earlier preparation remains history, not an execution or authority.

Close the S2 runtime window. Release original P37-P47 only under the fresh
BRIEFS/A1_VK_REMAINDER_09.md grant. R02's unprinted numerical-field gap remains;
existing scalar and TREE100 tests are bounded investigation inputs, not P49
credit or an amended criterion. P48-P53 remain held pending ROOT's next grant.
No source, oracle, protected criterion or F17 scope changes. A1 acceptance,
final-head gates, merge and F2a reliance remain open.

The sealed caller14 and kernel_reference22 K6c instantiations have been preserved
for independent reviews RV28 reference_callers_10 and RV30 kernel_reference_27.
These are conditional source-arithmetic candidates; composition, result ordering,
context/code translation, artifact qualification, admission and measurements
remain. They do not block A1's independent completion path.


## Conditional generic K6c components verified (ROOT, 2026-10-01)

ROOT read the complete RV30 kernel_reference_27/RETURN.md and RV28
reference_callers_10/REVIEW.md and verified all payloads under seals
2e0e0cf05cea3c11ea2a7b6eb4d26b0b5afb3886b0a1eae1e71bc816ecef6c82 and
c3eadadd67f038cbb4e56a10cf9cf89a168bea8e2ae4a554646f484a5cf930b7.
Accept their independently verified generic kernel and caller instantiations
conditionally on the explicitly named source, input, request-profile and launch
premises. The reports give full roster/metric figures and their scoped units;
no measured heap supplied an arithmetic term.

Reference B/b substitutions are tagged monotone population uppers, not graph
facts. Original stored z/h and actual generic spring/load/string terms remain.
Mathematical SingleCaseFamilyReferenceV1 and actual CLI24 retain their distinct
input/launch/count policies; equal case names do not identify those contexts.
No field is zeroed or forced solely to satisfy ordering. The complete result
join, context/allocation translation, checked maintained implementation, final
ordinary-production correspondence, admission and measurements remain open.
These component dispositions do not accept full E_max or delay A1's own gates.


## Remaining precision/mechanism witnesses qualified (ROOT, 2026-10-02 UTC)

ROOT read the complete RV29 vk_remaining_43/REVIEW.md, verified every payload
under sealcb1d7ce04d69db302d5d5c6b1dc26735b22d3bbb619a876f80c558e0fb2c6248,
and accepts the scoped UNKNOWN/F05/F06/F07 semantic witnesses and their actual
planned normal controls. The report's Scoped semantic results and Controls and
full scope sections preserve the exact verdicts, roles, bodies and parity scope.
Generic failure, record/work drift and the F17 alternative supply no credit.

Preserve the manager's disclosed late closeout as a timing exception: the review's
Source, artifact and raw provenance section establishes actual execution inside
the grant and distinguishes the manager's later seal. ROOT does not retroactively
extend the window or call the entire closeout compliant. Its child-seal timing
was reported by ROOT, not independently reconstructed from a manifest-write stamp.
The numerical/source/artifact evidence remains independently assessable and was
verified. The next runtime block reserved explicit sealing time and has returned;
R28/R02/scalar results remain pending independent review44. Final A1 gates remain.


## Conditional K6c result composition verified (ROOT, 2026-10-02 UTC)

ROOT read the complete RV30 result5_join_28/RETURN.md and verified every payload
under seald0f850310a9519441d274b7a270bdb834162811d5bc048b880edee70da354eac.
Accept the matching conditional reference/CLI composition and five-field ordering
within its exact inherited input, profile, launch and metric premises. The report
states all numerical maxima and comparison windows; they are expression bounds,
not measurements or admission decisions. No actual graph count, unspecified
future path allowance or current-executable qualification is inferred.

The accepted kernel/caller/join review packets live on the separate K6c branch,
currently commit60a52da9467b73d25898e01312e20d7a4c533902; their immutable manifests
are the warrants even where another checkout does not carry the relative paths.
ROOT merged reviewed A1 source/accounting942572 into K6c at8b6b4db5aa5ee8a3a3db149ff36b9418910bc9a3
and verified exact maintained core/validation equality before releasing the
bounded shared-kernel implementation. Original reference storage facts remain
pinned to their prior origins, not rebound to refreshed A1 observations.

Context16 is an unaccepted implementation proposal with named H caller binding
and origin/failure questions; their bounded reviews are in progress. Shared-kernel
translation does not release a partial legacy estimate or adapter. Complete
context translation, checked implementation, final artifact/source/request/input/
launch correspondence, admission replay and required measurements remain open.


## Final registered R28/R02 witnesses qualified (ROOT, 2026-10-02 UTC)

ROOT read the complete fresh RV31 vk_final_01/RETURN.md and verified all payloads
under seal831cdff691167473c40c17f95d27b7a23e4a40e516aa51b85511e15a3ac1d0f0.
Accept the original R28 and original VR TREE100 R02 semantic witnesses with their
actual normal controls. The report's Actual results section gives the exact
reached shift and named VerificationEstimate/acceptance transitions. No missing
ratio, work counter, generic refusal or certificate substitute supplies credit.

The separate original scalar test supplies complementary seed-site evidence only;
its later assertions remain unexecuted and uncredited in the fault call. No FK
TREE100 substitution is used. Other unshown Ceilings receive no invented cause or
separate credit. All original parity records restore, including their original
Sensitive statuses; restoration does not mean every comparison is Passed.

The interrupted RV29 final44 archive-only partial has a ROOT preservation note
and seal, not a completed reviewer verdict. RV31 is a fresh independent native
identity; its review binds cf7841 and does not silently approve later HEADs.
The complete V-K packets and their approved replacement/disposition history are
ready for final-candidate reconciliation. Prior unqualified attempts, the unrun
superseded P15, and the narrowly owner-approved F17 alternative remain distinct.
No protected criterion is weakened and no numerical source/test/oracle changed.

Freeze the final A1 candidate after integration, then require independent exact-
candidate confirmation, full-SHA hosted dispatch/CI, clean exact-head GEN-8 and
Mac DEC-025 comparison before merge. The source-reach check remains H/validation
only unless the final diff establishes otherwise. AUD-T3-01 and F2a reliance stay
held until actual closure; K6c implementation/qualification is a separate path.


## K6c shared implementation and caller contract (ROOT, 2026-10-02 UTC)

ROOT read the complete RV33 kernel_01 RETURN/REVIEW and verified its25 payloads
under sealf50f48bfdaa4874a556b7660285ed0328bba57ee0f7bf099892ad2bdf4e7d919
(run from packet root; manifest is under _run_records). Accept bounded kernel
fan-in for exact4a6cb3402f95d98c6f46f702055fe4b0c5245c56 under its declared
reference premises. The review's Independent compiled checks reports the actual
clean-archive tests and full reference comparisons. This does not cover later
H helper/adapter edits, current-artifact qualification or a complete admission
estimate. Those changes need their own exact-candidate review/backcheck.

ROOT also read and verified RV32 h_context_01 (seal8ca1f20ce41d85c513b2443990780983f8463a3dc24e91e80bcc3fe7a938e89f)
and selects its reviewed H1/H2/H3 caller contract, as concretely recorded in the
review and ROOT_CLARIFICATION. Explicit OriginalK6bPair is reference-only; actual
normal and counts-continuation contexts carry their actual known arguments.
Runner pre-admission must be rebound before using a numeric estimate or ratio
denominator. The source-refused variant retains raw-input construction bounds,
normal staged refusal and positive source-window totals; explicit zero components
mean unexecuted kernel phases only. Actual repeat digits enter outer-prefix
formatting. No flags, policy thresholds, stale fallback or unbounded future-path
allowance is introduced.

The implementation and review packets were relocated intact into their existing
_run_records evidence roles; old-location pointers and the complete blob/mode
ledger preserve custody. No source/evidence/checker bytes were changed by that
placement correction. I21 H and I24 VR implementation grants own disjoint caller
fences, with the narrowly notified raw-H helper extraction separately requiring
RV33 backcheck. Complete adapters, protected tests/mutants, final ordinary-source/
request/type/input/launch qualification, admission replay and measurements remain.


## A1 merged; AUD-T3-01 publication correction closed (ROOT, 2026-10-02 UTC)

A1 merged as PR1070 at3a0251874d6ab38008173a22ef09d651a0f20d9e from the exact
reviewed/gated headcb13dcd9fcc8c252fa9610b1c6fef0c5b2c3c20c. ROOT fetched and
verified unchanged main546e05a159a58f5ceffe1d315373bc2f31982ea8 immediately before
--merge --match-head-commit. The merge's parents match both bindings.
IMPLEMENTATION/A1_MERGE/RECORD.md carries the same-pass merge record and links
RV31's source/records and external-gate confirmations plus preserved gate evidence.

Close AUD-T3-01 for the reviewed retained publication route under the independently
derived, owner-selected M03-INTEGRITY-MP-v2 replacement. This does not vindicate the
old A1 proof assumption or edit the audit/counterexample history. The audit-specific
hold on relying on that corrected published guarantee is discharged on this basis.
F2a still waits for K6c/ROOT W1 limits and its own contract adoption/qualification;
no product receipt, native route or engineering release is implied.

K6c H and VR integration candidates have returned with author tests. ROOT full-
diff verification, independent actual-candidate review/helper backcheck, complete
mutation obligations, final ordinary-artifact/input/launch qualification,
chronological admission and required measurements remain before its acceptance.
All other T3 successors and owner decisions stay as recorded in the work graph.


## K6c runner holds precede automatic counts binding (ROOT, 2026-10-02 UTC)

The independent RV32 VR and RV33 H reviews of exact
9086964a1fb656a76cda6d1002d8594efa636fdc identified a blocking ordering defect:
new model-specific counts binding can occur before the existing conditional
approval or recorded-ascent hold is evaluated. ROOT read RV33's process-free
RUNNER_HOLD_REPRO.json and confirmed the source ordering. No actual 10,000-member
process was run by these reproductions. Final review returns remain pending.

Keep those holds before automatic model-specific counts/normal launches in
run_tier. Existing tier noop baselines and explicit standalone counts acquisition
retain their semantics. After nonnumeric eligibility passes, bind the actual
launch context before numeric admission/backstop and ratio use; no stale or fake
counts substitute is permitted. Existing named refusal, conditional approval,
recorded-ascent semantics, thresholds, normal caps, VR counts cap and process-free
plan behavior remain. This is a correction within the accepted caller contract,
not a new policy or owner-held limit.

ROOT grants I24 a 20-minute four-file H/VR runner-and-test repair, with additive
I24/runner_hold_repair_02 evidence and no other maintained edits. Same RV32/RV33
reviewers must backcheck the committed repair. They continue exact-908 review
against their own archives while I24 edits. I21 separately receives a 45-minute
archive-only kernel/H mutation assignment against908, with actual patches and
NONE control under I21/kernel_mutants_26. No performance slot or final-bound
acceptance follows from either assignment. Native tasks retain no Git/index
write authority; ROOT integrates. A1 remains merged and its audit closure stands.


## Owner pause after K6c runner closure and mutation stop (ROOT, 2026-10-02 UTC)

The owner requested a graceful halt at the current convenient conclusion.
All current bounded TASKs returned and stopped; no follow-on implementation,
experiment or performance slot was granted. T3 is paused by owner direction,
not complete. HANDOFF_2026-10-02_OWNER_PAUSE.md is the recovery entrypoint.

ROOT read both complete same-reviewer backchecks and verified their payloads.
Close RV33-H1 and RV32-V1 on10315a8167c47f43aa41402beb88ed2c70e62cf2:
R/source_review_RV33/runner_backcheck_03 and
R/design_review_RV32/_run_records/runner_backcheck_03. The latter was relocated
intact into its raw evidence role; the navigation pointer and custody ledger
preserve every sealed byte/mode. No numeric admission policy was weakened.
Author system-Python use is disclosed; independent backchecks used the requested
VENV. This closes the runner defects only, not complete K6c qualification.

Preserve I21/kernel_mutants_26 as an incomplete required programme: its RETURN
reports NONE passing, four numerical assertion kills and the real M05 Uc-overlap
survivor. The archive is restored but its last compiled mutant binary remains
diagnostic. Seven later kernel variants are unrun. No mutant survivor is waived.
I24/vr_mutants_prep_03 is uncompiled/unexecuted preparation with two pending
witness cells; I25/historical_replay_01 is a conditional author replay awaiting
independent review and historical VR launch-gap disposition. Neither is accepted
as complete E_max or a measurement/admission grant.

The source/review/evidence checkpoint is K6C4be7239b9f356beee5c7e4ab2f689033fad30245;
maintained source remains10315. ROOT verified the returned seals and preserved
the requested interpreter note and all raw evidence. No target/baseline was
pruned. A1 remains merged at3a0251874d6ab38008173a22ef09d651a0f20d9e, with the
corrected retained-route AUD-T3-01 closure and broader F2a prerequisites distinct.

ROOT also reconciled the graph's stale Current table cell with A1's already
recorded merge and this pause; historical progression remains preserved. Resume
only after owner direction, starting with the narrow M05 witness/test gap and
then the existing K6c sequence. Standing TASK/Git/gate rules and all owner-held
T3 choices remain in force.


## Owner resumes after graceful halt (ROOT, 2026-10-02 UTC)

The owner explicitly resumed T3 and asked ROOT to retain and improve its management
lessons. The preceding pause is historical and revoked by that direction. No A1
gate is reopened: GitHub confirms PR1070 merged and main remains3a0251874.
K6C40179f1da5961cfef7edeaf7be4eb675379f79b5 is clean and pushed; its maintained
source is10315. The M5guard is running and no Cargo/rustc/solver process was left.

Previous native child sessions are absent, so fresh I26 and RV34 identities are
dispatched directly by ROOT. I26 owns only a narrow source-valid Uc regression
witness/test and exact NONE/M05 check; RV34 independently reviews I25's saved
historical replay. Their scopes are disjoint; no large measurement slot or wider
implementation/host-tool grant follows. Briefs are R/BRIEFS/I26_UC_WITNESS_01.md
and RV34_HISTORICAL_REPLAY_01.md.

Operational lessons for this run: name the claim and a decisive falsification
check before delegation; expose individual lifetime terms that outer maxima can
mask; keep implementation, review, artifact qualification, admission and measured
acceptance distinct; give shared edits one owner and frozen review bases; report
supporting work against the numerical question and its time bound; retain one
compact recovery entrypoint with selected evidence. These are this run's execution
practice, not an amendment to standing agent instructions or a reusable workflow.


## Narrow VR lifetime-phase regression grant (ROOT, 2026-10-02 UTC)

The saved VR preparation names two unexecuted witness gaps. ROOT commissions
fresh I27 to distinguish phase-identity coverage from domination by later global
maxima, and to add only source-backed local phase regressions. A narrowly
mechanical private helper extraction in VR/src/envelope.rs is permitted if the
existing estimator consumes it and algebra/lifetimes remain unchanged. No public
contract, bound policy, supported input domain or protected criterion changes.
The detailed fence and25-minute bound are R/BRIEFS/I27_VR_PHASE_WITNESSES_01.md.
I26 retains the Cargo lane until explicit ROOT release. H/VR writes are disjoint.
Independent review follows ROOT verification/commit before any claimed closure.
