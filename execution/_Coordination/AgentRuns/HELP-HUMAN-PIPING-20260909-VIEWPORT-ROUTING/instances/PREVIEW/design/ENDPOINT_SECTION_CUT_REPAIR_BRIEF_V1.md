# Endpoint section-cut repair brief V1

Status: **FROZEN INDEPENDENT FINDING — CHANGES REQUIRED**. This is a read-only engineering-review output. It grants no source authority and deliberately does not resolve the broader pressure/Poisson/topology contract.

## Finding

Endpoint stress recovery consumes the wrong mechanical object. `corrected_local_forces` contains node-on-element endpoint actions and is correctly published unchanged by `append_element_force_results` (`core/product_physics/src/lib.rs:1570-1573,7176-7254`). Straight station recovery converts the i-side action to the common j-side section-cut convention by negating all six resultants (`:6325-6353`). Endpoint stress recovery instead passes raw i- and j-end action slots directly into `ForceResultants` (`:1660-1666,7475-7503`). A uniform tensile state `[-N,+N]` therefore yields endpoint axial stresses `[-N/As,+N/As]`, although the same physical section state is `+N/As` at both ends. The same object mismatch applies to torsion and bending.

Curved-bend endpoints have a second defect. `recover_curved_bend_local_forces` publishes end actions in the chord frame of the replaced straight span (`:6874-6879,6942-6953`), but physical section stress at each bend end needs the actual arc section frame. The existing `CurvedBendMacroElement::arc_section_resultants[_with_radial_pressure]` already defines a right-handed frame with `x` tangent toward node j, `z` the bend-plane normal, and `y=z cross x`; at fraction 0 it returns the negated i-end action and at fraction 1 the j-end action, including distributed-load and pressure-wall equilibrium (`core/solver/curved_bend/src/lib.rs:345-359,404-448,557-589`).

This finding is generic and independently confirms the sign conflict reported by P0. It is actionable before the new pressure material/topology contract. The isolated repair will make legacy endpoint stress agree with legacy station stress; it will not convert the current generic axial resultant from its legacy effective-force-like pressure meaning into physical wall force, add Poisson pressure strain, or restore the suppressed longitudinal-pressure row.

## Smallest coherent repair

Change only `core/product_physics/src/lib.rs` initially:

1. Preserve `append_element_force_results` and all raw endpoint action IDs, values, and metadata byte-semantically.
2. Add one resultants conversion seam that returns tension-positive/common j-side section resultants at `end_i` and `end_j`.
   - Straight pipe: evaluate the existing `straight_section_resultants` at fractions `0.0` and `1.0`. This retains its distributed-load equilibrium and its established all-six-component sign convention instead of duplicating signs.
   - Realized curved bend: generalize/reuse `curved_bend_station_resultants` so the existing arc equilibrium function is evaluated at fractions `0.0` and `1.0`, after the existing chord-end-j-to-global rotation and with the same uniform intensity and radial-pressure thrust. This returns resultants in each endpoint's actual tangent/radial arc section frame.
3. Replace `recover_endpoint_stress(&corrected_local_forces, offset, ...)` with recovery from those two section-resultant arrays. Reuse `recover_station_stress` or rename it to a location-neutral section-resultant helper.
4. Emit endpoint stress metadata that names `recovered_from_local_element_stiffness` / `element_local` for straight sections and `arc_section_equilibrium_from_assembled_end_forces` / `arc_section_frame` for realized bends. Keep endpoint stress IDs stable. Do not relabel raw endpoint action rows.
5. Keep component stress-multiplier result references and stress-summary collection pointed at the corrected endpoint stress objects; their stable endpoint stress IDs need not change.

No change is required for `core/loads/stress_recovery`, either schema, transform/adapter, UI, or public raw-force rows for this repair. If a committed generated preview fixture contains endpoint stress values or metadata, regenerate only that fixture through its existing deterministic generator after the focused Rust checks pass; do not hand-edit it.

## Required focused oracles

- Straight uniform axial tension: raw actions remain `[-N,+N]`; endpoint and all station axial stresses are `+N/As`.
- Straight uniform torsion: raw end moments remain `[-T,+T]`; endpoint and station torsional stress signs agree under the common cut convention.
- Straight pure bending about each local axis: endpoint recovery equals `straight_section_resultants(..., 0/1)` divided by the corresponding section modulus; an element-orientation reversal preserves the physical stress field after frame transformation.
- Combined pressure plus temperature, fixed/fixed and fixed/free: endpoint stress exactly equals the current quarter/mid/three-quarter legacy section convention for every uniform axial state. Explicitly record that this is parity with the legacy resultant meaning, not validation of wall-force pressure physics.
- Realized curved bend under pure pressure: raw endpoint force rows remain chord-frame actions; endpoint section axial stress is `+P/As` at both ends and uses `arc_section_frame`, matching arc stations. Under a nontrivial distributed or nodal load, fractions 0 and 1 match the arc helper's end equilibrium for all six resultants.
- Dense and sparse modes return identical corrected endpoint stress rows for the same linear case.

## Candidate fence and follow-on

Candidate source fence: `projects/chirality-piping/core/product_physics/src/lib.rs`, its co-located focused unit tests, and only the deterministic result fixture(s) proven to change. The pressure redesign remains separately held. After implementation, a fresh reviewer should inspect 100% of this frozen diff before the DEC-025 sweep; any later exact-annulus/Poisson/topology tranche must consume section resultants rather than raw endpoint actions for stress recovery.
