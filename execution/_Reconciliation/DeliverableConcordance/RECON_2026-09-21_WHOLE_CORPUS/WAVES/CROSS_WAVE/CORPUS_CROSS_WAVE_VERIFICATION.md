VERDICT: 4 LEGITIMATE, 1 CORRECT, 0 UNDECIDED

# Corpus cross-wave consistency verification

Run HELP-HUMAN-PIPING-20260921-RECONCILIATION. I am a fresh TASK (Type 2)
verifier working from evidence only. I am independent of every manager,
worker and verifier in the run. My parent is HELP_HUMAN Agent 0.

- Brief: `R2-VERIFIER_brief.md`, SHA-256 `47fb3c52…5b00`, which matches the
  launch message.
- Findings input: `CROSS_WAVE/CORPUS_BATCH_FINDINGS.md`, SHA-256
  `d1a49d13…f938`, which matches.
- Evidence comes from the freeze checkout of `00115c71931b…`. Freeze paths
  below are relative to `projects/chirality-piping/`. Ledger references are
  CSV record numbers, counting the header as record 1.
- Rules applied: `CANONICAL_SITUATIONS.md` CP-03 (line 73) and CP-11
  (line 84); `CONVENTIONS.md` C1, C3, C5, C6(a)/(b), C7, F1 and F8.
- Scope is the 5 flagged rows only. I edited no ledger and no resolutions
  file. I used no git, builds, tests or network.

## What the two patterns fix

- **CP-11** fixes two things. The row is never `ALIGNED`, and its
  disposition is `PARTIALLY_IMPLEMENTED` or `DOCUMENTED_UNIMPLEMENTED`, "with
  the gap's cause". It prescribes no tier, baseline class or layer. The batch
  majority for CP-11 is the 10 DEL-17-05 rows. They are
  `DEFERRED_BY_RULING · LOCAL_DESIGN · RULED_CRITERION · RECORD`, and DEC-080
  backs them. A row whose own gap is not deferred by a ruling must take its
  own cause. Its tier and layer follow C3, C5 and F8. The prior cross-package
  verifier reached the same reading. I checked it against the text and agree.
- **CP-03** fixes less. The row is its own `DECLARED_STATE` row and does not
  change its siblings. Pins go to CP-02 and delegation to `## Remaining` goes
  to A4. It prescribes no fields for a declaration whose substance is
  partial. The CP-03 majority is two PKG-00 rows (DEL-00-03 and DEL-00-04
  `AB#purpose.s02`, both `PARTIAL_SLICE · PROJECT_BASELINE · RECORD`), whose
  subjects are architecture seams. So for CP-03, tier and layer also follow
  each row's own subject under C3, C5 and F8.

A difference from the batch majority is therefore not an error in itself.
I judged each row on its own gap.

---

## 1. DEL-07-05:SOW#CLM-004.r07 (W1): LEGITIMATE

Sealed values: `PARTIALLY_IMPLEMENTED · PARTIAL_SLICE · LOCAL_DESIGN · NONE ·
RECORD` (record 29 of the ledger).

Evidence at the freeze:
- **The claim.** Ratios are displayed "only when a user-supplied rule pack and
  required inputs support the check; incomplete rule input remains an explicit
  finding" (`execution/PKG-07_…/DEL-07-05_…/ScopeOfWork.md:66`). It cites the
  SOW-023 note and OPS-K-DATA/RULE.
- **The viewer never shows a ratio.** `apps/desktop/src/features/results/ResultsPanel.tsx:114`
  renders `<GoverningRatioState ratioCount={0} />` unconditionally.
- **The contract has no eligible ratio row.** In
  `fixtures/results/semantic_contract_v0_2.json`, all 60 rows carry
  `"governing_ratio_eligible": false` and none carry `true`. Examples are at
  lines 99, 113 and 127.
