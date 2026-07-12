# DEL-12-03 W4 discovery notes

- Pilot: highest-capability GPT-5 discovery pilot, with adversarial SECURITY/privacy review for the F-PIP-1 fence-adjacent PKG-12 scope.
- Frozen evidence: `551f84ef6be656f1603ce0acfa5e3935aa9683c7` in the read-only frozen evidence worktree.
- Scope: `PKG-12/DEL-12-03` only. No lifecycle, dependency, product, or frozen-evidence writes were made.
- Census: 20 rows = 10 REQUIREMENT + 2 EXCLUSION + 6 DECLARED_STATE + 2 REMAINING_WORK.
- Dispositions: 10 ALIGNED, 5 PARTIALLY_IMPLEMENTED, 2 DOCUMENTED_UNIMPLEMENTED, 1 STALE_SETUP_SPECIFICATION, 2 REMAINING_STATE_MISMATCH.
- Confidence: 13 HIGH, 7 MEDIUM, 0 LOW.
- Authority: 7 OWNER, 5 ENGINEERING, 8 NO.
- Verification: `PYTHONDONTWRITEBYTECODE=1 python3 -m pytest -q -p no:cacheprovider projects/chirality-piping/tests/security/test_telemetry_policy.py` returned 15/15 PASS at the frozen SHA.
- SECURITY marker discipline: the exact `NONE_FOUND — sufficiency review deferred, owner-gated` marker is confined to sufficiency-deferral rows REQ-004, EXC-002, and REM-002. Other SECURITY rows record bounded technical evidence without implying security certification.
- Key judgment: the metadata-only helper provides real fail-closed and pre-payload guard evidence, but it cannot establish runtime reachability, route integration, affirmative consent, approval records, or product-config persistence. Those claims are partial or documented-unimplemented rather than vacuously aligned.
- Calibration carry-forward: `Datasheet.md` is stale only because its authority pointer still names SOFTWARE_DECOMP revision 0.7; the other current four-document declarations accurately describe the frozen implemented slice. Pending RF-001/RF-002 human dispositions and the substantive RF-002 deferrals are separately ledgered because `_STATUS.md` contains only the D-41 bootstrap item.
- Addendum-9 check: before and after re-execution, frozen ignored porcelain remained limited to the six run-allow-listed paths; no additional tracked, untracked, or ignored path was introduced.
