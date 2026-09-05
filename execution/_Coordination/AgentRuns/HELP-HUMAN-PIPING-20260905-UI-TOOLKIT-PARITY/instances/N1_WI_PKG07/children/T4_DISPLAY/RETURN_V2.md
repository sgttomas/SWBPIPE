# T4 display review remediation v2

Basis: sealed REMEDIATION_V2.md and fresh R_T4_DISPLAY finding. Same accepted upstream and scope as RETURN.md; original return and source manifest preserved as historical evidence. This derivative return is not decomposition truth or lifecycle acceptance.

Added US volume_per_length target in^2, which is accepted by Rust unit_supports_dimension through the Area alias. Correction: RETURN.md incorrectly listed volume-per-length among unavailable US catalog targets. It is supported and now displayed. Remaining unmapped US dimensions: rotational stiffness, velocity, acceleration, thermal conductivity, specific heat and thermal expansion coefficient.

Regression mounts a US readout for an immutable m^3/m source and verifies one converter request preserves dimension_id=volume_per_length, source value/unit, and target in^2; it verifies returned converted text and absence of fallback. Numeric conversion remains entirely Rust-owned (test wire result is fixed, not TS arithmetic).

Verification: desktop `npm test -- --run src/features/display-units/displayUnits.test.tsx` PASS 6/6 at 22:47:29 on 2026-09-05. Only targets.ts and displayUnits.test.tsx changed during remediation; index.tsx unchanged. SOURCE_MANIFEST_V2.json freezes all three module source files. Independent backcheck and B0 integrated review/test remain required; status REMEDIATED_BACKCHECK_PENDING.
