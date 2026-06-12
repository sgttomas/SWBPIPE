# OpenPipeStress — PRD Completion Plan

**Date:** 2026-06-10
**Epistemic status:** PROPOSAL (non-governing). This plan proposes a route from the current technical preview to the product defined in [docs/PRD.md](../docs/PRD.md). It does not change any lifecycle state, promote any deliverable, or create any release, professional, certification, or code-compliance claim. Every work item below is a *candidate tranche* to be proposed, approved, executed, and evidenced through the Application Integration And Issuance Loop in [execution/_Coordination/_COORDINATION.md](../execution/_Coordination/_COORDINATION.md). Humans decide all gates.

**Baseline:** [plans/ASSESSMENT_2026-06-10_repo_and_app_state.md](ASSESSMENT_2026-06-10_repo_and_app_state.md) (same date; all test surfaces green: 340/340 pytest, 422 Rust `#[test]`s across 25 crates, 13/13 Vitest, desktop production build passing).

**Plan maintenance:** this plan is a selection instrument, not a history. When an item lands, compress its row to one line — `LANDED <date> (<tranche id>)`, residual hand-offs, and pointers to the run record and [PLAN_COMPLETION_LOG.md](PLAN_COMPLETION_LOG.md) — and move the narrative detail to that log. Partially-landed items keep their remaining scope in the row and push landed detail to the log. Decision rows carry at most state, a `DEC-xxx` pointer, and a one-clause outcome.

---

## 1. Definition of "complete per the PRD"

The PRD defines completion through two surfaces, and this plan treats both as binding:

1. **Release milestones (PRD §22).** R0–R5 exit criteria, taken verbatim. R5 (Engineering Beta) is the terminal milestone: "External engineers can reproduce validation examples" and "Public repository contains no known protected standards data."
2. **Functional requirements (PRD §10).** All 16 `Must` requirements (FR-001..016) verified against their acceptance criteria; all 6 `Should` requirements (FR-017..022) implemented; the 3 `Could` requirements (FR-023..025) either implemented or **explicitly dispositioned by a human deferral record** at the R5 gate — silence is not a disposition.

Completion additionally requires the human-gated governance closures that the PRD and CONTRACT make prerequisites for any release claim (unit catalog acceptance, thresholds, CI/release authority, issuance of deliverables). These are scheduled as decisions, not assumed.

### Current milestone position (from the 2026-06-10 assessment)

| Milestone | Status | Blocking residual |
|---|---|---|
| R0 Architecture Prototype | Met in substance | Note: R0's "Unit system" deliverable was deferred by governance gating; it is pulled back onto the critical path here (Phase B) |
| R1 Core Solver MVP | Substantially met (linear static, dense solve) | Tolerance thresholds for benchmark evidence remain TBD (Decision D-04) |
| R2 GUI MVP | **Not met** | No intent→operation→persisted-model loop; no deformed-shape plot; no rendered report (Phase A) |
| R3 Rule packs + private libraries | Engine-side largely met | Expression grammar freeze; rule-pack editor GUI; private library management GUI (Phase C) |
| R4 Components + nonlinear supports | Schema/data-model only | No bend/branch/expansion-joint/hanger elements; no assembled nonlinear iterative solve (Phase D) |
| R5 Engineering Beta | Distant | All release machinery TBD; validation manual; redaction workflow; signed releases (Phase E) |

---

## 2. Critical-path human decisions (Decision Register)

These are the decisions only a human project authority can make. Each blocks specific phases; sequencing them early is the cheapest acceleration available to the project. Proposed mechanism: one bounded *decision-preparation tranche* per item that assembles options + evidence into a decision packet, then a recorded human acceptance (SCA/DEC-style entry in [SOFTWARE_DECOMP.md](../execution/_Decomposition/SOFTWARE_DECOMP.md) or a successor register).

