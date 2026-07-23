---
doc_id: R17-DEL1005-N4-RETURN
doc_kind: coordination.agent_return
status: success_ready_for_fresh_n5
created: 2026-07-23
run_id: HELP-HUMAN-PIPING-20260723-DEL1005-EXPORT-RESULTS-R17
instance_id: N4
frozen_git_basis: 1f2ecc1d06375c01a409041b8380e4d65b2a9f9a
candidate: CB-2026-07-23-DEL-10-05-EXPORT-RESULTS-001-v2
---

# N4 terminal return

N4 completed the adopted v2 implementation within the exact
`WRITE_MATRIX.csv` boundary.

- Focused gates passed: report-package 16 tests, runner 46 tests, Python
  contract 15 tests, and both Rust formatting checks.
- The final registered pre-sweep union passed: desktop 516 tests and production
  build, piping 546 tests, practitioner harness 311 tests, and repository
  self-check.
- Native runner proof passed at both small and 3,229,152-byte package sizes
  with six exact ZIP members and constant one-decision/one-finding aggregate
  redaction cardinality.
- Packaged macOS proof passed default-off blocking, explicit intent, real
  picker cancel, new save, and same-path replacement for a 3,518,303-byte
  package with no temporary-file residue.
- Frozen collateral, deterministic witnesses, static validations, JSON
  parsing, changed-path containment, and unstaged-diff checks passed.
- Exactly one authorized DEC-025 sweep ran and passed:
  `validation/evidence/sweeps/SWEEP_20260723T093917Z_1f2ecc1d0637-dirty.json`
  (`sha256:948ef9ee35f620cadc70685d9cf86efebc95e338ee37ef19644621f7712a8879`).
- No product, test, schema, Cargo, or witness file changed after the sweep.
- The known desktop Tauri Rust 74/75 stale-notice assertion is unchanged from
  the frozen basis, is outside this tranche, and is not represented as waived.

N4 does not claim independent `COMMIT-SAFE`. The implementation is sealed
pending fresh N5 verification. W3 remains blocked.
