# F — What binds the nine decision-packet items

**Return for sealed brief RESEARCH-F** (`{RUN}/briefs/RESEARCH-F_packet_bindings.md`).
Role: TASK, Type 2, bounded research, no delegation. Inventory only: no proposal,
no replacement wording, no judgement on what the owner should decide.

**Path convention.** Every path is relative to the repository root. Two
shorthands are used and expand to repo-relative paths:

- `{PROJ}` = `projects/chirality-piping`
- `{RUN}` = `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260916-UI-DESIGN-PROGRAM`

Everything outside `{PROJ}` is under `tools/`.

**Snapshot.** Counts were taken in one pass over the ROOT worktree on
2026-09-18, branch `claude/chirality-piping-ui-design-a31fd2`. The worktree
carries uncommitted changes under `{RUN}` (the run's own coordination files), so
counts that sweep `{RUN}` may move by one or two lines; counts in `{PROJ}/apps`,
`{PROJ}/core`, `{PROJ}/schemas`, `{PROJ}/fixtures`, `{PROJ}/docs` and
`tools/validation` are stable. `tools/validation/validate_claims_language.py`
run read-only at this revision reports `VALID claims-language surfaces: 320
files scanned; DEC-081 registry taxonomy satisfied`.

**One deviation from the brief's citation rule, and why.** The brief requires
file and line for every statement. `{RUN}/instances/DESIGN-SYSTEM/DESIGN_SYSTEM_V1.md`
is cited **by section and quotation, without a line number**, because it was
being rewritten while this inventory ran: a concurrent session executing the
sealed brief `DESIGN-SYSTEM-02` took it from 843 lines (V1) to 915, then 935,
then 993 during the pass, with text changing as well as moving. Its line 3 now
reads "Status: V1.1, revision under sealed brief DESIGN-SYSTEM-02 (phase 3 of
the interface program), 2026-09-18, for ROOT's acceptance." Line numbers into a
file in that state would be stale before they were read; the section number plus
the quoted sentence locates the passage and survives the renumbering, and each
such citation carries enough verbatim text to be found by search. Every other
file in this return is cited plain `file:line`, and nothing in `{PROJ}/apps`,
`{PROJ}/core`, `{PROJ}/schemas`, `{PROJ}/docs` or `tools/validation` moved during
the pass. See uncertainty 1a for what changed inside the passages quoted here.

---

## 0. Method, and what the counts mean

### 0.1 Occurrence classes

Every occurrence below is classified into exactly one of three classes.

| Class | Meaning | Paths |
|---|---|---|
| **Registered placement** | The text is there because a registry entry, an anchor, a schema constraint or a test requires it. Changing the governed text means changing this. | `{PROJ}/apps/**` (non-test), `{PROJ}/core/**` (non-test), `{PROJ}/schemas/**`, `{PROJ}/docs/**` (excluding `_history`), `tools/validation/**` |
| **Ordinary string** | The text appears inside prose, a fixture payload, a note or a test expectation that is not itself the registered placement, but would have to be reconciled. | `{PROJ}/fixtures/**`, `{PROJ}/examples/**`, `{PROJ}/validation/**`, `*.test.ts(x)`, `{PROJ}/tests/**`, `{PROJ}/apps/desktop/e2e/**`, `{PROJ}/apps/desktop/SMOKE.md` |
| **Historical record — must not change** | Ruled or accepted history, excluded from the claims lint by `{PROJ}/docs/claims_registry.md:130-134` and `tools/validation/validate_claims_language.py:117-128`. | `{PROJ}/execution/_Coordination/AgentRuns/**`, `**/_run_records/**`, `{PROJ}/execution/_Coordination/_DECISIONS/**`, `{PROJ}/execution/_ScopeChange/**`, `{PROJ}/execution/_Reconciliation/**`, `{PROJ}/execution/_Evaluation/**`, `{PROJ}/docs/_history/**`, `{PROJ}/plans/**`, `{PROJ}/loop/LOOP_RECEIPTS.md`, `{PROJ}/provenance/**` |

`{PROJ}/execution/PKG-*/1_Working/DEL-*/ScopeOfWork.md` is a fourth, distinct
case: it is a **live governance surface** in the lint's scan set
(`tools/validation/validate_claims_language.py:241-250`, Wave-2 gate `True` at
line 136), not history. It is counted separately as `SOW`.

### 0.2 What counts as an enforcement point

A discrete check that would fail, or a governed clause that would have to be
edited, if the item's text changed: one validator anchor, one lint constant, one
schema `const`/`enum`, one named test function, or one registry/contract clause.
Files are named with lines; the count is of checks, not of files.

### 0.3 Differences found between the constraints sheet and the source

The brief requires every citation in `{RUN}/instances/RESEARCH/C_ui_constraints.md`
to be verified at its source, and the source to govern where they differ. Nine
differences were found. Everything else in C that bears on the nine items
verified as written.

