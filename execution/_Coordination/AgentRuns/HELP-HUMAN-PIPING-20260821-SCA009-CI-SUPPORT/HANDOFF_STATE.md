# Final handoff state — owner-resolved all-node fan-in

- State: `ALL_NODES_ACCEPTED_PENDING_CHANGE_VERIFICATION`.
- Basis: `b1876a5e0f0083e10c0c18255cd92ed0079b63a2`; SOFTWARE_DECOMP revision 0.12; accepted SCA-009.
- Owner resolution: exact text and effect are in `OWNER_DIRECTIONS.md`; closeout history is in `CLOSEOUT_RESOLUTION_V2.md`.
- Authoritative/derivative boundary: accepted decomposition/SCA-009 remain upstream truth. DAG-010, the DEL-07-09 dependency/coverage mirrors, scoped Reconciliation, G4 manifest, and AgentRuns package are derivative evidence.

## Accepted nodes

- N1: `SUCCESS_CLOSEOUT_RESOLVED_V2`; DAG-010 is the current derivative successor to live-basis DAG-009, and N1's exact failed-then-resolved closeout history is preserved.
- N2: `SUCCESS_OWNER_AMENDED`; committed/pushed as `b988d9d0e4a7048ac28a73bbe53ce045c631dff8`.
- N3: `SUCCESS`; committed/pushed as `ffbc4834389f3095d22896e126b39085c3e00369`.
- N4: `SUCCESS_AFTER_OWNER_RESOLUTION_V2`; current V2 checks/review supersede the initial failed-gate disposition without deleting it.
- Terminal integrated review: `PASS`, with no actionable findings over 100% of the seven-file N2/N4 behavior diff. Exact reviewed hashes and invariants are frozen in `INTEGRATED_REVIEW.md`; DEC-025 remains pending the integrated clean commit.

## Coverage and deferrals

- Row 14: `OPEN / NOT_CLOSED / NOT_LANDED` — expansion-joint creation remains.
- Row 15: `PARTIAL / NOT_CLOSED / PARTIALLY_LANDED` — bend is landed; tee, reducer, valve, and flange remain.
- Row 16: `CLOSED / CLOSED / LANDED` — points to N4 V2 evidence in DEL-04-03 territory.
- Formal pre/post `AUDIT_DECOMP` remains explicitly parked because no clearly applicable in-session runner exists.
- The N3 G4 derivative export remains deferred until an accepted post-merge instruction-surface snapshot exists.
- Parked owner holds remain unchanged: PKG-02 runtime/transport/permission choices, `.opsproj` policy, PDU-011/PDU-047, and `MAINTAINER_REVIEWED` promotion.

## Git closeout

- Branch: `codex/piping-sca009-ci-support-20260821`; PR [#599](https://github.com/sgttomas/chirality/pull/599) against `main`.
- Already committed/pushed: N2 then N3 at the exact SHAs above.
- Current uncommitted candidate: `CANDIDATE_FILES.txt`; it contains N1, N4, the integrated review record, and final shared/control/receipt paths only.
- Next owner: CHANGE. Required order: stage exact candidate; pass staged whitespace/containment; commit N1 then N4 then shared/control as appropriate; run integrated clean-commit DEC-025; verify PR checks and upstream ancestry; push; record exact commit and sweep bindings.
- No broad project test or DEC-025 sweep was run in final fan-in. No `artifact-proof` label applies. No merge, lifecycle, release, issuance, publication, or professional-reliance effect is created.
