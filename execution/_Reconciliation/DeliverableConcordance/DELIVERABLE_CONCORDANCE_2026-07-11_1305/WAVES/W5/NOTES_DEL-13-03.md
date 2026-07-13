# DEL-13-03 W5 discovery notes

- Pilot: deliverable-grained discovery pilot.
- Frozen evidence: `551f84ef6be656f1603ce0acfa5e3935aa9683c7` in the read-only frozen evidence worktree.
- Scope: `PKG-13/DEL-13-03` only. No lifecycle, dependency, DAG, register, product, or frozen-evidence write was made.
- Normative-source alias: unqualified deliverable-document names refer to `execution/PKG-13_Physical Design Knowledge and Constraint Engine/1_Working/DEL-13-03_Constraint validation engine/` at the frozen SHA.
- Census: 16 rows = 8 REQUIREMENT + 2 EXCLUSION + 6 DECLARED_STATE.
- Dispositions: 12 ALIGNED, 1 PARTIALLY_IMPLEMENTED, 3 STALE_SETUP_SPECIFICATION.
- Confidence: 15 HIGH, 1 MEDIUM, 0 LOW.
- Authority: 6 OWNER, 1 ENGINEERING, 9 NO.
- Self-flags for fan-in: REQ-006 (formal runtime/result-envelope integration is partial); DECL-001/002 (explicit revision-0.7 authority pointers are stale under the W1-W4 rev-drift calibration); DECL-004 (the Procedure's in-tree `py_compile` instruction is overtaken by binding addendum 9). SECURITY/boundary rows REQ-005 and EXC-001 also warrant reviewer attention because their ALIGNED dispositions are bounded technical negative-guard judgments, not assurance or legal-clearance claims.
- Focused verification: `PYTHONDONTWRITEBYTECODE=1 PYTHONPYCACHEPREFIX=<external-temp>/pycache python3 -m pytest -q -p no:cacheprovider projects/chirality-piping/tests/test_constraint_validation.py` returned 6/6 PASS at the frozen SHA.
- Pending finding calibration: `PKG13-DEL-13-03-PKG02-001` remains `TECHNICALLY_ADDRESSED_PENDING_HUMAN`, but it is not load-bearing for the existence, determinism, or bounded coverage claims; the row-level validation evidence remains agent-generated and `UNVERIFIED`. No human disposition was inferred.
- Remaining-work census: `_STATUS.md` contains only the D-41 bootstrap item. Per addendum 2 it is reproduced only on DECL-005 and excluded from REM rows, residual counts, gate analysis, and selectability.
- Addendum-9 check: ignored-aware porcelain before and after re-execution contained exactly the six allow-listed incident paths and no seventh path: project `.pytest_cache/`, two reporting `Cargo.lock` files, two `__pycache__/` trees, and `validation/benchmarks/nonlinear/target/`. No cargo or in-tree `py_compile` ran.
- Fan-in correction: REQ-005 and EXC-001 retain their bounded ALIGNED judgments, with `ValidationEvidence` corrected to explicit-reason `NOT_APPLICABLE`; neither row invokes the owner-gated SECURITY sufficiency marker or implies assurance or legal clearance.
- Fences: all dispositions are agent judgments. ALIGNED rows are limited to the stated supplied-record/module/test grain and do not assert full geometric validation, runtime integration, release readiness, professional acceptance, security assurance, or protected-content clearance.
