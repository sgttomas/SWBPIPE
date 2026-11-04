# The §4 combination certificate (build plan and as built)

This is the build record for `operating-case.md` §4. §0 adopted option (i)
without (ii). Agent 0 decided the eligibility question as (b) on 2026-11-04. It
serves RT-A5 and bounds W-16b's linear EX combinations. Paths are relative to
the repository root. PP is `core/product_physics/src`; RE is
`core/reporting/result_export/src`.

Nothing here records or implies acceptance, issuance or lifecycle standing
(F-PIP-2/F-PIP-3). The product text never says "certified" (R5).

## 1. What is built

For each derived group of a `result_state_subtraction` or `mechanics`
combination over load cases, the producer publishes a bound B_D and a verdict.
The readers re-derive both bit for bit from the published operand rows and the
published operand group bounds.

**Operand groups.** These are the K-D5 groups of DEL-04-01
`kd5-member-relative-bound.md` §3.2, already formed per case as PP
`formation_guard::RecoveryRecord`s, with no FK change. The groups are:
- member end, cut and station `axial`, `shear`, `torsion` and `bending` groups;
- `pipe_wall` groups;
- support force and moment groups and their magnitudes;
- connector and tie-rod groups.

Each group has its published values v_g, its 2-norm q_g, its bound B_g and the
body's coupled scale S*_g (force or moment class, from `member_scales`).

**Operand eligibility (§4.2 item 1).** A case is an eligible operand exactly
when the solve formed its member bound: `member_bound` is present after every
prior guard. That means:
- integrity Passed before the recovery predicate;
- no load-row finding;
- either a linear structural Passed with no retained source selected, or a
  confirmed nonlinear state (Stage B).

A gap-bearing case is `NUMERICAL_INTEGRITY_RECOVERY_BASIS_UNQUALIFIED` and
forms no member bound, so it is not eligible. When G2 (R-3) confirms gap
states, those cases form their member bound and become eligible through the
same condition, with no certificate change. A case that the unified predicate
then demotes keeps its certified rows and its bounds (§3.1), so it stays
eligible.

