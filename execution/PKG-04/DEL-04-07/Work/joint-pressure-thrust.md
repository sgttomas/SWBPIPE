# T4-U5a: joint pressure thrust, untied

This governs T4-U5a, which admits pressure on an untied expansion joint authored as an objective connector (T4-U3). It runs in batch 5 with the nonlinear K-D5 unit and T4-U4a. T4-U5b (batch 7, with T4-U4b) adds elastic tie rods; its implementation design is [u4b-u5b-implementation.md](u4b-u5b-implementation.md) §3. This design is not an implementation claim; the code is another agent's work.

## Status

**Active since B7b (2026-10-17).** B7b (#1387, merge `64c22d5510`) defined both items in model 0.5.0's contract-present form (§5) and refused 0.3.0/0.4.0 by name, so the engine is reachable from documents. The nineteen `t4_u5a` requests, re-authored to 0.5.0 with an explicit zero corrosion allowance and Euler–Bernoulli selection, and the seventeen refusal variants run through the typed, captured and headless entries in both modes, and each admitted case exports (`core/runner/headless/tests/untied_joint_pressure_references.rs`). The applier accepts the constant-effective-area model and `member_connector_ids` on 0.5.0 exact-contract documents only, and the desktop's joint pressure-model editor and region connector picker author them. Of the remaining-work list below, items 1 to 4 (§8's checks 1 to 3) are carried out; open question 2 (adding the `t4_u5a` cases to VP-STATIC) remains the manager's choice. The 0.3.0/0.4.0 branches and the "until U5a lands" clauses below are the staging record.

**Since T4-U4b/U5b.** Model 0.6.0 is current and 0.5.0 is retired by name; U5a's two items carry into 0.6.0 unchanged, and the requests are re-authored in memory to 0.6.0 ([u4b-u5b-implementation.md](u4b-u5b-implementation.md) §1). The exact identity is `pressure-2`, its schema `results_pressure_2.v0.3.schema.json` and its verifier `tests/verifiers/pressure_2.py`. Condition 2 of §1 and §5's hardware line admit explicit elastic tie rods, `JOINT_HARDWARE_LAW_UNSUPPORTED` no longer refuses `explicit_axial_rods_v1`, RV-11 is superseded by `JOINT_TIE_ROD_INPUT_INVALID`, and `CONNECTOR_LIMITATION` has T4-U5b's text (u4b-u5b-implementation.md §§3.1 and 3.4). §9's open points are decided there. Where this file says 0.5.0, pressure-1 or `results_pressure_1`, it records U5a as built.

The owner resumed T4-U5a on 2026-10-13. Batch 5 carries its engine, result readers and authoring validation while document admission remains inactive until B7b supplies the exact-contract 0.5.0 form. The existing 0.3.0/0.4.0 shapes and joint-with-pressure refusals remain in force during that staging.

**Designed.** The admitted configuration, the region-chain edge and the classifier's precedence (§1); representation B's two summands (§2); the K-D5 closure (§3); recovery and the two new row families (§4); schema, readers and authoring, with the field specification of the two model items (§5); refusals (§6); the blast radius (§7); evidence and mutants (§8); and what T4-U5b adds (§9). The rulings it applies are recorded at `e0b83676af` (O1–O4, H1, H4, H5; SCA-012 lifts PRD non-goal 19 for tie rods).

**Frozen and independently verified.** The references in `validation/references/t4_u5a/` (frozen at `1bd57e429e`: 19 SHA-bound requests, their oracle and 17 refusal variants) were refuted by a separate TASK that did not write them and shares no code with their generator. Its method: the force method on the axial free bodies, with its own derivation, plus an independent nodal stiffness solve with its own equivalent pressure loads, both at 70 digits. Result: every expected value CONFIRMED (2127 checks, worst 3.8e-30 relative); representations A and B identical; the sign conventions confirmed; the four ledger mutants all killed, the smallest separation about 1e6 tolerances.

**Implemented, awaiting public activation.** The dormant implementation is sealed at `5979187253eea2068ac4257c81eba0216c54cbf6`. The public solve retains its admission and preparation checks before the private prepared-case implementation; its participating-joint reader control comes from a test-constructed classified supplier input, not public 0.5.0 admission. Focused mechanics, formation, closure, reader and authoring checks pass, including the seven restored producer mutations. Independent scrutiny and batch qualification remain part of integration. The full nineteen-case public reference path, headless export and live authoring path are still B7b-dependent duties below, not completed by these dormant controls.

**DEC-033 (HELP_HUMAN ruling, 2026-10-12).** The T4-U3 precedent does not extend to new content in 0.3.0/0.4.0. DEC-033's owner ruling of 2026-06-12 ("Additive bumps too") applies: any change to what a document can contain, including a new optional member or a new kind value, bumps the minor version. Consequences, applied in the body below:
- Both U5a document items, the connector pressure-model kind `constant_effective_area_reference_v1` and the region field `member_connector_ids`, are defined only in model 0.5.0 (T3's B7). Their field specification is in §5.
- The 0.3.0/0.4.0 shape does not change. On those versions both items stay refused exactly as today.
- U5a's engine may land in batch 5 with the items absent from 0.3.0/0.4.0, or stay unactivated until the B7 batch. 0.5.0's contract-present form arrives with B7b, so in practice the engine becomes reachable from documents when B7b lands. If batch 5 lands first, the engine lands unactivated.
- T4-U3 did change what a 0.3.0/0.4.0 document can contain (§5, "T4-U3 and the 0.3.0/0.4.0 shape"). That is a historical inconsistency, which B7b's retirement of 0.3.0/0.4.0 resolves. It is not a precedent and is not to be repeated.

**The requests must be re-expressed in 0.5.0.** 18 of the 19 requests (all but J3-R) carry the 0.5.0-only items inside a model 0.3.0 document, so as frozen they are not valid documents of any version. Before a product test consumes them, the generator's builder re-expresses all 19 in model 0.5.0's contract-present form, with H1's `section.corrosion_allowance` = 0 m and `pressure_contract.shear_model` = `euler_bernoulli`, through an intended `--write`. Only the request bytes and each case's `request_sha256` change. The oracle values and the refusal code sets stay as they are, and the refusal patches' paths are re-checked against the new documents. The verify mode and an independent value check then run again. J3-R is a valid 0.3.0 document today and moves with the set.

**Minor findings of the refutation and where they are handled.** None blocks.
- Fixed in this Design: §8's J4 wording (what the free-chain double count moves and does not move); the refusal ids (17 variants, RV-01 to RV-15 with RV-05a–c and RV-06b; there is no RV-10); J3-R's description (its only load is pipe X's net Lamé elongation); the envelope field of the pressure RHS; the definition of the faces-exchanged (`sign_flip`) mutant; and the precedence sentence (a listed connector refused by a model finding carries only that code).
- Fixed in the references' README, as text only: a key-to-floor-family map, including areas and unitless fields, and the same id, J3-R, `sign_flip` and 0.5.0 notes. No request, oracle value or generator byte changed; only the README line of `SHA256SUMS` is regenerated.
- Named as required unit tests in §8, because the oracle has no patch for them and adding patches would change the oracle: a duplicate `member_connector_ids` entry; a connector listed by two regions; a negative, nonfinite or wrongly dimensioned area; a face pipe that is not a straight member, beside the realized-arc case already named.

**Remaining work for public activation and integration.**
1. T3 defines the two items in 0.5.0 from §5's field specification, with H1's fields (B7b's contract-present form).
2. The requests are re-expressed in 0.5.0 (above).
3. Integrate and qualify the dormant implementation with U4a in batch 5; activate its document and live authoring path only with B7b.
4. The §8 evidence. Checks 1–3 need 0.5.0 documents, so they run once B7b's form exists.
5. Open question 2 (VP-STATIC). The scope question on the blanket-refusal lift is resolved in §1.

**Dependencies.**
- T3, DEL-05-02 B7: B7b defines 0.5.0's contract-present form, refuses 0.3.0/0.4.0 and re-authors their fixtures. B7a's pressure-free 0.5.0 does not carry these items.
- DEL-04-01 (FK): `LoadFormula::JointEffectiveArea` and its family row (§3).
- Batch 5 re-registers once (H7). T4-U3's connector is the base; T4-U5b (batch 7) follows.

## The rulings that govern it

- **H-2** ([pressure-contract.md](pressure-contract.md)): "retain the pre-cancelled ledger and add member-owned bend `K_b·u_free(ε_p) − c_b` and joint `p(Ae−Ai)` terms. One implementation owns the assembly. T4-U5 must independently compare representation (A), including JR's tie-force double-count control."
- **The K-D5 source-closure ruling** (2026-10-11): connector q_ref stays author-supplied, and a replaced span's transferred eigenloads are refused unless their raw intended midpoint and enclosure radius are exactly zero. Both stand unchanged.
- **H6** (HELP_HUMAN, 2026-10-12): unequal bore or pressure stays refused by name.
- **O1** (owner, 2026-10-12): "Lift for tie rods only" ([SCA-012](../../../_ScopeChange/SCA-012_2026-10-12_TIE_RODS/DECISION.md)). Elastic rods are T4-U5b's; hinges and gimbals stay non-goals.
- **H7**: one re-registration per batch head. U5a's priced text changes are carried in batch 5's.
- **D7**: `prepare_sources` already declines connectors by name; a pressurized joint takes the same decline.
- **DEC-033** (HELP_HUMAN, 2026-10-12, applying the owner's 2026-06-12 ruling "Additive bumps too"): U5a's document items are defined only in model 0.5.0. The 0.3.0/0.4.0 shape does not change (Status).

The physics is JR's (`CORRECTNESS_DESIGN/JOINT_REFERENCE/CONTRACT.md` §§2, 4 and 6, and its independent refutation §§2–3, archived at `11718e61a2^`). The references re-derive it (below).

## 1. What is admitted

**The pressure model.** On a model 0.5.0 document whose `pressure_contract` is `3.0.0/exact_pressure_v3` (B7b's contract-present form; DEC-033), `objective_connector.pressure_model` admits a second kind besides `unpressurized`. 0.3.0/0.4.0 documents keep `unpressurized` only. §5 gives the field specification.

```
{ "kind": "constant_effective_area_reference_v1",
  "effective_area": Quantity<area>,                       // finite, > 0
  "pressure_basis": "internal_differential_zero_external_v1",
  "source_reference": NonemptyString }
```

The pressure basis is the region's basis, not JR's proposed `internal_increment_zero_external_v1` name, so that one vocabulary names one quantity. Ae is the authored manufacturer or user effective area, a first-order pressure-volume derivative of the joint. It has no default and is never derived from the bore or a mean bellows diameter. The pressure model is model-scoped; the pressure magnitude is always the region's.

**The region-chain edge** (JR's `JointPressureParticipation`, in its simplest explicit form). A pressure region of a 0.5.0 document has an optional `member_connector_ids` list; 0.3.0/0.4.0 regions do not gain it (DEC-033). Absent or empty means the region has no joint edges, so every existing region is unchanged. A listed connector is an edge of the region's chain between its end nodes. It is never added to `member_pipe_ids`, and its welded faces are never terminals or closures.

**The admitted configuration.** A listed connector is admitted when all of these hold; otherwise the case is refused by name (section 6):
1. its pressure model is `constant_effective_area_reference_v1`, with a valid area and source;
2. its hardware is `untied`, with the existing constant-elasticity calibration without pressure-dependent tangent effects; since T4-U5b, also `explicit_axial_rods_v1` with valid rods (u4b-u5b-implementation.md §3.1);
3. both end nodes are interior nodes of the same region's chain. At each, the other chain edge is a straight member pipe of that region (not a realized arc, not another connector) welded to the joint face;
4. both attachment offsets are exactly zero, so each pressure-face centre is its end node;
5. the straight run through the joint (pipe, joint chord, pipe) passes the region's existing collinearity guard, with the joint chord xⱼ − xᵢ treated as a straight member. FK already requires Q·x along +r;
6. the region's equal-bore check spans both face pipes. Equal pressure follows because one region is the single pressure authority;
7. the region does not list the connector's replaced span, and no other region lists the connector.

A constant-effective-area connector in a case where no region lists it carries no pressure (p = 0 at the joint). On the 0.5.0 path, an unpressurized connector that no region touches is admitted beside pressurized regions: the blanket refusal of any case with pressure regions is lifted. **T4 manager technical ruling, 2026-10-13:** the lift applies only on 0.5.0. The 0.3.0/0.4.0 path retains its existing blanket refusal until B7b retires those formats. J3-R is a valid 0.3.0 document today, but its admission supersession is tested after re-expression in 0.5.0. The new admission belongs to the same current-format pressure contract as the joint-edge rules; adding a temporary admission on formats immediately due for retirement would introduce a second behavior boundary with no continuing consumer.

**Touching.** A region touches a connector when one of its members is incident to, or one of its terminals is at, a connector end node. A region that touches a connector without listing it is refused, so pressure membership and closure are never inferred.

**Precedence.** The joint classifier decides every region–connector finding before region traversal. A region with a joint finding is neither traversed nor assembled, so its refusal carries exactly the joint code. Bore and collinearity findings arise in traversal, under their existing names. A connector's own model findings (`model_findings`) come first. A listed connector refused by a model finding (for example `JOINT_PRESSURE_AREA_MISSING` or `JOINT_HARDWARE_LAW_UNSUPPORTED`) carries only that code. It is not also classified against the region, so it does not also take `JOINT_PRESSURE_INTERFACE_UNRESOLVED` (RV-05a–c, RV-11).

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
  - `connector_effective_axial_force_v1`, component `s_x`: S_joint = g_x + T_t − Pe, with T_t = 0 untied (with T4-U5b's rods, T_t = Σ T_r·(n_r·Q_x); §9). It is certified from g_x's certified row minus the exact operand p·Ae, rounded once; its radius is g_x's plus that product's error. It is published exactly when g_x's row is, and otherwise unavailable with the same reason.
  - Row ids follow the scheme: `result:connector:{suffix}:connector-pressure-force:connector_local:pe_x` and `…:connector-effective-axial-force:connector_local:s_x`.
- **Adjacent pipes.** The wall force comes from the pipe law, Nw = EA(ε − ε_T) + 2νpAi, and S_pipe = Nw − Pi. Nothing is added to it, and no joint quantity is published as a wall stress. At each face, S_pipe = S_joint (JR §4); the references check this exactly.
- **Contract evidence.** The connector record of a constant-effective-area joint gains `effective_area_m2`, `pressure_basis` and `pressure_source_reference`; an unpressurized record is unchanged. The region evidence is in section 2.
- **The limitation text.** `joint::CONNECTOR_LIMITATION` must change, because "unpressurized joints only (a pressurized case with a joint is refused)" stops being true. It becomes:

  > Objective connectors (expansion joints, objective_connector 1.0.0): symmetric-midpoint, small-displacement, small-rotation linear elastic law with the authored 6x6 work matrix, frame, offsets and installed reference q_ref, replacing exactly one straight span (replaces_span); untied hardware only. A pressurized joint (constant_effective_area_reference_v1) is a coaxial edge of one equal-bore pressure region between straight members, with zero attachment offsets, and carries the dead pressure load p(Ae - Ai) at its attachments, Ae being the authored effective area; unequal bore or pressure, offset faces, joints at region terminals or beside realized arcs, and ties are refused. No finite rotation, follower or geometric stiffness, pressure stiffening, squirm or stability, tie, temperature law or stop is modelled; the replaced span carries no load, member result or self-weight. Connector actions are published as generalized and end-node elastic actions, with the joint pressure force pAe and the effective axial force published separately; they are outside the straight-member recovery coverage.

## 5. Schema, readers and authoring

- **Model input (DEC-033).** The two items are defined only in model 0.5.0's contract-present form. T3's B7 defines that form (B7b), with H1's corrosion-allowance and shear-model fields, from the field specification below. On 0.3.0/0.4.0 nothing changes. `PressureRegionInput` is `deny_unknown_fields`, so `member_connector_ids` remains an unknown member there and the request is refused as today. The connector's `constant_effective_area_reference_v1` kind keeps today's model finding there. `PressureRegionInput` and the desktop's `PressureRegionInput` type gain the field for 0.5.0 documents only. Because B7b also refuses 0.3.0/0.4.0 by name, U5a needs no version-keyed branch once B7b has landed. Before then, the engine lands unactivated.
- **`schemas/results_pressure_1.v0.3.schema.json`.**
  - `PressureConnectorRecord.pressure_model` changes from const `unpressurized` to an enum of the two kinds, with the three new properties required for the constant kind and forbidden for `unpressurized`. `hardware` stays const `untied` (T4-U5b widens it).
  - `PressureRegion` gains the two joint approximation branches, each requiring `member_connector_ids` and `joint_edges`; the existing branches forbid both.
  - The group-term and group shapes gain the `connector_id` and `effective_area_bits` forms. The pressure-1 metadata vocabulary gains components `pe_x` and `s_x`.
- **The Rust reader** (`core/reporting/result_export/src/connector_evidence.rs`, the pressure route). `RECORD_LAW` accepts the constant kind with its fields, and the row table gains the two families. Each `joint_edges` entry is bound to its connector record and rows: `pe_x` equals p·Ae from the evidence; `s_x` equals `gt_x − pe_x` under the reader's binding rule; the attachment pair is ∓(pAe − pAi) along the record's Q·x. The reader refuses by name a pressurized record without evidence, evidence without a pressurized record, an unpressurized record carrying pressure fields, and any inconsistent pair or row.
- **The Python verifier** (`tests/verifiers/pressure_1.py`, test-only, sharing no code) mirrors these checks.
- **Authoring.** On 0.5.0 documents only, the applier's connector creation and edit accept the constant-effective-area pressure model, and its region operations accept `member_connector_ids` and refuse by the rules of section 1. On 0.3.0/0.4.0 the applier writes neither item (DEC-033). The desktop's objective-connector form, offered on v3 documents, gains the pressure model, and its region editor lists connectors, both on 0.5.0. JR J-B allows backend-only explicit data to land first; the unit is complete only when the live path authors it.

### Field specification (model 0.5.0)

Every refusal below gives `MODEL_INCOMPLETE` and no results. RV ids name the `t4_u5a` refusal variants; "unit test" marks a refusal that §8 check 2 requires and that has no oracle patch.

**1. Pressure-model kind `constant_effective_area_reference_v1`**

- **Name.** `pressure_model` of kind `constant_effective_area_reference_v1`, with members `kind`, `effective_area`, `pressure_basis` and `source_reference`.
- **Location.** A model 0.5.0 document whose `pressure_contract` is `3.0.0/exact_pressure_v3`, at `components[].objective_connector.pressure_model`, with `objective_connector.version` "1.0.0".
- **Type.** A closed object (no other members):
  - `kind`: string, the constant `"constant_effective_area_reference_v1"`;
  - `effective_area`: Quantity, `{ "value": number, "unit": string }`;
  - `pressure_basis`: string, the constant `"internal_differential_zero_external_v1"`;
  - `source_reference`: string.
- **Units.** `effective_area` has the dimension of area (`"m^2"` in the references). The other members are unitless.
- **Value constraints.**
  - `effective_area.value` is finite and > 0, and its unit is an area.
  - `source_reference` is non-empty.
  - `pressure_basis` equals the `pressure_basis` of the region that lists the connector.
  - The connector's `hardware` is `untied`. T4-U5b widened this to `untied` or `explicit_axial_rods_v1` with valid rods (u4b-u5b-implementation.md §3.1).
- **Required when.** Every member is required when the kind is this one. The kind is required for a connector listed in a region's `member_connector_ids`. Otherwise it is optional: a constant-effective-area connector that no region lists is admitted and carries no pressure (p = 0 at the joint, Pe = 0; J3-RP).
- **Named refusals.**
  - *Absent or invalid area or source.* `effective_area` absent, zero, negative, nonfinite or not an area, or `source_reference` absent or empty → `JOINT_PRESSURE_AREA_MISSING` (RV-05a absent, RV-05b zero, RV-05c no source; negative, nonfinite and wrong dimension are unit tests).
  - *Basis.* `pressure_basis` absent or different from the region's → `JOINT_PRESSURE_INTERFACE_UNRESOLVED` (RV-13).
  - *Other kinds.* Any kind other than `unpressurized` and this one → `JOINT_PRESSURE_INTERFACE_UNRESOLVED`, with the text "pressure model {kind} is not supported; only unpressurized and constant_effective_area_reference_v1 are" (RV-12).
  - *Present without the conditions.* Hardware other than `untied` → `JOINT_HARDWARE_LAW_UNSUPPORTED` (RV-11). Since T4-U5b, hardware other than the two defined kinds, or a rod law other than `bilateral_linear`, takes that code, and RV-11's empty rod list takes `JOINT_TIE_ROD_INPUT_INVALID`. A listed connector whose configuration fails §1 takes §6's codes. An offset joint is RV-06 or RV-06b.
  - *Precedence.* These are model findings, so a listed connector refused by one carries only that code (§1).
  - *On 0.3.0/0.4.0 (not defined there).* Today's model finding stays: `JOINT_PRESSURE_INTERFACE_UNRESOLVED`, "objective connector {id}: only the unpressurized pressure model is supported until T4-U5" (`joint.rs` `model_findings`). The document shape is unchanged.
  - *Until U5a lands.* If 0.5.0 defines the item before U5a's engine is active, the same model finding refuses it. It is never ignored and never solved as unpressurized.
- **Meaning.** Ae is the authored manufacturer or user effective area of the joint, a first-order pressure-volume derivative. It has no default and is never derived from the bore or a bellows diameter. With the region's p it gives Pe = p·Ae, the joint's dead generalized pressure force in the reference geometry, conjugate to qt_x. Pe is assembled as p(Ae − Ai) at the attachments (§2) and published as `pe_x` and `s_x` (§4). The pressure model is model-scoped; the pressure magnitude and basis are always the region's.

**2. Region field `member_connector_ids`**

- **Name.** `member_connector_ids`.
- **Location.** A model 0.5.0 document with the exact contract present, at `load_cases[].pressure_regions[].member_connector_ids`.
- **Type.** An array of strings, each a component `id`.
- **Units.** None.
- **Value constraints.**
  - Each entry is the `id` of a component in `components` that carries an `objective_connector`.
  - Entries are unique within the list.
  - Within one load case, at most one region lists a given connector.
  - Absent and `[]` mean the same: the region has no joint edges.
  - A listed connector satisfies §1's conditions 1–7.
- **Required when.** Optional. A region that touches a connector must list it. A region touches a connector when one of its members is incident to, or one of its terminals is at, a connector end node.
- **Named refusals** (`JOINT_PRESSURE_INTERFACE_UNRESOLVED` unless stated):
  - the region touches a connector without listing it (RV-01);
  - an entry that is not an objective connector of the model (RV-15);
  - a duplicate entry (unit test);
  - a connector listed by two regions (unit test);
  - the joint's faces in two regions, so at different pressures (H6; RV-09);
  - an unpressurized connector listed (RV-04);
  - a region terminal at a joint face (RV-02);
  - the replaced span listed in `member_pipe_ids` (RV-03);
  - a nonzero attachment offset (RV-06 lateral, RV-06b axial);
  - a face pipe that is not a straight member of the same region, such as a realized arc, another connector or a non-member (unit tests);
  - in traversal, under their existing names: unequal bore → `PRESSURE_REGION_BORE_MISMATCH` (RV-08), and a straight–joint kink → `PRESSURE_REGION_NONCOLLINEAR` (RV-07);
  - *on 0.3.0/0.4.0 (not defined there):* the member is unknown to `PressureRegionInput` (`deny_unknown_fields`), and the request is refused as today, unchanged;
  - *until U5a lands:* if 0.5.0 defines the field before U5a's engine is active, a non-empty list is refused with `JOINT_PRESSURE_INTERFACE_UNRESOLVED` on the region and the listed connectors. It is never ignored. An absent or empty list leaves the region as it is today.
- **Meaning.** The region's joint edges, JR's `JointPressureParticipation` in explicit form. A listed connector is an edge of the region's chain between its end nodes. It is never a member pipe, its welded faces are never terminals or closures, and the region's p and basis apply to it. Pressure membership and closure are never inferred from geometry.

### T4-U3 and the 0.3.0/0.4.0 shape (historical record)

Compared across T4-U3's merge to main (PR #1234, merge `71624868cb`, against its first parent):
- `schemas/model.schema.yaml` did not change. It was last changed by `c394365ca7` (2026-08-03), and it has one shape for every version and no `objective_connector`.
- The desktop's `model_document_migration.rs` did not change either.
- The optional member `components[].objective_connector` was already readable on 0.3.0/0.4.0 before T4-U3. Since `22452ecd14` (2026-09-25) it was an opaque value (`PreviewComponent.objective_connector`), always refused by name: `OBJECTIVE_CONNECTOR_NOT_IMPLEMENTED` on 0.3.0/0.4.0, `OBJECTIVE_CONNECTOR_VERSION_UNSUPPORTED` for a version other than 1.0.0, and `PREVIEW_CONTRACT_VERSION_MISMATCH` on other model versions.

T4-U3 added no member key. It did define and admit the content of `objective_connector` 1.0.0 (frames, attachments, work matrix, `replaces_span`, `hardware` and `pressure_model`) on 0.3.0/0.4.0 v3 documents. Its operation applier also began writing that member into 0.3.0/0.4.0 documents (`d5b53a15eb`). None of this bumped a version. Under DEC-033 ("Additive bumps too") that is a change to what a 0.3.0/0.4.0 document can contain. It is recorded here as a historical inconsistency. B7b resolves it by refusing 0.3.0/0.4.0 by name and re-authoring their fixtures in 0.5.0. HELP_HUMAN ruled on 2026-10-12 that it is not a precedent for new content, and it is not to be repeated.

## 6. Refusals lifted and kept

| Today | After U5a |
|---|---|
| `JOINT_PRESSURE_INTERFACE_UNRESOLVED` for every case with pressure regions (`joint.rs` `classify_connectors`) | Lifted only on 0.5.0 (§1); the blanket refusal stays on 0.3.0/0.4.0 until their retirement. On 0.5.0 the code names a specific joint–region finding: the connector not listed by a region that touches it, a terminal at a joint face, the replaced span listed, an unpressurized connector listed, a face pipe that is not a straight member of the same region, faces in two regions (unequal pressure, H6), a duplicate or unknown entry in `member_connector_ids`, a nonzero attachment offset, a pressure basis other than the region's, an unknown pressure-model kind |
| The pressure model other than `unpressurized` → `JOINT_PRESSURE_INTERFACE_UNRESOLVED` (`joint.rs` `model_findings`) | Lifted for `constant_effective_area_reference_v1` on 0.5.0 only; on 0.3.0/0.4.0 kept, code and text unchanged (DEC-033). An absent, nonpositive, nonfinite or wrongly dimensioned area, or a missing `source_reference`, is `JOINT_PRESSURE_AREA_MISSING`. Other kinds stay refused, with the text "pressure model {kind} is not supported; only unpressurized and constant_effective_area_reference_v1 are" |
| Hardware other than `untied` → `JOINT_HARDWARE_LAW_UNSUPPORTED` | Kept; T4-U5b lifted it for explicit elastic rods only (u4b-u5b-implementation.md §3.1) |
| Unequal bore → `PRESSURE_REGION_BORE_MISMATCH`; a straight–joint kink → `PRESSURE_REGION_NONCOLLINEAR` | Kept, by name (H6); they now span the joint |
| S20 `JOINT_REPLACED_SPAN_LOAD_UNOWNED`, the K-D5 source-closure rule, legacy and annotation refusals, `OBJECTIVE_CONNECTOR_*`, `JOINT_STIFFNESS_BASIS_UNSUPPORTED` | Kept |
| `prepare_sources`' connector decline (D7) | Kept, for pressurized joints too |

## 7. The declared blast radius

| What moves | Why |
|---|---|
| Every v3 envelope with a connector: its limitation list | `CONNECTOR_LIMITATION` text (priced TEXT, O-10; batch 5's one re-registration, H7) |
| `fixtures/results/pressure_v3_connector_reader_corpus.json` | the limitation text, and new entries with a pressurized joint (generator test only) |
| `fixtures/results/pressure_v3_arc_reader_corpus.json` | the limitation text (its connector case); the generator's doc comment in `pressure_arc_readers.rs` |
| `t4_i12/u3_reference_cases.json`, `joint_case_with_pressure_region` (`expected_round_02`, dated "until T4-U5 (J3/J4)") | Its region on P-100 does not touch C-150, so the case is admitted after B7b re-authors it in 0.5.0 (§1). The frozen file is not edited: the implementing change records the supersession in `t4_i12/README.md` (a C-d note). HR's `a_pressurized_case_with_a_joint_is_refused_without_export_in_both_modes` (`core/runner/headless/tests/objective_connector.rs`) then becomes an admission check (no joint code, results and export present). The numbers are pinned by `t4_u5a` J3-R/J3-RP, and the refusals by `t4_u5a`'s 17 refusal variants (RV-01 to RV-15 with RV-05a–c and RV-06b; there is no RV-10). Its original 0.3.0/0.4.0 document continues to be refused until B7b's format retirement |
| `core/product_physics/tests/connector_pp_paths.rs` "pressure model" variant | still refused under the same code, with the new text |
| `execution/PKG-03/DEL-03-06/Work/joint-replacement.md`, "as implemented" | "admitted only on unpressurized v3 documents" is restated |
| Unchanged | v2 and SP-1 twins; straight-only and arc regions' evidence; unpressurized connectors' rows and records; the straight corpus; PP's Cargo.lock and priced layouts (`MemberRecord`, `RecoveryRecord`, Preview inputs); LIMITATIONS of preview-physics |

## 8. Evidence and mutants

The references are [`validation/references/t4_u5a/`](../../../../validation/references/t4_u5a/README.md): 19 SHA-bound requests and their oracle, derived exactly over Q(π) by a generator that imports no product code. Mill tolerance and corrosion allowance are zero and every non-axial response is zero, so the values are basis- and shear-neutral. A second TASK that did not write them has refuted them and confirmed every value (Status). The requests are frozen as model 0.3.0 documents and are re-expressed in 0.5.0 before any product test consumes them (Status; DEC-033).

They cover:
- JR J3's untied rows 1 and 3;
- the thermal anchored relation (k + kp/2)·q = Pe − (1−2ν)·Pi − E·A·ε_T + k·q_ref (J3-1T, J3-1Q, J3-1TQ);
- ν = 0, the stiff and soft limits, a skew rotation with translation, connector and pipe reversals;
- Ae = Ai anchored and free, p = 0, a remotely supported closure;
- row 5's stiffness untied (the q that U5b's explicit rod must reproduce);
- an unpressurized joint beside a remote region, with and without the constant model (J3-R, J3-RP). The joint carries no pressure there. Its only load is pipe X's net Lamé elongation, (1 − 2ν)·Pi/kp, carried through the anchored chain. The oracle's frozen description, "the joint carries the remote cap's share", is imprecise on this point; the values are right.

**H-2's condition.**
- *Representation (A).* For every case the generator assembles JR's representation (A) independently: per-member full cap and Poisson pairs, the joint's Bᵀg_p from JR's B, and remote-closure cancellations. It asserts (A) equals (B) exactly and solves to the same state. The product, which implements (B), is compared with (A)'s pressure RHS. The product's field is the envelope's `contract_evidence.exact_cases[].pressure_rhs_assembly` (its `assembled_pressure_rhs_global`). The product is also compared with (A)'s solved state.
- *J4 in its untied form.* The double count reproduces JR's J4 values (anchored, q = (h + Pe)/(k + kp/2); free, q = (Pe + Pi)/k).
  - In the free chain (J3-3) it leaves Nw, S_pipe and the support reactions unchanged.
  - It moves q, the displacements u_C and u_D, the joint's elastic force g_x (Pe + Pi instead of Pe), the connector's endpoint rows and `s_x`, and so breaks the interface identity S_pipe = S_joint.
  - Kill tests on J3-3 must inspect those moved quantities, not global balance.

**Required checks** (both solver modes, criterion as in the reference README):
1. **PP system:** every `t4_u5a` case through PP's public entry: the connector rows (eight elastic and two new), member Nw and S, node displacements and rotations, support-on-model reactions, the region's `joint_edges` and the pressure RHS against (A). The interface identity is checked on published rows (Nw − Pi of each face pipe against `s_x`).
2. **Refusals:** the 17 refusal variants (RV-01 to RV-15 with RV-05a–c and RV-06b; there is no RV-10) with their exact code sets, `MODEL_INCOMPLETE` and no results. Unit tests, which have no oracle patch, with their codes from §5's field specification:
   - a joint beside a realized arc, and any other face pipe that is not a straight member of the region;
   - a duplicate `member_connector_ids` entry;
   - a connector listed by two regions;
   - a negative, a nonfinite and a wrongly dimensioned effective area.
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
   - faces exchanged (the oracle's `sign_flip`): both summands at each attachment take the other end's sign, so the whole pair −/+p(Ae − Ai) is reversed. Reversing only the `joint_effective_area` summand is a different mutant, which the oracle does not list; it is far from the correct values (J3-1 q ≈ −2.2e-5 m, J3-3 g_x = −800 N);
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
- **Publication.**
  - Each rod gets its own force row.
  - S_joint becomes Tb + Σ T_r·(n_r·Q_x) − Pe. This is the face-equilibrium identity S_pipe = S_joint. For rods parallel to the connector axis, n_r·Q_x = 1 and it reduces to Tb + ΣT_r − Pe.
  - Rods are not connector member loads either: the connector's g stays elastic.
- **Rulings (HELP_HUMAN, 2026-10-17).**
  1. **Non-parallel rods are admitted, with the projected rule above.**
     - *Basis.* The hardware already defines a general rod direction: e = n·(v_bⱼ − v_bᵢ), assembled as Cᵀk_rC. The earlier unprojected sum held only when n·Q_x = 1. On a skewed rod it breaks the interface identity by 28800/1661 N; `t4_u5b` SK-F is the discriminating control. Refusing "non-parallel" rods would need a parallelism tolerance with no physical basis.
     - *Left to the U5b design.* A rod nearly perpendicular to the joint axis. The U5b implementation design may refuse it by name if that is justified. It found no justification, and such a rod is admitted (u4b-u5b-implementation.md §3.4).
  2. **Rods with p = 0 at the joint are admitted.** This covers rods on an unpressurized connector and rods on a constant-effective-area connector that no region lists. There Pe = 0, and the projected rule applies unchanged.
     - *Basis.* Rods are hardware whose stiffness and reference extension act whatever the pressure. JR J-C's mechanical-only tied qualification is this case, and refusing it would leave a tied joint under thermal or mechanical load unsupported.
     - *If the U5b design disagrees.* If it finds a route or certificate reason to refuse these cases, it brings that to HELP_HUMAN. The references cover both outcomes. It found none, and these rods are admitted (u4b-u5b-implementation.md §3.4).
- **References.** Each was re-derived independently:
  - JR's ideal-tie rows 2 and 4 as the finite-rod limit k_r → ∞, with exact finite-k_r forms and their limits;
  - row 5's split (Tb = 2e5·q, Tt = 2e7·q at J3-5U's q);
  - offset rod pairs at ±a (K_rz = 2k_r·a², with cancelling cross terms);
  - JR's tied J4 control on the rod force: 800 + 75π instead of 800 − 50π anchored, and Pe + Pi instead of Pe free, with q, Nw and the reactions unchanged in the anchored tied case.
- **Qualified (2026-10-17).** [`validation/references/t4_u5b/`](../../../../validation/references/t4_u5b/README.md) covers each item, exact over Q(π) on the JR J3 fixture:
  - **Rows 2 and 4.**
    - Anchored: q = (800 − 50π)/(200000 + 13750000π + k_R). Free: q = 800/(200000 + k_R).
    - As k_R → ∞ the forms reach JR's ideal-tie values. As k_R → 0 they reach t4_u5a's J3-1 and J3-3.
    - A centreline rod and a ±a pair agree exactly.
  - **Row 5.** q = 1/25250 (J3-5U), Tb = 800/101 and ΣT_r = 80000/101. The anchored case is checked against J3-5UA.
  - **Offset pairs.**
    - A ±a pair gives K_xx = 2k_r and K_rz = 2k_r·a², and the cross terms cancel. Four rods stiffen both bending rotations.
    - A single offset rod is the only configuration where an offset-sign error shows. It acts through a series stiffness and does not make the joint axially rigid.
  - **The tied J4 control.**
    - ΣT_r → 800 + 75π anchored, and Pe + Pi free.
    - At finite k_R, the differences in q, Nw and the reactions are given exactly. They vanish only in the anchored limit.
  - **Other cases.**
    - Pretension (e_ref), reversal and rigid rotation.
    - The skewed rod SK-F.
    - The p = 0 controls:
      - MO-F, JR J-C's mechanical-only case;
      - MO-T, anchored thermal with pretension;
      - RR-U and RR-P, bound to J3-R and J3-RP.

  Every case is solved twice, by a nodal stiffness solve and by a force method, and the two agree exactly. The pressure RHS includes the Poisson pairs, as t4_u5a's does. §8 binds that convention to the product's `assembled_pressure_rhs_global`.

  A separate TASK refuted the set, using its own 100-digit nodal solve and hand free bodies, and confirmed every value. A second separate TASK confirmed the repairs and the new cases.

  The cases stay request-agnostic until 0.6.0 defines `explicit_axial_rods_v1`. Their oracle values do not move.
- **Decided by the U5b design, as implemented** ([u4b-u5b-implementation.md](u4b-u5b-implementation.md) §§3.1, 3.3 and 3.4):
  - JR §5's input refusals take `JOINT_TIE_ROD_INPUT_INVALID` (duplicate rods, missing offsets, contradictory hardware) and `JOINT_TIE_ROD_ZERO_LENGTH` (a degenerate represented length).
  - A rod's ends are always the connector's end nodes, so rods anchored elsewhere cannot be expressed.
  - `s_x` is the projected S_joint = g_x + Σ T_r·(n_r·Q_x) − Pe, published for every connector with rods (Pe = 0 when unpressurized) and for every constant-effective-area connector; `pe_x` is published only for a constant-effective-area connector.
  - Rods nearly perpendicular to the joint axis, and rods at p = 0, are admitted.
  - The ideal-inextensible rods and the collective constraint stay refused by name. They remain a later MPC unit, on the owner's choice only.

## Open questions

1. **The model field and DEC-033: closed.** HELP_HUMAN ruled on 2026-10-12 that the T4-U3 precedent does not extend to new content in 0.3.0/0.4.0. Both items are defined only in model 0.5.0 (Status; §5).
2. **The qualification package.** The `t4_u5a` cases are consumed by PP and HR tests. Adding them to VP-STATIC's `exact_pressure` package would need its own admission review; that is the manager's choice.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
