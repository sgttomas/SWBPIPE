# I39 private certificate helpers — return

Implemented both scoped helpers in the isolated checkout. Final numerical candidate is `SOURCE_04.json`, rooted at unchanged source `fdae294643b798c1849da8b2e643085562593686`. ROOT owns integration and fresh independent source review.

## Scope and decisions

- `directed.rs` has exactly one private child-module registration hunk; all implementation and private test registration reside in the new child module.
- `sqrt_endpoint` uses a fresh p1024 context, exact q*q-a, and at most one existing parent step. Both signed zeros become private +0; negative and intermediate-range refusals preserve native causes.
- `small_row_bound` admits finite value and finite 0<scale<2^-988. Its two divisions and two scale-back multiplications preserve separate RN64 operations. One maximum check and at most 63 bisections each own a fresh four-term exact sum.
- `EntrySpent` keeps private original result, actual AttemptWork/SumWork and exact u8 comparison/arithmetic counts. Its checked result accessor blocks success for non-E joined status while retaining an original numeric refusal independently. The f64 count is four rounded arithmetic operations; bitwise abs/next_up and predicates are explicitly excluded, with no invented tariff.
- Collection occurs outside fallible numeric closures and after every exact comparison. Private owner-taking seams support seeded tests without runtime fault flags or public construction.

## Checks and source correspondence

- `DEBUG_FINAL`: 10/10 pass (8 new helper tests and 2 existing directed controls, including their fixed fraction fixtures). `RELEASE_FINAL`: 8/8 pass.
- New standard-library generator gives 14 full-width sqrt cases using integer-isqrt and midpoint-square decisions, with symbolic huge exponent metadata; Rust checks both directions, nearest root, adjacency and operation envelopes.
- 18 small-bound cases use Fraction direct binary64 quantum-ceil, independently simulating separately rounded scale-back operations. Cases cover zero, subnormal/normal transitions, both nearest-even tie directions, exact equality, maximum finite value and the RN1024 double-rounding discriminator. Every vector also agrees with existing row_bound bits.
- Other tests cover invalid admission without work; negative/range sqrt prefixes; partial-comparison work/count retention; failure on the first adjacent-step insertion after the complete side test; coexisting numeric failure and non-E work; and blocked success extraction.
- Exact generator replay is byte-identical (`ORACLE_REPLAY_02.json`). `FORMAT_02` and final `git diff --check` pass. Candidate source hashes were captured before final checks, and rechecked after them.
- Exact argv, environment, source hash reference, process IDs, wall limit, exits and raw logs accompany each Cargo run. Runs used -j 4, RUST_TEST_THREADS=2, the separate i39-frame-kernel target, existing memguard 5387, and a 1200-second wall limit.

## Preserved failures and deviations

- `DEBUG_01` compiled and passed 7/8 filtered tests. The new dedicated range-refusal input was wrong: a near-perfect radicand gave an exactly representable q², so no residual range failure was required. It was replaced using the independent odd-exponent irrational-root argument; generated expected bits were never changed to match implementation. `REPAIR_01.md` records the correction.
- `FORMAT_01` found generator line-break formatting. Only tuple formatting changed; oracle values and cases remained fixed, and final debug/optimized checks were rerun.
- Initial reading accidentally displayed NEXT_CODE_AND_WITNESS sections 1–4 beyond authorized section 0. No reliance or scope expansion followed. Origins explicitly preserve that wider read.
- Initial git status omitted GIT_OPTIONAL_LOCKS=0. No explicit Git/index mutation was requested; automatic refresh was not observed. The final index has no staged changes, and subsequent Git reads disabled optional locks.

## Process and remaining boundary

- Receipt observed 2026-10-02T23:37:34+00:00; completed 2026-10-02T23:48:34.316640+00:00 (660.3 seconds). Early return before the 20-minute checkpoint and 45-minute expansion cutoff.
- All owned Cargo processes exited; lane returned to ROOT. Existing guard remains. No source checkout commit, integration, public API/caller, source adapter, solver/model run, policy selection, full F2a/profile/availability claim or merge acceptance follows. The existing Result-only binary64_up custody gap remains outside this slice; its sole new use is a numerical negative control in tests.

## Final files and SHA-256

- `projects/chirality-piping/core/solver/frame_kernel/src/structural/retained/directed.rs` — `3682ee4f7c78a4f7cf5f96ca3065dcae903f1fcce28c0265f614bf305eb441e6`
- `projects/chirality-piping/core/solver/frame_kernel/src/structural/retained/directed/certificate.rs` — `3525a99225a7d19a3e798cfdfe40921f658bb61acc2151098b28c3f19292926e`
- `projects/chirality-piping/core/solver/frame_kernel/tests/retained_k4/certificate_arithmetic_tests.rs` — `4b704dacd360a46ae82a6e0e27668a23fab9a6c574f1e63e24a02ca042300b1e`
- `projects/chirality-piping/core/solver/frame_kernel/tests/retained_k4/certificate_arithmetic_vectors.rs` — `24e9eb55045474ecabe754ecfed8850f92f6a481cae24de4f530468708880861`
- `projects/chirality-piping/core/solver/frame_kernel/tests/retained_k4/certificate_arithmetic_vectors.py` — `4c24add95da95062da72896f73a06124709afd419de065f11167470ce16e30e8`

Run evidence lives in this directory; `ORIGINS.json`, `EXECUTION.json`, `FINAL_PROCESS.json` and `FINAL_GIT_READS.json` give provenance, timing, command and write-boundary observations.
