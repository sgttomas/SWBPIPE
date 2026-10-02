# I22 B compiler fix 01 — focused sequence passed

**Repair attempt 1 of 2. FK: 11 passed. H: 2 passed. Both exited 0.**
No second repair, numerical-failure repeat, broad suite, external probe/oracle
run, mutant, generator, scale job or accounting counter dump occurred.
No owned job remains. A1 remains BLOCKING pending independent verification/merge.

Base candidate HEAD 3cf296e36645d97e4c657c8ad1a6322bc4163f16 plus REPAIR_01.diff.
ROOT addendum 01711fb955592ed079c7f9343c7b9327fed7cd71, hash
f67a647e1f4ae8b6ff1448d476d2d9c410bf260ee4236a33cc3f1f9ee9ed7591.
Same TASK and parent; no delegation or Git/index mutation.

The only new maintained edits are:
- publication_tests.rs: explicit Wide::<4>::from_f64 and Wide::<4>::ZERO.
- method_tests.rs: sole distinct PublicationEnclosure diagnostic arm, carrying
  layout index and predicate. No old arm, fixture, assertion or expected token changed.

SOURCE_SHA256SUMS binds all five relevant files. Numerical adaptive.rs remains
4e5618a8d809e6ffa4ddd73024c206cc45eac6d5404a2f2034674552e1a95db9.
Changed test hashes:
- publication_tests.rs: 12c65218ebbc5a442a6da1e0bf90623c1efa809f1f8d4d0dd1221ce03998a04e
- method_tests.rs: c55c8f60d898b748511774a3306629eb99a7945ba584548659107dc5d050bdea

| Job | Launch UTC / PTY | Result |
|---|---|---|
| FK --lib publication_tests | 06:50:27.574 / 50601 | 11 passed, 0 failed/ignored, 367 filtered; test 0.04 s; command 9.47 s |
| H --test k6b_export | 06:50:57.935 / 87714 | 2 passed, 0 failed/ignored; test 0.04 s; command 3.97 s |

Both used configured cargo with 1.97.1, auto-install=0, --offline --locked -j4,
incremental=0, RUST_TEST_THREADS=2, seed/flags/wrappers unset and <WT>/a1-target.
Exact argv/environment and tool completions are in COMMANDS.json; full stdout
and portable stderr/time streams accompany it. Original raw stderr remains in
owned scratch, bound by RAW_SCRATCH_SHA256SUMS. Only machine roots were replaced.

Existing guard PID5387 was observed before/between/after jobs. /usr/bin/time
reported max RSS 1,250,312,192 B for FK and 505,217,024 B for H, with zero swaps;
these are command observations, not group bounds, hard RSS caps or deadlines.
Both completed within the original 06:45:35.691–07:15:35.691 checkpoint window.

C17's authored regression passed; this is not the still-unrun complete external
bare-b oracle comparison. Existing absolute-ledger closure, both rounding-
direction relative controls, broader lifecycle/pair/mismatch and old protected
regression/mutant/availability/CI/DEC-025 gates remain outstanding. H's ordinary
cantilever smoke passed, not a general availability proof.

The frozen RV29 LEDGER_FREEZE.md/LEDGER.json were read for later tests and their
six-entry ledger seal verified against supplied hash8fe24165f9905950280a78c672bada583d819d3946b95abcaa28d574ff38209b.
No new helper counters were dumped or compared; this is not a full-case work
oracle. Three extra control load IDs still need the separately granted
EXTRA-FM-01:load / EXTRA-MF-01:load / EXTRA-ZR-01:load alignment.
The newly ready bare-b checker has no external-run grant here.

The original failed implementation_b packet and all prior sealed evidence remain
unchanged. New evidence is confined to implementation_b_fix_01, owned scratch
and the granted target. Return to ROOT for commit and the next bounded grant.

