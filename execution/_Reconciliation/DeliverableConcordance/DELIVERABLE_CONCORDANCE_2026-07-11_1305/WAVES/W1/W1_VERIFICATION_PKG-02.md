# W1 Fan-in Verification — PKG-02 (DEL-02-01 … DEL-02-05)

Run: DELIVERABLE_CONCORDANCE_2026-07-11_1305 · R2 wave W1 fan-in (owner-ruled:
fable at high effort). Frozen evidence tree
`.claude-worktrees/piping-frozen-551f84ef6` — HEAD verified
`551f84ef6be656f1603ce0acfa5e3935aa9683c7`; `git status --porcelain` empty
before and after every read and every re-execution in this pass. Method
authority: `R1_CONVENTIONS.md` (conventions 1–8, addenda 1–13, Part C
repairs); pinned plan §§6–7; `RUN_BASIS.md`. All findings below are
agent-authored and non-binding; nothing here is an owner or engineering
ruling. Verification never repaired a ledger — findings only.

**Scope.** Five ledgers (`CLAIM_CONCORDANCE_DEL-02-0{1..5}.csv` + notes):
every self-flagged row, every non-ALIGNED row, ≥2 ALIGNED rows fully
spot-checked per ledger, the mechanical convention sweeps, the fences, and
the package-specific dispatch items — (a) Part C convention-6 SECURITY
spot-check (DEL-02-04 REQ-013/REQ-017; DEL-02-05 REQ-013/REQ-025), (b)
adversarial test of DEL-02-04's uniform 27/27 ALIGNED against the
addendum-10/convention-8 overtaken-evidence bar and the DEC-010/DEC-012
permitted-deferral readings, (c) DEL-02-02 REQ-006 VERIFIED_NOT_VALIDATED and
the seven PARTIALLY_IMPLEMENTED core-vs-wiring splits, (d) DEL-02-05 REM rows
and the "0.1.0" version-literal chain re-verified against the frozen tree.

## Verdict summary

| Ledger | Verdict | Spot-checks (PASS/QUALIFIED/FAIL) |
|---|---|---|
| DEL-02-01 | **SOUND** | 13 / 2 / 0 |
| DEL-02-02 | **SOUND** (one named notes correction) | 12 / 3 / 1 |
| DEL-02-03 | **SOUND** | 11 / 1 / 0 |
| DEL-02-04 | **DEFECTIVE** (3 named defects) | 8 / 3 / 3 |
| DEL-02-05 | **SOUND** | 12 / 4 / 0 |

## Re-executions performed (side-effect-free, addendum 9)

All from the frozen working root with `PYTHONDONTWRITEBYTECODE=1`,
`pytest -p no:cacheprovider`, external `TMPDIR`, external `CARGO_TARGET_DIR`;
porcelain empty before and after each run.

| Suite | Result | Reproduces pilot claim |
|---|---|---|
| pytest batch: `test_plugin_manifest_schema.py` + `test_analysis_boundary_schema.py` + `test_model_schema.py` + `test_invented_example_models.py` + `test_units_schema.py` + `test_persistence_schema.py` + `test_project_persistence_service.py` + `tests/security/test_local_first_storage_policy.py` | **49 passed** | 2+2+4+7+3+2+15+14 — matches every pilot count (DEL-02-01 "11", DEL-02-02 PY-74 "3", DEL-02-03 PY-09 "2", DEL-02-04 PY-53 "2", DEL-02-05 "2+15" and "14") |
| `cargo test core/units --offline` (RUST-33) | **13 passed** | matches DEL-02-02 "13/13" |
| `cargo test core/serialization/canonical_json --offline` (RUST-23) | **8 passed** | matches DEL-02-05 "8 PASS" |

Mechanical sweep (scripted, all five CSVs): headers = the 20-column §6
contract exactly; every row 20 cells; all ClaimIDs match the addendum-12
fixed form `DEL-02-0X-<TYPE>-NNN` with no duplicates; all dispositions are §7
controlled values; `PackageID/DeliverableID` uniformly `PKG-02/DEL-02-0X`;
all DECLARED_STATE rows `SourceReliability=NOT_APPLICABLE` (addendum 6); no
REVIEWED/VETTED anywhere (all technical rows UNVERIFIED — ladder-conformant,
see per-ledger notes); the seeded `(gated: D-41)` bootstrap appears in
`RecordedRemaining` ONLY on each ledger's `_STATUS.md` surface row (addendum
2); no `SelectableUnderCurrentLoop=YES` row lacks a recorded item (addendum
12); **all five notes' Disposition and ClaimType histograms reproduce exactly
from the CSVs**. Requirement-row censuses match the frozen Specifications and
the R1 inventory ID-for-ID (12 / 16 / 12 / 18 / 26).

