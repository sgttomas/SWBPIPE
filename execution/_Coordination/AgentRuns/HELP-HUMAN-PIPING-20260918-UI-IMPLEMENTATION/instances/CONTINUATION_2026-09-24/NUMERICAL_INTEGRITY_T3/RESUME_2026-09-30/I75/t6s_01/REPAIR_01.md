# I75 T6S REPAIR_01: RV101 SF-1, `rustLowerExp` on exact decimal ties

- **Instance:** I75 (TASK, Type 2). **Parent and return:** ROOT (HELP_HUMAN). No delegation.
- **Finding repaired:** RV101 SF-1 (`R/REVIEW_RV101/t6s_01/REVIEW.md`): `rustLowerExp` differed from Rust `{:e}` on exact decimal ties, where V8 rounds to even and Rust rounds up.
- **Tree:** `WT/t6-outputs`, branch head `2033260c57`, base `c1bfc460fc`. The two edited files are left **uncommitted**. No Git writes were made.
- **Placeholders:** `WT` = the T3 worktree root, `P` = `projects/chirality-piping`, `DT` = `P/apps/desktop/src`, `R` = `…/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30`, `S` = `WT/scratch/i75_t6s`. Evidence is under `_run_records/repair_01/`. The sealed `SHA256SUMS` and `RETURN.md` are untouched; this record has its own sum file, `REPAIR_01.SHA256SUMS`.

## 1. Outcome

1. **Repaired, with an exception to the prescribed rule.** The product `rustLowerExp` now gives the same string as Rust `{:e}` on all **123,607** Rust-computed words: **0 mismatches**. Before the repair there were **6,395**. The words include RV101's nine differing words.
2. **The prescribed 17-digit rule is not sufficient, so the repair generalizes it.** I first implemented the rule as ROOT gave it: when the shortest form has 17 digits, reprint with `toExponential(16)`. It passed the suites and its mutants. My Rust oracle then found that the same tie also occurs at **16 significant digits**, where that rule leaves V8's even choice in place. It missed 2 of my 60,000 random words and 1,148 of 30,000 constructed 16-digit tie words. Example `4308628432e3716a` = 857964921253421.25 exactly: Rust prints `8.579649212534213e14`, the 17-digit rule prints `8.579649212534212e14`. All 1,150 misses are listed with Rust's output in `oracle/form1_misses.tsv`. The repair applies RV101's idea at every length, and at 17 digits it reduces to RV101's rule (§3).
3. **The doc comment is corrected.** The earlier text and RV101's note both said a tie needs 17 digits, which is wrong. The comment now says a tie occurs at 16 or 17 digits and explains the round-trip guard.
4. **Tests use Rust-computed vectors.** The `{:e}` test now has **69 word→text vectors**, each one Rust's own `{:e}` output. None was produced by a Python or JavaScript printer. They are RV101's nine, 17- and 16-digit ties in both rounding directions, powers of two, and the former literal edge cases, now keyed by word. Two more checks confirm that a 17-digit and a 16-digit tie bound appear correctly in a full `class_disclosure` message.
5. **Suites:** `tsc --noEmit` is clean. The T6S files pass 430/430 (10 files). The full desktop suite passes **3,590/3,590** (141 files). Compared test by test with base `c1bfc460fc`, the counts are the same as RETURN.md's: 44 added, 6 removed, 0 status changes, 3,546 unchanged. Compared with the pre-repair candidate, only the `{:e}` test changed, and only its name. **Mutants:** 9 of 9 killed. They include RV101's rule (S02) and the dropped round-trip guard (S03).

## 2. Changed files (uncommitted, in `WT/t6-outputs`)

