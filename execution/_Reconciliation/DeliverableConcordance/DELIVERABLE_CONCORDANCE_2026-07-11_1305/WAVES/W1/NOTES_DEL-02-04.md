# NOTES — DEL-02-04 Plugin and extension domain contracts (W1, R2)

Run: DELIVERABLE_CONCORDANCE_2026-07-11_1305 · Frozen tree SHA
`551f84ef6be656f1603ce0acfa5e3935aa9683c7` · Binding set: `R1_CONVENTIONS.md`
(conventions 1–8 + addenda 1–13). Deliverable lifecycle at frozen tree:
`IN_PROGRESS`. Type: `API_CONTRACT` (domain/API-contract deliverable — its work
product is the contract kit + manifest schema, not a runtime implementation).

## Path aliases (addendum 12)

- `KIT/` = `execution/PKG-02_Domain Model, Units, and Core Schemas/1_Working/DEL-02-04_Plugin and extension domain contracts/`
- All non-`KIT/` evidence paths (`schemas/…`, `fixtures/…`, `docs/…`,
  `tests/…`, `api/…`, `docs/_Registers/…`, `execution/_Decomposition/…`,
  `plans/…`, `validation/…`) are repo-root-relative under
  `projects/chirality-piping/` in the frozen tree.
- `NormativeSource` alias: requirement rows cite
  `KIT/Specification.md Requirements; DEL-02-04-REQ-NN` — the source
  requirement IDs are the Specification's own two-digit `DEL-02-04-REQ-01..18`.
  Per addendum 12 the ledger `ClaimID`s take the fixed three-digit form
  `DEL-02-04-REQ-001..018` (zero-padded from the source IDs; mapping is
  `REQ-0NN` ↔ `REQ-NN`). `EXC`/`DECL` ClaimIDs likewise use 3-digit `NNN`
  from 001.

## Disposition histogram (reproduces from CSV)

| Disposition | Count |
|---|---|
| ALIGNED | 24 |
| STALE_SETUP_SPECIFICATION | 2 |
| PARTIALLY_IMPLEMENTED | 1 |
| (all other dispositions) | 0 |

Total rows: 27.

## ClaimType histogram (reproduces from CSV)

| ClaimType | Count |
|---|---|
| REQUIREMENT | 18 |
| EXCLUSION | 3 |
| DECLARED_STATE | 6 |

Total rows: 27.

Census basis: 18 current requirement IDs (one REQUIREMENT row each,
substance disposition per convention 1); 3 EXCLUSION rows grouping the
Specification Scope exclusion sentence; 6 DECLARED_STATE rows per addendum 1
(Specification, Datasheet, Guidance, Procedure four-document kit + `_STATUS.md`
+ `MEMORY.md`). No ACCEPTANCE rows (see friction note 1). No
`IMPLEMENTED_UNMAPPED` rows: every material surface in this deliverable's orbit
(`schemas/plugin_manifest.schema.yaml` SURF-196, `fixtures/plugin_manifest/…`
SURF-159, `docs/architecture/extension_domain_contracts.md`,
`tests/test_plugin_manifest_schema.py` PY-53) is already attributed to
DEL-02-04 in the R1 indexes; `api/api_boundary_contract.yaml` (SURF-002) is
owned by DEL-10-01, cited only as companion evidence. The seeded
`(gated: D-41)` bootstrap item is recorded verbatim ONLY in the `_STATUS.md`
surface row (`DEL-02-04-DECL-005`) `RecordedRemaining`, and is excluded from all
residual/gate/selectability analysis (addendum 2) — hence every row carries
`GateOrStageConstraint=NONE_RECORDED`, `SelectableUnderCurrentLoop=NO`.

## Disposition rationale (post-repair profile: 24 ALIGNED / 2 STALE / 1 PI)

- The deliverable is a contract/documentation deliverable. Most requirements
  are of the form "the contract shall define/require X." The contract text
  (`Specification.md` + `docs/architecture/extension_domain_contracts.md`) and
  the manifest schema (`schemas/plugin_manifest.schema.yaml`) do define those
  things, and the re-executed schema/fixture test asserts the load-bearing
  constants. So at contract grain the substance disposition is `ALIGNED` for
  the requirement rows — except REQ-014, whose requirement text obligates
  layered verification checks with no deferral clause while only the
  schema/fixture layer is executable at the frozen SHA
  (`PARTIALLY_IMPLEMENTED`, matching the package-consistent DEL-02-02 REQ-012
  treatment of the same claim shape).
