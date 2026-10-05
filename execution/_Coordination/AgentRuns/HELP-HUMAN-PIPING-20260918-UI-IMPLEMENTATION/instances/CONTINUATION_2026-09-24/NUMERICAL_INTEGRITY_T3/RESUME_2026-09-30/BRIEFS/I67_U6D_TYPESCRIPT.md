# I67: U6d, the TypeScript carriers and standing

**Read `BRIEFS/U6_FANOUT_COMMON.md` first.** You are I67, a new TASK. You own U6d through repairs.

- **Unit:** PLAN.md §6 U6d, delivering §1d. This is the longest chain in U6, at about 8–11 h.
- **Worktree:** `WT/f2a-carriers-ts`, branch `codex/piping-f2a-carriers-ts-20261004`, from `844448112f`.
- **Fence:** exactly PLAN §6 U6d's list of TS files and new tests. `loadReferenceOutputAvailability.ts` is reserved for U6 (D-U6-8). **No T6 panel is edited.** Everything else in §1d is read-only and pinned by test.
- **Tests** as listed in PLAN §6 U6d:
  - registration through mocked direct and job IPC, with the fixture and its invocation;
  - synchronous standing;
  - a mutation that voids the registration;
  - AnalysisRun copy and validate;
  - reopen;
  - every T6 surface refusing;
  - the rule-check gate;
  - the 14 parity scenarios, which must agree with Rust U6a.
- **Runtime:** the existing `P/node_modules` (link) and prebuilt WASM assets (copy), as the reader rounds used them. Disclose both.
- **Limit (D-U6-3):** no native witness, because Tauri never delivers a successor (F-1). State it as a qualification limit.
- **Controls:**
  - the whole desktop Vitest suite and `tsc` pass;
  - every existing outcome is unchanged;
  - mutants are killed for each new branch.
- **Records:** `NUM/R/I67/u6d_typescript_01/`. **Budget:** 11 h. Return once, or return at a stop.