| File | sha256 at `2033260c57` | sha256 now | Change |
|---|---|---|---|
| `DT/features/results/retainedPrecisionDisclosure.ts` | `9ee23d2678426f6c13c2c89032bb2c8ada024c1cd954dd6acb81d04d5976b8b9` | `cb7c7e225c0575b742957a4c0e6f8f4548e5d682fc745867834d3c4dae361fc1` | `rustLowerExp` code and doc comment only |
| `DT/features/stress-neutral/retainedPrecisionStressNeutral.test.tsx` | `8e9766048c1d6a0da4a28917b95ff8f6cd6711e91eb425901718d4a145f87a66` | `dd65f2b61f3176a671dbdbb088958141c5ff0b41abcccd006d41d6c614d7aa47` | the `{:e}` test: Rust-computed vectors, renamed |

The diff against the branch head is `_run_records/repair_01/repair_01.diff` (sha256 `488a754d5ba1d1075027f4e4efcba4f098d5aeade55b914a1afa5c9b31b00e38`). No other file in the tree changed. `git status` shows only these two files and ROOT's `P/node_modules` symlink.

## 3. The repair

```ts
const magnitude = Math.abs(value);
const shortest = magnitude.toExponential();
const nearest = magnitude.toExponential(shortest.split("e")[0].replace(".", "").length - 1);
return `${sign}${(Number(nearest) === magnitude ? nearest : shortest).replace("e+", "e")}`;
```

`rustLowerExp` keeps its finiteness refusal, sign handling and `e+` normalization. It is called only through `retainedClassDisclosure`, so only successor `absolute_verified` rows use it: the result-export derivative and validator, and the stress-neutral class finding. No other route reaches it.

**Why this is correct.** Let `s` be V8's shortest form with `n` digits. Let `r` = `toExponential(n − 1)`, the `n`-digit decimal nearest the value. On an exact tie ECMAScript takes the larger candidate (`toExponential` step: "pick the e and n for which n × 10^(e–f) is larger"). V8's shortest form is the closest `n`-digit decimal inside the round-trip interval, with ties going to even. So if `r` round-trips and `r ≠ s`, `r` and `s` must be equally close, which is a tie. In that case `r` is the larger candidate, which is the one Rust prints. Every one of the 6,395 pre-repair differences is a tie of this kind. The check is in exact arithmetic: the value lies exactly halfway between the two printed decimals, V8's choice is the lower, even one, and Rust's is the larger (`results/difference_check.json`, `tools/difference_check.py`). If `r` does not round-trip, the shortest form `s` stands, as it does in Rust. This happens on **90 of the 12,580** power-of-two words: below a power of two the round-trip interval is half as wide, so the nearer decimal can fall outside it (example `0060000000000000` = 2^-1017: shortest `7.120236347223045e-307`, nearest 16-digit `7.120236347223044e-307`, which does not round-trip; Rust prints the shortest). Dropping this guard is mutant S03, and it is killed.

**Why ties occur only at 16 or 17 digits.** For two `n`-digit candidates spaced `10^k` to tie, both must lie within half an ulp of the value, so `10^k ≤ ulp`. A normal value `v` satisfies `v ≥ 2^52·ulp ≥ 2^52·10^k`, and an `n`-digit value at spacing `10^k` satisfies `v < 10^(n+k)`. Together these give `10^n > 2^52 ≈ 4.5·10^15`, so `n ≥ 16`. Subnormals cannot tie: an exact tie there would need candidates spaced below 10^-1021, which means hundreds of digits, and no shortest form has more than 17. The data agree: the pre-repair differences occur only at 16 digits (1,150) and 17 digits (5,245). At `n = 17` every correctly rounded 17-digit form round-trips, so the repair there is exactly RV101's rule.

## 4. Rust-computed vectors (the oracle)