| ID | Decision | Blocks | Proposed timing |
|---|---|---|---|
| D-01 | **Unit catalog acceptance**: canonical unit set, conversion constants, offset-temperature and gauge/absolute-pressure semantics, tolerance policy (SPEC TBDs) | Phase B entirely; full value of Phase A authoring; FR-002 | **RULED 2026-06-10** (`DEC-018`: SI-canonical with dual display catalog, as the packet proposed) — Phase B unblocked |
| D-02 | **Rule-pack expression grammar freeze** (final operator/function set, conformance suite) | Phase C; FR-011 final form | **RULED 2026-06-11** (`DEC-022`: packet Option A — frozen typed AST extended to the PRD §12.3 function set; D-02b text-syntax follow-up at the C2 lead-up) — Phase C grammar work unblocked |
| D-03 | **Sparse solver / model-scale strategy** (accept a sparse library or bounded dense limit with diagnostics) | Phase D scale targets; PRD §23.1 "solver stability for large models" | **RULED 2026-06-11** (`DEC-023`: packet Option C — hand-rolled in-repo sparse skyline/profile direct solver, zero new dependencies, determinism posture preserved) — Phase D scale path decided |
| D-04 | **Numerical tolerance + coverage thresholds** (benchmark permitted variance, regression coverage floors) — RGAP-004 | R1/R4/R5 evidence claims; RELEASE_QUALITY_GATES | **RULED 2026-06-11** (`DEC-024` revised same day by `DEC-026`: T-C + C-C with riders — class-tiered governed relative+absolute tolerance pairs, analytic class at measured 1.0e-9, tighten-only fixture overrides with loosening as a governance event; blocking inventory gates + recorded-not-blocking numeric coverage; D-04b tooling follow-up) |
| D-05 | **CI provider + hosted workflow location** — RGAP-003 | Phase E; continuous evidence for all phases | **RULED 2026-06-11** (`DEC-025`: packet Option D — five-surface local sweep codified as the deterministic commit-bound merge gate for agent branches, F-4 atomic-build rider; D-05b public-export CI activation with D-06). Implementation LANDED 2026-06-11, `TP-SWEEP-001` (`tools/release/run_evidence_sweep.py` + F-4 fix; details in the [completion log](PLAN_COMPLETION_LOG.md); run record in DEL-10-04) |
| D-06 | **Release matrix, installer formats, signing/notarization, publication targets** — RGAP-003/006 | Phase E packaging | With or after D-05 |
| D-07 | **Maintainer quorum + release authority; contributor legal mechanism** | Any release claim; R5 "IP contribution process" | **RULED 2026-06-11** (`DEC-027`, ruled directly: sole-maintainer quorum and release authority; external contributions closed at this time; D-07b gates any future intake; R5 milestone mapping dispositioned at the R5 gate with D-12) |
| D-08 | **Model-document schema migration policy** (versioning + migration ledger semantics for model documents, extending the store `user_version` ledger) | Phase A persistence (A2) | **RULED 2026-06-10** (`DEC-019`: per-document semver transform chain, migrate-in-memory-on-open / persist-on-save, as the packet proposed) — A2 unblocked |
| D-09 | **Native package physical container format** (single-file project container + public transport form) | Phase E distribution of projects; FR-001 "version" semantics at file level | **RULED 2026-06-11** (`DEC-028`: packet Option C — multi-member archive per the PKG-17 manifest contracts; canonical-JSON members carry the truth, evidence binds to members + manifest hashes; SQLite store stays a local projection; naming with D-06) |
| D-10 | **Report rendering target** (deterministic HTML and/or PDF pipeline choice) | Phase A report rendering (A7); R5 "full report package" | **RULED 2026-06-11** (`DEC-021`: packet Option B — deterministic hash-bound single-file HTML via a Rust renderer crate, plus webview print-to-PDF labeled a derived non-hash-bound view; D-10b PDF-emitter follow-up deferred to the R5 lead-up) — A7 unblocked |
| D-11 | **Issuance waves** for the 100 `CHECKING` deliverables | Governance closure (Phase F) | Human-paced throughout; suggested wave alignment in §5 Phase F |
| D-12 | **Disposition of FR-024 (dynamics) and FR-025 (local FEA export)** (`Could` priority): implement post-beta or record explicit deferral | Final PRD-completeness claim | At R5 gate |

---

## 3. Phase plan

Phases are ordered by dependency, not strictly by execution: A and B can run in parallel lanes; element work in D can start crate-side early. Each numbered item is sized to be one or a small series of bounded tranches (suggested naming continues the existing `TP-*` convention, e.g. `TP-APP-R2-EDITLOOP-001`). Evidence expectations per tranche follow the coordination loop: `npm test`/`build --workspace apps/desktop`, applicable `cargo test` and `pytest` surfaces, and smoke/UI evidence when behavior changes.

### Phase A — Close R2: the GUI MVP (largest phase; the product hinge)

**Objective:** PRD §22.3 exit criterion verbatim — "User can create, solve, and report a small piping model without editing raw files." Closes FR-003, FR-013, FR-014, FR-015, FR-016 residuals; FR-001 model-level round-trip; FR-007 GUI surface.

