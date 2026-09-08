# P5 sign-convention clarification V2

Status: ADDITIVE TERMINOLOGY CORRECTION. Read this file with `INVESTIGATION_REPORT.md`. It supersedes only the endpoint-action labels identified below; all equations, section-cut transforms, numerical results, physical cases, recommendations, and Owner choices in the frozen report remain unchanged.

Predecessor: `INVESTIGATION_REPORT.md`, SHA-256 `736b82572f887cfc367db579d66e5d55d2d3a3d618662b6a17a6c91966c831af`. The predecessor remains byte-for-byte unchanged so the correction and hash lineage are explicit.

## Correct terminology

For local `x:i->j` and a tensile wall resultant `N>0`,

```text
node-on-element, stiffness-conjugate endpoint vector: [-N,+N]
element-on-node reaction vector:                     [+N,-N]
tension-positive section cuts: Ncut_i=-Fi_x, Ncut_j=+Fj_x
```

Thus the base vector used throughout the P5 derivation is the external node-on-element load required to sustain the element deformation in `Kd=f_ext`. If a result reports element-on-node action, both endpoint signs reverse. The report’s section-cut transform was already correct.

## Current-source mapping

- `straight_pipe::recover_local_forces` computes `local_forces=K_local d`; its tensile-extension test verifies `[-N,+N]`. These are node-on-element, stiffness-conjugate endpoint forces.
- `add_pressure_thrust_loads` adds `[-P,+P]` to the assembled right-hand side. This is the applied external pressure-equivalent nodal load and, on the isolated element/closure system, uses the same node-on-element orientation.
- Straight recovery forms corrected endpoint forces from `Kd-f_equiv`; the product’s macro-recovery comment explicitly calls the analogous quantity the “true node-on-element end forces.” The corrected straight vector is `[-S,+S]` under the report’s effective-force definition.
- In the fixed/fixed pressure case, raw `Kd=0`; pressure correction yields the corrected/reported node-on-element endpoints `[+P,-P]`, equivalent to `S=-P`. The report’s phrase “raw endpoint rows” for this pair was imprecise.

## Supersession map

In `INVESTIGATION_REPORT.md`:

1. Decision-ready item 3 and the first endpoint-sign table row use “node-on-element, stiffness-conjugate endpoint pair/vector” for `[-Nw,+Nw]`.
2. The sentence following the sign table reads: “If a product reports element-on-node rather than node-on-element action, every endpoint-vector sign reverses.”
3. The current-comparison pressure pair `[-P,+P]` is the “applied external pressure-equivalent RHS vector.”
4. The fixed/fixed `[+P,-P]` pair is the “corrected/reported node-on-element endpoint pair”; raw `Kd` is zero.

The same normalization applies wherever the manager or child return uses the shorter phrase “member endpoint actions.” The child checker re-opened under its original bounded read-only scope and independently verified this correction against current source. It performed no writes or delegation.