- **Oracle.** A scratch crate `i75_rust_exp` with no dependencies (`oracle/rust_exp/`). For each word it writes `format!("{:e}", f64::from_bits(w))`, the formatting Rust `class_disclosure` uses for b. It was built and first run through `WT/tools/t3_cargo.sh` (`run --locked --offline --release`, `CARGO_BUILD_JOBS=4`, target `WT/targets/i75-t6s`, rustc 1.97.1). The job waited behind ROOT's DEC-025 job, then ran: START 16:25:56Z, END rc=0 16:25:58Z (`oracle/rust_job.txt`). The edges, `ties16` and `pow2` lists ran on the same built binary directly. That needed no cargo and no compile. Binary sha256 `dc841850…`.
- **RV101's nine.** The oracle's output equals RV101's recorded Rust column in `exp_differences.txt` exactly (`oracle/rv101_recorded.tsv`).
- **RV101's 81,332 words could not be rerun.** Its evidence records only the nine differences and the counts, not the full word lists. My lists are separate, seeded, and reproducible from `oracle/gen_words.py` and `oracle/gen_words_2.py`. The hashes of every list and Rust output are in `oracle/word_lists.sha256`.

| List (seeded) | Words | Pre-repair ≠ Rust | RV101's 17-digit rule ≠ Rust | **Repair ≠ Rust** |
|---|---:|---:|---:|---:|
| `random`: 30,000 normal, 28,000 subnormal, ±0, the 9 corpus/milestone bounds, random finite | 60,000 | 7 | 2 | **0** |
| `ties`: exact 18-digit decimals ending in 5 (17-digit ties), both signs | 21,000 | 5,227 | 0 | **0** |
| `rv101_differences`: RV101's nine | 9 | 9 | 0 | **0** |
| `edges`: the former literal cases | 18 | 0 | 0 | **0** |
| `ties16`: exact decimals of ≤17 digits ending in 5, m·2^-k (16-digit ties: 2,280, of which V8 printed the lower in 1,148) | 30,000 | 1,148 | 1,148 | **0** |
| `pow2`: every power of two, both neighbours, both signs | 12,580 | 4 | 0 | **0** |
| **Total** | **123,607** | **6,395** | **1,150** | **0** |

The product function's figures come from the probe (`results/tie_probe_summary.json`): it imports the product `rustLowerExp` in a scratch copy of the tree (`git archive 2033260c57` with the two changed files) and records 0 mismatches and no `+` in any output. The RV101-rule column comes from `results/forms_summary.json` (`tools/forms.mjs`).

**The test's vectors** (`oracle/test_vectors.tsv`): 69 distinct words, each matched against the oracle output with 0 exceptions.

| Group | Count | Rounding |
|---|---:|---|
| RV101's nine (17-digit, lower candidate even) | 9 | Rust rounds up; V8 printed the lower |
| Further 17-digit ties, lower candidate even | 8 | same |
| 17-digit ties, lower candidate odd | 6 | Rust and V8 both print the upper |
| Exact 18-digit decimals whose shortest form is shorter | 4 | no tie at that length |
| 16-digit ties, lower candidate even | 8 | Rust rounds up; V8 printed the lower |
| 16-digit ties, lower candidate odd | 6 | Rust and V8 both print the upper |
| Powers of two where the nearer decimal does not round-trip | 6 | the shortest form stands |
| 17-digit ties at a power of two and just above one | 4 | Rust rounds up |
| Edges: 1, 1.5, 1234.5, 1e21, 2^53, 0.5, the least subnormal, both sides of the subnormal/normal boundary, the greatest finite, ±0, −1.5, five corpus bounds | 18 | Rust's strings, identical to the former literals |

Both signs appear in every tie group. The same test checks that `class_disclosure` messages print `b = 1.6900607208313233e15 N` and `b = 8.579649212534213e14 m`, and it keeps the NaN/Infinity refusal.

## 5. Checks

The heavy runs (the probe, the mutants, tsc, the T6S files and the full suite) ran as one job under `lockf -k WT/guard/cargo_job.lock`, from 16:46:10Z to 16:49:00Z (`results/times.txt`).

