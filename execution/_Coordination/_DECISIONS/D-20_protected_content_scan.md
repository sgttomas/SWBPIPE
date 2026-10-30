# D-20 - Release-Artifact Protected-Content Scan Ownership And Procedure (RGAP-005)

**Date prepared:** 2026-07-04
**Prepared by:** bridge work loop agent at owner direction (Ryan Tufts,
2026-07-04) preparing all eight open register rows.
**Epistemic status:** PROPOSAL (non-governing). Only the human project
authority rules. Nothing here changes lifecycle state, issues deliverables,
creates release readiness, professional approval, certification, sealing,
authentication, code-compliance acceptance, or asserts that any PRD milestone
is met until the human records the ruling.

---

## 1. Decision Statement And Scope

Decide the **owner** and the **recorded procedure** for the release-time
artifact protected-content scan and the legal/protected-data release gate
(register row `D-20`, `execution/_Coordination/_DECISIONS/_REGISTER.md:53`;
gap `RGAP-005`). Concretely:

1. **Ownership** — who runs the scan and who signs the legal/protected-data
   release gate.
2. **Procedure** — which release-artifact classes are in scope, what checks
   run against them, and where the per-release scan record lands.
3. **Gate semantics** — what must exist (scan record + human acceptance)
   before any publication act, and what a blocking finding does.

This packet proposes scan ownership and procedure only. It does **not** claim
any artifact, fixture, repository state, or release candidate is clean of
protected content; no scan has been run under this procedure and no scan
record exists (§2). It does not rule `D-06` (release matrix / installers /
publication — prepared in this same tranche as
`D-06_release_matrix_installers_publication.md`; register row
`_REGISTER.md:33`), does not rule `D-10b` (hash-bound PDF emitter,
`_REGISTER.md:41`), does not activate `D-05b` public-export CI, and does not
create any lifecycle, release-readiness, professional, certification, or
milestone claim. Blocked items: Phase E, E7
(`plans/PLAN_2026-06-17_prd_completion.md:205`) and the R5 exit criterion
"no known protected standards data" (`docs/PRD.md:1272`).

## 2. Verified Facts (Checked Cold, 2026-07-04)

