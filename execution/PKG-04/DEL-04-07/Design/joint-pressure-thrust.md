# T4-U5a: joint pressure thrust, untied

This governs T4-U5a, which admits pressure on an untied expansion joint authored as an objective connector (T4-U3). It runs in batch 5 with the nonlinear K-D5 unit and T4-U4a. T4-U5b (batch 7, with T4-U4b) adds elastic tie rods. This design is not an implementation claim; the code is another agent's work.

## The rulings that govern it

- **H-2** ([pressure-contract.md](pressure-contract.md)): "retain the pre-cancelled ledger and add member-owned bend `K_b·u_free(ε_p) − c_b` and joint `p(Ae−Ai)` terms. One implementation owns the assembly. T4-U5 must independently compare representation (A), including JR's tie-force double-count control."
- **The K-D5 source-closure ruling** (2026-10-11): connector q_ref stays author-supplied, and a replaced span's transferred eigenloads are refused unless their raw intended midpoint and enclosure radius are exactly zero. Both stand unchanged.
- **H6** (HELP_HUMAN, 2026-10-12): unequal bore or pressure stays refused by name.
- **O1** (owner, 2026-10-12): "Lift for tie rods only" ([SCA-012](../../../_ScopeChange/SCA-012_2026-10-12_TIE_RODS/DECISION.md)). Elastic rods are T4-U5b's; hinges and gimbals stay non-goals.
- **H7**: one re-registration per batch head. U5a's priced text changes are carried in batch 5's.
- **D7**: `prepare_sources` already declines connectors by name; a pressurized joint takes the same decline.

The physics is JR's (`CORRECTNESS_DESIGN/JOINT_REFERENCE/CONTRACT.md` §§2, 4 and 6, and its independent refutation §§2–3, archived at `11718e61a2^`). The references re-derive it (below).

## 1. What is admitted

**The pressure model.** On a `3.0.0/exact_pressure_v3` document, `objective_connector.pressure_model` admits a second kind besides `unpressurized`:

```
{ "kind": "constant_effective_area_reference_v1",
  "effective_area": Quantity<area>,                       // finite, > 0
  "pressure_basis": "internal_differential_zero_external_v1",
  "source_reference": NonemptyString }
```

The pressure basis is the region's basis, not JR's proposed `internal_increment_zero_external_v1` name, so that one vocabulary names one quantity. Ae is the authored manufacturer or user effective area, a first-order pressure-volume derivative of the joint. It has no default and is never derived from the bore or a mean bellows diameter. The pressure model is model-scoped; the pressure magnitude is always the region's.

