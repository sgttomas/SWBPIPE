# P1 bounded implementation return

RUN_STATUS: SUCCESS (bounded implementation; fresh review and shared final gates remain).
ControlSurface: MERGED. TaskProfile: NONE. TaskSkill: software-bounded-implementation v1.
Model: unknown; Agent2 role/nondelegation instruction/config asserted. No children.

R01/R06/R07 implementation uses the selected nonlinear displacement/reaction state before ordinary node rows, maxima, end/station force and stress recovery, constant-effort movement checks, and primitive operands. Nonconverged/failed nonlinear loops retain diagnostics but cannot publish the preliminary linear solve as mechanics success. Linear-only scrutiny remains observational.

A compatibility-preserving nonlinear entry point accepts canonical `(global_dof, stiffness)` linear ground springs. Existing function and DTO remain unchanged. Every iteration assembles those springs. Necessary-only stability guard counts unique rigid and positive-spring ground DOFs; duplicates do not increase rank. Six springs and five rigid plus one spring solve through the assembled system. This is not a rank sufficiency test or a guarantee to detect every mechanism under the existing pivot policy.

Support force resultants now use translations only. Linear spring actions are `-k*u` in the selected state. Rigid/contact actions come from selected system reactions. Per-case internal signed support vectors retain unrounded Newton components. No new public row, field or kind is introduced.

Finite guards cover assembled matrices/force, selected displacement/reaction, recovered forces/stress components, result publication and maxima. Recovery errors become blocking. Nonlinear computed reactions/work overflow reject via existing InvalidInput; nonfinite optional delta and sparse scrutiny observations become unavailable. NaN cannot disappear in max folds. Large finite values no longer overflow solely through display-rounding multiplication.

## Verification

- P9 independent EXPECTED manifest verified before comparison; preserved values in EXPECTED_REFERENCE.json. Pre-P1 current baseline: 0/10 P9 tests passed. Post-P1: 2/10 passed (inactive gap plus spring and selected closed axial stop, both modes). Eight remaining tests reach P5-owned recovery/algebra/link/UDL failures; force-only portions of two of those now pass before later station assertions fail. Not claimed as full P9 acceptance.
- Product targeted suite: 129 passed, zero failed, one explicitly filtered stale generated-fixture comparison. Full suite evidence:129 passed/1 failed. The generated-fixture failure is unchanged checked-in browser UY .332485 versus selected native UY0; it remains enabled and must pass after authorized final fixture regeneration. No global product PASS claim.
- Nonlinear integration complete crate suite:22 passed, zero failed; doc tests pass. Includes independent representability controls for prescribed reaction overflow, work overflow, delta overflow and canceled nonfinite sparse residual arithmetic.
-20 executions of unchanged original I1 driver and original witness inputs (10 fixtures, both modes), with input/source hashes. Original350N ->350N support force; pure100Nm ->0N force; spring+inactive gap -> .187968mm display and187.967971N spring/162.032029N root. Independent frozen pre-round oracle remains .18796797141911276mm and187.96797141911276N. Six-spring/five-plus-spring cases solve; original finite1e308 overflow cases return nonsolved and no rows. Zero-gap original rerun preserved.
- New product tests cover both-mode six springs, five-plus-spring, three-DOF mechanism, duplicate ground DOFs, finite overflow publication, and default mixed selected-state parity.
- Historical exact no-ground-spring nonlinear reference control explicitly removes the variable spring hanger and checks old0.490101N friction and48.952652N normal in both modes. These are historical compatibility values, not independent physics truth. Existing independently expected100N normal reaction test remains. New spring+gap oracle is independent. Numeric adequacy of the retained-spring mixed friction fixture remains D01, not closed by sign/parity checks or observed-value rewrites.

## Scope and handoff

Only two authorized source files and P1 evidence changed. Registered affected checks selected; DEC025/Python/harness/global/native checks deferred to root after fan-in. Browser fixture untouched. No pressure, connector, friction/history, pivot or convergence policy changes; no Git mutation/network/user app use.

Parent must obtain fresh read-only review over both frozen patches/full sources. Artifact is derivative of accepted baseline and kernel checkpoint, not decomposition truth or lifecycle closure. No production release/engineering acceptance claim.
