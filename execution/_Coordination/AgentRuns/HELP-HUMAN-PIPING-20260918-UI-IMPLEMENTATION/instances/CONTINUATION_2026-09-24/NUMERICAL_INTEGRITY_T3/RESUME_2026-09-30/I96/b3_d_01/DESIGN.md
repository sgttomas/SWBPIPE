# I96 B3-D: the B3 design (B3a and B3b; documents and code reading only)

TASK (Type 2), I96, a designer for ROOT (HELP_HUMAN, Agent 0), who is the return path. I am a fresh instance and made no delegation. 2026-10-07 UTC.

**The brief:** `R/BRIEFS/B3D_DESIGN.md`, sha256 `08b34b12833157edf1c17b1c4b61dd8e41139071a75c10ba19925eedac9df41a`, verified before reading. I read NUM's `AGENTS.md` and `agents/AGENT_TASK.md` first. ROOT's mid-task message (I95's rulings) is answered in §4.4 and §5.

**What I did.** Documents and code reading at NUM's maintained tree, plus three small read-only scripts (standard-library Python from VENV, and one `git grep` script) and one `jsonschema` meta-validation run in `WT/scratch/i96_b3_d/` and copied to `_run_records/` with their outputs. No cargo, native or solver job, no install, no Git write; Git reads used `GIT_OPTIONAL_LOCKS=0`. Nothing went to the system temp directory. **No maintained file is edited; the statics in `statics/` are drafts for J1's package.**

**Notation.** WT, NUM, P, T, R, RR and VENV as in the dispatch. Also:
- **PP** = `P/core/product_physics`; **FK** = `P/core/solver/frame_kernel`, **FKR** = `FK/src/structural/retained`, **FKT** = `FK/tests/retained_k4`; **RE** = `P/core/reporting/result_export`.
- **RS**, **PY**, **TS** = `RE/src/retained_precision.rs`, `P/core/analysis_runs/retained_precision.py`, `P/apps/desktop/src/features/results/retainedPrecision.ts`.
- **PLAN**, **REV** = `R/I93/b2b3_plan_01/PLAN.md` and `REVISION_01.md`; **DESIGN_v2** = `R/I78/b0_contract_01/DESIGN_v2.md`; **KD** = `R/I94/b2_kd_01/DESIGN.md`; **STUDY** = `R/I95/b3_s_01/STUDY.md`; **C2**, **C3** = `R/I32/f2a_wire_c2/CONTRACT_DELTA.md`, `R/I52/prepared_public_contract_02/C3_DELTA.md`; **DN**, **D2** = `T/DESIGN_NUMERICS/DESIGN.md`, `T/DESIGN_STANDING/DESIGN.md`; **DOMAIN** = `R/I65/u4_g2_01/DOMAIN.md`.
- **DEF-O** = `P/fixtures/results/retained_precision_prepared_ordinary_v1.json`; **DEF-E** = the exact definition drafted here; **DEF-C** = B2-C's combination definition.
- **PTABLE** = `P/fixtures/results/semantic_contract_v0_3_preview_physics_retained_1.json`; **XTABLE** = the drafted `semantic_contract_v0_3_physics_retained_1.json`; **P1TABLE** = `semantic_contract_v0_3_physics_1.json`; **SCHEMA** = `P/schemas/retained_precision_mp_v2.schema.json`.
- "The exact route" = model 0.3.0 with pressure contract `{2.0.0, exact_straight_pressure_v2}`; "D1.5-exact" = every case's `pressure_regions` is `Some([])`.

**Basis.** NUM `f693bf170f` at dispatch and `ddac979e61` at my last read (records only between them); its `P/core`, `P/fixtures`, `P/schemas`, `P/apps` and `P/tests` trees equal main `2007709549` (`git diff --quiet`, checked at both). B1's branches were read at `b1` `603e238517` for the transaction shape only. Inputs and hashes are in §13.

## 0. Findings in brief

1. **The definition `RP-PREPARED-EXACT-DUAL-v1` (DEF-E) is DEF-O on the exact route, with five substantive changes** (§1):
   - **materials:** the base common E/ν only; the source lane encloses G = E/(2(1+ν)) from the exact E and ν (FK's internal `ExactENu`), never the represented Ĝ;
   - **the base contract:** physics-1, and `physics-retained-1` as its successor;
   - **the scope:** explicitly empty pressure regions and no pressure primitives; combinations, load states, point and interpolated E/ν excluded;
   - **a new `evidence` member:** the overlay regenerates the selected case's `pipe_sections` section values and `pipe_stress_extrema`;
   - **trust:** G7 is physics-1's base validator; G8 binds Ĝ bit for bit and reuses physics-source-1's actual-material check.

   Its preparation sub-object (`RP-PREPARED-ANNULUS-v1`) is byte-identical to DEF-O's. Draft: 9,480 B, raw sha256 `6edae5ff…3879`, **H(`retained_precision_formation_v1`) = `9b66492e56a25a10939973a9ceb060a538769b0cd8f0da52962d5ec90f68693d`**. All new names collision-checked (§10).
2. **Preparation is needed on the exact route, and it changes the exact producer's section bits** (§1.3). The ordinary exact section is `SourceAnnulus` (binary64 π, several roundings), not correctly rounded. On the committed exact fixtures (OD 0.2 m, wall 0.01 m) the prepared I, J and Z each differ by one ulp from the published `pipe_sections` (A is equal); in 2,000 random sections 75 % differ in at least one of A, I, J, Z. My emulation reproduces all 18 published `pipe_sections` bit for bit. KD's "no exact annulus version" stands: `prepare_product_annulus` applies unchanged.
3. **So a selected case's published section evidence must be regenerated** (§1.4): `exact_cases[c].pipe_sections[m].{As_m2, I_m4, J_m4, Z_m3}` become the prepared bits, and D2 G5b's bit-for-bit cross-check then holds. physics-1's base validator checks only positivity and the radii, so G7 is unaffected. Unselected cases keep their ordinary evidence.
4. **B3-K is needed, and it is slightly larger than KD said** (§8):
   - a public `ProductMaterial::BaseENu { e, nu }` mapping to `MaterialOperands::ExactENu`, as KD said;
   - **and** the represented-Z enclosure for `ExactENu`. `build_member` computes `represented_z` only for `Ordinary` and `Interpolated`, and the K-lane stress and maximum recipes require it (`final_case.rs` `recipe`, `bad("represented Z")`). **Without this change every exact case with a stress row fails its certificate.** RV115 did not name it.
   - The oracle pins `represented_z` as zero for its two `ExactENu` vectors (`product_certificate_vectors.py`: `rz=… if mode else (F(),F())`). Changing it is an existing-test-outcome change: **ROOT's declared exception** (as R-5 for B2-K), or alternative (b) in §8.
   - Estimate 3–6 h agent, RV-K 1–2 h.
5. **Ĝ is reproducible by every reader** (§1.2): PP's `Scaled` computation of G equals binary64 `e / (2.0 * (1.0 + nu))` bit for bit in 250,011 of 250,011 sampled cases (edge cases included). G8 can bind the receipt's `shear_modulus` exactly, not with physics-1's 2-ulp tolerance.
6. **The table `physics-retained-1`** (§2) is P1TABLE with three members changed and six appended, by the exact method that reproduces PTABLE byte for byte from preview-physics-1's table. Its rows and `reserved_inactive_successors` are physics-1's unchanged; its inherited hash is `9a2cf626…`. **RV78-N1's policies are bound from version 1 in a new member `receipt_bindings`**, which B2-C's PTABLE revision must spell identically. Draft: 51,163 B, **sha256 `5bf0d0dca79c975c5ade8436e150b8db5f3d3cdcdbef4f8df4776f5dd0cac5b7`**.
7. **Row families** (§3): with D1.5-exact the exact route publishes 7n + 51m + 8g rows per case, plus at most one mode row (when the ordinary linear solve returned) and at most one parity row in dense. Each has a DEF-O recipe. **No pressure-evidence row is published:** `append_exact_pressure_results` runs only for a region member. No row family lacks a recipe, so PLAN risk 4 does not fire.
8. **D1 texts** (§4):
   - B3a widens D1.3 to 0.3.0 with exactly `{1.0.0, legacy_pressure_v1}`;
   - B3b adds an exact branch to D1.3, an exact D1.4 clause (**no combination: ruling 4**) and an exact D1.5 clause (`pressure_regions == Some([])`, base selection only).
   - **N-11's reading:** a zero-magnitude legacy pressure load is outside D1.7, so it falls back with plain ordinary bytes, as on 0.1/0.2 today.
9. **Two producer requirements the plan did not name** (§5):
   - **(P-2) budget parity.** `permitted_run` builds `SourceRecoveryBudget::default()` (4,000,000 per case), while `ordinary_dispatch` gives the exact route `PHYSICS_SOURCE_WORK_LIMIT` (8,000,000). Lifting the exact refusal without matching it could change physics-source-1's selection on the Direct entry, and with it T-3 (c)'s exact-ordinary-bytes guarantee.
   - **(P-4)** W1 must not call the pressure runtime's builders, so I95's two zero rules hold.
10. **Readers** (§6): one `<physics-retained>` branch, dispatched on the identity, in all three languages.
    - **G0** reads XTABLE (N-12) and FORMATION by DEF-E's H.
    - **G5b** adds the evidence cross-check.
    - **G7** projects to physics-1.
    - **G8** checks the exact namespace (a combination is `INVOCATION_MISMATCH`, ruling 4), explicitly empty regions, `derived_e_nu` with exact Ĝ bits, route `exact`, and physics-source-1's actual materials (needs S-C's minimal exposure in each language's `physics_source`).
    - **No new failure code.**
    - B3a changes only G8's namespace predicate. That also removes a latent PY difference: PY admits `pressure_contract: {}`, RS and TS refuse it.
11. **Carriers and T6S** (§7): three schema branches, the TS route `retained_physics`, an output-policy entry admitting the same two panels with its own reason text, and a Rust golden of the exact successor's derivative.
12. **Estimates** (§11): B3a about 7–12 h agent; B3b about 50–79 h agent and 17–27 h review. On the plan's comparable rows that is +15–18 h, mostly B3-K's represented Z, the exact evidence overlay and observables, and S-C's exposure.

## 1. B3b's formation definition, `RP-PREPARED-EXACT-DUAL-v1`

### 1.1 What it binds, member by member against DEF-O

The draft is `statics/retained_precision_prepared_exact_v1.json`, generated by `_run_records/b3d_statics.py` from DEF-O's committed bytes, so every unchanged member is DEF-O's byte for byte. The form is DEF-O's: compact, sorted keys, UTF-8, no trailing newline, ASCII only, no JSON floats. So the sorted-key serialization is the JCS canonical form, and the raw sha256 and H differ, as for DEF-O (C3 §1).

| Member | DEF-E | Against DEF-O |
|---|---|---|
| `id`, `version` | `RP-PREPARED-EXACT-DUAL-v1`, 1 | new id |
| `inherits` | `base_contract` physics-1, `semantic_contract` physics-retained-1; the other six (kernel policy, method, projection, work, facade policies, canonicalization) unchanged | 2 values |
| `hash_domains` | the same five: definition `retained_precision_formation_v1`, preparation `retained_precision_preparation_v1`, receipt, publication and source as C1 | unchanged (B3D-2) |
| `preparation` | **`RP-PREPARED-ANNULUS-v1`, byte-identical** (checked: `preparation_subobject_identical_to_DEF_O: true`) | unchanged (B3D-3) |
| `lanes.admitted_k` | the actual admitted products E·A, Ĝ·J, E·I with the actual Ĝ bits | unchanged |
| `lanes.annular_source` | "directed1024 annulus geometry; E is the exact lift of the selected common E; G is enclosed as E/(2(1+nu)) from the exact lifts of the selected common E and nu by directed binary1024 add and divide (homogeneous_isotropic_E_nu_v1), never from the represented G_hat; …" | **changed:** the E/ν law (§1.2) |
| `lanes.*` (seed, order, owner, per-lane, private, reuse), `precision`, `projection`, `work` | as DEF-O | unchanged |
| `rows.component`, `component_stress`, `displacement_magnitude`, `prescribed`, `support_magnitude` | as DEF-O (component stress keeps "admitted-K bending Z is hull(actual Z, I_K/c)", which B3-K makes computable for E/ν, §8) | unchanged |
| `rows.maximum` | DEF-O's recipe, with its evidence named: "regenerate the eight numeric fields of `contract_evidence.exact_cases[].pipe_stress_extrema`" | changed: evidence location |
| `rows.ancillary` | "actual observed mode/parity bits/text; no modulus-basis record under the base common E/nu selection; …" | changed: no modulus record (D1.5) |
| **`evidence`** (new) | `pipe_sections`: regenerate As_m2, I_m4, J_m4, Z_m3 as the prepared A, I, J, Z; OD, wall, ro, ri, Ai, basis, id and order unchanged. `pipe_stress_extrema`: the eight regenerated fields only. `unchanged`: `pressure []`, `connector []`, and each case's `load_case_id`, `profile_mode`, `material_basis`, `pipe_materials`, `pressure_rhs_assembly` and `stress_maximum_coverage` (complete) byte-identical; no `recovery_method` | new (B3D-4) |
| `scope.excludes` | `typed_without_capture`, `ordinary_profile_prepared_formation`, `prepared_combinations`, `combinations`, `pressure_regions`, `pressure_primitives`, `load_reference_states`, `curved_members`, `components`, `nonlinear_supports`, `constant_effort_supports`, `directional_springs`, `nonzero_imposed_motion`, `equivalent_static`, `user_review_rows`, `named_point_common_E_nu`, `interpolated_common_E_nu` | changed |
| `scope.materials` | `["base_common_E_nu_derived_G"]` | changed (B3D-5) |
| `scope.pressure` (new) | `explicitly_empty_pressure_regions_and_no_pressure_primitives` | new |
| `scope.source` | `exact_straight_W1a` | changed |
| `scope.entry`, `loads`, `owners`, `prescriptions`, `requires`, `scope_limit`, `supports` | as DEF-O (`individual_normalized_nodal_terms`, `load_case`, `exact_positive_zero`, `whole_invocation_has_no_exact_block_selection`, rigid and global scalar springs) | unchanged |
| `acceptance.standing` | adds "no … ordinary-profile guarantee or physics-1 base change" in place of DEF-O's exact-profile clause | changed (wording) |
| `acceptance.final`, `native`, `warrant`, `scoped_cross_references` | as DEF-O: the same direct final certificate, unchanged b and inequalities, and the same three D2 cross-references | unchanged |
| `trust.G7` | "unchanged projected physics-1 base evidence validator" | changed |
| `trust.G8` | invocation, model and common E/ν rederivation with `shear_modulus` bits equal to RN64(E/(2·RN64(1+ν))); normalized-input and map rederivation; physics-source-1's actual-material binding over the physics-1 evidence; checked source and preparation associations; no historical physics-source stress recipe | changed |
| `trust.attestation`, `validation` | as DEF-O | unchanged |

`members_differing_from_DEF_O` (14 paths) is in `_run_records/b3d_statics.out.json`.

### 1.2 The material law: Ĝ and the exact G

- **The producer's G.** The exact route never uses an authored G. `pressure_material::resolve_base` sets each used material's `shear_modulus` to `IsotropicENu::new(E, ν).shear_modulus_pa()`, warning `EXACT_PRESSURE_REDUNDANT_G_IGNORED` if one was authored. In `Scaled` arithmetic that is Ĝ = to_f64(E ÷ (2·(1 + ν))) (`pressure_exact.rs:237–253`). The kernel's admitted K uses these Ĝ bits (`build_model`).
- **Ĝ is the binary64 expression.** For a normal Ĝ, each `Scaled` step rounds once at the same point as binary64, so Ĝ = RN64(E / (2·RN64(1 + ν))).
  - `b3d_numeric_checks.py` is a line-for-line transcription of `Scaled`. It compares Ĝ with Python's `e / (2.0 * (1.0 + nu))` on 250,011 pairs: random E in [1e-200, 1e300] and ν in (−1, ½), ν drawn by bits near zero, and 11 edge cases, including ν at ±5e-324, at −1 + ulp, at ½ − ulp, and E at the binary64 maximum.
  - **All 250,011 are equal; none was subnormal or refused.** JavaScript's `e / (2 * (1 + nu))` has the same IEEE semantics.
- **The two readouts** (DEF-E `lanes`):
  - the admitted K uses E and Ĝ as given;
  - the annular source uses the exact G = E/(2(1+ν)), enclosed outward from the exact lifts by FK's existing `MaterialOperands::ExactENu` (`product_certificate.rs:416–431`), which checks −1 < ν < ½ and E's bits.

  So the certificate states closeness to the exact-route physics, G derived from E and ν, as well as to the represented K. The Ĝ-to-G difference enters as `coefficient_differences` (DEF-O's E2), as an interpolated material's does.
- **G8 binds Ĝ exactly** (B3D-7): `shear_modulus == bits(e / (2.0 * (1.0 + nu)))`, normal and positive; `shear_origin = {kind: derived_e_nu, poisson_ratio: bits(ν), constitutive_basis: homogeneous_isotropic_E_nu_v1}`. The producer refuses, as a typed capture error, a Ĝ that is not a positive normal or differs from that expression. This is defensive: no sampled input reaches it.

### 1.3 Preparation on the exact route

- **The exact producer's section bits.** `build_model` (`lib.rs:7131–7155`) replaces A, I, J and Z by `SourceAnnulus::from_od_wall(OD, t_eff)`'s values. t_eff is `derive_pipe_section`'s wall minus mill tolerance: the same effective wall the preview route prepares from, and the one `SOURCE_ODWALL_EXPECTATIONS` and physics-source-1's `actual_materials` bind.
  - `SourceAnnulus` computes A = π̂·t·(D − t) with binary64 π̂, then I = A·(r_o² + r_i²)/4, J = 2I and Z = I/r_o, each a `Scaled` operation with one rounding (`source_geometry.rs:27–50`).
  - So the published `pipe_sections` are **not** correctly rounded.
- **The check** (`b3d_numeric_checks.out.json`, `sections`):
  - My transcription reproduces all 18 published `pipe_sections` in the committed physics-source raw outputs bit for bit.
  - The correctly rounded annulus (DEF-O's sequence on exact rationals, π enclosed to below 2^-1100) gives, for the committed section (D = 0.2 m, t = 0.01 m):

    | | A | I | J | Z |
    |---|---|---|---|---|
    | SourceAnnulus (published) | `3f7872fa3a37ac13` | `3efc52664442210b` | `3f0c52664442210b` | `3f31b37feaa954a7` |
    | Prepared (RP-PREPARED-ANNULUS-v1) | `3f7872fa3a37ac13` | `3efc52664442210a` | `3f0c52664442210a` | `3f31b37feaa954a6` |

  - **I, J and Z each differ by one ulp.** The prepared J is I51's `3f0c52664442210a` (I51 RETURN): the same correctly rounded value, from the same OD and wall (I51's committed `rf_skew_t_cant_off_122_r1e-04` request has OD 0.2 m and wall 0.01 m). The three representations of that J are therefore: preview's `derive_pipe_section` `…210e`, the exact route's SourceAnnulus `…210b`, prepared `…210a`.
  - In 2,000 random sections (D in [0.02, 1.5] m, t/D in [0.01, 0.2]), 1,504 differ in at least one property: A 815, I 1,078, J 1,078, Z 1,131. No rounding was ambiguous.
- **Decision B3D-3: prepare on the exact route,** with the identical `RP-PREPARED-ANNULUS-v1`.
  - It is the same formation as DEF-O, so the receipt's `preparation`, `PreparedMember`, the preparation hash and every C3 reader check are unchanged. Only the old tuple's values differ: on this route `old_source` = [E, Ĝ, A_s, I_s, I_s, J_s] and `old_facts` = [D, t, A_s, I_s, J_s, Z_s, D/2], from SourceAnnulus.
  - Without it, the K law would keep SourceAnnulus's ulp-level section error. That is the RV66 class I51 removed on the preview route (no common center for a cancellation-sensitive torsion case), and the exact route has a committed N05 torsion pair.
  - **Alternative:** no preparation, with the K law on SourceAnnulus bits. The definition would differ structurally from DEF-O (no preparation stage, new reader branches in G5 and G8), and its feasibility on n05/n06-class cases is unestablished. Rejected.
- **KD's answer confirmed:** `prepare_product_annulus(diameter, effective_wall)` is material- and route-independent, and its inputs are exactly the exact route's normalized OD and effective wall. **No exact annulus version.**

### 1.4 The section evidence (B3D-4)

The successor publishes physics-1's `contract_evidence` (`{pressure, connector, exact_cases}`). For a **selected** case:
- `exact_cases[c].pipe_sections[m]`: **As_m2, I_m4, J_m4 and Z_m3 are rewritten to the prepared A, I, J and Z.** OD, wall, ro, ri, Ai, `geometry_basis` and order are unchanged.
  - D2 G5b requires the receipt's section terms to equal "the exact route's `contract_evidence.exact_cases[].pipe_sections`: A_s and Z, bit for bit". The selected rows are computed from the prepared section, so the evidence must state it.
  - physics-1's base validator (`physics_evidence.rs` `geometry`) checks only key shape, positivity, ro = OD/2 and ri = OD/2 − wall. G7 passes.
  - `actual_materials` reads only OD and wall. G8's S-C check passes.
- `pipe_stress_extrema[]`: DEF-O's eight numeric fields regenerated from the final endpoint actions and the prepared section, as on the preview route. physics-1's `EXTREMA_BOUNDS` (row value = lower + ½(upper − lower)) then holds by construction.
- Everything else in `contract_evidence` is byte-identical to the ordinary exact envelope, including `pressure_rhs_assembly` (all-zero vectors, no groups) and `pipe_materials` (E, ν, Ĝ).

**Unselected cases** (`not_required`, `unavailable`) keep their ordinary rows, so they keep their ordinary evidence. **A disclosed property:** in a mixed successor, one pipe's section values can differ in the last bits between a selected and an unselected case (by one ulp in I, J and Z for the committed section). physics-1's validator compares member coverage across cases, not values.

**Alternatives:**
- (a) keep the ordinary `pipe_sections` and drop G5b's evidence cross-check: contrary to D2 §4.9.3 G5b, and the evidence would misstate the section behind the rows;
- (b) keep them and carry the prepared values only in the receipt: the published evidence and the published stresses disagree.

Both are rejected.

### 1.5 H domain, hashes and the file

- **H domain (B3D-2): reuse `retained_precision_formation_v1`**, as DEF-C does (RR "RV115 (RV-K) accepts B2-KD…", ruling 3, R-10). A domain names an object type; the ids differ. The preparation, receipt, publication and source domains are DEF-O's: the receipt family is shared, and `definition_id` separates the routes. Alternative: a new `retained_precision_formation_exact_v1` (0 hits); nothing gained.
- **Hashes of the draft:**
  - raw sha256 `6edae5ff3e0c72f36b3a7ba1f67edcddf94d392375c07f8220a641e73c933879` (9,480 B);
  - **H = sha256(canonical `{"domain":"retained_precision_formation_v1","payload":DEF-E}`) = `9b66492e56a25a10939973a9ceb060a538769b0cd8f0da52962d5ec90f68693d`.**
  - The same computation gives DEF-O's pinned `a7ed7ca0…0349` (control in the out file).
- **Maintained home:** `P/fixtures/results/retained_precision_prepared_exact_v1.json` (PLAN §1.4 write sets).
- **Any wording change at RV-D changes H, and so XTABLE's hash.** I-A regenerates both at J1 with `b3d_statics.py`, which builds them from committed bytes.

### 1.6 Collision check

`_run_records/b3d_collisions.out.txt` (`git grep -F` at NUM `ddac979e61`). Outside `P/execution` there are 0 hits for:
- `RP-PREPARED-EXACT-DUAL-v1`, `retained_precision_prepared_exact_v1`, `semantic_contract_v0_3_physics_retained_1`;
- the full form `openpipestress.result_semantics/0.3.0/physics-retained-1` and `exact_straight_retained_w1a_v2`, both already reserved 2026-10-03 (`R/verification/rv69_c3_selection_02/CHECKS.json`; DESIGN_v2 §5);
- every new definition token.

The full log is §10. DEF-E's H appears nowhere (it is new).

## 2. The `physics-retained-1` table

### 2.1 Construction

Draft: `statics/semantic_contract_v0_3_physics_retained_1.json`, 51,163 B, **sha256 `5bf0d0dca79c975c5ade8436e150b8db5f3d3cdcdbef4f8df4776f5dd0cac5b7`**.

**The method is PTABLE's own:** load the base table's bytes preserving key order, change three members, append the new ones, and write `json.dumps(indent=2, ensure_ascii=True)` plus a newline. **Control:** the same function applied to preview-physics-1's table and PTABLE's five appended members reproduces PTABLE's sha256 `c74742ce…` exactly (`control_ptable_method_reproduces: true`).

| Member | Value |
|---|---|
| `semantic_contract_id` | `openpipestress.result_semantics/0.3.0/physics-retained-1` |
| `inherited_semantic_contract_sha256` | `9a2cf6268b57bd5265a1a115497c07450819dd4d03cd5ab618097bd9d19da8cc`, the raw sha256 of P1TABLE's bytes |
| `formulation_profile_id` | `exact_straight_retained_w1a_v2` |
| `rows`, the vocabulary, hash vectors, counts, `metadata_policy`, `contract_evidence_policy`, `supported_profile_limitations`, `reserved_inactive_successors` | **P1TABLE's, unchanged** (checked: rows identical; successor list identical) |
| `receipt_policy`, `receipt_schema` | `M03-INTEGRITY-MP-v2`; `retained_precision_mp_v2.schema.json` (the shared SCHEMA, B3D-9) |
| `product_formation_definitions` | `[{"id":"RP-PREPARED-EXACT-DUAL-v1","sha256":"9b66492e…693d"}]` |
| `accuracy_classification` | PTABLE's, except `scope`: "registered exact-prepared source only: model 0.3.0 with the 2.0.0/exact_straight_pressure_v2 contract, explicitly empty pressure regions and the base common E/nu selection; no pressure-region, prepared-combination, load-reference-state or ordinary-profile extension" |
| `formation_warrant` | PTABLE's, with `definition_id` the exact id |
| **`receipt_bindings`** | `{"canonicalization":"openpipestress_jcs_ijson_v1","method":"contribution_preserving_multiprecision_v1","projection_policy":"RP-LOGICAL-ATTEMPTS-v1","work":{"case_limit":20000000000,"invocation_limit":60000000000},"work_policy":"W1-LME-20B-60B-v1"}` |

### 2.2 RV78-N1's policies from version 1 (B3D-8)

- **`receipt_bindings` binds exactly RV78-N1's set:** the projection and work policies, the 20B and 60B limits, the method token and the canonicalization profile (RR "I86's SW probe accepted…", ruling 1; REV §2).
  - Its keys are the receipt body's own paths: `projection_policy`, `work_policy`, `work.case_limit`, `work.invocation_limit` and `canonicalization`, plus the rows' `recovery_method` as `method`. G0's comparison is therefore path-for-path.
  - The kernel policy is already `receipt_policy`, and the facade policy is `accuracy_classification.policy`. Neither is duplicated.
  - The limits are exact JSON integers below 2^53.
- **B2-C must use the same member, name and shape in PTABLE's one revision** (REV §2), or ROOT chooses one spelling for both before J1. One reader G0 function then reads either table.

### 2.3 Inherited-hash rules

- XTABLE's `inherited_semantic_contract_sha256` is the raw sha256 of P1TABLE (DESIGN_v2 §5 item 1). P1TABLE is already a `REVIEWED_INPUT`.
- **P1TABLE's bytes are untouched,** including `reserved_inactive_successors` (DESIGN decision 14; the retained precedent left preview-physics-1's list alone).
- XTABLE's own list is P1TABLE's, copied as PTABLE copied preview-physics-1's.
- **Readers check:** sha256(packaged XTABLE bytes) equals their XTABLE constant; XTABLE's inherited value equals sha256(packaged P1TABLE bytes); XTABLE's identity and profile.

### 2.4 How G0 reads it (decision 31, N-12)

On the exact branch, in this order. The preview branch uses the same order with PTABLE once B2-C revises it.
1. Source identity `physics-retained-1` and profile `exact_straight_retained_w1a_v2` → `SOURCE_PRODUCER_CONTRACT_UNSUPPORTED`.
2. Producer component and version, envelope schema 0.2.0 → unsupported.
3. XTABLE's identity and profile → unsupported.
4. H(packaged DEF-E) equals the reader's `EXACT_DEFINITION_HASH`, **and** XTABLE's `product_formation_definitions` equals `[{id, sha256}]` → `RETAINED_PRECISION_FORMATION_MISMATCH`.
5. XTABLE's bytes hash equals its constant, and its inherited hash equals sha256(P1TABLE) → unsupported.
6. **Cross-check:** XTABLE's `receipt_bindings`, `receipt_policy` and `accuracy_classification.policy` each equal the reader's constants → unsupported. So neither the table nor a constant can drift alone.
7. `receipt_version` = 1 → unsupported.
8. The body's `policy`, `facade_policy`, `projection_policy`, `work_policy`, `canonicalization`, `work.case_limit` and `work.invocation_limit` equal the table's values → unsupported.
9. Every `product_attempts[].definition_id` equals XTABLE's definition id → unsupported.

G6 compares each selected row's `recovery_method` with the table's `method`, cross-checked at step 6.

**Tests:**
- 07o gains one mutation per bound receipt member on the exact base (§6.4);
- each reader gets one reader-local unit test of step 6 with a test-only table (REV §2).

## 3. The exact route's row families

With D1.5-exact, an exact case publishes (`lib.rs` `solve_load_case_observed`, `is_exact` branches; confirmed on the committed fixtures by I95's census):

| Family (kind, unit) | Per case | DEF-E recipe (`k::ProductRecipe`) | G5c class |
|---|---|---|---|
| `displacement_magnitude` (mm) | n | `Native(DisplacementMagnitude)`: H of the hull of the two lanes' norm functionals | translation, k = 1 |
| `global_nodal_displacement_{x,y,z}` (mm), `global_nodal_rotation_{x,y,z}` (rad) | 6n | `Native(Displacement)`; at a constrained DOF the prescribed recipe (exact +0) | translation or rotation, k = 1; `input_derived` by rule 2a |
| `support_reaction_component_v2` (N, N·m), Fx…Mz | 6g | `SupportComponent` | force or moment, k = 1 |
| `support_reaction_force_magnitude_v2` (N), `…_moment_magnitude_v2` (N·m) | 2g | `Native(SupportForce/MomentMagnitude)`; output binary64 hypot of the published components, certified against the dual physical norm | force or moment, k = 1 |
| `element_local_{axial_force, shear_force_y, shear_force_z}` (N), `{torsional_moment, bending_moment_y, bending_moment_z}` (N·m) at end_i, end_j, quarter_1, midspan, quarter_3 | 30m | `Native(EndAction)` at the ends; `Native(StationAction)` at the three B1 stations | force or moment, k = 1 |
| `element_local_{axial_normal, bending_normal_y, bending_normal_z, torsional_shear}_stress` (MPa) at the same five sites | 20m | `Stress` (signed corners of N/A, M/Z, T·c/J; K lane with represented Z, §8) | stress, k = 1 |
| `pipe_elastic_normal_stress_maximum_v2` (Pa) | m | `CircularMaximum`; output the bounder's midpoint; evidence regenerated (§1.4) | stress, k = 2√2: covered, since no member carries pressure under D1.5-exact |
| `linear_solver_mode_basis` (mode_code) | ≤ 1: present when the ordinary linear solve returned (no source is ever selected beside W1); absent when the ordinary attempt failed | `NonQuantity` | `non_quantity` |
| `sparse_live_path_dense_parity_relative_delta` | ≤ 1, dense only | `DenseParityObservation` | `non_quantity` |

**Total:** 7n + 51m + 8g, plus at most one mode row and, in dense, at most one parity row per case, within the chain's P_final = 7n + 51m + 8g + 3 (STUDY §3).

**Never published under D1.5-exact, and why:**
- **`pipe_wall_endpoint_action_v2`, `pipe_wall_axial_force_v2`, `pipe_effective_axial_force_v2`, `pipe_axial_membrane_stress_v2`, `pipe_lame_{radial,hoop}_stress_v2`:** only `append_exact_pressure_results` emits them, and its one call (`lib.rs:5373–5389`) needs a pipe state, which `build_pressure_case_with_members` creates only for a region member. **So there are no zero pressure-evidence rows.**
- **`pipe_section_pressure_{hoop,longitudinal}_stress`:** `pressure_for_pipe` reads pressure primitives, which the exact route refuses (`EXACT_PRESSURE_REQUIRES_REGION`, including zero values).
- **`open_formula_stress_summary`, `reaction_resultant`:** the exact route never emits them (`summary_value` is `None` on `is_exact`; the resultant is `!is_exact` only).
- **`modulus_basis_record`:** no basis selection under D1.5. **`combination_modulus_basis_record`:** no combination (ruling 4).
- **Constant-effort, nonlinear-support, component, curved-bend, spring-hanger and expansion-joint rows:** refused by `validate_profile`'s exact arm, D1.4 and D1.6.

If a future producer change emitted any of these, the capture would refuse ("uncovered actual row", `retained_product.rs` `bind_rows_view`), and the case would be `unavailable`, never silently uncovered. **Every published family has a recipe, so PLAN risk 4 does not fire.**

**G5c's pressure conditions on this route** (D2 §4.9.3 item 3):
- the receipt route is exact with empty regions, so "the case has zero pressure" holds;
- no member is in a pressure region, so `pipe_elastic_normal_stress_maximum_v2` and `pipe_axial_membrane_stress_v2` are not carved out.

The readers decide this from the invocation (`pressure_regions == []`, no pressure primitive), as today.

## 4. The D1 texts

In DOMAIN's form. Clauses not listed are unchanged from B1's text, as amended by B2-C.

### 4.1 B3a: D1.3 for `legacy_pressure_v1`

| Clause | Predicate | Warrant |
|---|---|---|
| **D1.3** (Namespace) | **Branch L:** `schema_version ∈ {"0.1.0","0.2.0"}` and `pressure_contract` is `None`; **or branch L3 (B3a):** `schema_version == "0.3.0"` and `pressure_contract == Some({version: Some("1.0.0"), mode: Some("legacy_pressure_v1")})`; [**or branch E** (B3b), §4.2]. On every branch: `reference_configurations` is `Absent`; every `material_expansion_laws[i]` is `Absent`; `request_material_expansion_laws` is empty; `sections` is empty; no pipe has `section_ref` | `validate_profile`'s 0.3.0 arm admits exactly this contract (and the exact one); `PressureContractInput` denies unknown fields. On L and L3, `is_exact` and `is_load_state` are false and the run publishes preview-physics-1 (`mechanics_producer_for_model`). `source_recovery` refuses the legacy namespace for 0.3.0 (`source_recovery.rs:604–608`), so exact-block never selects and T-3 (c) cannot fire |

- **Refusal map:**
  - a schema version in no branch → `Family(Namespace, SchemaVersion)`;
  - a contract that does not match its schema's branch → `Family(Namespace, PressureContract)`, for example 0.2.0 with a contract, 0.3.0 with none, or 0.3.0 with an unknown one.
  
  No new fact.
- **D1.4–D1.9 unchanged.** D1.5 keeps `pressure_regions` `None` on L3 (the ordinary route also blocks `Some` on a legacy model).
- **N-11's reading of DN §4.3's "zero pressure"** (recorded as REV §3 asks):
  - "Zero pressure" on `legacy_pressure_v1` means no pressure load in W1's domain: D1.7, unchanged, admits only node-targeted `force` and `moment` primitives.
  - **A zero-magnitude legacy pressure load** (dimension `pressure`, element target) is outside D1.7. Admission refuses it (`Family(Loads, LoadTarget)` or `LoadDimension`). The ordinary route admits it, because `validate_profile` refuses only non-zero values, and publishes it with `pipe_section_pressure_*` rows of value 0. **So it falls back with the plain ordinary bytes, exactly as a 0.1/0.2 model does today** (decision 12; RV114 N-11).
  - A node-targeted `force` or `moment` primitive whose free-text category is `pressure` is a nodal term under D1.7 (DOMAIN D1.7: "category … strings are free"). `validate_profile` refuses it when non-zero (`PRESSURE_MODEL_REAUTHOR_REQUIRED`), so in W1 it can only be a zero nodal term, as on 0.1/0.2.
  - **Neither is a defect.**
- **Pricing:** I93 expected none. On L3, `validate_profile`'s legacy arm emits nothing for a valid model. `source_recovery`'s closure refusal is cheaper than a 0.1/0.2 attempt. SQ2's G5 confirms both. B3a-W confirms the bytes.

### 4.2 B3b: D1.3, D1.4 and D1.5 for the exact route

| Clause | Predicate (branch E) | Warrant |
|---|---|---|
| **D1.3** | `schema_version == "0.3.0"` and `pressure_contract == Some({version: Some("2.0.0"), mode: Some("exact_straight_pressure_v2")})`; the shared namespace conditions of §4.1 | `is_exact` true and `is_load_state` false, so physics-1 is the base (`mechanics_producer_for_model`). **0.4.0, which `is_exact` also admits, stays out** (`SchemaVersion`), as do load states. Sections stay absent (no committed exact request has any) |
| **D1.4** (exact clause) | `combinations` is empty; `components` is empty; 1 ≤ c ≤ `LOAD_CASES` (B1's C = 3; no route cap: RR "I95's B3-S…", ruling 3) | **Ruling 4:** the ordinary route blocks every exact combination (`EXACT_PRESSURE_COMBINATION_UNSUPPORTED`), so W1's exact domain matches it and the exact forms never price combination text. B2-C's combination clauses apply only on L and L3 |
| **D1.5** (exact clause) | for every case: `pressure_regions == Some([])` (explicitly empty: `None` or non-empty → `Family(Case, PressureRegions)`); `equivalent_static` is `None`; `modulus_basis_ref` and `modulus_basis_temperature` are `None` (the base common E/ν only); `analysis_state` is `Absent` | physics-source-1's empty-region requirement (`source_recovery.rs:590–599`); DEF-E's scope; I95's D1.5-exact credits C-1 and C-2 assume exactly this |
| D1.6 | unchanged | `validate_profile` also refuses nonlinear and constant-effort supports on this route, so D1.6 is stricter than needed |
| D1.7 | unchanged | A pressure-category or pressure-dimension primitive, even zero, is blocking on this route (`EXACT_PRESSURE_REQUIRES_REGION`). A node-targeted force labelled `pressure` is in D1 but refused by the ordinary route: an in-domain request the ordinary route rejects, priced as an error prefix (DOMAIN §1) |
| D1.8, D1.9 | unchanged | STUDY §3: P_final unchanged; the exact contract evidence is a function of (n, m, g, c) within the text caps |

- **Refusal map:** `SchemaVersion`, `PressureContract`, `Combinations`, `Components`, `PressureRegions`, `ModulusBasisRef` and `ModulusBasisTemperature`, all existing facts. No new `FamilyFact`.
- **Law tests (B3b-A):**
  - admitted: 0.3.0 exact with `[]` on every case;
  - refused:
    - `pressure_regions` absent (`PressureRegions`), or with one region (`PressureRegions`);
    - one combination (`Combinations`);
    - 0.4.0 exact (`SchemaVersion`);
    - 0.3.0 without a contract (`PressureContract`);
    - a point basis (`ModulusBasisRef`).

### 4.3 How B3a's and B3b's clauses meet B1's and B2-C's

- D1.3's three branches are one clause, so I-A lands B3a's and B3b's D1.3 together (lane A owns `retained_memory.rs`).
- D1.4 is B2-C's text plus §4.2's exact clause.
- D1.5's exact clause sits beside B1's per-case clause.
- `family_clauses` decides the branch once, from (schema, contract), and applies the branch's D1.4 and D1.5 clauses. **That is also the route split G5 needs** (§4.4).

### 4.4 I95's rulings (the coordinator's message)

- **Ruling 1, routes separable.** The design keeps one route decision per stage, each behind its own call edge, so each TEXT graph can zero the other route's branches with I95's mirror rules:
  - admission's branch in `family_clauses`;
  - W1's `W1Route` from `permitted_run` (§5 P-1), with route-specific functions for capture, observables, maxima, overlay and wire identity;
  - the readers' dispatch on the identity (§6.1).
  
  **The rule for every lane:** no shared function runs both routes' route-specific work in one call; where a shared loop branches on the route, each arm calls a distinct named function. Nothing in the design forces one form set over both routes.
- **Ruling 2, the two zero rules** (`build_pressure_case_with_members` and `finish_source_groups` under empty regions). **They hold for this design:**
  - W1 never calls either function, nor any pressure-runtime builder (§5 P-4);
  - the ordinary route calls them unchanged, once per case;
  - D1.5-exact admits only `Some([])`, the premise of C-1 and C-2.
  
  The successor copies `pressure_rhs_assembly` unchanged and never regenerates it. If RV-D reaches those rules it can check them against this.
- **Ruling 3:** no route cap. **Ruling 4:** §4.2's D1.4 exact clause, and G8's expected refusal (§6.2).
- **The interim (REV §1.4):** at J2, B3b-A's admission would run on B1's S3 profile. That profile under-prices the exact route by about 152 MB at C = 3 (STUDY: erc 9,900,151,888 B against S3's 9,747,725,678 B), still 246,708,348 B under 0.9 M at 10.5 GiB.
  - REV §1.4 lets B3b-A land at J2 once B3-S shows a fit; it has.
  - SQ2's per-route registration supersedes the interim.
  - ROOT confirms (§12 item 5).

## 5. What the producer must do (lane P; requirements, not code)

The design needs these of B3b-P. The transaction is B1's T-1 to T-13, unchanged; only route-specific steps differ.

| # | Requirement | Why |
|---|---|---|
| **P-1** | **Route selection once.** `permitted_run` derives `W1Route::{Preview, Exact}` from (schema, contract) after admission. The defensive `Domain` fallback stays for `is_load_state` and 0.4.0 only | Ruling 1; DESIGN_v2 §5 item 4 |
| **P-2** | **Budget parity.** `permitted_run` builds `SourceRecoveryBudget` exactly as `ordinary_dispatch` does for the route: `per_case_limit = PHYSICS_SOURCE_WORK_LIMIT` (8,000,000) on the exact route; `default()` (4,000,000) is today's value | Otherwise physics-source-1 could select differently on the Direct entry than on the ordinary route, and T-3 (c)'s "exact ordinary bytes whenever exact-block selects" would fail. **Pin:** n05 and n06 through the registered Direct entry equal `ordinary_dispatch`'s bytes, in both modes |
| **P-3** | **Capture.** `normalized()` admits `is_exact` on route E and records, per used material, E, ν (from the normalized material, unit `1`) and Ĝ (`resolve_base`'s value), with `constitutive_basis == homogeneous_isotropic_E_nu_v1`. A non-normal Ĝ, or Ĝ ≠ RN64(E/(2·RN64(1+ν))), is a typed capture error | §1.2 |
| **P-4** | **No pressure runtime in W1.** D1.5-exact is read from the model (`pressure_regions == Some([])`); W1 never calls `build_pressure_case*`, `finish_source_groups` or `traverse_region` | Ruling 2's zero rules |
| **P-5** | **Facts and preparation.** `ProductMaterial::BaseENu { e, nu }` (B3-K). `old_source` and `old_facts` are the actual SourceAnnulus-derived bits; `prepare_product_annulus(D, t)` unchanged | §1.3; C3 §2 |
| **P-6** | **Observables for the exact evidence.** Closed `{pressure: [], connector: [], exact_cases}`. Per case: the 8 keys (no `recovery_method`); `profile_mode`; complete `stress_maximum_coverage`; one extremum per member; `pipe_sections` and `pipe_materials` covering every member; `pressure_rhs_assembly` with no groups and all-zero vectors | The preview observables read `preview_cases` and do not apply |
| **P-7** | **Maxima** from `exact_cases[c].pipe_stress_extrema[]`, regenerated with the prepared section | DEF-E `rows.maximum` |
| **P-8** | **Overlay (staging), selected cases only:** row values; §1.4's `pipe_sections` and extrema fields; the summary aliases (B1's headline rule at c ≥ 2) | §1.4 |
| **P-9** | **Wire:** identity `physics-retained-1`; profile `exact_straight_retained_w1a_v2` (limitations unchanged); `definition_id` the exact id; material `shear_origin` `derived_e_nu` with ν's bits; selection `base`; `section_terms[].geometry.route = exact`. Everything else as the preview route | SCHEMA already has these branches |
| **P-10** | **Precommit:** `retained_precision::validate` dispatches on the identity (§6.1). No PP change beyond the call | — |
| **P-11** | **Legacy-source disposition:** unchanged machinery. On this route the failed exact-block attempt is physics-source-1's; T1 (a) omits its `SOURCE_BLOCK_RECOVERY_UNAVAILABLE` on a selected case | `retained_wire.rs` `LEGACY_CODE` |
| **P-12** | **Hooks:** an exact-capture fault and an evidence-overlay fault (an index past `pipe_sections`), beside the existing ones | Mutation and fault coverage |
| **P-13** | **Pins** (both modes): the exact successor; coexistence on n05 and n06 (exact ordinary bytes); a mixed exact base if B3-W finds one; refusals (combination, regions absent or non-empty, 0.4.0, point basis) take the ordinary route | PLAN §1.4; SG2 |

## 6. The readers' `<physics-retained>` branch (RS, PY, TS)

### 6.1 Dispatch and what is shared

- **Dispatch:** `validate` and `validate_transport_metadata` branch once on `producer.semantic_contract_id`, preview or exact, and pass a route descriptor:
  - the identity, profile, packaged table and definition, and their constants;
  - the base identity and profile for G7's projection;
  - the evidence kind.
- **Shared and unchanged:** G1–G4, G5's native, ordinary and product checks (C1–C3), G5a, G5c's closed table, G6, and G8's topology, support, load, layout and preparation-binding checks.
- **Route-specific functions,** each behind its own call edge (ruling 1): G0's table read, G5b's evidence cross-check, G7's projection, and G8's namespace and material checks.
- **The integration sites the branch needs:**
  - **RS:** `semantic_contract.rs`:
    - `is_retained` covers both identities;
    - `PHYSICS_RETAINED_ID`, `_PROFILE` and `_SHA256`;
    - `verify_physics_retained_table` and `physics_retained_contract`;
    - `for_source` and `for_source_metadata`;
    - `retained_row_classes`, `rule_binding_refusal` and `numerical_use_standing`;
    - `forbid_retained_member` and `forbid_retained_rows`.
    
    `derivative.rs`: the receipt-copy and `contract_evidence` lists (D2 §4.9.7).
  - **PY:** `compatibility.py` (hash constant, standing and binding refusal).
  - **TS:** `numericalResultQuality.ts`, `knownSemanticLimitations.ts`, `retainedPrecisionStanding.ts`, `retainedPrecisionDisclosure.ts`, `resultSemantics.ts`, `services/analysisRunCompatibility.ts`, `ruleCheckService.ts`, `result-export/resultExportAdapter.ts`, `ResultsPanel.tsx`, `HistoricalRunContext.tsx` and `services/previewService.ts`. Each names the preview successor today (`git grep` at NUM); `tsc`'s exhaustive `SourceContract` records find any missed site.

### 6.2 Gates, codes and first failures on the exact branch

Gate order G0, G1, G2, G3, G4, G5, G5a, G5b, G5c, G6, G7, G8, as C1 and C3; first failure wins. **No new code.**

| Gate | Exact-branch check (new or changed in bold) | Code |
|---|---|---|
| G0 | §2.4 steps 1–9: **XTABLE, DEF-E's H, the table/constant cross-check, receipt values against the table, every attempt's `definition_id` = exact** | `SOURCE_PRODUCER_CONTRACT_UNSUPPORTED`; `RETAINED_PRECISION_FORMATION_MISMATCH` at step 4 |
| G1, G2 | SCHEMA (shared; `definition_id` enum); receipt and publication hashes | `RECEIPT_MISMATCH`, `ENCODING_MISMATCH` |
| G3, G4 | unchanged, including "no `SOURCE_BLOCK_RECOVERY_SELECTED`" (coexistence) and "no `SOURCE_BLOCK_RECOVERY_UNAVAILABLE` naming a selected case" | `COVERAGE_MISMATCH`, `DIAGNOSTIC_MISMATCH` |
| G5, G5a | unchanged; `old_source` and `old_facts` are read as given (G8 binds them) | existing |
| G5b | S\* recomputation and receipt-to-source section equality, unchanged; then **for each selected case c and member m: `section_terms[m].area` = `exact_cases[c].pipe_sections[m].As_m2` bits, `section_modulus` = `Z_m3`, and the source's `geometry`: `normalized_od` = `outside_diameter_m`, `effective_wall` = `effective_wall_thickness_m`, `actual_radius` = `ro_m`, `actual_second_moment` = `I_m4`, `actual_polar_moment` = `J_m4`** (evidence case located by `load_case_id`) | `RETAINED_PRECISION_SECTION_MISMATCH` |
| G5c, G6 | unchanged (§3's classes; the method token from XTABLE) | existing |
| G7 | **Projection:** remove `retained_precision` and the row tokens; set `physics-1` and `exact_straight_pressure_v2`. Then physics-1's unchanged base validator (RS `semantic_contract::for_source` → `validate_physics_evidence`; PY and TS equivalents) | each language's base code (C1 G7 settlement) |
| G8 | invocation shape and hash; mode and project id → `INVOCATION_MISMATCH`. **Namespace:** `schema_version == "0.3.0"`; `pressure_contract` exactly `{"version":"2.0.0","mode":"exact_straight_pressure_v2"}`; **`combinations` empty (ruling 4's expected refusal)**; `components` empty; no `reference_configurations` → `INVOCATION_MISMATCH` | `INVOCATION_MISMATCH` |
| G8 (cont.) | ids, ordinary attempts and mode/parity rows as preview. **Material bases:** selector `{kind: base}` for every case; each material `elastic_modulus` = bits(E), `shear_modulus` = bits(e/(2·(1+ν))), normal; `shear_origin` = `{derived_e_nu, bits(ν), homogeneous_isotropic_E_nu_v1}`; `selection` `{kind: base}`. **Then physics-source-1's `actual_materials` (S-C) for every `exact_cases` entry** (OD and wall against the authored values, E and ν against the selection, basis, thermal flags). Topology, supports, stations as preview. **Per source: the case's `pressure_regions` present and `[]`.** Members: **`geometry.route == "exact"`**, OD, wall, radius and A/I/J as preview; loads, layout, C3 preparation binding as preview | `RETAINED_PRECISION_PREPARATION_MISMATCH` (S-C's own code is detail only, B3D-13) |

- **Standing:** D2 §4.9.4 unchanged. A `not_required` case needs physics-1's ordinary eligibility on the projection.
- **Transport:** G0–G2 plus physics-1's transport-metadata check on the projection; never eligible.

**S-C's minimal exposure (B3D-12).** D2 §4.9.3 G8 uses physics-source-1's `actual_materials` "through S-C's parameter". S-C was never built: there is no `MaterialCheck` in any language.
- **Minimal form:**
  - RS: `physics_source::actual_materials` becomes `pub(crate)`;
  - PY: `physics_source._actual_materials` is imported as is;
  - TS: the per-case loop in `physicsSourceRecovery.ts` (about :329–358) is extracted into an exported function and called from the same place.
- Visibility and extraction only. Every physics-source-1 suite and corpus outcome stays identical, which is S-C's gate (D2 §7).
- This touches base-reader files, so ROOT rules it. **Alternative:** re-implement the checks in the retained readers, which duplicates logic that can diverge.

### 6.3 B3a's reader change (preview branch, all three readers)

**G8's namespace predicate becomes:** (schema 0.1.0 or 0.2.0, and `pressure_contract` absent or null) **or** (schema 0.3.0, and `pressure_contract` exactly `{"version":"1.0.0","mode":"legacy_pressure_v1"}`). Otherwise `INVOCATION_MISMATCH`. Today:
- RS requires null and admits 0.3.0 without a contract (D31);
- **PY tests `not model.get("pressure_contract")`, which admits `{}`; RS and TS refuse it.** That is a latent undeclared first-failure difference;
- TS refuses any contract.

The predicate aligns all three. **Two tightenings:**
- 0.3.0 without a contract, which the producer cannot emit (`PRESSURE_CONTRACT_REQUIRED`);
- `{}` in PY.

The census over 07n must show zero changed outcomes. 07m, at NUM, has only 0.1.0 and 0.2.0 invocations with a null contract. If 07n has one, the tightening is declared instead (B3D-10).

### 6.4 07o entries (SC2), with each first failure

Invocation edits recompute the receipt's invocation value and rehash, so the namespace is reached.

**B3a bases:** the milestone authored as 0.3.0 legacy, both modes (must-pass).

| Mutation | First failure |
|---|---|
| contract mode → `exact_straight_pressure_v2` (version 1.0.0) | G8 `INVOCATION_MISMATCH` |
| contract version → `1.0.1` | G8 `INVOCATION_MISMATCH` |
| contract removed (0.3.0, null) | G8 `INVOCATION_MISMATCH` |
| contract with an extra key | G8 `INVOCATION_MISMATCH` |
| schema 0.2.0 keeping the contract | G8 `INVOCATION_MISMATCH` |
| on a 0.2.0 base: `pressure_contract: {}` | G8 `INVOCATION_MISMATCH` in all three (PY aligns) |
| a zero-magnitude element `pressure` load added | G8 `PREPARATION_MISMATCH` (load rederivation) |

**B3b bases:** the exact successor, both modes (must-pass); a mixed exact base if B3-W finds one.

| Gate | Mutations (each expected first failure) |
|---|---|
| G0 | identity → preview id: `SOURCE_PRODUCER_CONTRACT_UNSUPPORTED`; profile → preview profile: same; an attempt's `definition_id` → ordinary: same; `projection_policy`, `work_policy`, `canonicalization`, `work.case_limit`, `work.invocation_limit` each changed: same; **the preview milestone successor relabelled as physics-retained-1** (identity and profile): step 9, same; **the exact successor relabelled as preview**: step 9, same |
| G5b | `pipe_sections[m].As_m2` one ulp; `Z_m3` one ulp; `I_m4` one ulp; `ro_m`: each `RETAINED_PRECISION_SECTION_MISMATCH` (before G7) |
| G7 | `connector` non-empty; `pipe_materials[].G_pa` three ulps off; `recovery_method` added to an exact case: each the base code (RS `SOURCE_PHYSICS_*`; PY and TS their own) |
| G8 | regions `null`; regions with one region; contract → legacy; schema 0.4.0; a combination added (ruling 4); `shear_origin` → `explicit_g`; `shear_modulus` one ulp; ν bits changed in the receipt; authored ν changed in the invocation (S-C); `geometry.route` → `preview`; a case naming `modulus_basis_ref`. Namespace edits give `INVOCATION_MISMATCH`, the rest `PREPARATION_MISMATCH` |

Plus one reader-local test per language of §2.4 step 6, and carrier transport cases for the exact successor (`retained_precision_carrier_cases.json`, if a carrier case is added).

## 7. Carrier branches, the output-policy entry and the golden (lane T; decision 21)

| File | Change |
|---|---|
| `P/schemas/results.v0.3.schema.yaml` | `producer.semantic_contract_id` and `semantic_contract_ref.ref_id` enums gain the exact id. **A new `oneOf` branch**, copying the preview successor's: requires `contract_evidence` and `retained_precision`; id consts; `profile_id` const `exact_straight_retained_w1a_v2`; **`limitations` const = the exact producer's seven strings** (`exact_straight_pressure_formulation_basis`, unchanged; B3D-15); `contract_evidence` `$ref` `PhysicsContractEvidence`; `result_sets` items `ResultSet` (physics-1's); `retained_precision` `$ref` SCHEMA; not `source_block_recovery`. The physics-1 branch keeps its `not retained_precision` |
| `P/schemas/analysis_run.v0.3.schema.json` | `SemanticContract.id` and `.sha256` enums gain the exact id and XTABLE's hash, with a `oneOf` pair. A new branch copying the preview successor's: requires `retained_precision`; `reproducibility.semantic_contract` and `result_refs[].semantic_contract` consts; not `source_block_recovery`, not `contract_evidence` (as physics-1 and the preview successor) |
| `P/schemas/stress_neutral_export.v0.3.schema.json` | The three `semantic_contract_id` enums gain the exact id. A new branch copying the preview successor's (about lines 3760–3830), with the exact id, XTABLE's hash, the exact profile and `contract_evidence` `$ref` `PhysicsContractEvidence`; the same UTF-8 CSV policy and member checksum canonicalization |
| `features/results/numericalResultQuality.ts` | `PHYSICS_RETAINED_CONTRACT_ID`, `_SHA256` and `_PROFILE`; **`SourceContract` gains `retained_physics`**; `sourceContract` recognizes it (id, profile, `contract_evidence` object, `retained_precision` object); `currentSemanticContract`; `hasCurrentSourceContract` false; `sourceContractTransport` runs the retained transport validator |
| `features/results/outputPolicy.ts` | **`retained_physics`:** `per_surface`, reason `RETAINED_PHYSICS_OUTPUT_REFUSAL` = `RETAINED-PRECISION-OUTPUT-NOT-YET-AVAILABLE: ` + `N_RETAINED_PHYSICS_OUTPUT` ("This output of retained-precision results (physics-retained-1) is not yet available on the desktop; only the result JSON and stress-neutral exports admit a numerically eligible result. It is routed to T6. The result remains readable here; this is not a finding about the result."); `result-export` and `stress-neutral` `admitted_when_eligible`; the other 19 refused. The preview text is untouched (B3D-14). `outputPolicy.test.ts` gains the entry's assertions. Its `"physics_retained"` unregistered-route example stays valid, because the route is named `retained_physics` |
| `features/stress-neutral/StressNeutralExportPanel.tsx` | the table-path map gains `retained_physics: "semantic_contract_v0_3_physics_retained_1.json"`; the panel tests gain the exact successor |
| `loadReferenceOutputAvailability.ts`, `retainedPrecisionOutputRefusal.test.tsx` | re-export and pin the new reason beside the preview one |
| **The golden** | `RE/tests/retained_precision_derivative_golden.rs` gains `retained_precision_exact_successor_derivative_{sparse_interactive,dense_scrutiny}.json`, `derive_document`'s output for the exact successor fixtures (T6S-2's pattern), and the TS derivative parity test checks them. B8's checklist item 4 needs it (decision 21) |

The receipt is copied whole, and classes come only from the reader (I74 §4.3). The goldens need only `derivative.rs`'s two list entries (RS lane).

## 8. B3-K: the E/ν gap

**Needed: yes.** B3b cannot construct an E/ν material for the certificate, and even with one, the K lane's stress recipe would fail.

| Item | Change | Why |
|---|---|---|
| K3-1 | `ProductMaterial::BaseENu { e: f64, nu: f64 }` (`final_case.rs`), with `operands()` → `MaterialOperands::ExactENu { e, nu }`. Re-export unchanged (`origins.rs:805` re-exports the enum) | KD §8; RV115 R-9 |
| **K3-2** | **`build_member` computes `represented_z` (hull of I_K/c and the actual Ẑ) and the I_y = I_z axis-bit check for every material, not only `Ordinary` and `Interpolated`.** Today `material()` returns `false` for `ExactENu`, so `represented_z` is `None` (`product_certificate.rs:544, 582–591`). The K-lane recipe for every `Stress` and `CircularMaximum` row then returns `bad("represented Z")` (`final_case.rs:748–752`) | The published exact stress divides by the actual Ẑ (prepared Z), exactly as on the preview route. The K readout must enclose both Ẑ and I_K/c, independent of the material route. **New finding: KD and RV115 did not name it** |
| K3-3 | Tests: `product_certificate_vectors.py` regenerated with the mode-0 `rz` = the hull for all modes; a final-case test with `BaseENu` (stress and maximum rows, ν = 0.3125 as RV56's control, the K and source lanes, the coefficient difference Ĝ·J against the enclosed G·J); `ProductMemberFacts` size unchanged (KD's atom; `Interpolated` governs it) | K-14's rule |

- **The declared exception (B3D-6).** The frozen oracle pins `represented_z` = [0, 0] for its two `ExactENu` vectors, `exact_normal` and `ratio_amplification`, through `rz = … if mode else (F(), F())`. With K3-2 they become the hull of I_K/c and Ẑ: [7, 7.5] (I_K = 15, c = 2, Ẑ = 7) and [0.5, 1] (I_K = 1, c = 2, Ẑ = 1). That is an existing FK test outcome changing (PLAN risk 9), so ROOT declares it, as R-5 did for B2-K.
- **Alternative (b):** keep `build_member` unchanged and compute the represented-Z enclosure inside the K-lane recipe when `sec.represented_z` is `None` and the material is `ExactENu`. No existing test changes, but there is a route-specific special case in the certificate.
- **I recommend K3-2 with the exception:** the represented-Z hull depends on how PP publishes stress, not on the material.
- **Optional, not required for soundness:** `ExactENu` could also require the admitted Ĝ bits to equal RN64(E/(2·RN64(1+ν))). The dual readout holds for any admitted Ĝ, and G8 binds it, so I do not recommend the kernel check.
- **Not needed:** an exact annulus version (§1.3), any change to the residual, tightening or bridge, any new `ReadoutLaw`, or any change to `MaterialOperands`.
- **Estimate:** 3–6 h agent (K3-1 about 1 h; K3-2 1–2 h; K3-3 1–3 h), RV-K 1–2 h (RV115 holds RV-K).
- **Placement (B3D-16):** B3-K is independent of B2-K's functions, but shares `final_case.rs` and `product_certificate.rs`, so it is lane K's first item. **A small J point (J2k)** merges it after RV-K's round, before B3b-P reaches its certificate stage. B3b-P develops capture and wire against J1 meanwhile.

## 9. Decisions

None is owner-held. "Decider" is ROOT throughout.

| # | Decision | Recommendation | Alternatives |
|---|---|---|---|
| B3D-1 | The exact formation | **A new definition, `RP-PREPARED-EXACT-DUAL-v1`,** at `P/fixtures/results/retained_precision_prepared_exact_v1.json` (draft in `statics/`) | Revising DEF-O to cover both routes: moves DEF-O's hash and its cascade, and mixes scopes |
| B3D-2 | Its H domain | **Reuse `retained_precision_formation_v1`** and DEF-O's other four domains (as R-10 for DEF-C) | `retained_precision_formation_exact_v1` (0 hits) |
| B3D-3 | Preparation | **Yes: the identical `RP-PREPARED-ANNULUS-v1`;** no exact annulus version | No preparation (K on SourceAnnulus bits): different structure, RV66-class risk |
| B3D-4 | Section evidence of a selected case | **Regenerate `pipe_sections` As/I/J/Z and the extrema; everything else byte-identical** | Keep the ordinary evidence (contradicts D2 G5b), or prepared values only in the receipt (evidence disagrees with the rows) |
| B3D-5 | Material scope | **The base common E/ν only** (D1.5); point and interpolated E/ν excluded in DEF-E | Admit named points now (same FK operands), with a D1.5 widening and G8's point path: later, by a new definition version |
| B3D-6 | B3-K | **Needed: K3-1 to K3-3,** with ROOT's declared exception for the two oracle vectors' represented Z | (b) K-lane special case with no test change |
| B3D-7 | Ĝ binding | **G8 exact bits** `e/(2·(1+ν))`, normal; producer refusal otherwise | physics-1's 2-ulp tolerance |
| B3D-8 | RV78-N1's bindings | **`receipt_bindings` in XTABLE v1, the same member in PTABLE's B2 revision;** G0 reads the table with constants as a cross-check (decision 31) | Flat top-level members (one spelling for both tables either way) |
| B3D-9 | Receipt schema | **One SCHEMA,** `definition_id` an enum (ordinary, combination, exact), `$comment` amended; draft diff in `statics/SCHEMA_ENUM.diff` | A separate exact schema file: another reviewed static, and a fourth reader schema load |
| B3D-10 | B3a's namespace predicate | **Exact (L or L3) in all three readers;** 0.3.0 with no contract and PY's `{}` become `INVOCATION_MISMATCH` if the 07n census shows zero changes; otherwise declared | Keep D31's admission of 0.3.0 without a contract (vacuous; keeps PY's `{}` difference) |
| B3D-11 | Legacy `pressure_regions: []` leniency (all readers accept absent or empty) | **Keep, unchanged;** the producer cannot emit it | Tighten to absent or null |
| B3D-12 | S-C | **Minimal exposure** of `actual_materials` in RS, PY and TS (visibility and extraction only), with physics-source-1's outcomes identical | Re-implement inside the retained readers |
| B3D-13 | G8's code for an S-C failure | **`RETAINED_PRECISION_PREPARATION_MISMATCH`,** with S-C's code as detail | Surface S-C's codes (their PY and TS spellings differ from RS) |
| B3D-14 | T6S route name and reason | **`retained_physics`;** a new `N_RETAINED_PHYSICS_OUTPUT` under the existing code; the preview text unchanged | Reword the shared text (a changed disclosure meaning, back to ROOT) |
| B3D-15 | The results carrier's `limitations` | **Pinned as const** (the producer's seven strings), as the preview successor's branch pins its own | Unpinned, as physics-1's branch |
| B3D-16 | B3-K's placement | **Lane K's first item, merged at a small J2k** before B3b-P's certificate stage | Merge with B2-K at J3 (B3b-P waits on B2-K) |
| B3D-17 | B3b's admission timing | **J2, under the interim,** disclosing that S3's profile under-prices the exact route by about 152 MB (1.6 %) at C = 3, still inside 0.9 M (REV §1.4 met by STUDY) | Wait for SQ2's registration |
| B3D-18 | `REVIEWED_INPUTS` order | **Append DEF-C, DEF-E and XTABLE** (14 → 17), so existing positions keep their meaning | I-A's choice at J1 |

## 10. Collision log: names for ROOT to reserve

From `_run_records/b3d_collisions.out.txt` (`git grep -F` at NUM `ddac979e61`; "maintained" = P outside `P/execution`). Whole-word checks for the code identifiers are listed below the table.

| Name | Kind | Maintained hits | Note |
|---|---|---|---|
| `RP-PREPARED-EXACT-DUAL-v1` | definition id | 0 | 10 in `P/execution` (plans) |
| `P/fixtures/results/retained_precision_prepared_exact_v1.json` | definition file | 0 | — |
| `P/fixtures/results/semantic_contract_v0_3_physics_retained_1.json` | table file | 0 | named in DESIGN_v2 §5 |
| `openpipestress.result_semantics/0.3.0/physics-retained-1`, `exact_straight_retained_w1a_v2` | identity, profile | 0, 0 | **reserved 2026-10-03;** reconfirmed. The bare `physics-retained-1` is one comment in `outputPolicy.ts` |
| `receipt_bindings` | table member (shared with PTABLE's revision) | 0 | — |
| `base_common_E_nu_derived_G`, `exact_straight_W1a`, `explicitly_empty_pressure_regions_and_no_pressure_primitives`, `ordinary_profile_prepared_formation`, `named_point_common_E_nu`, `interpolated_common_E_nu`, `pressure_primitives` | DEF-E tokens | 0 each | hashed values only |
| `constant_effort_supports`, `load_reference_states` | DEF-E tokens | 2, 126 | existing words with the same meaning (a test name; the evidence key); not identifiers here |
| `retained_physics` | TS `SourceContract` value | 0 | `physics_retained` is avoided: T6S's test uses it as the unregistered example |
| `PHYSICS_RETAINED_CONTRACT_ID`, `_CONTRACT_SHA256`, `_PROFILE` (TS); `PHYSICS_RETAINED_ID`, `_PROFILE`, `_SHA256`, `verify_physics_retained_table`, `physics_retained_contract` (RS) | code constants | 0 whole-word (substrings of `PREVIEW_PHYSICS_RETAINED_*`) | — |
| `SOURCE_PHYSICS_RETAINED_TABLE_HASH`, `SOURCE_PHYSICS_RETAINED_TABLE_IDENTITY` | RS table-pin errors (mirroring the preview's) | 0 | — |
| `EXACT_CONTRACT_ID`, `EXACT_DEFINITION_ID`, `EXACT_DEFINITION_HASH`, `EXACT_TABLE_HASH` | RS and PY reader constants | 0 | — |
| `N_RETAINED_PHYSICS_OUTPUT`, `RETAINED_PHYSICS_OUTPUT_REFUSAL` | TS output-policy text | 0 | code `RETAINED-PRECISION-OUTPUT-NOT-YET-AVAILABLE` reused |
| `retained_precision_exact_successor_{sparse_interactive,dense_scrutiny}.json`, `retained_precision_exact_successor_derivative_{…}.json` | fixtures and goldens | 0 | 1 in PLAN |
| `ProductMaterial::BaseENu` | FK variant (B3-K) | 0 | — |
| `retained_precision_formation_exact_v1` | only if B3D-2's alternative is chosen | 0 | — |

- **No new failure code, hash domain or `FamilyFact`.**
- **The DEF-E hash** `9b66492e…` and **XTABLE's** `5bf0d0dc…` are drafts: any RV-D wording change regenerates both.

## 11. Estimates

Agent hours, without repair rounds, against PLAN §2.2's rows.

| Slice | Content | Plan | **Refined** |
|---|---|---|---|
| B3a-A | D1.3 branches L and L3; law tests | 2–3 | **2–3** |
| B3a pins (lane P) | the 0.3.0-legacy milestone successor, both modes; equality with the 0.1.0 successor apart from invocation-bound hashes | — | **1–2** |
| B3a readers | G8 predicate and local tests: RS 1–2, PY 1–1.5, TS 1–1.5 | (in 13–20) | **3–5** |
| **B3a total** (+ 07o 1–2 h in SC2) | | | **7–12** |
| B3b-A | D1.3 branch E, D1.4 and D1.5 exact clauses; law tests | 2–3 | **2–4** |
| Statics (B3b part) | install the drafts, regenerate the hashes, SCHEMA enum, `REVIEWED_INPUTS` | (in 3–5) | **1–2** |
| B3-K | K3-1 to K3-3 | 0–8 | **3–6** |
| B3b-P | P-1 to P-13 | 12–18 | **14–22** |
| B3b readers | RS 6–9, PY 5–7, TS 6–9 (S-C exposure, about 1 h per language, included) | (in 13–20) | **17–25** |
| B3b-T | three carrier branches, the output policy and its tests, the panel map, the golden and its parity | 5–8 | **6–9** |
| 07o (B3b part, SC2) | bases and about 27 mutations | (in SC2) | **3–5** |
| SQ2 (B3b part) | per-route TEXT graph and mirror rules (ruling 1), the two credits' review inputs | (in SQ2) | **4–6** |
| **B3b total** | | | **50–79** |

- **Review:** RV-D 5–8 (this design), RV-K 1–2 (B3-K), RV-P2 4–6, RV-R2 4–6, RV-Q2 3–5, so **17–27 h for B3b** and about 1.5 h for B3a.
- **Against the plan's comparable rows,** excluding the SC2 and SQ2 shares the plan counts elsewhere: 49–78 h against 34–60 h, **+15–18 h.**
  - B3-K's represented Z and oracle: +3 h;
  - the exact observables and evidence overlay, P-6 to P-8: +2–4 h;
  - S-C's exposure in three languages: +3 h;
  - the budget-parity and coexistence pins: +1 h;
  - the readers' evidence cross-check and table read: +4–7 h.
- **Critical path:** B3-D → RV-D → J1 → B3b-P (14–22 h, +2–4 h) → RV-P2 round 1. J2k (B3-K, 3–6 h plus review) runs beside B3b-P's capture and wire work and is off the path if it lands before B3b-P's certificate stage.

## 12. For ROOT

**Rule on:**
1. **B3D-1 to B3D-18,** in particular:
   - **B3D-3 and B3D-4:** prepare on the exact route, and regenerate the selected case's section evidence;
   - **B3D-6:** B3-K with K3-2 and the declared exception for two oracle vectors;
   - **B3D-12:** S-C's minimal exposure in the base `physics_source` readers;
   - **B3D-16:** J2k for B3-K.
2. **Reserve the names in §10.** The identity and profile need only reconfirmation.
3. **`receipt_bindings`:** tell B2-C to use the same member in PTABLE's revision, or choose one spelling for both before J1 (B3D-8).
4. **P-2 (budget parity)** into B3b-P's brief, with the n05 and n06 Direct-against-ordinary byte pins.
5. **B3D-17:** B3b's admission at J2 under the interim, with the 152 MB disclosure, or after SQ2.
6. **B3D-10:** the namespace tightenings, if the 07n census is clean.

**Carried, no ruling needed:**
- I95's ruling 2 holds for this design (§4.4).
- No owner-held decision is touched. Decision 24's note stands: B3b adds `physics-retained-1`'s two panels to B8's native witness.

## 13. What I read, execution record and limits

**Read** (sha256; R, T, P as placeholders):

| Input | sha256 |
|---|---|
| The brief | `08b34b12…df41a` |
| PLAN; REV | `e1147dbd…1238a`; `63abb73f…bba0c` |
| RV114's REVIEW | `bc6918ce…a9ee6` |
| KD | `4e8c33a4…aabed3` |
| STUDY (and its SHA256SUMS, 44 of 44 OK) | `8975948a…b518c` |
| RV115's REVIEW (§7–§8) | `0f7ab77d…be93c` |
| DESIGN_v2 (§0, §5, §7, §8) | `5933b90b…1114` |
| C2 (§3–§4); C3 (§1–§4) | `923da0b9…0869`; `fd00d2c1…292e` |
| DN (§4.2–§4.4); D2 (§4.9) | `fb62ef4a…7a74`; `993f5f3a…4c8d` |
| I74 PLAN (§1, §4.3); DOMAIN (§1) | `0350c918…2ed9`; `08a72dde…c9fd` |
| I51 RETURN (rationale) | `a3e82a9a…df63` |
| `R/verification/rv69_c3_selection_02/CHECKS.json` | `289bd974…b012` |
| RR (sections "I86's SW probe accepted…", "B2/B3 R1…", "I93's REVISION_01 accepted…", "SP returned…", "I94's B2-KD…", "I95's B3-S…", "RV115 (RV-K) accepts B2-KD…"; the 2026-10-03 reservation) | `1569976c…ad44` at my last read (RR moves) |

**Code**, at NUM's tree (= main `2007709549`), with sha256 prefixes:
- **PP:**
  - `lib.rs` `4c33c250`: producer choice, formulation basis, `source_eligible`, dispatch and `permitted_run`, `retained_w1`, `solve_load_case_observed`'s exact branches, `build_model`'s section, `append_exact_pressure_results`, `append_signed_support_results`, `pressure_for_pipe`, modulus records, `derive_pipe_section`, the envelope assembly;
  - `pressure_runtime.rs` `5a4f07f4`; `pressure_material.rs` `a7d63ced`; `pressure_exact.rs` `b8563099` with `source_geometry.rs` `a0231707`; `annulus_geometry.rs`; `source_recovery.rs` `af58eaf7` (namespace);
  - `retained_product.rs` `f536bfe7` (scope, materials, row binding, observables, overlay); `retained_wire.rs` `4f383c04` (identity, `LEGACY_CODE`); `retained_memory.rs` `fbc7c9db` (`family_clauses`); `build_identity.rs` `058584f4`.
- **B1:** `b1`'s `lib.rs` (`permitted_run`, `retained_w1`, `w1_transaction`) and `family_clauses`.
- **FK:** `product_certificate.rs` `64e09224` (`MaterialOperands`, `material`, `build_member`, the annulus preparation); `final_case.rs` `d4dbaf3c` (`ProductMaterial`, `recipe`, the fact checks); FKT `product_certificate_vectors.py` `88fc0765` and `product_certificate_tests.rs` `94ff617d`.
- **RE:** `retained_precision.rs` `4722b505` (G0, G4, G5b, G8, `project`, `validate`); `semantic_contract.rs` `fbbc1a16`; `physics_evidence.rs` `f17c0f4c`; `physics_source.rs` `c4e380ee` (`actual_materials`); `derivative.rs` `85d22b77`.
- **PY:** `retained_precision.py` `9d1156ed` (constants, `_g8`); `physics_source.py` `81487a87`.
- **TS:** `retainedPrecision.ts` `7f9b47a9`; `outputPolicy.ts` `71294131` and its test `ee51a26d`; `numericalResultQuality.ts` `af72a4d5`; `physicsSourceRecovery.ts` `ef7ab4c9`.
- **Schemas:** `results.v0.3.schema.yaml` `eb21b496`, `analysis_run.v0.3.schema.json` `04cf09ca`, `stress_neutral_export.v0.3.schema.json` `6355640a`; SCHEMA `07951eda`.
- **Statics:** DEF-O `3e0779a4`, PTABLE `c74742ce`, P1TABLE `9a2cf626`, preview-physics-1's table `ae55503d`; CORPUS 07m `c21112fd`.
- **Fixtures:** the committed `fixtures/product_preview/physics_source/` requests and raw outputs.

**Executed** (Python 3.13 from VENV, standard library; `PYTHONDONTWRITEBYTECODE=1`, `TMPDIR` in scratch; `_run_records/RUN.md` has the commands with placeholders):
- `b3d_statics.py`: builds the three drafts from committed bytes, with the DEF-O and PTABLE controls. Output: `b3d_statics.out.json`.
- `b3d_numeric_checks.py`: the Ĝ check and the section-bits check. Output: `b3d_numeric_checks.out.json`.
- `b3d_collisions.sh`: `git grep -F` over NUM's HEAD. Output: `b3d_collisions.out.txt`, with `projects/chirality-piping/` written as `P/`.

Each ran once (the statics script twice, identically), in seconds. A one-line `jsonschema` meta-validation of the draft SCHEMA also passed (`RUN.md` step 2b). The scratch copies in `WT/scratch/i96_b3_d/` (including a read copy of `b1`'s `lib.rs`) are disposable.

**Limits:**
- **Nothing was compiled or run** beyond that Python and Git. Every producer, kernel and reader statement is read from source.
  - K3-2's necessity is read from `build_member` and `recipe`; B3-K's tests establish it.
  - P-2's divergence is read from the two budget constructions; the n05 and n06 pins establish it.
- **The exact-route verdicts are B3-W's:** which input selects (the milestone authored as exact, n05, n06), and whether a mixed exact base exists. The witness choices in §5 and §6.4 are conditional on it.
- **Ĝ's equality is sampled,** not proved for every binary64 pair. The step-by-step argument covers normal results, and the producer refuses anything else (P-3).
- **The section comparison** covers the committed exact section and 2,000 random sections; it does not cover D1's caps' worst inputs. Preparation's own refusal (`ambiguous_rounding`) remains the guard.
- **The drafts' hashes move** with any wording change at RV-D, and SCHEMA's draft is B3b's change alone (J1 merges B2-C's).
- **I did not read** B1's final T-11 headline staging code beyond its description in RR, or B2-C's (unwritten) PTABLE revision.
