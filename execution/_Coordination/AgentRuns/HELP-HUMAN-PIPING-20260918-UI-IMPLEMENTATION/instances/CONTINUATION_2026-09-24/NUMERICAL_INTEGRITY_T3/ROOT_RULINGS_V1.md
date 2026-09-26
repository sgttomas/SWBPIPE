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
