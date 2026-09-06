# P9-A V2 unit-contract correction

RUN_STATUS: SUCCESS
ControlSurface: MERGED (parent explicit followup correcting P9-R2-001)
TaskProfile: NONE
TaskSkill: NONE
ToolPolicyCompliance: PASS
WriteAuthorization: ALLOWED_WRITE_TARGETS

Both scalar and station selectors now assert independently specified published units before returning the numeric value. Exhaustive fixture-local kind mapping covers mm, N, N*m, MPa, and existing enum encoding state_code. Unknown kinds fail closed. Nonlinear displacement/reaction mapping is explicitly limited to this suite's translational DOFs; no rotational-unit generalization is claimed.

Only src/lib.rs changed. No numeric oracle, fixture input, tolerance, dependency, public product contract or production source changed. Original author18-member manifest reverified byte-identical, and all four original crate files copied byte-identically into source_before. New full four-file source binding is AUTHORED_SOURCE_FINAL.json; CHANGES.diff captures the entire correction.

Verification: rustfmt parsed/formatted successfully; both helper paths statically verified to call checked_value; original packet hashes verified. No build, product execution, negative-control execution or compiled test PASS claimed. Fresh bounded reviewer backcheck and later accepted-source tests remain required.

ToolsUsed: shell read inspection; python3 scoped edit/hash/diff; rustfmt.
MISSING: fresh bounded backcheck; accepted-source green run and mutation controls.
NEEDS_HUMAN_RULING: none.
DEPENDENCY_NOTES: P4/P5 source acceptance remains upstream of final integration verification.
