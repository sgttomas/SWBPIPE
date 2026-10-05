# I63 return: Rust reader on snapshot 07c

I63 is a TASK (Type 2) doing ROOT's standing step: adopt 07c once I62's record exists and READER's corpus hash matches it. The basis is D21 as widened (NUM `d232f0859f`): a non-null `verification` summary on an escalating failed verification also counts as evidence that the pass ran. I63 had no descendants.

- **Run:** first tool call 2026-10-04T01:03:11Z; done about 01:07Z, inside the 45-minute box. The memory guard (PID 5387) was running.
- **Limits held:** no Git writes or index operations; no install, new tooling or native job.
- **Toolchain:** the default, with no `DEVELOPER_DIR`.
- **Basis files:** READER at `0c81e09b09`. NUM was at `05b4798d89` when hashed.
- **Paths** use the brief's placeholders.
- **Status:** 07c passes in full. **Not accepted; eligibility still held** (`IMPLEMENTATION_COMPLETE = false`).

## Inputs verified

- **SHARED_SNAPSHOT_07C.json** (`81623caa32`) is present and listed in I62's SHA256SUMS, which verify.
- **All five shared files** match it: corpus `d33667719e`, schema `07951edacf`, definition `3e0779a45a`, preview table `c74742ce6a`, results yaml `4585a45fcf`.
- **Counts:** 15 cases, 254 mutations and 19 must-pass entries. The one new entry is `verification_summary_on_escalating_failed_verification` (expected G5 ATTEMPT_MISMATCH). Everything else is byte-identical to 07b, per I62's assertion.

## Changed files (READER, inside the fence)

Paths are under P/core/reporting/result_export.

| File | Before (`0c81e09b09`) | After |
|---|---|---|
| src/retained_precision.rs | abacc74664d6… | **unchanged.** Rust's D5b already requires a null `verification` summary (phase 1), so the widened D21 needs no code change |
| tests/retained_precision_contract.rs | d800f975dde2… | c984f49a54b69d6f08fc86ee0147ecfc8ad733ef9dc366db154ad7d23da38ce4 (103211 B) |
| src/lib.rs | 375b073135… | unchanged |

**The test changes:**
- the slice helper asserts 254 mutations;
- the 07b slice covers 236..254, with G5 ATTEMPT now 5.

## Commands and results

Both runs used the 06d command and environment variables, with the default toolchain and no `DEVELOPER_DIR`.

| Run | State | Result |
|---|---|---|
| run1 (**final, full command**) | final bytes | **47 passed, 0 failed** |
| run2 | `--nocapture snapshot_0 shared_must_pass` | 11 passed; outcomes captured |

**Against the bar (07c):**
- **Mutations:** all 254 match their expected first gate and code (G7 per reader). The new pin gives G5 ATTEMPT_MISMATCH.
- **must_pass:** all 19 pass with the base case's classifications.
- **Cases:** all 15 validate with their expected classifications.
- **Tables:** `OUTCOMES_07C.json`, all 254 mutations in corpus order and all 19 must-pass entries.

## Remaining known differences

**None known.** The D5b verification-summary difference from Python reported in RETURN_07B is now ruled (widened D21) and pinned in 07c. G7 per-language codes and fallback codes are by design. This rests on the earlier targeted comparisons with Python, not an exhaustive one. I have not compared TypeScript at its head.

## Files read (sha256)

| sha256 | File |
|---|---|
| 53cf4fb2968601a8b812a942e5876ed6720e5230babacf48fb0bec56720c9e82 | T3/ROOT_RULINGS_V1.md at NUM `05b4798d89` |
| 81623caa322ee530333b40692f06d6f83ec92f48025990d5cbc6f4ff156bd705 | R/I62/review_repair_07/SHARED_SNAPSHOT_07C.json |

## Bulk (WT/scratch/i63_review_repair_07c/)

| sha256 | bytes | file |
|---|---|---|
| 5de4426cb49e65680d90a545f7a8ec313973e6ab12769376b97273647bfb45bb | 1760 | I63_07C_DELTA_test.diff |
| 47a3726653f16091afba15f0c27811059d9a436c019db8bb072c82e857ff88ef | 3211 | run1.log (final) |
| d017f48a6b1e5000c484a74f34a72cb8b3e7cdc1101cb951f66e76255cbd8e3e | 65794 | run2_outcomes.log |
| d800f975dde26bb5d35708181d640a6fbcdcf4716baaa3d9f6b65bfe9111d050 | 103163 | before/retained_precision_contract.rs |
| abacc74664d67ce34887cf86481f3ff12090bab2b40689e20e297a9f9fc2c80f | 171474 | final/retained_precision.rs (= before) |
| c984f49a54b69d6f08fc86ee0147ecfc8ad733ef9dc366db154ad7d23da38ce4 | 103211 | final/retained_precision_contract.rs |
