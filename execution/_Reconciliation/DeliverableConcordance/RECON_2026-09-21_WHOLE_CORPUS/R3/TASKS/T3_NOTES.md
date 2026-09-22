# R3 T3: ownership of non-OWNED capabilities (OWNED_UNKEYED, DUPLICATE_OWNERSHIP, PARTIAL_ONLY, OWNED_SHARED)

T3 classified the 166 capabilities in `R3/CAPABILITY_COVERAGE.csv` whose
status is OWNED_UNKEYED (45), PARTIAL_ONLY (105), OWNED_SHARED (15) or
DUPLICATE_OWNERSHIP (1). Each got exactly one classification in
`T3_OWNERSHIP.csv`:

| Classification | Count |
|---|---|
| PARTIAL_UNOWNED_REMAINDER | 68 |
| UNKEYED_SCOPE_GAP | 57 |
| SHARED_OK | 33 |
| ROUTING_GAP | 6 |
| NON_DELIVERABLE | 1 |
| DUPLICATE | 1 |
| PRODUCT_UNOWNED | 0 |

**Split and keying findings.** All 15 OWNED_SHARED capabilities, and 18
PARTIAL_ONLY ones, are complementary splits. All 45 OWNED_UNKEYED
capabilities, and 12 PARTIAL_ONLY ones, are built work that no issued key
covers. These mostly need record repair.

**Structural gaps.** Most of the 68 unowned remainders are not scattered.
They fall into four places where no deliverable owns the work:
- the runtime application path of the model-operation engine (T3-G1);
- the product preview physics adapter `core/product_physics` (T3-G2);
- the DEC-044 nonlinear integration loop (T3-G3);
- the desktop shell and workspace infrastructure (T3-G4).

A fifth pattern: desktop panels that name their deliverable, where that
deliverable's SOW excludes GUI (T3-G5). These patterns need owner or
scope-change decisions, not record edits.

Proposed routing:

| Route | Count |
|---|---|
| R5_RECORD_REPAIR | 75 |
| SCOPE_CHANGE_HANDOFF | 38 |
| NO_ACTION | 37 |
| OWNER_DECISION | 15 |
| ENGINEERING_AUTHORITY | 1 |

Everything here is a proposal. No row value is changed.

## Method

- Population: filtered by the four statuses from `CAPABILITY_COVERAGE.csv`
  (hash in `SYNTHESIS_STATS.md`).
- Dossier: for each capability, a script listed every non-NOT_MINE reverse
  answer from the sealed reverse ledgers, mapped through
  `ROUTING_SAMPLE/SAMPLE_MANIFEST.csv`. The dossier included each answer's
  reason and the cited forward keys, with their summaries and effective
  Disposition/CauseTag from `CORPUS_CLAIMS.csv`.
- Remainder owners: for candidate owners, I read the NOT_MINE reasons and
  checked whether the candidate was routed at all.
- Verification reports: I searched every sealed report and notes file
  (excluding `superseded_*` and any `*DRAFT*` file) for the population's CAP
  and RC IDs. I used the reverse-pass findings (PKG-03, 04, 06, 08, 09, 10,
  12, 15, 16, 17).
- Resolutions: I checked the adopted resolution classes on every owner key.
  None corrects AuthorityNeeded in a way that changes the proposed owner. The
  CONTESTED, OBSERVED and FIELD rows are named in the CSV notes (rule 3).
- Freeze: file:line spot checks only. No builds or tests were run.

Classification choices where the vocabulary is tight:
- Where a remainder's plausible owner was **not routed** the row, I used
  ROUTING_GAP.
- Where the owning deliverable's own reason says the remainder is its own
  but **unkeyed**, I used UNKEYED_SCOPE_GAP, even if the status is
  PARTIAL_ONLY.
- Where a lone PARTIAL names no other owner, I classified by substance. See
  R3 observation O-4.

## Groups

Each CSV row carries `Group T3-Gnn` and `Route:` in its Notes.

### T3-G1: operation application path unowned (15)

Members: COREB-002, 004, 006, 007, 008, 009, 010, 011, 012, 013, 014, 015,
016, 017 and SHELL-020. All are PARTIAL_UNOWNED_REMAINDER.

**What the answers say.**
- DEL-16-02 owns validation and blocking before application. Its REQ-16-02-005
  places the "persistence/application API outside this slice".
- DEL-16-01 owns the operation-kind definitions.
- DEL-16-03 answers UNKEYED on the apply side (RC-16-0151, -0248, -0252), but
  its SOW also declares application outside its slice.
