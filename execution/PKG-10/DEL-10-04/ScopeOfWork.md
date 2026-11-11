---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-10-04
package_id: PKG-10
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-032]
package_objective_refs: [OBJ-008, OBJ-009]
---

# Scope of Work — DEL-10-04 Build, packaging, and CI/CD pipeline

## Purpose and Objective Traceability

This deliverable turns the source tree into the application a user installs, and makes every change prove itself before it is merged. It has four parts: a build that produces the desktop application and its engines from committed inputs alone; hosted checks that select what a change needs and give one required verdict; packaging that produces a signed, notarized macOS artifact bound to the revision it was built from; and the public repository that is SWBPIPE's home. The result it contributes is that anyone can tell which source a released SWBPIPE came from and that it is the build its developer signed, and that no change reaches the main branch without the checks its content calls for.

It serves SOW-032 under OBJ-008 and OBJ-009 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-10, row DEL-10-04, and architecture-basis row AB-00-08), with `docs/PRD.md`'s product principles (the MIT licence; the sole maintainer holds release authority), §20.4, §20.5, §21.2, §22.1, §22.4 and §24 V1. `docs/RELEASE_TARGET_v1.0.md` names it as the owner of two exit criteria:

- criterion 15: the build is signed and notarized, and MIT is recorded consistently;
- criterion 16: `sgttomas/SWBPIPE` is the independent public home, the release candidate is bound to its repository revision with standalone build, test and CI verification, and the owner has completed and signed the scan of that repository before release.

Outputs:

- **OUT-001** — The reproducible build of the desktop application, its WebAssembly engine and the Rust crates, from the root workspace manifest.
- **OUT-002** — Hosted verification: the workflows, check registry and selection that give the single required result `harness`.
- **OUT-003** — Release packaging: the signed, notarized macOS artifact, its checksum and its release artifact record, with the build and release guide `docs/BUILD_AND_RELEASE.md` and the release-notes template `docs/RELEASE_NOTES_TEMPLATE.md`.
- **OUT-004** — The public repository `sgttomas/SWBPIPE` as SWBPIPE's independent home.
- **OUT-005** — The desktop's build and package readiness panel.

## Deliverable Definition — Ontology

**Terms the pipeline implements.**

- *Build inputs.* The committed source, the lockfiles and the documented toolchain versions. The build takes nothing else.
- *Generated file.* A file a build step produces from build inputs and that is not hand-edited: the WebAssembly engine's glue, the bundled example results and their generation records, the application icon.
- *`harness`.* The single required result on a pull request. It passes only when every check selected for the change ran and passed on the exact head.
- *Selection.* The choice of checks for a change, made from the complete diff of the pull request.
- *Product route and document route.* A change to code, configuration, fixtures or file layout takes the product route and runs the checks its paths select. A change confined to declarative specifications under `execution/`, or to in-place Markdown edits covered by the current reader audit, takes the document route.
- *Declarative specification.* What `execution/` holds and nothing else: Markdown, YAML, JSON, CSV, text, SHA-256 lists and patches. Code that runs lives in the project's code locations.
- *Reader audit.* The inventory of code that reads document content, bound to the source it was made from. A source change makes it stale, and prose changes then take the product route until it is refreshed.
- *Release matrix.* The platforms a release is built for. For v1.0 it is macOS on Apple Silicon (`aarch64-apple-darwin`) only.
- *Artifact.* The Tauri `.app` bundle in a zip archive, with its SHA-256 checksum published beside it.
- *Signed and notarized.* Signed with a Developer ID Application certificate with the hardened runtime, and notarized by Apple with the ticket stapled, under the application identifier `com.chirality.swbpipe`.
- *Release artifact record.* The machine-readable record of one packaged artifact: its checksum, source commit, working-tree state, toolchain versions, build command, signing state and publication state.
- *Authenticity chain.* The three things that bind an artifact to its source: its checksum, the commit-bound release-candidate sweep summary, and the release artifact record that binds both to the revision.
- *Rehearsal.* Packaging an unsigned bundle to exercise the path. Its record is marked as a rehearsal and is not a distribution record.
- *Release.* The owner's act. A passing check, a merge, a signed artifact and a complete record are evidence for it and are never it.

