# RV108: independent complete-diff review of B6 (the reader items)

TASK (Type 2), an independent reviewer dispatched by ROOT (HELP_HUMAN, Agent 0). ROOT is your return path, and you do not delegate. **You are a fresh instance and wrote none of this change.** Build your own oracles; don't rely on the implementer's.

## The candidate

- **The branch:** `codex/piping-t3-b6-20261007` at `a7de2a918f`, in worktree `WT/b6`. It is three commits over main `bfb26596bf`:
  - `5e38293530`: items 1, 2 and 4 (TS's G7 header codes, corpus 07m, the 07k and 07m slices, Python reading its own expectation);
  - `a79dbd2e4a`: item 3 (Python's transport validator for the retained successor);
  - `a7de2a918f`: id checks in Rust's two new slices.
- **11 files, +535/−71.** The PR will be cut from current main `025c1cf326`. Main's later changes touch none of these files.
- **The implementer's return:** `R/I83/b6_01/RETURN.md` (`4e856c31…`). Read it after forming your own view.
- **The brief it answered:** `R/BRIEFS/B6_READER_ITEMS.md`.
- **ROOT's rulings on it:** RR "I86's SW probe accepted; I83's B6 return verified and ruled; RV108 and I87 dispatched". Item 2's widening and the fixtures reading are confirmed there, and RV78-N1 goes to B3.
- **The sources of the items:** RR:12697 (277's slice), RR:11246 (N-3), RR:9922 and RR:10449 (F-U6b-2), RR:11394 (RV94 N-5), PLAN decision 11 (`R/I61/u8_plan_01/PLAN.md`), and RV92's tampered probes (`R/REVIEW_RV92/u6f_01/`).

## Review, in priority order

1. **TS's G7 header refusal (item 2 and its widening).**
   - For single-defect, hash-consistent sources, TS's new `baseHeaderCode` gives the code that Python's `_source_contract` and Rust's base validators give. Cover every branch and the fallback.
   - **Build your own probe set,** beyond I83's 32. Report each three-way disagreement at head, and say whether the carrier case file declares it.
   - **Nothing TS admitted at base changes.** A statement TS admitted at base must read byte-identically at head. Use a differential over the corpus and your own admitted sources.
2. **Python's transport validator (item 3).**
   - `validate_retained_precision_transport` and the `raw=False` path are the twin of Rust's `validate_transport_metadata` and TS's `validateRetainedPrecisionTransport`. Compare codes over RV92's tampered set and your own tampered transports.
   - **The raw path is unchanged.** Python's raw validation reads the same at base and head over the full corpus.
   - **`compatibility.py`** maps reader failures truthfully.
   - **T6S decision 8 still holds:** Python still refuses successor packages (`T/IMPLEMENTATION/T6S/CHANGE_RECORD.md`).
3. **The corpus and the slices (items 1 and 4).**
   - **07m (`c21112fd…`) is append-only,** except 277's `expected_by_reader.typescript`.
   - Each of entries 286–293 is a single hash-consistent edit, and all three readers give its expected code.
   - The 07k and 07m slices run in all three harnesses and check ids.
   - **Python reads `expected_by_reader.python`** where it is present, and a wrong value is killed.
4. **The declarations tell the truth.**
   - Every declared difference left in the carrier case file is real at head, and its scope sentences are exact.
   - The removed F-U6b-2 and N-3 sentences no longer describe the code.
   - **Two undeclared differences are known and routed to B1's SC:** I83 §7 item 6, and the two-defect order in mutant T9. Report any other undeclared difference you find.
5. **The fence.**
   - The 11 files, and nothing in PP, a D1 crate's `src`, the schemas or the other fixtures.
   - **RV107 A1-N-10:** the edits to `retainedPrecisionIntegration.test.tsx` are only in the declared-difference pins, not in T6S's panel block.
6. **Weakening.** No assertion is dropped. Every removed line is a grown count, a narrowed slice bound, a changed expectation with its reason, a replaced pin or a renamed title. Check the five vitest tests that were removed or renamed.
7. **Tests and mutants.** Write your own mutants, at least one per changed site in TS, Python and the Rust tests, beyond I83's 26. The new tests should fail at base where they should, and pass at head.
8. **The suites, base against head, test by test:**
   - Python: the retained set as I83 ran it;
   - Rust: `result_export`, all targets;
   - vitest: the whole desktop suite;
   - `tsc --noEmit`.
9. **Item 5's checkpoint.** Is I83's account of RV78-N1 accurate? Note any error in the identification that would change ROOT's ruling.

## Host

- **Every cargo goes through `WT/tools/t3_cargo.sh`,** the host-wide T3 lock, with `--offline --locked`. Heavy vitest and pytest runs go under `/usr/bin/lockf -k WT/guard/cargo_job.lock`.
  - pytest under `P/tests` either sets `OPENPIPESTRESS_CHECKED_JSON_BIN` and `OPENPIPESTRESS_UNITS_BIN` to your own builds, or runs under the lock.
  - Other T3 jobs share the lock. Wait for it, and never kill another job.
- **Waits.** Use one wait per job. Every wait loop must also end when the job's process has gone (for example `while kill -0 <pid> 2>/dev/null; do sleep 20; done`), not only on a log line that may never appear. Stop your own waits before you return.
- **Your own copies.** Use a `git archive` of each revision into `WT/rv108/{base,cand}`, with fresh targets under `WT/targets/rv108-*`.
  - For vitest, link `node_modules` and copy the eight wasm assets as I83 did (RETURN.md, "Node and the wasm assets"). Remove both afterwards.
- **Not allowed:** DEC-025, evidence sweeps, native or solver jobs, installs, and Git writes.
- **Scratch** goes in `WT/scratch/rv108_b6_01/`, and so does `TMPDIR`. Nothing goes to the system temp directory.
- Delete your copies and targets afterwards.

## Output

- **The report:** `R/REVIEW_RV108/b6_01/REVIEW.md`, with `evidence/` and SHA256SUMS, placeholder paths only. It contains:
  - a verdict, PASS or FAIL;
  - counts of BLOCKING, SHOULD-FIX and NOTE findings;
  - a findings table with path, evidence and remedy;
  - a section per item.
- **If the host's write guard refuses a write into NUM,** write to `WT/scratch/rv108_b6_01/records/` and say so.
- **Budget:** 4–6 h.
- **End your turn with:**
  - the verdict;
  - the counts, with one line per finding;
  - the report's sha256;
  - anything ROOT must rule on.
