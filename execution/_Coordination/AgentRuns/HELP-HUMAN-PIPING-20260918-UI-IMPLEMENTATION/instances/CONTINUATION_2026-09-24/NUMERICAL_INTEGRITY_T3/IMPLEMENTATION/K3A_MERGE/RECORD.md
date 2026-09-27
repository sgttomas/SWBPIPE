# K3a merge record

- **PR:** https://github.com/sgttomas/chirality/pull/983, merged 2026-09-27 as `3e861f53cca8e4d94622fadfb1054420a8349cc1` (merge commit) by ROOT (HELP_HUMAN) under the owner's standing Git authorization.
- **Candidate head:** `706d027f0077d464d28e678b11d8810b44ba6fc0` (branch `codex/piping-k3a-20260926`). K3a commits `0353dcd23` and `a2e804a75`, merged forward onto main `3488a236a` at `43da7a24e`, plus the RV2 fix commit `706d027f0`.
- **Gates on the candidate head:**
  - **RV2 independent complete-diff review** (`REVIEW/K3A_REVIEW.md`, Addendum A): PASS, 0 BLOCKING, 0 open SHOULD-FIX. S1 and S2 were fixed in `706d027f0`, and the delta backcheck PASSED (mutant R2 killed; R1, M5 and M6 still killed; no arithmetic code changed).
  - **Hosted CI:** green.
  - **Surface-4 dual-viewport dispatch:** Piping Desktop E2E run 36288811078, target_base `3488a236a`, success.
  - **DEC-025 sandboxed sweep:** `dec025/SWEEP_*_706d027f0077.json`, overall pass, `working_tree_dirty: false`.
    - cargo crate sweep: pass.
    - pytest: 3023 passed, 32 skipped.
    - vitest: 2822/2822.
    - production build: pass.
  - The earlier head `43da7a24e` also passed its own sweep (`dec025/earlier_head_43da7a24e/`).
- **RV2 note N8 (optional, recorded here rather than by a further push):** `IMPLEMENTATION/K3A/RETURN.md` §1, §3 and §8 carry pre-fix figures (966 and 1,068 lines; 5.41 ulp over 3,305 vectors), and `CHANGE_RECORD.md` calls the fixes "uncommitted". The current figures are the arctangent regression worst of 6.0818 ulp over 3,319 vectors, the proved contract of 23.6 ulp, and 135 frame_kernel tests (116 base + 19 new). The fixes are committed in `706d027f0`.
- **Not run for this merge:** the src-tauri suite, which is outside the sweep and CI; I2 ran it at 114/114 on K3a. The native macOS witnesses do not apply: K3a touches no native path and is dormant until K-D5.
- **Paths:** placeholders only.
