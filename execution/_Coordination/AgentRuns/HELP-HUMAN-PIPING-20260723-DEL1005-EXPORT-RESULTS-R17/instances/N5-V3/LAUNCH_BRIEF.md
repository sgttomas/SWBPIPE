---
doc_id: R17-DEL1005-N5-V3-LAUNCH-BRIEF
doc_kind: coordination.agent_brief
status: dispatched
created: 2026-07-23
run_id: HELP-HUMAN-PIPING-20260723-DEL1005-EXPORT-RESULTS-R17
instance_id: N5-V3
parent_instance_id: W-PKG10
role: fresh_independent_verifier
package_id: PKG-10
deliverable_id: DEL-10-05
frozen_git_basis: 1f2ecc1d06375c01a409041b8380e4d65b2a9f9a
candidate: CB-2026-07-23-DEL-10-05-EXPORT-RESULTS-001-v3
---

# Fresh N5 verification after sealed v3 remediation

## Objective

Independently verify the entire adopted v2 implementation plus the owner-
adopted sealed v3 repair. Return exactly `COMMIT-SAFE` or `BLOCK`. This is a
fresh Agent 2 instance: do not delegate and do not perform W3.

## Required inputs

Read:

- the adopted candidate and `AMENDMENT_V2.md`;
- `OWNER_ADOPTION.md`;
- `AMENDMENT_V3_PROPOSAL.md` and `OWNER_ADOPTION_V3.md`;
- exact `WRITE_MATRIX.csv`;
- prior N5 `RETURN.md`;
- N4 `REMEDIATION_V3_BRIEF.md`, `REMEDIATION_V3_EVIDENCE.md`, and
  `RETURN_V3.md`;
- the complete dirty diff from frozen basis
  `1f2ecc1d06375c01a409041b8380e4d65b2a9f9a`;
- both R17 sweep artifacts and normalized registered evidence.

Repository authority and live files govern on disagreement.

## Permission boundary

Read the repository and run non-mutating verification commands only. Do not
edit, format, stage, commit, reset, checkout, merge, push, regenerate
witnesses, invoke DEC-025, or perform W3. Ordinary ignored compiler/test caches
are permitted consequences, but no tracked or governed output may change.

## Required verification

Recheck the original N5 brief in full, then independently establish:

1. the v3 delta is exactly the two owner-authorized files and the full R17
   dirty set remains contained;
2. absent `export_results` remains missing/exit `1`;
3. a present raw member is independently deserialized in the export-results
   handler;
4. missing required field, malformed field type, and explicit null each return
   exit `1`, `HEADLESS_RUNNER_EXPORT_RESULTS_PAYLOAD_INVALID`, the existing
   `REPORT-PACKAGE-WIRE-INCOMPLETE` prefix, no report-package payload, and no
   output file;
5. valid success, binding, producer block, aggregate redaction, package bytes,
   runner verb, benchmark/regression, shared wire, desktop persistence,
   schema/member, witness, and native behavior remain unchanged;
6. focused tests and the v3 registered union are accurately recorded;
7. the initial v2 sweep and exactly one replacement v3 sweep exist, the
   replacement artifact is
   `SWEEP_20260723T100430Z_1f2ecc1d0637-dirty.json`, its SHA-256 is
   `ee50a166142468752b3e84631dc3568e1116918ab79610dacee22cf40026f3d1`,
   its status is pass, and no implementation/witness edit followed it;
8. the unchanged desktop Rust 74/75 stale-notice residual remains accurately
   bounded and is not treated as waived.

Inspect implementation quality and security directly. Run focused
non-mutating checks as needed, but do not rerun a registered evidence sweep.

## Return contract

Return:

- `COMMIT-SAFE` or `BLOCK`;
- exact branch/basis and changed-path containment;
- prioritized findings with exact file/line evidence;
- independent reproductions/checks;
- both sweep identities/count and post-replacement mutation result;
- treatment of the frozen desktop Rust residual;
- explicit W3 release/hold statement.

The verdict is implementation verification only, not merge, lifecycle, stage,
release, issuance, or professional acceptance.
