VERDICT: ACCEPT WITH CONTESTED ROWS

# PKG-08 verification (wave W3, sampling STANDARD)

Fresh, evidence-only package verifier for run
`HELP-HUMAN-PIPING-20260921-RECONCILIATION`. Parent: HELP_HUMAN Agent 0.
Brief: `briefs/R2-VERIFIER_brief.md` (SHA-256 `47fb3c52…5b00`, checked and
matching). Evidence checkout: `00115c71931bcae79909602d653740d3bb72dfa1`
(freeze, read-only; `git rev-parse HEAD` confirmed). Rules applied:
`CONVENTIONS.md` (Parts A to F, including F1 to F8), `CANONICAL_SITUATIONS.md`,
`AUTHORITY_AND_SOURCE_RELIABILITY_MAP.md`. The PKG-08 manager's return,
validator transcripts and the worker notebooks were read for their flags
only. They were not used as evidence for any claim.

These are agent verification judgments. They are not owner rulings, and they
do not state or imply any release, approval, compliance or certification
(F-PIP-2; claims taxonomy per DEC-081).

## 0. Integrity and mechanical checks

- **Seals.** For all six ledgers, the forward SHA-256 I recomputed matches
  the `_SEAL.txt` line and the manager's return. The six reverse hashes also
  match the return.
- **Single mode.** `validate_ledger_v2.py` in single mode with `--forward
  --reverse --inventory ROUTING/PKG-08_capabilities.csv --notes-gap`, repo
  root = freeze. All six pass with 0 findings:

  | Deliverable | Forward rows | Required keys | Canonical rows |
  |---|---|---|---|
  | DEL-08-01 | 106 | 75 | 7 |
  | DEL-08-02 | 97 | 62 | 7 |
  | DEL-08-03 | 94 | 75 | 7 |
  | DEL-08-04 | 117 | 64 | 7 |
  | DEL-08-05 | 136 | 79 | 7 |
  | DEL-08-06 | 117 | 63 | 9 |

- **Batch mode.** `--batch` over the six forward ledgers: PASS, 0
  consistency findings (same as the manager). No `WAVES/W3/RESOLUTIONS.csv`
  exists.
- **Keyed canonical rows (CS-01, 02, 03, 04, 06).** Conformance is checked by
  the validator on 100%. All conform. PKG-08 has no CS-06 DRIFT rows (the
  DRIFT set is PKG-01, 02 and 07 only); all six Package Reference rows are
  CS-06 OK.
- No deliverable is ISSUED (all six are IN_PROGRESS in
  `DELIVERABLE_INVENTORY.csv`), so no all-rows-100% deliverable applies.

## Sampling (deterministic)

Each class's candidate keys were sorted by SHA-256 of the full claim key and
the lowest fraction taken, rounded up. Each row falls in the first class that
matches, in this order:

