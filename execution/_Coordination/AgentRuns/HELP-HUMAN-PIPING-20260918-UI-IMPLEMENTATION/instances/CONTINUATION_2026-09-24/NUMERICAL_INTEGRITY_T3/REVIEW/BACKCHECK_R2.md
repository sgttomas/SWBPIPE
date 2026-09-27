# T3 V1 — backcheck of D1's DESIGN revision 2

Type 2 TASK V1, the reviewer of `REVIEW/RETURN.md`, 2026-09-26. The T3 manager requested this backcheck; ROOT made it a narrow one.

- **Scope.** Read-only. Standard-library Python only. No cargo, no Git write. T1 was read only through `git show f3270ea79`.
- **Inputs at the T3 branch head `45cfc92b1`:**
  - `T3/DESIGN_NUMERICS/DESIGN.md` revision 2, committed at `63c3d503c` (sha256 `3ff9c1fd873d9f28…`, verified), with revision 1 at `_run_records/DESIGN_revision1.md`;
  - `_run_records/probe_rev2_b1.*` (I reran it and its output is byte-identical; SHA256SUMS verify);
  - `T3/ROOT_RULINGS_V1.md` and `ROOT_RULINGS_V2.md` §3;
  - `T3/MANAGER_NOTES/V1_DISPOSITIONS.md`;
  - `T3/DESIGN_STANDING/DESIGN.md` revision 2 (`d566713e9`) §4.5.2, §4.8, §4.9 and §5;
  - `T3/REFERENCE_CHECK/RETURN.md` §3.5 and F2;
  - the R1 RF-RANGE LEF-small inputs (`T3/REFERENCES/references.json`).
- **The F2 erratum** (`R2_ERRATUM_F2.md`) was **not committed** when this backcheck was written. Revision 2 is checked without it (§2.5).
- **Line numbers** refer to `c61a540ea` for main and `f3270ea79` for T1.

## 1. Verdict: **FINDINGS**

