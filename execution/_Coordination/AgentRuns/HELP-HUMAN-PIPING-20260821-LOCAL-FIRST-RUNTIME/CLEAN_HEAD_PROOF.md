# Clean-HEAD proof

- Node commit: `51e7f1e543d8ec46a1b4e677e07a6faae999fcd5`
- Branch: `codex/piping-local-first-runtime-20260821`
- Command: `{PINNED_PYTHON} tools/release/run_evidence_sweep.py --execute`; the exact interpreter and dependency pins are preserved in the cited sweep summary.
- Execution: host-capability run by HELP_HUMAN.
- Result: PASS; all five DEC-025/DEC-093 surfaces passed.
- Summary: `validation/evidence/sweeps/SWEEP_20260822T041718Z_51e7f1e543d8.json`
- Summary SHA-256: `52d5541545d69ce8ffcb5d1281f37a02ab63233890b38d5ee5ea28f2110a9a20`
- Binding: summary records exact commit `51e7f1e543d8ec46a1b4e677e07a6faae999fcd5`, branch, and `working_tree_dirty=false`.
- Surface results: Rust PASS; Python 913 PASS; desktop Vitest 575 PASS; Playwright development 22 PASS; Playwright production-dist 2 PASS; production build PASS.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