| Code | Class | Rate |
|---|---|---|
| C100 | `AuthorityTier=INVARIANT`, or disposition ACCEPTED_DIVERGENCE, AUTHORITY_CONFLICT, UNKNOWN, LIFECYCLE_REASSESSMENT_REQUIRED, or BaselineClass PROTECTED_CHECK / FROZEN_CONTRACT | 100% |
| SH | `SharedTextCount > 1` in `CLAIM_KEYS_V2.csv` (minted `.sNN` takes its parent's count) | 100% |
| F2/4 | ALIGNED with `GAP_WORDING_CHECKED:` or `OPEN_ACTION:` | 100% |
| F7 | ALIGNED with `PRODUCT_CALLER: NONE` | 25% |
| NA | Other non-aligned rows | 25% |
| AN | ALIGNED normative rows (REQUIREMENT, ACCEPTANCE, EXCLUSION) | 20% |
| Q | Structural, inherited canonical and other quiet rows | 10% |

Disclosed departures:

- **AN weighting.** The AN draw is by hash only. To weight toward
  lower-confidence rows I reviewed the full list of 30 ALIGNED normative rows
  at MEDIUM confidence or with `NONE` verification class; 13 of them are in
  the sample (by hash, by an earlier class, or as a flag-driven addition).
- **Flag-driven additions (5 rows, not by hash).** Rows the workers flagged
  for the verifier: DEL-08-04 CLM-011.r12 and CLM-025.r04; the Remaining rows
  DEL-08-01 R01, DEL-08-04 R01 and DEL-08-06 R01. They are counted in the
  sample. G1's two flagged rows (R-08-01-009, CLM-004.r05) were already
  sampled.

In total, 192 of 667 forward rows were sampled. Beyond the sample, I read
every ALIGNED row whose Notes defer or mention a gap (45 rows); findings
outside the sample are in §4.2.

## 1. Package-level result

- **Firm disagreements on sampled rows: 2** (DEL-08-02, DEL-08-05; both F1).
- **Weak disagreements on sampled rows: 5.**
- **Field disagreements on sampled rows: 1.**
- **Package firm false-alignment rate: 2 / 45 = 4.4%** (gate ≤ 5%: met,
  narrowly). The denominator is the 44 sampled ALIGNED rows sealed as
  REQUIREMENT, ACCEPTANCE or EXCLUSION, plus DEL-08-05 `CONTEXT#description`,
  which is sealed DECLARED_STATE but is an implementation requirement ("Implement
  checks that prevent …") and is typed REQUIREMENT in DEL-08-01/02/03. On the
  sealed claim types alone the rate is 1 / 44 = 2.3%.
- **No deliverable meets a rerun condition.** No deliverable's firm error rate
  exceeds 10% (highest: DEL-08-02, 1 / 28 = 3.6%). Neither firm error was
  drawn from a 100%-sampled class. One of them, DEL-08-05
  `CONTEXT#description`, would move to tier INVARIANT when corrected; see §7
  item 1.

## 2. Per-deliverable tables

Cells read "total / sampled". "False-align" is the firm false-alignment rate
among sampled ALIGNED normative rows.

| Deliverable | Rows | C100 | SH | F2/4 | F7 | NA | AN | Q | Flag | Sampled | Firm | Weak | Field | False-align |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| DEL-08-01 | 106 | 0 | 9/9 | 3/3 | 2/1 | 37/10 | 16/4 | 39/4 | +1 | 32 | 0 | 2 | 0 | 0/8 = 0% |
| DEL-08-02 | 97 | 0 | 7/7 | 2/2 | 5/2 | 38/10 | 12/3 | 33/4 | 0 | 28 | 1 | 0 | 0 | 1/7 = 14.3% |
| DEL-08-03 | 94 | 0 | 7/7 | 2/2 | 1/1 | 39/10 | 13/3 | 32/4 | 0 | 27 | 0 | 0 | 0 | 0/6 = 0% |
| DEL-08-04 | 117 | 1/1 | 8/8 | 4/4 | 0 | 26/7 | 19/4 | 59/6 | +3 | 33 | 0 | 3 | 1 | 0/8 = 0% |
| DEL-08-05 | 136 | 7/7 | 11/11 | 3/3 | 0 | 36/9 | 18/4 | 61/7 | 0 | 41 | 1 | 0 | 0 | 1/6 = 16.7% (counted as normative, see §1) |
| DEL-08-06 | 117 | 0 | 7/7 | 4/4 | 7/2 | 27/7 | 16/4 | 56/6 | +1 | 31 | 0 | 0 | 0 | 0/10 = 0% |

Firm error rate on sampled rows: DEL-08-02 3.6%, DEL-08-05 2.4%, others 0%.

Sampled keys (suffix after `<DEL>:`):

- **DEL-08-01:** CONTEXT#architecture-basis-injection (with .s01 and .s02),
  #decomposition-reference, #description, #package-reference,
  #preparation-notes, #scope-coverage; SOW CLM-004.r02, r05; CLM-005.r01,
  r02, r04, r07; CLM-010; CLM-011; CLM-011/R-08-01-006, 009, 010; CLM-014;
  CLM-020; CLM-023; CLM-026; CLM-028.r03, r06; CLM-030.r01, r03; CLM-032;
  CLM-033; CLM-033/OQ-08-01-001; STATUS; STATUS#remaining/R01 (flag).
- **DEL-08-02:** CONTEXT#architecture-basis-injection (with .s01 and .s02),
  #decomposition-reference, #description, #objective-support,
  #package-reference, #preparation-notes; SOW.s01; CLM-003; CLM-004.r02,
  r06; CLM-005.s03; CLM-006.r01; CLM-009; CLM-010; CLM-011.r03, r07; CLM-012;
  CLM-013/V-4, V-5; CLM-017; CLM-018; CLM-020; CLM-024; CLM-026; AC-001;
  STATUS#remaining.
- **DEL-08-03:** CONTEXT#architecture-basis-injection (with .s01),
  #decomposition-reference, #package-reference, #preparation-notes,
  #scope-coverage; SOW; CLM-004; CLM-005; CLM-006.r01, r02, r04; CLM-009;
  CLM-012/REQ-001, 003, 007; CLM-015; CLM-018; CLM-021; CLM-024; CLM-026.r01,
  r04, r05; completion-and-reliance-basis-epistemology;
  output-and-evaluation-matrix and its OUT-001; STATUS#remaining.
- **DEL-08-04:** CONTEXT#architecture-basis-injection (with .s01),
  #decomposition-reference, #package-reference, #preparation-notes,
  #register-references, #scope-coverage; MEMORY.s01; SOW CLM-003.r03, r04,
  r08; CLM-004; CLM-004.r04, r07; CLM-005; CLM-005.s01, s04; CLM-006;
  CLM-011.r03, r11, r12 (flag); CLM-012.r03; CLM-013/V-2, V-3, V-6, V-7;
  CLM-020.r02; CLM-021; CLM-025.r03, r04 (flag);
  governing-values-and-decisions-axiology; OUT-001 (matrix);
  STATUS#remaining/R01 (flag).
- **DEL-08-05:** CONTEXT#anticipated-artifacts,
  #architecture-basis-injection (with .s01), #decomposition-reference,
  #description, #package-reference, #preparation-notes; MEMORY.s01; SOW
  CLM-003.r03, r04, r05, r08; CLM-004.r04, r05, r06; CLM-006.r01, r02, r03;
  CLM-007.r01, r02, r04, r07; CLM-011/REQ-002, 003, 004, 005, 006;
  CLM-013.r04, r06, r08; CLM-014; CLM-015; CLM-016; CLM-020; CLM-024.r03;
  CLM-032; AC-001; governing-values-and-decisions-axiology;
  output-and-evaluation-matrix; STATUS#remaining/R01, R02.
- **DEL-08-06:** CONTEXT#architecture-basis-injection (with .s01),
  #decomposition-reference, #objective-support, #package-reference,
  #preparation-notes, #register-references,
  #sca-002-control-surface-refresh-note, #scope-coverage; SOW CLM-003.r04;
  CLM-004.r09; CLM-005.r05; CLM-006.r03; CLM-007; CLM-011; CLM-012.r02, r05,
  r08, r12, r13, r14; CLM-016; CLM-018; CLM-019; CLM-022.r02, r03; CLM-023;
  CLM-029.s01; completion-and-reliance-basis-epistemology;
  production-and-verification-method-praxeology; STATUS#remaining/R01 (flag).

## 3. Evidence spot-checks against the freeze (sampled substance)

- **Rule-pack binding (FG-DEL-08-01-01, -02-03, -03-03).**
  `renderableReportInput.ts` sends `rule_pack_refs: []` (line 431) and
  `assumptions: []` (line 340). `reportPackageRequest.ts:114-117` throws
  `REPORT-PACKAGE-RULE-PACK-BINDING-UNAVAILABLE` unless the rule check is
  `RULE_INPUTS_INCOMPLETE`. `workspaceSession.ts:793-794` builds every
  product input manifest with `active_rule_packs: []` and
  `external_assets: []`. This supports the PARTIALLY_IMPLEMENTED recording
  rows and the `PRODUCT_CALLER: NONE` constraint rows.
- **Canonicalization label (FG-DEL-08-02-01).** `audit_manifest`
  `enum Canonicalization { ProjectLocalDeterministicJson, None }` has no RFC
  8785 value. `reportPackageRequest.ts:66` labels renderer checksums
  `rfc8785_jcs`, while lines 300 and 307 label the model and input-manifest
  hashes (computed with `canonicalSha256Hex`) `project_local_deterministic_json`.
  The IMPLEMENTED_DIFFERENTLY · POSSIBLE_DEFECT rows are supported.
- **Diagnostic class inference (FG-DEL-08-03-01, FG-DEL-08-04-02).**
  `renderableReportInput.ts:91-111` derives class from severity and inserts a
  fixed remediation; `reportPackageRequest.ts:61-63` does the same. The
  desktop result-export writer sets every diagnostic's class to the constant
  `ASSUMPTION_WARNING`, including `blocking` ones, with a fixed remediation.
  The renderer's diagnostics table has columns Code, Severity, Message,
  Remediation only (`report_renderer/src/lib.rs:580`); provenance notes
  show Source, Location, License, Contributor only (line 600). FG-DEL-08-03-02
  is supported.
- **Tests cited by sampled ALIGNED rows exist:**
  `fixture_report_renders_deterministic_single_file_html`,
  `fixture_report_emits_deterministic_hash_bound_pdf`,
  `fixture_package_assembles_deterministic_unblocked_container`,
  `missing_core_manifest_evidence_emits_explicit_findings`,
  `missing_rule_pack_checksum_is_blocking`,
  `records_non_json_asset_hash_separately_from_model_hash`,
  `status_text_does_not_include_approval_or_compliance_claims`,
  `explicit_missing_user_rule_value_is_preserved_as_finding`,
  `user_rule_values_need_quantity_metadata`,
  `diagnostic_records_must_preserve_report_facing_fields`,
  `test_missing_source_values_become_explicit_findings_and_tbds`,
  `test_persisted_run_history_binds_to_report_sections_without_state_inference`.
  Pass status is taken only from `GATE:GATE_EVIDENCE/B4_4_SWEEP_9D55/SUMMARY.json`
  (not rerun), as A6 requires.
- **DEL-08-04 V-3.** `RESULT_EXPORT_PROVENANCE_MISSING` is emitted
  (`result_export/src/lib.rs:433`) but none of the crate's 13 tests exercises
  it. PARTIALLY_IMPLEMENTED is supported.
- **Linter (FG-DEL-08-05-01, -02).** `prohibited_claim_phrases()`
  (`protected_content_linter/src/lib.rs:636-645`) names only the former
  product name for certified/sealed/approved/authenticated. Detection of code
  text and formulas rests on planted `OPS_SYNTHETIC_*` markers plus the
  standards-table signature. The crate has an empty `[dependencies]`
  section. The INVARIANT rows and the CLM-013.r08 UNKNOWN (with its smallest
  next check) are supported.
- **Report package (DEL-08-01 R01).** `PACKAGE_MANIFEST_SCHEMA_VERSION` is
  fixed at `1.0.0` and `CONTAINER_EXTENSION` is `opsproj`. The CP-04
  identity-rename variant on the STATUS surface and the F2 gap disposition on
  R01 are supported.
- **DEL-08-06.** The TypeScript projection
  (`stateComparisonHandoffSections.ts`) was added in `d2d8975ef`
  (2026-07-23), after the 2026-07-12 Remaining text. The Python engine has no
  product caller (`report_package` names only the member role
  `state_comparison_handoff_sections`). `Dependencies.csv` has 29 rows (28 ACTIVE). R01
  REMAINING_STATE_MISMATCH, CLM-004.r09 and the F7 markers are supported.
- **CP-09.** `DELIVERABLE_INVENTORY.csv` gives `AnyPassParityMatchesFrozen=NO`
  for all six, which supports EVIDENCE_OVERTAKEN on the sampled OUT-001 rows.

Every sampled F2/F4 row is sound apart from W-1 below: each
`GAP_WORDING_CHECKED` clause is the last Notes clause and explains a use of
"missing", "without" or a test name. The two DEL-08-05 `OPEN_ACTION` targets
(REQ-010 and REQ-012) are non-aligned rows carrying the open work. Sampled F7
rows are claims about the engine or library (C6(b)), apart from the firm
error F-1.

## 4. Disagreements

### 4.1 Sampled rows

**F-1 (firm). `DEL-08-02:SOW#CLM-006.r01`, the input manifest.**

- **What the row says:** ALIGNED (MEDIUM), with no `PRODUCT_CALLER` marker.
  The input manifest "lists canonical model inputs, rule-pack references,
  unit basis, solver settings and external asset references". Its Notes add
  that the manifest's "rule-pack slots are populated with an empty list in
  the product (see FG-DEL-08-02-03 on recording)".
- **What I found:** the only product builder of the manifest
  (`apps/desktop/src/features/workspace/workspaceSession.ts:793-794`)
  always passes `active_rule_packs: []` and `external_assets: []`, whatever
  rule packs or assets the session has. The DEL-08-04 notes show a GUI rule
  check can leave the run `USER_RULE_CHECKED`. The worker's own F5 reason on
  RC-08-0017 confirms it cites `workspaceSession.ts` for exactly this. The
  row's Notes therefore record an unmet element of its own claim, and the
  sibling recording rows (CLM-004.r04, CLM-006.r04, `CONTEXT#description`)
  are PARTIALLY_IMPLEMENTED for the same gap. F1 applies "even when the same
  gap is also recorded on another row".
- **Right values:** PARTIALLY_IMPLEMENTED · PARTIAL_SLICE ·
  PROJECT_BASELINE · NONE · VALIDATION · FindingGroup FG-DEL-08-02-03 ·
  AuthorityNeeded NO. RemainingWork: bind active rule-pack references and
  external-asset references into the product input manifest.
- **Impact:** one more row on an existing finding group; tier and owner
  routing of the group are unchanged.

**F-2 (firm). `DEL-08-05:CONTEXT#description`.**

- **What the row says:** DECLARED_STATE · ALIGNED (MEDIUM). "Implement checks
  that prevent public report templates/examples from embedding protected code
  text, tables or formulas." Notes: "Checks implemented and wired into report
  rendering; detection breadth judged on REQ-002."
- **What I found:** the worker's REQ-002, CLM-006.r01 and AC-001 rows record,
  correctly, that copied code text and formulas are detected only when a
  planted synthetic marker is present
  (`protected_content_linter/src/lib.rs:366-402`). Real copied prose or
  formulas pass without a finding. That is two of the three content kinds
  this row names. The Notes defer the unmet element to REQ-002, which F1
  forbids. The row is also an implementation requirement, typed REQUIREMENT in
  the DEL-08-01/02/03 ledgers.
- **Right values:** REQUIREMENT · PARTIALLY_IMPLEMENTED · PARTIAL_SLICE ·
  INVARIANT · NONE · IP_DATA · FindingGroup FG-DEL-08-05-01 · AuthorityNeeded
  NO. RemainingWork as REQ-002.
- **Impact:** the tier changes from quiet to INVARIANT, joining the existing
  FG-DEL-08-05-01 group; the owner already sees that group at R4 through
  REQ-002. See §7 item 1 on how the rerun rule reads here.

**W-1 (weak). `DEL-08-01:SOW#CLM-011/R-08-01-009`, schema-first boundary
(worker-flagged).**

- **What the row says:** ALIGNED (MEDIUM), with
  `GAP_WORDING_CHECKED` explaining that the missing architecture-review record
  concerns only the verification method.
- **What I found:** `render_calculation_report`
  (`src-tauri/src/lib.rs:1491-1503`) deserialises a typed serde struct; no
  runtime JSON Schema validation. The schema is pinned by a contract test
  only. The verification column requires an "architecture review … before
  implementation acceptance", and none was located. Acceptance has not
  occurred (IN_PROGRESS), so the review is not yet due, which is the same
  reading the worker applied to R-08-01-003. The PKG-04 verifier treated a
  missing verification-column check as an F1 element (DEL-04-04 REQ-01).
- **Why weak:** both readings of "schema-first" (typed contract pinned to a
  schema versus runtime schema validation) and of a not-yet-due verification
  column are defensible. A convention should say whether a verification
  column's check is an element of the claim before its trigger.

**W-2 (weak). `DEL-08-01:SOW#CLM-004.r05`, architecture baseline incl.
"canonical JSON/JCS hash basis where JSON is hashed" (worker-flagged).**

- **What the row says:** ALIGNED (MEDIUM); renderer checksums use
  `rfc8785_jcs`.
- **What I found:** the report package, which DEL-08-01 carries
  (CLM-030.r01, RC-08-0183), labels its model and input-manifest hashes
  `project_local_deterministic_json`. DEL-08-02 disposes the same baseline
  element IMPLEMENTED_DIFFERENTLY · POSSIBLE_DEFECT (FG-DEL-08-02-01).
- **Why weak:** the audit-manifest member is DEL-08-02's subject, so the
  DEL-08-01 row is defensible on its own renderer. R3 should read CLM-004.r05
  with FG-DEL-08-02-01.

**W-3 (weak). `DEL-08-04:SOW#CLM-011.r12` (R12, no bypass around governed
result envelopes; worker-flagged).**

- **What the row says:** ALIGNED (MEDIUM). Adapter-format exports owned by
  other deliverables are outside this contract.
- **What I found:** the PCF, CAEPIPE and export-adapter-SDK panels consume
  `MechanicsResult` directly, not the DEL-08-04 envelope. R12 names
  "import/export adapters" and cites AB-00-07 ("Plugins/adapters cannot bypass
  … envelopes").
- **Why weak:** whether AB-00-07's "envelopes" means this result-export
  envelope or any governed result is not settled by the conventions. A
  broader reading would make the row PARTIALLY_IMPLEMENTED · OWNERSHIP_ELSEWHERE.

**W-4 (weak). `DEL-08-04:SOW#CLM-013/V-6`** (routing an export through an
adapter or downstream tool preserves unit, provenance, diagnostics and
boundary fields). ALIGNED on the report-package and headless-runner tests.
The same AB-00-07 reading as W-3 applies: no adapter route is tested.

**W-5 (weak). `DEL-08-04:SOW#CLM-005.s04`** (exported diagnostics carry
structured information, not prose-only warnings). ALIGNED (MEDIUM), with the
Notes deferring class and remediation fidelity to CLM-011.r05. The fields are
structured, which is this claim's subject. However, the product writer's class
is a constant `ASSUMPTION_WARNING` even for `blocking` diagnostics, so the
class carries no triage information. This sits at the F1 boundary.

**Field-1. `DEL-08-04:SOW#CLM-013/V-7` (C100).**

- **What the row says:** PARTIALLY_IMPLEMENTED · PARTIAL_SLICE · INVARIANT ·
  IP_DATA;CLAIMS · AuthorityNeeded NO. RemainingWork: "Extend a
  protected-content and claims scan to fixtures/results."
- **What I found:** the DEC-058 release-candidate scan
  (`projects/chirality-piping/tools/release/run_release_candidate_scan.py`)
  already has artifact class AC-5 ("Public examples, fixtures, validation
  benchmarks, corpora") and runs the DEL-08-05 engine lint on it. The unmet
  element is that no owner-run scan record exists (DEL-08-05 R02), not a
  missing scanner.
- **Right values:** disposition, tier and layers stand. Add the release-scan
  tool to ImplementationEvidence. RemainingWork: run the DEC-058 scan over
  AC-5 (owner act), noting the engine's former-name phrase gap
  (FG-DEL-08-05-02).

### 4.2 Outside the sample (read, not counted in the rates)

- **Weak. `DEL-08-05:SOW#CLM-004.r01`** (purpose: checks that keep templates
  free of protected code text, tables and formulas). ALIGNED with "Detection
  breadth is judged on REQ-002". This is the same F1 question as F-2, softened
  because the unit is a purpose statement.
- **Weak, intra-package pattern divergence on `AC-001`.** G1 (DEL-08-01, 02,
  03) and DEL-08-04 judge AC-001 on the contract text or schema (ALIGNED) and
  defer implementation to the requirement rows. DEL-08-05 judges it on the
  implementation (PARTIALLY_IMPLEMENTED · INVARIANT). DEL-08-06 judges it on
  the engine (ALIGNED, F7). A convention should settle AC-001's subject.
- **Observation.** `DEL-08-05:SOW#CLM-011/DEL-08-05-REQ-011` (no
  transmission by default) is ALIGNED on structural evidence, while the
  matching V row CLM-013.r08 is UNKNOWN. That is consistent: the requirement
  is met structurally and the V row asks for an exercising check.

## 5. Batch consistency and shared situations

- **Batch mode:** PASS, 0 findings. No resolutions file exists for W3.
- **Shared bodies (`SharedTextCount > 1`, 100% reviewed, 49 rows).** Every
  shared body is judged the same way across the PKG-08 ledgers, and the same
  as the sealed ledgers of other waves where they share it:
  - CS-01 and CS-04 pins: STALE_REVIEW_OR_EVIDENCE · BASIS_POINTER_STALE.
  - CS-02 and CS-03 notes: ALIGNED.
  - Empty Remaining: NOT_ASSESSED.
  - Conflict tables (CLM-032): ALIGNED.
- **Shared-situation conflict not caught by batch (minted `.s01`, no
  `BodySHA256`).** For "PKG-00 at SEMANTIC_READY", all six rows are
  STALE_REVIEW_OR_EVIDENCE · LOCAL_DESIGN · RECORD, as Direction 8 requires.
  The cause, however, splits by worker group: G1 (DEL-08-01, 02, 03) uses
  SCOPE_REDIRECTED_BY_RULING and G2 (DEL-08-04, 05, 06) uses RECORD_DRIFT.
  This is the corpus-wide cluster SR-1 already recorded in
  `WAVES/W2/RESOLUTIONS.csv`. The six rows should join it; no re-disposition
  is needed.
- **Minor cause divergence, same situation.** The setup-status field "Draft
  setup artifact; not implementation" takes STALE_REVIEW_OR_EVIDENCE in both
  groups. G1 gives it cause RECORD_DRIFT (DEL-08-02 CLM-003); G2 gives it
  DOC_BEHIND_CODE (DEL-08-04 CLM-003.r08). The tier, layer and routing are the
  same. R3 may harmonise the cause.
- **G2's CP-04 on DEL-08-06's SOW surface for code-only residue** (the SOW
  neither carries nor names the identifier). This is a reasonable C1 reading;
  CP-04's wording does not say where such residue is recorded. For R3.

## 6. Reverse pass

Coverage:

- 100% of CLAIMED_BY (31), PARTIAL (11), UNKEYED (3) and COVERS (9)
  answers. There are no CONSTRAINS answers.
- 10% of NOT_MINE: 188 of 1,872 (deliverable × capability), chosen by the
  lowest SHA-256 of `DEL|RC-ID`.
- F5 on 20% of NOT_MINE rows whose EntryPoints hit a path the deliverable's
  forward ledger cites: 14 of 67 candidates.

Routing-local IDs were resolved through `ROUTING_SAMPLE/SAMPLE_MANIFEST.csv`.
PKG-08 has 321 routed rows: 284 AREA and 37 SAMPLE. By area: DATA 70, COREC
53, DOCS 52, CHECKS 42, FEATC 36, FEATB 31, others 37.

**Findings:**

- **Capabilities claimed by more than one PKG-08 deliverable.** All six are
  legitimate splits, not contested ownership: the owner answers CLAIMED_BY
  and the others answer PARTIAL for their own part.
  - RC-08-0053 (renderer input builder): 01 CLAIMED_BY, 03 PARTIAL.
  - RC-08-0056 (package request): 01 CLAIMED_BY, 02 PARTIAL, 06 COVERS.
  - RC-08-0093 (three-point lint gate): 01 CLAIMED_BY, 05 PARTIAL, 03 COVERS.
  - RC-08-0160 (renderer): 01 CLAIMED_BY, 03 PARTIAL.
  - RC-08-0181 (package wire): 01 CLAIMED_BY, 02 PARTIAL.
  - RC-08-0238 (`fixtures/reports/`): 01 PARTIAL, 06 PARTIAL, each for its
    own fixtures.
- **Suspected missed claims or ownership gaps:**
  - **The report-package container has no owning key.** RC-08-0183 (the
    `.opsproj` container), RC-08-0130 (native save) and RC-08-0193 (save
    controls) are UNKEYED against DEL-08-01. The container was built under
    DEL-08-01 (DEC-028, DEC-057) but no issued key requires it. This is a
    scope finding for R3, like PKG-04's `curved_bend`.
  - RC-08-0037 (the shared RFC 8785 crate) is PARTIAL from DEL-08-02. The
    crate serves every hash seam, and the worker itself suggests COVERS.
    COVERS is the stricter reading (weak).
  - In the 10% NOT_MINE sample and in the full NOT_MINE population, no answer
    denies a capability whose entry point is the deliverable's own crate.
    There are 15 own-name hits in total. All are substring false positives:
    `canonical_json` as a function name in handoff engines, and
    `report_sections` inside `state_comparison_handoff_sections`.
  - The `core/handoff/*` packages, the run-audit panel and the analysis-run
    record are all NOT_MINE. None is PKG-08 scope on the frozen decomposition;
    DEL-08-06 covers handoff *report sections*, not the handoff packages.
- **F5.** All 14 sampled overlap reasons name the specific capability and say
  why the shared path does not confer ownership. None is a bare template. The
  per-deliverable template reasons ("Outside DEL-08-0x (…)") occur only where
  there is no path overlap.
- **Sampled rows compared with area rows:**
  - AREA (284): 244 all NOT_MINE (85.9%), 36 with an ownership answer, 4
    COVERS only.
  - SAMPLE (37): 33 all NOT_MINE (89.2%), 3 with an ownership answer, 1
    COVERS only.
  - The three sampled ownership answers are RC-08-0037 (canonical JSON),
    RC-08-0092 (input manifest service, CLAIMED_BY DEL-08-02) and RC-08-0130
    (native report-package save, UNKEYED DEL-08-01). Each is recognisable
    from its path. Each is also backed by a forward citation made before the
    routing file was read: `inputManifestService.ts` in DEL-08-02 CLM-006.r01,
    and `atomic_report_package_save.rs` in DEL-08-01 R01. So they are real
    ownership, not anchoring. R3 should check these three against the answers
    of the capabilities' home-area packages for double claims.
- **Anchored answers:** none identified.

## 7. For the owner, stated plainly

1. **Two aligned rows record their own gap (F1), and one of them is an IP
   boundary row.** DEL-08-05 `CONTEXT#description` says the linter should
   prevent templates embedding protected code text, tables or formulas, but
   real copied prose and formulas are not detected. Its correct tier is
   INVARIANT. It was drawn from the 10% quiet class, so the literal rerun rule
   ("a firm error on a 100%-sampled class") does not fire. The gap is already
   routed to R4 as INVARIANT through REQ-002 and FG-DEL-08-05-01. I recommend
   Agent 0 record both firm rows in `WAVES/W3/RESOLUTIONS.csv` (F6) rather
   than rerun. The owner may read the rule the other way; if so, DEL-08-05
   would need a rerun.
2. **The claims linter cannot see certification claims under the current
   product name** (FG-DEL-08-05-02, INVARIANT · CLAIMS, AuthorityNeeded
   REVIEW). The engine that gates every rendered report and PDF matches
   "certified/sealed/approved/authenticated by openpipestress" only. After
   the SCA-010 rename, the same claims naming SWBPIPE pass. This is rename
   residue with a functional professional-boundary effect.
3. **Diagnostic classes are invented, not carried.** The desktop report and
   package adapters derive class from severity and insert a fixed
   remediation. The result-export writer labels every diagnostic
   `ASSUMPTION_WARNING`, including blocking ones. These are R4 code-change
   candidates: FG-DEL-08-03-01 (OWNER) and FG-DEL-08-04-02 (REVIEW).
4. **Hash labels disagree with the hashing.** Product model and input-manifest
   hashes are RFC 8785 values labelled `project_local_deterministic_json`,
   because the audit-manifest wire has no JCS label (FG-DEL-08-02-01, R4
   code-change candidate).
5. **Rule packs never reach product reports, packages or input manifests.**
   The package request refuses any run with an active rule check. This is
   consistently PARTIALLY_IMPLEMENTED across DEL-08-01 to 04 (and F-1 adds one
   row).
6. **No protected-content release scan has been run.** The DEC-058 scan tool
   covers fixtures (AC-5), but no scan record exists (DEL-08-05 R02). DEL-08-04
   V-7 is INVARIANT on this.
7. **Ownership gap for R3.** The `.opsproj` report-package container, native
   save and save controls were built under DEL-08-01 with no issued key.
   DEL-08-01 Remaining R01 (the container compatibility-window policy) is
   DOCUMENTED_UNIMPLEMENTED with no governing row; the item itself names
   DEL-02-05.
8. **The `.s01` cause split recurs** (SR-1, RECORD_DRIFT against
   SCOPE_REDIRECTED_BY_RULING). The six PKG-08 rows should join the existing
   cluster.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
