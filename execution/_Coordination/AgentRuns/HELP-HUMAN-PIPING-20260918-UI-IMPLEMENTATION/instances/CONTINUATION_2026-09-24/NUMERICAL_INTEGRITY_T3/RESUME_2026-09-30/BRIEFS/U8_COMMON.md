# U8: what every U8 assignment shares (I68, I69–I71, I72, RV97, RV98)

Each assignment is a TASK (Type 2) under ROOT (HELP_HUMAN, Agent 0). ROOT is the return path, and none delegates.

**You are a fresh instance.** Your ID names a records folder and a role. It carries no memory of earlier sessions, so read the basis below before acting. Earlier holders of related roles (I61, I62–I65, RV89, RV93) left their work in records; cite those records, and do not assume anything beyond them.

## What U8 is

U8 is the deferred producer-solved witnesses (step-4 plan §U8; RR "Snapshot 05b verified; … producer-solved witnesses deferred"), now planned concretely. It comes first in the owner's F2a order (RR "Owner decision: F2a's delivery steps move to the end of F2a; erratum E-1"). It adds three witnesses:

1. **RV93 N-5, the real-input fallbacks.** Candidate (`first_load_only`) and Preparation (`tiny_spring`) are reached from real D1 inputs, not hooks. Each publishes the plain bytes plus exactly one notice, from one ordinary run.
2. **W-C1, a real-input Native fallback.** Its reason (expected Ceiling) is recorded by the probe, not asserted in code.
3. **The L = 0 base.** The milestone plus one memberless, fully restrained node. **If** the producer admits it and W1 selects, it publishes a pinned successor that enters corpus 07l. Otherwise L = 0 is deferred to B1 with its recorded cause.

The receipt Ceiling row (W-C2) is **not** U8's. It is B1's acceptance witness. U8's probe only builds and records W-C2's candidate input.

**U8 is test-only.** No PP production text, and no reader `src/` text, changes. If a reader defect forces a production change, that is a stop: return to ROOT, because it re-opens re-qualification (RR:10474; QUALIFICATION §11).

## The basis

- **The plan:** `R/I61/u8_plan_01/PLAN.md` (sha256 `f274a614…`), §0–§1 for U8. Its file:line citations are at NUM `697b402779`, whose maintained source equals main `0b00b8e8b6`.
- **The rulings** (`T3/ROOT_RULINGS_V1.md`):
  - "I61's U8 plan ruled; the owner pulls T6's successor-output slice forward; dispatches prepared for the handoff" (decisions 1–7);
  - "Owner decision: F2a's delivery steps move to the end of F2a; erratum E-1";
  - "DEC-025 on F finds a test-walker defect; repaired and refrozen as F′ = 5488136a19" (the full 40-manifest suite runs before any freeze).
- **The merged D1 milestone:** `T3/IMPLEMENTATION/F2A_D1/CHANGE_RECORD.md` (scope §4, obligations §5) and `copies/QUALIFICATION.md` (D1.0–D1.11 in §6; §11 re-qualification).
- **The registered build** (the only identity in `REGISTERED_PROFILES`):
  - aarch64-apple-darwin, rustc 1.97.1 `8bab26f4f68e`;
  - debug, opt-level 0, debug assertions, panic=unwind, no RUSTFLAGS;
  - M = 4,026,531,840 B.
  
  A default `cargo test` on this host compiles it. Any other build is Stale and keeps the ordinary route.
- **The milestone fixtures:** `P/fixtures/product_preview/rf_skew_t_cant_off_122_r1e-04.request.json`, with the successors sparse `ac6986b0…` and dense `6cd1d249…`.

## The working tree (set up by ROOT before dispatch)

- **The U8 branch** `codex/piping-f2a-u8-<date>` is cut from NUM after ROOT has absorbed main (PLAN §4's dry-run procedure), in worktree `WT/f2a-u8`. ROOT gives the exact head at dispatch.
- **Leave work uncommitted** unless your brief says otherwise. ROOT commits.

## Host rules

- **Git:** no Git writes. Reads use `GIT_OPTIONAL_LOCKS=0`.
- **Cargo:**
  - `--locked --offline`, `CARGO_BUILD_JOBS=4`, `RUST_TEST_THREADS=2`, one cargo job at a time on the host;
  - your own target directory, `WT/targets/<your-id>-<unit>/`;
  - no DEC-025, native or solver-at-scale jobs.
- **The memory guard** (`WT/guard/memguard.sh`; check with `pgrep -f memguard.sh`) must be running.
- **Node:** use the existing `node_modules` through an untracked symlink only, and remove the symlink when done. No installs.
- **Scratch** goes under `WT/scratch/<your-id>_<unit>/`: never the system temp directory, never the parent checkout's `scratch/`, and never a worktree's own folder.
- **Records** go in `NUM/R/<your-id>/<unit>_01/`: RETURN.md with SHA256SUMS, placeholder paths only (`WT`, `NUM`, `P`), and the changed files' hashes.
- **Cleanup:** delete disposable copies and your target directory when you return, unless your brief says to keep them. The periodic cleanup tool (`WT/tools/t3_cleanup.py`) removes leftovers, but don't rely on it.

## Binding rules

- **Nothing weakened.** No assertion is deleted or loosened, and a pin changes only its expected value, with the reason.
- **Existing identities and bytes do not change.** The milestone still publishes its pinned successors, every ordinary route is byte-identical, and the Stale route is unchanged.
- **The D1 call graph does not change.** Only test files, fixtures and the corpus.
- **Stops:**
  - a production change looks necessary;
  - a write would fall outside your fence;
  - an existing byte changes;
  - a check would be weakened;
  - a contract reading goes beyond the rulings.
  
  Report the stop, and continue the work it does not affect.
