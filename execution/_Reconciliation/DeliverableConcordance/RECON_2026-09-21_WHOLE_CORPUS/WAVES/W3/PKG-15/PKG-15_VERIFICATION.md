VERDICT: RERUN DEL-15-02

# PKG-15 verification (wave W3, sampling STANDARD)

This is the fresh, evidence-only package verifier for run
`HELP-HUMAN-PIPING-20260921-RECONCILIATION`. My parent is HELP_HUMAN Agent 0.

- **Brief.** `briefs/R2-VERIFIER_brief.md`. I checked its SHA-256
  (`47fb3c52…5b00`) and it matches.
- **Evidence checkout.** `00115c71931bcae79909602d653740d3bb72dfa1`. This is
  the read-only freeze; I confirmed its HEAD.
- **Rules applied.** `CONVENTIONS.md` Parts A to F, with Part F judged like
  any other rule; `CANONICAL_SITUATIONS.md`; and
  `AUTHORITY_AND_SOURCE_RELIABILITY_MAP.md`.
- **Manager records.** I read the PKG-15 manager's transcripts and the G1
  worker's verbatim return only for the flags they raise. I did not use them
  as evidence for any claim.

These are agent verification judgments, not owner rulings. They do not state
or imply any release, approval, compliance or certification (F-PIP-2; claims
taxonomy per DEC-081).

## 0. Integrity and mechanical checks

- **Seals.** For all four ledgers, the forward SHA-256 I recomputed matches
  the `_SEAL.txt` line, the worker return and the manager return. The reverse
  and notes hashes I recomputed also match the worker return.
- **Single mode.** I ran `validate_ledger_v2.py` against the freeze with
  `--forward --reverse --inventory ROUTING/PKG-15_capabilities.csv
  --notes-gap`. All four ledgers pass with 0 findings:

  | Deliverable | Forward rows | Required keys | Canonical rows |
  |---|---|---|---|
  | DEL-15-01 | 93 | 68 | 9 |
  | DEL-15-02 | 91 | 70 | 9 |
  | DEL-15-03 | 97 | 75 | 9 |
  | DEL-15-04 | 87 | 56 | 9 |

- **Batch mode.** `--batch` over the four forward ledgers passes with 0
  consistency findings, the same result the manager got.
  `WAVES/W3/RESOLUTIONS.csv` does not exist yet, and
  `RESOLUTIONS_DRAFT_PART1.csv` has no PKG-15 rows.
- **Keyed canonical rows.** The validator checks conformance on 100% of the
  CS-01, 02, 03, 04, 06 and 07 rows. All conform.
- **Reverse files.** Each has 290 capability rows, which matches the routing
  file.
- **Lifecycle.** No deliverable is ISSUED. No row carries `PROTECTED_CHECK`
  or `FROZEN_CONTRACT`, and there are no UNKNOWN, AUTHORITY_CONFLICT,
  ACCEPTED_DIVERGENCE or LIFECYCLE_REASSESSMENT_REQUIRED rows.

## Sampling (deterministic)

In each deliverable and each class, I sorted the candidate keys by the
SHA-256 of the full claim key and took the lowest fraction, rounded up. I
used the STANDARD rates:

| Class | Rate |
|---|---|
| C100: INVARIANT tier, UNKNOWN, ACCEPTED_DIVERGENCE, AUTHORITY_CONFLICT, LIFECYCLE_REASSESSMENT_REQUIRED, PROTECTED_CHECK, FROZEN_CONTRACT | 100% |
| SH: `SharedTextCount > 1` (a minted `.sNN` key takes its parent's count) | 100% |
| F2/4: ALIGNED with `GAP_WORDING_CHECKED:` or `OPEN_ACTION:` | 100% |
| F7: ALIGNED with `PRODUCT_CALLER: NONE` | 25% |
| NA: other non-aligned rows | 25% |
| AN: ALIGNED normative rows | 20% |
| Q: structural and other quiet rows | 10% |

- Each row falls in the first class that matches, in the order above.
- **Disclosed departure (same as earlier waves).** I selected AN rows by hash
  only. I did not weight them toward LOW or MEDIUM confidence. Every AN
  candidate in this package is HIGH confidence, so weighting would not have
  changed the selection.
- In total I sampled **128 of 368** forward rows.
- Beyond the sample, I read every row of all four ledgers in summary form. I
  also reviewed the rows the worker flagged for verifiers: FG-DEL-15-02-02,
  FG-DEL-15-03-02, FG-DEL-15-03-01, FG-DEL-15-04-01 and R7. §4.2 reports the
  findings outside the sample. They are not counted.

## 1. Package-level result

- **Firm disagreements on sampled rows: 1.** It is DEL-15-02
  `SOW#CLM-005`, in the F2/4 class, which is sampled at 100%.
- **Weak disagreements on sampled rows: 1.** It is DEL-15-04 `OUT-001`, also
  in the F2/4 class.
- **Field disagreements on sampled rows: 1.** It is DEL-15-03 `AC-001` (tier).
- **Package firm false-alignment rate: 1 / 27 sampled ALIGNED normative rows
  = 3.7%.** The gate is 5% or less, so it is met.
- **Rerun test.**
  - DEL-15-02's firm error rate is 1 of 35 sampled rows (2.9%), under the
    10% threshold.
  - However, the firm error falls in a 100%-sampled class (F2/4), and
    correcting it moves the row from a quiet row to tier `INVARIANT`. That
    meets the second rerun clause as written, so the verdict is RERUN
    DEL-15-02.
  - The same reading was applied to W2 PKG-03 DEL-03-07 (D1).
  - §7 says why Agent 0 may prefer to settle this through `RESOLUTIONS.csv`
    (F6) instead of a full rerun.
  - DEL-15-01, DEL-15-03 and DEL-15-04 meet no rerun condition.

## 2. Per-deliverable tables

Cells read "total / sampled". The "False-align" column is the firm
false-alignment rate among sampled ALIGNED rows sealed as REQUIREMENT,
ACCEPTANCE or EXCLUSION, drawn from any class.

| Deliverable | Rows | C100 | SH | F2/4 | F7 | NA | AN | Q | Sampled | Firm | Weak | Field | False-align |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| DEL-15-01 | 93 | 0 | 10/10 | 2/2 | 31/8 | 22/6 | 4/1 | 24/3 | 30 | 0 | 0 | 0 | 0/8 = 0% |
| DEL-15-02 | 91 | 4/4 | 12/12 | 2/2 | 30/8 | 19/5 | 1/1 | 23/3 | 35 | 1 | 0 (+1 out of sample) | 0 | 1/9 = 11.1% |
| DEL-15-03 | 97 | 6/6 | 9/9 | 5/5 | 15/4 | 34/9 | 2/1 | 26/3 | 37 | 0 | 0 | 1 (+3 rows out of sample) | 0/5 = 0% |
| DEL-15-04 | 87 | 2/2 | 7/7 | 1/1 | 27/7 | 19/5 | 2/1 | 29/3 | 26 | 0 | 1 (+1 out of sample) | 0 | 0/5 = 0% |

DEL-15-02's 11.1% sits on a denominator of 9. The package rate of 3.7% is
the gate input.

Sampled keys (claim-key suffixes after `<DEL>:`):

- **DEL-15-01.**
  - SH: CONTEXT #scope-coverage, #scope-detail, #objective-support,
    #package-reference, #decomposition-reference,
    #architecture-basis-injection (with .s01 and .s02),
    #sca-002-control-surface-refresh-note, #preparation-notes.
  - F2/4: SOW AC-001; CLM-029.
  - F7: CONTEXT#description; CLM-024; purpose OUT-001; CLM-025; CLM-006;
    CLM-011.r09; CONTEXT#d-41-e1-canonicalization-vocabulary-boundary;
    CLM-006.r09.
  - NA: CLM-004; matrix OUT-001; VER-001; CLM-013/V-05; CONTEXT; CLM-016.
  - AN: CLM-013/V-06.
  - Q: CONTEXT#anticipated-artifacts; CLM-022; CLM-007.
- **DEL-15-02.**
  - C100: CLM-011.r05; CLM-011.r07; AC-001; CLM-021.r03.
  - SH: output-and-evaluation-matrix (with OUT-001); CONTEXT
    #scope-coverage, #scope-detail, #objective-support, #package-reference,
    #decomposition-reference, #architecture-basis-injection (with .s01 and
    .s02), #sca-002-control-surface-refresh-note, #preparation-notes.
  - F2/4: CLM-005; CLM-011.r04.
  - F7: CLM-018; CLM-011.r06; CLM-013/V-003; CLM-013/V-004; CLM-011.r09;
    CLM-011.r01; CLM-030; CLM-011.r10.
  - NA: CONTEXT; STATUS; CLM-017; CLM-020; CLM-021.r06.
  - AN: CLM-021.r02.
  - Q: CONTEXT#context-budget-qa; CLM-011; STATUS#history.
