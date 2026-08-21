# N4 return — support-family emission

## Verdict

`SUCCESS_AFTER_OWNER_RESOLUTION_V2`

The bounded implementation and the coupled revision-pin repair now pass every
owner-requested N4 gate: focused regressions, the complete product-physics
crate, the registered Piping Python profile, formatting, diff check, direct
architecture validation, and a fresh 100%-diff review. See
`N4_RESOLUTION_V2.md`, `N4_CHECKS_V2.json`, `N4_FROZEN_DIFF_V2.md`, and
`N4_REVIEW_V2.md` for the current terminal evidence.

## Initial failed-gate history (preserved)

The initial run returned
`FAILED_GATE_WITH_ACCEPTED_IMPLEMENTATION_EVIDENCE`: the profile `python3`
lacked pytest, the first pytest-capable fallback lacked jsonschema, and the
architecture validator still pinned revision 0.11. That evidence remains in
`N4_CHECKS.json`, `N4_REVIEW.md`, and the historical validation/blocker
sections below. Owner direction later withdrew the stop rule and authorized
ordinary repair; V2 resolves all three conditions without deleting the
initial record.

## Accepted basis and scope

- Package / deliverable: `PKG-04 / DEL-04-03`
- Basis: `b1876a5e0f0083e10c0c18255cd92ed0079b63a2`
- Owner-directed coverage target: SCA-009 `Vocabulary_Annex.md` NORMATIVE-NOW row 16
- Initial product write: `core/product_physics/src/lib.rs` only
- Owner-resumed coupled repair: `tools/validation/validate_architecture_basis.py`
- Frozen diff: `N4_FROZEN_DIFF.md`
- Review: `N4_REVIEW.md`
- Checks: `N4_CHECKS.json`
- Current V2 resolution: `N4_RESOLUTION_V2.md`
- Current frozen diff and review: `N4_FROZEN_DIFF_V2.md`, `N4_REVIEW_V2.md`
- Current checks: `N4_CHECKS_V2.json`

## Changed files

- `projects/chirality-piping/core/product_physics/src/lib.rs`
  - routes canonical `line_stop` to solver `SupportFamily::LineStop`;
  - routes canonical `vertical_support` to solver
    `SupportFamily::VerticalSupport`;
  - preserves entered restraint DOFs for solver validation;
  - preserves six-DOF Anchor and partial-restraint Guide fallback behavior;
  - adds focused LineStop, VerticalSupport, and Guide regression tests.
- `projects/chirality-piping/tools/validation/validate_architecture_basis.py`
  - advances the accepted decomposition revision pin from 0.11 to 0.12;
  - records the SCA-009 Gate-5 authority without changing the D-43 reading
    contract.
- N4-local evidence files only: `N4_FROZEN_DIFF.md`, `N4_REVIEW.md`,
  `N4_CHECKS.json`, `N4_RETURN.md`, and `N4_STATUS.json`.

## Initial validation summary (historical)

- Focused regression tests: PASS, 3/3.
- Complete `product_physics` crate: PASS, 101/101 unit tests and 0 doc-test failures.
- Governance harness self-check: PASS, exit 0.
- `git diff --check`: PASS.
- Fresh independent review: PASS, 100% of the frozen diff, no actionable findings.
- Registered Piping Python profile: FAILED GATE; 812 passed, 17 skipped,
  31 failed, and 5 collection errors under the only pytest-capable fallback.
  Failures are dominated by missing `jsonschema`; an independent architecture
  policy test expects obsolete revision 0.11 instead of the required 0.12.
- Evidence sweep: not run after the registered profile failure, as required by
  the iteration's stop-on-failed-check rule.

## Coverage closeout evidence

SCA-009 NORMATIVE-NOW row 16 can point to the landed `product_physics` helper
and the three named regressions as implementation evidence, plus
`N4_REVIEW.md` as the independent review gate. This node does not write the
shared coverage ledger; HELP_HUMAN owns that fan-in surface.

## Initial blockers and rerun requirement (resolved by V2)

- Blocker: a project Python runtime with `pytest`, `pytest-xdist`, and the
  declared `jsonschema>=4,<5` dependency is not available through the profile's
  `python3` command in this session.
- Independent stale test: `tests/test_architecture_basis_validation.py` still
  expects SOFTWARE_DECOMP revision 0.11 while this iteration requires 0.12.
- Rerun: after the runtime dependency and stale policy assertion are resolved,
  run `python3 -m pytest -q tests -n auto --dist loadscope`, then the registered
  evidence sweep from the integrated clean commit.

No commit, push, PR, shared coverage/status write, lifecycle claim, or release
claim was made.

## Current V2 validation summary

- Focused regression tests: PASS, 3/3.
- Complete `product_physics` crate: PASS, 101/101; doc tests clean.
- Direct architecture-basis validation: PASS, 8 members at revision 0.12.
- Registered Piping pytest profile in the offline declared environment: PASS,
  902/902 in 6.29 seconds.
- `git diff --check`: PASS.
- Fresh independent V2 review: PASS, 100% of the two-file frozen diff, no
  actionable findings.
- Remaining N4 blocker: none. DEC-025 intentionally remains for later
  integrated clean-commit closeout per owner direction.
