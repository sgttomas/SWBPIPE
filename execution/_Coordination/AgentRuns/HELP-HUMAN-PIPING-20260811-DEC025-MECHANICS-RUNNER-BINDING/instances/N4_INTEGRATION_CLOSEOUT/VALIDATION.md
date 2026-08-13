# N4 Validation

Verdict: `PASS_READY_CHANGE`

- Branch/HEAD exact: `codex/piping-dec025-case-runner-binding-20260811` /
  `f1e311fb7ab1c2a0800b1d32c59445368428dee9`.
- N1 source SHA-256 exact:
  `18e7c1865dbd5fd07891562b98ea54c794b0227d7bf056c95e567c6e6de3c2b5`.
- N2 source SHA-256 exact:
  `4a45a0889391046fe6ab887409c791a2148bc30205478138a142af07fd4f1e6f`.
- N3 child return SHA-256 exact:
  `e7dfee006d050636e38e0e5def29f1acdda487902ea5f3928dd6925fde6de19b`.
- C-B exact:
  `1829a2a6a608dbbdd479248133f2c2a05406c0eabe7a03be57b998735fbe78b6`.
- Bounded acceptance evidence: 25/25, 206/206, new 14/115, original 11/91,
  six fail-closed behaviors, stress unchanged, nonlinear 5/5: PASS through
  fresh N3 verification.
- Lifecycle/residual/fence review: PASS.
- `git diff --check`: PASS.
- JSON parse: PASS.
- Exact N4 write containment: PASS.
- Staged paths: zero. Ignored paths: zero.
- Runtime summary: PASS, 24 events / 11 complete sessions.
- DEC-025 derivative evidence: not yet produced.
