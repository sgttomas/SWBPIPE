# O4: the Timoshenko and corrosion-allowance source extension

This is the implementation design for the fifth post-T3 shared epoch ([b7-plan.md](b7-plan.md), "After B7b", item 5). It extends retained-source recovery (`pressure-source-1`) and W1-E publication (`physics-retained-1`) to the exact-contract documents that DEL-04-08 ScopeOfWork REQ-006 (a) declines today: documents whose contract selects `timoshenko_annulus_energy_v1`, and documents with a nonzero mill tolerance or corrosion allowance on any pipe section. It keeps RF's closure, witness and charging rules, and it keeps every gate, limit, tolerance and frozen reference unchanged.

**Status: designed, not implemented.** The basis is main `ca8d6d5c23` plus the epoch-4 design (DEL-04-07 Design `u4b-u5b-implementation.md` at `c2e450ac7f` on `claude/piping-t4-u4b-u5b-20261017`). Epoch 4 is not merged. Section 0 names every assumption taken from it; they are re-checked against the merged epoch before implementation starts.

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
- **Admission and readers.** W1-E's G-A admits both shear models and nonzero allowances. The retired facts are `ShearModel` and `SectionAllowance`. G8, G5b and the source readers stop requiring Euler–Bernoulli and zero allowances. In their place they rederive the D-5 bases and the Timoshenko leaf from the captured invocation and bind them, through the rederivation the pressure-2 readers already have (DEL-05-03 `section_basis`).
- **What moves.** The limitation texts must stop stating the O4 declines (3.3). Every exact envelope carries them, so every exact ordinary envelope, every pressure-source-1 publication and every route-E successor moves in those strings and in the hashes that bind them. That includes J2's bases, MJ6 and MJ9's pinned bytes and J7's envelope digests, as U6 and epoch 4 also moved them. Numerical values, classes, selections, first causes and charges of Euler–Bernoulli, zero-allowance documents do not move, and a text-only diff check enforces that (4.9).
- **Proof obligations (section 4.4).** The dominance lemma is derived here from the formation code. The independent numerical review checks it before implementation. If it fails, the epoch returns to HELP_HUMAN with the concrete conflict; no law changes to obtain a pass.
- **Independent references (section 5).** A separate, later reference unit (`validation/references/o4_retained_source/`) judges the new publications against exact solutions of both readouts. It reuses the frozen, independent `kd5_timoshenko` engine. An independent exact solver checks each source receipt's declared problem through a test-only dump bound to the receipt's hashes.
- **Merge with the W1 coverage repair: no (section 8).** The two share no numerical law. O4 lands first, as b7-plan orders, and the repair's census runs as soon as epoch 4 merges. Two coupling rules keep the shared files consistent.
- **Reserved:**
  - the ScopeOfWork edits that discharge REQ-007 (a) (proposed in section 10);
  - AC-002's reading of "the RF census classification and its pins are unchanged" now that the limitation text moves J7's envelope digests (section 11);
  - any Decision 22 change if D3 does not fit;
  - TBD-002's table sentence, which stays untouched.

## 0. Basis and epoch-4 dependencies

O4 is designed against main `ca8d6d5c23` and against the epoch-4 design as written, not as built. Each assumption below is re-checked against epoch 4's merged head before O4's implementation starts; a failed assumption revises this design first.

