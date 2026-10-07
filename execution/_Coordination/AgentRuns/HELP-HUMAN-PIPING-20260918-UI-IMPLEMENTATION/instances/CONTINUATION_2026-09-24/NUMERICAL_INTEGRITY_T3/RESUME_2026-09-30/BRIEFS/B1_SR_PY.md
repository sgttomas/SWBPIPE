# B1-SR-PY: the Python reader (R-D38, F-1 text B, G8 per case, G5's `not_required` rule), with RV108's Python items

Read `R/BRIEFS/B1_COMMON.md` first (sha256 `2d170307516b98c40e8aa2dcf352cf11a13dbdd29aeddd78a4c39f3f15eb2c75`); its rules hold.

**The specification** is PLAN_v2 §2.4: the common part and SR-PY. The contract is DESIGN_v2 §2 (R-D38 with (4b), m1–m8), §3.2 (text B, P1–P4) and §3.3 (G8's per-case loop with `RETAINED_PRECISION_PREPARATION_MISMATCH`, and G5's `not_required` rule).

**Your worktree:** `WT/b1-p`, branch `codex/piping-t3-b1-p-20261007`, cut by ROOT from I1 (`262bd687f0`), which carries B6's 07m and its F-U6b-2 transport validator. You are I-PY; you will also own SC (corpus 07n) later. SP, SA and SR-RS run beside you in `WT/b1`, `WT/b1-a` and `WT/b1-r`; touch none of their files.

## The fence

- **PY** (`P/core/analysis_runs/retained_precision.py`) and `P/tests/test_retained_precision_contract.py`, as PLAN_v2 §1.
- **Extended by ROOT for RV108 N1** (RR "SR-PY dispatched; the N1 guard in `_source_contract` ruled in"):
  - `P/core/analysis_runs/compatibility.py`'s `_source_contract`, but only its enum membership tests. These are `numerical_quality.status`, and each case's `solve_quality`, `structural_status`, `model_matrix_fidelity` and `accuracy_evidence`.
  - The tests for it go in `P/tests/test_retained_precision_carriers.py` or the contract test file.
  - **The rule:** every input that did not raise there reads exactly as before.
- R8's stops apply to everything else, including any other base-reader change.

## First: the cascade census (R5)

Run the aligned reader over 07m's 294 mutations and 28 must-pass entries.
- **Expected: zero changes,** apart from the entries RV108's N1 and N2 would move, if any exist. List those.
- **Any other change stops the work.** Return under R5.

## Then

- **(4b) in `_g5_stages`, and G8's requested mode and P1–P4, per case.**
- **D38's obligation** `[r1: N-6]`:
  - list every check in PY that assumes a prepared source has a Call or a Run;
  - relax each to (4b), or show it does not apply.

  The list goes in your RETURN.
- **RV108 N1** (`R/REVIEW_RV108/b6_01/REVIEW.md`). A list- or dict-valued enum is unhashable, so `_source_contract`'s membership tests raise `TypeError`. That reaches the G7 fallback, and escapes callers that catch only `ValueError`, such as the v0.3 packager's validator.
  - Guard the type, so that Python gives its base header code (`SOURCE_NUMERICAL_QUALITY_INVALID` or `SOURCE_NUMERICAL_CASE_INVALID`), as Rust and TS do.
  - Pin it in the retained path and through a `ValueError`-only caller.
- **RV108 N2.** A case without `solve_quality` reaches a `KeyError` caught at G5 as `PRODUCT_ATTEMPT_MISMATCH`. Rust and TS give `ATTEMPT_MISMATCH`. Align Python by an explicit check, not by the fallback.
- **RV108 N6(a).** `validate_retained_precision_transport`'s docstring calls it "the twin" of Rust's and TS's. It is exact on G0–G2. At the base step it runs both Rust's check and TS's. Say so.
- **SP's several-notice bytes (T-12)** come later, from SP. Your PY check of them is part of SC, after SP.

## Acceptance

- The census holds.
- PY passes 07m in full; every count change is an added test.
- G8 and G5 behave per DESIGN §3.3 on synthetic n-case receipts.
- D38's list is complete.
- N1, N2 and N6(a) are done, with tests.
- **Every input that did not raise in `_source_contract` reads exactly as before.** Show it with a differential over the corpus and your own probes, base `262bd687f0` against your head.
- **The Python suites,** as I83 ran them for B6 (I69's set plus the retained files), run base against head, test by test.
- **Mutants, killed by assertions:**
  - the per-case G8 loop;
  - each of G5's three dropped conjuncts restored;
  - each relaxed D38 check restored;
  - the P2–P4 checks;
  - the N1 guard removed;
  - the N2 check removed.

**Return** with:
- the head and commits;
- the census;
- D38's list;
- the evidence per item;
- the differential;
- the suite differences;
- the mutants;
- anything for ROOT.

RV-R reviews SR-PY with SR-TS (I4).

## Host

- pytest under `P/tests` either sets `OPENPIPESTRESS_CHECKED_JSON_BIN` and `OPENPIPESTRESS_UNITS_BIN` to your own builds, or runs under `/usr/bin/lockf -k WT/guard/cargo_job.lock`. Every cargo goes through `WT/tools/t3_cargo.sh`.
- **Waits:** one wait per job, ending when the job's process has gone. Stop your waits before returning.
- **Writes:** absolute paths for every write. Name no record folder `build`.
- Never kill another job.
- **Scratch** goes in `WT/scratch/<id>_b1_sr_py/`, with targets in `WT/targets/<id>-b1-sr-py/`.

**Records:** `R/<id>/b1_sr_py_01/`. **Budget:** 5–7 h, plus about 1 h for N1, N2 and N6(a).