- **DEL-15-03.**
  - C100: CLM-005; REQ-004; REQ-005; CLM-013.r02; CLM-020.r03; CLM-025.
  - SH: output-and-evaluation-matrix (with OUT-001); CONTEXT
    #scope-coverage, #scope-detail, #objective-support, #package-reference,
    #decomposition-reference, #sca-002-control-surface-refresh-note,
    #preparation-notes.
  - F2/4: CLM-020.r04; TBD-001; TBD-002; TBD-003; TBD-004.
  - F7: REQ-006; REQ-003; CLM-006.r03; CLM-027.
  - NA: CLM-024; CLM-022; REQ-001; CLM-008; AC-001; CLM-002; CLM-031;
    CONTEXT#description; CLM-013.r03.
  - AN: REQ-007.
  - Q: CONTEXT#register-references; CLM-014; CLM-009.
- **DEL-15-04.**
  - C100: CLM-010.r07; CLM-012.r03.
  - SH: CONTEXT #package-reference, #decomposition-reference,
    #architecture-basis-injection (with .s01 and .s02),
    #sca-002-control-surface-refresh-note, #preparation-notes.
  - F2/4: purpose OUT-001.
  - F7: CLM-015; CLM-004.r03; CLM-010.r01; CLM-009; CLM-004.r07;
    CLM-012.r04; CLM-013.
  - NA: CLM-006; CLM-012.r01; CLM-017; STATUS#remaining/R01; matrix
    OUT-001.
  - AN: CLM-012.r05.
  - Q: CONTEXT#context-budget-qa; CLM-008; governing-values-and-decisions-axiology.

## 3. What was checked against the freeze (selected)

- **DEL-15-01 schema** (`schemas/handoff_package.schema.json`).
  - It is Draft 2020-12 and strict. `HandoffPackageManifest` requires all 21
    SOW-074 slots.
  - The `Checksum.canonicalization` enum carries
    `deterministic_sorted_compact_json_payload_hash` and the legacy JCS labels,
    with the "not RFC 8785" description.
  - `tests/test_handoff_package_schema.py` uses `Draft202012Validator` and
    has the two label tests (L140, L149).
  - **FG-DEL-15-01-01 confirmed.** `Checksum` has no `provenance` property.
    `UnitsManifestRef` has no entries or diagnostics; it has per-quantity
    unit fields. `EntityIdManifest` has typed ID lists, not
    `entity_id`/`entity_kind`/`source_ref` records. The SOW slot text
    (L92–95, L170–171) therefore misdescribes the schema.
    `IMPLEMENTED_DIFFERENTLY · OTHER · LOCAL_DESIGN` is right. The schema did
    not advance, so `DOC_BEHIND_CODE` would be wrong.
  - F7 applies correctly: the claims are about the contract itself, so a
    schema used only by tests satisfies them.
- **DEL-15-02 builder** (`core/handoff/target_mapping/contract.py`).
  - `_mapping_record` (L230–243) fills an absent `mapping_status` with
    `"mapped"`, and an absent `mapping_kind` and `value_kind` with
    `"metadata"`.
  - `build_target_mapping_contract` (L82–106) runs
    `diagnostics_for_target_mapping_contract` on the normalized contract.
    The raw input is never checked. So the missing-field case never reaches
    `_mapping_diagnostics`, whose unit check (L271) keys on
    `UNIT_VALUE_KINDS`.
  - **FG-DEL-15-02-02 is therefore confirmed by code reading.** One
    correction to the worker's wording: the absence of `mapping_kind` is also
    silent, because the `TM-MAPPING-KIND-UNSUPPORTED` check sees the
    defaulted value. No test covers the omitted-field case.
  - Confidence stays MEDIUM, because no test was run (A6).
- **DEL-15-03 exporter** (`core/handoff/exporter/workflow.py`).
  - L129–132 route the output through `control_route_export` with
    `route_id="REXC-CORE-001"` and `export_context="downstream_tool"`.
  - `tests/test_handoff_export_workflow.py` asserts `[REDACTED]` for the
    model hash, `coordinate_unit`, entity IDs, library refs, target-mapping
    metadata, assumption IDs, warning codes and unsupported-record IDs
    (L93–97, L403–422, L437–441). It asserts the same for every
    professional-boundary value (L685–686).
  - The test validates the input package and mapping against their schemas
    (L378–379). It does not validate the export output.
  - **FG-DEL-15-03-01 and FG-DEL-15-03-02 are confirmed.**
