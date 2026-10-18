# O4: the Timoshenko and corrosion-allowance source extension

This is the implementation design for the fifth post-T3 shared epoch ([b7-plan.md](b7-plan.md), "After B7b", item 5). It extends retained-source recovery (`pressure-source-1`) and W1-E publication (`physics-retained-1`) to the exact-contract documents that DEL-04-08 ScopeOfWork REQ-006 (a) declines today: documents whose contract selects `timoshenko_annulus_energy_v1`, and documents with a nonzero mill tolerance or corrosion allowance on any pipe section. It keeps RF's closure, witness and charging rules, and it keeps every gate, limit, tolerance and frozen reference unchanged.

**Status: designed, not implemented.** The basis is main `ca8d6d5c23` plus the epoch-4 design (DEL-04-07 Design `u4b-u5b-implementation.md` at `c2e450ac7f` on `claude/piping-t4-u4b-u5b-20261017`). Epoch 4 is not merged. Section 0 names every assumption taken from it; they are re-checked against the merged epoch before implementation starts.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

**Commission.** The owner, 2026-10-12 (O4): "Scope extension after B7b". The owner's resumption steer of 2026-10-14 orders the shared epochs "each under its own design and independent review". DEL-04-08 ScopeOfWork REQ-007 (a), which the owner approved on 2026-10-17 ("I approve #1405 as required"):

> O4: the Timoshenko and corrosion-allowance source extension. This is the fifth shared epoch in the owner's order of 2026-10-14. It extends retained-source and W1-E publication to the documents REQ-006 (a) declines, including those with a nonzero mill tolerance. It keeps RF's closure, witness and charging rules.

AC-007 (a) gives the discharge conditions: the extended cases publish verified results under REQ-001 or REQ-002; the retired decline tests are replaced by admission and refusal tests; AC-001 to AC-005 pass on the head with every gate, limit and frozen reference unchanged; the actual-source D3 registration passes.

## Summary

- **Retained-source recovery (sections 3).** The source method's declared operand is already the built frame's binary64 local matrix. A Timoshenko frame supplies its own Timoshenko local matrix, and the signed-permutation path and RF's exact congruence carry it unchanged. The dual-basis work is in the stress publication: each basis has its own recovery section, stress checks and endpoint maximum, all formed from the same retained endpoint functionals. `SOURCE_MAXIMUM_BASES_DISTINCT` and the three `prepare_sources` declines are retired.
- **W1-E (section 4).** The retained kernel gains a Timoshenko straight member. Its D block is written in natural-mode form, (EI/L)·P_a + 3(EI/L)/(1+φ)·P_s per bending plane, with a fixed formation recipe at the working precision. Each lane forms the shear rigidity from its own readout. The admitted lane uses the binary64 shear-area leaf; the annular lane uses an enclosure of κ_E from the exact annulus. Wherever the kernel bounds D by coefficient gaps and fixed patterns, a Timoshenko member is bounded by D-entry gaps; Euler–Bernoulli members keep today's code and bits. A member whose two section bases differ gets a reduced section term, and its reduced stress rows are certified on that section as first-class receipt rows. A member whose bases are bit-equal keeps today's overlay of the reduced twin.
- **Admission and readers.** W1-E's G-A admits both shear models and nonzero allowances. The retired facts are `ShearModel` and `SectionAllowance`. G8, G5b and the source readers stop requiring Euler–Bernoulli and zero allowances. In their place they rederive the D-5 bases and the Timoshenko leaf from the captured invocation and bind them, through the rederivation the pressure-2 readers already have (DEL-05-03 `section_basis`).
- **Euler–Bernoulli, zero-allowance documents keep their bytes.** Every new receipt field and encoding block is present only where the document needs it. The overlay rule stays for bit-equal sections. Existing route-E successors, J2's bases and the J7 census pin therefore move only where a reviewed-input digest they bind moves (DEF-E, section 4.8).
- **Proof obligations (section 4.4).** The Timoshenko formation must fit R7's unchanged t₁ = 2^(7−q_W)·B_b·N_u. The corrected recovery must fit the unchanged factor 69 under P − 16. The alpha law must hold with D-entry gaps. If any derivation fails, the epoch returns to HELP_HUMAN with the concrete conflict; no law changes to obtain a pass.
- **Independent references (section 5).** A separate, later reference unit (`validation/references/o4_retained_source/`) judges the new publications against exact solutions of both readouts. It reuses the frozen, independent `kd5_timoshenko` engine, and an independent exact solver checks each source receipt's declared problem.
- **Merge with the W1 coverage repair: no (section 8).** The two share no numerical law. O4 lands first, as b7-plan orders, and the repair's census runs as soon as epoch 4 merges. Two coupling rules keep the shared files consistent.
- **Reserved:** the ScopeOfWork edits that discharge REQ-007 (a) (proposed in section 10), any Decision 22 change if D3 does not fit, and TBD-002's table sentence, which stays untouched.

## 0. Basis and epoch-4 dependencies

O4 is designed against main `ca8d6d5c23` and against the epoch-4 design as written, not as built. Each assumption below is re-checked against epoch 4's merged head before O4's implementation starts; a failed assumption revises this design first.