**The region-chain edge** (JR's `JointPressureParticipation`, in its simplest explicit form). A pressure region gains an optional `member_connector_ids` list. Absent or empty means the region has no joint edges, so every existing region is unchanged. A listed connector is an edge of the region's chain between its end nodes. It is never added to `member_pipe_ids`, and its welded faces are never terminals or closures.

**The admitted configuration.** A listed connector is admitted when all of these hold; otherwise the case is refused by name (section 6):
1. its pressure model is `constant_effective_area_reference_v1`, with a valid area and source;
2. its hardware is `untied`, with the existing constant-elasticity calibration without pressure-dependent tangent effects;
3. both end nodes are interior nodes of the same region's chain. At each, the other chain edge is a straight member pipe of that region (not a realized arc, not another connector) welded to the joint face;
4. both attachment offsets are exactly zero, so each pressure-face centre is its end node;
5. the straight run through the joint (pipe, joint chord, pipe) passes the region's existing collinearity guard, with the joint chord xⱼ − xᵢ treated as a straight member. FK already requires Q·x along +r;
6. the region's equal-bore check spans both face pipes. Equal pressure follows because one region is the single pressure authority;
7. the region does not list the connector's replaced span, and no other region lists the connector.

A constant-effective-area connector in a case where no region lists it carries no pressure (p = 0 at the joint). An unpressurized connector that no region touches is admitted beside pressurized regions: the blanket refusal of any case with pressure regions is lifted.

**Touching.** A region touches a connector when one of its members is incident to, or one of its terminals is at, a connector end node. A region that touches a connector without listing it is refused, so pressure membership and closure are never inferred.

**Precedence.** The joint classifier decides every region–connector finding before region traversal. A region with a joint finding is neither traversed nor assembled, so its refusal carries exactly the joint code. Bore and collinearity findings arise in traversal, under their existing names.

**Traversal.** The chain graph's edges are the member pipes and the listed connectors. A listed connector's replaced span is not an external pipe for the internal-branch rule; it is not built (S21). Junction and tangency rules for realized arcs are unchanged, and a joint is never adjacent to an arc (condition 3).

## 2. The ledger (representation B)

The straight members keep the pre-cancelled ledger unchanged: physical caps only at transferring terminals, and the Poisson pairs. Realized arcs keep `K_b·u_free(ε_p) − c_b` and the kink remainders. Each listed joint adds, at each of its two attachments, two summands whose sum is p(Ae − Ai):

| Summand kind | At end i | At end j | Factor group |
|---|---|---|---|
| `joint_effective_area` | −p·Ae·Q·x | +p·Ae·Q·x | its own: (dof, p bits, Ae bits, \|direction component\| bits) |
| `joint_face_cap` | the face pipe's outward cap +pAi at its end at node i | the same at node j | the face pipe's existing group (dof, p, bore, \|d\|) |

Here Q·x is the connector's authored first axis. A face cap has exactly the coefficient a transferring terminal cap has at that pipe end (−sign·pAi·d, sign +1 at the pipe's node i and −1 at its node j), so it is computed from that pipe's own bore. With Q·x along +x, the joint term is −(Pe − Pi) at end i and +(Pe − Pi) at end j. JR's assembled vector for A — pipe — B — joint — C — pipe — D, caps at A and D, is [A, B, C, D] = [−Pi, Pi − Pe, Pe − Pi, Pi] plus the Poisson pairs.

**Why two summands.** The face cap is an existing `PressureOperand`, so its K-D5 re-formation and its result-relative formation bound already exist. A single p(Ae − Ai) summand would have its folded value cancel as Ae → Ai; a bound relative to that value would be unsound, and a bound relative to the operands would be a new derivation. Each summand also corresponds to a JR mutant: dropping the face caps is exactly J4's double count, and dropping the effective-area summand is a joint with Ae = 0.

**No moments.** With zero offsets and Q·x along r, JR's Bᵀg_p, g_p = [pAe, 0, 0, 0, 0, 0], is exactly the nodal pair above: Mᵢ = −(aᵢ + r/2) × F vanishes because r ∥ F. The moments that the collinearity guard admits, of order guard·pAe·L, are omitted, as the straight-run guard already omits sub-guard kinks. A lateral offset would make them finite, which is why condition 4 refuses offsets: 0.01 m gives 8 N·m in the reference fixture.

**Formation.** The effective-area group's value is p·Ae·c·d with c the exact coefficient sum (±1) and d the authored binary64 axis component. It is formed in `Scaled` arithmetic, with its S11-G formation bound counted from the source, as `exact_pressure_operand_bound` is (γ_2k plus the subnormal term). No ledger γ is assumed. The face cap uses the existing group value and bound. Thermal and q_ref loads are not pressure terms; they keep their producers.

**Evidence.** Only a region with a joint edge gains keys, so straight-only and arc regions keep their bytes (SP-1, the corpora):
- `member_connector_ids`;
- the approximations `straight_lame_annulus_and_joint_area_difference_h2_small_strain_v3`, and with arcs `straight_lame_annulus_realized_arc_and_joint_area_difference_h2_small_strain_v3`;
- `joint_edges`, one per joint: `connector_id`, `node_i_ref`, `node_j_ref`, `face_i_pipe_id`, `face_j_pipe_id`, `p_pa`, `effective_area_m2`, `face_bore_area_m2` [i, j], `joint_pressure_force_n` (pAe), `area_difference_force_n` [i, j] (pAe − pAi of each face), `attachment_pair_global_n` [i, j] and `representation` ("B: p(Ae - Ai) at the attachments; joint_effective_area + joint_face_cap").