---

## DEL-02-01 — Canonical domain model schema — SOUND

| # | Row / item | Check | Result |
|---|---|---|---|
| 1 | REQ-001 (ALIGNED sample, full) | `schemas/model.schema.yaml` line 2 `$schema` = draft/2020-12; `$id` openpipestress.org; `docs/TYPES.md` §8 restatement; 11-test re-execution; sweep record `SWEEP_20260711T040758Z_e648462f1d05.json` exists | PASS |
| 2 | REQ-007 (ALIGNED sample, full) | model `AnalysisStatus` enum = the exact 7 declared values; `CODE_COMPLIANT` absent | PASS |
| 3 | REQ-002 (self-flagged, parallel surface) | All 13 required `$defs` present; `g_factors` (schema L590/596) and `exposed_element_refs` (L621/633) present; session shape `g_factor_x`/`exposed_pipe_refs` confirmed in `apps/desktop/src/features/model-tree/schemaSlotEmission.test.tsx` (L156/160/220). ALIGNED-with-note over IMPLEMENTED_DIFFERENTLY is correct at deliverable grain: DEL-02-01's own artifact matches its requirement wording; the divergent field shape lives in downstream consumers with tests pinning the correspondence. Encoding is aggregation-safe. | PASS |
| 4 | REQ-004 | `Quantity` requires value/unit/dimension/provenance; no `default` key anywhere in the schema; span-comparison limitation disclosed as note | PASS |
| 5 | REQ-005 | model `Provenance.required` = the exact 7 fields cited | PASS |
| 6 | REQ-009 / REQ-012 (self-flagged) | `Checksum` required set + canonicalization enum `JCS/NONE/TBD`; Specification `### Hash And Persistence Boundary` present (L84). Scope-split-not-ACCEPTED_DIVERGENCE reading is the addendum-11-correct one: the deferral to DEL-02-05 is written into the deliverable's own scope boundary, not a divergence needing a permitting ruling; ALIGNED at MEDIUM is right. | PASS |
| 7 | REQ-010 (self-flagged) | `additionalProperties:false` posture; translation-not-bypass reading consistent with #3 | PASS |
| 8 | DECL-001 (non-ALIGNED) | Specification L43 ("planning content for the future `schemas/model.schema.yaml` … not evidence that the schema file has already been implemented") and L120–121 ("future primary schema artifact … outside the current write scope") verified verbatim; schema implemented at frozen SHA (46,031 bytes). STALE_SETUP_SPECIFICATION correct (addendum 4). | PASS |
| 9 | DECL-002 (non-ALIGNED) | Datasheet L50 "initial schema plan, not an implemented schema file" verbatim | PASS |
| 10 | DECL-003 (non-ALIGNED, self-flagged MEDIUM) | Guidance L23 "anticipated `docs/TYPES.md` update is outside this run's write scope"; `docs/TYPES.md` §8 carries the vocabulary at frozen SHA (L162 `y_reference`, L204 `ProjectPersistenceEnvelope`). STALE at MEDIUM is the right call; partial-staleness disclosed. | PASS |
| 11 | DECL-004 (non-ALIGNED) | Procedure L5 "future `schemas/model.schema.yaml` artifact", L121 "Records expected from future implementation/review work" verbatim; STALE correct | PASS |
| 12 | DECL-005 / bootstrap | `_STATUS.md ## Remaining` = bootstrap only; recorded verbatim on this row only; excluded from analysis | PASS |
| 13 | Census + candidate residuals (self-flagged) | 12 REQ = spec IDs; no in-tree README (folder listed); C-02-01-001/002 homed in the Guidance conflict table (convention 3 → no REMAINING_WORK row) and the span-comparison limitation is a disclosed bounded limitation noted on REQ-004 — neither warrants a row. | PASS |
| 14 | ACCEPTANCE = 0 (self-flagged) | Verification column restates requirements; consistent with all four sibling ledgers (package-uniform grain). Judgment call, disclosed, aggregation-safe. | QUALIFIED |
| 15 | Bootstrap gate-cell annotation | DECL-005 lists `(gated: D-41)` (annotated as excluded) in `GateOrStageConstraint`, where DEL-02-02/03/04 use `NONE_RECORDED`. Self-identifying and excluded either way; harmonization note only. | QUALIFIED |

Fences: held (notes boundary statement verified against outputs; no
lifecycle/DAG/scope mutation, no F-PIP claim language outside attributed
quotes, dispositions routed via `AuthorityNeeded`, OWNER on the four R5
repair candidates). **Verdict: SOUND.**

