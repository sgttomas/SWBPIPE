# C — OpenPipeStress GUI Governance Constraints (extraction only)

**Purpose:** objective reference sheet of accepted project invariants that constrain what the
OpenPipeStress GUI may show, say, or do. Extraction only; no design opinions.

**Repository basis:** `{REPO_ROOT}`,
branch `claude/chirality-piping-ui-design-a31fd2`, HEAD `28ad73cc3`.
All source paths below are relative to `projects/chirality-piping/` unless marked `[repo root]`.

**Authority note:** `docs/CONTRACT.md` and `docs/PROFESSIONAL_BOUNDARY.md` carry
`status: draft` in their front matter; `docs/claims_registry.md` is ruled vocabulary under
`DEC-081`. `docs/PRD.md` is the adopted PRD v0.3 (amended under `DEC-080`/SCA-007).
Current decomposition revision is **0.12** (SCA-009 accepted, `execution/_ScopeChange/_LATEST.md`).

---

## 1. Constraints table

### 1.1 Claims and professional boundary

| ID | Source | Rule (key phrase quoted) | UI implication |
|---|---|---|---|
| C-01 | `docs/CONTRACT.md` §1, `OPS-K-AUTH-1` | "Software and agents must not claim to certify, seal, approve, authenticate, or declare engineering code compliance for reliance." | No screen, label, badge, status chip, or generated text may assert certification, sealing, approval, authentication, or code compliance. |
| C-02 | `docs/CONTRACT.md` §1, `OPS-K-MECH-2` | "The solver computes mechanics; rule packs evaluate user-defined acceptability; professional compliance remains human judgment." | Results, rule-check, and acceptance must be visually and semantically three separate things; a screen must not merge them into one verdict. |
| C-03 | `docs/PRD.md` §4.3 Product Boundary | The software "may say" mechanics solved / internal analytical result produced / user-rule check evaluated / warnings and assumptions recorded / model state saved / analysis run saved / states compared / handoff package generated; it "must not say or imply" professional approval granted, design certified, design sealed, code compliance established for reliance, or external prover validation completed unless user-supplied. | The permitted list is the allowed vocabulary for status text; anything outside it must not appear as a product status. |
| C-04 | `docs/PRD.md` §21.2 Prohibited Claims | The product must not claim or imply that it "certifies / seals / approves / authenticates engineering work", "declares code compliance for professional reliance", "replaces the engineer of record", "replaces project-specific professional review", "provides standards-body approval or official interpretation", or "makes public example data suitable for project use". | Prohibited for all UI strings, tooltips, empty states, marketing copy inside the app, and example/fixture labels. |
| C-05 | `docs/PRD.md` §21.1 Permitted Claims | Permitted: mechanics results from recorded inputs; user-rule checks from user-supplied rules/data; diagnostics, warnings, assumptions, limitations; provenance/version/checksum/hash records; validation and regression evidence for software behavior; report generation for competent human review; local-first private-data handling where implemented. | A screen may make exactly these classes of claim, and must scope each claim to the feature, version, data, and evidence actually present (`docs/PROFESSIONAL_BOUNDARY.md` §4). |
| C-06 | `docs/PRD.md` §21.3; `docs/PROFESSIONAL_BOUNDARY.md` §7 | "Any human acceptance record, if stored in future releases, shall be separate from solver outputs, rule-pack outputs, comparison outputs, and report generation." / "MVP shall not require a formal acceptance workflow." | No accept/approve/sign-off control may be attached to a result, run, comparison, or report surface; MVP must not present an acceptance workflow. |
| C-07 | `docs/PROFESSIONAL_BOUNDARY.md` §3 (Authority Boundaries table) | Per-surface "what it may state / must not state" for Solver, Rule pack, Report, Design-engine workflow, Agent output, Release notes. | Each GUI surface inherits its own row; e.g. a comparison/handoff view may state "review aids" and must not state "external solver validation". |
| C-08 | `docs/PROFESSIONAL_BOUNDARY.md` §5 | "Prohibited language is unacceptable even when convenient for marketing, release notes, generated reports, examples, or **UI labels**." | There is no convenience exception for microcopy, icon tooltips, or short-form labels. |
| C-09 | `docs/PROFESSIONAL_BOUNDARY.md` §6 (Status Vocabulary) | Four preserved distinctions: "Mechanics solved", "User-rule checked", "Human review required", "Human accepted for project"; "Software must not emit automatic professional-approval or automatic code-compliance statuses." | Status displays must keep these four concepts distinct; the fourth is never software-emitted. |
| C-10 | `docs/TYPES.md` §4 | Automatic software-emitted statuses are limited to `MODEL_INCOMPLETE`, `MECHANICS_SOLVED`, `RULE_INPUTS_INCOMPLETE`, `USER_RULE_CHECKED`, `USER_RULE_FAILED`, `HUMAN_REVIEW_REQUIRED`; the software "must not use `HUMAN_APPROVED_FOR_PROJECT`, `CODE_COMPLIANT`, `CERTIFIED`, `SEALED`, or `APPROVED` ... as an automatic status." | Status enums rendered in the UI are drawn from the six-value automatic set only. |
| C-11 | `docs/SPEC.md` §4.3 Code-neutral analysis boundary | Three authority domains: "mechanics solve authority", "user-rule-check authority", "human acceptance authority: external project records bound to reviewed evidence hashes". | Any results screen must attribute each shown status to its authority domain, not to a single global "status". |
| C-12 | `docs/DIRECTIVE.md` §3 principle 6 | "**Human authority.** The software may compute; it does not certify, seal, approve, or authenticate engineering work (PRD §21.2)." | Product-level framing must position the app as computation plus decision support. |
| C-13 | `docs/PRD.md` §5.9 | "Reports, **UI labels**, agent output, examples, and documentation shall preserve the distinction between software-computed results and professional engineering acceptance." | Distinction must be visible at the label level, not only in a footer. |
| C-14 | `docs/claims_registry.md` §1 `BS-ACCEPT` | Canonical: "Results are engineering decision-support information. Acceptance, professional judgment, and any certification, sealing, or code-compliance determination remain with the responsible engineer and project authority." Belongs on "results, rule-check, comparison, solve, report-preview surfaces; any place a computed outcome is shown." | Every surface that shows a computed outcome carries BS-ACCEPT (canonical text or a listed short variant) verbatim. |
| C-15 | `docs/claims_registry.md` §1 `BS-MATURITY` | Canonical: "Technical preview — not a released product." — "One sentence ... Never a compound litany." Belongs on "the app shell banner/footer". | The app shell must carry this exact sentence; it must not be expanded, reworded, or turned into a list. |
| C-16 | `docs/claims_registry.md` §1 `BS-VALID` | Canonical: "Candidate designs are validated in the user's accepted professional tools (external-prover correlation, PRD §22.5). Internal benchmarks and rule checks are development verification and screening evidence." Belongs on "handoff/export UI (MBF, PCF, stress-neutral, native package, external-prover surfaces)". | Export/handoff screens carry BS-VALID; they must not present internal results as validation. |
| C-17 | `docs/claims_registry.md` §1 `BS-IP` | Canonical: "OpenPipeStress ships no protected standards content. All code-specific values, tables, allowables, and factors are supplied by the user or user-controlled private sources, with provenance recorded." Belongs on "import/library/contribution UI, redaction/export surfaces"; **not on** results views unless the surface genuinely handles imported content. | Import/library/redaction screens carry BS-IP; results screens must not carry it decoratively. |
| C-18 | `docs/claims_registry.md` §1 `GF-TOKEN` | "Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081)." — "**Never used on product surfaces.**" | The governance fence token must never appear in the GUI. |
| C-19 | `docs/claims_registry.md` §2 | Evidence-status labels `INTERNALLY_VERIFIED` / `PROVER_CORRELATED` / `ENGINEER_ACCEPTED` (reserved; "no acceptance workflow exists in MVP"); labels "attach to individual results, reports, and case pages instead of global hedging"; `non-authoritative` "never appears on product surfaces." | Evidence standing is shown per result/report/case, not as a blanket disclaimer; `ENGINEER_ACCEPTED` must not be emitted; the word "non-authoritative" is barred from the GUI. |
| C-20 | `docs/claims_registry.md` §1 `BS-ACCEPT` "Retires" clause; `[repo root] tools/validation/validate_claims_language.py` (self-check GEN-13) | Lint finding `AD_HOC_CLAIMS_LITANY`: a scanned line carrying ≥3 distinct litany terms (`certif`, `seal`, `authenticat`, `code compliance`, `professional`, `approval/approve`, `endorse`, `release-readiness`, `production-readiness`) without a registered boundary text, or any "not authoritative" phrasing, is a violation; scan covers all non-test `.ts`/`.tsx` under `apps/desktop/src`. "violations block closeout." | New UI copy must reuse registry texts verbatim; ad-hoc disclaimer stacks in any component file fail the gate. |
| C-21 | `loop/WORKPLAN_2026-07-18b_piping_loop.md` §"Standing constraints — fences", `F-PIP-2` | "no release-readiness, professional approval, certification, sealing, authentication, or code-compliance claims. Git closeout is source-control hygiene, not lifecycle issuance." Authoring note: "the fence text above is the governing definition and is never edited." | The fence is the governing definition behind C-01/C-04; UI surfaces use `docs/claims_registry.md` texts rather than restating it. |
| C-22 | `docs/PRD.md` §16.3; §7 non-goal 6 | "The MVP shall not enforce a formal prover-status vocabulary or lifecycle." Users instead use model/state names, tags, notes, external reference fields, attachments/links, comparison reports. | No approval/status pipeline widget; naming, tagging, and notes are the intended state-labelling affordances. |
| C-23 | `docs/PRD.md` §4.2 / §7 Non-Goals | The product shall not "claim to be an industry-standard professional stress-analysis prover tool", claim standards-body approval, or "automatically determine code compliance for professional reliance"; §7 items 11–19 record absent special domains (buried pipe, jacketed pipe, FRP, slug/two-phase, snubbers, fatigue counting, nozzle-flexibility objects, mitered-bend/stepped-reducer, EJ hardware kinematics) as decided non-goals per `DEC-069`. | Positioning copy and any capability matrix must not imply prover-equivalence; absent domains may be shown as recorded non-goals, not as "coming soon". |
| C-24 | `docs/PRD.md` §22.5 (per `DEC-080`) | External-prover correlation is "the **principal validation posture** — the internal engine generates and screens candidates, the external prover is the canonical validation oracle"; activation is owner-gated; external-prover metadata is "not automatic professional acceptance". | Any validation-flavoured indicator in the GUI must read as screening/handoff evidence, and prover records must read as user-supplied external records. |
| C-25 | `docs/SPEC.md` §3.2 | "Model states do not provide formal prover approval states, certification states, sealing states, or authentication states (PRD §21.2), automatic code-compliance statuses, or professional acceptance records." | The state browser must not offer approval-like state badges. |

