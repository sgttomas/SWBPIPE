---
doc_id: CB-2026-07-22-DEL-08-01-REPORT-PACKAGE-SEAM-001
doc_kind: coordination.candidate_brief
status: adopted_v9_authority_amendment_n4_verification_authorized
prepared: 2026-07-22
package_id: PKG-08
deliverable_id: DEL-08-01
frozen_sha: 8698b0338ac82556fee583dd3f85bb62d0b74f85
decision_basis: DEC-021, DEC-025, DEC-028, DEC-057, DEC-061, DEC-081, DEC-085, DEC-087, DAG-008
---

# CANDIDATE Brief — DEL-08-01 Report-Package Desktop and Atomic-Save Seam

**Status:** `ADOPTED V9 AUTHORITY AMENDMENT — REVERIFICATION AUTHORIZED`  
**Prepared by:** WORKING_ITEMS for HELP_HUMAN  
**Managed run:** `HELP-HUMAN-PIPING-20260722-DEL0801-REPORT-PACKAGE-R16`

**Review history:** v1 received N3B `BLOCK`; v2 received N3C `BLOCK`; v3
received N3D `BLOCK`; v4 received N3E `BLOCK`; N3F returned `COMMIT-SAFE` for
v5; the interrupted N3 attempt supplied no verdict and remains preserved. V6
is the owner-authorized one-file verification-unblock amendment recorded in
R16 `AMENDMENT_V6.md`.

This brief proposes the first DEL-08-01 `_STATUS.md ## Remaining` item only:
bind the accepted in-memory report-package producer to the desktop File menu
and make the desktop caller atomically persist the resulting `.opsproj` bytes.
It does not adopt itself or authorize N4. Owner adoption of the exact brief and
its bounded shared integration map in §8 is required after fresh refutation.

**Owner gate:** on 2026-07-22 the human owner adopted v5 and authorized N4,
then separately authorized v6 verbatim as recorded in the R16
`OWNER_ADOPTION.md`. Those external acts, not this document, changed the gate.
N4 is now authorized only inside §8 and `WRITE_MATRIX.csv`; all exclusions and
N5/W3 gates remain binding.

## 1. Accepted basis and objective

The executor resolves `REPO_ROOT` with `git rev-parse --show-toplevel` and sets
`WORKING_ROOT={REPO_ROOT}/projects/chirality-piping`.

Accepted basis:

- root/project `AGENTS.md`, `agents/AGENT_WORKING_ITEMS.md`, the ratified
  software workflow profile, and `software-workflow.json`;
- active owner-accepted DAG-008 and the live DEL-08-01
  `ScopeOfWork.md`, `_STATUS.md`, `MEMORY.md`, context, dependencies, and recent
  report-package/PDF run records;
- `DEC-021` deterministic HTML, `DEC-061` deterministic PDF, `DEC-028`
  multi-member package and caller-owned atomic-save posture, `DEC-057`
  `.opsproj` naming, and `DEC-025` evidence discipline;
- the accepted DEL-12-02 controlled-export/redaction contract and its current
  desktop bindings;
- R16 `ROUTE_MATRIX.csv`, `WRITE_MATRIX.csv`, N1/N2 returns, N3 return, and
  `FAN_IN.md` once completed.

Objective: expose one `Save Report Package…` action through the existing DOM
and native File-menu command architecture. It must assemble the existing
report package from the current solved session, refuse every blocked input or
package before filesystem effect, open one native destination picker, and save
the exact producer bytes using same-directory temp/write/sync/rename semantics.

## 2. Frozen behavior and exclusions

The five routes in `ROUTE_MATRIX.csv` are complete for this tranche. If a new
exposure or required owner surface is discovered, N4 stops for amendment.

Explicitly excluded:

- any new or changed runner verb, `core/runner/**`, DEL-10-05 `export-results`,
  CLI/stdout export, adapter behavior, public API, cloud/network/telemetry, or
  plugin runtime;
- changes to the accepted report/package member vocabulary, manifest schema,
  member order, hash bases, renderer/PDF bytes, container algorithm, or
  redaction action/context/reason vocabulary;
- compatibility-window/version policy beyond current `1.0.0`;
- project-store `.opsproj` open/import behavior, release/publication,
  lifecycle/stage/issuance, or professional acceptance;
- PKG-07/PKG-11 work and every non-DEL-08-01 state/memory/Remaining surface.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## 3. Existing producer remains authoritative

