# D-10 — Report Rendering Target (Deterministic HTML and/or PDF Pipeline)

**Status:** AWAITING_RULING
**Prepared:** 2026-06-10 by TASK (Type 2), requested by WORKING_ITEMS (Type 1), tranche TP-APP-R2-PERSIST-001 decision-preparation subscope.
**Register row:** `execution/_Coordination/_DECISIONS/_REGISTER.md` row D-10.
**Plan basis:** `plans/PLAN_2026-06-10_prd_completion.md` §2 row D-10; §3 Phase A item A7.
**Epistemic posture:** evidence below is `FACT` with citations; inferences are labeled `ASSUMPTION`; the recommendation is labeled `PROPOSAL`; unknowns stay `TBD`. This packet decides nothing.

---

## 1. Decision statement and scope

**Decide:** the rendering target and pipeline for turning the existing deterministic calculation-report envelopes/sections into a reviewable document in the desktop app — HTML, PDF, or both — including the deterministic-rendering guarantee, hash binding, and engine choice inside the Tauri shell.

**In scope:** output format(s); rendering engine location (Rust core vs frontend vs webview); determinism and hash-binding posture of the rendered artifact; how PRD §15.2 prohibitions and the protected-content linter gate rendered output; the offline/local-only constraint.

**Out of scope:** report *content* contracts — section kinds, required slots, disclosure rules, and schemas already exist (`core/reporting/report_generator/src/lib.rs`, `core/reporting/report_sections/src/lib.rs`, `schemas/report_generator.schema.yaml`, `schemas/report_sections.schema.yaml`) and are not reopened here. Also out of scope: release packaging/signing (D-06), evidence thresholds themselves (D-04), and issuance governance (D-11).

D-10 blocks Phase A item A7 ("Render the existing deterministic report envelopes/sections to a reviewable document (per D-10)", `plans/PLAN_2026-06-10_prd_completion.md` §3 row A7) and feeds the R5 "Full report package" deliverable (`docs/PRD.md` §22.6).

---

## 2. Current state evidence

Citations are pinned to the working baseline at preparation time (repo HEAD `9f9f6cd44`; `core/reporting` last changed by `5abc6b830`). Symbol names are the durable anchors if line numbers drift.

### 2.1 What the reporting crates already provide (and what they explicitly do not)

Five crates exist under `core/reporting/` (plus `state_comparison_handoff_sections`):

- **`report_generator`** — validates bounded in-memory `CalculationReport` records: `TemplateSlot { slot_id, required, section_kind, source_contract, ordering_index }` and `RenderedSection { section_id, slot_id, section_kind, title, source_refs, content_status }` (`core/reporting/report_generator/src/lib.rs:370–404`), eight required `SectionKind`s from `ModelInputSummary` through `ProfessionalBoundaryNotice` (`lib.rs:13–22`), blocking diagnostics for incomplete slots/sections including `content_status = "TBD"` (`RenderedSection::is_complete`, `lib.rs:394–404`), and deterministic section ordering for **neutral test output** (`NeutralReportSection`, `lib.rs:458+`). Its own header disclaims rendering: "It does not … render GUI previews" (`lib.rs:1–8`).
- **`report_sections`** — validates report-facing warnings/assumptions/provenance/limitations/TBD disclosure records; header: "It does not render final reports" (`core/reporting/report_sections/src/lib.rs:1–7`).
- **`audit_manifest`** — deterministic hash inputs over project-local canonical JSON (`CanonicalJson`, sorted-key serialization) and asset bytes (`core/reporting/audit_manifest/src/lib.rs:1–25`).
- **`protected_content_linter`** — evaluates caller-supplied public report/template/example **text** against deterministic synthetic markers and professional-boundary phrases; `SurfaceKind::{PublicReportTemplate, PublicReportExample, PublicFixture, PrivateUserTemplate, PrivateProjectExport}` with public-by-default lint, blocking `FindingCode`s incl. `ProhibitedProfessionalClaim` (`core/reporting/protected_content_linter/src/lib.rs:1–40`).
- **`result_export`** — deterministic schema-first JSON result envelopes; "does not … render reports" (`core/reporting/result_export/src/lib.rs:1–8`).

**FACT:** no crate, schema, or app code in the repo emits HTML or PDF for calculation reports. The completion plan states the same: "envelopes/sections exist; rendering does not" (`plans/PLAN_2026-06-10_prd_completion.md` §3 row A7).

### 2.2 What the desktop app shows and exports today

