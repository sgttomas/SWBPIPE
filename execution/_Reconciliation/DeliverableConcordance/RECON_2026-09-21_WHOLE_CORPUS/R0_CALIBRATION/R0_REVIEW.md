# R0 calibration review — RECON_2026-09-21_WHOLE_CORPUS

Reviewer: TASK (Type 2), fresh context, evidence-only, run
`HELP-HUMAN-PIPING-20260921-RECONCILIATION`. Brief SHA-256
`6d376ce98c7369208fec1af464137587c9ee11a2ed2ff06e7a6b8dcbd85b4f46`. Code and
documents were read from the evidence checkout at
`00115c71931bcae79909602d653740d3bb72dfa1`. No ledger was edited. No build, test
or network access was used. This review gives agent judgments, not owner
rulings. Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

**Verdict: READY WITH NAMED AMENDMENTS**

The workers' evidence is sound. All 576 cited `::case` symbols exist in the
frozen tree, every spot-checked substantive finding reproduced, and all eight
ledgers pass the validator. The conventions are not yet fit to scale, for two
reasons (F1, F2). First, A3 as written stops run records from counting as
evidence. Second, the same situation gets different dispositions, cause tags
and tiers from different workers. At 102 deliverables, R4 counts would be
noise. Both problems can be fixed by amending the conventions. Calibration does
not need to be rerun, but the amended rules should first be tried on a small
first wave (§7).

---

## 1. Decisions for the owner

1. **Split authority from evidence in A3 (blocking).** A3 classes AgentRuns
   records as context, and context "never changes a claim's Disposition". But
   run records hold the only parity reports, claim maps and review returns.
   Applied literally, DEL-11-01 could not credit an existing PASS parity report.
   Applied loosely, DEL-01-01 and DEL-12-03 used such records to reach `ALIGNED`.
   *Proposed:* context can never supply authority or permission, while any
   frozen-tree record can supply evidence. Exact text is in §4 (A3).