- The crate emits `deliverable_refs ["DEL-16-02","DEL-16-03"]` at
  `operation_applier/src/lib.rs:2003`. `apply_operation` is at `lib.rs:866`.
- The PKG-16 verifier found that "No issued key owns runtime application"
  (PKG-16_VERIFICATION.md:356).

**Unowned part.** Applying operations, and the per-kind resolvers for 11
editor operations. For COREB-012 it is the composite-support association
metadata contract (LOW).

**Owner decision.** Who owns runtime application of model operations. The
options in the evidence:
- (a) extend DEL-16-03, which gave the UNKEYED answers and holds the receipt
  and acceptance side;
- (b) extend DEL-16-02;
- (c) create a PKG-16 application deliverable.

**On ruling.** The scope-change workflow amends the chosen SOW and issues
keys. The R5 record repair then updates the reverse answers.

**Risk if left.** The only mutating path of every editor operation has no
accountable deliverable, no acceptance keys and no owner for its tests.

### T3-G2: product preview physics adapter unowned (9)

Members: PHYS-004, 008, 011, 013, 014, 015, 023, 024 and 026.

**What the answers say.** Each physics deliverable claims the physics
primitive it supplies: DEL-04-01, 04-03, 04-06, 05-01 and 05-02. Each says
the product adapter code in `core/product_physics` belongs elsewhere.
DEL-10-05 calls the service "without owning it" (RC-10-0109). T1 handles the
unmapped core of the same crate (PHYS-001, 002, 005 and others).

**Owner decision.** Options:
- create a product-preview integration deliverable;
- assign `core/product_physics` to DEL-10-05;
- keep the per-function split and key the adapter code into each physics
  deliverable.

**On ruling.** A scope-change handoff, joined with T1's PHYS findings.

**Risk if left.** The only solve path users reach has no owner of record.
Divergences from the reference crates are recorded but go to no one:
PHYS-008 departs from `apply_linear_supports`, PHYS-014 uses its own thrust
path, and PHYS-007 omits the corrosion allowance.

Related UNKEYED rows placed in T3-G6: PHYS-007, 028 and 029.

### T3-G3: DEC-044 nonlinear integration loop unowned (5)

Members: SOLVER-036, 038, 039, 040 and 041.

**What the evidence says.** DEC-044 puts the assembled loop in a PKG-04
integration tranche that no issued key owns. DEL-04-04 CLM-009 says the loop
is "owned by the integration tranche". DEL-04-01 calls itself the provider,
not the loop owner.

The PKG-04 verifier (PKG-04_VERIFICATION.md:356) reads the DEL-04-04 PARTIAL
answers as COVERS plus one unowned cluster. That cluster includes RC-04-0110,
which every deliverable answered NOT_MINE and T1 therefore holds.

**Owner decision.** Options:
- key the loop into DEL-04-04;
- create a PKG-04 integration deliverable;
- re-express the DEL-04-04 answers as COVERS and leave the loop explicitly
  unowned.

**Risk if left.** The nonlinear solve that product physics calls has no
owner for convergence policy, friction or the recovery rules.

### T3-G4: desktop shell and workspace infrastructure has no deliverable (12)

Members: COREB-035, SHELL-003, 006, 010, 052, 053, 058, 061, VIEW-034,
WSUI-008, 019 and 044.

**What the answers say.** Each partial owner claims its content and assigns
the rest to "the shell", "shell infrastructure", the "native host" or
"workspace orchestration". No deliverable holds that scope. The same gap
drives T2's UNMAPPED SHELL/WSUI set.

**Owner decision.** Options:
- create a PKG-07 desktop-shell deliverable;
- extend DEL-07-02 or DEL-07-08;
- treat host wiring as non-deliverable infrastructure.

**Risk if left.** Packaging identity, CSP (null at `tauri.conf.json:23`),
the job service, layout persistence and shared UI components have no
accountable owner.

### T3-G5: deliverable-identified panels outside their SOW's GUI scope (7)

Members: FEATC-002 (DEL-10-02), FEATC-003 (DEL-16-04), FEATC-007
(DEL-17-05), FEATC-008 (DEL-17-04), FEATC-014 (DEL-15-04), FEATC-015
(DEL-15-02/03) and FEATC-019 (DEL-10-03).

**What the evidence says.** Each panel serves one deliverable's packet, and
several embed `deliverable_id`. The owning SOW excludes GUI or does not key
the panel. PKG-17 answers these inconsistently (PKG-17_VERIFICATION.md:159):
DEL-17-07/08/09 answer CLAIMED_BY for their own panels, 17-04/05 answer
PARTIAL, and 17-03 answers NOT_MINE.