### 1.2 Missing data and no silent defaults

| ID | Source | Rule (key phrase quoted) | UI implication |
|---|---|---|---|
| C-26 | `docs/CONTRACT.md` §1, `OPS-K-DATA-2` (enforcement column names "GUI warnings") | "Missing solve-required or rule-check-required values are explicit findings, never silent defaults." | Every required-but-absent value must surface as a named finding on the screen that owns it; no placeholder/default substitution. |
| C-27 | `docs/CONTRACT.md` §1, `OPS-K-DATA-1` | "Code-specific values are user-supplied or lawfully imported private data, not bundled public defaults." | No code-value pickers seeded with bundled defaults; empty is the correct initial state. |
| C-28 | `docs/PRD.md` §5.5; `docs/DIRECTIVE.md` §3 principle 2 | "The product shall not silently insert code-relevant data, component values, SIFs, flexibility factors, allowables, owner requirements, or support assumptions. Missing data must produce visible warnings or blocking errors according to severity." | Form fields for these categories must never prefill; severity determines whether the UI warns or blocks. |
| C-29 | `docs/SPEC.md` §4.3 (closing paragraph) | Missing solve-required and rule-check-required inputs "are explicit findings with diagnostics and provenance. They are not defaulted by the solver, rule-pack evaluator, adapters, reports, **or GUI**." | The GUI is explicitly named as a surface forbidden from defaulting. |
| C-30 | `docs/PRD.md` §17.5 | "It shall not report user-rule pass/fail results unless all rule-required values are present." | Rule-check pass/fail must be suppressed (and replaced by `RULE_INPUTS_INCOMPLETE`) whenever inputs are incomplete. |
| C-31 | `docs/SPEC.md` §4 | "Dimensionless values are not a fallback for missing units. Missing or ambiguous unit metadata on a unit-bearing physical value is a diagnostic". | A unit selector must not default to dimensionless/blank-as-valid. |
| C-32 | `docs/PRD.md` §18.4 Import Warnings | Imported data "shall be flagged" when required fields are missing, units missing/inconsistent, provenance missing, redistribution status unclear, values appear to be protected standards data, or values are outside user-defined reasonableness ranges. | Import screens must render all six flag classes, not a single generic import error. |
| C-33 | `docs/INTENT.md` "Component library intent" | "The software should not silently insert protected standard dimensions or weights. Where required data is missing, the GUI should require user input or documented import." | Component/section forms block on missing dimensional data rather than auto-filling. |
| C-34 | `DEC-018` (`execution/_Decomposition/SOFTWARE_DECOMP.md` §12) | "explicit gauge/absolute pressure quantity kinds with **no silent atmospheric default**". | Pressure inputs must force an explicit gauge/absolute choice. |
| C-35 | `DEC-092` (D-45 ruling, `SOFTWARE_DECOMP.md` §12) | For temperature-indexed properties "the solve blocks rather than extrapolates when a qualifying bracket is absent"; every derived value "carries provenance naming both source points and the declared method". | Material property views must show interpolation provenance and present a block, not an extrapolated number. |

### 1.3 Units

| ID | Source | Rule (key phrase quoted) | UI implication |
|---|---|---|---|
| C-36 | `docs/CONTRACT.md` §1, `OPS-K-UNIT-1` | "All calculations, formulas, imported values, and exports must be unit-aware and dimensionally checked." | Every numeric field, table cell, and export preview must carry unit metadata. |
| C-37 | `docs/PRD.md` §5.6 | "The product shall reject incompatible units, preserve unit metadata, and report unit conversions deterministically." | Unit mismatch is a rejection with a message, never a silent coercion. |
| C-38 | `docs/SPEC.md` §4 | "Every physical value that crosses a schema, application-service, solver, import/export, report, or rule-evaluation boundary must carry explicit unit metadata unless ... explicitly classified as dimensionless, ratio, percentage, or coefficient." Accepted dimension vocabulary is enumerated; retired aliases (`temperature_difference`, `area_moment`, bare `stiffness`) "are not accepted enum values". | Unit/dimension pickers bind to the accepted vocabulary; retired aliases must not be offered. |
| C-39 | `DEC-018` (`SOFTWARE_DECOMP.md` §12, line 609) | "SI-canonical internal unit set with a **dual display catalog**, exact definitional conversion constants recorded as governed values, dual absolute/interval temperature semantics". | Display may offer dual (SI/US) presentation over an SI-canonical store; temperature UI must distinguish absolute from interval. |
| C-40 | SCA-009 `Vocabulary_Annex.md` §1.5 (Units framing) | Project-wide unit switching is "a **display-system toggle** ... entered-units-preserved storage is unchanged, and **no stored value is silently mutated** by a unit-system switch." | A global unit toggle changes presentation only; it is not a model operation and must not rewrite entered values. |
| C-41 | `DEL-07-06-RQ-007` (`.../DEL-07-06.../ScopeOfWork.md` line 164) | "Unit-bearing values, result quantities, and report-facing data shall display units and preserve unit-safety context." | Units remain visible in fields, tables, exports, and report previews — not stripped for density. |
| C-42 | `docs/SPEC.md` §4 (closing) | "Unit checks support mechanics and rule evaluation. They do not certify, seal, approve, or authenticate work, or declare code compliance (PRD §21.2)." | A green unit-check indicator must not read as an engineering approval. |