| # | Assumption from epoch 4 (u4b-u5b-implementation.md) | Where O4 relies on it |
|---|---|---|
| E4-1 | Model 0.6.0 is the only current version. 0.5.0 is refused by name, and behaviour keys on presence (§1.1). The literal `"0.5.0"` in `SourceContract::from_invocation` (`physics_source.rs`) and `exact_namespace` (`retained_precision.rs`) becomes 0.6.0, and `namespace_branch` keeps using `MODEL_SCHEMA_VERSION`. | Sections 3.4, 4.1, 4.7: O4 edits the contract predicates next to those literals, not the version. |
| E4-2 | `pressure-2` replaces `pressure-1`. `physics-retained-1` inherits pressure-2's table by hash. `pressure-source-1` keeps its name and its own table, which epoch 4 revises in place (reviewed input 9 moves). `results_pressure_1.v0.3.schema.json` is renamed `results_pressure_2.v0.3.schema.json`. `REVIEWED_INPUTS` keeps its order, with indices 10 and 11 renamed (§1.2–1.3). | Sections 4.8 and 7: the inputs O4 moves are named by their epoch-4 names. Limitation 7 of the unselected exact envelope is pressure-2's. |
| E4-3 | U4b's σ⁺/σ⁻ and Saint-Venant rows are published per station and basis. In retained successors and source publications they are `not_covered` (W1, `ProductRecipe::NotCovered`) or `derived_not_qualified` (source). They are formed from the publication's own published rows (owner, 2026-10-17, option (ii); §4.1, §4.3). | Sections 3.2 and 4.5. Where O4 makes the reduced stress rows distinct, the reduced derived rows follow automatically, because they are formed from the reduced basis's own published rows. O4 adds no recipe for them. |
| E4-4 | The ordinary K-D5 route publishes the corrected displacement fl(u + w + w2) wherever a certified bound exists (HELP_HUMAN, 2026-10-18). The Passed/Sensitive classification is unchanged. | Retained-source eligibility (ordinary Sensitive or errs) and W1-E's trigger do not move. Only the ordinary fallback bytes that O4's fallback tests compare against move, and those are taken from the merged producer. |
| E4-5 | U5b's tie rods live on objective connectors. D7 still declines connectors at `prepare_sources`, and W1 capture still declines them. | O4 does not interact with tie rods. |
| E4-6 | Epoch 4 re-registers under D3 (§6). Its registered headroom is unknown here. Before epoch 4, the tightest phase was merged dense W4 at +113,188,985 B (W1 registration). | Section 7: O4's D3 starts from epoch 4's registered profile. |
| E4-7 | DEF-O, DEF-C and DEF-E name preview-physics-2 and pressure-2 in `inherits.base_contract`. DEF-E's `scope.model_format` takes 0.6.0. Their meaning hashes move in epoch 4 (§1.2). | Section 4.8 edits DEF-E on top of epoch 4's bytes. |
| E4-8 | Route E's row premise is P_final = 7n + 102m + 8g + 3 (72 + 30 rows per member, two bases). Route E stays below its 16,384 row capacity at the D1 caps (§6). | Section 7. O4 adds no published row. It turns overlay twins into receipt rows only for members whose bases differ. |
| E4-9 | Epoch 4 edits REQ-001 and REQ-002's version tokens and REQ-001's value clause (§4.4). It leaves REQ-006 (a), REQ-001's other route-E bullets and AC-003 to this epoch. | Section 10's proposed text is written against epoch 4's edited ScopeOfWork. |
| E4-10 | On pressure-2 the Saint-Venant factor c_b uses the member's authored ν and each basis's own annulus. Route E recomputes it from the captured evidence (§2.4, §4.1). | Section 4.5. With distinct bases, the two c_b differ, and that follows from the evidence. |
| E4-11 | Epoch 4 adds no W1-E admission clause. It moves no W1 or source decline. | Section 1's inventory is main's, plus epoch 4's renames. |
| E4-12 | Epoch 4 moves every ordinary envelope's identity, limitation and version bytes, and regenerates J2, J7's consequences and pins, MJ6 and MJ9's bases and the route-E successors by their writers (u4b §9, "Moves"). | Section 4.9. O4's text-only moves follow the same practice; AC-002's reading is raised in section 11. |

## 1. What is declined today

Code facts on main `ca8d6d5c23` (paths relative to `projects/chirality-piping/`; PP is `core/product_physics/src/`, the kernel `core/solver/frame_kernel/src/structural/retained/`, RE `core/reporting/result_export/src/`):