**Proposal.** Record repair that brings each panel into its deliverable's
scope, with one consistent convention. Where scope must grow, route it
through scope change.

**Risk if left.** Preview panels with synthesized or fixed packets stay
unowned (FEATC-007, FEATC-014, FEATC-015).

### T3-G6: keying gaps (57, UNKEYED_SCOPE_GAP)

**What the evidence says.** The deliverable built the work (run records,
`deliverable_id`, MEMORY, STATUS) but no issued key covers it. Main clusters:

| Deliverable | Keying gap | Capabilities |
|---|---|---|
| DEL-07-01 | Viewport tools, contract artifacts and UI-foundation performance work (D-68/D-70) | COREB-031, DATA-024, DATA-050, VIEW-008, 012, 031, 032, SHELL-063..067, WSUI-017 |
| DEL-07-02 | Annex rows 5, 18 and 24, and CLM-034 table and creation surfaces | COREB-026, FEATB-024, FEATC-034, 036, VIEW-028, 037..039, WSUI-021, 026 |
| DEL-07-03 | Rule-pack authoring (CLM-036.r05 IMPLEMENTED_UNDOCUMENTED/SCOPE_GREW_BY_DIRECTION) | FEATB-003..006, SHELL-039, 045 |
| DEL-08-01 | Report-package container, save and save controls | COREC-010, FEATB-014, SHELL-047 |
| DEL-04-01 | `curved_bend` crate (DEC-070) | SOLVER-024..027 |
| DEL-09-02 | Witness tooling and pilot | CHECKS-037, 038 |
| DEL-05-01 | Self-weight planning | PHYS-028, SOLVER-067 |
| DEL-05-03 | Multipliers; dormant annulus kernel | PHYS-020, 029 |
| DEL-16-02 | Wasm build; contract corpus | COREB-019, DATA-053 |
| DEL-10-04 | Lockfile; dev pins | CHECKS-022, 023 |

Singles: CHECKS-007 (DEL-09-05; ownership contested), CHECKS-026 (DEL-01-03),
COREB-049 (DEL-03-07), COREC-021 (DEL-14-02), DOCS-019 (DEL-16-04),
FEATB-028 (DEL-12-04), FEATC-017 (DEL-10-05) and WSUI-014 (DEL-07-01, with
DEL-16-03 unkeyed).

**Routing.** R5 record repair, with three exceptions:
- FEATC-004/005: OWNER_DECISION. Annex row 21 lands them jointly in
  DEL-07-01 and DEL-07-02, so a ruling must choose one key home.
- PHYS-007: OWNER_DECISION. DEL-03-08 answers UNKEYED, but its own rows
  CLM-003.r06 and RQ-007 assign the routine elsewhere.
- PHYS-029: ENGINEERING_AUTHORITY. Its nearest key waits on the owner-held
  pressure reference model.

**Risk if left.** Built behaviour stays outside acceptance and verification
keys, and later loops cannot tell it is in scope.

### T3-G7: governing documents (5)

Members: DOCS-009 (DIRECTIVE), 012 (SPEC), 013 (TYPES), 014 (CONTRACT) and
016 (AGENTIC_DEVELOPMENT_WORKFLOW).

**What the answers say.** Deliverables own sections: DEL-01-01 governance
rows, DEL-02-02 SPEC §4, DEL-02-03 SPEC §4.3, DEL-06-01 SPEC §7 and
DEL-02-01 TYPES §8. The remainder is project governance authority, not
deliverable output.

**Proposal.** NO_ACTION, except DOCS-016. DEL-11-05's anticipated-artifact
pointer is stale because the register now names `_COORDINATION.md`, so
DOCS-016 goes to R5. The owner may confirm that governing documents stay
outside deliverable ownership.

### T3-G8: routing gaps (6)

| Capability | Plausible owner (not routed) | Unowned part |
|---|---|---|
| SHELL-025 | DEL-08-02 | Model hashing (OUT-001 names "JSON model hashes") |
| SHELL-037 | DEL-06-04 | Checksum stamping (R-06-04-004) |
| SHELL-038 | DEL-06-04 | Local rule-pack store (CONTEXT storage markers, "rule pack registry") |
| SHELL-043 | DEL-03-07 | Import storability gate (consumes the acceptance disposition) |
| WSUI-037 | DEL-08-04 | Result semantic contract (LOW) |
| WSUI-041 | DEL-14-02 | Reopened-run historical rule |