| # | Assumption from epoch 4 (u4b-u5b-implementation.md) | Where O4 relies on it |
|---|---|---|
| E4-1 | Model 0.6.0 is the only current version. 0.5.0 is refused by name, and behaviour keys on presence (§1.1). The literal `"0.5.0"` in `SourceContract::from_invocation` (`physics_source.rs`) and `exact_namespace` (`retained_precision.rs`) becomes 0.6.0, and `namespace_branch` keeps using `MODEL_SCHEMA_VERSION`. | Sections 3.4, 4.1, 4.7: O4 edits the contract predicates next to those literals, not the version. |
| E4-2 | `pressure-2` replaces `pressure-1`. `physics-retained-1` inherits pressure-2's table by hash. `pressure-source-1` keeps its name and table. `results_pressure_1.v0.3.schema.json` is renamed `results_pressure_2.v0.3.schema.json`. `REVIEWED_INPUTS` keeps its order, with indices 10 and 11 renamed (§1.2–1.3). | Sections 4.8 and 7: the inputs O4 moves are named by their epoch-4 names. Limitation 7 of the unselected exact envelope is pressure-2's. |
| E4-3 | U4b's σ⁺/σ⁻ and Saint-Venant rows are published per station and basis. In retained successors and source publications they are `not_covered` (W1, `ProductRecipe::NotCovered`) or `derived_not_qualified` (source). They are formed from the publication's own published rows (owner, 2026-10-17, option (ii); §4.1, §4.3). | Sections 3.2 and 4.5. Where O4 makes the reduced stress rows distinct, the reduced derived rows follow automatically, because they are formed from the reduced basis's own published rows. O4 adds no recipe for them. |
| E4-4 | The ordinary K-D5 route publishes the corrected displacement fl(u + w + w2) wherever a certified bound exists (HELP_HUMAN, 2026-10-18). The Passed/Sensitive classification is unchanged. | Retained-source eligibility (ordinary Sensitive or errs) and W1-E's trigger do not move. Only the ordinary fallback bytes that O4's fallback tests compare against move, and those are taken from the merged producer. |
| E4-5 | U5b's tie rods live on objective connectors. D7 still declines connectors at `prepare_sources`, and W1 capture still declines them. | O4 does not interact with tie rods. |
| E4-6 | Epoch 4 re-registers under D3 (§6). Its registered headroom is unknown here. Before epoch 4, the tightest phase was merged dense W4 at +113,188,985 B (W1 registration). | Section 7: O4's D3 starts from epoch 4's registered profile. |
| E4-7 | DEF-O, DEF-C and DEF-E name preview-physics-2 and pressure-2 in `inherits.base_contract`. DEF-E's `scope.model_format` takes 0.6.0. Their meaning hashes move in epoch 4 (§1.2). | Section 4.8 edits DEF-E on top of epoch 4's bytes. |
| E4-8 | Route E's row premise is P_final = 7n + 102m + 8g + 3 (72 + 30 rows per member, two bases). Route E stays below its 16,384 row capacity at the D1 caps (§6). | Section 7. O4 adds no published row. It turns overlay twins into receipt rows only for members whose bases differ. |
| E4-9 | Epoch 4 edits REQ-001 and REQ-002's version tokens and REQ-001's value clause (§4.4). It leaves REQ-006 (a), REQ-001's other route-E bullets and AC-003 to this epoch. | Section 10's proposed text is written against epoch 4's edited ScopeOfWork. |
| E4-10 | On pressure-2 the Saint-Venant factor c_b uses the member's authored ν and each basis's own annulus. Route E recomputes it from the captured evidence (§2.4, §4.1). | Section 4.5. With distinct bases, the two c_b differ, and that follows from the evidence. |
| E4-11 | Epoch 4 adds no W1-E admission clause. It moves no W1 or source decline. | Section 1's inventory is main's, plus epoch 4's renames. |

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
- **Formation inputs are named.** The normalized-source payload's per-frame `formation_inputs` (`source_receipt/source.rs` about 317) gains `shear_area_bits`, present only on a Timoshenko frame. The `Identity` hash gains one block, after the frame's section scalars and only on a Timoshenko frame: the name `timoshenko_annulus_energy_v1` and the shear-area scalar. The local-matrix bits already bind the matrix; this block names its formation leaf.
  - An Euler–Bernoulli frame's payload, identity and `normalized_source_sha256` keep their bytes.

### 3.2 Two section bases

Every stress row already exists per basis on this route (U6 §2.1). Today the reduced row is the nominal row bit for bit. With distinct bases it is formed on its own section:

- **Recovery section.** The per-frame normalized-source `recovery_section` stays the nominal section. A new `reduced_recovery_section` is present exactly when the member's reduced stress section differs from its nominal one in any of A, Z, J or r_o (`same_stress_section` false). It has the same keys: `section_modulus_bits` and `torsion_radius_bits`, plus `area_bits` and `torsion_constant_bits`. That key set is the parameter set the stress checks bind. The nominal object keeps its keys.
- **Stress checks.** `section_stress_checks` stay one per stress row per basis. The reduced check binds the same functional (index and id) and the same action, bit for bit. Its `parameters` are the reduced section's: `area_m2` = A_r, `section_modulus_m3` = Z_r, `torsion_constant_m4` = J_r, `torsion_radius_m` = r_o, `pa_per_mpa` = 1e6. Its value is the existing `retained_source_straight_stress_v1` evaluation with those parameters.
  - `rows.rs`'s "derived stress section bases differ" refusal is replaced by a per-basis loop over `exact_basis_section(bases, basis)`.
