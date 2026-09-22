VERDICT: ACCEPT WITH CONTESTED ROWS

# PKG-05 verification (wave W3, sampling STANDARD)

Fresh, evidence-only package verifier for run
`HELP-HUMAN-PIPING-20260921-RECONCILIATION`. Parent: HELP_HUMAN Agent 0.

- **Brief:** `briefs/R2-VERIFIER_brief.md`. Its SHA-256 (`47fb3c52…5b00`) was
  checked and matches.
- **Evidence checkout:** `00115c71931bcae79909602d653740d3bb72dfa1` (the
  freeze, read-only; `HEAD` confirmed).
- **Rules applied:** `CONVENTIONS.md` Parts A to F (Part F judged like any
  other rule), `CANONICAL_SITUATIONS.md` and
  `AUTHORITY_AND_SOURCE_RELIABILITY_MAP.md`.
- **Manager records:** I read the PKG-05 manager's transcripts and the worker
  returns only for the flags they raise. I did not use them as evidence for
  any claim.

These are agent verification judgments. They are not owner rulings, and they
do not state or imply any release, approval, compliance or certification
(F-PIP-2; claims taxonomy per DEC-081).

## 0. Integrity and mechanical checks

- **Seals.** For all five ledgers, the forward SHA-256 I recomputed matches
  the `_SEAL.txt` line and the manager's return. The reverse hashes also match
  the return.
- **Single mode.** I ran `validate_ledger_v2.py` in single mode with
  `--forward --reverse --inventory ROUTING/PKG-05_capabilities.csv
  --notes-gap`, using `--repo-root` at the freeze. All five ledgers pass with
  0 findings:

  | Deliverable | Forward rows | Required keys | Canonical rows |
  |---|---|---|---|
  | DEL-05-01 | 88 | 85 | 7 |
  | DEL-05-02 | 80 | 73 | 7 |
  | DEL-05-03 | 70 | 67 | 7 |
  | DEL-05-04 | 91 | 88 | 7 |
  | DEL-05-05 | 86 | 55 | 7 |

- **Batch mode.** `--batch` over the five forward ledgers returns PASS with
  0 consistency findings, the same as the manager's run.
  `WAVES/W3/RESOLUTIONS.csv` does not exist.
- **Keyed canonical rows.** The validator checks conformance on 100% of these
  rows, and all conform.
- **Reverse files.** Each reverse file answers all 419 routed capabilities
  and ends with a `#END` sentinel whose count is 419.

## Sampling (deterministic)

Each class's candidate keys were sorted by the SHA-256 of the full claim key,
and the lowest fraction was taken, rounded up. Rates are STANDARD:

| Class | Rate |
|---|---|
| C100: INVARIANT tier, UNKNOWN, AUTHORITY_CONFLICT, ACCEPTED_DIVERGENCE, LIFECYCLE_REASSESSMENT_REQUIRED, PROTECTED_CHECK or FROZEN_CONTRACT, ISSUED | 100% |
| SH: `SharedTextCount > 1` (a minted `.sNN` key takes its parent's count) | 100% |
| F2/4: ALIGNED rows carrying `GAP_WORDING_CHECKED:` or `OPEN_ACTION:` | 100% |
| F7: ALIGNED rows carrying `PRODUCT_CALLER: NONE` | 25% |
| NA: other non-aligned rows | 25% |
| AN: ALIGNED normative rows | 20% |
| Q: structural rows, inherited canonical rows and other quiet rows | 10% |

- Each row falls in the first class that matches, in the order shown.
- **Weighting of the AN class.** Rows with LOW or MEDIUM confidence,
  `NONE_FOUND` verification or verification class `NONE` were placed first.
  The rows within each group were then ordered by hash.
- No deliverable is ISSUED. None carries PROTECTED_CHECK, FROZEN_CONTRACT,
  UNKNOWN or AUTHORITY_CONFLICT.

In total, 136 of 415 forward rows were sampled. Beyond the sample, I read
every row of all five ledgers once. Findings outside the sample are reported
separately in §4.2 and are not counted in the rates.

## 1. Package-level result

- **Firm disagreements on sampled rows: 1** (DEL-05-01).
- **Weak: 9** on sampled rows, plus 3 outside the sample.
- **Field: 1** on a sampled row.
- **Package firm false-alignment rate: 1 / 40 sampled ALIGNED normative rows
  = 2.5%.** The gate is 5% or less, so it is met.
- **No deliverable meets a rerun condition.**
  - DEL-05-01's firm error rate is 1 of 32 sampled rows (3.1%), below 10%.
  - The firm error falls in the AN class (20% sampling), not in a 100% class.

## 2. Per-deliverable tables

Cells read "total / sampled". "False-align" is the firm false-alignment rate
among sampled ALIGNED normative rows, counted across every class.

| Deliverable | Rows | C100 | SH | F2/4 | F7 | NA | AN | Q | Sampled | Firm | Weak | Field | False-align |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| DEL-05-01 | 88 | 1/1 | 8/8 | 9/9 | 5/2 | 18/5 | 20/4 | 27/3 | 32 | 1 | 1 | 0 | 1/14 = 7.1% |
| DEL-05-02 | 80 | 0 | 6/6 | 3/3 | 1/1 | 24/6 | 18/4 | 28/3 | 23 | 0 | 3 | 0 | 0/6 = 0% |
| DEL-05-03 | 70 | 2/2 | 7/7 | 1/1 | 4/1 | 18/5 | 10/2 | 28/3 | 21 | 0 | 1 (+1 out of sample) | 0 | 0/4 = 0% |
| DEL-05-04 | 91 | 9/9 | 7/7 | 1/1 | 0 | 10/3 | 27/6 | 37/4 | 30 | 0 | 1 | 1 | 0/6 = 0% |
| DEL-05-05 | 86 | 0 | 7/7 | 6/6 | 33/9 | 14/4 | 1/1 | 25/3 | 30 | 0 | 3 (+2 out of sample) | 0 | 0/10 = 0% |

Sampled keys (the claim-key suffix after `<DEL>:`):

- **DEL-05-01:**
  - C100: CLM-016/REQ-05-01-003.
  - SH: CONTEXT#architecture-basis-injection (with .s01 and .s02);
    #decomposition-reference; #objective-support; #package-reference;
    #preparation-notes; STATUS#remaining.
  - F2/4 (all nine): CLM-014/REQ-05-01-005, 010 and 011; CLM-016/REQ-05-01-004,
    005 and 010; AC-001; CLM-023; CLM-030.
  - F7: CLM-014/REQ-05-01-006 and 012.
  - NA: CLM-002, CLM-006, CLM-009, CLM-022, MEMORY.s01.
  - AN: CLM-015, CLM-020, CLM-021, VER-001.
  - Q: CONTEXT#context-envelope, STATUS#history, CLM-025.
- **DEL-05-02:**
  - SH: CONTEXT#architecture-basis-injection (with .s01 and .s02);
    #decomposition-reference; #package-reference; #preparation-notes.
  - F2/4: CLM-010/REQ-05-02-006; CLM-012/REQ-05-02-006; CLM-021.
  - F7: CLM-023.r05.
  - NA: SOW; purpose OUT-001; CLM-003; CLM-005; STATUS#remaining/R02; CONTEXT.
  - AN: CLM-010/REQ-05-02-009, CLM-011, CLM-015, CLM-016.
  - Q: CLM-014; the epistemology wrapper; CONTEXT#context-budget-qa.
- **DEL-05-03:**
  - C100: CONTEXT#description; CLM-011/DEL-05-03-RQ-001.s01.
  - SH: CONTEXT#architecture-basis-injection (with .s01 and .s02);
    #decomposition-reference; #objective-support; #package-reference;
    #preparation-notes.
  - F2/4: RQ-003.
  - F7: RQ-009.
  - NA: CLM-003; RQ-006; STATUS#remaining/R02 and R03; CONTEXT.
  - AN: CLM-017, CLM-024.
  - Q: CLM-011; the purpose block; CONTEXT#scope-coverage.
- **DEL-05-04:**
  - C100 (all nine rows of FG-DEL-05-04-01): CLM-004; CLM-008; CLM-020;
    CLM-023; AC-001; CLM-011/REQ-05-04-008 and 014; CLM-013/REQ-05-04-008
    and 014.
  - SH: CONTEXT#architecture-basis-injection (with .s01 and .s02);
    #decomposition-reference; #objective-support; #package-reference;
    #preparation-notes.
  - F2/4: CLM-028.
  - NA: SOW.s01; CLM-007; output-matrix OUT-001.
  - AN: purpose OUT-001; CLM-011/REQ-05-04-007 and 011; CLM-013/REQ-05-04-001,
    004 and 011.
  - Q: CONTEXT; CLM-001; CLM-003; CLM-027.
- **DEL-05-05:**
  - SH: CONTEXT#architecture-basis-injection (with .s01 and .s02);
    #decomposition-reference; #package-reference; #preparation-notes;
    STATUS#remaining.
  - F2/4: CLM-004, CLM-005, CLM-010.r07, CLM-012.r02, CLM-012.r07, CLM-018.
  - F7: CLM-003.r01, r04 and r05; CLM-010.r02, r08 and r10; CLM-014; CLM-027;
    CONTEXT#description.
  - NA: CLM-003.r02 and r08; CLM-006; CLM-024.
  - AN: CLM-010.r01.
  - Q: the praxeology wrapper, CLM-015, CLM-016.

## 3. Evidence spot-checks against the freeze (sampled substance)

- **Test counts.** The freeze has 49 `#[test]` functions in `primitive_loads`,
  18 in `load_case_algebra`, 26 in `stress_recovery` and 28 in `user_loads`.
  These match the rows. I checked every named test cited by a sampled row
  (eight in `primitive_loads`, all 18 algebra tests, and nine in
  `user_loads`), and each exists.
- **Product callers (F7).**
  - `product_physics` imports only `prepare_loads`, the categories and types,
    and the two DEC-068 generators from `primitive_loads`.
  - `solver/diagnostics` imports only `FindingCode` and `LoadFinding`.
  - No product code calls the diagnostic bridge, the lumping, the axial helpers
    or `assemble_solver_load_vector`.
  - `product_physics` calls `recover_stresses` only. It does not call the
    station, range, unit-metadata or boundary-record surfaces.
  - `user_loads` has no product dependent. Its only dependent is
    `validation/benchmarks/mechanics`.
  - Every sampled `PRODUCT_CALLER: NONE` marker is therefore correct, and each
    sampled F7 row's claim is about the engine or crate itself.
- **DEL-05-01 REQ-05-01-010 (the worker flagged this reading).** The crate's
  `PrimitiveLoad` has no provenance field. However,
  `core/product_physics/src/validation.rs::validate_provenance` blocks with
  `PROVENANCE_INPUT_MISSING` on every primitive load, load case and
  equivalent-static generation input that lacks provenance. The product input
  record keeps that provenance. ALIGNED at MEDIUM is defensible, and I do not
  contest it.
- **DEL-05-01 CLM-016/REQ-05-01-003 (INVARIANT).** The last review found is
  from 2026-06-05. The DEC-068 generators landed on 2026-07-10. No later run
  record in the deliverable holds a protected-content review. I agree with
  STALE_REVIEW_OR_EVIDENCE · EVIDENCE_OVERTAKEN · INVARIANT.
- **DEL-05-01 MEMORY.s01.** `git log -S` places the header text at
  `7bee9ae41`, so STALE_SETUP_SPECIFICATION (F3) is right.
- **DEC-068** (`SOFTWARE_DECOMP.md` L659) authorizes user-input seismic and
  wind equivalent-static generation. This supports the SCOPE_REDIRECTED_BY_RULING
  rows in FG-DEL-05-01-02.
- **AB-00-06 gaps.** `AlgebraFinding`, `StressFinding` and `UserLoadFinding`
  each carry only a code, a subject or load ID, and a message.
  `LoadDiagnosticRecord` carries all eight fields. This supports DEL-05-02
  REQ-05-02-008 and DEL-05-03 RQ-005 as PARTIALLY_IMPLEMENTED. For DEL-05-05,
  see §4.2.
- **DEL-05-04 FG-DEL-05-04-01 (the worker flagged the R9 reading).**
  - The Remaining item before R9 (at `8fac6631a`) read: "Add the PDU-037
    runtime stale-hash acceptance-reuse negative when an owning acceptance
    runtime exists".
  - The R9 activation (`HELP-HUMAN-PIPING-20260820-R9-STALE-HASH/ACTIVATION.md`)
    scoped the work to the operation applier's claimed-model-hash gate.
  - At the freeze, `HumanAcceptanceRecord.bound_hashes` and
    `HumanAcceptanceRef.invalidates_on_hash_change` exist only in schemas and
    their tests.
  - `project_persistence/service.py` seeds `human_acceptance_refs` as `[]`,
    and nothing enforces invalidation at runtime.
  - I therefore agree with the worker's reading. R9 did not implement the
    "does not survive content changes" element for human acceptance records.
    That element holds only because no such records exist (CP-11). All nine
    rows stand as PARTIALLY_IMPLEMENTED · INVARIANT.
  - The owner-directed intent of R9 (A2) is context. It cannot amend an
    INVARIANT claim, and this goes to R4.
- **DEL-05-03 FG-DEL-05-03-02.**
  - `stress_recovery` computes thin-wall membrane stresses: hoop stress is
    p·r/t, and longitudinal stress is half of it.
  - `product_physics::pressure_exact` is the dormant kernel under D-67.
  - The activation plan records D01–D06 as held.
  - ENGINEERING_AUTHORITY_REQUIRED · OWNER_HOLD · INVARIANT is supported.
- **DEL-05-05 observation.** `authored_category_preview_mapping` maps
  `concentrated_force` and `concentrated_moment` to "occasional", and
  `distributed_force` to "weight". It emits `LOAD_CATEGORY_PREVIEW_MAPPED` as
  a warning, and a test covers it. The observation is confirmed. See §7.
- **CP-09.** In `EVIDENCE_MAP.csv`, only DEL-05-01 has a PASS parity record
  whose hash matches the frozen SOW. The other four are
  `AnyPassMatchesFrozen=NO`. The CP-09 rows conform.

In the sample, no F2/F4 row fails: every `GAP_WORDING_CHECKED` clause is the
last clause of its Notes, and each explains a test name, a finding-code name
or a feature name. The only failure of F1 is F-1 below.

## 4. Disagreements

### 4.1 Sampled rows

**F-1 (firm). `DEL-05-01:SOW#production-and-verification-method-praxeology/VER-001`**

- **What the row says:** ALIGNED, MEDIUM. It rests on the SOW-STAGE2 parity
  report (PASS, hash matches the frozen SOW), its claim map and the gate
  sweep. Notes: "The protected-content element is judged separately on
  CLM-016/REQ-05-01-003."
- **What I found:**
  - VER-001 names the review of "protected-content and professional
    boundaries" as an element of its own claim.
  - The same ledger's CLM-016/REQ-05-01-003 row records that element as unmet:
    STALE_REVIEW_OR_EVIDENCE · EVIDENCE_OVERTAKEN · INVARIANT. The last review
    (2026-06-05) predates the DEC-068 generator code (2026-07-10).
  - The parity report (`execution/_Coordination/AgentRuns/SOW-STAGE2-EXEC-20260712-01/instances/WORKING-P2-PKG05/manager-validation/DEL-05-01/parity.md`)
    is a 34-check source-mapping parity. It is not a protected-content review.
  - This is the F1 pattern: an ALIGNED row that defers an unmet element of its
    own claim to another row. F1 says such a row is not ALIGNED "even when the
    same gap is also recorded on another row".
- **Right values:** STALE_REVIEW_OR_EVIDENCE · EVIDENCE_OVERTAKEN · INVARIANT
  (F8: the gap touches the protected-content boundary) · NONE ·
  IP_DATA;RECORD · REVIEW. It should share a FindingGroup with
  CLM-016/REQ-05-01-003.
- **Impact:** tier LOCAL_DESIGN changes to INVARIANT, and routing changes to
  REVIEW. The row is in the AN class, not a 100% class. It is the only firm
  error in the sample.

**W-1 (weak). The AC-001 "unresolved policies" clause is judged three ways.**

- **Sampled row:** `DEL-05-01:SOW#completion-and-reliance-basis-epistemology/AC-001`
  (F2/4), which is ALIGNED.
- **The split:**
  - DEL-05-02 and DEL-05-03 AC-001 are STALE_REVIEW_OR_EVIDENCE ·
    SCOPE_REDIRECTED_BY_RULING. The worker's reason is that the "unresolved
    grammar/tolerance policy" clause was overtaken by DEC-022 and DEC-024/026,
    and it applied F1 strictly.
  - DEL-05-01 AC-001 ("unresolved production policies") is ALIGNED, although
    the same worker judged those production policies overtaken on CLM-017
    (DEC-018, DEC-024/026).
  - DEL-05-05 AC-001 ("unresolved … tolerance, and release policies") is
    ALIGNED, although its own CLM-003.r08 records the tolerance TBD as
    overtaken by DEC-026.
- **Why this is weak:** AC-001 can be read as a boundary ("where policies are
  unresolved, invent nothing"), which is met. It can also be read as a
  declaration that the policies are unresolved, which is stale. Worker G1
  itself flagged this as contested since W2.
- **Right values:** one wave-level reading. Tier and routing do not change
  (LOCAL_DESIGN, NO).

**W-2 (weak). `DEL-05-02:SOW#CLM-010/REQ-05-02-009` (AN, ALIGNED, MEDIUM).**

- **The claim:** tests shall cover "no-default rule-pack boundaries".
- **What I found:** the 18 tests cover status and human-approval boundaries.
  No test asserts that no default factor or combination is supplied. The
  row's Notes rely partly on "the absence of any factor table".
- **Why this is weak:** the reading is defensible, because the boundary
  concerns status and rule separation. Coverage by absence, though, borders on
  C6(a).
- **Right values:** as sealed, or PARTIALLY_IMPLEMENTED · PARTIAL_SLICE ·
  LOCAL_DESIGN · NO if a test is required for the no-default half.

**W-3 (weak). ALIGNED rows whose Notes defer a related gap to another row.**

- **Rows:** `DEL-05-02:SOW#CLM-021` (CONTEXT, F2/4) and
  `DEL-05-03:SOW#CLM-024` (principles, AN, MEDIUM).
- **What the rows say:**
  - CLM-021 says it leaves combinations "to user-owned rule packs". Its Notes
    say the rule-pack supply gap is judged on REQ-05-02-004.
  - CLM-024's principle "preserve … provenance through … outputs" says the
    diagnostic provenance question is RQ-005.
- **Why this is weak, not firm:** the deferred element sits at the edge of
  each claim's subject. CLM-021 is guidance, not an interface claim. The
  outputs that carry provenance (the boundary records) do carry it, while the
  findings are a separate artifact. Under a strict F1 reading, both would take
  their sibling's PARTIALLY_IMPLEMENTED.
- **Impact:** none on tier or routing.

**W-4 (weak). Purpose-block OUT-001 is treated as CP-09 by one worker and
not by the other.**

- **Sampled row:** `DEL-05-02:SOW#purpose-and-objective-traceability/OUT-001`
  is STALE · EVIDENCE_OVERTAKEN · CP-09.
- **The split:** worker G1 applied CP-09 to the purpose-block OUT-001
  (DEL-05-01 to 03). Worker G2 judged the purpose-block OUT-001 on its
  substance (DEL-05-04 and 05 are ALIGNED, not CP-09).
- **Which reading fits:** CP-09 names "Output-matrix OUT-001 and SOW parity
  rows". Read literally, that supports G2.
- **Right values:** a single package reading. Under G2's reading, DEL-05-02
  and 03 purpose OUT-001 would be judged on substance. They would likely be
  ALIGNED or PARTIALLY_IMPLEMENTED, given the rule-pack and AB-00-06 gaps.
  Tier and routing do not change.

**W-5 (weak) and F-2 (field). SURFACE rows are judged differently by the two
workers.**

- **SOW surface:**
  - Worker G1 records CP-04 on the `SOW` SURFACE row (DEL-05-01 to 03). It
    records no row for the frontmatter `decomposition_basis` pin
    (`eaad463c0` = revision 0.8).
  - Worker G2 records the pin (CP-02) on the SURFACE row and CP-04 on
    `SOW.s01` (DEL-05-04 and 05).
  - W2 practice is CP-04 on the SURFACE row and the pin on `.s01`.
  - **F-2 (field) on the sampled `DEL-05-04:SOW.s01`:** CP-04 says to record
    the finding "once, on the SURFACE row". G2's placement departs from that,
    with no `CANONICAL_DEPARTURE`. The fields themselves are correct.
- **CONTEXT surface (W-5, weak, on the sampled `DEL-05-04:CONTEXT`):**
  - DEL-05-01 to 03 are STALE · BASIS_POINTER_STALE for the revision-0.7
    pin. DEL-05-04 and 05 are ALIGNED, with the pins left to the CS-01 and
    CS-04 rows.
  - C1's common-defect rule arguably supports G2. Across W2 and the other
    W3 packages, the tally is 12 ALIGNED to 40 non-aligned.
  - Batch mode cannot see this split, because SURFACE bodies differ.

**W-6 (weak). Agent-produced validation witnesses support ALIGNED on
mechanics requirements.**

- **Sampled rows:** DEL-05-05 `CLM-010.r07`, `r08` and `r10`. Outside the
  sample, the same pattern appears on DEL-05-03 RQ-001 (the parent row).
- **What the rows cite:** hand-calculation witnesses with no human
  disposition, marked `UNVERIFIED`.
- **The worker's reading:** these requirements assert implemented behaviour,
  not reliance, so VERIFIED_NOT_VALIDATED does not apply.
- **The alternative:** A5 and C3 could be read to demand
  VERIFIED_NOT_VALIDATED · VALIDATION_GAP · INVARIANT · VALIDATION for
  engineering-mechanics recovery claims until a human disposes the witnesses.
- **Why this is weak:** the conventions do not settle whether an UNVERIFIED
  witness meets a validation need. It is a wave-level question, and the
  answer would change the tier.

### 4.2 Outside the sample (read, not counted in the rates)

- **Weak. `DEL-05-05:SOW#CLM-010.r06` (R6, ALIGNED).**
  - R6 requires that status and provenance distinctions be preserved "in
    diagnostics", citing AB-00-06.
  - `UserLoadFinding` carries only a code, a load ID and a message. That is
    the same shape that DEL-05-02 REQ-05-02-008 and DEL-05-03 RQ-005 dispose
    PARTIALLY_IMPLEMENTED · PROJECT_BASELINE.
  - R6's wording ("distinctions") is weaker than theirs, so the row is
    defensible. The cross-worker treatment is inconsistent.
- **Weak. `DEL-05-05:SOW#completion-and-reliance-basis-epistemology/AC-001`.**
  This is part of W-1.
- **Field (observation). The frontmatter pin is not recorded in DEL-05-01, 02
  and 03** (see W-5). The SURFACE rows themselves are correct.

## 5. Batch consistency and shared situations

- **Batch mode:** PASS, with 0 findings. There is no W3 resolutions file.
- **Shared bodies (SH, 100% reviewed, 35 rows).**
  - The CS-01, CS-02, CS-04 and CS-06 keyed rows conform.
  - I compared the minted `.s01` and `.s02` sub-claims by hand, because batch
    mode cannot see them. Their fields are identical in all five ledgers:
    - `.s01` is STALE_REVIEW_OR_EVIDENCE · SCOPE_REDIRECTED_BY_RULING. This
      follows the W2 AGENT_READING, with the cause in the SR-1 contested
      cluster.
    - `.s02` is STALE_SETUP_SPECIFICATION · SCOPE_REDIRECTED_BY_RULING.
- **Shared-situation splits that batch mode cannot see:**
  - SOW and CONTEXT SURFACE placement (W-5, F-2);
  - whether the purpose-block OUT-001 is CP-09 (W-4);
  - the AC-001 overtaken-TBD clause (W-1);
  - AB-00-06 diagnostics in DEL-05-05 R6 (§4.2).

  All of these split along worker lines: G1 took DEL-05-01 to 03, and G2 took
  DEL-05-04 and 05. None changes tier or owner routing.

## 6. Reverse pass

Coverage checked:

- 100% of CLAIMED_BY (26), PARTIAL (13), UNKEYED (4) and CONSTRAINS (0)
  answers.
- 10% of NOT_MINE: 203 of 2,015 (deliverable × capability), chosen by the
  lowest SHA-256 of `DEL|RC-ID`.
- F5 on 20% of NOT_MINE rows whose EntryPoints hit a path the deliverable's
  forward ledger cites: 34 of 160 candidates.

Routing-local IDs were resolved through `ROUTING_SAMPLE/SAMPLE_MANIFEST.csv`.
PKG-05 has 419 routed rows: 398 AREA and 21 SAMPLE. By area: DATA 70,
SOLVER 70, COREC 53, DOCS 52, PHYS 44, CHECKS 42, FEATC 36, FEATB 31, others
21.

**Capabilities claimed by more than one PKG-05 deliverable:**

- RC-05-0213, the solver and load README bundle, is PARTIAL from DEL-05-01,
  02, 03 and 05. Each claims only its own crate's README, so the split is
  legitimate.
- RC-05-0272 (`product_physics::build_load_case_primitive_loads`) is PARTIAL
  from DEL-05-01 (primitive categories) and DEL-05-05 (concentrated and
  distributed user loads). The split is legitimate, and it matches
  FG-DEL-05-05-01.
- No capability is CLAIMED_BY two deliverables.
- Cross answers are consistent:
  - RC-05-0034 and RC-05-0339: CLAIMED_BY in DEL-05-02 and 03, COVERS in
    DEL-05-04.
  - RC-05-0225: PARTIAL in DEL-05-02, COVERS in DEL-05-01 and 04.

**UNKEYED answers (all four checked and supported):**

- **Self-weight planning** (RC-05-0263 `product_physics/self_weight.rs`;
  RC-05-0408 `core/loads/self_weight_wasm`) → nearest key DEL-05-01 CLM-006.
  No PKG-05 record references self-weight, so the ownership is genuinely
  unassigned. The related UI, RC-05-0029 `SelfWeightPlanPanel`, is NOT_MINE
  everywhere. R3 should treat the three as one cluster.
- **The exact-annulus pressure kernel** (RC-05-0305) → DEL-05-03 RQ-001.s01.
  This is consistent with D-67's dormant kernel.
- **Bend and branch stress multipliers** (RC-05-0146) → DEL-05-03 CLM-005.
  The answer is plausible, but whether ownership lies with PKG-05 or with the
  PKG-03 components is an R3 question.

**Suspected missed claims:**

- RC-05-0063, the Load Cases manager's primitive-load creation.
  - DEL-05-05 answers PARTIAL and says the rest "belongs to DEL-05-01/GUI
    deliverables".
  - DEL-05-01 answers NOT_MINE with its GUI template reason, although a
    2026-06-11 `primitive_load_creation_editor` run record sits in DEL-05-01's
    own `_run_records`.
  - This is weak. A COVERS or PARTIAL answer from DEL-05-01 would have been
    more accurate.
- RC-05-0343 (`WindExposureForm`, input for DEC-068 wind generation) is
  NOT_MINE everywhere. It landed under the UI toolkit program (commits
  `46f3aed34` and `5e22bd14a`), not a DEL-05-01 tranche, so NOT_MINE is
  defensible.

**F5.** All 34 sampled overlap reasons name the specific capability and say
why the cited path (mostly `product_physics/src/lib.rs`, `solver/diagnostics`
or the registers) does not confer ownership. The reasons end in a template
tail per deliverable, but each opens with a clause specific to the
capability, which satisfies F5.

**Sampled rows compared with area rows:**

- SAMPLE (21): all NOT_MINE in all five deliverables (100%). Every sampled
  capability is desktop view, workspace, shell or core-base code.
- AREA (398): 332 all NOT_MINE (83%), 39 with an ownership answer, and 27
  COVERS only.
- The sample holds no load or stress capability and is recognisable from its
  `apps/desktop` paths. The comparison therefore cannot discriminate
  anchoring. It gives no sign of anchoring, but it is weak evidence either
  way.

**Anchored answers:** none identified. No sampled NOT_MINE answer denies a
capability whose entry point is the deliverable's own crate.

## 7. For the owner, stated plainly

1. **One firm false alignment.** DEL-05-01 VER-001 is marked ALIGNED although
   its protected-content review element is overtaken (F-1). The package rate
   is 2.5%, within the gate.
2. **No current protected-content review for primitive loads.**
   - The DEC-068 wind and seismic generators (2026-07-10) have had no
     boundary or protected-content review since 2026-06-05.
   - The rows are DEL-05-01 CLM-016/REQ-05-01-003 and, corrected, VER-001.
     Both are INVARIANT with routing REVIEW.
3. **Stale human-acceptance reuse is not enforced (INVARIANT, 9 rows,
   DEL-05-04).**
   - The status history and MEMORY say the 2026-08-20 R9 tranche closed
     this residual.
   - R9 in fact added a claimed-model-hash gate for model operations. Human
     acceptance records are bound to hashes only in schemas, and no runtime or
     negative test proves they stop applying after content changes.
   - The owner should say whether R9 was meant to close OPS-K-AUTH-2 for
     human acceptance records. If not, the obligation stays open until the
     OI-007 workflow exists.
4. **Pressure stress is held on an engineering decision (INVARIANT, owner
   hold).** The product publishes thin-wall membrane pressure stress. The
   reference model (thin-wall or exact annulus) and six companion decisions
   are open (DEL-05-03 RQ-001.s01 and CONTEXT#description).
5. **Possible defects (PROJECT_BASELINE):**
   - The algebra, stress and user-load findings lack the AB-00-06 class,
     remediation and provenance fields. The rows are DEL-05-02 REQ-05-02-008
     and CLM-004, DEL-05-03 RQ-005, and, not recorded as such, DEL-05-05 R6.
   - No rule-pack path can supply load combinations, although SOW-014 says
     rule packs supply them (DEL-05-02 REQ-05-02-004, routing OWNER).
6. **A software-chosen load category.**
   - The product applies user concentrated forces and moments under the
     "occasional" category, and distributed forces under "weight". It warns
     that this "is not a user-selected engineering classification".
   - No combination is created, but the category is a default the user did
     not choose. The owner of that surface is `product_physics` or DEL-05-01,
     not DEL-05-05.
7. **Ownership gaps for R3.**
   - The `user_loads` crate has no product caller. The product path runs
     through `primitive_loads` and `straight_pipe` (FG-DEL-05-05-01).
   - Self-weight planning (engine, WASM and UI) has no owning key.
   - Ownership of the bend and branch stress multipliers is unclear.
8. **Convention questions to settle at wave level:**
   - agent-produced validation witnesses against VERIFIED_NOT_VALIDATED
     (W-6);
   - the AC-001 overtaken-TBD clause (W-1);
   - the purpose-block OUT-001 and CP-09 (W-4);
   - SURFACE-row placement of CP-04 and pins (W-5);
   - there is no C5 layer for the professional boundary. FG-DEL-05-04-01 uses
     CLAIMS.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