- **The test pins the unavailable outcome.** `apps/desktop/src/App.test.tsx:869`
  and `:885` supply an invented `user_rule_governing_ratio` row, and the test
  expects the unavailable result.
- **The Remaining item.** `DEL-07-05/_STATUS.md:8` keeps ratio producer and
  rule-pack sufficiency upstream. The viewer shows "an explicit unavailable
  state when none are present".
- **No ruling defers ratio display.** No DEC row in
  `execution/_Decomposition/SOFTWARE_DECOMP.md` (lines 598–700) defers it.
  SOW-023 (line 131) keeps ratios IN scope, noting that they "depend on rule
  pack completeness". PR #787 is a design change, not a ruling.

Reasons:
- **CP-11 applies.** "Only when supported" holds only because no ratio is
  ever shown. Not `ALIGNED` is right.
- **Cause.** `PARTIAL_SLICE` is the row's own gap cause. The blocking and
  unavailable half landed (D-41 R5 T5 PDU-008, `_STATUS.md:12`), but the
  supported-ratio display path did not. No governing ruling defers that path.
  So the majority's `DEFERRED_BY_RULING` does not fit (C7). `NOT_STARTED`
  fits less well, because part of the claim is implemented.
- **Tier and layer.** They agree with the majority, and they are right on
  their own terms under F8:
  - The boundary part (no inferred criteria, no invented ratio) holds.
  - The gap is a missing viewer feature. It does not touch the IP/data or
    rule boundary, so the tier is not `INVARIANT`.
  - `LOCAL_DESIGN · RECORD` matches corpus practice for CP-11 feature gaps
    that no ruling defers: DEL-17-02, DEL-17-07 and DEL-17-09, which are
    already `RESOLVED_PAIR`.
- **A weak point, not a disagreement.** `AuthorityNeeded OWNER` alongside
  `LOCAL_DESIGN` is permitted. The Part D definition asks for OWNER "when a
  choice is needed", and the row records such a choice. Whether a REQ that
  traces to an IN scope item should read `PROJECT_BASELINE` is not settled by
  C3. The corpus splits on it: compare DEL-14-02 r09, which is
  `PROJECT_BASELINE · BASELINE`. That is a clustering point for R3, not a
  ground to re-dispose this row.

Record: `RESOLVED_PAIR` at the sealed values.

## 2. DEL-07-05:SOW#CLM-011/REQ-07-05-005 (W1): LEGITIMATE

Sealed values: `PARTIALLY_IMPLEMENTED · PARTIAL_SLICE · LOCAL_DESIGN · NONE ·
RECORD` (record 48).

Evidence:
- **The claim.** REQ-07-05-005 says rule-pack ratios display only when user
  inputs and checksum/provenance status support them; "otherwise the ratio
  surface shall show an unavailable or blocked state with a diagnostic"
  (`DEL-07-05/ScopeOfWork.md:152`).
- **The implementation** is the same as in section 1:
  `ResultsPanel.tsx:114`, the semantic contract with no eligible row, and
  `App.test.tsx:869`/`:885`.
- **The declaration** at `ScopeOfWork.md:209` records the explicit
  unavailable state.

Reasons:
- **Same gap as section 1.** Only the unavailable branch exists, and the
  supported branch is absent. `FindingGroup FG-DEL-07-05-02` is shared
  correctly under C1.
- **Cause and tier.** `PARTIAL_SLICE` is the row's own cause, because no
  ruling defers the gap. The tier and layer match the majority and F8.
- **Explanatory text, not a diagnostic record.** The row's RemainingWork
  records that the unavailable state is explanatory text rather than a
  diagnostic record. That is a second unmet element inside the same slice.
  It does not change any field.
- **The PKG-07 verifier confirmed the ratio CP-11 treatment.**
  `W1/PKG-07/PKG-07_VERIFICATION.md:139-140` cites `ResultsPanel.tsx:114`,
  and I found nothing to the contrary.