The source-group evidence lists the new kinds. A `joint_effective_area` term carries `connector_id` in place of `pipe_id`, and its group carries `effective_area_bits` in place of the bore bits.

## 3. The K-D5 closure

- **A new formed family.** FK's `LoadFormula` gains `JointEffectiveArea { connector, end, pressure, effective_area, axis, sign }`. It re-forms p·Ae·sign·Q[axis][0] at p from binary64 inputs (p, Ae and the axis entry are all authored operands; no direction is formed), at the 2^-120 level. Its summand identity is `{region}:{connector}:joint_effective_area` with the end. The face cap is the existing `PressureOperand { Straight { frame }, p, OD, t, Constant(±1), Chord { axis } }` of the face pipe, with identity `{region}:{pipe}:joint_face_cap`.
- **The manifest.** Both kinds enter through `pressure_runtime`'s factor groups and `push_exact_pressure_operands` (family `pressure_operand`). The expected manifest comes from the producer's own group enumeration, before and independently of the supplied primitives, as for every pressure family.
- **"One ε".** A summand is omitted only when its operand is source-known exactly zero: for the effective area, an authored axis entry of exactly 0.0; for the face cap, the pipe's existing `source_zero_axes`. A formed component whose binary64 fold is zero keeps its group and its operand.
- **W2.** The same primitives at 2^b, with classification and the complete manifest retained (DEL-04-01 §2.2).
- **Not connector member loads.** The joint summands are nodal pressure operands, not `member:` primitives of the connector. The connector's `CertifiedMemberLoads` keep only its q_ref reference load, so its certified ends stay elastic (section 4).
- **The family table.** When the family lands, DEL-04-01 Design `kd5-member-relative-bound.md` §1 gains its row ("T4-U5a joint effective-area terms"), and the pressure operand row names `joint_face_cap`.

## 4. Recovery and publication

- **Elastic rows are unchanged.** g = K(q − q_ref), f = Bᵀg and the eight row families keep their recovery and certification. Pressure is never subtracted from g.
- **Two new row families**, published only for a constant-effective-area connector, so an unpressurized connector's rows are unchanged byte for byte. Both are in the connector frame (`connector_local`, `connector_axes_q`), unit N, with the connector's stable basis:
  - `connector_pressure_force_v1`, component `pe_x`: Pe = p·Ae, the joint's dead generalized pressure force in the reference geometry, conjugate to qt_x (positive tends to open the joint). It is one binary64 product of the region's p and Ae, and 0 in a case where no region lists the connector.
  - `connector_effective_axial_force_v1`, component `s_x`: S_joint = g_x + T_t − Pe, with T_t = 0 untied. It is certified from g_x's certified row minus the exact operand p·Ae, rounded once; its radius is g_x's plus that product's error. It is published exactly when g_x's row is, and otherwise unavailable with the same reason.
  - Row ids follow the scheme: `result:connector:{suffix}:connector-pressure-force:connector_local:pe_x` and `…:connector-effective-axial-force:connector_local:s_x`.
- **Adjacent pipes.** The wall force comes from the pipe law, Nw = EA(ε − ε_T) + 2νpAi, and S_pipe = Nw − Pi. Nothing is added to it, and no joint quantity is published as a wall stress. At each face, S_pipe = S_joint (JR §4); the references check this exactly.
- **Contract evidence.** The connector record of a constant-effective-area joint gains `effective_area_m2`, `pressure_basis` and `pressure_source_reference`; an unpressurized record is unchanged. The region evidence is in section 2.
- **The limitation text.** `joint::CONNECTOR_LIMITATION` must change, because "unpressurized joints only (a pressurized case with a joint is refused)" stops being true. It becomes:

  > Objective connectors (expansion joints, objective_connector 1.0.0): symmetric-midpoint, small-displacement, small-rotation linear elastic law with the authored 6x6 work matrix, frame, offsets and installed reference q_ref, replacing exactly one straight span (replaces_span); untied hardware only. A pressurized joint (constant_effective_area_reference_v1) is a coaxial edge of one equal-bore pressure region between straight members, with zero attachment offsets, and carries the dead pressure load p(Ae - Ai) at its attachments, Ae being the authored effective area; unequal bore or pressure, offset faces, joints at region terminals or beside realized arcs, and ties are refused. No finite rotation, follower or geometric stiffness, pressure stiffening, squirm or stability, tie, temperature law or stop is modelled; the replaced span carries no load, member result or self-weight. Connector actions are published as generalized and end-node elastic actions, with the joint pressure force pAe and the effective axial force published separately; they are outside the straight-member recovery coverage.

