# I89 B1-SA follow-up 01: the parked-slot patch on `b1-a`

TASK (Type 2), I89, role I-A, for ROOT (HELP_HUMAN, Agent 0). I made no delegation. 2026-10-07 UTC.

**What I did, and on what basis.** This carries out RR "I89's SA verified and ruled; the parked-slot patch goes on `b1-a`; the seam saturates; RV-Q round 1 dispatched as RV112", ruling 1, as ROOT's message instructed:
- (a) merge `b1` at SP's `56c5579f07` into `b1-a` with `--no-ff`;
- (b) commit `parked_slots.diff` as its own commit;
- (c) rerun PP `--lib` and the law tests; show the negative check and c = 1 identity;
- (d) write this record.

Ruling 2 (the seam saturates) is I85's edit. My reading side already handles it (RETURN §7).

**Placeholders and limits** are as in RETURN.md. Every cargo job went through `WT/tools/t3_cargo.sh` (`--locked --offline`, registered, `RUSTFLAGS` unset). There were 4 jobs, one wait each; no process or wait of mine remains. The Git writes are my two commits on `codex/piping-t3-b1-a-20261007`, and nothing was pushed. SP's files were not edited.

## 1. Head and commits

**Head `9812c83deddc34eaba0bcb8e4d71684fded900f5`** on `codex/piping-t3-b1-a-20261007`.

| Commit | Content |
|---|---|
| `77f4391a85ae41d5c6638f173c050798d8149cb9` | `--no-ff` merge of SP's `56c5579f07` (parents `6b62606778`, `56c5579f07`). There was no conflict: SP's 6 files and SA's 3 are disjoint |
| `9812c83deddc34eaba0bcb8e4d71684fded900f5` | The parked-slot patch, and nothing else |

**The patch commit's contents:**
- **Files.** It touches only `PP/retained_memory.rs` and `PP/retained_memory_law_tests.rs` (+29 / −2).
- **Equality with the recorded patch.** Its `+`/`−` lines equal `_run_records/i2_preview/parked_slots.diff`, line for line (`followup_01/patch_commit.diff`).
- **Hashes at the head:** `retained_memory.rs` is `16cc41b9…3351`, as in the I2 preview; the law tests are `7944c4ef825533d85aa59e1f66a49a05e4b4e17d5559de71362531d93e2f09a2`.

**The code.**
- `retained_error_text` now folds `parked_cases()`' `error` and `observable_error` onto the capture's own fields.
- The bound C·(3m + 1)·Text(err) is unchanged; its check against SP's producer stays in phase 4.
- **The new law test `b1_sa_retained_error_text_reads_every_case_slot`:** a two-case milestone run parks case 0, giving (`cases_seen()`, `parked_cases().len()`) = (2, 1). Texts of 40 and 9 B go in the parked slot (`with_case(0, …)`) and 7 and 3 B in the capture's own fields, and G-C must read 59. Both modes.

**The tree.** The head's P (without `P/execution`) is byte-equal to the I2 preview tree `S/i2` of RETURN §8 (`diff -r` empty).

## 2. Results on the head (`_run_records/followup_01/`)

| Check | Result |
|---|---|
| Build identity | `identity_carries_every_key_in_order` passes. The printed identity is byte-equal to `REGISTERED_PROFILES[0].identity`: **registered** (`identity.txt`) |
| PP `--lib`, registered | **562 passed, 1 failed (the known Mac `t13` only), 11 ignored.** Outcome for outcome equal to the I2 preview's run (`lib.outcomes` against `i2_preview_lib.outcomes`) |
| Against I1's `--lib` (from RETURN's `base_reg_pp`) | **+13 tests, all ok:** SA's 8 `b1_sa_*`, the new parked-slot test included, and SP's 5 `b1_sp_*`. No test removed, and no other outcome changed (`lib_vs_i1.diff`) |
| The law tests | **All 45 `retained_memory::law_tests` pass,** the 8 `b1_sa_*` among them |
| The new test against the unpatched reader | **Fails at its assertion:** `left: 10, right: 59` ("SparseInteractive: both slots"). Run in `S/i2`, which is byte-equal to the head, with `retained_memory.rs` set back to SA's head `6b62606778` version; the patch was then restored and checked by sha256 `16cc41b9…` (`unpatched_parked_test.log`) |
| c = 1 identity | **Holds.** The three pin tests pass, and their 6 documents (milestone and L = 0, both modes) are byte-identical to I1's (RETURN §5.3's `S/pins/base`) and equal the fixtures (`pins.log`, `pins_cand2.sha256`, `followup_01.out`) |

## 3. Erratum: the placeholders in RETURN's committed `_run_records/`

**What went wrong.** RETURN's `sanitize.py` was I85's, copied one directory deeper (`S/scripts/` rather than `S/`). It took `WT` to be three levels above itself, which is `WT/scratch`. So in the records ROOT committed at `bc37d43a0a`:
- **`WT/…` in those logs and scripts meant `WT/scratch/…`.** For example, `WT/i89_b1_sa/mut/…` stood for `WT/scratch/i89_b1_sa/mut/…`.
- **Other WT paths appeared in a home-relative form of WT's path** (the home directory abbreviated, then the rest of WT's absolute path), not as `WT/…`.
- **The virtual environment appeared in a home-relative form of VENV's path,** not as `VENV`.

**Extent.** No absolute machine path was in those files, because the home directory was always abbreviated. But those home-relative paths were not placeholder-only. RETURN.md itself is hand-written and uses placeholders correctly.

**The fix in this follow-up.** The corrected `sanitize.py` takes WT four levels up and asserts it (`tools/` and `guard/` exist there). It also refuses any home-relative path that is left over. It is in `_run_records/followup_01/sanitize.py`, and every follow-up record was written with it.

On the same probe line, the old version gave the home-relative forms of `WT/b1-a/x` and `VENV/bin/python`, and `WT/i89_b1_sa/y` for a scratch path. The new one gives `WT/b1-a/x`, `WT/scratch/i89_b1_sa/y` and `VENV/bin/python`.

**Since redacted in place** (at ROOT's request, after this follow-up was verified):
- the 46 affected committed files were rewritten with the corrected sanitizer, and `SHA256SUMS` was regenerated over the same file set;
- this section was reworded so that it no longer reproduces the bad form.

`REDACTION_01.md` lists every rewritten file with its old and new sha256.

## 4. Records

`_run_records/followup_01/`:
- `commits.txt` and `patch_commit.diff`;
- `followup_01.sh` and `followup_01.out`;
- `identity.log` and `identity.txt`;
- `lib.log`, `lib.outcomes`, `i2_preview_lib.outcomes`, `i1_lib.outcomes` and `lib_vs_i1.diff`;
- `pins.log` and `pins_cand2.sha256`;
- `unpatched_parked_test.log`;
- `sanitize.py`;
- `cargo_jobs_followup_01.log`.

**SHA256SUMS.followup_01** covers this file and every file under `_run_records/followup_01/`. RETURN's `SHA256SUMS` is unchanged.

**Kept for RV112 until I2:** `S`, including `S/i2` and the full logs, and the targets `WT/targets/i89-b1-sa{,/base,/mut,/stale,/i2}`.

**Budget:** about 20 minutes of agent time (15:25–15:45 UTC).
