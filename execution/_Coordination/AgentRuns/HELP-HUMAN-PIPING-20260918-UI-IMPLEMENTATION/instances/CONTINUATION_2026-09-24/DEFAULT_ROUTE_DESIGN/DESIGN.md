# T0R — the default route must not publish silently wrong results

HELPS_HUMANS design record for ROOT (HELP_HUMAN), revision 3. It is a proposal until ROOT selects it after a narrow backcheck.

- **Basis:** Piping product paths at main `eb56e1083`. Main has since moved to `3245f9acd`, but no path under `P/core` or `P/apps` differs. T1 is read at `500d88644`.
- **Earlier revisions** are archived unchanged:
  - revision 1, read by the review and S0: `_run_records/DESIGN_revision1.md` (sha256 `ca355b8f…`);
  - revision 2, read by the backcheck: `_run_records/DESIGN_revision2.md` (sha256 `a60871d7…`).
- **Paths:** `P/` means `projects/chirality-piping/`. `PP` means `P/core/product_physics/src/lib.rs`, with line numbers at `eb56e1083`.
- **What I did not do:** edit any product source, test, fixture or other record; run any Git write.

## Revision 3 — what changed and why

**Inputs:** `REVIEW/BACKCHECK_R2.md` (FINDINGS, no BLOCKING), `REFERENCE_CHECK/REVISION_2/RETURN.md` (S0′), and ROOT's rulings on both. The S0 and S0′ archives are unchanged.

**References:** `references.py` changes only inside `revision_2`. The revision-1 block still re-serializes to the S0-checked hash `f3c8e2f9…`. `references.stdout.txt` and `SHA256SUMS` are regenerated.

| Finding | Change |
|---|---|
| **SF-A** post-loop contract | §5.6: the rendering step now runs right after source selection is known and **before** headline selection and `append_combination_results`. Combinations, their diagnostics, both headlines, `HIGH_DISPLACEMENT_REVIEW` and `component_stress_modifier_count` are all built once, from the new rows. The step's full contract is stated, including removal of retired codes such as the in-loop `COMPONENT_STRESS_MULTIPLIER_APPLIED`. A reader tamper case is added: no retired code, and every affected ref resolves |
| **SF-B** R-2 enforcement | §5.7: a shared helper `rule_binding_refusal` in `result_export`, called at every binding site: the two `src-tauri` resolution paths (through `solver_result_row_value`), `services/ruleCheckService.ts`, and `rule_check_runner`, which reports the refused input as incomplete with the reason. Headless has no solver-result binding path today; a test pins that. The notice text states the √2 bound. Tamper test added |
| **NOTE-3** | §5.7: Current exports of an all-selected source-blocks-1 envelope carry the same notice |
| **SF-C** | §6 and S2a add `P/core/loads/self_weight_wasm/tests/applied_self_weight.rs` (`result:reaction:support-root` assertions) |
| **SF-D and S0′-6** tie rule | §9.1 and references: raw values are recorded; the identity tie-break is asserted only on a bitwise tie in both orders and both modes; otherwise either location is accepted within 1e-9 and the run reports "tie not exercised". Applies to REF-M33-TIE and REF-M08-L. No new tolerance |
| **SF-E** M10 diagnostic (ruled in as an S2a item) | §2.2: `run_linear_static_preview_value_with_mode` checks the request `Value` for a primitive-load `target.type == "support"` on models before 0.4.0, before `CapturedInvocation::parse`. It returns a blocked envelope with `IMPOSED_DISPLACEMENT_REQUIRES_LOAD_STATE_MODEL`. No `LoadTargetInput` variant. Negative control added |
| **NOTE-1** | §5.4a: macro-bend (`curved_bend_macro_element`) components produce no intensified rows anywhere, including straight neighbours at the component node. Controls: REF-B1 with SIF 1.3 gives no intensified row; a marker with SIF and no k is published (REF-I-L `no_k_variant`) |
| **NOTE-2** | §5.7: the selected-plus-ordinary reader rule applies only to non-composite source-blocks-1; physics-source-1 legitimately carries ordinary cases |
| **NOTE-5** | §5.4 and §6: `arc_chord_frame` and the arc basis token enter the table's metadata vocabulary, the stress-neutral method CSV mapping, and the Python and TS signature readers |
| **NOTE-6** | §5.4: the 1e-6 kink threshold is called a geometric-consistency warning threshold |
| **S0′-1** COMB-2 | The moment-envelope controls are split: A1/T gives 1000 against a sum of 1500; A1/A2 gives 1000 against 2000 |
| **S0′-2** I-L | Both axial variants are labelled: added unintensified (13.3823 MPa) and intensified (13.4257 MPa) |
| **S0′-3** I-T | Fx = +800 at c, so the header now differs from the unreferenced continuation: header 13.9630 MPa, branch 29.1602 MPa; the old 7.40035 MPa is kept as a negative control |
| **S0′-4** B2 | The chord-frame cut face at b is frozen, with the product's end_i rows as its negation. The tangent-frame values are kept for information only. S2a confirms whether a lone arc span is accepted |
| **S0′-5** | The nonlinear JSON is pinned: NL-C3 and SPRING-GAP use `gap` with `closes_when: positive_displacement`; ATTR uses `one_way` with `active_when: positive_reaction` |
| **S0′-7** | The global-balance rule still applies to every reference; REF-ATTR is explicitly exempt and asserts instead that anchor rows are zero, both withheld records are present and no tip row is zero-filled |
| **S0′-8** | §9.2 detection list adds I-L1 (14.14 against 13.09 MPa) and I-T |
| **S0′-9** | X1-SUPPORTS is described as catching DOF-slot mapping errors only |
| **S0′-10** | REF-M14-TH gives moments their own zero scale: the axial force times the outer radius, 49762.83 N·m |
| **T1 overlap** | §10.2: the rendering call moves out of T1's envelope block into the region after source selection, which is adjacent only. Two small adjacent insertions are added: the entry-function check at `PP:1231-1233` (T1 `1402-1406`) and the blocked-envelope evidence (T1 `10679`) |

## Revision 2 — what changed and why

**Inputs to this revision:**
- `REVIEW/RETURN.md` (verdict BLOCKING);
- `REFERENCE_CHECK/RETURN.md` (S0);
- `ROOT_RULINGS.md`;
- `../OWNER_SIF_DECISION_2026-09-26.md`;
- `../OWNER_T1_DECISIONS_2026-09-26.md`, the owner's T1 decisions. D3 removes the need for pre-0.4 backward compatibility. D4 sequences T1's WP2, WP3's native fields and WP4 after T0R.
- the owner's acceptance of the report-package outage until T6, recorded in ROOT_RULINGS.

`references.py` keeps every revision-1 key and value, and the rerun verifies them byte-equal. The new cases sit under `revision_2`.

The S0 archive in `REFERENCE_CHECK/_run_records/` is unchanged.

A second probe run, `_run_records/design_probe_output_rev2.log`, sends every case through the captured entry the desktop uses (N-8). It reproduces the reviewer's B-1 and SF-1 values, and it establishes the imposed-displacement behaviour (§2.2).