Revision 2 resolves V1-B1 and every SHOULD-FIX assigned to D1 as ROOT ruled. I found no new silent-wrong route in revision 2 itself. S11B-1 (the binary64 route's prescribed-motion right-hand side) belongs to S11, and ROOT has ruled on it.

What remains is SHOULD-FIX or NOTE, and all of it is interface or specification work:

- the S8 floor's reader-side enforcement, which D2's S-G does not yet check;
- the retirement gate's standing condition for the joined family under D2's H-a;
- a kernel write set for formation-time scaling;
- the invocation digest when capture has none;
- the pending F2 erratum.

## 2. Answers

### 2.1 V1's findings against ROOT's rulings and the dispositions

| Finding | Status | Evidence |
|---|---|---|
| **B1** | **Resolved** | Exact-sum rule for every multi-term sum outside the factorization: loads, stiffness entries, reduced RHS with `K_fc·u_c`, residuals, reactions, recovery sums, combinations (§4.1.2 items 4–6, §4.1.4, §4.1.5, §4.1.9). Combinations over retained states are formed at p and 2p and put under the stop rule, with S\* from the combination's own body and escalation independent of the operands (§4.1.1, §4.1.6 "Scope"). Negative controls 13–16 (§7.3) cover check L (B1-L), A + B − A2 (B1-C), a combination taken out of the stop rule (B1-E) and duplicate-operand cancellation. The §3.1 claim is withdrawn and restated as limited to precision-dependent error (§3.1, l.198-201). I reran `probe_rev2_b1.py`: byte-identical |
| S1 | Resolved as ruled | R-3(a) and (b) and R-7 (i) stated (§4.4, l.413-416) |
| **S2** | **Resolved in structure.** One gap: S2-R below | The shared gate, with four conditions and per-family application at F2 and F3 (§4.4.1). D2 cites it (D2 §4.5.2) |
| S3 | Resolved | D2 owns the successor readers (§5; D2 §4.9) |
| **S4** | **Resolved** | Checked profile `openpipestress_jcs_ijson_v1`. Every non-integer receipt value is a 16-hex bit string, and plain numbers are exact integers only. A per-case hashing failure becomes `unavailable` (`receipt_encoding`) with ordinary standing. A publication-hash failure republishes under the base identity with every attempt declined, on the SF-1 pattern. Never an `Err` (§5 item 2). This matches D2 G1 and G2 |
| S5 | Resolved on D1's side | Capture-boundary cases in RF-RANGE and VP-ROBUST; the fix is D2's R-6 / S-H (§4.7). See S5-R below for the digest |
| **S7** | **Resolved** | b is not a `StructuralReport` field. It gets an extra evidence line only when b ≠ 0, and only in cases refused on main, so no committed byte changes. The b-rule is normative (evaluate at b = 0 first; subnormal refusal; window; one scaled retry; publication outcomes) (§4.7) |
| **S8** | **Stated; enforcement incomplete across the interface** (S8-R below) | Floor R = 2^-64/1e-9 stated. Receipt classification `absolute_verified` / `relative_verified`. VP-ROBUST zero-scale check. D-12 offers withholding (§4.1.6, §4.10) |
| S9 | Resolved | §4.11: the targeted classes, a differential of at least 10^6 operations, and seeded rounding mutants |
| S11 | Resolved in DESIGN; the detail is in `S11_CONTAINMENT.md` | See `REVIEW/S11_BACKCHECK.md` and ROOT's ruling at `45cfc92b1`. **S11-K's write set in §6 does not yet carry ROOT's S11B-1 decision** (the exact reduced RHS at `FK/structural.rs:603-606` and `FK/lib.rs:870-877` joins S11-K) |
| N1 | Resolved | Column (C) relabelled; (C′) added (§3.1) |
| N2 | Resolved | A represented-only `NegativeEnergy` triggers W1 for supported families, and W1's check at p decides (§4.3). Sound: a genuinely indefinite primitive model fails the pivot screen at every p and ends unresolved |
| N3 | No change, as proposed | P1 checks the Passed boundary |
| N4 | Resolved | K5 after K2b; S11-K → K2a → K1 → K2b → K5 (§6, D-10) |
| N5 | Resolved | Owners proposed; equivalent-static open as D-14 |
| N6 | Resolved | macOS RSS watchdog (§4.8) |
| S6, S10, N7, N8 | D2's items, not D1's | Not checked here |

### 2.2 The findings that need changes

**S8-R (SHOULD-FIX): the floor is enforced only if the readers check it, and D2's S-G does not yet.**

- D1 §4.1.6 item 2 relies on D2's S-G to "verify the classification against the published rows", and leaves rule binding of an `absolute_verified` quantity to D2.
- D2 revision 2 G5 (D2 §4.9.3) checks the stop-rule entries, p, pivot, rcond and the digest. It has no S\* check, no classification check, and no rule for `absolute_verified` quantities in rule binding or export.
- **Reproducibility.** D1 classifies on `|q_p| < R·S*`, which readers cannot see. D1 calls R an "exact binary64 constant", but R = 2^-64/1e-9 is not exactly representable.
- **Required change.**
  - D1 specifies the classification on the **published** binary64 value, as `|q| < fl(R_bits · S*_bits)`, with R's bits pinned in the receipt.
  - D1 defines S\* on published values, so readers can recompute it (or states why S\* cannot be verified).
  - D2 adds a G-check that recomputes the classification, and a consumer rule for `absolute_verified` quantities (for example, refuse relative rule binding, or bind with the absolute bound).
  - Until both exist, the floor is stated, not enforced.

**S2-R (SHOULD-FIX): the gate's standing condition for the joined family.**

- Condition 3 requires fresh results to be Current or eligible. D2's H-a makes any 0.4.0 case that uses the logarithmic law `needs_recompute` under S-G2 (D2 §4.9.5). D1 §4.4.1 condition 1 "records" that limit but does not say what it does to the gate.
- Read literally, the gate then never passes for a family whose committed joined witnesses include a log-law case, and `load-reference-source-1` would stay fresh past F3. That contradicts R-3(a).
- **Required change.** State the rule. For example: condition 3 for a retiring family is "no worse than the retiring identity's standing on the same request" (joined results are `needs_recompute` today), with H-a cases listed. Or H-b. Or R-3(a) is revisited. ROOT should see the choice.

**SCALE-W (SHOULD-FIX): formation-time scaling has no kernel write set.**

- §4.7 step 2 has the kernel forming elements with operands scaled by 2^b, "with F1, kernel-owned sparse assembly".
- F1's write set is facade-only (`PP`, `source_recovery.rs`, `nonlinear_integration`). K1 does not list `FK/lib.rs`, where `local_stiffness` and assembly live (`FK/lib.rs:699-768`). K2a changes `local_stiffness` only to refuse.
- **Required change.** Put the kernel half (scaled formation in `FK/lib.rs`, and the sparse assembly entry that takes b) in a named kernel slice, serialized after K2a.

**S5-R (NOTE, small): the invocation digest when capture has none.**

- D2's S-H (H-1) makes the capture digest fallible and leaves it to D1 whether its receipt needs the digest (D2 §4.8, I-7).
- D1 §4.3's trigger requires "a captured invocation", and D2 G1/G8 bind the invocation digest.
- **Required change.** State that W1 attempts require `digest_ok()`. Otherwise the case gets `RETAINED_PRECISION_UNAVAILABLE` (reason `invocation_not_representable`) and publishes ordinarily. Or specify an alternative digest.

### 2.3 Interface consistency with D2 revision 2

- **IF-1 / DD-11: consistent.** `numerical_quality` keeps the ordinary outcome, and the precision-p outcome lives only in the receipt (D1 §4.5, §5 item 3; D2 §4.9.2). DD-11 does not arise, so D2 can close it.
- **G1: consistent.** D1 names the checked profile; D2 accepts only registered profiles.
- **G2: consistent.** Bit strings, and exact integers only as plain numbers. D1's list (stop-rule ratios, pivot margin, rcond, S\*, R, bounds) is covered.
- **G5: consistent for p, verification, attempts, stop-rule entries (≤ 2^-64), pivot and rcond.** Missing, because D2 predates D1 revision 2: the S\*, classification and load-ledger digest fields D1 revision 2 adds (see S8-R). G5 still lists p ∈ {128, 256, 512} and verification 2p ≤ 1024, which matches D1's schedule.
- **Joined identity per R-3(a): consistent**, apart from S2-R. It is fresh until F3, retires at F3 under the gate, S-E1 comes with F3, S-E2 only if F3 misses T3, and `<load-reference-retained>` comes with S-G2 (D1 §4.4; D2 §4.9.1, §7).

### 2.4 The formation silent zero, and serialization

- **Reach: confirmed, with one correction.** Probe `r2_backcheck/probe_r2_formation.*` uses the LEF-small inputs, with the section approximated as π(OD⁴ − ID⁴)/64 and J = 2I.
  - D1 §4.7 says `12.0 * e * iy / length3`, `g * j / length` "and their siblings round to exactly 0".
  - 6EI/L², 4EI/L, 2EI/L and GJ/L are exactly 0.
  - But **12EI/L³ is a normal value, 3.5 % wrong.** Its intermediate `12.0*e*iy` is the least subnormal, 5e-324, which is then divided by the tiny L³.
  - So the element is inconsistent rather than simply stiffness-free. K2a's check of every intermediate product and quotient catches this, as specified.
  - The realistic-reach conclusion (none) stands.
- **Serialization.**
  - S11-K → K2a → K1 → K2b → K5 has no write conflict on `SA`, `FK/structural.rs` or `FK/lib.rs`.
  - Two notes:
    - (a) K3 and K4 are "new files", but `FK/structural/retained/` needs a `mod` declaration in `FK/structural.rs`, which S11-K, K1 and K2b write. Declare it once, in one named slice.
    - (b) With ROOT's S11B-1 ruling, S11-K also changes `FK/lib.rs:870-877`. §6's statement that S11-K and K2a "share only `FK/lib.rs`, and only trivially" needs updating. They stay serialized, so this is text only.

### 2.5 F2 (V2 §3.5; ROOT_RULINGS_V2 §3): erratum pending

- **Revision 2 as it stands.** It has four kinds, with twist counted as rotation and extension as translation (V2's variant A: 295 twist and extension comparisons below the floor), plus the RF-WEAK regional classes (43 to 53) and 3 RF-CANCEL rows.
- **§4.10's floor check** reports failing quantities and "flags the case as not covered". It does not yet say that **neither a below-floor comparison nor a not-covered flag counts as a pass**, as ROOT ruled.
- **Not met yet.** The erratum is expected to give twist and extension their own kinds and to state the no-pass rule. When it is committed, it should also specify:
  - **(a)** S\* for the new kinds, including their coupling partners if any;
  - **(b)** how the harness obtains twist and extension. If they are differences of published rotations and translations (θ_j − θ_i), their binary64 cancellation limits relative accuracy regardless of the stop rule. If they come from published torque and axial force (T/(GJ/L), N/(EA/L)), they inherit the verified accuracy;
  - **(c)** that V2's counts drop to: 43 to 53 RF-WEAK plus 3 RF-CANCEL still flagged, and the 295 cleared.
- I will backcheck the erratum when the manager gives the commit.

### 2.6 New silent-wrong routes introduced by revision 2

**None found.** Checked:

- **The V1-N2 trigger.** W1 can override a represented-only negative-energy verdict only by passing its own screens at p against the primitive model.
- **The publication-hash republication.** Every attempt is declined and ordinary standing kept.
- **Combination escalation.** A combination that fails at the ceiling is withheld; operand cases keep their standing.
- **K2a.** It turns a silent zero into a loud formation refusal, which blocks the invocation as other formation errors do.
- **The zero witness.** It is limited to the `ResidualRow` rendering, and rows are byte-bound at `source_receipt.rs:811`. This is consistent with my S11 backcheck.
- **W2 subnormal publication.** Subnormal results are labelled; this is unchanged from revision 1.
- **W1's own reduced RHS is exact** (§4.1.2 item 6), so S11B-1 does not affect the method at p.

## 3. Findings

| ID | Severity | Where | Consequence | Required change |
|---|---|---|---|---|
| S8-R | SHOULD-FIX | D1 §4.1.6 items 1–2; D2 §4.9.3 G5 | The floor is stated but not enforced at readers or rule binding; the classification is not reproducible as written | Classify on the published value with pinned R bits; make S\* verifiable; D2 adds a G-check and an `absolute_verified` consumer rule |
| S2-R | SHOULD-FIX | D1 §4.4.1 condition 3 against D2 §4.9.5 H-a | The joined family's gate may never pass, contradicting R-3(a) | State the standing condition for retiring families; ROOT chooses |
| SCALE-W | SHOULD-FIX | §4.7 step 2; §6 F1, K1 | Formation-time scaling has no kernel write set | A named kernel slice for scaled formation in `FK/lib.rs`, after K2a |
| F2-P | SHOULD-FIX (pending) | §4.1.6 kinds; §4.10 | ROOT's F2 ruling is unmet until the erratum lands | Erratum: own kinds for twist and extension, the no-pass rule, and items (a)–(c) of §2.5 |
| S11K-W | NOTE | §6 S11-K row, "Why K2a is separate" | The write set does not yet reflect ROOT's S11B-1 ruling | Add `FK/structural.rs:603-606` and `FK/lib.rs:870-877` to S11-K; update the text |
| S5-R | NOTE | §4.3 trigger; D2 I-7 | W1 behaviour when capture has no digest is unstated | W1 requires `digest_ok()`, otherwise `RETAINED_PRECISION_UNAVAILABLE` |
| FORM-C | NOTE | §4.7 "Reach" | LEF-small's 12EI/L³ is normal but 3.5 % wrong, not zero | Correct the text. K2a already covers it |
| MOD-D | NOTE | §6 K3, K4 | The `mod` declaration for `retained/` falls in a file other slices write | Assign the declaration to one slice |

## 4. Not checked

- No product build or run.
- D2's own revision-2 items (S6, S10 and the rest) beyond the interface rows named above.
- The D1 probes other than `probe_rev2_b1.py` were not rerun for this backcheck (`probe_s11_rev2.py` was rerun in `S11_BACKCHECK.md`).
- The F2 erratum, which was not yet committed.

## 5. Run records (`T3/REVIEW/_run_records/r2_backcheck/`)

- `probe_r2_formation.py.txt` → `probe_r2_formation.stdout.json` (LEF-small formation).
- Standard-library Python 3.11.15, `nice 19`, under 1 s. Hashes are in `T3/REVIEW/_run_records/SHA256SUMS`.