Record: `RESOLVED_PAIR` at the sealed values.

## 3. DEL-00-07:AB#normative-requirements/REQ-07-03 (W2): CORRECT (field; not a false alignment)

Sealed values: `PARTIALLY_IMPLEMENTED · PARTIAL_SLICE · PROJECT_BASELINE ·
NONE · BASELINE` (record 8).

Right values: **Disposition** `PARTIALLY_IMPLEMENTED` · **CauseTag**
`PARTIAL_SLICE` · **AuthorityTier** `INVARIANT` · **BaselineClass** `NONE` ·
**DivergenceLayers** `SECURITY`.

- The sealed row was **not** a firm false alignment. The disposition and
  cause stand.
- This is a field correction: the tier moves from `PROJECT_BASELINE` to
  `INVARIANT`, and the layer from `BASELINE` to `SECURITY`.
- After correction the row still differs from the CP-11 majority on cause,
  tier and layer. That difference is legitimate.

Evidence at the freeze:
- **The claim.** REQ-07-03 reads "Prevent plugins and adapters from bypassing
  domain validation, rule-pack sandboxing, result envelopes, or report
  boundary controls" (`execution/PKG-00_Software Architecture Runway/1_Working/DEL-00-07_…/ArchitectureBasis.md:19`).
  It restates AB-00-07 (`execution/_Decomposition/SOFTWARE_DECOMP.md:444`).
- **The same no-bypass rule is a specification invariant.** See
  `docs/SPEC.md:37` and `docs/SPEC.md:353`: "adapters and plugins must not
  bypass schema validation, … private-data controls, … professional-boundary
  checks". The API contract names the no-bypass controls at
  `docs/TYPES.md:216`.
- **Adapter side (the landed slice).**
  - `core/adapters/framework/adapter_framework.py:507-532` is a deny-only
    dispatch gate. It returns `BLOCKED_RUNTIME_NOT_SELECTED`, and its
    docstring (line 512) says the adapter execution model and the plugin
    runtime "remain owner-held TBDs".
  - The tests are `tests/test_adapter_framework_contract.py:701` and `:754`.
- **Plugin side (the gap).** `api/api_boundary_contract.yaml:27-28` has
  `plugin_runtime: TBD` and `plugin_loading_signing_isolation: TBD`. So
  plugins cannot bypass anything only because they cannot run (CP-11).
- **No ruling defers the plugin runtime.**
  - DEC-012 (`SOFTWARE_DECOMP.md:603`) keeps the grammar, transport, formats
    and similar items as implementation-level TBDs. It does not name a plugin
    runtime.
  - OI-006 (`:574`) is an open issue, not a deferral.

Reasons:
- **Disposition and cause.** CP-11 applies, and `PARTIALLY_IMPLEMENTED` is
  right. `PARTIAL_SLICE` is the row's own cause: the adapter no-bypass slice
  landed, and the plugin runtime did not and is not deferred by a ruling. The
  majority's `DEFERRED_BY_RULING` does not fit.
- **Tier: `INVARIANT`, not `PROJECT_BASELINE`.**
  - C3 puts a claim that restates a specification or boundary invariant on
    security and privacy at `INVARIANT`.
  - This rule is stated as such at `docs/SPEC.md:37` and `:353`.
  - F8 keeps `INVARIANT` when "the gap touches the boundary's subject". The
    remaining gap, shown in the row's own RemainingWork ("once a runtime
    exists, show its no-bypass enforcement"), is the no-bypass and sandboxing
    boundary itself. It is not a stale record clause.
  - That AB-00-07 also carries the rule does not lower the tier. The more
    specific invariant subject governs under F8.
- **Layer: `SECURITY`, not `BASELINE`.**
  - The unmet element is sandboxing and no-bypass enforcement for plugins,
    which is a security subject (C5).
  - `BASELINE` would mean the accepted baseline itself diverges. Here AB-00-07
    is not contradicted; its enforcement for plugins is only unbuilt.