| Finding | Change in this revision |
|---|---|
| **B-1** nonlinear superposition | §5.6: if the model has any nonlinear support, every `mechanics` combination row is withheld with `NONLINEAR_COMBINATION_REQUIRES_SOLVE`; case states are kept; `result_state_subtraction` stays as a labelled signed difference. Nonlinear kinds leave the `mechanics` list. REF-NL-C3 and a mutation control added. Part of M15 |
| **B-2** source-blocks-1 interaction | §5.7. The selected-with-failed mixture is already not eligible (evidence in §2.1(6)). The live exposure is selected-with-ordinary: those envelopes publish as today, but become not Current, rule- or export-eligible until T3. I propose deriving the reason in the readers from the receipt rather than adding a producer diagnostic (§12 R-1). All-selected envelopes keep their standing, with a notice (§12 R-2). §2.1(6) and decision 4 corrected. Mixed-envelope tests added. Producer rendering of the new semantics is deferred until the envelope identity is known (§5.6) |
| **SF-1** constant effort Σfactors | §5.6: when a consuming constant-effort support exists and Σfactors ≠ 1 (with a representation guard), `mechanics` combinations are withheld with `CONSTANT_EFFORT_COMBINATION_REQUIRES_SOLVE`; subtraction stays. REF-CE covers combinations. Part of M11 |
| **SF-2** mixed moduli | §5.6: operands on different modulus bases → `mechanics` combinations withheld with `COMBINATION_MODULUS_BASIS_MIXED`; subtraction stays, labelled; test added |
| **SF-3** merge constraint | §10.1: producer, readers and supersession gates land in one atomic PR |
| **SF-4** route helper | §5.7: a static rule (precision-1 is never admitted as a fresh identity), plus envelope-consistency checks. The identity set is a T1 obligation (§10.3) |
| **SF-5** missed consumers | §6 and §10.1 add all of them. The `physics_audit_regression` benchmark is migrated rather than deleted (§12 R-3) |
| **SF-6** verification | §9: legacy-quantity detection adapter; new references for withheld attribution, non-consuming constant effort, the mixed source-blocks envelope, nonlinear withholding, constant-effort Σfactors; REF-M08-L accepts either tie location; the per-case displacement tie site (`PP:2064-2071`) gets REF-M33-TIE |
| **SF-7** report package | §5.7: the legacy report package refuses every fresh result until T6 (owner accepted) |
| **SF-8** intensified stress | §5.4a: per-case equal-factor measure `i·hypot(My,Mz)/Z` at member ends adjacent to markers and branches. Labelled, no default factor, no k, never combined. REF-I-L and REF-I-T added. It is the last producer slice (S2b) and can move to T4 |
| **N-1** | §5.2: only two or more residual-determined devices (rigid or nonlinear) on one node-DOF are ambiguous. REF-M05-SPRING-GAP is the positive control |
| **N-2** | §5.4: the table gives arc nominal stress rows a basis discriminator, and chord-frame endpoint force rows a truthful coordinate label |
| **N-3** | §5.6: a blocked envelope carries empty `preview_cases` and `combination_gates` and no headline |
| **N-4** | §5.4a: `component_stress_modifier_count` = the number of published intensified rows; `COMPONENT_EQUAL_FACTOR_INTENSIFICATION_APPLIED` replaces `COMPONENT_STRESS_MULTIPLIER_APPLIED` |
| **N-5** | Not adopted (ruling 13). A model with an arc has no stress headline |
| **N-6** | REF-B2, an indeterminate arc whose resultants legitimately depend on k |
| **N-7** | §9.3: native witnesses run on the owner's Mac |
| **N-8** | The probe is rerun through the captured entry `run_linear_static_preview_value_with_mode` |
| **S0-1** REF-CE | Frozen: model, balance rule, combinations, non-consuming variant |
| **S0-2** REF-B1 | Frozen: signed arc-frame values at five stations, chord-frame endpoint rows with the node-on-element sign, endpoint stress rows, and the pinned `y_reference`. REF-B1-TANGENCY adds the kinked case (60° at both ends) |
| **S0-3** SPRING | REF-M05-SPRING2: spring on UY and guide on UX at one node |
| **S0-4** COMB envelope | Moment envelope (max 1000 against a sum of 1500), plus a displacement-magnitude combination that discriminates |
| **S0 coverage** | REF-M05-R1-TRANSLATED; REF-M14-X1-SUPPORTS; REF-ATTR; REF-M14-TH; stress coverage (REF-B1 plus a seam unit test); REF-M33-TIE; headline-scope expectation; mm/kN and mixed-modulus variants. S1 length, X1 restraint sets and every zero scale are recorded under `revision_2.case_definitions` and `zero_scales_rev1` |
| **Addendum** imposed displacement | §2.2: on the default route it is **refused, not dropped**. Evidence from source and the probe. No containment is required; an optional targeted diagnostic is offered (M10) |
| **T1 at `500d88644`** | §10.2 and §10.3 updated. Serialization of T0R S3–S5 against T1 WP2–WP4 is stated |

## 1. Recommendation in brief

Option (d), under one new identity for fresh ordinary-route solves: `openpipestress.result_semantics/0.3.0/preview-physics-1` (name accepted, ruling 13).

**Repaired:**
- six signed support components;
- the certified circular maximum on straight members;
- all-case headlines with complete-domain withholding;
- signed-only combination algebra;
- constant-effort actions;
- a per-case equal-factor intensified measure at markers and branches (S2b).

**Withheld, with a reason:**
- SIF×k rows;
- arc maxima, and therefore the stress headline of any model containing an arc;
- undetermined support attribution;
- `mechanics` combinations that are physically invalid on this route: nonlinear supports, constant effort with Σfactors ≠ 1, or mixed moduli;
- combination maxima and intensified combinations (T6).

**Standing:**
- The legacy report package is unavailable for every fresh result until T6.
- Historical precision-1 records keep their bytes. They are not Current, rule- or report-eligible, and they carry a derived notice.
- Mixed source-blocks-1 envelopes lose Current standing until T3.

## 2. What the default route publishes today

The T0 probe found:
- M33: first-case headline (22.77 MPa against 34.16);
- M14: abs-sum summary (√2 high);
- M05: force-norm-only reactions (pure torque shows 0 N and no moment);
- M08: SIF×k rows combined linearly (28.28 MPa from cancelling cases).

Sites: `PP:1497-1507`, `8930`, `2180-2211`, `9128-9230`, `10584`.

### 2.1 Further observations

From the design probe; the captured-entry rerun is `_run_records/design_probe_output_rev2.log`.

1. **Constant-effort reaction is published as zero.** A consuming constant-effort support publishes `reaction_resultant = 0 N` while its applied-load row reads 375 N. `build_model` drops constant-effort supports from the linear support list (`PP:4526`).
2. **Constant effort is counted Σfactors times in combinations** (SF-1, reproduced). The combination 1.0·L-100 + 0.5·L-200 publishes the constant-effort applied load as 562.5 N.
3. **Nonlinear states are superposed in combinations** (B-1, reproduced). The same combination superposes nonlinear support reactions: NL-140 −361.384 N, NL-130-FRIC 0.790554 N. Categorical and count nonlinear rows do not enter combinations.
4. **Arcs.** A tangent-consistent arc publishes correct signed tangent-frame resultants (REF-B1 matches to about 1e-12), and they are independent of k. A `y_reference` inconsistent with the neighbouring pipes builds a kinked arc, 60° at both ends, with no diagnostic. Station maxima are sampled: quarter_3 exceeds the endpoint value.
5. **SIF rows scale with k at fixed actions** (C: 14.14 → 28.28 MPa).
6. **source-blocks-1 (corrected from revision 1).**
   - **How it is reached.** A fresh ordinary solve publishes source-blocks-1 when at least one case is source-selected. The gate is straight members, no components, nodal loads only (`source_recovery.rs:419-470`).
   - **What each case contains.** Every non-selected case in that envelope keeps precision-1-shaped rows: norm-only reactions and the abs-sum summary.
   - **The readers already refuse a receipt with a failed case.** A case whose ordinary attempt passes is recorded with `outcome: qualified` and an ordinary `selected_method`. A failed case makes the receipt `partial`. Readers admit only `aggregate == "qualified"` (`result_export/src/source_blocks.rs:1219-1235`). So:
     - a **selected + failed** envelope is already not eligible;
     - a **selected + ordinary** envelope is eligible today while its ordinary case carries the M05 and M14 defects.
   - **All-selected envelopes.** The abs-sum summary is taken over the endpoints and three stations. Nodal loads only means the section actions are linear on each span, so the circular maximum sits at an endpoint and the abs-sum over endpoints bounds it from above (at most √2 high). It is conservative but mislabelled.

### 2.2 Legacy `imposed_displacement` on the default route (ROOT addendum, M10)

It is **refused**; it is not silently dropped or ignored.

- **The authored shape is refused at the entry.** The typed operation writes the target `{type: support, support, dof}` (`P/core/model_operations/operation_applier/src/lib.rs:5200-5201`). The product's `LoadTargetInput` has only `node` and `element` variants (`PP:573-578`). The captured entry therefore returns an error with no envelope: `request: unknown variant 'support', expected 'node' or 'element'` (probe E). The desktop solve command fails with that text. Nothing is solved and nothing is Current.
- **A node-targeted imposed displacement is refused as incomplete.** `parse_category` (`PP:10918-10931`) has no `imposed_displacement` variant, so the load is refused with blocking `LOAD_INPUT_INVALID` and `MODEL_INCOMPLETE` (probe E′).
- **The lower-level capability is never used.** `primitive_loads` can carry `imposed_displacements` (`loads/primitive_loads/src/lib.rs:2500-2535`), but the product never consumes it; only the source-recovery gate reads it (`source_recovery.rs:462`).

