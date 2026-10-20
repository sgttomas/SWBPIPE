# O4: the Timoshenko and corrosion-allowance source extension

This is the implementation design for the fifth post-T3 shared epoch ([b7-plan.md](b7-plan.md), "After B7b", item 5). It extends retained-source recovery (`pressure-source-1`) and W1-E publication (`physics-retained-1`) to the exact-contract documents that DEL-04-08 ScopeOfWork REQ-006 (a) declines today: documents whose contract selects `timoshenko_annulus_energy_v1`, and documents with a nonzero mill tolerance or corrosion allowance on any pipe section. It keeps RF's closure, witness and charging rules, and it keeps every gate, limit, tolerance and frozen reference unchanged.

**Status: implemented (epoch 5), as built on branch `claude/piping-o4-impl-20261019`.** Revision 7 records what was built where it differs from or sharpens revision 6: §3.3 (final limitation strings), §3.5 (measured work law), §4.2 (K4 encoding version and framed block), §4.7 (reduced preparation records validated like nominal ones), §5 (reference unit and oracle as frozen and reviewed), §6 (test deviations) and §7 (registered D3). The basis of revision 6 was main `084ff961f8`, epoch 4 as built; implementation started from main `3826a62881`.

**Revision 6** re-checks the design against epoch 4 as built (coordinated-knowledge-work §5). The changes:
- section 0 records each assumption's as-built basis (file and line at `084ff961f8`) and verdict;
- W1-E's derived not-covered rows need a basis dimension that epoch 4 did not build; section 4.5 specifies it, with its mutant (6) and owner (7). This replaces E4-3's "O4 adds no recipe for them", which held on the source route only;
- the source route's τ_v,max reads the basis-free shear rows; only its c_b follows the basis (3.2);
- section 7's feasibility is redone against epoch 4's registered profile; the reviewed-input numbering is stated;
- AC-002 is cited in the owner's 2026-10-19 text, which #1461 applied; the proposed wording and the reserved reading are withdrawn (3.6, 10, 11);
- the epoch-4 renames, the third O4 decline test and the refreshed line citations are applied (1, 4.7–4.9, 6, 12).
Sections 4.2–4.4 (the kernel member, the lanes, the lemma and λ = 2) are unchanged: epoch 4 touched no file they read (section 0, "Unchanged by epoch 4"), so rounds 1–5's review of them stands.

**Revision 2** answers the first round of dual review (Claude Opus 5.5 and Codex). The changes:
- the Timoshenko shear leaf moves into a sidecar, under the preparation law (4.2);
- the warrants are closed by a dominance lemma (4.4);
- the D-entry gaps are formed at the exact length (4.3);
- the private normalized source is kept apart from published evidence (3.1–3.4, 5);
- the source work terms are derived from their actual charging sites (3.5);
- both section-term schemas are covered (4.6);
- the reader row universe is specified (4.7);
- the account of which bytes move is corrected (3.6, 4.9, 12).

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

**Commission.** The owner, 2026-10-12 (O4): "Scope extension after B7b". The owner's resumption steer of 2026-10-14 orders the shared epochs "each under its own design and independent review". DEL-04-08 ScopeOfWork REQ-007 (a), which the owner approved on 2026-10-17 ("I approve #1405 as required"):

> O4: the Timoshenko and corrosion-allowance source extension. This is the fifth shared epoch in the owner's order of 2026-10-14. It extends retained-source and W1-E publication to the documents REQ-006 (a) declines, including those with a nonzero mill tolerance. It keeps RF's closure, witness and charging rules.

AC-007 (a) gives the discharge conditions: the extended cases publish verified results under REQ-001 or REQ-002; the retired decline tests are replaced by admission and refusal tests; AC-001 to AC-005 pass on the head with every gate, limit and frozen reference unchanged; the actual-source D3 registration passes.

## Summary

- **Retained-source recovery (sections 3).** The source method's declared operand is already the built frame's binary64 local matrix. A Timoshenko frame supplies its own Timoshenko local matrix, and the signed-permutation path and RF's exact congruence carry it unchanged. The dual-basis work is in the stress publication: each basis has its own recovery section, stress checks and endpoint maximum, all formed from the same retained endpoint functionals. `SOURCE_MAXIMUM_BASES_DISTINCT` and the three `prepare_sources` declines are retired.
- **W1-E (section 4).**
  - The retained kernel gains a Timoshenko straight member. The shear leaf lives in a member-only sidecar, so every layout a receipt records stays unchanged.
  - The D block is written in natural-mode form per bending plane, (EI/L)·P_a + 3(EI/L)/(1+φ)·P_s, with a fixed formation recipe at the working precision. Its absolute operand is D̄ = λ(c + c_s)·[[1,1],[1,1]] with λ = 2, rounded upward. Section 4.4 derives λ = 2 finitely from the existing derivations' own coefficient counts. With that operand, every term of every existing Euler–Bernoulli derivation bounds its Timoshenko counterpart, so R7's t₁ and E's factor 69, including the reaction and support warrant, transfer unchanged (section 4.4).
  - Each lane forms the shear rigidity from its own readout. The admitted lane uses the prepared binary64 shear-area leaf; the annular lane uses an enclosure of κ_E from the exact annulus.
  - Wherever the kernel bounds D by coefficient gaps and fixed patterns, a Timoshenko member is bounded by D-entry gaps formed at the exact length. Euler–Bernoulli members keep today's code and bits.
  - A member whose reduced stress section differs from its nominal one gets a reduced section term, and its reduced stress rows are certified on that section as receipt rows. A member whose sections are bit-equal keeps today's overlay of the reduced twin.
  - U4b's not-covered derived rows (σ⁺/σ⁻, τ_v,max) gain the basis dimension epoch 4 left out: each basis's rows are formed from that basis's own projected rows and section (4.5).
- **Admission and readers.** W1-E's G-A admits both shear models and nonzero allowances. The retired facts are `ShearModel` and `SectionAllowance`. G8, G5b and the source readers stop requiring Euler–Bernoulli and zero allowances. In their place they rederive the D-5 bases and the Timoshenko leaf from the captured invocation and bind them, through the rederivation the pressure-2 readers already have (DEL-05-03 `section_basis`).
- **What moves.** The limitation texts must stop stating the O4 declines (3.3). Every exact envelope carries them, so every exact ordinary envelope, every pressure-source-1 publication and every route-E successor moves in those strings and in the hashes that bind them. That includes J2's bases, MJ6 and MJ9's pinned bytes and J7's envelope digests (not its results or summary digests), under AC-002's owner text (3.6, 11). Numerical values, classes, selections, first causes and charges of Euler–Bernoulli, zero-allowance documents do not move, and a text-only diff check enforces that (4.9).
- **Proof obligations (section 4.4).** The dominance lemma is derived here from the formation code. The independent numerical review checks it before implementation. If it fails, the epoch returns to HELP_HUMAN with the concrete conflict; no law changes to obtain a pass.
- **Independent references (section 5).** A separate, later reference unit (`validation/references/o4_retained_source/`) judges the new publications against exact solutions of both readouts. It reuses the frozen, independent `kd5_timoshenko` engine. An independent exact solver checks each source receipt's declared problem through a test-only dump bound to the receipt's hashes.
- **Merge with the W1 coverage repair: no (section 8).** The two share no numerical law. O4 lands first, as b7-plan orders, and the repair's census runs as soon as epoch 4 merges. Two coupling rules keep the shared files consistent.
- **Reserved:**
  - the ScopeOfWork edits that discharge REQ-007 (a) (proposed in section 10);
  - any Decision 22 change if D3 does not fit;
  - TBD-002's table sentence, which stays untouched.

## 0. Basis and epoch-4 dependencies

