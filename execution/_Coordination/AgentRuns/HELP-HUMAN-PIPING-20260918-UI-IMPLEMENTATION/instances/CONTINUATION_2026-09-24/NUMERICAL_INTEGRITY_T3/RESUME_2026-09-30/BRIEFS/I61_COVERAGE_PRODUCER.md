# I61: summary coverage in the producer's typed C3 seam

TASK (Type 2), dispatched directly by ROOT (HELP_HUMAN, Agent 0) as a background subagent. ROOT is your return path. You do not delegate.

## Read first

- `REPO_ROOT/AGENTS.md`, `agents/AGENT_TASK.md`, `projects/chirality-piping/AGENTS.md`, and this brief.
- **The selected design:** `R/I57/summary_coverage_01/ADDENDUM.md`, all of it. Sections 2 and 3 are your specification.
- **Its review:** `R/REVIEW_RV76/summary_coverage_01/REVIEW.md`.
- **The rulings:** in `T3/ROOT_RULINGS_V1.md`, read "Summary-coverage representation selected for the successor" and "Resumption by the next ROOT; coverage implementation planned".

Record the files you actually read, with their sha256, in your RETURN.

## Paths

- `WT` = the T3 worktree root on the M5 host. Your dispatch prompt gives its absolute path; committed text uses the placeholder.
- **Your worktree:** `WT/f2a-coverage`, on branch `codex/piping-f2a-coverage-20261003`, which starts at CODE `652ad0cc1f`. Its maintained source equals NUM's. Edit only there.
- `NUM` = `WT/numerics` (records; read from here).
- `P` = projects/chirality-piping
- `T3` = P/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3
- `R` = T3/RESUME_2026-09-30
- `FK` = P/core/solver/frame_kernel/src/structural/retained
- `PP` = P/core/product_physics/src

## Objective

The prepared proof already computes a complete per-body summary-coverage vector, `ProductCertificateSpent::summary_coverage()` (FK/product_certificate/final_case.rs:301). The certified form is `CertifiedProductProof::summary_coverage()` (:1758). But the typed C3 trace does not carry it: `ProductProofTrace` (final_case.rs:232) omits it, so `PP/retained_receipt.rs::project` cannot expose it.

Carry that proof-owned vector through the typed seam exactly as I57 §1–§3 specify. **This is the typed seam only:**
- no serializer, JSON or public receipt;
- no schema, reader or corpus change;
- no public activation.

The receipt transaction that will encode it is later work.

## What to implement

1. **A borrowed view in `ProductProofTrace`.** Add a borrowed, typed view of the proof-owned coverage slice. It must come from the proof's own work (`ProductCertificateSpent` / `CertifiedProductProof`) and never from `ProductCapture.summary_coverage`. That field is the adapter's fallible copy (PP/retained_product.rs:3362–3366) and can hold a partial prefix. Add no new solve, residual, nonzero scan or coverage computation.
2. **The projection in `PP/retained_receipt.rs::project`,** a typed C3 coverage value that is either null or complete:
   - **Null** when the proof retained no complete vector. On this source, an empty proof vector means null, never an all-false array.
   - **Complete** otherwise: exactly one entry per native body, in ascending body id 0..body_count−1, including zero and no-data bodies. A non-empty vector that does not cover every body makes the projection refuse.
   - **The stage rules in §3's table:**
     - no proof means no trace and no coverage object;
     - non-null coverage requires the same source/run references, a selected native Run, both lanes completed in their declared order, proof_start, projection, maxima, values and aliases completed, and the certificate entered;
     - a completed certificate, or a passed G5a, requires non-null coverage;
     - Ready is never null;
     - a complete vector does not imply that the certificate succeeded.
3. **The nine-flag cross-check (§2).** Write a typed function that takes the compact payload `{body, stop[4], has_data}` plus the public source facts and reconstructs estimate and charge. The public facts are:
   - per-kind layout presence from the bound source maps;
   - the body extent L, recomputed by `adaptive::body_extent` in native order;
   - the selected native verification resolution_scale force/moment values E;
   - the native p512 floor, if any;
   - native p.

   The function then compares body, stop, estimate, charge and has_data bit for bit with the actual `ProductSummaryCoverage` entries. A mismatch is an association/encoding failure under the existing ordinary transaction error, and the vector is never altered. Remember:
   - p512 charge equals the force/moment stop flags;
   - p128/p256 charge equals estimate;
   - the fixed 1024 proof/projection precision never changes p/P and never creates a floor.