**Conclusion:** there is no silent-wrong exposure, so no T0R containment is required.

**Targeted diagnostic (ruled in as a small S2a item; SF-E).**
- **Where:** in `run_linear_static_preview_value_with_mode`, before `CapturedInvocation::parse`, inspect the request `Value`. If `model.schema_version` is not `0.4.0` and any `model.load_cases[].primitive_loads[].target.type == "support"`, refuse the request.
- **What is returned:** a blocked envelope. Status `MODEL_INCOMPLETE`, no results, and one blocking `IMPOSED_DISPLACEMENT_REQUIRES_LOAD_STATE_MODEL` per offending load, naming the load, case and support.
  - To build the typed identity of that envelope, the offending loads are removed from a *clone* of the value and the clone is parsed. Nothing is solved from it.
  - If the clone still fails to parse, the original error is returned.
  - The capture digest is not used, because no receipt is built.
- **What is not changed:** no `LoadTargetInput` variant is added. A variant would reach self-weight, validation, `operation_applier` (which T1 is editing) and the exact and 0.4.0 routes. The typed entry `run_linear_static_preview_with_mode` has no value, so it keeps the caller's serde error.
- **Scope:** 0.4.0 is excluded; T1 owns its boundary motion.
- **Test:** the authored shape gives the targeted diagnostic and no results. The negative control is that no envelope ever omits the load and solves.

The UI letting users author a load the solver refuses is the known M10 UI/DTO mismatch, left with T1 and UI-SUCCESSOR.

## 3. Distinctions this design depends on

**Physical section quantities versus code assessment.**
- A physical section quantity is a signed action, a support action, or an elastic stress of the represented beam model.
- The owner now permits derived intensified quantities built from the user's own SIF inputs, labelled with their formula and inputs (OWNER_SIF_DECISION).
- Allowables, stress categories and code-specific acceptance stay user rules. A code-specific effective section modulus, for example a branch Ze, stays a user rule too.

**Straight versus curved members.**
- On a straight member, beam stress is the represented physical stress.
- On an arc it is nominal: ovalization and curved-beam effects are T4.

**State versus derived quantity.**
- Linear algebra applies only to signed state quantities.
- Magnitudes and maxima are recomputed or envelope-selected.
- Intensified measures are never combined.

**When linear superposition is valid.** A `mechanics` combination is a physical state only when the operator is shared and linear, and persistent device loads are counted once:
- nonlinear supports break it (STRESS_REFERENCE C3);
- consuming constant effort breaks it unless Σfactors = 1;
- different modulus bases break it (C4).

`result_state_subtraction` is a labelled difference of states and needs none of these conditions.

**Complete domain.** A headline covers the complete declared domain, or it is withheld.

## 4. Options compared

The conclusions of revision 1 §4 (archived) stand:

- **(a)** Repair under a new identity. Done honestly, it becomes (d).
- **(b)** Contain only. It stops harm but repairs nothing; users get no reactions and no headline.
- **(c)** Exact profile as the authoring default.
  - It refuses bends, fittings, nonlinear and constant-effort supports, combinations, equivalent-static and E/G-only models.
  - It leaves every existing 0.2.0 document on the defective route.
  - It is the convergence path (§13).
- **(d)** Recommended.

Revision 2 adds two things to (d): the combination gates and the accepted report outage.

## 5. Recommended design: `preview-physics-1`

### 5.1 Identity and versions

| Family | Decision |
|---|---|
| New identity | `openpipestress.result_semantics/0.3.0/preview-physics-1`. Table `P/fixtures/results/semantic_contract_v0_3_preview_physics_1.json`; `inherited_semantic_contract_sha256` = precision-1 `d75aacee…`; `retired_source_kinds`; `combination_policy`; `contract_evidence_policy`; `supported_profile_limitations` |
| Which models it applies to | `pressure_runtime::is_exact == false`: models 0.1.0, 0.2.0 and 0.3.0+`legacy_pressure_v1` with zero pressure, including blocked envelopes. The envelope is emitted **only when no case is source-selected** (§5.6, §5.7) |
| Raw, producer, canonical, analysis, stress-neutral | Unchanged at 0.2.0, 0.2.0 and 0.3.0. Each registry gains one id/hash pair |
| Formulation profile | `product_preview_mechanics_v1`, with the §5.8 limitations |
| Model document | Unchanged |
| precision-1 | Frozen and historical-only (§5.7) |
| source-blocks-1 | Producer unchanged. Reader standing as in §5.7 |
| Reserved `reactions-1`, `stress-1` | Stay inactive |
| physics-1, physics-source-1, T1's identities | Untouched. T1's 0.4.0 is exact-route |

### 5.2 Reactions (M05)

Each device publishes six `support_reaction_component_v2` rows (global frame, support-on-pipe, at the attachment node), plus the two magnitude rows computed from them. `reaction_resultant` is retired.

Device laws:
- **Rigid restraint:** the nodal residual `K u − f` on its restrained DOFs, and physical zero elsewhere.
- **Spring:** `−k·u`.
- **Consuming constant effort:** its constant force along the +axis of its DOF (DEC-049).
- **Non-consuming constant effort:** rows withheld as `CONSTANT_EFFORT_NOT_CONSUMED`.
- **Nonlinear support:** its DOF's residual.

**Attribution (N-1).** Two groups of devices behave differently.
- **Law-determined devices** (springs, constant effort) take their force from their own law. The residual never contains it:
  - `K` includes the spring diagonal (`PP:1383-1385`);
  - the solve makes `K_ff u_f = f_f`;
  - the constant-effort force is already in `f` (`PP:9764`).
- **Residual-determined devices** are rigid restraints and nonlinear supports.

Only a node-DOF acted on by two or more residual-determined devices is ambiguous:
- two rigid restraints on one DOF already block (`PP:1356`; `validation.rs:70`);
- the case that remains is rigid + nonlinear, or nonlinear + nonlinear, across records.

Those devices' rows are withheld with `SUPPORT_ACTION_ATTRIBUTION_WITHHELD`, and so is the nonlinear device's `nonlinear_support_final_reaction` for that DOF. Nothing is zero-filled or duplicated. REF-ATTR is the negative case; REF-M05-SPRING-GAP and REF-M05-SPRING2 are the positive cases.

**Combinations.** Only admitted `mechanics` combinations (§5.6) and subtractions carry support rows:
- the signed components combine linearly;
- magnitudes are recomputed from the combined vector;
- rows are grouped by (kind, support, component), because v2 ids embed the case id (`PP:1535`);
- `support_force_vectors` becomes six-component.

A range envelope selects values component by component and is labelled as not a simultaneous state.

### 5.3 Straight-member stress (M14)

`open_formula_stress_summary` is retired and replaced by `pipe_elastic_normal_stress_maximum_v2`: unit Pa, `governing_station`. It is computed by `exact_straight_summary_extrema` (`PP:7487`) with no pressure. The published value is the enclosure midpoint; the bounds go in the evidence.

- Thermal axial force enters through the corrected end actions (REF-M14-TH).
- Torsion stays in its own signed rows.
- A member with no maximum is listed in the coverage as unavailable.
- Per-member ties and per-case displacement ties break by identity, as on the exact route. The ordinary branch's order-dependent comparisons at `PP:2064-2071` and `PP:2690-2702` change accordingly.

### 5.4 Bends: what the ordinary route may truthfully publish until T4