| Site | What it does |
|---|---|
| `source_recovery.rs` `prepare_sources` (about 799–822) | After the D7 declines and the budget refusal, and before the dimension closure, it declines on an exact document: `shear_model = timoshenko_annulus_energy_v1` ("timoshenko_annulus_energy_v1 straight members are not a retained-source formulation"), a nonzero mill tolerance and a nonzero corrosion allowance. The receipt records `unsupported_family`. |
| `source_receipt/rows.rs` (about 377–394) and PP `lib.rs` `same_stress_section` / `SOURCE_MAXIMUM_BASES_DISTINCT` (about 5965–5975, 10785–10794) | Source stress rows and the endpoint maximum use one section (`built.sections`, the nominal section) and require the reduced section to be bit-equal. Otherwise they refuse ("derived stress section bases differ") or withhold the reduced maximum by name. |
| `retained_memory.rs` `namespace_branch` (840–855), `family_clauses` (about 897–911) | W1-E's G-A refuses a Timoshenko contract under `FamilyFact::ShearModel`, and a nonzero mill tolerance or allowance on branch E under `FamilyFact::SectionAllowance`. The ordinary bytes are published unchanged. |
| Kernel `assemble.rs` `StraightMember::from_frame` (about 256–281), `RETAINED_TIMOSHENKO_REFUSAL` | It refuses a Timoshenko leaf by name. The retained element is Euler–Bernoulli, with D's bending block (EI/L)[[4,2],[2,4]]. PP `retained_product.rs` (about 2018–2033) maps the refusal. |
| RE `retained_precision.rs` `exact_namespace` / `section_allowances` / `exact_mill_tolerances` (4720–4760) | G8 requires the three-member contract with `euler_bernoulli`, an explicit length-valued zero allowance on every section, and zero mill tolerance (`MILL_TOLERANCE_DECLINE`). |
| RE `retained_precision.rs` `g5b_exact_evidence` (about 4405–4470) | G5b binds the receipt's one section to both evidence bases bit for bit, and requires each reduced stress row to be its nominal twin's retained value (the overlay). |
| RE `physics_source.rs` `SourceContract::from_invocation`, `zero_corrosion_allowances`, `zero_mill_tolerances`, the `SECTION_BASES` and `ACTUAL_SHEAR_MODEL` checks, and `validate_stress` (`STRESS_SECTION_PARAMETERS`, `STRESS_BASIS_TWIN`) | The source readers require the Euler–Bernoulli contract literal, zero allowance (`PHYSICS_SOURCE_ACTUAL_CORROSION_ALLOWANCE`), zero mill tolerance (`PHYSICS_SOURCE_ACTUAL_MILL_TOLERANCE`), coincident evidence sections, and one parameter set for both bases. |
| Python `tests/verifiers/physics_source.py`, `retained_precision.py` | They mirror every row above. |
| DEF-E `retained_precision_prepared_exact_v1.json` (reviewed input 7) | Its `scope.corrosion_allowance`, `scope.shear_model`, `scope.mill_tolerance`, `evidence` and `trust.G8` texts state zero allowance, zero mill tolerance and Euler–Bernoulli, "until O4". |
| `exact_admission.rs` limitation 0 (pressure-source-1) and limitation 7 (the unselected exact envelope and pressure-source-1) | The selected response is "Euler-Bernoulli with coincident section bases". Timoshenko and nonzero allowances "are declined by name". |
| Tests named in VER-003 | `o4_prepare_sources_declines_by_name` (`source_recovery.rs`), `o4_w1e_declines_at_admission` (`retained_facade_tests.rs`), and FK `timoshenko_element.rs`'s refusal test. |

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

The source method certifies "the exact solution of the equations formed exactly from the declared local matrices, transforms, springs and loads" (RF §3.1). Its declared operand per member is the built frame's binary64 `local_stiffness()`, closed against the pipe element's matrix and against the assembly (`prepare_sources`, member loop about 1269–1360):
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
  - The receipt's `endpoint_sections` (`composite.rs` about 947) stays one per member. Endpoints, functional indices and actions are basis-independent.
- **Headlines.** `validate_composite_summary` already binds both per-basis headlines. Each now binds its own basis's maximum rows.
- **U4b's derived rows** (σ⁺/σ⁻ and τ_v,max, `derived_not_qualified`) are formed from each basis's own published rows (E4-3). With distinct bases they differ by basis. That follows from the formula and needs no change here.
- **Bit-equal bases keep their numbers.** A document whose stress sections are bit-equal (zero mill tolerance and allowance, or values that round away) produces today's payload, checks, values and work. Only the limitation strings change (3.3, 4.9).

### 3.3 The receipt and schema