- **Cross-wave consistency.** The same substance under the same basis
  (AB-00-07) is already resolved at `PARTIAL_SLICE · INVARIANT · NONE ·
  SECURITY` on the three DEL-06-02 rows. Those are CLM-006.r05 and
  REQ-06-02-010 twice, recorded in `ALL_WAVES_RESOLUTIONS_COMBINED.csv`
  records 592–594, from `W3/CROSS_PACKAGE/W3_CROSS_PACKAGE_VERIFICATION.md`
  §1a–1c. Leaving REQ-07-03 at `PROJECT_BASELINE · BASELINE` would route one
  boundary under two tiers.
- **Disclosure.** The PKG-00 verifier sampled this row (`NA50`,
  `W2/PKG-00/PKG-00_VERIFICATION.md:108`) and did not contest it. I disagree,
  for the C3 and F8 reasons above.

Record: `RESOLVED_PAIR` at the corrected values, with a field correction of
AuthorityTier `PROJECT_BASELINE` → `INVARIANT` and DivergenceLayers
`BASELINE` → `SECURITY`. `AuthorityNeeded OWNER` stands.

## 4. DEL-05-04:SOW#CLM-008 (W3): LEGITIMATE

Sealed values: `PARTIALLY_IMPLEMENTED · PARTIAL_SLICE · INVARIANT · NONE ·
CLAIMS` (record 20).

Evidence at the freeze:
- **The declaration** (`execution/PKG-05_…/DEL-05-04_…/ScopeOfWork.md:121`)
  says status semantics are implemented, "External acceptance remains
  human-owned, and stale-hash negative behavior remains held where explicitly
  recorded".
- **The invariant.** OPS-K-AUTH-2 (`docs/CONTRACT.md:30`) reads "Human
  acceptance records, if used, bind to specific model/rule/report hashes and
  do not survive content changes without re-review". REQ-05-04-014
  (`ScopeOfWork.md:156`) asks for negative tests against "stale
  human-acceptance reuse after content changes".
- **The landed slice.** The schemas carry the hash binding:
  - `schemas/analysis_status.schema.yaml:162` and `:203` (`bound_hashes`,
    `minItems: 1`);
  - `schemas/project_persistence.schema.yaml:506` and `:527`
    (`invalidates_on_hash_change`).
- **The gap.** Nothing creates or enforces these records:
  - `core/project_persistence/service.py:127` seeds
    `"human_acceptance_refs": []`;
  - the Rust report tests carry `human_acceptance_ref: None`
    (`core/reporting/report_package/tests/container.rs:51`);
  - no runtime check or negative test covers invalidation after a hash
    change.
- **The human approval workflow is open.** OI-007
  (`SOFTWARE_DECOMP.md:575`) records it as TBD.
- **The PKG-05 verifier agreed.** It reached the same reading for all nine
  FG-DEL-05-04-01 rows (`W3/PKG-05/PKG-05_VERIFICATION.md:193-209`).

Reasons:
- **The pattern.** CP-03 applies: this is its own `DECLARED_STATE` row, and
  its siblings are unaffected. CP-03 prescribes no tier or layer for a partial
  declaration, so C3, C5 and F8 decide per row.
- **Disposition.** `PARTIALLY_IMPLEMENTED` describes the declared subject
  (C6(b)). The row's own RemainingWork records an unmet element, so it cannot
  be `ALIGNED` (F1).
- **Tier: `INVARIANT` is right.** The subject is the professional boundary
  (OPS-K-AUTH-2), a covered C3 subject. The gap, reuse of stale acceptance, is
  that boundary's subject, so F8 keeps `INVARIANT`. The majority rows are
  `PROJECT_BASELINE` because their subjects are architecture seams (command
  and query envelopes, persistence versioning). This is a difference of
  subject, not an inconsistency.