## 5. Schema, readers and authoring

- **Model input.** `PressureRegionInput` gains `member_connector_ids` (it is `deny_unknown_fields`). The desktop's `PressureRegionInput` type gains it too. For 0.5.0, the field and the pressure-model kind go to T3 with H1's corrosion-allowance and shear-model fields (open question 1).
- **`schemas/results_pressure_1.v0.3.schema.json`.**
  - `PressureConnectorRecord.pressure_model` changes from const `unpressurized` to an enum of the two kinds, with the three new properties required for the constant kind and forbidden for `unpressurized`. `hardware` stays const `untied` (T4-U5b widens it).
  - `PressureRegion` gains the two joint approximation branches, each requiring `member_connector_ids` and `joint_edges`; the existing branches forbid both.
  - The group-term and group shapes gain the `connector_id` and `effective_area_bits` forms. The pressure-1 metadata vocabulary gains components `pe_x` and `s_x`.
- **The Rust reader** (`core/reporting/result_export/src/connector_evidence.rs`, the pressure route). `RECORD_LAW` accepts the constant kind with its fields, and the row table gains the two families. Each `joint_edges` entry is bound to its connector record and rows: `pe_x` equals p·Ae from the evidence; `s_x` equals `gt_x − pe_x` under the reader's binding rule; the attachment pair is ∓(pAe − pAi) along the record's Q·x. The reader refuses by name a pressurized record without evidence, evidence without a pressurized record, an unpressurized record carrying pressure fields, and any inconsistent pair or row.
- **The Python verifier** (`tests/verifiers/pressure_1.py`, test-only, sharing no code) mirrors these checks.
- **Authoring.** The applier's connector creation and edit accept the pressure model. Its region operations accept `member_connector_ids` and refuse by the rules of section 1. The desktop's objective-connector form, offered on v3 documents, gains the pressure model, and its region editor lists connectors. JR J-B allows backend-only explicit data to land first; the unit is complete only when the live path authors it.

## 6. Refusals lifted and kept

| Today | After U5a |
|---|---|
| `JOINT_PRESSURE_INTERFACE_UNRESOLVED` for every case with pressure regions (`joint.rs` `classify_connectors`) | Lifted. The code now names a specific joint–region finding: the connector not listed by a region that touches it, a terminal at a joint face, the replaced span listed, an unpressurized connector listed, a face pipe that is not a straight member of the same region, faces in two regions (unequal pressure, H6), a duplicate or unknown entry in `member_connector_ids`, a nonzero attachment offset, a pressure basis other than the region's, an unknown pressure-model kind |
| The pressure model other than `unpressurized` → `JOINT_PRESSURE_INTERFACE_UNRESOLVED` (`joint.rs` `model_findings`) | Lifted for `constant_effective_area_reference_v1`. An absent, nonpositive, nonfinite or wrongly dimensioned area, or a missing `source_reference`, is `JOINT_PRESSURE_AREA_MISSING`. Other kinds stay refused, with the text "pressure model {kind} is not supported; only unpressurized and constant_effective_area_reference_v1 are" |
| Hardware other than `untied` → `JOINT_HARDWARE_LAW_UNSUPPORTED` | Kept; T4-U5b lifts it for explicit elastic rods only |
| Unequal bore → `PRESSURE_REGION_BORE_MISMATCH`; a straight–joint kink → `PRESSURE_REGION_NONCOLLINEAR` | Kept, by name (H6); they now span the joint |
| S20 `JOINT_REPLACED_SPAN_LOAD_UNOWNED`, the K-D5 source-closure rule, legacy and annotation refusals, `OBJECTIVE_CONNECTOR_*`, `JOINT_STIFFNESS_BASIS_UNSUPPORTED` | Kept |
| `prepare_sources`' connector decline (D7) | Kept, for pressurized joints too |

