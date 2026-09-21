VERDICT: ACCEPT WITH CONTESTED ROWS

# PKG-04 verification (wave W2, sampling DOUBLE)

Fresh, evidence-only package verifier for run
`HELP-HUMAN-PIPING-20260921-RECONCILIATION`. Parent: HELP_HUMAN Agent 0.
Brief: `briefs/R2-VERIFIER_brief.md` (SHA-256 `47fb3c52…5b00`, checked and
matching). Evidence checkout: `00115c71931bcae79909602d653740d3bb72dfa1`
(freeze, read-only). Rules applied: `CONVENTIONS.md` (Parts A to F),
`CANONICAL_SITUATIONS.md`, `AUTHORITY_AND_SOURCE_RELIABILITY_MAP.md`. The
PKG-04 manager's transcripts and the worker returns were read for their flags
only. They were not used as evidence for any claim.

These are agent verification judgments. They are not owner rulings, and they
do not state or imply any release, approval, compliance or certification
(F-PIP-2; claims taxonomy per DEC-081).

## 0. Integrity and mechanical checks

- **Seals.** For all six ledgers, the forward SHA-256 I recomputed matches
  the `_SEAL.txt` line and the manager's return. The reverse hashes also
  match the return.
- **Single mode.** I ran `validate_ledger_v2.py` in single mode with
  `--forward --reverse --inventory ROUTING/PKG-04_capabilities.csv
  --notes-gap` against the freeze. All six ledgers pass with 0 findings:

  | Deliverable | Forward rows | Required keys | Canonical rows |
  |---|---|---|---|
  | DEL-04-01 | 80 | 75 | 7 |
  | DEL-04-02 | 64 | 62 | 7 |
  | DEL-04-03 | 71 | 56 | 7 |
  | DEL-04-04 | 72 | 70 | 7 |
  | DEL-04-05 | 71 | 69 | 7 |
  | DEL-04-06 | 84 | 82 | 7 |

- **Batch mode.** `--batch` over the six forward ledgers returns FAIL with
  2 consistency findings, the same as the manager's run. No
  `WAVES/W2/RESOLUTIONS.csv` exists. The two findings are resolved in §5.
- **Keyed canonical rows (CS-01, 02, 04, 06, 07).** The validator checks
  conformance on 100% of these rows. All conform.

## Sampling (deterministic)

Each class's candidate keys were sorted by SHA-256 of the full claim key, and
the lowest fraction was taken, rounded up. DOUBLE doubles the rates marked *:

| Class | Rate used |
|---|---|
| 100% classes | 100% |
| F7 `PRODUCT_CALLER: NONE` aligned rows | 50% |
| Other non-aligned rows | 50% |
| Aligned normative rows | 40% |
| Structural and other quiet rows | 20% |

How rows were assigned to classes:

- "INVARIANT" was read as `AuthorityTier=INVARIANT`.
- `SharedTextCount` was looked up in `CLAIM_KEYS_V2.csv`. A minted `.sNN`
  key takes its parent's count.
- Each row falls in the first class that matches, in the order shown in the
  tables below.

Two departures from the brief, disclosed:

- **Aligned normative rows.** These were selected by hash only, not weighted
  toward LOW or MEDIUM confidence.
- **The quiet-row class.** It covers structural rows, inherited canonical
  rows, and aligned `DECLARED_STATE` and `HISTORY` rows.

In total, 214 of 442 forward rows were sampled. The sampled keys are listed
per deliverable in §2. Beyond the sample, I also read every row of all six
ledgers once; findings outside the sample are reported separately in §4.2.

## 1. Package-level result

- **Firm disagreements on sampled rows: 0.**
- **Weak disagreements: 5** (plus 3 outside the sample).
- **Field disagreements on sampled rows: 0** (plus 1 outside the sample,
  which the worker disclosed itself).
- **Package firm false-alignment rate: 0 / 47 sampled `ALIGNED` normative
  rows = 0.0%** (gate ≤ 5%: met).
- No deliverable meets a rerun condition. No firm error exceeds 10%, and
  none falls in a 100%-sampled class that changes tier or owner routing.

## 2. Per-deliverable tables

Class key:

| Code | Class |
|---|---|
| C100 | 100% class (INVARIANT tier, UNKNOWN, PROTECTED_CHECK and similar) |
| SH | `SharedTextCount > 1` |
| F2/4 | Aligned rows with `GAP_WORDING_CHECKED` or `OPEN_ACTION` |
| F7 | Aligned rows with `PRODUCT_CALLER: NONE` |
| NA | Other non-aligned rows |
| AN | Aligned normative rows |
| Q | Structural and other quiet rows |