- **Layer: `CLAIMS` is right.** C5 has no professional-boundary layer, as the
  PKG-05 verifier noted at line 521. `CLAIMS` is the nearest fit. It is also
  the layer already resolved for the same subject on DEL-15-04 CLM-012.r03
  and CLM-010.r07 (`INVARIANT · CLAIMS`, `RESOLVED_PAIR`).
- **Cause.** `PARTIAL_SLICE` agrees with the CP-03 majority and fits C7: a
  schema-level slice landed, and runtime enforcement did not. OI-007 is an
  open issue, not a ruling.
- **Observation for R3 (no field change).**
  - DEL-15-04 disposes the same acceptance gap as `DEFERRED_BY_RULING`, via
    DEC-081 (`SOFTWARE_DECOMP.md:672`). DEC-081 reserves `ENGINEER_ACCEPTED`
    with "no acceptance workflow created".
  - The two causes differ because the rows differ. DEL-05-04 has a landed
    schema slice, and DEL-15-04 has no representation at all.
  - Both are defensible under C7. R3 may harmonize the cluster.
- **Observation on the Notes (no field change).**
  - In D-41 declarations, "remains held" means a residual on hold. Examples
    of this usage are in other deliverables' ScopeOfWork files ("R10 remains
    held", "PDU-011 remains held because …").
  - So the declaration's third statement accurately records the stale-hash
    negative as open.
  - The worker's Notes can be read as calling that statement inaccurate.
    Either way, the subject is partial, and the fields are unchanged.

Record: `RESOLVED_PAIR` at the sealed values.

## 5. DEL-05-04:SOW#CLM-023 (W3): LEGITIMATE

Sealed values: `PARTIALLY_IMPLEMENTED · PARTIAL_SLICE · INVARIANT · NONE ·
CLAIMS` (record 62).

Evidence:
- `DEL-05-04/ScopeOfWork.md:294` is byte-identical to the CLM-008
  declaration text at `:121`.
- Otherwise the evidence is as in section 4.

Reasons:
- The row writes `DUPLICATE_OF DEL-05-04:SOW#CLM-008` and takes the same
  disposition and fields, as C1 (duplicates) requires. Every reason in
  section 4 applies.

Record: `RESOLVED_PAIR` at the sealed values.

---

## Summary

| # | Key | Decision | Values to record | Firm false alignment |
|---|---|---|---|---|
| 1 | DEL-07-05:SOW#CLM-004.r07 | LEGITIMATE | sealed | no |
| 2 | DEL-07-05:SOW#CLM-011/REQ-07-05-005 | LEGITIMATE | sealed | no |
| 3 | DEL-00-07:AB#normative-requirements/REQ-07-03 | CORRECT (field) | PARTIALLY_IMPLEMENTED · PARTIAL_SLICE · INVARIANT · NONE · SECURITY | no |
| 4 | DEL-05-04:SOW#CLM-008 | LEGITIMATE | sealed | no |
| 5 | DEL-05-04:SOW#CLM-023 | LEGITIMATE | sealed | no |

What the owner should know:
- **REQ-07-03 changes tier.** It moves from `PROJECT_BASELINE` to
  `INVARIANT`, which changes the tier it routes under; the disposition does
  not change. This aligns the PKG-00 statement of the plugin/adapter
  no-bypass rule with the three DEL-06-02 rows already corrected the same
  way.
- **The conventions leave two points open, for R3.**
  - Whether a deliverable REQ that traces to an IN scope item reads
    `LOCAL_DESIGN` or `PROJECT_BASELINE`. DEL-07-05 and DEL-17-0x use the
    first; DEL-14-02 r09 uses the second.
  - Whether the absent human-acceptance workflow is `DEFERRED_BY_RULING`
    (DEC-081) or `PARTIAL_SLICE`. DEL-15-04 uses the first; DEL-05-04 uses
    the second.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