---

## DEL-02-02 — Unit system and dimensional-analysis core contract — SOUND (with one named correction)

| # | Row / item | Check | Result |
|---|---|---|---|
| 1 | REQ-006 / U-006 VERIFIED_NOT_VALIDATED (dispatch item c, self-flagged) | Re-executed cargo 13/13; `core/units/README.md` confirms "B3 still owns the broader mixed-unit round-trip, conversion-witness, rejection, and D-04 tolerance corpus" and "B3 conversion-witness and tolerance corpus coverage under DEC-026"; DEC-026 resolves in `SOFTWARE_DECOMP.md` §12 (T-C tolerance classes). MECHANICS row, numeric-correctness claim, implemented + unit-tested, class-required validation explicitly open → VERIFIED_NOT_VALIDATED is the §7-exact disposition; the alternative ALIGNED-with-open-validation reading would breach the plan's "a unit test alone must not be promoted to engineering validation". CONFIRMED correct. | PASS |
| 2 | U-001 (PI, core-vs-wiring) | README: "B1 does not wire units into schemas, application fields, solver boundaries, reports, imports, exports, or rule-pack evaluation. Those remain Phase B2/B3 handoffs." — direct frozen-tree evidence for the split | PASS |
| 3 | U-002 (PI, self-flagged) | Disposition survives on the B2-wiring ground (see #2) and the mirror structure is real (`engine.py` defines its own literal dimension list; it cannot and does not call the Rust crate). **But the transcribed packet detail is overtaken — see the named correction below.** | QUALIFIED |
| 4 | U-008 (PI) | Persistence binding is DEL-02-05 surface (consistent with the DEL-02-05 ledger's evidence); DEC-010/DEC-019 resolve in §12 | PASS |
| 5 | U-010 / U-015 / U-016 (PI, self-flagged) | D-01 packet §1 "Not ruled here" verified verbatim: "unit identifier namespace/alias policy; dimensionless/ratio/percentage/coefficient classification; … schema file layout/tooling; unit diagnostic code namespace" — the open-decision grounds for the three PARTIALLY_IMPLEMENTED + OWNER encodings are real and current. Structural presence (QuantityKind enum, retired-alias rejection test) vs open human decision is a disclosed judgment; PI is the conservative, defensible side. | PASS |
| 6 | U-012 (PI) | Test-matrix strands (round-trip, import/export, rule-pack, hash) are B2/B3/other-deliverable suites; consistent with the frozen tree | PASS |
| 7 | U-003 (ALIGNED sample, full) | `Quantity` required set; `units.schema.yaml` `QuantityKind` incl. dimensionless; PY-74 3/3 re-executed | PASS |
| 8 | U-007 (ALIGNED sample, full) | `ConversionDeclaration.required` includes `provenance` + `review_status`; crate `ConversionProvenance`/`ReviewStatus` present | PASS |
| 9 | EXC-003 (self-flagged) | Fired-contingency exclusion: DEC-018 accepted catalog/constants/representation/tolerance. ALIGNED (exclusion sentence is conditional and not false) + `AuthorityNeeded=REVIEW` + cross-reference to DECL-001 is a disclosed, aggregation-safe encoding. | QUALIFIED |
| 10 | DECL-001..004 (non-ALIGNED) | Datasheet "Conversion factors: TBD" / "Dimensional basis: ASSUMPTION" rows, Procedure L12 "intended for later implementation work", Guidance C-DEL-02-02-001 (Human ruling TBD) all verified verbatim; DEC-018 (2026-06-10) + implemented B1 crate overtake them → 4× STALE correct | PASS |
| 11 | DECL-006 / DECL-007 (self-flagged census) | MEMORY row per addendum 1; `core/units/README.md` as the deliverable-owned in-tree README — the README self-identifies as the DEL-02-02 module contract; addendum-1-plausible, disclosed, and the only PKG-02 ledger where a README exists (census correct, not drift) | QUALIFIED |
| 12 | SourceReliability friction note | The 2026-06-03 bounded CHECKING acceptance + later IN_PROGRESS reset + post-disposition DEC-068-era changes → UNVERIFIED (not REVIEWED) is the addendum-6-correct rung; nuance disclosed | PASS |
| 13 | Histograms + census | Reproduce exactly; 16 REQ = U-001..016 | PASS |
| 14 | **Notes §7 force_per_length claim** | **FAIL — named correction:** notes §7 (and the U-002 `RemainingWork` parenthetical) state engine.py's `CANONICAL_DIMENSIONS` is "29 ids, omitting `force_per_length`" vs a crate vocabulary of "30 ids incl. `force_per_length`". At the frozen SHA `core/model_operations/validation_preview/engine.py` L41–70 contains 29 ids **including `force_per_length`**; the crate enum and `units.schema.yaml` `DimensionId` each carry 30 (the extra id is the `TBD` token). The D-01 packet E4 conflict (dated 2026-06-10) is real as cited but has since been fixed in code; the pilot transcribed it as a live frozen-tree fact without re-verification. The **mirror-not-call duplicate-authority risk still stands** (the list is an independent literal), so U-002's disposition and the DEL-16-02 routing survive; only the "omitted force_per_length" detail must not propagate to the DEL-16-02 wave pilot or the package summary. | FAIL (detail only) |

Fences: held. **Verdict: SOUND** — the one FAIL is a notes-level overtaken
detail with correct packet attribution in the CSV cell; no disposition,
citation-resolution, histogram, or aggregation surface is corrupted.
Correction recorded above for downstream use.

---

## DEL-02-03 — Code-neutral analysis boundary model — SOUND

The full `analysis_boundary.schema.yaml` was parsed and every schema-level
assertion in the ledger checked mechanically:

| # | Row / item | Check | Result |
|---|---|---|---|
| 1 | REQ-001 | `AnalysisStatus` enum = exactly the 7 `docs/TYPES.md` §4 statuses | PASS |
| 2 | REQ-002 (ALIGNED sample, full) | `AutomaticAnalysisStatus` excludes `CODE_COMPLIANT` and `HUMAN_APPROVED_FOR_PROJECT`; `forbidden_software_claims` = [code_compliance, certification, sealing, professional_acceptance, authentication, TBD]; `ProfessionalBoundary` claim fields const false; PY-09 re-executed 2/2 | PASS |
| 3 | REQ-003/004/006 | `MechanicsSolveBoundary.authority` const `solver_result_only` + status {MODEL_INCOMPLETE, MECHANICS_SOLVED}; `UserRuleCheckBoundary` requires `rule_pack_ref`; `RulePackRef.supplied_by_user` const true; rule statuses disjoint from human/forbidden set | PASS |
| 4 | REQ-005 | `MissingInputFinding.required` ⊇ the cited fields (actual set additionally requires `input_name`) | PASS |
| 5 | REQ-007 (ALIGNED sample, full) | `HumanAcceptanceRef` requires record_ref/human_status/bound_evidence_hashes(minItems 1)/scope_notice; `human_acceptance_authority` const `external_hash_bound_human_record_only` | PASS |
| 6 | REQ-008 / REQ-009 / REQ-011 | `Provenance.required` = 6 fields; `$schema` = draft/2020-12; 12 `TBD` tokens present | PASS |
| 7 | REQ-010 (self-flagged, ALIGNED MEDIUM) | Every object `$def` carries `additionalProperties:false` (mechanically confirmed — zero exceptions); no adapter/API surface exists in the deliverable's slice to exercise behavioral no-bypass. ALIGNED at model grain with MEDIUM confidence is right; the alternative would fault a data-model draft for behavior it cannot own. | PASS |
| 8 | DECL-001 / DECL-002 (non-ALIGNED, self-flagged MEDIUM) | Specification L131 "Future schema target: `analysis_status` enum/schema location TBD" and Datasheet "exact schema TBD"/ASSUMPTION construction rows verified; concrete schema + tests + fixtures exist at frozen SHA → widened-STALE correct; the pilot's reader-awareness boundary test (Spec/Datasheet stale; Guidance/Procedure timeless → ALIGNED) is applied consistently and is the most convention-faithful articulation in the package | PASS |
| 9 | Reviewer-material cross-checks | Guidance L74 "No unresolved source conflicts remain" verbatim; RF-001 overtaken as claimed (`Review_Findings.csv` says `_REFERENCES.md` = "accepted v0.2"; frozen `_REFERENCES.md` L15 reads "Accepted revision 0.7"); MEMORY L75–77 scopes `analysis_status.schema.yaml` to DEL-05-04 (surface correctly kept out of orbit) | PASS |
| 10 | Fence-adjacency handling | F-PIP-2-adjacent cells quote schema field names and requirement text, never assert compliance status; DEL-04-01-pilot cross-reference correctly demoted `_REVIEW.md` to an agent check | PASS |
| 11 | Histograms + census | Reproduce exactly; 12 REQ = R01–R12; 6 per-bullet EXC rows; ACCEPTANCE=0 (restating V01–V11 table) | PASS (ACCEPTANCE grain QUALIFIED, as package-uniform) |

Fences: held. **Verdict: SOUND.**

---

## DEL-02-04 — Plugin and extension domain contracts — DEFECTIVE

### Part C SECURITY spot-check (dispatch item a) — the encoding itself passes

| Row | Check | Result |
|---|---|---|
| REQ-013 (SECURITY) | `ValidationEvidence` opens with the exact convention-6 string `NONE_FOUND — sufficiency review deferred, owner-gated` (em-dash, marker-exact) + an explanatory parenthetical; Disposition stays ALIGNED (no VERIFIED_NOT_VALIDATED downgrade on the deferred-sufficiency ground); `AuthorityNeeded=OWNER` routes the owner-gated review. Schema consts verified in the frozen tree: `SandboxDeclaration` requires `sandbox_required`, `arbitrary_code_execution_allowed` (const false), `filesystem_access_default`/`network_access_default`/`process_spawn_default` (denied enums), `capability_declaration_required`; PY-53 re-executed 2/2. Rule grammar/library TBD is genuinely within DEC-012's enumerated retained-TBD list. | PASS |
| REQ-017 | Classed GOVERNANCE (not SECURITY), so the convention-6 string correctly does not appear; `ValidationEvidence=NOT_APPLICABLE` with in-cell reason (convention 5), `AuthorityNeeded=OWNER`. `PermissionModel.denied_by_default` const true and `grant_state[0]='not_granted'` verified; ScopeLedger SOW-038 "detailed permissions TBD" verified verbatim. Class assignment is defensible (a governance-of-open-decisions claim); a SECURITY classing would also have been supportable — noted for cross-wave watchers, not a defect. | PASS (class choice QUALIFIED) |

### Adversarial test of the uniform 27/27 ALIGNED (dispatch item b) — does NOT survive

The requirement rows largely do survive: the contract artifacts exist and the
load-bearing constants are test-asserted (verified above and via
`PersistenceAccess` consts for REQ-009 and `ApiBoundaryCompatibility`
consts for REQ-011); DEC-010 and DEC-012 resolve in `SOFTWARE_DECOMP.md` §12
(rows 588/590) and DEC-012 does record an accepted retained-TBD boundary. But
three encodings fail:

**Defect 1 — DECL-002 (Datasheet) wrongly ALIGNED; addendum-4 STALE_SETUP_SPECIFICATION applies.**
The frozen Datasheet's Construction table declares "Plugin interface spec |
**Draft target** | … Exact schema file layout is TBD", "Sandbox/permission
model notes | **Draft target**", and "Verification hooks | Required concept |
**Future implementation should support** schema validation, unit checks,
protected-content/provenance gates, and adapter/plugin tests" — while at the
frozen SHA `schemas/plugin_manifest.schema.yaml`,
`fixtures/plugin_manifest/invented_manifest_no_bypass.json`,
`tests/test_plugin_manifest_schema.py` (passing), and
`docs/architecture/extension_domain_contracts.md` all exist. The four kit
documents contain **zero references** to any of those artifacts (grep across
the kit: no `plugin_manifest`, no `extension_domain_contracts`, no
`test_plugin`), so the surface's declaration no longer describes the frozen
implemented slice — the exact addendum-4 widened-STALE trigger, and the same
posture the sibling pilots dispositioned STALE on their Datasheets
(DEL-02-01 DECL-002 HIGH "initial schema plan, not an implemented schema
file"; DEL-02-02 DECL-002 HIGH; DEL-02-05 DECL-002 HIGH "anticipated
artifacts / construction target"). The row's rationale discusses only the
rev-0.7 citation lag and never engages the Draft-target/future-implementation
framing. Aggregation impact: DEL-02-04's kit silently drops out of the
corpus-wide DECLARED_STATE staleness / R5-repair census that all four sibling
ledgers feed. **FAIL — wrong disposition.**

**Defect 2 — DECL-004 (Procedure) wrongly ALIGNED; same ground (self-flag confirmed and elevated).**
Frozen Procedure L58 "define a **future** verification path", L88 "**Future**
manifest/schema review record should identify…", L89 "**Future**
canonicalization/hash verification record…", L90 "**Future** sandbox/privacy
verification record…" — the manifest schema, fixture, and test those records
anticipate now exist; and L19 declares "For this Pass 3 run, state is
`SEMANTIC_READY`" against a current `_STATUS.md` of IN_PROGRESS. Identical
posture took STALE (HIGH) on DEL-02-01's and DEL-02-02's Procedures. The
pilot flagged this row as "the single most debatable disposition" and invited
elevation; elevation is confirmed. **FAIL — wrong disposition.**

**Defect 3 — REQ-014 (VALIDATION class) ALIGNED overstated; package-consistent encoding is PARTIALLY_IMPLEMENTED (self-flag ruled).**
Requirement text (Specification L35): "Plugin/adapter contract verification
**shall include** layered checks for schema conformance, unit safety,
provenance, diagnostics, protected-content gates, and relevant adapter/plugin
regression behavior." At the frozen SHA only the schema/fixture layer is
executable (PY-53); the unit-safety/provenance/diagnostics/protected-content/
regression layers are documented plans for PKG-10/PKG-12. The same claim
shape in the same package — DEL-02-02 U-012, "the unit test set shall cover
…" with strands missing — took PARTIALLY_IMPLEMENTED (MEDIUM). Unlike
DEL-02-01 REQ-012, whose requirement text itself builds in "recorded as
documented deferrals", REQ-14 carries no deferral clause. Encoding the two
differently corrupts the cross-package REQUIREMENT-disposition aggregate.
**FAIL — overstated disposition** (disclosed by the pilot, but the
convention-consistent answer is determinate, so it is graded a defect rather
than a note).

### Other checks

| # | Item | Check | Result |
|---|---|---|---|
| 1 | REQ-009 (ALIGNED sample, full) | `PersistenceAccess` consts exactly as cited (`application_service_command_query_job_only`; direct_sql/raw_sqlite/table-coupling/direct-mutation all const false); SCA-003 basis | PASS |
| 2 | REQ-011 (ALIGNED sample, full) | `ApiBoundaryCompatibility.schema_version_contract` enum carries 'JSON Schema 2020-12'; `result_envelope_required` const true; PY-53 re-executed. **DecisionBasis wording** "layout deferral permitted by DEC-012" overstates the record: DEC-012's enumerated list (dependency versions, solver library, rule grammar/library, transport, formats, CI provider, coverage thresholds, container) does not name schema file layout or codegen tooling — the notes' blanket "every embedded TBD is a named-decision-permitted deferral under DEC-010/DEC-012" likewise over-reaches for sandbox mechanism / permission taxonomy / registry / codegen (SOW-038 covers permissions). Dispositions are unaffected (the requirements themselves declare the TBDs, and per addendum 11 ALIGNED is what a residual/TBD without a permitting record yields anyway), so this is a basis-note over-attribution, not a failed citation. | QUALIFIED |
| 3 | DECL-001 / DECL-003 (self-flagged rev-lag) | Kit cites decomposition revision 0.7; frozen `SOFTWARE_DECOMP.md` is revision 0.8, whose delta (SCA-005 propagating D-21/DEC-056 — the PRD v0.2 milestone set and FR renumber) does not re-describe the plugin-contract basis (SCA-001/DEC-010/DEC-012/AB-00-07 unchanged). ALIGNED-with-metadata-note is defensible for these two surfaces on this ground alone; Guidance's TBDs (permission model, registry) are genuinely still open. CONF-02-04-001 overtaken-conflict claims verified (frozen `_REFERENCES.md` L15 reads "Accepted revision 0.7"; the kit/Review still say it reads v0.2). | QUALIFIED |
| 4 | DECL-005 / DECL-006 | `_STATUS.md` bootstrap-only Remaining; lifecycle-correction Decision_Log records present; MEMORY dated-entry handling per addendum 1 | PASS |
| 5 | Overtaken `_REVIEW.md` as sole VerificationEvidence (REQ-005/006/012/015/016) | `KIT/_REVIEW.md` is a 2026-04-30 AGENT_CHECK predating the rev-0.7 refresh (its XD-004 references rev 0.4). These are prose/governance claims with no class-required suite, every citation carries the `not re-executed at frozen SHA 551f84ef6` marker, and the pilot independently inspected the implementation surfaces — so convention-8's overtaken bar (class-required evidence) is not breached; noted as a reliability caveat. | QUALIFIED (folded into #3 count) |
| 6 | Histograms, census, ClaimID re-pad | Reproduce exactly; 18 REQ = spec IDs; two-digit→three-digit re-pad disclosed with traceability | PASS |
| 7 | Fences | Notes and CSV clean of F-PIP claim language (REQ-08 records a prohibition, attributed); read-only; routing via AuthorityNeeded | PASS |

**Verdict: DEFECTIVE.** Named defects: (1) DECL-002 disposition (ALIGNED →
addendum-4 STALE_SETUP_SPECIFICATION indicated); (2) DECL-004 disposition
(same; pilot's own elevation flag confirmed); (3) REQ-014 disposition
(ALIGNED → PARTIALLY_IMPLEMENTED for package-consistent encoding).
Recommended re-run scope: the pilot re-run should also tighten the
REQ-011/REQ-016 DecisionBasis wording and the notes' blanket
DEC-010/DEC-012 permitted-deferral claim (QUALIFIED items above). The Part C
convention-6 SECURITY encoding on REQ-013 is **correct as written** and can
be carried into the re-run unchanged.

---

## DEL-02-05 — Project persistence and round-trip serialization — SOUND

| # | Row / item | Check | Result |
|---|---|---|---|
| 1 | REQ-013 / REQ-025 SECURITY (dispatch item a) | Convention-6 encoding present: `NONE_FOUND - sufficiency review deferred, owner-gated (…)`; ALIGNED retained (no VERIFIED_NOT_VALIDATED downgrade); deferral correctly NOT promoted to ACCEPTED_DIVERGENCE (no named permitting ruling — addendum 11); `tests/security/test_local_first_storage_policy.py` re-executed 14/14. Two deviations from the DEL-02-04 twin rows: (i) ASCII hyphen instead of the convention's em-dash in the marker string; (ii) `AuthorityNeeded=NO` where DEL-02-04 REQ-013 routes OWNER for the same owner-gated deferred review. Both are aggregation-recoverable (the marker substring "sufficiency review deferred, owner-gated" matches across ledgers) but should be harmonized — see package risks. | QUALIFIED ×2 |
| 2 | REM-002 + REQ-006 "0.1.0" chain (dispatch item d) | **Re-verified in the frozen tree, line-exact:** `ProjectValidationPanel.tsx` L255/L580/L582, `ExportReviewPanel.tsx` L269 (also L1135), `ReportPanel.tsx` L798 (also L657/L833) all compare/stamp `"0.1.0"`; `projectService.ts` L36 `SUPPORTED_MODEL_SCHEMA_VERSION = "0.2.0"` and `model_document_migration.rs` L21 the same — so a current 0.2.0 document trips "unsupported_schema_review_required" exactly as the ledger's DEFECT note states; `evaluateModelDocumentLocal` at `projectService.ts` L68 (browser mirror of the Rust authority); DEC-033 §12 codification explicitly leaves "the H2 hardening-lane item (DEC-019 evaluation unification) … unchanged". PARTIALLY_IMPLEMENTED (REQ-006, HIGH) + REM-002 ALIGNED (residual valid/owned/bounded) are correct. | PASS |
| 3 | REM-001 + FR-001 chain | `migrate_project_store` at `service.py` L360 (store-schema level only); `service_operations` enum carries `migrate_project`; no explicit project-document migrate op; DEC-028 (D-09) and DEC-057 (D-06) ruling records present; `.opsproj` container owned outside DEL-02-05. Residual current and evidence-backed. | PASS |
| 4 | `_STATUS.md` Remaining ↔ ledger | Three items verified in the frozen `_STATUS.md`; FR-001/H2 copied onto exactly the touched rows + REM rows + DECL-005, `NONE_RECORDED` elsewhere (convention 8); gates `UNGATED; UNGATED; (gated: D-41)` in Remaining order (addendum 3); Selectable YES on the 9 residual-bearing rows only — matches the R1 inventory row (3 remaining, 2 non-bootstrap, UNGATED;UNGATED, Selectable YES). One nit: the CSV transliterates the `_STATUS.md` "§"/"§§" to "section"/"sections" inside otherwise-verbatim `RecordedRemaining` text (ASCII-only CSV choice, undisclosed). | PASS (transliteration QUALIFIED) |
| 5 | REQ-001/020/021/022 (PI splits) | Each PI ground re-verified (migrate-op gap #3; binary-manifest partition tied to the out-of-ownership container; DEC-019 compatibility-window "TBD — human ruling"); `MigrationStatus.status` enum contains unsupported_schema/stale/failed/migrated (+current/migration_needed/newer_than_supported/TBD) — the ledger's four-token summary is subset-accurate | PASS |
| 6 | REQ-010 (PI) | Dual-engine duplication (browser mirror vs Rust authority) verified live; H2 recorded | PASS |
| 7 | REQ-002 (ALIGNED sample, full) | Round-trip service suite re-executed 15/15; persistence schema suite 2/2 | PASS |
| 8 | REQ-017 (ALIGNED sample, full) | `project_persistence.schema.yaml` `$defs` parsed: ProjectEnvelope, HashMetadata, MigrationStatus, RoundTripManifest, ExternalArtifactReference, ValidationProfile, PersistenceOperation (+ ProfessionalBoundary, PrivateDataMarker, RulePackRef, …) all present | PASS |
| 9 | REQ-005 / REQ-012 | `core/serialization/canonical_json` re-executed 8/8 (RUST-23); H2 cross-referenced not verbatim-copied (conforming spread) | PASS |
| 10 | DECL-001..004 (non-ALIGNED, DECL-001/003 self-flagged) | Specification carries 29 TBD/PROPOSAL hits with container/migration/canonicalization framing overtaken by DEC-028/DEC-019/DEC-033 + implemented slice; Datasheet L44 "construction target is a three-artifact kit" (artifacts exist); Guidance L45 "Keep the physical package/container TBD until a human/architecture decision resolves it" and L46 "Record the exact schema layout/tooling decision … before implementation depends on it" (both overtaken); Procedure L66/L99 `public_fixture_data_boundary_review` PROPOSAL unrealized. 4× STALE correct; MEDIUM on Spec/Guidance appropriately reflects the still-true deliverable-scope reading. | PASS |
| 11 | REM-002 ClaimClass=MECHANICS (self-flagged) | Addendum-7 reading (numeric/hash-integrity correctness core) is defensible; a WORKFLOW reading was available; disclosed and aggregation-visible via ClaimType=REMAINING_WORK grain either way | QUALIFIED |
| 12 | Zero-count census, histograms, desktop-suite citation posture | IMPLEMENTED_UNMAPPED=0 confirmed (report_package correctly held outside orbit as PKG-08/17-owned); histograms reproduce; the pilot cites the R1 index's CONTENT_IDENTICAL binding for desktop suites rather than asserting the addendum-10 string itself, and marks the src-tauri crate recorded-only (RUST-37 NONE_FOUND) — the correct, honest posture | PASS |

Fences: held (incl. the shared-scratchpad note — no effect on outputs).
**Verdict: SOUND.**

---

## Package-level findings and cross-ledger consistency risks

1. **DEL-02-04 re-run required** (defects above). The other four ledgers'
   sibling treatment of setup-era kits gives the re-run pilot a determinate
   template.
2. **SECURITY marker dash variant:** DEL-02-04 uses the convention-6 em-dash
   (`NONE_FOUND — sufficiency…`); DEL-02-05 uses an ASCII hyphen. Any
   corpus-wide exact-string aggregation should match on the substring
   `sufficiency review deferred, owner-gated`; recommend the run standardize
   one form before W2+ mints more SECURITY rows (these are the wave's first).
3. **AuthorityNeeded on owner-gated SECURITY deferrals:** OWNER (DEL-02-04
   REQ-013) vs NO (DEL-02-05 REQ-013/REQ-025) for the same encoding.
   Convention 6 does not mandate the routing value, so neither is a defect,
   but an owner-attention census keyed on `AuthorityNeeded=OWNER` would
   undercount; recommend harmonizing to OWNER.
4. **Bootstrap gate-cell encoding split:** DEL-02-01/05 list the annotated
   `(gated: D-41)` suffix in the `_STATUS.md` row's `GateOrStageConstraint`;
   DEL-02-02/03/04 use `NONE_RECORDED`. Addendum 2 excludes the bootstrap
   from gate analysis either way; harmonization note only.
5. **RecordedRemaining ASCII transliteration** ("§"→"section") in
   DEL-02-01/05 vs literal "§" in DEL-02-02/04 breaks byte-exact verbatim
   joins on the bootstrap text (analysis-excluded, so low risk) and on
   DEL-02-05's real residuals (single-ledger, self-consistent).
6. **EXCLUSION row grain varies by ledger** (2/3/6/3/3 rows from differing
   grouping of out-of-scope bullets). All disclosed census choices;
   cross-package EXCLUSION counts are not comparable at face value.
7. **Correction to carry:** the DEL-02-02 notes-§7 detail — at the frozen SHA
   `engine.py` `CANONICAL_DIMENSIONS` INCLUDES `force_per_length` (29 ids;
   the delta vs the 30-id vocabulary is the `TBD` token); the D-01 packet E4
   omission is fixed. The duplicate-authority (mirror-not-call) risk routed
   toward DEL-16-02 remains real; route it without the stale omission claim.
8. **ACCEPTANCE=0 across all five ledgers** — package-uniform and
   brief-consistent, but W1 fan-in should confirm other packages applied the
   same restatement test so the corpus ACCEPTANCE grain stays comparable.

**Frozen-tree integrity:** `git status --porcelain` empty at start and end of
this verification (HEAD re-verified =
`551f84ef6be656f1603ce0acfa5e3935aa9683c7`); all re-executions used external
TMPDIR/CARGO_TARGET_DIR; no writes anywhere under the frozen worktree. This
review wrote exactly one file: `WAVES/W1/W1_VERIFICATION_PKG-02.md`.
