# R independent review return

Verdict: `CHANGES_REQUIRED`

Source: `779dedb8670625b36af07b89fc5557470e47c50e`

Reviewed inventory: `REVIEWED_INVENTORY_V1.sha256`, 105 files, inventory-file SHA-256 `9f59ef28fd7267ba95f2749e1ac81ab1cd4f8f27e90a24422c82a7e30058ad89`. The inventory was frozen before substantive review and all listed member hashes were revalidated.

## Actionable findings

1. **R-01 — RP active launch brief is not portable.** `instances/RP/LAUNCH_BRIEF.md` line 7 embeds `/Users/ryan/.codex/worktrees/8728/chirality-physics-ui-execution-20260908`. Active briefs must use repository-relative paths; exact host paths belong only in structural `_run_records`. Route a byte-only portability successor through the owner/author and preserve the original bytes as provenance. RP's already verified physics review need not be repeated for that normalization alone.

2. **R-02 — PS child-execution evidence is incomplete.** `instances/PS/PS1/` contains `SEALED_BRIEF.md` and `RETURN.md`, but has no child `STATUS.json`; `instances/PS/` has no recorded internal work graph and no explicit manager status/handoff record. This leaves the executable PS -> PS1 parentage, final child state, and manager closure below the durable-evidence contract in the repository and project `AGENTS.md`. Add transparent records for actual parentage, scopes, chronology, status, and handoff; do not rewrite the already returned factual evidence.

3. **R-03 — CHANGE setup has no durable launch/status record.** `execution/_Change/PHYSICS_UI_EXECUTION_20260908/MERGE_AND_LANE_SETUP.md` records `/root/implementation_change` as an Agent 1 child, but no matching launch brief or status record is present in the reviewed run evidence. Add a transparent retrospective record for the scope actually followed and actual chronology. It must not claim pre-launch persistence or pristine conformance.

4. **R-04 — F4 zero-normal branch consistency is underspecified.** `instances/F4/IMPLEMENTATION_BRIEF_V1.md` permits a zero prior reaction to take the base solve's sign, but its acceptance gate only says to retry when a *nonzero* sign changes. It does not explicitly reject an assumed zero branch whose final current reaction is nonzero, or an assumed nonzero branch whose final reaction is zero. That gap can permit a force inconsistent with `f_t = -direction * mu * abs(r_n(current))`. Clarify the exact discrete branch comparison for zero/nonzero transitions and use the existing retry/cap behavior. This clarification must not introduce a tolerance or another acceptance criterion.

## Verified coherent interfaces

- `CORRECTION_INDEX_V1.json` SHA-256 `50a7173e92ddf8adc79bbe645f6504fc48bb59570e36bfd6429c1808fa73fcaa` and `CONTROL_FREEZE_V3.json` SHA-256 `a44ea77f389e48fbb7490619623c3a039908c07f711b1e80208822f06772ed04` correctly distinguish historical logical hashes from normalized successor hashes. Older hashes in PS evidence bindings are therefore provenance, not unresolved mismatches.
- The PS candidate manifest SHA-256 is `60e70cab2e6bbf99468d006aa2ffe397f3ac8fe2d3a3690e089ac3e890c68a8e`. It covers exactly 33 unique nonaccepted Q1 rows and preserves the registered consumer, target, maturity, and disposition data.
- Current D-66 SHA-256 `578087fbad9ed71e61ea7a22acb3b42ede46684669d16cd2b5d480e0ffae904f` and register SHA-256 `dc141013a5d30f6d58c1f392a6c6f4f0af2e8f1b7dd809ea3c11ddfef5116679` agree on `PARTIALLY_RULED`, the selected first wave, and the remaining holds.
- F4 source/reproduction bindings, affine coupling equation, frozen positive and negative numeric expectations, and failure/public-interface fences are consistent apart from R-04. U7's nine-file future write fence, 15 UI rows, backend/schema exclusion, and incomplete node-only save/reopen behavior are consistent.
- Product source is unchanged at the frozen source. Raw host paths in reviewed F4/U7/CHANGE structural `_run_records` are provenance and do not activate nonportable controls.
- RP manifest SHA-256 `89d474de34d00c82676675865b746945866f346efdda04592b056260892deb7c` and reviewed-inventory SHA-256 `6676c17a5b8cea47e09a1bcedd13261f5ad9fa45ac608c117aa966c048c5579d` form a coherent bounded `PASS` interface. Its retrospective durable-brief timing remains disclosed; this review does not inspect the P5 body.

## Limits and release state

This is a staged review, not final tranche acceptance. K8/M9 bodies and RK/RM current write scopes were excluded. K8's original review was `CHANGES_REQUIRED`; its successor remains in backcheck. M9 is frozen and under RM review. Their final accepted returns and required tests remain later fan-in gates.

The cross-packet record must retain that K8 sent one direct message to idle RK before root mediation, contrary to hierarchy mediation. The message did not trigger execution; K8 recorded `instances/K8/HIERARCHY_MEDIATION_DEVIATION.md` SHA-256 `ee536a99ab961d0c21540b8157c4028d124b269c071ccf5f462844bcfb1d93e9`, and root independently verified the 14 outputs and alone triggered the backcheck. P5/RP retrospective brief timing and the disclosed M9 compile-slot overlap likewise prevent a pristine-procedure claim.

F4 and U7 remain `HELD` pending the unanswered Owner one-time Step 1 exception. No dependency, physics, compatibility, public-interface, lifecycle, or Git adoption is inferred. R performed no source edits, Git writes, Cargo invocation, full harness, or product tests.