- `apps/desktop/src/features/report/ReportPanel.tsx` renders a "Report Packet" panel of label/value lines and a **local JSON download** (`data:application/json` href, `jsonDataHref`, `ReportPanel.tsx:80–95, 692–694`). The export packet is `document_kind: "openpipestress.technical_preview.report_packet_export"`, `export_scope: "local_browser_download_preview"`, with hash refs, boundary blocks, and `private_payload_included: false` (`ReportPanel.tsx:316–388`). It is a preview evidence packet, not a calculation report document.
- `apps/desktop/src/features/report-lint/ReportLintPanel.tsx` already runs a report-content lint packet in the GUI with its own JSON download (`ReportLintPanel.tsx:1–32`), i.e. a lint enforcement surface exists app-side.
- The shell is Tauri 2 (`tauri = { version = "2" }`, `apps/desktop/src-tauri/Cargo.toml:21`), so an OS webview is already part of the product; the frontend is React/TS rendered in that webview.

### 2.3 Schema and PRD requirements the rendered document must satisfy

- `schemas/report_generator.schema.yaml` fixes the report assembly contract (DEL-08-01/PKG-08, required `calculation_report`, `report_generator_status`); its description disclaims runtime rendering behavior (`:5`). `schemas/report_sections.schema.yaml` likewise requires `report_renderer_status` and `report_sections` and "does not render final reports" (`:5–16`).
- PRD §15.1 enumerates ~30 required report contents (project name through user signoff block) (`docs/PRD.md:839–874`).
- PRD §15.2: public-template reports must not reproduce protected ASME/standards content; private templates are the user's responsibility (`docs/PRD.md:876–878`).
- PRD §15.3: reports must carry metadata sufficient to reproduce results (`docs/PRD.md:880–882`).
- FR-016 (Must): "Generate calculation reports — Report includes input summary, assumptions, rule-pack checksum, warnings, results, and provenance" (`docs/PRD.md:349`). Plan status row: "Deterministic envelopes; no rendered document" (`plans/PLAN_2026-06-10_prd_completion.md` §status table, FR-016 row).
- R2 deliverable "Basic report generator"; exit criterion "create, solve, and report … without editing raw files" (`docs/PRD.md` §22.3). R5 deliverable "Full report package" (`docs/PRD.md` §22.6).
- **FACT:** the PRD nowhere mandates HTML or PDF; the only "pdf" match in `docs/PRD.md` is a bibliography filename (`docs/PRD.md:1416`). The format is genuinely open — hence this decision.

### 2.4 The rendering target is a long-declared TBD

- DEL-08-01 `Guidance.md` open question OQ-08-01-001: "Exact renderer library, file format, and report template language. — TBD" (`execution/PKG-08_Reporting, Audit, and Reproducibility/1_Working/DEL-08-01_Calculation report generator/Guidance.md:64`; same posture at `Guidance.md:21` and run records `_run_records/TASK_RUN_2026-04-30_1200_four-documents-p1-p2.md:42`).

### 2.5 Boundary constraints on whatever renders

- Local-only posture is already evidenced product behavior: the report export boundary block asserts `local_only_project_store: true`, `network_required` surfaced explicitly (`ReportPanel.tsx:495–507`). The brief for this packet prohibits proposing any cloud/network/telemetry-dependent path.
- `docs/IP_AND_DATA_BOUNDARY.md` §2–3: public repo may contain blank templates and report schemas; must not contain standards text/tables or commercial report templates without permission (`IP_AND_DATA_BOUNDARY.md:36–52`). Any bundled public report template is a lintable public surface (§2.1 linter `SurfaceKind::PublicReportTemplate`).
- Status vocabulary constraint: A7 must not introduce compliance language; software emits only the six automatic statuses (`plans/PLAN_2026-06-10_prd_completion.md` §"Status vocabulary"; mirrored by `AnalysisStatus` enums in all reporting crates).

---

## 3. Open questions awaiting ruling