### 1.4 Provenance and IP boundary

| ID | Source | Rule (key phrase quoted) | UI implication |
|---|---|---|---|
| C-43 | `docs/CONTRACT.md` §1, `OPS-K-IP-1` | The public repository "must not contain protected standards text, tables, figures, examples, copied code formulas, material allowables, SIF/flexibility tables, protected dimensional tables, or proprietary commercial data." | No bundled catalogs, screenshots, sample projects, or demo data may embed protected content. |
| C-44 | `docs/CONTRACT.md` §1, `OPS-K-IP-3` | "Suspected protected content must be quarantined and escalated; agents must not paraphrase protected tables into public data." | Import UI must offer a quarantine/escalation path (`IP_AND_DATA_BOUNDARY` §5: stop ingestion, mark `protected_suspected`, move to quarantine, record issue, request review). |
| C-45 | `docs/CONTRACT.md` §1, `OPS-K-DATA-3` | "Materials, components, SIFs, flexibility factors, allowables, and rule-pack values carry provenance fields." | Editors for these entities must expose provenance fields as first-class, not optional metadata drawers. |
| C-46 | `docs/IP_AND_DATA_BOUNDARY.md` §4 | Required provenance fields: `source_name`, `source_location`, `source_license`, `contributor`, `contributor_certification`, `redistribution_status` (`public_permissive`/`private_only`/`unknown`/`protected_suspected`), `review_status` (`pending`/`accepted`/`rejected`/`quarantined`). | Provenance UI must render all seven fields with these enumerations. |
| C-47 | `docs/PRD.md` §14.3 Property Inspector | The inspector "shall show" entity type, geometry, units, material, section, component properties, loads, support settings, SIF/flexibility inputs, **source/provenance**, validation status, dependent rule-pack inputs, linked assumptions and notes. | This is the minimum inspector content contract; provenance and validation status are mandatory rows. |
| C-48 | `docs/PRD.md` §19.4; `docs/IP_AND_DATA_BOUNDARY.md` §7 | "Public report templates shall not include protected standards text, protected tables, proprietary formulas, private rule-pack content, or private project data." Reports "may reference a user rule-pack ID, version, checksum, and source note." | Report preview/export UI must show rule-pack identity by reference, never inline protected content. |
| C-49 | `DEL-07-06-RQ-009` (`ScopeOfWork.md` line 166) | "Public GUI fixtures, screenshots, report examples, and checklist examples shall not include protected standards content, proprietary values, private project data, or copied commercial examples." | Any design mock, screenshot, or fixture shipped publicly uses invented data only. |
| C-50 | `docs/CONTRACT.md` §1, `OPS-K-RULE-1`; `docs/PRD.md` §8.5 | "Public rule-pack examples must use invented non-code values and clear non-engineering notices." | Bundled example rule packs are labelled invented/non-engineering in the UI. |
| C-51 | SCA-009 `Vocabulary_Annex.md` §1.4 (Hanger-selection framing) | Spring-hanger selection operates only over "**user-imported** hanger libraries ... **No hanger catalog is bundled.**" | A hanger-selection UI must start from a user-imported library and offer no built-in catalog. |
| C-52 | `docs/CONTRACT.md` §1, `OPS-K-RULE-3`; `docs/PRD.md` §19.2 | Rule packs are "versioned, checksummed, source-noted, and marked public/private". | Rule-pack surfaces display name, version, checksum, source note, and public/private marking. |
| C-53 | `docs/CONTRACT.md` §1, `OPS-K-RULE-2` | "The expression evaluator is sandboxed; rule packs cannot execute arbitrary code." | No rule-pack surface may offer a scripting/eval escape hatch. |

### 1.5 Warning / blocking taxonomy

| ID | Source | Rule (key phrase quoted) | UI implication |
|---|---|---|---|
| C-54 | `docs/SPEC.md` §8 (warning-class table) | Six classes: `SOLVE_BLOCKING` (required physical input missing); `RULE_CHECK_BLOCKING` (mechanics can solve, rule-pack check lacks required user/code data); `PROVENANCE_WARNING` (value exists but source missing or weak); `ASSUMPTION_WARNING` (user or model assumption requires review); `NONLINEAR_WARNING` (convergence or active-state uncertainty); `IP_BOUNDARY_WARNING` (public contribution/report may contain protected or private data). | Diagnostics UI must render these six classes as distinct, addressable categories. |
| C-55 | `docs/PRD.md` §14.4 Warning and Blocking UX | Five UI distinctions: "Blocking for model validity" (schema/topology invalid); "Blocking for solve"; "Blocking for user-rule check"; "Warning" (data exists but incomplete/unverified/unusual/out of range); "Informational" (assumption or note recorded for review). | The GUI must expose model-validity blocking separately from solve blocking and rule-check blocking. |
| C-56 | `DEL-07-06-RQ-006` (`ScopeOfWork.md` line 163) | Inline validation messages "shall distinguish solve-blocking, rule-check-blocking, provenance, assumption, nonlinear, and IP-boundary warning classes"; verification asserts "**no collapse of solve warnings into code-check warnings**". | A unified issues list must preserve class identity per item. |
| C-57 | `docs/PRD.md` §11.3 `FR-GUI-006` | "GUI distinguishes solve blockers, rule-check blockers, warnings, and informational notes." (Priority: Must) | Mandatory functional requirement for any validation-message design. |
| C-58 | `docs/CONTRACT.md` §1, `OPS-K-SOLVER-2` | "Nonlinear support behavior must report convergence, active-set state, and unresolved non-convergence." | Solve/results UI must surface convergence, active-set state, and unresolved non-convergence rather than a binary solved/failed. |
| C-59 | `.../DEL-07-04_Missing-data warning and blocking UX/_STATUS.md` (Remaining) | The GUI "surfaces supplied `NONLINEAR_*` diagnostics but does not invent convergence outcomes or missing solver evidence." | The UI renders producer-supplied diagnostics only; it must not synthesise or infer convergence state. |
| C-60 | `docs/INTENT.md` "Graphical user interface intent" | The GUI "should use warnings and blocking checks to distinguish between: data required to solve the model; data required to perform a code check; assumptions made by the user; missing source/provenance information; nonlinear convergence issues; questionable model conditions; values outside expected engineering ranges." | Seven intent-level distinctions the warning design should remain able to express. |
| C-61 | `docs/SPEC.md` §7 (rule-pack evaluator, line ~668) | Blocking findings "map to `RULE_INPUTS_INCOMPLETE` and `RULE_CHECK_BLOCKING` semantics". | Rule-pack completeness failures render as these two codes, not as a rule failure. |

### 1.6 Model states, runs, results (Current vs Historical, immutability)