- **DEL-15-04.**
  - The schema is strict and has no comparison-link kind.
  - `ExternalProverBoundaryPanel.tsx` L213 adds `unit_policy_evidence` to its
    packet, which the strict schema does not allow. **FG-DEL-15-04-01 is
    confirmed.**
  - The CP-11 treatment of R7 and CLM-012.r03 matches DEC-081's reserved
    acceptance vocabulary, with no acceptance workflow.
- **Cross-package shared bodies.**
  - PKG-15's `CONTEXT#decomposition-reference` and
    `#architecture-basis-injection` bodies are shared with DEL-07-08,
    DEL-08-06, DEL-13-0x, DEL-14-0x and DEL-16-0x. Every sealed counterpart
    carries the same disposition, cause, tier and layer.
  - The DEL-15-02 and DEL-15-03 output matrix and its OUT-001 agree.

## 4. Disagreements

### 4.1 Sampled rows

**D1 — `DEL-15-02:SOW#CLM-005`** (F2/4 class; REQUIREMENT, Conditions).
This is **firm**.

- **The row says.** ALIGNED. Notes say the builder blocks unit-bearing
  mappings without unit metadata and emits the context and privacy
  diagnostics. The row ends with `GAP_WORDING_CHECKED: the wording names
  diagnostics the builder emits, not unmet elements`.
- **What I found.** The block's two leading conditions read as follows (SOW
  L76–77):
  - "Exports must remain unit-aware and dimensionally checked"
    (OPS-K-UNIT-1).
  - "Missing … values are explicit findings, not silent defaults"
    (OPS-K-DATA-2).

  These are the same substance, with the same sources, as R005 and R007. The
  worker disposes R005 and R007 IMPLEMENTED_DIFFERENTLY · POSSIBLE_DEFECT ·
  INVARIANT (FG-DEL-15-02-02). I confirmed that finding in
  `core/handoff/target_mapping/contract.py` L230–243 and L82–106 (§3). Its
  effects:
  - A record that omits `value_kind` skips the unit check.
  - A record that omits `mapping_status` is emitted as `mapped`.
  - In both cases no diagnostic is raised.

  The cited evidence therefore does not meet the Units and Missing-values
  conditions. The row is ALIGNED only because its Notes leave out the gap
  the ledger records elsewhere. C6(i) and F1 bar that, and the wording clause
  does not cure an unmet element.
- **Right values.** IMPLEMENTED_DIFFERENTLY · POSSIBLE_DEFECT · INVARIANT ·
  NONE · BASELINE · MEDIUM, with `FindingGroup` FG-DEL-15-02-02. The
  `AuthorityNeeded` value should match R005 and R007 (the ledger uses NO).
  The other four conditions (provenance, private data, claim fence, handoff
  role) hold, and Notes should say so.

**W1 — `DEL-15-04:SOW#purpose-and-objective-traceability/OUT-001`** (F2/4
class; REQUIREMENT). This is **weak**.

- **The row says.** ALIGNED. Its Notes record "Comparison links are generic
  only (FG-DEL-15-04-03 on the posture rows)". The row ends with
  `GAP_WORDING_CHECKED`, which says the comparison-link detail is judged on
  the posture rows.
- **What I found.** The claim reads "…for descriptive references,
  attachments, and handoff or comparison links is produced" (SOW L16).
  - Handoff, mapping, export and model-state links exist in the schema and
    builder. No comparison-link kind exists.
  - The worker's sibling rows CLM-004.r02, CLM-006 and CLM-012.r01 dispose
    the missing comparison link as PARTIALLY_IMPLEMENTED (FG-DEL-15-04-03).
  - Two readings survive:
    - Read "handoff or comparison links" as alternatives. Handoff links
      exist, so the output is met.
    - Read it as naming both link kinds. Then the row records an unmet
      element in its own Notes, and F1 applies whatever the justification
      given.
  - The conventions do not settle this reading. The justification given
    ("judged on the posture rows") is exactly the reasoning F1 rejects.
- **Preferred values.** PARTIALLY_IMPLEMENTED · PARTIAL_SLICE · LOCAL_DESIGN
  · NONE · RECORD, with FG-DEL-15-04-03 and `AuthorityNeeded` NO. This would
  make it consistent with CLM-012.r01. Neither reading changes the owner
  routing.

