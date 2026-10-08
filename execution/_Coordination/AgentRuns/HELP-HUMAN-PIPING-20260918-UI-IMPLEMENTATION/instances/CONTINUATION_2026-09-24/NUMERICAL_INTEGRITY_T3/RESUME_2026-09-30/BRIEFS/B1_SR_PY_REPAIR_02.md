# B1-SR-PY repair round 2 (I91): the three-reader alignment set, and RV113's S-4

Read `R/BRIEFS/B1_COMMON.md` and `R/BRIEFS/B1_SR_PY.md` again first; their rules still hold. Then read:
- RV113's review of SR-PY, `R/REVIEW_RV113/rvr_sr_py_01/REVIEW.md` (`d8611e59…`), especially §10 and its `evidence/fg/`;
- RV113's SR-TS review, `R/REVIEW_RV113/rvr_sr_ts_01/REVIEW.md` (`d44dec19…`): S-1, N-1 and N-2;
- RR "RV113's three returns verified; I3 made at `2ba2f81863`; the three-reader alignment set ruled". Its items 1–6 are your specification.

## What to change, in PY's lane only (`P/core/analysis_runs/retained_precision.py` and its tests, as SR-PY's fence)

1. **(f) and its family, at G3 COVERAGE:** `material_bases[].index` and `sources[].index` equal their positions, `case_indices` has no duplicates and is in range, and a source's owner is its own case.
   - **Add** the `material_bases[].index` check, which PY lacks today, even bound.
   - **Move** the others from G8 PREPARATION.
   - **The ordinary attempt's basis reference goes to G5 ATTEMPT_MISMATCH.**
   - G8 keeps only the facts derived from the invocation.
2. **(g), at G8 INVOCATION_MISMATCH,** in the model-scope check:
   - no `reference_configurations` member, null included;
   - `pressure_contract` absent or null;
   - `combinations` and `components` absent or `[]`.

   RV113 §10.2 gives the three PY expressions.
3. **The C2 cause table at G5 ATTEMPT_MISMATCH,** in the ordinary class, exactly as the RR ruling's item 3 states. That is TS's form, with the `precondition` keying.
4. **The transport header check moves from G7 to G2.** The preview-physics metadata check stays at G7.
5. **S-4:** add the test row that kills RV113's mutant Q17 (`not_required`'s `product_attempt_ref`-null conjunct), as SR-RS's repair did.
6. **N-1 and N-2:**
   - make the N6(a) docstring state the header's gate after item 4;
   - add a unit test for any (4b) conjunct whose mutant only the predicate's unit test kills, where cheap.
7. **S-2:** in your record, correct REPAIR_01's statement that (b) and (c) give the same gate and code in all three readers. State each reader's actual first failure for its three inputs, as RV113 found them.

**No change to any 07m verdict** is allowed. If one would change, stop and return.

## Evidence

- **The census over 07m:** 0 changes against your repair 01 head `11cc14e3e6` on all three verdicts, and 0 misses against the corpus's Python expectations.
- **RV113's probes** (`evidence/`; its 103 probes, plus the (f)/(g) table and the C2 probes):
  - PY now gives each item's ruled gate and code;
  - state where PY then equals TS's present result (TS already has items 1 and 3) and where RS and TS will move in their own rounds.
- **The Python suites:** against `11cc14e3e6`, the only differences are your added or changed tests, listed.
- **Mutants:** one per new or moved check, plus Q17, each killed by an assertion.
- **pytest under `P/tests`:** set the checked binaries (`OPENPIPESTRESS_CHECKED_JSON_BIN`, `OPENPIPESTRESS_BINARY64_JSON_BIN`, `OPENPIPESTRESS_UNITS_BIN`) to your own builds, or run under the lock.

## Host

- As B1_COMMON: every cargo goes through `WT/tools/t3_cargo.sh`, and heavy pytest runs under `/usr/bin/lockf -k WT/guard/cargo_job.lock`.
- **Waits:** one wait per job, ending when the job's process has gone. Stop your own waits before you return.
- **Paths:** absolute paths only.
- **Records:**
  - no symlink, and no folder named `build`;
  - remove the `hostname` attribute from any pytest junit output;
  - screen with the strict pattern and the machine's host name, decompressing `.gz` files.

## Output

- **Commit** on `b1-p`'s branch in `WT/b1-p`, with a truthful message. ROOT pushes.
- **The record:** `R/I91/b1_sr_py_01/REPAIR_02.md`, with `_run_records/repair_02/` and `SHA256SUMS.repair_02`.
- **Budget:** 3–5 h.
- **End your turn with:**
  - the new head;
  - REPAIR_02.md's sha256;
  - per item, the change and its evidence;
  - the census and suite deltas;
  - the mutants.