| # | Tranche scope | Key seams (current code) |
|---|---|---|
| A1 | **Apply-operation command path** — **LANDED 2026-06-10** (`TP-APP-R2-EDITLOOP-001`); residual hand-offs: gesture geometry capture → A3, unit conversion → Phase B. Detail: [completion log](PLAN_COMPLETION_LOG.md); evidence: DEL-16-02 run record, SMOKE TP-MAC-82 | `apply_model_operation` / `validate_model_operation` in [lib.rs](../apps/desktop/src-tauri/src/lib.rs); PKG-16 contracts |
| A2 | **Model-document persistence** — **LANDED 2026-06-10** (`TP-APP-R2-PERSIST-001`, per `DEC-019`); residuals: compatibility-window size (human ruling), explicit "Migrate project" operation, sibling JSON-slot coverage. Detail: [completion log](PLAN_COMPLETION_LOG.md); evidence: DEL-02-05 run record, SMOKE TP-MAC-83 | store v9 ledger + in-document semver (landed) |
| A3 | **Viewport editing UX.** Remaining scope: canvas gesture capture beyond node drafting and endpoint picking, component/rigid authoring, and broader editor coverage as new authoring surfaces land. Landed sub-slices 2026-06-10/11, each detailed in the [completion log](PLAN_COMPLETION_LOG.md): `TP-APP-R2-VIEWSELECT-001`, `-INLINEVALID-001`, `-CREATENODE-001`, `-UNDOREDO-001`, `-CONNECTPIPE-001`, `-CANVASNODE-001`, `-PIPEPICK-001` (SMOKE TP-MAC-84..87, 92..94; evidence in DEL-07-01/DEL-07-02/DEL-16-02/DEL-16-03 run records) | [PipeViewport.tsx](../apps/desktop/src/features/viewport/PipeViewport.tsx); model tree + property panels per PRD §14.1–14.3 |
| A4 | **Load case manager UI.** Remaining scope: subtraction/range expression authoring, Phase B unit picker/display retirement, and packaged-Tauri saved-project smoke over edited load data. Landed sub-slices 2026-06-11, each detailed in the [completion log](PLAN_COMPLETION_LOG.md): `TP-APP-R2-LOADMGR-001`, `-LOADMETA-001`, `-COMBFACTOR-001`, `-LOADCREATE-001`, `-PRIMCREATE-001`, `-DISTLOAD-001`, `-MOMENTCREATE-001`, `-PRESSTEMP-001`, `-IMPOSED-001`, `-COMBBASIS-001`, `-COMBTERMCREATE-001`, `-COMBTERMDELETE-001`, `-COMBCREATE-001` (SMOKE TP-MAC-91, 95..106; evidence in DEL-05-01/DEL-05-02/DEL-05-05/DEL-04-03/DEL-07-02/DEL-16-02/DEL-16-03 run records) | `core/loads` crates already implement the algebra; PRD §11.5–11.6, FR-007 |
| A5 | **Solve-from-edited-model.** Remaining scope: full packaged-Tauri GUI smoke over a saved edited project snapshot, UI polish for incomplete-model diagnostics, and broader persisted solve coverage as new authoring surfaces grow. Landed sub-slices 2026-06-11, each detailed in the [completion log](PLAN_COMPLETION_LOG.md): `TP-APP-R2-SOLVEBOUND-001`, `-PERSISTEDSOLVE-001` (SMOKE TP-MAC-88, 107; evidence in DEL-02-05/DEL-07-07/DEL-14-02/DEL-04-06 run records); solve-cancellation backend token landed separately 2026-06-10 | `run_preview_mechanics` / job commands; [core/product_physics](../core/product_physics/Cargo.toml) bridge |
| A6 | **Results visualization.** Remaining scope: true directional deformed shape once displacement vectors exist, governing-ratio views once ratio rows exist, and richer result-selection coupling. Landed sub-slices 2026-06-11, each detailed in the [completion log](PLAN_COMPLETION_LOG.md): `TP-APP-R2-DEFORMEDVIEW-001`, `-RESULTFAMILY-001` (SMOKE TP-MAC-89, 108; evidence in DEL-07-01/DEL-07-05 run records) | Three.js viewport; existing results envelopes |
| A7 | **Report rendering — LANDED 2026-06-11** (`TP-APP-R2-REPORTRENDER-001`, per `DEC-021`); residual hand-offs: browser-mode render seam decision → A8, rule-pack refs populate → Phase C, report-hash persistence → small follow-up. Detail: [completion log](PLAN_COMPLETION_LOG.md); evidence: DEL-08-01 run record, SMOKE TP-MAC-113 | `core/reporting/report_renderer` + `render_calculation_report` command + Rendered Report panel |
| A8 | **GUI test harness.** Remaining scope: full [SMOKE.md](../apps/desktop/SMOKE.md) checklist parity if every manual row must become an automated assertion. Browser Playwright now automates A12 from-blank authoring and records the honest desktop-only browser seams; Tauri backend regression now saves, reopens, solves, and renders the A12-authored model. The CI browser-provisioning policy question is ruled (**2026-06-11** `DEC-025`: the local five-surface sweep is the commit-bound merge gate; hosted CI deferred to D-05b). Landed sub-slices detailed in the [completion log](PLAN_COMPLETION_LOG.md): `TP-APP-R2-PLAYWRIGHT-001` (2026-06-11, SMOKE TP-MAC-90), `TP-APP-R2-FROMBLANK-E2E-001` (2026-06-12, SMOKE TP-MAC-127), `TP-APP-R2-SAVEDPROJECT-SMOKE-001` (2026-06-12, SMOKE TP-MAC-128; evidence in DEL-00-08 run records) | Playwright harness plus existing Vitest/Tauri suites; A12 fixture `fixtures/product_preview/r2_from_blank_rehearsal.json` |
| A9 | **Blank-project authoring path — LANDED 2026-06-12** (`TP-APP-R2-BLANK-001`); residual hand-offs: A10 support/material/section creation, A11 deletion coverage, packaged-Tauri saved-project blank smoke. Detail: [completion log](PLAN_COMPLETION_LOG.md); evidence: DEL-02-05 run record, SMOKE TP-MAC-116 | `buildBlankLocalModelDocument` + local project create/open/save boundary; Tauri solve path returns `MODEL_INCOMPLETE` for blank payloads |
| A10 | **From-scratch entity creation set — LANDED 2026-06-12** (`TP-APP-R2-CREATESUPPORT-001`, `TP-APP-R2-CREATEMATERIAL-001`, `TP-APP-R2-CREATESECTION-001`). The structured operation seam, contract corpus, model tree, and Property Inspector now cover explicit support, material, and pipe-section creation. Detail: [completion log](PLAN_COMPLETION_LOG.md); evidence: DEL-16-02/DEL-16-03/DEL-07-02 run records, SMOKE TP-MAC-117..119. Residual hand-offs: A8 GUI/e2e automation of the A12 script; optional later pipe-form reuse of standalone section refs | `core/model_operations/operation_applier` + contract corpus + viewport/inspector forms |
| A11 | **Entity deletion coverage - LANDED 2026-06-12** (`TP-APP-R2-DELSUPPORT-001`, `TP-APP-R2-DELPRIMLOAD-001`, `TP-APP-R2-DELCOMBINATION-001`, `TP-APP-R2-DELLOADCASE-001`, `TP-APP-R2-DELPIPE-001`, `TP-APP-R2-DELNODE-001`; `delete_combination_term` already existed). The structured operation seam, contract corpus, model tree, Property Inspector, and Load Cases manager now cover accepted deletion/refusal semantics for the R2 entity families, with 57-case corpus coverage including `OP-SUPPORT-DELETE-REFERENCED`, `OP-LOAD-CASE-DELETE-REFERENCED`, `OP-PIPE-DELETE-REFERENCED`, and `OP-NODE-DELETE-REFERENCED`. Detail: [completion log](PLAN_COMPLETION_LOG.md); evidence: DEL-16-02/DEL-16-03/DEL-07-02 run records, SMOKE TP-MAC-120..125. Residual hand-off: A8 journey automation | same seam; corpus cases pin the refusal semantics |
| A12 | **From-scratch R2 exit rehearsal - LANDED 2026-06-12** (`TP-APP-R2-FROMBLANK-REHEARSAL-001`). Invented fixture and Tauri regression author a minimal model from blank through structured operations (nodes → material/section → pipe run → support → load case → primitive load → combination), solve through the backend preview mechanics command, and render the A7 report. Detail: [completion log](PLAN_COMPLETION_LOG.md); evidence: DEL-07-07 run record, SMOKE TP-MAC-126. Residual hand-off: A8 GUI/e2e automation of the A12 script | exercises A9–A11 + A5 + A7 together |