- **Endpoint maximum.** `composite_member_maximum` (`source_receipt/composite.rs` about 165) evaluates `endpoint_maximum` once per basis from the same retained endpoint functionals [0, 4, 5] and that basis's A and Z. Each basis's `pipe_stress_extrema` entry (keyed by `section_basis`) carries its own value and evidence.
  - `SOURCE_MAXIMUM_BASES_DISTINCT` and its withholding branch in `lib.rs` are deleted.
  - The receipt's `endpoint_sections` (`composite.rs` about 947) stays one per member. Endpoints, functional indices and actions are basis-independent.
- **Headlines.** `validate_composite_summary` already binds both per-basis headlines. Each now binds its own basis's maximum rows.
- **U4b's derived rows** (σ⁺/σ⁻ and τ_v,max, `derived_not_qualified`) are formed from each basis's own published rows (E4-3). With distinct bases they differ by basis. That follows from the formula and needs no change here.
- **Bit-equal bases keep their bytes.** A document whose sections are bit-equal (zero mill tolerance and allowance, or values that round away) produces exactly today's bytes: no `reduced_recovery_section`, and checks and values as today.

### 3.3 The receipt and schema

- **Schema.** `physics_source_recovery.schema.json` (reviewed input 1) gains the optional `reduced_recovery_section` object (closed, the four keys) and the optional `formation_inputs.shear_area_bits`. Both are absent unless needed, so committed receipts stay valid and unchanged.
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
| Normalized source | `reduced_recovery_section` is present exactly when the evidence's two stress sections differ, and it equals the reduced section's bits. `shear_area_bits` is present exactly on a Timoshenko member, and it equals the rederived A_s. |
| Composite maximum and headlines | Per basis, against that basis's section (the readers' existing per-basis lookups key on `section_basis`). |

The readers still do not re-derive the exact solve, the closure or the charges (RF §7). The verifier stays independent and spawns nothing. `tests/test_verifier_independence.py` is unchanged.

### 3.5 Work accounting

The D5/MJ9 law (b7-plan, "D5/MJ9's accounting clarification") governs. For a selected case of a newly admitted document:
- the case charge changes by 24ΔE + ΔF_k + ΔQ_k;
- the publication charge changes by 24ΔE + ΔP;
- the invocation total changes by their sum.

Where the terms come from:
- ΔE is the encoded-request delta: a nonzero allowance and the shear-model token are real request bytes;
- ΔF_k is the new normalized-source and row-metadata bytes inside finalization: `reduced_recovery_section` and `shear_area_bits`;
- ΔQ_k is the second endpoint-maximum evaluation, charged by its existing site;
- ΔP is unchanged in row counts, because no published row is added.

An independent test derives every term, as the MJ9 test does. Every Euler–Bernoulli, zero-allowance control keeps its exact work and bytes, including MJ6's and MJ9's twelve requests in both modes. The 8,000,000 and 64,000,000 limits are unchanged, and an input at the limit still refuses by name.

### 3.6 What stays

- **RF's census is unchanged.** The J7 RF census re-authors every case to explicit Euler–Bernoulli with zero allowance (RF §10 item 1), so its classification, pins and four public envelopes do not move.
- **`load_reference_fallback_uz` keeps its role.** It stays an ordinary pressure-1 raw (pressure-2 after epoch 4) through its zero member load's non-nodal decline, which O4 does not touch (W1 stage 2 §8; RF §5.5).
- **D7's connector and bend declines** keep their text, order and work.

## 4. W1-E (`physics-retained-1`)

### 4.1 Admission

- **`namespace_branch`.** `contract_is` admits `{version "3.0.0", mode "exact_pressure_v3", shear_model ∈ {euler_bernoulli, timoshenko_annulus_energy_v1}}`. An absent or unknown shear model stays `PressureContract`. `FamilyFact::ShearModel` is deleted.
- **`family_clauses`.** The `SectionAllowance` refusal is deleted.
- **Unchanged:** every other D1 clause, including `ReferenceConfigurations`, the expansion laws, the modulus-basis and load facts and the caps. The pricing tool's admission pins (`base/batch5_model.py`, `base/b7a_domain_model.py`) re-pin to the narrower refusal set under D3.

### 4.2 The retained Timoshenko member (kernel)

**Representation.** `StraightMember` gains `beam_theory`, a two-variant leaf: `EulerBernoulli`, or `Timoshenko { shear_area: f64 }`. `from_frame` copies the frame's `FrameSection::beam_theory` bit for bit; the `Timoshenko` refusal variant and `RETAINED_TIMOSHENKO_REFUSAL` are deleted.

**K4STF and K4SRC.** After the stiffness part, a Timoshenko block is appended only when at least one member is Timoshenko: the count, then each Timoshenko member's id and shear-area bits. A source whose members are all Euler–Bernoulli keeps its K4STF and K4SRC bytes, so `kernel_source_sha256` and the stiffness-group identity of every existing receipt stay as they are.

**Formation at the working precision** (`assemble.rs` `form_member`; at p, q, 2p and q_W, wherever the member is re-formed). The frame, B and the axial and torsion coefficients are today's. Per bending plane, with c = the existing `bend_z` or `bend_y` = RN(RN(E·I)/L):
1. s = RN(G·A_s): one exact product, rounded once;
2. sl = RN(s·L);
3. t₃ = RN(3·c);
4. φ = RN(4·t₃ / sl): 4·t₃ is an exact scaling;
5. op = RN(1 + φ);
6. c_s = RN(t₃ / op);
7. d₁ = RN(c + c_s), d₂ = RN(c_s − c).