2. **Owner-direction records.** May a verbatim, hash-bound owner decision in an
   AgentRuns record (DEL-11-01's R18 guide-edit exception) support
   `ACCEPTED_DIVERGENCE`? Yes routes such rows to confirmation at R4. No keeps
   them as open divergences. Either answer works; one must be chosen. Optional
   text is in §4 (A3a).
3. **Rename reach.** Does DEC-101's "other governance documents take the name
   at their next amendment" cover deliverable `ScopeOfWork.md` files? One
   answer decides about 100 deliverables. The eight workers used four
   different treatments (F2).
4. **ISSUED text overtaken by later rulings (DEL-01-01).** The SOW was
   issued with maintainer, quorum and signing values as TBD, and DEC-027 and
   DEC-057 later filled them. Two options:
   - one `LIFECYCLE_REASSESSMENT_REQUIRED` finding group (proposed);
   - eight separate `IMPLEMENTED_DIFFERENTLY` rows, each needing a scope
     change (as sealed).
5. **Canonical situation table.** Authorize Agent 0 to publish a table of
   recurring situations, each with one required disposition, cause tag, tier
   and layer. It would be used in R2, and workers could depart from it only
   with a justification in Notes. It lives inside the candidate conventions, so
   it binds only after your R0 ruling. The table would also pre-disposition
   identical-text units once: 605 units carry text identical to another
   deliverable's.
6. **Re-issue claim keys under extractor v2** (§5). This gives a new
   `CLAIM_KEYS.csv` hash. It cuts roughly 20–25% of rows and replaces
   worker-minted `.sNN` sub-claims with deterministic keys.
7. **Rebuild the reverse inventory for R1** (§6). The pilot inventory held no
   capability for the implementation of five of the eight pilots, so the
   reverse pass gave them no ownership signal.

Decisions 1, 5 and 6 should precede R2 dispatch. Decisions 2–4 can be ruled
together and applied in the waves.

---

## 2. Per-deliverable results

The validator was run once per deliverable on the forward and reverse files
together, against `PILOT_CAPABILITIES.csv`, with `--repo-root` set to the
evidence checkout.

- The disagreement counts cover dispositions and material field or evidence
  errors.
- The false-alignment column counts, among the `ALIGNED` normative rows I
  spot-checked:
  - "firm": rows I would re-dispose;
  - "weak": rows defensible only under a reading that the amended conventions
    should settle.

| Deliverable | Structural result | Rows (issued + `.sNN`) | Non-aligned | Spot-checked | Disagreements | False alignment (sampled ALIGNED) |
|---|---|---|---|---|---|---|
| DEL-04-04 | PASS, 0 findings (reverse 96/96) | 70 (70 + 0) | 21 | 16 (12 non-aligned, 4 aligned or structural) | 2 | 0 of 4 (0%) |
| DEL-07-02 | PASS, 0 findings | 88 (83 + 5) | 26 | 15 (12, 3) | 4 | 0 firm, 1 weak of 3 |
| DEL-00-05 | PASS, 0 findings | 39 (29 + 10) | 10 | 13 (10, 3) | 1 | 0 of 3 (0%) |
| DEL-07-09 | PASS, 0 findings | 109 (93 + 16) | 24 | 15 (12, 3) | 3 | 0 of 3 (0%) |
| DEL-01-01 (ISSUED) | PASS, 0 findings | 85 (68 + 17) | 25 | 15 (12, 3) | 3 | 1 firm of 3 |
| DEL-12-03 | PASS, 0 findings | 92 (76 + 16) | 25 | 14 (11, 3) | 2 | 0 firm, 1 weak of 3 |
| DEL-11-01 | PASS, 0 findings | 92 (66 + 26) | 28 | 15 (11, 4) | 4 | 2 firm of 4 |
| DEL-17-05 | PASS, 0 findings | 113 (104 + 9) | 43 | 14 (11, 3) | 3 | 0 firm, 2 weak of 3 |
| **Total** | 8/8 PASS | 688 | 202 | 117 | 22 | 3 firm + 4 weak of 26 (12% firm, 27% incl. weak) |

`COVERED_BY_CHILDREN` and `NOT_ASSESSED` rows: I sampled the heading, section
and container blocks. I found no hidden normative claim, because every CLM
block is keyed separately. Section wrappers, however, are typed inconsistently
(F2).

---

## 3. Findings

**F1 — BLOCKING. A3 conflates authority with evidence, and evidence discovery
stops at the deliverable folder.** Parity reports for the SOW conversion exist
at the root, under `execution/_Coordination/AgentRuns/SOW-STAGE2-EXEC-20260712-01/instances/`:

- `CHANGE-P2/checks/DEL-07-02/parity.md`: PASS, 37 mappings;
- `CHANGE-P3/checks/DEL-11-01/parity.md`: PASS, 28 mappings;
- `CHANGE-P4/checks/DEL-17-05/parity.md`: PASS, 42 mappings;
- `CHANGE-P1-PKG04/checks/DEL-04-04/parity.md`;
- `CHANGE-P3/checks/DEL-12-03/parity.md`.

Three workers reported "no parity report found":

- DEL-07-02 OUT-001 (`UNKNOWN`);
- DEL-11-01 OUT-001 and CLM-021/VER-001 (`PARTIALLY_IMPLEMENTED`);
- DEL-17-05 OUT-001 (`UNKNOWN`).

Only DEL-04-04's frozen SOW hash matches its parity production hash (`23488618…`).
The other four SOWs changed after parity, so the right disposition is
`STALE_REVIEW_OR_EVIDENCE`, not "not found". The workers split on how to treat
these records:

- DEL-01-01 cites AgentRuns records as `ImplementationEvidence` to reach
  `ALIGNED` (OUT-001) and `ACCEPTED_DIVERGENCE` (AC-001);
- DEL-11-01 treats a verbatim owner adoption record as context only.

Both readings of A3 are defensible, and they give opposite results.

**F2 — BLOCKING. Workers classify shared situations inconsistently.**
Verified examples:

- *`CONTEXT#decomposition-reference`* ("rev 0.7 current_basis"; identical text
  in 99 deliverables):
  - disposition `STALE_SETUP_SPECIFICATION` everywhere;
  - cause `RECORD_DRIFT` ×5, `EVIDENCE_OVERTAKEN` ×2;
  - tier `PROJECT_BASELINE` ×6, `LOCAL_DESIGN` ×1;
  - claim type `DECLARED_STATE` ×5, `CONTEXT` ×2.
- *`CONTEXT` surface:* cause `DOC_BEHIND_CODE`, `EVIDENCE_OVERTAKEN` or
  `RECORD_DRIFT`; tier split 4/3.
- *Four-document to SOW migration residue:* cause `RECORD_DRIFT` (04-04,
  07-02, 12-03), `OTHER` (01-01 ×3, 17-05 ×1), `DOC_BEHIND_CODE` (17-05) and
  `CONTRACT_VERSION_ADVANCED` (11-01 ×5).
- *Identical Remaining text in DEL-07-02 and DEL-07-09:*
  - the ROOT-acceptance/PR #789 item is `PROJECT_BASELINE`/`RULED_CRITERION`
    in 07-02 R03 and `LOCAL_DESIGN`/`NONE` in 07-09 R03;
  - the AX-omission item has cause `OTHER` in 07-02 R04 and
    `EVIDENCE_OVERTAKEN` in 07-09 R04;
  - the missing `HISTORICAL_N7_ACCEPTANCE_INTAKE_V1.json` (I confirmed it is
    absent) is `REMAINING_STATE_MISMATCH` in 07-02 R11 and
    `STALE_REVIEW_OR_EVIDENCE` in 07-09 R09.
- *`_STATUS.md` "Last Updated" older than its own history:*
  `REMAINING_STATE_MISMATCH` (07-02), `STALE_REVIEW_OR_EVIDENCE` (01-01,
  11-01), and `ALIGNED` (12-03, whose notes name the same drift).
- *Output matrix `OUT-001`* (the same kind of row in six SOWs): `ALIGNED` ×3,
  `UNKNOWN` ×2, `PARTIALLY_IMPLEMENTED` ×1.
- *Section wrappers* (`#deliverable-definition-ontology` and siblings):
  `CONTAINER` in 4 deliverables, `NON_NORMATIVE` in 2.
- *Rename residue:*
  - no finding (04-04, 01-01);
  - noted but not a finding (07-02);
  - once on the surface row (11-01);
  - `ACCEPTED_DIVERGENCE` on three rows (12-03).

**F3 — MAJOR. `DivergenceLayers` has two meanings.** DEL-04-04, 07-09 and
17-05 use `CLAIMS` to mean "the claim text diverges" (78 rows). DEL-07-02,
00-05, 12-03 and 11-01 use `NONE` for the same kind of row (76 rows) and keep
`CLAIMS` for the claims-language boundary. As used, the field cannot be
aggregated.

**F4 — MAJOR. `AuthorityNeeded` and `SourceReliability` are inconsistent.**
For comparable stale-pointer rows, DEL-07-09 uses `NO` on 23 of 24 non-aligned
rows and DEL-11-01 uses `OWNER` on 28 of 28. DEL-00-05 uses `NO` on its
`_CONTEXT.md` blocks but `OWNER` on its surface row. DEL-12-03 marks 35 rows
that cite agent-run tests `UNVERIFIED`, while the other workers mark such rows
`NOT_APPLICABLE`.

**F5 — MAJOR. Gate evidence has three homes.** Workers cited `GATE_EVIDENCE/`
in `ContextRefs` (six workers), in Notes only (00-05), or in
`VerificationEvidence` (11-01). The validator passes 11-01's version silently,
because run-relative tokens fall outside its path-prefix list. Seven of eight
workers raised this as friction.

**F6 — MAJOR. ISSUED handling (DEL-01-01).**

- The DEC-027/057 "TBD filled later" finding is repeated on eight rows as
  `IMPLEMENTED_DIFFERENTLY` with `AuthorityNeeded=SCOPE_CHANGE`.
- `BaselineClass=ISSUED` is set on all 25 non-aligned rows. That includes
  `_CONTEXT.md`, `_STATUS.md` and `governance/MAINTAINERS.md` findings, none of
  which is the ISSUED artifact.
- OUT-001 is `ALIGNED` while AC-001 records that DEC-081 Wave 2 changed the SOW
  after parity. The two rows contradict each other.
- DEL-11-01 read a similar conditional TBD clause as `ALIGNED`.

The worker also found two real divergences, both of which I verified:

- `governance/MAINTAINERS.md` L35–36 still say TBD against DEC-027;
- PRD L24 still says "free and open-source" although L44 says that framing was
  superseded.

**F7 — MAJOR. Reverse-pass design and anchoring.**

- The pilot inventory (96 capabilities in five areas) has no capability for:
  - the DEL-07-09 palette and catalog;
  - the DEL-12-03 desktop seam and panel;
  - DEL-17-05's `core/handoff/caepipe_external`;
  - any documentation or governance surface (DEL-01-01, DEL-11-01).

  Three workers answered `NOT_MINE` to all 96 without that being evidence of
  anything.
- DEL-07-02 anchored nine answers to `STATUS#remaining/R01|R05|R09` keys, which
  contradicts A4: CAP-TREE-004, -011, -014, -016 and CAP-WS-022, -028, -030,
  -031, -034.
- DEL-07-09's twelve `PARTIAL` answers are coverage-row references, not
  ownership, and B2 has no answer for that relation.
- DEL-04-04 marks CAP-SOLVER-020 `CLAIMED_BY` but its sibling
  `nonlinear_integration` capabilities (016, 017, 019) `PARTIAL`.

**F8 — MAJOR. Some `ALIGNED` rows are vacuous or unsupported.**

- DEL-17-05 REQ-001 and REQ-004 are "met by construction": no invocation code
  exists and nothing fills the fields.
- DEL-11-01 CLM-013.s03 ("manual protected-content review finds no protected
  data") is `ALIGNED` with no evidence and no note. DEL-07-02 RQ-008 is
  `UNKNOWN` for the same situation.
- DEL-11-01 CLM-020.s03 and .s04 assert validator results without a rerun or a
  cited record. CLM-013.s05 did note "not rerun". A6 is breached in spirit.

**F9 — MAJOR. The grain carries dead weight and produces keys that cannot be
reproduced.** About 28% of the 688 rows carried no audit value:

- 110 structural rows;
- 63 trivially aligned `_CONTEXT.md` identity blocks;
- 21 aligned history rows.

On top of that:

- The workers minted 99 `.sNN` sub-claims with different split thresholds.
- The extractor mis-parents `AC`/`VER` items (DEL-04-04 notes 3) and parents
  CLM blocks to the surface rather than their section.
- DEL-07-09's two CSVs produce 28 mirrored row pairs.
- PDU-055 declarations repeat up to four times per SOW.

**F10 — MINOR. Validator gaps.**

- Tokens outside `PATH_TOKEN_RE` are not checked: `execution/` at the root,
  `agents/`, `GATE_EVIDENCE/`.
- `ContextRefs` is not checked.
- There is no check that reverse keys are not `STATUS#remaining`.
- The `BaselineClass` default is not defined for non-aligned rows.
- `DivergenceLayers` separators are inconsistent (`; ` in DEL-01-01).

Positive: I checked all 576 `::case` citations and 1,993 path tokens. Every
case exists. The only path failures are tokens with trailing free text, and
their paths exist.

**F11 — MINOR. Cause-tag fitness.** `OTHER` was used 7 times: representation
migration ×4 and evidence not located ×3. `EVIDENCE_OVERTAKEN` is stretched to
cover stale revision pointers. `RECORD_DRIFT` absorbs setup residue. See §4 C7.

**F12 — MINOR. Tier and baseline slips.**

- DEL-17-05 CLM-019 (`VERIFIED_NOT_VALIDATED`) is `PROJECT_BASELINE`, but C3
  lists engineering validation as `INVARIANT`.
- DEL-07-09 `PALETTE_OPERATION_ROUTING#verification-and-handoff` uses
  `PROTECTED_CHECK` for a stale test count, which is not a protected-check
  event.
- DEL-12-03's rename rows are `ACCEPTED_DIVERGENCE` with `AuthorityNeeded=NO`,
  although the worker says DEC-101's reach is an owner reading.

**F13 — MINOR. Context steered one disposition.** DEL-11-01's `STATUS` row
cites the R18 owner-adoption record as the reason it raises no lifecycle
reassessment (A3).

**F14 — Positive. Authority handling otherwise held.**

- No row changes a disposition on the strength of a merged PR alone. PR merge
  state is used only as fact, for example "a pre-merge condition is past".
- `ACCEPTED_DIVERGENCE` appears 4 times, each with a named ruling:
  - DEL-01-01 AC-001 on DEC-081. Its Wave 2 explicitly scopes "45 live
    `ScopeOfWork.md` litany occurrences", so I agree at MEDIUM.
  - DEL-12-03 ×3 on DEC-101, pending decision 3.
- `ADOPTED_BY_REFERENCE` was flagged where D-68 and DEC-074 O7 apply.

Spot-check reproductions:

- DEL-12-03 TEL-REQ-008: the telemetry diagnostic lacks the `class`, `source`,
  `affected_object` and `provenance` fields that `schemas/model.schema.yaml`
  requires.
- DEL-12-03 CLM-005.s05: the whole event is rejected with
  `telemetry_forbidden_field`.
- DEL-17-05 REQ-016: a substring test on `del-17-04`, and `REQUIRED_MBF_REF` is
  unused.
- DEL-04-04 REQ-08: no test outside `execution/` references the
  `NONLINEAR_ASSEMBLED_LOOP_*` codes, and `result_envelope_binding.rs` holds 2
  tests. The row should also cite the `product_physics`
  `assembled_loop_context_is_a_pure_passthrough` test as partial verification.
- DEL-07-09 R02: D-70 does not mention DEL-07-09.
- DEL-00-05 `resolved-decisions.s01`: `SOFTWARE_DECOMP.md` has no §8.4.

---

## 4. Convention dispositions

| Conv. | Disposition | Exact proposed text (for amendments) |
|---|---|---|
| A1 | Keep | — |
| A2 | Amend | Append: "A pull request's merge state and commit are facts and may be cited as evidence of what landed and when. A merged PR never supplies the permission that `ACCEPTED_DIVERGENCE` requires." |
| A3 | **Amend (blocking)** | "**A3. Authority versus evidence.** *Authority* (what is wanted) comes only from the governing sources in `RUN_BASIS.md`. Context records (AgentRuns records, owner-direction records, plans, design frames, handoffs, merged PRs, run records, `MEMORY.md`) never create, remove or relax a claim, never supply the permission that `ACCEPTED_DIVERGENCE` requires (except under A3a), and never decide a disposition by themselves. *Evidence* (what exists) may come from any record in the frozen tree, including AgentRuns and run records (parity reports, claim maps, review returns, validation outputs). It is cited in the evidence columns, and `SourceReliability` states whether a human disposition covers it. Evidence may change a disposition; context may only explain one. Before recording `NONE_FOUND` for a record-type claim (parity, claim map, review), the worker runs `git grep -l <DEL-ID>` over the frozen tree, including root `execution/_Coordination/AgentRuns/`. Rulings adopted by reference keep the `ADOPTED_BY_REFERENCE` flag." |
| A3a | Owner choice (decision 2) | "A verbatim, hash-bound owner decision in an owner-direction or adoption record that names the exact divergence may support `ACCEPTED_DIVERGENCE`. The row writes `OWNER_DIRECTION_RECORD:<path>` in Notes and is routed to confirmation, not decision, at R4." |
| A4 | Amend | Append: "A Remaining item whose text is accurate but that records no open action is `ALIGNED` and carries `NO_OPEN_ACTION` in Notes, for the R6 census. A governing declaration that delegates residuals to `## Remaining` is assessed as `DECLARED_STATE`; its delegation clause is recorded as not relied on and does not by itself make the row non-aligned. A `STATUS#remaining` key is never an ownership anchor in the reverse pass." |
| A5 | Keep | — |
| A6 | **Amend** | "Suite-level gate evidence is cited in `VerificationEvidence` as `GATE:<path relative to the run folder>`. The validator resolves it against the run folder. No per-test pass status and no validator or tool result is asserted unless a frozen-tree record or a GATE record shows it; otherwise the row cites the last record and says `not rerun`, or is `UNKNOWN`." |
| A7 | Keep | — |
| B1 | Amend | Replace "path hints only" with: "path hints (the deliverable's declared paths) plus the discovery step in A3. The worker never sees another deliverable's ledger or the reverse inventory before sealing." |
| B2 | Amend | "`Answer` is `CLAIMED_BY`, `PARTIAL`, `COVERS` (a coverage or record relation without ownership), `CONSTRAINS` (architecture-basis constraint without ownership), `UNKEYED` (this deliverable should own it but no issued key covers it; `ClaimKey` names the nearest key), or `NOT_MINE`. `ClaimKey` may hold a semicolon list. `STATUS#remaining/*` keys are not allowed." |
| B3 | Keep | — |
| C1 | **Amend** | Add these bullets. "Section wrappers (the four ontology, epistemology, praxeology and axiology headings) and heading-only blocks are `NON_NORMATIVE`. An identification table that carries status, phase or basis fields is `DECLARED_STATE`. A block with ITEM children and substance of its own is assessed directly, not as `CONTAINER`. `.sNN` sub-claims are allowed for any normative residue that no issued key covers, and are required when parts of a block would take different dispositions. A defect common to every ITEM of a surface is recorded once on the SURFACE row, and ITEM rows are judged on their own substance. The MEMORY history rule applies to every `HISTORY`-typed unit (`STATUS#history`, `CONTEXT#preparation-notes`): accurate history is `ALIGNED`, and only text that still reads as a current obligation is assessed. An undated current declaration inside a history surface is assessed as `DECLARED_STATE`. A blanket supersession declaration is its own `DECLARED_STATE` row and never changes sibling dispositions. A block preserved verbatim by the SOW migration is assessed as a current claim, with the migration record in `ContextRefs`. A duplicate of another unit in the same deliverable writes `DUPLICATE_OF <key>` in Notes and takes the same disposition." |
| C2 | Keep | — |
| C3 | Amend | Append: "Tier is the authority of the claim's substance as it stands. Pointer drift and setup residue that a deliverable catch-up would repair with no decision take `LOCAL_DESIGN`, even when the stale text names a ruling. Use `PROJECT_BASELINE` when the substance conflicts with, or restates, a ruling or accepted scope. Use `INVARIANT` when it restates a contract, boundary or validation invariant (including `VERIFIED_NOT_VALIDATED` rows); the `DivergenceLayers` field carries the subject." |
| C4 | Amend | Append: "`BaselineClass` names the baseline of the artifact that diverges, not the claim's deliverable. `NONE` is the default on non-aligned rows. An owner-ruled gate that is not a hold on this deliverable is `RULED_CRITERION`." |
| C5 | Amend | "`CLAIMS` means the claims-language boundary only (profile layer 1). Add `RECORD` for documentation or declared-state drift with no protected layer affected. `NONE` means no divergence layer (valid only on quiet rows)." |
| C6 | **Amend** | Add these rules. "(a) A claim satisfied only because the governed behaviour does not exist is not `ALIGNED`: use `PARTIALLY_IMPLEMENTED` or `DOCUMENTED_UNIMPLEMENTED` with the cause of the gap (for example `DEFERRED_BY_RULING`). (b) The disposition describes the claim's subject; whether a declaration is accurate goes in Notes. (c) `STALE_SETUP_SPECIFICATION` is for setup-era origin text; `STALE_REVIEW_OR_EVIDENCE` is for later declarations, review states, revision pins and metadata (dates) that were overtaken; `REMAINING_STATE_MISMATCH` is for `STATUS#remaining/*` only. (d) An ISSUED claim true at issuance but overtaken by a later ruling is `LIFECYCLE_REASSESSMENT_REQUIRED` and carries a `FindingGroup`; `IMPLEMENTED_DIFFERENTLY` is reserved for implementation departing from a claim, or a declared-open hold settled in code with no ruling (cause `AUTHORITY_UNCLEAR`). (e) A ruling that schedules a catch-up ('at next amendment') supports `ACCEPTED_DIVERGENCE` until its trigger occurs, with the trigger in `RemainingWork`. (f) `UNKNOWN` with `AUTHORITY_UNCLEAR` means the governing sources are silent." |
| C7 | Amend | See §9 below. |
| C8 | Amend | "Semicolon list allowed. Add `DOCUMENT_REVIEW` for human or agent review records of documents." |
| C9 | Keep | — |
| C10 | Amend | Append: "`SourceReliability` rates engineering and reliance-bearing sources (`ValidationEvidence`, cited human dispositions). Rows whose evidence is tests or run records are `NOT_APPLICABLE`." |
| Part D | Amend | Add `SUBCLAIM` to `UnitKind`. Add the columns `FindingGroup` (same id on rows that repeat one finding), `CanonicalSituation` (id from the Agent 0 table, or empty) and `AdoptedByReference` (`YES`/empty). Allow the `GATE:` token. List separator is `;` with no spaces. The validator checks every path token whatever its prefix (repo root, project root, `GATE:`), checks `ContextRefs` path tokens, rejects Remaining keys in reverse answers, and warns when rows with the same `TextSHA256` or `CanonicalSituation` differ in disposition, cause or tier without a Notes justification. |

**C7 and §9 — cause tags.** The sixteen tags covered most findings. The table
lists the changes.

| Change | Tag | Meaning |
|---|---|---|
| Add | `REPRESENTATION_MIGRATED` | The four-document kit was replaced by `ScopeOfWork.md` or `ArchitectureBasis.md`. |
| Add | `BASIS_POINTER_STALE` | Decomposition, DAG or revision pins, section references and relocated code anchors with unchanged behaviour. This absorbs the proposed `ANCHOR_RELOCATED`. |
| Add | `EVIDENCE_NOT_LOCATED` | For use with `UNKNOWN` only; replaces the `OTHER` uses. |
| Add | `VERIFICATION_REMOVED` | A cited test existed and was deleted (DEL-04-04 REQ-08). |
| Narrow | `EVIDENCE_OVERTAKEN` | Review or verification records that no longer bind the frozen bytes. |
| Narrow | `RECORD_DRIFT` | Status, Remaining or process declarations that disagree with other records. |
| Narrow | `DOC_BEHIND_CODE` | Code advanced; text describes the earlier behaviour. |

- `CONTRACT_VERSION_ADVANCED` stays schema and contract-version only.
- One cause per row: the cause of the remaining gap wins.
- `OTHER` stays, with its explanation requirement.

**Named repairs to calibration ledgers.** These are carried into the waves and
never patched in place.

- **DEL-07-02**
  - OUT-001: `STALE_REVIEW_OR_EVIDENCE`/`EVIDENCE_OVERTAKEN`, citing
    CHANGE-P2 parity (PASS; the production hash differs from the frozen SOW).
  - CLM-027/VER-001: add the CHANGE-P2 parity.
  - R03: tier and baseline class aligned with DEL-07-09 R03.
  - R11: harmonized with DEL-07-09 R09.
  - Re-answer the nine reverse answers anchored on Remaining keys (F7) with SOW
    keys or `UNKEYED`.
- **DEL-11-01**
  - OUT-001 and CLM-021/VER-001: cite CHANGE-P3 parity
    (`STALE_REVIEW_OR_EVIDENCE`, not `PARTIALLY_IMPLEMENTED` for parity;
    accessibility stays partial).
  - CLM-013.s03: `UNKNOWN` unless a review record is cited.
  - CLM-020.s03/.s04: cite the recorded validation and say "not rerun".
  - STATUS: move the R18 reasoning to Notes as context.
- **DEL-17-05**
  - OUT-001: cite CHANGE-P4 parity (`STALE_REVIEW_OR_EVIDENCE`).
  - REQ-001 and REQ-004: re-dispose under C6(a).
  - CLM-019: tier `INVARIANT`.
- **DEL-01-01**
  - OUT-001: consistent with AC-001 (parity does not bind the post-DEC-081
    bytes).
  - The eight DEC-027/057 rows: one `FindingGroup`, disposition per decision 4.
  - `BaselineClass`: `NONE` on the `_CONTEXT.md` and `_STATUS.md` rows; for
    MAINTAINERS.md, the baseline of that artifact.
- **DEL-12-03**
  - OUT-001: cite CHANGE-P3 parity (the production hash differs from the
    frozen SOW).
  - TEL-REQ-009: add `core/adapters/framework/adapter_framework.py` and
    `plugin_verification.py` evidence (self-reported).
  - CLM-010, CLM-018 and CLM-024: `AuthorityNeeded=OWNER` pending decision 3.
  - Change `UNVERIFIED` to `NOT_APPLICABLE` on test-evidence rows.
- **DEL-04-04**
  - OUT-001: cite CHANGE-P1-PKG04 parity and claim map (the hash matches the
    frozen SOW).
  - REQ-08: add the `product_physics` passthrough test; cause
    `VERIFICATION_REMOVED`.
  - CAP-SOLVER-020: `PARTIAL`.
- **DEL-07-09**
  - `verification-and-handoff`: `BaselineClass=NONE`.
  - CAP-WS-025: `COVERS`.
  - Collapse the mirrored CSV rows under the grain change.
  - VOCABULARY_COVERAGE/ROW-17: note the direct-apply section-edit route.
- **DEL-00-05**
  - Harmonize `AuthorityNeeded` on the `_CONTEXT.md` blocks.
  - The three open-hold rows go to the owner as sealed.
- **All eight:** re-map `DivergenceLayers`, cause tags and tiers to the
  amended vocabulary and the canonical situation table.

---

## 5. Grain recommendation

Keep one row per issued key, so coverage stays provable. Change what the
extractor issues (extractor v2, new `CLAIM_KEYS.csv` hash):

1. Parent CLM blocks to their section heading, and issue the four section
   wrappers (372 units corpus-wide) and heading-only CLM blocks as
   `NON_NORMATIVE` automatically.
2. Issue rows of tables that have no row IDs as deterministic `ITEM` keys
   (`.rNN`). This replaces the 99 worker-minted `.sNN` sub-claims, which were
   split at different thresholds and cannot be reproduced. Keep `.sNN` only for
   residual prose.
3. Parent unquoted `AC`/`VER`/`OUT` bullets to their `##` section.
4. Key the DEL-07-09 mirrored CSVs once per vocabulary row, citing both files.
5. Pre-disposition identical-text units once. There are 90 text hashes shared
   across deliverables, covering 605 units. The largest groups:

   | Unit title | Units with identical text |
   |---|---|
   | Decomposition Reference | 99 |
   | Package Reference | 99 |
   | PREPARATION Notes | 89 |
   | Architecture Basis Injection | 87 |
   | Objective Support | 75 |

   Agent 0 writes one canonical row per hash. Each deliverable's row inherits
   it through `CanonicalSituation` and deviates only with a justification.
   Verifiers sample the inherited rows.
6. Mark repeated declarations inside a deliverable `DUPLICATE_OF`.

Expected effect: about 20–25% fewer rows needing judgment, no worker-minted
keys, and structural rows typed mechanically. About 28% of calibration rows
carried no audit value (F9). The `_CONTEXT.md` surface could fall to one
surface row plus the canonical inheritances.

---

## 6. Reverse-pass recommendation for the R1 whole inventory

1. **Coverage.** Build the inventory by sweeping the whole product tree:
   `core/`, `apps/`, `schemas/`, `tools/`, `validation/`, `fixtures/`,
   `docs/`, `governance/`. Stratify it so that every deliverable's declared
   evidence paths contribute at least one capability. Documentation and
   governance surfaces are capabilities too. The pilot inventory missed the
   palette and catalog, the telemetry seam and panel, `core/handoff`, the user
   guide and the governance files.
2. **Answers.** Use the amended B2 vocabulary (`COVERS`, `CONSTRAINS`,
   `UNKEYED`, multi-key). Reject `STATUS#remaining` keys mechanically.
3. **Routing.** Each worker answers every capability in its package's areas,
   plus a random 10% cross-area sample. The sample detects anchoring and
   careless `NOT_MINE` answers without making 102 × N answers.
4. **Carry into R3:**
   - `CLAIMED_BY` from two or more deliverables: none in the pilot, but
     CAP-WS-022 has three `PARTIAL` claimants (07-02, 00-05, 07-09) and seven
     TREE capabilities have two;
   - zero-claimed capabilities: 79 of 96 in the pilot, plausibly owned by
     deliverables outside these eight (frame kernel, sparse solver, storage
     security, shell);
   - known gaps:
     - no key owns the `nonlinear_integration` crate (CAP-SOLVER-016 to 020,
       DEC-044);
     - tree and inspector creation, grid and deletion surfaces have no SOW key
       (DEL-07-02 CLM-034, versus DEL-07-03);
     - `core/gui/model_tree/engine.py` has no inventory row;
     - the diagnostics crate's standing "sparse default unresolved" diagnostic
       (CAP-SOLVER-027) conflicts with DEC-053.

---

## 7. Scale-out recommendation for R2

- **Order.** Adopt the amended conventions, the canonical situation table and
  extractor v2. Then run a first wave of 12 deliverables: two packages, one of
  them PKG-07 for its shared Remaining text. Verify that wave at double the
  rates below, and scale only if firm false alignment is 5% or less and no
  shared-situation conflicts remain.
- **Batch size.** One WORKING_ITEMS manager per package group. Each worker
  takes 3–4 deliverables from the same package, so shared text is judged by
  one mind. With 16 live agents: 3 managers, 9–10 workers and 2–3 verifiers
  per sub-batch.
- **Verifier sampling.**
  - 100% of rows that are `INVARIANT`, `ACCEPTED_DIVERGENCE`,
    `AUTHORITY_CONFLICT`, `UNKNOWN` or `LIFECYCLE_REASSESSMENT_REQUIRED`, and
    all rows on ISSUED deliverables.
  - 25% of other non-aligned rows.
  - 20% of `ALIGNED` normative rows (`REQUIREMENT`, `ACCEPTANCE`,
    `EXCLUSION`), weighted to `LOW` or `MEDIUM` confidence, `NONE_FOUND`
    verification and `NONE` verification class. Calibration showed 12% firm
    and 27% total questionable alignment in that class.
  - 10% of structural and inherited canonical rows.
  - 100% mechanical conformance check of `CanonicalSituation` and same-hash
    rows.
- **Manager briefs must add:**
  - the canonical situation table;
  - a pre-computed evidence map per deliverable. It lists SOW-STAGE2 parity
    and claim-map paths, their production hash, and whether that hash matches
    the frozen SOW. Agent 0 can compute it mechanically.
  - the `GATE:` token;
  - the definitions of layers, `AuthorityNeeded` and `SourceReliability`;
  - the rulings on decisions 2–4;
  - rule C6(a) on vacuous alignment;
  - a package-level cross-check of Remaining items that recur across
    deliverables;
  - the rule that reverse answers never cite Remaining keys;
  - a reminder that ADOPTED_BY_REFERENCE text in excluded `PROPOSED_*` files
    stays unread and flagged (DEL-12-03, DEC-074 O7).

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
