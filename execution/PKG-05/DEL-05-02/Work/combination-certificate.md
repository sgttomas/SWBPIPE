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
| `axial` (end_i, end_j) | `element_local_axial_force` at the end |
| `axial` (quarter_1, midspan, quarter_3) | the same at the station; also `pipe_effective_axial_force_v2` at the station |
| `axial` (cut_i, cut_j) | `pipe_effective_axial_force_v2` at end_i, end_j |
| `shear`, `torsion`, `bending` (ends, stations) | `element_local_shear_force_y`+`_z`; `element_local_torsional_moment`; `element_local_bending_moment_y`+`_z` |
| `pipe_wall` (wall_end_i, wall_end_j) | `pipe_wall_endpoint_action_v2` at the end |
| `pipe_wall` (wall_cut_i, wall_cut_j) | `pipe_wall_axial_force_v2` at end_i, end_j |
| `pipe_wall` (wall stations) | `pipe_wall_axial_force_v2` at the station |
| `support_force`, `support_moment` (node) | `support_reaction_component_v2` Fx, Fy, Fz; Mx, My, Mz |
| `support_force_magnitude`, `support_moment_magnitude` | `support_reaction_force_magnitude_v2`; `…_moment_magnitude_v2` |

A pressurized straight EX member publishes no `element_local_axial_force` row; its
wall and effective rows replace it. So its end `axial` groups have no row set, and
no row is covered by two groups' meanings. As built, the binding (q equal to the
rows' correctly rounded norm) is the producer's self-check and the readers'
re-derivation. Connector and tie-rod
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

**Premise: the operand bounds are each operand case's own; a held operand is a
different system and is not assessed.** A combination whose
`subtrahend_support_state` is anything but the default independent solve
(absent or `own`), today `minuend_final` (`operating-case.md` §0; O-C2 of the load-cases lane's §7), is not
assessed, whatever its gate says. Its subtrahend is re-solved at the minuend's
final support status, a different system from that load case, so the load
case's own B_g does not bound it. The record names the member
(`subtrahend_support_state`: null, `own`, `minuend_final`, or `unrecognized`
for any other present value) and both readers fail closed on any value but null
or `own`. Admitting held operands with their own bound is a later premise change,
not part of this build.

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
The producer publishes it exactly when the document is solved and the model has
a combination, so no other document's bytes change.

**Producer emission and reader acceptance are separate obligations.**
- *Producer (mandatory emission).* The current producer publishes the member
  exactly when the document is solved and the model has a combination. The
  schema descriptions state this producer obligation: "present exactly when a
  solved document has a combination" in `results_pressure_2.v0.3.schema.json`
  `CombinationCertificate`, and "omitted otherwise" in its
  `PressureContractEvidence`. They were left unedited so that the schema
  digests stay as pinned. The closed shapes keep the property optional, and
  its reading is the readers' rule below.
- *Reader (absence is no certificate claim).* The readers accept a document
  without the member as making **no §4 certificate claim**, whatever the
  document's age. Absence never selects historical semantics: a historical
  table variant is selected only by a record's recorded `{id, sha256}` binding
  (`absolute-sum-srss-combinations.md`, the recorded-run table bases), never
  by the member's presence. Without the member:
  - its combination rows are inspection results;
  - no row carries a certificate bound or verdict, and nothing changes the
    document's numerical eligibility (decision (b): the certificate has no
    eligibility effect either way);
  - no certificate notice may stand, under any id. The certificate's notices
    are a closed reserved vocabulary: an id in `diagnostic:combination-certificate:`,
    a code in the `COMBINATION_ROWS_` family, or a message that places rows
    within, or assesses them against, a stated bound (it mentions "stated
    bound" with "within" or "against"). A document without the member that has
    any such diagnostic is refused (`SOURCE_COMBINATION_CERTIFICATE_PRESENCE`).

  The readers do not treat a missing member as a defect, because an envelope
  names its table, not the table's digest. A reader therefore cannot tell a
  document recorded before the certificate from one without it.

  A stated member is read in full. Each of `operand_cases`, `combinations` and
  every operand case's `groups` must be an array. A partial, null or malformed
  member is refused by its shape (`SHAPE`), never read as absence. With the
  member, the reserved notices must be exactly the ones it re-derives, so an
  extra or renamed establishment notice is refused (`DIAGNOSTICS`).
- *Independently validated retained-precision claims are not §4 claims.* The
  registered W-CB3 retained combination successors were recorded before the
  certificate, and their bytes are unchanged. Their retained-precision standing
  is still governed by their own receipt. They simply make no §4 claim.

**Transport.** The preview-physics transport metadata check
(`retained_precision.rs` `preview_physics_transport_metadata`, and Python
`validate_transport_metadata`) admits the member on its closed shape
(`combinations` and `operand_cases`, both arrays). It cannot do more: the
transport view has no rows, diagnostics or status (the stress-neutral package's
`_transport_contract`), and the source reading checks the member against the
rows. Before that change, the transport refused the producer's own current
documents with a combination.

**Retained successors (for D3).** A retained successor is staged from the
ordinary run's envelope, so on the registered build it carries the ordinary
run's certificate. The precommit validation reads that certificate against the
successor's rows. An operand with a retained source selected is not eligible,
so its combinations are not assessed. An assessed combination's operands keep
their ordinary rows. The certificate should therefore read true on the
successor, but this lane could not run the registered route (the profile is
Stale), so this is not shown. The D3 registration must regenerate the W-CB3
combination successors and confirm that they publish with no Precommit
fallback. If they fall back, the member must be dropped at staging.
- `operand_cases`: one record per eligible load case that is an operand of an
  assessed combination, in the order of `numerical_quality.cases`. Each record
  is `{load_case_id, groups}`, where each group is
  `{entity, location, group, moment, rows, q, bound, floor_scale}`. `rows`
  lists, per row set of the coverage table, the covered result ids in component
  order, or null. The unit and frame are the covered rows' own (`unit`,
  `metadata.coordinate_system`). `bound` is null when the case's bound for that
  entity is unavailable.