N4 consumes, without redesign, this entry point:

```text
assemble_report_package_container(&ReportPackageContainerInput)
  -> ReportPackageContainerOutcome
```

The producer remains the sole authority for member inventory/order, canonical
JSON, exact HTML/PDF bytes, manifest/package identity, per-member hashes,
deterministic ZIP bytes, and package blocking reasons. It assembles diagnostic
bytes even when blocked; the desktop caller must never persist them.

No `core/reporting/report_package/**` write is authorized by v1. A discovered
need to change the producer or its tests is a consequential amendment.

## 4. One owned desktop request bridge

N4 adds one desktop-owned Rust bridge module and one TypeScript request builder.
They map existing current-session facts to the producer's existing native
types; they do not create a second package schema.

The controlled request has:

- deterministic package ID and `export_profile_id =
  "desktop_local_private_report_package_1"`;
- source-model and source-basis references from the current model and accepted
  analysis-run IDs/hashes;
- the existing `RenderableReportInput` built by
  `buildRenderableReportInput`;
- an audit manifest using the current model/input hashes, solver name/version/
  build ref, unit-system ref, current rule-pack references, actual asset
  records, and project-default professional boundary;
- one result envelope mapped directly from current model, mechanics-result,
  analysis-run, model-hash, and input-manifest-hash state,
  preserving result IDs, quantities, dimensions, units, diagnostics,
  provenance, reproducibility references, analysis statuses, rule-pack
  metadata, and professional boundary;
- one DEL-08-06 record derived from the current report packet's actual state,
  analysis-run, comparison, and handoff evidence, with deterministic
  `section_set_id`, explicit diagnostics, provenance, unresolved/TBD state, and
  professional boundary. Empty or synthetic placeholder records are forbidden.

The package request must not reuse the preview ResultExportPanel's hard-coded
`TBD`/invented rows. Required model/input hashes and solver build identity come
from the live current-session hash and analysis-run records; their absence is
blocking. Rule-pack references are emitted only for an actually active pack
with a real identity/version/checksum/source notice. When none is active, the
list is honestly empty and the analysis status retains
`RULE_INPUTS_INCOMPLETE`; no synthetic `TBD` rule-pack row is created. The
current App/report boundary does not own complete active-pack identity/version/
checksum/source metadata. App does own the nullable lifted rule-check aggregate.
An empty rule-pack list is permitted only when that aggregate is `null` and the
solve-time status is `RULE_INPUTS_INCOMPLETE`. Any non-null aggregate — whether
`RULE_INPUTS_INCOMPLETE`, `USER_RULE_CHECKED`, or `USER_RULE_FAILED` — and any
checked/failed solve status is explicitly blocked with stable diagnostic
`REPORT-PACKAGE-RULE-PACK-BINDING-UNAVAILABLE` before assembly. This tranche
does not falsely emit an empty list for an active pack and does not reach into
RuleCheckRunPanel-private state. Lifting validated active-pack metadata is a
separately re-derived follow-on, not silently queued here.

Solver identity is evidence-backed by the already-public
`open_pipe_stress_product_physics::solver_component_name()` and
`solver_component_version()` APIs. The bridge records the exact component
package reference `{name}@{version}` as the technical-preview build reference;
it does not claim a release-grade binary/Git identity. Missing/empty identity
blocks. The owned package projection uses hash/ref facts from the current
analysis-run record but does not copy its overtaken invented-preview privacy or
provenance labels.

The existing render-input builder is corrected within the fence to describe
the current local project session truthfully: source/ref facts from the current
model/run; `private_only`, `private_project_data`, pending review, contributor
`user_local_session`, and certification `not_asserted`. It must not hard-code
`project_fixture`, `invented_public_example`, or invented-preview prose for a
user-local session. The package-owned copies of model/result/analysis-run facts
apply the same correction without mutating source state; all values then pass
the local-private control with explicit intent.
Provenance names the current user-local session and actual source/run refs,
marks private-only/pending review truthfully, and asserts no contributor
certification. A runtime current-session-path success test must exercise this
mapping with invented, non-engineering fixture values but without any
production hard-coded `TBD` or invented provenance substitution. No real
private project data is committed.