- **Schema.** No change to `physics_source_recovery.schema.json` (reviewed input 1) is needed for the bases or the shear leaf. The new payload members are private, and the public checks and maxima keep their shapes. The actual-source D3 binds input 1 only if the implementation finds a public shape that has to move.
- **Producer.** `failure_fields` loses no mapping. The three O4 decline texts are deleted, and with them every receipt `unsupported_family` that carried them.
- **Limitations** (`exact_admission.rs`, whole static strings at the same positions; product text, reviewed against `docs/claims_registry.md` under AC-008):
  - pressure-source-1 limitation 0. Its last-but-one sentence, "The selected retained-source response is Euler-Bernoulli with coincident section bases.", becomes "The selected retained-source response uses the contract's shear model and publishes its stress rows on both section bases."
  - Limitation 7 (unselected and selected). The O4 clauses ("only for euler_bernoulli documents with zero mill tolerance and zero corrosion allowance, and", "the selected response is euler_bernoulli with zero mill tolerance and zero corrosion allowance, where the two bases coincide", and "Timoshenko documents, nonzero mill tolerance or corrosion allowance and") are removed. The rest of each sentence stands.
  - The exact final strings are fixed in the implementation PR and reviewed there. Every carrier (schemas, readers, verifiers, fixtures) takes the same bytes, as U6 §2.4 required.

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
  - ΔF_k comes from `finalization_size` (`source_receipt.rs` about 418–470). That function counts 12 × the source-identity string length, the source dimensions, descriptor lengths and row identity, kind, entity, unit and metadata lengths. It does not count the serialized normalized-source payload.
    - The identity string is `serde_json::to_string(&(names, bits))` (`source_recovery.rs` about 747–757), with every scalar written as its decimal u64 bit pattern, so its length depends on values. A nonzero allowance or mill tolerance serializes to about 19–21 characters where zero serializes to 1. A mill-tolerance quantity enters only when present. A Timoshenko K_local's entries have other digit counts, and an exact zero at φ = 2 shortens it.
    - Descriptor product lengths change wherever a structural zero drops a term.
    - So ΔF_k = 2·12·Δ(identity length) + Δ(descriptor-length terms). For example, an explicitly present mill tolerance changing from 0.0 to 0.001 adds 18 identity bytes, and so 432 to F_k. The test derives ΔF_k from the actual serialized identity and inputs and asserts equality, not zero. Published rows and their metadata lengths do not change.
  - ΔQ_k includes the first `retained_identity_digest` reservation, which charges a separate 12 × identity length: 216 more in the same example. It also includes the second basis's endpoint maximum. `composite_member_maximum` charges `RECIPE_WORK` per call (`composite.rs` about 172), and the second basis is a second call. Each later call also reads the cached identity digest, which charges `charge_finalization(128)` (`source_recovery.rs` about 233–234). The maximum is computed at two sites, and each gains a reduced-basis call: the publication loop (`lib.rs` about 5960) and the composite receipt's expected-rows pass (`source_receipt/composite.rs` about 785). So ΔQ_k = 2 × (RECIPE_WORK + 128) = 8,448 per member whose sections differ, plus each call's own metered work in `maximum_inner`. The test counts the calls at both sites rather than assuming one. `composite_member_maximum(input, selected, member_id)` (`composite.rs` about 165–170) has no basis argument today, and `maximum_inner` reads the built nominal section. So the reduced-basis call takes the reduced section explicitly, and the test asserts that the second call's evidence is bound to the reduced section's bits. Calling twice on the nominal section cannot satisfy it. Composite-exact wrapper terms depend on row counts, and none moves.
  - ΔP = 0, because no published row or diagnostic is added.

An independent test derives every term from the actual inputs and sites, as the MJ9 test does, and reconciles their sum with the counters exactly; it asserts ΔP = 0. MJ6's and MJ9's twelve Euler–Bernoulli requests keep their work counters in both modes; their bytes move only in the limitation strings and the hashes binding them (4.9). The 8,000,000 and 64,000,000 limits are unchanged, and an input at the limit still refuses by name.

### 3.6 What stays

- **RF's census classification is unchanged.** The J7 RF census re-authors every case to explicit Euler–Bernoulli with zero allowance (RF §10 item 1). Its classification, counts, first causes, selected members, frozen-row verdicts and work do not move. Its four published envelopes, and every consequence digest, change in the limitation strings, so their `envelope_sha256` pins are regenerated through `SWBPIPE_J7_RF_CENSUS_WRITE=1`. RF §6 requires that regenerated pins be compared entry by entry, with every difference in a stated class; the class here is the limitation text and the hashes that bind it. U6 regenerated the pin for its own text and row changes, and epoch 4 does so for the identity rename (E4-12). AC-002's sentence on the pins is raised for the owner's reading in section 11.
- **`load_reference_fallback_uz` keeps its role.** It stays an ordinary pressure-1 raw (pressure-2 after epoch 4) through its zero member load's non-nodal decline, which O4 does not touch (W1 stage 2 §8; RF §5.5).
- **D7's connector and bend declines** keep their text, order and work.

## 4. W1-E (`physics-retained-1`)

### 4.1 Admission

- **`namespace_branch`.** `contract_is` admits `{version "3.0.0", mode "exact_pressure_v3", shear_model ∈ {euler_bernoulli, timoshenko_annulus_energy_v1}}`. An absent or unknown shear model stays `PressureContract`. `FamilyFact::ShearModel` is deleted.
- **`family_clauses`.** The `SectionAllowance` refusal is deleted.
- **Unchanged:** every other D1 clause, including `ReferenceConfigurations`, the expansion laws, the modulus-basis and load facts and the caps. The pricing tool's admission pins (`base/batch5_model.py`, `base/b7a_domain_model.py`) re-pin to the narrower refusal set under D3.

