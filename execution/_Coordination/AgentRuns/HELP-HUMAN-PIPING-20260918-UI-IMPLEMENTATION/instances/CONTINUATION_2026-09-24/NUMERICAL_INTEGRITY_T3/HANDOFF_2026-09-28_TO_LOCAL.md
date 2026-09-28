# T3 handoff to a local session (2026-09-28)

> **[Status note, added 2026-09-28 (RV13-N7).** This file is a dated snapshot of the cloud-to-Mac handoff, and its present tense is as of that handoff. For current status see the T3 row of the work graph, and `ROOT_RULINGS_V1.md` for the rulings since.]

**Decision (ROOT, HELP_HUMAN, 2026-09-28).** T3 (numerical integrity, precision and scale) moves from the cloud container to a local session on the owner's Mac, which has much more CPU and disk.
- **K2a finishes here.** It is in its gate now.
- **K1 onward continues on the Mac,** from K1's WIP commit.

**Read `OPERATING_NOTES_FOR_LOCAL_ROOT.md` next.** It covers how the roles and delegation were run, messaging and host arbitration, keeping the pipeline full, epistemic discipline, and PR mechanics.

The owner's direction for T3 is unchanged: prioritize solver correctness and the validation programme; don't populate material or component libraries or code rules; preserve evidence and completed gates (`OWNER_DIRECTION.md`).

All paths below are relative to this folder (`T3/` = `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/`), unless they start with `projects/` or `.agents/`.

## 1. State

### Merged to main

