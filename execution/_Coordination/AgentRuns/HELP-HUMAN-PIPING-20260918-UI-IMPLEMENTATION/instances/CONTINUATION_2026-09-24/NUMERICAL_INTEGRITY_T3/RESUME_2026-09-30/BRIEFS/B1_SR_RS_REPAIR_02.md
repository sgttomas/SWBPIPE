# B1-SR-RS repair round 2 (I90): the three-reader alignment set

Read `R/BRIEFS/B1_COMMON.md` and `R/BRIEFS/B1_SR_RS.md` again first; their rules still hold. Then read:
- RV113's SR-PY review, `R/REVIEW_RV113/rvr_sr_py_01/REVIEW.md` (`d8611e59…`): §10 and `evidence/fg/`, which carry RS's columns;
- RV113's SR-TS review, `R/REVIEW_RV113/rvr_sr_ts_01/REVIEW.md` (`d44dec19…`): S-1, N-1 and N-2;
- RR "RV113's three returns verified; I3 made at `2ba2f81863`; the three-reader alignment set ruled". Items 1–4 are your specification.

SR-RS at `b5cb7faaeb` is merged into `b1` at I3. **This round continues on `b1-r`**, and ROOT merges it into `b1` again before I4.

## What to change, in RS's lane only (`RE/src/retained_precision.rs`, `RE/src/source_blocks.rs` if needed, and their tests, as SR-RS's fence)

1. **(f) and its family move from G8 PREPARATION to G3 COVERAGE:** `material_bases[].index` and `sources[].index` equal their positions, `case_indices` has no duplicates and is in range, and a source's owner is its own case. RS currently checks the owner at G5 ATTEMPT; move it to G3.
   - **The ordinary attempt's basis reference goes to G5 ATTEMPT_MISMATCH.**
   - G8 keeps only the facts derived from the invocation.
2. **(g), at G8 INVOCATION_MISMATCH,** in the model-scope check:
   - no `reference_configurations` member, null included (RS already reads PP's `is_authored`);
   - `pressure_contract` absent or null;
   - `combinations` and `components` absent or `[]`. Replace `list(&model["combinations"]).is_empty()` with "absent or an empty array", and do the same for `components`.
3. **The C2 cause table at G5 ATTEMPT_MISMATCH,** in the ordinary class, exactly as the RR ruling's item 3 states: TS's form plus the `precondition` keying. RS has no C2 branch rule today.
4. **The transport scope:** the header check stays at G2, and **add the preview-physics metadata check at G7**, as PY and TS have it.

**No change to any 07m verdict** is allowed. If one would change, stop and return.

## Evidence

- **The census over 07m:** 0 changes against `b5cb7faaeb` on all three verdicts, and 0 misses against the corpus's Rust expectations.
- **RV113's probes** (its 103, plus the (f)/(g) table and the six C2 probes): RS gives each item's ruled gate and code. State where RS then equals TS's present result.
- **The RS suites:** against `b5cb7faaeb`, the only differences are your added or changed tests, listed.
- **Mutants:** one per new or moved check, each killed by an assertion.
- `cargo fmt` is clean on your hunks.

## Host

- As B1_COMMON: every cargo goes through `WT/tools/t3_cargo.sh`, and direct test binaries run under the lock. I85 (on `b1`) and I91 (on `b1-p`) share the lock.
- **Waits:** one wait per job, ending when the job's process has gone. Stop your own waits before you return.
- **Paths:** absolute paths only.
- **Records:**
  - no symlink, and no folder named `build`;
  - remove the `hostname` attribute from any junit output;
  - screen with the strict pattern and the machine's host name, decompressing `.gz` files.

## Output

- **Commit** on `b1-r`'s branch in `WT/b1-r`, with a truthful message. ROOT pushes.
- **The record:** `R/I90/b1_sr_rs_01/REPAIR_02.md`, with `_run_records/repair_02/` and `SHA256SUMS.repair_02`.
- **Budget:** 3–4 h.
- **End your turn with:**
  - the new head;
  - REPAIR_02.md's sha256;
  - per item, the change and its evidence;
  - the census and suite deltas;
  - the mutants.