**Boundary.**

- The release gates and the release-candidate sweep are DEL-09-05's. The release-candidate scan and its record are DEL-08-05's; the owner signs it.
- The licence text and the governance baseline are DEL-01-01's. This deliverable records the licence consistently in what it builds and ships.
- The embedded agent's supplier, its pin manifest and its licence payloads are DEL-16-06's. This deliverable builds them into the application exactly as pinned.
- The headless runner is DEL-10-05's; its crate is built and tested here like any other.
- The desktop shell and its permissions are DEL-07-11's and the storage paths DEL-12-01's; the bundle settings in the same configuration file are this deliverable's.
- Coverage tooling and any coverage floor are DEL-09-05's. Numerical tolerances are the validation deliverables'.

**Windows.** Windows is not a release platform for v1.0; the owner revisits it immediately after. Windows or Linux enters the release matrix only with a packaged build, a recorded smoke run of the packaged application and a release artifact record for that platform. Of the Windows native boundaries, this deliverable holds the Windows bundle configuration and icon and the build's handling of Windows paths. A Windows build or Windows check of this deliverable does not meet the Windows conditions of the other native boundaries, which the deliverables owning those acts hold: shell dispatch and security (DEL-07-11), admission and worker custody (DEL-04-07), qualification (DEL-04-08), recovery (DEL-02-05), publication (DEL-08-01), model save (DEL-15-03) and the native user experience (DEL-07-06).

**Not decided.**

- Where a release is published.
- For Windows: the installer format, the WebView2 bootstrap policy, signing and distribution.
- Coverage, performance and timing thresholds as merge or release conditions. No check fails on one.

## Completion and Reliance Basis — Epistemology

### Requirements

Requirement numbers 001 to 008 are those of the earlier `REQ-10-04-nn` series.

- **REQ-001** — The project has reproducible build, packaging and CI workflows for the platforms of its release matrix (SOW-032).
- **REQ-002** — The checks the pipeline runs are the accepted baseline: Cargo tests, Vitest, Playwright, the Python contract and validation tests, and the protected-content and provenance gates. Together they cover the test kinds PRD §22.4 requires of every release (decomposition AB-00-08).
- **REQ-003** — The release matrix for v1.0 is macOS on Apple Silicon. The artifact is the `.app` bundle in a zip archive with a published SHA-256 checksum and a release artifact record.
- **REQ-004** — A pipeline choice the owner has not made stays unmade: the pipeline enforces no threshold, platform or publication target the owner has not selected.
- **REQ-005** — No build, packaging or CI step goes around unit checks, provenance checks, diagnostics, privacy controls or the adapter and plugin boundary.
- **REQ-006** — Nothing the pipeline publishes or uploads contains protected standards text or examples, proprietary engineering values, private rule packs, private project data or private library data. Hosted jobs receive none of these, and receive no signing secret or publishing credential (PRD §20.1, §20.3, §20.5).
- **REQ-007** — No release artifact, release note or pipeline status claims certification, sealing, approval, authentication or code compliance for reliance (PRD §21.2).
- **REQ-008** — Packaging and CI outputs carry the evidence needed to reproduce them: the source revision and working-tree state, toolchain versions, the commands run and their results, the validation status and the known limitations.
- **REQ-009** — `sgttomas/SWBPIPE` is SWBPIPE's independent public repository. It carries the project's practical source history. It builds, tests and runs its hosted verification on its own, and a release candidate is identified by a revision of it. Before v1.0 is released the owner has completed and signed the scan of that repository candidate, including the full private-term scan and a disposition for each finding; an unresolved blocking finding prevents release. The verified import may be public before the scan is signed (also cited as DEL-10-04:1; exit criterion 16).
- **REQ-010** — The first release, v1.0, is signed and notarized as defined above, and the checksum, the release-candidate sweep summary and the release artifact record bind the artifact to its source revision. Certificates, passwords and keys stay in the local keychain (also cited as DEL-10-04:2; exit criterion 15).
- **REQ-011** — The MIT licence is recorded consistently: in the licence file, in every package manifest, in the bundle's metadata, in the release notes and in the repository's description (exit criterion 15).
- **REQ-012** — `harness` is the one required result. It fails when any selected check fails, is cancelled or never runs. Selection uses the complete diff, and missing diff information selects product coverage. A passing result reused from an earlier run is accepted only when its inputs and environment are the same, and it names the run it came from.
- **REQ-013** — `execution/` holds declarative specifications only. No maintained code imports, loads, executes or collects anything from it, and reads of it are limited to the declared document readers. A document-route change runs the boundary guard, parses each changed JSON, YAML and CSV file, and runs the parity and graph checks those files feed; an invalid specification fails there. Product jobs check out the tree without `execution/`.
- **REQ-014** — A document-route change reaches its `harness` result in under two minutes from the hosted run event, queueing and setup included. The time target never removes a check that reads the changed content.
- **REQ-015** — The build produces every generated file from build inputs by a named command, and a generated file that ships is bound to its inputs by a generation record. A checkout on any platform keeps the committed bytes of sources, lockfiles, fixtures, schemas and frozen references, and no check normalizes reference text or replaces an expected hash to accommodate a checkout.
- **REQ-016** — The application build includes the embedded agent's supplier resource, its pin manifest and its licence payloads exactly as pinned. The build verifies the input it was given and the resources in the built bundle, and does not report success otherwise.
- **REQ-017** — Windows checks that run before Windows is a release platform are separate from `harness` and are described as covering only what they run. No Windows artifact is described as a release.
- **REQ-018** — The build and package readiness panel reports the build scripts and shell metadata that actually exist and the release decisions that are not made. It activates nothing and states no release or professional claim.