## 7. The declared blast radius

| What moves | Why |
|---|---|
| Every v3 envelope with a connector: its limitation list | `CONNECTOR_LIMITATION` text (priced TEXT, O-10; batch 5's one re-registration, H7) |
| `fixtures/results/pressure_v3_connector_reader_corpus.json` | the limitation text, and new entries with a pressurized joint (generator test only) |
| `fixtures/results/pressure_v3_arc_reader_corpus.json` | the limitation text (its connector case); the generator's doc comment in `pressure_arc_readers.rs` |
| `t4_i12/u3_reference_cases.json`, `joint_case_with_pressure_region` (`expected_round_02`, dated "until T4-U5 (J3/J4)") | Its region on P-100 no longer touches C-150, so the case is admitted. The frozen file is not edited: the implementing change records the supersession in `t4_i12/README.md` (a C-d note). HR's `a_pressurized_case_with_a_joint_is_refused_without_export_in_both_modes` (`core/runner/headless/tests/objective_connector.rs`) becomes an admission check (no joint code, results and export present). The numbers are pinned by `t4_u5a` J3-R/J3-RP, and the refusals by `t4_u5a` RV-01 to RV-15 |
| `core/product_physics/tests/connector_pp_paths.rs` "pressure model" variant | still refused under the same code, with the new text |
| `execution/PKG-03/DEL-03-06/Design/joint-replacement.md`, "as implemented" | "admitted only on unpressurized v3 documents" is restated |
| Unchanged | v2 and SP-1 twins; straight-only and arc regions' evidence; unpressurized connectors' rows and records; the straight corpus; PP's Cargo.lock and priced layouts (`MemberRecord`, `RecoveryRecord`, Preview inputs); LIMITATIONS of preview-physics |

## 8. Evidence and mutants

The references are [`validation/references/t4_u5a/`](../../../../validation/references/t4_u5a/README.md): 19 SHA-bound requests and their oracle, derived exactly over Q(π) by a generator that imports no product code. Mill tolerance and corrosion allowance are zero and every non-axial response is zero, so the values are basis- and shear-neutral. Before code relies on them, a second TASK that did not write them refutes them (plan 01 §5).

They cover:
- JR J3's untied rows 1 and 3;
- the thermal anchored relation (k + kp/2)·q = Pe − (1−2ν)·Pi − E·A·ε_T + k·q_ref (J3-1T, J3-1Q, J3-1TQ);
- ν = 0, the stiff and soft limits, a skew rotation with translation, connector and pipe reversals;
- Ae = Ai anchored and free, p = 0, a remotely supported closure;
- row 5's stiffness untied (the q that U5b's explicit rod must reproduce);
- an unpressurized joint beside a remote region, with and without the constant model.

**H-2's condition.**
- *Representation (A).* For every case the generator assembles JR's representation (A) independently: per-member full cap and Poisson pairs, the joint's Bᵀg_p from JR's B, and remote-closure cancellations. It asserts (A) equals (B) exactly and solves to the same state. The product, which implements (B), is compared with (A)'s published pressure RHS (`assembly_evidence.assembled_pressure_rhs_global`) and with (A)'s solved state.
- *J4 in its untied form.* The double count reproduces JR's J4 values (anchored, q = (h + Pe)/(k + kp/2); free, q = (Pe + Pi)/k). In the free chain it leaves the reactions and Nw unchanged; only the joint's elastic force (Pe + Pi instead of Pe) and the interface identity S_pipe = S_joint expose it. The tests must inspect those, not global balance.