**Phase exit evidence:** scripted end-to-end run of the R2 exit criterion on an invented model; all suites green; smoke automation replacing the manual checklist; derivative verification snapshot (TP-INTEGRATED-VERIFY successor) for human review.

### Phase B — Units engine (D-01 **RULED** `DEC-018`; parallel lane to Phase A)

**Objective:** FR-002 acceptance criteria — every numerical field carries units, incompatible units rejected, reports show units. Retires the project's most consequential deliberate TBD.

| # | Tranche scope |
|---|---|
| B1 | **Unit catalog + conversion crate - LANDED 2026-06-12** (`TP-UNITS-B1-CATALOG-001`): crate-side `core/units` now implements DEC-018 SI-canonical catalog/conversions, dimension algebra, absolute/interval temperature handling, and gauge/absolute pressure handling with explicit references. Detail: [completion log](PLAN_COMPLETION_LOG.md); evidence: DEL-02-02 run record and historical crate-local `core/units/_run_records/TASK_RUN_2026-06-12_0136.md` (future process records should stay in deliverable-local evidence folders). Residual hand-offs: B2 schema/app/solver/report unit I/O and B3 conversion-witness/tolerance corpus |
| B2 | **Unit-aware I/O**: schema field bindings, app input fields with unit display/entry, solver-boundary normalization, reports showing units used. Landed sub-slices: `TP-UNITS-B2B3-CONTRACT-001` exposed crate per-constant `factor_representation`, optional offset text, explicit provenance including `ConventionalPublicConstant` / `ProjectGovernedDecision`, and review status; `TP-UNITS-B2-CATALOGCMD-001` exposed that catalog through desktop `get_unit_catalog` with Tauri coverage; `TP-UNITS-B2-FRONTENDSVC-001` added the typed frontend service route with explicit browser-unavailable behavior; `TP-UNITS-B2-INSPECTORLABELS-001` added catalog-aware Property Inspector material/section unit-basis labels and status without changing payload units; `TP-UNITS-B2-REPORTUNITS-001` added Report Packet unit-system disclosure and DEC-018 rendered-report input reference. Remaining scope: broader visible app unit entry/pickers, solver-boundary normalization, report renderer body expansion beyond packet disclosure, imports/exports, and rule-pack unit I/O. |
| B3 | **Mixed-unit round-trip + tolerance tests**: conversion witnesses, rejection tests for incompatible units, tolerance policy per D-04. Rider landed 2026-06-12 (`TP-UNITS-B2B3-CONTRACT-001`): Rust schema-parity regression asserts crate dimension vocabulary set equality against `schemas/units.schema.yaml` `DimensionId`. Remaining scope: broader mixed-unit round-trip, conversion-witness, incompatible-unit rejection, and D-04/DEC-026 tolerance corpus. |