### 4.2 The retained Timoshenko member (kernel)

**Representation: a member-only sidecar.** Preparation records `size_of::<StraightMember>()` as `RequestedCopyBytes` for every member (`retained_product.rs` about 5478), and the receipt serializes adapter counts (`retained_wire.rs` about 1424). Growing `StraightMember`, `PreparedAssociation` or any other receipt-counted layout would move every route-L and route-E receipt. So, following stage 2's member-load sidecar (w1-member-load-certification.md §2.1, §6.2):
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
  - the kernel structs recorded through `TraceCopyWork::record` (`LaneWorkTrace`, `ProductProofTrace`, `CoverageFacts`, `ProductFailureView`; `final_case.rs` about 198–398, `source_residual.rs` about 116). These bytes appear to be checked for exactness without being serialized (`retained_receipt.rs` about 23–27). Pinning them is conservative; the review may drop the constraint if it confirms they never reach a receipt;
  - `SectionPreparationWork.layout_bytes` (`SectionPrepFrame`, `PreparedAnnulus`; serialized at `retained_wire.rs` about 1453);
  - `ProductRecipe`.
  
  The implementation lists every layout a receipt records and pins each one (`timoshenko_sidecar_preserves_receipt_layouts`; stage 2's `member_sidecars_preserve_nodal_adapter_capacity_bytes` precedent). Any O4 data that would grow one goes into a sidecar instead.

**The prepared shear-area law.** W1 preparation replaces each member's area, second moments and torsion constant with the prepared, correctly rounded annulus values (`retained_product.rs` about 5444–5490). `apply_exact_section_overlay` then writes those prepared values into the evidence. G7's pressure-2 base reader requires `shear_area == κ × nominal.area` bit for bit (`section_basis.rs` `beam_theory`, about 349–362).
- The ordinary leaf A_s = RN(κ̂·A_ordinary) (PP `lib.rs` about 7683) can differ from RN(κ̂·A_prepared) by an ulp. Codex's round-1 check found such a pair at D = 0.168 m, t = 0.007 m, ν = 0.3.
- So the prepared Timoshenko member takes **A_s,prep = RN(κ̂·A_prep)**, by FK's own `shear::shear_area(κ̂, A_prep)`, with κ̂ FK's binary64 factor from the nominal radii and ν, unchanged.
- The sidecar carries A_s,prep. The preparation association records the old and new shear leaves beside the existing association, in the sidecar's own record.
- The evidence overlay writes `beam_theory.shear_area_m2` = A_s,prep with the prepared nominal area, so G7's existing equality holds.
- K4STF, the admitted lane and the section-5 admitted readout all use A_s,prep.

**K4STF and K4SRC.** After the stiffness part, a Timoshenko block is appended only when the sidecar is non-empty: the count, then each entry's member and shear-area bits. An all-Euler–Bernoulli source keeps its K4STF and K4SRC bytes.

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
- φ̂ = φ\*·r·u⁻¹, with u = π_{ℓ+2}/(1 + ρ_φ) = π_{ℓ+3}, where ρ_φ is φ's own rounding.
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

- **Section facts (a sidecar too).** Capture reserves `facts` through the adapter (`retained_product.rs` about 1910), which records capacity × `size_of::<T>()` (about 4077–4083). So `k::ProductMemberFacts` (`final_case.rs` about 84–94) must not grow. The reduced section lives in a `ReducedSectionSet` sidecar, reserved and counted only where a member's bases differ, beside `TimoshenkoLeafSet`, with no tag added to `ProductRecipe`. Each member has a reduced section exactly when the member's reduced stress section differs from the nominal one in any of **{As, Z, I, J, effective wall}**. The wall is in the set because the annular lane reads t_r: two sections whose binary64 A, Z, I and J agree can still differ as exact annuli.
  - The reduced facts are binary64 A_r, Î_r, Ẑ_r and J_r, prepared by `prepare_product_annulus(D, t_r)`; for the annular lane, the exact annulus of D and the binary64 t_r = (t − c_m) − c_a.
  - The nominal facts are today's, with the nominal wall t. With a nonzero mill tolerance the stiffness, κ_E and the nominal stresses use t; today's route E, which admits only zero mill tolerance, already uses t.
- **How the kernel identifies a reduced row.** `ProductRecipe::Stress { member, site, stress }` has no basis field, and `row_scales` sizes stress coverage as `facts.len() × 21` (`final_case.rs` about 1146–1160). Its capacity (`capacities[4]`) is serialized (`retained_wire.rs` about 1389). So:
  - `ReducedSectionSet` carries, for each member whose bases differ, its reduced section and the final-row indices of its reduced stress and maximum rows.
  - The kernel reads a row's basis from that map, and uses that member's reduced section for it.
  - Reduced rows are covered in a separate reduced-coverage vector, allocated only when the set is non-empty.
  - `ProductRecipe`, the 21-slot coverage vector and `capacities[4]` are unchanged for bit-equal documents. A const size assertion pins `ProductRecipe`.
  - A reduced row whose index is not in the map, or a mapped index whose row is not a reduced stress or maximum row of that member, is refused by its kernel cause.
- **Reduced stress rows.** Each reduced stress row is certified by the same recipes as its nominal twin: DEF-E's directed signed-corner N/A, M_y/Z, M_z/Z and T·c/J, with the admitted lane's represented Z = hull(Ẑ_r, Î_r/c). The two-end maximum recipe is likewise applied with A_r and Z_r. Each row gets its own enclosure, class and bound under the unchanged gate. Its body scale is its body's force and moment scales divided by its own section (the reader's `row_scale`).
- **The overlay stays for bit-equal sections.** Where all five agree, the reduced twin is today's overlay: excluded from the receipt row universe and carrying its nominal twin's retained value. Two such sections give the same recipe and the same operands in both lanes, so the twin is the same certified quantity.
  - The rule is decided per member from the data, not by a document flag. A mixed document certifies distinct-basis members' twins and overlays the rest.
  - Existing route-E successors (zero allowances) keep their row universes, numbers and work.