For SHELL-038, the OBSERVED resolution on DEL-12-04:SOW#CLM-011.r01 stays
visible.

**Proposal.** A targeted reverse re-ask of the named deliverable, then record
repair.

### T3-G9: mutually deflected remainders in shared components (15)

Members: CHECKS-030, COREB-039, COREB-046, COREC-039, COREC-041, COREC-044,
DOCS-008, DOCS-021, DOCS-049, FEATC-027, SHELL-023, SHELL-044, SOLVER-070,
VIEW-025 and VIEW-026.

**What the answers say.** Two or more deliverables each point at the other
for the remainder:
- The PKG-06 codec, validator and runner (PKG-06_VERIFICATION.md:456-459).
- The shared RFC 8785 crate (COREB-039, OWNER_DECISION). Kept visible: the
  CONTESTED row on DEL-02-05 REQ-02-05-005, the DEL-08-02 R2 POSSIBLE_DEFECT,
  and the CONTESTED row on DEL-08-01 CLM-004.r05.
- The native result save (FEATC-027).
- The DEC-094 palette implementation home, where DEL-07-09's organization
  row is UNKNOWN/AUTHORITY_UNCLEAR (VIEW-025/026).

**Proposal.** Most need only a record repair that names the owner already
implied by the other side's reason. The following need an owner ruling:
COREB-039, COREC-044, DOCS-008 (adopt or retire the stale `MANIFEST.json`)
and VIEW-025/026.

### T3-G10: complementary shared ownership (33, SHARED_OK)

Members: all 15 OWNED_SHARED capabilities, plus 18 PARTIAL_ONLY ones whose
partial answers together cover the capability.

**Proposal.** NO_ACTION.

**Visible exceptions inside the group.**
- CONTESTED on DEL-08-01 R-08-01-009 (COREC-011, FEATB-012).
- CONTESTED on DEL-05-05 CLM-010.r07 (PHYS-012).
- Possible defects: DEL-08-03 CLM-027 (FEATB-012) and DEL-08-02 CLM-011.r02
  (FEATB-013).
- FIELD rows on DEL-10-05 keys (DOCS-044).
- VIEW-022 carries an R5 note: DEL-07-02's inspector creation is unkeyed.
- COREC-013 is SHARED_OK in substance only: DEL-08-04 is its sole owner (O-4).

### T3-G11: duplicate (1)

VIEW-017, the deformed-shape overlay. It is CLAIMED_BY DEL-07-01 (REQ-02)
and DEL-07-05 (CLM-004.r03, CLM-010).

**Better supported: DEL-07-05.** Its SOW names the translational overlay as
its landed slice. DEL-07-01 REQ-02 only lists "deformed shapes" among viewport
visuals. The code lives in DEL-07-01's `PipeViewport.tsx:4182`.

**Owner decision.** Options:
- DEL-07-05 owns and DEL-07-01 COVERS (proposed);
- DEL-07-01 owns;
- split rendering from result semantics.

### T3-G12: non-deliverable (1)

CHECKS-014, the D-43 architecture-basis validator. It is governance tooling.
DEL-00-08 keeps the gate definition, and seven AB members record coverage.
NO_ACTION.

## Owner-decision candidates (for R4)

1. **T3-G1.** Runtime application of model operations: DEL-16-03, DEL-16-02
   or a new PKG-16 deliverable (15 capabilities).
2. **T3-G2.** Ownership of `core/product_physics`: a new integration
   deliverable, DEL-10-05, or per-function keys. Join with T1's PHYS set.
3. **T3-G3.** The DEC-044 integration loop: DEL-04-04, a new PKG-04
   deliverable, or recorded as unowned.
4. **T3-G4.** Desktop shell and workspace ownership: a new PKG-07
   deliverable, DEL-07-02/07-08, or non-deliverable. Join with T2.
5. **T3-G5.** A convention for panels that name a deliverable whose SOW
   excludes GUI.
6. **VIEW-017.** The duplicate claim.
7. **FEATC-004/005.** One key home for annex row 21.
8. **PHYS-007.** DEL-03-08 (SOW-051) vs the product-physics owner.
9. **VIEW-025/026.** The palette implementation home under DEC-094.
10. **Smaller rulings:**
    - COREB-039: owner of the shared canonical-JSON crate;
    - COREC-044: owner of runner acceptability and orchestration (the PKG-06
      verifier suggests PKG-10);
    - DOCS-008: adopt or retire `MANIFEST.json`;
    - COREB-012: owner of the composite-support metadata contract.

## Coverage

