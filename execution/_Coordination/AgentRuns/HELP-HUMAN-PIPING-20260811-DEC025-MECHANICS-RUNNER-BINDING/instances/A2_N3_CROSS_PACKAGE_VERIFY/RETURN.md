# Return — A2-N3 Fresh Cross-package Verifier

Verdict: `COMMIT_SAFE`

## Contract and identities

- Fresh, non-delegating, non-repairing Agent 2 under
  `N3-WORKING-ITEMS-CROSS-PACKAGE-VERIFY`.
- Sealed brief SHA-256:
  `fe1577559d1653a749e7a3eef0e580b93a250a39acdfe9519c13fc4698377fb3`.
- Branch / HEAD:
  `codex/piping-dec025-case-runner-binding-20260811` /
  `f1e311fb7ab1c2a0800b1d32c59445368428dee9`.
- Adopted candidate SHA-256:
  `c8d2b63d6a1649102e2b233542975cc8e8062e582914900a2ec75c5acc961626`.
- N1 handoff SHA-256:
  `44138d6aa89f4c6420912ef894efa2ef280771cf2c3415e0e9399fa097fb10de`.
- N2 final handoff SHA-256:
  `69bc56ab62fb7213286fe415a670bbe92049e143700430d510e190ee331e4562`.
- Mechanics source SHA-256 / Git blob:
  `18e7c1865dbd5fd07891562b98ea54c794b0227d7bf056c95e567c6e6de3c2b5` /
  `a0d8b4a269a5dc0f9d2f868a0a0cec97bacb0197`.
- Runner source SHA-256 / Git blob:
  `4a45a0889391046fe6ab887409c791a2148bc30205478138a142af07fd4f1e6f` /
  `455b9e9dee1a6a8154f65a1b5218dd7e09b6444f`.
- C-B SHA-256 / Git blob:
  `1829a2a6a608dbbdd479248133f2c2a05406c0eabe7a03be57b998735fbe78b6` /
  `468d6dd4a85525b64989ff520a5f4ff10e7c6e6f`.
- Frozen original output SHA-256:
  `e41f8545cf6fc0603cb41a0bdc08d90726f59ce57e13ad6809036589db3ded8f`.

## Independent external verification

The retained external root is
`/private/tmp/n3-cross-package-verify-20260812.OnohUW`. The complete Piping
tree was copied byte-identically to `shadow/chirality-piping`; the exact prior
N2 verifier Cargo cache was copied into the fresh `cargo-home`. Cargo home,
target, temporary files, logs, binary, and generated runner output all stayed
inside this external root. Cargo ran offline and `HOME` was not repurposed.

- Mechanics `cargo fmt --check`: PASS.
- Runner `cargo fmt --check`: PASS.
- Mechanics complete tests: PASS, 41/41; doc tests zero failures.
- Runner complete tests: PASS, library 38/38, preview 1/1, binary 15/15;
  doc tests zero failures.
- Exact registered mechanics whole-suite run with explicit local/private
  intent: PASS, exit 0. Output size 1,561,214 bytes; SHA-256
  `12901f73ffdf7d828c6e179f25e672aa152006f183c5331e2c64b80354d1962d`;
  stdout and output file were byte-identical.

Independent JSON analysis proved 25 requested / 25 executed-and-matched,
zero mismatched, zero blocked, zero diagnostics; 206/206 populated finite
comparisons and 206/206 within the suite-owned recorded basis. The new slice
is exactly 14 cases / 115 values with exact case order, unique case IDs, and
unique value names per case. Every complete serialized object for the frozen
original 11 cases / 91 values is equal to the corresponding object in the
accepted frozen output.

## Fail-closed and preservation checks

Direct tests passed for all six required negative behaviors: unknown fixture
ID, incomplete suite observations, duplicate observation names, non-finite
observation, suite execution failure, and expected/observed name mismatch.

The full runner test also passed the unchanged stress/nonlinear assertion:
stress has zero mismatches with its exact three historical fail-closed cases,
and nonlinear is 5/5 matched with zero mismatch/block. Stress source SHA-256
`ad7239a073f5ca6c161ee3f63642d487414efed6e3176ae86a2af7979b73210c`
and nonlinear source SHA-256
`ff3f318224fa2c55392bfa17569182768453b3157cf35fc20d763f54bf02fd12`
both match HEAD exactly.

C-B is byte-identical to HEAD and the accepted hash/blob. The candidate diff
does not modify C-B or any nonlinear path. No C-B policy-file path or DEC-046
policy token was added to the mechanics/runner implementation; the only
`C-B` text in the source diff is the pre-existing non-promotion claim posture.
Thus C-B remains nonlinear-only and unchanged.

## Containment and terminal cleanliness

`git diff --check` passed. The production delta from the pinned base is
exactly the mechanics `src/lib.rs` and runner `src/benchmark_binding.rs`.
Final accepted source hashes remain exact and relevant source/shadow bytes
remain identical. Repository staged inventory is zero and ignored inventory
is zero. No production repair, network, provisioning, Git mutation, cleanup,
or deletion occurred. No blocker, waiver, or human ruling is needed.

This `COMMIT_SAFE` verdict authorizes N3 fan-in only. It does not itself
authorize N4 state/evidence changes, staging, commit, DEC-025, publication,
push, PR, merge, cleanup, lifecycle, release, or reliance effects.
