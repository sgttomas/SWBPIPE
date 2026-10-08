# RV115 (RV-K) ADDENDUM_03: S-4 (a)'s numerical content in I97's B2-C revision 01

TASK (Type 2), RV115, holding RV-K, for ROOT (HELP_HUMAN, Agent 0), who is the return path. I made no delegation. 2026-10-08 UTC. This addendum answers the coordinator's request under RR "RV118 (RV-C) accepts B2-C with amendments; B2-C ruled; …" (ruling 1: "RV115 (RV-K) confirms (a)'s numerical content"). REVIEW.md, ADDENDUM_01, ADDENDUM_02 and their sums are untouched.

**The subject:**
- `R/I97/b2_c_01/REVISION_01.md`, sha256 `6f6a583f601fa4f6111001196eeb1488a71f674547ff12bb109fc7cddcea38a1`, verified; SHA256SUMS.revision_01 10 of 10 OK. I read §0, §1.1, §1.2 and §6.
- DEF-C r1, `statics/r1/retained_precision_prepared_combination_v1.json`, raw `6467c733ff5060afd3040172a5df91b12c228555254ae6d33d40555e06f30c03`.
- I97's `_run_records/r1/b2c_checks_r1.py` (§2, `guard_study`) and its output (`s4_guard`).

**The code read:**
- FK `final_case.rs`: `gate`, `exact_test`, `ProductUnit::normalize`, `couple`, `recipe`;
- FK `adaptive.rs` `FLOOR_RATIO_BITS`;
- RE `preview_physics_evidence.rs` `guarded` and `combination_magnitudes`;
- PY `preview_physics_evidence.py` `_consistent_norm`;
- TS `previewPhysicsEvidence.ts` `consistentNorm`.

All at NUM `d1d6517455`, whose `P/core`, `P/fixtures` and `P/schemas` trees equal main `2007709549` (`git diff --quiet`, checked).

**Method.**
- One standard-library Python script of my own, run once with VENV (`-B`, `PYTHONDONTWRITEBYTECODE=1`, `TMPDIR` in `WT/scratch/rv115_b2_kd/`), in `addendum_03/`.
- It models correctly rounded and faithful hypot exactly, with `Fraction` and integer square roots, and applies the kernel gate's two sharper predicates literally.
- No cargo, no native job, no install, no Git write.

Placeholders are as in REVIEW.md.

## Verdict

**CONFIRMED on soundness, with 1 SHOULD-FIX on availability.** 0 BLOCKING, 1 SHOULD-FIX, 5 NOTE.

1. **The recipe is sound.** The final gate measures the actual published value against the dual enclosure. So no published combination magnitude can lie outside it, whatever the recipe. A failure is that combination's `facade_certificate`.
2. **The coverage argument holds as an upper bound.** The bound is h_mag ≤ √3·max_k h_k + |p − ‖x̂‖| + the SI rounding:
   - it held in 9,000 of 9,000 trials;
   - |p − ‖x̂‖| was at most 0.97 ulp for a correctly rounded hypot, and 2.00 ulps for an adversarial faithful one.

   **It is not a pass guarantee** (SA3-1). For relative-class rows within about 2^13 of the body scale S*, the allowance's rounding term (2^-53·|n|) was sized for a single rounding. The nested hypot's two roundings exceed it more often than v0's recipe does.
3. **The 64ε guard holds by construction** for any pair of hypot implementations each within about 32 ulps of the exact norm of the published components.
   - With faithful libraries, the margin is at least 16×.
   - My extreme pairing (correctly rounded against an adversarial faithful library, subnormals included) reached 1/32 of the allowance. I97's 1-ulp model reached 1/16.
   - The cases this does not prove are in NC-3.
4. **NB-1 and NB-3 are worded as asked.**
5. **Exactly four DEF-C paths change from v0,** and H rebuilds independently.

## Findings

