# Launch brief — D06B-FINAL-VERIFIER-2

## Construction and permissions

- Form: new fresh bounded ephemeral Agent 2 generalist.
- Parent: HELPS_HUMANS `HELPS-HUMANS-R19-INTEGRATION`.
- Objective: independently verify the corrected final pre-receipt R19 bytes.
- Read scope: repository.
- Write scope: none.
- Forbidden: edits, delegation, Git mutation, network/external action,
  credential access, signing/notarization, implementation, release, or
  publication.

V1's `BLOCK` and exact findings remain preserved in
`../D06B-FINAL-VERIFIER/RETURN.md`. Do not reuse V1's incomplete check
coverage. Recompute every item below.

## Corrected frozen basis

- HEAD:
  `2f8d35ceb30da734ca6dff24dcab36dded8c9b35`.
- Owner direction: 151 UTF-8 bytes, SHA-256
  `e8db8ca4d21cbbe4c21e4ee6189524ad2e348476289051a23b0617ce0e8e532f`.
- Corrected packet SHA-256:
  `7f72f244a5cfd896e0f33137eafd32302f052adec75aa20bc56c85c123afee49`.
- Register SHA-256:
  `f03b9ad9b4b7e7156cd6ec28d8a8933fb57ee28c7ecbc874f1405c763f53dd84`.
- Pre-V2-verifier R19 manifest SHA-256, computed from sorted
  `shasum -a 256` lines for every R19 file before this V2 subtree:
  `0e2f69b897587df941ef9f432a63763bda5509c229921f58112bf617e337a570`.
- Protected R18 manifest:
  `e1241d77647ce369ae9d8f1a13263055c27f49e6315b8127d302a0f689638c19`.
- Receipt-72 ledger:
  `7a3a29ffcd70b268ee34daca4df5da64db60c1750873e2e35c8bfe2174c4d31c`.
- Register pre-R19:
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

## Six mandatory checks

1. Recompute the owner-direction bytes/hash and confirm preparation-only
   effect.
2. Recompute the corrected packet hash and confirm every design requirement:
   historical v0.1/`DEC-056` R6-entry basis; exact `DEC-057` artifact; narrow
   O-A standing unsigned deviation; policy-only separately gated O-B;
   trigger-bound O-C with D-06b ruled and next-free rechecked
   `D-XX / NOT_PREPARED` successor; non-binding O-A recommendation; exact
   `Standard claim fence applies (F-PIP-2; DEC-081 claims taxonomy).`;
   decision-specific exclusions; and the complete future ruling mechanism.
3. Confirm exactly one D-06b register row, `AWAITING_RULING`, proposal pointer,
   owner-pending effect, and no ruling pointer.
4. Confirm the pre-R19 register hash/diff establishes that only D-06b's row
   was additionally changed by R19, preserving pre-existing D-56/R18 work.
5. Independently recompute every protected hash, the R18 manifest, and the
   pre-V2-verifier R19 manifest; confirm Receipt-73 is absent.
6. Parse every R19 JSON; scan R19, packet, and register-row addition for
   trailing whitespace; run `git diff --check`; enumerate exact four-target
   containment; and confirm no prohibited effect.

## Return contract

Return terminal `PASS / COMMIT-SAFE` or `BLOCK`, with concise evidence for
every numbered check. Do not stop early on a passing observation. Do not write
a file; HELPS_HUMANS records the return.