- **PP's staging.** `apply_exact_section_overlay` writes each basis's prepared section to its own evidence object; today it writes one prepared section to both. It also writes the prepared shear area (4.2). `stage_headlines` already selects each basis's maximum rows. U4b's derived rows on route E are formed from each basis's own projected rows (E4-3).

### 4.6 Receipt and SCHEMA (reviewed input 2)

`retained_precision_mp_v2.schema.json` has two closed section shapes, and both change:
- **`$defs/Section`** (the `sources[].section_terms` items) gains `shear_area` (Bits), present exactly on a Timoshenko member, and `reduced` (closed: the same keys as the nominal source section term), present exactly when the member's sections differ (4.5).
- **The inline `cases[].selection.section_terms` item** (about 5274; `member_id`, `area`, `section_modulus`, `length`, `axial_stiffness`, `torsional_stiffness`, `additionalProperties: false`) gains `shear_area` and `reduced` {`area`, `section_modulus`} under the same presence rules. Its operational length and stiffnesses stay nominal.
- **The two encodings** are written by `retained_wire.rs`: the source terms from the sidecar and the reduced facts, the selection terms in `selection`.
  - Their member identifiers differ (the kernel member index against the authored member id).
  - Their geometry shapes differ (`geometry` {`normalized_od`, `actual_radius`, `actual_second_moment`, `actual_polar_moment`, `effective_wall`} on source terms only).
  - G5b binds source and selection to each other for the new members as it does for the existing ones (4.7).
- Receipt rows for distinct-basis reduced twins enter `absolute_verified`, `not_covered` and every row-indexed list like other rows, in published order. An all-Euler–Bernoulli receipt with bit-equal sections has no new key.

### 4.7 Readers and verifier

- **The row universe.** `retained_precision.rs` `rows_for` (about 946–955) drops every reduced-basis row by metadata. G3's coverage, G5a, `numeric_owner` and the G5 diagnostic all read it.
  - After O4, `rows_for` drops a reduced-basis row only when its member's source section term has no `reduced`. Its inputs are the row's metadata and the receipt's `sources[].section_terms`, which G1 has already bound by hash and G2 by encoding.
  - With `reduced` present, the member's reduced rows are in the universe, and row indices follow published order.
- **New refusals and where they sit.** A `reduced` present where the evidence's sections agree, or absent where they differ, cannot be decided before the evidence is read at G5b. So:
  - G3 refuses, with its existing coverage code, a universe whose rows and indices do not match the receipt's row lists. That catches a dropped or duplicated reduced row.
  - G5b refuses `SECTION_MISMATCH` where `reduced`'s presence or values disagree with the evidence's sections, or where source and selection terms disagree.
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
- **Python verifier.** It mirrors each of these with its own arithmetic.
- **Desktop.** No TypeScript change. The desktop reads through the Rust reader (wasm), and test support takes the new DEF-E hash.

### 4.8 DEF-E, tables and texts