The readers decide eligibility from the published integrity diagnostic. The
case is eligible when its code is `NUMERICAL_INTEGRITY_CHECKS_PASSED`, or
`NUMERICAL_INTEGRITY_SENSITIVE` with the recovery-guard sentence ("S11-G
recovery guard (R-b'): load case …"). The guard sentence is emitted only after
a member bound is formed.

**Coverage.** Each operand group names its covered result rows, which are rows
of that case, in component order. The coverage table is closed, and a group's
row set is used only when every listed row exists in the case and the norm of
the rows' values equals q_g bit for bit (the binding). Unit and frame are the
rows' own unit and `coordinate_system`.

| Group (location) | Covered row set(s), each at the row location named |
|---|---|
| `axial` (end_i, end_j) | `element_local_axial_force` at the end, when the member's published axial rows are its elastic/effective force |
| `axial` (quarter_1, midspan, quarter_3) | the same at the station; also `pipe_effective_axial_force_v2` at the station |
| `axial` (cut_i, cut_j) | `pipe_effective_axial_force_v2` at end_i, end_j |
| `shear`, `torsion`, `bending` (ends, stations) | `element_local_shear_force_y`+`_z`; `element_local_torsional_moment`; `element_local_bending_moment_y`+`_z` |
| `pipe_wall` (wall_end_i, wall_end_j) | `pipe_wall_endpoint_action_v2` at the end; also `element_local_axial_force` at the end when the member's axial rows are wall rows |
| `pipe_wall` (wall_cut_i, wall_cut_j) | `pipe_wall_axial_force_v2` at end_i, end_j |
| `pipe_wall` (wall stations) | `pipe_wall_axial_force_v2` at the station; also `element_local_axial_force` there when the axial rows are wall rows |
| `support_force`, `support_moment` (node) | `support_reaction_component_v2` Fx, Fy, Fz; Mx, My, Mz |
| `support_force_magnitude`, `support_moment_magnitude` | `support_reaction_force_magnitude_v2`; `…_moment_magnitude_v2` |

A member's axial rows are wall rows on a pressurized straight EX member: the
producer publishes `wall` as the member rows there (PP `lib.rs`, the
`certified_publication::pressure_adjustment` branch). The case record names
those members (`wall_axial_members`). A reader admits a listed member only when
its global end rows carry `GLOBAL_END_WALL_BASIS`. Connector and tie-rod
groups, displacement rows, global end rows and stress rows have no row set.
Their combination rows are not covered and remain inspection results, and each
combination states their count.

**Derived groups.** For each operand group key and row set present in every
operand, the derived group's rows are the combination rows whose
`source_result_refs` are exactly the operands' rows for that component, in
evaluation order. With factors f_i (a subtraction: 1, −1):
- e_c = RU(|v_D,c − Σ f_i v_i,c|), formed exactly. For a subtraction this is
  the TwoSum error, which is zero under Sterbenz.
- B_D = RU(Σ |f_i|·B_i + ‖e‖₂).
- q_D = ‖v_D‖, the correctly rounded norm. It is |a|, `norm2` or `norm3`, as
  K-D5's q.
- q̂_D = max(q_D, RU(q_D + B_D)).
- S*_D = max_i RU(|f_i|·S*_i). This is the larger operand scale, the proposed
  derived floor of §4.2 item 6.

**Magnitude rows (§4.2 item 5).** A support magnitude row of the combination is
a derived group of its own. Its row must equal `norm3` of its derived component
rows bit for bit. Its bound is
B_M = RU(B_D,components + (next_up(v_M) − v_M)), where the last term bounds the
correctly rounded square root's error (0 when v_M = 0). q_M = v_M, and the
floor scale is the component group's.

**Upward rounding** (the same in Rust and Python):
- RU(a + b) and RU(a·b) are the binary64 result, or its `next_up` unless an
  exact comparison shows it exact.
- RU(√x) is the same with the exactness test r·r = x.
- ‖e‖₂ is RU(√(Σ RU(e_c²))), summed in component order.
- RU of an exact rational is the smallest binary64 not below it.
- A non-finite result fails closed.
- The exact rationals are `load_case_algebra::Dyadic` in Rust and `Fraction` in
  Python. Dyadic's 4,096-bit limit applies in both: Python checks the
  normalized significand after every operation, and a refused operation makes
  the group's comparison unavailable.

**Verdicts (§4.2 item 6)**, with formation_guard's own constants (RD(1e-9),
2^-34 and 2^-988):
1. A non-finite or invalid operand to the comparison gives `not_established`,
   reason `comparison_unavailable`.
2. Otherwise, S*_D < 2^-988 or q̂_D < 2^-34·S*_D gives `not_assessed`, reason
   `below_floor`.
3. Otherwise, B_D > RD(1e-9)·q_D, decided exactly, gives `not_established`,
   reason `bound_exceeds_criterion`. This is where the operands nearly cancel.
4. Otherwise the group is `established`.

**Combinations not assessed** keep their rows as inspection results, and each
is named with its reason:
- withheld by a gate (no rows);
- a range envelope, whose certification is deferred (§4.2 item 7);
- an absolute sum, an SRSS or a combination with a combination operand, all
  outside §4's scope;
- an operand case without certified rows.

**D-6 (R-4 §6.3, pressurized operands).** For a subtraction on EX, a member is
equal-pressure when both operand cases put it in a pressure region with the
same `p_pa`, `external_pressure_increment_pa` and `pressure_basis`, and with
bit-identical `geometry` and `materials` entries. Each `pipe_wall` derived
group of such a member carries `equal_pressure_elastic_group`, the derived
`axial` group at the same location.
- The intended wall-force difference equals the intended elastic (effective)
  axial difference exactly, since the pAi terms are formed from bit-identical
  inputs.
- That group's derived rows establish it within their own bound.
- The wall group keeps its own bound and verdict, which may honestly be
  `not_established`: its published rows carry the rounding of P + F.
- The same location has no published elastic counterpart at wall_end_i and
  wall_end_j on straight members, so the field is null there.

**Eligibility (b), decided by Agent 0, 2026-11-04.** Established derived groups
carry their own evidence and do not count toward the document's numerical
eligibility. `numerical_quality`, `reader_verdict.rs` and their Python mirrors
are unchanged. There is no policy switch.

## 2. What publishes

`contract_evidence.combination_certificate` is a closed member on PF and EX.
It is present exactly when the document is solved and the model has a
combination, so no other document's bytes change.
- `operand_cases`: one record per eligible load case that is an operand of an
  assessed combination, in the order of `numerical_quality.cases`. Each record
  is `{load_case_id, wall_axial_members, groups}`, where each group is
  `{entity, location, group, moment, unit, frame, rows, q, bound,
  floor_scale}`. `rows` is a list of row sets.
- `combinations`: one record per model combination in gate order. Each record
  is `{combination_id, basis, expression, assessment, reason, groups,
  uncovered_row_count}`, where each group is `{entity, location, group,
  row_set, rows, factors_bound, rounding_error, bound, q, q_hat, floor_scale,
  verdict, reason, equal_pressure_elastic_group}`.

The departure from §4.3's "extends W-16b's `exact_combinations` record shape"
is that this is a sibling member, not new keys in that record. It has the same
shape on PF and EX, it is not tied to W-16b's EX-only record, and it leaves the
held projection code (`exact_combination_evidence.rs` `direct_view`, C1)
untouched.

**Diagnostic** (info, one per combination, source `core/product_physics`, id
`diagnostic:combination-certificate:<identity>`). Code
`COMBINATION_ROWS_WITHIN_STATED_BOUND` when the combination is assessed;
otherwise `COMBINATION_ROWS_NOT_ASSESSED` with the reason. The text says
"established within its stated bound against the operand cases' intended
systems". It gives the counts per verdict and the uncovered rows. It states
that this is a numerical statement about a combination of independently solved
states, that it does not establish the meaning of "expansion" or engineering
acceptance, and that these rows do not count toward the document's numerical
eligibility.

## 3. References (independent of product code)

`validation/references/combination_certificate/` holds an exact-rational
generator with an independent π enclosure. It reads no product code.

| Case | Model and combination | Independent reference |
|---|---|---|
| Linear EX subtraction | the w16b line (`w16_exact_combinations`), `comb:S1` = C − A | its frozen exact rows: every derived group satisfies ‖v_D − D_exact‖ ≤ B_D; B_D is re-formed by the test's own TwoSum |
| Mechanics sum | the w16b line `comb:K1` = A + 1.3B; and K4 = C − A − B (exact cancellation) | K1: enclosure of the frozen exact rows; B_D = RU(B_A + RU(1.3·B_B) + ‖e‖); K4: every v_D = 0 and D_exact = 0, so each group is below the floor whenever B_D < 2^-34·S* |
| Cancellation-unestablished | an EX cantilever (L = 0.3 m) with tip (Fx, Fy); O − S with ΔFy = 2^-20 N | the anchor bending group: q_D/S* ≥ 2^-30 (assessed), and B_D ≥ |fl(Fy_O·L) − Fy_O·L| + |fl(Fy_S·L) − Fy_S·L| − 2^-100·S* > RD(1e-9)·q_D, so not established |
| Floor-excluded | the same cantilever, S′ − S with identical loads | v_D = 0 = D_exact; every group is below the floor (asserted against B_D < 2^-34·S*) |
| Equal-pressure axial group | the same cantilever, O − S with ΔFx = 2^-10 N at bit-identical pressure | D(N_w) = D(S) = ΔFx exactly; the wall groups name their elastic group; the wall station group is not established (its rounding gaps of P + F exceed RD(1e-9)·ΔFx) |
| PF confirmed nonlinear | op1a V-2 (one-way prop, s = 2s*): O − S | ΔR = −5P/16 and ΔMy = −5PL/16 enclosed by B_D; both operands are confirmed, so the combination is assessed |
| Gap operand | op1a V-1 (gap closing): O − S | both operands are gap-bearing (Unresolved), so `not_assessed` and no group is formed |

Every model runs in both solver modes. The readers (Rust RE and Python) read
the shared corpus `fixtures/results/combination_certificate_corpus.json`. They
refuse a forged B_D, verdict, q, floor scale, row binding, operand-bound
binding, eligibility and diagnostic.

**The eligibility witness** is the cancellation document. Its numerical
quality equals the primitive-case statement, unchanged. Its groups show
`established`, `not_established` (bound_exceeds_criterion) and `not_assessed`
(below_floor) together, and its diagnostic states the covered scope.

## 4. Shared-file hunks (reported, not widened)

PP:
- `lib.rs`: a `LoadCaseSolve` field, retaining the records after the recovery
  finding, and one call after the envelope is assembled;
- `formation_guard.rs`: two constants made `pub(crate)`.

RE:
- `physics_evidence.rs` (`validate_pressure_evidence` strips and checks the
  member);
- `preview_physics_evidence.rs` (the optional member and the check);
- `lib.rs` (`mod`).

Python: `tests/verifiers/pressure_2.py` and `preview_physics_evidence.py`, the
same two calls.

None in `preview_physics.rs` combination formation, the `preview_combination_*`
readers, `combination_plan`, `load_case_algebra`, the EX gate code or
`exact_combination_evidence.rs`.
