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
  member is refused by its shape (`SHAPE`), never read as absence. This holds
  for a combination record, an operand case or an operand group that is not an
  object, which both readers refuse by its shape with and without the model
  (`RECORD_SHAPE`, `SHAPE`). The model binding leaves such a record to the shape
  check rather than dereferencing it. With the member, the reserved notices
  must be exactly the ones it re-derives, so an extra or renamed establishment
  notice is refused (`DIAGNOSTICS`).
- *Independently validated retained-precision claims are not §4 claims.* The
  registered W-CB3 retained combination successors were recorded before the
  certificate. Their bytes are kept unchanged as the legacy successors
  (`retained_precision_combination_successor_<mode>_pre_certificate.json`,
  since the D3 registration regenerated the live ones, §6). Their
  retained-precision standing is still governed by their own receipt. They
  simply make no §4 claim.

**The invariant's scope (Agent 0, 2026-11-04, after review cert2).** The old
invariant read: "Without valid evidence, no certificate-derived row bound,
eligibility effect or established notice survives." Review cert1 stated the
same requirement as "No §4-derived bound, eligibility effect or established
notice without valid §4 evidence." It is replaced by: **"No READER-ATTESTED
certificate claim survives without valid bound evidence."** A reader-attested
claim is one the readers admit as a certificate statement: the member's
structured bounds and verdicts, and the roster's canonical notices below. The
new wording does not mean that the reader can recognize all establishment
prose. Prose the readers do not attest is handled by the roster and the text
boundary below, not by reading its meaning. Two rules stand unchanged:
- Absence under the current optional inspection contract does not select
  history.
- Historical variants are selected only by a recorded `{id, sha256}`.