| ID | Source | Rule (key phrase quoted) | UI implication |
|---|---|---|---|
| C-62 | `docs/PRD.md` §15.1; §8.6 | "A model state shall be immutable after creation. Users may create new states from existing states, but historical states shall remain reproducible." | No in-place editing affordance on a saved state; editing forks a new state. |
| C-63 | `docs/SPEC.md` §3.2 | "Changes to a state payload create a new model state." Model states carry "deterministic hashes, and a read-only snapshot policy". | State views are read-only and hash-labelled. |
| C-64 | `docs/PRD.md` §5.7; §15.2 | "Every result set shall bind to the model state, solver version, analysis settings, load cases, rule-pack references, and library versions used to produce it." An analysis run records run ID, model state ID, solver version, build/release identifier, settings, active load cases, combination cases, rule-pack refs, library refs, solve diagnostics, results, warnings, timestamp. | A results screen must show the full binding set, not results alone. |
| C-65 | `docs/CONTRACT.md` §1, `OPS-K-AUTH-2` | "Human acceptance records, if used, bind to specific model/rule/report hashes and do not survive content changes without re-review." | Any acceptance reference shown must display its bound hashes and visibly invalidate on change. |
| C-66 | `apps/desktop/src/features/results/HistoricalRunContext.tsx` (header comment, `designation: "historical_saved_run"`) | "Transient evidence only. Saved run references do not contain the historical input-manifest payload, so reopening cannot establish a current solve basis." | A reopened saved run must be labelled Historical (panel label "Historical saved run") and must not be presented as a current result. |
| C-67 | `.../DEL-05-04_Analysis status semantics/_STATUS.md` (2026-08-20) | The applier "compares supported claimed model hashes against the current RFC8785/JCS backend model hash and **fails closed for stale, malformed, or unsupported claims**". | The UI must not display a result against a mutated model; stale results are invalidated, not silently reused. |
| C-68 | `.../DEL-16-03_User acceptance and operation audit trail/_STATUS.md` (Remaining) | "Reopened records must remain review context with **acceptance unknown**, requiring explicit requeue and fresh validation." Durable accepted/rejected history is incomplete under SOW-070. | Reopened proposal history must show acceptance as unknown; no persisted "accepted" badge may be implied today. |
| C-69 | `docs/PRD.md` §14.5 Comparison UI | The comparison UI "shall show": selected states/runs; mapping table; unmatched entities; added/deleted/modified entities; changed solver settings; changed rule-pack/library references; result deltas; out-of-tolerance items; graphical overlays; export options. | Minimum comparison-screen content contract. |
| C-70 | `docs/PRD.md` §15.4 Entity Mapping | Mapping hierarchy: "1. stable ID match; 2. user-defined manual mapping; 3. unmatched / added / deleted classification; 4. optional future geometric/topological suggested matching." | Comparison UI must expose manual mapping and an explicit unmatched class; automatic geometric matching is not current. |
| C-71 | `docs/PRD.md` §15.5 | "Users shall be able to define absolute and relative tolerances by result category" (displacement, rotation, force, moment, reaction, stress, stress ratio, temperature, pressure, support gap or movement). | Tolerance profiles are user-editable per category; no hardcoded tolerance verdicts. |
| C-72 | `docs/PRD.md` §15.6 | An external result state "is not an MVP requirement". | No import-prover-results workflow in current scope; architecture only. |
| C-73 | `docs/PRD.md` §11.1 `FR-MOD-003`, `FR-MOD-004` | Entity IDs "persist across edits unless user intentionally creates new entities"; users "can label states/models using flexible naming conventions **without enforced prover status**". | Naming/tagging is free-form; the UI must not impose a state-status taxonomy. |

### 1.7 Agent proposals and review gates

| ID | Source | Rule (key phrase quoted) | UI implication |
|---|---|---|---|
| C-74 | `docs/PRD.md` §5.3 | "The user may modify the model manually through the GUI. Agents may also propose changes. In both cases, changes shall be represented as structured model operations, validated against the schema, shown to the user, and applied only through the model engine." / "Agent output shall never become accepted engineering work by itself." | Every mutation — human or agent — is a structured operation; the UI must show it before it applies. |
| C-75 | SCA-009 `Vocabulary_Annex.md` §1.2 (Single mutation route) | "Every vocabulary item, human- or agent-invoked, routes through the one PKG-16 structured-operation layer ... **No vocabulary item may introduce a second mutation route.**" | No direct-edit path may bypass the operation layer, however convenient. |
| C-76 | `docs/PRD.md` §11.8 `FR-AGENT-001`–`FR-AGENT-005` | Agent proposals are "structured operations, not direct unsupervised model mutations"; they "must pass schema and constraint validation before application"; "shown as diffs and require user acceptance unless the user configures another workflow"; accepted operations "record rationale, constraints considered, and unresolved assumptions"; `FR-AGENT-005` (Must): "Agent output cannot claim engineering acceptance, certification, or code compliance for reliance." | Agent panel needs: operation list, validation result, diff preview, explicit accept control, rationale/assumption capture, and boundary-compliant language. |
| C-77 | `docs/CONTRACT.md` §1, `OPS-K-AGENT-4`; `docs/PROFESSIONAL_BOUNDARY.md` §3 (Agent output row) | "Agent outputs are drafts/proposals until accepted by a human gate." Agent output may state "Drafts, proposals, evidence summaries, checks, and open issues" and must not state that it "is accepted policy, accepted engineering work, or professional approval without a human gate." | Agent output is visually marked proposal/draft until a user accepts it. |
| C-78 | `docs/CONTRACT.md` §1, `OPS-K-AGENT-1`, `OPS-K-AGENT-2` | "Agents must not invent engineering values, scope, source citations, or legal conclusions; unknowns become `TBD`." / "Agents must surface conflicts and gaps rather than silently resolving them." | The agent surface must render `TBD` and conflict/gap items rather than filled-in values. |
| C-79 | `docs/PRD.md` §9.6 (Agent-Enhanced User) | Key needs: "agent proposals as structured operations; visible diffs; user review gates; schema validation; undo/redo; audit trail." | Six affordances the agent workflow is expected to provide. |
| C-80 | `DEC-042` (`SOFTWARE_DECOMP.md` §12); `DEC-041`; `DEC-091` (D-58) | v0.2/R7 agent adoption "remain[s] HELD"; only "harness-INDEPENDENT, fence-free embedded-agent PREPARATION may proceed ... **No live agent binding**, no R7 scope adoption, and no app-dev dependency consumption proceed under this sanction." `DEC-091` retires the prior automation-condition mechanism and leaves it "unresolved". | A design may include an agent seam/panel, but a shipped live model-provider binding is gated and not currently released. |
| C-81 | `.../DEL-16-04_Agent rationale and professional-boundary controls/_STATUS.md` (Remaining) | The persisted-report workflow "does not bind agent rationale into downstream runtime integrations or infer human/professional approval"; the `FR-AGENT-005` "professional-boundary hard gate" is stage-gated to v0.2 R7. | Agent rationale may be recorded and displayed, but must not propagate as approval anywhere downstream. |
| C-82 | SCA-009 `Vocabulary_Annex.md` §1.1 (Taxonomy binding) | The vocabulary binds to "the **implemented** operation taxonomy — the closed change-kind set accepted by `operation_applier` `check_kinds` ... **not** to the broader schema `OperationKind` enum"; "Coverage claims against the schema enum alone are non-conforming." | A tool palette may expose only commands with a real resolver; schema tokens without resolvers (`move`, `reconnect`, `constraint`, `design_knowledge`, `move_geometry`) must not be surfaced as working tools. |
| C-83 | SCA-009 `Vocabulary_Annex.md` §1.3 (Sequencing rule) | "NORMATIVE-NOW wiring of **existing** backend capability ... comes first; **net-new** backend capability (Tier 3 items) is implemented separately and afterwards." | Palette work is sequenced: Tier-1/Tier-2 exposure before Tier-3 net-new commands. |
| C-84 | SCA-009 `Vocabulary_Annex.md` §5; `DEC-094` | "single palette-surface owner per D4"; "every palette command routes through the PKG-16 structured-operation layer"; `DEL-07-09` "never dispatches implementation; its coverage ledger routes each vocabulary item to the owning deliverable(s)". | One palette surface; DEL-07-09 owns the contract/coverage, DEL-07-01/02/03 and PKG-16 own implementation. |

### 1.8 Privacy, telemetry, local-first

