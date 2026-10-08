# B1-SR-TS repair round 1 (I92): the three-reader alignment set

Read `R/BRIEFS/B1_COMMON.md` and `R/BRIEFS/B1_SR_TS.md` again first; their rules still hold. Then read:
- RV113's SR-TS review, `R/REVIEW_RV113/rvr_sr_ts_01/REVIEW.md` (`d44dec19…`): S-1, N-1 and N-2;
- RV113's SR-PY review, `R/REVIEW_RV113/rvr_sr_py_01/REVIEW.md` (`d8611e59…`): §10, which carries TS's columns;
- RR "RV113's three returns verified; I3 made at `2ba2f81863`; the three-reader alignment set ruled". Items 2–4 are your specification. TS already has item 1's placements.

## What to change, in TS's lane only (`retainedPrecision.ts` and its tests, as SR-TS's fence)

1. **(g), at G8 INVOCATION_MISMATCH,** in `invocationBinding`'s model-scope check:
   - no `reference_configurations` member, null included;
   - `pressure_contract` absent or null;
   - `combinations` and `components` absent or `[]`.
2. **The C2 cause table (N-2),** exactly as the RR ruling's item 3 states. Keep your present form, and **add the `precondition` keying:**
   - `caller` → `caller_not_qualified`;
   - `resource_admission` → `resource_admission_not_available`;
   - `upstream_no_wrap` → `upstream_no_wrap_not_established`;
   - `capture` and `source_family` → `source_unavailable`.

   The `receipt_failure` set form stands.
3. **The transport scope (N-1):** **add the header check at G2**, with the same code RS uses there. The preview-physics metadata check stays at G7.
4. **Item 1** is already TS's placement: (f)'s family at G3, with the ordinary attempt's basis at G5. Confirm it with RV113's (f) probes, and change nothing unless a probe disagrees with the ruling.

**No change to any 07m verdict** is allowed. If one would change, stop and return.

## Evidence

- **The census over 07m:** 0 changes against `7e47e51b5d` on all three verdicts, and 0 misses against the corpus's TS expectations.
- **RV113's probes** (its 103, the (f)/(g) table and the C2 probes): TS gives each item's ruled gate and code.
- **vitest and tsc:** against `7e47e51b5d`, the only differences are your added or changed tests, listed. vitest runs under the lock.
- **Mutants:** one per new check, each killed by an assertion.

## Host

- As B1_COMMON: heavy vitest runs under `/usr/bin/lockf -k WT/guard/cargo_job.lock`, and every cargo goes through `WT/tools/t3_cargo.sh`. Other implementers share the lock.
- **Waits:** one wait per job, ending when the job's process has gone. Stop your own waits before you return.
- **Paths:** absolute paths only.
- **Records:** no symlink, and no folder named `build`. Screen with the strict pattern and the machine's host name, decompressing `.gz` files.

## Output

- **Commit** on `b1-t`'s branch in `WT/b1-t`, with a truthful message. ROOT pushes.
- **The record:** `R/I92/b1_sr_ts_01/REPAIR_01.md`, with `_run_records/repair_01/` and `SHA256SUMS.repair_01`.
- **Budget:** 2–3 h.
- **End your turn with:**
  - the new head;
  - REPAIR_01.md's sha256;
  - per item, the change and its evidence;
  - the census and suite deltas;
  - the mutants.