| Slice | PR | Merge SHA | Records |
|---|---|---|---|
| S11-K (S11 kernel and library half) | [#973](https://github.com/sgttomas/chirality/pull/973) | `3488a236a` | `IMPLEMENTATION/S11K/`, `S11K_MERGE/`, `REVIEW/S11K_REVIEW.md` |
| K3a (`Wide<2>` arithmetic) | [#983](https://github.com/sgttomas/chirality/pull/983) | `3e861f53c` | `IMPLEMENTATION/K3A/`, `K3A_MERGE/`, `REVIEW/K3A_REVIEW.md` |
| S11-F (S11 facade half) | [#1000](https://github.com/sgttomas/chirality/pull/1000) | `43b8f83aa` | `IMPLEMENTATION/S11F/`, `S11F_MERGE/`, `REVIEW/S11F_REVIEW.md` |
| RV3-S1 follow-up to S11-F | [#1002](https://github.com/sgttomas/chirality/pull/1002) | `72d5ff864` | `S11F_MERGE/` |
| S11-G (formation-noise guard, revision 2.2) | [#1003](https://github.com/sgttomas/chirality/pull/1003) | `b24b3d536` | `IMPLEMENTATION/S11G/`, `S11G_MERGE/`, `REVIEW/S11G_REVIEW.md`, `REVIEW/S11G_CHECK.md` |
| K-D5 (the D-5 formation check) | [#1017](https://github.com/sgttomas/chirality/pull/1017) | `5ae22926e` | `IMPLEMENTATION/KD5/`, `KD5_MERGE/` (with `ADDENDUM_1.md`), `REVIEW/KD5_REVIEW.md` |
| F1a (the D-5 evidence line and SUP-17) | [#1025](https://github.com/sgttomas/chirality/pull/1025) | `134eefc24` | `IMPLEMENTATION/F1A/`, `F1A_MERGE/`, `REVIEW/F1A_REVIEW.md` |

The T3 records reached main through records PRs #991, #1001, #1004, #1019 (`2dc6f513a`) and #1029 (`a90e89dcf`). Later records commits on `codex/piping-numerical-integrity-20260926` go to main in the next records PR.

The S11-G performance finding is withdrawn. The interleaved timing is in `IMPLEMENTATION/S11G_TIMING/`, and `KD5_MERGE/ADDENDUM_1.md` records the withdrawal.

### In flight

- **K2a (checked formation).** Branch `codex/piping-k2a-20260927`; implementer I6. Brief: `TASK_BRIEFS/I6_K2A_IMPLEMENTATION.md`, with addenda 1–3.
  - Mutations (31 killed), the 24-crate suites and T9 (112/112 byte-identical) are done here. The two-part gate is running.
  - K2a **finishes here**: clean point, commit, independent review, PR, CI, DEC-025 sweep and merge record.
  - Its product-reach position is set by `ROOT_RULINGS_V1.md`, "K2a product reach" (the original section and corrections 1–3). The authoritative figures are the derivation section of K2a's RETURN (correction 3, ruling 4).
- **K1 (W3 kernel sparse representation and sparse M03 gate).** A **WIP commit only** on `codex/piping-k1-20260928` (branched from main `134eefc24`): `d08b0efc767db1090a63268a180b1dd058617ac4`, titled "K1 WIP (handoff; not reviewed)".
  - Nothing is compiled or tested.
  - The state and the open items are in `IMPLEMENTATION/K1/WIP_STATE.md` on that branch.
  - Brief: `TASK_BRIEFS/I8_K1_IMPLEMENTATION.md`, with addenda 1–4.

## 2. Remaining T3 order

From `DESIGN_NUMERICS/DESIGN.md` revision 5a.2 §6 and D-10, as amended by ROOT's rulings. The work graph's T3 row (`projects/chirality-piping/execution/_Coordination/WorkGraphs/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/WORK_GRAPH.md`) governs where they differ.
- **Kernel:** K2a (finishing here) → **K1** (from WIP) → K2b → K5.
  - The rest of K3 can run in parallel.
  - K4 follows K1 and K3, then K6 and V-K.
  - S11-K, K-D5, K1, K2b and K5 share `SA` and `FK/structural.rs`, so they stay serialized.
- **Facade:** S11-F ✓ → S11-G ✓ → F1a ✓ → **F1b** (after K1 and K2b) → F2a (with D2's S-G1) → S-I → F2b per domain → F3.
  - F1b carries F1a's follow-ups N1 and N2 (`F1A_MERGE/RECORD.md`), and the both-entry gate that K1 skips.

## 3. The T3-close decision list

These are pointers; the entries themselves are in the work graph's T3 row and in `ROOT_RULINGS_V1.md`.
- **Curved-bend gate coverage.** The gate corpus has no realized curved bend. This is routed to the gate-corpus owner; ROOT decides whether it is required or a named gap. See the work graph, from RV5's K-D5 review.
- **Curved element at large coordinates** (≳ 2e6 m, from PP's binary64 centre). Routed to T4/W1c with its priority raised; see the T4 row and `KD5_MERGE/RECORD.md`.
- **F1b's LEF expectation.** "LEF-small and LEF-large solved" is unreachable at product level as written. F1b's brief must restate it before F1b spawns (work graph).
  - I6's gate refined the earlier statement: LEF-large is refused at capture **on the captured entry only**. On the typed entry it reaches `local_stiffness` and is blocked there. On main that was a non-finite formation error; with K2a it is the named range refusal. The standing is unchanged. Details are in K2a's RETURN.
- **Missing lower magnitude bound on inputs.** Routed to the input-validation owner. See the original "K2a: product reach" section, ruling 3.
- **Other stiffness-forming paths** (curved_bend, arc_model and others) with terms scaled by 1/L² or 1/L³. Check whether they can form an unbounded zero or a wrong rounding. If so, route to K5 or its own slice. See correction 2, ruling 4; the paths are listed in K2a's RETURN.
- **K-D5's `formation_check.rs` in the S11 site table.** It rides K1's PR as a separate commit ("K1: the S11 site table …").
- **Withdrawn:** the S11-G performance finding (see §1).

## 4. Standing rules

Read these in the records; they are not restated here.
- **TASK conduct:** `TASK_BRIEFS/_COMMON.md` (roles and returns, locations, writes, host resources, rules). TASKs make no Git writes; the manager verifies and commits.
- **Rulings:** `ROOT_RULINGS_V1.md` (and `ROOT_RULINGS_V2.md`). Later sections supersede earlier ones where marked in place.
- **Recorded lessons** (in `ROOT_RULINGS_V1.md`):
  - enumerate every caller by lexer scan;
  - a claim about product behaviour needs a product run;
  - a bound or general claim inferred from probes must be derived and independently checked before a ruling rests on it;
  - performance claims come only from interleaved runs of probes built fresh from archives, on a quiet host.
- **Evidence:**
  - never rewrite committed, hash-bound evidence; add new files;
  - SHA256SUMS on every record folder;
  - no machine paths in committed records (`<wt>`, `<scratch>`, `<WORKTREE>`, `<VENV>`, `<home>`);
  - GEN-8 (`pytest tools/practitioner_harness/test_live_baseline.py -k gen8`, from the repository root) before every records commit.
- **Gates for a slice PR:**
  - a complete-diff independent review;
  - hosted CI with the surface-4 dispatch;
  - a clean DEC-025 sandboxed sweep; **[on the Mac, see `OWNER_DIRECTION.md`, "Owner decision (2026-09-28): DEC-025 on the Mac" (ROOT's pointer, from RV9's S3)]**;
  - the committed-fixture diff (T9) with its stop rule;
  - where the slice changes a product path, the both-entry no-Passed-breach gate against the **empty** `GATE/S11_EXCEPTIONS.json` and `GATE/FORMATION_EXCEPTIONS.json`, in two parts, with the known dense timeouts on a quiet host.
  - Skip no tests, raise no timeouts, and strip no loads or features.
- **Host resources.** The container's rules (one heavy cargo job at a time; holding cargo during DEC-025 sweeps; keeping free disk above about 8 GB) exist because of the container's limits, and **may relax on the Mac.** Two parts do not relax:
  - **Timing runs still need a quiet host and interleaving.**
  - **Gate part 2** (the dense timeouts at 1800 s) still needs a quiet host.
- **Git:** the owner's standing Git authorization (root `AGENTS.md`, "Execution and governance") and `.agents/skills/chirality-change/SKILL.md`.

## 5. First steps on the Mac

1. **Environment.**
   - Rust toolchain 1.97.1 (`RUSTUP_TOOLCHAIN=1.97.1`, `CARGO_INCREMENTAL=0`, `--offline --locked` once vendored);
   - Node 24;
   - a Python venv with the repository's test requirements (for GEN-8 and the DEC-025 runner);
   - Playwright's Chromium, for the desktop E2E surfaces.
2. **Platform calibration, before any T3 comparison.**
   - Build main on the Mac.
   - Run the T9 committed-fixture diff (112 outputs), and compare the hashes with the Linux records (for example `F1A_MERGE` and `IMPLEMENTATION/F1A/` T9 hashes, which equal K-D5's).
   - Run a gate sample through both entries and compare it with the Linux gate records.
   - **Report byte-identity, or name each difference.** If any output differs by platform, then for every later comparison **keep main and the candidate on the same machine.** Never compare a Mac candidate against Linux main records.
3. **Resume K1 from `IMPLEMENTATION/K1/WIP_STATE.md`.**
   - Start a **fresh implementation TASK** with the I8 brief and addenda 1–4.
   - Later, a **fresh independent reviewer.**
   - K1 is based on main `134eefc24`. After K2a merges, merge main into K1's branch and land the K2a-interaction tests, whose drafts are in the WIP. K1's PR cannot merge before K2a.

## 6. Records conventions

- **Branches:**
  - T3 records go on `codex/piping-numerical-integrity-20260926` (numerics);
  - slices go on `codex/piping-<slice>-<yyyymmdd>`, for example `codex/piping-k1-20260928`.
- **Records-PR cadence:** after one or more slice merges, merge main into numerics, run GEN-8, and open a records PR with an independent reviewer. While a records PR is under review, keep numerics commits local, so the reviewed head stays put.
- **Per slice:** `IMPLEMENTATION/<SLICE>/` (CHANGE_RECORD, RETURN, `_run_records/`, SHA256SUMS), `IMPLEMENTATION/<SLICE>_MERGE/RECORD.md` (with a sanitized DEC-025 summary), and `REVIEW/<SLICE>_REVIEW.md`.
- **No model identifiers** in records.
- **Commit trailers:** the attribution trailers the session's host specifies (co-author and session link), and truthful authorship. Never imply personal owner review.

## 7. Open agents in the cloud session

- **I6 (K2a)** finishes K2a here: the gate, then its clean point.
- **The T3 manager** (WORKING_ITEMS) stays until K2a merges. It commits K2a, supports its review and PR, and writes `K2A_MERGE`, then stops.
- **I8 (K1)** has stopped after its WIP_STATE. The K1 WIP is committed.
- **RV6 (F1a reviewer) and I7 (F1a)** are finished.