Interim posture until B lands: Phase A authoring may proceed SI-consistent with an explicit visible "single unit system (preview)" label — no silent unit assumptions (CONTRACT no-silent-defaults).

### Phase C — Close R3: rule packs and private libraries

**Objective:** PRD §22.4 exit criteria — user defines a private non-code rule pack and runs checks; software blocks pass/fail on missing inputs (already engine-true, must be GUI-true). Closes FR-011 final form, FR-012 GUI surface, FR-022.

| # | Tranche scope |
|---|---|
| C1 | **Expression grammar freeze — LANDED 2026-06-11** (`TP-C1-GRAMMAR-001`, per `DEC-022`, grammar v1.0.0); residual hand-offs: `schemas/rule_pack.schema.yaml` grammar_version/table additions, DEL-06-02 spec rows at formal review, DEC-012 grammar-TBD retirement, exact-string unit semantics pending B1; three labeled ASSUMPTIONs flagged for human review in the run record. Detail: [completion log](PLAN_COMPLETION_LOG.md); evidence: DEL-06-02 run record |
| C2 | **Rule-pack editor GUI** (PRD §14.5): authoring, validation, checksum/lifecycle surfacing from the existing lifecycle/checksum module |
| C3 | **Private library management GUI** (PRD §13, §14.6): import wizard for materials/sections/components with provenance capture, §13.5 import warnings, and strict private-path handling (never redistributed, per IP boundary) |
| C4 | **End-to-end rule checks on authored models**: `USER_RULE_CHECKED` / `USER_RULE_FAILED` / `RULE_INPUTS_INCOMPLETE` statuses driven from solves of user-authored (not fixture) models into result tables |

### Phase D — Close R4: piping components and nonlinear supports (solver depth)

**Objective:** PRD §22.5 exit criteria — nonlinear support validation cases converge; component provenance appears in reports. Closes FR-017..021. All component factors are **user-entered** (SIFs, flexibility factors, stiffnesses) per PRD §9.2 — the code-neutral boundary means the solver never computes code-derived factors from protected tables.

| # | Tranche scope |
|---|---|
| D1 | **Bend element**: curved-pipe behavior via user-entered flexibility factors and SIFs (FR-017) applied to element stiffness/stress recovery; radius/angle/orientation geometry in model + viewport |
| D2 | **Branch connection objects** (FR-018): geometry + user-entered local modifiers for header/branch stress recovery |
| D3 | **Rigid/semi-rigid components** (FR-019): valves, flanges, reducers as rigid or semi-rigid elements with user-entered dimensions/weights/source notes (rigid elements already exist in the kernel; this is mostly data model + mapping + provenance) |
| D4 | **Expansion joints** (FR-020): element with user-entered stiffness set, effective pressure area, movement limits, manufacturer provenance |
| D5 | **Spring hangers** (R4 deliverable): variable/constant-effort hanger supports, user-entered properties |
| D6 | **Assembled nonlinear iterative solve** (FR-021): wrap the existing active-set state classifier in a global iteration loop (gaps, one-way, lift-off, friction) with convergence criteria mapped to the existing nonconvergence diagnostics taxonomy |
| D7 | **Scale strategy** per D-03. First slice LANDED 2026-06-11 (`TP-D03-SPARSE-001`, per `DEC-023`): `core/solver/sparse_direct` crate (deterministic RCM ordering, skyline storage, in-repo LDLT) + diagnostics integration + performance-harness sparse lane; detail in the [completion log](PLAN_COMPLETION_LOG.md), evidence in the DEL-04-01 run record. Remaining scope: live frame_kernel/product_physics solve-path adoption (handoff plan in the run record), profile-direct assembly for PRD §20 scale |
| D8 | **Validation set**: nonlinear convergence benchmarks with hand-calc/witness evidence; component provenance fields flowing into rendered reports (R4 exit) |