- **DEF-E** (`retained_precision_prepared_exact_v1.json`, reviewed input 7; sorted-key compact form kept):
  - `scope.shear_model`: "explicit euler_bernoulli or timoshenko_annulus_energy_v1 per document; straight members only (realized arcs never reach this route)".
  - `scope.corrosion_allowance` and `scope.mill_tolerance`: "explicit user-owned length on every used local pipe section, finite and nonnegative with c_m + c_a < t; the D-5 nominal section (wall t) carries stiffness and the Timoshenko shear area, and every stress row is certified on its own basis section".
  - `lanes`: section 4.2's Timoshenko D recipe and D̄, the prepared shear-area law, section 4.3's admitted and annular shear rigidities and the D-entry gap majorant at the exact length.
  - `rows`: reduced stress rows "certified on the reduced section with the nominal row's recipe where the two sections differ in As, Z, I, J or wall; the nominal row's retained value where they agree".
  - `evidence`: the per-basis prepared overlay and the prepared shear area.
  - `trust.G8`: the rederivation of both bases and the prepared Timoshenko leaf, in place of the zero-allowance and Euler–Bernoulli wording.
  - Its meaning hash moves in the producer (`retained_wire.rs`), the Rust reader, the verifier, the physics-retained table's `product_formation_definitions` and desktop test support.
- **physics-retained-1 table** (reviewed input 8): only its DEF-E hash. TBD-002's `accuracy_classification.scope` sentence is not touched; if any edit would need it, the edit goes to HELP_HUMAN first.
- **pressure-source-1 table** (reviewed input 9, as epoch 4 revises it): main's table has no Euler–Bernoulli or allowance text. If epoch 4's revision adds none, O4 does not move it.

### 4.9 Which committed bytes move

- **Limitation strings.** The limitation strings of section 3.3 move on every exact envelope that carries them:
  - every ordinary pressure-2 envelope and its corpora (`pressure_v3_{straight,arc,connector}_reader_corpus.json` among them);
  - every pressure-source-1 raw and derived document (`fixtures/product_preview/pressure_source/**`, `load_reference_source`, the stress-neutral exports);
  - J2's bases;
  - MJ6 and MJ9's captured bases;
  - J7's published envelopes and consequence digests;
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

## 6. Mutants and killing tests

Each is introduced into a scratch copy, run, reverted and recorded in the PR; nothing is a committed seam. Tests are new unless marked existing.

| Fault | Killed by |
|---|---|
| A Timoshenko document still declined by `prepare_sources` or G-A | `o4_timoshenko_source_case_selects_and_publishes`, `o4_w1e_timoshenko_case_is_admitted_and_selected` (replace the existing `o4_prepare_sources_declines_by_name` and `o4_w1e_declines_at_admission`) |
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
| Work delta misaccounted | `o4_source_work_deltas_follow_the_d5_law` (independent derivation from the serialized identity and the actual sites, as MJ9; asserts ΔP = 0) |
| Timoshenko D-entry gap formed at `lower_length`/`ell` | `timoshenko_gap_is_formed_at_the_exact_length` (a skew member at φ ≈ 1 whose gap at ℓ < L underestimates the exact gap) |
| `StraightMember` or another receipt-counted layout grown | `timoshenko_sidecar_preserves_receipt_layouts` (size and layout pins; full adapter counters of every committed route-L and route-E receipt unchanged) |
| Shear leaf from the ordinary area after preparation | `prepared_shear_area_follows_the_prepared_area` (the D = 0.168 m, t = 0.007 m, ν = 0.3 control, where the two differ by an ulp; G7 accepts only the prepared leaf) |
| Bit-equality tested without the wall | `o4_overlay_equality_includes_the_wall` (a discriminator where t_r ≠ t but A, Z, I and J round equal: the twin is certified, not overlaid) |
| Reader keeps dropping reduced rows by metadata | corpus mutations `reduced_rows_dropped_with_reduced_term` (G3) and `reduced_term_on_equal_sections` (G5b `SECTION_MISMATCH`); precedence pinned in Rust, wasm and Python |
| A regenerated envelope differs beyond the limitation text | `o4_text_only_envelope_diff` (4.9) |

## 7. D3 and pricing

M = 12 GiB, ⌊0.9 M⌋, R = 64 MiB and the upward 5 % TEXT reserve are unchanged (Decision 22).

**Reviewed inputs that move:**
- #2 `retained_precision_mp_v2.schema.json`: both section shapes (4.6);
- #7 DEF-E;
- #8 the physics-retained table: the DEF-E hash.

**Inputs that may move:**
- #1 is expected unchanged (3.3).
- #9, #10 and #11 move if they carry the edited limitation strings; on main, the pressure tables' `supported_profile_limitations` do not carry limitation 7.

