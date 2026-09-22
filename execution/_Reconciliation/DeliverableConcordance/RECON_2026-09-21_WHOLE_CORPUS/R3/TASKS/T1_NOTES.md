# R3 T1 — Unmapped capabilities, product and tooling areas

T1 classifies the 73 capabilities in `R3/CAPABILITY_COVERAGE.csv` that have status UNMAPPED (34) or UNMAPPED_RELATION_ONLY (39) and fall in areas CHECKS, COREB, COREC, DATA, DOCS, FEATB, FEATC, PHYS, SOLVER or VIEW. Each one gets exactly one row in `T1_UNMAPPED.csv`:

| Classification | Count |
|---|---|
| PRODUCT_UNOWNED | 31 |
| NON_DELIVERABLE | 27 |
| ROUTING_GAP | 14 |
| SHARED_OK | 1 |

Confidence is 32 HIGH, 37 MEDIUM and 4 LOW.

The biggest finding is the product-physics adapter. `core/product_physics/src/lib.rs` is the product's linear and nonlinear solve path. The desktop and the headless runner both reach it, yet no deliverable owns it. Every PKG-04 member, DEL-10-05 and the PKG-05 members call it "the caller of our surfaces". The same holds for the nonlinear integration slot that DEC-044 gave to an integration tranche which was never made a deliverable.

The second finding is a set of desktop panels with no GUI owner. The load-case manager is the sharpest case, because an owner ruling (DEC-094) and an accepted scope envelope (DEL-07-09) contradict each other.

Most NON_DELIVERABLE rows are governance, tooling and governing documents, or modules the owner already ruled "shared governed infrastructure" (DEC-074 O3).

This task classifies and proposes only. No deliverable, ledger row, code or DAG is changed.

Evidence paths are relative to `projects/chirality-piping/` at the freeze (`00115c71931bcae79909602d653740d3bb72dfa1`). A citation such as `DEL-07-09:CONTEXT#context-envelope (_CONTEXT.md:32-38)` gives a claim key and its source lines from `CLAIM_KEYS_V2.csv`. `SD` means `execution/_Decomposition/SOFTWARE_DECOMP.md` and `AX` means `execution/_ScopeChange/SCA-009_2026-08-20_0000/Vocabulary_Annex.md`. Each row's Notes field ends with a proposed route from the brief's route vocabulary.

## Findings by cluster

### F1. Product-physics adapter has no owner (12 PRODUCT_UNOWNED, 1 ROUTING_GAP)

**Capabilities.**
- Unowned (12): CAP-PHYS-001, 002, 003, 005, 006, 009, 010, 017, 018, 022 and 027, plus CAP-SOLVER-037.
- ROUTING_GAP (1): CAP-PHYS-021.

**Evidence.**
- Every routed mechanics deliverable answers NOT_MINE with the same reason: it "cites product_physics only as the product caller of" its own kernel. Examples are DEL-04-01 RC-04-0061 and DEL-10-05 RC-10-0256.
- The pointing goes in a circle on the mechanics result envelope (CAP-PHYS-003). PKG-05 members point to DEL-05-04. DEL-05-04 says it is "PKG-04 product-physics scope". DEL-04-06 disclaims it.
- For the nonlinear pieces (CAP-PHYS-009, CAP-PHYS-010, CAP-SOLVER-037), DEC-044 (`SD:635`) gives the assembled loop to "a new PKG-04 integration tranche". DEL-04-04:SOW#CLM-009 (ScopeOfWork.md:110-122) repeats this. No such deliverable exists. The tranche work was hosted inside DEL-04-01 (MEMORY.md:305 and :358-367). DEL-04-01 still answers NOT_MINE.
- Some sibling capabilities are partly owned: CAP-PHYS-004, 008, 011 to 016 and 023 to 026, and CAP-SOLVER-036 and 038 to 041. So ownership of the adapter is fragmentary, not absent.
- CAP-PHYS-021 (expansion-joint macro-element) is a ROUTING_GAP. DEL-03-06 MEMORY.md:120-158 shows that its tranches built this app path. PKG-05 points to DEL-03-06. The PHYS area was never routed to PKG-03.
- Area routing for PHYS omitted PKG-02 and PKG-03 (see O-4). So DEL-02-02 never saw unit normalization (CAP-PHYS-005). DEL-03-02 never saw shared-section resolution (CAP-PHYS-006). DEL-03-03 never saw bends (CAP-PHYS-022).

**Owner authority.** OWNER, then SCOPE_CHANGE.

**Proposed route.** OWNER_DECISION, then SCOPE_CHANGE_HANDOFF.

