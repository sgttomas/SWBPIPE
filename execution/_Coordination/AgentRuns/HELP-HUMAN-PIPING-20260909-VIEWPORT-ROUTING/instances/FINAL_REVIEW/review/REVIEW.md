# Combined independent source review

Verdict: `PASS`

Basis: `533332349a4607eee561d4ef90fb05a62d86519e`

Reviewed source cut: the exact 10 paths bound by `SOURCE_CUT_MANIFEST_V1.json`, including the two new viewport-routing files and the generated mechanics fixture. The decoded review diff is SHA-256 `23c7957aa9a16fecc9879e4e8f16cabb21d3cf25893da97742d321a138e0af44` over 221814 bytes.

## Findings

No actionable findings.

## Source-cut binding and scope

- The launch brief, source manifest, resolver, and every named U7, F4, and P5 input matched the required SHA-256.
- All ten live final-source hashes matched the source manifest. The eight tracked basis blobs independently matched their base hashes, and the two private viewport-routing files were absent at the basis.
- The decoded archive contained exactly ten `diff --git` entries and included both new files. The final archive and resolver hashes matched the launch brief.
- `validate_change_scope.py` passed against the exact ten-path allowlist. `select_affected_checks.py` selected `desktop-build`, `desktop-test`, `evidence-sweep`, `harness-self-check`, and `piping-pytest` from the registered profile.
- The exact two-file F4 diff independently hashed to `1246d391a5c362b3f92a57288fe3c03a51b2eb88f4e6d733dc90b27c3ee6404f`.

## U7 viewport routing

The private helper defines all three anchored planes, admits only in-plane constraints, projects to the plane before applying the constraint, and implements the exact 4 CSS-pixel maximum-displacement gate. The component starts on primary pointerdown, captures the actual event target, rejects mismatched IDs, movement beyond the bound, cancellation, and lost capture, and authors only on pointerup. The no-WebGL path leaves the projector unset and preserves manual entry.

Pointer placement sends X, Y, and Z in one `convertDisplayQuantities` request using the model length unit as `from_unit` and the draft unit as `to_unit`. It accepts exactly three finite converted results with distinct expected IDs and the requested unit. Generation invalidation covers manual coordinate/unit edits, cancel, mode/tool/selection changes, and model replacement, so delayed conversions cannot publish stale coordinates. Pointer-derived ghosts distinguish hover from captured model-space points; manual X/Y/Z/unit edits clear them. Existing-route ghosts require two distinct resolved IDs.

The Add/review/Apply path remains the existing intent producer. Continuation is armed by the reviewed route and runs only after that review's exact commit token returns through the model. It retains the selected plane, applicable constraint, coordinate unit, and reusable pipe fields while moving From to the accepted end and clearing consumed identity/provenance. External model events and canceled or failed application invalidate the pending continuation.

The focused helper tests exercise every plane and applicable constraint, exact and over-bound gesture movement, out-and-back cancellation, pointer mismatch/cancel, mixed-unit requests, strict result rejection, generation invalidation, and ghost resolution. App tests cover no-WebGL fail-closed behavior, stale conversion, Add/Apply preservation, and own-commit continuation. Playwright covers a visible elevated construction plane, disabled out-of-plane constraint, hover/captured/existing ghosts, drag cancellation, editable capture, exact atomic review, Apply, and continuation in both registered viewport projects. These assertions would fail for realistic plane, unit, gesture, stale-response, ghost, or continuation regressions.

## F4 friction final-state admissibility

The public active-set evaluator no longer persists its preliminary sliding classification unconditionally. The integration loop now resolves each current sliding row from the solved current iterate, supplies the validated per-support override to the evaluator, and requires both final tangential and derived-normal branch admissibility before convergence.

For positive contact and positive limit, the final sliding state requires a present applied force, nonzero current displacement, applied force opposing that displacement, and reported tangential reaction opposing the same displacement. Exact zero/negative normal contact leaves sliding, exact zero friction limit requires nonzero motion and no applied force, and an inadmissible derived-normal trial remains provisional for retry. Evidence is associated by support ID, so mixed-row order cannot change the result. A non-converged capped exit still emits a failure diagnostic even when the classifier residual is zero.

The tests cover both displacement signs, seeded and transitioned sliding, exact floating-point neighbors, sub-limit and super-limit cases, zero friction/zero motion, negative and zero contact, current-force/current-motion use, row reordering, iteration caps, derived-normal branch retry, and dense/sparse parity. The supplied focused evidence reports 22 nonlinear-support, 38 integration, 19 benchmark, 3 product-friction, and 1 mixed test passing. The exact source and diff hashes bind that evidence to this cut.

## P5 endpoint stress, curved frames, and fixture

Raw endpoint force/moment rows still publish the unchanged element end actions. Straight endpoint stresses now consume section-cut resultants at fractions 0 and 1, giving the same j-side section-action convention as interior stations. Curved endpoint and station recovery rotate the chord-frame j-end action back to global and use arc equilibrium to return the actual tangent/radial/bend-normal frame at each fraction. Stress recovery consumes the same six-resultant layout for endpoints and stations.

Pressure thrust and pressure membrane recovery now share one eligibility predicate: exact `category=pressure`, exact `dimension=pressure`, and an element target. Eligible records retain summation behavior; hydrotest-shaped records cannot create pressure thrust or membrane stress. The tests include analytic straight axial, torsion, and two-plane bending signs at both endpoints, reverse orientation, dense/sparse parity, fixed/free pressure and thermal cases, hydrotest exclusion, multiple-pressure summation, and all six curved resultants against an independent assembled-arc equilibrium and `K d - p` proof. These oracles would detect endpoint sign swaps, chord-frame publication, pressure misclassification, dropped eligible loads, and changes to raw actions.

The generated fixture retained all 830 result IDs. All 144 raw endpoint action rows are byte-equal and all numeric leaves in 378 station rows are preserved; 85 endpoint/metadata/combination rows changed. A structured comparison independently found no added or removed leaves and confined changes to result values and metadata on those rows. The candidate, live fixture, and public fixture-parity evidence share SHA-256 `fb6724aafca965509b999390b3abd4a3eed6b8b85551a6b308ec5aa529c2a58c`.

## Qualified P5 boundary

The complete-document schema suite does not pass. It reports 196 errors: 6 checksum canonicalization, 114 nodal-kinematics component vocabulary, 52 component multiplier metadata, 8 expansion-joint review metadata, and 16 support-review metadata errors. The baseline proof shows these failures at the accepted source basis and zero errors on the corrected endpoint and affected curved-station rows; the real public runner's attached document equals the explicitly built document in all three witness cases. Mapping the known values to current closed enums would be lossy, so this review does not accept an alias, `TBD`, omission, or a public contract. The registered DEC-025 suite and the broader schema contract repair remain mandatory downstream.

## Residual risk

The U7 browser and native checks supplied to this review validate the selected interaction paths, but do not exhaust every GPU, pointer-device, or catalog-unit environment. F4 is a bounded active-set implementation and is not external industry-solver validation. P5 retains the documented broader pressure and stress-model limitations, and the 196-error public-schema baseline remains open. These are explicit downstream constraints rather than regressions introduced by the reviewed cut.