1. **Format:** HTML only, PDF only, or HTML now + PDF later? FR-016/R2 need *a* reviewable document; R5's "full report package" is the natural forcing point for a print-grade artifact. `TBD` whether the human requires PDF as early as R2.
2. **Deterministic-rendering guarantee and hash binding:** is the rendered document itself a hash-bound evidence artifact (byte-identical output for identical inputs, hash recorded alongside the existing envelope hashes per `audit_manifest` canonical hashing, §2.1), or is the JSON envelope the sole canonical artifact with the rendered document a derived, best-effort view? This determines whether the renderer must live where byte determinism can be controlled.
3. **Engine location inside Tauri:** (a) Rust-side renderer in `core/reporting` emitting bytes directly (deterministic, testable with `cargo test`); (b) frontend/webview DOM rendering with OS print path (`window.print()`) for paper/PDF; (c) webview print-to-PDF API capture. ASSUMPTION: webview print output (margins, fonts, pagination) varies across OS/webview versions, so paths (b)/(c) cannot promise byte-identical PDFs; a Rust-side emitter can. `TBD`: no measurement of cross-platform webview print variance exists in-repo.
4. **Offline/local-only details:** fonts and assets must be bundled or system-default — no CDN/network fetches in templates. Self-contained single-file output (inline CSS, no external refs) vs multi-file report directory. `TBD` preferred container until ruled (interacts with D-09 single-file project container).
5. **Linter gating point:** the linter takes caller-supplied text (§2.1). Where does it run against *rendered* output — on the template (public surface, pre-bundling), on the assembled section text (pre-render), on the final document text (post-render, pre-export), or all three? `TBD`; §5 proposes all three with the post-render gate blocking export.
6. **Template authority:** does the public bundled template remain the only software-rendered template for now, with PRD §15.2 "private report templates" deferred? `TBD` — private-template rendering is not named in Phase A.
7. **D-04 interaction:** report evidence wording for tolerances/coverage cannot be finalized until D-04 rules thresholds; rendering must therefore treat threshold statements as data-driven slot content, not template prose. (`_REGISTER.md` row D-04.)

---

## 4. Options

All options are local-only: bundled assets, no network, no telemetry, per brief and §2.5.

### Option A — Deterministic HTML as the canonical rendered artifact; OS print path as a non-canonical convenience

A Rust renderer (new `core/reporting/report_renderer` crate) consumes the validated `CalculationReport` + section envelopes and emits a **single self-contained HTML file** (inline CSS, bundled-or-system fonts, no scripts, no external refs) with a deterministic byte layout; its SHA-256 joins the existing hash evidence (§2.1 `audit_manifest`). The desktop app displays it (the webview already renders HTML) and offers "save HTML" plus the OS print dialog for paper/PDF, explicitly labeled a derived, non-hash-bound view.

- For: one deterministic artifact, testable byte-for-byte in `cargo test` with golden files; zero new heavyweight dependencies; HTML is reviewable, diffable, archivable, and lint-able as text (§2.1 linter takes text); fastest path to the R2 exit criterion; webview display needs no new engine.
- Against: no hash-bound PDF — users wanting a controlled PDF get only the OS print path, whose output varies by platform (ASSUMPTION, §3 Q3); R5 "full report package" may still demand a print-grade artifact later, reopening part of this decision.

### Option B — HTML canonical + webview print-to-PDF capture, PDF labeled derived

As Option A, plus the app drives the Tauri webview's print-to-PDF to produce a PDF file automatically; only the HTML is hash-bound, and the PDF carries an embedded notice that it is a derived view of HTML hash `X`.

- For: users get a one-click PDF without a new rendering dependency; honest two-tier evidence labeling.
- Against: ASSUMPTION: print-to-PDF output is not byte-stable across OS/webview versions, so the PDF can never enter the evidence chain; webview PDF APIs differ per platform (`TBD`: exact Tauri 2 API surface per OS unverified in-repo); risk that reviewers treat the non-canonical PDF as the record, inverting the evidence hierarchy.

### Option C — Rust-side deterministic PDF renderer as the canonical artifact (PDF-first)

A Rust PDF generator in `core/reporting` (pure-Rust PDF emission with fixed layout, embedded fonts, fixed metadata/timestamps) produces a byte-deterministic PDF directly from the envelopes; HTML optional or skipped. The PDF hash is the bound evidence artifact.

- For: single print-grade, signable, archival artifact — the strongest fit for the R5 package; engineering deliverables conventionally circulate as PDF (ASSUMPTION: stakeholder convention, not a repo requirement).
- Against: largest Phase A cost — typography, pagination, and table layout for ~30 required content classes (§2.3) implemented by hand; byte-determinism in PDF requires suppressing every nondeterministic field (creation date, IDs, font subsetting order) — achievable but exacting; the linter gates text, so lint must run on the pre-render text model, with the binary PDF re-extraction unverifiable in-repo (`TBD`); slows A7 and therefore the R2 exit.

### Option D — Dual deterministic pipelines (HTML and PDF both hash-bound) from one section model

Both Option A's HTML emitter and Option C's PDF emitter, sharing one intermediate rendered-section model; both hashes recorded.

- For: every consumer served; the shared intermediate keeps content single-sourced.
- Against: double the rendering surface to keep deterministic and visually consistent; nearly all of Option C's cost paid during Phase A for a "basic report generator" milestone (§2.3 R2 wording suggests basic, not full); two artifacts to lint-gate and golden-test.

---

## 5. Recommendation — `PROPOSAL`

Adopt **Option A now, with Option C named as the R5 follow-up decision point**:

1. **Format for A7/R2:** deterministic, self-contained, scriptless single-file **HTML** is the rendered calculation report. PDF at R2 only via the OS print path, labeled "derived view — not hash-bound evidence."
2. **Engine:** a new Rust crate (suggested `core/reporting/report_renderer`) emitting bytes from the validated `CalculationReport`/section envelopes (§2.1), so determinism is enforced where `cargo test` golden-file tests can prove it. The frontend only displays and saves the artifact; it does not compose report content (keeps the existing `ReportPanel.tsx` preview packet distinct from the FR-016 document).
3. **Hash binding:** the HTML bytes are hashed with the same SHA-256/canonical-evidence discipline as existing envelope hashes (§2.1 `audit_manifest`, §2.2 hash refs) and the hash is recorded in the report's audit manifest section, satisfying PRD §15.3 reproducibility (§2.3).
4. **Linter gating:** three enforcement points — (i) the bundled public template is a `PublicReportTemplate` surface linted in CI/tests; (ii) assembled section text is linted pre-render; (iii) final rendered document text is linted post-render, and a blocking finding (incl. `ProhibitedProfessionalClaim`) **blocks export** (§2.1, §2.5). Status vocabulary stays the six automatic statuses; the signoff block renders as an empty human field, never a software claim (§2.5).
5. **Local-only:** no network fetches, no remote fonts/CSS, no telemetry; output opens in any browser offline (§2.5).
6. **R5 checkpoint:** whether a hash-bound deterministic PDF (Option C emitter) is required for the "full report package" is **deferred to a named follow-up ruling** (suggested D-10b) at the R5 lead-up — not silently assumed either way.

Rationale: Option A is the only option that closes A7/FR-016/R2 ("basic report generator", §2.3) at Phase A cost while keeping the deterministic-evidence chain intact end-to-end; Options C/D front-load print-grade typography that R2 does not demand; Option B spends effort producing an artifact that can never be evidence. The long-standing OQ-08-01-001 TBD (§2.4) is resolved as: renderer = repo Rust crate; format = deterministic HTML; template language = `TBD` implementation detail within the crate (suggested: typed Rust builders, not a text-template engine, to keep the linter surface and determinism trivial — `ASSUMPTION` that this is acceptable as an implementation choice below ruling level).

This recommendation is a `PROPOSAL` only. It confers no authority and changes no state.

---

## 6. Downstream impact map

| Surface | Impact of this ruling |
|---|---|
| **Phase A7 tranche** | Defines A7's acceptance shape: render envelopes/sections → deterministic HTML + hash + lint gate (`plans/PLAN_2026-06-10_prd_completion.md` §3 A7). A7 feeds the A8 end-to-end exit test (create → solve → report). |
| **FR-016 acceptance** | "Deterministic envelopes; no rendered document" gap closes; acceptance becomes: rendered document contains the §15.1 content classes with recorded hash (`docs/PRD.md:349, 839–874`). |
| **R2 exit "report" leg** | The R2 exit criterion's "report" verb gets a concrete artifact (`docs/PRD.md` §22.3). |
| **R5 full report package** | Inherits the HTML artifact; the deferred D-10b PDF checkpoint (§5.6) must be scheduled before the R5 gate (`docs/PRD.md` §22.6; `_REGISTER.md` row D-10 "R5 'full report package'" blocks column). |
| **D-04 thresholds** | Tolerance/coverage evidence wording in rendered reports stays slot-data, not template prose, so D-04's later ruling changes data, not the renderer (§3 Q7; `_REGISTER.md` row D-04). |
| **Report linter enforcement** | Adds the post-render blocking gate and makes the bundled template a standing lintable public surface (§2.1 linter; §2.2 `ReportLintPanel.tsx` is the existing app-side seam). |
| **D-09 container** | A single-file self-contained HTML report is trivially storable in whatever container D-09 selects; a multi-file report directory would constrain D-09 (`_REGISTER.md` row D-09). |
| **DEL-08-01 review** | OQ-08-01-001 (renderer/format TBD, §2.4) is resolvable at that deliverable's formal review once ruled. No lifecycle state is changed by this packet. |

---

## 7. Authority and ruling record

Only the **human project authority** rules on D-10. Agents prepared this packet and may not certify, approve, or adopt it.

Per existing decision practice, the accepted ruling is recorded as a `DEC`/`SCA` entry in `execution/_Decomposition/SOFTWARE_DECOMP.md` §12 (or its successor register) — D-01 and D-08 were recorded this way as `DEC-018` and `DEC-019` on 2026-06-10 (`_REGISTER.md` rows D-01/D-08) — after which the dispatching persona updates `execution/_Coordination/_DECISIONS/_REGISTER.md` row D-10 from `AWAITING_RULING` to `RULED` with a pointer (`_REGISTER.md` header). This packet does not edit the register and does not resolve the decision.
