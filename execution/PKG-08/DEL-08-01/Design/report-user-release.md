# Per-report user release of the user's own project data (W-18b)

**Basis:** `origin/main` at `2e4748eada` (2026-10-26). Revision 4. Every
file:line citation is at that tree and relative to `projects/chirality-piping/`
unless it names a root path. Revisions 1-3 were at `c7f52fd12a` and
`3027cf2f35` (2026-10-25). Revision 2 reduced the design to the owner's scope R
(§0), after Codex round 1 (three blocking, four should-fix) and round 2 (one
blocking, three should-fix). Revision 3 answered round 3 (one blocking, three
should-fix, one non-blocking, and A7's wording). §7 says where each finding is
resolved. Design before implementation; nothing here is built. This design
records the mechanism by which a user can state, for one report, that they
release their own project data for sharing. The design itself records no user
release and no software release, and it records, implies or derives no
acceptance, issuance, approval, compliance or lifecycle standing (F-PIP-3,
`loop/LOOP_INIT.md:90-94`).
Standard claim fence applies (claims taxonomy per DEC-081).

**Changed in revision 4.** The design was re-derived against main
`2e4748eada`. Its decisions are unchanged except that §0 records one new
decision, on exact-route results.
- **Exact-route results (W-11 opening, #1648).** The owner opened the rendered
  report and the report package to `pressure-2` and `pressure-source-1`
  results, local private only. Route 010 therefore no longer inherits the
  exact-route refusal from the local-private package. Under R it refuses
  exact-route results with its own code,
  `REPORT-USER-RELEASE-EXACT-ROUTE-NOT-ADMITTED`. The refusal applies in the
  webview, at native admission and in the shared producer, and the reader
  refuses such a package. This was a manager decision, confirmed by HELP_HUMAN
  (§0). Changed: §0, §2.1, §2.2, §2.5, §2.8, §2.9, §3.4, §6, §8, §9, §10 and
  Appendix A.
- **The report display rule (#1655).** The derivation names the shared
  functions it derives rows and the unit display summary with (§2.6).
- **The Diagnostics binding (#1659).** The derivation's diagnostics use the
  producer's `expected_report_diagnostics` (§2.6). Route 010 needs nothing
  more, since R refuses exact routes.
- **Object form (#1650, #1651).** Route 010's keyed checks follow the existing
  object-form check (§2.5). Library rule 1 also counts a non-object
  `library_ref` (§2.7).
- **FR-REP-003 provenance (W-11 (c), #1639).** Since #1639 the renderer
  refuses private record content outside `local_private`, and the owner ruled
  "C1 b, C2 a". So the renderer must admit the user-released class under
  `user_released_report` (§2.6, §4). The minimal input also leaves out the new
  material, section and support provenance entries and the library
  references (§2.6).
- **W-11 custody digests (#1637).** §1 and §2.4 now state the native custody
  record the solve-job registry keeps, and how route 010's attestation differs
  from it.
- **Citations.** Every file:line citation was re-checked against the new tree.
  Where a cited fact changed, the text now says so.
- **Appendix A.** Each Old was re-checked against main. A1, A10, A11 and A12
  are re-derived, A14 is new and A13 has moved line (Appendix A).
- **Codex round 1 at revision 4 (`3dfc794601`: one blocking, four
  should-fix).** The headless runner and the producer's general entries now
  refuse every user-release marker, and one derivation-bound producer entry
  is the only way to a user-released package (B1, §2.8). Route 010 refuses
  exact texts in the outer command, before they are priced (S1, §2.5). Library
  rule 1 counts a non-object `library_ref` at both layers by its own walk
  (S2, §2.7). A14's New gives one code per surface (S3). The naming rule is
  applied throughout, including `user_released_at` and the statement (S4).
  §7 says where each finding is resolved.
- **Codex round 2 at revision 4 (`1dce3ee2b2`: two should-fix, one
  non-blocking; B1 and S1-S4 closed).** The reach scan now pins every call of
  the renderer's and `pdf_emitter`'s document functions in the four crates
  that can make one, rejects aliases, re-exports, function values and
  closures, and has negative fixtures. The §2.8 table claims no more than the
  scan establishes (S5, §2.8, §6). The admitting entry runs item 0 and the
  user-released exact check before the derivation, so an exact document gets
  `REPORT-USER-RELEASE-EXACT-ROUTE-NOT-ADMITTED` whatever its report. The
  general entries' precedence is stated (S6, §2.8, §6). The runner's
  refusal message names the triggering member from a closed set of three
  paths (N1, §2.8). A1 is unchanged (§7).
- **Codex round 3 at revision 4 (`80b8244391`: no blocking, one should-fix,
  one non-blocking; S6 and N1 closed, A1-A14 pass).** The reach scan no longer
  borrows the line scanner's indentation and test-module rules. It parses the
  four crates' source with `syn`, which is already in `src-tauri`'s lock at
  the version it needs. It resolves their modules itself, reads every file
  that a production declaration loads, and attributes each name to its real
  lexical scope. It fails closed on `#[path]`, `include!`, macros, closures,
  function values, trait objects and re-exports, and gains the round's
  negative fixtures (S5a, S5b, §2.8, §6). The general entries' precedence now
  keeps `wire::assemble_wire_request`'s own earlier checks (N2, §2.8).
  Appendix A is unchanged, and all fourteen Olds still count 1 (§7).
- **Codex round 4 at revision 4 (`3d2f35911b`: no blocking, one should-fix,
  three non-blocking; A1-A14 pass).** A macro could still hide a production
  module or build a call path that the token screen cannot see (S5c, S5d), and
  a declarative macro from outside the four crates could emit a call without
  naming it (N4). The reach scan now fails closed on macros. A production
  macro definition is a finding unless it is on an allow-list pinned by the
  SHA-256 of its source text; only `app_invoke_handler` is listed. A macro
  invocation, attribute or derive is a finding unless it is on a list derived
  from the basis source, and imports cannot rebind a listed name. The scan's
  module traversal therefore sees every production `mod` (S5c, S5d, N4, §2.8,
  §6). The general entries' precedence now names `check_request_in`'s own
  cardinality checks (N2a, §2.8). §2.4's reason for digesting in the worker
  names the parsed result copy (N3). Appendix A is unchanged, and all
  fourteen Olds still count 1 (§7).

## 0. Rulings

- **Owner, 2026-10-24:** "I accept your recommendations for each of these
  three: G10, W-12, W-18." For W-18 this is option B (W-18a design §3, §7
  O-1): the user explicitly releases their own project data, in v0.1. As
  recorded, the shape is:
  - an explicit opt-in recorded in the document, on the basis of
    `docs/IP_AND_DATA_BOUNDARY.md` §6 (:99);
  - private rule-pack and library values stay at safe metadata;
  - the protected-content linter and the disposition blocks still apply;
  - the notice states the user release;
  - the private default is unchanged;
  - a DEL-12-02 runtime vocabulary addition;
  - commitment text goes through the wording grant with Codex;
  - targeted regression checks: the flag cannot leak private values; redaction
    applies to every other context; the user release is recorded and visible.
- **HELP_HUMAN, 2026-10-24, option (1), per report:**
  - recorded in that report's `report_context` plus the package manifest record;
  - bound to the model hash or result-set identity the report covers;
  - an unmistakable notice;
  - without the selection, the report stays local-private exactly as today;
  - tests prove no replay onto a different model or result set;
  - the DEL-12-02 classification and any commitment text go through the wording
    grant.
- **The selection does not persist.** It is not stored in the session, the
  project or the model document, and the model document does not change
  (W-18a design §7 O-1).
- **HELP_HUMAN, 2026-10-25:** "Release itself stays the owner's act, so W-18b
  designs only the mechanism for recording a per-report release (as decided
  earlier) and records no release." Software release and issuance are owner
  acts. This mechanism records a *user's* statement of their own
  redistribution right for one report.
- **Ruling 10A (owner, 2026-10-21):** exact-route results were to stay
  refused for reports and the report package until W-11 opened them. At
  revision 3 this design relied on that ruling: "The user release admits only
  what the local-private package admits (§3.4)."
  - **Changed since revision 3.** The owner has since opened the rendered
    report and the report package to `pressure-2` and `pressure-source-1`
    results, local private only. The opening ruling (2026-10-26) reads "My two
    decisions are to approve your recommendations in both cases. Proceed
    accordingly." The evidence ruling reads "#1648 I approve option A. For
    FR-REP-003 I approve option D." Both are recorded in DEL-08-01 Design
    `exact-route-report-sections.md` §0, and the opening merged in #1648
    (`289bd8d546`).
  - So the local-private package now admits exact-route results, and the
    sentence quoted above no longer keeps them off route 010. Revision 4
    withdraws it. The next item governs instead.
- **Exact-route results under R.** This was a manager decision (2026-10-26),
  made within scope R as the owner ruled it, and confirmed by HELP_HUMAN on
  2026-10-26: "Your decision stands, no veto. Under R, route 010 refuses
  exact-route results with its own named code at all three layers: webview,
  native admission and producer. That keeps the outcome the owner approved for
  R."
  - The owner ruled R while exact routes were refused, so under R route 010
    admits only a Current pressure-free (`preview-physics-2`) result.
  - It refuses an exact-route result with
    `REPORT-USER-RELEASE-EXACT-ROUTE-NOT-ADMITTED`. The refusal applies in the
    webview (§2.2, §3.4), at native admission (§2.5 step 2a) and in the shared
    producer (§2.8). The reader refuses such a package too (§2.9).
  - Admitting exact-route results under user release returns to the owner
    (§10).
- **DEC-028 (owner, 2026-10-10):** "Close: one version". The package has one
  version, 2.0.0 today (`report_package/src/lib.rs:89`); the reader refuses
  every other version by name (`reader.rs:281-305`).
- **Owner, 2026-10-25 (W-18b decisions):** "I approve your recommended options
  for the three W-18b decisions." As relayed by HELP_HUMAN, the approved
  recommendations are:
  - **Scope R first.** One result member. No rule outcomes, comparisons or
    handoff records. Library-referenced models are refused. A minimal native
    renderer input. Solve attestation, and no rule-check attestation. The rest
    (rule outcomes at safe metadata, library-referenced models, comparisons)
    stays in v0.1 as a later slice on the same mechanism; it is not dropped.
  - **O-A = V1.** The package stays at 2.0.0 with the optional record.
    DEC-028's "one version" is read as admitting an optional writer-defined
    record that the reader at the same head accepts.
  - **O-G: equality-only guarantee,** stated plainly in the reader text and in
    DEL-02-05 (A8).

  O-E is moot under R. O-B, O-C, O-D and O-F stay as recommended (HELP_HUMAN,
  2026-10-25).
- **HELP_HUMAN, 2026-10-25: V1 covers R's package form.** The ruling adopted the
  recommended V1 in full, including its second clause: "the reader at each
  head accepts what the writer at that head emits". A user-released package
  with no state/comparison/handoff member is emitted by this head's writer and
  accepted by its reader, so no handoff set is kept (§2.1, §4).
- **HELP_HUMAN, 2026-10-25: rule-pack-referenced models are refused under R**
  (`REPORT-USER-RELEASE-RULE-PACK-REFERENCED`, §2.7). This is a stated
  consequence of the owner's "no rule outcomes" together with W-18a's rule
  that a package names every active rule pack of the solve. It is sequencing
  within the ruled scope, not a narrowing of v0.1: the later slice, kept in
  v0.1 (§10), lifts it together with the library refusal. The dialog says
  plainly which models cannot be shared yet, and why (§2.2).

§2.1 states scope R as an exact member contract. §10 lists what the later
slice adds on the same mechanism.

**Naming rule.** The word "release" never appears alone in a product text,
identifier or record. It is always "user release", "user-released" or "release
your project data". No text uses "issued", "approved", "accepted",
"certified", "released version" or "release of SWBPIPE". The record's
`standing` member is the constant `user_statement_of_data_rights_only` (§2.3).

**What "the user's own project data" means here.** It is the content of the
model this application solved in the current session, the pressure-free result
its own solver produced for that model, and the run record of that solve, as native
admission binds and derives them (§2.5, §2.6). Under R the model carries no
detectable library or rule-pack reference (§2.7).

**The reliance limit.** The detector finds the marks that library applies
have written into a model (§2.7). It cannot find a library origin that a user
has erased, and it does not show that any value is the user's own. For every
value without a detectable library origin, the user release relies on the user's
own statement of their rights: the checkbox statement the record carries
(§2.2, §2.3). The native host does not verify that statement, and the record
does not prove it (§2.9).

## 1. Where things stand

- **Every session value is private.** The report builder classifies the
  session, every component and every spring hanger as `private_project_data`
  with `redistribution_status` `private_only` (`renderableReportInput.ts:142-153,204-220,539-554`).
  Since W-11 (c) (#1639), a material, section, component or support that
  carries a `provenance_record` gets that record mapped in its place: privacy
  is still `private_project_data`, but the record's own redistribution and
  review statuses are copied (`mappedProvenanceRecord`, `:250-263`; W-11 design
  §4.1).
- **Two report surfaces, two contexts.**
  - The rendered-report panel renders under `public_report` (blocked for every
    session) or, with its own checkbox, `local_private`
    (`RenderedReportPanel.tsx:84-85,253-261`). Its HTML save is a webview
    `data:` download (`:377-393`) and its PDF is the browser print of the
    derived view (`:394-401,417-426`), which the document itself says is not
    hash-bound (`report_renderer/src/lib.rs:2131-2134`).
  - For a pressure-free input, the native render command is a pure renderer.
    It receives no model, no route evidence and no screening
    (`render_calculation_report`, `src-tauri/src/lib.rs:1569-1584`, through
    `report_render_exact::render_calculation_report_args_in`,
    `report_render_exact.rs:94-117`). Since W-11 it first refuses a struct in
    positional form (`check_object_form`, `:127-132`) and applies the
    exact-identity step 0. An exact-route input must then be `local_private`
    and is bound to the bytes its solve job delivered (W-11 design §3.6).
  - Save Report Package… renders under `local_private` only, with
    `generatedAt: null` (`reportPackageRequest.ts:246-256`), through route
    `DREP-PACKAGE-SAVE-009` (`reportRedactionProjector.ts:58-68`). Native
    admission (`report_package_save.rs:764-860`) does the following:
    - it re-checks the evidence;
    - it refuses a positional struct (`check_request_object_form`, `:715-722`)
      before any check that reads members by key;
    - for an exact-route request, it binds the source and model to the job's
      custody (`bind_exact_save_source`, `:733-761`);
    - it binds the request, model and input-manifest texts by SHA-256;
    - it screens dispositions in the request and the model, and binds the
      context.

    "caller evidence alone admits nothing" (DEL-12-02 `ScopeOfWork.md:486`,
    REXC-OI-007).
  - **Exact-route results are now admitted on both surfaces, local private
    only** (W-11 opening, §0). The output policy admits them on
    `rendered-report` and `report-package` (`EXACT_ADMITTED_SURFACES`,
    `outputPolicy.ts:91-94`). The
    package's Rust boundary admits `preview-physics-2`, `pressure-2` and
    `pressure-source-1` (`report_package/src/document_form.rs:19`).
    `reportPackageUnavailableReason` (`reportPackageRequest.ts:83-85`) refuses
    only the retained routes.
- **Context vocabulary.** Five contexts: `public_report`, `public_example`,
  `shared_model`, `downstream_tool`, `local_private`
  (`redactionExportControls.ts:17-22,127-133`; `controls.py:18-19`; schema
  `redaction_export_controls.schema.yaml:402-411`; DEL-12-02
  `ScopeOfWork.md:58`). The public and shared contexts redact private values
  (`redactionExportControls.ts:465-477`; `controls.py:419-426`), so none of them
  carries a project value unredacted.
- **The decision order today.** `actionFor` (`redactionExportControls.ts:417-479`)
  and `_action_for` (`controls.py:372-426`) decide, in order: the claim,
  network, SQL, storage-bypass, secret, payload and path checks; the
  `protected_suspected` block (`:439-441`; `controls.py:402-403`); the
  quarantined and rejected block (`:442-444`; `controls.py:404-405`); then
  **unknown metadata** (missing metadata, unknown redistribution, unknown
  privacy, `:445-453`; `controls.py:406-411`); and only then the safe and private
  branches. An object whose privacy is `private_rule_pack_data` and whose
  redistribution is `unknown` is therefore decided by the unknown branch, never
  by the private one (Codex round 1, reproduced in memory).
- **The route projector labels by path, not by source.** `projectRouteValue`
  (`redactionExportControls.ts:844-925`) labels a leaf private when its path
  contains a private token and `unknown` otherwise. It strips the payload's
  intent keys globally first (`stripRouteIntent`, `:708-712,825-842`).
- **Library values reach result rows through genuine solver output** (Codex
  round 2, blocking). A library-applied component's flexibility factor and SIF
  are model values that the solver consumes:
  - the curved-bend flexibility row carries the factor as its value
    (`core/product_physics/src/lib.rs:13486-13488`);
  - the user-multiplier row's basis carries the SIF, the flexibility and the
    source reference (`:13390`);
  - the intensified-bending rows carry the SIF and its source in their sign
    convention, contract evidence and diagnostics
    (`preview_physics.rs:687,696-698,700-705`).

  Codex cited these at `05ec850eca` as `lib.rs:13230` and
  `preview_physics.rs:694` (at revision 3, `lib.rs:13341-13343,13245`). The solve's own attestation reproduces them, so no
  derivation check can tell a library value from a user-entered one once it is
  in the result.
- **How a library apply marks the model.** Every apply writes a
  `provenance_record.library_ref` (`libraryApply.ts:275`) and a library
  provenance string `library:<kind>:<library id>#<record id>`
  (`libraryProvenanceString`, `libraryApply.ts:81-83`). That string goes into
  component source-reference strings (`:752-759`) and a hanger's `provenance`
  text (`hangerSelection.ts:41-42,54-55`). The native save already walks every
  `library_ref` at any depth (`export_disposition_screen::library_ref_sites`
  over the one walk `walk_document`, `export_disposition_screen.rs:454-516`),
  pinned to `fixtures/model_document/library_ref_sites.json` with the webview's
  `libraryRefSites` (`libraryImportService.ts:440`). The same walk visits every
  `rule_pack_refs[]` entry. Since W-17 object form (#1651) it also visits a
  `provenance_record`, `rule_pack_refs` entry or `library_ref` that is not an
  object, and the disposition screen refuses it on every route
  (`MODEL-DISPOSITION-RECORD-MALFORMED`, `:398-404,485`). W-17's library
  disposition change (#1653) refuses an unreadable disposition at apply
  (`libraryApply.ts:99-146`). It writes no new library mark.
- **How the hanger selection marked the model before W-17** (Codex round 3,
  blocking). From `46f3aed341` (2026-09-05) until `00af9112c0` (2026-10-23),
  the hanger selection wrote neither mark. It stored a support's `provenance`
  as the RFC 8785 JSON text of `{hanger_id, kind:
  "user_imported_hanger_selection", library_id, library_kind: "hanger",
  library_metadata, record_snapshot}` (`hangerSelection.ts:35-39,52` at
  `00af9112c0^`). A model saved in that period still carries it. Its hanger
  values reach genuine `spring_hanger_user_input_review` rows
  (`core/product_physics/src/lib.rs:13574,13675`), and nothing current
  detects their library origin. `core/library_import/README.md:49-55` still
  describes that form as current.
- **Rule packs in the package.** A package names the rule packs of its
  rule-check run, and admission requires them to equal the solve's
  `active_rule_packs` (`report_package_save.rs:684-700`;
  `reportPackageRequest.ts:261`). The session's analysis record composes the
  rule-check aggregate into its status (`previewService.ts:584-600`). Since W-17
  (#1629), outcomes are per check and load case or combination, with an
  optional `basis_ref` on `UserRuleCheckOutcome`
  (`report_renderer/src/lib.rs:132-165`; `renderableReportInput.ts:732-755`).
- **Solve custody.** The host never materializes the solved model: the worker
  process does (`src-tauri/src/solve_jobs.rs:1-55`; `ordinary_solve`,
  `solve_worker.rs:465-472`). The worker returns the producer's exact bytes,
  which are serde's serialization, not RFC 8785 (`lib.rs:1894-1906`). The host
  retains them until the webview takes them, and frees them on disposal
  (`solve_jobs.rs:722-914`).
  - **Changed since revision 3 (W-11 (b), #1637).** Each job now records
    custody digests: the SHA-256 of the raw model body it admitted and of its
    result bytes, with the admitted solver mode. That is 65 bytes per job,
    hashed in place (`solve_jobs.rs:37-50,227-248`). `custody_binding` returns
    them only while the delivered result's custody lease is held
    (`:759-770`), and they are dropped when custody ends. The exact-route render
    and save bind resubmitted texts to them (W-11 design §3.6).
  - They are digests of raw bytes, not of canonical values. Once custody
    ends, nothing native records which result was produced for which model
    (§2.4 says how route 010's attestation differs).
  - The webview's Current export requires a captured native invocation for
    the result (`hasNativeMechanicsInvocation`, `resultExportAdapter.ts:294`),
    which is a webview check.
- **Native pieces R can reuse unchanged.**
  - `canonical_json::checked_v1_domain_canonical` with `ByteSink` streams a
    domain-separated canonical form into a hasher without holding the text
    (`core/serialization/canonical_json/src/lib.rs:124-187`).
  - `result_export::derivative::derive_document` (`derivative.rs:120`) derives
    a 0.3.0 document from a base, the model, the source and an origin.
  - Record Run's binding checks of a transient analysis record against its
    result and input manifest (`analysis_runs::checked_submission`,
    `analysis_runs.rs:634-782`).
  - The producer's report projections of a source, already derived natively
    (`report_package/src/bindings.rs`):
    - the report rows, `expected_report_result_rows` (item 6, `:129-167`;
      vectors `fixtures/reports/invented/report_result_rows_parity.json`).
      Since #1655 its value text is the report's display rule,
      `quantity_display::display_quantity` (`quantity_display.rs:41-44`): six
      significant digits, and a value stored in `Pa` shown in `MPa`
      (vectors `report_quantity_display_parity.json`, mirrored by
      `reportQuantityDisplay.ts`);
    - the full-precision row checksums, `check_report_result_refs`
      (`:177-191`);
    - the unit display summary, `check_unit_display_summary` (`:199-207`),
      with `QUANTITY_DISPLAY_POLICY` and
      `quantity_display::source_displays_scaled_value`
      (`quantity_display.rs:28,54-67`);
    - since #1659, the report's projection of the source's diagnostics,
      `expected_report_diagnostics` (`:234-274`; vectors
      `report_diagnostics_parity.json`). `check_report_diagnostics`
      (`:289-305`) applies it on the exact route only.
- **The package and its reader.**
  - The manifest's fields are fixed by the version except `package_id`,
    `export_profile_id`, the source references and the diagnostics
    (`lib.rs:667-758`).
  - Its third boundary note reads "No timestamps or generated identifiers are
    recorded anywhere in this package; identical inputs produce identical
    bytes." (`lib.rs:746-747`).
  - The reader rebuilds the manifest from those fields and refuses any
    difference (`reader.rs:233-252`).
  - The writer blocks a package without a state/comparison/handoff member
    (`lib.rs:279-283`), the producer requires exactly one handoff record set
    (`bindings.rs:488`), and the reader refuses an unblocked package without
    one (`reader.rs:976-983`; since W-11 it also requires a result member
    there).
  - The reader's role forms fix member order, not member count
    (`reader.rs:778-792`), so it admits more than one result member.
- **What is missing.** A runtime way for the user to record, for one report, the
  basis `docs/IP_AND_DATA_BOUNDARY.md:99` names: "unless a user intentionally
  exports or contributes it with documented redistribution rights".

## 2. Design

### 2.1 Scope R: the reduced member contract

A user-released package is the 2.0.0 container with the optional record (§4).
Its members, in archive order after the manifest, are exactly these:

| Member | Count | What it may contain | What is absent |
|---|---|---|---|
| `calculation_report.html`, `calculation_report.pdf` | one each | The rendering of the minimal native renderer input (§2.6): the title; the model summary (project name and id, units, model hash, input-manifest reference, display policy); the load cases (id, label, kind); the result rows (the five-field projection of every `results` row of the solve, its review rows included, such as a spring hanger's `spring_hanger_user_input_review` rows); the solve's diagnostics; the analysis-status disclosures; the fixed technical-preview limitation; the run context (software, solver identity, mode and settings, run name); the record and the notice | Rule-pack references; user-rule outcomes; component and spring-hanger provenance entries; dedicated per-object hanger quantity entries; numerical-health rows; the local persistence reference (always `TBD`); a generation time and a run recording time (not recorded) |
| `audit_manifest.json` | one | Its id (the input-manifest reference); the model and input-manifest hash records; the solver identity; the unit-system reference; `rule_pack_refs: []`; `assets: []`; the boundary constants | Any rule-pack reference; any asset |
| `result_export_<envelope>.json` | exactly one | The DEL-08-04 0.3.0 document of a pressure-free (`preview-physics-2`) result, which native admission derives from the bound model, the attested solve, the run record and the input manifest (§2.6) | A second result member; an exact-route (`pressure-2`, `pressure-source-1`) document (§0) |
| State/comparison/handoff record set | none | — | The member itself: no state/run, comparison or handoff section |
| `package_manifest.json` | one | The fixed fields; `package_id` `desktop-user-released-report-<run>`; the profile `desktop_user_released_report_package_1`; the source references; the producer's diagnostics; the record with its `content_binding`; the user-released boundary notes (§5.3) | — |

The request on route 010 also carries `rule_check_aggregate: null`, no
`user_rule_checks`, `solve_rule_check_status` equal to the solve's own
`status.rule_check`, and `state_comparison_handoff_records: []`.

The bound model, the solve's result, the input manifest and the run record are
validation inputs only. None is written to the package, and neither is any
rule-pack payload or library record (W-18a design §2.2, "Not a member").

**What R refuses.** The save is refused when:
- the model carries a detectable library reference, in any form a library
  apply has written, or a rule-pack reference (§2.7);
- any object in the model or the request carries a non-project private class
  (§2.7);
- the result was not produced by this session's solver for that model, or its
  attestation has been evicted (§2.4);
- any request member differs from its native derivation (§2.6);
- the result is an exact-route result (`pressure-2` or `pressure-source-1`):
  `REPORT-USER-RELEASE-EXACT-ROUTE-NOT-ADMITTED` (§0, §3.4);
- or any existing refusal applies: a retained route, a positional struct, a
  blocked disposition, the protected-content linter (§3.4).

**The W-17 constraint.** R carries no rule-check outcome, in any shape. The run
record it admits carries the solve's own rule status, not a rule-check
aggregate (§2.5 step 8). The per-(check, basis) outcome shape of #1629 is the
shape the later slice must carry (§10).

### 2.2 Where the selection is made, and what the user does

**Recommendation: the report package only.** The user release is offered on
Save Report Package…'s route, and the package's HTML and PDF members carry it.
The standalone rendered report (panel HTML save and Print / PDF) is not offered
a user release in W-18b (O-C, as recommended). Reasons, from current code:
- the standalone HTML save and the print view are webview sinks with no native
  admission (`RenderedReportPanel.tsx:377-401`), and for a pressure-free input the render
  command receives no model or evidence (`src-tauri/src/lib.rs:1569-1613`), so a
  standalone user release would rest on caller evidence alone, which REXC-OI-007
  forbids for this class of route;
- the PDF is a derived, non-hash-bound view (`report_renderer/src/lib.rs:2131-2134`),
  so a record in it could not be bound to the bytes the user shares;
- HELP_HUMAN's ruling names the package manifest record, which only the
  package has.

A user who wants to share the document shares the `.opsproj` or its HTML or PDF
member; each member carries the notice and the bound identities (§5.1).

**The controls.**
- In the rendered-report panel's package controls
  (`RenderedReportPanel.tsx:268-290`), a second button after
  "Save Report Package…": **"Save Shareable Report Package…"**
  (`data-testid="report-package-user-release"`), enabled under the same
  conditions as the local-private save (`:283`).
- In the File menu, after "Save Report Package…" (`App.tsx:1253`; native menu
  `src-tauri/src/lib.rs:4467`): **"Save Shareable Report Package…"**, command
  id `file.save-shareable-report-package`, disabled exactly when
  `file.save-report-package` is.
- Both open one dialog, `apps/desktop/src/features/report/UserReleaseDialog.tsx`.
  The local-private checkbox "Include known private values in this local
  package" (`RenderedReportPanel.tsx:270-279`) is not read by this route and
  keeps its meaning.

**The dialog** (pattern: `HandoffSaveDialog.tsx`, `role="dialog"`,
`aria-modal="true"` at :81-82). It takes focus in `useLayoutEffect`
(`HandoffSaveDialog.tsx:44`), on the statement checkbox. Escape and Cancel
close it and discard the selection. Exact proposed wording:

> **Release your project data in one shareable report**
>
> This saves a report package you may share. It includes this project's
> values and results, unredacted.
>
> - It covers only this report: model "{project name}" (SHA-256
>   {first 12 hex}…), run {run id}.
> - It carries the model summary, the load cases and this solve's results. It
>   carries no rule-pack reference, user-rule outcome or run comparison.
> - It needs a pressure-free result solved in this session. An exact-route
>   result, and a model with a library or rule-pack reference, cannot be
>   shared this way.
> - The application checks this model only for library and rule-pack marks.
>   It cannot tell where any other value came from, so your statement below
>   covers every value in this report.
> - Content marked protected, quarantined or rejected still stops the save.
> - Nothing is remembered: your next report is local private unless you
>   choose this again.
>
> ☐ I hold the right to share this project's data, and I release my project
> data in this report for sharing.
>
> [Save Shareable Report Package…] [Cancel]

When the webview's early check finds a library or rule-pack reference, in any
form §2.7 names,
the dialog shows, in place of the checkbox: "This model uses a library record
or a rule pack. Shareable packages do not yet carry library record values or
rule-pack checks, so a shareable package cannot be saved for this model. Its
local private package is unaffected." The save button stays disabled.

When the user-release cell refuses an exact-route result (below), the dialog
shows, in place of the checkbox: "This result is from an exact pressure route
(pressure-2 or pressure-source-1). Shareable packages do not yet carry
exact-route results, so a shareable package cannot be saved for this result.
Its local private package is unaffected." The save button stays disabled.

The save button is disabled until the checkbox is ticked. The checkbox starts
unticked every time the dialog opens.

**What the user does:** an explicit selection (opening the dialog from its own
control) plus a statement of the redistribution right (ticking the statement),
then the save. Confirming starts the save at once; the confirmation is one
argument of that one save (§2.3, "Session").

**Exact routes are refused on this route (§0).** The package's own cell,
`reportPackageUnavailableReason` (`reportPackageRequest.ts:83-85`), admits
exact-route results since the W-11 opening, so the dialog reads a cell of its
own, `userReleasedReportPackageUnavailableReason(result)` in
`reportPackageRequest.ts`:
- the package cell's refusal, when it refuses (the retained routes);
- else, for a result that `isExactReportSource` recognizes
  (`exactReportSections.ts:68`),
  `REPORT-USER-RELEASE-EXACT-ROUTE-NOT-ADMITTED: a shareable report package
  carries pressure-free (preview-physics-2) results only; this exact-route
  result can be saved in a local private package.`;
- else null.

`OUTPUT_POLICY` gains no surface and no cell changes, so every other surface's
decision is unchanged. `buildUserReleasedReportPackageRequest` applies the same
cell first. Its document identity check, the form of
`assembleReportPackageRequest`'s (`reportPackageRequest.ts:230-234`), admits
`preview-physics-2` only, not `REPORT_PACKAGE_ADMITTED_CONTRACT_IDS` (`:57`).
Native admission
(§2.5 step 2a) and the producer (§2.8) refuse the same results, and the reader
refuses such a package (§2.9). The webview check is early; native admission
decides.

### 2.3 The record

**Form.** One JSON object, key `user_release`. Every member is required; no
other member is admitted (`deny_unknown_fields` in Rust, an exact key check in
TypeScript and at the reader). Hex values are 64 lowercase hexadecimal
characters. The record is a typed member of the renderer input (below), so the
shared object-form check covers it. That check is
`report_renderer::object_form::report_positional_struct_path`
(`object_form.rs:485`, #1650). It refuses a record written in positional
(array) form at the native render, the native save and the producer, before
any check that reads its members by key.

| Member | Type | Value |
|---|---|---|
| `record_kind` | string, constant | `user_release_of_own_project_data` |
| `record_version` | string, constant | `1.0.0` |
| `statement` | string, constant | `I hold the right to share this project's data, and I release my project data in this report for sharing.` (the dialog's checkbox text, verbatim) |
| `statement_basis` | string, constant | `docs/IP_AND_DATA_BOUNDARY.md §6: a user intentionally exports their own data with documented redistribution rights` |
| `standing` | string, constant | `user_statement_of_data_rights_only` |
| `scope.scope_kind` | string, constant | `this_report_only` |
| `scope.report_id` | string | `report:<analysis_run_ref>`, which is the report's `calculation_report.report_id` (`renderableReportInput.ts:913`) |
| `scope.package_id` | string | the request's `package_id`, `desktop-user-released-report-<run>` |
| `scope.user_released_data_class` | string, constant | `private_project_data` |
| `scope.not_carried` | array of strings, constant | `["rule_pack_references", "user_rule_outcomes", "run_comparisons", "handoff_records", "library_records", "audit_assets"]` |
| `bound_to.model_ref` | string | the model's `project.id` |
| `bound_to.model_document_sha256` | hex | SHA-256 of the RFC 8785 bytes of the session model document |
| `bound_to.analysis_run_ref` | string | the run id the report covers (`result.run_id`) |
| `bound_to.input_manifest_ref` | string | the run's input-manifest reference, `input-manifest:<model id>:<sha256>` |
| `bound_to.input_manifest_sha256` | hex | SHA-256 of the RFC 8785 bytes of the run's input manifest |
| `bound_to.result_envelope_id` | string | the result document's `result_envelope.envelope_id` |
| `bound_to.result_document_derivative_sha256` | hex | the result document's `result_envelope.reproducibility.derivative_hash.value` |
| `user_released_at` | string | the application clock when the user confirmed, UTC, `YYYY-MM-DDTHH:MM:SSZ` (O-B, §7) |
| `content_binding` | object, **manifest copy only** | written by the package writer, never by the caller (below) |

The free-text members and `scope.not_carried` are constants, so a record cannot
carry any other wording (for example a claim) in their place. The later slice
that carries rule outcomes changes `scope.not_carried`, and with it
`record_version` (§10). `user_released_at` is the one caller-supplied value that
native admission cannot derive: it checks its form only, and the record states
it as the application clock's reading.

**Content binding (manifest copy only).** The writer adds `content_binding` to
the manifest's copy of the record after the members are materialized:
`{ "algorithm": "sha256", "members": [{ "file_name", "role", "sha256" }, …] }`,
one entry per non-manifest member (HTML, PDF, audit manifest, result member) in
archive order, each the SHA-256 of that member's exact bytes. The renderer never
sees `content_binding`: the request's `report_context.user_release` must not
carry one (`deny_unknown_fields`), and the notice and rows (§5.1) render the
other members only. So the rendered bytes do not depend on it, and binding their
hashes is not circular. The record then covers the report content, excluding
only the binding itself: the caller's record (rendered and in the manifest)
plus the hash of every member it covers.

**The three hashes, precisely.**
- **Model document SHA-256** (`model_document_sha256`): SHA-256 over the RFC
  8785 (JCS) bytes of the session model, as `canonicalSha256Hex(model)`
  computes it (`reportPackageRequest.ts:264`). It is the same value as:
  - the request's `audit_manifest.model_hash.value` (`:282-288`);
  - the result document's `reproducibility.model_hash.value` (bound equal by
    `bindings.rs:408-420`);
  - the native SHA-256 of `modelCanonicalText` (`report_package_save.rs:811`).
- **Result-set identity** (`result_envelope_id` and
  `result_document_derivative_sha256`): the DEL-08-04 0.3.0 document's
  envelope id and its derivative hash, SHA-256 over the RFC 8785 bytes of the
  document without its own hash (`resultExportAdapter.ts:153`).
  - The producer verifies that hash (`document_form::check_self_hash`), and
    the reader re-verifies it for every result member (`reader.rs:941-942`;
    `document_form::check_result_member_form`, `document_form.rs:282-286`).
  - The derivative hash, not the member's byte hash, is chosen because it is
    the document's own recorded identity and is known before the report is
    built. `content_binding` records the member's byte hash.
- **Audit-manifest input-manifest hash** (`input_manifest_sha256`): the
  request's `audit_manifest.input_manifest_hash.value`
  (`reportPackageRequest.ts:289-295`), which native admission binds to the
  SHA-256 of `inputManifestCanonicalText` (`report_package_save.rs:822-829`).
  `input_manifest_ref` is that hash record's `payload_ref`, which admission
  binds to the session's reference (`:843-850`).

**Where it is written.**
1. `report.report_context.user_release` in the request, from which the
   renderer emits it into the HTML and PDF members (§5.1). The Rust
   `ReportContext` (`report_renderer/src/lib.rs:103-118`) gains
   `#[serde(default)] pub user_release: Option<UserReleaseRecord>`.
2. The package manifest's top-level `user_release`, written by the writer as
   the typed `report_context.user_release` plus `content_binding` (one source
   for every caller member, so the two copies cannot differ in them). Absent
   from every package without a user release.
3. The save receipt gains `user_release_recorded: bool` and
   `user_release_model_document_sha256: String` ("" when not recorded), shown
   in the panel's status line (`RenderedReportPanel.tsx:304-322`).

**Session.** The selection lives only in the dialog's React state and in the
argument of the one save it starts:
`handleSaveShareableReportPackage({ userReleasedAt })`, beside
`handleSaveReportPackage` (`workspaceSession.ts:2976-3062`).
- Its basis is the same basis without `comparison`, `comparisonUnavailable`,
  `projectSummary`, `ruleCheckAggregate`, `ruleCheckRun` and `privateIntent`,
  and the builder computes every bound value from that basis (§2.6).
- The run record it sends is the solve's own transient record, built by
  `buildAnalysisRunPreview(result, { inputManifest })` with no rule-check
  aggregate (`previewService.ts:584-600`), not the session's composed record.
  Its `rulePackExclusions` are empty, since route 010 refuses every model with a
  rule-pack reference (§2.7).
- Nothing is written to session state that outlives the save, to the SQLite
  project, to the model document, or to any session snapshot.
- The dialog's open flag joins the modal guards of `runMenuCommand`
  (`workspaceSession.ts:3070`).
- If any basis member changes while the dialog is open, the dialog closes, and
  the next attempt needs a new confirmation.

### 2.4 Solve attestation

Native admission cannot derive a result value from a caller's carrier: the
carrier is supplied by the webview, and bindings establish consistency between
members, not that a solver produced them. So the host records what its own
solver produced. There is no rule-check attestation under R.

**In the worker** (`solve_worker.rs`, and the solve wrapper
`solve_preview_mechanics_with_mode`, `lib.rs:1898-1906`):
- before the solve, over the model value the worker has already materialized
  (`ordinary_solve`, `solve_worker.rs:467-472`): `d_M`, the SHA-256 of
  `checked_v1_domain_canonical("openpipestress.user_release.solved_model.v1", model)`;
- after the unchanged solve wrapper has produced the result bytes the worker
  delivers: `d_S`, the same over `"openpipestress.user_release.solved_result.v1"`
  and the value parsed back from those exact bytes. The parsed value is dropped
  before the frames are sent. (Revision 4, slice r2a, #1664:
  `run_linear_static_preview_value_with_mode` returns a typed
  `MechanicsEnvelope`, not a `Value`, so there is no result value to digest
  before serialization. Parsing the delivered bytes back digests what the
  webview receives, which is the round trip the equality argument below relies
  on. `serde_json::to_value` before serializing would also hold a full copy,
  without existing pricing; a streaming typed writer would change priced source,
  so neither is used.)

Each digest streams through `ByteSink` into a SHA-256 hasher, so no canonical
text is held. If the checked profile refuses either value, the worker sends no
digests. The solve and its result are unchanged; that result cannot be
user-released (`REPORT-USER-RELEASE-RESULT-NOT-ATTESTED`).

**Transport.** A new frame `ATST {status, 0, 0}` precedes `RSLT`, followed by
64 bytes (`d_M ‖ d_S`) when `status` is 1 and by nothing when it is 0. The
parent reads it into a fixed 64-byte buffer. An `ATST` anywhere else, a second
one, or another status or body length is a protocol violation, and the job
fails as any protocol violation does today.

**In the registry.** On publication (`publish`, `solve_jobs.rs:722`), the
registry records `{ job_id, solver_mode, d_M, d_S }` in a bounded session table
of the `MAX_ATTESTED_SOLVES` (32) most recent attested solves, evicting the
oldest. `dispose_all_results` (`:879`, page load) clears it. It is never
persisted and never sent to the webview. The table is separate from the job
records' custody digests (§1, below): it is not pruned with a record and not
dropped when custody ends.

**How this differs from W-11's custody digests (revision 4).** Since #1637 the
registry keeps a native record of what each job received and delivered (§1).
Route 010 does not bind to it. The two records differ in two ways:
- **What they digest.** The custody digests are SHA-256 over raw bytes. A
  check against them needs the webview to resubmit the exact delivered and
  intake texts, and the host to parse those texts under a priced lease
  (W-11 design §3.6 steps 1-3). The attestation digests canonical values, so
  admission compares them over the M and S it already parses.
- **How long they last.** The custody binding ends when the webview disposes
  its copy of the result. The attestation table keeps the 32 most recent
  attested solves until page load.

This design keeps the attestation that §0 names ("Solve attestation") and the
(r2a) slice as designed.

**At admission** (route 010), admission computes the same two digests, by the
same streaming function, over the model value M and the request's
`/source_results/0` (S). The table must hold an entry with those digests whose
mode is the input manifest's `solver_basis.solver_mode`.

Both sides produce the same canonical text, and so the same digest, through the
round trip the Current export already relies on. The producer's bytes are
parsed in the webview, written into the request's RFC 8785 text and parsed
again natively, and the values are equal. A float that serde writes as `5.0`
and RFC 8785 writes as `5` has one canonical form.

A result saved in an earlier session, or one whose attestation has been
evicted, cannot be user-released until it is solved again. The Current export
already requires an in-session native invocation in the webview (§1), so the
user-visible condition is the same; native admission now enforces it.

**What solve attestation costs.**
- **Worker transport:** one frame tag (`worker_alloc.rs:63-79`) and a 64-byte
  body, read by the supervisor before `RSLT` (its `RSLT` arm,
  `solve_worker.rs:1037-1090`), and
  the protocol text of `solve_worker.rs:10-25`.
- **DEL-04-07 memory accounting:**
  - The `d_S` pass holds one parsed copy of the delivered result while it
    runs, within the result text's ingress materialization bound
    (`ingress::prescan`); on the bundled demo the worker's counted peak goes
    from 3,936,832 to 4,842,992 bytes, with the grant unchanged (#1664).
  - Beyond that copy, the worker's two passes allocate only transient pieces in its counted
    heap; no canonical text is held. The pieces live at once are (Codex
    round 3, non-blocking):
    - one key vector per object on the current traversal path:
      `canonical_json::sorted_keys` allocates a `Vec<&String>` of the
      object's member count (8 bytes per member), and it stays alive while
      that object's members, and every object nested in them, are written
      (`core/serialization/canonical_json/src/lib.rs:339-394`). Their sum is
      at most 8 bytes per object member of the value, before allocator
      rounding of each vector. The recursion is at
      most 128 deep, since the checked profile refuses deeper values first
      (`:226-253`);
    - the stable sort's scratch buffer for the object being sorted, freed
      before its members are written;
    - one string's escaped form, from `write_string` (`:335-337`), or the
      bounded simultaneous temporary strings used by `canonical_number`,
      `ecma_number_to_string` and `shortest_digits` (`:400-497`), each freed
      after it is put (Codex round 4, non-blocking).
  - The worker's existing growth path covers that heap. The grant formula, the
    result lease and the 2R custody are unchanged. The allocation checks of §6
    measure the peak on values built to maximize each piece.
  - The parent holds 64 bytes per frame and a fixed table of at most 32
    entries, under 8 KiB, inside its process reserve. There is no new lease.
  - Admission's digests are streamed too, with the same transient pieces,
    inside the command's existing ingress lease. Route 010's lease also
    prices the parse of embedded provenance (§2.7).
  - The memory-budget design's worker paragraph
    (`DEL-04-07/Design/memory-budget-decision.md:37`) names the frame and the
    digest passes. The worker's process-reserve measurement
    (`process_reserve_covers_a_real_workers_measured_footprint`) is run again.
- **Time:** two linear passes in the worker and two at admission.
- **Priced files: none.** Every change is in `apps/desktop/src-tauri`, which is
  outside `core/product_physics`'s dependency closure
  (`DEL-04-07/Design/desktop-exact-outputs.md` §3). `canonical_json` and
  `result_export` are in that closure (`core/product_physics/Cargo.toml`). R
  uses them only through existing public functions, unchanged:
  `checked_v1_domain_canonical`, `ByteSink` and `derive_document`. The solver
  entry `run_linear_static_preview_value_with_mode` is unchanged. **No D3 turn
  is needed.**
  - **Flag.** An implementation that finds it must change
    `core/serialization/canonical_json`, `core/reporting/result_export` or
    `core/product_physics`, for example to add a plain RFC 8785 streaming
    writer, moves priced source. It then needs a HELP_HUMAN D3 turn before it
    proceeds. This design avoids that on purpose.

**Why the worker, not the host.** The host never materializes the model, and
the bytes it receives are not RFC 8785 (§1). A host-side digest would need the
host to parse the result, which needs a new lease and changes the custody
design. The worker already holds the model value it materialized for the
solve, and the result bytes it delivers. For `d_S` it parses those bytes back
into one transient copy, inside the result text's ingress materialization
bound (above), and drops it before the frames are sent. So the host side of
the solve needs no new parse and no new lease (Codex round 4 at revision 4,
N3).

### 2.5 Native admission on route 010

`admit_report_package` and `admit_report_package_in`
(`report_package_save.rs:764-860`). The command stays `save_report_package`;
the route id in the evidence selects the branch. Every existing check runs on
both routes in its current order, except that route 010 refuses the exact
texts in the outer command before they are sized or priced (below). Since
#1648 and #1650 that order is:
- the evidence and the text sizes (on route 009, the exact texts included);
- the evidence's payload hash;
- the request's parse;
- the object-form check (`check_request_object_form`, `:715-722`);
- for an exact-route request, the native source binding
  (`bind_exact_save_source`, `:733-761`);
- the model and input-manifest bindings;
- the disposition screens and the context;
- the input-manifest reference and the rule-pack agreement;
- the typed conversion and the shared producer.

**The outer command (Codex round 1 at revision 4, S1).** Main's
`save_report_package` (`src-tauri/src/lib.rs:66-95`) prices the texts before
admission parses anything. After the evidence check it calls
`parse_lease_bytes_with_exact` (`report_package_save.rs:396-416`). That helper
size-checks, prescans and prices both exact texts, and the command then grows
the lease by the total. So a malformed exact text is refused there as
`REPORT-PACKAGE-SAVE-NOT-CANONICAL`, and a large one grows the lease, before
any check in admission could name route 010's refusal. Route 010 therefore
refuses the exact texts in the outer command:
- The command's pricing becomes one route-aware function,
  `report_package_save::ingress_lease_bytes(&request)`. The command calls it
  in place of `parse_lease_bytes_with_exact`, and tests call it through the
  command's own ingress path (`command_ingress::admit` with a
  `save_report_package` body, as the tests at `lib.rs:9723-9837` do).
- Route 009: `parse_lease_bytes_with_exact`, unchanged.
- Route 010, in this order:
  - the evidence (step 1);
  - if `exact_result_text` or `exact_intake_text` is present,
    `REPORT-USER-RELEASE-EXACT-ROUTE-NOT-ADMITTED`, naming the member. No
    size cap, prescan or pricing runs on it, so a malformed or oversized
    exact text gets this code, and the lease stays at the ingress admission's
    bytes;
  - otherwise route 010's reservation, which is the sum of:
    - the ordinary reservation: `parse_lease_bytes` over the request, model
      and input-manifest texts, plus `text_parse_bytes` of
      `analysisRunCanonicalText` after its cap (step 2);
    - the derivation's working memory (step 9): one more
      `text_parse_bytes` of the request text and of the input-manifest text,
      since the derived request and manifest canonically equal the admitted
      ones;
    - the refusal formatting allowance
      (`user_release_admission::refusal_formatting_bytes`, "Refusals on route
      010" below): four allocation envelopes of one refusal message naming a
      pointer of `pointer_bound` (twice the longer of the request and model
      texts, plus 4,096 bytes), plus 65,536 bytes for bounding the refusal.

The two growths are different things. Route 010's reservation prices the
parse of the four texts that every route-010 save needs before any content
check, the derivation, and the one refusal it may format. The exact-text
growth prices W-11's resubmitted texts, which route 010 never parses, so it
never happens on route 010. Rule 3 (b)'s growth (§2.7) comes later, inside
admission. Every growth is refused above the budget the command was admitted
under (`IngressExceedsBudget`, DEL-04-07).

**Refusals on route 010 (Codex round 1 at #1682, P1).** Every allocation route
010 makes while it screens and refuses is within its reservation, and its
refusal output has a fixed bound:
- **One finding.** Route 010's checks refuse at their first finding. Its
  disposition screen (`report_package_save::screen_content_first`) applies
  `screen_content`'s rules to the same two values in the same order: the
  structural screens' first finding (the request's, then the model's), then,
  per value, the store-resolved screen's first own-site finding
  (`provenance_record`, `rule_pack_refs[]` and non-object members). Each
  first finding is the collecting screen's first entry, word for word
  (`first_findings_match_the_collecting_screens`). The structural screens
  retain their first finding and walk the rest of the value without
  building another, because they share their walks with the collecting
  screens; the store-resolved screen's walk stops at its first finding. Its
  walks hold their path on their own stack frames or in one buffer sized
  before the walk, so they allocate nothing per level or per finding, and
  only the reported finding's pointer is built. The library, rule-pack and
  private-class rules and the derivation's comparison (§2.6) stop at their
  first site, and their messages are written into buffers sized before they
  are written.
  The derivation's refusal builds only the 240 characters of the pointer it
  shows. The run-record check's messages are fixed texts, hashes and
  identities cut to 160 characters. The rule-pack agreement's refusal names
  the count of packs and the first identity, each text cut to 160
  characters.
- **No library store.** On route 010 the store-resolved screen reads no local
  library store. It refuses an object `library_ref`, in the request or the
  model, as `REPORT-USER-RELEASE-LIBRARY-REFERENCED` at that member's
  pointer. Step 5 refuses every library reference of the model whatever the
  store holds, so the store's verdict could never admit anything here. Its
  reads are memory the route's lease does not price. Route 009 still
  resolves every reference in the store.
- **A bounded refusal, at every stage (Codex round 2 at #1682, P2).** A
  route-010 refusal carries at most one diagnostic: an admission refusal
  carries its first finding, and a producer, bridge, save-dialog or
  atomic-writer error carries none. Its path is the finding's JSON pointer
  when that is at most 1,024 bytes. Otherwise it is the nearest enclosing
  pointer within 1,024 bytes, and the message gives the full pointer's
  length. The diagnostic's message and the refusal's message are each at
  most 2,048 bytes. The codes (the refusal's, the cleanup code and the
  diagnostic's) are each at most 128 bytes. The stage, the OS error kind and
  the path containment class are each at most 128 bytes, the selected
  basename at most 256 bytes, and the cleanup message (an OS error's text)
  and the limitation each at most 512 bytes. A longer string keeps its head
  and tail around a note of the bytes elided, so a basename keeps its
  extension (`report_package_save::bound_route_010_refusal`). A refusal
  within these bounds is unchanged. The bound applies to the whole command
  error, whatever stage refused: the outer command's pricing, admission, the
  save dialog, the custody recheck, the atomic writer and the worker. The
  save command returns every outcome through one exit
  (`report_package_save::SaveRoute::exit`), and only that exit makes the
  command's error type (`SaveCommandError`), so no stage can return around
  it. The lease is held through that exit for every outcome but one: when
  the blocking worker panics, the unwind drops the lease moved into it, and
  the command builds the fixed worker-failure error
  (`REPORT-PACKAGE-SAVE-WORKER-FAILED`, `save_report_package_decoded` in
  `lib.rs`) with no lease to return. That error still leaves through the
  exit, and its text is fixed and within the bound. Route 009 leaves the same exit unchanged, byte for byte. The
  ingress layer's own errors (a refused lease or growth, an argument that
  does not decode) and an unserializable response are every command's, of
  fixed form (DEL-04-07), and the exit leaves them unchanged. Codex's repro,
  a 50,000-byte basename that the production atomic writer refuses before
  writing anything, held 50,407 bytes of strings. It now holds 621
  (`route_010_atomic_writer_refusals_leave_bounded`). Every form of the
  error, built at its largest from characters JSON escapes to six bytes, is
  within the bound below
  (`every_route_010_error_form_leaves_within_its_bound`).
- **The response.** An admitted save's receipt is serialized under the held
  lease. On route 010 it carries no warnings, its member summaries are the
  package's fixed members, and its other fields are the checked evidence's
  constants and hashes. A refusal is returned as the command's error. Tauri
  converts it to its IPC value after the command returns and its lease is
  released, so those bytes are priced by no lease. On route 010 they are the
  bounded refusal's strings: at most 7,168 bytes of text, the sum of the
  bounds above (`ROUTE_010_REFUSAL_TEXT_BYTES`), and at most 43,520 bytes
  serialized (`ROUTE_010_REFUSAL_JSON_BYTES`: an escape is at most six bytes
  per byte of text, and the member names, the OS error code and the
  punctuation take less than 512 bytes). Those bytes are the unpriced
  residual of a route-010 refusal. Route 009's
  refusals keep their every finding (pinned byte for byte by
  `route_009_refusals_of_many_findings_are_unchanged`), as DEL-04-07's
  residual records.

Codex's repro (80 levels of 4,000-byte keys over 128 blocking findings)
measured a peak of 272,569,728 priced bytes against a reservation of
52,842,048 before this rule. With it, that shape, the same shape with keys and
findings scaled 2x and 10x, and 100,000 findings at depth 2 each stay within
the growth the lease was given, under a budget of exactly the reservation
(`route_010_refuses_many_findings_within_its_reservation`).

Route 010 adds the steps below, in this order. Every one of them that reads a
member by key runs after the object-form check, as the existing keyed checks
do. The run-record text (step 2) has no struct decoder and is read by key
only, so it needs no object-form comparison, as the request's untyped members
need none (`report_package/src/wire.rs:46-55`).

1. **Evidence** (`validate_report_package_save_evidence`, `:327-361`).
   `ReportPackageSaveScreening` (`:127-142`, `deny_unknown_fields`) and
   `ReportPackageLocalFirstEvidence` (`:147-156`) gain
   `#[serde(default)] explicit_user_release_intent: bool`.
   - Route 009: unchanged, plus `explicit_user_release_intent` false, and no
     `analysisRunCanonicalText` argument.
   - Route 010: context `user_released_report`, user-release intent true,
     local-private intent false, lossless, not withheld, exact payload, zero
     blocking; local-first evidence with storage context `private_report`,
     reason `USER_RELEASE_METADATA_ALLOWED`.
   - Any other route id: `REPORT-PACKAGE-REDACTION-EVIDENCE-INVALID`.
2. **Texts.** The request, model and input-manifest texts, as today. Route 010
   adds `analysisRunCanonicalText`, the solve's DEL-14-02 0.3.0 transient
   record. It must be its own canonical text, within
   `MAX_ANALYSIS_RUN_TEXT_BYTES` (one model-document cap), and priced in the
   command's ingress lease as the input-manifest text is, in the ordinary
   reservation above. Route 010 carries no exact texts (`exact_result_text`,
   `exact_intake_text`): the outer command refuses them (above).
2a. **Exact routes (§0).** This step runs directly after the object-form check
   and before the exact source binding. A route-010 request is refused with
   `REPORT-USER-RELEASE-EXACT-ROUTE-NOT-ADMITTED` when `exact_request` holds,
   that is, when the result document has an exact identity or the report
   carries a non-null `exact_route` (`:724-729`). Admission also refuses a
   supplied exact text the same way, for a caller of `admit_report_package_in`
   that did not come through the command; the command has already refused
   it. No custody is queried and no exact text is parsed. At this point the
   lease holds route 010's reservation (the ordinary reservation, the
   derivation's and the refusal formatting allowance, above) and nothing
   more; it never grows by an exact-text pricing on route 010. Route 009 is
   unchanged.
3. **Context** (`check_context`, `:563-603`).
   - Route 009: unchanged, plus `/report/report_context/user_release` absent,
     else `REPORT-USER-RELEASE-NOT-THIS-ROUTE`.
   - Route 010: `export_context` `user_released_report`, profile
     `desktop_user_released_report_package_1`
     (`REPORT-PACKAGE-SAVE-PROFILE-NOT-DESKTOP` otherwise), and the record
     present (`REPORT-USER-RELEASE-RECORD-INVALID` otherwise).
4. **Record form and bindings.** Exact members and constants as §2.3, no
   `content_binding`, the hex and `user_released_at` forms, else
   `REPORT-USER-RELEASE-RECORD-INVALID`. Then each bound member against a value
   admission itself computed or already bound, never against another caller
   field alone; any inequality is `REPORT-USER-RELEASE-UNBOUND`, naming the
   member:
   - `model_document_sha256` = `model_hex`, the SHA-256 of the model text
     admission received (`:811`);
   - `model_ref` = the model text's `/project/id` = `/source_model_ref/ref_id`;
   - `input_manifest_sha256` = `input_manifest_hex` (`:822`), and
     `input_manifest_ref` = the reference admission derives
     (`analysis_runs::input_manifest_ref`, `:843`);
   - `analysis_run_ref` = `/source_results/0/run_id`;
   - `result_envelope_id` and `result_document_derivative_sha256` =
     `/result_documents/0/result_envelope/envelope_id` and
     `…/reproducibility/derivative_hash/value` (the shared producer then
     verifies that hash over the document);
   - `scope.report_id` = `report:` + `analysis_run_ref` =
     `/report/calculation_report/report_id`, and `scope.package_id` =
     `/package_id` = `desktop-user-released-report-<run>`.
5. **Reference refusals** (§2.7): `REPORT-USER-RELEASE-LIBRARY-REFERENCED` and
   `REPORT-USER-RELEASE-RULE-PACK-REFERENCED`.
6. **Private classes** (§2.7): `REPORT-USER-RELEASE-PRIVATE-CLASS`.
7. **Solve attestation** (§2.4): `REPORT-USER-RELEASE-RESULT-NOT-ATTESTED`.
8. **The run record.** Record Run's binding checks
   (`analysis_runs::checked_submission`, `analysis_runs.rs:634-782`, factored
   to take parsed values) apply to the run record with S and the input
   manifest. They require:
   - the transient form (`created_at` null, the synthetic
     `state:<model>:preview`);
   - the record's own checksum;
   - the members the manifest fixes;
   - the exact two-hash inventory;
   - every row's checksum;
   - diagnostics equal to the result's, with no rule-pack reference
     diagnostics, since step 5 has refused every model with a rule-pack
     reference.

   Route 010 adds:
   - `analysis_status` is exactly `HUMAN_REVIEW_REQUIRED`, S's
     `status.mechanics` and S's own `status.rule_check`, so no rule-check
     aggregate (`previewService.ts:598`);
   - `run_name`, `unresolved_tbd` and `professional_boundary` are the
     builder's values (`analysisRunCompatibility.ts:192-206`).

   Any failure is `REPORT-USER-RELEASE-ANALYSIS-RECORD-UNBOUND`. The record
   reaches the package only through these members and its record hash.
9. **The derivation and the producer, in one call** (§2.6, §2.8). Route 010
   calls `report_package_bridge::assemble_user_released_request`, which calls
   the producer's one user-release entry,
   `report_package::user_release::assemble_derived`. That entry first runs
   its preflight, item 0 and the user-released exact check (§2.8), which
   reads no report member; on route 010, step 2a has already refused every
   exact request. It then requires the input manifest, then the whole
   request, to equal their derivations canonically. Otherwise it refuses with
   `REPORT-USER-RELEASE-CONTENT-NOT-DERIVED`, naming the first differing JSON
   pointer. Only then does it run the shared producer. No caller can reach
   the producer under the user-released profile without the derivation (§2.8).
10. **Unchanged, on both routes:**
    - the object-form check (`:715-722`);
    - the disposition screens' rules and order (`screen_content`,
      `:524-560`); route 010 refuses at the first finding and reads no
      library store ("Refusals on route 010" above);
    - the rule-pack agreement (`:684-700`), whose two sides are both empty on
      route 010 after step 5;
    - the shared producer's checks (§2.8). Route 009 reaches them through the
      general entry, `report_package_bridge::assemble_request`, which refuses
      every user-release marker (§2.8).

### 2.6 The minimal native renderer input and the derivation

The local-private package admits any content the request carries, because it
never leaves the user. A shareable package cannot. Under R, native admission
does not label or bound caller content; it **derives the whole request** from
the bound basis and requires canonical equality. A new module,
`core/reporting/report_package/src/user_release.rs`, does the derivation.
`report_package` is outside `core/product_physics`'s closure.

**The bound basis:**
- **M**, the model text (SHA-256-bound and screened, `report_package_save.rs:811,524-560`);
- **S**, the source carrier, attested for M (§2.4), with its attested solver
  mode **m**;
- **I**, the input-manifest text (bound, `:822-850`);
- **R**, the run record (checked, §2.5 step 8);
- **H**, the host's own software identity (`app.package_info()`, already
  checked by `check_context`);
- **U**, the record.

**The derivation `E(M, S, m, I, R, H, U)`:**
- **Input manifest.** I must canonically equal the manifest
  `buildCurrentSessionInputManifest` builds (`inputManifestService.ts:75-129`).
  It is built from M, S's producer and m, with the solve's fixed settings for m
  (`workspaceSession.ts:1048-1062`), `active_rule_packs: []` and
  `external_assets: []`. The store state at solve time is not re-read; step 5
  has already refused any model with a rule-pack reference.
- **Result document.** `derive_document(base, M, S, origin, None)`
  (`derivative.rs:120`, unchanged). Its base and origin are built by Rust
  mirrors of `currentResultDocumentBase` (`resultExportAdapter.ts:254-256`) and
  `currentReceivedOrigin` (`:241-245`, with the constant `CURRENT_ORIGIN_LIMIT`)
  from M, S, R and I.
- **Renderer input** (the minimal native renderer input):
  - `report_title`: `{M.project.name} — Calculation Report`;
  - `calculation_report`: as the builder writes it (`renderableReportInput.ts:912-988`),
    with:
    - the persistence reference `TBD`;
    - `rule_pack_refs: []`;
    - the project and model references, model hash and load-case summary
      (id, label, kind) from M;
    - the unit display summary as `buildUnitDisplaySummary(M, S)` writes it
      (`renderableReportInput.ts:130-140`). Its members are:
      - the storage convention `entered_units_preserved`;
      - `model_units`, M's `project.units` in key order;
      - `result_units`, the sorted distinct units of S's rows;
      - `quantity_display_policy`, the constant
        `quantity_display::QUANTITY_DISPLAY_POLICY` (`quantity_display.rs:28`,
        the renderer's, `report_renderer/src/lib.rs:1315`);
      - `conversion_performed`, which is
        `quantity_display::source_displays_scaled_value(S)`
        (`quantity_display.rs:54-67`), true exactly when a row of S is stored
        in `Pa`. S is never an exact source (§2.5 step 2a), so no headline
        enters it.

      These are the members the producer checks with
      `check_unit_display_summary` (`bindings.rs:199-207`), and the derivation
      calls the same constant and function (#1655);
    - the result references from R's `result_refs`, each with its row checksum.
      That checksum is the full-precision SHA-256 of the S row's canonical
      form, which `check_report_result_refs` (`bindings.rs:177-191`) requires
      and which R's checked row checksums equal (§2.5 step 8). The rows
      displayed at six significant digits are bound to their stored values by
      these checksums (#1655);
    - the two audit references, the analysis status and the unresolved TBDs
      from R;
    - the builder's constants: template slots, rendered sections, boundary,
      session provenance and privacy class;
  - `report_sections`:
    - the model and run references;
    - S's diagnostics only: `bindings::expected_report_diagnostics(S)`
      (`bindings.rs:234-274`, #1659), the Rust mirror of
      `diagnosticsForSections` (`renderableReportInput.ts:165-186`; vectors
      `fixtures/reports/invented/report_diagnostics_parity.json`). Each entry
      carries the session provenance, which that function leaves unbound. No
      model-provenance or library finding is appended. The producer's
      `check_report_diagnostics` (`:289-305`) runs on the exact route only and
      so not on route 010. Whole-request equality binds every entry, so route
      010 needs nothing more from #1659;
    - the status disclosures from R's status;
    - `provenance_notes`: the session provenance only;
    - `user_supplied_values: []`;
    - `assumptions: []`;
    - the fixed technical-preview limitation;
    - the unresolved TBDs from R;
    - the boundary;
  - `result_rows`: the existing five-field projection of S's rows,
    `bindings::expected_report_result_rows(S)` (`bindings.rs:129-167`), which
    the producer already requires (item 6). Since #1655 each row's
    `quantity_display` is `quantity_display::display_quantity` over the row's
    stored value and unit (`quantity_display.rs:41-44`): six significant
    digits, half away from zero, and a value stored in `Pa` shown in `MPa`.
    The webview mirror is `reportQuantityDisplay.ts`, held equal by
    `report_quantity_display_parity.json`. The minimal input adds no
    numerical-health row;
  - `report_context`:
    - export context `user_released_report`;
    - `generated_at` null;
    - software name and version from H;
    - build identifier null;
    - solver name, version and build reference, run name and settings
      reference from R;
    - `analysis_run_recorded_at` null (the transient record's `created_at`);
    - solver mode and settings from I (each value's canonical JSON text, keys
      sorted, as `buildReportContext` writes them, `:642-683`);
    - model coordinate system null;
    - result coordinate systems from S's row metadata;
    - `user_release` U;
  - `user_rule_checks`: absent;
  - `exact_route`: absent (§2.5 step 2a);
  - `library_references`: absent (§2.7 refuses every model with one).
- **Audit manifest and top-level members**, as `buildReportPackageRequest`
  writes them (`reportPackageRequest.ts:272-309`), with these differences:
  `rule_pack_refs: []`, the route-010 `package_id` and profile,
  `state_comparison_handoff_records: []`, `rule_check_aggregate: null` and
  `solve_rule_check_status` equal to S's.

The minimal input differs from the local-private input in what it leaves out:
- component and spring-hanger provenance entries;
- dedicated per-object hanger quantity entries;
- since W-11 (c) (#1639), the material, section and support provenance
  entries, the Library Records list and the library-store findings (W-11
  design §4.1-§4.2);
- every provenance diagnostic;
- rule-pack references and user-rule outcomes.

These return with the later slice (§10), subject to the owner's ruling on
#1639 (below).

**The owner's ruling on #1639 ("C1 b, C2 a", W-11 design §0) holds under R.**
- C1 (b): a record whose redistribution, review or privacy status is `unknown`
  or `TBD` "stays report-blocking" in every context but local-private, and
  the ruling names "W-18b's user-released report". The renderer judges
  `user_released_report` as not local-private (`provenance_context`,
  `report_renderer/src/lib.rs:1659-1664`), so that rule applies unchanged.
- C2 (a): the user's authored record text "stays as written in local-private
  outputs; no other context carries these records". The minimal input carries
  none of those records. Its only provenance record is the session provenance
  (`sessionProvenance`, `renderableReportInput.ts:142-153`), and the
  application writes it. It is `private_project_data`, `private_only` and
  `pending`, so it is resolved.
- **A renderer change this needs (r2c).** Since #1639 the renderer refuses any
  private record content when the context is not `local_private`
  (`REPORT_PROVENANCE_CONTEXT_NOT_LOCAL_PRIVATE`,
  `provenance_disposition_blocking_reasons`,
  `report_renderer/src/lib.rs:1731-1820`). The session provenance on the
  minimal input's diagnostics, notes and limitation would trip it, so a
  user-released render would always refuse. For `user_released_report` with
  a record, the rule therefore treats a record whose privacy class is
  `private_project_data` as the user-released class, which does not trip it. Any
  record with another non-public class still trips it, with the message
  naming `user_released_report`. This is the renderer's form of §3.2 steps 10
  and 11. Every other context keeps the rule exactly, and the
  disposition-blocking and unresolved-record rules are unchanged.
- **Who can reach that rule.** The renderer cannot name its caller, so the
  rule is closed at every product entry that reaches the renderer. Only the
  producer's derivation-bound entry, called only by route 010's admission,
  can render a `user_released_report` input with a record. The runner, the
  producer's general entries and the render command each refuse it (§2.8,
  "Who can reach a user-released package").

The solve's review rows are retained. `result_rows` is the projection of every
`results` row of S, so a spring hanger's user-entered stiffness and loads
appear, as the solver wrote them, in S's `spring_hanger_user_input_review` rows
(`core/product_physics/src/lib.rs:13574,13675`), for an ordinary user-entered
hanger as for any other. Only the dedicated per-object entries are left out.

**Parity.** The webview builds the same request with
`buildUserReleasedReportPackageRequest` (`reportPackageRequest.ts`) and a
`userReleasedRenderableReportInput` builder. Each Rust builder is held equal to
its TypeScript counterpart by committed invented vectors,
`fixtures/reports/invented/user_release_derivation_vectors.json` (inputs M, S,
m, I, R, H, U and the expected request), read by a vitest and by a Rust test,
both routed. A change to a builder changes its mirror and the vectors together,
as W-18a's row parity does (W-18a design §2.3 item 6).

**Why derivation replaces round 1's closed table.** Every member is equal to a
fixed derivation from M, S (the solver's own output for M), R, I, H and U, so
no member or value can exist that the derivation does not produce. That closes
Codex round 1 finding 1 (unconstrained annotations and opaque records) for R:
- source annotations come only from the attested S;
- there is no handoff record;
- an extra key added only in the request fails the attestation or the
  equality.

It closes finding 3 (a library quantity under a substituted source) too: R
carries no per-object entries, and every model with a detected library origin
is refused (§2.7).

### 2.7 The library and rule-pack refusals, and private classes

**"Library-referenced" is determined from the bound model M alone.** The
detector recognizes each form in which a library apply has ever written a
library origin into a model. The history of the two writers (`git log` of
`hangerSelection.ts` and `libraryApply.ts`) shows three forms:

| Form | Written by | Where in M | Rule |
|---|---|---|---|
| A `library_ref` object in `provenance_record` | every W-17 apply (material, section, component, hanger) from `00af9112c0`, 2026-10-23 (`libraryApply.ts:275`; `hangerSelection.ts:55,70-82`) | the applied entity's `provenance_record` | 1 |
| The library provenance string `library:<kind>:<library id>#<record id>` | the same applies (`libraryApply.ts:81-83,450,496-497,562,753-759`; `hangerSelection.ts:41-42,54`) | the entity's `provenance`; component source-reference strings; a hanger support's `provenance` | 2 |
| The legacy hanger selection evidence: the RFC 8785 JSON text of `{hanger_id, kind: "user_imported_hanger_selection", library_id, library_kind: "hanger", library_metadata, record_snapshot}`, held as a string | the hanger selection from `46f3aed341` (2026-09-05) until `00af9112c0` replaced it (`hangerSelection.ts:35-39,52` at `00af9112c0^`) | a hanger support's `provenance`, with no `library_ref` and no `library:` string | 3 |

There is no other form. Before `00af9112c0` no material, section or component
apply existed: `libraryApply.ts` arrived in that commit, and the model's
`LibraryRef` with model 0.7.0 (`60ed7f6478`). The hanger selection is the only
writer of `user_imported_hanger_selection` in the history. Committed model
fixtures carry only the W-17 forms (`fixtures/model_document/desktop_emitted.json`,
`library_ref_sites.json`). Re-checked at revision 4: since revision 3, the only
change to the library writers is W-17's disposition refusal (#1653,
`libraryApply.ts:99-146` and its two call sites). That change refuses an
unreadable disposition before apply and writes no new mark, and
`library_import_document`'s changes in the same PR are import-time disposition
checks. The re-apply note `Applied over the previous library reference
library:<kind>:…` (`libraryApply.ts:271`) sits in the `provenance_record` of
an entity that also carries its new `library_ref`, and rule 2 matches it too.

**The rules.** A *string* here is every JSON string value at any depth of M:
an object member's value or an array element. Object keys are not tested. The
substring tests are exact and case-sensitive, over the string's characters,
with no trimming. M is library-referenced when any of these holds:
1. **A `library_ref` member at any depth, whatever its value.** An object
   member named `library_ref` counts whether its value is an object, `null`,
   a string, a number, a boolean or an array (a positional record among
   them). A site whose identity is blank counts too. The existing helpers
   cannot give this rule on their own, because both collect objects only:
   - natively, `export_disposition_screen::library_ref_sites`
     (`export_disposition_screen.rs:504-515`) keeps only
     `DocumentSite::LibraryRef`. The walk visits a non-object `library_ref`
     as `DocumentSite::NotAnObject { member: "library_ref", .. }` (`:485`,
     W-17 object form, #1651), and that helper discards it;
   - in the webview, `libraryRefSites` (`libraryImportService.ts:440-457`)
     records a `library_ref` only when `isRecordObject(child)` holds.

   So each layer gains its own detector over the same walk, and the existing
   helpers stay unchanged (Record Run's revision and the pinned
   `fixtures/model_document/library_ref_sites.json` keep their meaning):
   - natively, `first_library_ref_member(M)` in `export_disposition_screen.rs`
     finds the first member named `library_ref`, whatever its value, in
     `walk_document`'s order, and stops there. It is pinned to the collecting
     walk `library_ref_member_sites`, which keeps, in walk order, every
     `DocumentSite::LibraryRef` and every
     `DocumentSite::NotAnObject { member: "library_ref", .. }` and is pinned to
     the shared member table; admission does not call the collecting walk;
   - in the webview, `libraryRefMemberSites(model)` beside `libraryRefSites`
     walks the same way and keeps every `library_ref` key, whatever its value.

   Rule 1 holds when that detector finds a site. Native disposition screening
   already refuses a non-object `library_ref` in M on both routes, as
   `MODEL-DISPOSITION-RECORD-MALFORMED` (`:398-404`). Rule 1 does not rely on
   that: it names such a member as a library site whichever check runs first,
   and it also applies inside embedded JSON (rule 3 (b)), which the
   disposition screen never sees.
2. **A string that holds a library provenance reference**: it contains one of
   `library:material:`, `library:section:`, `library:component:` or
   `library:hanger:`. A model whose `library_ref` was removed but whose values
   still name their library is refused too.
3. **A string that holds legacy hanger selection evidence**, in either of two
   ways:
   - (a) it contains `user_imported_hanger_selection`;
   - (b) it is *embedded JSON*: after removing leading and trailing JSON
     whitespace (space, tab, line feed, carriage return), it begins with `{`,
     `ingress::prescan` (`src-tauri/src/ingress.rs:550`) accepts it as one
     JSON value, and it parses (`serde_json::from_str::<Value>`) to an object.
     It holds legacy evidence when that object has a member named
     `library_kind`, `library_id`, `library_metadata` or `record_snapshot`, or
     when rule 1, 2 or 3 (a) holds within the parsed value
     (`holds_library_ref_member` over it, and the substring tests over its
     strings; each stops at its first hit).

   Parsing goes one level deep. Strings inside the parsed value are tested by
   the substring rules, not parsed again: no apply wrote embedded JSON inside
   embedded JSON. A candidate the prescan refuses for nesting deeper than 127
   containers (`ingress.rs:29,605-606`), for serde_json's reserved raw-value
   key (`:653`; both pinned by the test at `:1046-1056`), or as too large to
   price (`:355-360`) cannot be checked, and is refused under rule 3. Every
   other prescan finding is one serde_json's parse would refuse too. A string that does not begin with `{`, that the prescan
   refuses with such a finding, or that parses to a non-object is text only,
   and rules 2 and 3 (a) still apply to it.

The code is `REPORT-USER-RELEASE-LIBRARY-REFERENCED`. It names the first site
found, taking rule 1, then rule 2, then rule 3, each in document order. It
gives the site's JSON pointer, the rule (`library_ref`, `provenance_string` or
`legacy_hanger_selection`), and the library kind and id where the site states
them. For rule 3 the kind is `hanger`. A non-object `library_ref` states
neither, so its message gives the pointer and the rule only.

**Cost.** Rules 1, 2 and 3 (a) are linear passes over the parsed M. Every
walk of these rules, of the rule-pack rule and of the private-class rule stops
at its first site and holds its path on its own stack frames, borrowing each
key: it allocates nothing per level, whatever the depth or the keys' length.
Only the site that is reported is built: its JSON pointer in M, at its exact
length, and the library kind and id, each kept to the 161 characters the
message can show. The same walks over a parsed embedded value (rule 3 (b))
decide at the first hit and allocate nothing (Codex round 1 at #1673: a walk
that collected every site and built each level's path allocated about 94
times the lease growth). Admission
refuses at the first site. For rule 3 (b), admission first prescans every
candidate (a string that, trimmed as above, begins with `{`; `ingress::prescan`
allocates nothing) and grows the
command's ingress lease once, by the largest candidate's
`text_parse_bytes(·, Moved)`. It then parses the candidates one at a time and
drops each before the next, so at most one embedded value is held beside M,
and the peak beyond M stays within that growth and the reported site.
That bound covers detection (`library_reference_site`). Formatting the
refusal message from the reported site (`refuse_library_referenced`) allocates
further, in proportion to the site's pointer and kept texts (Codex round
2 at #1673: 8,388,656 bytes on 124 enclosing keys of 4,000 bytes each, against
7,280 bytes of growth plus a 2,099,200-byte site allowance). Slice (r2c)
prices that formatting within route 010's ingress lease, which the
budget-bounded growth rule of DEL-04-07 (draft #1676) caps at the budget the
command was admitted under. The same allowance covers the disposition
screen's first finding, and every route-010 refusal is bounded (§2.5,
"Refusals on route 010").

**The webview** applies the same three rules as an early check
(`libraryRefMemberSites`, the substring tests, and `JSON.parse` in a `try` for
3 (b)),
which disables the dialog's save (§2.2). Native admission decides; it alone
refuses the unchecked candidates, which `JSON.parse` may accept.

**The reliance limit.** The rules find the marks that library applies wrote.
They cannot find a library origin that a user has erased: a value whose every
mark was removed, or a value typed in by hand from a library or another
source, has no detectable library origin. The detector therefore never shows
that a value is the user's own. For every value without a detectable library
origin, the user release relies on the user's own statement of their rights, the
checkbox statement of §2.2 that the record carries (§2.3). The record states
that statement; it does not prove it (§2.9).

This resolves Codex round 2's blocker and round 3's. The library-applied
bend's raw flexibility factor and SIF reach result rows, contract evidence and
diagnostics through genuine solver output (§1), and so do a legacy library
hanger's loads and stiffness, in `spring_hanger_user_input_review` rows
(`core/product_physics/src/lib.rs:13574,13675`). Attestation reproduces them.
Under R a model with a detected library origin never reaches the derivation.

**Rule-pack-referenced.** M is rule-pack-referenced when the walk finds any
`rule_pack_refs[]` entry (`walk_document`, `export_disposition_screen.rs:454-497`), or when I
records any active rule pack. The code is
`REPORT-USER-RELEASE-RULE-PACK-REFERENCED`. HELP_HUMAN settled this refusal
on 2026-10-25 (§0). It follows from R:
- R carries no rule-pack reference and no rule-check outcome;
- the package's agreement rule requires the package to name every active pack
  of the solve (`report_package_save.rs:684-700`), which needs the rule-check
  run that R does not carry;
- references the manifest leaves out become rule-pack diagnostics in the run
  record (`analysis_runs.rs:298-343`).

The later slice, which stays in v0.1, carries rule-pack references at safe
metadata and lifts this refusal together with the library refusal (§10). Until
then the dialog says plainly that such a model cannot be shared yet, and why
(§2.2).

**Private classes (defence).** Any object in M or the request whose own
`privacy_classification` or `privacy_class` is a non-project private class is
refused with `REPORT-USER-RELEASE-PRIVATE-CLASS`. The non-project private
classes are `private_material_data`, `private_component_data`,
`private_rule_pack_data`, `owner_standard_data`, `company_design_basis_data`,
`path_data` and `secret_like_data`. A label can add a refusal; no label grants
inclusion.

### 2.8 Shared producer, writer and render command

**Shared producer** (`report_package::bindings::check_request` and
`check_request_in`, `bindings.rs:345-683`, reached through
`wire::assemble_wire_request_in`, `wire.rs:366-408`). There, item 0 (the route
preflight) and the report's object-form check run before `check_request_in`,
unchanged:
- **exact routes (§0):** a check of the same form as
  `exact_sections::check_runner_profile` (`bindings.rs:373`;
  `exact_sections.rs:86-93`), `exact_sections::check_user_released_profile(export_profile_id,
  document)`, refuses an exact identity under `USER_RELEASED_DESKTOP_PROFILE`
  with `REPORT-USER-RELEASE-EXACT-ROUTE-NOT-ADMITTED`. It reads only the
  profile it is given and the document's producer identity, so it refuses
  whatever the report carries. It runs in two places (Codex round 2 at
  revision 4, S6):
  - in the admitting entry's preflight, before the derivation, with the
    profile constant rather than the request's profile (below, refusal 3).
    This is where an exact document on the user-release path is refused;
  - in `check_request_in`, in this order: item 0, `check_runner_profile`, the
    general entries' marker refusal (refusal 2 below), then this check. Under
    a general entry the marker refusal has already refused every request with
    the user-released profile, so this check does not fire there. Under the
    user-release entry the preflight has already run it on the same
    documents, so it does not fire there either. It stays as the producer's
    own statement of the rule at the point where the profile is admitted.

  Item 0's admitted set is not changed and no second `AdmittedSet` is made, so
  W-11's forwarding-graph scan (`admitted.rs:1-15`) still holds, with the one
  root that the admitting entry adds (below). One code
  serves the webview, native admission and the producer, as HELP_HUMAN's
  confirmation names "its own named code at all three layers";
- the profile table (`:392-396`) gains
  `(USER_RELEASED_DESKTOP_PROFILE, DESKTOP_ORIGIN) => true`, which only the
  user-release entry reaches (below);
- for that profile the request must carry no handoff record set
  (`REPORT-PACKAGE-USER-RELEASE-UNBOUND` otherwise). The checks that read the
  handoff's run section (`:487-546`, `:583-589`, the handoff status at `:623`)
  have nothing to read. The run record they bind is not in the request
  (`bindings.rs:8-11`) and is bound natively by admission (§2.5 step 8). Every
  other item runs unchanged;
- item 7 (`:631-666`) is unchanged: with no aggregate, the report carries no
  outcomes and no rule-pack reference;
- item 8 (`:668-671`) becomes:
  - under the general entry (below), unchanged: the local-private profiles
    (desktop and runner) require `export_context` `local_private`. A record
    never reaches it there, since the general entry has already refused every
    user-release marker;
  - under the user-release entry, the request must carry the user-released
    profile, `user_released_report` and a record
    (`REPORT-PACKAGE-USER-RELEASE-UNBOUND` otherwise);
- item 9 (W-11 §3.5, `:673-682`) is unchanged. With a pressure-free document
  it requires that the report carry no exact sections, and it does not run
  `check_report_diagnostics`;
- a new item 10 (item 9 at revision 3, renumbered because W-11 took that
  number) checks the record against the request's members, each inequality
  `REPORT-PACKAGE-USER-RELEASE-UNBOUND`:
  - `model_ref` = `source_model_ref.ref_id` = the document's
    `model_ref.ref_id` = `audit_manifest.model_hash.payload_ref`;
  - `model_document_sha256` = `audit_manifest.model_hash.value` = the
    document's model hash;
  - `input_manifest_sha256` and `input_manifest_ref` = the audit
    input-manifest hash value and payload reference;
  - `analysis_run_ref` = the document's `run_ref.ref_id`;
  - the envelope id and derivative hash = the document's;
  - `scope.report_id` = `report:<analysis_run_ref>` = the report's
    `report_id`, and `scope.package_id` = the request's.

**Who can reach a user-released package (Codex round 1 at revision 4, B1).**
Revision 4 said that the headless runner "keeps its profile". That was wrong.
Main's runner decodes the caller's `export_results` as a complete package
request (`decode_report_package_request`, `openpipestress-runner.rs:473`). It
forwards the request's `export_profile_id` unchanged to `assemble_wire_request`
(`:549-556`). Its own checks impose no profile: the exact-identity refusal
(`:496-511`) and `validate_export_results_bindings` (`:614-690`). As
revision 4 specified r2c, a caller could submit a coherent pressure-free
desktop-origin request with the user-released profile and a record:
- it would reach the renderer's user-released class rule (§2.6) with no model
  or reference screening, no solve attestation and no whole-request
  derivation;
- session provenance notes that the caller added would satisfy that rule;
- `--explicit-local-private-intent` would then keep the package bytes as one
  opaque value (`core/runner/headless/src/redaction_binding.rs:407-443`).

The design closes this in (r2c), the slice where the profile, the record type
and the renderer's `UserReleasedReport` value first exist. Before (r2c) the
renderer's decode refuses the context and the profile table refuses the
profile. Three refusals apply, and each reads the request itself, not the
origin it claims. A request carries a **user-release marker** when any one of
these holds:
- its `export_profile_id` is `desktop_user_released_report_package_1`;
- its `report.report_context.export_context` is `user_released_report`;
- its `report.report_context` has a `user_release` member, whatever its value.

1. **The runner boundary.** `execute_export_results` refuses a payload that
   carries any marker. The check runs directly after the decode, beside the
   exact-identity refusal, and so before `validate_export_results_bindings`
   and the producer. The diagnostic is `HEADLESS_RUNNER_EXPORT_RESULTS_PAYLOAD_INVALID`
   with the message `REPORT-USER-RELEASE-RUNNER-NOT-ADMITTED: the headless
   runner packages no user-released report; only the desktop's route
   DREP-PACKAGE-USER-RELEASE-010 builds one (marker: <member>)`. `<member>`
   is one of a closed set of three fixed paths, one per marker above:
   `export_profile_id`, `report.report_context.export_context` and
   `report.report_context.user_release`. When more than one marker is
   present, it is the first in that order. The message never carries the
   member's value (Codex round 2 at revision 4, N1). The command exits 1, its output has no
   `report_package` and it writes no `--output` file, with or without
   `--explicit-local-private-intent` (`openpipestress-runner.rs:169`). One
   marker alone is enough, so the runner also refuses the profile without a
   record, and a record under the runner or the desktop local-private profile.
   The check reads the typed profile and reads the report by key. A report in
   positional form is refused by the producer's object-form check
   (`wire.rs:376-381`) before any keyed check, so no positional form can carry
   a marker past both.
2. **The producer's general entries.** `wire::assemble_wire_request`,
   `bindings::check_request` and `assemble_report_package_container`
   (`lib.rs:204`) are the public entries that the runner, route 009's bridge
   (`report_package_bridge::assemble_request`) and the tests call. Each
   refuses any marker with `REPORT-PACKAGE-USER-RELEASE-NOT-ADMITTED`. The
   first two refuse in `check_request_in`, after its two cardinality checks,
   item 0 and `check_runner_profile`, and before the user-released exact
   check (the order is under "exact routes" above); the writer refuses before
   it renders.
   - **What runs before `check_request_in` (Codex round 3 at revision 4,
     N2).** `bindings::check_request` goes straight to `check_request_in`
     (`bindings.rs:350`). `wire::assemble_wire_request` reaches it through
     `assemble_wire_request_in`, which first runs three checks of its own,
     unchanged from the basis: item 0 (`wire.rs:373-375`), the report's
     object-form check (`:376-381`), which refuses a positional report with
     its own diagnostic, and the audit-manifest conversion (`:382`).
     So a marker request whose report is in positional form, or whose audit
     manifest does not convert, gets that diagnostic first at this entry.
   - **What runs first inside `check_request_in` (Codex round 4 at revision
     4, N2a).** Reaching `check_request_in` does not by itself give the
     marker refusal precedence. Its two cardinality checks run first,
     unchanged from the basis (`bindings.rs:361-365`): exactly one result
     document, and one source result per result document. A marker request
     that fails either gets that diagnostic, whatever its document, and at
     `bindings::check_request` it gets it even before item 0. The marker
     refusal and the precedence below apply to the requests that reach
     `check_request_in` and pass those two checks. Either way the request is
     refused and nothing is built.
   - **Precedence for an exact document (Codex round 2 at revision 4, S6).**
     A user-release request whose result document is exact never gets
     `REPORT-USER-RELEASE-EXACT-ROUTE-NOT-ADMITTED` at a general entry:
     - at `wire::assemble_wire_request` and `bindings::check_request`, it gets
       item 0's own refusal if its identity is outside the admitted set, and
       otherwise, once it reaches `check_request_in` and passes its two
       cardinality checks (above), `REPORT-PACKAGE-USER-RELEASE-NOT-ADMITTED`.
       At `bindings::check_request` the cardinality checks come before item
       0, so a request that fails one gets that diagnostic instead. With the runner
       profile and a record it gets W-11's
       `REPORT-PACKAGE-EXACT-RUNNER-PROFILE` first, since that check runs
       directly after item 0 as W-11 placed it;
     - at `assemble_report_package_container` it gets
       `REPORT-PACKAGE-USER-RELEASE-NOT-ADMITTED`, whatever its identity. The
       writer's refusal runs before it renders, and so before the writer's
       own item-0 check (`lib.rs:265`), which follows the render.
     The reason: a general entry builds no user-released package of any
     content, so what it owes the caller is that this is the wrong entry. The
     exact-route code says why a user-released package refuses a result, and
     only the admitting entry builds one. The marker refusal also reads only
     the profile and two `report_context` members, so it too does not depend
     on the rest of the report.
3. **One admitting entry.** `report_package::user_release::assemble_derived(request,
   linked, basis)` is the only entry that admits the user-released profile.
   - **Preflight, before the derivation (Codex round 2 at revision 4, S6).**
     It first runs two checks on each result document, in this order. Neither
     reads any member of the report:
     1. item 0, through the public `document_form::check_route_admitted`
        (`document_form.rs:47`, the production root), so an identity outside
        the admitted set gets item 0's own refusal, as on every producer
        entry, and item 0 still runs before any other check (W-11 §5.5). No
        edge is added to W-11's forwarding graph, whose rows are the `_in`
        variants;
     2. `exact_sections::check_user_released_profile(USER_RELEASED_DESKTOP_PROFILE,
        document)`, with the profile constant, not the request's profile. An
        exact document is therefore refused with
        `REPORT-USER-RELEASE-EXACT-ROUTE-NOT-ADMITTED` whatever the request's
        profile, record or report carries, including a report that differs
        from the derivation or is in positional form.
   - Only after the preflight does it compute `E(M, S, m, I, R, H, U)` (§2.6),
     and require the input manifest and the whole request to equal it
     (`REPORT-USER-RELEASE-CONTENT-NOT-DERIVED`).
   - It then calls `wire::assemble_wire_request_in` with `admitted::PRODUCTION`
     and a crate-private entry value, `ProducerEntry::UserRelease(DerivedUserRelease)`.
     `DerivedUserRelease` has a private field and derives neither `Clone`,
     `Copy` nor `Default`. Only the derivation check, the module-private
     `user_release::check_derived`, constructs one, and only `assemble_derived`
     calls that check (reach scan). The general entries pass `ProducerEntry::General`, which
     refuses every marker. `assemble_wire_request_in` forwards the entry to
     `check_request_in` and to `assemble_report_package_container_in`, so the
     writer admits a record only under the user-release entry.
   - Native admission's route-010 branch reaches it through one new bridge
     function, `report_package_bridge::assemble_user_released_request` (§2.5
     step 9).

**Why this mechanism.** Rust visibility cannot name a caller in another
crate. `src-tauri` and the runner each link `open_pipe_stress_report_package`
as an external path dependency, so both can call any `pub` entry. A Cargo
feature enabled only by `src-tauri` was considered and not chosen. Any
manifest can turn a feature on, so it would still need the scan below, and
the crate's own tests would need a second build configuration. So the
producer does what it can express:
- it admits the profile only together with the whole-request derivation, so
  no caller reaches the writer with a record without that derivation;
- the checks that need the session stay in native admission (§2.5 steps 5-8):
  model screening against the local store and solve attestation against the
  registry;
- a scan pins the one admitting entry to its one caller, and every renderer
  and PDF call in the crates that can make one to its allowed callers.

**No second admitted set.** `AdmittedSet` and `PRODUCTION` do not change. The
entry value is not an admitted set and changes no item-0 decision. W-11's
forwarding-graph scan (`admitted_tests.rs`) keeps all five of its rules. Its
row for `assemble_wire_request_in` gains one root,
`("user_release.rs", "assemble_derived", Arg::Production)`, and no variant
becomes `pub fn`.

**The reach scan** (`user_release_reach_scan`, a `src-tauri` test). It was
extended after Codex round 2 at revision 4 (S5). After round 3 (S5a, S5b) it
reads parsed Rust rather than lines. It reads the production source of four
crates: `apps/desktop/src-tauri`, `core/reporting/report_package`,
`core/reporting/pdf_emitter` and `core/reporting/report_renderer`.

Revision 4 had borrowed `forwarding_graph_scan`'s `non_test_source` and its
call-site rule (`forwarding_graph_scan.rs:8-17`). Codex round 3 showed that
those rules do not fail closed:
- they find test code by column and the enclosing function by indentation,
  so a module-level `macro_rules!` after a function, or a closure laid out at
  the function's own indentation, carries a call that the rules attribute to
  the wrong function;
- they drop a file named by a `#[cfg(test)] mod` even when a production
  declaration also loads it;
- they do not see code brought in by `include!`.
The reach scan uses neither rule.

*Parser.* The scan parses each file with `syn::parse_file` and walks the tree
with `syn::visit::Visit`.
- **Dependencies.** `src-tauri` gains two dev-dependencies, pinned to the
  versions already in `apps/desktop/src-tauri/Cargo.lock`:
  - `syn` `=2.0.117`, with `default-features = false` and features `full`,
    `parsing` and `visit`;
  - `proc-macro2` `=1.0.106`, with feature `span-locations`, which gives line
    numbers in findings.
  `syn`'s only other required dependency, `unicode-ident` 1.0.24, is also
  locked there. Its optional `quote` (1.0.45, locked) is not enabled.
- **Offline.** The lock gains two dependency edges on the
  `openpipestress-desktop` package and no new package entry. This host's
  Cargo registry cache holds `syn-2.0.117.crate` and
  `proc-macro2-1.0.106.crate`, so an `--offline` build needs no fetch. Both
  points come from the lock file and the cache, not from a build. (r2c)
  updates the lock offline, then confirms both with an `--offline --locked`
  test build through `tools/t3_cargo.sh`.
- **Why not a restricted-syntax lexer** (Codex's alternative). It would have
  to forbid forms that these crates use today, so it would need an allow-list
  from the start: `app_invoke_handler`, and two production modules under a
  non-test `cfg` (below). Its scope tracking would also be a hand-written Rust
  lexer, the kind of rule that let S5a through. A parsed tree gives lexical
  scope directly, and it treats cfg-gated code as production with no
  exception list. The macro lists below (Codex round 4) are allow-lists too,
  but they cover only macros, which no parser can see into; scope still
  comes from the tree.

*Which source is production.*
- **Roots.** The roots are each crate's `src/lib.rs`, plus `src-tauri`'s
  `src/main.rs`. The scan reads the four manifests. It fails if a `[lib]`
  names a path other than `src/lib.rs`, if a manifest declares a `[[bin]]`
  target, or if a crate has a `src/bin` directory. None does at the basis.
- **Modules.** From each root the scan follows every `mod` item, outlined
  (`mod name;`) and inline. It resolves the files itself, by Rust's rules:
  - `name.rs` or `name/mod.rs` beside a root or `mod.rs` file;
  - otherwise the same names in the directory named after the declaring
    file;
  - under inline modules, the modules' names as further directories.
  A declaration whose file is missing, or that matches both candidates, is a
  finding.
- **Test-only items.** An item is test-only when one of its outer attributes
  is exactly `#[cfg(test)]`. Every other item is production, whatever `cfg`
  it carries. That includes `#[cfg(not(test))]`,
  `#[cfg(feature = "native-smoke")]`, `#[cfg(target_os = "macos")]` and
  `#[cfg(any(test, …))]`, and an item gated only through a `cfg_attr` or an
  inner `#![cfg(…)]`.
  Test-only items are not read; that includes test modules, outlined or
  inline. Statements and expressions are read whatever their attributes.
- **Files.** A file is production when any production declaration loads it,
  whatever test declarations also name it. A file named only by test
  declarations is not read.
- **Forbidden in production.** Each of these is a finding:
  - a `#[path = …]` on a `mod`;
  - a `cfg_attr` that applies `path`, at any depth of nested `cfg_attr`s and
    whatever its predicate. The scan does not evaluate predicates, so it
    cannot know which file such a module loads; it refuses the attribute
    rather than follow one candidate (Codex round 1 at #1682, P2b);
  - an `include!`, `include_bytes!` or `concat_idents!`, by any path
    (`std::include!`, `core::include_bytes!`), invoked or imported (code
    review r4 at #1682; *Unsafe code and macro names* below);
  - an `include_str!` by any path but its bare name, or whose argument is
    not one string literal, or names a `.rs` file;
  - a production file that does not parse.

  So no production Rust, generated or external, reaches these crates through
  a path the scan does not read, except through macro expansion, which the
  next rules close.

*Macros (Codex round 4 at revision 4, S5c, S5d, N4).* `syn` leaves a macro's
body and arguments as tokens, and the scan does not expand macros. Screening
those tokens for scanned names is not enough:
- **S5c.** A macro can emit a `mod`. For example,
  `macro_rules! load_escape { () => { pub mod escape; }; } load_escape!();` in
  `report_package/src/lib.rs` loads an `escape.rs` that the module traversal
  never reads.
- **S5d.** A macro can build a call path from its parameters. For example,
  `macro_rules! build { ($r:ident, $i:expr) => { $r::render_calculation_report($i) }; }`
  invoked as `build!(open_pipe_stress_report_renderer, input)` in `src-tauri`.
  Neither token stream holds the qualified sequence that the `src-tauri`
  rule matches, and the bare name is not matched there.
- **N4.** A declarative macro defined outside the four crates can emit a
  call without the name appearing in the invocation.

So the scan limits which macros may exist and which may be invoked, in the
production source of the four crates. Each rule below is a finding when
broken.
1. **Definitions.** No `macro_rules!` item and no `macro` (2.0) item, except
   one on the allow-list. An entry names the file and the macro, and pins the
   SHA-256 of its source text: the file's bytes from the `macro_rules` token
   through the closing delimiter, located by the item's span. A change to any
   token, comment or whitespace in those bytes fails the scan until the macro
   is reviewed again and its pin is replaced. An entry is admitted only if
   its text holds no `mod` token, no `$` parameter followed by `::`, and no
   scanned name other than one the name rules below allow. The list has one
   entry:
   - `app_invoke_handler`, `apps/desktop/src-tauri/src/lib.rs:5018-5081`
     at the basis (2,455 bytes, LF line ends, ending at the closing `}`),
     SHA-256 `45c3d325100524094e8e57c33a3a1d13e12abb5cdf2abfed51224be5adbe1ed1`.
     #1665 (B11 model checks) added two ordinary handler entries,
     `check_preview_model` and `cancel_model_checks`; reviewed, it was
     re-pinned at `:5177-5242` (2,521 bytes), SHA-256
     `cf2d34eb5102ed6d339c9ce067583bcd4e4ef3e1bb1faa63eb701c4f8ff8c1b5`.
     It holds no `mod`. Its one parameter, `$extra:path`, is used only as a
     whole handler entry (`$(, $extra)*`), never before `::`. Its one scanned
     name is the Tauri command's bare `render_calculation_report` (`:5076`
     at the basis, `:5237` now), which the `src-tauri` rule does not match.

   Also findings: `#[macro_export]` and `#[macro_use]` anywhere, and any
   item that `syn` gives as `Item::Verbatim`, which is how it gives a
   `macro` (2.0) item. No `macro` item exists at the basis. That `syn` gives
   no other basis item as `Item::Verbatim` is confirmed by the first scan
   run, in (r2c).
2. **Function-like invocations.** Every macro invocation must be on the list
   below, by the path as written. This covers every position: item,
   statement, expression, pattern and type, and an invocation inside another
   macro's tokens, such as the `tauri::generate_handler!` in
   `app_invoke_handler`. The list was derived from the basis's production
   source, so the rule fails nothing today. The counts are from a search of
   that source with comments, strings and `#[cfg(test)]` items left out, not
   from a scan run:
   - **`std` macros, bare:** `format!` (871: `src-tauri` 627,
     `report_package` 145, `report_renderer` 81, `pdf_emitter` 18), `vec!`
     (110), `format_args!` (71), `matches!` (33), `write!` (14), `eprintln!`
     (14), `include_str!` (8, restricted above), `assert!` (5),
     `unreachable!` (5), `debug_assert!` (4), `println!` (1, `main.rs:10`),
     `debug_assert_ne!` (1, `memory_pool.rs:477`), `panic!` (1,
     `schema_eval.rs:170`) and `env!` (1, `handoff_package.rs:1400`);
   - **external, bare, imported by name:** `json!` (101, from
     `serde_json`) and `params!` (73, from `rusqlite`);
   - **external, by path:** `serde_json::json!` (2, `native_smoke.rs:152,165`),
     `rusqlite::params!` (2, `stored_read.rs:398,409`),
     `serde::forward_to_deserialize_any!` (3,
     `command_ingress.rs:408,491,564`), `tauri::generate_handler!` (1,
     `lib.rs:5020`) and `tauri::generate_context!` (1, `lib.rs:5108`);
   - **local:** `app_invoke_handler!` (2, `lib.rs:5094,5096`).

   `thread_local!` occurs twice, but both are `#[cfg(test)]` items
   (`schema_eval.rs:750-751`, `handoff_package_publish.rs:826-827`), so it
   is not listed.
3. **Attributes and derives.** Every attribute's path, outer or inner, must
   be one of these, except each reporting crate's crate-root
   `#![forbid(unsafe_code)]` (*Unsafe code and macro names* below), which is
   admitted only there and only in exactly that form:
   - built in: `cfg`, `cfg_attr`, `allow`, `inline`, `cold`,
     `global_allocator`, `derive`, and `doc` (doc comments). The attributes a
     `cfg_attr` applies, at any depth and whatever the predicate, are checked
     by every rule here as a direct attribute is: this list, the derive list,
     the `serde` keys, `tauri::command`'s form, `#[macro_export]` and
     `#[macro_use]` (Codex round 1 at #1682, P2b). At the basis each one is
     an `allow(dead_code)`;
   - derive helpers, which emit nothing themselves: `default` (the `Default`
     derive's variant marker) and `serde`, with only the keys used at the
     basis: `default` with no value, `deny_unknown_fields`, `flatten`,
     `rename`, `rename_all`, `skip`, `skip_serializing_if`, `tag` and
     `untagged`. Other keys, such as `with`, `serialize_with`,
     `deserialize_with`, `default = "…"`, `from`, `try_from`, `into` and
     `getter`, make the derive call a function named in a string literal,
     which the name screen does not read. `skip_serializing_if` names one too,
     so its value must be one of the two at the basis: `"Option::is_none"`
     (`report_renderer/src/lib.rs:147,258,260`) or `"std::ops::Not::not"`
     (`:302`);
   - one attribute macro, `tauri::command` (67, all in `src-tauri`, none with
     arguments). It emits a wrapper that calls only the function it marks.
     It may mark no function with a scanned name, except the Tauri command
     `render_calculation_report` (`lib.rs:1568-1569`).

   A derive must be one of `Clone`, `Copy`, `Debug`, `Default`, `Eq`,
   `PartialEq`, `Ord` and `PartialOrd` from `std`, or `Serialize` and
   `Deserialize` from `serde`, written bare. Those are all the derives at the
   basis.
4. **Imports cannot rebind a listed name.** A bare name resolves through
   imports, so these are findings too:
   - any `extern crate`, and any glob import (there is none in production at
     the basis);
   - a `use` whose bound name, its last segment or its `as` alias, equals a
     listed bare macro or derive name, the first segment of a listed path
     (`serde`, `serde_json`, `rusqlite`, `tauri`) or a standard crate root
     (`std`, `core`, `alloc`; code review r4 at #1682), unless it imports
     that item from its listed source. At the basis these are
     `use serde_json::{json, …}`, `use rusqlite::{params, …}` and
     `use serde::{Serialize, Deserialize}` or one of the two, and none binds
     a crate root under another name.

*Why the listed macros emit no scanned call and no `mod`.* Each one emits
code made only from its own tokens and its own crate's internals:
- the formatting and assertion macros (`format!`, `format_args!`, `write!`,
  `eprintln!`, `println!`, `assert!`, `debug_assert!`, `debug_assert_ne!`,
  `panic!`, `unreachable!`) expand to `core::fmt` calls over their argument
  expressions. Those reach only `Display` and `Debug` impls. An impl in the
  four crates is a `fn` the scan reads like any other, and no other crate can
  call the renderer, since only these crates depend on it (asserted below);
- `vec!` moves its argument expressions into a vector, and `matches!` is a
  `match` on its expression with its pattern. `env!` and `include_str!`
  expand to a string literal (`include_str!` only bare and of a non-`.rs`
  file, so it brings in no code);
- `json!` builds a `serde_json::Value`, and `params!` a slice of references
  to its argument expressions. Any expression in either is written in the
  invocation's tokens, which the name screen reads;
- `serde::forward_to_deserialize_any!` emits `Deserializer` methods in the
  impl it is written in, each forwarding to `self.deserialize_any`;
- `tauri::generate_handler!` emits a dispatcher that calls each listed
  command through the wrapper that `tauri::command` made for it. Every
  command it calls is named in its tokens, inside `app_invoke_handler` or in
  that macro's `native_smoke::…` arguments, and so is screened;
- `tauri::generate_context!` reads `tauri.conf.json` and the frontend
  assets at build time and emits the app's context. It names no function of
  these crates;
- `app_invoke_handler!` is pinned by its hash (rule 1);
- the derives emit trait impls over the type's fields, which call the
  fields' own impls and, through `serde` keys, only the two
  `skip_serializing_if` paths above.

*What this closes.*
- **S5c.** The only macro that may be defined is `app_invoke_handler`, which
  holds no `mod`, and no listed invocation emits one. So every production
  `mod` is a `syn` item, and the module traversal above sees every
  production file.
- **S5d.** No new macro can be defined, so none can build a path from its
  parameters. The one listed has no `$` parameter before `::`, and its
  `$extra` is a whole path written in the invocation, where the screen reads
  it.
- **N4.** A macro defined outside the four crates may be invoked only if it
  is listed, and no listed one emits a call to a function it is not given.
- **What remains.** The listed external macros are trusted to do what the
  versions in the lock do. `tauri::command`, `tauri::generate_handler!`,
  `tauri::generate_context!` and the `serde` derives are procedural, and
  `json!`, `params!` and `forward_to_deserialize_any!` are declarative. If a
  version of one emitted a call to a scanned function not named in its
  input, the scan would not see it. None of today's does, and changing one
  of those dependencies is reviewed as a change to this boundary.

The module traversal and the name, scope and call rules below therefore rely
on every production `mod` and every production call being visible to `syn`,
apart from what the listed macros emit.

*What the four crates hold today* (at the basis). Each form above was
searched for in the basis source. The scan itself is not yet built, so this
was a search, not a scan run:
- **`macro_rules!`.** One in production: `app_invoke_handler`
  (`src-tauri/src/lib.rs:5018-5081`), the one allow-list entry (macro rule
  1 above). No crate has a `macro` (2.0) item, `#[macro_export]` or
  `#[macro_use]`.
  - Its handler list names the Tauri command `render_calculation_report`
    bare (`:5076`). The `src-tauri` rule below does not match the bare name,
    so the macro holds no matched name.
  - The other two, `probe_scalars` and `probe_unsupported`
    (`command_ingress.rs:1395,1403`), are inside the test-only `mod tests`
    (`:793-1812`).
  - The other three crates define none.
- **`include!`, `include_bytes!` and `concat_idents!`.** None.
- **`include_str!`.** Production uses read JSON and schema files, for example
  `src-tauri/src/lib.rs:168,170`, `report_package/src/schema_eval.rs:47-62`
  and `report_renderer/src/exact_statements.rs:20`. Each argument is one
  string literal, and none names a `.rs` file.
- **`#[path]`.** Four, none in production:
  - three on test-only `mod`s: `analysis_runs.rs:2394`,
    `report_package_save.rs:951` and `rule_solver_selection.rs:490`;
  - one in the test-only file `report_render_exact_tests.rs:14`.
- **`cfg_attr`.** Every occurrence is an `allow(dead_code)` form.
- **`mod` items under a `cfg` other than exactly `#[cfg(test)]`.** Two, both
  in `src-tauri` and both read as production. Neither holds a scanned name:
  - `#[cfg(feature = "native-smoke")] mod native_smoke;` (`lib.rs:18-19`);
  - `#[cfg(target_os = "macos")] mod mac { … }` (`host_memory.rs:109-110`).
- **Files.** No file is named by both a test and a production declaration.
  Every production `mod name;` is in a crate root, and the only production
  inline module outside a root is `mac`. No crate has a subdirectory under
  `src`.

So the rules hold at the basis. The allow-lists are the macro lists above,
each derived from the basis source, and those of the conservative rules
below (code review r3 at #1682): the defining module's macros, derived from
its source, and its type aliases and the reporting crates' raw identifiers,
both empty.

*Scanned names and how they are matched.* The scanned names are:
- the six document functions: `render_calculation_report`,
  `assemble_report_sections`, `emit_pdf_document`, `pdf_from_render`,
  `emit_calculation_report_pdf` and `assemble_full_report_package`;
- the producer's three internal entries: `check_request_in`,
  `assemble_report_package_container_in` and `assemble_wire_request_in`;
- `check_derived`, `assemble_derived`,
  `assemble_user_released_request` and `DerivedUserRelease`.

*Spellings (Codex round 1 at #1682, P2a).* Names and paths are compared
normalized, in calls, literals, types, imports, attributes, macro paths and
token screening alike:
- a raw identifier is compared without its `r#` (`r#assemble_derived` is
  `assemble_derived`), including a module's name when its file is resolved.
  In the three reporting crates a raw identifier is also itself a finding
  (*Conservative rules* below), so normalization matters there only for the
  finding's text;
- a leading `::` is dropped (`::serde_json::json!` is `serde_json::json!`);
- a path prefix of only `crate`, `self` and `super` is the current crate's
  own modules.

Every other spelling fails closed:
- `use … as` aliases: an import that names a scanned function, any import
  in `src-tauri` of the renderer or PDF crate, and an import that binds
  `ProducerEntry::UserRelease`, are findings. In `report_package` and
  `pdf_emitter` the renderer and PDF crates are bound only under the basis
  aliases (`renderer`, `pdf`); another alias is a finding, so (a) below sees
  every re-export;
- `<T as Trait>::name` and `<T>::name` paths to a scanned name are
  findings, never counted as calls;
- method-call syntax, function pointers and closures by a scanned name are
  occurrences other than a direct call (rule 2 below), so they are findings
  except a closure's direct call, which is a call from the closure;
- `Self::name`: in `src-tauri` a renderer-pair name through any prefix other
  than `crate`, `self` or `super` (`Self`, an alias, a re-export path) is
  scanned;
- `DerivedUserRelease`: a `Self { .. }` literal in an `impl` of it is its
  literal; naming it in a type alias or an associated type is a finding; its
  definition carries no attribute but its documentation (no derive); and the
  three reporting crates hold no `unsafe` code and no `extern` block,
  through which one could be built without naming it. The compiler enforces
  that (*Unsafe code and macro names* below);
- `DerivedUserRelease` under another name (Codex round 2 at #1682, P2): an
  associated type assigned `Self` in an impl for it names it under another
  path, through which a struct literal compiles (an alias of
  `<T as Identity>::Repr`, written `Alias::<DerivedUserRelease> { .. }`). So
  any impl of a trait for a type that names it is a finding, except an
  explicit allow-list of the impls that exist today, which is empty; any
  associated type in an impl for it is a finding; and in its defining module
  (`report_package/src/user_release.rs` and any module beneath it, the only
  code that can name its private field, so the only code that can write its
  literal) a struct literal is admitted only when its path is one plain
  identifier naming a struct the `report_package` crate defines that no type
  alias, associated type or import rename in the crate shares, or `Self` in
  an impl of such a name. A `<T as Trait>::` path, generic arguments, a path
  of more than one segment (`Self::Assoc`, `module::Alias`, an enum variant)
  and a leading `::` are findings, as is `Self { .. }` in a trait's default
  method. The rule reaches every literal in the module because every
  literal there is in code the scan parses: outside macros, or in a listed
  macro's tokens, which are parsed and visited as code (*Conservative rules*
  below); any other macro there, and a brace group in any other tokens
  there, is a finding. The rule then holds because a literal's path
  resolves to `DerivedUserRelease` only through its own name (the literal
  rule), an alias (a type alias, an associated type, an import rename, or a
  projection or defaulted parameter reached through one), `Self` in an impl
  of it, or generic arguments or a qualified path at the literal. Each of
  these is a finding: a type alias or associated type declared in the
  module, in any position, is one outright, and any other alias, import
  rename or associated type the crate declares shares its name with no
  admitted struct. The real sources' literals in the module are the crate's
  own structs, so the real scan still finds nothing;
- `ProducerEntry::UserRelease` named as a value, not only as a callee, and
  its braced form (`ProducerEntry::UserRelease { 0: derived }`), are
  constructions;
- macro-generated paths: only the listed macros may be invoked. A
  `matches!` anywhere, and in the defining module each macro it admits, is
  parsed by its input grammar and visited as code (*Conservative rules*
  below). So a `UserRelease` in a `matches!` pattern is a pattern and
  constructs nothing, while its scrutinee, its guard (`if let` included)
  and any expression inside its pattern (an inline `const` block, a const
  generic argument, an array length in a qualified path, a macro) are
  visited as expressions, constructions counted (Codex round 2 and code
  review r3 at #1682: a construction in a `matches!` guard, and one in a
  const generic argument of its pattern, escaped). A `matches!` whose tokens
  do not parse so is a finding. Every other macro's tokens are screened
  with the same normalization, and a `UserRelease` named in them is a
  construction: `include_str!` is also checked as one string literal,
  `include!`, `include_bytes!` and `concat_idents!` are refused by any
  path, item keywords and `macro_rules!` there are findings (*Unsafe code
  and macro names* below), and `macro_rules!` bodies are screened whole,
  none discarding a construction.

*Conservative rules (code review r3 at #1682).* Three review rounds each
found a form that the scanner modelled wrongly: an alias declared inside a
macro's tokens, a raw identifier (`r#const`) taken for a block keyword, and
a construction inside a const generic argument of a `matches!` pattern.
Rather than model each form, the scan refuses whole classes, failing closed
on what it does not parse into its model. Each rule below is a finding when
broken, and each allow-list is empty or derived from today's source:
- **Type aliases in the defining module.** Every type alias declared in
  `report_package/src/user_release.rs` or a module beneath it is a finding,
  whatever it names: an item, a block's item, an impl's associated type and
  a trait's associated type, inside a listed macro's tokens included. The
  allow-list (`SUBTREE_TYPE_ALIASES`) is empty; none exists today.
- **Macros in the defining module.** Only `format!`, `format_args!`,
  `vec!`, `matches!` and `json!` may be invoked there (`SUBTREE_MACROS`,
  exactly the ones its production code uses today).
  Each one's tokens are parsed by its input grammar into `syn`'s tree and
  visited by the same visitor as code outside macros, so every rule, the
  alias, literal and construction rules included, applies inside them:
  `format!` and `format_args!` as comma-separated expressions (a named
  argument is an assignment expression); `vec!` as `expr; expr` or
  comma-separated expressions; `matches!` as `expr, pattern (if expr)? ,?`;
  and `json!` as a lone `{ … }` object of `key: value` entries (the key an
  expression), a lone `[ … ]` array, or one expression, recursively. Every
  token is in a parsed expression or pattern or is the grammar's own
  punctuation. Tokens that do not parse so are a finding. Any other macro
  there (`println!`, `assert!`, an item-level macro) is a finding, and a
  brace group in tokens the scan does not parse there (an attribute's, or a
  refused macro's) is a finding. The scan chose to refuse every other
  macro, rather than screen for `type`, `Path { … }` or `ProducerEntry` in
  their tokens, because that screen would itself be a token-level model of
  Rust's grammar, the kind of rule each round found wanting; a parsed tree
  needs no such model.
- **Raw identifiers in the three reporting crates.** Every raw identifier
  (`r#name`) in a file the scan reads in `report_package`, `report_renderer`
  or `pdf_emitter`, in any position (code, attributes, macro tokens and
  inline test modules), is a finding. The rule runs on each file's tokens,
  as `proc_macro2` lexes it, so no position escapes it; a file that does not
  lex is a finding. The allow-list (`RAW_IDENTIFIERS`) is empty: the three
  crates hold no raw identifier today (raw string literals such as
  `r#"…"#` are literals, not identifiers). In `src-tauri` raw identifiers
  are compared normalized, as above.
- **Blocks in generic arguments in the three reporting crates.** On the
  same tokens, a brace group directly after `<`, or after `,` or `=` while
  an angle bracket is open, is a finding: `Type<{ … }>`,
  `path::<A, { … }>`, a const parameter's `= { … }` default, in types,
  expressions, patterns, macro tokens and attributes alike. The angle count
  is lexical: `<` opens unless it is half of `<<` or `<=`, `>` closes unless
  it ends `->` or `=>` or starts `>=`, and `;` and each brace group reset it.
  It can therefore also find a comparison followed by a block argument
  (`f(a < b, { … })`), which fails closed; today's source holds none. In the
  parsed tree, a const generic argument other than a literal, and an
  associated-constant argument, are findings too.
- **Syntax `syn` does not model.** Anywhere in the four crates' production
  source, an item,
  impl item, trait item, foreign item, expression, type, pattern or bound
  that `syn` gives as `Verbatim` is a finding, never skipped.

*Unsafe code and macro names (code review r4 at #1682).* Two forms compiled
and gave no finding. Outside the defining module a macro's tokens were only
screened, and the screen never checked `unsafe`, so
`vec![unsafe { std::mem::zeroed() }].pop().unwrap()` in
`report_package/src/lib.rs` built a `DerivedUserRelease`, and
`transmute::<u8, user_release::ProducerEntry>(1)` built
`ProducerEntry::UserRelease`. And a listed path could resolve to another
macro: `mod tauri { pub use std::include as generate_context; }` made
`tauri::generate_context!("review_escape.rs")` an `include!` of a file the
scan never read. The repairs are structural rather than more token
modelling:
- **`#![forbid(unsafe_code)]`, enforced by the compiler.** The crate roots
  of `report_package`, `report_renderer` and `pdf_emitter` carry it. The
  compiler then refuses every `unsafe` block, `unsafe fn`, `unsafe impl` and
  `unsafe trait` the crate's own source writes, including inside macro
  arguments and in included files (both repros, and `format!`, `assert!`
  and `vec!` forms, fail to compile), and no inner `#[allow(unsafe_code)]`
  can lift it (E0453). None of the three crates had `unsafe` code, so each
  builds unchanged. The scan requires that exact inner attribute at each of
  the three roots: its absence, or any other form (another lint added,
  `deny`, through `cfg_attr`, or as an outer attribute), is a finding, and a
  `forbid` anywhere else is not on the attribute list. The token rule also
  finds every `unsafe` keyword in the three crates' read files, in any
  position, and the parsed-tree `unsafe` findings stay.
  Without `unsafe`, nothing in these crates' dependencies can build either
  private type. `DerivedUserRelease` has a private field, so only a literal
  in its own module builds one (the literal rule), and a safe generic
  constructor in any crate (`Default`, `Clone`, `Deserialize`) needs a trait
  impl for it, which is refused (no derive; the trait-impl allow-list is
  empty). `ProducerEntry::UserRelease` holds one, so it cannot be built
  without one.
- **Macro names.** So that a listed invocation path resolves only to the
  listed macro, each of these is a finding:
  - (a) in the three reporting crates, a `use … as …` rename (`as _`
    included) other than today's seven, each a crate alias:
    `open_pipe_stress_audit_manifest as audit`,
    `open_pipe_stress_pdf_emitter as pdf`,
    `open_pipe_stress_report_renderer as renderer`,
    `open_pipe_stress_protected_content_linter as linter`,
    `open_pipe_stress_report_generator as report`,
    `open_pipe_stress_report_sections as sections` and `crate as package`
    (`USE_RENAMES`). Whether a renamed path is a macro cannot be decided from
    source, so every rename is listed rather than macro renames alone;
  - (b) in the three reporting crates, a non-private `use` (`pub`,
    `pub(crate)`, any restricted visibility) other than today's five, all in
    `report_package/src/lib.rs` and none a macro:
    `open_pipe_stress_report_renderer::serde_error_kind` and
    `reader::{read_report_package_container, PackageReadRefusal,
    ReadReportPackage, SUPPORTED_PACKAGE_VERSION}` (`PUBLIC_USES`). The
    same reason applies;
  - (c) in all four crates, a `mod`, `use` binding, `extern crate` (already
    a finding itself) or `macro_rules!` whose name is reserved: a standard
    crate root (`std`, `core`, `alloc`), a listed external crate (`serde`,
    `serde_json`, `rusqlite`, `tauri`), or the first segment of a listed
    invocation, which for a bare listed macro is its own name (`format`,
    `vec`, `json`, `params` and the rest). A `use` that imports a listed
    name from its listed source, and the pinned `app_invoke_handler`, are
    the exceptions. A local module or binding of such a name is what lets a
    listed path name another macro;
  - (d) in all four crates, an invocation of `include!`, `include_bytes!` or
    `concat_idents!` by any path, judged by its final segment, and an import
    of any of them or of `include_str!`. `include_str!` is admitted only
    bare, with one string literal that names no `.rs` file: production uses
    it for JSON and schema text (`schema_eval.rs:47-62`,
    `exact_statements.rs:20`, `src-tauri/src/lib.rs`). No production code
    uses `include_bytes!`. `env!` and `option_env!` expand only to a string
    literal, so they need no rule beyond the invocation list (`env!` is
    listed; `option_env!` is not). Only one `macro_rules!` exists, pinned;
  - in all four crates, the keywords `mod`, `use`, `extern` and `macro`, and
    `macro_rules!`, in tokens the scan does not parse (a macro's that is not
    parsed, an attribute's). An item there would bind a name the scan never
    sees, so the shadow above written inside `format!("{}", { … })` is a
    finding too;
  - (e) `#[macro_use]` on an `extern crate` or a `mod`, outlined or inline,
    or anywhere else, as before.

  None of these rules hit the real sources: today's renames and re-exports
  are exactly the allow-lists, and no module, binding or macro definition
  takes a reserved name.

The real-source scan finds nothing under these rules; no rule needed an
allow-list entry beyond today's renames and re-exports or a change to
production code other than the three crate-root attributes. What the scan does not
attempt to model, and refuses instead: macro expansion (beyond the listed
macros' input grammars), name resolution beyond the crate-wide inventory of
struct and alias names, type inference, constant evaluation and `cfg`
predicates.

An occurrence is an identifier equal to a scanned name, wherever it stands:
- in a path, a `use` tree, a method call, a field, a pattern or a type;
- in an attribute's arguments;
- in the tokens of any macro definition or invocation. `syn` leaves macro
  bodies unparsed, so the scan walks every token stream, groups included,
  except where it parses a listed macro's tokens and visits them as code.
  A `Verbatim` node is a finding (*Conservative rules* above).

In `src-tauri` the bare `render_calculation_report` is the Tauri command's:
its definition (`lib.rs:1569`) and its handler entry (`:5076`). Its command
string (`:1573`) is a literal and is never matched. So in `src-tauri` the
renderer's two functions are matched only as a path with a prefix other than
`crate`, `self` or `super` (such as `open_pipe_stress_report_renderer::<name>`
or `::open_pipe_stress_report_renderer::<name>`), or that token sequence in a
macro. Rule (b) below keeps any other path from naming the renderer there.
Every other scanned name is matched bare in all four crates.

*Lexical scope.* Each occurrence is attributed to its innermost enclosing
scope owner in the tree. A scope owner is one of these:
- a `fn` item, or a method in an `impl` or trait;
- a closure, `async` block, `const` or `static`;
- a macro's token stream;
- the module itself, for an occurrence outside every function.

Scope follows the tree, not indentation. A `macro_rules!` placed after a
function, a closure however it is laid out, and a nested `fn` each own their
contents. The allowed callers in the table are named `fn` items or methods,
identified by name and file. A call belongs to an allowed caller only when
that caller is the call's innermost scope owner.

*Scanned functions and their allowed callers.* Each allowed caller calls the
function exactly once. "None" means no call anywhere in the four crates'
production source. Lines are at the basis.

| Function (crate) | Allowed callers | Path the call must go through |
|---|---|---|
| `render_calculation_report` (renderer, `report_renderer/src/lib.rs:2002`) | `emit_calculation_report_pdf` and `assemble_full_report_package` (pdf_emitter, `pdf_emitter/src/lib.rs:130,197`); `render_calculation_report_value` (`src-tauri/src/lib.rs:1601`) | pdf_emitter's two callers: the rows below. `render_calculation_report_value`: the render-command refusal, before it decodes (below) |
| `assemble_report_sections` (renderer, `:730`) | `render_calculation_report` (renderer, `:2050`); `emit_pdf_document` (pdf_emitter, `:237`) | inside the render, behind its own gates; `emit_pdf_document`: the rows below |
| `emit_pdf_document` (pdf_emitter, private, `:233`) | `pdf_from_render` (`:155`) | the row below |
| `pdf_from_render` (pdf_emitter, `:138`) | `emit_calculation_report_pdf` (`:131`) and `assemble_full_report_package` (`:198`), each passing its own `input` and the render it has just made from that input | the rows below |
| `emit_calculation_report_pdf` (pdf_emitter, `:129`) | none | none: it is unreachable from product code in these crates |
| `assemble_full_report_package` (pdf_emitter, `:194`) | `assemble_report_package_container_in`, the writer (report_package, `lib.rs:236`) | the writer's callers: the row below |
| `assemble_report_package_container_in` (report_package, `lib.rs:213`) | `assemble_report_package_container` (`lib.rs:207`), passing `ProducerEntry::General`, and `wire::assemble_wire_request_in` (`wire.rs:407`), forwarding its own entry | refusal 2 (before the writer renders), or the row below |
| `check_request_in` (report_package, `bindings.rs:355`) | `check_request` (`bindings.rs:350`), passing `ProducerEntry::General`, and `wire::assemble_wire_request_in` (`wire.rs:383`), forwarding its own entry | refusal 2 |
| `wire::assemble_wire_request_in` (report_package, `wire.rs:366`) | `wire::assemble_wire_request` (`wire.rs:360`), passing `ProducerEntry::General`, and `user_release::assemble_derived`, passing `ProducerEntry::UserRelease(derived)` | refusal 2 (in `check_request_in`), or the admitting entry |
| `user_release::check_derived` (report_package) | `user_release::assemble_derived` | the preflight, then the derivation |
| `user_release::assemble_derived` (report_package) | `report_package_bridge::assemble_user_released_request` (src-tauri) | the row below |
| `report_package_bridge::assemble_user_released_request` (src-tauri) | the `report_package_save.rs` function that holds route 010's branch | native steps 1-8 |

*The rule.* In the four crates' production source, each occurrence of a
scanned name must be one of these:
1. its definition: the `fn` item's (or the struct's) own name, once, in the
   file the table gives;
2. the callee of a direct call: a call expression whose function is a path
   ending in the name, whose innermost scope owner is an allowed caller of
   that row. For the `src-tauri` renderer pair the path's prefix is
   `open_pipe_stress_report_renderer` (with or without a leading `::`);
3. for `DerivedUserRelease` only, a type position (a signature, a field, the
   entry variant's payload), or the struct literal in `check_derived`
   (rule (c)).

Everything else is a finding. That one rule rejects:
- a `use`, `pub use` or `use … as` that names the function;
- a function value in any position:
  - `let f = …` or `.map(…)`;
  - a `fn`-pointer constant or struct field;
  - a `Box<dyn Fn …>` trait object;
  - an argument to a generic `F: Fn` or `impl Fn` parameter;
  - an `impl Fn` return value;
- a call whose innermost scope owner is anything else: another function, a
  method on a wrapper type, a trait impl, a nested `fn`, or a closure or
  `async` block, including one inside an allowed caller;
- the name in any macro, defined or invoked. An allowed caller that reaches
  the function only through a macro has no direct call, which is a finding
  too.

The scan also reads the entry argument (the one before `admitted`) at every
call of `check_request_in`, `wire::assemble_wire_request_in` and
`assemble_report_package_container_in`:
- at the three public roots (`check_request`, `assemble_wire_request` and
  `assemble_report_package_container`) it must be the path
  `ProducerEntry::General`;
- at the edges inside `assemble_wire_request_in` it must be the forwarded
  `entry` parameter;
- only in `assemble_derived` may it be `ProducerEntry::UserRelease(derived)`.

The variant `UserRelease` may be constructed, that is appear in expression
position, only there. In a pattern it may appear anywhere, since a pattern
constructs nothing; an expression inside a pattern is read as an
expression.

*Re-exports and imports.* These are findings too:
- **(a) A re-export by module, alias or glob.** In `report_package` and
  `pdf_emitter`, no `use` item with any visibility other than private, and no
  `pub extern crate`, may name any of these:
  - the renderer or pdf_emitter crate;
  - its local alias (`renderer`, `pdf`);
  - a glob of either.
  Examples are `pub use pdf::*;` and
  `pub use open_pipe_stress_pdf_emitter as pdf;`. The one exception is the
  re-export at the basis, `pub use open_pipe_stress_report_renderer::serde_error_kind;`
  (`report_package/src/lib.rs:64`), which names no scanned function. So no
  dependent of `report_package`, the runner included, can name a renderer or
  PDF function through it.
- **(b) In `src-tauri`'s production source, imports.** No `use` item or
  `extern crate` may name the renderer crate, the pdf_emitter crate or a
  scanned function, and none may glob-import `open_pipe_stress_report_package`
  or its alias. At the basis there is none in production. The renderer
  imports at `object_form_tests.rs:23-24` and
  `report_render_exact_tests.rs:23-24,1420` are in test-only files. The only
  crate alias is `use open_pipe_stress_report_package as package;`
  (`report_package_bridge.rs:2`), and (a) leaves that path with no renderer
  or PDF function to name.
- **(c) A `DerivedUserRelease` struct literal** anywhere except in
  `check_derived`.

A crate alias such as `use open_pipe_stress_pdf_emitter as pdf;`
(`report_package/src/lib.rs:48`) or `use open_pipe_stress_report_renderer as
renderer;` (`pdf_emitter/src/lib.rs:29`) is allowed. The rule matches the
function's own name, whatever path comes before it.

*Also asserted:*
- no source under `core/runner` calls `assemble_derived` or the bridge
  function;
- the project's Cargo manifests that depend on
  `open_pipe_stress_report_renderer` are exactly `src-tauri`'s, `pdf_emitter`'s
  and `report_package`'s, and only `report_package`'s depends on
  `open_pipe_stress_pdf_emitter`.

A new caller, re-export, dependent, module path, included file, macro
definition, macro invocation, attribute or derive outside the lists, or a
changed `app_invoke_handler`, fails the scan, so changing one is reviewed as a
change to this boundary.

*What the scan does not establish.*
- **Code that does not call the functions.** The scan pins the calls of the
  functions named above. It does not judge new code that would build a
  document without calling them: for example, a renderer function that
  writes HTML from the renderer's per-line helpers, or `report_package` code
  that writes the input itself. Such code never reaches the renderer's
  user-released class rule. It would bypass the render's gates in every
  context, not this design's alone, and it is reviewed as a change to the
  renderer or the producer.
- **Macro expansion.** The scan reads source as written and does not expand
  macros. Every macro's tokens are screened, so no macro can carry a scanned
  name that is written in it. Beyond that, only the listed macros may be
  defined or invoked (*Macros* above), so no local macro and no unlisted
  external macro, declarative or procedural, can emit a `mod` or a call. What
  remains is a listed external macro whose version emitted a call to a
  scanned function not named in its input, for example by building the
  identifier from parts. The scan would not see it. None of today's does
  (Codex round 4 at revision 4, N4), and a change to one of those
  dependencies is reviewed as a change to this boundary.
- **`unsafe` in an external macro's own expansion.** The compiler does not
  lint code that an external macro's definition writes (as opposed to
  tokens the crate passes it), so `forbid(unsafe_code)` covers the crates'
  own source, macro arguments and included files, not an `unsafe` emitted
  by a listed external macro itself. The listed ones take no type to
  construct, and a change to one is reviewed as a change to this boundary.
- **Manifests and dependency names.** A listed external crate root is
  trusted to be that crate. A `Cargo.toml` rename of a dependency to a
  listed root's name (`tauri = { package = … }`) is not judged by the scan;
  it is reviewed as a manifest change.
- **W-11's scan.** W-11's forwarding-graph scan keeps its line rules for the
  admitted set, which is W-11's own claim. This design relies on that scan
  for no reach claim, since the reach scan pins the producer's internal
  entries itself (the table). Hardening W-11's scan the same way is a
  remaining item for W-11's design. It is taken up after this unit.

**Every path to the renderer's user-released class rule, at main plus this
design.** The renderer applies that rule (§2.6) to any `user_released_report`
input with a record, so every product entry that reaches the renderer was
checked:

| Path | Closed by |
|---|---|
| The runner's `export-results` | refusal 1, before the producer |
| Route 009's save (`report_package_bridge::assemble_request`) | native step 3 (`REPORT-USER-RELEASE-NOT-THIS-ROUTE`), then refusal 2 |
| The producer's public entries called from any other crate | refusal 2 |
| Renderer and PDF calls inside `report_package` and `pdf_emitter`: `pdf_emitter`'s public functions (`emit_calculation_report_pdf`, `pdf_from_render`, `assemble_full_report_package`, `pdf_emitter/src/lib.rs:129,138,194`) and the renderer's `render_calculation_report` and `assemble_report_sections` | the reach scan's rows. Only `report_package` depends on `pdf_emitter`. Inside `pdf_emitter` the renderer is called only from `emit_calculation_report_pdf` and `assemble_full_report_package`, the first has no caller, and the second is called only by `report_package`'s writer. The writer is reached only from `assemble_report_package_container` (refusal 2) and `wire::assemble_wire_request_in`, which forwards refusal 2 or the admitting entry. The scan reads every production file of the four crates as parsed Rust and attributes each name to its lexical scope. In that source each scanned name occurs only at its definition and as a direct call by an allowed caller that is the call's innermost scope owner. No `use`, re-export, function value, trait object, generic argument, closure, nested item or macro names one, and only the listed macros may be defined or invoked. This covers the calls of the functions named, as written. New code that builds a document without them, and a listed external macro whose version would make a call without naming it (none of today's does), are outside the scan (above) |
| The render command, pressure-free branch (`render_calculation_report_value`) | the render-command refusal below |
| The render command, exact branch | `check_exact_render_context` (`report_render_exact.rs:139-147`) admits `local_private` only; a record under `local_private` is refused by the render-command refusal below |
| Route 010 | the admitting entry, after native steps 1-8 |

The webview has no renderer of its own; it renders through the render
command. No Python code reaches the Rust renderer.

**Writer.**
- `ReportPackageContainerInput` (`lib.rs:130-150`) and `ManifestFields`
  (`lib.rs:667-674`) gain `user_release: Option<&Value>`.
- With a record, the writer requires zero state/comparison/handoff records and
  blocks any (`lib.rs:279-283` blocks none today). Without a record it requires
  at least one, as today.
- `package_manifest_document` (`lib.rs:676-758`) writes `"user_release"` only
  when present: the caller's record plus `content_binding` over the
  materialized members (`lib.rs:328-359`). It then writes the user-released
  boundary notes (§5.3).

**Render command.** `render_calculation_report_value`
(`src-tauri/src/lib.rs:1594-1613`) refuses, before it decodes or renders, an
input whose `report_context.export_context` is `user_released_report`, and,
independently, an input whose `report_context` has a `user_release` member,
whatever its context. The code is `RENDER-USER-RELEASE-PACKAGE-ONLY`. Since
W-11 the command reaches it through
`report_render_exact::render_calculation_report_args_in`
(`report_render_exact.rs:94-117`), after the object-form check and step 0. An
exact input that names `user_released_report` is refused earlier by
`check_exact_render_context` (`:139-147`). An exact `local_private` input with
a record reaches `render_calculation_report_value` and is refused there. A
user-released document is therefore produced only inside route 010's
admission, through the producer's one admitting entry. The webview already
refuses that context on this route (`controlledContextConsistent`,
`reportRenderService.ts:135-151`).

### 2.9 How the reader verifies it (DEL-02-05), and what that establishes

`read_report_package_container` (`reader.rs:163-275`):
- `manifest_fields` (`:695-733`) reads an optional `user_release` and passes
  it to the rebuild (`:237-252`), so a record outside the defined form fails the
  existing manifest-form check.
- The unblocked-package member rule (`:976-983`) requires at least one
  state/comparison/handoff member only when no record is present. Its result
  member requirement (since W-11) is unchanged.
- After `verify_members` (`:819-985`), when a record is present, each failure
  is `OPSPROJ_USER_RELEASE_UNBOUND` with its reason:
  - the package is unblocked, and `export_profile_id` is
    `desktop_user_released_report_package_1`;
  - **cardinality:** exactly one HTML, one PDF, one audit and one result
    member, and no state/comparison/handoff member;
  - **pressure-free only (§0):** the result member's producer identity is
    `preview-physics-2`. The reader's item 0 admits the exact identities in
    every package since W-11 (`reader.rs:941-946`), so this check sits beside
    its `check_runner_profile` call (`:946`), in the same form;
  - the record's constants and forms hold;
  - `model_ref` = the manifest's `source_model_ref.ref_id` = the result
    member's `model_ref.ref_id` = the audit member's `model_hash.payload_ref`;
  - `model_document_sha256` = the audit member's `model_hash.value`;
  - `input_manifest_sha256` and `input_manifest_ref` = the audit member's
    `input_manifest_hash.value` and `payload_ref`;
  - the result member's `envelope_id`, derivative hash value and
    `run_ref.ref_id` = `result_envelope_id`,
    `result_document_derivative_sha256` and `analysis_run_ref`;
  - `scope.package_id` = the manifest's `package_id`, `scope.report_id` =
    `report:<analysis_run_ref>`, and `analysis_run_ref` =
    `source_basis_refs[0].ref_id`;
  - `content_binding.members` lists exactly the non-manifest members, by file
    name and role in archive order, and each `sha256` equals the SHA-256 of
    that member's bytes (the inventory hash already equals it);
  - the HTML member contains the HTML-escaped text of
    `user_release_notice_text(record)` exactly once, regenerated by the
    renderer's own function from the manifest record. This ties the rendered
    notice to the manifest copy. The PDF is tied by its byte hash only.
- A user-released profile without a record, or a record with any other
  profile, is refused.
- Refusal: a new variant `PackageReadRefusal::UserReleaseUnbound { reason }`,
  code `OPSPROJ_USER_RELEASE_UNBOUND` (`reader.rs:92-123`).
- `ReadReportPackage` (`:66-87`) gains `user_release: Option<Value>`, so a
  consumer can show the record.

The audit and result identities are collected in `verify_members` as the audit
identity already is (`:931-934`).

**What these checks establish, and what they do not (O-G).** The reader's
checks are equality checks, not signatures (`reader.rs:30-41`). They establish
consistency: the record, the members and the member bytes agree with each
other. At save time, native admission also establishes that:
- the record came through route 010 with the wrapper's user-release intent;
- the model and result were this session's solve;
- every member was derived from them.

Neither establishes authentic user consent (that a person made the statement)
or producer origin (that this application wrote the package), and neither
establishes that the statement is true. Anyone who can write the file can build
a coherent package with a record, its members and every hash re-computed
together, and the reader will not detect it. The reader's module text
(`reader.rs:1-42`) says this in these terms, and so does DEL-02-05 (A8).

### 2.10 No replay

The claim is narrow: **an unchanged record** cannot be carried onto another
model, result set, report, package or member content. A record that is
re-written to match its new place is not detected (§2.9).

| Attempt | Refused by | Code |
|---|---|---|
| Record copied into a request for another model | native admission (model text SHA-256, `model_ref`), then the producer (audit and document model hash and reference) | `REPORT-USER-RELEASE-UNBOUND`; `REPORT-PACKAGE-USER-RELEASE-UNBOUND` |
| Same model, another solve or result document | native admission (run, envelope id, derivative hash, attestation); the producer | as above; `REPORT-USER-RELEASE-RESULT-NOT-ATTESTED` |
| Same result, another input manifest or run record | native admission (input-manifest text SHA-256 and reference; the run-record checks) | `REPORT-USER-RELEASE-UNBOUND`; `REPORT-USER-RELEASE-ANALYSIS-RECORD-UNBOUND` |
| Record with only `model_ref` or only `scope.report_id` changed | native admission; the producer; the reader | as above; `OPSPROJ_USER_RELEASE_UNBOUND` |
| Record placed on the local-private route 009 | native admission; the producer (local-private profile) | `REPORT-USER-RELEASE-NOT-THIS-ROUTE`; `REPORT-PACKAGE-USER-RELEASE-NOT-ADMITTED` |
| Manifest record copied into another package, re-hashed | the reader (bound identities, `content_binding`) | `OPSPROJ_USER_RELEASE_UNBOUND` |
| Same package, HTML, PDF or audit member swapped and the inventory restamped, record unchanged | the reader (`content_binding`; the notice text in the HTML) | `OPSPROJ_USER_RELEASE_UNBOUND` |
| A second result member, or a state/comparison/handoff member, added, inventory restamped | the reader (cardinality) | `OPSPROJ_USER_RELEASE_UNBOUND` |
| Record placed in a standalone render | webview route check; the render command; the renderer (record outside its context) | `REPORT-REDACTION-CONTEXT-MISMATCH`; `RENDER-USER-RELEASE-PACKAGE-ONLY`; `REPORT-CONTEXT-USER-RELEASE-MISMATCH` |
| A coherent user-released request, or any one user-release marker, submitted to the headless runner or to the producer's general entries | the runner boundary; the producer's general entries (§2.8) | `REPORT-USER-RELEASE-RUNNER-NOT-ADMITTED`; `REPORT-PACKAGE-USER-RELEASE-NOT-ADMITTED` |
| A later save reusing an earlier selection | nothing to reuse: the selection is not stored (§2.3) | none needed |
| A record whose statement, standing or `not_carried` text is altered | native form check; the reader | `REPORT-USER-RELEASE-RECORD-INVALID`; `OPSPROJ_USER_RELEASE_UNBOUND` |
| Record, members, `content_binding` and inventory all re-written together | not detected (equality checks, §2.9) | none |

## 3. The classification and routes

### 3.1 DEL-12-02 runtime classification

**Name:** a new export context, `user_released_report`.

**Meaning (proposed):** a report of the user's own session for which the user
has recorded a user release of their own project data, for that one report,
by an explicit selection with a statement that they hold the redistribution
right
(`docs/IP_AND_DATA_BOUNDARY.md` §6). In it:
- the user's private project data (`private_project_data`) is included, with a
  `USER_RELEASED_PROJECT_DATA` warning finding, only with the wrapper's user-release
  intent;
- every other private class is redacted as in `public_report`, so a lossless
  route refuses;
- `protected_suspected`, `quarantined` and `rejected` content blocks;
- unknown or missing metadata blocks (`USER_RELEASE_VALUE_UNBOUND`); unlike
  `local_private`, it is never warning-only;
- the context is admitted only on route `DREP-PACKAGE-USER-RELEASE-010`, with
  the wrapper's user-release intent. Payload content never selects or grants it.

It is an export context, not a privacy class, on purpose. A privacy class such
as `user_released_project_data` would be carried in the payload, and any
context that included it would let payload content grant exposure. The existing
design gives that authority only to the wrapper
(`docs/security/redaction_export_controls.md:158-163`).

**Vocabulary additions** (DEL-12-02 CLM-003, the schema and the three mirrors):
- export context `user_released_report`;
- reason codes `USER_RELEASED_PROJECT_DATA` (finding class
  `PRIVATE_DATA_WARNING`, severity WARNING), `USER_RELEASE_INTENT_REQUIRED`
  (`PRIVATE_DATA_WARNING`, BLOCKING), `USER_RELEASE_VALUE_UNBOUND`
  (`PROVENANCE_WARNING`, BLOCKING), and the local-first reasons
  `USER_RELEASE_METADATA_ALLOWED` and `USER_RELEASE_ROUTE_REQUIRED`;
- route id `DREP-PACKAGE-USER-RELEASE-010`.

### 3.2 The decision rule, in exact order (Codex round 1, finding 2)

`actionFor` (`redactionExportControls.ts:417-479`) and `_action_for`
(`controls.py:372-426`), and the Rust mirror, evaluate:
1. professional claim → `block_export` / `PROFESSIONAL_BOUNDARY_BLOCKED`;
2. cloud or network reference → `block_export`;
3. direct SQL access → `block_export`;
4. storage bypass → `block_export`;
5. secret material → `block_export`;
6. contained payload → `block_export` / `PAYLOAD_METADATA_ONLY_REQUIRED`;
7. concrete path → `redact_value` / `CONCRETE_PATH_REDACTED`, decided last:
   since the DEL-12-02 input contract (HELP_HUMAN, 2026-10-27) it applies only
   when no other step blocks, so it never turns a block of steps 8-13 or of
   the existing contexts' sequence into a redaction
   (`docs/security/redaction_export_controls.md`, Input Contract);
8. privacy or redistribution `protected_suspected` → `block_export` /
   `PROTECTED_CONTENT_BLOCKED`;
9. review status `quarantined` or `rejected` → `block_export` /
   `PROTECTED_CONTENT_BLOCKED`.

Steps 1-9 are the same for every context. Then, **only when the
context is `user_released_report`**, steps 10-13 decide and return, before any
unknown-metadata branch:

10. privacy in the non-project private set (§2.7), whatever the redistribution
    status (`unknown` included) → `redact_value` / `PRIVATE_DATA_REDACTED`. On
    route 010 the user-release projector is lossless, so this withholds the
    payload and the save is refused;
11. privacy `private_project_data`, whatever the redistribution status: with
    the wrapper's user-release intent → `warning_only` /
    `USER_RELEASED_PROJECT_DATA`; without it → `block_export` /
    `USER_RELEASE_INTENT_REQUIRED`;
12. privacy in `{public, public_metadata, invented_public_example}` and
    redistribution in `{public_permissive, invented_non_engineering_example}`
    → `include` / `SAFE_PUBLIC_METADATA` (or `SAFE_INVENTED_PUBLIC_EXAMPLE`);
13. anything else (missing metadata; unknown, `TBD` or null privacy or
    redistribution; `private_only` with a non-private privacy; any
    unrecognized value) → `block_export` / `USER_RELEASE_VALUE_UNBOUND`.

For the five existing contexts, step 10 onward is today's sequence, unchanged:
missing metadata, unknown redistribution, unknown privacy, the safe include,
the private branch and the fallback (`:445-478`; `controls.py:406-426`).
`unknownAction` and `_unknown_action` keep their bodies; the new context never
reaches them. Payload-carried local-private intent (`itemIntent`, `:394-411`)
is not read in the new context.

**The user-release projector** (`controlUserReleasedReportPackageRequest(payload,
model, explicitUserReleaseIntent)`, route 010, lossless) labels each leaf with
the privacy and redistribution of its nearest enclosing object that states them,
else `private_project_data` / `private_only`. So:
- a leaf of the derived request is decided by step 11: a warning with the
  wrapper's intent, a block without it;
- an object labelled with a non-project private class is withheld by step 10;
- a protected, quarantined or rejected object blocks (steps 8-9);
- an object with unknown own metadata blocks (step 13). An explicitly stated
  own privacy or redistribution (its own member, its provenance's, or its
  `privacy_class`, as the projector reads them) that the classification does not
  recognize, such as `{}`, `[]`, a number, a boolean, an empty or unknown
  string, or a null, `unknown` or `TBD` privacy, blocks at that object by step
  13 (`USER_RELEASE_VALUE_UNBOUND`), and no descendant's label replaces it;
  only an absent label inherits (Codex r4 of the r1 slice).

There is no path-token labelling and no field table on this route. The
webview's check is early; native admission's derivation decides (§2.6).

**The redaction walk refuses the context (Codex r3 of the r1 slice).**
`redactExportPayload` and `redact_export_payload` throw
`USER_RELEASE_ROUTE_REQUIRED` for `user_released_report`, whatever the intent
and the payload, after the context and user-release intent checks and before
any walk, as the generic route exporters do (§3.3). The walk decides a
value-bearing object as a whole and keeps an unlabelled value. In this context
a labelled object would therefore hide a restricted descendant: a nested
marker, protected or quarantined content, or a non-project private class. It
would also leave a private object's members other than `value` and `text`
unredacted. Sharing the projector's procedure was considered and not chosen,
for three reasons:
- Agreement with the projector on every input would need the projector's
  default label. The walk would then admit an unlabelled value with the
  wrapper's intent, which it blocked by step 13 before this change.
- Python carries no user-release projector (§3.3).
- Route 010 is lossless, so a redacted user-release payload has no consumer.

The classification of one item (`classifyExportItem` /
`classify_export_item`) still decides an item by steps 1-13; the shared corpus
holds it there. The corpus's new-context finding checks run on the
user-release projector in TypeScript, and on the item's own finding in
Python. Two corpus items state no privacy: the item rule blocks them by step
13, and on the projector they take the default label. The five existing
contexts' walk is unchanged.

### 3.3 Context and route

**A new governed route, `DREP-PACKAGE-USER-RELEASE-010`, with the new context.**
Reusing `shared_model` or `public_report` was considered and not chosen: either
would need a user-release flag inside an existing context, so that context's
behaviour would depend on a flag. The owner's check "redaction applies to every
other context" would then depend on every caller, not on the vocabulary.

Route tables and local-first admission:
- `GOVERNED_LOCAL_FIRST_ROUTE_IDS` (`redactionExportControls.ts:135-149`;
  `core/security/local_first_storage/route_control.py:17-41`, the
  `route_control.py` of this section) gain the route id.
- `enforceLocalFirstRoute` (`redactionExportControls.ts:151-209`) and
  `enforce_local_first_route` (`route_control.py:70-140`) gain an
  `explicitUserReleaseIntent` / `explicit_user_release_intent` option:
  - `user_released_report` on any route other than 010, or route 010 with any
    other context: `block_storage` / `USER_RELEASE_ROUTE_REQUIRED`;
  - route 010 without user-release intent: `block_storage` /
    `USER_RELEASE_INTENT_REQUIRED`;
  - otherwise `include_metadata_only` / `USER_RELEASE_METADATA_ALLOWED`, with
    storage context `private_report`. The file is written to the user's own
    local path by the same atomic save; the user release governs its content,
    not where it is stored. Python guards the route-metadata record under that
    existing local-private storage context (`local_first_storage/controls.py:25-33`),
    so the storage vocabulary does not change.
- `ROUTE_STORAGE_CONTEXTS` (`route_control.py:44-50`) maps
  `user_released_report` to `private_report`.
- **The intent strip is not widened (Codex round 1, finding 5).**
  `ROUTE_INTENT_KEYS` (`redactionExportControls.ts:708-712`), `stripRouteIntent`
  (`:825-842`) and the Python projector's strip
  (`core/security/redaction/route_control.py:19-23,159-180`) keep their current
  key lists.
  - User-release intent is read only from the wrapper's argument, so a payload
    member named `user_release_intent` or `explicit_user_release_intent`
    grants nothing and needs no strip.
  - In the five existing contexts and on routes 009, `DREP-IPC-003` and
    `DREP-HTML-SAVE-005`, such a member is raw content, preserved byte for byte
    as today, so a result document's self-hash and the exact-payload agreement
    are unaffected.
  - On route 010 it is not in the derivation and is refused natively
    (`REPORT-USER-RELEASE-CONTENT-NOT-DERIVED`, or
    `REPORT-USER-RELEASE-RESULT-NOT-ATTESTED` inside the carrier).
- `redactExportPayload` and `controlRouteExport` refuse
  `explicitUserReleaseIntent: true` with any context other than
  `user_released_report` (an exception, as `assertContext` throws,
  `:665-669`). The flag therefore cannot enable export in another context.
- **The generic route exporters refuse the context and the route (Codex r2
  of the r1 slice).** `controlRouteExport` and `control_route_export` throw
  `USER_RELEASE_ROUTE_REQUIRED` for `user_released_report` on every route and
  for route 010 in every context, after the context and user-release intent
  checks and before any projection, as they throw for an unsupported context.
  Their path-token projector would admit what the user-release screen
  refuses. The redaction walk refuses the context in the same way (§3.2). The
  user-release projector is therefore the only payload entry for the context
  and the route.
- The Rust mirror (`core/runner/headless/src/redaction_binding.rs:166,216-218`)
  implements the same rule so the shared corpus
  (`fixtures/redaction_export_controls/cases.json`, read at `:702-740`) holds.
  No runner route uses the context, and the runner's `export-results` refuses
  it (§2.8, refusal 1). Python carries the decision rule for corpus
  parity; no Python route uses the user-release projector.
- Schema `redaction_export_controls.schema.yaml`: `ExportContext` (:402-411)
  and `ReasonCode` (:485) enums gain the new values.

### 3.4 What still refuses, and what is unchanged

- **Exact routes (changed in revision 4; §0).** The W-11 opening admits
  exact-route results on the local-private package's output-policy cell
  (`reportPackageUnavailableReason`, `reportPackageRequest.ts:83-85`), its
  surface refusal (`refuseSurfaceOutput`, `:196`) and the producer's route
  preflight (item 0, `wire.rs:373-375`; `document_form.rs:19`). Revision 3
  relied on these to refuse them. Route 010 still uses them, so a retained
  route still refuses with its own code. It also refuses `pressure-2` and
  `pressure-source-1` results with
  `REPORT-USER-RELEASE-EXACT-ROUTE-NOT-ADMITTED` in three places:
  - the webview's user-release cell (§2.2);
  - native admission, before any exact source binding (§2.5 step 2a);
  - the producer, after item 0 (§2.8).

  The reader refuses a user-released package with an exact member (§2.9).
  The local-private routes keep the W-11 opening unchanged.
- **Protected-content linter.** The renderer runs it before and after rendering
  in every context (`report_renderer/src/lib.rs:2057-2098,2140-2159`); a
  blocking finding sets `export_blocked`, and admission refuses a blocked
  package (`REPORT-PACKAGE-PRODUCER-BLOCKED`, `report_package_save.rs:856-858`).
  Since #1639 a render refused for its provenance records returns no body
  (`:2012-2048`); §2.6 states how `user_released_report` meets those rules.
- **Disposition blocks.** The webview blocks `protected_suspected`,
  `quarantined` and `rejected` before any context branch (steps 8-9 of §3.2).
  Native admission runs both screens on route 010 as on 009
  (`report_package_save.rs:524-560`): `PROTECTED_CONTENT_BLOCKED`,
  `REPORT-PACKAGE-SAVE-DISPOSITION-MALFORMED` and, from the store-resolved
  screen, `MODEL-DISPOSITION-RECORD-MALFORMED` (W-17, #1651 and #1653).
- **Object form.** The save's `REPORT-PACKAGE-SAVE-REQUEST-POSITIONAL`, the
  producer's positional-report refusal (`wire.rs:376-381`) and the render's
  `check_object_form` apply on route 010 unchanged (#1650).
- **Every other context is unchanged.** `public_report`, `public_example`,
  `shared_model`, `downstream_tool` and `local_private` keep every decision on
  every input, and their controlled payloads keep their bytes, including raw
  members named like the new intent keys (§3.3). Routes 009, `DREP-IPC-003` and
  `DREP-HTML-SAVE-005` keep their contexts and evidence. A local-private
  package is byte-identical to one assembled before W-18b (§4).

## 4. Persistence and format

**Version: V1, stay at 2.0.0 (owner, 2026-10-25).** The owner's DEC-028
reading, recorded in §0, is: "one version" admits an optional writer-defined
record that the reader at the same head accepts. Under that reading the
user-released form is the 2.0.0 container selected by the optional record.
With the record:
- the manifest carries the optional `user_release` member (§2.3) with its
  `content_binding`;
- the manifest's boundary notes are the user-released set (§5.3);
- the writer writes no state/comparison/handoff member (§2.1, §2.8).

Without the record, every rule is today's. The reader at the same head accepts
exactly that form and refuses any mix: a record with a handoff member, a
handoff member missing without a record, a record under another profile.

**Packages without a user release stay byte-identical.** Such a package is
byte-identical to one assembled before W-18b. Its members, its manifest and its
notes are unchanged. Every existing package, fixture and witness is unchanged,
and so are the runner's witnesses. The intent strip is not widened (§3.3), so
no payload's bytes change. This is the most literal reading of "without the
selection, the report stays local-private exactly as today". It is held by
`local_private_package_bytes_unchanged_by_user_release` (§6).

**Consequence.** A reader built before W-18b refuses a user-released package as
`OPSPROJ_MANIFEST_MALFORMED` ("manifest differs from the version 2.0.0 manifest
form at: user_release", `reader.rs:248-252`), not by version name. No such
reader has been in a software release.

**The absent handoff member is part of the record-selected form.** HELP_HUMAN
settled on 2026-10-25 that V1 covers R's package form (§0): the reader at this
head accepts what the writer at this head emits, and that includes a
user-released package with no state/comparison/handoff member. So R keeps no
handoff record set, and the reader admits its absence only with the record.

**Rendered reports.** No JSON schema file describes the renderer input; its
contract is the Rust type (`report_renderer/src/lib.rs:249-263`) and the
TypeScript builder. Changes:
- `ReportExportContext` (`:82-86`) gains `UserReleasedReport`
  (`user_released_report`); `export_context_label` (`:445-450`) gains it;
- `ReportContext` gains `user_release` (§2.3), defaulting to `None`;
- `UserReleaseRecord` is a new typed struct with `deny_unknown_fields` and no
  `content_binding` member;
- `report_context_blocking_codes` (`:1824-1855`) adds
  `REPORT-CONTEXT-USER-RELEASE-MISMATCH` when the context is
  `user_released_report` without a record, or a record is present in another
  context;
- TypeScript `ReportExportContext` (`renderableReportInput.ts:72`) gains the
  value;
- `provenance_disposition_blocking_reasons` (`:1731-1820`) treats a
  `private_project_data` record under `user_released_report` with a record as
  the user-released class (§2.6); `provenance_context` (`:1659-1664`) is unchanged,
  so `user_released_report` is judged as not local-private (C1 b);
- the input's `exact_route` and `library_references` (W-11) are absent in a
  user-released input (§2.6), so they emit nothing.

`report_generator` and `schemas/report_generator.schema.yaml` do not change:
the record is in `report_context`, not in `calculation_report`. Existing
local-private and public renders are byte-identical, since the new member
defaults to absent and adds no row or banner then.

**Witnesses and fixtures.**
- Unchanged:
  - the headless runner inputs and witnesses
    (`validation/witness/inputs/del1005_*`, `validation/witness/generated/del1005_*`);
  - every existing reader and container fixture;
  - the existing corpus cases;
  - the owner-held `apps/desktop/native-smoke/expected.json` (not touched).
- New, invented:
  - a user-released profile in the shared basis builder
    (`core/reporting/report_package/tests/common/package_basis.rs`);
  - a committed record for that basis,
    `fixtures/reports/invented/user_release_record.json`;
  - the derivation vectors (§2.6);
  - new corpus cases in `fixtures/redaction_export_controls/cases.json`, each
    with `explicit_user_release_intent`.

## 5. The notice

### 5.1 Notice text (proposed exact)

The renderer gains `user_release_notice_text(input) -> Option<String>`. The
HTML body uses it (beside `local-private-banner`, `report_renderer/src/lib.rs:2115-2119`),
and so do the PDF (`pdf_emitter/src/lib.rs:254`) and the reader's notice check
(§2.9). It is emitted only for `user_released_report` with a record, in a
banner of its own,
`<div class="user-release-banner" data-export-context="user_released_report">`:

> USER-RELEASED REPORT — the user released their own project data in this
> report for sharing, stating that they hold the right to redistribute it
> (IP and data boundary §6). The user release covers this report only: model
> SHA-256 {model_document_sha256}, result document {result_envelope_id}
> (SHA-256 {result_document_derivative_sha256}), user-released {user_released_at}.
> The report carries this model's summary, load cases and solve results, and
> no rule-pack reference, user-rule outcome or run comparison. The application
> found no library or rule-pack mark in this model. It cannot verify who holds
> the rights in the data, which rest on the user's statement. This statement
> concerns rights in the data only; the analysis status and the Professional
> Boundary notice below are unchanged.

The run-context table (`run_context_rows`, `:452`) shows "Report export
context: user_released_report" and three added rows:
- "User release statement": the statement;
- "User release time (application clock)": `user_released_at`;
- "User release bound to": the model, run, input-manifest and result-document
  identities.

`content_binding` is not rendered.

### 5.2 Relation to the other notices

- **PRD §19.3** (`docs/PRD.md:1290-1296`; renderer `:1132-1155`): unchanged, in
  every report, user-released or not. A user release changes no analysis status
  and adds no review record.
- **The local-private notice** (W-3, `local_private_notice_text`,
  `report_renderer/src/lib.rs:1294-1303`): unchanged and still emitted for
  every `local_private` report. A user-released report carries the user-release
  notice in its place, because the two contexts exclude each other.
- **The panel's local-private warning** (`RenderedReportPanel.tsx:51-52`):
  unchanged. After a user-released save the status line reads:
  "User-released report package saved: it contains this project's values and
  results for sharing, as you stated in the Save Shareable Report Package dialog."

### 5.3 Package boundary notes (user-released packages only)

The three notes of `lib.rs:742-748` become, for a package with a record:
1. unchanged: "This container is an artifact format, not an issuance act;
   package contents are engineering decision-support information."
2. unchanged: "Human professional review is required before any reliance on
   package contents."
3. "No timestamps or generated identifiers are recorded in this package except
   user_release.user_released_at, the time of the user release;
   identical inputs, including that time, produce identical bytes."
4. added: "The user released their own project data in this package for
   sharing (user_release). The record is the user's statement about rights in
   that data; it is not an issuance act."

Note 3 is exact under R. The run record is the solve's transient record, with
`created_at: null` and the `created_at_unavailable` note
(`analysisRunCompatibility.ts:192,197`). The renderer input records no
generation time and no run recording time (§2.6). So `user_released_at` is the one
timestamp in the package. Packages without a record keep the three notes
exactly.

### 5.4 Claims-language check

Every proposed text in §2.2, §5.1, §5.2, §5.3 and Appendix A was run through
GEN-13 (`tools/validation/validate_claims_language.py`) at revision 3, both as
product source (`apps/desktop/src/…`) and as a project document, through the
tool's own line and retired-text scans. Result: no finding.
- No line carries three litany terms (`:44-56`). The highest count is one:
  "professional", in "Professional Boundary notice" and in note 2, which is
  unchanged text.
- No text uses the hedge phrasing that registry §1 retires, or a BS-ACCEPT or
  BS-MATURITY text (`docs/claims_registry.md` §1).
- The texts state a user's data right. They do not restate PRD §21 and add no
  prohibition list (registry §4).
- The PRD §19.3 notice and its lint anchors are untouched.

**Revision 4's new and changed texts have not been run through GEN-13.**
They are:
- the dialog's exact-route line and its changed bullet (§2.2);
- the webview refusal text of `REPORT-USER-RELEASE-EXACT-ROUTE-NOT-ADMITTED`
  (§2.2);
- the re-derived A1, A10, A11 and A12, and the new A14;
- after Codex round 1 at revision 4 (S4), the naming changes:
  - the statement (§2.2, §2.3, A11);
  - the notice's "The user release covers…" and "user-released
    {user_released_at}" (§5.1);
  - the status line (§5.2) and note 3 (§5.3);
  - A1, A2, A4, A5, A7 (also its detector, S2), A9, A11, and A14 (S3);
- the runner's refusal message `REPORT-USER-RELEASE-RUNNER-NOT-ADMITTED`
  (§2.8).

The revision 4 assignments ran no checks beyond Appendix A's `count == 1` and
the naming sweep. These texts go
through GEN-13 with the Codex review of the exact text, before HELP_HUMAN
(§6). Each was written to the same rules: no litany terms, no retired hedge
phrasing and no prohibition list.

Round 1's lint-scope finding is resolved on main: #1626 changed the
ScopeOfWork glob to `execution/PKG-*/DEL-*/ScopeOfWork.md`
(`validate_claims_language.py:268`), so the full lint now scans the live
ScopeOfWork files, Appendix A's targets among them.

## 6. Verification plan

The work touches the data boundary, persistence and file output. Each slice
therefore has targeted regression tests and an independent Codex review on its
own head. Pytest runs under `t3_slot.sh` with `-n 4`, vitest with
`--maxWorkers=3`, and cargo only through the T3 wrappers.

**Round 1 to round 3 counterexamples, as named regressions.** Each is refused,
or, where stated, admitted as a control or preserved. Each refused native case
leaves the sandbox directory empty, with no picker request.
- `user_release_library_applied_bend_refused` (round 2):
  - a model whose bend has `modifiers` and a `provenance_record.library_ref`
    from a component apply is refused natively with
    `REPORT-USER-RELEASE-LIBRARY-REFERENCED`, before attestation or
    derivation;
  - the same model with the `library_ref` removed, but its
    `modifiers.source_reference` still `library:component:…#…`, is refused
    the same way;
  - the same model with both marks removed is admitted: no library origin is
    detected, and the user release relies on the user's statement (the reliance
    limit, §2.7). The test pins that limit; it does not show the values are
    the user's own.
- `user_release_library_reference_sites_refused`: one case per site of
  `library_ref_sites.json`, including a blank-identity site, and a spring
  hanger applied from a library (its `library_ref` and its `provenance`
  string, `library:hanger:…`). Each is refused natively; in the webview the
  dialog's save is disabled with the same reason. Revision 4, Codex round 1
  (S2) adds a `library_ref` member that is not an object: `null`, a string, a
  number, a boolean, an array and a positional record, on an entity and
  nested.
  - The detector alone, with no disposition screen (a Rust unit test of
    `library_ref_member_sites` and a vitest of `libraryRefMemberSites`), names
    each one as a rule-1 site with its pointer. Admission's early-exit
    `first_library_ref_member` finds each in turn as that walk's first site,
    with the same pointer.
  - Natively each is refused, whichever check runs first.
  - In the webview the save is disabled.
  - The same members held inside embedded JSON (rule 3 (b) through rule 1),
    which the disposition screen never sees, are refused natively with
    `REPORT-USER-RELEASE-LIBRARY-REFERENCED`.
- `user_release_legacy_hanger_selection_refused` (round 3, blocking): an
  invented model whose support `provenance` is the legacy hanger selection
  evidence exactly as the pre-W-17 writer built it (the RFC 8785 text of the
  six members, from `hangerSelection.ts:35-39` at `00af9112c0^`), with no
  `library_ref` and no `library:` string, and whose solve has genuine
  `spring_hanger_user_input_review` rows. One case per recognition path, each
  refused natively with `REPORT-USER-RELEASE-LIBRARY-REFERENCED` and the rule
  `legacy_hanger_selection`, before attestation or derivation, and disabled in
  the webview:
  - the full legacy string (rules 3 (a) and 3 (b));
  - the same string with `kind` removed, so only the embedded-JSON members
    remain (3 (b)), once for each of `library_kind`, `library_id`,
    `library_metadata` and `record_snapshot` kept alone;
  - the same object with every one of those four members removed but `kind`
    kept (3 (a));
  - the full string with leading and trailing JSON whitespace;
  - an embedded object holding only a nested `library_ref` (3 (b) through
    rule 1);
  - an embedded object holding only a `library:hanger:…` string. §2.7's
    order governs: rule 2 tests the outer string first, so the plain form is
    refused as `provenance_string`; with the prefix escaped in the JSON text
    (for example its colon written `\u003a`), rule 2 does not match the outer
    string, and the prefix found after parsing (3 (b) through rule 2) is
    refused as `legacy_hanger_selection`;
  - the legacy string held in an array element and at a nested object member,
    not only at `supports[].provenance`;
  - a candidate beginning with `{` that the prescan refuses for nesting deeper
    than 127 containers, and one with the reserved raw-value key (unchecked,
    refused under rule 3).

  Controls, each admitted (the reliance limit, §2.7): a provenance string that
  begins with `{` but is not JSON; an embedded JSON array; an embedded object
  with none of the members and no mark; and a legacy object reduced to its
  `hanger_id` alone. A legacy string embedded inside a string of an embedded
  object is refused by 3 (a), since the substring test sees it at any depth.
  The hanger form is the only legacy form (§2.7): the material, section and
  component applies have always written rules 1 and 2's marks, which the two
  cases above cover.
- `user_release_rule_pack_reference_refused`: a model with one
  `rule_pack_refs[]` entry, with or without a rule-check run, and an input
  manifest with one active pack. Each is
  `REPORT-USER-RELEASE-RULE-PACK-REFERENCED`.
- `user_release_rule_aggregate_refused`: a run record whose status carries
  `USER_RULE_CHECKED` or `USER_RULE_FAILED` (the session's composed record) is
  `REPORT-USER-RELEASE-ANALYSIS-RECORD-UNBOUND`.
- `user_release_unknown_annotation_refused` (round 1): a source row whose
  `metadata` carries an extra key with an invented marker value, added only in
  the request, is refused by the attestation
  (`REPORT-USER-RELEASE-RESULT-NOT-ATTESTED`). The same extra member at any
  other request pointer is refused by the derivation
  (`REPORT-USER-RELEASE-CONTENT-NOT-DERIVED`, naming the pointer).
- `user_release_rule_pack_class_unknown_redistribution_redacted` (round 1): an
  object with `privacy_classification: private_rule_pack_data` and
  `redistribution_status: unknown` (and `TBD`, and null):
  - is `redact_value` / `PRIVATE_DATA_REDACTED` in `user_released_report` in
    TypeScript, Python and the Rust mirror (a corpus case), and the lossless
    route is withheld;
  - natively, is `REPORT-USER-RELEASE-PRIVATE-CLASS`.

  Its twin with `private_project_data` and unknown redistribution, without
  wrapper intent, is `block_export` / `USER_RELEASE_INTENT_REQUIRED`.
- `user_release_record_model_ref_unbound` and
  `user_release_record_report_id_unbound` (round 1): a record with only
  `model_ref`, or only `scope.report_id`, changed, every other member
  coherent. Each is refused natively (`REPORT-USER-RELEASE-UNBOUND`), by the
  producer (`REPORT-PACKAGE-USER-RELEASE-UNBOUND`) and, in a re-hashed package,
  by the reader (`OPSPROJ_USER_RELEASE_UNBOUND`).
- `user_release_extra_member_refused`: a valid user-released package with one of
  these added and the inventory restamped:
  - a second, valid result member;
  - a valid state/comparison/handoff member.

  The reader refuses each.
- `user_release_same_source_member_substitution_refused` (round 1): a valid
  user-released package whose HTML member is replaced by the HTML of another
  user-released save of the same model and run, with a different
  `user_released_at`.
  Its PDF is swapped the same way, and its audit member with an
  equal-identity variant. The inventory is restamped each time and the record
  left unchanged. The reader refuses each (`content_binding`; and the notice
  text for the HTML).
- `existing_contexts_preserve_raw_intent_named_members` (round 1): a request
  whose source-row `metadata` and a handoff section carry raw members named
  `user_release_intent` and `explicit_user_release_intent`, under
  `local_private` on route 009 and on `DREP-IPC-003` and `DREP-HTML-SAVE-005`.
  - The controlled payload is byte-identical to the input, the result
    document's self-hash verifies, and the exact-payload agreement holds, as at
    the parent head (a vitest and a pytest of the projector, plus a Tauri
    admission test on route 009).
  - The same members on route 010 are refused natively.
- `user_release_exact_route_refused` (revision 4, §0). Each case uses an
  eligible `pressure-2` and an eligible `pressure-source-1` result that Export
  Result JSON admits, and the local-private package admits each one as a
  control:
  - **webview** (vitest): the user-release cell returns
    `REPORT-USER-RELEASE-EXACT-ROUTE-NOT-ADMITTED`, the dialog shows its
    exact-route line with the save disabled, and
    `buildUserReleasedReportPackageRequest` throws with the code. For the same
    result, `reportPackageUnavailableReason` and the local-private Save Report
    Package… still admit it;
  - **native, through the outer command path** (Tauri, sandbox directory,
    counting picker). Each case enters as the command does:
    `command_ingress::admit` with a `save_report_package` body, then
    `ingress_lease_bytes` and the lease growth, then admission (§2.5, "The
    outer command"). Each is refused on route 010 with that code, with no
    custody query, no picker request and the directory left empty:
    - a pressure-free request that carries `exact_result_text`, and one that
      carries `exact_intake_text`, each once as valid JSON, once as text that
      is not JSON and once over its cap. The refusal comes from
      `ingress_lease_bytes`, never `REPORT-PACKAGE-SAVE-NOT-CANONICAL` or
      `REPORT-PACKAGE-SAVE-TOO-LARGE`, and the lease's bytes equal the
      ingress admission's: no growth at all;
    - a request whose result document has an exact identity, and a
      pressure-free request whose report carries a non-null `exact_route`.
      Each is refused at step 2a with the lease grown by exactly route 010's
      reservation: `parse_lease_bytes` over its three texts, the run-record
      text's pricing, the derivation's and the refusal formatting allowance,
      and no exact-text growth.

    The same exact request on route 009, with its exact texts, is still
    admitted, and its lease grows by `parse_lease_bytes_with_exact` as at the
    parent head;
  - **producer** (`tests/user_release_package.rs`; Codex round 2 at
    revision 4, S6). Each case is called through the public entry
    `user_release::assemble_derived`, with a basis whose derivation is
    coherent apart from the exact document:
    - the derivation vector's expected request with its result document
      replaced by the eligible exact document, once with the report
      unchanged, once with a report that differs from the derivation (a
      changed `report_title`), and once with the report in positional form.
      Each returns `REPORT-USER-RELEASE-EXACT-ROUTE-NOT-ADMITTED`, never
      `REPORT-USER-RELEASE-CONTENT-NOT-DERIVED` and never the object-form
      code, and the message names no JSON pointer;
    - the same, under the desktop local-private profile with no record: the
      same code, since the preflight uses the profile constant;
    - a `physics-retained-1` document: item 0's own refusal
      (`REPORT-PACKAGE-ROUTE-NOT-ADMITTED`), before the exact check.

    Through each general entry (`wire::assemble_wire_request`,
    `bindings::check_request`, `assemble_report_package_container`), the
    first request above gets `REPORT-PACKAGE-USER-RELEASE-NOT-ADMITTED`.
    Through the first two, which run `check_runner_profile`, the same request
    under the runner profile gets `REPORT-PACKAGE-EXACT-RUNNER-PROFILE` (§2.8,
    "Precedence for an exact document"). The W-11 forwarding-graph scans (`admitted_tests`,
    `report_render_exact_tests`) pass, with the one added root (§2.8);
  - **reader** (`tests/reader.rs`): a user-released package whose result
    member is an exact document, re-hashed coherently, is
    `OPSPROJ_USER_RELEASE_UNBOUND`.
- `user_release_runner_refused` (revision 4, Codex round 1, B1; §2.8). The
  request is the expected request of a derivation vector: coherent,
  pressure-free, desktop origin, the user-released profile and a record,
  which `assemble_derived` admits. Its session provenance notes are resolved
  `private_project_data`.
  - **CLI** (a pytest of the built `openpipestress-runner`, beside the
    del1005 export-results tests, and a Rust test of `execute_export_results`):
    `export-results --input <request> --output <path>` is run with and
    without `--explicit-local-private-intent`. Each time it produces no
    package output:
    - exit 1;
    - the `REPORT-USER-RELEASE-RUNNER-NOT-ADMITTED` diagnostic, naming the
      member;
    - no `report_package` in stdout;
    - no file at `<path>`.

    The same holds for each marker alone: the profile without a record; a
    record under the runner profile with the runner's attested origin; and
    `user_released_report` under the desktop local-private profile. The
    runner's existing witnesses (`del1005_*`) are unchanged.
  - **Producer** (`tests/user_release_package.rs`):
    `wire::assemble_wire_request`, `bindings::check_request` and
    `assemble_report_package_container` each refuse the same request and each
    marker alone with `REPORT-PACKAGE-USER-RELEASE-NOT-ADMITTED`.
    `assemble_derived` admits the coherent request, and refuses it once any
    member differs from the derivation. W-11's forwarding-graph scan passes
    with its one added root.
  - **Reach scan:** `user_release_reach_scan` (§2.8) passes on the real
    sources and on an invented closed set of the four crates, and fails on an
    invented added call site of `assemble_derived` outside the bridge. The
    invented sources are string literals, as W-11's scans test theirs
    (`report_render_exact_tests.rs:655-703`). Each change below, made alone to
    an existing dependent in the invented set, must be a finding (Codex round
    2 at revision 4, S5):
    - in `report_package`'s `lib.rs`, a new
      `pub fn forward(input: &renderer::RenderableReportInput)` whose body is
      `pdf::assemble_full_report_package(input)`;
    - in `report_package`, a new `pub fn` that calls
      `pdf::emit_calculation_report_pdf`, one that calls
      `renderer::render_calculation_report`, and one that calls
      `pdf::pdf_from_render` with a fabricated `RenderOutcome`;
    - in `pdf_emitter`, a new `pub fn` that calls
      `renderer::render_calculation_report`, and one that calls
      `pdf_from_render` or `renderer::assemble_report_sections`;
    - a second call of `pdf::assemble_full_report_package` in the writer;
    - `use open_pipe_stress_pdf_emitter::assemble_full_report_package as build;`
      with a call `build(input)` in a new function;
    - `pub use open_pipe_stress_pdf_emitter::assemble_full_report_package;`,
      `pub use pdf::*;` and `pub use open_pipe_stress_pdf_emitter as pdf;` in
      `report_package`'s `lib.rs`;
    - a function value: `const BUILD: fn(&renderer::RenderableReportInput) ->
      pdf::FullReportPackageOutcome = pdf::assemble_full_report_package;`, and
      `std::iter::once(input.report).map(pdf::assemble_full_report_package)`
      in the writer in place of its direct call;
    - in the writer, `let build = |input| pdf::assemble_full_report_package(input);`
      in place of its direct call, and the same call inside a multi-line
      closure body;
    - the same call inside a `macro_rules!` body in `report_package`;
    - in `src-tauri`, a call of `package::assemble_full_report_package` (the
      shape a glob re-export would allow), a
      `use open_pipe_stress_report_renderer::render_calculation_report;`, and a
      second `open_pipe_stress_report_renderer::render_calculation_report` call
      outside `render_calculation_report_value`;
    - a `DerivedUserRelease { … }` literal outside `check_derived`, and a call
      of `check_derived` outside `assemble_derived`;
    - `ProducerEntry::UserRelease(derived)` passed to
      `wire::assemble_wire_request_in` from a function other than
      `assemble_derived`, and `ProducerEntry::UserRelease` in place of
      `ProducerEntry::General` at a public root.

    Codex round 3 at revision 4 (S5a, S5b) adds the cases below. Each must be
    a finding unless it is marked as passing. The scan takes the sources as a
    map from path to text and resolves modules within that map, so an invented
    file such as `escape.rs` is one more entry in the map.
    - **A module-level macro after the writer** (S5a), in `report_package`'s
      `lib.rs`, at column 0 after the writer's closing brace:
      `macro_rules! build { ($i:expr) => { pdf::assemble_full_report_package($i) }; }`.
      The writer's direct call is replaced by `build!(input.report)`. Two
      findings are expected: the scanned name in the macro's tokens, and the
      writer's zero direct calls.
    - **A public wrapper invoking that macro** (S5a): the same macro, with the
      writer unchanged, and a new
      `pub fn wrap(input: &renderer::RenderableReportInput) -> pdf::FullReportPackageOutcome { build!(input) }`.
    - **A de-indented multi-line closure** (S5a). In the writer, the direct
      call is replaced by a three-line closure:
      - `let build = |input| {`
      - `pdf::assemble_full_report_package(input)`, written at the writer's own
        statement indentation and with no `|` on its line;
      - `};`
      It is followed by `let report = build(input.report);`. The call's
      innermost scope owner is the closure.
    - **The test and production pair** (S5b), in `report_package`'s `lib.rs`:
      `#[cfg(test)] mod escape;` and
      `#[cfg(not(test))] #[path = "escape.rs"] pub mod live_escape;`, with an
      invented `escape.rs` holding
      `pub fn forward(input: &crate::renderer::RenderableReportInput) -> crate::pdf::FullReportPackageOutcome { crate::pdf::assemble_full_report_package(input) }`.
      The findings are the production `#[path]` and the call in `escape.rs`,
      which is read as production. The same pair without `#[path]`
      (`#[cfg(test)] mod escape;` and `#[cfg(not(test))] pub mod escape;`)
      must still read `escape.rs` as production and report the call.
    - **Other production cfgs.** A second
      `open_pipe_stress_report_renderer::render_calculation_report` call in
      `src-tauri`, in the invented `native_smoke.rs`
      (`#[cfg(feature = "native-smoke")]`), and the same in an inline
      `#[cfg(target_os = "macos")] mod` are each read as production and
      reported.
    - **`include!` of Rust** (S5b): `include!("forward.rs");` in
      `report_package`'s `lib.rs`, with an invented `forward.rs` holding a
      forwarding `pub fn`. The `include!` is the finding, whatever the file
      holds. The same holds for `include_str!("forward.rs")`, and for an
      `include_str!` whose argument is `concat!(…)`.
    - **A trait object:**
      `pub fn builder() -> Box<dyn Fn(&renderer::RenderableReportInput) -> pdf::FullReportPackageOutcome> { Box::new(pdf::assemble_full_report_package) }`.
    - **Generic and `impl Fn` forms:**
      - `fn apply<F: Fn(&renderer::RenderableReportInput) -> pdf::FullReportPackageOutcome>(f: F, input: &renderer::RenderableReportInput) -> pdf::FullReportPackageOutcome { f(input) }`,
        called as `apply(pdf::assemble_full_report_package, input)` from a
        new `pub fn`;
      - `pub fn builder() -> impl Fn(&renderer::RenderableReportInput) -> pdf::FullReportPackageOutcome { pdf::assemble_full_report_package }`;
      - the same `builder` returning `|input| pdf::assemble_full_report_package(input)`.
    - **Methods on a wrapper:**
      - `pub struct Builder;` with
        `impl Builder { pub fn build(&self, input: &renderer::RenderableReportInput) -> pdf::FullReportPackageOutcome { pdf::assemble_full_report_package(input) } }`;
      - the same body as a trait method in `impl From<…> for …` or in an
        invented trait's impl;
      - an associated `const` holding the function.
    - **A nested `fn`** inside the writer that holds the call, with the
      writer's own call removed, and an `async` block in the writer that
      holds it.
    - **A method call by the scanned name:** `input.assemble_full_report_package()`
      in any function.

    Codex round 4 at revision 4 (S5c, S5d, N4) adds the cases below (§2.8,
    *Macros*). Each must be a finding unless it is marked as passing:
    - **A macro that loads a module** (S5c), in `report_package`'s `lib.rs`:
      `macro_rules! load_escape { () => { pub mod escape; }; } load_escape!();`,
      with an invented `escape.rs` holding
      `pub fn forward(input: &crate::renderer::RenderableReportInput) -> crate::pdf::FullReportPackageOutcome { crate::pdf::assemble_full_report_package(input) }`.
      The findings are the unlisted definition and the unlisted invocation.
    - **A parameterized path** (S5d), in `src-tauri`:
      `macro_rules! build { ($r:ident, $i:expr) => { $r::render_calculation_report($i) }; }`,
      invoked as `build!(open_pipe_stress_report_renderer, input)` in a new
      function. The findings are the definition and the invocation.
    - **An unlisted external declarative macro** (N4), from an invented
      dependency `forwarder`, in `report_package`:
      - `forwarder::emit!(input)` in a new function;
      - `use forwarder::emit;` with `emit!(input)`;
      - `use forwarder::emit as json;` with `json!(input)`, whose `use` is the
        finding (import rule), and the same with `as format`;
      - `#[macro_use] extern crate forwarder;`.
    - **A new local macro without a scanned name:**
      `macro_rules! twice { ($e:expr) => { ($e, $e) }; }` in `pdf_emitter`,
      with and without an invocation, and the same as a `macro` (2.0) item.
    - **A changed `app_invoke_handler`:**
      - one token changed: a handler entry added, removed or renamed;
      - whitespace or a comment changed inside it, which the pin also
        covers;
      - the basis text unchanged but moved to another `src-tauri` file, or
        defined twice.
    - **Unlisted attributes and derives:**
      - an attribute macro `#[forwarder::instrument]` on a function in
        `report_package`;
      - `#[tauri::command]` on a `src-tauri` function named
        `assemble_user_released_request`;
      - `#[serde(serialize_with = "forward")]` on a field, and
        `skip_serializing_if = "forward"`;
      - `#[derive(forwarder::Forward)]`, and `#[derive(Hash)]`, which is
        harmless but not on the list;
      - `#[cfg_attr(not(test), forwarder::instrument)]`.
    - **Passing cases for today's macros.** Each of these must pass:
      - a function that invokes each listed `std` macro once, with arguments
        that hold no scanned name;
      - bare `json!` and `params!` under `use serde_json::json;` and
        `use rusqlite::params;`, and the qualified `serde_json::json!` and
        `rusqlite::params!`;
      - `serde::forward_to_deserialize_any!` inside a `Deserializer` impl;
      - the basis text of `app_invoke_handler`, byte for byte, with its two
        invocations from `run` (without arguments, and with the
        `native_smoke::…` list), and `tauri::generate_context!`;
      - `#[tauri::command]` on an ordinary command;
      - each listed derive, the basis `serde` keys with both
        `skip_serializing_if` values, `#[default]`, and
        `#[cfg_attr(not(test), allow(dead_code))]`;
      - a `#[cfg(test)] thread_local!` item, and a `#[cfg(test)]` module that
        defines its own `macro_rules!`, as `command_ingress.rs` does.
    - **Passing cases.** Each of these must pass:
      - the basis text of `app_invoke_handler`, a `src-tauri` `macro_rules!`
        whose tokens name the bare Tauri command `render_calculation_report`
        and whose pin matches;
      - an `include_str!` of a `.json` file;
      - a `#[cfg(test)]` item, inline module or outlined module that holds any
        of the calls above;
      - a `ProducerEntry::UserRelease(_)` pattern in `check_request_in`.

    Codex round 1 at #1682 (P2a, P2b) adds the cases below (§2.8,
    *Spellings*; rule 3). Each must be a finding unless it is marked as
    passing; the first five are also applied to the real sources:
    - **Raw identifiers and a leading `::`:**
      `open_pipe_stress_report_package::user_release::r#assemble_derived(…)`
      outside the bridge; `::open_pipe_stress_report_renderer::render_calculation_report(input)`,
      the renderer function or crate written raw, a `fn` pointer and a
      closure through `::`, and the raw renderer path in a macro's tokens, in
      `src-tauri`; a raw scanned name in a macro's tokens, as a method and as
      a `fn` pointer; a `r#DerivedUserRelease` literal in its own module; a
      raw `ProducerEntry::r#UserRelease(derived)`; and `mod r#escape;`,
      whose `escape.rs` is read;
    - **Other prefixes:** `Self::render_calculation_report` and a re-export
      path to `assemble_report_sections` in `src-tauri`;
      `<Bridge as Assemble>::assemble_user_released_request(r)` and
      `<pdf::Emitter>::assemble_full_report_package(i)`; `::forwarder::emit!`;
    - **Imports:** `use ::open_pipe_stress_report_renderer as r;` and the
      crate written raw in `src-tauri`; a raw scanned name imported `as`
      another; `use open_pipe_stress_pdf_emitter as forge;`;
      `pub use crate::pdf as p;` and `pub(crate) use self::renderer as r;`;
      `use crate::user_release::ProducerEntry::UserRelease as U;`;
    - **`DerivedUserRelease` and `UserRelease` without their literal:** a
      `Self { _derived: () }` in an `impl` of it, a type alias and an
      associated type naming it, a derive on its definition (direct and
      through `cfg_attr`), an `unsafe` block, an `unsafe fn` and an `extern`
      block in a reporting crate, and `let f = ProducerEntry::UserRelease;`;
    - **Conditional attributes (P2b):**
      `#[cfg_attr(all(), cfg_attr(all(), path = "alternate.rs"))] mod ordinary;`
      (Rust loads `alternate.rs`), the same with a false predicate
      (`any()`), `#[cfg_attr(all(), derive(UnlistedDerive))]` direct and
      nested, `#[cfg_attr(all(), derive(Clone), derive(Hash))]`, a `serde`
      key, `macro_use`, `tauri::command` on the bridge function and an
      attribute macro under nested `cfg_attr`s, and a macro in a `cfg_attr`'s
      `doc = …` value;
    - **Passing cases:** `#[::tauri::command]` with `::serde_json::json!`;
      `crate::render_calculation_report(input)` in `src-tauri`, the local
      command; and a nested `cfg_attr` applying `allow(dead_code)` and listed
      derives.

    Codex round 2 at #1682 (P2) adds the cases below (§2.8, *Spellings*).
    Each must be a finding unless it is marked as passing; Codex's three
    repros are also applied to the real sources:
    - **Another name of `DerivedUserRelease`:** Codex's repro
      (`impl Identity for DerivedUserRelease { type Repr = Self; }` and
      `Alias::<DerivedUserRelease> { _derived: () }`, three findings: the
      impl, its associated type, the literal); the trait in another module
      with `<DerivedUserRelease as other::Identity>::Repr { .. }`; a generic
      associated type; `<DerivedUserRelease as Identity>::Repr { .. }` and
      `Alias::<DerivedUserRelease> { .. }` through a blanket impl; a plain
      alias through a defaulted parameter
      (`struct W<T = DerivedUserRelease>`), outside a macro, in a macro's
      tokens and after an `if … else` block there; generic arguments and a
      qualified path in a macro's tokens; `Self::Repr { .. }` in another
      type's impl; `Self { .. }` in a trait's default method and in an impl
      of an alias; an import renamed and written as a literal; a leading `::`
      literal; a trait impl for `&DerivedUserRelease` and one from another
      file;
    - **`ProducerEntry::UserRelease` constructions:** the braced form, through
      `Self::` and in a macro's tokens; a `matches!` guard (Codex's repro),
      an `if let` guard, the scrutinee, a guard after an or-pattern, a
      `matches!` inside `format!` and inside `assert!`, an inline `const`
      pattern, and a `matches!` that does not parse;
    - **Passing cases:** plain literals of the crate's structs in the
      defining module, outside a macro, as `Self` in their impl, and in a
      macro's tokens beside `if`, `match` and `for` blocks; a struct pattern
      in a `matches!`; an inherent impl of `DerivedUserRelease`; and
      `matches!` patterns naming `UserRelease` with a guard and with a
      trailing comma.

    Code review r3 at #1682 adds the cases below (§2.8, *Conservative
    rules*). Each must be a finding unless it is marked as passing; the
    reviewer's three repros, with their shared prelude
    (`trait ReviewIdentity<T = DerivedUserRelease> { type Repr; }` and
    `impl<T> ReviewIdentity<T> for () { type Repr = T; }`), are applied to
    the real sources, and each must give findings on its own lines:
    - **The repros:** an alias declared in `vec!`'s tokens and written as a
      literal (`type UserReleaseInputs = <() as ReviewIdentity>::Repr;`);
      `type r#const = …;` written as `vec![r#const { _derived: () }]`; and a
      `ProducerEntry::UserRelease(ReviewAlias { .. })` inside a const generic
      argument of a `matches!` pattern;
    - **Type aliases in the defining module:** at item level, in a block, in
      an inline module, as an impl's and a trait's associated type, and in
      `vec!`, `format!`, `format_args!`, a `json!` value, a `matches!` guard
      and a listed macro nested in another; `println!` and `assert!` there,
      and `serde_json::json!` by its path, are unlisted, and `println!`'s
      unparsed block is a finding too; `json!` tokens that are not its
      grammar (a block for an object, an empty entry); a literal of an alias
      in parsed `json!` tokens;
    - **Raw identifiers in the reporting crates:** `r#match` as a function
      name, `r#if` as a binding and in a macro's tokens, `r#unused` in an
      attribute, a raw identifier in an inline test module, and `r#if` as a
      literal's name in the defining module;
    - **Blocks in generic arguments:** in type position, in an expression,
      after another argument, in a pattern, in another macro's tokens, as a
      const parameter's default and in a qualified type; and a construction
      in an array length of a `matches!` pattern's qualified path;
    - **Syntax `syn` does not model:** a `box` pattern and a
      `builtin # offset_of` expression;
    - **Passing cases:** ordinary generics, const arguments that are
      literals, comparisons, `<=`, shifts, nested generics before a block,
      match guards and closures in a reporting crate; raw string literals; a
      raw identifier in `src-tauri`; a type alias outside the defining
      module; and each listed macro's grammar in the defining module (named
      `format!` arguments, `vec![x; n]`, a trailing comma, a `matches!` guard,
      and a `json!` object with nested arrays and objects, an expression
      key, a parenthesized key and a negative value).
  - **Render command:** a pressure-free `local_private` input with a
    `user_release` member, and an exact `local_private` input with one, are
    each refused with `RENDER-USER-RELEASE-PACKAGE-ONLY`, with no body.
- `user_release_render_admits_only_the_user_released_class` (revision 4, §2.6), in
  the renderer and the Tauri render:
  - a user-released input whose provenance records are all the session
    provenance renders, with no `REPORT_PROVENANCE_CONTEXT_NOT_LOCAL_PRIVATE`;
  - the same input with one record of another non-public class is refused,
    and so is one with an `unknown` or `TBD` record (C1 b);
  - a `public_report` input with the session provenance is still refused, as
    at the parent head;
  - a positional `user_release` record is refused by the object-form check at
    the native render, the save and the producer (#1650).
- `user_release_display_rule` (revision 4, §2.6): a derivation vector whose S
  has `Pa` rows and rows with more than six significant digits. The derived
  rows show six significant digits and `MPa`, `conversion_performed` is true,
  and the row checksums are the full-precision digests. A stored value
  mutated in its seventh significant digit leaves the displayed row
  unchanged. The request is still refused, by the run record's row checksums
  (§2.5 step 8) and by the producer's `check_report_result_refs`.

**Owner property 1: the flag cannot leak private values.**
- vitest `redactionExportControls.test.ts`:
  - for every corpus item and each of the five existing contexts,
    `explicitUserReleaseIntent: true` throws, so it cannot enable export there;
  - payload-carried `user_release_intent` and `explicit_user_release_intent`, at
    the top level and nested, grant nothing: the `user_released_report` context
    without wrapper intent blocks (`USER_RELEASE_INTENT_REQUIRED`);
  - in `user_released_report`, each non-project private class is redacted,
    with every redistribution status, and the lossless route is withheld;
  - step order: for each of steps 10-13, a case that the existing order would
    decide in an unknown branch is decided by the new step;
  - the user-release projector admits each derived request of the vectors with
    intent, and blocks it without.
- Corpus parity: new cases in `cases.json`, checked by the vitest corpus test,
  `tests/security/test_redaction_export_controls.py` and the Rust mirror test
  `canonical_parity_corpus_matches_rust_contract_mirror`
  (`redaction_binding.rs:702`).
- Rust, Tauri (`report_package_save.rs` tests, sandbox directory, counting
  picker): on route 010, each of the following is refused before the picker,
  and the directory stays empty:
  - a request carrying `private_material_data` at depth;
  - each reference refusal above;
  - an unattested carrier, an evicted attestation and a mode mismatch;
  - a run record that fails each step-8 check;
  - an input manifest with a non-fixed setting or an external asset;
  - a request whose `user_rule_checks` or rule-pack reference is present;
  - a request with a handoff record set;
  - each case of `user_release_legacy_hanger_selection_refused`.
- Rust, Tauri, rule 3 (b)'s lease: the ingress lease grows by the largest
  candidate's priced parse before any embedded parse, and a counting-allocator
  test over a model with several large embedded candidates shows at most one
  embedded value held at a time, within that growth. A second
  counting-allocator test holds deep and wide embedded values (Codex round 1
  at #1673's shape: 1,000 `library_ref` members beneath 90 object levels with
  1,000-character keys; 124 such levels; 100,000 members in one list; each
  without a site, and with the site last). Its peak stays within the growth
  plus the reported site, and the same values as M allocate nothing beyond
  the site, in the private-class and rule-pack walks too.
- Rust, `report_package::user_release`: the derivation vectors pass. A
  mutation of each member of a derived request is refused, naming its pointer.
- Parity: the derivation vectors in vitest and in Rust.
- Attestation (`solve_worker.rs`, `solve_jobs.rs` and `lib.rs` tests):
  - the solved demo's `d_M` and `d_S` equal admission's digests over the
    session model and the received result, taken through the webview round
    trip;
  - the `ATST` frame's protocol violations fail the job;
  - a value the checked profile refuses sends status 0, and its result is not
    attested;
  - the table is bounded, evicts the oldest entry, is cleared by
    `dispose_all_results`, and is never written to disk;
  - the worker's counted peak with the digest passes stays within its grant
    (counting allocator), and the process-reserve measurement still holds;
  - allocation checks (counting allocator) of the streamed digest, in the
    worker and at admission, each on an invented value built to maximize one
    transient piece of §2.4: objects nested to the checked profile's depth
    limit, each with many members,
    so the key vectors on the open path are largest; one object with the most
    members, for the sort scratch; and one long string of characters that
    escape, for `write_string`; and one numeric-heavy value exercising
    exponential formatting. Each measured peak is within the §2.4 allocation
    account, including number-formatting temporaries, and within the grant or
    lease that holds it.

**Owner property 2: redaction applies to every other context.**
- The existing corpus cases are not edited. A vitest and a pytest pin the
  SHA-256 of their canonical form, so editing an existing expectation fails.
- Existing route tests for `DREP-PACKAGE-SAVE-009`, `DREP-IPC-003` and
  `DREP-HTML-SAVE-005` pass unchanged (`reportRedactionProjector.test.ts`,
  `renderedReport.test.tsx`, `reportPackageSaveService.test.ts`), and
  `existing_contexts_preserve_raw_intent_named_members` holds.
- Rust, `report_package`:
  `local_private_package_bytes_unchanged_by_user_release` assembles the
  invented local-private basis and compares its container SHA-256 with the
  value recorded from the parent head before (r2c) changes the writer.
- Renderer (`report_renderer/tests/render.rs`): a local-private and a public
  render are byte-identical to the parent head's for the same input.

**Owner property 3: the user release is recorded and visible.**
- Renderer: a user-released input shows the banner in the HTML and in the PDF text,
  with the bound hashes and `user_released_at`, and the run-context rows. It has no
  local-private banner and no `content_binding` text.
- `report_package` (new `tests/user_release_package.rs`):
  - the manifest's `user_release` equals the report context's plus a
    `content_binding` that matches the four members;
  - the four boundary notes are written;
  - no handoff member is written;
  - a round trip through the reader returns `ReadReportPackage.user_release`.
- vitest:
  - the dialog's wording and its refusal line;
  - its initial focus on the checkbox, set in `useLayoutEffect`;
  - the save stays disabled until the checkbox is ticked;
  - the status line and the receipt fields after a save.

**Owner property 4 and HELP_HUMAN: no replay.** One test per row of §2.10:
- native (`report_package_save.rs` tests): another model's text; another
  result document; another input manifest; another run record; a record
  on route 009; an altered statement, standing or `not_carried`; `model_ref` or
  `report_id` alone;
- producer (`tests/user_release_package.rs`): each bound member mismatched
  alone, with every other member coherent; a record under the desktop
  local-private and runner profiles; a handoff record under the user-released
  profile;
- reader (`tests/reader.rs`):
  - the record moved into another valid package and the package re-hashed;
  - `user_release_extra_member_refused`;
  - each member swap of `user_release_same_source_member_substitution_refused`;
  - a record in a blocked package;
  - the user-released profile without a record;
  - a package without a user release and without a handoff member;
  - as a documented limit, a package whose record, members,
    `content_binding` and inventory are all re-written together is accepted;
- render command (`src-tauri/src/lib.rs` tests) and renderer: a user-released input
  outside the package path;
- webview (`reportPackageSave.session.test.tsx`):
  - after a user-released save, the next Save Report Package… is local private
    with no record;
  - the project save payload, the model document text and the session state
    hold no trace of the selection;
  - a basis change while the dialog is open closes it.

**Also:**
- **Linter and dispositions:**
  - a user-released request with a quarantined provenance at depth, or a
    `protected_suspected` privacy class, is refused natively
    (`PROTECTED_CONTENT_BLOCKED`);
  - a user-released report whose section text trips the protected-content linter is
    refused (`REPORT-PACKAGE-PRODUCER-BLOCKED`);
  - the webview blocks the same in the new context.
- **Forged evidence:** route-010 evidence with `local_private`; route-009
  evidence over a user-released request; user-release intent false; evidence for other
  bytes; local-first evidence with another storage context.
- **Native save and read-back:** admitted user-released bytes written to the sandbox
  read back with `read_report_package_container`, with the record, its content
  binding and the member hashes intact.
- **Native smoke:** a user-released save on the solved demo, which carries no
  library or rule-pack reference, through `native_smoke::take_chosen_path`
  (beside the W-18 (b2) step, `native-smoke/run.mjs:242-248,337`). The harness
  checks:
  - the file's SHA-256 against the receipt;
  - the record's model hash against the session model's;
  - the HTML member's banner;
  - that the package has no handoff member.

  It adds no pinned value; `apps/desktop/native-smoke/expected.json` is not
  changed.
- **Claims lint:** `validate_claims_language.py` on each slice's head; the new
  product strings are in scanned `apps/desktop/src` files.
- **Codex scrutiny:** one independent round per slice on its implementation
  head, plus the Appendix A texts as exact text before HELP_HUMAN. Points for
  review:
  - the decision order (§3.2);
  - the completeness of the derivation and its vectors;
  - the attestation's trust, transport and bounds;
  - the library and rule-pack rules, the legacy form's recognition and the
    reliance limit;
  - the run-record checks;
  - the reader's equality-only checks and the content binding;
  - the naming rule (§0).

## 7. Decisions

**Settled.**
- **Scope (owner, 2026-10-25): R first** (§2.1). The later slice stays in v0.1
  (§10).
- **O-A (owner, 2026-10-25): V1**, with the DEC-028 reading recorded in §0 and
  applied in §4.
- **O-B (HELP_HUMAN, as recommended):** record `user_released_at`, and change the
  third boundary note for user-released packages only (§5.3).
- **O-C (HELP_HUMAN, as recommended):** the package only (§2.2).
- **O-D (HELP_HUMAN, as recommended): refuse.** Under R the refusal covers every
  model with a detected library origin, in each form §2.7 names. For values
  without one, the user release relies on the user's statement (the reliance
  limit, §2.7).
- **O-E: moot under R.** R carries no rule-check identifier. The question
  returns with the later slice (§10).
- **O-F (HELP_HUMAN, as recommended): exclude.** Under R that means no
  comparison section and no handoff record set, the persistence reference
  `TBD`, and assets `[]` (§2.1).
- **O-G (owner, 2026-10-25): equality only,** stated in §2.9 and in A8.
- **V1 covers R's package form (HELP_HUMAN, 2026-10-25, §0).** The reader at
  this head accepts what the writer at this head emits, so the user-released
  package has no state/comparison/handoff member and R keeps no handoff record
  set (§2.1, §4).
- **Rule-pack-referenced models are refused under R (HELP_HUMAN, 2026-10-25,
  §0).** `REPORT-USER-RELEASE-RULE-PACK-REFERENCED` (§2.7) is sequencing within
  the ruled scope. The later slice, which stays in v0.1, lifts it (§10).
- **Wording grant.** Appendix A and the product texts of §2.2 and §5 go to
  Codex as exact text and then to HELP_HUMAN.

**Where each review finding is resolved.**
- Round 1:
  - findings 1 and 3: the derivation and the refusals (§2.6, §2.7);
  - finding 2: the decision order (§3.2);
  - finding 4: the bindings, cardinality, content binding and the narrowed
    replay claim (§2.5, §2.9, §2.10);
  - finding 5: the strip is not widened (§3.3);
  - finding 6: the notice (§5.1);
  - finding 7: A13.
- Round 2's blocker: every model with a detected library origin is refused
  (§2.7). The
  should-fix items:
  - (a) the recorded-revision alternative is removed, since O-B stands as
    recommended. That alternative conflicted with the unchanged Current-export
    and producer checks that require `state:<model>:preview`
    (`resultExportAdapter.ts:278-279`; `bindings.rs:495`). Route 010 admits
    only the transient record (§2.5 step 8), so note 3's statement holds
    (§5.3);
  - (b) the derivation wording of A1 and A7, reduced to R;
  - (c) the library-value refusal appended to A7, as the refusal of every
    model with a detected library origin;
  - (d) A9's forbidden cell, replaced as worded.
- Round 3:
  - the blocker: the legacy hanger selection evidence is recognized (§1, §2.7
    rule 3), with its regressions (§6), and the reliance limit is stated in
    §0, §2.7, the dialog (§2.2) and the notice (§5.1);
  - the two HELP_HUMAN readings are recorded as settled in §0 and stated so in
    §2.7, §4 and the settled list above; neither is open;
  - the hanger rows: "dedicated per-object hanger quantity entries" are left
    out, and the solve's review rows are retained (§2.1, §2.6, §10);
  - A5's derivation clause, replaced as worded;
  - the digest passes' transient key vectors (§2.4) and their allocation
    checks (§6);
  - A7's library clause, replaced as worded, and its detector extended to the
    legacy form.
- Round 1 at revision 4 (Codex, `3dfc794601`):
  - B1, the runner and the user-released profile: every user-release marker
    is refused at the runner's `export-results` before the producer
    (`REPORT-USER-RELEASE-RUNNER-NOT-ADMITTED`) and by the producer's general
    entries (`REPORT-PACKAGE-USER-RELEASE-NOT-ADMITTED`). The one admitting
    entry, `user_release::assemble_derived`, performs the whole-request
    derivation itself, with no second admitted set. A reach scan pins its
    caller, and the render command also refuses a record (§2.8, "Who can
    reach a user-released package"). Every path to the renderer's
    user-released class rule is listed and closed there (§2.6). The
    regressions are `user_release_runner_refused` and the render-command
    case (§6), and the work is in (r2c) (§8, §9);
  - S1, outer ingress pricing: route 010 refuses exact texts in the outer
    command before they are sized, prescanned or priced, and the ordinary
    reservation is distinct from exact-text growth (§2.5, "The outer
    command", steps 2 and 2a). The regression goes through the outer command
    path (§6, `user_release_exact_route_refused`);
  - S2, malformed library sites: rule 1 counts a `library_ref` member
    whatever its value, by `library_ref_member_sites` and
    `libraryRefMemberSites`, and does not rely on the disposition screen
    (§2.7). A7's detector says "member, whatever its value" (Appendix A).
    The regressions are in §6, `user_release_library_reference_sites_refused`;
  - S3, A14's two codes: its New names the shareable package's code first,
    then "every other" surface's (Appendix A);
  - S4, the naming rule: every product text, identifier, record member and
    Appendix A draft now uses "user release", "user-released" or the "release
    … project data" phrase. That includes `user_released_at`,
    `scope.user_released_data_class`, the statement "…and I release my
    project data in this report for sharing.", "The user release covers" and
    the status line. `O-B` above records `user_released_at` by its new name;
    the decision is unchanged. Four kinds of use are kept:
    - HELP_HUMAN's quotation in §0, verbatim;
    - the naming rule's own statement of the words;
    - "software release", for the owner's act;
    - "release their own project data" and "release my project data", which
      are the allowed "release your project data" in another person.
- Round 2 at revision 4 (Codex, `1dce3ee2b2`; B1 and S1-S4 closed, the B1
  mechanism sound):
  - S5, the reach scan's coverage: the scan now reads the non-test source
    of `src-tauri`, `report_package`, `pdf_emitter` and the renderer. It pins
    each call of `render_calculation_report`, `assemble_report_sections`,
    `emit_pdf_document`, `pdf_from_render`, `emit_calculation_report_pdf` and
    `assemble_full_report_package` to its allowed callers, by function, each
    called once, with the path each must go through. Within `report_package`
    the one caller is the writer, behind refusal 2 or the admitting entry. It
    also pins the producer-entry argument at each producer call, and the one
    constructor of `DerivedUserRelease`. It rejects aliases, `use` and
    `pub use` items, module and glob re-exports, function values, closures
    and macro bodies. The §2.8 table's row now claims what the scan
    establishes, and the scan's limit is stated (§2.8, "The reach scan").
    The negative fixtures are in §6 (`user_release_runner_refused`, "Reach
    scan");
  - S6, derivation-first ordering: `assemble_derived` now runs a preflight
    before the derivation. Item 0 runs through the public
    `check_route_admitted`, then `check_user_released_profile` runs with the
    profile constant. So an exact document gets
    `REPORT-USER-RELEASE-EXACT-ROUTE-NOT-ADMITTED` whatever the report
    carries. At the general entries, the marker refusal comes before the
    user-released exact check, so they give
    `REPORT-PACKAGE-USER-RELEASE-NOT-ADMITTED`, with the stated exceptions
    for item 0 and W-11's runner-profile check, and the reason for that
    order (§2.8, "exact routes" and "Precedence for an exact document"). The
    regression calls the public entry with an exact document and a report
    that differs from the derivation (§6, `user_release_exact_route_refused`,
    producer). A1 is unchanged. Its clause "an exact-route result is refused
    on that route … by the shared producer
    (`REPORT-USER-RELEASE-EXACT-ROUTE-NOT-ADMITTED`)" concerns route 010,
    whose only producer entry is `assemble_derived`. That entry now gives
    this code for every exact document, whatever the report carries, so the
    clause is exactly true;
  - N1, the runner's message: the exact proposed message ends
    `(marker: <member>)`. `<member>` is one of three fixed paths:
    `export_profile_id`, `report.report_context.export_context` and
    `report.report_context.user_release`. If more than one is present, the
    first in that order is named (§2.8, refusal 1).
- Round 3 at revision 4 (Codex, `80b8244391`; no blocking, S6 and N1 closed,
  A1-A14 pass):
  - S5a, lexical scope: the reach scan no longer uses
    `forwarding_graph_scan`'s indentation rule. It parses each file with
    `syn` and attributes every occurrence of a scanned name to its innermost
    scope owner in the tree: a function, a method, a closure, an `async`
    block, a `const` or `static`, or a macro's tokens. An allowed call must
    be a direct call whose scope owner is the allowed caller. Any other
    occurrence is a finding, in any position, macro tokens included. So a
    module-level macro after the writer, a wrapper that invokes it, a closure
    at the caller's indentation, a trait object, a generic or `impl Fn`
    argument, and a method on a wrapper each fail. The producer's internal
    entries are pinned by the reach scan's own rows, not by W-11's line scan.
    The §2.8 table's row now claims what the parsed scan establishes. The
    stated limit is kept, and it now names macro expansion (§2.8, "The reach
    scan").
  - S5b, collection: the scan resolves modules itself from the crate roots.
    Only an item marked exactly `#[cfg(test)]` is test-only, and a file that
    any production declaration loads is production. A production `#[path]`,
    a `path` in a `cfg_attr`, any `include!`, and an `include_str!` or
    `include_bytes!` of a `.rs` file or of a non-literal are findings. Each
    form was checked at the basis, and none needs an allow-list (§2.8, "What
    the four crates hold today").
  - The parser choice: `syn` 2.0.117 and `proc-macro2` 1.0.106 are already in
    `src-tauri`'s lock, with their dependency `unicode-ident`, and in this
    host's registry cache. So they are added as pinned dev-dependencies with
    no new lock entry, and that is to be confirmed offline in (r2c). The
    restricted-syntax lexer was not chosen, because it would need an
    allow-list at the basis and a hand-written scope tracker (§2.8,
    "Parser").
  - The fixtures for each counterexample are in §6 (`user_release_runner_refused`,
    "Reach scan").
  - N2, the general entries' precedence: at `wire::assemble_wire_request`,
    `assemble_wire_request_in` runs item 0, the report's object-form check
    and the audit-manifest conversion before `check_request_in`. So a marker
    request with a positional report gets the positional-form diagnostic
    first. The precedence paragraph now applies to the requests that reach
    `check_request_in`, and keeps those earlier checks (§2.8, refusal 2).
  - Appendix A: every Old was re-checked with `count == 1` at `2e4748eada`.
    All fourteen count 1 at their cited lines, and no Old or New changed.
- Round 4 at revision 4 (Codex, `3d2f35911b`; no blocking, A1-A14 pass; the
  parsed ownership, cfg handling, `syn` features and versions and §2.4's
  `d_S` basis confirmed):
  - S5c, a macro that emits a `mod`, and S5d, a macro that builds a call path
    from parameters: the reach scan now fails closed on macros instead of
    adding one more token pattern. In the four crates' production source a
    `macro_rules!` or `macro` item is a finding unless it is on an allow-list
    pinned by the SHA-256 of its source text. The list has only
    `app_invoke_handler` (`lib.rs:5018-5081`), which holds no `mod`, no `$`
    parameter before `::` and no matched name; a search of the basis found
    no other production definition. A function-like invocation, attribute or
    derive is a finding unless it is on a list derived from the basis
    production source, and imports cannot rebind a listed name. So no macro
    can emit a `mod` the traversal misses, and none can build a path (§2.8,
    *Macros*).
  - N4, external declarative macros: only the listed ones may be invoked
    (`json!`, `params!`, `forward_to_deserialize_any!`), and each emits code
    made of its own tokens and its crate's internals. The stated limit now
    names only what remains: a listed external macro, procedural or
    declarative, whose version emitted an unnamed call. None of today's
    does, and changing one of those dependencies is reviewed as a change to
    this boundary (§2.8, "What the scan does not establish").
  - The fixtures: S5c's `load_escape!`, S5d's parameterized path, an
    unlisted external macro (also imported under a listed name), a new local
    macro, a changed `app_invoke_handler`, unlisted attributes, `serde` keys
    and derives, and passing cases for every listed macro (§6, "Reach scan").
  - N2a, precedence inside `check_request_in`: its two cardinality checks
    (`bindings.rs:361-365`) run before item 0 and the marker refusal. A
    marker request that fails one gets that diagnostic, and the precedence
    now applies to requests that pass them (§2.8, refusal 2).
  - N3, §2.4's reason for the worker: the worker holds the model value and
    the delivered result bytes, and parses the bytes back into one transient
    copy for `d_S`, inside the result text's ingress bound.
  - Appendix A: every Old was re-checked with `count == 1` at `2e4748eada`.
    All fourteen count 1 at their cited lines, and no Old or New changed.

**Open.** None. Every decision this design rests on is settled.

## 8. Slices and size

| Slice | Content | Size | Persistence / data boundary |
|---|---|---|---|
| **(r1) Vocabulary and redaction** | The context, reason codes and route id in TypeScript, Python, the Rust mirror and the schema; the decision order of §3.2 (steps 10-13); the flag assertion; the local-first rule; the user-release projector (nearest-metadata labelling; unused until r3); corpus cases, the pinned-digest tests and `existing_contexts_preserve_raw_intent_named_members`. No route uses the context. | M (~650 lines with tests) | Data boundary (vocabulary only) |
| **(r2a) Solve attestation** | The worker's streamed digests, the `ATST` frame, the registry table, clearing and bounds, the memory-budget text with the digest passes' transient pieces (§2.4), its measurement and the allocation checks, tests. No rule-check attestation. | S (~450) | Data boundary; DEL-04-07 memory accounting; no priced file |
| **(r2b) Derivation and refusals** | `report_package::user_release`: Rust builders of the input manifest, the document base and origin, the minimal renderer input, and the audit and top-level members; the library rules 1-3 of §2.7, including the legacy hanger form, its embedded-JSON parse and lease, in `src-tauri` beside `export_disposition_screen`; the rule-pack and private-class rules (library rule 1 counting a non-object `library_ref` by its own detector at both layers, `library_ref_member_sites` and `libraryRefMemberSites`); the run-record checks (Record Run's, factored); the derivation's use of the shared display, row-checksum, unit-summary and diagnostics functions (§2.6); the derivation vectors and their vitest and Rust parity, `user_release_display_rule` among them; `user_release_legacy_hanger_selection_refused`. | M (~1.07k) | **Data boundary** (what may be shared) |
| **(r2c) Record, renderer, producer, writer, reader, native** | `UserReleaseRecord`; context and blocking codes; the renderer's user-released class rule for provenance records (§2.6); notice and rows in HTML and PDF; producer items 8 and 10, the profile, the handoff rule and the exact-route refusal after item 0; writer manifest, content binding, notes and member rule; reader checks, cardinality, the pressure-free-only check and refusal; native route-010 branch (§2.5 steps 1-10, step 2a included) with the run-record text and the outer command's route-aware `ingress_lease_bytes`; the render-command refusal of the context and of a record; the user-release markers' refusals in the runner's `export-results` and the producer's general entries, the one admitting entry `user_release::assemble_derived` with its bridge function, and `user_release_reach_scan` with its pinned `syn` and `proc-macro2` dev-dependencies, its macro, attribute, derive and import rules and the `app_invoke_handler` pin (§2.8); the Rust tests of §6, `user_release_exact_route_refused`, `user_release_runner_refused` (with its CLI pytest) and `user_release_render_admits_only_the_user_released_class` among them. The desktop does not offer it yet. | M-L (~1.84k) | **Persistence** (manifest, member set and reader) and **data boundary** (native admission) |
| **(r3) Desktop** | Dialog, panel button, menu item (webview and native menu), session wiring and modal guard, the route-010 builders, the user-release cell and the dialog's exact-route line (§2.2), the early reference check (the three rules of §2.7), the dialog's reliance line, save-service evidence and the run-record text, receipt display, vitest (the webview cases of `user_release_exact_route_refused` included), native smoke step. | M (~930) | Data boundary (webview evidence) and file output |

The order is (r1), then (r2a) and (r2b) in either order or together, then
(r2c), then (r3). (r2c) needs (r2a) and (r2b) in its ancestry; (r3) offers the
user release and needs (r2c). (r2c) and (r3) may land as one PR if review
prefers. The total is about 4.94k lines (4.1k at revision 2; the legacy
detector, its regressions and the allocation checks add about 250; revision 4's
exact-route refusal, the renderer's user-released class rule and their regressions
add about 110; Codex round 1 at revision 4 adds about 140 for the runner and
producer refusals, the reach scan, the outer command's pricing and the
non-object `library_ref` detectors; Codex round 2 at revision 4 adds about
80 for the reach scan's extension, its negative fixtures and the admitting
entry's preflight; Codex round 3 at revision 4 adds about 150 for the parsed
reach scan's module resolution, scope attribution and token screening, the
two dev-dependencies' lock edges and the added fixtures; Codex round 4 at
revision 4 adds about 90 for the macro, attribute, derive and import rules,
the `app_invoke_handler` pin and their fixtures), down from about 5.8k for revision 1's
full scope. The savings are:
- the builder mirrors (no component, hanger or rule-pack entries, no handoff
  sections, no analysis-record rebuild);
- the closed table;
- the rule-check attestation.

## 9. Reconciliation, per slice

Each text changes in the PR of the slice that makes it true. Commitment,
acceptance and ScopeOfWork texts are applied only as Appendix A's reviewed
exact text.
- **(r1)**
  - DEL-12-02 `ScopeOfWork.md` :58, the context vocabulary (A3).
  - `docs/security/redaction_export_controls.md`:
    - the Export Contexts table (:59-65) gains the `user_released_report` row,
      with its decision order;
    - the Final-Sink Route Binding section (:158-163) says that user-release intent
      comes only from the wrapper, and that the strip list is unchanged.
  - DEL-12-02 `deliverable.yaml` check pointers; `tools/hosted-ci-routing.json`
    for the changed sources and the tests.
- **(r2a)**
  - The `solve_worker.rs` protocol text (:10-25) names `ATST`; the
    `solve_jobs.rs` module text (:1-55) names the attestation table beside
    the custody digests (§2.4).
  - DEL-04-07 `Design/memory-budget-decision.md` :37, the worker paragraph,
    names the frame, the digest passes and their transient pieces (the key
    vectors on the open path, one sort scratch, one escaped string; §2.4);
    DEL-04-07 `deliverable.yaml`
    checks; CI routing for the Tauri sources and tests.
- **(r2b)**
  - DEL-08-01 `deliverable.yaml` code paths (`user_release` module and the
    library rules) and checks (the derivation vectors, the legacy-form
    regressions); CI routing.
  - `core/library_import/README.md` :49-55 still describes the pre-W-17 hanger
    provenance as what selection persists. It is changed to name the W-17 form
    (`library_ref` and the `library:hanger:` string) as current, and the
    legacy form as one that models saved before `00af9112c0` may still carry,
    which route 010 refuses (§2.7).
- **(r2c)**
  - DEL-02-05 `ScopeOfWork.md` :214, the reader (A8).
  - DEL-12-02 REXC-OI-007 :486, the native checks (A7), and CLM-030 :414 (A5;
    see its note).
  - DEL-08-01 `deliverable.yaml` checks (new `tests/user_release_package.rs`);
    the `reader.rs` module text (:1-42), with the §2.9 statement of what the
    checks establish; CI routing.
  - The producer's module texts say which entry admits what (§2.8):
    `bindings.rs:1-4` and `wire.rs:1-10` name the general entries and the
    one admitting entry, and `admitted_tests.rs:1-21` names its added root.
  - DEL-10-05 `deliverable.yaml` check pointers (`user_release_runner_refused`
    and its CLI pytest). The runner's comment beside its W-11 refusal
    (`openpipestress-runner.rs:496-502`) names the user-release refusal. Its
    `ScopeOfWork.md` does not change: its commitments name the verbs, not
    the export profiles (`:61,109`).
  - DEL-08-01 Design `report-package-current-results.md` keeps its text; it
    carries only a pointer to this design (`:1536`).
  - DEL-08-01 Design `exact-route-report-sections.md` keeps its text. Its §0
    ruling "C1 b" already names the user-released report as a context where an
    unresolved record blocks, and §2.6 applies that.
- **(r3)**
  - DEL-08-01 `ScopeOfWork.md` :81 (A1) and OQ-08-01-003 :383 (A2).
  - DEL-12-02 REXC-REQ-016 after :189 (A4), REXC-OI-002 :481 (A6), CLM-036
    :470 (A9).
  - DEL-04-08 REQ-006 (c) :219 (A14, new in revision 4).
  - User guide `docs/user_guide/index.md` :92 (A10), :113-115 (A11), :420
    (A13) and :547-548 (A12).
    - The §10 sentence's "renders a report only as a local private document"
      would mislead once a shareable package exists, so A13 names the
      standalone report.
    - Its table row (:152) stays true: the local-private package is still
      saved locally, and the row's exact-route clause concerns the
      local-private surfaces.
  - `docs/COMPLETENESS_MAP.md` records W-18b (R) and the later slice: gap 1
    (:96-104), FR-REP-001 (:209), the not-ready list (:470-473) and the W-18
    row (:559). Each states that shareable packages carry pressure-free
    results only, and that exact-route sharing returns to the owner (§10).
  - DEL-04-07 Design `desktop-exact-outputs.md` §4.5 (:203-211), the
    report-package bullet: the shareable package refuses exact-route results
    (`REPORT-USER-RELEASE-EXACT-ROUTE-NOT-ADMITTED`). This is a design text, so
    the slice edits it directly.
  - `apps/desktop/native-smoke/run.mjs`, the user-released step (§6).
  - DEL-08-01 `deliverable.yaml` code paths (`UserReleaseDialog.tsx`) and
    checks.
- **No change:**
  - DEL-04-08 AC-006 :240 stays true: the package's Rust boundary still
    refuses every identity except `preview-physics-2`, `pressure-2` and
    `pressure-source-1`, and the user-released profile's exact-route refusal is
    a profile check after it. Revision 3 listed REQ-006 (c) :219 here too.
    Since the W-11 opening (O1, #1648), it says the report package admits
    exact-route results and that every other surface refuses with
    `PRESSURE-2-OUTPUT-NOT-YET-AVAILABLE`. Neither describes the shareable
    package, so A14 adds it;
  - the DEL-08-02 audit manifest, and its ScopeOfWork;
  - the PRD §19.3 notice, and `docs/report_notice_template.md`;
  - no priced source (§2.4).

## 10. Later slice on the same mechanism (stays in v0.1)

The owner kept these in v0.1 (§0). Each extends R on the same context, route,
record, attestation, derivation and reader. None is designed here beyond its
constraints.
- **Rule outcomes at safe metadata.** It needs:
  - a rule-check attestation recorded by `run_rule_checks`
    (`src-tauri/src/lib.rs:2905`), of the safe projection of the pack reference
    and the outcomes;
  - the derivation's renderer input carrying that projection;
  - the run record's composed status admitted.

  It assumes W-17's per-basis shape (#1629). A check has one outcome per load
  case or combination its stress category maps, `UserRuleCheckOutcome` carries
  an optional `basis_ref` (`report_renderer/src/lib.rs:132-165`), and a check id
  repeats once per basis. The attestation and the derivation therefore key each
  outcome by (check id, basis), and bind the outcome count. The renderer's count
  note (`USER_RULE_CHECK_COUNT_SCOPE_NOTE`, `:163-165`) stays. Whether a
  user-released outcome may omit `basis_ref` ("load case not recorded") is that
  slice's question; the desktop always writes it
  (`renderableReportInput.ts:732-755`). O-E's question returns with it: whether
  check ids and missing-input names are safe metadata.
- **Rule-pack references at safe metadata**, with the agreement rule, which
  lifts the rule-pack-reference refusal of §2.7.
- **Library-referenced models.** It needs a rule for library values that
  genuine solver output carries (§1): refuse, or mark them, in result rows,
  annotations, diagnostics, witnesses and contract evidence. It also needs
  revision 1's source resolution for report entries. Since W-11 (c) the
  local-private report lists the model's library references at safe metadata
  (`library_references`, W-11 design §4.2). That slice decides whether the
  list is carried, and it is subject to the ruling below.
- **Comparisons** (FR-REP-004), by native derivation from attested results (O-F's
  alternative), and the handoff record set's state/run sections.
- **The report entries R leaves out:** component and spring-hanger provenance
  entries, the material, section and support provenance entries of W-11 (c),
  and dedicated per-object hanger quantity entries. The solve's review rows,
  hanger rows included, are already in R (§2.6).
  - **The owner's ruling "C2 a" (#1639, 2026-10-26)** says that the user's
    authored record text "stays as written in local-private outputs; no other
    context carries these records".
  - So carrying the provenance records under `user_released_report` needs
    the owner's ruling. That slice cannot settle it within R's mechanism.
    "C1 b" keeps unresolved records report-blocking there in any case.
- **The record:** a new `scope.not_carried` value and `record_version` for each
  widening. The reader admits each version it writes.

**Returns to the owner (not in the list above): exact-route results under user
release.** Under R, route 010 refuses `pressure-2` and `pressure-source-1`
results (§0). Admitting them is not part of the later slice the owner kept in
v0.1. HELP_HUMAN, 2026-10-26: "Opening a surface to sharing is the owner's
call, as the local-private opening was, so don't build it without that." It is
not built without the owner's decision. If the owner opens it, route 010 would
need these, on the same record, attestation and reader:
- W-11's exact source binding (W-11 design §3.6): custody, the resubmitted
  delivered and intake texts, and the native reader's eligible standing;
- the exact sections in the derivation (the `exact_route` block, the exact
  limitation entries and the headlines, which the display rule covers);
- the Diagnostics binding (`check_report_diagnostics`, #1659);
- the producer's item 9 under the user-released profile.

Lifting the refusal of §2.2, §2.5 step 2a, §2.8 and §2.9 would then be that
change's edit.

## Appendix A. Wording-route drafts (old → new; not applied)

Each Old is copied byte-exact from its file. At revision 4 each was re-checked
with a Python `count == 1` against main `2e4748eada`, the design's basis, and
again after Codex rounds 1, 2, 3 and 4 at revision 4: all fourteen count 1, at the lines
below. Line
numbers are where the Old begins. In an Old or New, `\n` stands for a line break
and `\"` for a quotation mark inside the quoted text. These drafts go to Codex
as exact text, then to HELP_HUMAN under the wording grant. Nothing here is
applied.

**Revision 4's re-check.** Between revision 3's tree (`3027cf2f35`) and
`2e4748eada`, #1648's opening texts (W-11 design Appendix A.2, O1-O15) changed
three of the target files. O6 consumed A1's Old, O9 consumed A10's, O11
consumed A11's and O15 consumed A12's. O12 rewrote the sentence around A13's
Old, and the other guide edits since revision 3 (#1639, #1640, #1644, #1652)
moved lines. At `2e4748eada`:

| Draft | Revision 3 Old, count | Revision 4 Old, count and line | Revision 4 |
|---|---|---|---|
| A1 | 0 | 1, :81 | changed: re-derived |
| A2 | 1 | 1, :383 | unchanged |
| A3 | 1 | 1, :58 | unchanged |
| A4 | 1 | 1, :189 | unchanged |
| A5 | 1 | 1, :414 | unchanged |
| A6 | 1 | 1, :481 | unchanged |
| A7 | 1 | 1, :486 | unchanged |
| A8 | 1 | 1, :214 | unchanged |
| A9 | 1 | 1, :470 | unchanged |
| A10 | 0 | 1, :92 | changed: re-derived |
| A11 | 0 | 1, :113 | changed: re-derived |
| A12 | 0 | 1, :547 | changed: re-derived |
| A13 | 1 | 1, :420 | unchanged; its line moved from :393 |
| A14 | — | 1, :219 | new in revision 4 |

Each unchanged New was read again against R as amended by §0. None of them
says or implies that a shareable package carries an exact-route result, so
each is still exactly true once its slice lands. The re-derived and new News
say that route 010 refuses exact-route results, and they no longer say that
reports are pressure-free only.

**Depends on** names what a New needs before it is applied:
- the slice that makes it true;
- the decisions it rests on, each **settled** (§0, §7), or **none**.

Every decision these texts rest on is settled. "Exact routes under R" is the
§0 decision that HELP_HUMAN confirmed on 2026-10-26. Each New is written to be
exactly true of scope R once its slice lands.

**Changed in revision 4:** A1, A10, A11 and A12 (re-derived), and A14 (new).
**Unchanged in revision 4:** A2, A3, A4, A5, A6, A7, A8, A9 and A13 (A13 has
moved line). At round 3, A5 and A7 changed.

**Changed after Codex round 1 at revision 4.** Each Old is unchanged; only
the News changed.
- The naming rule (S4) changed the News of A1, A2, A4, A5, A7, A9 and A11,
  and A5's note.
- A7's detector now reads "a `library_ref` member at any depth, whatever its
  value" and "a `library_ref` member or a string" (S2).
- A14's New was reordered (S3).

**After Codex round 2 at revision 4.** No Old or New changed. A1's producer
clause stays exactly true once the admitting entry's preflight is in place
(§7, S6).

**After Codex round 3 at revision 4.** No Old or New changed. All fourteen
Olds were counted again at `2e4748eada`, and each counts 1 at its cited line
(§7).

**After Codex round 4 at revision 4.** No Old or New changed. All fourteen
Olds were counted again at `2e4748eada`, and each counts 1 at its cited line
(§7).

**A1. DEL-08-01 `ScopeOfWork.md` :81** (current-implementation paragraph, last sentence; slice r3; depends on: scope R, O-D, O-F, exact routes under R, settled) — *changed in revision 4*
- Old: "Every other identity is refused."
- New: "Every other identity is refused. Save Shareable Report Package… saves a reduced package for sharing, for a Current pressure-free (preview-physics-2) result only, after the user's per-report user release of their own project data (route `DREP-PACKAGE-USER-RELEASE-010`, export context `user_released_report`; DEL-08-01 Design `report-user-release.md`): the user-release record in `report_context.user_release` and in the package manifest states the user's redistribution right and binds the model reference and document SHA-256, the run's input manifest and the result document, and the manifest copy also binds the hash of every other member; native admission requires every request member to equal its derivation from the bound model, its attested solve, the input manifest, the bound run, the host software identity and the user-release record; the package carries the rendered report, the audit manifest and one result member, and no user-rule outcome, rule-pack reference, run comparison or handoff record; an exact-route result is refused on that route in the desktop, at native admission and by the shared producer (`REPORT-USER-RELEASE-EXACT-ROUTE-NOT-ADMITTED`), and so is a model that carries a library reference or a rule-pack reference; and the rendered members carry the user-release notice in place of the local-private notice. Without that selection the package is local private as above, and the selection is not stored."

**A2. DEL-08-01 `ScopeOfWork.md` :383** (OQ-08-01-003, last cell; slice r3; depends on: scope R, settled)
- Old: "Preserve route-specific controlled output, explicit intent, source non-mutation and fail-closed behavior; this does not supply legal/privacy acceptance. |"
- New: "Preserve route-specific controlled output, explicit intent, source non-mutation and fail-closed behavior; this does not supply legal/privacy acceptance. A shared report carries private project data only through the user's per-report user release of their own data, and only as values derived natively from the bound model, its solve in this session and its run (DEL-12-02 `user_released_report`; DEL-08-01 Design `report-user-release.md`). |"

**A3. DEL-12-02 `ScopeOfWork.md` :58** (CLM-003; slice r1; depends on: none)
- Old: "| Export context vocabulary | `public_report`, `public_example`, `shared_model`, `downstream_tool`, `local_private` |"
- New: "| Export context vocabulary | `public_report`, `public_example`, `shared_model`, `downstream_tool`, `local_private`, `user_released_report` |"

**A4. DEL-12-02 `ScopeOfWork.md` :189** (CLM-012; a row added after REXC-REQ-015; slice r3; depends on: scope R, O-D, settled)
- Old: "> | REXC-REQ-015 | Any unresolved config schema, UI control, export format, public API transport, or physical project package/container choice shall remain `TBD` until resolved through an authorized implementation or architecture decision. | AB-00-04; AB-00-07; OPS-K-AGENT-1 | Review this deliverable for explicit TBD/open issue entries. |"
- New: the Old line unchanged, followed by this line:
  "> | REXC-REQ-016 | A shareable report may include the user's own private project data only after the user's explicit user release of that data for that one report, with the user's statement that they hold the redistribution right (`docs/IP_AND_DATA_BOUNDARY.md` §6). The export context is `user_released_report`; the user release is recorded in the report and its package, bound to the model and result it covers, and stated in the report's notice; it is not stored for later reports; a model that carries a library or rule-pack reference is refused, and no user-rule outcome or run comparison is carried; protected, quarantined and rejected content still blocks; every other export context is unchanged. | OPS-K-PRIV-1; `docs/IP_AND_DATA_BOUNDARY.md` §6; owner, 2026-10-24 (W-18 option B); owner, 2026-10-25 (W-18b scope R) | Focused tests: the user-release flag cannot enable export in another context; redaction is unchanged in every other context; the record is visible in the report and the package; an unchanged record with a mismatched bound model, result or package identity, or with member content other than the content it binds, is refused; a model with a library or rule-pack reference is refused. |"

**A5. DEL-12-02 `ScopeOfWork.md` :414** (CLM-030, last sentence; slice r2c; depends on: scope R, O-D, settled) — *changed since round 3*
- Note: CLM-030 states context policy, but its last clause ("the user release is recorded in the report") is true only once the recording path exists, so A5 is applied in (r2c), not with the vocabulary in (r1).
- Old: "`downstream_tool` may need values for technical continuity, but the adapter must\n> still preserve privacy, provenance, unit, and diagnostic checks."
- New: "`downstream_tool` may need values for technical continuity, but the adapter must\n> still preserve privacy, provenance, unit, and diagnostic checks.\n> `user_released_report` carries the user's own project data in one report only\n> after the user's explicit user release of it, with their statement of the\n> redistribution right, and only as values derived from the bound model, its\n> attested solve and its run; a model that carries a library or rule-pack\n> reference is refused, no user-rule outcome or run comparison is carried,\n> content outside the permitted source-bound projection is refused, protected,\n> quarantined and rejected content still blocks, and the user release is\n> recorded in the report."

**A6. DEL-12-02 `ScopeOfWork.md` :481** (REXC-OI-002; slice r3; depends on: none)
- Old: "Current desktop/local CLI wrappers own fixed route context and explicit local-private intent."
- New: "Current desktop/local CLI wrappers own fixed route context and explicit local-private intent. The desktop's per-report user release (`DREP-PACKAGE-USER-RELEASE-010`, `user_released_report`) is selected in its own dialog for one save and is not stored."

**A7. DEL-12-02 `ScopeOfWork.md` :486** (REXC-OI-007; slice r2c; depends on: scope R, O-D, O-F, settled) — *changed since round 3*
- Old: "before the save dialog; caller evidence alone admits nothing."
- New: "before the save dialog; caller evidence alone admits nothing. On its user-release route (`DREP-PACKAGE-USER-RELEASE-010`) it admits the `user_released_report` context only with a user-release record whose bound model, input-manifest and result-document identities equal the texts and members it received, for a result this session's solver produced for that model, and only when every request member equals its own derivation from the bound model, its attested solve, the input manifest, the bound run, the host software identity and the user-release record; it refuses a record on the local-private route, a rule-pack reference, a user-rule outcome, a comparison section, a handoff record set and any request object with a non-project private class. It also refuses every model that carries a library reference, so no value with a detected library origin reaches result rows, annotations, diagnostics, witnesses or contract evidence. A library reference is a `library_ref` member at any depth, whatever its value; a string value that contains `library:material:`, `library:section:`, `library:component:` or `library:hanger:`; or a string value that holds the legacy hanger selection evidence, being one that contains `user_imported_hanger_selection`, or one that, trimmed of JSON whitespace, begins with `{` and parses as a JSON object that has a `library_kind`, `library_id`, `library_metadata` or `record_snapshot` member or within which a `library_ref` member or a string containing one of those five texts occurs; a string so beginning that admission cannot check is refused too. For a value without a detected library origin, the user release relies on the user's statement of their redistribution right."

**A8. DEL-02-05 `ScopeOfWork.md` :214** (reader; slice r2c; depends on: O-A, O-G, settled)
- Old: "for an unblocked package, the admitted identity plus equality of the audit-manifest id and model-hash value;"
- New: "for an unblocked package, the admitted identity plus equality of the audit-manifest id and model-hash value; when the manifest records a user release, it checks the record's form, exactly one result member and no state/comparison/handoff member, the equality of its bound model reference and hash, input-manifest hash, result-document identity, report and package identities with the manifest, audit and result members, and the member hashes the record binds, and refuses a record in a blocked package or under another export profile (equality checks: they establish consistency between the record and the members, not authentic user consent or producer origin);"

**A9. DEL-12-02 `ScopeOfWork.md` :470** (CLM-036, a row added after "Local private report"; slice r3; depends on: scope R, settled)
- Old: "> | Local private report | Unredacted values only after explicit local-private export selection and warning/audit record | Silent unredacted export to a shareable/public location |"
- New: the Old line unchanged, followed by this line:
  "> | User-released report | The user's own project values in one report after the explicit per-report user release, derived natively from the bound model, its attested solve and its run, recorded in the report and stated in its notice; no user-rule outcome, rule-pack reference, library reference, run comparison or handoff record | An unchanged user-release record whose bound identities or member hashes do not match this report, a selection remembered for later reports, or content outside the permitted source-bound projection |"

**A10. User guide `docs/user_guide/index.md` :92** (slice r3; depends on: exact routes under R, settled) — *changed in revision 4*
- Old: "- **Reports are local-private only.**"
- New: "- **Reports are local-private only, except a shareable package of a pressure-free result saved with your per-report statement of rights to share your own project data.**"
- Note: at revision 3 the Old read "- **Reports are local-private only, and pressure-free only.**", and #1648's O9 replaced it. Reports are no longer pressure-free only, so the New no longer says so.

**A11. User guide `docs/user_guide/index.md` :113** (a bullet added; slice r3; depends on: scope R, O-D, O-F, exact routes under R, settled) — *changed in revision 4*
- Old: "  - Save Report Package… saves a local private package for a pressure-free\n    result, or for an exact-route result that Export Result JSON admits,\n    after its own local-private selection."
- New: the Old bullet unchanged, followed by:
  "\n  - Save Shareable Report Package… saves a reduced package for sharing after\n    you tick, in its dialog, \"I hold the right to share this project's data,\n    and I release my project data in this report for sharing.\" The user\n    release covers that one report. Its documents carry a \"USER-RELEASED\n    REPORT\" notice naming the model and result it covers. The package carries\n    the model summary, load cases and results of this session's solve. It\n    carries no rule-pack reference, user-rule outcome or run comparison. It\n    needs a pressure-free result solved in this session; an exact-route result\n    is refused with `REPORT-USER-RELEASE-EXACT-ROUTE-NOT-ADMITTED`. A model\n    with a library reference, such as a material, section, component or hanger\n    applied from a library, or with a rule-pack reference, is refused. The\n    selection applies only to this save."
- Note: the Old is #1648's O11 New. The added bullet is revision 3's, with "It needs a result solved in this session." replaced by the pressure-free and exact-route sentence.

**A12. User guide `docs/user_guide/index.md` :547** (limitations list; slice r3; depends on: exact routes under R, settled) — *changed in revision 4*
- Old: "[Current Desktop App](#current-desktop-app-010): reports only as local private\n  documents, and the report JSON packet only for pressure-free results,"
- New: "[Current Desktop App](#current-desktop-app-010): reports only as local private\n  documents or as shareable packages of pressure-free results with your per-report statement of rights to share your own project data, and the report JSON packet only for pressure-free results,"
- Note: the Old is #1648's O15 New. The line break after "local private" is the Old's own, kept so the list item's wrapping is unchanged.

**A13. User guide `docs/user_guide/index.md` :420** (§10, first sentence; slice r3; depends on: O-C, settled) — *unchanged in revision 4; its line moved from :393*
- Old: "renders a report only as a local private document"
- New: "renders the standalone report only as a local private document"
- Note: the sentence now continues "…, after the local-private selection, for a pressure-free result or an exact-route result that Export Result JSON admits, and the report package is saved locally for the same results" (#1648's O12). With the New, it describes the standalone report and the local-private package, and both clauses stay true under R.

**A14. DEL-04-08 `ScopeOfWork.md` :219** (REQ-006 (c), first bullet; slice r3; depends on: scope R, exact routes under R, settled) — *new in revision 4*
- Old: "Every other desktop output surface refuses it (`PRESSURE-2-OUTPUT-NOT-YET-AVAILABLE`)."
- New: "Save Shareable Report Package… (DEL-08-01 Design `report-user-release.md`) refuses it (`REPORT-USER-RELEASE-EXACT-ROUTE-NOT-ADMITTED`). Every other desktop output surface refuses it (`PRESSURE-2-OUTPUT-NOT-YET-AVAILABLE`)."
- Note: since #1648's O1 the bullet says that "the report package" admits an exact-route result. A14 says what the shareable package does, which is the consequence of the §0 decision. The shareable package's refusal comes first, so "every other" surface then excludes it and each surface has one code (Codex round 1 at revision 4, S3). The Old is unchanged. This is commitment text, so it goes through the wording grant like every other draft.