| Representation | Published | Withheld |
|---|---|---|
| `curved_bend_macro_element` arc | Signed tangent-frame resultants at the ends and three stations (REF-B1-SIGNED). Nominal stress components; the table gives them a basis discriminator (N-2) such as `nominal_straight_beam_formula_on_arc_resultants`. Endpoint force rows stay in the chord frame with node-on-element signs, and the table labels them with a truthful coordinate token (N-2) such as `arc_chord_frame`, not `element_local`. k enters stiffness only. `CURVED_BEND_TANGENT_DISCONTINUITY` (warning) is raised when an arc end tangent and the adjacent pipe direction differ by more than the geometric-consistency warning threshold of 1e-6 rad (the value of `DEC_070_CURVED_BEND_ANGLE_MATCH_TOLERANCE`, reused as a warning threshold, not a representation guard); the check uses `CurvedBendMacroElement::end_tangents()`. It does not close M02 | Any arc maximum, and so the stress headline (`PREVIEW_STRESS_HEADLINE_WITHHELD`). Any intensified row on the arc: its end values are not the arc's governing station |
| Geometry-only marker | Adjacent straight maxima. The equal-factor intensified measure at the adjacent member ends (§5.4a) | SIF×k rows |
| Branch or tee | Same as the marker, per referenced side | Same as the marker; an unreferenced pipe at the node gets no row |

**Vocabulary (NOTE-5).** The `arc_chord_frame` coordinate token and the arc nominal basis token are added to:
- the new table's `canonical_metadata_vocabulary`, with signatures that use `source_basis` as the discriminator;
- the stress-neutral method CSV mapping in `package_v0_3.py` and its TS mirror;
- the Python and TS signature readers.

Existing tables keep their vocabularies.

### 5.4a Equal-factor intensified measure (SF-8; producer slice S2b, can move to T4)

**What it is.** A new kind, proposed name `component_equal_factor_intensified_bending_stress_v1`, unit Pa. One row per case, per associated member end at the component node:
- a geometry-only bend marker (`mechanics_geometry_only`): every pipe end at its node. It needs a SIF only; k is neither required nor used;
- a branch: the `branch_header_pipe_ref` end with the header SIF, and the `branch_branch_pipe_ref` end with the branch SIF.

**Value:** `i·hypot(My, Mz)/Z`. Its parts:
- My and Mz are the section-cut bending moments at that end. hypot is invariant to rotation about the member axis, so no component frame is needed.
- Z is the member's own section modulus.
- `i` is the user's scalar SIF.

It includes no k, no axial term and no torsion.

**Labels.** The metadata names the formula, the factor consumed, its source reference, and "member Z; not a code effective section modulus; not a code stress".

**Rules:**
- No user SIF means no row, plus an info diagnostic; there is no default factor.
- A pipe ending at a branch node without a reference gets `COMPONENT_INTENSIFIED_COVERAGE_INCOMPLETE`.
- Never combined: combinations get `COMBINATION_INTENSIFIED_STRESS_UNAVAILABLE` (T6).
- Not part of the headline.
- `curved_bend_macro_element` components produce **no** intensified row anywhere, neither on the arc nor on straight neighbours at the component node (NOTE-1). They are T4.

**Counting (N-4).** `component_stress_modifier_count` = the number of intensified rows published, and each row carries `COMPONENT_EQUAL_FACTOR_INTENSIFICATION_APPLIED` (info). If S2b moves to T4, the count is 0 and each component gets `COMPONENT_STRESS_INTENSIFICATION_NOT_APPLIED`.

**References:** REF-I-L and REF-I-T.

### 5.5 Headlines (M33)

- **Stress:** `summary.max_open_formula_stress`, in Pa. It is `maximum_across_cases(…, true)` over all load cases, withheld when any case's member domain is incomplete (arcs, unavailable members).
- **Displacement:** `maximum_across_cases(…, false)`. `HIGH_DISPLACEMENT_REVIEW` (`PP:1583-1595`) is evaluated after this selection, on the same all-case value. Today it inherits the first-case value, a residual of M33 (SF-A).
- **Scope:** combinations are excluded. `PREVIEW_HEADLINE_SCOPE_LOAD_CASES` (info) is raised when combinations exist.
- **Mixed moduli** remain T6.

### 5.6 Combinations, evidence and rendering

**`mechanics` combination gate.** All `mechanics` combination rows are withheld, with one diagnostic per combination and the case states kept, when any of the following holds:
1. the model has a nonlinear support record: `NONLINEAR_COMBINATION_REQUIRES_SOLVE` (ruling 1);
2. the model has a consuming constant-effort support and `|Σfactors − 1| > 64·ε·max(1, Σ|factors|)`: `CONSTANT_EFFORT_COMBINATION_REQUIRES_SOLVE` (ruling 3). The guard is the representation guard `pressure_runtime.rs` already uses, not an engineering tolerance;
3. operands have different modulus-basis keys (`PP:6844`): `COMBINATION_MODULUS_BASIS_MIXED` (ruling 4).

The gate does not apply to the other two combination forms:
- `result_state_subtraction` stays published, labelled as a signed difference of states with no equilibrium claim. A constant-effort force cancels in it.
- `range_envelope` stays as a labelled selection.

**Kinds that may be combined** (admitted `mechanics` and subtraction):
- global nodal displacement and rotation components;
- element-local end and station forces and moments;
- signed element-local stress components, and the zero pressure rows;
- `support_reaction_component_v2`;
- `constant_effort_support_applied_load`.

Nonlinear kinds are removed. `displacement_magnitude` and the support magnitudes are recomputed. Maxima, intensified measures, reviews, records, counts, codes and residuals are never combined. `COMBINATION_STRESS_MAXIMUM_UNAVAILABLE` replaces `COMBINATION_STRESS_SUMMARY_SKIPPED`.

**`contract_evidence`**, closed; field names are frozen at S1:
```
{"preview_cases": [{"load_case_id": ...,
                    "pipe_stress_extrema": [<13-field exact shape>],
                    "stress_maximum_coverage": {"complete", "unavailable_pipe_ids", "outside_domain_pipe_ids"},
                    "support_attribution": {"attributed_support_ids", "withheld": [{"support_id", "reason"}]}}],
 "combination_gates": [{"combination_id", "withheld": bool, "reason"}]}
```
A blocked envelope carries `{"preview_cases": [], "combination_gates": []}` and no headline (N-3).

**Reader checks,** identical in Rust, Python and TS:
- only kinds from the table; retired kinds rejected;
- each maximum binds to its enclosure and lies within the bounds;
- coverage matches the members;
- the headline appears exactly when coverage is complete, as the maximum across cases with identity ties;
- each support has six components plus consistent magnitudes, or a withheld record;
- combination rows exist only for admitted combinations; their references resolve, and their magnitudes match their components;
- no intensified row and no maximum for a combination.

**How the producer renders it (SF-A, S2a).**

*Why rendering waits for the whole case loop.* Source selection is decided per case, but the envelope identity is known only after the last case solves. So `solve_load_case` keeps producing today's rows, and adds a side record per case: six-vectors per device, enclosures, per-member and per-node identity-tie data, intensified inputs and attribution.

*Placement (chosen).* `source_selected` is computed at `PP:1496` from the completed `load_case_solves`, before anything is summarized or combined. The step goes straight after it:

```
let source_selected = …;                                        // PP:1496
let preview = (!is_exact(&model) && !source_selected)
    .then(|| preview_physics::render(&model, &mut load_case_solves, &mut diagnostics));
// PP:1497-1507: headline selection uses maximum_across_cases when exact, source_selected or preview
// PP:1520-1570: result loop; the v2 id rule (keep case-bearing v2 ids) also applies when preview
// PP:1571: append_combination_results(…, preview.as_ref())       // gate, list and grouping when preview
```

*Contract of `render`.* It is a pure function over the solved cases and the diagnostics. For each case it:
1. **Replaces `solve.results`.** It drops `reaction_resultant`, `open_formula_stress_summary` and `component_user_stress_multiplier_review`. It adds the v2 support rows and maxima, and (S2b) the intensified rows. Arc rows get their new basis and coordinate tokens.
2. **Sets per-case headline inputs.** `solve.max_stress` becomes the Pa maximum with identity ties, or `None` when coverage is incomplete. `solve.max_displacement` uses identity ties.
3. **Sets `solve.support_force_vectors`** to six components.
4. **Sets `solve.component_stress_modifier_count`** to the number of intensified rows.
5. **Edits `diagnostics`.**
   - It removes every retired code: `COMPONENT_STRESS_MULTIPLIER_APPLIED` and any diagnostic whose affected refs name a removed row.
   - It adds the new codes: attribution withholding, `CONSTANT_EFFORT_NOT_CONSUMED`, the coverage and headline codes, intensification, coverage-incomplete, the kink warning and the headline-scope info.