**Required checks** (both solver modes, criterion as in the reference README):
1. **PP system:** every `t4_u5a` case through PP's public entry: the connector rows (eight elastic and two new), member Nw and S, node displacements and rotations, support-on-model reactions, the region's `joint_edges` and the pressure RHS against (A). The interface identity is checked on published rows (Nw − Pi of each face pipe against `s_x`).
2. **Refusals:** RV-01 to RV-15 with their exact code sets, `MODEL_INCOMPLETE` and no results; a unit test for a joint beside a realized arc.
3. **HR:** a representative subset through the headless runner in both modes, with export, including J3-1R and J3-3.
4. **K-D5 closure** (ordinary and W2, both modes; a named load-closure detail on each mutant, and the unmutated system Passed with the member bound):
   - drop-one-of-many of a `joint_face_cap` at an attachment where the face pipe's Poisson summand shares its group;
   - drop of the single `joint_effective_area` summand;
   - duplicate and extra summands;
   - wrong-connector, wrong-end and wrong-pipe bindings (a face cap bound to the far face pipe);
   - wrong operands (Ae, p, the axis entry);
   - FK's family test against an exact or 256-bit reference.
5. **Producer mutants**, each shown killed by the reference tests: applied to the producer, run, reverted, and recorded in the PR; no committed `#[cfg(test)]` seam, per R-2's spirit. The mutants:
   - the double count (face caps dropped);
   - faces exchanged (the sign at i/j);
   - face caps formed from the outside diameter (Ae measured against the wrong bore);
   - effective area dropped;
   - the region-chain edge missing (the connector not traversed: it must refuse, not drop the thrust);
   - pressure subtracted from g;
   - Pi subtracted in wall recovery (free case: Nw = 0 instead of Pi).

   The oracle gives the wrong values the first four produce.
6. **Readers:** RE and the Python verifier accept the new corpus entries and refuse each tampered form of section 5.
7. **M3 and K-D5:** the realistic set and RV30 show no new breaches; the joint cases are Passed with the member bound.
8. **Review:** independent scrutiny of the ledger, closure and publication (numerical meaning), and a T3-interface review where the unit touches formation, the ledger, the readers or the registered profiles (plan 01 §5).

## 9. What T4-U5b adds (batch 7, under O1)

- **Hardware.** `explicit_axial_rods_v1` with the `bilateral_linear` law only. Each rod has an id, attachments bᵢ and bⱼ with rigid offsets from its end nodes, a stiffness, a reference extension and a source. Its extension e = n·(v_bⱼ − v_bᵢ) = C·d has the correct infinitesimal rigid modes; its tension is T_r = k_r(e − e_ref), assembled as Cᵀk_rC with the reference load +Cᵀk_r·e_ref. `ideal_inextensible`, the collective axial constraint (they need an MPC, a later unit only on the owner's choice), tension-only or gapped rods, nuts and stops stay refused by name. No penalty stiffness.
- **Publication.** A rod force row per rod. S_joint becomes Tb + ΣT_r − Pe. Rods are not connector member loads either: the connector's g stays elastic.
- **References** (re-derived independently):
  - JR's ideal-tie rows 2 and 4 as the finite-rod limit k_r → ∞, with exact finite-k_r forms and their limits;
  - row 5's split (Tb = 2e5·q, Tt = 2e7·q at J3-5U's q);
  - offset rod pairs at ±a (K_rz = 2k_r·a², with cancelling cross terms);
  - JR's tied J4 control on the rod force: 800 + 75π instead of 800 − 50π anchored, Pe + Pi instead of Pe free, with q, Nw and the reactions unchanged in the anchored tied case.

## Open questions

1. **The model field and DEC-033.** `member_connector_ids` and the new pressure-model kind change what a 0.3.0/0.4.0 document can contain. DEC-033 would bump the minor version for that; the 2026-10-10 owner decision makes 0.5.0 the only current format from B7. T4-U3 added `objective_connector` to v3 documents without a bump, under H-1's "grows family by family". This design follows that precedent for batch 5 and sends the two additions to T3 for 0.5.0, with H1's fields. Whether the precedent covers a region field is for HELP_HUMAN to confirm.
2. **The qualification package.** The `t4_u5a` cases are consumed by PP and HR tests. Adding them to VP-STATIC's `exact_pressure` package would need its own admission review; that is the manager's choice.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