### Acceptance criteria

- **AC-001** — From a clean checkout of a committed revision, with the documented toolchain and the locked dependencies, the documented commands build the desktop application, its WebAssembly engine and every crate. Packaging the same bundle twice gives a byte-identical archive and checksum.
- **AC-002** — On a pull request `harness` passes only when every selected check ran and passed on the exact head. A selected check that fails, is cancelled or produces no result fails it, a change whose diff cannot be read selects product coverage, and each reused pass names its original run.
- **AC-003** — Each check family of REQ-002 is selected by a change to its inputs and runs under `harness`, and for each test kind of PRD §22.4 there is a registered check that a change to the tested code selects.
- **AC-004** — A change confined to declarative files under `execution/`, or to audited in-place Markdown, completes `harness` without a product build, having run the boundary guard, the parsers and the parity and graph checks. A file of a runnable kind added under `execution/`, and maintained code that imports, executes or reads from `execution/` outside the declared readers, each fail the guard. After a source change, a prose change takes the product route until the reader audit is refreshed.
- **AC-005** — A document-route run reaches its `harness` result in under two minutes from the run event, queueing and setup included.
- **AC-006** — The packaged artifact is an `aarch64-apple-darwin` `.app` in a zip with its `.sha256`. Extracted as a user receives it, the bundle verifies as Developer ID signed with the hardened runtime and a secure timestamp, carries a stapled notarization ticket and is accepted by Gatekeeper. Packaging refuses a bundle that fails any of these unless a rehearsal is requested, and a rehearsal's record says it is one.
- **AC-007** — The release artifact record carries every member of REQ-008 with the artifact's checksum, matrix, format, build command, signing state and publication state. Its authenticity chain is marked verified only when the record's commit is clean and is the commit of a passing clean-tree sweep summary, or follows it by commits that change evidence files only. The record's human-acceptance member is empty as emitted.
- **AC-008** — No certificate, password, key, signing identity name or team identifier is in the repository, in a record or in a log the pipeline writes, and no hosted job has access to a signing secret, a publishing credential or private project, rule or library data.
- **AC-009** — The licence file, every package manifest, the bundle's metadata, the release notes and the public repository's description name MIT, and none names another licence for the project's own code.
- **AC-010** — `sgttomas/SWBPIPE` builds and passes `harness` from its own contents, with no file read from another repository. Each pre-migration commit identity that a maintained check or record cites resolves to a commit of `sgttomas/SWBPIPE`. The release candidate's records name a revision of it. For the v1.0 candidate the scan record at that revision carries the owner's sign-off with every finding dispositioned and the private-term scan attached, and no record, status or note presents a merge, a passing check or an unsigned record as that sign-off.
- **AC-011** — Release notes prepared from the template carry the install note, the validation status, the known limitations and the data-boundary and professional-responsibility statements, and neither they nor any pipeline status makes a claim REQ-007 excludes.
- **AC-012** — A build given a supplier resource that differs from the pin manifest, or lacking a licence payload, fails, and a successful build's bundle contains the pinned resource and payloads byte for byte.
- **AC-013** — Every generated file that ships is reproduced byte for byte by its named command from the committed inputs, and its generation record matches. A checkout made with newline conversion enabled holds the committed bytes of every protected input.
- **AC-014** — The Windows workflows are not among the checks `harness` requires, and no workflow, record or document describes a Windows build as a release or as qualifying memory, persistence or the native user interface.
- **AC-015** — The readiness panel lists the scripts of the root and desktop manifests as they are, reports whether the bundle is active, lists the undecided release decisions, and contains no statement of release, signing or professional acceptance.
- **AC-016** — No workflow, check or packaging step fails on a coverage, performance or timing number, builds a release for a platform outside the release matrix, or publishes to a target the owner has not named. The two-minute target of AC-005 is measured from run records and fails no check.