4. **Accounting.** Account for the new borrowed view, the per-body projection and any copy under the existing typed trace-cost owners (for example `TraceCosts`). Claim no new allowance or M.

## Required tests

Each test names the I57 §5 control it covers. Use actual producer runs where the existing test fixtures make a case reachable; label anything synthetic as synthetic.

- **Bodies:** a zero/no-data body in a complete source keeps its entry with the actual flags.
- **Cancelled loads:** individual +x and −x free-DOF loads that net to zero give has_data = true. The existing cancelled-contribution test (PP/retained_product_tests.rs:1442–1500) is the source witness; build on it.
- **Absent kinds and extent:** an absent body/kind, and L = 0 against L ≠ 0, give exactly the derived estimate, with no invented coupling.
- **Precision:**
  - native p512 with a zero floor and with a positive floor gives charge = stop;
  - p128/p256 under the fixed 1024 product proof gives charge = estimate.
- **Failure prefixes:**
  - no proof means no trace;
  - a lane failure, projection failure or values/aliases abandonment gives null;
  - a certificate entered but failing before the summary assignment gives null;
  - a summary completed and then a later certificate failure gives the complete vector;
  - an adapter copy failing partway gives the proof-owned complete vector, not the adapter prefix;
  - Ready gives the complete vector.
- **Mutants** (record each patch as `_run_records/mutants/<id>.diff`). Each must be killed by a committed test, with a NONE control that passes:
  - sourcing coverage from the adapter copy;
  - mapping empty to an all-false array;
  - allowing null on Ready;
  - dropping one body;
  - p512 charge := estimate;
  - accepting a flag mismatch.

## Write fence

Only these paths in `WT/f2a-coverage`:
- `FK/product_certificate/final_case.rs`;
- `FK/product_certificate.rs` and the `retained_api` re-export in `P/core/solver/frame_kernel/src/structural.rs`, for a needed re-export only;
- `PP/retained_receipt.rs`;
- `PP/retained_product.rs`, only to pass the owner or trace through, with no behaviour change;
- `PP/retained_product_tests.rs`;
- `P/core/solver/frame_kernel/tests/retained_k4/product_final_case_tests.rs`;
- `P/core/solver/frame_kernel/tests/s11_site_table.rs`, only to declare a genuinely new integer site.

Anything else is a stop: return the exact need instead of editing.

## Host and runtime

- **Host:** the M5 Max. The memory guard `WT/guard/memguard.sh` must be running; check with `pgrep -fl memguard`.
- **Cargo:**
  - always `--locked --offline`, with `CARGO_BUILD_JOBS=4`, `RUST_TEST_THREADS=2`, and an explicit `--manifest-path`;
  - targets are `WT/targets/i61-coverage/frame_kernel` and `WT/targets/i61-coverage/product_physics`;
  - one cargo job at a time; I62 runs Python beside you, which is fine;
  - put a 1,200-second wall on each command.
- **Tests:** run focused tests while you work (for example, frame_kernel `--lib product_certificate`, product_physics `--lib retained_product_tests`, frame_kernel `--test s11_site_table`). At freeze, run frame_kernel `--lib` and `--test s11_site_table` once, and product_physics `--lib` once. `product_final_case_tests.rs` is a unit-test module that `final_case.rs` includes through `#[path]`, so it runs under frame_kernel `--lib`.
- **Never** build new host tooling, install anything, or run a solver at scale, DEC-025 or a native/UI job.

## Time box and stops

- **Two hours from your first tool call.** Stop new source edits at 1h45, then freeze and return. Report anything unfinished, not a silently narrowed result.
- **Stop and report** if:
  - the actual source contradicts the design (for example, the vector is not assigned atomically, or a stage seam differs from §3);
  - a required change falls outside the write fence;
  - a mutant survives;
  - setup takes more than 5 minutes.

## Output

- **No Git writes and no index operations.** Git reads are fine, with `GIT_OPTIONAL_LOCKS=0`. ROOT commits.
- **Evidence:** in `NUM/R/I61/coverage_producer_01/`. Write a short RETURN.md covering:
  - each changed file with its sha256 and line counts;
  - each test command with its result;
  - the mutants and their results;
  - each §5 control and whether it is an actual run or synthetic;
  - any deviation, and open items.
  
  Add a SHA256SUMS. Put bulk logs in `WT/scratch/i61_coverage_producer_01/`, listed with hash, size and path. Use placeholder paths (`WT`, `P`) in committed text.
- **End your turn** with a concise status for ROOT: what changed, the test and mutant results, anything unfinished, and anything ROOT must rule on.
