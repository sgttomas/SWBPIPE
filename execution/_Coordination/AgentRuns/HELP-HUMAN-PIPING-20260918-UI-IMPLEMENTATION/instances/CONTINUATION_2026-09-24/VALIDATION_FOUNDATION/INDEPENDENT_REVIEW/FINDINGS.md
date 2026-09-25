# Independent review findings — freeze 01

All locations below are relative to `projects/chirality-piping/validation/qualification/` in the resolved `/private/tmp/piping-validation-foundation-20260925` checkout. These are findings on the complete four-file SOURCE_FREEZE_01 candidate, not product defects or programme outcomes. No source repair has been authored by this reviewer.

## IR-01 / P2 — Preserve capability obligations in profile output sets

Location: `capability_inventory.json:1612–1618`, with related sets at `1641–1649`, `1667–1672`, and `1791–1794`.

The Q1 profile selects pressure_thermal, thermal_material_states and self_weight, but its required output list omits their pressure_state, material_state and load_inventory sets. Q2 omits the load_inventory required by wind_seismic_static. Q3 selects nonlinear_restraints, thermal_material_states and stress_results but omits member_actions, stress_fields, material_state and load_inventory. Q8 includes the rule framework without rule_context. A consumer building its required ledger from the profile list would therefore lose explicitly mandatory evidence while still satisfying that list. This contradicts the candidate's complete-denominator purpose and the frozen graph's VP-STATIC/RESTRAINT/WORKFLOW obligations.

Remediation: make the inheritance rule explicit and enforce it mechanically. Preserve every required capability-output edge in its profile, or record a justified explicit scope distinction. Keep Q0 machinery controls distinct from solver accuracy; do not turn a subset or a synthetic control into physical qualification. Do not remove required capabilities merely to reconcile the output lists. `PROBES.json` records all six profile differences and a dropped-output corruption that the current checker accepts.

## IR-02 / P2 — Check every declared link and the retained coverage sets

Location: `check_inventory.py:26–33`, `51–64`, and `68–72`.

The checker never traverses `vp_mapping.q_refs`, does not enforce the retained 17-node VP set, does not check first_profile_cuts IDs for uniqueness, and permits contracts/harness gaps to have no source links. Independent in-memory mutations adding a nonexistent Q target to VP-HARNESS, duplicating STATIC-CORE-FIRST, deleting VP-SOURCES, and clearing production_cli source_refs each return an empty error list. It also accepts reducing Q1 output obligations to run_identity alone. Thus PASS can certify an inventory with broken links, a missing required VP node or lost output/source obligations, despite the README's links/declared-coverage description.

Remediation: validate both directions of the declared Q/capability/VP relations, exact retained VP coverage, unique nonempty first-cut identities and their references, required nonempty source links, and capability-to-profile output obligations. Add focused mutation controls for these cases. A SHA-256 tamper is correctly rejected; retain that check. These changes should remain a small consistency checker, not a qualification framework or a general schema engine.

## IR-03 / P2 — Use the real solver-mode names and disclose CLI reachability

Location: `capability_inventory.json:808`; related future both-mode obligation at `first_profile_cuts.STATIC-CORE-FIRST.held_requirements`.

The inventory names DenseReference, but the pinned main enum is `DenseScrutiny` and its wire token is `dense_scrutiny` (`core/product_physics/src/lib.rs:603–614`). The proposed CLI route calls `run_preview_model_value_with_rule_check`, which currently selects SparseInteractive (`core/runner/headless/src/lib.rs:710–711`; CLI `:757–761`). Product-library support for both modes therefore does not itself make dense execution available through the proposed unchanged CLI wrapper.

Remediation: correct the enum/wire vocabulary and state that the current CLI cut is sparse only, with an explicit owner dependency for the dense mode route before claiming both-mode production coverage. This is manager-acknowledged, independently source-confirmed here; it does not require a product edit in this mapping slice.

## IR-04 / P2 — Bind the new SSLL106A formulation conclusion to its actual source packet

Location: `README.md:77–84` and `capability_inventory.json:2457–2471`.

The candidate says source preparation identified TUYAU_3M shear in the SSLL106A transverse target, but the catalogue links only the two preserved reports. Those report bytes provide SSLL106 leads and general shear-formulation warnings; they do not contain the asserted TUYAU_3M-specific conclusion. No source-owner packet/hash is bound in this freeze. The conservative incompatibility warning may be sound, but a reader cannot recover its actual evidence from the advertised source_refs.

Remediation: link the precise separate source-owner packet and exact source/packet hashes that support the conclusion, retaining its actual independent-review, admission and scoring-readiness state; alternatively mark the detail as an unverified source-owner lead. Acquisition is not reference admission and this review has not inspected external assets. The manager reports the source packet has now arrived, so this is a bounded provenance repair, not a request for new network work.
