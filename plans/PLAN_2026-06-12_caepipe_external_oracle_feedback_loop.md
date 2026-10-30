# OpenPipeStress - CAEPIPE External-Oracle Feedback Loop Plan

**Date:** 2026-06-12 (r2; r1 draft revised 2026-06-11 against the DEC-024/DEC-025/DEC-026 rulings, privacy-boundary review, and house plan conventions)

**Epistemic status:** PROPOSAL (non-governing). This plan proposes a governed CAEPIPE feedback loop as external numerical verification evidence for OpenPipeStress solver/model behavior. It does not change lifecycle state, accept a scope change, create a release claim, create a code-compliance claim, create professional approval, or create validation-for-reliance.

**Consumption note:** This plan is the content input for the SCA-005 scope-change workflow. The scope-change workflow owns all amendment record mechanics; this plan supplies only the proposed scope, deliverable, policy, and sequencing content.

**Relationship to current authority:**

- Existing decomposition revision `0.7` and `SCA-004` already admit CAEPIPE MBF export, optional external-run evidence, stress-neutral result packages, target mapping, loss reporting, and external-prover metadata (SOW-030, SOW-046, SOW-074, SOW-075; PKG-17).
- `DEC-016` defers comprehensive commercial stress-software result ingestion unless a later SCA explicitly reprioritizes it. This plan proposes that targeted amendment, `SCA-005 External-Oracle Verification Loop`. The proposed scope is narrower than comprehensive ingestion: manual, private, user-owned CAEPIPE verification loops for numerical/software improvement only.
- `DEC-024` (as revised by `DEC-026`, both 2026-06-11) now governs numerical tolerance and coverage policy: tolerances are organized by reference-result class with governed relative+absolute pairs per quantity kind, fixture-local overrides may only tighten, and any loosening is a governance event. Coverage is blocking surface-inventory gates plus non-blocking numeric telemetry (D-04b follow-up). Section 5 of this plan binds the oracle loop to that regime instead of restating an open tolerance question.
- `DEC-025` codifies the five-surface local evidence sweep as the commit-bound merge gate. All public tests added by this plan join the existing sweep surfaces; no new evidence entrypoint is created.
- `PLAN_2026-06-10_prd_completion.md` remains the active completion plan. Section 9 of this plan states sequencing against it; this plan does not preempt completion-plan tranche capacity.

---

## 1. Summary

Implement a governed CAEPIPE feedback loop as **external numerical verification evidence** for OpenPipeStress solver/model behavior.

OpenPipeStress runs on macOS and emits a deterministic CAEPIPE MBF handoff package. The user manually runs/imports that package in CAEPIPE on Windows. Private CAEPIPE result artifacts are returned to OpenPipeStress for parsing, unit-normalized comparison, discrepancy triage, and agent-assisted repair proposals. Every artifact derived from CAEPIPE output is private by default, at every processing layer.

Use existing packages rather than creating a new package:

| Package | Role in this plan |
|---|---|
| `PKG-17` | CAEPIPE MBF export, manual external-run package, parsing, private artifact boundaries. |
| `PKG-14` | Result-state comparison, mappings, tolerance profiles, and diff records. |
| `PKG-09` | External-oracle verification corpus and evidence policy. |
| `PKG-16` | Agent discrepancy triage and structured proposal controls. |
| `PKG-15` | External-prover metadata and professional-boundary references. |
| `PKG-12` | Redaction/export controls for external-tool-derived artifacts (default-deny export of private layers). |

No `PKG-18` is recommended. The loop is a workflow that stitches existing functional domains together; it is not a new functional domain of the kind that justified `PKG-17` under `SCA-004`.

## 2. Governance And Documentation Updates

Recommended scope-change amendment:

- **SCA-005 External-Oracle Verification Loop**: admits targeted private/manual CAEPIPE verification without creating comprehensive commercial-tool ingestion, automatic prover status, professional acceptance, code-compliance status, license-bypass behavior, or public-result-ingestion or public-result-publication claims.

Recommended new scope item:

- `SOW-077`: The product shall support private, user-owned external-oracle verification loops against CAEPIPE using deterministic MBF handoff packages, manual Windows execution, parsed private results, unit-normalized comparison, discrepancy classification, and agent-assisted repair proposals, with all external-tool-derived artifacts private by default, and without creating professional-reliance, code-compliance, license-bypass, public-result-ingestion, or public-result-publication claims.

