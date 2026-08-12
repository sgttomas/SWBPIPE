# Manager Validation — PR #550 Historical Path-Anchor Repair

Verdict: `PASS_READY_CHANGE`

WORKING_ITEMS accepts the author and fresh verifier fan-in for the bounded
PR #550 portability repair.

## Accepted child returns

- Author return:
  `instances/A2_PR550_PATH_ANCHOR_EXCEPTION_AUTHOR/RETURN.md`, SHA-256
  `741c031758a963fa4f5e9a3ac0984b2923287194199943bc3576b870260556dd`.
- Author status:
  `instances/A2_PR550_PATH_ANCHOR_EXCEPTION_AUTHOR/STATUS.json`, SHA-256
  `356727a184b3d4a6127aa677b5fd86cd236e733fd588aa4b7329b0cd1d2dede0`.
- Independent verifier return:
  `instances/A2_PR550_PATH_ANCHOR_EXCEPTION_VERIFY/RETURN.md`, SHA-256
  `986f41d916d41464d63c91d62955f7954903d9ec4cbdd5b72b9cf61ffa256743`.
- Independent verifier status:
  `instances/A2_PR550_PATH_ANCHOR_EXCEPTION_VERIFY/STATUS.json`, SHA-256
  `7df4d5dfdcdd20cffa0d11bbe6159ed8b889b3cc2500fb3269e8431c6e983b35`.

## Accepted repair

The V1 sealed brief remains exact at SHA-256
`d937f558ee2b38d6d1458790de5efd2d987d9a2ce584ba9cfec2869bebca4a41`
and Git blob `f52c0bf13f4ece6ed2631a0a3c7c941c7a6451d5`. The policy appends exactly one
hash-bound `control_path_exception`; its candidate SHA-256 is
`67a7868278e775cc00de53087029e6290af3682554122d563a3af5dbaacb84b9`
and Git blob is `9a9b2b9efcc4ce68d0903fa2dd8d0a7886c825d9`.

This is a classification-only repair of immutable completed control evidence.
It preserves Amendment V5's frozen V1/tokenized V2 provenance chain and does
not change the DEL-09-04 owner rulings, TM-PIP-037 closure, Receipt 99,
DEC-025 evidence, lifecycle, public-comparison residual, page posture, release,
reliance, or professional-approval effects.

## Validation accepted

- formerly failing live GEN-8 test: 1/1 pass;
- focused path-anchor/policy tests: 51/51 pass;
- full practitioner-harness suite: 349/349 pass;
- CI-equivalent routed `practitioner_harness + validation` suites: 660/660
  pass under the existing Python 3.13.14 / pytest 9.1.1 runtime;
- practitioner-harness self-check: exit 0, no BLOCK;
- final live path-anchor scan: zero findings, zero unacknowledged control,
  zero active unclassified records, zero policy issues, and five acknowledged
  historical controls;
- candidate whitespace, diff, claims-language, receipt, JSON, identity,
  containment, staged, and ignored checks: pass.

An initial routed-suite invocation selected the system developer-tools Python,
which has no pytest module, and stopped before test collection. It created no
repository side effect and was rerun once with the existing verified Python
3.13.14 runtime; the recorded 660/660 result is the terminal routed-suite
evidence.

The locally observed `origin/main` is
`43f89f96b6beaac87a448e281be512818eec8a60`; the pinned base is its ancestor,
the complete Piping tree remains identical at
`36b91d558cde9277112a70cde8fa6b017cb8b07d`, and repair overlap is zero.
CHANGE must refresh and revalidate currentness before export or merge.

No staging, commit, push, merge, fetch, rebase, reset, clean, deletion,
receipt append, lifecycle action, release act, or professional-approval effect
occurred in this WORKING_ITEMS tranche.
