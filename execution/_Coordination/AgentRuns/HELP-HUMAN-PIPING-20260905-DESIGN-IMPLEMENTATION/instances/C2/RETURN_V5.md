# C2 pre-closeout verification and release plan

Agent1 CHANGE `/root/sync_main`, parent `/root`; no children. Delegated-harness-native; role/scope instruction+config asserted. All writes confined to authorized policy entries, current-run exact-path `.gitattributes`, and own C2 records. No Git mutation, desktop build, native build or DEC-025 sweep occurred in this stage.

## Current candidate

Policy SHA-256 `c3b807f30947f021a2b2258bbe5292f8ca4b5bb3f2aaa2e51c0d8f3ad8d3aabd`: exactly three approved EVIDENCE historical overrides relative to HEAD; older policy unchanged. Third actual root acceptance is `dispositions/ACCEPT_C2_W2_EXECUTION_RETURN_CLASSIFICATION.json`, SHA-256 `f5477a0a21e3496f971936280e20d099dad706a8ef603b868fcf79ff3dfa666c`. Its target TASK_RUN.md SHA-256 `2f34ffbc8365af8efcfff1ef95f51b07453c1e92d63e0db130d7236c0e3f9f7d` is preserved. Policy parses without issues; all three targets classify EVIDENCE. All original 77 inputs still match C0 preservation manifest.

Current-run `.gitattributes` SHA-256 `a458ff80130d674279e734b6412ec8e2f81044a4d618a280eda39803439fd1c9` uses eleven exact relative paths with `-whitespace` only. Root expressly authorized this limited preservation of completed terminal output and frozen diff context. It does not suppress text diffs or alter merge/text behavior. Inventory/hashes: `_run_records/PRECLOSEOUT_V5/RAW_WHITESPACE_INVENTORY.json`. No original file was rewritten. Own unsealed practitioner failed-output bytes were preserved losslessly in base64 JSON instead of raw whitespace-bearing text, with original name/hash and decode equality.

Caveat: candidate whitespace's untracked scanner ignores the whitespace attribute (it checks diff/text/binary only). Therefore preserved untracked raw files continue to trigger its pre-stage scan. Git's staged/committed `diff --check` respects `-whitespace`; after authorized staging the ordinary complete candidate check can validate them correctly. No binary exemption or root-tool change is proposed. Never suppress unrelated whitespace findings.

## Executed checks

Logs/encoded exact output and command records are in `_run_records/PRECLOSEOUT_V5/`.

- Piping full pytest: PASS, 1011 tests.
- Practitioner harness first attempt: FAIL, 1 failure/349 passes, current-tranche portability findings in N1/PREFLIGHT.json and W2/I1/TASK_RUN.md. These were actual tranche issues, not baseline failures.
- After N1 preserved its original bytes under `_run_records` with a portable pointer and the approved third override classified immutable W2 history: full practitioner harness PASS, 350 tests; path-anchor validator PASS.
- Validation suite: PASS, 321 tests.
- Receipt validator: PASS before the new receipt.
- G0, G1, G2, G3 and G4 whole-corpus validation: PASS.
- Initial self-check: exit 0 with the same portability findings, not interpreted as a clean pass merely because exit zero. Later frozen-tree self-check remains part of final checks.
- Candidate whitespace: initial FAIL on preserved raw patch/log bytes. The scoped attribute repair is applied; final post-stage candidate run remains required. Other current writer artifacts may add candidates before freeze.
- Affected-check dry-run selects exactly practitioner_harness and validation; both complete suites have run. No root tools change causes other routed suites to activate at this candidate.

## Exact command basis

Use the provisioned Python via a task-local `PIPING_PY` variable assigned to its verified temporary environment; do not use system Python for missing dependencies. Executed equivalent commands from `{REPO_ROOT}`:

```sh
"$PIPING_PY" -m pytest -q tools/practitioner_harness
"$PIPING_PY" -m pytest -q -n auto --dist loadscope tools/validation
"$PIPING_PY" tools/practitioner_harness/harness.py self-check
"$PIPING_PY" tools/validation/validate_path_anchors.py --text
"$PIPING_PY" tools/validation/validate_piping_loop_receipts.py
"$PIPING_PY" tools/validation/validate_root_materialization_fence.py
"$PIPING_PY" tools/validation/validate_root_harness_adapter.py
"$PIPING_PY" tools/validation/validate_root_surface_ownership.py
"$PIPING_PY" tools/validation/validate_root_work_graph_dispatch.py
"$PIPING_PY" tools/validation/validate_instruction_tranche_manifest.py
"$PIPING_PY" tools/validation/validate_candidate_whitespace.py --base-ref 7458e9c1eb9399ed259da464207d9a507acdea2e
"$PIPING_PY" tools/run_affected_tests.py --base 7458e9c1eb9399ed259da464207d9a507acdea2e --dry-run
```