D's bending rows become [(2, d₁z), (3, d₂z)], [(2, d₂z), (3, d₁z)], and likewise for y. This is the exact identity (EI/(L(1+φ)))[[4+φ, 2−φ],[2−φ, 4+φ]] = (EI/L)·[[1,−1],[−1,1]] + (3EI/(L(1+φ)))·[[1,1],[1,1]]: the antisymmetric mode keeps EI/L, and only the symmetric, shear-carrying mode softens. φ and its siblings are formed from the member's own leaves.

- **φ = 0 is not this recipe.** t₃ = RN(3c) is inexact. An Euler–Bernoulli member is never formed through the Timoshenko recipe; it keeps today's `4c`/`2c` exact scalings, code path and bits. The branch is taken from the leaf.
- **d₂ may be zero or negative** (φ ≥ 2). It is a signed entry like any other, and no range check treats an exact zero as underflow (U8 §3.3).

**The bounded operator Ā and E's operands.** For a Timoshenko member, the absolute D operand used by `bounded_block` (Ā, which feeds N_u and t₁) and by E's recovery bounds is D̄ = d̄·[[1,1],[1,1]] per plane, with d̄ = RU(c + c_s) ≥ max(|d₁|, |d₂|). Each formation error of d₁ or d₂ is then a multiple of u·d̄, with no division by a possibly cancelled |d₂| (section 4.4). Euler–Bernoulli members keep [[4c,2c],[2c,4c]].

**Layouts.** `StraightMember` grows by the leaf (an enum with one f64 payload). `MemberOperators` gains the per-plane d̄ and the two D entries per plane. Both are priced (section 7).

### 4.3 The product certificate's lanes

The lanes certify every published row against both physical readouts (REQ-001: "The bound holds for both of the formation's stiffness readouts"). For a Timoshenko member each lane forms its own shear rigidity:

- **admitted_k:** K_s = G·Â_s, the exact product of the binary64 shear modulus and the frame's binary64 shear-area leaf (U8 §3.1's fixed-order κ̂·Â_n, formed by PP and bound in K4STF).
- **annular_source:** K_s = g·κ_E·A_exact, from directed 1024-bit enclosures of
  κ_E = 6(1+ν)²(1+m²)² / ((8ν²+14ν+7)(1−m²)² + 48m²(1+ν)²), with m² = r_i²/r_o² formed exactly from the lifted D/2 and the nominal wall, the material's ν (the exact E/ν route's operand), the lane's existing exact annulus area A and its existing g.
  - The denominator is positive for every real ν: 8ν² + 14ν + 7 has a negative discriminant, and 48m²(1+ν)² ≥ 0. So the enclosure never straddles zero.
  - This is the exact-geometry counterpart of the binary64 leaf, as the lane's A, I and J are of theirs.

Each lane forms φ, d₁ and d₂ from its own enclosures with directed arithmetic. Its member law uses them wherever it uses D today: lane matrix-vector products, recovery Q = D·e, end actions and stations.

**Coefficient gaps become D-entry gaps for Timoshenko members.** The alpha law's η and the bridge's majorants (`bridge.rs` `Majorants.dd`, `d_apply`; `source_residual.rs` `eta_member`, `D_PATTERN`) bound K_G − K_K by B̄ᵀ·(coefficient gap × fixed pattern)·B̄. For a Timoshenko member, |d₁,G − d₁,K| and |d₂,G − d₂,K| are formed per plane as outward maxima over the two lanes' enclosures. They enter B̄ᵀ|ΔD|B̄ in place of the pattern term.
- This is sound for any ΔD by the entrywise triangle inequality |Bᵀ ΔD B| ≤ B̄ᵀ|ΔD|B̄. No new premise enters: the premise ‖A_K^{-1}‖∞ ≤ β_b is the alpha law's own.
- Euler–Bernoulli members keep the coefficient-gap computation, its visits and its bits.

**Not involved:** the N06 congruence radius law applies only to DEF-O's preview proof on route L (b7-plan, "N06 absorbed-spring certificate extension"), and route L carries no Timoshenko member. DEF-E keeps the alpha law alone.

### 4.4 Proof obligations (for the independent numerical review)

These are derivations, not new laws. If any one cannot be established with the factors stated, the epoch returns to HELP_HUMAN with the concrete conflict (AX-005: "do not change the law to obtain a pass").

1. **t₁ covers the Timoshenko formation.** R7's verification formula t₁ = 2^(7−q_W)·B_b·N_u (`verify.rs` about 1183) covers K's formation error at q_W relative to Ā. The review must count the per-entry formation error of the section 4.2 recipe at precision q in units of u_q·(Ā entry). Each of s, sl, t₃, φ, op and c_s is one rounding. The relative error of op = 1 + φ is at most u + (φ/(1+φ))·err(φ). d₁ and d₂ each add one rounding, with errors bounded by u·d̄. Then come B, DB and BᵀDB as today, including higher-order terms. The review shows the total is within the 2^7 the existing constant allows.
2. **E's factor 69 under P − 16.** The full recovery DAG (local projection, strain, Q = D·e with Timoshenko d₁ and d₂, end actions, stations, reactions) must satisfy |q_P − R*(u_P)| ≤ 69·2^-P·E_q, with E formed from D̄, as W1 stage 2 §4.2 derived for member loads. Route E has no member-load terms, so the derivation is stage 2's displacement-dependent part plus the D-entry formation. Stage 2's count for that part is 45.008 ε-units for raw shear. The design estimates that the Timoshenko D entries add about eleven units at Q. That is an estimate, and the review replaces it with a derivation.
3. **The alpha law with D-entry gaps** (section 4.3): its premise, the triangle inequality and the outward rounding of |Δd|.
4. **κ_E's enclosure** contains the exact κ_E of the exact annulus for every admitted ν in (−1, ½). The exponent range of every intermediate stays within the lane's `Wide` range, or refuses by `ExponentRange`.
5. **K4STF's block** binds every leaf that enters the formation, so no attempt, group or replay can certify a member whose formation differs from its declared leaves.
6. **The Timoshenko frame's six rigid modes and pure-curvature patch at p.** These are properties of the exact element, and the formation is checked against them by the judged set (section 5).

