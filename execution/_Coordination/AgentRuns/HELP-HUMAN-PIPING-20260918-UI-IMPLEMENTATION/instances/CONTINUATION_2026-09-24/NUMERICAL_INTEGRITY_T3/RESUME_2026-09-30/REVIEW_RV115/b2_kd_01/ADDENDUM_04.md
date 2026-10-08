# RV115 (RV-K) ADDENDUM_04: option (ii)'s numerics in I97's B2-C revision 02

TASK (Type 2), RV115, holding RV-K, for ROOT (HELP_HUMAN, Agent 0), who is the return path. I made no delegation. 2026-10-08 UTC. This addendum answers the coordinator's request under RR "I97's B2-C revision 02 verified; RV115 and RV118 confirm it". REVIEW.md, ADDENDUM_01 to ADDENDUM_03 and their sums are untouched.

**The subject:**
- `R/I97/b2_c_01/REVISION_02.md`, sha256 `79007dcd768b2c92183236ee4aec4dbaa921ef26d441388b8bcf2365f686d85f`, verified; SHA256SUMS.revision_02 9 of 9 OK. I read §0–§3.
- DEF-C r2, `statics/r2/retained_precision_prepared_combination_v1.json`, raw `3cebce55d1b31e0031628d7542a33a8debdfa37d2d258cb27dfbdfd1fc28db22`.
- `_run_records/r2/b2c_checks_r2.py`, from which I took only `rn64_norm3`, as the thing under test.
- `exact_norm_vectors.json`, with 411 vectors.

The code basis is NUM `c8e54918cd`, whose `P/core`, `P/fixtures` and `P/schemas` equal main `2007709549` (`git diff --quiet`, checked). I read FKR `directed/certificate.rs` (`sqrt_owned`) and `wide_sum.rs`.

**Method.**
- One standard-library script of my own, run with VENV's Python (`-B`, `PYTHONDONTWRITEBYTECODE=1`, `TMPDIR` in `WT/scratch/rv115_b2_kd/`), in `addendum_04/`.
- **My oracle is computed differently from I97's.** It takes a 1,300-digit Decimal square root of the exact sum S and rounds it to binary64 by my own grid rounding. It then corrects that result exactly against `Fraction` midpoints of the true binary64 neighbours, read from the bit patterns, with ties to even. It refuses iff S ≥ (MAX + 2^970)².
- No cargo, no install, no Git write.

## Verdict

**CONFIRMED, with 1 SHOULD-FIX for B2-K's brief.** 0 BLOCKING, 1 SHOULD-FIX, 5 NOTE.

1. **The recipe is exact as stated.** It is RN64, ties to even, of √(x² + y² + z²) of the frozen raw mm components, with the midpoint decided exactly.
   - A-5 is confirmed. The midpoint ties to the even neighbour `3f80000004000000`.
   - With z = 2^-1074, S − m² = 2^-2148, and the correct result rounds up to `3f80000004000001`.
   - Both a sum rounded to 1,024 bits and nested `math.hypot` give the even (wrong) neighbour there. So the exact side test is necessary.
2. **`rn64_norm3` is correct.** It agrees bit for bit, tie flag included, with my oracle on 18,018 triples:
   - the SA3-1 study's 9,000 triples;
   - my 3,000 guard triples;
   - 3,000 random bit patterns over the whole range;
   - 2,000 extreme spreads (down to 2^-2000 of the largest, with a signed zero, a subnormal or MIN_POSITIVE as the third component);
   - **500 constructed exact midpoints,** from Pythagorean quadruples scaled over 2^-1000 to 2^900, all detected as ties and rounded to even;
   - 500 near-midpoints, one ulp or 2^-1074 off;
   - 18 curated edges: the eight signed-zero forms, the smallest and largest subnormals, MAX alone, MAX twice (refused), 1e±200 and 1e300 with 1e-300 and 2^-1074;
   - and all 411 of I97's vectors.
3. **Availability is v0's.** Failures per 3,000 SA3-1 rows (point enclosure, S* = n), v0 against (ii): comparable 378 vs 379, one small component 314 vs 315, planar 273 vs 292. The planar residual comes from the components' own publication rounding.
4. **G7's guard holds by construction.** Against my exact faithful nested hypot (RD or RU at each call), over 12,648 pairs:
   - |p − r| is at most 2.0 ulps, within the 2.5-ulp bound;
   - that is 1/32 of the allowance;
   - there were 0 failures.
5. **B2-K's planned formation is sound, under the conditions in SA4-1.** These are: an `ExactWideSum` of three exact squares, a 1,024-bit `sqrt` estimate rounded to binary64, then one exact midpoint-side test against the true neighbours.
   - The 1,024-bit estimate's relative error is about 2^-1023, so its RN64 is the right answer or its neighbour across one midpoint. **One step suffices.**
   - The side test sum (x², y², z² and −m²) spans at most about 4,200 bits, inside `ExactWideSum`'s 8,128.
   - Every `add_product` operand has at most 55 bits.
   - My model of that pipeline, with SA4-1's conditions, agrees with the oracle on 2,918 of 2,918 cases, including 300 ties and 300 near-ties.
   - **But the midpoint rule REVISION_02 §2.2 step 4 states is wrong at 2^-1022** (SA4-1).
6. **DEF-C r2 changes exactly two paths from r1:** `rows.displacement_magnitude` and `stages.observables`.
   - H rebuilds independently: `d3fde142aff9c05d709b2fc2a04add42e14c66be3e2b2ba82012da57edf3d957`, with DEF-O's `a7ed7ca0…` as the control.
   - The bytes are canonical.
   - `support_magnitude` is still DEF-O's, and `operand_definition` is still H(DEF-O).