Piping suite from `{WORKING_ROOT}`: `"$PIPING_PY" -m pytest -q tests -n auto --dist loadscope`.

After a clean source commit and root release, from `{WORKING_ROOT}`: `"$PIPING_PY" tools/release/run_evidence_sweep.py --execute` with verified native/Wasm dependencies and exact host escalation. The complete sweep independently re-executes its five registered surfaces; no selection/dirty-output flags satisfy full clean-commit proof. After source/evidence commits, final G4 range command is `"$PIPING_PY" tools/validation/validate_instruction_tranche_manifest.py --base 7458e9c1eb9399ed259da464207d9a507acdea2e --head HEAD --added-manifests-only`. CI-equivalent routed command is `"$PIPING_PY" tools/run_affected_tests.py --base 7458e9c1eb9399ed259da464207d9a507acdea2e`; do not repeat already passing suites absent subsequent relevant change or final-tree obligation.

## Minimal closeout scope

Provisional source/test/config inventory contains 16 paths; exact list in `_run_records/PRECLOSEOUT_V5/ARTIFACT_INVENTORY.json`. It includes the W7 workspace/UI sources and tests, native window config, W2 product_physics source and temperature fixture, and project portability policy. Add only parent-accepted later source changes. Required inputs: both unchanged planning documents and both complete prior accepted research runroots. Current required implementation evidence includes currentrun `.gitattributes`, sealed briefs, parent approvals, manager/child returns, snapshots, independent review, witness outputs and hashes.

At inventory time 407 nonignored untracked files total 15.34 MB. No compiled artifact candidates (.wasm/.dylib/.so/.o/.a/.exe/.zip/.dmg) appeared. Largest item was an approximately 1.29 MB failed terminal log; source copies and raw analysis envelopes are below 0.63 MB each, with PNG evidence screenshots. These are bounded reproducible evidence, not build products. Do not force-add ignored target/dist/app bundles. Repeat size/manifest containment inspection once writers freeze; keep all 77 original inputs unchanged.

Current candidate is entirely under projects/chirality-piping. G4 protects root AGENTS/CLAUDE, agents, skills, tools, docs, init and .github/workflows prefixes; current candidate intersects none. Thus no new root tranche manifest or root governance write is required. Final committed-range G4 still verifies this claim; add a new manifest only if root protected scope actually changes.

The committed selected loop plan is WORKPLAN_2026-07-18b_piping_loop.md, regular blob `61dbbca25b9be766383aa1e5a743a021ce4d63d1`, loaded from HEAD. Step5 requires owning managers' affected deliverable-local `_STATUS.md` Remaining/history, paired `MEMORY.md`, and `_run_records` updates reflecting landed scope. Do not claim automatic lifecycle promotion or update unactivated deliverables. Read status/memory together. These writes require root's precise activation/closeout fence; C2 has not performed them.

Append one minimal versioned receipt to `loop/LOOP_RECEIPTS.md` under the existing contract (next available ID currently Receipt-131, actual parent basis Receipt-130, Step0 Examined-Through `7458e9c1eb9399ed259da464207d9a507acdea2e`). It should point to the owner-direction artifact, accepted fan-in, final review, source/sweep evidence, attribution and unresolved owner gates. Checks are pass/fail-only; numerical details remain in run evidence. No new NEXT_INSTANCE_STATE file: the project has none. Revalidate receipt after append and before commit.

## Release ordering

Root releases the source freeze only after W7 repair/review, W2 closure, integration review and accepted metadata fan-in. CHANGE then stages the explicit accepted inventory, rechecks containment/hashes/whitespace, commits locally and verifies clean whole-repository status. Run the full clean five-surface sweep; read every verdict. Parent accepts source-bound results and authorizes an evidence-only closeout commit preserving the complete reviewed source/configuration hash set. Recheck final-tree governance and committed-range gates, verify upstream ancestry, then ordinary fast-forward push and PR. No merge/rebase/force or hidden skip. Any source/config change after freeze requires review and new source-bound sweep. Keep prior failed executions as evidence.

## Handoff

Readiness is CONDITIONAL on source freeze, current metadata, final review, staged whitespace validation and clean committed-source sweep. C2 has not claimed global pass, committed, pushed or opened a PR. Exact bounded classification/config changes are ready for integrated review. Existing report/map and numerical witness limitations remain unaltered. No decomposition, lifecycle or professional acceptance is conferred by this closeout plan.