### Phase E — Close R5: engineering beta and release machinery

**Objective:** PRD §22.6 exit criteria — external engineers can reproduce validation examples; no known protected standards data in the public repository. Resolves RGAP-002/003/004/005/006.

| # | Tranche scope |
|---|---|
| E1 | **Headless runner CLI**: add the binary + CLI/process policy to [core/runner/headless](../core/runner/headless) (DEL-10-05 TBDs; `init/init-prompt.md` seam) so validation examples are reproducible without the GUI |
| E2 | **Validation manual** (PRD §16.5): assembled from benchmark witnesses + tolerances accepted under D-04; reproduction instructions exercised via E1 |
| E3 | **Full report package + redaction workflow** (R5 deliverables; PRD §17.3, §18): export controls, private-data redaction tooling for bug reports/examples |
| E4 | **CI + release implementation** per D-05/D-06: provider workflows mapped to the provider-neutral phases in [BUILD_AND_RELEASE.md](../docs/BUILD_AND_RELEASE.md) §7; release matrix, packaging, signing/notarization; signed releases |
| E5 | **Release artifact protected-content scan** (RGAP-005): recorded scan + legal/protected-data release gate evidence |
| E6 | **Community/IP surface** (R5 deliverables): public issue templates, contributor certification flow (PRD §17.2), IP contribution process per D-07 |
| E7 | **Gate records**: release-quality gate outcomes per [RELEASE_QUALITY_GATES.md](../docs/RELEASE_QUALITY_GATES.md) with thresholds from D-04; release labels restricted to maturity/evidence vocabulary (no reliance language — RGAP-007 stays human-gated) |

### Phase F — Governance closure (parallel, human-paced)

Not a code phase; scheduled so it never becomes the surprise blocker:

- **F1 — Issuance waves (D-11).** Prepare candidate review packets for the 100 `CHECKING` deliverables in waves aligned to phase completions (suggested: PKG-02/03/05/06 solver-side after Phase D evidence; PKG-04/07/13–16 app-side after Phase A/C; PKG-08/09/10/17 after Phase E). Agents prepare packets; only humans change lifecycle state (K-AUTH-1 analog OPS invariants).
- **F2 — Authority records.** Maintainer roster/quorum, contributor legal mechanism, release-label vocabulary (D-07 **RULED** `DEC-027`: sole-maintainer quorum and release authority; contributions closed; D-07b gates any future intake).
- **F3 — Register hygiene.** Keep SOFTWARE_DECOMP, registers, and DAG synchronized as tranches land; any scope evolution (e.g., D-12 deferral of dynamics) gets a recorded SCA/DEC entry, not silent drift.

---

### Hardening lane (cross-phase; select when it blocks or de-risks current-stage work)

This 2026-06-11 amendment absorbs the closed seam plan's §9 deferral list and the post-closure verification's F-5 finding. No separate "revised development plan" will be drafted — this plan remains the single selection surface; references elsewhere to a "revised development plan" resolve here.