| ID | Source | Rule (key phrase quoted) | UI implication |
|---|---|---|---|
| C-85 | `docs/PRD.md` §20.4; `docs/SPEC.md` §4.4 | "Modeling, solving, rule evaluation, reporting, comparison, and export shall work locally without a cloud service." "The MVP storage posture is local-first and offline-capable ... no hosted database, daemon, cloud sync, or telemetry path by default." | No feature may require sign-in, network, or cloud availability to model, solve, check, report, compare, or export. |
| C-86 | `docs/CONTRACT.md` §1, `OPS-K-PRIV-2`; `docs/PRD.md` §20.5 | "Telemetry is off by default and cannot include private engineering/code data." If added, telemetry "shall be opt-in and shall not transmit private model, result, material, component, rule-pack, or project data without explicit user action." | Any analytics affordance is off by default and opt-in, and excludes engineering payloads. |
| C-87 | `docs/CONTRACT.md` §1, `OPS-K-PRIV-1` as amended by `DEC-051` (D-T0-04 open residency) | "the app does not enforce data residency on the agent / model-provider channel: when the owner configures a model provider (local, Anthropic, or other), private project/model and Class-B data may be transmitted to it, **with no further app-side guard, opt-in gate, or indicator**." | The owner ruled explicitly against surfacing an egress indicator or consent gate on that channel; a design must not reintroduce one as if required (and the accidental-egress risk is owner-accepted). |
| C-88 | `docs/PRD.md` §20.3; `docs/IP_AND_DATA_BOUNDARY.md` §6 | Private rule packs, material libraries, component catalogs, owner standards, and project design bases "shall be stored in user-controlled paths and excluded from telemetry, public examples, and issue bundles by default." | Bug-report/issue/export flows default to excluding private data; inclusion is an explicit user act. |
| C-89 | `loop/WORKPLAN_2026-07-18b_piping_loop.md` `F-PIP-1` | "local-only operation — no cloud, daemon, network, or telemetry features; no repository-default private-data writes; user-created models never committed; no protected standards content or private project data; **invented bundled fixtures only**." | Bundled sample content is invented; no default writes into the repository tree. |
| C-90 | `docs/SPEC.md` §4.4 | Adapters and plugins "must not bypass schema validation, unit metadata checks, provenance checks, rule-pack reference checks, private-data controls, protected-content screening, professional-boundary checks, or the SCA-003 prohibition on direct SQL access". | Any extension/plugin surface in the GUI inherits the same no-bypass controls. |

### 1.9 Accessibility and usability holds

| ID | Source | Rule (key phrase quoted) | UI implication |
|---|---|---|---|
| C-91 | `.../DEL-07-06.../_STATUS.md` (Remaining) — PDU-046 | "Preserve PDU-046 / `DEL-07-06-RQ-004` as `VERIFIED_NOT_VALIDATED` until a separately authorized independent usability basis and human-selected measurable contrast/readability target exist; the target remains `TBD_by_human_project_authority`." | No design may assert a contrast/readability conformance level; the numeric target is not yet chosen. |
| C-92 | `.../DEL-07-06.../_STATUS.md` (Remaining) — PDU-045 | "Obtain a separately authorized independent usability basis before upgrading PDU-045 beyond `VERIFIED_NOT_VALIDATED`". | Project-authored usability checks remain verification, never validation; no usability-validated claim. |
| C-93 | `.../DEL-07-06.../ScopeOfWork.md` CLM-017 | "Desktop runtime evaluation is not performed, **no accessibility conformance claim is emitted**, and contrast/readability findings remain **warnings** while the measurable target is `TBD_by_human_project_authority`." | Contrast/readability issues render as warnings, not failures; no WCAG/A11y badge. |
| C-94 | `.../DEL-07-06.../ScopeOfWork.md` CLM-035 | "Do not upgrade them to usability validation or measurable contrast/readability conformance without a separately authorized independent basis and a selected target." | Standing instruction against self-certifying the accessibility baseline. |
| C-95 | `DEL-07-06-RQ-001` | "keep model creation, missing data, results, assumptions, provenance, and diagnostics **visible enough for engineering review**." | Information density/progressive disclosure must not hide these six categories. |
| C-96 | `DEL-07-06-RQ-002` | "Major GUI panels and review workflows shall support keyboard navigation paths." | Keyboard reachability for every major panel and modal/task flow. |
| C-97 | `DEL-07-06-RQ-003` | "Icon actions, engineering status indicators, and compact controls shall have clear labels or tooltips." | Icon-only controls require accessible names/tooltips. |
| C-98 | `DEL-07-06-RQ-005` | "Large model-tree and result-review surfaces shall support search/filter and copy/export paths." | Search/filter and copy/export are required on trees and result tables. |
| C-99 | `DEL-07-06-RQ-008` | "Undo/redo affordances shall apply only to reversible model edits and shall preserve diagnostics when solve readiness changes." | Undo/redo must not be offered on irreversible actions; diagnostics refresh after undo. |
| C-100 | `.../DEL-07-01.../_run_records/WORKING_ITEMS_RUN_2026-09-05_WORKSPACE_DESIGN.md` | "The owner selected 1024 × 768 as the **initial minimum window/layout test target**; this does not establish independent usability acceptance." "Native configuration now allows a minimum client logical size of 1024 × 768; the **default 1440 × 920** size and all packaging/security settings are unchanged." | Layouts must work at 1024×768 client logical size; default window is 1440×920. |
| C-101 | `.../DEL-07-01.../MEMORY.md` line 456 | "The owner's 1024 × 768 target is an initial client logical window/layout test target, **not independent readability/usability acceptance**. DEL-07-06 PDU-045/PDU-046 remain held under their existing requirements." | Passing at 1024×768 is a layout regression signal only; it does not discharge the usability holds. |
| C-102 | `.../DEL-07-06.../ScopeOfWork.md` conflict row `DEL-07-06-CF-001` (sources: `docs/_Registers/ScopeLedger.csv` SOW-036 notes; `SOFTWARE_DECOMP.md` OI-002) | "the detailed WCAG target is explicitly TBD"; proposed authority: "Keep conformance target as `TBD`; allow only **qualitative** setup requirements until human ruling." Human ruling column: `TBD`. | Accessibility requirements stay qualitative until the owner rules a measurable target. |

### 1.10 Testing and evidence expectations for UI changes

| ID | Source | Rule (key phrase quoted) | UI implication |
|---|---|---|---|
| C-103 | `plans/DRAFT_2026-06-11_H4_coordination_evidence_posture.md` (approved and applied 2026-06-11 into `_COORDINATION.md` step 8) | "when a tranche changes user-visible desktop behavior, the **default evidence is an extension of the Playwright e2e spec(s)** exercising the changed behavior in a real browser; a manual live-browser smoke note in `apps/desktop/SMOKE.md` without a spec extension is **the exception and must record why** automation was not extended." | Any user-visible UI change ships with an e2e spec extension by default. |
| C-104 | Same, H4 | "New React components land with unit tests (Vitest) at or above the slice's existing coverage pattern; a component with no unit test is a **recorded evidence gap, not a silent omission**." | Each new component needs a Vitest unit test or an explicitly recorded gap. |
| C-105 | Same, H4 (template-batch rule) | "Homogeneous UI slices (several near-identical forms, rows, or panels produced from one template) may record **one template-level test plus per-instance smoke assertions** ... provided the run record names every instance covered by the template rule." | Repeated UI patterns may share one template test if every instance is enumerated in the run record. |
| C-106 | `loop/WORKPLAN_2026-07-18b_piping_loop.md` step 4; `DEC-025` | "the `DEC-025` five-surface evidence sweep (`python3 tools/release/run_evidence_sweep.py --execute`) as the pre-push merge gate for **every code-touching branch**"; plus "repo-wide `self-check` exit 0 and full practitioner-harness pytest at closeout." | UI code changes are gated by the five-surface sweep and repo self-check (which includes the GEN-13 claims lint, C-20). |
| C-107 | `DEC-093` (D-65 ruling, 2026-08-19) | A sweep may satisfy surface 4 from `.github/workflows/piping-desktop-e2e.yml` when the summary binds workflow path, run ID, run attempt, exact head SHA (== sweep `commit_hash`), conclusion `success`, `working_tree_dirty=false`, "and the registered e2e specs must have executed **across both registered viewport projects**." | E2E evidence must cover both registered viewport projects; a spec failure is a genuine blocking failure. |
| C-108 | `DEC-093` (D-65 ruling) | "**Exploratory and agent-as-user testing are excluded from all sweep surfaces.**" | Agent-driven click-throughs of the GUI are not admissible evidence for a UI change. |
| C-109 | `docs/SPEC.md` §12 Acceptance semantics | An increment may be accepted for development use only when scope/deliverable ID match, artifacts exist, "no protected public data has been introduced", "missing data and assumptions are surfaced", validation gates pass, and "a human accepts the work at the review gate." | UI increments carry the same six acceptance conditions. |
| C-110 | `loop/WORKPLAN_2026-07-18b_piping_loop.md` `F-PIP-3` | "deliverable lifecycle transitions follow the register's ruled gates; no `CHECKING -> ISSUED` issuance without the owner's gate." | Shipping a UI redesign does not promote PKG-07 deliverables; all PKG-07 deliverables are currently `IN_PROGRESS`/`OPEN`. |

