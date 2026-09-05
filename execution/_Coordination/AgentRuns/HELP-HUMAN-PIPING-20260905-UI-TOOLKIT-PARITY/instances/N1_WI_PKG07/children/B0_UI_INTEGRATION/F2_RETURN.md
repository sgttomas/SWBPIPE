# N7 F2 scoped repair return

RUN_STATUS: REVIEW_PENDING
Parent: N1 WORKING_ITEMS / HELP_HUMAN formal v9 release.
Role: bounded Agent2, software-bounded-implementation method, no delegation.
Basis: N7_F2_RELEASE_V9.md, released N7_F2_BRIEF_HELD.md, N7_F2_READONLY_PLAN.md and alias amendment N7_F2_AMENDMENT_V9A.md. Original two source preimages match V5 frozen hashes.

Corrected selected support stiffness metadata in modelView.ts. The chosen complete top-level stiffness record takes precedence over hanger stiffness; its DOF stays associated with its original quantity. Accepted UX/Ux/ux through UZ/Uz/uz produce Linear stiffness/linear_stiffness; RX/Rx/rx through RZ/Rz/rz produce Rotational stiffness/rotational_stiffness. Exact shared rich_authoring alias spellings are recognized without modifying source, trimming whitespace or accepting unsupported mixed-case spellings. Unknown DOF produces neutral Stiffness/unknown and visible entered-unit fallback. Legacy properties.linear_stiffness is used only without a complete typed stiffness record and retains its existing linear meaning.

Only modelView.ts and DisplayIntegration.test.tsx changed. No other source, schema/backend/artifact, Git, lifecycle, documentation, or build writes/actions. ExistingToolkitEngine.test.tsx remains outside this repair. ToolPolicyCompliance: PASS; bounded reads, two-file edits, focused desktop tests and scope validation only.

Verification: DisplayIntegration.test.tsx PASS51/51 (includes36 top/hanger×six DOF×accepted case variants, precedence/legacy/unknown checks and actual Rust presentation); existing App inspector selection regression PASS1/1 with117unrelated skipped. Rotational SI reports converted N*m/rad, US rotational visibly preserves entered value without a target; translational US converts to lbf/in. Model JSON/hash and entered quantity/unit/draft strings remain unchanged. F2_CHECKS.json records commands, exact raw log SHA references under _run_records/f2, and unchanged accepted operation artifact e9590a97b6ff9f965262ec1567457b98ccff5183d8539607b68ec14e4af6a30d. No build or operation mutation tests run. F2_SCOPE.json PASS.

Frozen outputs: F2_SOURCE_MANIFEST.json; exact F2_SOURCE_FROZEN files; F2_REPAIR.diff (complete two-file delta from accepted V5). All implementation/tests complete within this fence. Source now frozen for fresh repair review; no further source edits until routed findings. Root N7 backcheck and parent aggregate snapshot update remain required. This return is derivative implementation evidence, not lifecycle closure or replacement decomposition truth.