### 4.5 Two section bases on route E

- **Section facts.** Each member's product facts gain an optional reduced section, present exactly when the reduced stress section differs bitwise from the nominal one:
  - binary64 A_r, Î_r, Ẑ_r and J_r, the prepared values for the reduced wall;
  - for the annular lane, the exact annulus of D and the binary64 reduced wall t_r = (t − c_m) − c_a.
  - The nominal facts are today's, with the nominal wall t. With a nonzero mill tolerance the stiffness, κ_E and the nominal stresses use t; today's route E, which admits only zero mill tolerance, already uses t.
- **Reduced stress rows.** Each reduced stress row is certified by the same recipes as its nominal twin: DEF-E's directed signed-corner N/A, M_y/Z, M_z/Z and T·c/J, with the admitted lane's represented Z = hull(Ẑ_r, Î_r/c). The two-end maximum recipe is likewise applied with the reduced section's A_r and Z_r. Each row gets its own enclosure, class and bound under the unchanged gate. Its body scale is that of its body's force and moment kinds, divided by its own section (the reader's `row_scale`).
- **The overlay stays for bit-equal sections.** Where a member's two stress sections are bit-equal, the reduced twin is today's overlay: excluded from the receipt row universe (`reduced_twin`, `row_index`) and carrying its nominal twin's retained value. Two bit-equal sections give the same recipe and the same operands, so the twin is the same certified quantity, and listing it again would add nothing.
  - The rule is decided per member from the data. It is not a document flag: a mixed document certifies distinct-basis members' twins and overlays the rest.
  - Existing route-E successors (zero allowances) keep their row universes and bytes. That includes RV120's exact shapes and the two route-E successor fixtures.
- **PP's staging.** `apply_exact_section_overlay` writes each basis's prepared section to its own evidence object, nominal to `nominal_section` and reduced to `reduced_section`; today it writes the one prepared section to both. `stage_headlines` already selects each basis's maximum rows. U4b's derived rows on route E are formed from each basis's own projected rows (E4-3).
- **The Timoshenko evidence** (`beam_theory`, κ and A_s) is not overlaid. G7's pressure-2 base read and G8 bind it by rederivation.

### 4.6 Receipt and SCHEMA (reviewed input 2)

`$defs/Section` (the `section_terms` items, in `sources[]` and in `cases[].selection`) gains two optional, closed members:
- `shear_area` (Bits): present exactly on a Timoshenko member;
- `reduced` {`area`, `section_modulus`, `geometry` {`effective_wall`, `actual_second_moment`, `actual_polar_moment`}}: present exactly when the member's two stress sections differ. Its keys and encodings are the nominal entry's.

Receipt rows for distinct-basis reduced twins enter `absolute_verified`, `not_covered` and every row-indexed list exactly like other rows, in published order. Nothing else in the receipt changes shape. An all-Euler–Bernoulli receipt with bit-equal sections has no new key.

### 4.7 Readers and verifier

- **G8.**
  - `exact_namespace` admits either shear model (section 4.1). It keeps the three-member contract, no top-level `shear_model`, and an explicit, finite, length-valued allowance ≥ 0 with c_m + c_a < t on every section (the producer's own refusals). `section_allowances(model, true)` stops requiring zero.
  - `exact_mill_tolerances` and `MILL_TOLERANCE_DECLINE` are deleted.
  - G8 rederives each member's nominal and reduced sections (`section_basis`'s D-5 rederivation) and its `beam_theory`. It binds the receipt's `section_terms[].shear_area` to the rederived A_s, and `reduced` to the rederived reduced section.
  - It rebuilds K4SRC and K4STF with the Timoshenko block. Euler–Bernoulli members keep their checks. The B7b-ZA per-section allowance loop stays bounded by m.
- **G5b.**
  - The body-scale check is unchanged.
  - The section check binds `reduced` like the nominal entry, when present.
  - `row_scale` uses a reduced row's own section.
  - `g5b_exact_evidence`, per member: with `reduced` absent, the evidence's two sections must be bit-equal and the twin overlay rule applies (today's check). With `reduced` present, the evidence's sections must differ, `reduced` must equal the evidence's `reduced_section` (As, Z, I, J, wall), and the reduced rows are receipt rows, not overlays. A `reduced` on a member whose sections are equal is refused, and so is a missing `reduced` where they differ (`SECTION_MISMATCH`).