**Decision.** Who owns `core/product_physics` and `core/solver/nonlinear_integration`? The options below are as they stand in the evidence:
- (a) CREATE a product-physics integration deliverable in PKG-04. This would also instantiate the DEC-044 integration owner. DEC-044 already requires DEL-04-04's loop wording to be re-pointed when that owner opens.
- (b) Assign the adapter to an existing member. DEL-04-01 hosted the tranches. DEL-10-05 is the runner.
- (c) Rule it shared governed infrastructure, as DEC-074 O3 did for `core/product_preview`.

Option (c) would leave the product solve path without any deliverable-level verification owner.

**Risk if left unrepaired.** Changes to the product solve path fall outside every deliverable's verification and Remaining lists. Lifecycle closure of PKG-04 and PKG-05 would not cover the code the product actually runs.

### F2. Desktop panels without a GUI owner (15 PRODUCT_UNOWNED, 7 ROUTING_GAP, 1 SHARED_OK)

- **Load-case manager: CAP-FEATB-022, 023 and 025 (PRODUCT_UNOWNED).**
  - DEC-094 (`SD:685`) re-points the DEL-07-03-R-005/R-006 landing to DEL-07-09.
  - DEL-07-09:CONTEXT#context-envelope (_CONTEXT.md:32-38) says "editor implementation does not land here".
  - DEL-07-03 records R-005 as an ACCEPTED_DIVERGENCE (ScopeOfWork.md:131). Its CLM-041 (ScopeOfWork.md:552-560) forbids taking the editor without an accepted binding.
  - The self-weight panel (CAP-FEATC-031) has the same gap. Annex row 22 (`AX:126`) names no GUI landing.
  - This is an **owner-decision candidate**: which deliverable holds the load-case/support editor surface?
    - (a) Amend DEL-07-09's envelope.
    - (b) Rebind to DEL-07-03 through the change path.
    - (c) CREATE a load-case/support editor deliverable.
- **Rule-check run panel: CAP-FEATB-008 and 009 (PRODUCT_UNOWNED).**
  - This is the only user path for running a rule pack.
  - PKG-06 partly owns the runner (CAP-COREC-044).
  - DEL-07-03, 07-04, 07-05 and 07-09 and all of PKG-06 disclaim the panel.
  - This is an **owner-decision / scope-change candidate**.
- **Other panels with a best-supported candidate: PRODUCT_UNOWNED.**
  - Export safety review (CAP-FEATC-012) → DEL-12-02.
  - Offline proposal intake (CAP-FEATC-022) → DEL-16-04.
  - Run audit (CAP-FEATC-029) → DEL-14-02. Its relation key DEL-14-02:SOW#CLM-024 carries a **CONTESTED** resolution.
  - Batch review (CAP-VIEW-027) → DEL-16-02.
  - Form helpers (CAP-VIEW-033) → DEL-07-02.
  - Workflow helper (CAP-FEATC-023) follows its consumers.
  - ResultQuantity (CAP-VIEW-030) follows the diagnostics panel's partial owners, DEL-07-04 and DEL-07-07.
  - Route: SCOPE_CHANGE_HANDOFF.
- **Panels needing a D-42-style attribution ruling: PRODUCT_UNOWNED.**
  - Validation evidence review (CAP-FEATB-031). This matches the SURF-011 pattern that DEC-076 settled (`SD:667`), but no single identity is embedded. The packet refs include DEL-09-04, DEL-09-05 and DEL-10-04 (ValidationEvidencePanel.tsx:142).
  - Native package review (CAP-FEATC-021). DEL-17-03:SOW#CLM-003 (ScopeOfWork.md:38-46) excludes GUI. The panel builds its own record rather than calling the DEL-17-03 builder.
  - Route: OWNER_DECISION.