Recommended new objective:

- `OBJ-019`: Use private external-oracle evidence to improve OpenPipeStress solver/model correctness while preserving human professional validation responsibility.

### 2.1 Open-issue wiring

SCA-005 progresses, and must cite, the existing open issues rather than duplicating them:

- `OI-014` (comparison tolerance defaults and mapping workflows): progressed by Section 5's binding of oracle tolerances to the DEC-026 class regime and the D-14 gate.
- `OI-015` (handoff target list and CAEPIPE-first sequencing): progressed by the manual-loop narrowing and the Tier 0/1 scenario corpus.
- `OI-017` (CAEPIPE developer-team clarifications and source evidence): progressed by the DEL-17-01 amendment in Section 3; the new dossier questions below are folded into the existing OI-017 dossier, not a parallel list.

SCA-005 appends new open-issue rows (IDs assigned by the scope-change workflow) for the following content, so these TBDs live in the decomposition register rather than plan-locally:

1. Supported CAEPIPE version/profile assumption for the manual loop, and how version drift is recorded per case.
2. Tier 0 MBF field coverage that can be source-confirmed from lawful public/official documentation (no reverse engineering).
3. Result-return format in manual mode: which CSV/result exports a normal interactive CAEPIPE session can produce, and which sections are suitable for comparison.
4. CAEPIPE license/EULA posture on (a) manual verification use of this kind and (b) publication of comparison results, statistics, or derived agreement claims. Until answered with documented basis and a human ruling, the publication stance in Section 4.3 is deny-by-default.

### 2.2 Documents to update after SCA-005 acceptance

- `docs/VALIDATION_STRATEGY.md`: add a distinct evidence surface named **external-oracle numerical verification**. Keep it separate from hand-calc mechanics verification, workflow validation, user-rule checks, release quality, and professional reliance. State explicitly: CAEPIPE is *an* oracle, not ground truth; machine-readable hand-calc witnesses remain the primary verification anchor; oracle agreement never substitutes for analytic benchmarks; and oracle tolerances are a distinct reference-result class under the DEC-026 regime (Section 5).
- `docs/PROFESSIONAL_BOUNDARY.md`: state that CAEPIPE agreement is verification evidence for software behavior, while competent human professional review (engineer of record or other competent professional reviewer) is still required to validate that agreement arises from correct model parameters and engineering assumptions. Use the existing §6 status vocabulary; no new status concepts.
- `docs/IP_AND_DATA_BOUNDARY.md`: name commercial-tool-derived result artifacts (raw, parsed, compared, and triaged layers alike) as a private data class under §6, covered by the §5 quarantine rule if any such artifact is found in tracked content.
- `docs/PRD.md`: admit a private/manual CAEPIPE oracle loop as a targeted post-MVP or engineering-beta verification workflow, while preserving the ban on automatic prover status, professional acceptance, and broad commercial-tool ingestion.
- `docs/security/threat_model.md`: cover the manual Windows loop, private-result handling at every layer (including comparison and agent layers), no bundled CAEPIPE, no license bypass, no silent executable discovery, and no public real CAEPIPE outputs or derived values.
- `docs/architecture/plugin_boundary.md`: clarify that the v1 loop is manual exchange, not adapter auto-execution or executable discovery.
- `docs/developer_guide/index.md`: describe the oracle case workspace root, case package structure, per-layer privacy classification, and comparison/triage workflow.
- `docs/user_guide/index.md`: describe user-owned Windows execution and the distinction between numerical verification and human professional validation.
- `docs/validation_manual/index.md`: add sections for external-oracle verification evidence and the separate, subsequent human professional validation review.
- `docs/_Registers/ScopeLedger.csv`, `docs/_Registers/Deliverables.csv`, and `docs/_Registers/ContextBudgetQA.csv`: add `SOW-077`, `OBJ-019`, and the new deliverable rows only after SCA acceptance.

## 3. Deliverable Mapping

Amend existing deliverables:

| Deliverable | Required amendment |
|---|---|
| `DEL-17-01` | Extend the existing OI-017 question dossier with CAEPIPE oracle questions: supported version, minimal MBF smoke profile, stable ID strategy, CSV/result sections producible from a normal interactive session and suitable for comparison, Mac-to-Windows manual workflow assumptions, and the license/EULA posture on manual verification use and on publication of comparison results or statistics. |
| `DEL-17-04` | Mature the MBF writer from smoke foundation to source-confirmed case-package emitter for the first verified subset. Field coverage claims remain gated on lawful public/official source evidence per the existing deliverable boundary. |
| `DEL-17-05` | Narrow first implementation to the **manual Windows loop**. It writes run instructions and imports user-returned private results; it does not invoke CAEPIPE. This is a major version bump of `schemas/caepipe_external_run.schema.json` (Section 4.4), not an additive tweak. |
| `DEL-17-06` | Provide OpenPipeStress stress-neutral result exports used as the internal comparison side. |
| `DEL-14-04` / `DEL-14-05` | Compare OpenPipeStress and CAEPIPE result states using stable mappings and explicit tolerance profiles bound to a named comparison context and to the DEC-026 reference-result class regime; no global hidden defaults; overrides tighten-only per DEC-026. |
| `DEL-15-04` | Record external-oracle metadata as flexible references only, not approval status. |
| `DEL-09-04` | Add validation-manual sections for external-oracle verification and the separate human professional validation review. |
| `DEL-16-04` | Ensure agent findings cannot become accepted engineering changes without structured operation review and user acceptance. |
| `DEL-12-02` | Extend redaction/export controls to recognize oracle case-package layers and default-deny export or sharing of external-tool-derived values (raw, parsed, comparison, and triage layers). Any share path must pass these controls; redaction cannot make CAEPIPE-derived values public. |

Add two new deliverables under existing packages:

| New deliverable | Package | Purpose |
|---|---|---|
| `DEL-09-06` - CAEPIPE external-oracle verification corpus and evidence policy | `PKG-09` | Defines scenario tiers, the oracle case workspace root, evidence bundle structure with per-layer privacy classification, private artifact retention, the deny-by-default publication stance, the single-oracle stance (CAEPIPE is an oracle, not ground truth), the D-14 tolerance/gating dependency, the discrepancy classification axes, and evidence labels. |
| `DEL-16-05` - Agent discrepancy triage and repair proposal workflow | `PKG-16` | Converts comparison outcomes into classified findings (outcome, cause hypothesis, disposition) and proposed structured model operations, assumptions, bug reports, dossier questions, or professional-review questions. |

## 4. Implementation Shape

### 4.1 Storage root

Oracle case packages are working data, not tracked repository content. Default root:

- `validation/oracle_cases/` — excluded wholesale by the root `.gitignore` (same pattern as the existing `**/_Sources/` exclusion). Developer-guide documentation describes the layout; no tracked README lives inside the ignored subtree.
- Users may choose an external folder instead; the case manifest records the case root either way.
- Defense in depth: the case-package generator writes a `.gitignore` containing `*` at the workspace root, so the exclusion travels with the folder if it is relocated into another git repository.
- The private-artifact guard test (Section 6) asserts the ignore rule holds (`git check-ignore` on representative paths) and that no tracked file exists under the root.

### 4.2 Case package layout and privacy classification

Manual case package generated on macOS. **Every layer below the outbound `caepipe_in/` package is private**: comparison diffs and triage reports embed CAEPIPE-derived numerical values and therefore carry the same privacy class as the raw results. Privacy classification is recorded per layer in the case manifest, not inferred from folder names.

```text
validation/oracle_cases/                  # untracked private workspace (root .gitignore)
  oracle_case_<case_id>/
    oracle_case_manifest.json             # schema-validated; refs + hashes + per-layer privacy labels; no CAEPIPE result values
    openpipestress_model.json             # internal-derived; private by workspace default
    openpipestress_run.json               # internal-derived; private by workspace default
    caepipe_in/                           # outbound package; contains only OpenPipeStress-derived data
      model.mbf
      export_manifest.json
      stable_id_map.json
      loss_report.json
      WINDOWS_RUN_INSTRUCTIONS.md
    caepipe_out/                          # PRIVATE: raw user-returned CAEPIPE artifacts
      raw_user_returned_results/
    parsed/                               # PRIVATE: normalized CAEPIPE values
      caepipe_results.normalized.json
    comparison/                           # PRIVATE: diffs embed CAEPIPE-derived values
      oracle_diff.json
      discrepancy_classification.json
    agent/                                # PRIVATE: triage quotes comparison values
      triage_report.md
      proposed_operations.json
    human_review/                         # PRIVATE: informal working notes for competent human review
      validation_notes.private.md
```

