# Final handoff state — owner-resolved all-node fan-in

- State: `ALL_NODES_ACCEPTED_CLOSEOUT_COMPLETE`.
- Basis: `b1876a5e0f0083e10c0c18255cd92ed0079b63a2`; SOFTWARE_DECOMP revision 0.12; accepted SCA-009.
- Owner resolution: exact text and effect are in `OWNER_DIRECTIONS.md`; closeout history is in `CLOSEOUT_RESOLUTION_V2.md`.
- Authoritative/derivative boundary: accepted decomposition/SCA-009 remain upstream truth. DAG-010, the DEL-07-09 dependency/coverage mirrors, scoped Reconciliation, G4 manifest, and AgentRuns package are derivative evidence.

## Accepted nodes

- N1: `SUCCESS_CLOSEOUT_RESOLVED_V2`; committed as `cef84d954ac2b2c6faa19e4a3f04e43b94c86200`. DAG-010 is the current derivative successor to live-basis DAG-009, and N1's exact failed-then-resolved closeout history is preserved.
- N2: `SUCCESS_OWNER_AMENDED`; committed/pushed as `b988d9d0e4a7048ac28a73bbe53ce045c631dff8`.
- N3: `SUCCESS`; committed/pushed as `ffbc4834389f3095d22896e126b39085c3e00369`.
- N4: `SUCCESS_AFTER_OWNER_RESOLUTION_V2`; committed as `a166e30b9f58b8470d96e33d95c12e104b8f01bc`. Current V2 checks/review supersede the initial failed-gate disposition without deleting it.
- Terminal integrated review: `PASS`, with no actionable findings over 100% of the seven-file N2/N4 behavior diff. Exact reviewed hashes and invariants are frozen in `INTEGRATED_REVIEW.md`.

## Coverage and deferrals

- Row 14: `OPEN / NOT_CLOSED / NOT_LANDED` — expansion-joint creation remains.
- Row 15: `PARTIAL / NOT_CLOSED / PARTIALLY_LANDED` — bend is landed; tee, reducer, valve, and flange remain.
- Row 16: `CLOSED / CLOSED / LANDED` — points to N4 V2 evidence in DEL-04-03 territory.
- Formal pre/post `AUDIT_DECOMP` remains explicitly parked because no clearly applicable in-session runner exists.
- The N3 G4 derivative export remains deferred until an accepted post-merge instruction-surface snapshot exists.
- Parked owner holds remain unchanged: PKG-02 runtime/transport/permission choices, `.opsproj` policy, PDU-011/PDU-047, and `MAINTAINER_REVIEWED` promotion.

## Git closeout

- Branch: `codex/piping-sca009-ci-support-20260821`; PR [#599](https://github.com/sgttomas/chirality/pull/599) against `main`.
- Ordered commits after the already-pushed N2/N3 pair: N1 `cef84d954ac2b2c6faa19e4a3f04e43b94c86200`; N4 `a166e30b9f58b8470d96e33d95c12e104b8f01bc`; shared/control pre-sweep `f7148f9b97e37babee19e18f5588a4122705ae5b`; harness baseline repair `07909d7193de22b9edeacc686175d28500022b52`; failed-sweep evidence `f6064019f80298d1d3f1e475aece49fe28f478bc`; pre-sync proof `c6552d5bf2eadf08504b97a53b9a1e62ed29e7df`; owner-authorized sync `5b98fc6b79f9f28f873dbdc8dd51c13953b55b72`; post-sync proof `fb5599c93920206c31913f18e79daebe0e5b7e0c`; G4 CI repair `13891f8c0e9019f26a3de73d9395734c61e05c8a`.
- Required closeout gates: registered Piping pytest 902 PASS; practitioner harness 350 PASS; repository self-check exit 0; receipt validator PASS; G4 policy tests 47 PASS; G4 corpus 40 manifests PASS.
- Pre-sync DEC-025: PASS at `f6064019f80298d1d3f1e475aece49fe28f478bc`; clean tree; summary `validation/evidence/sweeps/SWEEP_20260821T053243Z_f6064019f802.json`; SHA-256 `d4f726255e8d57f9aae851748ad863e1a5c0d094f5ae07ca62dcdc6540cf559c`.
- Upstream sync: after `origin/main` advanced through PR #600, the owner authorized a non-rewriting merge. Merge commit `5b98fc6b79f9f28f873dbdc8dd51c13953b55b72` has parents `c6552d5bf2eadf08504b97a53b9a1e62ed29e7df` and `7fd085b54ae0cfb9136766f395a560020c8be65b`; its first-parent delta is exactly `README.md`, with no tranche-surface conflict or mutation.
- Post-sync DEC-025: PASS at clean merge commit `5b98fc6b79f9f28f873dbdc8dd51c13953b55b72`; summary `validation/evidence/sweeps/SWEEP_20260821T054123Z_5b98fc6b79f9.json`; SHA-256 `513110e5c43007fb3cbdc753c02162c0173a1a15e395e60a42b4e3d8e372a2c2`; all five surfaces passed, including host Playwright dev 22 and dist 2.
- PR CI repair: governance run `32451967800` failed its G4 diff gate because the accepted `tools/practitioner_harness/test_live_baseline.py` change was not declared in the tranche manifest. Commit `13891f8c0e9019f26a3de73d9395734c61e05c8a` adds that exact path and its owner-authorized closeout basis to `PIPING-CI-SLOW-OBJECT-HARDENING-20260821.yaml`; local G4 policy tests (47), corpus validation (40 manifests), and `origin/main..HEAD` diff validation all pass. Failure history is preserved here; the PR retry remains the publication gate.
- Failed attempt history remains in `validation/evidence/sweeps/SWEEP_20260821T053152Z_07909d7193de.json`: Rust passed and Python collection failed because the bare interpreter lacked `jsonschema`; the offline uv requirements retry resolved it.
- `CANDIDATE_FILES.txt` records the final post-N2/N3 closeout membership. The final proof commit contains this updated handoff and Receipt-122 and is intentionally not self-referential.
- Next owner: CHANGE waits for the repaired PR #599 checks to reach terminal success, then human review. No `artifact-proof` label applies. No merge, lifecycle, release, issuance, publication, or professional-reliance effect is created.