- The Datasheet and Procedure declared-state surfaces still frame the
  implemented slice as "Draft target" / "Future implementation should support"
  / "Future … record" work, declare a SEMANTIC_READY prerequisite against a
  current IN_PROGRESS lifecycle, and reference none of the implemented
  artifacts — the addendum-4 widened `STALE_SETUP_SPECIFICATION` trigger
  (see the Fan-in repair section).
- Embedded TBDs and their permitting basis, stated precisely (the original
  ledger's blanket "every embedded TBD is a named-decision-permitted deferral
  under DEC-010/DEC-012" over-reached): public API transport, import/export
  formats, rule expression grammar/library, dependency versions, CI provider,
  and coverage thresholds ARE named in DEC-012's enumerated retained-TBD list
  (human-approved 2026-04-30, `SOFTWARE_DECOMP.md` §12 row 590); detailed
  permissions carry ScopeLedger SOW-038's recorded "detailed permissions TBD"
  note. Exact schema file layout, codegen tooling, sandbox runtime mechanism,
  and the extension-point registry are NOT named in DEC-012's list — those
  TBDs are declared by the requirement texts themselves, and per addendum 11 a
  requirement-declared residual TBD without a permitting record still yields
  `ALIGNED` (not `ACCEPTED_DIVERGENCE`; there is no divergence at contract
  grain). All TBDs are carried in `RemainingWork` with authority routed.
- `SourceReliability=UNVERIFIED` on all requirement/exclusion rows: the
  technical evidence is project-original, agent-generated, with only an
  AGENT_CHECK self-review (`KIT/_REVIEW.md`); no named human disposition covers
  the contract's technical content (addendum 6). `DECLARED_STATE` prose rows are
  `NOT_APPLICABLE` (addendum 6).
- Confidence is `MEDIUM` on rows whose obligation is prose-only or only
  partially covered by the executable test (REQ-02/05/06/07/10/12/13/14 and the
  DECL-001/-003/-005/-006 surface rows), `HIGH` on rows whose exact constant is
  asserted by the re-executed schema test (REQ-01/03/04/08/09/11/17/18 and the
  exclusion rows) and on the two STALE surface rows (DECL-002/-004), whose
  contradiction with the frozen implemented slice is direct-inspection-exact.

## F-PIP-4 gate handling

The dispatch flagged that F-PIP-4 scope-gates external SDK/harness promotion and
domain-engine bindings behind named register rows, and that gated absence must
not be read as a defect. Two requirement rows sit on that boundary and are
encoded accordingly rather than downgraded:

- `DEL-02-04-REQ-013` (SECURITY — rule-pack sandbox): deny-by-default sandbox
  posture is implemented and test-asserted; the **sufficiency review** of the
  sandbox mechanism is deferred and owner-gated. Encoded per convention 6:
  `ValidationEvidence = "NONE_FOUND — sufficiency review deferred,
  owner-gated"`, `Disposition=ALIGNED` (no `VERIFIED_NOT_VALIDATED` downgrade on
  that ground), `AuthorityNeeded=OWNER`.
- `DEL-02-04-REQ-017` (registry/permission taxonomy/sandbox/approval path TBD):
  the interim contract grants no capabilities by default (schema
  `denied_by_default=true`, `grant_state='not_granted'`, both test-asserted);
  the approved registry/permission taxonomy/approval owner remain TBD behind a
  human ruling (DEC-012; SOW-038 notes). `Disposition=ALIGNED`,
  `AuthorityNeeded=OWNER`.

No F-PIP-1..5 output is asserted anywhere: REQ-08 records the *prohibition* of
certification/sealing/approval/compliance claims (it does not make one).

## Self-flagged rows

- **DEL-02-04-REQ-014** (layered verification, VALIDATION class): RESOLVED at
  fan-in repair — the original `ALIGNED` contract-grain reading was ruled
  overstated by the R2 verifier and the row is now `PARTIALLY_IMPLEMENTED`
  (MEDIUM), consistent with DEL-02-02 REQ-012's treatment of the same claim
  shape (see Fan-in repair section). No longer an open reviewer call.
- **DEL-02-04-REQ-013** (SECURITY): first exercise of the convention-6 SECURITY
  encoding in a substantive way for this deliverable; flagged for the Part C R2
  SECURITY-encoding spot-check if DEL-02-04 is the wave's first SECURITY-class
  row (that determination is the orchestrator's, not mine).
- **DEL-02-04-DECL-001 / -003 (Specification / Guidance)**: the kit cites
  decomposition **revision 0.7** as current basis, but the frozen
  `execution/_Decomposition/SOFTWARE_DECOMP.md` is **revision 0.8**
  (SCA-005 / D-21). The plugin-contract substance is unaffected by the v0.8
  amendment (SCA-001/DEC-010/DEC-012/AB-00-07 basis is unchanged), so these two
  surfaces stay `ALIGNED` with the revision lag noted as metadata drift rather
  than `STALE_SETUP_SPECIFICATION` — a reading the R2 verifier examined and
  graded defensible on this ground alone (Guidance's TBDs are genuinely still
  open). Reviewer may still disagree if v0.8 is judged to materially
  re-describe this slice. (DECL-002, previously in this flag, is now
  `STALE_SETUP_SPECIFICATION` on the addendum-4 Draft-target ground — see
  Fan-in repair section — with the rev-lag folded into its repair scope.)
- **DEL-02-04-DECL-003 (Guidance)**: carries a now-stale statement of
  CONF-02-04-001 — it claims `KIT/_REFERENCES.md` still reads "accepted v0.2",
  but the frozen `_REFERENCES.md` has been updated to "revision 0.7", so the
  files now agree and the recorded conflict is resolved/stale. Noted on the
  surface row; not raised to a separate `REMAINING_STATE_MISMATCH` row because
  it lives inside the kit's own self-declared conflict/traceability prose.
  (The Datasheet's copy of the same stale note is now covered by DECL-002's
  STALE repair scope. `KIT/_REVIEW.md` RF-001 and its XD-004 entry are likewise
  stale on this point; `_REVIEW.md`/`Review_Findings.csv` are not part of the
  addendum-1 DECLARED_STATE census — no README — so they get no own row.)
- **DEL-02-04-DECL-004 (Procedure)**: RESOLVED at fan-in repair — the original
  ledger read the "For this Pass 3 run, state is `SEMANTIC_READY`" framing as
  historical authoring context and kept `ALIGNED`, self-flagging it as the
  single most debatable disposition and inviting elevation. The R2 verifier
  confirmed the elevation (the Procedure also anticipates as "Future" the
  verification records whose subject artifacts now exist); the row is now
  `STALE_SETUP_SPECIFICATION` (see Fan-in repair section).

## Fan-in repair (fable re-run)

Owner-ruled R2 fan-in repair: the W1 verification package
(`W1_VERIFICATION_PKG-02.md`) graded this ledger DEFECTIVE with three named
defects; a fable repair pilot independently re-verified each against the
frozen tree before re-encoding and now owns the changed claims. All repaired
dispositions remain agent judgments routed via `AuthorityNeeded`.

Independent re-verification (all confirmed; no disagreements with the
verifier):

- Frozen `KIT/Datasheet.md` Construction rows 60/61/66 carry "Draft target"
  (plugin interface spec; sandbox/permission model notes) and "Future
  implementation should support schema validation, unit checks,
  protected-content/provenance gates, and adapter/plugin tests", while
  `schemas/plugin_manifest.schema.yaml`,
  `fixtures/plugin_manifest/invented_manifest_no_bypass.json`,
  `tests/test_plugin_manifest_schema.py` (re-executed, 2 passed), and
  `docs/architecture/extension_domain_contracts.md` all exist at the frozen
  SHA; a grep across the four kit documents finds zero references to
  `plugin_manifest`, `extension_domain_contracts`, or `test_plugin`.
- Frozen `KIT/Procedure.md` L19 declares "For this Pass 3 run, state is
  `SEMANTIC_READY`" (current `_STATUS.md` is IN_PROGRESS); L58 "define a
  future verification path"; L88–90 three "Future … record" expectations whose
  subject artifacts now exist.
- Frozen `KIT/Specification.md` L35 REQ-14: "…verification **shall include**
  layered checks…" with no deferral clause; only the schema/fixture layer is
  executable at the frozen SHA.
- Frozen `SOFTWARE_DECOMP.md` L590 DEC-012 enumerates dependency versions,
  solver numerical library, rule expression grammar/library, public API
  transport, import/export format list, CI provider, coverage thresholds, and
  physical package/container — it does NOT name schema file layout, codegen
  tooling, sandbox runtime mechanism, or the extension-point registry.

Rows changed (old → new):

| ClaimID | Old | New | Ground |
|---|---|---|---|
| DEL-02-04-DECL-002 (Datasheet) | ALIGNED (MEDIUM, AuthorityNeeded=NO) | STALE_SETUP_SPECIFICATION (HIGH, OWNER; R5 repair candidate) | Addendum-4 widened STALE trigger: Draft-target/future-implementation framing vs implemented, unreferenced frozen slice; sibling-consistent (DEL-02-01/-02/-05 Datasheets) |
| DEL-02-04-DECL-004 (Procedure) | ALIGNED (MEDIUM, NO) | STALE_SETUP_SPECIFICATION (HIGH, OWNER; R5 repair candidate) | Same ground + SEMANTIC_READY prerequisite vs IN_PROGRESS; original pilot's own elevation flag confirmed by the verifier |
| DEL-02-04-REQ-014 | ALIGNED (MEDIUM) | PARTIALLY_IMPLEMENTED (MEDIUM) | Requirement obligates layered checks with no deferral clause; only schema/fixture layer executable; package-consistent with DEL-02-02 REQ-012 (ValidationEvidence NOT_APPLICABLE → NONE_FOUND accordingly) |

Wording repairs (dispositions unchanged, per the verifier's QUALIFIED items):

- **DEL-02-04-REQ-011** `DecisionBasis` and `RemainingWork`: removed "layout
  deferral permitted by DEC-012 / DEC-010/DEC-012" — DEC-012's enumerated list
  does not name schema layout or codegen; the TBD is requirement-declared
  (ALIGNED per addendum 11 residual-TBD handling).
- **DEL-02-04-REQ-016** `DecisionBasis`: same correction (transport-binding
  TBD is DEC-012-listed; layout/field-names/codegen are requirement-declared).
- Notes "Disposition rationale" section: the blanket "every embedded TBD is a
  named-decision-permitted deferral under DEC-010/DEC-012" claim replaced with
  the precise DEC-012/SOW-038 vs requirement-declared partition.

Carried unchanged on verifier PASS: REQ-013's convention-6 SECURITY encoding
(marker string, ALIGNED, AuthorityNeeded=OWNER) and the
SourceReliability=UNVERIFIED ladder on all technical rows.

Repair evidence execution: `tests/test_plugin_manifest_schema.py` re-executed
by the repair pilot at frozen SHA 551f84ef6 (pytest `-p no:cacheprovider`,
cache_dir in scratch, `PYTHONDONTWRITEBYTECODE=1`: **2 passed**; direct
`python3` run: **pass**); all other checks were read-only inspections/greps of
the frozen tree; `git status --porcelain` empty before AND after.

## Evidence-execution log

- **Re-executed (side-effect-free, addendum 9):**
  `tests/test_plugin_manifest_schema.py` at the frozen SHA, from
  `projects/chirality-piping/`, with `PYTHONDONTWRITEBYTECODE=1` and
  `pytest -p no:cacheprovider` (pytest cache_dir redirected to scratch;
  option unknown-warning is harmless, cache already disabled by the plugin
  flag). Result: **pytest 2 passed** (`test_plugin_manifest_schema_contract`,
  `test_plugin_manifest_fixture_preserves_no_bypass_defaults`); direct
  `python3 tests/test_plugin_manifest_schema.py` **passed**. The suite is pure
  stdlib (`json`, `pathlib`), no build.
  `git -C <FROZEN> status --porcelain` was **empty before AND after** both runs.
- **Cited as recorded (not re-executed):** PY-53 recorded pass
  `validation/evidence/sweeps/SWEEP_20260711T040758Z_e648462f1d05.json` on
  ancestor commit `e648462f1d05…`, carried as `CONTENT_IDENTICAL` in
  `VERIFICATION_INDEX.csv`. My own re-execution supersedes the need for the
  addendum-10 content-identical qualifier, so requirement rows cite the direct
  re-execution as the primary verification marker and reference the sweep as the
  recorded corroborating pass. `KIT/_REVIEW.md` (AGENT_CHECK self-check) is cited
  with `not re-executed at frozen SHA 551f84ef6` for prose-only obligations.
- No other checks were run (no validation harness exists for this deliverable;
  none re-runnable side-effect-free was identified beyond PY-53).

## Convention-friction notes

1. **Acceptance rows.** The Specification carries a requirement→verification
   matrix plus a five-row interim gate table (Schema/manifest review, Layered
   plugin/adapter, Rule-pack sandbox, Human-ruling, Diagnostic class). Per the
   brief and addendum 12, verification tables that merely restate/bundle
   requirements do NOT get mirrored ACCEPTANCE rows. The gates bundle the
   existing requirements with pass/HOLD conditions rather than introducing new
   acceptance grain, so I created **zero** ACCEPTANCE rows and folded the gate
   HOLD conditions into the affected requirement rows' `RemainingWork` /
   `AuthorityNeeded`. Flagged in case the reviewer wants the sandbox and
   human-ruling gates surfaced as their own acceptance claims.
2. **`GateOrStageConstraint` vs. Specification "gates".** Convention 5 (D5) ties
   `GateOrStageConstraint` to recorded `_STATUS.md ## Remaining` residuals, of
   which the only one is the excluded D-41 bootstrap. The Specification's
   verification "gates" are contract-internal acceptance criteria, not recorded
   residual gates, so they do NOT populate `GateOrStageConstraint`; all rows are
   `NONE_RECORDED`. Called out because "gate" is overloaded between the two
   surfaces.
3. **ClaimID digit width (repaired at wave fan-in direction).** The
   Specification's own requirement IDs are two-digit (`DEL-02-04-REQ-01..18`).
   The ledger initially carried them verbatim; on coordinator structural-
   validation direction they were re-padded to the addendum-12 fixed
   three-digit form (`DEL-02-04-REQ-001..018`) for the corpus-wide join.
   Traceability: ClaimID `REQ-0NN` ↔ Specification `REQ-NN` (the
   `NormativeSource` cells retain the Specification's two-digit IDs verbatim).
   `EXC`/`DECL` IDs were already 3-digit.
4. **ClaimClass for contract-of-behavior requirements.** Several requirements
   describe downstream *behavior* (unit reporting, hashing, command/query/job)
   but at this deliverable they are schema/governance *contract* claims. I
   classed by the contract artifact (SCHEMA / GOVERNANCE / REPORTING / SECURITY
   / VALIDATION / DOCUMENTATION) rather than by the eventual runtime behavior,
   and set `ValidationEvidence=NOT_APPLICABLE` with an in-cell reason (no
   numeric/engineering behavior is realized here to validate). No row was
   promoted from unit-test evidence to engineering validation.

## Boundary-compliance statement

- All fences held. Discovery was read-only; no lifecycle transition applied
  (`LIFECYCLE_REASSESSMENT_REQUIRED` not used — none warranted); no DAG
  mutation; no cross-project edit; no edit to any `_STATUS.md`, register, or
  product file; no `DEFERRED_AGENT_WORKFLOW` implications acted on.
- No release-readiness, issuance, certification, sealing, professional-approval,
  or code-compliance claim appears anywhere in the outputs (F-PIP-1..5).
- All dispositions are agent judgments, routed via `AuthorityNeeded`; none is
  phrased as an owner or engineering ruling.
- Writes confined to exactly two files under `WAVES/W1/`
  (`CLAIM_CONCORDANCE_DEL-02-04.csv`, `NOTES_DEL-02-04.md`).
- Frozen evidence tree: `git status --porcelain` empty before and after all
  reads and the side-effect-free re-execution.
