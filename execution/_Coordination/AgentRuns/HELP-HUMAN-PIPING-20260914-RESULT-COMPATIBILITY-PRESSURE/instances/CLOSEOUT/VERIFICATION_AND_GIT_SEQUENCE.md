# Final verification, receipt, CI and Git sequence

This is an ordered procedure for HELP_HUMAN/CHANGE after the candidate is immutable. Commands are proposals and were not executed by this preparation task.

1. Collect every writer/refutation return. Verify its declared hashes against live files. Reject the fan-in if a source file changed after its review/refutation or if an unapproved path appears.
2. Freeze `git diff --name-status origin/main...HEAD`, untracked candidates, submodule state if any, and hashes for every changed maintained source/schema/fixture/test file. Map only actual behavioral paths to the consumer inventory in `CLOSEOUT_INVENTORY.md`.
3. Inspect source/lock/schema/fixture boundaries before broad checks:
   - Existing 0.1 fixture bytes and serializers remain available and version-dispatched; any active schema `$id`/`$ref` adjustment is not described as byte-identical legacy content.
   - Unversioned schemas are dispatchers without duplicate `$id` resources; all version schemas resolve locally from a fresh checkout.
   - Checked canonicalization additions do not alter existing model, persistence, input-manifest or operation hash profiles.
   - Private canonicalizer setup builds explicitly; runtime code never invokes Cargo, searches `PATH`, or falls back to another serializer. No `target/` artifact is tracked.
   - Each changed `Cargo.lock` is owned by a changed crate graph and `--locked` checks consume it. Avoid unrelated lock churn.
   - Maintained tests/fixtures do not import dated AgentRuns evidence. One-time raw evidence stays in this run; reusable regressions stay under maintained `tests/` and `fixtures/`.
   - Dev and dist Playwright specs are discovered by their respective configs; source replacement and asynchronous hash completion are covered.
4. Run focused writer suites and the pressure refutation against the exact frozen files. Preserve each raw output once in its owning run record. A repaired known failure needs a direct backcheck; a later green aggregate alone is insufficient.
5. Run the serialized native/GUI procedure against the same source. Bind native build inputs, generated WASM, app source, saved files, model/input-manifest identities and exact candidate hash. Do not use the dormant pressure kernel result as runtime-pressure evidence.
6. Apply the exact deliverable metadata/referral updates from `CLOSEOUT_INVENTORY.md`, using only proven final wording and immutable pointers. Add Receipt-141 from the prepared template. Record the DEL-17-06 semantic/lensing/dependency derivative deferral in both deliverable state and the final handoff if regeneration is not performed.
7. Freeze and independently review 100% of the complete maintained source/test/schema/fixture diff. Also perform a separate additive closeout review over metadata, receipt, handoff, manifests and evidence placement. Resolve findings and reassess affected checks after any change.
8. Run candidate-range and structural checks from repository root:

   ```text
   git status --short --branch
   git diff --name-status origin/main...HEAD
   git diff --check origin/main...HEAD
   python3 tools/validation/validate_conflict_markers.py --base origin/main --head HEAD
   python3 tools/validation/validate_candidate_whitespace.py --base-ref origin/main
   python3 tools/validation/validate_piping_loop_receipts.py --repo-root . --validation-commit HEAD
   python3 tools/validation/validate_path_anchors.py
   python3 tools/practitioner_harness/harness.py self-check
   python3 -m pytest -q tools/practitioner_harness
   ```

   Candidate whitespace is diagnostic under the live rule. Conflict markers, receipt structure, path containment and BLOCK-level harness findings remain gates. Use the task-local Python recorded by Step 0 if system Python lacks the registered dependencies.
9. Commit the integrated technical candidate with only authorized paths staged. Require a clean working tree, then run the complete registered sweep from `projects/chirality-piping` with host capability:

   ```text
   python3 tools/release/run_evidence_sweep.py --execute --require-capability host
   ```

   The complete order is Cargo crate sweep, repository Python tests, WASM build plus desktop Vitest, dev and dist Playwright, then desktop production build. The summary must say complete, pass, clean, and bind the technical commit. A partial capability run is not the DEC-025 gate.
10. Preserve the canonical sweep summary and final acceptance/handoff/receipt in the evidence closeout commit. Because the sweep writes after inspecting a clean source commit, do not claim it binds the later evidence-only commit. Verify the evidence-only delta contains no maintained product behavior. Re-run receipt validation, self-check, practitioner-harness tests, conflict-marker/range checks, and any check selected by the actual additive delta. Independent additive review must cover that delta. Repeat DEC-025 only if source/build inputs, maintained fixtures, schemas or tests changed after the bound sweep.
11. Re-run receipt validation after the final append and before commit. Receipt-141 uses Step 0 `Examined-Through` `eff9a58dd712ff9673fa26b9fa809a2f725f6f97` and parent `Receipt-140`; if HELP_HUMAN performed a later genuine Step 0, use that later examined commit instead and update the template before append.
12. Inspect staged paths and commit. Fetch upstream before push. If the remote branch advanced beyond local ancestry, stop rather than merge/rebase/force-push. Push the scoped branch and open/update one PR under standing authorization.
13. Required CI visible from repository configuration is at least `governance-harness` for every PR and `Piping Desktop E2E / Desktop E2E (source mode)` for `projects/chirality-piping/**`. Query the PR's actual required-check set after publication; branch protection may require additional checks not inferable offline. The desktop CI is source-mode only, so the local complete DEC-025 dev+dist evidence remains necessary.
14. Require every required check to pass on the exact PR head and confirm the PR diff equals the reviewed/packaged candidate. Any new commit requires affected review/check reassessment. Merge under the standing D-8 authorization only with no unresolved blocking review finding. Verify the resulting merge contains the PR head. Git publication does not change deliverable lifecycle, dependency satisfaction, release status or professional reliance.

## Packaging pitfalls to reject

- A clean DEC-025 summary bound to a predecessor followed by unreviewed source/schema/test changes.
- A receipt that copies test counts or raw evidence instead of pointing to one owning artifact.
- `Gate-Outcome: PENDING`; the validator admits only `STOPPED`, `EXECUTED`, or `AWAITING_OWNER` followed by an em dash and reason.
- Receipt fields repeated, nested bullets, headings/tables in the versioned body, more than twelve records, or an `Examined-Through` SHA not reachable from the validation commit.
- Treating explicit `unverifiable` legacy evidence as verification, or changing received historical labels/digests.
- Updating `Dependencies.csv`, DAG, decomposition, lifecycle, release or owner dispositions because the bounded implementation passed.
- Leaving `_SEMANTIC.md` / `_SEMANTIC_LENSING.md` or `_DEPENDENCIES.md` apparently current without regeneration or the explicit deferral required by the inventory.
- Runtime/test imports from this dated run, committed build targets, duplicate schema identifiers, or a private adapter that silently compiles/falls back at runtime.