| Check | Result |
|---|---|
| `RGAP-005` gap text | `execution/_Aggregation/TP-RELEASE-GAP-REGISTER-REFRESH-001_2026-05-31/Gap_Disposition_Register.csv:6` — owner `IP_AND_DATA_BOUNDARY`, severity `BLOCKER_TO_RELEASE_CLAIM`: "Release artifact scan is not recorded." Evidence note: "TP-INTEGRATED-VERIFY-002 includes protected-content test surfaces but no release artifact scan record or legal/protected-data release gate." Recommended action: "Run a bounded release artifact protected-content scan tranche before any public release claim." |
| R5 exit criterion | `docs/PRD.md:1272` (§22.6 Release R5: Engineering Beta, `:1258`): "Public repository contains no known protected standards data." |
| Completion-plan D-20 row | `plans/PLAN_2026-06-17_prd_completion.md:109` — "Assign an owner and a recorded procedure for the release-time artifact scan + legal/protected-data release gate. Test surfaces exist (`core/reporting/protected_content_linter`, `tests/security/`, DEL-08-05) but no release-artifact scan record exists." Plan's own note: "Recommend: DEL-08-05 owns the scanner; a new release-gate procedure records the scan disposition + human legal acceptance." |
| E7 work item | `plans/PLAN_2026-06-17_prd_completion.md:205` — E7 is "a recorded release-time artifact scan + legal/protected-data release gate evidence, satisfying the §22.6 exit 'no known protected standards data.' Scanner surfaces exist (`protected_content_linter`, `tests/security/`); the *release-time scan record* does not"; dependency column: `D-20` (owner/procedure). |
| No scan record exists | `validation/evidence/` contains only `sweeps/` (DEC-025 five-surface sweep summaries, `docs/BUILD_AND_RELEASE.md:142`); a repo-wide search finds no release-artifact scan record file. Consistent with the RGAP-005 gap text. |
| Protected-content definition (policy) | `docs/IP_AND_DATA_BOUNDARY.md:44-53` (§3 "Public repository must not contain"): ASME or other standards-body code text/tables/figures/examples/commentary; material allowable tables copied from standards; B31J SIF/flexibility-factor content copied from standards; B16/B36/MSS/ISO/EN/CSA dimensional or rating tables copied from standards; proprietary vendor catalogs without redistribution rights; commercial software examples/report templates/benchmark files without permission; user private rule packs, owner standards, company design bases; code-specific load combinations, allowables, SIFs, flexibility factors, acceptance criteria, or owner requirements unless invented examples or otherwise cleared. |
| Policy is not a legal clearance | `docs/IP_AND_DATA_BOUNDARY.md:17` — "This policy is a project governance control, not legal advice … Human/legal review remains required when redistribution rights or protected-content status are uncertain." |
| Code-neutral boundary (PRD) | `docs/PRD.md:84` — "Copyright-respecting architecture: Code-specific values and protected data are entered by the user or imported from lawful private sources." `docs/PRD.md:439` — "The software shall not supply protected B31J or code-derived bend values in the public distribution." Component factors are user-entered throughout: FR-017/018/019/020 (`docs/PRD.md:369-372`), material allowables user-entered (`:679-682`). Register row `D-18` names this "the code-neutral boundary" (`_REGISTER.md:51`). |
| PRD public-surface prohibitions | `docs/PRD.md:620` (public examples: invented values only, no protected ASME content or copied code formulas); `:897` (§15.2: public report templates shall not reproduce protected ASME code text, tables, examples, or proprietary standards content); `:972` (validation examples not copied from protected standards or commercial manuals without written permission); `:986` (no screenshots or scans of protected standards); `:1014` (report notice text: "It does not include ASME code text, ASME tables, ASME material allowables, or proprietary code data"). |
| Existing scanner surface (DEL-08-05) | `execution/_Decomposition/SOFTWARE_DECOMP.md:324` — DEL-08-05 "Report protected-content linter", TEST_SUITE, with the recorded caveat "Heuristic plus review; cannot be sole legal control." Crate: `core/reporting/protected_content_linter/src/lib.rs:1-9` — evaluates caller-supplied public report/template/example text; explicitly "does not read arbitrary project files … choose CI/release policy, or provide legal clearance". |
| Linter capability today | `core/reporting/protected_content_linter/src/lib.rs:364-410` — the shipped markers are **deterministic synthetic markers only** (`OPS_SYNTHETIC_PROTECTED_TABLE`, `OPS_SYNTHETIC_CODE_FORMULA`, `OPS_SYNTHETIC_PRIVATE_RULE_PAYLOAD`, `OPS_SYNTHETIC_VENDOR_CATALOG`) plus prohibited professional-claim phrases (`:412-419`). It exercises the lint mechanism; it does **not** currently pattern-match real standards-table signatures. |
| Security test surface | `tests/security/` — `test_local_first_storage_policy.py`, `test_redaction_export_controls.py`, `test_secret_private_library_handling.py`, `test_telemetry_policy.py`; runnable via `check_release_readiness.py --profile security` (`docs/BUILD_AND_RELEASE.md` §5 profile table). |
| Release process already names the scan | `docs/BUILD_AND_RELEASE.md:97` (§4 release-evidence field "Data boundary status: Protected-content, private-data, and real-secret scan result"); `:176` (§6 packaging step 4: "Confirm protected-content, private-data, and real-secret scan disposition"); `:200` (§7 release-candidate review phase requires "Release notes, gate record, scan record, and human acceptance record"). |
| Gate framework already names the scan | `docs/RELEASE_QUALITY_GATES.md:53` (every gate record must include "protected-content, private-data, and real-secret scan disposition"); `:124` (report-template gate: protected-content lint passes for public examples and templates); `:133` (release-label floor includes protected-content lint); `:151` ("No waiver may authorize protected-content copying, private-data exposure, certification claims…"). |
| Provenance metadata contract | `docs/IP_AND_DATA_BOUNDARY.md:57-66` (§4): every public data record carries `source_name`, `source_location`, `source_license`, `contributor`, `contributor_certification`, `redistribution_status` (`public_permissive` / `private_only` / `unknown` / `protected_suspected`), `review_status`; records with `source_license=TBD`, `redistribution_status=unknown`, or missing certification "are not acceptable as public data." |
| Quarantine rule | `docs/IP_AND_DATA_BOUNDARY.md:73-77` (§5): suspected protected derivation → stop ingestion, mark `protected_suspected`, move to `quarantine/protected-content/` (path named by policy; directory not yet instantiated — created on demand), record, request human/legal review. |
| Container artifact (D-09/DEC-028) | `execution/_Decomposition/SOFTWARE_DECOMP.md:606` — the native project package / public transport form is a multi-member archive (zip/directory) per the PKG-17 contracts: member inventory, manifest, per-member JCS hashes; evidence binds to canonical members + manifest hashes, not raw container bytes; extension/naming lands with D-06. Packet: `D-09_native_package_container.md:99` (members are canonical-JSON documents plus optional materialized artifacts and the DEC-021 HTML report, with a manifest listing every member). |
| Report artifact (D-10/DEC-021) | `execution/_Decomposition/SOFTWARE_DECOMP.md:599` — canonical hash-bound report is a deterministic, self-contained, scriptless single-file HTML (SHA-256 recorded; "three-point linter gating with the post-render gate blocking export"); webview print-to-PDF is a labeled derived view; hash-bound PDF emitter deferred to `D-10b` at the R5 lead-up (`_REGISTER.md:41`, NOT_PREPARED). |
| Installer artifacts (D-06) | `docs/BUILD_AND_RELEASE.md` §6 item 7 (record package path/target/checksum/signing state if installers are produced); formats/targets/signing TBD pending `D-06` (`_REGISTER.md:33`, NOT_PREPARED; packet `D-06_release_matrix_installers_publication.md` prepared in this same tranche). |
| Public-export coupling | `D-05_ci_provider_workflow.md:97` — the export pipeline, publication decision, "and RGAP-005 protected-content scan are prerequisites" for public-export CI; `:142` — the public OpenPipeStress export profile is "a new Phase E work item with RGAP-005 scan coupling." Hosted CI must not receive protected standards content (`docs/BUILD_AND_RELEASE.md:203`). |
| Sole-maintainer authority (DEC-027) | `execution/_Decomposition/SOFTWARE_DECOMP.md:605` — maintainer quorum is one; "the sole human project authority is the sole developer, maintainer, and release authority"; external contributions closed (`D-07b` gates any future intake). |
| Synthetic-fixture precedent | `execution/_Decomposition/SOFTWARE_DECOMP.md:596` (DEC-018: unit-catalog conversion constants recorded as governed values — definitional, not standards-copied); `:600` (DEC-022: rule-pack conformance corpus under `fixtures/rule_expressions/` is "synthetic values only"). |