| ID | Severity | Where | Finding | Required change |
|---|---|---|---|---|
| **SA3-1** | SHOULD-FIX | REVISION §1.1, "The certificate's coverage" | **S-4 (a) trades certificate availability for the guard-by-construction, and the text does not say so.**<br>• **Why the bound is not a pass guarantee.** The coverage bound is correct, but in the relative class the gate's allowance (`exact_test`: (2^-64 + 2^-85)·max(\|n\|, S*) + 2^-53·\|n\| + 2^-1074) has one round-to-nearest term. The recipe brings two hypot roundings (≤ 1 ulp correctly rounded, ≤ 2 ulps faithful) plus the mm→SI conversion. Rows with \|n\| ≥ 2^-34·S* are relative, and only those above about 2^-13·S* lack absolute slack.<br>• **Measured with an exact (point) enclosure, at S* = n, 4n or 64n** (the most favourable certificate), per 3,000 rows:<br>&nbsp;&nbsp;– v0's recipe: 9–13 % fail;<br>&nbsp;&nbsp;– r1's with a correctly rounded hypot: 10–17 %;<br>&nbsp;&nbsp;– r1's with an adversarial faithful hypot: 63–100 %;<br>&nbsp;&nbsp;– at S* = 8192n, every recipe: 0 %.<br>• **The failures are a regression,** not inherited: in 98 % of r1's failures every component row passes.<br>• **Platform dependence.** A combination fails as a whole on any row. The published bits and the verdict also depend on the producer platform's libm `hypot` (Rust `f64::hypot`), so the same input can certify on one OS and not another. | **Correct §1.1:** the bound is an upper bound, not availability parity.<br>**ROOT chooses one of:**<br>• **(i)** accept, and have B2-W's witnesses (W-CB1 and W-CB4a/b) report the magnitude rows' predicate outcomes on the producer platforms;<br>• **(ii)** specify the published magnitude as **RN64 of the exact 3-norm of the published components** (one rounding, computed exactly, the same on every platform). In my model it restores v0's failure rate (379 vs 378, 315 vs 314, 292 vs 273). It keeps G7 by construction, with \|p − r\| ≤ 2.5 ulps, a 25× margin.<br>DEF-O's `support_magnitude` has the same property for cases, and is frozen |
| NC-1 | NOTE | Context for SA3-1 | **DEF-O's own baseline.** Even DEF-O's projection fails the sharper predicates for top-scale **mm** rows with a point enclosure, about 7 % per component row in my model. The mm→SI conversion in `normalize` (`y / 1000.0`, binary64) is a second round-to-nearest after y = RN64(raw), against an allowance sized for one. Explicit example (evidence §6): h is 1.42× the allowance. This is DEF-O's frozen behaviour, consistent with the `SharperExact` fallbacks seen in RV97, I98 and I99's probes, but I did not attribute those to it. It is outside this addendum. | For ROOT's awareness; route to DEF-O's owner if wanted |
| NC-2 | NOTE | DEF-C r1 `stages.observables` | "Within 64 epsilon relative" is relative only above MIN_POSITIVE. The readers use 64ε·max(\|p\|, MIN_POSITIVE), which is an absolute 64·2^-1074 below it. | Optional: write the formula |
| NC-3 | NOTE | REVISION §1.1 item 2, the 16× margin | **I97's 4,013-triple study models faithful libraries** (math.hypot ± 1 ulp per call), and supports the claim for them. It does not prove the claim for:<br>• **(a) the TS reader's `Math.hypot`.** ECMAScript makes it implementation-approximated. The shipped Tauri app runs on the platform webview (JavaScriptCore on macOS and Linux, V8 through WebView2 on Windows), and vitest runs on Node's V8. No accuracy bound is specified, although the scaled, compensated algorithms in use are far inside 32 ulps.<br>• **(b) an unscaled implementation,** which would overflow or underflow for components beyond about 1e±154 mm.<br>• **(c) non-IEEE modes,** such as flush-to-zero or x87 extended precision. None is a target.<br>RS's `f64::hypot` and the producer are the same libm only on the same platform. A receipt checked elsewhere relies on the margin. | Optional: a TS unit test of `consistentNorm` on adversarial and subnormal triples in each JS engine CI uses |
| NC-4 | NOTE | DEF-C r1 `rows.displacement_magnitude` | **The recipe reads the frozen raw mm values.** That is the bits the guard reads (`value`). One row per node and component (R-7) makes the guard's last-match `find` bind the same row. | None |
| NC-5 | NOTE | REVISION §6; DEF-C r1 | **NB-1** (`scope.operand_equality`: "the selected material operands of every member bit for bit") and **NB-3** (`lanes.loads`: "data flags from each individual product (c_i!=0 and v_ij!=0), never from a net, so an exactly cancelled net stays data") are worded as I asked, and NB-3 matches FKR/ledger.rs:225. | None |

## 1. The recipe and the certificate's coverage

**DEF-C r1's recipe:** p = binary64 hypot(hypot(x, y), z) of the node's frozen raw mm components, n = RN64(p/1000), "still certify dual physical norm".

**Soundness.** `check_intervals` treats a `Native(DisplacementMagnitude)` row like any other. It computes h = sup_{q∈H}|n − q| over the hull of the two lanes' norm enclosures, then applies the class predicates to the published n (final_case.rs `gate`). The recipe chooses n, but the certificate decides. An n outside the allowance refuses; it is never published as certified. **Confirmed.**

**The coverage bound.** Take the components x̂ (published), each with h_k = sup over its own hull. For any q in a lane's box, ‖x̂ − q‖₂ ≤ √3·max_k h_k. A point of the norm hull between the two lanes is bounded by the attained endpoints. So:

