# RV101 addendum 01: I75's repair of SF-1 (`rustLowerExp` on exact ties)

**Reviewer:** RV101, TASK (Type 2), for ROOT (HELP_HUMAN, Agent 0). No descendants. 2026-10-06 UTC. This is the same-reviewer confirmation of a repair to my review `R/REVIEW_RV101/t6s_01/REVIEW.md` (`510fbdb5…`). That review is unchanged; this addendum has its own sum file, `ADDENDUM_01.SHA256SUMS`.

**Placeholders:** as in the review: `WT`, `NUM`, `P`, `DT`, `R`; and `S` = `WT/scratch/rv101_t6s_01`. No machine paths appear here or in `addendum_01/`.

**Basis read:**
- ROOT's request to confirm the repair;
- I75's `R/I75/t6s_01/REPAIR_01.md` (`143ada4e…`, whole) and `REPAIR_01.SHA256SUMS` (`87307b0a…`);
- I75's `_run_records/repair_01/oracle/gen_words_2.py` and `word_lists.sha256`;
- the candidate's diff.

**Candidate:** `codex/piping-t6-successor-outputs-20261005` at `fdcdb5e024c09e87c4df7e37302112f21a837e57`, one commit over `2033260c57`. I read it with `GIT_OPTIONAL_LOCKS=0` and wrote nothing to `WT/t6-outputs`.

## Verdict: **CONFIRMED**

SF-1 is closed. The repaired `rustLowerExp` prints exactly what Rust's `{:e}` prints on all 185,401 words of my own lists and of I75's `ties16`. Those words include 21,997 16-digit and 10,788 17-digit exact ties. The pre-repair form misprints 16,288 of them, and my suggested 17-digit rule misprints 10,978. The general form, with its round-trip guard, is correct and is the right replacement for my rule.

**Findings:** none new. One erratum to my own review (E-1).

## 1. The general form and REPAIR_01 §3's argument

**The form:**
1. Let n be the digit count of V8's shortest form s.
2. Let r = `toExponential(n − 1)`: the nearest n-digit decimal, which ECMAScript resolves on a tie to the larger candidate.
3. Print r if it round-trips, and s otherwise.

**Why this is correct.**
- Rust prints the closest shortest-length decimal inside the round-trip interval, rounding an exact tie up in magnitude. V8 prints the closest one too, but rounds a tie to even. Over 185,401 words the two agree everywhere except on ties.
- **When r round-trips:** r is a closest n-digit candidate inside the interval. If r ≠ s, the two are equally close, which is a tie, so r is Rust's choice.
- **When r does not round-trip:** r lies below the value, outside the narrower half of the interval at a power of two. No other n-digit candidate is as close as r. The upper neighbour is the unique closest candidate inside the interval, and that is s, the same as Rust's choice.
- **Edge cases:** the form cannot print trailing zeros or a carry into the next decade, because either would make a shorter decimal round-trip and contradict n. The sign is applied to the magnitude, so a negative tie rounds away from zero, as in Rust.

**Ties occur only at 16 or 17 digits.** §3's argument holds:
- Both candidates must lie within half an ulp, so the spacing satisfies 10^k ≤ ulp.
- A normal value satisfies v ≥ 2^52·ulp, and an n-digit value satisfies v < 10^(n+k). Together these give 10^n > 2^52, so n ≥ 16. Shortest forms never exceed 17 digits.
- A subnormal's exact expansion has hundreds of significant digits, so it can never be a short tie.

**Empirically,** my lists contain 32,785 ties: 21,997 at 16 digits, 10,788 at 17, and none shorter (`addendum_01/oracle/checks.txt`).

**The round-trip guard is necessary.** On my power-of-two list (16,772 words), the nearest decimal fails to round-trip on 90 words. Dropping the guard misprints all 90, and with it none.

## 2. My own Rust oracle on 16- and 17-digit ties

**The oracle.** A new dependency-free crate of mine (`addendum_01/tools/rv101_exp/`) prints `format!("{:e}", f64::from_bits(w))`, the formatting `derivative::class_disclosure` applies to b.
- I built and ran it through `WT/tools/t3_cargo.sh run --locked --offline --release` (START 17:55:49Z, END rc=0 17:55:50Z).
- On my review's 60,000 random words, its output is byte-identical to my review's Rust oracle (the RE-crate test).

**The product function.** I imported the product's `rustLowerExp` (and `decodeBinary64`) from an archive copy of `fdcdb5e024`, with one scratch probe file. For comparison, I also computed the pre-repair form and my 17-digit rule.

