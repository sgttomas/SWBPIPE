# N5 Change Whitespace Normalization Verification Brief

- RequestedBy: HELP_HUMAN
- RunID: HELP-HUMAN-PIPING-20260811-DEC025-MECHANICS-RUNNER-BINDING
- ParentInstanceID: CHANGE-GITDIR-RECOVERY
- ChildInstanceID: N5-CHANGE-WHITESPACE-NORMALIZATION-VERIFY-01
- Role: fresh non-repairing managed verifier
- Package/deliverable: cross-package representation-only closeout
- Status at dispatch: SEALED

## Accepted basis

- Base commit: `f1e311fb7ab1c2a0800b1d32c59445368428dee9`
- Frozen preservation commit: `e2f675664bfc2cd361bbdb3a2c9d5f5f67c5e32d`
- Frozen preservation tree: `daae3d4450e6d40464979bfd43d5e1bb07e0b65a`
- Frozen preservation payload: exactly 59 paths
- Adopted candidate historical blob: `6172a87080aec82a9bf22cf56435cf8f0368a847`
- Original eight-row manifest SHA-256: `7d8a43c5b7be10fa5ed426c7ff01aa866389fb8faa22c0ccced14692d9197f55`
- Projected normalized eight-row manifest SHA-256: `56b5b3ace4b012e737a64a4029821ba3d56a056b7f8151dfd20255fdcb57f639`
- STEP 1 cached-check stdout: 1,467 bytes, SHA-256 `26028f7739165fcce4ce5815d0fffdd31bbabbdce7ed8fe020dec37213e64527`, exit 2, stderr empty, exactly eight bound `new blank line at EOF` findings

## Objective

Independently verify the owner-authorized representation-only normalization. Confirm that exactly one final LF and no other byte was removed from each of the eight bound Markdown files, the frozen STEP 1 identities remain reachable and truthful, the current normalized identities equal the projected manifest, and no product, policy, scope, lifecycle, evidence-result, or execution-history meaning changed.

## Permissions and scope

- Repository write scope: none.
- External temporary logs/build/cache scope: a fresh directory under `/private/tmp`, with all outputs externalized.
- Network: prohibited.
- Repair, normalization, staging, commit, push, PR, merge, fetch, rebase, reset, clean, deletion, dependency provisioning, and any other mutation: prohibited.
- Read scope: the complete recovery clone, STEP 1 commit/tree, the eight current normalized files, the accepted N1/N2/N3 records, the two implementation sources, DEC-046 C-B, and applicable validators/tests.
- Durable return: structured PASS or HOLD delivered to the parent; the parent materializes it only into the authorized `VALIDATION.md` and `STATUS.json` records.

## Required checks

1. Verify STEP 1 commit, tree, sole parent, 59-path payload, and historical candidate blob exactly.
2. Reproduce the original and normalized eight-row manifests using `path<TAB>sha256<TAB>git_blob<TAB>size<LF>` in C path order.
3. For every row, prove the current file is exactly the STEP 1 blob minus one final LF, with no other byte difference.
4. Prove every STEP 1 old blob is reachable at the frozen preservation commit and every normalized blob is current in the worktree.
5. Verify substantive and rendered Markdown equivalence for all eight files.
6. Verify the two implementation source files and DEC-046 C-B are byte-identical to their accepted identities.
7. Independently reproduce 25/25 mechanics cases, 206/206 values, the newly supplied 14/115, exact original 11/91 projection, all six fail-closed behaviors, unchanged stress behavior, nonlinear 5/5, and byte-identical nonlinear-only DEC-046 C-B.
8. Parse all candidate JSON and JSONL records applicable to the run.
9. Require the normalization candidate to contain exactly eight modified files plus the five owner-authorized new records, with no other candidate path.
10. Require candidate-whitespace validation, `git diff --check`, and exact path containment to pass; require zero staged, ignored, or out-of-scope drift.

Any mismatch returns HOLD. The verifier never repairs.