| # | Item | Notes |
|---|---|---|
| H1 | **F-5a — canonical-hash unification.** LANDED 2026-06-11 (`TP-H1-HASHUNIFY-001`): wasm exports + parity corpus + frontend swap; allowlist rider measured — `backend_canonicalization` compared and native self-consistency asserted, while `backend_model_hash` value exclusions stood for measured transport reasons (both exclusions retired at H5, which removed the transport divergence). Detail: [completion log](PLAN_COMPLETION_LOG.md); run record at DEL-08-02; SMOKE TP-MAC-114 |
| H2 | **F-5b — DEC-019 evaluation unification.** Relocate the model-document migration evaluation out of the Tauri-only crate into a wasm-compilable crate; export it through the wasm channel; replace `projectService.ts` `evaluateModelDocumentLocal`; cross-engine parity tests | Failure mode today is conservative (over-refusal in preview memory); pair with the next persistence-adjacent tranche. Source: F-5 |
| H3 | **Unit-test backfill and factoring** for `LoadCaseManagerPanel`, `PipeViewport`, `PropertyInspector` | Seam plan §9.4 roll-forward |
| H4 | **Evidence-posture amendments — LANDED 2026-06-11** (`TP-H4-EVIDENCE-POSTURE-001`; human-approved wording applied to `_COORDINATION.md` step 8: Playwright-spec extension as default UI evidence, Vitest floor for new components, template-batch rule). Draft basis + approval pointer: [DRAFT_2026-06-11_H4_coordination_evidence_posture.md](DRAFT_2026-06-11_H4_coordination_evidence_posture.md) | Seam plan §9.2 roll-forward closed |
| H5 | **RFC 8785 number rendering — LANDED 2026-06-11** (`TP-H5-JCSRENDER-001`, human-directed): shared `core/serialization/canonical_json` crate (ECMA numbers via ryu, UTF-16 key sort), engine + headless + labels unified to `rfc8785_jcs`, harness ECMA twins and range constraint deleted, corpora re-blessed, backend hashes byte-equal across all lanes. Residual hand-offs: vocabulary-only `"JCS"` label sites (`primitive_loads`, `result_export` fixtures), Python truth-label wording, optional claimed-hash equality evaluation. Detail: [completion log](PLAN_COMPLETION_LOG.md); run record at DEL-08-02 (TASK sub-record at DEL-10-05) | Closed the `TP-H1-HASHUNIFY-001` measured blockers; DEC-010 JCS alignment done |

Packaged-Tauri smoke stays tracked inside the A5/A8 rows; CI browser provisioning was ruled (`DEC-025`); the seam plan's other §9 items are complete or in flight (plan hygiene done 2026-06-11; A7/B/decision packets dispatched or ruled).

---

## 4. FR-by-FR completion map

Current status is as assessed 2026-06-10; "Closes in" names the phase item that brings the FR to its acceptance criteria.

| FR | Priority | Current status | Closes in |
|---|---|---|---|
| FR-001 create/open/save/version projects | Must | Largely met (A1+A2 landed: editable, versioned, migration-governed model documents per DEC-019); residuals: compatibility-window ruling, explicit migrate operation, file-container semantics | D-09 (file-level semantics); A2 residuals |
| FR-002 unit systems + conversions | Must | Unit metadata everywhere; no conversion engine (`core/units` empty) | B1–B3 (gated D-01) |
| FR-003 3D node/element modeling | Must | Partially met (edit via inspector + viewport selection + explicit form-based node and straight-pipe creation landed; canvas click node drafting landed; viewport node endpoint picking for straight-pipe connectivity landed); broader canvas gestures and element/component authoring pending | A3 |
| FR-004 six DOF per node | Must | Met (frame kernel) | — |
| FR-005 pipe section properties | Must | Met for straight pipe (straight-pipe adapter + stress recovery) | — (re-verify in A5 QA) |
| FR-006 user-defined materials | Must | Data model + fixtures; no GUI authoring | A3/C3 |
| FR-007 basic load cases | Must | Engine met (loads + algebra); GUI manager partially met (primitive-load list, magnitude edits, load-case status/kind edits, empty load-case shell creation, concentrated node-force primitive creation, distributed element-force primitive creation, concentrated nodal moment primitive creation, pressure and thermal primitive creation, support-target imposed-displacement primitive creation, existing combination term-factor edits, combination basis editing, explicit combination term creation, explicit combination term deletion, and explicit mechanics combination creation landed); subtraction/range expression authoring pending | A4 |
| FR-008 linear static solve | Must | Met (dense; benchmarks green) | — (thresholds: D-04) |
| FR-009 element force/moment recovery | Must | Met (linear elements) | — |
| FR-010 fundamental stress recovery | Must | Met (axial/bending/torsion/pressure) | — |
| FR-011 rule-pack schema | Must | Met at schema/engine level; grammar not frozen | C1 |
| FR-012 block incomplete code checks | Must | Met (engine `RULE_INPUTS_INCOMPLETE` + panel surfacing) | C4 end-to-end confirmation |
| FR-013 graphical 3D modeler | Must | Partially met (viewport selection → tree/inspector landed; property-inspector inline validation landed; explicit node and straight-pipe creation via structured operations landed; canvas click-to-node-draft landed; viewport node endpoint picking for straight-pipe connectivity landed; local session undo/redo landed); broader canvas creation/edit gestures and validation for future editors pending | A3 |
| FR-014 model tree + property editor | Must | Partially met (entity-bound edits via A1 seam for supported fields; viewport selection binds rendered entities to the inspector; draft inspector edits now have validate-only inline feedback); full editor UX pending | A3 |
| FR-015 results visualization | Must | Tables from preview runs; no deformed shape | A6 |
| FR-016 calculation reports | Must | Deterministic envelopes; no rendered document | A7 (format: D-10) |
| FR-017 bend objects | Should | Schema only | D1 |
| FR-018 branch objects | Should | Schema only | D2 |
| FR-019 valves/flanges/reducers | Should | Schema only (rigid elements exist) | D3 |
| FR-020 expansion joints | Should | Schema only | D4 |
| FR-021 nonlinear restraints | Should | Active-set classifier only; no global iteration | D6 |
| FR-022 private libraries | Should | Contracts + private-path policy; no management GUI | C3 |
| FR-023 import/export open formats | Could | Largely implemented (PKG-17: native JSON, MBF, PCF, neutral CSV/JSON, GLB; export panels) | Residual GUI round-trip wiring in A/E; disposition at D-12 if residue remains |
| FR-024 dynamic analysis modules | Could | Not implemented | D-12 disposition (implement post-beta or record deferral) |
| FR-025 local FEA export | Could | Not implemented | D-12 disposition |