| List | Words | Product ≠ Rust | Pre-repair ≠ Rust | 17-digit rule ≠ Rust | Ties at 16 / 17 digits |
|---|---:|---:|---:|---:|---|
| my review's 60,000 random words | 60,000 | **0** | 9 | 0 | 5 / 14 |
| my 17-digit tie list (the review's generator, seed 25) | 21,332 | **0** | 5,262 | 0 | 0 / 10,740 |
| **my own 16-digit construction** (m odd, k in 1…24, an exact 17-digit decimal ending in 5; seed 101016; both signs) | 57,124 | **0** | 9,818 | **9,818** | 19,702 / 0 |
| the small-magnitude edge (k = 23…26) | 104 | **0** | 16 | 2 | 6 / 28 |
| every power of two, ±1 ulp and +2 ulps, both signs | 16,772 | **0** | 6 | 2 | 4 / 6 |
| I75's `ties16` (`gen_words_2.py`, seed 75016, regenerated; sha256 equal to I75's `9c0ad6db…`) | 30,000 | **0** | 1,148 | 1,148 | 2,280 / 0 |
| the test's 69 vectors | 69 | **0** | 29 | 8 | 14 / 27 |
| **Total** | **185,401** | **0** | 16,288 | 10,978 | |

**The smallest tie in any list is exactly 2^-25.** It is the test's vector `3e60000000000000`. So my review's reach statement (no b below 2^-25 can tie) holds and is tight. Note that 16-digit ties need k ≤ 24, so |x| ≥ 2^-24.

## 3. The test's vectors are Rust's output

- **All 69 word→text pairs** parsed from the repaired test file equal my own Rust oracle's output for those words, with no exceptions (`oracle/test_vectors_from_test_file.tsv` against `test_vectors_rust_oracle.tsv`).
- **They are not Python `repr`.** At least 29 of them are ties whose lower candidate is even, and a round-half-even printer such as Python `repr` prints those differently.
- **They cover:** both tie lengths; both rounding directions and both signs; the six power-of-two fallbacks; the 2^-25 boundary; and the former edges, now keyed by word.
- The test also checks two full `class_disclosure` messages carrying 16- and 17-digit tie bounds, and keeps the NaN/Infinity refusal.

## 4. The diff, the suites and `tsc`

**The diff.** `git diff --stat 2033260c57..fdcdb5e024` shows exactly two files, +59 / −14:
- `DT/features/results/retainedPrecisionDisclosure.ts`: `rustLowerExp`'s body and doc comment only (sha256 `cb7c7e22…`);
- `DT/features/stress-neutral/retainedPrecisionStressNeutral.test.tsx`: one hunk, the `{:e}` test only (`dd65f2b6…`).

Both hashes equal REPAIR_01's. Against main `c1bfc460fc` the branch still touches the same 19 files the review checked; nothing else changed.

**Suites, on an archive copy of `fdcdb5e024`, as one job under the T3 lock** (`lockf`, START 17:56:52Z, END 17:57:39Z):
- **The ten T6S test files:** 10 files, **430 / 430**. This includes byte parity with both Rust goldens, so no golden bound changed.
- **`tsc --noEmit -p .`:** rc 0, no output.

I did not rerun the full desktop suite; the change touches one function and one test. I75 reports the full suite at 3,590/3,590.

## 5. Erratum to my review

**E-1. SF-1's remedy text was incomplete.** My review said that ties occur only at 17 digits, and suggested reprinting only 17-digit shortest forms. Ties also occur at 16 digits. On my own 16-digit construction my rule misprints 9,818 words, and on I75's list 1,148. My review's 81,332-word check could not show this: its only 16-digit ties had an odd lower candidate, so V8 already printed the upper one. I75 found the gap and generalized the repair correctly. SF-1's diagnosis and its reach bound (2^-25) stand. The REVIEW.md bytes are unchanged; this erratum lives here.

## 6. Host and records

- **Copy:** an archive of `fdcdb5e024` (`P/` without `P/execution/_Coordination/AgentRuns`) in `WT/rv101/cand3`, with the `node_modules` link and the copied wasm assets (hashes equal to the review's). The oracle's target is `WT/targets/rv101-exp`. Both are deleted at return. I keep scratch in `S/addendum/` for any further round.
- **Lock:** cargo ran only through `WT/tools/t3_cargo.sh`; the heavy vitest and `tsc` job ran under `lockf`, with WAIT/START/END marker lines in `WT/guard/cargo_jobs.log` (`addendum_01/suites/lock_lines.txt`). The probe was one small test file and ran unlocked. Nothing went to the system temp directory. No Git writes, installs, DEC-025 or native jobs.
- **`addendum_01/`:**
  - `tools/`: my word-list generator, my oracle crate, the probe and the two run scripts;
  - `oracle/`: the comparison table, the further checks, the test vectors (as parsed, and from the oracle), the hashes of every word list and Rust output, and the oracle's run log;
  - `suites/`: the T6S files' per-file totals, `tsc`, and the lock lines.

  The word lists themselves are not stored; they regenerate from the seeded generator (and I75's `gen_words_2.py`), and their hashes are recorded.