| # | C says | Source says | Where |
|---|---|---|---|
| D-1 | M-01 is "Currently rendered at `apps/desktop/src/App.tsx:2647`" (`C_ui_constraints.md:282`) | The footer carrying it is `{PROJ}/apps/desktop/src/App.tsx:3281-3283`. Line 2647 is `canRedo={...}` inside `WorkspaceToolbar` | `{PROJ}/apps/desktop/src/App.tsx:2647`, `:3282` |
| D-2 | M-02 is "Present in `App.tsx:2647` footer (short variant)" (`C_ui_constraints.md:283`) | Same correction: `{PROJ}/apps/desktop/src/App.tsx:3282`. The co-cited `features/rule-check/RuleCheckPanel.tsx:99` verifies exactly | `{PROJ}/apps/desktop/src/App.tsx:3282`; `{PROJ}/apps/desktop/src/features/rule-check/RuleCheckPanel.tsx:99` |
| D-3 | M-03 is "emitted by the report renderer (`core/reporting/report_renderer/src/lib.rs`)", no line (`C_ui_constraints.md:284`) | Emitted at `{PROJ}/core/reporting/report_renderer/src/lib.rs:718-727` as `SectionBlock::BoundaryBox`, text at `:719-725`. Written with `\` line continuations, so a single-line grep for the first clause does not find it | `{PROJ}/core/reporting/report_renderer/src/lib.rs:719-725` |
| D-4 | M-13 requires the designation "with the stated limit that *reopening cannot establish a current solve basis*" (`C_ui_constraints.md:294`; C-66 at `:113`) | That sentence is a **source comment**, `{PROJ}/apps/desktop/src/features/results/HistoricalRunContext.tsx:8-9`, not rendered text. The rendered text is the panel title "Historical saved run" (`:321`) plus two sentences at `:322-323` that do not contain the quoted limit | `{PROJ}/apps/desktop/src/features/results/HistoricalRunContext.tsx:8-9`, `:320-323` |
| D-5 | C §3 item 23 lists spring-hanger selection as **Tier 3, accepted net-new** (`C_ui_constraints.md:262`); C-51 cites `LibraryKind` = Material \| Section \| Component at `library_import_document/src/lib.rs:50-53` and calls a hanger kind "a governed extension" (`:88`) | `LibraryKind` now has a fourth variant `Hanger` at `{PROJ}/core/library_import/library_import_document/src/lib.rs:54`; `{PROJ}/schemas/hanger.schema.yaml` exists; `{PROJ}/core/library_import/provenance_checker.py:35,107-108,355-393` dispatches and validates it; and a `HangerSelectionPanel` ships at `{PROJ}/apps/desktop/src/features/hanger-selection/HangerSelectionPanel.tsx`, wired at `{PROJ}/apps/desktop/src/App.tsx:3,2850`. The item is at least partly landed, not purely net-new | `{PROJ}/core/library_import/library_import_document/src/lib.rs:50-55`; `{PROJ}/schemas/hanger.schema.yaml:1-27` |
| D-6 | C-19/M-09 present the evidence labels as attached to results today (`C_ui_constraints.md:41`, `:290`) | `INTERNALLY_VERIFIED`, `PROVER_CORRELATED` and `ENGINEER_ACCEPTED` have **zero** occurrences in `{PROJ}/apps`, `{PROJ}/core`, `{PROJ}/schemas`, `{PROJ}/fixtures`. They exist only in `{PROJ}/docs/claims_registry.md:89-92`, `{PROJ}/docs/validation_manual/headless_runner_reproduction.md:255` and eight `{PROJ}/validation/evidence/reproduction/**` bundle files | see §4.3 |
| D-7 | C-09 preserves "Human review required" as a status concept (`C_ui_constraints.md:31`) | The product already renders curated short labels for two of the six tokens, and they are not the ones the design proposes: `readableWorkspaceStatus` (`{PROJ}/apps/desktop/src/App.tsx:3766-3773`) maps `HUMAN_REVIEW_REQUIRED` to **"Review required"** (`:3768`) and `RULE_INPUTS_INCOMPLETE` to **"Inputs needed"** (`:3769`), and the other four tokens fall through to a lowercased, de-underscored form (`:3772`). The same de-underscored form is produced independently by `formatStatus`, defined twice at `{PROJ}/apps/desktop/src/features/solve/SolvePanel.tsx:189-191` and `{PROJ}/apps/desktop/src/features/viewport/PipeViewport.tsx:4518-4520`. It differs from the design's proposed labels in case, and for `USER_RULE_CHECKED` also in number ("user rule checked" against the proposed "User rules checked"). One surface renders the raw token into prose (`{PROJ}/apps/desktop/src/features/report/renderableReportInput.ts:318`), and the status pill shows the raw token beneath its label (`App.tsx:3761`). Separately, the phrase "human review required" is hand-written prose in five non-test desktop files | see §4.3 |
| D-8 | The brief's starting anchor list names `BS-MATURITY` and `BS-ACCEPT` as lint anchors | These are **registry statement IDs**, not lint finding codes. The three lint finding codes are `AD_HOC_CLAIMS_LITANY`, and the three anchor codes `MISSING_PRD_NOTICE` / `MISSING_MATURITY_BANNER` / `MISSING_RENDERER_NOTICE` (`tools/validation/validate_claims_language.py:96-113`). C-15 at `C_ui_constraints.md:282` names `MISSING_MATURITY_BANNER` correctly | `tools/validation/validate_claims_language.py:96-113` |
| D-9 | C-20 lists the litany vocabulary as nine terms with `code compliance` once (`C_ui_constraints.md:42`) | The source has eleven terms, including both `"code-compliance"` and `"code compliance"` as separate entries, at `tools/validation/validate_claims_language.py:35-47`. C-20's statement that the scan "covers all non-test `.ts`/`.tsx` under `apps/desktop/src`" is correct but partial: the scan set is desktop sources **plus** `{PROJ}/docs/**/*.md` minus exclusions **plus** `{PROJ}/execution/PKG-*/1_Working/DEL-*/ScopeOfWork.md` (`:253-260`) | `tools/validation/validate_claims_language.py:35-47`, `:196-260` |

One further observation, not a difference but a consequence of the exclusion
list: `{PROJ}/docs/PRD.md:127` carries the heading "Full Analytical Engine,
Non-Authoritative Result" and `:131` reads "internal results are not
authoritative for professional reliance". Both would match the lint's
`RETIRED_PHRASE` rule (`tools/validation/validate_claims_language.py:50`,
`:284-290`), but `PRD.md` is allow-listed at `:118`, so the lint does not see
them. This is the PRD's own authority text, not a product surface.

---

## Item 1 — the maturity line

### 1.1 Governed text, verbatim

Registry entry `BS-MATURITY`, `{PROJ}/docs/claims_registry.md:67-74`:

> **Canonical:** "Technical preview — not a released product." (One sentence,
> derived from the ruled stage record; updated only when the stage record
> changes. Never a compound litany.)
>
> **Belongs on:** the app shell banner/footer; packaging/build-readiness
> surfaces may reuse it.

- **Registry id:** `BS-MATURITY` (`{PROJ}/docs/claims_registry.md:67`)
- **Canonical text:** `{PROJ}/docs/claims_registry.md:69`
- **Listed short variants:** none. This is the only one of the five registry
  statements with no short-variant list (`{PROJ}/docs/claims_registry.md:67-74`,
  compare `:25-26`, `:38-44`, `:59-62`).
- **Bound surfaces:** app shell banner/footer; packaging/build-readiness
  surfaces may reuse it (`{PROJ}/docs/claims_registry.md:72-74`).
- **Authority:** D-48 Option O-A, codified `DEC-081`
  (`{PROJ}/docs/claims_registry.md:3-10`; `{PROJ}/execution/_Decomposition/SOFTWARE_DECOMP.md:672`).
  The em dash in the canonical text is `—` (U+2014).

### 1.2 What enforces it — 7 points

| # | Enforcement point | File and line | What exactly it checks |
|---|---|---|---|
| 1 | Registry entry | `{PROJ}/docs/claims_registry.md:67-74` | The canonical text and its placement rule; "Never a compound litany" |
| 2 | Lint anchor `MISSING_MATURITY_BANNER` | `tools/validation/validate_claims_language.py:103-107`, checked at `:294-316` | That `{PROJ}/apps/desktop/src/App.tsx`, read whole and whitespace-normalised, contains the exact sentence `Technical preview — not a released product.` Absence of the file, or of the sentence, is a finding |
| 3 | Lint registered-text suppression | `tools/validation/validate_claims_language.py:86` | The sentence is one of the 14 `REGISTERED_TEXTS`, so a line carrying it is exempt from the litany rule (`:179-189`) |
| 4 | Lint test `test_missing_maturity_banner_fires` | `tools/validation/test_validate_claims_language.py:135-138` | Replacing `App.tsx` with content lacking the sentence produces exactly `["MISSING_MATURITY_BANNER"]` |
| 5 | Lint test fixture | `tools/validation/test_validate_claims_language.py:12`, `:35` | The literal `MATURITY` constant is written into the synthetic `App.tsx` of every clean-tree case; every other test in the file depends on it |
| 6 | Vitest `does not claim professional or release acceptance` | `{PROJ}/apps/desktop/src/App.test.tsx:7136-7147` | `findAllByText(/Technical preview — not a released product/i)` renders, and `footer.app-footer` textContent contains both the maturity sentence (`:7141-7143`) and the BS-ACCEPT short variant (`:7144-7146`) |
| 7 | Self-check registration GEN-13 | `tools/practitioner_harness/cmd_self_check.py:923`; `{PROJ}/docs/claims_registry.md:12-13` | The validator runs as self-check `GEN-13`; "violations block closeout" |

No Playwright e2e spec asserts the banner: `{PROJ}/apps/desktop/e2e/` contains
no match for `Technical preview` or `app-footer`.

### 1.3 Where it appears today — 128 lines

| Class | Count | Detail |
|---|---|---|
| Registered placement | 3 | `{PROJ}/apps/desktop/src/App.tsx:3282` (app-shell footer, the lint anchor's target); `tools/validation/validate_claims_language.py:86` (registered text) and `:106` (anchor fragment) |
| Registered placement — permitted reuse | 1 | `{PROJ}/apps/desktop/src/features/build-readiness/BuildReadinessPanel.tsx:74-75`, a build-readiness surface, which `{PROJ}/docs/claims_registry.md:72-74` permits to reuse the sentence |
| Registry definition | 1 | `{PROJ}/docs/claims_registry.md:69` |
| Ordinary string | 4 | `{PROJ}/apps/desktop/src/features/export-review/ExportReviewPanel.tsx:658` (inside a `review_note` string in an export-review packet payload, not a rendered shell banner); `{PROJ}/apps/desktop/src/App.test.tsx:7139`, `:7142`; `tools/validation/test_validate_claims_language.py:12` |
| Historical record — must not change | 119 | 106 lines under `{PROJ}/execution/_Coordination/AgentRuns/**`; 12 under `{PROJ}/execution/_Evaluation/PHYSICS_AUDIT_2026-09-05/**` (before/after source snapshots); 1 at `{PROJ}/execution/_Coordination/_DECISIONS/D-48_claims_language_taxonomy.md:82` |

Note that `App.tsx:3282` is one line carrying **two** registered statements: the
maturity sentence and the BS-ACCEPT short variant, in that order.

### 1.4 Where the redesign puts it

- **Design system:** `{RUN}/instances/DESIGN-SYSTEM/DESIGN_SYSTEM_V1.md §7.4` —
  "M-01 | About; until packet item 1 is ruled, the status bar's information
  popover carries the single sentence."
- **Frames:** one frame only,
  `{RUN}/instances/MOCKS/frames/s1_table_light.html`, inside the status-bar
  information popover, next to a row reading "About SWB Piping Designer…".
  Fourteen of the fifteen frames do not carry it.
- **Why it is asked:** `{RUN}/instances/ROOT/DESIGN_BRIEF_V1.md:168`;
  `{RUN}/instances/CONCEPTS/CONCEPT_DIRECTIONS_V1.md:315`.

### 1.5 What a change requires

Two distinct acts, depending on which way the change goes.

- **Relocating it** (shell banner → About plus a status-bar popover) touches the
  registry's placement clause `{PROJ}/docs/claims_registry.md:72-74` and the
  lint anchor's file target `tools/validation/validate_claims_language.py:105`.
  The registry is ruled vocabulary under `DEC-081`
  (`{PROJ}/docs/claims_registry.md:3-4`); changing ruled vocabulary is a
  decision act, recorded as a new `DEC` row in
  `{PROJ}/execution/_Decomposition/SOFTWARE_DECOMP.md` §12 and a register row in
  `{PROJ}/execution/_Coordination/_DECISIONS/_REGISTER.md`
  (`:6-15`: "agents prepare packets labeled `PROPOSAL`; only the human project
  authority rules"; packets live at
  `{PROJ}/execution/_Coordination/_DECISIONS/D-XX_<slug>.md`, `:12`).
- **Changing the sentence itself.** The registry says it is "derived from the
  ruled stage record; updated only when the stage record changes"
  (`{PROJ}/docs/claims_registry.md:69-71`). The stage record is
  `_COORDINATION.md`'s current target stage, advanced only by an owner stage
  ruling — the pattern of `D-14`/`DEC-029`, `D-23`/`DEC-048`,
  `D-26`/`DEC-053`, `D-27`/`DEC-054`
  (`{PROJ}/execution/_Coordination/_DECISIONS/_REGISTER.md:46,56,59,60`).
  `F-PIP-3` bars lifecycle promotion without the owner's gate
  (`C_ui_constraints.md:58` citing `{PROJ}/loop/WORKPLAN_2026-07-18b_piping_loop.md`).
- **D-48's own precedent** for registry work: `§7 On-Ruling Mechanism`,
  `{PROJ}/execution/_Coordination/_DECISIONS/D-48_claims_language_taxonomy.md:198-216`
  — append the ruling to the packet, flip the register row, codify as the next
  free `DEC-XXX` row, then execute as governed loop work, *not* an SCA, because
  no PRD text or scope item changes. Branch-first, PR per wave.
- **Deliverable home:** the registry and lint are homed under `DEL-16-04`
  (`{PROJ}/execution/PKG-16_Model Operation and Agent Proposal Framework/1_Working/DEL-16-04_Agent rationale and professional-boundary controls/_STATUS.md:12`),
  which is `IN_PROGRESS` (`:3`).
- **Evidence obligation:** a change to `App.tsx` is a code-touching branch, so
  the `DEC-025` five-surface sweep applies, plus repo-wide `self-check` exit 0
  including GEN-13, plus the H4 desktop evidence posture (an e2e spec extension
  by default) — `C_ui_constraints.md:51,54`; `DEC-093` requires both registered
  viewport projects (`C_ui_constraints.md:55`).
- **Notice obligations:** none found outside this project loop. `BS-MATURITY`
  appears in no other project's instruction corpus in this repository.

### 1.6 Unknowns

- **TBD** — whether the owner treats "the app shell banner/footer" as satisfied
  by an on-demand popover, or requires the registry clause revised. Settled by
  the owner's ruling on packet item 1; the question is posed at
  `{RUN}/instances/CONCEPTS/CONCEPT_DIRECTIONS_V1.md:315`.
- **TBD** — `PB-TBD-003` "Release-label vocabulary and final release policy
  language", owner "Human project authority"
  (`{PROJ}/docs/PROFESSIONAL_BOUNDARY.md:174`). Any replacement maturity
  wording would sit inside this open TBD. Settled by an owner record of release-
  label vocabulary.

---

## Item 2 — the acceptance sentence and its placements

### 2.1 Governed text, verbatim

Registry entry `BS-ACCEPT`, `{PROJ}/docs/claims_registry.md:31-50`:

> **Canonical:** "Results are engineering decision-support information.
> Acceptance, professional judgment, and any certification, sealing, or
> code-compliance determination remain with the responsible engineer and
> project authority."

- **Registry id:** `BS-ACCEPT` (`{PROJ}/docs/claims_registry.md:31`)
- **Listed short variants**, four, at `{PROJ}/docs/claims_registry.md:38-44`:
  1. the canonical second sentence standalone — "Acceptance, professional
     judgment, and any certification, sealing, or code-compliance determination
     remain with the responsible engineer and project authority."
  2. "acceptance and professional judgment remain with the responsible engineer"
  3. "human review remains required; acceptance stays with the responsible
     engineer"
  4. "decision-support information for review by the responsible engineer"
- **Bound surfaces:** "results, rule-check, comparison, solve, report-preview
  surfaces; any place a computed outcome is shown"
  (`{PROJ}/docs/claims_registry.md:46-47`).
- **Retires clause:** "Retires: 'not authoritative'-family phrasing and
  multi-noun prohibition litanies on product surfaces — BS-ACCEPT states who
  holds judgment instead of demoting the output"
  (`{PROJ}/docs/claims_registry.md:47-50`).
- **Policy depth:** `{PROJ}/docs/PROFESSIONAL_BOUNDARY.md` is the anchor doc
  (`{PROJ}/docs/claims_registry.md:111-113`; `{PROJ}/docs/PROFESSIONAL_BOUNDARY.md:16-17`).

### 2.2 What enforces it — 14 points

| # | Enforcement point | File and line | What exactly it checks |
|---|---|---|---|
| 1 | Registry entry | `{PROJ}/docs/claims_registry.md:31-50` | Canonical text, four short variants, bound surfaces, retires clause |
| 2 | Lint `REGISTERED_TEXTS` | `tools/validation/validate_claims_language.py:65-77` | Five entries (canonical + 4 variants), whitespace-unwrapped, compared casefolded |
| 3 | Lint suppression window | `tools/validation/validate_claims_language.py:93`, `:179-189` | A line whose own decoration-stripped content is ≥ 15 consecutive characters of a registered text is exempt, so wrapped multi-line statements pass |
| 4 | Lint `AD_HOC_CLAIMS_LITANY` | `tools/validation/validate_claims_language.py:35-48`, `:270-283` | 3+ distinct litany terms on one line without a registered text is a finding. Exempt: lines citing `§21.2`/`section 21.2` (`:151`, `:274`) and lines carrying `"software certifies"` (`:155`, `:275`) |
| 5 | Lint `RETIRED_PHRASE` | `tools/validation/validate_claims_language.py:50`, `:284-290` | Any occurrence of `not authoritative`, case-insensitive, on a scanned surface is a finding |
| 6 | Lint scan set | `tools/validation/validate_claims_language.py:196-260` | Non-test `.ts`/`.tsx` under `{PROJ}/apps/desktop/src`; `{PROJ}/docs/**/*.md` minus `:117-128` exclusions; all `{PROJ}/execution/PKG-*/1_Working/DEL-*/ScopeOfWork.md` |
| 7 | Lint test `test_registered_canonical_text_is_not_flagged` | `tools/validation/test_validate_claims_language.py:69-72` | The canonical text in a user-guide doc produces no findings |
| 8 | Lint test `test_wrapped_registered_text_line_is_not_flagged` | `tools/validation/test_validate_claims_language.py:75-85` | The wrapped fragment `professional judgment, and any certification, sealing, or` is suppressed |
| 9 | Lint test `test_not_authoritative_is_flagged` | `tools/validation/test_validate_claims_language.py:88-91` | `Output is Not Authoritative.` yields `AD_HOC_CLAIMS_LITANY` |
| 10 | Lint test `test_desktop_source_litany_is_flagged` | `tools/validation/test_validate_claims_language.py:118-126` | A four-term disclaimer in a feature `.tsx` yields `AD_HOC_CLAIMS_LITANY` |
| 11 | Doc test `test_user_guide_lists_external_human_approval_status_without_software_claim` | `{PROJ}/tests/test_user_guide_status_wording.py:11-25` | `{PROJ}/docs/user_guide/index.md` contains the standalone second-sentence variant, lowercased, in the `HUMAN_APPROVED_FOR_PROJECT` row |
| 12 | Rust constant `PROFESSIONAL_BOUNDARY_NOTICE` | `{PROJ}/core/rules/rule_check_runner/src/lib.rs:78-81` | Fixes the rule-check notice text: "Rule-check results are engineering decision-support information computed from user-supplied rules and data; acceptance and professional judgment remain with the responsible engineer. Human review remains required." |
| 13 | Schema `const` on the rule-check notice | `{PROJ}/schemas/rule_check_run_result.schema.json:14`, `:37`; asserted by `{PROJ}/tests/test_operation_result_schemas.py:169-175` | `professional_boundary_notice` is a `const` equal to the runner's notice; any wording change breaks the schema and the test |
| 14 | Rust and Vitest assertions on the emitted notice | `{PROJ}/apps/desktop/src-tauri/src/lib.rs:7003`; `{PROJ}/apps/desktop/src/App.test.tsx:1241,7145,7255,10175,11350,11390,12039,12920,13282`; `{PROJ}/apps/desktop/src/features/library/LibraryManagerPanel.test.tsx:318`; `{PROJ}/apps/desktop/src/features/rule-check/RuleCheckRunPanel.test.tsx:336,358,372,421`; `{PROJ}/apps/desktop/src/features/rule-packs/RulePackManagerPanel.test.tsx:135`; `{PROJ}/core/solver/nonlinear_supports/src/lib.rs:1380` | Fifteen test lines assert one of the variants is present in rendered output or an emitted record |

GEN-13 registration (`tools/practitioner_harness/cmd_self_check.py:923`) covers
points 2-6 as one gate, as for item 1.

### 2.3 Where it appears today — 609 lines, of which 102 live

| Variant | Total | Historical | Live |
|---|---:|---:|---:|
| Canonical, first sentence "Results are engineering decision-support information" | 18 | 14 | 4 |
| Short variant 1, standalone second sentence | 21 | 13 | 8 |
| Short variant 2, "acceptance and professional judgment remain with the responsible engineer" | 491 | 414 | 77 |
| Short variant 3, "human review remains required; acceptance stays with the responsible engineer" | 76 | 66 | 10 |
| Short variant 4, "decision-support information for review by the responsible engineer" | 3 | 0 | 3 |

**Registered placements — 53 lines in 21 product-source files** (non-test
`{PROJ}/apps/**`, `{PROJ}/core/**`, `{PROJ}/api/**`): 46 lines of variant 2 and
7 lines of variant 3.

| File | Lines |
|---|---|
| `{PROJ}/apps/desktop/src/App.tsx` | `3282` (app-shell footer) |
| `{PROJ}/apps/desktop/src/features/rule-check/RuleCheckPanel.tsx` | `99` |
| `{PROJ}/apps/desktop/src/features/results/ResultsPanel.tsx` | `192` |
| `{PROJ}/apps/desktop/src/features/results/resultInterpretation.ts` | `63`, `114` |
| `{PROJ}/apps/desktop/src/features/comparison/ComparisonPanel.tsx` | `131` |
| `{PROJ}/apps/desktop/src/features/solve/SolvePanel.tsx` | `184` |
| `{PROJ}/apps/desktop/src/features/report/ReportPanel.tsx` | `264`, `604`, `627`, `1008` |
| `{PROJ}/apps/desktop/src/features/report-lint/ReportLintPanel.tsx` | `522`, `529`, `536`, `543`, `550`, `634`, `641`, `648`, `655`, `662`, `669`, `676`, `683`, `690`, `697`, `711`, `718`, `725`, `732`, `739` (20 lines: the report-template expectation corpus) |
| `{PROJ}/apps/desktop/src/features/diff-preview/DiffPreviewPanel.tsx` | `110`, `345` |
| `{PROJ}/apps/desktop/src/features/operations/OperationLedgerPanel.tsx` | `436` |
| `{PROJ}/apps/desktop/src/features/missing-data/MissingDataBlockingPanel.tsx` | `115`, `539` |
| `{PROJ}/apps/desktop/src/features/project-storage/ProjectStorageAuditPanel.tsx` | `142`, `287` |
| `{PROJ}/apps/desktop/src/features/project-validation/ProjectValidationPanel.tsx` | `197`, `787`, `853`, `881` |
| `{PROJ}/apps/desktop/src/features/run-audit/RunAuditPanel.tsx` | `78`, `111` |
| `{PROJ}/apps/desktop/src/features/validation-evidence/ValidationEvidencePanel.tsx` | `120` |
| `{PROJ}/apps/desktop/src/features/export-review/ExportReviewPanel.tsx` | `1095` |
| `{PROJ}/apps/desktop/src/features/result-export/ResultExportPanel.tsx` | `128` |
| `{PROJ}/apps/desktop/src/services/rulePackService.ts` | `241`, `365` |
| `{PROJ}/apps/desktop/src-tauri/src/lib.rs` | `7003` (a test assertion inside the Rust crate) |
| `{PROJ}/core/solver/nonlinear_supports/src/lib.rs` | `343`, `1380` |
| `{PROJ}/core/solver/nonlinear_integration/src/lib.rs` | `671` |

**Registered placements — documents:** `{PROJ}/docs/claims_registry.md:33`
(definition), `{PROJ}/docs/user_guide/index.md:43` (canonical) and `:186`
(standalone second sentence, in the `HUMAN_APPROVED_FOR_PROJECT` row),
`tools/validation/validate_claims_language.py:66-77`.

**Live governance surfaces (`SOW`):** 26 lines across ScopeOfWork files —
7 carrying the standalone second sentence, 18 carrying variant 2, 1 carrying
variant 4 — in `DEL-01-04`, `DEL-02-02`, `DEL-02-04`, `DEL-02-05`, `DEL-03-08`,
`DEL-06-01`, `DEL-06-03`, `DEL-06-05`, `DEL-07-04`, `DEL-07-05`, `DEL-07-06`,
`DEL-07-08`, `DEL-08-01`, `DEL-08-02`, `DEL-08-03`, `DEL-08-04`, `DEL-10-03`,
`DEL-11-03`, `DEL-12-01`, `DEL-12-02`, `DEL-12-05`, `DEL-13-02`, `DEL-16-02`,
`DEL-16-03`. These are the D-48 Wave-2 surfaces
(`tools/validation/validate_claims_language.py:130-136`).

**Ordinary strings:** 15 test lines (§2.2 point 14), plus
`{PROJ}/README.md:39`.

**Historical records — must not change:** 507 lines, overwhelmingly under
`{PROJ}/execution/_Coordination/AgentRuns/**` and
`{PROJ}/execution/_Evaluation/**`.

### 2.4 Where the redesign puts it

- **Rule:** "One placement per surface class"
  (`{RUN}/instances/DESIGN-SYSTEM/DESIGN_SYSTEM_V1.md §7.4`): the results
  header's information disclosure for the results surface class, once at the top
  of the Review page, and in the report; explicitly *not* on the Model or Loads
  pages, in the inspector, the probe, the canvas or the agent panel.
- **Disclosure-home table:** `{RUN}/instances/DESIGN-SYSTEM/DESIGN_SYSTEM_V1.md §7.4`.
- **Probe card explicitly does not repeat it:**
  `{RUN}/instances/DESIGN-SYSTEM/DESIGN_SYSTEM_V1.md §5.6`.
- **Review page placement:** `{RUN}/instances/DESIGN-SYSTEM/DESIGN_SYSTEM_V1.md §5.5`
  — "once, at the top of the content column … It does not repeat in any section."
- **Frames:** four of fifteen carry it, always the full canonical text:
  `{RUN}/instances/MOCKS/frames/s7_both_light.html`,
  `s7_both_dark.html` (results header disclosure, class `disc`), and
  `s9_table_light.html`, `s9_table_dark.html` (Review page top, class `accept`).
- **Why it is asked:** `{RUN}/instances/ROOT/DESIGN_BRIEF_V1.md:169` names the
  contrast — "one placement per surface class … rather than the current fifty";
  `{RUN}/instances/CONCEPTS/CONCEPT_DIRECTIONS_V1.md:316`;
  `{RUN}/instances/ROOT/DIRECTION_DECISION_2026-09-17.md:74`.

### 2.5 What a change requires

- A **placement change alone** does not touch the canonical text. The clause it
  touches is `{PROJ}/docs/claims_registry.md:46-47` ("any place a computed
  outcome is shown"), which is ruled vocabulary under `DEC-081` — so the same
  decision-act mechanics as item 1 §1.5: packet at
  `{PROJ}/execution/_Coordination/_DECISIONS/D-XX_<slug>.md`, register row in
  `{PROJ}/execution/_Coordination/_DECISIONS/_REGISTER.md`, codified as a `DEC`
  row in `{PROJ}/execution/_Decomposition/SOFTWARE_DECOMP.md` §12.
- No PRD amendment is implied. D-48 §3.4 records that the registry "derives from
  the PRD, and §21 stays the claims authority" and that "No PRD text change is
  required" (`{PROJ}/execution/_Coordination/_DECISIONS/D-48_claims_language_taxonomy.md:117-121`).
  The constraint carried by every D-48 option is that PRD §19.3 and §21.2 "are
  anchors: the alignment may relocate and deduplicate them, never weaken them"
  (`:180-184`).
- The **underlying requirement** that survives any placement change is PRD §5.9,
  "Reports, UI labels, agent output, examples, and documentation shall preserve
  the distinction between software-computed results and professional engineering
  acceptance" (`C_ui_constraints.md:35`), and `OPS-K-MECH-2`
  (`{PROJ}/docs/CONTRACT.md:32`).
- **Execution consequence if the count drops from 53 to a handful:** each
  removal is a code-touching edit to a file with a Vitest or Rust assertion
  (§2.2 point 14), so the tests move with it. D-48 §9 records that "any string
  change that alters behavior beyond text … returns to the owner instead of
  being absorbed"
  (`{PROJ}/execution/_Coordination/_DECISIONS/D-48_claims_language_taxonomy.md:235-239`).
- **Evidence obligation:** as item 1 §1.5 — `DEC-025` five-surface sweep,
  repo-wide self-check exit 0 including GEN-13, H4 e2e extension by default,
  `DEC-093` both viewport projects.
- **Notice obligations:** the 26 live `ScopeOfWork.md` lines are Wave-2 surfaces
  of the same `DEC-081` act; a placement change that narrows product surfaces
  does not by itself touch them, since the registry's placement rule for
  governance artifacts is `GF-TOKEN`, not `BS-ACCEPT`
  (`{PROJ}/docs/claims_registry.md:76-85`). No cross-project notice found.

### 2.6 Unknowns

- **TBD** — whether an information disclosure that must be opened counts as "the
  surface carrying the sentence". Posed at
  `{RUN}/instances/CONCEPTS/CONCEPT_DIRECTIONS_V1.md:316`. Settled by the owner's
  ruling on packet item 2.
- **TBD** — `PB-TBD-005`, "Final acceptance or revision of this draft policy and
  report notice template", owner "Human project authority"
  (`{PROJ}/docs/PROFESSIONAL_BOUNDARY.md:176`). Both
  `{PROJ}/docs/PROFESSIONAL_BOUNDARY.md` and `{PROJ}/docs/CONTRACT.md` carry
  `status: draft` in front matter (`:4` in each), noted in
  `C_ui_constraints.md:10-11`.

---

## Item 3 — the product name, and the rename

### 3.1 Governed text, verbatim

The name is load-bearing in four governed texts and one policy target.

**(a) `BS-IP` canonical**, `{PROJ}/docs/claims_registry.md:19-21`:

> "OpenPipeStress ships no protected standards content. All code-specific
> values, tables, allowables, and factors are supplied by the user or
> user-controlled private sources, with provenance recorded."

Registry id `BS-IP` (`:17`); two short variants at `:23-26`, **neither of which
contains the name**; belongs on "import/library/contribution UI,
redaction/export surfaces, contribution docs", not on results views unless the
surface genuinely handles imported content (`:27-29`).

**(b) PRD §19.3 required notice**, `{PROJ}/docs/PRD.md:1244-1250`, quoted here
from its template home `{PROJ}/docs/report_notice_template.md:40-49`:

> OpenPipeStress is decision-support software for piping design, flexibility,
> and stress-analysis workflows. It computes mechanical results from recorded
> user inputs and may evaluate user-supplied rule packs. It does not certify,
> seal, approve, authenticate, or determine code compliance for professional
> reliance. Code-specific and project-specific data are supplied by the user or
> user-controlled private sources. Competent human review and, where required,
> validation in accepted professional tools remain the responsibility of the
> user and project authority.

The PRD's own framing is "a notice substantially equivalent to" (`{PROJ}/docs/PRD.md:1246`);
the template's is "quoted verbatim (changes require a PRD-level owner act)"
(`{PROJ}/docs/report_notice_template.md:37-38`); the registry's is "the required
composite for generated reports … Changes require a PRD-level owner act"
(`{PROJ}/docs/claims_registry.md:106-109`).

**(c) M-04 supplement**, `{PROJ}/docs/report_notice_template.md:55-59` — does
**not** contain the name.

**(d) `PROFESSIONAL_BOUNDARY` §1 and §8**, `{PROJ}/docs/PROFESSIONAL_BOUNDARY.md:22`,
`:24`, `:62`, `:83`, `:118`, `:136`; `IP_AND_DATA_BOUNDARY` §1
(`{PROJ}/docs/IP_AND_DATA_BOUNDARY.md:18`) — the name in policy prose.

**(e) `DEC-089` (D-06b)** — App ID description "**OpenPipeStress Technical
Preview**", bundle ID `org.openpipestress.technical-preview`
(`C_ui_constraints.md:84`), realised at
`{PROJ}/apps/desktop/src-tauri/tauri.conf.json:3`, `:5`, `:15`.

### 3.2 What enforces it — 13 points

| # | Enforcement point | File and line | What exactly it checks |
|---|---|---|---|
| 1 | `BS-IP` registry entry | `{PROJ}/docs/claims_registry.md:17-29` | Canonical text carrying the name; two name-free short variants; placement |
| 2 | Lint registered text for `BS-IP` | `tools/validation/validate_claims_language.py:59-61` | The canonical text, name included, is a `REGISTERED_TEXTS` entry — the only lint constant containing the name |
| 3 | PRD §19.3 | `{PROJ}/docs/PRD.md:1244-1250`, repeated in the PRD appendix at `:1786` | The required notice text |
| 4 | Lint anchor `MISSING_PRD_NOTICE` | `tools/validation/validate_claims_language.py:97-102` | `{PROJ}/docs/PRD.md` contains the fragment `does not certify, seal, approve, authenticate, or determine code compliance for professional reliance` (this fragment does **not** contain the name) |
| 5 | Lint anchor `MISSING_RENDERER_NOTICE` | `tools/validation/validate_claims_language.py:108-112` | `{PROJ}/core/reporting/report_renderer/src/lib.rs` contains the fragment `decision-support software` (this fragment does **not** contain the name) |
| 6 | Lint registered text, PRD fragment | `tools/validation/validate_claims_language.py:89-91` | The §19.3 fragment is a `REGISTERED_TEXTS` entry |
| 7 | Renderer emission | `{PROJ}/core/reporting/report_renderer/src/lib.rs:704-727`, text at `:719-725`; section key registered at `:41` in `SECTION_ORDER` | The `professional_boundary_notice` section emits the full §19.3 composite, with the name as its first word, as a `BoundaryBox` |
| 8 | Template home | `{PROJ}/docs/report_notice_template.md:37-49`; metadata slot "Software version — OpenPipeStress version or commit basis" at `:68` | The verbatim notice, plus the rule that changes require a PRD-level owner act |
| 9 | Lint tests for anchors 4 and 5 | `tools/validation/test_validate_claims_language.py:8-13`, `:129-132`, `:141-144` | Removing the PRD fragment yields `MISSING_PRD_NOTICE`; deleting the renderer file yields `MISSING_RENDERER_NOTICE` |
| 10 | Schema `$id` namespace | 46 lines in 46 files under `{PROJ}/schemas/` and `{PROJ}/api/`, e.g. `{PROJ}/schemas/analysis_status.schema.yaml:3` `https://openpipestress.org/schemas/analysis_status.schema.yaml`; `{PROJ}/schemas/hanger.schema.yaml:3` | Every schema's identity URL carries the lowercase name |
| 11 | `document_kind` / `const` tokens | 61 distinct `openpipestress.*` string constants across `{PROJ}/schemas`, `{PROJ}/core`, `{PROJ}/apps`, `{PROJ}/fixtures`, `{PROJ}/api`, e.g. `{PROJ}/schemas/rule_check_run_result.schema.json` `openpipestress.rule_check.run` asserted at `{PROJ}/tests/test_operation_result_schemas.py:172` | These are closed data-contract vocabularies asserted by `const` and by tests; they are the interoperability identity of every emitted document |
| 12 | Packaging identity | `{PROJ}/apps/desktop/src-tauri/tauri.conf.json:3`, `:5`, `:15`; `{PROJ}/apps/desktop/index.html:10`; `{PROJ}/package.json:2` (`openpipestress-workspace`); `{PROJ}/apps/desktop/package.json:2` (`@openpipestress/desktop`); Cargo crate names `openpipestress-runner`, `openpipestress_jcs_ijson`, `openpipestress-desktop`, `openpipestress_desktop_lib` (`{PROJ}/core/runner/headless/Cargo.toml:12`, `{PROJ}/core/serialization/canonical_json/Cargo.toml:15`, `{PROJ}/apps/desktop/src-tauri/Cargo.toml:2`, `:9`); binary `{PROJ}/core/runner/headless/src/bin/openpipestress-runner.rs` | Product name, bundle identifier, window title, workspace and package names, crate names, binary name |
| 13 | Document titles | `{PROJ}/docs/DIRECTIVE.md:15`, `{PROJ}/docs/CONTRACT.md:13`, `{PROJ}/docs/PRD.md:1`, `{PROJ}/docs/README.md:8`, `{PROJ}/docs/PLAN.md:8`, `{PROJ}/docs/SPEC.md:15`, `{PROJ}/docs/TYPES.md:11` | The seven governance H1 titles carry the name |

### 3.3 Where it appears today

**"OpenPipeStress"** (case-sensitive): **101,493 lines in 2,079 files**
repo-wide. **1,873 lines in 693 files** are live; 99,620 lines in 1,386 files
are historical records that must not change.

| Class | Files | Detail |
|---|---:|---|
| Registered placement / product source | 188 | `{PROJ}/apps`, `{PROJ}/core`, `{PROJ}/schemas`, `{PROJ}/api`, `{PROJ}/fixtures`. Highest concentrations: `{PROJ}/fixtures/component/invented_section_component_library_valid.json` (68 lines), `{PROJ}/fixtures/component/invented_component_library_valid.json` (58), `{PROJ}/fixtures/persistence/invented_persisted_preview_project.json` (30), `{PROJ}/fixtures/caepipe_mbf/invented/caepipe_mbf_export_package.json` (25), `{PROJ}/apps/desktop/src/features/review-geometry/ReviewGeometryPanel.tsx` (13), `{PROJ}/apps/desktop/src-tauri/src/lib.rs` (11) |
| Registered placement / docs | 94 | including all seven governance titles |
| Live governance surfaces | 56 | `ScopeOfWork.md` files |
| Other live execution surfaces | 152 | `_STATUS.md`, `_CONTEXT.md`, `MEMORY.md`, `_DAG`, `_Change` |
| Ordinary string | 24 test files + 166 other | `{PROJ}/validation/hand_calcs` (69 files), `{PROJ}/validation/evidence` (37), `{PROJ}/validation/witness` (29), `{PROJ}/tools/release` (9), `{PROJ}/validation/benchmarks` (7), `{PROJ}/.github/ISSUE_TEMPLATE` (5), `{PROJ}/examples/**`, `{PROJ}/governance/MAINTAINERS.md`, `{PROJ}/governance/CONTRIBUTOR_CERTIFICATION_TEMPLATE.md`, `{PROJ}/README.md`, `{PROJ}/LICENSE.md`, `{PROJ}/CONTRIBUTING.md`, `{PROJ}/AGENTS.md`, `{PROJ}/apps/desktop/SMOKE.md` |
| Root tools | 13 | `tools/validation/**`, `tools/**` |
| Historical record — must not change | 1,386 | `AgentRuns/**`, `_run_records/**`, `_DECISIONS/**`, `_ScopeChange/**`, `_Reconciliation/**`, `_Evaluation/**`, `docs/_history/**`, `plans/**`, `provenance/**`, `LOOP_RECEIPTS.md` |

**Lowercase `openpipestress`** (identifiers, bundle IDs, URLs, crate and file
names): **87,067 lines in 2,950 files** repo-wide; **1,485 lines** live, of
which **656 lines** in product source.

**The rename target.** `SWBPIPE` / `SWB Piping Designer` / `swbpipe`: **371
lines repo-wide, zero of them outside `{PROJ}/execution/_Coordination/AgentRuns/`**.
129 lines are in this run's own artifacts (`{RUN}/instances/ROOT/DESIGN_BRIEF_V1.md`,
`{RUN}/instances/CONCEPTS/CONCEPT_DIRECTIONS_V1.md`,
`{RUN}/instances/DESIGN-SYSTEM/DESIGN_SYSTEM_V1.md` and `specimen.html`,
`{RUN}/instances/MOCKS/frames/*.html` and `tools/build.mjs`). 242 lines are in
`{PROJ}/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260913-RESULTS-ENGINEERING-3D/instances/NATIVE_VERIFY/_run_records/**`,
where a locally built witness application carried the window title "SWBPIPE
Results Engineering 3D Witness 20260913" over the unchanged bundle identifier
`org.openpipestress.technical-preview.results-engineering-3d-20260913`
(`.../FRESH_NATIVE_BUILD_BINDING_V1.json:130,134`;
`.../V2_RUNTIME/NATIVE_PROCESS_SAMPLE_V2.txt:166`). Those are historical run
records. **The name `SWBPIPE` appears nowhere in product source, schemas,
packaging or governance documents.**

### 3.4 Where the redesign puts it

- `{RUN}/instances/DESIGN-SYSTEM/DESIGN_SYSTEM_V1.md §7.5` (§7.5 The product
  name): "SWB Piping Designer is the product; SWBPIPE is the wordmark, the
  window title's short form and the file badge. The old name appears only when a
  registered sentence is quoted by ID, and the replacement sentences are
  decision-packet item 3."
- `{RUN}/instances/DESIGN-SYSTEM/DESIGN_SYSTEM_V1.md §8` (open item 8): "a
  typeset SWBPIPE is proposed; a drawn mark is not part of this system."
- **Frames:** 33 occurrences of "SWB Piping Designer" and 31 of "SWBPIPE" across
  `{RUN}/instances/MOCKS/frames/*.html`. No frame carries "OpenPipeStress".
- **Why it is asked:** `{RUN}/instances/ROOT/DESIGN_BRIEF_V1.md:170` —
  "The rename … changes those registered texts and their lint anchors, and the
  packet must state the exact replacement sentences";
  `{RUN}/instances/CONCEPTS/CONCEPT_DIRECTIONS_V1.md:317`.

### 3.5 What a change requires

Three separable acts, of different kinds.

1. **The PRD §19.3 notice.** A PRD-level owner act, named as such in two places:
   `{PROJ}/docs/claims_registry.md:106-109` and
   `{PROJ}/docs/report_notice_template.md:37-38`. The governed path for a PRD
   text change is the `SCOPE_CHANGE` (SCA) workflow — the precedent is SCA-007,
   which amended the adopted PRD to v0.3 under `D-47`/`DEC-080`. Its bundle
   shows the required shape:
   `{PROJ}/execution/_ScopeChange/SCA-007_2026-07-16_2026/` containing `Brief.md`,
   `Impact_Assessment.md`, `Amendment_Preview.md`, `Amendment_Actions.csv`,
   `Propagation_Plan.md`, `Decision_Log.md`, `ACCEPTANCE_RECORD.md`,
   `RUN_SUMMARY.md` and the PRD draft. The acceptance record states the owner
   act explicitly and bounds it: "Scope of acceptance: exactly the amendment
   actions enumerated in `Amendment_Actions.csv` … Any action discovered at
   fan-in that falls outside the packet §5 scope … returns to the owner before
   execution"
   (`{PROJ}/execution/_ScopeChange/SCA-007_2026-07-16_2026/ACCEPTANCE_RECORD.md:15-20`).
   The current accepted amendment is SCA-009
   (`{PROJ}/execution/_ScopeChange/_LATEST.md:5-7`); a new SCA takes the next
   free ID and becomes the new `_LATEST`.
2. **The `BS-IP` canonical text.** A registry change: ruled vocabulary under
   `DEC-081`, so a decision act with a `D-XX` packet, register row and `DEC` row,
   as in item 1 §1.5. D-48 §5 constrains every option: "PRD §19.3 notice text
   and §21.2 prohibited-claims table are anchors: the alignment may relocate and
   deduplicate them, never weaken them"
   (`{PROJ}/execution/_Coordination/_DECISIONS/D-48_claims_language_taxonomy.md:180-184`).
   Note that the two `BS-IP` **short variants** carry no name, so a rename does
   not by itself touch them (`{PROJ}/docs/claims_registry.md:23-26`).
3. **Packaging and data-contract identity.** `DEC-089` fixes the App ID
   description and bundle ID (`C_ui_constraints.md:84`); changing them is a
   packaging decision act. The 46 schema `$id` URLs and 61 `openpipestress.*`
   document-kind constants are **data contracts**, not prose — the registry puts
   structured metadata fields outside its own scope
   (`{PROJ}/docs/claims_registry.md:121-124`) — so changing them is a schema
   compatibility act, not a claims act, and would break every emitted document's
   identity and every `const` assertion in `{PROJ}/tests/`. `{PROJ}/docs/PLAN.md`
   is allow-listed from the claims lint (`tools/validation/validate_claims_language.py:124`).

**Evidence obligation:** any of the three is code- or schema-touching, so the
`DEC-025` five-surface sweep, repo-wide self-check including GEN-13, H4 e2e
posture and `DEC-093` viewport coverage all apply.

**Notice obligations:** the name is the project's own; no other project loop in
this repository pins it. A schema `$id` change would, however, reach every
consumer of the published schema namespace — TBD below.

### 3.6 Unknowns

- **TBD** — the exact replacement sentences. The brief records that the packet
  "must state the exact replacement sentences"
  (`{RUN}/instances/ROOT/DESIGN_BRIEF_V1.md:170`); none exists anywhere in the
  repository. Settled by the owner's ruling on packet item 3 plus, for §19.3, an
  SCA amendment preview.
- **TBD** — whether the rename extends to the schema namespace
  `openpipestress.org`, the 61 document-kind tokens, the bundle identifier and
  the crate names, or is confined to displayed text and prose. No governance
  record was found that treats the name as split between an identity layer and a
  display layer. Settled by the owner scoping the rename.
- **TBD** — whether any trademark or naming constraint attaches to "SWB Piping
  Designer" or "SWBPIPE". `{PROJ}/docs/PROFESSIONAL_BOUNDARY.md:172-176` and
  `{PROJ}/docs/IP_AND_DATA_BOUNDARY.md:31-35` leave the legal-review authority
  (`PB-TBD-004`) as `TBD`; no naming or trademark clearance record exists.
  Settled by the human project authority recording a legal-review authority, or
  by an explicit owner statement that none is required.

---

## Item 4 — short labels for the six statuses and the two evidence labels

### 4.1 Governed text, verbatim

**(a) The six automatic statuses**, `{PROJ}/docs/TYPES.md:52-68`. The table at
`:54-62` gives each status a meaning and an authority level; the binding
sentence is `:64-68`:

> Automatic software-emitted statuses are limited to `MODEL_INCOMPLETE`,
> `MECHANICS_SOLVED`, `RULE_INPUTS_INCOMPLETE`, `USER_RULE_CHECKED`,
> `USER_RULE_FAILED`, and `HUMAN_REVIEW_REQUIRED`. The software must not use
> `HUMAN_APPROVED_FOR_PROJECT`, `CODE_COMPLIANT`, `CERTIFIED`, `SEALED`, or
> `APPROVED` (PRD §21.2), or equivalent professional/code-compliance language,
> as an automatic status.

Per-status meanings at `{PROJ}/docs/TYPES.md:56-62`; `HUMAN_REVIEW_REQUIRED` is
"Result requires competent engineering review before reliance. | Always true for
professional use." (`:61`).

**(b) SPEC §4.3**, `{PROJ}/docs/SPEC.md:313-333`, adds the authority attribution:
three domains at `:318-323` (mechanics solve authority; user-rule-check
authority; human acceptance authority), the same six tokens at `:325-327`, and
the prohibition at `:327-329`.

**(c) `PROFESSIONAL_BOUNDARY` §6**, `{PROJ}/docs/PROFESSIONAL_BOUNDARY.md:101-114`:

> Product and report language must preserve these distinctions:
> Mechanics solved | User-rule checked | Human review required | Human accepted
> for project
>
> Software must not emit automatic professional-approval or automatic
> code-compliance statuses. Human acceptance records are external governance or
> project records, not solver-generated conclusions.

Note these are **status concepts in prose**, not tokens — "Human review
required" at `:109` is the closest thing in governance to a short label, defined
as "Competent review remains required before professional reliance."

**(d) The two evidence labels**, `{PROJ}/docs/claims_registry.md:87-94`:

> `INTERNALLY_VERIFIED` — internal benchmark/regression/rule-check evidence ·
> `PROVER_CORRELATED` — an external-prover comparison recorded with the PRD
> §22.5 evidence fields · `ENGINEER_ACCEPTED` — reserved for a future separate
> human-acceptance record per §21.3; no acceptance workflow exists in MVP.
> Labels attach to individual results, reports, and case pages instead of global
> hedging.

There is **no registry entry, and no listed short variant, for any of the six
status tokens or the two evidence labels.** They are not boundary statements in
`{PROJ}/docs/claims_registry.md` §1; §2 is a label vocabulary, not a statement
with variants. This is the structural fact behind the packet question.

### 4.2 What enforces it — 14 points

| # | Enforcement point | File and line | What exactly it checks |
|---|---|---|---|
| 1 | `TYPES.md` §4 | `{PROJ}/docs/TYPES.md:52-74` | The seven-value vocabulary, the six-value automatic subset, the five barred tokens, and the code-neutral boundary sentence at `:70-74` |
| 2 | `SPEC.md` §4.3 | `{PROJ}/docs/SPEC.md:313-333` | Three authority domains; same six; the prohibition on human-approval labels |
| 3 | `PROFESSIONAL_BOUNDARY.md` §6 | `:101-114` | The four preserved distinctions; the no-automatic-approval-status rule |
| 4 | `AutomaticAnalysisStatus` enum | `{PROJ}/schemas/analysis_status.schema.yaml:107-118` | Exactly the six tokens; the description states "Positive user-rule outcomes remain USER_RULE_CHECKED plus rule_check_details; USER_RULE_PASSED is intentionally absent" |
| 5 | `AnalysisStatusVocabulary` enum | `{PROJ}/schemas/analysis_status.schema.yaml:94-106` | The seven-value vocabulary including `HUMAN_APPROVED_FOR_PROJECT`, "Canonical analysis-status vocabulary from docs/TYPES.md" |
| 6 | `human_review_required` const | `{PROJ}/schemas/analysis_status.schema.yaml:33-35` | `"const": true` — a required top-level property that can only be `true` |
| 7 | `analysis_boundary` enums | `{PROJ}/schemas/analysis_boundary.schema.yaml:66-75`, `:123-131`, `:281-285` | Seven-value vocabulary, six-value automatic set, and a solver-only subset `{MODEL_INCOMPLETE, MECHANICS_SOLVED}` bound to `"authority": "solver_result_only"` (`:286-288`) |
| 8 | Schema carriage | 16 files: `analysis_boundary`, `analysis_run.legacy-desktop.v0.1`, `analysis_run.v0.1`, `analysis_run.v0.2`, `analysis_status`, `headless_runner`, `model`, `model_operation`, `model_state`, `report_generator`, `report_sections`, `results`, `results.v0.1`, `results.v0.2`, `rule_check_run_result`, `rule_pack` — all under `{PROJ}/schemas/` | Every one carries the token set as an `enum` |
| 9 | Test carriage | 17 files under `{PROJ}/tests/`: `product_preview/test_product_preview_service.py`, `test_accessibility_usability_baseline.py`, `test_analysis_run_comparison.py`, `test_analysis_run_records.py`, `test_analysis_status_schema.py`, `test_api_boundary_contract.py`, `test_design_authoring_comparison_workspace.py`, `test_headless_runner_contract.py`, `test_model_schema.py`, `test_model_state_schema.py`, `test_project_persistence_service.py`, `test_report_generator_contract.py`, `test_report_sections_contract.py`, `test_results_schema.py`, `test_results_viewer_contract.py`, `test_solve_execution_ux.py`, `test_state_comparison_handoff_report_sections.py` | Assert the enums and the round-trip of the tokens |
| 10 | Exclusion assertions | `{PROJ}/tests/test_analysis_status_schema.py:27-30`, `:43`, `:56`, `:133`; `{PROJ}/tests/test_api_boundary_contract.py:294`; `{PROJ}/tests/test_rule_pack_schema.py:367`; `{PROJ}/tests/test_state_comparison_handoff_report_sections.py:475,500,512-513` | `HUMAN_APPROVED_FOR_PROJECT` is absent from the automatic set and is stripped from emitted sections |
| 11 | `RuleCheckStatus` closed enum | `{PROJ}/schemas/rule_check_run_result.schema.json`, asserted `{PROJ}/tests/test_operation_result_schemas.py:176-180` | Exactly `["RULE_INPUTS_INCOMPLETE","USER_RULE_CHECKED","USER_RULE_FAILED"]` |
| 12 | User-guide wording test | `{PROJ}/tests/test_user_guide_status_wording.py:11-25` | The user guide names `HUMAN_APPROVED_FOR_PROJECT`, calls it an external human acceptance record bound to exact reviewed hashes, says it is "not emitted by the solver or rule-pack evaluator", and carries the BS-ACCEPT standalone variant instead of a litany |
| 13 | Claims lint, over any new label text | `tools/validation/validate_claims_language.py:35-48`, `:270-290` | Any proposed short label that stacks three litany terms on one line, or uses "not authoritative", is a finding on a scanned surface |
| 14 | Rendered-label assertions | `{PROJ}/apps/desktop/src/App.test.tsx:1068`, `:9971`, `:9974`, `:12881`, `:13787`, `:13810`, `:16970` | Seven Vitest assertions pin the *displayed* status text, not the token: `readiness-rule` contains "rule inputs incomplete"; `readiness-mechanics` contains "mechanics solved" and "0 computed result rows; model incomplete"; `proposal-professional-boundary` contains "human review required"; `viewport-deformation-status` contains "blocked; mechanics=model incomplete; rows=0" (twice); and `status-pill-mechanics` contains the raw `MODEL_INCOMPLETE`. Changing the displayed form of a status breaks these. *(Added after ROOT's check — see the Return section.)* |

### 4.3 Where it appears today

**The six tokens: 9,209 lines repo-wide; 1,124 lines in the live tree**
(`{PROJ}/apps`, `core`, `schemas`, `fixtures`, `docs`, `tests`, `api`,
`examples`, `validation`, `tools`).

| Token | Live lines | Live files |
|---|---:|---:|
| `MODEL_INCOMPLETE` | 158 | 65 |
| `MECHANICS_SOLVED` | 308 | 108 |
| `RULE_INPUTS_INCOMPLETE` | 225 | 101 |
| `USER_RULE_CHECKED` | 128 | 66 |
| `USER_RULE_FAILED` | 112 | 63 |
| `HUMAN_REVIEW_REQUIRED` | 193 | 97 |

Representative registered placements in the desktop product:
`{PROJ}/apps/desktop/src/App.tsx:3777`, `:4200`;
`{PROJ}/apps/desktop/src/features/missing-data/MissingDataBlockingPanel.tsx:270,291,312,338,368,397,418,500-505,511`;
`{PROJ}/apps/desktop/src/features/report/renderableReportInput.ts:23,296,313,323`;
`{PROJ}/apps/desktop/src/features/report/reportPackageRequest.ts:285`;
`{PROJ}/apps/desktop/src/features/report/stateComparisonHandoffSections.ts:127`;
`{PROJ}/apps/desktop/src/features/results/HistoricalRunContext.tsx:120,152,163`;
`{PROJ}/apps/desktop/src/features/rule-packs/CheckDefinitionsEditor.tsx:70,182`;
`{PROJ}/apps/desktop/src/features/result-export/resultExportAdapter.ts:164`;
`{PROJ}/apps/desktop/src/features/headless-runner/HeadlessRunnerPanel.tsx:205`.

`HUMAN_APPROVED_FOR_PROJECT`, the barred token: 169 lines repo-wide, 15 in
product source (all as a barred value in an enum or a strip-list) and 26 in
tests.

**Short labels: two are already curated in the product, and the other four are
produced by a mechanical transform.**
*(Corrected after ROOT's check — see the Return section.)*

Three renderers turn an analysis-status token into displayed text today.

**1. `readableWorkspaceStatus`** — `{PROJ}/apps/desktop/src/App.tsx:3766-3773`.
A curated mapping with a de-underscoring fallback:

> `if (token === "human_review_required" || token === "not_provided") return "Review required";`
> `if (token === "rule_inputs_incomplete" || token === "not_performed_user_rule_inputs_missing") return "Inputs needed";`
> `if (token === "not_run" || token === "not_computed") return "Not run";`
> `if (token === "computed_for_invented_demo") return "Demo computed";`
> `return value.replace(/_/g, " ").toLowerCase();`

Two of its four curated entries cover two of the six automatic statuses:
`HUMAN_REVIEW_REQUIRED` displays as **"Review required"** (`:3768`) and
`RULE_INPUTS_INCOMPLETE` as **"Inputs needed"** (`:3769`). The remaining four
tokens fall through to the lowercased, de-underscored form (`:3772`). The
function is called from exactly one place — `StatusPill` (`:3760`) — which the
workspace status bar renders four times: Mechanics, Rule check, Professional and
a conditional Solve proof (`:3599-3614`). The pill carries the label as its
`<summary>` and the **raw token** beneath it,
`<strong>Recorded status</strong><code>{value}</code>` (`:3761`), so the label
and the token are both on screen.

Two token-to-token mappers sit in front of those pills and are not labels:
`professionalStatusLabel` (`:3775-3780`) rewrites `NOT_PROVIDED` to the token
`HUMAN_REVIEW_REQUIRED`, and `ruleCheckStatusLabel` (`:3782-3787`) rewrites
`NOT_PERFORMED_USER_RULE_INPUTS_MISSING` to `RULE_INPUTS_INCOMPLETE`.

**2. `formatStatus`** — defined twice, identically, with no curation:

> `function formatStatus(value: string): string { return value.replaceAll("_", " ").toLowerCase(); }`

at `{PROJ}/apps/desktop/src/features/solve/SolvePanel.tsx:189-191` and
`{PROJ}/apps/desktop/src/features/viewport/PipeViewport.tsx:4518-4520`. Every
token becomes its lowercased, de-underscored form. Displayed as the value of the
Solve panel's "Mechanics readiness" and "Rule-check readiness" rows
(`SolvePanel.tsx:131-139`, via `:176`, `:178`, `:186`) and inside the viewport's
deformation-status summary (`PipeViewport.tsx:4412`).

**3. The raw token straight into prose** —
`{PROJ}/apps/desktop/src/features/report/renderableReportInput.ts:318`:
`` `Automatic analysis status reported by the run record: ${status}.` ``

No source file under `{PROJ}/apps/desktop/src`, `{PROJ}/core`, `{PROJ}/schemas`
or `{PROJ}/fixtures` contains a hand-written short label bound to one of the six
tokens outside those mappers. Because every displayed form except the raw token
is computed, the label strings appear as literals only at the two curated lines
above and in the tests that assert the rendered output —
`{PROJ}/apps/desktop/src/App.test.tsx:1068`, `:9971`, `:9974`, `:12881`,
`:13787`, `:13810`, `:16970` (enforcement point 14, §4.2).

Neither existing form matches the design system's proposed labels (§4.4). The
curated pair differs in **wording** — "Review required" against "Human review
required", "Inputs needed" against "Rule inputs incomplete". The fallback
differs in **case** ("model incomplete", not "Model incomplete") and, for one
token, in **number**: `USER_RULE_CHECKED` becomes "user rule checked" where the
design proposes "User rules checked". The strings "Review required" and "Inputs
needed" occur nowhere else in `{PROJ}/apps`, `{PROJ}/core`, `{PROJ}/schemas`,
`{PROJ}/fixtures` or `{PROJ}/docs`, and no test asserts them; the only pinned
pill text is the raw token (`App.test.tsx:16970`). A search for the literals
"user rule checked" and "user rule failed" over the same roots returns nothing,
which is consistent with both being produced only at runtime.

**The phrase "human review required" also occurs as hand-written prose**, in
five non-test desktop files, none of them a status label for the token:

| File and line | Text | Class |
|---|---|---|
| `{PROJ}/apps/desktop/src/features/solve/SolvePanel.tsx:369` | the literal `"human review required"` joined into a boundary summary beside `private payload=…`, `protected content=…`, `release/professional claim=…` (`:364-371`) | prose carrying the phrase; a boundary disclosure, not a status label |
| `{PROJ}/apps/desktop/src/features/results/ResultsPanel.tsx:188` | "available; *n* user-rule ratio rows; human review required" — the Governing Ratio status sentence | prose carrying the phrase |
| `{PROJ}/apps/desktop/src/features/report/renderableReportInput.ts:109` | "Human review required: evaluate this diagnostic against its referenced results before any reliance." | ordinary string: a diagnostic `remediation` value |
| `{PROJ}/apps/desktop/src/features/report/reportPackageRequest.ts:268` | `item.remediation ?? "Human review required before reliance."` | ordinary string: a default `remediation` value |
| `{PROJ}/apps/desktop/src/features/result-export/resultExportAdapter.ts:31` | `contributor_certification: 'local technical preview; human review required'` | ordinary string: a provenance metadata value |

and outside the desktop app, as ordinary strings in
`{PROJ}/core/reporting/protected_content_linter/src/lib.rs:764` (a sample
payload) and `{PROJ}/fixtures/report_lint/invented/safe_metadata_template.txt:5`
(an invented fixture), plus the two governed-prose placements already quoted in
§4.1: `{PROJ}/docs/PROFESSIONAL_BOUNDARY.md:109` and `{PROJ}/docs/SPEC.md:680`.

The other label phrases occur as follows. "Rule inputs incomplete": one
non-test product-source line, `{PROJ}/apps/desktop/src/features/solve/SolvePanel.tsx:176`
("rule inputs incomplete; mechanics results remain reviewable only; …"), plus
two test assertions. "Mechanics solved": no non-test desktop source; twelve live
lines, all governed prose or ordinary strings —
`{PROJ}/docs/PRD.md:106` (the §4.3 permitted-vocabulary bullet),
`{PROJ}/docs/PROFESSIONAL_BOUNDARY.md:107`, `{PROJ}/docs/SPEC.md:679`,
`{PROJ}/docs/architecture/extension_domain_contracts.md:85`,
`{PROJ}/docs/_Registers/Deliverables.csv:36`,
`{PROJ}/docs/_Registers/ScopeLedger.csv:48`,
`{PROJ}/core/rules/expression_evaluator/README.md:44`,
`{PROJ}/core/loads/load_case_algebra/README.md:19`, three Rust test fixtures
("Invented linear-static preview mechanics solved.") at
`{PROJ}/core/reporting/report_renderer/tests/render.rs:55`,
`{PROJ}/core/reporting/pdf_emitter/tests/emit.rs:37`,
`{PROJ}/core/reporting/report_package/tests/container.rs:49`, and one test
assertion. "Model incomplete": three lines, all test assertions in
`{PROJ}/apps/desktop/src/App.test.tsx`.

**The two evidence labels: 14 live lines, none of them in product source.**

| Label | Live occurrences |
|---|---|
| `INTERNALLY_VERIFIED` | `{PROJ}/docs/claims_registry.md:89`; `{PROJ}/docs/validation_manual/headless_runner_reproduction.md:255`; `{PROJ}/validation/evidence/reproduction/REPRO_DEL0904_20260719T023848Z_525ef0903e68/README.md:11`; `.../REPRO_DEL0904_20260719T202023Z_23eeaabc9040/README.md:7` and `manifest.json:11`; `.../REPRO_DEL0904_20260720T074714Z_a5235340aae3/README.md:4,32`, `manifest.json:621,698`, `validation_summary.json:36` |
| `PROVER_CORRELATED` | `{PROJ}/docs/claims_registry.md:90`; `{PROJ}/validation/evidence/reproduction/REPRO_DEL0904_20260719T023848Z_525ef0903e68/README.md:11` |
| `ENGINEER_ACCEPTED` | `{PROJ}/docs/claims_registry.md:91`; `{PROJ}/validation/evidence/reproduction/REPRO_DEL0904_20260719T023848Z_525ef0903e68/README.md:11` |

Zero occurrences in `{PROJ}/apps`, `{PROJ}/core`, `{PROJ}/schemas`,
`{PROJ}/fixtures`. M-09 is a registry rule with no product implementation.

**Historical records — must not change:** 7,940 lines of the six tokens and 132
lines of the evidence labels, under `AgentRuns/**`, `_run_records/**`,
`_Evaluation/**` and the other excluded paths.

### 4.4 Where the redesign puts it

- **The mapping table:** `{RUN}/instances/DESIGN-SYSTEM/DESIGN_SYSTEM_V1.md §2.3`
  — six rows, raw token to short label to chip tokens to authority domain
  ("Solver", "Rule pack", "Human"). `§2.3` states its own standing: "Short
  labels are decision-packet item 4 (whether they are listed variants of the
  registered texts); until ruled, the mapping below is the design's proposal and
  the raw token is always reachable."
- **Chip anatomy and the raw token one click away:**
  `{RUN}/instances/DESIGN-SYSTEM/DESIGN_SYSTEM_V1.md §2.3`.
- **Evidence labels:** `{RUN}/instances/DESIGN-SYSTEM/DESIGN_SYSTEM_V1.md §2.3` —
  "Internally verified" and "Prover correlated" beside the run name in a results
  header, raw token in the tooltip; "`ENGINEER_ACCEPTED` is never emitted".
- **Copy rule:** `{RUN}/instances/DESIGN-SYSTEM/DESIGN_SYSTEM_V1.md §7.1` and
  `§7.1` forbid any status outside the six, and forbid "Engineer accepted (never
  emitted), Validated, Certified".
- **Disclosure homes:** M-08 to "The status bar chip, with the authority domain
  in its popover"; M-09 to "The results header chip beside the run name; the
  report's case pages" (`§7.4`).
- **Frames:** every frame carries a status chip whose `title` attribute is the
  raw token plus its authority domain, e.g.
  `{RUN}/instances/MOCKS/frames/s1_table_light.html` — `title="MODEL_INCOMPLETE
  · authority: Solver · click for the run identity"` with visible text "Model
  incomplete". Counted across the fifteen frames: "Model incomplete" 7,
  "Mechanics solved" 4, "User rules checked" 6, "Internally verified" 6, "Human
  review required" 2. "Rule inputs incomplete" and "User rule failed" do not
  appear in this frame set.
- **"Human review required" on every surface:** the frames show it on the Review
  page only (`{RUN}/instances/MOCKS/frames/s9_table_light.html` and
  `s9_table_dark.html`). The owner adopted this on 2026-09-18:
  `{RUN}/instances/ROOT/DIRECTION_DECISION_2026-09-17.md:153` — "Two chips after
  a solve, one per authority domain (mechanics, user rules). 'Human review
  required' is shown on the Review page only, **unless the registry requires it
  elsewhere, which decision-packet item 4 settles.**" The question was raised at
  `{RUN}/instances/MOCKS/MOCKS_V1.md:146` (Q-3) and `:153` (Q-10).
- **Added during this inventory by the V1.1 revision:** a chip-display policy
  paragraph, `{RUN}/instances/DESIGN-SYSTEM/DESIGN_SYSTEM_V1.md §2.3`,
  headed "When chips show (decisions 2, 3 and 4; D-4)". It states the design's
  full proposal: one chip (Model incomplete, Solver) before any run and whenever
  the model cannot be solved; **no chip** when the model is complete but
  unsolved, because "none of the six statuses applies, an absent status is
  honest, and no seventh label is added"; two chips after a solved run, one per
  authority domain; on the Review page, "User rules checked and Human review
  required (Human); **Human review required appears on no other page unless the
  registry requires it elsewhere, which decision-packet item 4 settles**"; after
  a model change the chips drop and the rail's Results caption reads Stale; a
  reopened saved run keeps its chips and carries the historical caption in the
  popover (M-13); "Chips are read, never clicked to change anything." This is
  the design's proposal, conditioned on this packet item, not a governed rule.
- **Why it is asked:** `{RUN}/instances/CONCEPTS/CONCEPT_DIRECTIONS_V1.md:318`;
  `{RUN}/instances/ROOT/DESIGN_BRIEF_V1.md:160`, `:171`;
  `{RUN}/instances/DESIGN-SYSTEM/DESIGN_SYSTEM_V1.md §8`.

### 4.5 What a change requires

- **Adding short labels as listed variants** would put them in
  `{PROJ}/docs/claims_registry.md` — but §1 holds boundary *statements* and §2
  holds *labels*, and neither currently carries variant lists for status tokens.
  Creating that structure is a change to ruled vocabulary under `DEC-081`: a
  decision act with a `D-XX` packet, register row and `DEC` row
  (`{PROJ}/execution/_Coordination/_DECISIONS/_REGISTER.md:6-15`;
  `{PROJ}/execution/_Coordination/_DECISIONS/D-48_claims_language_taxonomy.md:198-202`).
- **Changing the token set itself** is a `TYPES.md` §4 change, which
  `{PROJ}/schemas/analysis_status.schema.yaml:96` names as its source
  ("Canonical analysis-status vocabulary from docs/TYPES.md"), and would
  propagate to 16 schemas and 17 test files. `TYPES.md` §9 was itself amended by
  an owner decision act, `D-39`
  (`{PROJ}/execution/_Coordination/_DECISIONS/_REGISTER.md:73`), which is the
  precedent for amending that document.
- **Short labels alone, without registry change,** remain product microcopy
  governed by `{PROJ}/docs/PROFESSIONAL_BOUNDARY.md:98-99` ("Prohibited language
  is unacceptable even when convenient for … UI labels") and PRD §5.9
  (`C_ui_constraints.md:35`), and by the claims lint over any line that stacks
  boundary terms. No governance text requires that a rendered status label be
  the raw token — and the product already does not render one everywhere: the
  status pill shows a curated label for two tokens ("Review required", "Inputs
  needed") with the raw token beneath it, and the two `formatStatus` helpers
  show a lowercased, de-underscored form (§4.3). So the packet question is not
  whether to introduce a human-readable label, which exists in two competing
  forms, but which label set becomes registered vocabulary and whether the
  existing curated pair is superseded. *(Corrected after ROOT's check — see the
  Return section.)*
- **Whether "Human review required" must be on every surface showing solve or
  rule-check state.** The requirement as written is M-08's placement column,
  "Any surface showing solve or rule-check state"
  (`C_ui_constraints.md:289`) — which is C's own summary, not a governed
  sentence. At source, three texts bear on it, and none of them states a
  placement rule: `{PROJ}/docs/TYPES.md:61` gives `HUMAN_REVIEW_REQUIRED` the
  authority level "Always true for professional use";
  `{PROJ}/schemas/analysis_status.schema.yaml:33-35` requires the envelope field
  `human_review_required` to be `const true`, which is a **data** obligation on
  every status envelope, not a display obligation; and
  `{PROJ}/docs/PROFESSIONAL_BOUNDARY.md:109` lists "Human review required" as
  one of four distinctions "Product and report language must preserve". So the
  binding today is: the status envelope must always carry the flag, and product
  language must preserve the distinction; no located text says the label must be
  rendered on every surface. Confirming or setting a display rule is the
  registry-level decision act above.
- **Evidence obligation:** any label change is code-touching —
  `DEC-025` sweep, self-check including GEN-13, H4 e2e posture, `DEC-093`
  viewport coverage. A schema enum change additionally moves `{PROJ}/tests/**`.
- **Notice obligations:** `{PROJ}/docs/TYPES.md` is a shared governance document
  for this project only; no other loop's contract mirror pins it in this
  repository.

### 4.6 Unknowns

- **TBD** — whether short labels are to be registered as listed variants or left
  as ungoverned microcopy. Settled by the owner's ruling on packet item 4, which
  `{RUN}/instances/DESIGN-SYSTEM/DESIGN_SYSTEM_V1.md §2.3` explicitly defers.
- **TBD** — what becomes of the three existing renderers (§4.3) if a curated
  label set is adopted: `readableWorkspaceStatus`, whose curated pair "Review
  required" / "Inputs needed" is shorter than and different from the design's
  proposed wording; the two duplicated `formatStatus` helpers, whose output
  differs from the proposal in case and, for `USER_RULE_CHECKED`, in number; and
  the raw-token prose line at `renderableReportInput.ts:318`. Whether the owner
  regards any of these as already satisfying M-08, or as microcopy to be
  replaced, is not something this inventory can decide. Settled by the ruling on
  packet item 4.
- **TBD** — whether the shortened curated labels are themselves acceptable under
  `{PROJ}/docs/PROFESSIONAL_BOUNDARY.md:109`, which lists "Human review
  required" as a distinction "Product and report language must preserve". The
  status pill renders "Review required", dropping "Human". Nothing located rules
  on whether that shortening preserves the distinction; the raw token is
  displayed beside it (`App.tsx:3761`), which may bear on the answer. Settled by
  the owner's ruling on packet item 4, or by a registry placement clause.
- **TBD** — whether "Human review required" must appear on every surface showing
  solve or rule-check state. No governed sentence states a placement rule; the
  adopted mock decision (`{RUN}/instances/ROOT/DIRECTION_DECISION_2026-09-17.md:153`)
  is conditioned on this item. Settled by the owner's ruling, or by adding a
  placement clause to the registry.
- **TBD** — whether a seventh label is needed for "model complete but unsolved".
  Raised at `{RUN}/instances/MOCKS/MOCKS_V1.md:145` (Q-2); the owner adopted "no
  status chip … an absent status is honest and no seventh label is added"
  (`{RUN}/instances/ROOT/DIRECTION_DECISION_2026-09-17.md:152`). Whether an
  absent chip satisfies M-08's own placement wording is unsettled and is part of
  packet item 4.
- **TBD** — whether M-09 is to be implemented at all, given zero product
  occurrences of either evidence label today (§4.3). Settled by the owner
  deciding whether the results-header evidence chip ships.

---

## Item 5 — vendor hanger tables as a library class

### 5.1 Governed text, verbatim

**(a) `BS-IP` placement**, `{PROJ}/docs/claims_registry.md:27-29`:

> **Belongs on:** import/library/contribution UI, redaction/export surfaces,
> contribution docs. **Not on:** results views (unless the surface genuinely
> handles imported content).

with the canonical text at `:19-21` (quoted in item 3 §3.1).

**(b) SCA-009 Vocabulary Annex §1.4, Hanger-selection framing**,
`{PROJ}/execution/_ScopeChange/SCA-009_2026-08-20_0000/Vocabulary_Annex.md:57-64`:

> **Hanger-selection framing.** Spring-hanger selection operates only over
> **user-imported** hanger libraries, consistent with DEC-049 … and the
> user-supplied library-import boundary … a hanger library kind is a governed
> extension of that user-supplied model). No hanger catalog is bundled.

**(c) Vocabulary item 23**, `{PROJ}/execution/_ScopeChange/SCA-009_2026-08-20_0000/Vocabulary_Annex.md:127`:

> | 23 | Spring-hanger selection from user-imported hanger libraries | 3
> (accepted) | `LibraryKind` (user-supplied, provenance-gated),
> `core/library_import/library_import_document/src/lib.rs:50-53` …

**(d) `DEC-049`**, `{PROJ}/execution/_Decomposition/SOFTWARE_DECOMP.md:640`:
the D-15 Option B ruling — "a minimal dedicated spring-hanger model with
explicit user-entered variable spring hanger and constant-effort support
data/provenance … The implementation must remain code-neutral: no catalog
sizing, no protected standards values, no hidden defaults, and no
professional/code-compliance claim."

**(e) `DEC-094`**, `{PROJ}/execution/_Decomposition/SOFTWARE_DECOMP.md:685`: the
SCA-009 ruling that makes the Vocabulary Annex "the DEL-07-09 coverage contract"
bound "to the implemented operation taxonomy, not the schema OperationKind
enum".

**(f) Contract invariants**: `OPS-K-DATA-1`
(`{PROJ}/docs/CONTRACT.md:26`, "not bundled public defaults"), `OPS-K-DATA-3`
(`:28`, provenance fields), `OPS-K-IP-1` (`:23`, "proprietary commercial data"
barred from the public repository), `OPS-K-IP-3` (`:25`, quarantine).
`{PROJ}/docs/IP_AND_DATA_BOUNDARY.md:53` bars "proprietary vendor catalogs
without redistribution rights" from the public repository; `:58-72` lists the
seven required provenance fields and says records with `source_license = TBD`,
`redistribution_status = unknown` or no contributor certification "are not
acceptable as public data".

### 5.2 What enforces it — 10 points

| # | Enforcement point | File and line | What exactly it checks |
|---|---|---|---|
| 1 | `DEC-049` | `{PROJ}/execution/_Decomposition/SOFTWARE_DECOMP.md:640` | Code-neutral, user-entered only, no catalog sizing, no professional claim |
| 2 | SCA-009 Annex §1.4 | `{PROJ}/execution/_ScopeChange/SCA-009_2026-08-20_0000/Vocabulary_Annex.md:57-64` | User-imported libraries only; "No hanger catalog is bundled" |
| 3 | SCA-009 item 23 | same file, `:127` | The vocabulary item and its tier; DEC-049 framing "is mandatory" per `C_ui_constraints.md:262` |
| 4 | `DEC-094` | `{PROJ}/execution/_Decomposition/SOFTWARE_DECOMP.md:685` | The annex is the DEL-07-09 coverage contract; single palette surface; every command through the PKG-16 operation layer |
| 5 | `BS-IP` placement rule | `{PROJ}/docs/claims_registry.md:27-29` | Import/library/contribution UI carries BS-IP |
| 6 | Contract invariants | `{PROJ}/docs/CONTRACT.md:23,25,26,28` | No bundled public defaults; provenance fields; no proprietary commercial data; quarantine on suspicion |
| 7 | `hanger.schema.yaml` | `{PROJ}/schemas/hanger.schema.yaml:1-27` and `$defs` | `"additionalProperties": false`; `schema_version` `const "1.0.0"`; required `hanger_library` + `hanger_records`; description: "Explicit imported values only; import validity does not establish solve readiness. Duplicate hanger_id values are rejected by the import boundary." |
| 8 | `LibraryKind::Hanger` | `{PROJ}/core/library_import/library_import_document/src/lib.rs:50-55`, parser at `:57-62` | A fourth library kind exists; unsupported kinds return `None` and "callers at the seam should reject unknown kinds rather than guess" (`:59-60`) |
| 9 | Python import validator | `{PROJ}/core/library_import/provenance_checker.py:35` (record-key dispatch), `:107-108` (kind routing), `:355-393` (`_hanger_schema`, `_hanger_shape`, finding `IMPORT_HANGER_SCHEMA_INVALID` at `:368`, `:392`) | The imported payload satisfies the declared hanger schema; a missing required field is a blocking finding with a remediation |
| 10 | `{PROJ}/tests/test_hanger_library_schema.py` | `:16`, `:21`, `:39`, `:46`, `:55` | Draft-2020-12 validity; schema/runtime agreement; non-finite and non-positive values rejected; all supported units; unknown schema keyword fails closed |

### 5.3 Where it appears today

"hanger", case-insensitive, live tree: **2,283 lines in 81 files**.

| Class | Detail |
|---|---|
| Registered placement — schema | `{PROJ}/schemas/hanger.schema.yaml` (18 name occurrences) |
| Registered placement — core | `{PROJ}/core/library_import/library_import_document/src/lib.rs` (34); `{PROJ}/core/library_import/provenance_checker.py` (27); `{PROJ}/core/product_physics/src/lib.rs:335` `SpringHangerInput` (cited by `Vocabulary_Annex.md:111`) |
| Registered placement — desktop | `{PROJ}/apps/desktop/src/features/hanger-selection/HangerSelectionPanel.tsx` (41); `.../hangerSelection.ts` (25); `.../index.ts` (2); wired at `{PROJ}/apps/desktop/src/App.tsx:3` (import) and `:2850` (render); type at `{PROJ}/apps/desktop/src/types.ts:199-200` |
| Ordinary string | `{PROJ}/tests/test_hanger_library_schema.py`; `{PROJ}/apps/desktop/src/App.test.tsx` (10 lines); fixtures under `{PROJ}/fixtures/hanger/` |
| Live governance surfaces | `{PROJ}/execution/_ScopeChange/SCA-009_2026-08-20_0000/Brief.md:154` — "lengths, spring-hanger catalogs) remains user/vendor-supplied private" |
| Historical record — must not change | `{PROJ}/plans/VERIFICATION_2026-06-23_r4_exit_chain_refresh.md:162`; `{PROJ}/plans/VERIFICATION_2026-06-22_r4_exit_chain.md:101`; `{PROJ}/plans/PLAN_2026-06-17_prd_completion.md:160`; `{PROJ}/execution/_Coordination/_DECISIONS/D-25_r4_exit_scope.md:84`; `.../D-27_r4_exit_clearance_stage_advancement.md:98` |

**Where BS-IP actually sits on library surfaces today: 23 lines in 11 files**
carry the phrase "no protected standards content" in the live tree. Of those:

- `{PROJ}/apps/desktop/src/features/library/LibraryManagerPanel.tsx:976` carries
  a form of the BS-IP short variant using an em dash — "software findings over
  an already-parsed payload; no protected standards content —".
- `{PROJ}/apps/desktop/src/features/missing-data/MissingDataBlockingPanel.tsx:539`
  carries "no protected standards content — code-specific data is
  user-supplied; acceptance and professional judgment remain with …". The
  registered short variant at `{PROJ}/docs/claims_registry.md:23-24` uses a
  **semicolon**, not an em dash; the lint normalises whitespace and case but not
  punctuation (`tools/validation/validate_claims_language.py:166-167`), so this
  line passes the litany rule only because it also carries the BS-ACCEPT
  variant.
- `{PROJ}/apps/desktop/src/features/report-lint/ReportLintPanel.tsx` carries the
  exact registered semicolon variant 9 times (`:564`, `:571`, `:585`, `:592`,
  `:599`, `:606`, `:613`, `:620`, `:627`).
- `{PROJ}/apps/desktop/src/features/native-package/NativePackagePanel.tsx:340`
  carries an ordinary string, "no protected standards content or private rule
  payload is bundled".
- **`{PROJ}/apps/desktop/src/features/hanger-selection/HangerSelectionPanel.tsx`
  carries no BS-IP text at all.** Its visible copy is the heading "Select an
  imported hanger" (`:100`), a browser-unavailability line (`:102`), a
  provenance `<dl>` (`:114`) and an "Imported values and all source provenance"
  disclosure (`:115`). The BS-IP placement rule is registry text, not a lint
  anchor, so nothing currently fails over this.
- The `BS-IP` **canonical** sentence appears in exactly one live place,
  `{PROJ}/docs/claims_registry.md:19` — it is on no product surface at all.

### 5.4 Where the redesign puts it

- **Disclosure home:** `{RUN}/instances/DESIGN-SYSTEM/DESIGN_SYSTEM_V1.md §7.4` —
  "M-05 | The Libraries import dialog; the export and redaction surfaces."
- **M-04 home names hanger selection:**
  `{RUN}/instances/DESIGN-SYSTEM/DESIGN_SYSTEM_V1.md §7.4` — "The report and its
  preview when export metadata, **hanger selection records** or handoff data are
  included; named in the export dialog."
- **Report outline includes a Hanger selection section:**
  `{RUN}/instances/DESIGN-SYSTEM/DESIGN_SYSTEM_V1.md §5.5`.
- **Frames:** state 8 shows "the hanger table against a user-supplied library"
  (`{RUN}/instances/DESIGN-SYSTEM/DESIGN_SYSTEM_V1.md §8`); the frames are
  `{RUN}/instances/MOCKS/frames/s8_table_light.html` and `s8_model_light.html`
  (`{RUN}/instances/MOCKS/MOCKS_V1.md:105-120`).
- **Why it is asked:** `{RUN}/instances/CONCEPTS/CONCEPT_DIRECTIONS_V1.md:319` —
  "The directions treat them as imported libraries with provenance under M-05.
  The packet should confirm that framing against the hanger-selection item in
  the vocabulary (C §3, item 23)."

### 5.5 What a change requires

- **Confirming the framing** — that vendor hanger tables are a library class
  under M-05 — changes no governed text. It restates `DEC-049` plus SCA-009 §1.4
  plus the `BS-IP` placement rule, all of which already say user-imported,
  provenance-gated, nothing bundled. The owner act here is a **confirmation**,
  recordable as a `D-XX` register row and ruling, not an amendment.
- **Promoting item 23 out of ROADMAP/Tier-3 sequencing, or changing its tier**,
  is a scope act. The annex states it directly for ROADMAP items: "Promoting any
  ROADMAP item into NORMATIVE-NOW is a future owner act (scope-change or
  owner-ruled deliverable amendment), **not an implementation choice**"
  (`C_ui_constraints.md:273-274`, quoting
  `{PROJ}/execution/_ScopeChange/SCA-009_2026-08-20_0000/Vocabulary_Annex.md`).
  Item 23 is already NORMATIVE-NOW Tier 3, so this clause does not apply to it;
  its sequencing rule ("net-new backend capability is implemented separately and
  afterwards", `Vocabulary_Annex.md:53-56`) is an owner ruling block.
- **Bundling any vendor catalog** is barred and cannot be reached by an owner act
  of this kind: `{PROJ}/docs/IP_AND_DATA_BOUNDARY.md:53` and `OPS-K-IP-1`
  (`{PROJ}/docs/CONTRACT.md:23`) bar proprietary vendor catalogs from the public
  repository, and `{PROJ}/docs/IP_AND_DATA_BOUNDARY.md:94` records that the
  owner "cannot waive third parties' rights by configuration".
- **Adding BS-IP to the hanger-selection surface** is an ordinary code-touching
  edit under the existing registry placement rule; it needs no new owner act,
  only the `DEC-025` sweep and H4 evidence.
- **Deliverable homes:** `DEL-07-09` owns the vocabulary and coverage ledger and
  "never dispatches implementation" (`C_ui_constraints.md:84` citing
  `Vocabulary_Annex.md` §5 and `DEC-094`); implementation routes to
  `DEL-07-01/02/03` and PKG-16.
- **Notice obligations:** none found outside this project loop.

### 5.6 Unknowns

- **TBD** — whether the owner treats the annex's "governed extension of that
  user-supplied model" as already discharged by the shipped `LibraryKind::Hanger`
  and `hanger.schema.yaml`, or as still requiring a recorded extension act. The
  annex's own citation is stale against the source (difference D-5). Settled by
  the owner's ruling on packet item 5, or by a `DEL-07-09` coverage-ledger update.
- **TBD** — whether the hanger-selection surface is required to carry BS-IP. The
  registry rule says import/library UI carries it; nothing enforces it, and the
  surface does not. Settled by the same ruling, or by an ordinary alignment
  tranche.

---

## Item 6 — the historical-run wording, as the existing component states it

### 6.1 Governed text, verbatim

There is **no registry entry and no governance document** for this wording. The
only text is in the component itself,
`{PROJ}/apps/desktop/src/features/results/HistoricalRunContext.tsx`.

**(a) The stated limit — a source comment**, `:8-9`:

> // Transient evidence only. Saved run references do not contain the historical
> // input-manifest payload, so reopening cannot establish a current solve basis.

**(b) The designation — a type literal**, `:11`:

> designation: "historical_saved_run";

emitted at `:315`.

**(c) The rendered text**, `:320-323`:

> `<section className="panel" aria-label="Historical saved run"
> data-testid="historical-run-context">`
> `<div className="panel-title">Historical saved run</div>`
> "Saved evidence for {context.runId}; mechanics={…}. Run a fresh solve to
> establish current results."
> "Historical results cannot drive current overlays, rule checks, comparisons or
> report readiness."

The quoted limit in (a) is **not** what the component displays. See difference
D-4 in §0.3.

The supporting governance is indirect: PRD §15.1/§8.6 immutability
(`C_ui_constraints.md:109`), SPEC §3.2 read-only snapshot policy
(`C_ui_constraints.md:110`), and the DEL-05-04 fail-closed applier
(`C_ui_constraints.md:114`).

### 6.2 What enforces it — 6 points

| # | Enforcement point | File and line | What exactly it checks |
|---|---|---|---|
| 1 | Type literal | `{PROJ}/apps/desktop/src/features/results/HistoricalRunContext.tsx:11` | `designation` is the string literal type `"historical_saved_run"`; any other value is a compile error |
| 2 | Emission | same file, `:315` | The constructed context always carries that designation |
| 3 | Accessible name | same file, `:320` | `aria-label="Historical saved run"` |
| 4 | Rendered panel title | same file, `:321` | Visible text "Historical saved run" |
| 5 | Rendered limit sentences | same file, `:322-323` | The two sentences that carry the limit to the user |
| 6 | Unit tests | `{PROJ}/apps/desktop/src/features/results/HistoricalRunContext.test.tsx:167`, `:223` | `expect(context!.designation).toBe("historical_saved_run")` in two cases |

Indirect: the status-token guards at `:120`, `:152`, `:163` require the reopened
run's `analysis_status` to include `HUMAN_REVIEW_REQUIRED` and to be a
deduplicated subset of the six-token set — these are item 4's enforcement
points, reused here.

### 6.3 Where it appears today — 6 lines in 2 files

| Class | Occurrence |
|---|---|
| Registered placement | `{PROJ}/apps/desktop/src/features/results/HistoricalRunContext.tsx:11`, `:315`, `:320`, `:321` (and the comment at `:8-9`) |
| Ordinary string | `{PROJ}/apps/desktop/src/features/results/HistoricalRunContext.test.tsx:167`, `:223` |
| Historical record — must not change | none of this wording appears under an excluded path |

No occurrence in `{PROJ}/docs`, `{PROJ}/schemas`, `{PROJ}/core` or
`{PROJ}/fixtures`. M-13 is a component fact, not a registry fact.

### 6.4 Where the redesign puts it

- **Marks vocabulary row:** `{RUN}/instances/DESIGN-SYSTEM/DESIGN_SYSTEM_V1.md §4`
  — "Historical run (M-13) | a clock | `historical.ink` on `historical.tint` |
  band across the results header; caption on the rail | Historical saved run ·
  reopening cannot establish a current solve basis | Space on the band |
  Results: Historical". The tooltip text reuses both the designation and the
  source comment's limit.
- **Colour rule:** `{RUN}/instances/DESIGN-SYSTEM/DESIGN_SYSTEM_V1.md §2.5` —
  "Historical is a warm neutral band … with a hatch, never a colour that reads
  as status."
- **Disclosure home:** `{RUN}/instances/DESIGN-SYSTEM/DESIGN_SYSTEM_V1.md §7.4` —
  "M-13 | The results header band and the rail's Historical caption; the status
  chip's popover."
- **Status chip popover:** `{RUN}/instances/DESIGN-SYSTEM/DESIGN_SYSTEM_V1.md §2.3`.
- **Open item 7 states the intent not to write new wording:**
  `{RUN}/instances/DESIGN-SYSTEM/DESIGN_SYSTEM_V1.md §8` — "The rail's Results
  item caption 'Historical' and the results header band reuse the existing
  component's wording (M-13); no new wording is proposed."
- **Frames:** no frame in the fifteen carries "Historical saved run"; the closest
  is state 8's stale-results treatment
  (`{RUN}/instances/ROOT/DIRECTION_DECISION_2026-09-17.md:154`).
- **Why it is asked:** `{RUN}/instances/CONCEPTS/CONCEPT_DIRECTIONS_V1.md:320` —
  "The wording of the stated limit on the banner should be taken from the
  existing component, not written new."

### 6.5 What a change requires

- **No owner act is named by any governance text for this wording.** It is
  component copy under a deliverable, not registry vocabulary. Changing it is an
  ordinary code-touching edit: `DEC-025` five-surface sweep, repo-wide
  self-check including GEN-13, H4 evidence (a Vitest change is already required
  by the two assertions in §6.2 point 6), `DEC-093` viewport coverage.
- **The constraint that does bind** the wording is the claims boundary: any
  replacement must not read as an approval or acceptance status
  (`{PROJ}/docs/PROFESSIONAL_BOUNDARY.md:98-99`, `{PROJ}/docs/SPEC.md` §3.2 per
  `C_ui_constraints.md:47`), and must not stack three litany terms without a
  registered text (`tools/validation/validate_claims_language.py:270-283`) —
  the component is a non-test `.tsx` under `{PROJ}/apps/desktop/src`, so it is
  in the lint's scan set.
- **The `designation` string literal is different in kind:**
  `"historical_saved_run"` is a typed token in a component contract, and the
  brief's item asks about wording, not the token. Changing the token is a
  TypeScript type change with two test assertions.
- **Notice obligations:** none.

### 6.6 Unknowns

- **TBD** — whether the owner intends the banner to carry the source comment's
  sentence ("reopening cannot establish a current solve basis", which the design
  system's tooltip at `§4` adopts) or the component's two rendered sentences
  at `{PROJ}/apps/desktop/src/features/results/HistoricalRunContext.tsx:322-323`.
  These are different texts; C treats the first as the required display text
  (difference D-4). Settled by the owner's ruling on packet item 6, reading the
  component.

---

## Item 7 — the Checked mark

### 7.1 Governed text, verbatim

There is no governed text for a "Checked" mark. Four texts bound the space it
would occupy.

**(a) C-22, the state-labelling affordances — PRD §16.3**,
`{PROJ}/docs/PRD.md:1060-1070`:

> ### 16.3 Naming, Tags, and Notes Instead of Forced Prover Status
>
> The MVP shall not enforce a formal prover-status vocabulary or lifecycle.
> Instead, users shall be able to use: model names; state names; tags; notes;
> external reference fields; attachments or links where supported; comparison
> reports.

with example naming conventions at `:1074-1079`, and PRD §7 non-goal 6,
`{PROJ}/docs/PRD.md:210`: "Force a formal prover-status lifecycle in the MVP."

**(b) C-06, the acceptance-record prohibition — PRD §21.3**,
`{PROJ}/docs/PRD.md:1339-1343`:

> Any human acceptance record, if stored in future releases, shall be separate
> from solver outputs, rule-pack outputs, comparison outputs, and report
> generation.
>
> MVP shall not require a formal acceptance workflow.

and `PROFESSIONAL_BOUNDARY` §7, `{PROJ}/docs/PROFESSIONAL_BOUNDARY.md:116-130`,
which lists five requirements on any stored human acceptance record — identify
the authority; bind to model hash, rule-pack checksum, report hash, software
version and input manifest; identify scope and limitations; "become stale or
invalid for reliance when any bound model, rule-pack, report, input manifest, or
relevant software basis changes"; "remain separate from solver, rule-pack, and
report-generation outputs" — and closes "The exact acceptance-record storage
workflow remains `TBD`" (`:129-130`).

**(c) C-65, the binding rule — `OPS-K-AUTH-2`**, `{PROJ}/docs/CONTRACT.md:30`:

> Human acceptance records, if used, bind to specific model/rule/report hashes
> and do not survive content changes without re-review. | Audit manifest; hash
> checks

**(d) `ENGINEER_ACCEPTED` reserved**, `{PROJ}/docs/claims_registry.md:91-92`:
"reserved for a future separate human-acceptance record per §21.3; no acceptance
workflow exists in MVP."

### 7.2 What enforces it — 9 points

| # | Enforcement point | File and line | What exactly it checks |
|---|---|---|---|
| 1 | PRD §16.3 | `{PROJ}/docs/PRD.md:1060-1070` | Names the seven permitted affordances; bars an enforced prover-status vocabulary |
| 2 | PRD §7 non-goal 6 | `{PROJ}/docs/PRD.md:210` | No forced prover-status lifecycle in the MVP |
| 3 | PRD §21.3 | `{PROJ}/docs/PRD.md:1339-1343` | Separation of any acceptance record from solver/rule/comparison/report outputs; no required acceptance workflow in MVP |
| 4 | `PROFESSIONAL_BOUNDARY` §7 | `:116-130` | The five requirements on any stored acceptance record, including staleness on any bound-basis change |
| 5 | `OPS-K-AUTH-2` | `{PROJ}/docs/CONTRACT.md:30` | Hash binding; no survival across content change without re-review. Enforcement column: "Audit manifest; hash checks" |
| 6 | SPEC §3.2 | per `C_ui_constraints.md:47` | Model states provide no approval, certification, sealing or authentication states, no automatic code-compliance statuses, and no professional acceptance records |
| 7 | `tags` / `notes` in the model-state schema | `{PROJ}/schemas/model_state.schema.json:303-304` (required), `:346-351` (`StateTag[]`), `:352-357` (`StateNote[]`) | The existing, schema-backed carriers of the PRD §16.3 affordances |
| 8 | `ENGINEER_ACCEPTED` reserved | `{PROJ}/docs/claims_registry.md:91-92` | Reserved label; not emitted |
| 9 | `DEL-16-03` Remaining | `{PROJ}/execution/PKG-16_Model Operation and Agent Proposal Framework/1_Working/DEL-16-03_User acceptance and operation audit trail/_STATUS.md:8` | "Durable accepted/rejected operation history remains incomplete under SOW-070" — and per `C_ui_constraints.md:115`, reopened records "must remain review context with acceptance unknown" |

**There is no code enforcement, because there is no Checked affordance.** A
search of `{PROJ}/apps`, `{PROJ}/core`, `{PROJ}/schemas`, `{PROJ}/fixtures`,
`{PROJ}/docs` and `{PROJ}/tests` for `checkedBy`, `checked_by`, the literal
`"Checked"` and `>Checked<` returns nothing. The word "Checked" as a mark does
not exist in the product.

### 7.3 Where it appears today — zero in the product

| Class | Count | Detail |
|---|---:|---|
| Registered placement | 0 | none |
| Ordinary string | 0 | none |
| Historical record | 0 | none |
| Design artefacts in this run | — | see §7.4 |

### 7.4 Where the redesign puts it

- **Copy rules, §7.3 Checked:**
  `{RUN}/instances/DESIGN-SYSTEM/DESIGN_SYSTEM_V1.md §7.3` — the control is
  "Check" (⌘⇧K), the mark is "Checked"; tooltip "Checked by *name* · *date time*
  · bound to this row's content · **not a software status**"; stale tooltip
  "…the row changed since · Check again or Clear"; filter "Unchecked rows".
  `§7.3` states its own standing: "The word is a tag on a row; its governance
  standing and final wording remain decision-packet item 7."
- **Allowed and forbidden words:**
  `{RUN}/instances/DESIGN-SYSTEM/DESIGN_SYSTEM_V1.md §7.1` — allowed "Checked,
  Checked by *name*, stale, Check again, Clear check, Unchecked rows"; forbidden
  "Verified, reviewed and approved, signed, accepted".
- **Marks vocabulary:** `{RUN}/instances/DESIGN-SYSTEM/DESIGN_SYSTEM_V1.md §4`
  (Checked: a check glyph, `mark.checked`, state slot) and `§4` (Checked,
  stale: a dashed check with a small dot, `mark.checkedStale`).
- **Colour rule:** `{RUN}/instances/DESIGN-SYSTEM/DESIGN_SYSTEM_V1.md §2.5` —
  "Checked is neutral ink (`mark.checked`), a human tag and not a colour; stale
  Checked is amber (`mark.checkedStale`) with a dashed glyph."
- **Slot priority:** `{RUN}/instances/DESIGN-SYSTEM/DESIGN_SYSTEM_V1.md §4` —
  state slot order blocking, warning, stale Checked, Checked, note.
- **Frames:** five of fifteen carry it —
  `{RUN}/instances/MOCKS/frames/s4_both_light.html` and `s4_table_light.html`
  (the state-slot glyph with the full tooltip "Checked by R. Tufts · 2026-09-17
  14:02 · bound to this row's content · not a software status"), and
  `s2_model_light.html`, `s2_model_dark.html`, `s8_model_light.html` (an
  inspector section headed "Origin and Checked").
- **Why it is asked:** `{RUN}/instances/ROOT/DIRECTION_DECISION_2026-09-17.md:73`
  — "The checked tag is a human tag on a row, in the sense of the
  state-labelling affordances the PRD intends (C-22), not a status from the
  automatic vocabulary (C-09, C-10) and not an acceptance record (C-06, C-65).
  It binds to the row's content so that it lapses visibly on change, in the
  spirit of C-65." Packet item at `:82`.

### 7.5 What a change requires

- **Introducing the mark** does not change any governed text, and no governance
  text names an owner act for adding a row tag. It is an ordinary deliverable
  implementation under PRD §16.3's permitted affordances, subject to the same
  code-touching evidence obligations as the other items. The owner act the
  packet seeks is therefore a **ruling on classification**, not an amendment:
  whether the mark sits inside PRD §16.3 (`{PROJ}/docs/PRD.md:1060-1070`) and
  outside PRD §21.3 (`:1339-1343`) and `OPS-K-AUTH-2`
  (`{PROJ}/docs/CONTRACT.md:30`). That is recordable as a `D-XX` packet and
  ruling with a `DEC` row, per
  `{PROJ}/execution/_Coordination/_DECISIONS/_REGISTER.md:6-15`.
- **If the mark were classified as a human acceptance record**, five
  requirements attach immediately: the five bullets of
  `{PROJ}/docs/PROFESSIONAL_BOUNDARY.md:121-127`, plus hash binding under
  `OPS-K-AUTH-2` with "Audit manifest; hash checks" as its enforcement, plus
  separation from solver/rule/comparison/report outputs (PRD §21.3), plus the
  §21.3 rule that MVP "shall not require a formal acceptance workflow". The
  `ENGINEER_ACCEPTED` label is reserved for exactly that case
  (`{PROJ}/docs/claims_registry.md:91-92`) and "no acceptance workflow exists in
  MVP".
- **The staleness behaviour** the design proposes ("bound to this row's content",
  stale on change) mirrors `OPS-K-AUTH-2`'s "do not survive content changes
  without re-review" and `PROFESSIONAL_BOUNDARY` §7's fourth bullet
  (`:125-126`), without being required by them, since neither applies to a tag
  that is not an acceptance record.
- **Wording is constrained regardless of classification:**
  `{PROJ}/docs/PROFESSIONAL_BOUNDARY.md:98-99` (no convenience exception for UI
  labels) and the claims lint over `{PROJ}/apps/desktop/src/**`.
- **Deliverable home:** `DEL-16-03` ("User acceptance and operation audit
  trail"), `IN_PROGRESS`, with durable accepted/rejected history recorded as
  incomplete under `SOW-070`
  (`{PROJ}/execution/PKG-16_Model Operation and Agent Proposal Framework/1_Working/DEL-16-03_User acceptance and operation audit trail/_STATUS.md:3`, `:8`).
  `SOW-069`/`SOW-070` are ledger rows at `{PROJ}/docs/_Registers/ScopeLedger.csv:70-71`;
  a scope change to either is an SCA act.
- **Notice obligations:** none found.

### 7.6 Unknowns

- **TBD** — the classification itself: PRD §16.3 affordance, or acceptance
  record under §21.3/`OPS-K-AUTH-2`. Settled by the owner's ruling on packet
  item 7.
- **TBD** — what the mark may be called.
  `{RUN}/instances/ROOT/DIRECTION_DECISION_2026-09-17.md:82` asks "and what it
  may be called"; `{RUN}/instances/DESIGN-SYSTEM/DESIGN_SYSTEM_V1.md §7.3` and
  `§8` defer the tooltip wording to the same item. Settled by the owner's
  ruling.
- **TBD** — `PB-TBD-002`, "Exact storage and invalidation workflow for human
  acceptance records", owner "Future persistence/report/governance deliverables"
  (`{PROJ}/docs/PROFESSIONAL_BOUNDARY.md:173`). If the mark is classified as an
  acceptance record, this open TBD becomes its gating dependency.

---

## Item 8 — agent feedback on the Review page

### 8.1 Governed text, verbatim

**(a) C-77, the agent-output row** — `PROFESSIONAL_BOUNDARY` §3,
`{PROJ}/docs/PROFESSIONAL_BOUNDARY.md:57`:

> | Agent output | Drafts, proposals, evidence summaries, checks, and open
> issues. | That agent output is accepted policy, accepted engineering work, or
> professional approval without a human gate. |

and `OPS-K-AGENT-4`, `{PROJ}/docs/CONTRACT.md:50`:

> Agent outputs are drafts/proposals until accepted by a human gate. | Review;
> lifecycle state

The five permitted classes are therefore, verbatim: **drafts, proposals,
evidence summaries, checks, and open issues.**

Adjacent: `OPS-K-AGENT-1` (`{PROJ}/docs/CONTRACT.md:47`) "Agents must not invent
engineering values, scope, source citations, or legal conclusions; unknowns
become `TBD`" and `OPS-K-AGENT-2` (`:48`) "Agents must surface conflicts and
gaps rather than silently resolving them."

**(b) PRD §11.8, FR-AGENT-001 to 005**, `{PROJ}/docs/PRD.md:537-541`:

| ID | Requirement | Priority | Acceptance Criteria |
|---|---|---|---|
| FR-AGENT-001 | Represent agent output as proposed operations | Should | Agent proposals are structured operations, not direct unsupervised model mutations. |
| FR-AGENT-002 | Validate agent operations | Should | Proposed operations must pass schema and constraint validation before application. |
| FR-AGENT-003 | Require user acceptance | Should | Agent-proposed changes are shown as diffs and require user acceptance unless the user configures another workflow. |
| FR-AGENT-004 | Record rationale and assumptions | Should | Accepted operations record rationale, constraints considered, and unresolved assumptions. |
| FR-AGENT-005 | Preserve professional boundary | Must | Agent output cannot claim engineering acceptance, certification, or code compliance for reliance. |

**(c) The BS-ACCEPT surface class** — `{PROJ}/docs/claims_registry.md:46-47`:
"results, rule-check, comparison, solve, report-preview surfaces; **any place a
computed outcome is shown**." There is no named "Review page" surface class in
any governed text.

### 8.2 What enforces it — 10 points

| # | Enforcement point | File and line | What exactly it checks |
|---|---|---|---|
| 1 | `OPS-K-AGENT-4` | `{PROJ}/docs/CONTRACT.md:50` | Drafts/proposals until a human gate; enforcement "Review; lifecycle state" |
| 2 | `OPS-K-AGENT-1`, `OPS-K-AGENT-2` | `{PROJ}/docs/CONTRACT.md:47-48` | No invented values; unknowns become `TBD`; conflicts and gaps surfaced |
| 3 | `PROFESSIONAL_BOUNDARY` §3 agent row | `:57` | The five permitted classes and the three barred statements |
| 4 | PRD §11.8 | `{PROJ}/docs/PRD.md:537-541` | The five FR-AGENT requirements; 005 is priority **Must** |
| 5 | `PROFESSIONAL_BOUNDARY` constant in the rationale guard | `{PROJ}/core/model_operations/agent_rationale/engine.py:19-30` | Nine booleans fixed at construction: `human_review_required: True`, `agent_output_is_decision_support_only: True`, and seven `False` flags including `software_can_accept_engineering_work` and `software_can_mutate_accepted_model_state` |
| 6 | `PROHIBITED_CLAIM_PATTERNS` | `{PROJ}/core/model_operations/agent_rationale/engine.py:43-74`, applied at `:372-381` | Seven compiled regexes scan agent rationale text and copied audit context: `AUTHORITY-COMPLIANCE` (`:44-47`), `AUTHORITY-CERTIFICATION` (`:48-51`), `AUTHORITY-SEALING` (`:52-55`), `AUTHORITY-AUTHENTICATION` (`:56-59`), `AUTHORITY-PROFESSIONAL-APPROVAL` (`:60-64`), `AUTHORITY-EXTERNAL-VALIDATION` (`:65-68`), `AUTHORITY-AUTONOMOUS-ACCEPTANCE` (`:69-73`). A match sets `rationale.status = "blocked_by_professional_boundary"` (`:151`) |
| 7 | `{PROJ}/tests/test_agent_rationale_boundary.py` | 8 test functions; `:214-231` and `:234-258` are the boundary cases | All seven `RATIONALE-AUTHORITY-*-BLOCKED` codes fire on a prohibited rationale, and again when the text arrives via copied audit context; `:163-166` and `:187-191` assert `decision_support_only`, `creates_accepted_operation_record: False`, `mutates_accepted_model_state: False`, `bypasses_user_acceptance: False`, `software_can_accept_engineering_work: False` |
| 8 | BS-ACCEPT placement rule | `{PROJ}/docs/claims_registry.md:46-47` | Whether the Review page is in the bound surface class turns on "any place a computed outcome is shown" |
| 9 | `DEC-042` / `DEC-091` live-binding hold | `C_ui_constraints.md:76`, `:85` | "No live agent binding, no R7 scope adoption"; the automation-condition mechanism "remains unresolved" — an agent seam may be designed, a live provider binding may not ship |
| 10 | `DEL-16-04` Remaining | `{PROJ}/execution/PKG-16_Model Operation and Agent Proposal Framework/1_Working/DEL-16-04_Agent rationale and professional-boundary controls/_STATUS.md:7-9` | The persisted-report workflow "does not bind agent rationale into downstream runtime integrations or infer human/professional approval"; the FR-AGENT-005 hard gate is stage-gated to v0.2 R7; `PKG16-DEL1604-PKG02-001` carries `HumanDisposition=TBD` |

### 8.3 Where it appears today

| Item | Live lines | Detail |
|---|---:|---|
| `OPS-K-AGENT-4` cited by ID | 2 lines in 1 file | `{PROJ}/docs/CONTRACT.md:50`, `:61` (enforcement map row "Human review gate") |
| `FR-AGENT-001`…`005` | 5 lines in 1 file | `{PROJ}/docs/PRD.md:537-541`; the scope-ledger rows `SOW-069`/`SOW-070` at `{PROJ}/docs/_Registers/ScopeLedger.csv:70-71` carry the same substance without the FR IDs |
| Runtime guard | 1 file | `{PROJ}/core/model_operations/agent_rationale/engine.py` |
| Tests | 1 file, 8 tests | `{PROJ}/tests/test_agent_rationale_boundary.py` |
| `professional_boundary` booleans in agent surfaces | registered placements | `{PROJ}/apps/desktop/src/features/diff-preview/DiffPreviewPanel.tsx:110`, `:345`; `{PROJ}/apps/desktop/src/features/operations/OperationLedgerPanel.tsx:436` ("review-only; requires explicit user acceptance; does not mutate accepted model state; acceptance and professional judgment…") |
| Historical record — must not change | — | `DEC-042`, `DEC-091` and their D-21/D-58 packets under `{PROJ}/execution/_Coordination/_DECISIONS/` and `{PROJ}/execution/_Decomposition/SOFTWARE_DECOMP.md` §12 |

**No "Review page" exists in the product today.** There is no surface whose
class the registry would have to name; the question is prospective.

### 8.4 Where the redesign puts it

- **Agent panel Checks tab:** `{RUN}/instances/DESIGN-SYSTEM/DESIGN_SYSTEM_V1.md §5.4`
  — "The agent's checks and open issues on the engineer's work, by reference:
  each is a card with the class ('Check' or 'Open issue'), the referent as a
  link …, the text, and a state the engineer sets (Open, Resolved). Checks never
  alter a table and never use the words the boundary forbids (§7.2). The same
  stream appears in the Review page's third column."
- **Review page comment stream:**
  `{RUN}/instances/DESIGN-SYSTEM/DESIGN_SYSTEM_V1.md §5.5` — one stream ordered by
  referent then time, filter row (All, Checks, Open issues, Resolved, Mine),
  "Comments attach by reference to rows, results and report text and never alter
  the tables."
- **Acceptance sentence on the Review page:**
  `{RUN}/instances/DESIGN-SYSTEM/DESIGN_SYSTEM_V1.md §5.5` — once at the top of
  the content column, "as the page's one placement (§7.4). It does not repeat in
  any section"; disclosure-home row at `§7.4` marks it "(Review surface class,
  packet item 8)".
- **Sign-off block:** `{RUN}/instances/DESIGN-SYSTEM/DESIGN_SYSTEM_V1.md §5.5` —
  "it is the report's review/sign-off block (M-07) and the product neither fills
  nor labels it as accepted."
- **Copy rules:** `{RUN}/instances/DESIGN-SYSTEM/DESIGN_SYSTEM_V1.md §7.1` —
  agent output allowed "check, open issue, proposal, draft, evidence summary,
  TBD"; forbidden "accepted, approved, verified, correct, compliant". `§7.1`
  govern proposal controls and proposal standing.
- **Proposal card standing:** `{RUN}/instances/DESIGN-SYSTEM/DESIGN_SYSTEM_V1.md §5.4`
  — "The words 'draft until accepted' are the M-14 standing and always appear."
- **Frames:** `{RUN}/instances/MOCKS/frames/s9_table_light.html` and
  `s9_table_dark.html` (Review page, three columns, comment stream, acceptance
  sentence once at the top); `s8_table_light.html` (proposal landing as banded
  rows).
- **Why it is asked:** `{RUN}/instances/ROOT/DIRECTION_DECISION_2026-09-17.md:72`
  and `:74`; packet item at `:83`.

### 8.5 What a change requires

- **Confirming the permitted output classes** changes no governed text: the five
  classes are already written at `{PROJ}/docs/PROFESSIONAL_BOUNDARY.md:57`. The
  owner act sought is a **confirmation of the wording envelope** against that
  row — recordable as a `D-XX` ruling with a `DEC` row. Note that
  `{PROJ}/docs/PROFESSIONAL_BOUNDARY.md` carries `status: draft` (`:4`) and
  `PB-TBD-005` leaves its "Final acceptance or revision" with the human project
  authority (`:176`).
- **Naming the Review page as a surface class for BS-ACCEPT** is a registry
  change to the placement clause `{PROJ}/docs/claims_registry.md:46-47` — ruled
  vocabulary under `DEC-081`, so a decision act with a `D-XX` packet, register
  row and `DEC` row, per D-48's on-ruling mechanism
  (`{PROJ}/execution/_Coordination/_DECISIONS/D-48_claims_language_taxonomy.md:198-202`).
  It may not be needed: if the Review page shows computed outcomes, the existing
  clause "any place a computed outcome is shown" already reaches it, and the
  design reads it that way
  (`{RUN}/instances/ROOT/DIRECTION_DECISION_2026-09-17.md:74`). The packet is
  asking the owner to confirm that reading rather than to amend.
- **Changing FR-AGENT-005** would be a PRD amendment on the SCA path (item 3
  §3.5), and it is priority **Must** (`{PROJ}/docs/PRD.md:541`). Nothing in the
  redesign proposes to.
- **Shipping any of it live** is gated: `DEC-042` holds v0.2/R7 agent adoption,
  `DEC-091` leaves the automation-condition mechanism unresolved
  (`C_ui_constraints.md:76`, `:85`), and the FR-AGENT-005 hard gate is
  stage-gated to v0.2 R7
  (`{PROJ}/execution/PKG-16_.../DEL-16-04_.../_STATUS.md:8`). Moving any of
  those is an owner stage act, not an implementation choice.
- **Evidence obligation:** code-touching, so the `DEC-025` sweep, self-check
  including GEN-13, H4 posture, `DEC-093` viewport coverage. `DEC-093` also
  excludes "exploratory and agent-as-user testing … from all sweep surfaces"
  (`C_ui_constraints.md:56`), which bears directly on how an agent panel is
  evidenced.
- **Deliverable homes:** `DEL-16-04` (rationale and professional-boundary
  controls) and `DEL-16-02` (operation validation and diff preview);
  `DEL-16-03` for the accepted record.
- **Notice obligations:** `DEC-091` records that "Piping stays outside the
  App-harness client set" (`C_ui_constraints.md:85`), so no App-loop notice is
  triggered by design work here. A live binding would reopen that.

### 8.6 Unknowns

- **TBD** — whether the owner confirms the Review page is inside the existing
  BS-ACCEPT surface class or wants the clause amended to name it. Settled by the
  ruling on packet item 8.
- **TBD** — the exact wording envelope for checks and open issues. The governed
  text gives five class names and three prohibitions; it gives no sentence
  templates. Settled by the ruling.
- **TBD** — `PKG16-DEL1604-PKG02-001` carries `HumanDisposition=TBD` and is
  preserved as `TECHNICALLY_ADDRESSED_PENDING_HUMAN`
  (`{PROJ}/execution/PKG-16_.../DEL-16-04_.../_STATUS.md:9`). Settled by the
  owning human disposition.

---

## Item 9 — the export's name before compatibility evidence exists

### 9.1 Governed text, verbatim

**(a) C-05, permitted claims — PRD §21.1**, `{PROJ}/docs/PRD.md:1313-1323`:

> The product may claim: mechanics results computed from recorded inputs;
> user-rule checks computed from user-supplied rules and data; diagnostics,
> warnings, assumptions, and limitations; provenance, version, checksum, and
> hash records; validation and regression evidence for software behavior; report
> generation for competent human review; local-first private-data handling where
> implemented.

with `PROFESSIONAL_BOUNDARY` §4's scoping rule,
`{PROJ}/docs/PROFESSIONAL_BOUNDARY.md:78-79`: "Claims must be specific to the
feature, version, data, and evidence available. Unsupported claims remain `TBD`,
`ASSUMPTION`, or `PROPOSAL`."

**(b) M-04, the export-metadata notice** —
`{PROJ}/docs/report_notice_template.md:51-59`:

> Where a report includes design-authoring records, comparison outputs, handoff
> packages, export metadata, or external-prover references, the report must also
> state:
>
> "Design-authoring records, comparison outputs, handoff packages, export
> metadata, and external-prover references are review aids only unless a separate
> competent human acceptance record states otherwise for the bound project
> basis."

**(c) The design-engine workflow row** — `PROFESSIONAL_BOUNDARY` §3,
`{PROJ}/docs/PROFESSIONAL_BOUNDARY.md:56`: may state "Model-authoring support,
comparison views, handoff packages, export packages, warnings, assumptions, and
review aids"; must not state "That design-authoring, export, or comparison
output is professional acceptance or external solver validation."

**(d) `BS-VALID`** — `{PROJ}/docs/claims_registry.md:52-65`, belongs on
"handoff/export UI (MBF, PCF, stress-neutral, native package, external-prover
surfaces), validation manual, headless-runner evidence surfaces".

**(e) IP and interoperability constraints** —
`{PROJ}/docs/IP_AND_DATA_BOUNDARY.md:53-54` bars from the public repository
"proprietary vendor catalogs without redistribution rights" and "commercial
software examples, report templates, or benchmark files without permission";
PRD §7 non-goal 7 (`{PROJ}/docs/PRD.md:211`) records "Implement comprehensive
parsing or ingestion of external commercial stress-analysis outputs in the MVP"
as a non-goal.

### 9.2 What enforces it — 13 points

| # | Enforcement point | File and line | What exactly it checks |
|---|---|---|---|
| 1 | PRD §21.1 | `{PROJ}/docs/PRD.md:1313-1323` | The seven permitted claim classes; a compatibility claim is not among them |
| 2 | PRD §21.2 | `{PROJ}/docs/PRD.md:1325-1337` | Nine prohibited claims |
| 3 | `PROFESSIONAL_BOUNDARY` §4 scoping rule | `:78-79` | Claims specific to feature, version, data and evidence; otherwise `TBD`/`ASSUMPTION`/`PROPOSAL` |
| 4 | `PROFESSIONAL_BOUNDARY` §3 design-engine row | `:56` | Export packages as review aids; never as external solver validation |
| 5 | M-04 supplement | `{PROJ}/docs/report_notice_template.md:51-59` | The verbatim supplement, triggered by export metadata in a report |
| 6 | Schema `const false` — MBF | `{PROJ}/schemas/caepipe_mbf_export.schema.json:662` (required), `:680-682` (`"const": false`) | `professional_boundary.software_makes_caepipe_compatibility_claim` **cannot be true**. Siblings in the same required block, all `const false`: `software_makes_release_claim`, `software_makes_solver_validation_claim`, `software_makes_compliance_claim`, `software_makes_certification_claim`, `software_makes_sealing_claim`, `software_makes_approval_claim`, `software_creates_professional_reliance_record` (`:661-668`) |
| 7 | Schema `const false` — external run | `{PROJ}/schemas/caepipe_external_run.schema.json:512`, `:539` | Same flag, same constraint, on the external-harness run package |
| 8 | Producer defaults — Python | `{PROJ}/core/handoff/caepipe_mbf/package.py:66`; `{PROJ}/core/handoff/caepipe_external/run.py:52` | Both emit `"software_makes_caepipe_compatibility_claim": False` |
| 9 | Producer defaults — desktop | `{PROJ}/apps/desktop/src/features/caepipe-mbf/CaepipeMbfExportPanel.tsx:281`; `{PROJ}/apps/desktop/src/features/caepipe-external/CaepipeExternalHarnessPanel.tsx:495`, gate at `:511` | Same, and the harness panel gates on `!boundary.software_makes_caepipe_compatibility_claim` |
| 10 | Displayed boundary line | `{PROJ}/apps/desktop/src/features/caepipe-mbf/CaepipeMbfExportPanel.tsx:128-132`; `{PROJ}/apps/desktop/src/features/caepipe-external/CaepipeExternalHarnessPanel.tsx:112` | The flag's value is rendered to the user as `caepipe_compatibility=false` |
| 11 | Tests | `{PROJ}/tests/test_caepipe_mbf_export_package.py:459`; `{PROJ}/tests/test_caepipe_external_run_package.py:230`, `:252` | `assert package["professional_boundary"]["software_makes_caepipe_compatibility_claim"] is False` |
| 12 | Panel note | `{PROJ}/apps/desktop/src/features/caepipe-mbf/CaepipeMbfExportPanel.tsx:134-137` | "CAEPIPE MBF export is a local invented smoke-subset package only; target version, record subset, direct stable-ID carrying, external execution, and target result interpretation remain governed TBDs." |
| 13 | `BS-VALID` placement | `{PROJ}/docs/claims_registry.md:52-65` | Export/handoff UI carries BS-VALID; 31 lines in the live tree carry one of its short variants |

Point 6 is the strongest binding of any of the nine items: it is a JSON-Schema
`const`, not a convention. **No product surface can assert CAEPIPE compatibility
today without a schema change.**

### 9.3 Where it appears today

"caepipe", case-insensitive, live tree: **1,092 lines in 49 files**.

| Class | Detail |
|---|---|
| Registered placement — schema | `{PROJ}/schemas/caepipe_mbf_export.schema.json`, `{PROJ}/schemas/caepipe_external_run.schema.json` |
| Registered placement — core | `{PROJ}/core/handoff/caepipe_mbf/package.py`, `{PROJ}/core/handoff/caepipe_external/run.py` |
| Registered placement — desktop | `{PROJ}/apps/desktop/src/features/caepipe-mbf/CaepipeMbfExportPanel.tsx` (panel title "CAEPIPE MBF Export" at `:70`, `aria-label="CAEPIPE MBF export"` at `:67`, download filename `openpipestress-preview-caepipe-mbf-package-…json` at `:76`, version `0.1.0` at `:33`, target unit `mm` at `:36`); `{PROJ}/apps/desktop/src/features/caepipe-external/CaepipeExternalHarnessPanel.tsx` |
| Ordinary string | `{PROJ}/fixtures/caepipe_mbf/invented/caepipe_mbf_export_package.json:601`; `{PROJ}/fixtures/caepipe_external/invented/parser_only_run_package.json:260`; `.../skipped_run_package.json:175`; the three test files in §9.2 point 11 |
| Live governance surfaces | `DEL-10-03` and PKG-17 ScopeOfWork files |
| Historical record — must not change | `{RUN}/instances/RESEARCH/E_caepipe_format.md` (this run's own research return, 8 sections, TBDs at §7) |

**The other export surfaces**, each with its own panel title and its own
`professional_boundary` block:
`{PROJ}/apps/desktop/src/features/pcf-export/PcfExportPanel.tsx:84`,
`.../stress-neutral/StressNeutralExportPanel.tsx:103`,
`.../native-package/NativePackagePanel.tsx`,
`.../handoff/HandoffPanel.tsx:75`,
`.../local-fea-handoff/LocalFeaHandoffPanel.tsx:54`,
`.../export-adapter-sdk/ExportAdapterSdkPanel.tsx:60`,
`.../export-review/ExportReviewPanel.tsx:61`,
`.../result-export/ResultExportPanel.tsx:30`,
`.../redaction-controls/RedactionExportControlsPanel.tsx:73`.

**BS-VALID:** the canonical sentence appears on **no** product surface (zero
occurrences in `{PROJ}/apps`, `{PROJ}/core`, `{PROJ}/docs` outside the registry);
its two short variants appear on 31 live lines.

### 9.4 Where the redesign puts it

- **Disclosure homes:** `{RUN}/instances/DESIGN-SYSTEM/DESIGN_SYSTEM_V1.md §7.4`
  — "M-04 | The report and its preview when export metadata, hanger selection
  records or handoff data are included; named in the export dialog"; `§7.4` —
  "M-06 | The export and handoff dialog."
- **Open item 2:** `{RUN}/instances/DESIGN-SYSTEM/DESIGN_SYSTEM_V1.md §8` — "The
  export's name in the export dialog before compatibility evidence exists
  (packet item 9)."
- **Report production from the Review page:**
  `{RUN}/instances/DESIGN-SYSTEM/DESIGN_SYSTEM_V1.md §5.5` — "'Export…' opens
  export and handoff. The report is produced from this page and nowhere else";
  the report outline's Notice row names M-04 at `§5.5`. The owner adopted the
  header placement on 2026-09-18
  (`{RUN}/instances/ROOT/DIRECTION_DECISION_2026-09-17.md:160`).
- **Frames:** `{RUN}/instances/MOCKS/frames/s9_table_light.html` and
  `s9_table_dark.html` carry the "Export…" control in the page header; no frame
  shows the export dialog itself.
- **Why it is asked:** `{RUN}/instances/ROOT/DIRECTION_DECISION_2026-09-17.md:76`
  — "'Compatible with CAEPIPE' is a product claim that needs evidence before it
  is made on any surface (C-05); until then the export is described by what it
  produces." Packet item at `:84`. The format research is
  `{RUN}/instances/RESEARCH/E_caepipe_format.md`, whose §7 is a TBD list.

### 9.5 What a change requires

- **Naming the export by what it produces** — the current posture — requires no
  owner act. It is already what the product does
  (`{PROJ}/apps/desktop/src/features/caepipe-mbf/CaepipeMbfExportPanel.tsx:134-137`).
- **Asserting compatibility** requires, in order: (i) evidence that satisfies
  `{PROJ}/docs/PROFESSIONAL_BOUNDARY.md:62-63` ("may claim supported behavior
  when the claim is backed by implementation, tests, documentation, or recorded
  evidence") and `:78-79` (specific to feature, version, data and evidence);
  (ii) a schema change to lift `"const": false` on
  `software_makes_caepipe_compatibility_claim` in **two** schemas
  (`{PROJ}/schemas/caepipe_mbf_export.schema.json:680-682`,
  `{PROJ}/schemas/caepipe_external_run.schema.json:539`), which moves three test
  assertions and two producers; (iii) a check against PRD §21.1's permitted
  list, which does not include interoperability claims — so an addition to §21.1
  would be a PRD amendment on the SCA path (item 3 §3.5).
- **The M-04 trigger is automatic**, not discretionary: a report that includes
  export metadata "must also state" the supplement
  (`{PROJ}/docs/report_notice_template.md:51-53`). Changing the supplement's text
  is a change to the template that is the PRD §19.3 home; the template's own rule
  that "changes require a PRD-level owner act" is scoped at `:37-38` to the
  required notice, and the supplement sits in the same "Required Notice" section.
- **IP and interoperability constraints recorded in governance**, in full: the
  public repository must not contain "commercial software examples, report
  templates, or benchmark files without permission" or "proprietary vendor
  catalogs without redistribution rights"
  (`{PROJ}/docs/IP_AND_DATA_BOUNDARY.md:53-54`); `OPS-K-IP-1`
  (`{PROJ}/docs/CONTRACT.md:23`) repeats this as an invariant; PRD §7 non-goal 7
  (`{PROJ}/docs/PRD.md:211`) records that comprehensive parsing or ingestion of
  external commercial stress-analysis outputs is out of MVP scope; PRD §7
  non-goal 2 (`:206`) bars standards-body approval claims. `DEC-024`/§22.5 make
  the external prover "the canonical validation oracle" with owner-gated
  activation and prover metadata that is "not automatic professional acceptance"
  (`C_ui_constraints.md:46`).
- **Deliverable homes:** `DEL-10-03` (Local FEA handoff data contract) and
  PKG-17 (Export Format Interoperability) carry the export surfaces;
  `{RUN}/instances/RESEARCH/E_caepipe_format.md` is the format research basis.
- **Notice obligations:** none found outside this project loop.

### 9.6 Unknowns

- **TBD** — what the export may be called. No candidate name exists in any
  governed text or in the design system;
  `{RUN}/instances/DESIGN-SYSTEM/DESIGN_SYSTEM_V1.md §8` records it as open.
  Settled by the owner's ruling on packet item 9.
- **TBD** — what would count as compatibility evidence. Nothing in governance
  defines an interoperability evidence standard; `{PROJ}/docs/PRD.md` §22.5
  defines an external-**prover correlation** standard, which is a different
  claim. Settled by an owner-recorded evidence standard, or by an SCA that adds
  an interoperability claim class to PRD §21.1.
- **TBD** — the format-level unknowns that would gate any claim are enumerated
  at `{RUN}/instances/RESEARCH/E_caepipe_format.md` §7 (§7 begins at line 621 of
  that file). Settled by the work that closes them.
- **TBD** — whether any CAEPIPE trademark or licence constraint attaches to
  naming an export after the target tool. No governance record addresses it;
  `PB-TBD-004` leaves the legal-review authority `TBD`
  (`{PROJ}/docs/PROFESSIONAL_BOUNDARY.md:175`). Settled by the human project
  authority recording that authority, or by an explicit owner statement.

---

## Summary table

"Enforcement points" counted as defined in §0.2. "Occurrences" are lines across
the whole repository, with the live subset given in parentheses; "live" excludes
the historical paths listed in §0.1.

| # | Item | Enforcement points | Occurrences (live) | Kind of owner act a change requires |
|---:|---|---:|---|---|
| 1 | Maturity line "Technical preview — not a released product." | 7 | 128 (9) | Registry change to ruled `DEC-081` vocabulary: `D-XX` packet, register row, `DEC` row. Changing the sentence itself additionally needs a stage-record act (the `_COORDINATION.md` target-stage ruling pattern). Not an SCA. |
| 2 | Acceptance sentence (`BS-ACCEPT`) and its placements | 14 | 609 (102; 53 in non-test product source across 21 files; 26 on live `ScopeOfWork.md`) | Registry change to the placement clause: `D-XX` packet, register row, `DEC` row. No PRD amendment (D-48 §3.4). Product edits are code-touching tranche work. |
| 3 | Product name "OpenPipeStress" and the rename | 13 | 101,493 (1,873) for the name; 87,067 (1,485) lowercase; `SWBPIPE` 371, all in AgentRuns, 0 in product | Three acts: **PRD-level owner act via the SCA path** for the §19.3 notice (SCA-007 precedent); registry act for `BS-IP`; packaging/schema-identity act for bundle IDs, `$id` URLs and 61 document-kind constants. |
| 4 | Short labels for six statuses and two evidence labels | 14 | six tokens 9,209 (1,124); evidence labels 176 (14, none in product source); short-label phrases 28 live lines over `apps/desktop/src`, `core`, `schemas`, `fixtures`, `docs` (6 in non-test desktop source, none a hand-written status label) — but two curated labels are rendered today, "Review required" and "Inputs needed" (`App.tsx:3768-3769`), and the other four tokens are rendered by a de-underscoring transform | Registry act to create listed variants (`DEC-081` vocabulary). Changing the token set is a `TYPES.md` §4 amendment (D-39 precedent) reaching 16 schemas and 17 test files. Short labels without registry change are ordinary microcopy, but seven Vitest assertions pin the displayed text. |
| 5 | Vendor hanger tables as a library class; vocabulary item 23 | 10 | "hanger" 2,283 (81 files) live; `BS-IP` canonical on 0 product surfaces | **Confirmation ruling**, not an amendment: a `D-XX` ruling and `DEC` row confirming the M-05 framing. Tier or scope change to item 23 is an SCA or owner-ruled deliverable amendment. Bundling a vendor catalog is barred outright. |
| 6 | Historical-run wording (M-13) | 6 | 6 (6), in 2 files, 0 in governance | **No owner act is named.** Ordinary code-touching deliverable work under the claims boundary and the lint. The packet question is which of two existing texts the owner means. |
| 7 | The Checked mark | 9 (all governance; 0 in code) | 0 in the product | **Classification ruling**: a `D-XX` packet and `DEC` row placing it inside PRD §16.3 and outside PRD §21.3 / `OPS-K-AUTH-2`. If classified as an acceptance record, `PROFESSIONAL_BOUNDARY` §7's five requirements and `PB-TBD-002` attach. |
| 8 | Agent feedback on the Review page; Review as a surface class | 10 | `FR-AGENT-00x` 5 lines; `OPS-K-AGENT-4` 2; 1 runtime guard with 7 blocking patterns; 8 tests; no Review page exists | **Confirmation ruling** on the wording envelope and on the surface class. A registry act only if the placement clause must name the Review page. Changing FR-AGENT-005 (priority Must) would be a PRD amendment via SCA. Shipping live remains gated by `DEC-042`/`DEC-091`. |
| 9 | The export's name before compatibility evidence | 13 | "caepipe" 1,092 (49 files); the compatibility flag is `"const": false` in 2 schemas, 2 producers, 2 panels, 3 tests | Naming by output: **no act**. Asserting compatibility: recorded evidence, a schema `const` change in two schemas, and — since PRD §21.1 does not list an interoperability claim — a **PRD amendment via the SCA path**. |

Totals: **96 enforcement points**; **213,239 occurrence lines** repo-wide across
the nine items, of which roughly **4,850** are live and the remainder are
historical records that must not change.

---

## Return

### Correction after ROOT's check

**What was wrong.** The first version of this return stated that "Human review
required" had **zero** occurrences under `{PROJ}/apps/desktop/src`, and built on
that a broader claim that the product renders raw status tokens with no short
label. Both were false. The cause was a search fault, not a reading fault: the
sweep for the six short-label phrases was issued with a working directory
already inside `{PROJ}` **and** a path argument that repeated the project prefix,
so every path was nonexistent; `2>/dev/null` swallowed the resulting error and
the command reported a clean zero. The re-run uses `rg -i` from a single known
working directory, over `apps/desktop/src`, `core`, `schemas`, `fixtures` and
`docs`, matching each phrase case-insensitively as a substring so that a label
inside a longer string is caught.

**What the corrected search finds.** 28 live lines across the six phrases —
"Model incomplete" 3, "Mechanics solved" 12, "Rule inputs incomplete" 3, "User
rules checked" 0, "User rule failed" 0, "Human review required" 10 — of which 6
are in non-test desktop source. None of the 6 is a hand-written status label:
five are prose or ordinary strings carrying the phrase (§4.3 table) and one is
the Solve panel's readiness sentence. The two occurrences ROOT cited,
`{PROJ}/apps/desktop/src/features/report/reportPackageRequest.ts:268` and
`{PROJ}/apps/desktop/src/features/report/renderableReportInput.ts:109`, are both
classified here as ordinary strings — diagnostic `remediation` values.

**What the re-run additionally exposed.** Following the corrected hits into the
rendering path showed that the substantive claim was also wrong, and in a way
the phrase search alone would not have revealed, because the displayed strings
are computed rather than written. The product renders status text through three
mechanisms (§4.3): `readableWorkspaceStatus` (`{PROJ}/apps/desktop/src/App.tsx:3766-3773`),
which curates **"Review required"** for `HUMAN_REVIEW_REQUIRED` and **"Inputs
needed"** for `RULE_INPUTS_INCOMPLETE`; two duplicate `formatStatus` helpers,
which lowercase and de-underscore; and one raw-token prose line. So short labels
for two of the six tokens exist in the product today, and they are not the
labels the design system proposes.

**What changed in this file.** §0.3 row D-7 (rewritten); §4.2 (enforcement point
14 added — the seven Vitest assertions that pin displayed status text — and the
section count raised from 13 to 14); §4.3 (the "Short labels" block replaced);
§4.5 (the "Short labels alone" bullet); §4.6 (two TBDs revised or added); the
summary table's row 4 and its enforcement-point total, 95 to 96; uncertainty 8;
and this note. Nothing else in the file was touched, and no other file was
modified.

### What was read

**Sealed brief:** `{RUN}/briefs/RESEARCH-F_packet_bindings.md` (56 lines, in
full, before any other file).

**Accepted basis — the extraction to verify:**
`{RUN}/instances/RESEARCH/C_ui_constraints.md` (328 lines, in full).

**Governance sources, in full:** `{PROJ}/docs/claims_registry.md` (134),
`{PROJ}/docs/CONTRACT.md` (67), `{PROJ}/docs/PROFESSIONAL_BOUNDARY.md` (176),
`{PROJ}/docs/report_notice_template.md` (123),
`{PROJ}/docs/IP_AND_DATA_BOUNDARY.md` (113),
`{PROJ}/execution/_Coordination/_DECISIONS/_REGISTER.md` (137).

**Governance sources, by section:** `{PROJ}/docs/PRD.md` §4.3 (`:100-122`), §5.1
(`:127-131`), §7 (`:201-227`), §11.8/§11.9 (`:533-551`), §16.3 (`:1060-1079`),
§19 (`:1199-1254`), §21 (`:1311-1343`), §22.1 (`:1349-1355`), appendix
(`:1786`); `{PROJ}/docs/SPEC.md` §4.3 (`:313-334`), §12 (`:925-934`);
`{PROJ}/docs/TYPES.md` §4-§5.1 (`:52-100`); `{PROJ}/docs/DIRECTIVE.md` §3
principle 6 (`:65`).

**Rulings and scope changes:**
`{PROJ}/execution/_Coordination/_DECISIONS/D-48_claims_language_taxonomy.md`
(§3.3, §3.4, §4-§9, `:99-245`);
`{PROJ}/execution/_Decomposition/SOFTWARE_DECOMP.md` rows `DEC-049` (`:640`),
`DEC-081` (`:672`), `DEC-094` (`:685`);
`{PROJ}/execution/_ScopeChange/_LATEST.md`;
`{PROJ}/execution/_ScopeChange/SCA-009_2026-08-20_0000/Vocabulary_Annex.md`
(`:50-80`, `:111`, `:127`); `{PROJ}/execution/_ScopeChange/SCA-007_2026-07-16_2026/`
(directory listing and `ACCEPTANCE_RECORD.md:1-25`);
`{PROJ}/execution/PKG-16_.../DEL-16-03_.../_STATUS.md` and
`DEL-16-04_.../_STATUS.md`.

**Enforcement:** `tools/validation/validate_claims_language.py` (366 lines, in
full) and `tools/validation/test_validate_claims_language.py` (191 lines, in
full); `tools/practitioner_harness/cmd_self_check.py:905-926`;
`{PROJ}/tests/test_user_guide_status_wording.py` (in full);
`{PROJ}/tests/test_agent_rationale_boundary.py` (assertions);
`{PROJ}/tests/test_operation_result_schemas.py:165-190`;
`{PROJ}/tests/test_hanger_library_schema.py` (test names);
`{PROJ}/tests/test_caepipe_mbf_export_package.py` and
`test_caepipe_external_run_package.py` (boundary assertions); the
`HUMAN_APPROVED_FOR_PROJECT` exclusion assertions across 10 test files.

**Product source:** `{PROJ}/apps/desktop/src/App.tsx` (footer region, status
region), `App.test.tsx:7128-7160`;
`features/results/HistoricalRunContext.tsx` (`:1-40`, `:315-327`);
`features/rule-check/RuleCheckPanel.tsx:92-104`;
`features/build-readiness/BuildReadinessPanel.tsx:60-90`;
`features/export-review/ExportReviewPanel.tsx:650-665`;
`features/caepipe-mbf/CaepipeMbfExportPanel.tsx`;
`features/hanger-selection/HangerSelectionPanel.tsx`;
`{PROJ}/core/reporting/report_renderer/src/lib.rs:30-55`, `:695-745`;
`{PROJ}/core/rules/rule_check_runner/src/lib.rs:78-82`;
`{PROJ}/core/model_operations/agent_rationale/engine.py:19-80`, `:372-381`;
`{PROJ}/core/library_import/library_import_document/src/lib.rs:45-62`;
`{PROJ}/core/library_import/provenance_checker.py` (dispatch and hanger
validation); `{PROJ}/apps/desktop/src-tauri/tauri.conf.json`,
`{PROJ}/apps/desktop/index.html`.

**Schemas:** `{PROJ}/schemas/analysis_status.schema.yaml`,
`analysis_boundary.schema.yaml`, `model_state.schema.json`,
`rule_check_run_result.schema.json`, `caepipe_mbf_export.schema.json`,
`caepipe_external_run.schema.json`, `hanger.schema.yaml`; plus the 16-file enum
carriage list and the 46-file `$id` namespace sweep.

**Run context:** `{RUN}/instances/ROOT/DESIGN_BRIEF_V1.md` §6 (`:160-173`);
`{RUN}/instances/ROOT/DIRECTION_DECISION_2026-09-17.md` §5, §6, §7, §11
(`:66-95`, `:143-168`);
`{RUN}/instances/CONCEPTS/CONCEPT_DIRECTIONS_V1.md` §6 (`:309-325`);
`{RUN}/instances/DESIGN-SYSTEM/DESIGN_SYSTEM_V1.md` §2.3–§2.6, §4, §5.4–§5.6
and §7–§8, first at its 843-line V1 state and again at the 935-line V1.1 state
that the concurrent `DESIGN-SYSTEM-02` revision produced (§0 and uncertainty 1a);
the V1.1 status line and change list at `:3`;
`{RUN}/instances/MOCKS/MOCKS_V1.md` §4-§5 (`:142-171`);
`{RUN}/instances/RESEARCH/E_caepipe_format.md` (outline and §7 location); all
fifteen frames plus `index.html` under `{RUN}/instances/MOCKS/frames/`
(searched, with contexts extracted for the maturity line, the acceptance
sentence, the status chips, the evidence chip and the Checked mark).

**Searches run:** repo-wide, case-sensitive and case-insensitive, over `{PROJ}`
and `tools/`, excluding `node_modules` and build `target` directories, for: the
five `BS-ACCEPT` texts, the three `BS-IP` texts, the two `BS-VALID` short
variants, the `BS-MATURITY` sentence, the `GF-TOKEN` sentence, the PRD §19.3
notice and its lint fragment, the M-04 supplement, `OpenPipeStress` /
`openpipestress` / `open_pipe_stress`, `SWBPIPE` / `SWB Piping Designer`, the
six status tokens plus `HUMAN_APPROVED_FOR_PROJECT`, the three evidence labels,
`historical_saved_run`, `Checked`, `hanger`, `caepipe`,
`software_makes_caepipe_compatibility_claim`, `professional_boundary_notice`,
`FR-AGENT-00`, `OPS-K-AGENT-4`, `GEN-13`, `MISSING_*` lint codes. After ROOT's
check, re-run case-insensitively as substrings over `{PROJ}/apps/desktop/src`,
`{PROJ}/core`, `{PROJ}/schemas`, `{PROJ}/fixtures` and `{PROJ}/docs`: the six
short-label phrases, then `readableWorkspaceStatus`, `formatStatus`,
`StatusPill`, "Review required" and "Inputs needed".

`tools/validation/validate_claims_language.py` was executed read-only against
the worktree; it writes nothing and reported `VALID … 320 files scanned`.

### Model and effort actually used

Claude Opus 5 (1M context), model id `claude-opus-5[1m]`, running as a Claude
Code `Agent` subagent, general-purpose type, in the ROOT worktree. This matches
the model the brief requested (`RESEARCH-F_packet_bindings.md:3`). Effort: one
session plus a correction pass after ROOT's check, on the order of sixty tool
calls, no delegation (Type 2 does not delegate), no parallel children. Read-only throughout except for this single
return file; no git operation of any kind was run; no file outside
`{RUN}/instances/RESEARCH/F_packet_bindings.md` was created or modified.

### Uncertainties

1. **Occurrence counts over `{RUN}` are a moving target.** The worktree has
   uncommitted changes under `{RUN}` at HEAD, and the run's own artifacts contain
   several of the searched strings. Two counts taken minutes apart differed by
   one line for two patterns. Counts confined to `{PROJ}/apps`, `{PROJ}/core`,
   `{PROJ}/schemas`, `{PROJ}/fixtures`, `{PROJ}/docs` and `tools/validation` are
   stable and are the ones the summary table leans on.

1a. **The design system was rewritten under this inventory, three times, and
   is probably still moving.** `{RUN}/instances/DESIGN-SYSTEM/DESIGN_SYSTEM_V1.md`
   was 843 lines (V1) when the "where the redesign puts it" subsections were
   written, then 915, then 935, then 993 lines as a concurrent session executed
   the sealed brief `DESIGN-SYSTEM-02`; `{RUN}/instances/UX-SPEC/` appeared
   during the pass as well. Text changed, not only position: at the 993-line
   state the §5.5 header example reads "Run 04" where it read "Run 03", the §7.4
   M-02 row has gained a clause about the run, and §8's open items are
   renumbered. Because of that, design-system citations here are section plus
   quotation, not `file:line` (§0). Every quoted sentence was verified present at
   the 935-line state; the three passages that had already changed by then are
   recorded — the new chip-display policy at `DESIGN_SYSTEM_V1.md §2.3` (quoted
   in §4.4), the §4 marks table's historical row now also naming
   `rail.captionHistorical`, and the §4 state-slot priority order now inserting
   "comment" before "note". A quoted design-system sentence may have been revised
   again after this return was written; the design system's own acceptance, not
   this inventory, is where that is settled. **Nothing this return depends on for
   items 1 to 9 — the governed texts, the enforcement points and the occurrence
   counts — sits in that file.** Nothing in `{PROJ}/apps`, `{PROJ}/core`,
   `{PROJ}/schemas`, `{PROJ}/docs` or `tools/validation` moved during the pass.

2. **The line-continuation problem.** Rust and Python string literals split
   across lines with `\` or implicit concatenation do not match a single-line
   grep. This was caught for the §19.3 notice in the report renderer (difference
   D-3) and for `PROFESSIONAL_BOUNDARY_NOTICE`. Other split literals carrying
   governed text may exist that these searches did not surface. The lint has the
   same blind spot in reverse: its `SUPPRESSION_WINDOW` of 15 characters
   (`tools/validation/validate_claims_language.py:93`) exists precisely to handle
   wrapped statements in Markdown, but it operates line by line, so a governed
   sentence split across source lines is never matched as a whole by the anchor
   check either — the anchor check normalises the **whole file**
   (`:307-314`), which is why `MISSING_RENDERER_NOTICE` passes.

3. **Punctuation is not normalised by the lint.** `{PROJ}/apps/desktop/src/features/missing-data/MissingDataBlockingPanel.tsx:539`
   and `{PROJ}/apps/desktop/src/features/library/LibraryManagerPanel.tsx:976`
   carry the `BS-IP` short variant with an em dash where the registry has a
   semicolon (`{PROJ}/docs/claims_registry.md:23-24`). The lint normalises
   whitespace and case only (`:166-167`), so these are not byte-verbatim uses of
   a listed variant. Whether the owner regards them as registered placements or
   as near-misses is not something this inventory can decide.

4. **"Enforcement point" counting is a judgement.** §0.2 defines it, and each
   section enumerates the points it counts, so the tables can be re-derived. A
   different granularity — counting files, or counting the 16 schemas and 17
   test files of item 4 individually — would produce different totals. The
   enumerations, not the totals, are the load-bearing part.

5. **Item 5's landed state was not traced to a decision.** `LibraryKind::Hanger`,
   `{PROJ}/schemas/hanger.schema.yaml` and the `HangerSelectionPanel` exist in
   the source, but no `DEC` row, SCA action or `DEL-07-09` coverage-ledger entry
   was located that records the extension as accepted. The SCA-009 annex still
   describes it prospectively (difference D-5). I did not run `git log` or any
   other git command (the brief bars git operations), so I cannot say when or
   under what tranche it landed. Settling that would need the deliverable's
   `_STATUS.md` history or the loop receipts, which I did not read in full.

6. **Cross-loop notice obligations were checked only within this repository.** No
   other project loop's instruction corpus or contract mirror in this repository
   pins any of the nine items' governed text. Whether any consumer outside the
   repository pins the `openpipestress.org` schema namespace or the 61
   document-kind constants is outside what a repository search can establish.

7. **The Vocabulary Annex's own status is unresolved in its header.** Its file
   header still reads `CANDIDATE NORMATIVE TEXT — GATE 3 NOT APPROVED` while
   `DEL-07-09/_STATUS.md` and `SOFTWARE_DECOMP` revision 0.12 treat it as the
   accepted coverage contract (`C_ui_constraints.md:103-105`). I verified that C
   states this correctly and did not attempt to resolve it; it bears on item 5.

8. **Item 4's placement question has no governed answer.** §4.5 records what the
   three candidate texts actually say. I did not find a governed sentence
   requiring "Human review required" on every surface; the absence of a rule is
   harder to establish than its presence, and I am reporting it as an absence
   found by search over `{PROJ}/docs`, `{PROJ}/schemas` and
   `{PROJ}/execution/_Decomposition/SOFTWARE_DECOMP.md`, not as a proof. This
   absence is separate from the occurrence claim that ROOT corrected (see
   "Correction after ROOT's check"): the corrected search changed what the
   product *renders*, not what governance *requires*, and I re-checked the three
   governed texts after the correction rather than carrying the earlier reading
   forward.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