Revisions 1–5 relied on twelve assumptions taken from the epoch-4 design. Each is re-checked here against epoch 4 as built: main `084ff961f8` (#1461 at `f6c0732d58`; #1462 merged just before it, at `cecda3d90a`, repaired an unrelated reader warrant). The basis column names the merged source actually used. Paths are relative to `projects/chirality-piping/`; PP is `core/product_physics/src/`, RE `core/reporting/result_export/src/`, the kernel `core/solver/frame_kernel/src/structural/retained/`.

| # | Assumption from the epoch-4 design | As built (basis at `084ff961f8`) | Verdict; effect on O4 |
|---|---|---|---|
| E4-1 | Model 0.6.0 is the only current version; 0.5.0 is refused by name; behaviour keys on presence. The `"0.5.0"` literals in `SourceContract::from_invocation` and `exact_namespace` become 0.6.0; `namespace_branch` uses `MODEL_SCHEMA_VERSION`. | PP `model_format.rs:13` `MODEL_SCHEMA_VERSION = "0.6.0"`, `:15` `RETIRED_MODEL_SCHEMA_VERSIONS` ending in "0.5.0", `:35` presence; RE `physics_source.rs:26` and `retained_precision.rs:4738` read "0.6.0"; PP `retained_memory.rs:845–852` uses `crate::MODEL_SCHEMA_VERSION`, `:850` `FamilyFact::ShearModel`. | Holds. Sections 3.4, 4.1 and 4.7 edit the contract predicates beside those literals, not the version. Further 0.6.0 literals sit beside O4's edits: `physics_source.rs` `ACTUAL_EXACT_PROFILE` and `ACTUAL_MODEL_SCHEMA`, `retained_precision.rs` `preview_namespace`. |
| E4-2 | `pressure-2` replaces `pressure-1`; physics-retained-1 inherits it by hash; pressure-source-1 keeps its name and its own table, revised in place; `results_pressure_2.v0.3.schema.json`; `REVIEWED_INPUTS` keeps its order with indices 10 and 11 renamed. | PP `build_identity.rs:158–181`: twelve inputs in the same order; indices **5** (`semantic_contract_v0_3_preview_physics_2.json`), 10 (`results_pressure_2.v0.3.schema.json`) and 11 (`semantic_contract_v0_3_pressure_2.json`) renamed. physics-retained-1's `inherited_semantic_contract_sha256` is pressure-2's table hash. pressure-source-1's table moved in place (U4b kinds and limitations). | Holds, with one more rename (index 5). This design numbers reviewed inputs from 0, in `REVIEWED_INPUTS` order; `retained-pricing-rules.md` §8.8 and the registration comment count from 1 (section 7). |
| E4-3 | U4b's σ⁺/σ⁻ and τ_v,max rows are `not_covered` in retained successors and `derived_not_qualified` in source publications, formed from the publication's own published rows. O4 adds no recipe for them; with distinct bases the reduced derived rows follow automatically. | Source route: `source_receipt/rows.rs:648–686` forms σ⁺/σ⁻ from the station's stress rows of the same basis (basis-suffix match) and τ from the two basis-free shear rows; the reader (`u4b_evidence.rs`, keyed by case, entity, kind, component, location and basis) recomputes τ's c_b per basis. W1: `retained_product.rs` `prepared_not_covered` (from `:5949`) keys its operand table `[[Option<usize>;5];5]` by member, site and slot with **no basis**, refuses a second fill (`"derived operand identity"`), and forms τ's c_b from the member's one nominal `facts[mi]`; its comment states the premise: "A route-E row of either basis reads the bound nominal rows (… the bases coincide)". | **Holds on the source route; invalidated on W1-E.** With distinct bases the reduced stress rows become receipt rows with the same `ProductRecipe::Stress` key, so the operand table would refuse, and a reduced τ would use the nominal annulus. Section 4.5 adds the basis dimension and the reduced facts' c_b; section 3.2 states τ's inputs correctly. No recipe is added: the rows stay `NotCovered`. REQ-001's merged clause ("formed in binary64 from the successor's own published stress, section-action and end-force rows and its captured section and material") and DEF-E's `rows.derived_not_covered` ("c_b from the regenerated basis section") already describe the repaired behaviour. |
| E4-4 | The ordinary K-D5 route publishes the corrected displacement fl(u + w + w2) wherever a certified bound exists; Passed/Sensitive classification is unchanged. | PP `lib.rs:14035–14060` `published_nodal_displacements`; limitation text (`lib.rs:1111`, `preview_physics.rs:71`): "… rounded once to binary64 in SI units and then converted to the published unit as every other row is …". DEL-04-01 `kd5-member-relative-bound.md` §3.4: the classification is unchanged. | Holds. O4's fallback tests compare against the merged producer's bytes, which can carry fl(u + w + w2) on Sensitive cases. |
| E4-5 | U5b's tie rods live on objective connectors; D7 declines connectors at `prepare_sources`; W1 capture declines them. | PP `joint.rs:208` (`tie_rods` on the connector spec); `source_recovery.rs:796–797` (D7). W1 refuses connectors at admission through D1's components fact (`retained_memory.rs:943–944`, `:1024–1025`), not at capture. | Holds (W1's decline is at admission). O4 does not interact with tie rods. |
| E4-6 | Epoch 4 re-registers under D3; its headroom is unknown. | `retained_memory.rs:1213–1239` `REGISTERED_PROFILES` (tree `b4c9f35f8e`, 307 atoms, 100 forms, unchanged policy); `retained-pricing-rules.md` §8.8: all 70 evaluations fit; the tightest is merged dense W3 at +2,923,863,736 B. T4-S6's streamed publication hashing (S6-HS) at T16, T17 and T25. | Holds; now known. Section 7 is redone against it. |
| E4-7 | DEF-O, DEF-C and DEF-E name preview-physics-2 and pressure-2 in `inherits.base_contract`; DEF-E's `scope.model_format` takes 0.6.0; their meaning hashes move. | All three as stated; DEF-E `scope.model_format` "current model 0.6.0 …"; DEF-E `scope.shear_model` and `scope.mill_tolerance` still say "until O4"; each definition gains a `rows.derived_not_covered` text. | Holds. Section 4.8 edits DEF-E on epoch 4's bytes and leaves `rows.derived_not_covered` unchanged. |
| E4-8 | Route E's row premise is P_final = 7n + 102m + 8g + 3, below its 16,384 row capacity at the D1 caps. | `retained_memory.rs:3324` `P_FINAL_EXACT = 7·NODES + 102·MEMBERS + 8·SUPPORTS + 3`; caps 32/32/32/3 (`:531–541`); 3 × 3,747 = 11,241 rows. Route L's P_final is 7n + 66m + 8g + 3. | Holds. Section 7. |
| E4-9 | Epoch 4 edits REQ-001 and REQ-002's version tokens and REQ-001's value clause; it leaves REQ-006 (a), REQ-001's other route-E bullets and AC-003 to O4. | DEL-04-08 ScopeOfWork at `084ff961f8`: as stated, plus REQ-003 (c), REQ-006 (c), REQ-007 (c) and TBD-002's token, and **AC-002's second bullet** in the owner's 2026-10-19 text. REQ-006 (a), REQ-001's route-E domain bullets and AC-003 are unchanged. | Holds, with AC-002 now decided. Section 10's proposals are written against this text; section 11 cites AC-002. |
| E4-10 | On pressure-2 the Saint-Venant c_b uses the member's authored ν and each basis's own annulus; route E recomputes it from the captured evidence. | PP `u4b_rows.rs:53` `SHEAR_SIGN_EXACT`, `member_bases`; RE `physics_evidence.rs:1193–1203` forms `shear_factors` per basis from `sections.basis(b)`. | Holds for the ordinary and source producers and for the readers. W1's producer is the exception under E4-3. |
| E4-11 | Epoch 4 adds no W1-E admission clause and moves no W1 or source decline. | `retained_memory.rs` `family_clauses` (`:884–911`) keeps `F::ShearModel` and `F::SectionAllowance`; `source_recovery.rs` keeps the three O4 declines (`:808–823`); `exact_admission.rs` changed tokens only. | Holds. Section 1's inventory is refreshed to these lines. |
| E4-12 | Epoch 4 moves every ordinary envelope's identity, limitation and version bytes, and regenerates J2, J7's consequences and pins, MJ6 and MJ9's bases and the route-E successors by their writers. | `fixtures/results/j7_rf_census_expected.json`: of 322 entries, only `public.identity`, `public.envelope_sha256`, `public.results_sha256` and `public.summary_sha256` moved; class, first cause, work and counts did not. MJ6's 24 rows and MJ9's table hash re-pinned; J2 and the route-E successors regenerated. | Holds. Section 4.9 follows the same practice under AC-002's owner text. MJ9's both-bases tamper relies on coincident sections and is revised (6). |

**Unchanged by epoch 4.** Epoch 4 changed no file in the kernel's `structural/retained/` except `product_certificate/final_case.rs` (the `NotCovered` recipe and its value slot). `assemble.rs` (`form_member`, `dot`, `normalize`, `from_frame`, the Timoshenko refusal), `verify.rs`, `recover.rs`, `product_certificate.rs`, `source_residual.rs`, `bridge.rs` and `loaded_maximum.rs` are byte-identical. So section 4.4's coefficient counts (ℓ = 2.5), the finite D-entry bound and λ = 2 rest on unchanged code. `composite_member_maximum` is unchanged (`composite.rs:165`).

**Other epoch-4 facts O4 now relies on:**
- the twin overlay applies only to `Stress` and `CircularMaximum` rows (`retained_product.rs:2666`); derived rows bind their own positions and are never twins (`:2671`; RE `retained_precision.rs:943–953`);
- the kernel's not-covered value slot is completed after projection (`final_case.rs` `complete_not_covered`), so O4's reduced receipt rows must keep the bound positions it reads consistent (4.5);
- the Python verifier gained `tests/verifiers/u4b.py` and `u4b_model.py`, and `pressure_1.py` became `pressure_2.py`; O4's verifier edits mirror the reduced derived rows there (4.7);
- a third O4 decline test exists outside VER-003: `o4_timoshenko_declines_are_reached_by_published_documents` (`timoshenko_product_tests.rs:474`). O4 converts it (6).

## 1. What is declined today

Code facts on main `084ff961f8` (paths relative to `projects/chirality-piping/`; PP is `core/product_physics/src/`, the kernel `core/solver/frame_kernel/src/structural/retained/`, RE `core/reporting/result_export/src/`):

| Site | What it does |
|---|---|
| `source_recovery.rs` `prepare_sources` (about 800–823) | After the D7 declines and the budget refusal, and before the dimension closure, it declines on an exact document: `shear_model = timoshenko_annulus_energy_v1` ("timoshenko_annulus_energy_v1 straight members are not a retained-source formulation"), a nonzero mill tolerance and a nonzero corrosion allowance. The receipt records `unsupported_family`. |
| `source_receipt/rows.rs` (about 377–394) and PP `lib.rs` `same_stress_section` / `SOURCE_MAXIMUM_BASES_DISTINCT` (about 6161–6171, 11099–11108) | Source stress rows and the endpoint maximum use one section (`built.sections`, the nominal section) and require the reduced section to be bit-equal. Otherwise they refuse ("derived stress section bases differ") or withhold the reduced maximum by name. |
| `retained_memory.rs` `namespace_branch` (840–855), `family_clauses` (about 897–911) | W1-E's G-A refuses a Timoshenko contract under `FamilyFact::ShearModel`, and a nonzero mill tolerance or allowance on branch E under `FamilyFact::SectionAllowance`. The ordinary bytes are published unchanged. |
| Kernel `assemble.rs` `StraightMember::from_frame` (about 256–281), `RETAINED_TIMOSHENKO_REFUSAL` | It refuses a Timoshenko leaf by name. The retained element is Euler–Bernoulli, with D's bending block (EI/L)[[4,2],[2,4]]. PP `retained_product.rs` (about 2023–2038) maps the refusal. |
| RE `retained_precision.rs` `exact_namespace` / `section_allowances` / `exact_mill_tolerances` (4737–4777) | G8 requires the three-member contract with `euler_bernoulli`, an explicit length-valued zero allowance on every section, and zero mill tolerance (`MILL_TOLERANCE_DECLINE`). |
| RE `retained_precision.rs` `g5b_exact_evidence` (about 4419–4487) | G5b binds the receipt's one section to both evidence bases bit for bit, and requires each reduced stress row to be its nominal twin's retained value (the overlay). |
| RE `physics_source.rs` `SourceContract::from_invocation`, `zero_corrosion_allowances`, `zero_mill_tolerances`, the `SECTION_BASES` and `ACTUAL_SHEAR_MODEL` checks, and `validate_stress` (`STRESS_SECTION_PARAMETERS`, `STRESS_BASIS_TWIN`) | The source readers require the Euler–Bernoulli contract literal, zero allowance (`PHYSICS_SOURCE_ACTUAL_CORROSION_ALLOWANCE`), zero mill tolerance (`PHYSICS_SOURCE_ACTUAL_MILL_TOLERANCE`), coincident evidence sections, and one parameter set for both bases. |
| Python `tests/verifiers/physics_source.py`, `retained_precision.py` | They mirror every row above. |
| DEF-E `retained_precision_prepared_exact_v1.json` (reviewed input 7) | Its `scope.corrosion_allowance`, `scope.shear_model`, `scope.mill_tolerance`, `evidence` and `trust.G8` texts state zero allowance, zero mill tolerance and Euler–Bernoulli, "until O4". |
| `exact_admission.rs` limitation 0 (pressure-source-1) and limitation 7 (the unselected exact envelope and pressure-source-1) | The selected response is "Euler-Bernoulli with coincident section bases". Timoshenko and nonzero allowances "are declined by name". |
| Tests named in VER-003, and one more | `o4_prepare_sources_declines_by_name` (`source_recovery.rs`), `o4_w1e_declines_at_admission` (`retained_facade_tests.rs`), and FK `timoshenko_element.rs`'s refusal test; outside VER-003, `o4_timoshenko_declines_are_reached_by_published_documents` (`timoshenko_product_tests.rs`). |

Since T4-U8 made Timoshenko the authoring default, every default-authored exact document takes these declines.

## 2. Scope

**Admitted after O4**, on both retained routes, within each route's otherwise unchanged family:
- `pressure_contract.shear_model` either `euler_bernoulli` or `timoshenko_annulus_energy_v1`, per document, on straight members;
- any finite mill tolerance c_m ≥ 0 and corrosion allowance c_a ≥ 0 with c_m + c_a < t, as T4-U6 admits them, under the D-5 bases (DEL-05-03 Design `section-bases-and-timoshenko.md` §1).

**What D-5 means on these routes.** Both routes require explicitly empty pressure regions. W1-E admits nodal loads only (D1.7 refuses element loads on route E), and the source family declines every non-nodal load. So the pressure bore (Ai, contents mass) and the mass bases never reach either route. On these routes, D-5 comes down to two things:
1. stiffness, the Timoshenko shear area and the eigenload area E·A·ε (source route, load/reference-state group only) use the nominal wall t;
2. every stress row is published on the nominal section and on the reduced section t_r = (t − c_m) − c_a, the D-5 binary64 order. Each basis has its own maximum and headline.

**Unchanged and still out:**
- D7: realized arcs and connectors (with or without tie rods) stay declined by name.
- The source family: nonlinear supports, present pressure regions, components, non-nodal loads, combinations, more than 256 DOFs, RF's order cap of 4, the descriptor-unit law, the subnormal-reserve exclusion and `PHYSICS_SOURCE_WORK_LIMIT`.
- W1-E's D1 domain and caps: nodal loads only, base common E and ν, no named-point or temperature modulus basis, no combinations on route E, and DEF-E's other limits.
- Route L (pressure-free) is not touched. It carries no shear model and no allowance (`SHEAR_MODEL_WITHOUT_PRESSURE_CONTRACT`, `CORROSION_ALLOWANCE_WITHOUT_PRESSURE_CONTRACT`), and it keeps its one (t − c_m) section (H-1).
- Every required-field refusal: `EXACT_PRESSURE_*_REQUIRED`, `*_WITHOUT_PRESSURE_CONTRACT`, the unknown-shear refusal, the section-sum check and the quantity refusals.

## 3. Retained-source recovery (`pressure-source-1`)

### 3.1 Timoshenko: the declared operand is the frame's own local matrix

The source method certifies "the exact solution of the equations formed exactly from the declared local matrices, transforms, springs and loads" (RF §3.1). Its declared operand per member is the built frame's binary64 `local_stiffness()`, closed against the pipe element's matrix and against the assembly (`prepare_sources`, member loop about 1270–1411):
- **Signed-permutation members:** each contribution is ±K_local exactly.
- **Other members:** RF's exact congruence Tᵀ K_local T, with its block-rotation, formation-determinism and unit-precheck closures.

A Timoshenko frame's `local_stiffness()` is the FK Timoshenko matrix (U8 §3.3), with the same nonzero pattern, so both paths take it unchanged. RF §10 item 11 anticipated this: "Its Timoshenko local matrices then enter RF's congruence unchanged. The congruence handles any declared K_local". The closure checks then bind the source to exactly the matrix the ordinary route assembles.

- **What does not change:** the block search, its exact zero test and order cap 4, Sylvester's warrant, the adjugate solve and shared denominator, `BlockWitness`, replay, the metered `Work::mul` with C₂ = 96, the descriptor-unit precheck and the 256-DOF and work limits.
- **When φ is exactly 2:** k_θθ′ = (2 − φ)EI/(L(1+φ)) is an exact zero there. The exact zero test and `member_descriptor_units` already handle a structurally zero entry, and RF-13b's equality control (`congruence_unit_precheck_equals_descriptors_charge`) gains a φ = 2 member.
- **Recovery rows are statics.** Member end actions are K_local·(T u_e). Station rows follow from the end actions through the existing [sign, fraction, length] products. With no member loads on this route, station actions are linear in t, which holds for any beam theory (U6/U8 §3.5). The endpoint maximum (an unloaded span, maximum at an end) is likewise beam-theory independent.
- **Formation inputs are named, privately.** The normalized-source payload is a private commitment, not published evidence. `source_receipt/source.rs` (about 317 and 397–440) builds it, hashes it into `normalized_source_sha256` and returns only hashes and summary fields; the closed public schema carries neither the payload nor its frames. The payload's per-frame `formation_inputs` gains `shear_area_bits`, present only on a Timoshenko frame.
  - The local-matrix bits, already in the payload and in the `Identity` scalars, bind the matrix itself. No `Identity` block is added: the round-1 draft's block was redundant and would have added identity charges (section 3.5).
  - An Euler–Bernoulli frame's payload and `normalized_source_sha256` keep their bytes.

### 3.2 Two section bases

Every stress row already exists per basis on this route (U6 §2.1). Today the reduced row is the nominal row bit for bit. With distinct bases it is formed on its own section:

- **Recovery section (private payload).** The per-frame `recovery_section` stays the nominal section. A new `reduced_recovery_section` is present exactly when the member's reduced stress section differs from its nominal one in any of A, Z, J or r_o (`same_stress_section` false). Those four are the only operands a source stress row or the endpoint maximum reads. It has the same keys, `section_modulus_bits` and `torsion_radius_bits`, plus `area_bits` and `torsion_constant_bits`. The nominal object keeps its keys. Like `shear_area_bits`, this is bound only through `normalized_source_sha256`.
- **Stress checks (public).** `section_stress_checks` stay one per stress row per basis. The reduced check binds the same functional (index and id) and the same action, bit for bit. Its `parameters` are the reduced section's: `area_m2` = A_r, `section_modulus_m3` = Z_r, `torsion_constant_m4` = J_r, `torsion_radius_m` = r_o, `pa_per_mpa` = 1e6. Its value is the existing `retained_source_straight_stress_v1` evaluation with those parameters. These parameters, the per-basis maxima and the evidence's `pipe_sections` are where the readers check the bases (3.4).
  - `rows.rs`'s "derived stress section bases differ" refusal is replaced by a per-basis loop over `exact_basis_section(bases, basis)`.
- **Endpoint maximum.** `composite_member_maximum` (`source_receipt/composite.rs` about 165) evaluates `endpoint_maximum` once per basis from the same retained endpoint functionals [0, 4, 5] and that basis's A and Z. Each basis's `pipe_stress_extrema` entry (keyed by `section_basis`) carries its own value and evidence.
  - `SOURCE_MAXIMUM_BASES_DISTINCT` and its withholding branch in `lib.rs` are deleted.
  - The receipt's `endpoint_sections` (`composite.rs` about 960) stays one per member. Endpoints, functional indices and actions are basis-independent.
- **Headlines.** `validate_composite_summary` already binds both per-basis headlines. Each now binds its own basis's maximum rows.
- **U4b's derived rows** (`derived_not_qualified`) need no change on this route (E4-3, as built). σ⁺/σ⁻ are formed from the axial and bending stress rows of the same station and basis, so with distinct bases they follow each basis's own stress rows. τ_v,max reads the station's two shear rows, which have no basis; it follows the basis only through c_b, which the reader forms from that basis's annulus and the member's ν. The receipt checks σ⁺/σ⁻ bit for bit and τ as finite and nonnegative (`rows.rs` `not_qualified_for`).
- **Bit-equal bases keep their numbers.** A document whose stress sections are bit-equal (zero mill tolerance and allowance, or values that round away) produces today's payload, checks, values and work. Only the limitation strings change (3.3, 4.9).

### 3.3 The receipt and schema

- **Schema.** No change to `physics_source_recovery.schema.json` (reviewed input 1) is needed for the bases or the shear leaf. The new payload members are private, and the public checks and maxima keep their shapes. The actual-source D3 binds input 1 only if the implementation finds a public shape that has to move.
- **Producer.** `failure_fields` loses no mapping. The three O4 decline texts are deleted, and with them every receipt `unsupported_family` that carried them.
- **Limitations** (`exact_admission.rs`, whole static strings at the same positions; product text, reviewed against `docs/claims_registry.md` under AC-008):
  - pressure-source-1 limitation 0. Its last-but-one sentence, "The selected retained-source response is Euler-Bernoulli with coincident section bases.", becomes "The selected retained-source response uses the contract's shear model and publishes its stress rows on both section bases."
  - Limitation 7 (unselected and selected). The O4 clauses ("only for euler_bernoulli documents with zero mill tolerance and zero corrosion allowance, and", "the selected response is euler_bernoulli with zero mill tolerance and zero corrosion allowance, where the two bases coincide", and "Timoshenko documents, nonzero mill tolerance or corrosion allowance and") are removed. The rest of each sentence stands.
  - Every carrier (schemas, readers, verifiers, fixtures) takes the same bytes, as U6 §2.4 required. *As built,* the final strings (only the joining words the removals broke were changed: "; " became ". " before the removed selected clause, and "and other source families" became "Other source families"):
    - limitation 7, selected: "Every member section declares its corrosion allowance and the contract declares its shear model. This pressure-source-1 envelope carries a selected retained-source response. The source family is straight members, linear restraints or springs, nodal force and moment loads and explicitly empty pressure regions; an authored load/reference-state group includes resolved eigenloads and prescribed motions. Other source families are declined by name. Other cases keep their ordinary recovery and standing. No code compliance or professional acceptance is produced, and actual numerical admission remains separate."
    - limitation 7, unselected: "Every member section declares its corrosion allowance and the contract declares its shear model. This envelope publishes ordinary structural recovery. Retained-source recovery is available only for straight members, linear restraints or springs, nodal force and moment loads and explicitly empty pressure regions; an authored load/reference-state group includes resolved eigenloads and prescribed motions. Other source families are declined by name. No code compliance or professional acceptance is produced, and actual numerical admission remains separate."
  - `tools/validation/validate_claims_language.py` passes on them.

### 3.4 Readers and verifier (Rust native and wasm; Python)

| Check today | After O4 |
|---|---|
| `SourceContract::from_invocation`: the contract equals the literal with `euler_bernoulli` | The contract is `{version "3.0.0", mode "exact_pressure_v3", shear_model}`, with `shear_model` either value, still no top-level `shear_model` and still exactly three members. An unknown value is refused as today (`PHYSICS_SOURCE_ACTUAL_CONTRACT_UNSUPPORTED`). |
| `zero_corrosion_allowances` (`ACTUAL_CORROSION_ALLOWANCE`) | Every pipe section declares a length-valued, finite allowance ≥ 0, with c_m + c_a < t. Absent, null, non-length, negative or over-wall allowances are refused under the same code. Zero is no longer required. |
| `zero_mill_tolerances` (`ACTUAL_MILL_TOLERANCE`) | Deleted. The mill tolerance is bound by value through the existing `ACTUAL_SOURCE_GEOMETRY` check, and its range through the shared D-5 rederivation. |
| `SECTION_BASES` (zero allowances, Euler–Bernoulli, nominal = reduced) | Each `pipe_sections` entry's nominal section, reduced section and `beam_theory` equal their rederivation from the captured invocation. This is `section_basis`'s existing D-5 and Timoshenko rederivation (κ and A_s bit for bit, U8 §3.1's fixed order), which the pressure-2 reader already applies. |
| `ACTUAL_SHEAR_MODEL` (Euler–Bernoulli only) | `beam_theory.model` equals `section_basis::selected_model(contract shear_model, arc = false)`. |
| `STRESS_SECTION_PARAMETERS` (one parameter set equal on both bases) | The check's parameters equal its own row's basis section in the evidence (`As_m2`, `Z_m3`, `J_m4`, `ro_m`). |
| `STRESS_BASIS_TWIN` (the reduced check equals the nominal one, values equal) | The reduced check shares recipe, pipe, location, component, functional index, functional id and action with its nominal twin, bit for bit. Its parameters are the reduced section's (above), and its value is recomputed by the existing rule. Equal values follow only when the sections are equal. |
| Composite maximum and headlines | Per basis, against that basis's section (the readers' existing per-basis lookups key on `section_basis`). |

The private normalized payload is not readable by the readers, so they make no check on `reduced_recovery_section` or `shear_area_bits`. Those are bound by `normalized_source_sha256`, by the closure checks in the producer, and by the test-side declared-problem oracle of section 5. The readers still do not re-derive the exact solve, the closure or the charges (RF §7). The verifier stays independent and spawns nothing. `tests/test_verifier_independence.py` is unchanged.

### 3.5 Work accounting

The D5/MJ9 law (b7-plan, "D5/MJ9's accounting clarification") governs. No charging rule changes. Each delta is whatever the existing charging sites charge, and nothing is substituted for a charge the code does not make.

**Baseline.** The comparison is a newly admitted document against its Euler–Bernoulli, zero-allowance twin: the same request with `shear_model` and the allowances changed. Two kinds of work are kept apart.
- **Method work** (`prepare_sources`, the exact Context, the solve, the functionals, replay) runs under RF's metered `Work::mul` law unchanged. It changes wherever the declared matrices change, because a Timoshenko K_local has other values, and metered charges follow the actual expansion lengths. These deltas are measured, not predicted, and the oracle of RF §5.4 (`metered_mul_charges_match_the_independent_oracle`) and the charged ≥ counted control cover Timoshenko attempts too.
- **Finalization and publication work** follows from the actual sites:
  - ΔE is the encoded request's byte delta. The shear-model token and nonzero allowance values change the request bytes.
  - ΔF_k comes from `finalization_size` (`source_receipt.rs` about 409–466). That function counts 12 × the source-identity string length, the source dimensions, descriptor lengths and row identity, kind, entity, unit and metadata lengths. It does not count the serialized normalized-source payload.
    - The identity string is `serde_json::to_string(&(names, bits))` (`source_recovery.rs` about 748–758), with every scalar written as its decimal u64 bit pattern, so its length depends on values. A present mill tolerance serializes to about 19–21 characters where zero serializes to 1; a mill-tolerance quantity enters only when present. *As built:* the corrosion allowance is not an identity operand, so an allowance-only change gives ΔI = 0. A Timoshenko K_local's entries are identity scalars with other digit counts, and an exact zero at φ = 2 shortens it.
    - Descriptor product lengths change wherever a structural zero drops a term.
    - So ΔF_k = 2·(12·ΔI + 12·ΔE + ΔD), with ΔI the identity-length change, ΔE the encoded request's byte change and ΔD the descriptor-term change (as built; the encoded-request term was missing here). For example, an explicitly present mill tolerance changing from 0.0 to 0.001 adds 18 identity bytes, and so 432 to F_k. The test derives ΔF_k from the actual serialized identity and inputs and asserts equality, not zero. Published rows and their metadata lengths do not change.
  - ΔQ_k includes the first `retained_identity_digest` reservation, which charges a separate 12 × identity length: 216 more in the same example. It also includes the second basis's endpoint maximum. `composite_member_maximum` charges `RECIPE_WORK` per call (`composite.rs` about 172), and the second basis is a second call. Each later call also reads the cached identity digest, which charges `charge_finalization(128)` (`source_recovery.rs` about 233–234). The maximum is computed at two sites, and each gains a reduced-basis call: the publication loop (`lib.rs` about 6157) and the composite receipt's expected-rows pass (`source_receipt/composite.rs` about 785). So ΔQ_k = 2 × (RECIPE_WORK + 128) = 8,448 per member whose sections differ, plus each call's own metered work in `maximum_inner`. The test counts the calls at both sites rather than assuming one. `composite_member_maximum(input, selected, member_id)` (`composite.rs` about 165–170) has no basis argument today, and `maximum_inner` reads the built nominal section. So the reduced-basis call takes the reduced section explicitly, and the test asserts that the second call's evidence is bound to the reduced section's bits. Calling twice on the nominal section cannot satisfy it. Composite-exact wrapper terms depend on row counts, and none moves.
  - ΔP = 12·Δ(diagnostic bytes). No published row or diagnostic is added, so ΔP = 0 for the Euler–Bernoulli twins (distinct bases, round-away values). *As built:* a Timoshenko twin can have ΔP ≠ 0, because the existing diagnostics that state measured numbers (the ordinary report, the selected summary) change text length.
- **Method work, as built.** Where the source identity changes, method work moves by 5·ΔI: 2·ΔI in the live attempt, 2·ΔI in the captured replay and ΔI at the binding check, through `source_preflight`'s identity-length charge. For the Euler–Bernoulli twins that is the whole method change; for a Timoshenko twin the method work is measured, and the binding-check change equals ΔI where the descriptor terms are unchanged.

An independent test (`o4_source_work_deltas_follow_the_d5_law`) derives every term from the actual inputs and sites, as the MJ9 test does, and reconciles their sum with the counters exactly; it asserts the derived ΔP, including ΔP = 0 for the Euler–Bernoulli twins. MJ6's and MJ9's twelve Euler–Bernoulli requests keep their work counters in both modes; their bytes move only in the limitation strings and the hashes binding them (4.9). The 8,000,000 and 64,000,000 limits are unchanged, and an input at the limit still refuses by name.

### 3.6 What stays

- **RF's census classification is unchanged.** The J7 RF census re-authors every case to explicit Euler–Bernoulli with zero allowance (RF §10 item 1). Its classification, counts, first causes, selected members, frozen-row verdicts and work do not move. Its published envelopes, and the J7 join's consequence fixtures (`j7_t4_consequences.rs`) that bind them, change in the limitation strings, so their `envelope_sha256` pins are regenerated through `SWBPIPE_J7_RF_CENSUS_WRITE=1` (`fixtures/results/j7_rf_census_expected.json`). The census hashes `results`, `summary` and the other named members separately from the whole envelope (`j7_rf_census.rs` about 333–340), and the limitation strings live in the envelope's formulation basis (`exact_admission.rs`), so `results_sha256` and `summary_sha256` do not move, and no `public.identity` moves; epoch 4's moved for its rename. Any other member digest that moves must be shown to carry the strings. RF §6 requires that regenerated pins be compared entry by entry, with every difference in a stated class; the class here is the limitation text and the hashes that bind it. U6 regenerated the pin for its own text and row changes, and epoch 4 for its identity rename and U4b rows (E4-12). AC-002, in the owner's text of 2026-10-19 that #1461 applied, governs this: "the RF census classification, counts, first causes, selections, charged work and frozen-row verdicts are unchanged; its envelope, results and summary digests change only as a stated product-text, identity or published-row change requires". O4's limitation-text change is a stated product-text change; the census's classification, counts, first causes, selections, charged work and frozen-row verdicts stay unchanged, and `o4_text_only_envelope_diff` (4.9) enforces that (section 11).
- **`load_reference_fallback_uz` keeps its role.** It stays an ordinary pressure-1 raw (pressure-2 after epoch 4) through its zero member load's non-nodal decline, which O4 does not touch (W1 stage 2 §8; RF §5.5).
- **D7's connector and bend declines** keep their text, order and work.

## 4. W1-E (`physics-retained-1`)

### 4.1 Admission

- **`namespace_branch`.** `contract_is` admits `{version "3.0.0", mode "exact_pressure_v3", shear_model ∈ {euler_bernoulli, timoshenko_annulus_energy_v1}}`. An absent or unknown shear model stays `PressureContract`. `FamilyFact::ShearModel` is deleted.
- **`family_clauses`.** The `SectionAllowance` refusal is deleted.
- **Unchanged:** every other D1 clause, including `ReferenceConfigurations`, the expansion laws, the modulus-basis and load facts and the caps. The pricing tool's admission pins (`base/batch5_model.py`, `base/b7a_domain_model.py`) re-pin to the narrower refusal set under D3.

### 4.2 The retained Timoshenko member (kernel)

**Representation: a member-only sidecar.** Preparation records `size_of::<StraightMember>()` as `RequestedCopyBytes` for every member (`retained_product.rs` about 5554), and the receipt serializes adapter counts (`retained_wire.rs` about 1415). Growing `StraightMember`, `PreparedAssociation` or any other receipt-counted layout would move every route-L and route-E receipt. So, following stage 2's member-load sidecar (w1-member-load-certification.md §2.1, §6.2):
- `StraightMember`, `PrimitiveSource`, `SourceParts`, `PreparedAssociation`, the preparation trace entries and every adapter-reserved slot keep their layouts.
- A validated `TimoshenkoLeafSet` owns `{member, shear_area}` for each Timoshenko member, sorted by member. A borrowed view pairs it with the source, as `MemberSourceRef` pairs member loads.
  - It exists only for a Timoshenko document. Its allocation and every move are reserved and counted truthfully where it exists.
  - An all-Euler–Bernoulli invocation has no new allocation and no new adapter event.
- `StraightMember::from_frame` keeps refusing a Timoshenko leaf. A new constructor builds the member together with its sidecar entry from a Timoshenko frame. The retained assembler refuses a Timoshenko document's member that has no sidecar entry, and a sidecar entry without its member, as an association error.
- `RETAINED_TIMOSHENKO_REFUSAL` stays as the text of that association path. It no longer declines a document.
- Kernel-internal layouts that no receipt records may grow; `MemberOperators` and `BoundedCoefficients` (its `bounded()` and `widen` copies) gain the per-plane Timoshenko entries.
- **Receipt-recorded layouts stay as they are.** These include:
  - the adapter's `RequestedCopyBytes` and `RustCapacityBytes` types (`StraightMember`, `PreparedAssociation`, `ProductMemberFacts`, the case slots);
  - the proof's serialized `capacities` arrays;
  - the kernel structs recorded through `TraceCopyWork::record` (`LaneWorkTrace`, `ProductProofTrace`, `CoverageFacts`, `ProductFailureView`; `final_case.rs` about 205–405, `source_residual.rs` about 116). These bytes appear to be checked for exactness without being serialized (`retained_receipt.rs` about 23–27). Pinning them is conservative; the review may drop the constraint if it confirms they never reach a receipt;
  - `SectionPreparationWork.layout_bytes` (`SectionPrepFrame`, `PreparedAnnulus`; serialized at `retained_wire.rs` about 1444);
  - `ProductRecipe`.
  
  The implementation lists every layout a receipt records and pins each one (`timoshenko_sidecar_preserves_receipt_layouts`; stage 2's `member_sidecars_preserve_nodal_adapter_capacity_bytes` precedent). Any O4 data that would grow one goes into a sidecar instead.

**The prepared shear-area law.** W1 preparation replaces each member's area, second moments and torsion constant with the prepared, correctly rounded annulus values (`retained_product.rs` about 5520–5566). `apply_exact_section_overlay` then writes those prepared values into the evidence. G7's pressure-2 base reader requires `shear_area == κ × nominal.area` bit for bit (`section_basis.rs` `beam_theory`, about 356–369).
- The ordinary leaf A_s = RN(κ̂·A_ordinary) (PP `lib.rs` about 7990) can differ from RN(κ̂·A_prepared) by an ulp. Codex's round-1 check found such a pair at D = 0.168 m, t = 0.007 m, ν = 0.3.
- So the prepared Timoshenko member takes **A_s,prep = RN(κ̂·A_prep)**, by FK's own `shear::shear_area(κ̂, A_prep)`, with κ̂ FK's binary64 factor from the nominal radii and ν, unchanged.
- The sidecar carries A_s,prep. The preparation association records the old and new shear leaves beside the existing association, in the sidecar's own record.
- The evidence overlay writes `beam_theory.shear_area_m2` = A_s,prep with the prepared nominal area, so G7's existing equality holds.
- K4STF, the admitted lane and the section-5 admitted readout all use A_s,prep.

**K4STF and K4SRC.** A Timoshenko block is present only when the sidecar is non-empty. Such a source takes encoding version 2 (the sixth byte `0x02`, `TIMOSHENKO_VERSION`), and the block follows the count-delimited stiffness part as a framed record: the tag `TSH\x01` (`TIMOSHENKO_BLOCK_TAG`), a u32 count n ≥ 1, then n entries of (member u32, shear-area bits u64). An all-Euler–Bernoulli source, or one with an empty sidecar, keeps version 1 and today's K4STF and K4SRC bytes. *As built:* the version byte and the tag were added after independent review found that an untagged block made K4SRC non-injective (a Timoshenko source and an Euler–Bernoulli source with a crafted load list encoded to the same bytes); the version split and the framing make the encoding injective over (source, sidecar), and a regression test pins the colliding pair.

**Formation at the working precision** (`assemble.rs` `form_member`; at p, q, 2p and q_W, wherever the member is re-formed). The frame, B and the axial and torsion coefficients are today's. Per bending plane, with c = the existing `bend_z` or `bend_y` = RN(RN(E·I)/L̂):
1. s = RN(G·A_s): one exact product, rounded once;
2. sl = RN(s·L̂);
3. t₃ = RN(3·c);
4. φ = RN(4·t₃ / sl): 4·t₃ is an exact scaling;
5. op = RN(1 + φ);
6. c_s = RN(t₃ / op);
7. d₁ = RN(c + c_s), d₂ = RN(c_s − c).

D's bending rows become [(2, d₁), (3, d₂)], [(2, d₂), (3, d₁)] per plane. This is the exact identity (EI/(L(1+φ)))[[4+φ, 2−φ],[2−φ, 4+φ]] = (EI/L)·[[1,−1],[−1,1]] + (3EI/(L(1+φ)))·[[1,1],[1,1]]: the antisymmetric mode keeps EI/L, and only the symmetric, shear-carrying mode softens. φ = 4t₃/sl is 12EI/(G·A_s·L²).
- **φ = 0 is not this recipe.** t₃ = RN(3c) is inexact. An Euler–Bernoulli member keeps today's exact `4c`/`2c` scalings, code path and bits; the branch is taken from the sidecar.
- **d₂ may be zero or negative** (φ ≥ 2). It is a signed entry, and no range check treats an exact zero as underflow (U8 §3.3).

**The absolute operand D̄.** For a Timoshenko member, the absolute D operand is **D̄ = d̄·[[1,1],[1,1]] per plane, with d̄ = RU(λ·(ĉ + ĉ_s))** and λ = 2, formed from the computed operands. The factor is what makes section 4.4's dominance lemma hold, and section 4.4 derives it.
- D̄ replaces |D| in every consumer of the absolute bending block:
  - `assemble.rs` `bounded_block` (Ā, which feeds N_u and t₁, and reactions in E);
  - `verify.rs` `formation_scale` (about 164);
  - `BoundedCoefficients` and its copies;
  - native `recover.rs` (about 344), which forms Q = D·e with the signed d₁ and d₂;
  - the product certificate's lanes (4.3).
- Euler–Bernoulli members keep [[4c,2c],[2c,4c]] everywhere.
- D̄ is conservative: near φ = 0 it is about four times Euler–Bernoulli's off-diagonal and twice its diagonal. More Timoshenko cases may fail closed in R7 or the gate.
  - It also reaches the p = 512 floor: E → `resolution_scale` → ê → Φ = 2^-438·ê (`verify.rs` about 432) → the scale floor in `check_intervals`. For a near-zero body the published scale S can grow up to about fourfold. That gives fewer relative rows, larger absolute bounds RU(2^-64·S), and a looser S term in the gate's allowance.
  - All of this is sound under the unchanged laws. Section 5's premise census counts the bodies where the floor is active against their Euler–Bernoulli twins.
  - A tighter entrywise operand would need its own derivation and is not adopted here.

### 4.3 The product certificate's lanes

The lanes certify every published row against both physical readouts (REQ-001: "The bound holds for both of the formation's stiffness readouts"). For a Timoshenko member each lane forms its own shear rigidity:
- **admitted_k:** K_s = G·Â_s,prep, the exact product of the binary64 shear modulus and the prepared shear-area leaf (4.2).
- **annular_source:** K_s = g·κ_E·A_exact, from directed 1024-bit enclosures of
  κ_E = 6(1+ν)²(1+m²)² / ((8ν²+14ν+7)(1−m²)² + 48m²(1+ν)²), with m² = r_i²/r_o² formed exactly from the lifted D/2 and the nominal wall, the material's ν (the exact E/ν operand, `product_certificate.rs` about 464), the lane's exact annulus area A and its existing g = E/(2(1+ν)).
  - The denominator is positive for every real ν: 8ν² + 14ν + 7 has a negative discriminant, and 48m²(1+ν)² ≥ 0. So the enclosure never straddles zero.
  - This is the exact-geometry counterpart of the binary64 leaf, as the lane's A, I and J are of theirs.

Each lane forms φ, d₁ and d₂ from its own enclosures with directed arithmetic, **at an enclosure of the exact member length**. It uses them wherever it uses D today: lane matrix-vector products, recovery Q = D·e, end actions and stations.

**D-entry gaps, at the exact length.** The alpha law's η and the bridge's majorants bound K_G − K_K by B̄ᵀ·(coefficient gap × fixed pattern)·B̄. The current code divides each coefficient gap by a lower bound on the length: `source_residual.rs` about 558–563 (`dd / lower_length`), and `bridge.rs` about 226–250 (`coefficient_differences / ell`, with ell = max_k|Δx_k| ≥ L/√3). That is valid for Euler–Bernoulli because each entry is a coefficient divided by L, which is monotone in L. A Timoshenko gap is not monotone in L.
- The round-1 review gives Δd₁ = 3EI·L·(k_K − k_G)/((L² + k_G)(L² + k_K)), with k = 12EI/(G·A_s). That gap is larger at L than at a smaller ℓ for some members.
- So for a Timoshenko member, |Δd₁| and |Δd₂| per plane are the outward maxima of |d_G − d_K| over the two lanes' enclosures formed at the **exact-length enclosure**, never by dividing by `lower_length` or `ell`. They enter B̄ᵀ|ΔD|B̄ in place of the pattern term, and B̄'s 1/L entries keep today's bound.
- Soundness: the entrywise triangle inequality |Bᵀ ΔD B| ≤ B̄ᵀ|ΔD|B̄ holds for any ΔD. No new premise enters; ‖A_K^{-1}‖∞ ≤ β_b is the alpha law's own.
- The bridge's d̄ majorant (`dg`) may use ℓ, because d̄ decreases in L: d/dL[EI/L + 3EI·L/(L² + k)] < 0, since max over x of 3x(1−x)/(1+x)² = 3/8 < 1.
- Euler–Bernoulli members keep the coefficient-gap computation, its visits and its bits.

**Not involved:** the N06 congruence radius law applies only to DEF-O's preview proof on route L, and route L carries no Timoshenko member. DEF-E keeps the alpha law alone.

### 4.4 The dominance lemma and the remaining obligations

These are derivations under unchanged laws. If one fails, the epoch returns to HELP_HUMAN with the concrete conflict (AX-005: "do not change the law to obtain a pass"). Let ε = 2^-q at the formation precision, and g = g_m ≥ 1 the member's Gram amplification under the existing g-check (gε ≤ 2^-16).

**What the lemma does.** It does not re-derive the existing warrants' totals. It shows that every existing Euler–Bernoulli derivation holds unchanged, term by term, when a Timoshenko member's D replaces Euler–Bernoulli's D. The derivations in question:
- R7's t₁ = 2^(7−q_W)·B_b·N_u;
- stage 2 §4.2's member-recovery bound (47.1);
- the reaction, support-component and support-magnitude bound (68/(1 − 68·2^-16)² < 68.142 < 69, with E's own rounding and the higher-order terms);
- E's factor 69 under P − 16.

**How each derivation uses D.** It uses D only through two things:
- **(i) an absolute operand built from |D|:** Ā = g·B̄ᵀ|D|B̄ (`bounded_block`), E's Q̄ = |D|·ē (`verify.rs` `formation_scale`), and the |D| terms in the recovery recurrences;
- **(ii) an error coefficient c_D:** the derivation's bound |D̃ − D| ≤ c_D·ε·|D| entrywise for the formed D. Stage 2 §4.2 states its value for section terms: "Reverse coefficient counts are 4 for L, 5 for 1/L and 6 for section terms", so c_D = 6 there.

Every other stage is the same code for both theories:
- the frame, B and its g-amplified axis errors;
- the DB and BᵀDB sums, assembly, recovery of e, Q's products;
- end actions, stations, reactions (K·u − f), support components and magnitudes;
- E's staging and the higher-order recurrences.

None of those reads D except through (i) and (ii).

**Notation.** Exact quantities are starred (c\*, c_s\*, d₁\*, d₂\*, formed from the exact leaves and the exact length L); computed ones carry a hat. x̂ = x\*·π_k means that the ratio π_k lies in [(1 − ε)^k, (1 − ε)^-k], for a count k that may be fractional. Each rounding contributes one, as (1 + δ) or 1/(1 + δ) with |δ| ≤ ε; a correctly rounded square root halves its argument's count. Ratios multiply and divide by adding counts. |π_k − 1| ≤ ρ_k = (1 − ε)^-k − 1 = kε + O(ε²). This form is exact for fractional counts, which Higham's γ_k is not. It is at least as strong as R7's Lemma B factor form (first-order count × at most 1 + 2^-8.8).

**The coefficient counts of the two derivations.** Both form c as `over_length` does (`assemble.rs` about 340–355): ĉ = RN(RN(E·I)/L̂), one rounding for the product, one for the quotient, plus the length's count ℓ. So c's count is c_D = ℓ + 2.
- **The length, from the code.** `form_member` (`assemble.rs` about 296–306) rounds each coordinate difference once from exact binary64 sums. `normalize` and `dot` (about 190–224) accumulate the three products exactly in `ExactWideSum` and round once, then take one correctly rounded square root. So L̂/L lies in [(1 − ε)^2.5, (1 + ε)^2.5]: the squared differences carry 2, the sum's rounding 1, the root halves those 3, and its own rounding adds 1. This is ℓ = 2.5 in the exact form above, not only at first order. Obligation 3 confirms this DAG, and it is load-bearing for R7's margin.
- R7 §3.3: "1/L; the section terms | 3.5; 4.5", so ℓ = 2.5 and c_D = 4.5, matching the code.
- Stage 2 §4.2: "Reverse coefficient counts are 4 for L, 5 for 1/L and 6 for section terms", so ℓ = 4 and c_D = 6. Taking ℓ = 4 is conservative against the code's 2.5.

**Timoshenko D entries, finitely.** ĉ = c\*·π_{c_D} and L̂ = L·π_ℓ.
- t̂₃ = 3c\*·r, with r = π_{c_D+1}.
- ŝl = s\*·L·π_{ℓ+2}: G·A_s rounded once, L̂, and the product rounded once.
- φ̂ = φ\*·r·u⁻¹, with u = π_{ℓ+2}/(1 + δ_φ) = π_{ℓ+3}, where δ_φ is φ's own rounding.
- The ratio of computed to exact before c_s's last two roundings is f = r(1 + φ\*)/(1 + φ\*·r·u⁻¹) = (r + r·φ\*)/(1 + r·φ\*·u⁻¹). That is the mediant of r/1 and (r·φ\*)/(r·φ\*·u⁻¹) = u, so f lies between r and u exactly, at every φ\* > 0. Hence f = π_m with m = max(c_D + 1, ℓ + 3) = ℓ + 3, since c_D = ℓ + 2. This is the exact form of the convex-combination remark, and it is why the bound does not grow with φ.
- ĉ_s = c_s\*·f·(1 + κ)/(1 + ω) = c_s\*·π_{ℓ+5}, with κ and ω the roundings of c_s and op.

So, with k = ℓ + 5 (7.5 in R7, 9 in stage 2):
- **d₁** = RN(ĉ + ĉ_s), both positive: d̂₁ = (c\* + c_s\*)·π_{k+1};
- **d₂** = RN(ĉ_s − ĉ): |d̂₂ − d₂\*| ≤ ρ_k(c\* + c_s\*)(1 + ε) + ε|d₂\*| ≤ ρ_{k+1}(c\* + c_s\*), because |d₂\*| ≤ c\* + c_s\* and (1 + ε)(1 − ε)^-k ≤ (1 − ε)^-(k+1);
- **the operand** d̄ = RU(2(ĉ + ĉ_s)) ≥ 2(ĉ + ĉ_s) ≥ 2(1 − ε)^k·(c\* + c_s\*). It is formed from the computed operands and rounded upward, so it dominates.

**The entrywise bound in each derivation's own units.** For every Timoshenko D entry,
|d̂ − d\*| ≤ ρ_{k+1}·(c\* + c_s\*) ≤ [ρ_{k+1} / (2(1 − ε)^k)]·d̄ ≤ c_D·ε·d̄, and |d̂|, |d\*| ≤ (1 − ε)^-(k+1)·(c\* + c_s\*) ≤ d̄.
- **Against the computed operand.** The bound is against the computed, upward-rounded d̄. That is the form stage 2's recurrence requires ("If a coefficient obeys |C\* − C| ≤ cε|C|", with C the computed coefficient; §4.2). It also gives R7's Lemma B form, since c_D·ε is no larger than the first-order count times 1 + 2^-8.8.
- **Checked.** Both inequalities were evaluated with 400-digit decimal arithmetic at ε = 2^-53, 2^-64, 2^-113 and 2^-128, for both chains. Both sides scale so that a smaller ε only helps.
- **The margin.** The left side is about (k + 1)/2 = (ℓ + 6)/2, which is below c_D = ℓ + 2 whenever ℓ > 2:
  - R7: ρ_{8.5}/(2(1 − ε)^7.5) = 4.25ε(1 + O(ε)) ≤ 4.5ε;
  - stage 2: ρ_{10}/(2(1 − ε)^9) = 5ε(1 + O(ε)) ≤ 6ε.
- **The case it depends on.** At ℓ = 2 the left side's first order equals c_D, and the higher-order excess would break the inequality. So R7's margin rests on ℓ = 2.5 being the exact count, as derived from the code above.

In words: with λ = 2, each derivation's own section-term count, c_D, bounds every formed Timoshenko entry relative to the absolute operand D̄. It bounds an Euler–Bernoulli entry relative to |D| in the same way. No count changes.

**The lemma.** Each derivation reads D only through (i) its absolute operand and (ii) the entrywise error bound |D̂ − D\*| ≤ c_D·ε·(absolute operand), in its own exact form (obligation 3 confirms this). Replace |D| by D̄ in (i), and use the bound just derived for (ii). Then every term of every derivation is bounded by the same term of its Euler–Bernoulli form with |D| replaced by D̄:
- a D-error term is at most c_D·ε·D̄, which is the term the derivation already carries;
- a term multiplying |D| by another stage's error is at most that error times D̄;
- the rounding terms do not involve D.

The D count is supplied in the same exact form the derivations already assume for their stage counts. So each derivation's own remainder treatment applies unchanged: R7's Lemma B factor 1 + 2^-8.8 under g ≤ 2^(P−16), and stage 2's (1 − 68·2^-16)^-2. Every total, and so every law's constant, holds for Timoshenko members with Ā, E and Q̄ formed from D̄. In particular:
- R7's t₁ holds with N_u from Ā_T = g·B̄ᵀD̄B̄;
- stage 2's 47.1 and the reaction and support 68.142 hold;
- factor 69, P − 16 and 2^7 are unchanged.

**The choice of λ.** **λ = 2** is derived, not provisional. It needs (ℓ + 6)/2 < ℓ + 2 with the exact remainder, which holds for both chains as stated. A derivation with ℓ ≤ 2 would need λ > (ℓ + 6)/(ℓ + 2). Neither existing derivation has one: the code's length count is 2.5, and stage 2 uses 4. A larger λ is more conservative still (4.2's note on D̄).

**A sanity reconstruction, not the warrant.** Counting `assemble.rs` directly puts an Euler–Bernoulli member's formed K entries within about 67–68ε·Ā: B within 30gε, DB within 36gε, BᵀDB within 67gε, assembly rounding ε. That is comfortably inside t₁'s 2^7. It is not offered as the reaction and support count; that warrant's later stages (reaction and support rounding, magnitudes, E's rounding) are covered by the substitution argument above, not by this count.

**Remaining obligations** for the independent numerical review, before implementation code:
3. Confirm, for each existing derivation (stage 2 §4.2; R7's t₁; the reaction and support derivation):
   - that it reads D only through (i) and (ii);
   - that it uses the D error entrywise against its absolute operand, and does not use the Euler–Bernoulli fact that both D entries share c's single relative error. That fact correlates the Q₂ and Q₃ errors; Timoshenko errors are not proportional across d₁ and d₂;
   - that its total is non-decreasing in its absolute operand;
   - that its length count ℓ and section count c_D are those quoted above (2.5 and 4.5 for R7; 4 and 6 for stage 2), and that c_D = ℓ + 2 at the code's `over_length`. The length DAG at `form_member`, `dot` and `normalize` is load-bearing for R7's margin: one rounding per coordinate difference, exact products with one rounding of the sum, and one correctly rounded square root.
   
   Also recheck the finite D-entry derivation and the high-precision check of ρ_{k+1}/(2(1 − ε)^k) ≤ c_D·ε.
4. The alpha law with D-entry gaps at the exact length (4.3): its premise, the triangle inequality and the outward rounding of |Δd|.
5. κ_E's enclosure contains the exact κ_E of the exact annulus for every admitted ν in (−1, ½). Every intermediate stays within the lane's `Wide` range, or refuses by `ExponentRange`.
6. K4STF's Timoshenko block and the sidecar bind every leaf that enters formation, so no attempt, group or replay can certify a member whose formation differs from its declared leaves.
7. Exact or directed controls measure the actual error against each warrant at P = 128, 256, 512 and 1024: formed K against t₁'s allowance, member recovery and reactions, support components and magnitudes against 69·2^-P·E_q. They cover φ = 2 − 2^-40, 2 and 2 + 2^-40, φ ≈ 1e-6 and φ > 10, skew members, and g at P − 16 and above. The D̄-inflated path through E, the p = 512 Φ floor, body scales, classes and absolute bounds is exercised explicitly.

### 4.5 Two section bases on route E

- **Section facts (a sidecar too).** Capture reserves `facts` through the adapter (`retained_product.rs` about 1915), which records capacity × `size_of::<T>()` (about 4127–4133). So `k::ProductMemberFacts` (`final_case.rs` about 84–94) must not grow. The reduced section lives in a `ReducedSectionSet` sidecar, reserved and counted only where a member's bases differ, beside `TimoshenkoLeafSet`, with no tag added to `ProductRecipe`. Each member has a reduced section exactly when the member's reduced stress section differs from the nominal one in any of **{As, Z, I, J, effective wall}**. The wall is in the set because the annular lane reads t_r: two sections whose binary64 A, Z, I and J agree can still differ as exact annuli.
  - The reduced facts are binary64 A_r, Î_r, Ẑ_r and J_r, prepared by `prepare_product_annulus(D, t_r)`; for the annular lane, the exact annulus of D and the binary64 t_r = (t − c_m) − c_a.
  - The nominal facts are today's, with the nominal wall t. With a nonzero mill tolerance the stiffness, κ_E and the nominal stresses use t; today's route E, which admits only zero mill tolerance, already uses t.
- **How the kernel identifies a reduced row.** `ProductRecipe::Stress { member, site, stress }` has no basis field, and `row_scales` sizes stress coverage as `facts.len() × 21` (`final_case.rs` about 1153–1167). Its capacity (`capacities[4]`) is serialized (`retained_wire.rs` about 1380). So:
  - `ReducedSectionSet` carries, for each member whose bases differ, its reduced section and the final-row indices of its reduced stress and maximum rows.
  - The kernel reads a row's basis from that map, and uses that member's reduced section for it.
  - Reduced rows are covered in a separate reduced-coverage vector, allocated only when the set is non-empty.
  - `ProductRecipe`, the 21-slot coverage vector and `capacities[4]` are unchanged for bit-equal documents. A const size assertion pins `ProductRecipe`.
  - A reduced row whose index is not in the map, or a mapped index whose row is not a reduced stress or maximum row of that member, is refused by its kernel cause.
- **Reduced stress rows.** Each reduced stress row is certified by the same recipes as its nominal twin: DEF-E's directed signed-corner N/A, M_y/Z, M_z/Z and T·c/J, with the admitted lane's represented Z = hull(Ẑ_r, Î_r/c). The two-end maximum recipe is likewise applied with A_r and Z_r. Each row gets its own enclosure, class and bound under the unchanged gate. Its body scale is its body's force and moment scales divided by its own section (the reader's `row_scale`).
- **The overlay stays for bit-equal sections.** Where all five agree, the reduced twin is today's overlay: excluded from the receipt row universe and carrying its nominal twin's retained value. Two such sections give the same recipe and the same operands in both lanes, so the twin is the same certified quantity.
  - The rule is decided per member from the data, not by a document flag. A mixed document certifies distinct-basis members' twins and overlays the rest.
  - Existing route-E successors (zero allowances) keep their row universes, numbers and work.
- **The producer's twin predicate becomes per member.** Today `reduced_twin` and `reduced_twin_wire` (`retained_product.rs` about 5654–5659) decide by the row's metadata alone: on route E every reduced-basis stress or maximum row is a twin. After O4 a row is a twin exactly when it is a reduced-basis stress or maximum row **and** its member has no `ReducedSectionSet` entry. Every consumer reads that one predicate, so the bound positions stay consistent:
  - the capture binding and its `check_reduced_twin` value equality (about 2447);
  - `bound_row_indices` (about 5681), which maps published rows to kernel rows;
  - `prepared_not_covered`'s `bound` counter (about 5987), which must equal the kernel index `complete_not_covered` checks (`NotCovered { row }`);
  - `apply_prepared_overlay` (about 6276);
  - the receipt's `row_ids` (`retained_wire.rs` about 2176).
  The metadata validation that fixes `SectionBasis::Nominal` for every route-E `Stress` row (about 2664–2666) admits the reduced basis for a distinct-basis member's reduced rows. A consumer left on the metadata-only predicate fails closed (a twin-value refusal or "not-covered completion identity"), so the case never selects; section 6 pins each site.
- **PP's staging.** `apply_exact_section_overlay` writes each basis's prepared section to its own evidence object; today it writes one prepared section to both. It also writes the prepared shear area (4.2). `stage_headlines` already selects each basis's maximum rows.
- **U4b's derived rows on route E (the reliance epoch 4 invalidated).** Epoch 4 built `prepared_not_covered` for coincident bases only (E4-3): its operand table has no basis, a second `Stress` operand for the same member, site and slot refuses as `"derived operand identity"`, and τ's c_b reads the member's nominal facts. Once a member's reduced stress rows are receipt rows, that would refuse every distinct-basis case, and a reduced τ would use the nominal annulus. So:
  - today's operand table, one `[[Option<usize>;5];5]` per member, keeps its type, reservation and adapter events. Beside it, a reduced operand table holds one `[[Option<usize>;5];5]` per distinct-basis member, indexed through `ReducedSectionSet`, and is reserved through the adapter only when the set is non-empty. The adapter counts `capacity × size_of::<T>()` into the serialized receipt counters (`retained_product.rs` about 4127–4133; `retained_wire.rs` about 1415), so growing today's table would move every existing receipt, route L included;
  - a bound `Stress` row is filed in the reduced table exactly when `ReducedSectionSet` maps its final-row index to that member's reduced rows, and in today's table otherwise. A bit-equal member files only nominal operands, as today; its reduced derived rows read them, which is today's behaviour;
  - each derived row reads the reduced table exactly when its metadata names the reduced basis (the `section_basis` suffix, the same token the source receipt and `u4b_evidence.rs` match) **and** its member has a `ReducedSectionSet` entry; otherwise it reads today's table. So a bit-equal member's reduced derived rows read the nominal operands, as today;
  - τ_v,max of a distinct-basis member's reduced row (same rule) takes c_b from the reduced facts: r_i = D/2 − t_r and Î_r, with the member's captured ν, as DEF-E's `rows.derived_not_covered` states ("c_b from the regenerated basis section");
  - the shear operands stay basis-free, as on the other routes;
  - the rows stay `ProductRecipe::NotCovered`, with no lane, certificate, scale, bound or coverage slot. O4 adds no recipe;
  - only an unknown basis token, or an empty operand cell under that rule, refuses, with today's refusal text.
  A document with no distinct-basis member takes exactly today's path: the same table, values, bytes and adapter events (`timoshenko_sidecar_preserves_receipt_layouts`, 6). The reduced table is a new priced owner (7).

### 4.6 Receipt and SCHEMA (reviewed input 2)

`retained_precision_mp_v2.schema.json` has two closed section shapes, and both change:
- **`$defs/Section`** (the `sources[].section_terms` items) gains `shear_area` (Bits), present exactly on a Timoshenko member, and `reduced` (closed: the same keys as the nominal source section term), present exactly when the member's sections differ (4.5).
- **The inline `cases[].selection.section_terms` item** (about 5274; `member_id`, `area`, `section_modulus`, `length`, `axial_stiffness`, `torsional_stiffness`, `additionalProperties: false`) gains `shear_area` and `reduced` {`area`, `section_modulus`} under the same presence rules. Its operational length and stiffnesses stay nominal.
- **The two encodings** are written by `retained_wire.rs`: the source terms from the sidecar and the reduced facts, the selection terms in `selection`.
  - Their member identifiers differ (the kernel member index against the authored member id).
  - Their geometry shapes differ (`geometry` {`normalized_od`, `actual_radius`, `actual_second_moment`, `actual_polar_moment`, `effective_wall`} on source terms only).
  - G5b binds source and selection to each other for the new members as it does for the existing ones (4.7).
- **The reduced preparation record.** `product_attempts[].preparation.members[]` (written at `retained_wire.rs` about 1436–1447: `result`, `work`, `conversions`) gains `reduced`, a closed object of the same shape recording the reduced annulus's `prepare_product_annulus` run, present exactly when the member's sections differ. Its work counts and conversions are the reduced run's own; the member's existing fields keep the nominal run. The readers check it as they check the member's own record.
- Receipt rows for distinct-basis reduced twins enter `absolute_verified`, `not_covered` and every row-indexed list like other rows, in published order. An all-Euler–Bernoulli receipt with bit-equal sections has no new key.

### 4.7 Readers and verifier

- **The row universe.** `retained_precision.rs` `rows_for` (about 964–966) drops every reduced-basis row by metadata. G3's coverage, G5a, `numeric_owner` and the G5 diagnostic all read it.
  - After O4, `rows_for` drops a reduced-basis row only when its member's source section term has no `reduced`. Its inputs are the row's metadata and the receipt's `sources[].section_terms`, which G1 has already bound by hash and G2 by encoding.
  - With `reduced` present, the member's reduced rows are in the universe, and row indices follow published order.
- **New refusals and where they sit.** A `reduced` present where the evidence's sections agree, or absent where they differ, cannot be decided before the evidence is read at G5b. So:
  - G3 refuses, with its existing coverage code, a universe whose rows and indices do not match the receipt's row lists. That catches a dropped or duplicated reduced row.
  - G5b refuses `SECTION_MISMATCH` where `reduced`'s presence or values disagree with the evidence's sections, or where source and selection terms disagree.
  - *As built:* a member's reduced preparation record (`preparation.members[].reduced`) is read at G5 with every rule its own record has: present only beside a prepared member, conversions in property order and of admitted kinds, nine normal positive conversions equal to its section when prepared, a refused record only on the last member, its work bound to its conversion count, and its work required exact wherever the member's own work is. Faults inside it are judged against its own work. Independent review found the first version bound only the record's section.
  - Route L: a receipt carrying `reduced` or `shear_area` in either section shape is refused at G5b `SECTION_MISMATCH`, since route L has one section and no shear model (H-1).
  - Every existing refusal keeps its gate and order. The only new outcomes are on statements that carry the new key or new rows. Precedence mutations (section 6) pin both.
- **G8.**
  - `exact_namespace` admits either shear model (4.1). It keeps the three-member contract, no top-level `shear_model`, and an explicit, finite, length-valued allowance ≥ 0 with c_m + c_a < t on every section (the producer's own refusals). `section_allowances(model, true)` stops requiring zero.
  - `exact_mill_tolerances` and `MILL_TOLERANCE_DECLINE` are deleted.
  - G8 rederives each member's nominal and reduced sections (`section_basis`), its `beam_theory` (κ̂ by U8 §3.1's fixed order) and **A_s,prep = RN(κ̂·A_prep)** from the prepared nominal area the receipt states. It binds `shear_area` in both section shapes to that value, and `reduced` to the rederived reduced section.
  - It rebuilds K4SRC and K4STF with the Timoshenko block. The B7b-ZA per-section allowance loop stays bounded by m.
- **G5b.**
  - The body-scale equality is unchanged.
  - The section check binds `reduced` like the nominal entry, when present.
  - `row_scale` uses a reduced row's own section.
  - `g5b_exact_evidence`, per member: with `reduced` absent, the evidence's two sections must agree in {As, Z, I, J, wall} and the overlay rule applies (today's check). With `reduced` present, the evidence's sections must differ in that set, `reduced` must equal the evidence's `reduced_section`, and the reduced rows are receipt rows, not overlays.
- **G5c** classifies a distinct-basis reduced row like any other stress row.
- **G7** runs the pressure-2 base reader, which already checks both bases and the Timoshenko evidence, the latter against the prepared shear area (4.2).
- **Python verifier.** It mirrors each of these with its own arithmetic, including the reduced derived rows' per-basis check in `tests/verifiers/u4b.py`, which epoch 4 already keys by basis.
- **Derived rows.** No reader change: `u4b_evidence.rs` already recomputes every derived row by (case, entity, kind, component, location, basis) from that basis's rows and section. A reduced derived row formed from nominal operands is refused there; this is the reader-side kill of section 4.5's mutant.
- **Desktop.** No TypeScript change. The desktop reads through the Rust reader (wasm), and test support takes the new DEF-E hash.

### 4.8 DEF-E, tables and texts

- **DEF-E** (`retained_precision_prepared_exact_v1.json`, reviewed input 7; sorted-key compact form kept):
  - `scope.shear_model`: "explicit euler_bernoulli or timoshenko_annulus_energy_v1 per document; straight members only (realized arcs never reach this route)".
  - `scope.corrosion_allowance` and `scope.mill_tolerance`: "explicit user-owned length on every used local pipe section, finite and nonnegative with c_m + c_a < t; the D-5 nominal section (wall t) carries stiffness and the Timoshenko shear area, and every stress row is certified on its own basis section".
  - `lanes`: section 4.2's Timoshenko D recipe and D̄, the prepared shear-area law, section 4.3's admitted and annular shear rigidities and the D-entry gap majorant at the exact length.
  - `rows`: reduced stress rows "certified on the reduced section with the nominal row's recipe where the two sections differ in As, Z, I, J or wall; the nominal row's retained value where they agree".
  - `evidence`: the per-basis prepared overlay and the prepared shear area.
  - `rows.derived_not_covered` (added by epoch 4) is unchanged: it already forms σ⁺/σ⁻ "from the same station's and basis's axial and bending stress rows" and τ's c_b "from the regenerated basis section and the member's captured nu".
  - `trust.G8`: the rederivation of both bases and the prepared Timoshenko leaf, in place of the zero-allowance and Euler–Bernoulli wording.
  - Its meaning hash moves in the producer (`retained_wire.rs`), the Rust reader, the verifier, the physics-retained table's `product_formation_definitions` and desktop test support.
- **physics-retained-1 table** (reviewed input 8): only its DEF-E hash. TBD-002's `accuracy_classification.scope` sentence is not touched; if any edit would need it, the edit goes to HELP_HUMAN first.
- **pressure-source-1 table** (reviewed input 9): epoch 4's revision (U4b kinds, the 0.6.0 token, the derived-rows limitation) adds no Euler–Bernoulli, shear-model or allowance text, so O4 does not move it.

### 4.9 Which committed bytes move

- **Limitation strings.** The limitation strings of section 3.3 move on every exact envelope that carries them:
  - every ordinary pressure-2 envelope and its corpora (`pressure_v3_{straight,arc,connector}_reader_corpus.json` among them);
  - every pressure-source-1 raw and derived document (`fixtures/product_preview/pressure_source/**`, `load_reference_source`, the stress-neutral exports);
  - J2's bases;
  - MJ6 and MJ9's captured bases;
  - J7's published envelopes (their `envelope_sha256` pins only) and the J7 join's consequence fixtures;
  - the route-E successors and RV120's exact inputs, whose projected statements carry the unselected limitation 7;
  - the retained corpus `retained_precision_cases.json`, through its route-E entries (`w1_member_route_e_baseline_*`) and the limitation string they carry;
  - the R2 records and the desktop parity inputs.
  
  Each is regenerated by its own writer. A check (`o4_text_only_envelope_diff`) requires every such regenerated envelope to differ from its pre-O4 bytes only in those strings and the hashes that bind them: no number token, row, class, first cause, selection or work counter.
- **DEF-E-bound hashes** of every route-E successor move under the E14 precedent (HELP_HUMAN, 2026-10-17: "The DEF-O hash moves under the E14 precedent are fine").
- **Nothing else moves** for Euler–Bernoulli, zero-allowance documents: the sidecars keep receipt layouts (4.2, 4.5), the overlay rule keeps row universes (4.5), and work is unchanged (3.5). Route L and its successors do not move at all. The retained corpus moves only in its route-E entries.
- **New bytes:** the Timoshenko and dual-basis J2 bases and route-E controls, with their mutations.

## 5. Independent references and judging (a later, separate reference unit)

The certificate certifies the problem the retained kernel forms. It cannot see a wrong but well-defined formation, such as a Timoshenko recipe with a wrong factor. As T4-U8 did for K-D5 (M3-T), O4's new publications are judged by an exact reference that reads no product code. That reference is a separate unit, derived and frozen by a TASK that neither writes nor reads the product implementation, and refuted by a second TASK, before any product output is compared with it (U6/U8 §7, "Independence"). It is not generated in this design PR.

**`validation/references/o4_retained_source/`** contains:
- **The engine:** the frozen `validation/references/kd5_timoshenko/kd5t_engine.py`, used unchanged. Its exact element is SHEAR_REFERENCE's, with κ_E formed exactly, D-5 bases with t_r in the fixed binary64 order, and exact A, I and J. Its solution of a request is therefore W1-E's annular readout.
- **An admitted-readout mode,** added by the new unit's generator, not by editing the engine. It solves the same model with the prepared binary64 leaves:
  - prepared A, I, J and Z per basis, each the correctly rounded exact annulus value;
  - κ̂ by U8 §3.1's fixed order, and Â_s,prep = RN(κ̂·A_prep);
  - G = RN64(E/RN64(2·RN64(1+ν))).
  
  These are formed exactly. For stress rows, the admitted readout's section modulus is DEF-O/DEF-E's represented interval hull(Ẑ, Î/c), and a row is judged against the whole interval, so the judge cannot report a false breach. REQ-001's bound is claimed for both readouts, so both judge.
- **A judged set within W1-E's and the source family's domains:**
  - straight-member exact documents with nodal loads, rigid restraints and grounded scalar springs, and explicitly empty pressure regions;
  - Timoshenko and Euler–Bernoulli twins of every design;
  - zero, mill-only, allowance-only and both-nonzero sections, plus values that round away (c_m = 2^-80·t);
  - φ from about 1e-6 to above 10, including φ = 2 exactly where representable;
  - axis-aligned and RF-style congruence (3,4,0) and (1,2,2) members;
  - designs chosen to be Sensitive on the ordinary route (soft springs at k/a ≈ 1e-8 and 1e-12, stiff contrast, short members), so that W1-E and the source method actually trigger;
  - both solver modes, typed and captured entries, single-case invocations inside D1's caps.
- **Twins of the source controls:** Timoshenko and nonzero-allowance re-authorings, in memory, of J2's twelve source requests and of RF's PIN-OFF-345 r1e-08 and r1e-12 cases. The frozen bytes stay unchanged; tests re-author in memory, the t4_u4a precedent.
- **A declared-problem oracle for the source route.** The declared operands live in the private normalized payload and functional plan (3.1), so they are read from a **test-only dump** of the actual payload and plan.
  - The dump is a `#[cfg(test)]` / dev-feature hook in `source_receipt/source.rs`, the RF probe precedent. The oracle first checks that the dump hashes to the receipt's `normalized_source_sha256` and `functional_plan_sha256`, so it judges exactly the committed source.
  - Standard-library Python (`fractions`) solves that declared problem exactly. It asserts that every published projection interval contains the exact value and every public stress check recomputes.
  - It also re-forms each Timoshenko K_local exactly from the formation inputs (E, G, A, I, J, A_s, L) and requires every declared entry to lie within a bound derived from FK's formation chain of U8 §3.3: per-step roundings, counted the way section 4.4 counts them, stated in the unit before any product output is compared. That catches a wrong Timoshenko formula without depending on FK's operation order.
  - This is RF's P1 method made a maintained oracle.
- **Discriminators,** each at least 1e3 tolerances from the correct value on its listed rows: φ with A in place of A_s, κ on the reduced section, stiffness on t − c_m, a reduced row copied from the nominal row where the sections differ, a nominal-section reduced maximum, a Euler–Bernoulli member formed in a Timoshenko document, Cowper or 5/6 for κ_E, and an annular lane that reuses κ̂.

**Acceptance (M3-O4).**
- Zero judged breaches: every W1-E `relative_verified` row lies within 1e-9·|value| (and DEF-E's sharper allowance) of both readouts' exact values, and every `absolute_verified` row within its published bound.
- Every source projection interval contains the declared problem's exact value.
- Zero strict coverage failures.
- The Euler–Bernoulli zero-allowance twins reproduce today's numbers, which validates the harness.

**Premise census.** On the judged set, the census reports R7 outcomes, alpha_b, t₁'s share of the verification budget, bodies where the p = 512 floor is active, and fail-closed counts, each against the Euler–Bernoulli twins. Fail-closed outcomes are acceptable. A judged breach on a published row that a sound repair cannot remove stops the epoch, and the epoch returns to HELP_HUMAN.

**As built.**
- *The unit.* `validation/references/o4_retained_source/` holds 172 judged-set documents (33 designs) and 104 source-control twins (13 of the 17 load cases of J2's twelve requests and RF's PIN-OFF-345 r1e-08/-12; four are out of the engine's scope: present pressure regions, modulus bases, `eigen_motion`'s line stop and motions). It was frozen before any product output was compared and refuted twice by a separate TASK sharing no code with its generator (all 136,580 written values reproduced: 111,636 judged-set and 24,944 twin values). In eight Timoshenko documents near φ = 2 the two readouts disagree on near-end bending rows by up to 6.2e9 tolerances (carry-over (2 − φ)/(4 + φ)); such rows cannot be relative-verified in both lanes. The κ̂-reuse discriminator moves only those rows, so it is killed product-side (`annular_timoshenko_lane_uses_the_exact_kappa`).
- *The declared-problem oracle.* `declared_problem/` (BOUND.md frozen before any dump existed; a later addendum states that the bound covers U8 §3.3's expressions as written and excludes rewrites that add operations, with no count changed). Four rounds of independent review removed its false passes (missing evidence, identity swaps, signs, units, duplicates). It judges 578 selected source cases, including the load/reference-state family, (3,4,0) and (1,2,2) congruence members and φ = 2 exactly: 0 FAIL, and every control fails as required. The largest K_local share of its bound is 0.572.
- *W1-E judging* (`o4_m3_judging_tests.rs`, basis `65fd80b176`; later changes touched no numerical path). 118 successors (6 judged-set and 112 source-twin, the latter through W1's own disconnected-trigger test path), 2,934 relative and 8,798 absolute rows per readout, zero breaches; the worst relative margin is 1.1e-7 of the tolerance and 0.965 of DEF-E's sharper allowance. Every covered row must be compared with both readouts or the test fails. The 182 Euler–Bernoulli zero-allowance runs reproduce main's 1,978 recorded quantities bit for bit.
- *Coverage.* On the plain binding no judged-set document publishes a W1-E successor: ordinary checks pass (242 runs), the source route selects (18), or W1-E's existing proof fails closed (SharperExact 44, alpha 24, preparation 16). The two readouts' near-ulp disagreement leaves W1-E publishing few successors for these documents; Euler–Bernoulli documents behave the same on main. No φ ≈ 2 row was published in any class.
- *Census.* On a grid of 3,142 sections × lengths × four ν (15,081,600 pairs), φ is within 1e-3 of 2 in 6.2e-5 of pairs, and no pair falls within the band where the readouts differ (|φ − 2| < 2e-7). Extrapolating the measured density near φ = 2 (about 0.062 per unit of |φ − 2|, from the wider bands) to that band gives about 1e-8; that figure is an extrapolation, not a count. All are grid frequencies, not population probabilities. On 3D skew single members with ordinary decimal inputs the source route falls back with `Exact(Budget)` at the unchanged 8,000,000 limit in 17 % of Euler–Bernoulli and 37 % of Timoshenko runs; planar members and dyadic inputs do not (driver attribution descriptive: non-dyadic bit-lengths, skew congruence, Timoshenko).

## 6. Mutants and killing tests

Each is introduced into a scratch copy, run, reverted and recorded in the PR; nothing is a committed seam. Tests are new unless marked existing.

| Fault | Killed by |
|---|---|
| A Timoshenko document still declined by `prepare_sources` or G-A | `o4_timoshenko_source_case_selects_and_publishes`, `o4_w1e_timoshenko_case_is_admitted_and_selected` (replace the existing `o4_prepare_sources_declines_by_name` and `o4_w1e_declines_at_admission`); the existing `o4_timoshenko_declines_are_reached_by_published_documents` (`timoshenko_product_tests.rs`) becomes `o4_timoshenko_documents_publish_retained_source`, asserting that the Timoshenko twin of its N05 control now selects pressure-source-1 |
| A nonzero mill tolerance or allowance still declined | `o4_dual_basis_source_case_selects`, `o4_w1e_dual_basis_case_is_admitted` |
| Retained member formed Euler–Bernoulli for a Timoshenko leaf; Timoshenko recipe applied to a Euler–Bernoulli leaf | `retained_timoshenko_formation_matches_exact_element` (P = 128/256/512/1024 against the exact element); the existing K4 vectors unchanged |
| φ with A instead of A_s, or with Iy/Iz swapped; factor 3 or 4 wrong; d₂ sign flipped | the same, plus the judged set's discriminators |
| Ā, E or Q̄ using \|D\| or c + c_s instead of D̄ = λ(c + c_s) (cancellation at φ ≈ 2) | `retained_timoshenko_formation_error_is_within_t1` and `..._recovery_error_is_bounded_by_69uE` (the section 4.4 item 7 controls, asserted against each warrant's allowance) |
| Annular lane reuses κ̂ or Â_s (no exact κ_E) | `annular_timoshenko_lane_uses_the_exact_kappa` (enclosure contains exact κ_E; κ̂ excluded on a section where they differ) |
| Coefficient-gap pattern used for a Timoshenko member's η | `timoshenko_eta_uses_d_entry_gaps` (a member where the pattern bound underestimates \|ΔK\|) |
| K4STF/K4SRC Timoshenko block missing, or present for an all-Euler–Bernoulli source | encoding tests; the existing K4 vectors unchanged; corpus `section_terms_shear_area_on_euler_bernoulli` |
| Stiffness on t − c_m; κ_E on the reduced section | `o4_w1e_stiffness_uses_the_nominal_wall`, `o4_kappa_uses_the_nominal_section` (judged set) |
| Reduced row overlaid where the sections differ; reduced row certified separately where they are bit-equal | `o4_w1e_distinct_bases_certify_reduced_rows`, `o4_w1e_bit_equal_bases_keep_the_overlay`; reader mutations `reduced_section_term_missing`, `reduced_section_term_on_equal_bases`, `reduced_row_overlaid_on_distinct_bases` |
| Source reduced check using nominal parameters; source maximum reused across bases | `o4_source_reduced_checks_bind_the_reduced_section`, `o4_source_maximum_per_basis`; J2 mutations `reduced_check_nominal_parameters`, `reduced_maximum_copied` |
| A Timoshenko source frame without `shear_area_bits`, or an Euler–Bernoulli frame with it (private payload) | `o4_source_payload_members_are_conditional` (producer test over the test-only dump); `o4_euler_bernoulli_sources_keep_their_numbers` (MJ6/MJ9's twelve requests: work and every number unchanged; text-only diff) |
| Reader admits an unknown shear model, a negative or over-wall allowance, or an absent allowance | reader unit tests in Rust, wasm and Python (`PHYSICS_SOURCE_ACTUAL_CONTRACT_UNSUPPORTED`, `PHYSICS_SOURCE_ACTUAL_CORROSION_ALLOWANCE`, G8 `RETAINED_PRECISION_INVOCATION_MISMATCH`) |
| Reader accepts a `beam_theory` or `shear_area` that is not the rederived κ and A_s | corpus and J2 mutations `shear_area_changed`, `kappa_changed` (resealed) |
| Limitation text left stating the O4 declines | `o4_limitations_state_the_extension` (pinned bytes; claims review) |
| Work delta misaccounted | `o4_source_work_deltas_follow_the_d5_law` (independent derivation from the serialized identity and the actual sites, as MJ9; asserts the derived ΔP = 12·Δ(diagnostic bytes), which is zero for the Euler–Bernoulli twins, §3.5) |
| Timoshenko D-entry gap formed at `lower_length`/`ell` | `timoshenko_gap_is_formed_at_the_exact_length` (a skew member whose gap at ℓ < L underestimates the exact gap; as built at φ ≈ 6, because at φ ≈ 1 a K_s-driven gap shrinks as L grows, so ℓ would overestimate it) |
| `StraightMember` or another receipt-counted layout grown | `timoshenko_sidecar_preserves_receipt_layouts` (size and layout pins; full adapter counters of every committed route-L and route-E receipt unchanged) |
| Shear leaf from the ordinary area after preparation | `prepared_shear_area_follows_the_prepared_area` (the D = 0.168 m, t = 0.007 m, ν = 0.3 control, where the two differ by an ulp; G7 accepts only the prepared leaf) |
| Bit-equality tested without the wall | `o4_overlay_equality_includes_the_wall` (a discriminator where t_r ≠ t but A, Z, I and J round equal: the twin is certified, not overlaid) |
| Reader keeps dropping reduced rows by metadata | corpus mutations `reduced_rows_dropped_with_reduced_term` (G3) and `reduced_term_on_equal_sections` (G5b `SECTION_MISMATCH`); precedence pinned in Rust, wasm and Python |
| A regenerated envelope differs beyond the limitation text | `o4_text_only_envelope_diff` (4.9) |
| W1-E's reduced derived rows formed from the nominal operands or the nominal annulus; the operand table left without a basis | `o4_w1e_reduced_derived_rows_use_their_own_basis` (a distinct-basis member: σ⁺/σ⁻ equal `normal_stress_extremes` of the reduced stress rows, τ_v,max uses the reduced annulus's c_b; the case selects, so the "derived operand identity" refusal is not reached); the reader's `u4b_evidence.rs` check refuses the mutant's successor in Rust, wasm and Python; bit-equal members keep their bytes under `o4_text_only_envelope_diff` |
| A twin-predicate consumer left on metadata only (each of the five sites in 4.5, notably `prepared_not_covered`'s `bound` skip) | `o4_w1e_twin_predicate_is_per_member` (a mixed document, one distinct-basis and one bit-equal member: the case selects, bound positions match the kernel's `NotCovered` indices, and the bit-equal member's twins keep their overlay); each site's mutant fails closed and is recorded |
| MJ9's both-bases tamper still assumes coincident sections | MJ9's tamper control is revised: on a distinct-basis member it tampers one basis and expects the other basis's own check to pass; on a bit-equal member it keeps today's both-bases tamper |

## 7. D3 and pricing

M = 12 GiB, ⌊0.9 M⌋ = 11,596,411,699 B, R = 64 MiB and the upward 5 % TEXT reserve are unchanged (Decision 22).

**Starting profile (epoch 4 as built).** The registered tree is epoch 4's actual-source run at `b4c9f35f8e` (`retained_memory.rs:1213–1239`; `retained-pricing-rules.md` §8.8, "Registration (T4-S7)"): 307 atoms, 100 forms, all 70 evaluations fitting at the unchanged policy, with T4-S6's streamed publication hashing (S6-HS) at T16 (the W1 publication hashes), T17 (the precommit reader's G1, G3 and G8 hashes) and T25 (the source-receipt replay). The headroom against ⌊0.9 M⌋:
- tightest overall: merged dense **W3** (T16 publication with the staged copy) at **+2,923,863,736 B**;
- route E's own tightest: dense W3 at +3,001,970,473 B;
- the source route's phase: merged dense X1 (T25) at +2,962,451,404 B.
O4's owners enter route E and the source route. The merged tree is the coefficient-wise maximum of routes L and E, so O4's route-E delta can lower the merged W3 headroom by at most that delta.

**Numbering.** Reviewed inputs are numbered from 0 in `REVIEWED_INPUTS` order (`build_identity.rs:158–181`): #0 `Cargo.lock`, #1 `physics_source_recovery.schema.json`, #2 `retained_precision_mp_v2.schema.json`, #3 DEF-O, #4 the preview-retained table, #5 the preview-physics-2 table, #6 DEF-C, #7 DEF-E, #8 the physics-retained table, #9 the pressure-source-1 table, #10 `results_pressure_2.v0.3.schema.json`, #11 the pressure-2 table. `retained-pricing-rules.md` and the registration comment count the same list from 1.

**Reviewed inputs that move:**
- #2 `retained_precision_mp_v2.schema.json`: both section shapes (4.6);
- #7 DEF-E;
- #8 the physics-retained table: the DEF-E hash.

**Inputs that may move:**
- #1 is expected unchanged (3.3).
- #9, #10 and #11 do not move. At `084ff961f8` neither table (#9, #11) carries limitation 7 or any shear-model or allowance text. The pressure-2 schema (#10) already has the shapes O4's documents publish (`mill_tolerance_m`, `corrosion_allowance_m`, `beam_theory`; its `euler_bernoulli` constraint is arc-only, and arcs stay declined) and no O4 decline text.

The actual-source D3 binds whatever moves. #0 (`Cargo.lock`) does not move, because no crate edge is added.

**New positive owners** (each with a rule argued from code facts in `retained-pricing-rules.md`, a source pin and independent review):
- the `TimoshenkoLeafSet` sidecar, its view and its preparation record, at the cap with every member Timoshenko, and `MemberOperators`' and `BoundedCoefficients`' D̄ and D entries, at every precision a member is formed, times the existing member multiplicities;
- the κ_E and D-entry enclosures in both lanes, and the η and bridge D-entry gaps;
- the reduced section facts per member, at the cap with every member distinct;
- the receipt entries of distinct-basis reduced rows (`absolute_verified` items and row indices), at the cap with every member distinct (21 stress rows per member). U4b's derived reduced rows are already receipt rows under `not_covered` and are not new;
- the premises priced at m that become 2m for distinct-basis members: the certified reduced maximum (`MaximumE`), the reduced annulus's preparation events (`ConversionEvent`, `PreparedMemberEvent`) and its work record;
- K4SRC/K4STF's Timoshenko block (`E_src`, `E_stf`) and the T17 reader's G8 rederivation of both bases and the shear-area leaf;
- `ReducedSectionSet`'s row-index map and the reduced-coverage vector (4.5);
- `prepared_not_covered`'s reduced operand table: one `[[Option<usize>;5];5]` per distinct-basis member, per attempted case in T14, at the cap with every member distinct (at most U4B-W1's table term again);
- the source route's private `reduced_recovery_section` and `shear_area_bits`, the second endpoint-maximum evaluation and the per-basis evidence entry;
- TEXT: the new and edited literals (the limitation strings included), and the deletion of the three decline texts, `MILL_TOLERANCE_DECLINE` and `SOURCE_MAXIMUM_BASES_DISTINCT`. `RETAINED_TIMOSHENKO_REFUSAL` stays as the association text (4.2);
- the narrowed admission pins (`batch5_model.py`, `b7a_domain_model.py`).

**Feasibility, against the registered profile.** O4 adds no published row and no D1 domain. Route E's published-row premise already counts both bases' rows (P_final = 7n + 102m + 8g + 3, E4-8), and the receipt's class entries are priced one per published row (`g4_caps.py` `SELECTION`, `absolute_verified`/`not_covered`). So the twins' class entries are inside today's premise, provided O4 adds at most one class entry per published row; the row-index map is a new owner. A design-time estimate at the caps (m = 32, three attempted cases), using the pricing tool's own strides (`producer_caps.py`: `Wide<16>` = 144 B; `MemberOperators` and `BoundedCoefficients` charged per member at each of the four precisions in the retained set and once more in scratch):
- D and D̄ entries: at most 8 more `Wide` per member in each of the two structs, so ≤ 2 × 8 × 144 B × 32 members × 5 instances ≈ 0.37 MB per case, about 1.1 MB for three cases;
- κ_E and D-entry enclosures in two lanes, and the sidecars: a few `Wide<16>` per member per lane, under 0.1 MB per case;
- `ReducedSectionSet`, its row-index map and the reduced-coverage vector: under 0.1 MB per case (`ProductMemberFacts` is priced at 512 B per member);
- the reduced operand table: 32 × 400 B ≈ 13 KB per attempted case;
- the new section-term keys in both receipt shapes, and the reduced preparation record, at the receipt's string-entry rate (RID = 1,024 B per entry, conservatively): about 0.5 MB per case, carried through the staged copy;
- the source route's reduced recovery section, shear-area bits and second endpoint evidence: a few KB per member at T25;
- TEXT: the edited limitation literals, net a few KB, plus the 5 % reserve.
The total is of order 10 MB in every phase, under 1 % of the tightest headroom (W3, +2,923,863,736 B). The estimate is a design figure, not a registration: the actual-source D3 binds every owner at its priced multiplicity.

**As built (registered).** The actual-source D3 registered at basis "SQ2 G5 at 7823a79f45" (ATOMS 319) with all 70 evaluations fitting at the unchanged policy. The design estimate above was low by about a factor of twelve: merged dense W3's E + R grew by +123,736,741 B, so its headroom is +2,795,665,581 B (epoch 4: +2,923,863,736 B). The main owners at W3 are conservative by-name TEXT growth (TAV_W, +89.2 MB), the distinct-basis reduced receipt entries charged once per copy (T16, +33.2 MB), and the grown kernel strides (+0.37 MB); the other phases' tightest are X1 +2,928,613,134 B and W4 +3,001,138,616 B. Reviewed inputs #2, #7 and #8 moved; #0, #1 and #9–#11 did not. No product source changed for the registration; the rules and owners are in `retained-pricing-rules.md` §8.9.
- If the evaluation does not fit, byte-preserving reductions come first, as U6-DH/RB/SH and W1's L-levers did.
- A measured shortfall that those cannot close goes to HELP_HUMAN before any budget change.

## 8. Relation to the W1 coverage repair: sequence, not merge

b7-plan: "Its design proceeds alongside O4, which revisits the same section bases and W1-E admission. Implementation follows O4 unless that design shows the two should merge." The companion design is [w1-coverage-repair.md](w1-coverage-repair.md).

**Decision: they do not merge.** O4 is implemented first, as ordered. The W1 repair's census runs as soon as epoch 4 merges, alongside O4's reference unit. The repair is implemented after O4, on O4's head.

**Why:**
- **No shared numerical law or cause.**
  - The first W1 corner is the loaded-maximum search on route L, whose admitted lane is driven over an interval-valued section modulus. Route L carries no Timoshenko member and no allowance, and route E has no member loads (D1.7), so no O4 case reaches that search.
  - The second W1 corner is the provenance of the receipt's `body_scales` field: native evidence, where the gate and readers use the final rows. O4 changes neither the scale law nor its provenance.
  - O4's laws (the Timoshenko D recipe, the κ_E lane and the D-entry gaps) do not touch either corner.
- **Different evidence paths.** O4 needs a new independent reference unit and an M3-style judged set before its outputs can be judged. The W1 repair needs a census and two small, byte-preserving changes. Merging would hold the repair behind O4's reference unit with no technical benefit, and it would put both checklists into one independent review.
- **Attributable D3.** Each unit re-registers on its own head, and each one's delta stays attributable. Two registrations cost one more pricing run. One merged registration would hide which unit consumed the headroom.
- **The shared files are textual neighbours, not shared meaning.** These are `retained_wire.rs` `selection` (O4's `section_terms`, the repair's `body_scales`), RE `g5b` (O4's section and `row_scale` checks, beside the unchanged body-scale equality) and the registered profile. Sequencing settles them.

**Coupling rules** (binding on O4's implementation):
1. O4 does not change the represented-Z hull, the loaded-maximum search or the provenance of the receipt's `body_scales`. The repair changes them once for every route, O4's new successors included.
2. O4's new route-E controls and judged-set designs avoid exact binary64-halfway body maxima, or they record a G5b withdrawal on such a control as the expected fallback until the repair lands. They never edit a scale to pass.

Section 8 of the W1 repair design states the same decision from its side.

## 9. Verification and independent review

**Checks on O4's head:**
- the mutants of section 6;
- M3-O4 (section 5) on the frozen reference unit;
- the suites `select_affected_checks.py` selects: frame kernel (including `retained_k4` and `timoshenko_element`), product physics (source recovery, the join and its consequences, J7, the facade and census tests, W1-E admission), result_export, headless, the Python verifiers and the desktop wasm-equivalence suites;
- J2, the R1 and R2 records and the retained corpus in native Rust, wasm and Python;
- the actual-source D3 (`tools/retained_pricing`, `generate --check`);
- the hosted `harness` on the exact head.

The registered branch runs on the qualification host's registered build, and the `Stale` branch in an emulated unregistered build, before the push (CI_STRATEGY).

**Independent numerical review checklist** (separate agents; AGENTS.md):
1. The Timoshenko recipe (4.2): its identity with U8's element, φ's and c_s's rounding schedule, and the Euler–Bernoulli branch's byte preservation.
2. Section 4.4: that each existing derivation reads D only through its absolute operand and its coefficient c_D, the counts ℓ and c_D, the finite D-entry bound against the upward-rounded D̄ and λ = 2, the substitution lemma, the alpha law with D-entry gaps at the exact length, κ_E's enclosure and range, and the K4STF and sidecar binding. This review precedes implementation code.
3. The two readouts (4.3): the admitted lane's prepared Â_s,prep (and G7's equality with it), and the annular lane's exact κ_E on the nominal annulus.
4. The dual basis on both routes (3.2, 4.5): each reduced row on its own section, the bit-equal overlay rule, the per-basis maxima and headlines, the reader bindings, and W1-E's derived not-covered rows formed per basis.
5. The source route (3.1): that the declared Timoshenko K_local passes every RF closure unchanged, and the declared-problem oracle's verdicts.
6. Readers and verifier: every lifted refusal is replaced by a rederivation, not dropped. The row universe and the new refusals sit where section 4.7 places them, and existing refusals keep their gate and order.
7. Bytes: the text-only diff (4.9) holds on every regenerated envelope. Euler–Bernoulli, zero-allowance numbers, work and receipt layouts are unchanged (sidecar, overlay rule). The only other move is the authorized DEF-E-bound hashes.
8. The work law (3.5) and the D3 registration (7).
9. Claims review of the edited limitation texts (AC-008).

## 10. Proposed ScopeOfWork, dependency and b7-plan text

These edit commitments or declared conditions, so they are proposals. The owner decides the ScopeOfWork edits; the implementation PR carries them only once approved. The text below is written against epoch 4's edited ScopeOfWork (E4-9).

**DEL-04-08 ScopeOfWork** (on O4's discharge):
- **REQ-001, "Route E domain".** Replace the two bullets "an explicit Euler–Bernoulli shear model;" and "zero mill tolerance and zero corrosion allowance." with: "either explicit shear model (`euler_bernoulli` or `timoshenko_annulus_energy_v1`) on straight members; and any admitted mill tolerance and corrosion allowance, under the D-5 bases: stiffness and the Timoshenko shear area on the nominal wall, and each stress row on its own basis section."
- **REQ-001, "What each class guarantees",** third bullet. Append: "For a Timoshenko member, the admitted readout uses the binary64 shear-area leaf and the exact-annulus readout the exact energy-matched factor κ_E."
- **REQ-006 (a).** Delete the item. Renumber, or leave "(a) [discharged by O4]" if the owner prefers stable item letters.
- **REQ-007 (a).** Delete on discharge (AC-007: "Until then it remains listed here as open").
- **AC-003,** third bullet. "the two W1 corner cases and the O4 declines keep their internal refusals and fall back" becomes "the two W1 corner cases keep their internal refusals and fall back", until the W1 repair discharges it too.
- **VER-003.** Replace `o4_w1e_declines_at_admission` and `o4_prepare_sources_declines_by_name` with `o4_w1e_timoshenko_case_is_admitted_and_selected`, `o4_w1e_dual_basis_case_is_admitted`, `o4_timoshenko_source_case_selects_and_publishes` and `o4_dual_basis_source_case_selects`.
- **VER-007.** Add "and, for O4, M3-O4 on `validation/references/o4_retained_source/`".

**DEL-04-08 `deliverable.yaml`** (the DEL-05-03 need). Replace the sentence "Its Timoshenko formulation and its mill-tolerance and corrosion-allowance fields are inputs this method declines by name until O4." with: "It also supplies the Timoshenko formulation (U8's fixed-order κ_E and shear-area leaf), which both retained routes form and the readers rederive, and the mill-tolerance and corrosion-allowance fields behind the D-5 bases." The DEL-04-01 need gains: "… and the retained Timoshenko straight member (natural-mode D recipe, κ_E lane enclosure and D-entry gap majorants)".

**DEL-04-01 Design** (when O4 lands; it is the kernel owner's design): a pointer from the retained-kernel description to section 4.2's Timoshenko member and section 4.4's lemma.

**DEL-05-03 Design `section-and-shear-decisions.md`,** "Retained-source and W1-E publication (O4)". Add a pointer line: "As designed: DEL-04-08 Design `o4-source-extension.md`." The as-built text follows when O4 lands.

**b7-plan.** Items 5 and 6 carry their design pointers, added after epoch 4: "Design: [o4-source-extension.md](o4-source-extension.md)." and "Design: [w1-coverage-repair.md](w1-coverage-repair.md). Sequenced after O4; the two do not merge (o4-source-extension.md §8)." When O4 lands, item 5 is replaced by a one-line "as built" pointer, as items 1–3 were.

## 11. Rulings

**Applied:**
- the owner's O4 ("Scope extension after B7b");
- D-5 and D-6 ("D-5 and D-6 as recommended");
- O2 and O3, as their refusals stand;
- the E14 precedent for definition-hash moves (HELP_HUMAN, 2026-10-17);
- REQ-007 (a)'s "keeps RF's closure, witness and charging rules".

**Decided in this design,** for the independent review to check:
- the source operand is the frame's own Timoshenko K_local (3.1);
- the conditional private payload members, K4STF block and receipt members, and the sidecar that keeps receipt layouts (3.1, 4.2, 4.6);
- the prepared shear-area law (4.2);
- the natural-mode Timoshenko recipe and D̄ = λ(c + c_s), with the substitution lemma (4.2, 4.4);
- exact κ_E in the annular lane (4.3);
- D-entry gaps for Timoshenko members, at the exact length (4.3);
- the data-determined overlay rule, per member, on {As, Z, I, J, wall} (4.5);
- the reader row universe and the placement of the new refusals (4.7);
- the test-only payload dump for the source oracle (5);
- the reference unit's reuse of the frozen kd5t engine (5);
- sequence, not merge (8).

**Decided by the owner:**
- **AC-002 and J7's digests.** The owner first adopted this design's proposed AC-002 wording (2026-10-18, "Adopt the wording (Recommended)"), and on 2026-10-19 replaced AC-002's second bullet with: "the RF census classification, counts, first causes, selections, charged work and frozen-row verdicts are unchanged; its envelope, results and summary digests change only as a stated product-text, identity or published-row change requires". #1461 (epoch 4, main `084ff961f8`) applied it, and it is AC-002's text on main. O4 relies on it as written: the limitation-text change of section 3.3 is a stated product-text change, so J7's envelope digests move with it while its results and summary digests do not (3.6, 4.9), and the census's classification, counts, first causes, selections, charged work and frozen-row verdicts do not. The earlier proposed wording is withdrawn from section 10.

**Reserved and untouched:**
- the ScopeOfWork edits of section 10;
- Decision 22 (M, the margins and the budget model);
- TBD-001 and TBD-002;
- b7-plan's "Held by the owner": G10, B8 and the frozen UI benchmark;
- desktop outputs for the exact identity;
- release.

## 12. Blast radius

**Code**, by owning deliverable:
- **Frame kernel (DEL-04-01):** `structural/retained/` `assemble.rs` (sidecar, recipe, Ā, `BoundedCoefficients`), `source.rs` (sidecar view, K4STF/K4SRC block), `recover.rs` (Q = D·e with d₁ and d₂), `adaptive.rs` and `verify.rs` (re-formation sites, E operands), `product_certificate.rs` (κ_E and D-entry enclosures, reduced facts), `product_certificate/source_residual.rs` and `bridge.rs` (D-entry gaps), `final_case.rs` (reduced rows), `retained_resource.rs` (layouts), `structural.rs` (exports, refusal removal), and tests (`retained_k4`, `timoshenko_element.rs`).
- **Product physics (DEL-04-07):** `source_recovery.rs` (declines removed), `source_receipt/rows.rs`, `source.rs` and `composite.rs` (per-basis sections and maxima), `lib.rs` (`SOURCE_MAXIMUM_BASES_DISTINCT` removed), `exact_admission.rs` (limitations), `retained_memory.rs` (admission; registered profile), `retained_product.rs` (capture, facts, overlay, staging, `prepared_not_covered`'s basis dimension), `retained_wire.rs` (Section members; DEF-E hash), and the facade, census, source and `timoshenko_product_tests.rs` tests.
- **Readers (DEL-05-02 shared, DEL-04-08 owned):** RE `retained_precision.rs` (G5b, G8), `physics_source.rs` and `source_blocks.rs`; `tests/verifiers/retained_precision.py`, `physics_source.py` and `u4b.py`; `semantic_contract.rs` pins.
- **Schemas and definitions:** reviewed inputs #2, #7 and #8, and possibly #1 (section 7).
- **Pricing:** `tools/retained_pricing` (owners, admission pins, rules) and `retained-pricing-rules.md`.

**Fixtures and corpora**, each regenerated by its own writer:
- every exact envelope that carries the edited limitation strings (4.9): ordinary pressure-2 fixtures and corpora, pressure-source-1 raws and derived documents, J2's bases, MJ6 and MJ9's bases, J7's consequences and envelope pins, the route-E successors and RV120's exact inputs, and the R2 records and desktop parity inputs. All are text-only, checked by `o4_text_only_envelope_diff`;
- the route-E successors also move in their DEF-E-bound hashes;
- J2: new Timoshenko and dual-basis bases in both modes, plus section 6's mutations;
- the R1 records and the desktop wasm inputs, where their inputs moved;
- the K4 vectors: new Timoshenko vectors, existing ones unchanged;
- hash pins.

**Must stay byte-identical:**
- every pressure-free byte;
- every route-L successor, and the retained corpus's route-L entries;
- every receipt-recorded layout and adapter counter;
- `validation/references/**` and the frozen UI benchmark.

**Must keep every number, class, selection, first cause and work counter:** every Euler–Bernoulli, zero-allowance exact envelope, source publication and route-E successor, the J7 classification and the MJ6/MJ9 controls.

**Deliverables:** DEL-04-08 (method, readers, definitions, pricing), DEL-04-01 (kernel), DEL-04-07 (PP) and DEL-05-02 (shared readers). `tools.deliverables impact DEL-04-08` lists 39 affected deliverables through DEL-04-07 and DEL-05-02. O4 changes no editor, guide or desktop product surface beyond the limitation strings those already display.
