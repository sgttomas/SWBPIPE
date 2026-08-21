# HELP_HUMAN orchestration plan — SCA-009 continuation + CI/support hardening

- RunID: `HELP-HUMAN-PIPING-20260821-SCA009-CI-SUPPORT`
- Plan version: `3` (`v1` frozen before implementation; `v2` recorded failed-node exclusion; `v3` records the owner's ordinary-repair resumption and resolved all-node fan-in)
- Selection authority: `HUMAN` — standing direction in the 2026-08-21 Piping iteration steer; this replaces default Step-0 selection for this run.
- Posture: `MIXED`
- Authorized width: `N=4`
- Branch: `codex/piping-sca009-ci-support-20260821`
- Exact basis: `b1876a5e0f0083e10c0c18255cd92ed0079b63a2` (merge of PR #598), which is also the launch `HEAD`.
- Accepted authority: `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.12; `execution/_ScopeChange/_LATEST.md` at accepted SCA-009; `DEL-07-09` in the PKG-07 decomposition table; the existing `check_claimed_model_hash` operation-applier gate; `execution/_Coordination/NOTICE_2026-08-20_PIPING_SCA-009_DEL-07-09_VOCABULARY_PALETTE.md`; and the SCA-009 `Vocabulary_Annex.md` NORMATIVE-NOW landing ledger.
- Integration owner: `HELP_HUMAN`.
- Closeout shape: one tranche, one branch, one PR against `main`; predecessors are merged. N2 commit `b988d9d0e4a7048ac28a73bbe53ce045c631dff8` and N3 commit `ffbc4834389f3095d22896e126b39085c3e00369` are pushed on PR #599; N1, N4, and final shared/control surfaces remain for CHANGE in dependency order.

## Frozen graph and execution order

1. N1 is the prerequisite and sole shared-instrument owner. It freezes this package, then performs only SCA-009's mechanical downstream obligations.
2. N2, N3, and N4 may run concurrently after this plan and `WORK_GRAPH.json` are frozen. Their declared write sets are disjoint.
3. No node writes shared closeout surfaces. At fan-in, `HELP_HUMAN` writes deliverable-status effects, the coverage-ledger closure, final AgentRuns returns/status/handoff, and the iteration receipt exactly once.
4. Mandatory fresh independent read-only review applies to all frozen changes under `core/**` or `apps/desktop/src/**` before the DEC-025 sweep and publication.

## Exact owner steer retained by this plan

- N1: PREPARATION scaffold DEL-07-09; rebuild current DAG-008 for `+DEL-07-09`, `+SOW-077`, and exactly the three additive edges `DEL-07-09 -> DEL-16-01`, `DEL-07-01`, `DEL-07-02`; extract schema-v3.1 `Dependencies.csv`; targeted RECONCILIATION current-authority refresh for DEL-07-09, DEL-07-03, DEL-07-01/02, DEL-16-01 and R-005/R-006 supersession. Formal AUDIT_DECOMP may ride only if an in-session runner clearly supports it; otherwise park it by name. Estimate/schedule is advisory stale only and is not recomputed.
- N2 product: prioritize component creation. Retire the `insert_component_symbol` completion-plan-A3 guard only in favor of an implemented `create_component` resolver with explicit geometry/connectivity inputs in DEL-16-01 territory, plus viewport tools in DEL-07-01 territory and inspector forms in DEL-07-02 territory. If too large, take resolver plus one component kind end-to-end and record the remainder against coverage rows. If blocked for a recorded reason, use the authorized `hanger.*` / `nonlinear.*` authoring field-path alternative. DEL-07-09 owns contract and coverage only; implementation never lands there.
- N3 CI hardening: disable Playwright GitCommitInfo collection or prefetch merge-base, with reason; add a bounded, legible step timeout to governance-harness Candidate whitespace; update coupled policy tests; and ship all required G4 tranche manifests. This is preventive hardening; do not reproduce slow-object serving and do not burn reruns. Root governance-harness edit is explicitly authorized.
- N4 product: correct `product_physics` support-family mapping so named partial-DOF restraint families emit solver `LineStop` and `VerticalSupport`, with focused tests, in DEL-04-03 territory.

## Shared rules and exclusions

- Product implementations close NORMATIVE-NOW coverage rows only by pointing to landed evidence at the deliverables named in the ledger's `Implementation lands in` column.
- Shared-surface writes occur once at fan-in by `HELP_HUMAN`; siblings do not write or reconcile those surfaces.
- No `artifact-proof` label; no node requires the unsigned-artifact macOS lane.
- Parked owner holds are unchanged and excluded: PKG-02 runtime/transport/permission choices, `.opsproj` policy, PDU-011/PDU-047, and `MAINTAINER_REVIEWED` promotion.
- No lifecycle promotion, release, publication, professional acceptance, certification, sealing, authentication, or code-compliance effect.
- Initial failure rule: retain what passed, record the failure, and stop. The owner later withdrew that rule for ordinary repair/closeout with the exact direction recorded in `OWNER_DIRECTIONS.md`; `N1_RESOLUTION_V2.md` and `N4_RESOLUTION_V2.md` preserve the failure history and prove the bounded resolutions.

## Final all-node posture — owner resolution v3

- Owner resolution: “Good. I shouldn't have said that though. Try to close everything out. Resolve failures.”
- N1: `SUCCESS_CLOSEOUT_RESOLVED_V2`; DEL-07-09 scaffold, DAG-010 current derivative, dependency mirror, and scoped current-authority Reconciliation are restored and validated. Formal pre/post `AUDIT_DECOMP` remains explicitly parked because no applicable in-session runner exists.
- N2: `SUCCESS_OWNER_AMENDED`; committed as `b988d9d0e4a7048ac28a73bbe53ce045c631dff8`. Bend creation is landed; tee/reducer/valve/flange remain row-15 residuals and row 14 remains open.
- N3: `SUCCESS`; committed as `ffbc4834389f3095d22896e126b39085c3e00369`. G4 derivative export remains deferred until an accepted post-merge instruction-surface snapshot exists.
- N4: `SUCCESS_AFTER_OWNER_RESOLUTION_V2`; named LineStop/VerticalSupport emission and the live revision-0.12 architecture pin are validated by the registered profile and fresh V2 review. Vocabulary row 16 is closed/landed.
- Pull request: [#599](https://github.com/sgttomas/chirality/pull/599), branch `codex/piping-sca009-ci-support-20260821`, against `main`.
- Final publication verification remains with CHANGE: commit N1/N4/shared/control in dependency order, run the integrated clean-commit DEC-025 sweep, verify required CI, push the PR branch, and report the exact commit/evidence bindings. No `artifact-proof` label applies.

## Runtime attribution

Every instance in this graph is assigned the current `GPT-5.6` Codex runtime. No model substitution is authorized. Any actual runtime deviation would be a blocking plan variance and must be recorded before further execution.
