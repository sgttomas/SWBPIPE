# Structural numeric addendum — ready for independent review

[STRUCTURAL_NUMERIC_BINDING.json](STRUCTURAL_NUMERIC_BINDING.json) completes the existing `complete_case_material_section` obligation with nine explicit section quantities: outside diameter, effective wall, outer/inner radii, wall/bore areas, principal/polar second moments and section modulus. It applies to both frozen cases in both solver modes. All original 25 files match their original manifest and checkpoint `74bd1bac253a9d58bac095efd8b8b888a3d351cd`; none was edited.

The only new reference quantity is bore area, `Ai = pi*ri² = 0.0081*pi m²`, approximately `0.025446900494077325 m²`. The existing independently checked Decimal100 annulus/pi supplies the basis. A/I/J/Z and radii retain their original reference meanings; no product result was used as a target. The addendum gives each field's dimension, unit, formula, decimal expectation, binary64 reference value and explicit criterion reference.

Selected bounded criteria for independent review:

- Outside diameter and effective wall must equal their actual bound, already-metre input values exactly. Neither case has a wall deduction or shared-section override; this identity rule must not be generalized to other unit/section inputs.
- All seven positive derived quantities use `abs(observed-reference) <= 1e-9*max(abs(observed),abs(reference))`, in the stated unit, with no absolute floor. Require finite positive numeric values; missing, boolean, string or nonfinite data cannot pass. No near-zero policy is needed for these fixed positive quantities.

These are new structural development bindings for the two named cases, completing the selected field-comparison policy. They do not modify inherited assertions, numerical-method criteria or a global/release tolerance. This file is a small structural binding, not a replacement result-row tolerance schema; do not assign area/inertia a fictitious result family to fit a row-only adapter.

Implementation: locate exactly one `exact_cases` record with `load_case_id="case"`, then exactly one `pipe_sections` record with `pipe_id="pipe"` and the declared geometry basis. Check all nine fields using their rules and exact input byte/hash/model identities. Failure of any subcheck fails the existing structural obligation. Retain nine subcheck outcomes per case/mode: 36 for four future executions. The original 73 scalar rows and 10 structural obligations per execution are unchanged.

No additional numeric enclosure tolerance is introduced. Finite ordering/containment, station constraints and summary equality remain exact relational checks. Method-specific certified-gap consistency stays with the pinned ordinary physics-1 validator; if that check is unavailable, report the existing maximum-evidence obligation unavailable instead of inventing an epsilon. Compare the analytical normal maximum using its already selected scalar criterion separately; do not require the ideal-decimal oracle to lie inside the narrower binary64-coefficient enclosure.

[CHECKS.json](CHECKS.json) confirms arithmetic preparation and original-byte preservation. No solver, product, gate, build, native or external run occurred. ROOT will arrange independent review and technical selection; no further owner-method decision is requested.