## 3. What Counts As Protected Content Here

The project's own wording, not an invented list, defines the scan target. The
canonical risk in this domain is piping-code data copied from standards:
ASME/standards-body code text, tables, figures, examples, and commentary;
material allowable tables; B31J stress-intensification / flexibility-factor
content; B16/B36/MSS/ISO/EN/CSA dimensional or rating tables — plus
proprietary vendor catalogs, commercial-software examples/benchmarks without
permission, and private/company data (`docs/IP_AND_DATA_BOUNDARY.md:44-53`).
The architecture keeps all such values user-entered on the private side of the
code-neutral boundary (`docs/PRD.md:84`, `:439`, FR-017–FR-020; `D-18`
register row), so any appearance of them inside a *public release artifact*
is by construction a boundary violation, never intended content. The scan is
the release-time check that this construction held.

Two boundary clarifications already ruled elsewhere and only restated here:
private runtime residency of the owner's own data is out of scope
(`DEC-051` / `docs/IP_AND_DATA_BOUNDARY.md:91` — runtime residency is not
redistribution); and invented demonstration values with clear notices are
permitted public content (`docs/IP_AND_DATA_BOUNDARY.md:39`,
`docs/PRD.md:620`) — the scan distinguishes provenance-documented invented
data from unprovenanced numeric tables, it does not ban numbers.