The Rust bridge uses explicit owned wire DTOs and checked conversion. It does
not add serde features or wire authority to the `audit_manifest` or
`result_export` crates. Unknown enum spellings, incomplete hashes/references,
non-finite numbers, missing units/dimensions, missing required member families,
or unsupported values return stable blocking diagnostics before assembly.

The existing ResultExportPanel packet stays unchanged and is not an
integration input. For DEL-08-06, freeze the exact source set to current model
state, analysis-run record, optional current comparison, and any actually
present handoff/export records. A deterministic TypeScript builder mirrors the
accepted Python engine's envelope, coverage, limitation/TBD, unit, authority-
term, diagnostic, provenance, and professional-boundary semantics. One shared
invented fixture generated by the Python engine is checked byte/structure-wise
by both Python and TypeScript tests; a deliberate parity mutation must fail.
Dynamic current-session records use that proven mapping and are revalidated
before package assembly. No empty/synthetic success record is permitted.

## 5. Redaction and no-bypass contract

Add one fixed route ID `DREP-PACKAGE-SAVE-009` using existing context
`local_private`. The on-disk action is an explicitly user-requested local
native package save. Known private values are retained only when the user
checks a UI-owned, default-off `Include known private values in this local
package` control. Source/payload-carried intent is stripped and cannot satisfy
the gate. Unknown values retain the existing warning-only unknown posture and
are not reclassified public or private. This local save does not claim that the
result is public/releasable; any future public distribution must pass its own
`public_report`/shared export route.

Before package assembly:

1. build a fresh immutable request representation from current session state;
2. pass every value-bearing leaf through the existing controlled-export
   contract at fixed `local_private`, with only the UI-owned explicit-intent
   state supplied by the wrapper;
3. preserve the sanitized payload plus decisions, findings, blocked state, and
   summary; never mutate the source model/result/run/report packets;
4. expose the evidence in the Report UI and include the redaction route ID and
   summary counts in the save receipt;
5. if any blocking finding exists, if a known-private value would be retained
   without explicit intent, if lossless materialization is impossible, or if
   payload is null, do not invoke package assembly, do not open the picker, and
   do not create a file;
6. after assembly, honor the producer's `export_blocked` independently before
   opening the picker or writing.

The explicit-intent control and package-save status are App-owned so both menu
surfaces share one state; the Report panel renders and changes that state.
Native-menu enabled state is never an authority substitute.

Redaction decisions/findings/summary remain caller/UI evidence. They are not
inserted into the package manifest because doing so would change the accepted
package member/schema contract. This is preservation, not silent deletion: the
save receipt and UI name the exact route and counts, while the package contains
only the sanitized established members.

## 6. Menu, native picker, and atomic save

Add command ID `file.save-report-package` with label `Save Report Package…` to
both the DOM and native File menus. Both converge on the existing React command
sink. The sink rechecks solved-session readiness, busy state, redaction result,
and package result; native menu enabled state is never trusted.

Picker design: add the Rust `tauri-plugin-dialog` dependency only, initialize
it in the Tauri builder, and invoke its backend API from the custom command.
No JavaScript dialog/filesystem dependency and no broad frontend filesystem
capability is authorized. The native picker suggests the producer filename,
filters `.opsproj`, and cancellation returns `cancelled` without filesystem
effect. Browser/source/dist H4 returns the stable
`REPORT-PACKAGE-SAVE-DESKTOP-ONLY` route and never substitutes a download.

Atomic writer, limited to the current ruled macOS target:

1. validate `.opsproj`, existing parent directory, non-directory destination,
   caller/producer digests, and unblocked state;
2. perform best-effort preflight rejection of a symlink destination or symlink
   parent and create the temp path only under the user-selected parent;
3. create an unpredictable `create_new` temp file in that same parent;
4. `write_all`, `flush`, `sync_all`, and close the temp handle;
5. rename in place only after all prior steps pass; on macOS same-filesystem
   rename may atomically replace an existing regular file, with the previous
   file retained until the rename commits; never delete the destination first;
6. sync the parent directory after rename. If this post-rename sync fails,
   report `saved_durability_uncertain` truthfully; do not claim rollback;
7. on every pre-rename failure, remove only the owned temp and preserve the
   existing destination. Preserve primary and cleanup diagnostics separately.

This path-based implementation is explicitly bounded to a non-adversarial
local-user model. It does not claim race-free containment against a concurrent
malicious parent/destination swap between preflight and create/rename. That
TOCTOU residual is shown in the save receipt, tests, run record, and handoff;
directory-handle-relative/no-follow hardening is not invented or queued here.