Check: `_scratch_T3/assemble.py` (deleted after use) did the following:
- loaded `R3/CAPABILITY_COVERAGE.csv`;
- filtered Status in {OWNED_UNKEYED, DUPLICATE_OWNERSHIP, PARTIAL_ONLY,
  OWNED_SHARED};
- asserted that the set of CapabilityIDs in `T3_OWNERSHIP.csv` equals that
  population, with no duplicates;
- asserted that every Classification is in the rule-5 vocabulary, every
  Confidence is HIGH, MEDIUM or LOW, and every row carries a `Route:` from
  the route vocabulary.

Results:
- Population 166; rows written 166; missing 0; extra 0; duplicates 0.
- The `#END` row records 166.
- Status × Classification:

| Status | Classification | Count |
|---|---|---|
| OWNED_UNKEYED (45) | UNKEYED_SCOPE_GAP | 45 |
| OWNED_SHARED (15) | SHARED_OK | 15 |
| DUPLICATE_OWNERSHIP (1) | DUPLICATE | 1 |
| PARTIAL_ONLY (105) | PARTIAL_UNOWNED_REMAINDER | 68 |
| PARTIAL_ONLY | SHARED_OK | 18 |
| PARTIAL_ONLY | UNKEYED_SCOPE_GAP | 12 |
| PARTIAL_ONLY | ROUTING_GAP | 6 |
| PARTIAL_ONLY | NON_DELIVERABLE | 1 |

Routes: R5_RECORD_REPAIR 75, SCOPE_CHANGE_HANDOFF 38, NO_ACTION 37,
OWNER_DECISION 15 and ENGINEERING_AUTHORITY 1.

Limits:
- Confidence is LOW where no candidate owner was routed and no scope text
  names the work, or where a lone PARTIAL names no remainder. These 13 rows
  are COREB-012, COREC-044, DOCS-049, FEATC-014, FEATC-036, PHYS-007,
  SHELL-045, SHELL-058, SHELL-061, VIEW-034, WSUI-019, WSUI-037 and
  WSUI-044.
- NOT_MINE answers were read only for candidate remainder owners, not
  exhaustively.

## R3 observations

These are observations only, not corrections.

- **O-1.** DEL-00-05 and DEL-00-06 CONSTRAINS answers name "PKG-16/DEL-10-03"
  as the implementation owner of the operation engine and its wasm build
  (RC-00-0220, RC-00-0269). DEL-10-03 is the local FEA handoff contract, and
  no DEL-10-03 answer claims operation-engine work. The pointer appears
  wrong. It does not change ownership, because CONSTRAINS is a relation.
- **O-2.** The PKG-04 and PKG-08 verifiers read some PARTIAL answers as
  COVERS under a strict reading:
  - DEL-04-04 on the `nonlinear_integration` crate (RC-04-0149, 0157, 0063,
    0298);
  - DEL-08-02 on the RFC 8785 crate (RC-08-0037).

  Under that reading, SOLVER-036/039/040/041 would be relation-only and would
  join T1's unmapped set. COREB-039 would keep only DEL-02-05's partial.
  T3's classification is the same either way, because the remainder is
  unowned.
- **O-3.** For PHYS-007, DEL-03-08's UNKEYED reverse answer (RC-03-0091)
  conflicts with its own forward rows CLM-003.r06 and RQ-007, which assign
  the product routine to other deliverables (PKG-03_VERIFICATION.md:315).
- **O-4.** Four PARTIAL_ONLY capabilities have a single PARTIAL answer that
  names no other owner for any remainder: COREC-013 (DEL-08-04), FEATC-036
  (DEL-07-02), SHELL-045 (DEL-07-03) and VIEW-012 (DEL-07-01). Their
  PARTIAL_ONLY status reflects key coverage or neighbouring code, not a split
  of ownership. I classified them by substance: one SHARED_OK and three
  UNKEYED_SCOPE_GAP.
- **O-5.** The note for CAP-COREB-046 in `IMPLEMENTATION_SURFACES.csv` says
  the Rust port has no hanger path. The PKG-03 verifier found
  `validate_hanger_import` at `library_import_document/src/lib.rs` L797
  (PKG-03_VERIFICATION.md:317-321). The R1 inventory note is inaccurate.
- **O-6.** The reverse-pass protocol (CONVENTIONS B2) has no tie-break when
  two deliverables each assign a remainder to the other. T3-G9 lists the 15
  cases where this happened.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). This file
describes records and evidence. It makes no claim of certification, code
compliance, professional approval or engineering acceptance.
