# Launch brief — D06B-FINAL-VERIFIER

## Construction and permissions

- Form: fresh bounded ephemeral Agent 2 generalist.
- Parent: HELPS_HUMANS `HELPS-HUMANS-R19-INTEGRATION`.
- Objective: independently verify the final pre-receipt R19 preparation.
- Read scope: repository, with attention to the cited project sources.
- Write scope: none.
- Forbidden: edits, delegation, Git mutation, network/external action,
  credential access, signing/notarization, implementation, release, or
  publication.

## Frozen basis

- HEAD:
  `2f8d35ceb30da734ca6dff24dcab36dded8c9b35`.
- Owner direction: exact 151 UTF-8 bytes, SHA-256
  `e8db8ca4d21cbbe4c21e4ee6189524ad2e348476289051a23b0617ce0e8e532f`.
- Final packet SHA-256:
  `a602b798926121a9fff6bec500c6e4e05ee2462b2676b1d35e4820006f4bfc7d`.
- Final register SHA-256:
  `f03b9ad9b4b7e7156cd6ec28d8a8933fb57ee28c7ecbc874f1405c763f53dd84`.
- Pre-verifier R19 manager-record manifest SHA-256, computed from sorted
  `shasum -a 256` lines for every R19 file before this verifier subtree:
  `360a15d91b6162bda5a65ab204e3fb340351b1932950fb293b7292e33603764f`.
- Protected R18 manifest SHA-256:
  `e1241d77647ce369ae9d8f1a13263055c27f49e6315b8127d302a0f689638c19`.
- Receipt-72 ledger SHA-256:
  `7a3a29ffcd70b268ee34daca4df5da64db60c1750873e2e35c8bfe2174c4d31c`.
- Register pre-R19 SHA-256:
  `531e3379a2d46f1c73d9e7487bf60eac68d05d3dc97005428e1a0a4beb52a03e`.
- D-06:
  `17bfaa1de4c90cbb8d031eb375826c915446240c2f10e32d0aec9a9ba1524956`.
- D-21:
  `029b25fd16b3f680eb49d55b1de108ffe44531fd78df0dc0270385436103f0a4`.
- `SOFTWARE_DECOMP.md`:
  `6536db3aa86ad0eae22ede93ceedb6e52f0ce33264b135812593b14c92045349`.
- Current PRD:
  `9c3bccd8d2eb8e68c10e05d50bdd29619892196c98069a2587ba3a1ff4880793`.
- Archived PRD v0.1:
  `d165efc02bff002e629d5734490bb51014d0f7336cdf38b7b52f41f9c89d509b`.
- DEL-10-04 status:
  `097ea5ecc389cdadfb8ada41da6c0066cc4d9ce447430150171ce476f1efca04`.
- `BUILD_AND_RELEASE.md`:
  `b0ea8de3a3338d8f0c767f898fbf2ab00d146eb6e9cf27c221430ea354a46066`.

## Required checks

1. Confirm the owner-direction bytes/hash and exact preparation-only effect.
2. Confirm the packet hash and source-grounded corrected design:
   - archived v0.1 §22.6 “Signed releases” is a `DEC-056` R6-entry residual,
     not current PRD R5;
   - O-A closes only the exact `DEC-057` macOS Apple-Silicon `.app`-zip
     signing residual as a standing unsigned deviation;
   - O-B is policy-only, with enrollment, spend, credentials, network/notary,
     implementation, publication, and release separately gated;
   - O-C makes D-06b `RULED` while creating one next-free rechecked
     trigger-bound `D-XX / NOT_PREPARED` successor;
   - O-A is non-bindingly recommended;
   - GF-TOKEN and decision-specific exclusions are present; and
   - the future mechanism requires a ruling record, rechecked `DEC-XXX`,
     D-06b transition, optional O-C successor, and separate later bounded
     documentation/status candidate.
3. Confirm exactly one D-06b register row, `AWAITING_RULING`, packet pointer
   present, owner-pending effect, and no ruling pointer.
4. Confirm the pre-R19 register hash and diff establish that only D-06b's row
   was additionally changed by R19; preserve the pre-existing D-56/R18 work.
5. Recompute all protected hashes and the pre-verifier manager-record
   manifest; confirm Receipt-73 is absent.
6. Confirm valid R19 JSON, no trailing whitespace, `git diff --check`, exact
   four-target containment, and no prohibited effect.

## Return contract

Return terminal `PASS / COMMIT-SAFE` or `BLOCK`, with concise evidence for
each numbered check and exact blocker/correction scope if blocked. Do not
write a file; HELPS_HUMANS records the return.