## 4. Release-Artifact Classes In Scope (Checked Cold)

| # | Class | Basis | Scan surface |
|---|---|---|---|
| AC-1 | Native project package / public transport container: multi-member archive, canonical-JSON members + manifest with per-member JCS hashes | `DEC-028` (`SOFTWARE_DECOMP.md:606`); `D-09_native_package_container.md:99` | Walk the manifest member inventory; lint text/JSON members; verify no member carries unprovenanced code-table data |
| AC-2 | Canonical calculation-report artifact: deterministic single-file HTML (hash-bound) and any derived PDF view; report templates | `DEC-021` (`SOFTWARE_DECOMP.md:599`); `docs/PRD.md:897` | DEL-08-05 lint (already three-point gated at render/export); template gate per `RELEASE_QUALITY_GATES.md:37` |
| AC-3 | Installers / platform packages and their embedded resources (when produced under the `D-06` ruling) | `docs/BUILD_AND_RELEASE.md` §6 item 7; `D-06` row `_REGISTER.md:33` | Payload file listing; lint bundled examples/templates/docs; checksum recorded per §6 |
| AC-4 | Public repository / sanitized public export tree (the R5 exit object itself; `D-05b` coupling) | `docs/PRD.md:1272`; `D-05_ci_provider_workflow.md:97,142` | Tree-wide sweep of public fixtures, examples, templates, docs; provenance-manifest check |
| AC-5 | Public examples, fixtures, validation benchmarks, and conformance corpora shipped inside any of AC-1–AC-4 | `docs/PRD.md:620,972,986`; `DEC-022` synthetic corpus; `docs/IP_AND_DATA_BOUNDARY.md:57-66` | §4 provenance fields present and acceptable (`public_permissive` + `accepted`); anything else blocks |
| AC-6 | Release notes, gate records, and release-evidence documents accompanying the candidate | `docs/BUILD_AND_RELEASE.md:97,200`; `docs/RELEASE_QUALITY_GATES.md:53` | Lint (they are public text); confirm the scan-disposition field is filled, not asserted blank |

## 5. Existing Practice The Procedure Extends

The scan is not a new invention; the governing docs already require its
*result* in three places (`docs/BUILD_AND_RELEASE.md:97,176,200`;
`docs/RELEASE_QUALITY_GATES.md:53`) — what is missing is the owner, the
recorded procedure, and any actual per-release record (RGAP-005). Existing
mechanisms to build on rather than duplicate:

- **DEL-08-05 linter** — the lint engine and finding taxonomy exist
  (severity/policy/review-route per finding, `lib.rs:294-310`), with the
  recorded caveat that it is "heuristic plus review; cannot be sole legal
  control" (`SOFTWARE_DECOMP.md:324`). Today it matches synthetic markers
  only (§2), so a release-time scan gains real detection power only through
  the pattern/provenance extensions proposed below.
- **`tests/security/` + `--profile security`** — private-data, redaction,
  secret-handling, telemetry checks already run as a local gate
  (`docs/BUILD_AND_RELEASE.md` §5).
- **Evidence-sweep record convention** — machine-readable, commit-bound JSON
  under `validation/evidence/sweeps/` (`docs/BUILD_AND_RELEASE.md:142`) is
  the in-repo precedent for where and how a scan record should land.
- **Provenance metadata + quarantine** — §4 fields and the §5 quarantine
  path (`docs/IP_AND_DATA_BOUNDARY.md:57-77`) give the scan its
  fixture-disposition vocabulary and its remediation route.

## 6. Ownership Reality

