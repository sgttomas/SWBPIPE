# KF3 merge record

- **PR:** https://github.com/sgttomas/chirality/pull/1059.
  - ROOT (HELP_HUMAN) merged it on 2026-09-30 at 05:28:58Z as `dd61120ff271b02bf5fcb032f264564af5e8bb09`: a merge commit with `--match-head-commit aa83f6796`, under the owner's standing Git authorization.
  - **Main had moved.** It was `45ffd91d1` (PR #1061) at the merge, not the `78f55f927` that the head carries. **ROOT merged before checking that move, contrary to its own operating notes** (OPERATING_NOTES §6, "Watch main"). The check, made right after:
    - PR #1061 changes only `projects/chirality-app-v4/**` (336 files);
    - `git diff aa83f6796 dd61120ff -- projects/chirality-piping tools .github` is empty, so the merged piping, `tools/` and `.github/` trees are byte-identical to the gated head;
    - every gate below therefore covers what merged.
    - The lesson is recorded in `HANDOFF_2026-09-30_AUDIT_PAUSE.md` §7.
- **Candidate head:** `aa83f67969c2f618034b856ba6e1fc13ae10762e`, on branch `codex/piping-kf3-20260929`.
- **Where it ran:** the owner's Mac (`aarch64-apple-darwin`, rustc 1.97.1). ROOT dispatched the implementer (I19) and the reviewer (RV23) directly.
- **Scope:** T3 slice KF3, W1a at scale.
  - **D1 revision 5a.3 amendment A2** (ROOT's ruling):
    - a Uc_c or S_c whose formation is refused (`Span` or `Exponent`) is unavailable, and B_c is the minimum over the formed bounds;
    - a block with data and no bound stops the attempt with its refusal, which outranks a `uc` rejection;
    - refusals are recorded per block (`AttemptRecord::bound_refusals`), on every path.
  - **Partial stage work** is staged on every path of all four builds.
  - **The code:** `FK/src/structural/retained/{adaptive,bound,verify,wide_sum}.rs`, K4's tests, GEN and `kf3.txt`, plus one `retained_api` line exporting the five refusal types.
  - **The harness:** `performance_harness`'s parity check is back to equality on every attempt (K6b's test updated).
  - **Kernel and harness only:** no product crate names `retained_api`.
  - The account is `IMPLEMENTATION/KF3/RETURN.md` (with addendum 1) and `CHANGE_RECORD.md`, on the merged branch.

## The chain (branch point: main `0f5d8c7b4`)

| Commit | Content |
|---|---|
| `75a1222a6` | Checkpoint 0: the diagnosis and plan |
| `29c0b69e4`, `a7ec4981a` | A: amendment A2 and the partial stages; the checkpoint and run records |
| `c0473301e` | A merge of main `f8400d290` (V-K), by ROOT. There was one conflict (`AttemptRecord`), which I19 resolved by editing, under ROOT's visibility ruling |
| `e114b23c1` | A merge of main `78f55f927` (K6b), by ROOT, with no conflict |
| `ae831ca51` | B: K6b's parity restored in H; the scale evidence at 100 to 10,000 members |
| `b8c55c92e` | D: RETURN, CHANGE_RECORD, run records, and the KF3-B1 and KF3-B2 derivations |
| `aa83f6796` | RV23's review closed: refusals kept in the evidence when a stop follows them (RV23-1); the `verify_state` precedence test (RV23-N1) |

## Gates

- **Independent review, RV23** (`REVIEW/KF3_REVIEW.md`, with `REVIEW/_run_records/kf3_review/`):
  - **PASS** at `b8c55c92e`, with 0 BLOCKING, 1 SHOULD-FIX and 5 NOTEs. RV23 found no way for A2 to publish a wrong value.
    - **A2's honesty,** by three independent oracles:
      - synthetic multi-block refusals against exact norms;
      - 52 forced-refusal W1 runs on 16 controls;
      - KF3-UC-SPAN in closed form.
    - **The partial-stage identity:** 0 mismatches over 3,611 case-limited W1 runs.
    - **B:** reproduced byte for byte on two 10,000-member frames.
    - **Reach:** no product crate names `retained_api`.
    - **RV23-1 (SHOULD-FIX):** a refusal was dropped from the evidence when a budget stop followed it in the same build.
  - **PASS** at the final head `aa83f6796` (a confirmation):
    - RV23-1 is closed on both sites, including a cached failure reused by a later case;
    - M4b is killed;
    - every earlier probe is byte-identical, apart from the refusals now recorded.
    - One NOTE, RV23C-N1, is recorded: no committed test covers the cache keeping refusals on a non-budget failure, and there is no hook to force one.
  - ROOT's rulings: `ROOT_RULINGS_V1.md`, from "KF3: rulings on I19's diagnosis and plan; D1 revision 5a.3 amendment A2" to "KF3: RV23 confirms aa83f6796".
- **Tests at the final head:**
  - FK: 351 lib tests, 7 integration files and 6 doc-tests;
  - `gen_k4_vectors.py --check`: 24 of 24;
  - H: 75, plus `k6_alloc`;
  - the runner: 47;
  - VR: 47 (at B).
- **Mutants:**
  - 11 at A, all killed;
  - 6 at the RV23 round, all killed;
  - RV23 ran 18 plus NONE, and 7 at confirmation. The survivors: M4b at review (killed at confirmation), two equivalents (RV23-N4), and RV23C-M1 (RV23C-N1).
- **Hosted CI on `aa83f6796`:**
  - pull_request runs: Piping Desktop E2E **36669372845**, Harness Pre-merge **36669372847**, governance-harness **36669372872** and pec-tests **36669372875**, all successful. 12 checks passed and 4 were skipped as selected;
  - the full-SHA dispatch **36669370536** (target_base `78f55f927db47c7f44299fd32793d4b64e8b3572`): success;
  - the numerical cargo job took 21.5 min on the pull_request run and 21.4 min on the dispatch.
- **DEC-025,** under the owner's Mac decision (`OWNER_DIRECTION.md`, 2026-09-28), on the final head `aa83f6796`, with a fresh sweep target and a clean sweep worktree. The evidence is in `dec025/`, and the driver is `M03_SKEW_PIN_MERGE/dec025/dec025_mac.sh.txt`, unchanged.
  1. **The sweep invocation** failed at the cargo surface on product_physics's platform test `t13` (fail-fast), as expected on the Mac.
  2. **All 40 manifests** were run with `--no-fail-fast`, against K6b's Mac run at `597c81ba4`, whose piping source equals main `78f55f927`'s. The comparison is keyed by manifest path (`suites_vs_baseline.txt`).
     - frame_kernel grows 402 → 417, and performance_harness 74 → 75. These are exactly KF3's added tests: `#[test]` +15 and +1 against main.
     - The failing tests are exactly the baseline's three Mac platform tests.
  3. **Surfaces 2, 3 and 5:**
     - pytest: 3070 passed, 32 skipped;
     - the wasm build: exit 0;
     - vitest: 134 files, 2822 of 2822;
     - the production build: exit 0.
  - Machine paths are sanitized (`<wt>`, `<VENV>`), and ANSI colour codes are stripped. The unsanitized JSON's sha256 is in `sweep_json_original_sha256.txt`.
- **GEN-8** on `aa83f6796`: 1 passed (ROOT, with `set -o pipefail`, in a clean working tree of the head).
- **T9 and the both-entry gate:** not run. KF3 is kernel and harness only, which RV23 re-scanned.

## What KF3 established

- **At 10,000 members** (`vk_scale`, B):
  - CHAIN-AX, CHAIN-ROT and CONT-ROT move from `Unresolved(ExactSumSpan)` to selected at 128, and pass R1 on every row. B = S_c where Uc is refused in the forward pass.
  - CONT-AX is unchanged.
  - The TREE frames end `Unresolved(Ceiling)` from estimate (b), and publish nothing, before and after.
  - 100 and 1,000 members are byte-identical to V-K's records.
- **The constructed CI control KF3-UC-SPAN** (390 members) is selected and honest against GEN.
- **The stage breakdown equals the charged total on every path.** K6b's parity check holds with equality.
- **Refusals are recorded per block on every path.**

## Routed

- **KF3-B1** (estimate (b) on large trees; the λ split) goes to a D1 design question. The availability loss joins the owner's list.
- **KF3-B2** (E_max omits `nl_pass`'s buffers on the shifted 1024 factor) goes to K6c, together with VR's stale E_max port and W1-T4 re-run post-KF3 (`TASK_BRIEFS/I21_K6C_IMPLEMENTATION.md`).
- **RV23C-N1** is added with a fault hook for an `Arithmetic` stop, if one is ever made.
- **The stale doc** in `H/src/bin/k6_observe/w1.rs:6-10` goes to K6c.

## Scratch to prune

`<wt>/kf3-target`, `<wt>/scratch/sweep_kf3`, and `<wt>/kf3` once no follow-up needs it. The records above keep every hash.