`human_review/validation_notes.private.md` is an informal working note, **not** a human acceptance record. If a human acceptance record is ever stored or referenced, it must follow `docs/PROFESSIONAL_BOUNDARY.md` §7 (hash binding, scope, staleness rules) and remains outside this loop's artifacts.

### 4.3 Public repository and publication policy

- Real CAEPIPE outputs stay private and untracked — and so do all artifacts derived from them: parsed normalizations, comparison diffs, discrepancy classifications, and triage reports. Sharing a diff that embeds CAEPIPE values republishes CAEPIPE output; the privacy boundary therefore covers the full derivation chain.
- **Publication stance is deny-by-default**: no public claim, statistic, aggregate, or example derived from private CAEPIPE results — including claims of the form "verified against CAEPIPE on N cases" — until (a) the DEL-17-01 dossier answers the EULA/benchmark-publication question with documented basis, and (b) a human ruling explicitly permits a named claim form. Until then, oracle evidence informs development privately and is referenced in governed records only as private-evidence pointers without embedded values.
- Public tests may use synthetic parser-contract fixtures only if clearly labeled as not CAEPIPE-produced output (existing `fixtures/*/invented/` convention).
- No CAEPIPE binaries, license files, proprietary examples, standards tables, project data, owner criteria, or commercial examples enter the public repository.
- If any real CAEPIPE artifact is found in tracked content, the `docs/IP_AND_DATA_BOUNDARY.md` §5 quarantine rule applies (stop, quarantine, record, human review).

### 4.4 Core interfaces to add or extend

These are versioned contract changes, not additive tweaks; tranche sizing must treat the two existing-schema changes as breaking major bumps with fixture and test migration:

- **Add** `schemas/oracle_verification_case.schema.json` (new, additive): case manifest, source hashes, MBF package refs, per-layer privacy classification, private artifact refs (pointers, never embedded values), comparison refs, professional-boundary labels, and the publication-stance flag.
- **Major bump** `schemas/caepipe_external_run.schema.json`: the current schema requires `executable_config`, `command_profile`, and `execution_result` unconditionally and has no invocation-mode concept. Add an `invocation_mode` discriminator with `manual_windows_exchange` as the first mode (no executable path required) alongside the retained executable-harness mode; make the executable-related members conditionally required by mode; extend `package_status` for manual-exchange states (e.g., pending-return, results-returned). Migrate the existing invented fixtures and parser-contract tests.
- **Major bump** `schemas/comparison_tolerance.schema.json` (and `schemas/comparison_mapping.schema.json` where touched): introduce a named comparison-context concept (internal state compare, internal run compare, `external_oracle_caepipe`), bind tolerance profiles to a DEC-026 reference-result class, and extend the objectives enum to admit `OBJ-019`. Tolerances remain explicit and case/profile-bound; the tighten-only override rider from DEC-026 applies (Section 5).
- Where persisted documents are affected, follow the DEC-019 migration discipline (per-document semver, explicit transform chain, no silent destructive rewrites).

### 4.5 Discrepancy classification

Classification is recorded on three axes so that outcomes, causes, and routing stay distinguishable. No silent defaults on any axis.

Outcome (computed per mapped quantity):

- `matched_within_tolerance`
- `delta_exceeds_tolerance`
- `row_unmatched_internal_only`
- `row_unmatched_external_only`
- `tolerance_tbd` (per-kind governed value not yet filled; surfaces the DEC-026 `tolerance_policy_tbd_diagnostic` posture rather than guessing)

Cause hypothesis (for non-matching outcomes; labeled `ASSUMPTION` until evidenced):

- `solver_defect_candidate`
- `export_mapping_issue`
- `parser_mapping_issue`
- `unit_or_axis_mismatch`
- `support_semantics_mismatch`
- `load_case_mismatch`
- `missing_model_parameter`
- `documented_behavior_difference` (legitimate, documented difference in tool convention or method; neither side defective)
- `caepipe_profile_unknown`