Per `DEC-027` the maintainer quorum is one and the sole human project
authority is the sole release authority (`SOFTWARE_DECOMP.md:605`). There is
no second person to own the scan, and the gate framework already requires a
"human acceptance record" at release-candidate review
(`docs/BUILD_AND_RELEASE.md:200`) and forbids waiving protected-content
copying (`RELEASE_QUALITY_GATES.md:151`). The IP policy adds that human/legal
review remains required where status is uncertain
(`docs/IP_AND_DATA_BOUNDARY.md:17`). The realistic decision is therefore not
*who* signs — the owner does — but how much of the scan is tooling versus
manual checklist, and whether the gate act itself may ever be automated.

## 7. Relationship To Adjacent Decisions (No Ruling Herein)

| Row | Relationship to D-20 |
|---|---|
| `D-06` (`_REGISTER.md:33`, NOT_PREPARED; packet `D-06_release_matrix_installers_publication.md` prepared in this same tranche) | D-06 decides *what installers/publication targets exist*; D-20 decides *who scans them and how*. AC-3 payload-walk mechanics finalize under the D-06 ruling (§10 U-3). The two rulings compose; neither depends on the other's option choice. |
| `D-05b` (`_REGISTER.md:44`, NOT_PREPARED) | Public sanitized-export CI is downstream of this scan: the D-05 packet records the RGAP-005 scan as a prerequisite of the public export profile (`D-05_ci_provider_workflow.md:97,142`). A D-20 ruling unblocks that prerequisite's *definition*, not its *satisfaction*. |
| `D-10b` (`_REGISTER.md:41`, NOT_PREPARED) | If a hash-bound PDF emitter is later ruled in, its output joins AC-2 as a scanned member; no change to this procedure is needed beyond listing the new member kind. |
| `D-07`/`DEC-027` (`_REGISTER.md:34`, RULED) | Supplies the ownership fact (§6): sole maintainer, sole release authority. This packet does not reopen quorum. |
| `D-12` (`_REGISTER.md:39`, NOT_PREPARED) | The R5-gate disposition of FR-024/FR-025 is unrelated in content but shares the R5 lead-up; the R5 exit criterion this packet serves (`docs/PRD.md:1272`) is asserted only by a future scan record plus owner acceptance, never by this packet. |

## 8. Options

| ID | Option | Consequence |
|---|---|---|
| **O-A** | **Owner-manual scan: recorded checklist only.** The owner walks a per-release checklist over AC-1–AC-6 (member inventory review, example/fixture provenance review, template/report spot lint, security-profile run) and writes the scan record + sign-off by hand. No new tooling. | Cheapest to stand up; fully consistent with the human-gate posture. But detection is entirely eyeball-based over growing artifact trees — the linter's synthetic-marker limitation stays unaddressed, checklist fatigue is the failure mode for a sole maintainer, and the scan record's evidentiary value is "owner looked," with no machine-readable finding trail. RGAP-005 closes formally but weakly. |
| **O-B** | **Tooling-assisted scan with owner sign-off (extend DEL-08-05 + provenance checks; the gate stays a human act).** A bounded tranche adds a release-scan runner that (a) walks each artifact class's member/payload inventory, (b) runs the DEL-08-05 lint extended with real pattern classes for standards-table signatures (standards designators such as B31.1/B31.3/B31J/B16.x adjacent to dense numeric tabular data; allowable/SIF table shapes), (c) verifies §4 provenance fields on every public data record (`public_permissive` + `accepted` or it blocks), (d) runs the `security` profile, and emits a machine-readable per-release scan record. The owner reviews findings, records dispositions, and signs the legal/protected-data gate; publication is barred until the signed record exists. | Matches the plan's own recommendation (`PLAN_2026-06-17:109`: DEL-08-05 owns the scanner; procedure records disposition + human legal acceptance) and the DEL-08-05 caveat (heuristic **plus review**). Machine findings + human ruling gives the strongest honest record a sole maintainer can produce. Cost: one bounded tooling tranche (pattern design is genuinely hard — see §10 U-2 — so the tool must fail toward `UnknownProvenanceReviewRequired`, never toward silent pass). |
| **O-C** | **Fully automated gate: scanner pass = gate satisfied, no per-release human act.** | Fastest per release, but inconsistent with the recorded governance: DEL-08-05 "cannot be sole legal control" (`SOFTWARE_DECOMP.md:324`), the linter disclaims providing legal clearance (`lib.rs:8`), the gate framework requires a human acceptance record (`BUILD_AND_RELEASE.md:200`), and the IP policy requires human/legal review under uncertainty (`IP_AND_DATA_BOUNDARY.md:17`). A pattern scanner cannot decide redistribution rights; automating the legal gate converts a heuristic into a legal claim. Not recommended. |
| **O-D** | **Defer D-20 until D-06 rules the installer/publication matrix.** | Sequencing-only. But the artifact classes are already sufficiently defined by ruled decisions (AC-1/AC-2 per `DEC-028`/`DEC-021`; AC-4 per the PRD R5 exit), and `D-06` is being prepared in this same tranche — deferral buys no information the D-06 cross-cite does not, while leaving E7 and RGAP-005 open. Not recommended. |

