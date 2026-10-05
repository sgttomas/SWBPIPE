# RV82: independent review of U1 grant 1 and U2 (the private serializer and owner binding)

TASK (Type 2), an independent reviewer dispatched directly by ROOT (HELP_HUMAN, Agent 0) as a background subagent. ROOT is your return path. You do not delegate. **You did not write this code. Don't rely on the implementer's tests or its experiment emitter as your oracles.**

## The candidate

- **The commit** on branch `codex/piping-f2a-serializer-20261004`, against base NUM `43a6368c21`. Your dispatch prompt gives the exact head.
- **The change:** I61's U1 grant 1 with U2, inside this fence:
  - `PP/src/retained_product.rs`: the typed capture fields (G-l, G-b, the two eligibility flags) and `certificate` made private with accessors (U2);
  - `PP/src/lib.rs`, only at the G-l and G-b ordinary capture sites, each guarded by `if let Some(observer) = product.as_deref_mut()`;
  - `PP/src/retained_receipt.rs`: the C3-seam projection;
  - the new `PP/src/retained_wire.rs`: the JSON projection and hashes;
  - `FK/.../product_certificate/final_case.rs`: only `CertifiedProductProof::owner_matches`;
  - tests: `PP/src/retained_product_tests.rs` and the new `PP/src/retained_wire_tests.rs`;
  - `PP/Cargo.toml` and `Cargo.lock`: a `result_export` **dev-dependency** only.
- **The implementer's account:** `R/I61/u1_serializer_01/RETURN.md`, with its mutants and outcome files.

## Read first