## Findings

| ID | Severity | Where | Finding | Required change |
|---|---|---|---|---|
| **SA4-1** | SHOULD-FIX | REVISION_02 §2.2 step 4; B2-K's brief (§2.3) | **The midpoint rule as worded misrounds at 2^-1022.** Step 4 says "the lower one half as far at a binade's first value, and ±2^-1075 for subnormals". But 2^-1022 (MIN_POSITIVE) is a normal binade's first value whose lower gap equals its upper gap (2^-1074), so both its midpoints are ±2^-1075.<br>**Counterexample** (evidence §5): x = `000fffffffffffff` (the largest subnormal), y = `0000000004000001`, z = 0.<br>• The correct result is `0010000000000000` (2^-1022), and `rn64_norm3` and my oracle agree.<br>• The 1,024-bit estimate also gives 2^-1022.<br>• A lower midpoint "half as far" (2^-1022 − 2^-1076) wrongly steps down to `000fffffffffffff`.<br>The rule's other two edges (MAX, and an `Overflow` estimate) are stated incompletely. | **State in B2-K's brief:**<br>• **(a)** form both midpoints from y₀'s actual neighbours (the predecessor and successor bit patterns, averaged exactly in `Wide`), not from a binade rule;<br>• **(b)** at y₀ = MAX the upper midpoint is MAX + 2^970, and S ≥ its square refuses (ties to even gives 2^1024);<br>• **(c)** an `Overflow` estimate is decided by that same exact comparison (MAX if S < (MAX + 2^970)², otherwise refused), not refused outright;<br>• **(d)** at most one step, with a second as an invariant failure;<br>• **(e)** S > 0 with a zero or underflowing estimate is an invariant failure.<br>Add the counterexample and a near-threshold triple (e.g. MAX, 2^997, 0) to `exact_norm_vectors.json` and K-09 |
| NA4-1 | NOTE | §1.5 | `rn64_norm3` reviewed line by line: dyadic split, E = ⌊log₂√S⌋, q = max(E − 52, −1074), the scaled `isqrt` with a sticky bit, ties to even, refusal iff bitlen(m) + q > 1024. Its oracle agreement is above | None |
| NA4-2 | NOTE | §1.1 | The claim "subnormal results never tie" holds, and also in the binade [2^-1022, 2^-1021). S is a multiple of 2^-2148, while a midpoint (2j+1)·2^-1075 has a square that is an odd multiple of 2^-2150. My constructed ties are all in normal binades | None |
| NA4-3 | NOTE | §1.3 | (ii) restores v0's availability, within 19 rows per 3,000 for planar vectors, from the components' own rounding | None |
| NA4-4 | NOTE | §1.2 | I97's 3-ulp maximum comes from its superset model, which moves the host's own `hypot` by ±1 ulp. Against an exact faithful library, the maximum is 2.0 ulps. Both are far inside the 64-ulp allowance | None |
| NA4-5 | NOTE | §1.4 | The ordinary route keeps nested `hypot` for combination magnitudes, and the retained route forms (ii). The same components can give values about 2 ulps apart on the two routes. G7 holds on both | None |

## Notes on item 5 (the formation)

- **It follows `sqrt_owned`'s shape.** That helper takes one 1,024-bit nearest `sqrt`, an exact q·q − a side, and one step (FKR `directed/certificate.rs:98–128`). The binary64 adaptation replaces q with y₀'s two midpoints. **That is where SA4-1 applies.**
- **Precision.** The 1,024-bit context leaves the estimate about 2^-1023 relative from √S. Every binary64 midpoint is either exactly hit (a tie, decided exactly by the side test) or farther than that from √S, except within that error band. There the side test moves y₀ one step.
- **The A-5 near-tie** (S = m² + 2^-2148) is the case the side test exists for. The estimate is exactly m, `to_binary64` ties to even, and the side test steps up. My pipeline model does exactly this.
- **Span and operands.** `wide_sum.rs`'s 8,128-bit limit covers S − m² for any binary64 components, about 4,200 bits at most. The squares are of 53-bit components and of midpoints with at most 55 bits, all within the 1,024-bit context's `add_product`.

## Inputs, execution and limits

**Read** (sha256 prefixes):

| Input | sha256 |
|---|---|
| REVISION_02.md | `79007dcd768b2c92` |
| DEF-C r2 | `3cebce55d1b31e00` |
| DEF-C r1 | `6467c733ff5060af` |
| DEF-O | `3e0779a45a74cf0b` |
| I97's `b2c_checks_r2.py` and `exact_norm_vectors.json` | verified by SHA256SUMS.revision_02 |

**Executed.**
- **The run:** `addendum_04/rv115_addendum04_checks.py`, once, exit 0. It takes I97's `b2c_checks_r2.py` (to import `rn64_norm3`), the vectors, DEF-C r1, DEF-C r2 and DEF-O as arguments, and writes only stdout.
- **The output** is in `addendum_04/rv115_addendum04_checks.out.json`, and `addendum_04/RUN_ADDENDUM_04.md` has the command with placeholders.
- **The draft runs.** Two earlier draft runs in scratch failed or were superseded before the final run: a vector-format parse error, and a trap search over too small a range. Their outputs are not records.

**Limits:**
- **No code was compiled or run.** The pipeline in §5 is a Python model of B2-K's plan, not FK.
- **`to_binary64`'s rounding is taken from K3's documentation:** a single rounding, ties to even, subnormals included.
- **The availability figures are model rates** with a point enclosure.