---

## 2. Accepted GUI-related decisions

`DEC-nnn` rows live in `execution/_Decomposition/SOFTWARE_DECOMP.md` §12; `D-nn` packets/rulings in
`execution/_Coordination/_DECISIONS/`.

| ID | Date | One-line summary | Binds future UI design? |
|---|---|---|---|
| `DEC-018` (D-01) | 2026-06-10 | SI-canonical internal unit set with a **dual display catalog**, dual absolute/interval temperature semantics, explicit gauge/absolute pressure kinds with no silent atmospheric default, two-tier conversion-witness tolerance. | **Yes** — governs unit display, temperature/pressure input semantics. |
| `DEC-019` (D-08) | 2026-06-10 | Model-document schema migration policy over the store `user_version` ledger. | Indirect — migration status is surfaced in the persistence envelope. |
| `DEC-020` (D-13) | 2026-06-11 | Operation-seam engine unification: **wasm32 build of `operation_applier` is the sole browser-mode operation engine**; TypeScript retains React UI, intent builders, and a thin routing adapter; ADR surface stood up (ADR-0001). | **Yes** — the UI layer may not own validation/apply logic. |
| `DEC-021` (D-10) | 2026-06-11 | Report rendering target (Option B); `D-10b`/`DEC-061` adds a hash-bound deterministic PDF emitter. | **Yes** — constrains report preview/export surfaces. |
| `DEC-029` / `DEC-035` (D-14) | 2026-06-12 | R2→R3 stage advancement, with the **authoring-usability finding** named as a blocking residual at the R3 exit review. | Indirect — usability residual is recorded project history. |
| `DEC-037` (D-02b) | 2026-06-14 | **No writable rule-expression text input may ship**; the C2 composer may show a deterministic one-way AST→text rendering "UI-labeled display-only / not accepted as input"; no parser anywhere; re-presented on a recorded human usability finding against the structured composer. | **Yes** — hard constraint on rule-pack editor design. |
| `DEC-041` (D-22) | 2026-06-18 | Embedded-agent runtime substrate: consume the app-dev harness as versioned packages (contract pkg → React UI pkg → Node sidecar); Tauri-refactor and Rust re-implementation rejected. | **Yes** — shapes the agent panel seam. |
| `DEC-042` (D-21 disposition) | 2026-06-18 | v0.2/R7 agent scope **remains HELD**; only harness-**independent** embedded-agent preparation is sanctioned — "No live agent binding, no R7 scope adoption". | **Yes** — an agent panel must be harness-independent and must not ship a live binding. |
| `DEC-049` (D-15) | 2026-06-21 | Minimal dedicated user-entered spring-hanger model; "no catalog/protected/default/professional claim". | **Yes** — hanger authoring UI is user-entered only. |
| `DEC-051` (D-24) | 2026-06-21 | Open residency: agent/model-provider channel gets **no app-side guard, opt-in gate, or indicator**; telemetry-off and public-commit boundaries unaffected. | **Yes** — explicitly forecloses an egress-consent UI on that channel. |
| `DEC-056` (D-21) | 2026-07-02 | Adopts the v0.2 milestone set (R6 Design Knowledge/Handoff Beta, R7 Agent-Assisted Design, inserted v0.2 R3 States/Runs/Comparison) with the Annex A FR crosswalk. | **Yes** — defines the milestone frame for states/comparison and agent UI. |
| `DEC-069` (D-37) | 2026-07-09 | Names the absent special domains (buried pipe, jacketed, FRP, slug/two-phase, snubbers, fatigue counting, nozzle flexibility, mitered bend/stepped reducer, EJ hardware kinematics) as recorded PRD non-goals. | **Yes** — bars implying these capabilities in UI affordances. |
| `DEC-080` (D-47) | 2026-07-16 | PRD v0.2→v0.3: external-prover correlation is the **principal validation posture**; internal suite is development verification. | **Yes** — anchors `BS-VALID` placement. |
| `DEC-081` (D-48) | 2026-07-16 | **Claims-language taxonomy**: BS-IP / BS-ACCEPT / BS-VALID / BS-MATURITY / GF-TOKEN with canonical texts and placement rules; evidence-status labels; governed `docs/claims_registry.md`; deterministic claims lint; Wave 1 aligned ~12 product UI files. | **Yes** — the single source for all boundary language in the GUI. |
| `DEC-082` / `DEC-085` / `DEC-087` (D-49 / D-52 / D-54) | 2026-07-17 / 2026-07-18 / 2026-07-18 | Standing agent decision-latitude delegation and standing-approval overlay, with limits reserving lifecycle/acceptance/scope/normative acts to the owner. | Indirect — governs how design changes may be decided, not what the UI shows. |
| `DEC-089` (D-06b) | 2026-07-25 | Policy-only future target of Apple signing/notarization; App ID description "**OpenPipeStress Technical Preview**", bundle ID `org.openpipestress.technical-preview`; current unsigned posture and unsigned-install caveat continue. | Indirect — product naming/maturity framing. |
| `DEC-091` (D-58) | 2026-07-27 / effective 2026-07-28 | Retires current reliance on the App-era synchronized-consumption mechanism; **the automation-condition mechanism remains unresolved**; Piping stays outside the App-harness client set; PRD R7 unchanged. | **Yes** — live agent provider binding remains ungated-open work, not shippable. |
| `DEC-093` (D-65) | 2026-08-19 | CI-produced surface-4 evidence accepted with exact binding; e2e specs must execute "across both registered viewport projects"; exploratory and agent-as-user testing excluded from all sweep surfaces. | **Yes** — evidence rules for UI changes. |
| `DEC-094` (SCA-009) | 2026-08-20 | Adds `SOW-077` + `DEL-07-09` "Interactive operation vocabulary and tool palette contract" (PKG-07, `UX_UI_SLICE`, envelope L); ratifies the two-class vocabulary bound to the **implemented** operation taxonomy; **single palette surface**, every command through the PKG-16 operation layer; DEL-07-03 R-005/R-006 ownership re-points to DEL-07-09. | **Yes** — the governing contract for any tool palette / command surface. |
| `D-64` ruling | 2026-08-04 | Adopts the owner intent records as the Piping product basis: OpenPipeStress as first domain-specific application target with "robust UI, semantically equivalent agent-facing API, and a Piping-tailored operative runtime/harness, all bounded as a **limited-purpose design tool with engineering judgment and validation external**." | **Yes** — product-basis framing for UI ambition and its boundary. |
| `D-66` ruling (+ addendum) | 2026-09-08 | Partially ruled physics/coverage/native-3D-authoring wave; "F4/UI product source writes remain prerequisite-conditioned". | **Yes** — UI source writes are sequenced behind named prerequisites. |
| `D-67` ruling | 2026-09-14 | Adopts analysis-record 0.2.0 / stress-neutral 0.2.0 compatibility, private dormant pressure kernel, and a "**production-UI readiness handoff**"; "Production UI, CAEPIPE and live harness integration are **deferred**; the handoff names when UI foundation can begin." | **Yes** — current gate on beginning production UI work. |

