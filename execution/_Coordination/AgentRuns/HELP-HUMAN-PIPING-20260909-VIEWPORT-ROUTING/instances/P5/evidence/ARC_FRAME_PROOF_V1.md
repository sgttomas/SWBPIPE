# P5 independent curved endpoint arc-frame proof

Status: PASS

Frozen source SHA-256: `dbfff6d5c2fa9241e972042fd2c3a6ce35c6d60f4a973dd5a083a7fe5dfb9c58`

The focused oracle `endpoint_section_cut_curved_endpoints_use_all_six_arc_resultants` supplies an independent frame/equilibrium proof rather than comparing the product path with itself:

1. It solves a quarter-circle product model carrying a nonzero global-y nodal force, a nonzero global-z distributed load, and nonzero pressure. This combination excites the curved frame and distributed/radial equilibrium terms (`lib.rs:17058-17073`).
2. It reconstructs the public raw end-action vector from the twelve published chord-frame force/moment rows (`lib.rs:17075-17113`).
3. Separately, it constructs the curved-bend crate element from explicit end nodes and an explicit center at `[1, -1, 0]`; the center comes from the radius/chord sagitta, so its endpoint tangents and inward radial axes differ from the straight chord frame (`lib.rs:16369-16397`). This direct element does not use product-physics bend assembly.
4. It independently solves `K d - p` for all twelve chord-frame end actions and proves every published raw row remains that action, within the public six-decimal serialization tolerance (`lib.rs:17115-17150`).
5. At fractions `0.0` and `1.0`, it compares all six resultants returned by the product section helper against a direct call to the curved-bend crate's `arc_section_resultants_with_radial_pressure`, using the reconstructed j-end action, distributed intensity, and radial pressure thrust (`lib.rs:17169-17195`). The relative tolerance is `1e-9`.
6. It independently recovers the four mechanical stress components from each direct six-resultant endpoint cut and compares them to the published endpoint stress rows (`lib.rs:17196-17220`). It also checks the exact curved convention and canonical metadata (`lib.rs:17221-17233`).
7. The same direct arc helper checks all six public resultants at fractions `0.25`, `0.5`, and `0.75`, plus curved station force and stress metadata (`lib.rs:17235-17290`).

The exact curved convention frozen in the producer is:

> local x is endpoint arc tangent toward j; local z is bend-plane normal; local y is z cross x toward arc center; resultants come from section equilibrium over assembled end actions

Validation command and result:

`cargo test --manifest-path core/product_physics/Cargo.toml endpoint_section_cut -- --nocapture`

Result: PASS, 8 passed, 0 failed, 137 filtered out.