Disposition (routing; never an acceptance):

- `proposed_operation` (structured model operation via PKG-16)
- `proposed_code_fix` (export/parser/solver change proposal)
- `bug_report`
- `dossier_question` (feeds the DEL-17-01 / OI-017 dossier)
- `assumption_recorded`
- `requires_professional_review`
- `no_action`

### 4.6 Agent behavior

- The agent may classify outcomes, propose cause hypotheses, and propose fixes.
- The agent must not tune parameters merely to match CAEPIPE. `documented_behavior_difference` exists precisely so persistent legitimate differences do not pressure either false `solver_defect_candidate` findings or tolerance loosening.
- Proposed model-parameter completions are `ASSUMPTION` unless sourced from user data or competent human professional review.
- Any model change must flow through PKG-16 structured operations, validation, diff preview, audit receipt, and user acceptance. Agent findings never become accepted engineering changes by themselves.

## 5. Tolerance And Evidence Policy Under DEC-024/DEC-026

This section replaces the r1 draft's open "D-04 or successor" dependency. D-04 was ruled on 2026-06-11 (`DEC-024`, Part 1 revised same day by `DEC-026`; Part 2 coverage posture unchanged).

- **External-oracle comparisons are a new reference-result class.** The DEC-026 regime organizes tolerances by reference-result class (analytic benchmark; cross-engine-exact; regression-golden-exact). Cross-solver agreement with CAEPIPE is none of these; SCA-005 adds an `external_oracle` class. The analytic-class seed (1.0e-9 relative) and the exact-class tiers must never be applied to oracle diffs — they are unattainable and meaningless across independent solvers.
- **Class values are governed, measured, and start TBD.** Per-quantity-kind relative+absolute pairs for the `external_oracle` class (the absolute member is the explicit near-zero floor) begin as `TBD`, are seeded from measured Tier 0/1 private runs, and become governed values only through the D-14 ruling (Section 7). Until then, each case carries an explicit, case-bound tolerance profile labeled `ASSUMPTION`, usable for triage only.
- **Tighten-only discipline extends to the oracle class.** Case-local overrides may only tighten the governed class values; any loosening is a governance event recorded in the governed tolerance record, never a case-local edit. This carries the DEC-026 rider into the context where override-scatter pressure is highest.
- **No pass/fail release-quality claims before D-14.** Consistent with `docs/VALIDATION_STRATEGY.md` §4, oracle evidence is input to a future release-quality review only. Coverage of oracle scenarios follows the DEC-024 Part 2 posture: a blocking surface-inventory gate applies only to the synthetic public contract surfaces; private oracle-run coverage is recorded as non-blocking telemetry; numeric floors arrive only by later human promotion (D-04b/D-14).
- **Evidence integration follows DEC-025.** All public tests added by this plan join the existing five-surface local evidence sweep (pytest surface for handoff/comparison contract tests; cargo/Vitest surfaces only where those layers are actually touched). No new evidence entrypoint. Tests that read private local artifacts skip cleanly and visibly when none exist, mirroring the existing skip-without-executable pattern.
- **Single-oracle stance.** CAEPIPE is an oracle, not ground truth. Machine-readable hand-calc witnesses (`docs/VALIDATION_STRATEGY.md` §2) remain the primary verification anchor. Oracle agreement never substitutes for analytic benchmarks; oracle disagreement never automatically means OpenPipeStress is wrong — `documented_behavior_difference` is a legitimate terminal classification.

## 6. Scenario And Test Plan

Scenario tiers for `DEL-09-06`:

| Tier | Scenario family |
|---|---|
| Tier 0 | Straight pipe, anchors, cantilever, self-weight, simple thermal expansion. |
| Tier 1 | Imposed displacement, distributed loads, guide/restraint combinations, basic reactions. |
| Tier 2 | Element forces/moments and fundamental stress recovery comparisons. |
| Tier 3 | Bends, branches, rigid/semi-rigid components, expansion joints, hangers after those features mature (completion-plan Phase D component depth). |
| Tier 4 | Nonlinear supports only after OpenPipeStress nonlinear convergence is implemented (completion-plan Phase D). |

Required tests:

- Schema validation for oracle case manifests, manual-run records, normalized CAEPIPE results, and comparison records.
- Schema-migration tests for the two major bumps in Section 4.4: existing invented fixtures migrate, prior-version documents are handled per the DEC-019 discipline, and no fixture silently changes meaning.
- MBF package determinism: same model produces byte-stable MBF, manifest, ID map, and loss report.
- Private artifact guard: `validation/oracle_cases/` is ignored (`git check-ignore` on representative paths), no tracked file exists under the root, and the generator emits the workspace-local `.gitignore`.
- Parser import tests for private local files (skip cleanly when absent), plus synthetic public parser-contract tests labeled per the `invented/` convention.
- Comparison tests for matched rows, unmatched rows (both directions), unit mismatch, axis/sign mismatch, missing mappings, `tolerance_tbd` surfacing, and explicit tolerance classification with tighten-only override enforcement.
- Agent triage tests proving findings become proposed operations, fixes, reports, or review questions — never accepted changes — and that all three classification axes are populated without silent defaults.
- Boundary tests scanning docs, UI, and report text for prohibited certification, compliance, approval, sealing, or professional-validation claims, and for publication-stance violations (no public claim derived from private oracle results).
- All public tests run green inside the existing DEC-025 five-surface sweep.

Acceptance requires:

- Manual Windows loop demonstrated on at least one private CAEPIPE run.
- OpenPipeStress-vs-CAEPIPE comparison produces stable diffs and diagnostics.
- Discrepancies are classifiable on all three axes without silent defaults.
- Every tolerance value in use is labeled: governed value (post-D-14) or `ASSUMPTION` (pre-D-14); no unlabeled tolerances.
- Schema major bumps landed with migrations and updated fixtures; five-surface sweep green.
- Competent human professional review remains a separate private/human step; no acceptance record is generated by the loop.
- No real CAEPIPE result artifact, nor any derived value (parsed, diffed, or triaged), is committed or published.

## 7. Human Gates

