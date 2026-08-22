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

## Authorized sync and integrated-head proof

- Owner authorization: pure, non-rewriting merge of `origin/main@33e871fc38d8ef4bb51f7c25cdc6ca2e8dcb69e0` into the tranche branch, followed by integration checks, DEC-025 rerun, publication, and Receipt 125 closeout.
- Sync merge: `f4ebdab86208c5f0c6edd44de55794de32d06997`; parents `9e9b001a88a116289708fb795c84b97e2c73b952` and `33e871fc38d8ef4bb51f7c25cdc6ca2e8dcb69e0`.
- Containment: no conflicts; the incoming first-parent delta is 39 root `execution/**` paths; the 13-path N1 product/test hash inventory is unchanged.
- Post-sync portability repair: `a477bd41b01f1bb90d30e62c5611754e0838d8ad`; replaced one machine-absolute proof command with `{PINNED_PYTHON}` after the practitioner harness identified it.
- Command: `{PINNED_PYTHON} tools/release/run_evidence_sweep.py --execute`; the exact interpreter and dependency pins are preserved in the cited sweep summary.
- Result: PASS on clean integrated head; all five DEC-025/DEC-093 surfaces passed.
- Summary: `validation/evidence/sweeps/SWEEP_20260822T043242Z_a477bd41b01f.json`
- Summary SHA-256: `f492644f443455e72dce99cb6c8b4165d3167e405cd93f5d8cfa000b295f696d`
- Binding: summary records exact commit `a477bd41b01f1bb90d30e62c5611754e0838d8ad`, branch, and `working_tree_dirty=false`.
- Surface results: Rust PASS; Python 913 PASS; desktop Vitest 575 PASS; Playwright development 22 PASS; Playwright production-dist 2 PASS; production build PASS.