## 9. Recommended Disposition (PROPOSAL)

Recommend **O-B**, with the following procedure text if accepted:

1. **Ownership.** The owner (sole maintainer and release authority,
   `DEC-027`) owns the release-artifact protected-content scan and is the
   sole signatory of the legal/protected-data release gate. DEL-08-05 owns
   the scanner tooling surface, per the completion plan's recommendation
   (`plans/PLAN_2026-06-17_prd_completion.md:109`).
2. **Scope.** Every release candidate scans artifact classes AC-1–AC-6 (§4)
   as applicable to what the candidate actually ships; classes not shipped
   are recorded `not_applicable`, never silently omitted.
3. **Checks.** (a) Artifact-inventory walk: container manifest members
   (`DEC-028`), installer payload listings (`D-06`), public-export tree.
   (b) Protected-content lint over all public text/JSON surfaces via the
   DEL-08-05 engine, extended in a bounded tranche with standards-table
   signature patterns (§8 O-B; failure direction is
   `UnknownProvenanceReviewRequired` → human review, never silent pass).
   (c) Provenance-manifest check: every public data record carries the
   `docs/IP_AND_DATA_BOUNDARY.md` §4 fields with
   `redistribution_status=public_permissive` and `review_status=accepted`;
   `unknown`/`TBD`/missing-certification blocks. (d) Private-data and
   real-secret checks via `check_release_readiness.py --profile security`.
   (e) Quarantine hygiene: no `quarantine/protected-content/` material in
   any artifact.
4. **Record.** Each scan writes a machine-readable record plus owner
   sign-off block to
   `validation/evidence/releases/SCAN_<candidate>_<utc>_<commit12>.json`
   (sibling convention to the `DEC-025` sweep records,
   `docs/BUILD_AND_RELEASE.md:142`), capturing the §4 release-evidence
   fields (`docs/BUILD_AND_RELEASE.md:88-99`), per-artifact checksums,
   per-finding dispositions, and the human acceptance record required at
   release-candidate review (`docs/BUILD_AND_RELEASE.md:200`).
5. **Gate semantics.** The signed scan record is a **precondition to any
   publication act** (installer upload, public export push, container-sample
   publication, release-notes publication). A `Blocking` finding halts
   publication until the item is removed or quarantined per
   `docs/IP_AND_DATA_BOUNDARY.md` §5 and the scan is re-run; no waiver may
   authorize protected-content copying (`docs/RELEASE_QUALITY_GATES.md:151`).
   The scan record asserts only what was scanned and found; the release-gate
   *acceptance* is the owner's human act, and a clean scan is not a legal
   clearance, release claim, or milestone claim.

**Scan record minimum content** (composing the already-required fields; no
new invention):