- **G5c** classifies a distinct-basis reduced row like any other stress row.
- **G7** runs the pressure-2 base reader, which already checks both bases and Timoshenko evidence.
- **Python verifier.** It mirrors each of these with its own arithmetic.
- **Desktop.** No TypeScript change. The desktop reads through the Rust reader (wasm), and test support takes the new DEF-E hash.

### 4.8 DEF-E, tables and texts

- **DEF-E** (`retained_precision_prepared_exact_v1.json`, reviewed input 7; sorted-key compact form kept):
  - `scope.shear_model`: "explicit euler_bernoulli or timoshenko_annulus_energy_v1 per document; straight members only (realized arcs never reach this route)".
  - `scope.corrosion_allowance` and `scope.mill_tolerance`: "explicit user-owned length on every used local pipe section, finite and nonnegative with c_m + c_a < t; the D-5 nominal section (wall t) carries stiffness and the Timoshenko shear area, and every stress row is certified on its own basis section".
  - `lanes`: the Timoshenko D recipe of section 4.2, the admitted and annular shear rigidities of section 4.3, and the D-entry gap majorant.
  - `rows`: reduced stress rows "certified on the reduced section with the nominal row's recipe where the two sections differ; the nominal row's retained value where they are bit-equal".
  - `evidence`: the per-basis prepared overlay.
  - `trust.G8`: the rederivation of both bases and the Timoshenko leaf in place of the zero-allowance and Euler–Bernoulli wording.
  - Its meaning hash moves in the producer (`retained_wire.rs`), the Rust reader, the verifier, the physics-retained table's `product_formation_definitions` and desktop test support.
- **physics-retained-1 table** (reviewed input 8): only its DEF-E hash. TBD-002's `accuracy_classification.scope` sentence is not touched; if any edit would need it, the edit goes to HELP_HUMAN first.
- **pressure-source-1 table** (reviewed input 9): no EB or allowance text was found in it on main, so it is expected unchanged. Its limitation text lives in `exact_admission.rs` (section 3.3).
- **Corpus effect.**
  - The DEF-E hash moves the DEF-E-bound receipt hashes of every route-E successor, under the E14 precedent HELP_HUMAN accepted for DEF-O ("The DEF-O hash moves under the E14 precedent are fine", 2026-10-17). Publication and ordinary bytes do not move. This is not an exclusion from byte preservation; it is the authorized consequence of a reviewed-input change.
  - The retained corpus has no exact base (b7-plan), so its bases do not move.

## 5. Independent references and judging (a later, separate reference unit)

The certificate certifies the problem the retained kernel forms. It cannot see a wrong but well-defined formation, such as a Timoshenko recipe with a wrong factor. As T4-U8 did for K-D5 (M3-T), O4's new publications are judged by an exact reference that reads no product code. That reference is a separate unit, derived and frozen by a TASK that neither writes nor reads the product implementation, and refuted by a second TASK, before any product output is compared with it (U6/U8 §7, "Independence"). It is not generated in this design PR.