**F1 — `DEL-15-03:SOW#completion-and-reliance-basis-epistemology/AC-001`**
(NA class). This is a **field** disagreement.

- **The row says.** IMPLEMENTED_DIFFERENTLY · POSSIBLE_DEFECT ·
  PROJECT_BASELINE · BASELINE (FG-DEL-15-03-02).
- **What I found.** The disposition is right. However, AC-001 lists "units"
  among the preserved contents, and the row's own Notes say content
  preservation fails in the redacted output. That includes
  `units_manifest` values (test L406).
  - Under F8, `INVARIANT` applies when the gap touches the boundary's
    subject. The worker applies this reading to CLM-005 and CLM-025, which
    also mix unit and non-unit elements.
- **Right value.** Tier `INVARIANT`. The owner routing is unchanged, because
  the row is already `AuthorityNeeded` OWNER and in the R4 finding group.

### 4.2 Outside the sample (not counted)

- **`DEL-15-02:SOW#CLM-027`** (F7 class; REQUIREMENT, Principles).
  - It is ALIGNED, but its "Explicit non-support … not hidden by defaults"
    and "Unit safety" principles (SOW L365–366) carry the same substance as
    D1.
  - This is a firm-equivalent finding. It would take the D1 values and join
    FG-DEL-15-02-02.
- **`DEL-15-04:SOW#completion-and-reliance-basis-epistemology/AC-001`** (F7
  class; ACCEPTANCE). This is weak.
  - It is ALIGNED, but it lists "external human-owned hash-bound acceptance
    only" (SOW L183). The worker treats the same element under CP-11 on R7
    and CLM-012.r03 (PARTIALLY_IMPLEMENTED or DOCUMENTED_UNIMPLEMENTED ·
    DEFERRED_BY_RULING · INVARIANT · CLAIMS).
  - Read as a pure restriction ("no acceptance except external"), AC-001
    holds. Read as CP-11 reads R7, it is PARTIALLY_IMPLEMENTED ·
    DEFERRED_BY_RULING · INVARIANT · CLAIMS in FG-DEL-15-04-02. The worker's
    own R7 treatment points to the second reading.
- **`DEL-15-03` REQ-002, CLM-006.r04 and CLM-019.** These are field
  disagreements, for the same reason as F1.
  - Each lists the units manifest or unit preservation among the preserved
    contents, yet carries tier PROJECT_BASELINE.
  - Under F8 they would be INVARIANT, like REQ-004 and CLM-005.
  - CLM-013.r03 (hash, provenance and assumptions only) is correctly
    PROJECT_BASELINE.
- **Observation (not a disagreement).**
  - FG-DEL-15-02-02 is INVARIANT with `AuthorityNeeded` NO. FG-DEL-15-03-02
    is INVARIANT with OWNER.
  - The difference is defensible. A code fix with a negative test needs no
    decision, whereas the PR #307 redaction needs an intent decision.
  - R3 should still read the two as distinct kinds of item.

## 5. Batch consistency and shared situations

- **Batch mode.** PASS, with 0 findings across the four forward ledgers.
- **Shared situations.** There is no shared-situation conflict. The keyed CS
  rows, CP-02, CP-03, CP-04, CP-05 and CP-09 rows are uniform across the four
  deliverables.
  - Every CS-04 row carries the same `.s01` (PKG-00 `SEMANTIC_READY`) and
    `.s02` (since-ruled TBDs) split.
  - DEL-15-03 adds `.s03` for its own D-41 T2A "Resolved Baseline" clause,
    which is correctly its own body.
- **Cross-package shared bodies.** They agree with every sealed counterpart
  in W1 and W3 (§3).
- **Finding-group tiers.** FG-DEL-15-03-02 has a tier split inside the group,
  INVARIANT versus PROJECT_BASELINE. F8 intends this, but §4.1 F1 and §4.2
  list the members whose gap includes units and so should be INVARIANT.

## 6. Reverse pass

- **Checks made.**
  - 100% of the non-NOT_MINE answers: CLAIMED_BY 7 and PARTIAL 2. There are
    no COVERS, CONSTRAINS or UNKEYED answers.
  - A 10% hash-selected NOT_MINE sample: 116 of 1,151 (29 per deliverable).
  - F5: all 14 NOT_MINE answers whose EntryPoints hit a path the
    deliverable's forward ledger cites, which exceeds the 20% required.