| Field | Source of the requirement |
|---|---|
| Source revision, working-tree state, runtime versions, commands run | `docs/BUILD_AND_RELEASE.md:88-99` (§4 release-evidence fields) |
| Artifact inventory per class AC-1–AC-6 with per-artifact checksums; `not_applicable` recorded for unshipped classes | §4 above; `docs/BUILD_AND_RELEASE.md` §6 item 7 |
| Per-finding list: code, severity, policy tag, review route, disposition | DEL-08-05 finding taxonomy (`core/reporting/protected_content_linter/src/lib.rs:294-310`) |
| Provenance-check result for every public data record (fields per `docs/IP_AND_DATA_BOUNDARY.md:57-66`) | IP policy §4; blocks on `unknown`/`TBD` |
| Security-profile pass/fail | `docs/BUILD_AND_RELEASE.md` §5 |
| Known limitations and open risks | `docs/RELEASE_QUALITY_GATES.md:53` common evidence |
| Owner acceptance record (the human legal/protected-data gate act), or the halt disposition | `docs/BUILD_AND_RELEASE.md:200`; `docs/RELEASE_QUALITY_GATES.md:151` |
| Statement that the record is software-quality/data-boundary evidence only — not legal clearance, release claim, certification, or milestone claim | `docs/RELEASE_QUALITY_GATES.md` §3 last bullet; this packet §1 |

Rationale: O-B is the only option that simultaneously extends the existing
recorded practice (§5), respects the DEL-08-05 caveat and the human-gate
requirements the governing docs already state, and produces evidence stronger
than "owner looked" without pretending a pattern scanner can make a legal
determination. It also matches the completion plan's standing recommendation
verbatim.

## 10. Unresolved Items (Not Asserted)

- **U-1:** Whether any *current* repository or fixture content would trip an
  extended scanner is unknown — no scan under this procedure has run, and
  this packet makes no cleanliness claim about any artifact (§1).
- **U-2:** The detection efficacy of standards-table signature patterns is
  undemonstrated; the current linter matches synthetic markers only
  (`lib.rs:364-410`). Pattern design and its false-negative bounds are
  bounded-tranche work under the O-B ruling, with the fail-toward-review
  posture as the stated mitigation, not a solved problem.
- **U-3:** Exact installer payload-walk mechanics depend on the `D-06`
  ruling (formats/targets TBD, `_REGISTER.md:33`); AC-3 check details are
  finalized when D-06 rules, without reopening this decision.
- **U-4:** How the scan record surfaces in a future sanitized public export
  (`D-05b`) — whether the record itself is exported or summarized — is
  deferred to D-05b activation.

## 11. Human Ruling And Disposition

**Ruling recorded:** O-B approved as recommended by owner (Ryan Tufts), 2026-07-04 — in-session blanket slate direction ('proceed as recommended through the "Decision slate"'; verbatim in `_DomainEngines/bridge/LOOP_RECEIPTS.md` Receipt 20), adopting §9's procedure text as written: owner-owned scan and sole-signatory release gate (`DEC-027`), DEL-08-05 as the scanner tooling surface, AC-1–AC-6 scope with `not_applicable` recording, checks (a)–(e), the machine-readable `SCAN_<candidate>_<utc>_<commit12>.json` record plus owner sign-off block, and the gate semantics (signed scan record as a precondition to any publication act; `Blocking` findings halt publication; no waiver may authorize protected-content copying). Codified as `DEC-058` in `execution/_Decomposition/SOFTWARE_DECOMP.md` §12; register row D-20 → RULED. Nothing is scanned, cleared, or published under this ruling; the scanner-extension tranche is separate downstream work.

## 12. Ruling Mechanism

Per existing practice, the human project authority selects an option or rules
directly. The ruling is appended to
`execution/_Decomposition/SOFTWARE_DECOMP.md` §12 as the next `DEC` entry
citing this packet; the register row `D-20` then moves from `AWAITING_RULING`
to `RULED` with the decision pointer. Any scanner-extension tranche, scan
execution, or scan-record creation happens only after and under that ruling;
nothing is scanned, cleared, or published under this packet.