The actual-source D3 binds whatever moves. #0 (`Cargo.lock`) does not move, because no crate edge is added.

**New positive owners** (each with a rule argued from code facts in `retained-pricing-rules.md`, a source pin and independent review):
- the `TimoshenkoLeafSet` sidecar, its view and its preparation record, at the cap with every member Timoshenko, and `MemberOperators`' and `BoundedCoefficients`' D̄ and D entries, at every precision a member is formed, times the existing member multiplicities;
- the κ_E and D-entry enclosures in both lanes, and the η and bridge D-entry gaps;
- the reduced section facts per member, at the cap with every member distinct;
- the receipt entries of distinct-basis reduced rows (`absolute_verified`/`not_covered` items and row indices), at the cap with every member distinct (21 stress rows per member, plus U4b's derived reduced rows listed under `not_covered`);
- the source route's private `reduced_recovery_section` and `shear_area_bits`, the second endpoint-maximum evaluation and the per-basis evidence entry;
- TEXT: the new and edited literals (the limitation strings included), and the deletion of the three decline texts, `MILL_TOLERANCE_DECLINE` and `SOURCE_MAXIMUM_BASES_DISTINCT`. `RETAINED_TIMOSHENKO_REFUSAL` stays as the association text (4.2);
- the narrowed admission pins (`batch5_model.py`, `b7a_domain_model.py`).

**Feasibility.** O4 adds no published row and no D1 domain. Its new storage scales with members, not rows, apart from the receipt entries of distinct-basis twins. Route E's published-row premise (E4-8) already counts the twins. Whether the receipt-entry premise does is a fact the actual-source D3 establishes.
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
4. The dual basis on both routes (3.2, 4.5): each reduced row on its own section, the bit-equal overlay rule, the per-basis maxima and headlines, and the reader bindings.
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

**DEL-04-08 ScopeOfWork AC-002** (only if the owner reads "the RF census classification and its pins are unchanged" as covering J7's envelope digests): "the RF census classification, counts, first causes, selections and frozen-row verdicts are unchanged; its envelope digests change only as a stated product-text or identity change requires." This is the practice U6 and epoch 4 followed (E4-12).

**DEL-04-01 Design** (when O4 lands; it is the kernel owner's design): a pointer from the retained-kernel description to section 4.2's Timoshenko member and section 4.4's lemma.

**DEL-05-03 Design `section-and-shear-decisions.md`,** "Retained-source and W1-E publication (O4)". Add a pointer line: "As designed: DEL-04-08 Design `o4-source-extension.md`." The as-built text follows when O4 lands.

**b7-plan** (Agent 0 adds the pointers when these designs land):
- "After B7b", item 5. Append "Design: [o4-source-extension.md](o4-source-extension.md)."
- Item 6. Append "Design: [w1-coverage-repair.md](w1-coverage-repair.md). Sequenced after O4; the two do not merge (o4-source-extension.md §8)."

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

**Reserved, for the owner through Agent 0:**
- **AC-002's reading** (one reading should cover epoch 4 as well, which moves the same envelopes for its identity rename and does not raise AC-002 itself). O4's limitation-text change moves J7's four published envelope digests and its consequence digests, with every classification, count, first cause, selection and frozen-row verdict unchanged. If "the RF census classification and its pins are unchanged" covers those digests, O4 (like U6 and epoch 4) needs either the owner's reading or the AC-002 wording proposed in section 10.
  - *Consequence if neither:* the limitation text cannot change. It would then keep stating the O4 declines after O4 removes them, which is false product text, so O4 could not land.

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
- **Product physics (DEL-04-07):** `source_recovery.rs` (declines removed), `source_receipt/rows.rs`, `source.rs` and `composite.rs` (per-basis sections and maxima), `lib.rs` (`SOURCE_MAXIMUM_BASES_DISTINCT` removed), `exact_admission.rs` (limitations), `retained_memory.rs` (admission; registered profile), `retained_product.rs` (capture, facts, overlay, staging), `retained_wire.rs` (Section members; DEF-E hash), and the facade, census and source tests.
- **Readers (DEL-05-02 shared, DEL-04-08 owned):** RE `retained_precision.rs` (G5b, G8), `physics_source.rs` and `source_blocks.rs`; `tests/verifiers/retained_precision.py` and `physics_source.py`; `semantic_contract.rs` pins.
- **Schemas and definitions:** reviewed inputs #2, #7 and #8, and possibly #1 and #9–#11 (section 7).
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