Cells read "total / sampled". "False-align" is the firm false-alignment rate
among sampled `ALIGNED` normative rows.

| Deliverable | Rows | C100 | SH | F2/4 | F7 | NA | AN | Q | Sampled | Firm | Weak | Field | False-align |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| DEL-04-01 | 80 | 2/2 | 7/7 | 0 | 1/1 | 31/16 | 17/7 | 22/5 | 38 | 0 | 2 | 0 | 0/8 = 0% |
| DEL-04-02 | 64 | 1/1 | 9/9 | 0 | 1/1 | 17/9 | 10/4 | 26/6 | 30 | 0 | 1 | 0 (+1 out of sample) | 0/5 = 0% |
| DEL-04-03 | 71 | 1/1 | 8/8 | 0 | 2/1 | 14/7 | 15/6 | 31/7 | 30 | 0 | 1 (+1 out of sample) | 0 | 0/6 = 0% |
| DEL-04-04 | 72 | 6/6 | 7/7 | 8/8 | 0 | 21/11 | 8/4 | 22/5 | 41 | 0 | 0 (+1 out of sample) | 0 | 0/10 = 0% |
| DEL-04-05 | 71 | 5/5 | 8/8 | 1/1 | 11/6 | 24/12 | 2/1 | 20/4 | 37 | 0 | 0 (+1 out of sample) | 0 | 0/5 = 0% |
| DEL-04-06 | 84 | 0 | 7/7 | 0 | 9/5 | 23/12 | 19/8 | 26/6 | 38 | 0 | 1 | 0 | 0/13 = 0% |

No deliverable is ISSUED, so there are no all-rows-100% deliverables.

Sampled keys (claim-key suffixes after `<DEL>:`):

- **DEL-04-01:**
  - CONTEXT surfaces: CONTEXT; CONTEXT#architecture-basis-injection (with
    .s01 and .s02); #decomposition-reference; #objective-support;
    #package-reference; #preparation-notes; #register-references.
  - MEMORY.
  - SOW blocks: CLM-002, 004, 006, 008, 009, 013, 016, 017, 020, 021, 023,
    025, 027.
  - SOW requirements: CLM-010/REQ-005, 006, 007, 009, 011, 012.
  - SOW verification rows: CLM-012.r01, r02, r03; CLM-012/REQ-005, 007,
    012.
  - SOW output matrix: output-and-evaluation-matrix/OUT-001.
  - STATUS; STATUS#history.
- **DEL-04-02:**
  - CONTEXT: #anticipated-artifacts; #architecture-basis-injection (with
    .s01 and .s02); #decomposition-reference; #description;
    #objective-support; #package-reference; #preparation-notes.
  - SOW blocks: CLM-003, 007, 013, 014, 015, 017, 019, 021, 023, 024, 025,
    028.
  - SOW requirements: CLM-011/RQ-001, 003, 004, 005.
  - SOW other: AC-001; deliverable-definition-ontology; matrix OUT-001;
    VER-001.
  - STATUS#remaining.
- **DEL-04-03:**
  - CONTEXT: CONTEXT; #architecture-basis-injection (with .s01 and .s02);
    #context-envelope; #decomposition-reference; #description;
    #objective-support; #package-reference; #preparation-notes;
    #register-references.
  - SOW: SOW; purpose-and-objective-traceability.
  - SOW blocks: CLM-003, 004, 006, 007, 011, 012, 013, 018, 020, 021, 028.
  - SOW requirements: CLM-010.r03, r04, r06, r07.
  - SOW other: AC-001.
  - STATUS#remaining.
- **DEL-04-04:**
  - CONTEXT: CONTEXT; #anticipated-artifacts;
    #architecture-basis-injection (with .s01 and .s02);
    #decomposition-reference; #description; #objective-support;
    #package-reference; #preparation-notes.
  - MEMORY.
  - SOW: SOW; production-and-verification-method-praxeology.
  - SOW blocks: CLM-002, 003, 004, 005, 012, 016, 018, 020, 021, 022, 023,
    024, 025.
  - SOW requirements: REQ-01 to REQ-09 (all).
  - STATUS; STATUS#remaining; R01, R02, R03, R06.