---

## 5. Sequencing, parallelism, and rough scale

Dependency spine: **A1→A2→A5** is the single most load-bearing chain (everything user-facing hangs off a mutable persisted model). B is independent until B2 touches the same input fields A3 builds — coordinate those two tranches' write scopes. C2/C3 need A3's authoring shell; C1 can run immediately. D1–D5 crate-side work can start any time; D6 needs A5 only for end-to-end demonstration, not for crate development. E starts after D-05/D-06 decisions; E1/E2 can begin once Phase D validation cases exist in draft.

Rough relative scale (tranche-count order of magnitude, not calendar estimates): Phase A ~8–14 bounded tranches (largest); B ~3–5; C ~4–6; D ~8–12; E ~6–10; F is human-paced. No calendar dates are proposed — the project runs on bounded, evidenced tranches, and inventing dates here would be false precision (`ASSUMPTION` if ever needed: sizing above assumes tranche granularity similar to the May–June 2026 app tranches). Observed granularity through the 2026-06-10/11 tranches ran ~2× finer than this sizing assumed (seam plan §9.3); read the phase ranges as skewing toward their high ends.

The original first-three-tranches sequence (D-01/D-08 decision prep → A1 → A2) **completed 2026-06-10** — see the [completion log](PLAN_COMPLETION_LOG.md). Selection now follows the §3 phase tables under the coordination loop's spine-first order.

Hardening candidates are enumerated in the §3 hardening lane (H1–H4); H1 (F-5a hash unification) is sequenced ahead of the A10 authoring scale-up.

---

## 6. Invariant guardrails carried through every phase

- **Status vocabulary:** software emits only `MODEL_INCOMPLETE` / `MECHANICS_SOLVED` / `RULE_INPUTS_INCOMPLETE` / `USER_RULE_CHECKED` / `USER_RULE_FAILED` / `HUMAN_REVIEW_REQUIRED`; `HUMAN_APPROVED_FOR_PROJECT` stays external and hash-bound. Phases A6/A7/C4/D8 must not introduce compliance language.
- **No silent defaults:** Phase B interim SI-only labeling, Phase D user-entered factors, and report rendering all keep this visible.
- **IP boundary:** no populated code tables, no protected constants; C3 import paths and E5 scans are the enforcement points.
- **Provenance:** every new authoring surface (A3, A4, C2, C3, D1–D5) captures source/provenance fields already present in the schemas.
- **Human authority:** lifecycle changes, thresholds, release claims, and issuance happen only at the D-xx decision gates and Phase F waves.

## 7. Top risks to this plan

| Risk | Mitigation in plan |
|---|---|
| D-01 (units) stalls and silently re-blocks R2 value | Lead with the decision-prep tranche; Phase A proceeds SI-labeled, so the stall is visible, not hidden |
| Edit-loop (A1–A3) underestimated — it is the product hinge | Smallest-possible tranche slicing; Playwright (A8) lands early against each increment, not at phase end |
| Nonlinear iteration (D6) convergence quality | Validation-first: D8 cases drafted alongside D6; existing diagnostics taxonomy reused, per PRD §24 mitigation row |
| GUI test debt compounds (13 tests today) | A8 makes the R2 journey itself the regression spine before C/D add surface area |
| Release machinery decisions (D-05..07) deferred until they block R5 | Decision register makes them explicit mid-plan items with RGAP traceability, not end-loaded surprises |
| Scope drift via app tranches crossing deliverable boundaries | Coordination loop rule retained: tranches name the app-owned slice and allowed write targets; register hygiene in F3 |