- **Probe:** 2 files, 16/16 passed. 0 of 123,607 words mismatched.
- **Mutants** (`tools/mutants_repair_01.py`; each is one exact single-occurrence replacement in the probe copy, restored and byte-compared after its run; tests: `retainedPrecisionStressNeutral` and `retainedPrecisionResultExport`). The control passes 32/32.

| Id | Mutant | Result |
|---|---|---|
| S01 | repair reverted: the shortest form only | killed (`43180467b3a7ed6d`) |
| S02 | RV101's rule: only a 17-digit shortest form reprinted | killed (`4308628432e3716a`, 16-digit tie) |
| S03 | round-trip guard dropped | killed (`0060000000000000`, power of two) |
| S04 | one digit too many (`toExponential(n)`) | killed (10 tests, including both golden parities) |
| S05 | digit count includes the point | killed (10 tests) |
| S06 | the signed value reprinted | killed (negative tie `c30c9bbde3da0f6a`) |
| S07 | guard compared with the signed value | killed (negative tie) |
| S08 | `e+` left unnormalized | killed |
| S09 | negative-zero sign dropped | killed (`-0e0`) |

- **tsc:** `tsc --noEmit -p .` in the desktop app returns rc 0 with no output.
- **The ten T6S test files:** `retainedPrecisionStressNeutral`, `outputPolicy`, `retainedPrecisionOutputRefusal`, `loadReferenceOutputRefusal`, `StressNeutralExportPanel`, `ResultExportPanel`, `retainedPrecisionResultExport`, `resultExportAdapter`, `physicsResultExport` and `retainedPrecisionIntegration` pass **430/430**. This includes byte parity with I76's Rust goldens (`958df02e…`, `3f9905ad…`).
- **Full desktop suite:** 141 files, **3,590/3,590** (`suites/full_suite_summary.txt`, `suites/repair_tests.tsv`).
  - Against base `c1bfc460fc` (the base list from the checkpoint run, `_run_records/suites/base_tests.tsv`, sha256 `b219b0b8…`, 3,552 passed): 44 added, 6 removed, 0 status changes, 3,546 unchanged (`suites/compare_base_repair.txt`). These are the same counts as RETURN.md's.
  - Against the pre-repair candidate (`_run_records/final/suites/final_tests.tsv`): 1 added and 1 removed, which is the `{:e}` test's rename, plus 0 status changes and 3,589 unchanged (`suites/compare_final_repair.txt`).
- **The first form, for the record** (`results/first_form/`): with RV101's rule alone, the suites passed 3,590/3,590 and its own 9 mutants were killed. Its probe failed on the 2 random 16-digit ties (`tie_probe_summary_form1.json`), which is what led to the general form.

## 6. Host rules and cleanup

- Edits were confined to the two named files. Nothing was committed, and no Git writes were made (reads used `GIT_OPTIONAL_LOCKS=0`). There were no installs and no DEC-025, native or solver jobs. Cargo ran only through `WT/tools/t3_cargo.sh`. Nothing was written to the system temp directory.
- At return, the copied wasm assets were removed from `WT/t6-outputs/P/apps/desktop/public`, and the scratch directory `S` (the probe tree, oracle crate and word lists) was deleted. The oracle's build target `WT/targets/i75-t6s` was left in place. ROOT's `P/node_modules` symlink was left in place.
- `SHA256SUMS` still verifies 56/56 (sha256 `4118a5dd…`), and `RETURN.md` is unchanged (`2ccafccb…`).

## 7. For ROOT and RV101

- **RV101 to confirm:** (a) the general form in place of the suggested 17-digit rule, including the round-trip guard; (b) the 16-digit tie evidence (`oracle/form1_misses.tsv`, 1,150 words with Rust's output). RV101's own words appear to have contained no 16-digit tie in which V8 chose the lower candidate. A rerun of its oracle with a 16-digit tie list, such as `ties16` from `gen_words_2.py` (seed 75016), would test this repair independently.
- I made no other change. The Rust side (`derivative::class_disclosure`) is the reference and was not touched.