It returns the `preview_cases` evidence and a combination context (the model's gate facts). Everything downstream is then built once, from the new rows:
- the headlines;
- `HIGH_DISPLACEMENT_REVIEW` and its refs;
- `component_stress_modifier_count` in the summary;
- `append_combination_results`, which applies the gate, the list, the grouping and the new `COMBINATION_*` codes in place of `COMBINATION_STRESS_SUMMARY_SKIPPED`;
- the finite-value check.

After the `MechanicsEnvelope` literal, one added line sets `contract_evidence` for preview, and the producer and formulation basis are chosen from the rendered flag. The literal's `contract_evidence` expression, which T1 edits, stays as it is.

*When a case was source-selected,* nothing is rendered, and the envelope is today's source-blocks-1 byte for byte. That includes its `product_preview_mechanics_v1` limitations text: the new limitations of §5.8 apply only when rendered. `source_row_bindings` and `validate_summary` (`source_receipt/rows.rs:736`) are unaffected.

*Blocked envelopes.* `blocked_envelope` sets the empty preview evidence in one added statement after its literal, for non-exact models (N-3). T1's expression is not edited.

*Tamper and completeness checks (SF-A).* A rendered envelope must contain no retired kind and no retired diagnostic code. Every `affected_refs` and `result_ref` entry of every diagnostic and summary must resolve to an emitted row or a model entity. All three readers test this, and so does a producer test.

### 5.7 Historical records, source-blocks-1 and standing

**Static identity rule (SF-4).** The fresh identities are {preview-physics-1, source-blocks-1, physics-1, physics-source-1}, plus T1's identities once activated. precision-1 is never admitted as a fresh publication. The rule is the same constant set in each language, with no route predicate. Existing envelope-consistency checks (profile against identity, `contract_evidence` presence) back it up. Current, rule-check, report and Current export admission all use it.

**precision-1:** readable and verifiable. Not Current, rule-, report- or Current-export-eligible. A derived notice lists M05 (norm-only reactions, 0 N constant effort), M14, M33 and M08.

**source-blocks-1:**
- **Selected + ordinary** (the live exposure, §2.1(6)): published as today, but readers set standing `needs_recompute` with reason `SOURCE_BLOCKS_ORDINARY_CASE_LEGACY_SEMANTICS` when any receipt case has an ordinary `selected_method`, until T3. The rule applies **only to non-composite source-blocks-1** (policy `SOURCE-BLOCKS-1`). physics-source-1 legitimately carries ordinary cases under physics-1 semantics (NOTE-2). The reason is derived in the Rust, Python and TS source-blocks readers from the receipt, not added by the producer, because this way:
  - the source-blocks-1 producer bytes and receipt code T1 is editing stay unchanged;
  - it applies equally to stored envelopes;
  - it avoids the T1 overlap region in `lib.rs`.
  If ROOT prefers a producer diagnostic as well, it goes in the post-loop step of §5.6.
- **Selected + failed:** already not eligible (receipt `partial`). A test pins this.
- **All-selected:** keeps its Current standing for its other quantities (ROOT ruling R-2). Its abs-sum summary may not be relied on as if correct.
  - **Notice text,** for the results view, Current exports (NOTE-3) and the rule-refusal reason: "Summary stress in this retained-source result is the sum of absolute axial and bending components, not the circular-section maximum. The admitted loads are nodal only, so it is conservative and at most √2 (about 1.414) times the maximum. Rule checks cannot bind to it until T3."
  - **Rule-binding enforcement (SF-B).** A shared helper in `result_export`, `semantic_contract::rule_binding_refusal(envelope, row) -> Option<&'static str>`, returns `RULE_SOURCE_BLOCKS_SUMMARY_NOT_RELIABLE`. It does so for any `open_formula_stress_summary` row, or the headline's `result_ref`, in a non-composite source-blocks-1 envelope. It is called at every binding site:
    1. `P/apps/desktop/src-tauri/src/lib.rs` `solver_result_row_value` (`3184`). Both `resolve_solver_result_bindings` (`3202`) and `resolve_authored_solver_result_bindings` (`3248`) use it, so neither path can bind the row.
    2. `P/core/rules/rule_check_runner`. `RuleCheckRunInput` gains `refused_solver_results: Vec<(input_id, reason)>`, and the runner reports those inputs as `RULE_INPUTS_INCOMPLETE` with the reason, never as a pass. The runner itself sees no envelope.
    3. `apps/desktop/src/services/ruleCheckService.ts`: its pre-check mirrors the helper, so the UI shows the reason before invoking.
    4. `P/core/runner/headless`: no solver-result binding path exists today (only an aggregate is passed). A test pins that no headless path binds solver rows. Any later binding path, for example T1 WP4 or T6, must call the helper.
  - **Tamper test:** a rule pack bound to `result:stress:<pipe>` in an all-selected source-blocks-1 fixture reports incomplete with the reason, in all three places. A binding to a force row in the same envelope still resolves.

**Legacy report package:** refuses every fresh identity until T6, owner accepted. After T0R, no fresh result is report-package eligible.

**Browser preview:** remains the historical precision-1 bundles, not Current, with the notice. The T6-owned precision-1 fixture pair is not regenerated.

### 5.8 Formulation limitations for the new identity

1. Small-displacement, linear-elastic Euler–Bernoulli frame preview; numerical integrity does not establish physical correctness.
2. Nonzero pressure is refused on this route, and so is legacy `imposed_displacement`.
3. On straight members, the normal-stress maximum is |N/A| + hypot(My,Mz)/Z, bounded over all statics intervals. Torsional shear is separate. No transverse shear, equivalent stress or code stress.
4. On arcs: signed tangent-frame resultants and nominal stress components. Endpoint force rows are in the chord frame. No maximum; no stress headline for a model containing an arc.
5. Intensified measures are `i·hypot(My,Mz)/Z` with the user's scalar SIF at member ends adjacent to markers and branches. Member Z. They are not code stresses and are never combined. k never multiplies stress.
6. Signed support actions for rigid restraints, springs, consuming constant effort and attributable nonlinear supports. Ambiguous attribution is withheld.
7. Headlines cover all load cases. `mechanics` combinations are withheld for nonlinear supports, constant effort with Σfactors ≠ 1, or mixed moduli. Subtraction is a labelled difference. No combination maxima and no code compliance.

## 6. Consumers that must change

Rows marked **new** were added in this revision.

| Area | File | Change |
|---|---|---|
| Producer | `P/core/product_physics/src/lib.rs` | Call sites and the post-loop step |
| | new `src/preview_physics.rs` | All new logic |
| | `lib.rs` unit tests asserting retired rows, including `result:reaction:*` at about `PP:15009` and `18292-20443`, the self-weight tests and `bend_component_user_multipliers_emit_stress_review_rows` (~`15210`); also `src/source_budget_tests.rs` if affected | Disposition table |
| Producer (rev. 3) | `P/core/loads/self_weight_wasm/tests/applied_self_weight.rs` | Migrate its `result:reaction:support-root` assertions to signed v2 components (SF-C) |
| Producer (new) | `P/validation/benchmarks/physics_audit_regression/{src/lib.rs, tests/authored_units_product.rs}` | Migrate `reaction_resultant` assertions to v2 rows. `pure_moment_does_not_become_force` keeps its zero-force assertion and gains the signed moment Mz = −100 N·m |
| Rust readers | `P/core/reporting/result_export/src/{semantic_contract.rs, derivative.rs, lib.rs}` | Registry, dispatch, static fresh set |
| | new `.../preview_physics_evidence.rs` | Reader checks |
| | `source_blocks.rs` (new) | Ordinary-case standing; `physics_evidence.rs` and existing tests only where they enumerate ids |
| Rust runner | `P/core/runner/headless/src/{lib.rs, result_envelope_binding.rs}` and tests | Admission and expectations |
| Rust rules (rev. 3) | `P/core/rules/rule_check_runner/src/lib.rs` | `refused_solver_results` input, reported as incomplete with its reason (SF-B) |
| Rust app | `P/apps/desktop/src-tauri/src/lib.rs` | `qualify_rule_mechanics_with_context`; (new) the packaged self-test (`4309-4660`, which pins producer headers at `4921`) and native tests (`5084`, `5150`) |
| Python | `P/core/analysis_runs/{compatibility.py, source_blocks.py (new)}`; new `preview_physics_evidence.py`; `P/core/handoff/stress_neutral/package_v0_3.py` | Registry, readers, packaging |
| Schemas | The three `P/schemas/*v0.3*` files | Enumerations and evidence shape |
| Tools | `P/tools/serialization/generate_product_preview_mechanics.mjs` | New mode |
| TS | `features/results/{numericalResultQuality.ts, resultSemantics.ts, sourceBlockRecovery.ts (new)}`; new `previewPhysicsEvidence.ts`; new `knownSemanticLimitations.ts` | Dispatch, pins, readers, notice |
| | `features/workspace/resultsSessionState.ts` | Static set |
| | `features/report/{reportPackageRequest.ts, ReportPanel.tsx}` | Unavailable reason; SIF section |
| | `features/results/ResultsPanel.tsx` (new), `features/comparison/ComparisonPanel.tsx` (new) | Headline label, withheld reason, new kinds |
| | `services/{previewService.ts, ruleCheckService.ts, analysisRunCompatibility.ts (new: preview-evidence check)}` | Dispatch and checks; `ruleCheckService.ts` also mirrors `rule_binding_refusal` (rev. 3) |
| | Stress-neutral and result-export panels; Handoff, LocalFeaHandoff and NativePackage | Headline label |
| | `test/nativeMechanicsReplay.ts` (new) and TS tests using fixtures (new): `previewService.test.ts`, `rendererIntegration.test.tsx`, `currentResultUnitPolicy.test.tsx`, `sourceBlockRecovery.test.ts`, `physicsResultExport.test.ts`, e2e `r2-smoke.spec.ts`, `gui-workflow-validation.spec.ts` | Replay and fixture use |
| Derived fixtures (new owner: S2a) | New `P/fixtures/results/preview_physics_connected_{sparse,dense}.json` with request; the preview-physics browser bundle if S5 needs one | Generated only by the actual producer |

Historical fixtures stay unchanged.

## 7. The minimum UI change

Text only; no authoring, model or layout change:
1. reader dispatch, not visible;
2. the headline label ("maximum elastic normal stress; nominal; no component intensification; not a code stress") and its withheld reason;
3. the historical notice on precision-1 and on the source-blocks-1 abs-sum, including on Current exports of all-selected source-blocks-1 envelopes (NOTE-3);
4. the report's unavailable reason;
5. the rule-check supersession message;
6. (new) combination-gate reasons where combination rows would appear;
7. (new) the mixed source-blocks standing reason;
8. (new) a label for the intensified measure where it is listed.

## 8. Test plans for the other options

As in revision 1 §8 (archived):
- (b) has negative controls only;
- (c) needs authoring-default witnesses and leaves the ordinary route untested.

## 9. Verification plan

### 9.1 Frozen references

**Source.** `references.py` defines them and `_run_records/references.stdout.txt` holds the output. First principles only: rigid-body statics, Euler–Bernoulli and spring compatibility, the elastic annulus, thin-curved-beam unit-load compliance, and the product's documented arc construction for tangency.

**Checking.**
- S0 checked the revision-1 values.
- S0′ checked the `revision_2` values: 256 of 257 agreed, with one mislabelled control, now fixed.
- The revision-3 changes are listed in the Revision 3 table: COMB-2 labels, I-L labels, I-T loads, the B2 chord frame, the JSON pins, the tie rule, the ATTR exemption and the TH moment scale. They need the narrow backcheck.

**Comparison.** `|observed − expected| ≤ 1e-9·max(|expected|, scale)`, using the stated zero scales, plus X1 enclosure containment. No new tolerance.

**Global balance.** Every reference checks force and moment balance from published support rows and applied loads, with one exemption: REF-ATTR. Both of its tip devices are withheld, so published rows cannot close the balance. It asserts instead that the anchor rows are zero, both withheld records are present, and no tip row is zero-filled.

**Tie rule (SF-D, S0′-6).** For REF-M33-TIE and REF-M08-L:
- Record the raw tied values in both input orders and both solver modes.
- If they are bitwise equal, assert the identity tie-break: the same location for either order.
- Otherwise, accept either location when both values lie within 1e-9 of the reference, and report "tie not exercised" rather than pass or fail.

Revision-1 references are unchanged: REF-M14-A, S1, X1, M33-G, M05-T, R1, SPRING, COMB, M08-L, B1.

REF-M08-L's tied locations are `pipe:a-b` at a and `pipe:b-c` at b, under the tie rule above.

| New reference | Proves | Negative control |
|---|---|---|
| REF-M05-R1-TRANSLATED | Moments are taken about the attachment node | M about the global origin, (34, 185, 152) |
| REF-M05-COMB-2 | Moment envelope 1000 over A1/T, and 1000 over A1/A2; A1 ± A2 displacement magnitude 0.447251 mm recomputed from components | Sums: 1500 over A1/T, 2000 over A1/A2; magnitude sum 0.632508 mm or difference 0 |
| REF-M14-X1-SUPPORTS | Pin and roller values for rigid rows that restrain only some DOFs. It catches DOF-slot mapping errors only | A reaction in the wrong DOF slot |
| REF-M14-TH | Thermal axial force in the maximum: 240 MPa; anchors ±829380.46 N; zero scales 829380.46 N and 49762.83 N·m | Missing thermal axial force |
| REF-CE | Constant-effort action (0,375,0;0,0,0); balance with the force counted once; 0.5/0.5 combination published (anchor Fy 375); 1.0/0.5 withheld; subtraction published; non-consuming variant withheld | Main's 0 N; 562.5 N applied; anchor 687.5 N from linear algebra against 875 N if actually solved |
| REF-M05-SPRING2 | Two devices at one node on different DOFs: spring −240.268 N, guide −2000 N, anchor Fx 0 | Nodal total copied to both devices |
| REF-M05-SPRING-GAP | N-1: spring −84.094 N and inactive gap 0 are both published | Over-withholding |
| REF-ATTR | Guide + one-way (`active_when: positive_reaction`) on one DOF: both withheld; anchor 0; tip total 1000 N. Exempt from the global-balance rule | A zero-filled tip row |
| REF-NL-C3 | `gap` with `closes_when: positive_displacement`, gap 0.4 mm. Nonlinear `mechanics` combination withheld; cases kept (0.253003 mm, stop 0); subtraction labelled | Superposition 0.506007 mm penetrates the 0.4 mm gap; true solve 0.4 mm with −335.195 N |
| REF-I-L | Intensified: L1 b-c end 13.0929 MPa, a-b end 0 (torsion only); L2 b-c 18.5162 MPa, a-b 13.0929 MPa; unchanged when k changes; published with a SIF and no k (13.0929 MPa); none in combinations; none without a SIF | abs-sum 26.1859; with k 19.9975; axial added unintensified 13.3823 or intensified 13.4257 |
| REF-I-T | Loads (800, 0, 1000) at c and Fz 500 at e. Header 13.9630 MPa (bending 943.398 N·m), branch 29.1602 MPa (1280.62 N·m); unreferenced continuation gets a coverage diagnostic | Unreferenced pipe given a row; a header value equal to b-e's, 7.40035 MPa |
| REF-B1-SIGNED | Signed stations, e.g. midspan T 765.685 and My −848.528; chord endpoint rows; endpoint stress rows. With the bend's SIF set to 1.3, no intensified row appears on the arc or on c-d (NOTE-1) | Any in-plane action; frame mix-up; an intensified row on a macro-bend |
| REF-B1-TANGENCY | Consistent arc: no warning. Kinked arc: warning, 60° at both ends | Silent kink |
| REF-B2 | Indeterminate arc: roller 589.860 / 612.803 / 624.599 N for k = 1 / 2 / 4. Chord-frame cut face at b, e.g. k = 2: (1140.42, −273.790, 0, 0, 0, −77.4395); the product's end_i rows are its negation. S2a records whether a lone arc span is accepted | A test asserting k-independence here; any stress scaled by k at fixed resultants |
| REF-M33-TIE | Displacement tie 0.316254 mm, under the tie rule | Input-order selection on a bitwise tie |
| Imposed displacement (rev. 3, SF-E) | The authored support-target load gives a blocked envelope with `IMPOSED_DISPLACEMENT_REQUIRES_LOAD_STATE_MODEL`, and no results | Any envelope that solves without the load |
| Rendering completeness (rev. 3, SF-A) | No retired kind or code; every `affected_refs` and `result_ref` resolves; `HIGH_DISPLACEMENT_REVIEW` uses the all-case value | A first-case review value, or a dangling ref |
| Rule-binding refusal (rev. 3, SF-B) | All-selected source-blocks-1: a binding to the abs-sum summary is refused with the reason at each site; a force row still binds | The summary binds |
| Mixed source-blocks (from the existing `multicase` and `n05`/`n06` source-block requests) | Selected + ordinary is published but not Current, with the standing reason. Selected + failed is not eligible. All-selected keeps its standing and carries the notice | Selected + ordinary admitted as Current |
| Mixed modulus (test only) | `mechanics` withheld as `COMBINATION_MODULUS_BASIS_MIXED`; subtraction labelled | Published |
| Stress coverage | REF-B1 headline withheld (outside domain); a seam unit test in `preview_physics.rs` for an unavailable member | Headline taken from the available subset |
| Headline scope | REF-M05-COMB models carry `PREVIEW_HEADLINE_SCOPE_LOAD_CASES` | — |
| mm/kN variant | REF-M05-R1 authored in mm and kN gives identical N and N·m rows | Unit-dependent rows |

Two limits on these references:
- **REF-B2:** with only one redundant, k enters through the ratio of bending to axial compliance, so the sensitivity is modest (about 6% from k = 1 to 4). It is still far above the criterion.
- **REF-ATTR:** if the product refuses the pair before solving, a targeted refusal is acceptable, and S2a records which one happens.

### 9.2 Detection adapter and controls (SF-6)

**Detection adapter.** Before implementation, run a read-only adapter on main: an out-of-repository crate like the design probe, using the captured entry. It maps each reference's quantity to main's legacy quantity:
- force magnitude → `reaction_resultant`;
- member maximum → `open_formula_stress_summary`;
- headline → `summary`;
- intensified measure → the SIF row;
- combination → combination rows.

It records a pass or mismatch for each quantity. Expected mismatches: M14-A, S1 unrotated, X1, M33-G (A first), M05-T (moment absent), COMB magnitudes, CE, M08-L (SIF row present), I-L1 (i·k·σ = 14.14 against 13.09 MPa), I-L2, I-T, NL-C3 (published combination), CE 1.0/0.5 (published). Per S0, some references discriminate only in combination: M33 needs the A-first order, S1-rotated only pairs with S1, and pure torque does not detect the abs-sum.

**Mutation controls,** each of which must fail:
- the abs-sum objective;
- the `first()` headline;
- dropping moments;
- linear combination of magnitudes;
- restoring the SIF×k row;
- publishing an arc maximum;
- zero-filling a withheld support;
- removing any of the three combination gates;
- intensified rows that include k or axial force;
- an intensified row appearing in a combination.

**Reader tamper tests,** in all three languages:
- retired kinds;
- a headline that does not govern, or appears while coverage is incomplete;
- a value outside its bounds;
- inconsistent combination magnitudes;
- a combination row for a gated combination;
- a missing withheld record;
- a precision-1 envelope offered as fresh;
- a selected + ordinary source-blocks-1 envelope offered as Current (non-composite only; a physics-source-1 envelope with ordinary cases stays admitted);
- (rev. 3) a rendered envelope carrying a retired diagnostic code, or a diagnostic ref that does not resolve;
- (rev. 3) a rule binding to an all-selected source-blocks-1 summary row.

**History.** Every precision-1 and source-blocks-1 fixture, table and hash-verification test passes byte-unchanged.

### 9.3 Native witness and gates

On the owner's Mac (N-7), with candidate, model hash and case ids recorded:
- a fresh 0.2.0 model with a marker, a combination and one nonlinear support: the combination is withheld with its reason, and the Current standing is recorded;
- a rule pack bound to the new maximum or intensified id;
- a rule pack bound to a retired id: `RULE_INPUTS_INCOMPLETE`;
- exports, with the report unavailable;
- save and reopen; a precision-1 result reopens with the notice and is not Current;
- an arc model: headline withheld, and the kink warning when `y_reference` is inconsistent;
- an imposed-displacement load: refusal shown.

Then hosted CI, including the dual-viewport dispatch, and a clean DEC-025 sweep.

## 10. Slices, order and integration

### 10.1 Slices

Write sets are disjoint.

| Slice | Owner | Write set |
|---|---|---|
| S0′ backcheck of `revision_2` references | Done (`REFERENCE_CHECK/REVISION_2/`) | — |
| S0″ narrow backcheck of the revision-3 reference changes | Independent TASK arranged by ROOT | A new folder beside the S0 and S0′ archives |
| S1 table and interface freeze | T0R manager | New table; interface note (evidence fields, diagnostic codes, row id formats, the new intensified kind) |
| S2a producer core | The only T0R `core/product_physics` writer | New `src/preview_physics.rs`; the `lib.rs` call sites (§10.2), including the rendering step after source selection, the blocked-envelope evidence and the imposed-displacement check in the captured entry; unit-test dispositions (including self-weight); new `tests/preview_physics_runtime.rs`; `P/core/loads/self_weight_wasm/tests/applied_self_weight.rs` (SF-C); `P/validation/benchmarks/physics_audit_regression/**`; derived fixtures `P/fixtures/results/preview_physics_*` |
| S2b intensified measure (last; can move to T4) | Same writer | `preview_physics.rs`, its tests, and the table's intensified signature (via S1 amendment) |
| S3 Rust readers and rule binding | One TASK | `P/core/reporting/result_export/**`, including `rule_binding_refusal`; `P/core/rules/rule_check_runner/**` (`refused_solver_results`); `P/core/runner/headless/**`; in `P/apps/desktop/src-tauri/src/lib.rs`, the rule gate, `solver_result_row_value`, the packaged self-test and the native tests that pin producer identity |
| S4 Python, schemas, tools | One TASK | `P/core/analysis_runs/**`; `stress_neutral/package_v0_3.py`; the three v0.3 schemas; the generator's new mode; Python tests |
| S5 TS readers and minimum UI | One TASK (desktop convention) | §6 TS rows, including ResultsPanel, ComparisonPanel, analysisRunCompatibility, nativeMechanicsReplay and the fixture-using tests and e2e; `types.ts` only if a type is missing |
| S6 join | ROOT with the T0R manager | Records, graph, PR, CI, native witness |

**Order:** S0′ → S1 → S2a → S2b, while S3, S4 and S5 run in parallel once S1 is frozen and S2a's first raw output exists → S6.

**One atomic PR (SF-3):** producer, readers and gates merge together. If S2b slips, S6 proceeds without it; S2b then goes to T4, and the count and diagnostic follow §5.4a's fallback.

### 10.2 The shared facade and T1 (`500d88644`)

The T0R manager is the sole `core/product_physics` writer for T0R, and ROOT owns fan-in. T0R lands first. The T1 manager then merges main into `codex/piping-load-states-20260925`.

T1's `lib.rs` hunks at `500d88644` sit at the same places as at `c0ef4a8e0`:
- `817-842` producer and formulation;
- `1241-1303` entry;
- `1397-1516` case loop;
- `1612-1660` envelope and producer override;
- `1725-1983`;
- `2826-2886` evidence and `LoadCaseSolve`;
- `3294-3344`;
- `4365-4451`;
- `10679`.

| File or region | T0R | T1 | Kind |
|---|---|---|---|
| `lib.rs` `817-842` | Ordinary branch → new id and limitations | Load-state branch | Adjacent |
| `lib.rs` `1231-1233` captured entry (T1 `1402-1406`) | Imposed-displacement value check inserted before `CapturedInvocation::parse` | Only the `matches!` line further down | Adjacent |
| `lib.rs` `1496-1507` source selection and headline | The `render` call right after `source_selected`; the headline condition includes preview | Hunks at `1483` and `1516` | Adjacent |
| `lib.rs` `1520-1571` result loop and combinations | The v2 id rule when preview; `append_combination_results` gets the preview context | `load_reference_states` | Adjacent |
| `lib.rs` `1612-1660` envelope and override | One added statement after the literal (preview `contract_evidence`); producer and formulation from the rendered flag | Restructured `contract_evidence`; joined override | **Adjacent only** (revision 3). T1's expression is not edited |
| `lib.rs` `10677-10713` `blocked_envelope` | One added statement after the literal for non-exact models (N-3) | Edits the `contract_evidence` expression | Adjacent |
| `lib.rs` `2826-2886` `LoadCaseSolve` | Side record, six-vectors | `load_state_evidence` | Adjacent |
| `lib.rs` reactions, summary, SIF and combinations | Calls into `preview_physics` | — | None |
| `pressure_runtime.rs`, `source_receipt*`, `source_recovery.rs` | — | Heavy | None: the source-blocks producer is untouched |
| `result_export/src/semantic_contract.rs`, `derivative.rs`, `lib.rs` | Registry and static set | Load-reference registry (+75/+4/+1) | Mechanical |
| `result_export/src/source_blocks.rs` | Ordinary-case standing | — | None |
| `analysis_runs/compatibility.py`, `stress_neutral/package_v0_3.py`, the three schemas | Enumerations | Enumerations and the WP1 packager edit (owner approved) | Mechanical |

### 10.3 Serialization of T0R S3–S5 against T1 WP1–WP6

These follow ROOT's D4.

- **Now, in parallel with T0R:**
  - **WP1** (joined readers, packager): shares only enumeration files with S3 and S4. Mechanical merge at T1's main merge.
  - **WP3 typed operations** (`operation_applier`): T0R does not touch it.
  - **WP5 and WP6** (harness, VP-STATIC): no shared files.
- **After the T0R PR merges:**
  - **WP2 types and persistence.**
    - It shares `apps/desktop/src/types.ts` and the result readers with S5.
    - It shares `src-tauri/src/lib.rs` with S3 (IPC against the rule gate and self-test).
    - `model_document_migration.rs` and the persistence files are WP2's alone.
  - **WP3 native fields.** Its resolved-state block shares `ResultsPanel.tsx` with S5.
  - **WP4 headless.** It shares `core/runner/headless/**` with S3.
  - T1 rebases these packages on the merged T0R, so no file has two concurrent writers.
- **T1 obligations** arising from T0R:
  - add `load-reference-1` and `load-reference-source-1` to the static fresh-identity set in all three languages when they are activated (SF-4);
  - keep the route→identity invariant for 0.4.0: exact-route identities only;
  - carry the historical-notice pattern if any 0.4.0 reader gains one.

## 11. What T0R completes and what remains

No group closes.

| Group | T0R completes | Remains |
|---|---|---|
| M05 | Signed actions for rigid restraints, springs, consuming constant effort and attributable nonlinear supports; admitted combinations | T5: ambiguous and nonlinear attribution, hangers. T6: frames, envelopes, report and export carriage |
| M14 | Straight circular maximum with thermal axial force; abs-sum retired on preview-physics-1 | T4: arc maximum and wall stress, transverse shear, fibre outputs. T3: all-selected source-blocks-1 summary |
| M33 | All-case headlines with identity ties and complete domain; `HIGH_DISPLACEMENT_REVIEW` on the all-case value | T6: combination maxima, mixed-modulus diagnosis (T0R only gates `mechanics`). T4: bends and pressure |
| M08 | SIF×k removed. Equal-factor intensified measure at markers and branches (S2b). Never combined. k-independence verified | T4: directional ii/io in component frames, arcs, M02 geometry. T6: intensified ranges and combinations under the owner's rule |
| M15 (part) | Nonlinear `mechanics` superposition withheld | T6: solving the combined load state |
| M11 (part) | Constant-effort Σfactors gate | T5 |
| M10 (none) | Imposed displacement shown to be refused, not dropped; targeted `IMPOSED_DISPLACEMENT_REQUIRES_LOAD_STATE_MODEL` replaces the raw serde error on the captured entry (S2a) | T1: 0.4.0 boundary motion |

## 12. Decisions and notes on the rulings

**Settled by ROOT_RULINGS:** the identity name; bends; constant-effort publication; source-blocks re-homing to T3; the kink warning; retired rule-pack IDs; the report outage.

**ROOT's rulings on revision 2** accepted R-1, R-2 (rule binding blocked, which §5.7 now enforces), R-3 to R-6, and the imposed-displacement diagnostic. The notes below are kept as the revision-2 record.

**Where I believe a ruling is imprecise or needs adjusting,** with evidence:

- **R-1 (ruling 2, the failed-case clause).** "Any ordinary or failed case" overstates the new work. A failed case already makes the receipt `partial`, and every reader admits only `aggregate == "qualified"` (`result_export/src/source_blocks.rs:1219-1235`). The only live Current exposure is selected + ordinary: an ordinary case has `outcome: qualified`. I propose deriving the standing reason in the readers (§5.7) rather than adding a producer diagnostic. That keeps source-blocks-1 bytes and the receipt code T1 is editing unchanged, covers stored envelopes, and avoids the `lib.rs` overlap region.
- **R-2 (ruling 2, all-selected).** Keeping all-selected source-blocks-1 Current with its abs-sum summary conflicts with the ruling's own standard that "no known-defective quantity stays Current". It is defensible only as an explicit exception:
  - the gate admits nodal loads only (`source_recovery.rs:460-470`), so actions are linear on each span;
  - the circular maximum is then at an endpoint;
  - the abs-sum over the sampled set, which includes both endpoints, is an upper bound at most √2 high.
  I recommend recording this as a named exception with that proof, or withholding the source-blocks-1 summary kind from rule binding until T3.
- **R-3 (ruling 7).** `pure_moment_does_not_become_force` asserts that the force norm is 0 under a pure moment (`physics_audit_regression/src/lib.rs:181-189`). That is true physics, not the M05 defect. The defect is the missing moment rows. The prescribed repair is still right: migrate to v2 rows, keep the zero-force assertion, and add Mz = −100 N·m.
- **R-4 (ruling 1).** All operands share one model, so "any operand's model has nonlinear supports" reduces to "the model has any nonlinear support record". This includes supports that are inactive in every case, per STRESS_REFERENCE C3.
- **R-5 (ruling 3).** Σfactors = 1 needs a stated representation guard. §5.6 uses the existing 64ε guard, which is not an engineering tolerance.
- **R-6 (ruling 10).** The branch measure uses the member's own Z. Code-specific branch effective moduli stay user rules; the label states this. Only one header pipe can be referenced, so a header continuation gets a coverage diagnostic rather than a guessed row. Arc ends are excluded, because their end values are not the arc's governing station.
- **R-7 (addendum).** No containment is needed for imposed displacement, because it is refused (§2.2). ROOT has since ruled the targeted diagnostic in; §2.2 defines it (SF-E).

**Owner-level:** none remain open for T0R. The SIF question was settled by OWNER_SIF_DECISION.

## 13. Convergence path

Unchanged. Once T4, T5 and T6 extend the exact profile, the exact profile becomes the authoring default and preview-physics-1 becomes historical.

## 14. Sources, probe and limits

**Additionally read for revision 2:**
- REVIEW, REFERENCE_CHECK (its return and archived scripts, read only), ROOT_RULINGS, and the two owner decision records;
- `T1_PLAN.md` at `500d88644`;
- `source_blocks.rs` (reader);
- `source_receipt/rows.rs`;
- the `operation_applier` imposed-displacement construction;
- `loads/primitive_loads`;
- the `physics_audit_regression` benchmark;
- the `src-tauri` packaged self-test.

**Executed:**
- `references.py` (standard library only);
- the design probe through the captured entry, built outside the repository against the worktree's `core/product_physics`. The build directory and scratch copy were deleted.

No product, test, fixture or other record was changed, and no Git write was run.

**Limits:**
- The `revision_2` references are mine and unchecked until S0′.
- Consumer inventories come from grep and reading; each PR lists its actual inventory.
- Line numbers drift with T1.
- I did not run the existing suites, the native app, CI or a DEC-025 sweep.

**Records in this folder:**
- `references.py`, `_run_records/references.stdout.txt`;
- `_run_records/DESIGN_revision1.md`, `_run_records/DESIGN_revision2.md`;
- `_run_records/design_probe/`, `_run_records/design_probe_output.log` (revision 1), `_run_records/design_probe_output_rev2.log`, `_run_records/toolchain.txt`;
- `_run_records/SHA256SUMS`.

The files under `REVIEW/`, `REFERENCE_CHECK/` and `ROOT_RULINGS.md` are ROOT's and were not edited.