## Production and Verification Method — Praxeology

**Method choices that define the pipeline.**

- *One required result.* Every hosted check reports through `harness`, so that branch protection names one thing and a missing job cannot pass unnoticed.
- *Select by content.* Checks are selected from what a change touches, and uncertainty selects more, never less.
- *Two routes.* Documents and code are verified differently because product tests cannot establish that a document edit is right.
- *Sign locally, package deterministically.* Signing and notarization run on the owner's machine against the local keychain. Packaging is a separate deterministic step that verifies what it is given.
- *Records, not statuses.* What a release rests on is written to records bound to a commit; no status badge carries it.

**How each criterion is examined.**

- **VER-001** — Executable check (observes the build's outputs): runs the documented build from a clean checkout of a committed revision and packages one bundle twice, comparing the archives and checksums.
- **VER-002** — Executable check (observes selection and the aggregate result): presents invented diffs and job outcomes, including a failed, a cancelled and a missing selected job, an unreadable diff and a reused pass, and reads the selected checks and the `harness` result.
- **VER-003** — Executable check (observes the document route): presents declarative, runnable and mixed changes under `execution/`, code that reads from it, and a stale reader audit, and reads the route chosen, the guard's result and the checks run.
- **VER-004** — Executable check (observes hosted run timings): takes the run-event and `harness` completion times of document-route runs from the hosting service's own run records.
- **VER-005** — Comparison with a named reference: Apple's `codesign`, `stapler` and Gatekeeper assessments of the bundle extracted from the archive.
- **VER-006** — Executable check (observes packaging and its record): packages signed, unsigned and rehearsal bundles against clean, dirty and mismatched sweep summaries, and reads the refusals, the record's members and the chain's state.
- **VER-007** — Executable check (observes the repository, records and logs): searches them for credentials, identity names and team identifiers, reads each workflow's secrets and inputs, and reads the licence named by each manifest, the bundle and the notes.
- **VER-008** — Judgment by a named role (the owner): signs the scan record of the v1.0 repository candidate after dispositioning every finding.
- **VER-009** — Executable check (observes the public repository on its own): clones it alone, builds it and reads its `harness` result, the revision its candidate records name, and the commit each cited pre-migration identity resolves to.
- **VER-010** — Executable check (observes the supplier resource in the build): builds with a matching, a differing and an incomplete supplier input and compares the bundle's resources with the pin manifest.
- **VER-011** — Executable check (observes generated files and checkouts): regenerates each generated file and compares it with the committed one and its record, and compares a converted-newline checkout with the committed bytes.
- **VER-012** — Judgment by a named role (an independent reviewer who did not write them): reads the guide, the release-notes template, the workflows with their descriptions, the check registry, the packaging steps and the readiness panel against REQ-004, REQ-007, REQ-017 and REQ-018, and reports any unruled threshold, platform or publication target enforced, any prohibited claim, any wording a reader could take for a release or an acceptance, and any Windows statement beyond what was run.

## Governing Values and Decisions — Axiology

- **AX-001** — *Sooner verdicts, same verification.* Better is a shorter time from a push to its `harness` result. It is never bought by dropping a check that reads what changed, by letting a missing or cancelled job pass, or by weakening what a reused result must match.
- **AX-002** — *Traceable artifacts.* Better is fewer inputs to a release that are not in the commit and fewer steps a second person could not repeat, measured by the generated files and build inputs not bound to the revision. Convenience in building never loosens the binding of an artifact to its source.
- **AX-003** — *Secrets stay with the owner.* Better is more of the release path automated, up to the point where a credential would have to leave the owner's keychain. That point is never crossed.
- **AX-004** — *Evidence is not acceptance.* Better is fewer statuses, records and notes whose wording a reader could take for a release or an acceptance, counted by VER-012. This is never bought by leaving out evidence a record must carry.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-032 OBJ-008 | REQ-001 REQ-015 | AC-001 | VER-001 | The criterion itself determines the expected result: the build completes from committed inputs, and two packagings of one bundle are compared with each other |
| OUT-002 | SOW-032 OBJ-008 | REQ-012 | AC-002 | VER-002 | REQ-012's rules. Each invented case of diff and job outcomes states the selection and verdict those rules give it, written for the check and not by the selector |
| OUT-002 | SOW-032 OBJ-008 | REQ-002 REQ-005 | AC-003 | VER-002 | Decomposition AB-00-08 names the check families and PRD §22.4 the test kinds |
| OUT-002 | SOW-032 OBJ-008 | REQ-013 | AC-004 | VER-003 | `execution/` holds declarative specifications only; each invented change states the route and result it must get |
| OUT-002 | SOW-032 OBJ-008 | REQ-014 | AC-005 | VER-004 | The two-minute target is the expected result; the times are the hosting service's run records |
| OUT-003 | SOW-032 OBJ-008 OBJ-009 | REQ-003 REQ-010 | AC-006 | VER-005 VER-006 | Outside source: Apple's signature, notarization and Gatekeeper assessments decide whether the bundle is signed and notarized. The matrix and format are those REQ-003 states |
| OUT-003 | SOW-032 OBJ-008 | REQ-008 REQ-010 | AC-007 | VER-006 | Each invented combination of bundle and sweep summary states the chain state the requirement gives it |
| OUT-003 | SOW-032 OBJ-008 | REQ-006 REQ-010 | AC-008 | VER-007 | Credentials stay in the local keychain, and PRD §20.3 and §20.5 keep private data out of hosted jobs; the search is over the repository and the hosting service's configuration |
| OUT-003 | SOW-032 OBJ-008 | REQ-011 | AC-009 | VER-007 | The licence is MIT (exit criterion 15); the check reads the licence each place names |
| OUT-004 | SOW-032 OBJ-008 OBJ-009 | REQ-009 | AC-010 | VER-008 VER-009 | Exit criterion 16, with the public repository replacing the sanitized export and keeping the practical source history. The sign-off is the owner's own act and no check supplies it |
| OUT-003 | SOW-032 OBJ-008 | REQ-007 | AC-011 | VER-012 | PRD §21.2 lists the prohibited claims; the reviewer compares |
| OUT-001 | SOW-032 OBJ-008 | REQ-016 | AC-012 | VER-010 | The pin manifest, which DEL-16-06 owns, says what the bundle must contain; the bundle is compared with it |
| OUT-001 | SOW-032 OBJ-008 | REQ-015 | AC-013 | VER-011 | The product's own earlier output, which only guards against change: a regenerated file is compared with the committed one. That the committed values are right is the criterion of the deliverable whose results they are |
| OUT-002 | SOW-032 OBJ-008 | REQ-017 | AC-014 | VER-012 | v1.0 is macOS first and Windows is revisited after it; the reviewer compares the workflows, records and documents with REQ-017 |
| OUT-005 | SOW-032 OBJ-009 | REQ-018 | AC-015 | VER-012 | The manifests and the not-decided list of this scope say what the panel must show; the reviewer compares |
| OUT-002 | SOW-032 OBJ-008 | REQ-004 | AC-016 | VER-012 | The release matrix and the not-decided list of this scope say which thresholds, platforms and publication targets are selected; the reviewer compares |