Related but non-GUI-binding rows consulted and found not to constrain UI content: `DEC-022`/`DEC-023`
(grammar/solver), `DEC-024`/`DEC-026` (tolerances), `DEC-025` (merge gate — see C-106), `DEC-027`
(maintainer quorum), `DEC-028` (package container), `DEC-057`–`DEC-062` (release machinery),
`DEC-086`/`DEC-088`/`DEC-090` (one-off whitespace exceptions).

---

## 3. SCA-009 interactive-operation vocabulary — 24 NORMATIVE-NOW + 3 ROADMAP

Source: `execution/_ScopeChange/SCA-009_2026-08-20_0000/Vocabulary_Annex.md` §§3–4
(ratified by the owner 2026-08-20; codified as `DEC-094`; SCA-009 status `accepted` per
`execution/_ScopeChange/_LATEST.md`). The annex file header still reads
`CANDIDATE NORMATIVE TEXT — GATE 3 NOT APPROVED`; `DEL-07-09/_STATUS.md` (2026-08-21) and
`SOFTWARE_DECOMP` revision 0.12 treat it as the accepted coverage contract.

### 3.1 Tier 1 — end-to-end today (exposure/polish gaps only)

| # | Item | Meaning (one line) |
|---|---|---|
| 1 | Pipe run definition (routing, continuation) | Create/delete nodes and connect or delete pipe runs to lay out routing. |
| 2 | Anchors and DOF-set restraints | Create, edit, and delete linear supports defined by their restrained degrees of freedom. |
| 3 | Materials with temperature-dependent E/G/alpha (create + edit; no delete) | Author material records with temperature-indexed properties; no delete change kind exists. |
| 4 | Temperature / pressure load cases | Create, edit, and delete thermal and pressure primitive load cases. |
| 5 | Force / moment / displacement loads | Create, update, and delete concentrated force, moment, and imposed-displacement loads. |
| 6 | Wind equivalent-static loading | Author equivalent-static wind loads (known gap: `exposed_spans` editing not yet exposed). |
| 7 | Seismic static-g loading | Generate equivalent-static seismic loads from a static-g input. |
| 8 | Load-case combinations (3 algebra bases incl. range envelopes) | Build combinations over the closed basis set `mechanics`, `result_state_subtraction`, `range_envelope`. |
| 9 | Typed selection identity | `FieldKind::EntityRef` is the typed targeting contract for `set_field`/`update_*` operations. |
| 10 | Session-scoped undo/redo | Undo/redo checkpoint inversion across all applied change kinds, within a session. |
| 11 | Agent-proposal route (full validate/preview/apply) | `OperationAuthorType` includes `"agent"`; agent proposals use the same applier route as human edits. |

### 3.2 Tier 2 — backend-complete, authoring vocabulary missing

| # | Item | Meaning (one line) |
|---|---|---|
| 12 | Spring hangers (variable / constant-effort) | Author installed/cold/hot loads, travel, and limits onto existing `SpringHangerInput` support fields. |
| 13 | Nonlinear supports (one-way, lift-off, gap, friction) | Author solve-complete nonlinear support parameters, including derived normal reaction, via support operations. |
| 14 | Expansion joints (4-axis stiffness, pressure-thrust refs) | Editable today via `set_field`; creation is blocked behind `insert_component_symbol`. |
| 15 | Component creation — bends, tees, reducers, valves, flanges | Unblock/complete `insert_component_symbol` so components can be created, not only field-edited. |
| 16 | Named support families LineStop / VerticalSupport surfaced | Stop collapsing partial-DOF support sets to `Guide`; preserve the emitted family name. |
| 17 | Sections: create exists; delete and by-ref assignment missing | Add section deletion and by-reference assignment to the operation taxonomy and editor. |
| 18 | Material / section / component deletion | Requires new `delete_*` change kinds plus removal affordances in the inspector/editors. |

### 3.3 Tier 3 — accepted net-new (NORMATIVE-NOW, sequenced after existing-capability wiring)

| # | Item | Meaning (one line) |
|---|---|---|
| 19 | Element insert / split (break pipe, insert-in-run) | New change kind(s) to split a run and insert an element; no resolver exists today. |
| 20 | Copy / rotate / mirror of runs | New resolver-backed geometry-transform kinds; the bare schema tokens `move_geometry`/`reconnect` are non-conforming. |
| 21 | Nozzle / equipment boundary conditions | New boundary-condition authoring bound to `create_support`-class or new kinds; exports currently name it unsupported. |
| 22 | Automatic self-weight case generation | Generation path emitting standard `create_load_case`/`create_primitive_load` operations from computed mass-per-length. |
| 23 | Spring-hanger selection from user-imported hanger libraries | Selection resolves to support payloads; DEC-049 framing (no bundled catalog) is mandatory. |
| 24 | Project-wide unit switching as a display-system toggle | Display-layer only; no model mutation, no change kind, entered-units storage preserved. |

### 3.4 ROADMAP (deferred; not DEL-07-09 coverage obligations)

| # | Item | Meaning (one line) |
|---|---|---|
| R1 | Node renumbering | No implementation or change kind exists; deferred in the accepted Tier-3 triage. |
| R2 | Snubbers | Zero repository hits across `core/`, `apps/`, `schemas/` (verified 2026-08-20); also a PRD §7 non-goal. |
| R3 | Cold spring / cut-short | No repository implementation; owner: "rarely used and I'm comfortable adding that later rather than now". |

"Promoting any ROADMAP item into NORMATIVE-NOW is a future owner act (scope-change or owner-ruled
deliverable amendment), **not an implementation choice**."

---

## 4. Mandatory disclosures — text or state that must be visible somewhere