- **DEL-04-05:**
  - CONTEXT: CONTEXT; #anticipated-artifacts;
    #architecture-basis-injection (with .s01 and .s02);
    #decomposition-reference; #package-reference; #preparation-notes;
    #register-references.
  - SOW: SOW; purpose-and-objective-traceability; its OUT-001.
  - SOW blocks: CLM-001, 002, 003, 005, 007, 010, 011, 013, 014, 015, 016,
    021, 022, 023, 026, 027, 031.
  - SOW requirements: CLM-012/RQ-001, 003, 004, 005, 006.
  - SOW other: AC-001.
  - STATUS; STATUS#remaining/R01.
- **DEL-04-06:**
  - CONTEXT: #architecture-basis-injection (with .s01 and .s02);
    #context-budget-qa; #decomposition-reference; #description;
    #package-reference; #preparation-notes; #scope-coverage.
  - MEMORY.
  - SOW: SOW; purpose-and-objective-traceability; its OUT-001;
    output-and-evaluation-matrix; VER-001.
  - SOW blocks: CLM-002, 004, 007, 008, 014, 016, 017, 022, 023, 028, 033.
  - SOW requirements: CLM-013/REQ-002, 003, 004, 008.
  - SOW verification rows: CLM-015/REQ-002, 004, 005, 006, 008, 011.
  - STATUS#history; STATUS#remaining.

## 3. Evidence spot-checks against the freeze (sampled substance)

Code and records I read at the freeze to test the rows:

- **Frame kernel (DEL-04-01).**
  - `frame_kernel` has 37 `#[test]` functions (DEL-04-01 CONTEXT#anticipated-artifacts, REQ-010).
  - `FrameKernelUnitBasis` has no product caller. It is used only by the harness, `straight_pipe` and an audit harness. This supports REQ-007's `PRODUCT_CALLER: NONE`.
- **Product diagnostic payload (REQ-011).** `product_physics::Diagnostic` carries only `id, code, severity, message, source, affected_refs`. This confirms DEL-04-01 REQ-011's PARTIALLY_IMPLEMENTED on the desktop service boundary.
- **Sparse default (CLM-014 and CLM-021).** `product_physics` records `default_sparse_promotion=interactive_default` (DEC-053). This confirms the stale "sparse-as-default held" text.
- **Straight pipe (DEL-04-02).**
  - The crate has 34 tests.
  - `weight_hook` and the station-sweep functions have no product caller.
  - `recover_local_forces_from_global_model` and `equivalent_*_loads_with_spans` are product-called.
- **Linear supports (DEL-04-03).**
  - The crate has 15 tests. This confirms the "14 tests" text is stale (CLM-005, CLM-018).
  - `apply_linear_supports` is called only by one product unit test and the benchmarks, not the product solve. This bears on r12.
  - The SOW names `open_pipe_stress_frame_kernel` 4 times. The other five PKG-04 SOWs name it 0 times, which is consistent with CP-04 on DEL-04-03's SOW surface only.
- **Removed protected check (DEL-04-04 REQ-08).**
  - Commit `b43cc00c4` deletes `#[test] nonlinear_bearing_solve_attaches_validated_envelope_with_context` and its siblings from `core/runner/headless/src/result_envelope_binding.rs`. Two tests remain.
  - `b43cc00c4` is an ancestor of PR #787's merge commit `eff9a58dd` (read-only `gh`: MERGED).
  - Outside `execution/`, `NONLINEAR_ASSEMBLED_LOOP` appears only in the producer source.
  - This confirms VERIFICATION_REMOVED · PROTECTED_CHECK · INVARIANT.
- **DEL-04-04 REQ-03.** The remaining headless test asserts `NONLINEAR_SUPPORT_NONCONVERGENCE` for `gap-chain-five-active`, which supports ALIGNED.
- **DEL-04-04 REQ-01.** Its own verification column requires a solver-boundary test for "no local FEA substitution". None was located, so F1 applies correctly.
- **Performance harness (DEL-04-05).**
  - Every cited test exists (`rejects_protected_or_unknown_fixture_provenance`, `conditioning_observation_is_recorded_without_threshold_policy`, and the others).
  - The README states that fixture unit metadata is "for reproducibility only", with no conversion. This supports the RQ-006 and CLM-021 PARTIALLY_IMPLEMENTED · INVARIANT rows.
  - The A3 discovery grep found no human or IP review record for the fixtures, which is consistent with the CLM-013 UNKNOWN.
- **Diagnostics crate (DEL-04-06).**
  - The crate has 24 tests; the 2026-06-05 record says 19 (CLM-008 and CLM-015/REQ-001 are EVIDENCE_OVERTAKEN).
  - `SolverDiagnostic` has exactly the ten fields REQ-002 lists.
  - `with_canonical_ref` and `with_quantity_unit` are used only inside the crate. `classify_condition_ratio` is called only by the harness. These support the F7 markers.

Among the sampled rows, no `ALIGNED` row records an unmet element of its own
claim (F1). Every sampled F2/F4 row is sound:

- Each `GAP_WORDING_CHECKED` clause is the last clause of Notes.
- Each one explains a support-category or test-name use of "gap" or
  "missing".
- The two `OPEN_ACTION` targets (DEL-04-04 REQ-06 and DEL-04-05 RQ-006)
  carry the open work their Remaining rows name.

Sampled F7 rows are all about the engine or library itself (C6(b)), so
`PRODUCT_CALLER: NONE` with ALIGNED is correct.

## 4. Disagreements

### 4.1 Sampled rows (all weak; none firm)

**W-1. PKG-00 SEMANTIC_READY sub-claim: shared situation judged two ways.**

Keys: `DEL-04-01:CONTEXT#architecture-basis-injection.s01`, and the same key
in DEL-04-02 and DEL-04-03.

- **What the rows say:** STALE_SETUP_SPECIFICATION · SCOPE_REDIRECTED_BY_RULING
  · LOCAL_DESIGN · NONE · RECORD. The worker invoked F3's origin test
  (text first present at `7bee9ae41`).
- **What I found:** the same text in DEL-04-04, DEL-04-05 and DEL-04-06 is
  STALE_REVIEW_OR_EVIDENCE · RECORD_DRIFT, under F3's exception that review
  and lifecycle states stay STALE_REVIEW_OR_EVIDENCE whatever their origin.
  - All eight PKG-00 `_STATUS.md` files read IN_PROGRESS at the freeze, so
    both readings agree the claim is stale.
  - Across W2 the split is: PKG-01, 02, 03 and 07 mostly use
    STALE_REVIEW_OR_EVIDENCE; PKG-03 has 5 STALE_SETUP_SPECIFICATION rows;
    PKG-04 is split 3 and 3.
- **Right values (recommended):** STALE_REVIEW_OR_EVIDENCE, because
  "SEMANTIC_READY" is a lifecycle state and F3's exception names review
  states. The cause (SCOPE_REDIRECTED_BY_RULING via D-43, or RECORD_DRIFT)
  should be settled wave-wide.
- **Impact:** tier and layer are unchanged (LOCAL_DESIGN, RECORD), and so is
  the owner routing (NO).
- **Why batch mode missed it:** minted `.sNN` keys have no `BodySHA256`
  (§7).

**W-2. `DEL-04-01:SOW#CLM-021` (and its duplicate `SOW#CLM-014`, outside the
sample), the batch finding.**

- **What the rows say:** STALE_REVIEW_OR_EVIDENCE · DOC_BEHIND_CODE ·
  LOCAL_DESIGN · RECORD, with `CanonicalSituation=CP-03`.
- **What I found:** the disposition, tier and layer are right. The
  declaration says sparse-as-default "remain[s] held". Product code records
  `default_sparse_promotion=interactive_default`, and the worker dates the
  promotion to 2026-06-22, which is before the declaration.
- **Why the batch majority does not apply:** the majority fields
  (BASIS_POINTER_STALE) come from CP-03 rows that pin superseded revisions.
  CP-03 prescribes CP-02 fields only for such pins. So the batch "majority"
  is not a canonical requirement.
- **Where the weakness is:**
  - The cause for the same DEC-053-overtaken sparse text differs across the
    package. DEL-04-01 uses DOC_BEHIND_CODE; DEL-04-04 CLM-003 and REQ-09,
    and DEL-04-06 CLM-004 and REQ-010, use SCOPE_REDIRECTED_BY_RULING.
  - Both rows lack `CANONICAL_DEPARTURE`.
- **Right values:** keep as sealed. Resolve the batch pair as a non-conflict
  in `RESOLUTIONS.csv` (F6). R3 may harmonize the cause.

**W-3. `DEL-04-06:SOW#production-and-verification-method-praxeology/VER-001`,
CP-09 applicability.**

- **What the row says:** PARTIALLY_IMPLEMENTED · PARTIAL_SLICE ·
  LOCAL_DESIGN · RECORD, AuthorityNeeded REVIEW. Its grounds are that parity
  does not match the frozen SOW and the Review_Findings human disposition is
  pending.
- **What I found:** group G1 treats the structurally identical VER-001 rows
  (DEL-04-01 to 03) as CP-09 SOW-parity rows: STALE_REVIEW_OR_EVIDENCE ·
  EVIDENCE_OVERTAKEN.
- **Why this is weak:** both readings are defensible. VER-001 names source
  parity and a broader review, and C7 says the remaining gap's cause wins.
  Whether VER-001 is a CP-09 row should be settled by convention.
- **Impact:** tier and routing are the same apart from AuthorityNeeded
  (REVIEW versus NO).

### 4.2 Outside the sample (read, not counted in the rates)

- **Field. `DEL-04-02:SOW#CLM-010` (ALIGNED).** It should carry
  `PRODUCT_CALLER: NONE` for the station-sweep, weight-hook and
  boundary-metadata surfaces, which have no product caller at the freeze. The
  worker disclosed this itself. The disposition stands.
- **Weak. `DEL-04-03:SOW#CLM-010.r12`.** It is IMPLEMENTED_DIFFERENTLY ·
  OWNERSHIP_ELSEWHERE, because the product does not route support
  application through `apply_linear_supports`.
  - Under F7, a claim about the crate's "current implemented surfaces" could
    be read as a library claim (ALIGNED with `PRODUCT_CALLER: NONE`). That is
    how CLM-003 and CLM-025 of the same ledger treat `apply_linear_supports`.
  - The reading the worker chose (the product dense path) is defensible.
    There is an internal tension, not an error.
- **Weak. `DEL-04-04` and `DEL-04-05` VER-001.** These raise the same CP-09
  question as W-3. For DEL-04-04, a PASS parity record matches the frozen SOW,
  so under CP-09 the row would be ALIGNED. The worker's PARTIALLY_IMPLEMENTED
  is the more conservative reading, not a false alignment.

## 5. Batch consistency and shared situations

- **Batch mode:** 2 findings (DEL-04-01 CLM-014 and CLM-021 against the
  CP-03 majority). I resolve them as a non-conflict (W-2). No resolutions
  file existed.
- **Shared bodies (`SharedTextCount > 1`, 100% reviewed, 46 rows):**
  - The CS-01, CS-02, CS-04 and CS-06 keyed rows conform.
  - The empty-Remaining rows (DEL-04-02, 03 and 06) are consistently
    NON_NORMATIVE · NOT_ASSESSED.
  - The conflict-table rows are consistently ALIGNED.
  - `.s02` (solver library still TBD) is STALE_SETUP_SPECIFICATION ·
    DOC_BEHIND_CODE in all six ledgers.
- **Shared-situation conflict not caught by batch:** `.s01` SEMANTIC_READY,
  judged 3 against 3 in this package (W-1). This conflict also runs across
  W2 packages.
- **Intra-package pattern divergence:** VER-001 (CP-09 or not) splits by
  worker group (W-3).

## 6. Reverse pass

Coverage checked:

- 100% of CLAIMED_BY (37), PARTIAL (16), UNKEYED (4) and CONSTRAINS (0)
  answers.
- 10% of NOT_MINE: 197 of 1,965 (deliverable × capability), chosen by the
  lowest SHA-256 of `DEL|RC-ID`.
- F5 on 20% of NOT_MINE rows whose EntryPoints hit a path the deliverable's
  forward ledger cites: 48 of 225 candidates.

Routing-local IDs were resolved through `ROUTING_SAMPLE/SAMPLE_MANIFEST.csv`.
PKG-04 has 340 routed rows: 314 AREA and 26 SAMPLE. By area: SOLVER 70,
DATA 70, DOCS 52, PHYS 44, CHECKS 42, FEATC 36, others 27.

**Findings:**

- **Capabilities claimed by more than one PKG-04 deliverable:**
  - RC-04-0025 (CAP-SOLVER-070, the bundle of solver READMEs) is PARTIAL
    from all six deliverables. Each claims only its own crate's README, so
    this is a legitimate split, not contested ownership.
  - No other capability is CLAIMED_BY or PARTIAL in two deliverables.
  - Cross answers are consistent:
    - RC-04-0049: DEL-04-01 COVERS, DEL-04-06 PARTIAL.
    - RC-04-0209: DEL-04-01 CLAIMED_BY, DEL-04-06 COVERS.
- **Suspected missed claims or ownership gaps:**
  - **The `nonlinear_integration` crate (DEC-044 assembled loop) has no
    owning key.** DEL-04-04 answers PARTIAL on RC-04-0149, 0157, 0063 and
    0298, while its own reasons say DEC-044 places the loop outside
    DEL-04-04. A strict reading makes these COVERS answers plus an R3
    ownership gap rather than PARTIAL. This is weak.
    - RC-04-0110 (macro-element slots in the loop) is NOT_MINE in all six
      deliverables.
    - RC-04-0045 (per-iteration solve-mode selection) is PARTIAL from
      DEL-04-01.
    - The crate is fragmented across answers; R3 should treat it as one
      unowned capability cluster.
    - The manager's return calls this "UNKEYED"; the reverse files in fact
      use PARTIAL.
  - **The `curved_bend` crate (RC-04-0082, 0119, 0155, 0244) is UNKEYED
    against DEL-04-01 AC-001.** This is supported by four
    `TP-PMM-P1-CURVEDBEND-00x` run records in the DEL-04-01 folder. It is a
    scope finding for R3.
  - The SOLVER-area rows that are all NOT_MINE are `core/loads/*` and
    `core/section_properties/*`, which belong to PKG-05. None looks like a
    missed PKG-04 claim.
  - In the 10% NOT_MINE sample, no answer denies a capability whose entry
    point is the deliverable's own crate. The 7 own-path hits across all
    NOT_MINE rows are substring false positives (`linear_supports` inside
    `nonlinear_supports`) or `core/loads` callers.
- **F5.** All 48 sampled overlap reasons name the specific capability and
  say why the cited path does not confer ownership. None is a bare template.
  - DEL-04-02, 03 and 06 reuse a per-deliverable tail ("cites product_physics
    only for …"), but each opens with a capability-specific clause, which
    satisfies F5.
- **Sampled rows compared with area rows:**
  - SAMPLE (26): all NOT_MINE in all six deliverables (100%). Every sampled
    capability is desktop, shell, UI, rule-pack, reporting or security code.
  - AREA (314): 251 all NOT_MINE (80%), 52 with an ownership answer, 11
    COVERS only.
  - The sample holds no solver-relevant capability and is recognisable from
    its `apps/desktop` paths, so it cannot discriminate anchoring. The
    comparison gives no sign of anchoring, but it is weak evidence either
    way.
- **Anchored answers:** none identified.

## 7. For the owner, stated plainly

1. **A removed protected check.** Merged PR #787 (commit `b43cc00c4`)
   deleted the envelope-level tests that proved nonlinear loop identity and
   the `NONLINEAR_ASSEMBLED_LOOP` assumption and limitation diagnostics reach
   the exported result. No test now covers that. See DEL-04-04 REQ-08,
   PROTECTED_CHECK · INVARIANT, AuthorityNeeded REVIEW.
2. **No current protected-content review.** None covers the frozen solver
   crates: the last dedicated checks date from 2026-06-05 or earlier, and no
   DEC-058 scan record exists. Rows: DEL-04-01 CLM-012/REQ-012 and CLM-020;
   DEL-04-02 CLM-021; DEL-04-03 CLM-012; DEL-04-05 RQ-004 and CLM-013
   (UNKNOWN). All are INVARIANT (IP/data), with AuthorityNeeded REVIEW.
3. **Unit safety is only partly met.** For the nonlinear classifier and the
   performance-harness fixtures it is at metadata grain, with no dimensional
   checks or rejected-metadata tests, and the human review dispositions are
   pending (DEL-04-04 CLM-004 and REQ-06; DEL-04-05 RQ-006 and CLM-021).
   These rows are INVARIANT.
4. **Two ownership gaps for R3.** The `nonlinear_integration` loop has no
   owning key. The `curved_bend` crate was built under DEL-04-01 but no
   DEL-04-01 key covers it.
5. **A validator blind spot.** Batch mode cannot see minted `.sNN`
   sub-claims that share text across deliverables, because they have no
   `BodySHA256`. The `.s01` SEMANTIC_READY sub-claim is judged differently
   within PKG-04 and across W2. A wave-level resolution, and hashing minted
   sub-claims by their quoted text, would close this.
6. **Rename residue in active code not attached to any row.**
   `core/solver/diagnostics/src/lib.rs:79` emits the source name
   "OpenPipeStress solver diagnostics". The DEL-04-06 SOW does not name it,
   so CP-04 does not attach. The worker noted it for R3.
7. **A code inconsistency.** `default_remediation(SparseSolverTbd)` still
   asks for the "governed default sparse-solver promotion" that DEC-053
   already made (DEL-04-06 notes). A code-change candidate, not a ledger
   error.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
