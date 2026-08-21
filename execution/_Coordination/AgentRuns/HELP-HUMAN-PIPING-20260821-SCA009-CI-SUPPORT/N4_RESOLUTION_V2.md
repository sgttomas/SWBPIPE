# N4 owner-resumed resolution V2

## Resolution

`SUCCESS`

The owner withdrew the prior stop-on-failure instruction and directed N4 to
resolve ordinary failures. The reviewed support-family product diff was
restored exactly, the project-declared Python requirements were loaded from the
existing local uv cache without network access or a repository dependency
change, and the stale architecture-basis revision pin was advanced from 0.11
to the accepted live revision 0.12.

## Failure history preserved

The initial run remains recorded in `N4_CHECKS.json`, `N4_REVIEW.md`, and the
historical sections of `N4_RETURN.md`: the profile `python3` lacked pytest, the
first fallback lacked jsonschema, and the architecture validator still pinned
0.11. That was accurately returned as
`FAILED_GATE_WITH_ACCEPTED_IMPLEMENTATION_EVIDENCE` under the then-active stop
rule. No evidence from that attempt is deleted or relabelled as a pass.

## Repairs

1. Restored `core/product_physics/src/lib.rs` to the reviewed mapping:
   `line_stop` → `SupportFamily::LineStop`, `vertical_support` →
   `SupportFamily::VerticalSupport`, explicit DOFs retained, existing
   Anchor/Guide fallbacks retained.
2. Updated only `EXPECTED_DECOMP_REVISION` and its explanatory comment in
   `tools/validation/validate_architecture_basis.py`: SCA-009 Gate 5 accepted
   and closed revision 0.12 and advanced the live decomposition pointer; the
   D-43 reading contract itself did not change.
3. Used `requirements-dev.txt` through offline uv with cached pinned
   dependencies. No network, lockfile, requirements, or host installation was
   used.

## Terminal evidence

- Focused Rust regressions: 3 passed.
- Complete product-physics crate: 101 passed; doc tests clean.
- Direct architecture-basis validator: PASS, 8 members.
- Registered Piping pytest profile: 902 passed in 6.29 seconds.
- `git diff --check`: PASS.
- Fresh independent V2 review: PASS, 100% of frozen two-file diff, no findings.
- Frozen diff: `N4_FROZEN_DIFF_V2.md`.
- Structured checks: `N4_CHECKS_V2.json`.
- Review: `N4_REVIEW_V2.md`.

## Remaining blocker

None within N4. DEC-025 remains intentionally unrun per the owner-resumption
brief and belongs to later integrated clean-commit closeout.

No commit, push, PR, shared receipt/handoff edit, lifecycle act, release act,
or DEC-025 execution was performed.