- **Findings.**
  - All 9 non-NOT_MINE answers are right. The CLAIMED_BY answers are:
    - handoff schema: DEL-15-01;
    - target-mapping builder and schema: DEL-15-02;
    - exporter: DEL-15-03;
    - authority-term screen, prover metadata builder and prover schema:
      DEL-15-04.
  - The PARTIAL answers are HandoffPanel (DEL-15-03) and
    ExternalProverBoundaryPanel (DEL-15-04). Each names the part that belongs
    elsewhere (PKG-17 profile; the export unit-disclosure area).
  - Every F5 reason addresses its capability. The reasons are specific and
    cite the row or the exclusion concerned, for example "DEL-15-01 CLM-010
    explicitly excludes the downstream export workflow".
  - No sampled NOT_MINE looks like a missed claim.
- **Capabilities claimed by more than one PKG-15 deliverable.** None. The
  shared authority-term screen (RC-15-0001) is claimed only by DEL-15-04.
  DEL-15-03 answers NOT_MINE with a consumer reason, which is consistent.
- **Suspected missed claims.** None.
  - I listed every routed capability whose EntryPoints or text touch
    handoff, prover, target mapping, export or redaction (70 capabilities).
  - Apart from the nine claimed, the rest belong to PKG-17 export targets,
    to reporting, or to security redaction.
- **Anchored answers.** No sign of anchoring.
  - Routing is resolved through `ROUTING_SAMPLE/SAMPLE_MANIFEST.csv`.
  - SAMPLE-routed rows: 148 answers, all NOT_MINE (0% non-NOT_MINE).
  - AREA rows: 9 of 1,012 non-NOT_MINE (0.9%).
  - The rates are comparable. As the brief notes, many SAMPLE rows are
    obviously foreign by their paths, so this comparison is weak evidence of
    independence.

## 7. What the owner must see

1. **DEL-15-02 rerun trigger (D1).**
   - The rule as written requires a rerun, because a firm error in a
     100%-sampled class moves a row from quiet to INVARIANT.
   - The fix is narrow. `CLM-005`, and on the same evidence `CLM-027`, join
     FG-DEL-15-02-02 with the values in §4.1.
   - Agent 0 may prefer to record D1 and CLM-027 in `WAVES/W3/RESOLUTIONS.csv`
     (F6) instead of a fresh worker. That is the same choice offered for W2
     PKG-03 D1. The rest of DEL-15-02 is sound.
2. **Export redaction since PR #307 (FG-DEL-15-03-02; INVARIANT and
   PROJECT_BASELINE, owner decision).** The downstream handoff exporter
   redacts to `[REDACTED]` almost everything the scope of work requires it to
   preserve, including for invented public data:
   - the model hash and unit values;
   - entity IDs, references and target-mapping metadata;
   - assumption IDs, warning codes and even professional-boundary flags.

   Only provenance and diagnostics survive. A merged PR does not amend a
   baseline or invariant claim (A2), so R4 must decide one of two things:
   either the redaction classification is over-broad, or the deliverable
   catches up to redaction-by-default.
3. **Silent defaults in the target-mapping builder (FG-DEL-15-02-02;
   INVARIANT; possible defect, confirmed by code reading, not by a test).**
   - An omitted `mapping_status` becomes `mapped`.
   - An omitted `value_kind` or `mapping_kind` becomes `metadata`. The unit
     check is then skipped.
   - No diagnostic is raised in any of these cases.
4. **No product path emits schema-compliant PKG-15 artifacts (F7).**
   - No product code calls any `core/handoff` module.
   - HandoffPanel's export does not follow `handoff_package.schema.json`
     (FG-DEL-15-03-01).
   - The desktop external-prover packet adds `unit_policy_evidence`, which
     its strict schema does not allow (FG-DEL-15-04-01).

   The contract and engine claims are met. The product claims are partial
   slices.
5. **Hash-bound human acceptance** (DEL-15-04 R7 and CLM-012.r03; CP-11,
   deferred by DEC-081). This is correctly left non-aligned. AC-001 should
   probably follow it (§4.2).
6. **Convention gap.** No `DivergenceLayers` value fits a pure
   product-implementation gap. The worker used RECORD for FG-DEL-15-03-01 and
   FG-DEL-15-04-01, and said so in its notes. R3 or the owner may want a
   ruling, because RECORD is defined as "no protected layer affected".

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