- **ROUTING_GAP to a named deliverable.**
  - CAP-FEATB-007 → DEL-07-04 (R-DEL-07-04-003, ScopeOfWork.md:129).
  - CAP-FEATB-010 → DEL-08-01. It owns the sibling RenderedReportPanel.
  - CAP-FEATB-029 → DEL-02-05 (CONTEXT#description; FEATB was never routed to PKG-02).
  - CAP-FEATB-030 → DEL-02-05 (LOW; DEL-12-01 is the alternative).
  - CAP-FEATC-016 → DEL-07-03 (annex row 23, `AX:127`).
  - CAP-VIEW-029 → DEL-07-03 (annex row 17, `AX:116`; DEL-07-02 agrees).
  - CAP-COREB-018 → DEL-02-02 (annex row 24, `AX:128`).
- **SHARED_OK: CAP-FEATC-013** (export unit disclosure). DEC-076 rules it shared desktop export infrastructure, and DEL-17-02 is contract owner only. Route: NO_ACTION.

### F3. Schemas and fixtures that landed outside their owner (5 ROUTING_GAP, 4 PRODUCT_UNOWNED)

- **Published under the DEL-10-03 lane.**
  - Operation outcome schema (CAP-DATA-010) → DEL-16-02. It owns the Rust `OperationOutcome` (CAP-COREB-003) and REQ-16-02-006.
  - Rule-check run result schema (CAP-DATA-018) → DEL-06-02 (REQ-06-02-009).
  - Both were published under the DEL-10-03 lane (its MEMORY), but DEL-10-03's scope of work does not claim them. Their DEL-00-06 REQ-06-03 relation carries a **FIRM** resolution.
  - Route: R5_RECORD_REPAIR.
- **Hanger schema and fixtures.**
  - Schema (CAP-DATA-006) → DEL-03-02. Fixtures (CAP-DATA-051) → DEL-03-07.
  - Annex row 23 names both landings. DEL-03-07 already partly owns hanger import validation (CAP-COREB-046).
  - This is an **owner-decision candidate**: confirm the landing. See O-1.
- **Plugin manifest verification: CAP-COREC-053 → DEL-02-04** (REQ-14, ScopeOfWork.md:183).
  - COREC was never routed to PKG-02. The test file carries the DEL-02-04 id.
  - Attribution is split: some answers name DEL-02-04 and some name DEL-10-02.
  - Its DEL-12-04 relation key carries a **FIELD** resolution.
- **Canonical-JSON crate remainder: CAP-COREB-040, CAP-COREB-041 and CAP-DATA-046 (PRODUCT_UNOWNED).**
  - DEL-00-04 REQ-04-03 (ArchitectureBasis.md:21) presumes "the serialization owner", but no deliverable is that owner.
  - The unchecked canonicalizer (CAP-COREB-039) is PARTIAL to DEL-02-05 and DEL-08-02.
  - Best-supported owner: DEL-08-02 (JCS hash basis, SOW#CLM-011.r02). The alternative is CREATE.
  - CAP-DATA-046's relation key DEL-02-05 REQ-02-05-005 carries a **CONTESTED** resolution.
  - Route: SCOPE_CHANGE_HANDOFF.
- **CAP-COREB-030** (shared GUI boundary helper `core/gui/pkg02_boundary.py`, LOW). Assign it, or rule it shared infrastructure. Its DEL-02-02:SOW#CLM-020 relation key carries a **FIRM** resolution.

### F4. Tooling, governance and governing documents (27 NON_DELIVERABLE, 1 ROUTING_GAP)

**Capabilities.**
- CHECKS: 013, 015, 016, 017, 018, 019, 020, 024, 025 and 042.
- COREB: 038.
- DATA: 058, 069 and 070.
- DOCS: 004, 010, 011, 015, 026, 050, 051 and 052.
- PHYS: 030, 031, 032, 033 and 034.

**Reasons.**
- DEC-074 O3 (`SD:665`) explicitly names these as shared governed infrastructure:
  - `tools/REGISTRY.md`;
  - `dependency_semantic_refresh_fanin.py`;
  - `dependency_type_rectification.py`;
  - `core/product_preview`.
- At the freeze, only `tests/product_preview/` imports `core.product_preview`.
- INTENT, PRD, PLAN, the registers and the scope-change records are governing or planning authority above deliverable scope.
- The harness config, lockfile snapshots, `.gitignore`, `.gitattributes` and the portability policy are repository hygiene or governance.

**Items needing care.**
- CAP-DATA-058 (preview fixtures) is shipped runtime input. It is classed by analogy to DEC-074 O3, which names the service but not the fixture folder. Route: OWNER_DECISION to confirm. The DEL-13-01 REQ-13-01-011 relation carries a **FIELD** resolution, and the same resolution applies to CAP-PHYS-034.
- CAP-CHECKS-015's DEL-10-04 relation carries an **OBSERVED** resolution.
- CAP-CHECKS-024 mixes secret exclusion with build hygiene. The owner may want the secret part held by DEL-12-04.

**Top-level README (1 ROUTING_GAP).** CAP-DOCS-001 is a ROUTING_GAP, not NON_DELIVERABLE. The top-level README is named as a DEL-01-01 repo-level target (SOW#CLM-019.r02, ScopeOfWork.md:266). It still carries the free and open-source posture conflict (CLM-009.s01).

## Owner-decision candidates

1. **Ownership of the product-physics adapter and the nonlinear integration crate** (F1; 13 capabilities). Options: CREATE, assign, or rule shared infrastructure.
2. **Landing of the load-case/support editor surface** (F2). This resolves the DEC-094 versus DEL-07-09 envelope conflict for CAP-FEATB-022, 023 and 025, and for CAP-FEATC-031.
3. **Owner for the rule-check run panel** (CAP-FEATB-008 and 009).
4. **D-42-style attribution** for CAP-FEATB-031 and CAP-FEATC-021.
5. **Confirmation of the hanger schema, fixtures and GUI landing** under annex row 23 and DEC-103 item 5 (CAP-DATA-006, CAP-DATA-051, CAP-FEATC-016).
6. **Status of the preview fixture family** (CAP-DATA-058) and of the shared GUI helper (CAP-COREB-030).
7. **Owner of the canonical-JSON crate** (CAP-COREB-040, CAP-COREB-041, CAP-DATA-046). This is a scope-change handoff and might not need an owner ruling.

## Coverage

**Population.** The launch filter over `R3/CAPABILITY_COVERAGE.csv` selects 73 capabilities: Status in {UNMAPPED, UNMAPPED_RELATION_ONLY} and Area in the ten T1 areas.

| Area | Rows |
|---|---|
| PHYS | 17 |
| CHECKS | 10 |
| FEATB | 10 |
| DOCS | 9 |
| DATA | 8 |
| FEATC | 8 |
| COREB | 5 |
| VIEW | 4 |
| COREC | 1 |
| SOLVER | 1 |

**Check.** A read-only Python script re-read `T1_UNMAPPED.csv` and confirmed:
- CRLF line endings throughout;
- an `#END` body count of 73;
- 73 body rows, none duplicated, none missing, none extra;
- Area and Status match the coverage file on every row;
- every Classification is in the brief's vocabulary;
- every Confidence is HIGH, MEDIUM or LOW.

**Cross-tab of Status by Classification.**

| Status | Classification | Rows |
|---|---|---|
| UNMAPPED | NON_DELIVERABLE | 17 |
| UNMAPPED | PRODUCT_UNOWNED | 13 |
| UNMAPPED | ROUTING_GAP | 4 |
| UNMAPPED_RELATION_ONLY | PRODUCT_UNOWNED | 18 |
| UNMAPPED_RELATION_ONLY | NON_DELIVERABLE | 10 |
| UNMAPPED_RELATION_ONLY | ROUTING_GAP | 10 |
| UNMAPPED_RELATION_ONLY | SHARED_OK | 1 |

**Resolutions.** Reverse answers were read from the sealed `DEL-*_reverse.csv` files through `ROUTING_SAMPLE/SAMPLE_MANIFEST.csv`. Every relation claim key was looked up in `CORPUS_CLAIMS.csv`, and all exist. The keys with resolution classes are named in the rows and above:
- FIELD ×3;
- CONTESTED ×2;
- OBSERVED ×1;
- FIRM ×3.

No adopted resolution references any of the 73 capabilities directly.

**Method limit.** Classification relies on reverse answers, scope text and freeze paths. No build or test was run.

## R3 observations

These are observations only. No effective value is corrected.

- **O-1 (hanger routing reading).** DEL-03-07's reverse reasons for CAP-DATA-006 and CAP-DATA-051 read DEC-103 item 5 as routing the hanger schema to DEL-07-09. The DEC-103 text (`SD:694`) says only that it is "recordable in the `DEL-07-09` coverage ledger". Meanwhile:
  - DEL-07-09 is coverage-only (_CONTEXT.md:32-38);
  - annex row 23 (`AX:127`) names DEL-03-02 and DEL-03-07 as landings.

  The reverse answer may rest on a misreading.
- **O-2 (ruling and envelope conflict).** DEC-094 re-points DEL-07-03-R-005/R-006 ownership to DEL-07-09. DEL-07-09's accepted envelope excludes editor implementation. As recorded, no deliverable can hold the load-case or support editor.
- **O-3 (tooling ownership pointer).** For CAP-CHECKS-020, DEL-00-08's CONSTRAINS reason says `software-workflow.json` is "owned by the loop and DEL-10-04 tooling". DEL-10-04 answers NOT_MINE (RC-10-0321).
- **O-4 (area-routing blind spots).** Area routing left out packages whose deliverables plausibly own a capability. All of the following turned into ROUTING_GAP or PRODUCT_UNOWNED rows that the plausible owner never answered:
  - COREC was not routed to PKG-02 (CAP-COREC-053 → DEL-02-04).
  - FEATB was not routed to PKG-02 (CAP-FEATB-029 and 030 → DEL-02-05).
  - PHYS was not routed to PKG-02 or PKG-03 (CAP-PHYS-005, 006, 021 and 022 → DEL-02-02, 03-02, 03-06 and 03-03).
- **O-5 (publication lane).** Two schemas (CAP-DATA-010 and CAP-DATA-018) were published under the DEL-10-03 lane per its MEMORY. DEL-10-03's scope of work claims neither. The owning deliverables (DEL-16-02 and DEL-06-02) answered COVERS.
- **O-6 (parallel builder).** Per DEL-17-03 RC-17-0086, the native package panel (CAP-FEATC-021) builds its own review record instead of calling `core/handoff/native_json`. The GUI package review may therefore diverge from the owned writer. This is a record of the reverse reason, not a finding about engineering adequacy.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). This file describes records and evidence only. It makes no certification, code-compliance, professional-approval or engineering-acceptance claim.
