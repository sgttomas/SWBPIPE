# B1: what every B1 assignment shares

Each assignment is a TASK (Type 2) under ROOT (HELP_HUMAN, Agent 0). ROOT is the return path, and none delegates. **You are a fresh instance.** Your ID names a records folder and a role, with no memory of earlier sessions. Cite the records; assume nothing beyond them.

## The specification

B1's plan is `R/I84/b1_plan_01/PLAN_v2.md` (sha256 `c85786b704805311485b44ba27c8826ca278c1237d92010f9f997dbf3c919be0`). It is accepted at R1 with RV107's two later amendments (RR "B1's PLAN_v2 accepted; RV107's A1 amendments; phase 1 dispatched"):
- **A1-S-1 (the challenge, SQ):** every run that did W1 work is bounded by the overall maximum. Each run's furthest phase comes from the witness-driver run of the same input, mode and build, and is recorded in `RSS_TIME.md`.
- **A1-S-2 (SP and SA):** SP's multi-case tests need SA's `LOAD_CASES` and D1.4 change, so I2 comes before SP's R3′ evidence of multi-case custody. Until I2, SP's multi-case tests return `Domain`.

**The basis:**
- the contract: `R/I78/b0_contract_01/DESIGN_v2.md`, selected in RR "B0 selected on DESIGN_v2; …";
- the probe: `R/I81/b1_probe_01/PROBE.md`, ruled in RR "I81's B1-0 probe verified; …";
- the study: `R/I82/b1_cap_study_01/STUDY.md` and `ADDENDUM_01.md`, with S3 selected in RR "I82's addendum: B1's target is S3, …";
- the owner's M decision: RR "Owner decision: M's practical limit is 12 GiB; target machines";
- the reviews: `R/REVIEW_RV107/b1_plan_01/` (REVIEW and ADDENDUM_01).

**PLAN_v2 §1's fence is binding.** A stop in §1 or R8 means you return to ROOT, not proceed.

## Host rules

- **Every cargo goes through `WT/tools/t3_cargo.sh`** (`--locked --offline`). Heavy vitest and pytest go under `/usr/bin/lockf -k WT/guard/cargo_job.lock …`.
- **pytest under `P/tests`** either sets `OPENPIPESTRESS_CHECKED_JSON_BIN` and `OPENPIPESTRESS_UNITS_BIN` to existing binaries, or runs under the lock (RV104 N-6).
- **Other T3 jobs share the lock** (ROOT's DEC-025s, other implementers and reviewers). Wait for it, and never kill another job.
- **Not allowed:** DEC-025, evidence sweeps, native or solver-at-scale jobs beyond the inputs your brief names, and installs.
- **Scratch** goes in `WT/scratch/<id>_<unit>/`, with targets in `WT/targets/<id>-<unit>/`. Nothing goes to the system temp directory; set `TMPDIR` to your scratch if a tool needs one.
- **Git:** commit only on your own branch, with truthful messages ending in `Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>`. ROOT pushes and merges. Read Git with `GIT_OPTIONAL_LOCKS=0`.

## Records

- Records go in `R/<id>/<unit>_01/`: RETURN.md (or PROBE.md), `_run_records/` and SHA256SUMS, with placeholder paths only (`WT`, `NUM`, `P`, `PP`, `RE`, `T`, `R`).
- **If the host's write guard refuses a write into NUM,** write to `WT/scratch/<id>_<unit>/records/` and say so.
- **ROOT commits the records.**