h_mag ≤ |n − p/1000| + |p − ‖x̂‖|/1000 + √3·max_k h_k (+ the components' own SI-conversion terms).

- **Checked:** 9,000 of 9,000 rows.
- **Measured |p − ‖x̂‖|:** at most 0.97 ulp(p) with a correctly rounded hypot, 2.00 ulps with an adversarial faithful one, and 0.50 and 1.00 for planar vectors (z = 0).
- So I97's "√3 factor on absolute floors, at most 2 ulps more" holds, with the SI conversion's ½ ulp on top. **Confirmed as a bound.**

**Why the bound is not availability parity** (SA3-1). For relative-class rows, the absolute part of the allowance is (2^-64 + 2^-85)·max(|n|, S*). That is negligible against 2^-53·|n| unless S* ≥ about 2^13·|n|. The 2^-53·|n| term is half an ulp to one ulp of n, so it covers one round-to-nearest.

**The study** (evidence §1): exact truths q, published components RN64(q_k) in mm, a point dual enclosure (the most favourable certificate), and the exact and binary64 sharper predicates as `gate` applies them. Failures per 3,000 rows:

| Rows | Scale | v0 (RN64 of the norm, then SI) | r1, correctly rounded hypot | r1, adversarial faithful hypot | RN64 of the exact 3-norm of the published components |
|---|---|---|---|---|---|
| Comparable components | S* = n | 378 | 487 | 2,704 | 379 |
| One small component | S* = n | 314 | 505 | 2,995 | 315 |
| Planar (z = 0) | S* = n | 273 | 292 | 1,878 | 292 |
| Any | S* = 4n, 64n | about the same as at n | | | |
| Any | S* = 8192n | 0 | 0 | 0 | 0 |

**Reading the study:**
- In about 98 % of r1's failures, every component row passes (`cr_fail_components_pass`). So the failures are the recipe's own.
- A real libm sits between the correctly rounded and adversarial faithful columns.
- The 3-norm column is SA3-1 option (ii). One exact rounding of the published components' norm matches v0's availability in my model, and it is platform-independent.

## 2. The 64ε guard by construction

**The guard** (RE `guarded`, PY `_consistent_norm`, TS `consistentNorm`) is |p − r| ≤ 64ε·max(|p|, MIN_POSITIVE). Here r = hypot(hypot(x, y), z) of the same `value` bits in the reader's own library.

**Why it holds:**
- For normal p, 64ε·|p| ≥ 64·ulp(p).
- Each side's nested error is ≤ 2 ulps for a faithful library, so |p − r| ≤ 4 ulps: a 16× margin.
- Below MIN_POSITIVE, the allowance is 64·2^-1074, and faithful nested errors are ≤ 2 subnormal ulps per side.
- In general, the guard holds for any two implementations each within about 32 ulps of the exact norm.

**Confirmed by two studies:**
- my pairing of a correctly rounded library against an adversarial faithful one (700 triples, subnormal, normal and large): at most 1/32 of the allowance (evidence §3);
- I97's 81 pairs per triple over 4,013 triples: at most 1/16.

**Arithmetic and edge cases:**
- The guard's own arithmetic is exact: the multiplier is a power of two, and the subtraction of nearby values is exact (Sterbenz).
- p = hypot ≥ +0 satisfies PY's and TS's `magnitude >= 0`.

NC-3 lists what these studies do not prove.

## 3. DEF-C r1 against v0, and H

**Exactly four leaf paths change:** `lanes.loads` (NB-3), `rows.displacement_magnitude` (S-4 (a)), `scope.operand_equality` (NB-1) and `stages.observables` (new, S-4 (b)). Evidence §4 has the texts.

**`support_magnitude` equals DEF-O's,** byte for byte.

**H rebuilt with my own canonical form** (evidence §5):

| Object | Hash |
|---|---|
| DEF-O (control) | `a7ed7ca0…` |
| DEF-C r1 | **`0c43cf427b35d35e291e42382d242bca3762b61344b705744127c7c9b4372b6f`** |
| DEF-C r1, alternative domain | `82fccc74…`, as I97 states |

**DEF-C r1's raw bytes** are their own canonical form, and `operand_definition.sha256` is still H(DEF-O).

**`stages.observables` is the guard plus the support coverage,** run before G5a, with a failure scoped to the combination. Apart from NC-2's wording, it needs no further numerical sign-off.

## 4. Inputs, execution and limits

**Read** (sha256 prefixes):

| Input | sha256 |
|---|---|
| REVISION_01.md | `6f6a583f601fa4f6` |
| DEF-C r1 | `6467c733ff5060af` |
| DEF-C v0 | `03d40598be82a5df` |
| DEF-O | `3e0779a45a74cf0b` |
| I97's `b2c_checks_r1.py` and output | verified by SHA256SUMS.revision_01 |
| The kernel and reader code | NUM's maintained tree (= main `2007709549`) |

**Executed.** `addendum_03/rv115_addendum03_checks.py`, run once, exit 0. It takes DEF-C r1, DEF-C v0 and DEF-O as arguments and writes only stdout. Output is in `addendum_03/rv115_addendum03_checks.out.json`, and `addendum_03/RUN_ADDENDUM_03.md` has the command with placeholders.

**Limits:**
- **No code was compiled or run.** The rates are from an exact model with a point enclosure, not from kernel runs. Real enclosures carry the K/G law gap, which only adds failures.
- **The real libm's error distribution is unmeasured.** SA3-1 (i) would measure it on the producer platforms.
- **NC-1 is a model observation** about DEF-O, outside this review's scope.