| Gate | Content | Timing |
|---|---|---|
| **G1 — SCA-005 acceptance** | The scope amendment in Sections 2–3. Prepared and recorded by the scope-change workflow; only the human project authority accepts. | Before any register, docs, or implementation change. |
| **G2 — D-14 ruling** | New decision-register row (next free ID; append per the register's convention for newly discovered human-gated TBDs): governed `external_oracle` reference-result class values (per-quantity-kind relative+absolute pairs seeded from measured runs), any oracle-based pass/fail gating posture, and whether any named public claim form derived from oracle evidence is permitted (jointly with the DEL-17-01 EULA dossier answer). | Packet prepared after first measured Tier 0/1 private runs (T6); ruling required before any pass/fail or publication use. |

Until G2, oracle tolerances are case-bound `ASSUMPTION` profiles for triage only, and the publication stance is deny-by-default (Section 4.3).

## 8. Tranche Plan

Bounded tranches in house convention; each lands only with its tests green in the five-surface sweep.

### T1 — `TP-ORACLE-SCA-001`: SCA-005 preparation and acceptance
Scope-change workflow consumes Sections 2–3 of this plan. Exit: human acceptance recorded; registers and Section 2.2 doc updates unblocked.

### T2 — `TP-ORACLE-SCHEMA-001`: contracts and guards
New `oracle_verification_case` schema; the two major bumps with fixture/test migration (Section 4.4); workspace root + ignore rule + private-artifact guard tests. Exit: all schema and guard tests green; no behavior change to existing exporters.

### T3 — `TP-ORACLE-MBF-001`: Tier 0 outbound leg
DEL-17-04 maturation to the source-confirmed Tier 0 subset; case-package generator; `WINDOWS_RUN_INSTRUCTIONS.md` emitter (DEL-17-05 manual mode, write side). Exit: deterministic Tier 0 case package generated end-to-end on macOS; byte-stability test green. Stop rule: if Tier 0 MBF fields cannot be source-confirmed lawfully, pause this tranche and route the gap to the OI-017 dossier (no reverse engineering).

### T4 — `TP-ORACLE-RETURN-001`: return leg and comparison
User-returned result import and normalization (DEL-17-05 read side); `external_oracle_caepipe` comparison context through DEL-14-04/05; three-axis classification (Section 4.5). Exit: synthetic-fixture comparison and classification tests green; private-file paths skip cleanly.

### T5 — `TP-ORACLE-TRIAGE-001`: agent triage and controls
DEL-16-05 triage workflow; PKG-16 routing of dispositions; DEL-12-02 default-deny export controls for oracle layers; boundary-scan tests. Exit: triage produces proposals/questions only; export of private layers denied by default; boundary tests green.

### T6 — `TP-ORACLE-EVIDENCE-001`: first real loop and D-14 packet
First private Tier 0 CAEPIPE run (user-executed on Windows); measured tolerance baselines per quantity kind; D-14 decision packet prepared from measurements; Section 2.2 docs propagation completed. Exit: acceptance criteria of Section 6 met except items gated on the D-14 ruling itself.

## 9. Sequencing Against The Completion Plan

- T1 (governance) and T2 (contracts/guards) may proceed now in the parallel governance lane, alongside completion-plan Phase F-style work; neither touches the product spine.
- T3–T6 (implementation) are sequenced **after the completion-plan Phase A exit test (A8) lands**: the oracle loop consumes the same model → run → result → stress-neutral export spine that A8 proves end-to-end, and bounded-tranche capacity is currently committed to the Phase A/C work that DEC-022..DEC-025 just unblocked. Oracle tranches must not preempt that capacity.
- Tier 2 scenarios follow stress-recovery maturity; Tier 3 follows Phase D component depth; Tier 4 follows Phase D nonlinear convergence. The corpus grows with the product; the loop does not pull feature work forward.

## 10. Risks And Stop Rules

| Risk | Containment |
|---|---|
| Real CAEPIPE artifact (or derived values) lands in tracked content | Stop; apply `docs/IP_AND_DATA_BOUNDARY.md` §5 quarantine; human review before resuming. Guard tests in T2 make this a tested invariant, not a habit. |
| EULA/dossier review finds manual verification use or publication restricted | Stop the affected use; route to human ruling (G2 scope); publication remains deny-by-default regardless until ruled. |
| Overfitting OpenPipeStress to CAEPIPE conventions | No tuning-to-match rule (Section 4.6); `documented_behavior_difference` classification; hand-calc witnesses remain primary anchor (Section 5). |
| Tolerance scatter / silent loosening in the oracle context | DEC-026 tighten-only rider extended to the `external_oracle` class; loosening only as a recorded governance event (Section 5). |
| Schema major bumps destabilize existing PKG-17 fixtures/tests | Migration tests in T2 before any writer/parser change; five-surface sweep green required at every tranche exit. |
| Capacity contention with the completion plan | Section 9 sequencing: implementation tranches gated on the Phase A exit test; governance lane only until then. |
| Agent findings drift toward acceptance-like language | DEL-16-04 controls; boundary-scan tests in T5; PROFESSIONAL_BOUNDARY.md §5/§6 vocabulary enforced in docs updates. |

## 11. Assumptions And Defaults

- Scope carrier: amend existing packages through `SCA-005`; do not create `PKG-18`.
- First runner: manual Mac-to-Windows loop; no automated CAEPIPE invocation in v1; the executable-harness mode remains schema-supported but unimplemented for this loop.
- Evidence policy: all CAEPIPE-derived artifacts — raw, parsed, compared, triaged — are private only; publication deny-by-default until the G2 ruling.
- Storage default: `validation/oracle_cases/` untracked workspace (Section 4.1); user-relocatable with traveling ignore rule.
- MBF is the primary CAEPIPE handoff target; PCF remains secondary interoperability and not the verification backbone.
- Tolerances: `external_oracle` is a new DEC-026 reference-result class; case profiles are `ASSUMPTION`-labeled until the D-14 ruling seeds governed values from measured runs; tighten-only overrides; no pass/fail release-quality claim before D-14.
- Vocabulary: this plan and all downstream artifacts use the `docs/PROFESSIONAL_BOUNDARY.md` status vocabulary ("competent human review", "human accepted for project", "engineer of record or other competent professional reviewer"); jurisdiction-specific licensure terms are not used.
- The loop verifies numerical/software behavior; only a knowledgeable, competent professional validates engineering correctness for reliance, and nothing in this loop generates or implies such validation.