Return a structured receipt with `saved|cancelled`, stable stage/code,
container filename, byte count, manifest/package identity hash, container hash,
member summaries, replacement flag, durability state, redaction route/counts,
and path display limited to the user-selected basename, plus
`path_containment=best_effort_non_adversarial` and the TOCTOU limitation. No
private absolute path enters report content, telemetry, or public evidence.

Cross-platform atomic replacement remains unclaimed and out of scope.

## 7. Verification and terminal evidence

Focused acceptance tests must prove:

- report-package container producer tests remain byte/hash/member green;
- bridge mappings preserve real IDs/hashes, units/dimensions, provenance,
  diagnostics, statuses/TBDs, rule-pack metadata, and boundary notices;
- malformed/incomplete wire values fail before assembly; blocked redaction and
  blocked producer outcomes open no picker and create no file;
- menu parity, readiness, cancellation, reentrancy, retry, browser honest
  unavailability, success receipt, and structured failure UI;
- new destination and existing-file replacement write exact bytes; each
  pre-rename error preserves prior bytes and cleans temp; post-rename directory
  sync failure reports uncertain durability; static symlink/extension/parent/
  collision cases fail closed; receipt/UI exposes the non-adversarial TOCTOU
  residual and no adversarial-swap protection claim is made;
- no headless-runner file or verb changes.

Run the affected `software-workflow.json` union plus focused checks:

- focused Rust tests for report package, PDF, renderer, and desktop bridge/save;
- focused report/service/App Vitest;
- registered desktop Vitest and production build;
- H4 Playwright source and production-dist tests for menu reachability,
  readiness/blocking, and honest browser routing;
- one bounded packaged-macOS H4-native smoke at the terminal candidate: launch
  the built app with invented/current-session-safe test state; prove picker
  cancellation creates nothing, save creates an exact producer-hash-matching
  `.opsproj`, and explicit replacement leaves no temp and produces the new
  exact bytes. Record actor, app SHA, destination basenames/digests, outcomes,
  and cleanup in `apps/desktop/SMOKE.md` without private absolute paths. Rust
  fault-injection remains authoritative for all failure stages;
- piping pytest, full practitioner-harness pytest, repository self-check;
- exactly one acceptance-eligible DEC-025 sweep for the terminal attempt;
- claims, paths, JSON/JSONL, containment, change-scope, raw-evidence, and
  `git diff --check` validations as applicable.

N4 returns terminal evidence but does not update deliverable state or perform
Git closeout. A fresh N5 reviews the exact diff and evidence. Only N5
`COMMIT-SAFE` releases DEL-08-01-only W3 closeout.

## 8. Exact bounded shared-integration authorization requested

N4 is the sole serialized implementation owner. Owner adoption must explicitly
authorize the path rows in `WRITE_MATRIX.csv`. The activation remains
PKG-08/DEL-08-01; shared desktop/report/result surfaces receive only the
integration edits shown there and no lifecycle, memory, Remaining, acceptance,
or handoff effect.

No other path is writable. Generated lockfile changes must be solely the
dependency resolution implied by the adopted Cargo additions. Any new path,
dependency, member/schema change, route-context change, or broadened platform
claim stops for amendment.

## 9. Owner adoption act requested after N3

N3F returned `COMMIT-SAFE`. The owner supplied this exact act:

```text
I ADOPT v5 of CB-2026-07-22-DEL-08-01-REPORT-PACKAGE-SEAM-001 and explicitly
authorize the bounded shared-integration scope in WRITE_MATRIX.csv, with N4 as
sole serialized implementation owner, N5 as fresh verifier, and DEL-08-01-only
W3 closeout only after N5 returns COMMIT-SAFE.
```

Adoption does not authorize merge, lifecycle/stage advancement, release,
issuance, DEL-10-05 execution, or a runner change.

**Gate outcome:** `ADOPTED V5 / N4 AUTHORIZED` on 2026-07-22. N5 and W3 remain
unreleased until the preceding terminal gates pass.

## 10. Owner-authorized v6 verification-unblock amendment

The terminal pre-sweep run reproduced four failures from the exact frozen SHA:
`tests/test_release_readiness_script.py` hard-coded DAG-007 although the same
frozen baseline's root `_DAG/_LATEST.md` resolves accepted DAG-008. The owner
authorized one and only one added N4 write target:

```text
projects/chirality-piping/tests/test_release_readiness_script.py
```

Its sole permitted effect is replacing stale DAG-007 assumptions with active
DAG-008 expectations. No product, runner, graph, pointer, lifecycle, or other
test scope changes are authorized. N4 must rerun the focused file and full
piping pytest. Only a green result releases exactly one acceptance-eligible
DEC-025 sweep; no filesystem edit may follow that sweep. N5 and W3 remain held
until the sweep returns terminal evidence.

**Gate outcome:** `ADOPTED V6 / N4 ONE-FILE VERIFICATION REPAIR AUTHORIZED` on
2026-07-22.

## 11. Owner-accepted v7 remediation basis

Fresh N5 returned `BLOCK` after the v6 terminal sweep on four semantic defects:
false result-envelope-as-input-manifest evidence, incorrect stiffness
dimensions, overtaken invented-public classifications in current-private
records, and insufficient SHA-256 shape validation.

The owner accepted run-local derived plan
`R16-SOFTWARE-DECOMP-REMEDIATION-PLAN-001` without creating or publishing a
canonical decomposition snapshot. `AMENDMENT_V7.md` and the expanded
`WRITE_MATRIX.csv` are the exact managed execution basis.

N4 remains the sole serialized implementation owner and executes in order:

```text
R16-SLICE-01 → R16-SLICE-02 → R16-SLICE-03 → R16-SLICE-04
             → R16-SLICE-05
```

Any excluded-path need stops for owner amendment. The prior DEC-025 sweep is
immutable superseded evidence. Only after every focused/full gate and the new
packaged-native proof passes may N4 invoke the one owner-authorized replacement
DEC-025 sweep. The tree then freezes for a fresh read-only N5. W3 remains
DEL-08-01-only and requires N5 `COMMIT-SAFE`.

**Gate outcome:** `ADOPTED V7 REMEDIATION / N4 SLICES 01–05 AUTHORIZED` on
2026-07-23.

## 12. Owner-authorized v8 sealed remediation attempt

Fresh v7 N5 verified that all four v6 semantic blockers were closed, then
returned `BLOCK` on three narrower implementation findings. The owner
authorized a sealed v8 attempt limited to:

1. exact model-bound InputManifest reference validation in the existing
   TypeScript and Python surfaces;
2. removal of remaining result-family inference from unit text;
3. OS-random atomic temp names using direct `getrandom = "0.3"` through the
   existing desktop Cargo manifest/lockfile, plus deterministic injected
   write/flush/temp-sync/rename failure tests proving cleanup, prior-destination
   preservation, and primary-diagnostic retention.

No other path, dependency, semantic, schema, package member, or product effect
is authorized. V6 and v7 sweeps remain immutable superseded evidence. Only
after full gates and a fresh packaged-native proof pass may N4 run exactly one
v8 replacement DEC-025 sweep. No edit follows that sweep; fresh N5 is then
read-only and terminal-only. W3 still requires N5 `COMMIT-SAFE`.

**Gate outcome:** `ADOPTED V8 REMEDIATION / N4 THREE-FINDING CLOSURE
AUTHORIZED` on 2026-07-23.

## 13. Owner-authorized v9 authority amendment

Fresh v8 N5 confirmed every v7 product finding closed and returned `BLOCK` only
because `WRITE_MATRIX.csv` row 6 did not authorize an existing spring-hanger
report-adapter semantic correction. The owner expressly ratified the existing
behavior, without authorizing any new product/source/schema/decomposition
change:

- select spring-hanger force in this precedence:
  `constant_load → hot_load → cold_load → installed_load`;
- emit `{magnitude, unit, dimension: "force"}`;
- limit `required_for` to reporting and human review;
- report missing data when no force quantity exists.

`AMENDMENT_V9.md` and the updated row 6 are the complete authority change. The
implementation must remain unchanged. N4 repeats affected/full/native gates,
seals records, and only then runs exactly one v9 replacement DEC-025 sweep. No
edit follows that sweep. Fresh N5 is read-only/terminal-only. V6–v8 sweeps
remain immutable superseded evidence; W3 still requires N5 `COMMIT-SAFE`.

**Gate outcome:** `ADOPTED V9 AUTHORITY AMENDMENT / REVERIFICATION AUTHORIZED`
on 2026-07-23.