| # | Disclosure | Required text / state | Where | Source |
|---|---|---|---|---|
| M-01 | Technical-preview maturity line | Exactly: "Technical preview — not a released product." (one sentence, never a compound litany) | App shell banner/footer; packaging/build-readiness surfaces may reuse it. Currently rendered at `apps/desktop/src/App.tsx:2647`. | `docs/claims_registry.md` §1 `BS-MATURITY`; lint anchor `MISSING_MATURITY_BANNER` in `[repo root] tools/validation/validate_claims_language.py` |
| M-02 | Acceptance/authority boundary (professional judgment) | "Results are engineering decision-support information. Acceptance, professional judgment, and any certification, sealing, or code-compliance determination remain with the responsible engineer and project authority." (or a listed short variant) | Results, rule-check, comparison, solve, report-preview — "any place a computed outcome is shown". Present in `App.tsx:2647` footer (short variant) and `features/rule-check/RuleCheckPanel.tsx:99`. | `docs/claims_registry.md` §1 `BS-ACCEPT` |
| M-03 | Required report notice (PRD §19.3 composite) | "OpenPipeStress is decision-support software for piping design, flexibility, and stress-analysis workflows. It computes mechanical results from recorded user inputs and may evaluate user-supplied rule packs. It does not certify, seal, approve, authenticate, or determine code compliance for professional reliance. Code-specific and project-specific data are supplied by the user or user-controlled private sources. Competent human review and, where required, validation in accepted professional tools remain the responsibility of the user and project authority." | Every generated report and public report template; emitted by the report renderer (`core/reporting/report_renderer/src/lib.rs`), so also visible in report preview. Changes "require a PRD-level owner act". | `docs/PRD.md` §19.3; `docs/report_notice_template.md`; lint anchors `MISSING_PRD_NOTICE`, `MISSING_RENDERER_NOTICE` |
| M-04 | Report supplement for workflow-evidence artifacts | "Design-authoring records, comparison outputs, handoff packages, export metadata, and external-prover references are review aids only unless a separate competent human acceptance record states otherwise for the bound project basis." | Reports that include design-authoring records, comparison outputs, handoff packages, export metadata, or external-prover references. | `docs/report_notice_template.md` §"Required Notice" (second block) |
| M-05 | Content/IP boundary | "OpenPipeStress ships no protected standards content. All code-specific values, tables, allowables, and factors are supplied by the user or user-controlled private sources, with provenance recorded." (or a listed short variant) | Import / library / contribution UI, redaction and export surfaces, contribution docs. Not on results views unless the surface handles imported content. | `docs/claims_registry.md` §1 `BS-IP` |
| M-06 | Validation posture | "Candidate designs are validated in the user's accepted professional tools (external-prover correlation, PRD §22.5). Internal benchmarks and rule checks are development verification and screening evidence." (or a listed short variant) | Handoff/export UI (MBF, PCF, stress-neutral, native package, external-prover surfaces), validation manual, headless-runner evidence surfaces. | `docs/claims_registry.md` §1 `BS-VALID` |
| M-07 | Report required content block | Software version, solver version, build/release identifier, date/time, units, coordinate system, model state ID/name/hash, analysis run ID/name, library references, rule-pack name/version/checksum, source/provenance summary, warnings, unresolved assumptions, load cases, solver settings and diagnostics, results, comparison results where selected, and a **review/signoff block**. | Generated reports (and therefore report preview). | `docs/PRD.md` §19.2; `docs/SPEC.md` §9; `OPS-K-REPORT-1` |
| M-08 | Analysis status (authority-attributed) | One of `MODEL_INCOMPLETE`, `MECHANICS_SOLVED`, `RULE_INPUTS_INCOMPLETE`, `USER_RULE_CHECKED`, `USER_RULE_FAILED`, `HUMAN_REVIEW_REQUIRED` — attributed to its authority domain. | Any surface showing solve or rule-check state. | `docs/TYPES.md` §4; `docs/SPEC.md` §4.3 |
| M-09 | Evidence-status label per result | `INTERNALLY_VERIFIED` or `PROVER_CORRELATED` attached to individual results, reports, and case pages (`ENGINEER_ACCEPTED` reserved, not emitted in MVP). | Individual results/reports/case pages — "instead of global hedging". | `docs/claims_registry.md` §2 |
| M-10 | Missing-data findings and warning classes | Explicit findings for every solve-required and rule-check-required missing value, classed as `SOLVE_BLOCKING` / `RULE_CHECK_BLOCKING` / `PROVENANCE_WARNING` / `ASSUMPTION_WARNING` / `NONLINEAR_WARNING` / `IP_BOUNDARY_WARNING`. | Wherever the affected entity is authored or reviewed. | `docs/SPEC.md` §8; `OPS-K-DATA-2`; `docs/PRD.md` §14.4 |
| M-11 | Units on every unit-bearing value | Visible unit metadata in fields, tables, exports, and report previews. | All numeric surfaces. | `OPS-K-UNIT-1`; `DEL-07-06-RQ-007` |
| M-12 | Provenance on governed data | Source, provenance, redistribution status, and review status on materials, components, sections, SIF/flexibility inputs, allowables, and rule-pack values. | Property inspector, library editors, import review. | `OPS-K-DATA-3`; `docs/PRD.md` §14.3; `docs/IP_AND_DATA_BOUNDARY.md` §4 |
| M-13 | Historical-vs-current run designation | A reopened saved run must be presented as "Historical saved run" (`designation: "historical_saved_run"`), with the stated limit that "reopening cannot establish a current solve basis". | Results area when a saved run is reopened. | `apps/desktop/src/features/results/HistoricalRunContext.tsx` |
| M-14 | Agent output standing | Agent output marked as draft/proposal, shown as a diff, requiring explicit user acceptance; rationale, constraints considered, and unresolved assumptions recorded on accepted operations. | Agent proposal/review surface. | `docs/PRD.md` §11.8 `FR-AGENT-001`–`005`; `OPS-K-AGENT-4` |
| M-15 | Rule-expression text rendering label | Any AST→text rendering must be "UI-labeled display-only / not accepted as input", with the notation declared non-frozen. | Rule-pack expression composer. | `DEC-037` |
| M-16 | Accessibility standing | Contrast/readability findings presented as **warnings**; no accessibility conformance claim emitted; measurable target `TBD_by_human_project_authority`. | Accessibility/usability surfaces and any a11y reporting. | `.../DEL-07-06.../ScopeOfWork.md` CLM-017/CLM-018; `_STATUS.md` Remaining |
| M-17 | Release-level disclosure (adjacent) | Public releases must disclose "scope, validation status, known limitations, data-boundary constraints, and professional-responsibility limitations". | Release notes / about-and-limitations surface. | `OPS-K-GOV-3`; `docs/DIRECTIVE.md` §6 |

**Negative disclosure rules (must NOT be visible):**
`GF-TOKEN` ("Standard claim fence applies…") on any product surface (`docs/claims_registry.md` §1);
the phrase "not authoritative" and the `non-authoritative` evidence qualifier on any product surface
(`docs/claims_registry.md` §1–§2, lint `RETIRED_PHRASE`); any ad-hoc stack of ≥3 litany terms without
a registered boundary text (lint `AD_HOC_CLAIMS_LITANY`); `HUMAN_APPROVED_FOR_PROJECT`,
`CODE_COMPLIANT`, `CERTIFIED`, `SEALED`, `APPROVED` as automatic statuses (`docs/TYPES.md` §4).

---

## 5. Source index

- `docs/CONTRACT.md` — invariant catalog (`OPS-K-*`), enforcement map.
- `docs/DIRECTIVE.md` — founding intent, §3 non-negotiable principles, §4 scope, §5 stop rules, §6 governance baseline.
- `docs/PROFESSIONAL_BOUNDARY.md` — §3 authority boundaries, §4 permitted, §5 prohibited, §6 status vocabulary, §7 acceptance records, §8 report notices, §9 product-claim checklist, §10 open TBDs.
- `docs/claims_registry.md` — DEC-081 boundary statements, evidence labels, composite anchors, authoring directive, exclusions.
- `docs/PRD.md` — §4.2/§4.3, §5.3/5.5/5.6/5.7/5.9, §7, §8, §9, §11.1/11.3/11.8/11.9, §14 (all), §15, §16, §17.5, §18.4, §19, §20, §21, §22.5, §23.
- `docs/SPEC.md` — §3.2, §4, §4.3, §4.4, §4.5, §7, §8 (GUI + warning-class table), §9, §12.
- `docs/TYPES.md` — §3 deliverable types (`UX_UI_SLICE`), §4 analysis-status vocabulary.
- `docs/IP_AND_DATA_BOUNDARY.md` — §3–§8, incl. §6.1 runtime residency.
- `docs/report_notice_template.md` — PRD §19.3 composite and workflow-evidence supplement.
- `execution/_Decomposition/SOFTWARE_DECOMP.md` §12 — DEC decision log (rev 0.12).
- `execution/_Coordination/_DECISIONS/_REGISTER.md` and packet/ruling files — D-01…D-67.
- `execution/_ScopeChange/SCA-009_2026-08-20_0000/Vocabulary_Annex.md`; `execution/_ScopeChange/_LATEST.md`.
- `execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-01 … DEL-07-09` — `_STATUS.md`, `_CONTEXT.md`, `ScopeOfWork.md`, `MEMORY.md`, `_run_records/`.
- `execution/PKG-16_Model Operation and Agent Proposal Framework/1_Working/DEL-16-01 … DEL-16-04` — `_STATUS.md`.
- `plans/DRAFT_2026-06-11_H4_coordination_evidence_posture.md` — approved UI evidence posture.
- `loop/WORKPLAN_2026-07-18b_piping_loop.md` — fences `F-PIP-1` … `F-PIP-5`; evidence/merge-gate steps.
- `[repo root] tools/validation/validate_claims_language.py` — GEN-13 claims-language lint.
- `apps/desktop/src/App.tsx`, `apps/desktop/src/features/**` — current implemented product surfaces.
