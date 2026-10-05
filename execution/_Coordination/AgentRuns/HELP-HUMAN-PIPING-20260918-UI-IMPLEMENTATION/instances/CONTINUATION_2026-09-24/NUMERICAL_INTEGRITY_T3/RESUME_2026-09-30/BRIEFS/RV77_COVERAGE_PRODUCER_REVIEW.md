# RV77: independent review of I61's producer coverage seam

TASK (Type 2), an independent reviewer dispatched directly by ROOT (HELP_HUMAN, Agent 0) as a background subagent. ROOT is your return path. You do not delegate. **You did not write this code. Don't rely on the implementer's tests as your oracles.**

## The candidate

- **Commit `c618675e84`** on branch `codex/piping-f2a-coverage-20261003`, against base CODE `652ad0cc1f`.
- **The change:** four files.
  - `FK/product_certificate/final_case.rs` gains a borrowed coverage view in `ProductProofTrace`, plus `rederive_coverage` and `check_summary_coverage`.
  - `PP/retained_receipt.rs` gains the null/complete projection.
  - Tests in `PP/retained_product_tests.rs` and `P/core/solver/frame_kernel/tests/retained_k4/product_final_case_tests.rs`.
- **The implementer** was I61. Its account is `R/I61/coverage_producer_01/RETURN.md`, with mutant patches in `_run_records/mutants/`.
- **A later commit** on the same branch restores `P/core/product_physics/tests/formation_check_runtime.rs` to main's bytes. It is a separate repair of a pre-existing regression; ROOT will ask you to confirm it at the end.

## Read first

- `REPO_ROOT/AGENTS.md`, `agents/AGENT_TASK.md`, `projects/chirality-piping/AGENTS.md`, and this brief.
- **The selected design:** `R/I57/summary_coverage_01/ADDENDUM.md`, §1–§3 and §5. Also its review, `R/REVIEW_RV76/summary_coverage_01/REVIEW.md`.
- **The rulings:** in `T3/ROOT_RULINGS_V1.md`, read "Summary-coverage representation selected for the successor" and "I61 producer seam committed for review; F1a regression found; I64 TypeScript coverage verified".

## Review, in priority order

1. **Custody and sourcing (I57 §3).**
   - The trace's coverage must be the proof's own vector (`ProductCertificateSpent` / `CertifiedProductProof`), never `ProductCapture.summary_coverage` (the adapter's fallible copy).
   - No new solve, residual, nonzero scan or coverage computation may run in projection.
   - An empty proof vector must map to null, and a complete one must cover every body in order.
   - Check every §3 table row against the actual stage and failure seams in `final_case.rs` and `PP/retained_product.rs` (the typed_trace paths).
2. **The stage rules.**
   - Null is refused on Ready, on a completed or passed certificate, and on a passed G5a.
   - Non-null coverage requires the attempt's own source and Run, a selected native Run, both lanes completed in order, the proof stages through aliases completed, and the certificate entered.
   - A complete vector must never imply certificate success.
3. **`rederive_coverage` against native `summary_coverage_data`** (`final_case.rs:1371–1448`). Establish bit-for-bit equivalence by your own derivation and by your own tests or exhaustive enumeration over presence, nonzero, extent, floor and precision patterns:
   - the extent coupling;
   - the floor ORed after the coupling;
   - stop;
   - estimate from E;
   - charge: p512 = present ∧ positive, which equals the stop pair; otherwise charge = estimate.
   
   Also check that the extent recomputation with `adaptive::body_extent`, required to match the prepared extent bit for bit, is sound. And check that the extra refusals I61 added (a floor only at p512; no stop on an absent kind; a positive floor forces its stop) hold for every genuine vector, so no genuine case can be refused.
4. **Owner binding.** List every caller of `check_summary_coverage` and of the projection, and confirm that each passes the proof's own owner. I61 notes that the seam cannot compare the proof's anchor with the owner. Say whether any reachable call path could pair a proof with a different owner.
5. **Mutants.**
   - Re-run I61's six and its NONE control from a clean `git archive` of `c618675e84`.
   - Add at least four of your own. Suggestions: the floor ORed inside the L ≠ 0 branch; the extent check dropped; has_data taken from final rows or the adapter; a weakened stage rule (for example, certificate-entered not required).
   - Report every survivor.
6. **No other behaviour change.**
   - Run frame_kernel `--lib` and `--test s11_site_table`, and product_physics `--lib`, on the candidate.
   - Compare the failure set with base `652ad0cc1f`. At base, product_physics fails seven tests: six `f1a_tests` and `s11g t13`. The candidate must add no failure.
   - Check the accounting change (16 B in `ProductProofTrace`; per-body copy charges) for honesty.

## Host and method

- **Your copy:** build from your own `git archive c618675e84` in `WT/rv77/`, with targets `WT/targets/rv77/{frame_kernel,product_physics}`. Delete the copy afterwards, but keep its logs in `WT/scratch/rv77_coverage_producer_01/`.
- **Cargo:** `--locked --offline`, `CARGO_BUILD_JOBS=4`, `RUST_TEST_THREADS=2`, one cargo job at a time. The memory guard must be running (`pgrep -fl memguard`). Other TASKs are working (I61 in `WT/f2a-coverage`, plus reader authors); don't touch their files.
- **Never:** Git writes, index operations, installs, new tooling, or solver-at-scale, DEC-025 or native jobs.

## Output

- **The report:** `NUM/R/REVIEW_RV77/coverage_producer_01/REVIEW.md`, containing:
  - a verdict, PASS or FAIL;
  - counts of BLOCKING, SHOULD-FIX and NOTE findings;
  - a findings table with path:line, evidence and remedy;
  - a short section per review item.
  
  Add your derivation, enumeration and mutant results as files there, with a SHA256SUMS. Use placeholder paths (`WT`, `P`) in committed text.
- **Time box:** 90 minutes from your first tool call.
- **End your turn** with a concise status for ROOT: the verdict, the counts with one line per finding, the report's sha256, and anything ROOT must rule on.