**The closed combination-diagnostic roster (review cert2 finding 1, Agent 0's
option A).** Both readers (RE `combination_certificate.rs` `roster`, Python
`roster`) read it after the member, on a solved document.
- *Which diagnostics.* A diagnostic is a combination diagnostic when an
  affected ref names a combination identity of the document, whatever its id
  or code. The identities are taken from the gates, the EX records, the
  certificate and operand records, and the owners of combination-typed rows
  and numerical cases. A literal prefix plays no part: the unicode and W-CB3
  documents' identities carry none. An identity the document also gives
  another entity (a load case, a row's entity) is left out, because an untyped
  ref naming it cannot be told apart.
- *The model only adds (review cert3 finding 2, Agent 0's no-masking rule).*
  With the model, a model combination the document does not state is added,
  unless another document or model entity shares its id. No model record
  removes an identity the document types as a combination's: an unused model
  material named `comb:cancel` does not take `comb:cancel` out of the roster,
  so a claim under it is still refused (`ROSTER_EXTRA`). The document's own
  population (`document_combination_identities`, the report's) is therefore
  always within the population the roster checks, with or without the model,
  and the report never copies as roster-checked a message the model-bound
  reader left unchecked. A model-only combination's notices are checked with
  the model and quoted as supplied text by the source-only report, which
  understates, never promotes. The ambiguous untyped ref is still not
  interpreted: a ref the document itself shares with another entity stays
  outside the roster in both readings.
- *Canonical form.* Each combination diagnostic must be an entry of the roster.
  Its id names one entry, and its code, severity, message, source and refs are
  that entry's, derived from the evidence. A diagnostic carrying a set-bound
  entry's exact id is read even when its refs name only an identity left out
  above (a combination sharing a load case's id, PP V7-a): the id selects the
  entry it is compared with whole and admits nothing alone, and the delegated
  skips below still come first.
  - Set-bound, each required once: PF's headline scope, its maximum and
    intensified notices per admitted gate, and its gate notice per withheld
    gate (an operand withholding names its withheld operands). A held gate's
    notice (RT-A5, `operating-case.md` §7.5: the five `HELD_STATUS_*` gate
    codes) is exactly "Combination withheld: " + R + ".", where R is the
    refusal stated by the document's held-status record for that combination
    (`held_status_derivation`, `standing.reason` of a `withheld` record of that
    outcome); the canonical entry is built from the record and the notice is
    compared with it whole. The held-status reader and the gates reader, which
    precede the roster, bind the record to the combination and its refusal code
    to the gate's; a held code whose record is absent, repeated, malformed or of
    another outcome is refused (`ROSTER_CONTRADICTION`). Every other gate code
    keeps its fixed text.
  - *The held refusal's boundary (Agent 0, 2026-11-05, option (a), bound).*
    Binding R to the record establishes correspondence, not truth. R supplies
    no row bound, eligibility, established or certificate notice and no
    numerical standing; the held combination's record stays `not_assessed`
    with no group and no bound. Its language is not policed: the reserved
    vocabulary's message test does not apply to the one notice that is a
    withheld held gate's own, recognized by typed identity and its whole
    canonical message (its code, its one combination, that gate's id, the
    gate withheld with that code, exactly one held-status record, a withheld
    one of that outcome, and the message exactly "Combination withheld: " + R
    + "."; RE `bound_held_gate`), so a claim-like R in a well-bound record is
    admitted and changes nothing. The exemption lifts only that test: the
    gates and held-status readers, which precede the certificate, still
    refuse a forged binding or state. In the retained base read (G7,
    `retained_precision.rs`'s `retained_base_read(|| for_source(&projected))`)
    the roster is not applied, so a held notice whose text is not R is
    admitted unread with ordinary wording (G7 attests no combination
    diagnostic prose) and refused by the absence rule with the reserved
    wording, since it is then not the bound entry.
  - *Presentation.* Every presenting path quotes a notice whose typed code is
    a held-status code whole as supplied text, whatever the roster or a
    certificate attests: the five held gates and `HELD_STATUS_RELEASED`
    (`HELD_STATUS_QUOTED_CODES`, a set distinct from the gate set), in desktop
    `suppliedDiagnostics.ts` (the Diagnostics panel, the result
    interpretation and the report projection), the run comparison, and Rust
    `report_package::bindings` (every native report path). Quoting is
    presentation only: it leaves the result's raw diagnostic and evidence
    bytes untouched. The producer's `HELD_STATUS_RELEASED` wording changes on
    purpose for fresh output (P1, below; priced and recorded in DEL-04-08
    Work `retained-pricing-rules.md` §8.54); recorded bytes and the recorded
    variant selection are preserved.
  - *`HELD_STATUS_RELEASED` (P1, RT-A3).* One set-bound entry per held
    combination whose validated held-status record releases a support: id
    `diagnostic:preview-physics:held-status-released:<identity>`, warning, refs
    the combination then the released supports in record order, and the whole
    message rebuilt from the record's recorded fields only (support ids, held
    and classified states, release reasons, and the deciding bit token or
    strict-gap signs; `operating-case.md` §7.5), compared bit for bit. The
    held-status reader, which precedes the roster, binds those fields to the
    record's re-derived decisions and refuses first a notice naming its
    supports out of order, a duplicate, a dropped notice and one on a
    combination that releases nothing; the roster refuses a changed token,
    sign, state or reason (`ROSTER_CONTRADICTION`). The wording is outside the
    reserved vocabulary, and the notice is quoted whole by every presenter. The headline's
    refs are the model's kept combinations in authored order; without the
    model, the gates' identities in any order, which the document cannot fix.
    With the model, S1's interim withholdings and their dependants, on both
    routes; as RT-A5 leaves S1 there are none (a `minuend_final` subtrahend is
    gated at the solve).
  - Form-bound, each parameter bound to the evidence: PF's fixed-load count,
    source-missing (a row base of the document), row-withheld (each name a row
    base or a support with a producer reason) and vector notices, and the
    retained-precision combination notices (their fixed producer texts; their
    existence is the retained reader's G4).
  - Read whole by their own readers, which precede the roster: the certificate's
    own notices (`check`), EX's `diagnostic:exact-combination:` namespace
    (W-16b's conditional notice included) and PF's operand standing and state
    notices (`preview_combination_operands`).
- *Refusals.* `SOURCE_COMBINATION_CERTIFICATE_`:
  - `ROSTER_EXTRA`: no entry, including a duplicate;
  - `ROSTER_RENAMED`: a roster code under an id outside every roster family;
  - `ROSTER_CONTRADICTION`: an entry's id with another form;
  - `ROSTER_MISSING`: a set-bound entry absent.
- *Reserved vocabulary.* The reserved certificate codes and canonical notices
  stay protected by `check` whatever their id (the absence rule above). That
  protects the known codes and canonical texts. It is not a semantic
  validation of arbitrary paraphrase, which the roster refuses under a
  combination ref instead.
- *The retained successor's base read (Agent 0's ruling, 2026-11-04).* The
  retained reader's G7 base read runs in an explicit read context
  (`retained_base_read`, Rust and Python). Only that read enters it; it is
  never inferred from a document's content, absence or age, and it selects no
  history. Under it the roster's text closure alone is not applied. A present
  member is still validated in full, and the reserved certificate codes and
  canonical notices stay protected, so a certificate notice without a valid
  member is still refused (`PRESENCE`). No reader-attested certificate fact
  comes from prose. **The scope of this read (review cert3 finding 3).** It
  is no precommit-only exception: every valid read through the public
  retained reader and CLI reaches it. G7 checks the projected successor for
  base consistency inside the retained reader. It does not attest
  combination diagnostic prose. G4 binds the combination
  selected/unavailable notice codes, their cardinalities and the combination
  each names (an unavailable notice's id too), not their message text; G5
  binds diagnostic membership; the receipt and publication hashes bind
  supplied bytes, not truth or producer origin. A coherently rehashed
  successor whose selected combination notice is reworded, or which appends
  a note under a combination ref and lists it in that combination's
  `diagnostic_refs`, passes the full retained reader in both languages (the
  review's probes); its text gains no standing. Prose excluded from the
  ordinary roster remains supplied, unverified text wherever exposed: the
  Diagnostics panel quotes every message of a retained successor with
  combinations after the supplemental label (desktop
  `suppliedDiagnostics.ts`, below), except the certificate's own notices
  where the registered retained reader accepted the successor carrying the
  member (§6), and retained identities are not report sources.
  Reader-attested §4 facts require the validated member and its canonical
  notices. The shared retained corpus (`retained_precision_cases.json`),
  whose base documents carry the pre-T4-U4b maximum text, and its pinned
  native outcomes were unchanged by this stage; the D3 registration
  regenerated its combination cases (§6). Retained identities are not admitted report sources
  (`report_package` `document_form::ADMITTED_SEMANTIC_CONTRACTS`).

**The residual text boundary (Agent 0's item 5).** Diagnostic prose under refs
that name no combination (a load case, a support, the model, or no ref) is
admitted only as unchecked supplemental data. Two consumers show it beside
derived-row and certificate content, with one classification (desktop
`features/results/suppliedDiagnostics.ts`): the report's diagnostics
projection (`renderableReportInput.ts` `diagnosticsForSections`, checked on
the exact route by `report_package` `bindings::expected_report_diagnostics`),
and the Diagnostics panel's list and explanation (review cert3 finding 1:
`DiagnosticsPanel.tsx`, `resultInterpretation.ts`).
- On a result with combinations, such a message is quoted whole after the
  label "Supplied diagnostic text, unverified (not a combination certificate
  statement): ".
- A diagnostic naming a combination is a roster entry the reader admitted in
  its canonical form, and is copied. The report takes the roster as checked
  because its exact and PF routes are admitted by the reader before assembly. That
  admission is a precondition the helper relies on, not a check it performs:
  `diagnosticsForSections` is not a standalone validator. A route or caller
  that cannot establish reader admission before assembly must use the
  unchecked presentation or perform admission first. Only the exact and PF
  report routes were qualified (review cert4 note 2).
  The panel copies it only when the current Rust reader admits the result
  itself, whole, on an ordinary route (the header dispatch's route and that
  route's full reader, `readerValidate`, without the model, whose population
  is the result's). On a refused or unread result, and on a
  retained successor (whose G7 base read applies no text closure, above),
  every message is quoted as supplied.
- Certificate facts and names (established, not established, not assessed,
  bounds, the per-combination statement) come only from the validated member
  and the roster's canonical notices. No establishment, eligibility, health,
  token or summary is derived from free text: no desktop or report code reads
  a message for standing.
- Report text keeps the renderer's ordinary escaping, and no phrase list is
  kept.
- A result without combinations is unchanged.

The shared parity vectors (`fixtures/reports/invented/report_diagnostics_parity.json`)
and both languages' test on an actual certificate document hold the rule. The
panel's test (`DiagnosticsPanel.supplied.test.tsx`) renders the actual
component on the EX cantilever with the review's model-ref note, the
model-only id collision, both retained counterexamples, and a result without
combinations.

**Not assigned: a document-wide closed diagnostic roster (option B).** Closing
every diagnostic family of the document, not only those naming a combination,
is not assigned. There is no new Must and no standing backlog. Free prose under
non-combination refs is the residual. It is quoted as supplied text where it
appears beside combination content, and is for later judgement only if a
concrete consumer needs more.

**EX limitations: withdrawn from this scope (Agent 0, 2026-11-04, option (3)).**
The earlier requirement read: "EX `formulation_basis.limitations` must equal
the roster of the actually selected table variant. PF already does this." It
is withdrawn, because it rested on an incomplete premise. No table variant
carries an EX limitation roster. Recorded pressure-2 runs reach the current
reader with no binding, because the compatibility layer maps their recorded
digests to the current table. A bare envelope names no table digest. The
scope now reads: **"The current reader does not attest the whole EX limitation
roster. Raw EX limitation prose never supplies a reader-attested certificate
claim."**
- The EX readers keep their existing check: each limitation is non-empty text.
  A source they accept has not had its limitation roster checked, so
  acceptance does not mean every claim in it is verified.
- Typed certificate facts, establishment and eligibility come solely from a
  valid member and the canonical roster.
- The exact report keeps refusing any limitation statement it has no
  registered statement for (`report_package` `exact_sections::expected_exact_limitations`,
  `EXACT_LIMITATION_UNREGISTERED`; desktop `exactReportStatements`).
- The review's bypass is a regression test at that boundary: an appended
  "combination comb:cancel: all derived row groups are established within
  their stated bound of 1e-30." is admitted by the source readers as untrusted
  text, it changes no member, roster entry or standing, and the exact-report
  builder refuses it.

No allow-list of historical rosters is kept, no history is selected by absence
and no record binding is synthesized. Selecting history by its recorded
digest (a roster registry keyed by the pressure-2 table digest) is a possible
separate compatibility closure. It is not assigned, and is neither a release
Must nor a backlog item.

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
fallback. If they fall back, the member must be dropped at staging. (The D3
found an earlier fallback, at the W1 freeze, and repaired it there; the
successors carry the member and read true: §6.)
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
| a combination record null, a scalar or an array | RECORD_SHAPE (with and without the model) |
| an operand case or an operand group null, a scalar or an array | SHAPE |

The roster's cases (review cert2 finding 1), applied with and without the model:

| Case | Code |
|---|---|
| the review's prose (REVIEW_NOTE, a paraphrase, "bounded by 1e-30", uppercase, NBSP) under a combination ref, member kept or absent | ROSTER_EXTRA |
| the same prose under a load-case ref, no ref or a model ref | admitted (supplemental) |
| a maximum notice copied, or renamed, under an id outside every roster family | ROSTER_RENAMED |
| a maximum, headline or gate notice reworded, a headline missing a combination, a maximum naming two | ROSTER_CONTRADICTION |
| a row-withheld notice naming prose | ROSTER_CONTRADICTION |
| a source-missing notice naming no row base | ROSTER_EXTRA |
| a maximum or gate notice dropped | ROSTER_MISSING |
| the headline's refs reordered | admitted without the model; ROSTER_CONTRADICTION with it |
| pf_shared_id's maximum notice for `S` (a load case's id too): reworded, under another code, naming a load case, with an extra field; duplicated; dropped | ROSTER_CONTRADICTION; ROSTER_EXTRA; ROSTER_MISSING |
| pf_held: S1's removed interim withholding notice restated | ROSTER_EXTRA |
| pf_held: the held record stated assessed | RECORD |
| a held gate notice (RT-A5's held-status corpus): its inner text not the record's refusal; its frame changed; the refusal changed in the record only | ROSTER_CONTRADICTION |
| a held gate notice duplicated; dropped | ROSTER_EXTRA; ROSTER_MISSING |
| a HELD_STATUS_RELEASED notice: a changed bit token, sign, state or reason | ROSTER_CONTRADICTION |
| a HELD_STATUS_RELEASED notice: its supports out of order, duplicated, dropped, or on a combination that releases nothing | the held-status reader's HELD_STATUS_DERIVATION |
| a held code on a gate with no held record; another code on a held record's gate | the gates reader's GATE_DERIVATION |
| a claim-like refusal ("established", a tiny bound, any capitalization, Unicode) in a well-bound record | admitted; no row bound, eligibility or certificate standing changes |
| the same refusal in the notice only | the certificate's own notice reader's DIAGNOSTICS for the reserved wording, else ROSTER_CONTRADICTION |
| the same refusal in the record only | ROSTER_CONTRADICTION |
| a well-bound claim-like refusal with a forged record outcome, source case or state, or its gate not withheld | the gates or held-status reader's refusal |

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
- `lib.rs` (`mod`, and the re-export of `document_combination_identities` for
  the report's text boundary);
- `preview_combination_gates.rs`: `RowIndex`, its `new` and `cases` made
  `pub(crate)` (visibility only), so the roster reads the row bases as the
  nested-row reader does;
- `exact_combination_evidence.rs`: the `DERIVED_TEXT` constant only, not the
  held projection code.

Report (the text boundary): `report_package` `bindings.rs`
(`expected_report_diagnostics` and its label) and the desktop's
`renderableReportInput.ts` (`diagnosticsForSections`).

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
readers' logic, `combination_plan`, `load_case_algebra`, the EX gate code or
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

## 6. D3 registration (DEL-04-08 `retained-pricing-rules.md` §8.53)

- **The W1 freeze.** With the profile registered, every retained run with a
  combination fell back with `W1Fallback::Candidate` before staging: the
  preview freeze's closed evidence-key list (`retained_product.rs`
  `preview_case_evidence`) did not admit the member. It now does when the
  member is present (two lines in place; no coefficient moves). Route E
  admits no combination (D1.9).
- **(a) The successor reads true.** The regenerated W-CB3 successors and the
  combination corpus cases carry the ordinary run's member and notices. The
  full retained reader accepts them in Rust and Python, and their projected
  base passes the ordinary certificate check (RE
  `registered_w_cb3_successors_carry_the_certificate_and_it_reads_true`,
  Python `test_a_registered_retained_successor_carries_the_certificate_and_it_reads_true`).
  Each reseals to its pre-certificate bytes once the member, its notices and
  ids, and the freeze's adapter counts are removed
  (`tests/test_rt_a3_certificate_d3_retained_diff.py`).
- **(b) The retained label.** The Diagnostics panel copies a retained
  successor's certificate notices (the typed id and a reserved code only)
  when the registered retained reader accepted those exact bytes and the
  member is present: G7 validates the member on the projection, whose
  diagnostics are the successor's own, so those notices are the ones the
  validated member re-derives. Every other message of a retained successor
  stays supplied text; the roster's text closure is still not applied there
  (`suppliedDiagnostics.ts`, `DiagnosticsPanel.supplied.test.tsx`).
- **(c) The legacy successors.** The pre-certificate W-CB3 documents are kept
  byte for byte (`..._pre_certificate.json`; their SHA-256 pins are
  unchanged). `legacy_w_cb3_successors_are_accepted` and its Python
  counterpart, and the retained-context unit tests whose premise is a
  successor without the member, read them. The live successors publish with
  no precommit fallback.