- `REPO_ROOT/AGENTS.md`, `agents/AGENT_TASK.md`, `projects/chirality-piping/AGENTS.md`, and this brief.
- **The grant:** `R/I61/receipt_experiment_03/RETURN.md`, "U1 grant-1 brief proposal", as confirmed by ROOT.
- **The rulings** in `T3/ROOT_RULINGS_V1.md`:
  - "T1 and T2: I61's analysis and the rulings" and "T1 confirmed by the owner" (option (a));
  - "Step 4 planned: decisions and dispatch" (decisions 1–9, D38);
  - "Experiment 03: the real facade produces the same certified receipt" (D39, the grant's confirmation).
- **The contract:** C1, C2 and C3 with completions 06–08 (located by `R/I61/step4_plan_01/PLAN.md` §1), and the accepted schema and fixtures at NUM (read-only).
- **U2's origin:** `R/REVIEW_RV77/coverage_producer_01/REVIEW.md`, finding N4 and §4.

## Review, in priority order

1. **Ordinary bytes are untouched.** Read the `lib.rs` and `final_case.rs` diffs line by line. Every new write must sit inside the observer guard, and no diagnostic, debit, message text or control flow may change. Then run PP `--lib` and the PP, runner/headless and result_export integration suites on the candidate and on base. The failure sets must match. The expected Mac platform failures at base are exactly `t13_committed_fallback_uz_is_byte_identical` and the two runner_headless `load_reference` tests. Confirm that controls A, B and B′ are committed tests, and that the milestone's ordinary bytes stay `9c7ec1a1…` (sparse) and `21ca629c…` (dense) with capture installed and absent.
2. **Contract fidelity of the receipt.** For the milestone case in both modes, derive the expected `retained_precision` member yourself from the contract, the rulings and the native state. Do not derive it from the emitter. Cover:
   - T1 (a): no legacy `SOURCE_BLOCK_RECOVERY_UNAVAILABLE` on the retained-selected case, `legacy_source = {unavailable, null, work_ref k}`, and `legacy_source_work[k]`;
   - the D39 disposition table: every producer branch reachable in the fence, with typed `source_eligible` and `needs_source_recovery` (not a merged pair);
   - decision 2 (A2/D6a): the exact per-case `diagnostic_refs`;
   - decision 1 (A1): raw SHA256 of the K4RST bytes;
   - G-a's fixed `RETAINED_PRECISION_SELECTED` text, G-d's typed `support_indices`, G-e's `not_covered` from the certificate verdicts, and G-j;
   - the definition and table hashes from the in-tree fixtures.
3. **Byte identity with experiment 03.** The output must equal `bca4e9ca…2e59` (sparse) and `06449153…f310` (dense), with receipt hashes `2c8cee1a…` and `dbcc7dd0…`. Any listed difference must trace to a closed gap (G-d, G-e, A1 or G-a's text). Check each listed cause against the bytes yourself.
4. **The three accepted readers** pass G0–G8 on the serializer's output, with eligibility off. Run Python, TypeScript and Rust with your own invocation, and compare classification parity (expected 98/98 and 99/99).
5. **U2, the owner binding.** Confirm that `certificate` is no longer crate-writable on either holder. Check `owner_matches` against `ProofAnchor::matches_owner` for soundness: it must refuse a foreign owner whose public facts are identical. List every path from a proof to the projection and confirm that each reaches the check, ending in `receipt_failure{association}` on a mismatch.
6. **Failure discipline and checked work custody.**
   - Every variant that grant 1 does not translate must return a typed `receipt_failure` naming it. No Debug text may reach the wire, and no non-test build may panic. Check `unwrap`, `expect`, indexing and arithmetic on the projection path.
   - Every work amount must be read through its checked view (the record's `checked_*` accessors, `StageWork::checked_total` with its status, `RunWork`, `InvocationMeter::checked_charged`, `checked_lme`), and it must be exact or abandon. No legacy saturating `u64` field may be used, and no per-stage slot may be emitted under a faulted status. These are the U1 obligations in `R/I65/u4_g2_01/D4_RECONCILIATION.md` §3 and ROOT's ruling "Checked work custody in U1".
7. **Unreachability.** The serializer must be private and unreachable from every public entry. The dispatch and `retained_memory.rs` must be untouched, and no permit or test permit may exist in maintained code (decision 7).
8. **Mutants.** Re-run I61's mutants and its NONE control from a clean `git archive` of the candidate. Add at least four of your own. Suggestions: a G-b member read from the wrong report; the D39 `not_required` row folded into `not_eligible`; the D6a filter admitting a `RETAINED_PRECISION_*` diagnostic; `owner_matches` comparing only public facts; a hash computed over non-canonical text. Report every survivor with its reason.
9. **Nothing weakened, and the fence.** Confirm that the diff from base touches only the fenced paths, that `Cargo.toml` adds only the dev-dependency, and that no reader, schema, fixture or existing check is changed.

## Host and method

- **Your copy:** build from your own `git archive` of the candidate in `WT/rv82/`, with targets `WT/targets/rv82/`. Keep logs in `WT/scratch/rv82_u1_serializer_01/`, and delete the copy afterwards.
- **Cargo:** the default toolchain (do not set `DEVELOPER_DIR`), `--locked --offline`, `CARGO_BUILD_JOBS=4`, `RUST_TEST_THREADS=2`, one cargo job at a time.
- **The TypeScript reader** needs NUM's prebuilt `public/wasm-engine` and `public/self-weight-engine`, copied and not built, and the `node_modules` link. Disclose both.
- **The memory guard** must be running (`pgrep -fl memguard`). Other TASKs are working (I65 on U4 records); don't touch their files.
- **Never:** Git writes, index operations, installs, new tooling, or solver-at-scale, DEC-025 or native jobs.

## Output

- **The report:** `NUM/R/REVIEW_RV82/u1_serializer_01/REVIEW.md`, containing:
  - a verdict, PASS or FAIL;
  - counts of BLOCKING, SHOULD-FIX and NOTE findings;
  - a findings table with path:line, evidence and remedy;
  - a short section per review item.
  
  Add your derivation, reader runs and mutant results as files there, with a SHA256SUMS. Use placeholder paths (`WT`, `P`, `PP`, `FK`, `NUM`) in committed text.
- **Time box:** 3 hours from your first tool call. Report anything unfinished.
- **End your turn** with a concise status for ROOT: the verdict, the counts with one line per finding, the report's sha256, and anything ROOT must rule on.