- `combinations`: one record per model combination in gate order. Each record
  is `{combination_id, basis, expression, subtrahend_support_state, assessment, reason, groups,
  uncovered_row_count}`, where each group is `{entity, location, group,
  row_set, rows, factors_bound, rounding_error, bound, q, q_hat, floor_scale,
  verdict, reason, equal_pressure}`. `equal_pressure` is null, or
  `{elastic_group}` on a D-6 wall group. Non-finite numbers are null.

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
| Cancellation-unestablished | an EX cantilever (L = 0.3 m) with tip (Fx, Fy); O − S with ΔFy = 2^-16 N | the anchor bending group: q_D/S* ≥ 2^-30 (assessed), and B_D ≥ |fl(Fy_O·L) − Fy_O·L| + |fl(Fy_S·L) − Fy_S·L| − 2^-100·S* > RD(1e-9)·q_D, so not established |
| Floor-excluded | the same cantilever, S′ − S with identical loads | v_D = 0 = D_exact; every group is below the floor (asserted against B_D < 2^-34·S*) |
| Equal-pressure axial group | the same cantilever, O − S with ΔFx = 2^-10 N at bit-identical pressure | D(N_w) = D(S) = ΔFx exactly; the wall groups name their elastic group; the wall station group is not established (its rounding gaps of P + F exceed RD(1e-9)·ΔFx) |
| PF confirmed nonlinear | op1a V-2 (one-way prop, s = 2s*): O − S | ΔR = −5P/16 and ΔMy = −5PL/16 enclosed by B_D; both operands are confirmed, so the combination is assessed |
| Gap operand | op1a V-1 (gap closing): O − S | both operands are gap-bearing (Unresolved), so `not_assessed` and no group is formed |

Every model runs in both solver modes. The readers (Rust RE and Python) read
the shared corpus `fixtures/results/combination_certificate_corpus.json`. They
refuse a forged B_D, verdict, q, floor scale, row binding, operand-bound
binding, eligibility and diagnostic. The shared forgery list
(`combination_certificate_forgeries.json`) is applied with and without each
document's model. A `model_code` names the code where the model contradicts
the forged record first. The list includes the absence matrix on both actual
envelopes:

| Case | Code |
|---|---|
| member dropped, its notices kept | PRESENCE |
| notices dropped, the member kept | DIAGNOSTICS |
| both dropped | admitted, no certificate claim (asserted separately) |
| member dropped, notices kept under renamed ids | PRESENCE |
| member and notices dropped, an establishment sentence under the reserved code | PRESENCE |
| member and notices dropped, the canonical sentence under another code and id | PRESENCE |
| member kept, an extra establishment sentence or renamed notices | DIAGNOSTICS |
| a null member, or `operand_cases` missing or null | SHAPE |
| `combinations`, `operand_cases` or an operand case's `groups` not an array | SHAPE |

RE `absence_is_inspection_only_with_no_certificate_claim` and its Python
counterpart admit the both-dropped case. With and without the model, they
assert three things:
- no member survives;
- no certificate-vocabulary diagnostic survives;
- numerical_quality and the reader standing equal the certified document's.

