# C2 frozen configuration and precloseout return

Supersedes V5 configuration hash and its pending repaired-selfcheck note; the detailed release sequence and provisional inventory remain useful planning evidence.

Frozen candidate configuration:

- `{WORKING_ROOT}/validation/portability_policy.json`: SHA-256 `c3b807f30947f021a2b2258bbe5292f8ca4b5bb3f2aaa2e51c0d8f3ad8d3aabd`, exactly three accepted historical EVIDENCE entries beyond HEAD, older policy unchanged.
- `{RUN_ROOT}/.gitattributes`: SHA-256 `63887a7680ee03f26049b9729fbe78865fefeb85317d61669c346494539f8270`, exactly twelve raw log/patch paths with `-whitespace` only. V5 had eleven; root explicitly extended to actual SOURCE_V2 frozen diff. New path `instances/W7/snapshots/SOURCE_V2/DIFF.patch` SHA-256 `5519dffef63def83b43bc11f90dbd6e063f65621a503169c27fb719015cf164f` has 23 valid unified-context whitespace lines. No additional raw-log offenders discovered at freeze. Final exact inventory and per-file SHA/size/findings: `_run_records/PRECLOSEOUT_V5/RAW_WHITESPACE_INVENTORY_FINAL.json`.

Final independent read-only checks for this stage: full piping pytest PASS1011; validation pytest PASS321; repaired full practitioner pytest PASS350; repaired path anchors PASS (3133 live surfaces); repaired self-check exit0 with INFO14/NOT_APPLICABLE1/REVIEW4/WARN43 and zero active-portability findings. Initial practitioner/path failures remain preserved and were attributed to actual current-tranche evidence classification, then corrected through authorized parent/owner actions. Receipt, G0-G4 corpus checks PASS before further final metadata. No repeated passing product tests were run beyond the authorized checks. No native/desktop builds or full DEC025 ran here.

All twelve whitespace-preserved file hashes and all77 original input hashes reverified unchanged at this freeze. Text diffs remain visible. Untracked whitespace scanner does not honor -whitespace, so ordinary full candidate check after authorized staging remains required; no pass is asserted yet for that gate. Final record/evidence changes may require fresh metadata checks. Source/integration review and clean committed-source sweep remain open.

C2 config is frozen for integrated review. No further configuration writes planned without specific parent direction. Agent1 CHANGE `/root/sync_main`, parent `/root`; no children; inherited runtime, exact model identifier unavailable, native instruction+config assertion. No Git mutation or publication. Ready to receive a later exact source-commit release after all writers and parent fan-in gates close.