**`validation/references/o4_retained_source/`** contains:
- **The engine:** the frozen `validation/references/kd5_timoshenko/kd5t_engine.py`, used unchanged. Its exact element is SHEAR_REFERENCE's, with κ_E formed exactly, D-5 bases with t_r in the fixed binary64 order, and exact A, I and J. Its solution of a request is therefore W1-E's annular readout.
- **An admitted-readout mode,** added by the new unit's generator, not by editing the engine. It solves the same model with the request's binary64 leaves (prepared A, I, J, Z per basis; κ̂ and Â_s by U8 §3.1's fixed order; G = RN64(E/RN64(2·RN64(1+ν)))) formed exactly. REQ-001's bound is claimed for both readouts, so both judge.
- **A judged set within W1-E's and the source family's domains:**
  - straight-member exact documents with nodal loads, rigid restraints and grounded scalar springs, and explicitly empty pressure regions;
  - Timoshenko and Euler–Bernoulli twins of every design;
  - zero, mill-only, allowance-only and both-nonzero sections, plus values that round away (c_m = 2^-80·t);
  - φ from about 1e-6 to above 10, including φ = 2 exactly where representable;
  - axis-aligned and RF-style congruence (3,4,0) and (1,2,2) members;
  - designs chosen to be Sensitive on the ordinary route (soft springs at k/a ≈ 1e-8 and 1e-12, stiff contrast, short members), so that W1-E and the source method actually trigger;
  - both solver modes, typed and captured entries, single-case invocations inside D1's caps.
- **Twins of the source controls:** Timoshenko and nonzero-allowance re-authorings, in memory, of J2's twelve source requests and of RF's PIN-OFF-345 r1e-08 and r1e-12 cases. The frozen bytes stay unchanged; tests re-author in memory, the t4_u4a precedent.
- **A declared-problem oracle for the source route.** Standard-library Python (`fractions`) reads each published pressure-source-1 receipt's declared operands (formation inputs, local-matrix and transform bits, springs, loads, prescribed values) and solves that declared problem exactly. It asserts that every published projection interval contains the exact value and every stress check recomputes. It also re-forms each Timoshenko K_local exactly from the formation inputs and requires every declared entry within a stated small multiple of u of the exact entry. That catches a wrong Timoshenko formula without depending on FK's operation order. This is RF's P1 method made a maintained oracle.
- **Discriminators,** each at least 1e3 tolerances from the correct value on its listed rows: φ with A in place of A_s, κ on the reduced section, stiffness on t − c_m, a reduced row copied from the nominal row where the sections differ, a nominal-section reduced maximum, a Euler–Bernoulli member formed in a Timoshenko document, Cowper or 5/6 for κ_E, and an annular lane that reuses κ̂.

**Acceptance (M3-O4).**
- Zero judged breaches: every W1-E `relative_verified` row lies within 1e-9·|value| (and DEF-E's sharper allowance) of both readouts' exact values, and every `absolute_verified` row within its published bound.
- Every source projection interval contains the declared problem's exact value.
- Zero strict coverage failures.
- The Euler–Bernoulli zero-allowance twins reproduce today's bytes, which validates the harness.

**Premise census.** On the judged set, the census reports R7 outcomes, alpha_b, t₁'s share of the verification budget and fail-closed counts against the Euler–Bernoulli twins. Fail-closed outcomes are acceptable. A judged breach on a published row that a sound repair cannot remove stops the epoch, and the epoch returns to HELP_HUMAN.

## 6. Mutants and killing tests

Each is introduced into a scratch copy, run, reverted and recorded in the PR; nothing is a committed seam. Tests are new unless marked existing.

| Fault | Killed by |
|---|---|
| A Timoshenko document still declined by `prepare_sources` or G-A | `o4_timoshenko_source_case_selects_and_publishes`, `o4_w1e_timoshenko_case_is_admitted_and_selected` (replace the existing `o4_prepare_sources_declines_by_name` and `o4_w1e_declines_at_admission`) |
| A nonzero mill tolerance or allowance still declined | `o4_dual_basis_source_case_selects`, `o4_w1e_dual_basis_case_is_admitted` |
| Retained member formed Euler–Bernoulli for a Timoshenko leaf; Timoshenko recipe applied to a Euler–Bernoulli leaf | `retained_timoshenko_formation_matches_exact_element` (P = 128/256/512/1024 against the exact element); the existing K4 vectors unchanged |
| φ with A instead of A_s, or with Iy/Iz swapped; factor 3 or 4 wrong; d₂ sign flipped | the same, plus the judged set's discriminators |
| Ā or E using \|d₂\| instead of d̄ (cancellation at φ ≈ 2) | `retained_timoshenko_formation_error_is_within_t1` and `..._recovery_error_is_bounded_by_69uE` (exact/directed error controls at φ = 2 − 2^-40, 2 and 2 + 2^-40) |
| Annular lane reuses κ̂ or Â_s (no exact κ_E) | `annular_timoshenko_lane_uses_the_exact_kappa` (enclosure contains exact κ_E; κ̂ excluded on a section where they differ) |
| Coefficient-gap pattern used for a Timoshenko member's η | `timoshenko_eta_uses_d_entry_gaps` (a member where the pattern bound underestimates \|ΔK\|) |
| K4STF/K4SRC Timoshenko block missing, or present for an all-Euler–Bernoulli source | encoding tests; the existing K4 vectors and route-E successors byte-identical; corpus `section_terms_shear_area_on_euler_bernoulli` |
| Stiffness on t − c_m; κ_E on the reduced section | `o4_w1e_stiffness_uses_the_nominal_wall`, `o4_kappa_uses_the_nominal_section` (judged set) |
| Reduced row overlaid where the sections differ; reduced row certified separately where they are bit-equal | `o4_w1e_distinct_bases_certify_reduced_rows`, `o4_w1e_bit_equal_bases_keep_the_overlay`; reader mutations `reduced_section_term_missing`, `reduced_section_term_on_equal_bases`, `reduced_row_overlaid_on_distinct_bases` |
| Source reduced check using nominal parameters; source maximum reused across bases | `o4_source_reduced_checks_bind_the_reduced_section`, `o4_source_maximum_per_basis`; J2 mutations `reduced_check_nominal_parameters`, `reduced_maximum_copied` |
| A Timoshenko source frame without `shear_area_bits`, or an Euler–Bernoulli frame with it | J2 mutations; `o4_euler_bernoulli_sources_keep_their_bytes` (MJ6/MJ9's twelve requests unchanged) |
| Reader admits an unknown shear model, a negative or over-wall allowance, or an absent allowance | reader unit tests in Rust, wasm and Python (`PHYSICS_SOURCE_ACTUAL_CONTRACT_UNSUPPORTED`, `PHYSICS_SOURCE_ACTUAL_CORROSION_ALLOWANCE`, G8 `RETAINED_PRECISION_INVOCATION_MISMATCH`) |
| Reader accepts a `beam_theory` or `shear_area` that is not the rederived κ and A_s | corpus and J2 mutations `shear_area_changed`, `kappa_changed` (resealed) |
| Limitation text left stating the O4 declines | `o4_limitations_state_the_extension` (pinned bytes; claims review) |
| Work delta misaccounted | `o4_source_work_deltas_follow_the_d5_law` (independent derivation, as MJ9) |

## 7. D3 and pricing

M = 12 GiB, ⌊0.9 M⌋, R = 64 MiB and the upward 5 % TEXT reserve are unchanged (Decision 22).

**Reviewed inputs that move:**
- #1 `physics_source_recovery.schema.json`: the two optional members;
- #2 `retained_precision_mp_v2.schema.json`: `Section`'s two optional members;
- #7 DEF-E;
- #8 the physics-retained table: the DEF-E hash.

#9 and the pressure-2 inputs (#10, #11) are expected unchanged; the actual-source D3 binds whatever moves. #0 (`Cargo.lock`) does not move, because no crate edge is added.

**New positive owners** (each with a rule argued from code facts in `retained-pricing-rules.md`, a source pin and independent review):
- the `StraightMember` leaf and `MemberOperators`' D̄ and D entries, at every precision a member is formed, times the existing member multiplicities;
- the κ_E and D-entry enclosures in both lanes, and the η and bridge D-entry gaps;
- the reduced section facts per member, at the cap with every member distinct;
- the receipt entries of distinct-basis reduced rows (`absolute_verified`/`not_covered` items and row indices), at the cap with every member distinct (21 stress rows per member, plus U4b's derived reduced rows listed under `not_covered`);
- the source route's `reduced_recovery_section`, `shear_area_bits`, the second endpoint-maximum evaluation and the per-basis evidence entry;
- TEXT: the new and edited literals, and the deletion of the three decline texts, `MILL_TOLERANCE_DECLINE`, `RETAINED_TIMOSHENKO_REFUSAL` and `SOURCE_MAXIMUM_BASES_DISTINCT`;
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
2. The obligations of 4.4: t₁ within 2^7, E within 69 under P − 16, the alpha law with D-entry gaps, κ_E's enclosure and range, and the K4STF binding.
3. The two readouts (4.3): the admitted lane's binary64 Â_s, and the annular lane's exact κ_E on the nominal annulus.
4. The dual basis on both routes (3.2, 4.5): each reduced row on its own section, the bit-equal overlay rule, the per-basis maxima and headlines, and the reader bindings.
5. The source route (3.1): that the declared Timoshenko K_local passes every RF closure unchanged, and the declared-problem oracle's verdicts.
6. Readers and verifier: every lifted refusal is replaced by a rederivation, not dropped; refusal precedence is unchanged.
7. Byte preservation: every Euler–Bernoulli, zero-allowance control, MJ6/MJ9, J2's bases, the J7 pin and RV120 keep their bytes, apart from the authorized DEF-E-bound hashes.
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
- the conditional normalized-source, identity, K4STF and receipt members that keep Euler–Bernoulli bytes (3.1, 3.3, 4.2, 4.6);
- the natural-mode Timoshenko recipe and D̄ (4.2);
- exact κ_E in the annular lane (4.3);
- D-entry gaps for Timoshenko members (4.3);
- the data-determined overlay rule, per member (4.5);
- the reference unit's reuse of the frozen kd5t engine (5);
- sequence, not merge (8).

**Reserved and untouched:**
- the ScopeOfWork edits of section 10;
- Decision 22 (M, the margins and the budget model);
- TBD-001 and TBD-002;
- b7-plan's "Held by the owner": G10, B8 and the frozen UI benchmark;
- desktop outputs for the exact identity;
- release.

## 12. Blast radius

**Code**, by owning deliverable:
- **Frame kernel (DEL-04-01):** `structural/retained/` `assemble.rs` (leaf, recipe, Ā), `source.rs` (K4STF/K4SRC block), `adaptive.rs` and `verify.rs` (re-formation sites, E operands), `product_certificate.rs` (κ_E and D-entry enclosures, reduced facts), `product_certificate/source_residual.rs` and `bridge.rs` (D-entry gaps), `final_case.rs` (reduced rows), `retained_resource.rs` (layouts), `structural.rs` (exports, refusal removal), and tests (`retained_k4`, `timoshenko_element.rs`).
- **Product physics (DEL-04-07):** `source_recovery.rs` (declines removed), `source_receipt/rows.rs`, `source.rs` and `composite.rs` (per-basis sections and maxima), `lib.rs` (`SOURCE_MAXIMUM_BASES_DISTINCT` removed), `exact_admission.rs` (limitations), `retained_memory.rs` (admission; registered profile), `retained_product.rs` (capture, facts, overlay, staging), `retained_wire.rs` (Section members; DEF-E hash), and the facade, census and source tests.
- **Readers (DEL-05-02 shared, DEL-04-08 owned):** RE `retained_precision.rs` (G5b, G8), `physics_source.rs` and `source_blocks.rs`; `tests/verifiers/retained_precision.py` and `physics_source.py`; `semantic_contract.rs` pins.
- **Schemas and definitions:** reviewed inputs #1, #2, #7 and #8 (section 7).
- **Pricing:** `tools/retained_pricing` (owners, admission pins, rules) and `retained-pricing-rules.md`.

**Fixtures and corpora**, each regenerated by its own writer:
- J2: new Timoshenko and dual-basis bases in both modes, plus section 6's mutations;
- the route-E successor fixtures and RV120: DEF-E-bound hashes only;
- the R1 and R2 reader records and the desktop wasm inputs;
- the K4 vectors: new Timoshenko vectors, existing ones unchanged;
- hash pins.

**Must stay byte-identical:**
- every ordinary and pressure-free byte;
- every Euler–Bernoulli zero-allowance source publication and its work (MJ6/MJ9, J2's existing bases);
- the J7 RF census pin;
- every route-L successor and the retained corpus;
- every route-E successor except its DEF-E-bound hashes;
- `validation/references/**` and the frozen UI benchmark.

**Deliverables:** DEL-04-08 (method, readers, definitions, pricing), DEL-04-01 (kernel), DEL-04-07 (PP) and DEL-05-02 (shared readers). `tools.deliverables impact DEL-04-08` lists 39 affected deliverables through DEL-04-07 and DEL-05-02. O4 changes no editor, guide or desktop product surface beyond the limitation strings those already display.