RE `legacy_w_cb3_successors_are_accepted` and its Python counterpart accept the
legacy successors under their own receipt. The Python test also pins the
files' SHA-256.

**The eligibility witness** is the cancellation document. Its numerical
quality equals the primitive-case statement, unchanged. Its groups show
`established`, `not_established` (bound_exceeds_criterion) and `not_assessed`
(below_floor) together, and its diagnostic states the covered scope.

## 4. Shared-file hunks (reported, not widened)

PP:
- `lib.rs`: a `LoadCaseSolve` field set before `nonlinear_state` in every
  construction, because the dynamics lane's structural test reads `nonlinear_state`
  as the last field; the operand retained after the recovery finding; the operands
  collected in the assembly loop; and one call after the envelope is assembled.
- `formation_guard.rs`: two constants made `pub(crate)`.
- `exact_combinations.rs`: one string, the `EXACT_COMBINATION_DERIVED_FROM_CASE_RESULTS`
  text. "They carry no numerical check of their own" became untrue.

RE:
- `physics_evidence.rs` (`validate_pressure_evidence` checks the member and
  strips it);
- `retained_precision.rs` `preview_physics_transport_metadata`: the member
  admitted in the closed evidence key list and read by its check (two hunks);
- `preview_physics_evidence.rs` (the optional member and the check);
- `reader_verdict.rs` (the model binding on both routes);
- `lib.rs` (`mod`);
- `exact_combination_evidence.rs`: the `DERIVED_TEXT` constant only, not the
  held projection code.

Python: `tests/verifiers/pressure_2.py`, `preview_physics_evidence.py`
(including `validate_transport_metadata`), `reader_verdict.py` and
`exact_combinations.py` (`DERIVED_TEXT`), the same calls.

Tests that build documents from a producer document without its certificate
drop the member and its notices: RE `preview_physics_contract.rs`
`with_admitted_combination`, RE `exact_combination_contract.rs` (the direct
view), Python `test_rt_a4_pf_reader_tampers.py` (key set). PP `op1a_operand_standing.rs`,
`preview_physics_runtime.rs`, `w16_exact_combinations.rs` and
`rt_a4_combinations.rs` filter the member in their key-set and equality
assertions and assert its presence.

None in `preview_physics.rs` combination formation, the `preview_combination_*`
readers, `combination_plan`, `load_case_algebra`, the EX gate code or
`exact_combination_evidence.rs`.

## 5. Desktop carriers (regenerated with their writers)

The desktop vitest writers regenerated the carriers that bind the restated
pressure-2 table (`2c20583d` → `21b7426e`). The guard
`tests/test_rt_a3_combination_certificate_envelope_diff.py` (`DESKTOP_WRITERS`)
now compares them like every other carrier. The carriers and their writers are:
- the four W-11 exact-report requests and the same-run substitute overlay
  (`exactReportBelowGate.test.tsx`, `OPENPIPESTRESS_WRITE_EXACT_PACKAGE_REQUESTS`);
- `user_release_exact/pressure_2_straight.json` (`userReleaseExact.parity.test.ts`,
  `OPENPIPESTRESS_WRITE_D6_VECTORS`);
- the four desktop-shaped plain-connected carriers (`exactOutputTwins.test.tsx`,
  `DESKTOP_STRESS_NEUTRAL_WRITE_FIXTURES`).

The writers left `pressure_source_1` byte-identical.

**Inverse proof.** For each regenerated carrier, the guard restates only two
things back to the endpoint's values: its moved digests, each first verified as
derived, and its restated table statement. The result must reproduce the
endpoint's bytes exactly.

**W-11 run records.** The AnalysisRun record of each W-11 request (and of the
substitute) is bound by digest but materialized by no carrier. Its pairs are
named in `UNMATERIALIZED`. They were derived, not copied from the writer's
output:
1. The writer's own records were captured once, by a transient,
   uncommitted dump.
2. Each record's own `analysis_run_record` projection hashes to the new digest.
3. The same record, with only the pressure-2 table digest restated, hashes to
   the endpoint's digest.

Each endpoint digest is increment 4's new value for that request, so the chain
is continuous.

**Generation records.** The product-preview generation records
(`demo_fixture_generation.json`, `preview_physics_fixture_generation.json`)
were regenerated by `generate_product_preview_mechanics.mjs`, with Cargo run
through the host wrapper. Only their bound reader-source digests and inventory
digest moved; their outputs are byte-identical.
